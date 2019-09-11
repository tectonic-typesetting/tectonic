#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    pub type pdf_obj;
    pub type Type0Font;
    pub type otl_gsub;
    /* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    /* Both XeTeX and bibtex use this enum: */
    /* The weird enum values are historical and could be rationalized. But it is
     * good to write them explicitly since they must be kept in sync with
     * `src/engines/mod.rs`.
     */
    /* quasi-hack to get the primary input */
    /* Bridge API. Keep synchronized with src/engines/mod.rs. */
    /* These functions are not meant to be used in the C/C++ code. They define the
     * API that we expose to the Rust side of things. */
    /* The internal, C/C++ interface: */
    /* Global symbols that route through the global API variable. Hopefully we
     * will one day eliminate all of the global state and get rid of all of
     * these. */
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn dpx_open_truetype_file(filename: *const i8) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_dfont_file(filename: *const i8) -> rust_input_handle_t;
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
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: f64) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
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
    /* pdf_open_document() call them. */
    /* font_name is used when mrec is NULL.
     * font_scale (point size) used by PK font.
     * It might be necessary if dvipdfmx supports font format with
     * various optical sizes supported in the future.
     */
    /* Each font drivers use the followings. */
    /* without unique tag */
    #[no_mangle]
    fn pdf_font_make_uniqueTag(tag: *mut i8);
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn pdf_new_stream(flags: i32) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: i32,
    );
    #[no_mangle]
    fn pdf_stream_length(stream: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn Type0Font_get_usedchars(font: *mut Type0Font) -> *mut i8;
    #[no_mangle]
    fn Type0Font_cache_get(id: i32) -> *mut Type0Font;
    #[no_mangle]
    fn CIDFont_get_embedding(font: *mut CIDFont) -> i32;
    #[no_mangle]
    fn CIDFont_get_parent_id(font: *mut CIDFont, wmode: i32) -> i32;
    #[no_mangle]
    fn CIDFont_is_BaseFont(font: *mut CIDFont) -> bool;
    #[no_mangle]
    fn CMap_decode_char(
        cmap: *mut CMap,
        inbuf: *mut *const u8,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut u8,
        outbytesleft: *mut size_t,
    );
    #[no_mangle]
    fn CMap_cache_get(id: i32) -> *mut CMap;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const i8) -> i32;
    #[no_mangle]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[no_mangle]
    fn dfont_open(handle: rust_input_handle_t, index: i32) -> *mut sfnt;
    #[no_mangle]
    fn sfnt_close(sfont: *mut sfnt);
    /* table directory */
    #[no_mangle]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: u32) -> i32;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> u32;
    #[no_mangle]
    fn sfnt_require_table(sfont: *mut sfnt, tag: *const i8, must_exist: i32) -> i32;
    #[no_mangle]
    fn sfnt_create_FontFile_stream(sfont: *mut sfnt) -> *mut pdf_obj;
    /* TTC (TrueType Collection) */
    #[no_mangle]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: i32) -> u32;
    /* FontDescriptor */
    #[no_mangle]
    fn tt_get_fontdesc(
        sfont: *mut sfnt,
        embed: *mut i32,
        stemv: i32,
        type_0: i32,
        fontname: *const i8,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn tt_cmap_read(sfont: *mut sfnt, platform: u16, encoding: u16) -> *mut tt_cmap;
    #[no_mangle]
    fn tt_cmap_lookup(cmap: *mut tt_cmap, cc: u32) -> u16;
    #[no_mangle]
    fn tt_cmap_release(cmap: *mut tt_cmap);
    #[no_mangle]
    fn tt_build_init() -> *mut tt_glyphs;
    #[no_mangle]
    fn tt_build_finish(g: *mut tt_glyphs);
    #[no_mangle]
    fn tt_add_glyph(g: *mut tt_glyphs, gid: u16, new_gid: u16) -> u16;
    #[no_mangle]
    fn tt_get_index(g: *mut tt_glyphs, gid: u16) -> u16;
    #[no_mangle]
    fn tt_build_tables(sfont: *mut sfnt, g: *mut tt_glyphs) -> i32;
    #[no_mangle]
    fn tt_get_metrics(sfont: *mut sfnt, g: *mut tt_glyphs) -> i32;
    /* LookupType for GSUB */
    #[no_mangle]
    fn otl_gsub_new() -> *mut otl_gsub;
    #[no_mangle]
    fn otl_gsub_release(gsub_list: *mut otl_gsub);
    #[no_mangle]
    fn otl_gsub_select(
        gsub_list: *mut otl_gsub,
        script: *const i8,
        language: *const i8,
        feature: *const i8,
    ) -> i32;
    #[no_mangle]
    fn otl_gsub_add_feat(
        gsub_list: *mut otl_gsub,
        script: *const i8,
        language: *const i8,
        feature: *const i8,
        sfont: *mut sfnt,
    ) -> i32;
    #[no_mangle]
    fn otl_gsub_apply(gsub_list: *mut otl_gsub, gid: *mut u16) -> i32;
    /* name table */
    #[no_mangle]
    fn tt_get_ps_fontname(sfont: *mut sfnt, dest: *mut i8, destlen: u16) -> u16;
}
pub type size_t = u64;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDFont {
    pub ident: *mut i8,
    pub name: *mut i8,
    pub fontname: *mut i8,
    pub subtype: i32,
    pub flags: i32,
    pub parent: [i32; 2],
    pub csi: *mut CIDSysInfo,
    pub options: *mut cid_opt,
    pub indirect: *mut pdf_obj,
    pub fontdict: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cid_opt {
    pub name: *mut i8,
    pub csi: *mut CIDSysInfo,
    pub index: i32,
    pub style: i32,
    pub embed: i32,
    pub stemv: i32,
    pub cff_charsets: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_cmap {
    pub format: u16,
    pub platform: u16,
    pub encoding: u16,
    pub language: u32,
    pub map: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut i8,
    pub type_0: i32,
    pub wmode: i32,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: i32,
    pub profile: C2RustUnnamed,
    pub reverseMap: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
/* 2 for CID, variable for Code..  */
/* CID (as 16-bit BE), Code ...    */
/* Next Subtbl for LOOKUP_CONTINUE */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: i32,
    /* Position of next free data segment */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: i32,
    pub len: size_t,
    pub code: *mut u8,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: u32,
    pub max: u32,
    pub ranges: *mut rangeDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut u8,
    pub codeHi: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: i32,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub num_kept_tables: u16,
    pub flags: *mut i8,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut i8,
}
pub type CID = u16;
/*
 * PDF viewer applications use following tables (CIDFontType 2)
 *
 *  head, hhea, loca, maxp, glyf, hmtx, fpgm, cvt_, prep
 *
 *                                         - from PDF Ref. v.1.3, 2nd ed.
 *
 * The fpgm, cvt_, and prep tables appears only when TrueType instructions
 * requires them. Those tables must be preserved if they exist.
 * We use must_exist flag to indicate `preserve it if present'
 * and to make sure not to cause an error when it does not exist.
 *
 * post and name table must exist in ordinary TrueType font file,
 * but when a TrueType font is converted to CIDFontType 2 font, those tables
 * are no longer required.
 *
 * The OS/2 table (required for TrueType font for Windows and OS/2) contains
 * liscencing information, but PDF viewers seems not using them.
 *
 * The 'name' table added. See comments in ttf.c.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub name: *const i8,
    pub must_exist: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyphs {
    pub num_glyphs: u16,
    pub max_glyphs: u16,
    pub last_gid: u16,
    pub emsize: u16,
    pub dw: u16,
    pub default_advh: u16,
    pub default_tsb: i16,
    pub gd: *mut tt_glyph_desc,
    pub used_slot: *mut u8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyph_desc {
    pub gid: u16,
    pub ogid: u16,
    pub advw: u16,
    pub advh: u16,
    pub lsb: i16,
    pub tsb: i16,
    pub llx: i16,
    pub lly: i16,
    pub urx: i16,
    pub ury: i16,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub alt1: u16,
    pub alt2: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub platform: u16,
    pub encoding: u16,
    pub pdfnames: [*const i8; 5],
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
/*
 * TrueType glyf table is sorted by CID and no CIDToGIDMap is used here.
 * GhostScript can't handle CIDToGIDMap correctly.
 */
/* pseudo unique tag */
/* CID font */
/* TrueType */
static mut verbose: i32 = 0i32;
static mut opt_flags: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type2_set_verbose(mut level: i32) {
    verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type2_set_flags(mut flags: i32) {
    opt_flags = flags;
}
static mut required_table: [C2RustUnnamed_1; 12] = [
    {
        let mut init = C2RustUnnamed_1 {
            name: b"OS/2\x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"head\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"hhea\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"loca\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"maxp\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"name\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"glyf\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"hmtx\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"fpgm\x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"cvt \x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: b"prep\x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            name: 0 as *const i8,
            must_exist: 0i32,
        };
        init
    },
];
unsafe extern "C" fn validate_name(mut fontname: *mut i8, mut len: i32) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    static mut badstrlist: [*const i8; 5] = [
        b"-WIN-RKSJ-H\x00" as *const u8 as *const i8,
        b"-WINP-RKSJ-H\x00" as *const u8 as *const i8,
        b"-WING-RKSJ-H\x00" as *const u8 as *const i8,
        b"-90pv-RKSJ-H\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    count = 0i32;
    i = 0i32;
    while i < len {
        if *fontname.offset(i as isize) as i32 == 0i32 {
            memmove(
                fontname.offset(i as isize) as *mut libc::c_void,
                fontname.offset(i as isize).offset(1) as *const libc::c_void,
                (len - i) as u64,
            );
            count += 1;
            len -= 1
        }
        i += 1
    }
    if count > 0i32 {
        dpx_warning(
            b"Removed %d null character(s) from fontname --> %s\x00" as *const u8 as *const i8,
            count,
            fontname,
        );
    }
    *fontname.offset(len as isize) = '\u{0}' as i32 as i8;
    /* For some fonts that have bad PS name. ad hoc. remove me. */
    i = 0i32;
    while !badstrlist[i as usize].is_null() {
        p = strstr(fontname, badstrlist[i as usize]);
        if !p.is_null() && p > fontname {
            dpx_warning(
                b"Removed string \"%s\" from fontname \"%s\".\x00" as *const u8 as *const i8,
                badstrlist[i as usize],
                fontname,
            );
            *p.offset(0) = '\u{0}' as i32 as i8;
            len = p.wrapping_offset_from(fontname) as i64 as i32;
            break;
        } else {
            i += 1
        }
    }
    if len < 1i32 {
        _tt_abort(b"No valid character found in fontname string.\x00" as *const u8 as *const i8);
    };
}
static mut known_encodings: [C2RustUnnamed_3; 11] = [
    {
        let mut init = C2RustUnnamed_3 {
            platform: 3_u16,
            encoding: 10_u16,
            pdfnames: [
                b"UCSms-UCS4\x00" as *const u8 as *const i8,
                b"UCSms-UCS2\x00" as *const u8 as *const i8,
                b"UCS4\x00" as *const u8 as *const i8,
                b"UCS2\x00" as *const u8 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 3_u16,
            encoding: 1_u16,
            pdfnames: [
                b"UCSms-UCS4\x00" as *const u8 as *const i8,
                b"UCSms-UCS2\x00" as *const u8 as *const i8,
                b"UCS4\x00" as *const u8 as *const i8,
                b"UCS2\x00" as *const u8 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 3_u16,
            encoding: 2_u16,
            pdfnames: [
                b"90ms-RKSJ\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 3_u16,
            encoding: 3_u16,
            pdfnames: [
                b"GBK-EUC\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 3_u16,
            encoding: 4_u16,
            pdfnames: [
                b"ETen-B5\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 3_u16,
            encoding: 5_u16,
            pdfnames: [
                b"KSCms-UHC\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 1_u16,
            encoding: 1_u16,
            pdfnames: [
                b"90pv-RKSJ\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 1_u16,
            encoding: 2_u16,
            pdfnames: [
                b"B5pc\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 1_u16,
            encoding: 25_u16,
            pdfnames: [
                b"GBpc-EUC\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 1_u16,
            encoding: 3_u16,
            pdfnames: [
                b"KSCpc-EUC\x00" as *const u8 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            platform: 0_u16,
            encoding: 0_u16,
            pdfnames: [
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
            ],
        };
        init
    },
];
unsafe extern "C" fn find_tocode_cmap(
    mut reg: *const i8,
    mut ord: *const i8,
    mut select: i32,
) -> *mut CMap {
    let mut cmap_id: i32 = -1i32;
    let mut i: i32 = 0;
    let mut cmap_name: *mut i8 = 0 as *mut i8;
    let mut append: *const i8 = 0 as *const i8;
    if reg.is_null() || ord.is_null() || select < 0i32 || select > 9i32 {
        _tt_abort(b"Character set unknown.\x00" as *const u8 as *const i8);
    }
    if streq_ptr(ord, b"UCS\x00" as *const u8 as *const i8) as i32 != 0 && select <= 1i32 {
        return 0 as *mut CMap;
    }
    i = 0i32;
    while cmap_id < 0i32 && i < 5i32 {
        append = known_encodings[select as usize].pdfnames[i as usize];
        if append.is_null() {
            break;
        }
        cmap_name = new((strlen(reg)
            .wrapping_add(strlen(ord))
            .wrapping_add(strlen(append))
            .wrapping_add(3i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        sprintf(
            cmap_name,
            b"%s-%s-%s\x00" as *const u8 as *const i8,
            reg,
            ord,
            append,
        );
        cmap_id = CMap_cache_find(cmap_name);
        free(cmap_name as *mut libc::c_void);
        i += 1
    }
    if cmap_id < 0i32 {
        dpx_warning(
            b"Could not find CID-to-Code mapping for \"%s-%s\".\x00" as *const u8 as *const i8,
            reg,
            ord,
        );
        dpx_warning(
            b"I tried to load (one of) the following file(s):\x00" as *const u8 as *const i8,
        );
        i = 0i32;
        while i < 5i32 {
            append = known_encodings[select as usize].pdfnames[i as usize];
            if append.is_null() {
                break;
            }
            dpx_message(b" %s-%s-%s\x00" as *const u8 as *const i8, reg, ord, append);
            i += 1
        }
        dpx_warning(b"Please check if this file exists.\x00" as *const u8 as *const i8);
        _tt_abort(b"Cannot continue...\x00" as *const u8 as *const i8);
    }
    CMap_cache_get(cmap_id)
}
/*
 * CIDFont glyph metrics:
 * Mostly same as add_CID[HV]Metrics in cidtype0.c.
 */
unsafe extern "C" fn add_TTCIDHMetrics(
    mut fontdict: *mut pdf_obj,
    mut g: *mut tt_glyphs,
    mut used_chars: *mut i8,
    mut cidtogidmap: *mut u8,
    mut last_cid: u16,
) {
    let mut cid: i32 = 0;
    let mut start: i32 = 0i32;
    let mut prev: i32 = 0i32;
    let mut w_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut an_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dw: f64 = 0.;
    let mut empty: i32 = 1i32;
    w_array = pdf_new_array();
    if (*g).dw as i32 != 0i32 && (*g).dw as i32 <= (*g).emsize as i32 {
        dw = (1000.0f64 * (*g).dw as i32 as f64 / (*g).emsize as i32 as f64 / 1i32 as f64 + 0.5f64)
            .floor()
            * 1i32 as f64
    } else {
        dw = (1000.0f64 * (*(*g).gd.offset(0)).advw as i32 as f64
            / (*g).emsize as i32 as f64
            / 1i32 as f64
            + 0.5f64)
            .floor()
            * 1i32 as f64
    }
    cid = 0i32;
    while cid <= last_cid as i32 {
        let mut idx: u16 = 0;
        let mut gid: u16 = 0;
        let mut width: f64 = 0.;
        if !(*used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 == 0) {
            gid = (if !cidtogidmap.is_null() {
                (*cidtogidmap.offset((2i32 * cid) as isize) as i32) << 8i32
                    | *cidtogidmap.offset((2i32 * cid + 1i32) as isize) as i32
            } else {
                cid
            }) as u16;
            idx = tt_get_index(g, gid);
            if !(cid != 0i32 && idx as i32 == 0i32) {
                width = (1000.0f64 * (*(*g).gd.offset(idx as isize)).advw as i32 as f64
                    / (*g).emsize as i32 as f64
                    / 1i32 as f64
                    + 0.5f64)
                    .floor()
                    * 1i32 as f64;
                if width == dw {
                    if !an_array.is_null() {
                        pdf_add_array(w_array, pdf_new_number(start as f64));
                        pdf_add_array(w_array, an_array);
                        an_array = 0 as *mut pdf_obj;
                        empty = 0i32
                    }
                } else {
                    if cid != prev + 1i32 {
                        if !an_array.is_null() {
                            pdf_add_array(w_array, pdf_new_number(start as f64));
                            pdf_add_array(w_array, an_array);
                            an_array = 0 as *mut pdf_obj;
                            empty = 0i32
                        }
                    }
                    if an_array.is_null() {
                        an_array = pdf_new_array();
                        start = cid
                    }
                    pdf_add_array(an_array, pdf_new_number(width));
                    prev = cid
                }
            }
        }
        cid += 1
    }
    if !an_array.is_null() {
        pdf_add_array(w_array, pdf_new_number(start as f64));
        pdf_add_array(w_array, an_array);
        empty = 0i32
    }
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"DW\x00" as *const u8 as *const i8),
        pdf_new_number(dw),
    );
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W\x00" as *const u8 as *const i8),
            pdf_ref_obj(w_array),
        );
    }
    pdf_release_obj(w_array);
}
unsafe extern "C" fn add_TTCIDVMetrics(
    mut fontdict: *mut pdf_obj,
    mut g: *mut tt_glyphs,
    mut used_chars: *mut i8,
    mut last_cid: u16,
) {
    let mut w2_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut an_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cid: i32 = 0;
    let mut defaultVertOriginY: f64 = 0.;
    let mut defaultAdvanceHeight: f64 = 0.;
    let mut empty: i32 = 1i32;
    defaultVertOriginY = (1000.0f64 * ((*g).default_advh as i32 - (*g).default_tsb as i32) as f64
        / (*g).emsize as i32 as f64
        / 1i32 as f64
        + 0.5f64)
        .floor()
        * 1i32 as f64;
    defaultAdvanceHeight =
        (1000.0f64 * (*g).default_advh as i32 as f64 / (*g).emsize as i32 as f64 / 1i32 as f64
            + 0.5f64)
            .floor()
            * 1i32 as f64;
    w2_array = pdf_new_array();
    cid = 0i32;
    while cid <= last_cid as i32 {
        let mut idx: u16 = 0;
        let mut vertOriginX: f64 = 0.;
        let mut vertOriginY: f64 = 0.;
        let mut advanceHeight: f64 = 0.;
        if !(*used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 == 0) {
            idx = tt_get_index(g, cid as u16);
            if !(cid != 0i32 && idx as i32 == 0i32) {
                advanceHeight = (1000.0f64 * (*(*g).gd.offset(idx as isize)).advh as i32 as f64
                    / (*g).emsize as i32 as f64
                    / 1i32 as f64
                    + 0.5f64)
                    .floor()
                    * 1i32 as f64;
                vertOriginX = (1000.0f64
                    * (0.5f64 * (*(*g).gd.offset(idx as isize)).advw as i32 as f64)
                    / (*g).emsize as i32 as f64
                    / 1i32 as f64
                    + 0.5f64)
                    .floor()
                    * 1i32 as f64;
                vertOriginY = (1000.0f64
                    * ((*(*g).gd.offset(idx as isize)).tsb as i32
                        + (*(*g).gd.offset(idx as isize)).ury as i32) as f64
                    / (*g).emsize as i32 as f64
                    / 1i32 as f64
                    + 0.5f64)
                    .floor()
                    * 1i32 as f64;
                /*
                 * c_first c_last w1_y v_x v_y
                 * This form may hit Acrobat's implementation limit of array element size,
                 * 8192. AFPL GhostScript 8.11 stops with rangecheck error with this.
                 * Maybe GS's bug?
                 */
                if vertOriginY != defaultVertOriginY || advanceHeight != defaultAdvanceHeight {
                    pdf_add_array(w2_array, pdf_new_number(cid as f64));
                    pdf_add_array(w2_array, pdf_new_number(cid as f64));
                    pdf_add_array(w2_array, pdf_new_number(-advanceHeight));
                    pdf_add_array(w2_array, pdf_new_number(vertOriginX));
                    pdf_add_array(w2_array, pdf_new_number(vertOriginY));
                    empty = 0i32
                }
            }
        }
        cid += 1
    }
    if defaultVertOriginY != 880i32 as f64 || defaultAdvanceHeight != 1000i32 as f64 {
        an_array = pdf_new_array();
        pdf_add_array(an_array, pdf_new_number(defaultVertOriginY));
        pdf_add_array(an_array, pdf_new_number(-defaultAdvanceHeight));
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"DW2\x00" as *const u8 as *const i8),
            an_array,
        );
    }
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W2\x00" as *const u8 as *const i8),
            pdf_ref_obj(w2_array),
        );
    }
    pdf_release_obj(w2_array);
}
/*
 * The following routine fixes few problems caused by vendor specific
 * Unicode mappings.
 */
unsafe extern "C" fn fix_CJK_symbols(mut code: u16) -> u16 {
    let mut alt_code: u16 = 0;
    static mut CJK_Uni_symbols: [C2RustUnnamed_2; 10] = [
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0x2014_u16,
                alt2: 0x2015_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0x2016_u16,
                alt2: 0x2225_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0x203e_u16,
                alt2: 0xffe3_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0x2026_u16,
                alt2: 0x22ef_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0x2212_u16,
                alt2: 0xff0d_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0x301c_u16,
                alt2: 0xff5e_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0xffe0_u16,
                alt2: 0xa2_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0xffe1_u16,
                alt2: 0xa3_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0xffe2_u16,
                alt2: 0xac_u16,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                alt1: 0xffff_u16,
                alt2: 0xffff_u16,
            };
            init
        },
    ];
    let mut i: u32 = 0;
    alt_code = code;
    i = 0_u32;
    while (i as u64)
        < (::std::mem::size_of::<[C2RustUnnamed_2; 10]>() as u64)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_2>() as u64)
    {
        if CJK_Uni_symbols[i as usize].alt1 as i32 == code as i32 {
            alt_code = CJK_Uni_symbols[i as usize].alt2;
            break;
        } else if CJK_Uni_symbols[i as usize].alt2 as i32 == code as i32 {
            alt_code = CJK_Uni_symbols[i as usize].alt1;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    alt_code
}
unsafe extern "C" fn cid_to_code(mut cmap: *mut CMap, mut cid: CID) -> i32 {
    let mut inbuf: [u8; 2] = [0; 2];
    let mut outbuf: [u8; 32] = [0; 32];
    let mut inbytesleft: size_t = 2i32 as size_t;
    let mut outbytesleft: size_t = 32i32 as size_t;
    let mut p: *const u8 = 0 as *const u8;
    let mut q: *mut u8 = 0 as *mut u8;
    if cmap.is_null() {
        return cid as i32;
    }
    inbuf[0] = (cid as i32 >> 8i32 & 0xffi32) as u8;
    inbuf[1] = (cid as i32 & 0xffi32) as u8;
    p = inbuf.as_mut_ptr();
    q = outbuf.as_mut_ptr();
    CMap_decode_char(cmap, &mut p, &mut inbytesleft, &mut q, &mut outbytesleft);
    if inbytesleft != 0i32 as u64 {
        return 0i32;
    } else {
        if outbytesleft == 31i32 as u64 {
            return outbuf[0] as i32;
        } else {
            if outbytesleft == 30i32 as u64 {
                return (outbuf[0] as i32) << 8i32 | outbuf[1] as i32;
            } else {
                if outbytesleft == 28i32 as u64 {
                    /* We assume the output encoding is UTF-16. */
                    let mut hi: CID = 0;
                    let mut lo: CID = 0;
                    hi = ((outbuf[0] as i32) << 8i32 | outbuf[1] as i32) as CID;
                    lo = ((outbuf[2] as i32) << 8i32 | outbuf[3] as i32) as CID;
                    if hi as i32 >= 0xd800i32
                        && hi as i32 <= 0xdbffi32
                        && lo as i32 >= 0xdc00i32
                        && lo as i32 <= 0xdfffi32
                    {
                        return (hi as i32 - 0xd800i32) * 0x400i32 + 0x10000i32 + lo as i32
                            - 0xdc00i32;
                    } else {
                        return (hi as i32) << 16i32 | lo as i32;
                    }
                }
            }
        }
    }
    0i32
}
/* #define NO_GHOSTSCRIPT_BUG 1 */
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type2_dofont(mut font: *mut CIDFont) {
    let mut fontfile: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut h_used_chars: *mut i8 = 0 as *mut i8;
    let mut v_used_chars: *mut i8 = 0 as *mut i8;
    let mut used_chars: *mut i8 = 0 as *mut i8;
    let mut glyphs: *mut tt_glyphs = 0 as *mut tt_glyphs;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut ttcmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut offset: u32 = 0_u32;
    let mut cid: CID = 0;
    let mut last_cid: CID = 0;
    let mut cidtogidmap: *mut u8 = 0 as *mut u8;
    let mut num_glyphs: u16 = 0;
    let mut i: i32 = 0;
    let mut glyph_ordering: i32 = 0i32;
    let mut unicode_cmap: i32 = 0i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if (*font).indirect.is_null() {
        return;
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
        pdf_ref_obj((*font).descriptor),
    );
    if CIDFont_is_BaseFont(font) {
        return;
    }
    /*
     * CIDSystemInfo comes here since Supplement can be increased.
     */
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    tmp = pdf_new_dict();
    pdf_add_dict(
        tmp,
        pdf_new_name(b"Registry\x00" as *const u8 as *const i8),
        pdf_new_string(
            (*(*font).csi).registry as *const libc::c_void,
            strlen((*(*font).csi).registry),
        ),
    );
    pdf_add_dict(
        tmp,
        pdf_new_name(b"Ordering\x00" as *const u8 as *const i8),
        pdf_new_string(
            (*(*font).csi).ordering as *const libc::c_void,
            strlen((*(*font).csi).ordering),
        ),
    );
    pdf_add_dict(
        tmp,
        pdf_new_name(b"Supplement\x00" as *const u8 as *const i8),
        pdf_new_number((*(*font).csi).supplement as f64),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"CIDSystemInfo\x00" as *const u8 as *const i8),
        tmp,
    );
    /* Quick exit for non-embedded & fixed-pitch font. */
    if CIDFont_get_embedding(font) == 0 && opt_flags & 1i32 << 1i32 != 0 {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"DW\x00" as *const u8 as *const i8),
            pdf_new_number(1000.0f64),
        );
        return;
    }
    handle = dpx_open_truetype_file((*font).ident);
    if handle.is_null() {
        handle = dpx_open_dfont_file((*font).ident);
        if handle.is_null() {
            _tt_abort(
                b"Could not open TTF/dfont file: %s\x00" as *const u8 as *const i8,
                (*font).ident,
            );
        }
        sfont = dfont_open(handle, (*(*font).options).index)
    } else {
        sfont = sfnt_open(handle)
    }
    if sfont.is_null() {
        _tt_abort(
            b"Could not open TTF file: %s\x00" as *const u8 as *const i8,
            (*font).ident,
        );
    }
    match (*sfont).type_0 {
        16 => {
            offset = ttc_read_offset(sfont, (*(*font).options).index);
            if offset == 0_u32 {
                _tt_abort(
                    b"Invalid TTC index in %s.\x00" as *const u8 as *const i8,
                    (*font).ident,
                );
            }
        }
        1 => {
            if (*(*font).options).index > 0i32 {
                _tt_abort(
                    b"Found TrueType font file while expecting TTC file (%s).\x00" as *const u8
                        as *const i8,
                    (*font).ident,
                );
            }
            offset = 0_u32
        }
        256 => offset = (*sfont).offset,
        _ => {
            _tt_abort(
                b"Not a TrueType/TTC font (%s)?\x00" as *const u8 as *const i8,
                (*font).ident,
            );
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0i32 {
        _tt_abort(
            b"Could not read TrueType table directory (%s).\x00" as *const u8 as *const i8,
            (*font).ident,
        );
    }
    /*
     * Adobe-Identity means font's internal glyph ordering here.
     */
    if streq_ptr(
        (*(*font).csi).registry,
        b"Adobe\x00" as *const u8 as *const i8,
    ) as i32
        != 0
        && streq_ptr(
            (*(*font).csi).ordering,
            b"Identity\x00" as *const u8 as *const i8,
        ) as i32
            != 0
    {
        glyph_ordering = 1i32
    } else {
        glyph_ordering = 0i32
    }
    /*
     * Select TrueType cmap table, find ToCode CMap for each TrueType encodings.
     */
    if glyph_ordering != 0 {
        ttcmap = 0 as *mut tt_cmap;
        cmap = 0 as *mut CMap
    } else {
        /*
         * This part contains a bug. It may choose SJIS encoding TrueType cmap
         * table for Adobe-GB1.
         */
        i = 0i32;
        while i <= 9i32 {
            ttcmap = tt_cmap_read(
                sfont,
                known_encodings[i as usize].platform,
                known_encodings[i as usize].encoding,
            );
            if !ttcmap.is_null() {
                break;
            }
            i += 1
        }
        if ttcmap.is_null() {
            dpx_warning(
                b"No usable TrueType cmap table found for font \"%s\".\x00" as *const u8
                    as *const i8,
                (*font).ident,
            );
            dpx_warning(
                b"CID character collection for this font is set to \"%s-%s\"\x00" as *const u8
                    as *const i8,
                (*(*font).csi).registry,
                (*(*font).csi).ordering,
            );
            _tt_abort(b"Cannot continue without this...\x00" as *const u8 as *const i8);
        } else {
            if i <= 1i32 {
                unicode_cmap = 1i32
            } else {
                unicode_cmap = 0i32
            }
        }
        /*
         * NULL is returned if CMap is Identity CMap.
         */
        cmap = find_tocode_cmap((*(*font).csi).registry, (*(*font).csi).ordering, i)
    } /* .notdef */
    glyphs = tt_build_init();
    last_cid = 0i32 as CID;
    num_glyphs = 1_u16;
    v_used_chars = 0 as *mut i8;
    h_used_chars = v_used_chars;
    used_chars = h_used_chars;
    let mut parent: *mut Type0Font = 0 as *mut Type0Font;
    let mut parent_id: i32 = 0;
    let mut c: i32 = 0;
    parent_id = CIDFont_get_parent_id(font, 0i32);
    if parent_id >= 0i32 {
        parent = Type0Font_cache_get(parent_id);
        h_used_chars = Type0Font_get_usedchars(parent)
    }
    parent_id = CIDFont_get_parent_id(font, 1i32);
    if parent_id >= 0i32 {
        parent = Type0Font_cache_get(parent_id);
        v_used_chars = Type0Font_get_usedchars(parent)
    }
    if h_used_chars.is_null() && v_used_chars.is_null() {
        _tt_abort(b"Unexpected error.\x00" as *const u8 as *const i8);
    }
    /*
     * Quick check of max CID.
     */
    c = 0i32;
    i = 8191i32;
    while i >= 0i32 {
        if !h_used_chars.is_null() && *h_used_chars.offset(i as isize) as i32 != 0i32 {
            last_cid = (i * 8i32 + 7i32) as CID;
            c = *h_used_chars.offset(i as isize) as i32;
            break;
        } else {
            i -= 1
        }
    }
    i = 8191i32;
    while i >= 0i32 {
        if !v_used_chars.is_null() && *v_used_chars.offset(i as isize) as i32 != 0i32 {
            if i * 8i32 + 7i32 >= last_cid as i32 {
                c = if i * 8i32 + 7i32 > last_cid as i32 {
                    *v_used_chars.offset(i as isize) as i32
                } else {
                    c | *v_used_chars.offset(i as isize) as i32
                };
                last_cid = (i * 8i32 + 7i32) as CID;
                break;
            }
        }
        i -= 1
    }
    if last_cid as i32 > 0i32 {
        i = 0i32;
        while i < 8i32 {
            if c >> i & 1i32 != 0 {
                break;
            }
            last_cid = last_cid.wrapping_sub(1);
            i += 1
        }
    }
    if last_cid as u32 >= 0xffffu32 {
        _tt_abort(b"CID count > 65535\x00" as *const u8 as *const i8);
    }
    cidtogidmap = 0 as *mut u8;
    /* !NO_GHOSTSCRIPT_BUG */
    /*
     * Map CIDs to GIDs.
     * Horizontal and vertical used_chars are merged.
     */
    /*
     * Horizontal
     */
    if !h_used_chars.is_null() {
        used_chars = h_used_chars;
        cid = 1i32 as CID;
        while cid as i32 <= last_cid as i32 {
            let mut code: i32 = 0;
            let mut gid: u16 = 0;
            if !(*h_used_chars.offset((cid as i32 / 8i32) as isize) as i32
                & 1i32 << 7i32 - cid as i32 % 8i32
                == 0)
            {
                if glyph_ordering != 0 {
                    gid = cid;
                    code = cid as i32
                } else {
                    code = cid_to_code(cmap, cid);
                    gid = tt_cmap_lookup(ttcmap, code as u32);
                    if gid as i32 == 0i32 && unicode_cmap != 0 {
                        let mut alt_code: i32 = 0;
                        alt_code = fix_CJK_symbols(code as u16) as i32;
                        if alt_code != code {
                            gid = tt_cmap_lookup(ttcmap, alt_code as u32);
                            if gid as i32 != 0i32 {
                                dpx_warning(
                                    b"Unicode char U+%04x replaced with U+%04x.\x00" as *const u8
                                        as *const i8,
                                    code,
                                    alt_code,
                                );
                            }
                        }
                    }
                    /* FIX_CJK_UNIOCDE_SYMBOLS */
                }
                if gid as i32 == 0i32 {
                    dpx_warning(
                        b"Glyph missing in font. (CID=%u, code=0x%04x)\x00" as *const u8
                            as *const i8,
                        cid as i32,
                        code,
                    );
                }
                /* TODO: duplicated glyph */
                gid = tt_add_glyph(glyphs, gid, cid);
                /* !NO_GHOSTSCRIPT_BUG */
                num_glyphs = num_glyphs.wrapping_add(1)
            }
            cid = cid.wrapping_add(1)
        }
    }
    /*
     * Vertical
     */
    if !v_used_chars.is_null() {
        let mut gsub_list: *mut otl_gsub = 0 as *mut otl_gsub;
        /*
         * Require `vrt2' or `vert'.
         */
        if glyph_ordering != 0 {
            gsub_list = 0 as *mut otl_gsub
        } else {
            gsub_list = otl_gsub_new();
            if otl_gsub_add_feat(
                gsub_list,
                b"*\x00" as *const u8 as *const i8,
                b"*\x00" as *const u8 as *const i8,
                b"vrt2\x00" as *const u8 as *const i8,
                sfont,
            ) < 0i32
            {
                if otl_gsub_add_feat(
                    gsub_list,
                    b"*\x00" as *const u8 as *const i8,
                    b"*\x00" as *const u8 as *const i8,
                    b"vert\x00" as *const u8 as *const i8,
                    sfont,
                ) < 0i32
                {
                    dpx_warning(b"GSUB feature vrt2/vert not found.\x00" as *const u8 as *const i8);
                    otl_gsub_release(gsub_list);
                    gsub_list = 0 as *mut otl_gsub
                } else {
                    otl_gsub_select(
                        gsub_list,
                        b"*\x00" as *const u8 as *const i8,
                        b"*\x00" as *const u8 as *const i8,
                        b"vert\x00" as *const u8 as *const i8,
                    );
                }
            } else {
                otl_gsub_select(
                    gsub_list,
                    b"*\x00" as *const u8 as *const i8,
                    b"*\x00" as *const u8 as *const i8,
                    b"vrt2\x00" as *const u8 as *const i8,
                );
            }
        }
        cid = 1i32 as CID;
        while cid as i32 <= last_cid as i32 {
            let mut code_0: i32 = 0;
            let mut gid_0: u16 = 0;
            if !(*v_used_chars.offset((cid as i32 / 8i32) as isize) as i32
                & 1i32 << 7i32 - cid as i32 % 8i32
                == 0)
            {
                /* There may be conflict of horizontal and vertical glyphs
                 * when font is used with /UCS. However, we simply ignore
                 * that...
                 */
                if !(!h_used_chars.is_null()
                    && *h_used_chars.offset((cid as i32 / 8i32) as isize) as i32
                        & 1i32 << 7i32 - cid as i32 % 8i32
                        != 0)
                {
                    if glyph_ordering != 0 {
                        gid_0 = cid;
                        code_0 = cid as i32
                    } else {
                        code_0 = cid_to_code(cmap, cid);
                        gid_0 = tt_cmap_lookup(ttcmap, code_0 as u32);
                        if gid_0 as i32 == 0i32 && unicode_cmap != 0 {
                            let mut alt_code_0: i32 = 0;
                            alt_code_0 = fix_CJK_symbols(code_0 as u16) as i32;
                            if alt_code_0 != code_0 {
                                gid_0 = tt_cmap_lookup(ttcmap, alt_code_0 as u32);
                                if gid_0 as i32 != 0i32 {
                                    dpx_warning(
                                        b"Unicode char U+%04x replaced with U+%04x.\x00"
                                            as *const u8
                                            as *const i8,
                                        code_0,
                                        alt_code_0,
                                    );
                                }
                            }
                        }
                        /* FIX_CJK_UNIOCDE_SYMBOLS */
                    }
                    if gid_0 as i32 == 0i32 {
                        dpx_warning(
                            b"Glyph missing in font. (CID=%u, code=0x%04x)\x00" as *const u8
                                as *const i8,
                            cid as i32,
                            code_0,
                        );
                    } else if !gsub_list.is_null() {
                        otl_gsub_apply(gsub_list, &mut gid_0);
                    }
                    gid_0 = tt_add_glyph(glyphs, gid_0, cid);
                    /* !NO_GHOSTSCRIPT_BUG */
                    if !used_chars.is_null() {
                        /* merge vertical used_chars to horizontal */
                        let ref mut fresh0 = *used_chars.offset((cid as i32 / 8i32) as isize);
                        *fresh0 = (*fresh0 as i32 | 1i32 << 7i32 - cid as i32 % 8i32) as i8
                    }
                    num_glyphs = num_glyphs.wrapping_add(1)
                }
            }
            cid = cid.wrapping_add(1)
        }
        if !gsub_list.is_null() {
            otl_gsub_release(gsub_list);
        }
        if used_chars.is_null() {
            /* We have no horizontal. */
            used_chars = v_used_chars
        }
    }
    if used_chars.is_null() {
        _tt_abort(b"Unexpected error.\x00" as *const u8 as *const i8);
    }
    tt_cmap_release(ttcmap);
    if CIDFont_get_embedding(font) != 0 {
        if tt_build_tables(sfont, glyphs) < 0i32 {
            _tt_abort(b"Could not created FontFile stream.\x00" as *const u8 as *const i8);
        }
        if verbose > 1i32 {
            dpx_message(
                b"[%u glyphs (Max CID: %u)]\x00" as *const u8 as *const i8,
                (*glyphs).num_glyphs as i32,
                last_cid as i32,
            );
        }
    } else if tt_get_metrics(sfont, glyphs) < 0i32 {
        _tt_abort(b"Reading glyph metrics failed...\x00" as *const u8 as *const i8);
    }
    /*
     * DW, W, DW2, and W2
     */
    if opt_flags & 1i32 << 1i32 != 0 {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"DW\x00" as *const u8 as *const i8),
            pdf_new_number(1000.0f64),
        );
    } else {
        add_TTCIDHMetrics((*font).fontdict, glyphs, used_chars, cidtogidmap, last_cid);
        if !v_used_chars.is_null() {
            add_TTCIDVMetrics((*font).fontdict, glyphs, used_chars, last_cid);
        }
    }
    tt_build_finish(glyphs);
    /* Finish here if not embedded. */
    if CIDFont_get_embedding(font) == 0 {
        free(cidtogidmap as *mut libc::c_void);
        sfnt_close(sfont);
        if !handle.is_null() {
            ttstub_input_close(handle);
        }
        return;
    }
    /* Create font file */
    i = 0i32;
    while !required_table[i as usize].name.is_null() {
        if sfnt_require_table(
            sfont,
            required_table[i as usize].name,
            required_table[i as usize].must_exist,
        ) < 0i32
        {
            _tt_abort(
                b"Some required TrueType table (%s) does not exist.\x00" as *const u8 as *const i8,
                required_table[i as usize].name,
            );
        }
        i += 1
    }
    /*
     * FontFile2
     */
    fontfile = sfnt_create_FontFile_stream(sfont);
    sfnt_close(sfont);
    if !handle.is_null() {
        ttstub_input_close(handle);
    }
    if fontfile.is_null() {
        _tt_abort(
            b"Could not created FontFile stream for \"%s\".\x00" as *const u8 as *const i8,
            (*font).ident,
        );
    }
    if verbose > 1i32 {
        dpx_message(
            b"[%d bytes]\x00" as *const u8 as *const i8,
            pdf_stream_length(fontfile),
        );
    }
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontFile2\x00" as *const u8 as *const i8),
        pdf_ref_obj(fontfile),
    );
    pdf_release_obj(fontfile);
    /*
     * CIDSet
     */
    let mut cidset: *mut pdf_obj = 0 as *mut pdf_obj;
    cidset = pdf_new_stream(1i32 << 0i32);
    pdf_add_stream(
        cidset,
        used_chars as *const libc::c_void,
        last_cid as i32 / 8i32 + 1i32,
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"CIDSet\x00" as *const u8 as *const i8),
        pdf_ref_obj(cidset),
    );
    pdf_release_obj(cidset);
    /*
     * CIDToGIDMap
     * Adobe's PDF Reference had been describing it as "optional" and
     * default value as "Identity". However, ISO 32000-1 requires it
     * for Type 2 CIDFonts with embedded font programs.
     */
    if cidtogidmap.is_null() {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"CIDToGIDMap\x00" as *const u8 as *const i8),
            pdf_new_name(b"Identity\x00" as *const u8 as *const i8),
        );
    } else {
        let mut c2gmstream: *mut pdf_obj = 0 as *mut pdf_obj;
        c2gmstream = pdf_new_stream(1i32 << 0i32);
        pdf_add_stream(
            c2gmstream,
            cidtogidmap as *const libc::c_void,
            (last_cid as i32 + 1i32) * 2i32,
        );
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"CIDToGIDMap\x00" as *const u8 as *const i8),
            pdf_ref_obj(c2gmstream),
        );
        pdf_release_obj(c2gmstream);
        free(cidtogidmap as *mut libc::c_void);
    };
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
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type2_open(
    mut font: *mut CIDFont,
    mut name: *const i8,
    mut cmap_csi: *mut CIDSysInfo,
    mut opt: *mut cid_opt,
) -> i32 {
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: u32 = 0_u32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    assert!(!font.is_null() && !opt.is_null());
    handle = dpx_open_truetype_file(name);
    if handle.is_null() {
        handle = dpx_open_dfont_file(name);
        if handle.is_null() {
            return -1i32;
        }
        sfont = dfont_open(handle, (*opt).index)
    } else {
        sfont = sfnt_open(handle)
    }
    if sfont.is_null() {
        ttstub_input_close(handle);
        return -1i32;
    }
    match (*sfont).type_0 {
        16 => offset = ttc_read_offset(sfont, (*opt).index),
        1 => {
            if (*opt).index > 0i32 {
                _tt_abort(
                    b"Invalid TTC index (not TTC font): %s\x00" as *const u8 as *const i8,
                    name,
                );
            }
            offset = 0_u32
        }
        256 => offset = (*sfont).offset,
        _ => {
            sfnt_close(sfont);
            if !handle.is_null() {
                ttstub_input_close(handle);
            }
            return -1i32;
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0i32 {
        _tt_abort(b"Reading TrueType table directory failed.\x00" as *const u8 as *const i8);
    }
    /* Ignore TrueType Collection with CFF table. */
    if (*sfont).type_0 == 1i32 << 4i32
        && sfnt_find_table_pos(sfont, b"CFF \x00" as *const u8 as *const i8) != 0
    {
        sfnt_close(sfont);
        if !handle.is_null() {
            ttstub_input_close(handle);
        }
        return -1i32;
    }
    let mut shortname: *mut i8 = 0 as *mut i8;
    let mut namelen: i32 = 0;
    /* MAC-ROMAN-EN-POSTSCRIPT or WIN-UNICODE-EN(US)-POSTSCRIPT */
    shortname = new((127_u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8; /* for SJIS, UTF-16, ... string */
    namelen = tt_get_ps_fontname(sfont, shortname, 127_u16) as i32;
    if namelen == 0i32 {
        memset(shortname as *mut libc::c_void, 0i32, 127i32 as u64);
        strncpy(shortname, name, 127i32 as u64);
        namelen = strlen(shortname) as i32
    }
    validate_name(shortname, namelen);
    /*
     * Strlen works, after validate_named string.
     * Mangled name requires more 7 bytes.
     * Style requires more 11 bytes.
     */
    fontname = new((strlen(shortname).wrapping_add(19i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(fontname, shortname);
    free(shortname as *mut libc::c_void);
    if (*opt).embed != 0 && (*opt).style != 0i32 {
        dpx_warning(
            b"Embedding disabled due to style option for %s.\x00" as *const u8 as *const i8,
            name,
        );
        (*opt).embed = 0i32
    }
    match (*opt).style {
        1 => {
            strcat(fontname, b",Bold\x00" as *const u8 as *const i8);
        }
        2 => {
            strcat(fontname, b",Italic\x00" as *const u8 as *const i8);
        }
        3 => {
            strcat(fontname, b",BoldItalic\x00" as *const u8 as *const i8);
        }
        _ => {}
    }
    /*
     * CIDSystemInfo is determined from CMap or from map record option.
     */
    (*font).fontname = fontname; /* This means font's internal glyph ordering. */
    (*font).subtype = 2i32;
    (*font).csi = new((1_u64).wrapping_mul(::std::mem::size_of::<CIDSysInfo>() as u64) as u32)
        as *mut CIDSysInfo;
    if !(*opt).csi.is_null() {
        if !cmap_csi.is_null() {
            if strcmp((*(*opt).csi).registry, (*cmap_csi).registry) != 0
                || strcmp((*(*opt).csi).ordering, (*cmap_csi).ordering) != 0
            {
                dpx_warning(
                    b"CID character collection mismatched:\n\x00" as *const u8 as *const i8,
                );
                dpx_message(
                    b"\tFont: %s-%s-%d\n\x00" as *const u8 as *const i8,
                    (*(*opt).csi).registry,
                    (*(*opt).csi).ordering,
                    (*(*opt).csi).supplement,
                );
                dpx_message(
                    b"\tCMap: %s-%s-%d\n\x00" as *const u8 as *const i8,
                    (*cmap_csi).registry,
                    (*cmap_csi).ordering,
                    (*cmap_csi).supplement,
                );
                _tt_abort(
                    b"Incompatible CMap specified for this font.\x00" as *const u8 as *const i8,
                );
            }
            if (*(*opt).csi).supplement < (*cmap_csi).supplement {
                dpx_warning(
                    b"Supplmement value in CIDSystemInfo increased.\x00" as *const u8 as *const i8,
                );
                dpx_warning(b"Some characters may not shown.\x00" as *const u8 as *const i8);
                (*(*opt).csi).supplement = (*cmap_csi).supplement
            }
        }
        (*(*font).csi).registry = new((strlen((*(*opt).csi).registry).wrapping_add(1i32 as u64)
            as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy((*(*font).csi).registry, (*(*opt).csi).registry);
        (*(*font).csi).ordering = new((strlen((*(*opt).csi).ordering).wrapping_add(1i32 as u64)
            as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy((*(*font).csi).ordering, (*(*opt).csi).ordering);
        (*(*font).csi).supplement = (*(*opt).csi).supplement
    } else if !cmap_csi.is_null() {
        (*(*font).csi).registry = new(
            (strlen((*cmap_csi).registry).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        strcpy((*(*font).csi).registry, (*cmap_csi).registry);
        (*(*font).csi).ordering = new(
            (strlen((*cmap_csi).ordering).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        strcpy((*(*font).csi).ordering, (*cmap_csi).ordering);
        (*(*font).csi).supplement = (*cmap_csi).supplement
    } else {
        (*(*font).csi).registry = new((strlen(b"Adobe\x00" as *const u8 as *const i8)
            .wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy(
            (*(*font).csi).registry,
            b"Adobe\x00" as *const u8 as *const i8,
        );
        (*(*font).csi).ordering = new((strlen(b"Identity\x00" as *const u8 as *const i8)
            .wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy(
            (*(*font).csi).ordering,
            b"Identity\x00" as *const u8 as *const i8,
        );
        (*(*font).csi).supplement = 0i32
    }
    (*font).fontdict = pdf_new_dict();
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Font\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"CIDFontType2\x00" as *const u8 as *const i8),
    );
    (*font).descriptor = tt_get_fontdesc(sfont, &mut (*opt).embed, (*opt).stemv, 0i32, name);
    if (*font).descriptor.is_null() {
        _tt_abort(b"Could not obtain necessary font info.\x00" as *const u8 as *const i8);
    }
    if (*opt).embed != 0 {
        memmove(
            fontname.offset(7) as *mut libc::c_void,
            fontname as *const libc::c_void,
            strlen(fontname).wrapping_add(1i32 as u64),
        );
        pdf_font_make_uniqueTag(fontname);
        *fontname.offset(6) = '+' as i32 as i8
    }
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
        pdf_new_name(fontname),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"BaseFont\x00" as *const u8 as *const i8),
        pdf_new_name(fontname),
    );
    sfnt_close(sfont);
    if !handle.is_null() {
        ttstub_input_close(handle);
    }
    /*
     * Don't write fontdict here.
     * /Supplement in /CIDSystemInfo may change.
     */
    0i32
}
