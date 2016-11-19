use libc;
use mktemp::Temp;
use std::fs::File;
use std::io::{copy, Read, Seek};
use std::os::unix::io::{IntoRawFd, RawFd};
use std::path::Path;
use std::sync::Mutex;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

#[derive(Debug)]
pub enum FileFormat {
    TFM,
    Pict,
    Tex,
    Format,
}


pub fn c_format_to_rust (format: libc::c_int) -> Option<FileFormat> {
    // See the kpse_file_format_type enum.
    match format {
        3 => Some(FileFormat::TFM),
        10 => Some(FileFormat::Format),
        25 => Some(FileFormat::Pict),
        26 => Some(FileFormat::Tex),
        _ => None
    }
}


struct FinderState<R: Read + Seek> {
    zip: ZipArchive<R>
}

impl<R: Read + Seek> FinderState<R> {
    pub fn new (reader: R) -> ZipResult<FinderState<R>> {
        ZipArchive::new(reader).map (|zip|
            FinderState {
                zip: zip
            }
        )
    }

    pub fn get_readable_fd<'a> (&'a mut self, name: &'a Path, _: FileFormat, _: bool) -> Option<RawFd> {
        /* We currently don't care about the format or must_exist. */

        /* For now: if we can open straight off of the filesystem, do that. */
        if let Ok(f) = File::open (name) {
            return Some(f.into_raw_fd());
        }

        /* OK, let's see if it's in the bundle. */

        let mut zipitem = match self.zip.by_name (name.to_str ().unwrap ()) {
            Err(e) => {
                if let ZipError::FileNotFound = e {
                    println!("PKGW: failed to locate: {:?}", name);
                    return None;
                }
                panic!("error reading bundle: {}", e);
            },
            Ok(f) => f
        };

        /* It is. We extract the contents to a temporary file that we then
         * unlink. We do this because: (1) the format file is read in as a
         * gzip file, and the way that it is created requires that the file be
         * associated with a Unix file handle. But (2) the file must be
         * seekable, so we can't just use pipes. The temp file is unlinked at
         * the end of this function, but the open file handle keeps it around
         * for as long as the progam needs it. Yay Unix! */

        let temp_file = Temp::new_file ().unwrap ();
        {
            let mut f = File::create (temp_file.to_path_buf ()).unwrap ();
            copy (&mut zipitem, &mut f).unwrap ();
        }

        let f = File::open (temp_file.to_path_buf ()).unwrap ();
        Some(f.into_raw_fd ())
    }
}


// Finding files through the global singleton FinderState instance.

const BUNDLE_PATH: &'static str = "/a/texlive/testing.zip"; // whee.

lazy_static! {
    static ref SINGLETON: Mutex<FinderState<File>> = {
        let file = File::open(BUNDLE_PATH).unwrap ();
        Mutex::new(FinderState::new (file).unwrap ())
    };
}

pub fn get_readable_fd (name: &Path, format: FileFormat, must_exist: bool) -> Option<RawFd> {
    let mut s = SINGLETON.lock ().unwrap ();
    s.get_readable_fd (name, format, must_exist)
}
