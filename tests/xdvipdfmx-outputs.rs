// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.
extern crate flate2;
#[macro_use] extern crate lazy_static;
extern crate tectonic;

use std::sync::Mutex;
use std::env;

use tectonic::status::NoopStatusBackend;
use tectonic::XdvipdfmxEngine;
use tectonic::engines::NoopIoEventBackend;

mod util;
use util::*;

lazy_static! {
    static ref LOCK: Mutex<()> = Mutex::new(());
}

fn do_one(stem: &str) {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut xdv_path = test_path(&["xdvipdfmx-outputs", stem]);
    xdv_path.set_extension("xdv");

    let mut pdf_path = xdv_path.clone();
    pdf_path.set_extension("pdf");

    let pdfname = pdf_path.file_name().unwrap().to_owned();
    let expected_pdf = read_file(&pdf_path);

    let xdvname = xdv_path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut tb = TestBundle::new()
        .with_file(&xdv_path)
        .with_folder(&test_path(&["xenia"]))
        .with_static_bundle();

    // While the xdv and log output is deterministic without setting
    // SOURCE_DATE_EPOCH, xdvipdfmx uses the current date in various places.
    env::set_var("SOURCE_DATE_EPOCH", "1456304492"); // TODO: default to deterministic behaviour

    // Run the engine!
    {
        XdvipdfmxEngine::new()
            .with_compression(false)
            .process(&mut tb.as_iostack(), &mut NoopIoEventBackend::new(),
                     &mut NoopStatusBackend::new(), &xdvname, &*pdfname.to_string_lossy())
            .unwrap();
    }

    // Check that log, xdv and pdf match expectations.

    let files = tb.mem_io.files.borrow();

    let observed_pdf = files.get(&pdfname).unwrap();
    assert_file_eq(&pdfname, &expected_pdf, observed_pdf);
}


#[test]
fn md5_of_hello_pdf() {
    do_one("md5_of_hello")
}

#[test]
fn the_letter_a_pdf() {
    do_one("the_letter_a")
}

#[test]
#[cfg(not(target_os = "macos"))]
fn paper_pdf() {
    do_one("paper")
}
