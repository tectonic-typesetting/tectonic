// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! A URL-get backend based on the `curl` crate.

use curl::easy::Easy;
use std::io::Cursor;
use tectonic_errors::{anyhow::bail, Result};
use tectonic_status_base::StatusBackend;

use crate::{GetUrlBackend, RangeReader};

const MAX_HTTP_REDIRECTS_ALLOWED: u32 = 10;

fn get_url_generic(
    handle: &mut Easy,
    url: &str,
    range: Option<(u64, usize)>,
) -> Result<Cursor<Vec<u8>>> {
    handle.url(url)?;
    handle.follow_location(true)?;
    handle.max_redirections(MAX_HTTP_REDIRECTS_ALLOWED)?;

    if let Some((start, length)) = range {
        let end = start + length as u64 - 1;
        handle.range(&format!("{}-{}", start, end))?;
    }

    let mut buf = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    let code = handle.response_code()?;

    if !(200..300).contains(&code) {
        bail!(
            "unsuccessful HTTP GET status code {} for url `{}`",
            code,
            url
        );
    }

    Ok(Cursor::new(buf))
}

/// URL-get backend implemented using the `curl` crate.
#[derive(Debug)]
pub struct CurlBackend {
    handle: Easy,
}

impl Default for CurlBackend {
    fn default() -> Self {
        CurlBackend {
            handle: Easy::new(),
        }
    }
}

impl GetUrlBackend for CurlBackend {
    type Response = Cursor<Vec<u8>>;
    type RangeReader = CurlRangeReader;

    fn get_url(&mut self, url: &str, _status: &mut dyn StatusBackend) -> Result<Self::Response> {
        get_url_generic(&mut self.handle, url, None)
    }

    fn resolve_url(&mut self, url: &str, _status: &mut dyn StatusBackend) -> Result<String> {
        Ok(url.into())
    }

    fn open_range_reader(&self, url: &str) -> Self::RangeReader {
        CurlRangeReader::new(url)
    }
}

/// Curl-based byte-range reader.
#[derive(Debug)]
pub struct CurlRangeReader {
    url: String,
    handle: Easy,
}

impl CurlRangeReader {
    fn new(url: &str) -> CurlRangeReader {
        CurlRangeReader {
            url: url.to_owned(),
            handle: Easy::new(),
        }
    }
}

impl RangeReader for CurlRangeReader {
    type Response = Cursor<Vec<u8>>;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Self::Response> {
        get_url_generic(&mut self.handle, &self.url, Some((offset, length)))
    }
}
