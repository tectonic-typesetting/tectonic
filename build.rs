// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! The Tectonic build script. Not only do we have internal C/C++ code, we
//! also depend on several external C/C++ libraries, so there's a lot to do
//! here. It would be great to streamline things.

use std::{env, path::PathBuf};
use tectonic_cfg_support::*;
use tectonic_dep_support::{Backend, Configuration, Dependency, Spec};

struct LibpngSpec;

impl Spec for LibpngSpec {
    fn get_pkgconfig_spec(&self) -> &str {
        "libpng"
    }

    fn get_vcpkg_spec(&self) -> &[&str] {
        &["libpng"]
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // Generate bindings for the C/C++ code to interface with backend Rust code.
    // As a heuristic we trigger rebuilds on changes to src/engines/mod.rs since
    // most of `core-bindgen.h` comes from this file.
    //
    // With the move to sub-crates, we can now (2021 Jan) almost get rid of this
    // bindgen layer, but not quite yet.

    let mut cbindgen_header_path: PathBuf = out_dir.clone().into();
    cbindgen_header_path.push("core-bindgen.h");

    cbindgen::generate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .unwrap()
        .write_to_file(&cbindgen_header_path);

    println!("cargo:rerun-if-changed=src/engines/mod.rs");

    // Re-export $TARGET during the build so that our executable tests know
    // what environment variable CARGO_TARGET_@TARGET@_RUNNER to check when
    // they want to spawn off executables.

    println!("cargo:rustc-env=TARGET={}", target);

    // Dependencies.

    let dep_cfg = Configuration::default();
    let dep = Dependency::probe(LibpngSpec, &dep_cfg);

    // Include paths exported by our internal dependencies.

    let xetex_layout_include_path = env::var("DEP_TECTONIC_XETEX_LAYOUT_INCLUDE_PATH").unwrap();
    let core_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();
    let flate_include_dir = env::var("DEP_TECTONIC_BRIDGE_FLATE_INCLUDE").unwrap();
    let graphite2_include_path = env::var("DEP_GRAPHITE2_INCLUDE_PATH").unwrap();
    let graphite2_static = !env::var("DEP_GRAPHITE2_DEFINE_STATIC").unwrap().is_empty();
    let harfbuzz_include_path = env::var("DEP_HARFBUZZ_INCLUDE_PATH").unwrap();
    let icu_include_path = env::var("DEP_ICUUC_INCLUDE_PATH").unwrap();

    // If we want to profile, the default assumption is that we must force the
    // compiler to include frame pointers. We whitelist platforms that are
    // known to be able to profile *without* frame pointers: currently, only
    // Linux/x86_64.

    let profile_target_requires_frame_pointer: bool =
        target_cfg!(not(all(target_os = "linux", target_arch = "x86_64")));

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

    for p in &[&out_dir, ".", &core_include_dir, &flate_include_dir] {
        c_cfg.include(p);
        cxx_cfg.include(p);
    }

    dep.foreach_include_path(|p| {
        c_cfg.include(p);
        cxx_cfg.include(p);
    });

    for item in xetex_layout_include_path.split(';') {
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

    if graphite2_static {
        c_cfg.define("GRAPHITE2_STATIC", "1");
        cxx_cfg.define("GRAPHITE2_STATIC", "1");
    }

    // Platform-specific adjustments:

    let is_mac_os = target_cfg!(target_os = "macos");

    if is_mac_os {
        c_cfg.define("XETEX_MAC", Some("1"));
        c_cfg.file("tectonic/xetex-macos.c");
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

    c_cfg.compile("libtectonic_c.a");
    cxx_cfg.compile("libtectonic_cxx.a");

    dep.emit();

    // vcpkg-rs is not guaranteed to emit libraries in the order required by a
    // single-pass linker, so we might need to make sure that's done right.

    if dep_cfg.backend == Backend::Vcpkg && target.contains("-linux-") {
        // add icudata to the end of the list of libs as vcpkg-rs
        // does not order individual libraries as a single pass
        // linker requires.
        println!("cargo:rustc-link-lib=icudata");
    }

    // Rebuild if C/C++ files have changed. We scan the whole directory to get
    // the headers too.

    for file in PathBuf::from("tectonic").read_dir().unwrap() {
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
    "tectonic/core-bridge.c",
    "tectonic/dpx-agl.c",
    "tectonic/dpx-bmpimage.c",
    "tectonic/dpx-cff.c",
    "tectonic/dpx-cff_dict.c",
    "tectonic/dpx-cid.c",
    "tectonic/dpx-cidtype0.c",
    "tectonic/dpx-cidtype2.c",
    "tectonic/dpx-cmap.c",
    "tectonic/dpx-cmap_read.c",
    "tectonic/dpx-cmap_write.c",
    "tectonic/dpx-cs_type2.c",
    "tectonic/dpx-dpxconf.c",
    "tectonic/dpx-dpxcrypt.c",
    "tectonic/dpx-dpxfile.c",
    "tectonic/dpx-dpxutil.c",
    "tectonic/dpx-dvi.c",
    "tectonic/dpx-dvipdfmx.c",
    "tectonic/dpx-epdf.c",
    "tectonic/dpx-error.c",
    "tectonic/dpx-fontmap.c",
    "tectonic/dpx-jp2image.c",
    "tectonic/dpx-jpegimage.c",
    "tectonic/dpx-mem.c",
    "tectonic/dpx-mfileio.c",
    "tectonic/dpx-mpost.c",
    "tectonic/dpx-mt19937ar.c",
    "tectonic/dpx-numbers.c",
    "tectonic/dpx-otl_conf.c",
    "tectonic/dpx-otl_opt.c",
    "tectonic/dpx-pdfcolor.c",
    "tectonic/dpx-pdfdev.c",
    "tectonic/dpx-pdfdoc.c",
    "tectonic/dpx-pdfdraw.c",
    "tectonic/dpx-pdfencoding.c",
    "tectonic/dpx-pdfencrypt.c",
    "tectonic/dpx-pdffont.c",
    "tectonic/dpx-pdfnames.c",
    "tectonic/dpx-pdfobj.c",
    "tectonic/dpx-pdfparse.c",
    "tectonic/dpx-pdfresource.c",
    "tectonic/dpx-pdfximage.c",
    "tectonic/dpx-pkfont.c",
    "tectonic/dpx-pngimage.c",
    "tectonic/dpx-pst.c",
    "tectonic/dpx-pst_obj.c",
    "tectonic/dpx-sfnt.c",
    "tectonic/dpx-spc_color.c",
    "tectonic/dpx-spc_dvipdfmx.c",
    "tectonic/dpx-spc_dvips.c",
    "tectonic/dpx-spc_html.c",
    "tectonic/dpx-spc_misc.c",
    "tectonic/dpx-spc_pdfm.c",
    "tectonic/dpx-spc_tpic.c",
    "tectonic/dpx-spc_util.c",
    "tectonic/dpx-spc_xtx.c",
    "tectonic/dpx-specials.c",
    "tectonic/dpx-subfont.c",
    "tectonic/dpx-t1_char.c",
    "tectonic/dpx-t1_load.c",
    "tectonic/dpx-tfm.c",
    "tectonic/dpx-truetype.c",
    "tectonic/dpx-tt_aux.c",
    "tectonic/dpx-tt_cmap.c",
    "tectonic/dpx-tt_glyf.c",
    "tectonic/dpx-tt_gsub.c",
    "tectonic/dpx-tt_post.c",
    "tectonic/dpx-tt_table.c",
    "tectonic/dpx-type0.c",
    "tectonic/dpx-type1.c",
    "tectonic/dpx-type1c.c",
    "tectonic/dpx-unicode.c",
    "tectonic/dpx-vf.c",
    "tectonic/xetex-engine-interface.c",
    "tectonic/xetex-errors.c",
    "tectonic/xetex-ext.c",
    "tectonic/xetex-ini.c",
    "tectonic/xetex-io.c",
    "tectonic/xetex-linebreak.c",
    "tectonic/xetex-math.c",
    "tectonic/xetex-output.c",
    "tectonic/xetex-pagebuilder.c",
    "tectonic/xetex-pic.c",
    "tectonic/xetex-scaledmath.c",
    "tectonic/xetex-shipout.c",
    "tectonic/xetex-stringpool.c",
    "tectonic/xetex-synctex.c",
    "tectonic/xetex-texmfmp.c",
    "tectonic/xetex-xetex0.c",
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

const CXX_FILES: &[&str] = &[
    "tectonic/teckit-Engine.cpp",
    "tectonic/xetex-XeTeXOTMath.cpp",
];
