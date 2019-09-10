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
    fn cos(_: f64) -> f64;
    #[no_mangle]
    fn sin(_: f64) -> f64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
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
    fn xstrdup(s: *const i8) -> *mut i8;
    #[no_mangle]
    static mut name_of_file: *mut i8;
    #[no_mangle]
    static mut file_line_error_style_p: libc::c_int;
    #[no_mangle]
    static mut help_line: [*const i8; 6];
    #[no_mangle]
    static mut help_ptr: u8;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut cur_list: list_state_record;
    #[no_mangle]
    static mut cur_val: i32;
    #[no_mangle]
    static mut cur_name: str_number;
    #[no_mangle]
    static mut cur_area: str_number;
    #[no_mangle]
    static mut cur_ext: str_number;
    #[no_mangle]
    fn scan_keyword(s: *const i8) -> bool;
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
    fn print(s: i32);
    #[no_mangle]
    fn print_cstr(s: *const i8);
    #[no_mangle]
    fn print_nl_cstr(s: *const i8);
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
    /* info for each glyph is location (FixedPoint) + glyph ID (u16) */
    /* glyph ID field in a glyph_node */
    /* For Unicode encoding form interpretation... */
    /* single-purpose metrics accessors */
    #[no_mangle]
    fn Fix2D(f: Fixed) -> f64;
    #[no_mangle]
    fn print_file_name(n: i32, a: i32, e: i32);
    #[no_mangle]
    fn print_file_line();
    #[no_mangle]
    fn print_scaled(s: scaled_t);
    #[no_mangle]
    fn D2Fix(d: f64) -> Fixed;
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
        v1: f64,
        v2: f64,
        v3: f64,
        v4: f64,
    ) -> f64;
    #[no_mangle]
    fn max4(
        v1: f64,
        v2: f64,
        v3: f64,
        v4: f64,
    ) -> f64;
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
    fn pdf_open(ident: *const i8, handle: rust_input_handle_t) -> *mut pdf_file;
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
        xdensity: *mut f64,
        ydensity: *mut f64,
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
        xdensity: *mut f64,
        ydensity: *mut f64,
    ) -> libc::c_int;
    #[no_mangle]
    fn check_for_bmp(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn bmp_get_bbox(
        handle: rust_input_handle_t,
        width: *mut libc::c_uint,
        height: *mut libc::c_uint,
        xdensity: *mut f64,
        ydensity: *mut f64,
    ) -> libc::c_int;
}
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
pub type scaled_t = i32;
pub type Fixed = scaled_t;
pub type str_number = i32;
pub type small_number = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: i32,
    pub s1: i32,
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
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_state_record {
    pub mode: libc::c_short,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_t {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct real_point {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct real_rect {
    pub x: f32,
    pub y: f32,
    pub wd: f32,
    pub ht: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: f64,
    pub y: f64,
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
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
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
    mut filename: *mut i8,
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
    (*box_0).x = (72.27f64 / 72i32 as f64 * bbox.llx) as f32;
    (*box_0).y = (72.27f64 / 72i32 as f64 * bbox.lly) as f32;
    (*box_0).wd = (72.27f64 / 72i32 as f64 * (bbox.urx - bbox.llx)) as f32;
    (*box_0).ht = (72.27f64 / 72i32 as f64 * (bbox.ury - bbox.lly)) as f32;
    return 0i32;
}
unsafe extern "C" fn get_image_size_in_inches(
    mut handle: rust_input_handle_t,
    mut width: *mut f32,
    mut height: *mut f32,
) -> libc::c_int {
    let mut err: libc::c_int = 1i32;
    let mut width_pix: libc::c_uint = 0;
    let mut height_pix: libc::c_uint = 0;
    let mut xdensity: f64 = 0.;
    let mut ydensity: f64 = 0.;
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
        *width = -1i32 as f32;
        *height = -1i32 as f32;
        return err;
    }
    /* xdvipdfmx defines density = 72 / dpi, so ... */
    *width = (width_pix as f64 * xdensity / 72i32 as f64) as f32;
    *height = (height_pix as f64 * ydensity / 72i32 as f64) as f32;
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
    mut path: *mut *mut i8,
    mut bounds: *mut real_rect,
    mut pdfBoxType: libc::c_int,
    mut page: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = -1i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    handle = ttstub_input_open(name_of_file, TTIF_PICT, 0i32);
    (*bounds).ht = 0.0f64 as f32;
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
        (*bounds).wd = ((*bounds).wd as f64 * 72.27f64) as f32;
        (*bounds).ht = ((*bounds).ht as f64 * 72.27f64) as f32
    }
    if err == 0i32 {
        *path = xstrdup(name_of_file)
    }
    ttstub_input_close(handle);
    return err;
}
unsafe extern "C" fn transform_point(mut p: *mut real_point, mut t: *const transform_t) {
    let mut r: real_point = real_point { x: 0., y: 0. };
    r.x = ((*t).a * (*p).x as f64 + (*t).c * (*p).y as f64 + (*t).x)
        as f32;
    r.y = ((*t).b * (*p).x as f64 + (*t).d * (*p).y as f64 + (*t).y)
        as f32;
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
    mut xscale: f64,
    mut yscale: f64,
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
    mut dx: f64,
    mut dy: f64,
) {
    (*t).a = 1.0f64;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = 1.0f64;
    (*t).x = dx;
    (*t).y = dy;
}
unsafe extern "C" fn make_rotation(mut t: *mut transform_t, mut a: f64) {
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
    let mut pic_path: *mut i8 = 0 as *mut i8;
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
    let mut x_size_req: f64 = 0.;
    let mut y_size_req: f64 = 0.;
    let mut check_keywords: bool = false;
    let mut xmin: f64 = 0.;
    let mut xmax: f64 = 0.;
    let mut ymin: f64 = 0.;
    let mut ymax: f64 = 0.;
    let mut i: small_number = 0;
    let mut page: i32 = 0;
    let mut pdf_box_type: i32 = 0;
    let mut result: i32 = 0;
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
    pdf_box_type = 0i32;
    page = 0i32;
    if is_pdf {
        if scan_keyword(b"page\x00" as *const u8 as *const i8) {
            scan_int();
            page = cur_val
        }
        pdf_box_type = 6i32;
        if scan_keyword(b"crop\x00" as *const u8 as *const i8) {
            pdf_box_type = 1i32
        } else if scan_keyword(b"media\x00" as *const u8 as *const i8) {
            pdf_box_type = 2i32
        } else if scan_keyword(b"bleed\x00" as *const u8 as *const i8) {
            pdf_box_type = 3i32
        } else if scan_keyword(b"trim\x00" as *const u8 as *const i8) {
            pdf_box_type = 4i32
        } else if scan_keyword(b"art\x00" as *const u8 as *const i8) {
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
        if scan_keyword(b"scaled\x00" as *const u8 as *const i8) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(
                    &mut t2,
                    cur_val as f64 / 1000.0f64,
                    cur_val as f64 / 1000.0f64,
                );
                let mut for_end: i32 = 0;
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
        } else if scan_keyword(b"xscaled\x00" as *const u8 as *const i8) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(&mut t2, cur_val as f64 / 1000.0f64, 1.0f64);
                let mut for_end_0: i32 = 0;
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
        } else if scan_keyword(b"yscaled\x00" as *const u8 as *const i8) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(&mut t2, 1.0f64, cur_val as f64 / 1000.0f64);
                let mut for_end_1: i32 = 0;
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
        } else if scan_keyword(b"width\x00" as *const u8 as *const i8) {
            scan_dimen(0i32 != 0, 0i32 != 0, 0i32 != 0);
            if cur_val <= 0i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                }
                print_cstr(b"Improper image \x00" as *const u8 as *const i8);
                print_cstr(b"size (\x00" as *const u8 as *const i8);
                print_scaled(cur_val);
                print_cstr(b"pt) will be ignored\x00" as *const u8 as *const i8);
                help_ptr = 2i32 as u8;
                help_line[1] = b"I can\'t scale images to zero or negative sizes,\x00" as *const u8
                    as *const i8;
                help_line[0] = b"so I\'m ignoring this.\x00" as *const u8 as *const i8;
                error();
            } else {
                x_size_req = Fix2D(cur_val)
            }
        } else if scan_keyword(b"height\x00" as *const u8 as *const i8) {
            scan_dimen(0i32 != 0, 0i32 != 0, 0i32 != 0);
            if cur_val <= 0i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                }
                print_cstr(b"Improper image \x00" as *const u8 as *const i8);
                print_cstr(b"size (\x00" as *const u8 as *const i8);
                print_scaled(cur_val);
                print_cstr(b"pt) will be ignored\x00" as *const u8 as *const i8);
                help_ptr = 2i32 as u8;
                help_line[1] = b"I can\'t scale images to zero or negative sizes,\x00" as *const u8
                    as *const i8;
                help_line[0] = b"so I\'m ignoring this.\x00" as *const u8 as *const i8;
                error();
            } else {
                y_size_req = Fix2D(cur_val)
            }
        } else if scan_keyword(b"rotated\x00" as *const u8 as *const i8) {
            scan_decimal();
            if x_size_req != 0.0f64 || y_size_req != 0.0f64 {
                xmin = 1000000.0f64;
                xmax = -(xmin as i32) as f64;
                ymin = xmin;
                ymax = xmax;
                let mut for_end_2: i32 = 0;
                i = 0i32 as small_number;
                for_end_2 = 3i32;
                if i as libc::c_int <= for_end_2 {
                    loop {
                        if (corners[i as usize].x as f64) < xmin {
                            xmin = corners[i as usize].x as f64
                        }
                        if corners[i as usize].x as f64 > xmax {
                            xmax = corners[i as usize].x as f64
                        }
                        if (corners[i as usize].y as f64) < ymin {
                            ymin = corners[i as usize].y as f64
                        }
                        if corners[i as usize].y as f64 > ymax {
                            ymax = corners[i as usize].y as f64
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
                let mut for_end_3: i32 = 0;
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
            let mut for_end_4: i32 = 0;
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
            xmax = -(xmin as i32) as f64;
            ymin = xmin;
            ymax = xmax;
            let mut for_end_5: i32 = 0;
            i = 0i32 as small_number;
            for_end_5 = 3i32;
            if i as libc::c_int <= for_end_5 {
                loop {
                    if (corners[i as usize].x as f64) < xmin {
                        xmin = corners[i as usize].x as f64
                    }
                    if corners[i as usize].x as f64 > xmax {
                        xmax = corners[i as usize].x as f64
                    }
                    if (corners[i as usize].y as f64) < ymin {
                        ymin = corners[i as usize].y as f64
                    }
                    if corners[i as usize].y as f64 > ymax {
                        ymax = corners[i as usize].y as f64
                    }
                    let fresh6 = i;
                    i = i + 1;
                    if !((fresh6 as libc::c_int) < for_end_5) {
                        break;
                    }
                }
            }
            corners[0].x = xmin as f32;
            corners[0].y = ymin as f32;
            corners[1].x = xmin as f32;
            corners[1].y = ymax as f32;
            corners[2].x = xmax as f32;
            corners[2].y = ymax as f32;
            corners[3].x = xmax as f32;
            corners[3].y = ymin as f32;
            transform_concat(&mut t, &mut t2);
        } else {
            check_keywords = 0i32 != 0
        }
    }
    if x_size_req != 0.0f64 || y_size_req != 0.0f64 {
        xmin = 1000000.0f64;
        xmax = -(xmin as i32) as f64;
        ymin = xmin;
        ymax = xmax;
        let mut for_end_6: i32 = 0;
        i = 0i32 as small_number;
        for_end_6 = 3i32;
        if i as libc::c_int <= for_end_6 {
            loop {
                if (corners[i as usize].x as f64) < xmin {
                    xmin = corners[i as usize].x as f64
                }
                if corners[i as usize].x as f64 > xmax {
                    xmax = corners[i as usize].x as f64
                }
                if (corners[i as usize].y as f64) < ymin {
                    ymin = corners[i as usize].y as f64
                }
                if corners[i as usize].y as f64 > ymax {
                    ymax = corners[i as usize].y as f64
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
        let mut for_end_7: i32 = 0;
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
    xmax = -(xmin as i32) as f64;
    ymin = xmin;
    ymax = xmax;
    let mut for_end_8: i32 = 0;
    i = 0i32 as small_number;
    for_end_8 = 3i32;
    if i as libc::c_int <= for_end_8 {
        loop {
            if (corners[i as usize].x as f64) < xmin {
                xmin = corners[i as usize].x as f64
            }
            if corners[i as usize].x as f64 > xmax {
                xmax = corners[i as usize].x as f64
            }
            if (corners[i as usize].y as f64) < ymin {
                ymin = corners[i as usize].y as f64
            }
            if corners[i as usize].y as f64 > ymax {
                ymax = corners[i as usize].y as f64
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
        (-(xmin as i32) * 72i32) as f64 / 72.27f64,
        (-(ymin as i32) * 72i32) as f64 / 72.27f64,
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
            (*mem.offset(cur_list.tail as isize)).b16.s0 = 44i32 as u16
        }
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 = strlen(pic_path) as u16;
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s0 = page as u16;
        (*mem.offset((cur_list.tail + 8i32) as isize)).b16.s1 = pdf_box_type as u16;
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
                as *mut u8 as *mut libc::c_void,
            pic_path as *const libc::c_void,
            strlen(pic_path),
        );
        free(pic_path as *mut libc::c_void);
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const i8);
        }
        print_cstr(
            b"Unable to load picture or PDF file \'\x00" as *const u8 as *const i8,
        );
        print_file_name(cur_name, cur_area, cur_ext);
        print('\'' as i32);
        if result == -43i32 {
            help_ptr = 2i32 as u8;
            help_line[1] = b"The requested image couldn\'t be read because\x00" as *const u8
                as *const i8;
            help_line[0] = b"the file was not found.\x00" as *const u8 as *const i8
        } else {
            help_ptr = 2i32 as u8;
            help_line[1] = b"The requested image couldn\'t be read because\x00" as *const u8
                as *const i8;
            help_line[0] =
                b"it was not a recognized image format.\x00" as *const u8 as *const i8
        }
        error();
    };
}
