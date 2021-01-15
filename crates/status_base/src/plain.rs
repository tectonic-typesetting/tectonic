// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

//! A basic status-reporting backend that prints messages via stdio.

use std::{
    fmt::Arguments,
    io::{self, Write},
};
use tectonic_errors::Error;

use super::{ChatterLevel, MessageKind, StatusBackend};

/// A basic status-reporting backend that prints messages via stdio.
#[derive(Clone, Debug, Default)]
pub struct PlainStatusBackend {
    chatter: ChatterLevel,
}

impl PlainStatusBackend {
    /// Create a new backend with the specified chatter level.
    pub fn new(chatter: ChatterLevel) -> Self {
        PlainStatusBackend { chatter }
    }
}

impl StatusBackend for PlainStatusBackend {
    fn report(&mut self, kind: MessageKind, args: Arguments, err: Option<&Error>) {
        if self.chatter.suppress_message(kind) {
            return;
        }

        let prefix = match kind {
            MessageKind::Note => "note:",
            MessageKind::Warning => "warning:",
            MessageKind::Error => "error:",
        };

        if kind == MessageKind::Note {
            println!("{} {}", prefix, args);
        } else {
            eprintln!("{} {}", prefix, args);
        }

        if let Some(e) = err {
            for item in e.chain() {
                eprintln!("caused by: {}", item);
            }
        }
    }

    fn report_error(&mut self, err: &Error) {
        let mut prefix = "error";

        for item in err.chain() {
            eprintln!("{}: {}", prefix, item);
            prefix = "caused by";
        }
    }

    fn note_highlighted(&mut self, before: &str, highlighted: &str, after: &str) {
        self.report(
            MessageKind::Note,
            format_args!("{}{}{}", before, highlighted, after),
            None,
        );
    }

    fn dump_error_logs(&mut self, output: &[u8]) {
        eprintln!(
            "==============================================================================="
        );

        io::stderr()
            .write_all(output)
            .expect("write to stderr failed");

        eprintln!(
            "==============================================================================="
        );
    }
}
