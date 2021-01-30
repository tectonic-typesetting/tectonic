// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! ICU build script.
//!
//! We find it externally, and probably will continue to do so for as long as
//! this crate is needed. Vendoring the ICU library is almost certainly not
//! something that one should do.

use tectonic_dep_support::{Configuration, Dependency, Spec};

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
}
