// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Build script for the layout subsystem. Things get a little "fun" since we
//! use different font-finding and layout frameworks depending on the target
//! platform.
//!
//! Specifically, on macOS we use CoreText. On all other platforms, including
//! Windows, we use Fontconfig to discover fonts.

use std::{
    env,
    path::{Path, PathBuf},
};
use tectonic_cfg_support::target_cfg;
use tectonic_dep_support::{Configuration, Dependency, Spec};

struct FontconfigSpec;

impl Spec for FontconfigSpec {
    fn get_pkgconfig_spec(&self) -> &str {
        "fontconfig"
    }

    fn get_vcpkg_spec(&self) -> &[&str] {
        &["fontconfig"]
    }
}

/// Note that we have to decide what to look for at runtime, because we might be
/// cross-compiling, in which case the target configuration settings are exposed
/// dynamically through environment variables.
struct PlatformLayoutDeps<'a> {
    fontconfig: Option<Dependency<'a, FontconfigSpec>>,
}

impl<'a> PlatformLayoutDeps<'a> {
    pub fn new(dep_cfg: &'a Configuration, is_mac_os: bool) -> Self {
        let fontconfig = if is_mac_os {
            None
        } else {
            Some(Dependency::probe(FontconfigSpec, dep_cfg))
        };

        PlatformLayoutDeps { fontconfig }
    }

    pub fn foreach_include_path<F>(&self, f: F)
    where
        F: FnMut(&Path),
    {
        if let Some(ref fc) = self.fontconfig {
            fc.foreach_include_path(f);
        }
    }

    pub fn emit(&self) {
        if let Some(ref fc) = self.fontconfig {
            fc.emit();
        }
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let is_mac_os = target_cfg!(target_os = "macos");

    // Find any necessary deps.

    let dep_cfg = Configuration::default();
    let deps = PlatformLayoutDeps::new(&dep_cfg, is_mac_os);

    // Include paths and settings exported by our internal dependencies.

    let core_include_dir = env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap();
    let freetype2_include_path = env::var("DEP_FREETYPE2_INCLUDE_PATH").unwrap();
    let graphite2_include_path = env::var("DEP_GRAPHITE2_INCLUDE_PATH").unwrap();
    let graphite2_static = !env::var("DEP_GRAPHITE2_DEFINE_STATIC").unwrap().is_empty();
    let harfbuzz_include_path = env::var("DEP_HARFBUZZ_INCLUDE_PATH").unwrap();
    let icu_include_path = env::var("DEP_ICUUC_INCLUDE_PATH").unwrap();

    // Define the C++ support library.

    let mut cppcfg = cc::Build::new();

    let cppflags = [
        "-std=c++17",
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

    fn compile(cfg: &mut cc::Build, s: &str) {
        cfg.file(s);
        println!("cargo:rerun-if-changed={s}");
    }

    cppcfg
        .cpp(true)
        .flag("-Wall")
        .include("layout")
        .include(&core_include_dir);

    deps.foreach_include_path(|p| {
        cppcfg.include(p);
    });

    for item in harfbuzz_include_path.split(';') {
        cppcfg.include(item);
    }

    for item in freetype2_include_path.split(';') {
        cppcfg.include(item);
    }

    for item in graphite2_include_path.split(';') {
        cppcfg.include(item);
    }

    for item in icu_include_path.split(';') {
        cppcfg.include(item);
    }

    compile(&mut cppcfg, "layout/xetex-XeTeXFontInst.cpp");
    compile(&mut cppcfg, "layout/xetex-XeTeXFontMgr.cpp");
    compile(&mut cppcfg, "layout/xetex-XeTeXLayoutInterface.cpp");

    if graphite2_static {
        cppcfg.define("GRAPHITE2_STATIC", "1");
    }

    // Platform-specific adjustments:

    if is_mac_os {
        cppcfg.define("XETEX_MAC", Some("1"));
        compile(&mut cppcfg, "layout/xetex-XeTeXFontInst_Mac.cpp");
        compile(&mut cppcfg, "layout/xetex-XeTeXFontMgr_Mac.mm");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    if !is_mac_os {
        // At the moment we use Fontconfig on both Linux and Windows.
        compile(&mut cppcfg, "layout/xetex-XeTeXFontMgr_FC.cpp");
    }

    if target.contains("-msvc") {
        cppcfg.flag("/EHsc");
    }

    // OK, back to generic build rules.

    cppcfg.compile("libtectonic_xetex_layout.a");

    deps.emit();

    // Copy the static header file for C preprocessing convenience.

    let mut main_header_src = manifest_dir.clone();
    main_header_src.push("layout");
    main_header_src.push("tectonic_xetex_layout.h");

    let mut main_header_dest = PathBuf::from(out_dir.clone());
    main_header_dest.push("tectonic_xetex_layout.h");

    std::fs::copy(&main_header_src, &main_header_dest).expect("failed to copy main header");

    let mut gen_header_src = manifest_dir;
    gen_header_src.push("layout");
    gen_header_src.push("layout_bindings.h");

    let mut gen_header_dest = PathBuf::from(out_dir.clone());
    gen_header_dest.push("layout_bindings.h");

    std::fs::copy(&gen_header_src, &gen_header_dest).expect("failed to copy bindgen header");

    // Cargo exposes this as the environment variable DEP_XXX_INCLUDE_PATH,
    // where XXX is the "links" setting in Cargo.toml. This is the key element
    // that allows us to have a network of crates containing both C/C++ and Rust
    // code that all interlink.

    print!("cargo:include-path={out_dir}");

    for item in harfbuzz_include_path.split(';') {
        print!(";{item}");
    }

    for item in freetype2_include_path.split(';') {
        print!(";{item}");
    }

    for item in graphite2_include_path.split(';') {
        print!(";{item}");
    }

    for item in icu_include_path.split(';') {
        print!(";{item}");
    }

    println!();

    let info = os_info::get();

    if info.os_type() == os_info::Type::Macos {
        if let &os_info::Version::Semantic(major, minor, _) = info.version() {
            if (major == 10 && minor < 6) || major < 10 {
                println!("cargo:rustc-cfg=feature=\"MACOS_LE_10_6\"")
            }
        }
    }
}
