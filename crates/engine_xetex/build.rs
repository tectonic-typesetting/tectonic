// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

use std::{env, path::PathBuf};
use tectonic_cfg_support::*;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Include paths exported by our internal dependencies.

    let xetex_layout_include_path = env::var("DEP_TECTONIC_XETEX_LAYOUT_INCLUDE_PATH").unwrap();
    let pdfio_include_path = env::var("DEP_TECTONIC_PDF_IO_INCLUDE_PATH").unwrap();
    let core_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();
    let flate_include_dir = env::var("DEP_TECTONIC_BRIDGE_FLATE_INCLUDE").unwrap();
    let graphite2_include_path = env::var("DEP_GRAPHITE2_INCLUDE_PATH").unwrap();
    let graphite2_static = !env::var("DEP_GRAPHITE2_DEFINE_STATIC").unwrap().is_empty();
    let harfbuzz_include_path = env::var("DEP_HARFBUZZ_INCLUDE_PATH").unwrap();
    let fontconfig_include_path = env::var("DEP_FONTCONFIG_INCLUDE_PATH");
    let icu_include_path = env::var("DEP_ICUUC_INCLUDE_PATH").unwrap();

    // If we want to profile, the default assumption is that we must force the
    // compiler to include frame pointers. We whitelist platforms that are
    // known to be able to profile *without* frame pointers: currently, only
    // Linux/x86_64.

    let profile_target_requires_frame_pointer: bool =
        target_cfg!(not(all(target_os = "linux", target_arch = "x86_64")));

    #[allow(unexpected_cfgs)]
    const PROFILE_BUILD_ENABLED: bool = cfg!(feature = "profile");

    let profile_config = |cfg: &mut cc::Build| {
        if PROFILE_BUILD_ENABLED {
            cfg.debug(true)
                .force_frame_pointer(profile_target_requires_frame_pointer);
        }
    };

    // Time to set up the C/C++ support libraries.

    let mut c_cfg = cc::Build::new();
    let mut cxx_cfg = cc::Build::new();

    cxx_cfg.cpp(true);

    for flag in C_FLAGS {
        c_cfg.flag_if_supported(flag);
    }

    for flag in CXX_FLAGS {
        cxx_cfg.flag_if_supported(flag);
    }

    profile_config(&mut c_cfg);
    profile_config(&mut cxx_cfg);

    for p in &[".", &core_include_dir, &flate_include_dir] {
        c_cfg.include(p);
        cxx_cfg.include(p);
    }

    for item in xetex_layout_include_path.split(';') {
        c_cfg.include(item);
        cxx_cfg.include(item);
    }

    for item in pdfio_include_path.split(';') {
        c_cfg.include(item);
        cxx_cfg.include(item);
    }

    for item in harfbuzz_include_path.split(';') {
        c_cfg.include(item);
        cxx_cfg.include(item);
    }

    for item in graphite2_include_path.split(';') {
        c_cfg.include(item);
        cxx_cfg.include(item);
    }

    for item in icu_include_path.split(';') {
        c_cfg.include(item);
        cxx_cfg.include(item);
    }

    if let Ok(fc_includes) = fontconfig_include_path {
        for item in fc_includes.split(';') {
            c_cfg.include(item);
            cxx_cfg.include(item);
        }
    }

    if graphite2_static {
        c_cfg.define("GRAPHITE2_STATIC", "1");
        cxx_cfg.define("GRAPHITE2_STATIC", "1");
    }

    // Platform-specific adjustments:

    let is_mac_os = target_cfg!(target_os = "macos");

    if is_mac_os {
        c_cfg.define("XETEX_MAC", Some("1"));
        c_cfg.file("xetex/xetex-macos.c");
        cxx_cfg.define("XETEX_MAC", Some("1"));
    }

    let is_big_endian = target_cfg!(target_endian = "big");
    if is_big_endian {
        c_cfg.define("WORDS_BIGENDIAN", "1");
        cxx_cfg.define("WORDS_BIGENDIAN", "1");
    }

    if target.contains("-msvc") {
        c_cfg.flag("/EHsc");
        cxx_cfg.flag("/EHsc");
    }

    // OK, back to generic build rules.

    for file in C_FILES {
        c_cfg.file(file);
    }

    for file in CXX_FILES {
        cxx_cfg.file(file);
    }

    c_cfg.compile("libtectonic_engine_xetex_c.a");
    cxx_cfg.compile("libtectonic_engine_xetex_cxx.a");

    // Rebuild if C/C++ files have changed. We scan the whole directory to get
    // the headers too.

    for file in PathBuf::from("xetex").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}

const C_FLAGS: &[&str] = &[
    "-Wall",
    "-Wcast-qual",
    "-Wdate-time",
    "-Wendif-labels",
    "-Wextra",
    "-Wextra-semi",
    "-Wformat=2",
    "-Winit-self",
    "-Wlogical-op",
    "-Wmissing-declarations",
    "-Wmissing-include-dirs",
    "-Wmissing-prototypes",
    "-Wmissing-variable-declarations",
    "-Wnested-externs",
    "-Wold-style-definition",
    "-Wpointer-arith",
    "-Wredundant-decls",
    "-Wstrict-prototypes",
    "-Wsuggest-attribute=format",
    "-Wswitch-bool",
    "-Wundef",
    "-Wwrite-strings",
    // TODO: Fix existing warnings before enabling these:
    // "-Wbad-function-cast",
    // "-Wcast-align",
    // "-Wconversion",
    // "-Wdouble-promotion",
    // "-Wshadow",
    // "-Wsuggest-attribute=const",
    // "-Wsuggest-attribute=noreturn",
    // "-Wsuggest-attribute=pure",
    // "-Wunreachable-code-aggresive",
    "-Wno-unused-parameter",
    "-Wno-implicit-fallthrough",
    "-Wno-sign-compare",
    "-std=gnu11",
];

const C_FILES: &[&str] = &[
    "xetex/xetex-engine-interface.c",
    "xetex/xetex-errors.c",
    "xetex/xetex-ext.c",
    "xetex/xetex-ini.c",
    "xetex/xetex-io.c",
    "xetex/xetex-linebreak.c",
    "xetex/xetex-math.c",
    "xetex/xetex-output.c",
    "xetex/xetex-pagebuilder.c",
    "xetex/xetex-pic.c",
    "xetex/xetex-scaledmath.c",
    "xetex/xetex-shipout.c",
    "xetex/xetex-stringpool.c",
    "xetex/xetex-synctex.c",
    "xetex/xetex-texmfmp.c",
    "xetex/xetex-xetex0.c",
];

const CXX_FLAGS: &[&str] = &[
    "-std=c++14",
    "-Wall",
    "-Wdate-time",
    "-Wendif-labels",
    "-Wextra",
    "-Wformat=2",
    "-Wlogical-op",
    "-Wmissing-declarations",
    "-Wmissing-include-dirs",
    "-Wpointer-arith",
    "-Wredundant-decls",
    "-Wsuggest-attribute=noreturn",
    "-Wsuggest-attribute=format",
    "-Wshadow",
    "-Wswitch-bool",
    "-Wundef",
    // TODO: Fix existing warnings before enabling these:
    // "-Wdouble-promotion",
    // "-Wcast-align",
    // "-Wconversion",
    // "-Wmissing-variable-declarations",
    "-Wextra-semi",
    // "-Wsuggest-attribute=const",
    // "-Wsuggest-attribute=pure",
    // "-Wunreachable-code-aggresive",
    "-Wno-unused-parameter",
    "-Wno-implicit-fallthrough",
    "-fno-exceptions",
    "-fno-rtti",
];

const CXX_FILES: &[&str] = &["xetex/teckit-Engine.cpp", "xetex/xetex-XeTeXOTMath.cpp"];
