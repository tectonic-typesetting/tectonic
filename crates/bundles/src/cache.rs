// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Local caching of bundle data.
//!
//! This module implements Tectonic’s local filesystem caching mechanism for TeX
//! support files. To make a cachable bundle, wrap any [`CachableBundle`] with a
//! [`BundleCache`].

use crate::{Bundle, CachableBundle, FileIndex, FileInfo};
use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{
    app_dirs,
    digest::{self, DigestData},
    InputHandle, InputOrigin, IoProvider, OpenResult,
};
use tectonic_status_base::StatusBackend;

/// A convenience method to provide a better error message when writing to a created file.
fn file_create_write<P, F, E>(path: P, write_fn: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut File) -> std::result::Result<(), E>,
    E: std::error::Error + 'static + Sync + Send,
{
    let path = path.as_ref();
    let mut f = atry!(
        File::create(path);
        ["couldn't open {} for writing", path.display()]
    );
    atry!(
        write_fn(&mut f);
        ["couldn't write to {}", path.display()]
    );
    Ok(())
}

// Make sure a directory exists.
// "inline" version is for convenience.
macro_rules! ensure_dir {
    (inline, $path:expr) => {
        {
            atry!(
                fs::create_dir_all(&$path);
                ["failed to create directory `{}` or one of its parents", $path.display()]
            );
            $path
        }
    };

    ($path:expr) => {
        atry!(
            fs::create_dir_all(&$path);
            ["failed to create directory `{}` or one of its parents", $path.display()]
        );
    };
}

/// A cache wrapper for another bundle.
///
/// This bundle implementation is the key to Tectonic’s ability to download TeX
/// support files on the fly. This is usually used to wrap some kind of network-
/// based bundle, but can be used with any struct that implements [`Bundle`].
///
/// The caching scheme here is designed so that a document build may avoid
/// touching the network altogether if no new files need to be downloaded.
pub struct BundleCache<'this, F, T> {
    /// If true, only use cached files -- never connect to the backend.
    ///
    /// This option can be useful if we are operating disconnected from the
    /// network (e.g., on an airplane). If you add a new figure to your
    /// document, the engine will inquire about several related files that it
    /// thinks might exist. Without this option, such an inquiry might require
    /// Tectonic to hit the network, when the user knows for sure that the
    /// bundle is not going to contain these files.
    only_cached: bool,

    /// The connection to the cache backend, maybe.
    ///
    /// This field will be `None` if there are locally cached data present and
    /// there has not yet been a need to connect to the backend. If it becomes
    /// necessary to "pull" and/or download a new file from the backend, this
    /// value will become `Some` — it represents something like an open network
    /// connection.
    bundle: Box<dyn CachableBundle<'this, F, T>>,

    /// The root directory of this cache.
    /// All other paths are subdirectories of this path.
    cache_root: PathBuf,

    // The hash of the bundle we're caching.
    bundle_hash: DigestData,
}

impl<'this, F: FileInfo + 'this, T: FileIndex<'this, F>> BundleCache<'this, F, T> {
    /// Make a new filesystem-backed cache from `bundle`.
    pub fn new(
        mut bundle: Box<dyn CachableBundle<'this, F, T>>,
        only_cached: bool,
        cache_root: Option<PathBuf>,
    ) -> Result<Self> {
        // If cache_root is none, use default location.
        let cache_root = match cache_root {
            None => app_dirs::get_user_cache_dir("bundles")?,
            Some(p) => ensure_dir!(inline, p),
        };

        let hash_dir = ensure_dir!(inline, &cache_root.join("hashes"));
        let hash_file = hash_dir.join(app_dirs::app_dirs2::sanitized(&bundle.get_location()));

        let saved_hash = {
            if !hash_file.exists() {
                None
            } else {
                match File::open(&hash_file) {
                    Err(e) => return Err(e.into()),
                    Ok(f) => {
                        let mut digest_text = String::with_capacity(digest::DIGEST_LEN);
                        f.take(digest::DIGEST_LEN as u64)
                            .read_to_string(&mut digest_text)?;
                        Some(DigestData::from_str(&digest_text)?)
                    }
                }
            }
        };

        let live_hash = bundle.get_digest().ok();

        // Check remote bundle digest
        let bundle_hash: DigestData = match (saved_hash, live_hash) {
            (None, None) => {
                bail!("this bundle isn't cached, and we couldn't get it from the internet.");
            }
            (Some(s), Some(l)) => {
                if s != l {
                    //tt_warning!(status "Bundle hash changed, updating cache...");
                    file_create_write(&hash_file, |f| writeln!(f, "{}", &l.to_string()))?;
                    l
                } else {
                    l
                }
            }
            (None, Some(l)) => {
                file_create_write(&hash_file, |f| writeln!(f, "{}", &l.to_string()))?;
                l
            }
            (Some(h), None) => h, // Bundle is offline, but we're ok.
        };

        let bundle = BundleCache {
            only_cached,
            bundle,
            cache_root,
            bundle_hash,
        };

        ensure_dir!(&bundle
            .cache_root
            .join(&format!("data/{}", bundle.bundle_hash.to_string())));

        return Ok(bundle);
    }

    // Build path for a bundle file
    fn get_file_path(&self, info: &F) -> PathBuf {
        return self
            .cache_root
            .join(&format!("data/{}", self.bundle_hash.to_string()))
            .join(&info.path()[1..]);
    }

    fn ensure_index(&mut self) -> Result<()> {
        if self.bundle.index().is_initialized() {
            return Ok(());
        }

        let target = self
            .cache_root
            .join(format!("{}.index", self.bundle_hash.to_string()));

        if target.exists() {
            let mut file = File::open(target)?;
            self.bundle.initialize_index(&mut file)?;
            return Ok(());
        }

        let mut reader = self.bundle.get_index_reader()?;
        let mut file = File::create(&target)?;
        io::copy(&mut reader, &mut file)?;
        drop(file);

        let mut file = File::open(&target)?;
        self.bundle.initialize_index(&mut file)?;

        return Ok(());
    }

    /// Get a FileInfo from a name.
    /// This returns (in_cache, info), where in_cache is true
    /// if this file is already in our cache and can be retrieved
    /// without touching the backing bundle.
    fn get_fileinfo(&mut self, name: &str) -> OpenResult<(bool, F)> {
        match self.ensure_index() {
            Ok(_) => {}
            Err(e) => return OpenResult::Err(e),
        };

        let info = match self.bundle.search(name) {
            Some(i) => i,
            None => return OpenResult::NotAvailable,
        };

        let target = self.get_file_path(&info);
        return OpenResult::Ok((target.exists(), info));
    }

    /// Fetch a file from the bundle backing this cache.
    /// Returns a path to the file that was created.
    fn fetch_file(&mut self, info: F, status: &mut dyn StatusBackend) -> OpenResult<PathBuf> {
        let target = self.get_file_path(&info);
        fs::create_dir_all(&target.parent().unwrap()).unwrap();

        // Already in the cache?
        if target.exists() {
            return OpenResult::Ok(target);
        }

        // No, it's not. Are we in cache-only mode?
        if self.only_cached {
            return OpenResult::NotAvailable;
        }

        // Get the file.
        let mut handle = match self.bundle.open_fileinfo(&info, status) {
            OpenResult::Ok(c) => c,
            OpenResult::Err(e) => return OpenResult::Err(e),
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
        };

        if let Err(e) = file_create_write(&target, |f| io::copy(&mut handle, f).map(|_| ())) {
            return OpenResult::Err(e);
        }

        OpenResult::Ok(target)
    }
}

impl<'this, F: FileInfo + 'this, T: FileIndex<'this, F>> IoProvider for BundleCache<'this, F, T> {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let path = match self.get_fileinfo(name) {
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
            OpenResult::Ok((true, f)) => self.get_file_path(&f),
            OpenResult::Ok((false, f)) => match self.fetch_file(f, status) {
                OpenResult::Ok(p) => p,
                OpenResult::NotAvailable => return OpenResult::NotAvailable,
                OpenResult::Err(e) => return OpenResult::Err(e),
            },
        };

        let f = match File::open(path) {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into()),
        };

        OpenResult::Ok(InputHandle::new_read_only(
            name,
            BufReader::new(f),
            InputOrigin::Other,
        ))
    }
}

impl<'this, F: FileInfo + 'this, T: FileIndex<'this, F>> Bundle for BundleCache<'this, F, T> {
    fn get_digest(&mut self) -> Result<DigestData> {
        Ok(self.bundle_hash)
    }

    // Returns a list of all files that are *available*.
    // If we're online, this is the list of all files in the bundle.
    // If we're offline, this is every file in the cache.
    //
    // TODO: Maybe we want different errors for "offline" and "actually doesn't exist"?
    fn all_files(&mut self) -> Result<Vec<String>> {
        return self.bundle.all_files();
    }
}
