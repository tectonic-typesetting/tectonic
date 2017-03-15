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
use tectonic::engines::{AccessPattern, FileSummary};
use tectonic::errors::{Result, ResultExt};
use tectonic::io::{FilesystemIo, GenuineStdoutIo, InputOrigin, IoProvider, IoStack, MemoryIo};
use tectonic::io::itarbundle::{HttpITarIoFactory, ITarBundle};
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


/// The ProcessingSession struct runs the whole show when we're actually
/// processing a file. It merges the command-line arguments and the persistent
/// configuration to figure out what exactly we're going to do.

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum OutputFormat {
    Aux,
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
    summaries: HashMap<OsString, FileSummary>,
    pass: PassSetting,
    tex_path: String,
    format_path: String,
    aux_path: PathBuf,
    xdv_path: PathBuf,
    pdf_path: PathBuf,
    output_format: OutputFormat,
    makefile_output_path: Option<PathBuf>,
    tex_rerun_specification: Option<usize>,
    keep_intermediates: bool,
    keep_logs: bool,
    noted_tex_warnings: bool,
}


const DEFAULT_MAX_TEX_PASSES: usize = 6;

impl ProcessingSession {
    pub fn new(args: &ArgMatches, config: &PersistentConfig, status: &mut TermcolorStatusBackend) -> Result<ProcessingSession> {
        let format_path = args.value_of("format").unwrap();
        let tex_path = args.value_of("INPUT").unwrap();

        let output_format = match args.value_of("outfmt").unwrap() {
            "aux" => OutputFormat::Aux,
            "xdv" => OutputFormat::Xdv,
            "pdf" => OutputFormat::Pdf,
            _ => unreachable!()
        };

        let pass = match args.value_of("pass").unwrap() {
            "default" => PassSetting::Default,
            "tex" => PassSetting::Tex,
            _ => unreachable!()
        };

        let reruns = match args.value_of("reruns") {
            Some(s) => Some(usize::from_str_radix(s, 10)?),
            None => None,
        };

        let keep_intermediates = if output_format == OutputFormat::Aux {
            true
        } else {
            args.is_present("keep_intermediates")
        };

        let makefile_output_path = args.value_of_os("makefile_rules").map(|s| s.into());

        // We hardcode these but could someday make them more configurable.

        let mut aux_path = PathBuf::from(&tex_path);
        aux_path.set_extension("aux");

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
            summaries: HashMap::new(),
            pass: pass,
            tex_path: tex_path.to_owned(),
            format_path: format_path.to_owned(),
            aux_path: aux_path,
            xdv_path: xdv_path,
            pdf_path: pdf_path,
            output_format: output_format,
            makefile_output_path: makefile_output_path,
            tex_rerun_specification: reruns,
            keep_intermediates: keep_intermediates,
            keep_logs: args.is_present("keep_logs"),
            noted_tex_warnings: false,
        })
    }


    /// Assess whether we need to rerun an engine. This is the case if there
    /// was a file that the engine read and then rewrote, and the rewritten
    /// version is different than the version that it read in.
    fn rerun_needed(&mut self, status: &mut TermcolorStatusBackend) -> Option<String> {
        // TODO: we should probably wire up diagnostics since I expect this
        // stuff could get finicky and we're going to want to be able to
        // figure out why rerun detection is breaking.

        for (name, info) in &self.summaries {
            if info.access_pattern == AccessPattern::ReadThenWritten {
                let file_changed = match (&info.read_digest, &info.write_digest) {
                    (&Some(ref d1), &Some(ref d2)) => d1 != d2,
                    (&None, &Some(_)) => true,
                    (_, _) => {
                        // Other cases shouldn't happen.
                        tt_warning!(status, "internal consistency problem when checking if {} changed",
                                    name.to_string_lossy());
                        true
                    }
                };

                if file_changed {
                    return Some(name.to_string_lossy().into_owned());
                }
            }
        }

        return None;
    }

    #[allow(dead_code)]
    fn _dump_access_info(&self, status: &mut TermcolorStatusBackend) {
        for (name, info) in &self.summaries {
            if info.access_pattern != AccessPattern::Read {
                use std::string::ToString;
                let r = match info.read_digest {
                    Some(ref d) => d.to_string(),
                    None => "-".into()
                };
                let w = match info.write_digest {
                    Some(ref d) => d.to_string(),
                    None => "-".into()
                };
                tt_note!(status, "ACCESS: {} {:?} {:?} {:?}",
                         name.to_string_lossy(),
                         info.access_pattern, r, w);
            }
        }
    }

    fn run(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        // Do the meat of the work.

        match self.pass {
            PassSetting::Tex => self.tex_pass(None, status),
            PassSetting::Default => self.default_pass(status),
        }?;

        // Write output files and the first line of our Makefile output.

        let mut mf_dest_maybe = match self.makefile_output_path {
            Some(ref p) => Some(File::create(p)?),
            None => None
        };

        let mut n_skipped_intermediates = 0;

        for (name, contents) in &*self.io.mem.files.borrow() {
            if name == self.io.mem.stdout_key() {
                continue;
            }

            let sname = name.to_string_lossy();
            let summ = self.summaries.get(name).unwrap();

            if summ.access_pattern != AccessPattern::Written && !self.keep_intermediates {
                n_skipped_intermediates += 1;
                continue;
            }

            if summ.access_pattern == AccessPattern::Written && self.output_format == OutputFormat::Aux {
                continue;
            }

            if (sname.ends_with(".log") || sname.ends_with(".blg")) && !self.keep_logs {
                continue;
            }

            if contents.len() == 0 {
                status.note_highlighted("Not writing ", &sname, ": it would be empty.");
                continue;
            }

            status.note_highlighted("Writing ", &sname, &format!(" ({} bytes)", contents.len()));

            let mut f = File::create(Path::new(name))?;
            f.write_all(contents)?;

            if let Some(ref mut mf_dest) = mf_dest_maybe {
                // Maybe it'd be better to have this just be a warning? But if
                // the program is supposed to write the file, you don't want
                // it exiting with error code zero if it couldn't do that
                // successfully.
                //
                // Not quite sure why, but I can't pull out the target path
                // here. I think 'self' is borrow inside the loop?
                write!(mf_dest, "{} ", sname).chain_err(|| "couldn't write to Makefile-rules file")?;
            }
        }

        if n_skipped_intermediates > 0 {
            status.note_highlighted("Skipped writing ", &format!("{}", n_skipped_intermediates),
                                    " intermediate files (use --keep-intermediates to keep them)");
        }

        // Finish Makefile rules, maybe.

        if let Some(ref mut mf_dest) = mf_dest_maybe {
            write!(mf_dest, ":").chain_err(|| "couldn't write to Makefile-rules file")?;

            for (name, info) in &self.summaries {
                if info.input_origin != InputOrigin::Filesystem {
                    continue;
                }

                write!(mf_dest, " \\\n  {}",
                       name.to_string_lossy()).chain_err(|| "couldn't write to Makefile-rules file")?;
            }

            writeln!(mf_dest, "").chain_err(|| "couldn't write to Makefile-rules file")?;
        }

        // All done.

        Ok(0)
    }


    /// The "default" pass really runs a bunch of sub-passes. It is a "Do What
    /// I Mean" operation.
    fn default_pass(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        self.tex_pass(None, status)?;
        let mut rerun_result = self.rerun_needed(status);

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
            rerun_result = Some(String::new());
        }

        // Rerun.

        let (pass_count, reruns_fixed) = match self.tex_rerun_specification {
            Some(n) => (n, true),
            None => (DEFAULT_MAX_TEX_PASSES, false),
        };

        for i in 0..pass_count {
            let rerun_explanation = if reruns_fixed {
                "I was told to".to_owned()
            } else {
                match rerun_result {
                    Some(ref s) => {
                        if s == "" {
                            "bibtex was run".to_owned()
                        } else {
                            format!("\"{}\" changed", s)
                        }
                    },
                    None => {
                        break;
                    }
                }
            };

            // We're restarting the engine afresh, so clear the read inputs.
            // We do *not* clear the entire HashMap since we want to remember,
            // e.g., that bibtex wrote out the .bbl file, since that way we
            // can later know that it's OK to delete. I am not super confident
            // that the access_pattern data can just be left as-is when we do
            // this, but, uh, so far it seems to work.
            for summ in self.summaries.values_mut() {
                summ.read_digest = None;
            }

            self.tex_pass(Some(&rerun_explanation), status)?;

            if !reruns_fixed {
                rerun_result = self.rerun_needed(status);

                if rerun_result.is_some() && i == DEFAULT_MAX_TEX_PASSES - 1 {
                    tt_warning!(status, "TeX rerun seems needed, but stopping at {} passes", DEFAULT_MAX_TEX_PASSES);
                    break;
                }
            }
        }

        // And finally, xdvipdfmx. Maybe.

        if let OutputFormat::Pdf = self.output_format {
            self.xdvipdfmx_pass(status)?;
        }

        Ok(0)
    }

    /// Run one pass of the TeX engine.

    fn tex_pass(&mut self, rerun_explanation: Option<&str>, status: &mut TermcolorStatusBackend) -> Result<i32> {
        let result = {
            let mut stack = self.io.as_stack();
            let mut engine = TexEngine::new();
            engine.set_halt_on_error_mode(true);
            if let Some(s) = rerun_explanation {
                status.note_highlighted("Rerunning ", "TeX", &format!(" because {} ...", s));
            } else {
                status.note_highlighted("Running ", "TeX", " ...");
            }
            engine.process(&mut stack, Some(&mut self.summaries), status,
                           &self.format_path, &self.tex_path)
        };

        match result {
            Ok(TexResult::Spotless) => {},
            Ok(TexResult::Warnings) => {
                if !self.noted_tex_warnings {
                    tt_note!(status, "warnings were issued by the TeX engine; use --print and/or --keep-logs for details.");
                    self.noted_tex_warnings = true;
                }
            },
            Ok(TexResult::Errors) => {
                if !self.noted_tex_warnings {
                    // Weakness: if a first pass produces warnings and a
                    // second pass produces ignored errors, we won't say so.
                    tt_warning!(status, "errors were issued by the TeX engine, but were ignored; \
                                         use --print and/or --keep-logs for details.");
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
            engine.process(&mut stack, Some(&mut self.summaries), status,
                           &self.aux_path.to_str().unwrap())
        };

        match result {
            Ok(TexResult::Spotless) => {},
            Ok(TexResult::Warnings) => {
                tt_note!(status, "warnings were issued by BibTeX; use --print and/or --keep-logs for details.");
            },
            Ok(TexResult::Errors) => {
                tt_warning!(status, "errors were issued by BibTeX, but were ignored; \
                                          use --print and/or --keep-logs for details.");
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
            engine.process(&mut stack, Some(&mut self.summaries), status,
                           &self.xdv_path.to_str().unwrap(), &self.pdf_path.to_str().unwrap())?;
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
        .version("0.1.2")
        .about("Process a (La)TeX document.")
        .arg(Arg::with_name("format")
             .long("format")
             .value_name("PATH")
             .help("The name of the \"format\" file used to initialize the TeX engine.")
             .default_value("xelatex.fmt"))
        .arg(Arg::with_name("bundle")
             .long("bundle")
             .short("b")
             .value_name("PATH")
             .help("Use this Zip-format bundle file to find resource files instead of the default.")
             .takes_value(true))
        .arg(Arg::with_name("web_bundle")
             .long("web-bundle")
             .short("w")
             .value_name("URL")
             .help("Use this URL find resource files instead of the default.")
             .takes_value(true))
        .arg(Arg::with_name("outfmt")
             .long("outfmt")
             .value_name("FORMAT")
             .help("The kind of output to generate.")
             .possible_values(&["pdf", "xdv", "aux"])
             .default_value("pdf"))
        .arg(Arg::with_name("makefile_rules")
             .long("makefile-rules")
             .value_name("PATH")
             .help("Write Makefile-format rules expressing the dependencies of this run to <PATH>."))
        .arg(Arg::with_name("pass")
             .long("pass")
             .value_name("PASS")
             .help("Which engines to run.")
             .possible_values(&["default", "tex"])
             .default_value("default"))
        .arg(Arg::with_name("reruns")
             .long("reruns")
             .short("r")
             .value_name("COUNT")
             .help("Rerun the TeX engine exactly this many times after the first."))
        .arg(Arg::with_name("keep_intermediates")
             .short("k")
             .long("keep-intermediates")
             .help("Keep the intermediate files generated during processing."))
        .arg(Arg::with_name("keep_logs")
             .long("keep-logs")
             .help("Keep the log files generated during processing."))
        .arg(Arg::with_name("print_stdout")
             .long("print")
             .short("p")
             .help("Print the engine's chatter during processing."))
        .arg(Arg::with_name("chatter_level")
             .long("chatter")
             .short("c")
             .value_name("LEVEL")
             .help("How much chatter to print when running.")
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
