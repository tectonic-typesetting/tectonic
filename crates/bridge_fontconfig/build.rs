use std::path::Path;
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

    pub fn foreach_include_path(&self, f: impl FnMut(&Path)) {
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
    let is_mac_os = target_cfg!(target_os = "macos");

    // Find any necessary deps.

    let dep_cfg = Configuration::default();
    let dep = PlatformLayoutDeps::new(&dep_cfg, is_mac_os);

    // This is the key. What we print here will be propagated into depending
    // crates' build scripts as the environment variable DEP_FONTCONFIG_INCLUDE_PATH,
    // allowing them to find the headers internally. If/when we start vendoring
    // FreeType, this can become $OUT_DIR.
    let mut sep = "cargo:include-path=";

    dep.foreach_include_path(|p| {
        print!("{}{}", sep, p.to_str().unwrap());
        sep = ";";
    });

    println!();

    dep.emit();
}
