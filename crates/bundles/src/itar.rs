// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! The web-friendly "indexed tar" bundle backend.
//!
//! The main type offered by this module is the [`ItarBundle`] struct,
//! which can (but should not) be used directly as a [`tectonic_io_base::IoProvider`].
//!
//! Instead, wrap it with a [`crate:cache::BundleCache`] for filesystem-backed
//! caching.
//!
//! While the on-server file format backing the “indexed tar” backend is indeed
//! a standard `tar` file, as far as the client is concerned, this backend is
//! centered on HTTP byte-range requests. For each file contained in the backing
//! resource, the index file merely contains a byte offset and length that are
//! then used to construct an HTTP Range request to obtain the file as needed.

use crate::Bundle;
use flate2::bufread;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::{collections::HashMap, io::BufReader};
use std::{thread, time};
use tectonic_errors::prelude::*;
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend, RangeReader};
use tectonic_io_base::OpenResult;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider};
use tectonic_status_base::{tt_note, tt_warning, StatusBackend};

const MAX_HTTP_ATTEMPTS: usize = 4;
const RETRY_SLEEP_MS: u64 = 250;

/// The internal file-information struct used by the [`ItarBundle`].
#[derive(Clone, Copy, Debug)]
pub struct FileInfo {
    offset: u64,
    length: usize,
}

/// A simple web-based file backend based on HTTP Range requests.
///
/// This bundle does not cache on its own, you probably want to wrap it
/// in a [`crate:cache::BundleCache`].
#[derive(Debug)]
pub struct ItarBundle {
    url: String,
    reader: DefaultRangeReader,

    /// Maps all available file names to [`FileInfo`]s.
    /// This is empty after we create this bundle, so we don't need network
    /// to make an object. It is automatically filled by get_index when we need it.
    index: HashMap<String, FileInfo>,
}

impl ItarBundle {
    /// Make a new ItarBundle
    pub fn new(url: String, status: &mut dyn StatusBackend) -> Result<ItarBundle> {
        // Step 1: resolve URL
        let mut geturl_backend = DefaultBackend::default();
        let resolved_url = geturl_backend.resolve_url(&url, status)?;

        Ok(ItarBundle {
            index: HashMap::new(),
            url,
            reader: geturl_backend.open_range_reader(&resolved_url),
        })
    }

    /// Fill self.index
    fn get_index(&mut self, status: &mut dyn StatusBackend) -> Result<()> {
        let mut geturl_backend = DefaultBackend::default();

        let index_url = format!("{}.index.gz", &self.url);
        tt_note!(status, "downloading index {}", index_url);
        let reader =
            bufread::GzDecoder::new(BufReader::new(geturl_backend.get_url(&index_url, status)?));

        self.index.clear();
        for line in BufReader::new(reader).lines() {
            if let Ok((name, info)) = Self::parse_index_line(&line?) {
                self.index.insert(name, info);
            }
        }

        return Ok(());
    }

    /// Parse one line of index file
    fn parse_index_line(line: &str) -> Result<(String, FileInfo)> {
        let mut bits = line.split_whitespace();

        if let (Some(name), Some(offset), Some(length)) = (bits.next(), bits.next(), bits.next()) {
            Ok((
                name.to_owned(),
                FileInfo {
                    offset: offset.parse::<u64>()?,
                    length: length.parse::<usize>()?,
                },
            ))
        } else {
            // TODO: preserve the warning info or something!
            bail!("malformed index line");
        }
    }
}

impl IoProvider for ItarBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        // Fetch index if it is empty
        if self.index.len() == 0 {
            match self.get_index(status) {
                Err(e) => return OpenResult::Err(e),
                _ => {}
            };
        }

        tt_note!(status, "downloading {}", name);

        let info = match self.index.get(name) {
            Some(a) => a,
            None => return OpenResult::NotAvailable,
        };

        let mut buf = Vec::with_capacity(info.length);

        // Our HTTP implementation actually has problems with zero-sized ranged
        // reads (Azure gives us a 200 response, which we don't properly
        // handle), but when the file is 0-sized we're all set anyway!
        if info.length == 0 {
            return OpenResult::Ok(InputHandle::new_read_only(
                name,
                Cursor::new(buf),
                InputOrigin::Other,
            ));
        }

        // Get file with retries
        for n in 0..MAX_HTTP_ATTEMPTS {
            let mut stream = match self.reader.read_range(info.offset, info.length) {
                Ok(r) => r,
                Err(e) => {
                    tt_warning!(status, "failure requesting \"{}\" from network", name; e);
                    thread::sleep(time::Duration::from_millis(RETRY_SLEEP_MS));
                    continue;
                }
            };

            if let Err(e) = stream.read_to_end(&mut buf) {
                tt_warning!(status, "failure downloading \"{}\" from network", name; e.into());
                thread::sleep(time::Duration::from_millis(RETRY_SLEEP_MS));
                continue;
            }

            if n == MAX_HTTP_ATTEMPTS - 1 {
                // All attempts failed
                return OpenResult::Err(anyhow!(
                    "failed to retrieve \"{}\" from the network;
                    this most probably is not Tectonic's fault \
                    -- please check your network connection.",
                    name
                ));
            } else if n != 0 {
                // At least one attempt failed
                tt_note!(status, "download succeeded after retry");
            }
            break;
        }

        return OpenResult::Ok(InputHandle::new_read_only(
            name,
            Cursor::new(buf),
            InputOrigin::Other,
        ));
    }
}

impl Bundle for ItarBundle {
    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        if self.index.len() == 0 {
            // Try to fetch index if it is empty
            let _ = self.get_index(status);
        }

        Ok(self.index.keys().cloned().collect())
    }

    fn get_location(&mut self) -> String {
        return self.url.clone();
    }
}
