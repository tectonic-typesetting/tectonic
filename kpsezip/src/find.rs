use libc;
use std::cmp::min;
use std::fs::File;
use std::io::{Error, Read, Result, Seek, Write};
use std::mem;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::path::Path;
use std::sync::Mutex;
use std::thread;
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


fn pipe() -> Result<(RawFd, RawFd)> {
    unsafe {
        let mut fds: [libc::c_int; 2] = mem::uninitialized();
        let res = libc::pipe(fds.as_mut_ptr());
        if res < 0 {
            Err(Error::last_os_error ())
        } else {
            Ok((fds[0], fds[1]))
        }
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
                    return None;
                }
                panic!("error reading bundle: {}", e);
            },
            Ok(f) => f
        };

        /* It is. Because we're terrible, we're going to read the whole thing,
         * then launch a thread to stream the file contents into a pipe! We
         * can return the pipe's FD to the C code, which can use libc's
         * fdopen() to create a FILE* that the existing code can use, none the
         * wiser of what's going on under the hood. */

        let mut buf = Vec::with_capacity (zipitem.size () as usize);
        if let Err(e) = zipitem.read_to_end (&mut buf) {
            panic!("error reading item {:?} in bundle: {}", name, e);
        }

        let fdpair = match pipe () {
            Err(e) => panic!("cannot create internal pipe: {}", e),
            Ok(p) => p
        };
        let reader_fd = fdpair.0;
        let writer_fd = fdpair.1;

        thread::spawn (move || {
            /* TODO: wanted to use io::copy, but didn't see a non-experimental
             * way to turn our bytes array into a Reader. `zipfile` is not
             * sendable (which makes sense) so we can't just use it. */

            const CHUNK_SIZE: usize = 4096;

            let mut w = unsafe { File::from_raw_fd (writer_fd) };
            let mut ofs = 0;
            let mut nleft = buf.len ();

            while nleft > 0{
                let n_to_write = min (CHUNK_SIZE, nleft);
                let n_written = match w.write (&buf[ofs .. ofs + n_to_write]) {
                    Ok(n) => n,
                    Err(_) => break,
                };
                ofs += n_written;
                nleft -= n_written;
            }
        });

        Some(reader_fd)
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
