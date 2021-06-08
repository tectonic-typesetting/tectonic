// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

//! A URL-get backend based on the `reqwest` crate.

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, RANGE},
    redirect::Policy,
    StatusCode,
};
use tectonic_errors::{anyhow::bail, Result};
use tectonic_status_base::{tt_note, StatusBackend};

use crate::{GetUrlBackend, RangeReader};

const MAX_HTTP_REDIRECTS_ALLOWED: usize = 10;

/// URL-get backend implemented using the `reqwest` crate.
#[derive(Debug, Default)]
pub struct ReqwestBackend {}

impl GetUrlBackend for ReqwestBackend {
    type Response = Response;
    type RangeReader = ReqwestRangeReader;

    fn get_url(&mut self, url: &str, _status: &mut dyn StatusBackend) -> Result<Response> {
        let res = Client::new().get(url).send()?;
        if !res.status().is_success() {
            bail!(
                "unexpected HTTP response code {} for URL {}",
                res.status(),
                url
            );
        }
        Ok(res)
    }

    fn resolve_url(&mut self, url: &str, status: &mut dyn StatusBackend) -> Result<String> {
        tt_note!(status, "connecting to {}", url);

        // First, we actually do a HEAD request on the URL for the data file.
        // If it's redirected, we update our URL to follow the redirects. If
        // we didn't do this separately, the index file would have to be the
        // one with the redirect setup, which would be confusing and annoying.
        let redirect_policy = Policy::custom(|attempt| {
            // In the process of resolving the file url it might be necessary
            // to stop at a certain level of redirection. This might be required
            // because some hosts might redirect to a version of the url where
            // it isn't possible to select the index file by appending .index.gz.
            // (This mostly happens because CDNs redirect to the file hash.)
            if attempt.previous().len() >= MAX_HTTP_REDIRECTS_ALLOWED {
                attempt.error("too many redirections")
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
            bail!(
                "unexpected HTTP response code {} for URL {}",
                res.status(),
                url
            );
        }

        let final_url: String = res.url().clone().into();
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
#[derive(Debug)]
pub struct ReqwestRangeReader {
    url: String,
    client: Client,
}

impl ReqwestRangeReader {
    fn new(url: &str) -> ReqwestRangeReader {
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
        let header_val = format!("bytes={}-{}", offset, end_inclusive).parse()?;

        let mut headers = HeaderMap::new();
        headers.insert(RANGE, header_val);

        let res = self.client.get(&self.url).headers(headers).send()?;

        if res.status() != StatusCode::PARTIAL_CONTENT {
            bail!(
                "unexpected HTTP response code {} for URL {}",
                res.status(),
                self.url
            );
        }

        Ok(res)
    }
}
