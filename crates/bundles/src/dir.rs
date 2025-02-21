// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! A module for the directory bundle [`DirBundle`].

use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{digest, filesystem::FilesystemIo, InputHandle, IoProvider, OpenResult};
use tectonic_status_base::{NoopStatusBackend, StatusBackend};

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
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        self.0.input_open_name(name, status)
    }

    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        self.0.input_open_name_with_abspath(name, status)
    }
}

impl Bundle for DirBundle {
    fn all_files(&self) -> Vec<String> {
        fs::read_dir(self.0.root())
            .unwrap()
            .filter_map(|x| x.ok())
            .filter(|x| !x.file_type().map(|x| x.is_dir()).unwrap_or(false))
            .map(|x| x.file_name().to_str().unwrap_or("").to_owned())
            .filter(|x| !x.is_empty())
            .collect()
    }

    fn get_digest(&mut self) -> Result<tectonic_io_base::digest::DigestData> {
        let digest_text = match self.input_open_name(digest::DIGEST_NAME, &mut NoopStatusBackend {})
        {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                h.take(64).read_to_string(&mut text)?;
                text
            }

            OpenResult::NotAvailable => {
                bail!("bundle does not provide needed SHA256SUM file");
            }

            OpenResult::Err(e) => {
                return Err(e);
            }
        };

        Ok(atry!(digest::DigestData::from_str(&digest_text); ["corrupted SHA256 digest data"]))
    }
}
