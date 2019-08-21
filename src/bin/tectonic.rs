// src/bin/tectonic.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

use clap::crate_version;
use tectonic;

use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::path::Path;
use std::process;

use tectonic::config::PersistentConfig;
use tectonic::driver::{OutputFormat, PassSetting, ProcessingSessionBuilder};
use tectonic::errors::{ErrorKind, Result};
use tectonic::io::zipbundle::ZipBundle;
use tectonic::status::termcolor::TermcolorStatusBackend;
use tectonic::status::{ChatterLevel, StatusBackend};

use tectonic::{ctry, errmsg, tt_error, tt_error_styled, tt_note};

fn inner(
    args: ArgMatches,
    config: PersistentConfig,
    status: &mut TermcolorStatusBackend,
) -> Result<()> {
    let mut sess_builder = ProcessingSessionBuilder::default();
    let format_path = args.value_of("format").unwrap();
    sess_builder
        .format_name(format_path)
        .keep_logs(args.is_present("keep_logs"))
        .keep_intermediates(args.is_present("keep_intermediates"))
        .format_cache_path(config.format_cache_path()?)
        .synctex(args.is_present("synctex"));

    let output_format = match args.value_of("outfmt").unwrap() {
        "aux" => OutputFormat::Aux,
        "html" => OutputFormat::Html,
        "xdv" => OutputFormat::Xdv,
        "pdf" => OutputFormat::Pdf,
        "format" => OutputFormat::Format,
        _ => unreachable!(),
    };
    sess_builder.output_format(output_format);

    let pass = match args.value_of("pass").unwrap() {
        "default" => PassSetting::Default,
        "bibtex_first" => PassSetting::BibtexFirst,
        "tex" => PassSetting::Tex,
        _ => unreachable!(),
    };
    sess_builder.pass(pass);

    if let Some(s) = args.value_of("reruns") {
        sess_builder.reruns(usize::from_str_radix(s, 10)?);
    }

    if let Some(p) = args.value_of_os("makefile_rules") {
        sess_builder.makefile_output_path(p);
    }

    // Input and path setup

    let input_path = args.value_of_os("INPUT").unwrap();
    if input_path == "-" {
        // Don't provide an input path to the ProcessingSession, so it will default to stdin.
        sess_builder.tex_input_name("texput.tex");
        sess_builder.output_dir(Path::new(""));
        tt_note!(
            status,
            "reading from standard input; outputs will appear under the base name \"texput\""
        );
    } else {
        let input_path = Path::new(input_path);
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

    if let Some(dir) = args.value_of_os("outdir") {
        let output_dir = Path::new(dir);
        if !output_dir.is_dir() {
            return Err(errmsg!(
                "output directory \"{}\" does not exist",
                output_dir.display()
            ));
        }
        sess_builder.output_dir(output_dir);
    }

    // Set up the rest of I/O.

    sess_builder.print_stdout(args.is_present("print_stdout"));

    if let Some(items) = args.values_of_os("hide") {
        for v in items {
            sess_builder.hide(v);
        }
    }

    let only_cached = args.is_present("only_cached");
    if only_cached {
        tt_note!(status, "using only cached resource files");
    }
    if let Some(p) = args.value_of("bundle") {
        let zb = ctry!(ZipBundle::<File>::open(Path::new(&p)); "error opening bundle");
        sess_builder.bundle(Box::new(zb));
    } else if let Some(u) = args.value_of("web_bundle") {
        sess_builder.bundle(Box::new(config.make_cached_url_provider(
            &u,
            only_cached,
            status,
        )?));
    } else {
        sess_builder.bundle(config.default_bundle(only_cached, status)?);
    }

    let mut sess = sess_builder.create(status)?;
    let result = sess.run(status);

    if let Err(e) = &result {
        if let ErrorKind::EngineError(engine) = e.kind() {
            if let Some(output) = sess.io.mem.files.borrow().get(sess.io.mem.stdout_key()) {
                tt_error!(
                    status,
                    "something bad happened inside {}; its output follows:\n",
                    engine
                );
                tt_error_styled!(status, "===============================================================================");
                status.dump_to_stderr(&output);
                tt_error_styled!(status, "===============================================================================");
                tt_error_styled!(status, "");
            }
        }
    }
    result
}

fn main() {
    let matches = App::new("Tectonic")
        .version(crate_version!())
        .about("Process a (La)TeX document")
        .arg(Arg::with_name("format")
             .long("format")
             .value_name("PATH")
             .help("The name of the \"format\" file used to initialize the TeX engine")
             .default_value("latex"))
        .arg(Arg::with_name("bundle")
             .long("bundle")
             .short("b")
             .value_name("PATH")
             .help("Use this Zip-format bundle file to find resource files instead of the default")
             .takes_value(true))
        .arg(Arg::with_name("web_bundle")
             .long("web-bundle")
             .short("w")
             .value_name("URL")
             .help("Use this URL find resource files instead of the default")
             .takes_value(true))
        .arg(Arg::with_name("only_cached")
             .short("C")
             .long("only-cached")
             .help("Use only resource files cached locally"))
        .arg(Arg::with_name("outfmt")
             .long("outfmt")
             .value_name("FORMAT")
             .help("The kind of output to generate")
             .possible_values(&["pdf", "html", "xdv", "aux", "format"])
             .default_value("pdf"))
        .arg(Arg::with_name("makefile_rules")
             .long("makefile-rules")
             .value_name("PATH")
             .help("Write Makefile-format rules expressing the dependencies of this run to <PATH>"))
        .arg(Arg::with_name("pass")
             .long("pass")
             .value_name("PASS")
             .help("Which engines to run")
             .possible_values(&["default", "tex", "bibtex_first"])
             .default_value("default"))
        .arg(Arg::with_name("reruns")
             .long("reruns")
             .short("r")
             .value_name("COUNT")
             .help("Rerun the TeX engine exactly this many times after the first"))
        .arg(Arg::with_name("keep_intermediates")
             .short("k")
             .long("keep-intermediates")
             .help("Keep the intermediate files generated during processing"))
        .arg(Arg::with_name("keep_logs")
             .long("keep-logs")
             .help("Keep the log files generated during processing"))
        .arg(Arg::with_name("synctex")
             .long("synctex")
             .help("Generate SyncTeX data"))
        .arg(Arg::with_name("hide")
             .long("hide")
             .value_name("PATH")
             .multiple(true)
             .number_of_values(1)
             .help("Tell the engine that no file at <PATH> exists, if it tries to read it"))
        .arg(Arg::with_name("print_stdout")
             .long("print")
             .short("p")
             .help("Print the engine's chatter during processing"))
        .arg(Arg::with_name("chatter_level")
             .long("chatter")
             .short("c")
             .value_name("LEVEL")
             .help("How much chatter to print when running")
             .possible_values(&["default", "minimal"])
             .default_value("default"))
        .arg(Arg::with_name("outdir")
             .long("outdir")
             .short("o")
             .value_name("OUTDIR")
             .help("The directory in which to place output files [default: the directory containing INPUT]"))
        .arg(Arg::with_name("INPUT")
             .help("The file to process, or \"-\" to process the standard input stream")
             .required(true)
             .index(1))
        .get_matches ();

    let chatter = match matches.value_of("chatter_level").unwrap() {
        "default" => ChatterLevel::Normal,
        "minimal" => ChatterLevel::Minimal,
        _ => unreachable!(),
    };

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

    let mut status = TermcolorStatusBackend::new(chatter);

    // For now ...

    tt_note!(
        status,
        "this is a BETA release; ask questions and report bugs at https://tectonic.newton.cx/"
    );

    // Now that we've got colorized output, we're to pass off to the inner
    // function ... all so that we can print out the word "error:" in red.
    // This code parallels various bits of the `error_chain` crate.

    if let Err(ref e) = inner(matches, config, &mut status) {
        status.bare_error(e);
        process::exit(1)
    }
}
