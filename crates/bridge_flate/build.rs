// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! Build script for bridging flate2 into C code

use std::{env, path::PathBuf};

fn main() {
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    let mut include_dir = manifest_dir;
    include_dir.push("include");

    // Cargo exposes this as the environment variable DEP_XXX_INCLUDE, where XXX
    // is the "links" setting in Cargo.toml. This is the key element that allows
    // us to have a network of crates containing both C/C++ and Rust code that
    // all interlink.
    println!("cargo:include={}", include_dir.display());
}
