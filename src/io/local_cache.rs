// src/io/local_cache.rs -- a local cache of files obtained from another IoProvider
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use crypto::digest::Digest;
use crypto::sha3;
use mkstemp;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::io::ErrorKind as IoErrorKind;
use std::path::{Path, PathBuf};

use errors::{ErrorKind, Result};
use io::{try_open_file, InputHandle, IoProvider, OpenResult};
use status::StatusBackend;


type Sha256Digest = [u8; 32];

// Note: can't impl ToString since we're an array type; could wrap it in a
// trivial struct.
fn digest_to_hex(digest: &Sha256Digest) -> String {
    digest
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .concat()
}

fn parse_digest_text(text: &str) -> Option<Sha256Digest> {
    if text.len() != 64 {
        return None;
    }

    let mut digest_buf: Sha256Digest = [0u8; 32];

    for i in 0..32 {
        if let Ok(v) = u8::from_str_radix(&text[i*2..(i+1)*2], 16) {
            digest_buf[i] = v;
        } else {
            return None;
        }
    }

    Some(digest_buf)
}


struct LocalCacheItem {
    _length: u64,
    digest: Option<Sha256Digest>, // None => negative cache: this file is not in the bundle
}

pub struct LocalCache<B: IoProvider> {
    backend: B,
    digest_path: PathBuf,
    cached_digest: Sha256Digest,
    checked_digest: bool,
    manifest_path: PathBuf,
    data_path: PathBuf,
    contents: HashMap<OsString,LocalCacheItem>,
}


impl<B: IoProvider> LocalCache<B> {
    pub fn new(mut backend: B, digest: &Path, manifest_base: &Path, data: &Path, status: &mut StatusBackend) -> Result<LocalCache<B>> {
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
                match backend.input_open_name(OsStr::new("SHA256SUM"), status) {
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

        let cached_digest = match parse_digest_text(&digest_text) {
            Some(d) => d,
            None => return Err(ErrorKind::Msg("corrupted SHA256 digest cache".to_owned()).into()),
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
            OpenResult::NotAvailable => {},
            OpenResult::Err(e) => { return Err(e.into()); },
            OpenResult::Ok(mfile) => {
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
                        match parse_digest_text(&bits[2]) {
                            None => continue,
                            some => some,
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
            data_path: data.to_owned(),
            contents: contents
        })
    }


    fn digest_to_path(&self, digest: &Sha256Digest) -> Result<PathBuf> {
        let mut p = self.data_path.clone();
        p.push(format!("{:02x}", digest[0]));
        fs::create_dir_all(&p)?;
        // NOTE: we're dropping the first two bytes here!
        p.push(&digest[1..].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().concat());
        Ok(p)
    }


    fn record_cache_result(&mut self, name: &OsStr, length: u64, digest: Option<Sha256Digest>) -> Result<()> {
        let digest_text = match digest {
            Some(ref d) => digest_to_hex(d),
            None => "-".to_owned(),
        };

        let mut man = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.manifest_path)?;

        writeln!(man, "{} {} {}", name.to_string_lossy(), length, digest_text)?;
        self.contents.insert(name.to_owned(), LocalCacheItem { _length: length, digest: digest });
        Ok(())
    }


    fn path_for_name(&mut self, name: &OsStr, status: &mut StatusBackend) -> OpenResult<PathBuf> {
        if let Some(info) = self.contents.get(name) {
            return match info.digest {
                None => OpenResult::NotAvailable,
                Some(ref d) => match self.digest_to_path(d) {
                    Ok(p) => OpenResult::Ok(p),
                    Err(e) => OpenResult::Err(e.into()),
                },
            };
        }

        // Bummer, we haven't seen this file before. We need to (try to) fetch
        // the item from the backend, saving it to disk and calculating its
        // digest ourselves, then enter it in the cache and in our manifest.
        // Fun times.
        //
        // First, if we're going to go to the backend, we should check that its
        // digest is what we expect. If not, we do a lame thing where we error
        // out but set things up so that things should succeed if the program
        // is re-run. Exactly the lame TeX user experience that I've been trying
        // to avoid!

        if !self.checked_digest {
            let dtext = match self.backend.input_open_name(OsStr::new("SHA256SUM"), status) {
                OpenResult::Ok(h) => {
                    let mut text = String::new();
                    if let Err(e) = h.take(64).read_to_string(&mut text) {
                        return OpenResult::Err(e.into());
                    }
                    text
                },
                OpenResult::NotAvailable => {
                    // Broken or un-cacheable backend.
                    return OpenResult::Err(ErrorKind::Msg("backend does not provide needed SHA256SUM file".to_owned()).into());
                },
                OpenResult::Err(e) => {
                    return OpenResult::Err(e.into());
                }
            };

            let current_digest = match parse_digest_text(&dtext) {
                Some(d) => d,
                None => {
                    return OpenResult::Err(ErrorKind::Msg("bad SHA256 digest from backend".to_owned()).into());
                },
            };

            if self.cached_digest != current_digest {
                // Crap! The backend isn't what we thought it was. Rewrite the
                // digest file so that next time we'll start afresh.

                match File::create(&self.digest_path) {
                    Ok(mut f) => {
                        let hexdigest = digest_to_hex(&current_digest);
                        if let Err(e) = writeln!(f, "{}", hexdigest) {
                            return OpenResult::Err(e.into());
                        }
                    },
                    Err(e) => {
                        // XXX this will be super confusing since we don't
                        // indicate that the error is related to the digest
                        // file, not the underlying file.
                        return OpenResult::Err(e.into());
                    }
                };

                return OpenResult::Err(ErrorKind::Msg("backend digest changed; rerun to use updated information".to_owned()).into());
            }

            // Phew, the backend hasn't changed. Don't check again.

            self.checked_digest = true;
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

        let mut digest_builder = sha3::Sha3::sha3_256();
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

        let mut digest: Sha256Digest = [0u8; 32];
        digest_builder.result(&mut digest);

        // Now we can move it to its final destination ..

        let final_path = match self.digest_to_path(&digest) {
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

        OpenResult::Ok(Box::new(BufReader::new(f)))
    }
}
