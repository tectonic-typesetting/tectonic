// src/c_api.rs -- exposing the C/C++ API
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use libc;

// The API of the C/C++ library that we interact with to drive the engine
// under the hood.

extern {
    pub fn tt_get_error_message() -> *const i8;
    pub fn tt_misc_initialize(dump_name: *const i8) -> ();
    pub fn tt_set_int_variable(var_name: *const u8, value: libc::c_int) -> libc::c_int;
    //pub fn tt_set_string_variable(var_name: *const u8, value: *const i8) -> libc::c_int;
    pub fn tt_run_engine(input_file_name: *const i8) -> libc::c_int;
}
