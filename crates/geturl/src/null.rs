// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! A geturl backend that always fails.

use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    io::Empty,
    result::Result as StdResult,
};
use tectonic_errors::Result;
use tectonic_status_base::StatusBackend;

use crate::{GetUrlBackend, RangeReader};

/// The error type for the always-failing geturl backend.
#[derive(Debug)]
pub struct NoGetUrlBackendError {}

impl Display for NoGetUrlBackendError {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(f, "no get-URL backend was enabled")
    }
}

impl Error for NoGetUrlBackendError {}

/// The "null" URL-get backend, which always fails.
#[derive(Debug, Default)]
pub struct NullBackend {}

impl GetUrlBackend for NullBackend {
    type Response = Empty;
    type RangeReader = NullRangeReader;

    fn get_url(&mut self, _url: &str, _status: &mut dyn StatusBackend) -> Result<Empty> {
        Err((NoGetUrlBackendError {}).into())
    }

    fn resolve_url(&mut self, _url: &str, _status: &mut dyn StatusBackend) -> Result<String> {
        Err((NoGetUrlBackendError {}).into())
    }

    fn open_range_reader(&self, _url: &str) -> Self::RangeReader {
        NullRangeReader {}
    }
}

/// The "null" URL-get range reader, which always fails.
#[derive(Debug)]
pub struct NullRangeReader {}

impl RangeReader for NullRangeReader {
    type Response = Empty;

    fn read_range(&mut self, _offset: u64, _length: usize) -> Result<Empty> {
        Err((NoGetUrlBackendError {}).into())
    }
}
