// src/engines/mod.rs -- interface to Tectonic engines written in C
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::{Compression, GzBuilder};
use flate2::read::{GzDecoder};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{Read, SeekFrom, Write};
use std::path::PathBuf;
use std::ptr;

use digest::DigestData;
use errors::Result;
use io::{IoProvider, IoStack, InputFeatures, InputHandle, OpenResult, OutputHandle};
use status::StatusBackend;
use self::file_format::{format_to_extension, FileFormat};


// Internal sub-modules. Some of these must be public so that their symbols are
// exported for the C library to see them. That could be changed if we gave
// the C code a struct with pointers to the functions.

mod c_api;
mod file_format;
pub mod io_api;
pub mod kpse_api;
pub mod md5_api;


// Public sub-modules and reexports.

pub mod tex;
pub mod xdvipdfmx;
pub mod bibtex;

pub use self::tex::TexEngine;
pub use self::xdvipdfmx::XdvipdfmxEngine;
pub use self::bibtex::BibtexEngine;


/// Different patterns with which files may have been accessed by the
/// underlying engines. Once a file is marked as ReadThenWritten or
/// WrittenThenRead, its pattern does not evolve further.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum AccessPattern {
    /// This file is only ever read.
    Read,

    /// This file is only ever written. This suggests that it is
    /// a final output of the processing session.
    Written,

    /// This file is read, then written. We call this a "circular" access
    /// pattern. Multiple passes of an engine will result in outputs that
    /// change if this file's contents change, or if the file did not exist at
    /// the time of the first pass.
    ReadThenWritten,

    /// This file is written, then read. We call this a "temporary" access
    /// pattern. This file is likely a temporary buffer that is not of
    /// interest to the user.
    WrittenThenRead,
}

/// A summary of the I/O that happened on a file. We record its access
/// pattern, the cryptographic digest of the file when it was last read, and
/// the cryptographic digest of the file as it was last written.
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct FileSummary {
    pub access_pattern: AccessPattern,
    pub read_digest: Option<DigestData>,
    pub write_digest: Option<DigestData>,
}

impl FileSummary {
    pub fn new(access_pattern: AccessPattern) -> FileSummary {
        FileSummary {
            access_pattern: access_pattern,
            read_digest: None,
            write_digest: None,
        }
    }
}


// The private interface for executing various engines implemented in C.
//
// The C code relies on an enormous number of global variables so, despite our
// fancy API, we can only ever safely run one backend at a time in any given
// process. (For now.) Here we set up the infrastructure to manage this. Of
// course, this is totally un-thread-safe, etc., because the underlying C code
// is.

// The double-boxing of the handles in ExecutionState isn't nice. I *think*
// that in principle we could turn the inner boxes into pointers and pass
// those around. But I can't get it to work in practice -- it may be Boxes of
// trait objects become fat pointers themselves. It's really not a big deal so
// let's just roll with it for now.

struct ExecutionState<'a, I: 'a + IoProvider>  {
    io: &'a mut I,
    summaries: Option<&'a mut HashMap<OsString, FileSummary>>,
    status: &'a mut StatusBackend,
    input_handles: Vec<Box<InputHandle>>,
    output_handles: Vec<Box<OutputHandle>>,
}


impl<'a, I: 'a + IoProvider> ExecutionState<'a, I> {
    pub fn new (io: &'a mut I, summaries: Option<&'a mut HashMap<OsString, FileSummary>>,
                status: &'a mut StatusBackend) -> ExecutionState<'a, I> {
        ExecutionState {
            io: io,
            summaries: summaries,
            status: status,
            output_handles: Vec::new(),
            input_handles: Vec::new(),
        }
    }

    // Helpers.

    fn input_open_name_format(&mut self, name: &OsStr, format: FileFormat) -> OpenResult<InputHandle> {
        // TODO: for some formats we should check multiple extensions, not
        // just one.

        let r = self.io.input_open_name(name, self.status);
        if let OpenResult::NotAvailable = r {
        } else {
            return r;
        }

        // Maybe there's a nicer way to alter the extension without turning
        // `name` into a Path?

        let mut ext = PathBuf::from(name);
        let mut ename = OsString::from(match ext.file_name() {
            Some(s) => s,
            None => return OpenResult::NotAvailable
        });
        ename.push(format_to_extension(format));
        ext.set_file_name(ename);

        return self.io.input_open_name(&ext.into_os_string(), self.status);
    }

    fn input_open_name_format_gz(&mut self, name: &OsStr, format: FileFormat,
                                 is_gz: bool) -> OpenResult<InputHandle> {
        let base = self.input_open_name_format(name, format);

        if !is_gz {
            return base;
        }

        match base {
            OpenResult::Ok(ih) => {
                match GzDecoder::new(ih) {
                    Ok(dr) => OpenResult::Ok(InputHandle::new(name, dr)),
                    Err(e) => OpenResult::Err(e.into()),
                }
            },
            _ => base
        }
    }

    // These functions are called from C via `io_api`:

    fn output_open(&mut self, name: &OsStr, is_gz: bool) -> *const OutputHandle {
        let mut oh = match self.io.output_open_name(name) {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of output {} failed", name.to_string_lossy(); e);
                return ptr::null()
            }
        };

        let name = oh.name().to_os_string(); // mainly for symmetry with input_open()

        if is_gz {
            oh = OutputHandle::new(&name, GzBuilder::new().write(oh, Compression::Default));
        }

        if let Some(ref mut summaries) = self.summaries {
            // Borrow-checker fight.
            if {
                if let Some(summ) = summaries.get_mut(&name) {
                    summ.access_pattern = match summ.access_pattern {
                        AccessPattern::Read => AccessPattern::ReadThenWritten,
                        c => c, // identity mapping makes sense for remaining options
                    };
                    false // no, do not insert a new item
                } else {
                    true // yes, insert a new item
                }
            } {
                // The 'else' branch above returned 'true'.
                summaries.insert(name, FileSummary::new(AccessPattern::Written));
            }
        }

        self.output_handles.push(Box::new(oh));
        &*self.output_handles[self.output_handles.len()-1]
    }

    fn output_open_stdout(&mut self) -> *const OutputHandle {
        let oh = match self.io.output_open_stdout() {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of stdout failed"; e);
                return ptr::null()
            }
        };

        // Life is easier if we track stdout in the same way that we do other
        // output files.

        if let Some(ref mut summaries) = self.summaries {
            // Borrow-checker fight.
            if {
                if let Some(summ) = summaries.get_mut(OsStr::new("")) {
                    summ.access_pattern = match summ.access_pattern {
                        AccessPattern::Read => AccessPattern::ReadThenWritten,
                        c => c, // identity mapping makes sense for remaining options
                    };
                    false // no, do not insert a new item
                } else {
                    true // yes, insert a new item
                }
            } {
                // The 'else' branch above returned 'true'.
                summaries.insert(OsString::from(""), FileSummary::new(AccessPattern::Written));
            }
        }

        self.output_handles.push(Box::new(oh));
        &*self.output_handles[self.output_handles.len()-1]
    }

    fn output_write(&mut self, handle: *mut OutputHandle, buf: &[u8]) -> bool {
        let rhandle: &mut OutputHandle = unsafe { &mut *handle };
        let result = rhandle.write_all(buf);

        match result {
            Ok(_) => false,
            Err(e) => {
                tt_warning!(self.status, "write failed"; e.into());
                true
            }
        }
    }

    fn output_flush(&mut self, handle: *mut OutputHandle) -> bool {
        let rhandle: &mut OutputHandle = unsafe { &mut *handle };
        let result = rhandle.flush();

        match result {
            Ok(_) => false,
            Err(e) => {
                tt_warning!(self.status, "flush failed"; e.into());
                true
            }
        }
    }

    fn output_close(&mut self, handle: *mut OutputHandle) -> bool {
        let len = self.output_handles.len();

        for i in 0..len {
            let p: *const OutputHandle = &*self.output_handles[i];

            if p == handle {
                let oh = self.output_handles.swap_remove(i);
                let (name, digest) = oh.into_name_digest();

                if let Some(ref mut summaries) = self.summaries {
                    let mut summ = summaries.get_mut(&name).expect("closing file that wasn't opened?");
                    summ.write_digest = Some(digest);
                }

                break;
            }
        }

        false
    }

    fn input_open(&mut self, name: &OsStr, format: FileFormat, is_gz: bool) -> *const InputHandle {
        let ih = match self.input_open_name_format_gz(name, format, is_gz) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => return ptr::null(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of input {} failed", name.to_string_lossy(); e);
                return ptr::null()
            }
        };

        let name = ih.name().to_os_string(); // final name may have had an extension added, etc.

        if let Some(ref mut summaries) = self.summaries {
            // Borrow-checker fight.
            if {
                if let Some(summ) = summaries.get_mut(&name) {
                    summ.access_pattern = match summ.access_pattern {
                        AccessPattern::Written => AccessPattern::WrittenThenRead,
                        c => c, // identity mapping makes sense for remaining options
                    };
                    false // no, do not insert a new item
                } else {
                    true // yes, insert a new item
                }
            } {
                // The 'else' branch above returned 'true'.
                summaries.insert(name, FileSummary::new(AccessPattern::Read));
            }
        }

        self.input_handles.push(Box::new(ih));
        &*self.input_handles[self.input_handles.len()-1]
    }

    fn input_get_size(&mut self, handle: *mut InputHandle) -> usize {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        match rhandle.get_size() {
            Ok(s) => s,
            Err(e) => {
                tt_warning!(self.status, "failed to get the size of an input"; e);
                0
            }
        }
    }

    fn input_seek(&mut self, handle: *mut InputHandle, pos: SeekFrom) -> u64 {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        match rhandle.try_seek(pos) {
            Ok(pos) => pos,
            Err(e) => {
                tt_warning!(self.status, "input seek failed"; e);
                0
            }
        }
    }

    fn input_read(&mut self, handle: *mut InputHandle, buf: &mut [u8]) -> Result<()> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        Ok(rhandle.read_exact(buf)?)
    }

    fn input_close(&mut self, handle: *mut InputHandle) -> bool {
        let len = self.input_handles.len();

        for i in 0..len {
            let p: *const InputHandle = &*self.input_handles[i];

            if p == handle {
                self.input_handles.swap_remove(i);
                return false;
            }
        }

        panic!("unexpected handle {:?}", handle);
    }
}


// Here's the hacky framework for letting the C code get back an ExecutionState.

// note: ptr::null_mut() gives me a compile error related to const fns right now.
static mut GLOBAL_ENGINE_PTR: *mut () = 0 as *mut _;

// This wraps a Rust function called by the C code via some ttstub_*() function.
fn with_global_state<F, T> (f: F) -> T where F: FnOnce(&mut ExecutionState<IoStack>) -> T {
    unsafe { f(&mut *(GLOBAL_ENGINE_PTR as *mut ExecutionState<IoStack>)) }
}

// This wraps any activities that cause the C code to spin up.
unsafe fn assign_global_state<F, T> (state: &mut ExecutionState<IoStack>, f: F) -> T where F: FnOnce() -> T {
    GLOBAL_ENGINE_PTR = state as *mut ExecutionState<IoStack> as *mut ();
    let rv = f();
    GLOBAL_ENGINE_PTR = 0 as *mut _;
    rv
}
