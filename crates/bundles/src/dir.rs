// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! A module for the directory bundle [`DirBundle`].

use std::{
    fs,
    path::{Path, PathBuf},
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{filesystem::FilesystemIo, InputHandle, IoProvider, OpenResult};

use super::Bundle;

/// A "bundle" of a bunch of files in a directory.
///
/// This implementation essentially just wraps
/// [`tectonic_io_base::filesystem::FilesystemIo`], ensuring that it is
/// read-only, self-contained, and implements the [`Bundle`] trait. The
/// directory should contain a file named `SHA256SUM` if the bundle fingerprint
/// will be needed.
pub struct DirBundle(FilesystemIo);

impl DirBundle {
    /// Create a new directory bundle.
    ///
    /// No validation of the input path is performed, which is why this function
    /// is infallible.
    pub fn new<P: AsRef<Path>>(dir: P) -> DirBundle {
        DirBundle(FilesystemIo::new(
            dir.as_ref(),
            false,              // no writes
            false,              // no absolute paths
            Default::default(), // no hidden files
        ))
    }
}

impl IoProvider for DirBundle {
    fn input_open_name(&mut self, name: &str) -> OpenResult<InputHandle> {
        self.0.input_open_name(name)
    }

    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        self.0.input_open_name_with_abspath(name)
    }
}

impl Bundle for DirBundle {
    fn all_files(&mut self) -> Result<Vec<String>> {
        let mut files = Vec::new();

        // We intentionally do not explore the directory recursively.
        for entry in fs::read_dir(self.0.root())? {
            let entry = entry?;

            // This catches both regular files and symlinks:`
            if !entry.file_type()?.is_dir() {
                if let Some(s) = entry.file_name().to_str() {
                    files.push(s.to_owned());
                }
            }
        }

        Ok(files)
    }
}
