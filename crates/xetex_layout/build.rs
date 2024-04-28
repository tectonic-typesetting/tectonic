// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Build script for the layout subsystem. Things get a little "fun" since we
//! use different font-finding and layout frameworks depending on the target
//! platform.
//!
//! Specifically, on macOS we use `CoreText`. On all other platforms, including
//! Windows, we use Fontconfig to discover fonts.

use std::{env, path::PathBuf};
use tectonic_cfg_support::target_cfg;

fn main() {
    // let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let is_mac_os = target_cfg!(target_os = "macos");

    // Include paths and settings exported by our internal dependencies.

    // TODO: Don't unwrap, won't exist on mac
    let fontconfig_include_path = env::var("DEP_FONTCONFIG_INCLUDE_PATH").unwrap();
    let freetype2_include_path = env::var("DEP_FREETYPE2_INCLUDE_PATH").unwrap();
    let graphite2_include_path = env::var("DEP_GRAPHITE2_INCLUDE_PATH").unwrap();
    let harfbuzz_include_path = env::var("DEP_HARFBUZZ_INCLUDE_PATH").unwrap();
    let icu_include_path = env::var("DEP_ICUUC_INCLUDE_PATH").unwrap();

    if is_mac_os {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    // OK, back to generic build rules.
    // Copy the static header file for C preprocessing convenience.

    let mut main_header_src = manifest_dir.clone();
    main_header_src.push("layout");
    main_header_src.push("tectonic_xetex_layout.h");

    let mut main_header_dest = PathBuf::from(out_dir.clone());
    main_header_dest.push("tectonic_xetex_layout.h");

    std::fs::copy(&main_header_src, &main_header_dest).expect("failed to copy main header");

    println!("cargo:rerun-if-changed=layout/tectonic_xetex_layout.h");

    // Cargo exposes this as the environment variable DEP_XXX_INCLUDE_PATH,
    // where XXX is the "links" setting in Cargo.toml. This is the key element
    // that allows us to have a network of crates containing both C/C++ and Rust
    // code that all interlink.

    print!("cargo:include-path={out_dir}");

    for item in fontconfig_include_path.split(';') {
        print!(";{item}");
    }

    for item in harfbuzz_include_path.split(';') {
        print!(";{item}");
    }

    for item in freetype2_include_path.split(';') {
        print!(";{item}");
    }

    for item in graphite2_include_path.split(';') {
        print!(";{item}");
    }

    for item in icu_include_path.split(';') {
        print!(";{item}");
    }

    println!();
}
