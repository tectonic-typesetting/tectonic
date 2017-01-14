// src/io.rs -- input/output interface for Tectonic.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, stdout, BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

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
// accessing it. We take file paths as OsStrs, although since we parse input
// files as Unicode it may not be possible to actually express zany
// non-Unicode Unix paths inside the engine.

pub enum OpenResult<T> {
    Ok(T),
    NotAvailable,
    Err(Error)
}

pub trait IOProvider {
    #[allow(unused_variables)]
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    #[allow(unused_variables)]
    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        OpenResult::NotAvailable
    }
}


// An IOStack is an IOProvider that just delegates to an ordered list of
// subordinate IOProviders.

pub struct IOStack<'a> {
    items: &'a mut [&'a mut IOProvider],
}


impl<'a> IOStack<'a> {
    pub fn new(items: &'a mut [&'a mut IOProvider]) -> IOStack<'a> {
        IOStack { items: items }
    }
}

impl<'a> IOProvider for IOStack<'a> {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
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

    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
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
    absolute_allowed: bool,
    root: PathBuf
}

impl FilesystemIO {
    pub fn new(root: &Path, writes_allowed: bool, absolute_allowed: bool) -> FilesystemIO {
        FilesystemIO {
            writes_allowed: writes_allowed,
            absolute_allowed: absolute_allowed,
            root: PathBuf::from(root),
        }
    }

    fn construct_path(&mut self, name: &OsStr) -> Result<PathBuf> {
        let path = Path::new(name);

        if path.is_absolute() && !self.absolute_allowed {
            let as_str = String::from(path.to_string_lossy());
            return Err(ErrorKind::PathForbidden(as_str).into());
        }

        let mut combined = PathBuf::from(&self.root);
        combined.push(path);
        Ok(combined)
    }
}

impl IOProvider for FilesystemIO {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
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

    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
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

        OpenResult::Ok(Box::new(BufReader::new(f)))
    }
}


// MemoryIO is an IOProvider that stores "files" in in-memory buffers.
//
// When a file is "opened", we create a MemoryIOItem struct that tracks the
// data, seek cursor state, etc.

struct MemoryIOItem {
    // TODO: smarter buffering structure than Vec<u8>? E.g., linked list of 4k
    // chunks or something. In the current scheme reallocations will get
    // expensive.
    files: Rc<RefCell<HashMap<OsString, Vec<u8>>>>,
    name: OsString,
    state: Cursor<Vec<u8>>,
}

impl MemoryIOItem {
    pub fn new(files: &Rc<RefCell<HashMap<OsString, Vec<u8>>>>, name: &OsStr) -> MemoryIOItem {
        let cur = match files.borrow_mut().remove(name) {
            Some(data) => data,
            None => Vec::new(),
        };

        MemoryIOItem {
            files: files.clone(),
            name: name.to_os_string(),
            state: Cursor::new(cur)
        }
    }
}

impl Read for MemoryIOItem {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.state.read(buf)
    }
}

impl Write for MemoryIOItem {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.state.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.state.flush()
    }
}

impl Seek for MemoryIOItem {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.state.seek(pos)
    }
}

impl InputFeatures for MemoryIOItem {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.state.get_ref().len())
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.state.seek(pos)?)
    }
}

impl Drop for MemoryIOItem {
    fn drop(&mut self) {
        // I think split_off() is an efficient way to move our data vector
        // back into the hashmap? Ideally we could "consume" self but I don't
        // believe that's possible in a Drop implementation.
        let mut mfiles = self.files.borrow_mut();
        mfiles.insert(self.name.clone(), self.state.get_mut().split_off(0));
    }
}


pub struct MemoryIO {
    pub files: Rc<RefCell<HashMap<OsString, Vec<u8>>>>,
    stdout_allowed: bool,
}

impl MemoryIO {
    pub fn new(stdout_allowed: bool) -> MemoryIO {
        MemoryIO {
            files: Rc::new(RefCell::new(HashMap::new())),
            stdout_allowed: stdout_allowed,
        }
    }

    pub fn create_entry(&mut self, name: &OsStr, data: Vec<u8>) {
        let mut mfiles = self.files.borrow_mut();
        mfiles.insert(name.to_os_string(), data);
    }

    pub fn stdout_key(& self) -> &OsStr {
        OsStr::new("")
    }
}

impl IOProvider for MemoryIO {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        assert!(name.len() > 0, "name must be non-empty");
        OpenResult::Ok(Box::new(MemoryIOItem::new(&self.files, name)))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        if !self.stdout_allowed {
            return OpenResult::NotAvailable;
        }

        OpenResult::Ok(Box::new(MemoryIOItem::new(&self.files, self.stdout_key())))
    }

    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        assert!(name.len() > 0, "name must be non-empty");

        if self.files.borrow().contains_key(name) {
            OpenResult::Ok(Box::new(MemoryIOItem::new(&self.files, name)))
        } else {
            OpenResult::NotAvailable
        }
    }
}


// GenuineStdoutIO provides a mechanism for the "stdout" output to actually go
// to the process's stdout.

pub struct GenuineStdoutIO {}

impl GenuineStdoutIO {
    pub fn new() -> GenuineStdoutIO {
        GenuineStdoutIO {}
    }
}

impl IOProvider for GenuineStdoutIO {
    fn output_open_name(&mut self, _: &OsStr) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::Ok(Box::new(stdout()))
    }

    fn input_open_name(&mut self, _: &OsStr) -> OpenResult<InputHandle> {
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

impl InputFeatures for BufReader<File> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_mut().metadata()?.len() as usize)
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


// Helper for testing. FIXME: I want this to be conditionally compiled with
// #[cfg(test)] but things break if I do that.

pub mod testing {
    use std::ffi::{OsStr, OsString};
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use super::*;

    pub struct SingleInputFileIO {
        name: OsString,
        full_path: PathBuf
    }

    impl SingleInputFileIO {
        pub fn new(path: &Path) -> SingleInputFileIO {
            let p = path.to_path_buf();

            SingleInputFileIO {
                name: p.file_name().unwrap().to_os_string(),
                full_path: p,
            }
        }
    }

    impl IOProvider for SingleInputFileIO {
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
