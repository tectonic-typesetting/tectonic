// build.rs -- build helper script for Tectonic.
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.
//
// TODO: this surely needs to become much smarter and more flexible.

extern crate cc;
extern crate pkg_config;
extern crate regex;
extern crate sha2;

use std::env;
use std::path::PathBuf;


// No fontconfig on MacOS:
#[cfg(target_os = "macos")]
const LIBS: &'static str = "harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

#[cfg(not(target_os = "macos"))]
const LIBS: &'static str = "fontconfig harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

// DLL linking on Windows is a huge hassle, so try to do everything statically:
#[cfg(target_os = "windows")]
const STATIK: bool = true;

#[cfg(not(target_os = "windows"))]
const STATIK: bool = false;


fn main() {
    // We (have to) rerun the search again below to emit the metadata at the right time.

    let deps = pkg_config::Config::new().cargo_metadata(false).statik(STATIK).probe(LIBS).unwrap();

    // First, emit the string pool C code. Sigh.

    let out_dir = env::var("OUT_DIR").unwrap();

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
        "-std=gnu11"
    ];

    for flag in &cflags {
        ccfg.flag_if_supported(flag);
    }

    ccfg
        .file("tectonic/bibtex.c")
        .file("tectonic/core-bridge.c")
        .file("tectonic/core-kpathutil.c")
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
        .file("tectonic/engine-interface.c")
        .file("tectonic/errors.c")
        .file("tectonic/io.c")
        .file("tectonic/mathutil.c")
        .file("tectonic/output.c")
        .file("tectonic/stringpool.c")
        .file("tectonic/synctex.c")
        .file("tectonic/texmfmp.c")
        .file("tectonic/xetex0.c")
        .file("tectonic/XeTeX_ext.c")
        .file("tectonic/xetexini.c")
        .file("tectonic/XeTeX_pic.c")
        .file("tectonic/xetex-linebreak.c")
        .file("tectonic/xetex-shipout.c")
        .define("HAVE_ZLIB", "1")
        .define("HAVE_ZLIB_COMPRESS2", "1")
        .define("ZLIB_CONST", "1")
        .include(".")
        .include(&out_dir);

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
        // "-Wextra-semi",
        // "-Wmissing-variable-declarations",
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
        .file("tectonic/Engine.cpp")
        .file("tectonic/XeTeXFontInst.cpp")
        .file("tectonic/XeTeXFontMgr.cpp")
        .file("tectonic/XeTeXLayoutInterface.cpp")
        .file("tectonic/XeTeXOTMath.cpp")
        .include(".")
        .include(&out_dir);

    for p in deps.include_paths {
        ccfg.include(&p);
        cppcfg.include(&p);
    }

    // Platform-specific adjustments:

    if cfg!(target_os = "macos") {
        ccfg.define("XETEX_MAC", Some("1"));
        ccfg.file("tectonic/XeTeX_mac.c");

        cppcfg.define("XETEX_MAC", Some("1"));
        cppcfg.file("tectonic/XeTeXFontInst_Mac.cpp");
        cppcfg.file("tectonic/XeTeXFontMgr_Mac.mm");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    if cfg!(not(target_os = "macos")) {
        // At the moment we use Fontconfig on both Linux and Windows.
        cppcfg.file("tectonic/XeTeXFontMgr_FC.cpp");
    }

    if cfg!(target_endian = "big") {
        ccfg.define("WORDS_BIGENDIAN", "1");
        cppcfg.define("WORDS_BIGENDIAN", "1");
    }

    // OK, back to generic build rules.

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");

    // Now that we've emitted the info for our own libraries, we can emit the
    // info for their dependents.

    pkg_config::Config::new().cargo_metadata(true).statik(STATIK).probe(LIBS).unwrap();

    // Tell cargo to rerun build.rs only if files in the tectonic/ directory have changed.
    for file in PathBuf::from("tectonic").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
