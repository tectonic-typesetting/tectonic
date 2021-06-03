use flate2::{write::GzEncoder, GzBuilder};
use futures::future;
use headers::HeaderMapExt;
use hyper::header::{self, HeaderValue};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::ops::Bound;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use tectonic::config::PersistentConfig;
use tectonic::driver::ProcessingSessionBuilder;
use tectonic::io::OpenResult;
use tectonic::status::termcolor::TermcolorStatusBackend;
use tectonic::status::ChatterLevel;
use tokio::runtime::current_thread;

mod util;

/// Build a fake tarindex by concatenating files.
struct TarIndexBuilder {
    tar: Vec<u8>,
    index: GzEncoder<Vec<u8>>,
    /// Map from (offset, length) to file name.
    map: HashMap<(u64, u64), String>,
}

impl TarIndexBuilder {
    fn new() -> TarIndexBuilder {
        let tar = Vec::new();
        let index = GzBuilder::new()
            .filename("bundle.tar.index.gz")
            .write(Vec::new(), flate2::Compression::default());
        let map = HashMap::new();

        TarIndexBuilder { tar, index, map }
    }

    /// Add a file.
    fn push(&mut self, name: &str, content: &[u8]) -> &mut Self {
        let offset = self.tar.len();
        let len = content.len();
        let _ = writeln!(&mut self.index, "{} {} {}", name, offset, len);
        self.map
            .insert((offset as u64, len as u64), name.to_owned());
        self.tar.extend_from_slice(&content);
        self
    }

    /// Create a tar index.
    fn finish(self) -> TarIndex {
        TarIndex {
            tar: self.tar,
            index: self.index.finish().unwrap(),
            map: self.map,
        }
    }
}

#[derive(Clone, Debug)]
struct TarIndex {
    tar: Vec<u8>,
    index: Vec<u8>,
    map: HashMap<(u64, u64), String>,
}

impl TarIndex {
    fn from_dir<P: AsRef<Path>>(path: P) -> io::Result<TarIndex> {
        let path = path.as_ref();
        let mut builder = TarIndexBuilder::new();
        for de in path.read_dir()? {
            let path = de?.path();
            let content = fs::read(&path)?;
            builder.push(path.file_name().unwrap().to_str().unwrap(), &content);
        }

        builder.push(
            tectonic::digest::DIGEST_NAME,
            b"0000000000000000000000000000000000000000000000000000000000000000",
        );

        Ok(builder.finish())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TectonicRequest {
    Head(String),
    Index,
    File(String),
}

struct TarIndexService {
    tar_index: Mutex<TarIndex>,
    requests: Mutex<Vec<TectonicRequest>>,
    local_addr: Mutex<Option<SocketAddr>>,
}

type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = io::Error> + Send>;

impl TarIndexService {
    fn new(tar_index: TarIndex) -> TarIndexService {
        TarIndexService {
            tar_index: Mutex::new(tar_index),
            requests: Mutex::new(Vec::new()),
            local_addr: Mutex::new(None),
        }
    }

    fn set_local_addr(&self, local_addr: SocketAddr) {
        *self.local_addr.lock().unwrap() = Some(local_addr);
    }

    fn set_tar_index(&self, tar_index: TarIndex) {
        *self.tar_index.lock().unwrap() = tar_index;
    }

    fn response(&self, req: Request<Body>) -> ResponseFuture {
        match (
            req.method(),
            req.uri().path(),
            req.headers().typed_get::<headers::Range>(),
        ) {
            (&Method::HEAD, "/tectonic-default", None) => {
                self.log_request(TectonicRequest::Head(req.uri().path().to_owned()));
                let mut resp = Response::builder();
                resp.status(StatusCode::FOUND);
                resp.headers_mut().unwrap().insert(
                    header::LOCATION,
                    HeaderValue::from_str(&format!(
                        "http://{}/bundle.tar",
                        self.local_addr.lock().unwrap().unwrap()
                    ))
                    .unwrap(),
                );
                Box::new(future::ok(resp.body(Body::empty()).unwrap()))
            }
            (&Method::HEAD, "/bundle.tar", None) => {
                self.log_request(TectonicRequest::Head(req.uri().path().to_owned()));
                Box::new(future::ok(Response::new(Body::empty())))
            }
            (&Method::GET, "/bundle.tar", Some(range)) => {
                if let Some((Bound::Included(l), Bound::Included(h))) = range.iter().next() {
                    let tar_index = self.tar_index.lock().unwrap();
                    let name = tar_index
                        .map
                        .get(&(l, h - l + 1))
                        .expect("unknown file data requested");
                    self.log_request(TectonicRequest::File(name.to_owned()));
                    let mut resp = Response::builder();
                    resp.status(StatusCode::PARTIAL_CONTENT);
                    resp.headers_mut()
                        .unwrap()
                        .typed_insert(headers::ContentRange::bytes(l..=h, None).unwrap());
                    Box::new(future::ok(
                        resp.body((&tar_index.tar[l as usize..=h as usize]).to_vec().into())
                            .unwrap(),
                    ))
                } else {
                    panic!("unexpected");
                }
            }
            (&Method::GET, "/bundle.tar.index.gz", None) => {
                self.log_request(TectonicRequest::Index);
                Box::new(future::ok(Response::new(
                    self.tar_index.lock().unwrap().index.to_vec().into(),
                )))
            }
            _ => Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap(),
            )),
        }
    }

    fn log_request(&self, request: TectonicRequest) {
        self.requests.lock().unwrap().push(request);
    }

    fn url(&self) -> String {
        format!(
            "http://{}/tectonic-default",
            self.local_addr.lock().unwrap().unwrap()
        )
    }
}

/// Run the provided closure while http service is running. Use the tar index given as
/// the first variable, or a default on if None.
fn run_test<R>(tar_index: Option<TarIndex>, run: R) -> Vec<TectonicRequest>
where
    R: FnOnce(Arc<TarIndexService>, &str),
{
    // Automatically select a port
    let addr = ([127, 0, 0, 1], 0).into();

    let tar_service = Arc::new(TarIndexService::new(tar_index.unwrap_or_else(|| {
        let root = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("assets");
        TarIndex::from_dir(root).unwrap()
    })));
    let tar_service_clone = Arc::clone(&tar_service);

    let server = Server::bind(&addr).serve(move || {
        let tar_service = Arc::clone(&tar_service_clone);
        service_fn(move |req| tar_service.response(req))
    });

    // server is listening now
    tar_service.set_local_addr(server.local_addr());
    let url = tar_service.url();

    let (server_shutdown_tx, server_shutdown_rx) = futures::sync::oneshot::channel::<()>();

    let graceful = server.with_graceful_shutdown(server_shutdown_rx);

    let server_thread = thread::spawn(|| {
        // Run the server on a single thread (current thread)
        current_thread::run(graceful.map_err(|_| ()));
    });

    // Server running, run the provided test
    run(Arc::clone(&tar_service), &url);

    println!("Shutting down");

    // Shut down server
    let _ = server_shutdown_tx.send(());
    server_thread.join().unwrap();

    // Check tectonic's requests.
    let requests = tar_service.requests.lock().unwrap();

    requests.clone()
}

fn check_req_count(requests: &[TectonicRequest], request: TectonicRequest, expected_number: usize) {
    let number = requests.iter().filter(|r| **r == request).count();
    assert_eq!(
        number, expected_number,
        "Expected {} requests of {:?}, got {}",
        expected_number, request, number
    );
}
#[test]
fn test_full_session() {
    let requests = run_test(None, |_, url| {
        let tempdir = tempfile::tempdir().unwrap();

        let config = PersistentConfig::default();

        let run = |path| {
            let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);
            let mut sess_builder = ProcessingSessionBuilder::default();
            sess_builder.bundle(Box::new(
                config
                    .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                    .unwrap(),
            ));
            let input_path = Path::new(path);
            sess_builder.primary_input_path(input_path);
            sess_builder.tex_input_name(&input_path.file_name().unwrap().to_string_lossy());
            sess_builder.output_dir(tempdir.path());
            sess_builder.format_name("plain");
            sess_builder.format_cache_path(tempdir.path());

            let mut sess = sess_builder.create(&mut status).unwrap();
            sess.run(&mut status).unwrap();
        };

        // Run tectonic twice
        run("tests/tex-outputs/the_letter_a.tex");
        // On this run everything should be cached.
        run("tests/tex-outputs/the_letter_a.tex");
        // Run tectonic with a file that needs a new resource
        run("tests/tex-outputs/redbox_png.tex");
    });

    check_req_count(&requests, TectonicRequest::Index, 1);
    check_req_count(
        &requests,
        TectonicRequest::File(tectonic::digest::DIGEST_NAME.into()),
        2,
    );
    // This file should be cached.
    check_req_count(&requests, TectonicRequest::File("plain.tex".into()), 1);
}

#[test]
fn test_cached_url_provider() {
    let tar_index = {
        let mut builder = TarIndexBuilder::new();
        builder
            .push("plain.tex", b"test")
            .push("other.tex", b"other content")
            .push(
                tectonic::digest::DIGEST_NAME,
                b"0000000000000000000000000000000000000000000000000000000000000000",
            );
        builder.finish()
    };

    let requests = run_test(Some(tar_index), |_, url| {
        let tempdir = tempfile::tempdir().unwrap();
        let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);

        let config = PersistentConfig::default();

        {
            let mut cache = config
                .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                .unwrap();

            match cache.input_open_name("plain.tex", &mut status) {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
            match cache.input_open_name("plain.tex", &mut status) {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
        }
        {
            let mut cache = config
                .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                .unwrap();

            // should be cached
            match cache.input_open_name("plain.tex", &mut status) {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
        }
        {
            let mut cache = config
                .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                .unwrap();

            // should be cached
            match cache.input_open_name("plain.tex", &mut status) {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
            // in index, should check digest and download the file
            match cache.input_open_name("other.tex", &mut status) {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open other.tex"),
            }
        }
        {
            let mut cache = config
                .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                .unwrap();

            // not in index
            match cache.input_open_name("my-favourite-file.tex", &mut status) {
                OpenResult::NotAvailable => {}
                _ => panic!("'my-favourite-file.tex' file exists?"),
            }
        }
    });

    check_req_count(&requests, TectonicRequest::Index, 1);
    check_req_count(
        &requests,
        TectonicRequest::File(tectonic::digest::DIGEST_NAME.into()),
        2,
    );
    // This files should be cached.
    check_req_count(&requests, TectonicRequest::File("plain.tex".into()), 1);
    check_req_count(&requests, TectonicRequest::File("other.tex".into()), 1);
}

#[test]
fn test_bundle_update() {
    let tempdir = tempfile::tempdir().unwrap();
    let tar_index = {
        let mut builder = TarIndexBuilder::new();
        builder
            .push("only-first.tex", b"test")
            .push("file-in-both.tex", b"in both")
            .push(
                tectonic::digest::DIGEST_NAME,
                b"0000000000000000000000000000000000000000000000000000000000000000",
            );
        builder.finish()
    };

    run_test(Some(tar_index), |service, url| {
        let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);

        let config = PersistentConfig::default();

        {
            // Run with first tar index.
            {
                let mut cache = config
                    .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                    .unwrap();

                match cache.input_open_name("only-first.tex", &mut status) {
                    OpenResult::Ok(_) => {}
                    _ => panic!("Failed to open only-first.tex"),
                }
            }

            // Set a tar index with a different digest.
            let tar_index = {
                let mut builder = TarIndexBuilder::new();
                builder
                    .push("only-second.tex", b"test")
                    .push("file-in-both.tex", b"in both")
                    .push(
                        tectonic::digest::DIGEST_NAME,
                        b"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                    );
                builder.finish()
            };
            service.set_tar_index(tar_index);

            // Run with the new tar index.
            {
                let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);

                let config = PersistentConfig::default();

                {
                    let mut cache = config
                        .make_cached_url_provider(&url, false, Some(tempdir.path()), &mut status)
                        .unwrap();

                    // This should be cached even thought the bundle does not contain it.
                    match cache.input_open_name("only-first.tex", &mut status) {
                        OpenResult::Ok(_) => {}
                        _ => panic!("Failed to open only-first.tex"),
                    }

                    // Not in index of the first bundle and therefore no digest check.
                    match cache.input_open_name("only-second.tex", &mut status) {
                        OpenResult::NotAvailable => {}
                        _ => panic!("File should not be in the first bundle"),
                    }
                    // File in the first bundle and the second bundle, but not cached yet. Should
                    // trigger a digest check.
                    match cache.input_open_name("file-in-both.tex", &mut status) {
                        OpenResult::Err(_) => {}
                        _ => panic!("Bundle digest changed but no error"),
                    }
                }
            }
        }
    });
}
