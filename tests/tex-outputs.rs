// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

#[macro_use] extern crate lazy_static;
extern crate tectonic;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Mutex;

use tectonic::engines::NoopIoEventBackend;
use tectonic::io::{IoStack, MemoryIo};
use tectonic::io::testing::SingleInputFileIo;
use tectonic::status::NoopStatusBackend;
use tectonic::TexEngine;

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");


lazy_static! {
    static ref LOCK: Mutex<u8> = Mutex::new(0u8);
}


fn do_one(stem: &str) {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut p = PathBuf::from(TOP);
    p.push("tests");

    // An IoProvider for the format file.
    let mut fmt_path = p.clone();
    fmt_path.push("xetex.fmt");
    let mut fmt = SingleInputFileIo::new(&fmt_path);

    // Ditto for the input file.
    p.push("tex-outputs");
    p.push(stem);
    p.set_extension("tex");
    let texname = p.file_name().unwrap().to_str().unwrap().to_owned();
    let mut tex = SingleInputFileIo::new(&p);

    // Read in the expected "log" output ...
    p.set_extension("log");
    let logname = p.file_name().unwrap().to_owned();
    let mut expected_log;
    {
        let mut f = File::open(&p).unwrap();
        expected_log = Vec::new();
        f.read_to_end(&mut expected_log).unwrap();
    }

    // ... and the expected XDVI output.
    p.set_extension("xdv");
    let xdvname = p.file_name().unwrap().to_owned();
    let mut expected_xdv;
    {
        let mut f = File::open(&p).unwrap();
        expected_xdv = Vec::new();
        f.read_to_end(&mut expected_xdv).unwrap();
    }

    // MemoryIo layer that will accept the outputs.
    let mut mem = MemoryIo::new(true);

    // Run the engine!
    {
        let mut io = IoStack::new(vec![
            &mut mem,
            &mut tex,
            &mut fmt,
        ]);
        let mut e = TexEngine::new ();
        e.process(&mut io, &mut NoopIoEventBackend::new(),
                  &mut NoopStatusBackend::new(), "xetex.fmt", &texname).unwrap();
    }

    // Check that log and xdv match expectations.

    let files = mem.files.borrow();

    let observed_log = files.get(&logname).unwrap();
    assert_eq!(&expected_log, observed_log);

    let observed_xdv = files.get(&xdvname).unwrap();
    assert_eq!(&expected_xdv, observed_xdv);
}


// Keep these alphabetized.

#[test]
fn md5_of_hello() { do_one("md5_of_hello") }

#[test]
fn negative_roman_numeral() { do_one("negative_roman_numeral") }

#[test]
fn the_letter_a() { do_one("the_letter_a") }
