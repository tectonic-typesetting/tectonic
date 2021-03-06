// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashSet;
use std::default::Default;

use tectonic::io::{FilesystemIo, IoProvider, IoStack, MemoryIo};
use tectonic::BibtexEngine;
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_io_base::stdstreams::GenuineStdoutIo;
use tectonic_status_base::NoopStatusBackend;

#[path = "util/mod.rs"]
mod util;
use crate::util::{test_path, ExpectedInfo};

struct TestCase {
    stem: String,
}

impl TestCase {
    fn new(stem: &str) -> Self {
        TestCase {
            stem: stem.to_owned(),
        }
    }

    fn go(&mut self) {
        util::set_test_root();

        let mut p = test_path(&["bibtex"]);

        p.push(&self.stem);

        p.set_extension("aux");
        let auxname = p.file_name().unwrap().to_str().unwrap().to_owned();

        // MemoryIo layer that will accept the outputs.
        let mut mem = MemoryIo::new(true);

        let mut assets = FilesystemIo::new(&test_path(&["bibtex"]), false, false, HashSet::new());

        let mut genio = GenuineStdoutIo::new();

        let io_list: Vec<&mut dyn IoProvider> = vec![&mut genio, &mut mem, &mut assets];

        let io = IoStack::new(io_list);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend::default();
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);

        BibtexEngine::new()
            .process(&mut launcher, &auxname, &Default::default())
            .unwrap();

        // Check that outputs match expectations.

        let expected_bbl = ExpectedInfo::read_with_extension(&mut p, "bbl");
        let expected_blg = ExpectedInfo::read_with_extension(&mut p, "blg");

        let files = mem.files.borrow();

        expected_bbl.test_from_collection(&files);
        expected_blg.test_from_collection(&files);
    }
}

#[test]
fn single_entry() {
    TestCase::new("single_entry").go()
}
