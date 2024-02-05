#![allow(nonstandard_style)]

pub type UErrorCode = libc::c_int;

pub const U_ZERO_ERROR: UErrorCode = 0;

pub fn U_SUCCESS(code: UErrorCode) -> bool {
    code <= U_ZERO_ERROR
}

#[repr(C)]
pub struct UConverter(());

extern "C" {
    pub fn ucnv_open(name: *const libc::c_char, err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_close(conv: *mut UConverter);
}
