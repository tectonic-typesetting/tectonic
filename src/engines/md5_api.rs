// src/engines/md5_api.rs -- exposing MD5 tools to the C/C++ code
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use libc;
use md5;
use std::slice;


#[no_mangle]
pub extern fn ttstub_get_file_md5(_path: *const i8, _digest: *mut [u8]) -> libc::c_int {
    // TODO: bother to implement this
    1
}


#[no_mangle]
pub extern fn ttstub_get_data_md5(data: *const u8, len: libc::size_t, digest: *mut u8) -> libc::c_int {
    let rdata = unsafe { slice::from_raw_parts(data, len) };
    let rdest = unsafe { slice::from_raw_parts_mut(digest, 16) };
    let rdigest = md5::compute(rdata);

    for i in 0..16 {
        rdest[i] = rdigest[i];
    }

    0
}
