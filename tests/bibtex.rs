// Copyright 2016-2019 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashSet;
use std::default::Default;

use tectonic::engines::NoopIoEventBackend;
use tectonic::io::{FilesystemIo, IoProvider, IoStack, MemoryIo};
use tectonic::BibtexEngine;
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

        let mut io = IoStack::new(io_list);

        let mut events = NoopIoEventBackend::default();
        let mut status = NoopStatusBackend::default();

        BibtexEngine::new()
            .process(
                &mut io,
                &mut events,
                &mut status,
                &auxname,
                &Default::default(),
            )
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
