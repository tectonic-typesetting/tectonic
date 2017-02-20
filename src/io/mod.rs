// src/io/mod.rs -- input/output interfaces for Tectonic.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

use errors::{Error, ErrorKind, Result};

pub mod filesystem;
pub mod genuine_stdout;
//pub mod hyper_seekable; -- Not currently used, but nice code to keep around.
pub mod itarbundle;
pub mod local_cache;
pub mod memory;
pub mod stack;
pub mod zipbundle;


// Input handles are Read with a few extras. We don't require the standard
// io::Seek because we need to provide a dummy implementation for GZip
// streams, which we wouldn't be allowed to do because both the trait and the
// target struct are outside of our crate.
//
// Output handles are just Write.

pub trait InputFeatures: Read {
    fn get_size(&mut self) -> Result<usize>;
    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

pub type InputHandle = Box<InputFeatures>;
pub type OutputHandle = Box<Write>;


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

pub trait IoProvider {
    fn output_open_name(&mut self, _name: &OsStr) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, _name: &OsStr) -> OpenResult<InputHandle> {
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

        fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
            if name == self.name {
                OpenResult::Ok(Box::new(File::open(&self.full_path).unwrap()))
            } else {
                OpenResult::NotAvailable
            }
        }
    }
}
