#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> libc::c_uchar;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> libc::c_ushort;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn cff_release_dict(dict: *mut cff_dict);
    #[no_mangle]
    fn cff_dict_get(
        dict: *mut cff_dict,
        key: *const libc::c_char,
        idx: libc::c_int,
    ) -> libc::c_double;
    #[no_mangle]
    fn cff_dict_known(dict: *mut cff_dict, key: *const libc::c_char) -> libc::c_int;
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
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: uint32_t) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
pub type card8 = libc::c_uchar;
/* 1-byte unsigned number */
pub type card16 = libc::c_ushort;
/* 2-byte unsigned number */
pub type c_offsize = libc::c_uchar;
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
pub type l_offset = uint32_t;
/* 1, 2, 3, or 4-byte offset */
pub type s_SID = libc::c_ushort;
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
    pub id: libc::c_int,
    pub key: *const libc::c_char,
    pub count: libc::c_int,
    pub values: *mut libc::c_double,
    /* values                                  */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: libc::c_int,
    pub count: libc::c_int,
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
    pub fontname: *mut libc::c_char,
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
    pub filter: libc::c_int,
    pub index: libc::c_int,
    pub flag: libc::c_int,
    pub is_notdef_notzero: libc::c_int,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
static mut cff_stdstr: [*const libc::c_char; 391] = [
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"space\x00" as *const u8 as *const libc::c_char,
    b"exclam\x00" as *const u8 as *const libc::c_char,
    b"quotedbl\x00" as *const u8 as *const libc::c_char,
    b"numbersign\x00" as *const u8 as *const libc::c_char,
    b"dollar\x00" as *const u8 as *const libc::c_char,
    b"percent\x00" as *const u8 as *const libc::c_char,
    b"ampersand\x00" as *const u8 as *const libc::c_char,
    b"quoteright\x00" as *const u8 as *const libc::c_char,
    b"parenleft\x00" as *const u8 as *const libc::c_char,
    b"parenright\x00" as *const u8 as *const libc::c_char,
    b"asterisk\x00" as *const u8 as *const libc::c_char,
    b"plus\x00" as *const u8 as *const libc::c_char,
    b"comma\x00" as *const u8 as *const libc::c_char,
    b"hyphen\x00" as *const u8 as *const libc::c_char,
    b"period\x00" as *const u8 as *const libc::c_char,
    b"slash\x00" as *const u8 as *const libc::c_char,
    b"zero\x00" as *const u8 as *const libc::c_char,
    b"one\x00" as *const u8 as *const libc::c_char,
    b"two\x00" as *const u8 as *const libc::c_char,
    b"three\x00" as *const u8 as *const libc::c_char,
    b"four\x00" as *const u8 as *const libc::c_char,
    b"five\x00" as *const u8 as *const libc::c_char,
    b"six\x00" as *const u8 as *const libc::c_char,
    b"seven\x00" as *const u8 as *const libc::c_char,
    b"eight\x00" as *const u8 as *const libc::c_char,
    b"nine\x00" as *const u8 as *const libc::c_char,
    b"colon\x00" as *const u8 as *const libc::c_char,
    b"semicolon\x00" as *const u8 as *const libc::c_char,
    b"less\x00" as *const u8 as *const libc::c_char,
    b"equal\x00" as *const u8 as *const libc::c_char,
    b"greater\x00" as *const u8 as *const libc::c_char,
    b"question\x00" as *const u8 as *const libc::c_char,
    b"at\x00" as *const u8 as *const libc::c_char,
    b"A\x00" as *const u8 as *const libc::c_char,
    b"B\x00" as *const u8 as *const libc::c_char,
    b"C\x00" as *const u8 as *const libc::c_char,
    b"D\x00" as *const u8 as *const libc::c_char,
    b"E\x00" as *const u8 as *const libc::c_char,
    b"F\x00" as *const u8 as *const libc::c_char,
    b"G\x00" as *const u8 as *const libc::c_char,
    b"H\x00" as *const u8 as *const libc::c_char,
    b"I\x00" as *const u8 as *const libc::c_char,
    b"J\x00" as *const u8 as *const libc::c_char,
    b"K\x00" as *const u8 as *const libc::c_char,
    b"L\x00" as *const u8 as *const libc::c_char,
    b"M\x00" as *const u8 as *const libc::c_char,
    b"N\x00" as *const u8 as *const libc::c_char,
    b"O\x00" as *const u8 as *const libc::c_char,
    b"P\x00" as *const u8 as *const libc::c_char,
    b"Q\x00" as *const u8 as *const libc::c_char,
    b"R\x00" as *const u8 as *const libc::c_char,
    b"S\x00" as *const u8 as *const libc::c_char,
    b"T\x00" as *const u8 as *const libc::c_char,
    b"U\x00" as *const u8 as *const libc::c_char,
    b"V\x00" as *const u8 as *const libc::c_char,
    b"W\x00" as *const u8 as *const libc::c_char,
    b"X\x00" as *const u8 as *const libc::c_char,
    b"Y\x00" as *const u8 as *const libc::c_char,
    b"Z\x00" as *const u8 as *const libc::c_char,
    b"bracketleft\x00" as *const u8 as *const libc::c_char,
    b"backslash\x00" as *const u8 as *const libc::c_char,
    b"bracketright\x00" as *const u8 as *const libc::c_char,
    b"asciicircum\x00" as *const u8 as *const libc::c_char,
    b"underscore\x00" as *const u8 as *const libc::c_char,
    b"quoteleft\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"c\x00" as *const u8 as *const libc::c_char,
    b"d\x00" as *const u8 as *const libc::c_char,
    b"e\x00" as *const u8 as *const libc::c_char,
    b"f\x00" as *const u8 as *const libc::c_char,
    b"g\x00" as *const u8 as *const libc::c_char,
    b"h\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"j\x00" as *const u8 as *const libc::c_char,
    b"k\x00" as *const u8 as *const libc::c_char,
    b"l\x00" as *const u8 as *const libc::c_char,
    b"m\x00" as *const u8 as *const libc::c_char,
    b"n\x00" as *const u8 as *const libc::c_char,
    b"o\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"r\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"t\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"v\x00" as *const u8 as *const libc::c_char,
    b"w\x00" as *const u8 as *const libc::c_char,
    b"x\x00" as *const u8 as *const libc::c_char,
    b"y\x00" as *const u8 as *const libc::c_char,
    b"z\x00" as *const u8 as *const libc::c_char,
    b"braceleft\x00" as *const u8 as *const libc::c_char,
    b"bar\x00" as *const u8 as *const libc::c_char,
    b"braceright\x00" as *const u8 as *const libc::c_char,
    b"asciitilde\x00" as *const u8 as *const libc::c_char,
    b"exclamdown\x00" as *const u8 as *const libc::c_char,
    b"cent\x00" as *const u8 as *const libc::c_char,
    b"sterling\x00" as *const u8 as *const libc::c_char,
    b"fraction\x00" as *const u8 as *const libc::c_char,
    b"yen\x00" as *const u8 as *const libc::c_char,
    b"florin\x00" as *const u8 as *const libc::c_char,
    b"section\x00" as *const u8 as *const libc::c_char,
    b"currency\x00" as *const u8 as *const libc::c_char,
    b"quotesingle\x00" as *const u8 as *const libc::c_char,
    b"quotedblleft\x00" as *const u8 as *const libc::c_char,
    b"guillemotleft\x00" as *const u8 as *const libc::c_char,
    b"guilsinglleft\x00" as *const u8 as *const libc::c_char,
    b"guilsinglright\x00" as *const u8 as *const libc::c_char,
    b"fi\x00" as *const u8 as *const libc::c_char,
    b"fl\x00" as *const u8 as *const libc::c_char,
    b"endash\x00" as *const u8 as *const libc::c_char,
    b"dagger\x00" as *const u8 as *const libc::c_char,
    b"daggerdbl\x00" as *const u8 as *const libc::c_char,
    b"periodcentered\x00" as *const u8 as *const libc::c_char,
    b"paragraph\x00" as *const u8 as *const libc::c_char,
    b"bullet\x00" as *const u8 as *const libc::c_char,
    b"quotesinglbase\x00" as *const u8 as *const libc::c_char,
    b"quotedblbase\x00" as *const u8 as *const libc::c_char,
    b"quotedblright\x00" as *const u8 as *const libc::c_char,
    b"guillemotright\x00" as *const u8 as *const libc::c_char,
    b"ellipsis\x00" as *const u8 as *const libc::c_char,
    b"perthousand\x00" as *const u8 as *const libc::c_char,
    b"questiondown\x00" as *const u8 as *const libc::c_char,
    b"grave\x00" as *const u8 as *const libc::c_char,
    b"acute\x00" as *const u8 as *const libc::c_char,
    b"circumflex\x00" as *const u8 as *const libc::c_char,
    b"tilde\x00" as *const u8 as *const libc::c_char,
    b"macron\x00" as *const u8 as *const libc::c_char,
    b"breve\x00" as *const u8 as *const libc::c_char,
    b"dotaccent\x00" as *const u8 as *const libc::c_char,
    b"dieresis\x00" as *const u8 as *const libc::c_char,
    b"ring\x00" as *const u8 as *const libc::c_char,
    b"cedilla\x00" as *const u8 as *const libc::c_char,
    b"hungarumlaut\x00" as *const u8 as *const libc::c_char,
    b"ogonek\x00" as *const u8 as *const libc::c_char,
    b"caron\x00" as *const u8 as *const libc::c_char,
    b"emdash\x00" as *const u8 as *const libc::c_char,
    b"AE\x00" as *const u8 as *const libc::c_char,
    b"ordfeminine\x00" as *const u8 as *const libc::c_char,
    b"Lslash\x00" as *const u8 as *const libc::c_char,
    b"Oslash\x00" as *const u8 as *const libc::c_char,
    b"OE\x00" as *const u8 as *const libc::c_char,
    b"ordmasculine\x00" as *const u8 as *const libc::c_char,
    b"ae\x00" as *const u8 as *const libc::c_char,
    b"dotlessi\x00" as *const u8 as *const libc::c_char,
    b"lslash\x00" as *const u8 as *const libc::c_char,
    b"oslash\x00" as *const u8 as *const libc::c_char,
    b"oe\x00" as *const u8 as *const libc::c_char,
    b"germandbls\x00" as *const u8 as *const libc::c_char,
    b"onesuperior\x00" as *const u8 as *const libc::c_char,
    b"logicalnot\x00" as *const u8 as *const libc::c_char,
    b"mu\x00" as *const u8 as *const libc::c_char,
    b"trademark\x00" as *const u8 as *const libc::c_char,
    b"Eth\x00" as *const u8 as *const libc::c_char,
    b"onehalf\x00" as *const u8 as *const libc::c_char,
    b"plusminus\x00" as *const u8 as *const libc::c_char,
    b"Thorn\x00" as *const u8 as *const libc::c_char,
    b"onequarter\x00" as *const u8 as *const libc::c_char,
    b"divide\x00" as *const u8 as *const libc::c_char,
    b"brokenbar\x00" as *const u8 as *const libc::c_char,
    b"degree\x00" as *const u8 as *const libc::c_char,
    b"thorn\x00" as *const u8 as *const libc::c_char,
    b"threequarters\x00" as *const u8 as *const libc::c_char,
    b"twosuperior\x00" as *const u8 as *const libc::c_char,
    b"registered\x00" as *const u8 as *const libc::c_char,
    b"minus\x00" as *const u8 as *const libc::c_char,
    b"eth\x00" as *const u8 as *const libc::c_char,
    b"multiply\x00" as *const u8 as *const libc::c_char,
    b"threesuperior\x00" as *const u8 as *const libc::c_char,
    b"copyright\x00" as *const u8 as *const libc::c_char,
    b"Aacute\x00" as *const u8 as *const libc::c_char,
    b"Acircumflex\x00" as *const u8 as *const libc::c_char,
    b"Adieresis\x00" as *const u8 as *const libc::c_char,
    b"Agrave\x00" as *const u8 as *const libc::c_char,
    b"Aring\x00" as *const u8 as *const libc::c_char,
    b"Atilde\x00" as *const u8 as *const libc::c_char,
    b"Ccedilla\x00" as *const u8 as *const libc::c_char,
    b"Eacute\x00" as *const u8 as *const libc::c_char,
    b"Ecircumflex\x00" as *const u8 as *const libc::c_char,
    b"Edieresis\x00" as *const u8 as *const libc::c_char,
    b"Egrave\x00" as *const u8 as *const libc::c_char,
    b"Iacute\x00" as *const u8 as *const libc::c_char,
    b"Icircumflex\x00" as *const u8 as *const libc::c_char,
    b"Idieresis\x00" as *const u8 as *const libc::c_char,
    b"Igrave\x00" as *const u8 as *const libc::c_char,
    b"Ntilde\x00" as *const u8 as *const libc::c_char,
    b"Oacute\x00" as *const u8 as *const libc::c_char,
    b"Ocircumflex\x00" as *const u8 as *const libc::c_char,
    b"Odieresis\x00" as *const u8 as *const libc::c_char,
    b"Ograve\x00" as *const u8 as *const libc::c_char,
    b"Otilde\x00" as *const u8 as *const libc::c_char,
    b"Scaron\x00" as *const u8 as *const libc::c_char,
    b"Uacute\x00" as *const u8 as *const libc::c_char,
    b"Ucircumflex\x00" as *const u8 as *const libc::c_char,
    b"Udieresis\x00" as *const u8 as *const libc::c_char,
    b"Ugrave\x00" as *const u8 as *const libc::c_char,
    b"Yacute\x00" as *const u8 as *const libc::c_char,
    b"Ydieresis\x00" as *const u8 as *const libc::c_char,
    b"Zcaron\x00" as *const u8 as *const libc::c_char,
    b"aacute\x00" as *const u8 as *const libc::c_char,
    b"acircumflex\x00" as *const u8 as *const libc::c_char,
    b"adieresis\x00" as *const u8 as *const libc::c_char,
    b"agrave\x00" as *const u8 as *const libc::c_char,
    b"aring\x00" as *const u8 as *const libc::c_char,
    b"atilde\x00" as *const u8 as *const libc::c_char,
    b"ccedilla\x00" as *const u8 as *const libc::c_char,
    b"eacute\x00" as *const u8 as *const libc::c_char,
    b"ecircumflex\x00" as *const u8 as *const libc::c_char,
    b"edieresis\x00" as *const u8 as *const libc::c_char,
    b"egrave\x00" as *const u8 as *const libc::c_char,
    b"iacute\x00" as *const u8 as *const libc::c_char,
    b"icircumflex\x00" as *const u8 as *const libc::c_char,
    b"idieresis\x00" as *const u8 as *const libc::c_char,
    b"igrave\x00" as *const u8 as *const libc::c_char,
    b"ntilde\x00" as *const u8 as *const libc::c_char,
    b"oacute\x00" as *const u8 as *const libc::c_char,
    b"ocircumflex\x00" as *const u8 as *const libc::c_char,
    b"odieresis\x00" as *const u8 as *const libc::c_char,
    b"ograve\x00" as *const u8 as *const libc::c_char,
    b"otilde\x00" as *const u8 as *const libc::c_char,
    b"scaron\x00" as *const u8 as *const libc::c_char,
    b"uacute\x00" as *const u8 as *const libc::c_char,
    b"ucircumflex\x00" as *const u8 as *const libc::c_char,
    b"udieresis\x00" as *const u8 as *const libc::c_char,
    b"ugrave\x00" as *const u8 as *const libc::c_char,
    b"yacute\x00" as *const u8 as *const libc::c_char,
    b"ydieresis\x00" as *const u8 as *const libc::c_char,
    b"zcaron\x00" as *const u8 as *const libc::c_char,
    b"exclamsmall\x00" as *const u8 as *const libc::c_char,
    b"Hungarumlautsmall\x00" as *const u8 as *const libc::c_char,
    b"dollaroldstyle\x00" as *const u8 as *const libc::c_char,
    b"dollarsuperior\x00" as *const u8 as *const libc::c_char,
    b"ampersandsmall\x00" as *const u8 as *const libc::c_char,
    b"Acutesmall\x00" as *const u8 as *const libc::c_char,
    b"parenleftsuperior\x00" as *const u8 as *const libc::c_char,
    b"parenrightsuperior\x00" as *const u8 as *const libc::c_char,
    b"twodotenleader\x00" as *const u8 as *const libc::c_char,
    b"onedotenleader\x00" as *const u8 as *const libc::c_char,
    b"zerooldstyle\x00" as *const u8 as *const libc::c_char,
    b"oneoldstyle\x00" as *const u8 as *const libc::c_char,
    b"twooldstyle\x00" as *const u8 as *const libc::c_char,
    b"threeoldstyle\x00" as *const u8 as *const libc::c_char,
    b"fouroldstyle\x00" as *const u8 as *const libc::c_char,
    b"fiveoldstyle\x00" as *const u8 as *const libc::c_char,
    b"sixoldstyle\x00" as *const u8 as *const libc::c_char,
    b"sevenoldstyle\x00" as *const u8 as *const libc::c_char,
    b"eightoldstyle\x00" as *const u8 as *const libc::c_char,
    b"nineoldstyle\x00" as *const u8 as *const libc::c_char,
    b"commasuperior\x00" as *const u8 as *const libc::c_char,
    b"threequartersemdash\x00" as *const u8 as *const libc::c_char,
    b"periodsuperior\x00" as *const u8 as *const libc::c_char,
    b"questionsmall\x00" as *const u8 as *const libc::c_char,
    b"asuperior\x00" as *const u8 as *const libc::c_char,
    b"bsuperior\x00" as *const u8 as *const libc::c_char,
    b"centsuperior\x00" as *const u8 as *const libc::c_char,
    b"dsuperior\x00" as *const u8 as *const libc::c_char,
    b"esuperior\x00" as *const u8 as *const libc::c_char,
    b"isuperior\x00" as *const u8 as *const libc::c_char,
    b"lsuperior\x00" as *const u8 as *const libc::c_char,
    b"msuperior\x00" as *const u8 as *const libc::c_char,
    b"nsuperior\x00" as *const u8 as *const libc::c_char,
    b"osuperior\x00" as *const u8 as *const libc::c_char,
    b"rsuperior\x00" as *const u8 as *const libc::c_char,
    b"ssuperior\x00" as *const u8 as *const libc::c_char,
    b"tsuperior\x00" as *const u8 as *const libc::c_char,
    b"ff\x00" as *const u8 as *const libc::c_char,
    b"ffi\x00" as *const u8 as *const libc::c_char,
    b"ffl\x00" as *const u8 as *const libc::c_char,
    b"parenleftinferior\x00" as *const u8 as *const libc::c_char,
    b"parenrightinferior\x00" as *const u8 as *const libc::c_char,
    b"Circumflexsmall\x00" as *const u8 as *const libc::c_char,
    b"hyphensuperior\x00" as *const u8 as *const libc::c_char,
    b"Gravesmall\x00" as *const u8 as *const libc::c_char,
    b"Asmall\x00" as *const u8 as *const libc::c_char,
    b"Bsmall\x00" as *const u8 as *const libc::c_char,
    b"Csmall\x00" as *const u8 as *const libc::c_char,
    b"Dsmall\x00" as *const u8 as *const libc::c_char,
    b"Esmall\x00" as *const u8 as *const libc::c_char,
    b"Fsmall\x00" as *const u8 as *const libc::c_char,
    b"Gsmall\x00" as *const u8 as *const libc::c_char,
    b"Hsmall\x00" as *const u8 as *const libc::c_char,
    b"Ismall\x00" as *const u8 as *const libc::c_char,
    b"Jsmall\x00" as *const u8 as *const libc::c_char,
    b"Ksmall\x00" as *const u8 as *const libc::c_char,
    b"Lsmall\x00" as *const u8 as *const libc::c_char,
    b"Msmall\x00" as *const u8 as *const libc::c_char,
    b"Nsmall\x00" as *const u8 as *const libc::c_char,
    b"Osmall\x00" as *const u8 as *const libc::c_char,
    b"Psmall\x00" as *const u8 as *const libc::c_char,
    b"Qsmall\x00" as *const u8 as *const libc::c_char,
    b"Rsmall\x00" as *const u8 as *const libc::c_char,
    b"Ssmall\x00" as *const u8 as *const libc::c_char,
    b"Tsmall\x00" as *const u8 as *const libc::c_char,
    b"Usmall\x00" as *const u8 as *const libc::c_char,
    b"Vsmall\x00" as *const u8 as *const libc::c_char,
    b"Wsmall\x00" as *const u8 as *const libc::c_char,
    b"Xsmall\x00" as *const u8 as *const libc::c_char,
    b"Ysmall\x00" as *const u8 as *const libc::c_char,
    b"Zsmall\x00" as *const u8 as *const libc::c_char,
    b"colonmonetary\x00" as *const u8 as *const libc::c_char,
    b"onefitted\x00" as *const u8 as *const libc::c_char,
    b"rupiah\x00" as *const u8 as *const libc::c_char,
    b"Tildesmall\x00" as *const u8 as *const libc::c_char,
    b"exclamdownsmall\x00" as *const u8 as *const libc::c_char,
    b"centoldstyle\x00" as *const u8 as *const libc::c_char,
    b"Lslashsmall\x00" as *const u8 as *const libc::c_char,
    b"Scaronsmall\x00" as *const u8 as *const libc::c_char,
    b"Zcaronsmall\x00" as *const u8 as *const libc::c_char,
    b"Dieresissmall\x00" as *const u8 as *const libc::c_char,
    b"Brevesmall\x00" as *const u8 as *const libc::c_char,
    b"Caronsmall\x00" as *const u8 as *const libc::c_char,
    b"Dotaccentsmall\x00" as *const u8 as *const libc::c_char,
    b"Macronsmall\x00" as *const u8 as *const libc::c_char,
    b"figuredash\x00" as *const u8 as *const libc::c_char,
    b"hypheninferior\x00" as *const u8 as *const libc::c_char,
    b"Ogoneksmall\x00" as *const u8 as *const libc::c_char,
    b"Ringsmall\x00" as *const u8 as *const libc::c_char,
    b"Cedillasmall\x00" as *const u8 as *const libc::c_char,
    b"questiondownsmall\x00" as *const u8 as *const libc::c_char,
    b"oneeighth\x00" as *const u8 as *const libc::c_char,
    b"threeeighths\x00" as *const u8 as *const libc::c_char,
    b"fiveeighths\x00" as *const u8 as *const libc::c_char,
    b"seveneighths\x00" as *const u8 as *const libc::c_char,
    b"onethird\x00" as *const u8 as *const libc::c_char,
    b"twothirds\x00" as *const u8 as *const libc::c_char,
    b"zerosuperior\x00" as *const u8 as *const libc::c_char,
    b"foursuperior\x00" as *const u8 as *const libc::c_char,
    b"fivesuperior\x00" as *const u8 as *const libc::c_char,
    b"sixsuperior\x00" as *const u8 as *const libc::c_char,
    b"sevensuperior\x00" as *const u8 as *const libc::c_char,
    b"eightsuperior\x00" as *const u8 as *const libc::c_char,
    b"ninesuperior\x00" as *const u8 as *const libc::c_char,
    b"zeroinferior\x00" as *const u8 as *const libc::c_char,
    b"oneinferior\x00" as *const u8 as *const libc::c_char,
    b"twoinferior\x00" as *const u8 as *const libc::c_char,
    b"threeinferior\x00" as *const u8 as *const libc::c_char,
    b"fourinferior\x00" as *const u8 as *const libc::c_char,
    b"fiveinferior\x00" as *const u8 as *const libc::c_char,
    b"sixinferior\x00" as *const u8 as *const libc::c_char,
    b"seveninferior\x00" as *const u8 as *const libc::c_char,
    b"eightinferior\x00" as *const u8 as *const libc::c_char,
    b"nineinferior\x00" as *const u8 as *const libc::c_char,
    b"centinferior\x00" as *const u8 as *const libc::c_char,
    b"dollarinferior\x00" as *const u8 as *const libc::c_char,
    b"periodinferior\x00" as *const u8 as *const libc::c_char,
    b"commainferior\x00" as *const u8 as *const libc::c_char,
    b"Agravesmall\x00" as *const u8 as *const libc::c_char,
    b"Aacutesmall\x00" as *const u8 as *const libc::c_char,
    b"Acircumflexsmall\x00" as *const u8 as *const libc::c_char,
    b"Atildesmall\x00" as *const u8 as *const libc::c_char,
    b"Adieresissmall\x00" as *const u8 as *const libc::c_char,
    b"Aringsmall\x00" as *const u8 as *const libc::c_char,
    b"AEsmall\x00" as *const u8 as *const libc::c_char,
    b"Ccedillasmall\x00" as *const u8 as *const libc::c_char,
    b"Egravesmall\x00" as *const u8 as *const libc::c_char,
    b"Eacutesmall\x00" as *const u8 as *const libc::c_char,
    b"Ecircumflexsmall\x00" as *const u8 as *const libc::c_char,
    b"Edieresissmall\x00" as *const u8 as *const libc::c_char,
    b"Igravesmall\x00" as *const u8 as *const libc::c_char,
    b"Iacutesmall\x00" as *const u8 as *const libc::c_char,
    b"Icircumflexsmall\x00" as *const u8 as *const libc::c_char,
    b"Idieresissmall\x00" as *const u8 as *const libc::c_char,
    b"Ethsmall\x00" as *const u8 as *const libc::c_char,
    b"Ntildesmall\x00" as *const u8 as *const libc::c_char,
    b"Ogravesmall\x00" as *const u8 as *const libc::c_char,
    b"Oacutesmall\x00" as *const u8 as *const libc::c_char,
    b"Ocircumflexsmall\x00" as *const u8 as *const libc::c_char,
    b"Otildesmall\x00" as *const u8 as *const libc::c_char,
    b"Odieresissmall\x00" as *const u8 as *const libc::c_char,
    b"OEsmall\x00" as *const u8 as *const libc::c_char,
    b"Oslashsmall\x00" as *const u8 as *const libc::c_char,
    b"Ugravesmall\x00" as *const u8 as *const libc::c_char,
    b"Uacutesmall\x00" as *const u8 as *const libc::c_char,
    b"Ucircumflexsmall\x00" as *const u8 as *const libc::c_char,
    b"Udieresissmall\x00" as *const u8 as *const libc::c_char,
    b"Yacutesmall\x00" as *const u8 as *const libc::c_char,
    b"Thornsmall\x00" as *const u8 as *const libc::c_char,
    b"Ydieresissmall\x00" as *const u8 as *const libc::c_char,
    b"001.000\x00" as *const u8 as *const libc::c_char,
    b"001.001\x00" as *const u8 as *const libc::c_char,
    b"001.002\x00" as *const u8 as *const libc::c_char,
    b"001.003\x00" as *const u8 as *const libc::c_char,
    b"Black\x00" as *const u8 as *const libc::c_char,
    b"Bold\x00" as *const u8 as *const libc::c_char,
    b"Book\x00" as *const u8 as *const libc::c_char,
    b"Light\x00" as *const u8 as *const libc::c_char,
    b"Medium\x00" as *const u8 as *const libc::c_char,
    b"Regular\x00" as *const u8 as *const libc::c_char,
    b"Roman\x00" as *const u8 as *const libc::c_char,
    b"Semibold\x00" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn get_unsigned(
    mut handle: rust_input_handle_t,
    mut n: libc::c_int,
) -> libc::c_uint {
    let mut v: libc::c_uint = 0i32 as libc::c_uint;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0i32) {
            break;
        }
        v = v
            .wrapping_mul(0x100u32)
            .wrapping_add(tt_get_unsigned_byte(handle) as libc::c_uint)
    }
    return v;
}
/*
 * Read Header, Name INDEX, Top DICT INDEX, and String INDEX.
 */
#[no_mangle]
pub unsafe extern "C" fn cff_open(
    mut handle: rust_input_handle_t,
    mut offset: libc::c_int,
    mut n: libc::c_int,
) -> *mut cff_font {
    let mut cff: *mut cff_font = 0 as *mut cff_font; /* not used */
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    cff = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_font>() as libc::c_ulong) as uint32_t)
        as *mut cff_font;
    (*cff).fontname = 0 as *mut libc::c_char;
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
        (*cff).offset.wrapping_add(0i32 as libc::c_uint) as ssize_t,
        0i32,
    );
    (*cff).header.major = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.minor = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.hdr_size = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.offsize = tt_get_unsigned_byte((*cff).handle);
    if ((*cff).header.offsize as libc::c_int) < 1i32 || (*cff).header.offsize as libc::c_int > 4i32
    {
        _tt_abort(b"invalid offsize data\x00" as *const u8 as *const libc::c_char);
    }
    if (*cff).header.major as libc::c_int > 1i32 || (*cff).header.minor as libc::c_int > 0i32 {
        dpx_warning(
            b"%s: CFF version %u.%u not supported.\x00" as *const u8 as *const libc::c_char,
            b"CFF\x00" as *const u8 as *const libc::c_char,
            (*cff).header.major as libc::c_int,
            (*cff).header.minor as libc::c_int,
        );
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    ttstub_input_seek(
        (*cff).handle,
        (*cff)
            .offset
            .wrapping_add((*cff).header.hdr_size as libc::c_uint) as ssize_t,
        0i32,
    );
    /* Name INDEX */
    idx = cff_get_index(cff);
    if n > (*idx).count as libc::c_int - 1i32 {
        dpx_warning(
            b"%s: Invalid CFF fontset index number.\x00" as *const u8 as *const libc::c_char,
            b"CFF\x00" as *const u8 as *const libc::c_char,
        );
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    (*cff).name = idx;
    (*cff).fontname = cff_get_name(cff);
    /* Top DICT INDEX */
    idx = cff_get_index(cff);
    if n > (*idx).count as libc::c_int - 1i32 {
        _tt_abort(b"CFF Top DICT not exist...\x00" as *const u8 as *const libc::c_char);
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
        _tt_abort(b"Parsing CFF Top DICT data failed...\x00" as *const u8 as *const libc::c_char);
    }
    cff_release_index(idx);
    if cff_dict_known(
        (*cff).topdict,
        b"CharstringType\x00" as *const u8 as *const libc::c_char,
    ) != 0
        && cff_dict_get(
            (*cff).topdict,
            b"CharstringType\x00" as *const u8 as *const libc::c_char,
            0i32,
        ) != 2i32 as libc::c_double
    {
        dpx_warning(
            b"Only Type 2 Charstrings supported...\x00" as *const u8 as *const libc::c_char,
        );
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    if cff_dict_known(
        (*cff).topdict,
        b"SyntheticBase\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        dpx_warning(b"CFF Synthetic font not supported.\x00" as *const u8 as *const libc::c_char);
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    /* String INDEX */
    (*cff).string = cff_get_index(cff);
    /* offset to GSubr */
    (*cff).gsubr_offset = ttstub_input_seek((*cff).handle, 0i32 as ssize_t, 1i32)
        .wrapping_sub(offset as libc::c_ulong) as l_offset;
    /* Number of glyphs */
    offset = cff_dict_get(
        (*cff).topdict,
        b"CharStrings\x00" as *const u8 as *const libc::c_char,
        0i32,
    ) as libc::c_int;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
        0i32,
    );
    (*cff).num_glyphs = tt_get_unsigned_pair((*cff).handle);
    /* Check for font type */
    if cff_dict_known(
        (*cff).topdict,
        b"ROS\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        (*cff).flag |= 1i32 << 0i32
    } else {
        (*cff).flag |= 1i32 << 1i32
    }
    /* Check for encoding */
    if cff_dict_known(
        (*cff).topdict,
        b"Encoding\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Encoding\x00" as *const u8 as *const libc::c_char,
            0i32,
        ) as libc::c_int;
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
    if cff_dict_known(
        (*cff).topdict,
        b"charset\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        offset = cff_dict_get(
            (*cff).topdict,
            b"charset\x00" as *const u8 as *const libc::c_char,
            0i32,
        ) as libc::c_int;
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
            while (i as libc::c_int) < (*cff).num_fds as libc::c_int {
                if !(*(*cff).fdarray.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).fdarray.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).fdarray as *mut libc::c_void);
        }
        if !(*cff).private.is_null() {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*cff).num_fds as libc::c_int {
                if !(*(*cff).private.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).private.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).private as *mut libc::c_void);
        }
        if !(*cff).subrs.is_null() {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*cff).num_fds as libc::c_int {
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
pub unsafe extern "C" fn cff_get_name(mut cff: *mut cff_font) -> *mut libc::c_char {
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: l_offset = 0;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    idx = (*cff).name;
    len = (*(*idx).offset.offset(((*cff).index + 1i32) as isize))
        .wrapping_sub(*(*idx).offset.offset((*cff).index as isize));
    fontname = new((len.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    memcpy(
        fontname as *mut libc::c_void,
        (*idx)
            .data
            .offset(*(*idx).offset.offset((*cff).index as isize) as isize)
            .offset(-1) as *const libc::c_void,
        len as libc::c_ulong,
    );
    *fontname.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    return fontname;
}
#[no_mangle]
pub unsafe extern "C" fn cff_set_name(
    mut cff: *mut cff_font,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    if strlen(name) > 127i32 as libc::c_ulong {
        _tt_abort(b"FontName string length too large...\x00" as *const u8 as *const libc::c_char);
    }
    if !(*cff).name.is_null() {
        cff_release_index((*cff).name);
    }
    idx = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_index>() as libc::c_ulong)
        as uint32_t) as *mut cff_index;
    (*cff).name = idx;
    (*idx).count = 1i32 as card16;
    (*idx).offsize = 1i32 as c_offsize;
    (*idx).offset = new((2i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<l_offset>() as libc::c_ulong)
        as uint32_t) as *mut l_offset;
    *(*idx).offset.offset(0) = 1i32 as l_offset;
    *(*idx).offset.offset(1) = strlen(name).wrapping_add(1i32 as libc::c_ulong) as l_offset;
    (*idx).data = new((strlen(name) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
        as uint32_t) as *mut card8;
    memmove(
        (*idx).data as *mut libc::c_void,
        name as *const libc::c_void,
        strlen(name),
    );
    return (5i32 as libc::c_ulong).wrapping_add(strlen(name)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cff_put_header(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: libc::c_int,
) -> libc::c_int {
    if destlen < 4i32 {
        _tt_abort(b"Not enough space available...\x00" as *const u8 as *const libc::c_char);
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
    return 4i32;
}
/* Only read header part but not body */
#[no_mangle]
pub unsafe extern "C" fn cff_get_index_header(mut cff: *mut cff_font) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut i: card16 = 0;
    let mut count: card16 = 0;
    idx = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_index>() as libc::c_ulong)
        as uint32_t) as *mut cff_index;
    count = tt_get_unsigned_pair((*cff).handle);
    (*idx).count = count;
    if count as libc::c_int > 0i32 {
        (*idx).offsize = tt_get_unsigned_byte((*cff).handle);
        if ((*idx).offsize as libc::c_int) < 1i32 || (*idx).offsize as libc::c_int > 4i32 {
            _tt_abort(b"invalid offsize data\x00" as *const u8 as *const libc::c_char);
        }
        (*idx).offset = new(((count as libc::c_int + 1i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as libc::c_ulong)
            as uint32_t) as *mut l_offset;
        i = 0i32 as card16;
        while (i as libc::c_int) < count as libc::c_int {
            *(*idx).offset.offset(i as isize) =
                get_unsigned((*cff).handle, (*idx).offsize as libc::c_int);
            i = i.wrapping_add(1)
        }
        if count as libc::c_int == 0xffffi32 {
            ttstub_input_seek(
                (*cff).handle,
                ttstub_input_seek((*cff).handle, 0i32 as ssize_t, 1i32)
                    .wrapping_add((*idx).offsize as libc::c_ulong) as ssize_t,
                0i32,
            );
        } else {
            *(*idx).offset.offset(i as isize) =
                get_unsigned((*cff).handle, (*idx).offsize as libc::c_int)
        }
        if *(*idx).offset.offset(0) != 1i32 as libc::c_uint {
            _tt_abort(
                b"cff_get_index(): invalid index data\x00" as *const u8 as *const libc::c_char,
            );
        }
        (*idx).data = 0 as *mut card8
    } else {
        (*idx).offsize = 0i32 as c_offsize;
        (*idx).offset = 0 as *mut l_offset;
        (*idx).data = 0 as *mut card8
    }
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_index(mut cff: *mut cff_font) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut i: card16 = 0;
    let mut count: card16 = 0;
    let mut length: libc::c_int = 0;
    let mut nb_read: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    idx = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_index>() as libc::c_ulong)
        as uint32_t) as *mut cff_index;
    count = tt_get_unsigned_pair((*cff).handle);
    (*idx).count = count;
    if count as libc::c_int > 0i32 {
        (*idx).offsize = tt_get_unsigned_byte((*cff).handle);
        if ((*idx).offsize as libc::c_int) < 1i32 || (*idx).offsize as libc::c_int > 4i32 {
            _tt_abort(b"invalid offsize data\x00" as *const u8 as *const libc::c_char);
        }
        (*idx).offset = new(((count as libc::c_int + 1i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as libc::c_ulong)
            as uint32_t) as *mut l_offset;
        i = 0i32 as card16;
        while (i as libc::c_int) < count as libc::c_int + 1i32 {
            *(*idx).offset.offset(i as isize) =
                get_unsigned((*cff).handle, (*idx).offsize as libc::c_int);
            i = i.wrapping_add(1)
        }
        if *(*idx).offset.offset(0) != 1i32 as libc::c_uint {
            _tt_abort(b"Invalid CFF Index offset data\x00" as *const u8 as *const libc::c_char);
        }
        length = (*(*idx).offset.offset(count as isize)).wrapping_sub(*(*idx).offset.offset(0))
            as libc::c_int;
        (*idx).data = new((length as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
            as uint32_t) as *mut card8;
        offset = 0i32;
        while length > 0i32 {
            nb_read = ttstub_input_read(
                (*cff).handle,
                ((*idx).data as *mut libc::c_char).offset(offset as isize),
                length as size_t,
            ) as libc::c_int;
            offset += nb_read;
            length -= nb_read
        }
    } else {
        (*idx).offsize = 0i32 as c_offsize;
        (*idx).offset = 0 as *mut l_offset;
        (*idx).data = 0 as *mut card8
    }
    return idx;
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_index(
    mut idx: *mut cff_index,
    mut dest: *mut card8,
    mut destlen: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut datalen: size_t = 0;
    let mut i: card16 = 0;
    if ((*idx).count as libc::c_int) < 1i32 {
        if destlen < 2i32 {
            _tt_abort(b"Not enough space available...\x00" as *const u8 as *const libc::c_char);
        }
        memset(dest as *mut libc::c_void, 0i32, 2i32 as libc::c_ulong);
        return 2i32;
    }
    len = cff_index_size(idx);
    datalen =
        (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1i32 as libc::c_uint) as size_t;
    if destlen < len {
        _tt_abort(b"Not enough space available...\x00" as *const u8 as *const libc::c_char);
    }
    let fresh5 = dest;
    dest = dest.offset(1);
    *fresh5 = ((*idx).count as libc::c_int >> 8i32 & 0xffi32) as card8;
    let fresh6 = dest;
    dest = dest.offset(1);
    *fresh6 = ((*idx).count as libc::c_int & 0xffi32) as card8;
    if datalen < 0xff {
        (*idx).offsize = 1i32 as c_offsize;
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = 1i32 as card8;
        i = 0i32 as card16;
        while i as libc::c_int <= (*idx).count as libc::c_int {
            let fresh8 = dest;
            dest = dest.offset(1);
            *fresh8 = (*(*idx).offset.offset(i as isize) & 0xffi32 as libc::c_uint) as card8;
            i = i.wrapping_add(1)
        }
    } else if datalen < 0xffff {
        (*idx).offsize = 2i32 as c_offsize;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = 2i32 as card8;
        i = 0i32 as card16;
        while i as libc::c_int <= (*idx).count as libc::c_int {
            let fresh10 = dest;
            dest = dest.offset(1);
            *fresh10 =
                (*(*idx).offset.offset(i as isize) >> 8i32 & 0xffi32 as libc::c_uint) as card8;
            let fresh11 = dest;
            dest = dest.offset(1);
            *fresh11 = (*(*idx).offset.offset(i as isize) & 0xffi32 as libc::c_uint) as card8;
            i = i.wrapping_add(1)
        }
    } else if datalen < 0xffffff {
        (*idx).offsize = 3i32 as c_offsize;
        let fresh12 = dest;
        dest = dest.offset(1);
        *fresh12 = 3i32 as card8;
        i = 0i32 as card16;
        while i as libc::c_int <= (*idx).count as libc::c_int {
            let fresh13 = dest;
            dest = dest.offset(1);
            *fresh13 =
                (*(*idx).offset.offset(i as isize) >> 16i32 & 0xffi32 as libc::c_uint) as card8;
            let fresh14 = dest;
            dest = dest.offset(1);
            *fresh14 =
                (*(*idx).offset.offset(i as isize) >> 8i32 & 0xffi32 as libc::c_uint) as card8;
            let fresh15 = dest;
            dest = dest.offset(1);
            *fresh15 = (*(*idx).offset.offset(i as isize) & 0xffi32 as libc::c_uint) as card8;
            i = i.wrapping_add(1)
        }
    } else {
        (*idx).offsize = 4i32 as c_offsize;
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = 4i32 as card8;
        i = 0i32 as card16;
        while i as libc::c_int <= (*idx).count as libc::c_int {
            let fresh17 = dest;
            dest = dest.offset(1);
            *fresh17 =
                (*(*idx).offset.offset(i as isize) >> 24i32 & 0xffi32 as libc::c_uint) as card8;
            let fresh18 = dest;
            dest = dest.offset(1);
            *fresh18 =
                (*(*idx).offset.offset(i as isize) >> 16i32 & 0xffi32 as libc::c_uint) as card8;
            let fresh19 = dest;
            dest = dest.offset(1);
            *fresh19 =
                (*(*idx).offset.offset(i as isize) >> 8i32 & 0xffi32 as libc::c_uint) as card8;
            let fresh20 = dest;
            dest = dest.offset(1);
            *fresh20 = (*(*idx).offset.offset(i as isize) & 0xffi32 as libc::c_uint) as card8;
            i = i.wrapping_add(1)
        }
    }
    memmove(
        dest as *mut libc::c_void,
        (*idx).data as *const libc::c_void,
        (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1i32 as libc::c_uint)
            as libc::c_ulong,
    );
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn cff_index_size(mut idx: *mut cff_index) -> libc::c_int {
    if (*idx).count as libc::c_int > 0i32 {
        let mut datalen: l_offset = 0;
        datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1i32 as libc::c_uint);
        if (datalen as libc::c_ulong) < 0xff {
            (*idx).offsize = 1i32 as c_offsize
        } else if (datalen as libc::c_ulong) < 0xffff {
            (*idx).offsize = 2i32 as c_offsize
        } else if (datalen as libc::c_ulong) < 0xffffff {
            (*idx).offsize = 3i32 as c_offsize
        } else {
            (*idx).offsize = 4i32 as c_offsize
        }
        return ((3i32 + (*idx).offsize as libc::c_int * ((*idx).count as libc::c_int + 1i32))
            as libc::c_uint)
            .wrapping_add(datalen) as libc::c_int;
    } else {
        return 2i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_new_index(mut count: card16) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    idx = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_index>() as libc::c_ulong)
        as uint32_t) as *mut cff_index;
    (*idx).count = count;
    (*idx).offsize = 0i32 as c_offsize;
    if count as libc::c_int > 0i32 {
        (*idx).offset = new(((count as libc::c_int + 1i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as libc::c_ulong)
            as uint32_t) as *mut l_offset;
        *(*idx).offset.offset(0) = 1i32 as l_offset
    } else {
        (*idx).offset = 0 as *mut l_offset
    }
    (*idx).data = 0 as *mut card8;
    return idx;
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
pub unsafe extern "C" fn cff_get_string(
    mut cff: *mut cff_font,
    mut id: s_SID,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if (id as libc::c_int) < 391i32 {
        len = strlen(cff_stdstr[id as usize]) as libc::c_int;
        result = new(((len + 1i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_char;
        memcpy(
            result as *mut libc::c_void,
            cff_stdstr[id as usize] as *const libc::c_void,
            len as libc::c_ulong,
        );
        *result.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    } else if !cff.is_null() && !(*cff).string.is_null() {
        let mut strings: *mut cff_index = (*cff).string;
        id = (id as libc::c_int - 391i32) as s_SID;
        if (id as libc::c_int) < (*strings).count as libc::c_int {
            len = (*(*strings)
                .offset
                .offset((id as libc::c_int + 1i32) as isize))
            .wrapping_sub(*(*strings).offset.offset(id as isize)) as libc::c_int;
            result = new(((len + 1i32) as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as uint32_t) as *mut libc::c_char;
            memmove(
                result as *mut libc::c_void,
                (*strings)
                    .data
                    .offset(*(*strings).offset.offset(id as isize) as isize)
                    .offset(-1) as *const libc::c_void,
                len as libc::c_ulong,
            );
            *result.offset(len as isize) = '\u{0}' as i32 as libc::c_char
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_sid(
    mut cff: *mut cff_font,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut i: card16 = 0;
    if cff.is_null() || str.is_null() {
        return -1i32;
    }
    /* I search String INDEX first. */
    if !cff.is_null() && !(*cff).string.is_null() {
        let mut idx: *mut cff_index = (*cff).string;
        i = 0i32 as card16;
        while (i as libc::c_int) < (*idx).count as libc::c_int {
            if strlen(str)
                == (*(*idx).offset.offset((i as libc::c_int + 1i32) as isize))
                    .wrapping_sub(*(*idx).offset.offset(i as isize))
                    as libc::c_ulong
                && memcmp(
                    str as *const libc::c_void,
                    (*idx)
                        .data
                        .offset(*(*idx).offset.offset(i as isize) as isize)
                        .offset(-1) as *const libc::c_void,
                    strlen(str),
                ) == 0
            {
                return i as libc::c_int + 391i32;
            }
            i = i.wrapping_add(1)
        }
    }
    i = 0i32 as card16;
    while (i as libc::c_int) < 391i32 {
        if streq_ptr(str, cff_stdstr[i as usize]) {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_seac_sid(
    mut cff: *mut cff_font,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut i: card16 = 0;
    if cff.is_null() || str.is_null() {
        return -1i32;
    }
    i = 0i32 as card16;
    while (i as libc::c_int) < 391i32 {
        if streq_ptr(str, cff_stdstr[i as usize]) {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}
unsafe extern "C" fn cff_match_string(
    mut cff: *mut cff_font,
    mut str: *const libc::c_char,
    mut sid: s_SID,
) -> libc::c_int {
    let mut i: card16 = 0;
    if (sid as libc::c_int) < 391i32 {
        return if streq_ptr(str, cff_stdstr[sid as usize]) as libc::c_int != 0 {
            1i32
        } else {
            0i32
        };
    } else {
        i = (sid as libc::c_int - 391i32) as card16;
        if cff.is_null()
            || (*cff).string.is_null()
            || i as libc::c_int >= (*(*cff).string).count as libc::c_int
        {
            _tt_abort(b"Invalid SID\x00" as *const u8 as *const libc::c_char);
        }
        if strlen(str)
            == (*(*(*cff).string)
                .offset
                .offset((i as libc::c_int + 1i32) as isize))
            .wrapping_sub(*(*(*cff).string).offset.offset(i as isize))
                as libc::c_ulong
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
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn cff_update_string(mut cff: *mut cff_font) {
    if cff.is_null() {
        _tt_abort(b"CFF font not opened.\x00" as *const u8 as *const libc::c_char);
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
    mut str: *const libc::c_char,
    mut unique: libc::c_int,
) -> s_SID
/* Setting unique == 1 eliminates redundant or predefined strings. */ {
    let mut idx: card16 = 0;
    let mut strings: *mut cff_index = 0 as *mut cff_index;
    let mut offset: l_offset = 0;
    let mut size: l_offset = 0;
    let mut len: size_t = strlen(str);
    if cff.is_null() {
        _tt_abort(b"CFF font not opened.\x00" as *const u8 as *const libc::c_char);
    }
    if (*cff)._string.is_null() {
        (*cff)._string = cff_new_index(0i32 as card16)
    }
    strings = (*cff)._string;
    if unique != 0 {
        /* TODO: do binary search to speed things up */
        idx = 0i32 as card16;
        while (idx as libc::c_int) < 391i32 {
            if streq_ptr(cff_stdstr[idx as usize], str) {
                return idx;
            }
            idx = idx.wrapping_add(1)
        }
        idx = 0i32 as card16;
        while (idx as libc::c_int) < (*strings).count as libc::c_int {
            size = (*(*strings)
                .offset
                .offset((idx as libc::c_int + 1i32) as isize))
            .wrapping_sub(*(*strings).offset.offset(idx as isize));
            offset = *(*strings).offset.offset(idx as isize);
            if size as libc::c_ulong == len
                && memcmp(
                    (*strings).data.offset(offset as isize).offset(-1) as *const libc::c_void,
                    str as *const libc::c_void,
                    len,
                ) == 0
            {
                return (idx as libc::c_int + 391i32) as s_SID;
            }
            idx = idx.wrapping_add(1)
        }
    }
    offset = if (*strings).count as libc::c_int > 0i32 {
        *(*strings).offset.offset((*strings).count as isize)
    } else {
        1i32 as libc::c_uint
    };
    (*strings).offset = renew(
        (*strings).offset as *mut libc::c_void,
        (((*strings).count as libc::c_int + 2i32) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as libc::c_ulong) as uint32_t,
    ) as *mut l_offset;
    if (*strings).count as libc::c_int == 0i32 {
        *(*strings).offset.offset(0) = 1i32 as l_offset
    }
    idx = (*strings).count;
    (*strings).count = ((*strings).count as libc::c_int + 1i32) as card16;
    *(*strings).offset.offset((*strings).count as isize) =
        (offset as libc::c_ulong).wrapping_add(len) as l_offset;
    (*strings).data = renew(
        (*strings).data as *mut libc::c_void,
        ((offset as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_sub(1i32 as libc::c_ulong) as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong) as uint32_t,
    ) as *mut card8;
    memcpy(
        (*strings).data.offset(offset as isize).offset(-1) as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    return (idx as libc::c_int + 391i32) as s_SID;
}
/*
 * Encoding and Charset
 *
 *  Encoding and Charset arrays always begin with GID = 1.
 */
#[no_mangle]
pub unsafe extern "C" fn cff_read_encoding(mut cff: *mut cff_font) -> libc::c_int {
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut offset: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut i: card8 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"Top DICT data not found\x00" as *const u8 as *const libc::c_char);
    }
    if cff_dict_known(
        (*cff).topdict,
        b"Encoding\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*cff).flag |= 1i32 << 3i32;
        (*cff).encoding = 0 as *mut cff_encoding;
        return 0i32;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"Encoding\x00" as *const u8 as *const libc::c_char,
        0i32,
    ) as libc::c_int;
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
        (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
        0i32,
    );
    encoding = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_encoding>() as libc::c_ulong)
        as uint32_t) as *mut cff_encoding;
    (*cff).encoding = encoding;
    (*encoding).format = tt_get_unsigned_byte((*cff).handle);
    length = 1i32;
    match (*encoding).format as libc::c_int & !0x80i32 {
        0 => {
            (*encoding).num_entries = tt_get_unsigned_byte((*cff).handle);
            (*encoding).data.codes = new(((*encoding).num_entries as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                as uint32_t) as *mut card8;
            i = 0i32 as card8;
            while (i as libc::c_int) < (*encoding).num_entries as libc::c_int {
                *(*encoding).data.codes.offset(i as isize) = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            length += (*encoding).num_entries as libc::c_int + 1i32
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            (*encoding).num_entries = tt_get_unsigned_byte((*cff).handle);
            ranges = new(((*encoding).num_entries as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cff_range1>() as libc::c_ulong)
                as uint32_t) as *mut cff_range1;
            (*encoding).data.range1 = ranges;
            i = 0i32 as card8;
            while (i as libc::c_int) < (*encoding).num_entries as libc::c_int {
                (*ranges.offset(i as isize)).first = tt_get_unsigned_byte((*cff).handle) as s_SID;
                (*ranges.offset(i as isize)).n_left = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            length += (*encoding).num_entries as libc::c_int * 2i32 + 1i32
        }
        _ => {
            free(encoding as *mut libc::c_void);
            _tt_abort(b"Unknown Encoding format\x00" as *const u8 as *const libc::c_char);
        }
    }
    /* Supplementary data */
    if (*encoding).format as libc::c_int & 0x80i32 != 0 {
        let mut map: *mut cff_map = 0 as *mut cff_map;
        (*encoding).num_supps = tt_get_unsigned_byte((*cff).handle);
        map = new(((*encoding).num_supps as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cff_map>() as libc::c_ulong)
            as uint32_t) as *mut cff_map;
        (*encoding).supp = map;
        i = 0i32 as card8;
        while (i as libc::c_int) < (*encoding).num_supps as libc::c_int {
            (*map.offset(i as isize)).code = tt_get_unsigned_byte((*cff).handle);
            (*map.offset(i as isize)).glyph = tt_get_unsigned_pair((*cff).handle);
            i = i.wrapping_add(1)
            /* SID */
        }
        length += (*encoding).num_supps as libc::c_int * 3i32 + 1i32
    } else {
        (*encoding).num_supps = 0i32 as card8;
        (*encoding).supp = 0 as *mut cff_map
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_encoding(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut i: card16 = 0;
    if (*cff).flag & (1i32 << 3i32 | 1i32 << 4i32) != 0 || (*cff).encoding.is_null() {
        return 0i32;
    }
    if destlen < 2i32 {
        _tt_abort(
            b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8 as *const libc::c_char,
        );
    }
    encoding = (*cff).encoding;
    let fresh21 = len;
    len = len + 1;
    *dest.offset(fresh21 as isize) = (*encoding).format;
    let fresh22 = len;
    len = len + 1;
    *dest.offset(fresh22 as isize) = (*encoding).num_entries;
    match (*encoding).format as libc::c_int & !0x80i32 {
        0 => {
            if destlen < len + (*encoding).num_entries as libc::c_int {
                _tt_abort(
                    b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0i32 as card16;
            while (i as libc::c_int) < (*encoding).num_entries as libc::c_int {
                let fresh23 = len;
                len = len + 1;
                *dest.offset(fresh23 as isize) = *(*encoding).data.codes.offset(i as isize);
                i = i.wrapping_add(1)
            }
        }
        1 => {
            if destlen < len + (*encoding).num_entries as libc::c_int * 2i32 {
                _tt_abort(
                    b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0i32 as card16;
            while (i as libc::c_int) < (*encoding).num_entries as libc::c_int {
                let fresh24 = len;
                len = len + 1;
                *dest.offset(fresh24 as isize) = ((*(*encoding).data.range1.offset(i as isize))
                    .first as libc::c_int
                    & 0xffi32) as card8;
                let fresh25 = len;
                len = len + 1;
                *dest.offset(fresh25 as isize) =
                    (*(*encoding).data.range1.offset(i as isize)).n_left;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Encoding format\x00" as *const u8 as *const libc::c_char);
        }
    }
    if (*encoding).format as libc::c_int & 0x80i32 != 0 {
        if destlen < len + (*encoding).num_supps as libc::c_int * 3i32 + 1i32 {
            _tt_abort(
                b"in cff_pack_encoding(): Buffer overflow\x00" as *const u8 as *const libc::c_char,
            );
        }
        let fresh26 = len;
        len = len + 1;
        *dest.offset(fresh26 as isize) = (*encoding).num_supps;
        i = 0i32 as card16;
        while (i as libc::c_int) < (*encoding).num_supps as libc::c_int {
            let fresh27 = len;
            len = len + 1;
            *dest.offset(fresh27 as isize) = (*(*encoding).supp.offset(i as isize)).code;
            let fresh28 = len;
            len = len + 1;
            *dest.offset(fresh28 as isize) =
                ((*(*encoding).supp.offset(i as isize)).glyph as libc::c_int >> 8i32 & 0xffi32)
                    as card8;
            let fresh29 = len;
            len = len + 1;
            *dest.offset(fresh29 as isize) =
                ((*(*encoding).supp.offset(i as isize)).glyph as libc::c_int & 0xffi32) as card8;
            i = i.wrapping_add(1)
        }
    }
    return len;
}
/* input: code, output: glyph index */
#[no_mangle]
pub unsafe extern "C" fn cff_encoding_lookup(mut cff: *mut cff_font, mut code: card8) -> card16 {
    let mut gid: card16 = 0i32 as card16;
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut i: card16 = 0;
    if (*cff).flag & (1i32 << 3i32 | 1i32 << 4i32) != 0 {
        _tt_abort(
            b"Predefined CFF encoding not supported yet\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*cff).encoding.is_null() {
            _tt_abort(b"Encoding data not available\x00" as *const u8 as *const libc::c_char);
        }
    }
    encoding = (*cff).encoding;
    gid = 0i32 as card16;
    match (*encoding).format as libc::c_int & !0x80i32 {
        0 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*encoding).num_entries as libc::c_int {
                if code as libc::c_int == *(*encoding).data.codes.offset(i as isize) as libc::c_int
                {
                    gid = (i as libc::c_int + 1i32) as card16;
                    break;
                } else {
                    i = i.wrapping_add(1)
                }
            }
        }
        1 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*encoding).num_entries as libc::c_int {
                if code as libc::c_int
                    >= (*(*encoding).data.range1.offset(i as isize)).first as libc::c_int
                    && code as libc::c_int
                        <= (*(*encoding).data.range1.offset(i as isize)).first as libc::c_int
                            + (*(*encoding).data.range1.offset(i as isize)).n_left as libc::c_int
                {
                    gid = (gid as libc::c_int
                        + (code as libc::c_int
                            - (*(*encoding).data.range1.offset(i as isize)).first as libc::c_int
                            + 1i32)) as card16;
                    break;
                } else {
                    gid = (gid as libc::c_int
                        + ((*(*encoding).data.range1.offset(i as isize)).n_left as libc::c_int
                            + 1i32)) as card16;
                    i = i.wrapping_add(1)
                }
            }
            if i as libc::c_int == (*encoding).num_entries as libc::c_int {
                gid = 0i32 as card16
            }
        }
        _ => {
            _tt_abort(b"Unknown Encoding format.\x00" as *const u8 as *const libc::c_char);
        }
    }
    /* Supplementary data */
    if gid as libc::c_int == 0i32 && (*encoding).format as libc::c_int & 0x80i32 != 0 {
        let mut map: *mut cff_map = 0 as *mut cff_map;
        if (*encoding).supp.is_null() {
            _tt_abort(
                b"No CFF supplementary encoding data read.\x00" as *const u8 as *const libc::c_char,
            );
        }
        map = (*encoding).supp;
        i = 0i32 as card16;
        while (i as libc::c_int) < (*encoding).num_supps as libc::c_int {
            if code as libc::c_int == (*map.offset(i as isize)).code as libc::c_int {
                gid = cff_charsets_lookup(cff, (*map.offset(i as isize)).glyph);
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
    }
    return gid;
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_encoding(mut encoding: *mut cff_encoding) {
    if !encoding.is_null() {
        match (*encoding).format as libc::c_int & !0x80i32 {
            0 => {
                free((*encoding).data.codes as *mut libc::c_void);
            }
            1 => {
                free((*encoding).data.range1 as *mut libc::c_void);
            }
            _ => {
                _tt_abort(b"Unknown Encoding format.\x00" as *const u8 as *const libc::c_char);
            }
        }
        if (*encoding).format as libc::c_int & 0x80i32 != 0 {
            free((*encoding).supp as *mut libc::c_void);
        }
        free(encoding as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_charsets(mut cff: *mut cff_font) -> libc::c_int {
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut offset: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut count: card16 = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"Top DICT not available\x00" as *const u8 as *const libc::c_char);
    }
    if cff_dict_known(
        (*cff).topdict,
        b"charset\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*cff).flag |= 1i32 << 5i32;
        (*cff).charsets = 0 as *mut cff_charsets;
        return 0i32;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"charset\x00" as *const u8 as *const libc::c_char,
        0i32,
    ) as libc::c_int;
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
        (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
        0i32,
    );
    charset = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_charsets>() as libc::c_ulong)
        as uint32_t) as *mut cff_charsets;
    (*cff).charsets = charset;
    (*charset).format = tt_get_unsigned_byte((*cff).handle);
    (*charset).num_entries = 0i32 as card16;
    count = ((*cff).num_glyphs as libc::c_int - 1i32) as card16;
    length = 1i32;
    /* Not sure. Not well documented. */
    match (*charset).format as libc::c_int {
        0 => {
            (*charset).num_entries = ((*cff).num_glyphs as libc::c_int - 1i32) as card16; /* no .notdef */
            (*charset).data.glyphs = new(((*charset).num_entries as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<s_SID>() as libc::c_ulong)
                as uint32_t) as *mut s_SID; /* no-overrap */
            length += (*charset).num_entries as libc::c_int * 2i32; /* non-overrapping */
            i = 0i32 as card16; /* or CID */
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                *(*charset).data.glyphs.offset(i as isize) = tt_get_unsigned_pair((*cff).handle);
                i = i.wrapping_add(1)
            }
            count = 0i32 as card16
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            while count as libc::c_int > 0i32
                && ((*charset).num_entries as libc::c_int) < (*cff).num_glyphs as libc::c_int
            {
                ranges = renew(
                    ranges as *mut libc::c_void,
                    (((*charset).num_entries as libc::c_int + 1i32) as uint32_t as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<cff_range1>() as libc::c_ulong)
                        as uint32_t,
                ) as *mut cff_range1;
                (*ranges.offset((*charset).num_entries as isize)).first =
                    tt_get_unsigned_pair((*cff).handle);
                (*ranges.offset((*charset).num_entries as isize)).n_left =
                    tt_get_unsigned_byte((*cff).handle);
                count = (count as libc::c_int
                    - ((*ranges.offset((*charset).num_entries as isize)).n_left as libc::c_int
                        + 1i32)) as card16;
                (*charset).num_entries = ((*charset).num_entries as libc::c_int + 1i32) as card16;
                (*charset).data.range1 = ranges
            }
            length += (*charset).num_entries as libc::c_int * 3i32
        }
        2 => {
            let mut ranges_0: *mut cff_range2 = 0 as *mut cff_range2;
            while count as libc::c_int > 0i32
                && ((*charset).num_entries as libc::c_int) < (*cff).num_glyphs as libc::c_int
            {
                ranges_0 = renew(
                    ranges_0 as *mut libc::c_void,
                    (((*charset).num_entries as libc::c_int + 1i32) as uint32_t as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<cff_range2>() as libc::c_ulong)
                        as uint32_t,
                ) as *mut cff_range2;
                (*ranges_0.offset((*charset).num_entries as isize)).first =
                    tt_get_unsigned_pair((*cff).handle);
                (*ranges_0.offset((*charset).num_entries as isize)).n_left =
                    tt_get_unsigned_pair((*cff).handle);
                count = (count as libc::c_int
                    - ((*ranges_0.offset((*charset).num_entries as isize)).n_left as libc::c_int
                        + 1i32)) as card16;
                (*charset).num_entries = ((*charset).num_entries as libc::c_int + 1i32) as card16
            }
            (*charset).data.range2 = ranges_0;
            length += (*charset).num_entries as libc::c_int * 4i32
        }
        _ => {
            free(charset as *mut libc::c_void);
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const libc::c_char);
        }
    }
    if count as libc::c_int > 0i32 {
        _tt_abort(b"Charset data possibly broken\x00" as *const u8 as *const libc::c_char);
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_charsets(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut i: card16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    if (*cff).flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 || (*cff).charsets.is_null()
    {
        return 0i32;
    }
    if destlen < 1i32 {
        _tt_abort(
            b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8 as *const libc::c_char,
        );
    }
    charset = (*cff).charsets;
    let fresh30 = len;
    len = len + 1;
    *dest.offset(fresh30 as isize) = (*charset).format;
    match (*charset).format as libc::c_int {
        0 => {
            if destlen < len + (*charset).num_entries as libc::c_int * 2i32 {
                _tt_abort(
                    b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                let mut sid: s_SID = *(*charset).data.glyphs.offset(i as isize);
                let fresh31 = len;
                len = len + 1;
                *dest.offset(fresh31 as isize) = (sid as libc::c_int >> 8i32 & 0xffi32) as card8;
                let fresh32 = len;
                len = len + 1;
                *dest.offset(fresh32 as isize) = (sid as libc::c_int & 0xffi32) as card8;
                i = i.wrapping_add(1)
            }
        }
        1 => {
            if destlen < len + (*charset).num_entries as libc::c_int * 3i32 {
                _tt_abort(
                    b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                let fresh33 = len;
                len = len + 1;
                *dest.offset(fresh33 as isize) =
                    ((*(*charset).data.range1.offset(i as isize)).first as libc::c_int >> 8i32
                        & 0xffi32) as card8;
                let fresh34 = len;
                len = len + 1;
                *dest.offset(fresh34 as isize) = ((*(*charset).data.range1.offset(i as isize)).first
                    as libc::c_int
                    & 0xffi32) as card8;
                let fresh35 = len;
                len = len + 1;
                *dest.offset(fresh35 as isize) =
                    (*(*charset).data.range1.offset(i as isize)).n_left;
                i = i.wrapping_add(1)
            }
        }
        2 => {
            if destlen < len + (*charset).num_entries as libc::c_int * 4i32 {
                _tt_abort(
                    b"in cff_pack_charsets(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                let fresh36 = len;
                len = len + 1;
                *dest.offset(fresh36 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).first as libc::c_int >> 8i32
                        & 0xffi32) as card8;
                let fresh37 = len;
                len = len + 1;
                *dest.offset(fresh37 as isize) = ((*(*charset).data.range2.offset(i as isize)).first
                    as libc::c_int
                    & 0xffi32) as card8;
                let fresh38 = len;
                len = len + 1;
                *dest.offset(fresh38 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).n_left as libc::c_int >> 8i32
                        & 0xffi32) as card8;
                let fresh39 = len;
                len = len + 1;
                *dest.offset(fresh39 as isize) = ((*(*charset).data.range2.offset(i as isize))
                    .n_left as libc::c_int
                    & 0xffi32) as card8;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const libc::c_char);
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_glyphname(
    mut cff: *mut cff_font,
    mut gid: card16,
) -> *mut libc::c_char {
    let mut sid: s_SID = 0;
    sid = cff_charsets_lookup_inverse(cff, gid);
    return cff_get_string(cff, sid);
}
#[no_mangle]
pub unsafe extern "C" fn cff_glyph_lookup(
    mut cff: *mut cff_font,
    mut glyph: *const libc::c_char,
) -> card16 {
    let mut gid: card16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut i: card16 = 0;
    let mut n: card16 = 0;
    if (*cff).flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        _tt_abort(
            b"Predefined CFF charsets not supported yet\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*cff).charsets.is_null() {
            _tt_abort(b"Charsets data not available\x00" as *const u8 as *const libc::c_char);
        }
    }
    /* .notdef always have glyph index 0 */
    if glyph.is_null()
        || streq_ptr(glyph, b".notdef\x00" as *const u8 as *const libc::c_char) as libc::c_int != 0
    {
        return 0i32 as card16;
    }
    charset = (*cff).charsets;
    gid = 0i32 as card16;
    match (*charset).format as libc::c_int {
        0 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                gid = gid.wrapping_add(1);
                if cff_match_string(cff, glyph, *(*charset).data.glyphs.offset(i as isize)) != 0 {
                    return gid;
                }
                i = i.wrapping_add(1)
            }
        }
        1 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                n = 0i32 as card16;
                while n as libc::c_int
                    <= (*(*charset).data.range1.offset(i as isize)).n_left as libc::c_int
                {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range1.offset(i as isize)).first as libc::c_int
                            + n as libc::c_int) as s_SID,
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
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                n = 0i32 as card16;
                while n as libc::c_int
                    <= (*(*charset).data.range2.offset(i as isize)).n_left as libc::c_int
                {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range2.offset(i as isize)).first as libc::c_int
                            + n as libc::c_int) as s_SID,
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
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const libc::c_char);
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
        _tt_abort(
            b"Predefined CFF charsets not supported yet\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*cff).charsets.is_null() {
            _tt_abort(b"Charsets data not available\x00" as *const u8 as *const libc::c_char);
        }
    }
    return cff_charsets_lookup_gid((*cff).charsets, cid);
}
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_gid(
    mut charset: *mut cff_charsets,
    mut cid: card16,
) -> card16 {
    let mut gid: card16 = 0i32 as card16;
    let mut i: card16 = 0;
    if cid as libc::c_int == 0i32 {
        return 0i32 as card16;
        /* GID 0 (.notdef) */
    }
    match (*charset).format as libc::c_int {
        0 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                if cid as libc::c_int == *(*charset).data.glyphs.offset(i as isize) as libc::c_int {
                    gid = (i as libc::c_int + 1i32) as card16;
                    return gid;
                }
                i = i.wrapping_add(1)
            }
        }
        1 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                if cid as libc::c_int
                    >= (*(*charset).data.range1.offset(i as isize)).first as libc::c_int
                    && cid as libc::c_int
                        <= (*(*charset).data.range1.offset(i as isize)).first as libc::c_int
                            + (*(*charset).data.range1.offset(i as isize)).n_left as libc::c_int
                {
                    gid = (gid as libc::c_int
                        + (cid as libc::c_int
                            - (*(*charset).data.range1.offset(i as isize)).first as libc::c_int
                            + 1i32)) as card16;
                    return gid;
                }
                gid = (gid as libc::c_int
                    + ((*(*charset).data.range1.offset(i as isize)).n_left as libc::c_int + 1i32))
                    as card16;
                i = i.wrapping_add(1)
            }
        }
        2 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                if cid as libc::c_int
                    >= (*(*charset).data.range2.offset(i as isize)).first as libc::c_int
                    && cid as libc::c_int
                        <= (*(*charset).data.range2.offset(i as isize)).first as libc::c_int
                            + (*(*charset).data.range2.offset(i as isize)).n_left as libc::c_int
                {
                    gid = (gid as libc::c_int
                        + (cid as libc::c_int
                            - (*(*charset).data.range2.offset(i as isize)).first as libc::c_int
                            + 1i32)) as card16;
                    return gid;
                }
                gid = (gid as libc::c_int
                    + ((*(*charset).data.range2.offset(i as isize)).n_left as libc::c_int + 1i32))
                    as card16;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const libc::c_char);
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
        _tt_abort(
            b"Predefined CFF charsets not supported yet\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*cff).charsets.is_null() {
            _tt_abort(b"Charsets data not available\x00" as *const u8 as *const libc::c_char);
        }
    }
    if gid as libc::c_int == 0i32 {
        return 0i32 as card16;
        /* .notdef */
    }
    return cff_charsets_lookup_cid((*cff).charsets, gid);
}
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_cid(
    mut charset: *mut cff_charsets,
    mut gid: card16,
) -> card16 {
    let mut sid: card16 = 0i32 as card16;
    let mut i: card16 = 0;
    match (*charset).format as libc::c_int {
        0 => {
            if gid as libc::c_int - 1i32 >= (*charset).num_entries as libc::c_int {
                _tt_abort(b"Invalid GID.\x00" as *const u8 as *const libc::c_char);
            }
            sid = *(*charset)
                .data
                .glyphs
                .offset((gid as libc::c_int - 1i32) as isize)
        }
        1 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                if gid as libc::c_int
                    <= (*(*charset).data.range1.offset(i as isize)).n_left as libc::c_int + 1i32
                {
                    sid = (gid as libc::c_int
                        + (*(*charset).data.range1.offset(i as isize)).first as libc::c_int
                        - 1i32) as card16;
                    break;
                } else {
                    gid = (gid as libc::c_int
                        - ((*(*charset).data.range1.offset(i as isize)).n_left as libc::c_int
                            + 1i32)) as card16;
                    i = i.wrapping_add(1)
                }
            }
            if i as libc::c_int == (*charset).num_entries as libc::c_int {
                _tt_abort(b"Invalid GID\x00" as *const u8 as *const libc::c_char);
            }
        }
        2 => {
            i = 0i32 as card16;
            while (i as libc::c_int) < (*charset).num_entries as libc::c_int {
                if gid as libc::c_int
                    <= (*(*charset).data.range2.offset(i as isize)).n_left as libc::c_int + 1i32
                {
                    sid = (gid as libc::c_int
                        + (*(*charset).data.range2.offset(i as isize)).first as libc::c_int
                        - 1i32) as card16;
                    break;
                } else {
                    gid = (gid as libc::c_int
                        - ((*(*charset).data.range2.offset(i as isize)).n_left as libc::c_int
                            + 1i32)) as card16;
                    i = i.wrapping_add(1)
                }
            }
            if i as libc::c_int == (*charset).num_entries as libc::c_int {
                _tt_abort(b"Invalid GID\x00" as *const u8 as *const libc::c_char);
            }
        }
        _ => {
            _tt_abort(b"Unknown Charset format\x00" as *const u8 as *const libc::c_char);
        }
    }
    return sid;
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_charsets(mut charset: *mut cff_charsets) {
    if !charset.is_null() {
        match (*charset).format as libc::c_int {
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
pub unsafe extern "C" fn cff_read_fdselect(mut cff: *mut cff_font) -> libc::c_int {
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut offset: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(b"Top DICT not available\x00" as *const u8 as *const libc::c_char);
    }
    if (*cff).flag & 1i32 << 0i32 == 0 {
        return 0i32;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"FDSelect\x00" as *const u8 as *const libc::c_char,
        0i32,
    ) as libc::c_int;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
        0i32,
    );
    fdsel = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_fdselect>() as libc::c_ulong)
        as uint32_t) as *mut cff_fdselect;
    (*cff).fdselect = fdsel;
    (*fdsel).format = tt_get_unsigned_byte((*cff).handle);
    length = 1i32;
    match (*fdsel).format as libc::c_int {
        0 => {
            (*fdsel).num_entries = (*cff).num_glyphs;
            (*fdsel).data.fds = new(((*fdsel).num_entries as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                as uint32_t) as *mut card8;
            i = 0i32 as card16;
            while (i as libc::c_int) < (*fdsel).num_entries as libc::c_int {
                *(*fdsel).data.fds.offset(i as isize) = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            length += (*fdsel).num_entries as libc::c_int
        }
        3 => {
            let mut ranges: *mut cff_range3 = 0 as *mut cff_range3;
            (*fdsel).num_entries = tt_get_unsigned_pair((*cff).handle);
            ranges = new(((*fdsel).num_entries as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cff_range3>() as libc::c_ulong)
                as uint32_t) as *mut cff_range3;
            (*fdsel).data.ranges = ranges;
            i = 0i32 as card16;
            while (i as libc::c_int) < (*fdsel).num_entries as libc::c_int {
                (*ranges.offset(i as isize)).first = tt_get_unsigned_pair((*cff).handle);
                (*ranges.offset(i as isize)).fd = tt_get_unsigned_byte((*cff).handle);
                i = i.wrapping_add(1)
            }
            if (*ranges.offset(0)).first as libc::c_int != 0i32 {
                _tt_abort(b"Range not starting with 0.\x00" as *const u8 as *const libc::c_char);
            }
            if (*cff).num_glyphs as libc::c_int
                != tt_get_unsigned_pair((*cff).handle) as libc::c_int
            {
                _tt_abort(
                    b"Sentinel value mismatched with number of glyphs.\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            length += (*fdsel).num_entries as libc::c_int * 3i32 + 4i32
        }
        _ => {
            free(fdsel as *mut libc::c_void);
            _tt_abort(b"Unknown FDSelect format.\x00" as *const u8 as *const libc::c_char);
        }
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_fdselect(
    mut cff: *mut cff_font,
    mut dest: *mut card8,
    mut destlen: libc::c_int,
) -> libc::c_int {
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut len: libc::c_int = 0i32;
    let mut i: card16 = 0;
    if (*cff).fdselect.is_null() {
        return 0i32;
    }
    if destlen < 1i32 {
        _tt_abort(
            b"in cff_pack_fdselect(): Buffur overflow\x00" as *const u8 as *const libc::c_char,
        );
    }
    fdsel = (*cff).fdselect;
    let fresh40 = len;
    len = len + 1;
    *dest.offset(fresh40 as isize) = (*fdsel).format;
    match (*fdsel).format as libc::c_int {
        0 => {
            if (*fdsel).num_entries as libc::c_int != (*cff).num_glyphs as libc::c_int {
                _tt_abort(
                    b"in cff_pack_fdselect(): Invalid data\x00" as *const u8 as *const libc::c_char,
                );
            }
            if destlen < len + (*fdsel).num_entries as libc::c_int {
                _tt_abort(
                    b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            i = 0i32 as card16;
            while (i as libc::c_int) < (*fdsel).num_entries as libc::c_int {
                let fresh41 = len;
                len = len + 1;
                *dest.offset(fresh41 as isize) = *(*fdsel).data.fds.offset(i as isize);
                i = i.wrapping_add(1)
            }
        }
        3 => {
            if destlen < len + 2i32 {
                _tt_abort(
                    b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            len += 2i32;
            i = 0i32 as card16;
            while (i as libc::c_int) < (*fdsel).num_entries as libc::c_int {
                if destlen < len + 3i32 {
                    _tt_abort(
                        b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                let fresh42 = len;
                len = len + 1;
                *dest.offset(fresh42 as isize) =
                    ((*(*fdsel).data.ranges.offset(i as isize)).first as libc::c_int >> 8i32
                        & 0xffi32) as card8;
                let fresh43 = len;
                len = len + 1;
                *dest.offset(fresh43 as isize) = ((*(*fdsel).data.ranges.offset(i as isize)).first
                    as libc::c_int
                    & 0xffi32) as card8;
                let fresh44 = len;
                len = len + 1;
                *dest.offset(fresh44 as isize) = (*(*fdsel).data.ranges.offset(i as isize)).fd;
                i = i.wrapping_add(1)
            }
            if destlen < len + 2i32 {
                _tt_abort(
                    b"in cff_pack_fdselect(): Buffer overflow\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            let fresh45 = len;
            len = len + 1;
            *dest.offset(fresh45 as isize) =
                ((*cff).num_glyphs as libc::c_int >> 8i32 & 0xffi32) as card8;
            let fresh46 = len;
            len = len + 1;
            *dest.offset(fresh46 as isize) = ((*cff).num_glyphs as libc::c_int & 0xffi32) as card8;
            *dest.offset(1) = (len / 3i32 - 1i32 >> 8i32 & 0xffi32) as card8;
            *dest.offset(2) = (len / 3i32 - 1i32 & 0xffi32) as card8
        }
        _ => {
            _tt_abort(b"Unknown FDSelect format.\x00" as *const u8 as *const libc::c_char);
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_fdselect(mut fdselect: *mut cff_fdselect) {
    if !fdselect.is_null() {
        if (*fdselect).format as libc::c_int == 0i32 {
            free((*fdselect).data.fds as *mut libc::c_void);
        } else if (*fdselect).format as libc::c_int == 3i32 {
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
            b"in cff_fdselect_lookup(): FDSelect not available\x00" as *const u8
                as *const libc::c_char,
        );
    }
    fdsel = (*cff).fdselect;
    if gid as libc::c_int >= (*cff).num_glyphs as libc::c_int {
        _tt_abort(
            b"in cff_fdselect_lookup(): Invalid glyph index\x00" as *const u8
                as *const libc::c_char,
        );
    }
    match (*fdsel).format as libc::c_int {
        0 => fd = *(*fdsel).data.fds.offset(gid as isize),
        3 => {
            if gid as libc::c_int == 0i32 {
                fd = (*(*fdsel).data.ranges.offset(0)).fd
            } else {
                let mut i: card16 = 0;
                i = 1i32 as card16;
                while (i as libc::c_int) < (*fdsel).num_entries as libc::c_int {
                    if (gid as libc::c_int)
                        < (*(*fdsel).data.ranges.offset(i as isize)).first as libc::c_int
                    {
                        break;
                    }
                    i = i.wrapping_add(1)
                }
                fd = (*(*fdsel)
                    .data
                    .ranges
                    .offset((i as libc::c_int - 1i32) as isize))
                .fd
            }
        }
        _ => {
            _tt_abort(
                b"in cff_fdselect_lookup(): Invalid FDSelect format\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if fd as libc::c_int >= (*cff).num_fds as libc::c_int {
        _tt_abort(
            b"in cff_fdselect_lookup(): Invalid Font DICT index\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_subrs(mut cff: *mut cff_font) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
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
    (*cff).subrs = new(((*cff).num_fds as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut cff_index>() as libc::c_ulong)
        as uint32_t) as *mut *mut cff_index;
    if (*cff).flag & 1i32 << 0i32 != 0 {
        i = 0i32;
        while i < (*cff).num_fds as libc::c_int {
            if (*(*cff).private.offset(i as isize)).is_null()
                || cff_dict_known(
                    *(*cff).private.offset(i as isize),
                    b"Subrs\x00" as *const u8 as *const libc::c_char,
                ) == 0
            {
                let ref mut fresh47 = *(*cff).subrs.offset(i as isize);
                *fresh47 = 0 as *mut cff_index
            } else {
                offset = cff_dict_get(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) as libc::c_int;
                offset = (offset as libc::c_double
                    + cff_dict_get(
                        *(*cff).private.offset(i as isize),
                        b"Subrs\x00" as *const u8 as *const libc::c_char,
                        0i32,
                    )) as libc::c_int;
                ttstub_input_seek(
                    (*cff).handle,
                    (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
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
            b"Subrs\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let ref mut fresh49 = *(*cff).subrs.offset(0);
        *fresh49 = 0 as *mut cff_index
    } else {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Private\x00" as *const u8 as *const libc::c_char,
            1i32,
        ) as libc::c_int;
        offset = (offset as libc::c_double
            + cff_dict_get(
                *(*cff).private.offset(0),
                b"Subrs\x00" as *const u8 as *const libc::c_char,
                0i32,
            )) as libc::c_int;
        ttstub_input_seek(
            (*cff).handle,
            (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
            0i32,
        );
        let ref mut fresh50 = *(*cff).subrs.offset(0);
        *fresh50 = cff_get_index(cff);
        len += cff_index_size(*(*cff).subrs.offset(0))
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_fdarray(mut cff: *mut cff_font) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        _tt_abort(
            b"in cff_read_fdarray(): Top DICT not found\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*cff).flag & 1i32 << 0i32 == 0 {
        return 0i32;
    }
    /* must exist */
    offset = cff_dict_get(
        (*cff).topdict,
        b"FDArray\x00" as *const u8 as *const libc::c_char,
        0i32,
    ) as libc::c_int;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
        0i32,
    );
    idx = cff_get_index(cff);
    (*cff).num_fds = (*idx).count as card8;
    (*cff).fdarray = new(((*idx).count as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as libc::c_ulong)
        as uint32_t) as *mut *mut cff_dict;
    i = 0i32 as card16;
    while (i as libc::c_int) < (*idx).count as libc::c_int {
        let mut data: *mut card8 = (*idx)
            .data
            .offset(*(*idx).offset.offset(i as isize) as isize)
            .offset(-1);
        size = (*(*idx).offset.offset((i as libc::c_int + 1i32) as isize))
            .wrapping_sub(*(*idx).offset.offset(i as isize)) as libc::c_int;
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
    return len;
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
pub unsafe extern "C" fn cff_read_private(mut cff: *mut cff_font) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if (*cff).flag & 1i32 << 0i32 != 0 {
        let mut i: libc::c_int = 0;
        if (*cff).fdarray.is_null() {
            cff_read_fdarray(cff);
        }
        (*cff).private = new(((*cff).num_fds as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as libc::c_ulong)
            as uint32_t) as *mut *mut cff_dict;
        i = 0i32;
        while i < (*cff).num_fds as libc::c_int {
            if !(*(*cff).fdarray.offset(i as isize)).is_null()
                && cff_dict_known(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const libc::c_char,
                ) != 0
                && {
                    size = cff_dict_get(
                        *(*cff).fdarray.offset(i as isize),
                        b"Private\x00" as *const u8 as *const libc::c_char,
                        0i32,
                    ) as libc::c_int;
                    size > 0i32
                }
            {
                offset = cff_dict_get(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const libc::c_char,
                    1i32,
                ) as libc::c_int;
                ttstub_input_seek(
                    (*cff).handle,
                    (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
                    0i32,
                );
                data = new((size as uint32_t as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                    as uint32_t) as *mut card8;
                if ttstub_input_read((*cff).handle, data as *mut libc::c_char, size as size_t)
                    != size as libc::c_long
                {
                    _tt_abort(b"reading file failed\x00" as *const u8 as *const libc::c_char);
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
        (*cff).private = new((1i32 as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as libc::c_ulong)
            as uint32_t) as *mut *mut cff_dict;
        if cff_dict_known(
            (*cff).topdict,
            b"Private\x00" as *const u8 as *const libc::c_char,
        ) != 0
            && {
                size = cff_dict_get(
                    (*cff).topdict,
                    b"Private\x00" as *const u8 as *const libc::c_char,
                    0i32,
                ) as libc::c_int;
                size > 0i32
            }
        {
            offset = cff_dict_get(
                (*cff).topdict,
                b"Private\x00" as *const u8 as *const libc::c_char,
                1i32,
            ) as libc::c_int;
            ttstub_input_seek(
                (*cff).handle,
                (*cff).offset.wrapping_add(offset as libc::c_uint) as ssize_t,
                0i32,
            );
            data = new((size as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                as uint32_t) as *mut card8;
            if ttstub_input_read((*cff).handle, data as *mut libc::c_char, size as size_t)
                != size as libc::c_long
            {
                _tt_abort(b"reading file failed\x00" as *const u8 as *const libc::c_char);
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
    return len;
}
