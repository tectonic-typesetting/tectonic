// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! graphite2 build script. For now, we always find it externally. One day, we'd
//! like to be able to vendor it.

use tectonic_dep_support::{Configuration, Dependency, Spec};

struct Graphite2Spec;

impl Spec for Graphite2Spec {
    fn get_pkgconfig_spec(&self) -> &str {
        "graphite2"
    }

    fn get_vcpkg_spec(&self) -> &[&str] {
        &["graphite2"]
    }
}

fn main() {
    let cfg = Configuration::default();
    let dep = Dependency::probe(Graphite2Spec, &cfg);

    // This is the key. What we print here will be propagated into depending
    // crates' build scripts as the environment variable DEP_GRAPHITE2_INCLUDE,
    // allowing them to find the headers internally. If/when we start vendoring
    // graphite2, this can become $OUT_DIR.
    dep.foreach_include_path(|p| {
        println!("cargo:include={}", p.to_str().unwrap());
    });

    dep.emit();
}
