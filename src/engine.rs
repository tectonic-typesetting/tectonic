// src/engine.rs -- interface for the Tectonic engine
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString, OsString};
use std::fs::File;
use std::os::unix::io::{IntoRawFd, RawFd};
use std::path::{Path, PathBuf};
use zip::result::ZipResult;

use bundle::Bundle;
use c_api;
use file_format::{format_to_extension, FileFormat};


pub struct Engine {
    bundle: Option<Bundle<File>>,
}

// The C code relies on an enormous number of global variables so, despite our
// fancy API, there can only ever actually be one Engine instance. (For now.)
// Here we set up the infrastructure to manage this. Of course, this is
// totally un-thread-safe, etc., because the underlying C code is.

// note: ptr::null_mut() gives me a compile error related to const fns right now.
static mut GLOBAL_ENGINE_PTR: *mut Engine = 0 as *mut _;

// This wraps a Rust function called by the C code via some ttstub_*() function.
pub fn with_global_engine<F, T> (f: F) -> T where F: FnOnce(&mut Engine) -> T {
    unsafe { f(&mut *GLOBAL_ENGINE_PTR) }
}

// This wraps any activities that cause the C code to spin up.
unsafe fn assign_global_engine<F, T> (engine: &mut Engine, f: F) -> T where F: FnOnce() -> T {
    GLOBAL_ENGINE_PTR = engine;
    let rv = f();
    GLOBAL_ENGINE_PTR = 0 as *mut _;
    rv
}


// Let's make this engine run!

impl Engine {
    pub fn new () -> Engine {
        Engine {
            bundle: None,
        }
    }

    pub fn set_output_format (&mut self, outfmt: &str) -> () {
        // TODO: use enums for safety, etc.
        if outfmt == "xdv" {
            unsafe { c_api::tt_set_int_variable(b"no_pdf_output\0".as_ptr(), 1); }
        }
    }

    pub fn use_bundle (&mut self, path: &Path) -> ZipResult<()> {
        match Bundle::open (path) {
            Ok(b) => { self.bundle = Some(b) ; Ok(()) },
            Err(e) => Err(e)
        }
    }

    pub fn process (&mut self, format_file_name: &str, input_file_name: &str) -> Option<String> {
        let cformat = CString::new(format_file_name).unwrap();
        let cinput = CString::new(input_file_name).unwrap();

        unsafe {
            assign_global_engine (self, || {
                c_api::tt_misc_initialize(cformat.as_ptr());
                let result = c_api::tt_run_engine(cinput.as_ptr());

                if result == 3 {
                    let ptr = c_api::tt_get_error_message();
                    let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    return Some(msg)
                }

                None
            })
        }
    }

    pub fn get_readable_fd(&mut self, name: &Path, format: FileFormat, must_exist: bool) -> Option<RawFd> {
        /* We currently don't care about must_exist. */

        /* For now: if we can open straight off of the filesystem, do that. No
         * bundle needed. */

        if let Ok(f) = File::open (name) {
            return Some(f.into_raw_fd());
        }

        let mut ext = PathBuf::from (name);
        let mut ename = OsString::from (ext.file_name ().unwrap ());
        ename.push (format_to_extension (format));
        ext.set_file_name (ename);

        if let Ok(f) = File::open (ext.clone ()) {
            return Some(f.into_raw_fd());
        }

        /* If the bundle has been opened, see if it's got the file. */

        match self.bundle {
            Some(ref mut bundle) => bundle.get_readable_fd(name, format, must_exist),
            None => None
        }
    }
}
