// Copyright 2018-2020 the Tectonic Project
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

pub use tectonic::test_util::{test_path, TestBundle};
use tectonic::{errors::Result, io::memory::MemoryFileCollection};

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
    use ::tectonic::engines::NoopIoEventBackend;
    use ::tectonic::io::{
        try_open_file, FilesystemIo, FilesystemPrimaryInputIo, IoStack, MemoryIo,
    };
    use ::tectonic::TexEngine;
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
            let mut io = IoStack::new(vec![&mut mem, &mut fs_primary, &mut fs_support]);

            TexEngine::new()
                .halt_on_error_mode(true)
                .initex_mode(true)
                .process(
                    &mut io,
                    &mut NoopIoEventBackend::default(),
                    &mut NoopStatusBackend::default(),
                    "UNUSED.fmt",
                    "plain.tex",
                    &Default::default(),
                )?;
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
pub struct ExpectedInfo {
    name: String,
    contents: Vec<u8>,
    gzipped: bool,
}

impl ExpectedInfo {
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

        ExpectedInfo {
            name,
            contents,
            gzipped: false,
        }
    }

    pub fn read_with_extension(pbase: &mut PathBuf, extension: &str) -> Self {
        pbase.set_extension(extension);
        Self::read(pbase)
    }

    pub fn read_with_extension_gz(pbase: &mut PathBuf, extension: &str) -> Self {
        pbase.set_extension(extension);
        let name = pbase.file_name().unwrap().to_str().unwrap().to_owned();

        let mut dec = GzDecoder::new(File::open(pbase).unwrap());
        let mut contents = Vec::new();
        dec.read_to_end(&mut contents).unwrap();

        ExpectedInfo {
            name,
            contents,
            gzipped: true,
        }
    }

    pub fn test_data(&self, observed: &[u8]) {
        if self.contents == observed {
            return;
        }

        // For nontrivial tests, it's really tough to figure out what
        // changed without being able to do diffs, etc. So, write out the
        // buffers.
        {
            let mut n = self.name.clone();
            n.push_str(".expected");
            let mut f = File::create(&n)
                .unwrap_or_else(|_| panic!("failed to create {} for test failure diagnosis", n));
            f.write_all(&self.contents)
                .unwrap_or_else(|_| panic!("failed to write {} for test failure diagnosis", n));
        }
        {
            let mut n = self.name.clone();
            n.push_str(".observed");
            let mut f = File::create(&n)
                .unwrap_or_else(|_| panic!("failed to create {} for test failure diagnosis", n));
            f.write_all(observed)
                .unwrap_or_else(|_| panic!("failed to write {} for test failure diagnosis", n));
        }
        panic!("difference in {}; contents saved to disk", self.name);
    }

    pub fn test_from_collection(&self, files: &MemoryFileCollection) {
        if !self.gzipped {
            if let Some(file) = files.get(&self.name) {
                self.test_data(&file.data)
            } else {
                panic!(
                    "{:?} not in {:?}",
                    self.name,
                    files.keys().collect::<Vec<_>>()
                )
            }
        } else {
            let mut buf = Vec::new();
            let mut dec = GzDecoder::new(&files.get(&self.name).unwrap().data[..]);
            dec.read_to_end(&mut buf).unwrap();
            self.test_data(&buf);
        }
    }
}
