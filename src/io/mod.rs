// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic’s pluggable I/O backend.

use std::{io::Read, str::FromStr};
use tectonic_errors::{anyhow::bail, atry, Result};
use tectonic_status_base::StatusBackend;

pub mod cached_itarbundle;
pub mod dirbundle;
pub mod format_cache;
pub mod memory;
pub mod setup;
pub mod zipbundle;

// Convenience re-exports.

pub use tectonic_io_base::{
    digest::{self, DigestData},
    filesystem::{FilesystemIo, FilesystemPrimaryInputIo},
    normalize_tex_path,
    stack::IoStack,
    stdstreams::GenuineStdoutIo,
    try_open_file, InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle,
};

// Internal Reexports

pub use self::{
    memory::MemoryIo,
    setup::{IoSetup, IoSetupBuilder},
};

/// A special IoProvider that can make TeX format files.
///
/// A “bundle” is expected to contain a large number of TeX support files —
/// for instance, a compilation of a TeXLive distribution. In terms of the
/// software architecture, though, what is special about a bundle is that one
/// can generate one or more TeX format files from its contents without
/// reference to any other I/O resources.
pub trait Bundle: IoProvider {
    /// Get a cryptographic digest summarizing this bundle’s contents.
    ///
    /// The digest summarizes the exact contents of every file in the bundle.
    /// It is computed from the sorted names and SHA256 digests of the
    /// component files [as implemented in the script
    /// builder/make-zipfile.py](https://github.com/tectonic-typesetting/tectonic-staging/blob/master/builder/make-zipfile.py#L138)
    /// in the `tectonic-staging` module.
    ///
    /// The default implementation gets the digest from a file name
    /// `SHA256SUM`, which is expected to contain the digest in hex-encoded
    /// format.
    fn get_digest(&mut self, status: &mut dyn StatusBackend) -> Result<DigestData> {
        let digest_text = match self.input_open_name(digest::DIGEST_NAME, status) {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                h.take(64).read_to_string(&mut text)?;
                text
            }

            OpenResult::NotAvailable => {
                // Broken or un-cacheable backend.
                bail!("bundle does not provide needed SHA256SUM file");
            }

            OpenResult::Err(e) => {
                return Err(e);
            }
        };

        Ok(atry!(DigestData::from_str(&digest_text); ["corrupted SHA256 digest data"]))
    }
}

impl<B: Bundle + ?Sized> Bundle for Box<B> {
    fn get_digest(&mut self, status: &mut dyn StatusBackend) -> Result<DigestData> {
        (**self).get_digest(status)
    }
}

// Helper for testing. FIXME: I want this to be conditionally compiled with
// #[cfg(test)] but things break if I do that.

pub mod testing {
    use super::*;
    use std::fs::File;
    use std::path::{Path, PathBuf};

    pub struct SingleInputFileIo {
        name: String,
        full_path: PathBuf,
    }

    impl SingleInputFileIo {
        pub fn new(path: &Path) -> SingleInputFileIo {
            let p = path.to_path_buf();

            SingleInputFileIo {
                name: p.file_name().unwrap().to_str().unwrap().to_owned(),
                full_path: p,
            }
        }
    }

    impl IoProvider for SingleInputFileIo {
        fn output_open_name(&mut self, _: &str) -> OpenResult<OutputHandle> {
            OpenResult::NotAvailable
        }

        fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
            OpenResult::NotAvailable
        }

        fn input_open_name(
            &mut self,
            name: &str,
            _status: &mut dyn StatusBackend,
        ) -> OpenResult<InputHandle> {
            if name == self.name {
                OpenResult::Ok(InputHandle::new(
                    name,
                    File::open(&self.full_path).unwrap(),
                    InputOrigin::Filesystem,
                ))
            } else {
                OpenResult::NotAvailable
            }
        }
    }
}
