use flate2::{write::GzEncoder, GzBuilder};
use headers::HeaderMapExt;
use hyper::header::{self, HeaderValue};
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::ops::Bound;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::{env, fs, thread};
use tectonic::config::PersistentConfig;
use tectonic::driver::ProcessingSessionBuilder;
use tectonic::io::OpenResult;
use tokio::runtime;

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
        let _ = writeln!(&mut self.index, "{name} {offset} {len}");
        self.map
            .insert((offset as u64, len as u64), name.to_owned());
        self.tar.extend_from_slice(content);
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

type ResponseFuture = Pin<Box<dyn Future<Output = Response<Body>> + Send + Sync + 'static>>;

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
                let mut resp = Response::builder().status(StatusCode::FOUND);
                resp.headers_mut().unwrap().insert(
                    header::LOCATION,
                    HeaderValue::from_str(&format!(
                        "http://{}/bundle.tar",
                        self.local_addr.lock().unwrap().unwrap()
                    ))
                    .unwrap(),
                );
                Box::pin(async move { resp.body(Body::empty()).unwrap() })
            }
            (&Method::HEAD, "/bundle.tar", None) => {
                self.log_request(TectonicRequest::Head(req.uri().path().to_owned()));
                Box::pin(async move { Response::new(Body::empty()) })
            }
            (&Method::GET, "/bundle.tar", Some(range)) => {
                if let Some((Bound::Included(l), Bound::Included(h))) = range.iter().next() {
                    let tar_index = self.tar_index.lock().unwrap();
                    let name = tar_index
                        .map
                        .get(&(l, h - l + 1))
                        .expect("unknown file data requested");
                    self.log_request(TectonicRequest::File(name.to_owned()));
                    let mut resp = Response::builder().status(StatusCode::PARTIAL_CONTENT);
                    resp.headers_mut()
                        .unwrap()
                        .typed_insert(headers::ContentRange::bytes(l..=h, None).unwrap());
                    let body = (tar_index.tar[l as usize..=h as usize]).to_vec().into();
                    Box::pin(async move { resp.body(body).unwrap() })
                } else {
                    panic!("unexpected");
                }
            }
            (&Method::GET, "/bundle.tar.index.gz", None) => {
                self.log_request(TectonicRequest::Index);
                let resp = self.tar_index.lock().unwrap().index.to_vec().into();
                Box::pin(async move { Response::new(resp) })
            }
            _ => Box::pin(async move {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap()
            }),
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

    let (url_available_tx, url_available_rx) = std::sync::mpsc::channel();
    let (server_shutdown_tx, server_shutdown_rx) = futures::channel::oneshot::channel::<()>();
    let tar_service_clone = Arc::clone(&tar_service);

    let server_thread = thread::spawn(move || {
        let tar_service = tar_service_clone;

        let rt = runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();

        let tar_service_clone = Arc::clone(&tar_service);
        rt.block_on(async move {
            let server = Server::bind(&addr).serve(make_service_fn(move |_| {
                let tar_service_clone = Arc::clone(&tar_service_clone);
                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        let tar_service = Arc::clone(&tar_service_clone);
                        async move { Ok::<_, Infallible>(tar_service.response(req).await) }
                    }))
                }
            }));

            // server is listening now
            tar_service.set_local_addr(server.local_addr());
            let url = tar_service.url();
            url_available_tx.send(url).unwrap();

            let graceful = server.with_graceful_shutdown(async move {
                server_shutdown_rx.await.unwrap();
            });

            graceful.await
        })
    });

    // Server running, run the provided test
    let url = url_available_rx.recv().unwrap();
    run(Arc::clone(&tar_service), &url);

    println!("Shutting down");

    // Shut down server
    let _ = server_shutdown_tx.send(());
    server_thread.join().unwrap().unwrap();

    // Check tectonic's requests.
    let requests = tar_service.requests.lock().unwrap();

    requests.clone()
}

fn check_req_count(requests: &[TectonicRequest], request: TectonicRequest, expected_number: usize) {
    let number = requests.iter().filter(|r| **r == request).count();
    assert_eq!(
        number, expected_number,
        "Expected {expected_number} requests of {request:?}, got {number}"
    );
}
#[test]
fn test_full_session() {
    let requests = run_test(None, |_, url| {
        let tempdir = tempfile::tempdir().unwrap();

        let config = PersistentConfig::default();

        let run = |path| {
            let mut sess_builder = ProcessingSessionBuilder::default();
            sess_builder.bundle(Box::new(
                config
                    .make_cached_url_provider(url, false, Some(tempdir.path()))
                    .unwrap(),
            ));
            let input_path = Path::new(path);
            sess_builder.primary_input_path(input_path);
            sess_builder.tex_input_name(&input_path.file_name().unwrap().to_string_lossy());
            sess_builder.output_dir(tempdir.path());
            sess_builder.format_name("plain");
            sess_builder.format_cache_path(tempdir.path());

            let mut sess = sess_builder.create().unwrap();
            sess.run().unwrap();
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
        let config = PersistentConfig::default();

        {
            let mut cache = config
                .make_cached_url_provider(url, false, Some(tempdir.path()))
                .unwrap();

            match cache.input_open_name("plain.tex") {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
            match cache.input_open_name("plain.tex") {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
        }
        {
            let mut cache = config
                .make_cached_url_provider(url, false, Some(tempdir.path()))
                .unwrap();

            // should be cached
            match cache.input_open_name("plain.tex") {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
        }
        {
            let mut cache = config
                .make_cached_url_provider(url, false, Some(tempdir.path()))
                .unwrap();

            // should be cached
            match cache.input_open_name("plain.tex") {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open plain.tex"),
            }
            // in index, should check digest and download the file
            match cache.input_open_name("other.tex") {
                OpenResult::Ok(_) => {}
                _ => panic!("Failed to open other.tex"),
            }
        }
        {
            let mut cache = config
                .make_cached_url_provider(url, false, Some(tempdir.path()))
                .unwrap();

            // not in index
            match cache.input_open_name("my-favourite-file.tex") {
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
        let config = PersistentConfig::default();

        {
            // Run with first tar index.
            {
                let mut cache = config
                    .make_cached_url_provider(url, false, Some(tempdir.path()))
                    .unwrap();

                match cache.input_open_name("only-first.tex") {
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
                let config = PersistentConfig::default();

                {
                    let mut cache = config
                        .make_cached_url_provider(url, false, Some(tempdir.path()))
                        .unwrap();

                    // This should be cached even thought the bundle does not contain it.
                    match cache.input_open_name("only-first.tex") {
                        OpenResult::Ok(_) => {}
                        _ => panic!("Failed to open only-first.tex"),
                    }

                    // Not in index of the first bundle and therefore no digest check.
                    match cache.input_open_name("only-second.tex") {
                        OpenResult::NotAvailable => {}
                        _ => panic!("File should not be in the first bundle"),
                    }
                    // File in the first bundle and the second bundle, but not cached yet. Should
                    // trigger a digest check.
                    match cache.input_open_name("file-in-both.tex") {
                        OpenResult::Err(_) => {}
                        _ => panic!("Bundle digest changed but no error"),
                    }
                }
            }
        }
    });
}

#[test]
fn test_cache_location_redirect() {
    const CACHE_DIR_KEY: &str = "TECTONIC_CACHE_DIR";
    let tempdir = tempfile::tempdir().unwrap();

    // In this test we intentionally set the environment variable and don't use the custom cache root parameter,
    // to test the internal mechanism for a custom cache location based on an environment variable.
    env::set_var(CACHE_DIR_KEY, tempdir.path().as_os_str());

    let tar_index = {
        let mut builder = TarIndexBuilder::new();
        builder.push("plain.tex", b"simple").push(
            tectonic::digest::DIGEST_NAME,
            b"0000000000000000000000000000000000000000000000000000000000000000",
        );

        builder.finish()
    };

    run_test(Some(tar_index), |_, url| {
        let config = PersistentConfig::default();

        let mut cache = config.make_cached_url_provider(url, false, None).unwrap();

        match cache.input_open_name("plain.tex") {
            OpenResult::Ok(_) => {}
            _ => panic!("Failed to open plain.tex"),
        }

        // the filename of the target location is the SHA256 hash of the file content "simple"
        let expected_file_path = tempdir
            .path()
            .join("files")
            .join("a7")
            .join("a39b72f29718e653e73503210fbb597057b7a1c77d1fe321a1afcff041d4e1");

        if !expected_file_path.exists() {
            panic!("Couldn't find the cached file in the expected location.");
        }
    });
}
