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

use crate::dpx_pdfparse::parse_number;
use crate::mfree;
use crate::warn;
use crate::{streq_ptr, strstartswith};

use super::dpx_pdfcolor::{pdf_color_cmykcolor, pdf_color_graycolor, pdf_color_rgbcolor};
use super::dpx_pdfdev::{
    pdf_coord, pdf_dev_put_image, pdf_rect, pdf_tmatrix, transform_info, transform_info_clear,
};
use super::dpx_pdfdoc::{pdf_doc_begin_grabbing, pdf_doc_set_mediabox};
use super::dpx_pdfdraw::{
    pdf_dev_concat, pdf_dev_currentmatrix, pdf_dev_currentpoint, pdf_dev_dtransform,
    pdf_dev_idtransform, pdf_dev_set_color,
};
use crate::dpx_pdfobj::{
    pdf_add_dict, pdf_array_length, pdf_file, pdf_get_array, pdf_lookup_dict, pdf_name_value,
    pdf_new_dict, pdf_new_name, pdf_new_number, pdf_number_value, pdf_obj, pdf_obj_typeof,
    pdf_release_obj, pdf_set_number, pdf_string_length, pdf_string_value,
};
use crate::dpx_pdfparse::{
    parse_ident, parse_pdf_array, parse_pdf_dict, parse_pdf_name, parse_pdf_string,
    pdfparse_skip_line,
};
use libc::free;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* Here is the complete list of PDF object types */
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
    #[no_mangle]
    fn file_size(file: *mut FILE) -> i32;
    /* Name does not include the / */
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    /* returns 1.0/unit_conv */
    #[no_mangle]
    fn dev_unit_dviunit() -> f64;
    /* Draw texts and rules:
     *
     * xpos, ypos, width, and height are all fixed-point numbers
     * converted to big-points by multiplying unit_conv (dvi2pts).
     * They must be position in the user space.
     *
     * ctype:
     *   0 - input string is in multi-byte encoding.
     *   1 - input string is in 8-bit encoding.
     *   2 - input string is in 16-bit encoding.
     */
    #[no_mangle]
    fn pdf_dev_set_string(
        xpos: spt_t,
        ypos: spt_t,
        instr_ptr: *const libc::c_void,
        instr_len: size_t,
        text_width: spt_t,
        font_id: i32,
        ctype: i32,
    );
    /* The design_size and ptsize required by PK font support...
     */
    #[no_mangle]
    fn pdf_dev_locate_font(font_name: *const i8, ptsize: spt_t) -> i32;
    /* Access text state parameters. */
    #[no_mangle]
    fn pdf_dev_get_font_wmode(font_id: i32) -> i32;
    /* ps: special support want this (pTeX). */
    /* Text composition (direction) mode
     * This affects only when auto_rotate is enabled.
     */
    #[no_mangle]
    fn pdf_dev_get_dirmode() -> i32;
    #[no_mangle]
    fn pdf_dev_set_dirmode(dir_mode: i32);
    /* Accessor to various device parameters.
     */
    #[no_mangle]
    fn pdf_dev_get_param(param_type: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_set_param(param_type: i32, value: i32);
    /* Text is normal and line art is not normal in dvipdfmx. So we don't have
     * begin_text (BT in PDF) and end_text (ET), but instead we have graphics_mode()
     * to terminate text section. pdf_dev_flushpath() and others call this.
     */
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    static mut translate_origin: i32;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn pdf_lookup_fontmap_record(kp: *const i8) -> *mut fontmap_rec;
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_doc_current_page_number() -> i32;
    /* Page */
    #[no_mangle]
    fn pdf_doc_begin_page(scale: f64, x_origin: f64, y_origin: f64);
    #[no_mangle]
    fn pdf_doc_end_page();
    /* Returns xobj_id of started xform. */
    #[no_mangle]
    fn pdf_doc_end_grabbing(attrib: *mut pdf_obj);
    #[no_mangle]
    fn pdf_dev_setlinewidth(width: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_setmiterlimit(mlimit: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_setlinecap(style: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_setlinejoin(style: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_setdash(count: i32, pattern: *mut f64, offset: f64) -> i32;
    /* Path Construction */
    #[no_mangle]
    fn pdf_dev_moveto(x: f64, y: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_rmoveto(x: f64, y: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_closepath() -> i32;
    #[no_mangle]
    fn pdf_dev_lineto(x0: f64, y0: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_rlineto(x0: f64, y0: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_curveto(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_rcurveto(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_arc(c_x: f64, c_y: f64, r: f64, a_0: f64, a_1: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_arcn(c_x: f64, c_y: f64, r: f64, a_0: f64, a_1: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_newpath() -> i32;
    /* Path Painting */
    #[no_mangle]
    fn pdf_dev_clip() -> i32;
    #[no_mangle]
    fn pdf_dev_eoclip() -> i32;
    #[no_mangle]
    fn pdf_dev_flushpath(p_op: i8, fill_rule: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_gsave() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore() -> i32;
    /* Please remove this */
    #[no_mangle]
    fn dump(start: *const i8, end: *const i8);
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    #[no_mangle]
    fn lookup_sfd_record(rec_id: i32, code: u8) -> u16;
    #[no_mangle]
    fn sfd_load_record(sfd_name: *const i8, subfont_id: *const i8) -> i32;
    #[no_mangle]
    fn tfm_open(tex_name: *const i8, must_exist: i32) -> i32;
    #[no_mangle]
    fn tfm_get_width(font_id: i32, ch: i32) -> f64;
    #[no_mangle]
    fn tfm_string_width(font_id: i32, s: *const u8, len: u32) -> fixword;
    #[no_mangle]
    fn tfm_exists(tfm_name: *const i8) -> bool;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
use libc::FILE;
pub type fixword = i32;

pub use super::dpx_pdfcolor::pdf_color;

pub type spt_t = i32;

use super::dpx_fontmap::fontmap_rec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp_font {
    pub font_name: *mut i8,
    pub font_id: i32,
    pub tfm_id: i32,
    pub subfont_id: i32,
    pub pt_size: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operators {
    pub token: *const i8,
    pub opcode: i32,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
/*
 * Define the origin as (llx, lly) in order to
 * match the new xetex.def and dvipdfmx.def
 */
static mut Xorigin: f64 = 0.;
static mut Yorigin: f64 = 0.;
static mut font_stack: [mp_font; 256] = [
    {
        let mut init = mp_font {
            font_name: 0 as *const i8 as *mut i8,
            font_id: -1i32,
            tfm_id: -1i32,
            subfont_id: -1i32,
            pt_size: 0i32 as f64,
        }; /* No currentfont */
        init
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const i8 as *mut i8,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
];
static mut currentfont: i32 = -1i32;
static mut mp_cmode: i32 = 0i32;
unsafe extern "C" fn mp_setfont(mut font_name: *const i8, mut pt_size: f64) -> i32 {
    let mut name: *const i8 = font_name;
    let mut font: *mut mp_font = 0 as *mut mp_font;
    let mut subfont_id: i32 = -1i32;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    font = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if !font.is_null() {
        if streq_ptr((*font).font_name, font_name) as i32 != 0 && (*font).pt_size == pt_size {
            return 0i32;
        }
    } else {
        /* ***TODO*** Here some problem exists! */
        font = &mut *font_stack.as_mut_ptr().offset(0) as *mut mp_font;
        (*font).font_name = 0 as *mut i8;
        currentfont = 0i32
    }
    mrec = pdf_lookup_fontmap_record(font_name);
    if !mrec.is_null()
        && !(*mrec).charmap.sfd_name.is_null()
        && !(*mrec).charmap.subfont_id.is_null()
    {
        subfont_id = sfd_load_record((*mrec).charmap.sfd_name, (*mrec).charmap.subfont_id)
    }
    /* See comments in dvi_locate_font() in dvi.c. */
    if !mrec.is_null() && !(*mrec).map_name.is_null() {
        name = (*mrec).map_name
    } else {
        name = font_name
    } /* Need not exist in MP mode */
    free((*font).font_name as *mut libc::c_void);
    (*font).font_name = new((strlen(font_name).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    strcpy((*font).font_name, font_name);
    (*font).subfont_id = subfont_id;
    (*font).pt_size = pt_size;
    (*font).tfm_id = tfm_open(font_name, 0i32);
    (*font).font_id = pdf_dev_locate_font(name, (pt_size * dev_unit_dviunit()) as spt_t);
    if (*font).font_id < 0i32 {
        _tt_abort(
            b"MPOST: No physical font assigned for \"%s\".\x00" as *const u8 as *const i8,
            font_name,
        );
    }
    0i32
}
unsafe extern "C" fn save_font() {
    let mut current: *mut mp_font = 0 as *mut mp_font;
    let mut next: *mut mp_font = 0 as *mut mp_font;
    if currentfont < 0i32 {
        font_stack[0].font_name = new((strlen(b"Courier\x00" as *const u8 as *const i8)
            .wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy(
            font_stack[0].font_name,
            b"Courier\x00" as *const u8 as *const i8,
        );
        font_stack[0].pt_size = 1i32 as f64;
        font_stack[0].tfm_id = 0i32;
        font_stack[0].subfont_id = 0i32;
        currentfont = 0i32
    }
    let fresh0 = currentfont;
    currentfont = currentfont + 1;
    current = &mut *font_stack.as_mut_ptr().offset(fresh0 as isize) as *mut mp_font;
    next = &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font;
    (*next).font_name = new(
        (strlen((*current).font_name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    strcpy((*next).font_name, (*current).font_name);
    (*next).pt_size = (*current).pt_size;
    (*next).subfont_id = (*current).subfont_id;
    (*next).tfm_id = (*current).tfm_id;
}
unsafe extern "C" fn restore_font() {
    let mut current: *mut mp_font = 0 as *mut mp_font;
    current = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if !current.is_null() {
        (*current).font_name = mfree((*current).font_name as *mut libc::c_void) as *mut i8
    } else {
        panic!("No currentfont...");
    }
    currentfont -= 1;
}
unsafe extern "C" fn clear_fonts() {
    while currentfont >= 0i32 {
        free(font_stack[currentfont as usize].font_name as *mut libc::c_void);
        currentfont -= 1
    }
}
unsafe extern "C" fn is_fontname(mut token: *const i8) -> bool {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    mrec = pdf_lookup_fontmap_record(token);
    if !mrec.is_null() {
        return true;
    }
    tfm_exists(token)
}
#[no_mangle]
pub unsafe extern "C" fn mps_scan_bbox(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    bbox: &mut pdf_rect,
) -> i32 {
    let mut number: *mut i8 = 0 as *mut i8;
    let mut values: [f64; 4] = [0.; 4];
    let mut i: i32 = 0;
    /* skip_white() skips lines starting '%'... */
    while *pp < endptr && libc::isspace(**pp as _) != 0 {
        *pp = (*pp).offset(1)
    }
    /* Scan for bounding box record */
    while *pp < endptr && **pp as i32 == '%' as i32 {
        if (*pp).offset(14) < endptr
            && !strstartswith(*pp, b"%%BoundingBox:\x00" as *const u8 as *const i8).is_null()
        {
            *pp = (*pp).offset(14);
            i = 0i32;
            while i < 4i32 {
                skip_white(pp, endptr);
                number = parse_number(pp, endptr);
                if number.is_null() {
                    break;
                }
                values[i as usize] = atof(number);
                free(number as *mut libc::c_void);
                i += 1
            }
            if i < 4i32 {
                return -1i32;
            } else {
                /* The new xetex.def and dvipdfmx.def require bbox->llx = bbox->lly = 0.  */
                if translate_origin != 0 {
                    bbox.llx = 0i32 as f64;
                    bbox.lly = 0i32 as f64;
                    bbox.urx = values[2] - values[0];
                    bbox.ury = values[3] - values[1];
                    Xorigin = values[0];
                    Yorigin = values[1]
                } else {
                    bbox.llx = values[0];
                    bbox.lly = values[1];
                    bbox.urx = values[2];
                    bbox.ury = values[3];
                    Xorigin = 0.0f64;
                    Yorigin = 0.0f64
                }
                return 0i32;
            }
        }
        pdfparse_skip_line(pp, endptr);
        while *pp < endptr && libc::isspace(**pp as _) != 0 {
            *pp = (*pp).offset(1)
        }
    }
    -1i32
}
unsafe extern "C" fn skip_prolog(mut start: *mut *const i8, mut end: *const i8) {
    let mut found_prolog: i32 = 0i32;
    let mut save: *const i8 = 0 as *const i8;
    save = *start;
    while *start < end {
        if **start as i32 != '%' as i32 {
            skip_white(start, end);
        }
        if *start >= end {
            break;
        }
        if !strstartswith(*start, b"%%EndProlog\x00" as *const u8 as *const i8).is_null() {
            found_prolog = 1i32;
            pdfparse_skip_line(start, end);
            break;
        } else if !strstartswith(*start, b"%%Page:\x00" as *const u8 as *const i8).is_null() {
            pdfparse_skip_line(start, end);
            break;
        } else {
            pdfparse_skip_line(start, end);
        }
    }
    if found_prolog == 0 {
        *start = save
    };
}
static mut ps_operators: [operators; 48] = [
    {
        let mut init = operators {
            token: b"add\x00" as *const u8 as *const i8,
            opcode: 1i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"mul\x00" as *const u8 as *const i8,
            opcode: 3i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"div\x00" as *const u8 as *const i8,
            opcode: 4i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"neg\x00" as *const u8 as *const i8,
            opcode: 5i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"sub\x00" as *const u8 as *const i8,
            opcode: 2i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"truncate\x00" as *const u8 as *const i8,
            opcode: 6i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"clear\x00" as *const u8 as *const i8,
            opcode: 10i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"exch\x00" as *const u8 as *const i8,
            opcode: 11i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"pop\x00" as *const u8 as *const i8,
            opcode: 12i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"clip\x00" as *const u8 as *const i8,
            opcode: 44i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"eoclip\x00" as *const u8 as *const i8,
            opcode: 45i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"closepath\x00" as *const u8 as *const i8,
            opcode: 32i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"concat\x00" as *const u8 as *const i8,
            opcode: 52i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"newpath\x00" as *const u8 as *const i8,
            opcode: 31i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"moveto\x00" as *const u8 as *const i8,
            opcode: 33i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rmoveto\x00" as *const u8 as *const i8,
            opcode: 34i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"lineto\x00" as *const u8 as *const i8,
            opcode: 37i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rlineto\x00" as *const u8 as *const i8,
            opcode: 38i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"curveto\x00" as *const u8 as *const i8,
            opcode: 35i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rcurveto\x00" as *const u8 as *const i8,
            opcode: 36i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"arc\x00" as *const u8 as *const i8,
            opcode: 39i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"arcn\x00" as *const u8 as *const i8,
            opcode: 40i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"stroke\x00" as *const u8 as *const i8,
            opcode: 42i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"fill\x00" as *const u8 as *const i8,
            opcode: 41i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"show\x00" as *const u8 as *const i8,
            opcode: 43i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"showpage\x00" as *const u8 as *const i8,
            opcode: 49i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"gsave\x00" as *const u8 as *const i8,
            opcode: 50i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"grestore\x00" as *const u8 as *const i8,
            opcode: 51i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"translate\x00" as *const u8 as *const i8,
            opcode: 54i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rotate\x00" as *const u8 as *const i8,
            opcode: 55i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"scale\x00" as *const u8 as *const i8,
            opcode: 53i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setlinecap\x00" as *const u8 as *const i8,
            opcode: 62i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setlinejoin\x00" as *const u8 as *const i8,
            opcode: 63i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setlinewidth\x00" as *const u8 as *const i8,
            opcode: 60i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setmiterlimit\x00" as *const u8 as *const i8,
            opcode: 64i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setdash\x00" as *const u8 as *const i8,
            opcode: 61i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setgray\x00" as *const u8 as *const i8,
            opcode: 70i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setrgbcolor\x00" as *const u8 as *const i8,
            opcode: 71i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setcmykcolor\x00" as *const u8 as *const i8,
            opcode: 72i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"currentpoint\x00" as *const u8 as *const i8,
            opcode: 80i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"dtransform\x00" as *const u8 as *const i8,
            opcode: 82i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"idtransform\x00" as *const u8 as *const i8,
            opcode: 81i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"findfont\x00" as *const u8 as *const i8,
            opcode: 201i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"scalefont\x00" as *const u8 as *const i8,
            opcode: 202i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setfont\x00" as *const u8 as *const i8,
            opcode: 203i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"currentfont\x00" as *const u8 as *const i8,
            opcode: 204i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"stringwidth\x00" as *const u8 as *const i8,
            opcode: 210i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"def\x00" as *const u8 as *const i8,
            opcode: 999i32,
        };
        init
    },
];
static mut mps_operators: [operators; 28] = [
    {
        let mut init = operators {
            token: b"fshow\x00" as *const u8 as *const i8,
            opcode: 1001i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"startTexFig\x00" as *const u8 as *const i8,
            opcode: 1002i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"endTexFig\x00" as *const u8 as *const i8,
            opcode: 1003i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"hlw\x00" as *const u8 as *const i8,
            opcode: 1004i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"vlw\x00" as *const u8 as *const i8,
            opcode: 1005i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"l\x00" as *const u8 as *const i8,
            opcode: 37i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"r\x00" as *const u8 as *const i8,
            opcode: 38i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"c\x00" as *const u8 as *const i8,
            opcode: 35i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"m\x00" as *const u8 as *const i8,
            opcode: 33i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"p\x00" as *const u8 as *const i8,
            opcode: 32i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"n\x00" as *const u8 as *const i8,
            opcode: 31i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"C\x00" as *const u8 as *const i8,
            opcode: 72i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"G\x00" as *const u8 as *const i8,
            opcode: 70i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"R\x00" as *const u8 as *const i8,
            opcode: 71i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"lj\x00" as *const u8 as *const i8,
            opcode: 63i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"ml\x00" as *const u8 as *const i8,
            opcode: 64i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"lc\x00" as *const u8 as *const i8,
            opcode: 62i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"S\x00" as *const u8 as *const i8,
            opcode: 42i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"F\x00" as *const u8 as *const i8,
            opcode: 41i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"q\x00" as *const u8 as *const i8,
            opcode: 50i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"Q\x00" as *const u8 as *const i8,
            opcode: 51i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"s\x00" as *const u8 as *const i8,
            opcode: 53i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"t\x00" as *const u8 as *const i8,
            opcode: 52i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"sd\x00" as *const u8 as *const i8,
            opcode: 61i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rd\x00" as *const u8 as *const i8,
            opcode: 1006i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"P\x00" as *const u8 as *const i8,
            opcode: 49i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"B\x00" as *const u8 as *const i8,
            opcode: 1007i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"W\x00" as *const u8 as *const i8,
            opcode: 44i32,
        };
        init
    },
];
unsafe extern "C" fn get_opcode(mut token: *const i8) -> i32 {
    let mut i: u32 = 0;
    i = 0_u32;
    while (i as u64)
        < (::std::mem::size_of::<[operators; 48]>() as u64)
            .wrapping_div(::std::mem::size_of::<operators>() as u64)
    {
        if streq_ptr(token, ps_operators[i as usize].token) {
            return ps_operators[i as usize].opcode;
        }
        i = i.wrapping_add(1)
    }
    i = 0_u32;
    while (i as u64)
        < (::std::mem::size_of::<[operators; 28]>() as u64)
            .wrapping_div(::std::mem::size_of::<operators>() as u64)
    {
        if streq_ptr(token, mps_operators[i as usize].token) {
            return mps_operators[i as usize].opcode;
        }
        i = i.wrapping_add(1)
    }
    -1i32
}
static mut stack: [*mut pdf_obj; 1024] = [0 as *const pdf_obj as *mut pdf_obj; 1024];
static mut top_stack: u32 = 0_u32;
unsafe extern "C" fn do_exch() -> i32 {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    if top_stack < 2_u32 {
        return -1i32;
    }
    tmp = stack[top_stack.wrapping_sub(1_u32) as usize];
    stack[top_stack.wrapping_sub(1_u32) as usize] = stack[top_stack.wrapping_sub(2_u32) as usize];
    stack[top_stack.wrapping_sub(2_u32) as usize] = tmp;
    0i32
}
unsafe extern "C" fn do_clear() -> i32 {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    while top_stack > 0_u32 {
        tmp = if top_stack > 0_u32 {
            top_stack = top_stack.wrapping_sub(1);
            stack[top_stack as usize]
        } else {
            0 as *mut pdf_obj
        };
        pdf_release_obj(tmp);
    }
    0i32
}
unsafe extern "C" fn pop_get_numbers(mut values: *mut f64, mut count: i32) -> i32 {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    loop {
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        tmp = if top_stack > 0_u32 {
            top_stack = top_stack.wrapping_sub(1);
            stack[top_stack as usize]
        } else {
            0 as *mut pdf_obj
        };
        if tmp.is_null() {
            warn!("mpost: Stack underflow.");
            break;
        } else if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
            warn!("mpost: Not a number!");
            pdf_release_obj(tmp);
            break;
        } else {
            *values.offset(count as isize) = pdf_number_value(tmp);
            pdf_release_obj(tmp);
        }
    }
    count + 1i32
}
unsafe extern "C" fn cvr_array(
    mut array: *mut pdf_obj,
    mut values: *mut f64,
    mut count: i32,
) -> i32 {
    if !(!array.is_null() && pdf_obj_typeof(array) == 5i32) {
        warn!("mpost: Not an array!");
    } else {
        let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
        loop {
            let fresh2 = count;
            count = count - 1;
            if !(fresh2 > 0i32) {
                break;
            }
            tmp = pdf_get_array(array, count);
            if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
                warn!("mpost: Not a number!");
                break;
            } else {
                *values.offset(count as isize) = pdf_number_value(tmp)
            }
        }
    }
    pdf_release_obj(array);
    count + 1i32
}
unsafe extern "C" fn is_fontdict(mut dict: *mut pdf_obj) -> bool {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    if !(!dict.is_null() && pdf_obj_typeof(dict) == 6i32) {
        return false;
    }
    tmp = pdf_lookup_dict(dict, b"Type\x00" as *const u8 as *const i8);
    if tmp.is_null()
        || !(!tmp.is_null() && pdf_obj_typeof(tmp) == 4i32)
        || strcmp(pdf_name_value(tmp), b"Font\x00" as *const u8 as *const i8) != 0
    {
        return false;
    }
    tmp = pdf_lookup_dict(dict, b"FontName\x00" as *const u8 as *const i8);
    if tmp.is_null() || !(!tmp.is_null() && pdf_obj_typeof(tmp) == 4i32) {
        return false;
    }
    tmp = pdf_lookup_dict(dict, b"FontScale\x00" as *const u8 as *const i8);
    if tmp.is_null() || !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
        return false;
    }
    true
}
unsafe extern "C" fn do_findfont() -> i32 {
    let mut error: i32 = 0i32;
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut font_name: *mut pdf_obj = 0 as *mut pdf_obj;
    font_name = if top_stack > 0_u32 {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        0 as *mut pdf_obj
    };
    if font_name.is_null() {
        return 1i32;
    } else {
        if !font_name.is_null() && pdf_obj_typeof(font_name) == 3i32
            || !font_name.is_null() && pdf_obj_typeof(font_name) == 4i32
        {
            /* Do not check the existence...
             * The reason for this is that we cannot locate PK font without
             * font scale.
             */
            font_dict = pdf_new_dict();
            pdf_add_dict(
                font_dict,
                pdf_new_name(b"Type\x00" as *const u8 as *const i8),
                pdf_new_name(b"Font\x00" as *const u8 as *const i8),
            );
            if !font_name.is_null() && pdf_obj_typeof(font_name) == 3i32 {
                pdf_add_dict(
                    font_dict,
                    pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
                    pdf_new_name(pdf_string_value(font_name) as *const i8),
                );
                pdf_release_obj(font_name);
            } else {
                pdf_add_dict(
                    font_dict,
                    pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
                    font_name,
                );
            }
            pdf_add_dict(
                font_dict,
                pdf_new_name(b"FontScale\x00" as *const u8 as *const i8),
                pdf_new_number(1.0f64),
            );
            if top_stack < 1024_u32 {
                let fresh3 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh3 as usize] = font_dict
            } else {
                warn!("PS stack overflow including MetaPost file or inline PS code");
                pdf_release_obj(font_dict);
                error = 1i32
            }
        } else {
            error = 1i32
        }
    }
    error
}
unsafe extern "C" fn do_scalefont() -> i32 {
    let mut error: i32 = 0i32;
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut font_scale: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut scale: f64 = 0.;
    error = pop_get_numbers(&mut scale, 1i32);
    if error != 0 {
        return error;
    }
    font_dict = if top_stack > 0_u32 {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        0 as *mut pdf_obj
    };
    if font_dict.is_null() {
        error = 1i32
    } else if is_fontdict(font_dict) {
        font_scale = pdf_lookup_dict(font_dict, b"FontScale\x00" as *const u8 as *const i8);
        pdf_set_number(font_scale, pdf_number_value(font_scale) * scale);
        if top_stack < 1024_u32 {
            let fresh4 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh4 as usize] = font_dict
        } else {
            warn!("PS stack overflow including MetaPost file or inline PS code");
            pdf_release_obj(font_dict);
            error = 1i32
        }
    } else {
        error = 1i32
    }
    error
}
unsafe extern "C" fn do_setfont() -> i32 {
    let mut error: i32 = 0i32;
    let mut font_name: *mut i8 = 0 as *mut i8;
    let mut font_scale: f64 = 0.;
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    font_dict = if top_stack > 0_u32 {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        0 as *mut pdf_obj
    };
    if !is_fontdict(font_dict) {
        error = 1i32
    } else {
        /* Subfont support prevent us from managing
         * font in a single place...
         */
        font_name = pdf_name_value(pdf_lookup_dict(
            font_dict,
            b"FontName\x00" as *const u8 as *const i8,
        ));
        font_scale = pdf_number_value(pdf_lookup_dict(
            font_dict,
            b"FontScale\x00" as *const u8 as *const i8,
        ));
        error = mp_setfont(font_name, font_scale)
    }
    pdf_release_obj(font_dict);
    error
}
/* Push dummy font dict onto PS stack */
unsafe extern "C" fn do_currentfont() -> i32 {
    let mut error: i32 = 0i32; /* Should not be error... */
    let mut font: *mut mp_font = 0 as *mut mp_font; /* Should not be error... */
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    font = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if font.is_null() {
        warn!("Currentfont undefined...");
        return 1i32;
    } else {
        font_dict = pdf_new_dict();
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"Type\x00" as *const u8 as *const i8),
            pdf_new_name(b"Font\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
            pdf_new_name((*font).font_name),
        );
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontScale\x00" as *const u8 as *const i8),
            pdf_new_number((*font).pt_size),
        );
        if top_stack < 1024_u32 {
            let fresh5 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh5 as usize] = font_dict
        } else {
            warn!("PS stack overflow...");
            pdf_release_obj(font_dict);
            error = 1i32
        }
    }
    error
}
unsafe extern "C" fn do_show() -> i32 {
    let mut font: *mut mp_font = 0 as *mut mp_font;
    let mut cp = pdf_coord::new();
    let mut text_str: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut length: i32 = 0;
    let mut strptr: *mut u8 = 0 as *mut u8;
    let mut text_width: f64 = 0.;
    font = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if font.is_null() {
        warn!("Currentfont not set.");
        return 1i32;
    }
    pdf_dev_currentpoint(&mut cp);
    text_str = if top_stack > 0_u32 {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        0 as *mut pdf_obj
    };
    if !(!text_str.is_null() && pdf_obj_typeof(text_str) == 3i32) {
        pdf_release_obj(text_str);
        return 1i32;
    }
    if (*font).font_id < 0i32 {
        warn!("mpost: not set.");
        pdf_release_obj(text_str);
        return 1i32;
    }
    strptr = pdf_string_value(text_str) as *mut u8;
    length = pdf_string_length(text_str) as i32;
    if (*font).tfm_id < 0i32 {
        dpx_warning(
            b"mpost: TFM not found for \"%s\".\x00" as *const u8 as *const i8,
            (*font).font_name,
        );
        warn!("mpost: Text width not calculated...");
    }
    text_width = 0.0f64;
    if (*font).subfont_id >= 0i32 {
        let mut uch: u16 = 0;
        let mut ustr: *mut u8 = 0 as *mut u8;
        let mut i: i32 = 0;
        ustr = new(
            ((length * 2i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
        ) as *mut u8;
        i = 0i32;
        while i < length {
            uch = lookup_sfd_record((*font).subfont_id, *strptr.offset(i as isize));
            *ustr.offset((2i32 * i) as isize) = (uch as i32 >> 8i32) as u8;
            *ustr.offset((2i32 * i + 1i32) as isize) = (uch as i32 & 0xffi32) as u8;
            if (*font).tfm_id >= 0i32 {
                text_width += tfm_get_width((*font).tfm_id, *strptr.offset(i as isize) as i32)
            }
            i += 1
        }
        text_width *= (*font).pt_size;
        pdf_dev_set_string(
            (cp.x * dev_unit_dviunit()) as spt_t,
            (cp.y * dev_unit_dviunit()) as spt_t,
            ustr as *const libc::c_void,
            (length * 2i32) as size_t,
            (text_width * dev_unit_dviunit()) as spt_t,
            (*font).font_id,
            0i32,
        );
        free(ustr as *mut libc::c_void);
    } else {
        if (*font).tfm_id >= 0i32 {
            text_width = tfm_string_width((*font).tfm_id, strptr, length as u32) as f64
                / (1i32 << 20i32) as f64;
            text_width *= (*font).pt_size
        }
        pdf_dev_set_string(
            (cp.x * dev_unit_dviunit()) as spt_t,
            (cp.y * dev_unit_dviunit()) as spt_t,
            strptr as *const libc::c_void,
            length as size_t,
            (text_width * dev_unit_dviunit()) as spt_t,
            (*font).font_id,
            0i32,
        );
    }
    if pdf_dev_get_font_wmode((*font).font_id) != 0 {
        pdf_dev_rmoveto(0.0f64, -text_width);
    } else {
        pdf_dev_rmoveto(text_width, 0.0f64);
    }
    graphics_mode();
    pdf_release_obj(text_str);
    0i32
}
unsafe extern "C" fn do_mpost_bind_def(
    mut ps_code: *const i8,
    mut x_user: f64,
    mut y_user: f64,
) -> i32 {
    let mut error: i32 = 0i32;
    let mut start: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    start = ps_code;
    end = start.offset(strlen(start) as isize);
    error = mp_parse_body(&mut start, end, x_user, y_user);
    error
}
unsafe extern "C" fn do_texfig_operator(mut opcode: i32, mut x_user: f64, mut y_user: f64) -> i32 {
    static mut fig_p: transform_info = transform_info::new();
    static mut in_tfig: i32 = 0i32;
    static mut xobj_id: i32 = -1i32;
    static mut count: i32 = 0i32;
    let mut values: [f64; 6] = [0.; 6];
    let mut error: i32 = 0i32;
    match opcode {
        1002 => {
            error = pop_get_numbers(values.as_mut_ptr(), 6i32);
            if error == 0 {
                let mut dvi2pts: f64 = 0.;
                let mut resname: [i8; 256] = [0; 256];
                transform_info_clear(&mut fig_p);
                dvi2pts = 1.0f64 / dev_unit_dviunit();
                fig_p.width = values[0] * dvi2pts;
                fig_p.height = values[1] * dvi2pts;
                fig_p.bbox.llx = values[2] * dvi2pts;
                fig_p.bbox.lly = -values[3] * dvi2pts;
                fig_p.bbox.urx = values[4] * dvi2pts;
                fig_p.bbox.ury = -values[5] * dvi2pts;
                fig_p.flags |= 1i32 << 0i32;
                sprintf(
                    resname.as_mut_ptr(),
                    b"__tf%d__\x00" as *const u8 as *const i8,
                    count,
                );
                xobj_id = pdf_doc_begin_grabbing(
                    resname.as_mut_ptr(),
                    fig_p.bbox.llx,
                    fig_p.bbox.ury,
                    &mut fig_p.bbox,
                );
                in_tfig = 1i32;
                count += 1
            }
        }
        1003 => {
            if in_tfig == 0 {
                panic!("endTexFig without valid startTexFig!.");
            }
            pdf_doc_end_grabbing(0 as *mut pdf_obj);
            pdf_dev_put_image(xobj_id, &mut fig_p, x_user, y_user);
            in_tfig = 0i32
        }
        _ => error = 1i32,
    }
    error
}
unsafe extern "C" fn ps_dev_CTM(M: &mut pdf_tmatrix) -> i32 {
    pdf_dev_currentmatrix(M);
    M.a *= 1000.;
    M.b *= 1000.;
    M.c *= 1000.;
    M.d *= 1000.;
    M.e *= 1000.;
    M.f *= 1000.;
    0i32
}
/*
 * Again, the only piece that needs x_user and y_user is
 * that piece dealing with texfig.
 */
unsafe extern "C" fn do_operator(mut token: *const i8, mut x_user: f64, mut y_user: f64) -> i32 {
    let mut error: i32 = 0i32;
    let mut opcode: i32 = 0i32;
    let mut values: [f64; 12] = [0.; 12];
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut matrix = pdf_tmatrix::new();
    let mut cp = pdf_coord::new();
    let mut color: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: None,
        values: [0.; 4],
    };
    opcode = get_opcode(token);
    let mut current_block_294: u64;
    match opcode {
        1 => {
            /*
             * Arithmetic operators
             */
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh6 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh6 as usize] = pdf_new_number(values[0] + values[1])
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        3 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh7 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh7 as usize] = pdf_new_number(values[0] * values[1])
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        5 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh8 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh8 as usize] = pdf_new_number(-values[0])
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        2 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh9 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh9 as usize] = pdf_new_number(values[0] - values[1])
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        4 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh10 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh10 as usize] = pdf_new_number(values[0] / values[1])
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        6 => {
            /* Round toward zero. */
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh11 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh11 as usize] = pdf_new_number(if values[0] > 0i32 as f64 {
                        values[0].floor()
                    } else {
                        values[0].ceil()
                    })
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        10 => {
            /* Stack operation */
            error = do_clear()
        }
        12 => {
            tmp = if top_stack > 0_u32 {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            pdf_release_obj(tmp);
        }
        11 => error = do_exch(),
        33 => {
            /* Path construction */
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                error = pdf_dev_moveto(values[0], values[1])
            }
        }
        34 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                error = pdf_dev_rmoveto(values[0], values[1])
            }
        }
        37 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                error = pdf_dev_lineto(values[0], values[1])
            }
        }
        38 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                error = pdf_dev_rlineto(values[0], values[1])
            }
        }
        35 => {
            error = pop_get_numbers(values.as_mut_ptr(), 6i32);
            if error == 0 {
                error = pdf_dev_curveto(
                    values[0], values[1], values[2], values[3], values[4], values[5],
                )
            }
        }
        36 => {
            error = pop_get_numbers(values.as_mut_ptr(), 6i32);
            if error == 0 {
                error = pdf_dev_rcurveto(
                    values[0], values[1], values[2], values[3], values[4], values[5],
                )
            }
        }
        32 => error = pdf_dev_closepath(),
        39 => {
            error = pop_get_numbers(values.as_mut_ptr(), 5i32);
            if error == 0 {
                error = pdf_dev_arc(values[0], values[1], values[2], values[3], values[4])
            }
        }
        40 => {
            error = pop_get_numbers(values.as_mut_ptr(), 5i32);
            if error == 0 {
                error = pdf_dev_arcn(values[0], values[1], values[2], values[3], values[4])
            }
        }
        31 => {
            pdf_dev_newpath();
        }
        42 => {
            /* fill rule not supported yet */
            pdf_dev_flushpath('S' as i32 as i8, 0i32);
        }
        41 => {
            pdf_dev_flushpath('f' as i32 as i8, 0i32);
        }
        44 => error = pdf_dev_clip(),
        45 => error = pdf_dev_eoclip(),
        50 => {
            /* Graphics state operators: */
            error = pdf_dev_gsave(); /* This does pdf_release_obj() */
            save_font();
        }
        51 => {
            error = pdf_dev_grestore();
            restore_font();
        }
        52 => {
            tmp = if top_stack > 0_u32 {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            error = cvr_array(tmp, values.as_mut_ptr(), 6i32);
            tmp = 0 as *mut pdf_obj;
            if error != 0 {
                warn!("Missing array before \"concat\".");
            } else {
                matrix.a = values[0];
                matrix.b = values[1];
                matrix.c = values[2];
                matrix.d = values[3];
                matrix.e = values[4];
                matrix.f = values[5];
                error = pdf_dev_concat(&mut matrix)
            }
        }
        53 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                match mp_cmode {
                    _ => {}
                }
                matrix.a = values[0];
                matrix.b = 0.0f64;
                matrix.c = 0.0f64;
                matrix.d = values[1];
                matrix.e = 0.0f64;
                matrix.f = 0.0f64;
                error = pdf_dev_concat(&mut matrix)
            }
        }
        55 => {
            /* Positive angle means clock-wise direction in graphicx-dvips??? */
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                values[0] = values[0] * 3.14159265358979323846f64 / 180i32 as f64;
                match mp_cmode {
                    1 | 0 => {
                        /* Really? */
                        let (s, c) = values[0].sin_cos();
                        matrix.a = c;
                        matrix.b = -s;
                        matrix.c = s;
                        matrix.d = c;
                        matrix.e = 0.;
                        matrix.f = 0.
                    }
                    _ => {
                        let (s, c) = values[0].sin_cos();
                        matrix.a = c;
                        matrix.b = s;
                        matrix.c = -s;
                        matrix.d = c;
                        matrix.e = 0.;
                        matrix.f = 0.
                    }
                }
                error = pdf_dev_concat(&mut matrix)
            }
        }
        54 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                matrix.a = 1.;
                matrix.b = 0.;
                matrix.c = 0.;
                matrix.d = 1.;
                matrix.e = values[0];
                matrix.f = values[1];
                error = pdf_dev_concat(&mut matrix)
            }
        }
        61 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                let mut pattern: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut dash: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut i: i32 = 0;
                let mut num_dashes: i32 = 0;
                let mut dash_values: [f64; 16] = [0.; 16];
                let mut offset: f64 = 0.;
                offset = values[0];
                pattern = if top_stack > 0_u32 {
                    top_stack = top_stack.wrapping_sub(1);
                    stack[top_stack as usize]
                } else {
                    0 as *mut pdf_obj
                };
                if !(!pattern.is_null() && pdf_obj_typeof(pattern) == 5i32) {
                    pdf_release_obj(pattern);
                    error = 1i32
                } else {
                    num_dashes = pdf_array_length(pattern) as i32;
                    if num_dashes > 16i32 {
                        warn!("Too many dashes...");
                        pdf_release_obj(pattern);
                        error = 1i32
                    } else {
                        i = 0i32;
                        while i < num_dashes && error == 0 {
                            dash = pdf_get_array(pattern, i);
                            if !(!dash.is_null() && pdf_obj_typeof(dash) == 2i32) {
                                error = 1i32
                            } else {
                                dash_values[i as usize] = pdf_number_value(dash)
                            }
                            i += 1
                        }
                        pdf_release_obj(pattern);
                        if error == 0 {
                            error = pdf_dev_setdash(num_dashes, dash_values.as_mut_ptr(), offset)
                        }
                    }
                }
            }
        }
        62 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                error = pdf_dev_setlinecap(values[0] as i32)
            }
        }
        63 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                error = pdf_dev_setlinejoin(values[0] as i32)
            }
        }
        60 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                error = pdf_dev_setlinewidth(values[0])
            }
        }
        64 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                error = pdf_dev_setmiterlimit(values[0])
            }
        }
        72 => {
            error = pop_get_numbers(values.as_mut_ptr(), 4i32);
            /* Not handled properly */
            if error == 0 {
                pdf_color_cmykcolor(&mut color, values[0], values[1], values[2], values[3]);
                pdf_dev_set_color(&mut color, 0_i8, 0i32);
                pdf_dev_set_color(&mut color, 0x20_i8, 0i32);
            }
        }
        70 => {
            /* Not handled properly */
            error = pop_get_numbers(values.as_mut_ptr(), 1i32); /* This does pdf_release_obj() */
            if error == 0 {
                pdf_color_graycolor(&mut color, values[0]);
                pdf_dev_set_color(&mut color, 0_i8, 0i32);
                pdf_dev_set_color(&mut color, 0x20_i8, 0i32);
            }
        }
        71 => {
            error = pop_get_numbers(values.as_mut_ptr(), 3i32);
            if error == 0 {
                pdf_color_rgbcolor(&mut color, values[0], values[1], values[2]);
                pdf_dev_set_color(&mut color, 0_i8, 0i32);
                pdf_dev_set_color(&mut color, 0x20_i8, 0i32);
            }
        }
        49 => {}
        80 => {
            error = pdf_dev_currentpoint(&mut cp);
            if error == 0 {
                if top_stack < 1024_u32 {
                    let fresh12 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh12 as usize] = pdf_new_number(cp.x);
                    if top_stack < 1024_u32 {
                        let fresh13 = top_stack;
                        top_stack = top_stack.wrapping_add(1);
                        stack[fresh13 as usize] = pdf_new_number(cp.y)
                    } else {
                        warn!("PS stack overflow including MetaPost file or inline PS code");
                        error = 1i32
                    }
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            }
        }
        82 => {
            let mut has_matrix: i32 = 0i32;
            tmp = if top_stack > 0_u32 {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            if !tmp.is_null() && pdf_obj_typeof(tmp) == 5i32 {
                error = cvr_array(tmp, values.as_mut_ptr(), 6i32);
                tmp = 0 as *mut pdf_obj;
                if error != 0 {
                    current_block_294 = 9125367800366194000;
                } else {
                    matrix.a = values[0];
                    matrix.b = values[1];
                    matrix.c = values[2];
                    matrix.d = values[3];
                    matrix.e = values[4];
                    matrix.f = values[5];
                    tmp = if top_stack > 0_u32 {
                        top_stack = top_stack.wrapping_sub(1);
                        stack[top_stack as usize]
                    } else {
                        0 as *mut pdf_obj
                    };
                    has_matrix = 1i32;
                    current_block_294 = 15375688482130298215;
                }
            } else {
                current_block_294 = 15375688482130298215;
            }
            match current_block_294 {
                9125367800366194000 => {}
                _ => {
                    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
                        error = 1i32
                    } else {
                        cp.y = pdf_number_value(tmp);
                        pdf_release_obj(tmp);
                        tmp = if top_stack > 0_u32 {
                            top_stack = top_stack.wrapping_sub(1);
                            stack[top_stack as usize]
                        } else {
                            0 as *mut pdf_obj
                        };
                        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
                            error = 1i32
                        } else {
                            cp.x = pdf_number_value(tmp);
                            pdf_release_obj(tmp);
                            if has_matrix == 0 {
                                ps_dev_CTM(&mut matrix);
                                /* Here, we need real PostScript CTM */
                            } /* This does pdf_release_obj() */
                            pdf_dev_dtransform(&mut cp, Some(&mut matrix));
                            if top_stack < 1024_u32 {
                                let fresh14 = top_stack;
                                top_stack = top_stack.wrapping_add(1);
                                stack[fresh14 as usize] = pdf_new_number(cp.x);
                                if top_stack < 1024_u32 {
                                    let fresh15 = top_stack;
                                    top_stack = top_stack.wrapping_add(1);
                                    stack[fresh15 as usize] = pdf_new_number(cp.y)
                                } else {
                                    warn!("PS stack overflow including MetaPost file or inline PS code");
                                    error = 1i32
                                }
                            } else {
                                warn!(
                                    "PS stack overflow including MetaPost file or inline PS code"
                                );
                                error = 1i32
                            }
                        }
                    }
                }
            }
        }
        81 => {
            let mut has_matrix_0: i32 = 0i32;
            tmp = if top_stack > 0_u32 {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            if !tmp.is_null() && pdf_obj_typeof(tmp) == 5i32 {
                error = cvr_array(tmp, values.as_mut_ptr(), 6i32);
                tmp = 0 as *mut pdf_obj;
                if error != 0 {
                    current_block_294 = 9125367800366194000;
                } else {
                    matrix.a = values[0];
                    matrix.b = values[1];
                    matrix.c = values[2];
                    matrix.d = values[3];
                    matrix.e = values[4];
                    matrix.f = values[5];
                    tmp = if top_stack > 0_u32 {
                        top_stack = top_stack.wrapping_sub(1);
                        stack[top_stack as usize]
                    } else {
                        0 as *mut pdf_obj
                    };
                    has_matrix_0 = 1i32;
                    current_block_294 = 9910899284672532069;
                }
            } else {
                current_block_294 = 9910899284672532069;
            }
            match current_block_294 {
                9125367800366194000 => {}
                _ => {
                    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
                        error = 1i32
                    } else {
                        cp.y = pdf_number_value(tmp);
                        pdf_release_obj(tmp);
                        tmp = if top_stack > 0_u32 {
                            top_stack = top_stack.wrapping_sub(1);
                            stack[top_stack as usize]
                        } else {
                            0 as *mut pdf_obj
                        };
                        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
                            error = 1i32
                        } else {
                            cp.x = pdf_number_value(tmp);
                            pdf_release_obj(tmp);
                            if has_matrix_0 == 0 {
                                ps_dev_CTM(&mut matrix);
                                /* Here, we need real PostScript CTM */
                            }
                            pdf_dev_idtransform(&mut cp, Some(&matrix));
                            if top_stack < 1024_u32 {
                                let fresh16 = top_stack;
                                top_stack = top_stack.wrapping_add(1);
                                stack[fresh16 as usize] = pdf_new_number(cp.x);
                                if top_stack < 1024_u32 {
                                    let fresh17 = top_stack;
                                    top_stack = top_stack.wrapping_add(1);
                                    stack[fresh17 as usize] = pdf_new_number(cp.y)
                                } else {
                                    warn!("PS stack overflow including MetaPost file or inline PS code");
                                    error = 1i32
                                }
                            } else {
                                warn!(
                                    "PS stack overflow including MetaPost file or inline PS code"
                                );
                                error = 1i32
                            }
                        }
                    }
                }
            }
        }
        201 => error = do_findfont(),
        202 => error = do_scalefont(),
        203 => error = do_setfont(),
        204 => error = do_currentfont(),
        43 => error = do_show(),
        210 => error = 1i32,
        1001 => {
            /* Extensions */
            error = do_mpost_bind_def(
                b"exch findfont exch scalefont setfont show\x00" as *const u8 as *const i8,
                x_user,
                y_user,
            )
        }
        1002 | 1003 => error = do_texfig_operator(opcode, x_user, y_user),
        1004 => {
            error = do_mpost_bind_def(
                b"0 dtransform exch truncate exch idtransform pop setlinewidth\x00" as *const u8
                    as *const i8,
                x_user,
                y_user,
            )
        }
        1005 => {
            error = do_mpost_bind_def(
                b"0 exch dtransform truncate idtransform setlinewidth pop\x00" as *const u8
                    as *const i8,
                x_user,
                y_user,
            )
        }
        1006 => {
            error = do_mpost_bind_def(
                b"[] 0 setdash\x00" as *const u8 as *const i8,
                x_user,
                y_user,
            )
        }
        1007 => {
            error = do_mpost_bind_def(
                b"gsave fill grestore\x00" as *const u8 as *const i8,
                x_user,
                y_user,
            )
        }
        999 => {
            tmp = if top_stack > 0_u32 {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            tmp = if top_stack > 0_u32 {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            }
        }
        _ => {
            if is_fontname(token) {
                if top_stack < 1024_u32 {
                    let fresh18 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh18 as usize] = pdf_new_name(token)
                } else {
                    warn!("PS stack overflow including MetaPost file or inline PS code");
                    error = 1i32
                }
            } else {
                dpx_warning(b"Unknown token \"%s\"\x00" as *const u8 as *const i8, token);
                error = 1i32
            }
        }
    }
    error
}
/*
 * In PDF, current path is not a part of graphics state parameter.
 * Hence, current path is not saved by the "q" operator  and is not
 * recovered by the "Q" operator. This means that the following PS
 * code
 *
 *   <path construction> gsave <path painting> grestore ...
 *
 * can't be translated to PDF code
 *
 *   <path construction> q <path painting> Q ...
 *
 * . Only clipping path (which is graphics state parameter in PDF
 * too) is treated in the same way. So, we write clipping path
 * immediately and forget about it but remember current path.
 */
/*
 * The only sections that need to know x_user and y _user are those
 * dealing with texfig.
 */
unsafe extern "C" fn mp_parse_body(
    mut start: *mut *const i8,
    mut end: *const i8,
    mut x_user: f64,
    mut y_user: f64,
) -> i32 {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: i32 = 0i32;
    skip_white(start, end);
    while *start < end && error == 0 {
        if libc::isdigit(**start as _) != 0
            || *start < end.offset(-1)
                && (**start as i32 == '+' as i32
                    || **start as i32 == '-' as i32
                    || **start as i32 == '.' as i32)
        {
            let mut value: f64 = 0.;
            let mut next: *mut i8 = 0 as *mut i8;
            value = strtod(*start, &mut next);
            if next < end as *mut i8
                && strchr(b"<([{/%\x00" as *const u8 as *const i8, *next as i32).is_null()
                && libc::isspace(*next as _) == 0
            {
                warn!("Unkown PostScript operator.");
                dump(*start, next);
                error = 1i32
            } else if top_stack < 1024_u32 {
                let fresh19 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh19 as usize] = pdf_new_number(value);
                *start = next
            } else {
                warn!("PS stack overflow including MetaPost file or inline PS code");
                error = 1i32;
                break;
            }
        /*
         * PDF parser can't handle PS operator inside arrays.
         * This shouldn't use parse_pdf_array().
         */
        } else if **start as i32 == '[' as i32 && {
            obj = parse_pdf_array(start, end, 0 as *mut pdf_file);
            !obj.is_null()
        } {
            if top_stack < 1024_u32 {
                let fresh20 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh20 as usize] = obj
            } else {
                warn!("PS stack overflow including MetaPost file or inline PS code");
                error = 1i32;
                break;
            }
        /* This cannot handle ASCII85 string. */
        } else if *start < end.offset(-1)
            && (**start as i32 == '<' as i32 && *(*start).offset(1) as i32 == '<' as i32)
            && {
                obj = parse_pdf_dict(start, end, 0 as *mut pdf_file);
                !obj.is_null()
            }
        {
            if top_stack < 1024_u32 {
                let fresh21 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh21 as usize] = obj
            } else {
                warn!("PS stack overflow including MetaPost file or inline PS code");
                error = 1i32;
                break;
            }
        } else if (**start as i32 == '(' as i32 || **start as i32 == '<' as i32) && {
            obj = parse_pdf_string(start, end);
            !obj.is_null()
        } {
            if top_stack < 1024_u32 {
                let fresh22 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh22 as usize] = obj
            } else {
                warn!("PS stack overflow including MetaPost file or inline PS code");
                error = 1i32;
                break;
            }
        } else if **start as i32 == '/' as i32 && {
            obj = parse_pdf_name(start, end);
            !obj.is_null()
        } {
            if top_stack < 1024_u32 {
                let fresh23 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh23 as usize] = obj
            } else {
                warn!("PS stack overflow including MetaPost file or inline PS code");
                error = 1i32;
                break;
            }
        } else {
            token = parse_ident(start, end);
            if token.is_null() {
                error = 1i32
            } else {
                error = do_operator(token, x_user, y_user);
                free(token as *mut libc::c_void);
            }
        }
        skip_white(start, end);
    }
    error
}
#[no_mangle]
pub unsafe extern "C" fn mps_eop_cleanup() {
    clear_fonts();
    do_clear();
}
#[no_mangle]
pub unsafe extern "C" fn mps_stack_depth() -> i32 {
    top_stack as i32
}
#[no_mangle]
pub unsafe extern "C" fn mps_exec_inline(
    mut p: *mut *const i8,
    mut endptr: *const i8,
    mut x_user: f64,
    mut y_user: f64,
) -> i32 {
    let mut error: i32 = 0;
    let mut dirmode: i32 = 0;
    let mut autorotate: i32 = 0;
    /* Compatibility for dvipsk. */
    dirmode = pdf_dev_get_dirmode();
    if dirmode != 0 {
        mp_cmode = 2i32
    } else {
        mp_cmode = 1i32
    }
    autorotate = pdf_dev_get_param(1i32);
    pdf_dev_set_param(1i32, 0i32);
    //pdf_color_push(); /* ... */
    /* Comment in dvipdfm:
     * Remember that x_user and y_user are off by 0.02 %
     */
    pdf_dev_moveto(x_user, y_user);
    error = mp_parse_body(p, endptr, x_user, y_user);
    //pdf_color_pop(); /* ... */
    pdf_dev_set_param(1i32, autorotate);
    pdf_dev_set_dirmode(dirmode);
    error
}
#[no_mangle]
pub unsafe extern "C" fn mps_do_page(mut image_file: *mut FILE) -> i32 {
    let mut error: i32 = 0i32; /* scale, xorig, yorig */
    let mut bbox = pdf_rect::new();
    let mut buffer: *mut i8 = 0 as *mut i8;
    let mut start: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut size: i32 = 0;
    let mut dir_mode: i32 = 0;
    rewind(image_file);
    size = file_size(image_file);
    if size == 0i32 {
        warn!("Can\'t read any byte in the MPS file.");
        return -1i32;
    }
    buffer =
        new(((size + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    fread(
        buffer as *mut libc::c_void,
        ::std::mem::size_of::<i8>() as u64,
        size as u64,
        image_file,
    );
    *buffer.offset(size as isize) = 0_i8;
    start = buffer;
    end = buffer.offset(size as isize);
    error = mps_scan_bbox(&mut start, end, &mut bbox);
    if error != 0 {
        warn!("Error occured while scanning MetaPost file headers: Could not find BoundingBox.");
        free(buffer as *mut libc::c_void);
        return -1i32;
    }
    mp_cmode = 0i32;
    pdf_doc_begin_page(1.0f64, -Xorigin, -Yorigin);
    pdf_doc_set_mediabox(pdf_doc_current_page_number() as u32, &mut bbox);
    dir_mode = pdf_dev_get_dirmode();
    pdf_dev_set_param(1i32, 0i32);
    skip_prolog(&mut start, end);
    error = mp_parse_body(&mut start, end, 0.0f64, 0.0f64);
    if error != 0 {
        warn!("Errors occured while interpreting MetaPost file.");
    }
    pdf_dev_set_param(1i32, 1i32);
    pdf_dev_set_dirmode(dir_mode);
    pdf_doc_end_page();
    free(buffer as *mut libc::c_void);
    /*
     * The reason why we don't return XObject itself is
     * PDF inclusion may not be made so.
     */
    if error != 0 {
        -1i32
    } else {
        0i32
    }
}
