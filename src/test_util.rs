// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! This inelegant module helps with testing.
//!
//! We would like it if all of our tests -- unit tests (embedded in this
//! crate), integration tests (in `tests/`), and doctests -- used the "test
//! bundle" in `tests/assets/`, rather than having to touch the network to
//! work. Unfortunately, the integration and doctests are compiled against the
//! "default" build of the main crate -- that is, the version built *without*
//! #[cfg(test)]. Therefore, to support such usage in all three scenarios, we
//! need to build the code into the main crate unconditionally. We can hide it
//! from the documentation, but we can't use #[cfg(test)] attributes to
//! conditionally compile code.
//!
//! So, that's what we do. Importantly, the code here should all be nice and
//! lightweight, and have very low impact on the final binary artifacts we
//! produce. We also take care not to embed the build directory into the
//! final binaries.
//!
//! There are two main functionalities covered in this module. First, there
//! are some functions that help in locating test assets and making a fakey
//! "bundle" that uses them.
//!
//! Second, there is some infrastructure to activate a "test" mode that
//! affects the crate's behavior in a few ways. The binary we distribute
//! activates this mode when a magic environment variable is set.
//!
//! This is all implemented so that out doctests can look totally innocuous --
//! they are what you'd actually want in your own code, except we have a hidden
//! one-liner when needed:
//!
//! # tectonic::test_util::activate_test_mode_augmented(env!("CARGO_MANIFEST_DIR"));
//!
//! That call simultaneously tells this module where to find the test assets,
//! and also activates the test mode.

use std::{collections::HashSet, env, ffi::OsStr, path::PathBuf};
use tectonic_errors::Result;

use crate::{
    digest::DigestData,
    io::{Bundle, FilesystemIo, InputHandle, IoProvider, OpenResult},
    status::StatusBackend,
};

/// The name of the environment variable that the test code will consult to
/// figure out where to find the testing resource files.
pub const TEST_ROOT_ENV_VAR: &str = "TECTONIC_INTERNAL_TEST_ROOT";

/// Set `TEST_ROOT_ENV_VAR` in the current process to the specified value,
/// with a path element "tests" appended. If the variable was previously
/// unset, this will make it such that `test_path()` and the other pieces of
/// testing infrastructure in this module will start working.
///
/// The peculiar form of this function makes for easy one-liners in the test
/// code exploiting the environment variable $CARGO_MANIFEST_DIR.
pub fn set_test_root_augmented<V: AsRef<OsStr>>(root: V) {
    let mut root = PathBuf::from(root.as_ref());
    root.push("tests");
    env::set_var(TEST_ROOT_ENV_VAR, root);
}

/// Activate this crate's "test mode", if-and-only-if the magic testing
/// environment variable has been set. This allows testing of the Tectonic
/// executable in a transparent way — notably, we avoid embedding the build
/// prefix in the resulting binary, and we don't need to pass it any magical
/// command line arguments.
pub fn maybe_activate_test_mode() {
    if env::var_os(TEST_ROOT_ENV_VAR).is_none() {
        return;
    }

    crate::config::activate_config_test_mode(true);
}

/// A combination of the two above functions. Set the "test root" variable,
/// making it such that the testing infrastructure in this module can work;
/// and then activate "test mode", such that certain other parts of the crate
/// alter their behavior for use in the test environment. This makes for
/// convenient doctests — you can secretly run them in the special test setup
/// with a one-liner:
///
/// # tectonic::test_util::activate_test_mode_augmented(env!("CARGO_MANIFEST_DIR"));
pub fn activate_test_mode_augmented<V: AsRef<OsStr>>(root: V) {
    set_test_root_augmented(root);
    maybe_activate_test_mode();
}

/// Obtain a path to a testing resource file. The environment variable whose
/// name is stored in the constant `TEST_ROOT_ENV_VAR` must be set to an
/// appropriate directory. (Note: `TEST_ROOT_ENV_VAR` is a constant giving the
/// *name* of the relevant variable — not the name of the variable itself!)
pub fn test_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env::var_os(TEST_ROOT_ENV_VAR).expect(
        "Tectonic testing infrastructure cannot be used without \
         setting the magic test-root environment variable",
    ));
    path.push(parts.iter().collect::<PathBuf>());
    path
}

/// Utility for being able to treat the "assets/" directory as a bundle.
pub struct TestBundle(FilesystemIo);

impl Default for TestBundle {
    fn default() -> Self {
        TestBundle(FilesystemIo::new(
            &test_path(&["assets"]),
            false,
            false,
            HashSet::new(),
        ))
    }
}

impl IoProvider for TestBundle {
    // All other functions can default to NotAvailable/error.
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        self.0.input_open_name(name, status)
    }
}

impl Bundle for TestBundle {
    fn get_digest(&mut self, _status: &mut dyn StatusBackend) -> Result<DigestData> {
        Ok(DigestData::zeros())
    }
}
