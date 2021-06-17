// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

use std::{env, path::PathBuf};

fn main() {
    let bc_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut bt_include_dir = manifest_dir;
    bt_include_dir.push("bibtex");

    let mut build = cc::Build::new();
    build
        .warnings(true)
        .file("bibtex/bibtex.c")
        .include(&bt_include_dir)
        .include(&bc_include_dir)
        .compile("libtectonic_engine_bibtex.a");

    println!("cargo:rerun-if-changed=bibtex/bibtex.c");
    println!("cargo:rerun-if-changed=bibtex/bibtex_bindings.h");
}
