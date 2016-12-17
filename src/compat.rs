// src/compat.rs -- Tectonic driver program, with output behavior compatible with web2c XeTeX.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

extern crate clap;
extern crate tectonic;

use clap::{Arg, App};
use std::path::Path;
use std::process;
use tectonic::Engine;

fn main() {
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

    let mut e = Engine::new ();
    e.set_output_format (outfmt);

    if let Some(btext) = matches.value_of("bundle") {
        e.use_bundle(Path::new(&btext)).unwrap ();
    }

    if let Some(msg) = e.process (format, input) {
        println!("error: {}", msg);
        process::exit(1);
    }
}
