// tests/trip.rs - implemention the TRIP test for Tectonic
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

//! Our incarnation of the classic TRIP test. Unfortunately, the test is
//! defined in terms of the precise terminal output and error handling behavior
//! of the engine, so you can't do anything to improve the (incredibly poor) UX
//! of the TeX engine without having to fudge what "the TRIP test" is. That is
//! what we have done.
//!
//! Cargo tries to run tests in multiple simultaneous threads, which of course
//! totally fails for Tectonic since the engine has tons of global state. The
//! multithreading can be disabled by setting the RUST_TEST_THREADS environment
//! variable to "1", but that's an annoying solution. So, we use a global mutex
//! to achieve the same effect. Classy.

use std::default::Default;

use tectonic::engines::NoopIoEventBackend;
use tectonic::io::testing::SingleInputFileIo;
use tectonic::io::{FilesystemPrimaryInputIo, IoProvider, IoStack, MemoryIo};
use tectonic::TexEngine;
use tectonic_status_base::NoopStatusBackend;

#[path = "util/mod.rs"]
mod util;
use crate::util::{test_path, ExpectedInfo};

#[test]
fn trip_test() {
    util::set_test_root();

    let mut p = test_path(&["trip", "trip"]);

    // An IoProvider for the input file.
    p.set_extension("tex");
    let mut tex = FilesystemPrimaryInputIo::new(&p);

    // And the TFM file.
    p.set_extension("tfm");
    let mut tfm = SingleInputFileIo::new(&p);

    // Read in the expected outputs.
    let expected_log = ExpectedInfo::read_with_extension(&mut p, "log");
    let expected_xdv = ExpectedInfo::read_with_extension(&mut p, "xdv");
    let expected_fot = ExpectedInfo::read_with_extension(&mut p, "fot");
    p.set_file_name("tripos");
    let expected_os = ExpectedInfo::read_with_extension(&mut p, "tex");

    // MemoryIo layer that will accept the outputs. Save `files` since the
    // engine consumes `mem`.
    let mut mem = MemoryIo::new(true);

    // First engine pass -- make the format file.
    {
        let mut io = IoStack::new(vec![&mut mem as &mut dyn IoProvider, &mut tex, &mut tfm]);
        TexEngine::new()
            .halt_on_error_mode(false)
            .initex_mode(true)
            .process(
                &mut io,
                &mut NoopIoEventBackend::default(),
                &mut NoopStatusBackend::default(),
                "INITEX",
                "trip",
                &Default::default(),
            )
            .unwrap();
    }

    // Second pass -- process it
    {
        let mut io = IoStack::new(vec![&mut mem as &mut dyn IoProvider, &mut tex, &mut tfm]);
        TexEngine::new()
            .halt_on_error_mode(false)
            .initex_mode(false)
            .process(
                &mut io,
                &mut NoopIoEventBackend::default(),
                &mut NoopStatusBackend::default(),
                "trip.fmt",
                "trip",
                &Default::default(),
            )
            .unwrap();
    }

    // Check that outputs match expectations.
    let files = &*mem.files.borrow();
    expected_log.test_from_collection(files);
    expected_xdv.test_from_collection(files);
    expected_os.test_from_collection(files);
    expected_fot.test_data(&files.get("").unwrap().data);
}

#[test]
fn etrip_test() {
    util::set_test_root();

    let mut p = test_path(&["trip", "etrip"]);

    // An IoProvider the input file.
    p.set_extension("tex");
    let mut tex = FilesystemPrimaryInputIo::new(&p);

    // And the TFM file.
    p.set_extension("tfm");
    let mut tfm = SingleInputFileIo::new(&p);

    // Read in the expected outputs.
    let expected_log = ExpectedInfo::read_with_extension(&mut p, "log");
    let expected_xdv = ExpectedInfo::read_with_extension(&mut p, "xdv");
    let expected_fot = ExpectedInfo::read_with_extension(&mut p, "fot");
    let expected_out = ExpectedInfo::read_with_extension(&mut p, "out");

    // MemoryIo layer that will accept the outputs. Save `files` since the
    // engine consumes `mem`.
    let mut mem = MemoryIo::new(true);
    let files = mem.files.clone();

    // First engine pass -- make the format file.
    {
        let mut io = IoStack::new(vec![&mut mem as &mut dyn IoProvider, &mut tex, &mut tfm]);
        TexEngine::new()
            .halt_on_error_mode(false)
            .initex_mode(true)
            .process(
                &mut io,
                &mut NoopIoEventBackend::default(),
                &mut NoopStatusBackend::default(),
                "INITEX",
                "etrip",
                &Default::default(),
            )
            .unwrap();
    }

    // Second pass -- process it
    {
        let mut io = IoStack::new(vec![&mut mem, &mut tex, &mut tfm]);
        TexEngine::new()
            .halt_on_error_mode(false)
            .initex_mode(false)
            .process(
                &mut io,
                &mut NoopIoEventBackend::default(),
                &mut NoopStatusBackend::default(),
                "etrip.fmt",
                "etrip",
                &Default::default(),
            )
            .unwrap();
    }

    // Check that outputs match expectations.
    let files = &*files.borrow();
    expected_log.test_from_collection(files);
    expected_xdv.test_from_collection(files);
    expected_out.test_from_collection(files);
    expected_fot.test_data(&files.get("").unwrap().data);
}
