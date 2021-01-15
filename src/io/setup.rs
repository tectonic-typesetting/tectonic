// Copyright 2018-2020 the Tectonic Project
// Licensed under the MIT License.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use tectonic_errors::{atry, Result};
use tectonic_io_base::{
    filesystem::{FilesystemIo, FilesystemPrimaryInputIo},
    stack::IoStack,
    stdstreams::{BufferedPrimaryIo, GenuineStdoutIo},
    IoProvider,
};
use tectonic_status_base::StatusBackend;

use super::{format_cache::FormatCache, Bundle, MemoryIo};

/// An `IoSetup` is essentially a typed, structured version of an [`IoStack`].
///
/// The `IoStack` struct must necessarily erase types (i.e., turn I/O layers into `IoProvider`
/// trait objects). But, between invocations of various engines, we want to look at our individual
/// typed I/O providers and interrogate them (i.e., see what files were created in the memory
/// layer).  The `IoSetup` struct helps us maintain detailed knowledge of types while creating an
/// `IoStack` when needed.
///
/// The `IoStack` produced by an `IoSetup` follows a particular structure: memory I/O backed by
/// filesystem I/O, backed by an optional `Bundle`.  This way, any newly created files will be
/// created in memory, and you can examine them by poking at the `mem` field.

pub struct IoSetup {
    primary_input: Box<dyn IoProvider>,
    pub bundle: Option<Box<dyn Bundle>>,
    pub mem: MemoryIo,
    filesystem: FilesystemIo,
    pub format_cache: Option<FormatCache>,
    genuine_stdout: Option<GenuineStdoutIo>,
    format_primary: Option<BufferedPrimaryIo>,
}

impl IoSetup {
    pub fn as_stack(&mut self) -> IoStack {
        let mut providers: Vec<&mut dyn IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }

        providers.push(&mut *self.primary_input);
        providers.push(&mut self.mem);
        providers.push(&mut self.filesystem);

        if let Some(ref mut b) = self.bundle {
            providers.push(b.as_ioprovider_mut());
        }

        if let Some(ref mut c) = self.format_cache {
            providers.push(&mut *c);
        }

        IoStack::new(providers)
    }

    /// Creates an `IoStack` for the specific purpose of writing out a format file.
    ///
    /// This differs from [`IoSetup::as_stack`] in two ways:
    ///
    /// - the primary input is not used here; instead, this method provides a "dummy" primary input
    ///   file containing only "\input format-file-name.tex"
    /// - the filesystem is not included, and so only files that are present in the bundle can have
    ///   an effect on the format file.
    ///
    /// You can use the resulting `IoStack` to run the TeX engine with `initex_mode` set to `true`;
    /// then the resulting format file(s) can be read from the memory I/O layer (i.e. `self.mem`).

    pub fn as_stack_for_format<'a>(&'a mut self, format_file_name: &str) -> IoStack<'a> {
        let mut providers: Vec<&mut dyn IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }

        self.format_primary = Some(BufferedPrimaryIo::from_text(&format!(
            "\\input {}",
            format_file_name
        )));
        providers.push(self.format_primary.as_mut().unwrap());
        providers.push(&mut self.mem);

        if let Some(ref mut b) = self.bundle {
            providers.push(b.as_ioprovider_mut());
        }

        if let Some(ref mut c) = self.format_cache {
            providers.push(&mut *c);
        }

        IoStack::new(providers)
    }
}

/// Where does the "primary input" stream come from?
enum PrimaryInputMode {
    /// The caller never specified; we'll panic.
    Undefined,

    /// The process's standard input stream.
    Stdin,

    /// Somewhere on the filesystem.
    Path(PathBuf),

    /// An in-memory buffer.
    Buffer(Vec<u8>),
}

/// The IoSetupBuilder provides a convenient builder interface for specifying
/// the I/O setup.
pub struct IoSetupBuilder {
    primary_input: PrimaryInputMode,
    filesystem_root: PathBuf,
    format_cache_path: Option<PathBuf>,
    bundle: Option<Box<dyn Bundle>>,
    use_genuine_stdout: bool,
    hidden_input_paths: HashSet<PathBuf>,
}

impl Default for IoSetupBuilder {
    fn default() -> Self {
        IoSetupBuilder {
            primary_input: PrimaryInputMode::Undefined,
            filesystem_root: PathBuf::new(),
            format_cache_path: None,
            bundle: None,
            use_genuine_stdout: false,
            hidden_input_paths: HashSet::new(),
        }
    }
}

impl IoSetupBuilder {
    /// Sets the path for the primary input (i.e. the main .tex file).
    ///
    /// If other functions that set the primary input mode are called (e.g.
    /// [`IoSetupBuilder::primary_input_stdin`], the setting will be
    /// overridden.
    pub fn primary_input_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.primary_input = PrimaryInputMode::Path(path.as_ref().to_owned());
        self
    }

    /// Configures us to read the primary input (i.e. the main .tex file) from stdin.
    ///
    /// If other functions that set the primary input mode are called (e.g.
    /// [`IoSetupBuilder::primary_input_path`], the setting will be
    /// overridden.
    pub fn primary_input_stdin(&mut self) -> &mut Self {
        self.primary_input = PrimaryInputMode::Stdin;
        self
    }

    /// Configures us to read the primary input (i.e. the main .tex file) from
    /// an in-memory buffer.
    ///
    /// If other functions that set the primary input mode are called (e.g.
    /// [`IoSetupBuilder::primary_input_path`], the setting will be
    /// overridden.
    pub fn primary_input_buffer(&mut self, buf: Vec<u8>) -> &mut Self {
        self.primary_input = PrimaryInputMode::Buffer(buf);
        self
    }

    /// Sets the path for the filesystem root.
    ///
    /// This should probably not actually be the root of the entire filesystem; rather, it is a
    /// path where the TeX engine will look for included files.
    pub fn filesystem_root<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.filesystem_root = path.as_ref().to_owned();
        self
    }

    /// Sets the path for the format cache.
    ///
    /// The IoSetup created from this object will cache format files if the
    /// primary bundle has been specified (by calling `self.bundle()`). This
    /// call sets the filesystem path where format files are cached. If left
    /// unset, it defaults to the filesystem root, which is probably not what
    /// you want.
    pub fn format_cache_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.format_cache_path = Some(path.as_ref().to_owned());
        self
    }

    /// Adds a bundle to the I/O configuration.
    pub fn bundle<T: 'static + Bundle>(&mut self, bundle: T) -> &mut Self {
        self.bundle = Some(Box::new(bundle));
        self
    }

    /// Determines whether to use stdout.
    ///
    /// If this is set to false, I/O that was destined to stdout is instead redirected to the
    /// memory I/O layer (where it can be examined using the `mem` field on [`IoSetup`]).
    pub fn use_genuine_stdout(&mut self, setting: bool) -> &mut Self {
        self.use_genuine_stdout = setting;
        self
    }

    /// Marks a path as hidden, meaning that the filesystem layer will pretend it doesn't exist.
    pub fn hide_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.hidden_input_paths.insert(path.as_ref().to_owned());
        self
    }

    /// Creates an `IoSetup`.
    ///
    /// # Panics
    ///
    /// Panics if no primary input mechanism was specified.
    pub fn create(mut self, status: &mut dyn StatusBackend) -> Result<IoSetup> {
        let format_cache = if let Some(ref mut b) = self.bundle {
            let default_path = self.filesystem_root.clone(); // unwrap_or_else() causes borrowck issues
            let format_cache_path = self.format_cache_path.unwrap_or(default_path);
            Some(FormatCache::new(b.get_digest(status)?, format_cache_path))
        } else {
            None
        };

        let pio: Box<dyn IoProvider> = match self.primary_input {
            PrimaryInputMode::Stdin => {
                Box::new(atry!(BufferedPrimaryIo::from_stdin(); ["error reading standard input"]))
            }

            PrimaryInputMode::Path(pip) => Box::new(FilesystemPrimaryInputIo::new(&pip)),

            PrimaryInputMode::Buffer(buf) => Box::new(BufferedPrimaryIo::from_buffer(buf)),

            PrimaryInputMode::Undefined => {
                panic!("no primary input mechanism specified");
            }
        };

        Ok(IoSetup {
            primary_input: pio,
            mem: MemoryIo::new(true),
            filesystem: FilesystemIo::new(
                &self.filesystem_root,
                false,
                true,
                self.hidden_input_paths,
            ),
            format_cache,
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
