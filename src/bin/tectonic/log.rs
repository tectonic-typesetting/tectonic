use core::fmt;

use nu_ansi_term::{Color, Style};
use tracing::{
    field::{Field, Visit},
    Level, Subscriber,
};
use tracing_subscriber::{
    fmt::{format, FormatEvent, FormatFields},
    registry::LookupSpan,
};

/// Utility struct, used to read log data
#[derive(Default)]
pub struct StringVisitor {
    source: Option<String>,
    message: String,
}

impl Visit for StringVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        #[allow(clippy::single_match)]
        match field.name() {
            "message" => self.message = format!("{:?}", value),
            _ => {}
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        #[allow(clippy::single_match)]
        match field.name() {
            "tectonic_log_source" => {
                self.source = Some(value.to_string());
            }
            _ => {}
        }
    }
}

/// Utility struct, formats log lines
pub struct LogFormatter {
    /// If true, use ansi styling.
    /// If false, use no styling.
    with_ansi: bool,
}

impl LogFormatter {
    pub fn new(with_ansi: bool) -> Self {
        Self { with_ansi }
    }
}

impl<S, N> FormatEvent<S, N> for LogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let m = event.metadata();

        let level_str = match *m.level() {
            Level::TRACE => "TRACE",
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
        };

        // Write log level
        if self.with_ansi {
            let level_sty = Style::new();
            let level_sty = match *m.level() {
                Level::TRACE => level_sty.fg(Color::White),
                Level::DEBUG => level_sty.fg(Color::Blue),
                Level::INFO => level_sty.fg(Color::Green),
                Level::WARN => level_sty.fg(Color::Yellow),
                Level::ERROR => level_sty.fg(Color::Red),
            };
            // Pad BEFORE painting so ansi chars don't mess with padding.
            write!(writer, "{} ", level_sty.paint(format!("{:<5}", level_str)))?;
        } else {
            write!(writer, "{:<5} ", level_str)?;
        }

        // Get log content
        let mut sv = StringVisitor::default();
        event.record(&mut sv);

        // Write log source and message
        if self.with_ansi {
            write!(
                writer,
                "{} {}",
                Color::DarkGray.paint(format!("{:<8}", sv.source.unwrap_or("".to_string()))),
                sv.message
            )?;
        } else {
            write!(
                writer,
                "{:<8} {}",
                sv.source.unwrap_or("".to_string()),
                sv.message
            )?;
        }

        writeln!(writer)
    }
}
