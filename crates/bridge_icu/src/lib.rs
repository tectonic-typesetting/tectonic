// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

#![allow(nonstandard_style)]

//! This crate exists to export the ICU *C* API into the Cargo framework, as well as
//! provide bindings to other tectonic crates.

macro_rules! versioned_names {
    (
        $(
        pub fn $name:ident($($argname:ident: $argty:ty),* $(,)?) $(-> $output:ty)?;
        )+
    ) => {
        $(
        paste::paste! {
            #[link(name = "[< $name _ env!(\"ICU_MAJOR_VERSION\") >]")]
            pub fn $name($($argname: $argty),*) $(-> $output)?;
        }
        )*
    };
}

pub const UBIDI_DEFAULT_LTR: u8 = 0xFE;
pub const UBIDI_DEFAULT_RTL: u8 = 0xFF;
pub const U_ZERO_ERROR: UErrorCode = 0;

pub type UErrorCode = libc::c_int;
pub type UChar = u16;
pub type UChar32 = i32;

pub fn U_SUCCESS(code: UErrorCode) -> bool {
    code <= U_ZERO_ERROR
}

#[repr(C)]
pub struct UConverter(());

extern "C" {
    versioned_names! {
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
}
