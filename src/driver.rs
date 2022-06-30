// Copyright 2018-2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! The high-level Tectonic document processing interface.
//!
//! The main struct in this module is [`ProcessingSession`], which knows how to
//! run (and re-run if necessary) the various engines in the right order. Such a
//! session can be created with a [`ProcessingSessionBuilder`], which you might
//! obtain from a [`tectonic_docmodel::document::Document`] using the
//! [`crate::docmodel::DocumentExt::setup_session`] extension method, if you’re
//! using the Tectonic document model. You can set one up manually if not.
//!
//! For an example of how to use this module, see `src/bin/tectonic/main.rs`,
//! which contains tectonic's main CLI program.

use byte_unit::Byte;
use quick_xml::{events::Event, Reader};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
    result::Result as StdResult,
    str::FromStr,
    time::SystemTime,
};
use tectonic_bridge_core::{CoreBridgeLauncher, DriverHooks, SecuritySettings, SystemRequestError};
use tectonic_bundles::Bundle;
use tectonic_io_base::{
    digest::DigestData,
    filesystem::{FilesystemIo, FilesystemPrimaryInputIo},
    stdstreams::{BufferedPrimaryIo, GenuineStdoutIo},
    InputHandle, IoProvider, OpenResult, OutputHandle,
};

use crate::{
    ctry, errmsg,
    errors::{ChainErrCompatExt, ErrorKind, Result},
    io::{
        format_cache::FormatCache,
        memory::{MemoryFileCollection, MemoryIo},
        InputOrigin,
    },
    status::StatusBackend,
    tt_error, tt_note, tt_warning,
    unstable_opts::UnstableOptions,
    BibtexEngine, Spx2HtmlEngine, TexEngine, TexOutcome, XdvipdfmxEngine,
};

/// Different patterns with which files may have been accessed by the
/// underlying engines. Once a file is marked as ReadThenWritten or
/// WrittenThenRead, its pattern does not evolve further.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
struct FileSummary {
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

/// The subset of the driver state that is captured when running a C/C++ engine.
///
/// The main purpose of this type is to implement the [`DriverHooks`] trait,
/// which is defined by the `tectonic_core_bridge` crate and defines that
/// interface that the C/C++ processing engines can use to access the outside
/// world. While these engines are running, they hold a mutable reference to
/// these data, so it is helpful to separate them out into a sub-structure of
/// the larger [`ProcessingSession`] type.
///
/// Due to the needs of the C/C++ engines, this means that [`BridgeState`] must
/// hold the fully-prepared I/O stack information as well as the “event”
/// information that helps the driver implement the rerun logic.
struct BridgeState {
    /// I/O for the primary input source. This is boxed since it can come
    /// from different sources: maybe a file, maybe an in-memory buffer, etc.
    primary_input: Box<dyn IoProvider>,

    /// I/O for the main backing bundle. This is boxed since there are several
    /// different bundle implementations that might be used at runtime.
    bundle: Box<dyn Bundle>,

    /// Memory buffering for files written during processing.
    mem: MemoryIo,

    /// The main filesystem backing for input files in the project.
    filesystem: FilesystemIo,

    /// Extra paths we search through for files.
    extra_search_paths: Vec<FilesystemIo>,

    /// Additional filesystem backing used if "shell escape" functionality is
    /// activated. If None, we take that to mean that shell-escape is
    /// disallowed. We have to use a persistent filesystem directory for this
    /// since some packages perform a whole series of shell-escape operations
    /// that assume continuity from one to the next.
    shell_escape_work: Option<FilesystemIo>,

    /// I/O for saving any generated format files.
    format_cache: FormatCache,

    /// Possible redirection of "standard output" writes to actual standard
    /// output.
    genuine_stdout: Option<GenuineStdoutIo>,

    /// A possible alternative "primary input" when generating format files. If
    /// Some(), we're in format-file generation mode; in most cases this is
    /// None.
    format_primary: Option<BufferedPrimaryIo>,

    /// The I/O events that occurred while processing.
    events: HashMap<String, FileSummary>,
}

impl BridgeState {
    /// Tell the IoProvider implementation of the bridge state to enter “format
    /// mode”, in which the “primary input” is fixed, based on the requested
    /// format file name, and filesystem I/O is bypassed.
    fn enter_format_mode(&mut self, format_file_name: &str) {
        self.format_primary = Some(BufferedPrimaryIo::from_text(&format!(
            "\\input {}",
            format_file_name
        )));
    }

    /// Leave “format mode”.
    fn leave_format_mode(&mut self) {
        self.format_primary = None;
    }

    /// Invoke an external tool as a pass in the processing pipeline.
    fn external_tool_pass(
        &mut self,
        tool: &ExternalToolPass,
        status: &mut dyn StatusBackend,
    ) -> Result<()> {
        status.note_highlighted("Running external tool ", &tool.argv[0], " ...");

        // Process the command arguments. Filenames appearing in the arguments
        // are treated as "requirements" that will be placed in the tool's
        // working directory.

        let mut cmd = Command::new(&tool.argv[0]);
        let mut read_files = tool.extra_requires.clone();

        {
            let mem_files = &*self.mem.files.borrow();

            for arg in &tool.argv[1..] {
                cmd.arg(arg);

                if mem_files.contains_key(arg) {
                    read_files.insert(arg.to_owned());
                }
            }
        }

        // Now that we're validated, write those files to disk so that the tool
        // can actually use them.

        let tempdir = ctry!(
            tempfile::Builder::new().tempdir();
            "can't create temporary directory for external tool"
        );

        {
            for name in &read_files {
                // If a relative parent is found in the file to open, this fn
                // does not properly handle that. Thus, throw an error.
                if name.contains("../") {
                    return Err(errmsg!(
                        "relative parent paths are not supported for the \
                        external tool. Got path `{}`.",
                        name
                    ));
                }

                let mut ih = ctry!(
                    self.input_open_name(name, status).must_exist();
                    "can't open path `{}`", name
                );

                // If the input path is absolute, we don't need to create a
                // version in the tempdir, and in fact the current
                // implementation below will blow away the input file. However,
                // we do want to try to open the input so that it gets
                // registered with the I/O tracking system.

                let path = Path::new(name);
                if path.is_absolute() {
                    continue;
                }

                let tool_path = tempdir.path().join(name);
                let tool_parent = tool_path.parent().unwrap();

                if tool_parent != tempdir.path() {
                    ctry!(
                        std::fs::create_dir_all(&tool_parent);
                        "failed to create sub directory `{}`", tool_parent.display()
                    );
                }
                let mut f = ctry!(
                    File::create(&tool_path);
                    "failed to create file `{}`", tool_path.display()
                );
                ctry!(
                    std::io::copy(&mut ih, &mut f);
                    "failed to write file `{}`", tool_path.display()
                );
            }
        }

        // Now we can actually run the command.

        let output = cmd.current_dir(tempdir.path()).output()?;

        if let Some(0) = output.status.code() {
        } else {
            tt_error!(
                status,
                "the external tool exited with an error code; its stdout was:\n"
            );
            status.dump_error_logs(&output.stdout[..]);
            tt_error!(status, "its stderr was:\n");
            status.dump_error_logs(&output.stderr[..]);

            return if let Some(n) = output.status.code() {
                Err(errmsg!("the external tool exited with error code {}", n))
            } else {
                Err(errmsg!("the external tool was terminated by a signal"))
            };
        }

        // Search for any files that the tool created, and import them into the
        // memory layer.

        for entry in std::fs::read_dir(tempdir.path())? {
            let entry = entry?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            if let Some(basename) = entry.file_name().to_str() {
                if !self.mem.files.borrow().contains_key(basename) {
                    let path = entry.path();
                    let mut data = Vec::new();

                    let mut f = ctry!(
                        File::open(&path);
                        "failed to open tool-created file `{}`", path.display()
                    );
                    ctry!(
                        f.read_to_end(&mut data);
                        "failed to read tool-created file `{}`", path.display()
                    );

                    self.mem.create_entry(basename, data);
                    self.events.insert(
                        basename.to_owned(),
                        FileSummary::new(AccessPattern::Written, InputOrigin::NotInput),
                    );
                }
            }
        }

        // Mark the input files as having been read, and we're done.

        for name in &read_files {
            let mut summ = self.events.get_mut(name).unwrap();
            summ.access_pattern = match summ.access_pattern {
                AccessPattern::Written => AccessPattern::WrittenThenRead,
                c => c, // identity mapping makes sense for remaining options
            };
        }

        Ok(())
    }

    // Get the names of all intermediate files which are generated from
    // previous passes.
    fn get_intermediate_file_names(&self) -> Vec<String> {
        // Currently, we only consider files in memory as intermediate files.
        return self.mem.files.borrow().keys().cloned().collect();
    }
}

macro_rules! bridgestate_ioprovider_try {
    ($provider:expr, $($inner:tt)+) => {
        let r = $provider.$($inner)+;
        match r {
            OpenResult::NotAvailable => {},
            _ => return r,
        };
    }
}

macro_rules! bridgestate_ioprovider_cascade {
    ($self:ident, $($inner:tt)+) => {
        if let Some(ref mut p) = $self.genuine_stdout {
            bridgestate_ioprovider_try!(p, $($inner)+);
        }

        // See enter_format_mode above. If creating a format file, disable local
        // filesystem I/O.
        let use_fs = if let Some(ref mut p) = $self.format_primary {
            bridgestate_ioprovider_try!(p, $($inner)+);
            false
        } else {
            bridgestate_ioprovider_try!($self.primary_input, $($inner)+);
            true
        };

        bridgestate_ioprovider_try!($self.mem, $($inner)+);

        if use_fs {
            bridgestate_ioprovider_try!($self.filesystem, $($inner)+);

            // With this ordering, we are preventing files created by
            // shell-escape commands from overwriting/replacing source files.
            // This seems very much like the behavior we want, unless there are
            // some freaky shell-escape uses that depend on this behavior.
            if let Some(ref mut p) = $self.shell_escape_work {
                bridgestate_ioprovider_try!(p, $($inner)+);
            }

            // Extra search paths. This has higher priority than bundles but lower than current
            // working dir to support the use case of overriding broken bundles (see issue #816).
            for fsio in $self.extra_search_paths.iter_mut() {
                bridgestate_ioprovider_try!(fsio, $($inner)+);
            }
        }

        bridgestate_ioprovider_try!($self.bundle.as_ioprovider_mut(), $($inner)+);
        bridgestate_ioprovider_try!($self.format_cache, $($inner)+);

        return OpenResult::NotAvailable;
    }
}

impl IoProvider for BridgeState {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        let r = (|| {
            bridgestate_ioprovider_cascade!(self, output_open_name(name));
        })();

        if let OpenResult::Ok(_) = r {
            if let Some(summ) = self.events.get_mut(name) {
                summ.access_pattern = match summ.access_pattern {
                    AccessPattern::Read => AccessPattern::ReadThenWritten,
                    c => c, // identity mapping makes sense for remaining options
                };
            } else {
                self.events.insert(
                    name.to_owned(),
                    FileSummary::new(AccessPattern::Written, InputOrigin::NotInput),
                );
            }
        }

        r
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        let r = (|| {
            bridgestate_ioprovider_cascade!(self, output_open_stdout());
        })();

        // Life is easier if we track stdout in the same way that we do other
        // output files.

        if let OpenResult::Ok(_) = r {
            if let Some(summ) = self.events.get_mut("") {
                summ.access_pattern = match summ.access_pattern {
                    AccessPattern::Read => AccessPattern::ReadThenWritten,
                    c => c, // identity mapping makes sense for remaining options
                };
            } else {
                self.events.insert(
                    String::from(""),
                    FileSummary::new(AccessPattern::Written, InputOrigin::NotInput),
                );
            }
        }

        r
    }

    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        match self.input_open_name_with_abspath(name, status) {
            OpenResult::Ok((ih, _path)) => OpenResult::Ok(ih),
            OpenResult::Err(e) => OpenResult::Err(e),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
        }
    }

    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let r = (|| {
            bridgestate_ioprovider_cascade!(self, input_open_name_with_abspath(name, status));
        })();

        match r {
            OpenResult::Ok((ref ih, ref _path)) => {
                if let Some(summ) = self.events.get_mut(name) {
                    summ.access_pattern = match summ.access_pattern {
                        AccessPattern::Written => AccessPattern::WrittenThenRead,
                        c => c, // identity mapping makes sense for remaining options
                    };
                } else {
                    self.events.insert(
                        name.to_owned(),
                        FileSummary::new(AccessPattern::Read, ih.origin()),
                    );
                }
            }

            OpenResult::NotAvailable => {
                // For the purposes of file access pattern tracking, an attempt to
                // open a nonexistent file counts as a read of a zero-size file. I
                // don't see how such a file could have previously been written, but
                // let's use the full update logic just in case.

                if let Some(summ) = self.events.get_mut(name) {
                    summ.access_pattern = match summ.access_pattern {
                        AccessPattern::Written => AccessPattern::WrittenThenRead,
                        c => c, // identity mapping makes sense for remaining options
                    };
                } else {
                    // Unlike other cases, here we need to fill in the read_digest. `None`
                    // is not an appropriate value since, if the file is written and then
                    // read again later, the `None` will be overwritten; but what matters
                    // is the contents of the file the very first time it was read.
                    let mut fs = FileSummary::new(AccessPattern::Read, InputOrigin::NotInput);
                    fs.read_digest = Some(DigestData::of_nothing());
                    self.events.insert(name.to_owned(), fs);
                }
            }

            OpenResult::Err(_) => {}
        }

        r
    }

    fn input_open_primary(&mut self, status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        match self.input_open_primary_with_abspath(status) {
            OpenResult::Ok((ih, _path)) => OpenResult::Ok(ih),
            OpenResult::Err(e) => OpenResult::Err(e),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
        }
    }

    fn input_open_primary_with_abspath(
        &mut self,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        bridgestate_ioprovider_cascade!(self, input_open_primary_with_abspath(status));
    }

    fn input_open_format(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let r = (|| {
            bridgestate_ioprovider_cascade!(self, input_open_format(name, status));
        })();

        if let OpenResult::Ok(ref ih) = r {
            if let Some(summ) = self.events.get_mut(name) {
                summ.access_pattern = match summ.access_pattern {
                    AccessPattern::Written => AccessPattern::WrittenThenRead,
                    c => c, // identity mapping makes sense for remaining options
                };
            } else {
                self.events.insert(
                    name.to_owned(),
                    FileSummary::new(AccessPattern::Read, ih.origin()),
                );
            }
        }

        r
    }
}

impl DriverHooks for BridgeState {
    fn io(&mut self) -> &mut dyn IoProvider {
        self
    }

    fn event_output_closed(
        &mut self,
        name: String,
        digest: DigestData,
        _status: &mut dyn StatusBackend,
    ) {
        let summ = self
            .events
            .get_mut(&name)
            .expect("closing file that wasn't opened?");
        summ.write_digest = Some(digest);
    }

    fn event_input_closed(
        &mut self,
        name: String,
        digest: Option<DigestData>,
        _status: &mut dyn StatusBackend,
    ) {
        let summ = self
            .events
            .get_mut(&name)
            .expect("closing file that wasn't opened?");

        // It's what was in the file the *first* time that it was read that
        // matters, so don't replace the read digest if it's already got one.

        if summ.read_digest.is_none() {
            summ.read_digest = digest;
        }
    }

    fn sysrq_shell_escape(
        &mut self,
        command: &str,
        status: &mut dyn StatusBackend,
    ) -> StdResult<(), SystemRequestError> {
        #[cfg(unix)]
        const SHELL: &[&str] = &["sh", "-c"];

        #[cfg(windows)]
        const SHELL: &[&str] = &["cmd.exe", "/c"];

        // Write any TeX-created files in the memory cache to the shell-escape
        // working directory, since the shell-escape program may need to use
        // them. (This is the case for `minted`.) We basically just hope that
        // nothing will want to access the actual TeX source, which will live in
        // a different directory.
        //
        // This is suboptimally slow since we'll be rewriting the same files
        // repeatedly for repeated shell-escape invocations, but I don't feel
        // like optimizing that I/O right now. Shell-escape is a gnarly hack
        // anyway!

        if let Some(work) = self.shell_escape_work.as_ref() {
            for (name, file) in &*self.mem.files.borrow() {
                // If it's in the `mem` backend, it's of interest here ...
                // unless it's stdout.
                if name == self.mem.stdout_key() {
                    continue;
                }

                let real_path = work.root().join(name);
                let mut f = File::create(&real_path).map_err(|e| {
                    tt_error!(status, "failed to create file `{}`", real_path.display(); e.into());
                    SystemRequestError::Failed
                })?;
                f.write_all(&file.data).map_err(|e| {
                    tt_error!(status, "failed to write file `{}`", real_path.display(); e.into());
                    SystemRequestError::Failed
                })?;
            }

            // Now we can actually run the command.

            tt_note!(status, "running shell command: `{}`", command);

            match Command::new(SHELL[0])
                .args(&SHELL[1..])
                .arg(&command)
                .current_dir(work.root())
                .status()
            {
                Ok(s) => match s.code() {
                    Some(0) => Ok(()),
                    Some(n) => {
                        tt_warning!(status, "command exited with error code {}", n);
                        Err(SystemRequestError::Failed)
                    }
                    None => {
                        tt_warning!(status, "command was terminated by signal");
                        Err(SystemRequestError::Failed)
                    }
                },
                Err(err) => {
                    tt_warning!(status, "failed to run command"; err.into());
                    Err(SystemRequestError::Failed)
                }
            }

            // That's it! We shouldn't clean up here, because there might be
            // multiple shell-escapes that build up in sequence, and any new
            // files created by the shell-escape command will be picked up by
            // the filesystem I/O.
        } else {
            // No shell-escape work directory. This "shouldn't happen" but means
            // that shell-escape is supposed to be disabled anyway!
            tt_error!(
                status,
                "the engine requested a shell-escape invocation but it's currently disabled"
            );
            Err(SystemRequestError::NotAllowed)
        }
    }
}

/// Possible modes for handling shell-escape functionality
#[derive(Clone, Debug, Eq, PartialEq)]
enum ShellEscapeMode {
    /// "Default" mode: shell-escape is disabled, unless it's been turned on in
    /// the unstable options, in which case it will be allowed through a
    /// temporary directory.
    Defaulted,

    /// Shell-escape is disabled, overriding any unstable-option setting.
    Disabled,

    /// Shell-escape is enabled, using a temporary work directory managed by the
    /// processing session. The work directory will be deleted after processing
    /// completes.
    TempDir,

    /// Shell-escape is enabled, using some other work directory that is managed
    /// externally. The processing session won't delete this directory.
    ExternallyManagedDir(PathBuf),
}

impl Default for ShellEscapeMode {
    fn default() -> Self {
        ShellEscapeMode::Defaulted
    }
}

/// A custom extra pass that invokes an external tool.
///
/// This is bad for reproducibility but comes in handy.
#[derive(Debug)]
struct ExternalToolPass {
    argv: Vec<String>,
    extra_requires: HashSet<String>,
}

/// A builder-style interface for creating a [`ProcessingSession`].
///
/// This uses standard builder patterns. The `Default` implementation defaults
/// to restrictive security settings that disable all known-insecure features
/// that could be abused by untrusted inputs. Use
/// [`ProcessingSessionBuilder::new_with_security()`] in order to have the
/// option to enable potentially-insecure features such as shell-escape.
#[derive(Default)]
pub struct ProcessingSessionBuilder {
    security: SecuritySettings,
    primary_input: PrimaryInputMode,
    tex_input_name: Option<String>,
    output_dest: OutputDestination,
    filesystem_root: Option<PathBuf>,
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
    build_date: Option<SystemTime>,
    unstables: UnstableOptions,
    shell_escape_mode: ShellEscapeMode,
}

impl ProcessingSessionBuilder {
    /// Create a new builder with customized security settings.
    pub fn new_with_security(security: SecuritySettings) -> Self {
        ProcessingSessionBuilder {
            security,
            ..Default::default()
        }
    }

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
    /// not provided, [`ProcessingSessionBuilder::create`] will panic).
    pub fn tex_input_name(&mut self, s: &str) -> &mut Self {
        self.tex_input_name = Some(s.to_owned());
        self
    }

    /// Set the directory that serves as the root for finding files on disk.
    ///
    /// If unspecified, and there is a primary input file, the directory
    /// containing that file will serve as the filesystem root. Otherwise, it is
    /// set to the current directory.
    pub fn filesystem_root<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.filesystem_root = Some(p.as_ref().to_owned());
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
    /// This file does not necessarily have to exist already; it will be created
    /// if it doesn't. This parameter is mandatory (if it is not provided,
    /// [`ProcessingSessionBuilder::create`] will panic).
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

    /// Sets the date and time of the processing session.
    /// See `TexEngine::build_date` for mor information.
    pub fn build_date(&mut self, date: SystemTime) -> &mut Self {
        self.build_date = Some(date);
        self
    }

    /// Loads unstable options into the processing session
    pub fn unstables(&mut self, opts: UnstableOptions) -> &mut Self {
        self.unstables = opts;
        self
    }

    /// Enable "shell escape" commands in the engines, and use the specified
    /// directory for shell-escape work. The caller is responsible for the
    /// creation and/or destruction of this directory. The default is to
    /// disable shell-escape unless the [`UnstableOptions`] say otherwise,
    /// in which case a driver-managed temporary directory will be used.
    pub fn shell_escape_with_work_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        if self.security.allow_shell_escape() {
            self.shell_escape_mode =
                ShellEscapeMode::ExternallyManagedDir(path.as_ref().to_owned());
        }
        self
    }

    /// Forcibly enable shell-escape mode with a temporary directory, overriding
    /// any [`UnstableOptions`] settings. The default is to disable shell-escape
    /// unless the [`UnstableOptions`] say otherwise, in which case a
    /// driver-managed temporary directory will be used.
    pub fn shell_escape_with_temp_dir(&mut self) -> &mut Self {
        if self.security.allow_shell_escape() {
            self.shell_escape_mode = ShellEscapeMode::TempDir;
        }
        self
    }

    /// Forcibly disable shell-escape mode, overriding any [`UnstableOptions`]
    /// settings. The default is to disable shell-escape unless the
    /// [`UnstableOptions`] say otherwise, in which case a driver-managed
    /// temporary directory will be used.
    pub fn shell_escape_disabled(&mut self) -> &mut Self {
        self.shell_escape_mode = ShellEscapeMode::Disabled;
        self
    }

    /// Creates a `ProcessingSession`.
    pub fn create(self, status: &mut dyn StatusBackend) -> Result<ProcessingSession> {
        // First, work on the "bridge state", which gathers the subset of our
        // state that has to be held in a mutable reference while running the
        // C/C++ engines:

        let mut bundle = self.bundle.expect("a bundle must be specified");

        let mut filesystem_root = self.filesystem_root.unwrap_or_default();

        let (pio, primary_input_path, default_output_path) = match self.primary_input {
            PrimaryInputMode::Path(p) => {
                // Set the filesystem root (that's the directory we'll search
                // for files in) to be the same directory as the main input
                // file.
                let parent = match p.parent() {
                    Some(parent) => parent.to_owned(),
                    None => {
                        return Err(errmsg!(
                            "can't figure out a parent directory for input path \"{}\"",
                            p.display()
                        ));
                    }
                };

                filesystem_root = parent.clone();
                let pio: Box<dyn IoProvider> = Box::new(FilesystemPrimaryInputIo::new(&p));
                (pio, Some(p), parent)
            }

            PrimaryInputMode::Stdin => {
                // If the main input file is stdin, we don't set a filesystem
                // root, which means we'll default to the current working
                // directory.
                //
                // Note that, due to the expected need to rerun the engine
                // multiple times, we'll need to buffer stdin in its entirety,
                // so we might as well do that now.
                let pio = ctry!(BufferedPrimaryIo::from_stdin(); "error reading standard input");
                let pio: Box<dyn IoProvider> = Box::new(pio);
                (pio, None, "".into())
            }

            PrimaryInputMode::Buffer(buf) => {
                // Same behavior as with stdin.
                let pio: Box<dyn IoProvider> = Box::new(BufferedPrimaryIo::from_buffer(buf));
                (pio, None, "".into())
            }
        };

        let format_cache_path = self
            .format_cache_path
            .unwrap_or_else(|| filesystem_root.clone());
        let format_cache = FormatCache::new(bundle.get_digest(status)?, format_cache_path);

        let genuine_stdout = if self.print_stdout {
            Some(GenuineStdoutIo::new())
        } else {
            None
        };

        // move this out of self to get around borrow checker issues
        let hidden_input_paths = self.hidden_input_paths;

        let extra_search_paths = if self.security.allow_extra_search_paths() {
            self.unstables
                .extra_search_paths
                .iter()
                .map(|p| FilesystemIo::new(p, false, false, hidden_input_paths.clone()))
                .collect()
        } else {
            if !self.unstables.extra_search_paths.is_empty() {
                tt_warning!(status, "Extra search path(s) ignored due to security");
            }
            Vec::new()
        };

        let filesystem = FilesystemIo::new(&filesystem_root, false, true, hidden_input_paths);

        let mem = MemoryIo::new(true);

        let bs = BridgeState {
            primary_input: pio,
            mem,
            filesystem,
            extra_search_paths,
            shell_escape_work: None,
            format_cache,
            bundle,
            genuine_stdout,
            format_primary: None,
            events: HashMap::new(),
        };

        // Now we can do the rest.

        let output_path = match self.output_dest {
            OutputDestination::Default => Some(default_output_path),
            OutputDestination::Path(p) => Some(p),
            OutputDestination::Nowhere => None,
        };

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

        let shell_escape_mode = if !self.security.allow_shell_escape() {
            ShellEscapeMode::Disabled
        } else {
            match self.shell_escape_mode {
                ShellEscapeMode::Defaulted => {
                    if self.unstables.shell_escape {
                        ShellEscapeMode::TempDir
                    } else {
                        ShellEscapeMode::Disabled
                    }
                }

                other => other,
            }
        };

        Ok(ProcessingSession {
            security: self.security,
            bs,
            pass: self.pass,
            primary_input_path,
            primary_input_tex_path: tex_input_name,
            format_name: self.format_name.unwrap(),
            tex_aux_path: aux_path.display().to_string(),
            tex_xdv_path: xdv_path.display().to_string(),
            tex_pdf_path: pdf_path.display().to_string(),
            output_format: self.output_format,
            makefile_output_path: self.makefile_output_path,
            output_path,
            tex_rerun_specification: self.reruns,
            keep_intermediates: self.keep_intermediates,
            keep_logs: self.keep_logs,
            synctex_enabled: self.synctex,
            build_date: self.build_date.unwrap_or(SystemTime::UNIX_EPOCH),
            unstables: self.unstables,
            shell_escape_mode,
        })
    }
}

#[derive(Debug, Clone)]
enum RerunReason {
    Biber,
    Bibtex,
    FileChange(String),
}

/// The ProcessingSession struct runs the whole show when we're actually
/// processing a file. It understands, for example, the need to re-run the TeX
/// engine if the `.aux` file changed.
pub struct ProcessingSession {
    // Security settings.
    security: SecuritySettings,

    /// The subset of the session state that's can be mutated while the C/C++
    /// engines are running. Importantly, this includes the full I/O stack.
    bs: BridgeState,

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
    /// just `primary_input_tex_path` with the extension changed.
    tex_aux_path: String,
    tex_xdv_path: String,
    tex_pdf_path: String,

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
    synctex_enabled: bool,

    /// See `TexEngine::with_date` and `XdvipdfmxEngine::with_date`.
    build_date: SystemTime,

    unstables: UnstableOptions,

    /// How to handle shell-escape. The `Defaulted` option will never
    /// be used here.
    shell_escape_mode: ShellEscapeMode,
}

const DEFAULT_MAX_TEX_PASSES: usize = 6;
const ALWAYS_INTERMEDIATE_EXTENSIONS: &[&str] = &[
    ".snm", ".toc", // generated by Beamer
];

impl ProcessingSession {
    /// Assess whether we need to rerun an engine. This is the case if there
    /// was a file that the engine read and then rewrote, and the rewritten
    /// version is different than the version that it read in.
    fn is_rerun_needed(&self, status: &mut dyn StatusBackend) -> Option<RerunReason> {
        // TODO: we should probably wire up diagnostics since I expect this
        // stuff could get finicky and we're going to want to be able to
        // figure out why rerun detection is breaking.

        for (name, info) in &self.bs.events {
            if info.access_pattern == AccessPattern::ReadThenWritten {
                let file_changed = match (&info.read_digest, &info.write_digest) {
                    (&Some(ref d1), &Some(ref d2)) => d1 != d2,
                    (&None, &Some(_)) => true,
                    (_, _) => {
                        // Other cases shouldn't happen.
                        tt_warning!(
                            status,
                            "internal consistency problem when checking if {} changed",
                            name
                        );
                        true
                    }
                };

                if file_changed {
                    return Some(RerunReason::FileChange(name.clone()));
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn _dump_access_info(&self, status: &mut dyn StatusBackend) {
        for (name, info) in &self.bs.events {
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
                    name,
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
    pub fn run(&mut self, status: &mut dyn StatusBackend) -> Result<()> {
        // Pre-invocation setup that requires cleanup even if the processing errors out.

        let (shell_escape_work, clean_up_shell_escape) = match self.shell_escape_mode {
            ShellEscapeMode::Disabled => (None, false),

            ShellEscapeMode::ExternallyManagedDir(ref p) => (
                Some(FilesystemIo::new(p, false, false, HashSet::new())),
                false,
            ),

            ShellEscapeMode::TempDir => {
                let tempdir = ctry!(tempfile::Builder::new().tempdir(); "can't create temporary directory for shell-escape work");
                (
                    Some(FilesystemIo::new(
                        &tempdir.into_path(),
                        false,
                        false,
                        HashSet::new(),
                    )),
                    true,
                )
            }

            ShellEscapeMode::Defaulted => unreachable!(),
        };

        self.bs.shell_escape_work = shell_escape_work;

        // Go-time!
        let result = self.run_inner(status);

        // Do that cleanup.

        if clean_up_shell_escape {
            let shell_escape_work = self.bs.shell_escape_work.take().unwrap();
            let shell_escape_err = std::fs::remove_dir_all(shell_escape_work.root());

            if let Err(e) = shell_escape_err {
                tt_warning!(status, "an error occurred while cleaning up the \
                    shell-escape temporary directory `{}`", shell_escape_work.root().display(); e.into());
            }
        }

        // Propagate the actual result.
        result
    }

    /// The bulk of the `run` implementation. We need to wrap it to manage the
    /// lifecycle of resources like the shell-escape temporary directory, if
    /// needed.
    fn run_inner(&mut self, status: &mut dyn StatusBackend) -> Result<()> {
        // Do we need to generate the format file?

        let generate_format = if self.output_format == OutputFormat::Format {
            false
        } else {
            match self.bs.input_open_format(&self.format_name, status) {
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
            PassSetting::Tex => match self.tex_pass(None, status) {
                Ok(Some(warnings)) => {
                    tt_warning!(status, "{}", warnings);
                    Ok(0)
                }
                Ok(None) => Ok(0),
                Err(e) => Err(e),
            },
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
                let opip = ctry!(pip.to_str(); "Makefile-rules file path must be Unicode-able");
                ctry!(mf_dest.write_all(opip.as_bytes()); "couldn't write to Makefile-rules file");
            }

            // The check above ensures that this is never None.
            let root = self.output_path.as_ref().unwrap();

            for (name, info) in &self.bs.events {
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
                    tt_warning!(status, "omitting circular Makefile dependency for {}", name);
                    continue;
                }

                ctry!(write!(mf_dest, " \\\n  {}", root.join(name).display()); "couldn't write to Makefile-rules file");
            }

            ctry!(writeln!(mf_dest, ""); "couldn't write to Makefile-rules file");
        }

        // All done.

        Ok(())
    }

    fn write_files(
        &mut self,
        mut mf_dest_maybe: Option<&mut File>,
        status: &mut dyn StatusBackend,
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

        for (name, file) in &*self.bs.mem.files.borrow() {
            if name == self.bs.mem.stdout_key() {
                continue;
            }

            let sname = name;
            let summ = self.bs.events.get_mut(name).unwrap();

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

            if file.data.is_empty() {
                status.note_highlighted(
                    "Not writing ",
                    &format!("`{}`", sname),
                    ": it would be empty.",
                );
                continue;
            }

            let real_path = root.join(name);
            let byte_len = Byte::from_bytes(file.data.len() as u128);
            status.note_highlighted(
                "Writing ",
                &format!("`{}`", real_path.display()),
                &format!(" ({})", byte_len.get_appropriate_unit(true)),
            );

            let mut f = File::create(&real_path)?;
            f.write_all(&file.data)?;
            summ.got_written_to_disk = true;

            if let Some(ref mut mf_dest) = mf_dest_maybe {
                // Maybe it'd be better to have this just be a warning? But if
                // the program is supposed to write the file, you don't want
                // it exiting with error code zero if it couldn't do that
                // successfully.
                //
                // Not quite sure why, but I can't pull out the target path
                // here. I think 'self' is borrow inside the loop?
                ctry!(write!(mf_dest, "{} ", real_path.display()); "couldn't write to Makefile-rules file");
            }
        }

        Ok(n_skipped_intermediates)
    }

    /// The "default" pass really runs a bunch of sub-passes. It is a "Do What
    /// I Mean" operation.
    fn default_pass(&mut self, bibtex_first: bool, status: &mut dyn StatusBackend) -> Result<i32> {
        // If `bibtex_first` is true, we start by running bibtex, and run
        // proceed with the standard rerun logic. Otherwise, we run TeX,
        // auto-detect whether we need to run bibtex, possibly run it, and
        // then go ahead.

        let mut warnings = None;
        let mut rerun_result = if bibtex_first {
            self.bibtex_pass(status)?;
            Some(RerunReason::Bibtex)
        } else {
            warnings = self.tex_pass(None, status)?;
            let maybe_biber = self.check_biber_requirement()?;

            if let Some(biber) = maybe_biber {
                self.bs.external_tool_pass(&biber, status)?;
                Some(RerunReason::Biber)
            } else if self.is_bibtex_needed() {
                self.bibtex_pass(status)?;
                Some(RerunReason::Bibtex)
            } else {
                self.is_rerun_needed(status)
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
                    Some(RerunReason::Biber) => "biber was run".to_owned(),
                    Some(RerunReason::Bibtex) => "bibtex was run".to_owned(),
                    Some(RerunReason::FileChange(ref s)) => format!("\"{}\" changed", s),
                    None => break,
                }
            };

            // We're restarting the engine afresh, so clear the read inputs.
            // We do *not* clear the entire HashMap since we want to remember,
            // e.g., that bibtex wrote out the .bbl file, since that way we
            // can later know that it's OK to delete. I am not super confident
            // that the access_pattern data can just be left as-is when we do
            // this, but, uh, so far it seems to work.
            for summ in self.bs.events.values_mut() {
                summ.read_digest = None;
            }

            warnings = self.tex_pass(Some(&rerun_explanation), status)?;

            if !reruns_fixed {
                rerun_result = self.is_rerun_needed(status);

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

        // The last tex pass generated warnings.
        if let Some(warnings) = warnings {
            tt_warning!(status, "{}", warnings);
        }

        // And finally, xdvipdfmx or spx2html. Maybe.

        if let OutputFormat::Pdf = self.output_format {
            self.xdvipdfmx_pass(status)?;
        } else if let OutputFormat::Html = self.output_format {
            self.spx2html_pass(status)?;
        }

        Ok(0)
    }

    fn is_bibtex_needed(&self) -> bool {
        const BIBDATA: &[u8] = b"\\bibdata";

        self.bs
            .mem
            .files
            .borrow()
            .get(&self.tex_aux_path)
            .map(|file| {
                // We used to use aho-corasick crate here, but it was removed to reduce the code
                // size.
                file.data.windows(BIBDATA.len()).any(|s| s == BIBDATA)
            })
            .unwrap_or(false)
    }

    /// Use the TeX engine to generate a format file.
    #[allow(clippy::manual_split_once)] // requires Rust 1.52 (note that we don't actually define our MSRV)
    fn make_format_pass(&mut self, status: &mut dyn StatusBackend) -> Result<i32> {
        // PathBuf.file_stem() doesn't do what we want since it only strips
        // one extension. As of 1.17, the compiler needs a type annotation for
        // some reason, which is why we use the `r` variable.
        let r: Result<&str> = self.format_name.split('.').next().ok_or_else(|| {
            ErrorKind::Msg(format!(
                "incomprehensible format file name \"{}\"",
                self.format_name
            ))
            .into()
        });
        let stem = r?;

        let result = {
            self.bs
                .enter_format_mode(&format!("tectonic-format-{}.tex", stem));
            let mut launcher =
                CoreBridgeLauncher::new_with_security(&mut self.bs, status, self.security.clone());
            let r = TexEngine::default()
                .halt_on_error_mode(true)
                .initex_mode(true)
                .shell_escape(self.shell_escape_mode != ShellEscapeMode::Disabled)
                .process(&mut launcher, "UNUSED.fmt", "texput");
            self.bs.leave_format_mode();
            r
        };

        match result {
            Ok(TexOutcome::Spotless) => {}
            Ok(TexOutcome::Warnings) => {
                tt_warning!(status, "warnings were issued by the TeX engine; use --print and/or --keep-logs for details.");
            }
            Ok(TexOutcome::Errors) => {
                tt_error!(status, "errors were issued by the TeX engine; use --print and/or --keep-logs for details.");
                return Err(ErrorKind::Msg("unhandled TeX engine error".to_owned()).into());
            }
            Err(e) => {
                return Err(e.into());
            }
        }

        // Now we can write the format file to its special location. In
        // principle we could stream the format file directly to the staging
        // area as we ran the TeX engine, but we don't bother.

        for (name, file) in &*self.bs.mem.files.borrow() {
            if name == self.bs.mem.stdout_key() {
                continue;
            }

            let sname = name;

            if !sname.ends_with(".fmt") {
                continue;
            }

            // Note that we intentionally pass 'stem', not 'name'.
            ctry!(self.bs.format_cache.write_format(stem, &file.data, status); "cannot write format file {}", sname);
        }

        // All done. Clear the memory layer since this was a special preparatory step.
        self.bs.mem.files.borrow_mut().clear();

        Ok(0)
    }

    /// Run one pass of the TeX engine.
    fn tex_pass(
        &mut self,
        rerun_explanation: Option<&str>,
        status: &mut dyn StatusBackend,
    ) -> Result<Option<&'static str>> {
        let result = {
            if let Some(s) = rerun_explanation {
                status.note_highlighted("Rerunning ", "TeX", &format!(" because {} ...", s));
            } else {
                status.note_highlighted("Running ", "TeX", " ...");
            }

            let mut launcher =
                CoreBridgeLauncher::new_with_security(&mut self.bs, status, self.security.clone());

            TexEngine::default()
                .halt_on_error_mode(true)
                .initex_mode(self.output_format == OutputFormat::Format)
                .synctex(self.synctex_enabled)
                .semantic_pagination(self.output_format == OutputFormat::Html)
                .shell_escape(self.shell_escape_mode != ShellEscapeMode::Disabled)
                .build_date(self.build_date)
                .process(
                    &mut launcher,
                    &self.format_name,
                    &self.primary_input_tex_path,
                )
        };

        let warnings = match result {
            Ok(TexOutcome::Spotless) => None,
            Ok(TexOutcome::Warnings) =>
                    Some("warnings were issued by the TeX engine; use --print and/or --keep-logs for details."),
            Ok(TexOutcome::Errors) =>
                    Some("errors were issued by the TeX engine, but were ignored; \
                         use --print and/or --keep-logs for details."),
            Err(e) =>
                return Err(e.into()),
        };

        if !self.bs.mem.files.borrow().contains_key(&self.tex_xdv_path) {
            // TeX did not produce the expected output file
            tt_warning!(
                status,
                "did not produce \"{}\"; this may mean that your document is empty",
                self.tex_xdv_path
            )
        }

        Ok(warnings)
    }

    // Run Bibtex process for one .aux file.
    fn bibtex_pass_for_one_aux_file(
        &mut self,
        status: &mut dyn StatusBackend,
        aux_file: &String,
    ) -> Result<i32> {
        let result = {
            status.note_highlighted("Running ", "BibTeX", &format!(" on {} ...", aux_file));
            let mut launcher =
                CoreBridgeLauncher::new_with_security(&mut self.bs, status, self.security.clone());
            let mut engine = BibtexEngine::new();
            engine.process(&mut launcher, aux_file, &self.unstables)
        };

        match result {
            Ok(TexOutcome::Spotless) => {}
            Ok(TexOutcome::Warnings) => {
                tt_note!(
                    status,
                    "warnings were issued by BibTeX; use --print and/or --keep-logs for details."
                );
            }
            Ok(TexOutcome::Errors) => {
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

    fn bibtex_pass(&mut self, status: &mut dyn StatusBackend) -> Result<i32> {
        let mut aux_files = vec![self.tex_aux_path.clone()];

        // find other .aux files generated by tex_pass
        for f in self.bs.get_intermediate_file_names() {
            if f.ends_with(".aux") && f != self.tex_aux_path {
                aux_files.push(f);
            }
        }

        for f in aux_files {
            let r = self.bibtex_pass_for_one_aux_file(status, &f);
            if let Err(e) = r {
                return Err(e);
            }
        }

        Ok(0)
    }

    fn xdvipdfmx_pass(&mut self, status: &mut dyn StatusBackend) -> Result<i32> {
        {
            status.note_highlighted("Running ", "xdvipdfmx", " ...");

            let mut launcher =
                CoreBridgeLauncher::new_with_security(&mut self.bs, status, self.security.clone());
            let mut engine = XdvipdfmxEngine::default();

            engine.build_date(self.build_date);

            if let Some(ref ps) = self.unstables.paper_size {
                engine.paper_spec(ps.clone());
            }

            engine.process(&mut launcher, &self.tex_xdv_path, &self.tex_pdf_path)?;
        }

        self.bs.mem.files.borrow_mut().remove(&self.tex_xdv_path);
        Ok(0)
    }

    fn spx2html_pass(&mut self, status: &mut dyn StatusBackend) -> Result<i32> {
        let op = match self.output_path {
            Some(ref p) => p,
            None => return Err(errmsg!("HTML output must be saved directly to disk")),
        };

        {
            let mut engine = Spx2HtmlEngine::default();
            status.note_highlighted("Running ", "spx2html", " ...");
            engine.process_to_filesystem(&mut self.bs, status, &self.tex_xdv_path, op)?;
        }

        self.bs.mem.files.borrow_mut().remove(&self.tex_xdv_path);
        Ok(0)
    }

    /// Get what was printed to standard output, if anything.
    pub fn get_stdout_content(&self) -> Vec<u8> {
        self.bs
            .mem
            .files
            .borrow()
            .get(self.bs.mem.stdout_key())
            .map(|mfi| mfi.data.clone())
            .unwrap_or_else(Vec::new)
    }

    /// Consume this session and return the current set of files in memory.
    ///
    /// This convenience function tries to help with the annoyances of getting
    /// access to the in-memory file data after the engine has been run.
    pub fn into_file_data(self) -> MemoryFileCollection {
        Rc::try_unwrap(self.bs.mem.files)
            .expect("multiple strong refs to MemoryIo files")
            .into_inner()
    }

    /// See if we need to run `biber`, and parse the `.run.xml` file from the
    /// `loqreq` package to figure out what files `biber` needs. This
    /// functionality should probably become more generic, but I don't have a
    /// great sense as to how widely-used `logreq` is.
    fn check_biber_requirement(&self) -> Result<Option<ExternalToolPass>> {
        // Is there a `.run.xml` file?

        let mut run_xml_path = PathBuf::from(&self.primary_input_tex_path);
        run_xml_path.set_extension("run.xml");
        let run_xml_path = run_xml_path.display().to_string();

        let mem_files = &*self.bs.mem.files.borrow();
        let run_xml_entry = match mem_files.get(&run_xml_path) {
            Some(e) => e,
            None => return Ok(None),
        };

        // Yes, there is. Set up to potentially run biber. For testing support,
        // we let the rig specify a custom executable to use, which lets us
        // exercise different pieces of the external-tool behavior.

        let s = (
            crate::config::is_config_test_mode_activated(),
            std::env::var("TECTONIC_TEST_FAKE_BIBER"),
        );

        let mut argv = match s {
            (true, Ok(text)) => text.split_whitespace().map(|x| x.to_owned()).collect(),
            _ => vec!["biber".to_owned()],
        };

        let mut extra_requires = HashSet::new();

        // Do a sketchy XML parse to see if there's info about a biber
        // invocation.

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum State {
            /// Searching for the biber section
            Searching,

            /// In a <binary> element. Will its value be "biber"??!?
            InBinaryName,

            /// In the <cmdline> part of the biber section.
            InBiberCmdline,

            /// About to read an argument to the biber command.
            InBiberArgument,

            /// Reading through the post-cmdline part of the biber section.
            InBiberRemainder,

            /// In a "requirement" section like <input> or <requires> that contains
            /// filenames we should provide
            InBiberRequirementSection,

            /// In a <file> requirement
            InBiberFileRequirement,
        }

        let curs = Cursor::new(&run_xml_entry.data[..]);
        let mut reader = Reader::from_reader(curs);
        let mut buf = Vec::new();
        let mut state = State::Searching;

        loop {
            let event = ctry!(
                reader.read_event(&mut buf);
                "error parsing run.xml file"
            );

            if let Event::Eof = event {
                break;
            }

            match (state, event) {
                (State::Searching, Event::Start(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    if name == "binary" {
                        state = State::InBinaryName;
                    }
                }

                (State::InBinaryName, Event::Text(ref e)) => {
                    let text = e.unescape_and_decode(&reader)?;

                    state = if &text == "biber" {
                        State::InBiberCmdline
                    } else {
                        State::Searching
                    };
                }

                (State::InBinaryName, _) => {
                    state = State::Searching;
                }

                (State::InBiberCmdline, Event::Start(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    // Note that the "infile" might be `foo` without the `.bcf`
                    // extension, so we can't use it for file-finding.
                    state = match name {
                        "infile" | "outfile" | "option" => State::InBiberArgument,
                        _ => State::InBiberRemainder,
                    }
                }

                (State::InBiberCmdline, Event::End(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    if name == "cmdline" {
                        state = State::InBiberRemainder;
                    }
                }

                (State::InBiberArgument, Event::Text(ref e)) => {
                    argv.push(e.unescape_and_decode(&reader)?);
                    state = State::InBiberCmdline;
                }

                (State::InBiberRemainder, Event::Start(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    state = match name {
                        "input" | "requires" => State::InBiberRequirementSection,
                        _ => State::InBiberRemainder,
                    }
                }

                (State::InBiberRemainder, Event::End(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    if name == "external" {
                        break;
                    }
                }

                (State::InBiberRequirementSection, Event::Start(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    state = match name {
                        "file" => State::InBiberFileRequirement,
                        _ => State::InBiberRemainder,
                    }
                }

                (State::InBiberRequirementSection, Event::End(ref e)) => {
                    let name = reader.decode(e.local_name())?;

                    if name == "input" || name == "requires" {
                        state = State::InBiberRemainder;
                    }
                }

                (State::InBiberFileRequirement, Event::Text(ref e)) => {
                    extra_requires.insert(e.unescape_and_decode(&reader)?);
                    state = State::InBiberRequirementSection;
                }

                (State::InBiberFileRequirement, _) => {
                    state = State::InBiberRequirementSection;
                }

                _ => {}
            }
        }

        // All done!

        Ok(if state == State::Searching {
            // No biber invocation, in the end.
            None
        } else {
            Some(ExternalToolPass {
                argv,
                extra_requires,
            })
        })
    }
}
