// src/find.rs -- the file-finding layer used by the C code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.
//
// This all needs cleanup: we eventually want some kind of stackable set of
// layers that we investigated for files to read or write. But for now it gets
// the job done.

use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::Path;
use zip::result::ZipResult;
use zip::ZipArchive;


pub struct Bundle<R: Read + Seek> {
    zip: ZipArchive<R>
}


impl<R: Read + Seek> Bundle<R> {
    pub fn new (reader: R) -> ZipResult<Bundle<R>> {
        ZipArchive::new(reader).map (|zip|
            Bundle {
                zip: zip
            }
        )
    }

    pub fn get_buffer(&mut self, name: &Path) -> ZipResult<Cursor<Vec<u8>>> {
        let mut zipitem = self.zip.by_name (name.to_str ().unwrap ())?;
        let mut buf = Vec::with_capacity(zipitem.size() as usize);
        zipitem.read_to_end(&mut buf)?;
        Ok(Cursor::new(buf))
    }
}


impl Bundle<File> {
    pub fn open (path: &Path) -> ZipResult<Bundle<File>> {
        let file = File::open(path)?;
        Self::new(file)
    }
}
