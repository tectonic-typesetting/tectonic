// Copyright 2023-2024 the Tectonic Project
// Licensed under the MIT License.

//! Read ttb v1 bundles on the filesystem.
//!
//! The main type offered by this module is the [`Ttbv1NetBundle`] struct.

use crate::{
    ttbv1::{TTBFileIndex, TTBFileInfo, TTBv1Header},
    Bundle, FileIndex,
};
use flate2::read::GzDecoder;
use std::{
    convert::TryFrom,
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{digest::DigestData, InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::StatusBackend;

/// Read a [`TTBFileInfo`] from this bundle.
/// We assume that `fileinfo` points to a valid file in this bundle.
fn read_fileinfo<'a>(fileinfo: &TTBFileInfo, reader: &'a mut File) -> Result<Box<dyn Read + 'a>> {
    reader.seek(SeekFrom::Start(fileinfo.start))?;
    return Ok(Box::new(GzDecoder::new(
        reader.take(fileinfo.gzip_len as u64),
    )));
}

/// A bundle backed by a ZIP file.
pub struct Ttbv1FsBundle {
    file: File,
    index: TTBFileIndex,
    header: TTBv1Header,
}

/// The internal file-information struct used by the [`Ttbv1FsBundle`].

impl<'a> Ttbv1FsBundle {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(mut file: File) -> Result<Ttbv1FsBundle> {
        // Parse header
        file.seek(SeekFrom::Start(0))?;
        let mut header: [u8; 70] = [0u8; 70];
        file.read_exact(&mut header)?;
        file.seek(SeekFrom::Start(0))?;
        let header = TTBv1Header::try_from(header)?;

        Ok(Ttbv1FsBundle {
            file,
            index: TTBFileIndex::new(),
            header,
        })
    }

    // Fill this bundle's search rules, fetching files from our backend.
    fn fill_index(&mut self) -> Result<()> {
        let info = TTBFileInfo {
            start: self.header.index_start,
            gzip_len: self.header.index_real_len,
            real_len: self.header.index_gzip_len,
            path: "/INDEX".to_owned(),
            name: "INDEX".to_owned(),
            hash: None,
        };

        let mut reader = read_fileinfo(&info, &mut self.file)?;
        self.index.initialize(&mut reader)?;

        return Ok(());
    }
}

impl Ttbv1FsBundle {
    /// Open a file on the filesystem as a zip bundle.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Ttbv1FsBundle> {
        Self::new(File::open(path)?)
    }
}

impl IoProvider for Ttbv1FsBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        // Fetch index if it is empty
        if self.index.len() == 0 {
            match self.fill_index() {
                Err(e) => return OpenResult::Err(e.into()),
                _ => {}
            }
        }

        let info = match self.index.search(&name) {
            None => return OpenResult::NotAvailable,
            Some(s) => s,
        };

        let mut v: Vec<u8> = Vec::with_capacity(info.real_len as usize);

        match read_fileinfo(&info, &mut self.file) {
            Err(e) => return OpenResult::Err(e.into()),
            Ok(mut b) => match b.read_to_end(&mut v) {
                Err(e) => return OpenResult::Err(e.into()),
                Ok(_) => {}
            },
        };

        return OpenResult::Ok(InputHandle::new_read_only(
            name,
            Cursor::new(v),
            InputOrigin::Other,
        ));
    }
}

impl Bundle for Ttbv1FsBundle {
    fn all_files(&mut self) -> Result<Vec<String>> {
        Ok(self.index.iter().map(|x| x.path.clone()).collect())
    }

    fn get_digest(&mut self) -> Result<DigestData> {
        return Ok(self.header.digest.clone());
    }
}
