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
    fn tt_get_positive_quad(
        handle: rust_input_handle_t,
        type_0: *const libc::c_char,
        name: *const libc::c_char,
    ) -> u32;
    #[no_mangle]
    fn tt_get_signed_quad(handle: rust_input_handle_t) -> int32_t;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_skip_bytes(n: libc::c_uint, handle: rust_input_handle_t);
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
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
    fn dpx_message(fmt: *const libc::c_char, _: ...);
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
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type size_t = u64;
pub type off_t = __off_t;
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
pub type fixword = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_metric {
    pub tex_name: *mut libc::c_char,
    pub designsize: fixword,
    pub codingscheme: *mut libc::c_char,
    pub fontdir: libc::c_int,
    pub firstchar: libc::c_int,
    pub lastchar: libc::c_int,
    pub widths: *mut fixword,
    pub heights: *mut fixword,
    pub depths: *mut fixword,
    pub charmap: C2RustUnnamed,
    pub source: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub type_0: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coverage {
    pub first_char: libc::c_int,
    pub num_chars: libc::c_int,
}
/* quasi-hack to get the primary input */
/*
 * TFM Record structure:
 * Multiple TFM's may be read in at once.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfm_font {
    pub level: int32_t,
    pub wlenfile: u32,
    pub wlenheader: u32,
    pub bc: u32,
    pub ec: u32,
    pub nwidths: u32,
    pub nheights: u32,
    pub ndepths: u32,
    pub nitcor: u32,
    pub nlig: u32,
    pub nkern: u32,
    pub nextens: u32,
    pub nfonparm: u32,
    pub fontdir: u32,
    pub nco: u32,
    pub ncw: u32,
    pub npc: u32,
    pub header: *mut fixword,
    pub char_info: *mut u32,
    pub width_index: *mut u16,
    pub height_index: *mut u8,
    pub depth_index: *mut u8,
    pub width: *mut fixword,
    pub height: *mut fixword,
    pub depth: *mut fixword,
}
/*
 * All characters in the same range have same metrics.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct range_map {
    pub num_coverages: u16,
    pub coverages: *mut coverage,
    pub indices: *mut u16,
}
/* Special case of num_coverages = 1 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_map {
    pub coverage: coverage,
    pub indices: *mut u16,
}
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
static mut verbose: libc::c_int = 0i32;
unsafe extern "C" fn tfm_font_init(mut tfm: *mut tfm_font) {
    (*tfm).header = 0 as *mut fixword;
    (*tfm).level = 0i32;
    (*tfm).fontdir = 0i32 as u32;
    (*tfm).npc = 0i32 as u32;
    (*tfm).ncw = (*tfm).npc;
    (*tfm).nco = (*tfm).ncw;
    (*tfm).char_info = 0 as *mut u32;
    (*tfm).width_index = 0 as *mut u16;
    (*tfm).height_index = 0 as *mut u8;
    (*tfm).depth_index = 0 as *mut u8;
    (*tfm).depth = 0 as *mut fixword;
    (*tfm).height = (*tfm).depth;
    (*tfm).width = (*tfm).height;
}
unsafe extern "C" fn tfm_font_clear(mut tfm: *mut tfm_font) {
    if !tfm.is_null() {
        (*tfm).header = mfree((*tfm).header as *mut libc::c_void) as *mut fixword;
        (*tfm).char_info = mfree((*tfm).char_info as *mut libc::c_void) as *mut u32;
        (*tfm).width = mfree((*tfm).width as *mut libc::c_void) as *mut fixword;
        (*tfm).height = mfree((*tfm).height as *mut libc::c_void) as *mut fixword;
        (*tfm).depth = mfree((*tfm).depth as *mut libc::c_void) as *mut fixword;
        (*tfm).width_index = mfree((*tfm).width_index as *mut libc::c_void) as *mut u16;
        (*tfm).height_index = mfree((*tfm).height_index as *mut libc::c_void) as *mut u8;
        (*tfm).depth_index = mfree((*tfm).depth_index as *mut libc::c_void) as *mut u8
    };
}
unsafe extern "C" fn release_char_map(mut map: *mut char_map) {
    (*map).indices = mfree((*map).indices as *mut libc::c_void) as *mut u16;
    free(map as *mut libc::c_void);
}
unsafe extern "C" fn release_range_map(mut map: *mut range_map) {
    free((*map).coverages as *mut libc::c_void);
    free((*map).indices as *mut libc::c_void);
    (*map).coverages = 0 as *mut coverage;
    (*map).indices = 0 as *mut u16;
    free(map as *mut libc::c_void);
}
unsafe extern "C" fn lookup_char(
    mut map: *const char_map,
    mut charcode: libc::c_int,
) -> libc::c_int {
    if charcode >= (*map).coverage.first_char
        && charcode <= (*map).coverage.first_char + (*map).coverage.num_chars
    {
        return *(*map)
            .indices
            .offset((charcode - (*map).coverage.first_char) as isize)
            as libc::c_int;
    } else {
        return -1i32;
    };
}
unsafe extern "C" fn lookup_range(
    mut map: *const range_map,
    mut charcode: libc::c_int,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    idx = (*map).num_coverages as libc::c_int - 1i32;
    while idx >= 0i32 && charcode >= (*(*map).coverages.offset(idx as isize)).first_char {
        if charcode
            <= (*(*map).coverages.offset(idx as isize)).first_char
                + (*(*map).coverages.offset(idx as isize)).num_chars
        {
            return *(*map).indices.offset(idx as isize) as libc::c_int;
        }
        idx -= 1
    }
    return -1i32;
}
unsafe extern "C" fn fm_init(mut fm: *mut font_metric) {
    (*fm).tex_name = 0 as *mut libc::c_char;
    (*fm).firstchar = 0i32;
    (*fm).lastchar = 0i32;
    (*fm).fontdir = 0i32;
    (*fm).codingscheme = 0 as *mut libc::c_char;
    (*fm).designsize = 0i32;
    (*fm).widths = 0 as *mut fixword;
    (*fm).heights = 0 as *mut fixword;
    (*fm).depths = 0 as *mut fixword;
    (*fm).charmap.type_0 = 0i32;
    (*fm).charmap.data = 0 as *mut libc::c_void;
    (*fm).source = 0i32;
}
unsafe extern "C" fn fm_clear(mut fm: *mut font_metric) {
    if !fm.is_null() {
        free((*fm).tex_name as *mut libc::c_void);
        free((*fm).widths as *mut libc::c_void);
        free((*fm).heights as *mut libc::c_void);
        free((*fm).depths as *mut libc::c_void);
        free((*fm).codingscheme as *mut libc::c_void);
        match (*fm).charmap.type_0 {
            1 => {
                release_char_map((*fm).charmap.data as *mut char_map);
            }
            2 => {
                release_range_map((*fm).charmap.data as *mut range_map);
            }
            _ => {}
        }
    };
}
static mut fms: *mut font_metric = 0 as *const font_metric as *mut font_metric;
static mut numfms: libc::c_uint = 0i32 as libc::c_uint;
static mut max_fms: libc::c_uint = 0i32 as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn tfm_reset_global_state() {
    fms = 0 as *mut font_metric;
    numfms = 0i32 as libc::c_uint;
    max_fms = 0i32 as libc::c_uint;
}
unsafe extern "C" fn fms_need(mut n: libc::c_uint) {
    if n > max_fms {
        max_fms = if max_fms.wrapping_add(16i32 as libc::c_uint) > n {
            max_fms.wrapping_add(16i32 as libc::c_uint)
        } else {
            n
        };
        fms = renew(
            fms as *mut libc::c_void,
            (max_fms as u64)
                .wrapping_mul(::std::mem::size_of::<font_metric>() as u64)
                as u32,
        ) as *mut font_metric
    };
}
#[no_mangle]
pub unsafe extern "C" fn tfm_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
unsafe extern "C" fn fread_fwords(
    mut words: *mut fixword,
    mut nmemb: u32,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut i: u32 = 0;
    i = 0i32 as u32;
    while i < nmemb {
        *words.offset(i as isize) = tt_get_signed_quad(handle);
        i = i.wrapping_add(1)
    }
    return nmemb.wrapping_mul(4i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn fread_uquads(
    mut quads: *mut u32,
    mut nmemb: u32,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut i: u32 = 0;
    i = 0i32 as u32;
    while i < nmemb {
        *quads.offset(i as isize) = tt_get_unsigned_quad(handle);
        i = i.wrapping_add(1)
    }
    return nmemb.wrapping_mul(4i32 as libc::c_uint) as libc::c_int;
}
/*
 * TFM and JFM
 */
unsafe extern "C" fn tfm_check_size(mut tfm: *mut tfm_font, mut tfm_file_size: off_t) {
    let mut expected_size: u32 = 6i32 as u32;
    /* Removed the warning message caused by EC TFM metric files.
     *
     if (tfm->wlenfile != tfm_file_size / 4) {
     dpx_warning("TFM file size is %ld bytes but it says it is %ld bytes!",
     tfm_file_size, tfm->wlenfile * 4);
     if (tfm_file_size > tfm->wlenfile * 4) {
     dpx_warning("Proceeding nervously...");
     } else {
     _tt_abort("Can't proceed...");
     }
     }
    */
    if tfm_file_size < (*tfm).wlenfile as int64_t * 4i32 as libc::c_long {
        _tt_abort(b"Can\'t proceed...\x00" as *const u8 as *const libc::c_char);
    }
    expected_size = (expected_size as libc::c_uint).wrapping_add(
        (*tfm)
            .ec
            .wrapping_sub((*tfm).bc)
            .wrapping_add(1i32 as libc::c_uint),
    ) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).wlenheader) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nwidths) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nheights) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).ndepths) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nitcor) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nlig) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nkern) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nextens) as u32;
    expected_size =
        (expected_size as libc::c_uint).wrapping_add((*tfm).nfonparm) as u32;
    if expected_size != (*tfm).wlenfile {
        dpx_warning(
            b"TFM file size is expected to be %ld bytes but it says it is %ldbytes!\x00"
                as *const u8 as *const libc::c_char,
            expected_size as int64_t * 4i32 as libc::c_long,
            (*tfm).wlenfile as int64_t * 4i32 as libc::c_long,
        );
        if tfm_file_size > expected_size as int64_t * 4i32 as libc::c_long {
            dpx_warning(b"Proceeding nervously...\x00" as *const u8 as *const libc::c_char);
        } else {
            _tt_abort(b"Can\'t proceed...\x00" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn tfm_get_sizes(
    mut tfm_handle: rust_input_handle_t,
    mut tfm_file_size: off_t,
    mut tfm: *mut tfm_font,
) {
    (*tfm).wlenfile = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).wlenheader = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).bc = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).ec = tt_get_unsigned_pair(tfm_handle) as u32;
    if (*tfm).ec < (*tfm).bc {
        _tt_abort(
            b"TFM file error: ec(%u) < bc(%u) ???\x00" as *const u8 as *const libc::c_char,
            (*tfm).ec,
            (*tfm).bc,
        );
    }
    (*tfm).nwidths = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).nheights = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).ndepths = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).nitcor = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).nlig = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).nkern = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).nextens = tt_get_unsigned_pair(tfm_handle) as u32;
    (*tfm).nfonparm = tt_get_unsigned_pair(tfm_handle) as u32;
    tfm_check_size(tfm, tfm_file_size);
}
unsafe extern "C" fn tfm_unpack_arrays(mut fm: *mut font_metric, mut tfm: *mut tfm_font) {
    let mut charinfo: u32 = 0;
    let mut width_index: u16 = 0;
    let mut height_index: u8 = 0;
    let mut depth_index: u8 = 0;
    let mut i: u32 = 0;
    (*fm).widths = new((256i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
        as u32) as *mut fixword;
    (*fm).heights = new((256i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
        as u32) as *mut fixword;
    (*fm).depths = new((256i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
        as u32) as *mut fixword;
    i = 0i32 as u32;
    while i < 256i32 as libc::c_uint {
        *(*fm).widths.offset(i as isize) = 0i32;
        *(*fm).heights.offset(i as isize) = 0i32;
        *(*fm).depths.offset(i as isize) = 0i32;
        i = i.wrapping_add(1)
    }
    i = (*tfm).bc;
    while i <= (*tfm).ec {
        charinfo = *(*tfm).char_info.offset(i.wrapping_sub((*tfm).bc) as isize);
        width_index = (charinfo >> 24i32) as u16;
        height_index = (charinfo >> 20i32 & 0xfi32 as libc::c_uint) as u8;
        depth_index = (charinfo >> 16i32 & 0xfi32 as libc::c_uint) as u8;
        *(*fm).widths.offset(i as isize) = *(*tfm).width.offset(width_index as isize);
        *(*fm).heights.offset(i as isize) = *(*tfm).height.offset(height_index as isize);
        *(*fm).depths.offset(i as isize) = *(*tfm).depth.offset(depth_index as isize);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn sput_bigendian(
    mut s: *mut libc::c_char,
    mut v: int32_t,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = n - 1i32;
    while i >= 0i32 {
        *s.offset(i as isize) = (v & 0xffi32) as libc::c_char;
        v >>= 8i32;
        i -= 1
    }
    return n;
}
unsafe extern "C" fn tfm_unpack_header(mut fm: *mut font_metric, mut tfm: *mut tfm_font) {
    if (*tfm).wlenheader < 12i32 as libc::c_uint {
        (*fm).codingscheme = 0 as *mut libc::c_char
    } else {
        let mut i: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        len = *(*tfm).header.offset(2) >> 24i32;
        if len < 0i32 || len > 39i32 {
            _tt_abort(b"Invalid TFM header.\x00" as *const u8 as *const libc::c_char);
        }
        if len > 0i32 {
            (*fm).codingscheme = new((40i32 as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
                as u32) as *mut libc::c_char;
            p = (*fm).codingscheme;
            p = p.offset(sput_bigendian(p, *(*tfm).header.offset(2), 3i32) as isize);
            i = 1i32;
            while i <= len / 4i32 {
                p = p.offset(
                    sput_bigendian(p, *(*tfm).header.offset((2i32 + i) as isize), 4i32) as isize,
                );
                i += 1
            }
            *(*fm).codingscheme.offset(len as isize) = '\u{0}' as i32 as libc::c_char
        } else {
            (*fm).codingscheme = 0 as *mut libc::c_char
        }
    }
    (*fm).designsize = *(*tfm).header.offset(1);
}
unsafe extern "C" fn ofm_check_size_one(mut tfm: *mut tfm_font, mut ofm_file_size: off_t) {
    let mut ofm_size: u32 = 14i32 as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add(
        (2i32 as libc::c_uint).wrapping_mul(
            (*tfm)
                .ec
                .wrapping_sub((*tfm).bc)
                .wrapping_add(1i32 as libc::c_uint),
        ),
    ) as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).wlenheader) as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).nwidths) as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).nheights) as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).ndepths) as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).nitcor) as u32;
    ofm_size = (ofm_size as libc::c_uint)
        .wrapping_add((2i32 as libc::c_uint).wrapping_mul((*tfm).nlig)) as u32
        as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).nkern) as u32;
    ofm_size = (ofm_size as libc::c_uint)
        .wrapping_add((2i32 as libc::c_uint).wrapping_mul((*tfm).nextens))
        as u32;
    ofm_size = (ofm_size as libc::c_uint).wrapping_add((*tfm).nfonparm) as u32;
    if (*tfm).wlenfile as libc::c_long != ofm_file_size / 4i32 as libc::c_long
        || (*tfm).wlenfile != ofm_size
    {
        _tt_abort(
            b"OFM file problem.  Table sizes don\'t agree.\x00" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn ofm_get_sizes(
    mut ofm_handle: rust_input_handle_t,
    mut ofm_file_size: off_t,
    mut tfm: *mut tfm_font,
) {
    (*tfm).level = tt_get_signed_quad(ofm_handle);
    (*tfm).wlenfile = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"wlenfile\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).wlenheader = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"wlenheader\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).bc = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"bc\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).ec = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"ec\x00" as *const u8 as *const libc::c_char,
    );
    if (*tfm).ec < (*tfm).bc {
        _tt_abort(
            b"OFM file error: ec(%u) < bc(%u) ???\x00" as *const u8 as *const libc::c_char,
            (*tfm).ec,
            (*tfm).bc,
        );
    }
    (*tfm).nwidths = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nwidths\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).nheights = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nheights\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).ndepths = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"ndepths\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).nitcor = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nitcor\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).nlig = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nlig\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).nkern = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nkern\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).nextens = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nextens\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).nfonparm = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"nfonparm\x00" as *const u8 as *const libc::c_char,
    );
    (*tfm).fontdir = tt_get_positive_quad(
        ofm_handle,
        b"OFM\x00" as *const u8 as *const libc::c_char,
        b"fontdir\x00" as *const u8 as *const libc::c_char,
    );
    if (*tfm).fontdir != 0 {
        dpx_warning(
            b"I may be interpreting a font direction incorrectly.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if (*tfm).level == 0i32 {
        ofm_check_size_one(tfm, ofm_file_size);
    } else if (*tfm).level == 1i32 {
        (*tfm).nco = tt_get_positive_quad(
            ofm_handle,
            b"OFM\x00" as *const u8 as *const libc::c_char,
            b"nco\x00" as *const u8 as *const libc::c_char,
        );
        (*tfm).ncw = tt_get_positive_quad(
            ofm_handle,
            b"OFM\x00" as *const u8 as *const libc::c_char,
            b"nco\x00" as *const u8 as *const libc::c_char,
        );
        (*tfm).npc = tt_get_positive_quad(
            ofm_handle,
            b"OFM\x00" as *const u8 as *const libc::c_char,
            b"npc\x00" as *const u8 as *const libc::c_char,
        );
        ttstub_input_seek(
            ofm_handle,
            4i32 as libc::c_long * (*tfm).nco.wrapping_sub((*tfm).wlenheader) as off_t,
            0i32,
        );
    } else {
        _tt_abort(
            b"can\'t handle OFM files with level > 1\x00" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn ofm_do_char_info_zero(
    mut ofm_handle: rust_input_handle_t,
    mut tfm: *mut tfm_font,
) {
    let mut num_chars: u32 = 0;
    num_chars = (*tfm)
        .ec
        .wrapping_sub((*tfm).bc)
        .wrapping_add(1i32 as libc::c_uint);
    if num_chars != 0i32 as libc::c_uint {
        let mut i: u32 = 0;
        (*tfm).width_index = new((num_chars as u64)
            .wrapping_mul(::std::mem::size_of::<u16>() as u64)
            as u32) as *mut u16;
        (*tfm).height_index = new((num_chars as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64)
            as u32) as *mut u8;
        (*tfm).depth_index = new((num_chars as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64)
            as u32) as *mut u8;
        i = 0i32 as u32;
        while i < num_chars {
            *(*tfm).width_index.offset(i as isize) = tt_get_unsigned_pair(ofm_handle);
            *(*tfm).height_index.offset(i as isize) = tt_get_unsigned_byte(ofm_handle);
            *(*tfm).depth_index.offset(i as isize) = tt_get_unsigned_byte(ofm_handle);
            /* Ignore remaining quad */
            tt_skip_bytes(4i32 as libc::c_uint, ofm_handle);
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn ofm_do_char_info_one(
    mut ofm_handle: rust_input_handle_t,
    mut tfm: *mut tfm_font,
) {
    let mut num_char_infos: u32 = 0;
    let mut num_chars: u32 = 0;
    num_char_infos = (*tfm).ncw.wrapping_div(
        (3i32 as libc::c_uint).wrapping_add((*tfm).npc.wrapping_div(2i32 as libc::c_uint)),
    );
    num_chars = (*tfm)
        .ec
        .wrapping_sub((*tfm).bc)
        .wrapping_add(1i32 as libc::c_uint);
    if num_chars != 0i32 as libc::c_uint {
        let mut i: u32 = 0;
        let mut char_infos_read: u32 = 0;
        (*tfm).width_index = new((num_chars as u64)
            .wrapping_mul(::std::mem::size_of::<u16>() as u64)
            as u32) as *mut u16;
        (*tfm).height_index = new((num_chars as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64)
            as u32) as *mut u8;
        (*tfm).depth_index = new((num_chars as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64)
            as u32) as *mut u8;
        char_infos_read = 0i32 as u32;
        i = 0i32 as u32;
        while i < num_chars && char_infos_read < num_char_infos {
            let mut repeats: u32 = 0;
            let mut j: u32 = 0;
            *(*tfm).width_index.offset(i as isize) = tt_get_unsigned_pair(ofm_handle);
            *(*tfm).height_index.offset(i as isize) = tt_get_unsigned_byte(ofm_handle);
            *(*tfm).depth_index.offset(i as isize) = tt_get_unsigned_byte(ofm_handle);
            /* Ignore next quad */
            tt_skip_bytes(4i32 as libc::c_uint, ofm_handle);
            repeats = tt_get_unsigned_pair(ofm_handle) as u32;
            /* Skip params */
            j = 0i32 as u32;
            while j < (*tfm).npc {
                tt_get_unsigned_pair(ofm_handle);
                j = j.wrapping_add(1)
            }
            /* Remove word padding if necessary */
            if (*tfm)
                .npc
                .wrapping_div(2i32 as libc::c_uint)
                .wrapping_mul(2i32 as libc::c_uint)
                == (*tfm).npc
            {
                tt_get_unsigned_pair(ofm_handle);
            }
            char_infos_read = char_infos_read.wrapping_add(1);
            if i.wrapping_add(repeats) > num_chars {
                _tt_abort(
                    b"OFM \"repeats\" causes number of characters to be exceeded.\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            j = 0i32 as u32;
            while j < repeats {
                *(*tfm)
                    .width_index
                    .offset(i.wrapping_add(j).wrapping_add(1i32 as libc::c_uint) as isize) =
                    *(*tfm).width_index.offset(i as isize);
                *(*tfm)
                    .height_index
                    .offset(i.wrapping_add(j).wrapping_add(1i32 as libc::c_uint) as isize) =
                    *(*tfm).height_index.offset(i as isize);
                *(*tfm)
                    .depth_index
                    .offset(i.wrapping_add(j).wrapping_add(1i32 as libc::c_uint) as isize) =
                    *(*tfm).depth_index.offset(i as isize);
                j = j.wrapping_add(1)
            }
            /* Skip ahead because we have already handled repeats */
            i = (i as libc::c_uint).wrapping_add(repeats) as u32;
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn ofm_unpack_arrays(
    mut fm: *mut font_metric,
    mut tfm: *mut tfm_font,
    mut num_chars: u32,
) {
    let mut i: u32 = 0;
    (*fm).widths = new(((*tfm).bc.wrapping_add(num_chars) as u64)
        .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
        as u32) as *mut fixword;
    (*fm).heights = new(((*tfm).bc.wrapping_add(num_chars) as u64)
        .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
        as u32) as *mut fixword;
    (*fm).depths = new(((*tfm).bc.wrapping_add(num_chars) as u64)
        .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
        as u32) as *mut fixword;
    i = 0i32 as u32;
    while i < num_chars {
        *(*fm).widths.offset((*tfm).bc.wrapping_add(i) as isize) = *(*tfm)
            .width
            .offset(*(*tfm).width_index.offset(i as isize) as isize);
        *(*fm).heights.offset((*tfm).bc.wrapping_add(i) as isize) = *(*tfm)
            .height
            .offset(*(*tfm).height_index.offset(i as isize) as isize);
        *(*fm).depths.offset((*tfm).bc.wrapping_add(i) as isize) = *(*tfm)
            .depth
            .offset(*(*tfm).depth_index.offset(i as isize) as isize);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn read_ofm(
    mut fm: *mut font_metric,
    mut ofm_handle: rust_input_handle_t,
    mut ofm_file_size: off_t,
) {
    let mut tfm: tfm_font = tfm_font {
        level: 0,
        wlenfile: 0,
        wlenheader: 0,
        bc: 0,
        ec: 0,
        nwidths: 0,
        nheights: 0,
        ndepths: 0,
        nitcor: 0,
        nlig: 0,
        nkern: 0,
        nextens: 0,
        nfonparm: 0,
        fontdir: 0,
        nco: 0,
        ncw: 0,
        npc: 0,
        header: 0 as *mut fixword,
        char_info: 0 as *mut u32,
        width_index: 0 as *mut u16,
        height_index: 0 as *mut u8,
        depth_index: 0 as *mut u8,
        width: 0 as *mut fixword,
        height: 0 as *mut fixword,
        depth: 0 as *mut fixword,
    };
    tfm_font_init(&mut tfm);
    ofm_get_sizes(ofm_handle, ofm_file_size, &mut tfm);
    if tfm.level < 0i32 || tfm.level > 1i32 {
        _tt_abort(
            b"OFM level %d not supported.\x00" as *const u8 as *const libc::c_char,
            tfm.level,
        );
    }
    if tfm.wlenheader > 0i32 as libc::c_uint {
        tfm.header = new((tfm.wlenheader as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.header, tfm.wlenheader, ofm_handle);
    }
    if tfm.level == 0i32 {
        ofm_do_char_info_zero(ofm_handle, &mut tfm);
    } else if tfm.level == 1i32 {
        ofm_do_char_info_one(ofm_handle, &mut tfm);
    }
    if tfm.nwidths > 0i32 as libc::c_uint {
        tfm.width = new((tfm.nwidths as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.width, tfm.nwidths, ofm_handle);
    }
    if tfm.nheights > 0i32 as libc::c_uint {
        tfm.height = new((tfm.nheights as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.height, tfm.nheights, ofm_handle);
    }
    if tfm.ndepths > 0i32 as libc::c_uint {
        tfm.depth = new((tfm.ndepths as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.depth, tfm.ndepths, ofm_handle);
    }
    ofm_unpack_arrays(
        fm,
        &mut tfm,
        tfm.ec
            .wrapping_sub(tfm.bc)
            .wrapping_add(1i32 as libc::c_uint),
    );
    tfm_unpack_header(fm, &mut tfm);
    (*fm).firstchar = tfm.bc as libc::c_int;
    (*fm).lastchar = tfm.ec as libc::c_int;
    (*fm).source = 2i32;
    tfm_font_clear(&mut tfm);
}
unsafe extern "C" fn read_tfm(
    mut fm: *mut font_metric,
    mut tfm_handle: rust_input_handle_t,
    mut tfm_file_size: off_t,
) {
    let mut tfm: tfm_font = tfm_font {
        level: 0,
        wlenfile: 0,
        wlenheader: 0,
        bc: 0,
        ec: 0,
        nwidths: 0,
        nheights: 0,
        ndepths: 0,
        nitcor: 0,
        nlig: 0,
        nkern: 0,
        nextens: 0,
        nfonparm: 0,
        fontdir: 0,
        nco: 0,
        ncw: 0,
        npc: 0,
        header: 0 as *mut fixword,
        char_info: 0 as *mut u32,
        width_index: 0 as *mut u16,
        height_index: 0 as *mut u8,
        depth_index: 0 as *mut u8,
        width: 0 as *mut fixword,
        height: 0 as *mut fixword,
        depth: 0 as *mut fixword,
    };
    tfm_font_init(&mut tfm);
    tfm_get_sizes(tfm_handle, tfm_file_size, &mut tfm);
    (*fm).firstchar = tfm.bc as libc::c_int;
    (*fm).lastchar = tfm.ec as libc::c_int;
    if tfm.wlenheader > 0i32 as libc::c_uint {
        tfm.header = new((tfm.wlenheader as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.header, tfm.wlenheader, tfm_handle);
    }
    if tfm
        .ec
        .wrapping_sub(tfm.bc)
        .wrapping_add(1i32 as libc::c_uint)
        > 0i32 as libc::c_uint
    {
        tfm.char_info = new((tfm
            .ec
            .wrapping_sub(tfm.bc)
            .wrapping_add(1i32 as libc::c_uint) as u64)
            .wrapping_mul(::std::mem::size_of::<u32>() as u64)
            as u32) as *mut u32;
        fread_uquads(
            tfm.char_info,
            tfm.ec
                .wrapping_sub(tfm.bc)
                .wrapping_add(1i32 as libc::c_uint),
            tfm_handle,
        );
    }
    if tfm.nwidths > 0i32 as libc::c_uint {
        tfm.width = new((tfm.nwidths as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.width, tfm.nwidths, tfm_handle);
    }
    if tfm.nheights > 0i32 as libc::c_uint {
        tfm.height = new((tfm.nheights as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.height, tfm.nheights, tfm_handle);
    }
    if tfm.ndepths > 0i32 as libc::c_uint {
        tfm.depth = new((tfm.ndepths as u64)
            .wrapping_mul(::std::mem::size_of::<fixword>() as u64)
            as u32) as *mut fixword;
        fread_fwords(tfm.depth, tfm.ndepths, tfm_handle);
    }
    tfm_unpack_arrays(fm, &mut tfm);
    tfm_unpack_header(fm, &mut tfm);
    tfm_font_clear(&mut tfm);
}
#[no_mangle]
pub unsafe extern "C" fn tfm_open(
    mut tfm_name: *const libc::c_char,
    mut must_exist: libc::c_int,
) -> libc::c_int {
    let mut tfm_handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut format: libc::c_int = 1i32;
    let mut tfm_file_size: off_t = 0;
    let mut ofm_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0i32;
    while (i as libc::c_uint) < numfms {
        if streq_ptr(tfm_name, (*fms.offset(i as isize)).tex_name) {
            return i;
        }
        i += 1
    }
    /* NOTE: the following comment is no longer operative with the switch to
     * the Tectonic I/O system since we don't have `must_exist`. The logic
     * of the current implementation might not be right; to be investigated.
     * Comment preserved for posterity.
     *
     * "The procedure to search tfm or ofm files:
     * 1. Search tfm file with the given name with the must_exist flag unset.
     * 2. Search ofm file with the given name with the must_exist flag unset.
     * 3. If not found and must_exist flag is set, try again to search
     *    tfm file with the must_exist flag set.
     * 4. If not found and must_exist flag is not set, return -1.
     *
     * We first look for OFM and then TFM.
     * The reason for this change is incompatibility introduced when dvipdfmx
     * started to write correct glyph metrics to output PDF for CID fonts.
     * I'll not explain this in detail... This change is mostly specific to
     * Japanese support."
     */
    suffix = strrchr(tfm_name, '.' as i32);
    if suffix.is_null()
        || strcmp(suffix, b".tfm\x00" as *const u8 as *const libc::c_char) != 0i32
            && strcmp(suffix, b".ofm\x00" as *const u8 as *const libc::c_char) != 0i32
    {
        ofm_name = new((strlen(tfm_name)
            .wrapping_add(strlen(b".ofm\x00" as *const u8 as *const libc::c_char))
            .wrapping_add(1i32 as u64) as u32
            as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32) as *mut libc::c_char;
        strcpy(ofm_name, tfm_name);
        strcat(ofm_name, b".ofm\x00" as *const u8 as *const libc::c_char);
    } else {
        ofm_name = 0 as *mut libc::c_char
    }
    if !ofm_name.is_null() && {
        tfm_handle = ttstub_input_open(ofm_name, TTIF_OFM, 0i32);
        !tfm_handle.is_null()
    } {
        format = 2i32
    } else {
        tfm_handle = ttstub_input_open(tfm_name, TTIF_TFM, 0i32);
        if !tfm_handle.is_null() {
            format = 1i32
        } else {
            tfm_handle = ttstub_input_open(tfm_name, TTIF_OFM, 0i32);
            if !tfm_handle.is_null() {
                format = 2i32
            }
        }
    }
    free(ofm_name as *mut libc::c_void);
    if tfm_handle.is_null() {
        if must_exist != 0 {
            _tt_abort(
                b"Unable to find TFM file \"%s\".\x00" as *const u8 as *const libc::c_char,
                tfm_name,
            );
        }
        return -1i32;
    }
    if verbose != 0 {
        if format == 1i32 {
            dpx_message(b"(TFM:%s\x00" as *const u8 as *const libc::c_char, tfm_name);
        } else if format == 2i32 {
            dpx_message(b"(OFM:%s\x00" as *const u8 as *const libc::c_char, tfm_name);
        }
    }
    tfm_file_size = ttstub_input_get_size(tfm_handle) as off_t;
    if tfm_file_size as u64 > 0x1ffffffffu64 {
        _tt_abort(b"TFM/OFM file size exceeds 33-bit\x00" as *const u8 as *const libc::c_char);
    }
    if tfm_file_size < 24i32 as libc::c_long {
        _tt_abort(
            b"TFM/OFM file too small to be a valid file.\x00" as *const u8 as *const libc::c_char,
        );
    }
    fms_need(numfms.wrapping_add(1i32 as libc::c_uint));
    fm_init(fms.offset(numfms as isize));
    if format == 2i32 {
        read_ofm(&mut *fms.offset(numfms as isize), tfm_handle, tfm_file_size);
    } else {
        read_tfm(&mut *fms.offset(numfms as isize), tfm_handle, tfm_file_size);
    }
    ttstub_input_close(tfm_handle);
    let ref mut fresh0 = (*fms.offset(numfms as isize)).tex_name;
    *fresh0 = new(
        (strlen(tfm_name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32,
    ) as *mut libc::c_char;
    strcpy((*fms.offset(numfms as isize)).tex_name, tfm_name);
    if verbose != 0 {
        dpx_message(b")\x00" as *const u8 as *const libc::c_char);
    }
    let fresh1 = numfms;
    numfms = numfms.wrapping_add(1);
    return fresh1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tfm_close_all() {
    let mut i: libc::c_uint = 0;
    if !fms.is_null() {
        i = 0i32 as libc::c_uint;
        while i < numfms {
            fm_clear(&mut *fms.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free(fms as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tfm_get_fw_width(mut font_id: libc::c_int, mut ch: int32_t) -> fixword {
    let mut fm: *mut font_metric = 0 as *mut font_metric;
    let mut idx: libc::c_int = 0i32;
    if font_id < 0i32 || font_id as libc::c_uint >= numfms {
        _tt_abort(
            b"TFM: Invalid TFM ID: %d\x00" as *const u8 as *const libc::c_char,
            font_id,
        );
    }
    fm = &mut *fms.offset(font_id as isize) as *mut font_metric;
    if ch >= (*fm).firstchar && ch <= (*fm).lastchar {
        match (*fm).charmap.type_0 {
            1 => {
                idx = lookup_char((*fm).charmap.data as *const char_map, ch);
                if idx < 0i32 {
                    _tt_abort(
                        b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                }
            }
            2 => {
                idx = lookup_range((*fm).charmap.data as *const range_map, ch);
                if idx < 0i32 {
                    _tt_abort(
                        b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                }
            }
            _ => idx = ch,
        }
    } else {
        _tt_abort(
            b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
            ch,
        );
    }
    return *(*fm).widths.offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn tfm_get_fw_height(mut font_id: libc::c_int, mut ch: int32_t) -> fixword {
    let mut fm: *mut font_metric = 0 as *mut font_metric;
    let mut idx: libc::c_int = 0i32;
    if font_id < 0i32 || font_id as libc::c_uint >= numfms {
        _tt_abort(
            b"TFM: Invalid TFM ID: %d\x00" as *const u8 as *const libc::c_char,
            font_id,
        );
    }
    fm = &mut *fms.offset(font_id as isize) as *mut font_metric;
    if ch >= (*fm).firstchar && ch <= (*fm).lastchar {
        match (*fm).charmap.type_0 {
            1 => {
                idx = lookup_char((*fm).charmap.data as *const char_map, ch);
                if idx < 0i32 {
                    _tt_abort(
                        b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                }
            }
            2 => {
                idx = lookup_range((*fm).charmap.data as *const range_map, ch);
                if idx < 0i32 {
                    _tt_abort(
                        b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                }
            }
            _ => idx = ch,
        }
    } else {
        _tt_abort(
            b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
            ch,
        );
    }
    return *(*fm).heights.offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn tfm_get_fw_depth(mut font_id: libc::c_int, mut ch: int32_t) -> fixword {
    let mut fm: *mut font_metric = 0 as *mut font_metric;
    let mut idx: libc::c_int = 0i32;
    if font_id < 0i32 || font_id as libc::c_uint >= numfms {
        _tt_abort(
            b"TFM: Invalid TFM ID: %d\x00" as *const u8 as *const libc::c_char,
            font_id,
        );
    }
    fm = &mut *fms.offset(font_id as isize) as *mut font_metric;
    if ch >= (*fm).firstchar && ch <= (*fm).lastchar {
        match (*fm).charmap.type_0 {
            1 => {
                idx = lookup_char((*fm).charmap.data as *const char_map, ch);
                if idx < 0i32 {
                    _tt_abort(
                        b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                }
            }
            2 => {
                idx = lookup_range((*fm).charmap.data as *const range_map, ch);
                if idx < 0i32 {
                    _tt_abort(
                        b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                }
            }
            _ => idx = ch,
        }
    } else {
        _tt_abort(
            b"Invalid char: %d\n\x00" as *const u8 as *const libc::c_char,
            ch,
        );
    }
    return *(*fm).depths.offset(idx as isize);
}
/*
 * tfm_get_width returns the width of the font
 * as a (double) fraction of the design size.
 */
#[no_mangle]
pub unsafe extern "C" fn tfm_get_width(
    mut font_id: libc::c_int,
    mut ch: int32_t,
) -> libc::c_double {
    return tfm_get_fw_width(font_id, ch) as libc::c_double / (1i32 << 20i32) as libc::c_double;
}
/* tfm_string_xxx() do not work for OFM... */
#[no_mangle]
pub unsafe extern "C" fn tfm_string_width(
    mut font_id: libc::c_int,
    mut s: *const u8,
    mut len: libc::c_uint,
) -> fixword {
    let mut result: fixword = 0i32;
    let mut i: libc::c_uint = 0;
    if font_id < 0i32 || font_id as libc::c_uint >= numfms {
        _tt_abort(
            b"TFM: Invalid TFM ID: %d\x00" as *const u8 as *const libc::c_char,
            font_id,
        );
    }
    i = 0i32 as libc::c_uint;
    while i < len {
        result += tfm_get_fw_width(font_id, *s.offset(i as isize) as int32_t);
        i = i.wrapping_add(1)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn tfm_get_design_size(mut font_id: libc::c_int) -> libc::c_double {
    if font_id < 0i32 || font_id as libc::c_uint >= numfms {
        _tt_abort(
            b"TFM: Invalid TFM ID: %d\x00" as *const u8 as *const libc::c_char,
            font_id,
        );
    }
    return (*fms.offset(font_id as isize)).designsize as libc::c_double
        / (1i32 << 20i32) as libc::c_double
        * (72.0f64 / 72.27f64);
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
/* From TFM header */
#[no_mangle]
pub unsafe extern "C" fn tfm_exists(mut tfm_name: *const libc::c_char) -> bool {
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    handle = ttstub_input_open(tfm_name, TTIF_OFM, 0i32) as *mut rust_input_handle_t;
    if !handle.is_null() {
        ttstub_input_close(handle as rust_input_handle_t);
        return 1i32 != 0;
    }
    handle = ttstub_input_open(tfm_name, TTIF_TFM, 0i32) as *mut rust_input_handle_t;
    if !handle.is_null() {
        ttstub_input_close(handle as rust_input_handle_t);
        return 1i32 != 0;
    }
    return 0i32 != 0;
}
