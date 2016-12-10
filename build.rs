// build.rs -- build helper script for Tectonic.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.
//
// TODO: this surely needs to become much smarter and more flexible.

extern crate gcc;
extern crate pkg_config;

const LIBS: &'static str = "fontconfig harfbuzz harfbuzz-icu icu-uc freetype2 graphite2 libpng poppler zlib";

fn main() {
    let deps = pkg_config::probe_library(LIBS).unwrap();

    let mut ccfg = gcc::Config::new();
    let mut cppcfg = gcc::Config::new();

    ccfg.file("tectonic/bmpimage.c")
        .file("tectonic/engine-interface.c")
        .file("tectonic/inimisc.c")
        .file("tectonic/jpegimage.c")
        .file("tectonic/md5.c")
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
        .define("__SyncTeX__", None)
        .include("tectonic")
        .include(".");

    cppcfg
        .cpp(true)
        .file("tectonic/Engine.cpp")
        .file("tectonic/hz.cpp")
        .file("tectonic/pdfimage.cpp")
        .file("tectonic/XeTeXFontInst.cpp")
        .file("tectonic/XeTeXFontMgr.cpp")
        .file("tectonic/XeTeXFontMgr_FC.cpp")
        .file("tectonic/XeTeXLayoutInterface.cpp")
        .file("tectonic/XeTeXOTMath.cpp")
        .define("__SyncTeX__", None)
        .include("tectonic")
        .include(".");

    for p in deps.include_paths {
        ccfg.include(&p);
        cppcfg.include(&p);
    }

    ccfg.compile("libctectonic.a");
    cppcfg.compile("libcpptectonic.a");
}
