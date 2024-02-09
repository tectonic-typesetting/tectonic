// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! ICU build script.
//!
//! We find it externally, and probably will continue to do so for as long as
//! this crate is needed. Vendoring the ICU library is almost certainly not
//! something that one should do.

use tectonic_dep_support::{Backend, Configuration, Dependency, Spec};

struct IcuSpec;

impl Spec for IcuSpec {
    fn get_pkgconfig_spec(&self) -> &str {
        "icu-uc"
    }

    fn get_vcpkg_spec(&self) -> &[&str] {
        &["icu"]
    }
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let cfg = Configuration::default();
    let dep = Dependency::probe(IcuSpec, &cfg);

    // This is the key. What we print here will be propagated into depending
    // crates' build scripts as the environment variable DEP_ICUUC_INCLUDE_PATH,
    // allowing them to find the headers internally.

    let mut sep = "cargo:include-path=";

    dep.foreach_include_path(|p| {
        print!("{}{}", sep, p.to_str().unwrap());
        sep = ";";
    });

    println!();

    dep.emit();

    let icu_version = dep.version().split(".").collect::<Vec<_>>();
    let &[major, minor, ..] = &*icu_version else {
        panic!()
    };
    println!("cargo:rustc-env=ICU_MAJOR_VERSION={}", major);
    println!("cargo:rustc-env=ICU_MINOR_VERSION={}", minor);

    // vcpkg-rs is not guaranteed to emit libraries in the order required by a
    // single-pass linker, so we might need to make sure that's done right.

    if cfg.backend == Backend::Vcpkg && target.contains("-linux-") {
        // add icudata to the end of the list of libs as vcpkg-rs
        // does not order individual libraries as a single pass
        // linker requires.
        println!("cargo:rustc-link-lib=icudata");
    }
}
