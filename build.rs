// build.rs -- build helper script for Tectonic.
// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! The Tectonic build script. Not only do we have internal C/C++ code, we
//! also depend on several external C/C++ libraries, so there's a lot to do
//! here. It would be great to streamline things.

use std::{env, path::PathBuf};
use tectonic_cfg_support::*;
use tectonic_dep_support::{Backend, Configuration, Dependency, Spec};

struct TectonicRestSpec;

impl Spec for TectonicRestSpec {
    #[cfg(not(target_os = "macos"))]
    fn get_pkgconfig_spec(&self) -> &str {
        "fontconfig libpng"
    }

    // No fontconfig on macOS.
    #[cfg(target_os = "macos")]
    fn get_pkgconfig_spec(&self) -> &str {
        "libpng"
    }

    // Would be nice to have a way to check that the vcpkg harfbuzz port has
    // graphite2 and icu options enabled.
    #[cfg(not(target_os = "macos"))]
    fn get_vcpkg_spec(&self) -> &[&str] {
        &["fontconfig"]
    }

    #[cfg(target_os = "macos")]
    fn get_vcpkg_spec(&self) -> &[&str] {
        &[]
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();

    // Generate bindings for the C/C++ code to interface with backend Rust code.
    // As a heuristic we trigger rebuilds on changes to src/engines/mod.rs since
    // most of `core-bindgen.h` comes from this file.

    let mut cbindgen_header_path: PathBuf = env::var("OUT_DIR").unwrap().into();
    cbindgen_header_path.push("core-bindgen.h");

    cbindgen::generate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .unwrap()
        .write_to_file(&cbindgen_header_path);

    println!("cargo:rerun-if-changed=src/engines/mod.rs");

    // Re-export $TARGET during the build so that our executable tests know
    // what environment variable CARGO_TARGET_@TARGET@_RUNNER to check when
    // they want to spawn off executables.

    println!("cargo:rustc-env=TARGET={}", target);

    // Find our dependencies that aren't provided by any bridge or -sys crates.

    let dep_cfg = Configuration::default();
    let dep = Dependency::probe(TectonicRestSpec, &dep_cfg);

    // Include paths exported by our internal dependencies.

    let flate_include_dir = env::var("DEP_TECTONIC_BRIDGE_FLATE_INCLUDE").unwrap();
    let freetype2_include_dir = env::var("DEP_FREETYPE2_INCLUDE").unwrap();
    let graphite2_include_dir = env::var("DEP_GRAPHITE2_INCLUDE").unwrap();
    let graphite2_static = !env::var("DEP_GRAPHITE2_DEFINE_STATIC").unwrap().is_empty();
    let harfbuzz_include_dir = env::var("DEP_HARFBUZZ_INCLUDE").unwrap();
    let icu_include_dir = env::var("DEP_ICUUC_INCLUDE").unwrap();

    // Specify the C/C++ support libraries. Actually I'm not 100% sure that I
    // can't compile the C and C++ code into one library, but it's no a big deal
    // -- it all gets linked together in the end.

    let mut ccfg = cc::Build::new();
    let mut cppcfg = cc::Build::new();
    let cflags = [
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

    for flag in &cflags {
        ccfg.flag_if_supported(flag);
    }

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

    profile_config(&mut ccfg);

    ccfg.file("tectonic/bibtex.c")
        .file("tectonic/core-bridge.c")
        .file("tectonic/core-memory.c")
        .file("tectonic/dpx-agl.c")
        .file("tectonic/dpx-bmpimage.c")
        .file("tectonic/dpx-cff.c")
        .file("tectonic/dpx-cff_dict.c")
        .file("tectonic/dpx-cid.c")
        .file("tectonic/dpx-cidtype0.c")
        .file("tectonic/dpx-cidtype2.c")
        .file("tectonic/dpx-cmap.c")
        .file("tectonic/dpx-cmap_read.c")
        .file("tectonic/dpx-cmap_write.c")
        .file("tectonic/dpx-cs_type2.c")
        .file("tectonic/dpx-dpxconf.c")
        .file("tectonic/dpx-dpxcrypt.c")
        .file("tectonic/dpx-dpxfile.c")
        .file("tectonic/dpx-dpxutil.c")
        .file("tectonic/dpx-dvi.c")
        .file("tectonic/dpx-dvipdfmx.c")
        .file("tectonic/dpx-epdf.c")
        .file("tectonic/dpx-error.c")
        .file("tectonic/dpx-fontmap.c")
        .file("tectonic/dpx-jp2image.c")
        .file("tectonic/dpx-jpegimage.c")
        .file("tectonic/dpx-mem.c")
        .file("tectonic/dpx-mfileio.c")
        .file("tectonic/dpx-mpost.c")
        .file("tectonic/dpx-mt19937ar.c")
        .file("tectonic/dpx-numbers.c")
        .file("tectonic/dpx-otl_conf.c")
        .file("tectonic/dpx-otl_opt.c")
        .file("tectonic/dpx-pdfcolor.c")
        .file("tectonic/dpx-pdfdev.c")
        .file("tectonic/dpx-pdfdoc.c")
        .file("tectonic/dpx-pdfdraw.c")
        .file("tectonic/dpx-pdfencoding.c")
        .file("tectonic/dpx-pdfencrypt.c")
        .file("tectonic/dpx-pdffont.c")
        .file("tectonic/dpx-pdfnames.c")
        .file("tectonic/dpx-pdfobj.c")
        .file("tectonic/dpx-pdfparse.c")
        .file("tectonic/dpx-pdfresource.c")
        .file("tectonic/dpx-pdfximage.c")
        .file("tectonic/dpx-pkfont.c")
        .file("tectonic/dpx-pngimage.c")
        .file("tectonic/dpx-pst.c")
        .file("tectonic/dpx-pst_obj.c")
        .file("tectonic/dpx-sfnt.c")
        .file("tectonic/dpx-spc_color.c")
        .file("tectonic/dpx-spc_dvipdfmx.c")
        .file("tectonic/dpx-spc_dvips.c")
        .file("tectonic/dpx-spc_html.c")
        .file("tectonic/dpx-spc_misc.c")
        .file("tectonic/dpx-spc_pdfm.c")
        .file("tectonic/dpx-spc_tpic.c")
        .file("tectonic/dpx-spc_util.c")
        .file("tectonic/dpx-spc_xtx.c")
        .file("tectonic/dpx-specials.c")
        .file("tectonic/dpx-subfont.c")
        .file("tectonic/dpx-t1_char.c")
        .file("tectonic/dpx-t1_load.c")
        .file("tectonic/dpx-tfm.c")
        .file("tectonic/dpx-truetype.c")
        .file("tectonic/dpx-tt_aux.c")
        .file("tectonic/dpx-tt_cmap.c")
        .file("tectonic/dpx-tt_glyf.c")
        .file("tectonic/dpx-tt_gsub.c")
        .file("tectonic/dpx-tt_post.c")
        .file("tectonic/dpx-tt_table.c")
        .file("tectonic/dpx-type0.c")
        .file("tectonic/dpx-type1.c")
        .file("tectonic/dpx-type1c.c")
        .file("tectonic/dpx-unicode.c")
        .file("tectonic/dpx-vf.c")
        .file("tectonic/xetex-engine-interface.c")
        .file("tectonic/xetex-errors.c")
        .file("tectonic/xetex-ext.c")
        .file("tectonic/xetex-ini.c")
        .file("tectonic/xetex-io.c")
        .file("tectonic/xetex-linebreak.c")
        .file("tectonic/xetex-math.c")
        .file("tectonic/xetex-output.c")
        .file("tectonic/xetex-pagebuilder.c")
        .file("tectonic/xetex-pic.c")
        .file("tectonic/xetex-scaledmath.c")
        .file("tectonic/xetex-shipout.c")
        .file("tectonic/xetex-stringpool.c")
        .file("tectonic/xetex-synctex.c")
        .file("tectonic/xetex-texmfmp.c")
        .file("tectonic/xetex-xetex0.c")
        .include(env::var("OUT_DIR").unwrap())
        .include(".")
        .include(&harfbuzz_include_dir)
        .include(&freetype2_include_dir)
        .include(&graphite2_include_dir)
        .include(&icu_include_dir)
        .include(&flate_include_dir);

    let cppflags = [
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

    for flag in &cppflags {
        cppcfg.flag_if_supported(flag);
    }

    profile_config(&mut cppcfg);

    cppcfg
        .cpp(true)
        .flag("-Wall")
        .file("tectonic/teckit-Engine.cpp")
        .file("tectonic/xetex-XeTeXFontInst.cpp")
        .file("tectonic/xetex-XeTeXFontMgr.cpp")
        .file("tectonic/xetex-XeTeXLayoutInterface.cpp")
        .file("tectonic/xetex-XeTeXOTMath.cpp")
        .include(env::var("OUT_DIR").unwrap())
        .include(".")
        .include(&harfbuzz_include_dir)
        .include(&freetype2_include_dir)
        .include(&graphite2_include_dir)
        .include(&icu_include_dir)
        .include(&flate_include_dir);

    dep.foreach_include_path(|p| {
        ccfg.include(p);
        cppcfg.include(p);
    });

    if graphite2_static {
        ccfg.define("GRAPHITE2_STATIC", "1");
        cppcfg.define("GRAPHITE2_STATIC", "1");
    }

    // Platform-specific adjustments:

    let is_mac_os = target_cfg!(target_os = "macos");

    if is_mac_os {
        ccfg.define("XETEX_MAC", Some("1"));
        ccfg.file("tectonic/xetex-macos.c");

        cppcfg.define("XETEX_MAC", Some("1"));
        cppcfg.file("tectonic/xetex-XeTeXFontInst_Mac.cpp");
        cppcfg.file("tectonic/xetex-XeTeXFontMgr_Mac.mm");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    if !is_mac_os {
        // At the moment we use Fontconfig on both Linux and Windows.
        cppcfg.file("tectonic/xetex-XeTeXFontMgr_FC.cpp");
    }

    let is_big_endian = target_cfg!(target_endian = "big");
    if is_big_endian {
        ccfg.define("WORDS_BIGENDIAN", "1");
        cppcfg.define("WORDS_BIGENDIAN", "1");
    }

    if target.contains("-msvc") {
        ccfg.flag("/EHsc");
        cppcfg.flag("/EHsc");
    }

    // OK, back to generic build rules.

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");

    dep.emit();

    // vcpkg-rs is not guaranteed to emit libraries in the order required by a
    // single-pass linker, so we might need to make sure that's done right.

    if dep_cfg.backend == Backend::Vcpkg && target.contains("-linux-") {
        // add icudata to the end of the list of libs as vcpkg-rs
        // does not order individual libraries as a single pass
        // linker requires.
        println!("cargo:rustc-link-lib=icudata");
    }

    // Tell cargo to rerun build.rs only if files in the tectonic/ directory have changed.
    for file in PathBuf::from("tectonic").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
