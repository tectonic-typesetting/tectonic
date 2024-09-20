// Copyright 2023-2024 the Tectonic Project
// Licensed under the MIT License.

//! Read ttb v1 bundles on the internet.
//!
//! The main type offered by this module is the [`TTBNetBundle`] struct,
//! which can (but should not) be used directly as a [`tectonic_io_base::IoProvider`].
//!
//! Instead, wrap it in a [`crate::BundleCache`] for filesystem-backed caching.

use crate::{
    ttb::{TTBFileIndex, TTBFileInfo, TTBv1Header},
    Bundle, CachableBundle, FileIndex, FileInfo, NET_RETRY_ATTEMPTS, NET_RETRY_SLEEP_MS,
};
use flate2::read::GzDecoder;
use std::{
    convert::TryFrom,
    io::{Cursor, Read},
    thread,
    time::Duration,
};
use tectonic_errors::prelude::*;
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend, RangeReader};
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::{tt_note, tt_warning, StatusBackend};

/// Read a [`TTBFileInfo`] from this bundle.
/// We assume that `fileinfo` points to a valid file in this bundle.
fn read_fileinfo(fileinfo: &TTBFileInfo, reader: &mut DefaultRangeReader) -> Result<Box<dyn Read>> {
    // fileinfo.length is a u32, so it must fit inside a usize (assuming 32/64-bit machine).
    let stream = reader.read_range(fileinfo.start, fileinfo.gzip_len as usize)?;
    Ok(Box::new(GzDecoder::new(stream)))
}

/// Access ttbv1 bundle hosted on the internet.
/// This struct provides NO caching. All files
/// are downloaded.
///
/// As such, this bundle should probably be wrapped in a [`crate::BundleCache`].
pub struct TTBNetBundle<T>
where
    for<'a> T: FileIndex<'a>,
{
    url: String,
    index: T,

    // We need the network to load these.
    // They're None until absolutely necessary.
    reader: Option<DefaultRangeReader>,
}

/// The internal file-information struct used by the [`TTBNetBundle`].

impl TTBNetBundle<TTBFileIndex> {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    /// This method does not require network access.
    /// It will succeed even in we can't connect to the bundle, or if we're given a bad url.
    pub fn new(url: String) -> Result<Self> {
        Ok(TTBNetBundle {
            reader: None,
            index: TTBFileIndex::default(),
            url,
        })
    }

    fn connect_reader(&mut self) -> Result<()> {
        if self.reader.is_some() {
            return Ok(());
        }
        let geturl_backend = DefaultBackend::default();
        self.reader = Some(geturl_backend.open_range_reader(&self.url));
        Ok(())
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
        Ok(header)
    }

    // Fill this bundle's index if it is empty.
    fn ensure_index(&mut self) -> Result<()> {
        if self.index.is_initialized() {
            return Ok(());
        }

        let mut reader = self.get_index_reader()?;
        self.index.initialize(&mut reader)?;
        Ok(())
    }
}

impl IoProvider for TTBNetBundle<TTBFileIndex> {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        if let Err(e) = self.ensure_index() {
            return OpenResult::Err(e);
        };

        let info = match self.search(name) {
            None => return OpenResult::NotAvailable,
            Some(s) => s,
        };

        // Retries are handled in open_fileinfo,
        // since BundleCache never calls input_open_name.
        self.open_fileinfo(&info, status)
    }
}

impl Bundle for TTBNetBundle<TTBFileIndex> {
    fn all_files(&self) -> Vec<String> {
        self.index.iter().map(|x| x.path().to_owned()).collect()
    }

    fn get_digest(&mut self) -> Result<tectonic_io_base::digest::DigestData> {
        let header = self.get_header()?;
        Ok(header.digest)
    }
}

impl<'this> CachableBundle<'this, TTBFileIndex> for TTBNetBundle<TTBFileIndex> {
    fn get_location(&mut self) -> String {
        self.url.clone()
    }

    fn initialize_index(&mut self, source: &mut dyn Read) -> Result<()> {
        self.index.initialize(source)?;
        Ok(())
    }

    fn index(&mut self) -> &mut TTBFileIndex {
        &mut self.index
    }

    fn search(&mut self, name: &str) -> Option<TTBFileInfo> {
        self.index.search(name)
    }

    fn get_index_reader(&mut self) -> Result<Box<dyn Read>> {
        self.connect_reader()?;
        let header = self.get_header()?;

        read_fileinfo(
            &TTBFileInfo {
                start: header.index_start,
                gzip_len: header.index_gzip_len,
                real_len: header.index_real_len,
                path: "".to_owned(),
                name: "".to_owned(),
                hash: None,
            },
            self.reader.as_mut().unwrap(),
        )
    }

    fn open_fileinfo(
        &mut self,
        info: &TTBFileInfo,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let mut v: Vec<u8> = Vec::with_capacity(info.real_len as usize);
        tt_note!(status, "downloading {}", info.name);

        // Edge case for zero-sized reads
        // (these cause errors on some web hosts)
        if info.gzip_len == 0 {
            return OpenResult::Ok(InputHandle::new_read_only(
                info.name.to_owned(),
                Cursor::new(v),
                InputOrigin::Other,
            ));
        }

        // Get file with retries
        for i in 0..NET_RETRY_ATTEMPTS {
            let mut reader = match read_fileinfo(info, self.reader.as_mut().unwrap()) {
                Ok(r) => r,
                Err(e) => {
                    tt_warning!(status,
                        "failure fetching \"{}\" from network ({}/{NET_RETRY_ATTEMPTS})",
                        info.name, i+1; e
                    );
                    thread::sleep(Duration::from_millis(NET_RETRY_SLEEP_MS));
                    continue;
                }
            };

            match reader.read_to_end(&mut v) {
                Ok(_) => {}
                Err(e) => {
                    tt_warning!(status,
                        "failure downloading \"{}\" from network ({}/{NET_RETRY_ATTEMPTS})",
                        info.name, i+1; e.into()
                    );
                    thread::sleep(Duration::from_millis(NET_RETRY_SLEEP_MS));
                    continue;
                }
            };

            return OpenResult::Ok(InputHandle::new_read_only(
                info.name.to_owned(),
                Cursor::new(v),
                InputOrigin::Other,
            ));
        }

        OpenResult::Err(anyhow!(
            "failed to download \"{}\"; please check your network connection.",
            info.name
        ))
    }
}
