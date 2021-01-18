// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Core APIs for bridging the C and Rust portions of Tectonic’s processing
//! backends.
//!
//! We use [cbindgen] to generate C/C++ bindings that can invoke Rust functions
//! providing I/O and other support services.
//!
//! [cbindgen]: https://github.com/eqrion/cbindgen
//!
//! In order to launch your C/C++ engine, you should expect to be given a
//! reference to a `CoreBridgeLauncher` struct. You should use that struct's
//! `with_global_lock` method to obtain a `CoreBridgeState` reference, and then
//! pass that reference across the FFI layer. On the other side of the FFI
//! divide, your code *must* call the functions `ttbc_global_engine_enter` and
//! `ttbc_global_engine_exit` according to the pattern described in
//! `tectonic_bridge_core.h`. If an abort is detected, the callback function
//! must return `Err(EngineAbortedError::new_abort_indicator().into())`.
//!
//! (Unfortunately, this is the cleanest and most reliable API that we can
//! provide because our abort handling uses `setjmp`/`longjmp` and those can't
//! cross FFI boundaries.)

use flate2::{read::GzDecoder, Compression, GzBuilder};
use md5::{Digest, Md5};
use std::{
    ffi::CStr,
    fmt::{Display, Error as FmtError, Formatter},
    io::{self, Read, SeekFrom, Write},
    ptr, slice,
    sync::Mutex,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{
    digest::DigestData, normalize_tex_path, InputFeatures, InputHandle, InputOrigin, IoProvider,
    OpenResult, OutputHandle,
};
use tectonic_status_base::{tt_error, tt_warning, MessageKind, StatusBackend};

/// The IoEventBackend trait allows the program driving the TeX engines to track
/// its input and output access patterns.
///
/// The CLI program uses this information to intelligently decide when to rerun
/// the TeX engine, to choose which files to actually save to disk, and to emit
/// Makefile rules describing the dependency of the outputs on the inputs.
///
/// All of the trait methods have default implementations that do nothing.
pub trait IoEventBackend {
    /// This function is called when a file is opened for output.
    fn output_opened(&mut self, _name: &str) {}

    /// This function is called when the wrapped "standard output"
    /// ("console", "terminal") stream is opened.
    fn stdout_opened(&mut self) {}

    /// This function is called when an output file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// written. Note that this function takes ownership of the name and
    /// digest.
    fn output_closed(&mut self, _name: String, _digest: DigestData) {}

    /// This function is called when a file is opened for input.
    fn input_opened(&mut self, _name: &str, _origin: InputOrigin) {}

    /// This function is called when the "primary input" stream is opened.
    fn primary_input_opened(&mut self, _origin: InputOrigin) {}

    /// This function is called when the engine attempted to open a file of
    /// the specified name but it was not available.
    fn input_not_available(&mut self, _name: &str) {}

    /// This function is called when an input file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// read, if available. This digest is not always available, if the engine
    /// used seeks while reading the file. Note that this function takes
    /// ownership of the name and digest.
    fn input_closed(&mut self, _name: String, _digest: Option<DigestData>) {}
}

/// This struct implements the IoEventBackend trait but does nothing.
#[derive(Clone, Debug, Default)]
pub struct NoopIoEventBackend {}

impl IoEventBackend for NoopIoEventBackend {}

// Function defined in the C support code:
extern "C" {
    fn _ttbc_get_error_message() -> *const libc::c_char;
}

lazy_static::lazy_static! {
    static ref ENGINE_LOCK: Mutex<u8> = Mutex::new(0u8);
}

/// An error type indicating the the FFI code aborted.
///
/// FFI bridge callbacks should return this type, which will then be filled in
/// with error text extracted from the global FFI bridge framework.
#[derive(Debug)]
pub struct EngineAbortedError {
    message: String,
}

impl EngineAbortedError {
    /// Create an error indicating that the FFI engine aborted.
    ///
    /// Upon exit, the global bridge FFI framework will report an
    /// error of this same type, but filled in with error text extracted
    /// from the global FFI bridge framework.
    pub fn new_abort_indicator() -> Self {
        EngineAbortedError {
            message: "[failed to extract detailed error message]".to_owned(),
        }
    }

    unsafe fn new_with_details() -> Self {
        let ptr = _ttbc_get_error_message();
        let message = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        EngineAbortedError { message }
    }
}

impl Display for EngineAbortedError {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EngineAbortedError {}

/// A mechanism for launching bridged FFI code.
pub struct CoreBridgeLauncher<'a> {
    io: &'a mut dyn IoProvider,
    events: &'a mut dyn IoEventBackend,
    status: &'a mut dyn StatusBackend,
}

impl<'a> CoreBridgeLauncher<'a> {
    /// Set up a new context for launching bridged FFI code.
    pub fn new(
        io: &'a mut dyn IoProvider,
        events: &'a mut dyn IoEventBackend,
        status: &'a mut dyn StatusBackend,
    ) -> Self {
        CoreBridgeLauncher { io, events, status }
    }

    /// Invoke a function to launch a bridged FFI engine with a global mutex
    /// held.
    ///
    /// This function *must* be used when invoking C code that makes use of the
    /// global core bridge state functions. Unfortunately, because our error
    /// reporting is based on setjmp/longjmp and it is Undefined Behavior to
    /// setjmp/longjmp across FFI boundaries, we cannot provide a more foolproof
    /// API.
    ///
    /// The invoked code *must* call the functions `ttbc_global_engine_enter`
    /// and `ttbc_global_engine_exit` according to the pattern described in
    /// `tectonic_bridge_core.h`. If an abort is detected, the callback function
    /// should return `Err(EngineAbortedError::new_abort_indicator())`.
    pub fn with_global_lock<F, T>(&mut self, callback: F) -> Result<T>
    where
        F: FnOnce(&mut CoreBridgeState<'_>) -> Result<T>,
    {
        let _guard = ENGINE_LOCK.lock().unwrap();
        let mut state = CoreBridgeState::new(self.io, self.events, self.status);
        let result = callback(&mut state);

        if let Err(ref e) = result {
            if e.downcast_ref::<EngineAbortedError>().is_some() {
                return Err(unsafe { EngineAbortedError::new_with_details() }.into());
            }
        }

        result
    }
}

/// The CoreBridgeState structure is a handle to Rust state that can be used by
/// C/C++ engine code to perform basic I/O functions.
///
/// Code that invokes a Tectonic C/C++ engine should pass a pointer to one of
/// these state structures into the C/C++ layer. It is essential that lifetimes
/// be properly managed across the Rust/C boundary.
pub struct CoreBridgeState<'a> {
    io: &'a mut dyn IoProvider,
    events: &'a mut dyn IoEventBackend,
    status: &'a mut dyn StatusBackend,
    #[allow(clippy::vec_box)]
    input_handles: Vec<Box<InputHandle>>,
    #[allow(clippy::vec_box)]
    output_handles: Vec<Box<OutputHandle>>,
}

impl<'a> CoreBridgeState<'a> {
    fn new(
        io: &'a mut dyn IoProvider,
        events: &'a mut dyn IoEventBackend,
        status: &'a mut dyn StatusBackend,
    ) -> CoreBridgeState<'a> {
        CoreBridgeState {
            io,
            events,
            status,
            output_handles: Vec::new(),
            input_handles: Vec::new(),
        }
    }

    fn input_open_name_format(
        &mut self,
        name: &str,
        format: FileFormat,
    ) -> OpenResult<InputHandle> {
        let r = if let FileFormat::Format = format {
            self.io.input_open_format(name, self.status)
        } else {
            self.io.input_open_name(name, self.status)
        };

        match r {
            OpenResult::NotAvailable => {}
            r => return r,
        }

        // It wasn't available under the immediately-given name. Try adding
        // extensions. Note that we always add a new extension here, even if the
        // filename already has one. E.g., lipsum in TeXLive 2020 asks for
        // `lipsum.ltd.tex` under the name `lipsum.ltd`.

        for e in format.extensions() {
            let ext = format!("{}.{}", name, e);

            if let FileFormat::Format = format {
                if let r @ OpenResult::Ok(_) = self.io.input_open_format(&ext, self.status) {
                    return r;
                }
            } else if let r @ OpenResult::Ok(_) = self.io.input_open_name(&ext, self.status) {
                return r;
            }
        }
        OpenResult::NotAvailable
    }

    fn input_open_name_format_gz(
        &mut self,
        name: &str,
        format: FileFormat,
        is_gz: bool,
    ) -> OpenResult<InputHandle> {
        let base = self.input_open_name_format(name, format);

        if !is_gz {
            return base;
        }

        match base {
            OpenResult::Ok(ih) => {
                let origin = ih.origin();
                let dr = GzDecoder::new(ih.into_inner());

                OpenResult::Ok(InputHandle::new(name, dr, origin))
            }
            _ => base,
        }
    }

    fn get_file_md5(&mut self, name: &str, dest: &mut [u8]) -> bool {
        let name = normalize_tex_path(name);
        let mut hash = Md5::default();

        // We could try to be fancy and look up the file in our cache to see
        // if we've already computed is SHA256 ... and then lie and use a
        // truncated SHA256 digest as the MD5 ... but it seems like a better
        // idea to just go and read the file.

        let mut ih = match self.input_open_name_format(&name, FileFormat::Tex) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => {
                // We could issue a warning here, but the standard LaTeX
                // "rerun check" implementations trigger it very often, which
                // gets annoying. So we'll let this particular failure mode be
                // silent.
                return true;
            }
            OpenResult::Err(e) => {
                tt_error!(self.status, "error trying to open file \"{}\" for MD5 calculation",
                          name; e);
                return true;
            }
        };

        self.events.input_opened(ih.name(), ih.origin());

        // No canned way to stream the whole file into the digest, it seems.

        const BUF_SIZE: usize = 1024;
        let mut buf = [0u8; BUF_SIZE];
        let mut error_occurred = false;

        loop {
            let nread = match ih.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    tt_error!(self.status, "error reading file \"{}\" for MD5 calculation",
                              ih.name(); e.into());
                    error_occurred = true;
                    break;
                }
            };
            hash.update(&buf[..nread]);
        }

        // Clean up.

        let (name, digest_opt) = ih.into_name_digest();
        self.events.input_closed(name, digest_opt);

        if !error_occurred {
            let result = hash.finalize();
            dest.copy_from_slice(result.as_slice());
        }

        error_occurred
    }

    fn output_open(&mut self, name: &str, is_gz: bool) -> *mut OutputHandle {
        let name = normalize_tex_path(name);

        let mut oh = match self.io.output_open_name(&name) {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null_mut(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of output {} failed", name; e);
                return ptr::null_mut();
            }
        };

        if is_gz {
            let name = oh.name().to_owned();
            oh = OutputHandle::new(
                name,
                GzBuilder::new().write(oh.into_inner(), Compression::default()),
            );
        }

        self.events.output_opened(oh.name());
        self.output_handles.push(Box::new(oh));
        &mut **self.output_handles.last_mut().unwrap()
    }

    fn output_open_stdout(&mut self) -> *mut OutputHandle {
        let oh = match self.io.output_open_stdout() {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null_mut(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of stdout failed"; e);
                return ptr::null_mut();
            }
        };

        self.events.stdout_opened();
        self.output_handles.push(Box::new(oh));
        &mut **self.output_handles.last_mut().unwrap()
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
                    tt_warning!(self.status, "error when closing output {}", oh.name(); e.into());
                    rv = true;
                }
                let (name, digest) = oh.into_name_digest();
                self.events.output_closed(name, digest);
                break;
            }
        }

        rv
    }

    fn input_open(&mut self, name: &str, format: FileFormat, is_gz: bool) -> *mut InputHandle {
        let name = normalize_tex_path(name);

        let ih = match self.input_open_name_format_gz(&name, format, is_gz) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => {
                self.events.input_not_available(&name);
                return ptr::null_mut();
            }
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of input {} failed", name; e);
                return ptr::null_mut();
            }
        };

        // the file name may have had an extension added, so we use ih.name() here:
        self.events.input_opened(ih.name(), ih.origin());
        self.input_handles.push(Box::new(ih));
        &mut **self.input_handles.last_mut().unwrap()
    }

    fn input_open_primary(&mut self) -> *mut InputHandle {
        let ih = match self.io.input_open_primary(self.status) {
            OpenResult::Ok(ih) => ih,
            OpenResult::NotAvailable => {
                tt_error!(self.status, "primary input not available (?!)");
                return ptr::null_mut();
            }
            OpenResult::Err(e) => {
                tt_error!(self.status, "open of primary input failed"; e);
                return ptr::null_mut();
            }
        };

        self.events.primary_input_opened(ih.origin());
        self.input_handles.push(Box::new(ih));
        &mut **self.input_handles.last_mut().unwrap()
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

    fn input_get_mtime(&mut self, handle: *mut InputHandle) -> libc::time_t {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        let maybe_time = match rhandle.get_unix_mtime() {
            Ok(t) => t,
            Err(e) => {
                tt_warning!(self.status, "failed to get the modification time of an input"; e);
                Some(0)
            }
        };

        if let Some(t) = maybe_time {
            t as libc::time_t
        } else {
            1 // Intentionally make this distinguishable from the error value 0
        }
    }

    fn input_seek(&mut self, handle: *mut InputHandle, pos: SeekFrom) -> Result<u64> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        Ok(rhandle.try_seek(pos)?)
    }

    fn input_read(&mut self, handle: *mut InputHandle, buf: &mut [u8]) -> Result<()> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.read_exact(buf).map_err(Error::from)
    }

    fn input_getc(&mut self, handle: *mut InputHandle) -> Result<u8> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        Ok(rhandle.getc()?)
    }

    fn input_ungetc(&mut self, handle: *mut InputHandle, byte: u8) -> Result<()> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        Ok(rhandle.ungetc(byte)?)
    }

    fn input_close(&mut self, handle: *mut InputHandle) -> bool {
        let len = self.input_handles.len();

        for i in 0..len {
            let p: *const InputHandle = &*self.input_handles[i];

            if p == handle {
                let mut ih = self.input_handles.swap_remove(i);
                let mut rv = false;

                if let Err(e) = ih.scan_remainder() {
                    tt_warning!(self.status, "error closing out input {}", ih.name(); e);
                    rv = true;
                }

                let (name, digest_opt) = ih.into_name_digest();
                self.events.input_closed(name, digest_opt);
                return rv;
            }
        }

        // TODO: Handle the error better. This indicates a bug in the engine.
        tt_error!(
            self.status,
            "serious internal bug: unexpected handle in input close: {:?}",
            handle
        );

        true
    }
}

// The entry points.

/// Issue a warning.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_issue_warning(es: &mut CoreBridgeState, text: *const libc::c_char) {
    let rtext = CStr::from_ptr(text);
    tt_warning!(es.status, "{}", rtext.to_string_lossy());
}

/// Issue an error.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_issue_error(es: &mut CoreBridgeState, text: *const libc::c_char) {
    let rtext = CStr::from_ptr(text);
    tt_error!(es.status, "{}", rtext.to_string_lossy());
}

/// Calculate the MD5 digest of a Tectonic file.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C.
#[no_mangle]
pub unsafe extern "C" fn ttbc_get_file_md5(
    es: &mut CoreBridgeState,
    path: *const libc::c_char,
    digest: *mut u8,
) -> libc::c_int {
    let rpath = CStr::from_ptr(path).to_string_lossy();
    let rdest = slice::from_raw_parts_mut(digest, 16);

    if es.get_file_md5(rpath.as_ref(), rdest) {
        1
    } else {
        0
    }
}

/// Calculate the MD5 digest of a block of binary data.
///
/// This actually doesn't rely on the state and isn't really I/O, but we also
/// have a get-file-MD5 routine so it's convenient to have this here.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C.
#[no_mangle]
pub unsafe extern "C" fn ttbc_get_data_md5(
    data: *const u8,
    len: libc::size_t,
    digest: *mut u8,
) -> libc::c_int {
    let rdata = slice::from_raw_parts(data, len);
    let rdest = slice::from_raw_parts_mut(digest, 16);

    let mut hash = Md5::default();
    hash.update(rdata);
    let result = hash.finalize();
    rdest.copy_from_slice(result.as_slice());

    0
}

/// Open a Tectonic file for output.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_output_open(
    es: &mut CoreBridgeState,
    name: *const libc::c_char,
    is_gz: libc::c_int,
) -> *mut OutputHandle {
    let rname = CStr::from_ptr(name).to_string_lossy();
    let ris_gz = is_gz != 0;

    es.output_open(&rname, ris_gz)
}

/// Open the general user output stream as a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_open_stdout(es: &mut CoreBridgeState) -> *mut OutputHandle {
    es.output_open_stdout()
}

/// Write a single character to a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_putc(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
    c: libc::c_int,
) -> libc::c_int {
    let rc = c as u8;

    if es.output_write(handle, &[rc]) {
        libc::EOF
    } else {
        c
    }
}

/// Write data to a Tectonic output file.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw C pointers.
#[no_mangle]
pub unsafe extern "C" fn ttbc_output_write(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
    data: *const u8,
    len: libc::size_t,
) -> libc::size_t {
    let rdata = slice::from_raw_parts(data, len);

    // NOTE: we use f.write_all() so partial writes are not gonna be a thing.

    if es.output_write(handle, rdata) {
        0
    } else {
        len
    }
}

/// Flush pending writes to a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_flush(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
) -> libc::c_int {
    if es.output_flush(handle) {
        1
    } else {
        0
    }
}

/// Close a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_close(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
) -> libc::c_int {
    if handle.is_null() {
        return 0; // This is/was the behavior of close_file() in C.
    }

    if es.output_close(handle) {
        1
    } else {
        0
    }
}

/// Open a Tectonic file for input.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_input_open(
    es: &mut CoreBridgeState,
    name: *const libc::c_char,
    format: FileFormat,
    is_gz: libc::c_int,
) -> *mut InputHandle {
    let rname = CStr::from_ptr(name).to_string_lossy();
    let ris_gz = is_gz != 0;
    es.input_open(&rname, format, ris_gz)
}

/// Open the "primary input" file.
#[no_mangle]
pub extern "C" fn ttbc_input_open_primary(es: &mut CoreBridgeState) -> *mut InputHandle {
    es.input_open_primary()
}

/// Get the size of a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_get_size(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::size_t {
    es.input_get_size(handle)
}

/// Get the modification time of a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_get_mtime(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::time_t {
    es.input_get_mtime(handle)
}

/// Seek in a Tectonic input stream.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C.
#[no_mangle]
pub unsafe extern "C" fn ttbc_input_seek(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
    offset: libc::ssize_t,
    whence: libc::c_int,
    internal_error: *mut libc::c_int,
) -> libc::size_t {
    let rwhence = match whence {
        libc::SEEK_SET => SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => SeekFrom::Current(offset as i64),
        libc::SEEK_END => SeekFrom::End(offset as i64),
        _ => {
            tt_error!(
                es.status,
                "serious internal bug: unexpected \"whence\" parameter to fseek() wrapper: {}",
                whence
            );
            *internal_error = 1;
            return 0;
        }
    };

    match es.input_seek(handle, rwhence) {
        Ok(pos) => pos as libc::size_t,
        Err(e) => {
            // TODO: Handle the error better. Report the error properly to the caller?
            tt_error!(es.status, "input seek failed"; e);
            0
        }
    }
}

/// Get a single character from a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_getc(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::c_int {
    // If we couldn't fill the whole (1-byte) buffer, that's boring old EOF.
    // No need to complain. Fun match statement here.

    match es.input_getc(handle) {
        Ok(b) => libc::c_int::from(b),
        Err(e) => {
            if let Some(ioe) = e.downcast_ref::<io::Error>() {
                if ioe.kind() == io::ErrorKind::UnexpectedEof {
                    return libc::EOF;
                }
            }

            tt_warning!(es.status, "getc failed"; e);
            -1
        }
    }
}

/// Put back a character that was obtained from a `getc` call.
#[no_mangle]
pub extern "C" fn ttbc_input_ungetc(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
    ch: libc::c_int,
) -> libc::c_int {
    match es.input_ungetc(handle, ch as u8) {
        Ok(_) => 0,
        Err(e) => {
            tt_warning!(es.status, "ungetc() failed"; e);
            -1
        }
    }
}

/// Read data from a Tectonic input handle
///
/// # Safety
///
/// This function is unsafe because it dereferences raw C pointers.
#[no_mangle]
pub unsafe extern "C" fn ttbc_input_read(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
    data: *mut u8,
    len: libc::size_t,
) -> libc::ssize_t {
    let rdata = slice::from_raw_parts_mut(data, len);

    match es.input_read(handle, rdata) {
        Ok(_) => len as isize,
        Err(e) => {
            tt_warning!(es.status, "{}-byte read failed", len; e);
            -1
        }
    }
}

/// Close a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_close(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::c_int {
    if handle.is_null() {
        return 0; // This is/was the behavior of close_file() in C.
    }

    if es.input_close(handle) {
        1
    } else {
        0
    }
}

/// A buffer for diagnostic messages. Rust code does not need to use this type.
///
/// This type has to be public so that it can be exposed in the C/C++ headers,
/// but it doesn't provide any useful functionality on the Rust side.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    message: String,
    kind: MessageKind,
}

/// Create a new diagnostic that will be reported as a warning.
#[no_mangle]
pub extern "C" fn ttbc_diag_begin_warning() -> *mut Diagnostic {
    let warning = Box::new(Diagnostic {
        message: String::new(),
        kind: MessageKind::Warning,
    });
    Box::into_raw(warning)
}

/// Create a new diagnostic that will be reported as an error.
#[no_mangle]
pub extern "C" fn ttbc_diag_begin_error() -> *mut Diagnostic {
    let warning = Box::new(Diagnostic {
        message: String::new(),
        kind: MessageKind::Error,
    });
    Box::into_raw(warning)
}

/// Append text to a diagnostic.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_diag_append(diag: &mut Diagnostic, text: *const libc::c_char) {
    let rtext = CStr::from_ptr(text);
    diag.message.push_str(&rtext.to_string_lossy());
}

/// "Finish" a diagnostic: report it to the driver and free the diagnostic object.
#[no_mangle]
pub extern "C" fn ttbc_diag_finish(es: &mut CoreBridgeState, diag: *mut Diagnostic) {
    // By creating the box, we will free the diagnostic when this function exits.
    let rdiag = unsafe { Box::from_raw(diag as *mut Diagnostic) };

    es.status
        .report(rdiag.kind, format_args!("{}", rdiag.message), None);
}

/// Different types of files that can be opened by TeX engines
///
/// This enumeration is used to guess filename extensions to try when looking
/// for a file to open.
///
/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FileFormat {
    /// An Adobe Font Metrics file.
    AFM = 4,

    /// A BibTeX bibliography data file.
    Bib = 6,

    /// A BibTeX style file.
    Bst = 7,

    /// A character map data file.
    Cmap = 45,

    /// A configuration file.
    Cnf = 8,

    /// An encoding data file.
    Enc = 44,

    /// A TeX "format" file.
    Format = 10,

    /// A font-map file.
    FontMap = 11,

    /// A miscellaneous font file.
    MiscFonts = 41,

    /// An OFM font metrics file.
    Ofm = 20,

    /// An OpenType font file.
    OpenType = 47,

    /// An OVF file.
    Ovf = 23,

    /// An image file.
    Pict = 25,

    /// A PK font file.
    Pk = 1,

    /// A general program data file.
    ProgramData = 39,

    /// An SFD file.
    Sfd = 46,

    /// The Tectonic primary input file.
    TectonicPrimary = 59,

    /// A TeX language file.
    Tex = 26,

    /// A TeX PostScript header file.
    TexPsHeader = 30,

    /// A TeX Font Metrics file.
    TFM = 3,

    /// A TrueType font file.
    TrueType = 36,

    /// A Type1 font file.
    Type1 = 32,

    /// A Virtual Font file.
    Vf = 33,
}

impl FileFormat {
    fn extensions(&self) -> &[&str] {
        match *self {
            FileFormat::AFM => &["afm"],
            FileFormat::Bib => &["bib"],
            FileFormat::Bst => &["bst"],
            FileFormat::Cmap => &[],
            FileFormat::Cnf => &["cnf"],
            FileFormat::Enc => &["enc"],
            FileFormat::Format => &["fmt"],
            FileFormat::FontMap => &["map"],
            FileFormat::MiscFonts => &[],
            FileFormat::Ofm => &["ofm"],
            FileFormat::OpenType => &["otf", "OTF"],
            FileFormat::Ovf => &["ovf", "vf"],
            FileFormat::Pict => &["pdf", "jpg", "eps", "epsi"],
            FileFormat::Pk => &["pk"],
            FileFormat::ProgramData => &[],
            FileFormat::Sfd => &["sfd"],
            FileFormat::TectonicPrimary => &[],
            FileFormat::Tex => &["tex", "sty", "cls", "fd", "aux", "bbl", "def", "clo", "ldf"],
            FileFormat::TexPsHeader => &["pro"],
            FileFormat::TFM => &["tfm"],
            FileFormat::TrueType => &["ttf", "ttc", "TTF", "TTC", "dfont"],
            FileFormat::Type1 => &["pfa", "pfb"],
            FileFormat::Vf => &["vf"],
        }
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
