use tectonic_cfg_support::target_cfg;

fn main() {
    let is_mac_os = target_cfg!(target_os = "macos");

    if is_mac_os {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }
}
