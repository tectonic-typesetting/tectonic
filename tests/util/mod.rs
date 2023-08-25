// Copyright 2018-2021 the Tectonic Project
// Licensed under the MIT License.

//! Note: we need to store this code as `tests/util/mod.rs` rather than
//! `tests/util.rs` because otherwise Cargo thinks it is a test executable of
//! its own.

// An item is considered unused if at least one testing binary
// has no reference to it. This yields a lot of false-positives
// using this testing setup...
#![allow(dead_code)]

use flate2::read::GzDecoder;
use std::{
    collections::HashSet,
    default::Default,
    env,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tectonic::{errors::Result, io::memory::MemoryFileCollection};
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};

pub use tectonic::test_util::{test_path, TestBundle};

/// Set the magic environment variable that enables the testing infrastructure
/// embedded in the main Tectonic crate. This function is separated out from
/// the main crate because it embeds `CARGO_MANIFEST_DIR`, which is not
/// something we want to leak into the binary artifacts we produce.
pub fn set_test_root() {
    ::tectonic::test_util::set_test_root_augmented(env!("CARGO_MANIFEST_DIR"));
}

// Duplicated from Cargo's own testing code:
// https://github.com/rust-lang/cargo/blob/19fdb308/tests/cargotest/support/mod.rs#L305-L318
pub fn cargo_dir() -> PathBuf {
    env::var_os("CARGO_BIN_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            env::current_exe().ok().map(|mut path| {
                path.pop();
                if path.ends_with("deps") {
                    path.pop();
                }
                path
            })
        })
        .unwrap_or_else(|| panic!("CARGO_BIN_PATH wasn't set. Cannot continue running test"))
}

/// Generate a plain.fmt file using local files only -- a variety of tests
/// need such a file to exist.
///
/// Note that because tests are run in parallel, this can get quite racy. At
/// the moment we just let everybody write and overwrite the file, but we
/// could use a locking scheme to get smarter about this.
pub fn ensure_plain_format() -> Result<PathBuf> {
    use tectonic::{
        io::{try_open_file, FilesystemIo, FilesystemPrimaryInputIo, IoStack, MemoryIo},
        TexEngine,
    };
    use tectonic_status_base::NoopStatusBackend;

    let fmt_path = test_path(&["plain.fmt"]);

    if try_open_file(&fmt_path).is_not_available() {
        let mut mem = MemoryIo::new(true);

        let mut assets_dir = test_path(&["assets"]);
        let mut fs_support = FilesystemIo::new(&assets_dir, false, false, HashSet::new());

        assets_dir.push("plain");
        assets_dir.set_extension("tex");
        let mut fs_primary = FilesystemPrimaryInputIo::new(&assets_dir);

        {
            let io = IoStack::new(vec![&mut mem, &mut fs_primary, &mut fs_support]);
            let mut hooks = MinimalDriver::new(io);
            let mut status = NoopStatusBackend::default();
            let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);

            TexEngine::default()
                .halt_on_error_mode(true)
                .initex_mode(true)
                .process(&mut launcher, "UNUSED.fmt", "plain.tex")?;
        }

        let mut temp_fmt = tempfile::Builder::new()
            .prefix("plain_fmt")
            .rand_bytes(6)
            .tempfile_in(test_path(&[]))?;
        temp_fmt.write_all(&mem.files.borrow().get("plain.fmt").unwrap().data)?;
        temp_fmt.persist(&fmt_path)?;
    }

    Ok(fmt_path)
}

/// Convenience structure for comparing expected and actual output in various
/// tests.
#[must_use = "Expectations do nothing if not `finish`ed"]
pub struct Expected<'a> {
    files: Vec<ExpectedFile<'a>>,
}

impl<'a> Expected<'a> {
    pub fn new() -> Self {
        Expected { files: Vec::new() }
    }

    pub fn file(mut self, file: ExpectedFile<'a>) -> Self {
        self.files.push(file);
        self
    }

    pub fn finish(&self) {
        let mut failures = Vec::new();
        for file in &self.files {
            if let Err(msg) = file.finish() {
                failures.push(msg);
            }
        }
        if !failures.is_empty() {
            panic!("Expectations not met:\n{}", failures.join("\n\n"));
        }
    }
}

pub struct ExpectedFile<'a> {
    name: String,
    contents: Vec<u8>,
    gzipped: bool,

    output: Option<ExpectedOutTy<'a>>,
}

impl<'a> ExpectedFile<'a> {
    pub fn read<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let name = path
            .file_name()
            .unwrap_or_else(|| panic!("couldn't get file name of {:?}", path))
            .to_str()
            .unwrap_or_else(|| panic!("couldn't Unicode-ify file name of {:?}", path))
            .to_owned();

        let mut f = File::open(path).unwrap_or_else(|_| panic!("failed to open {:?}", path));
        let mut contents = Vec::new();
        f.read_to_end(&mut contents).unwrap();

        ExpectedFile {
            name,
            contents,
            gzipped: false,

            output: None,
        }
    }

    pub fn read_with_extension(pbase: &mut PathBuf, extension: &str) -> Self {
        pbase.set_extension(extension);
        Self::read(pbase)
    }

    /// Special handling for synctex files -- we need to decode the gzip and
    /// fill in the absolute paths of the output files (cf. #720)
    pub fn read_with_extension_rooted_gz(pbase: &mut PathBuf, extension: &str) -> Self {
        pbase.set_extension(extension);
        let name = pbase.file_name().unwrap().to_str().unwrap().to_owned();

        let mut dec = GzDecoder::new(File::open(&pbase).unwrap());
        let mut contents = Vec::new();
        dec.read_to_end(&mut contents).unwrap();

        // Special SyncTeX rooting. We need a *mega* hack since there is a
        // byte-offset field whose value depends on the length of the file
        // prefix.
        let root = format!(
            "{}{}",
            pbase.parent().unwrap().to_str().unwrap(),
            std::path::MAIN_SEPARATOR
        );
        let contents = String::from_utf8(contents)
            .unwrap()
            .replace("${ROOT}", &root)
            .replace(
                "${len(ROOT)+106}",
                &(root.as_bytes().len() + 106).to_string(),
            )
            .into_bytes();

        ExpectedFile {
            name,
            contents,
            gzipped: true,

            output: None,
        }
    }

    pub fn data(mut self, observed: &'a [u8]) -> Self {
        self.output = Some(ExpectedOutTy::Raw(observed));
        self
    }

    pub fn collection(mut self, files: &'a MemoryFileCollection) -> Self {
        self.output = Some(ExpectedOutTy::FromCollection(files));
        self
    }

    fn finish_data(&self, observed: &[u8]) -> core::result::Result<(), String> {
        if self.contents == observed {
            return Ok(());
        }

        // For nontrivial tests, it's really tough to figure out what
        // changed without being able to do diffs, etc. So, write out the
        // buffers.
        {
            let mut n = self.name.clone();
            n.push_str(".expected");
            let mut f = File::create(&n)
                .map_err(|_| format!("failed to create {} for test failure diagnosis", n))?;
            f.write_all(&self.contents)
                .map_err(|_| format!("failed to write {} for test failure diagnosis", n))?;
        }
        {
            let mut n = self.name.clone();
            n.push_str(".observed");
            let mut f = File::create(&n)
                .map_err(|_| format!("failed to create {} for test failure diagnosis", n))?;
            f.write_all(observed)
                .map_err(|_| format!("failed to write {} for test failure diagnosis", n))?;
        }
        Err(format!(
            "difference in {}; contents saved to disk",
            self.name
        ))
    }

    fn finish(&self) -> core::result::Result<(), String> {
        match self.output {
            Some(ExpectedOutTy::Raw(observed)) => self.finish_data(observed),
            Some(ExpectedOutTy::FromCollection(files)) => {
                if !self.gzipped {
                    if let Some(file) = files.get(&self.name) {
                        self.finish_data(&file.data)
                    } else {
                        Err(format!(
                            "{:?} not in {:?}",
                            self.name,
                            files.keys().collect::<Vec<_>>()
                        ))
                    }
                } else {
                    let mut buf = Vec::new();
                    let mut dec = GzDecoder::new(&files.get(&self.name).unwrap().data[..]);
                    dec.read_to_end(&mut buf).unwrap();
                    self.finish_data(&buf)
                }
            }
            None => Err(format!(
                "No expected output provided for file {}",
                self.name
            )),
        }
    }
}

enum ExpectedOutTy<'a> {
    Raw(&'a [u8]),
    FromCollection(&'a MemoryFileCollection),
}
