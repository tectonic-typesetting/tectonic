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

    ccfg.file("tectonic/bmpimage.c")
        .file("tectonic/engine-interface.c")
        .file("tectonic/inimisc.c")
        .file("tectonic/jpegimage.c")
        .file("tectonic/openclose.c")
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

    for p in deps.include_paths {
        ccfg.include(&p);
        cppcfg.include(&p);
    }

    c_platform_specifics(&mut ccfg);
    cpp_platform_specifics(&mut cppcfg);

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");
}
