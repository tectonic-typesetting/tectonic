// src/io/itarbundle.rs -- I/O on files in an indexed tar file "bundle"
// Copyright 2017-2018 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use hyper::client::{RedirectPolicy, Response};
use hyper::header::{Headers, Range};
use hyper::status::StatusCode;
use hyper::{self, Client, Url};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, BufReader, Cursor, Read};

use super::{create_hyper_client, Bundle, InputHandle, InputOrigin, IoProvider, OpenResult};
use errors::{Error, ErrorKind, Result, ResultExt};
use status::StatusBackend;

const MAX_HTTP_ATTEMPTS: usize = 4;

// A simple way to read chunks out of a big seekable byte stream. You could
// implement this for io::File pretty trivially but that's not currently
// needed.

pub trait RangeRead {
    type InnerRead: Read;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Self::InnerRead>;
}

pub struct HttpRangeReader {
    url: String,
    client: Client,
}

impl HttpRangeReader {
    pub fn new(url: &str) -> HttpRangeReader {
        let client = create_hyper_client();

        HttpRangeReader {
            url: url.to_owned(),
            client: client,
        }
    }
}

impl RangeRead for HttpRangeReader {
    type InnerRead = Response;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Response> {
        let end_inclusive = offset + length as u64 - 1;

        let mut headers = Headers::new();
        headers.set(Range::bytes(offset, end_inclusive));

        let req = self.client.get(&self.url).headers(headers);
        let res = req.send()?;

        if res.status != StatusCode::PartialContent {
            // FIXME: this loses the actual status code! Should report it.
            return Err(hyper::Error::Status.into());
        }

        Ok(res)
    }
}

// The IoProvider. We jump through some hoops so that web-based bundles can
// be created without immediately connecting to the network.

pub trait ITarIoFactory {
    type IndexReader: Read;
    type DataReader: RangeRead;

    fn get_index(&mut self, status: &mut StatusBackend) -> Result<Self::IndexReader>;
    fn get_data(&self) -> Result<Self::DataReader>;
    fn report_fetch(&self, name: &OsStr, status: &mut StatusBackend);
}

struct FileInfo {
    offset: u64,
    length: u64,
}

pub struct ITarBundle<F: ITarIoFactory> {
    factory: F,
    data: Option<F::DataReader>,
    index: HashMap<OsString, FileInfo>,
}

impl<F: ITarIoFactory> ITarBundle<F> {
    fn construct(factory: F) -> ITarBundle<F> {
        ITarBundle {
            factory: factory,
            data: None,
            index: HashMap::new(),
        }
    }

    fn ensure_loaded(&mut self, status: &mut StatusBackend) -> Result<()> {
        if self.data.is_some() {
            return Ok(());
        }

        // We need to initialize. First, the index ...

        let index = self.factory.get_index(status)?;
        let br = BufReader::new(index);

        for res in br.lines() {
            let line = res?;
            let bits = line.split_whitespace().collect::<Vec<_>>();

            if bits.len() < 3 {
                continue; // TODO: preserve the warning info or something!
            }

            let name = OsString::from(bits[0]);
            let offset = bits[1].parse::<u64>()?;
            let length = bits[2].parse::<u64>()?;
            self.index.insert(
                name,
                FileInfo {
                    offset: offset,
                    length: length,
                },
            );
        }

        // ... then, the data reader.

        self.data = Some(self.factory.get_data()?);
        Ok(())
    }
}

impl<F: ITarIoFactory> IoProvider for ITarBundle<F> {
    fn input_open_name(
        &mut self,
        name: &OsStr,
        status: &mut StatusBackend,
    ) -> OpenResult<InputHandle> {
        if let Err(e) = self.ensure_loaded(status) {
            return OpenResult::Err(e.into());
        }

        // In principle it'd be cool to return a handle right to the HTTP
        // response, but those can't be seekable, and doing so introduces
        // lifetime-related issues. So for now we just slurp the whole thing
        // into RAM.

        let info = match self.index.get(name) {
            Some(i) => i,
            None => return OpenResult::NotAvailable,
        };

        self.factory.report_fetch(name, status);

        // When fetching a bunch of resource files (i.e., on the first
        // invocation), bintray will sometimes drop connections. The error
        // manifests itself in a way that has a not-so-nice user experience.
        // Our solution: retry the HTTP a few times in case it was a transient
        // problem.

        let mut buf = Vec::with_capacity(info.length as usize);
        let mut overall_failed = true;
        let mut any_failed = false;

        for _ in 0..MAX_HTTP_ATTEMPTS {
            let mut stream = match self
                .data
                .as_mut()
                .unwrap()
                .read_range(info.offset, info.length as usize)
            {
                Ok(r) => r,
                Err(e) => {
                    tt_warning!(status, "failure requesting \"{}\" from network", name.to_string_lossy(); e.into());
                    any_failed = true;
                    continue;
                }
            };

            if let Err(e) = stream.read_to_end(&mut buf) {
                tt_warning!(status, "failure downloading \"{}\" from network", name.to_string_lossy(); e.into());
                any_failed = true;
                continue;
            }

            overall_failed = false;
            break;
        }

        if overall_failed {
            // Note: can't save & reuse the hyper errors since they're not cloneable
            return OpenResult::Err(
                ErrorKind::Msg(format!(
                    "failed to retrieve \"{}\" from the network; \
                     this most probably is not Tectonic's fault \
                     -- please check your network connection.",
                    name.to_string_lossy()
                ))
                .into(),
            );
        } else if any_failed {
            tt_note!(status, "download succeeded after retry");
        }

        OpenResult::Ok(InputHandle::new(name, Cursor::new(buf), InputOrigin::Other))
    }
}

impl<F: ITarIoFactory> Bundle for ITarBundle<F> {}

pub struct HttpITarIoFactory {
    url: String,
}

impl ITarIoFactory for HttpITarIoFactory {
    type IndexReader = GzDecoder<Response>;
    type DataReader = HttpRangeReader;

    fn get_index(&mut self, status: &mut StatusBackend) -> Result<GzDecoder<Response>> {
        tt_note!(status, "indexing {}", self.url);

        // First, we actually do a HEAD request on the URL for the data file.
        // If it's redirected, we update our URL to follow the redirects. If
        // we didn't do this separately, the index file would have to be the
        // one with the redirect setup, which would be confusing and annoying.

        let mut probe_client = create_hyper_client();
        fn url_redirection_policy(url: &Url) -> bool {
            // In the process of resolving the file url it might be neccesary
            // to stop at a certain level of redirection. This might be required
            // because some hosts might redirect to a version of the url where
            // it isn't possible to select the index file by appending .index.gz.
            // (This mostly happens because CDNs redirect to the file hash.)
            if let Some(segments) = url.path_segments() {
                segments
                    .last()
                    .map(|file| file.contains('.'))
                    .unwrap_or(true)
            } else {
                true
            }
        }
        probe_client.set_redirect_policy(RedirectPolicy::FollowIf(url_redirection_policy));

        let req = probe_client.head(&self.url);
        let res = req.send()?;

        if !(res.status.is_success() || res.status == StatusCode::Found) {
            return Err(Error::from(hyper::Error::Status))
                .chain_err(|| format!("couldn\'t probe {}", self.url));
        }

        let final_url = res.url.clone().into_string();

        if final_url != self.url {
            tt_note!(status, "resolved to {}", final_url);
            self.url = final_url;
        }

        // Now let's actually go for the index.

        let mut index_url = self.url.clone();
        index_url.push_str(".index.gz");

        let client = create_hyper_client();
        let req = client.get(&index_url);
        let res = req.send()?;
        if !res.status.is_success() {
            return Err(Error::from(hyper::Error::Status))
                .chain_err(|| format!("couldn\'t fetch {}", index_url));
        }

        Ok(GzDecoder::new(res))
    }

    fn get_data(&self) -> Result<HttpRangeReader> {
        Ok(HttpRangeReader::new(&self.url))
    }

    fn report_fetch(&self, name: &OsStr, status: &mut StatusBackend) {
        tt_note!(status, "downloading {}", name.to_string_lossy());
    }
}

impl ITarBundle<HttpITarIoFactory> {
    pub fn new(url: &str) -> ITarBundle<HttpITarIoFactory> {
        Self::construct(HttpITarIoFactory {
            url: url.to_owned(),
        })
    }
}
