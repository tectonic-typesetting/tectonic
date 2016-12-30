// src/io.rs -- input/output interface for Tectonic.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use errors::{Error, ErrorKind, Result};


// Input handles are Read with a few extras. We don't require the standard
// io::Seek because we need to provide a dummy implementation for GZip
// streams, which we wouldn't be allowed to do because both the trait and the
// target struct are outside of our crate.

pub trait InputFeatures: Read {
    fn get_size(&mut self) -> Result<usize>;
    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

pub type InputHandle = Box<InputFeatures>;


// Output handles only need Write.

pub type OutputHandle = Box<Write>;


// An IO provider is just a source of handles. One wrinkle is that it's good
// to be able to distinguish between unavailability of a given name and error
// accessing it.

pub enum OpenResult<T> {
    Ok(T),
    NotAvailable,
    Err(Error)
}

pub trait IOProvider {
    #[allow(unused_variables)]
    fn output_open_name(&mut self, name: &[u8]) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    #[allow(unused_variables)]
    fn input_open_name(&mut self, name: &[u8]) -> OpenResult<InputHandle> {
        OpenResult::NotAvailable
    }
}


// InputFeatures impls for the relevant types.

impl InputFeatures for File {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.metadata()?.len() as usize)
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

impl InputFeatures for Cursor<Vec<u8>> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_ref().len())
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

impl<R: Read> InputFeatures for GzDecoder<R> {
    fn get_size(&mut self) -> Result<usize> {
        Err(ErrorKind::NotSizeable.into())
    }

    fn try_seek(&mut self, _: SeekFrom) -> Result<u64> {
        Err(ErrorKind::NotSeekable.into())
    }
}
