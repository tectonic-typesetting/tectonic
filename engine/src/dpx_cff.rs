#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::warn;

extern crate libc;
use libc::free;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn cff_release_dict(dict: *mut cff_dict);
    #[no_mangle]
    fn cff_dict_get(dict: *mut cff_dict, key: *const i8, idx: i32) -> f64;
    #[no_mangle]
    fn cff_dict_known(dict: *mut cff_dict, key: *const i8) -> i32;
    /* decode/encode DICT */
    #[no_mangle]
    fn cff_dict_unpack(data: *mut card8, endptr: *mut card8) -> *mut cff_dict;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* CFF Data Types */
/* SID SID number */
/* offset(0) */
/* size offset(0) */
pub type card8 = u8;
/* 1-byte unsigned number */
pub type card16 = u16;
/* 2-byte unsigned number */
pub type c_offsize = u8;
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
pub type l_offset = u32;
/* 1, 2, 3, or 4-byte offset */
pub type s_SID = u16;
/* 2-byte string identifier  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
    /* Object data                       */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
    /* Absolute offset (0) size             */
}
/* Dictionary */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: i32,
    pub key: *const i8,
    pub count: i32,
    pub values: *mut f64,
    /* values                                  */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: i32,
    pub count: i32,
    pub entries: *mut cff_dict_entry,
}
/* Encoding, Charset and FDSelect */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
    /* no. of remaining gids/codes in this range */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
    /* card16-version of range1 */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed,
    pub num_supps: card8,
    pub supp: *mut cff_map,
    /* supplement */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
/* CID-Keyed font specific */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_1,
    /* card16 sentinel; */
    /* format 3 only, must be equals to num_glyphs */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut i8,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: card16,
    pub num_fds: card8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: i32,
    pub index: i32,
    pub flag: i32,
    pub is_notdef_notzero: i32,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    false
}
static mut cff_stdstr: [*const i8; 391] = [
    b".notdef\x00" as *const u8 as *const i8,
    b"space\x00" as *const u8 as *const i8,
    b"exclam\x00" as *const u8 as *const i8,
    b"quotedbl\x00" as *const u8 as *const i8,
    b"numbersign\x00" as *const u8 as *const i8,
    b"dollar\x00" as *const u8 as *const i8,
    b"percent\x00" as *const u8 as *const i8,
    b"ampersand\x00" as *const u8 as *const i8,
    b"quoteright\x00" as *const u8 as *const i8,
    b"parenleft\x00" as *const u8 as *const i8,
    b"parenright\x00" as *const u8 as *const i8,
    b"asterisk\x00" as *const u8 as *const i8,
    b"plus\x00" as *const u8 as *const i8,
    b"comma\x00" as *const u8 as *const i8,
    b"hyphen\x00" as *const u8 as *const i8,
    b"period\x00" as *const u8 as *const i8,
    b"slash\x00" as *const u8 as *const i8,
    b"zero\x00" as *const u8 as *const i8,
    b"one\x00" as *const u8 as *const i8,
    b"two\x00" as *const u8 as *const i8,
    b"three\x00" as *const u8 as *const i8,
    b"four\x00" as *const u8 as *const i8,
    b"five\x00" as *const u8 as *const i8,
    b"six\x00" as *const u8 as *const i8,
    b"seven\x00" as *const u8 as *const i8,
    b"eight\x00" as *const u8 as *const i8,
    b"nine\x00" as *const u8 as *const i8,
    b"colon\x00" as *const u8 as *const i8,
    b"semicolon\x00" as *const u8 as *const i8,
    b"less\x00" as *const u8 as *const i8,
    b"equal\x00" as *const u8 as *const i8,
    b"greater\x00" as *const u8 as *const i8,
    b"question\x00" as *const u8 as *const i8,
    b"at\x00" as *const u8 as *const i8,
    b"A\x00" as *const u8 as *const i8,
    b"B\x00" as *const u8 as *const i8,
    b"C\x00" as *const u8 as *const i8,
    b"D\x00" as *const u8 as *const i8,
    b"E\x00" as *const u8 as *const i8,
    b"F\x00" as *const u8 as *const i8,
    b"G\x00" as *const u8 as *const i8,
    b"H\x00" as *const u8 as *const i8,
    b"I\x00" as *const u8 as *const i8,
    b"J\x00" as *const u8 as *const i8,
    b"K\x00" as *const u8 as *const i8,
    b"L\x00" as *const u8 as *const i8,
    b"M\x00" as *const u8 as *const i8,
    b"N\x00" as *const u8 as *const i8,
    b"O\x00" as *const u8 as *const i8,
    b"P\x00" as *const u8 as *const i8,
    b"Q\x00" as *const u8 as *const i8,
    b"R\x00" as *const u8 as *const i8,
    b"S\x00" as *const u8 as *const i8,
    b"T\x00" as *const u8 as *const i8,
    b"U\x00" as *const u8 as *const i8,
    b"V\x00" as *const u8 as *const i8,
    b"W\x00" as *const u8 as *const i8,
    b"X\x00" as *const u8 as *const i8,
    b"Y\x00" as *const u8 as *const i8,
    b"Z\x00" as *const u8 as *const i8,
    b"bracketleft\x00" as *const u8 as *const i8,
    b"backslash\x00" as *const u8 as *const i8,
    b"bracketright\x00" as *const u8 as *const i8,
    b"asciicircum\x00" as *const u8 as *const i8,
    b"underscore\x00" as *const u8 as *const i8,
    b"quoteleft\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"c\x00" as *const u8 as *const i8,
    b"d\x00" as *const u8 as *const i8,
    b"e\x00" as *const u8 as *const i8,
    b"f\x00" as *const u8 as *const i8,
    b"g\x00" as *const u8 as *const i8,
    b"h\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"j\x00" as *const u8 as *const i8,
    b"k\x00" as *const u8 as *const i8,
    b"l\x00" as *const u8 as *const i8,
    b"m\x00" as *const u8 as *const i8,
    b"n\x00" as *const u8 as *const i8,
    b"o\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"r\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"t\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"v\x00" as *const u8 as *const i8,
    b"w\x00" as *const u8 as *const i8,
    b"x\x00" as *const u8 as *const i8,
    b"y\x00" as *const u8 as *const i8,
    b"z\x00" as *const u8 as *const i8,
    b"braceleft\x00" as *const u8 as *const i8,
    b"bar\x00" as *const u8 as *const i8,
    b"braceright\x00" as *const u8 as *const i8,
    b"asciitilde\x00" as *const u8 as *const i8,
    b"exclamdown\x00" as *const u8 as *const i8,
    b"cent\x00" as *const u8 as *const i8,
    b"sterling\x00" as *const u8 as *const i8,
    b"fraction\x00" as *const u8 as *const i8,
    b"yen\x00" as *const u8 as *const i8,
    b"florin\x00" as *const u8 as *const i8,
    b"section\x00" as *const u8 as *const i8,
    b"currency\x00" as *const u8 as *const i8,
    b"quotesingle\x00" as *const u8 as *const i8,
    b"quotedblleft\x00" as *const u8 as *const i8,
    b"guillemotleft\x00" as *const u8 as *const i8,
    b"guilsinglleft\x00" as *const u8 as *const i8,
    b"guilsinglright\x00" as *const u8 as *const i8,
    b"fi\x00" as *const u8 as *const i8,
    b"fl\x00" as *const u8 as *const i8,
    b"endash\x00" as *const u8 as *const i8,
    b"dagger\x00" as *const u8 as *const i8,
    b"daggerdbl\x00" as *const u8 as *const i8,
    b"periodcentered\x00" as *const u8 as *const i8,
    b"paragraph\x00" as *const u8 as *const i8,
    b"bullet\x00" as *const u8 as *const i8,
    b"quotesinglbase\x00" as *const u8 as *const i8,
    b"quotedblbase\x00" as *const u8 as *const i8,
    b"quotedblright\x00" as *const u8 as *const i8,
    b"guillemotright\x00" as *const u8 as *const i8,
    b"ellipsis\x00" as *const u8 as *const i8,
    b"perthousand\x00" as *const u8 as *const i8,
    b"questiondown\x00" as *const u8 as *const i8,
    b"grave\x00" as *const u8 as *const i8,
    b"acute\x00" as *const u8 as *const i8,
    b"circumflex\x00" as *const u8 as *const i8,
    b"tilde\x00" as *const u8 as *const i8,
    b"macron\x00" as *const u8 as *const i8,
    b"breve\x00" as *const u8 as *const i8,
    b"dotaccent\x00" as *const u8 as *const i8,
    b"dieresis\x00" as *const u8 as *const i8,
    b"ring\x00" as *const u8 as *const i8,
    b"cedilla\x00" as *const u8 as *const i8,
    b"hungarumlaut\x00" as *const u8 as *const i8,
    b"ogonek\x00" as *const u8 as *const i8,
    b"caron\x00" as *const u8 as *const i8,
    b"emdash\x00" as *const u8 as *const i8,
    b"AE\x00" as *const u8 as *const i8,
    b"ordfeminine\x00" as *const u8 as *const i8,
    b"Lslash\x00" as *const u8 as *const i8,
    b"Oslash\x00" as *const u8 as *const i8,
    b"OE\x00" as *const u8 as *const i8,
    b"ordmasculine\x00" as *const u8 as *const i8,
    b"ae\x00" as *const u8 as *const i8,
    b"dotlessi\x00" as *const u8 as *const i8,
    b"lslash\x00" as *const u8 as *const i8,
    b"oslash\x00" as *const u8 as *const i8,
    b"oe\x00" as *const u8 as *const i8,
    b"germandbls\x00" as *const u8 as *const i8,
    b"onesuperior\x00" as *const u8 as *const i8,
    b"logicalnot\x00" as *const u8 as *const i8,
    b"mu\x00" as *const u8 as *const i8,
    b"trademark\x00" as *const u8 as *const i8,
    b"Eth\x00" as *const u8 as *const i8,
    b"onehalf\x00" as *const u8 as *const i8,
    b"plusminus\x00" as *const u8 as *const i8,
    b"Thorn\x00" as *const u8 as *const i8,
    b"onequarter\x00" as *const u8 as *const i8,
    b"divide\x00" as *const u8 as *const i8,
    b"brokenbar\x00" as *const u8 as *const i8,
    b"degree\x00" as *const u8 as *const i8,
    b"thorn\x00" as *const u8 as *const i8,
    b"threequarters\x00" as *const u8 as *const i8,
    b"twosuperior\x00" as *const u8 as *const i8,
    b"registered\x00" as *const u8 as *const i8,
    b"minus\x00" as *const u8 as *const i8,
    b"eth\x00" as *const u8 as *const i8,
    b"multiply\x00" as *const u8 as *const i8,
    b"threesuperior\x00" as *const u8 as *const i8,
    b"copyright\x00" as *const u8 as *const i8,
    b"Aacute\x00" as *const u8 as *const i8,
    b"Acircumflex\x00" as *const u8 as *const i8,
    b"Adieresis\x00" as *const u8 as *const i8,
    b"Agrave\x00" as *const u8 as *const i8,
    b"Aring\x00" as *const u8 as *const i8,
    b"Atilde\x00" as *const u8 as *const i8,
    b"Ccedilla\x00" as *const u8 as *const i8,
    b"Eacute\x00" as *const u8 as *const i8,
    b"Ecircumflex\x00" as *const u8 as *const i8,
    b"Edieresis\x00" as *const u8 as *const i8,
    b"Egrave\x00" as *const u8 as *const i8,
    b"Iacute\x00" as *const u8 as *const i8,
    b"Icircumflex\x00" as *const u8 as *const i8,
    b"Idieresis\x00" as *const u8 as *const i8,
    b"Igrave\x00" as *const u8 as *const i8,
    b"Ntilde\x00" as *const u8 as *const i8,
    b"Oacute\x00" as *const u8 as *const i8,
    b"Ocircumflex\x00" as *const u8 as *const i8,
    b"Odieresis\x00" as *const u8 as *const i8,
    b"Ograve\x00" as *const u8 as *const i8,
    b"Otilde\x00" as *const u8 as *const i8,
    b"Scaron\x00" as *const u8 as *const i8,
    b"Uacute\x00" as *const u8 as *const i8,
    b"Ucircumflex\x00" as *const u8 as *const i8,
    b"Udieresis\x00" as *const u8 as *const i8,
    b"Ugrave\x00" as *const u8 as *const i8,
    b"Yacute\x00" as *const u8 as *const i8,
    b"Ydieresis\x00" as *const u8 as *const i8,
    b"Zcaron\x00" as *const u8 as *const i8,
    b"aacute\x00" as *const u8 as *const i8,
    b"acircumflex\x00" as *const u8 as *const i8,
    b"adieresis\x00" as *const u8 as *const i8,
    b"agrave\x00" as *const u8 as *const i8,
    b"aring\x00" as *const u8 as *const i8,
    b"atilde\x00" as *const u8 as *const i8,
    b"ccedilla\x00" as *const u8 as *const i8,
    b"eacute\x00" as *const u8 as *const i8,
    b"ecircumflex\x00" as *const u8 as *const i8,
    b"edieresis\x00" as *const u8 as *const i8,
    b"egrave\x00" as *const u8 as *const i8,
    b"iacute\x00" as *const u8 as *const i8,
    b"icircumflex\x00" as *const u8 as *const i8,
    b"idieresis\x00" as *const u8 as *const i8,
    b"igrave\x00" as *const u8 as *const i8,
    b"ntilde\x00" as *const u8 as *const i8,
    b"oacute\x00" as *const u8 as *const i8,
    b"ocircumflex\x00" as *const u8 as *const i8,
    b"odieresis\x00" as *const u8 as *const i8,
    b"ograve\x00" as *const u8 as *const i8,
    b"otilde\x00" as *const u8 as *const i8,
    b"scaron\x00" as *const u8 as *const i8,
    b"uacute\x00" as *const u8 as *const i8,
    b"ucircumflex\x00" as *const u8 as *const i8,
    b"udieresis\x00" as *const u8 as *const i8,
    b"ugrave\x00" as *const u8 as *const i8,
    b"yacute\x00" as *const u8 as *const i8,
    b"ydieresis\x00" as *const u8 as *const i8,
    b"zcaron\x00" as *const u8 as *const i8,
    b"exclamsmall\x00" as *const u8 as *const i8,
    b"Hungarumlautsmall\x00" as *const u8 as *const i8,
    b"dollaroldstyle\x00" as *const u8 as *const i8,
    b"dollarsuperior\x00" as *const u8 as *const i8,
    b"ampersandsmall\x00" as *const u8 as *const i8,
    b"Acutesmall\x00" as *const u8 as *const i8,
    b"parenleftsuperior\x00" as *const u8 as *const i8,
    b"parenrightsuperior\x00" as *const u8 as *const i8,
    b"twodotenleader\x00" as *const u8 as *const i8,
    b"onedotenleader\x00" as *const u8 as *const i8,
    b"zerooldstyle\x00" as *const u8 as *const i8,
    b"oneoldstyle\x00" as *const u8 as *const i8,
    b"twooldstyle\x00" as *const u8 as *const i8,
    b"threeoldstyle\x00" as *const u8 as *const i8,
    b"fouroldstyle\x00" as *const u8 as *const i8,
    b"fiveoldstyle\x00" as *const u8 as *const i8,
    b"sixoldstyle\x00" as *const u8 as *const i8,
    b"sevenoldstyle\x00" as *const u8 as *const i8,
    b"eightoldstyle\x00" as *const u8 as *const i8,
    b"nineoldstyle\x00" as *const u8 as *const i8,
    b"commasuperior\x00" as *const u8 as *const i8,
    b"threequartersemdash\x00" as *const u8 as *const i8,
    b"periodsuperior\x00" as *const u8 as *const i8,
    b"questionsmall\x00" as *const u8 as *const i8,
    b"asuperior\x00" as *const u8 as *const i8,
    b"bsuperior\x00" as *const u8 as *const i8,
    b"centsuperior\x00" as *const u8 as *const i8,
    b"dsuperior\x00" as *const u8 as *const i8,
    b"esuperior\x00" as *const u8 as *const i8,
    b"isuperior\x00" as *const u8 as *const i8,
    b"lsuperior\x00" as *const u8 as *const i8,
    b"msuperior\x00" as *const u8 as *const i8,
    b"nsuperior\x00" as *const u8 as *const i8,
    b"osuperior\x00" as *const u8 as *const i8,
    b"rsuperior\x00" as *const u8 as *const i8,
    b"ssuperior\x00" as *const u8 as *const i8,
    b"tsuperior\x00" as *const u8 as *const i8,
    b"ff\x00" as *const u8 as *const i8,
    b"ffi\x00" as *const u8 as *const i8,
    b"ffl\x00" as *const u8 as *const i8,
    b"parenleftinferior\x00" as *const u8 as *const i8,
    b"parenrightinferior\x00" as *const u8 as *const i8,
    b"Circumflexsmall\x00" as *const u8 as *const i8,
    b"hyphensuperior\x00" as *const u8 as *const i8,
    b"Gravesmall\x00" as *const u8 as *const i8,
    b"Asmall\x00" as *const u8 as *const i8,
    b"Bsmall\x00" as *const u8 as *const i8,
    b"Csmall\x00" as *const u8 as *const i8,
    b"Dsmall\x00" as *const u8 as *const i8,
    b"Esmall\x00" as *const u8 as *const i8,
    b"Fsmall\x00" as *const u8 as *const i8,
    b"Gsmall\x00" as *const u8 as *const i8,
    b"Hsmall\x00" as *const u8 as *const i8,
    b"Ismall\x00" as *const u8 as *const i8,
    b"Jsmall\x00" as *const u8 as *const i8,
    b"Ksmall\x00" as *const u8 as *const i8,
    b"Lsmall\x00" as *const u8 as *const i8,
    b"Msmall\x00" as *const u8 as *const i8,
    b"Nsmall\x00" as *const u8 as *const i8,
    b"Osmall\x00" as *const u8 as *const i8,
    b"Psmall\x00" as *const u8 as *const i8,
    b"Qsmall\x00" as *const u8 as *const i8,
    b"Rsmall\x00" as *const u8 as *const i8,
    b"Ssmall\x00" as *const u8 as *const i8,
    b"Tsmall\x00" as *const u8 as *const i8,
    b"Usmall\x00" as *const u8 as *const i8,
    b"Vsmall\x00" as *const u8 as *const i8,
    b"Wsmall\x00" as *const u8 as *const i8,
    b"Xsmall\x00" as *const u8 as *const i8,
    b"Ysmall\x00" as *const u8 as *const i8,
    b"Zsmall\x00" as *const u8 as *const i8,
    b"colonmonetary\x00" as *const u8 as *const i8,
    b"onefitted\x00" as *const u8 as *const i8,
    b"rupiah\x00" as *const u8 as *const i8,
    b"Tildesmall\x00" as *const u8 as *const i8,
    b"exclamdownsmall\x00" as *const u8 as *const i8,
    b"centoldstyle\x00" as *const u8 as *const i8,
    b"Lslashsmall\x00" as *const u8 as *const i8,
    b"Scaronsmall\x00" as *const u8 as *const i8,
    b"Zcaronsmall\x00" as *const u8 as *const i8,
    b"Dieresissmall\x00" as *const u8 as *const i8,
    b"Brevesmall\x00" as *const u8 as *const i8,
    b"Caronsmall\x00" as *const u8 as *const i8,
    b"Dotaccentsmall\x00" as *const u8 as *const i8,
    b"Macronsmall\x00" as *const u8 as *const i8,
    b"figuredash\x00" as *const u8 as *const i8,
    b"hypheninferior\x00" as *const u8 as *const i8,
    b"Ogoneksmall\x00" as *const u8 as *const i8,
    b"Ringsmall\x00" as *const u8 as *const i8,
    b"Cedillasmall\x00" as *const u8 as *const i8,
    b"questiondownsmall\x00" as *const u8 as *const i8,
    b"oneeighth\x00" as *const u8 as *const i8,
    b"threeeighths\x00" as *const u8 as *const i8,
    b"fiveeighths\x00" as *const u8 as *const i8,
    b"seveneighths\x00" as *const u8 as *const i8,
    b"onethird\x00" as *const u8 as *const i8,
    b"twothirds\x00" as *const u8 as *const i8,
    b"zerosuperior\x00" as *const u8 as *const i8,
    b"foursuperior\x00" as *const u8 as *const i8,
    b"fivesuperior\x00" as *const u8 as *const i8,
    b"sixsuperior\x00" as *const u8 as *const i8,
    b"sevensuperior\x00" as *const u8 as *const i8,
    b"eightsuperior\x00" as *const u8 as *const i8,
    b"ninesuperior\x00" as *const u8 as *const i8,
    b"zeroinferior\x00" as *const u8 as *const i8,
    b"oneinferior\x00" as *const u8 as *const i8,
    b"twoinferior\x00" as *const u8 as *const i8,
    b"threeinferior\x00" as *const u8 as *const i8,
    b"fourinferior\x00" as *const u8 as *const i8,
    b"fiveinferior\x00" as *const u8 as *const i8,
    b"sixinferior\x00" as *const u8 as *const i8,
    b"seveninferior\x00" as *const u8 as *const i8,
    b"eightinferior\x00" as *const u8 as *const i8,
    b"nineinferior\x00" as *const u8 as *const i8,
    b"centinferior\x00" as *const u8 as *const i8,
    b"dollarinferior\x00" as *const u8 as *const i8,
    b"periodinferior\x00" as *const u8 as *const i8,
    b"commainferior\x00" as *const u8 as *const i8,
    b"Agravesmall\x00" as *const u8 as *const i8,
    b"Aacutesmall\x00" as *const u8 as *const i8,
    b"Acircumflexsmall\x00" as *const u8 as *const i8,
    b"Atildesmall\x00" as *const u8 as *const i8,
    b"Adieresissmall\x00" as *const u8 as *const i8,
    b"Aringsmall\x00" as *const u8 as *const i8,
    b"AEsmall\x00" as *const u8 as *const i8,
    b"Ccedillasmall\x00" as *const u8 as *const i8,
    b"Egravesmall\x00" as *const u8 as *const i8,
    b"Eacutesmall\x00" as *const u8 as *const i8,
    b"Ecircumflexsmall\x00" as *const u8 as *const i8,
    b"Edieresissmall\x00" as *const u8 as *const i8,
    b"Igravesmall\x00" as *const u8 as *const i8,
    b"Iacutesmall\x00" as *const u8 as *const i8,
    b"Icircumflexsmall\x00" as *const u8 as *const i8,
    b"Idieresissmall\x00" as *const u8 as *const i8,
    b"Ethsmall\x00" as *const u8 as *const i8,
    b"Ntildesmall\x00" as *const u8 as *const i8,
    b"Ogravesmall\x00" as *const u8 as *const i8,
    b"Oacutesmall\x00" as *const u8 as *const i8,
    b"Ocircumflexsmall\x00" as *const u8 as *const i8,
    b"Otildesmall\x00" as *const u8 as *const i8,
    b"Odieresissmall\x00" as *const u8 as *const i8,
    b"OEsmall\x00" as *const u8 as *const i8,
    b"Oslashsmall\x00" as *const u8 as *const i8,
    b"Ugravesmall\x00" as *const u8 as *const i8,
    b"Uacutesmall\x00" as *const u8 as *const i8,
    b"Ucircumflexsmall\x00" as *const u8 as *const i8,
    b"Udieresissmall\x00" as *const u8 as *const i8,
    b"Yacutesmall\x00" as *const u8 as *const i8,
    b"Thornsmall\x00" as *const u8 as *const i8,
    b"Ydieresissmall\x00" as *const u8 as *const i8,
    b"001.000\x00" as *const u8 as *const i8,
    b"001.001\x00" as *const u8 as *const i8,
    b"001.002\x00" as *const u8 as *const i8,
    b"001.003\x00" as *const u8 as *const i8,
    b"Black\x00" as *const u8 as *const i8,
    b"Bold\x00" as *const u8 as *const i8,
    b"Book\x00" as *const u8 as *const i8,
    b"Light\x00" as *const u8 as *const i8,
    b"Medium\x00" as *const u8 as *const i8,
    b"Regular\x00" as *const u8 as *const i8,
    b"Roman\x00" as *const u8 as *const i8,
    b"Semibold\x00" as *const u8 as *const i8,
];
unsafe extern "C" fn get_unsigned(mut handle: rust_input_handle_t, mut n: i32) -> u32 {
    let mut v: u32 = 0_u32;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0i32) {
            break;
        }
        v = v
            .wrapping_mul(0x100u32)
            .wrapping_add(tt_get_unsigned_byte(handle) as u32)
    }
    v
}
/*
 * Read Header, Name INDEX, Top DICT INDEX, and String INDEX.
 */
#[no_mangle]
pub unsafe extern "C" fn cff_open(
    mut handle: rust_input_handle_t,
    mut offset: i32,
    mut n: i32,
) -> *mut cff_font {
    let mut cff: *mut cff_font = 0 as *mut cff_font; /* not used */
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    cff =
        new((1_u64).wrapping_mul(::std::mem::size_of::<cff_font>() as u64) as u32) as *mut cff_font;
    (*cff).fontname = 0 as *mut i8;
    (*cff).index = n;
    (*cff).handle = handle;
    (*cff).offset = offset as l_offset;
    (*cff).filter = 0i32;
    (*cff).flag = 0i32;
    (*cff).name = 0 as *mut cff_index;
    (*cff).topdict = 0 as *mut cff_dict;
    (*cff).gsubr = 0 as *mut cff_index;
    (*cff).encoding = 0 as *mut cff_encoding;
    (*cff).charsets = 0 as *mut cff_charsets;
    (*cff).fdselect = 0 as *mut cff_fdselect;
    (*cff).cstrings = 0 as *mut cff_index;
    (*cff).fdarray = 0 as *mut *mut cff_dict;
    (*cff).private = 0 as *mut *mut cff_dict;
    (*cff).subrs = 0 as *mut *mut cff_index;
    (*cff).num_glyphs = 0i32 as card16;
    (*cff).num_fds = 0i32 as card8;
    (*cff).string = 0 as *mut cff_index;
    (*cff)._string = 0 as *mut cff_index;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(0_u32) as ssize_t,
        0i32,
    );
    (*cff).header.major = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.minor = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.hdr_size = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.offsize = tt_get_unsigned_byte((*cff).handle);
    if ((*cff).header.offsize as i32) < 1i32 || (*cff).header.offsize as i32 > 4i32 {
        _tt_abort(b"invalid offsize data\x00" as *const u8 as *const i8);
    }
    if (*cff).header.major as i32 > 1i32 || (*cff).header.minor as i32 > 0i32 {
        dpx_warning(
            b"%s: CFF version %u.%u not supported.\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
            (*cff).header.major as i32,
            (*cff).header.minor as i32,
        );
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add((*cff).header.hdr_size as u32) as ssize_t,
        0i32,
    );
    /* Name INDEX */
    idx = cff_get_index(cff);
    if n > (*idx).count as i32 - 1i32 {
        warn!("{}: Invalid CFF fontset index number.", "CFF");
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    (*cff).name = idx;
    (*cff).fontname = cff_get_name(cff);
    /* Top DICT INDEX */
    idx = cff_get_index(cff);
    if n > (*idx).count as i32 - 1i32 {
        _tt_abort(b"CFF Top DICT not exist...\x00" as *const u8 as *const i8);
    }
    (*cff).topdict = cff_dict_unpack(
        (*idx)
            .data
            .offset(*(*idx).offset.offset(n as isize) as isize)
            .offset(-1),
        (*idx)
            .data
            .offset(*(*idx).offset.offset((n + 1i32) as isize) as isize)
            .offset(-1),
    );
    if (*cff).topdict.is_null() {
        _tt_abort(b"Parsing CFF Top DICT data failed...\x00" as *const u8 as *const i8);
    }
    cff_release_index(idx);
    if cff_dict_known(
        (*cff).topdict,
        b"CharstringType\x00" as *const u8 as *const i8,
    ) != 0
        && cff_dict_get(
            (*cff).topdict,
            b"CharstringType\x00" as *const u8 as *const i8,
            0i32,
        ) != 2i32 as f64
    {
        warn!("Only Type 2 Charstrings supported...");
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    if cff_dict_known(
        (*cff).topdict,
        b"SyntheticBase\x00" as *const u8 as *const i8,
    ) != 0
    {
        warn!("CFF Synthetic font not supported.");
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    /* String INDEX */
    (*cff).string = cff_get_index(cff);
    /* offset to GSubr */
    (*cff).gsubr_offset = ttstub_input_seek((*cff).handle, 0i32 as ssize_t, 1i32)
        .wrapping_sub(offset as u64) as l_offset;
    /* Number of glyphs */
    offset = cff_dict_get(
        (*cff).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    (*cff).num_glyphs = tt_get_unsigned_pair((*cff).handle);
    /* Check for font type */
    if cff_dict_known((*cff).topdict, b"ROS\x00" as *const u8 as *const i8) != 0 {
        (*cff).flag |= 1i32 << 0i32
    } else {
        (*cff).flag |= 1i32 << 1i32
    }
    /* Check for encoding */
    if cff_dict_known((*cff).topdict, b"Encoding\x00" as *const u8 as *const i8) != 0 {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Encoding\x00" as *const u8 as *const i8,
            0i32,
        ) as i32;
        if offset == 0i32 {
            /* predefined */
            (*cff).flag |= 1i32 << 3i32
        } else if offset == 1i32 {
            (*cff).flag |= 1i32 << 4i32
        }
    } else {
        (*cff).flag |= 1i32 << 3i32
    }
    /* Check for charset */
    if cff_dict_known((*cff).topdict, b"charset\x00" as *const u8 as *const i8) != 0 {
        offset = cff_dict_get(
            (*cff).topdict,
            b"charset\x00" as *const u8 as *const i8,
            0i32,
        ) as i32;
        if offset == 0i32 {
            /* predefined */
            (*cff).flag |= 1i32 << 5i32
        } else if offset == 1i32 {
            (*cff).flag |= 1i32 << 6i32
        } else if offset == 2i32 {
            (*cff).flag |= 1i32 << 7i32
        }
    } else {
        (*cff).flag |= 1i32 << 5i32
    } /* seek back to GSubr */
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add((*cff).gsubr_offset) as ssize_t,
        0i32,
    ); /* no trailing '\0' */
    return cff; /* Additional data in between header and
                 * Name INDEX ignored.
                 */
}
#[no_mangle]
pub unsafe extern "C" fn cff_close(mut cff: *mut cff_font) {
    let mut i: card16 = 0;
    if !cff.is_null() {
        free((*cff).fontname as *mut libc::c_void);
        if !(*cff).name.is_null() {
            cff_release_index((*cff).name);
        }
        if !(*cff).topdict.is_null() {
            cff_release_dict((*cff).topdict);
        }
        if !(*cff).string.is_null() {
            cff_release_index((*cff).string);
        }
        if !(*cff).gsubr.is_null() {
            cff_release_index((*cff).gsubr);
        }
        if !(*cff).encoding.is_null() {
            cff_release_encoding((*cff).encoding);
        }
        if !(*cff).charsets.is_null() {
            cff_release_charsets((*cff).charsets);
        }
        if !(*cff).fdselect.is_null() {
            cff_release_fdselect((*cff).fdselect);
        }
        if !(*cff).cstrings.is_null() {
            cff_release_index((*cff).cstrings);
        }
        if !(*cff).fdarray.is_null() {
            i = 0i32 as card16;
            while (i as i32) < (*cff).num_fds as i32 {
                if !(*(*cff).fdarray.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).fdarray.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).fdarray as *mut libc::c_void);
        }
        if !(*cff).private.is_null() {
            i = 0i32 as card16;
            while (i as i32) < (*cff).num_fds as i32 {
                if !(*(*cff).private.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).private.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).private as *mut libc::c_void);
        }
        if !(*cff).subrs.is_null() {
            i = 0i32 as card16;
            while (i as i32) < (*cff).num_fds as i32 {
                if !(*(*cff).subrs.offset(i as isize)).is_null() {
                    cff_release_index(*(*cff).subrs.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).subrs as *mut libc::c_void);
        }
        if !(*cff)._string.is_null() {
            cff_release_index((*cff)._string);
        }
        free(cff as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_name(mut cff: *mut cff_font) -> *mut i8 {
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut len: l_offset = 0;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    idx = (*cff).name;
    len = (*(*idx).offset.offset(((*cff).index + 1i32) as isize))
        .wrapping_sub(*(*idx).offset.offset((*cff).index as isize));
    fontname = new(
        (len.wrapping_add(1_u32) as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    memcpy(
        fontname as *mut libc::c_void,
        (*idx)
            .data
            .offset(*(*idx).offset.offset((*cff).index as isize) as isize)
            .offset(-1) as *const libc::c_void,
        len as u64,
    );
    *fontname.offset(len as isize) = '\u{0}' as i32 as i8;
    fontname
}
#[no_mangle]
pub unsafe extern "C" fn cff_set_name(mut cff: *mut cff_font, mut name: *mut i8) -> i32 {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    if strlen(name) > 127i32 as u64 {
        _tt_abort(b"FontName string length too large...\x00" as *const u8 as *const i8);
    }
    if !(*cff).name.is_null() {
        cff_release_index((*cff).name);
    }
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    (*cff).name = idx;
    (*idx).count = 1i32 as card16;
    (*idx).offsize = 1i32 as c_offsize;
    (*idx).offset =
        new((2_u64).wrapping_mul(::std::mem::size_of::<l_offset>() as u64) as u32) as *mut l_offset;
    *(*idx).offset.offset(0) = 1i32 as l_offset;
    *(*idx).offset.offset(1) = strlen(name).wrapping_add(1i32 as u64) as l_offset;
    (*idx).data = new(
        (strlen(name) as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32
    ) as *mut card8;
    memmove(
        (*idx).data as *mut libc::c_void,
        name as *const libc::c_void,
        strlen(name),
    );
    (5i32 as u64).wrapping_add(strlen(name)) as i32
}
#[no_mangle]
pub unsafe extern "C" fn cff_put_header(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    if destlen < 4i32 {
        _tt_abort(b"Not enough space available...\x00" as *const u8 as *const i8);
    }
    let fresh1 = dest;
    dest = dest.offset(1);
    *fresh1 = (*cff).header.major;
    let fresh2 = dest;
    dest = dest.offset(1);
    *fresh2 = (*cff).header.minor;
    let fresh3 = dest;
    dest = dest.offset(1);
    *fresh3 = 4i32 as card8;
    /* We will set all offset (0) to four-byte integer. */
    let fresh4 = dest;
    dest = dest.offset(1);
    *fresh4 = 4i32 as card8;
    (*cff).header.offsize = 4i32 as c_offsize;
    4i32
}
/* Only read header part but not body */
#[no_mangle]
pub unsafe extern "C" fn cff_get_index_header(mut cff: *mut cff_font) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut i: card16 = 0;
    let mut count: card16 = 0;
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    count = tt_get_unsigned_pair((*cff).handle);
    (*idx).count = count;
    if count as i32 > 0i32 {
        (*idx).offsize = tt_get_unsigned_byte((*cff).handle);
        if ((*idx).offsize as i32) < 1i32 || (*idx).offsize as i32 > 4i32 {
            _tt_abort(b"invalid offsize data\x00" as *const u8 as *const i8);
        }
        (*idx).offset = new(((count as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64)
            as u32) as *mut l_offset;
        i = 0i32 as card16;
        while (i as i32) < count as i32 {
            *(*idx).offset.offset(i as isize) = get_unsigned((*cff).handle, (*idx).offsize as i32);
            i = i.wrapping_add(1)
        }
        if count as i32 == 0xffffi32 {
            ttstub_input_seek(
                (*cff).handle,
                ttstub_input_seek((*cff).handle, 0i32 as ssize_t, 1i32)
                    .wrapping_add((*idx).offsize as u64) as ssize_t,
                0i32,
            );
        } else {
            *(*idx).offset.offset(i as isize) = get_unsigned((*cff).handle, (*idx).offsize as i32)
        }
        if *(*idx).offset.offset(0) != 1_u32 {
            _tt_abort(b"cff_get_index(): invalid index data\x00" as *const u8 as *const i8);
        }
        (*idx).data = 0 as *mut card8
    } else {
        (*idx).offsize = 0i32 as c_offsize;
        (*idx).offset = 0 as *mut l_offset;
        (*idx).data = 0 as *mut card8
    }
    idx
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_index(mut cff: *mut cff_font) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut i: card16 = 0;
    let mut count: card16 = 0;
    let mut length: i32 = 0;
    let mut nb_read: i32 = 0;
    let mut offset: i32 = 0;
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    count = tt_get_unsigned_pair((*cff).handle);
    (*idx).count = count;
    if count as i32 > 0i32 {
        (*idx).offsize = tt_get_unsigned_byte((*cff).handle);
        if ((*idx).offsize as i32) < 1i32 || (*idx).offsize as i32 > 4i32 {
            _tt_abort(b"invalid offsize data\x00" as *const u8 as *const i8);
        }
        (*idx).offset = new(((count as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64)
            as u32) as *mut l_offset;
        i = 0i32 as card16;
        while (i as i32) < count as i32 + 1i32 {
            *(*idx).offset.offset(i as isize) = get_unsigned((*cff).handle, (*idx).offsize as i32);
            i = i.wrapping_add(1)
        }
        if *(*idx).offset.offset(0) != 1_u32 {
            _tt_abort(b"Invalid CFF Index offset data\x00" as *const u8 as *const i8);
        }
        length =
            (*(*idx).offset.offset(count as isize)).wrapping_sub(*(*idx).offset.offset(0)) as i32;
        (*idx).data =
            new((length as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32)
                as *mut card8;
        offset = 0i32;
        while length > 0i32 {
            nb_read = ttstub_input_read(
                (*cff).handle,
                ((*idx).data as *mut i8).offset(offset as isize),
                length as size_t,
            ) as i32;
            offset += nb_read;
            length -= nb_read
        }
    } else {
        (*idx).offsize = 0i32 as c_offsize;
        (*idx).offset = 0 as *mut l_offset;
        (*idx).data = 0 as *mut card8
    }
    idx
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_index(
    mut idx: *mut cff_index,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut datalen: size_t = 0;
    let mut i: card16 = 0;
    if ((*idx).count as i32) < 1i32 {
        if destlen < 2i32 {
            _tt_abort(b"Not enough space available...\x00" as *const u8 as *const i8);
        }
        memset(dest as *mut libc::c_void, 0i32, 2i32 as u64);
        return 2i32;
    }
    len = cff_index_size(idx);
    datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1_u32) as size_t;
    if destlen < len {
        _tt_abort(b"Not enough space available...\x00" as *const u8 as *const i8);
    }
    let fresh5 = dest;
    dest = dest.offset(1);
    *fresh5 = ((*idx).count as i32 >> 8i32 & 0xffi32) as card8;
    let fresh6 = dest;
    dest = dest.offset(1);
    *fresh6 = ((*idx).count as i32 & 0xffi32) as card8;
    if datalen < 0xff {
        (*idx).offsize = 1i32 as c_offsize;
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = 1i32 as card8;
        i = 0i32 as card16;
        while i as i32 <= (*idx).count as i32 {
            let fresh8 = dest;
            dest = dest.offset(1);
            *fresh8 = (*(*idx).offset.offset(i as isize) & 0xff_u32) as card8;
            i = i.wrapping_add(1)
        }
    } else if datalen < 0xffff {
        (*idx).offsize = 2i32 as c_offsize;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = 2i32 as card8;
        i = 0i32 as card16;
        while i as i32 <= (*idx).count as i32 {
            let fresh10 = dest;
            dest = dest.offset(1);
            *fresh10 = (*(*idx).offset.offset(i as isize) >> 8i32 & 0xff_u32) as card8;
            let fresh11 = dest;
            dest = dest.offset(1);
            *fresh11 = (*(*idx).offset.offset(i as isize) & 0xff_u32) as card8;
            i = i.wrapping_add(1)
        }
    } else if datalen < 0xffffff {
        (*idx).offsize = 3i32 as c_offsize;
        let fresh12 = dest;
        dest = dest.offset(1);
        *fresh12 = 3i32 as card8;
        i = 0i32 as card16;
        while i as i32 <= (*idx).count as i32 {
            let fresh13 = dest;
            dest = dest.offset(1);
            *fresh13 = (*(*idx).offset.offset(i as isize) >> 16i32 & 0xff_u32) as card8;
            let fresh14 = dest;
            dest = dest.offset(1);
            *fresh14 = (*(*idx).offset.offset(i as isize) >> 8i32 & 0xff_u32) as card8;
            let fresh15 = dest;
            dest = dest.offset(1);
            *fresh15 = (*(*idx).offset.offset(i as isize) & 0xff_u32) as card8;
            i = i.wrapping_add(1)
        }
    } else {
        (*idx).offsize = 4i32 as c_offsize;
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = 4i32 as card8;
        i = 0i32 as card16;
        while i as i32 <= (*idx).count as i32 {
            let fresh17 = dest;
            dest = dest.offset(1);
            *fresh17 = (*(*idx).offset.offset(i as isize) >> 24i32 & 0xff_u32) as card8;
            let fresh18 = dest;
            dest = dest.offset(1);
            *fresh18 = (*(*idx).offset.offset(i as isize) >> 16i32 & 0xff_u32) as card8;
            let fresh19 = dest;
            dest = dest.offset(1);
            *fresh19 = (*(*idx).offset.offset(i as isize) >> 8i32 & 0xff_u32) as card8;
            let fresh20 = dest;
            dest = dest.offset(1);
            *fresh20 = (*(*idx).offset.offset(i as isize) & 0xff_u32) as card8;
            i = i.wrapping_add(1)
        }
    }
    memmove(
        dest as *mut libc::c_void,
        (*idx).data as *const libc::c_void,
        (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1_u32) as u64,
    );
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_index_size(mut idx: *mut cff_index) -> i32 {
    if (*idx).count as i32 > 0i32 {
        let mut datalen: l_offset = 0;
        datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1_u32);
        if (datalen as u64) < 0xff {
            (*idx).offsize = 1i32 as c_offsize
        } else if (datalen as u64) < 0xffff {
            (*idx).offsize = 2i32 as c_offsize
        } else if (datalen as u64) < 0xffffff {
            (*idx).offsize = 3i32 as c_offsize
        } else {
            (*idx).offsize = 4i32 as c_offsize
        }
        return ((3i32 + (*idx).offsize as i32 * ((*idx).count as i32 + 1i32)) as u32)
            .wrapping_add(datalen) as i32;
    } else {
        return 2i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_new_index(mut count: card16) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    (*idx).count = count;
    (*idx).offsize = 0i32 as c_offsize;
    if count as i32 > 0i32 {
        (*idx).offset = new(((count as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64)
            as u32) as *mut l_offset;
        *(*idx).offset.offset(0) = 1i32 as l_offset
    } else {
        (*idx).offset = 0 as *mut l_offset
    }
    (*idx).data = 0 as *mut card8;
    idx
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_index(mut idx: *mut cff_index) {
    if !idx.is_null() {
        if !(*idx).data.is_null() {
            free((*idx).data as *mut libc::c_void);
        }
        if !(*idx).offset.is_null() {
            free((*idx).offset as *mut libc::c_void);
        }
        free(idx as *mut libc::c_void);
    };
}
/* Strings */
#[no_mangle]
pub unsafe extern "C" fn cff_get_string(mut cff: *mut cff_font, mut id: s_SID) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    if (id as i32) < 391i32 {
        len = strlen(cff_stdstr[id as usize]) as i32;
        result = new(
            ((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        memcpy(
            result as *mut libc::c_void,
            cff_stdstr[id as usize] as *const libc::c_void,
            len as u64,
        );
        *result.offset(len as isize) = '\u{0}' as i32 as i8
    } else if !cff.is_null() && !(*cff).string.is_null() {
        let mut strings: *mut cff_index = (*cff).string;
        id = (id as i32 - 391i32) as s_SID;
        if (id as i32) < (*strings).count as i32 {
            len = (*(*strings).offset.offset((id as i32 + 1i32) as isize))
                .wrapping_sub(*(*strings).offset.offset(id as isize)) as i32;
            result = new(((len + 1i32) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
                as *mut i8;
            memmove(
                result as *mut libc::c_void,
                (*strings)
                    .data
                    .offset(*(*strings).offset.offset(id as isize) as isize)
                    .offset(-1) as *const libc::c_void,
                len as u64,
            );
            *result.offset(len as isize) = '\u{0}' as i32 as i8
        }
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_sid(mut cff: *mut cff_font, mut str: *const i8) -> i32 {
    let mut i: card16 = 0;
    if cff.is_null() || str.is_null() {
        return -1i32;
    }
    /* I search String INDEX first. */
    if !cff.is_null() && !(*cff).string.is_null() {
        let mut idx: *mut cff_index = (*cff).string;
        i = 0i32 as card16;
        while (i as i32) < (*idx).count as i32 {
            if strlen(str)
                == (*(*idx).offset.offset((i as i32 + 1i32) as isize))
                    .wrapping_sub(*(*idx).offset.offset(i as isize)) as u64
                && memcmp(
                    str as *const libc::c_void,
                    (*idx)
                        .data
                        .offset(*(*idx).offset.offset(i as isize) as isize)
                        .offset(-1) as *const libc::c_void,
                    strlen(str),
                ) == 0
            {
                return i as i32 + 391i32;
            }
            i = i.wrapping_add(1)
        }
    }
    i = 0i32 as card16;
    while (i as i32) < 391i32 {
        if streq_ptr(str, cff_stdstr[i as usize]) {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    -1i32
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_seac_sid(mut cff: *mut cff_font, mut str: *const i8) -> i32 {
    let mut i: card16 = 0;
    if cff.is_null() || str.is_null() {
        return -1i32;
    }
    i = 0i32 as card16;
    while (i as i32) < 391i32 {
        if streq_ptr(str, cff_stdstr[i as usize]) {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    -1i32
}
unsafe extern "C" fn cff_match_string(
    mut cff: *mut cff_font,
    mut str: *const i8,
    mut sid: s_SID,
) -> i32 {
    let mut i: card16 = 0;
    if (sid as i32) < 391i32 {
        return if streq_ptr(str, cff_stdstr[sid as usize]) as i32 != 0 {
            1i32
        } else {
            0i32
        };
    } else {
        i = (sid as i32 - 391i32) as card16;
        if cff.is_null() || (*cff).string.is_null() || i as i32 >= (*(*cff).string).count as i32 {
            _tt_abort(b"Invalid SID\x00" as *const u8 as *const i8);
        }
        if strlen(str)
            == (*(*(*cff).string).offset.offset((i as i32 + 1i32) as isize))
                .wrapping_sub(*(*(*cff).string).offset.offset(i as isize)) as u64
        {
            return if memcmp(
                str as *const libc::c_void,
                (*(*cff).string)
                    .data
                    .offset(*(*(*cff).string).offset.offset(i as isize) as isize)
                    .offset(-1) as *const libc::c_void,
                strlen(str),
            ) == 0
            {
                1i32
            } else {
                0i32
            };
        }
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn cff_update_string(mut cff: *mut cff_font) {
    if cff.is_null() {
        _tt_abort(b"CFF font not opened.\x00" as *const u8 as *const i8);
    }
    if !(*cff).string.is_null() {
        cff_release_index((*cff).string);
    }
    (*cff).string = (*cff)._string;
    (*cff)._string = 0 as *mut cff_index;
}
/* String */
#[no_mangle]
pub unsafe extern "C" fn cff_add_string(
    mut cff: *mut cff_font,
    mut str: *const i8,
    mut unique: i32,
) -> s_SID
/* Setting unique == 1 eliminates redundant or predefined strings. */ {
    let mut idx: card16 = 0;
    let mut strings: *mut cff_index = 0 as *mut cff_index;
    let mut offset: l_offset = 0;
    let mut size: l_offset = 0;
    let mut len: size_t = strlen(str);
    if cff.is_null() {
        _tt_abort(b"CFF font not opened.\x00" as *const u8 as *const i8);
    }
    if (*cff)._string.is_null() {
        (*cff)._string = cff_new_index(0i32 as card16)
    }
    strings = (*cff)._string;
    if unique != 0 {
        /* TODO: do binary search to speed things up */
        idx = 0i32 as card16;
        while (idx as i32) < 391i32 {
            if streq_ptr(cff_stdstr[idx as usize], str) {
                return idx;
            }
            idx = idx.wrapping_add(1)
        }
        idx = 0i32 as card16;
        while (idx as i32) < (*strings).count as i32 {
            size = (*(*strings).offset.offset((idx as i32 + 1i32) as isize))
                .wrapping_sub(*(*strings).offset.offset(idx as isize));
            offset = *(*strings).offset.offset(idx as isize);
            if size as u64 == len
                && memcmp(
                    (*strings).data.offset(offset as isize).offset(-1) as *const libc::c_void,
                    str as *const libc::c_void,
                    len,
                ) == 0
            {
                return (idx as i32 + 391i32) as s_SID;
            }
            idx = idx.wrapping_add(1)
        }
    }
    offset = if (*strings).count as i32 > 0i32 {
        *(*strings).offset.offset((*strings).count as isize)
    } else {
        1_u32
    };
    (*strings).offset = renew(
        (*strings).offset as *mut libc::c_void,
        (((*strings).count as i32 + 2i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64) as u32,
    ) as *mut l_offset;
    if (*strings).count as i32 == 0i32 {
        *(*strings).offset.offset(0) = 1i32 as l_offset
    }
    idx = (*strings).count;
    (*strings).count = ((*strings).count as i32 + 1i32) as card16;
    *(*strings).offset.offset((*strings).count as isize) =
        (offset as u64).wrapping_add(len) as l_offset;
    (*strings).data = renew(
        (*strings).data as *mut libc::c_void,
        ((offset as u64).wrapping_add(len).wrapping_sub(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32,
    ) as *mut card8;
    memcpy(
        (*strings).data.offset(offset as isize).offset(-1) as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    (idx as i32 + 391i32) as s_SID
}
/*
 * Encoding and Charset
 *
 *  Encoding and Charset arrays always begin with GID = 1.
 */
#[no_mangle]
pub unsafe extern "C" fn cff_read_encoding(mut cff: *mut cff_font) -> i32 {
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut i: card8 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"Top DICT data not found\x00" as *const u8 as *const i8);
    }
    if cff_dict_known((*cff).topdict, b"Encoding\x00" as *const u8 as *const i8) == 0 {
        (*cff).flag |= 1i32 << 3i32;
        (*cff).encoding = 0 as *mut cff_encoding;
        return 0i32;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"Encoding\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    if offset == 0i32 {
        /* predefined */
        (*cff).flag |= 1i32 << 3i32;
        (*cff).encoding = 0 as *mut cff_encoding;
        return 0i32;
    } else {
        if offset == 1i32 {
            (*cff).flag |= 1i32 << 4i32;
            (*cff).encoding = 0 as *mut cff_encoding;
            return 0i32;
        }
    }
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    encoding = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_encoding>() as u64) as u32)
        as *mut cff_encoding;
    (*cff).encoding = encoding;
    (*encoding).format = tt_get_unsigned_byte((*cff).handle);
    length = 1i32;
    match (*encoding).format as i32 & !0x80i32 {
        0 => {
            (*encoding).num_entries = tt_get_unsigned_byte((*cff).handle);
            (*encoding).data.codes = new(((*encoding).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<card8>() as u64)
                as u32) as *mut card8;
            i = 0i32 as card8;
            while (i as i32) < (*encoding).num_entries as i32 {
                *(*encoding).data.codes.offset(i as isize) = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            length += (*encoding).num_entries as i32 + 1i32
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            (*encoding).num_entries = tt_get_unsigned_byte((*cff).handle);
            ranges = new(((*encoding).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<cff_range1>() as u64)
                as u32) as *mut cff_range1;
            (*encoding).data.range1 = ranges;
            i = 0i32 as card8;
            while (i as i32) < (*encoding).num_entries as i32 {
                (*ranges.offset(i as isize)).first = tt_get_unsigned_byte((*cff).handle) as s_SID;
                (*ranges.offset(i as isize)).n_left = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            length += (*encoding).num_entries as i32 * 2i32 + 1i32
        }
        _ => {
            free(encoding as *mut libc::c_void);
            _tt_abort(b"Unknown Encoding format\x00" as *const u8 as *const i8);
        }
    }
    /* Supplementary data */
    if (*encoding).format as i32 & 0x80i32 != 0 {
        let mut map: *mut cff_map = 0 as *mut cff_map;
        (*encoding).num_supps = tt_get_unsigned_byte((*cff).handle);
        map = new(((*encoding).num_supps as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<cff_map>() as u64) as u32)
            as *mut cff_map;
        (*encoding).supp = map;
        i = 0i32 as card8;
        while (i as i32) < (*encoding).num_supps as i32 {
            (*map.offset(i as isize)).code = tt_get_unsigned_byte((*cff).handle);
            (*map.offset(i as isize)).glyph = tt_get_unsigned_pair((*cff).handle);
            i = i.wrapping_add(1)
            /* SID */
        }
        length += (*encoding).num_supps as i32 * 3i32 + 1i32
    } else {
        (*encoding).num_supps = 0i32 as card8;
        (*encoding).supp = 0 as *mut cff_map
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_encoding(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut i: card16 = 0;
    if (*cff).flag & (1i32 << 3i32 | 1i32 << 4i32) != 0 || (*cff).encoding.is_null() {
        return 0i32;
    }
    if destlen < 2i32 {
        _tt_abort(b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8 as *const i8);
    }
    encoding = (*cff).encoding;
    let fresh21 = len;
    len = len + 1;
    *dest.offset(fresh21 as isize) = (*encoding).format;
    let fresh22 = len;
    len = len + 1;
    *dest.offset(fresh22 as isize) = (*encoding).num_entries;
    match (*encoding).format as i32 & !0x80i32 {
        0 => {
            if destlen < len + (*encoding).num_entries as i32 {
                _tt_abort(b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            i = 0i32 as card16;
            while (i as i32) < (*encoding).num_entries as i32 {
                let fresh23 = len;
                len = len + 1;
                *dest.offset(fresh23 as isize) = *(*encoding).data.codes.offset(i as isize);
                i = i.wrapping_add(1)
            }
        }
        1 => {
            if destlen < len + (*encoding).num_entries as i32 * 2i32 {
                _tt_abort(b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            i = 0i32 as card16;
            while (i as i32) < (*encoding).num_entries as i32 {
                let fresh24 = len;
                len = len + 1;
                *dest.offset(fresh24 as isize) =
                    ((*(*encoding).data.range1.offset(i as isize)).first as i32 & 0xffi32) as card8;
                let fresh25 = len;
                len = len + 1;
                *dest.offset(fresh25 as isize) =
                    (*(*encoding).data.range1.offset(i as isize)).n_left;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Encoding format\x00" as *const u8 as *const i8);
        }
    }
    if (*encoding).format as i32 & 0x80i32 != 0 {
        if destlen < len + (*encoding).num_supps as i32 * 3i32 + 1i32 {
            _tt_abort(b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8 as *const i8);
        }
        let fresh26 = len;
        len = len + 1;
        *dest.offset(fresh26 as isize) = (*encoding).num_supps;
        i = 0i32 as card16;
        while (i as i32) < (*encoding).num_supps as i32 {
            let fresh27 = len;
            len = len + 1;
            *dest.offset(fresh27 as isize) = (*(*encoding).supp.offset(i as isize)).code;
            let fresh28 = len;
            len = len + 1;
            *dest.offset(fresh28 as isize) =
                ((*(*encoding).supp.offset(i as isize)).glyph as i32 >> 8i32 & 0xffi32) as card8;
            let fresh29 = len;
            len = len + 1;
            *dest.offset(fresh29 as isize) =
                ((*(*encoding).supp.offset(i as isize)).glyph as i32 & 0xffi32) as card8;
            i = i.wrapping_add(1)
        }
    }
    len
}
/* input: code, output: glyph index */
#[no_mangle]
pub unsafe extern "C" fn cff_encoding_lookup(mut cff: *mut cff_font, mut code: card8) -> card16 {
    let mut gid: card16 = 0i32 as card16;
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut i: card16 = 0;
    if (*cff).flag & (1i32 << 3i32 | 1i32 << 4i32) != 0 {
        _tt_abort(b"Predefined CFF encoding not supported yet\x00" as *const u8 as *const i8);
    } else {
        if (*cff).encoding.is_null() {
            _tt_abort(b"Encoding data not available\x00" as *const u8 as *const i8);
        }
    }
    encoding = (*cff).encoding;
    gid = 0i32 as card16;
    match (*encoding).format as i32 & !0x80i32 {
        0 => {
            i = 0i32 as card16;
            while (i as i32) < (*encoding).num_entries as i32 {
                if code as i32 == *(*encoding).data.codes.offset(i as isize) as i32 {
                    gid = (i as i32 + 1i32) as card16;
                    break;
                } else {
                    i = i.wrapping_add(1)
                }
            }
        }
        1 => {
            i = 0i32 as card16;
            while (i as i32) < (*encoding).num_entries as i32 {
                if code as i32 >= (*(*encoding).data.range1.offset(i as isize)).first as i32
                    && code as i32
                        <= (*(*encoding).data.range1.offset(i as isize)).first as i32
                            + (*(*encoding).data.range1.offset(i as isize)).n_left as i32
                {
                    gid = (gid as i32
                        + (code as i32
                            - (*(*encoding).data.range1.offset(i as isize)).first as i32
                            + 1i32)) as card16;
                    break;
                } else {
                    gid = (gid as i32
                        + ((*(*encoding).data.range1.offset(i as isize)).n_left as i32 + 1i32))
                        as card16;
                    i = i.wrapping_add(1)
                }
            }
            if i as i32 == (*encoding).num_entries as i32 {
                gid = 0i32 as card16
            }
        }
        _ => {
            _tt_abort(b"Unknown Encoding format.\x00" as *const u8 as *const i8);
        }
    }
    /* Supplementary data */
    if gid as i32 == 0i32 && (*encoding).format as i32 & 0x80i32 != 0 {
        let mut map: *mut cff_map = 0 as *mut cff_map;
        if (*encoding).supp.is_null() {
            _tt_abort(b"No CFF supplementary encoding data read.\x00" as *const u8 as *const i8);
        }
        map = (*encoding).supp;
        i = 0i32 as card16;
        while (i as i32) < (*encoding).num_supps as i32 {
            if code as i32 == (*map.offset(i as isize)).code as i32 {
                gid = cff_charsets_lookup(cff, (*map.offset(i as isize)).glyph);
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
    }
    gid
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_encoding(mut encoding: *mut cff_encoding) {
    if !encoding.is_null() {
        match (*encoding).format as i32 & !0x80i32 {
            0 => {
                free((*encoding).data.codes as *mut libc::c_void);
            }
            1 => {
                free((*encoding).data.range1 as *mut libc::c_void);
            }
            _ => {
                _tt_abort(b"Unknown Encoding format.\x00" as *const u8 as *const i8);
            }
        }
        if (*encoding).format as i32 & 0x80i32 != 0 {
            free((*encoding).supp as *mut libc::c_void);
        }
        free(encoding as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_charsets(mut cff: *mut cff_font) -> i32 {
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut count: card16 = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"Top DICT not available\x00" as *const u8 as *const i8);
    }
    if cff_dict_known((*cff).topdict, b"charset\x00" as *const u8 as *const i8) == 0 {
        (*cff).flag |= 1i32 << 5i32;
        (*cff).charsets = 0 as *mut cff_charsets;
        return 0i32;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"charset\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    if offset == 0i32 {
        /* predefined */
        (*cff).flag |= 1i32 << 5i32;
        (*cff).charsets = 0 as *mut cff_charsets;
        return 0i32;
    } else {
        if offset == 1i32 {
            (*cff).flag |= 1i32 << 6i32;
            (*cff).charsets = 0 as *mut cff_charsets;
            return 0i32;
        } else {
            if offset == 2i32 {
                (*cff).flag |= 1i32 << 7i32;
                (*cff).charsets = 0 as *mut cff_charsets;
                return 0i32;
            }
        }
    }
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    charset = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_charsets>() as u64) as u32)
        as *mut cff_charsets;
    (*cff).charsets = charset;
    (*charset).format = tt_get_unsigned_byte((*cff).handle);
    (*charset).num_entries = 0i32 as card16;
    count = ((*cff).num_glyphs as i32 - 1i32) as card16;
    length = 1i32;
    /* Not sure. Not well documented. */
    match (*charset).format as i32 {
        0 => {
            (*charset).num_entries = ((*cff).num_glyphs as i32 - 1i32) as card16; /* no .notdef */
            (*charset).data.glyphs = new(((*charset).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<s_SID>() as u64)
                as u32) as *mut s_SID; /* no-overrap */
            length += (*charset).num_entries as i32 * 2i32; /* non-overrapping */
            i = 0i32 as card16; /* or CID */
            while (i as i32) < (*charset).num_entries as i32 {
                *(*charset).data.glyphs.offset(i as isize) = tt_get_unsigned_pair((*cff).handle);
                i = i.wrapping_add(1)
            }
            count = 0i32 as card16
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            while count as i32 > 0i32 && ((*charset).num_entries as i32) < (*cff).num_glyphs as i32
            {
                ranges = renew(
                    ranges as *mut libc::c_void,
                    (((*charset).num_entries as i32 + 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<cff_range1>() as u64)
                        as u32,
                ) as *mut cff_range1;
                (*ranges.offset((*charset).num_entries as isize)).first =
                    tt_get_unsigned_pair((*cff).handle);
                (*ranges.offset((*charset).num_entries as isize)).n_left =
                    tt_get_unsigned_byte((*cff).handle);
                count = (count as i32
                    - ((*ranges.offset((*charset).num_entries as isize)).n_left as i32 + 1i32))
                    as card16;
                (*charset).num_entries = ((*charset).num_entries as i32 + 1i32) as card16;
                (*charset).data.range1 = ranges
            }
            length += (*charset).num_entries as i32 * 3i32
        }
        2 => {
            let mut ranges_0: *mut cff_range2 = 0 as *mut cff_range2;
            while count as i32 > 0i32 && ((*charset).num_entries as i32) < (*cff).num_glyphs as i32
            {
                ranges_0 = renew(
                    ranges_0 as *mut libc::c_void,
                    (((*charset).num_entries as i32 + 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<cff_range2>() as u64)
                        as u32,
                ) as *mut cff_range2;
                (*ranges_0.offset((*charset).num_entries as isize)).first =
                    tt_get_unsigned_pair((*cff).handle);
                (*ranges_0.offset((*charset).num_entries as isize)).n_left =
                    tt_get_unsigned_pair((*cff).handle);
                count = (count as i32
                    - ((*ranges_0.offset((*charset).num_entries as isize)).n_left as i32 + 1i32))
                    as card16;
                (*charset).num_entries = ((*charset).num_entries as i32 + 1i32) as card16
            }
            (*charset).data.range2 = ranges_0;
            length += (*charset).num_entries as i32 * 4i32
        }
        _ => {
            free(charset as *mut libc::c_void);
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const i8);
        }
    }
    if count as i32 > 0i32 {
        _tt_abort(b"Charset data possibly broken\x00" as *const u8 as *const i8);
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_charsets(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut i: card16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    if (*cff).flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 || (*cff).charsets.is_null()
    {
        return 0i32;
    }
    if destlen < 1i32 {
        _tt_abort(b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8 as *const i8);
    }
    charset = (*cff).charsets;
    let fresh30 = len;
    len = len + 1;
    *dest.offset(fresh30 as isize) = (*charset).format;
    match (*charset).format as i32 {
        0 => {
            if destlen < len + (*charset).num_entries as i32 * 2i32 {
                _tt_abort(b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                let mut sid: s_SID = *(*charset).data.glyphs.offset(i as isize);
                let fresh31 = len;
                len = len + 1;
                *dest.offset(fresh31 as isize) = (sid as i32 >> 8i32 & 0xffi32) as card8;
                let fresh32 = len;
                len = len + 1;
                *dest.offset(fresh32 as isize) = (sid as i32 & 0xffi32) as card8;
                i = i.wrapping_add(1)
            }
        }
        1 => {
            if destlen < len + (*charset).num_entries as i32 * 3i32 {
                _tt_abort(b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                let fresh33 = len;
                len = len + 1;
                *dest.offset(fresh33 as isize) =
                    ((*(*charset).data.range1.offset(i as isize)).first as i32 >> 8i32 & 0xffi32)
                        as card8;
                let fresh34 = len;
                len = len + 1;
                *dest.offset(fresh34 as isize) =
                    ((*(*charset).data.range1.offset(i as isize)).first as i32 & 0xffi32) as card8;
                let fresh35 = len;
                len = len + 1;
                *dest.offset(fresh35 as isize) =
                    (*(*charset).data.range1.offset(i as isize)).n_left;
                i = i.wrapping_add(1)
            }
        }
        2 => {
            if destlen < len + (*charset).num_entries as i32 * 4i32 {
                _tt_abort(b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                let fresh36 = len;
                len = len + 1;
                *dest.offset(fresh36 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).first as i32 >> 8i32 & 0xffi32)
                        as card8;
                let fresh37 = len;
                len = len + 1;
                *dest.offset(fresh37 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).first as i32 & 0xffi32) as card8;
                let fresh38 = len;
                len = len + 1;
                *dest.offset(fresh38 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).n_left as i32 >> 8i32 & 0xffi32)
                        as card8;
                let fresh39 = len;
                len = len + 1;
                *dest.offset(fresh39 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).n_left as i32 & 0xffi32) as card8;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const i8);
        }
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_glyphname(mut cff: *mut cff_font, mut gid: card16) -> *mut i8 {
    let mut sid: s_SID = 0;
    sid = cff_charsets_lookup_inverse(cff, gid);
    cff_get_string(cff, sid)
}
#[no_mangle]
pub unsafe extern "C" fn cff_glyph_lookup(mut cff: *mut cff_font, mut glyph: *const i8) -> card16 {
    let mut gid: card16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut i: card16 = 0;
    let mut n: card16 = 0;
    if (*cff).flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        _tt_abort(b"Predefined CFF charsets not supported yet\x00" as *const u8 as *const i8);
    } else {
        if (*cff).charsets.is_null() {
            _tt_abort(b"Charsets data not available\x00" as *const u8 as *const i8);
        }
    }
    /* .notdef always have glyph index 0 */
    if glyph.is_null() || streq_ptr(glyph, b".notdef\x00" as *const u8 as *const i8) as i32 != 0 {
        return 0i32 as card16;
    }
    charset = (*cff).charsets;
    gid = 0i32 as card16;
    match (*charset).format as i32 {
        0 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                gid = gid.wrapping_add(1);
                if cff_match_string(cff, glyph, *(*charset).data.glyphs.offset(i as isize)) != 0 {
                    return gid;
                }
                i = i.wrapping_add(1)
            }
        }
        1 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                n = 0i32 as card16;
                while n as i32 <= (*(*charset).data.range1.offset(i as isize)).n_left as i32 {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range1.offset(i as isize)).first as i32 + n as i32)
                            as s_SID,
                    ) != 0
                    {
                        return gid;
                    }
                    n = n.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
        2 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                n = 0i32 as card16;
                while n as i32 <= (*(*charset).data.range2.offset(i as isize)).n_left as i32 {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range2.offset(i as isize)).first as i32 + n as i32)
                            as s_SID,
                    ) != 0
                    {
                        return gid;
                    }
                    n = n.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const i8);
        }
    }
    return 0i32 as card16;
    /* not found, returns .notdef */
}
/* Input : SID or CID (16-bit unsigned int)
 * Output: glyph index
 */
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup(mut cff: *mut cff_font, mut cid: card16) -> card16 {
    if (*cff).flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        _tt_abort(b"Predefined CFF charsets not supported yet\x00" as *const u8 as *const i8);
    } else {
        if (*cff).charsets.is_null() {
            _tt_abort(b"Charsets data not available\x00" as *const u8 as *const i8);
        }
    }
    cff_charsets_lookup_gid((*cff).charsets, cid)
}
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_gid(
    mut charset: *mut cff_charsets,
    mut cid: card16,
) -> card16 {
    let mut gid: card16 = 0i32 as card16;
    let mut i: card16 = 0;
    if cid as i32 == 0i32 {
        return 0i32 as card16;
        /* GID 0 (.notdef) */
    }
    match (*charset).format as i32 {
        0 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                if cid as i32 == *(*charset).data.glyphs.offset(i as isize) as i32 {
                    gid = (i as i32 + 1i32) as card16;
                    return gid;
                }
                i = i.wrapping_add(1)
            }
        }
        1 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                if cid as i32 >= (*(*charset).data.range1.offset(i as isize)).first as i32
                    && cid as i32
                        <= (*(*charset).data.range1.offset(i as isize)).first as i32
                            + (*(*charset).data.range1.offset(i as isize)).n_left as i32
                {
                    gid = (gid as i32
                        + (cid as i32 - (*(*charset).data.range1.offset(i as isize)).first as i32
                            + 1i32)) as card16;
                    return gid;
                }
                gid = (gid as i32
                    + ((*(*charset).data.range1.offset(i as isize)).n_left as i32 + 1i32))
                    as card16;
                i = i.wrapping_add(1)
            }
        }
        2 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                if cid as i32 >= (*(*charset).data.range2.offset(i as isize)).first as i32
                    && cid as i32
                        <= (*(*charset).data.range2.offset(i as isize)).first as i32
                            + (*(*charset).data.range2.offset(i as isize)).n_left as i32
                {
                    gid = (gid as i32
                        + (cid as i32 - (*(*charset).data.range2.offset(i as isize)).first as i32
                            + 1i32)) as card16;
                    return gid;
                }
                gid = (gid as i32
                    + ((*(*charset).data.range2.offset(i as isize)).n_left as i32 + 1i32))
                    as card16;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const i8);
        }
    }
    return 0i32 as card16;
    /* not found */
}
/* Input : GID
 * Output: SID/CID (card16)
 */
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_inverse(
    mut cff: *mut cff_font,
    mut gid: card16,
) -> card16 {
    if (*cff).flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        _tt_abort(b"Predefined CFF charsets not supported yet\x00" as *const u8 as *const i8);
    } else {
        if (*cff).charsets.is_null() {
            _tt_abort(b"Charsets data not available\x00" as *const u8 as *const i8);
        }
    }
    if gid as i32 == 0i32 {
        return 0i32 as card16;
        /* .notdef */
    }
    cff_charsets_lookup_cid((*cff).charsets, gid)
}
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_cid(
    mut charset: *mut cff_charsets,
    mut gid: card16,
) -> card16 {
    let mut sid: card16 = 0i32 as card16;
    let mut i: card16 = 0;
    match (*charset).format as i32 {
        0 => {
            if gid as i32 - 1i32 >= (*charset).num_entries as i32 {
                _tt_abort(b"Invalid GID.\x00" as *const u8 as *const i8);
            }
            sid = *(*charset).data.glyphs.offset((gid as i32 - 1i32) as isize)
        }
        1 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                if gid as i32 <= (*(*charset).data.range1.offset(i as isize)).n_left as i32 + 1i32 {
                    sid = (gid as i32 + (*(*charset).data.range1.offset(i as isize)).first as i32
                        - 1i32) as card16;
                    break;
                } else {
                    gid = (gid as i32
                        - ((*(*charset).data.range1.offset(i as isize)).n_left as i32 + 1i32))
                        as card16;
                    i = i.wrapping_add(1)
                }
            }
            if i as i32 == (*charset).num_entries as i32 {
                _tt_abort(b"Invalid GID\x00" as *const u8 as *const i8);
            }
        }
        2 => {
            i = 0i32 as card16;
            while (i as i32) < (*charset).num_entries as i32 {
                if gid as i32 <= (*(*charset).data.range2.offset(i as isize)).n_left as i32 + 1i32 {
                    sid = (gid as i32 + (*(*charset).data.range2.offset(i as isize)).first as i32
                        - 1i32) as card16;
                    break;
                } else {
                    gid = (gid as i32
                        - ((*(*charset).data.range2.offset(i as isize)).n_left as i32 + 1i32))
                        as card16;
                    i = i.wrapping_add(1)
                }
            }
            if i as i32 == (*charset).num_entries as i32 {
                _tt_abort(b"Invalid GID\x00" as *const u8 as *const i8);
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const i8);
        }
    }
    sid
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_charsets(mut charset: *mut cff_charsets) {
    if !charset.is_null() {
        match (*charset).format as i32 {
            0 => {
                free((*charset).data.glyphs as *mut libc::c_void);
            }
            1 => {
                free((*charset).data.range1 as *mut libc::c_void);
            }
            2 => {
                free((*charset).data.range2 as *mut libc::c_void);
            }
            _ => {}
        }
        free(charset as *mut libc::c_void);
    };
}
/* CID-Keyed font specific */
#[no_mangle]
pub unsafe extern "C" fn cff_read_fdselect(mut cff: *mut cff_font) -> i32 {
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"Top DICT not available\x00" as *const u8 as *const i8);
    }
    if (*cff).flag & 1i32 << 0i32 == 0 {
        return 0i32;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"FDSelect\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    fdsel = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_fdselect>() as u64) as u32)
        as *mut cff_fdselect;
    (*cff).fdselect = fdsel;
    (*fdsel).format = tt_get_unsigned_byte((*cff).handle);
    length = 1i32;
    match (*fdsel).format as i32 {
        0 => {
            (*fdsel).num_entries = (*cff).num_glyphs;
            (*fdsel).data.fds = new(((*fdsel).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<card8>() as u64)
                as u32) as *mut card8;
            i = 0i32 as card16;
            while (i as i32) < (*fdsel).num_entries as i32 {
                *(*fdsel).data.fds.offset(i as isize) = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            length += (*fdsel).num_entries as i32
        }
        3 => {
            let mut ranges: *mut cff_range3 = 0 as *mut cff_range3;
            (*fdsel).num_entries = tt_get_unsigned_pair((*cff).handle);
            ranges = new(((*fdsel).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<cff_range3>() as u64)
                as u32) as *mut cff_range3;
            (*fdsel).data.ranges = ranges;
            i = 0i32 as card16;
            while (i as i32) < (*fdsel).num_entries as i32 {
                (*ranges.offset(i as isize)).first = tt_get_unsigned_pair((*cff).handle);
                (*ranges.offset(i as isize)).fd = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            if (*ranges.offset(0)).first as i32 != 0i32 {
                _tt_abort(b"Range not starting with 0.\x00" as *const u8 as *const i8);
            }
            if (*cff).num_glyphs as i32 != tt_get_unsigned_pair((*cff).handle) as i32 {
                _tt_abort(
                    b"Sentinel value mismatched with number of glyphs.\x00" as *const u8
                        as *const i8,
                );
            }
            length += (*fdsel).num_entries as i32 * 3i32 + 4i32
        }
        _ => {
            free(fdsel as *mut libc::c_void);
            _tt_abort(b"Unknown FDSelect format.\x00" as *const u8 as *const i8);
        }
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_fdselect(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut len: i32 = 0i32;
    let mut i: card16 = 0;
    if (*cff).fdselect.is_null() {
        return 0i32;
    }
    if destlen < 1i32 {
        _tt_abort(b"in cff_pack_fdselect(): Buffur overflow\x00" as *const u8 as *const i8);
    }
    fdsel = (*cff).fdselect;
    let fresh40 = len;
    len = len + 1;
    *dest.offset(fresh40 as isize) = (*fdsel).format;
    match (*fdsel).format as i32 {
        0 => {
            if (*fdsel).num_entries as i32 != (*cff).num_glyphs as i32 {
                _tt_abort(b"in cff_pack_fdselect(): Invalid data\x00" as *const u8 as *const i8);
            }
            if destlen < len + (*fdsel).num_entries as i32 {
                _tt_abort(b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            i = 0i32 as card16;
            while (i as i32) < (*fdsel).num_entries as i32 {
                let fresh41 = len;
                len = len + 1;
                *dest.offset(fresh41 as isize) = *(*fdsel).data.fds.offset(i as isize);
                i = i.wrapping_add(1)
            }
        }
        3 => {
            if destlen < len + 2i32 {
                _tt_abort(b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            len += 2i32;
            i = 0i32 as card16;
            while (i as i32) < (*fdsel).num_entries as i32 {
                if destlen < len + 3i32 {
                    _tt_abort(
                        b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8 as *const i8,
                    );
                }
                let fresh42 = len;
                len = len + 1;
                *dest.offset(fresh42 as isize) =
                    ((*(*fdsel).data.ranges.offset(i as isize)).first as i32 >> 8i32 & 0xffi32)
                        as card8;
                let fresh43 = len;
                len = len + 1;
                *dest.offset(fresh43 as isize) =
                    ((*(*fdsel).data.ranges.offset(i as isize)).first as i32 & 0xffi32) as card8;
                let fresh44 = len;
                len = len + 1;
                *dest.offset(fresh44 as isize) = (*(*fdsel).data.ranges.offset(i as isize)).fd;
                i = i.wrapping_add(1)
            }
            if destlen < len + 2i32 {
                _tt_abort(b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            let fresh45 = len;
            len = len + 1;
            *dest.offset(fresh45 as isize) = ((*cff).num_glyphs as i32 >> 8i32 & 0xffi32) as card8;
            let fresh46 = len;
            len = len + 1;
            *dest.offset(fresh46 as isize) = ((*cff).num_glyphs as i32 & 0xffi32) as card8;
            *dest.offset(1) = (len / 3i32 - 1i32 >> 8i32 & 0xffi32) as card8;
            *dest.offset(2) = (len / 3i32 - 1i32 & 0xffi32) as card8
        }
        _ => {
            _tt_abort(b"Unknown FDSelect format.\x00" as *const u8 as *const i8);
        }
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_fdselect(mut fdselect: *mut cff_fdselect) {
    if !fdselect.is_null() {
        if (*fdselect).format as i32 == 0i32 {
            free((*fdselect).data.fds as *mut libc::c_void);
        } else if (*fdselect).format as i32 == 3i32 {
            free((*fdselect).data.ranges as *mut libc::c_void);
        }
        free(fdselect as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_fdselect_lookup(mut cff: *mut cff_font, mut gid: card16) -> card8 {
    let mut fd: card8 = 0xffi32 as card8;
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    if (*cff).fdselect.is_null() {
        _tt_abort(
            b"in cff_fdselect_lookup(): FDSelect not available\x00" as *const u8 as *const i8,
        );
    }
    fdsel = (*cff).fdselect;
    if gid as i32 >= (*cff).num_glyphs as i32 {
        _tt_abort(b"in cff_fdselect_lookup(): Invalid glyph index\x00" as *const u8 as *const i8);
    }
    match (*fdsel).format as i32 {
        0 => fd = *(*fdsel).data.fds.offset(gid as isize),
        3 => {
            if gid as i32 == 0i32 {
                fd = (*(*fdsel).data.ranges.offset(0)).fd
            } else {
                let mut i: card16 = 0;
                i = 1i32 as card16;
                while (i as i32) < (*fdsel).num_entries as i32 {
                    if (gid as i32) < (*(*fdsel).data.ranges.offset(i as isize)).first as i32 {
                        break;
                    }
                    i = i.wrapping_add(1)
                }
                fd = (*(*fdsel).data.ranges.offset((i as i32 - 1i32) as isize)).fd
            }
        }
        _ => {
            _tt_abort(
                b"in cff_fdselect_lookup(): Invalid FDSelect format\x00" as *const u8 as *const i8,
            );
        }
    }
    if fd as i32 >= (*cff).num_fds as i32 {
        _tt_abort(
            b"in cff_fdselect_lookup(): Invalid Font DICT index\x00" as *const u8 as *const i8,
        );
    }
    fd
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_subrs(mut cff: *mut cff_font) -> i32 {
    let mut len: i32 = 0i32;
    let mut offset: i32 = 0;
    let mut i: i32 = 0;
    if (*cff).flag & 1i32 << 0i32 != 0 && (*cff).fdarray.is_null() {
        cff_read_fdarray(cff);
    }
    if (*cff).private.is_null() {
        cff_read_private(cff);
    }
    if (*cff).gsubr.is_null() {
        ttstub_input_seek(
            (*cff).handle,
            (*cff).offset.wrapping_add((*cff).gsubr_offset) as ssize_t,
            0i32,
        );
        (*cff).gsubr = cff_get_index(cff)
    }
    (*cff).subrs = new(((*cff).num_fds as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*mut cff_index>() as u64) as u32)
        as *mut *mut cff_index;
    if (*cff).flag & 1i32 << 0i32 != 0 {
        i = 0i32;
        while i < (*cff).num_fds as i32 {
            if (*(*cff).private.offset(i as isize)).is_null()
                || cff_dict_known(
                    *(*cff).private.offset(i as isize),
                    b"Subrs\x00" as *const u8 as *const i8,
                ) == 0
            {
                let ref mut fresh47 = *(*cff).subrs.offset(i as isize);
                *fresh47 = 0 as *mut cff_index
            } else {
                offset = cff_dict_get(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                    1i32,
                ) as i32;
                offset = (offset as f64
                    + cff_dict_get(
                        *(*cff).private.offset(i as isize),
                        b"Subrs\x00" as *const u8 as *const i8,
                        0i32,
                    )) as i32;
                ttstub_input_seek(
                    (*cff).handle,
                    (*cff).offset.wrapping_add(offset as u32) as ssize_t,
                    0i32,
                );
                let ref mut fresh48 = *(*cff).subrs.offset(i as isize);
                *fresh48 = cff_get_index(cff);
                len += cff_index_size(*(*cff).subrs.offset(i as isize))
            }
            i += 1
        }
    } else if (*(*cff).private.offset(0)).is_null()
        || cff_dict_known(
            *(*cff).private.offset(0),
            b"Subrs\x00" as *const u8 as *const i8,
        ) == 0
    {
        let ref mut fresh49 = *(*cff).subrs.offset(0);
        *fresh49 = 0 as *mut cff_index
    } else {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Private\x00" as *const u8 as *const i8,
            1i32,
        ) as i32;
        offset = (offset as f64
            + cff_dict_get(
                *(*cff).private.offset(0),
                b"Subrs\x00" as *const u8 as *const i8,
                0i32,
            )) as i32;
        ttstub_input_seek(
            (*cff).handle,
            (*cff).offset.wrapping_add(offset as u32) as ssize_t,
            0i32,
        );
        let ref mut fresh50 = *(*cff).subrs.offset(0);
        *fresh50 = cff_get_index(cff);
        len += cff_index_size(*(*cff).subrs.offset(0))
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_fdarray(mut cff: *mut cff_font) -> i32 {
    let mut len: i32 = 0i32;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut offset: i32 = 0;
    let mut size: i32 = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"in cff_read_fdarray(): Top DICT not found\x00" as *const u8 as *const i8);
    }
    if (*cff).flag & 1i32 << 0i32 == 0 {
        return 0i32;
    }
    /* must exist */
    offset = cff_dict_get(
        (*cff).topdict,
        b"FDArray\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    idx = cff_get_index(cff);
    (*cff).num_fds = (*idx).count as card8;
    (*cff).fdarray = new(((*idx).count as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64)
        as u32) as *mut *mut cff_dict;
    i = 0i32 as card16;
    while (i as i32) < (*idx).count as i32 {
        let mut data: *mut card8 = (*idx)
            .data
            .offset(*(*idx).offset.offset(i as isize) as isize)
            .offset(-1);
        size = (*(*idx).offset.offset((i as i32 + 1i32) as isize))
            .wrapping_sub(*(*idx).offset.offset(i as isize)) as i32;
        if size > 0i32 {
            let ref mut fresh51 = *(*cff).fdarray.offset(i as isize);
            *fresh51 = cff_dict_unpack(data, data.offset(size as isize))
        } else {
            let ref mut fresh52 = *(*cff).fdarray.offset(i as isize);
            *fresh52 = 0 as *mut cff_dict
        }
        i = i.wrapping_add(1)
    }
    len = cff_index_size(idx);
    cff_release_index(idx);
    len
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* Flag */
/* FontName */
/* - CFF structure - */
/* CFF Header */
/* Name INDEX */
/* Top DICT (single) */
/* String INDEX */
/* Global Subr INDEX */
/* Encodings */
/* Charsets  */
/* FDSelect, CIDFont only */
/* CharStrings */
/* CIDFont only */
/* per-Font DICT */
/* Local Subr INDEX, per-Private DICT */
/* -- extra data -- */
/* non-zero for OpenType or PostScript wrapped */
/* number of glyphs (CharString INDEX count) */
/* number of Font DICT */
/* Updated String INDEX.
 * Please fix this. We should separate input and output.
 */
/* not used, ASCII Hex filter if needed */
/* CFF fontset index */
/* Flag: see above */
/* 1 if .notdef is not the 1st glyph */
/* CFF Header */
/* CFF INDEX */
/* Name INDEX */
/* Global and Local Subrs INDEX */
/* Encoding */
/* Charsets */
/* Returns GID of PS name "glyph" */
/* Return PS name of "gid" */
/* Returns GID of glyph with SID/CID "cid" */
/* Returns SID or CID */
/* FDSelect */
/* Font DICT(s) */
/* Private DICT(s) */
#[no_mangle]
pub unsafe extern "C" fn cff_read_private(mut cff: *mut cff_font) -> i32 {
    let mut len: i32 = 0i32;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut offset: i32 = 0;
    let mut size: i32 = 0;
    if (*cff).flag & 1i32 << 0i32 != 0 {
        let mut i: i32 = 0;
        if (*cff).fdarray.is_null() {
            cff_read_fdarray(cff);
        }
        (*cff).private = new(((*cff).num_fds as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64)
            as u32) as *mut *mut cff_dict;
        i = 0i32;
        while i < (*cff).num_fds as i32 {
            if !(*(*cff).fdarray.offset(i as isize)).is_null()
                && cff_dict_known(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                ) != 0
                && {
                    size = cff_dict_get(
                        *(*cff).fdarray.offset(i as isize),
                        b"Private\x00" as *const u8 as *const i8,
                        0i32,
                    ) as i32;
                    size > 0i32
                }
            {
                offset = cff_dict_get(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                    1i32,
                ) as i32;
                ttstub_input_seek(
                    (*cff).handle,
                    (*cff).offset.wrapping_add(offset as u32) as ssize_t,
                    0i32,
                );
                data = new(
                    (size as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32,
                ) as *mut card8;
                if ttstub_input_read((*cff).handle, data as *mut i8, size as size_t) != size as i64
                {
                    _tt_abort(b"reading file failed\x00" as *const u8 as *const i8);
                }
                let ref mut fresh53 = *(*cff).private.offset(i as isize);
                *fresh53 = cff_dict_unpack(data, data.offset(size as isize));
                free(data as *mut libc::c_void);
                len += size
            } else {
                let ref mut fresh54 = *(*cff).private.offset(i as isize);
                *fresh54 = 0 as *mut cff_dict
            }
            i += 1
        }
    } else {
        (*cff).num_fds = 1i32 as card8;
        (*cff).private =
            new((1_u64).wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64) as u32)
                as *mut *mut cff_dict;
        if cff_dict_known((*cff).topdict, b"Private\x00" as *const u8 as *const i8) != 0 && {
            size = cff_dict_get(
                (*cff).topdict,
                b"Private\x00" as *const u8 as *const i8,
                0i32,
            ) as i32;
            size > 0i32
        } {
            offset = cff_dict_get(
                (*cff).topdict,
                b"Private\x00" as *const u8 as *const i8,
                1i32,
            ) as i32;
            ttstub_input_seek(
                (*cff).handle,
                (*cff).offset.wrapping_add(offset as u32) as ssize_t,
                0i32,
            );
            data = new(
                (size as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32,
            ) as *mut card8;
            if ttstub_input_read((*cff).handle, data as *mut i8, size as size_t) != size as i64 {
                _tt_abort(b"reading file failed\x00" as *const u8 as *const i8);
            }
            let ref mut fresh55 = *(*cff).private.offset(0);
            *fresh55 = cff_dict_unpack(data, data.offset(size as isize));
            free(data as *mut libc::c_void);
            len += size
        } else {
            let ref mut fresh56 = *(*cff).private.offset(0);
            *fresh56 = 0 as *mut cff_dict;
            len = 0i32
        }
    }
    len
}
