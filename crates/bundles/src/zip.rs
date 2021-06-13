// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! ZIP files as Tectonic bundles.

use std::{
    fs::File,
    io::{Cursor, Read, Seek},
    path::Path,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::StatusBackend;
use zip::{result::ZipError, ZipArchive};

use crate::Bundle;

/// A bundle backed by a ZIP file.
pub struct ZipBundle<R: Read + Seek> {
    zip: ZipArchive<R>,
}

impl<R: Read + Seek> ZipBundle<R> {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(reader: R) -> Result<ZipBundle<R>> {
        Ok(ZipBundle {
            zip: ZipArchive::new(reader)?,
        })
    }
}

impl ZipBundle<File> {
    /// Open a file on the filesystem as a ZIP bundle.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ZipBundle<File>> {
        Self::new(File::open(path)?)
    }
}

impl<R: Read + Seek> IoProvider for ZipBundle<R> {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        // We need to be able to look at other items in the Zip file while
        // reading this one, so the only path forward is to read the entire
        // contents into a buffer right now. RAM is cheap these days.

        let mut zipitem = match self.zip.by_name(name) {
            Ok(f) => f,
            Err(e) => {
                return match e {
                    ZipError::Io(sube) => OpenResult::Err(sube.into()),
                    ZipError::FileNotFound => OpenResult::NotAvailable,
                    _ => OpenResult::Err(e.into()),
                };
            }
        };

        let mut buf = Vec::with_capacity(zipitem.size() as usize);

        if let Err(e) = zipitem.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(InputHandle::new_read_only(
            name,
            Cursor::new(buf),
            InputOrigin::Other,
        ))
    }
}

impl<R: Read + Seek> Bundle for ZipBundle<R> {
    fn all_files(&mut self, _status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        Ok(self.zip.file_names().map(|s| s.to_owned()).collect())
    }
}
