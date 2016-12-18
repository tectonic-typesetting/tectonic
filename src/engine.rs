// src/engine.rs -- interface for the Tectonic engine
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString, OsString};
use std::fs::File;
use std::io::{stderr, stdout, Write};
use std::os::unix::io::{IntoRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::ptr;
use zip::result::ZipResult;

use ::{assign_global_engine, EngineInternals};
use bundle::Bundle;
use c_api;
use file_format::{format_to_extension, FileFormat};


pub enum OutputItem {
    // FIXME: this shouldn't be public but visibly checker complains
    // otherwise. The type is exposed in an impl of a private trait so I'm not
    // sure why there's a problem.
    Stdout,
    File(File)
}

pub struct Engine {
    bundle: Option<Bundle<File>>,
    output_handles: Vec<Box<OutputItem>>,
}


// The public interface.

impl Engine {
    pub fn new () -> Engine {
        Engine {
            bundle: None,
            output_handles: Vec::new(),
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
}


impl EngineInternals for Engine {
    fn get_readable_fd(&mut self, name: &Path, format: FileFormat, must_exist: bool) -> Option<RawFd> {
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

    type OutputHandle = OutputItem;

    fn output_open(&mut self, name: &Path) -> *const Self::OutputHandle {
        // TODO: use the I/O layer and write to a buffer!

        match File::create (name) {
            Ok(f) => {
                self.output_handles.push(Box::new(OutputItem::File(f)));
                &*self.output_handles[self.output_handles.len()-1]
            },
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: open of {} failed: {}",
                         name.display(), e).expect("stderr failed");
                ptr::null()
            }
        }
    }

    fn output_open_stdout(&mut self) -> *const Self::OutputHandle {
        self.output_handles.push(Box::new(OutputItem::Stdout));
        &*self.output_handles[self.output_handles.len()-1]
    }

    fn output_putc(&mut self, handle: *mut Self::OutputHandle, c: u8) -> bool {
        let rhandle: &mut OutputItem = unsafe { &mut *handle };
        let buf = &[c];

        let result = match *rhandle {
            OutputItem::Stdout => stdout().write(buf),
            OutputItem::File(ref mut f) => f.write(buf)
        };

        match result {
            Ok(_) => false,
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: write failed: {}", e).expect("stderr failed");
                true
            }
        }
    }

    fn output_flush(&mut self, handle: *mut Self::OutputHandle) -> bool {
        let rhandle: &mut OutputItem = unsafe { &mut *handle };

        let result = match *rhandle {
            OutputItem::Stdout => stdout().flush(),
            OutputItem::File(ref mut f) => f.flush()
        };

        match result {
            Ok(_) => false,
            Err(e) => {
                // TODO: better error handling
                writeln!(&mut stderr(), "WARNING: flush failed: {}", e).expect("stderr failed");
                true
            }
        }
    }

    fn output_close(&mut self, handle: *mut Self::OutputHandle) -> bool {
        let len = self.output_handles.len();

        for i in 0..len {
            let p: *const Self::OutputHandle = &*self.output_handles[i];

            if p == handle {
                self.output_handles.swap_remove(i);
                break;
            }
        }

        false
    }
}
