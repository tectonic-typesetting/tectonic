// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::env;

/// The current hardcoded default prefix for tectonic's web bundle.
const TECTONIC_WEB_BUNDLE_PREFIX_DEFAULT: &str = "https://relay.fullyjustified.net";

/// Environment variable names to look for the bundle URLs.
const LOCKED_VAR: &str = "TECTONIC_WEB_BUNDLE_LOCKED";
const PREFIX_VAR: &str = "TECTONIC_WEB_BUNDLE_PREFIX";

/// Sets the environment variables for the default web bundle.
///
/// `${TECTONIC_WEB_BUNDLE_PREFIX}` would lead to a url in the form of
/// `${TECTONIC_WEB_BUNDLE_PREFIX}/default_bundle.tar`, while the optional
/// "locked" url, `${TECTONIC_WEB_BUNDLE_LOCKED}`, can be used to pin the
/// default bundle to a specific version if specified. This would be useful
/// for reproducible builds.
fn web_bundle_presets() {
    // load from env
    let web_bundle_locked = env::var(LOCKED_VAR).unwrap_or("".into());
    let web_bundle_prefix = match env::var(PREFIX_VAR) {
        Ok(x) if !x.is_empty() => x,
        _ => TECTONIC_WEB_BUNDLE_PREFIX_DEFAULT.into(),
    };

    // export to rustc
    println!("cargo:rustc-env={LOCKED_VAR}={web_bundle_locked}");
    println!("cargo:rustc-env={PREFIX_VAR}={web_bundle_prefix}");
}

fn main() {
    web_bundle_presets();
}
