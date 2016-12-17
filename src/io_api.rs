// src/io_api.rs -- exposing a simple I/O API for the C/C++ code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use libc;
use std::ffi::CStr;
use std::ptr;


/* The C/C++ API. */

#[no_mangle]
pub extern fn ttstub_output_open (name: *const i8) -> *const libc::c_void {
    let rname = unsafe { CStr::from_ptr(name) }.to_bytes();
    ptr::null()
}

#[no_mangle]
pub extern fn ttstub_output_putc (handle: *mut libc::c_void) -> libc::c_int {
    0
}
