// src/bin/tectonic/main.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2023 the Tectonic Project
// Licensed under the MIT License.

//! `tectonic` binary - the main entry point for command-line users.

use clap::{Parser, ValueEnum};
use std::{env, io::IsTerminal, process};
use tectonic_status_base::plain::PlainStatusBackend;

use tectonic::{
    config::PersistentConfig,
    errors::SyncError,
    status::{
        termcolor::TermcolorStatusBackend,
        {ChatterLevel, StatusBackend},
    },
    unstable_opts,
};

mod compile;

#[cfg(feature = "serialization")]
mod v2cli;

// Defused V2 support if serialization is unavailable.
#[cfg(not(feature = "serialization"))]
mod v2cli {
    use std::{ffi::OsString, process};

    pub fn v2_main(_effective_args: &[OsString]) {
        eprintln!(
            "fatal error: the \"V2\" Tectonic CLI requires the code to have been built \
            with the \"serialization\" Cargo feature active. This one wasn't."
        );
        process::exit(1);
    }
}

#[derive(Debug, Parser)]
#[command(name = "Tectonic", about = "Process a (La)TeX document")]
struct CliOptions {
    /// Use experimental V2 interface (see `tectonic -X --help`)
    #[arg(short = 'X')]
    use_v2: bool,

    /// How much chatter to print when running
    #[arg(long = "chatter", short, default_value = "default")]
    chatter_level: ChatterLevel,

    /// Enable/disable colorful log output
    #[arg(long = "color", default_value = "auto")]
    cli_color: CliColor,

    #[command(flatten)]
    compile: compile::CompileOptions,
}

#[derive(ValueEnum, Clone, Debug)]
enum CliColor {
    #[value(name = "always")]
    Always,

    #[value(name = "auto")]
    Auto,

    #[value(name = "never")]
    Never,
}

impl CliColor {
    pub fn should_enable(&self) -> bool {
        match self {
            Self::Always => true,
            Self::Auto => std::io::stdout().is_terminal(),
            Self::Never => false,
        }
    }
}

#[derive(Parser)]
struct PeekUnstableOptions {
    #[arg(name = "option", short = 'Z')]
    unstable: Vec<unstable_opts::UnstableArg>,

    #[arg()]
    _remainder: Vec<std::ffi::OsString>,
}

fn main() {
    let os_args: Vec<_> = env::args_os().collect();

    // A hack so that you can just run `tectonic -Z help` without getting a
    // usage error about a missing input file specification. If
    // `from_unstable_args()` sees a `help` option, it will print the usage and
    // exit. Otherwise, this will all be a no-op, and we'll re-parse the args
    // "for real" momentarily.

    if let Ok(args) = PeekUnstableOptions::try_parse() {
        unstable_opts::UnstableOptions::from_unstable_args(args.unstable.into_iter());
    }

    // Migration to the "cargo-style" command-line interface. If the arguments
    // list contains `-X`, or argv[0] contains `nextonic`, we activate the
    // alternative operation mode. Once this experimental mode is working OK,
    // we'll start printing a message telling people to prefer the `-X` option
    // and use `-X compile` for the "classic" ("rustc"-style, current)
    // interface. After that's been in place for a while, we'll make V2 mode the
    // default.

    let mut v2cli_enabled = false;
    let mut v2cli_args = os_args[1..].to_vec(); // deep copy

    if !os_args.is_empty() && os_args[0].to_str().map(|s| s.contains("nextonic")) == Some(true) {
        v2cli_enabled = true;
    } else if let Some(index) = v2cli_args
        .to_vec()
        .iter()
        .position(|s| s.to_str().unwrap_or_default() == "-X")
    {
        // Try to parse as v1 cli first, and when that doesn't work,
        // interpret it as v2 cli:
        if CliOptions::try_parse().is_err() || CliOptions::parse().use_v2 {
            v2cli_enabled = true;
            v2cli_args.remove(index);
        }
    }

    if v2cli_enabled {
        v2cli::v2_main(&v2cli_args);
        return;
    }

    // OK, we're still using the "rustc-style" CLI. Proceed here.

    let args = CliOptions::parse();

    // The Tectonic crate comes with a hidden internal "test mode" that forces
    // it to use a specified set of local files, rather than going to the
    // bundle -- this makes it so that we can run tests without having to go
    // to the network or touch the current user's cache.
    //
    // This mode is activated by setting a special environment variable. The
    // following call checks for it and activates the mode if necessary. Note
    // that this test infrastructure is lightweight, so I don't think it's a
    // big deal to include the code in the final executable artifacts we
    // distribute.

    tectonic::test_util::maybe_activate_test_mode();

    // I want the CLI program to take as little configuration as possible, but
    // we do need to at least provide a mechanism for storing the default
    // bundle.

    let config = match PersistentConfig::open(false) {
        Ok(c) => c,
        Err(ref e) => {
            // Uhoh, we couldn't get the configuration. Our main
            // error-printing code requires a 'status' object, which we don't
            // have yet. If we can't even load the config we might really be
            // in trouble, so it seems safest to keep things simple anyway and
            // just use bare stderr without colorization.
            e.dump_uncolorized();
            process::exit(1);
        }
    };

    // Set up colorized output. This comes after the config because you could
    // imagine wanting to be able to configure the colorization (which is
    // something I'd be relatively OK with since it'd only affect the progam
    // UI, not the processing results).

    let mut status = if args.cli_color.should_enable() {
        Box::new(TermcolorStatusBackend::new(args.chatter_level)) as Box<dyn StatusBackend>
    } else {
        Box::new(PlainStatusBackend::new(args.chatter_level)) as Box<dyn StatusBackend>
    };

    // Now that we've got colorized output, pass off to the inner function ...
    // all so that we can print out the word "error:" in red. This code
    // parallels various bits of the `error_chain` crate.

    if let Err(e) = args.compile.execute(config, &mut *status) {
        status.report_error(&SyncError::new(e).into());
        process::exit(1)
    }
}
