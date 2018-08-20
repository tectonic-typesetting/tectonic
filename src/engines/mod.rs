// src/engines/mod.rs -- interface to Tectonic engines written in C
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

//! Access to Tectonic’s processing backends.
//!
//! These backends subsume the functionality of programs such as `bibtex`,
//! `xetex`, and `xdvipdfmx`. The API for each of these is defined in a
//! sub-module with the corresponding name.
//!
//! Due to the way Rust's visibility rules work, this module contains a
//! substantial private API that defines the interface between Tectonic's Rust
//! code and the C/C++ code that the backends are (currently) implemented in.

use flate2::{Compression, GzBuilder};
use flate2::read::{GzDecoder};
use md5::{Md5, Digest};
use libc;
use std::ffi::{CStr, OsStr, OsString};
use std::io::{Read, SeekFrom, Write};
use std::path::Path;
use std::{io, ptr, slice};
use std::borrow::Cow;

use digest::DigestData;
use errors::{Error, ErrorKind, Result};
use io::{InputOrigin, IoProvider, InputFeatures, InputHandle, OpenResult, OutputHandle};
use status::StatusBackend;


// Public sub-modules and reexports.

pub mod bibtex;
pub mod spx2html;
pub mod tex;
pub mod xdvipdfmx;

pub use self::bibtex::BibtexEngine;
pub use self::spx2html::Spx2HtmlEngine;
pub use self::tex::TexEngine;
pub use self::xdvipdfmx::XdvipdfmxEngine;


#[cfg(unix)]
fn osstr_from_cstr(s: &CStr) -> Cow<OsStr> {
	use std::os::unix::ffi::OsStrExt;
	Cow::Borrowed(OsStr::from_bytes(s.to_bytes()))
}
#[cfg(windows)]
fn osstr_from_cstr(s: &CStr) -> Cow<OsStr> {
	Cow::Owned(OsString::from(s.to_string_lossy().to_owned().to_string()))
}

// Now, the public API.

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

    /// This function is called when the "primary input" stream is opened.
    fn primary_input_opened(&mut self, _origin: InputOrigin) {}

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


// Now, the private interfaces for executing various engines implemented in C/C++.

/// During the execution of a C/C++ engine, an ExecutionState structure holds
/// all of the state relevant on the *Rust* side of things: I/O, essentially.
/// The methods on ExecutionState pretty much all work to implement for the
/// "bridge" API (C/C++ => Rust) defined below.

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
        let r = if let FileFormat::Format = format {
            self.io.input_open_format(name, self.status)
        } else {
            self.io.input_open_name(name, self.status)
        };

        let path = Path::new(name);

        match r {
            OpenResult::NotAvailable if path.extension().is_some() =>
                // Do not change the extension if provided.
                return OpenResult::NotAvailable,
            OpenResult::NotAvailable => {},
            r => return r,
        }

        if path.file_name().is_none() {
            // No file name, Path::set_extension will not do anything, so why even try?
            return OpenResult::NotAvailable;
        }

        let mut ext = path.to_owned();

        for e in format_to_extension(format) {
            ext.set_extension(e);

            if let FileFormat::Format = format {
                if let r @ OpenResult::Ok(_) = self.io.input_open_format(ext.as_ref(), self.status) {
                    return r
                }
            } else if let r @ OpenResult::Ok(_) = self.io.input_open_name(ext.as_ref(), self.status) {
                return r
            }
        }
        OpenResult::NotAvailable
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
                let dr = GzDecoder::new(ih.into_inner());

                OpenResult::Ok(InputHandle::new(name, dr, origin))
            },
            _ => base
        }
    }

    // These functions are called from C through the bridge API.

    fn get_file_md5(&mut self, name: &OsStr, dest: &mut [u8]) -> bool {
        let mut hash = Md5::default();

        // We could try to be fancy and look up the file in our cache to see
        // if we've already computed is SHA256 ... and then lie and use a
        // truncated SHA256 digest as the MD5 ... but it seems like a better
        // idea to just go and read the file.

        let mut ih = match self.input_open_name_format(name, FileFormat::Tex) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => {
                // We could issue a warning here, but the standard LaTeX
                // "rerun check" implementations trigger it very often, which
                // gets annoying. So we'll let this particular failure mode be
                // silent.
                return true;
            },
            OpenResult::Err(e) => {
                tt_error!(self.status, "error trying to open file \"{}\" for MD5 calculation",
                          name.to_string_lossy(); e.into());
                return true;
            },
        };

        self.events.input_opened(ih.name(), ih.origin());

        // No canned way to stream the whole file into the digest, it seems.

        const BUF_SIZE: usize = 1024;
        let mut buf = [0u8; BUF_SIZE];
        let mut error_occurred = false;

        loop {
            let nread = match ih.read(&mut buf) {
                Ok(0) => { break; },
                Ok(n) => n,
                Err(e) => {
                    tt_error!(self.status, "error reading file \"{}\" for MD5 calculation",
                              ih.name().to_string_lossy(); e.into());
                    error_occurred = true;
                    break;
                }
            };
            hash.input(&buf[..nread]);
        }

        // Clean up.

        let (name, digest_opt) = ih.into_name_digest();
        self.events.input_closed(name, digest_opt);

        if !error_occurred {
            let result = hash.result();
            dest.copy_from_slice(result.as_slice());
        }

        error_occurred
    }

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
            oh = OutputHandle::new(&name, GzBuilder::new().write(oh.into_inner(), Compression::default()));
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

    fn input_open_primary(&mut self) -> *const InputHandle {
        let ih = match self.io.input_open_primary(self.status) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => {
                tt_error!(self.status, "primary input not available (?!)");
                return ptr::null();
            },
            OpenResult::Err(e) => {
                tt_error!(self.status, "open of primary input failed"; e);
                return ptr::null();
            },
        };

        self.events.primary_input_opened(ih.origin());
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


// Now, here' the actual C API. There are two parts to this: the functions in
// the backing C/C++ code that *we* call, and the API bridge -- a struct of
// function pointers that we pass to the C/C++ entry points so that they can
// call back into our code. Keep synchronized with **tectonic/core-bridge.h**.

#[repr(C)]
struct TectonicBridgeApi {
    context: *const libc::c_void,
    issue_warning: *const libc::c_void,
    issue_error: *const libc::c_void,
    get_file_md5: *const libc::c_void,
    get_data_md5: *const libc::c_void,
    output_open: *const libc::c_void,
    output_open_stdout: *const libc::c_void,
    output_putc: *const libc::c_void,
    output_write: *const libc::c_void,
    output_flush: *const libc::c_void,
    output_close: *const libc::c_void,
    input_open: *const libc::c_void,
    input_open_primary: *const libc::c_void,
    input_get_size: *const libc::c_void,
    input_seek: *const libc::c_void,
    input_read: *const libc::c_void,
    input_getc: *const libc::c_void,
    input_ungetc: *const libc::c_void,
    input_close: *const libc::c_void,
}

extern {
    fn tt_get_error_message() -> *const libc::c_char;
    fn tt_set_int_variable(var_name: *const libc::c_char, value: libc::c_int) -> libc::c_int;
    //fn tt_set_string_variable(var_name: *const libc::c_char, value: *const libc::c_char) -> libc::c_int;
    fn tex_simple_main(api: *const TectonicBridgeApi, dump_name: *const libc::c_char, input_file_name: *const libc::c_char) -> libc::c_int;
    fn dvipdfmx_simple_main(api: *const TectonicBridgeApi,
                            dviname: *const libc::c_char,
                            pdfname: *const libc::c_char,
                            enable_compression: bool,
                            deterministic_tags: bool) -> libc::c_int;
    fn bibtex_simple_main(api: *const TectonicBridgeApi, aux_file_name: *const libc::c_char) -> libc::c_int;
}


// Entry points for the C/C++ API functions.

fn issue_warning<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, text: *const libc::c_char) {
    let es = unsafe { &mut *es };
    let rtext = unsafe { CStr::from_ptr(text) };

    tt_warning!(es.status, "{}", rtext.to_string_lossy());
}

fn issue_error<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, text: *const libc::c_char) {
    let es = unsafe { &mut *es };
    let rtext = unsafe { CStr::from_ptr(text) };

    tt_error!(es.status, "{}", rtext.to_string_lossy());
}

fn get_file_md5<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, path: *const libc::c_char, digest: *mut u8) -> libc::c_int {
    let es = unsafe { &mut *es };
    let rpath = osstr_from_cstr(unsafe { CStr::from_ptr(path) });
    let rdest = unsafe { slice::from_raw_parts_mut(digest, 16) };

    if es.get_file_md5(rpath.as_ref(), rdest) {
        1
    } else {
        0
    }
}

fn get_data_md5<'a, I: 'a + IoProvider>(_es: *mut ExecutionState<'a, I>, data: *const u8, len: libc::size_t, digest: *mut u8) -> libc::c_int {
    //let es = unsafe { &mut *es };
    let rdata = unsafe { slice::from_raw_parts(data, len) };
    let rdest = unsafe { slice::from_raw_parts_mut(digest, 16) };

    let mut hash = Md5::default();
    hash.input(rdata);
    let result = hash.result();
    rdest.copy_from_slice(result.as_slice());

    0
}

fn output_open<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, name: *const libc::c_char, is_gz: libc::c_int) -> *const libc::c_void {
    let es = unsafe { &mut *es };
    let rname = osstr_from_cstr(&unsafe { CStr::from_ptr(name) });
    let ris_gz = is_gz != 0;

    es.output_open(&rname, ris_gz) as *const _
}

fn output_open_stdout<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, ) -> *const libc::c_void {
    let es = unsafe { &mut *es };

    es.output_open_stdout() as *const _
}

fn output_putc<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void, c: libc::c_int) -> libc::c_int {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut OutputHandle;
    let rc = c as u8;

    if es.output_write(rhandle, &[rc]) {
        libc::EOF
    } else {
        c
    }
}

fn output_write<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void, data: *const u8, len: libc::size_t) -> libc::size_t {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut OutputHandle;
    let rdata = unsafe { slice::from_raw_parts(data, len) };

    // NOTE: we use f.write_all() so partial writes are not gonna be a thing.

    if es.output_write(rhandle, rdata) {
        0
    } else {
        len
    }
}

fn output_flush<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void) -> libc::c_int {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut OutputHandle;

    if es.output_flush(rhandle) {
        1
    } else {
        0
    }
}

fn output_close<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void) -> libc::c_int {
    let es = unsafe { &mut *es };

    if handle == 0 as *mut _ {
        return 0; // This is/was the behavior of close_file() in C.
    }

    let rhandle = handle as *mut OutputHandle;

    if es.output_close(rhandle) {
        1
    } else {
        0
    }
}


fn input_open<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, name: *const libc::c_char, format: libc::c_int, is_gz: libc::c_int) -> *const libc::c_void {
    let es = unsafe { &mut *es };
    let rname = osstr_from_cstr(unsafe { CStr::from_ptr(name) });
    let rformat = c_format_to_rust(format);
    let ris_gz = is_gz != 0;

    match rformat {
        Some(fmt) => {
            es.input_open(&rname, fmt, ris_gz) as *const _
        },
        None => ptr::null()
    }
}

fn input_open_primary<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>) -> *const libc::c_void {
    let es = unsafe { &mut *es };

    es.input_open_primary() as *const _
}

fn input_get_size<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void) -> libc::size_t {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut InputHandle;

    es.input_get_size(rhandle)
}

fn input_seek<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void, offset: libc::ssize_t, whence: libc::c_int) -> libc::size_t {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut InputHandle;

    let rwhence = match whence {
        libc::SEEK_SET => SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => SeekFrom::Current(offset as i64),
        libc::SEEK_END => SeekFrom::End(offset as i64),
        _ => panic!("Unexpected \"whence\" parameter to fseek() wrapper: {}", whence),
    };

    es.input_seek(rhandle, rwhence) as libc::size_t
}

fn input_getc<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void) -> libc::c_int {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut InputHandle;

    // If we couldn't fill the whole (1-byte) buffer, that's boring old EOF.
    // No need to complain. Fun match statement here.

    match es.input_getc(rhandle) {
        Ok(b) => b as libc::c_int,
        Err(Error(ErrorKind::Io(ref ioe), _)) if ioe.kind() == io::ErrorKind::UnexpectedEof => libc::EOF,
        Err(e) => {
            tt_warning!(es.status, "getc failed"; e);
            -1
        }
    }
}

fn input_ungetc<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void, ch: libc::c_int) -> libc::c_int {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut InputHandle;

    match es.input_ungetc(rhandle, ch as u8) {
        Ok(_) => 0,
        Err(e) => {
            tt_warning!(es.status, "ungetc() failed"; e);
            -1
        }
    }
}

fn input_read<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void, data: *mut u8, len: libc::size_t) -> libc::ssize_t {
    let es = unsafe { &mut *es };
    let rhandle = handle as *mut InputHandle;
    let rdata = unsafe { slice::from_raw_parts_mut(data, len) };

    match es.input_read(rhandle, rdata) {
        Ok(_) => len as isize,
        Err(e) => {
            tt_warning!(es.status, "{}-byte read failed", len; e);
            -1
        }
    }
}

fn input_close<'a, I: 'a + IoProvider>(es: *mut ExecutionState<'a, I>, handle: *mut libc::c_void) -> libc::c_int {
    let es = unsafe { &mut *es };

    if handle == 0 as *mut _ {
        return 0; // This is/was the behavior of close_file() in C.
    }

    let rhandle = handle as *mut InputHandle;

    if es.input_close(rhandle) {
        1
    } else {
        0
    }
}


// All of these entry points are used to populate the bridge API struct:

impl TectonicBridgeApi {
    fn new<'a, I: 'a + IoProvider>(exec_state: &ExecutionState<'a, I>) -> TectonicBridgeApi {
        TectonicBridgeApi {
            context: (exec_state as *const ExecutionState<'a, I>) as *const libc::c_void,
            issue_warning: issue_warning::<'a, I> as *const libc::c_void,
            issue_error: issue_error::<'a, I> as *const libc::c_void,
            get_file_md5: get_file_md5::<'a, I> as *const libc::c_void,
            get_data_md5: get_data_md5::<'a, I> as *const libc::c_void,
            output_open: output_open::<'a, I> as *const libc::c_void,
            output_open_stdout: output_open_stdout::<'a, I> as *const libc::c_void,
            output_putc: output_putc::<'a, I> as *const libc::c_void,
            output_write: output_write::<'a, I> as *const libc::c_void,
            output_flush: output_flush::<'a, I> as *const libc::c_void,
            output_close: output_close::<'a, I> as *const libc::c_void,
            input_open: input_open::<'a, I> as *const libc::c_void,
            input_open_primary: input_open_primary::<'a, I> as *const libc::c_void,
            input_get_size: input_get_size::<'a, I> as *const libc::c_void,
            input_seek: input_seek::<'a, I> as *const libc::c_void,
            input_read: input_read::<'a, I> as *const libc::c_void,
            input_getc: input_getc::<'a, I> as *const libc::c_void,
            input_ungetc: input_ungetc::<'a, I> as *const libc::c_void,
            input_close: input_close::<'a, I> as *const libc::c_void,
        }
    }
}


// Finally, some support -- several of the C API functions pass arguments that
// are "file format" enumerations. This code bridges the two. See the
// `tt_input_format_type` enum in <tectonic/core-bridge.h>.

#[derive(Clone,Copy,Debug)]
enum FileFormat {
    AFM,
    Bib,
    Bst,
    Cmap,
    Enc,
    Format,
    FontMap,
    MiscFonts,
    Ofm,
    OpenType,
    Ovf,
    Pict,
    Pk,
    ProgramData,
    Sfd,
    Tex,
    TexPsHeader,
    TFM,
    TrueType,
    Type1,
    Vf,
}

fn format_to_extension (format: FileFormat) -> Vec<&'static str> {
    match format {
        FileFormat::AFM => vec!["afm"],
        FileFormat::Bib => vec!["bib"],
        FileFormat::Bst => vec!["bst"],
        FileFormat::Cmap => vec!["cmap"], /* XXX: kpathsea doesn't define any suffixes for this */
        FileFormat::Enc => vec!["enc"],
        FileFormat::Format => vec!["fmt"],
        FileFormat::FontMap => vec!["map"],
        FileFormat::MiscFonts => vec!["miscfonts"], /* XXX: no kpathsea suffixes */
        FileFormat::Ofm => vec!["ofm"],
        FileFormat::OpenType => vec!["otf", "OTF"],
        FileFormat::Ovf => vec!["ovf", "vf"],
        FileFormat::Pict => vec!["pdf", "jpg", "eps", "epsi"], /* XXX: also .eps, .epsi, ... */
        FileFormat::Pk => vec!["pk"],
        FileFormat::ProgramData => vec!["programdata"], /* XXX no suffixes */
        FileFormat::Sfd => vec!["sfd"],
        FileFormat::Tex => vec!["tex", "sty", "cls", "fd", "aux", "bbl", "def", "clo", "ldf"],
        FileFormat::TexPsHeader => vec!["pro"],
        FileFormat::TFM => vec!["tfm"],
        FileFormat::TrueType => vec!["ttf", "ttc", "TTF", "TTC", "dfont"],
        FileFormat::Type1 => vec!["pfa", "pfb"],
        FileFormat::Vf => vec!["vf"],
    }
}

fn c_format_to_rust (format: libc::c_int) -> Option<FileFormat> {
    match format {
        1 => Some(FileFormat::Pk),
        3 => Some(FileFormat::TFM),
        4 => Some(FileFormat::AFM),
        6 => Some(FileFormat::Bib),
        7 => Some(FileFormat::Bst),
        10 => Some(FileFormat::Format),
        11 => Some(FileFormat::FontMap),
        20 => Some(FileFormat::Ofm),
        23 => Some(FileFormat::Ovf),
        25 => Some(FileFormat::Pict),
        26 => Some(FileFormat::Tex),
        30 => Some(FileFormat::TexPsHeader),
        32 => Some(FileFormat::Type1),
        33 => Some(FileFormat::Vf),
        36 => Some(FileFormat::TrueType),
        39 => Some(FileFormat::ProgramData),
        40 => Some(FileFormat::ProgramData), // NOTE: kpathsea distinguishes text/binary; we don't
        41 => Some(FileFormat::MiscFonts),
        44 => Some(FileFormat::Enc),
        45 => Some(FileFormat::Cmap),
        46 => Some(FileFormat::Sfd),
        47 => Some(FileFormat::OpenType),
        _ => None
    }
}
