//! fontconfig build script. For now, we always find it externally.

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

fn main() {
    // Find any necessary deps.

    let dep_cfg = Configuration::default();
    let dep = Dependency::probe(FontconfigSpec, &dep_cfg);

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
