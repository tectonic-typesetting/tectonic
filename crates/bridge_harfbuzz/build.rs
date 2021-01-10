// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Harfbuzz build script.

use tectonic_dep_support::{Configuration, Dependency, Spec};

struct HarfbuzzSpec;

impl Spec for HarfbuzzSpec {
    fn get_pkgconfig_spec(&self) -> &str {
        "harfbuzz >= 1.4 harfbuzz-icu"
    }

    fn get_vcpkg_spec(&self) -> &[&str] {
        &["harfbuzz"]
    }
}

fn main() {
    let cfg = Configuration::default();
    let dep = Dependency::probe(HarfbuzzSpec, &cfg);

    // This is the key. What we print here will be propagated into depending
    // crates' build scripts as the envirnoment variable DEP_HARFBUZZ_INCLUDE,
    // allowing them to find the headers internally.
    dep.foreach_include_path(|p| {
        println!("cargo:include={}", p.to_str().unwrap());
    });

    dep.emit();
}
