// src/bin/tectonic/main.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2023 the Tectonic Project
// Licensed under the MIT License.
use log::LogFormatter;
use std::{env, fmt::Debug, process};
use structopt::StructOpt;
use tracing::{error, Level};

use tectonic::{config::PersistentConfig, unstable_opts};

mod compile;
mod log;
mod watch;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "Tectonic", about = "Process a (La)TeX document")]
struct CliOptions {
    /// Use experimental V2 interface (see `tectonic -X --help`)
    #[structopt(short = "X")]
    use_v2: bool,

    /// Maximum log level to print when running
    #[structopt(long = "log", short, name = "level", default_value = "info", possible_values(&["debug", "info", "warn", "error"]))]
    log_level: String,

    /// Enable/disable colorful log output
    #[structopt(long = "color", name = "when", default_value = "auto", possible_values(&["always", "auto", "never"]))]
    cli_color: String,

    /// Use this URL to find resource files instead of the default
    #[structopt(takes_value(true), long, short, name = "url", overrides_with = "url")]
    // TODO add URL validation
    web_bundle: Option<String>,

    #[structopt(flatten)]
    compile: compile::CompileOptions,
}

#[derive(StructOpt)]
struct PeekUnstableOptions {
    #[structopt(name = "option", short = "Z", number_of_values = 1)]
    unstable: Vec<unstable_opts::UnstableArg>,

    #[structopt()]
    _remainder: Vec<std::ffi::OsString>,
}

fn main() {
    let os_args: Vec<_> = env::args_os().collect();

    // A hack so that you can just run `tectonic -Z help` without getting a
    // usage error about a missing input file specification. If
    // `from_unstable_args()` sees a `help` option, it will print the usage and
    // exit. Otherwise, this will all be a no-op, and we'll re-parse the args
    // "for real" momentarily.

    if let Ok(args) = PeekUnstableOptions::from_args_safe() {
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
        if CliOptions::from_args_safe().is_err() || CliOptions::from_args().use_v2 {
            v2cli_enabled = true;
            v2cli_args.remove(index);
        }
    }

    if v2cli_enabled {
        v2cli::v2_main(&v2cli_args);
        return;
    }

    // OK, we're still using the "rustc-style" CLI. Proceed here.
    let args = CliOptions::from_args();

    // Set up logger
    let log_level = match &*args.log_level {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => unreachable!(),
    };
    let use_cli_color = match &*args.cli_color {
        "always" => true,
        "auto" => atty::is(atty::Stream::Stdout),
        "never" => false,
        _ => unreachable!(),
    };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .event_format(LogFormatter::new(use_cli_color))
        .init();

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

    if let Err(e) = args.compile.execute(config, args.web_bundle) {
        error!(tectonic_log_source = "cli", "{}", e.0.to_string());
        process::exit(1)
    }
}
