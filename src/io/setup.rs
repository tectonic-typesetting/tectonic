use std::collections::HashSet;
use std::path::{Path, PathBuf};

use errors::{Result, ResultExt};
use io::{FilesystemIo, FilesystemPrimaryInputIo, GenuineStdoutIo,
                   IoProvider, IoStack, MemoryIo};
use io::stdstreams::BufferedPrimaryIo;

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
    primary_input: Box<IoProvider>,
    pub bundle: Option<Box<IoProvider>>,
    pub mem: MemoryIo,
    filesystem: FilesystemIo,
    genuine_stdout: Option<GenuineStdoutIo>,
    format_primary: Option<BufferedPrimaryIo>,
}

impl IoSetup {
    pub fn as_stack<'a> (&'a mut self) -> IoStack<'a> {
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

    pub fn as_stack_for_format<'a> (&'a mut self, format_file_name: &str) -> IoStack<'a> {
        let mut providers: Vec<&mut IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }

        self.format_primary = Some(BufferedPrimaryIo::from_text(
                &format!("\\input {}", format_file_name)
        ));
        providers.push(self.format_primary.as_mut().unwrap());
        providers.push(&mut self.mem);

        if let Some(ref mut b) = self.bundle {
            providers.push(&mut **b);
        }

        IoStack::new(providers)
    }
}

/// The IoSetupBuilder provides a convenient builder interface for specifying
/// the I/O setup.

pub struct IoSetupBuilder {
    primary_input_path: Option<PathBuf>,
    filesystem_root: PathBuf,
    use_stdin: bool,
    bundle: Option<Box<IoProvider>>,
    use_genuine_stdout: bool,
    hidden_input_paths: HashSet<PathBuf>,
}

impl Default for IoSetupBuilder {
    fn default() -> Self {
        IoSetupBuilder {
            primary_input_path: None,
            filesystem_root: PathBuf::new(),
            use_stdin: false,
            bundle: None,
            use_genuine_stdout: false,
            hidden_input_paths: HashSet::new(),
        }
    }
}

impl IoSetupBuilder {
    /// Sets the path for the primary input (i.e. the main .tex file).
    ///
    /// This method is mutually exclusive with [`IoSetupBuilder::primary_input_stdin`]: you must
    /// call *exactly* one out of these two methods, or we will panic.
    ///
    /// # Panics
    ///
    /// Panics if `primary_input_stdin` was already invoked.
    pub fn primary_input_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        if self.use_stdin {
            panic!("cannot use both stdin and primary_input_path");
        }

        self.primary_input_path = Some(path.as_ref().to_owned());
        self
    }

    /// Configures us to read the primary input (i.e. the main .tex file) from stdin.
    ///
    /// This method is mutually exclusive with [`IoSetupBuilder::primary_input_path`]: you must
    /// call *exactly* one out of these two methods, or we will panic.
    ///
    /// # Panics
    ///
    /// Panics if `primary_input_path` was already invoked.
    pub fn primary_input_stdin(&mut self) -> &mut Self {
        if self.primary_input_path.is_some() {
            panic!("cannot use both primary_input_path and stdin");
        }

        self.use_stdin = true;
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

    /// Adds a bundle to the I/O configuration.
    pub fn bundle<T: 'static + IoProvider>(&mut self, bundle: T) -> &mut Self {
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

    pub fn hide_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.hidden_input_paths.insert(path.as_ref().to_owned());
        self
    }

    /// Creates an `IoSetup`.
    ///
    /// # Panics
    ///
    /// Panics if no primary input mechanism was specified.
    pub fn create(self) -> Result<IoSetup> {
        let pio: Box<IoProvider> = if self.use_stdin {
            Box::new(ctry!(BufferedPrimaryIo::from_stdin(); "error reading standard input"))
        } else if let Some(pip) = self.primary_input_path {
            Box::new(FilesystemPrimaryInputIo::new(&pip))
        } else {
            panic!("no primary input mechanism specified");
        };

        Ok(IoSetup {
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


