// src/compat.rs -- Tectonic driver program, with output behavior compatible with web2c XeTeX.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate tectonic;

use clap::{Arg, App};
use std::fs::File;
use std::path::Path;

use tectonic::bundle::Bundle;
use tectonic::errors::{Result, ResultExt};
use tectonic::io::{FilesystemIO, GenuineStdoutIO, IOProvider, IOStack};
use tectonic::Engine;


fn run() -> Result<i32> {
    let matches = App::new("Tectonic")
        .version("0.1")
        .about("Process a (La)TeX document.")
        .arg(Arg::with_name("format")
             .long("format")
             .value_name("PATH")
             .help("The \"format\" used to initialize the engine")
             .default_value("xelatex.fmt"))
        .arg(Arg::with_name("bundle")
             .long("bundle")
             .short("b")
             .value_name("PATH")
             .help("The bundle file containing LaTeX resource files")
             .takes_value(true))
        .arg(Arg::with_name("outfmt")
             .long("outfmt")
             .value_name("FORMAT")
             .help("The kind of output to generate")
             .possible_values(&["pdf", "xdv"])
             .default_value("pdf"))
        .arg(Arg::with_name("INPUT")
             .help("The file to process.")
             .required(true)
             .index(1))
        .get_matches ();

    let format = matches.value_of("format").unwrap();
    let outfmt = matches.value_of("outfmt").unwrap();
    let input = matches.value_of("INPUT").unwrap();

    // Create the IO stack that the engine will use.

    let mut providers: Vec<Box<IOProvider>> = vec![
        Box::new(GenuineStdoutIO::new()),
        Box::new(FilesystemIO::new(Path::new(""), true)),
    ];

    if let Some(btext) = matches.value_of("bundle") {
        let b = Bundle::<File>::open(Path::new(&btext)).chain_err(|| "error opening bundle")?;
        providers.push(Box::new(b));
    }

    let io = IOStack::new(providers);

    // Ready to go.

    let mut e = Engine::new (io);
    e.set_output_format (outfmt);
    e.process (format, input)?;
    Ok(0)
}

quick_main!(run);
