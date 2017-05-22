// src/io/local_cache.rs -- a local cache of files obtained from another IoProvider
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use fs2::FileExt;
use mkstemp;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::io::ErrorKind as IoErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use digest::{self, Digest, DigestData};
use errors::{ErrorKind, Result, ResultExt};
use super::{try_open_file, InputHandle, InputOrigin, IoProvider, OpenResult};
use status::StatusBackend;


struct LocalCacheItem {
    _length: u64,
    digest: Option<DigestData>, // None => negative cache: this file is not in the bundle
}

pub struct LocalCache<B: IoProvider> {
    backend: B,
    digest_path: PathBuf,
    cached_digest: DigestData,
    checked_digest: bool,
    manifest_path: PathBuf,
    formats_base: PathBuf,
    data_path: PathBuf,
    contents: HashMap<OsString,LocalCacheItem>,
}


impl<B: IoProvider> LocalCache<B> {
    pub fn new(mut backend: B, digest: &Path, manifest_base: &Path, formats_base: &Path,
               data: &Path, status: &mut StatusBackend) -> Result<LocalCache<B>> {
        // If the `digest` file exists, we assume that it is valid; this is
        // *essential* so that we can use a URL as our default IoProvider
        // without requiring a network connection to run. If it does not
        // exist, we need to query the backend.

        let mut checked_digest = false;

        let digest_text = match File::open(digest) {
            Ok(f) => {
                let mut text = String::new();
                f.take(64).read_to_string(&mut text)?;
                text
            },
            Err(e) => {
                if e.kind() != IoErrorKind::NotFound {
                    // Unexpected error reading the digest cache file. Ruh roh!
                    return Err(e.into());
                }

                // OK, digest file just doesn't exist. We need to query the backend for it.
                match backend.input_open_name(OsStr::new(digest::DIGEST_NAME), status) {
                    OpenResult::Ok(h) => {
                        // Phew, the backend has the info we need.
                        let mut text = String::new();
                        h.take(64).read_to_string(&mut text)?;
                        checked_digest = true;
                        text
                    },
                    OpenResult::NotAvailable => {
                        // Broken or un-cacheable backend.
                        return Err(ErrorKind::Msg("backend does not provide needed SHA256SUM file".to_owned()).into());
                    },
                    OpenResult::Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        };

        let cached_digest = ctry!(DigestData::from_str(&digest_text); "corrupted SHA256 digest cache");

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
            OpenResult::NotAvailable => {},
            OpenResult::Err(e) => { return Err(e.into()); },
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
                    let bits = line.split_whitespace().collect::<Vec<_>>();

                    if bits.len() < 3 {
                        continue; // TODO: warn or something?
                    }

                    let name = OsString::from(bits[0]);

                    let length = match bits[1].parse::<u64>() {
                        Ok(l) => l,
                        Err(_) => continue
                    };

                    let digest = if bits[2] == "-" {
                        None
                    } else {
                        match DigestData::from_str(&bits[2]) {
                            Ok(d) => Some(d),
                            Err(e) => {
                                tt_warning!(status, "ignoring bad digest data \"{}\" for \"{}\" in \"{}\"",
                                            &bits[2], bits[0], manifest_path.display() ; e);
                                continue;
                            }
                        }
                    };

                    contents.insert(name, LocalCacheItem { _length: length, digest: digest });
                }
            }
        }

        // All set.

        Ok(LocalCache {
            backend: backend,
            digest_path: digest.to_owned(),
            cached_digest: cached_digest,
            checked_digest: checked_digest,
            manifest_path: manifest_path,
            formats_base: formats_base.to_owned(),
            data_path: data.to_owned(),
            contents: contents
        })
    }


    fn record_cache_result(&mut self, name: &OsStr, length: u64, digest: Option<DigestData>) -> Result<()> {
        let digest_text = match digest {
            Some(ref d) => d.to_string(),
            None => "-".to_owned(),
        };

        let mut man = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.manifest_path)?;

        // Lock will be released when file is closed at the end of this function.
        ctry!(man.lock_exclusive(); "failed to lock manifest file \"{}\" for writing", self.manifest_path.display());

        writeln!(man, "{} {} {}", name.to_string_lossy(), length, digest_text)?;
        self.contents.insert(name.to_owned(), LocalCacheItem { _length: length, digest: digest });
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

        let dtext = match self.backend.input_open_name(OsStr::new("SHA256SUM"), status) {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                ctry!(h.take(64).read_to_string(&mut text); "error reading {}", self.digest_path.to_string_lossy());
                text
            },
            OpenResult::NotAvailable => {
                // Broken or un-cacheable backend.
                return Err(ErrorKind::Msg("backend does not provide needed SHA256SUM file".to_owned()).into());
            },
            OpenResult::Err(e) => {
                return Err(e.into());
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
            return Err(ErrorKind::Msg("backend digest changed; rerun to use updated information".to_owned()).into());
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
                    Err(e) => OpenResult::Err(e.into()),
                },
            };
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

        let mut stream = match self.backend.input_open_name (name, status) {
            OpenResult::Ok(s) => s,
            OpenResult::Err(e) => return OpenResult::Err(e),
            OpenResult::NotAvailable => {
                if let Err(e) = self.record_cache_result(name, 0, None) {
                    return OpenResult::Err(e.into());
                }
                return OpenResult::NotAvailable;
            }
        };

        // OK, we can stream the file to a temporary location on disk,
        // computing its SHA256 as we go.

        let mut digest_builder = digest::create();
        let mut length = 0;

        let temp_path = {
            let mut templ = self.data_path.clone();
            templ.push("download_XXXXXX");

            let mut temp_dest = match mkstemp::TempFile::new(&templ.to_string_lossy(), false) {
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

            // XXX MAKE READONLY -- the ideal approach depends on unstable APIs

            temp_dest.path().to_owned()
        };

        let digest = DigestData::from(digest_builder);

        // Now we can move it to its final destination ..

        let final_path = match digest.create_two_part_path(&self.data_path) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e.into()),
        };

        if let Err(e) = fs::rename(&temp_path, &final_path) {
            return OpenResult::Err(e.into());
        }

        // And finally add a record of this file to our manifest. Note that
        // we're opening and closing this file every time we load a new file;
        // not so efficient, but whatever.

        if let Err(e) = self.record_cache_result(name, length as u64, Some(digest)) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(final_path)
    }


    /// Get an on-disk path name for a given format file. Unlike
    /// path_for_name(), we do *not* do any fetching or caching; this simply
    /// produces a path that may or may not exist. We rely on the cached
    /// digest and do *not* call check_digest() because otherwise we'd need to
    /// open up the backend whenever we wanted to open a format file, breaking
    /// network-free operation.
    fn path_for_format(&mut self, name: &OsStr) -> Result<PathBuf> {
        // Remove all extensions from the format name. PathBuf.file_stem() doesn't
        // do what we want since it only strips one extension, so here we go:

        let stem = match name.to_str().and_then(|s| s.splitn(2, ".").next()) {
            Some(s) => s,
            None => {
                return Err(ErrorKind::Msg(format!("incomprehensible format file name \"{}\"",
                                                  name.to_string_lossy())).into());
            }
        };

        let mut p = self.formats_base.clone();
        p.push(format!("{}-{}-{}.fmt.gz", self.cached_digest.to_string(), stem, ::FORMAT_SERIAL));
        Ok(p)
    }
}


impl<B: IoProvider> IoProvider for LocalCache<B> {
    fn input_open_name(&mut self, name: &OsStr, status: &mut StatusBackend) -> OpenResult<InputHandle> {
        let path = match self.path_for_name(name, status) {
            OpenResult::Ok(p) => p,
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
        };

        let f = match File::open (&path) {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into())
        };

        OpenResult::Ok(InputHandle::new(name, BufReader::new(f), InputOrigin::Other))
    }


    fn input_open_format(&mut self, name: &OsStr, _status: &mut StatusBackend) -> OpenResult<InputHandle> {
        let path = match self.path_for_format(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e.into()),
        };

        let f = match super::try_open_file(&path) {
            OpenResult::Ok(f) => f,
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
        };

        OpenResult::Ok(InputHandle::new(name, BufReader::new(f), InputOrigin::Other))
    }


    fn write_format(&mut self, name: &str, data: &[u8], _status: &mut StatusBackend) -> Result<()> {
        let final_path = self.path_for_format(OsStr::new(name))?;

        let mut templ = self.formats_base.clone();
        templ.push("format_XXXXXX");

        let temp_path = {
            let mut temp_dest = mkstemp::TempFile::new(&templ.to_string_lossy(), false)?;
            temp_dest.write_all(data)?;
            temp_dest.path().to_owned()
        };

        fs::rename(&temp_path, &final_path).map_err(|e| e.into())
    }
}
