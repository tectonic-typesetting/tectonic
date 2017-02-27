// src/status/termcolor.rs -- 'termcolor' based status backend
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

// TODO: make this module a feature that can be disable if the user doesn't want to
// link with termcolor

use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::StatusBackend;


pub struct TermcolorStatusBackend {
    stdout: StandardStream,
    stderr: StandardStream,
    warning_spec: ColorSpec,
    error_spec: ColorSpec,
}


impl TermcolorStatusBackend {
    pub fn new() -> TermcolorStatusBackend {
        let mut warning_spec = ColorSpec::new();
        warning_spec.set_fg(Some(Color::Yellow)).set_bold(true);

        let mut error_spec = ColorSpec::new();
        error_spec.set_fg(Some(Color::Red)).set_bold(true);

        TermcolorStatusBackend {
            stdout: StandardStream::stdout(ColorChoice::Auto),
            stderr: StandardStream::stderr(ColorChoice::Auto),
            warning_spec: warning_spec,
            error_spec: error_spec,
        }
    }

    // Helpers for the CLI program that aren't needed by the internal bits,
    // so we put them here to minimize the cross-section of the StatusBackend
    // trait.

    pub fn error_styled(&mut self, message: &str) {
        self.stderr.set_color(&self.error_spec).expect("write to stderr failed");
        writeln!(self.stderr, "{}", message).expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
    }

    pub fn dump_to_stderr(&mut self, output: &[u8]) {
        self.stderr.write_all(output).expect("write to stderr failed");
    }
}


impl StatusBackend for TermcolorStatusBackend {
    fn info(&mut self, message: &str) {
        writeln!(self.stdout, "{}", message).expect("write to stdout failed");
    }

    fn warning(&mut self, message: &str) {
        self.stderr.set_color(&self.warning_spec).expect("write to stderr failed");
        write!(self.stderr, "warning:").expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
        writeln!(self.stderr, " {}", message).expect("write to stderr failed");
    }

    fn error(&mut self, message: &str) {
        self.stderr.set_color(&self.error_spec).expect("write to stderr failed");
        write!(self.stderr, "error:").expect("write to stderr failed");
        self.stderr.reset().expect("write to stderr failed");
        writeln!(self.stderr, " {}", message).expect("write to stderr failed");
    }
}
