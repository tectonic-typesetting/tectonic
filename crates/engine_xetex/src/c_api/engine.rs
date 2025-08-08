use crate::ty::StrNumber;
use std::ffi::CStr;

pub fn with_tex_string<T>(s: StrNumber, f: impl FnOnce(&CStr) -> T) -> T {
    let ptr = unsafe { gettexstring(s) };
    let str = unsafe { CStr::from_ptr(ptr) };
    let out = f(str);
    unsafe { libc::free(ptr.cast()) };
    out
}

unsafe extern "C" {
    fn gettexstring(s: StrNumber) -> *mut libc::c_char;
}
