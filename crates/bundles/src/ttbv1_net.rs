// Copyright 2023-2024 the Tectonic Project
// Licensed under the MIT License.

//! Read ttb v1 bundles on the internet.
//!
//! The main type offered by this module is the [`Ttbv1NetBundle`] struct,
//! which can (but should not) be used directly as a [`tectonic_io_base::IoProvider`].
//!
//! Instead, wrap it in a [`crate::BundleCache`] for filesystem-backed caching.

use std::{
    convert::TryFrom,
    io::{Cursor, Read},
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::StatusBackend;

use crate::{
    ttbv1::{TTBFileIndex, TTBFileInfo, TTBv1Header},
    Bundle, CachableBundle, FileIndex,
};
use flate2::read::GzDecoder;

use tectonic_geturl::DefaultRangeReader;
use tectonic_geturl::{DefaultBackend, GetUrlBackend, RangeReader};

/// Read a [`TTBFileInfo`] from this bundle.
/// We assume that `fileinfo` points to a valid file in this bundle.
fn read_fileinfo(fileinfo: &TTBFileInfo, reader: &mut DefaultRangeReader) -> Result<Box<dyn Read>> {
    // fileinfo.length is a u32, so it must fit inside a usize (assuming 32/64-bit machine).
    let stream = reader.read_range(fileinfo.start, fileinfo.length as usize)?;
    return Ok(Box::new(GzDecoder::new(stream)));
}

/// Access ttbv1 bundle hosted on the internet.
/// This struct provides NO caching. All files
/// are downloaded.
///
/// As such, this bundle should probably be wrapped in a [`crate::BundleCache`].
pub struct Ttbv1NetBundle {
    url: String,
    index: TTBFileIndex,

    // We need the network to load these.
    // They're None until absolutely necessary.
    reader: Option<DefaultRangeReader>,
}

/// The internal file-information struct used by the [`Ttbv1NetBundle`].

impl Ttbv1NetBundle {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(url: String) -> Result<Ttbv1NetBundle> {
        Ok(Ttbv1NetBundle {
            reader: None,
            index: TTBFileIndex::new(),
            url,
        })
    }

    fn connect_reader(&mut self) -> Result<()> {
        if self.reader.is_some() {
            return Ok(());
        }
        let geturl_backend = DefaultBackend::default();
        self.reader = Some(geturl_backend.open_range_reader(&self.url));
        return Ok(());
    }

    fn get_header(&mut self) -> Result<TTBv1Header> {
        self.connect_reader()?;
        let mut header: [u8; 70] = [0u8; 70];
        self.reader
            .as_mut()
            .unwrap()
            .read_range(0, 70)?
            .read_exact(&mut header)?;
        let header = TTBv1Header::try_from(header)?;
        return Ok(header);
    }

    // Fill this bundle's index if it is empty.
    fn ensure_index(&mut self) -> Result<()> {
        if self.index.is_initialized() {
            return Ok(());
        }

        let mut reader = self.get_index_reader()?;
        self.index.initialize(&mut reader)?;
        return Ok(());
    }
}

impl IoProvider for Ttbv1NetBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        match self.ensure_index() {
            Err(e) => return OpenResult::Err(e),
            _ => {}
        };

        let info = match self.index.search(&name) {
            None => return OpenResult::NotAvailable,
            Some(s) => s,
        };

        return self.open_fileinfo(&info);
    }
}

impl Bundle for Ttbv1NetBundle {
    fn all_files(&mut self, _status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        Ok(self.index.iter().map(|x| x.path.clone()).collect())
    }

    fn get_digest(
        &mut self,
        _status: &mut dyn StatusBackend,
    ) -> Result<tectonic_io_base::digest::DigestData> {
        let header = self.get_header()?;
        return Ok(header.digest);
    }
}

impl<'this> CachableBundle<'this, TTBFileInfo, TTBFileIndex> for Ttbv1NetBundle {
    fn get_location(&mut self) -> String {
        return self.url.clone();
    }

    fn initialize_index(&mut self, source: &mut dyn Read) -> Result<()> {
        self.index.initialize(source)?;
        return Ok(());
    }

    fn index(&mut self) -> &mut TTBFileIndex {
        return &mut self.index;
    }

    fn search(&mut self, name: &str) -> Option<TTBFileInfo> {
        return self.index.search(name);
    }

    fn get_index_reader(&mut self) -> Result<Box<dyn Read>> {
        self.connect_reader()?;
        let header = self.get_header()?;

        return Ok(read_fileinfo(
            &TTBFileInfo {
                start: header.index_start,
                length: header.index_len,
                path: "".to_owned(),
                name: "".to_owned(),
                hash: None,
            },
            self.reader.as_mut().unwrap(),
        )?);
    }

    fn open_fileinfo(&mut self, info: &TTBFileInfo) -> OpenResult<InputHandle> {
        let mut v: Vec<u8> = Vec::new();

        let mut reader = match read_fileinfo(&info, self.reader.as_mut().unwrap()) {
            Ok(r) => r,
            Err(e) => return OpenResult::Err(e.into()),
        };

        match reader.read_to_end(&mut v) {
            Ok(_) => {}
            Err(e) => return OpenResult::Err(e.into()),
        };

        return OpenResult::Ok(InputHandle::new_read_only(
            info.name.to_owned(),
            Cursor::new(v),
            InputOrigin::Other,
        ));
    }
}
