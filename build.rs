// build.rs -- build helper script for Tectonic.
// Copyright 2016-2019 the Tectonic Project
// Licensed under the MIT License.

/// The Tectonic build script. Not only do we have internal C/C++ code, we
/// also depend on several external C/C++ libraries, so there's a lot to do
/// here. It would be great to streamline things.
///
/// TODO: this surely needs to become much smarter and more flexible.
use cc;
use pkg_config;
use tectonic_cfg_support::*;
use vcpkg;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "macos"))]
const PKGCONFIG_LIBS: &str =
    "fontconfig harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

// No fontconfig on MacOS:
#[cfg(target_os = "macos")]
const PKGCONFIG_LIBS: &str = "harfbuzz >= 1.4 harfbuzz-icu icu-uc freetype2 graphite2 libpng zlib";

/// Build-script state when using pkg-config as the backend.
#[derive(Debug)]
struct PkgConfigState {
    libs: pkg_config::Library,
}

// Need a way to check that the vcpkg harfbuzz port has graphite2 and icu options enabled.
#[cfg(not(target_os = "macos"))]
const VCPKG_LIBS: &[&str] = &["fontconfig", "harfbuzz", "freetype", "graphite2"];

#[cfg(target_os = "macos")]
const VCPKG_LIBS: &[&str] = &["harfbuzz", "freetype", "graphite2"];

/// Build-script state when using vcpkg as the backend.
#[derive(Clone, Debug)]
struct VcPkgState {
    include_paths: Vec<PathBuf>,
}

/// State for discovering and managing our dependencies, which may vary
/// depending on the framework that we're using to discover them.
///
/// The basic gameplan is that we probe our dependencies to check that they're
/// available and pull out the C/C++ include directories; then we emit info
/// for building our C/C++ libraries; then we emit info for our dependencies.
/// Building stuff pretty much always requires some level of hackery, though,
/// so we don't try to be purist about the details.
#[derive(Debug)]
enum DepState {
    /// pkg-config
    PkgConfig(PkgConfigState),

    /// vcpkg
    VcPkg(VcPkgState),
}

impl DepState {
    /// Probe for our dependent libraries using pkg-config.
    fn new_pkg_config() -> Self {
        let statik = env::var("TECTONIC_PKGCONFIG_FORCE_SEMI_STATIC").is_ok();

        let libs = pkg_config::Config::new()
            .cargo_metadata(false)
            .statik(statik)
            .probe(PKGCONFIG_LIBS)
            .unwrap();
        DepState::PkgConfig(PkgConfigState { libs })
    }

    /// Probe for our dependent libraries using vcpkg.
    fn new_vcpkg() -> Self {
        let mut include_paths = vec![];

        for dep in VCPKG_LIBS {
            let library = vcpkg::find_package(dep)
                .unwrap_or_else(|e| panic!("failed to load package {} from vcpkg: {}", dep, e));
            include_paths.extend(library.include_paths.iter().cloned());
        }

        DepState::VcPkg(VcPkgState { include_paths })
    }

    /// Invoke a callback for each C/C++ include directory injected by our
    /// dependencies.
    fn foreach_include_path<F>(&self, mut f: F)
    where
        F: FnMut(&Path),
    {
        match self {
            DepState::PkgConfig(ref s) => {
                for p in &s.libs.include_paths {
                    f(p);
                }
            }

            DepState::VcPkg(ref s) => {
                for p in &s.include_paths {
                    f(p);
                }
            }
        }
    }

    /// This function is called after we've emitted the cargo compilation info
    /// for our own libraries. Now we can emit any special information
    /// relating to our dependencies, which may depend on the dep-finding
    /// backend or the target.
    fn emit_late_extras(&self, target: &str) {
        match self {
            DepState::PkgConfig(ref state) => {
                if env::var("TECTONIC_PKGCONFIG_FORCE_SEMI_STATIC").is_ok() {
                    // pkg-config will prevent "system libraries" from being
                    // linked statically even when PKG_CONFIG_ALL_STATIC=1,
                    // but its definition of a system library isn't always
                    // perfect. For Debian cross builds, we'd like to make
                    // binaries that are dynamically linked with things like
                    // libc and libm but not libharfbuzz, etc. In this mode we
                    // override pkg-config's logic by emitting the metadata
                    // ourselves.
                    for link_path in &state.libs.link_paths {
                        println!("cargo:rustc-link-search=native={}", link_path.display());
                    }

                    for fw_path in &state.libs.framework_paths {
                        println!("cargo:rustc-link-search=framework={}", fw_path.display());
                    }

                    for libbase in &state.libs.libs {
                        let do_static = match libbase.as_ref() {
                            "c" | "m" | "dl" | "pthread" => false,
                            _ => {
                                // Frustratingly, graphite2 seems to have
                                // issues with static builds; e.g. static
                                // graphite2 is not available on Debian. So
                                // let's jump through the hoops of testing
                                // whether the static archive seems findable.
                                let libname = format!("lib{}.a", libbase);
                                state
                                    .libs
                                    .link_paths
                                    .iter()
                                    .any(|d| d.join(&libname).exists())
                            }
                        };

                        let mode = if do_static { "static=" } else { "" };
                        println!("cargo:rustc-link-lib={}{}", mode, libbase);
                    }

                    for fw in &state.libs.frameworks {
                        println!("cargo:rustc-link-lib=framework={}", fw);
                    }
                } else {
                    // Just let pkg-config do its thing.
                    pkg_config::Config::new()
                        .cargo_metadata(true)
                        .probe(PKGCONFIG_LIBS)
                        .unwrap();
                }
            }

            DepState::VcPkg(_) => {
                if target.contains("-linux-") {
                    // add icudata to the end of the list of libs as vcpkg-rs
                    // does not order individual libraries as a single pass
                    // linker requires.
                    println!("cargo:rustc-link-lib=icudata");
                }
            }
        }
    }
}

/// The default dependency-finding backend is pkg-config.
impl Default for DepState {
    fn default() -> Self {
        DepState::new_pkg_config()
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let rustflags = env::var("RUSTFLAGS").unwrap_or_default();

    // Re-export $TARGET during the build so that our executable tests know
    // what environment variable CARGO_TARGET_@TARGET@_RUNNER to check when
    // they want to spawn off executables.

    println!("cargo:rustc-env=TARGET={}", target);

    // OK, how are we finding our dependencies?

    println!("cargo:rerun-if-env-changed=TECTONIC_DEP_BACKEND");
    println!("cargo:rerun-if-env-changed=TECTONIC_PKGCONFIG_FORCE_SEMI_STATIC");

    let dep_state = if let Ok(dep_backend_str) = env::var("TECTONIC_DEP_BACKEND") {
        match dep_backend_str.as_ref() {
            "pkg-config" => DepState::new_pkg_config(),
            "vcpkg" => DepState::new_vcpkg(),
            "default" => DepState::default(),
            other => panic!("unrecognized TECTONIC_DEP_BACKEND setting {:?}", other),
        }
    } else {
        DepState::default()
    };

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
        .include(".");

    dep_state.foreach_include_path(|p| {
        ccfg.include(p);
        cppcfg.include(p);
    });

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
        if rustflags.contains("+crt-static") {
            ccfg.define("GRAPHITE2_STATIC", None);
            cppcfg.define("GRAPHITE2_STATIC", None);
        }
    }

    // OK, back to generic build rules.

    ccfg.compile("libtectonic_c.a");
    cppcfg.compile("libtectonic_cpp.a");

    dep_state.emit_late_extras(&target);

    // Tell cargo to rerun build.rs only if files in the tectonic/ directory have changed.
    for file in PathBuf::from("tectonic").read_dir().unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
