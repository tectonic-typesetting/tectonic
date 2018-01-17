// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate flate2;
#[macro_use] extern crate lazy_static;
extern crate tectonic;

use flate2::read::GzDecoder;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tectonic::errors::{DefinitelySame, ErrorKind, Result};
use tectonic::engines::NoopIoEventBackend;
use tectonic::engines::tex::TexResult;
use tectonic::io::{FilesystemIo, FilesystemPrimaryInputIo, IoStack, MemoryIo, try_open_file};
use tectonic::io::testing::SingleInputFileIo;
use tectonic::status::NoopStatusBackend;
use tectonic::TexEngine;

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");

mod util;
use util::{assert_file_eq, read_file};

lazy_static! {
    static ref LOCK: Mutex<u8> = Mutex::new(0u8);
}


fn set_up_format_file(tests_dir: &Path) -> Result<SingleInputFileIo> {
    let mut fmt_path = tests_dir.to_owned();
    fmt_path.push("plain.fmt.gz");

    if try_open_file(&fmt_path).is_not_available() {
        // Well, we need to regenerate the format file. Not too difficult.
        let mut mem = MemoryIo::new(true);

        let mut plain_format_dir = tests_dir.to_owned();
        plain_format_dir.push("formats");
        plain_format_dir.push("plain");
        let mut fs_support = FilesystemIo::new(&plain_format_dir, false, false, HashSet::new());

        plain_format_dir.push("plain");
        plain_format_dir.set_extension("tex");
        let mut fs_primary = FilesystemPrimaryInputIo::new(&plain_format_dir);

        {
            let mut io = IoStack::new(vec![
                &mut mem,
                &mut fs_primary,
                &mut fs_support,
            ]);

            TexEngine::new()
                .halt_on_error_mode(true)
                .initex_mode(true)
                .process(&mut io, &mut NoopIoEventBackend::new(),
                          &mut NoopStatusBackend::new(), "UNUSED.fmt.gz", "plain.tex")?;
        }

        let mut fmt_file = File::create(&fmt_path)?;
        fmt_file.write_all(mem.files.borrow().get(OsStr::new("plain.fmt.gz")).unwrap())?;
    }

    Ok(SingleInputFileIo::new(&fmt_path))
}

struct TestCase {
    stem: String,
    expected_result: Result<TexResult>,
    check_synctex: bool,
    // TODO: would be nice to reuse ExpectedInfo from trip.rs
}


impl TestCase {
    fn new(stem: &str) -> Self {
        TestCase {
            stem: stem.to_owned(),
            expected_result: Ok(TexResult::Spotless),
            check_synctex: false,
        }
    }

    fn check_synctex(&mut self, check_synctex: bool) -> &mut Self {
        self.check_synctex = check_synctex;
        self
    }

    fn expect(&mut self, result: Result<TexResult>) -> &mut Self {
        self.expected_result = result;
        self
    }

    fn expect_msg(&mut self, msg: &str) -> &mut Self {
        self.expect(Err(ErrorKind::Msg(msg.to_owned()).into()))
    }

    fn go(&self) {
        let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

        let expect_xdv = self.expected_result.is_ok();

        let mut p = PathBuf::from(TOP);
        p.push("tests");

        // IoProvider for the format file; with magic to generate the format
        // on-the-fly if needed.
        let mut fmt = set_up_format_file(&p).expect("couldn't write format file");

        // Ditto for the input file.
        p.push("tex-outputs");
        p.push(&self.stem);
        p.set_extension("tex");
        let texname = p.file_name().unwrap().to_str().unwrap().to_owned();
        let mut tex = FilesystemPrimaryInputIo::new(&p);

        // Read in the expected "log" output ...
        p.set_extension("log");
        let logname = p.file_name().unwrap().to_owned();
        let expected_log = read_file(&p);

        // MemoryIo layer that will accept the outputs.
        let mut mem = MemoryIo::new(true);

        // Run the engine!
        let res = {
            let mut io = IoStack::new(vec![
                &mut mem,
                &mut tex,
                &mut fmt,
            ]);
            TexEngine::new()
                .process(&mut io, &mut NoopIoEventBackend::new(),
                         &mut NoopStatusBackend::new(), "plain.fmt.gz", &texname)
        };

        if !res.definitely_same(&self.expected_result) {
            panic!(format!("expected TeX result {:?}, got {:?}", self.expected_result, res));
        }

        // Check that log and xdv match expectations.

        let files = mem.files.borrow();

        let observed_log = files.get(&logname).unwrap();
        assert_file_eq(&logname, &expected_log, observed_log);

        if expect_xdv {
            p.set_extension("xdv");
            let xdvname = p.file_name().unwrap().to_owned();
            let expected_xdv = read_file(&p);
            let observed_xdv = files.get(&xdvname).unwrap();
            assert_file_eq(&xdvname, &expected_xdv, observed_xdv);
        }

        if self.check_synctex {
            p.set_extension("synctex.gz");
            // Gzipped files seem to be platform dependent and so we decompress them first.
            let mut expected_synctex = Vec::new();
            GzDecoder::new(File::open(&p).unwrap())
                .read_to_end(&mut expected_synctex).unwrap();
            let synctexname = p.file_name().unwrap().to_owned();
            let mut observed_synctex = Vec::new();
            GzDecoder::new(&files.get(&synctexname).unwrap()[..])
                .read_to_end(&mut observed_synctex).unwrap();
            assert_file_eq(&synctexname, &expected_synctex, &observed_synctex);
        }
    }
}


// Keep these alphabetized.

#[test]
fn md5_of_hello() { TestCase::new("md5_of_hello").go() }

#[test]
fn negative_roman_numeral() { TestCase::new("negative_roman_numeral").go() }

#[test]
fn pdfoutput() { TestCase::new("pdfoutput").go() }

#[test]
fn synctex() { TestCase::new("synctex").check_synctex(true).go() }

#[test]
fn tectoniccodatokens_errinside() {
    TestCase::new("tectoniccodatokens_errinside")
        .expect_msg("halted on potentially-recoverable error as specified")
        .go()
}

#[test]
fn tectoniccodatokens_noend() {
    TestCase::new("tectoniccodatokens_noend")
        .expect_msg("*** (job aborted, no legal \\end found)")
        .go()
}

#[test]
fn tectoniccodatokens_ok() { TestCase::new("tectoniccodatokens_ok").go() }

#[test]
fn the_letter_a() { TestCase::new("the_letter_a").go() }
