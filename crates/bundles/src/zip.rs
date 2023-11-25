// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! zip files as Tectonic bundles.

use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::{
    fs::File,
    io::{Cursor, Read, Seek},
    path::Path,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::tt_error;
use tectonic_status_base::StatusBackend;
use zip::{result::ZipError, ZipArchive};

use crate::Bundle;

/// A bundle backed by a ZIP file.
pub struct ZipBundle<R: Read + Seek> {
    zip: ZipArchive<R>,

    // Maps file names to (possibly many) paths.
    index: HashMap<String, Vec<String>>,

    search: Vec<String>,
}

impl<R: Read + Seek> ZipBundle<R> {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(reader: R) -> Result<ZipBundle<R>> {
        let mut zip = ZipArchive::new(reader)?;
        let mut index = HashMap::new();
        let mut search = Vec::new();

        // Extract and read index
        match zip.by_name("INDEX") {
            Ok(f) => {
                for line in BufReader::new(f).lines() {
                    Self::add_index_line(&line?, &mut index)?;
                }
            }
            Err(e) => return Err(e.into()),
        };

        // Extract and read index
        match zip.by_name("SEARCH") {
            Ok(f) => {
                for line in BufReader::new(f).lines() {
                    Self::add_search_line(&line?, &mut search)?;
                }
            }
            Err(e) => return Err(e.into()),
        };

        Ok(ZipBundle { zip, index, search })
    }

    /// Parse one line of an index file
    fn add_index_line(line: &str, index: &mut HashMap<String, Vec<String>>) -> Result<()> {
        let mut bits = line.split_whitespace();

        if let (Some(name), Some(path)) = (bits.next(), bits.next()) {
            let v = index.entry(name.to_string()).or_insert(Vec::new());
            v.push(path.to_string());
        } else {
            bail!("malformed index line");
        }

        return Ok(());
    }

    /// Parse one line of an index file
    fn add_search_line(line: &str, search: &mut Vec<String>) -> Result<()> {
        let mut bits = line.split_whitespace();

        if let Some(path) = bits.next() {
            search.push(path.to_string());
        } else {
            bail!("malformed search priority line");
        }

        return Ok(());
    }

    // Turn a name into a path
    fn find_name(&self, name: &str, status: &mut dyn StatusBackend) -> Option<String> {
        // Get last element of path, since
        // some packages reference a path to a file.
        // `fithesis4` is one example.
        let relative_parent: bool;
        let n = match name.rsplit_once("/") {
            Some(n) => {
                relative_parent = true;
                n.1
            }
            None => {
                relative_parent = false;
                name
            }
        };

        // If we don't have this path in the index, this file doesn't exist.
        // Make sure the bundle script adds ALL files to the index!
        //
        // The code below takes care to clone these strings only if it needs to.
        let paths: &Vec<String> = match self.index.get(n) {
            Some(s) => s,
            None => return None,
        };

        if relative_parent {
            let mut matching: Option<String> = None;
            for p in paths {
                if p.ends_with(&name) {
                    match matching {
                        Some(_) => {
                            tt_error!(
                                status,
                                "found two files for string \"{}\". Please report this bug.",
                                name
                            );
                        }
                        None => matching = Some(p.to_string()),
                    }
                }
            }
            return matching;
        } else {
            if paths.len() == 1 {
                return Some(paths[0].to_string());
            }

            let mut picked: Vec<String> = Vec::new();
            for rule in &self.search {
                for path in paths {
                    if rule.ends_with("//") {
                        // Match start of patent path
                        // (cutting off the last slash from)
                        if path.starts_with(&rule[0..rule.len() - 1]) {
                            picked.push(path.clone());
                        }
                    } else {
                        // Match full parent path
                        if &path[0..path.len() - name.len()] == rule {
                            picked.push(path.clone());
                        }
                    }
                }
                if picked.len() != 0 {
                    break;
                }
            }

            if picked.len() == 0 {
                // We didn't match any lines in the search file
                picked = paths.clone()
            } else if picked.len() == 1 {
                return Some(picked.into_iter().next().unwrap());
            }

            // If we haven't resolved the conflict yet, choose alphabetically.
            picked.sort();
            return Some(picked.into_iter().next().unwrap());
        }
    }
}

impl ZipBundle<File> {
    /// Open a file on the filesystem as a zip bundle.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ZipBundle<File>> {
        Self::new(File::open(path)?)
    }
}

impl<R: Read + Seek> IoProvider for ZipBundle<R> {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let path = match self.find_name(name, status) {
            Some(s) => s,
            None => return OpenResult::NotAvailable,
        };

        let mut zipitem = match self.zip.by_name(&path) {
            Ok(f) => f,
            Err(e) => {
                return match e {
                    ZipError::Io(sube) => OpenResult::Err(sube.into()),
                    ZipError::FileNotFound => OpenResult::NotAvailable,
                    _ => OpenResult::Err(e.into()),
                };
            }
        };

        // We need to be able to look at other items in the Zip file while
        // reading this one, so the only path forward is to read the entire
        // contents into a buffer right now. RAM is cheap these days.
        let mut buf = Vec::with_capacity(zipitem.size() as usize);

        if let Err(e) = zipitem.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(InputHandle::new_read_only(
            name,
            Cursor::new(buf),
            InputOrigin::Other,
        ))
    }
}

impl<R: Read + Seek> Bundle for ZipBundle<R> {
    fn all_files(&mut self, _status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        // This is almost good, but we don't want directories to be listed.
        Ok(self.zip.file_names().map(|s| s.to_owned()).collect())
    }
}
