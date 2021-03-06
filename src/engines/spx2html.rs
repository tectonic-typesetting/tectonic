// Copyright 2018-2021 the Tectonic Project
// Licensed under the MIT License.

//! Convert Tectonic’s SPX format to HTML
//!
//! Yay, an engine actually written in pure Rust!

use std::io::Write;
use tectonic_bridge_core::DriverHooks;
use tectonic_xdv::{FileType, XdvEvents, XdvParser};

use crate::errors::{Error, Result};
use crate::io::{OpenResult, OutputHandle};
use crate::status::StatusBackend;
use crate::{errmsg, tt_warning};

#[derive(Default)]
pub struct Spx2HtmlEngine {}

impl Spx2HtmlEngine {
    pub fn new() -> Spx2HtmlEngine {
        Default::default()
    }

    pub fn process(
        &mut self,
        hooks: &mut dyn DriverHooks,
        status: &mut dyn StatusBackend,
        spx: &str,
    ) -> Result<()> {
        let mut input = hooks.io().input_open_name(spx, status).must_exist()?;

        // FIXME? The engine should probably be responsible for choosing this.
        let outname = {
            let mut s = spx.to_owned();

            if spx.ends_with(".spx") {
                let l = s.len();
                s.truncate(l - 4);
            }

            s.push_str(".html");
            s
        };

        {
            let state = State::new(outname, hooks, status);
            let (state, _n_bytes) = XdvParser::process(&mut input, state)?;
            state.finished();
        }

        let (name, digest_opt) = input.into_name_digest();
        hooks.event_input_closed(name, digest_opt, status);
        Ok(())
    }
}

struct State<'a> {
    outname: String,
    hooks: &'a mut dyn DriverHooks,
    status: &'a mut dyn StatusBackend,
    cur_output: Option<OutputHandle>,
    warned_lost_chars: bool,
    buf: Vec<u8>,
}

impl<'a> State<'a> {
    pub fn new(
        outname: String,
        hooks: &'a mut dyn DriverHooks,
        status: &'a mut dyn StatusBackend,
    ) -> Self {
        Self {
            outname,
            hooks,
            status,
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

impl<'a> State<'a> {
    pub fn finished(self) {
        if let Some(oh) = self.cur_output {
            let (name, digest) = oh.into_name_digest();
            self.hooks.event_output_closed(name, digest, self.status);
        }
    }
}

impl<'a> XdvEvents for State<'a> {
    type Error = Error;

    fn handle_header(&mut self, filetype: FileType, _comment: &[u8]) -> Result<()> {
        if filetype != FileType::Spx {
            return Err(errmsg!("file should be SPX format but got {}", filetype));
        }

        self.cur_output = Some(match self.hooks.io().output_open_name(&self.outname) {
            OpenResult::Ok(h) => h,
            OpenResult::NotAvailable => {
                return Err(errmsg!("no way to write output file \"{}\"", self.outname));
            }
            OpenResult::Err(e) => {
                return Err(e.into());
            }
        });

        Ok(())
    }

    fn handle_char_run(&mut self, chars: &[i32]) -> Result<()> {
        let dest = match self.cur_output {
            Some(ref mut h) => h,
            None => {
                if !self.warned_lost_chars {
                    tt_warning!(
                        self.status,
                        "losing characters in SPX file: no current output"
                    );
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

        if !self.buf.is_empty() {
            self.buf.push(0x0a); // newline
            dest.write_all(&self.buf)?;
        }

        Ok(())
    }
}
