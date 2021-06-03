// src/io/zipbundle.rs -- I/O on files in a Zipped-up "bundle"
// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

use std::{
    fs::File,
    io::{Cursor, Read, Seek},
    path::Path,
};
use zip::{result::ZipError, ZipArchive};

use super::{Bundle, InputHandle, InputOrigin, IoProvider, OpenResult};
use crate::errors::Result;
use crate::status::StatusBackend;

pub struct ZipBundle<R: Read + Seek> {
    zip: ZipArchive<R>,
}

impl<R: Read + Seek> ZipBundle<R> {
    pub fn new(reader: R) -> Result<ZipBundle<R>> {
        Ok(ZipBundle {
            zip: ZipArchive::new(reader)?,
        })
    }
}

impl ZipBundle<File> {
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

impl<R: Read + Seek> Bundle for ZipBundle<R> {}
