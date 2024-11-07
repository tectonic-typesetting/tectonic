// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic I/O implementations for types provided by `flate2`.
//!
//! Due to Rust's coherence rules, such implementations must be provided in this
//! crate, unless we want to get into creating wrapper types.

use flate2::read::GzDecoder;
use std::io::{Read, SeekFrom};
use tectonic_errors::Result;

use super::{InputFeatures, TectonicIoError};

impl<R: Read> InputFeatures for GzDecoder<R> {
    fn get_size(&mut self) -> Result<usize> {
        Err(TectonicIoError::NotSizeable.into())
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        // In principle we could arrange to potentially get an mtime from the
        // underlying stream, but this API is only used for the \filemodtime
        // primitive which shouldn't be getting access to gzipped streams.
        Ok(None)
    }

    fn try_seek(&mut self, _: SeekFrom) -> Result<u64> {
        Err(TectonicIoError::NotSeekable.into())
    }
}
