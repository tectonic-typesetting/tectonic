// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! Parse an XDV/SPX file and dump some stats about its contents.

#[macro_use]
extern crate clap;
extern crate tectonic_xdv;

use clap::{App, Arg};
use std::fmt::{Display, Error as FmtError, Formatter};
use std::fs::File;
use std::io;
use std::process;
use std::str;
use tectonic_xdv::{FileType, XdvError};

/// We'd like to use String as our error type, but we also would like to
/// use the `XdvParser::process()` function, which when imposes the requirement
/// that String: From<IoError>, which we can't satisfy. So, we have to create
/// a wrapper type.
///
/// We can't implement `From<T: Display> for Error` because `Error: Display`,
/// which conflicts with the language's blanket `From<T> for T`
/// implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error(format!("{}", e)) // note: weirdly, can't use `Self` on this line
    }
}

impl From<XdvError> for Error {
    fn from(e: XdvError) -> Self {
        Error(format!("{}", e))
    }
}

struct Stats {}

impl Stats {
    pub fn new() -> Self {
        Stats {}
    }
}

impl tectonic_xdv::XdvEvents for Stats {
    type Error = Error;

    fn handle_header(&mut self, filetype: FileType, comment: &[u8]) -> Result<(), Self::Error> {
        println!("file type: {}", filetype);

        match str::from_utf8(comment) {
            Ok(s) => {
                println!("comment: {}", s);
            }
            Err(e) => {
                println!("cannot parse comment: {}", e);
            }
        };

        Ok(())
    }

    fn handle_begin_page(
        &mut self,
        counters: &[i32],
        previous_bop: i32,
    ) -> Result<(), Self::Error> {
        println!(
            "new page: [{} {} {} {} {} {} {} {} {} {}] {}",
            counters[0],
            counters[1],
            counters[2],
            counters[3],
            counters[4],
            counters[5],
            counters[6],
            counters[7],
            counters[8],
            counters[9],
            previous_bop
        );
        Ok(())
    }

    fn handle_special(&mut self, contents: &[u8]) -> Result<(), Self::Error> {
        match str::from_utf8(contents) {
            Ok(s) => {
                println!("special: {}", s);
            }
            Err(e) => {
                println!("cannot UTF8-parse special: {}", e);
            }
        };

        Ok(())
    }

    fn handle_char_run(&mut self, chars: &[i32]) -> Result<(), Self::Error> {
        let all_ascii_printable = chars.iter().all(|c| *c > 0x20 && *c < 0x7F);
        println!(
            "chars: {:?} all_ascii_printable={:?}",
            chars, all_ascii_printable
        );
        Ok(())
    }
}

fn main() {
    let matches = App::new("xdvdump")
        .version(crate_version!())
        .about("Parse an XDV or SPX file and report some stats about its contents")
        .arg(
            Arg::with_name("PATH")
                .help("The path to the XDV or SPX file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let path = matches.value_of_os("PATH").unwrap();

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "error: could not open \"{}\": {}",
                path.to_string_lossy(),
                e
            );
            process::exit(1);
        }
    };

    let (_stats, n_bytes) = match tectonic_xdv::XdvParser::process(file, Stats::new()) {
        Ok(x) => x,
        Err(e) => {
            eprintln!(
                "error: failed to parse \"{}\": {}",
                path.to_string_lossy(),
                e
            );
            process::exit(1);
        }
    };

    println!("{} bytes parsed.", n_bytes);
}
