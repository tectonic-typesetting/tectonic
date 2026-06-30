// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! The web-friendly "indexed tar" bundle backend.
//!
//! The main type offered by this module is the [`ItarBundle`] struct,
//! which can (but should not) be used directly as any other bundle.
//!
//! Instead, wrap it in a [`crate::BundleCache`] for filesystem-backed
//! caching.
//!
//! While the on-server file format backing the "indexed tar" backend is indeed
//! a standard `tar` file, as far as the client is concerned, this backend is
//! centered on HTTP byte-range requests. For each file contained in the backing
//! resource, the index file merely contains a byte offset and length that are
//! then used to construct an HTTP Range request to obtain the file as needed.

use crate::{Bundle, CachableBundle, FileIndex, FileInfo, NET_RETRY_ATTEMPTS, NET_RETRY_SLEEP_MS};
use flate2::read::GzDecoder;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Cursor, Read},
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};
use tectonic_errors::prelude::*;
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend, RangeReader};
use tectonic_io_base::{digest, InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::{tt_note, tt_warning, NoopStatusBackend, StatusBackend};

/// Default number of concurrent connections used to prefetch files.
const DEFAULT_PREFETCH_CONCURRENCY: usize = 16;

/// Hard upper bound on prefetch concurrency, applied even when the user sets
/// `TECTONIC_PREFETCH_CONCURRENCY` explicitly. This keeps a large manifest or a
/// mistakenly huge env value from spawning an unreasonable number of worker
/// threads and simultaneous connections.
const MAX_PREFETCH_CONCURRENCY: usize = 64;

/// Read a single file's bytes through `reader`, retrying on transient failures.
///
/// This is the shared core used by both the one-at-a-time [`ItarBundle::open_fileinfo`]
/// path and the concurrent [`ItarBundle::batch_open`] path.
fn read_file_with_retries(
    reader: &mut DefaultRangeReader,
    info: &ItarFileInfo,
    status: &mut dyn StatusBackend,
) -> OpenResult<Vec<u8>> {
    // Edge case for zero-sized reads (these cause errors on some web hosts).
    if info.length == 0 {
        return OpenResult::Ok(Vec::new());
    }

    for i in 0..NET_RETRY_ATTEMPTS {
        let mut stream = match reader.read_range(info.offset, info.length) {
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

        let mut v = Vec::with_capacity(info.length);
        match stream.read_to_end(&mut v) {
            Ok(_) => return OpenResult::Ok(v),
            Err(e) => {
                tt_warning!(status,
                    "failure downloading \"{}\" from network ({}/{NET_RETRY_ATTEMPTS})",
                    info.name, i+1; e.into()
                );
                thread::sleep(Duration::from_millis(NET_RETRY_SLEEP_MS));
                continue;
            }
        };
    }

    OpenResult::Err(anyhow!(
        "failed to download \"{}\"; please check your network connection.",
        info.name
    ))
}

/// The internal file-information struct used by the [`ItarBundle`].
#[derive(Clone, Debug)]
pub struct ItarFileInfo {
    name: String,
    offset: u64,
    length: usize,
}

impl FileInfo for ItarFileInfo {
    fn name(&self) -> &str {
        &self.name
    }
    fn path(&self) -> &str {
        &self.name
    }
}

/// A simple FileIndex for compatiblity with [`crate::BundleCache`]
#[derive(Default, Debug)]
pub struct ItarFileIndex {
    content: HashMap<String, ItarFileInfo>,
}

impl<'this> FileIndex<'this> for ItarFileIndex {
    type InfoType = ItarFileInfo;

    fn iter(&'this self) -> Box<dyn Iterator<Item = &'this ItarFileInfo> + 'this> {
        Box::new(self.content.values())
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn initialize(&mut self, reader: &mut dyn Read) -> Result<()> {
        self.content.clear();

        for line in BufReader::new(reader).lines() {
            let line = line?;
            let mut bits = line.split_whitespace();

            if let (Some(name), Some(offset), Some(length)) =
                (bits.next(), bits.next(), bits.next())
            {
                self.content.insert(
                    name.to_owned(),
                    ItarFileInfo {
                        name: name.to_owned(),
                        offset: offset.parse::<u64>()?,
                        length: length.parse::<usize>()?,
                    },
                );
            } else {
                // TODO: preserve the warning info or something!
                bail!("malformed index line");
            }
        }
        Ok(())
    }

    /// Find a file in this index
    fn search(&'this mut self, name: &str) -> Option<ItarFileInfo> {
        self.content.get(name).cloned()
    }
}

/// The old-fashoned Tectonic web bundle format.
pub struct ItarBundle {
    url: String,
    /// Maps all available file names to [`FileInfo`]s.
    /// This is empty after we create this bundle, so we don't need network
    /// to make an object. It is automatically filled by get_index when we need it.
    index: ItarFileIndex,

    /// RangeReader object, responsible for sending queries.
    /// Will be None when the object is created, automatically
    /// replaced with Some(...) once needed.
    reader: Option<DefaultRangeReader>,
}

impl ItarBundle {
    /// Make a new ItarBundle.
    /// This method does not require network access.
    /// It will succeed even in we can't connect to the bundle, or if we're given a bad url.
    pub fn new(url: String) -> Result<ItarBundle> {
        Ok(ItarBundle {
            index: ItarFileIndex::default(),
            reader: None,
            url,
        })
    }

    fn connect_reader(&mut self) {
        let geturl_backend = DefaultBackend::default();
        // Connect reader if it is not already connected
        if self.reader.is_none() {
            self.reader = Some(geturl_backend.open_range_reader(&self.url));
        }
    }

    /// Fill this bundle's index, if it is empty.
    fn ensure_index(&mut self) -> Result<()> {
        // Always make sure we have a range reader for fetching file contents.
        // The index may have been initialized from the on-disk cache (via
        // `initialize_index`, called by `BundleCache`) rather than downloaded
        // here. In that case the early return below would leave `self.reader`
        // as `None`, and the on-demand fetch path (`open_fileinfo`) would panic
        // unwrapping it. `connect_reader` is idempotent and does no network I/O
        // until a range is actually read, so this is cheap.
        self.connect_reader();

        // Fetch index if it is empty
        if self.index.is_initialized() {
            return Ok(());
        }

        let mut reader = self.get_index_reader()?;
        self.index.initialize(&mut reader)?;

        Ok(())
    }
}

impl IoProvider for ItarBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        if let Err(e) = self.ensure_index() {
            return OpenResult::Err(e);
        };

        let info = match self.index.search(name) {
            Some(a) => a,
            None => return OpenResult::NotAvailable,
        };

        // Retries are handled in open_fileinfo,
        // since BundleCache never calls input_open_name.
        self.open_fileinfo(&info, status)
    }
}

impl Bundle for ItarBundle {
    fn all_files(&self) -> Vec<String> {
        self.index.iter().map(|x| x.path().to_owned()).collect()
    }

    fn get_digest(&mut self) -> Result<tectonic_io_base::digest::DigestData> {
        let digest_text = match self.input_open_name(digest::DIGEST_NAME, &mut NoopStatusBackend {})
        {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                h.take(64).read_to_string(&mut text)?;
                text
            }

            OpenResult::NotAvailable => {
                // Broken or un-cacheable backend.
                bail!("bundle does not provide needed SHA256SUM file");
            }

            OpenResult::Err(e) => {
                return Err(e);
            }
        };

        Ok(atry!(digest::DigestData::from_str(&digest_text); ["corrupted SHA256 digest data"]))
    }
}

impl CachableBundle<'_, ItarFileIndex> for ItarBundle {
    fn get_location(&mut self) -> String {
        self.url.clone()
    }

    fn initialize_index(&mut self, source: &mut dyn Read) -> Result<()> {
        self.index.initialize(source)?;
        Ok(())
    }

    fn index(&mut self) -> &mut ItarFileIndex {
        &mut self.index
    }

    fn search(&mut self, name: &str) -> Option<ItarFileInfo> {
        self.index.search(name)
    }

    fn get_index_reader(&mut self) -> Result<Box<dyn Read>> {
        let mut geturl_backend = DefaultBackend::default();
        let index_url = format!("{}.index.gz", &self.url);
        let reader = GzDecoder::new(geturl_backend.get_url(&index_url)?);
        Ok(Box::new(reader))
    }

    fn open_fileinfo(
        &mut self,
        info: &ItarFileInfo,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        match self.ensure_index() {
            Ok(_) => {}
            Err(e) => return OpenResult::Err(e),
        };

        tt_note!(status, "downloading {}", info.name);

        match read_file_with_retries(self.reader.as_mut().unwrap(), info, status) {
            OpenResult::Ok(v) => OpenResult::Ok(InputHandle::new_read_only(
                info.name.to_owned(),
                Cursor::new(v),
                InputOrigin::Other,
            )),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
            OpenResult::Err(e) => OpenResult::Err(e),
        }
    }

    /// Download many files concurrently.
    ///
    /// Each file is an independent HTTP byte-range request, so on a cold cache
    /// the dominant cost is round-trip latency multiplied by the number of
    /// files. Issuing them one-at-a-time (as the engine does on demand) is
    /// therefore badly latency-bound. Here we fan the requests out across a pool
    /// of worker threads, each with its own range reader (and thus its own
    /// pooled HTTP connection), which collapses N serial round-trips into
    /// roughly N/concurrency.
    fn batch_open(
        &mut self,
        infos: &[ItarFileInfo],
        status: &mut dyn StatusBackend,
    ) -> Vec<OpenResult<Vec<u8>>> {
        if let Err(e) = self.ensure_index() {
            return infos
                .iter()
                .map(|_| OpenResult::Err(anyhow!("failed to load bundle index: {e}")))
                .collect();
        }

        let concurrency = std::env::var("TECTONIC_PREFETCH_CONCURRENCY")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|n| *n > 0)
            .unwrap_or(DEFAULT_PREFETCH_CONCURRENCY)
            .min(MAX_PREFETCH_CONCURRENCY)
            .min(infos.len().max(1));

        tt_note!(
            status,
            "prefetching {} files ({}-way concurrent)",
            infos.len(),
            concurrency
        );

        let url = self.url.clone();
        let next = AtomicUsize::new(0);
        // One result slot per input, written exactly once by the worker that
        // claims that index. `NoopStatusBackend` is used inside workers because
        // `StatusBackend` is not `Sync`; transient retry warnings are dropped.
        let result_slots: Vec<std::sync::Mutex<OpenResult<Vec<u8>>>> = (0..infos.len())
            .map(|_| std::sync::Mutex::new(OpenResult::NotAvailable))
            .collect();

        thread::scope(|scope| {
            let next = &next;
            let result_slots = &result_slots;
            let url = &url;
            for _ in 0..concurrency {
                scope.spawn(move || {
                    let mut reader = DefaultBackend::default().open_range_reader(url);
                    let mut noop = NoopStatusBackend {};
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= infos.len() {
                            break;
                        }
                        let res = read_file_with_retries(&mut reader, &infos[i], &mut noop);
                        *result_slots[i].lock().unwrap() = res;
                    }
                });
            }
        });

        result_slots
            .into_iter()
            .map(|m| m.into_inner().unwrap())
            .collect()
    }
}
