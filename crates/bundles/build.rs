// Copyright 2016-2025 the Tectonic Project
// Licensed under the MIT License.

use std::env;

/// The current hardcoded default prefix for tectonic's bundle.
const TECTONIC_BUNDLE_PREFIX_DEFAULT: &str = "https://relay.fullyjustified.net";

/// Environment variable names to look for the bundle URLs.
const LOCKED_VAR_NAME: &str = "TECTONIC_BUNDLE_LOCKED";
const PREFIX_VAR_NAME: &str = "TECTONIC_BUNDLE_PREFIX";

/// Sets the environment variables for the default bundle.
///
/// `${TECTONIC_BUNDLE_PREFIX}` would lead to a url in the form of
/// `${TECTONIC_BUNDLE_PREFIX}/default_bundle.tar`, while the optional
/// "locked" url, `${TECTONIC_BUNDLE_LOCKED}`, can be used to pin the
/// default bundle to a specific version if specified. This would be useful
/// for reproducible builds.
///
fn bundle_presets() {
    // load from env
    let bundle_locked = env::var(LOCKED_VAR_NAME).unwrap_or("".into());
    let bundle_prefix = match env::var(PREFIX_VAR_NAME) {
        Ok(x) if !x.is_empty() => x,
        _ => TECTONIC_BUNDLE_PREFIX_DEFAULT.into(),
    };

    // export to rustc
    println!("cargo::rustc-env={LOCKED_VAR_NAME}={bundle_locked}");
    println!("cargo::rustc-env={PREFIX_VAR_NAME}={bundle_prefix}");
}

fn main() {
    bundle_presets();
}
