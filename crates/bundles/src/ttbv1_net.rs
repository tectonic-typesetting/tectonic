// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! Read ttbv1 files on the filesystem

use flate2::read::GzDecoder;
use std::{
    convert::TryInto,
    io::{BufRead, BufReader},
    io::{Cursor, Read},
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::StatusBackend;

use crate::Bundle;

use tectonic_geturl::DefaultRangeReader;
use tectonic_geturl::{DefaultBackend, GetUrlBackend, RangeReader};

/// file info for TTbundle
#[derive(Clone, Debug)]
pub struct FileInfo {
    pub start: u64,
    pub length: u64,
    pub path: String,
    pub name: String,
    pub hash: Option<String>,
}

impl FileInfo {
    // Assuming this FileInfo is an index into `reader`,
    // build a reader that reads this file.
    pub(crate) fn read_from(&self, reader: &mut DefaultRangeReader) -> Box<dyn Read> {
        let stream = reader.read_range(self.start, self.length as usize).unwrap();
        return Box::new(GzDecoder::new(stream));
    }
}

/// A bundle backed by a ZIP file.
pub struct Ttbv1NetBundle {
    url: String,
    reader: Option<DefaultRangeReader>,

    // Maps file names to (possibly many) paths.
    index: Vec<FileInfo>,
    search: Vec<String>,
}

/// The internal file-information struct used by the [`Ttbv1NetBundle`].

impl Ttbv1NetBundle {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(url: String) -> Result<Ttbv1NetBundle> {
        Ok(Ttbv1NetBundle {
            search: Vec::new(),
            reader: None,
            index: Vec::new(),
            url,
        })
    }

    fn fill_index(&mut self) -> Result<()> {
        let mut header = vec![0u8; 24];
        let mut stream = self.reader.as_mut().unwrap().read_range(0, 24).unwrap();
        stream.read_exact(&mut header).unwrap();

        let reader = FileInfo {
            start: u64::from_le_bytes(header[8..16].try_into().unwrap()),
            length: u64::from_le_bytes(header[16..24].try_into().unwrap()),
            path: "/INDEX".to_owned(),
            name: "INDEX".to_owned(),
            hash: None,
        }
        .read_from(self.reader.as_mut().unwrap());

        for line in BufReader::new(reader).lines() {
            if let Ok(info) = Self::parse_index_line(&line?) {
                self.index.push(info);
            }
        }
        return Ok(());
    }

    fn fill_search(&mut self) -> Result<()> {
        let info: Vec<&FileInfo> = self.index.iter().filter(|x| x.name == "SEARCH").collect();
        if info.len() != 1 {
            bail!("bundle has invalid SEARCH specification");
        }

        self.search = BufReader::new(info[0].read_from(self.reader.as_mut().unwrap()))
            .lines()
            .collect::<Result<Vec<String>, std::io::Error>>()?;
        return Ok(());
    }

    /// Parse one line of index file
    fn parse_index_line(line: &str) -> Result<FileInfo> {
        let mut bits = line.split_whitespace();

        if let (Some(start), Some(length), Some(path), Some(hash)) =
            (bits.next(), bits.next(), bits.next(), bits.next())
        {
            let (_, name) = path.rsplit_once("/").unwrap();

            Ok(FileInfo {
                start: start.parse::<u64>()?,
                length: length.parse::<u64>()?,
                path: path.to_owned(),
                name: name.to_owned(),
                hash: match hash {
                    "nohash" => None,
                    _ => Some(hash.to_owned()),
                },
            })
        } else {
            // TODO: preserve the warning info or something!
            bail!("malformed index line");
        }
    }
}

impl IoProvider for Ttbv1NetBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        // Fetch index if it is empty
        if self.index.len() == 0 {
            let geturl_backend = DefaultBackend::default();

            // Connect reader if it is not already connected
            if self.reader.is_none() {
                self.reader = Some(geturl_backend.open_range_reader(&self.url));
            }

            self.fill_index().unwrap();
            self.fill_search().unwrap();
        }

        let info = match search_for_file(&self.index, &self.search, name) {
            Ok(None) => return OpenResult::NotAvailable,
            Err(e) => return OpenResult::Err(e.into()),
            Ok(Some(s)) => s,
        };

        let mut v: Vec<u8> = Vec::new();
        match info
            .read_from(self.reader.as_mut().unwrap())
            .read_to_end(&mut v)
        {
            Ok(_) => {}
            Err(e) => return OpenResult::Err(e.into()),
        };

        return OpenResult::Ok(InputHandle::new_read_only(
            name,
            Cursor::new(v),
            InputOrigin::Other,
        ));
    }
}

impl Bundle for Ttbv1NetBundle {
    fn all_files(&mut self, _status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        Ok(self.index.iter().map(|x| x.path.clone()).collect())
    }

    fn get_location(&mut self) -> String {
        return self.url.clone();
    }
}

fn search_for_file(
    index: &Vec<FileInfo>,
    search: &Vec<String>,
    name: &str,
) -> Result<Option<FileInfo>> {
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
    // The code below will clone these strings iff it has to.
    let infos: Vec<&FileInfo> = index.iter().filter(|x| x.name == n).collect();

    if relative_parent {
        // TODO: REWORK
        let mut matching: Option<FileInfo> = None;
        for info in infos {
            if info.path.ends_with(&name) {
                match matching {
                    Some(_) => {
                        bail!(
                            "found two files for string \"{}\". Please report this bug.",
                            name
                        );
                    }
                    None => matching = Some(info.to_owned()),
                }
            }
        }
        return Ok(matching);
    } else {
        // Even if paths.len() is 1, we don't return here.
        // We need to make sure this file matches a search path:
        // if it's in a directory we don't search, we shouldn't find it!

        let mut picked: Vec<&FileInfo> = Vec::new();
        for rule in search {
            for info in &infos {
                if rule.ends_with("//") {
                    // Match start of patent path
                    // (cutting off the last slash)
                    if info.path.starts_with(&rule[0..rule.len() - 1]) {
                        picked.push(info);
                    }
                } else {
                    // Match full parent path
                    if &info.path[0..info.path.len() - name.len()] == rule {
                        picked.push(info);
                    }
                }
            }
            if picked.len() != 0 {
                break;
            }
        }

        if picked.len() == 0 {
            // No file in our search dirs had this name.
            return Ok(None);
        } else if picked.len() == 1 {
            // We found exactly one file with this name.
            //
            // This chain of functions is essentially picked[0],
            // but takes ownership of the string without requiring
            // a .clone().
            return Ok(Some(picked[0].clone()));
        } else {
            // We found multiple files with this name, all of which
            // have the same priority. Pick alphabetically to emulate
            // an "alphabetic DFS" search order.
            picked.sort_by(|a, b| a.path.cmp(&b.path));
            return Ok(Some(picked[0].clone()));
        }
    }
}
