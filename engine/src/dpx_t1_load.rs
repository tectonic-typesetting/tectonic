#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pst_obj;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
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
    /* String */
    #[no_mangle]
    fn cff_update_string(cff: *mut cff_font);
    #[no_mangle]
    fn cff_add_string(cff: *mut cff_font, str: *const libc::c_char, unique: libc::c_int) -> s_SID;
    #[no_mangle]
    fn cff_get_sid(cff: *mut cff_font, str: *const libc::c_char) -> libc::c_int;
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
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn cff_close(cff: *mut cff_font);
    #[no_mangle]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[no_mangle]
    fn cff_set_name(cff: *mut cff_font, name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    #[no_mangle]
    fn cff_new_dict() -> *mut cff_dict;
    #[no_mangle]
    fn cff_dict_set(
        dict: *mut cff_dict,
        key: *const libc::c_char,
        idx: libc::c_int,
        value: libc::c_double,
    );
    #[no_mangle]
    fn cff_dict_add(dict: *mut cff_dict, key: *const libc::c_char, count: libc::c_int);
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
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pst_get_token(inbuf: *mut *mut libc::c_uchar, inbufend: *mut libc::c_uchar) -> *mut pst_obj;
    #[no_mangle]
    fn pst_release_obj(obj: *mut pst_obj);
    #[no_mangle]
    fn pst_type_of(obj: *mut pst_obj) -> pst_type;
    #[no_mangle]
    fn pst_getIV(obj: *mut pst_obj) -> libc::c_int;
    #[no_mangle]
    fn pst_getRV(obj: *mut pst_obj) -> libc::c_double;
    #[no_mangle]
    fn pst_getSV(obj: *mut pst_obj) -> *mut libc::c_uchar;
    #[no_mangle]
    fn pst_data_ptr(obj: *mut pst_obj) -> *mut libc::c_void;
}
pub type __ssize_t = libc::c_long;
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
pub type l_offset = u32;
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
pub type pst_type = libc::c_int;
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
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
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn t1_decrypt(
    mut key: libc::c_ushort,
    mut dst: *mut libc::c_uchar,
    mut src: *const libc::c_uchar,
    mut skip: libc::c_int,
    mut len: libc::c_int,
) {
    len -= skip;
    loop {
        let fresh0 = skip;
        skip = skip - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = src;
        src = src.offset(1);
        key = ((key as libc::c_int + *fresh1 as libc::c_int) as libc::c_uint)
            .wrapping_mul(52845u32)
            .wrapping_add(22719u32) as libc::c_ushort
    }
    loop {
        let fresh2 = len;
        len = len - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = src;
        src = src.offset(1);
        let mut c: libc::c_uchar = *fresh3;
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = (c as libc::c_int ^ key as libc::c_int >> 8i32) as libc::c_uchar;
        key = ((key as libc::c_int + c as libc::c_int) as libc::c_uint)
            .wrapping_mul(52845u32)
            .wrapping_add(22719u32) as libc::c_ushort
    }
}
/* T1CRYPT */
unsafe extern "C" fn get_next_key(
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> *mut libc::c_char {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    while *start < end && {
        tok = pst_get_token(start, end);
        !tok.is_null()
    } {
        if pst_type_of(tok) == 6i32 {
            key = pst_getSV(tok) as *mut libc::c_char;
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            break;
        } else if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
    }
    return key;
}
unsafe extern "C" fn seek_operator(
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut op: *const libc::c_char,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    while *start < end && {
        tok = pst_get_token(start, end);
        !tok.is_null()
    } {
        if !tok.is_null()
            && pst_type_of(tok) < 0i32
            && !strstartswith(pst_data_ptr(tok) as *const libc::c_char, op).is_null()
        {
            break;
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
    }
    if tok.is_null() {
        return -1i32;
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    return 0i32;
}
unsafe extern "C" fn parse_svalue(
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut value: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -1i32;
    } else {
        if pst_type_of(tok) == 6i32 || pst_type_of(tok) == 5i32 {
            *value = pst_getSV(tok) as *mut libc::c_char
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            return -1i32;
        }
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    return 1i32;
}
unsafe extern "C" fn parse_bvalue(
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut value: *mut libc::c_double,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -1i32;
    } else {
        if pst_type_of(tok) == 1i32 {
            *value = pst_getIV(tok) as libc::c_double
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            return -1i32;
        }
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    return 1i32;
}
unsafe extern "C" fn parse_nvalue(
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut value: *mut libc::c_double,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut argn: libc::c_int = 0i32;
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -1i32;
    }
    /*
     * All array elements must be numeric token. (ATM compatible)
     */
    if (pst_type_of(tok) == 2i32 || pst_type_of(tok) == 3i32) && max > 0i32 {
        *value.offset(0) = pst_getRV(tok);
        argn = 1i32
    } else if pst_type_of(tok) == 7i32 {
        /* It does not distinguish '[' and '{'... */
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        while *start < end
            && {
                tok = pst_get_token(start, end);
                !tok.is_null()
            }
            && (pst_type_of(tok) == 2i32 || pst_type_of(tok) == 3i32)
            && argn < max
        {
            let fresh5 = argn;
            argn = argn + 1;
            *value.offset(fresh5 as isize) = pst_getRV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
        }
        if tok.is_null() {
            return -1i32;
        }
        if !(!tok.is_null()
            && pst_type_of(tok) < 0i32
            && !strstartswith(
                pst_data_ptr(tok) as *const libc::c_char,
                b"]\x00" as *const u8 as *const libc::c_char,
            )
            .is_null())
            && !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"}\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
        {
            argn = -1i32
        }
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    return argn;
}
static mut StandardEncoding: [*const libc::c_char; 256] = [
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
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
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
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
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"endash\x00" as *const u8 as *const libc::c_char,
    b"dagger\x00" as *const u8 as *const libc::c_char,
    b"daggerdbl\x00" as *const u8 as *const libc::c_char,
    b"periodcentered\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"paragraph\x00" as *const u8 as *const libc::c_char,
    b"bullet\x00" as *const u8 as *const libc::c_char,
    b"quotesinglbase\x00" as *const u8 as *const libc::c_char,
    b"quotedblbase\x00" as *const u8 as *const libc::c_char,
    b"quotedblright\x00" as *const u8 as *const libc::c_char,
    b"guillemotright\x00" as *const u8 as *const libc::c_char,
    b"ellipsis\x00" as *const u8 as *const libc::c_char,
    b"perthousand\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"questiondown\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"grave\x00" as *const u8 as *const libc::c_char,
    b"acute\x00" as *const u8 as *const libc::c_char,
    b"circumflex\x00" as *const u8 as *const libc::c_char,
    b"tilde\x00" as *const u8 as *const libc::c_char,
    b"macron\x00" as *const u8 as *const libc::c_char,
    b"breve\x00" as *const u8 as *const libc::c_char,
    b"dotaccent\x00" as *const u8 as *const libc::c_char,
    b"dieresis\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"ring\x00" as *const u8 as *const libc::c_char,
    b"cedilla\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"hungarumlaut\x00" as *const u8 as *const libc::c_char,
    b"ogonek\x00" as *const u8 as *const libc::c_char,
    b"caron\x00" as *const u8 as *const libc::c_char,
    b"emdash\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"AE\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"ordfeminine\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"Lslash\x00" as *const u8 as *const libc::c_char,
    b"Oslash\x00" as *const u8 as *const libc::c_char,
    b"OE\x00" as *const u8 as *const libc::c_char,
    b"ordmasculine\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"ae\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"dotlessi\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"lslash\x00" as *const u8 as *const libc::c_char,
    b"oslash\x00" as *const u8 as *const libc::c_char,
    b"oe\x00" as *const u8 as *const libc::c_char,
    b"germandbls\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
];
static mut ISOLatin1Encoding: [*const libc::c_char; 256] = [
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"space\x00" as *const u8 as *const libc::c_char,
    b"exclam\x00" as *const u8 as *const libc::c_char,
    b"quotedbl\x00" as *const u8 as *const libc::c_char,
    b"numbersign\x00" as *const u8 as *const libc::c_char,
    b"dollar\x00" as *const u8 as *const libc::c_char,
    b"percent\x00" as *const u8 as *const libc::c_char,
    b"ampersand\x00" as *const u8 as *const libc::c_char,
    b"quotesingle\x00" as *const u8 as *const libc::c_char,
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
    b"grave\x00" as *const u8 as *const libc::c_char,
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
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"dotlessi\x00" as *const u8 as *const libc::c_char,
    b"quoteleft\x00" as *const u8 as *const libc::c_char,
    b"quoteright\x00" as *const u8 as *const libc::c_char,
    b"circumflex\x00" as *const u8 as *const libc::c_char,
    b"tilde\x00" as *const u8 as *const libc::c_char,
    b"macron\x00" as *const u8 as *const libc::c_char,
    b"breve\x00" as *const u8 as *const libc::c_char,
    b"dotaccent\x00" as *const u8 as *const libc::c_char,
    b"dieresis\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"ring\x00" as *const u8 as *const libc::c_char,
    b"cedilla\x00" as *const u8 as *const libc::c_char,
    b".notdef\x00" as *const u8 as *const libc::c_char,
    b"hungarumlaut\x00" as *const u8 as *const libc::c_char,
    b"ogonek\x00" as *const u8 as *const libc::c_char,
    b"caron\x00" as *const u8 as *const libc::c_char,
    b"space\x00" as *const u8 as *const libc::c_char,
    b"exclamdown\x00" as *const u8 as *const libc::c_char,
    b"cent\x00" as *const u8 as *const libc::c_char,
    b"sterling\x00" as *const u8 as *const libc::c_char,
    b"currency\x00" as *const u8 as *const libc::c_char,
    b"yen\x00" as *const u8 as *const libc::c_char,
    b"brokenbar\x00" as *const u8 as *const libc::c_char,
    b"section\x00" as *const u8 as *const libc::c_char,
    b"dieresis\x00" as *const u8 as *const libc::c_char,
    b"copyright\x00" as *const u8 as *const libc::c_char,
    b"ordfeminine\x00" as *const u8 as *const libc::c_char,
    b"guillemotleft\x00" as *const u8 as *const libc::c_char,
    b"logicalnot\x00" as *const u8 as *const libc::c_char,
    b"hyphen\x00" as *const u8 as *const libc::c_char,
    b"registered\x00" as *const u8 as *const libc::c_char,
    b"macron\x00" as *const u8 as *const libc::c_char,
    b"degree\x00" as *const u8 as *const libc::c_char,
    b"plusminus\x00" as *const u8 as *const libc::c_char,
    b"twosuperior\x00" as *const u8 as *const libc::c_char,
    b"threesuperior\x00" as *const u8 as *const libc::c_char,
    b"acute\x00" as *const u8 as *const libc::c_char,
    b"mu\x00" as *const u8 as *const libc::c_char,
    b"paragraph\x00" as *const u8 as *const libc::c_char,
    b"periodcentered\x00" as *const u8 as *const libc::c_char,
    b"cedilla\x00" as *const u8 as *const libc::c_char,
    b"onesuperior\x00" as *const u8 as *const libc::c_char,
    b"ordmasculine\x00" as *const u8 as *const libc::c_char,
    b"guillemotright\x00" as *const u8 as *const libc::c_char,
    b"onequarter\x00" as *const u8 as *const libc::c_char,
    b"onehalf\x00" as *const u8 as *const libc::c_char,
    b"threequarters\x00" as *const u8 as *const libc::c_char,
    b"questiondown\x00" as *const u8 as *const libc::c_char,
    b"Agrave\x00" as *const u8 as *const libc::c_char,
    b"Aacute\x00" as *const u8 as *const libc::c_char,
    b"Acircumflex\x00" as *const u8 as *const libc::c_char,
    b"Atilde\x00" as *const u8 as *const libc::c_char,
    b"Adieresis\x00" as *const u8 as *const libc::c_char,
    b"Aring\x00" as *const u8 as *const libc::c_char,
    b"AE\x00" as *const u8 as *const libc::c_char,
    b"Ccedilla\x00" as *const u8 as *const libc::c_char,
    b"Egrave\x00" as *const u8 as *const libc::c_char,
    b"Eacute\x00" as *const u8 as *const libc::c_char,
    b"Ecircumflex\x00" as *const u8 as *const libc::c_char,
    b"Edieresis\x00" as *const u8 as *const libc::c_char,
    b"Igrave\x00" as *const u8 as *const libc::c_char,
    b"Iacute\x00" as *const u8 as *const libc::c_char,
    b"Icircumflex\x00" as *const u8 as *const libc::c_char,
    b"Idieresis\x00" as *const u8 as *const libc::c_char,
    b"Eth\x00" as *const u8 as *const libc::c_char,
    b"Ntilde\x00" as *const u8 as *const libc::c_char,
    b"Ograve\x00" as *const u8 as *const libc::c_char,
    b"Oacute\x00" as *const u8 as *const libc::c_char,
    b"Ocircumflex\x00" as *const u8 as *const libc::c_char,
    b"Otilde\x00" as *const u8 as *const libc::c_char,
    b"Odieresis\x00" as *const u8 as *const libc::c_char,
    b"multiply\x00" as *const u8 as *const libc::c_char,
    b"Oslash\x00" as *const u8 as *const libc::c_char,
    b"Ugrave\x00" as *const u8 as *const libc::c_char,
    b"Uacute\x00" as *const u8 as *const libc::c_char,
    b"Ucircumflex\x00" as *const u8 as *const libc::c_char,
    b"Udieresis\x00" as *const u8 as *const libc::c_char,
    b"Yacute\x00" as *const u8 as *const libc::c_char,
    b"Thorn\x00" as *const u8 as *const libc::c_char,
    b"germandbls\x00" as *const u8 as *const libc::c_char,
    b"agrave\x00" as *const u8 as *const libc::c_char,
    b"aacute\x00" as *const u8 as *const libc::c_char,
    b"acircumflex\x00" as *const u8 as *const libc::c_char,
    b"atilde\x00" as *const u8 as *const libc::c_char,
    b"adieresis\x00" as *const u8 as *const libc::c_char,
    b"aring\x00" as *const u8 as *const libc::c_char,
    b"ae\x00" as *const u8 as *const libc::c_char,
    b"ccedilla\x00" as *const u8 as *const libc::c_char,
    b"egrave\x00" as *const u8 as *const libc::c_char,
    b"eacute\x00" as *const u8 as *const libc::c_char,
    b"ecircumflex\x00" as *const u8 as *const libc::c_char,
    b"edieresis\x00" as *const u8 as *const libc::c_char,
    b"igrave\x00" as *const u8 as *const libc::c_char,
    b"iacute\x00" as *const u8 as *const libc::c_char,
    b"icircumflex\x00" as *const u8 as *const libc::c_char,
    b"idieresis\x00" as *const u8 as *const libc::c_char,
    b"eth\x00" as *const u8 as *const libc::c_char,
    b"ntilde\x00" as *const u8 as *const libc::c_char,
    b"ograve\x00" as *const u8 as *const libc::c_char,
    b"oacute\x00" as *const u8 as *const libc::c_char,
    b"ocircumflex\x00" as *const u8 as *const libc::c_char,
    b"otilde\x00" as *const u8 as *const libc::c_char,
    b"odieresis\x00" as *const u8 as *const libc::c_char,
    b"divide\x00" as *const u8 as *const libc::c_char,
    b"oslash\x00" as *const u8 as *const libc::c_char,
    b"ugrave\x00" as *const u8 as *const libc::c_char,
    b"uacute\x00" as *const u8 as *const libc::c_char,
    b"ucircumflex\x00" as *const u8 as *const libc::c_char,
    b"udieresis\x00" as *const u8 as *const libc::c_char,
    b"yacute\x00" as *const u8 as *const libc::c_char,
    b"thorn\x00" as *const u8 as *const libc::c_char,
    b"ydieresis\x00" as *const u8 as *const libc::c_char,
];
/* Treat cases such as "dup num num getinterval num exch putinterval"
 * or "dup num exch num get put"
 */
unsafe extern "C" fn try_put_or_putinterval(
    mut enc_vec: *mut *mut libc::c_char,
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut i: libc::c_int = 0;
    let mut num1: libc::c_int = 0;
    let mut num2: libc::c_int = 0;
    let mut num3: libc::c_int = 0;
    tok = pst_get_token(start, end);
    if tok.is_null()
        || !(pst_type_of(tok) == 2i32)
        || {
            num1 = pst_getIV(tok);
            num1 > 255i32
        }
        || num1 < 0i32
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        return -1i32;
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -1i32;
    } else {
        if !tok.is_null()
            && pst_type_of(tok) < 0i32
            && !strstartswith(
                pst_data_ptr(tok) as *const libc::c_char,
                b"exch\x00" as *const u8 as *const libc::c_char,
            )
            .is_null()
        {
            /* dup num exch num get put */
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if tok.is_null()
                || !(pst_type_of(tok) == 2i32)
                || {
                    num2 = pst_getIV(tok);
                    num2 > 255i32
                }
                || num2 < 0i32
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"get\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"put\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            free(*enc_vec.offset(num1 as isize) as *mut libc::c_void);
            let ref mut fresh6 = *enc_vec.offset(num1 as isize);
            *fresh6 = xstrdup(*enc_vec.offset(num2 as isize))
        } else if pst_type_of(tok) == 2i32
            && {
                num2 = pst_getIV(tok);
                num2 + num1 <= 255i32
            }
            && num2 >= 0i32
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"getinterval\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if tok.is_null()
                || !(pst_type_of(tok) == 2i32)
                || {
                    num3 = pst_getIV(tok);
                    num3 + num2 > 255i32
                }
                || num3 < 0i32
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"exch\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"putinterval\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            i = 0i32;
            while i < num2 {
                if !(*enc_vec.offset((num1 + i) as isize)).is_null() {
                    /* num1 + i < 256 here */
                    let ref mut fresh7 = *enc_vec.offset((num3 + i) as isize);
                    *fresh7 = mfree(*enc_vec.offset((num3 + i) as isize) as *mut libc::c_void)
                        as *mut libc::c_char;
                    let ref mut fresh8 = *enc_vec.offset((num3 + i) as isize);
                    *fresh8 = xstrdup(*enc_vec.offset((num1 + i) as isize))
                }
                i += 1
            }
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            return -1i32;
        }
    }
    return 0i32;
}
unsafe extern "C" fn parse_encoding(
    mut enc_vec: *mut *mut libc::c_char,
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut code: libc::c_int = 0;
    /*
     *  StandardEncoding def
     * or
     *  ISOLatin1Encoding def
     * or
     *  0 1 255 {1 index exch /.notdef put } for
     *  dup int name put
     *  ...
     *  [readonly] def
     */
    tok = pst_get_token(start, end);
    if !tok.is_null()
        && pst_type_of(tok) < 0i32
        && !strstartswith(
            pst_data_ptr(tok) as *const libc::c_char,
            b"StandardEncoding\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        if !enc_vec.is_null() {
            code = 0i32;
            while code < 256i32 {
                if !StandardEncoding[code as usize].is_null()
                    && strcmp(
                        StandardEncoding[code as usize],
                        b".notdef\x00" as *const u8 as *const libc::c_char,
                    ) != 0i32
                {
                    let ref mut fresh9 = *enc_vec.offset(code as isize);
                    *fresh9 = new((strlen(StandardEncoding[code as usize])
                        .wrapping_add(1i32 as libc::c_ulong)
                        as u32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        as u32) as *mut libc::c_char;
                    strcpy(
                        *enc_vec.offset(code as isize),
                        StandardEncoding[code as usize],
                    );
                } else {
                    let ref mut fresh10 = *enc_vec.offset(code as isize);
                    *fresh10 = 0 as *mut libc::c_char
                }
                code += 1
            }
        }
    } else if !tok.is_null()
        && pst_type_of(tok) < 0i32
        && !strstartswith(
            pst_data_ptr(tok) as *const libc::c_char,
            b"ISOLatin1Encoding\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        if !enc_vec.is_null() {
            code = 0i32;
            while code < 256i32 {
                if !ISOLatin1Encoding[code as usize].is_null()
                    && strcmp(
                        ISOLatin1Encoding[code as usize],
                        b".notdef\x00" as *const u8 as *const libc::c_char,
                    ) != 0i32
                {
                    let ref mut fresh11 = *enc_vec.offset(code as isize);
                    *fresh11 = new((strlen(ISOLatin1Encoding[code as usize])
                        .wrapping_add(1i32 as libc::c_ulong)
                        as u32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        as u32) as *mut libc::c_char;
                    strcpy(
                        *enc_vec.offset(code as isize),
                        ISOLatin1Encoding[code as usize],
                    );
                } else {
                    let ref mut fresh12 = *enc_vec.offset(code as isize);
                    *fresh12 = 0 as *mut libc::c_char
                }
                code += 1
            }
        }
    } else if !tok.is_null()
        && pst_type_of(tok) < 0i32
        && !strstartswith(
            pst_data_ptr(tok) as *const libc::c_char,
            b"ExpertEncoding\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        if !enc_vec.is_null() {
            dpx_warning(b"ExpertEncoding not supported.\x00" as *const u8 as *const libc::c_char);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            return -1i32;
        }
    /*
     * Not supported yet.
     */
    } else {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        seek_operator(start, end, b"array\x00" as *const u8 as *const libc::c_char);
        /*
         * Pick all seaquences that matches "dup n /Name put" until
         * occurrence of "def" or "readonly".
         */
        while *start < end && {
            tok = pst_get_token(start, end);
            !tok.is_null()
        } {
            if !tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"def\x00" as *const u8 as *const libc::c_char,
                )
                .is_null()
                || !tok.is_null()
                    && pst_type_of(tok) < 0i32
                    && !strstartswith(
                        pst_data_ptr(tok) as *const libc::c_char,
                        b"readonly\x00" as *const u8 as *const libc::c_char,
                    )
                    .is_null()
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                break;
            } else if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"dup\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
            } else {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                /* cmctt10.pfb for examples contains the following PS code
                 *     dup num num getinterval num exch putinterval
                 *     dup num exch num get put
                 */
                tok = pst_get_token(start, end);
                if !tok.is_null()
                    && pst_type_of(tok) < 0i32
                    && !strstartswith(
                        pst_data_ptr(tok) as *const libc::c_char,
                        b"dup\x00" as *const u8 as *const libc::c_char,
                    )
                    .is_null()
                {
                    /* possibly putinterval type */
                    if enc_vec.is_null() {
                        dpx_warning(b"This kind of type1 fonts are not supported as native fonts.\n                   They are supported if used with tfm fonts.\n\x00"
                                        as *const u8 as *const libc::c_char);
                    } else {
                        try_put_or_putinterval(enc_vec, start, end);
                    }
                    if !tok.is_null() {
                        pst_release_obj(tok);
                        tok = 0 as *mut pst_obj
                    }
                } else if tok.is_null()
                    || !(pst_type_of(tok) == 2i32)
                    || {
                        code = pst_getIV(tok);
                        code > 255i32
                    }
                    || code < 0i32
                {
                    if !tok.is_null() {
                        pst_release_obj(tok);
                        tok = 0 as *mut pst_obj
                    }
                } else {
                    if !tok.is_null() {
                        pst_release_obj(tok);
                        tok = 0 as *mut pst_obj
                    }
                    tok = pst_get_token(start, end);
                    if tok.is_null() || !(pst_type_of(tok) == 6i32) {
                        if !tok.is_null() {
                            pst_release_obj(tok);
                            tok = 0 as *mut pst_obj
                        }
                    } else {
                        if !enc_vec.is_null() {
                            free(*enc_vec.offset(code as isize) as *mut libc::c_void);
                            let ref mut fresh13 = *enc_vec.offset(code as isize);
                            *fresh13 = pst_getSV(tok) as *mut libc::c_char
                        }
                        if !tok.is_null() {
                            pst_release_obj(tok);
                            tok = 0 as *mut pst_obj
                        }
                        tok = pst_get_token(start, end);
                        if !(!tok.is_null()
                            && pst_type_of(tok) < 0i32
                            && !strstartswith(
                                pst_data_ptr(tok) as *const libc::c_char,
                                b"put\x00" as *const u8 as *const libc::c_char,
                            )
                            .is_null())
                        {
                            let ref mut fresh14 = *enc_vec.offset(code as isize);
                            *fresh14 = mfree(*enc_vec.offset(code as isize) as *mut libc::c_void)
                                as *mut libc::c_char;
                            if !tok.is_null() {
                                pst_release_obj(tok);
                                tok = 0 as *mut pst_obj
                            }
                        } else if !tok.is_null() {
                            pst_release_obj(tok);
                            tok = 0 as *mut pst_obj
                        }
                    }
                }
            }
        }
    }
    return 0i32;
}
unsafe extern "C" fn parse_subrs(
    mut font: *mut cff_font,
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut lenIV: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut subrs: *mut cff_index = 0 as *mut cff_index;
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut max_size: libc::c_int = 0;
    let mut offsets: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lengths: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut data: *mut card8 = 0 as *mut card8;
    tok = pst_get_token(start, end);
    if !(pst_type_of(tok) == 2i32) || pst_getIV(tok) < 0i32 {
        dpx_warning(b"Parsing Subrs failed.\x00" as *const u8 as *const libc::c_char);
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        return -1i32;
    }
    count = pst_getIV(tok);
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    if count == 0i32 {
        let ref mut fresh15 = *(*font).subrs.offset(0);
        *fresh15 = 0 as *mut cff_index;
        return 0i32;
    }
    tok = pst_get_token(start, end);
    if !(!tok.is_null()
        && pst_type_of(tok) < 0i32
        && !strstartswith(
            pst_data_ptr(tok) as *const libc::c_char,
            b"array\x00" as *const u8 as *const libc::c_char,
        )
        .is_null())
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        return -1i32;
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    if mode != 1i32 {
        max_size = 65536i32;
        data = new((max_size as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
            as u32) as *mut card8;
        offsets = new((count as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as u32) as *mut libc::c_int;
        lengths = new((count as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as u32) as *mut libc::c_int;
        memset(
            offsets as *mut libc::c_void,
            0i32,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(count as libc::c_ulong),
        );
        memset(
            lengths as *mut libc::c_void,
            0i32,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(count as libc::c_ulong),
        );
    } else {
        max_size = 0i32;
        data = 0 as *mut card8;
        offsets = 0 as *mut libc::c_int;
        lengths = 0 as *mut libc::c_int
    }
    offset = 0i32;
    /* dup subr# n-bytes RD n-binary-bytes NP */
    i = 0i32;
    while i < count {
        let mut idx: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        tok = pst_get_token(start, end);
        if tok.is_null() {
            free(data as *mut libc::c_void);
            free(offsets as *mut libc::c_void);
            free(lengths as *mut libc::c_void);
            return -1i32;
        } else if !tok.is_null()
            && pst_type_of(tok) < 0i32
            && !strstartswith(
                pst_data_ptr(tok) as *const libc::c_char,
                b"ND\x00" as *const u8 as *const libc::c_char,
            )
            .is_null()
            || !tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"|-\x00" as *const u8 as *const libc::c_char,
                )
                .is_null()
            || !tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"def\x00" as *const u8 as *const libc::c_char,
                )
                .is_null()
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            break;
        } else if !(!tok.is_null()
            && pst_type_of(tok) < 0i32
            && !strstartswith(
                pst_data_ptr(tok) as *const libc::c_char,
                b"dup\x00" as *const u8 as *const libc::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            /* Found "dup" */
            tok = pst_get_token(start, end);
            if !(pst_type_of(tok) == 2i32) || pst_getIV(tok) < 0i32 || pst_getIV(tok) >= count {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                free(data as *mut libc::c_void);
                free(offsets as *mut libc::c_void);
                free(lengths as *mut libc::c_void);
                return -1i32;
            }
            idx = pst_getIV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(pst_type_of(tok) == 2i32) || pst_getIV(tok) < 0i32 || pst_getIV(tok) > 65536i32 {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            len = pst_getIV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"RD\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
                && !(!tok.is_null()
                    && pst_type_of(tok) < 0i32
                    && !strstartswith(
                        pst_data_ptr(tok) as *const libc::c_char,
                        b"-|\x00" as *const u8 as *const libc::c_char,
                    )
                    .is_null())
                && seek_operator(
                    start,
                    end,
                    b"readstring\x00" as *const u8 as *const libc::c_char,
                ) < 0i32
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                free(data as *mut libc::c_void);
                free(offsets as *mut libc::c_void);
                free(lengths as *mut libc::c_void);
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            *start = (*start).offset(1);
            if (*start).offset(len as isize) >= end {
                free(data as *mut libc::c_void);
                free(offsets as *mut libc::c_void);
                free(lengths as *mut libc::c_void);
                return -1i32;
            }
            if mode != 1i32 {
                if offset + len >= max_size {
                    max_size += 65536i32;
                    data =
                        renew(
                            data as *mut libc::c_void,
                            (max_size as u32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                                as u32,
                        ) as *mut card8
                }
                if lenIV >= 0i32 {
                    t1_decrypt(
                        4330u32 as libc::c_ushort,
                        data.offset(offset as isize),
                        *start,
                        lenIV,
                        len,
                    );
                    *offsets.offset(idx as isize) = offset;
                    let ref mut fresh16 = *lengths.offset(idx as isize);
                    *fresh16 = len - lenIV;
                    offset += *fresh16
                } else if len > 0i32 {
                    *offsets.offset(idx as isize) = offset;
                    *lengths.offset(idx as isize) = len;
                    memcpy(
                        &mut *data.offset(offset as isize) as *mut card8 as *mut libc::c_void,
                        *start as *const libc::c_void,
                        len as libc::c_ulong,
                    );
                    offset += len
                }
            }
            *start = (*start).offset(len as isize);
            i += 1
        }
    }
    if mode != 1i32 {
        if (*(*font).subrs.offset(0)).is_null() {
            let ref mut fresh17 = *(*font).subrs.offset(0);
            *fresh17 = cff_new_index(count as card16);
            subrs = *fresh17;
            (*subrs).data = new((offset as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                as u32) as *mut card8;
            offset = 0i32;
            i = 0i32;
            while i < count {
                *(*subrs).offset.offset(i as isize) = (offset + 1i32) as l_offset;
                if *lengths.offset(i as isize) > 0i32 {
                    memcpy(
                        (*subrs).data.offset(offset as isize) as *mut libc::c_void,
                        data.offset(*offsets.offset(i as isize) as isize) as *const libc::c_void,
                        *lengths.offset(i as isize) as libc::c_ulong,
                    );
                    offset += *lengths.offset(i as isize)
                }
                i += 1
            }
            *(*subrs).offset.offset(count as isize) = (offset + 1i32) as l_offset
        } else {
            /* Adobe's OPO_____.PFB and OPBO____.PFB have two /Subrs dicts,
             * and also have /CharStrings not followed by dicts.
             * Simply ignores those data. By ChoF on 2009/04/08. */
            dpx_warning(
                b"Already found /Subrs; ignores the other /Subrs dicts.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        free(data as *mut libc::c_void);
        free(offsets as *mut libc::c_void);
        free(lengths as *mut libc::c_void);
    }
    return 0i32;
}
unsafe extern "C" fn parse_charstrings(
    mut font: *mut cff_font,
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut lenIV: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut charstrings: *mut cff_index = 0 as *mut cff_index;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut have_notdef: libc::c_int = 0;
    let mut max_size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    /* /CharStrings n dict dup begin
     * /GlyphName n-bytes RD -n-binary-bytes- ND
     * ...
     * end
     *  - stack - ... /CharStrings dict
     */
    tok = pst_get_token(start, end); /* .notdef must be at gid = 0 in CFF */
    if !(pst_type_of(tok) == 2i32) || pst_getIV(tok) < 0i32 || pst_getIV(tok) > 64999i32 {
        let mut s: *mut libc::c_uchar = pst_getSV(tok);
        dpx_warning(
            b"Ignores non dict \"/CharStrings %s ...\"\x00" as *const u8 as *const libc::c_char,
            s,
        );
        free(s as *mut libc::c_void);
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = 0 as *mut pst_obj
        }
        return 0i32;
    }
    count = pst_getIV(tok);
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = 0 as *mut pst_obj
    }
    if mode != 1i32 {
        charstrings = cff_new_index(count as card16);
        max_size = 65536i32;
        (*charstrings).data = new((max_size as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
            as u32) as *mut card8
    } else {
        charstrings = 0 as *mut cff_index;
        max_size = 0i32
    }
    (*font).cstrings = charstrings;
    (*font).charsets = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_charsets>() as libc::c_ulong)
        as u32) as *mut cff_charsets;
    charset = (*font).charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = (count - 1i32) as card16;
    (*charset).data.glyphs = new(((count - 1i32) as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<s_SID>() as libc::c_ulong)
        as u32) as *mut s_SID;
    memset(
        (*charset).data.glyphs as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<s_SID>() as libc::c_ulong)
            .wrapping_mul((count - 1i32) as libc::c_ulong),
    );
    offset = 0i32;
    have_notdef = 0i32;
    (*font).is_notdef_notzero = 0i32;
    seek_operator(start, end, b"begin\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < count {
        let mut glyph_name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        let mut gid: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        /* BUG-20061126 (by ChoF):
         * Some fonts (e.g., belleek/blsy.pfb) does not have the correct number
         * of glyphs. Modify the codes even to work with these broken fonts.
         */
        tok = pst_get_token(start, end);
        glyph_name = pst_getSV(tok) as *mut libc::c_char;
        if i == 0i32
            && !glyph_name.is_null()
            && strcmp(
                glyph_name,
                b".notdef\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
        {
            (*font).is_notdef_notzero = 1i32
        }
        if pst_type_of(tok) == 6i32 {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            if glyph_name.is_null() {
                return -1i32;
            } else {
                if streq_ptr(
                    glyph_name,
                    b".notdef\x00" as *const u8 as *const libc::c_char,
                ) {
                    gid = 0i32;
                    have_notdef = 1i32
                } else if have_notdef != 0 {
                    gid = i
                } else if i == count - 1i32 {
                    dpx_warning(b"No .notdef glyph???\x00" as *const u8 as *const libc::c_char);
                    return -1i32;
                } else {
                    gid = i + 1i32
                }
            }
            if gid > 0i32 {
                *(*charset).data.glyphs.offset((gid - 1i32) as isize) =
                    cff_add_string(font, glyph_name, 0i32)
            }
            /*
             * We don't care about duplicate strings here since
             * later a subset font of this font will be generated.
             */
            free(glyph_name as *mut libc::c_void); /* start at 1 */
            tok = pst_get_token(start, end);
            if !(pst_type_of(tok) == 2i32) || pst_getIV(tok) < 0i32 || pst_getIV(tok) > 65536i32 {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            len = pst_getIV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"RD\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
                && !(!tok.is_null()
                    && pst_type_of(tok) < 0i32
                    && !strstartswith(
                        pst_data_ptr(tok) as *const libc::c_char,
                        b"-|\x00" as *const u8 as *const libc::c_char,
                    )
                    .is_null())
                && seek_operator(
                    start,
                    end,
                    b"readstring\x00" as *const u8 as *const libc::c_char,
                ) < 0i32
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            if (*start).offset(len as isize).offset(1) >= end {
                return -1i32;
            }
            if mode != 1i32 {
                if offset + len >= max_size {
                    max_size += if len > 65536i32 { len } else { 65536i32 };
                    (*charstrings).data =
                        renew(
                            (*charstrings).data as *mut libc::c_void,
                            (max_size as u32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<card8>() as libc::c_ulong)
                                as u32,
                        ) as *mut card8
                }
                if gid == 0i32 {
                    if lenIV >= 0i32 {
                        memmove(
                            (*charstrings)
                                .data
                                .offset(len as isize)
                                .offset(-(lenIV as isize))
                                as *mut libc::c_void,
                            (*charstrings).data as *const libc::c_void,
                            offset as libc::c_ulong,
                        );
                        j = 1i32;
                        while j <= i {
                            let ref mut fresh18 = *(*charstrings).offset.offset(j as isize);
                            *fresh18 = (*fresh18 as libc::c_uint)
                                .wrapping_add((len - lenIV) as libc::c_uint)
                                as l_offset as l_offset;
                            j += 1
                        }
                    } else {
                        memmove(
                            (*charstrings).data.offset(len as isize) as *mut libc::c_void,
                            (*charstrings).data as *const libc::c_void,
                            offset as libc::c_ulong,
                        );
                        j = 1i32;
                        while j <= i {
                            let ref mut fresh19 = *(*charstrings).offset.offset(j as isize);
                            *fresh19 = (*fresh19 as libc::c_uint).wrapping_add(len as libc::c_uint)
                                as l_offset as l_offset;
                            j += 1
                        }
                    }
                }
            }
            *start = (*start).offset(1);
            if mode != 1i32 {
                if lenIV >= 0i32 {
                    let mut offs: libc::c_int = if gid != 0 { offset } else { 0i32 };
                    *(*charstrings).offset.offset(gid as isize) = (offs + 1i32) as l_offset;
                    t1_decrypt(
                        4330u32 as libc::c_ushort,
                        (*charstrings).data.offset(offs as isize),
                        *start,
                        lenIV,
                        len,
                    );
                    offset += len - lenIV
                } else {
                    if gid == 0i32 {
                        *(*charstrings).offset.offset(gid as isize) = 1i32 as l_offset;
                        memcpy(
                            &mut *(*charstrings).data.offset(0) as *mut card8 as *mut libc::c_void,
                            *start as *const libc::c_void,
                            len as libc::c_ulong,
                        );
                    } else {
                        *(*charstrings).offset.offset(gid as isize) = (offset + 1i32) as l_offset;
                        memcpy(
                            &mut *(*charstrings).data.offset(offset as isize) as *mut card8
                                as *mut libc::c_void,
                            *start as *const libc::c_void,
                            len as libc::c_ulong,
                        );
                    }
                    offset += len
                }
            }
            *start = (*start).offset(len as isize);
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0i32
                && !strstartswith(
                    pst_data_ptr(tok) as *const libc::c_char,
                    b"ND\x00" as *const u8 as *const libc::c_char,
                )
                .is_null())
                && !(!tok.is_null()
                    && pst_type_of(tok) < 0i32
                    && !strstartswith(
                        pst_data_ptr(tok) as *const libc::c_char,
                        b"|-\x00" as *const u8 as *const libc::c_char,
                    )
                    .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = 0 as *mut pst_obj
                }
                return -1i32;
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            i += 1
        } else if pst_type_of(tok) < 0i32
            && streq_ptr(glyph_name, b"end\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            break;
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = 0 as *mut pst_obj
            }
            return -1i32;
        }
    }
    if mode != 1i32 {
        *(*charstrings).offset.offset(count as isize) = (offset + 1i32) as l_offset
    }
    (*font).num_glyphs = count as card16;
    return 0i32;
}
unsafe extern "C" fn parse_part2(
    mut font: *mut cff_font,
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: [libc::c_double; 127] = [0.; 127];
    let mut argn: libc::c_int = 0;
    let mut lenIV: libc::c_int = 4i32;
    while *start < end && {
        key = get_next_key(start, end);
        !key.is_null()
    } {
        if streq_ptr(key, b"Subrs\x00" as *const u8 as *const libc::c_char) {
            /* levIV must appear before Subrs */
            if parse_subrs(font, start, end, lenIV, mode) < 0i32 {
                free(key as *mut libc::c_void);
                return -1i32;
            }
        } else if streq_ptr(key, b"CharStrings\x00" as *const u8 as *const libc::c_char) {
            if parse_charstrings(font, start, end, lenIV, mode) < 0i32 {
                free(key as *mut libc::c_void);
                return -1i32;
            }
        } else if streq_ptr(key, b"lenIV\x00" as *const u8 as *const libc::c_char) {
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 1i32);
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            lenIV = argv[0] as libc::c_int
        } else if streq_ptr(key, b"BlueValues\x00" as *const u8 as *const libc::c_char)
            as libc::c_int
            != 0
            || streq_ptr(key, b"OtherBlues\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"FamilyBlues\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(
                key,
                b"FamilyOtherBlues\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_int
                != 0
            || streq_ptr(key, b"StemSnapH\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"StemSnapV\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
        {
            /*
             * Operand values are delta in CFF font dictionary encoding.
             */
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 127i32);
            if argn < 0i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    0i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            cff_dict_add(*(*font).private.offset(0), key, argn);
            loop {
                let fresh20 = argn;
                argn = argn - 1;
                if !(fresh20 > 0i32) {
                    break;
                }
                cff_dict_set(
                    *(*font).private.offset(0),
                    key,
                    argn,
                    if argn == 0i32 {
                        argv[argn as usize]
                    } else {
                        argv[argn as usize] - argv[(argn - 1i32) as usize]
                    },
                );
            }
        } else if streq_ptr(key, b"StdHW\x00" as *const u8 as *const libc::c_char) as libc::c_int
            != 0
            || streq_ptr(key, b"StdVW\x00" as *const u8 as *const libc::c_char) as libc::c_int != 0
            || streq_ptr(key, b"BlueScale\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"BlueShift\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"BlueFuzz\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(
                key,
                b"LanguageGroup\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_int
                != 0
            || streq_ptr(
                key,
                b"ExpansionFactor\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_int
                != 0
        {
            /*
             * Value of StdHW and StdVW is described as an array in the
             * Type 1 Font Specification but is a number in CFF format.
             */
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 1i32);
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            cff_dict_add(*(*font).private.offset(0), key, 1i32);
            cff_dict_set(*(*font).private.offset(0), key, 0i32, argv[0]);
        } else if streq_ptr(key, b"ForceBold\x00" as *const u8 as *const libc::c_char) {
            argn = parse_bvalue(start, end, &mut *argv.as_mut_ptr().offset(0));
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            if argv[0] != 0i32 as libc::c_double {
                cff_dict_add(*(*font).private.offset(0), key, 1i32);
                cff_dict_set(
                    *(*font).private.offset(0),
                    key,
                    0i32,
                    1i32 as libc::c_double,
                );
            }
        }
        /*
         * MinFeature, RndStemUp, UniqueID, Password ignored.
         */
        free(key as *mut libc::c_void); /* Macro CHECK_ARGN_XX assume 'argn' is used. */
    }
    return 0i32;
}
unsafe extern "C" fn parse_part1(
    mut font: *mut cff_font,
    mut enc_vec: *mut *mut libc::c_char,
    mut start: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> libc::c_int {
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: [libc::c_double; 127] = [0.; 127];
    let mut argn: libc::c_int = 0;
    /*
     * We skip PostScript code inserted before the beginning of
     * font dictionary so that parser will not be confused with
     * it. See LMRoman10-Regular (lmr10.pfb) for example.
     */
    if seek_operator(start, end, b"begin\x00" as *const u8 as *const libc::c_char) < 0i32 {
        return -1i32;
    }
    while *start < end && {
        key = get_next_key(start, end);
        !key.is_null()
    } {
        if streq_ptr(key, b"Encoding\x00" as *const u8 as *const libc::c_char) {
            if parse_encoding(enc_vec, start, end) < 0i32 {
                free(key as *mut libc::c_void);
                return -1i32;
            }
        } else if streq_ptr(key, b"FontName\x00" as *const u8 as *const libc::c_char) {
            argn = parse_svalue(start, end, &mut strval);
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            if strlen(strval) > 127i32 as libc::c_ulong {
                dpx_warning(
                    b"FontName too long: %s (%zu bytes)\x00" as *const u8 as *const libc::c_char,
                    strval,
                    strlen(strval),
                );
                *strval.offset(127) = '\u{0}' as i32 as libc::c_char
            }
            cff_set_name(font, strval);
            free(strval as *mut libc::c_void);
        } else if streq_ptr(key, b"FontType\x00" as *const u8 as *const libc::c_char) {
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 1i32);
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            if argv[0] != 1.0f64 {
                dpx_warning(
                    b"FontType %d not supported.\x00" as *const u8 as *const libc::c_char,
                    argv[0] as libc::c_int,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
        } else if streq_ptr(key, b"ItalicAngle\x00" as *const u8 as *const libc::c_char)
            as libc::c_int
            != 0
            || streq_ptr(key, b"StrokeWidth\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"PaintType\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
        {
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 1i32);
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            if argv[0] != 0.0f64 {
                cff_dict_add((*font).topdict, key, 1i32);
                cff_dict_set((*font).topdict, key, 0i32, argv[0]);
            }
        } else if streq_ptr(
            key,
            b"UnderLinePosition\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int
            != 0
            || streq_ptr(
                key,
                b"UnderLineThickness\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_int
                != 0
        {
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 1i32);
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            cff_dict_add((*font).topdict, key, 1i32);
            cff_dict_set((*font).topdict, key, 0i32, argv[0]);
        } else if streq_ptr(key, b"FontBBox\x00" as *const u8 as *const libc::c_char) {
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 4i32);
            if argn != 4i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    4i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            cff_dict_add((*font).topdict, key, 4i32);
            loop {
                let fresh21 = argn;
                argn = argn - 1;
                if !(fresh21 > 0i32) {
                    break;
                }
                cff_dict_set((*font).topdict, key, argn, argv[argn as usize]);
            }
        } else if streq_ptr(key, b"FontMatrix\x00" as *const u8 as *const libc::c_char) {
            argn = parse_nvalue(start, end, argv.as_mut_ptr(), 6i32);
            if argn != 6i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    6i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            if argv[0] != 0.001f64
                || argv[1] != 0.0f64
                || argv[2] != 0.0f64
                || argv[3] != 0.001f64
                || argv[4] != 0.0f64
                || argv[5] != 0.0f64
            {
                cff_dict_add((*font).topdict, key, 6i32);
                loop {
                    let fresh22 = argn;
                    argn = argn - 1;
                    if !(fresh22 > 0i32) {
                        break;
                    }
                    cff_dict_set((*font).topdict, key, argn, argv[argn as usize]);
                }
            }
        } else if streq_ptr(key, b"version\x00" as *const u8 as *const libc::c_char) as libc::c_int
            != 0
            || streq_ptr(key, b"Notice\x00" as *const u8 as *const libc::c_char) as libc::c_int != 0
            || streq_ptr(key, b"FullName\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"FamilyName\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
            || streq_ptr(key, b"Weight\x00" as *const u8 as *const libc::c_char) as libc::c_int != 0
            || streq_ptr(key, b"Copyright\x00" as *const u8 as *const libc::c_char) as libc::c_int
                != 0
        {
            /*
             * FontInfo
             */
            argn = parse_svalue(start, end, &mut strval); /* FIXME */
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            let mut sid: s_SID = 0;
            cff_dict_add((*font).topdict, key, 1i32);
            sid = cff_get_sid(font, strval) as s_SID;
            if sid as libc::c_int == 65535i32 {
                sid = cff_add_string(font, strval, 0i32)
            }
            /*
             * We don't care about duplicate strings here since
             * later a subset font of this font will be generated.
             */
            cff_dict_set((*font).topdict, key, 0i32, sid as libc::c_double); /* No Global Subr */
            free(strval as *mut libc::c_void);
        } else if streq_ptr(key, b"IsFixedPitch\x00" as *const u8 as *const libc::c_char) {
            argn = parse_bvalue(start, end, &mut *argv.as_mut_ptr().offset(0));
            if argn != 1i32 {
                dpx_warning(
                    b"%d values expected but only %d read.\x00" as *const u8 as *const libc::c_char,
                    1i32,
                    argn,
                );
                free(key as *mut libc::c_void);
                return -1i32;
            }
            if argv[0] != 0.0f64 {
                cff_dict_add(*(*font).private.offset(0), key, 1i32);
                cff_dict_set(
                    *(*font).private.offset(0),
                    key,
                    0i32,
                    1i32 as libc::c_double,
                );
            }
        }
        free(key as *mut libc::c_void);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_pfb(mut handle: rust_input_handle_t) -> bool {
    let mut sig: [libc::c_char; 15] = [0; 15];
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    ch = ttstub_input_getc(handle);
    if ch != 128i32
        || {
            ch = ttstub_input_getc(handle);
            ch < 0i32
        }
        || ch > 3i32
    {
        return 0i32 != 0;
    }
    i = 0i32;
    while i < 4i32 {
        ch = ttstub_input_getc(handle);
        if ch < 0i32 {
            return 0i32 != 0;
        }
        i += 1
    }
    i = 0i32;
    while i < 14i32 {
        ch = ttstub_input_getc(handle);
        if ch < 0i32 {
            return 0i32 != 0;
        }
        sig[i as usize] = ch as libc::c_char;
        i += 1
    }
    if memcmp(
        sig.as_mut_ptr() as *const libc::c_void,
        b"%!PS-AdobeFont\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        14i32 as libc::c_ulong,
    ) == 0
        || memcmp(
            sig.as_mut_ptr() as *const libc::c_void,
            b"%!FontType1\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            11i32 as libc::c_ulong,
        ) == 0
    {
        return 1i32 != 0;
    }
    if memcmp(
        sig.as_mut_ptr() as *const libc::c_void,
        b"%!PS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4i32 as libc::c_ulong,
    ) == 0
    {
        sig[14] = '\u{0}' as i32 as libc::c_char;
        dpx_warning(
            b"Ambiguous PostScript resource type: %s\x00" as *const u8 as *const libc::c_char,
            sig.as_mut_ptr(),
        );
        return 1i32 != 0;
    }
    dpx_warning(b"Not a PFB font file?\x00" as *const u8 as *const libc::c_char);
    return 0i32 != 0;
}
unsafe extern "C" fn get_pfb_segment(
    mut handle: rust_input_handle_t,
    mut expected_type: libc::c_int,
    mut length: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bytesread: libc::c_int = 0i32;
    loop {
        let mut ch: libc::c_int = 0;
        let mut slen: libc::c_int = 0;
        let mut rlen: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        ch = ttstub_input_getc(handle);
        if ch < 0i32 {
            break;
        }
        if ch != 128i32 {
            _tt_abort(b"Not a pfb file?\x00" as *const u8 as *const libc::c_char);
        }
        ch = ttstub_input_getc(handle);
        if ch < 0i32 || ch != expected_type {
            ttstub_input_seek(handle, -2i32 as ssize_t, 1i32);
            break;
        } else {
            slen = 0i32;
            i = 0i32;
            while i < 4i32 {
                ch = ttstub_input_getc(handle);
                if ch < 0i32 {
                    free(buffer as *mut libc::c_void);
                    return 0 as *mut libc::c_uchar;
                }
                slen = slen + (ch << 8i32 * i);
                i += 1
            }
            buffer = renew(
                buffer as *mut libc::c_void,
                ((bytesread + slen) as u32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                    as u32,
            ) as *mut libc::c_uchar;
            while slen > 0i32 {
                rlen = ttstub_input_read(
                    handle,
                    (buffer as *mut libc::c_char).offset(bytesread as isize),
                    slen as size_t,
                ) as libc::c_int;
                if rlen < 0i32 {
                    free(buffer as *mut libc::c_void);
                    return 0 as *mut libc::c_uchar;
                }
                slen -= rlen;
                bytesread += rlen
            }
        }
    }
    if bytesread == 0i32 {
        _tt_abort(b"PFB segment length zero?\x00" as *const u8 as *const libc::c_char);
    }
    buffer = renew(
        buffer as *mut libc::c_void,
        ((bytesread + 1i32) as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32,
    ) as *mut libc::c_uchar;
    *buffer.offset(bytesread as isize) = 0i32 as libc::c_uchar;
    if !length.is_null() {
        *length = bytesread
    }
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn t1_get_standard_glyph(mut code: libc::c_int) -> *const libc::c_char {
    if StandardEncoding[code as usize].is_null() {
        return 0 as *const libc::c_char;
    }
    return StandardEncoding[code as usize];
}
#[no_mangle]
pub unsafe extern "C" fn t1_get_fontname(
    mut handle: rust_input_handle_t,
    mut fontname: *mut libc::c_char,
) -> libc::c_int {
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut start: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: libc::c_int = 0;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fn_found: libc::c_int = 0i32;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    buffer = get_pfb_segment(handle, 1i32, &mut length);
    if buffer.is_null() || length == 0i32 {
        _tt_abort(b"Reading PFB (ASCII part) file failed.\x00" as *const u8 as *const libc::c_char);
    }
    start = buffer;
    end = buffer.offset(length as isize);
    if seek_operator(
        &mut start,
        end,
        b"begin\x00" as *const u8 as *const libc::c_char,
    ) < 0i32
    {
        free(buffer as *mut libc::c_void);
        return -1i32;
    }
    while fn_found == 0 && start < end && {
        key = get_next_key(&mut start, end);
        !key.is_null()
    } {
        if streq_ptr(key, b"FontName\x00" as *const u8 as *const libc::c_char) {
            let mut strval: *mut libc::c_char = 0 as *mut libc::c_char;
            if parse_svalue(&mut start, end, &mut strval) == 1i32 {
                if strlen(strval) > 127i32 as libc::c_ulong {
                    dpx_warning(
                        b"FontName \"%s\" too long. (%zu bytes)\x00" as *const u8
                            as *const libc::c_char,
                        strval,
                        strlen(strval),
                    );
                    *strval.offset(127) = '\u{0}' as i32 as libc::c_char
                }
                strcpy(fontname, strval);
                free(strval as *mut libc::c_void);
                fn_found = 1i32
            }
        }
        free(key as *mut libc::c_void);
    }
    free(buffer as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn init_cff_font(mut cff: *mut cff_font) {
    (*cff).handle = 0 as *mut libc::c_void;
    (*cff).filter = 0i32;
    (*cff).fontname = 0 as *mut libc::c_char;
    (*cff).index = 0i32;
    (*cff).flag = 1i32 << 1i32;
    (*cff).header.major = 1i32 as card8;
    (*cff).header.minor = 0i32 as card8;
    (*cff).header.hdr_size = 4i32 as card8;
    (*cff).header.offsize = 4i32 as c_offsize;
    (*cff).name = cff_new_index(1i32 as card16);
    (*cff).topdict = cff_new_dict();
    (*cff).string = 0 as *mut cff_index;
    (*cff).gsubr = cff_new_index(0i32 as card16);
    (*cff).encoding = 0 as *mut cff_encoding;
    (*cff).charsets = 0 as *mut cff_charsets;
    (*cff).fdselect = 0 as *mut cff_fdselect;
    (*cff).cstrings = 0 as *mut cff_index;
    (*cff).fdarray = 0 as *mut *mut cff_dict;
    (*cff).private = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as libc::c_ulong)
        as u32) as *mut *mut cff_dict;
    let ref mut fresh23 = *(*cff).private.offset(0);
    *fresh23 = cff_new_dict();
    (*cff).subrs = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut cff_index>() as libc::c_ulong)
        as u32) as *mut *mut cff_index;
    let ref mut fresh24 = *(*cff).subrs.offset(0);
    *fresh24 = 0 as *mut cff_index;
    (*cff).offset = 0i32 as l_offset;
    (*cff).gsubr_offset = 0i32 as l_offset;
    (*cff).num_glyphs = 0i32 as card16;
    (*cff).num_fds = 1i32 as card8;
    (*cff)._string = cff_new_index(0i32 as card16);
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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
pub unsafe extern "C" fn t1_load_font(
    mut enc_vec: *mut *mut libc::c_char,
    mut mode: libc::c_int,
    mut handle: rust_input_handle_t,
) -> *mut cff_font {
    let mut length: libc::c_int = 0;
    let mut cff: *mut cff_font = 0 as *mut cff_font;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut start: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    /* ASCII section */
    buffer = get_pfb_segment(handle, 1i32, &mut length);
    if buffer.is_null() || length == 0i32 {
        _tt_abort(b"Reading PFB (ASCII part) file failed.\x00" as *const u8 as *const libc::c_char);
    }
    cff = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<cff_font>() as libc::c_ulong) as u32)
        as *mut cff_font;
    init_cff_font(cff);
    start = buffer;
    end = buffer.offset(length as isize);
    if parse_part1(cff, enc_vec, &mut start, end) < 0i32 {
        cff_close(cff);
        free(buffer as *mut libc::c_void);
        _tt_abort(b"Reading PFB (ASCII part) file failed.\x00" as *const u8 as *const libc::c_char);
    }
    free(buffer as *mut libc::c_void);
    /* Binary section */
    buffer = get_pfb_segment(handle, 2i32, &mut length);
    if buffer.is_null() || length == 0i32 {
        cff_close(cff);
        free(buffer as *mut libc::c_void);
        _tt_abort(
            b"Reading PFB (BINARY part) file failed.\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        t1_decrypt(55665u32 as libc::c_ushort, buffer, buffer, 0i32, length);
    }
    start = buffer.offset(4);
    end = buffer.offset(length as isize);
    if parse_part2(cff, &mut start, end, mode) < 0i32 {
        cff_close(cff);
        free(buffer as *mut libc::c_void);
        _tt_abort(
            b"Reading PFB (BINARY part) file failed.\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Remaining section ignored. */
    free(buffer as *mut libc::c_void);
    cff_update_string(cff);
    return cff;
}
