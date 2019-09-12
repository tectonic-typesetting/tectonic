#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::strstartswith;
use crate::{info, warn};

use crate::ttstub_input_close;
use libc::free;
extern "C" {
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn atoi(__nptr: *const i8) -> i32;
    #[no_mangle]
    fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> i64;
    #[no_mangle]
    fn strtoul(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
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
    static mut work_buffer: [i8; 0];
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(buffer: *mut i8, length: i32, file: rust_input_handle_t) -> *mut i8;
    /* Tectonic-enabled I/O alternatives */
    #[no_mangle]
    fn dpx_tt_open(
        filename: *const i8,
        suffix: *const i8,
        format: TTInputFormat,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[no_mangle]
    fn ht_clear_table(ht: *mut ht_table);
    #[no_mangle]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_remove_table(ht: *mut ht_table, key: *const libc::c_void, keylen: i32) -> i32;
    #[no_mangle]
    fn ht_insert_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
        value: *mut libc::c_void,
    );
    #[no_mangle]
    fn parse_float_decimal(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_string(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
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
    fn dpx_message(fmt: *const i8, _: ...);
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
    fn release_sfd_record();
    #[no_mangle]
    fn sfd_get_subfont_ids(sfd_name: *const i8, num_subfonts: *mut i32) -> *mut *mut i8;
}
pub type size_t = u64;

use crate::TTInputFormat;

pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: f64,
    pub extend: f64,
    pub bold: f64,
    pub mapc: i32,
    pub flags: i32,
    pub otl_tags: *mut i8,
    pub tounicode: *mut i8,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: f64,
    pub charcoll: *mut i8,
    pub index: i32,
    pub style: i32,
    pub stemv: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut i8,
    pub font_name: *mut i8,
    pub enc_name: *mut i8,
    pub charmap: C2RustUnnamed_0,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sfd_name: *mut i8,
    pub subfont_id: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: i32,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut i8,
    pub keylen: i32,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    0 as *mut libc::c_void
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
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    false
}
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_fontmap_set_verbose(mut level: i32) {
    verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_fontmap_record(mut mrec: *mut fontmap_rec) {
    assert!(!mrec.is_null());
    (*mrec).map_name = 0 as *mut i8;
    /* SFD char mapping */
    (*mrec).charmap.sfd_name = 0 as *mut i8;
    (*mrec).charmap.subfont_id = 0 as *mut i8;
    /* for OFM */
    (*mrec).opt.mapc = -1i32; /* compatibility */
    (*mrec).font_name = 0 as *mut i8; /* not given explicitly by an option */
    (*mrec).enc_name = 0 as *mut i8;
    (*mrec).opt.slant = 0.0f64;
    (*mrec).opt.extend = 1.0f64;
    (*mrec).opt.bold = 0.0f64;
    (*mrec).opt.flags = 0i32;
    (*mrec).opt.design_size = -1.0f64;
    (*mrec).opt.tounicode = 0 as *mut i8;
    (*mrec).opt.otl_tags = 0 as *mut i8;
    (*mrec).opt.index = 0i32;
    (*mrec).opt.charcoll = 0 as *mut i8;
    (*mrec).opt.style = 0i32;
    (*mrec).opt.stemv = -1i32;
    (*mrec).opt.cff_charsets = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_clear_fontmap_record(mut mrec: *mut fontmap_rec) {
    assert!(!mrec.is_null());
    free((*mrec).map_name as *mut libc::c_void);
    free((*mrec).charmap.sfd_name as *mut libc::c_void);
    free((*mrec).charmap.subfont_id as *mut libc::c_void);
    free((*mrec).enc_name as *mut libc::c_void);
    free((*mrec).font_name as *mut libc::c_void);
    free((*mrec).opt.tounicode as *mut libc::c_void);
    free((*mrec).opt.otl_tags as *mut libc::c_void);
    free((*mrec).opt.charcoll as *mut libc::c_void);
    pdf_init_fontmap_record(mrec);
}
/* strdup: just returns NULL for NULL */
unsafe extern "C" fn mstrdup(mut s: *const i8) -> *mut i8 {
    let mut r: *mut i8 = 0 as *mut i8;
    if s.is_null() {
        return 0 as *mut i8;
    }
    r = new((strlen(s).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(r, s);
    r
}
unsafe extern "C" fn pdf_copy_fontmap_record(
    mut dst: *mut fontmap_rec,
    mut src: *const fontmap_rec,
) {
    assert!(!dst.is_null() && !src.is_null());
    (*dst).map_name = mstrdup((*src).map_name);
    (*dst).charmap.sfd_name = mstrdup((*src).charmap.sfd_name);
    (*dst).charmap.subfont_id = mstrdup((*src).charmap.subfont_id);
    (*dst).font_name = mstrdup((*src).font_name);
    (*dst).enc_name = mstrdup((*src).enc_name);
    (*dst).opt.slant = (*src).opt.slant;
    (*dst).opt.extend = (*src).opt.extend;
    (*dst).opt.bold = (*src).opt.bold;
    (*dst).opt.flags = (*src).opt.flags;
    (*dst).opt.mapc = (*src).opt.mapc;
    (*dst).opt.tounicode = mstrdup((*src).opt.tounicode);
    (*dst).opt.otl_tags = mstrdup((*src).opt.otl_tags);
    (*dst).opt.index = (*src).opt.index;
    (*dst).opt.charcoll = mstrdup((*src).opt.charcoll);
    (*dst).opt.style = (*src).opt.style;
    (*dst).opt.stemv = (*src).opt.stemv;
    (*dst).opt.cff_charsets = (*src).opt.cff_charsets;
}
unsafe extern "C" fn hval_free(mut vp: *mut libc::c_void) {
    let mut mrec: *mut fontmap_rec = vp as *mut fontmap_rec;
    pdf_clear_fontmap_record(mrec);
    free(mrec as *mut libc::c_void);
}
unsafe extern "C" fn fill_in_defaults(mut mrec: *mut fontmap_rec, mut tex_name: *const i8) {
    if !(*mrec).enc_name.is_null()
        && (streq_ptr((*mrec).enc_name, b"default\x00" as *const u8 as *const i8) as i32 != 0
            || streq_ptr((*mrec).enc_name, b"none\x00" as *const u8 as *const i8) as i32 != 0)
    {
        (*mrec).enc_name = mfree((*mrec).enc_name as *mut libc::c_void) as *mut i8
    }
    if !(*mrec).font_name.is_null()
        && (streq_ptr((*mrec).font_name, b"default\x00" as *const u8 as *const i8) as i32 != 0
            || streq_ptr((*mrec).font_name, b"none\x00" as *const u8 as *const i8) as i32 != 0)
    {
        (*mrec).font_name = mfree((*mrec).font_name as *mut libc::c_void) as *mut i8
    }
    /* We *must* fill font_name either explicitly or by default */
    if (*mrec).font_name.is_null() {
        (*mrec).font_name = new((strlen(tex_name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy((*mrec).font_name, tex_name);
    }
    (*mrec).map_name = new((strlen(tex_name).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*mrec).map_name, tex_name);
    /* Use "UCS" character collection for Unicode SFD
     * and Identity CMap combination. For backward
     * compatibility.
     */
    if !(*mrec).charmap.sfd_name.is_null()
        && !(*mrec).enc_name.is_null()
        && (*mrec).opt.charcoll.is_null()
    {
        if (streq_ptr(
            (*mrec).enc_name,
            b"Identity-H\x00" as *const u8 as *const i8,
        ) as i32
            != 0
            || streq_ptr(
                (*mrec).enc_name,
                b"Identity-V\x00" as *const u8 as *const i8,
            ) as i32
                != 0)
            && (!strstr(
                (*mrec).charmap.sfd_name,
                b"Uni\x00" as *const u8 as *const i8,
            )
            .is_null()
                || !strstr(
                    (*mrec).charmap.sfd_name,
                    b"UBig\x00" as *const u8 as *const i8,
                )
                .is_null()
                || !strstr(
                    (*mrec).charmap.sfd_name,
                    b"UBg\x00" as *const u8 as *const i8,
                )
                .is_null()
                || !strstr(
                    (*mrec).charmap.sfd_name,
                    b"UGB\x00" as *const u8 as *const i8,
                )
                .is_null()
                || !strstr(
                    (*mrec).charmap.sfd_name,
                    b"UKS\x00" as *const u8 as *const i8,
                )
                .is_null()
                || !strstr(
                    (*mrec).charmap.sfd_name,
                    b"UJIS\x00" as *const u8 as *const i8,
                )
                .is_null())
        {
            (*mrec).opt.charcoll = new((strlen(b"UCS\x00" as *const u8 as *const i8)
                .wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8; /* we don't have quoted string */
            strcpy((*mrec).opt.charcoll, b"UCS\x00" as *const u8 as *const i8);
        }
    };
}
unsafe extern "C" fn tt_readline(
    mut buf: *mut i8,
    mut buf_len: i32,
    mut handle: rust_input_handle_t,
) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    assert!(!buf.is_null() && buf_len > 0i32 && !handle.is_null());
    p = tt_mfgets(buf, buf_len, handle);
    if p.is_null() {
        return 0 as *mut i8;
    }
    q = strchr(p, '%' as i32);
    if !q.is_null() {
        *q = '\u{0}' as i32 as i8
    }
    p
}
unsafe extern "C" fn skip_blank(mut pp: *mut *const i8, mut endptr: *const i8) {
    let mut p: *const i8 = *pp;
    if p.is_null() || p >= endptr {
        return;
    }
    while p < endptr && (*p as i32 & !0x7fi32 == 0i32 && crate::isblank(*p as _) != 0) {
        p = p.offset(1)
    }
    *pp = p;
}
unsafe extern "C" fn parse_string_value(mut pp: *mut *const i8, mut endptr: *const i8) -> *mut i8 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = *pp;
    let mut n: u32 = 0;
    if p.is_null() || p >= endptr {
        return 0 as *mut i8;
    }
    if *p as i32 == '\"' as i32 {
        q = parse_c_string(&mut p, endptr)
    } else {
        n = 0_u32;
        while p < endptr && libc::isspace(*p as _) == 0 {
            p = p.offset(1);
            n = n.wrapping_add(1)
        }
        if n == 0_u32 {
            return 0 as *mut i8;
        }
        q = new(
            (n.wrapping_add(1_u32) as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        memcpy(q as *mut libc::c_void, *pp as *const libc::c_void, n as u64);
        *q.offset(n as isize) = '\u{0}' as i32 as i8
    }
    *pp = p;
    q
}
/* no preceeding spaces allowed */
unsafe extern "C" fn parse_integer_value(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut base: i32,
) -> *mut i8 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = *pp;
    let mut has_sign: i32 = 0i32;
    let mut has_prefix: i32 = 0i32;
    let mut n: i32 = 0;
    assert!(base == 0i32 || base >= 2i32 && base <= 36i32);
    if p.is_null() || p >= endptr {
        return 0 as *mut i8;
    }
    if *p as i32 == '-' as i32 || *p as i32 == '+' as i32 {
        p = p.offset(1);
        has_sign = 1i32
    }
    if (base == 0i32 || base == 16i32)
        && p.offset(2) <= endptr
        && *p.offset(0) as i32 == '0' as i32
        && *p.offset(1) as i32 == 'x' as i32
    {
        p = p.offset(2);
        has_prefix = 1i32
    }
    if base == 0i32 {
        if has_prefix != 0 {
            base = 16i32
        } else if p < endptr && *p as i32 == '0' as i32 {
            base = 8i32
        } else {
            base = 10i32
        }
    }
    n = 0i32;
    while p < endptr
        && (base <= 10i32 && *p as i32 >= '0' as i32 && (*p as i32) < '0' as i32 + base
            || base > 10i32
                && (*p as i32 >= '0' as i32 && *p as i32 <= '9' as i32
                    || *p as i32 >= 'a' as i32 && (*p as i32) < 'a' as i32 + (base - 10i32)
                    || *p as i32 >= 'A' as i32 && (*p as i32) < 'A' as i32 + (base - 10i32)))
    {
        p = p.offset(1);
        n += 1
    }
    if n == 0i32 {
        return 0 as *mut i8;
    }
    if has_sign != 0 {
        n += 1i32
    }
    if has_prefix != 0 {
        n += 2i32
    }
    q = new(((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    memcpy(q as *mut libc::c_void, *pp as *const libc::c_void, n as u64);
    *q.offset(n as isize) = '\u{0}' as i32 as i8;
    *pp = p;
    q
}
unsafe extern "C" fn fontmap_parse_mapdef_dpm(
    mut mrec: *mut fontmap_rec,
    mut mapdef: *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut p: *const i8 = mapdef;
    /*
     * Parse record line in map file.  First two fields (after TeX font
     * name) are position specific.  Arguments start at the first token
     * beginning with a  '-'.
     *
     * NOTE:
     *   Dvipdfm basically uses parse_ident() for parsing enc_name,
     *   font_name, and other string values which assumes PostScript-like
     *   syntax.
     *   skip_white() skips '\r' and '\n' but they should terminate
     *   fontmap line here.
     */
    skip_blank(&mut p, endptr);
    /* encoding field */
    if p < endptr && *p as i32 != '-' as i32 {
        /* May be NULL */
        (*mrec).enc_name = parse_string_value(&mut p, endptr);
        skip_blank(&mut p, endptr);
    }
    /* fontname or font filename field */
    if p < endptr && *p as i32 != '-' as i32 {
        /* May be NULL */
        (*mrec).font_name = parse_string_value(&mut p, endptr);
        skip_blank(&mut p, endptr);
    }
    if !(*mrec).font_name.is_null() {
        let mut tmp: *mut i8 = 0 as *mut i8;
        /* Several options are encoded in font_name for
         * compatibility with dvipdfm.
         */
        tmp = strip_options((*mrec).font_name, &mut (*mrec).opt);
        if !tmp.is_null() {
            free((*mrec).font_name as *mut libc::c_void);
            (*mrec).font_name = tmp
        }
    }
    skip_blank(&mut p, endptr);
    /* Parse any remaining arguments */
    while p.offset(1) < endptr
        && *p as i32 != '\r' as i32
        && *p as i32 != '\n' as i32
        && *p as i32 == '-' as i32
    {
        let mut q: *mut i8 = 0 as *mut i8;
        let mut mopt: i8 = *p.offset(1);
        let mut v: i32 = 0;
        p = p.offset(2);
        skip_blank(&mut p, endptr);
        match mopt as i32 {
            115 => {
                /* Slant option */
                q = parse_float_decimal(&mut p, endptr);
                if q.is_null() {
                    warn!("Missing a number value for \'s\' option.");
                    return -1i32;
                }
                (*mrec).opt.slant = atof(q);
                free(q as *mut libc::c_void);
            }
            101 => {
                /* Extend option */
                q = parse_float_decimal(&mut p, endptr);
                if q.is_null() {
                    warn!("Missing a number value for \'e\' option.");
                    return -1i32;
                }
                (*mrec).opt.extend = atof(q);
                if (*mrec).opt.extend <= 0.0f64 {
                    dpx_warning(
                        b"Invalid value for \'e\' option: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    return -1i32;
                }
                free(q as *mut libc::c_void);
            }
            98 => {
                /* Fake-bold option */
                q = parse_float_decimal(&mut p, endptr);
                if q.is_null() {
                    warn!("Missing a number value for \'b\' option.");
                    return -1i32;
                }
                (*mrec).opt.bold = atof(q);
                if (*mrec).opt.bold <= 0.0f64 {
                    dpx_warning(
                        b"Invalid value for \'b\' option: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    return -1i32;
                }
                free(q as *mut libc::c_void);
            }
            114 => {}
            105 => {
                /* TTC index */
                q = parse_integer_value(&mut p, endptr, 10i32);
                if q.is_null() {
                    warn!("Missing TTC index number...");
                    return -1i32;
                }
                (*mrec).opt.index = atoi(q);
                if (*mrec).opt.index < 0i32 {
                    dpx_warning(
                        b"Invalid TTC index number: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    return -1i32;
                }
                free(q as *mut libc::c_void);
            }
            112 => {
                /* UCS plane: just for testing */
                q = parse_integer_value(&mut p, endptr, 0i32);
                if q.is_null() {
                    warn!("Missing a number for \'p\' option.");
                    return -1i32;
                }
                v = strtol(q, 0 as *mut *mut i8, 0i32) as i32;
                if v < 0i32 || v > 16i32 {
                    dpx_warning(
                        b"Invalid value for option \'p\': %s\x00" as *const u8 as *const i8,
                        q,
                    );
                } else {
                    (*mrec).opt.mapc = v << 16i32
                }
                free(q as *mut libc::c_void);
            }
            117 => {
                /* ToUnicode */
                q = parse_string_value(&mut p, endptr);
                if !q.is_null() {
                    (*mrec).opt.tounicode = q
                } else {
                    warn!("Missing string value for option \'u\'.");
                    return -1i32;
                }
            }
            118 => {
                /* StemV */
                q = parse_integer_value(&mut p, endptr, 10i32);
                if q.is_null() {
                    warn!("Missing a number for \'v\' option.");
                    return -1i32;
                }
                (*mrec).opt.stemv = strtol(q, 0 as *mut *mut i8, 0i32) as i32;
                free(q as *mut libc::c_void);
            }
            108 => {
                /* 2017.4.15 back again */
                q = parse_string_value(&mut p, endptr);
                if !q.is_null() {
                    (*mrec).opt.otl_tags = q
                } else {
                    warn!("Missing string value for option \'l\'.");
                    return -1i32;
                }
            }
            109 => {
                /* Omega uses both single-byte and double-byte set_char command
                 * even for double-byte OFMs. This confuses CMap decoder.
                 */
                /* Map single bytes char 0xab to double byte char 0xcdab  */
                if p.offset(4) <= endptr
                    && *p.offset(0) as i32 == '<' as i32
                    && *p.offset(3) as i32 == '>' as i32
                {
                    p = p.offset(1);
                    q = parse_integer_value(&mut p, endptr, 16i32);
                    if q.is_null() {
                        warn!("Invalid value for option \'m\'.");
                        return -1i32;
                    } else {
                        if p < endptr && *p as i32 != '>' as i32 {
                            dpx_warning(
                                b"Invalid value for option \'m\': %s\x00" as *const u8 as *const i8,
                                q,
                            );
                            free(q as *mut libc::c_void);
                            return -1i32;
                        }
                    }
                    v = strtol(q, 0 as *mut *mut i8, 16i32) as i32;
                    (*mrec).opt.mapc = ((v << 8i32) as i64 & 0xff00) as i32;
                    free(q as *mut libc::c_void);
                    p = p.offset(1)
                } else if p.offset(4) <= endptr
                    && memcmp(
                        p as *const libc::c_void,
                        b"sfd:\x00" as *const u8 as *const i8 as *const libc::c_void,
                        strlen(b"sfd:\x00" as *const u8 as *const i8),
                    ) == 0
                {
                    let mut r: *mut i8 = 0 as *mut i8;
                    let mut rr: *const i8 = 0 as *const i8;
                    /* SFD mapping: sfd:Big5,00 */
                    p = p.offset(4);
                    skip_blank(&mut p, endptr);
                    q = parse_string_value(&mut p, endptr);
                    if q.is_null() {
                        warn!("Missing value for option \'m\'.");
                        return -1i32;
                    }
                    r = strchr(q, ',' as i32);
                    if r.is_null() {
                        dpx_warning(
                            b"Invalid value for option \'m\': %s\x00" as *const u8 as *const i8,
                            q,
                        );
                        free(q as *mut libc::c_void);
                        return -1i32;
                    }
                    *r = 0_i8;
                    r = r.offset(1);
                    rr = r;
                    skip_blank(&mut rr, r.offset(strlen(r) as isize));
                    if *rr as i32 == '\u{0}' as i32 {
                        dpx_warning(
                            b"Invalid value for option \'m\': %s,\x00" as *const u8 as *const i8,
                            q,
                        );
                        free(q as *mut libc::c_void);
                        return -1i32;
                    }
                    (*mrec).charmap.sfd_name = mstrdup(q);
                    (*mrec).charmap.subfont_id = mstrdup(rr);
                    free(q as *mut libc::c_void);
                } else if p.offset(4) < endptr
                    && memcmp(
                        p as *const libc::c_void,
                        b"pad:\x00" as *const u8 as *const i8 as *const libc::c_void,
                        strlen(b"pad:\x00" as *const u8 as *const i8),
                    ) == 0
                {
                    p = p.offset(4);
                    skip_blank(&mut p, endptr);
                    q = parse_integer_value(&mut p, endptr, 16i32);
                    if q.is_null() {
                        warn!("Invalid value for option \'m\'.");
                        return -1i32;
                    } else {
                        if p < endptr && libc::isspace(*p as _) == 0 {
                            dpx_warning(
                                b"Invalid value for option \'m\': %s\x00" as *const u8 as *const i8,
                                q,
                            );
                            free(q as *mut libc::c_void);
                            return -1i32;
                        }
                    }
                    v = strtol(q, 0 as *mut *mut i8, 16i32) as i32;
                    (*mrec).opt.mapc = ((v << 8i32) as i64 & 0xff00) as i32;
                    free(q as *mut libc::c_void);
                } else {
                    warn!("Invalid value for option \'m\'.");
                    return -1i32;
                }
            }
            119 => {
                /* Writing mode (for unicode encoding) */
                if (*mrec).enc_name.is_null()
                    || strcmp((*mrec).enc_name, b"unicode\x00" as *const u8 as *const i8) != 0
                {
                    warn!("Fontmap option \'w\' meaningless for encoding other than \"unicode\".");
                    return -1i32;
                }
                q = parse_integer_value(&mut p, endptr, 10i32);
                if q.is_null() {
                    warn!("Missing wmode value...");
                    return -1i32;
                }
                if atoi(q) == 1i32 {
                    (*mrec).opt.flags |= 1i32 << 2i32
                } else if atoi(q) == 0i32 {
                    (*mrec).opt.flags &= !(1i32 << 2i32)
                } else {
                    dpx_warning(
                        b"Invalid value for option \'w\': %s\x00" as *const u8 as *const i8,
                        q,
                    );
                }
                free(q as *mut libc::c_void);
            }
            _ => {
                dpx_warning(
                    b"Unrecognized font map option: \'%c\'\x00" as *const u8 as *const i8,
                    mopt as i32,
                );
                return -1i32;
            }
        }
        skip_blank(&mut p, endptr);
    }
    if p < endptr && *p as i32 != '\r' as i32 && *p as i32 != '\n' as i32 {
        dpx_warning(
            b"Invalid char in fontmap line: %c\x00" as *const u8 as *const i8,
            *p as i32,
        );
        return -1i32;
    }
    0i32
}
/* Parse record line in map file of DVIPS/pdfTeX format. */
unsafe extern "C" fn fontmap_parse_mapdef_dps(
    mut mrec: *mut fontmap_rec,
    mut mapdef: *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut p: *const i8 = mapdef;
    let mut q: *mut i8 = 0 as *mut i8;
    skip_blank(&mut p, endptr);
    /* The first field (after TFM name) must be PostScript name. */
    /* However, pdftex.map allows a line without PostScript name. */
    if *p as i32 != '\"' as i32 && *p as i32 != '<' as i32 {
        if p < endptr {
            q = parse_string_value(&mut p, endptr);
            free(q as *mut libc::c_void);
            skip_blank(&mut p, endptr);
        } else {
            warn!("Missing a PostScript font name.");
            return -1i32;
        }
    }
    if p >= endptr {
        return 0i32;
    }
    /* Parse any remaining arguments */
    while p < endptr
        && *p as i32 != '\r' as i32
        && *p as i32 != '\n' as i32
        && (*p as i32 == '<' as i32 || *p as i32 == '\"' as i32)
    {
        match *p as i32 {
            60 => {
                p = p.offset(1); /*skip */
                if p < endptr && (*p as i32 == '[' as i32 || *p as i32 == '<' as i32) {
                    p = p.offset(1)
                }
                skip_blank(&mut p, endptr);
                q = parse_string_value(&mut p, endptr);
                if !q.is_null() {
                    let mut n: i32 = strlen(q) as i32;
                    if n > 4i32
                        && !strstartswith(
                            q.offset(n as isize).offset(-4),
                            b".enc\x00" as *const u8 as *const i8,
                        )
                        .is_null()
                    {
                        (*mrec).enc_name = q
                    } else {
                        (*mrec).font_name = q
                    }
                }
                skip_blank(&mut p, endptr);
            }
            34 => {
                /* encoding or fontfile field */
                /* If we see <[ or <<, just ignore the second char instead
                of doing as directed (define encoding file, fully embed); sorry.  */
                /* Options */
                q = parse_string_value(&mut p, endptr);
                if !q.is_null() {
                    let mut r: *const i8 = q;
                    let mut e: *const i8 = q.offset(strlen(q) as isize);
                    let mut s: *mut i8 = 0 as *mut i8;
                    let mut t: *mut i8 = 0 as *mut i8;
                    skip_blank(&mut r, e);
                    while r < e {
                        s = parse_float_decimal(&mut r, e);
                        if !s.is_null() {
                            skip_blank(&mut r, e);
                            t = parse_string_value(&mut r, e);
                            if !t.is_null() {
                                if streq_ptr(t, b"SlantFont\x00" as *const u8 as *const i8) {
                                    (*mrec).opt.slant = atof(s)
                                } else if streq_ptr(t, b"ExtendFont\x00" as *const u8 as *const i8)
                                {
                                    (*mrec).opt.extend = atof(s)
                                }
                                free(t as *mut libc::c_void);
                            }
                            free(s as *mut libc::c_void);
                        } else {
                            s = parse_string_value(&mut r, e);
                            if !s.is_null() {
                                /* skip */
                                free(s as *mut libc::c_void); /* including two '@' */
                            }
                        }
                        skip_blank(&mut r, e);
                    }
                    free(q as *mut libc::c_void);
                }
                skip_blank(&mut p, endptr);
            }
            _ => {
                dpx_warning(
                    b"Found an invalid entry: %s\x00" as *const u8 as *const i8,
                    p,
                );
                return -1i32;
            }
        }
        skip_blank(&mut p, endptr);
    }
    if p < endptr && *p as i32 != '\r' as i32 && *p as i32 != '\n' as i32 {
        dpx_warning(
            b"Invalid char in fontmap line: %c\x00" as *const u8 as *const i8,
            *p as i32,
        );
        return -1i32;
    }
    0i32
}
static mut fontmap: *mut ht_table = 0 as *const ht_table as *mut ht_table;
unsafe extern "C" fn chop_sfd_name(mut tex_name: *const i8, mut sfd_name: *mut *mut i8) -> *mut i8 {
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut len: i32 = 0;
    *sfd_name = 0 as *mut i8;
    p = strchr(tex_name, '@' as i32);
    if p.is_null() || *p.offset(1) as i32 == '\u{0}' as i32 || p == tex_name as *mut i8 {
        return 0 as *mut i8;
    }
    m = p.wrapping_offset_from(tex_name) as i64 as i32;
    p = p.offset(1);
    q = strchr(p, '@' as i32);
    if q.is_null() || q == p {
        return 0 as *mut i8;
    }
    n = q.wrapping_offset_from(p) as i64 as i32;
    q = q.offset(1);
    len = strlen(tex_name).wrapping_sub(n as u64) as i32;
    fontname =
        new(((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    memcpy(
        fontname as *mut libc::c_void,
        tex_name as *const libc::c_void,
        m as u64,
    );
    *fontname.offset(m as isize) = '\u{0}' as i32 as i8;
    if *q != 0 {
        strcat(fontname, q);
    }
    *sfd_name =
        new(((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    memcpy(
        *sfd_name as *mut libc::c_void,
        p as *const libc::c_void,
        n as u64,
    );
    *(*sfd_name).offset(n as isize) = '\u{0}' as i32 as i8;
    fontname
}
unsafe extern "C" fn make_subfont_name(
    mut map_name: *const i8,
    mut sfd_name: *const i8,
    mut sub_id: *const i8,
) -> *mut i8 {
    let mut tfm_name: *mut i8 = 0 as *mut i8;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    p = strchr(map_name, '@' as i32);
    if p.is_null() || p == map_name as *mut i8 {
        return 0 as *mut i8;
    }
    m = p.wrapping_offset_from(map_name) as i64 as i32;
    q = strchr(p.offset(1), '@' as i32);
    if q.is_null() || q == p.offset(1) {
        return 0 as *mut i8;
    }
    n = q.wrapping_offset_from(p) as i64 as i32 + 1i32;
    if strlen(sfd_name) != (n - 2i32) as u64
        || memcmp(
            p.offset(1) as *const libc::c_void,
            sfd_name as *const libc::c_void,
            (n - 2i32) as u64,
        ) != 0
    {
        return 0 as *mut i8;
    }
    tfm_name = new((strlen(map_name)
        .wrapping_sub(n as u64)
        .wrapping_add(strlen(sub_id))
        .wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    memcpy(
        tfm_name as *mut libc::c_void,
        map_name as *const libc::c_void,
        m as u64,
    );
    *tfm_name.offset(m as isize) = '\u{0}' as i32 as i8;
    strcat(tfm_name, sub_id);
    if *q.offset(1) != 0 {
        /* not ending with '@' */
        strcat(tfm_name, q.offset(1));
    }
    tfm_name
}
/* "foo@A@ ..." is expanded to
 *   fooab ... -m sfd:A,ab
 *   ...
 *   fooyz ... -m sfd:A,yz
 * where 'ab' ... 'yz' is subfont IDs in SFD 'A'.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_append_fontmap_record(
    mut kp: *const i8,
    mut vp: *const fontmap_rec,
) -> i32 {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec; /* link */
    let mut fnt_name: *mut i8 = 0 as *mut i8; /* link to this entry */
    let mut sfd_name: *mut i8 = 0 as *mut i8;
    if kp.is_null() || (vp.is_null() || (*vp).map_name.is_null() || (*vp).font_name.is_null()) {
        warn!("Invalid fontmap record...");
        return -1i32;
    }
    if verbose > 3i32 {
        dpx_message(
            b"fontmap>> append key=\"%s\"...\x00" as *const u8 as *const i8,
            kp,
        );
    }
    fnt_name = chop_sfd_name(kp, &mut sfd_name);
    if !fnt_name.is_null() && !sfd_name.is_null() {
        let mut tfm_name: *mut i8 = 0 as *mut i8;
        let mut subfont_ids: *mut *mut i8 = 0 as *mut *mut i8;
        let mut n: i32 = 0i32;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &mut n);
        if subfont_ids.is_null() {
            return -1i32;
        }
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 > 0i32) {
                break;
            }
            tfm_name = make_subfont_name(kp, sfd_name, *subfont_ids.offset(n as isize));
            if tfm_name.is_null() {
                continue;
            }
            mrec = ht_lookup_table(
                fontmap,
                tfm_name as *const libc::c_void,
                strlen(tfm_name) as i32,
            ) as *mut fontmap_rec;
            if mrec.is_null() {
                mrec = new((1_u64).wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64) as u32)
                    as *mut fontmap_rec;
                pdf_init_fontmap_record(mrec);
                (*mrec).map_name = mstrdup(kp);
                (*mrec).charmap.sfd_name = mstrdup(sfd_name);
                (*mrec).charmap.subfont_id = mstrdup(*subfont_ids.offset(n as isize));
                ht_insert_table(
                    fontmap,
                    tfm_name as *const libc::c_void,
                    strlen(tfm_name) as i32,
                    mrec as *mut libc::c_void,
                );
            }
            free(tfm_name as *mut libc::c_void);
        }
        free(fnt_name as *mut libc::c_void);
        free(sfd_name as *mut libc::c_void);
    }
    mrec =
        ht_lookup_table(fontmap, kp as *const libc::c_void, strlen(kp) as i32) as *mut fontmap_rec;
    if mrec.is_null() {
        mrec = new((1_u64).wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64) as u32)
            as *mut fontmap_rec;
        pdf_copy_fontmap_record(mrec, vp);
        if !(*mrec).map_name.is_null() && streq_ptr(kp, (*mrec).map_name) as i32 != 0 {
            (*mrec).map_name = mfree((*mrec).map_name as *mut libc::c_void) as *mut i8
        }
        ht_insert_table(
            fontmap,
            kp as *const libc::c_void,
            strlen(kp) as i32,
            mrec as *mut libc::c_void,
        );
    }
    if verbose > 3i32 {
        info!("\n");
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_remove_fontmap_record(mut kp: *const i8) -> i32 {
    let mut fnt_name: *mut i8 = 0 as *mut i8;
    let mut sfd_name: *mut i8 = 0 as *mut i8;
    if kp.is_null() {
        return -1i32;
    }
    if verbose > 3i32 {
        dpx_message(
            b"fontmap>> remove key=\"%s\"...\x00" as *const u8 as *const i8,
            kp,
        );
    }
    fnt_name = chop_sfd_name(kp, &mut sfd_name);
    if !fnt_name.is_null() && !sfd_name.is_null() {
        let mut tfm_name: *mut i8 = 0 as *mut i8;
        let mut subfont_ids: *mut *mut i8 = 0 as *mut *mut i8;
        let mut n: i32 = 0i32;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &mut n);
        if subfont_ids.is_null() {
            return -1i32;
        }
        if verbose > 3i32 {
            dpx_message(
                b"\nfontmap>> Expand @%s@:\x00" as *const u8 as *const i8,
                sfd_name,
            );
        }
        loop {
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 > 0i32) {
                break;
            }
            tfm_name = make_subfont_name(kp, sfd_name, *subfont_ids.offset(n as isize));
            if tfm_name.is_null() {
                continue;
            }
            if verbose > 3i32 {
                dpx_message(b" %s\x00" as *const u8 as *const i8, tfm_name);
            }
            ht_remove_table(
                fontmap,
                tfm_name as *const libc::c_void,
                strlen(tfm_name) as i32,
            );
            free(tfm_name as *mut libc::c_void);
        }
        free(fnt_name as *mut libc::c_void);
        free(sfd_name as *mut libc::c_void);
    }
    ht_remove_table(fontmap, kp as *const libc::c_void, strlen(kp) as i32);
    if verbose > 3i32 {
        info!("\n");
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_insert_fontmap_record(
    mut kp: *const i8,
    mut vp: *const fontmap_rec,
) -> *mut fontmap_rec {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut fnt_name: *mut i8 = 0 as *mut i8;
    let mut sfd_name: *mut i8 = 0 as *mut i8;
    if kp.is_null() || (vp.is_null() || (*vp).map_name.is_null() || (*vp).font_name.is_null()) {
        warn!("Invalid fontmap record...");
        return 0 as *mut fontmap_rec;
    }
    if verbose > 3i32 {
        dpx_message(
            b"fontmap>> insert key=\"%s\"...\x00" as *const u8 as *const i8,
            kp,
        );
    }
    fnt_name = chop_sfd_name(kp, &mut sfd_name);
    if !fnt_name.is_null() && !sfd_name.is_null() {
        let mut tfm_name: *mut i8 = 0 as *mut i8;
        let mut subfont_ids: *mut *mut i8 = 0 as *mut *mut i8;
        let mut n: i32 = 0i32;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &mut n);
        if subfont_ids.is_null() {
            dpx_warning(
                b"Could not open SFD file: %s\x00" as *const u8 as *const i8,
                sfd_name,
            );
            free(fnt_name as *mut libc::c_void);
            free(sfd_name as *mut libc::c_void);
            return 0 as *mut fontmap_rec;
        }
        if verbose > 3i32 {
            dpx_message(
                b"\nfontmap>> Expand @%s@:\x00" as *const u8 as *const i8,
                sfd_name,
            );
        }
        loop {
            let fresh2 = n;
            n = n - 1;
            if !(fresh2 > 0i32) {
                break;
            }
            tfm_name = make_subfont_name(kp, sfd_name, *subfont_ids.offset(n as isize));
            if tfm_name.is_null() {
                continue;
            }
            if verbose > 3i32 {
                dpx_message(b" %s\x00" as *const u8 as *const i8, tfm_name);
            }
            mrec = new((1_u64).wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64) as u32)
                as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            (*mrec).map_name = mstrdup(kp);
            (*mrec).charmap.sfd_name = mstrdup(sfd_name);
            (*mrec).charmap.subfont_id = mstrdup(*subfont_ids.offset(n as isize));
            ht_insert_table(
                fontmap,
                tfm_name as *const libc::c_void,
                strlen(tfm_name) as i32,
                mrec as *mut libc::c_void,
            );
            free(tfm_name as *mut libc::c_void);
        }
        free(fnt_name as *mut libc::c_void);
        free(sfd_name as *mut libc::c_void);
    }
    mrec = new((1_u64).wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64) as u32)
        as *mut fontmap_rec;
    pdf_copy_fontmap_record(mrec, vp);
    if !(*mrec).map_name.is_null() && streq_ptr(kp, (*mrec).map_name) as i32 != 0 {
        (*mrec).map_name = mfree((*mrec).map_name as *mut libc::c_void) as *mut i8
    }
    ht_insert_table(
        fontmap,
        kp as *const libc::c_void,
        strlen(kp) as i32,
        mrec as *mut libc::c_void,
    );
    if verbose > 3i32 {
        info!("\n");
    }
    mrec
}
#[no_mangle]
pub unsafe extern "C" fn pdf_read_fontmap_line(
    mut mrec: *mut fontmap_rec,
    mut mline: *const i8,
    mut mline_len: i32,
    mut format: i32,
) -> i32 {
    let mut error: i32 = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    assert!(!mrec.is_null());
    p = mline;
    endptr = p.offset(mline_len as isize);
    skip_blank(&mut p, endptr);
    if p >= endptr {
        return -1i32;
    }
    q = parse_string_value(&mut p, endptr);
    if q.is_null() {
        return -1i32;
    }
    if format > 0i32 {
        /* DVIPDFM format */
        error = fontmap_parse_mapdef_dpm(mrec, p, endptr)
    } else {
        /* DVIPS/pdfTeX format */
        error = fontmap_parse_mapdef_dps(mrec, p, endptr)
    }
    if error == 0 {
        let mut fnt_name: *mut i8 = 0 as *mut i8;
        let mut sfd_name: *mut i8 = 0 as *mut i8;
        fnt_name = chop_sfd_name(q, &mut sfd_name);
        if !fnt_name.is_null() && !sfd_name.is_null() {
            if (*mrec).font_name.is_null() {
                /* In the case of subfonts, the base name (before the character '@')
                 * will be used as a font_name by default.
                 * Otherwise tex_name will be used as a font_name by default.
                 */
                (*mrec).font_name = fnt_name
            } else {
                free(fnt_name as *mut libc::c_void);
            }
            free((*mrec).charmap.sfd_name as *mut libc::c_void);
            (*mrec).charmap.sfd_name = sfd_name
        }
        fill_in_defaults(mrec, q);
    }
    free(q as *mut libc::c_void);
    error
}
/* DVIPS/pdfTeX fontmap line if one of the following three cases found:
 *
 * (1) any line including the character '"'
 * (2) any line including the character '<'
 * (3) if the line consists of two entries (tfmname and psname)
 *
 * DVIPDFM fontmap line otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn is_pdfm_mapline(mut mline: *const i8) -> i32
/* NULL terminated. */ {
    let mut n: u32 = 0_u32; /* DVIPS/pdfTeX format */
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    if !strchr(mline, '\"' as i32).is_null() || !strchr(mline, '<' as i32).is_null() {
        return -1i32;
    }
    p = mline;
    endptr = p.offset(strlen(mline) as isize);
    skip_blank(&mut p, endptr);
    while p < endptr {
        /* Break if '-' preceeded by blanks is found. (DVIPDFM format) */
        if *p as i32 == '-' as i32 {
            return 1i32;
        }
        n = n.wrapping_add(1);
        while p < endptr && !(*p as i32 & !0x7fi32 == 0i32 && crate::isblank(*p as _) != 0) {
            p = p.offset(1)
        }
        skip_blank(&mut p, endptr);
    }
    /* Two entries: TFM_NAME PS_NAME only (DVIPS format)
     * Otherwise (DVIPDFM format) */
    if n == 2_u32 {
        0i32
    } else {
        1i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_load_fontmap_file(mut filename: *const i8, mut mode: i32) -> i32 {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut llen: i32 = 0;
    let mut lpos: i32 = 0i32;
    let mut error: i32 = 0i32;
    let mut format: i32 = 0i32;
    assert!(!filename.is_null());
    assert!(!fontmap.is_null());
    if verbose != 0 {
        info!("<FONTMAP:");
    }
    handle = dpx_tt_open(
        filename,
        b".map\x00" as *const u8 as *const i8,
        TTInputFormat::FONTMAP,
    );
    if handle.is_null() {
        dpx_warning(
            b"Couldn\'t open font map file \"%s\".\x00" as *const u8 as *const i8,
            filename,
        );
        return -1i32;
    }
    while error == 0 && {
        p = tt_readline(work_buffer.as_mut_ptr(), 1024i32, handle);
        !p.is_null()
    } {
        let mut m: i32 = 0;
        lpos += 1;
        llen = strlen(work_buffer.as_mut_ptr()) as i32;
        endptr = p.offset(llen as isize);
        skip_blank(&mut p, endptr);
        if p == endptr {
            continue;
        }
        m = is_pdfm_mapline(p);
        if format * m < 0i32 {
            /* mismatch */
            dpx_warning(
                b"Found a mismatched fontmap line %d from %s.\x00" as *const u8 as *const i8,
                lpos,
                filename,
            );
            dpx_warning(
                b"-- Ignore the current input buffer: %s\x00" as *const u8 as *const i8,
                p,
            );
        } else {
            format += m;
            mrec = new((1_u64).wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64) as u32)
                as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            /* format > 0: DVIPDFM, format <= 0: DVIPS/pdfTeX */
            error = pdf_read_fontmap_line(mrec, p, llen, format); // CHECK
            if error != 0 {
                dpx_warning(
                    b"Invalid map record in fontmap line %d from %s.\x00" as *const u8 as *const i8,
                    lpos,
                    filename,
                );
                dpx_warning(
                    b"-- Ignore the current input buffer: %s\x00" as *const u8 as *const i8,
                    p,
                );
                pdf_clear_fontmap_record(mrec);
                free(mrec as *mut libc::c_void);
            } else {
                match mode {
                    0 => {
                        pdf_insert_fontmap_record((*mrec).map_name, mrec);
                    }
                    43 => {
                        pdf_append_fontmap_record((*mrec).map_name, mrec);
                    }
                    45 => {
                        pdf_remove_fontmap_record((*mrec).map_name);
                    }
                    _ => {}
                }
                pdf_clear_fontmap_record(mrec);
                free(mrec as *mut libc::c_void);
            }
        }
    }
    ttstub_input_close(handle);
    if verbose != 0 {
        info!(">");
    }
    error
}
#[no_mangle]
pub unsafe extern "C" fn pdf_insert_native_fontmap_record(
    mut path: *const i8,
    mut index: u32,
    mut layout_dir: i32,
    mut extend: i32,
    mut slant: i32,
    mut embolden: i32,
) -> *mut fontmap_rec {
    let mut fontmap_key: *mut i8 = 0 as *mut i8;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut ret: *mut fontmap_rec = 0 as *mut fontmap_rec;
    assert!(!path.is_null());
    fontmap_key = xmalloc(strlen(path).wrapping_add(40i32 as u64)) as *mut i8;
    sprintf(
        fontmap_key,
        b"%s/%d/%c/%d/%d/%d\x00" as *const u8 as *const i8,
        path,
        index,
        if layout_dir == 0i32 {
            'H' as i32
        } else {
            'V' as i32
        },
        extend,
        slant,
        embolden,
    );
    if verbose != 0 {
        dpx_message(
            b"<NATIVE-FONTMAP:%s\x00" as *const u8 as *const i8,
            fontmap_key,
        );
    }
    mrec = new((1_u64).wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64) as u32)
        as *mut fontmap_rec;
    pdf_init_fontmap_record(mrec);
    (*mrec).map_name = fontmap_key;
    (*mrec).enc_name = mstrdup(if layout_dir == 0i32 {
        b"Identity-H\x00" as *const u8 as *const i8
    } else {
        b"Identity-V\x00" as *const u8 as *const i8
    });
    (*mrec).font_name = mstrdup(path);
    (*mrec).opt.index = index as i32;
    if layout_dir != 0i32 {
        (*mrec).opt.flags |= 1i32 << 2i32
    }
    fill_in_defaults(mrec, fontmap_key);
    free(fontmap_key as *mut libc::c_void);
    (*mrec).opt.extend = extend as f64 / 65536.0f64;
    (*mrec).opt.slant = slant as f64 / 65536.0f64;
    (*mrec).opt.bold = embolden as f64 / 65536.0f64;
    ret = pdf_insert_fontmap_record((*mrec).map_name, mrec);
    pdf_clear_fontmap_record(mrec);
    free(mrec as *mut libc::c_void);
    if verbose != 0 {
        info!(">");
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn pdf_lookup_fontmap_record(mut tfm_name: *const i8) -> *mut fontmap_rec {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    if !fontmap.is_null() && !tfm_name.is_null() {
        mrec = ht_lookup_table(
            fontmap,
            tfm_name as *const libc::c_void,
            strlen(tfm_name) as i32,
        ) as *mut fontmap_rec
    }
    mrec
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_fontmaps() {
    fontmap =
        new((1_u64).wrapping_mul(::std::mem::size_of::<ht_table>() as u64) as u32) as *mut ht_table;
    ht_init_table(
        fontmap,
        Some(hval_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
}
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
/* Options */
/* Synthetic font */
/* comaptibility and other flags */
/* currently unused */
/* not implemented yet */
/* unused */
/* Adobe-Japan1-4, etc. */
/* TTC index */
/* ,Bold, etc. */
/* StemV value especially for CJK fonts */
/* Subfont mapping: translate 8-bit charcode to 16-bit charcode
 * via SFD.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_close_fontmaps() {
    if !fontmap.is_null() {
        ht_clear_table(fontmap);
        free(fontmap as *mut libc::c_void);
    }
    fontmap = 0 as *mut ht_table;
    release_sfd_record();
}
/* CIDFont options
 *
 * FORMAT:
 *
 *   (:int:)?!?string(/string)?(,string)?
 */
unsafe extern "C" fn substr(mut str: *mut *const i8, mut stop: i8) -> *mut i8 {
    let mut sstr: *mut i8 = 0 as *mut i8;
    let mut endptr: *const i8 = 0 as *const i8;
    endptr = strchr(*str, stop as i32);
    if endptr.is_null() || endptr == *str {
        return 0 as *mut i8;
    }
    sstr = new(
        ((endptr.wrapping_offset_from(*str) as i64 + 1i32 as i64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    memcpy(
        sstr as *mut libc::c_void,
        *str as *const libc::c_void,
        endptr.wrapping_offset_from(*str) as i64 as u64,
    );
    *sstr.offset(endptr.wrapping_offset_from(*str) as i64 as isize) = '\u{0}' as i32 as i8;
    *str = endptr.offset(1);
    sstr
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
/* CIDFont */
unsafe extern "C" fn strip_options(mut map_name: *const i8, mut opt: *mut fontmap_opt) -> *mut i8 {
    let mut font_name: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut next: *mut i8 = 0 as *mut i8;
    let mut have_csi: i32 = 0i32;
    let mut have_style: i32 = 0i32;
    assert!(!opt.is_null());
    p = map_name;
    font_name = 0 as *mut i8;
    (*opt).charcoll = 0 as *mut i8;
    (*opt).index = 0i32;
    (*opt).style = 0i32;
    (*opt).flags = 0i32;
    if *p as i32 == ':' as i32 && libc::isdigit(*p.offset(1) as _) != 0 {
        (*opt).index = strtoul(p.offset(1), &mut next, 10i32) as i32;
        if *next as i32 == ':' as i32 {
            p = next.offset(1)
        } else {
            (*opt).index = 0i32
        }
    }
    if *p as i32 == '!' as i32 {
        /* no-embedding */
        p = p.offset(1);
        if *p as i32 == '\u{0}' as i32 {
            _tt_abort(
                b"Invalid map record: %s (--> %s)\x00" as *const u8 as *const i8,
                map_name,
                p,
            );
        }
        (*opt).flags |= 1i32 << 1i32
    }
    next = strchr(p, '/' as i32);
    if !next.is_null() {
        if next == p as *mut i8 {
            _tt_abort(
                b"Invalid map record: %s (--> %s)\x00" as *const u8 as *const i8,
                map_name,
                p,
            );
        }
        font_name = substr(&mut p, '/' as i32 as i8);
        have_csi = 1i32
    } else {
        next = strchr(p, ',' as i32);
        if !next.is_null() {
            if next == p as *mut i8 {
                _tt_abort(
                    b"Invalid map record: %s (--> %s)\x00" as *const u8 as *const i8,
                    map_name,
                    p,
                );
            }
            font_name = substr(&mut p, ',' as i32 as i8);
            have_style = 1i32
        } else {
            font_name = new((strlen(p).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            strcpy(font_name, p);
        }
    }
    if have_csi != 0 {
        next = strchr(p, ',' as i32);
        if !next.is_null() {
            (*opt).charcoll = substr(&mut p, ',' as i32 as i8);
            have_style = 1i32
        } else if *p.offset(0) as i32 == '\u{0}' as i32 {
            _tt_abort(
                b"Invalid map record: %s.\x00" as *const u8 as *const i8,
                map_name,
            );
        } else {
            (*opt).charcoll = new((strlen(p).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            strcpy((*opt).charcoll, p);
        }
    }
    if have_style != 0 {
        if !strstartswith(p, b"BoldItalic\x00" as *const u8 as *const i8).is_null() {
            if *p.offset(10) != 0 {
                _tt_abort(
                    b"Invalid map record: %s (--> %s)\x00" as *const u8 as *const i8,
                    map_name,
                    p,
                );
            }
            (*opt).style = 3i32
        } else if !strstartswith(p, b"Bold\x00" as *const u8 as *const i8).is_null() {
            if *p.offset(4) != 0 {
                _tt_abort(
                    b"Invalid map record: %s (--> %s)\x00" as *const u8 as *const i8,
                    map_name,
                    p,
                );
            }
            (*opt).style = 1i32
        } else if !strstartswith(p, b"Italic\x00" as *const u8 as *const i8).is_null() {
            if *p.offset(6) != 0 {
                _tt_abort(
                    b"Invalid map record: %s (--> %s)\x00" as *const u8 as *const i8,
                    map_name,
                    p,
                );
            }
            (*opt).style = 2i32
        }
    }
    font_name
}
