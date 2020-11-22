// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! The "newcli" command-line interface -- a "multitool" interface resembling
//! Cargo, as compared to the classic "rustc-like" CLI.

use std::{ffi::OsString, process, str::FromStr};
use structopt::{clap::AppSettings, StructOpt};
use tectonic::{
    self,
    config::PersistentConfig,
    errors::Result,
    status::{
        plain::PlainStatusBackend, termcolor::TermcolorStatusBackend, ChatterLevel, StatusBackend,
    },
    tt_note,
};

/// The main options for the "V2"/"newcli" command-line interface.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "tectonic -X",
    about = "Process (La)TeX documents",
    setting(AppSettings::NoBinaryName)
)]
struct NewCliOptions {
    /// How much chatter to print when running
    #[structopt(long = "chatter", short, name = "level", default_value = "default", possible_values(&["default", "minimal"]))]
    chatter_level: String,

    /// Control colorization of output
    #[structopt(long = "color", name = "when", default_value = "auto", possible_values(&["always", "auto", "never"]))]
    cli_color: String,
}

/// The main function for the Cargo-like, "new" CLI. This intentionally
/// duplicates a lot of the "old" main() function, so that the implementation
/// can drift over time as needed.
pub fn new_main(effective_args: &[OsString]) {
    // See main() -- we have a test mode that might need activating.

    tectonic::test_util::maybe_activate_test_mode();

    // I like the idea of not having any global config at all, but as we migrate
    // to the new CLI we'll keep it for now.

    let config = match PersistentConfig::open(false) {
        Ok(c) => c,
        Err(ref e) => {
            e.dump_uncolorized();
            process::exit(1);
        }
    };

    // Parse args -- this will exit if there are problems.

    let args = NewCliOptions::from_iter(effective_args);

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

    if let Err(ref e) = new_inner(args, config, &mut *status) {
        status.report_error(e);
        process::exit(1)
    }
}

fn new_inner(
    _args: NewCliOptions,
    _config: PersistentConfig,
    status: &mut dyn StatusBackend,
) -> Result<()> {
    tt_note!(status, "not implemented");
    Ok(())
}