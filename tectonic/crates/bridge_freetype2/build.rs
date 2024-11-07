// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! FreeType2 build script. For now, we always find it externally. One day, we'd
//! like to be able to vendor it. When that day comes, we should leverage
//! existing work done in the other freetype-sys crates, preferably just using
//! them directly instead of duplicating effort.

use tectonic_dep_support::{Configuration, Dependency, Spec};

struct Freetype2Spec;

impl Spec for Freetype2Spec {
    fn get_pkgconfig_spec(&self) -> &str {
        "freetype2"
    }

    fn get_vcpkg_spec(&self) -> &[&str] {
        &["freetype"]
    }
}

fn main() {
    let cfg = Configuration::default();
    let dep = Dependency::probe(Freetype2Spec, &cfg);

    // This is the key. What we print here will be propagated into depending
    // crates' build scripts as the environment variable DEP_FREETYPE2_INCLUDE_PATH,
    // allowing them to find the headers internally. If/when we start vendoring
    // FreeType, this can become $OUT_DIR.
    let mut sep = "cargo:include-path=";

    dep.foreach_include_path(|p| {
        print!("{}{}", sep, p.to_str().unwrap());
        sep = ";";
    });

    println!();

    dep.emit();
}
