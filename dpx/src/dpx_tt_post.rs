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
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use super::dpx_numbers::{
    tt_get_signed_pair, tt_get_unsigned_byte, tt_get_unsigned_pair, tt_get_unsigned_quad,
};
use super::dpx_sfnt::sfnt_locate_table;
use crate::streq_ptr;
use crate::warn;

use super::dpx_mem::{new, xstrdup};
use crate::ttstub_input_read;
use libc::free;

pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type Fixed = u32;
pub type FWord = i16;

use super::dpx_sfnt::sfnt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: u32,
    pub minMemType42: u32,
    pub maxMemType42: u32,
    pub minMemType1: u32,
    pub maxMemType1: u32,
    pub numberOfGlyphs: u16,
    pub glyphNamePtr: *mut *const i8,
    pub names: *mut *mut i8,
    pub count: u16,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
/* offset from begenning of the post table */
unsafe extern "C" fn read_v2_post_names(mut post: *mut tt_post_table, mut sfont: *mut sfnt) -> i32 {
    let mut i: u16 = 0;
    let mut idx: u16 = 0;
    let mut indices: *mut u16 = 0 as *mut u16;
    let mut maxidx: u16 = 0;
    let mut len: i32 = 0;
    (*post).numberOfGlyphs = tt_get_unsigned_pair((*sfont).handle);
    indices = new(((*post).numberOfGlyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32) as *mut u16;
    maxidx = 257_u16;
    i = 0_u16;
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
                static mut warning_issued: i8 = 0_i8;
                if warning_issued == 0 {
                    warn!("TrueType post table name index {} > 32767", idx);
                    warning_issued = 1_i8
                }
                /* In a real-life large font, (x)dvipdfmx crashes if we use
                nonvanishing idx in the case of idx > 32767.
                If we set idx = 0, (x)dvipdfmx works fine for the font and
                created pdf seems fine. The post table may not be important
                in such a case */
                idx = 0_u16
            }
        }
        *indices.offset(i as isize) = idx;
        i = i.wrapping_add(1)
    }
    (*post).count = (maxidx as i32 - 257i32) as u16;
    if ((*post).count as i32) < 1i32 {
        (*post).names = 0 as *mut *mut i8
    } else {
        (*post).names = new(((*post).count as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64)
            as u32) as *mut *mut i8;
        i = 0_u16;
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
                *(*(*post).names.offset(i as isize)).offset(len as isize) = 0_i8
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
    i = 0_u16;
    while (i as i32) < (*post).numberOfGlyphs as i32 {
        idx = *indices.offset(i as isize);
        if (idx as i32) < 258i32 {
            let ref mut fresh2 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh2 = macglyphorder[idx as usize].as_ptr() as *const i8
        } else if idx as i32 - 258i32 < (*post).count as i32 {
            let ref mut fresh3 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh3 = *(*post).names.offset((idx as i32 - 258i32) as isize)
        } else {
            warn!(
                "Invalid glyph name index number: {} (>= {})",
                idx,
                (*post).count + 258,
            );
            free(indices as *mut libc::c_void);
            return -1i32;
        }
        i = i.wrapping_add(1)
    }
    free(indices as *mut libc::c_void);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_post_table(mut sfont: *mut sfnt) -> *mut tt_post_table {
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    /* offset = */
    sfnt_locate_table(sfont, b"post\x00" as *const u8 as *const i8); /* Fixed */
    post = new((1_u64).wrapping_mul(::std::mem::size_of::<tt_post_table>() as u64) as u32)
        as *mut tt_post_table; /* Fixed */
    (*post).Version = tt_get_unsigned_quad((*sfont).handle); /* FWord */
    (*post).italicAngle = tt_get_unsigned_quad((*sfont).handle); /* FWord */
    (*post).underlinePosition = tt_get_signed_pair((*sfont).handle); /* wrong */
    (*post).underlineThickness = tt_get_signed_pair((*sfont).handle);
    (*post).isFixedPitch = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).numberOfGlyphs = 0_u16;
    (*post).glyphNamePtr = 0 as *mut *const i8;
    (*post).count = 0_u16;
    (*post).names = 0 as *mut *mut i8;
    if (*post).Version as u64 == 0x10000 {
        (*post).numberOfGlyphs = 258_u16;
        (*post).glyphNamePtr = macglyphorder.as_mut_ptr() as *mut *const u8 as *mut *const i8
    } else if (*post).Version as u64 == 0x28000 {
        warn!("TrueType \'post\' version 2.5 found (deprecated)");
    } else if (*post).Version as u64 == 0x20000 {
        if read_v2_post_names(post, sfont) < 0i32 {
            warn!("Invalid version 2.0 \'post\' table");
            tt_release_post_table(post);
            post = 0 as *mut tt_post_table
        }
    } else if !((*post).Version as u64 == 0x30000 || (*post).Version as u64 == 0x40000) {
        warn!(
            "Unknown \'post\' version: {:08X}, assuming version 3.0",
            (*post).Version,
        );
    }
    post
}
#[no_mangle]
pub unsafe extern "C" fn tt_lookup_post_table(
    mut post: *mut tt_post_table,
    mut glyphname: *const i8,
) -> u16 {
    let mut gid: u16 = 0;
    assert!(!post.is_null() && !glyphname.is_null());
    gid = 0_u16;
    while (gid as i32) < (*post).count as i32 {
        if !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
            && streq_ptr(glyphname, *(*post).glyphNamePtr.offset(gid as isize)) as i32 != 0
        {
            return gid;
        }
        gid = gid.wrapping_add(1)
    }
    0_u16
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_glyphname(mut post: *mut tt_post_table, mut gid: u16) -> *mut i8 {
    if (gid as i32) < (*post).count as i32
        && !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
    {
        return xstrdup(*(*post).glyphNamePtr.offset(gid as isize));
    }
    0 as *mut i8
}
/* Glyph names (pointer to C string) */
/* Non-standard glyph names */
/* Number of glyph names in names[] */
#[no_mangle]
pub unsafe extern "C" fn tt_release_post_table(mut post: *mut tt_post_table) {
    let mut i: u16 = 0;
    assert!(!post.is_null());
    if !(*post).glyphNamePtr.is_null() && (*post).Version as u64 != 0x10000 {
        free((*post).glyphNamePtr as *mut libc::c_void);
    }
    if !(*post).names.is_null() {
        i = 0_u16;
        while (i as i32) < (*post).count as i32 {
            free(*(*post).names.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free((*post).names as *mut libc::c_void);
    }
    (*post).count = 0_u16;
    (*post).glyphNamePtr = 0 as *mut *const i8;
    (*post).names = 0 as *mut *mut i8;
    free(post as *mut libc::c_void);
}
/* Macintosh glyph order - from apple's TTRefMan */
static mut macglyphorder: [&[u8]; 258] = [
    b".notdef\x00",
    b".null\x00",
    b"nonmarkingreturn\x00",
    b"space\x00",
    b"exclam\x00",
    b"quotedbl\x00",
    b"numbersign\x00",
    b"dollar\x00",
    b"percent\x00",
    b"ampersand\x00",
    b"quotesingle\x00",
    b"parenleft\x00",
    b"parenright\x00",
    b"asterisk\x00",
    b"plus\x00",
    b"comma\x00",
    b"hyphen\x00",
    b"period\x00",
    b"slash\x00",
    b"zero\x00",
    b"one\x00",
    b"two\x00",
    b"three\x00",
    b"four\x00",
    b"five\x00",
    b"six\x00",
    b"seven\x00",
    b"eight\x00",
    b"nine\x00",
    b"colon\x00",
    b"semicolon\x00",
    b"less\x00",
    b"equal\x00",
    b"greater\x00",
    b"question\x00",
    b"at\x00",
    b"A\x00",
    b"B\x00",
    b"C\x00",
    b"D\x00",
    b"E\x00",
    b"F\x00",
    b"G\x00",
    b"H\x00",
    b"I\x00",
    b"J\x00",
    b"K\x00",
    b"L\x00",
    b"M\x00",
    b"N\x00",
    b"O\x00",
    b"P\x00",
    b"Q\x00",
    b"R\x00",
    b"S\x00",
    b"T\x00",
    b"U\x00",
    b"V\x00",
    b"W\x00",
    b"X\x00",
    b"Y\x00",
    b"Z\x00",
    b"bracketleft\x00",
    b"backslash\x00",
    b"bracketright\x00",
    b"asciicircum\x00",
    b"underscore\x00",
    b"grave\x00",
    b"a\x00",
    b"b\x00",
    b"c\x00",
    b"d\x00",
    b"e\x00",
    b"f\x00",
    b"g\x00",
    b"h\x00",
    b"i\x00",
    b"j\x00",
    b"k\x00",
    b"l\x00",
    b"m\x00",
    b"n\x00",
    b"o\x00",
    b"p\x00",
    b"q\x00",
    b"r\x00",
    b"s\x00",
    b"t\x00",
    b"u\x00",
    b"v\x00",
    b"w\x00",
    b"x\x00",
    b"y\x00",
    b"z\x00",
    b"braceleft\x00",
    b"bar\x00",
    b"braceright\x00",
    b"asciitilde\x00",
    b"Adieresis\x00",
    b"Aring\x00",
    b"Ccedilla\x00",
    b"Eacute\x00",
    b"Ntilde\x00",
    b"Odieresis\x00",
    b"Udieresis\x00",
    b"aacute\x00",
    b"agrave\x00",
    b"acircumflex\x00",
    b"adieresis\x00",
    b"atilde\x00",
    b"aring\x00",
    b"ccedilla\x00",
    b"eacute\x00",
    b"egrave\x00",
    b"ecircumflex\x00",
    b"edieresis\x00",
    b"iacute\x00",
    b"igrave\x00",
    b"icircumflex\x00",
    b"idieresis\x00",
    b"ntilde\x00",
    b"oacute\x00",
    b"ograve\x00",
    b"ocircumflex\x00",
    b"odieresis\x00",
    b"otilde\x00",
    b"uacute\x00",
    b"ugrave\x00",
    b"ucircumflex\x00",
    b"udieresis\x00",
    b"dagger\x00",
    b"degree\x00",
    b"cent\x00",
    b"sterling\x00",
    b"section\x00",
    b"bullet\x00",
    b"paragraph\x00",
    b"germandbls\x00",
    b"registered\x00",
    b"copyright\x00",
    b"trademark\x00",
    b"acute\x00",
    b"dieresis\x00",
    b"notequal\x00",
    b"AE\x00",
    b"Oslash\x00",
    b"infinity\x00",
    b"plusminus\x00",
    b"lessequal\x00",
    b"greaterequal\x00",
    b"yen\x00",
    b"mu\x00",
    b"partialdiff\x00",
    b"summation\x00",
    b"product\x00",
    b"pi\x00",
    b"integral\x00",
    b"ordfeminine\x00",
    b"ordmasculine\x00",
    b"Omega\x00",
    b"ae\x00",
    b"oslash\x00",
    b"questiondown\x00",
    b"exclamdown\x00",
    b"logicalnot\x00",
    b"radical\x00",
    b"florin\x00",
    b"approxequal\x00",
    b"Delta\x00",
    b"guillemotleft\x00",
    b"guillemotright\x00",
    b"ellipsis\x00",
    b"nonbreakingspace\x00",
    b"Agrave\x00",
    b"Atilde\x00",
    b"Otilde\x00",
    b"OE\x00",
    b"oe\x00",
    b"endash\x00",
    b"emdash\x00",
    b"quotedblleft\x00",
    b"quotedblright\x00",
    b"quoteleft\x00",
    b"quoteright\x00",
    b"divide\x00",
    b"lozenge\x00",
    b"ydieresis\x00",
    b"Ydieresis\x00",
    b"fraction\x00",
    b"currency\x00",
    b"guilsinglleft\x00",
    b"guilsinglright\x00",
    b"fi\x00",
    b"fl\x00",
    b"daggerdbl\x00",
    b"periodcentered\x00",
    b"quotesinglbase\x00",
    b"quotedblbase\x00",
    b"perthousand\x00",
    b"Acircumflex\x00",
    b"Ecircumflex\x00",
    b"Aacute\x00",
    b"Edieresis\x00",
    b"Egrave\x00",
    b"Iacute\x00",
    b"Icircumflex\x00",
    b"Idieresis\x00",
    b"Igrave\x00",
    b"Oacute\x00",
    b"Ocircumflex\x00",
    b"apple\x00",
    b"Ograve\x00",
    b"Uacute\x00",
    b"Ucircumflex\x00",
    b"Ugrave\x00",
    b"dotlessi\x00",
    b"circumflex\x00",
    b"tilde\x00",
    b"macron\x00",
    b"breve\x00",
    b"dotaccent\x00",
    b"ring\x00",
    b"cedilla\x00",
    b"hungarumlaut\x00",
    b"ogonek\x00",
    b"caron\x00",
    b"Lslash\x00",
    b"lslash\x00",
    b"Scaron\x00",
    b"scaron\x00",
    b"Zcaron\x00",
    b"zcaron\x00",
    b"brokenbar\x00",
    b"Eth\x00",
    b"eth\x00",
    b"Yacute\x00",
    b"yacute\x00",
    b"Thorn\x00",
    b"thorn\x00",
    b"minus\x00",
    b"multiply\x00",
    b"onesuperior\x00",
    b"twosuperior\x00",
    b"threesuperior\x00",
    b"onehalf\x00",
    b"onequarter\x00",
    b"threequarters\x00",
    b"franc\x00",
    b"Gbreve\x00",
    b"gbreve\x00",
    b"Idotaccent\x00",
    b"Scedilla\x00",
    b"scedilla\x00",
    b"Cacute\x00",
    b"cacute\x00",
    b"Ccaron\x00",
    b"ccaron\x00",
    b"dcroat\x00",
];
