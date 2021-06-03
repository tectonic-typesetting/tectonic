// src/io/memory.rs -- I/O to in-memory buffers
// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! MemoryIo is an IoProvider that stores "files" in in-memory buffers.

use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    rc::Rc,
    time::SystemTime,
};
use tectonic_errors::Result;
use tectonic_status_base::StatusBackend;

use super::{
    normalize_tex_path, InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult,
    OutputHandle,
};

/// Information about a file created or used inside the memory-backed I/O
/// provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryFileInfo {
    // TODO: smarter buffering structure than Vec<u8>? E.g., linked list of 4k
    // chunks or something. In the current scheme reallocations will get
    // expensive.
    pub data: Vec<u8>,
    pub unix_mtime: Option<i64>,
}

/// A collection of files created or used inside a memory-backed I/O provider.
pub type MemoryFileCollection = HashMap<String, MemoryFileInfo>;

/// When a file is "opened", we create a MemoryIoItem struct that tracks the
/// data, seek cursor state, etc.
struct MemoryIoItem {
    // This is the best way I can come up with to allow the file object to
    // update its data in its parent data structure.
    files: Rc<RefCell<MemoryFileCollection>>,

    name: String,
    state: Cursor<Vec<u8>>,
    unix_mtime: Option<i64>,
    was_modified: bool,
}

/// Get the current time as a Unix time, in a manner consistent with our Unix
/// file modification time API. We choose to make this function infallible
/// rather than injecting a bunch of Results.
fn now_as_unix_time() -> i64 {
    // No cleaner way to convert a SystemTime to time_t, as far as I can
    // tell.
    let now = SystemTime::now();
    let dur = match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d,
        Err(_) => return 0, // indicates error to C code, if it cares
    };
    dur.as_secs() as i64
}

impl MemoryIoItem {
    pub fn new(
        files: &Rc<RefCell<MemoryFileCollection>>,
        name: &str,
        truncate: bool,
    ) -> MemoryIoItem {
        let (cur_data, cur_mtime) = match files.borrow_mut().remove(name) {
            Some(info) => {
                if truncate {
                    (Vec::new(), Some(now_as_unix_time()))
                } else {
                    (info.data, info.unix_mtime)
                }
            }
            None => (Vec::new(), Some(now_as_unix_time())),
        };

        MemoryIoItem {
            files: files.clone(),
            name: name.to_owned(),
            state: Cursor::new(cur_data),
            unix_mtime: cur_mtime,
            was_modified: false,
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
        self.was_modified = true;
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

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        Ok(self.unix_mtime)
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.state.seek(pos)?)
    }
}

impl Drop for MemoryIoItem {
    fn drop(&mut self) {
        let unix_mtime = if self.was_modified {
            Some(now_as_unix_time())
        } else {
            self.unix_mtime
        };

        // I think split_off() is an efficient way to move our data vector
        // back into the hashmap? Ideally we could "consume" self but I don't
        // believe that's possible in a Drop implementation.
        let mut mfiles = self.files.borrow_mut();
        mfiles.insert(
            self.name.clone(),
            MemoryFileInfo {
                data: self.state.get_mut().split_off(0),
                unix_mtime,
            },
        );
    }
}

pub struct MemoryIo {
    pub files: Rc<RefCell<MemoryFileCollection>>,
    stdout_allowed: bool,
}

impl MemoryIo {
    pub fn new(stdout_allowed: bool) -> MemoryIo {
        MemoryIo {
            files: Rc::new(RefCell::new(HashMap::new())),
            stdout_allowed,
        }
    }

    pub fn create_entry(&mut self, name: &str, data: Vec<u8>) {
        let mut mfiles = self.files.borrow_mut();
        mfiles.insert(
            name.to_owned(),
            MemoryFileInfo {
                data,
                unix_mtime: Some(now_as_unix_time()),
            },
        );
    }

    pub fn stdout_key(&self) -> &str {
        ""
    }
}

impl IoProvider for MemoryIo {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        if name.is_empty() {
            return OpenResult::NotAvailable;
        }

        let name = normalize_tex_path(name);

        OpenResult::Ok(OutputHandle::new(
            name.to_owned(),
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
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        if name.is_empty() {
            return OpenResult::NotAvailable;
        }

        let name = normalize_tex_path(name);

        if self.files.borrow().contains_key(&*name) {
            OpenResult::Ok(InputHandle::new(
                name.to_owned(),
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
    use crate::status::NoopStatusBackend;
    use std::io::{BufRead, BufReader};

    /// Early versions had a bug where files were not truncated when opened
    /// for writing, which led to junk after the intended EOF when the engine
    /// ran for multiple passes and the file shrank from one pass to the next.
    #[test]
    fn shrinking_file() {
        let mut mem = MemoryIo::new(false);
        let name = "test.tex";
        let mut sb = NoopStatusBackend::default();

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
