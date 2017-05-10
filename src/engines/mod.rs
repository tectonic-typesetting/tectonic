// src/engines/mod.rs -- interface to Tectonic engines written in C
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::{Compression, GzBuilder};
use flate2::read::{GzDecoder};
use std::ffi::{OsStr, OsString};
use std::io::{Read, SeekFrom, Write};
use std::path::PathBuf;
use std::ptr;

use digest::DigestData;
use errors::Result;
use io::{InputOrigin, IoProvider, IoStack, InputFeatures, InputHandle, OpenResult, OutputHandle};
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


/// The IoEventBackend trait allows the program driving the TeX engines to
/// track its input and output access patterns. The CLI program uses this
/// information to intelligently decide when to rerun the TeX engine, to
/// choose which files to actually save to disk, and to emit Makefile rules
/// describing the dependency of the outputs on the inputs.
///
/// All of the trait methods have default implementations that do nothing.

pub trait IoEventBackend {
    /// This function is called when a file is opened for output.
    fn output_opened(&mut self, _name: &OsStr) {}

    /// This function is called when the wrapped "standard output"
    /// ("console", "terminal") stream is opened.
    fn stdout_opened(&mut self) {}

    /// This function is called when an output file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// written. Note that this function takes ownership of the name and
    /// digest.
    fn output_closed(&mut self, _name: OsString, _digest: DigestData) {}

    /// This function is called when a file is opened for input.
    fn input_opened(&mut self, _name: &OsStr, _origin: InputOrigin) {}

    /// This function is called when the engine attempted to open a file of
    /// the specified name but it was not available.
    fn input_not_available(&mut self, _name: &OsStr) {}

    /// This function is called when an input file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// read, if available. This digest is not always available, if the engine
    /// used seeks while reading the file. Note that this function takes
    /// ownership of the name and digest.
    fn input_closed(&mut self, _name: OsString, _digest: Option<DigestData>) {}
}


/// This struct implements the IoEventBackend trait but does nothing.
pub struct NoopIoEventBackend {}

impl NoopIoEventBackend {
    pub fn new() -> NoopIoEventBackend { NoopIoEventBackend {} }
}

impl IoEventBackend for NoopIoEventBackend { }


// The private interface for executing various engines implemented in C.
//
// The C code relies on an enormous number of global variables so, despite our
// fancy API, we can only ever safely run one backend at a time in any given
// process. (For now.) Here we set up the infrastructure to manage this. Of
// course, this is totally un-thread-safe, etc., because the underlying C code
// is.

struct ExecutionState<'a, I: 'a + IoProvider>  {
    io: &'a mut I,
    events: &'a mut IoEventBackend,
    status: &'a mut StatusBackend,
    input_handles: Vec<Box<InputHandle>>,
    output_handles: Vec<Box<OutputHandle>>,
}


impl<'a, I: 'a + IoProvider> ExecutionState<'a, I> {
    pub fn new (io: &'a mut I, events: &'a mut IoEventBackend,
                status: &'a mut StatusBackend) -> ExecutionState<'a, I> {
        ExecutionState {
            io: io,
            events: events,
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
                let origin = ih.origin();

                match GzDecoder::new(ih.into_inner()) {
                    Ok(dr) => OpenResult::Ok(InputHandle::new(name, dr, origin)),
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

        if is_gz {
            let name = oh.name().to_os_string();
            oh = OutputHandle::new(&name, GzBuilder::new().write(oh.into_inner(), Compression::Default));
        }

        self.events.output_opened(oh.name());
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

        self.events.stdout_opened();
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
        let mut rv = false;

        for i in 0..len {
            let p: *const OutputHandle = &*self.output_handles[i];

            if p == handle {
                let mut oh = self.output_handles.swap_remove(i);
                if let Err(e) = oh.flush() {
                    tt_warning!(self.status, "error when closing output {}", oh.name().to_string_lossy(); e.into());
                    rv = true;
                }
                let (name, digest) = oh.into_name_digest();
                self.events.output_closed(name, digest);
                break;
            }
        }

        rv
    }

    fn input_open(&mut self, name: &OsStr, format: FileFormat, is_gz: bool) -> *const InputHandle {
        let ih = match self.input_open_name_format_gz(name, format, is_gz) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => {
                self.events.input_not_available(name);
                return ptr::null();
            },
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of input {} failed", name.to_string_lossy(); e);
                return ptr::null();
            },
        };

        // the file name may have had an extension added, so we use ih.name() here:
        self.events.input_opened(ih.name(), ih.origin());
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

    fn input_getc(&mut self, handle: *mut InputHandle) -> Result<u8> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.getc()
    }

    fn input_ungetc(&mut self, handle: *mut InputHandle, byte: u8) -> Result<()> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.ungetc(byte)
    }

    fn input_close(&mut self, handle: *mut InputHandle) -> bool {
        let len = self.input_handles.len();

        for i in 0..len {
            let p: *const InputHandle = &*self.input_handles[i];

            if p == handle {
                let ih = self.input_handles.swap_remove(i);
                let (name, digest_opt) = ih.into_name_digest();
                self.events.input_closed(name, digest_opt);
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
