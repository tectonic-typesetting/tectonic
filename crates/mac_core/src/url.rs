use super::{sys, CoreType};
use std::ffi::{CStr, CString};

cfty! {
    /// A URL value
    CFURL CFUrl : CFURLGetTypeID
}

impl CFUrl {
    /// If possible, returns the native file system representation of the path identified by this
    /// URL. Returns `None` if the URL cannot be represented as a filesystem path.
    pub fn fs_representation(&self) -> Option<CString> {
        let mut buf = [0u8; libc::PATH_MAX as usize];
        // SAFETY: Self is guaranteed valid, buf length is guaranteed to match allocated buffer.
        let res = unsafe {
            sys::CFURLGetFileSystemRepresentation(
                self.as_type_ref(),
                true,
                buf.as_mut_ptr(),
                buf.len() as sys::CFIndex,
            )
        };
        if res {
            CStr::from_bytes_until_nul(&buf).ok().map(CStr::to_owned)
        } else {
            None
        }
    }
}
