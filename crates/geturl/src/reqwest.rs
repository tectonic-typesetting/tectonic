// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

use reqwest::{header::HeaderMap, Client, RedirectPolicy, Response, StatusCode};
use tectonic_errors::{anyhow::bail, Result};
use tectonic_status_base::{tt_note, StatusBackend};

use crate::{GetUrlBackend, RangeReader};

const MAX_HTTP_REDIRECTS_ALLOWED: usize = 10;

/// URL-get backed implemented using the `reqwest` crate.
#[derive(Debug, Default)]
pub struct ReqwestBackend {}

impl GetUrlBackend for ReqwestBackend {
    type Response = Response;
    type RangeReader = ReqwestRangeReader;

    fn get_url(&mut self, url: &str, status: &mut dyn StatusBackend) -> Result<Response> {
        let index_url = format!("{}.index.gz", url);
        tt_note!(status, "downloading index {}", index_url);
        let res = Client::new().get(&index_url).send()?;
        if !res.status().is_success() {
            // return UnexpectedHttpResponse(index_url, res.status())
            bail!("unexpected HTTP resonse code for URL {}", url);
        }
        Ok(res)
    }

    fn resolve_url(&mut self, url: &str, status: &mut dyn StatusBackend) -> Result<String> {
        tt_note!(status, "connecting to {}", url);

        // First, we actually do a HEAD request on the URL for the data file.
        // If it's redirected, we update our URL to follow the redirects. If
        // we didn't do this separately, the index file would have to be the
        // one with the redirect setup, which would be confusing and annoying.
        let redirect_policy = RedirectPolicy::custom(|attempt| {
            // In the process of resolving the file url it might be necessary
            // to stop at a certain level of redirection. This might be required
            // because some hosts might redirect to a version of the url where
            // it isn't possible to select the index file by appending .index.gz.
            // (This mostly happens because CDNs redirect to the file hash.)
            if attempt.previous().len() >= MAX_HTTP_REDIRECTS_ALLOWED {
                attempt.too_many_redirects()
            } else if let Some(segments) = attempt.url().clone().path_segments() {
                let follow = segments
                    .last()
                    .map(|file| file.contains('.'))
                    .unwrap_or(true);
                if follow {
                    attempt.follow()
                } else {
                    attempt.stop()
                }
            } else {
                attempt.follow()
            }
        });

        let res = Client::builder()
            .redirect(redirect_policy)
            .build()?
            .head(url)
            .send()?;

        if !(res.status().is_success() || res.status() == StatusCode::FOUND) {
            //return UnexpectedHttpResponse(url.to_string(), res.status())
            bail!("unexpected HTTP resonse code for URL {}", url);
        }

        let final_url = res.url().clone().into_string();
        if final_url != url {
            tt_note!(status, "resolved to {}", final_url);
        }

        Ok(final_url)
    }

    fn open_range_reader(&self, url: &str) -> Self::RangeReader {
        ReqwestRangeReader::new(url)
    }
}

/// A simple way to read chunks out of a big seekable byte stream. You could
/// implement this for io::File pretty trivially but that's not currently
/// needed.
#[derive(Clone, Debug)]
pub struct ReqwestRangeReader {
    url: String,
    client: Client,
}

impl ReqwestRangeReader {
    pub fn new(url: &str) -> ReqwestRangeReader {
        ReqwestRangeReader {
            url: url.to_owned(),
            client: Client::new(),
        }
    }
}

impl RangeReader for ReqwestRangeReader {
    type Response = Response;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Response> {
        let end_inclusive = offset + length as u64 - 1;

        let mut headers = HeaderMap::new();
        use headers::HeaderMapExt;
        headers.typed_insert(headers::Range::bytes(offset..=end_inclusive).unwrap());

        let res = self.client.get(&self.url).headers(headers).send()?;

        if res.status() != StatusCode::PARTIAL_CONTENT {
            //return UnexpectedHttpResponse(self.url.clone(), res.status())
            bail!("unexpected HTTP resonse code for URL {}", self.url);
        }

        Ok(res)
    }
}
