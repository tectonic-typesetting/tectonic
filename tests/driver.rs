// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! This test rig is a total hack to quickly exercise `src/driver.rs`.
//!
//! I should make it real, but I just want Codecov to stop complaining about
//! my test coverage. To re-use the `tex-outputs` test artifacts, the
//! ProcessingSessionBuilder will need to learn how to tell `xdvipdfmx` to
//! enable the reproducibility options used in the `tex-outputs` test rig.

extern crate tectonic;
extern crate tempdir;

use tectonic::config::PersistentConfig;
use tectonic::driver::ProcessingSessionBuilder;
use tectonic::status::ChatterLevel;
use tectonic::status::termcolor::TermcolorStatusBackend;
use tempdir::TempDir;

mod util;


// Keep these alphabetized.

#[test]
fn the_letter_a() {
    util::set_test_root();

    let _config = PersistentConfig::default();

    // The "Normal" chatter escapes the test rig's attempts to eat stdout ...
    let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);

    let bundle = util::TestBundle::default();

    let tempdir = TempDir::new("tectonic_driver_test").unwrap();

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

    session.run(&mut status)
        .expect("failed to execute processing session");
}
