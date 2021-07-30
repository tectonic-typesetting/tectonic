// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Local caching of bundle data.
//!
//! This module implements Tectonic’s local filesystem caching mechanism for TeX
//! support files. To enable efficient caching with proper invalidation
//! semantics, the caching layer does *not* merely wrap [`IoProvider`]
//! implementations. Instead, a cacheable bundle must implement the
//! [`CacheBackend`] trait defined in this module. An example of such a bundle
//! is the [`crate::itar::IndexedTarBackend`] for bundles served over HTTP.
//!
//! In order to access a cacheable bundle, you need a handle to a local
//! [`Cache`], probably obtained with [`Cache::get_user_default()`], and a URL,
//! which you’ll pass to [`Cache::open()`]. When using this function, you must
//! explicitly specify the concrete [`CacheBackend`] type that will service
//! backend requests.

use fs2::FileExt;
use std::{
    collections::HashMap,
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
use tectonic_status_base::{tt_warning, StatusBackend};

use crate::Bundle;

/// A cache of data from one or more bundles using the local filesystem.
#[derive(Debug)]
pub struct Cache {
    root: PathBuf,
}

impl Cache {
    /// Get a handle to a bundle cache, using default per-user settings.
    ///
    /// This method may perform I/O to create the user cache directory, so it is
    /// fallible. (Due to its `app_dirs2` implementation, it would have to be
    /// fallible even if it didn't perform I/O.)
    pub fn get_user_default() -> Result<Self> {
        Ok(Cache {
            root: app_dirs::ensure_user_cache_dir("")?,
        })
    }

    /// Get a handle to a bundle cache, using a custom cache directory.
    pub fn get_for_custom_directory<P: Into<PathBuf>>(root: P) -> Self {
        Cache { root: root.into() }
    }

    /// Get the root directory of this cache.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Open a bundle through the cache layer.
    ///
    /// The URL specifies where the backend data live; it must be understood by,
    /// and contain data appropriate for, the [`CacheBackend`] type associated
    /// with the bundle that you’re creating. If *only_cached* is true, this
    /// instance will never actually connect to the backend; if any uncached
    /// files are requested, they will be represented as “not found”.
    pub fn open<CB: CacheBackend>(
        &mut self,
        url: &str,
        only_cached: bool,
        status: &mut dyn StatusBackend,
    ) -> Result<CachingBundle<CB>> {
        CachingBundle::new(url, only_cached, status, &self.root)
    }
}

/// Information describing a cache backend.
///
/// This type is returned by a [`CacheBackend`] on a "pull", a first-time
/// connection to the backend. It contains the detailed information that needs
/// to be saved in the cache to provide for efficient operation in subsequent
/// uses.
#[derive(Clone, Debug)]
pub struct BackendPullData {
    /// The final, "resolved" URL pointing to the backing content, in the case
    /// that the starting URL redirects.
    pub resolved_url: String,

    /// The digest of the overall bundle content.
    pub digest: DigestData,

    /// The bundle indexing data, allowing efficient retrieval of files from the
    /// backend.
    ///
    /// This is a multi-line string, where each line is an entry for a file.
    /// These lines will be parsed by [`CacheBackend::parse_index_line`]. This
    /// string will potentially contain several megabytes of data.
    pub index: String,
}

/// A source of files that can supply a cache-based bundle.
///
/// This trait is combined with [`CachingBundle`] to implement a caching bundle
/// interface.
pub trait CacheBackend: Sized {
    /// Information about a file stored in the backend.
    ///
    /// This information should be serializable to a single line of text. It is
    /// parsed out of the contents of [`BackendPullData::index`] by
    /// [`Self::parse_index_line`], and later passed to [`Self::get_file`] to
    /// enable the backend to efficiently retrieve the file in question. For
    /// instance, it might contain offset information informing the backend how
    /// to efficiently retrieve the file in question.
    type FileInfo: Clone;

    /// Connect to the backend and download its key information.
    ///
    /// This method is used the first time that the cache connects to a backend.
    /// The return value includes a package of information ([`BackendPullData`])
    /// that the cache will store to enable efficient operation on subsequent
    /// requests.
    fn open_with_pull(
        start_url: &str,
        status: &mut dyn StatusBackend,
    ) -> Result<(Self, BackendPullData)>;

    /// Connect to the backend and fetch validation information.
    ///
    /// This method is used when this backend has already been accessed by the
    /// cache during a previous execution. If we need to download more data from
    /// the backend, we first need to verify that the cached data still look
    /// valid. This method asks the backend to pull its “digest file” (currently
    /// named `SHA256SUM`) and return its contents for validate. The method
    /// should return `Err` on actual errors, and `Ok(None)` if there are any
    /// indications that the cached indexing data should be thrown out and
    /// re-fetched.
    fn open_with_quick_check(
        resolved_url: &str,
        digest_file_info: &Self::FileInfo,
        status: &mut dyn StatusBackend,
    ) -> Result<Option<(Self, DigestData)>>;

    /// Parse a line of the indexing data.
    ///
    /// The returned tuple should give the file name and an opaque
    /// [`Self::FileInfo`] that may help the backend retrieve the file in the
    /// future. The indexing data are originally obtained from
    /// [`BackendPullData::index`], but are stored in a file locally. This
    /// method should return an error if this particular line of index data
    /// seems to be malformatted. Such lines will probably just be silently
    /// ignored.
    fn parse_index_line(line: &str) -> Result<(String, Self::FileInfo)>;

    /// Obtain a file from the backend.
    ///
    /// Backend-specific retrieval information can be passed in the
    /// [`Self::FileInfo`] item, which is constructed from the backend’s index
    /// information. The file should be returned as one large byte vector.
    fn get_file(
        &mut self,
        name: &str,
        info: &Self::FileInfo,
        status: &mut dyn StatusBackend,
    ) -> Result<Vec<u8>>;
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
#[derive(Debug)]
pub struct CachingBundle<CB: CacheBackend> {
    /// The URL specifying where to start looking for the bundle data.
    ///
    /// The caching layer maintains two URLs: the "start" URL and the "resolved"
    /// URL. The goal here is to be able to store a single URL for fetching
    /// data, but maintain the capability to update the bundle data behind that
    /// URL. Requests to the start URL may get redirected (one or more times)
    /// until eventually we arrive at the "resolved" URL. While the redirection
    /// of the start URL might change, the contents of a resolved URL should
    /// never change once published.
    start_url: String,

    /// The "resolved" URL for the backing data.
    ///
    /// The bundle data located at this URL should never change.
    resolved_url: String,

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

    /// Information about all of the files known to the backend.
    ///
    /// This maps filenames to [`CacheBackend::FileInfo`] data that can be used
    /// to retrieve a file from the backend if needed.
    index: HashMap<String, CB::FileInfo>,

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
    backend: Option<CB>,

    /// The path to a file containing a cached copy of the backend's content
    /// digest.
    ///
    /// This file path is based on [`Self::start_url`].
    digest_path: PathBuf,

    /// A directory where we will save [`Self::resolved_url`].
    ///
    /// We need to cache `resolved_url` to enable the "quick check" backend
    /// reconnection path. The actual cache file path is based on the backend’s
    /// content digest.
    resolved_base: PathBuf,

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
    data_base: PathBuf,
}

/// A locally-cached analogue of [`BackendPullData`].
///
/// This data structure is what we try to recover from the cache to see if we
/// can avoid connecting to the backend.
#[derive(Clone, Debug)]
struct CachedPullData<FI> {
    /// The saved backend content digest.
    pub digest: DigestData,

    /// The saved "resolved URL" for the backend.
    pub resolved_url: String,

    /// The saved indexing information for the backend.
    pub index: HashMap<String, FI>,
}

impl<CB: CacheBackend> CachingBundle<CB> {
    fn new(
        start_url: &str,
        only_cached: bool,
        status: &mut dyn StatusBackend,
        cache_root: &Path,
    ) -> Result<Self> {
        // Set up our paths.
        let digest_path =
            ensure_cache_dir(cache_root, "urls")?.join(app_dirs::app_dirs2::sanitized(start_url));
        let resolved_base = ensure_cache_dir(cache_root, "redirects")?;
        let index_base = ensure_cache_dir(cache_root, "indexes")?;
        let manifest_base = ensure_cache_dir(cache_root, "manifests")?;
        let data_base = ensure_cache_dir(cache_root, "files")?;

        // The whole point of this cache is to avoid connecting to the backend
        // if at all possible. So we first see if we have cached the "pull data"
        // that describe the overall backend contents.

        let mut backend = None;

        let cached_pull_data =
            match load_cached_pull_data::<CB>(&digest_path, &resolved_base, &index_base)? {
                Some(c) => c,
                None => {
                    // Some portion of the required cached data is missing. We need to
                    // do a complete pull and then cache the results.

                    let (new_backend, pull_data) = CB::open_with_pull(start_url, status)?;
                    backend = Some(new_backend);

                    let digest_text = pull_data.digest.to_string();
                    file_create_write(&digest_path, |f| writeln!(f, "{}", &digest_text))?;
                    file_create_write(make_txt_path(&resolved_base, &digest_text), |f| {
                        f.write_all(pull_data.resolved_url.as_bytes())
                    })?;
                    file_create_write(make_txt_path(&index_base, &digest_text), |f| {
                        f.write_all(pull_data.index.as_bytes())
                    })?;

                    // Now that we've done that, load_cached_pull_data() really ought to succeed ...
                    atry!(
                        load_cached_pull_data::<CB>(&digest_path, &resolved_base, &index_base)?;
                        ["cache files missing even after they were created"]
                    )
                }
            };

        // We call this `cached_digest`, but if `backend` is Some, it is a
        // validated, fresh digest.

        let cached_digest = cached_pull_data.digest;

        // Now that we have the backend content digest, we know which manifest
        // to use. Read it in, if it exists.

        let manifest_path = make_txt_path(&manifest_base, &cached_digest.to_string());
        let mut contents = HashMap::new();

        match try_open_file(&manifest_path) {
            OpenResult::NotAvailable => {}
            OpenResult::Err(e) => {
                return Err(e);
            }
            OpenResult::Ok(mfile) => {
                // Note that the lock is released when the file is closed,
                // which is good since BufReader::new() and BufReader::lines()
                // consume their objects.
                if let Err(e) = mfile.lock_shared() {
                    tt_warning!(status, "failed to lock manifest file \"{}\" for reading; this might be fine",
                                manifest_path.display(); e.into());
                }

                let f = BufReader::new(mfile);

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
            }
        }

        // All set.

        Ok(CachingBundle {
            start_url: start_url.to_owned(),
            resolved_url: cached_pull_data.resolved_url,
            digest_path,
            cached_digest,
            manifest_path,
            data_base,
            resolved_base,
            contents,
            only_cached,
            backend,
            index: cached_pull_data.index,
        })
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
            writeln!(man, "{} {} {}", name, length, digest_text)?;
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

    /// Ensure that the backend is connected and valid.
    ///
    /// Here we do a "quick check" to see if the backend's digest is what we
    /// expect. If not, we do a lame thing where we error out but set things up
    /// so that things should succeed if the program is re-run. Exactly the lame
    /// TeX user experience that I've been trying to avoid!
    ///
    /// After this function has been called, you can assume that `self.backend`
    /// is Some.
    fn ensure_backend_validity(&mut self, status: &mut dyn StatusBackend) -> Result<()> {
        // If backend is Some, we already have a validated connection to it.
        if self.backend.is_some() {
            return Ok(());
        }

        // Do the quick check. If anything goes wrong, eat the error and try a
        // fresh pull.
        if let Some(info) = self.index.get(digest::DIGEST_NAME) {
            if let Ok(Some((backend, digest))) =
                CB::open_with_quick_check(&self.resolved_url, info, status)
            {
                if self.cached_digest == digest {
                    // We managed to pull some data that match the digest. We
                    // can be quite confident that the bundle is what we expect
                    // it to be.
                    self.backend = Some(backend);
                    return Ok(());
                }
            }
        }

        // The quick check failed. Try to pull all data to make sure that it
        // wasn't a network error or that the resolved URL hasn't been updated.
        let (new_backend, pull_data) = CB::open_with_pull(&self.start_url, status)?;

        if self.cached_digest != pull_data.digest {
            // Crap! The backend isn't what we thought it was. We may have been
            // giving incorrect results if we pulled files out of the cache
            // before this invocation. Rewrite the digest file so that next time
            // we'll start afresh, then bail.
            file_create_write(&self.digest_path, |f| {
                writeln!(f, "{}", pull_data.digest.to_string())
            })?;
            bail!("backend digest changed; rerun tectonic to use updated information");
        }

        if self.resolved_url != pull_data.resolved_url {
            // The resolved URL has changed, but the digest is the same. So
            // let's just update the URL and keep going.
            let resolved_path = make_txt_path(&self.resolved_base, &pull_data.digest.to_string());
            file_create_write(&resolved_path, |f| {
                f.write_all(pull_data.resolved_url.as_bytes())
            })?;

            self.resolved_url = pull_data.resolved_url;
        }

        // OK, it seems that everything is in order.
        self.backend = Some(new_backend);
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
            return match info.digest.create_two_part_path(&self.data_base) {
                Ok(p) => OpenResult::Ok(p),
                Err(e) => OpenResult::Err(e),
            };
        }

        // No, it's not. Are we in cache-only mode?
        if self.only_cached {
            return OpenResult::NotAvailable;
        }

        // Is the file in the backend at all?
        let info = match self.index.get(name).cloned() {
            Some(info) => info,
            None => return OpenResult::NotAvailable,
        };

        // Yes, it is. Time to fetch it! In order to do that, we need to ensure
        // that we have a valid backend connection.
        if let Err(e) = self.ensure_backend_validity(status) {
            return OpenResult::Err(e);
        }

        // Cool, we're connected to the backend now. Get the file. Note that we
        // don't need to check for updates to the index after the
        // ensure-validity, because we require that the contents of the bundle
        // are unchanged (as expressed in the content digest): if they did
        // change, ensure_backend_validity() would have bailed, because we might
        // have returned incorrect data for previous requests that hit the
        // cache.

        let content = match self.backend.as_mut().unwrap().get_file(name, &info, status) {
            Ok(c) => c,
            Err(e) => return OpenResult::Err(e),
        };

        let length = content.len();

        let mut digest_builder = digest::create();
        digest_builder.update(&content);
        let digest = DigestData::from(digest_builder);

        let final_path = match digest.create_two_part_path(&self.data_base) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e),
        };

        // Perform a racy check for the destination existing, because this
        // matters on Windows: if the destination is already there, we'll get
        // an error because the destination is marked read-only. Assuming
        // non-pathological filesystem manipulation, though, we'll only be
        // subject to the race once.

        if !final_path.exists() {
            if let Err(e) = file_create_write(&final_path, |f| f.write_all(&content)) {
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

impl<CB: CacheBackend> IoProvider for CachingBundle<CB> {
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

        let f = match File::open(&path) {
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

impl<CB: CacheBackend> Bundle for CachingBundle<CB> {
    fn get_digest(&mut self, _status: &mut dyn StatusBackend) -> Result<DigestData> {
        Ok(self.cached_digest)
    }

    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        if !self.only_cached {
            self.ensure_backend_validity(status)?;
        }
        Ok(self.index.keys().cloned().collect())
    }
}

/// Load the cached "pull" data for a backend.
///
/// If any of the files are not found or otherwise have issues, return None.
fn load_cached_pull_data<CB: CacheBackend>(
    digest_path: &Path,
    resolved_base: &Path,
    index_base: &Path,
) -> Result<Option<CachedPullData<CB::FileInfo>>> {
    // Convert file-not-found errors into None.
    return match inner::<CB>(digest_path, resolved_base, index_base) {
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

    fn inner<CB: CacheBackend>(
        digest_path: &Path,
        resolved_base: &Path,
        index_base: &Path,
    ) -> Result<CachedPullData<CB::FileInfo>> {
        let digest_text = {
            let f = File::open(digest_path)?;
            let mut digest_text = String::with_capacity(digest::DIGEST_LEN);
            f.take(digest::DIGEST_LEN as u64)
                .read_to_string(&mut digest_text)?;
            digest_text
        };

        let resolved_path = make_txt_path(resolved_base, &digest_text);
        let resolved_url = fs::read_to_string(resolved_path)?;

        let index_path = make_txt_path(index_base, &digest_text);
        let index = {
            let f = File::open(index_path)?;
            let mut index = HashMap::new();
            for line in BufReader::new(f).lines() {
                if let Ok((name, info)) = CB::parse_index_line(&line?) {
                    index.insert(name, info);
                }
            }
            index
        };

        Ok(CachedPullData {
            digest: DigestData::from_str(&digest_text)?,
            resolved_url,
            index,
        })
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
    base.join(&name).with_extension("txt")
}
