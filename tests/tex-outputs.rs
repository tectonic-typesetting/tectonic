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
    let mut fmt = SingleInputFileIO::new(&fmt_path);

    // Ditto for the input file.
    p.push("tex-outputs");
    p.push(stem);
    p.set_extension("tex");
    let texname = p.file_name().unwrap().to_str().unwrap().to_owned();
    let mut tex = SingleInputFileIO::new(&p);

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

    // MemoryIO layer that will accept the outputs.
    let mut mem = MemoryIO::new(true);

    // Run the engine!
    {
        let mut io = IOStack::new(vec![
            &mut mem,
            &mut tex,
            &mut fmt,
        ]);
        let mut e = Engine::new ();
        e.set_output_format ("xdv");
        e.process_tex(&mut io, "xetex.fmt", &texname).unwrap();
    }

    // Check that log and xdv match expectations.

    let files = mem.files.borrow();

    let observed_log = files.get(&logname).unwrap();
    assert_eq!(&expected_log, observed_log);

    let observed_xdv = files.get(&xdvname).unwrap();
    assert_eq!(&expected_xdv, observed_xdv);
}

#[test]
fn the_letter_a() { do_one("the-letter-a") }
