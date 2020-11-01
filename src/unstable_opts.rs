// src/bin/tectonic.rs -- Command-line driver for the Tectonic engine.
// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Unstable options for the Tectonic engine.
//!
//! This is similar to the -Z options on rustc - they're unstable options that are not guaranteed
//! to be reliable or very polished. In particular, many of these prevent the build from being
//! reproducible.

use crate::errors::{Error, Result};
use std::default::Default;
use std::str::FromStr;

const HELPMSG: &str = r#"Available unstable options:

    -Z help                     Lists all unstable options
    -Z continue-on-errors       Keep compiling even when severe errors occur
    -Z min-crossrefs=<num>      Equivalent to bibtex's -min-crossrefs flag - "include after <num>
                                    crossrefs" [default: 2]
    -Z paper-size=<spec>        Change the default paper size [default: letter]
"#;

// Re-add this when \write18 gets implemented
//    -Z shell-escape             Enable \write18

// Each entry of this should correspond to a field of UnstableOptions.
#[derive(Debug)]
pub enum UnstableArg {
    ContinueOnErrors,
    Help,
    MinCrossrefs(i32),
    PaperSize(String),
    ShellEscapeEnabled,
}

impl FromStr for UnstableArg {
    type Err = Error;

    /// Parse from the argument to -Z
    fn from_str(s: &str) -> Result<Self> {
        let mut splitter = s.splitn(2, '=');
        let arg = splitter.next().unwrap(); // splitn will always have at least 1 item
        let value = splitter.next();

        // For structopt/clap, if you pass a value to a flag which doesn't accept one, it's
        // silently ignored.

        match arg {
            "help" => Ok(UnstableArg::Help),

            "continue-on-errors" => Ok(UnstableArg::ContinueOnErrors),

            "min-crossrefs" => value
                .ok_or_else(|| {
                    "'-Z min-crossrefs <spec>' requires a value but none was supplied".into()
                })
                .and_then(|s| {
                    FromStr::from_str(s).map_err(|e| format!("-Z min-crossrefs: {}", e).into())
                })
                .map(UnstableArg::MinCrossrefs),

            "paper-size" => value
                .ok_or_else(|| {
                    "'-Z paper-size <spec>' requires a value but none was supplied".into()
                })
                .map(|s| UnstableArg::PaperSize(s.to_string())),

            // Re-add this when \write18 gets implemetned
            // "shell-escape" => Ok(UnstableArg::ShellEscapeEnabled),
            _ => Err(format!("Unknown unstable option '{}'", arg).into()),
        }
    }
}

#[derive(Debug, Default)]
pub struct UnstableOptions {
    pub continue_on_errors: bool,
    pub paper_size: Option<String>,
    pub shell_escape: bool,
    pub min_crossrefs: Option<i32>,
}

impl UnstableOptions {
    pub fn from_unstable_args<I>(uargs: I) -> Self
    where
        I: Iterator<Item = UnstableArg>,
    {
        let mut opts = UnstableOptions::default();

        for u in uargs {
            use UnstableArg::*;
            match u {
                Help => {
                    print!("{}", HELPMSG);
                    std::process::exit(0);
                }
                ContinueOnErrors => opts.continue_on_errors = true,
                MinCrossrefs(num) => opts.min_crossrefs = Some(num),
                PaperSize(size) => opts.paper_size = Some(size),
                ShellEscapeEnabled => opts.shell_escape = true,
            }
        }

        opts
    }
}
