// src/engines/kpse_api.rs -- exposing a kpse-like API for the C/C++ code
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use libc;
use std::ffi::{CStr};
use std::io::{stderr, Write};
use std::ptr;

use super::c_api::c_format_to_rust;


#[no_mangle]
pub extern fn kpse_find_file(name: *const i8, format: libc::c_int, must_exist: libc::c_int) -> *const i8 {
    /* This function can never work for Tectonic because files in the bundle
     * can't be referenced by path names. */

    let rname = unsafe { CStr::from_ptr (name) };
    let rformat = c_format_to_rust (format);
    let rmust_exist = must_exist != 0;
    writeln!(&mut stderr(), "WARNING: always-fail kpse_find_file: {:?}, {:?} ({}), {}",
             rname, rformat, format, rmust_exist).expect ("stderr failed");
    ptr::null()
}
