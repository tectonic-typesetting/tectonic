// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

extern crate tectonic;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use tectonic::io::{IOStack, MemoryIO};
use tectonic::io::testing::SingleInputFileIO;
use tectonic::Engine;

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");


fn do_one(stem: &str) {
    let mut p = PathBuf::from(TOP);
    p.push("tests");

    // An IOProvider for the format file.
    let mut fmt_path = p.clone();
    fmt_path.push("xetex.fmt");
    let fmt = SingleInputFileIO::new(&fmt_path);

    // Ditto for the input file.
    p.push("tex-outputs");
    p.push(stem);
    p.set_extension("tex");
    let texname = p.file_name().unwrap().to_str().unwrap().to_owned();
    let tex = SingleInputFileIO::new(&p);

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

    // MemoryIO layer that will accept the outputs. Save `files` since the
    // engine consumes `mem`.
    let mem = MemoryIO::new(true);
    let files = mem.files.clone();

    // Run the engine!
    let mut e = Engine::new (IOStack::new(vec![
        Box::new(mem),
        Box::new(tex),
        Box::new(fmt),
    ]));
    e.set_output_format ("xdv");
    e.process("xetex.fmt", &texname).unwrap();

    // Check that log and xdv match expectations.

    let files = files.borrow();

    let observed_log = files.get(&logname).unwrap();
    assert_eq!(&expected_log, observed_log);

    let observed_xdv = files.get(&xdvname).unwrap();
    assert_eq!(&expected_xdv, observed_xdv);
}

#[test]
fn the_letter_a() { do_one("the-letter-a") }
