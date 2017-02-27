// src/status/termcolor.rs -- 'termcolor' based status backend
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

// TODO: make this module a feature that can be disable if the user doesn't want to
// link with termcolor

use std::fmt::{Arguments, Display};
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::{ChatterLevel, StatusBackend};


pub struct TermcolorStatusBackend {
    chatter: ChatterLevel,
    stdout: StandardStream,
    stderr: StandardStream,
    note_spec: ColorSpec,
    highlight_spec: ColorSpec,
    warning_spec: ColorSpec,
    error_spec: ColorSpec,
}


impl TermcolorStatusBackend {
    pub fn new(chatter: ChatterLevel) -> TermcolorStatusBackend {
        let mut note_spec = ColorSpec::new();
        note_spec.set_fg(Some(Color::Green)).set_bold(true);

        let mut highlight_spec = ColorSpec::new();
        highlight_spec.set_bold(true);

        let mut warning_spec = ColorSpec::new();
        warning_spec.set_fg(Some(Color::Yellow)).set_bold(true);

        let mut error_spec = ColorSpec::new();
        error_spec.set_fg(Some(Color::Red)).set_bold(true);

        TermcolorStatusBackend {
            chatter: chatter,
            stdout: StandardStream::stdout(ColorChoice::Auto),
            stderr: StandardStream::stderr(ColorChoice::Auto),
            note_spec: note_spec,
            highlight_spec: highlight_spec,
            warning_spec: warning_spec,
            error_spec: error_spec,
        }
    }

    // Helpers for the CLI program that aren't needed by the internal bits,
    // so we put them here to minimize the cross-section of the StatusBackend
    // trait.

    pub fn note_styled(&mut self, args: Arguments) {
        if self.chatter > ChatterLevel::Minimal {
            writeln!(self.stdout, "{}", args).expect("write to stdout failed");
        }
    }

    pub fn note_highlighted(&mut self, before: &str, highlighted: &str, after: &str) {
        if self.chatter > ChatterLevel::Minimal {
            write!(self.stdout, "{}", before).expect("write to stdout failed");
            self.stdout.set_color(&self.highlight_spec).expect("write to stdout failed");
            write!(self.stdout, "{}", highlighted).expect("write to stdout failed");
            self.stdout.reset().expect("write to stdout failed");
            writeln!(self.stdout, "{}", after).expect("write to stdout failed");
        }
    }

    pub fn error_styled(&mut self, args: Arguments) {
        self.stderr.set_color(&self.error_spec).expect("write to stderr failed");
        writeln!(self.stderr, "{}", args).expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
    }

    pub fn caused_by<T: Display>(&mut self, item: T) {
        self.stderr.set_color(&self.error_spec).expect("write to stderr failed");
        write!(self.stderr, "caused by:").expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
        writeln!(self.stderr, " {}", item).expect("write to stderr failed");
    }

    pub fn dump_to_stderr(&mut self, output: &[u8]) {
        self.stderr.write_all(output).expect("write to stderr failed");
    }
}


#[macro_export]
macro_rules! tt_note_styled {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.note_styled(format_args!($( $fmt_args ),*))
    };
}

#[macro_export]
macro_rules! tt_error_styled {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.error_styled(format_args!($( $fmt_args ),*))
    };
}


impl StatusBackend for TermcolorStatusBackend {
    fn note(&mut self, args: Arguments) {
        if self.chatter > ChatterLevel::Minimal {
            self.stdout.set_color(&self.note_spec).expect("write to stdout failed");
            write!(self.stdout, "note:").expect("write to stdout failed");
            self.stdout.reset().expect("write to stdout failed");
            writeln!(self.stdout, " {}", args).expect("write to stdout failed");
        }
    }

    fn warning(&mut self, args: Arguments) {
        self.stderr.set_color(&self.warning_spec).expect("write to stderr failed");
        write!(self.stderr, "warning:").expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
        writeln!(self.stderr, " {}", args).expect("write to stderr failed");
    }

    fn error(&mut self, args: Arguments) {
        self.stderr.set_color(&self.error_spec).expect("write to stderr failed");
        write!(self.stderr, "error:").expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
        writeln!(self.stderr, " {}", args).expect("write to stderr failed");
    }
}
