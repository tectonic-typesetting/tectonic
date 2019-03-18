// src/io/memory.rs -- I/O to in-memory buffers
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::rc::Rc;

use super::{
    normalize_tex_path, InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult,
    OutputHandle,
};
use errors::Result;
use status::StatusBackend;

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
    pub fn new(
        files: &Rc<RefCell<HashMap<OsString, Vec<u8>>>>,
        name: &OsStr,
        truncate: bool,
    ) -> MemoryIoItem {
        let cur = match files.borrow_mut().remove(name) {
            Some(data) => {
                if truncate {
                    Vec::new()
                } else {
                    data
                }
            }
            None => Vec::new(),
        };

        MemoryIoItem {
            files: files.clone(),
            name: name.to_os_string(),
            state: Cursor::new(cur),
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
            stdout_allowed,
        }
    }

    pub fn create_entry(&mut self, name: &OsStr, data: Vec<u8>) {
        let mut mfiles = self.files.borrow_mut();
        mfiles.insert(name.to_os_string(), data);
    }

    pub fn stdout_key(&self) -> &OsStr {
        OsStr::new("")
    }
}

impl IoProvider for MemoryIo {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        if name.is_empty() {
            return OpenResult::NotAvailable;
        }

        let name = normalize_tex_path(name);

        OpenResult::Ok(OutputHandle::new(
            &name,
            MemoryIoItem::new(&self.files, &name, true),
        ))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        if !self.stdout_allowed {
            return OpenResult::NotAvailable;
        }

        OpenResult::Ok(OutputHandle::new(
            self.stdout_key(),
            MemoryIoItem::new(&self.files, self.stdout_key(), true),
        ))
    }

    fn input_open_name(
        &mut self,
        name: &OsStr,
        _status: &mut StatusBackend,
    ) -> OpenResult<InputHandle> {
        if name.is_empty() {
            return OpenResult::NotAvailable;
        }

        let name = normalize_tex_path(name);

        if self.files.borrow().contains_key(&*name) {
            OpenResult::Ok(InputHandle::new(
                &name,
                MemoryIoItem::new(&self.files, &name, false),
                InputOrigin::Other,
            ))
        } else {
            OpenResult::NotAvailable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use status::NoopStatusBackend;
    use std::io::{BufRead, BufReader};

    /// Early versions had a bug where files were not truncated when opened
    /// for writing, which led to junk after the intended EOF when the engine
    /// ran for multiple passes and the file shrank from one pass to the next.
    #[test]
    fn shrinking_file() {
        let mut mem = MemoryIo::new(false);
        let name = OsStr::new("test.tex");
        let mut sb = NoopStatusBackend::new();

        // Write a line to a file, then (implicitly) close it.
        {
            let mut h = mem.output_open_name(name).unwrap();
            writeln!(h, "0123456789").unwrap();
        }

        // Reopen the file for input, then close it.
        {
            mem.input_open_name(name, &mut sb).unwrap();
        }

        // Open for input yet again; file should *not* have been truncated.
        {
            let h = mem.input_open_name(name, &mut sb).unwrap();
            let mut br = BufReader::new(h);
            let mut s = String::new();
            br.read_line(&mut s).unwrap();
            assert_eq!(s.len(), 11);
        }

        // Now open for output and write a shorter line.
        {
            let mut h = mem.output_open_name(name).unwrap();
            writeln!(h, "0123").unwrap();
        }

        // Open for input one last time; file should now have been truncated.
        {
            let h = mem.input_open_name(name, &mut sb).unwrap();
            let mut br = BufReader::new(h);
            let mut s = String::new();
            br.read_line(&mut s).unwrap();
            assert_eq!(s.len(), 5);
            s.clear();
            br.read_line(&mut s).unwrap();
            assert_eq!(s.len(), 0);
        }
    }
}
