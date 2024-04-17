// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! Standalone compilation of TeX documents. This implements the "classic" /
//! "V1" / "rustc-like" Tectonic command-line interface, as well as the
//! `compile` subcommand of the "V2" / "cargo-like" interface.

use clap::Parser;
use std::path::{Path, PathBuf};
use tectonic_bridge_core::{SecuritySettings, SecurityStance};

use tectonic::{
    config::PersistentConfig,
    driver::{OutputFormat, PassSetting, ProcessingSession, ProcessingSessionBuilder},
    errmsg,
    errors::{ErrorKind, Result},
    status::StatusBackend,
    tt_error, tt_note,
    unstable_opts::{UnstableArg, UnstableOptions},
};

#[derive(Debug, Parser)]
pub struct CompileOptions {
    /// The file to process, or "-" to process the standard input stream
    #[arg(name = "input")]
    input: String,

    /// The name of the "format" file used to initialize the TeX engine
    #[arg(long, short, name = "path", default_value = "latex")]
    format: String,

    /// Use this directory or Zip-format bundle file to find resource files instead of the default
    #[arg(long, short, name = "file_path")]
    bundle: Option<PathBuf>,

    /// Use only resource files cached locally
    #[arg(short = 'C', long)]
    only_cached: bool,

    /// The kind of output to generate
    #[arg(long, name = "format", default_value = "pdf")]
    outfmt: OutputFormat,

    /// Write Makefile-format rules expressing the dependencies of this run to <dest_path>
    #[arg(long, name = "dest_path")]
    makefile_rules: Option<PathBuf>,

    /// Which engines to run
    #[arg(long, default_value = "default")]
    pass: PassSetting,

    /// Rerun the TeX engine exactly this many times after the first
    #[arg(name = "count", long = "reruns", short = 'r')]
    reruns: Option<usize>,

    /// Keep the intermediate files generated during processing
    #[arg(short, long)]
    keep_intermediates: bool,

    /// Keep the log files generated during processing
    #[arg(long)]
    keep_logs: bool,

    /// Generate SyncTeX data
    #[arg(long)]
    synctex: bool,

    /// Tell the engine that no file at <hide_path> exists, if it tries to read it
    #[arg(long, name = "hide_path")]
    hide: Option<Vec<PathBuf>>,

    /// Print the engine's chatter during processing
    #[arg(long = "print", short)]
    print_stdout: bool,

    /// The directory in which to place output files [default: the directory containing <input>]
    #[arg(name = "outdir", short, long)]
    outdir: Option<PathBuf>,

    /// Input is untrusted -- disable all known-insecure features
    #[arg(long)]
    untrusted: bool,

    /// Unstable options. Pass -Zhelp to show a list
    #[arg(name = "option", short = 'Z')]
    unstable: Vec<UnstableArg>,

    /// Use this URL to find resource files instead of the default
    #[arg(long, short, name = "url", overrides_with = "url", global(true))]
    web_bundle: Option<String>,
}

// TODO: deprecate v1 interface and move this to v2cli/commands

//impl TectonicCommand for CompileOptions {
impl CompileOptions {
    //fn customize(&self, _cc: &mut CommandCustomizations) {}

    pub fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let unstable = UnstableOptions::from_unstable_args(self.unstable.into_iter());

        // Default to allowing insecure since it would be super duper annoying
        // to have to pass `--trusted` every time to build a personal document
        // that uses shell-escape! This default can be overridden by setting the
        // environment variable TECTONIC_UNTRUSTED_MODE to a nonempty value.
        let stance = if self.untrusted {
            SecurityStance::DisableInsecures
        } else {
            SecurityStance::MaybeAllowInsecures
        };

        let mut sess_builder =
            ProcessingSessionBuilder::new_with_security(SecuritySettings::new(stance));
        let format_path = self.format;
        let deterministic_mode = unstable.deterministic_mode;
        sess_builder
            .unstables(unstable)
            .format_name(&format_path)
            .keep_logs(self.keep_logs)
            .keep_intermediates(self.keep_intermediates)
            .format_cache_path(config.format_cache_path()?)
            .synctex(self.synctex)
            .output_format(self.outfmt)
            .pass(self.pass);

        if let Some(s) = self.reruns {
            sess_builder.reruns(s);
        }

        if let Some(p) = self.makefile_rules {
            sess_builder.makefile_output_path(p);
        }

        // Input and path setup

        let input_path = self.input;
        if input_path == "-" {
            // Don't provide an input path to the ProcessingSession, so it will default to stdin.
            sess_builder.tex_input_name("texput.tex");
            sess_builder.output_dir(Path::new(""));
            tt_note!(
                status,
                "reading from standard input; outputs will appear under the base name \"texput\""
            );
        } else {
            let input_path = Path::new(&input_path);
            sess_builder.primary_input_path(input_path);

            if let Some(fname) = input_path.file_name() {
                sess_builder.tex_input_name(&fname.to_string_lossy());
            } else {
                return Err(errmsg!(
                    "can't figure out a basename for input path \"{}\"",
                    input_path.to_string_lossy()
                ));
            };

            if let Some(par) = input_path.parent() {
                sess_builder.output_dir(par);
            } else {
                return Err(errmsg!(
                    "can't figure out a parent directory for input path \"{}\"",
                    input_path.to_string_lossy()
                ));
            }
        }

        if let Some(output_dir) = self.outdir {
            if !output_dir.is_dir() {
                return Err(errmsg!(
                    "output directory \"{}\" does not exist",
                    output_dir.display()
                ));
            }
            sess_builder.output_dir(output_dir);
        }

        // Set up the rest of I/O.

        sess_builder.print_stdout(self.print_stdout);

        if let Some(items) = self.hide {
            for v in items {
                sess_builder.hide(v);
            }
        }

        let only_cached = self.only_cached;
        if only_cached {
            tt_note!(status, "using only cached resource files");
        }
        if let Some(path) = self.bundle {
            sess_builder.bundle(config.make_local_file_provider(path, status)?);
        } else if let Some(u) = self.web_bundle {
            sess_builder.bundle(config.make_cached_url_provider(&u, only_cached, None, status)?);
        } else {
            sess_builder.bundle(config.default_bundle(only_cached, status)?);
        }
        sess_builder.build_date_from_env(deterministic_mode);

        run_and_report(sess_builder, status).map(|_| 0)
    }
}

pub(crate) fn run_and_report(
    sess_builder: ProcessingSessionBuilder,
    status: &mut dyn StatusBackend,
) -> Result<ProcessingSession> {
    let mut sess = sess_builder.create(status)?;
    let result = sess.run(status);

    if let Err(e) = &result {
        if let ErrorKind::EngineError(engine) = e.kind() {
            let output = sess.get_stdout_content();

            if output.is_empty() {
                tt_error!(
                    status,
                    "something bad happened inside {}, but no output was logged",
                    engine
                );
            } else {
                tt_error!(
                    status,
                    "something bad happened inside {}; its output follows:\n",
                    engine
                );
                status.dump_error_logs(&output);
            }
        }
    }

    result.map(|_| sess)
}
