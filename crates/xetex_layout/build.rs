// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Build script for the layout subsystem. Things get a little "fun" since we
//! use different font-finding and layout frameworks depending on the target
//! platform.
//!
//! Specifically, on macOS we use `CoreText`. On all other platforms, including
//! Windows, we use Fontconfig to discover fonts.

use std::{env, path::PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    // Copy the generated header file for exported functions.

    let mut main_header_src = manifest_dir.clone();
    main_header_src.push("layout");
    main_header_src.push("tectonic_xetex_layout.h");

    let mut main_header_dest = PathBuf::from(out_dir.clone());
    main_header_dest.push("tectonic_xetex_layout.h");

    std::fs::copy(&main_header_src, &main_header_dest).expect("failed to copy main header");

    println!("cargo:rerun-if-changed=layout/tectonic_xetex_layout.h");
    println!("cargo:include-path={out_dir}");
}
