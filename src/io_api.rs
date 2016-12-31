// src/io_api.rs -- exposing a simple I/O API for the C/C++ code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use libc;
use std::ffi::{CStr, OsStr};
use std::io::{SeekFrom};
use std::os::unix::ffi::OsStrExt;
use std::ptr;
use std::slice;

use ::{with_global_engine, EngineInternals};
use c_api::c_format_to_rust;
use io::{InputHandle, OutputHandle};


#[no_mangle]
pub extern fn ttstub_output_open (name: *const i8, is_gz: libc::c_int) -> *const libc::c_void {
    let rname = OsStr::from_bytes (unsafe { CStr::from_ptr(name) }.to_bytes());
    let ris_gz = is_gz != 0;

    with_global_engine(|eng| {
        eng.output_open (&rname, ris_gz) as *const _
    })
}

#[no_mangle]
pub extern fn ttstub_output_open_stdout () -> *const libc::c_void {
    with_global_engine(|eng| {
        eng.output_open_stdout () as *const _
    })
}

#[no_mangle]
pub extern fn ttstub_output_putc (handle: *mut libc::c_void, c: libc::c_int) -> libc::c_int {
    let rhandle = handle as *mut OutputHandle;
    let rc = c as u8;

    let error_occurred = with_global_engine(|eng| {
        eng.output_write(rhandle, &[rc])
    });

    if error_occurred {
        libc::EOF
    } else {
        c
    }
}

#[no_mangle]
pub extern fn ttstub_output_write (handle: *mut libc::c_void, data: *const u8, len: libc::size_t) -> libc::size_t {
    let rhandle = handle as *mut OutputHandle;
    let rdata = unsafe { slice::from_raw_parts(data, len) };

    // NOTE: we use f.write_all() so partial writes are not gonna be a thing.

    let error_occurred = with_global_engine(|eng| {
        eng.output_write(rhandle, rdata)
    });

    if error_occurred {
        0
    } else {
        len
    }
}

#[no_mangle]
pub extern fn ttstub_output_flush (handle: *mut libc::c_void) -> libc::c_int {
    let rhandle = handle as *mut OutputHandle;

    let error_occurred = with_global_engine(|eng| {
        eng.output_flush(rhandle)
    });

    if error_occurred {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern fn ttstub_output_close (handle: *mut libc::c_void) -> libc::c_int {
    if handle == 0 as *mut _ {
        return 0; // This is/was the behavior of close_file() in C.
    }

    let rhandle = handle as *mut OutputHandle;

    let error_occurred = with_global_engine(|eng| {
        eng.output_close(rhandle)
    });

    if error_occurred {
        1
    } else {
        0
    }
}


#[no_mangle]
pub extern fn ttstub_input_open (name: *const i8, format: libc::c_int, is_gz: libc::c_int) -> *const libc::c_void {
    let rname = OsStr::from_bytes (unsafe { CStr::from_ptr(name) }.to_bytes());
    let rformat = c_format_to_rust (format);
    let ris_gz = is_gz != 0;

    match rformat {
        Some(fmt) => {
            with_global_engine(|eng| {
                eng.input_open (&rname, fmt, ris_gz) as *const _
            })
        },
        None => ptr::null()
    }
}

#[no_mangle]
pub extern fn ttstub_input_get_size (handle: *mut libc::c_void) -> libc::size_t {
    let rhandle = handle as *mut InputHandle;

    with_global_engine(|eng| {
        eng.input_get_size(rhandle)
    })
}

#[no_mangle]
pub extern fn ttstub_input_seek (handle: *mut libc::c_void, offset: libc::ssize_t, whence: libc::c_int) -> libc::size_t {
    let rhandle = handle as *mut InputHandle;

    let rwhence = match whence {
        libc::SEEK_SET => SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => SeekFrom::Current(offset as i64),
        libc::SEEK_END => SeekFrom::End(offset as i64),
        _ => panic!("Unexpected \"whence\" parameter to fseek() wrapper: {}", whence),
    };

    with_global_engine(|eng| {
        eng.input_seek(rhandle, rwhence) as libc::size_t
    })
}

#[no_mangle]
pub extern fn ttstub_input_getc (handle: *mut libc::c_void) -> libc::c_int {
    let rhandle = handle as *mut InputHandle;
    let mut buf = [0u8; 1];

    let error_occurred = with_global_engine(|eng| {
        eng.input_read(rhandle, &mut buf)
    });

    if error_occurred {
        libc::EOF
    } else {
        buf[0] as libc::c_int
    }
}

#[no_mangle]
pub extern fn ttstub_input_read (handle: *mut libc::c_void, data: *mut u8, len: libc::size_t) -> libc::ssize_t {
    let rhandle = handle as *mut InputHandle;
    let rdata = unsafe { slice::from_raw_parts_mut(data, len) };

    let error_occurred = with_global_engine(|eng| {
        eng.input_read(rhandle, rdata)
    });

    if error_occurred {
        -1
    } else {
        len as isize
    }
}

#[no_mangle]
pub extern fn ttstub_input_close (handle: *mut libc::c_void) -> libc::c_int {
    if handle == 0 as *mut _ {
        return 0; // This is/was the behavior of close_file() in C.
    }

    let rhandle = handle as *mut InputHandle;

    let error_occurred = with_global_engine(|eng| {
        eng.input_close(rhandle)
    });

    if error_occurred {
        1
    } else {
        0
    }
}
