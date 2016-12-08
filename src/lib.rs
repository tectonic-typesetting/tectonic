#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate mktemp;
extern crate zip;

use std::ffi::CString;

pub mod find;
pub mod kpse_api;

// We only need a few C functions.

extern {
    fn tt_misc_initialize(dump_name: *const i8) -> ();
    fn tt_set_int_variable(var_name: *const u8, value: libc::c_int) -> libc::c_int;
    fn tt_set_string_variable(var_name: *const u8, value: *const i8) -> libc::c_int;
    fn tt_run_engine(input_file_name: *const i8) -> libc::c_int;
}


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
            unsafe { tt_set_int_variable(b"no_pdf_output\0".as_ptr(), 1); }
        }
    }

    pub fn set_dvi_comment (&mut self, comment: &str) -> () {
        let ccomment = CString::new(comment).unwrap();

        unsafe {
            tt_set_string_variable(b"output_comment\0".as_ptr(), ccomment.as_ptr());
        }
    }

    pub fn process (&mut self, format_file_name: &str, input_file_name: &str) -> libc::c_int {
        let cformat = CString::new(format_file_name).unwrap();
        let cinput = CString::new(input_file_name).unwrap();

        unsafe {
            tt_misc_initialize(cformat.as_ptr());
            tt_run_engine(cinput.as_ptr())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
