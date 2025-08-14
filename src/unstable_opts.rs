// src/bin/tectonic.rs -- Command-line driver for the Tectonic engine.
// Copyright 2020-2022 the Tectonic Project
// Licensed under the MIT License.

//! Unstable options for the Tectonic engine.
//!
//! This is similar to the -Z options on rustc - they're unstable options that are not guaranteed
//! to be reliable or very polished. In particular, many of these prevent the build from being
//! reproducible.

use std::default::Default;
use std::path::PathBuf;
use std::str::FromStr;

const HELPMSG: &str = r#"Available unstable options:

    -Z help                     List all unstable options
    -Z continue-on-errors       Keep compiling even when severe errors occur
    -Z min-crossrefs=<num>      Equivalent to bibtex's -min-crossrefs flag - "include after <num>
                                    crossrefs" [default: 2]
    -Z paper-size=<spec>        Change the initial paper size [default: letter]
    -Z search-path=<path>       Also look in <path> for files (unless --untrusted has been specified),
                                    like TEXINPUTS. Can be specified multiple times.
    -Z shell-escape             Enable \write18 (unless --untrusted has been specified)
    -Z shell-escape-cwd=<path>  Working directory to use for \write18. Use $(pwd) for same behaviour as
                                    most other engines (e.g. for relative paths in \inputminted).
                                    Implies -Z shell-escape
    -Z deterministic-mode       Force a deterministic build environment. Note that setting
                                    `SOURCE_DATE_EPOCH` is usually sufficient for reproducible builds,
                                    and this option makes some extra functionality trade-offs.
                                    Specifically, deterministic mode breaks SyncTeX's auxiliary files
                                    as they include and rely on absolute file paths
"#;

// Each entry of this should correspond to a field of UnstableOptions.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum UnstableArg {
    ContinueOnErrors,
    Help,
    MinCrossrefs(u32),
    PaperSize(String),
    SearchPath(PathBuf),
    ShellEscapeEnabled,
    ShellEscapeCwd(String),
    DeterministicModeEnabled,
}

impl FromStr for UnstableArg {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;

    /// Parse from the argument to -Z
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splitter = s.splitn(2, '=');
        let arg = splitter.next().unwrap(); // splitn will always have at least 1 item
        let value = splitter.next();

        // For structopt/clap, if you pass a value to a flag which doesn't accept one, it's
        // silently ignored.

        let require_value = |value_name| {
            value.ok_or_else(|| {
                format!("'-Z {arg}=<{value_name}>' requires a value but none was supplied",).into()
            })
        };

        let require_no_value = |unwanted_value: Option<&str>, builtin_value: UnstableArg| {
            if let Some(value) = unwanted_value {
                Err(format!(
                    "'-Z {arg}={value}', was supplied but '-Z {arg}' does not take a value."
                )
                .into())
            } else {
                Ok(builtin_value)
            }
        };

        match arg {
            "help" => Ok(UnstableArg::Help),

            "continue-on-errors" => Ok(UnstableArg::ContinueOnErrors),

            "min-crossrefs" => require_value("num")
                .and_then(|s| {
                    FromStr::from_str(s).map_err(|e| format!("-Z min-crossrefs: {e}").into())
                })
                .map(UnstableArg::MinCrossrefs),

            "paper-size" => require_value("spec").map(|s| UnstableArg::PaperSize(s.to_string())),

            "search-path" => require_value("path").map(|s| UnstableArg::SearchPath(s.into())),

            "shell-escape" => require_no_value(value, UnstableArg::ShellEscapeEnabled),

            "shell-escape-cwd" => {
                require_value("path").map(|s| UnstableArg::ShellEscapeCwd(s.to_string()))
            }

            "deterministic-mode" => require_no_value(value, UnstableArg::DeterministicModeEnabled),

            _ => Err(format!("Unknown unstable option '{arg}'").into()),
        }
    }
}

/// Unstable options available for engine backends. These options may be added or removed in minor
/// releases, and are not considered breaking.
///
/// These options may affect the reproducibility of built documents.
#[derive(Debug, Default)]
pub struct UnstableOptions {
    /// Don't stop on errors - attempt to generate a document anyway, for all but the most fatal of
    /// problems.
    pub continue_on_errors: bool,

    /// Set the paper size used by the output document.
    pub paper_size: Option<String>,

    /// Allow using shell commands during document compilation. All shell escapes will be executed
    /// within a custom temporary directory that lives for the duration of the compilation session.
    /// [`Self::shell_escape_cwd`] will take precedence over this flag.
    pub shell_escape: bool,

    /// Minimum number of cross-references in `bibtex` before an item gets its own standalone entry.
    pub min_crossrefs: Option<u32>,

    /// Extra directories to search for input files during a processing session.
    pub extra_search_paths: Vec<PathBuf>,

    /// The working directory to use for shell escapes. The directory will be preserved after
    /// compilation is complete. This overrides [`Self::shell_escape`].
    pub shell_escape_cwd: Option<String>,

    /// Ensure a deterministic build environment.
    ///
    /// The most significant user-facing difference is a static document build
    /// date, but this is already covered by [`crate::driver::ProcessingSessionBuilder::build_date_from_env`],
    /// which accepts a `deterministic` flag. Additionally, deterministic mode
    /// spoofs file modification times and hides absolute paths from the engine.
    ///
    /// There's a few ways to break determinism (shell escape, reading from
    /// `/dev/urandom`), but anything else (especially behaviour in TeXLive
    /// packages) is considered a bug.
    pub deterministic_mode: bool,
}

impl UnstableOptions {
    #[doc(hidden)]
    pub fn from_unstable_args<I>(uargs: I) -> Self
    where
        I: Iterator<Item = UnstableArg>,
    {
        let mut opts = UnstableOptions::default();

        for u in uargs {
            use UnstableArg::*;
            match u {
                Help => print_unstable_help_and_exit(),
                ContinueOnErrors => opts.continue_on_errors = true,
                MinCrossrefs(num) => opts.min_crossrefs = Some(num),
                PaperSize(size) => opts.paper_size = Some(size),
                ShellEscapeEnabled => opts.shell_escape = true,
                SearchPath(p) => opts.extra_search_paths.push(p),
                ShellEscapeCwd(p) => {
                    opts.shell_escape_cwd = Some(p);
                    opts.shell_escape = true;
                }
                DeterministicModeEnabled => opts.deterministic_mode = true,
            }
        }

        opts
    }
}

#[doc(hidden)]
pub fn print_unstable_help_and_exit() {
    print!("{HELPMSG}");
    std::process::exit(0);
}
