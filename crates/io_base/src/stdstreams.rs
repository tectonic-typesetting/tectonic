// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic I/O trait implementations for the standard I/O streams.

use std::{
    io::{stdin, stdout, Cursor, Read, Seek, SeekFrom},
    rc::Rc,
};
use tectonic_errors::Result;
use tectonic_status_base::StatusBackend;

use super::{InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle};

/// GenuineStdoutIo provides a mechanism for the "stdout" output to actually
/// go to the process's stdout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GenuineStdoutIo {}

impl GenuineStdoutIo {
    /// Create a new object.
    ///
    /// TODO: this is redundant with `Default::default`.
    pub fn new() -> GenuineStdoutIo {
        Default::default()
    }
}

impl IoProvider for GenuineStdoutIo {
    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        // NOTE: keep in sync with io::memory::MemoryIo::stdout_key()
        OpenResult::Ok(OutputHandle::new("", stdout()))
    }
}

/// This helper type is needed to get full InputFeatures functionality on a
/// shared, ref-counted Vec<u8>: we're not allowed to implement AsRef<[u8]> on
/// Rc<Vec<u8>> since none of the types or traits come from the Tectonic
/// crate.
#[derive(Clone, Debug, Eq, PartialEq)]
struct SharedByteBuffer(Rc<[u8]>);

impl SharedByteBuffer {
    fn new(data: Vec<u8>) -> SharedByteBuffer {
        SharedByteBuffer(data.into())
    }
}

impl AsRef<[u8]> for SharedByteBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl InputFeatures for Cursor<SharedByteBuffer> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_ref().0.len())
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        Ok(None)
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

/// BufferedPrimaryIo provides a mechanism for the TeX "primary input"
/// to come from stdin. Because Tectonic makes multiple passes through the
/// input by default, we have to buffer it in memory so that the input can be
/// read multiple times. It wouldn't be hard to make an alternative
/// implementation that skips the buffering and errors out if one tries to
/// open the stream more than once.
///
/// TODO: it might be better to stream stdin to a temporary file on disk that
/// we then delete while holding on to the file handle. But mkstemp-rs doesn't
/// give us Files and the whole approach might get a bit hairy, so we don't do
/// that.
///
/// TODO: it also would be nicer to actually stream through stdin at pace on
/// the first pass rather than slurping it all into memory upon construction,
/// but once more we're being lazy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferedPrimaryIo {
    buffer: SharedByteBuffer,
}

impl BufferedPrimaryIo {
    /// Create a new primary-I/O buffer from a type implementing Read.
    ///
    /// The stream will be read and buffered in memory — regardless of how
    /// large it is. This approach is required because Tectonic will generally
    /// need to make multiple passes over the input file.
    pub fn from_stream<T: Read>(stream: &mut T) -> Result<Self> {
        let mut buf = [0u8; 8192];
        let mut alldata = Vec::<u8>::new();

        loop {
            let nbytes = stream.read(&mut buf)?;

            if nbytes == 0 {
                break;
            }

            alldata.extend_from_slice(&buf[..nbytes]);
        }

        Ok(BufferedPrimaryIo {
            buffer: SharedByteBuffer::new(alldata),
        })
    }

    /// Create a new primary-I/O buffer from this processes's standard input.
    ///
    /// Standard input will be read and buffered in memory — regardless of how
    /// large it is. This approach is required because Tectonic will generally
    /// need to make multiple passes over the input file.
    pub fn from_stdin() -> Result<Self> {
        Self::from_stream(&mut stdin())
    }

    /// Create a new primary-I/O buffer from a string.
    ///
    /// The string is converted into bytes as per [`str.as_bytes`].
    pub fn from_text<T: AsRef<str>>(text: T) -> Self {
        BufferedPrimaryIo {
            buffer: SharedByteBuffer::new(text.as_ref().as_bytes().to_owned()),
        }
    }

    /// Create a new primary-I/O buffer from a byte vector.
    pub fn from_buffer(buf: Vec<u8>) -> Self {
        BufferedPrimaryIo {
            buffer: SharedByteBuffer::new(buf),
        }
    }
}

impl IoProvider for BufferedPrimaryIo {
    fn input_open_primary(&mut self, _status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        OpenResult::Ok(InputHandle::new(
            "",
            Cursor::new(self.buffer.clone()),
            InputOrigin::Other,
        ))
    }
}
