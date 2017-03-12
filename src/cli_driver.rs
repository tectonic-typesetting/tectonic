// src/cli_driver.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate aho_corasick;
extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate tectonic;
extern crate termcolor;

use aho_corasick::{Automaton, AcAutomaton};
use clap::{Arg, ArgMatches, App};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use tectonic::config::PersistentConfig;
use tectonic::errors::{Result, ResultExt};
use tectonic::io::{FilesystemIo, GenuineStdoutIo, IoProvider, IoStack, MemoryIo};
use tectonic::io::itarbundle::{HttpITarIoFactory, ITarBundle};
use tectonic::io::stack::FileSummary;
use tectonic::io::zipbundle::ZipBundle;
use tectonic::status::{ChatterLevel, StatusBackend};
use tectonic::status::termcolor::TermcolorStatusBackend;
use tectonic::{BibtexEngine, TexEngine, TexResult, XdvipdfmxEngine};


/// The CliIoSetup struct encapsulates, well, the input/output setup used by
/// the Tectonic engines in this CLI session.
///
/// The IoStack struct must necessarily erase types (i.e., turn I/O layers
/// into IoProvider trait objects) while it lives. But, between invocations of
/// various engines, we want to look at our individual typed I/O providers and
/// interrogate them (i.e., see what files were created in the memory layer.
/// The CliIoSetup struct helps us maintain detailed knowledge of types while
/// creating an IoStack when needed. In principle we could reuse the same
/// IoStack for each processing step, but the borrow checker doesn't let us
/// poke at (e.g.) io.mem while the IoStack exists, since the IoStack keeps a
/// mutable borrow of it.

struct CliIoSetup {
    pub bundle: Option<Box<IoProvider>>,
    pub mem: MemoryIo,
    pub filesystem: FilesystemIo,
    pub genuine_stdout: Option<GenuineStdoutIo>,
    pub summaries: HashMap<OsString, FileSummary>,
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
            },
            summaries: HashMap::new(),
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

        IoStack::new(providers, Some(&mut self.summaries))
    }
}


/// The ProcessingSession struct runs the whole show when we're actually
/// processing a file. It merges the command-line arguments and the persistent
/// configuration to figure out what exactly we're going to do.

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum OutputFormat {
    Xdv,
    Pdf,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum PassSetting {
    Tex,
    Default,
}

struct ProcessingSession {
    io: CliIoSetup,
    pass: PassSetting,
    tex_path: String,
    format_path: String,
    aux_path: PathBuf,
    bbl_path: PathBuf,
    xdv_path: PathBuf,
    pdf_path: PathBuf,
    output_format: OutputFormat,
    keep_log: bool,
    noted_tex_warnings: bool,
}

impl ProcessingSession {
    pub fn new(args: &ArgMatches, config: &PersistentConfig, status: &mut TermcolorStatusBackend) -> Result<ProcessingSession> {
        let format_path = args.value_of("format").unwrap();
        let tex_path = args.value_of("INPUT").unwrap();

        let output_format = match args.value_of("outfmt").unwrap() {
            "xdv" => OutputFormat::Xdv,
            "pdf" => OutputFormat::Pdf,
            _ => unreachable!()
        };

        let pass = match args.value_of("pass").unwrap() {
            "default" => PassSetting::Default,
            "tex" => PassSetting::Tex,
            _ => unreachable!()
        };

        // We hardcode these but could someday make them more configurable.

        let mut aux_path = PathBuf::from(&tex_path);
        aux_path.set_extension("aux");

        let mut bbl_path = PathBuf::from(&tex_path);
        bbl_path.set_extension("bbl");

        let mut xdv_path = PathBuf::from(&tex_path);
        xdv_path.set_extension("xdv");

        let mut pdf_path = PathBuf::from(&tex_path);
        pdf_path.set_extension("pdf");

        // Set up I/O.

        let bundle: Option<Box<IoProvider>>;

        if let Some(p) = args.value_of("bundle") {
            let zb = ZipBundle::<File>::open(Path::new(&p)).chain_err(|| "error opening bundle")?;
            bundle = Some(Box::new(zb));
        } else if let Some(u) = args.value_of("web_bundle") {
            let tb = ITarBundle::<HttpITarIoFactory>::new(&u);
            bundle = Some(Box::new(tb));
        } else {
            bundle = Some(config.default_io_provider(status)?);
        }

        let io = CliIoSetup::new(bundle, args.is_present("print_stdout"))?;

        // Ready to roll.

        Ok(ProcessingSession {
            io: io,
            pass: pass,
            tex_path: tex_path.to_owned(),
            format_path: format_path.to_owned(),
            aux_path: aux_path,
            bbl_path: bbl_path,
            xdv_path: xdv_path,
            pdf_path: pdf_path,
            output_format: output_format,
            keep_log: args.is_present("keeplog"),
            noted_tex_warnings: false,
        })
    }


    fn run(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        match self.pass {
            PassSetting::Tex => self.tex_pass(status),
            PassSetting::Default => self.default_pass(status),
        }?;

        for (name, contents) in &*self.io.mem.files.borrow() {
            let sname = name.to_string_lossy();

            if name == self.io.mem.stdout_key() {
                continue;
            }

            if (sname.ends_with(".log") || sname.ends_with(".blg")) && !self.keep_log {
                continue;
            }

            if contents.len() == 0 {
                status.note_highlighted("Not writing ", &sname, ": it would be empty.");
                continue;
            }

            status.note_highlighted("Writing ", &sname, &format!(" ({} bytes)", contents.len()));

            let mut f = File::create(Path::new(name))?;
            f.write_all(contents)?;
        }

        Ok(0)
    }


    /// The "default" pass really runs a bunch of sub-passes. It is a "Do What
    /// I Mean" operation. TODO: we are not nearly clever enough about
    /// figuring out how many times TeX needs to be run.
    fn default_pass(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        self.tex_pass(status)?;

        // Figure out if we need to run bibtex by looking for "\citation" or
        // "\bibcite" in the aux file. This Aho-Corasick automaton business is
        // kind of overkill but it's an easy way to do the search efficiently.

        let use_bibtex = {
            if let Some(auxdata) = self.io.mem.files.borrow().get(self.aux_path.as_os_str()) {
                let cite_aut = AcAutomaton::new(vec!["\\citation", "\\bibcite"]);
                cite_aut.find(auxdata).count() > 0
            } else {
                false
            }
        };

        if use_bibtex {
            self.bibtex_pass(status)?;
            self.tex_pass(status)?;
            self.io.mem.files.borrow_mut().remove(self.bbl_path.as_os_str());
        }

        if let OutputFormat::Pdf = self.output_format {
            self.xdvipdfmx_pass(status)?;
        }

        Ok(0)
    }

    /// Run one pass of the TeX engine.

    fn tex_pass(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        let result = {
            let mut stack = self.io.as_stack();
            let mut engine = TexEngine::new();
            engine.set_halt_on_error_mode(true);
            status.note_highlighted("Running ", "TeX", " ...");
            engine.process(&mut stack, status, &self.format_path, &self.tex_path)
        };

        match result {
            Ok(TexResult::Spotless) => {},
            Ok(TexResult::Warnings) => {
                if !self.noted_tex_warnings {
                    tt_note!(status, "warnings were issued by the TeX engine; use --print and/or --keeplog for details.");
                    self.noted_tex_warnings = true;
                }
            },
            Ok(TexResult::Errors) => {
                if !self.noted_tex_warnings {
                    // Weakness: if a first pass produces warnings and a
                    // second pass produces ignored errors, we won't say so.
                    tt_warning!(status, "errors were issued by the TeX engine, but were ignored; \
                                         use --print and/or --keeplog for details.");
                    self.noted_tex_warnings = true;
                }
            },
            Err(e) => {
                if let Some(output) = self.io.mem.files.borrow().get(self.io.mem.stdout_key()) {
                    tt_error!(status, "something bad happened inside TeX; its output follows:\n");
                    tt_error_styled!(status, "===============================================================================");
                    status.dump_to_stderr(&output);
                    tt_error_styled!(status, "===============================================================================");
                    tt_error_styled!(status, "");
                }

                return Err(e);
            }
        }

        Ok(0)
    }


    fn bibtex_pass(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        let result = {
            let mut stack = self.io.as_stack();
            let mut engine = BibtexEngine::new ();
            status.note_highlighted("Running ", "BibTeX", " ...");
            engine.process(&mut stack, status, &self.aux_path.to_str().unwrap())
        };

        match result {
            Ok(TexResult::Spotless) => {},
            Ok(TexResult::Warnings) => {
                tt_note!(status, "warnings were issued by BibTeX; use --print and/or --keeplog for details.");
            },
            Ok(TexResult::Errors) => {
                tt_warning!(status, "errors were issued by BibTeX, but were ignored; \
                                          use --print and/or --keeplog for details.");
            },
            Err(e) => {
                if let Some(output) = self.io.mem.files.borrow().get(self.io.mem.stdout_key()) {
                    tt_error!(status, "something bad happened inside BibTeX; its output follows:\n");
                    tt_error_styled!(status, "===============================================================================");
                    status.dump_to_stderr(&output);
                    tt_error_styled!(status, "===============================================================================");
                    tt_error_styled!(status, "");
                }

                return Err(e);
            }
        }

        Ok(0)
    }


    fn xdvipdfmx_pass(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        {
            let mut stack = self.io.as_stack();
            let mut engine = XdvipdfmxEngine::new ();
            status.note_highlighted("Running ", "xdvipdfmx", " ...");
            engine.process(&mut stack, status, &self.xdv_path.to_str().unwrap(), &self.pdf_path.to_str().unwrap())?;
        }

        self.io.mem.files.borrow_mut().remove(self.xdv_path.as_os_str());
        Ok(0)
    }
}


fn inner(matches: ArgMatches, config: PersistentConfig, status: &mut TermcolorStatusBackend) -> Result<i32> {
    let mut sess = ProcessingSession::new(&matches, &config, status)?;
    sess.run(status)
}


fn main() {
    let matches = App::new("Tectonic")
        .version("0.1.1")
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
        .arg(Arg::with_name("pass")
             .long("pass")
             .value_name("PASS")
             .help("Which engines to run.")
             .possible_values(&["default", "tex"])
             .default_value("default"))
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

    tt_note!(status, "this is a BETA release; report issues at https://github.com/tectonic-typesetting/tectonic/issues");

    // Now that we've got colorized output, we're to pass off to the inner
    // function ... all so that we can print out the word "error:" in red.
    // This code parallels various bits of the `error_chain` crate.

    process::exit(match inner(matches, config, &mut status) {
        Ok(ret) => ret,

        Err(ref e) => {
            status.bare_error(e);
            1
        }
    })
}
