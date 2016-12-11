// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate md5;
extern crate mktemp;
extern crate zip;

use std::ffi::{CStr, CString};

mod c_api;
pub mod find;
pub mod kpse_api;
pub mod md5_api;


// Let's make this engine run!

pub struct Engine {
}

impl Engine {
    pub fn new () -> Engine {
        Engine {}
    }

    pub fn set_output_format (&mut self, outfmt: &str) -> () {
        // TODO: use enums for safety, etc.
        if outfmt == "xdv" {
            unsafe { c_api::tt_set_int_variable(b"no_pdf_output\0".as_ptr(), 1); }
        }
    }

    pub fn process (&mut self, format_file_name: &str, input_file_name: &str) -> Option<String> {
        let cformat = CString::new(format_file_name).unwrap();
        let cinput = CString::new(input_file_name).unwrap();
        let result = unsafe {
            c_api::tt_misc_initialize(cformat.as_ptr());
            c_api::tt_run_engine(cinput.as_ptr())
        };

        if result == 3 {
            let msg = unsafe {
                CStr::from_ptr(c_api::tt_get_error_message()).to_string_lossy().into_owned()
            };
            return Some(msg);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
