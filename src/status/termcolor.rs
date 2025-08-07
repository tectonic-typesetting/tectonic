// src/status/termcolor.rs -- 'termcolor' based status backend
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

// TODO: make this module a feature that can be disabled if the user doesn't want to
//! Status backend that emits colorized errors to the terminal.

use std::fmt::Arguments;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use tectonic_errors::Error;

use super::{ChatterLevel, MessageKind, StatusBackend};

/// Status backend based on `termcolor` that emits compile errors and note with terminal colors.
pub struct TermcolorStatusBackend {
    chatter: ChatterLevel,
    always_stderr: bool,
    stdout: StandardStream,
    stderr: StandardStream,
    note_spec: ColorSpec,
    highlight_spec: ColorSpec,
    warning_spec: ColorSpec,
    error_spec: ColorSpec,
}

impl TermcolorStatusBackend {
    /// Create a new instance of this backend with default colorization.
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
            chatter,
            always_stderr: false,
            stdout: StandardStream::stdout(ColorChoice::Auto),
            stderr: StandardStream::stderr(ColorChoice::Auto),
            note_spec,
            highlight_spec,
            warning_spec,
            error_spec,
        }
    }

    /// Set whether non-error messages such as notes should be sent to stderr or stdout.
    pub fn always_stderr(&mut self, setting: bool) -> &mut Self {
        self.always_stderr = setting;
        self
    }

    fn styled<F>(&mut self, kind: MessageKind, f: F)
    where
        F: FnOnce(&mut StandardStream),
    {
        if kind == MessageKind::Note && self.chatter <= ChatterLevel::Minimal {
            return;
        }

        let (spec, stream) = match kind {
            MessageKind::Note => {
                if self.always_stderr {
                    (&self.note_spec, &mut self.stderr)
                } else {
                    (&self.note_spec, &mut self.stdout)
                }
            }
            MessageKind::Warning => (&self.warning_spec, &mut self.stderr),
            MessageKind::Error => (&self.error_spec, &mut self.stderr),
        };

        stream.set_color(spec).expect("failed to set color");
        f(stream);
        stream.reset().expect("failed to clear color");
    }

    fn with_stream<F>(&mut self, kind: MessageKind, f: F)
    where
        F: FnOnce(&mut StandardStream),
    {
        if kind == MessageKind::Note && self.chatter <= ChatterLevel::Minimal {
            return;
        }

        let stream = match kind {
            MessageKind::Note => {
                if self.always_stderr {
                    &mut self.stderr
                } else {
                    &mut self.stdout
                }
            }
            MessageKind::Warning => &mut self.stderr,
            MessageKind::Error => &mut self.stderr,
        };

        f(stream);
    }

    fn generic_message(&mut self, kind: MessageKind, prefix: Option<&str>, args: Arguments) {
        let text = match prefix {
            Some(s) => s,
            None => match kind {
                MessageKind::Note => "note:",
                MessageKind::Warning => "warning:",
                MessageKind::Error => "error:",
            },
        };

        self.styled(kind, |s| {
            write!(s, "{text}").expect("failed to write to standard stream");
        });
        self.with_stream(kind, |s| {
            writeln!(s, " {args}").expect("failed to write to standard stream");
        });
    }

    // Helpers for the CLI program that aren't needed by the internal bits,
    // so we put them here to minimize the cross-section of the StatusBackend
    // trait.

    /// Write the result of `fmt_args!` as a colorized note.
    pub fn note_styled(&mut self, args: Arguments) {
        if self.chatter > ChatterLevel::Minimal {
            if self.always_stderr {
                writeln!(self.stderr, "{args}").expect("write to stderr failed");
            } else {
                writeln!(self.stdout, "{args}").expect("write to stdout failed");
            }
        }
    }

    /// Write the results of `fmt_args!` as a colorized error.
    pub fn error_styled(&mut self, args: Arguments) {
        self.styled(MessageKind::Error, |s| {
            writeln!(s, "{args}").expect("write to stderr failed");
        });
    }

    /// Write an [`Error`] to output directly.
    pub fn bare_error(&mut self, err: &Error) {
        let mut prefix = "error:";

        for item in err.chain() {
            self.generic_message(MessageKind::Error, Some(prefix), format_args!("{item}"));
            prefix = "caused by:";
        }
    }
}

/// Show formatted text to the user, styled as an error message.
///
/// On the console, this will normally cause the printed text to show up in
/// bright red.
#[macro_export]
macro_rules! tt_error_styled {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.error_styled(format_args!($( $fmt_args ),*))
    };
}

impl StatusBackend for TermcolorStatusBackend {
    fn report(&mut self, kind: MessageKind, args: Arguments, err: Option<&Error>) {
        self.generic_message(kind, None, args);

        if let Some(e) = err {
            for item in e.chain() {
                self.generic_message(kind, Some("caused by:"), format_args!("{item}"));
            }
        }
    }

    fn report_error(&mut self, err: &Error) {
        let mut first = true;
        let kind = MessageKind::Error;

        for item in err.chain() {
            if first {
                self.generic_message(kind, None, format_args!("{item}"));
                first = false;
            } else {
                self.generic_message(kind, Some("caused by:"), format_args!("{item}"));
            }
        }
    }

    fn note_highlighted(&mut self, before: &str, highlighted: &str, after: &str) {
        if self.chatter > ChatterLevel::Minimal {
            let stream = if self.always_stderr {
                &mut self.stderr
            } else {
                &mut self.stdout
            };

            write!(stream, "{before}").expect("write failed");
            stream
                .set_color(&self.highlight_spec)
                .expect("write failed");
            write!(stream, "{highlighted}").expect("write failed");
            stream.reset().expect("write failed");
            writeln!(stream, "{after}").expect("write failed");
        }
    }

    fn dump_error_logs(&mut self, output: &[u8]) {
        tt_error_styled!(
            self,
            "==============================================================================="
        );

        self.stderr
            .write_all(output)
            .expect("write to stderr failed");

        tt_error_styled!(
            self,
            "==============================================================================="
        );
    }
}
