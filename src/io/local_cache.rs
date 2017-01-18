// src/io/local_cache.rs -- a local cache of files obtained from another IoProvider
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use crypto::digest::Digest;
use crypto::sha1;
use mkstemp;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use errors::Result;
use io::{InputHandle, IoProvider, OpenResult};


type Sha1Digest = [u8; 20];


struct LocalCacheItem {
    _length: u64,
    digest: Sha1Digest,
}

pub struct LocalCache<B: IoProvider> {
    backend: B,
    manifest_path: PathBuf,
    data_path: PathBuf,
    contents: HashMap<OsString,LocalCacheItem>,
}


impl<B: IoProvider> LocalCache<B> {
    pub fn new(backend: B, manifest: &Path, data: &Path) -> Result<LocalCache<B>> {
        let mut contents = HashMap::new();
        // TODO: HANDLE NONEXISTENT!
        let f = BufReader::new(File::open(manifest)?);
        let mut digest_buf: Sha1Digest = [0u8; 20];

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

            let digest_str = bits[2];
            if digest_str.len() != 40 {
                continue;
            }

            // There's surely a better way to do this, but whatever.
            let mut failed = false;

            for i in 0..20 {
                if let Ok(v) = u8::from_str_radix(&digest_str[i*2..(i+1)*2], 16) {
                    digest_buf[i] = v;
                } else {
                    failed = true;
                    break;
                }
            }

            if failed {
                continue;
            }

            contents.insert(name, LocalCacheItem { _length: length, digest: digest_buf.clone() });
        }

        Ok(LocalCache {
            backend: backend,
            manifest_path: manifest.to_owned(),
            data_path: data.to_owned(),
            contents: contents
        })
    }


    fn digest_to_path(&self, digest: &Sha1Digest) -> Result<PathBuf> {
        let mut p = self.data_path.clone();
        p.push(format!("{:02x}", digest[0]));
        fs::create_dir_all(&p)?;
        p.push(&digest[1..].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().concat());
        Ok(p)
    }


    fn path_for_name(&mut self, name: &OsStr) -> OpenResult<PathBuf> {
        if let Some(info) = self.contents.get(name) {
            return match self.digest_to_path(&info.digest) {
                Ok(p) => OpenResult::Ok(p),
                Err(e) => OpenResult::Err(e.into()),
            };
        }

        // Bummer, we haven't seen this file before. We need to (try to) fetch
        // the item from the backend, saving it to disk and calculating its
        // digest ourselves, then enter it in the cache and in our manifest.
        // Fun times.
        //
        // First, stream the file to a temporary location on disk, computing
        // its SHA1 as we go.

        let mut stream = match self.backend.input_open_name (name) {
            OpenResult::Ok(s) => s,
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
        };

        let mut digest_builder = sha1::Sha1::new();
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

            // XXX MAKE READONLY

            temp_dest.path().to_owned()
        };

        let mut digest: Sha1Digest = [0u8; 20];
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

        let hexdigest = &digest
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .concat();

        {
            let mut man = match fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.manifest_path) {
                    Ok(f) => f,
                    Err(e) => return OpenResult::Err(e.into())
                };

            if let Err(e) = writeln!(man, "{} {} {}", name.to_string_lossy(), length, hexdigest) {
                return OpenResult::Err(e.into());
            }
        }

        // Everything worked. Remember the file in our map.
        self.contents.insert(name.to_owned(), LocalCacheItem { _length: length as u64, digest: digest });
        OpenResult::Ok(final_path)
    }
}


impl<B: IoProvider> IoProvider for LocalCache<B> {
    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        let path = match self.path_for_name(name) {
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
