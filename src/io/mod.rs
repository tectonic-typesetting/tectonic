// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! Extensions to Tectonic’s pluggable I/O backend.

use tectonic_status_base::StatusBackend;

pub mod format_cache;
pub mod memory;

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

pub use self::memory::MemoryIo;

// Helper for testing. FIXME: I want this to be conditionally compiled with
// #[cfg(test)] but things break if I do that.

#[doc(hidden)]
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
