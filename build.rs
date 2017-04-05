// build.rs -- build helper script for Tectonic.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.
//
// TODO: this surely needs to become much smarter and more flexible.

extern crate emit_stringpool;
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;


// MacOS platform specifics:

#[cfg(target_os = "macos")]
const LIBS: &'static str = "harfbuzz harfbuzz-icu icu-uc freetype2 graphite2 libpng poppler zlib";

#[cfg(target_os = "macos")]
fn c_platform_specifics(cfg: &mut gcc::Config) {
   cfg.define("XETEX_MAC", Some("1"));
   cfg.file("tectonic/XeTeX_mac.c");

   println!("cargo:rustc-link-lib=framework=Foundation");
   println!("cargo:rustc-link-lib=framework=CoreFoundation");
   println!("cargo:rustc-link-lib=framework=CoreGraphics");
   println!("cargo:rustc-link-lib=framework=CoreText");
   println!("cargo:rustc-link-lib=framework=AppKit");
}

#[cfg(target_os = "macos")]
fn cpp_platform_specifics(cfg: &mut gcc::Config) {
   cfg.define("XETEX_MAC", Some("1"));
   cfg.file("tectonic/XeTeXFontInst_Mac.cpp");
   cfg.file("tectonic/XeTeXFontMgr_Mac.mm");
}


// Not-MacOS:

#[cfg(not(target_os = "macos"))]
const LIBS: &'static str = "fontconfig harfbuzz harfbuzz-icu icu-uc freetype2 graphite2 libpng poppler zlib";

#[cfg(not(target_os = "macos"))]
fn c_platform_specifics(_: &mut gcc::Config) {
}

#[cfg(not(target_os = "macos"))]
fn cpp_platform_specifics(cfg: &mut gcc::Config) {
   cfg.file("tectonic/XeTeXFontMgr_FC.cpp");
}


fn main() {
    let deps = pkg_config::probe_library(LIBS).unwrap();

    // First, emit the string pool C code. Sigh.

    let out_dir = env::var("OUT_DIR").unwrap();

    {
        let listfile = PathBuf::from("tectonic/strings.txt");
        let mut outstem = PathBuf::from(&out_dir);
        outstem.push("stringpool_generated");
        emit_stringpool::emit(&listfile, &outstem).expect("failed to generate \"string pool\" C source code");
    }

    // Actually I'm not 100% sure that I can't compile the C and C++ code
    // into one library, but who cares?

    let mut ccfg = gcc::Config::new();
    let mut cppcfg = gcc::Config::new();

    ccfg.file("tectonic/bibtex.c")
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
        .file("tectonic/dpx-xfseeko.c")
        .file("tectonic/dpx-xftello.c")
        .file("tectonic/engine-interface.c")
        .file("tectonic/errors.c")
        .file("tectonic/inimisc.c")
        .file("tectonic/io.c")
        .file("tectonic/mathutil.c")
        .file("tectonic/output.c")
        .file("tectonic/stringpool.c")
        .file("tectonic/synctex.c")
        .file("tectonic/texmfmp.c")
        .file("tectonic/tidy_kpathutil.c")
        .file("tectonic/xetex0.c")
        .file("tectonic/XeTeX_ext.c")
        .file("tectonic/xetexini.c")
        .file("tectonic/XeTeX_pic.c")
        .define("HAVE_GETENV", Some("1"))
        .define("HAVE_LIBPNG", Some("1"))
        .define("HAVE_MKSTEMP", Some("1"))
        .define("HAVE_STDINT_H", Some("1"))
        .define("HAVE_SYS_TYPES_H", Some("1"))
        .define("HAVE_SYS_WAIT_H", Some("1"))
        .define("HAVE_TM_GMTOFF", Some("1"))
        .define("HAVE_ZLIB", Some("1"))
        .define("HAVE_ZLIB_COMPRESS2", Some("1"))
        .include(".")
        .include(&out_dir);

    cppcfg
        .cpp(true)
        .file("tectonic/Engine.cpp")
        .file("tectonic/hz.cpp")
        .file("tectonic/pdfimage.cpp")
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

    c_platform_specifics(&mut ccfg);
    cpp_platform_specifics(&mut cppcfg);

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");
}
