// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! The "newcli" command-line interface -- a "multitool" interface resembling
//! Cargo, as compared to the classic "rustc-like" CLI.

use std::ffi::OsString;

pub fn new_main(effective_args: &[OsString]) {
    println!("NEWCLI");
}
