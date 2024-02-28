// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashSet;
use std::path::PathBuf;

use tectonic::io::{FilesystemIo, IoProvider, IoStack, MemoryIo};
use tectonic::{errors::Result, BibtexEngine};
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_engine_xetex::TexOutcome;
use tectonic_status_base::NoopStatusBackend;

#[path = "util/mod.rs"]
mod util;
use crate::util::{test_path, Expected, ExpectedFile};

struct TestCase {
    parts: &'static [&'static str],
    test_bbl: bool,
    expected_result: Result<TexOutcome>,
}

impl TestCase {
    fn new(parts: &'static [&'static str]) -> Self {
        assert!(!parts.is_empty());
        TestCase {
            parts,
            test_bbl: true,
            expected_result: Ok(TexOutcome::Spotless),
        }
    }

    fn expect(mut self, result: Result<TexOutcome>) -> Self {
        self.expected_result = result;
        self
    }

    fn test_bbl(mut self, test: bool) -> Self {
        self.test_bbl = test;
        self
    }

    fn test_dir(&self) -> PathBuf {
        let mut p = test_path(&["bibtex"]);
        for sub in &self.parts[..self.parts.len() - 1] {
            p.push(sub);
        }
        p
    }

    fn go(self) {
        util::set_test_root();

        let mut p = self.test_dir();

        let auxname = format!("{}.aux", self.parts[self.parts.len() - 1]);

        // MemoryIo layer that will accept the outputs.
        let mut mem = MemoryIo::new(true);

        let mut assets = FilesystemIo::new(&p, false, false, HashSet::new());

        let io_list: Vec<&mut dyn IoProvider> = vec![&mut mem, &mut assets];

        let io = IoStack::new(io_list);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend::default();
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);

        let res = BibtexEngine::new().process(&mut launcher, &auxname, &Default::default());

        // Check that outputs match expectations.

        p.push(self.parts[self.parts.len() - 1]);

        let files = mem.files.borrow();

        let mut expect = Expected::new().res(self.expected_result, res);

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
    TestCase::new(&["cites", "single_entry"]).go()
}

#[test]
fn test_brace_string() {
    TestCase::new(&["cites", "odd_strings"]).go();
}

#[test]
fn test_many() {
    TestCase::new(&["cites", "many"])
        .expect(Ok(TexOutcome::Warnings))
        .go();
}

#[test]
fn test_colon() {
    TestCase::new(&["cites", "colon"])
        .expect(Ok(TexOutcome::Warnings))
        .go();
}

#[test]
fn test_control_sequences() {
    TestCase::new("control_seq", Some("cites")).go();
}

#[test]
fn test_multi_bib() {
    TestCase::new("multi_file", Some("cites")).go();
}

#[test]
fn test_empty_files() {
    TestCase::new(&["empty"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go()
}

#[test]
fn test_mismatched_function() {
    TestCase::new(&["mismatched_braces", "function"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_expr() {
    TestCase::new(&["mismatched_braces", "expr"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_data() {
    TestCase::new(&["mismatched_braces", "data"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_style() {
    TestCase::new(&["mismatched_braces", "style"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go();
}

#[test]
fn test_duplicated_data() {
    TestCase::new(&["duplicated", "data"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go();
}

#[test]
fn test_duplicated_style() {
    TestCase::new(&["duplicated", "style"])
        .expect(Ok(TexOutcome::Errors))
        .test_bbl(false)
        .go();
}

#[test]
fn test_bad_crossref() {
    TestCase::new(&["crossref", "bad"])
        .expect(Ok(TexOutcome::Errors))
        .go();
}

#[test]
fn test_min_crossref() {
    TestCase::new(&["crossref", "min"])
        .expect(Ok(TexOutcome::Warnings))
        .go();
}

#[test]
fn test_single_preamble() {
    TestCase::new(&["preamble", "single"])
        .expect(Ok(TexOutcome::Warnings))
        .go();
}

#[test]
fn test_many_preamble() {
    TestCase::new(&["preamble", "many"])
        .expect(Ok(TexOutcome::Warnings))
        .go();
}

#[test]
fn test_nested_aux() {
    TestCase::new(&["aux_files", "nested"]).go();
}

/// Test for [#1105](https://github.com/tectonic-typesetting/tectonic/issues/1105), with enough
/// citations in the aux and fields in the bst to require more than one allocation of field space
/// at once.
#[test]
fn test_lots_of_cites() {
    TestCase::new(&["aux_files", "lots_of_cites"])
        .expect(Ok(TexOutcome::Warnings))
        .test_bbl(false)
        .go();
}
