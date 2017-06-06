// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate flate2;
#[macro_use] extern crate lazy_static;
extern crate tectonic;

use flate2::read::GzDecoder;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tectonic::engines::NoopIoEventBackend;
use tectonic::io::{FilesystemIo, IoStack, MemoryIo, try_open_file};
use tectonic::io::testing::SingleInputFileIo;
use tectonic::status::NoopStatusBackend;
use tectonic::TexEngine;

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");


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
        let mut fs = FilesystemIo::new(&plain_format_dir, false, false, HashSet::new());

        {
            let mut io = IoStack::new(vec![
                &mut mem,
                &mut fs,
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


fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut f = File::open(&path).unwrap();
    f.read_to_end(&mut buffer).unwrap();
    buffer
}


pub fn test_file(name: &OsStr, expected: &Vec<u8>, observed: &Vec<u8>) {
    if expected == observed {
        return;
    }

    // For nontrivial tests, it's really tough to figure out what
    // changed without being able to do diffs, etc. So, write out the
    // buffers.

    {
        let mut n = name.to_owned();
        n.push(".expected");
        let mut f = File::create(&n).expect(&format!("failed to create {} for test failure diagnosis", n.to_string_lossy()));
        f.write_all(expected).expect(&format!("failed to write {} for test failure diagnosis", n.to_string_lossy()));
    }
    {
        let mut n = name.to_owned();
        n.push(".observed");
        let mut f = File::create(&n).expect(&format!("failed to create {} for test failure diagnosis", n.to_string_lossy()));
        f.write_all(observed).expect(&format!("failed to write {} for test failure diagnosis", n.to_string_lossy()));
    }

    panic!("difference in {}; contents saved to disk", name.to_string_lossy());
}


fn do_one(stem: &str, check_synctex: bool) {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut p = PathBuf::from(TOP);
    p.push("tests");

    // IoProvider for the format file; with magic to generate the format
    // on-the-fly if needed.
    let mut fmt = set_up_format_file(&p).expect("couldn't write format file");

    // Ditto for the input file.
    p.push("tex-outputs");
    p.push(stem);
    p.set_extension("tex");
    let texname = p.file_name().unwrap().to_str().unwrap().to_owned();
    let mut tex = SingleInputFileIo::new(&p);

    // Read in the expected "log" output ...
    p.set_extension("log");
    let logname = p.file_name().unwrap().to_owned();
    let expected_log = read_file(&p);

    // ... and the expected XDVI output.
    p.set_extension("xdv");
    let xdvname = p.file_name().unwrap().to_owned();
    let expected_xdv = read_file(&p);


    // MemoryIo layer that will accept the outputs.
    let mut mem = MemoryIo::new(true);

    // Run the engine!
    {
        let mut io = IoStack::new(vec![
            &mut mem,
            &mut tex,
            &mut fmt,
        ]);
        TexEngine::new()
            .process(&mut io, &mut NoopIoEventBackend::new(),
                      &mut NoopStatusBackend::new(), "plain.fmt.gz", &texname).unwrap();
    }

    // Check that log and xdv match expectations.

    let files = mem.files.borrow();

    let observed_log = files.get(&logname).unwrap();
    test_file(&logname, &expected_log, observed_log);

    let observed_xdv = files.get(&xdvname).unwrap();
    test_file(&xdvname, &expected_xdv, observed_xdv);

    if check_synctex {
        p.set_extension("synctex.gz");
        // Gzipped files seem to be platform dependent and so we decompress them first.
        let mut expected_synctex = Vec::new();
        GzDecoder::new(File::open(&p).unwrap()).unwrap()
            .read_to_end(&mut expected_synctex).unwrap();
        let synctexname = p.file_name().unwrap().to_owned();
        let mut observed_synctex = Vec::new();
        GzDecoder::new(&files.get(&synctexname).unwrap()[..]).unwrap()
            .read_to_end(&mut observed_synctex).unwrap();
        test_file(&synctexname, &expected_synctex, &observed_synctex);
    }
}


// Keep these alphabetized.

#[test]
fn md5_of_hello() { do_one("md5_of_hello", false) }

#[test]
fn negative_roman_numeral() { do_one("negative_roman_numeral", false) }

#[test]
fn pdfoutput() { do_one("pdfoutput", false) }

#[test]
fn synctex() { do_one("synctex", true) }

#[test]
fn the_letter_a() { do_one("the_letter_a", false) }
