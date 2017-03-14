// src/io/memory.rs -- I/O to in-memory buffers
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::rc::Rc;

use errors::Result;
use status::StatusBackend;
use super::{InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle};


// MemoryIo is an IoProvider that stores "files" in in-memory buffers.
//
// When a file is "opened", we create a MemoryIoItem struct that tracks the
// data, seek cursor state, etc.

struct MemoryIoItem {
    // TODO: smarter buffering structure than Vec<u8>? E.g., linked list of 4k
    // chunks or something. In the current scheme reallocations will get
    // expensive.
    files: Rc<RefCell<HashMap<OsString, Vec<u8>>>>,
    name: OsString,
    state: Cursor<Vec<u8>>,
}


impl MemoryIoItem {
    pub fn new(files: &Rc<RefCell<HashMap<OsString, Vec<u8>>>>, name: &OsStr) -> MemoryIoItem {
        let cur = match files.borrow_mut().remove(name) {
            Some(data) => data,
            None => Vec::new(),
        };

        MemoryIoItem {
            files: files.clone(),
            name: name.to_os_string(),
            state: Cursor::new(cur)
        }
    }
}

impl Read for MemoryIoItem {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.state.read(buf)
    }
}

impl Write for MemoryIoItem {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.state.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.state.flush()
    }
}

impl Seek for MemoryIoItem {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.state.seek(pos)
    }
}

impl InputFeatures for MemoryIoItem {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.state.get_ref().len())
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.state.seek(pos)?)
    }
}

impl Drop for MemoryIoItem {
    fn drop(&mut self) {
        // I think split_off() is an efficient way to move our data vector
        // back into the hashmap? Ideally we could "consume" self but I don't
        // believe that's possible in a Drop implementation.
        let mut mfiles = self.files.borrow_mut();
        mfiles.insert(self.name.clone(), self.state.get_mut().split_off(0));
    }
}


pub struct MemoryIo {
    pub files: Rc<RefCell<HashMap<OsString, Vec<u8>>>>,
    stdout_allowed: bool,
}

impl MemoryIo {
    pub fn new(stdout_allowed: bool) -> MemoryIo {
        MemoryIo {
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

impl IoProvider for MemoryIo {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        assert!(name.len() > 0, "name must be non-empty");
        OpenResult::Ok(OutputHandle::new(name, MemoryIoItem::new(&self.files, name)))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        if !self.stdout_allowed {
            return OpenResult::NotAvailable;
        }

        OpenResult::Ok(OutputHandle::new(self.stdout_key(), MemoryIoItem::new(&self.files, self.stdout_key())))
    }

    fn input_open_name(&mut self, name: &OsStr, _status: &mut StatusBackend) -> OpenResult<InputHandle> {
        assert!(name.len() > 0, "name must be non-empty");

        if self.files.borrow().contains_key(name) {
            OpenResult::Ok(InputHandle::new(name, MemoryIoItem::new(&self.files, name), InputOrigin::Other))
        } else {
            OpenResult::NotAvailable
        }
    }
}
