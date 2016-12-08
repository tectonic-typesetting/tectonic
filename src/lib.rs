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
    fn main_body(input_file_name: *const i8) -> libc::c_int;
    fn tt_misc_initialize(dump_name: *const i8) -> ();
}


// Let's make this engine run!

pub struct Engine {
}

impl Engine {
    pub fn new () -> Engine {
        Engine {}
    }

    pub fn process (&mut self, format_file_name: &str, input_file_name: &str) -> libc::c_int {
        let cformat = CString::new(format_file_name).unwrap();
        let cinput = CString::new(input_file_name).unwrap();

        unsafe {
            tt_misc_initialize(cformat.as_ptr());
            main_body(cinput.as_ptr())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
