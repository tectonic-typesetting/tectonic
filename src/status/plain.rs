use std::fmt::Arguments;

use super::{ChatterLevel, MessageKind, StatusBackend};
use crate::errors::Error;
use std::io::{self, Write};

pub struct PlainStatusBackend {
    chatter: ChatterLevel,
}

impl PlainStatusBackend {
    pub fn new(chatter: ChatterLevel) -> Self {
        PlainStatusBackend { chatter }
    }
}

impl StatusBackend for PlainStatusBackend {
    fn report(&mut self, kind: MessageKind, args: Arguments, err: Option<&Error>) {
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
            for item in e.iter() {
                eprintln!("caused by: {}", item);
            }
            if let Some(backtrace) = e.backtrace() {
                eprintln!("debugging: backtrace follows:");
                eprintln!("{:?}", backtrace);
            }
        }
    }

    fn note_highlighted(&mut self, before: &str, highlighted: &str, after: &str) {
        if self.chatter > ChatterLevel::Minimal {
            self.report(
                MessageKind::Note,
                format_args!("{}{}{}", before, highlighted, after),
                None,
            );
        }
    }

    fn dump_to_stderr(&mut self, output: &[u8]) {
        io::stderr()
            .write_all(output)
            .expect("write to stderr failed");
    }
}
