// src/engines/mod.rs -- interface to Tectonic engines written in C
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::{Compression, GzBuilder};
use flate2::read::{GzDecoder};
use std::ffi::{OsStr, OsString};
use std::io::{stderr, SeekFrom, Write};
use std::path::PathBuf;
use std::ptr;

use errors::Result;
use io::{IOProvider, IOStack, InputHandle, OpenResult, OutputHandle};
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

pub use self::tex::TexEngine;


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

struct ExecutionState<'a, I: 'a + IOProvider>  {
    io: &'a mut I,
    input_handles: Vec<Box<InputHandle>>,
    output_handles: Vec<Box<OutputHandle>>,
}


impl<'a, I: 'a + IOProvider> ExecutionState<'a, I> {
    pub fn new (io: &'a mut I) -> ExecutionState<'a, I> {
        ExecutionState {
            io: io,
            output_handles: Vec::new(),
            input_handles: Vec::new(),
        }
    }

    // The key function of the ExecutionState struct is to provide I/O
    // functions for the C-based engines.

    fn input_open_name_format(&mut self, name: &OsStr, format: FileFormat) -> OpenResult<InputHandle> {
        // TODO: for some formats we should check multiple extensions, not
        // just one.

        let r = self.io.input_open_name(name);
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

        return self.io.input_open_name(&ext.into_os_string());
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
                    Ok(dr) => OpenResult::Ok(Box::new(dr)),
                    Err(e) => OpenResult::Err(e.into()),
                }
            },
            _ => base
        }
    }

    fn output_open(&mut self, name: &OsStr, is_gz: bool) -> *const OutputHandle {
        let mut oh = match self.io.output_open_name(name) {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null(),
            OpenResult::Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: open of {} failed: {}",
                         name.to_string_lossy(), e).expect("stderr failed");
                return ptr::null()
            }
        };

        if is_gz {
            oh = Box::new(GzBuilder::new().write(oh, Compression::Default));
        }

        self.output_handles.push(Box::new(oh));
        &*self.output_handles[self.output_handles.len()-1]
    }

    fn output_open_stdout(&mut self) -> *const OutputHandle {
        let oh = match self.io.output_open_stdout() {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null(),
            OpenResult::Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: open of stdout failed: {}",
                         e).expect("stderr failed");
                return ptr::null()
            }
        };

        self.output_handles.push(Box::new(oh));
        &*self.output_handles[self.output_handles.len()-1]
    }

    fn output_write(&mut self, handle: *mut OutputHandle, buf: &[u8]) -> bool {
        let rhandle: &mut OutputHandle = unsafe { &mut *handle };
        let result = rhandle.write_all(buf);

        match result {
            Ok(_) => false,
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: write failed: {}", e).expect("stderr failed");
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
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: flush failed: {}", e).expect("stderr failed");
                true
            }
        }
    }

    fn output_close(&mut self, handle: *mut OutputHandle) -> bool {
        let len = self.output_handles.len();

        for i in 0..len {
            let p: *const OutputHandle = &*self.output_handles[i];

            if p == handle {
                self.output_handles.swap_remove(i);
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
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: open of input {} failed: {}",
                         name.to_string_lossy(), e).expect("stderr failed");
                return ptr::null()
            }
        };

        self.input_handles.push(Box::new(ih));
        &*self.input_handles[self.input_handles.len()-1]
    }

    fn input_get_size(&mut self, handle: *mut InputHandle) -> usize {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        match rhandle.get_size() {
            Ok(s) => s,
            Err(e) => {
                writeln!(&mut stderr(), "WARNING: get-size failed: {}", e).expect("stderr failed");
                0
            }
        }
    }

    fn input_seek(&mut self, handle: *mut InputHandle, pos: SeekFrom) -> u64 {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        match rhandle.try_seek(pos) {
            Ok(pos) => pos,
            Err(e) => {
                writeln!(&mut stderr(), "WARNING: input seek failed: {}", e).expect("stderr failed");
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
fn with_global_state<F, T> (f: F) -> T where F: FnOnce(&mut ExecutionState<IOStack>) -> T {
    unsafe { f(&mut *(GLOBAL_ENGINE_PTR as *mut ExecutionState<IOStack>)) }
}

// This wraps any activities that cause the C code to spin up.
unsafe fn assign_global_state<F, T> (state: &mut ExecutionState<IOStack>, f: F) -> T where F: FnOnce() -> T {
    GLOBAL_ENGINE_PTR = state as *mut ExecutionState<IOStack> as *mut ();
    let rv = f();
    GLOBAL_ENGINE_PTR = 0 as *mut _;
    rv
}
