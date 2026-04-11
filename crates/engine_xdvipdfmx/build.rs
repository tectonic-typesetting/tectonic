// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! xdvipdfmx build script. Builds and links our locally modified copy of the tool.

use std::env;

fn main() {
    let bc_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();
    let pi_include_path = env::var("DEP_TECTONIC_PDF_IO_INCLUDE_PATH").unwrap();

    let mut build = cc::Build::new();
    build
        .warnings(true)
        .file("xdvipdfmx/dvipdfmx.c")
        .include(&bc_include_dir);

    for item in pi_include_path.split(';') {
        build.include(item);
    }

    build.compile("libtectonic_engine_xdvipdfmx.a");

    println!("cargo:rerun-if-changed=xdvipdfmx/dvipdfmx.c");
    println!("cargo:rerun-if-changed=xdvipdfmx/xdvipdfmx_bindings.h");
}
