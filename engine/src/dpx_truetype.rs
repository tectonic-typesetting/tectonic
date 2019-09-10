#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
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
    /* Here is the complete list of PDF object types */
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    pub type pdf_font;
    pub type otl_gsub;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atoi(__nptr: *const i8) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: libc::c_uint,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64)
        -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_set_fontname(font: *mut pdf_font, fontname: *const i8) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_index(font: *mut pdf_font) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_encoding(font: *mut pdf_font) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_usedchars(font: *mut pdf_font) -> *mut i8;
    #[no_mangle]
    fn pdf_font_get_descriptor(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_mapname(font: *mut pdf_font) -> *mut i8;
    #[no_mangle]
    fn pdf_font_get_ident(font: *mut pdf_font) -> *mut i8;
    #[no_mangle]
    fn pdf_font_is_in_use(font: *mut pdf_font) -> bool;
    #[no_mangle]
    fn pdf_font_get_verbose() -> libc::c_int;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> libc::c_int;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
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
    fn pdf_array_length(array: *mut pdf_obj) -> libc::c_uint;
    #[no_mangle]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_stream_length(stream: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const i8, _: libc::c_int) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn agl_chop_suffix(
        glyphname: *const i8,
        suffix: *mut *mut i8,
    ) -> *mut i8;
    #[no_mangle]
    fn agl_name_is_unicode(glyphname: *const i8) -> bool;
    #[no_mangle]
    fn agl_name_convert_unicode(glyphname: *const i8) -> i32;
    #[no_mangle]
    fn agl_suffix_to_otltag(suffix: *const i8) -> *const i8;
    #[no_mangle]
    fn agl_lookup_list(glyphname: *const i8) -> *mut agl_name;
    #[no_mangle]
    fn dpx_open_truetype_file(filename: *const i8) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_dfont_file(filename: *const i8) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
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
    fn pdf_encoding_is_predefined(enc_id: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_encoding_get_encoding(enc_id: libc::c_int) -> *mut *mut i8;
    /* 16.16-bit signed fixed-point number */
    /* table header */
    /* table data */
    /* Fixed for Win */
    /* number of kept tables */
    /* keep or omit */
    /* sfnt resource */
    /* Convert sfnt "fixed" type to double */
    /* get_***_*** from numbers.h */
    #[no_mangle]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[no_mangle]
    fn sfnt_close(sfont: *mut sfnt);
    /* table directory */
    #[no_mangle]
    fn sfnt_set_table(
        sfont: *mut sfnt,
        tag: *const i8,
        data: *mut libc::c_void,
        length: SFNT_ULONG,
    );
    #[no_mangle]
    fn sfnt_require_table(
        sfont: *mut sfnt,
        tag: *const i8,
        must_exist: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn sfnt_create_FontFile_stream(sfont: *mut sfnt) -> *mut pdf_obj;
    #[no_mangle]
    fn dfont_open(handle: rust_input_handle_t, index: libc::c_int) -> *mut sfnt;
    #[no_mangle]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> libc::c_int;
    #[no_mangle]
    fn put_big_endian(s: *mut libc::c_void, q: SFNT_LONG, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tfm_open(tex_name: *const i8, must_exist: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tfm_get_width(font_id: libc::c_int, ch: i32) -> libc::c_double;
    /* TTC (TrueType Collection) */
    #[no_mangle]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: libc::c_int) -> SFNT_ULONG;
    /* FontDescriptor */
    #[no_mangle]
    fn tt_get_fontdesc(
        sfont: *mut sfnt,
        embed: *mut libc::c_int,
        stemv: libc::c_int,
        type_0: libc::c_int,
        fontname: *const i8,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn tt_cmap_read(sfont: *mut sfnt, platform: USHORT, encoding: USHORT) -> *mut tt_cmap;
    #[no_mangle]
    fn tt_cmap_lookup(cmap: *mut tt_cmap, cc: SFNT_ULONG) -> USHORT;
    #[no_mangle]
    fn tt_cmap_release(cmap: *mut tt_cmap);
    #[no_mangle]
    fn tt_build_init() -> *mut tt_glyphs;
    #[no_mangle]
    fn tt_build_finish(g: *mut tt_glyphs);
    #[no_mangle]
    fn tt_add_glyph(g: *mut tt_glyphs, gid: USHORT, new_gid: USHORT) -> USHORT;
    #[no_mangle]
    fn tt_get_index(g: *mut tt_glyphs, gid: USHORT) -> USHORT;
    #[no_mangle]
    fn tt_find_glyph(g: *mut tt_glyphs, gid: USHORT) -> USHORT;
    #[no_mangle]
    fn tt_build_tables(sfont: *mut sfnt, g: *mut tt_glyphs) -> libc::c_int;
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
    ) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_add_feat(
        gsub_list: *mut otl_gsub,
        script: *const i8,
        language: *const i8,
        feature: *const i8,
        sfont: *mut sfnt,
    ) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_apply(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_apply_alt(
        gsub_list: *mut otl_gsub,
        alt_idx: USHORT,
        gid: *mut USHORT,
    ) -> libc::c_int;
    #[no_mangle]
    fn otl_gsub_apply_lig(
        gsub_list: *mut otl_gsub,
        gid_in: *mut USHORT,
        num_gids: USHORT,
        gid_out: *mut USHORT,
    ) -> libc::c_int;
    #[no_mangle]
    fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table;
    #[no_mangle]
    fn tt_release_post_table(post: *mut tt_post_table);
    #[no_mangle]
    fn tt_lookup_post_table(post: *mut tt_post_table, glyphname: *const i8) -> USHORT;
    /* name table */
    #[no_mangle]
    fn tt_get_ps_fontname(sfont: *mut sfnt, dest: *mut i8, destlen: USHORT) -> USHORT;
}
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
pub type SFNT_ULONG = u32;
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
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut i8,
}
pub type USHORT = u16;
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
    /* Number of glyph names in names[] */
}
pub type FWord = libc::c_short;
pub type Fixed = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_cmap {
    pub format: USHORT,
    pub platform: USHORT,
    pub encoding: USHORT,
    pub language: SFNT_ULONG,
    pub map: *mut libc::c_void,
}
/*
 * The 'name' table should be preserved since it contains copyright
 * information, but it might cause problem when there are invalid
 * table entries (wrongly encoded text which is often the case in
 * CJK fonts). Acrobat does not use 'name' table. Unicode TrueType
 * fonts may have 10K bytes 'name' table...
 *
 * We preserve the 'OS/2' table too, since it contains the license
 * information. PDF applications should use this table to decide
 * whether the font is embedded only for the purpose of preview &
 * printing. Otherwise, we must encrypt the document. Acrobat does
 * not use 'OS/2' table, though...
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *const i8,
    pub must_exist: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyphs {
    pub num_glyphs: USHORT,
    pub max_glyphs: USHORT,
    pub last_gid: USHORT,
    pub emsize: USHORT,
    pub dw: USHORT,
    pub default_advh: USHORT,
    pub default_tsb: SHORT,
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
    pub gid: USHORT,
    pub ogid: USHORT,
    pub advw: USHORT,
    pub advh: USHORT,
    pub lsb: SHORT,
    pub tsb: SHORT,
    pub llx: SHORT,
    pub lly: SHORT,
    pub urx: SHORT,
    pub ury: SHORT,
    pub length: SFNT_ULONG,
    pub data: *mut BYTE,
}
pub type BYTE = u8;
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
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */
pub type SHORT = libc::c_short;
/* Order of lookup should be
 *  post, unicode+otl
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyph_mapper {
    pub codetogid: *mut tt_cmap,
    pub gsub: *mut otl_gsub,
    pub sfont: *mut sfnt,
    pub nametogid: *mut tt_post_table,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct agl_name {
    pub name: *mut i8,
    pub suffix: *mut i8,
    pub n_components: libc::c_int,
    pub unicodes: [i32; 16],
    pub alternate: *mut agl_name,
    pub is_predef: libc::c_int,
}
pub type SFNT_LONG = i32;
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
/* TrueType */
/* Modifying this has no effect :P */
#[no_mangle]
pub unsafe extern "C" fn pdf_font_open_truetype(mut font: *mut pdf_font) -> libc::c_int {
    let mut ident: *mut i8 = 0 as *mut i8; /* Must be embedded. */
    let mut index: libc::c_int = 0;
    let mut encoding_id: libc::c_int = 0;
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut embedding: libc::c_int = 1i32;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut length: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            65i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[i8; 39]>(
                b"int pdf_font_open_truetype(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    ident = pdf_font_get_ident(font);
    index = pdf_font_get_index(font);
    if !ident.is_null() {
    } else {
        __assert_fail(
            b"ident\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            70i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[i8; 39]>(
                b"int pdf_font_open_truetype(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    handle = dpx_open_truetype_file(ident) as *mut rust_input_handle_t;
    if handle.is_null() {
        handle = dpx_open_dfont_file(ident) as *mut rust_input_handle_t;
        if handle.is_null() {
            return -1i32;
        }
        sfont = dfont_open(handle as rust_input_handle_t, index)
    } else {
        sfont = sfnt_open(handle as rust_input_handle_t)
    }
    if sfont.is_null() {
        dpx_warning(
            b"Could not open TrueType font: %s\x00" as *const u8 as *const i8,
            ident,
        );
        ttstub_input_close(handle as rust_input_handle_t);
        return -1i32;
    }
    if (*sfont).type_0 == 1i32 << 4i32 {
        let mut offset: SFNT_ULONG = 0;
        offset = ttc_read_offset(sfont, index);
        if offset == 0i32 as libc::c_uint {
            _tt_abort(
                b"Invalid TTC index in %s.\x00" as *const u8 as *const i8,
                ident,
            );
        }
        error = sfnt_read_table_directory(sfont, offset)
    } else {
        error = sfnt_read_table_directory(sfont, (*sfont).offset)
    }
    if error != 0 {
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        return -1i32;
        /* Silently */
    }
    /* Reading fontdict before checking fonttype conflicts with PKFONT
     * because pdf_font_get_resource() always makes a dictionary.
     */
    encoding_id = pdf_font_get_encoding(font);
    fontdict = pdf_font_get_resource(font);
    descriptor = pdf_font_get_descriptor(font);
    /* ENABLE_NOEMBED */
    if !fontdict.is_null() && !descriptor.is_null() {
    } else {
        __assert_fail(
            b"fontdict && descriptor\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            114i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[i8; 39]>(
                b"int pdf_font_open_truetype(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut fontname: [i8; 256] = [0; 256];
    let mut n: libc::c_int = 0;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    memset(
        fontname.as_mut_ptr() as *mut libc::c_void,
        0i32,
        256i32 as u64,
    );
    length = tt_get_ps_fontname(sfont, fontname.as_mut_ptr(), 255i32 as USHORT) as libc::c_int;
    if length < 1i32 {
        length = (if strlen(ident) < 255i32 as u64 {
            strlen(ident)
        } else {
            255i32 as u64
        }) as libc::c_int;
        /* Suppress some warnings on GCC. Clang supports the same warning control
         * #pragmas (and #defines __GNUC__!), but not these particular warnings, which
         * leads to a meta-warning if they're left unguarded. */
        strncpy(fontname.as_mut_ptr(), ident, length as u64);
    }
    fontname[length as usize] = '\u{0}' as i32 as i8;
    n = 0i32;
    while n < length {
        if fontname[n as usize] as libc::c_int == 0i32 {
            memmove(
                fontname.as_mut_ptr().offset(n as isize) as *mut libc::c_void,
                fontname.as_mut_ptr().offset(n as isize).offset(1) as *const libc::c_void,
                (length - n - 1i32) as u64,
            );
        }
        n += 1
    }
    if strlen(fontname.as_mut_ptr()) == 0i32 as u64 {
        _tt_abort(
            b"Can\'t find valid fontname for \"%s\".\x00" as *const u8 as *const i8,
            ident,
        );
    }
    pdf_font_set_fontname(font, fontname.as_mut_ptr());
    tmp = tt_get_fontdesc(sfont, &mut embedding, -1i32, 1i32, fontname.as_mut_ptr());
    if tmp.is_null() {
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        _tt_abort(b"Could not obtain necessary font info.\x00" as *const u8 as *const i8);
    }
    if pdf_obj_typeof(tmp) == 6i32 {
    } else {
        __assert_fail(
            b"pdf_obj_typeof(tmp) == PDF_DICT\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            154i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[i8; 39]>(
                b"int pdf_font_open_truetype(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    pdf_merge_dict(descriptor, tmp);
    pdf_release_obj(tmp);
    if embedding == 0 {
        if encoding_id >= 0i32 && pdf_encoding_is_predefined(encoding_id) == 0 {
            sfnt_close(sfont);
            _tt_abort(
                b"Custom encoding not allowed for non-embedded TrueType font.\x00" as *const u8
                    as *const i8,
            );
        } else {
            /* There are basically no guarantee for font substitution
             * can work with "symblic" fonts. At least all glyphs
             * contained in the font must be identified; glyphs covers
             * by this instance of font should contain glyphs only from
             * Adobe Standard Latin Set. We allow non-embedded font
             * only to predefined encodings for this reason. Note that
             * "builtin" encoding means "MacRoman" here.
             */
            _tt_abort(
                b"Font file=\"%s\" can\'t be embedded due to liscence restrictions.\x00"
                    as *const u8 as *const i8,
                ident,
            );
            /* ENABLE_NOEMBED */
        }
    }
    sfnt_close(sfont);
    ttstub_input_close(handle as rust_input_handle_t);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Font\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"TrueType\x00" as *const u8 as *const i8),
    );
    return 0i32;
}
static mut required_table: [C2RustUnnamed; 13] = [
    {
        let mut init = C2RustUnnamed {
            name: b"OS/2\x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"head\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"hhea\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"loca\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"maxp\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"name\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"glyf\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"hmtx\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"fpgm\x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"cvt \x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"prep\x00" as *const u8 as *const i8,
            must_exist: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"cmap\x00" as *const u8 as *const i8,
            must_exist: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: 0 as *const i8,
            must_exist: 0i32,
        };
        init
    },
];
unsafe extern "C" fn do_widths(mut font: *mut pdf_font, mut widths: *mut libc::c_double) {
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmparray: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut code: libc::c_int = 0;
    let mut firstchar: libc::c_int = 0;
    let mut lastchar: libc::c_int = 0;
    let mut tfm_id: libc::c_int = 0;
    let mut usedchars: *mut i8 = 0 as *mut i8;
    fontdict = pdf_font_get_resource(font);
    usedchars = pdf_font_get_usedchars(font);
    tmparray = pdf_new_array();
    firstchar = 255i32;
    lastchar = 0i32;
    code = 0i32;
    while code < 256i32 {
        if *usedchars.offset(code as isize) != 0 {
            if code < firstchar {
                firstchar = code
            }
            if code > lastchar {
                lastchar = code
            }
        }
        code += 1
    }
    if firstchar > lastchar {
        dpx_warning(b"No glyphs actually used???\x00" as *const u8 as *const i8);
        pdf_release_obj(tmparray);
        return;
    }
    tfm_id = tfm_open(pdf_font_get_mapname(font), 0i32);
    code = firstchar;
    while code <= lastchar {
        if *usedchars.offset(code as isize) != 0 {
            let mut width: libc::c_double = 0.;
            if tfm_id < 0i32 {
                /* tfm is not found */
                width = *widths.offset(code as isize)
            } else {
                width = 1000.0f64 * tfm_get_width(tfm_id, code)
            }
            pdf_add_array(
                tmparray,
                pdf_new_number(floor(width / 0.1f64 + 0.5f64) * 0.1f64),
            );
        } else {
            pdf_add_array(tmparray, pdf_new_number(0.0f64));
        }
        code += 1
    }
    if pdf_array_length(tmparray) > 0i32 as libc::c_uint {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Widths\x00" as *const u8 as *const i8),
            pdf_ref_obj(tmparray),
        );
    }
    pdf_release_obj(tmparray);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FirstChar\x00" as *const u8 as *const i8),
        pdf_new_number(firstchar as libc::c_double),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"LastChar\x00" as *const u8 as *const i8),
        pdf_new_number(lastchar as libc::c_double),
    );
}
static mut verbose: libc::c_int = 0i32;
/*
 * There are several issues in TrueType font support in PDF.
 * How PDF viewers select TrueType cmap table is not so clear.
 * Most reliable way seem to reencode font and sort glyphs as
 * charcode == gid and to use Mac-Roman format 0 subtable.
 * It does not work with encodings that uses full 256 range since
 * GID = 0 is reserved for .notdef, so GID = 256 is not accessible.
 */
unsafe extern "C" fn do_builtin_encoding(
    mut font: *mut pdf_font,
    mut usedchars: *const i8,
    mut sfont: *mut sfnt,
) -> libc::c_int {
    let mut glyphs: *mut tt_glyphs = 0 as *mut tt_glyphs;
    let mut cmap_table: *mut i8 = 0 as *mut i8;
    let mut ttcm: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut gid: USHORT = 0;
    let mut idx: USHORT = 0;
    let mut code: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut widths: [libc::c_double; 256] = [0.; 256];
    ttcm = tt_cmap_read(sfont, 1u32 as USHORT, 0u32 as USHORT);
    if ttcm.is_null() {
        dpx_warning(
            b"Could not read Mac-Roman TrueType cmap table...\x00" as *const u8
                as *const i8,
        );
        return -1i32;
    }
    cmap_table = new((274i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    memset(
        cmap_table as *mut libc::c_void,
        0i32,
        274i32 as u64,
    );
    put_big_endian(cmap_table as *mut libc::c_void, 0i32, 2i32);
    /* Version  */
    put_big_endian(cmap_table.offset(2) as *mut libc::c_void, 1i32, 2i32);
    /* Number of subtables */
    put_big_endian(
        cmap_table.offset(4) as *mut libc::c_void,
        1u32 as SFNT_LONG,
        2i32,
    );
    /* Platform ID */
    put_big_endian(
        cmap_table.offset(6) as *mut libc::c_void,
        0u32 as SFNT_LONG,
        2i32,
    );
    /* Encoding ID */
    put_big_endian(cmap_table.offset(8) as *mut libc::c_void, 12i32, 4i32);
    /* Offset   */
    put_big_endian(cmap_table.offset(12) as *mut libc::c_void, 0i32, 2i32);
    /* Format   */
    put_big_endian(cmap_table.offset(14) as *mut libc::c_void, 262i32, 2i32);
    /* Length   */
    put_big_endian(cmap_table.offset(16) as *mut libc::c_void, 0i32, 2i32);
    /* Language */
    glyphs = tt_build_init(); /* .notdef */
    if verbose > 2i32 {
        dpx_message(b"[glyphs:/.notdef\x00" as *const u8 as *const i8);
    }
    count = 1i32;
    code = 0i32;
    while code < 256i32 {
        if !(*usedchars.offset(code as isize) == 0) {
            if verbose > 2i32 {
                dpx_message(b"/.c0x%02x\x00" as *const u8 as *const i8, code);
            }
            gid = tt_cmap_lookup(ttcm, code as SFNT_ULONG);
            if gid as libc::c_int == 0i32 {
                dpx_warning(
                    b"Glyph for character code=0x%02x missing in font font-file=\"%s\".\x00"
                        as *const u8 as *const i8,
                    code,
                    pdf_font_get_ident(font),
                );
                idx = 0i32 as USHORT
            } else {
                idx = tt_find_glyph(glyphs, gid);
                if idx as libc::c_int == 0i32 {
                    idx = tt_add_glyph(glyphs, gid, count as USHORT)
                }
                /* count returned. */
            } /* bug here */
            *cmap_table.offset((18i32 + code) as isize) =
                (idx as libc::c_int & 0xffi32) as i8;
            count += 1
        }
        code += 1
    }
    tt_cmap_release(ttcm);
    if verbose > 2i32 {
        dpx_message(b"]\x00" as *const u8 as *const i8);
    }
    if tt_build_tables(sfont, glyphs) < 0i32 {
        dpx_warning(
            b"Packing TrueType font into SFNT failed!\x00" as *const u8 as *const i8,
        );
        tt_build_finish(glyphs);
        free(cmap_table as *mut libc::c_void);
        return -1i32;
    }
    code = 0i32;
    while code < 256i32 {
        if *usedchars.offset(code as isize) != 0 {
            idx = tt_get_index(
                glyphs,
                *cmap_table.offset((18i32 + code) as isize) as USHORT,
            );
            widths[code as usize] = floor(
                1000.0f64
                    * (*(*glyphs).gd.offset(idx as isize)).advw as libc::c_int as libc::c_double
                    / (*glyphs).emsize as libc::c_int as libc::c_double
                    / 1i32 as libc::c_double
                    + 0.5f64,
            ) * 1i32 as libc::c_double
        } else {
            widths[code as usize] = 0.0f64
        }
        code += 1
    }
    do_widths(font, widths.as_mut_ptr());
    if verbose > 1i32 {
        dpx_message(
            b"[%d glyphs]\x00" as *const u8 as *const i8,
            (*glyphs).num_glyphs as libc::c_int,
        );
    }
    tt_build_finish(glyphs);
    sfnt_set_table(
        sfont,
        b"cmap\x00" as *const u8 as *const i8,
        cmap_table as *mut libc::c_void,
        274i32 as SFNT_ULONG,
    );
    return 0i32;
}
/* WARNING: This modifies glyphname itself */
unsafe extern "C" fn agl_decompose_glyphname(
    mut glyphname: *mut i8,
    mut nptrs: *mut *mut i8,
    mut size: libc::c_int,
    mut suffix: *mut *mut i8,
) -> libc::c_int {
    let mut q: *mut i8 = 0 as *mut i8; /* chop every thing after *first* dot */
    let mut p: *mut i8 = glyphname; /* _FIXME_ */
    let mut n: libc::c_int = 0;
    q = strchr(p, '.' as i32);
    if q.is_null() {
        *suffix = 0 as *mut i8
    } else {
        *q = '\u{0}' as i32 as i8;
        q = q.offset(1);
        *suffix = q
    }
    let ref mut fresh0 = *nptrs.offset(0);
    *fresh0 = p;
    n = 1i32;
    while !p.is_null() && *p as libc::c_int != 0 {
        p = strchr(p, '_' as i32);
        if p.is_null() || *p.offset(1) as libc::c_int == '\u{0}' as i32 {
            break;
        }
        if n >= size {
            _tt_abort(b"Uh ah...\x00" as *const u8 as *const i8);
        }
        *p = '\u{0}' as i32 as i8;
        p = p.offset(1);
        let ref mut fresh1 = *nptrs.offset(n as isize);
        *fresh1 = p;
        n += 1
    }
    return n;
}
unsafe extern "C" fn select_gsub(
    mut feat: *const i8,
    mut gm: *mut glyph_mapper,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    if feat.is_null() || *feat as libc::c_int == 0i32 || gm.is_null() || (*gm).gsub.is_null() {
        return -1i32;
    }
    /* First treat as is */
    idx = otl_gsub_select(
        (*gm).gsub,
        b"*\x00" as *const u8 as *const i8,
        b"*\x00" as *const u8 as *const i8,
        feat,
    );
    if idx >= 0i32 {
        return 0i32;
    }
    if verbose > 1i32 {
        dpx_message(
            b"\ntrutype>> Try loading OTL GSUB for \"*.*.%s\"...\x00" as *const u8
                as *const i8,
            feat,
        );
    }
    error = otl_gsub_add_feat(
        (*gm).gsub,
        b"*\x00" as *const u8 as *const i8,
        b"*\x00" as *const u8 as *const i8,
        feat,
        (*gm).sfont,
    );
    if error == 0 {
        idx = otl_gsub_select(
            (*gm).gsub,
            b"*\x00" as *const u8 as *const i8,
            b"*\x00" as *const u8 as *const i8,
            feat,
        );
        return if idx >= 0i32 { 0i32 } else { -1i32 };
    }
    return -1i32;
}
/* Apply GSUB. This is a bit tricky... */
unsafe extern "C" fn selectglyph(
    mut in_0: USHORT,
    mut suffix: *const i8,
    mut gm: *mut glyph_mapper,
    mut out: *mut USHORT,
) -> libc::c_int {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut t: [i8; 5] = [0; 5];
    let mut r: *const i8 = 0 as *const i8;
    let mut n: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    if !suffix.is_null() && !gm.is_null() && !out.is_null() {
    } else {
        __assert_fail(
            b"suffix && gm && out\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            451i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[i8; 71]>(
                b"int selectglyph(USHORT, const char *, struct glyph_mapper *, USHORT *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !suffix.is_null() && *suffix as libc::c_int != 0i32 {
    } else {
        __assert_fail(
            b"suffix && *suffix != 0\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            452i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[i8; 71]>(
                b"int selectglyph(USHORT, const char *, struct glyph_mapper *, USHORT *)\x00",
            ))
            .as_ptr(),
        );
    }
    s = new(
        (strlen(suffix).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32,
    ) as *mut i8;
    strcpy(s, suffix);
    /* First try converting suffix to feature tag.
     * agl.c currently only knows less ambiguos cases;
     * e.g., 'sc', 'superior', etc.
     */
    r = agl_suffix_to_otltag(s); /* 'suffix' may represent feature tag. */
    if !r.is_null() {
        /* We found feature tag for 'suffix'. */
        error = select_gsub(r, gm); /* no fallback for this */
        if error == 0 {
            error = otl_gsub_apply((*gm).gsub, &mut in_0)
        }
    } else {
        /* Try loading GSUB only when length of 'suffix' is less
         * than or equal to 4. tt_gsub give a warning otherwise.
         */
        if strlen(s) > 4i32 as u64 {
            error = -1i32
        } else if strlen(s) == 4i32 as u64 {
            error = select_gsub(s, gm)
        } else {
            /* Uh */
            /* less than 4. pad ' '. */
            memset(
                t.as_mut_ptr() as *mut libc::c_void,
                ' ' as i32,
                4i32 as u64,
            );
            t[4] = '\u{0}' as i32 as i8;
            memcpy(
                t.as_mut_ptr() as *mut libc::c_void,
                s as *const libc::c_void,
                strlen(s),
            );
            error = select_gsub(t.as_mut_ptr(), gm)
        }
        if error == 0 {
            /* 'suffix' represents feature tag. */
            error = otl_gsub_apply((*gm).gsub, &mut in_0)
        } else {
            /* other case: alt1, nalt10... (alternates) */
            q = s.offset(strlen(s) as isize).offset(-1);
            while q > s && *q as libc::c_int >= '0' as i32 && *q as libc::c_int <= '9' as i32 {
                q = q.offset(-1)
            }
            if q == s {
                error = -1i32
            } else {
                /* starting at 1 */
                n = atoi(q.offset(1)) - 1i32;
                *q.offset(1) = '\u{0}' as i32 as i8;
                if strlen(s) > 4i32 as u64 {
                    error = -1i32
                } else {
                    /* This may be alternate substitution. */
                    memset(
                        t.as_mut_ptr() as *mut libc::c_void,
                        ' ' as i32,
                        4i32 as u64,
                    );
                    t[4] = '\u{0}' as i32 as i8;
                    memcpy(
                        t.as_mut_ptr() as *mut libc::c_void,
                        s as *const libc::c_void,
                        strlen(s),
                    );
                    error = select_gsub(s, gm);
                    if error == 0 {
                        error =
                            otl_gsub_apply_alt((*gm).gsub, n as USHORT, &mut in_0 as *mut USHORT)
                    }
                }
            }
        }
    }
    free(s as *mut libc::c_void);
    *out = in_0;
    return error;
}
/* Compose glyphs via ligature substitution. */
unsafe extern "C" fn composeglyph(
    mut glyphs: *mut USHORT,
    mut n_glyphs: libc::c_int,
    mut feat: *const i8,
    mut gm: *mut glyph_mapper,
    mut gid: *mut USHORT,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut t: [i8; 5] = [
        ' ' as i32 as i8,
        ' ' as i32 as i8,
        ' ' as i32 as i8,
        ' ' as i32 as i8,
        0i32 as i8,
    ];
    if !glyphs.is_null() && n_glyphs > 0i32 && !gm.is_null() && !gid.is_null() {
    } else {
        __assert_fail(b"glyphs && n_glyphs > 0 && gm && gid\x00" as *const u8
                          as *const i8,
                      b"dpx-truetype.c\x00" as *const u8 as
                          *const i8, 514i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 79],
                                                &[i8; 79]>(b"int composeglyph(USHORT *, int, const char *, struct glyph_mapper *, USHORT *)\x00")).as_ptr());
    }
    if feat.is_null() || *feat.offset(0) as libc::c_int == '\u{0}' as i32 {
        /* meaning "Unknown" */
        error = select_gsub(
            b"(?lig|lig?|?cmp|cmp?|frac|afrc)\x00" as *const u8 as *const i8,
            gm,
        )
    } else if strlen(feat) > 4i32 as u64 {
        error = -1i32
    } else {
        memcpy(
            t.as_mut_ptr() as *mut libc::c_void,
            feat as *const libc::c_void,
            strlen(feat),
        );
        error = select_gsub(t.as_mut_ptr(), gm)
    }
    if error == 0 {
        error = otl_gsub_apply_lig((*gm).gsub, glyphs, n_glyphs as USHORT, gid)
    }
    return error;
}
/* This may be called by findparanoiac(). */
unsafe extern "C" fn composeuchar(
    mut unicodes: *mut i32,
    mut n_unicodes: libc::c_int,
    mut feat: *const i8,
    mut gm: *mut glyph_mapper,
    mut gid: *mut USHORT,
) -> libc::c_int {
    let mut gids: *mut USHORT = 0 as *mut USHORT;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    if (*gm).codetogid.is_null() {
        return -1i32;
    }
    gids = new((n_unicodes as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<USHORT>() as u64) as u32)
        as *mut USHORT;
    i = 0i32;
    while error == 0 && i < n_unicodes {
        *gids.offset(i as isize) =
            tt_cmap_lookup((*gm).codetogid, *unicodes.offset(i as isize) as SFNT_ULONG);
        error = if *gids.offset(i as isize) as libc::c_int == 0i32 {
            -1i32
        } else {
            0i32
        };
        i += 1
    }
    if error == 0 {
        error = composeglyph(gids, n_unicodes, feat, gm, gid)
    }
    free(gids as *mut libc::c_void);
    return error;
}
/* Search 'post' table. */
unsafe extern "C" fn findposttable(
    mut glyph_name: *const i8,
    mut gid: *mut USHORT,
    mut gm: *mut glyph_mapper,
) -> libc::c_int {
    if (*gm).nametogid.is_null() {
        return -1i32;
    }
    *gid = tt_lookup_post_table((*gm).nametogid, glyph_name);
    return if *gid as libc::c_int == 0i32 {
        -1i32
    } else {
        0i32
    };
}
/* This is wrong. We must care about '.'. */
/* Glyph names are concatinated with '_'. */
unsafe extern "C" fn findcomposite(
    mut glyphname: *const i8,
    mut gid: *mut USHORT,
    mut gm: *mut glyph_mapper,
) -> libc::c_int {
    let mut gname: *mut i8 = 0 as *mut i8; /* first try composing glyph */
    let mut suffix: *mut i8 = 0 as *mut i8;
    let mut gids: [USHORT; 32] = [0; 32];
    let mut nptrs: [*mut i8; 32] = [0 as *mut i8; 32];
    let mut i: libc::c_int = 0;
    let mut n_comp: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    error = findposttable(glyphname, gid, gm);
    if error == 0 {
        return 0i32;
    }
    gname = new(
        (strlen(glyphname).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32,
    ) as *mut i8;
    strcpy(gname, glyphname);
    memset(
        gids.as_mut_ptr() as *mut libc::c_void,
        0i32,
        (32i32 as u64).wrapping_mul(::std::mem::size_of::<USHORT>() as u64),
    );
    n_comp = agl_decompose_glyphname(gname, nptrs.as_mut_ptr(), 32i32, &mut suffix);
    error = 0i32;
    i = 0i32;
    while error == 0 && i < n_comp {
        error = resolve_glyph(
            nptrs[i as usize],
            &mut *gids.as_mut_ptr().offset(i as isize),
            gm,
        );
        if error != 0 {
            dpx_warning(
                b"Could not resolve glyph \"%s\" (%dth component of glyph \"%s\").\x00" as *const u8
                    as *const i8,
                nptrs[i as usize],
                i,
                glyphname,
            );
        }
        i += 1
    }
    if error == 0 {
        if !suffix.is_null()
            && (streq_ptr(suffix, b"liga\x00" as *const u8 as *const i8) as libc::c_int
                != 0
                || streq_ptr(suffix, b"dlig\x00" as *const u8 as *const i8)
                    as libc::c_int
                    != 0
                || streq_ptr(suffix, b"hlig\x00" as *const u8 as *const i8)
                    as libc::c_int
                    != 0
                || streq_ptr(suffix, b"frac\x00" as *const u8 as *const i8)
                    as libc::c_int
                    != 0
                || streq_ptr(suffix, b"ccmp\x00" as *const u8 as *const i8)
                    as libc::c_int
                    != 0
                || streq_ptr(suffix, b"afrc\x00" as *const u8 as *const i8)
                    as libc::c_int
                    != 0)
        {
            error = composeglyph(gids.as_mut_ptr(), n_comp, suffix, gm, gid)
        } else {
            error = composeglyph(gids.as_mut_ptr(), n_comp, 0 as *const i8, gm, gid);
            if error == 0 && !suffix.is_null() {
                /* a_b_c.vert */
                error = selectglyph(*gid, suffix, gm, gid)
            }
        }
    }
    free(gname as *mut libc::c_void);
    return error;
}
/* glyphname should not have suffix here */
unsafe extern "C" fn findparanoiac(
    mut glyphname: *const i8,
    mut gid: *mut USHORT,
    mut gm: *mut glyph_mapper,
) -> libc::c_int {
    let mut agln: *mut agl_name = 0 as *mut agl_name;
    let mut idx: USHORT = 0u32 as USHORT;
    let mut error: libc::c_int = 0i32;
    agln = agl_lookup_list(glyphname);
    while !agln.is_null() && idx as libc::c_int == 0i32 {
        if !(*agln).suffix.is_null() {
            error = findparanoiac((*agln).name, &mut idx, gm);
            if error != 0 {
                return error;
            }
            error = selectglyph(idx, (*agln).suffix, gm, &mut idx);
            if error != 0 {
                dpx_warning(
                    b"Variant \"%s\" for glyph \"%s\" might not be found.\x00" as *const u8
                        as *const i8,
                    (*agln).suffix,
                    (*agln).name,
                );
                dpx_warning(
                    b"Using glyph name without suffix instead...\x00" as *const u8
                        as *const i8,
                );
                error = 0i32
                /* ignore */
            }
        } else if (*agln).n_components == 1i32 {
            idx = tt_cmap_lookup((*gm).codetogid, (*agln).unicodes[0] as SFNT_ULONG)
        } else if (*agln).n_components > 1i32 {
            if verbose >= 0i32 {
                /* give warning */
                dpx_warning(
                    b"Glyph \"%s\" looks like a composite glyph...\x00" as *const u8
                        as *const i8,
                    (*agln).name,
                );
            }
            error = composeuchar(
                (*agln).unicodes.as_mut_ptr(),
                (*agln).n_components,
                0 as *const i8,
                gm,
                &mut idx,
            );
            if verbose >= 0i32 {
                if error != 0 {
                    dpx_warning(b"Not found...\x00" as *const u8 as *const i8);
                } else {
                    let mut _i: libc::c_int = 0;
                    let mut _n: libc::c_int = 0i32;
                    let mut _p: *mut i8 = 0 as *mut i8;
                    let mut _buf: [i8; 256] = [0; 256];
                    dpx_warning(
                        b">> Composite glyph glyph-name=\"%s\" found at glyph-id=\"%u\".\x00"
                            as *const u8 as *const i8,
                        (*agln).name,
                        idx as libc::c_int,
                    );
                    _p = _buf.as_mut_ptr();
                    _i = 0i32;
                    while _i < (*agln).n_components && _n < 245i32 {
                        let fresh2 = _n;
                        _n = _n + 1;
                        *_p.offset(fresh2 as isize) =
                            (if _i == 0i32 { '<' as i32 } else { ' ' as i32 }) as i8;
                        if (*agln).unicodes[_i as usize] >= 0x10000i32 {
                            _n += sprintf(
                                _p.offset(_n as isize),
                                b"U+%06X\x00" as *const u8 as *const i8,
                                (*agln).unicodes[_i as usize],
                            )
                        } else {
                            _n += sprintf(
                                _p.offset(_n as isize),
                                b"U+%04X\x00" as *const u8 as *const i8,
                                (*agln).unicodes[_i as usize],
                            )
                        }
                        let fresh3 = _n;
                        _n = _n + 1;
                        *_p.offset(fresh3 as isize) = (if _i == (*agln).n_components - 1i32 {
                            '>' as i32
                        } else {
                            ',' as i32
                        }) as i8;
                        _i += 1
                    }
                    let fresh4 = _n;
                    _n = _n + 1;
                    *_p.offset(fresh4 as isize) = '\u{0}' as i32 as i8;
                    dpx_warning(b">> Input Unicode seq.=\"%s\" ==> glyph-id=\"%u\" in font-file=\"_please_try_-v_\".\x00"
                                    as *const u8 as *const i8,
                                _buf.as_mut_ptr(), idx as libc::c_int);
                }
            }
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const i8,
                b"dpx-truetype.c\x00" as *const u8 as *const i8,
                670i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 65], &[i8; 65]>(
                    b"int findparanoiac(const char *, USHORT *, struct glyph_mapper *)\x00",
                ))
                .as_ptr(),
            );
        }
        agln = (*agln).alternate
    }
    *gid = idx;
    return if idx as libc::c_int == 0i32 {
        -1i32
    } else {
        0i32
    };
}
unsafe extern "C" fn resolve_glyph(
    mut glyphname: *const i8,
    mut gid: *mut USHORT,
    mut gm: *mut glyph_mapper,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut suffix: *mut i8 = 0 as *mut i8;
    let mut ucv: i32 = 0;
    if !glyphname.is_null() {
    } else {
        __assert_fail(
            b"glyphname\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            686i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 65], &[i8; 65]>(
                b"int resolve_glyph(const char *, USHORT *, struct glyph_mapper *)\x00",
            ))
            .as_ptr(),
        );
    }
    /* Boooo */
    /*
     * First we try glyph name to GID mapping using post table if post table
     * is available. If post table is not available or glyph is not listed
     * in the post table, then we try Unicode if Windows-Unicode TrueType
     * cmap is available.
     */
    error = findposttable(glyphname, gid, gm);
    if error == 0 {
        return 0i32;
    }
    if (*gm).codetogid.is_null() {
        return -1i32;
    }
    name = agl_chop_suffix(glyphname, &mut suffix);
    if name.is_null() {
        /* .notdef, .foo */
        error = -1i32
    } else if agl_name_is_unicode(name) {
        ucv = agl_name_convert_unicode(name);
        *gid = tt_cmap_lookup((*gm).codetogid, ucv as SFNT_ULONG);
        error = if *gid as libc::c_int == 0i32 {
            -1i32
        } else {
            0i32
        }
    } else {
        error = findparanoiac(name, gid, gm)
    }
    if error == 0 && !suffix.is_null() {
        error = selectglyph(*gid, suffix, gm, gid);
        if error != 0 {
            dpx_warning(
                b"Variant \"%s\" for glyph \"%s\" might not be found.\x00" as *const u8
                    as *const i8,
                suffix,
                name,
            );
            dpx_warning(
                b"Using glyph name without suffix instead...\x00" as *const u8
                    as *const i8,
            );
            error = 0i32
            /* ignore */
        }
    }
    free(suffix as *mut libc::c_void);
    free(name as *mut libc::c_void);
    return error;
}
/* Things are complicated. We still need to use PostScript
 * glyph names. But OpenType fonts may not have PS name to
 * glyph mapping. We use Unicode plus OTL GSUB for finding
 * glyphs in this case.
 */
unsafe extern "C" fn setup_glyph_mapper(
    mut gm: *mut glyph_mapper,
    mut sfont: *mut sfnt,
) -> libc::c_int {
    (*gm).sfont = sfont;
    (*gm).nametogid = tt_read_post_table(sfont);
    (*gm).codetogid = tt_cmap_read(sfont, 3u32 as USHORT, 10u32 as USHORT);
    if (*gm).codetogid.is_null() {
        (*gm).codetogid = tt_cmap_read(sfont, 3u32 as USHORT, 1u32 as USHORT)
    }
    if (*gm).nametogid.is_null() && (*gm).codetogid.is_null() {
        return -1i32;
    }
    (*gm).gsub = otl_gsub_new();
    return 0i32;
}
unsafe extern "C" fn clean_glyph_mapper(mut gm: *mut glyph_mapper) {
    if !(*gm).gsub.is_null() {
        otl_gsub_release((*gm).gsub);
    }
    if !(*gm).codetogid.is_null() {
        tt_cmap_release((*gm).codetogid);
    }
    if !(*gm).nametogid.is_null() {
        tt_release_post_table((*gm).nametogid);
    }
    (*gm).gsub = 0 as *mut otl_gsub;
    (*gm).codetogid = 0 as *mut tt_cmap;
    (*gm).nametogid = 0 as *mut tt_post_table;
    (*gm).sfont = 0 as *mut sfnt;
}
unsafe extern "C" fn do_custom_encoding(
    mut font: *mut pdf_font,
    mut encoding: *mut *mut i8,
    mut usedchars: *const i8,
    mut sfont: *mut sfnt,
) -> libc::c_int {
    let mut glyphs: *mut tt_glyphs = 0 as *mut tt_glyphs;
    let mut cmap_table: *mut i8 = 0 as *mut i8;
    let mut code: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut widths: [libc::c_double; 256] = [0.; 256];
    let mut gm: glyph_mapper = glyph_mapper {
        codetogid: 0 as *mut tt_cmap,
        gsub: 0 as *mut otl_gsub,
        sfont: 0 as *mut sfnt,
        nametogid: 0 as *mut tt_post_table,
    };
    let mut idx: USHORT = 0;
    let mut gid: USHORT = 0;
    let mut error: libc::c_int = 0i32;
    if !font.is_null() && !encoding.is_null() && !usedchars.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"font && encoding && usedchars && sfont\x00" as *const u8 as *const i8,
            b"dpx-truetype.c\x00" as *const u8 as *const i8,
            778i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 66], &[i8; 66]>(
                b"int do_custom_encoding(pdf_font *, char **, const char *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    error = setup_glyph_mapper(&mut gm, sfont);
    if error != 0 {
        dpx_warning(
            b"No post table nor Unicode cmap found in font: %s\x00" as *const u8
                as *const i8,
            pdf_font_get_ident(font),
        );
        dpx_warning(
            b">> I can\'t find glyphs without this!\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    cmap_table = new((274i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    memset(
        cmap_table as *mut libc::c_void,
        0i32,
        274i32 as u64,
    );
    put_big_endian(cmap_table as *mut libc::c_void, 0i32, 2i32);
    /* Version  */
    put_big_endian(cmap_table.offset(2) as *mut libc::c_void, 1i32, 2i32);
    /* Number of subtables */
    put_big_endian(
        cmap_table.offset(4) as *mut libc::c_void,
        1u32 as SFNT_LONG,
        2i32,
    );
    /* Platform ID */
    put_big_endian(
        cmap_table.offset(6) as *mut libc::c_void,
        0u32 as SFNT_LONG,
        2i32,
    );
    /* Encoding ID */
    put_big_endian(cmap_table.offset(8) as *mut libc::c_void, 12i32, 4i32);
    /* Offset   */
    put_big_endian(cmap_table.offset(12) as *mut libc::c_void, 0i32, 2i32);
    /* Format   */
    put_big_endian(cmap_table.offset(14) as *mut libc::c_void, 262i32, 2i32);
    /* Length   */
    put_big_endian(cmap_table.offset(16) as *mut libc::c_void, 0i32, 2i32);
    /* Language */
    glyphs = tt_build_init(); /* +1 for .notdef */
    count = 1i32;
    code = 0i32;
    while code < 256i32 {
        if !(*usedchars.offset(code as isize) == 0) {
            if (*encoding.offset(code as isize)).is_null()
                || streq_ptr(
                    *encoding.offset(code as isize),
                    b".notdef\x00" as *const u8 as *const i8,
                ) as libc::c_int
                    != 0
            {
                dpx_warning(b"Character code=\"0x%02X\" mapped to \".notdef\" glyph used in font font-file=\"%s\"\x00"
                                as *const u8 as *const i8, code,
                            pdf_font_get_ident(font));
                dpx_warning(
                    b">> Maybe incorrect encoding specified?\x00" as *const u8
                        as *const i8,
                );
                idx = 0i32 as USHORT
            } else {
                if !strchr(*encoding.offset(code as isize), '_' as i32).is_null() {
                    error = findcomposite(*encoding.offset(code as isize), &mut gid, &mut gm)
                } else {
                    error = resolve_glyph(*encoding.offset(code as isize), &mut gid, &mut gm)
                }
                /*
                 * Older versions of gs had problem with glyphs (other than .notdef)
                 * mapped to gid = 0.
                 */
                if error != 0 {
                    dpx_warning(
                        b"Glyph \"%s\" not available in font \"%s\".\x00" as *const u8
                            as *const i8,
                        *encoding.offset(code as isize),
                        pdf_font_get_ident(font),
                    ); /* count returned. */
                } else if verbose > 1i32 {
                    dpx_message(
                        b"truetype>> Glyph glyph-name=\"%s\" found at glyph-id=\"%u\".\n\x00"
                            as *const u8 as *const i8,
                        *encoding.offset(code as isize),
                        gid as libc::c_int,
                    );
                }
                idx = tt_find_glyph(glyphs, gid);
                if idx as libc::c_int == 0i32 {
                    idx = tt_add_glyph(glyphs, gid, count as USHORT);
                    count += 1
                }
            }
            *cmap_table.offset((18i32 + code) as isize) =
                (idx as libc::c_int & 0xffi32) as i8
        }
        code += 1
        /* bug here */
    } /* _FIXME_: wrong message */
    clean_glyph_mapper(&mut gm);
    if tt_build_tables(sfont, glyphs) < 0i32 {
        dpx_warning(
            b"Packing TrueType font into SFNT file faild...\x00" as *const u8
                as *const i8,
        );
        tt_build_finish(glyphs);
        free(cmap_table as *mut libc::c_void);
        return -1i32;
    }
    code = 0i32;
    while code < 256i32 {
        if *usedchars.offset(code as isize) != 0 {
            idx = tt_get_index(
                glyphs,
                *cmap_table.offset((18i32 + code) as isize) as USHORT,
            );
            widths[code as usize] = floor(
                1000.0f64
                    * (*(*glyphs).gd.offset(idx as isize)).advw as libc::c_int as libc::c_double
                    / (*glyphs).emsize as libc::c_int as libc::c_double
                    / 1i32 as libc::c_double
                    + 0.5f64,
            ) * 1i32 as libc::c_double
        } else {
            widths[code as usize] = 0.0f64
        }
        code += 1
    }
    do_widths(font, widths.as_mut_ptr());
    if verbose > 1i32 {
        dpx_message(
            b"[%d glyphs]\x00" as *const u8 as *const i8,
            (*glyphs).num_glyphs as libc::c_int,
        );
    }
    tt_build_finish(glyphs);
    sfnt_set_table(
        sfont,
        b"cmap\x00" as *const u8 as *const i8,
        cmap_table as *mut libc::c_void,
        274i32 as SFNT_ULONG,
    );
    return 0i32;
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
#[no_mangle]
pub unsafe extern "C" fn pdf_font_load_truetype(mut font: *mut pdf_font) -> libc::c_int {
    let mut descriptor: *mut pdf_obj = pdf_font_get_descriptor(font);
    let mut ident: *mut i8 = pdf_font_get_ident(font);
    let mut encoding_id: libc::c_int = pdf_font_get_encoding(font);
    let mut usedchars: *mut i8 = pdf_font_get_usedchars(font);
    /* ENABLE_NOEMBED */
    let mut index: libc::c_int = pdf_font_get_index(font); /* Should find *truetype* here */
    let mut enc_vec: *mut *mut i8 = 0 as *mut *mut i8;
    let mut fontfile: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    if !pdf_font_is_in_use(font) {
        return 0i32;
    }
    verbose = pdf_font_get_verbose();
    handle = dpx_open_truetype_file(ident) as *mut rust_input_handle_t;
    if handle.is_null() {
        handle = dpx_open_dfont_file(ident) as *mut rust_input_handle_t;
        if handle.is_null() {
            _tt_abort(
                b"Unable to open TrueType/dfont font file: %s\x00" as *const u8
                    as *const i8,
                ident,
            );
        }
        sfont = dfont_open(handle as rust_input_handle_t, index)
    } else {
        sfont = sfnt_open(handle as rust_input_handle_t)
    }
    if sfont.is_null() {
        ttstub_input_close(handle as rust_input_handle_t);
        _tt_abort(
            b"Unable to open TrueType/dfont file: %s\x00" as *const u8 as *const i8,
            ident,
        );
    } else {
        if (*sfont).type_0 != 1i32 << 0i32
            && (*sfont).type_0 != 1i32 << 4i32
            && (*sfont).type_0 != 1i32 << 8i32
        {
            sfnt_close(sfont);
            ttstub_input_close(handle as rust_input_handle_t);
            _tt_abort(
                b"Font \"%s\" not a TrueType/dfont font?\x00" as *const u8 as *const i8,
                ident,
            );
        }
    }
    if (*sfont).type_0 == 1i32 << 4i32 {
        let mut offset: SFNT_ULONG = 0;
        offset = ttc_read_offset(sfont, index);
        if offset == 0i32 as libc::c_uint {
            _tt_abort(
                b"Invalid TTC index in %s.\x00" as *const u8 as *const i8,
                ident,
            );
        }
        error = sfnt_read_table_directory(sfont, offset)
    } else {
        error = sfnt_read_table_directory(sfont, (*sfont).offset)
    }
    if error != 0 {
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        _tt_abort(
            b"Reading SFND table dir failed for font-file=\"%s\"... Not a TrueType font?\x00"
                as *const u8 as *const i8,
            ident,
        );
    }
    /*
     * Create new TrueType cmap table with MacRoman encoding.
     */
    if encoding_id < 0i32 {
        error = do_builtin_encoding(font, usedchars, sfont)
    } else {
        enc_vec = pdf_encoding_get_encoding(encoding_id);
        error = do_custom_encoding(font, enc_vec, usedchars, sfont)
    }
    if error != 0 {
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        _tt_abort(
            b"Error occured while creating font subfont for \"%s\"\x00" as *const u8
                as *const i8,
            ident,
        );
    }
    /* ENABLE_NOEMBED */
    /*
     * TODO: post table?
     */
    i = 0i32;
    while !required_table[i as usize].name.is_null() {
        if sfnt_require_table(
            sfont,
            required_table[i as usize].name,
            required_table[i as usize].must_exist,
        ) < 0i32
        {
            sfnt_close(sfont);
            ttstub_input_close(handle as rust_input_handle_t);
            _tt_abort(
                b"Required TrueType table \"%s\" does not exist in font: %s\x00" as *const u8
                    as *const i8,
                required_table[i as usize].name,
                ident,
            );
        }
        i += 1
    }
    /*
     * FontFile2
     */
    fontfile = sfnt_create_FontFile_stream(sfont); /* XXX */
    if fontfile.is_null() {
        _tt_abort(
            b"Could not created FontFile stream for \"%s\".\x00" as *const u8
                as *const i8,
            ident,
        );
    }
    sfnt_close(sfont);
    ttstub_input_close(handle as rust_input_handle_t);
    if verbose > 1i32 {
        dpx_message(
            b"[%d bytes]\x00" as *const u8 as *const i8,
            pdf_stream_length(fontfile),
        );
    }
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontFile2\x00" as *const u8 as *const i8),
        pdf_ref_obj(fontfile),
    );
    pdf_release_obj(fontfile);
    return 0i32;
}
