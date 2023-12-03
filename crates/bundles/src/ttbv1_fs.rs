// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! Read ttbv1 files on the filesystem

use crate::Bundle;
use flate2::read::GzDecoder;
use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::StatusBackend;

/// file info for TTbundle
#[derive(Clone, Debug)]
struct FileInfo {
    start: u64,
    length: u64,
    path: String,
    name: String,
    hash: Option<String>,
}

impl FileInfo {
    // Assuming this FileInfo is an index into `reader`,
    // build a reader that reads this file.
    pub(crate) fn read_from<'a, R: Read + Seek + 'a>(
        &self,
        reader: &'a mut R,
    ) -> Result<Box<dyn Read + 'a>> {
        reader.seek(SeekFrom::Start(self.start))?;
        return Ok(Box::new(GzDecoder::new(reader.take(self.length))));
    }
}

/// A bundle backed by a ZIP file.
pub struct Ttbv1FsBundle<R: Read + Seek> {
    reader: R,

    // Maps file names to (possibly many) paths.
    index: Vec<FileInfo>,
    search: Vec<String>,
}

/// The internal file-information struct used by the [`Ttbv1FsBundle`].

impl<R: Read + Seek> Ttbv1FsBundle<R> {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(reader: R) -> Result<Ttbv1FsBundle<R>> {
        /*
        reader.seek(SeekFrom::Start(0))?;

        // Parse ttb header
        let mut header = vec![0u8; 24];
        reader.read_exact(&mut header)?;
        let version = u64::from_le_bytes(header[0..8].try_into().unwrap());

        if version != 1 {
            bail!("wrong ttb version");
        }
        */

        Ok(Ttbv1FsBundle {
            search: Vec::new(),
            reader,
            index: Vec::new(),
        })
    }

    fn fill_index(&mut self) -> Result<()> {
        self.reader.seek(SeekFrom::Start(0))?;
        let mut header = vec![0u8; 24];
        self.reader.read_exact(&mut header)?;

        let reader = FileInfo {
            start: u64::from_le_bytes(header[8..16].try_into().unwrap()),
            length: u64::from_le_bytes(header[16..24].try_into().unwrap()),
            path: "/INDEX".to_owned(),
            name: "INDEX".to_owned(),
            hash: None,
        }
        .read_from(&mut self.reader)?;

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

        self.search = BufReader::new(info[0].read_from(&mut self.reader)?)
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

impl Ttbv1FsBundle<File> {
    /// Open a file on the filesystem as a zip bundle.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Ttbv1FsBundle<File>> {
        Self::new(File::open(path)?)
    }
}

impl<R: Read + Seek> IoProvider for Ttbv1FsBundle<R> {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        // Fetch index if it is empty
        if self.index.len() == 0 {
            match self.fill_index() {
                Err(e) => return OpenResult::Err(e.into()),
                _ => {}
            }

            match self.fill_search() {
                Err(e) => return OpenResult::Err(e.into()),
                _ => {}
            }
        }

        let info = match search_for_file(&self.index, &self.search, name) {
            Ok(None) => return OpenResult::NotAvailable,
            Err(e) => return OpenResult::Err(e.into()),
            Ok(Some(s)) => s,
        };

        let mut v: Vec<u8> = Vec::new();

        match info.read_from(&mut self.reader) {
            Err(e) => return OpenResult::Err(e.into()),
            Ok(mut b) => match b.read_to_end(&mut v) {
                Err(e) => return OpenResult::Err(e.into()),
                Ok(_) => {}
            },
        };

        return OpenResult::Ok(InputHandle::new_read_only(
            name,
            Cursor::new(v),
            InputOrigin::Other,
        ));
    }
}

impl<R: Read + Seek> Bundle for Ttbv1FsBundle<R> {
    fn all_files(&mut self, _status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        Ok(self.index.iter().map(|x| x.path.clone()).collect())
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
