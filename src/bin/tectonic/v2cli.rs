// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! The "v2cli" command-line interface -- a "multitool" interface resembling
//! Cargo, as compared to the classic "rustc-like" CLI.

use std::{env, ffi::OsString, path::PathBuf, process, str::FromStr};
use structopt::{clap::AppSettings, StructOpt};
use tectonic::{
    self,
    config::PersistentConfig,
    ctry,
    errors::{Result, SyncError},
    status::{termcolor::TermcolorStatusBackend, ChatterLevel, StatusBackend},
    tt_error, tt_note,
    workspace::{self, Workspace},
};
use tectonic_status_base::plain::PlainStatusBackend;

/// The main options for the "V2" command-line interface.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "tectonic -X",
    about = "Process (La)TeX documents",
    setting(AppSettings::NoBinaryName)
)]
struct V2CliOptions {
    /// How much chatter to print when running
    #[structopt(
        long = "chatter",
        short,
        name = "level",
        default_value = "default",
        possible_values(&["default", "minimal"])
    )]
    chatter_level: String,

    /// Control colorization of output
    #[structopt(
        long = "color",
        name = "when",
        default_value = "auto",
        possible_values(&["always", "auto", "never"])
    )]
    cli_color: String,

    #[structopt(subcommand)]
    command: Commands,
}

/// The main function for the Cargo-like, "V2" CLI. This intentionally
/// duplicates a lot of the "old" main() function, so that the implementation
/// can drift over time as needed.
pub fn v2_main(effective_args: &[OsString]) {
    // See main() -- we have a test mode that might need activating.

    tectonic::test_util::maybe_activate_test_mode();

    // I like the idea of not having any global config at all, but as we migrate
    // to the V2 CLI we'll keep it for now.

    let config = match PersistentConfig::open(false) {
        Ok(c) => c,
        Err(ref e) => {
            e.dump_uncolorized();
            process::exit(1);
        }
    };

    // Parse args -- this will exit if there are problems.

    let args = V2CliOptions::from_iter(effective_args);

    // Set up colorized output.

    let chatter_level = ChatterLevel::from_str(&args.chatter_level).unwrap();
    let use_cli_color = match &*args.cli_color {
        "always" => true,
        "auto" => atty::is(atty::Stream::Stdout),
        "never" => false,
        _ => unreachable!(),
    };

    let mut status = if use_cli_color {
        Box::new(TermcolorStatusBackend::new(chatter_level)) as Box<dyn StatusBackend>
    } else {
        Box::new(PlainStatusBackend::new(chatter_level)) as Box<dyn StatusBackend>
    };

    // For now ...

    tt_note!(
        status,
        "\"version 2\" Tectonic command-line interface activated"
    );

    // Now that we've got colorized output, pass off to the inner function.

    if let Err(e) = args.command.execute(config, &mut *status) {
        status.report_error(&SyncError::new(e).into());
        process::exit(1)
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, StructOpt)]
enum Commands {
    #[structopt(name = "build")]
    /// Build a document
    Build(BuildCommand),

    #[structopt(name = "compile")]
    /// Run a standalone (La)TeX compilation
    Compile(crate::compile::CompileOptions),

    #[structopt(name = "watch")]
    /// Watch input files and execute commands on change
    Watch(WatchCommand),

    #[structopt(name = "new")]
    /// Create a new document
    New(NewCommand),
}

impl Commands {
    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        match self {
            Commands::Build(o) => o.execute(config, status),
            Commands::Compile(o) => o.execute(config, status),
            Commands::Watch(o) => o.execute(config, status),
            Commands::New(o) => o.execute(config, status),
        }
    }
}

/// `build`: Build a document
#[derive(Debug, PartialEq, StructOpt)]
pub struct BuildCommand {
    /// Use only resource files cached locally
    #[structopt(short = "C", long)]
    only_cached: bool,

    /// Keep the intermediate files generated during processing
    #[structopt(short, long)]
    keep_intermediates: bool,

    /// Keep the log files generated during processing
    #[structopt(long)]
    keep_logs: bool,

    /// Print the engine's chatter during processing
    #[structopt(long = "print", short)]
    print_stdout: bool,

    /// Open built document using system handler
    #[structopt(long)]
    open: bool,
}

impl BuildCommand {
    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let ws = Workspace::open_from_environment()?;
        let doc = ws.first_document();

        for output_name in doc.output_names() {
            let mut opts = doc.build_options_for(output_name);
            opts.format_cache_path(config.format_cache_path()?)
                .only_cached(self.only_cached)
                .keep_intermediates(self.keep_intermediates)
                .keep_logs(self.keep_logs)
                .print_stdout(self.print_stdout)
                .open(self.open);
            doc.build(output_name, &opts, status)?;
        }

        Ok(0)
    }
}

/// `watch`: Watch input files and execute commands on change
#[derive(Debug, PartialEq, StructOpt)]
pub struct WatchCommand {
    /// Tectonic commands to execute on build [default: build]
    #[structopt(long = "exec", short = "x")]
    execute: Vec<String>,
}

impl WatchCommand {
    fn execute(self, _config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let exe_name = crate::watch::get_trimmed_exe_name()
            .into_os_string()
            .into_string()
            .expect("Executable path wasn't valid UTF-8");
        let mut cmds = Vec::new();
        for x in self.execute.iter() {
            let mut cmd = format!("{} -X ", exe_name);
            let x = x.trim();
            if !x.is_empty() {
                cmd.push_str(x);
                cmds.push(cmd)
            }
        }

        if cmds.is_empty() {
            cmds.push(format!("{} -X build", exe_name))
        }

        let command = cmds.join(" && ");

        let mut final_command = command.clone();
        #[cfg(unix)]
        final_command.push_str("; echo [Finished running. Exit status: $?]");
        #[cfg(windows)]
        final_command.push_str(" & echo [Finished running. Exit status: %ERRORLEVEL%]");
        #[cfg(not(any(unix, windows)))]
        final_command.push_str(" ; echo [Finished running]");

        let mut args = watchexec::ArgsBuilder::default();
        args.cmd(vec![final_command])
            .paths(vec![env::current_dir()?])
            .ignores(vec!["build".to_owned()]);

        let exec_handler = watchexec::run::ExecHandler::new(args.build()?);
        match exec_handler {
            Err(e) => {
                tt_error!(
                    status,
                    "failed to build arguments for watch ExecHandler";
                    e.into()
                );
                Ok(1)
            }
            Ok(exec_handler) => {
                let handler = crate::watch::Watcher {
                    command,
                    inner: exec_handler,
                };
                if let Err(e) = watchexec::watch(&handler) {
                    tt_error!(status, "failed to execute watch"; e.into());
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

/// `new`: Create a new document
#[derive(Debug, PartialEq, StructOpt)]
pub struct NewCommand {
    /// The name of the document directory to create.
    #[structopt(default_value = ".")]
    path: PathBuf,
}

impl NewCommand {
    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        tt_note!(
            status,
            "creating new document in directory `{}`",
            self.path.display()
        );

        let wc = workspace::WorkspaceCreator::new(self.path);
        ctry!(
            wc.create(&config, status);
            "failed to create the new Tectonic workspace"
        );
        Ok(0)
    }
}
