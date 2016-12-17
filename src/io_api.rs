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
    let handle = with_global_engine(|eng| {
        eng.output_open (&rname)
    });
    match handle {
        Some(r) => (r as *const <Engine as EngineInternals>::OutputHandle) as *const _,
        None => 0 as *const _
    }
}

#[no_mangle]
pub extern fn ttstub_output_putc (handle: *mut libc::c_void, c: libc::c_int) -> libc::c_int {
    with_global_engine(|eng| {
        let mut rhandle = unsafe { (handle as *mut <Engine as EngineInternals>::OutputHandle).as_mut() };
        eng.output_putc(rhandle.as_mut().unwrap(), c)
    })
}
