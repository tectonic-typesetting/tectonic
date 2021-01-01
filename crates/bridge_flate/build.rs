// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

use std::{env, path::PathBuf};

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();

    // Cargo exposes this as the environment variable DEP_XXX_INCLUDE, where XXX
    // is the "links" setting in Cargo.toml. This is the key element that allows
    // us to have a network of crates containing both C/C++ and Rust code that
    // all interlink.
    println!("cargo:include={}", outdir);

    let mut header_path: PathBuf = outdir.into();
    header_path.push("tectonic_bridge_flate.h");

    let mut config = cbindgen::Config::default();
    config.cpp_compat = true;
    config.enumeration.prefix_with_name = true;

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .with_language(cbindgen::Language::C)
        .with_include_guard("TECTONIC_BRIDGE_FLATE_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&header_path);
}
