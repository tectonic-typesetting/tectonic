// src/cli_driver.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate aho_corasick;
extern crate clap;
#[macro_use] extern crate tectonic;
extern crate termcolor;

use aho_corasick::{Automaton, AcAutomaton};
use clap::{Arg, ArgMatches, App};
use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process;

use tectonic::config::PersistentConfig;
use tectonic::digest::DigestData;
use tectonic::engines::IoEventBackend;
use tectonic::errors::{ErrorKind, Result, ResultExt};
use tectonic::io::{FilesystemIo, FilesystemPrimaryInputIo, GenuineStdoutIo, InputOrigin,
                   IoProvider, IoStack, MemoryIo, OpenResult};
use tectonic::io::itarbundle::{HttpITarIoFactory, ITarBundle};
use tectonic::io::stdstreams::BufferedPrimaryIo;
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
    primary_input: Box<IoProvider>,
    bundle: Option<Box<IoProvider>>,
    mem: MemoryIo,
    filesystem: FilesystemIo,
    genuine_stdout: Option<GenuineStdoutIo>,
    format_primary: Option<BufferedPrimaryIo>,
}

impl CliIoSetup {
    fn as_stack<'a> (&'a mut self) -> IoStack<'a> {
        let mut providers: Vec<&mut IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }

        providers.push(&mut *self.primary_input);
        providers.push(&mut self.mem);
        providers.push(&mut self.filesystem);

        if let Some(ref mut b) = self.bundle {
            providers.push(&mut **b);
        }

        IoStack::new(providers)
    }

    fn as_stack_for_format<'a> (&'a mut self, kickstart: &str) -> IoStack<'a> {
        let mut providers: Vec<&mut IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }


        self.format_primary = Some(BufferedPrimaryIo::from_text(kickstart));
        providers.push(self.format_primary.as_mut().unwrap());
        providers.push(&mut self.mem);

        if let Some(ref mut b) = self.bundle {
            providers.push(&mut **b);
        }

        IoStack::new(providers)
    }
}

/// The CliIoBuilder provides a convenient builder interface for specifying
/// the I/O setup.

struct CliIoBuilder {
    primary_input_path: Option<PathBuf>,
    filesystem_root: PathBuf,
    use_stdin: bool,
    bundle: Option<Box<IoProvider>>,
    use_genuine_stdout: bool,
    hidden_input_paths: HashSet<PathBuf>,
}

impl Default for CliIoBuilder {
    fn default() -> Self {
        CliIoBuilder {
            primary_input_path: None,
            filesystem_root: PathBuf::new(),
            use_stdin: false,
            bundle: None,
            use_genuine_stdout: false,
            hidden_input_paths: HashSet::new(),
        }
    }
}

impl CliIoBuilder {
    fn primary_input_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        if self.use_stdin {
            panic!("cannot use both stdin and primary_input_path");
        }

        self.primary_input_path = Some(path.as_ref().to_owned());
        self
    }

    fn primary_input_stdin(&mut self) -> &mut Self {
        if self.primary_input_path.is_some() {
            panic!("cannot use both primary_input_path and stdin");
        }

        self.use_stdin = true;
        self
    }

    fn filesystem_root<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.filesystem_root = path.as_ref().to_owned();
        self
    }

    fn bundle<T: 'static + IoProvider>(&mut self, bundle: T) -> &mut Self {
        self.bundle = Some(Box::new(bundle));
        self
    }

    fn boxed_bundle(&mut self, bundle: Box<IoProvider>) -> &mut Self {
        self.bundle = Some(bundle);
        self
    }

    fn use_genuine_stdout(&mut self, setting: bool) -> &mut Self {
        self.use_genuine_stdout = setting;
        self
    }

    fn hide_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.hidden_input_paths.insert(path.as_ref().to_owned());
        self
    }

    fn create(self) -> Result<CliIoSetup> {
        let pio: Box<IoProvider> = if self.use_stdin {
            Box::new(ctry!(BufferedPrimaryIo::from_stdin(); "error reading standard input"))
        } else if let Some(pip) = self.primary_input_path {
            Box::new(FilesystemPrimaryInputIo::new(&pip))
        } else {
            panic!("no primary input mechanism specified");
        };

        Ok(CliIoSetup {
            primary_input: pio,
            mem: MemoryIo::new(true),
            filesystem: FilesystemIo::new(&self.filesystem_root, false, true, self.hidden_input_paths),
            bundle: self.bundle,
            genuine_stdout: if self.use_genuine_stdout {
                Some(GenuineStdoutIo::new())
            } else {
                None
            },
            format_primary: None,
        })
    }
}


/// Different patterns with which files may have been accessed by the
/// underlying engines. Once a file is marked as ReadThenWritten or
/// WrittenThenRead, its pattern does not evolve further.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum AccessPattern {
    /// This file is only ever read.
    Read,

    /// This file is only ever written. This suggests that it is
    /// a final output of the processing session.
    Written,

    /// This file is read, then written. We call this a "circular" access
    /// pattern. Multiple passes of an engine will result in outputs that
    /// change if this file's contents change, or if the file did not exist at
    /// the time of the first pass.
    ReadThenWritten,

    /// This file is written, then read. We call this a "temporary" access
    /// pattern. This file is likely a temporary buffer that is not of
    /// interest to the user.
    WrittenThenRead,
}


/// A summary of the I/O that happened on a file. We record its access
/// pattern; where it came from, if it was used as an input; the cryptographic
/// digest of the file when it was last read; and the cryptographic digest of
/// the file as it was last written.
#[derive(Clone,Debug,Eq,PartialEq)]
struct FileSummary {
    access_pattern: AccessPattern,
    input_origin: InputOrigin,
    read_digest: Option<DigestData>,
    write_digest: Option<DigestData>,
    got_written_to_disk: bool,
}

impl FileSummary {
    fn new(access_pattern: AccessPattern, input_origin: InputOrigin) -> FileSummary {
        FileSummary {
            access_pattern: access_pattern,
            input_origin: input_origin,
            read_digest: None,
            write_digest: None,
            got_written_to_disk: false,
        }
    }
}


/// The CliIoEvents type implements the IoEventBackend. The CLI uses it to
/// figure out when to rerun the TeX engine; to figure out which files should
/// be written to disk; and to emit Makefile rules.
struct CliIoEvents(HashMap<OsString, FileSummary>);

impl CliIoEvents {
    fn new() -> CliIoEvents { CliIoEvents(HashMap::new()) }
}

impl IoEventBackend for CliIoEvents {
    fn output_opened(&mut self, name: &OsStr) {
        if let Some(summ) = self.0.get_mut(name) {
            summ.access_pattern = match summ.access_pattern {
                AccessPattern::Read => AccessPattern::ReadThenWritten,
                c => c, // identity mapping makes sense for remaining options
            };
            return;
        }

        self.0.insert(name.to_os_string(), FileSummary::new(AccessPattern::Written, InputOrigin::NotInput));
    }

    fn stdout_opened(&mut self) {
        // Life is easier if we track stdout in the same way that we do other
        // output files.

        if let Some(summ) = self.0.get_mut(OsStr::new("")) {
            summ.access_pattern = match summ.access_pattern {
                AccessPattern::Read => AccessPattern::ReadThenWritten,
                c => c, // identity mapping makes sense for remaining options
            };
            return;
        }

        self.0.insert(OsString::from(""), FileSummary::new(AccessPattern::Written, InputOrigin::NotInput));
    }

    fn output_closed(&mut self, name: OsString, digest: DigestData) {
        let summ = self.0.get_mut(&name).expect("closing file that wasn't opened?");
        summ.write_digest = Some(digest);
    }

    fn input_not_available(&mut self, name: &OsStr) {
        // For the purposes of file access pattern tracking, an attempt to
        // open a nonexistent file counts as a read of a zero-size file. I
        // don't see how such a file could have previously been written, but
        // let's use the full update logic just in case.

        if let Some(summ) = self.0.get_mut(name) {
            summ.access_pattern = match summ.access_pattern {
                AccessPattern::Written => AccessPattern::WrittenThenRead,
                c => c, // identity mapping makes sense for remaining options
            };
            return;
        }

        // Unlike other cases, here we need to fill in the read_digest. `None`
        // is not an appropriate value since, if the file is written and then
        // read again later, the `None` will be overwritten; but what matters
        // is the contents of the file the very first time it was read.
        let mut fs = FileSummary::new(AccessPattern::Read, InputOrigin::NotInput);
        fs.read_digest = Some(DigestData::of_nothing());
        self.0.insert(name.to_os_string(), fs);
    }

    fn input_opened(&mut self, name: &OsStr, origin: InputOrigin) {
        if let Some(summ) = self.0.get_mut(name) {
            summ.access_pattern = match summ.access_pattern {
                AccessPattern::Written => AccessPattern::WrittenThenRead,
                c => c, // identity mapping makes sense for remaining options
            };
            return;
        }

        self.0.insert(name.to_os_string(), FileSummary::new(AccessPattern::Read, origin));
    }

    //fn primary_input_opened(&mut self, _origin: InputOrigin) {}

    fn input_closed(&mut self, name: OsString, digest: Option<DigestData>) {
        let summ = self.0.get_mut(&name).expect("closing file that wasn't opened?");

        // It's what was in the file the *first* time that it was read that
        // matters, so don't replace the read digest if it's already got one.

        if summ.read_digest.is_none() {
            summ.read_digest = digest;
        }
    }
}


/// The ProcessingSession struct runs the whole show when we're actually
/// processing a file. It merges the command-line arguments and the persistent
/// configuration to figure out what exactly we're going to do.

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum OutputFormat {
    Aux,
    Xdv,
    Pdf,
    Format,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum PassSetting {
    Tex,
    Default,
    BibtexFirst,
}

struct ProcessingSession {
    io: CliIoSetup,
    events: CliIoEvents,

    /// If our primary input is an actual file on disk, this is its path.
    primary_input_path: Option<PathBuf>,

    /// This is the name of the input that we tell TeX. It is the basename of
    /// the UTF8-ified version of `primary_input_path`; or something anodyne
    /// if the latter is None. (Name, "texput.tex").
    primary_input_tex_path: String,

    /// This is the name of the format file to use. TeX has to open it by name
    /// internally, so it has to be String compatible.
    format_path: String,

    /// These are the paths of the various output files as TeX knows them --
    /// just `primary_input_tex_path` with the extension changed. We store
    /// them as OsStrings since that's what the main crate currently uses for
    /// TeX paths, even though I've since realized that it should really just
    /// use String.
    tex_aux_path: OsString,
    tex_xdv_path: OsString,
    tex_pdf_path: OsString,

    /// If we're writing out Makefile rules, this is where they go. The TeX
    /// engine doesn't know about this path at all.
    makefile_output_path: Option<PathBuf>,

    /// This is the path, the processed file will be saved at. It defaults 
    /// to the path of `primary_input_path` or `.` if STDIN is used.
    output_path: PathBuf,

    pass: PassSetting,
    output_format: OutputFormat,
    tex_rerun_specification: Option<usize>,
    keep_intermediates: bool,
    keep_logs: bool,
    noted_tex_warnings: bool,
    synctex_enabled: bool,
}


const DEFAULT_MAX_TEX_PASSES: usize = 6;

impl ProcessingSession {
    fn new(args: &ArgMatches, config: &PersistentConfig,
           status: &mut TermcolorStatusBackend) -> Result<ProcessingSession> {
        let format_path = args.value_of("format").unwrap();
        let tex_path = args.value_of_os("INPUT").unwrap();

        let output_format = match args.value_of("outfmt").unwrap() {
            "aux" => OutputFormat::Aux,
            "xdv" => OutputFormat::Xdv,
            "pdf" => OutputFormat::Pdf,
            "format" => OutputFormat::Format,
            _ => unreachable!()
        };

        let pass = match args.value_of("pass").unwrap() {
            "default" => PassSetting::Default,
            "bibtex_first" => PassSetting::BibtexFirst,
            "tex" => PassSetting::Tex,
            _ => unreachable!()
        };

        let reruns = match args.value_of("reruns") {
            Some(s) => Some(usize::from_str_radix(s, 10)?),
            None => None,
        };

        let makefile_output_path = args.value_of_os("makefile_rules").map(|s| s.into());

        // Input and path setup

        let mut io_builder = CliIoBuilder::default();
        let primary_input_path;
        let mut output_path;
        let tex_input_stem;

        if tex_path == "-" {
            io_builder.primary_input_stdin();

            primary_input_path = None;
            output_path = Path::new("");
            tex_input_stem = OsStr::new("texput.tex");
            tt_note!(status, "reading from standard input; outputs will appear under the base name \"texput\"");
        } else {
            let tex_path = Path::new(tex_path);
            io_builder.primary_input_path(&tex_path);

            primary_input_path = Some(tex_path.to_owned());

            tex_input_stem = match tex_path.file_name() {
                Some(fname) => fname,
                None => { return Err(ErrorKind::Msg(format!("can't figure out a basename for input path \"{}\"",
                                                            tex_path.to_string_lossy())).into()); },
            };

            if let Some(par) = tex_path.parent() {
                output_path = par;
                io_builder.filesystem_root(par);
            } else {
                return Err(ErrorKind::Msg(format!("can't figure out a parent directory for input path \"{}\"",
                                                  tex_path.to_string_lossy())).into());
            }
        }

        if let Some(dir) = args.value_of_os("outdir") {
            output_path = Path::new(dir);
            if !output_path.is_dir() {
                return Err(ErrorKind::Msg(format!("output directory \"{}\" does not exist", output_path.display())).into());
                }
        }

        let mut aux_path = Path::new(tex_input_stem).to_owned();
        aux_path.set_extension("aux");
        let mut xdv_path = aux_path.clone();
        xdv_path.set_extension("xdv");
        let mut pdf_path = aux_path.clone();
        pdf_path.set_extension("pdf");

        // Set up the rest of I/O.

        io_builder.use_genuine_stdout(args.is_present("print_stdout"));

        if let Some(items) = args.values_of_os("hide") {
            for v in items {
                io_builder.hide_path(v);
            }
        }

        if let Some(p) = args.value_of("bundle") {
            let zb = ctry!(ZipBundle::<File>::open(Path::new(&p)); "error opening bundle");
            io_builder.bundle(zb);
        } else if let Some(u) = args.value_of("web_bundle") {
            let tb = ITarBundle::<HttpITarIoFactory>::new(&u);
            io_builder.bundle(tb);
        } else {
            io_builder.boxed_bundle(config.default_io_provider(status)?);
        }

        let io = io_builder.create()?;

        // Ready to roll.

        Ok(ProcessingSession {
            io: io,
            events: CliIoEvents::new(),
            pass: pass,
            primary_input_path: primary_input_path,
            primary_input_tex_path: tex_input_stem.to_string_lossy().into_owned(),
            format_path: format_path.to_owned(),
            tex_aux_path: aux_path.into_os_string(),
            tex_xdv_path: xdv_path.into_os_string(),
            tex_pdf_path: pdf_path.into_os_string(),
            output_format: output_format,
            makefile_output_path: makefile_output_path,
            output_path: output_path.to_owned(),
            tex_rerun_specification: reruns,
            keep_intermediates: args.is_present("keep_intermediates"),
            keep_logs: args.is_present("keep_logs"),
            noted_tex_warnings: false,
            synctex_enabled: args.is_present("synctex"),
        })
    }


    /// Assess whether we need to rerun an engine. This is the case if there
    /// was a file that the engine read and then rewrote, and the rewritten
    /// version is different than the version that it read in.
    fn rerun_needed(&mut self, status: &mut TermcolorStatusBackend) -> Option<String> {
        // TODO: we should probably wire up diagnostics since I expect this
        // stuff could get finicky and we're going to want to be able to
        // figure out why rerun detection is breaking.

        for (name, info) in &self.events.0 {
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

        None
    }

    #[allow(dead_code)]
    fn _dump_access_info(&self, status: &mut TermcolorStatusBackend) {
        for (name, info) in &self.events.0 {
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
        // Do we need to generate the format file?

        let generate_format = if self.output_format == OutputFormat::Format {
            false
        } else {
            let fmt_result = {
                let mut stack = self.io.as_stack();
                stack.input_open_format(OsStr::new(&self.format_path), status)
            };

            match fmt_result {
                OpenResult::Ok(_) => false,
                OpenResult::NotAvailable => true,
                OpenResult::Err(e) => {
                    return Err(e).chain_err(|| format!("could not open format file {}", self.format_path));
                },
            }
        };

        if generate_format {
            tt_note!(status, "generating format \"{}\"", self.format_path);
            self.make_format_pass(status)?;
        }

        // Do the meat of the work.

        let result = match self.pass {
            PassSetting::Tex => self.tex_pass(None, status),
            PassSetting::Default => self.default_pass(false, status),
            PassSetting::BibtexFirst => self.default_pass(true, status),
        };

        if let Err(e) = result {
            self.write_files(None, status, true)?;
            return Err(e);
        };

        // Write output files and the first line of our Makefile output.

        let mut mf_dest_maybe = match self.makefile_output_path {
            Some(ref p) => Some(File::create(p)?),
            None => None
        };

        let n_skipped_intermediates = self.write_files(mf_dest_maybe.as_mut(), status, false)?;

        if n_skipped_intermediates > 0 {
            status.note_highlighted("Skipped writing ", &format!("{}", n_skipped_intermediates),
                                    " intermediate files (use --keep-intermediates to keep them)");
        }

        // Finish Makefile rules, maybe.

        if let Some(ref mut mf_dest) = mf_dest_maybe {
            ctry!(write!(mf_dest, ": "); "couldn't write to Makefile-rules file");

            if let Some(ref pip) = self.primary_input_path {
                ctry!(mf_dest.write_all(pip.as_os_str().as_bytes()); "couldn't write to Makefile-rules file");
            }

            for (name, info) in &self.events.0 {
                if info.input_origin != InputOrigin::Filesystem {
                    continue;
                }

                if info.got_written_to_disk {
                    // If the file originally came from the filesystem, and it
                    // was written as well as read, and we actually wrote it
                    // to disk, there's a circular dependency that's
                    // inappropriate to express in a Makefile. If it was
                    // "written" by the engine but we didn't actually write
                    // those modifications to disk, we're OK. If there's a
                    // two-stage compilation involving the .aux file, the
                    // latter case is what arises unless --keep-intermediates
                    // is specified.
                    tt_warning!(status, "omitting circular Makefile dependency for {}", name.to_string_lossy());
                    continue;
                }

                ctry!(write!(mf_dest, " \\\n  {}", self.output_path.join(name).display()); "couldn't write to Makefile-rules file");
            }

            ctry!(writeln!(mf_dest, ""); "couldn't write to Makefile-rules file");
        }

        // All done.

        Ok(0)
    }


    fn write_files(&mut self, mut mf_dest_maybe: Option<&mut File>, status: &mut
                   TermcolorStatusBackend, only_logs: bool) -> Result<u32> {
        let mut n_skipped_intermediates = 0;
        for (name, contents) in &*self.io.mem.files.borrow() {
            if name == self.io.mem.stdout_key() {
                continue;
            }

            let sname = name.to_string_lossy();
            let summ = self.events.0.get_mut(name).unwrap();

            if !only_logs && (self.output_format == OutputFormat::Aux) {
                // In this mode we're only writing the .aux file. I initially
                // wanted to be clever-ish and output all auxiliary-type
                // files, but doing so ended up causing non-obvious problems
                // for my use case, which involves using Ninja to manage
                // dependencies.
                if !sname.ends_with(".aux") {
                    continue;
                }
            } else if summ.access_pattern != AccessPattern::Written && !self.keep_intermediates {
                n_skipped_intermediates += 1;
                continue;
            }

            let is_logfile = sname.ends_with(".log") || sname.ends_with(".blg");

            if is_logfile && !self.keep_logs {
                continue;
            }

            if !is_logfile && only_logs {
                continue;
            }

            if contents.is_empty() {
                status.note_highlighted("Not writing ", &sname, ": it would be empty.");
                continue;
            }

            let real_path = self.output_path.join(name);


            status.note_highlighted("Writing ", &real_path.to_string_lossy(), &format!(" ({} bytes)", contents.len()));

            let mut f = File::create(&real_path)?;
            f.write_all(contents)?;
            summ.got_written_to_disk = true;

            if let Some(ref mut mf_dest) = mf_dest_maybe {
                // Maybe it'd be better to have this just be a warning? But if
                // the program is supposed to write the file, you don't want
                // it exiting with error code zero if it couldn't do that
                // successfully.
                //
                // Not quite sure why, but I can't pull out the target path
                // here. I think 'self' is borrow inside the loop?
                ctry!(write!(mf_dest, "{} ", real_path.to_string_lossy()); "couldn't write to Makefile-rules file");
            }
        }
        Ok(n_skipped_intermediates)
    }

    /// The "default" pass really runs a bunch of sub-passes. It is a "Do What
    /// I Mean" operation.
    fn default_pass(&mut self, bibtex_first: bool, status: &mut TermcolorStatusBackend) -> Result<i32> {
        // If `bibtex_first` is true, we start by running bibtex, and run
        // proceed with the standard rerun logic. Otherwise, we run TeX,
        // auto-detect whether we need to run bibtex, possibly run it, and
        // then go ahead.

        let mut rerun_result = if bibtex_first {
            self.bibtex_pass(status)?;
            Some(String::new())
        } else {
            self.tex_pass(None, status)?;
            //

            let use_bibtex = {
                if let Some(auxdata) = self.io.mem.files.borrow().get(&self.tex_aux_path) {
                    let cite_aut = AcAutomaton::new(vec!["\\bibdata"]);
                    cite_aut.find(auxdata).next().is_some()
                } else {
                    false
                }
            };

            if use_bibtex {
                self.bibtex_pass(status)?;
                Some(String::new())
            } else {
                self.rerun_needed(status)
            }
        };

        // Now we enter the main rerun loop.

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
            for summ in self.events.0.values_mut() {
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


    /// Use the TeX engine to generate a format file.
    fn make_format_pass(&mut self, status: &mut TermcolorStatusBackend) -> Result<i32> {
        if self.io.bundle.is_none() {
            return Err(ErrorKind::Msg("cannot create formats without using a bundle".to_owned()).into())
        }

        // PathBuf.file_stem() doesn't do what we want since it only strips
        // one extension. As of 1.17, the compiler needs a type annotation for
        // some reason, which is why we use the `r` variable.
        let r: Result<&str> = self.format_path.splitn(2, '.').next().ok_or_else(
            || ErrorKind::Msg(format!("incomprehensible format file name \"{}\"", self.format_path)).into()
        );
        let stem = r?;

        let result = {
            let mut stack = self.io.as_stack_for_format(&format!("\\input tectonic-format-{}.tex", stem));
            TexEngine::new()
                    .halt_on_error_mode(true)
                    .initex_mode(true)
                    .process(&mut stack, &mut self.events, status, "UNUSED.fmt.gz", "texput")
        };

        match result {
            Ok(TexResult::Spotless) => {},
            Ok(TexResult::Warnings) => {
                tt_warning!(status, "warnings were issued by the TeX engine; use --print and/or --keep-logs for details.");
            },
            Ok(TexResult::Errors) => {
                tt_error!(status, "errors were issued by the TeX engine; use --print and/or --keep-logs for details.");
                return Err(ErrorKind::Msg("unhandled TeX engine error".to_owned()).into());
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

        // Now we can write the format file to its special location. In
        // principle we could stream the format file directly to the staging
        // area as we ran the TeX engine, but we don't bother.

        let bundle = &mut *self.io.bundle.as_mut().unwrap();

        for (name, contents) in &*self.io.mem.files.borrow() {
            if name == self.io.mem.stdout_key() {
                continue;
            }

            let sname = name.to_string_lossy();

            if !sname.ends_with(".fmt.gz") {
                continue;
            }

            // Note that we intentionally pass 'stem', not 'name'.
            ctry!(bundle.write_format(stem, contents, status); "cannot write format file {}", sname);
        }

        // All done. Clear the memory layer since this was a special preparatory step.
        self.io.mem.files.borrow_mut().clear();

        Ok(0)
    }


    /// Run one pass of the TeX engine.
    fn tex_pass(&mut self, rerun_explanation: Option<&str>, status: &mut TermcolorStatusBackend) -> Result<i32> {
        let result = {
            let mut stack = self.io.as_stack();
            if let Some(s) = rerun_explanation {
                status.note_highlighted("Rerunning ", "TeX", &format!(" because {} ...", s));
            } else {
                status.note_highlighted("Running ", "TeX", " ...");
            }

            TexEngine::new()
                    .halt_on_error_mode(true)
                    .initex_mode(self.output_format == OutputFormat::Format)
                    .synctex(self.synctex_enabled)
                    .process(&mut stack, &mut self.events, status, &self.format_path, &self.primary_input_tex_path)
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
            engine.process(&mut stack, &mut self.events, status,
                           &self.tex_aux_path.to_str().unwrap())
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
            engine.process(&mut stack, &mut self.events, status,
                           &self.tex_xdv_path.to_str().unwrap(), &self.tex_pdf_path.to_str().unwrap())?;
        }

        self.io.mem.files.borrow_mut().remove(&self.tex_xdv_path);
        Ok(0)
    }
}


fn inner(matches: ArgMatches, config: PersistentConfig, status: &mut TermcolorStatusBackend) -> Result<i32> {
    let mut sess = ProcessingSession::new(&matches, &config, status)?;
    sess.run(status)
}


fn main() {
    let matches = App::new("Tectonic")
        .version("0.1.7")
        .about("Process a (La)TeX document.")
        .arg(Arg::with_name("format")
             .long("format")
             .value_name("PATH")
             .help("The name of the \"format\" file used to initialize the TeX engine.")
             .default_value("latex"))
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
             .possible_values(&["pdf", "xdv", "aux", "format"])
             .default_value("pdf"))
        .arg(Arg::with_name("makefile_rules")
             .long("makefile-rules")
             .value_name("PATH")
             .help("Write Makefile-format rules expressing the dependencies of this run to <PATH>."))
        .arg(Arg::with_name("pass")
             .long("pass")
             .value_name("PASS")
             .help("Which engines to run.")
             .possible_values(&["default", "tex", "bibtex_first"])
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
        .arg(Arg::with_name("synctex")
             .long("synctex")
             .help("Generate SyncTeX data."))
        .arg(Arg::with_name("hide")
             .long("hide")
             .value_name("PATH")
             .multiple(true)
             .number_of_values(1)
             .help("Tell the engine that no file at <PATH> exists, if it tries to read it."))
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
        .arg(Arg::with_name("outdir")
             .long("outdir")
             .short("o")
             .value_name("OUTDIR")
             .help("The directory in which to place output files. [default: the directory containing INPUT]"))
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

    tt_note!(status, "this is a BETA release; ask questions and report bugs at https://tectonic.newton.cx/");

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
