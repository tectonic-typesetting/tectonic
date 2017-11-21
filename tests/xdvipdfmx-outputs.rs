// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.
extern crate flate2;
#[macro_use] extern crate lazy_static;
extern crate tectonic;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::env;
use std::collections::HashSet;

use tectonic::config::PersistentConfig;
use tectonic::engines::NoopIoEventBackend;
use tectonic::io::{FilesystemIo, GenuineStdoutIo, IoStack, MemoryIo};
use tectonic::io::testing::SingleInputFileIo;
use tectonic::status::NoopStatusBackend;
use tectonic::XdvipdfmxEngine;

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");

lazy_static! {
    static ref LOCK: Mutex<u8> = Mutex::new(0u8);
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


    println!("hex'd observed output:");
    for &byte in observed {
        print!("{:X}", byte);
    }
    println!("\n");

    panic!("difference in {}; contents saved to disk", name.to_string_lossy());
}

fn do_one(stem: &str) {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut p = PathBuf::from(TOP);
    p.push("tests");

    // Prepare the input file.
    p.push("xdvipdfmx-outputs");
    p.push(stem);
    p.set_extension("xdv");

    let xdvname = p.file_name().unwrap().to_str().unwrap().to_owned();
    let mut xdv = SingleInputFileIo::new(&p);

    // Read in the expected "pdf" output ...
    p.set_extension("pdf");
    let pdfname = p.file_name().unwrap().to_owned();
    let expected_pdf = read_file(&p);

    let mut paper_support_files = PathBuf::from(TOP);
    paper_support_files.push("tests");
    paper_support_files.push("xenia");
    let mut fs_paper_support = FilesystemIo::new(&paper_support_files, false, false, HashSet::new());



    // MemoryIo layer that will accept the output.
    let mut mem = MemoryIo::new(true);

    let mut genuine_stdout = GenuineStdoutIo::new();

    // this hits the network for ~10 MB on every execution .__.
    // TODO: keep assets in git?
    let config = PersistentConfig::open(false).unwrap();
    let mut tb =
        config.default_io_provider(&mut NoopStatusBackend::new()).unwrap()
    ;
    // let mut tb = ITarBundle::<HttpITarIoFactory>::new("https://dl.bintray.com/pkgw/tectonic/tl2016extras/2016.0r4/tlextras-2016.0r4.tar");

    // While the xdv and log output is deterministic without setting
    // SOURCE_DATE_EPOCH, xdvipdfmx uses the current time in various places.
    env::set_var("SOURCE_DATE_EPOCH", "1456304492"); // TODO: default to deterministic behaviour

    // Run the engine!
    {
        let mut io = IoStack::new(vec![
            &mut genuine_stdout,
            &mut mem,
            &mut xdv,
            &mut fs_paper_support,
            &mut *tb
        ]);
        XdvipdfmxEngine::new()
            .with_compression(false)
            .process(&mut io, &mut NoopIoEventBackend::new(),
                    &mut NoopStatusBackend::new(), &xdvname, &*pdfname.to_string_lossy()).unwrap();
    }

    // Check that log, xdv and pdf match expectations.

    let files = mem.files.borrow();

    let observed_pdf = files.get(&pdfname).unwrap();
    test_file(&pdfname, &expected_pdf, observed_pdf);
}


// Keep these alphabetized.
/*
#[test]
fn md5_of_hello_pdf() { do_one("md5_of_hello") }

#[test]
fn the_letter_a_pdf() { do_one("the_letter_a") }
*/
#[test]
fn paper_pdf() { do_one("paper") }
