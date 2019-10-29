use std::error;
use std::fmt;
use std::io;

use crate::errors::Result;
use crate::status::StatusBackend;

#[derive(Default, Debug)]
pub struct Client {}

pub struct Response {}

#[derive(Debug)]
pub struct Error {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StatusCode(u32);

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Tectonic was compiled without a download backend.")
    }
}

impl Client {
    pub fn new() -> Client {
        Client {}
    }
}

impl Response {
    pub fn status(&self) -> StatusCode {
        unreachable!()
    }
}

impl io::Read for Response {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unreachable!()
    }
}

impl StatusCode {
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(206);

    pub fn is_success(self) -> bool {
        unreachable!()
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

pub fn get(_url: &str) -> Result<Response> {
    Err(Error {}.into())
}

pub fn get_range_inclusive(
    _client: &mut Client,
    _url: &str,
    _start: u64,
    _end: u64,
) -> Result<Response> {
    Err(Error {}.into())
}

pub fn resolve_url(_url: &str, _status: &mut dyn StatusBackend) -> Result<String> {
    Err(Error {}.into())
}
