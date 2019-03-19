// src/io/local_cache.rs -- a local cache of files obtained from another IoProvider
// Copyright 2017-2018 the Tectonic Project
// Licensed under the MIT License.

use fs2::FileExt;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::ErrorKind as IoErrorKind;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tempfile;

use super::{try_open_file, Bundle, InputHandle, InputOrigin, IoProvider, OpenResult};
use digest::{self, Digest, DigestData};
use errors::{ErrorKind, Result};
use status::StatusBackend;

struct LocalCacheItem {
    _length: u64,
    digest: Option<DigestData>, // None => negative cache: this file is not in the bundle
}

pub struct LocalCache<B: Bundle> {
    backend: B,
    digest_path: PathBuf,
    cached_digest: DigestData,
    checked_digest: bool,
    manifest_path: PathBuf,
    data_path: PathBuf,
    contents: HashMap<OsString, LocalCacheItem>,
    only_cached: bool,
}

impl<B: Bundle> LocalCache<B> {
    pub fn new(
        mut backend: B,
        digest: &Path,
        manifest_base: &Path,
        data: &Path,
        only_cached: bool,
        status: &mut StatusBackend,
    ) -> Result<LocalCache<B>> {
        // If the `digest` file exists, we assume that it is valid; this is
        // *essential* so that we can use a URL as our default IoProvider
        // without requiring a network connection to run. If it does not
        // exist, we need to query the backend.

        let (digest_text, cached_digest, checked_digest) = match File::open(digest) {
            Ok(f) => {
                let mut text = String::new();
                f.take(64).read_to_string(&mut text)?;
                let cached_digest =
                    ctry!(DigestData::from_str(&text); "corrupted SHA256 digest cache");
                (text, cached_digest, false)
            }

            Err(e) => {
                if e.kind() != IoErrorKind::NotFound {
                    // Unexpected error reading the digest cache file. Ruh roh!
                    return Err(e.into());
                }

                // Digest file just doesn't exist. We need to query the backend for it.
                let cached_digest =
                    ctry!(backend.get_digest(status); "could not get backend summary digest");
                let text = cached_digest.to_string();
                (text, cached_digest, true)
            }
        };

        if checked_digest {
            // If checked_digest is true, the digest cache file did not exist
            // and we got the text fresh from the backend. So, we should write
            // it out to the cache file.
            let mut f = File::create(&digest)?;
            writeln!(f, "{}", digest_text)?;
        }

        // We can now figure out which manifest to use.

        let mut manifest_path = manifest_base.to_owned();
        manifest_path.push(&digest_text);
        manifest_path.set_extension("txt");

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

                    let name = OsString::from(original_name);

                    let length = match length.parse::<u64>() {
                        Ok(l) => l,
                        Err(_) => continue,
                    };

                    let digest = if digest == "-" {
                        None
                    } else {
                        match DigestData::from_str(&digest) {
                            Ok(d) => Some(d),
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

        Ok(LocalCache {
            backend,
            digest_path: digest.to_owned(),
            cached_digest,
            checked_digest,
            manifest_path,
            data_path: data.to_owned(),
            contents,
            only_cached,
        })
    }

    fn record_cache_result(
        &mut self,
        name: &OsStr,
        length: u64,
        digest: Option<DigestData>,
    ) -> Result<()> {
        let digest_text = match digest {
            Some(ref d) => d.to_string(),
            None => "-".to_owned(),
        };

        // Due to a quirk about permissions for file locking on Windows, we
        // need to add `.read(true)` to be able to lock a file opened in
        // append mode.

        let mut man = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open(&self.manifest_path)?;

        // Lock will be released when file is closed at the end of this function.
        ctry!(man.lock_exclusive(); "failed to lock manifest file \"{}\" for writing", self.manifest_path.display());

        if let Some(name_utf8) = name.to_str() {
            if !name_utf8.contains(|c| c == '\n' || c == '\r') {
                writeln!(man, "{} {} {}", name_utf8, length, digest_text)?;
            }
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
    fn check_digest(&mut self, status: &mut StatusBackend) -> Result<()> {
        if self.checked_digest {
            return Ok(());
        }

        let dtext = match self
            .backend
            .input_open_name(OsStr::new("SHA256SUM"), status)
        {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                ctry!(h.take(64).read_to_string(&mut text); "error reading {}", self.digest_path.to_string_lossy());
                text
            }
            OpenResult::NotAvailable => {
                // Broken or un-cacheable backend.
                return Err(ErrorKind::Msg(
                    "backend does not provide needed SHA256SUM file".to_owned(),
                )
                .into());
            }
            OpenResult::Err(e) => {
                return Err(e);
            }
        };

        let current_digest = ctry!(DigestData::from_str(&dtext); "bad SHA256 digest from backend");

        if self.cached_digest != current_digest {
            // Crap! The backend isn't what we thought it was. Rewrite the
            // digest file so that next time we'll start afresh.

            let mut f = ctry!(File::create(&self.digest_path); "couldn\'t open {} for writing",
                              self.digest_path.to_string_lossy());
            ctry!(writeln!(f, "{}", current_digest.to_string()); "couldn\'t write to {}",
                  self.digest_path.to_string_lossy());
            return Err(ErrorKind::Msg(
                "backend digest changed; rerun to use updated information".to_owned(),
            )
            .into());
        }

        // Phew, the backend hasn't changed. Don't check again.
        self.checked_digest = true;
        Ok(())
    }

    fn path_for_name(&mut self, name: &OsStr, status: &mut StatusBackend) -> OpenResult<PathBuf> {
        if let Some(info) = self.contents.get(name) {
            return match info.digest {
                None => OpenResult::NotAvailable,
                Some(ref d) => match d.create_two_part_path(&self.data_path) {
                    Ok(p) => OpenResult::Ok(p),
                    Err(e) => OpenResult::Err(e),
                },
            };
        }

        // The file is not in the cache and we are asked not to try to fetch it.
        if self.only_cached {
            return OpenResult::NotAvailable;
        }

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

        let mut stream = match self.backend.input_open_name(name, status) {
            OpenResult::Ok(s) => s,
            OpenResult::Err(e) => return OpenResult::Err(e),
            OpenResult::NotAvailable => {
                if let Err(e) = self.record_cache_result(name, 0, None) {
                    return OpenResult::Err(e);
                }
                return OpenResult::NotAvailable;
            }
        };

        // OK, we can stream the file to a temporary location on disk,
        // computing its SHA256 as we go.

        let mut digest_builder = digest::create();
        let mut length = 0;

        let mut temp_dest = match tempfile::Builder::new()
            .prefix("download_")
            .rand_bytes(6)
            .tempfile_in(&self.data_path)
        {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into()),
        };

        let mut buf = [0u8; 8192];

        while let Ok(nbytes) = stream.read(&mut buf) {
            if nbytes == 0 {
                break;
            }

            length += nbytes;
            let chunk = &buf[..nbytes];

            digest_builder.input(chunk);
            if let Err(e) = temp_dest.write_all(chunk) {
                return OpenResult::Err(e.into());
            }
        }

        let digest = DigestData::from(digest_builder);

        // Now we can almost move it to its final destination ..

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
            if let Err(e) = temp_dest.persist(&final_path) {
                return OpenResult::Err(e.error.into());
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

        if let Err(e) = self.record_cache_result(name, length as u64, Some(digest)) {
            return OpenResult::Err(e);
        }

        OpenResult::Ok(final_path)
    }
}

impl<B: Bundle> IoProvider for LocalCache<B> {
    fn input_open_name(
        &mut self,
        name: &OsStr,
        status: &mut StatusBackend,
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

        OpenResult::Ok(InputHandle::new(
            name,
            BufReader::new(f),
            InputOrigin::Other,
        ))
    }
}

impl<B: Bundle> Bundle for LocalCache<B> {
    fn get_digest(&mut self, _status: &mut StatusBackend) -> Result<DigestData> {
        Ok(self.cached_digest)
    }
}
