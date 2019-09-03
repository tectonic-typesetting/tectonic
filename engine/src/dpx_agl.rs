#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           label_break_value,
           ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(buffer: *mut libc::c_char, length: libc::c_int,
                 file: rust_input_handle_t) -> *mut libc::c_char;
    /* tmp freed here */
    /* Tectonic-enabled I/O alternatives */
    #[no_mangle]
    fn dpx_tt_open(filename: *const libc::c_char, suffix: *const libc::c_char,
                   format: tt_input_format_type) -> rust_input_handle_t;
    #[no_mangle]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[no_mangle]
    fn ht_clear_table(ht: *mut ht_table);
    #[no_mangle]
    fn ht_lookup_table(ht: *mut ht_table, key: *const libc::c_void,
                       keylen: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_append_table(ht: *mut ht_table, key: *const libc::c_void,
                       keylen: libc::c_int, value: *mut libc::c_void);
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
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
    /* Please remove this */
    #[no_mangle]
    fn parse_ident(start: *mut *const libc::c_char, end: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
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
    fn UC_is_valid(ucv: int32_t) -> bool;
    #[no_mangle]
    fn UC_UTF16BE_encode_char(ucv: int32_t, dstpp: *mut *mut libc::c_uchar,
                              endptr: *mut libc::c_uchar) -> size_t;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct agl_name {
    pub name: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub n_components: libc::c_int,
    pub unicodes: [int32_t; 16],
    pub alternate: *mut agl_name,
    pub is_predef: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ht_table {
    pub count: libc::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut libc::c_char,
    pub keylen: libc::c_int,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub key: *const libc::c_char,
    pub otl_tag: *const libc::c_char,
    pub suffixes: [*const libc::c_char; 16],
}
/* quasi-hack to get the primary input */
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char,
                               mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32
    } /* Acutesmall, Gravesmall, etc */
    return 0i32 != 0;
}
#[inline]
unsafe extern "C" fn strstartswith(mut s: *const libc::c_char,
                                   mut prefix: *const libc::c_char)
 -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 { return s.offset(length as isize) }
    return 0 as *const libc::c_char;
}
static mut verbose: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn agl_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
unsafe extern "C" fn agl_new_name() -> *mut agl_name {
    let mut agln: *mut agl_name = 0 as *mut agl_name;
    agln =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<agl_name>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut agl_name;
    (*agln).name = 0 as *mut libc::c_char;
    (*agln).suffix = 0 as *mut libc::c_char;
    (*agln).n_components = 0i32;
    (*agln).alternate = 0 as *mut agl_name;
    (*agln).is_predef = 0i32;
    return agln;
}
unsafe extern "C" fn agl_release_name(mut agln: *mut agl_name) {
    let mut next: *mut agl_name = 0 as *mut agl_name;
    while !agln.is_null() {
        next = (*agln).alternate;
        free((*agln).name as *mut libc::c_void);
        free((*agln).suffix as *mut libc::c_void);
        (*agln).name = 0 as *mut libc::c_char;
        free(agln as *mut libc::c_void);
        agln = next
    };
}
#[no_mangle]
pub unsafe extern "C" fn agl_chop_suffix(mut glyphname: *const libc::c_char,
                                         mut suffix: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if !glyphname.is_null() && !suffix.is_null() {
    } else {
        __assert_fail(b"glyphname && suffix\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-agl.c\x00" as *const u8 as *const libc::c_char,
                      95i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"char *agl_chop_suffix(const char *, char **)\x00")).as_ptr());
    }
    p = strchr(glyphname, '.' as i32);
    if !p.is_null() {
        len = strlen(glyphname).wrapping_sub(strlen(p)) as libc::c_int;
        if len < 1i32 {
            name = 0 as *mut libc::c_char;
            *suffix =
                new((strlen(glyphname) as uint32_t as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                         as libc::c_ulong) as
                        uint32_t) as *mut libc::c_char;
            strcpy(*suffix, glyphname.offset(1));
        } else {
            p = p.offset(1);
            name =
                new(((len + 1i32) as uint32_t as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                         as libc::c_ulong) as
                        uint32_t) as *mut libc::c_char;
            strncpy(name, glyphname, len as libc::c_ulong);
            *name.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
            if *p.offset(0) as libc::c_int == '\u{0}' as i32 {
                *suffix = 0 as *mut libc::c_char
            } else {
                *suffix =
                    new((strlen(p).wrapping_add(1i32 as libc::c_ulong) as
                             uint32_t as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                             as libc::c_ulong)
                            as uint32_t) as *mut libc::c_char;
                strcpy(*suffix, p);
            }
        }
    } else {
        name =
            new((strlen(glyphname).wrapping_add(1i32 as libc::c_ulong) as
                     uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy(name, glyphname);
        *suffix = 0 as *mut libc::c_char
    }
    return name;
}
static mut modifiers: [*const libc::c_char; 21] =
    [b"acute\x00" as *const u8 as *const libc::c_char,
     b"breve\x00" as *const u8 as *const libc::c_char,
     b"caron\x00" as *const u8 as *const libc::c_char,
     b"cedilla\x00" as *const u8 as *const libc::c_char,
     b"circumflex\x00" as *const u8 as *const libc::c_char,
     b"dieresis\x00" as *const u8 as *const libc::c_char,
     b"dotaccent\x00" as *const u8 as *const libc::c_char,
     b"grave\x00" as *const u8 as *const libc::c_char,
     b"hungarumlaut\x00" as *const u8 as *const libc::c_char,
     b"macron\x00" as *const u8 as *const libc::c_char,
     b"ogonek\x00" as *const u8 as *const libc::c_char,
     b"ring\x00" as *const u8 as *const libc::c_char,
     b"tilde\x00" as *const u8 as *const libc::c_char,
     b"commaaccent\x00" as *const u8 as *const libc::c_char,
     b"slash\x00" as *const u8 as *const libc::c_char,
     b"ampersand\x00" as *const u8 as *const libc::c_char,
     b"exclam\x00" as *const u8 as *const libc::c_char,
     b"exclamdown\x00" as *const u8 as *const libc::c_char,
     b"question\x00" as *const u8 as *const libc::c_char,
     b"questiondown\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
unsafe extern "C" fn skip_capital(mut p: *mut *const libc::c_char,
                                  mut endptr: *const libc::c_char)
 -> libc::c_int {
    let mut slen: libc::c_int = 0i32;
    let mut len: libc::c_int = 0;
    len = endptr.wrapping_offset_from(*p) as libc::c_long as libc::c_int;
    if len >= 2i32 &&
           (**p as libc::c_int == 'A' as i32 &&
                *(*p).offset(1) as libc::c_int == 'E' as i32 ||
                **p as libc::c_int == 'O' as i32 &&
                    *(*p).offset(1) as libc::c_int == 'E' as i32) {
        *p = (*p).offset(2);
        slen = 2i32
    } else if len >= 3i32 && **p as libc::c_int == 'E' as i32 &&
                  *(*p).offset(1) as libc::c_int == 't' as i32 &&
                  *(*p).offset(2) as libc::c_int == 'h' as i32 {
        *p = (*p).offset(3);
        slen = 3i32
    } else if len >= 5i32 && **p as libc::c_int == 'T' as i32 &&
                  *(*p).offset(1) as libc::c_int == 'h' as i32 &&
                  *(*p).offset(2) as libc::c_int == 'o' as i32 &&
                  *(*p).offset(3) as libc::c_int == 'r' as i32 &&
                  *(*p).offset(4) as libc::c_int == 'n' as i32 {
        *p = (*p).offset(5);
        slen = 5i32
    } else if len >= 1i32 && **p as libc::c_int >= 'A' as i32 &&
                  **p as libc::c_int <= 'Z' as i32 {
        *p = (*p).offset(1);
        slen = 1i32
    }
    return slen;
}
unsafe extern "C" fn skip_modifier(mut p: *mut *const libc::c_char,
                                   mut endptr: *const libc::c_char)
 -> size_t {
    let mut slen: size_t = 0i32 as size_t;
    let mut len: size_t = 0;
    let mut i: libc::c_uint = 0;
    len = endptr.wrapping_offset_from(*p) as libc::c_long as size_t;
    i = 0i32 as libc::c_uint;
    while !modifiers[i as usize].is_null() {
        if len >= strlen(modifiers[i as usize]) &&
               memcmp(*p as *const libc::c_void,
                      modifiers[i as usize] as *const libc::c_void, len) == 0
           {
            slen = strlen(modifiers[i as usize]);
            *p = (*p).offset(slen as isize);
            break ;
        } else { i = i.wrapping_add(1) }
    }
    return slen;
}
unsafe extern "C" fn is_smallcap(mut glyphname: *const libc::c_char) -> bool {
    let mut len: size_t = 0;
    let mut slen: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    if glyphname.is_null() { return 0i32 != 0 }
    p = glyphname;
    len = strlen(glyphname);
    if len < 6i32 as libc::c_ulong ||
           strcmp(p.offset(len as isize).offset(-5),
                  b"small\x00" as *const u8 as *const libc::c_char) != 0 {
        return 0i32 != 0
    }
    endptr = p.offset(len as isize).offset(-5);
    len =
        (len as libc::c_ulong).wrapping_sub(5i32 as libc::c_ulong) as size_t
            as size_t;
    slen = skip_modifier(&mut p, endptr);
    if slen == len {
        return 1i32 != 0
    } else {
        if slen > 0i32 as libc::c_ulong {
            /* ??? */
            return 0i32 != 0
        }
    }
    len =
        (len as
             libc::c_ulong).wrapping_sub(skip_capital(&mut p, endptr) as
                                             libc::c_ulong) as size_t as
            size_t;
    if len == 0i32 as libc::c_ulong {
        return 1i32 != 0
        /* Asmall, AEsmall, etc */
    }
    while len > 0i32 as libc::c_ulong {
        /* allow multiple accent */
        slen = skip_modifier(&mut p, endptr);
        if slen == 0i32 as libc::c_ulong { return 0i32 != 0 }
        len = (len as libc::c_ulong).wrapping_sub(slen) as size_t as size_t
    }
    return 1i32 != 0;
}
static mut var_list: [C2RustUnnamed_0; 14] =
    [{
         let mut init =
             C2RustUnnamed_0{key:
                                 b"small\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"smcp\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [b"sc\x00" as *const u8 as
                                      *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"swash\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"swsh\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"superior\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"sups\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"inferior\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"sinf\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"numerator\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"numr\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"denominator\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"dnom\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"oldstyle\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag:
                                 b"onum\x00" as *const u8 as
                                     *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"display\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"text\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"big\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"bigg\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"Big\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key:
                                 b"Bigg\x00" as *const u8 as
                                     *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{key: 0 as *const libc::c_char,
                             otl_tag: 0 as *const libc::c_char,
                             suffixes:
                                 [0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char,
                                  0 as *const libc::c_char],};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn agl_suffix_to_otltag(mut suffix: *const libc::c_char)
 -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while !var_list[i as usize].key.is_null() {
        j = 0i32;
        while !var_list[i as usize].suffixes[j as usize].is_null() {
            if streq_ptr(suffix, var_list[i as usize].suffixes[j as usize]) {
                return var_list[i as usize].otl_tag
            }
            j += 1
        }
        if streq_ptr(suffix, var_list[i as usize].key) {
            return var_list[i as usize].otl_tag
        }
        if !var_list[i as usize].otl_tag.is_null() &&
               streq_ptr(suffix, var_list[i as usize].otl_tag) as libc::c_int
                   != 0 {
            return var_list[i as usize].otl_tag
        }
        i += 1
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn agl_guess_name(mut glyphname: *const libc::c_char)
 -> ssize_t {
    let mut i: ssize_t = 0;
    let mut len: size_t = 0;
    if is_smallcap(glyphname) { return 0i32 as ssize_t }
    len = strlen(glyphname);
    i = 1i32 as ssize_t;
    while !var_list[i as usize].key.is_null() {
        if len > strlen(var_list[i as usize].key) &&
               streq_ptr(glyphname.offset(len as
                                              isize).offset(-(strlen(var_list[i
                                                                                  as
                                                                                  usize].key)
                                                                  as isize)),
                         var_list[i as usize].key) as libc::c_int != 0 {
            return i
        }
        i += 1
    }
    return -1i32 as ssize_t;
}
unsafe extern "C" fn agl_normalized_name(mut glyphname: *mut libc::c_char)
 -> *mut agl_name {
    let mut agln: *mut agl_name = 0 as *mut agl_name;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if glyphname.is_null() { return 0 as *mut agl_name }
    agln = agl_new_name();
    suffix = strchr(glyphname, '.' as i32);
    if !suffix.is_null() {
        n = strlen(glyphname).wrapping_sub(strlen(suffix)) as libc::c_int;
        if *suffix.offset(1) as libc::c_int != '\u{0}' as i32 {
            (*agln).suffix =
                new((strlen(suffix) as uint32_t as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                         as libc::c_ulong) as
                        uint32_t) as *mut libc::c_char;
            strcpy((*agln).suffix, suffix.offset(1));
        }
        (*agln).name =
            new(((n + 1i32) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        memcpy((*agln).name as *mut libc::c_void,
               glyphname as *const libc::c_void, n as libc::c_ulong);
        *(*agln).name.offset(n as isize) = '\u{0}' as i32 as libc::c_char
    } else if is_smallcap(glyphname) {
        n =
            strlen(glyphname).wrapping_sub(5i32 as libc::c_ulong) as
                libc::c_int;
        (*agln).suffix =
            new((3i32 as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy((*agln).suffix, b"sc\x00" as *const u8 as *const libc::c_char);
        (*agln).name =
            new(((n + 1i32) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        i = 0i32;
        while i < n {
            *(*agln).name.offset(i as isize) =
                (if *(*__ctype_b_loc()).offset(*glyphname.offset(i as isize)
                                                   as libc::c_uchar as
                                                   libc::c_int as isize) as
                        libc::c_int &
                        _ISupper as libc::c_int as libc::c_ushort as
                            libc::c_int != 0 {
                     *glyphname.offset(i as isize) as libc::c_int + 32i32
                 } else { *glyphname.offset(i as isize) as libc::c_int }) as
                    libc::c_char;
            i += 1
        }
        *(*agln).name.offset(n as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        let mut var_idx: ssize_t = 0;
        var_idx = agl_guess_name(glyphname);
        if var_idx < 0i32 as libc::c_long ||
               var_list[var_idx as usize].key.is_null() {
            n = strlen(glyphname) as libc::c_int
        } else {
            n =
                strlen(glyphname).wrapping_sub(strlen(var_list[var_idx as
                                                                   usize].key))
                    as libc::c_int;
            if !var_list[var_idx as usize].suffixes[0].is_null() {
                (*agln).suffix =
                    new((strlen(var_list[var_idx as
                                             usize].suffixes[0]).wrapping_add(1i32
                                                                                  as
                                                                                  libc::c_ulong)
                             as uint32_t as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                             as libc::c_ulong)
                            as uint32_t) as *mut libc::c_char;
                strcpy((*agln).suffix,
                       var_list[var_idx as usize].suffixes[0]);
            } else {
                (*agln).suffix =
                    new((strlen(var_list[var_idx as
                                             usize].key).wrapping_add(1i32 as
                                                                          libc::c_ulong)
                             as uint32_t as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                             as libc::c_ulong)
                            as uint32_t) as *mut libc::c_char;
                strcpy((*agln).suffix, var_list[var_idx as usize].key);
            }
        }
        (*agln).name =
            new(((n + 1i32) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        memcpy((*agln).name as *mut libc::c_void,
               glyphname as *const libc::c_void, n as libc::c_ulong);
        *(*agln).name.offset(n as isize) = '\u{0}' as i32 as libc::c_char
    }
    return agln;
}
static mut aglmap: ht_table =
    ht_table{count: 0,
             hval_free_fn: None,
             table: [0 as *const ht_entry as *mut ht_entry; 503],};
#[inline]
unsafe extern "C" fn hval_free(mut hval: *mut libc::c_void) {
    agl_release_name(hval as *mut agl_name);
}
#[no_mangle]
pub unsafe extern "C" fn agl_init_map() {
    ht_init_table(&mut aglmap,
                  Some(hval_free as
                           unsafe extern "C" fn(_: *mut libc::c_void) -> ()));
    agl_load_listfile(b"texglyphlist.txt\x00" as *const u8 as
                          *const libc::c_char, 0i32);
    if agl_load_listfile(b"pdfglyphlist.txt\x00" as *const u8 as
                             *const libc::c_char, 1i32) < 0i32 {
        dpx_warning(b"Failed to load AGL file \"%s\"...\x00" as *const u8 as
                        *const libc::c_char,
                    b"pdfglyphlist.txt\x00" as *const u8 as
                        *const libc::c_char);
    }
    if agl_load_listfile(b"glyphlist.txt\x00" as *const u8 as
                             *const libc::c_char, 0i32) < 0i32 {
        dpx_warning(b"Failed to load AGL file \"%s\"...\x00" as *const u8 as
                        *const libc::c_char,
                    b"glyphlist.txt\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn agl_close_map() { ht_clear_table(&mut aglmap); }
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
/*
 * References:
 *
 *  Unicode and Glyph Names, ver. 2.3., Adobe Solution Network
 *  http://partners.adobe.com/asn/tech/type/unicodegn.jsp
 */
/* Hash */
unsafe extern "C" fn agl_load_listfile(mut filename: *const libc::c_char,
                                       mut is_predef: libc::c_int)
 -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut nextptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wbuf: [libc::c_char; 1024] = [0; 1024];
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if filename.is_null() { return -1i32 }
    handle =
        dpx_tt_open(filename, b".txt\x00" as *const u8 as *const libc::c_char,
                    TTIF_FONTMAP);
    if handle.is_null() { return -1i32 }
    if verbose != 0 {
        dpx_message(b"<AGL:%s\x00" as *const u8 as *const libc::c_char,
                    filename);
    }
    loop  {
        p = tt_mfgets(wbuf.as_mut_ptr(), 1024i32, handle);
        if p.is_null() { break ; }
        let mut agln: *mut agl_name = 0 as *mut agl_name;
        let mut duplicate: *mut agl_name = 0 as *mut agl_name;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n_unicodes: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut unicodes: [int32_t; 16] = [0; 16];
        endptr = p.offset(strlen(p) as isize);
        skip_white(&mut p, endptr);
        /* Need table version check. */
        if p.is_null() || *p.offset(0) as libc::c_int == '#' as i32 ||
               p >= endptr {
            continue ;
        }
        nextptr = strchr(p, ';' as i32);
        if nextptr.is_null() || nextptr == p as *mut libc::c_char {
            continue ;
        }
        name = parse_ident(&mut p, nextptr);
        skip_white(&mut p, endptr);
        if name.is_null() || *p.offset(0) as libc::c_int != ';' as i32 {
            dpx_warning(b"Invalid AGL entry: %s\x00" as *const u8 as
                            *const libc::c_char, wbuf.as_mut_ptr());
            free(name as *mut libc::c_void);
        } else {
            p = p.offset(1);
            skip_white(&mut p, endptr);
            n_unicodes = 0i32;
            while p < endptr &&
                      (*p.offset(0) as libc::c_int >= '0' as i32 &&
                           *p.offset(0) as libc::c_int <= '9' as i32 ||
                           *p.offset(0) as libc::c_int >= 'A' as i32 &&
                               *p.offset(0) as libc::c_int <= 'F' as i32) {
                if n_unicodes >= 16i32 {
                    dpx_warning(b"Too many Unicode values\x00" as *const u8 as
                                    *const libc::c_char);
                    break ;
                } else {
                    let fresh0 = n_unicodes;
                    n_unicodes = n_unicodes + 1;
                    unicodes[fresh0 as usize] =
                        strtol(p, &mut nextptr, 16i32) as int32_t;
                    p = nextptr;
                    skip_white(&mut p, endptr);
                }
            }
            if n_unicodes == 0i32 {
                dpx_warning(b"AGL entry ignored (no mapping): %s\x00" as
                                *const u8 as *const libc::c_char,
                            wbuf.as_mut_ptr());
                free(name as *mut libc::c_void);
            } else {
                agln = agl_normalized_name(name);
                (*agln).is_predef = is_predef;
                (*agln).n_components = n_unicodes;
                i = 0i32;
                while i < n_unicodes {
                    (*agln).unicodes[i as usize] = unicodes[i as usize];
                    i += 1
                }
                duplicate =
                    ht_lookup_table(&mut aglmap, name as *const libc::c_void,
                                    strlen(name) as libc::c_int) as
                        *mut agl_name;
                if duplicate.is_null() {
                    ht_append_table(&mut aglmap, name as *const libc::c_void,
                                    strlen(name) as libc::c_int,
                                    agln as *mut libc::c_void);
                } else {
                    while !(*duplicate).alternate.is_null() {
                        duplicate = (*duplicate).alternate
                    }
                    (*duplicate).alternate = agln
                }
                if verbose > 3i32 {
                    if !(*agln).suffix.is_null() {
                        dpx_message(b"agl: %s [%s.%s] -->\x00" as *const u8 as
                                        *const libc::c_char, name,
                                    (*agln).name, (*agln).suffix);
                    } else {
                        dpx_message(b"agl: %s [%s] -->\x00" as *const u8 as
                                        *const libc::c_char, name,
                                    (*agln).name);
                    }
                    i = 0i32;
                    while i < (*agln).n_components {
                        if (*agln).unicodes[i as usize] > 0xffffi32 {
                            dpx_message(b" U+%06X\x00" as *const u8 as
                                            *const libc::c_char,
                                        (*agln).unicodes[i as usize]);
                        } else {
                            dpx_message(b" U+%04X\x00" as *const u8 as
                                            *const libc::c_char,
                                        (*agln).unicodes[i as usize]);
                        }
                        i += 1
                    }
                    dpx_message(b"\n\x00" as *const u8 as
                                    *const libc::c_char);
                }
                free(name as *mut libc::c_void);
                count += 1
            }
        }
    }
    ttstub_input_close(handle);
    if verbose != 0 {
        dpx_message(b">\x00" as *const u8 as *const libc::c_char);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn agl_lookup_list(mut glyphname: *const libc::c_char)
 -> *mut agl_name {
    let mut agln: *mut agl_name = 0 as *mut agl_name;
    if glyphname.is_null() { return 0 as *mut agl_name }
    agln =
        ht_lookup_table(&mut aglmap, glyphname as *const libc::c_void,
                        strlen(glyphname) as libc::c_int) as *mut agl_name;
    return agln;
}
#[no_mangle]
pub unsafe extern "C" fn agl_name_is_unicode(mut glyphname:
                                                 *const libc::c_char)
 -> bool {
    let mut c: libc::c_char = 0;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    if glyphname.is_null() { return 0i32 != 0 }
    suffix = strchr(glyphname, '.' as i32);
    len =
        if !suffix.is_null() {
            suffix.wrapping_offset_from(glyphname) as libc::c_long as size_t
        } else { strlen(glyphname) };
    /*
   * uni02ac is invalid glyph name and mapped to th empty string.
   */
    if len >= 7i32 as libc::c_ulong &&
           len.wrapping_sub(3i32 as
                                libc::c_ulong).wrapping_rem(4i32 as
                                                                libc::c_ulong)
               == 0i32 as libc::c_ulong &&
           !strstartswith(glyphname,
                          b"uni\x00" as *const u8 as
                              *const libc::c_char).is_null() {
        c = *glyphname.offset(3);
        /*
     * Check if the 4th character is uppercase hexadecimal digit.
     * "union" should not be treated as Unicode glyph name.
     */
        if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
               ||
               c as libc::c_int >= 'A' as i32 &&
                   c as libc::c_int <= 'F' as i32 {
            return 1i32 != 0
        } else { return 0i32 != 0 }
    } else {
        if len <= 7i32 as libc::c_ulong && len >= 5i32 as libc::c_ulong &&
               *glyphname.offset(0) as libc::c_int == 'u' as i32 {
            i = 1i32 as size_t;
            while i < len.wrapping_sub(1i32 as libc::c_ulong) {
                c = *glyphname.offset(i as isize);
                if *(*__ctype_b_loc()).offset(c as libc::c_uchar as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 &&
                       ((c as libc::c_int) < 'A' as i32 ||
                            c as libc::c_int > 'F' as i32) {
                    return 0i32 != 0
                }
                i = i.wrapping_add(1)
            }
            return 1i32 != 0
        }
    }
    return 0i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn agl_name_convert_unicode(mut glyphname:
                                                      *const libc::c_char)
 -> int32_t {
    let mut ucv: int32_t = -1i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if !agl_name_is_unicode(glyphname) { return -1i32 }
    if strlen(glyphname) > 7i32 as libc::c_ulong &&
           *glyphname.offset(7) as libc::c_int != '.' as i32 {
        dpx_warning(b"Mapping to multiple Unicode characters not supported.\x00"
                        as *const u8 as *const libc::c_char);
        return -1i32
    }
    if *glyphname.offset(1) as libc::c_int == 'n' as i32 {
        p = glyphname.offset(3)
    } else { p = glyphname.offset(1) }
    ucv = 0i32;
    while *p as libc::c_int != '\u{0}' as i32 &&
              *p as libc::c_int != '.' as i32 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
               &&
               ((*p as libc::c_int) < 'A' as i32 ||
                    *p as libc::c_int > 'F' as i32) {
            dpx_warning(b"Invalid char %c in Unicode glyph name %s.\x00" as
                            *const u8 as *const libc::c_char,
                        *p as libc::c_int, glyphname);
            return -1i32
        }
        ucv <<= 4i32;
        ucv +=
            if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int
                                              as isize) as libc::c_int &
                   _ISdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                *p as libc::c_int - '0' as i32
            } else { *p as libc::c_int - 'A' as i32 + 10i32 };
        p = p.offset(1)
    }
    if !UC_is_valid(ucv) {
        if ucv < 0x10000i32 {
            dpx_warning(b"Invalid Unicode code value U+%04X.\x00" as *const u8
                            as *const libc::c_char, ucv);
        } else {
            dpx_warning(b"Invalid Unicode code value U+%06X.\x00" as *const u8
                            as *const libc::c_char, ucv);
        }
        ucv = -1i32
    }
    return ucv;
}
unsafe extern "C" fn xtol(mut start: *const libc::c_char,
                          mut len: libc::c_int) -> libc::c_int {
    let mut v: libc::c_int = 0i32;
    loop  {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 > 0i32) { break ; }
        v <<= 4i32;
        if *(*__ctype_b_loc()).offset(*start as libc::c_uchar as libc::c_int
                                          as isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            v += *start as libc::c_int - '0' as i32
        } else if *start as libc::c_int >= 'A' as i32 &&
                      *start as libc::c_int <= 'F' as i32 {
            v += *start as libc::c_int - 'A' as i32 + 10i32
        } else { return -1i32 }
        start = start.offset(1)
    }
    return v;
}
unsafe extern "C" fn put_unicode_glyph(mut name: *const libc::c_char,
                                       mut dstpp: *mut *mut libc::c_uchar,
                                       mut limptr: *mut libc::c_uchar)
 -> int32_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: int32_t = 0i32;
    let mut ucv: int32_t = 0;
    p = name;
    ucv = 0i32;
    if *p.offset(1) as libc::c_int != 'n' as i32 {
        p = p.offset(1);
        ucv = xtol(p, strlen(p) as libc::c_int);
        len =
            (len as
                 libc::c_ulong).wrapping_add(UC_UTF16BE_encode_char(ucv,
                                                                    dstpp,
                                                                    limptr))
                as int32_t as int32_t
    } else {
        p = p.offset(3);
        while *p as libc::c_int != '\u{0}' as i32 {
            ucv = xtol(p, 4i32);
            len =
                (len as
                     libc::c_ulong).wrapping_add(UC_UTF16BE_encode_char(ucv,
                                                                        dstpp,
                                                                        limptr))
                    as int32_t as int32_t;
            p = p.offset(4)
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn agl_sput_UTF16BE(mut glyphstr: *const libc::c_char,
                                          mut dstpp: *mut *mut libc::c_uchar,
                                          mut limptr: *mut libc::c_uchar,
                                          mut fail_count: *mut libc::c_int)
 -> int32_t {
    let mut len: int32_t = 0i32;
    let mut count: libc::c_int = 0i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    if !glyphstr.is_null() && !dstpp.is_null() {
    } else {
        __assert_fail(b"glyphstr && dstpp\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-agl.c\x00" as *const u8 as *const libc::c_char,
                      656i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 81],
                                                &[libc::c_char; 81]>(b"int32_t agl_sput_UTF16BE(const char *, unsigned char **, unsigned char *, int *)\x00")).as_ptr());
    }
    p = glyphstr;
    endptr = strchr(p, '.' as i32);
    if endptr.is_null() { endptr = p.offset(strlen(p) as isize) }
    while p < endptr {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut delim: *const libc::c_char = 0 as *const libc::c_char;
        let mut sub_len: int32_t = 0;
        let mut i: libc::c_int = 0;
        let mut agln0: *mut agl_name = 0 as *mut agl_name;
        let mut agln1: *mut agl_name = 0 as *mut agl_name;
        delim = strchr(p, '_' as i32);
        if delim == p {
            /*
       * Glyph names starting with a underscore or two subsequent
       * underscore in glyph name not allowed?
       */
            dpx_warning(b"Invalid glyph name component in \"%s\".\x00" as
                            *const u8 as *const libc::c_char, glyphstr);
            count += 1;
            if !fail_count.is_null() { *fail_count = count }
            return len
            /* Cannot continue */
        } else { if delim.is_null() || delim > endptr { delim = endptr } }
        sub_len = delim.wrapping_offset_from(p) as libc::c_long as int32_t;
        name =
            new(((sub_len + 1i32) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        memcpy(name as *mut libc::c_void, p as *const libc::c_void,
               sub_len as libc::c_ulong);
        *name.offset(sub_len as isize) = '\u{0}' as i32 as libc::c_char;
        if agl_name_is_unicode(name) {
            sub_len = put_unicode_glyph(name, dstpp, limptr);
            if sub_len > 0i32 { len += sub_len } else { count += 1 }
        } else {
            agln1 = agl_lookup_list(name);
            if agln1.is_null() ||
                   (*agln1).n_components == 1i32 &&
                       ((*agln1).unicodes[0] as libc::c_long >= 0xe000i64 &&
                            (*agln1).unicodes[0] as libc::c_long <= 0xf8ffi64
                            ||
                            (*agln1).unicodes[0] as libc::c_long >= 0xf0000i64
                                &&
                                (*agln1).unicodes[0] as libc::c_long <=
                                    0xffffdi64 ||
                            (*agln1).unicodes[0] as libc::c_long >=
                                0x100000i64 &&
                                (*agln1).unicodes[0] as libc::c_long <=
                                    0x10fffdi64) {
                agln0 = agl_normalized_name(name);
                if !agln0.is_null() {
                    if verbose > 1i32 && !(*agln0).suffix.is_null() {
                        dpx_warning(b"agl: fix %s --> %s.%s\x00" as *const u8
                                        as *const libc::c_char, name,
                                    (*agln0).name, (*agln0).suffix);
                    }
                    agln1 = agl_lookup_list((*agln0).name);
                    agl_release_name(agln0);
                }
            }
            if !agln1.is_null() {
                i = 0i32;
                while i < (*agln1).n_components {
                    len =
                        (len as
                             libc::c_ulong).wrapping_add(UC_UTF16BE_encode_char((*agln1).unicodes[i
                                                                                                      as
                                                                                                      usize],
                                                                                dstpp,
                                                                                limptr))
                            as int32_t as int32_t;
                    i += 1
                }
            } else {
                if verbose != 0 {
                    dpx_warning(b"No Unicode mapping for glyph name \"%s\" found.\x00"
                                    as *const u8 as *const libc::c_char,
                                name);
                }
                count += 1
            }
        }
        free(name as *mut libc::c_void);
        p = delim.offset(1)
    }
    if !fail_count.is_null() { *fail_count = count }
    return len;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
pub unsafe extern "C" fn agl_get_unicodes(mut glyphstr: *const libc::c_char,
                                          mut unicodes: *mut int32_t,
                                          mut max_unicodes: libc::c_int)
 -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    p = glyphstr;
    endptr = strchr(p, '.' as i32);
    if endptr.is_null() { endptr = p.offset(strlen(p) as isize) }
    while p < endptr {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut delim: *const libc::c_char = 0 as *const libc::c_char;
        let mut sub_len: int32_t = 0;
        let mut i: libc::c_int = 0;
        let mut agln0: *mut agl_name = 0 as *mut agl_name;
        let mut agln1: *mut agl_name = 0 as *mut agl_name;
        delim = strchr(p, '_' as i32);
        if delim == p {
            /*
       * Glyph names starting with a underscore or two subsequent
       * underscore in glyph name not allowed?
       */
            dpx_warning(b"Invalid glyph name component in \"%s\".\x00" as
                            *const u8 as *const libc::c_char, glyphstr);
            return -1i32
            /* Cannot continue */
        } else { if delim.is_null() || delim > endptr { delim = endptr } }
        sub_len = delim.wrapping_offset_from(p) as libc::c_long as int32_t;
        name =
            new(((sub_len + 1i32) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        memcpy(name as *mut libc::c_void, p as *const libc::c_void,
               sub_len as libc::c_ulong);
        *name.offset(sub_len as isize) = '\u{0}' as i32 as libc::c_char;
        if agl_name_is_unicode(name) {
            p = name;
            if *p.offset(1) as libc::c_int != 'n' as i32 {
                /* uXXXXXXXX */
                if count >= max_unicodes {
                    free(name as *mut libc::c_void);
                    return -1i32
                }
                p = p.offset(1);
                let fresh2 = count;
                count = count + 1;
                *unicodes.offset(fresh2 as isize) =
                    xtol(p, strlen(p) as libc::c_int)
            } else {
                p = p.offset(3);
                while *p as libc::c_int != '\u{0}' as i32 {
                    if count >= max_unicodes {
                        free(name as *mut libc::c_void);
                        return -1i32
                    }
                    let fresh3 = count;
                    count = count + 1;
                    *unicodes.offset(fresh3 as isize) = xtol(p, 4i32);
                    p = p.offset(4)
                }
            }
        } else {
            agln1 = agl_lookup_list(name);
            if agln1.is_null() ||
                   (*agln1).n_components == 1i32 &&
                       ((*agln1).unicodes[0] as libc::c_long >= 0xe000i64 &&
                            (*agln1).unicodes[0] as libc::c_long <= 0xf8ffi64
                            ||
                            (*agln1).unicodes[0] as libc::c_long >= 0xf0000i64
                                &&
                                (*agln1).unicodes[0] as libc::c_long <=
                                    0xffffdi64 ||
                            (*agln1).unicodes[0] as libc::c_long >=
                                0x100000i64 &&
                                (*agln1).unicodes[0] as libc::c_long <=
                                    0x10fffdi64) {
                agln0 = agl_normalized_name(name);
                if !agln0.is_null() {
                    if verbose > 1i32 && !(*agln0).suffix.is_null() {
                        dpx_warning(b"agl: fix %s --> %s.%s\x00" as *const u8
                                        as *const libc::c_char, name,
                                    (*agln0).name, (*agln0).suffix);
                    }
                    agln1 = agl_lookup_list((*agln0).name);
                    agl_release_name(agln0);
                }
            }
            if !agln1.is_null() {
                if count + (*agln1).n_components > max_unicodes {
                    free(name as *mut libc::c_void);
                    return -1i32
                }
                i = 0i32;
                while i < (*agln1).n_components {
                    let fresh4 = count;
                    count = count + 1;
                    *unicodes.offset(fresh4 as isize) =
                        (*agln1).unicodes[i as usize];
                    i += 1
                }
            } else {
                if verbose > 1i32 {
                    dpx_warning(b"No Unicode mapping for glyph name \"%s\" found.\x00"
                                    as *const u8 as *const libc::c_char,
                                name);
                }
                free(name as *mut libc::c_void);
                return -1i32
            }
        }
        free(name as *mut libc::c_void);
        p = delim.offset(1)
    }
    return count;
}
