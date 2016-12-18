// src/io_api.rs -- exposing a simple I/O API for the C/C++ code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use libc;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use ::{with_global_engine, Engine, EngineInternals};


#[no_mangle]
pub extern fn ttstub_output_open (name: *const i8) -> *const libc::c_void {
    let rname = Path::new (OsStr::from_bytes (unsafe { CStr::from_ptr(name) }.to_bytes()));
    with_global_engine(|eng| {
        eng.output_open (&rname) as *const _
    })
}

#[no_mangle]
pub extern fn ttstub_output_putc (handle: *mut libc::c_void, c: libc::c_int) -> libc::c_int {
    let rhandle = handle as *mut <Engine as EngineInternals>::OutputHandle;
    let rc = c as u8;

    let error_occurred = with_global_engine(|eng| {
        eng.output_putc(rhandle, rc)
    });

    if error_occurred {
        libc::EOF
    } else {
        c
    }
}

#[no_mangle]
pub extern fn ttstub_output_close (handle: *mut libc::c_void) -> libc::c_int {
    let rhandle = handle as *mut <Engine as EngineInternals>::OutputHandle;

    let error_occurred = with_global_engine(|eng| {
        eng.output_close(rhandle)
    });

    if error_occurred {
        1
    } else {
        0
    }
}
