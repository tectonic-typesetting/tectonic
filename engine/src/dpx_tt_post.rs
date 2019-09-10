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
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut i8,
        len: size_t,
    ) -> ssize_t;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const i8) -> *mut i8;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> i16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
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
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type BYTE = u8;
pub type USHORT = u16;
pub type SHORT = i16;
pub type SFNT_ULONG = u32;
pub type Fixed = u32;
pub type FWord = i16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut i8,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: i32,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const i8,
    pub names: *mut *mut i8,
    pub count: USHORT,
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
    return 0i32 != 0;
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
/* offset from begenning of the post table */
unsafe extern "C" fn read_v2_post_names(
    mut post: *mut tt_post_table,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut i: USHORT = 0;
    let mut idx: USHORT = 0;
    let mut indices: *mut USHORT = 0 as *mut USHORT;
    let mut maxidx: USHORT = 0;
    let mut len: i32 = 0;
    (*post).numberOfGlyphs = tt_get_unsigned_pair((*sfont).handle);
    indices = new(((*post).numberOfGlyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as u64)
        as u32) as *mut USHORT;
    maxidx = 257i32 as USHORT;
    i = 0i32 as USHORT;
    while (i as i32) < (*post).numberOfGlyphs as i32 {
        idx = tt_get_unsigned_pair((*sfont).handle);
        if idx as i32 >= 258i32 {
            if idx as i32 > maxidx as i32 {
                maxidx = idx
            }
            if idx as i32 > 32767i32 {
                /* Although this is strictly speaking out of spec, it seems to work
                and there are real-life fonts that use it.
                We show a warning only once, instead of thousands of times */
                static mut warning_issued: i8 = 0i32 as i8;
                if warning_issued == 0 {
                    dpx_warning(
                        b"TrueType post table name index %u > 32767\x00" as *const u8
                            as *const i8,
                        idx as i32,
                    );
                    warning_issued = 1i32 as i8
                }
                /* In a real-life large font, (x)dvipdfmx crashes if we use
                nonvanishing idx in the case of idx > 32767.
                If we set idx = 0, (x)dvipdfmx works fine for the font and
                created pdf seems fine. The post table may not be important
                in such a case */
                idx = 0i32 as USHORT
            }
        }
        *indices.offset(i as isize) = idx;
        i = i.wrapping_add(1)
    }
    (*post).count = (maxidx as i32 - 257i32) as USHORT;
    if ((*post).count as i32) < 1i32 {
        (*post).names = 0 as *mut *mut i8
    } else {
        (*post).names = new(((*post).count as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64)
            as u32) as *mut *mut i8;
        i = 0i32 as USHORT;
        while (i as i32) < (*post).count as i32 {
            /* read Pascal strings */
            len = tt_get_unsigned_byte((*sfont).handle) as i32;
            if len > 0i32 {
                let ref mut fresh0 = *(*post).names.offset(i as isize);
                *fresh0 = new(((len + 1i32) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                    as u32) as *mut i8;
                ttstub_input_read(
                    (*sfont).handle,
                    *(*post).names.offset(i as isize),
                    len as size_t,
                );
                *(*(*post).names.offset(i as isize)).offset(len as isize) = 0i32 as i8
            } else {
                let ref mut fresh1 = *(*post).names.offset(i as isize);
                *fresh1 = 0 as *mut i8
            }
            i = i.wrapping_add(1)
        }
    }
    (*post).glyphNamePtr = new(((*post).numberOfGlyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*const i8>() as u64)
        as u32) as *mut *const i8;
    i = 0i32 as USHORT;
    while (i as i32) < (*post).numberOfGlyphs as i32 {
        idx = *indices.offset(i as isize);
        if (idx as i32) < 258i32 {
            let ref mut fresh2 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh2 = macglyphorder[idx as usize]
        } else if idx as i32 - 258i32 < (*post).count as i32 {
            let ref mut fresh3 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh3 = *(*post).names.offset((idx as i32 - 258i32) as isize)
        } else {
            dpx_warning(
                b"Invalid glyph name index number: %u (>= %u)\x00" as *const u8
                    as *const i8,
                idx as i32,
                (*post).count as i32 + 258i32,
            );
            free(indices as *mut libc::c_void);
            return -1i32;
        }
        i = i.wrapping_add(1)
    }
    free(indices as *mut libc::c_void);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_post_table(mut sfont: *mut sfnt) -> *mut tt_post_table {
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    /* offset = */
    sfnt_locate_table(sfont, b"post\x00" as *const u8 as *const i8); /* Fixed */
    post = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<tt_post_table>() as u64)
        as u32) as *mut tt_post_table; /* Fixed */
    (*post).Version = tt_get_unsigned_quad((*sfont).handle); /* FWord */
    (*post).italicAngle = tt_get_unsigned_quad((*sfont).handle); /* FWord */
    (*post).underlinePosition = tt_get_signed_pair((*sfont).handle); /* wrong */
    (*post).underlineThickness = tt_get_signed_pair((*sfont).handle);
    (*post).isFixedPitch = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).numberOfGlyphs = 0i32 as USHORT;
    (*post).glyphNamePtr = 0 as *mut *const i8;
    (*post).count = 0i32 as USHORT;
    (*post).names = 0 as *mut *mut i8;
    if (*post).Version as u64 == 0x10000 {
        (*post).numberOfGlyphs = 258i32 as USHORT;
        (*post).glyphNamePtr = macglyphorder.as_mut_ptr()
    } else if (*post).Version as u64 == 0x28000 {
        dpx_warning(
            b"TrueType \'post\' version 2.5 found (deprecated)\x00" as *const u8
                as *const i8,
        );
    } else if (*post).Version as u64 == 0x20000 {
        if read_v2_post_names(post, sfont) < 0i32 {
            dpx_warning(
                b"Invalid version 2.0 \'post\' table\x00" as *const u8 as *const i8,
            );
            tt_release_post_table(post);
            post = 0 as *mut tt_post_table
        }
    } else if !((*post).Version as u64 == 0x30000
        || (*post).Version as u64 == 0x40000)
    {
        dpx_warning(
            b"Unknown \'post\' version: %08X, assuming version 3.0\x00" as *const u8
                as *const i8,
            (*post).Version,
        );
    }
    return post;
}
#[no_mangle]
pub unsafe extern "C" fn tt_lookup_post_table(
    mut post: *mut tt_post_table,
    mut glyphname: *const i8,
) -> USHORT {
    let mut gid: USHORT = 0;
    if !post.is_null() && !glyphname.is_null() {
    } else {
        __assert_fail(
            b"post && glyphname\x00" as *const u8 as *const i8,
            b"dpx-tt_post.c\x00" as *const u8 as *const i8,
            157i32 as u32,
            (*::std::mem::transmute::<&[u8; 66], &[i8; 66]>(
                b"USHORT tt_lookup_post_table(struct tt_post_table *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    gid = 0i32 as USHORT;
    while (gid as i32) < (*post).count as i32 {
        if !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
            && streq_ptr(glyphname, *(*post).glyphNamePtr.offset(gid as isize)) as i32 != 0
        {
            return gid;
        }
        gid = gid.wrapping_add(1)
    }
    return 0i32 as USHORT;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_glyphname(
    mut post: *mut tt_post_table,
    mut gid: USHORT,
) -> *mut i8 {
    if (gid as i32) < (*post).count as i32
        && !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
    {
        return xstrdup(*(*post).glyphNamePtr.offset(gid as isize));
    }
    return 0 as *mut i8;
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
/* Glyph names (pointer to C string) */
/* Non-standard glyph names */
/* Number of glyph names in names[] */
#[no_mangle]
pub unsafe extern "C" fn tt_release_post_table(mut post: *mut tt_post_table) {
    let mut i: USHORT = 0;
    if !post.is_null() {
    } else {
        __assert_fail(
            b"post\x00" as *const u8 as *const i8,
            b"dpx-tt_post.c\x00" as *const u8 as *const i8,
            182i32 as u32,
            (*::std::mem::transmute::<&[u8; 51], &[i8; 51]>(
                b"void tt_release_post_table(struct tt_post_table *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*post).glyphNamePtr.is_null() && (*post).Version as u64 != 0x10000 {
        free((*post).glyphNamePtr as *mut libc::c_void);
    }
    if !(*post).names.is_null() {
        i = 0i32 as USHORT;
        while (i as i32) < (*post).count as i32 {
            free(*(*post).names.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free((*post).names as *mut libc::c_void);
    }
    (*post).count = 0i32 as USHORT;
    (*post).glyphNamePtr = 0 as *mut *const i8;
    (*post).names = 0 as *mut *mut i8;
    free(post as *mut libc::c_void);
}
/* Macintosh glyph order - from apple's TTRefMan */
static mut macglyphorder: [*const i8; 258] = [
    b".notdef\x00" as *const u8 as *const i8,
    b".null\x00" as *const u8 as *const i8,
    b"nonmarkingreturn\x00" as *const u8 as *const i8,
    b"space\x00" as *const u8 as *const i8,
    b"exclam\x00" as *const u8 as *const i8,
    b"quotedbl\x00" as *const u8 as *const i8,
    b"numbersign\x00" as *const u8 as *const i8,
    b"dollar\x00" as *const u8 as *const i8,
    b"percent\x00" as *const u8 as *const i8,
    b"ampersand\x00" as *const u8 as *const i8,
    b"quotesingle\x00" as *const u8 as *const i8,
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
    b"grave\x00" as *const u8 as *const i8,
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
    b"Adieresis\x00" as *const u8 as *const i8,
    b"Aring\x00" as *const u8 as *const i8,
    b"Ccedilla\x00" as *const u8 as *const i8,
    b"Eacute\x00" as *const u8 as *const i8,
    b"Ntilde\x00" as *const u8 as *const i8,
    b"Odieresis\x00" as *const u8 as *const i8,
    b"Udieresis\x00" as *const u8 as *const i8,
    b"aacute\x00" as *const u8 as *const i8,
    b"agrave\x00" as *const u8 as *const i8,
    b"acircumflex\x00" as *const u8 as *const i8,
    b"adieresis\x00" as *const u8 as *const i8,
    b"atilde\x00" as *const u8 as *const i8,
    b"aring\x00" as *const u8 as *const i8,
    b"ccedilla\x00" as *const u8 as *const i8,
    b"eacute\x00" as *const u8 as *const i8,
    b"egrave\x00" as *const u8 as *const i8,
    b"ecircumflex\x00" as *const u8 as *const i8,
    b"edieresis\x00" as *const u8 as *const i8,
    b"iacute\x00" as *const u8 as *const i8,
    b"igrave\x00" as *const u8 as *const i8,
    b"icircumflex\x00" as *const u8 as *const i8,
    b"idieresis\x00" as *const u8 as *const i8,
    b"ntilde\x00" as *const u8 as *const i8,
    b"oacute\x00" as *const u8 as *const i8,
    b"ograve\x00" as *const u8 as *const i8,
    b"ocircumflex\x00" as *const u8 as *const i8,
    b"odieresis\x00" as *const u8 as *const i8,
    b"otilde\x00" as *const u8 as *const i8,
    b"uacute\x00" as *const u8 as *const i8,
    b"ugrave\x00" as *const u8 as *const i8,
    b"ucircumflex\x00" as *const u8 as *const i8,
    b"udieresis\x00" as *const u8 as *const i8,
    b"dagger\x00" as *const u8 as *const i8,
    b"degree\x00" as *const u8 as *const i8,
    b"cent\x00" as *const u8 as *const i8,
    b"sterling\x00" as *const u8 as *const i8,
    b"section\x00" as *const u8 as *const i8,
    b"bullet\x00" as *const u8 as *const i8,
    b"paragraph\x00" as *const u8 as *const i8,
    b"germandbls\x00" as *const u8 as *const i8,
    b"registered\x00" as *const u8 as *const i8,
    b"copyright\x00" as *const u8 as *const i8,
    b"trademark\x00" as *const u8 as *const i8,
    b"acute\x00" as *const u8 as *const i8,
    b"dieresis\x00" as *const u8 as *const i8,
    b"notequal\x00" as *const u8 as *const i8,
    b"AE\x00" as *const u8 as *const i8,
    b"Oslash\x00" as *const u8 as *const i8,
    b"infinity\x00" as *const u8 as *const i8,
    b"plusminus\x00" as *const u8 as *const i8,
    b"lessequal\x00" as *const u8 as *const i8,
    b"greaterequal\x00" as *const u8 as *const i8,
    b"yen\x00" as *const u8 as *const i8,
    b"mu\x00" as *const u8 as *const i8,
    b"partialdiff\x00" as *const u8 as *const i8,
    b"summation\x00" as *const u8 as *const i8,
    b"product\x00" as *const u8 as *const i8,
    b"pi\x00" as *const u8 as *const i8,
    b"integral\x00" as *const u8 as *const i8,
    b"ordfeminine\x00" as *const u8 as *const i8,
    b"ordmasculine\x00" as *const u8 as *const i8,
    b"Omega\x00" as *const u8 as *const i8,
    b"ae\x00" as *const u8 as *const i8,
    b"oslash\x00" as *const u8 as *const i8,
    b"questiondown\x00" as *const u8 as *const i8,
    b"exclamdown\x00" as *const u8 as *const i8,
    b"logicalnot\x00" as *const u8 as *const i8,
    b"radical\x00" as *const u8 as *const i8,
    b"florin\x00" as *const u8 as *const i8,
    b"approxequal\x00" as *const u8 as *const i8,
    b"Delta\x00" as *const u8 as *const i8,
    b"guillemotleft\x00" as *const u8 as *const i8,
    b"guillemotright\x00" as *const u8 as *const i8,
    b"ellipsis\x00" as *const u8 as *const i8,
    b"nonbreakingspace\x00" as *const u8 as *const i8,
    b"Agrave\x00" as *const u8 as *const i8,
    b"Atilde\x00" as *const u8 as *const i8,
    b"Otilde\x00" as *const u8 as *const i8,
    b"OE\x00" as *const u8 as *const i8,
    b"oe\x00" as *const u8 as *const i8,
    b"endash\x00" as *const u8 as *const i8,
    b"emdash\x00" as *const u8 as *const i8,
    b"quotedblleft\x00" as *const u8 as *const i8,
    b"quotedblright\x00" as *const u8 as *const i8,
    b"quoteleft\x00" as *const u8 as *const i8,
    b"quoteright\x00" as *const u8 as *const i8,
    b"divide\x00" as *const u8 as *const i8,
    b"lozenge\x00" as *const u8 as *const i8,
    b"ydieresis\x00" as *const u8 as *const i8,
    b"Ydieresis\x00" as *const u8 as *const i8,
    b"fraction\x00" as *const u8 as *const i8,
    b"currency\x00" as *const u8 as *const i8,
    b"guilsinglleft\x00" as *const u8 as *const i8,
    b"guilsinglright\x00" as *const u8 as *const i8,
    b"fi\x00" as *const u8 as *const i8,
    b"fl\x00" as *const u8 as *const i8,
    b"daggerdbl\x00" as *const u8 as *const i8,
    b"periodcentered\x00" as *const u8 as *const i8,
    b"quotesinglbase\x00" as *const u8 as *const i8,
    b"quotedblbase\x00" as *const u8 as *const i8,
    b"perthousand\x00" as *const u8 as *const i8,
    b"Acircumflex\x00" as *const u8 as *const i8,
    b"Ecircumflex\x00" as *const u8 as *const i8,
    b"Aacute\x00" as *const u8 as *const i8,
    b"Edieresis\x00" as *const u8 as *const i8,
    b"Egrave\x00" as *const u8 as *const i8,
    b"Iacute\x00" as *const u8 as *const i8,
    b"Icircumflex\x00" as *const u8 as *const i8,
    b"Idieresis\x00" as *const u8 as *const i8,
    b"Igrave\x00" as *const u8 as *const i8,
    b"Oacute\x00" as *const u8 as *const i8,
    b"Ocircumflex\x00" as *const u8 as *const i8,
    b"apple\x00" as *const u8 as *const i8,
    b"Ograve\x00" as *const u8 as *const i8,
    b"Uacute\x00" as *const u8 as *const i8,
    b"Ucircumflex\x00" as *const u8 as *const i8,
    b"Ugrave\x00" as *const u8 as *const i8,
    b"dotlessi\x00" as *const u8 as *const i8,
    b"circumflex\x00" as *const u8 as *const i8,
    b"tilde\x00" as *const u8 as *const i8,
    b"macron\x00" as *const u8 as *const i8,
    b"breve\x00" as *const u8 as *const i8,
    b"dotaccent\x00" as *const u8 as *const i8,
    b"ring\x00" as *const u8 as *const i8,
    b"cedilla\x00" as *const u8 as *const i8,
    b"hungarumlaut\x00" as *const u8 as *const i8,
    b"ogonek\x00" as *const u8 as *const i8,
    b"caron\x00" as *const u8 as *const i8,
    b"Lslash\x00" as *const u8 as *const i8,
    b"lslash\x00" as *const u8 as *const i8,
    b"Scaron\x00" as *const u8 as *const i8,
    b"scaron\x00" as *const u8 as *const i8,
    b"Zcaron\x00" as *const u8 as *const i8,
    b"zcaron\x00" as *const u8 as *const i8,
    b"brokenbar\x00" as *const u8 as *const i8,
    b"Eth\x00" as *const u8 as *const i8,
    b"eth\x00" as *const u8 as *const i8,
    b"Yacute\x00" as *const u8 as *const i8,
    b"yacute\x00" as *const u8 as *const i8,
    b"Thorn\x00" as *const u8 as *const i8,
    b"thorn\x00" as *const u8 as *const i8,
    b"minus\x00" as *const u8 as *const i8,
    b"multiply\x00" as *const u8 as *const i8,
    b"onesuperior\x00" as *const u8 as *const i8,
    b"twosuperior\x00" as *const u8 as *const i8,
    b"threesuperior\x00" as *const u8 as *const i8,
    b"onehalf\x00" as *const u8 as *const i8,
    b"onequarter\x00" as *const u8 as *const i8,
    b"threequarters\x00" as *const u8 as *const i8,
    b"franc\x00" as *const u8 as *const i8,
    b"Gbreve\x00" as *const u8 as *const i8,
    b"gbreve\x00" as *const u8 as *const i8,
    b"Idotaccent\x00" as *const u8 as *const i8,
    b"Scedilla\x00" as *const u8 as *const i8,
    b"scedilla\x00" as *const u8 as *const i8,
    b"Cacute\x00" as *const u8 as *const i8,
    b"cacute\x00" as *const u8 as *const i8,
    b"Ccaron\x00" as *const u8 as *const i8,
    b"ccaron\x00" as *const u8 as *const i8,
    b"dcroat\x00" as *const u8 as *const i8,
];
