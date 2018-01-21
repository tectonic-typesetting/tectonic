// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! Note: we need to store this code as `tests/util/mod.rs` rather than
//! `tests/util.rs` because otherwise Cargo thinks it is a test executable of
//! its own.

// An item is considered unused if at least one testing binary
// has no reference to it. This yields a lot of false-positives
// using this testing setup...
#![allow(dead_code)]

use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const TOP: &str = env!("CARGO_MANIFEST_DIR");


pub fn test_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(TOP);
    path.push("tests");
    path.push(parts.iter().collect::<PathBuf>());
    path
}


// Duplicated from Cargo's own testing code:
// https://github.com/rust-lang/cargo/blob/19fdb308/tests/cargotest/support/mod.rs#L305-L318
pub fn cargo_dir() -> PathBuf {
    env::var_os("CARGO_BIN_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            env::current_exe()
                .ok()
                .map(|mut path| {
                         path.pop();
                         if path.ends_with("deps") {
                             path.pop();
                         }
                         path
                     })
        })
        .unwrap_or_else(|| panic!("CARGO_BIN_PATH wasn't set. Cannot continue running test"))
}


/// Convenience structure for comparing expected and actual output in various
/// tests.
pub struct ExpectedInfo {
    name: OsString,
    contents: Vec<u8>,
    gzipped: bool,
}

impl ExpectedInfo {
    pub fn read<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let name = path.file_name().unwrap().to_owned();

        let mut f = File::open(path).unwrap();
        let mut contents = Vec::new();
        f.read_to_end(&mut contents).unwrap();

        ExpectedInfo { name: name, contents: contents, gzipped: false }
    }

    pub fn read_with_extension(pbase: &mut PathBuf, extension: &str) -> Self {
        pbase.set_extension(extension);
        Self::read(pbase)
    }

    pub fn read_with_extension_gz(pbase: &mut PathBuf, extension: &str) -> Self {
        pbase.set_extension(extension);
        let name = pbase.file_name().unwrap().to_owned();

        let mut dec = GzDecoder::new(File::open(pbase).unwrap());
        let mut contents = Vec::new();
        dec.read_to_end(&mut contents).unwrap();

        ExpectedInfo { name: name, contents: contents, gzipped: true }
    }

    pub fn test_data(&self, observed: &Vec<u8>) {
        if &self.contents == observed {
            return;
        }

        // For nontrivial tests, it's really tough to figure out what
        // changed without being able to do diffs, etc. So, write out the
        // buffers.
        {
            let mut n = self.name.clone();
            n.push(".expected");
            let mut f = File::create(&n).expect(&format!("failed to create {} for test failure diagnosis", n.to_string_lossy()));
            f.write_all(&self.contents).expect(&format!("failed to write {} for test failure diagnosis", n.to_string_lossy()));
        }
        {
            let mut n = self.name.clone();
            n.push(".observed");
            let mut f = File::create(&n).expect(&format!("failed to create {} for test failure diagnosis", n.to_string_lossy()));
            f.write_all(observed).expect(&format!("failed to write {} for test failure diagnosis", n.to_string_lossy()));
        }
        panic!("difference in {}; contents saved to disk", self.name.to_string_lossy());
    }

    pub fn test_from_collection(&self, files: &HashMap<OsString, Vec<u8>>) {
        if !self.gzipped {
            self.test_data(files.get(&self.name).unwrap())
        } else {
            let mut buf = Vec::new();
            let mut dec = GzDecoder::new(&files.get(&self.name).unwrap()[..]);
            dec.read_to_end(&mut buf).unwrap();
            self.test_data(&buf);
        }
    }
}
