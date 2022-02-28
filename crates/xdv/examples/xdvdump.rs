// Copyright 2018-2021 the Tectonic Project
// Licensed under the MIT License.

//! Parse an XDV/SPX file and dump some stats about its contents.

use clap::{crate_version, App, Arg};
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

    fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: i32,
        size: i32,
        face_index: u32,
        color_rgba: Option<u32>,
        extend: Option<u32>,
        slant: Option<u32>,
        embolden: Option<u32>,
    ) -> Result<(), Self::Error> {
        println!(
            "define native font: `{}` num={} size={} faceIndex={} color={:?} extend={:?} slant={:?} embolden={:?}",
            name, font_num, size, face_index, color_rgba, extend, slant, embolden
        );
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

    fn handle_special(&mut self, x: i32, y: i32, contents: &[u8]) -> Result<(), Self::Error> {
        match str::from_utf8(contents) {
            Ok(s) => {
                println!("special: {} (@ {},{})", s, x, y);
            }
            Err(e) => {
                println!("cannot UTF8-parse special: {}", e);
            }
        };

        Ok(())
    }

    fn handle_char_run(&mut self, font_num: i32, chars: &[i32]) -> Result<(), Self::Error> {
        let all_ascii_printable = chars.iter().all(|c| *c > 0x20 && *c < 0x7F);
        println!(
            "chars font={}: {:?} all_ascii_printable={:?}",
            font_num, chars, all_ascii_printable
        );
        Ok(())
    }

    fn handle_glyph_run(
        &mut self,
        font_num: i32,
        glyphs: &[u16],
        x: &[i32],
        y: &[i32],
    ) -> Result<(), Self::Error> {
        println!("glyphs font={}: {:?} (@ {:?}, {:?}", font_num, glyphs, x, y);
        Ok(())
    }

    fn handle_rule(&mut self, x: i32, y: i32, height: i32, width: i32) -> Result<(), Self::Error> {
        println!("rule W={} H={} @ {:?}, {:?}", width, height, x, y);
        Ok(())
    }
}

fn main() {
    let matches = App::new("xdvdump")
        .version(crate_version!())
        .about("Parse an XDV or SPX file and report some stats about its contents")
        .arg(
            Arg::with_name("seek")
                .long("seek")
                .help("Seek around the file, parsing the trailers first"),
        )
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

    if matches.is_present("seek") {
        match tectonic_xdv::XdvParser::process_with_seeks(file, Stats::new()) {
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

        println!("Finished parsing the file.");
    } else {
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
}
