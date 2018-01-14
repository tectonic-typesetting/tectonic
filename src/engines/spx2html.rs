// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! Convert Tectonic’s SPX format to HTML
//!
//! Yay, an engine actually written in pure Rust!

use std::ffi::OsStr;
use std::io::Write;
use tectonic_xdv::{self, FileType, XdvError, XdvEvents, XdvParser};

use errors::{Error, ErrorKind, Result};
use io::{IoProvider, IoStack, OpenResult, OutputHandle};
use status::StatusBackend;
use super::IoEventBackend;


pub struct Spx2HtmlEngine {
}


impl Spx2HtmlEngine {
    pub fn new() -> Spx2HtmlEngine {
        Spx2HtmlEngine {}
    }

    pub fn process(&mut self, io: &mut IoStack, events: &mut IoEventBackend,
                   status: &mut StatusBackend, spx: &str) -> Result<()> {
        let mut input = io.input_open_name(OsStr::new(spx), status).must_exist()?;
        events.input_opened(input.name(), input.origin());

        // FIXME? The engine should probably be responsible for choosing this.
        let outname = {
            let mut s = spx.to_owned();

            if spx.ends_with(".spx") {
                let l = s.len();
                s.split_off(l - 4);
            }

            s.push_str(".html");
            s
        };

        {
            // The error type here is XdvError<tectonic::errors::Error>, so we
            // can't include XdvError as an error-chain type because the type
            // would be recursive! So we must manually handle the various
            // outcomes.
            let state = State::new(outname, io, events, status);

            let (state, _n_bytes) = match XdvParser::process(&mut input, state) {
                Ok(x) => x,
                Err(e) => {
                    return Err(match e {
                        XdvError::Inner(e) => e,
                        XdvError::Io(e) => e.into(),
                        XdvError::Malformed(offset) => {
                            errmsg!("malformed SPX file at byte offset {}", offset)
                        },
                        XdvError::IllegalOpcode(op, offset) => {
                            errmsg!("illegal SPX opcode {} at byte offset {}", op, offset)
                        },
                        XdvError::NeedMoreData => {
                            // This shouldn't leak out to this point, but the enum value is exposed, so.
                            errmsg!("couldn't parse entire operand (?!)")
                        },
                        XdvError::UnexpectedEndOfStream => {
                            errmsg!("unexpected termination of the SPX data")
                        },
                    });
                },
            };

            state.finished();
        }

        let (name, digest_opt) = input.into_name_digest();
        events.input_closed(name, digest_opt);

        Ok(())
    }
}



type XdvResult<T> = tectonic_xdv::Result<T, Error>;

impl From<ErrorKind> for XdvError<Error> {
    fn from(k: ErrorKind) -> Self {
        XdvError::Inner(k.into())
    }
}

impl From<Error> for XdvError<Error> {
    fn from(e: Error) -> Self {
        XdvError::Inner(e)
    }
}


struct State<'a, 'b: 'a> {
    outname: String,
    io: &'a mut IoStack<'b>,
    events: &'a mut IoEventBackend,
    status: &'a mut StatusBackend,
    cur_output: Option<OutputHandle>,
    warned_lost_chars: bool,
    buf: Vec<u8>,
}

impl<'a, 'b: 'a> State<'a, 'b> {
    pub fn new(outname: String, io: &'a mut IoStack<'b>,
               events: &'a mut IoEventBackend, status: &'a mut StatusBackend) -> Self {
        Self {
            outname: outname,
            io: io,
            events: events,
            status: status,
            cur_output: None,
            warned_lost_chars: false,
            buf: Vec::new(),
        }
    }
}

#[inline(always)]
fn as_printable_ascii(c: i32) -> Option<u8> {
    if c > 0x20 && c < 0x7F {
        Some(c as u8)
    } else {
        None
    }
}

impl<'a, 'b: 'a> State<'a, 'b> {
    pub fn finished(self) {
        if let Some(oh) = self.cur_output {
            let (name, digest) = oh.into_name_digest();
            self.events.output_closed(name, digest);
        }
    }
}

impl<'a, 'b: 'a> XdvEvents for State<'a, 'b> {
    type Error = Error;

    fn handle_header(&mut self, filetype: FileType, _comment: &[u8]) -> XdvResult<()> {
        if filetype != FileType::Spx {
            return Err(errmsg!("file should be SPX format but got {}", filetype));
        }

        self.cur_output = Some(
            match self.io.output_open_name(OsStr::new(&self.outname)) {
                OpenResult::Ok(h) => h,
                OpenResult::NotAvailable => {
                    return Err(errmsg!("no way to write output file \"{}\"", self.outname));
                },
                OpenResult::Err(e) => {
                    return Err(e.into());
                },
            }
        );

        self.events.output_opened(OsStr::new(&self.outname));

        Ok(())
    }

    fn handle_char_run(&mut self, chars: &[i32]) -> XdvResult<()> {
        let dest = match self.cur_output {
            Some(ref mut h) => h,
            None => {
                if !self.warned_lost_chars {
                    tt_warning!(self.status, "losing characters in SPX file: no current output");
                    self.warned_lost_chars = true;
                }

                return Ok(());
            }
        };

        self.buf.clear();

        for c in chars.iter() {
            if let Some(b) = as_printable_ascii(*c) {
                self.buf.push(b);
            }
        }

        if self.buf.len() > 0 {
            self.buf.push(0x0a); // newline
            dest.write(&self.buf)?;
        }

        Ok(())
    }
}
