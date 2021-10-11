// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! The web-friendly "indexed tar" bundle backend.
//!
//! The main type offered by this module is the [`IndexedTarBackend`] struct,
//! which cannot be used directly as a [`tectonic_io_base::IoProvider`] but is
//! the default backend for cached web-based bundle access through the
//! [`crate::cache::CachingBundle`] framework.
//!
//! While the on-server file format backing the “indexed tar” backend is indeed
//! a standard `tar` file, as far as the client is concerned, this backend is
//! centered on HTTP byte-range requests. For each file contained in the backing
//! resource, the index file merely contains a byte offset and length that are
//! then used to construct an HTTP Range request to obtain the file as needed.

use flate2::read::GzDecoder;
use std::{convert::TryInto, io::Read, str::FromStr};
use tectonic_errors::prelude::*;
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend, RangeReader};
use tectonic_io_base::digest::{self, DigestData};
use tectonic_status_base::{tt_note, tt_warning, StatusBackend};

use crate::cache::{BackendPullData, CacheBackend};

const MAX_HTTP_ATTEMPTS: usize = 4;

/// The internal file-information struct used by the [`IndexedTarBackend`].
#[derive(Clone, Copy, Debug)]
pub struct FileInfo {
    offset: u64,
    length: u64,
}

/// A simple web-based file backend based on HTTP Range requests.
///
/// This type implements the [`CacheBackend`] trait and so can be used for
/// web-based bundle access thorugh the [`crate::cache::CachingBundle`]
/// framework.
#[derive(Debug)]
pub struct IndexedTarBackend {
    reader: DefaultRangeReader,
}

impl CacheBackend for IndexedTarBackend {
    type FileInfo = FileInfo;

    fn open_with_pull(
        start_url: &str,
        status: &mut dyn StatusBackend,
    ) -> Result<(Self, BackendPullData)> {
        // Step 1: resolve URL
        let mut geturl_backend = DefaultBackend::default();
        let resolved_url = geturl_backend.resolve_url(start_url, status)?;

        // Step 2: fetch index
        let index = {
            let mut index = String::new();
            let index_url = format!("{}.index.gz", &resolved_url);
            tt_note!(status, "downloading index {}", index_url);
            GzDecoder::new(geturl_backend.get_url(&index_url, status)?)
                .read_to_string(&mut index)?;
            index
        };

        // Step 3: get digest, setting up instance as we go

        let mut cache_backend = IndexedTarBackend {
            reader: geturl_backend.open_range_reader(&resolved_url),
        };

        let digest_info = {
            let mut digest_info = None;

            for line in index.lines() {
                if let Ok((name, info)) = Self::parse_index_line(line) {
                    if name == digest::DIGEST_NAME {
                        digest_info = Some(info);
                        break;
                    }
                }
            }

            atry!(
                digest_info;
                ["backend does not provide needed {} file", digest::DIGEST_NAME]
            )
        };

        let digest_text =
            String::from_utf8(cache_backend.get_file(digest::DIGEST_NAME, &digest_info, status)?)
                .map_err(|e| e.utf8_error())?;
        let digest = DigestData::from_str(&digest_text)?;

        // All done.
        Ok((
            cache_backend,
            BackendPullData {
                resolved_url,
                digest,
                index,
            },
        ))
    }

    fn open_with_quick_check(
        resolved_url: &str,
        digest_file_info: &Self::FileInfo,
        status: &mut dyn StatusBackend,
    ) -> Result<Option<(Self, DigestData)>> {
        let mut cache_backend = IndexedTarBackend {
            reader: DefaultBackend::default().open_range_reader(resolved_url),
        };

        if let Ok(d) = cache_backend.get_file(digest::DIGEST_NAME, digest_file_info, status) {
            if let Ok(d) = String::from_utf8(d) {
                if let Ok(d) = DigestData::from_str(&d) {
                    return Ok(Some((cache_backend, d)));
                }
            }
        }

        Ok(None)
    }

    fn parse_index_line(line: &str) -> Result<(String, Self::FileInfo)> {
        let mut bits = line.split_whitespace();

        if let (Some(name), Some(offset), Some(length)) = (bits.next(), bits.next(), bits.next()) {
            Ok((
                name.to_owned(),
                FileInfo {
                    offset: offset.parse::<u64>()?,
                    length: length.parse::<u64>()?,
                },
            ))
        } else {
            // TODO: preserve the warning info or something!
            bail!("malformed index line");
        }
    }

    fn get_file(
        &mut self,
        name: &str,
        info: &Self::FileInfo,
        status: &mut dyn StatusBackend,
    ) -> Result<Vec<u8>> {
        tt_note!(status, "downloading {}", name);

        // Historically, sometimes our web service would drop connections when
        // fetching a bunch of resource files (i.e., on the first invocation).
        // The error manifested itself in a way that has a not-so-nice user
        // experience. Our solution: retry the request a few times in case it
        // was a transient problem.

        let n = info.length.try_into().unwrap();
        let mut buf = Vec::with_capacity(n);
        let mut overall_failed = true;
        let mut any_failed = false;

        for _ in 0..MAX_HTTP_ATTEMPTS {
            let mut stream = match self.reader.read_range(info.offset, n) {
                Ok(r) => r,
                Err(e) => {
                    tt_warning!(status, "failure requesting \"{}\" from network", name; e);
                    any_failed = true;
                    continue;
                }
            };

            if let Err(e) = stream.read_to_end(&mut buf) {
                tt_warning!(status, "failure downloading \"{}\" from network", name; e.into());
                any_failed = true;
                continue;
            }

            overall_failed = false;
            break;
        }

        if overall_failed {
            bail!(
                "failed to retrieve \"{}\" from the network; \
             this most probably is not Tectonic's fault \
             -- please check your network connection.",
                name
            );
        } else if any_failed {
            tt_note!(status, "download succeeded after retry");
        }

        Ok(buf)
    }
}
