// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Build script for the PDF+XDV+graphics I/O subsystem.

use std::{env, path::PathBuf};
use tectonic_cfg_support::*;
use tectonic_dep_support::{Configuration, Dependency, Spec};

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
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    // Dependencies.

    let dep_cfg = Configuration::default();
    let dep = Dependency::probe(LibpngSpec, &dep_cfg);

    // Include paths and settings exported by our internal dependencies.

    let core_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();
    let flate_include_dir = env::var("DEP_TECTONIC_BRIDGE_FLATE_INCLUDE").unwrap();

    // Define the C support library.

    let mut ccfg = cc::Build::new();

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

    for flag in &cflags[..] {
        ccfg.flag_if_supported(flag);
    }

    fn compile(cfg: &mut cc::Build, s: &str) {
        cfg.file(s);
        println!("cargo:rerun-if-changed={}", s);
    }

    ccfg.include("pdf_io")
        .include(&flate_include_dir)
        .include(&core_include_dir);

    dep.foreach_include_path(|p| {
        ccfg.include(p);
    });

    let is_big_endian = target_cfg!(target_endian = "big");
    if is_big_endian {
        ccfg.define("WORDS_BIGENDIAN", "1");
    }

    let files = [
        "pdf_io/dpx-agl.c",
        "pdf_io/dpx-bmpimage.c",
        "pdf_io/dpx-cff.c",
        "pdf_io/dpx-cff_dict.c",
        "pdf_io/dpx-cid.c",
        "pdf_io/dpx-cidtype0.c",
        "pdf_io/dpx-cidtype2.c",
        "pdf_io/dpx-cmap.c",
        "pdf_io/dpx-cmap_read.c",
        "pdf_io/dpx-cmap_write.c",
        "pdf_io/dpx-cs_type2.c",
        "pdf_io/dpx-dpxconf.c",
        "pdf_io/dpx-dpxcrypt.c",
        "pdf_io/dpx-dpxfile.c",
        "pdf_io/dpx-dpxutil.c",
        "pdf_io/dpx-dvi.c",
        "pdf_io/dpx-dvipdfmx.c",
        "pdf_io/dpx-epdf.c",
        "pdf_io/dpx-error.c",
        "pdf_io/dpx-fontmap.c",
        "pdf_io/dpx-jp2image.c",
        "pdf_io/dpx-jpegimage.c",
        "pdf_io/dpx-mem.c",
        "pdf_io/dpx-mfileio.c",
        "pdf_io/dpx-mpost.c",
        "pdf_io/dpx-mt19937ar.c",
        "pdf_io/dpx-numbers.c",
        "pdf_io/dpx-otl_opt.c",
        "pdf_io/dpx-pdfcolor.c",
        "pdf_io/dpx-pdfdev.c",
        "pdf_io/dpx-pdfdoc.c",
        "pdf_io/dpx-pdfdraw.c",
        "pdf_io/dpx-pdfencoding.c",
        "pdf_io/dpx-pdfencrypt.c",
        "pdf_io/dpx-pdffont.c",
        "pdf_io/dpx-pdfnames.c",
        "pdf_io/dpx-pdfobj.c",
        "pdf_io/dpx-pdfparse.c",
        "pdf_io/dpx-pdfresource.c",
        "pdf_io/dpx-pdfximage.c",
        "pdf_io/dpx-pkfont.c",
        "pdf_io/dpx-pngimage.c",
        "pdf_io/dpx-pst.c",
        "pdf_io/dpx-pst_obj.c",
        "pdf_io/dpx-sfnt.c",
        "pdf_io/dpx-spc_color.c",
        "pdf_io/dpx-spc_dvipdfmx.c",
        "pdf_io/dpx-spc_dvips.c",
        "pdf_io/dpx-spc_html.c",
        "pdf_io/dpx-spc_misc.c",
        "pdf_io/dpx-spc_pdfm.c",
        "pdf_io/dpx-spc_tpic.c",
        "pdf_io/dpx-spc_util.c",
        "pdf_io/dpx-spc_xtx.c",
        "pdf_io/dpx-specials.c",
        "pdf_io/dpx-subfont.c",
        "pdf_io/dpx-t1_char.c",
        "pdf_io/dpx-t1_load.c",
        "pdf_io/dpx-tfm.c",
        "pdf_io/dpx-truetype.c",
        "pdf_io/dpx-tt_aux.c",
        "pdf_io/dpx-tt_cmap.c",
        "pdf_io/dpx-tt_glyf.c",
        "pdf_io/dpx-tt_gsub.c",
        "pdf_io/dpx-tt_post.c",
        "pdf_io/dpx-tt_table.c",
        "pdf_io/dpx-type0.c",
        "pdf_io/dpx-type1.c",
        "pdf_io/dpx-type1c.c",
        "pdf_io/dpx-unicode.c",
        "pdf_io/dpx-vf.c",
    ];

    for fname in &files[..] {
        compile(&mut ccfg, fname);
    }

    ccfg.compile("libtectonic_pdf_io.a");

    dep.emit();

    // Cargo exposes this as the environment variable DEP_XXX_INCLUDE_PATH,
    // where XXX is the "links" setting in Cargo.toml. This is the key element
    // that allows us to have a network of crates containing both C/C++ and Rust
    // code that all interlink.
    //
    // At the moment, we're not doing any codegen, so don't need to copy headers:

    let mut main_header_src = manifest_dir;
    main_header_src.push("pdf_io");
    println!("cargo:include-path={}", main_header_src.display());
}
