#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pdf_file;
    pub type pdf_obj;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut name_of_file: *mut libc::c_char;
    #[no_mangle]
    static mut file_line_error_style_p: libc::c_int;
    #[no_mangle]
    static mut help_line: [*const libc::c_char; 6];
    #[no_mangle]
    static mut help_ptr: libc::c_uchar;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut cur_list: list_state_record;
    #[no_mangle]
    static mut cur_val: int32_t;
    #[no_mangle]
    static mut cur_name: str_number;
    #[no_mangle]
    static mut cur_area: str_number;
    #[no_mangle]
    static mut cur_ext: str_number;
    #[no_mangle]
    fn scan_keyword(s: *const libc::c_char) -> bool;
    #[no_mangle]
    fn scan_int();
    #[no_mangle]
    fn scan_dimen(mu: bool, inf: bool, shortcut: bool);
    #[no_mangle]
    fn scan_decimal();
    #[no_mangle]
    fn pack_file_name(n: str_number, a: str_number, e: str_number);
    #[no_mangle]
    fn scan_file_name();
    #[no_mangle]
    fn new_whatsit(s: small_number, w: small_number);
    /* xetex-errors */
    #[no_mangle]
    fn error();
    #[no_mangle]
    fn print(s: int32_t);
    #[no_mangle]
    fn print_cstr(s: *const libc::c_char);
    #[no_mangle]
    fn print_nl_cstr(s: *const libc::c_char);
    /* ***************************************************************************\
     Part of the XeTeX typesetting system
     Copyright (c) 1994-2008 by SIL International
     Copyright (c) 2009, 2011 by Jonathan Kew
     Copyright (c) 2012, 2013 by Jiang Jiang
     Copyright (c) 2012-2015 by Khaled Hosny

     SIL Author(s): Jonathan Kew

    Permission is hereby granted, free of charge, to any person obtaining
    a copy of this software and associated documentation files (the
    "Software"), to deal in the Software without restriction, including
    without limitation the rights to use, copy, modify, merge, publish,
    distribute, sublicense, and/or sell copies of the Software, and to
    permit persons to whom the Software is furnished to do so, subject to
    the following conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
    FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
    WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

    Except as contained in this notice, the name of the copyright holders
    shall not be used in advertising or otherwise to promote the sale,
    use or other dealings in this Software without prior written
    authorization from the copyright holders.
    \****************************************************************************/
    /* some typedefs that XeTeX uses - on Mac OS, we get these from Apple headers,
    but otherwise we'll need these substitute definitions */
    /* command codes for XeTeX extension commands */
    /* accessing info in a native_word_node */
    /* info for each glyph is location (FixedPoint) + glyph ID (uint16_t) */
    /* glyph ID field in a glyph_node */
    /* For Unicode encoding form interpretation... */
    /* single-purpose metrics accessors */
    #[no_mangle]
    fn Fix2D(f: Fixed) -> libc::c_double;
    #[no_mangle]
    fn print_file_name(n: int32_t, a: int32_t, e: int32_t);
    #[no_mangle]
    fn print_file_line();
    #[no_mangle]
    fn print_scaled(s: scaled_t);
    #[no_mangle]
    fn D2Fix(d: libc::c_double) -> Fixed;
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
    #[no_mangle]
    fn min4(
        v1: libc::c_double,
        v2: libc::c_double,
        v3: libc::c_double,
        v4: libc::c_double,
    ) -> libc::c_double;
    #[no_mangle]
    fn max4(
        v1: libc::c_double,
        v2: libc::c_double,
        v3: libc::c_double,
        v4: libc::c_double,
    ) -> libc::c_double;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2007-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
    /* PDF document metadata */
    /* They just return PDF dictionary object.
     * Callers are completely responsible for doing right thing...
     */
    #[no_mangle]
    fn pdf_doc_get_page_count(pf: *mut pdf_file) -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_get_page(
        pf: *mut pdf_file,
        page_no: libc::c_int,
        options: libc::c_int,
        bbox: *mut pdf_rect,
        matrix: *mut pdf_tmatrix,
        resources_p: *mut *mut pdf_obj,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_open(ident: *const libc::c_char, handle: rust_input_handle_t) -> *mut pdf_file;
    #[no_mangle]
    fn pdf_close(pf: *mut pdf_file);
    #[no_mangle]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[no_mangle]
    fn check_for_png(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn png_get_bbox(
        handle: rust_input_handle_t,
        width: *mut u32,
        height: *mut u32,
        xdensity: *mut libc::c_double,
        ydensity: *mut libc::c_double,
    ) -> libc::c_int;
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
    fn check_for_jpeg(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn jpeg_get_bbox(
        handle: rust_input_handle_t,
        width: *mut libc::c_uint,
        height: *mut libc::c_uint,
        xdensity: *mut libc::c_double,
        ydensity: *mut libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn check_for_bmp(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn bmp_get_bbox(
        handle: rust_input_handle_t,
        width: *mut libc::c_uint,
        height: *mut libc::c_uint,
        xdensity: *mut libc::c_double,
        ydensity: *mut libc::c_double,
    ) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
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
pub type scaled_t = int32_t;
pub type Fixed = scaled_t;
pub type str_number = int32_t;
pub type small_number = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: int32_t,
    pub s1: int32_t,
}
/* quasi-hack to get the primary input */
/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */
pub type b32x2 = b32x2_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: uint16_t,
    pub s1: uint16_t,
    pub s2: uint16_t,
    pub s3: uint16_t,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: libc::c_double,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_state_record {
    pub mode: libc::c_short,
    pub head: int32_t,
    pub tail: int32_t,
    pub eTeX_aux: int32_t,
    pub prev_graf: int32_t,
    pub mode_line: int32_t,
    pub aux: memory_word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_t {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct real_point {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct real_rect {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub wd: libc::c_float,
    pub ht: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: libc::c_double,
    pub y: libc::c_double,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub f: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn count_pdf_file_pages() -> libc::c_int {
    let mut pages: libc::c_int = 0;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    handle = ttstub_input_open(name_of_file, TTIF_PICT, 0i32);
    if handle.is_null() {
        return 0i32;
    }
    pf = pdf_open(name_of_file, handle);
    if pf.is_null() {
        /* TODO: issue warning */
        ttstub_input_close(handle);
        return 0i32;
    }
    pages = pdf_doc_get_page_count(pf);
    pdf_close(pf);
    ttstub_input_close(handle);
    return pages;
}
unsafe extern "C" fn pdf_get_rect(
    mut filename: *mut libc::c_char,
    mut handle: rust_input_handle_t,
    mut page_num: libc::c_int,
    mut pdf_box: libc::c_int,
    mut box_0: *mut real_rect,
) -> libc::c_int {
    let mut pages: libc::c_int = 0;
    let mut dpx_options: libc::c_int = 0;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    let mut page: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut bbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut matrix: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p4: pdf_coord = pdf_coord { x: 0., y: 0. };
    pf = pdf_open(filename, handle);
    if pf.is_null() {
        /* TODO: issue warning */
        return -1i32;
    }
    pages = pdf_doc_get_page_count(pf);
    if page_num > pages {
        page_num = pages
    }
    if page_num < 0i32 {
        page_num = pages + 1i32 + page_num
    }
    if page_num < 1i32 {
        page_num = 1i32
    }
    /* OMG, magic numbers specifying page bound types do not agree between
     * xdvipdfmx code (dpx-pdfdoc.c:pdf_doc_get_page) and XeTeX/Apple's
     * pdfbox_* definitions (xetex-ext.h). */
    match pdf_box {
        2 => dpx_options = 2i32,
        3 => dpx_options = 5i32,
        4 => dpx_options = 4i32,
        5 => dpx_options = 3i32,
        1 | _ => dpx_options = 1i32,
    }
    page = pdf_doc_get_page(
        pf,
        page_num,
        dpx_options,
        &mut bbox,
        &mut matrix,
        0 as *mut *mut pdf_obj,
    );
    pdf_close(pf);
    if page.is_null() {
        /* TODO: issue warning */
        return -1i32;
    }
    pdf_release_obj(page);
    /* Image's attribute "bbox" here is affected by /Rotate entry of included
     * PDF page.
     */
    p1.x = bbox.llx;
    p1.y = bbox.lly;
    pdf_dev_transform(&mut p1, &mut matrix);
    p2.x = bbox.urx;
    p2.y = bbox.lly;
    pdf_dev_transform(&mut p2, &mut matrix);
    p3.x = bbox.urx;
    p3.y = bbox.ury;
    pdf_dev_transform(&mut p3, &mut matrix);
    p4.x = bbox.llx;
    p4.y = bbox.ury;
    pdf_dev_transform(&mut p4, &mut matrix);
    bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
    bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
    bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
    bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);
    (*box_0).x = (72.27f64 / 72i32 as libc::c_double * bbox.llx) as libc::c_float;
    (*box_0).y = (72.27f64 / 72i32 as libc::c_double * bbox.lly) as libc::c_float;
    (*box_0).wd = (72.27f64 / 72i32 as libc::c_double * (bbox.urx - bbox.llx)) as libc::c_float;
    (*box_0).ht = (72.27f64 / 72i32 as libc::c_double * (bbox.ury - bbox.lly)) as libc::c_float;
    return 0i32;
}
unsafe extern "C" fn get_image_size_in_inches(
    mut handle: rust_input_handle_t,
    mut width: *mut libc::c_float,
    mut height: *mut libc::c_float,
) -> libc::c_int {
    let mut err: libc::c_int = 1i32;
    let mut width_pix: libc::c_uint = 0;
    let mut height_pix: libc::c_uint = 0;
    let mut xdensity: libc::c_double = 0.;
    let mut ydensity: libc::c_double = 0.;
    if check_for_jpeg(handle) != 0 {
        err = jpeg_get_bbox(
            handle,
            &mut width_pix,
            &mut height_pix,
            &mut xdensity,
            &mut ydensity,
        )
    } else if check_for_bmp(handle) != 0 {
        err = bmp_get_bbox(
            handle,
            &mut width_pix,
            &mut height_pix,
            &mut xdensity,
            &mut ydensity,
        )
    } else if check_for_png(handle) != 0 {
        err = png_get_bbox(
            handle,
            &mut width_pix,
            &mut height_pix,
            &mut xdensity,
            &mut ydensity,
        )
    }
    if err != 0 {
        *width = -1i32 as libc::c_float;
        *height = -1i32 as libc::c_float;
        return err;
    }
    /* xdvipdfmx defines density = 72 / dpi, so ... */
    *width = (width_pix as libc::c_double * xdensity / 72i32 as libc::c_double) as libc::c_float;
    *height = (height_pix as libc::c_double * ydensity / 72i32 as libc::c_double) as libc::c_float;
    return 0i32;
}
/*
  pdfBoxType indicates which pdf bounding box to use (0 for \XeTeXpicfile)
  page indicates which page is wanted (0-based)
  return 0 for success, or non-zero error code for failure
  return full path in *path
  return bounds (tex points) in *bounds
*/
unsafe extern "C" fn find_pic_file(
    mut path: *mut *mut libc::c_char,
    mut bounds: *mut real_rect,
    mut pdfBoxType: libc::c_int,
    mut page: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = -1i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    handle = ttstub_input_open(name_of_file, TTIF_PICT, 0i32);
    (*bounds).ht = 0.0f64 as libc::c_float;
    (*bounds).wd = (*bounds).ht;
    (*bounds).y = (*bounds).wd;
    (*bounds).x = (*bounds).y;
    if handle.is_null() {
        return 1i32;
    }
    if pdfBoxType != 0i32 {
        /* if cmd was \XeTeXpdffile, use xpdflib to read it */
        err = pdf_get_rect(name_of_file, handle, page, pdfBoxType, bounds)
    } else {
        err = get_image_size_in_inches(handle, &mut (*bounds).wd, &mut (*bounds).ht);
        (*bounds).wd = ((*bounds).wd as libc::c_double * 72.27f64) as libc::c_float;
        (*bounds).ht = ((*bounds).ht as libc::c_double * 72.27f64) as libc::c_float
    }
    if err == 0i32 {
        *path = xstrdup(name_of_file)
    }
    ttstub_input_close(handle);
    return err;
}
unsafe extern "C" fn transform_point(mut p: *mut real_point, mut t: *const transform_t) {
    let mut r: real_point = real_point { x: 0., y: 0. };
    r.x = ((*t).a * (*p).x as libc::c_double + (*t).c * (*p).y as libc::c_double + (*t).x)
        as libc::c_float;
    r.y = ((*t).b * (*p).x as libc::c_double + (*t).d * (*p).y as libc::c_double + (*t).y)
        as libc::c_float;
    *p = r;
}
unsafe extern "C" fn make_identity(mut t: *mut transform_t) {
    (*t).a = 1.0f64;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = 1.0f64;
    (*t).x = 0.0f64;
    (*t).y = 0.0f64;
}
unsafe extern "C" fn make_scale(
    mut t: *mut transform_t,
    mut xscale: libc::c_double,
    mut yscale: libc::c_double,
) {
    (*t).a = xscale;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = yscale;
    (*t).x = 0.0f64;
    (*t).y = 0.0f64;
}
unsafe extern "C" fn make_translation(
    mut t: *mut transform_t,
    mut dx: libc::c_double,
    mut dy: libc::c_double,
) {
    (*t).a = 1.0f64;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = 1.0f64;
    (*t).x = dx;
    (*t).y = dy;
}
unsafe extern "C" fn make_rotation(mut t: *mut transform_t, mut a: libc::c_double) {
    (*t).a = cos(a);
    (*t).b = sin(a);
    (*t).c = -sin(a);
    (*t).d = cos(a);
    (*t).x = 0.0f64;
    (*t).y = 0.0f64;
}
unsafe extern "C" fn transform_concat(mut t1: *mut transform_t, mut t2: *const transform_t) {
    let mut r: transform_t = transform_t {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        x: 0.,
        y: 0.,
    };
    r.a = (*t1).a * (*t2).a + (*t1).b * (*t2).c + 0.0f64 * (*t2).x;
    r.b = (*t1).a * (*t2).b + (*t1).b * (*t2).d + 0.0f64 * (*t2).y;
    r.c = (*t1).c * (*t2).a + (*t1).d * (*t2).c + 0.0f64 * (*t2).x;
    r.d = (*t1).c * (*t2).b + (*t1).d * (*t2).d + 0.0f64 * (*t2).y;
    r.x = (*t1).x * (*t2).a + (*t1).y * (*t2).c + 1.0f64 * (*t2).x;
    r.y = (*t1).x * (*t2).b + (*t1).y * (*t2).d + 1.0f64 * (*t2).y;
    *t1 = r;
}
#[no_mangle]
pub unsafe extern "C" fn load_picture(mut is_pdf: bool) {
    let mut pic_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bounds: real_rect = real_rect {
        x: 0.,
        y: 0.,
        wd: 0.,
        ht: 0.,
    };
    let mut t: transform_t = transform_t {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        x: 0.,
        y: 0.,
    };
    let mut t2: transform_t = transform_t {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        x: 0.,
        y: 0.,
    };
    let mut corners: [real_point; 4] = [real_point { x: 0., y: 0. }; 4];
    let mut x_size_req: libc::c_double = 0.;
    let mut y_size_req: libc::c_double = 0.;
    let mut check_keywords: bool = false;
    let mut xmin: libc::c_double = 0.;
    let mut xmax: libc::c_double = 0.;
    let mut ymin: libc::c_double = 0.;
    let mut ymax: libc::c_double = 0.;
    let mut i: small_number = 0;
    let mut page: int32_t = 0;
    let mut pdf_box_type: int32_t = 0;
    let mut result: int32_t = 0;
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
    pdf_box_type = 0i32;
    page = 0i32;
    if is_pdf {
        if scan_keyword(b"page\x00" as *const u8 as *const libc::c_char) {
            scan_int();
            page = cur_val
        }
        pdf_box_type = 6i32;
        if scan_keyword(b"crop\x00" as *const u8 as *const libc::c_char) {
            pdf_box_type = 1i32
        } else if scan_keyword(b"media\x00" as *const u8 as *const libc::c_char) {
            pdf_box_type = 2i32
        } else if scan_keyword(b"bleed\x00" as *const u8 as *const libc::c_char) {
            pdf_box_type = 3i32
        } else if scan_keyword(b"trim\x00" as *const u8 as *const libc::c_char) {
            pdf_box_type = 4i32
        } else if scan_keyword(b"art\x00" as *const u8 as *const libc::c_char) {
            pdf_box_type = 5i32
        }
    }
    if pdf_box_type == 6i32 {
        result = find_pic_file(&mut pic_path, &mut bounds, 1i32, page)
    } else {
        result = find_pic_file(&mut pic_path, &mut bounds, pdf_box_type, page)
    }
    corners[0].x = bounds.x;
    corners[0].y = bounds.y;
    corners[1].x = corners[0].x;
    corners[1].y = bounds.y + bounds.ht;
    corners[2].x = bounds.x + bounds.wd;
    corners[2].y = corners[1].y;
    corners[3].x = corners[2].x;
    corners[3].y = corners[0].y;
    x_size_req = 0.0f64;
    y_size_req = 0.0f64;
    make_identity(&mut t);
    check_keywords = 1i32 != 0;
    while check_keywords {
        if scan_keyword(b"scaled\x00" as *const u8 as *const libc::c_char) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(
                    &mut t2,
                    cur_val as libc::c_double / 1000.0f64,
                    cur_val as libc::c_double / 1000.0f64,
                );
                let mut for_end: int32_t = 0;
                i = 0i32 as small_number;
                for_end = 3i32;
                if i as libc::c_int <= for_end {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh0 = i;
                        i = i + 1;
                        if !((fresh0 as libc::c_int) < for_end) {
                            break;
                        }
                    }
                }
                transform_concat(&mut t, &mut t2);
            }
        } else if scan_keyword(b"xscaled\x00" as *const u8 as *const libc::c_char) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(&mut t2, cur_val as libc::c_double / 1000.0f64, 1.0f64);
                let mut for_end_0: int32_t = 0;
                i = 0i32 as small_number;
                for_end_0 = 3i32;
                if i as libc::c_int <= for_end_0 {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh1 = i;
                        i = i + 1;
                        if !((fresh1 as libc::c_int) < for_end_0) {
                            break;
                        }
                    }
                }
                transform_concat(&mut t, &mut t2);
            }
        } else if scan_keyword(b"yscaled\x00" as *const u8 as *const libc::c_char) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(&mut t2, 1.0f64, cur_val as libc::c_double / 1000.0f64);
                let mut for_end_1: int32_t = 0;
                i = 0i32 as small_number;
                for_end_1 = 3i32;
                if i as libc::c_int <= for_end_1 {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh2 = i;
                        i = i + 1;
                        if !((fresh2 as libc::c_int) < for_end_1) {
                            break;
                        }
                    }
                }
                transform_concat(&mut t, &mut t2);
            }
        } else if scan_keyword(b"width\x00" as *const u8 as *const libc::c_char) {
            scan_dimen(0i32 != 0, 0i32 != 0, 0i32 != 0);
            if cur_val <= 0i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
                }
                print_cstr(b"Improper image \x00" as *const u8 as *const libc::c_char);
                print_cstr(b"size (\x00" as *const u8 as *const libc::c_char);
                print_scaled(cur_val);
                print_cstr(b"pt) will be ignored\x00" as *const u8 as *const libc::c_char);
                help_ptr = 2i32 as libc::c_uchar;
                help_line[1] = b"I can\'t scale images to zero or negative sizes,\x00" as *const u8
                    as *const libc::c_char;
                help_line[0] = b"so I\'m ignoring this.\x00" as *const u8 as *const libc::c_char;
                error();
            } else {
                x_size_req = Fix2D(cur_val)
            }
        } else if scan_keyword(b"height\x00" as *const u8 as *const libc::c_char) {
            scan_dimen(0i32 != 0, 0i32 != 0, 0i32 != 0);
            if cur_val <= 0i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
                }
                print_cstr(b"Improper image \x00" as *const u8 as *const libc::c_char);
                print_cstr(b"size (\x00" as *const u8 as *const libc::c_char);
                print_scaled(cur_val);
                print_cstr(b"pt) will be ignored\x00" as *const u8 as *const libc::c_char);
                help_ptr = 2i32 as libc::c_uchar;
                help_line[1] = b"I can\'t scale images to zero or negative sizes,\x00" as *const u8
                    as *const libc::c_char;
                help_line[0] = b"so I\'m ignoring this.\x00" as *const u8 as *const libc::c_char;
                error();
            } else {
                y_size_req = Fix2D(cur_val)
            }
        } else if scan_keyword(b"rotated\x00" as *const u8 as *const libc::c_char) {
            scan_decimal();
            if x_size_req != 0.0f64 || y_size_req != 0.0f64 {
                xmin = 1000000.0f64;
                xmax = -(xmin as int32_t) as libc::c_double;
                ymin = xmin;
                ymax = xmax;
                let mut for_end_2: int32_t = 0;
                i = 0i32 as small_number;
                for_end_2 = 3i32;
                if i as libc::c_int <= for_end_2 {
                    loop {
                        if (corners[i as usize].x as libc::c_double) < xmin {
                            xmin = corners[i as usize].x as libc::c_double
                        }
                        if corners[i as usize].x as libc::c_double > xmax {
                            xmax = corners[i as usize].x as libc::c_double
                        }
                        if (corners[i as usize].y as libc::c_double) < ymin {
                            ymin = corners[i as usize].y as libc::c_double
                        }
                        if corners[i as usize].y as libc::c_double > ymax {
                            ymax = corners[i as usize].y as libc::c_double
                        }
                        let fresh3 = i;
                        i = i + 1;
                        if !((fresh3 as libc::c_int) < for_end_2) {
                            break;
                        }
                    }
                }
                if x_size_req == 0.0f64 {
                    make_scale(
                        &mut t2,
                        y_size_req / (ymax - ymin),
                        y_size_req / (ymax - ymin),
                    );
                } else if y_size_req == 0.0f64 {
                    make_scale(
                        &mut t2,
                        x_size_req / (xmax - xmin),
                        x_size_req / (xmax - xmin),
                    );
                } else {
                    make_scale(
                        &mut t2,
                        x_size_req / (xmax - xmin),
                        y_size_req / (ymax - ymin),
                    );
                }
                let mut for_end_3: int32_t = 0;
                i = 0i32 as small_number;
                for_end_3 = 3i32;
                if i as libc::c_int <= for_end_3 {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh4 = i;
                        i = i + 1;
                        if !((fresh4 as libc::c_int) < for_end_3) {
                            break;
                        }
                    }
                }
                x_size_req = 0.0f64;
                y_size_req = 0.0f64;
                transform_concat(&mut t, &mut t2);
            }
            make_rotation(
                &mut t2,
                Fix2D(cur_val) * 3.14159265358979323846f64 / 180.0f64,
            );
            let mut for_end_4: int32_t = 0;
            i = 0i32 as small_number;
            for_end_4 = 3i32;
            if i as libc::c_int <= for_end_4 {
                loop {
                    transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                    let fresh5 = i;
                    i = i + 1;
                    if !((fresh5 as libc::c_int) < for_end_4) {
                        break;
                    }
                }
            }
            xmin = 1000000.0f64;
            xmax = -(xmin as int32_t) as libc::c_double;
            ymin = xmin;
            ymax = xmax;
            let mut for_end_5: int32_t = 0;
            i = 0i32 as small_number;
            for_end_5 = 3i32;
            if i as libc::c_int <= for_end_5 {
                loop {
                    if (corners[i as usize].x as libc::c_double) < xmin {
                        xmin = corners[i as usize].x as libc::c_double
                    }
                    if corners[i as usize].x as libc::c_double > xmax {
                        xmax = corners[i as usize].x as libc::c_double
                    }
                    if (corners[i as usize].y as libc::c_double) < ymin {
                        ymin = corners[i as usize].y as libc::c_double
                    }
                    if corners[i as usize].y as libc::c_double > ymax {
                        ymax = corners[i as usize].y as libc::c_double
                    }
                    let fresh6 = i;
                    i = i + 1;
                    if !((fresh6 as libc::c_int) < for_end_5) {
                        break;
                    }
                }
            }
            corners[0].x = xmin as libc::c_float;
            corners[0].y = ymin as libc::c_float;
            corners[1].x = xmin as libc::c_float;
            corners[1].y = ymax as libc::c_float;
            corners[2].x = xmax as libc::c_float;
            corners[2].y = ymax as libc::c_float;
            corners[3].x = xmax as libc::c_float;
            corners[3].y = ymin as libc::c_float;
            transform_concat(&mut t, &mut t2);
        } else {
            check_keywords = 0i32 != 0
        }
    }
    if x_size_req != 0.0f64 || y_size_req != 0.0f64 {
        xmin = 1000000.0f64;
        xmax = -(xmin as int32_t) as libc::c_double;
        ymin = xmin;
        ymax = xmax;
        let mut for_end_6: int32_t = 0;
        i = 0i32 as small_number;
        for_end_6 = 3i32;
        if i as libc::c_int <= for_end_6 {
            loop {
                if (corners[i as usize].x as libc::c_double) < xmin {
                    xmin = corners[i as usize].x as libc::c_double
                }
                if corners[i as usize].x as libc::c_double > xmax {
                    xmax = corners[i as usize].x as libc::c_double
                }
                if (corners[i as usize].y as libc::c_double) < ymin {
                    ymin = corners[i as usize].y as libc::c_double
                }
                if corners[i as usize].y as libc::c_double > ymax {
                    ymax = corners[i as usize].y as libc::c_double
                }
                let fresh7 = i;
                i = i + 1;
                if !((fresh7 as libc::c_int) < for_end_6) {
                    break;
                }
            }
        }
        if x_size_req == 0.0f64 {
            make_scale(
                &mut t2,
                y_size_req / (ymax - ymin),
                y_size_req / (ymax - ymin),
            );
        } else if y_size_req == 0.0f64 {
            make_scale(
                &mut t2,
                x_size_req / (xmax - xmin),
                x_size_req / (xmax - xmin),
            );
        } else {
            make_scale(
                &mut t2,
                x_size_req / (xmax - xmin),
                y_size_req / (ymax - ymin),
            );
        }
        let mut for_end_7: int32_t = 0;
        i = 0i32 as small_number;
        for_end_7 = 3i32;
        if i as libc::c_int <= for_end_7 {
            loop {
                transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                let fresh8 = i;
                i = i + 1;
                if !((fresh8 as libc::c_int) < for_end_7) {
                    break;
                }
            }
        }
        x_size_req = 0.0f64;
        y_size_req = 0.0f64;
        transform_concat(&mut t, &mut t2);
    }
    xmin = 1000000.0f64;
    xmax = -(xmin as int32_t) as libc::c_double;
    ymin = xmin;
    ymax = xmax;
    let mut for_end_8: int32_t = 0;
    i = 0i32 as small_number;
    for_end_8 = 3i32;
    if i as libc::c_int <= for_end_8 {
        loop {
            if (corners[i as usize].x as libc::c_double) < xmin {
                xmin = corners[i as usize].x as libc::c_double
            }
            if corners[i as usize].x as libc::c_double > xmax {
                xmax = corners[i as usize].x as libc::c_double
            }
            if (corners[i as usize].y as libc::c_double) < ymin {
                ymin = corners[i as usize].y as libc::c_double
            }
            if corners[i as usize].y as libc::c_double > ymax {
                ymax = corners[i as usize].y as libc::c_double
            }
            let fresh9 = i;
            i = i + 1;
            if !((fresh9 as libc::c_int) < for_end_8) {
                break;
            }
        }
    }
    make_translation(
        &mut t2,
        (-(xmin as int32_t) * 72i32) as libc::c_double / 72.27f64,
        (-(ymin as int32_t) * 72i32) as libc::c_double / 72.27f64,
    );
    transform_concat(&mut t, &mut t2);
    if result == 0i32 {
        new_whatsit(
            43i32 as small_number,
            (9i32 as u64).wrapping_add(
                strlen(pic_path)
                    .wrapping_add(::std::mem::size_of::<memory_word>() as u64)
                    .wrapping_sub(1i32 as u64)
                    .wrapping_div(::std::mem::size_of::<memory_word>() as u64),
            ) as small_number,
        );
        if is_pdf {
            (*mem.offset(cur_list.tail as isize)).b16.s0 = 44i32 as uint16_t
        }
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 = strlen(pic_path) as uint16_t;
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s0 = page as uint16_t;
        (*mem.offset((cur_list.tail + 8i32) as isize)).b16.s1 = pdf_box_type as uint16_t;
        (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s1 = D2Fix(xmax - xmin);
        (*mem.offset((cur_list.tail + 3i32) as isize)).b32.s1 = D2Fix(ymax - ymin);
        (*mem.offset((cur_list.tail + 2i32) as isize)).b32.s1 = 0i32;
        (*mem.offset((cur_list.tail + 5i32) as isize)).b32.s0 = D2Fix(t.a);
        (*mem.offset((cur_list.tail + 5i32) as isize)).b32.s1 = D2Fix(t.b);
        (*mem.offset((cur_list.tail + 6i32) as isize)).b32.s0 = D2Fix(t.c);
        (*mem.offset((cur_list.tail + 6i32) as isize)).b32.s1 = D2Fix(t.d);
        (*mem.offset((cur_list.tail + 7i32) as isize)).b32.s0 = D2Fix(t.x);
        (*mem.offset((cur_list.tail + 7i32) as isize)).b32.s1 = D2Fix(t.y);
        memcpy(
            &mut *mem.offset((cur_list.tail + 9i32) as isize) as *mut memory_word
                as *mut libc::c_uchar as *mut libc::c_void,
            pic_path as *const libc::c_void,
            strlen(pic_path),
        );
        free(pic_path as *mut libc::c_void);
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(
            b"Unable to load picture or PDF file \'\x00" as *const u8 as *const libc::c_char,
        );
        print_file_name(cur_name, cur_area, cur_ext);
        print('\'' as i32);
        if result == -43i32 {
            help_ptr = 2i32 as libc::c_uchar;
            help_line[1] = b"The requested image couldn\'t be read because\x00" as *const u8
                as *const libc::c_char;
            help_line[0] = b"the file was not found.\x00" as *const u8 as *const libc::c_char
        } else {
            help_ptr = 2i32 as libc::c_uchar;
            help_line[1] = b"The requested image couldn\'t be read because\x00" as *const u8
                as *const libc::c_char;
            help_line[0] =
                b"it was not a recognized image format.\x00" as *const u8 as *const libc::c_char
        }
        error();
    };
}
