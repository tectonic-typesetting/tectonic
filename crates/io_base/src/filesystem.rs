// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic I/O implementations for `std::fs` types.

use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufReader, Seek, SeekFrom},
    path::{Path, PathBuf},
};
use tectonic_errors::Result;
use tectonic_status_base::StatusBackend;

use super::{
    try_open_file, InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle,
    TectonicIoError,
};

/// FilesystemPrimaryInputIo is an I/O provider that provides the TeX "primary input"
/// file off of the filesystem. This can *pretty much* be achieved with
/// Filesystem I/O, but we need the "primary input" formalism to decently support
/// processing if stdin, and by doing things this way we can handle paths on
/// Unix systems that can't be converted to UTF-8.
///
/// TODO: it might be technically superior to open the path immediately and
/// keep that handle open, rewinding as needed, but for now we're not doing
/// that.
#[derive(Debug)]
pub struct FilesystemPrimaryInputIo {
    path: PathBuf,
}

impl FilesystemPrimaryInputIo {
    /// Create a new I/O provider providing the Tectonic "primary input" from
    /// the specified path.
    pub fn new<P: AsRef<Path>>(path: P) -> FilesystemPrimaryInputIo {
        FilesystemPrimaryInputIo {
            path: path.as_ref().to_owned(),
        }
    }
}

impl IoProvider for FilesystemPrimaryInputIo {
    fn input_open_primary(&mut self, _status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        let f = match try_open_file(&self.path) {
            OpenResult::Ok(f) => f,
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
        };

        OpenResult::Ok(InputHandle::new(
            "",
            BufReader::new(f),
            InputOrigin::Filesystem,
        ))
    }
}

/// FilesystemIo is an I/O provider that reads, and optionally writes, files
/// from a given root directory.
///
/// NOTE: no effort is made to contain I/O within the specified root!! We have
/// an option to disallow absolute paths, but we don't do anything about
/// `../../../....` paths.
pub struct FilesystemIo {
    root: PathBuf,
    writes_allowed: bool,
    absolute_allowed: bool,
    hidden_input_paths: HashSet<PathBuf>,
}

impl FilesystemIo {
    /// Create a new filesystem I/O provider rooted at the specified path.
    pub fn new(
        root: &Path,
        writes_allowed: bool,
        absolute_allowed: bool,
        hidden_input_paths: HashSet<PathBuf>,
    ) -> FilesystemIo {
        FilesystemIo {
            root: PathBuf::from(root),
            writes_allowed,
            absolute_allowed,
            hidden_input_paths,
        }
    }

    fn construct_path(&mut self, name: &str) -> Result<PathBuf> {
        let path = Path::new(name);

        if path.is_absolute() && !self.absolute_allowed {
            return Err(TectonicIoError::PathForbidden(path.to_owned()).into());
        }

        let mut combined = PathBuf::from(&self.root);
        combined.push(path);
        Ok(combined)
    }
}

impl IoProvider for FilesystemIo {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        if !self.writes_allowed {
            return OpenResult::NotAvailable;
        }

        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e),
        };

        let f = match File::create(path) {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into()),
        };

        OpenResult::Ok(OutputHandle::new(name, f))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        // TODO: option to record "stdout" into a file with a particular name?
        OpenResult::NotAvailable
    }

    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e),
        };

        if self.hidden_input_paths.contains(&path) {
            return OpenResult::NotAvailable;
        }

        let f = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return if e.kind() == io::ErrorKind::NotFound {
                    OpenResult::NotAvailable
                } else if let Some(libc::ENOTDIR) = e.raw_os_error() {
                    // xdvipdfmx has a code path that basically tries to open a
                    // font path assuming that it is a directory, which causes an
                    // ENOTDIR to happen (i.e., it tries the equivalent of
                    // open("/etc/passwd/subdir"). This circumstance is harmless.
                    OpenResult::NotAvailable
                } else {
                    OpenResult::Err(e.into())
                };
            }
        };

        OpenResult::Ok(InputHandle::new(
            name,
            BufReader::new(f),
            InputOrigin::Filesystem,
        ))
    }
}

impl InputFeatures for File {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.metadata()?.len() as usize)
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        let sys_time = self.metadata()?.modified()?;

        // No cleaner way to convert a SystemTime to time_t, as far as I can
        // tell.
        let dur = sys_time.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
        Ok(Some(dur.as_secs() as i64))
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

impl InputFeatures for BufReader<File> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_mut().metadata()?.len() as usize)
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        let sys_time = self.get_mut().metadata()?.modified()?;
        let dur = sys_time.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
        Ok(Some(dur.as_secs() as i64))
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}
