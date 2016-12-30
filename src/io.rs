// src/io.rs -- input/output interface for Tectonic.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

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


// An IOStack is an IOProvider that just delegates to an ordered list of
// subordinate IOProviders. We take file paths as u8 vectors since Unix file
// paths can be arbitrary nul-terminated strings -- i.e., they are not
// necessarily UTF8, so &str is too restrictive. And we'll do our best to
// allow such zany paths, although since we parse input files as Unicode it
// may not be possible to actually express such paths inside the engine.

pub struct IOStack {
    items: Vec<Box<IOProvider>>,
}


impl IOStack {
    pub fn new(items: Vec<Box<IOProvider>>) -> IOStack {
        IOStack { items: items }
    }
}

impl IOProvider for IOStack {
    fn output_open_name(&mut self, name: &[u8]) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_stdout();

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, name: &[u8]) -> OpenResult<InputHandle> {
        for item in self.items.iter_mut() {
            let r = item.input_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }
}


// FilesystemIO is an I/O provider that reads, and optionally writes, files
// from a given root directory. NOTE: no effort is made to contain I/O within
// the specified root!! We disallow absolute paths, but we don't do anything
// about "../../../...." paths.

pub struct FilesystemIO {
    writes_allowed: bool,
    root: PathBuf
}

impl FilesystemIO {
    pub fn new(root: &Path, writes_allowed: bool) -> FilesystemIO {
        FilesystemIO {
            writes_allowed: writes_allowed,
            root: PathBuf::from(root),
        }
    }

    fn construct_path(&mut self, name: &[u8]) -> Result<PathBuf> {
        let path = Path::new(OsStr::from_bytes(name));

        if path.is_absolute() {
            let as_str = String::from(path.to_string_lossy());
            return Err(ErrorKind::PathForbidden(as_str).into());
        }

        let mut combined = PathBuf::from(&self.root);
        combined.push(path);
        Ok(combined)
    }
}

impl IOProvider for FilesystemIO {
    fn output_open_name(&mut self, name: &[u8]) -> OpenResult<OutputHandle> {
        if !self.writes_allowed {
            return OpenResult::NotAvailable;
        }

        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e.into())
        };

        let f = match File::create (path) {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into())
        };

        OpenResult::Ok(Box::new(f))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        // TODO: option to record "stdout" into a file with a particular name?
        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, name: &[u8]) -> OpenResult<InputHandle> {
        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e.into())
        };

        let f = match File::open (path) {
            Ok(f) => f,
            Err(e) => return if e.kind() == io::ErrorKind::NotFound {
                OpenResult::NotAvailable
            } else {
                OpenResult::Err(e.into())
            }
        };

        OpenResult::Ok(Box::new(f))
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
