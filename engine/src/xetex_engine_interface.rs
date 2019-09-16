#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::streq_ptr;
use crate::xetex_ini::{
    halt_on_error_p, in_initex_mode, semantic_pagination_enabled, synctex_enabled,
};

/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
/* engine-interface.c: programmatic interface to control the engine behavior
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* These functions aren't used within the C/C++ library, but are called
 * by the Rust code to configure the XeTeX engine before launching it. */
#[no_mangle]
pub unsafe extern "C" fn tt_xetex_set_int_variable(mut var_name: *mut i8, mut value: i32) -> i32 {
    if streq_ptr(var_name, b"halt_on_error_p\x00" as *const u8 as *const i8) {
        halt_on_error_p = value
    } else if streq_ptr(var_name, b"in_initex_mode\x00" as *const u8 as *const i8) {
        in_initex_mode = value != 0i32
    } else if streq_ptr(var_name, b"synctex_enabled\x00" as *const u8 as *const i8) {
        synctex_enabled = (value != 0i32) as i32
    } else if streq_ptr(
        var_name,
        b"semantic_pagination_enabled\x00" as *const u8 as *const i8,
    ) {
        semantic_pagination_enabled = value != 0i32
    } else {
        return 1i32;
    } /* Uh oh: unrecognized variable */
    0i32
    /* success */
}
#[no_mangle]
pub unsafe extern "C" fn tt_xetex_set_string_variable(
    mut var_name: *mut i8,
    mut value: *mut i8,
) -> i32 {
    /* Currently unused; see Git history for how we used to set output_comment */
    1
}
