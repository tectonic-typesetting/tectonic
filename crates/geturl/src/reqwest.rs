// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

//! A URL-get backend based on the `reqwest` crate.

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, RANGE},
    StatusCode,
};
use tectonic_errors::{anyhow::bail, Result};
use tectonic_status_base::StatusBackend;

use crate::{GetUrlBackend, RangeReader};

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
        let header_val = format!("bytes={offset}-{end_inclusive}").parse()?;

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
