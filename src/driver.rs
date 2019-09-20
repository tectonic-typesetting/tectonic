// src/driver.rs -- utilities for running and rerunning the tex engine
// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! This module contains the high-level interface that ties together the various engines. The main
//! struct is [`ProcessingSession`], which knows how to run (and re-run if
//! necessary) the various engines in the right order.
//!
//! For an example of how to use this module, see `src/bin/tectonic.rs`, which contains tectonic's main
//! CLI program.

use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;

use crate::digest::DigestData;
use crate::engines::IoEventBackend;
use crate::errors::{ErrorKind, Result, ResultExt};
use crate::io::{Bundle, InputOrigin, IoProvider, IoSetup, IoSetupBuilder, OpenResult};
use crate::status::StatusBackend;
use crate::{ctry, errmsg, tt_error, tt_note, tt_warning};
use crate::{BibtexEngine, Spx2HtmlEngine, TexEngine, TexResult, XdvipdfmxEngine};
use std::result::Result as StdResult;

/// Different patterns with which files may have been accessed by the
/// underlying engines. Once a file is marked as ReadThenWritten or
/// WrittenThenRead, its pattern does not evolve further.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessPattern {
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileSummary {
    access_pattern: AccessPattern,

    /// If this file was read, where did it come from?
    pub input_origin: InputOrigin,

    /// If this file was read, this is the digest of its contents at the time it was *first* read.
    /// The "first" is significant for files that were read and then written (for example, `.aux`
    /// files).
    ///
    /// There's some chance that this will be `None` even if the file was read. Tectonic makes an
    /// effort to compute the digest as the data is being read from the file, but this can fail if
    /// tex decides to seek in the file as it is being written.
    pub read_digest: Option<DigestData>,

    /// If this file was written, this is the digest of its contents at the time it was last
    /// written.
    pub write_digest: Option<DigestData>,
    got_written_to_disk: bool,
}

impl FileSummary {
    fn new(access_pattern: AccessPattern, input_origin: InputOrigin) -> FileSummary {
        FileSummary {
            access_pattern,
            input_origin,
            read_digest: None,
            write_digest: None,
            got_written_to_disk: false,
        }
    }
}

/// The IoEvents type implements the IoEventBackend. It is used to figure out when to rerun the TeX
/// engine, to figure out which files should be written to disk, and to emit Makefile rules.
pub struct IoEvents(pub HashMap<OsString, FileSummary>);

impl IoEvents {
    fn new() -> IoEvents {
        IoEvents(HashMap::new())
    }
}

impl IoEventBackend for IoEvents {
    fn output_opened(&mut self, name: &OsStr) {
        if let Some(summ) = self.0.get_mut(name) {
            summ.access_pattern = match summ.access_pattern {
                AccessPattern::Read => AccessPattern::ReadThenWritten,
                c => c, // identity mapping makes sense for remaining options
            };
            return;
        }

        self.0.insert(
            name.to_os_string(),
            FileSummary::new(AccessPattern::Written, InputOrigin::NotInput),
        );
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

        self.0.insert(
            OsString::from(""),
            FileSummary::new(AccessPattern::Written, InputOrigin::NotInput),
        );
    }

    fn output_closed(&mut self, name: OsString, digest: DigestData) {
        let summ = self
            .0
            .get_mut(&name)
            .expect("closing file that wasn't opened?");
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

        self.0.insert(
            name.to_os_string(),
            FileSummary::new(AccessPattern::Read, origin),
        );
    }

    //fn primary_input_opened(&mut self, _origin: InputOrigin) {}

    fn input_closed(&mut self, name: OsString, digest: Option<DigestData>) {
        let summ = self
            .0
            .get_mut(&name)
            .expect("closing file that wasn't opened?");

        // It's what was in the file the *first* time that it was read that
        // matters, so don't replace the read digest if it's already got one.

        if summ.read_digest.is_none() {
            summ.read_digest = digest;
        }
    }
}

/// The different types of output files that tectonic knows how to produce.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    /// A '.aux' file.
    Aux,
    /// A '.html' file.
    Html,
    /// An extended DVI file.
    Xdv,
    /// A '.pdf' file.
    Pdf,
    /// A '.fmt' file, for initializing the TeX engine.
    Format,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(a_str: &str) -> StdResult<Self, Self::Err> {
        match a_str {
            "aux" => Ok(OutputFormat::Aux),
            "html" => Ok(OutputFormat::Html),
            "xdv" => Ok(OutputFormat::Xdv),
            "pdf" => Ok(OutputFormat::Pdf),
            "fmt" => Ok(OutputFormat::Format),
            _ => Err("unsupported or unknown format"),
        }
    }
}

impl Default for OutputFormat {
    fn default() -> OutputFormat {
        OutputFormat::Pdf
    }
}

/// The different types of "passes" that [`ProcessingSession`] knows how to run. See
/// [`ProcessingSession::run`] for more details.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PassSetting {
    /// The default pass, which repeatedly runs TeX and BibTeX until it doesn't need to any more.
    Default,
    /// Just run the TeX engine once.
    Tex,
    /// Like the default pass, but runs BibTeX once first, before doing anything else.
    BibtexFirst,
}

impl Default for PassSetting {
    fn default() -> PassSetting {
        PassSetting::Default
    }
}

impl FromStr for PassSetting {
    type Err = &'static str;

    fn from_str(a_str: &str) -> StdResult<Self, Self::Err> {
        match a_str {
            "default" => Ok(PassSetting::Default),
            "bibtex_first" => Ok(PassSetting::BibtexFirst),
            "tex" => Ok(PassSetting::Tex),
            _ => Err("unsupported or unknown pass setting"),
        }
    }
}

/// Different places from which the "primary input" might originate.
#[derive(Clone, Debug, Eq, PartialEq)]
enum PrimaryInputMode {
    /// This process's standard input.
    Stdin,

    /// A path on the filesystem.
    Path(PathBuf),

    /// An in-memory buffer.
    Buffer(Vec<u8>),
}

impl Default for PrimaryInputMode {
    fn default() -> PrimaryInputMode {
        PrimaryInputMode::Stdin
    }
}

/// Different places where the output files might land.
#[derive(Clone, Debug, Eq, PartialEq)]
enum OutputDestination {
    /// The "sensible" default. Files will land in the same directory as the
    /// input file, or the current working directory if the input is something
    /// without a path (such as standard input).
    Default,

    /// Files should land in this particular directory.
    Path(PathBuf),

    /// Files will not be written to disk. The code running the engine should
    /// examine the memory layer of the I/O stack to obtain the output files.
    Nowhere,
}

impl Default for OutputDestination {
    fn default() -> OutputDestination {
        OutputDestination::Default
    }
}

/// A builder-style interface for creating a [`ProcessingSession`].
#[derive(Default)]
pub struct ProcessingSessionBuilder {
    primary_input: PrimaryInputMode,
    tex_input_name: Option<String>,
    output_dest: OutputDestination,
    format_name: Option<String>,
    format_cache_path: Option<PathBuf>,
    output_format: OutputFormat,
    makefile_output_path: Option<PathBuf>,
    hidden_input_paths: HashSet<PathBuf>,
    pass: PassSetting,
    reruns: Option<usize>,
    print_stdout: bool,
    bundle: Option<Box<dyn Bundle>>,
    keep_intermediates: bool,
    keep_logs: bool,
    synctex: bool,
}

impl ProcessingSessionBuilder {
    /// Sets the path to the primary input file.
    ///
    /// If a primary input path is not specified, we will default to reading it from stdin.
    pub fn primary_input_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.primary_input = PrimaryInputMode::Path(p.as_ref().to_owned());
        self
    }

    /// Sets the primary input to be a caller-specified buffer.
    ///
    /// If neither this nor a primary input path is specified, we will default
    /// to reading the primary input from stdin.
    pub fn primary_input_buffer(&mut self, buf: &[u8]) -> &mut Self {
        self.primary_input = PrimaryInputMode::Buffer(buf.to_owned());
        self
    }

    /// Sets the name of the main input file.
    ///
    /// This value will be used to infer the names of the output files; for example, if
    /// `tex_input_name` is set to `"texput.tex"` then the pdf output file will be `"texput.pdf"`.
    /// As such, this parameter is mandatory, even if the real input is coming from stdin (if it is
    /// not provided, [`create`] will panic).
    pub fn tex_input_name(&mut self, s: &str) -> &mut Self {
        self.tex_input_name = Some(s.to_owned());
        self
    }

    /// A path to the directory where output files should be created.
    ///
    /// This will default to the directory containing `primary_input_path`, or
    /// the current working directory if the primary input is coming from
    /// stdin.
    pub fn output_dir<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.output_dest = OutputDestination::Path(p.as_ref().to_owned());
        self
    }

    /// Indicate that output files should not be written to disk.
    ///
    /// By default, output files will be written to the directory containing
    /// `primary_input_path`, or the current working directory if the primary
    /// input is coming from stdin.
    pub fn do_not_write_output_files(&mut self) -> &mut Self {
        self.output_dest = OutputDestination::Nowhere;
        self
    }

    /// The name of the `.fmt` file used to initialize the TeX engine.
    ///
    /// This file does not necessarily have to exist already; it will be created if it doesn't.
    /// This parameter is mandatory (if it is not provided, [`create`] will panic).
    pub fn format_name(&mut self, p: &str) -> &mut Self {
        self.format_name = Some(p.to_owned());
        self
    }

    /// Sets the path to the format file cache.
    ///
    /// This is used to, well, cache format files, which are generated as
    /// needed from the backing bundle. Defaults to the same directory as the
    /// input file, or PWD if the input is a non-file (such as standard
    /// input).
    pub fn format_cache_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.format_cache_path = Some(p.as_ref().to_owned());
        self
    }

    /// The type of output to create.
    pub fn output_format(&mut self, f: OutputFormat) -> &mut Self {
        self.output_format = f;
        self
    }

    /// If set, a makefile will be written out at the given path.
    pub fn makefile_output_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.makefile_output_path = Some(p.as_ref().to_owned());
        self
    }

    /// Which kind of pass should the `ProcessingSession` run? Defaults to `PassSetting::Default`
    /// (duh).
    pub fn pass(&mut self, p: PassSetting) -> &mut Self {
        self.pass = p;
        self
    }

    /// If set, and if the pass is set to `PassSetting::Default`, the TeX engine will be re-run
    /// *exactly* this many times.
    ///
    /// If `reruns` is unset, we will auto-detect how many times the TeX engine needs to be re-run.
    pub fn reruns(&mut self, r: usize) -> &mut Self {
        self.reruns = Some(r);
        self
    }

    /// If set to `true`, stdout from the TeX engine will be forwarded to actual stdout. (By
    /// default, it will be suppressed.)
    pub fn print_stdout(&mut self, p: bool) -> &mut Self {
        self.print_stdout = p;
        self
    }

    /// Marks a path as hidden, meaning that the TeX engine will pretend that it doesn't exist in
    /// the filesystem.
    pub fn hide<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.hidden_input_paths.insert(p.as_ref().to_owned());
        self
    }

    /// Sets the bundle, which the various engines will use for finding style files, font files,
    /// etc.
    pub fn bundle(&mut self, b: Box<dyn Bundle>) -> &mut Self {
        self.bundle = Some(b);
        self
    }

    /// If set to `true`, various intermediate files will be written out to the filesystem.
    pub fn keep_intermediates(&mut self, k: bool) -> &mut Self {
        self.keep_intermediates = k;
        self
    }

    /// If set to `true`, '.log' and '.blg' files will be written out to the filesystem.
    pub fn keep_logs(&mut self, k: bool) -> &mut Self {
        self.keep_logs = k;
        self
    }

    /// If set to `true`, tex files will be compiled using synctex information.
    pub fn synctex(&mut self, s: bool) -> &mut Self {
        self.synctex = s;
        self
    }

    /// Creates a `ProcessingSession`.
    pub fn create(self, status: &mut dyn StatusBackend) -> Result<ProcessingSession> {
        let mut io = IoSetupBuilder::default();
        io.bundle(self.bundle.expect("a bundle must be specified"))
            .use_genuine_stdout(self.print_stdout);
        for p in &self.hidden_input_paths {
            io.hide_path(p);
        }

        let (primary_input_path, default_output_path) = match self.primary_input {
            PrimaryInputMode::Path(p) => {
                io.primary_input_path(&p);

                // Set the filesystem root (that's the directory we'll search
                // for files in) to be the same directory as the main input
                // file.
                let parent = match p.parent() {
                    Some(parent) => parent.to_owned(),
                    None => {
                        return Err(errmsg!(
                            "can't figure out a parent directory for input path \"{}\"",
                            p.to_string_lossy()
                        ));
                    }
                };

                io.filesystem_root(&parent);
                (Some(p), parent)
            }

            PrimaryInputMode::Stdin => {
                // If the main input file is stdin, we don't set a filesystem
                // root, which means we'll default to the current working
                // directory.
                io.primary_input_stdin();
                (None, "".into())
            }

            PrimaryInputMode::Buffer(buf) => {
                // Same behavior as with stdin.
                io.primary_input_buffer(buf);
                (None, "".into())
            }
        };

        let output_path = match self.output_dest {
            OutputDestination::Default => Some(default_output_path),
            OutputDestination::Path(p) => Some(p),
            OutputDestination::Nowhere => None,
        };

        if let Some(ref p) = self.format_cache_path {
            io.format_cache_path(p);
        }

        let tex_input_name = self
            .tex_input_name
            .expect("tex_input_name must be specified");
        let mut aux_path = PathBuf::from(tex_input_name.clone());
        aux_path.set_extension("aux");
        let mut xdv_path = aux_path.clone();
        xdv_path.set_extension(if self.output_format == OutputFormat::Html {
            "spx"
        } else {
            "xdv"
        });
        let mut pdf_path = aux_path.clone();
        pdf_path.set_extension("pdf");

        Ok(ProcessingSession {
            io: io.create(status)?,
            events: IoEvents::new(),
            pass: self.pass,
            primary_input_path,
            primary_input_tex_path: tex_input_name,
            format_name: self.format_name.unwrap(),
            tex_aux_path: aux_path.into_os_string(),
            tex_xdv_path: xdv_path.into_os_string(),
            tex_pdf_path: pdf_path.into_os_string(),
            output_format: self.output_format,
            makefile_output_path: self.makefile_output_path,
            output_path,
            tex_rerun_specification: self.reruns,
            keep_intermediates: self.keep_intermediates,
            keep_logs: self.keep_logs,
            noted_tex_warnings: false,
            synctex_enabled: self.synctex,
        })
    }
}

/// The ProcessingSession struct runs the whole show when we're actually
/// processing a file. It understands, for example, the need to re-run the TeX
/// engine if the `.aux` file changed.
pub struct ProcessingSession {
    /// This contains the full I/O setup of the processing session. After
    /// running the session, you can inspect this to see what I/O was
    /// produced. (For example, the memory layer might contain some files that
    /// were produced by the TeX engine but not actually written to disk.)
    pub io: IoSetup,

    /// This contains all the I/O events that occurred while processing.
    pub events: IoEvents,

    /// If our primary input is an actual file on disk, this is its path.
    primary_input_path: Option<PathBuf>,

    /// This is the name of the input that we tell TeX. It is the basename of
    /// the UTF8-ified version of `primary_input_path`; or something anodyne
    /// if the latter is None. (Name, "texput.tex").
    primary_input_tex_path: String,

    /// This is the name of the format file to use. TeX has to open it by name
    /// internally, so it has to be String compatible.
    format_name: String,

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

    /// This is the path that the processed file will be saved at. It defaults
    /// to the path of `primary_input_path` or `.` if STDIN is used. If set to
    /// None, the output files will not be saved to disk — in which case, the
    /// caller should access the memory layer of the `io` field to gain access
    /// to the output files.
    output_path: Option<PathBuf>,

    pass: PassSetting,
    output_format: OutputFormat,
    tex_rerun_specification: Option<usize>,
    keep_intermediates: bool,
    keep_logs: bool,
    noted_tex_warnings: bool,
    synctex_enabled: bool,
}

const DEFAULT_MAX_TEX_PASSES: usize = 6;
const ALWAYS_INTERMEDIATE_EXTENSIONS: &[&str] = &[
    ".snm", ".toc", // generated by Beamer
];

impl ProcessingSession {
    /// Assess whether we need to rerun an engine. This is the case if there
    /// was a file that the engine read and then rewrote, and the rewritten
    /// version is different than the version that it read in.
    fn rerun_needed<S: StatusBackend>(&mut self, status: &mut S) -> Option<String> {
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
                        tt_warning!(
                            status,
                            "internal consistency problem when checking if {} changed",
                            name.to_string_lossy()
                        );
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
    fn _dump_access_info<S: StatusBackend>(&self, status: &mut S) {
        for (name, info) in &self.events.0 {
            if info.access_pattern != AccessPattern::Read {
                let r = match info.read_digest {
                    Some(ref d) => d.to_string(),
                    None => "-".into(),
                };
                let w = match info.write_digest {
                    Some(ref d) => d.to_string(),
                    None => "-".into(),
                };
                tt_note!(
                    status,
                    "ACCESS: {} {:?} {:?} {:?}",
                    name.to_string_lossy(),
                    info.access_pattern,
                    r,
                    w
                );
            }
        }
    }

    /// Runs the session, generating the desired outputs.
    ///
    /// What this does depends on which [`PassSetting`] you asked for. The most common choice is
    /// `PassSetting::Default`, in which case this method does the following:
    ///
    /// - if a `.fmt` file does not yet exist, generate one and cache it
    /// - run the TeX engine once
    /// - run BibTeX, if it seems to be required
    /// - repeat the last two steps as often as needed
    /// - write the output files to disk, including a Makefile if it was requested.
    pub fn run<S: StatusBackend>(&mut self, status: &mut S) -> Result<()> {
        // Do we need to generate the format file?

        let generate_format = if self.output_format == OutputFormat::Format {
            false
        } else {
            let fmt_result = {
                let mut stack = self.io.as_stack();
                stack.input_open_format(OsStr::new(&self.format_name), status)
            };

            match fmt_result {
                OpenResult::Ok(_) => false,
                OpenResult::NotAvailable => true,
                OpenResult::Err(e) => {
                    return Err(e)
                        .chain_err(|| format!("could not open format file {}", self.format_name));
                }
            }
        };

        if generate_format {
            tt_note!(status, "generating format \"{}\"", self.format_name);
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
            Some(ref p) => {
                if self.output_path.is_none() {
                    tt_warning!(
                        status,
                        "requested to generate Makefile rules, but no files written to disk!"
                    );
                    None
                } else {
                    Some(File::create(p)?)
                }
            }

            None => None,
        };

        let n_skipped_intermediates = self.write_files(mf_dest_maybe.as_mut(), status, false)?;

        if n_skipped_intermediates > 0 {
            status.note_highlighted(
                "Skipped writing ",
                &format!("{}", n_skipped_intermediates),
                " intermediate files (use --keep-intermediates to keep them)",
            );
        }

        // Finish Makefile rules, maybe.

        if let Some(ref mut mf_dest) = mf_dest_maybe {
            ctry!(write!(mf_dest, ": "); "couldn't write to Makefile-rules file");

            if let Some(ref pip) = self.primary_input_path {
                ctry!(mf_dest.write_all(pip.to_string_lossy().as_ref().as_bytes()); "couldn't write to Makefile-rules file");
            }

            // The check above ensures that this is never None.
            let root = self.output_path.as_ref().unwrap();

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
                    tt_warning!(
                        status,
                        "omitting circular Makefile dependency for {}",
                        name.to_string_lossy()
                    );
                    continue;
                }

                ctry!(write!(mf_dest, " \\\n  {}", root.join(name).display()); "couldn't write to Makefile-rules file");
            }

            ctry!(writeln!(mf_dest, ""); "couldn't write to Makefile-rules file");
        }

        // All done.

        Ok(())
    }

    fn write_files<S: StatusBackend>(
        &mut self,
        mut mf_dest_maybe: Option<&mut File>,
        status: &mut S,
        only_logs: bool,
    ) -> Result<u32> {
        let root = match self.output_path {
            Some(ref p) => p,

            None => {
                // We were told not to write anything!
                return Ok(0);
            }
        };

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
            } else if !self.keep_intermediates
                && (summ.access_pattern != AccessPattern::Written
                    || ALWAYS_INTERMEDIATE_EXTENSIONS
                        .iter()
                        .any(|ext| sname.ends_with(ext)))
            {
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

            let real_path = root.join(name);
            status.note_highlighted(
                "Writing ",
                &real_path.to_string_lossy(),
                &format!(" ({} bytes)", contents.len()),
            );

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
    fn default_pass<S: StatusBackend>(
        &mut self,
        bibtex_first: bool,
        status: &mut S,
    ) -> Result<i32> {
        // If `bibtex_first` is true, we start by running bibtex, and run
        // proceed with the standard rerun logic. Otherwise, we run TeX,
        // auto-detect whether we need to run bibtex, possibly run it, and
        // then go ahead.

        let mut rerun_result = if bibtex_first {
            self.bibtex_pass(status)?;
            Some(String::new())
        } else {
            self.tex_pass(None, status)?;

            if self.use_bibtex() {
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
                    }
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
                    tt_warning!(
                        status,
                        "TeX rerun seems needed, but stopping at {} passes",
                        DEFAULT_MAX_TEX_PASSES
                    );
                    break;
                }
            }
        }

        // And finally, xdvipdfmx or spx2html. Maybe.

        if let OutputFormat::Pdf = self.output_format {
            self.xdvipdfmx_pass(status)?;
        } else if let OutputFormat::Html = self.output_format {
            self.spx2html_pass(status)?;
        }

        Ok(0)
    }

    fn use_bibtex(&self) -> bool {
        const BIBDATA: &[u8] = b"\\bibdata";

        self.io
            .mem
            .files
            .borrow()
            .get(&self.tex_aux_path)
            .map(|data| {
                // We used to use aho-corasick crate here, but it was removed to reduce the code
                // size.
                data.windows(BIBDATA.len()).any(|s| s == BIBDATA)
            })
            .unwrap_or(false)
    }

    /// Use the TeX engine to generate a format file.
    fn make_format_pass<S: StatusBackend>(&mut self, status: &mut S) -> Result<i32> {
        if self.io.bundle.is_none() {
            return Err(
                ErrorKind::Msg("cannot create formats without using a bundle".to_owned()).into(),
            );
        }

        if self.io.format_cache.is_none() {
            return Err(ErrorKind::Msg(
                "cannot create formats without having a place to save them".to_owned(),
            )
            .into());
        }

        // PathBuf.file_stem() doesn't do what we want since it only strips
        // one extension. As of 1.17, the compiler needs a type annotation for
        // some reason, which is why we use the `r` variable.
        let r: Result<&str> = self.format_name.splitn(2, '.').next().ok_or_else(|| {
            ErrorKind::Msg(format!(
                "incomprehensible format file name \"{}\"",
                self.format_name
            ))
            .into()
        });
        let stem = r?;

        let result = {
            let mut stack = self
                .io
                .as_stack_for_format(&format!("tectonic-format-{}.tex", stem));
            TexEngine::new()
                .halt_on_error_mode(true)
                .initex_mode(true)
                .process(&mut stack, &mut self.events, status, "UNUSED.fmt", "texput")
        };

        match result {
            Ok(TexResult::Spotless) => {}
            Ok(TexResult::Warnings) => {
                tt_warning!(status, "warnings were issued by the TeX engine; use --print and/or --keep-logs for details.");
            }
            Ok(TexResult::Errors) => {
                tt_error!(status, "errors were issued by the TeX engine; use --print and/or --keep-logs for details.");
                return Err(ErrorKind::Msg("unhandled TeX engine error".to_owned()).into());
            }
            Err(e) => {
                return Err(e.chain_err(|| ErrorKind::EngineError("TeX")));
            }
        }

        // Now we can write the format file to its special location. In
        // principle we could stream the format file directly to the staging
        // area as we ran the TeX engine, but we don't bother.

        let format_cache = &mut *self.io.format_cache.as_mut().unwrap();

        for (name, contents) in &*self.io.mem.files.borrow() {
            if name == self.io.mem.stdout_key() {
                continue;
            }

            let sname = name.to_string_lossy();

            if !sname.ends_with(".fmt") {
                continue;
            }

            // Note that we intentionally pass 'stem', not 'name'.
            ctry!(format_cache.write_format(stem, contents, status); "cannot write format file {}", sname);
        }

        // All done. Clear the memory layer since this was a special preparatory step.
        self.io.mem.files.borrow_mut().clear();

        Ok(0)
    }

    /// Run one pass of the TeX engine.
    fn tex_pass<S: StatusBackend>(
        &mut self,
        rerun_explanation: Option<&str>,
        status: &mut S,
    ) -> Result<i32> {
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
                .semantic_pagination(self.output_format == OutputFormat::Html)
                .process(
                    &mut stack,
                    &mut self.events,
                    status,
                    &self.format_name,
                    &self.primary_input_tex_path,
                )
        };

        match result {
            Ok(TexResult::Spotless) => {}
            Ok(TexResult::Warnings) => {
                if !self.noted_tex_warnings {
                    tt_note!(status, "warnings were issued by the TeX engine; use --print and/or --keep-logs for details.");
                    self.noted_tex_warnings = true;
                }
            }
            Ok(TexResult::Errors) => {
                if !self.noted_tex_warnings {
                    // Weakness: if a first pass produces warnings and a
                    // second pass produces ignored errors, we won't say so.
                    tt_warning!(
                        status,
                        "errors were issued by the TeX engine, but were ignored; \
                         use --print and/or --keep-logs for details."
                    );
                    self.noted_tex_warnings = true;
                }
            }
            Err(e) => {
                return Err(e.chain_err(|| ErrorKind::EngineError("TeX")));
            }
        }

        Ok(0)
    }

    fn bibtex_pass<S: StatusBackend>(&mut self, status: &mut S) -> Result<i32> {
        let result = {
            let mut stack = self.io.as_stack();
            let mut engine = BibtexEngine::new();
            status.note_highlighted("Running ", "BibTeX", " ...");
            engine.process(
                &mut stack,
                &mut self.events,
                status,
                &self.tex_aux_path.to_str().unwrap(),
            )
        };

        match result {
            Ok(TexResult::Spotless) => {}
            Ok(TexResult::Warnings) => {
                tt_note!(
                    status,
                    "warnings were issued by BibTeX; use --print and/or --keep-logs for details."
                );
            }
            Ok(TexResult::Errors) => {
                tt_warning!(
                    status,
                    "errors were issued by BibTeX, but were ignored; \
                     use --print and/or --keep-logs for details."
                );
            }
            Err(e) => {
                return Err(e.chain_err(|| ErrorKind::EngineError("BibTeX")));
            }
        }

        Ok(0)
    }

    fn xdvipdfmx_pass<S: StatusBackend>(&mut self, status: &mut S) -> Result<i32> {
        {
            let mut stack = self.io.as_stack();
            let mut engine = XdvipdfmxEngine::new();
            status.note_highlighted("Running ", "xdvipdfmx", " ...");
            engine.process(
                &mut stack,
                &mut self.events,
                status,
                &self.tex_xdv_path.to_str().unwrap(),
                &self.tex_pdf_path.to_str().unwrap(),
            )?;
        }

        self.io.mem.files.borrow_mut().remove(&self.tex_xdv_path);
        Ok(0)
    }

    fn spx2html_pass<S: StatusBackend>(&mut self, status: &mut S) -> Result<i32> {
        {
            let mut stack = self.io.as_stack();
            let mut engine = Spx2HtmlEngine::new();
            status.note_highlighted("Running ", "spx2html", " ...");
            engine.process(
                &mut stack,
                &mut self.events,
                status,
                &self.tex_xdv_path.to_str().unwrap(),
            )?;
        }

        self.io.mem.files.borrow_mut().remove(&self.tex_xdv_path);
        Ok(0)
    }

    /// Consume this session and return the current set of files in memory.
    ///
    /// This convenience function tries to help with the annoyances of getting
    /// access to the in-memory file data after the engine has been run.
    ///
    /// ### Panics
    ///
    /// This will panic if you there are multiple strong references to the
    /// `files` map. This should only happen if you create and keep a clone of
    /// the `Rc<>` wrapping it before calling this function.
    pub fn into_file_data(self) -> HashMap<OsString, Vec<u8>> {
        Rc::try_unwrap(self.io.mem.files)
            .expect("multiple strong refs to MemoryIo files")
            .into_inner()
    }
}
