// src/io/filesystem.rs -- I/O on the local filesystem.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use errors::{ErrorKind, Result};
use super::{InputFeatures, InputHandle, IOProvider, OpenResult, OutputHandle};


// FilesystemIO is an I/O provider that reads, and optionally writes, files
// from a given root directory. NOTE: no effort is made to contain I/O within
// the specified root!! We have an option to disallow absolute paths, but we
// don't do anything about "../../../...." paths.

pub struct FilesystemIO {
    writes_allowed: bool,
    absolute_allowed: bool,
    root: PathBuf
}

impl FilesystemIO {
    pub fn new(root: &Path, writes_allowed: bool, absolute_allowed: bool) -> FilesystemIO {
        FilesystemIO {
            writes_allowed: writes_allowed,
            absolute_allowed: absolute_allowed,
            root: PathBuf::from(root),
        }
    }

    fn construct_path(&mut self, name: &OsStr) -> Result<PathBuf> {
        let path = Path::new(name);

        if path.is_absolute() && !self.absolute_allowed {
            let as_str = String::from(path.to_string_lossy());
            return Err(ErrorKind::PathForbidden(as_str).into());
        }

        let mut combined = PathBuf::from(&self.root);
        combined.push(path);
        Ok(combined)
    }
}


impl IOProvider for FilesystemIO {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        if !self.writes_allowed {
            return OpenResult::NotAvailable;
        }

        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e.into())
        };

        let f = match File::create (path) {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into())
        };

        OpenResult::Ok(Box::new(f))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        // TODO: option to record "stdout" into a file with a particular name?
        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e.into())
        };

        let f = match File::open (path) {
            Ok(f) => f,
            Err(e) => return if e.kind() == io::ErrorKind::NotFound {
                OpenResult::NotAvailable
            } else {
                OpenResult::Err(e.into())
            }
        };

        OpenResult::Ok(Box::new(BufReader::new(f)))
    }
}


impl InputFeatures for File {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.metadata()?.len() as usize)
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}


impl InputFeatures for BufReader<File> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_mut().metadata()?.len() as usize)
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}
