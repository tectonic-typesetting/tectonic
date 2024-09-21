// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! The "v2cli" command-line interface -- a "multitool" interface resembling
//! Cargo, as compared to the classic "rustc-like" CLI.

use clap::{Parser, Subcommand};
use std::{env, ffi::OsString, fs, path::Path, path::PathBuf, process};
use tectonic::{
    config::PersistentConfig,
    errors::{Result, SyncError},
    status::{termcolor::TermcolorStatusBackend, ChatterLevel, StatusBackend},
    tt_note,
};
use tectonic_errors::prelude::anyhow;
use tectonic_status_base::plain::PlainStatusBackend;
use tracing::level_filters::LevelFilter;

use self::commands::{
    build::BuildCommand,
    bundle::BundleCommand,
    dump::DumpCommand,
    new::{InitCommand, NewCommand},
    show::ShowCommand,
    watch::WatchCommand,
};

mod commands;

/// The main options for the "V2" command-line interface.
#[derive(Debug, Parser)]
#[command(
    name = "tectonic -X",
    about = "Process (La)TeX documents",
    no_binary_name(true)
)]

struct V2CliOptions {
    /// How much chatter to print when running
    #[arg(long = "chatter", short, default_value = "default")]
    chatter_level: ChatterLevel,

    /// Control colorization of output
    #[arg(long = "color", default_value = "auto")]
    cli_color: crate::CliColor,

    /// The command to run
    #[command(subcommand)]
    command: Commands,
}

/// A semi-hack to allow command-specific customizations of the centralized app
/// initialization.
#[derive(Debug, Default)]
struct CommandCustomizations {
    always_stderr: bool,
    minimal_chatter: bool,
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

    let args = V2CliOptions::parse_from(effective_args);

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .with_target(false)
        .without_time()
        .with_ansi(args.cli_color.should_enable())
        .init();

    // Command-specific customizations before we do our centralized setup.
    // This is a semi-hack so that we can set up certain commands to ensure
    // that status info is always printed to stderr.

    let mut customizations = CommandCustomizations::default();
    match &args.command {
        Commands::Build(o) => o.customize(&mut customizations),
        Commands::Bundle(o) => o.customize(&mut customizations),
        Commands::Compile(_) => {} // avoid namespacing/etc issues
        Commands::Dump(o) => o.customize(&mut customizations),
        Commands::New(o) => o.customize(&mut customizations),
        Commands::Init(o) => o.customize(&mut customizations),
        Commands::Show(o) => o.customize(&mut customizations),
        Commands::Watch(o) => o.customize(&mut customizations),
        Commands::External(_) => {}
    }

    // Set up colorized output.
    let chatter_level = if customizations.minimal_chatter {
        ChatterLevel::Minimal
    } else {
        args.chatter_level
    };

    let mut status = if args.cli_color.should_enable() {
        let mut sb = TermcolorStatusBackend::new(chatter_level);
        sb.always_stderr(customizations.always_stderr);
        Box::new(sb) as Box<dyn StatusBackend>
    } else {
        let mut sb = PlainStatusBackend::new(chatter_level);
        sb.always_stderr(customizations.always_stderr);
        Box::new(sb) as Box<dyn StatusBackend>
    };

    // For now ...

    tt_note!(
        status,
        "\"version 2\" Tectonic command-line interface activated"
    );

    // Now that we've got colorized output, pass off to the inner function.

    let r = match args.command {
        Commands::Build(o) => o.execute(config, &mut *status),
        Commands::Bundle(o) => o.execute(config, &mut *status),
        Commands::Compile(o) => o.execute(config, &mut *status),
        Commands::Dump(o) => o.execute(config, &mut *status),
        Commands::New(o) => o.execute(config, &mut *status),
        Commands::Init(o) => o.execute(config, &mut *status),
        Commands::Show(o) => o.execute(config, &mut *status),
        Commands::Watch(o) => o.execute(config, &mut *status),
        Commands::External(all_args) => do_external(all_args),
    };

    process::exit(match r {
        Ok(c) => c,
        Err(e) => {
            status.report_error(&SyncError::new(e).into());
            1
        }
    })
}

trait TectonicCommand {
    fn customize(&self, cc: &mut CommandCustomizations);
    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32>;
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "build")]
    /// Build a document
    Build(BuildCommand),

    #[command(name = "bundle")]
    /// Commands relating to this document’s TeX file bundle
    Bundle(BundleCommand),

    #[command(name = "compile")]
    /// Run a standalone (La)TeX compilation
    Compile(crate::compile::CompileOptions),

    #[command(name = "dump")]
    /// Run a partial compilation and output an intermediate file
    Dump(DumpCommand),

    #[command(name = "new")]
    /// Create a new document project
    New(NewCommand),

    #[command(name = "init")]
    /// Initializes a new document in the current directory
    Init(InitCommand),

    #[command(name = "show")]
    /// Display various useful pieces of information
    Show(ShowCommand),

    #[command(name = "watch")]
    /// Watch input files and execute commands on change
    Watch(WatchCommand),

    #[command(external_subcommand)]
    /// Runs the external command `tectonic-[command]` if one exists.
    External(Vec<String>),
}

#[cfg(unix)]
/// On Unix, exec() to replace ourselves with the child process. This function
/// *should* never return.
fn exec_or_spawn(cmd: &mut process::Command) -> Result<i32> {
    use std::os::unix::process::CommandExt;

    // exec() only returns an io::Error directly, since on success it never
    // returns; the following tomfoolery transforms it into our Result
    // machinery as desired.
    Err(cmd.exec().into())
}

#[cfg(not(unix))]
/// On other platforms, just run the process and wait for it.
fn exec_or_spawn(cmd: &mut process::Command) -> Result<i32> {
    // code() can only return None on Unix when the subprocess was killed by a
    // signal. This function only runs if we're not on Unix, so we'll always
    // get Some.
    Ok(cmd.status()?.code().unwrap())
}

#[cfg(unix)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::prelude::*;
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn search_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(val) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&val));
    }
    dirs
}

/// Run an external command by executing a subprocess.
fn do_external(all_args: Vec<String>) -> Result<i32> {
    let (cmd, args) = all_args.split_first().unwrap();

    let command_exe = format!("tectonic-{}{}", cmd, env::consts::EXE_SUFFIX);
    let path = search_directories()
        .iter()
        .map(|dir| dir.join(&command_exe))
        .find(|file| is_executable(file));

    let command = path.ok_or_else(|| {
        anyhow!(
            "no internal or external subcommand `{0}` is available (install `tectonic-{0}`?)",
            cmd
        )
    })?;

    exec_or_spawn(process::Command::new(command).args(args))
}
