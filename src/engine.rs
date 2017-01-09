// src/engine.rs -- interface for the Tectonic engine
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use flate2::{Compression, GzBuilder};
use flate2::read::{GzDecoder};
use libc;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::io::{stderr, Read, SeekFrom, Write};
use std::path::PathBuf;
use std::ptr;

use ::{assign_global_engine, EngineInternals};
use c_api;
use errors::{ErrorKind, Result};
use file_format::{format_to_extension, FileFormat};
use io::{InputHandle, IOProvider, IOStack, OpenResult, OutputHandle};


// The double-boxing of the handles here isn't nice. I *think* that in
// principle we could turn the inner boxes into pointers and pass those
// around. But I can't get it to work in practice -- it may be Boxes of trait
// objects become fat pointers themselves. It's really not a big deal so let's
// just roll with it for now.

pub struct Engine<I: IOProvider>  {
    pub io: I,
    input_handles: Vec<Box<InputHandle>>,
    output_handles: Vec<Box<OutputHandle>>,
}


// The public interface.

impl<I: IOProvider> Engine<I> {
    pub fn new (io: I) -> Engine<I> {
        Engine {
            io: io,
            output_handles: Vec::new(),
            input_handles: Vec::new(),
        }
    }

    pub fn set_output_format (&mut self, outfmt: &str) -> () {
        // TODO: use enums for safety, etc.
        if outfmt == "xdv" {
            unsafe { c_api::tt_set_int_variable(b"no_pdf_output\0".as_ptr(), 1); }
        }
    }

    pub fn set_halt_on_error_mode (&mut self, halt_on_error: bool) -> () {
        let v = if halt_on_error { 1 } else { 0 };
        unsafe { c_api::tt_set_int_variable(b"halt_on_error_p\0".as_ptr(), v); }
    }

    // I/O helpers that are not part of the EngineInternals trait

    fn input_open_name_format(&mut self, name: &OsStr, format: FileFormat) -> OpenResult<InputHandle> {
        // TODO: shouldn't make the mutated version of `name` unless we need
        // to, but the first time I tried this I had trouble with `name` being
        // consumed. I'm sure I was just doing something silly.
        //
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
}


pub enum TeXResult {
    // The Errors possibility should only occur if halt_on_error_p is false --
    // otherwise, errors get upgraded to fatals. The fourth "history" option,
    // "HISTORY_FATAL_ERROR" results in an Err result, not Ok(TeXResult).
    Spotless,
    Warnings,
    Errors,
}

impl<'a> Engine<IOStack<'a>> {
    // These functions must go here since `assign_global_engine` must hardcode
    // the IOProvider type parameter.

    pub fn process_tex (&mut self, format_file_name: &str, input_file_name: &str) -> Result<TeXResult> {
        let cformat = CString::new(format_file_name)?;
        let cinput = CString::new(input_file_name)?;

        let result = unsafe {
            assign_global_engine (self, || {
                c_api::tt_misc_initialize(cformat.as_ptr());
                match c_api::tt_run_engine(cinput.as_ptr()) {
                    0 => Ok(TeXResult::Spotless),
                    1 => Ok(TeXResult::Warnings),
                    2 => Ok(TeXResult::Errors),
                    3 => {
                        let ptr = c_api::tt_get_error_message();
                        let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                        Err(ErrorKind::TeXError(msg).into())
                    },
                    x => Err(ErrorKind::TeXError(format!("internal error: unexpected 'history' value {}", x)).into())
                }
            })
        };

        // Close any files that were left open -- namely, stdout.
        self.input_handles.clear();
        self.output_handles.clear();

        result
    }

    pub fn process_xdvipdfmx (&mut self, dvi: &str, pdf: &str) -> Result<libc::c_int> {
        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let result = unsafe {
            assign_global_engine (self, || {
                match c_api::dvipdfmx_simple_main(cdvi.as_ptr(), cpdf.as_ptr()) {
                    99 => {
                        let ptr = c_api::tt_get_error_message();
                        let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                        Err(ErrorKind::DpxError(msg).into())
                    },
                    x => Ok(x)
                }
            })
        };

        self.input_handles.clear();
        self.output_handles.clear();

        result
    }
}


impl<T: IOProvider> EngineInternals for Engine<T> {
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
