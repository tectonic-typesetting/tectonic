use std::fmt;
use std::io;

use crate::errors::Result;
use crate::status::StatusBackend;
use curl::easy::Easy;

pub use curl::Error;

const MAX_HTTP_REDIRECTS_ALLOWED: u32 = 10;

#[derive(Debug)]
pub struct Client {
    handle: Easy,
}

pub struct Response {
    data: io::Cursor<Vec<u8>>,
    status: StatusCode,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StatusCode(u32);

impl Default for Client {
    fn default() -> Client {
        Client {
            handle: Easy::new(),
        }
    }
}

impl Client {
    pub fn new() -> Client {
        Client::default()
    }

    fn get(&mut self, url: &str, range: Option<(u64, u64)>) -> Result<Response> {
        let handle = &mut self.handle;
        handle.url(url)?;
        handle.follow_location(true)?;
        handle.max_redirections(MAX_HTTP_REDIRECTS_ALLOWED)?;
        if let Some((start, end)) = range {
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
        let data = io::Cursor::new(buf);
        let status = StatusCode(handle.response_code()?);
        Ok(Response { data, status })
    }
}

impl Response {
    pub fn status(&self) -> StatusCode {
        self.status
    }
}

impl io::Read for Response {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.data.read(buf)
    }
}

impl StatusCode {
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(206);

    pub fn is_success(self) -> bool {
        200 <= self.0 && self.0 <= 299
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "StatusCode({})", self.0)
    }
}

pub fn get(url: &str) -> Result<Response> {
    Client::new().get(url, None)
}

pub fn get_range_inclusive(
    client: &mut Client,
    url: &str,
    start: u64,
    end: u64,
) -> Result<Response> {
    client.get(url, Some((start, end)))
}

pub fn resolve_url(url: &str, _status: &mut dyn StatusBackend) -> Result<String> {
    Ok(url.into())
}
