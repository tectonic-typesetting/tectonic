// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use fs2::FileExt;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use tectonic_errors::{anyhow::bail, atry, Result};
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend, RangeReader};

use super::{try_open_file, Bundle, InputHandle, InputOrigin, IoProvider, OpenResult};
use crate::app_dirs;
use crate::digest::{self, Digest, DigestData};
use crate::errors::SyncError;
use crate::status::StatusBackend;
use crate::{tt_note, tt_warning};

const MAX_HTTP_ATTEMPTS: usize = 4;

#[derive(Clone, Copy, Debug)]
struct FileInfo {
    offset: u64,
    length: u64,
}

#[derive(Clone, Copy, Debug)]
struct LocalCacheItem {
    _length: u64,
    digest: DigestData,
}

/// Attempts to download a file from the bundle.
fn get_file(
    data: &mut DefaultRangeReader,
    name: &str,
    offset: u64,
    length: usize,
    status: &mut dyn StatusBackend,
) -> Result<Vec<u8>> {
    // In principle it'd be cool to return a handle right to the HTTP
    // response, but those can't be seekable, and doing so introduces
    // lifetime-related issues. So for now we just slurp the whole thing
    // into RAM.

    tt_note!(status, "downloading {}", name);

    // When fetching a bunch of resource files (i.e., on the first
    // invocation), bintray will sometimes drop connections. The error
    // manifests itself in a way that has a not-so-nice user experience.
    // Our solution: retry the HTTP a few times in case it was a transient
    // problem.

    let mut buf = Vec::with_capacity(length);
    let mut overall_failed = true;
    let mut any_failed = false;

    for _ in 0..MAX_HTTP_ATTEMPTS {
        let mut stream = match data.read_range(offset, length) {
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

fn parse_index_line(line: &str) -> Result<Option<(String, FileInfo)>> {
    let mut bits = line.split_whitespace();

    if let (Some(name), Some(offset), Some(length)) = (bits.next(), bits.next(), bits.next()) {
        Ok(Some((
            name.to_owned(),
            FileInfo {
                offset: offset.parse::<u64>()?,
                length: length.parse::<u64>()?,
            },
        )))
    } else {
        // TODO: preserve the warning info or something!
        Ok(None)
    }
}

/// Attempts to find the redirected url, download the index and digest.
fn get_everything(
    backend: &mut DefaultBackend,
    url: &str,
    status: &mut dyn StatusBackend,
) -> Result<(String, String, String)> {
    let url = backend.resolve_url(url, status)?;

    let index = {
        let mut index = String::new();
        let index_url = format!("{}.index.gz", &url);
        tt_note!(status, "downloading index {}", index_url);
        GzDecoder::new(backend.get_url(&index_url, status)?).read_to_string(&mut index)?;
        index
    };

    let digest_text = {
        // Find the location of the digest file.
        let digest_info = {
            let mut digest_info = None;
            for line in index.lines() {
                if let Some((name, info)) = parse_index_line(line)? {
                    if name == digest::DIGEST_NAME {
                        digest_info = Some(info);
                        break;
                    }
                }
            }
            atry!(digest_info; ["backend does not provide needed {} file", digest::DIGEST_NAME])
        };

        let mut range_reader = backend.open_range_reader(&url);
        String::from_utf8(get_file(
            &mut range_reader,
            digest::DIGEST_NAME,
            digest_info.offset,
            digest_info.length as usize,
            status,
        )?)
        .map_err(|e| e.utf8_error())?
    };

    Ok((digest_text, index, url))
}

#[derive(Clone, Debug)]
struct CacheContent {
    digest_text: String,
    redirect_url: String,
    index: HashMap<String, FileInfo>,
}

/// Load cached data.
///
/// If any of the files is not found return None.
fn load_cache(
    digest_path: &Path,
    redirect_base: &Path,
    index_base: &Path,
) -> Result<Option<CacheContent>> {
    // Convert file-not-found errors into None.
    match load_cache_inner(digest_path, redirect_base, index_base) {
        Ok(r) => Ok(Some(r)),
        Err(e) => {
            if let Some(ioe) = e.downcast_ref::<IoError>() {
                if ioe.kind() == IoErrorKind::NotFound {
                    return Ok(None);
                }
            }

            Err(e)
        }
    }
}

/// See `load_cache`.
fn load_cache_inner(
    digest_path: &Path,
    redirect_base: &Path,
    index_base: &Path,
) -> Result<CacheContent> {
    let digest_text = {
        let f = File::open(digest_path)?;
        let mut digest_text = String::with_capacity(digest::DIGEST_LEN);
        f.take(digest::DIGEST_LEN as u64)
            .read_to_string(&mut digest_text)?;
        digest_text
    };

    let redirect_path = make_txt_path(redirect_base, &digest_text);
    let redirect_url = fs::read_to_string(redirect_path)?;

    let index_path = make_txt_path(index_base, &digest_text);

    let index = {
        let f = File::open(index_path)?;
        let mut index = HashMap::new();
        for line in BufReader::new(f).lines() {
            if let Some((name, info)) = parse_index_line(&line?)? {
                index.insert(name, info);
            }
        }
        index
    };
    Ok(CacheContent {
        digest_text,
        redirect_url,
        index,
    })
}

fn make_txt_path(base: &Path, digest_text: &str) -> PathBuf {
    base.join(&digest_text).with_extension("txt")
}

/// Bundle provided by an indexed tar file over http with a local cache.
#[derive(Debug)]
pub struct CachedITarBundle {
    url: String,
    redirect_url: String,
    digest_path: PathBuf,
    cached_digest: DigestData,
    checked_digest: bool,
    redirect_base: PathBuf,
    manifest_path: PathBuf,
    data_base: PathBuf,
    contents: HashMap<String, LocalCacheItem>,
    only_cached: bool,

    tar_data: DefaultRangeReader,
    index: HashMap<String, FileInfo>,
}

impl CachedITarBundle {
    pub fn new(
        url: &str,
        only_cached: bool,
        custom_cache_root: Option<&Path>,
        status: &mut dyn StatusBackend,
    ) -> Result<CachedITarBundle> {
        let mut backend = DefaultBackend::default();
        let digest_path = cache_dir("urls", custom_cache_root)?.join(app_dirs::sanitized(url));

        let redirect_base = &cache_dir("redirects", custom_cache_root)?;
        let index_base = &cache_dir("indexes", custom_cache_root)?;
        let manifest_base = &cache_dir("manifests", custom_cache_root)?;
        let data_base = &cache_dir("files", custom_cache_root)?;

        let mut checked_digest = false;
        let CacheContent {digest_text, redirect_url, index} =
            // Try loading the cached files.
            match load_cache(&digest_path, &redirect_base, &index_base)? {
                Some(c) => c,
                None => {
                    // At least one of the cached files does not exists. We fetch everything from
                    // scratch and save the files.
                    let (digest_text, index, redirect_url) = get_everything(&mut backend, url, status)?;
                    let _ = DigestData::from_str(&digest_text)?;
                    checked_digest = true;

                    file_create_write(&digest_path, |f| writeln!(f, "{}", digest_text))?;
                    file_create_write(make_txt_path(&redirect_base, &digest_text), |f| f.write_all(redirect_url.as_bytes()))?;
                    file_create_write(make_txt_path(&index_base, &digest_text), |f| f.write_all(index.as_bytes()))?;

                    // Reload the cached files now when they were saved.
                    atry!(load_cache(&digest_path, &redirect_base, &index_base)?; ["cache files missing even after they were created"])
                }
            };

        let cached_digest = DigestData::from_str(&digest_text)?;

        // We can now figure out which manifest to use.
        let manifest_path = make_txt_path(manifest_base, &digest_text);

        // Read it in, if it exists.

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
                        match DigestData::from_str(&digest) {
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
                        LocalCacheItem {
                            _length: length,
                            digest,
                        },
                    );
                }
            }
        }

        // All set.

        let tar_data = backend.open_range_reader(&redirect_url);

        Ok(CachedITarBundle {
            url: url.to_owned(),
            redirect_url,
            digest_path,
            cached_digest,
            checked_digest,
            manifest_path,
            data_base: data_base.to_owned(),
            redirect_base: redirect_base.to_owned(),
            contents,
            only_cached,
            tar_data,
            index,
        })
    }

    fn record_cache_result(&mut self, name: &str, length: u64, digest: DigestData) -> Result<()> {
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
        atry!(man.lock_exclusive(); ["failed to lock manifest file \"{}\" for writing", self.manifest_path.display()]);

        if !name.contains(|c| c == '\n' || c == '\r') {
            writeln!(man, "{} {} {}", name, length, digest_text)?;
        }
        self.contents.insert(
            name.to_owned(),
            LocalCacheItem {
                _length: length,
                digest,
            },
        );
        Ok(())
    }

    /// If we're going to make a request of the backend, we should check that
    /// its digest is what we expect. If not, we do a lame thing where we
    /// error out but set things up so that things should succeed if the
    /// program is re-run. Exactly the lame TeX user experience that I've been
    /// trying to avoid!
    fn check_digest(&mut self, status: &mut dyn StatusBackend) -> Result<()> {
        if self.checked_digest {
            return Ok(());
        }

        // Do a quick and dirty check first and ignore errors.
        if let Some(info) = self.index.get(digest::DIGEST_NAME) {
            if let Ok(d) = get_file(
                &mut self.tar_data,
                digest::DIGEST_NAME,
                info.offset,
                info.length as usize,
                status,
            ) {
                if let Ok(d) = String::from_utf8(d) {
                    if let Ok(d) = DigestData::from_str(&d) {
                        if self.cached_digest == d {
                            // We managed to pull some data that match the digest.
                            // We can be quite confident that the bundle is what we expect it to be.
                            self.checked_digest = true;
                            return Ok(());
                        }
                    }
                }
            }
        }

        // The quick check failed. Try to pull all data to make sure that it wasn't a network
        // error or that the redirect url hasn't been updated.
        let mut backend = DefaultBackend::default();
        let (digest_text, _index, redirect_url) = get_everything(&mut backend, &self.url, status)?;

        let current_digest =
            atry!(DigestData::from_str(&digest_text); ["bad SHA256 digest from bundle"]);

        if self.cached_digest != current_digest {
            // Crap! The backend isn't what we thought it was. Rewrite the
            // digest file so that next time we'll start afresh.

            file_create_write(&self.digest_path, |f| {
                writeln!(f, "{}", current_digest.to_string())
            })?;
            bail!("backend digest changed; rerun tectonic to use updated information");
        }

        if self.redirect_url != redirect_url {
            // The redirect url has changed, let's update it.
            let redirect_path = make_txt_path(&self.redirect_base, &digest_text);
            file_create_write(&redirect_path, |f| f.write_all(redirect_url.as_bytes()))?;

            self.redirect_url = redirect_url;
        }

        // Index should've changed as the digest hasn't.

        // Phew, the backend hasn't changed. Don't check again.
        self.checked_digest = true;
        Ok(())
    }

    /// Find the path in the local cache for the provided file. Download the file first if it is
    /// not in the local cache already.
    fn path_for_name(&mut self, name: &str, status: &mut dyn StatusBackend) -> OpenResult<PathBuf> {
        if let Some(info) = self.contents.get(name) {
            return match info.digest.create_two_part_path(&self.data_base) {
                Ok(p) => OpenResult::Ok(p),
                Err(e) => OpenResult::Err(e),
            };
        }

        // The file is not in the cache and we are asked not to try to fetch it.
        if self.only_cached {
            return OpenResult::NotAvailable;
        }

        let info = match self.index.get(name).cloned() {
            Some(info) => info,
            None => return OpenResult::NotAvailable,
        };

        // Bummer, we haven't seen this file before. We need to (try to) fetch
        // the item from the backend, saving it to disk and calculating its
        // digest ourselves, then enter it in the cache and in our manifest.
        // Fun times. Because we're touching the backend, we need to verify that
        // its digest is what we think.

        if let Err(e) = self.check_digest(status) {
            return OpenResult::Err(e);
        }

        // The bundle's overall digest is OK. Now try open the file. If it's
        // not available, cache that result, since LaTeX compilations commonly
        // touch nonexistent files. If we didn't maintain the negative cache,
        // we'd have to touch the network for virtually every compilation.

        let content = match get_file(
            &mut self.tar_data,
            name,
            info.offset,
            info.length as usize,
            status,
        ) {
            Ok(c) => c,
            Err(e) => return OpenResult::Err(e),
        };

        // OK, we can stream the file to a temporary location on disk,
        // computing its SHA256 as we go.

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
        // we're opening and closing this file every time we load a new file;
        // not so efficient, but whatever.

        if let Err(e) = self.record_cache_result(name, length as u64, digest) {
            return OpenResult::Err(e);
        }

        OpenResult::Ok(final_path)
    }
}

impl IoProvider for CachedITarBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let path = match self.path_for_name(name, status) {
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

impl Bundle for CachedITarBundle {
    fn get_digest(&mut self, _status: &mut dyn StatusBackend) -> Result<DigestData> {
        Ok(self.cached_digest)
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
    let mut f = atry!(File::create(path); ["couldn't open {} for writing",
                      path.display()]);
    atry!(write_fn(&mut f); ["couldn't write to {}", path.display()]);
    Ok(())
}

fn cache_dir(path: &str, custom_cache_root: Option<&Path>) -> Result<PathBuf> {
    if let Some(root) = custom_cache_root {
        if !root.is_dir() {
            bail!("Custom cache path {} is not a directory", root.display());
        }
        let full_path = root.join(path);
        atry!(fs::create_dir_all(&full_path); ["failed to create directory {}", full_path.display()]);
        Ok(full_path)
    } else {
        Ok(app_dirs::user_cache_dir(path).map_err(SyncError::new)?)
    }
}
