//! Integration tests of the bundles crate.

use std::env;
use std::path::PathBuf;
use std::process::Command;

const PRE_FORMAT_VERSION: u32 = 31;
const TEST_FORMAT_VERSION: u32 = 32;
const TEST_BUNDLE_LOCKED: &str = "https://example.com/locked_bundle.tar";
const TEST_BUNDLE_PREFIX: &str = "https://custom.example.com";

/// Runs the test project and gets the output
fn run_test_program(format_version: u32, env_vars: &[(&str, &str)]) -> String {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_dir = PathBuf::from(&manifest_dir)
        .join("tests")
        .join("bundle_env_overrides_test");
    let target_platform = env::var("TARGET").unwrap();

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--quiet")
        .arg("--target")
        .arg(target_platform)
        .arg("--")
        .arg(format_version.to_string())
        .current_dir(&test_dir);

    for (key, value) in env_vars {
        cmd.env(key, value);
    }

    let output = cmd.output().expect("Failed to run test program");
    assert!(
        output.status.success(),
        "Test program failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn test_bundle_locked() {
    let url_locked = run_test_program(
        PRE_FORMAT_VERSION,
        &[("TECTONIC_BUNDLE_LOCKED", TEST_BUNDLE_LOCKED)],
    );
    let url_locked_versioned = run_test_program(
        TEST_FORMAT_VERSION,
        &[("TECTONIC_BUNDLE_LOCKED", TEST_BUNDLE_LOCKED)],
    );

    assert_eq!(url_locked, TEST_BUNDLE_LOCKED);
    assert_eq!(url_locked_versioned, TEST_BUNDLE_LOCKED);
}

#[test]
fn test_bundle_prefix() {
    let url_prefixed = run_test_program(
        PRE_FORMAT_VERSION,
        &[("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX)],
    );
    let url_prefixed_versioned = run_test_program(
        TEST_FORMAT_VERSION,
        &[("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX)],
    );

    assert_eq!(
        url_prefixed,
        format!("{TEST_BUNDLE_PREFIX}/default_bundle.tar")
    );
    assert_eq!(
        url_prefixed_versioned,
        format!("{TEST_BUNDLE_PREFIX}/default_bundle_v{TEST_FORMAT_VERSION}.tar")
    );
}

#[test]
fn test_precedence_locked_over_prefix() {
    let url_both_env_set = run_test_program(
        TEST_FORMAT_VERSION,
        &[
            ("TECTONIC_BUNDLE_LOCKED", TEST_BUNDLE_LOCKED),
            ("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX),
        ],
    );
    assert_eq!(url_both_env_set, TEST_BUNDLE_LOCKED);
}

#[test]
fn test_empty_locked_bundle_ignored() {
    let url_empty_locked = run_test_program(
        TEST_FORMAT_VERSION,
        &[
            ("TECTONIC_BUNDLE_LOCKED", ""),
            ("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX),
        ],
    );
    assert_eq!(
        url_empty_locked,
        format!("{TEST_BUNDLE_PREFIX}/default_bundle_v{TEST_FORMAT_VERSION}.tar",)
    );
}
