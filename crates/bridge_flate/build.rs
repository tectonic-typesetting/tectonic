// Copyright 2020-2021 the Tectonic Project
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

    let mut config = cbindgen::Config {
        cpp_compat: true,
        ..Default::default()
    };
    config.enumeration.prefix_with_name = true;

    let mut manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    // Experimental (2021 June): currently the build of `tectonic` on docs.rs
    // fails because cbindgen calls `cargo metadata`, which has to hit the
    // network because the standalone crate has not Cargo.lock file -- and
    // docs.rs disable network access. We can't control the Cargo command line,
    // but hopefully this environment variable will tell Cargo not to try? I
    // don't know if Cargo will succeed this way, but I think the only way to
    // test is to make a release and see.
    std::env::set_var("CARGO_NET_OFFLINE", "1");

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(&manifest_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("TECTONIC_BRIDGE_FLATE_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&header_path);

    // Workaround so that we can `cargo package` this crate. Cf
    // https://github.com/eqrion/cbindgen/issues/560 . cbindgen calls `cargo
    // metadata` which creates a new Cargo.lock file when building this crate as
    // part of its packaging process. This isn't noticed in regular builds since
    // they occur in a workspace context. Lame but effective solution:
    // unconditionally blow away the file.
    manifest_dir.push("Cargo.lock");
    let _ignored = std::fs::remove_file(&manifest_dir);
}
