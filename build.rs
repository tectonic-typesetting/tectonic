// build.rs -- build helper script for Tectonic.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.
//
// TODO: this surely needs to become much smarter and more flexible.

extern crate gcc;
extern crate pkg_config;

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

    // Actually I'm not 100% sure that I can't compile the C and C++ code
    // into one library, but who cares?

    let mut ccfg = gcc::Config::new();
    let mut cppcfg = gcc::Config::new();
    let mut dpxcfg = gcc::Config::new();

    ccfg.file("tectonic/bmpimage.c")
        .file("tectonic/engine-interface.c")
        .file("tectonic/errors.c")
        .file("tectonic/inimisc.c")
        .file("tectonic/io.c")
        .file("tectonic/jpegimage.c")
        .file("tectonic/mathutil.c")
        .file("tectonic/output.c")
        .file("tectonic/pngimage.c")
        .file("tectonic/synctex.c")
        .file("tectonic/texmfmp.c")
        .file("tectonic/tidy_kpathutil.c")
        .file("tectonic/xetex0.c")
        .file("tectonic/XeTeX_ext.c")
        .file("tectonic/xetexini.c")
        .file("tectonic/XeTeX_pic.c")
        .file("tectonic/xetex-pool.c")
        .include(".");

    cppcfg
        .cpp(true)
        .file("tectonic/Engine.cpp")
        .file("tectonic/hz.cpp")
        .file("tectonic/pdfimage.cpp")
        .file("tectonic/XeTeXFontInst.cpp")
        .file("tectonic/XeTeXFontMgr.cpp")
        .file("tectonic/XeTeXLayoutInterface.cpp")
        .file("tectonic/XeTeXOTMath.cpp")
        .include(".");

    dpxcfg.file("xdvipdfmx/agl.c")
        .file("xdvipdfmx/bmpimage.c")
        .file("xdvipdfmx/cff.c")
        .file("xdvipdfmx/cff_dict.c")
        .file("xdvipdfmx/cid.c")
        .file("xdvipdfmx/cidtype0.c")
        .file("xdvipdfmx/cidtype2.c")
        .file("xdvipdfmx/cmap.c")
        .file("xdvipdfmx/cmap_read.c")
        .file("xdvipdfmx/cmap_write.c")
        .file("xdvipdfmx/cs_type2.c")
        .file("xdvipdfmx/dpxconf.c")
        .file("xdvipdfmx/dpxcrypt.c")
        .file("xdvipdfmx/dpxfile.c")
        .file("xdvipdfmx/dpxutil.c")
        .file("xdvipdfmx/dvi.c")
        .file("xdvipdfmx/dvipdfmx.c")
        .file("xdvipdfmx/epdf.c")
        .file("xdvipdfmx/error.c")
        .file("xdvipdfmx/fontmap.c")
        .file("xdvipdfmx/jp2image.c")
        .file("xdvipdfmx/jpegimage.c")
        .file("xdvipdfmx/mem.c")
        .file("xdvipdfmx/mfileio.c")
        .file("xdvipdfmx/mpost.c")
        .file("xdvipdfmx/numbers.c")
        .file("xdvipdfmx/otl_conf.c")
        .file("xdvipdfmx/otl_opt.c")
        .file("xdvipdfmx/pdfcolor.c")
        .file("xdvipdfmx/pdfdev.c")
        .file("xdvipdfmx/pdfdoc.c")
        .file("xdvipdfmx/pdfdraw.c")
        .file("xdvipdfmx/pdfencoding.c")
        .file("xdvipdfmx/pdfencrypt.c")
        .file("xdvipdfmx/pdffont.c")
        .file("xdvipdfmx/pdfnames.c")
        .file("xdvipdfmx/pdfobj.c")
        .file("xdvipdfmx/pdfparse.c")
        .file("xdvipdfmx/pdfresource.c")
        .file("xdvipdfmx/pdfximage.c")
        .file("xdvipdfmx/pkfont.c")
        .file("xdvipdfmx/pngimage.c")
        .file("xdvipdfmx/pst.c")
        .file("xdvipdfmx/pst_obj.c")
        .file("xdvipdfmx/sfnt.c")
        .file("xdvipdfmx/spc_color.c")
        .file("xdvipdfmx/spc_dvipdfmx.c")
        .file("xdvipdfmx/spc_dvips.c")
        .file("xdvipdfmx/spc_html.c")
        .file("xdvipdfmx/spc_misc.c")
        .file("xdvipdfmx/spc_pdfm.c")
        .file("xdvipdfmx/spc_tpic.c")
        .file("xdvipdfmx/spc_util.c")
        .file("xdvipdfmx/spc_xtx.c")
        .file("xdvipdfmx/specials.c")
        .file("xdvipdfmx/subfont.c")
        .file("xdvipdfmx/t1_char.c")
        .file("xdvipdfmx/t1_load.c")
        .file("xdvipdfmx/tfm.c")
        .file("xdvipdfmx/truetype.c")
        .file("xdvipdfmx/tt_aux.c")
        .file("xdvipdfmx/tt_cmap.c")
        .file("xdvipdfmx/tt_glyf.c")
        .file("xdvipdfmx/tt_gsub.c")
        .file("xdvipdfmx/tt_post.c")
        .file("xdvipdfmx/tt_table.c")
        .file("xdvipdfmx/type0.c")
        .file("xdvipdfmx/type1.c")
        .file("xdvipdfmx/type1c.c")
        .file("xdvipdfmx/unicode.c")
        .file("xdvipdfmx/vf.c")
        .file("xdvipdfmx/xbb.c")
        .include("xdvipdfmx");

    for p in deps.include_paths {
        ccfg.include(&p);
        cppcfg.include(&p);
        dpxcfg.include(&p);
    }

    c_platform_specifics(&mut ccfg);
    cpp_platform_specifics(&mut cppcfg);

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");
    dpxcfg.compile("libtectonic_dvipdfmx.a");
}
