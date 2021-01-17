// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

use std::{env, fs, io::ErrorKind, path::PathBuf};

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();

    // Cargo exposes this as the environment variable DEP_XXX_INCLUDE, where XXX
    // is the "links" setting in Cargo.toml. This is the key element that allows
    // us to have a network of crates containing both C/C++ and Rust code that
    // all interlink.
    println!("cargo:include={}", outdir);

    // cbindgen to generate the C header from our Rust code.

    let mut gen_header_path: PathBuf = outdir.clone().into();
    gen_header_path.push("tectonic_bridge_core_generated.h");

    println!("cargo:rerun-if-changed=src/lib.rs");

    let mut config = cbindgen::Config {
        cpp_compat: true,
        ..Default::default()
    };
    config.enumeration.prefix_with_name = true;

    let mut manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(&manifest_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("TECTONIC_BRIDGE_CORE_GENERATED_H")
        .with_style(cbindgen::Style::Type)
        .with_after_include(
            "
typedef struct ttbc_input_handle_t ttbc_input_handle_t;
typedef struct ttbc_output_handle_t ttbc_output_handle_t;

typedef ttbc_input_handle_t *rust_input_handle_t;
typedef ttbc_output_handle_t *rust_output_handle_t;",
        )
        .rename_item("CoreBridgeState", "ttbc_state_t")
        .rename_item("Diagnostic", "ttbc_diagnostic_t")
        .rename_item("FileFormat", "ttbc_file_format")
        .rename_item("InputHandle", "ttbc_input_handle_t")
        .rename_item("OutputHandle", "ttbc_output_handle_t")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&gen_header_path);

    // Copy the static header file for C preprocessing convenience.

    let mut main_header_src = manifest_dir.clone();
    main_header_src.push("support");
    main_header_src.push("tectonic_bridge_core.h");

    let mut main_header_dest = PathBuf::from(outdir.clone());
    main_header_dest.push("tectonic_bridge_core.h");

    std::fs::copy(&main_header_src, &main_header_dest).expect("failed to copy main header");

    println!("cargo:rerun-if-changed=support/tectonic_bridge_core.h");

    // Now that we have that, we can compile our C support code.

    let mut build = cc::Build::new();
    build
        .warnings(true)
        .file("support/support.c")
        .include(&outdir)
        .compile("libtectonic_bridge_core.a");

    println!("cargo:rerun-if-changed=support/support.c");

    // Workaround so that we can `cargo package` this crate. Cf
    // https://github.com/eqrion/cbindgen/issues/560 . cbindgen calls `cargo
    // metadata` which creates a new Cargo.lock file when building this crate as
    // part of its packaging process. This isn't noticed in regular builds since
    // they occur in a workspace context. Lame but effective solution:
    // unconditionally blow away the file.

    manifest_dir.push("Cargo.lock");
    if let Err(e) = fs::remove_file(&manifest_dir) {
        if e.kind() != ErrorKind::NotFound {
            panic!("unexpected error clearing local Cargo.lock: {}", e);
        }
    }
}
