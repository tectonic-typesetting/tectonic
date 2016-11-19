use libc;
use std::fs::File;
use std::io::{Read, Seek};
use std::sync::Mutex;
use zip::result::ZipResult;
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

    pub fn find_file<'a> (&'a self, name: &'a [u8], format: FileFormat, must_exist: bool) -> Option<&[u8]> {
        Some(name)
    }
}


// Finding files through the global singleton FinderState instance.

const BUNDLE_PATH: &'static str = "/a/texlive/testing.zip"; // whee.

lazy_static! {
    static ref SINGLETON_LOCK: Mutex<u32> = Mutex::new(0);
    static ref SINGLETON: FinderState<File> = {
            let file = File::open(BUNDLE_PATH).unwrap ();
            FinderState::new (file).unwrap ()
    };
}

pub fn find_file (name: &[u8], format: FileFormat, must_exist: bool) -> Option<&[u8]> {
    let _g = SINGLETON_LOCK.lock ().unwrap ();
    SINGLETON.find_file (name, format, must_exist)
}
