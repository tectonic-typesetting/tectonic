// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Decode a format file.

use clap::{crate_version, App, Arg, ArgMatches};
use std::{fs::File, io::Read, process};
use tectonic_errors::prelude::*;
use tectonic_xetex_format::format::Format;

fn inner(matches: ArgMatches) -> Result<()> {
    let path = matches.value_of_os("PATH").unwrap();
    let mut file = File::open(&path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let _fmt = Format::parse(&data[..])?;
    println!("ok!");
    Ok(())
}

fn main() {
    let matches = App::new("decode")
        .version(crate_version!())
        .about("Decode a format file")
        .arg(
            Arg::with_name("PATH")
                .help("The path to the format file")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Err(e) = inner(matches) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
