// src/find.rs -- the file-finding layer used by the C code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.
//
// This all needs cleanup: we eventually want some kind of stackable set of
// layers that we investigated for files to read or write. But for now it gets
// the job done.

use mktemp::Temp;
use std::ffi::OsString;
use std::fs::File;
use std::io::{copy, stderr, Cursor, Read, Seek, Write};
use std::os::unix::io::{IntoRawFd, RawFd};
use std::path::{Path, PathBuf};
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

use file_format::{format_to_extension, FileFormat};


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
        let zipitem = self.zip.by_name (name.to_str ().unwrap ())?;
        Ok(Cursor::new(Vec::with_capacity(zipitem.size() as usize)))
    }

    fn zip_to_temp_fd (&mut self, name: &Path) -> ZipResult<RawFd> {
        let mut zipitem = self.zip.by_name (name.to_str ().unwrap ())?;

        let temp_file = Temp::new_file ().unwrap ();
        {
            let mut f = File::create (temp_file.to_path_buf ()).unwrap ();
            copy (&mut zipitem, &mut f).unwrap ();
        }

        let f = File::open (temp_file.to_path_buf ()).unwrap ();
        Ok(f.into_raw_fd ())
    }

    pub fn get_readable_fd<'a> (&'a mut self, name: &'a Path, format: FileFormat, _: bool) -> Option<RawFd> {
        /* See if a file's in the bundle. If so, we need to extract the
         * contents to a temporary file that we then unlink, because: (1) the
         * format file is read in as a gzip file, and the way that it is
         * created requires that the file be associated with a Unix file
         * handle. But (2) the file must be seekable, so we can't just use
         * pipes. The temp file is unlinked at the end of this function, but
         * the open file handle keeps it around for as long as the progam
         * needs it. Yay Unix!
         *
         * We need to use the zip_to_temp_fd helper because the first ZipResult
         * we look at keeps a mutable borrow on the ZipArchive.
         */

        let mut ext = PathBuf::from (name); // XXX redundant code
        let mut ename = OsString::from (ext.file_name ().unwrap ());
        ename.push (format_to_extension (format));
        ext.set_file_name (ename);

        if let Ok(fd) = self.zip_to_temp_fd (name) {
            return Some(fd);
        }

        return match self.zip_to_temp_fd (&ext) {
            Err(e) => {
                if let ZipError::FileNotFound = e {
                    writeln!(&mut stderr(), "PKGW: failed to locate: {:?}", name).expect ("stderr failed");
                    None
                } else {
                    panic!("error reading bundle: {}", e)
                }
            },
            Ok(fd) => Some(fd)
        };
    }
}


impl Bundle<File> {
    pub fn open (path: &Path) -> ZipResult<Bundle<File>> {
        let file = File::open(path)?;
        Self::new(file)
    }
}
