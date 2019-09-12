#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::stub_errno as errno;
use crate::warn;
use libc::free;

extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
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
    static mut work_buffer: [i8; 0];
    /* String */
    #[no_mangle]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut i8;
    #[no_mangle]
    fn cff_add_string(cff: *mut cff_font, str: *const i8, unique: i32) -> s_SID;
}
pub type rust_input_handle_t = *mut libc::c_void;
pub type card8 = u8;
pub type card16 = u16;
pub type c_offsize = u8;
pub type l_offset = u32;
pub type s_SID = u16;
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
/* 1-byte unsigned number */
/* 2-byte unsigned number */
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
/* 1, 2, 3, or 4-byte offset */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub opname: *const i8,
    pub argtype: i32,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    0 as *mut libc::c_void
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
#[no_mangle]
pub unsafe extern "C" fn cff_new_dict() -> *mut cff_dict {
    let mut dict: *mut cff_dict = 0 as *mut cff_dict;
    dict =
        new((1_u64).wrapping_mul(::std::mem::size_of::<cff_dict>() as u64) as u32) as *mut cff_dict;
    (*dict).max = 16i32;
    (*dict).count = 0i32;
    (*dict).entries = new(((*dict).max as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<cff_dict_entry>() as u64)
        as u32) as *mut cff_dict_entry;
    dict
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_dict(mut dict: *mut cff_dict) {
    if !dict.is_null() {
        if !(*dict).entries.is_null() {
            let mut i: i32 = 0;
            i = 0i32;
            while i < (*dict).count {
                free((*(*dict).entries.offset(i as isize)).values as *mut libc::c_void);
                i += 1
            }
            free((*dict).entries as *mut libc::c_void);
        }
        free(dict as *mut libc::c_void);
    };
}
static mut stack_top: i32 = 0i32;
static mut arg_stack: [f64; 64] = [0.; 64];
static mut dict_operator: [C2RustUnnamed_2; 61] = [
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"version\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"Notice\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FullName\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FamilyName\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"Weight\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FontBBox\x00" as *const u8 as *const i8,
            argtype: 1i32 << 4i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"BlueValues\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"OtherBlues\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FamilyBlues\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FamilyOtherBlues\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"StdHW\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"StdVW\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"UniqueID\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"XUID\x00" as *const u8 as *const i8,
            argtype: 1i32 << 4i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"charset\x00" as *const u8 as *const i8,
            argtype: 1i32 << 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"Encoding\x00" as *const u8 as *const i8,
            argtype: 1i32 << 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"CharStrings\x00" as *const u8 as *const i8,
            argtype: 1i32 << 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"Private\x00" as *const u8 as *const i8,
            argtype: 1i32 << 8i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"Subrs\x00" as *const u8 as *const i8,
            argtype: 1i32 << 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"defaultWidthX\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"nominalWidthX\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"Copyright\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"IsFixedPitch\x00" as *const u8 as *const i8,
            argtype: 1i32 << 2i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"ItalicAngle\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"UnderlinePosition\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"UnderlineThickness\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"PaintType\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"CharstringType\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FontMatrix\x00" as *const u8 as *const i8,
            argtype: 1i32 << 4i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"StrokeWidth\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"BlueScale\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"BlueShift\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"BlueFuzz\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"StemSnapH\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"StemSnapV\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"ForceBold\x00" as *const u8 as *const i8,
            argtype: 1i32 << 2i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"LanguageGroup\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"ExpansionFactor\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"InitialRandomSeed\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"SyntheticBase\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"PostScript\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"BaseFontName\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"BaseFontBlend\x00" as *const u8 as *const i8,
            argtype: 1i32 << 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: 0 as *const i8,
            argtype: -1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"ROS\x00" as *const u8 as *const i8,
            argtype: 1i32 << 6i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"CIDFontVersion\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"CIDFontRevision\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"CIDFontType\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"CIDCount\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"UIDBase\x00" as *const u8 as *const i8,
            argtype: 1i32 << 0i32 | 1i32 << 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FDArray\x00" as *const u8 as *const i8,
            argtype: 1i32 << 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FDSelect\x00" as *const u8 as *const i8,
            argtype: 1i32 << 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            opname: b"FontName\x00" as *const u8 as *const i8,
            argtype: 1i32 << 3i32,
        };
        init
    },
];
/* Parse DICT data */
unsafe extern "C" fn get_integer(
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut status: *mut i32,
) -> f64 {
    let mut result: i32 = 0i32;
    let mut b0: card8 = 0;
    let mut b1: card8 = 0;
    let mut b2: card8 = 0;
    let fresh0 = *data;
    *data = (*data).offset(1);
    b0 = *fresh0;
    if b0 as i32 == 28i32 && *data < endptr.offset(-2) {
        /* shortint */
        let fresh1 = *data;
        *data = (*data).offset(1);
        b1 = *fresh1;
        let fresh2 = *data;
        *data = (*data).offset(1);
        b2 = *fresh2;
        result = b1 as i32 * 256i32 + b2 as i32;
        if result as i64 > 0x7fff {
            result = (result as i64 - 0x10000) as i32
        }
    } else if b0 as i32 == 29i32 && *data < endptr.offset(-4) {
        /* longint */
        let mut i: i32 = 0;
        let fresh3 = *data;
        *data = (*data).offset(1);
        result = *fresh3 as i32;
        if result > 0x7fi32 {
            result -= 0x100i32
        }
        i = 0i32;
        while i < 3i32 {
            result = result * 256i32 + **data as i32;
            *data = (*data).offset(1);
            i += 1
        }
    } else if b0 as i32 >= 32i32 && b0 as i32 <= 246i32 {
        /* int (1) */
        result = b0 as i32 - 139i32
    } else if b0 as i32 >= 247i32 && b0 as i32 <= 250i32 {
        /* int (2) */
        let fresh4 = *data;
        *data = (*data).offset(1);
        b1 = *fresh4;
        result = (b0 as i32 - 247i32) * 256i32 + b1 as i32 + 108i32
    } else if b0 as i32 >= 251i32 && b0 as i32 <= 254i32 {
        let fresh5 = *data;
        *data = (*data).offset(1);
        b1 = *fresh5;
        result = -(b0 as i32 - 251i32) * 256i32 - b1 as i32 - 108i32
    } else {
        *status = -1i32
    }
    result as f64
}
/* Simply uses strtod */
unsafe extern "C" fn get_real(
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut status: *mut i32,
) -> f64 {
    let mut result: f64 = 0.0f64; /* skip first byte (30) */
    let mut nibble: i32 = 0i32;
    let mut pos: i32 = 0i32;
    let mut len: i32 = 0i32;
    let mut fail: i32 = 0i32;
    if **data as i32 != 30i32 || *data >= endptr.offset(-1) {
        *status = -1i32;
        return 0.0f64;
    }
    *data = (*data).offset(1);
    pos = 0i32;
    while fail == 0 && len < 1024i32 - 2i32 && *data < endptr {
        /* get nibble */
        if pos % 2i32 != 0 {
            nibble = **data as i32 & 0xfi32;
            *data = (*data).offset(1)
        } else {
            nibble = **data as i32 >> 4i32 & 0xfi32
        }
        if nibble >= 0i32 && nibble <= 0x9i32 {
            let fresh6 = len;
            len = len + 1;
            *work_buffer.as_mut_ptr().offset(fresh6 as isize) = (nibble + '0' as i32) as i8
        } else if nibble == 0xai32 {
            /* . */
            let fresh7 = len;
            len = len + 1;
            *work_buffer.as_mut_ptr().offset(fresh7 as isize) = '.' as i32 as i8
        } else if nibble == 0xbi32 || nibble == 0xci32 {
            /* E, E- */
            let fresh8 = len;
            len = len + 1;
            *work_buffer.as_mut_ptr().offset(fresh8 as isize) = 'e' as i32 as i8;
            if nibble == 0xci32 {
                let fresh9 = len;
                len = len + 1;
                *work_buffer.as_mut_ptr().offset(fresh9 as isize) = '-' as i32 as i8
            }
        } else if nibble == 0xei32 {
            /* `-' */
            let fresh10 = len; /* invalid */
            len = len + 1;
            *work_buffer.as_mut_ptr().offset(fresh10 as isize) = '-' as i32 as i8
        } else if !(nibble == 0xdi32) {
            if nibble == 0xfi32 {
                /* end */
                let fresh11 = len;
                len = len + 1;
                *work_buffer.as_mut_ptr().offset(fresh11 as isize) = '\u{0}' as i32 as i8;
                if pos % 2i32 == 0i32 && **data as i32 != 0xffi32 {
                    fail = 1i32
                }
                break;
            } else {
                fail = 1i32
            }
        }
        /* skip */
        /* do nothing */
        pos += 1
    }
    /* returned values */
    if fail != 0 || nibble != 0xfi32 {
        *status = -1i32
    } else {
        let mut s: *mut i8 = 0 as *mut i8;
        result = strtod(work_buffer.as_mut_ptr(), &mut s);
        if *s as i32 != 0i32 || errno::errno() == errno::ERANGE {
            *status = -1i32
        }
    }
    result
}
/* operators */
unsafe extern "C" fn add_dict(
    mut dict: *mut cff_dict,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut status: *mut i32,
) {
    let mut id: i32 = 0;
    let mut argtype: i32 = 0;
    id = **data as i32;
    if id == 0xci32 {
        *data = (*data).offset(1);
        if *data >= endptr || {
            id = **data as i32 + 22i32;
            id >= 22i32 + 39i32
        } {
            *status = -1i32;
            return;
        }
    } else if id >= 22i32 {
        *status = -1i32;
        return;
    }
    argtype = dict_operator[id as usize].argtype;
    if dict_operator[id as usize].opname.is_null() || argtype < 0i32 {
        /* YuppySC-Regular.otf from OS X for instance uses op id 37, simply ignore
        this dict instead of treat it as parsing error. */
        return;
    }
    if (*dict).count >= (*dict).max {
        (*dict).max += 16i32;
        (*dict).entries = renew(
            (*dict).entries as *mut libc::c_void,
            ((*dict).max as u32 as u64).wrapping_mul(::std::mem::size_of::<cff_dict_entry>() as u64)
                as u32,
        ) as *mut cff_dict_entry
    }
    (*(*dict).entries.offset((*dict).count as isize)).id = id;
    let ref mut fresh12 = (*(*dict).entries.offset((*dict).count as isize)).key;
    *fresh12 = dict_operator[id as usize].opname;
    if argtype == 1i32 << 0i32 | 1i32 << 1i32
        || argtype == 1i32 << 2i32
        || argtype == 1i32 << 3i32
        || argtype == 1i32 << 7i32
    {
        /* check for underflow here, as exactly one operand is expected */
        if stack_top < 1i32 {
            *status = -3i32;
            return;
        }
        stack_top -= 1;
        (*(*dict).entries.offset((*dict).count as isize)).count = 1i32;
        let ref mut fresh13 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh13 =
            new((1_u64).wrapping_mul(::std::mem::size_of::<f64>() as u64) as u32) as *mut f64;
        *(*(*dict).entries.offset((*dict).count as isize))
            .values
            .offset(0) = arg_stack[stack_top as usize];
        (*dict).count += 1i32
    } else if stack_top > 0i32 {
        (*(*dict).entries.offset((*dict).count as isize)).count = stack_top;
        let ref mut fresh14 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh14 =
            new((stack_top as u32 as u64).wrapping_mul(::std::mem::size_of::<f64>() as u64) as u32)
                as *mut f64;
        while stack_top > 0i32 {
            stack_top -= 1;
            *(*(*dict).entries.offset((*dict).count as isize))
                .values
                .offset(stack_top as isize) = arg_stack[stack_top as usize]
        }
        (*dict).count += 1i32
    }
    *data = (*data).offset(1);
}
/* just ignore operator if there were no operands provided;
don't treat this as underflow (e.g. StemSnapV in TemporaLGCUni-Italic.otf) */
/*
 * All operands are treated as number or array of numbers.
 *  Private: two numbers, size and offset
 *  ROS    : three numbers, SID, SID, and a number
 */
#[no_mangle]
pub unsafe extern "C" fn cff_dict_unpack(
    mut data: *mut card8,
    mut endptr: *mut card8,
) -> *mut cff_dict {
    let mut dict: *mut cff_dict = 0 as *mut cff_dict;
    let mut status: i32 = 0i32;
    stack_top = 0i32;
    dict = cff_new_dict();
    while data < endptr && status == 0i32 {
        if (*data as i32) < 22i32 {
            /* operator */
            add_dict(dict, &mut data, endptr, &mut status);
        } else if *data as i32 == 30i32 {
            /* real - First byte of a sequence (variable) */
            if stack_top < 64i32 {
                arg_stack[stack_top as usize] = get_real(&mut data, endptr, &mut status); /* everything else are integer */
                stack_top += 1
            } else {
                status = -2i32
            }
        } else if *data as i32 == 255i32 || *data as i32 >= 22i32 && *data as i32 <= 27i32 {
            /* reserved */
            data = data.offset(1)
        } else if stack_top < 64i32 {
            arg_stack[stack_top as usize] = get_integer(&mut data, endptr, &mut status);
            stack_top += 1
        } else {
            status = -2i32
        }
    }
    if status != 0i32 {
        _tt_abort(
            b"%s: Parsing CFF DICT failed. (error=%d)\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
            status,
        );
    } else {
        if stack_top != 0i32 {
            warn!("{}: Garbage in CFF DICT data.", "CFF");
            stack_top = 0i32
        }
    }
    dict
}
/* Pack DICT data */
unsafe extern "C" fn pack_integer(mut dest: *mut card8, mut destlen: i32, mut value: i32) -> i32 {
    let mut len: i32 = 0i32; /* longint */
    if value >= -107i32 && value <= 107i32 {
        if destlen < 1i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        *dest.offset(0) = (value + 139i32 & 0xffi32) as card8;
        len = 1i32
    } else if value >= 108i32 && value <= 1131i32 {
        if destlen < 2i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        value = 0xf700u32.wrapping_add(value as u32).wrapping_sub(108_u32) as i32;
        *dest.offset(0) = (value >> 8i32 & 0xffi32) as card8;
        *dest.offset(1) = (value & 0xffi32) as card8;
        len = 2i32
    } else if value >= -1131i32 && value <= -108i32 {
        if destlen < 2i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        value = 0xfb00u32.wrapping_sub(value as u32).wrapping_sub(108_u32) as i32;
        *dest.offset(0) = (value >> 8i32 & 0xffi32) as card8;
        *dest.offset(1) = (value & 0xffi32) as card8;
        len = 2i32
    } else if value >= -32768i32 && value <= 32767i32 {
        /* shortint */
        if destlen < 3i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        *dest.offset(0) = 28i32 as card8;
        *dest.offset(1) = (value >> 8i32 & 0xffi32) as card8;
        *dest.offset(2) = (value & 0xffi32) as card8;
        len = 3i32
    } else {
        if destlen < 5i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        *dest.offset(0) = 29i32 as card8;
        *dest.offset(1) = (value >> 24i32 & 0xffi32) as card8;
        *dest.offset(2) = (value >> 16i32 & 0xffi32) as card8;
        *dest.offset(3) = (value >> 8i32 & 0xffi32) as card8;
        *dest.offset(4) = (value & 0xffi32) as card8;
        len = 5i32
    }
    len
}
unsafe extern "C" fn pack_real(mut dest: *mut card8, mut destlen: i32, mut value: f64) -> i32 {
    let mut i: i32 = 0i32;
    let mut pos: i32 = 2i32;
    let mut buffer: [i8; 32] = [0; 32];
    if destlen < 2i32 {
        _tt_abort(
            b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
        );
    }
    *dest.offset(0) = 30i32 as card8;
    if value == 0.0f64 {
        *dest.offset(1) = 0xfi32 as card8;
        return 2i32;
    }
    if value < 0.0f64 {
        *dest.offset(1) = 0xe0i32 as card8;
        value *= -1.0f64;
        pos += 1
    }
    /* To avoid the problem with Mac OS X 10.4 Quartz,
     * change the presion of the real numbers
     * on June 27, 2007 for musix20.pfb */
    sprintf(
        buffer.as_mut_ptr(),
        b"%.13g\x00" as *const u8 as *const i8,
        value,
    );
    i = 0i32;
    while buffer[i as usize] as i32 != '\u{0}' as i32 {
        let mut ch: u8 = 0_u8;
        if buffer[i as usize] as i32 == '.' as i32 {
            ch = 0xa_u8
        } else if buffer[i as usize] as i32 >= '0' as i32 && buffer[i as usize] as i32 <= '9' as i32
        {
            ch = (buffer[i as usize] as i32 - '0' as i32) as u8
        } else if buffer[i as usize] as i32 == 'e' as i32 {
            i += 1;
            ch = (if buffer[i as usize] as i32 == '-' as i32 {
                0xci32
            } else {
                0xbi32
            }) as u8
        } else {
            _tt_abort(
                b"%s: Invalid character.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        if destlen < pos / 2i32 + 1i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        if pos % 2i32 != 0 {
            let ref mut fresh15 = *dest.offset((pos / 2i32) as isize);
            *fresh15 = (*fresh15 as i32 + ch as i32) as card8
        } else {
            *dest.offset((pos / 2i32) as isize) = ((ch as i32) << 4i32) as card8
        }
        pos += 1;
        i += 1
    }
    if pos % 2i32 != 0 {
        let ref mut fresh16 = *dest.offset((pos / 2i32) as isize);
        *fresh16 = (*fresh16 as i32 + 0xfi32) as card8;
        pos += 1
    } else {
        if destlen < pos / 2i32 + 1i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        *dest.offset((pos / 2i32) as isize) = 0xffi32 as card8;
        pos += 2i32
    }
    pos / 2i32
}
unsafe extern "C" fn cff_dict_put_number(
    mut value: f64,
    mut dest: *mut card8,
    mut destlen: i32,
    mut type_0: i32,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut nearint: f64 = 0.;
    nearint = (value + 0.5f64).floor();
    /* set offset to longint */
    if type_0 == 1i32 << 7i32 {
        let mut lvalue: i32 = 0; /* integer */
        lvalue = value as i32;
        if destlen < 5i32 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
        *dest.offset(0) = 29i32 as card8;
        *dest.offset(1) = (lvalue >> 24i32 & 0xffi32) as card8;
        *dest.offset(2) = (lvalue >> 16i32 & 0xffi32) as card8;
        *dest.offset(3) = (lvalue >> 8i32 & 0xffi32) as card8;
        *dest.offset(4) = (lvalue & 0xffi32) as card8;
        len = 5i32
    } else if value > 0x7fffffffi32 as f64
        || value < (-0x7fffffffi32 - 1i32) as f64
        || (value - nearint).abs() > 1.0e-5f64
    {
        /* real */
        len = pack_real(dest, destlen, value)
    } else {
        len = pack_integer(dest, destlen, nearint as i32)
    }
    len
}
unsafe extern "C" fn put_dict_entry(
    mut de: *mut cff_dict_entry,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut i: i32 = 0;
    let mut type_0: i32 = 0;
    let mut id: i32 = 0;
    if (*de).count > 0i32 {
        id = (*de).id;
        if dict_operator[id as usize].argtype == 1i32 << 7i32
            || dict_operator[id as usize].argtype == 1i32 << 8i32
        {
            type_0 = 1i32 << 7i32
        } else {
            type_0 = 1i32 << 0i32 | 1i32 << 1i32
        }
        i = 0i32;
        while i < (*de).count {
            len += cff_dict_put_number(
                *(*de).values.offset(i as isize),
                dest.offset(len as isize),
                destlen - len,
                type_0,
            );
            i += 1
        }
        if id >= 0i32 && id < 22i32 {
            if len + 1i32 > destlen {
                _tt_abort(
                    b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                    b"CFF\x00" as *const u8 as *const i8,
                );
            }
            let fresh17 = len;
            len = len + 1;
            *dest.offset(fresh17 as isize) = id as card8
        } else if id >= 0i32 && id < 22i32 + 39i32 {
            if len + 2i32 > destlen {
                _tt_abort(b"in cff_dict_pack(): Buffer overflow\x00" as *const u8 as *const i8);
            }
            let fresh18 = len;
            len = len + 1;
            *dest.offset(fresh18 as isize) = 12i32 as card8;
            let fresh19 = len;
            len = len + 1;
            *dest.offset(fresh19 as isize) = (id - 22i32) as card8
        } else {
            _tt_abort(
                b"%s: Invalid CFF DICT operator ID.\x00" as *const u8 as *const i8,
                b"CFF\x00" as *const u8 as *const i8,
            );
        }
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_dict_pack(
    mut dict: *mut cff_dict,
    mut dest: *mut card8,
    mut destlen: i32,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*dict).count {
        if streq_ptr(
            (*(*dict).entries.offset(i as isize)).key,
            b"ROS\x00" as *const u8 as *const i8,
        ) {
            len += put_dict_entry(&mut *(*dict).entries.offset(i as isize), dest, destlen);
            break;
        } else {
            i += 1
        }
    }
    i = 0i32;
    while i < (*dict).count {
        if strcmp(
            (*(*dict).entries.offset(i as isize)).key,
            b"ROS\x00" as *const u8 as *const i8,
        ) != 0
        {
            len += put_dict_entry(
                &mut *(*dict).entries.offset(i as isize),
                dest.offset(len as isize),
                destlen - len,
            )
        }
        i += 1
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_dict_add(mut dict: *mut cff_dict, mut key: *const i8, mut count: i32) {
    let mut id: i32 = 0;
    let mut i: i32 = 0;
    id = 0i32;
    while id < 22i32 + 39i32 {
        if !key.is_null()
            && !dict_operator[id as usize].opname.is_null()
            && streq_ptr(dict_operator[id as usize].opname, key) as i32 != 0
        {
            break;
        }
        id += 1
    }
    if id == 22i32 + 39i32 {
        _tt_abort(
            b"%s: Unknown CFF DICT operator.\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
        );
    }
    i = 0i32;
    while i < (*dict).count {
        if (*(*dict).entries.offset(i as isize)).id == id {
            if (*(*dict).entries.offset(i as isize)).count != count {
                _tt_abort(
                    b"%s: Inconsistent DICT argument number.\x00" as *const u8 as *const i8,
                    b"CFF\x00" as *const u8 as *const i8,
                );
            }
            return;
        }
        i += 1
    }
    if (*dict).count + 1i32 >= (*dict).max {
        (*dict).max += 8i32;
        (*dict).entries = renew(
            (*dict).entries as *mut libc::c_void,
            ((*dict).max as u32 as u64).wrapping_mul(::std::mem::size_of::<cff_dict_entry>() as u64)
                as u32,
        ) as *mut cff_dict_entry
    }
    (*(*dict).entries.offset((*dict).count as isize)).id = id;
    let ref mut fresh20 = (*(*dict).entries.offset((*dict).count as isize)).key;
    *fresh20 = dict_operator[id as usize].opname;
    (*(*dict).entries.offset((*dict).count as isize)).count = count;
    if count > 0i32 {
        let ref mut fresh21 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh21 =
            new((count as u32 as u64).wrapping_mul(::std::mem::size_of::<f64>() as u64) as u32)
                as *mut f64;
        memset(
            (*(*dict).entries.offset((*dict).count as isize)).values as *mut libc::c_void,
            0i32,
            (::std::mem::size_of::<f64>() as u64).wrapping_mul(count as u64),
        );
    } else {
        let ref mut fresh22 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh22 = 0 as *mut f64
    }
    (*dict).count += 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn cff_dict_remove(mut dict: *mut cff_dict, mut key: *const i8) {
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) {
            (*(*dict).entries.offset(i as isize)).count = 0i32;
            let ref mut fresh23 = (*(*dict).entries.offset(i as isize)).values;
            *fresh23 =
                mfree((*(*dict).entries.offset(i as isize)).values as *mut libc::c_void) as *mut f64
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn cff_dict_known(mut dict: *mut cff_dict, mut key: *const i8) -> i32 {
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) as i32 != 0
            && (*(*dict).entries.offset(i as isize)).count > 0i32
        {
            return 1i32;
        }
        i += 1
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn cff_dict_get(
    mut dict: *mut cff_dict,
    mut key: *const i8,
    mut idx: i32,
) -> f64 {
    let mut value: f64 = 0.0f64;
    let mut i: i32 = 0;
    assert!(!key.is_null() && !dict.is_null());
    i = 0i32;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) {
            if (*(*dict).entries.offset(i as isize)).count > idx {
                value = *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(idx as isize)
            } else {
                _tt_abort(
                    b"%s: Invalid index number.\x00" as *const u8 as *const i8,
                    b"CFF\x00" as *const u8 as *const i8,
                );
            }
            break;
        } else {
            i += 1
        }
    }
    if i == (*dict).count {
        _tt_abort(
            b"%s: DICT entry \"%s\" not found.\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
            key,
        );
    }
    value
}
#[no_mangle]
pub unsafe extern "C" fn cff_dict_set(
    mut dict: *mut cff_dict,
    mut key: *const i8,
    mut idx: i32,
    mut value: f64,
) {
    let mut i: i32 = 0;
    assert!(!dict.is_null() && !key.is_null());
    i = 0i32;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) {
            if (*(*dict).entries.offset(i as isize)).count > idx {
                *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(idx as isize) = value
            } else {
                _tt_abort(
                    b"%s: Invalid index number.\x00" as *const u8 as *const i8,
                    b"CFF\x00" as *const u8 as *const i8,
                );
            }
            break;
        } else {
            i += 1
        }
    }
    if i == (*dict).count {
        _tt_abort(
            b"%s: DICT entry \"%s\" not found.\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
            key,
        );
    };
}
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
/* decode/encode DICT */
#[no_mangle]
pub unsafe extern "C" fn cff_dict_update(mut dict: *mut cff_dict, mut cff: *mut cff_font) {
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*dict).count {
        if (*(*dict).entries.offset(i as isize)).count > 0i32 {
            let mut str: *mut i8 = 0 as *mut i8;
            let mut id: i32 = 0;
            id = (*(*dict).entries.offset(i as isize)).id;
            if dict_operator[id as usize].argtype == 1i32 << 3i32 {
                str = cff_get_string(
                    cff,
                    *(*(*dict).entries.offset(i as isize)).values.offset(0) as s_SID,
                );
                *(*(*dict).entries.offset(i as isize)).values.offset(0) =
                    cff_add_string(cff, str, 1i32) as f64;
                free(str as *mut libc::c_void);
            } else if dict_operator[id as usize].argtype == 1i32 << 6i32 {
                str = cff_get_string(
                    cff,
                    *(*(*dict).entries.offset(i as isize)).values.offset(0) as s_SID,
                );
                *(*(*dict).entries.offset(i as isize)).values.offset(0) =
                    cff_add_string(cff, str, 1i32) as f64;
                free(str as *mut libc::c_void);
                str = cff_get_string(
                    cff,
                    *(*(*dict).entries.offset(i as isize)).values.offset(1) as s_SID,
                );
                *(*(*dict).entries.offset(i as isize)).values.offset(1) =
                    cff_add_string(cff, str, 1i32) as f64;
                free(str as *mut libc::c_void);
            }
        }
        i += 1
    }
}
