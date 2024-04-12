use super::{sys, CoreType};
use std::ffi::{CStr, CString};

cfty! {
    CFURL CFUrl : CFURLGetTypeID
}

impl CFUrl {
    pub fn fs_representation(&self) -> Option<CString> {
        let mut buf = [0u8; libc::PATH_MAX as usize];
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
