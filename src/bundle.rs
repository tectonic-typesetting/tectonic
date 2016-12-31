// src/find.rs -- the file-finding layer used by the C code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.
//
// This all needs cleanup: we eventually want some kind of stackable set of
// layers that we investigated for files to read or write. But for now it gets
// the job done.

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::Path;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

use io::{InputHandle, IOProvider, OpenResult};


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


impl<R: Read + Seek> IOProvider for Bundle<R> {
    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        // We need to be able to look at other items in the Zip file while
        // reading this one, so the only path forward is to read the entire
        // contents into a buffer right now. RAM is cheap these days.

        // If `name` cannot be converted to Unicode, we return NotAvailable. I
        // *think* that's what we should do.

        let namestr = match name.to_str() {
            Some(s) => s,
            None => return OpenResult::NotAvailable
        };

        let mut zipitem = match self.zip.by_name (namestr) {
            Ok(f) => f,
            Err(e) => {
                return match e {
                    ZipError::Io(sube) => OpenResult::Err(sube.into()),
                    ZipError::FileNotFound => OpenResult::NotAvailable,
                    _ => OpenResult::Err(e.into()),
                }
            }
        };

        let mut buf = Vec::with_capacity(zipitem.size() as usize);

        if let Err(e) = zipitem.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(Box::new(Cursor::new(buf)))
    }
}
