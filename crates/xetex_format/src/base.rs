// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Truly basic definitions relating to the XeTeX engine.
//!
//! We don't expect these types and constants to ever change.

use byteorder::ByteOrder;

/// The `byteorder` endianness of Tectonic format files.
pub use byteorder::BigEndian as FormatEndian;

/// The number of Unicode Scalar Values.
pub const NUMBER_USVS: usize = 0x11_0000;

/// The number of basic TeX register.
pub const NUMBER_REGS: usize = 256;

/// The number of TeX math fonts.
pub const NUMBER_MATH_FONTS: usize = 3 * 256;

/// The number of bytes in a TeX "memory word" variable.
pub const SIZEOF_MEMORY_WORD: usize = 8;

/// The minimum allowed value of a TeX "halfword" variable
pub const MIN_HALFWORD: i32 = -0x0FFF_FFFF; // = -268,435,455 = 0xF000_0001

/// The maximum allowed value of a TeX "halfword" variable
pub const MAX_HALFWORD: i32 = 0x3FFF_FFFF; // = 1,073,741,823

/// The value of a "null" memory pointer in TeX.
pub const TEX_NULL: i32 = MIN_HALFWORD;

/// Read the value of the `b16.s0` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.u.B1` field in WEB2C or `hh.b1` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_read_b16_s0(arr: &[u8], index: i32) -> i16 {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::read_i16(&arr[i + 6..i + 8])
}

/// Read the value of the `b16.s1` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.u.B0` field in WEB2C or `hh.b0` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_read_b16_s1(arr: &[u8], index: i32) -> i16 {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::read_i16(&arr[i + 4..i + 6])
}

/// Read the value of the `b32.s0` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.v.LH` field in WEB2C or `hh.lh` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_read_b32_s0(arr: &[u8], index: i32) -> i32 {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::read_i32(&arr[i + 4..i + 8])
}

/// Read the *value* of the `b32.s1` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.v.RH` field in WEB2C or `hh.rh` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_read_b32_s1(arr: &[u8], index: i32) -> i32 {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::read_i32(&arr[i..i + 4])
}

/// Write *value* to the `b16.s0` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.u.B1` field in WEB2C or `hh.b1` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_write_b16_s0(arr: &mut [u8], index: i32, value: i16) {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::write_i16(&mut arr[i + 6..i + 8], value);
}

/// Write *value* to the `b16.s1` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.u.B0` field in WEB2C or `hh.b0` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_write_b16_s1(arr: &mut [u8], index: i32, value: i16) {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::write_i16(&mut arr[i + 4..i + 6], value);
}

/// Write *value* to the `b32.s0` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.v.LH` field in WEB2C or `hh.lh` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_write_b32_s0(arr: &mut [u8], index: i32, value: i32) {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::write_i32(&mut arr[i + 4..i + 8], value);
}

/// Write *value* to the `b32.s1` field of the *index*'th  word in *arr*.
///
/// This is also known as the `hh.v.RH` field in WEB2C or `hh.rh` in WEB. Note
/// that the index is counted in "memory word" units, e.g. 8 bytes, not single
/// bytes.
#[inline(always)]
pub fn memword_write_b32_s1(arr: &mut [u8], index: i32, value: i32) {
    let i = index as usize * SIZEOF_MEMORY_WORD;
    FormatEndian::write_i32(&mut arr[i..i + 4], value);
}
