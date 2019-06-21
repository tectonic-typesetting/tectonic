// build.rs -- build helper script for Tectonic.
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.
//
// TODO: this surely needs to become much smarter and more flexible.

use cc;
use pkg_config;
use vcpkg;

use std::env;
use std::path::PathBuf;

// No fontconfig on MacOS:
#[cfg(target_os = "macos")]
const LIBS: &'static str = "harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

#[cfg(target_os = "macos")]
const VCPKG_LIBS: &[&'static str] = &["harfbuzz", "freetype", "graphite2"];

#[cfg(not(target_os = "macos"))]
const LIBS: &'static str =
    "fontconfig harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

#[cfg(not(target_os = "macos"))]
const VCPKG_LIBS: &[&'static str] = &["fontconfig", "harfbuzz", "freetype", "graphite2"];
// Need a way to check that the vcpkg harfbuzz port has graphite2 and icu options enabled.

fn load_vcpkg_deps(include_paths: &mut Vec<PathBuf>) {
    for dep in VCPKG_LIBS {
        let library = vcpkg::find_package(dep).expect("failed to load package from vcpkg");
        include_paths.extend(library.include_paths.iter().cloned());
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let rustflags = env::var("RUSTFLAGS").unwrap_or(String::new());

    let use_vcpkg = env::var("TECTONIC_VCPKG").is_ok();
    let mut deps = None;
    let mut vcpkg_includes = vec![];
    if use_vcpkg {
        load_vcpkg_deps(&mut vcpkg_includes);
        eprintln!("{:?}", vcpkg_includes);

        if target.contains("-linux-") {
            // add icudata to the end of the list of libs as vcpkg-rs does not
            // order individual libraries as a single pass linker requires
            println!("cargo:rustc-link-lib=icudata");
        }
    } else {
        // We (have to) rerun the search again below to emit the metadata at the right time.
        let deps_library = pkg_config::Config::new()
            .cargo_metadata(false)
            .probe(LIBS)
            .unwrap();
        deps = Some(deps_library);
    }

    // Actually I'm not 100% sure that I can't compile the C and C++ code
    // into one library, but who cares?

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
        .define("HAVE_ZLIB", "1")
        .define("HAVE_ZLIB_COMPRESS2", "1")
        .define("ZLIB_CONST", "1")
        .include(".");

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
    ];

    for flag in &cppflags {
        cppcfg.flag_if_supported(flag);
    }

    cppcfg
        .cpp(true)
        .flag("-Wall")
        .file("tectonic/teckit-Engine.cpp")
        .file("tectonic/xetex-XeTeXFontInst.cpp")
        .file("tectonic/xetex-XeTeXFontMgr.cpp")
        .file("tectonic/xetex-XeTeXLayoutInterface.cpp")
        .file("tectonic/xetex-XeTeXOTMath.cpp")
        .include(".");

    if let Some(deps) = &deps {
        for p in &deps.include_paths {
            ccfg.include(p);
            cppcfg.include(p);
        }
    } else {
        for p in &vcpkg_includes {
            ccfg.include(p);
            cppcfg.include(p);
        }
    }

    // Platform-specific adjustments:

    if cfg!(target_os = "macos") {
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

    if cfg!(not(target_os = "macos")) {
        // At the moment we use Fontconfig on both Linux and Windows.
        cppcfg.file("tectonic/xetex-XeTeXFontMgr_FC.cpp");
    }

    if cfg!(target_endian = "big") {
        ccfg.define("WORDS_BIGENDIAN", "1");
        cppcfg.define("WORDS_BIGENDIAN", "1");
    }

    if target.contains("-msvc") {
        ccfg.flag("/EHsc");
        cppcfg.flag("/EHsc");
        if rustflags.contains("+crt-static") {
            ccfg.define("GRAPHITE2_STATIC", None);
            cppcfg.define("GRAPHITE2_STATIC", None);
        }
    }

    // OK, back to generic build rules.

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");

    // Now that we've emitted the info for our own libraries, we can emit the
    // info for their dependents.

    if let Some(_deps) = &deps {
        pkg_config::Config::new()
            .cargo_metadata(true)
            .probe(LIBS)
            .unwrap();
    }

    // Tell cargo to rerun build.rs only if files in the tectonic/ directory have changed.
    for file in PathBuf::from("tectonic").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
