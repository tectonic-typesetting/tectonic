// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Local caching of bundle data.
//!
//! This module implements Tectonic’s local filesystem caching mechanism for TeX
//! support files. To enable efficient caching with proper invalidation
//! semantics, the caching layer does *not* merely wrap [`IoProvider`]
//! implementations. Instead, a cacheable bundle must implement the
//! [`CacheBackend`] trait defined in this module. An example of such a bundle
//! is the [`crate::itar::IndexedTarBundle`] for bundles served over HTTP.
//!
//! In order to access a cacheable bundle, you need a handle to a local
//! [`Cache`], probably obtained with [`Cache::get_user_default()`], and a URL,
//! which you’ll pass to [`Cache::open()`]. When using this function, you must
//! explicitly specify the concrete [`CacheBackend`] type that will service
//! backend requests.

use fs2::FileExt;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{
    app_dirs,
    digest::{self, Digest, DigestData},
    try_open_file, InputHandle, InputOrigin, IoProvider, OpenResult,
};
use tectonic_status_base::{tt_note, tt_warning, StatusBackend};

use crate::Bundle;

/// A cache of data from one or more bundles using the local filesystem.
#[derive(Debug)]
pub struct Cache {
    root: PathBuf,
}

impl Cache {
    /// Get a handle to a bundle cache, using default per-user settings.
    ///
    /// The cache location defaults to the `AppDataType::UserCache`
    /// provided by `app_dirs2` but can be overwritten using the
    /// `TECTONIC_CACHE_DIR` environment variable.
    ///
    /// This method may perform I/O to create the user cache directory, so it is
    /// fallible. (Due to its `app_dirs2` implementation, it would have to be
    /// fallible even if it didn't perform I/O.)
    pub fn get_user_default() -> Result<Self> {
        let env_cache_path = env::var_os("TECTONIC_CACHE_DIR");

        let cache_path = match env_cache_path {
            Some(env_cache_path) => {
                let env_cache_path = env_cache_path.into();
                fs::create_dir_all(&env_cache_path)?;
                env_cache_path
            }
            None => app_dirs::ensure_user_cache_dir("")?,
        };

        Ok(Cache { root: cache_path })
    }

    /// Get a handle to a bundle cache, using a custom cache directory.
    pub fn get_for_custom_directory<P: Into<PathBuf>>(root: P) -> Self {
        Cache { root: root.into() }
    }

    /// Get the root directory of this cache.
    pub fn root(&self) -> &Path {
        &self.root
    }
}

/// Information about a cached file.
#[derive(Clone, Copy, Debug)]
struct CachedFileInfo {
    /// The length of the file in bytes.
    ///
    /// This field isn't currently used, but seems handy to keep around.
    _length: u64,

    /// The digest of the file contents.
    ///
    /// This digest is used to locate the cached data on disk.
    digest: DigestData,
}

/// A caching bundle that obtains files from some a backend.
///
/// This bundle implementation is the key to Tectonic’s ability to download TeX
/// support files on the fly. The cache backend is generally expected to be some
/// kind of network-based resource, and the caching scheme is designed so that a
/// document build can avoid touching the network altogether if no new files
/// need to be downloaded.
//#[derive(Debug)]
pub struct BundleCache {
    /// The cached value of the backend’s content digest.
    ///
    /// This is stored in a file at [`Self::digest_path`]. This value may be
    /// inaccurate, if the backing bundle has been updated (or if the cache is
    /// corrupt, etc.) and we haven't yet synchronized with the backend and
    /// discovered that fact.
    cached_digest: DigestData,

    /// Information about all of the files that have been cached locally.
    ///
    /// This maps filenames to summary information that can then be used to
    /// retrieve file data from [`Self::data_base`]. The contents are loaded
    /// from the manifest file if the cache is non-empty.
    contents: HashMap<String, CachedFileInfo>,

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
    bundle: Box<dyn Bundle>,

    /// The root directory of this cache.
    /// All other paths are subdirectories of this path.
    //root_path: PathBuf,

    /// The path to a file containing a cached copy of the backend's content
    /// digest.
    ///
    /// This file path is based on [`Self::start_url`].
    //digest_path: PathBuf,

    /// A directory where we will save the cache manifest.
    ///
    /// The manifest file contains information about the files that have
    /// actually been fetched from the backend and saved locally. The actual
    /// manifest file path is based on the backend’s content digest.
    manifest_path: PathBuf,

    /// A directory where we will save cached file data.
    ///
    /// This directory contains the actual cached file contents, in a directory
    /// structured based on the digest of each file’s content.
    data_path: PathBuf,
}

impl BundleCache {
    /// Make a new filesystem-backed cache from `bundle`.
    pub fn new(
        mut bundle: Box<dyn Bundle>,
        only_cached: bool,
        status: &mut dyn StatusBackend,
        root_path: PathBuf,
    ) -> Result<Self> {
        // Set up our paths.
        let digest_path = ensure_cache_dir(&root_path, "digests")?
            .join(app_dirs::app_dirs2::sanitized(&bundle.get_location()));
        let manifest_base = ensure_cache_dir(&root_path, "manifests")?;
        let data_path = ensure_cache_dir(&root_path, "files")?;

        // The whole point of this cache is to avoid connecting to the backend
        // if at all possible. So we first see if we have cached the "pull data"
        // that describe the overall backend contents.

        let cached_metadata = match Self::load_cached_metadata(&digest_path)? {
            Some(c) => c,
            None => {
                // Some portion of the required cached data is missing. We need to
                // do a complete pull and then cache the results.

                let digest = bundle.get_digest(status)?;
                file_create_write(&digest_path, |f| writeln!(f, "{}", &digest.to_string()))?;

                // Now that we've done that, load_cached_pull_data() really ought to succeed ...
                atry!(
                    Self::load_cached_metadata(&digest_path)?;
                    ["cache files missing even after they were created"]
                )
            }
        };

        let cached_digest = cached_metadata;

        // Make sure the source bundle's digest is what we expect it to be.
        // This is expensive, since we need to download a fresh digest from web bundles.
        // We only check this once, and assume that connection status will not change during a build.
        //
        // Update cached_digest if bundle digest changes.
        let cached_digest = match bundle.get_digest(status) {
            // If we can't connect to the bundle, we can't cache any
            // bad files. Continue.
            Err(_) => {
                tt_note!(
                    status,
                    "Could not connect to bundle, skipping digest check."
                );
                cached_digest
            }
            Ok(bundle_digest) => {
                // The backend isn't what we thought it was.
                // Rewrite the digest file (and variable) so our cache stays correct.
                if cached_digest != bundle_digest {
                    file_create_write(&digest_path, |f| {
                        writeln!(f, "{}", bundle_digest.to_string())
                    })?;
                    tt_warning!(status, "Bundle digest changed; adjusting cache.");
                    bundle_digest
                } else {
                    cached_digest
                }
            }
        };

        // Now that we have the backend content digest, we know which manifest
        // to use. Read it in, if it exists.
        let manifest_path = make_txt_path(&manifest_base, &cached_digest.to_string());
        let manifest = Self::parse_manifest(&manifest_path, status)?;

        Ok(BundleCache {
            cached_digest,

            contents: manifest,
            only_cached,
            bundle,

            manifest_path,
            //digest_path,
            data_path,
            //root_path,
        })
    }

    /// Load all cached metadata. If any files are missing or wrong, return None.
    fn load_cached_metadata(digest_path: &Path) -> Result<Option<DigestData>> {
        // Convert file-not-found errors into None.
        return match inner(digest_path) {
            Ok(r) => Ok(Some(r)),
            Err(e) => {
                if let Some(ioe) = e.downcast_ref::<IoError>() {
                    if ioe.kind() == IoErrorKind::NotFound {
                        return Ok(None);
                    }
                }

                Err(e)
            }
        };

        fn inner(digest_path: &Path) -> Result<DigestData> {
            let digest_text = {
                let f = File::open(digest_path)?;
                let mut digest_text = String::with_capacity(digest::DIGEST_LEN);
                f.take(digest::DIGEST_LEN as u64)
                    .read_to_string(&mut digest_text)?;
                digest_text
            };

            Ok(DigestData::from_str(&digest_text)?)
        }
    }

    /// Parse a cache manifest
    fn parse_manifest(
        manifest_path: &PathBuf,
        status: &mut dyn StatusBackend,
    ) -> Result<HashMap<String, CachedFileInfo>> {
        let mut contents = HashMap::new(); // Read manifest into here
        let file = match try_open_file(manifest_path) {
            OpenResult::NotAvailable => {
                return Ok(contents);
            }
            OpenResult::Err(e) => {
                return Err(e);
            }
            OpenResult::Ok(file) => file,
        };

        // Note that the lock is released when the file is closed,
        // which is good since BufReader::new() and BufReader::lines()
        // consume their objects.
        if let Err(e) = file.lock_shared() {
            tt_warning!(status, "failed to lock manifest file \"{}\" for reading; this might be fine",
                        manifest_path.display(); e.into());
        }

        let f = BufReader::new(file);

        for res in f.lines() {
            let line = res?;
            let mut bits = line.rsplitn(3, ' ');

            let (original_name, length, digest) =
                match (bits.next(), bits.next(), bits.next(), bits.next()) {
                    (Some(s), Some(t), Some(r), None) => (r, t, s),
                    _ => continue,
                };

            let name = original_name.to_owned();

            let length = match length.parse::<u64>() {
                Ok(l) => l,
                Err(_) => continue,
            };

            let digest = if digest == "-" {
                continue;
            } else {
                match DigestData::from_str(digest) {
                    Ok(d) => d,
                    Err(e) => {
                        tt_warning!(status, "ignoring bad digest data \"{}\" for \"{}\" in \"{}\"",
                                    &digest, original_name, manifest_path.display() ; e);
                        continue;
                    }
                }
            };

            contents.insert(
                name,
                CachedFileInfo {
                    _length: length,
                    digest,
                },
            );
        }

        return Ok(contents);
    }

    /// Save data about a file to our local cache manifest.
    fn save_to_manifest(&mut self, name: &str, length: u64, digest: DigestData) -> Result<()> {
        let digest_text = digest.to_string();

        // Due to a quirk about permissions for file locking on Windows, we
        // need to add `.read(true)` to be able to lock a file opened in
        // append mode.
        let mut man = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open(&self.manifest_path)?;

        // Lock will be released when file is closed at the end of this function.
        atry!(
            man.lock_exclusive();
            ["failed to lock manifest file \"{}\" for writing", self.manifest_path.display()]
        );

        // If a filename contains newline characters, it will mess up our
        // line-based manifest format. Be paranoid and refuse to record such
        // filenames.
        if !name.contains(|c| c == '\n' || c == '\r') {
            writeln!(man, "{name} {length} {digest_text}")?;
        }

        self.contents.insert(
            name.to_owned(),
            CachedFileInfo {
                _length: length,
                digest,
            },
        );

        Ok(())
    }

    /// Make sure that a file is available, and return its filesystem path.
    ///
    /// If the file is already cached, just pull it out. Otherwise, fetch it
    /// from the backend.
    fn ensure_file_availability(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<PathBuf> {
        // Already in the cache?
        if let Some(info) = self.contents.get(name) {
            return match info.digest.create_two_part_path(&self.data_path) {
                Ok(p) => OpenResult::Ok(p),
                Err(e) => OpenResult::Err(e),
            };
        }

        // No, it's not. Are we in cache-only mode?
        if self.only_cached {
            return OpenResult::NotAvailable;
        }

        // Get the file.
        let mut content = match self.bundle.input_open_name(name, status) {
            OpenResult::Ok(c) => c,
            OpenResult::Err(e) => return OpenResult::Err(e),
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
        };
        let mut buf: Vec<u8> = Vec::new();
        if let Err(e) = content.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        };
        let length = buf.len();

        let mut digest_builder = digest::create();
        digest_builder.update(&buf);
        let digest = DigestData::from(digest_builder);

        let final_path = match digest.create_two_part_path(&self.data_path) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e),
        };

        // Perform a racy check for the destination existing, because this
        // matters on Windows: if the destination is already there, we'll get
        // an error because the destination is marked read-only. Assuming
        // non-pathological filesystem manipulation, though, we'll only be
        // subject to the race once.

        if !final_path.exists() {
            if let Err(e) = file_create_write(&final_path, |f| f.write_all(&buf)) {
                return OpenResult::Err(e);
            }

            // Now we can make the file readonly. It would be nice to set the
            // permissions using the already-open file handle owned by the
            // tempfile, but mkstemp doesn't give us access.
            let mut perms = match fs::metadata(&final_path) {
                Ok(p) => p,
                Err(e) => {
                    return OpenResult::Err(e.into());
                }
            }
            .permissions();
            perms.set_readonly(true);

            if let Err(e) = fs::set_permissions(&final_path, perms) {
                return OpenResult::Err(e.into());
            }
        }

        // And finally add a record of this file to our manifest. Note that
        // we're opening and closing the manifest every time we cache a new
        // file; not so efficient, but whatever.

        if let Err(e) = self.save_to_manifest(name, length as u64, digest) {
            return OpenResult::Err(e);
        }

        OpenResult::Ok(final_path)
    }
}

impl IoProvider for BundleCache {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let path = match self.ensure_file_availability(name, status) {
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

impl Bundle for BundleCache {
    fn get_digest(&mut self, _status: &mut dyn StatusBackend) -> Result<DigestData> {
        Ok(self.cached_digest)
    }

    // Returns a list of all files that are *available*.
    // If we're online, this is the list of all files in the bundle.
    // If we're offline, this is every file in the cache.
    //
    // TODO: Maybe we want different errors for "offline" and "actually doesn't exist"?
    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        return Ok(match self.bundle.all_files(status) {
            Err(e) => return Err(e),
            Ok(a) => {
                if a.len() == 0 {
                    self.contents.keys().cloned().collect()
                } else {
                    a
                }
            }
        });
    }
}

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

/// Convenience to generate a text filename
fn make_txt_path(base: &Path, name: &str) -> PathBuf {
    base.join(name).with_extension("txt")
}
