// src/io/mod.rs -- input/output interfaces for Tectonic.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

use digest::{self, Digest, DigestData};
use errors::{Error, ErrorKind, Result};
use status::StatusBackend;

pub mod filesystem;
pub mod genuine_stdout;
//pub mod hyper_seekable; -- Not currently used, but nice code to keep around.
pub mod itarbundle;
pub mod local_cache;
pub mod memory;
pub mod stack;
pub mod zipbundle;



pub trait InputFeatures: Read {
    fn get_size(&mut self) -> Result<usize>;
    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64>;
}


/// What kind of source an input file ultimately came from. We keep track of
/// this in order to be able to emit Makefile-style dependencies for input
/// files. Right now, we only provide enough options to achieve this goal; we
/// could add more.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum InputOrigin {
    /// This file lives on the filesystem and might change under us. (That is
    /// it is not a cached bundle file.)
    Filesystem,

    /// This file was never used as an input.
    NotInput,

    /// This file is none of the above.
    Other,
}


/// Input handles are basically Read objects with a few extras. We don't
/// require the standard io::Seek because we need to provide a dummy
/// implementation for GZip streams, which we wouldn't be allowed to do
/// because both the trait and the target struct are outside of our crate.
///
/// An important role for the InputHandle struct is computing a cryptographic
/// digest of the input file. The driver uses this information in order to
/// figure out if the TeX engine needs rerunning. TeX makes our life more
/// difficult, though, since it has somewhat funky file access patterns. LaTeX
/// file opens work by opening a file and immediately closing it, which tests
/// whether the file exists, and then by opening it again for real. Under the
/// hood, XeTeX reads a couple of bytes from each file upon open to sniff its
/// encoding. So we can't just stream data from `read()` calls into the SHA2
/// computer, since we end up seeking and reading redundant data.
///
/// The current system maintains some internal state that, so far, helps us Do
/// The Right Thing given all this. If there's a seek on the file, we give up
/// on our digest computation. But if there's a seek back to the file
/// beginning, we are open to the possibility of restarting the computation.
/// But if nothing is ever read from the file, we once again give up on the
/// computation. The `ExecutionState` code then has further pieces that track
/// access to nonexistent files, which we treat as being equivalent to an
/// existing empty file for these purposes.

pub struct InputHandle {
    name: OsString,
    inner: Box<InputFeatures>,
    digest: digest::DigestComputer,
    origin: InputOrigin,
    ever_read: bool,
    did_unhandled_seek: bool,
    ungetc_char: Option<u8>,
}


impl InputHandle {
    pub fn new<T: 'static + InputFeatures>(name: &OsStr, inner: T, origin: InputOrigin) -> InputHandle {
        InputHandle {
            name: name.to_os_string(),
            inner: Box::new(inner),
            digest: Default::default(),
            origin: origin,
            ever_read: false,
            did_unhandled_seek: false,
            ungetc_char: None,
        }
    }

    pub fn name(&self) -> &OsStr {
        self.name.as_os_str()
    }

    pub fn origin(&self) -> InputOrigin {
        self.origin
    }

    /// Consumes the object and returns the underlying readable handle that
    /// it references.
    pub fn into_inner(self) -> Box<InputFeatures> {
        self.inner
    }

    /// Consumes the object and returns the SHA256 sum of the content that was
    /// written. No digest is returned if there was ever a seek on the input
    /// stream, since in that case the results will not be reliable. We also
    /// return None if the stream was never read, which is another common
    /// TeX access pattern: files are opened, immediately closed, and then
    /// opened again.
    pub fn into_name_digest(self) -> (OsString, Option<DigestData>) {
        if self.did_unhandled_seek || !self.ever_read {
            (self.name, None)
        } else {
            (self.name, Some(DigestData::from(self.digest)))
        }
    }

    /// Various piece of TeX want to use the libc `ungetc()` function a lot.
    /// It's kind of gross, but happens often enough that we provide special
    /// support for it. Here's `getc()` emulation that can return a previously
    /// `ungetc()`-ed character.
    pub fn getc(&mut self) -> Result<u8> {
        if let Some(c) = self.ungetc_char {
            self.ungetc_char = None;
            return Ok(c);
        }

        let mut byte = [0u8; 1];

        if self.read(&mut byte[..1])? == 0 {
            // EOF
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF in getc").into());
        }


        Ok(byte[0])
    }

    /// Here's the `ungetc()` emulation.
    pub fn ungetc(&mut self, byte: u8) -> Result<()> {
        if self.ungetc_char.is_some() {
            return Err(ErrorKind::Msg("internal problem: cannot ungetc() more than once in a row".into()).into());
        }

        self.ungetc_char = Some(byte);
        Ok(())
    }
}

impl Read for InputHandle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() > 0 {
            if let Some(c) = self.ungetc_char {
                // This does sometimes happen, so we need to deal with it. It's not that bad really.
                buf[0] = c;
                self.ungetc_char = None;
                return Ok(self.read(&mut buf[1..])? + 1);
            }
        }

        self.ever_read = true;
        let n = self.inner.read(buf)?;
        self.digest.input(&buf[..n]);
        Ok(n)
    }
}

impl InputFeatures for InputHandle {
    fn get_size(&mut self) -> Result<usize> {
        self.inner.get_size()
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(0) => {
                // As describe above, there is a common pattern in TeX file
                // accesses: read a few bytes to sniff, then go back to the
                // beginning. We should tidy up the I/O to just buffer instead
                // of seeking, but in the meantime, we can handle this.
                self.digest = Default::default();
                self.ever_read = false;
            }
            SeekFrom::Current(0) => {
                // Noop.
            },
            _ => {
                self.did_unhandled_seek = true;
            }
        }
        self.inner.try_seek(pos)
    }
}


pub struct OutputHandle {
    name: OsString,
    inner: Box<Write>,
    digest: digest::DigestComputer,
}


impl OutputHandle {
    pub fn new<T: 'static + Write>(name: &OsStr, inner: T) -> OutputHandle {
        OutputHandle {
            name: name.to_os_string(),
            inner: Box::new(inner),
            digest: digest::create(),
        }
    }

    pub fn name(&self) -> &OsStr {
        self.name.as_os_str()
    }

    /// Consumes the object and returns the underlying writable handle that
    /// it references.
    pub fn into_inner(self) -> Box<Write> {
        self.inner
    }

    /// Consumes the object and returns the SHA256 sum of the content that was
    /// written.
    pub fn into_name_digest(self) -> (OsString, DigestData) {
        (self.name, DigestData::from(self.digest))
    }
}

impl Write for OutputHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.digest.input(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}


// An Io provider is a source of handles. One wrinkle is that it's good to be
// able to distinguish between unavailability of a given name and error
// accessing it. We take file paths as OsStrs, although since we parse input
// files as Unicode it may not be possible to actually express zany
// non-Unicode Unix paths inside the engine.

pub enum OpenResult<T> {
    Ok(T),
    NotAvailable,
    Err(Error)
}


impl<T> OpenResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            OpenResult::Ok(t) => t,
            _ => panic!("expected an open file"),
        }
    }
}


pub trait IoProvider {
    fn output_open_name(&mut self, _name: &OsStr) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, _name: &OsStr, _status: &mut StatusBackend) -> OpenResult<InputHandle> {
        OpenResult::NotAvailable
    }
}


// Some generically helpful InputFeatures impls

impl<R: Read> InputFeatures for GzDecoder<R> {
    fn get_size(&mut self) -> Result<usize> {
        Err(ErrorKind::NotSizeable.into())
    }

    fn try_seek(&mut self, _: SeekFrom) -> Result<u64> {
        Err(ErrorKind::NotSeekable.into())
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


// Reexports

pub use self::filesystem::FilesystemIo;
pub use self::genuine_stdout::GenuineStdoutIo;
pub use self::memory::MemoryIo;
pub use self::stack::IoStack;


// Helpful.

pub fn try_open_file(path: &Path) -> OpenResult<File> {
    use std::io::ErrorKind::NotFound;

    match File::open(path) {
        Ok(f) => OpenResult::Ok(f),
        Err(e) => {
            if e.kind() == NotFound {
                OpenResult::NotAvailable
            } else {
                OpenResult::Err(e.into())
            }
        },
    }
}


// Helper for testing. FIXME: I want this to be conditionally compiled with
// #[cfg(test)] but things break if I do that.

pub mod testing {
    use std::ffi::{OsStr, OsString};
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use super::*;

    pub struct SingleInputFileIo {
        name: OsString,
        full_path: PathBuf
    }

    impl SingleInputFileIo {
        pub fn new(path: &Path) -> SingleInputFileIo {
            let p = path.to_path_buf();

            SingleInputFileIo {
                name: p.file_name().unwrap().to_os_string(),
                full_path: p,
            }
        }
    }

    impl IoProvider for SingleInputFileIo {
        fn output_open_name(&mut self, _: &OsStr) -> OpenResult<OutputHandle> {
            OpenResult::NotAvailable
        }

        fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
            OpenResult::NotAvailable
        }

        fn input_open_name(&mut self, name: &OsStr, _status: &mut StatusBackend) -> OpenResult<InputHandle> {
            if name == self.name {
                OpenResult::Ok(InputHandle::new(name, File::open(&self.full_path).unwrap(), InputOrigin::Filesystem))
            } else {
                OpenResult::NotAvailable
            }
        }
    }
}
