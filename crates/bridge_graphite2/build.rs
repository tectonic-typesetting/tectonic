// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! graphite2 build script. For now, we always find it externally. One day, we'd
//! like to be able to vendor it.

use std::env;
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
    // crates' build scripts as the environment variable
    // DEP_GRAPHITE2_INCLUDE_PATH, allowing them to find the headers internally.
    // If/when we start vendoring graphite2, this can become $OUT_DIR.

    let mut sep = "cargo:include-path=";

    dep.foreach_include_path(|p| {
        print!("{}{}", sep, p.to_str().unwrap());
        sep = ";";
    });

    println!();

    dep.emit();

    // As a special case, code that compiles against graphite2 must also
    // sometimes provide -DGRAPHITE2_STATIC. We'd prefer not to get into the
    // business of propagating arbitrary cflags through our build system, so we
    // indicate it with a specialized variable.

    let target = env::var("TARGET").unwrap();
    let rustflags = env::var("RUSTFLAGS").unwrap_or_default();
    let define_static_flag = if target.contains("-msvc") && rustflags.contains("+crt-static") {
        "1"
    } else {
        ""
    };

    println!("cargo:define_static={}", define_static_flag);
}
