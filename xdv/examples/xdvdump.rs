// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! Parse an XDV/SPX file and dump some stats about its contents.

#[macro_use] extern crate clap;
extern crate tectonic_xdv;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::process;
use std::str;
use tectonic_xdv::{FileType, Result};


struct Stats {
}

impl Stats {
    pub fn new() -> Self {
        Stats {}
    }
}

impl tectonic_xdv::XdvEvents for Stats {
    type Error = String;

    fn handle_header(&mut self, filetype: FileType, comment: &[u8]) -> Result<(), Self::Error> {
        println!("file type: {}", filetype);

        match str::from_utf8(comment) {
            Ok(s) => {
                println!("comment: {}", s);
            },
            Err(e) => {
                println!("cannot parse comment: {}", e);
            }
        };

        Ok(())
    }

    fn handle_begin_page(&mut self, counters: &[i32], previous_bop: i32) -> Result<(), Self::Error> {
        println!("new page: [{} {} {} {} {} {} {} {} {} {}] {}",
                 counters[0], counters[1], counters[2], counters[3], counters[4],
                 counters[5], counters[6], counters[7], counters[8], counters[9],
                 previous_bop);
        Ok(())
    }

    fn handle_special(&mut self, contents: &[u8]) -> Result<(), Self::Error> {
        match str::from_utf8(contents) {
            Ok(s) => {
                println!("special: {}", s);
            },
            Err(e) => {
                println!("cannot UTF8-parse special: {}", e);
            }
        };

        Ok(())
    }
}


fn main() {
    let matches = App::new("xdvdump")
        .version(crate_version!())
        .about("Parse an XDV or SPX file and report some stats about its contents")
        .arg(Arg::with_name("PATH")
             .help("The path to the XDV or SPX file")
             .required(true)
             .index(1))
        .get_matches();

    let path = matches.value_of_os("PATH").unwrap();

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: could not open \"{}\": {}", path.to_string_lossy(), e);
            process::exit(1);
        }
    };

    let mut parser = tectonic_xdv::XdvParser::new(Stats::new());
    let mut buf = Vec::with_capacity(4096);
    unsafe { buf.set_len(4096); }
    let mut n_saved_bytes = 0;

    loop {
        let n_read = match file.read(&mut buf[n_saved_bytes..]) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("error: failed to read data from \"{}\": {}", path.to_string_lossy(), e);
                process::exit(1);
            }
        };

        let n_in_buffer = n_saved_bytes + n_read;

        let n_consumed = match parser.parse(&buf[..n_in_buffer]) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("error: couldn't parse \"{}\": {}", path.to_string_lossy(), e);
                process::exit(1);
            }
        };

        n_saved_bytes = n_in_buffer - n_consumed;

        if n_consumed != 0 && n_saved_bytes != 0 {
            // The current parse did not consume the full buffer, so we must
            // copy the un-parsed bytes to its beginning. The next time that
            // we read data, we will append to these already-read bytes so
            // that the parser gets a nice contiguous set of bytes to look at.
            // The copy may involve overlapping memory regions (imagine we
            // read 4096 bytes but only consume 1) so we have to get unsafe.
            use std::ptr;
            let ptr = buf.as_mut_ptr();
            unsafe { ptr::copy(ptr.offset(n_consumed as isize), ptr, n_saved_bytes); }
        }

        if n_in_buffer != 0 && n_consumed == 0 {
            // We're going to need a bigger buffer in order to handle whatever
            // we're reading. Let's double it.
            let len = buf.len();
            buf.reserve(len);
            unsafe { buf.set_len(2 * len); }
        }

        if n_read == 0 {
            break;
        }
    }

    let n_bytes = parser.current_offset();

    let _stats = match parser.finish() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("error: couldn't parse \"{}\": {}", path.to_string_lossy(), e);
            process::exit(1);
        }
    };

    println!("{} bytes parsed.", n_bytes);
}
