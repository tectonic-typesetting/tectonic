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
    process,
    str::FromStr,
};
use tectonic_errors::{anyhow::Context, prelude::*};
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
pub struct BundleCache<'this, T> {
    /// If true, only use cached files -- never connect to the backend.
    ///
    /// This option can be useful if we are operating disconnected from the
    /// network (e.g., on an airplane). If you add a new figure to your
    /// document, the engine will inquire about several related files that it
    /// thinks might exist. Without this option, such an inquiry might require
    /// Tectonic to hit the network, when the user knows for sure that the
    /// bundle is not going to contain these files.
    only_cached: bool,

    /// The bundle we're wrapping. When files don't exist in the cache,
    /// we'll get them from here.
    bundle: Box<dyn CachableBundle<'this, T>>,

    /// The root directory of this cache.
    /// All other paths are subdirectories of this path.
    cache_root: PathBuf,

    // The hash of the bundle we're caching.
    bundle_hash: DigestData,
}

impl<'this, T: FileIndex<'this>> BundleCache<'this, T> {
    /// Make a new filesystem-backed cache from `bundle`.
    ///
    /// This method will fail if we can't connect to the bundle AND
    /// we don't already have it in our cache.
    /// Other than that, this method does not require network access.
    pub fn new(
        mut bundle: Box<dyn CachableBundle<'this, T>>,
        only_cached: bool,
        cache_root: Option<PathBuf>,
    ) -> Result<Self> {
        // If cache_root is none, use default location.
        let cache_root = match cache_root {
            None => app_dirs::get_user_cache_dir("bundles").context("while making cache root")?,
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
                            .read_to_string(&mut digest_text)
                            .with_context(|| {
                                format!("while reading hash from {hash_file:?} in cache")
                            })?;
                        Some(
                            DigestData::from_str(&digest_text)
                                .with_context(|| format!("while parsing hash `{digest_text}`"))?,
                        )
                    }
                }
            }
        };

        let live_hash = bundle.get_digest();

        // Check remote bundle digest
        let bundle_hash: DigestData = match (saved_hash, live_hash) {
            (None, Err(e)) => {
                bail!("this bundle isn't cached, and we couldn't get it from the internet. Error: {e}");
            }
            (Some(s), Ok(l)) => {
                if s != l {
                    // Silently update hash in cache.
                    // We don't need to delete anything, since data is indexed by hash.
                    // TODO: show a warning
                    file_create_write(&hash_file, |f| writeln!(f, "{}", &l.to_string()))
                        .with_context(|| {
                            format!("while updating bundle hash in {hash_file:?} in cache")
                        })?;
                    l
                } else {
                    l
                }
            }
            (None, Ok(l)) => {
                file_create_write(&hash_file, |f| writeln!(f, "{}", &l.to_string())).with_context(
                    || format!("while writing bundle hash to {hash_file:?} in cache"),
                )?;
                l
            }
            (Some(h), Err(_)) => h, // Bundle is offline, but we're ok.
        };

        let bundle = BundleCache {
            only_cached,
            bundle,
            cache_root,
            bundle_hash,
        };

        // Right now, files are stored in
        // `<root>/data/<bundle hash>/<file path>.
        // This works for now, but may cause issues if we add multiple
        // bundle formats with incompatible path schemes. We assume that
        // all bundles with the same hash use the same path scheme,
        // which is true for network TTB and fs TTB.
        // Adding support for multiple formats of a single bundle hash
        // shouldn't be too hard, but isn't necessary yet.
        ensure_dir!(&bundle
            .cache_root
            .join(format!("data/{}", bundle.bundle_hash)));

        Ok(bundle)
    }

    /// Build a cache path for the given bundle file
    fn get_file_path(&self, info: &T::InfoType) -> PathBuf {
        let mut out = self.cache_root.clone();
        out.push(format!("data/{}", self.bundle_hash));
        out.push(info.path());
        out
    }

    /// Build a temporary path for the given bundle file
    /// To ensure safety with multiple instances of tectonic,
    /// files are first downloaded to a known-unique location, then renamed.
    fn get_file_path_tmp(&self, info: &T::InfoType) -> PathBuf {
        let mut out = self.cache_root.clone();
        out.push(format!("data/{}", self.bundle_hash));
        out.push(format!("{}-tmp-pid{}", info.path(), process::id()));
        out
    }

    fn ensure_index(&mut self) -> Result<()> {
        let target = self
            .cache_root
            .join(format!("data/{}.index", self.bundle_hash));

        // We check for two things here:
        // - that the bundle index is initialized
        // - that the bundle index is cached.
        //
        // It would be nice to assume that the bundle index is never initialized
        // before this function is called, but we can't do that. Unlike ttb,
        // itar bundles cannot retrieve the bundle hash without loading the index.
        if target.exists() {
            if self.bundle.index().is_initialized() {
                return Ok(());
            }

            // Initialize bundle index using cached file
            let mut file = File::open(&target)
                .with_context(|| format!("while opening index {target:?} in cache"))?;
            self.bundle
                .initialize_index(&mut file)
                .with_context(|| format!("while inititalizing index using cached {target:?}"))?;
        } else {
            // Download index

            // We first download to a temporary file, rename to target
            // Makes sure that parallel runs of tectonic don't break the index
            let tmp_target = self.cache_root.join(format!(
                "data/{}.index-tmp-pid{}",
                self.bundle_hash,
                process::id()
            ));

            let mut reader = self
                .bundle
                .get_index_reader()
                .context("while getting index reader")?;
            let mut file = File::create(&tmp_target)
                .with_context(|| format!("while creating index {tmp_target:?} in cache"))?;
            io::copy(&mut reader, &mut file)
                .with_context(|| format!("while writing index {tmp_target:?} in cache"))?;
            drop(file);

            fs::rename(&tmp_target, &target).with_context(|| {
                format!("while renaming index {tmp_target:?} to {target:?} in cache")
            })?;

            if self.bundle.index().is_initialized() {
                return Ok(());
            }

            let mut file = File::open(&target)
                .with_context(|| format!("while opening index from {target:?} in cache"))?;
            self.bundle
                .initialize_index(&mut file)
                .with_context(|| format!("while initializing index {target:?} in cache"))?;
        }

        Ok(())
    }

    /// Get a FileInfo from a name.
    /// This returns (in_cache, info), where in_cache is true
    /// if this file is already in our cache and can be retrieved
    /// without touching the backing bundle.
    fn get_fileinfo(&mut self, name: &str) -> OpenResult<(bool, T::InfoType)> {
        if let Err(e) = self.ensure_index() {
            return OpenResult::Err(e);
        };

        let info = match self.bundle.search(name) {
            Some(i) => i,
            None => return OpenResult::NotAvailable,
        };

        let target = self.get_file_path(&info);
        OpenResult::Ok((target.exists(), info))
    }

    /// Fetch a file from the bundle backing this cache.
    /// Returns a path to the file that was created.
    fn fetch_file(
        &mut self,
        info: T::InfoType,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<PathBuf> {
        let target = self.get_file_path(&info);
        match fs::create_dir_all(target.parent().unwrap()) {
            Ok(()) => {}
            Err(e) => return OpenResult::Err(e.into()),
        };

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

        // Download to a known-unique temporary location, then move.
        // This prevents issues when running multiple processes.
        let tmp_path = self.get_file_path_tmp(&info);
        if let Err(e) = file_create_write(&tmp_path, |f| io::copy(&mut handle, f).map(|_| ())) {
            return OpenResult::Err(e);
        }
        if let Err(e) = fs::rename(&tmp_path, &target) {
            return OpenResult::Err(e.into());
        };

        OpenResult::Ok(target)
    }
}

impl<'this, T: FileIndex<'this>> IoProvider for BundleCache<'this, T> {
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

impl<'this, T: FileIndex<'this>> Bundle for BundleCache<'this, T> {
    fn get_digest(&mut self) -> Result<DigestData> {
        Ok(self.bundle_hash)
    }

    fn all_files(&self) -> Vec<String> {
        self.bundle.all_files()
    }
}
