// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Harfbuzz build script.

#[cfg(feature = "external-harfbuzz")]
mod inner {
    use tectonic_dep_support::{Configuration, Dependency, Spec};

    // TODO: ICU not necessary if Harfbuzz >= 2.5.
    struct HarfbuzzSpec;

    impl Spec for HarfbuzzSpec {
        fn get_pkgconfig_spec(&self) -> &str {
            "harfbuzz >= 1.4 harfbuzz-icu"
        }

        // TODO: can we ensure that the ICU and graphite2 options are enabled?
        fn get_vcpkg_spec(&self) -> &[&str] {
            &["harfbuzz"]
        }
    }

    pub fn build_harfbuzz() {
        let cfg = Configuration::default();
        let dep = Dependency::probe(HarfbuzzSpec, &cfg);

        // This is the key. What we print here will be propagated into depending
        // crates' build scripts as the envirnoment variable DEP_HARFBUZZ_INCLUDE_PATH,
        // allowing them to find the headers internally.

        let mut sep = "cargo:include-path=";

        dep.foreach_include_path(|p| {
            print!("{}{}", sep, p.to_str().unwrap());
            sep = ";";
        });

        println!();

        dep.emit();
    }
}

#[cfg(not(feature = "external-harfbuzz"))]
mod inner {
    use std::{env, ffi::OsStr, fs, path::PathBuf};

    pub fn build_harfbuzz() {
        let target = env::var("TARGET").unwrap();

        // Check that the submodule has been checked out.

        let src_dir = {
            let mut p = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
            p.push("harfbuzz");
            p.push("src");
            p
        };

        let test_file = src_dir.join("harfbuzz.cc");

        if !test_file.exists() {
            eprintln!("error: no such file {}", test_file.display());
            eprintln!(
                "   if you have checked out from Git, you probably need to fetch submodules:"
            );
            eprintln!("       git submodule update --init");
            eprintln!(
                "   This is needed because we are attempting to compile a local copy of Harfbuzz."
            );
            std::process::exit(1);
        }

        // Include paths exported by our internal dependencies:
        let graphite2_include_path = env::var("DEP_GRAPHITE2_INCLUDE_PATH").unwrap();
        let graphite2_static = !env::var("DEP_GRAPHITE2_DEFINE_STATIC").unwrap().is_empty();
        let icu_include_path = env::var("DEP_ICUUC_INCLUDE_PATH").unwrap();

        let mut cfg = cc::Build::new();

        cfg.cpp(true)
            .flag("-std=c++11")
            .warnings(false)
            .define("HAVE_GRAPHITE2", "1")
            .define("HAVE_ICU", "1")
            .file("harfbuzz/src/harfbuzz.cc")
            .file("harfbuzz/src/hb-icu.cc");

        for item in graphite2_include_path.split(';') {
            cfg.include(item);
        }

        for item in icu_include_path.split(';') {
            cfg.include(item);
        }

        if graphite2_static {
            cfg.define("GRAPHITE2_STATIC", "1");
        }

        if !target.contains("windows") {
            cfg.define("HAVE_PTHREAD", "1");
        }

        if target.contains("apple") {
            cfg.define("HAVE_CORETEXT", "1");
        }

        if target.contains("windows-gnu") {
            cfg.flag("-Wa,-mbig-obj");
        }

        cfg.compile("harfbuzz");

        // Copy the headers to have the same directory structure as a system install.

        let include_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        print!(
            "cargo:include-path={}",
            include_dir.to_str().expect("non-string-friendly OUT_DIR")
        );

        for item in graphite2_include_path.split(';') {
            print!(";{}", item);
        }

        for item in icu_include_path.split(';') {
            print!(";{}", item);
        }

        println!();

        let dest_dir = include_dir.join("harfbuzz");
        // CC build process already creates this for us:
        //fs::create_dir(&dest_dir).expect("error creating dest_dir");

        for entry in fs::read_dir(&src_dir).expect("failed to read dir") {
            let entry = entry.expect("failed to get dir entry");
            if entry.path().extension() == Some(OsStr::new("h")) {
                let hdest = dest_dir.join(entry.path().file_name().unwrap());
                fs::copy(entry.path(), hdest).expect("failed to copy header");
            }
        }
    }
}

fn main() {
    inner::build_harfbuzz();
}
