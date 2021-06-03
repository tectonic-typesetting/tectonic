// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

use std::{env, path::PathBuf};

fn main() {
    let bc_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // cbindgen to generate the C header from our Rust code.

    let mut gen_header_path: PathBuf = out_dir.clone().into();
    gen_header_path.push("bibtex_bindings.h");

    println!("cargo:rerun-if-changed=src/lib.rs");

    let mut config = cbindgen::Config {
        cpp_compat: true,
        ..Default::default()
    };
    config.enumeration.prefix_with_name = true;

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(&manifest_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("BIBTEX_BINDINGS_H")
        .with_style(cbindgen::Style::Type)
        .rename_item("CoreBridgeState", "ttbc_state_t") // unfortunately we need to propagate this rename
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&gen_header_path);

    // Now we can compile the C code

    let mut build = cc::Build::new();
    build
        .warnings(true)
        .file("bibtex/bibtex.c")
        .include(&bc_include_dir)
        .include(&out_dir)
        .compile("libtectonic_engine_bibtex.a");

    println!("cargo:rerun-if-changed=bibtex/bibtex.c");

    // Workaround so that we can `cargo package` this crate. Cf
    // https://github.com/eqrion/cbindgen/issues/560 . cbindgen calls `cargo
    // metadata` which creates a new Cargo.lock file when building this crate as
    // part of its packaging process. This isn't noticed in regular builds since
    // they occur in a workspace context. Lame but effective solution:
    // unconditionally blow away the file.

    manifest_dir.push("Cargo.lock");
    if let Err(e) = std::fs::remove_file(&manifest_dir) {
        if e.kind() != std::io::ErrorKind::NotFound {
            panic!("unexpected error clearing local Cargo.lock: {}", e);
        }
    }
}
