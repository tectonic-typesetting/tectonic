// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashSet;
use std::default::Default;
use std::path::PathBuf;

use tectonic::io::{FilesystemIo, IoProvider, IoStack, MemoryIo};
use tectonic::BibtexEngine;
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_status_base::NoopStatusBackend;

#[path = "util/mod.rs"]
mod util;
use crate::util::{test_path, Expected, ExpectedFile};

struct TestCase {
    stem: String,
    subdir: Option<String>,
    test_bbl: bool,
}

impl TestCase {
    fn new(stem: &str, subdir: Option<&str>) -> Self {
        TestCase {
            stem: stem.to_owned(),
            subdir: subdir.map(String::from),
            test_bbl: true,
        }
    }

    fn test_bbl(mut self, test: bool) -> Self {
        self.test_bbl = test;
        self
    }

    fn test_dir(&self) -> PathBuf {
        let mut p = test_path(&["bibtex"]);
        if let Some(subdir) = &self.subdir {
            p.push(subdir);
        }
        p
    }

    fn go(&mut self) {
        util::set_test_root();

        let mut p = self.test_dir();

        let auxname = format!("{}.aux", self.stem);

        // MemoryIo layer that will accept the outputs.
        let mut mem = MemoryIo::new(true);

        let mut assets = FilesystemIo::new(&p, false, false, HashSet::new());

        let io_list: Vec<&mut dyn IoProvider> = vec![&mut mem, &mut assets];

        let io = IoStack::new(io_list);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend::default();
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);

        BibtexEngine::new()
            .process(&mut launcher, &auxname, &Default::default())
            .unwrap();

        // Check that outputs match expectations.

        p.push(&self.stem);

        let files = mem.files.borrow();

        let mut expect = Expected::new();

        if self.test_bbl {
            expect =
                expect.file(ExpectedFile::read_with_extension(&mut p, "bbl").collection(&files));
        }

        expect
            .file(ExpectedFile::read_with_extension(&mut p, "blg").collection(&files))
            .finish();
    }
}

#[test]
fn test_single_entry() {
    TestCase::new("single_entry", Some("cites")).go()
}

#[test]
fn test_brace_string() {
    TestCase::new("odd_strings", Some("cites")).go();
}

#[test]
fn test_many() {
    TestCase::new("many", Some("cites")).go();
}

#[test]
fn test_colon() {
    TestCase::new("colon", Some("cites")).go();
}

#[test]
fn test_empty_files() {
    TestCase::new("empty", None).test_bbl(false).go()
}

#[test]
fn test_mismatched_function() {
    TestCase::new("function", Some("mismatched_braces"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_expr() {
    TestCase::new("expr", Some("mismatched_braces"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_data() {
    TestCase::new("data", Some("mismatched_braces"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_style() {
    TestCase::new("style", Some("mismatched_braces"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_duplicated_data() {
    TestCase::new("data", Some("duplicated"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_duplicated_style() {
    TestCase::new("style", Some("duplicated"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_bad_crossref() {
    TestCase::new("bad", Some("crossref")).go();
}

#[test]
fn test_min_crossref() {
    TestCase::new("min", Some("crossref")).go();
}

#[test]
fn test_single_preamble() {
    TestCase::new("single", Some("preamble")).go();
}

#[test]
fn test_many_preamble() {
    TestCase::new("many", Some("preamble")).go();
}

#[test]
fn test_nested_aux() {
    TestCase::new("nested", Some("aux_files")).go();
}
