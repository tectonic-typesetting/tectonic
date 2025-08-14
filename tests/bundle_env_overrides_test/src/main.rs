#![doc = include_str!("../README.md")]

use tectonic_bundles::get_fallback_bundle_url;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let format_version: u32 = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .expect("one must provide a valid format version to `get_fallback_bundle_url`");

    let url = get_fallback_bundle_url(format_version);
    println!("{url}");
}
