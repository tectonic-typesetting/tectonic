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
}


// Let's make this engine run!

pub struct Engine {
}

impl Engine {
    pub fn new () -> Engine {
        Engine {}
    }

    pub fn process (&mut self, input_file_name: &str) -> libc::c_int {
        let cname = CString::new(input_file_name).unwrap();
        unsafe {
            main_body (cname.as_ptr())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
