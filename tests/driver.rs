// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! This test rig is a total hack to quickly exercise `src/driver.rs`.
//!
//! I should make it real, but I just want Codecov to stop complaining about
//! my test coverage. To re-use the `tex-outputs` test artifacts, the
//! ProcessingSessionBuilder will need to learn how to tell `xdvipdfmx` to
//! enable the reproducibility options used in the `tex-outputs` test rig.

use tectonic::{
    config::PersistentConfig, driver::ProcessingSessionBuilder,
    status::termcolor::TermcolorStatusBackend, status::ChatterLevel, test_util::TestBundle,
};

mod util;

// Keep these alphabetized.

#[test]
fn the_letter_a() {
    util::set_test_root();

    let _config = PersistentConfig::default();

    // The "Normal" chatter escapes the test rig's attempts to eat stdout ...
    let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);

    let bundle = TestBundle::default();

    let tempdir = tempfile::Builder::new()
        .prefix("tectonic_driver_test")
        .tempdir()
        .unwrap();

    let mut pbuilder = ProcessingSessionBuilder::default();
    pbuilder
        .primary_input_path(util::test_path(&["tex-outputs", "the_letter_a.tex"]))
        .tex_input_name("the_letter_a.tex")
        .format_name("plain")
        .format_cache_path(util::test_path(&[]))
        .output_dir(tempdir.path())
        .bundle(Box::new(bundle));

    let mut session = pbuilder
        .create(&mut status)
        .expect("couldn't create processing session");

    session
        .run(&mut status)
        .expect("failed to execute processing session");
}
