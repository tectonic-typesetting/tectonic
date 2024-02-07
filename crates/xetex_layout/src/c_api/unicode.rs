#![allow(nonstandard_style)]

pub type UErrorCode = libc::c_int;
pub type UChar = u16;

pub const U_ZERO_ERROR: UErrorCode = 0;

pub fn U_SUCCESS(code: UErrorCode) -> bool {
    code <= U_ZERO_ERROR
}

#[repr(C)]
pub struct UConverter(());

extern "C" {
    pub fn ucnv_open(name: *const libc::c_char, err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_close(conv: *mut UConverter);
    pub fn ucnv_toUChars(
        conv: *mut UConverter,
        dest: *mut UChar,
        dest_capacity: i32,
        src: *const libc::c_char,
        src_len: i32,
        p_error_code: *mut UErrorCode,
    ) -> i32;
    pub fn ucnv_fromUChars(
        conv: *mut UConverter,
        dest: *mut libc::c_char,
        dest_capacity: i32,
        src: *const UChar,
        src_len: i32,
        p_error_code: *mut UErrorCode,
    ) -> i32;
}
