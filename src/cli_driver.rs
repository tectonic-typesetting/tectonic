// src/cli_driver.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate tectonic;

use clap::{Arg, App};
use std::fs::File;
use std::io::{stderr, Write};
use std::path::Path;

use tectonic::bundle::Bundle;
use tectonic::errors::{Result, ResultExt};
use tectonic::io::{FilesystemIO, GenuineStdoutIO, IOProvider, IOStack, MemoryIO};
use tectonic::{Engine, TeXResult};


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
        .arg(Arg::with_name("keeplog")
             .long("keeplog")
             .help("Keep the \"<INPUT>.log\" file generated during processing."))
        .arg(Arg::with_name("print_stdout")
             .long("print")
             .short("p")
             .help("Print the engine's chatter during processing."))
        .arg(Arg::with_name("INPUT")
             .help("The file to process.")
             .required(true)
             .index(1))
        .get_matches ();

    let format = matches.value_of("format").unwrap();
    let outfmt = matches.value_of("outfmt").unwrap();
    let input = matches.value_of("INPUT").unwrap();

    // Set up and run the engine; we need to nest a bit to get mutable borrow
    // lifetimes right.

    let mut gsi;
    let mut mem = MemoryIO::new(true);
    let mut fsi = FilesystemIO::new(Path::new(""), false);
    let mut bundle;

    let result = {
        let mut providers: Vec<&mut IOProvider> = Vec::new();

        if matches.is_present("print_stdout") {
            gsi = GenuineStdoutIO::new();
            providers.push(&mut gsi);
        }

        providers.push(&mut mem);
        providers.push(&mut fsi);

        if let Some(btext) = matches.value_of("bundle") {
            bundle = Bundle::<File>::open(Path::new(&btext)).chain_err(|| "error opening bundle")?;
            providers.push(&mut bundle);
        }

        let io = IOStack::new(providers);

        // Ready to go.

        let mut engine = Engine::new (io);
        engine.set_halt_on_error_mode (true);
        engine.set_output_format (outfmt);
        engine.process (format, input)
    };

    // How did we do?

    match result {
        Ok(TeXResult::Spotless) => {},
        Ok(TeXResult::Warnings) => {
            println!("NOTE: warnings were issued by the TeX engine; use --print and/or --keeplog for details.");
        },
        Ok(TeXResult::Errors) => {
            println!("NOTE: errors were issued by the TeX engine, but were ignored; \
                      use --print and/or --keeplog for details.");
        },
        Err(e) => {
            let mut s = &mut stderr();

            if let Some(output) = mem.files.borrow().get(mem.stdout_key()) {
                writeln!(s, "NOTE: the engine reported an error; its output follows:\n").expect("stderr failed");
                writeln!(s, "========================================").expect("stderr failed");
                s.write_all(output).expect("stderr failed");
                writeln!(s, "========================================").expect("stderr failed");
                writeln!(s, "").expect("stderr failed");
            }

            return Err(e);
        }
    }

    // If we got this far, then we did OK. For now, write out the output files
    // of interest.

    for (name, contents) in &*mem.files.borrow() {
        let sname = name.to_string_lossy();

        if name == mem.stdout_key() {
            continue;
        }

        if sname.ends_with(".log") && !matches.is_present("keeplog") {
            continue;
        }

        if contents.len() == 0 {
            println!("Not writing {}: it would be empty.", sname);
            continue;
        }

        println!("Writing {} ({} bytes).", sname, contents.len());

        let mut f = File::create(Path::new(name))?;
        f.write_all(contents)?;
    }

    Ok(0)
}

quick_main!(run);
