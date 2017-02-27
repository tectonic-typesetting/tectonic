// src/cli_driver.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate tectonic;
extern crate termcolor;

use clap::{Arg, ArgMatches, App};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use tectonic::config::Config;
use tectonic::engines::tex::OutputFormat;
use tectonic::errors::{Result, ResultExt};
use tectonic::io::{FilesystemIo, GenuineStdoutIo, IoProvider, IoStack, MemoryIo};
use tectonic::io::itarbundle::{HttpITarIoFactory, ITarBundle};
use tectonic::io::zipbundle::ZipBundle;
use tectonic::status::{ChatterLevel, StatusBackend};
use tectonic::status::termcolor::TermcolorStatusBackend;
use tectonic::{TexEngine, TexResult, XdvipdfmxEngine};


struct CliIoSetup {
    pub bundle: Option<Box<IoProvider>>,
    pub mem: MemoryIo,
    pub filesystem: FilesystemIo,
    pub genuine_stdout: Option<GenuineStdoutIo>,
}

impl CliIoSetup {
    pub fn new(bundle: Option<Box<IoProvider>>, use_genuine_stdout: bool) -> Result<CliIoSetup> {
        Ok(CliIoSetup {
            mem: MemoryIo::new(true),
            filesystem: FilesystemIo::new(Path::new(""), false, true),
            bundle: bundle,
            genuine_stdout: if use_genuine_stdout {
                Some(GenuineStdoutIo::new())
            } else {
                None
            }
        })
    }

    fn as_stack<'a> (&'a mut self) -> IoStack<'a> {
        let mut providers: Vec<&mut IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }

        providers.push(&mut self.mem);
        providers.push(&mut self.filesystem);

        if let Some(ref mut b) = self.bundle {
            providers.push(&mut **b);
        }

        IoStack::new(providers)
    }
}


fn inner(matches: ArgMatches, config: Config, status: &mut TermcolorStatusBackend) -> Result<i32> {
    let format = matches.value_of("format").unwrap();
    let input = matches.value_of("INPUT").unwrap();

    let outfmt = match matches.value_of("outfmt").unwrap() {
        "xdv" => OutputFormat::Xdv,
        "pdf" => OutputFormat::Pdf,
        _ => unreachable!()
    };

    // Set up I/O. The IoStack struct must necessarily erase types (i.e., turn
    // I/O layers into IoProvider trait objects) while it lives. But, between
    // invocations of various engines, we want to look at our individual typed
    // I/O providers and interrogate them (i.e., see what files were created
    // in the memory layer. The CliIoSetup struct helps us maintain detailed
    // knowledge of types while creating an IoStack when needed. In principle
    // we could reuse the same IoStack for each processing step, but the
    // borrow checker doesn't let us poke at (e.g.) io.mem while the IoStack
    // exists, since the IoStack keeps a mutable borrow of it.

    let bundle: Option<Box<IoProvider>>;

    if let Some(p) = matches.value_of("bundle") {
        let zb = ZipBundle::<File>::open(Path::new(&p)).chain_err(|| "error opening bundle")?;
        bundle = Some(Box::new(zb));
    } else if let Some(u) = matches.value_of("web_bundle") {
        let tb = ITarBundle::<HttpITarIoFactory>::new(&u);
        bundle = Some(Box::new(tb));
    } else {
        bundle = Some(config.default_io_provider(status)?);
    }

    let mut io = CliIoSetup::new(bundle, matches.is_present("print_stdout"))?;

    // First TeX pass.

    let result = {
        let mut stack = io.as_stack();
        let mut engine = TexEngine::new();
        engine.set_halt_on_error_mode(true);
        // NOTE! We manage PDF output by running the xdvipdfmx engine
        // separately, not by having the C code deal with it.
        engine.set_output_format(OutputFormat::Xdv);
        tt_note_styled!(status, "Running TeX ...");
        engine.process(&mut stack, status, format, input)
    };

    match result {
        Ok(TexResult::Spotless) => {},
        Ok(TexResult::Warnings) => {
            tt_note!(status, "warnings were issued by the TeX engine; use --print and/or --keeplog for details.");
        },
        Ok(TexResult::Errors) => {
            tt_warning!(status, "errors were issued by the TeX engine, but were ignored; \
                                 use --print and/or --keeplog for details.");
        },
        Err(e) => {
            if let Some(output) = io.mem.files.borrow().get(io.mem.stdout_key()) {
                tt_error!(status, "something bad happened inside TeX; its output follows\n");
                tt_error_styled!(status, "===============================================================================");
                status.dump_to_stderr(&output);
                tt_error_styled!(status, "===============================================================================");
                tt_error_styled!(status, "");
            }

            return Err(e);
        }
    }

    // If requested, convert the XDV output to PDF.

    if let OutputFormat::Pdf = outfmt {
        let mut xdv_path = PathBuf::from(input);
        xdv_path.set_extension("xdv");

        let mut pdf_path = PathBuf::from(input);
        pdf_path.set_extension("pdf");

        {
            let mut stack = io.as_stack();
            let mut engine = XdvipdfmxEngine::new ();
            tt_note_styled!(status, "Running xdvipdfmx ...");
            engine.process(&mut stack, status, &xdv_path.to_str().unwrap(), &pdf_path.to_str().unwrap())?;
        }

        io.mem.files.borrow_mut().remove(xdv_path.as_os_str());
    }

    // If we got this far, then we did OK. Write out the output files of
    // interest.

    for (name, contents) in &*io.mem.files.borrow() {
        let sname = name.to_string_lossy();

        if name == io.mem.stdout_key() {
            continue;
        }

        if sname.ends_with(".log") && !matches.is_present("keeplog") {
            continue;
        }

        if contents.len() == 0 {
            tt_note_styled!(status, "Not writing {}: it would be empty.", sname);
            continue;
        }

        tt_note_styled!(status, "Writing {} ({} bytes).", sname, contents.len());

        let mut f = File::create(Path::new(name))?;
        f.write_all(contents)?;
    }

    Ok(0)
}


fn main() {
    let matches = App::new("Tectonic")
        .version("0.1")
        .about("Process a (La)TeX document.")
        .arg(Arg::with_name("format")
             .long("format")
             .value_name("PATH")
             .help("The \"format\" used to initialize the engine")
             .default_value("xelatex.fmt"))
        .arg(Arg::with_name("bundle")
             .long("bundle")
             .short("b")
             .value_name("PATH")
             .help("The bundle file containing LaTeX resource files")
             .takes_value(true))
        .arg(Arg::with_name("web_bundle")
             .long("web-bundle")
             .short("w")
             .value_name("URL")
             .help("The URL of a bundle file containing LaTeX resource files")
             .takes_value(true))
        .arg(Arg::with_name("outfmt")
             .long("outfmt")
             .value_name("FORMAT")
             .help("The kind of output to generate")
             .possible_values(&["pdf", "xdv"])
             .default_value("pdf"))
        .arg(Arg::with_name("keeplog")
             .long("keeplog")
             .help("Keep the \"<INPUT>.log\" file generated during processing."))
        .arg(Arg::with_name("print_stdout")
             .long("print")
             .short("p")
             .help("Print the engine's chatter during processing."))
        .arg(Arg::with_name("chatter_level")
             .long("chatter")
             .short("c")
             .value_name("LEVEL")
             .help("How much chatter to print when running")
             .possible_values(&["default", "minimal"])
             .default_value("default"))
        .arg(Arg::with_name("INPUT")
             .help("The file to process.")
             .required(true)
             .index(1))
        .get_matches ();

    let chatter = match matches.value_of("chatter_level").unwrap() {
        "default" => ChatterLevel::Normal,
        "minimal" => ChatterLevel::Minimal,
        _ => unreachable!()
    };

    // I want the CLI program to take as little configuration as possible, but
    // we do need to at least provide a mechanism for storing the default
    // bundle.

    let config = match Config::open() {
        Ok(c) => c,
        Err(ref e) => {
            // Uhoh, we couldn't get the configuration. Our main
            // error-printing code requires a 'status' object, which we don't
            // have yet. If we can't even load the config we might really be
            // in trouble, so it seems safest to keep things simple anyway and
            // just use bare stderr without colorization.

            use std::io::stderr;
            let mut first = true;
            let mut s = stderr();

            for item in e.iter() {
                if first {
                    writeln!(s, "error: {}", item).expect("write to stderr failed");
                    first = false;
                } else {
                    writeln!(s, "caused by: {}", item).expect("write to stderr failed");
                }
            }

            if let Some(backtrace) = e.backtrace() {
                writeln!(s, "{:?}", backtrace).expect("write to stderr failed");
            }

            process::exit(1);
        }
    };

    // Set up colorized output. This comes after the config because you could
    // imagine wanting to be able to configure the colorization (which is
    // something I'd be relatively OK with since it'd only affect the progam
    // UI, not the processing results).

    let mut status = TermcolorStatusBackend::new(chatter);

    // Now that we've got colorized output, we're to pass off to the inner
    // function ... all so that we can print out the word "error:" in red.
    // This code parallels various bits of the `error_chain` crate.

    process::exit(match inner(matches, config, &mut status) {
        Ok(ret) => ret,

        Err(ref e) => {
            let mut first = true;

            for item in e.iter() {
                if first {
                    tt_error!(status, "{}", item);
                    first = false;
                } else {
                    status.caused_by(item);
                }
            }

            if let Some(backtrace) = e.backtrace() {
                use std::io::stderr;
                writeln!(stderr(), "{:?}", backtrace).expect("write to stderr failed");
            }

            1
        }
    })
}
