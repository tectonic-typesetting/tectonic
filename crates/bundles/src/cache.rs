// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Local caching of bundle data.
//!
//! This module implements Tectonic’s local filesystem caching mechanism for TeX
//! support files. To make a cachable bundle, wrap any [`CachableBundle`] with a
//! [`BundleCache`].

use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{
    app_dirs,
    digest::{self, Digest, DigestData},
    InputHandle, InputOrigin, IoProvider, OpenResult,
};
use tectonic_status_base::StatusBackend;

use crate::{Bundle, CachableBundle, FileIndex, FileInfo};

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

/// Ensure that a directory exists.
fn ensure_cache_dir(root: &Path, path: &str) -> Result<PathBuf> {
    let full_path = root.join(path);
    atry!(
        fs::create_dir_all(&full_path);
        ["failed to create directory `{}` or one of its parents", full_path.display()]
    );
    Ok(full_path)
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

    // Absolute path, child of cache_root.
    // Where this we store files from this bundle.
    data_path: PathBuf,

    // The hash of the bundle we're caching.
    bundle_hash: DigestData,
}

impl<'this, F: FileInfo + 'this, T: FileIndex<'this, F>> BundleCache<'this, F, T> {
    /// Make a new filesystem-backed cache from `bundle`.
    pub fn new(
        mut bundle: Box<dyn CachableBundle<'this, F, T>>,
        only_cached: bool,
        status: &mut dyn StatusBackend,
        cache_root: Option<PathBuf>,
    ) -> Result<Self> {
        // If cache_root is none, use default location.
        let cache_root = match cache_root {
            None => app_dirs::get_user_cache_dir("bundles")?,
            Some(p) => p,
        };

        let hash_file = ensure_cache_dir(&cache_root, "hashes")?
            .join(app_dirs::app_dirs2::sanitized(&bundle.get_location()));

        let saved_hash = {
            match File::open(&hash_file) {
                Err(_) => None, // TODO: error on actual errors
                Ok(f) => {
                    let mut digest_text = String::with_capacity(digest::DIGEST_LEN);
                    f.take(digest::DIGEST_LEN as u64)
                        .read_to_string(&mut digest_text)?;
                    Some(DigestData::from_str(&digest_text)?)
                }
            }
        };

        let live_hash = bundle.get_digest(status).ok();

        // Check remote bundle digest
        let bundle_hash: DigestData = match (saved_hash, live_hash) {
            (None, None) => {
                bail!("Couldn't get bundle");
            }
            (Some(h), Some(l)) => {
                if h != l {
                    bail!("Bundle changed!")
                } else {
                    h
                }
            }
            (None, Some(l)) => {
                file_create_write(&hash_file, |f| writeln!(f, "{}", &l.to_string()))?;
                l
            }
            (Some(h), None) => h, // No internet connection, but we're ok.
        };

        let data_path =
            ensure_cache_dir(&cache_root, &format!("data/{}", bundle_hash.to_string()))?;

        Ok(BundleCache {
            only_cached,
            bundle,
            cache_root,
            bundle_hash,
            data_path,
        })
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

    /// Make sure that a file is available, and return its filesystem path.
    ///
    /// If the file is already cached, just pull it out. Otherwise, fetch it
    /// from the backend.
    fn ensure_file_availability(&mut self, name: &str) -> OpenResult<PathBuf> {
        match self.ensure_index() {
            Ok(_) => {}
            Err(e) => return OpenResult::Err(e),
        };

        let info = match self.bundle.search(name) {
            Some(i) => i,
            None => return OpenResult::NotAvailable,
        };
        let target = self.data_path.join(&info.path()[1..]);
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
        let mut content = match self.bundle.open_fileinfo(&info) {
            OpenResult::Ok(c) => c,
            OpenResult::Err(e) => return OpenResult::Err(e),
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
        };
        let mut buf: Vec<u8> = Vec::new();
        if let Err(e) = content.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        };
        //let length = buf.len();

        let mut digest_builder = digest::create();
        digest_builder.update(&buf);

        // Perform a racy check for the destination existing, because this
        // matters on Windows: if the destination is already there, we'll get
        // an error because the destination is marked read-only. Assuming
        // non-pathological filesystem manipulation, though, we'll only be
        // subject to the race once.

        if !target.exists() {
            if let Err(e) = file_create_write(&target, |f| f.write_all(&buf)) {
                return OpenResult::Err(e);
            }

            // Now we can make the file readonly. It would be nice to set the
            // permissions using the already-open file handle owned by the
            // tempfile, but mkstemp doesn't give us access.
            let mut perms = match fs::metadata(&target) {
                Ok(p) => p,
                Err(e) => {
                    return OpenResult::Err(e.into());
                }
            }
            .permissions();
            perms.set_readonly(true);

            if let Err(e) = fs::set_permissions(&target, perms) {
                return OpenResult::Err(e.into());
            }
        }

        // And finally add a record of this file to our manifest. Note that
        // we're opening and closing the manifest every time we cache a new
        // file; not so efficient, but whatever.

        //if let Err(e) = self.save_to_manifest(name, length as u64, digest) {
        //    return OpenResult::Err(e);
        //}

        OpenResult::Ok(target)
    }
}

impl<'this, F: FileInfo + 'this, T: FileIndex<'this, F>> IoProvider for BundleCache<'this, F, T> {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let path = match self.ensure_file_availability(name) {
            OpenResult::Ok(p) => p,
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
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
    fn get_digest(&mut self, _status: &mut dyn StatusBackend) -> Result<DigestData> {
        Ok(self.bundle_hash)
    }

    // Returns a list of all files that are *available*.
    // If we're online, this is the list of all files in the bundle.
    // If we're offline, this is every file in the cache.
    //
    // TODO: Maybe we want different errors for "offline" and "actually doesn't exist"?
    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        return self.bundle.all_files(status);
    }
}
