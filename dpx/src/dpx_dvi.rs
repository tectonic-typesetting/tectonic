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

use super::dpx_sfnt::{
    dfont_open, sfnt_close, sfnt_find_table_pos, sfnt_locate_table, sfnt_open,
    sfnt_read_table_directory,
};
use crate::mfree;
use crate::streq_ptr;
use crate::warn;

use super::dpx_cff::cff_close;
use super::dpx_cff_dict::{cff_dict_get, cff_dict_known};
use super::dpx_dpxfile::{
    dpx_open_dfont_file, dpx_open_opentype_file, dpx_open_truetype_file, dpx_open_type1_file,
};
use super::dpx_numbers::tt_skip_bytes;
use super::dpx_numbers::{
    tt_get_positive_quad, tt_get_signed_quad, tt_get_unsigned_byte, tt_get_unsigned_num,
    tt_get_unsigned_pair, tt_get_unsigned_quad,
};
use super::dpx_pdfcolor::{pdf_color_pop, pdf_color_push, pdf_color_rgbcolor};
use super::dpx_pdfdev::{pdf_dev_begin_actualtext, pdf_dev_end_actualtext, pdf_dev_set_rect};
use super::dpx_pdfdev::{
    pdf_dev_locate_font, pdf_dev_set_dirmode, pdf_dev_set_rule, pdf_dev_set_string,
};
use super::dpx_pdfdoc::pdf_doc_expand_box;
use super::dpx_pdfdoc::{pdf_doc_begin_page, pdf_doc_break_annot, pdf_doc_end_page};
use super::dpx_pdfparse::{parse_pdf_number, parse_pdf_string};
use super::dpx_tt_table::{
    tt_read_head_table, tt_read_hhea_table, tt_read_longMetrics, tt_read_maxp_table,
    tt_read_vhea_table,
};
use crate::dpx_pdfobj::{
    pdf_number_value, pdf_obj, pdf_obj_typeof, pdf_release_obj, pdf_string_value, PdfObjType,
};
use crate::{
    ttstub_input_close, ttstub_input_get_size, ttstub_input_getc, ttstub_input_open,
    ttstub_input_read, ttstub_input_seek, ttstub_input_ungetc,
};
use libc::free;
extern "C" {
    /* Here is the complete list of PDF object types */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> i64;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn sqxfw(sq: i32, fw: fixword) -> i32;
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
    /* Set rect to rectangle in device space.
     * Unit conversion spt_t to bp and transformation applied within it.
     */
    /* Text is normal and line art is not normal in dvipdfmx. So we don't have
     * begin_text (BT in PDF) and end_text (ET), but instead we have graphics_mode()
     * to terminate text section. pdf_dev_flushpath() and others call this.
     */
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    static mut paper_width: f64;
    #[no_mangle]
    static mut paper_height: f64;
    #[no_mangle]
    static mut landscape_mode: i32;
    #[no_mangle]
    static mut is_xdv: i32;
    #[no_mangle]
    fn pdf_lookup_fontmap_record(kp: *const i8) -> *mut fontmap_rec;
    #[no_mangle]
    fn pdf_insert_native_fontmap_record(
        filename: *const i8,
        index: u32,
        layout_dir: i32,
        extend: i32,
        slant: i32,
        embolden: i32,
    ) -> *mut fontmap_rec;
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    /* Please remove this */
    #[no_mangle]
    fn dump(start: *const i8, end: *const i8);
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    /* This should not use pdf_. */
    #[no_mangle]
    fn spc_set_verbose(level: i32);
    #[no_mangle]
    fn spc_exec_at_begin_page() -> i32;
    #[no_mangle]
    fn spc_exec_at_end_page() -> i32;
    #[no_mangle]
    fn spc_exec_special(p: *const i8, size: i32, x_user: f64, y_user: f64, mag: f64) -> i32;
    #[no_mangle]
    fn subfont_set_verbose(level: i32);
    #[no_mangle]
    fn lookup_sfd_record(rec_id: i32, code: u8) -> u16;
    #[no_mangle]
    fn sfd_load_record(sfd_name: *const i8, subfont_id: *const i8) -> i32;
    #[no_mangle]
    fn t1char_get_metrics(
        src: *mut card8,
        srclen: i32,
        subrs: *mut cff_index,
        ginfo: *mut t1_ginfo,
    ) -> i32;
    #[no_mangle]
    fn t1_load_font(enc_vec: *mut *mut i8, mode: i32, handle: rust_input_handle_t)
        -> *mut cff_font;
    #[no_mangle]
    fn tfm_set_verbose(level: i32);
    #[no_mangle]
    fn tfm_open(tex_name: *const i8, must_exist: i32) -> i32;
    #[no_mangle]
    fn tfm_close_all();
    #[no_mangle]
    fn tfm_get_fw_width(font_id: i32, ch: i32) -> fixword;
    #[no_mangle]
    fn tfm_get_fw_height(font_id: i32, ch: i32) -> fixword;
    #[no_mangle]
    fn tfm_get_fw_depth(font_id: i32, ch: i32) -> fixword;
    /* TTC (TrueType Collection) */
    #[no_mangle]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: i32) -> u32;
    #[no_mangle]
    fn vf_set_verbose(level: i32);
    #[no_mangle]
    fn vf_locate_font(tex_name: *const i8, ptsize: spt_t) -> i32;
    #[no_mangle]
    fn vf_set_char(ch: i32, vf_font: i32);
    #[no_mangle]
    fn vf_close_all_fonts();
    #[no_mangle]
    fn parse_float_decimal(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
}
use crate::*;

use crate::TTInputFormat;

pub type rust_input_handle_t = *mut libc::c_void;
pub type fixword = i32;
/* quasi-hack to get the primary input */

pub use super::dpx_pdfcolor::pdf_color;

pub type spt_t = i32;
use super::dpx_pdfdev::pdf_rect;
/*
 * The section below this line deals with the actual processing of the
 * dvi file.
 *
 * The dvi file processor state is contained in the following variables:
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_registers {
    pub h: i32,
    pub v: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub d: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_def {
    pub tex_id: u32,
    pub point_size: spt_t,
    pub design_size: spt_t,
    pub font_name: *mut i8,
    pub font_id: i32,
    pub used: i32,
    pub native: i32,
    pub rgba_color: u32,
    pub face_index: u32,
    pub layout_dir: i32,
    pub extend: i32,
    pub slant: i32,
    pub embolden: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loaded_font {
    pub type_0: i32,
    pub font_id: i32,
    pub subfont_id: i32,
    pub tfm_id: i32,
    pub size: spt_t,
    pub source: i32,
    pub rgba_color: u32,
    pub hvmt: *mut tt_longMetrics,
    pub ascent: i32,
    pub descent: i32,
    pub unitsPerEm: u32,
    pub cffont: *mut cff_font,
    pub numGlyphs: u32,
    pub layout_dir: i32,
    pub extend: f32,
    pub slant: f32,
    pub embolden: f32,
}

use super::dpx_cff::cff_font;

use super::dpx_cff::cff_index;
pub type card8 = u8;
pub type l_offset = u32;
pub type c_offsize = u8;
pub type card16 = u16;

pub type s_SID = u16;
use super::dpx_fontmap::fontmap_rec;
use super::dpx_tt_table::tt_longMetrics;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_header {
    pub unit_num: u32,
    pub unit_den: u32,
    pub mag: u32,
    pub media_width: u32,
    pub media_height: u32,
    pub stackdepth: u32,
    pub comment: [i8; 257],
}
/* skimming through reflected segment measuring its width */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_lr {
    pub state: i32,
    pub font: i32,
    pub buf_index: u32,
}

use super::dpx_t1_char::t1_ginfo;

use super::dpx_sfnt::sfnt;
use super::dpx_tt_table::{tt_head_table, tt_hhea_table, tt_maxp_table};
/* 16.16-bit signed fixed-point number */
pub type FWord = i16;
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */
pub type Fixed = u32;

pub type uFWord = u16;

use super::dpx_tt_table::tt_vhea_table;
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
/* UTF-32 over U+FFFF -> UTF-16 surrogate pair */
/* Interal Variables */
static mut dvi_handle: rust_input_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut linear: i8 = 0_i8;
/* set to 1 for strict linear processing of the input */
static mut page_loc: *mut u32 = 0 as *const u32 as *mut u32;
static mut num_pages: u32 = 0_u32;
static mut dvi_file_size: u32 = 0_u32;
static mut dvi_info: dvi_header = {
    let mut init = dvi_header {
        unit_num: 25400000_u32,
        unit_den: 473628672_u32,
        mag: 1000_u32,
        media_width: 0_u32,
        media_height: 0_u32,
        stackdepth: 0_u32,
        comment: [
            '\u{0}' as i32 as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    };
    init
};
static mut dev_origin_x: f64 = 72.0f64;
static mut dev_origin_y: f64 = 770.0f64;
#[no_mangle]
pub unsafe extern "C" fn get_origin(mut x: i32) -> f64 {
    if x != 0 {
        dev_origin_x
    } else {
        dev_origin_y
    }
}
static mut lr_state: dvi_lr = dvi_lr {
    state: 0,
    font: 0,
    buf_index: 0,
};
/* state at start of current skimming  */
static mut lr_mode: i32 = 0;
/* current direction or skimming depth */
static mut lr_width: u32 = 0;
/* total width of reflected segment    */
static mut lr_width_stack: [u32; 256] = [0; 256];
static mut lr_width_stack_depth: u32 = 0_u32;
static mut loaded_fonts: *mut loaded_font = 0 as *const loaded_font as *mut loaded_font;
static mut num_loaded_fonts: u32 = 0_u32;
static mut max_loaded_fonts: u32 = 0_u32;
unsafe extern "C" fn need_more_fonts(mut n: u32) {
    if num_loaded_fonts.wrapping_add(n) > max_loaded_fonts {
        max_loaded_fonts = max_loaded_fonts.wrapping_add(16u32);
        loaded_fonts = renew(
            loaded_fonts as *mut libc::c_void,
            (max_loaded_fonts as u64).wrapping_mul(::std::mem::size_of::<loaded_font>() as u64)
                as u32,
        ) as *mut loaded_font
    };
}
static mut def_fonts: *mut font_def = 0 as *const font_def as *mut font_def;
static mut num_def_fonts: u32 = 0_u32;
static mut max_def_fonts: u32 = 0_u32;
static mut compute_boxes: i32 = 0i32;
static mut link_annot: i32 = 1i32;
static mut verbose: i32 = 0i32;
/* 64K should be plenty for most pages */
static mut dvi_page_buffer: *mut u8 = 0 as *const u8 as *mut u8;
static mut dvi_page_buf_size: u32 = 0;
static mut dvi_page_buf_index: u32 = 0;
/* functions to read numbers from the dvi file and store them in dvi_page_buffer */
unsafe extern "C" fn get_and_buffer_unsigned_byte(mut handle: rust_input_handle_t) -> i32 {
    let mut ch: i32 = 0;
    ch = ttstub_input_getc(handle);
    if ch < 0i32 {
        panic!("File ended prematurely\n");
    }
    if dvi_page_buf_index >= dvi_page_buf_size {
        dvi_page_buf_size = dvi_page_buf_size.wrapping_add(0x10000u32);
        dvi_page_buffer = renew(
            dvi_page_buffer as *mut libc::c_void,
            (dvi_page_buf_size as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
        ) as *mut u8
    }
    let fresh0 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    *dvi_page_buffer.offset(fresh0 as isize) = ch as u8;
    ch
}
unsafe extern "C" fn get_and_buffer_unsigned_pair(mut handle: rust_input_handle_t) -> u32 {
    let mut pair: u32 = get_and_buffer_unsigned_byte(handle) as u32;
    pair = pair << 8i32 | get_and_buffer_unsigned_byte(handle) as u32;
    pair
}
unsafe extern "C" fn get_and_buffer_bytes(mut handle: rust_input_handle_t, mut count: u32) {
    if dvi_page_buf_index.wrapping_add(count) >= dvi_page_buf_size {
        dvi_page_buf_size = dvi_page_buf_index
            .wrapping_add(count)
            .wrapping_add(0x10000u32);
        dvi_page_buffer = renew(
            dvi_page_buffer as *mut libc::c_void,
            (dvi_page_buf_size as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
        ) as *mut u8
    }
    if ttstub_input_read(
        handle,
        (dvi_page_buffer as *mut i8).offset(dvi_page_buf_index as isize),
        count as size_t,
    ) != count as i64
    {
        panic!("File ended prematurely\n");
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(count);
}
/* functions to fetch values from dvi_page_buffer */
unsafe extern "C" fn get_buffered_unsigned_byte() -> i32 {
    let fresh1 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    *dvi_page_buffer.offset(fresh1 as isize) as i32
}
unsafe extern "C" fn get_buffered_unsigned_pair() -> u32 {
    let fresh2 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut pair: u32 = *dvi_page_buffer.offset(fresh2 as isize) as u32;
    let fresh3 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    pair = pair << 8i32 | *dvi_page_buffer.offset(fresh3 as isize) as u32;
    pair
}
unsafe extern "C" fn get_buffered_signed_quad() -> i32 {
    let mut i: u32 = 0;
    let fresh4 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut quad: i32 = *dvi_page_buffer.offset(fresh4 as isize) as i32;
    /* Check sign on first byte before reading others */
    if quad >= 0x80i32 {
        quad -= 0x100i32
    }
    i = 0_u32;
    while i < 3_u32 {
        let fresh5 = dvi_page_buf_index;
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
        quad = quad << 8i32 | *dvi_page_buffer.offset(fresh5 as isize) as i32;
        i = i.wrapping_add(1)
    }
    quad
}
unsafe extern "C" fn get_buffered_signed_num(mut num: u8) -> i32 {
    let fresh6 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut quad: i32 = *dvi_page_buffer.offset(fresh6 as isize) as i32;
    if quad > 0x7fi32 {
        quad -= 0x100i32
    }
    let mut current_block_4: u64;
    match num as i32 {
        3 => {
            let fresh7 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh7 as isize) as i32;
            current_block_4 = 16810064376154758607;
        }
        2 => {
            current_block_4 = 16810064376154758607;
        }
        1 => {
            current_block_4 = 7573734397012025053;
        }
        _ => {
            current_block_4 = 7815301370352969686;
        }
    }
    match current_block_4 {
        16810064376154758607 => {
            let fresh8 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh8 as isize) as i32;
            current_block_4 = 7573734397012025053;
        }
        _ => {}
    }
    match current_block_4 {
        7573734397012025053 => {
            let fresh9 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh9 as isize) as i32
        }
        _ => {}
    }
    quad
}
unsafe extern "C" fn get_buffered_unsigned_num(mut num: u8) -> i32 {
    let fresh10 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut quad: i32 = *dvi_page_buffer.offset(fresh10 as isize) as i32;
    let mut current_block_4: u64;
    match num as i32 {
        3 => {
            if quad > 0x7fi32 {
                quad -= 0x100i32
            }
            let fresh11 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh11 as isize) as i32;
            current_block_4 = 4809506329084940920;
        }
        2 => {
            current_block_4 = 4809506329084940920;
        }
        1 => {
            current_block_4 = 10786445149178684808;
        }
        _ => {
            current_block_4 = 7815301370352969686;
        }
    }
    match current_block_4 {
        4809506329084940920 => {
            let fresh12 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh12 as isize) as i32;
            current_block_4 = 10786445149178684808;
        }
        _ => {}
    }
    match current_block_4 {
        10786445149178684808 => {
            let fresh13 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh13 as isize) as i32
        }
        _ => {}
    }
    quad
}
#[no_mangle]
pub unsafe extern "C" fn dvi_set_verbose(mut level: i32) {
    verbose = level;
    subfont_set_verbose(level);
    tfm_set_verbose(level);
    vf_set_verbose(level);
    spc_set_verbose(level);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_npages() -> u32 {
    num_pages
}
static mut invalid_signature: [i8; 53] = [
    83, 111, 109, 101, 116, 104, 105, 110, 103, 32, 105, 115, 32, 119, 114, 111, 110, 103, 46, 32,
    65, 114, 101, 32, 121, 111, 117, 32, 115, 117, 114, 101, 32, 116, 104, 105, 115, 32, 105, 115,
    32, 97, 32, 68, 86, 73, 32, 102, 105, 108, 101, 63, 0,
];
static mut pre_id_byte: i32 = 0;
static mut post_id_byte: i32 = 0;
static mut is_ptex: i32 = 0i32;
static mut has_ptex: i32 = 0i32;
unsafe extern "C" fn check_id_bytes() {
    if pre_id_byte != post_id_byte && (pre_id_byte != 2i32 || post_id_byte != 3i32) {
        panic!(
            "Inconsistent DVI id_bytes {} (pre) and {} (post)",
            pre_id_byte, post_id_byte
        );
    };
}
unsafe extern "C" fn need_XeTeX(mut c: i32) {
    if is_xdv == 0 {
        panic!("DVI opcode {} only valid for XeTeX", c,);
    };
}
unsafe extern "C" fn need_pTeX(mut c: i32) {
    if is_ptex == 0 {
        panic!("DVI opcode {} only valid for Ascii pTeX", c,);
    }
    has_ptex = 1i32;
}
unsafe extern "C" fn find_post() -> i32 {
    let mut dvi_size: libc::off_t = 0;
    let mut current: i32 = 0;
    let mut ch: i32 = 0;
    dvi_size = ttstub_input_get_size(dvi_handle) as libc::off_t;
    if dvi_size > 0x7fffffffi32 as libc::off_t {
        panic!("DVI file size exceeds 31-bit");
    }
    dvi_file_size = dvi_size as u32;
    ttstub_input_seek(dvi_handle, 0i32 as ssize_t, 2i32);
    current = dvi_size as i32;
    loop
    /* Scan backwards through PADDING */
    {
        current -= 1;
        ttstub_input_seek(dvi_handle, current as ssize_t, 0i32);
        ch = ttstub_input_getc(dvi_handle);
        if !(ch == 223i32 && current > 0i32) {
            break;
        }
    }
    /* file_position now points to last non padding character or
     * beginning of file */
    if dvi_file_size.wrapping_sub(current as u32) < 4_u32
        || current == 0i32
        || !(ch == 2i32 || ch == 3i32 || ch == 7i32 || ch == 6i32)
    {
        info!("DVI ID = {}\n", ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    post_id_byte = ch;
    is_xdv = (ch == 7i32 || ch == 6i32) as i32;
    is_ptex = (ch == 3i32) as i32;
    /* Make sure post_post is really there */
    current = current - 5i32;
    ttstub_input_seek(dvi_handle, current as ssize_t, 0i32);
    ch = ttstub_input_getc(dvi_handle);
    if ch != 249i32 {
        info!("Found {} where post_post opcode should be\n", ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    current = tt_get_signed_quad(dvi_handle);
    ttstub_input_seek(dvi_handle, current as ssize_t, 0i32);
    ch = ttstub_input_getc(dvi_handle);
    if ch != 248i32 {
        info!("Found {} where post_post opcode should be\n", ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    /* Finally check the ID byte in the preamble */
    /* An Ascii pTeX DVI file has id_byte DVI_ID in the preamble but DVIV_ID in the postamble. */
    ttstub_input_seek(dvi_handle, 0i32 as ssize_t, 0i32);
    ch = tt_get_unsigned_byte(dvi_handle) as i32;
    if ch != 247i32 {
        info!("Found {} where PRE was expected\n", ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    ch = tt_get_unsigned_byte(dvi_handle) as i32;
    if !(ch == 2i32 || ch == 7i32 || ch == 6i32) {
        info!("DVI ID = {}\n", ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    pre_id_byte = ch;
    check_id_bytes();
    current
}
unsafe extern "C" fn get_page_info(mut post_location: i32) {
    let mut i: i32 = 0;
    ttstub_input_seek(dvi_handle, (post_location + 27i32) as ssize_t, 0i32);
    num_pages = tt_get_unsigned_pair(dvi_handle) as u32;
    if num_pages == 0_u32 {
        panic!("Page count is 0!");
    }
    if verbose > 2i32 {
        info!("Page count:\t {:4}\n", num_pages,);
    }
    page_loc = new((num_pages as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64) as u32)
        as *mut u32;
    ttstub_input_seek(dvi_handle, (post_location + 1i32) as ssize_t, 0i32);
    *page_loc.offset(num_pages.wrapping_sub(1_u32) as isize) = tt_get_unsigned_quad(dvi_handle);
    if (*page_loc.offset(num_pages.wrapping_sub(1_u32) as isize)).wrapping_add(41_u32)
        > dvi_file_size
    {
        _tt_abort(invalid_signature.as_ptr());
    }
    i = num_pages.wrapping_sub(2_u32) as i32;
    while i >= 0i32 {
        ttstub_input_seek(
            dvi_handle,
            (*page_loc.offset((i + 1i32) as isize)).wrapping_add(41_u32) as ssize_t,
            0i32,
        );
        *page_loc.offset(i as isize) = tt_get_unsigned_quad(dvi_handle);
        if (*page_loc.offset(num_pages.wrapping_sub(1_u32) as isize)).wrapping_add(41_u32)
            > dvi_file_size
        {
            _tt_abort(invalid_signature.as_ptr());
        }
        i -= 1
    }
}
/* Following are computed "constants" used for unit conversion */
static mut dvi2pts: f64 = 1.52018f64;
static mut total_mag: f64 = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dvi_tell_mag() -> f64 {
    return total_mag; /* unused */
}
unsafe extern "C" fn do_scales(mut mag: f64) {
    total_mag = dvi_info.mag as f64 / 1000.0f64 * mag; /* 1.0 */
    dvi2pts = dvi_info.unit_num as f64 / dvi_info.unit_den as f64; /* font name length */
    dvi2pts *= 72.0f64 / 254000.0f64; /* hard-code as 10pt for now, not used anyway */
}
unsafe extern "C" fn get_dvi_info(mut post_location: i32) {
    ttstub_input_seek(dvi_handle, (post_location + 5i32) as ssize_t, 0i32); /* direction */
    dvi_info.unit_num = tt_get_unsigned_quad(dvi_handle);
    dvi_info.unit_den = tt_get_unsigned_quad(dvi_handle);
    dvi_info.mag = tt_get_unsigned_quad(dvi_handle);
    dvi_info.media_height = tt_get_unsigned_quad(dvi_handle);
    dvi_info.media_width = tt_get_unsigned_quad(dvi_handle);
    dvi_info.stackdepth = tt_get_unsigned_pair(dvi_handle) as u32;
    if dvi_info.stackdepth > 256u32 {
        warn!("DVI need stack depth of {},", dvi_info.stackdepth);
        warn!("but DVI_STACK_DEPTH_MAX is {}.", 256u32);
        panic!("Capacity exceeded.");
    }
    if verbose > 2i32 {
        info!("DVI File Info\n");
        info!("Unit: {} / {}\n", dvi_info.unit_num, dvi_info.unit_den);
        info!("Magnification: {}\n", dvi_info.mag);
        info!("Media Height: {}\n", dvi_info.media_height);
        info!("Media Width: {}\n", dvi_info.media_width);
        info!("Stack Depth: {}\n", dvi_info.stackdepth);
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_comment() -> *const i8 {
    dvi_info.comment.as_mut_ptr()
}
unsafe extern "C" fn read_font_record(mut tex_id: u32) {
    let mut dir_length: i32 = 0;
    let mut name_length: i32 = 0;
    let mut point_size: u32 = 0;
    let mut design_size: u32 = 0;
    let mut directory: *mut i8 = 0 as *mut i8;
    let mut font_name: *mut i8 = 0 as *mut i8;
    if num_def_fonts >= max_def_fonts {
        max_def_fonts = max_def_fonts.wrapping_add(16u32);
        def_fonts = renew(
            def_fonts as *mut libc::c_void,
            (max_def_fonts as u64).wrapping_mul(::std::mem::size_of::<font_def>() as u64) as u32,
        ) as *mut font_def
    }
    tt_get_unsigned_quad(dvi_handle);
    point_size = tt_get_positive_quad(
        dvi_handle,
        b"DVI\x00" as *const u8 as *const i8,
        b"point_size\x00" as *const u8 as *const i8,
    );
    design_size = tt_get_positive_quad(
        dvi_handle,
        b"DVI\x00" as *const u8 as *const i8,
        b"design_size\x00" as *const u8 as *const i8,
    );
    dir_length = tt_get_unsigned_byte(dvi_handle) as i32;
    name_length = tt_get_unsigned_byte(dvi_handle) as i32;
    directory = new(
        ((dir_length + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32
    ) as *mut i8;
    if ttstub_input_read(dvi_handle, directory, dir_length as size_t) != dir_length as i64 {
        _tt_abort(invalid_signature.as_ptr());
    }
    *directory.offset(dir_length as isize) = '\u{0}' as i32 as i8;
    free(directory as *mut libc::c_void);
    font_name = new(((name_length + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    if ttstub_input_read(dvi_handle, font_name, name_length as size_t) != name_length as i64 {
        _tt_abort(invalid_signature.as_ptr());
    }
    *font_name.offset(name_length as isize) = '\u{0}' as i32 as i8;
    (*def_fonts.offset(num_def_fonts as isize)).tex_id = tex_id;
    let ref mut fresh14 = (*def_fonts.offset(num_def_fonts as isize)).font_name;
    *fresh14 = font_name;
    (*def_fonts.offset(num_def_fonts as isize)).point_size = point_size as spt_t;
    (*def_fonts.offset(num_def_fonts as isize)).design_size = design_size as spt_t;
    (*def_fonts.offset(num_def_fonts as isize)).used = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).native = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).rgba_color = 0xffffffffu32;
    (*def_fonts.offset(num_def_fonts as isize)).face_index = 0_u32;
    (*def_fonts.offset(num_def_fonts as isize)).layout_dir = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).extend = 0x10000i32;
    (*def_fonts.offset(num_def_fonts as isize)).slant = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).embolden = 0i32;
    num_def_fonts = num_def_fonts.wrapping_add(1);
}
unsafe extern "C" fn read_native_font_record(mut tex_id: u32) {
    let mut flags: u32 = 0;
    let mut point_size: u32 = 0;
    let mut font_name: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut index: u32 = 0;
    if num_def_fonts >= max_def_fonts {
        max_def_fonts = max_def_fonts.wrapping_add(16u32);
        def_fonts = renew(
            def_fonts as *mut libc::c_void,
            (max_def_fonts as u64).wrapping_mul(::std::mem::size_of::<font_def>() as u64) as u32,
        ) as *mut font_def
    }
    point_size = tt_get_positive_quad(
        dvi_handle,
        b"DVI\x00" as *const u8 as *const i8,
        b"point_size\x00" as *const u8 as *const i8,
    );
    flags = tt_get_unsigned_pair(dvi_handle) as u32;
    len = tt_get_unsigned_byte(dvi_handle) as i32;
    font_name =
        new(((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    if ttstub_input_read(dvi_handle, font_name, len as size_t) != len as i64 {
        _tt_abort(invalid_signature.as_ptr());
    }
    *font_name.offset(len as isize) = '\u{0}' as i32 as i8;
    index = tt_get_positive_quad(
        dvi_handle,
        b"DVI\x00" as *const u8 as *const i8,
        b"index\x00" as *const u8 as *const i8,
    );
    (*def_fonts.offset(num_def_fonts as isize)).tex_id = tex_id;
    let ref mut fresh15 = (*def_fonts.offset(num_def_fonts as isize)).font_name;
    *fresh15 = font_name;
    (*def_fonts.offset(num_def_fonts as isize)).face_index = index;
    (*def_fonts.offset(num_def_fonts as isize)).point_size = point_size as spt_t;
    (*def_fonts.offset(num_def_fonts as isize)).design_size = 655360i32;
    (*def_fonts.offset(num_def_fonts as isize)).used = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).native = 1i32;
    (*def_fonts.offset(num_def_fonts as isize)).layout_dir = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).rgba_color = 0xffffffffu32;
    (*def_fonts.offset(num_def_fonts as isize)).extend = 0x10000i32;
    (*def_fonts.offset(num_def_fonts as isize)).slant = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).embolden = 0i32;
    if flags & 0x100_u32 != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).layout_dir = 1i32
    }
    if flags & 0x200_u32 != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).rgba_color = tt_get_unsigned_quad(dvi_handle)
    }
    if flags & 0x1000_u32 != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).extend = tt_get_signed_quad(dvi_handle)
    }
    if flags & 0x2000_u32 != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).slant = tt_get_signed_quad(dvi_handle)
    }
    if flags & 0x4000_u32 != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).embolden = tt_get_signed_quad(dvi_handle)
    }
    num_def_fonts = num_def_fonts.wrapping_add(1);
}
unsafe extern "C" fn get_dvi_fonts(mut post_location: i32) {
    let mut code: i32 = 0;
    ttstub_input_seek(dvi_handle, (post_location + 29i32) as ssize_t, 0i32);
    loop {
        code = tt_get_unsigned_byte(dvi_handle) as i32;
        if !(code != 249i32) {
            break;
        }
        match code {
            243 | 244 | 245 | 246 => {
                read_font_record(tt_get_unsigned_num(dvi_handle, (code - 243i32) as u8));
            }
            252 => {
                need_XeTeX(code);
                read_native_font_record(tt_get_signed_quad(dvi_handle) as u32);
            }
            _ => {
                info!("Unexpected op code: {:3}\n", code,);
                _tt_abort(invalid_signature.as_ptr());
            }
        }
    }
    if verbose > 2i32 {
        let mut i: u32 = 0;
        info!("\n");
        info!("DVI file font info\n");
        i = 0_u32;
        while i < num_def_fonts {
            dpx_message(
                b"TeX Font: %10s loaded at ID=%5d, \x00" as *const u8 as *const i8,
                (*def_fonts.offset(i as isize)).font_name,
                (*def_fonts.offset(i as isize)).tex_id,
            );
            info!(
                "size={:5.2}pt (scaled {:4.1}%)",
                (*def_fonts.offset(i as isize)).point_size as f64 * dvi2pts,
                100.0f64
                    * ((*def_fonts.offset(i as isize)).point_size as f64
                        / (*def_fonts.offset(i as isize)).design_size as f64),
            );
            info!("\n");
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn get_comment() {
    let mut length: i32 = 0;
    ttstub_input_seek(dvi_handle, 14i32 as ssize_t, 0i32);
    length = tt_get_unsigned_byte(dvi_handle) as i32;
    if ttstub_input_read(dvi_handle, dvi_info.comment.as_mut_ptr(), length as size_t)
        != length as i64
    {
        _tt_abort(invalid_signature.as_ptr());
    }
    dvi_info.comment[length as usize] = '\u{0}' as i32 as i8;
    if verbose != 0 {
        dpx_message(
            b"DVI Comment: %s\n\x00" as *const u8 as *const i8,
            dvi_info.comment.as_mut_ptr(),
        );
    };
}
static mut dvi_state: dvi_registers = dvi_registers {
    h: 0,
    v: 0,
    w: 0,
    x: 0,
    y: 0,
    z: 0,
    d: 0,
};
static mut dvi_stack: [dvi_registers; 256] = [dvi_registers {
    h: 0,
    v: 0,
    w: 0,
    x: 0,
    y: 0,
    z: 0,
    d: 0,
}; 256];
static mut dvi_stack_depth: i32 = 0i32;
static mut current_font: i32 = -1i32;
static mut processing_page: i32 = 0i32;
unsafe extern "C" fn clear_state() {
    dvi_state.h = 0i32;
    dvi_state.v = 0i32;
    dvi_state.w = 0i32;
    dvi_state.x = 0i32;
    dvi_state.y = 0i32;
    dvi_state.z = 0i32;
    dvi_state.d = 0_u32;
    pdf_dev_set_dirmode(0i32);
    dvi_stack_depth = 0i32;
    current_font = -1i32;
}
/* Migrated from pdfdev.c:
 * The following codes are originally put into pdfdev.c.
 * But they are moved to here to make PDF output independent
 * from DVI input.
 * pdfdoc, pdfspecial and htex are also modified. pdfspecial
 * and htex does tag/untag depth. pdfdev and pdfdoc now does
 * not care about line-breaking at all.
 */
static mut marked_depth: i32 = 0i32;
static mut tagged_depth: i32 = -1i32;
unsafe extern "C" fn dvi_mark_depth() {
    /* If decreasing below tagged_depth */
    if link_annot != 0 && marked_depth == tagged_depth && dvi_stack_depth == tagged_depth - 1i32 {
        /*
         * See if this appears to be the end of a "logical unit"
         * that's been broken.  If so, flush the logical unit.
         */
        pdf_doc_break_annot();
    }
    marked_depth = dvi_stack_depth;
}
/*
 * The following routines setup and tear down a callback at a
 * certain stack depth. This is used to handle broken (linewise)
 * links.
 */
#[no_mangle]
pub unsafe extern "C" fn dvi_tag_depth() {
    tagged_depth = marked_depth;
    dvi_compute_boxes(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_untag_depth() {
    tagged_depth = -1i32;
    dvi_compute_boxes(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_compute_boxes(mut flag: i32) {
    compute_boxes = flag;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_link_annot(mut flag: i32) {
    link_annot = flag;
}
/* allow other modules (pdfdev) to ask whether we're collecting box areas */
#[no_mangle]
pub unsafe extern "C" fn dvi_is_tracking_boxes() -> bool {
    compute_boxes != 0 && link_annot != 0 && marked_depth >= tagged_depth
}
/* link or nolink:
 * See dvipdfm (not x) user's manual on pdf:link and pdf:nolink.
 * This is workaround for preventing inclusion of pagenation artifact such as
 * footnote and page number in link annotation.
 */
/* The followings are for calculating bounding box of text for annotation.
 * DVI uses push/pop to do line-feed-carriage-return. So line breaking is
 * handled by inspecting current depth of DVI register stack.
 */
#[no_mangle]
pub unsafe extern "C" fn dvi_do_special(mut buffer: *const libc::c_void, mut size: i32) {
    let mut x_user: f64 = 0.; /* VF or device font ID */
    let mut y_user: f64 = 0.;
    let mut mag: f64 = 0.;
    let mut p: *const i8 = 0 as *const i8;
    graphics_mode();
    p = buffer as *const i8;
    x_user = dvi_state.h as f64 * dvi2pts;
    y_user = -dvi_state.v as f64 * dvi2pts;
    mag = dvi_tell_mag();
    if spc_exec_special(p, size, x_user, y_user, mag) < 0i32 {
        if verbose != 0 {
            dump(p, p.offset(size as isize));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_unit_size() -> f64 {
    dvi2pts
}
#[no_mangle]
pub unsafe extern "C" fn dvi_locate_font(mut tfm_name: *const i8, mut ptsize: spt_t) -> u32 {
    let mut cur_id: u32 = 0;
    let mut name: *const i8 = tfm_name;
    let mut subfont_id: i32 = -1i32;
    let mut font_id: i32 = 0;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    if verbose != 0 {
        dpx_message(
            b"<%s@%.2fpt\x00" as *const u8 as *const i8,
            tfm_name,
            ptsize as f64 * dvi2pts,
        );
    }
    need_more_fonts(1_u32);
    /* This routine needs to be recursive/reentrant. Load current high water
     * mark into an automatic variable.
     */
    let fresh16 = num_loaded_fonts;
    num_loaded_fonts = num_loaded_fonts.wrapping_add(1);
    cur_id = fresh16;
    mrec = pdf_lookup_fontmap_record(tfm_name);
    /* Load subfont mapping table */
    if !mrec.is_null()
        && !(*mrec).charmap.sfd_name.is_null()
        && !(*mrec).charmap.subfont_id.is_null()
    {
        subfont_id = sfd_load_record((*mrec).charmap.sfd_name, (*mrec).charmap.subfont_id)
    }
    memset(
        &mut *loaded_fonts.offset(cur_id as isize) as *mut loaded_font as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<loaded_font>() as u64,
    );
    /* TFM must exist here. */
    (*loaded_fonts.offset(cur_id as isize)).tfm_id = tfm_open(tfm_name, 1i32);
    (*loaded_fonts.offset(cur_id as isize)).subfont_id = subfont_id;
    (*loaded_fonts.offset(cur_id as isize)).size = ptsize;
    /* This will be reset later if it was really generated by the dvi file. */
    (*loaded_fonts.offset(cur_id as isize)).source = 2i32;
    /* The order of searching fonts is as follows:
     *
     * 1. If mrec is null, that is, there is no map entry matching
     *    with tfm_name, then search a virtual font matching with
     *    tfm_name at first. If no virtual font is found, search a
     *    PK font matching with tfm_name.
     *
     * 2. If mrec is non-null, search a physical scalable font.
     *
     * 3. Notice that every subfont gets non-null mrec. In this case,
     *    enc_name corresponding to mrec will be used instead of mrec.
     *    That is enc_name is NULL, search a virtual font for Omega (.ovf)
     *    matching with the base name of the subfont. If no virtual font
     *    for Omega is found, it is a fatal error because there is no PK font
     *    for Omega.
     */
    if mrec.is_null() {
        font_id = vf_locate_font(tfm_name, ptsize);
        if font_id >= 0i32 {
            (*loaded_fonts.offset(cur_id as isize)).type_0 = 2i32;
            (*loaded_fonts.offset(cur_id as isize)).font_id = font_id;
            if verbose != 0 {
                info!("(VF)>");
            }
            return cur_id;
        }
    } else if subfont_id >= 0i32 && !(*mrec).map_name.is_null() {
        let mut mrec1: *mut fontmap_rec = pdf_lookup_fontmap_record((*mrec).map_name);
        /* Sorry, I don't understand this well... Please fix.
         * The purpose of this seems to be:
         *
         *   Map 8-bit char codes in subfont to 16-bit code with SFD mapping
         *   and map subfonts to single OVF font.
         *
         * But it apparently only does TFM -> OVF mapping but no character
         * code mapping. Please see dvi_set(), you can't have both font->type
         * VIRTUAL and font->subfont_id >= 0. Am I missing something?
         */
        /* enc_name=NULL should be used only for 'built-in' encoding.
         * Please fix this!
         */
        if !mrec1.is_null() && (*mrec1).enc_name.is_null() {
            font_id = vf_locate_font((*mrec1).font_name, ptsize);
            if font_id < 0i32 {
                dpx_warning(
                    b"Could not locate Omega Virtual Font \"%s\" for \"%s\".\x00" as *const u8
                        as *const i8,
                    (*mrec1).font_name,
                    tfm_name,
                );
            } else {
                (*loaded_fonts.offset(cur_id as isize)).type_0 = 2i32;
                (*loaded_fonts.offset(cur_id as isize)).font_id = font_id;
                if verbose != 0 {
                    info!("(OVF)>");
                }
                return cur_id;
            }
        }
    }
    /* 1 */
    /* Failed to load a virtual font so we try to load a physical font. */
    /* If mrec->map_name is not NULL, font name identified in PDF output
     * is different than tfm_name, this can happen for subfonts grouped
     * into a single "intermediate" font foo@SFD@.
     * This is necessary for optimal output; to avoid unnecessary creation
     * of multiple instances of a same font, to avoid frequent font selection
     * and break of string_mode.
     */
    if !mrec.is_null() && !(*mrec).map_name.is_null() {
        name = (*mrec).map_name
    } else {
        name = tfm_name
    }
    /* We need ptsize for PK font creation. */
    font_id = pdf_dev_locate_font(name, ptsize);
    if font_id < 0i32 {
        dpx_warning(
            b"Could not locate a virtual/physical font for TFM \"%s\".\x00" as *const u8
                as *const i8,
            tfm_name,
        );
        if !mrec.is_null() && !(*mrec).map_name.is_null() {
            /* has map_name */
            let mut mrec1_0: *mut fontmap_rec = pdf_lookup_fontmap_record((*mrec).map_name); // CHECK this is enough
            dpx_warning(b">> This font is mapped to an intermediate 16-bit font \"%s\" with SFD charmap=<%s,%s>,\x00"
                            as *const u8 as *const i8,
                        (*mrec).map_name, (*mrec).charmap.sfd_name,
                        (*mrec).charmap.subfont_id);
            if mrec1_0.is_null() {
                dpx_warning(
                    b">> but I couldn\'t find font mapping for \"%s\".\x00" as *const u8
                        as *const i8,
                    (*mrec).map_name,
                );
            } else {
                dpx_warning(
                    b">> and then mapped to a physical font \"%s\" by fontmap.\x00" as *const u8
                        as *const i8,
                    (*mrec1_0).font_name,
                );
                dpx_warning(
                    b">> Please check if kpathsea library can find this font: %s\x00" as *const u8
                        as *const i8,
                    (*mrec1_0).font_name,
                );
            }
        } else if !mrec.is_null() && (*mrec).map_name.is_null() {
            dpx_warning(
                b">> This font is mapped to a physical font \"%s\".\x00" as *const u8 as *const i8,
                (*mrec).font_name,
            );
            dpx_warning(
                b">> Please check if kpathsea library can find this font: %s\x00" as *const u8
                    as *const i8,
                (*mrec).font_name,
            );
        } else {
            warn!(">> There are no valid font mapping entry for this font.");
            dpx_warning(
                b">> Font file name \"%s\" was assumed but failed to locate that font.\x00"
                    as *const u8 as *const i8,
                tfm_name,
            );
        }
        panic!("Cannot proceed without .vf or \"physical\" font for PDF output...");
    }
    (*loaded_fonts.offset(cur_id as isize)).type_0 = 1i32;
    (*loaded_fonts.offset(cur_id as isize)).font_id = font_id;
    if verbose != 0 {
        info!(">");
    }
    cur_id
}
unsafe extern "C" fn dvi_locate_native_font(
    mut filename: *const i8,
    mut index: u32,
    mut ptsize: spt_t,
    mut layout_dir: i32,
    mut extend: i32,
    mut slant: i32,
    mut embolden: i32,
) -> i32 {
    let mut cur_id: i32 = -1i32;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut fontmap_key: *mut i8 = 0 as *mut i8;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: u32 = 0_u32;
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut is_dfont: i32 = 0i32;
    let mut is_type1: i32 = 0i32;
    if verbose != 0 {
        dpx_message(
            b"<%s@%.2fpt\x00" as *const u8 as *const i8,
            filename,
            ptsize as f64 * dvi2pts,
        );
    }
    handle = dpx_open_dfont_file(filename);
    if !handle.is_null() {
        is_dfont = 1i32
    } else {
        handle = dpx_open_type1_file(filename);
        if !handle.is_null() {
            is_type1 = 1i32
        } else {
            handle = dpx_open_opentype_file(filename);
            if handle.is_null() && {
                handle = dpx_open_truetype_file(filename);
                handle.is_null()
            } {
                _tt_abort(
                    b"Cannot proceed without the font: %s\x00" as *const u8 as *const i8,
                    filename,
                );
            }
        }
    }
    need_more_fonts(1_u32);
    let fresh17 = num_loaded_fonts;
    num_loaded_fonts = num_loaded_fonts.wrapping_add(1);
    cur_id = fresh17 as i32;
    fontmap_key = xmalloc(strlen(filename).wrapping_add(40i32 as u64)) as *mut i8;
    sprintf(
        fontmap_key,
        b"%s/%u/%c/%d/%d/%d\x00" as *const u8 as *const i8,
        filename,
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
    mrec = pdf_lookup_fontmap_record(fontmap_key);
    if mrec.is_null() {
        mrec =
            pdf_insert_native_fontmap_record(filename, index, layout_dir, extend, slant, embolden);
        if mrec.is_null() {
            _tt_abort(
                b"Failed to insert font record for font: %s\x00" as *const u8 as *const i8,
                filename,
            );
        }
    }
    memset(
        &mut *loaded_fonts.offset(cur_id as isize) as *mut loaded_font as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<loaded_font>() as u64,
    );
    (*loaded_fonts.offset(cur_id as isize)).font_id = pdf_dev_locate_font(fontmap_key, ptsize);
    (*loaded_fonts.offset(cur_id as isize)).size = ptsize;
    (*loaded_fonts.offset(cur_id as isize)).type_0 = 4i32;
    free(fontmap_key as *mut libc::c_void);
    if is_type1 != 0 {
        let mut cffont: *mut cff_font = 0 as *mut cff_font;
        let mut enc_vec: [*mut i8; 256] = [0 as *mut i8; 256];
        /*if (!is_pfb(fp))
         *  _tt_abort("Failed to read Type 1 font \"%s\".", filename);
         */
        warn!("skipping PFB sanity check -- needs Tectonic I/O update");
        memset(
            enc_vec.as_mut_ptr() as *mut libc::c_void,
            0i32,
            (256i32 as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
        );
        cffont = t1_load_font(enc_vec.as_mut_ptr(), 0i32, handle);
        if cffont.is_null() {
            _tt_abort(
                b"Failed to read Type 1 font \"%s\".\x00" as *const u8 as *const i8,
                filename,
            );
        }
        let ref mut fresh18 = (*loaded_fonts.offset(cur_id as isize)).cffont;
        *fresh18 = cffont;
        if cff_dict_known((*cffont).topdict, b"FontBBox\x00" as *const u8 as *const i8) != 0 {
            (*loaded_fonts.offset(cur_id as isize)).ascent = cff_dict_get(
                (*cffont).topdict,
                b"FontBBox\x00" as *const u8 as *const i8,
                3i32,
            ) as i32;
            (*loaded_fonts.offset(cur_id as isize)).descent = cff_dict_get(
                (*cffont).topdict,
                b"FontBBox\x00" as *const u8 as *const i8,
                1i32,
            ) as i32
        } else {
            (*loaded_fonts.offset(cur_id as isize)).ascent = 690i32;
            (*loaded_fonts.offset(cur_id as isize)).descent = -190i32
        }
        (*loaded_fonts.offset(cur_id as isize)).unitsPerEm = 1000_u32;
        (*loaded_fonts.offset(cur_id as isize)).numGlyphs = (*cffont).num_glyphs as u32;
        ttstub_input_close(handle);
    } else {
        if is_dfont != 0 {
            sfont = dfont_open(handle, index as i32)
        } else {
            sfont = sfnt_open(handle)
        }
        if (*sfont).type_0 == 1i32 << 4i32 {
            offset = ttc_read_offset(sfont, index as i32)
        } else if (*sfont).type_0 == 1i32 << 8i32 {
            offset = (*sfont).offset
        }
        sfnt_read_table_directory(sfont, offset);
        head = tt_read_head_table(sfont);
        maxp = tt_read_maxp_table(sfont);
        hhea = tt_read_hhea_table(sfont);
        (*loaded_fonts.offset(cur_id as isize)).ascent = (*hhea).ascent as i32;
        (*loaded_fonts.offset(cur_id as isize)).descent = (*hhea).descent as i32;
        (*loaded_fonts.offset(cur_id as isize)).unitsPerEm = (*head).unitsPerEm as u32;
        (*loaded_fonts.offset(cur_id as isize)).numGlyphs = (*maxp).numGlyphs as u32;
        if layout_dir == 1i32
            && sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8) > 0_u32
        {
            let mut vhea: *mut tt_vhea_table = tt_read_vhea_table(sfont);
            sfnt_locate_table(sfont, b"vmtx\x00" as *const u8 as *const i8);
            let ref mut fresh19 = (*loaded_fonts.offset(cur_id as isize)).hvmt;
            *fresh19 = tt_read_longMetrics(
                sfont,
                (*maxp).numGlyphs,
                (*vhea).numOfLongVerMetrics,
                (*vhea).numOfExSideBearings,
            );
            free(vhea as *mut libc::c_void);
        } else {
            sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const i8);
            let ref mut fresh20 = (*loaded_fonts.offset(cur_id as isize)).hvmt;
            *fresh20 = tt_read_longMetrics(
                sfont,
                (*maxp).numGlyphs,
                (*hhea).numOfLongHorMetrics,
                (*hhea).numOfExSideBearings,
            )
        }
        free(hhea as *mut libc::c_void);
        free(maxp as *mut libc::c_void);
        free(head as *mut libc::c_void);
        sfnt_close(sfont);
        ttstub_input_close(handle);
    }
    (*loaded_fonts.offset(cur_id as isize)).layout_dir = layout_dir;
    (*loaded_fonts.offset(cur_id as isize)).extend = (*mrec).opt.extend as f32;
    (*loaded_fonts.offset(cur_id as isize)).slant = (*mrec).opt.slant as f32;
    (*loaded_fonts.offset(cur_id as isize)).embolden = (*mrec).opt.bold as f32;
    if verbose != 0 {
        info!(">");
    }
    cur_id
}
#[no_mangle]
pub unsafe extern "C" fn dvi_dev_xpos() -> f64 {
    dvi_state.h as f64 * dvi2pts
}
#[no_mangle]
pub unsafe extern "C" fn dvi_dev_ypos() -> f64 {
    -(dvi_state.v as f64 * dvi2pts)
}
unsafe extern "C" fn do_moveto(mut x: i32, mut y: i32) {
    dvi_state.h = x;
    dvi_state.v = y;
}
/* FIXME: dvi_forward() might be a better name */
#[no_mangle]
pub unsafe extern "C" fn dvi_right(mut x: i32) {
    if lr_mode >= 2i32 {
        lr_width = (lr_width as u32).wrapping_add(x as u32) as u32;
        return;
    }
    if lr_mode == 1i32 {
        x = -x
    }
    match dvi_state.d {
        0 => dvi_state.h += x,
        1 => dvi_state.v += x,
        3 => dvi_state.v -= x,
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_down(mut y: i32) {
    if lr_mode < 2i32 {
        match dvi_state.d {
            0 => dvi_state.v += y,
            1 => dvi_state.h -= y,
            3 => dvi_state.h += y,
            _ => {}
        }
    };
}
/* _FIXME_
 * CMap decoder wants multibyte strings as input but
 * how DVI char codes are converted to multibyte sting
 * is not clear.
 */
#[no_mangle]
pub unsafe extern "C" fn dvi_set(mut ch: i32) {
    let mut font: *mut loaded_font = 0 as *mut loaded_font;
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut wbuf: [u8; 4] = [0; 4];
    if current_font < 0i32 {
        panic!("No font selected!");
    }
    /* The division by dvi2pts seems strange since we actually know the
     * "dvi" size of the fonts contained in the DVI file.  In other
     * words, we converted from DVI units to pts and back again!
     * The problem comes from fonts defined in VF files where we don't know
     * the DVI size.  It's keeping me sane to keep *point sizes* of *all*
     * fonts in the dev.c file and convert them back if necessary.
     */
    font = &mut *loaded_fonts.offset(current_font as isize) as *mut loaded_font; /* Will actually move left */
    width = tfm_get_fw_width((*font).tfm_id, ch);
    width = sqxfw((*font).size, width);
    if lr_mode >= 2i32 {
        lr_width = (lr_width as u32).wrapping_add(width as u32) as u32;
        return;
    }
    if lr_mode == 1i32 {
        dvi_right(width);
    }
    match (*font).type_0 {
        1 => {
            if ch > 65535i32 {
                /* _FIXME_ */
                wbuf[0] =
                    (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) >> 8i32 & 0xffi32) as u8;
                wbuf[1] = (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) & 0xffi32) as u8;
                wbuf[2] = (0xdc00i32 + (ch & 0x3ffi32) >> 8i32 & 0xffi32) as u8;
                wbuf[3] = (0xdc00i32 + (ch & 0x3ffi32) & 0xffi32) as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    4i32 as size_t,
                    width,
                    (*font).font_id,
                    2i32,
                );
            } else if ch > 255i32 {
                /* _FIXME_ */
                wbuf[0] = (ch >> 8i32 & 0xffi32) as u8; /* push/pop invoked */
                wbuf[1] = (ch & 0xffi32) as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    2i32 as size_t,
                    width,
                    (*font).font_id,
                    2i32,
                );
            } else if (*font).subfont_id >= 0i32 {
                let mut uch: u16 = lookup_sfd_record((*font).subfont_id, ch as u8);
                wbuf[0] = (uch as i32 >> 8i32 & 0xffi32) as u8;
                wbuf[1] = (uch as i32 & 0xffi32) as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    2i32 as size_t,
                    width,
                    (*font).font_id,
                    2i32,
                );
            } else {
                wbuf[0] = ch as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    1i32 as size_t,
                    width,
                    (*font).font_id,
                    1i32,
                );
            }
            if dvi_is_tracking_boxes() {
                let mut rect = pdf_rect::new();
                height = tfm_get_fw_height((*font).tfm_id, ch);
                depth = tfm_get_fw_depth((*font).tfm_id, ch);
                height = sqxfw((*font).size, height);
                depth = sqxfw((*font).size, depth);
                pdf_dev_set_rect(&mut rect, dvi_state.h, -dvi_state.v, width, height, depth);
                pdf_doc_expand_box(&mut rect);
            }
        }
        2 => {
            vf_set_char(ch, (*font).font_id);
        }
        _ => {}
    }
    if lr_mode == 0i32 {
        dvi_right(width);
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_put(mut ch: i32) {
    let mut font: *mut loaded_font = 0 as *mut loaded_font;
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut wbuf: [u8; 4] = [0; 4];
    if current_font < 0i32 {
        panic!("No font selected!");
    }
    font = &mut *loaded_fonts.offset(current_font as isize) as *mut loaded_font;
    match (*font).type_0 {
        1 => {
            width = tfm_get_fw_width((*font).tfm_id, ch);
            width = sqxfw((*font).size, width);
            /* Treat a single character as a one byte string and use the
             * string routine.
             */
            if ch > 65535i32 {
                /* _FIXME_ */
                wbuf[0] =
                    (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) >> 8i32 & 0xffi32) as u8;
                wbuf[1] = (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) & 0xffi32) as u8;
                wbuf[2] = (0xdc00i32 + (ch & 0x3ffi32) >> 8i32 & 0xffi32) as u8;
                wbuf[3] = (0xdc00i32 + (ch & 0x3ffi32) & 0xffi32) as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    4i32 as size_t,
                    width,
                    (*font).font_id,
                    2i32,
                );
            } else if ch > 255i32 {
                /* _FIXME_ */
                wbuf[0] = (ch >> 8i32 & 0xffi32) as u8;
                wbuf[1] = (ch & 0xffi32) as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    2i32 as size_t,
                    width,
                    (*font).font_id,
                    2i32,
                );
            } else if (*font).subfont_id >= 0i32 {
                let mut uch: u32 = 0;
                uch = lookup_sfd_record((*font).subfont_id, ch as u8) as u32;
                wbuf[0] = (uch >> 8i32 & 0xff_u32) as u8;
                wbuf[1] = (uch & 0xff_u32) as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    2i32 as size_t,
                    width,
                    (*font).font_id,
                    2i32,
                );
            } else {
                wbuf[0] = ch as u8;
                pdf_dev_set_string(
                    dvi_state.h,
                    -dvi_state.v,
                    wbuf.as_mut_ptr() as *const libc::c_void,
                    1i32 as size_t,
                    width,
                    (*font).font_id,
                    1i32,
                );
            }
            if dvi_is_tracking_boxes() {
                let mut rect = pdf_rect::new();
                height = tfm_get_fw_height((*font).tfm_id, ch);
                depth = tfm_get_fw_depth((*font).tfm_id, ch);
                height = sqxfw((*font).size, height);
                depth = sqxfw((*font).size, depth);
                pdf_dev_set_rect(&mut rect, dvi_state.h, -dvi_state.v, width, height, depth);
                pdf_doc_expand_box(&mut rect);
            }
        }
        2 => {
            vf_set_char(ch, (*font).font_id);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_rule(mut width: i32, mut height: i32) {
    if width > 0i32 && height > 0i32 {
        do_moveto(dvi_state.h, dvi_state.v);
        match dvi_state.d {
            0 => {
                pdf_dev_set_rule(dvi_state.h, -dvi_state.v, width, height);
            }
            1 => {
                pdf_dev_set_rule(dvi_state.h, -dvi_state.v - width, height, width);
            }
            3 => {
                pdf_dev_set_rule(dvi_state.h - height, -dvi_state.v, height, width);
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_dirchg(mut dir: u8) {
    if verbose != 0 {
        eprintln!("  > dvi_dirchg {}", dir as i32);
    }
    dvi_state.d = dir as u32;
    pdf_dev_set_dirmode(dvi_state.d as i32);
    /* 0: horizontal, 1,3: vertical */
}
unsafe extern "C" fn do_setrule() {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    height = get_buffered_signed_quad();
    width = get_buffered_signed_quad();
    match lr_mode {
        0 => {
            dvi_rule(width, height);
            dvi_right(width);
        }
        1 => {
            dvi_right(width);
            dvi_rule(width, height);
        }
        _ => lr_width = (lr_width as u32).wrapping_add(width as u32) as u32 as u32,
    };
}
unsafe extern "C" fn do_putrule() {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    height = get_buffered_signed_quad();
    width = get_buffered_signed_quad();
    match lr_mode {
        0 => {
            dvi_rule(width, height);
        }
        1 => {
            dvi_right(width);
            dvi_rule(width, height);
            dvi_right(-width);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_push() {
    if dvi_stack_depth as u32 >= 256u32 {
        panic!("DVI stack exceeded limit.");
    }
    let fresh21 = dvi_stack_depth;
    dvi_stack_depth = dvi_stack_depth + 1;
    dvi_stack[fresh21 as usize] = dvi_state;
}
#[no_mangle]
pub unsafe extern "C" fn dpx_dvi_pop() {
    if dvi_stack_depth <= 0i32 {
        panic!("Tried to pop an empty stack.");
    }
    dvi_stack_depth -= 1;
    dvi_state = dvi_stack[dvi_stack_depth as usize];
    do_moveto(dvi_state.h, dvi_state.v);
    pdf_dev_set_dirmode(dvi_state.d as i32);
    /* 0: horizontal, 1,3: vertical */
}
#[no_mangle]
pub unsafe extern "C" fn dvi_w(mut ch: i32) {
    dvi_state.w = ch;
    dvi_right(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_w0() {
    dvi_right(dvi_state.w);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_x(mut ch: i32) {
    dvi_state.x = ch;
    dvi_right(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_x0() {
    dvi_right(dvi_state.x);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_y(mut ch: i32) {
    dvi_state.y = ch;
    dvi_down(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_y0() {
    dvi_down(dvi_state.y);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_z(mut ch: i32) {
    dvi_state.z = ch;
    dvi_down(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_z0() {
    dvi_down(dvi_state.z);
}
unsafe extern "C" fn skip_fntdef() {
    let mut area_len: i32 = 0;
    let mut name_len: i32 = 0;
    tt_skip_bytes(12_u32, dvi_handle);
    area_len = tt_get_unsigned_byte(dvi_handle) as i32;
    name_len = tt_get_unsigned_byte(dvi_handle) as i32;
    tt_skip_bytes((area_len + name_len) as u32, dvi_handle);
}
/* when pre-scanning the page, we process fntdef
and remove the fntdef opcode from the buffer */
unsafe extern "C" fn do_fntdef(mut tex_id: u32) {
    if linear != 0 {
        read_font_record(tex_id);
    } else {
        skip_fntdef();
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_set_font(mut font_id: i32) {
    current_font = font_id;
}
unsafe extern "C" fn do_fnt(mut tex_id: u32) {
    let mut i: u32 = 0;
    i = 0_u32;
    while i < num_def_fonts {
        if (*def_fonts.offset(i as isize)).tex_id == tex_id {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == num_def_fonts {
        panic!(
            "Tried to select a font that hasn\'t been defined: id={}",
            tex_id
        );
    }
    if (*def_fonts.offset(i as isize)).used == 0 {
        let mut font_id: u32 = 0;
        if (*def_fonts.offset(i as isize)).native != 0 {
            font_id = dvi_locate_native_font(
                (*def_fonts.offset(i as isize)).font_name,
                (*def_fonts.offset(i as isize)).face_index,
                (*def_fonts.offset(i as isize)).point_size,
                (*def_fonts.offset(i as isize)).layout_dir,
                (*def_fonts.offset(i as isize)).extend,
                (*def_fonts.offset(i as isize)).slant,
                (*def_fonts.offset(i as isize)).embolden,
            ) as u32
        } else {
            font_id = dvi_locate_font(
                (*def_fonts.offset(i as isize)).font_name,
                (*def_fonts.offset(i as isize)).point_size,
            )
        }
        (*loaded_fonts.offset(font_id as isize)).rgba_color =
            (*def_fonts.offset(i as isize)).rgba_color;
        (*loaded_fonts.offset(font_id as isize)).source = 1i32;
        (*def_fonts.offset(i as isize)).used = 1i32;
        (*def_fonts.offset(i as isize)).font_id = font_id as i32
    }
    current_font = (*def_fonts.offset(i as isize)).font_id;
}
unsafe extern "C" fn do_xxx(mut size: i32) {
    if lr_mode < 2i32 {
        dvi_do_special(
            dvi_page_buffer.offset(dvi_page_buf_index as isize) as *const libc::c_void,
            size,
        );
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(size as u32);
}
unsafe extern "C" fn do_bop() {
    let mut i: u32 = 0;
    if processing_page != 0 {
        panic!("Got a bop in the middle of a page!");
    }
    /* For now, ignore TeX's count registers */
    i = 0_u32;
    while i < 10_u32 {
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4_u32);
        i = i.wrapping_add(1)
    }
    /* Ignore previous page pointer since we have already
     * saved this information
     */
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4_u32);
    clear_state();
    processing_page = 1i32;
    pdf_doc_begin_page(dvi_tell_mag(), dev_origin_x, dev_origin_y);
    spc_exec_at_begin_page();
}
unsafe extern "C" fn do_eop() {
    processing_page = 0i32;
    if dvi_stack_depth != 0i32 {
        panic!("DVI stack depth is not zero at end of page");
    }
    spc_exec_at_end_page();
    pdf_doc_end_page();
}
unsafe extern "C" fn do_dir() {
    dvi_state.d = get_buffered_unsigned_byte() as u32;
    pdf_dev_set_dirmode(dvi_state.d as i32);
    /* 0: horizontal, 1,3: vertical */
}
unsafe extern "C" fn lr_width_push() {
    if lr_width_stack_depth >= 256u32 {
        panic!("Segment width stack exceeded limit.");
        /* must precede dvi_right */
    }
    let fresh22 = lr_width_stack_depth;
    lr_width_stack_depth = lr_width_stack_depth.wrapping_add(1);
    lr_width_stack[fresh22 as usize] = lr_width;
}
unsafe extern "C" fn lr_width_pop() {
    if lr_width_stack_depth <= 0_u32 {
        panic!("Tried to pop an empty segment width stack.");
    }
    lr_width_stack_depth = lr_width_stack_depth.wrapping_sub(1);
    lr_width = lr_width_stack[lr_width_stack_depth as usize];
}
unsafe extern "C" fn dvi_begin_reflect() {
    if lr_mode >= 2i32 {
        lr_mode += 1
    } else {
        lr_state.buf_index = dvi_page_buf_index;
        lr_state.font = current_font;
        lr_state.state = lr_mode;
        lr_mode = 2i32;
        lr_width = 0_u32
    };
}
unsafe extern "C" fn dvi_end_reflect() {
    match lr_mode {
        2 => {
            current_font = lr_state.font;
            dvi_page_buf_index = lr_state.buf_index;
            lr_mode = 0i32 + 1i32 - lr_state.state;
            dvi_right(-(lr_width as i32));
            lr_width_push();
        }
        0 | 1 => {
            lr_width_pop();
            dvi_right(-(lr_width as i32));
            lr_mode = 0i32 + 1i32 - lr_mode
        }
        _ => {
            /* lr_mode > SKIMMING */
            lr_mode -= 1
        }
    }; /* skip point size */
}
unsafe extern "C" fn skip_native_font_def() {
    let mut flags: u32 = 0;
    let mut name_length: i32 = 0;
    tt_skip_bytes(4_u32, dvi_handle);
    flags = tt_get_unsigned_pair(dvi_handle) as u32;
    name_length = tt_get_unsigned_byte(dvi_handle) as i32;
    tt_skip_bytes((name_length + 4i32) as u32, dvi_handle);
    if flags & 0x200_u32 != 0 {
        tt_skip_bytes(4_u32, dvi_handle);
    }
    if flags & 0x1000_u32 != 0 {
        tt_skip_bytes(4_u32, dvi_handle);
    }
    if flags & 0x2000_u32 != 0 {
        tt_skip_bytes(4_u32, dvi_handle);
    }
    if flags & 0x4000_u32 != 0 {
        tt_skip_bytes(4_u32, dvi_handle);
    };
}
unsafe extern "C" fn do_native_font_def(mut tex_id: i32) {
    if linear != 0 {
        read_native_font_record(tex_id as u32);
    } else {
        skip_native_font_def();
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_sub(1);
    /* don't buffer the opcode */
}
unsafe extern "C" fn skip_glyphs() {
    let mut i: u32 = 0; /* Will actually move left */
    let mut slen: u32 = 0_u32; /* freetype glyph index */
    slen = get_buffered_unsigned_pair();
    i = 0_u32;
    while i < slen {
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4_u32);
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4_u32);
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(2_u32);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn do_glyphs(mut do_actual_text: i32) {
    let mut font: *mut loaded_font = 0 as *mut loaded_font;
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut xloc: *mut spt_t = 0 as *mut spt_t;
    let mut yloc: *mut spt_t = 0 as *mut spt_t;
    let mut glyph_width: spt_t = 0i32;
    let mut wbuf: [u8; 2] = [0; 2];
    let mut i: u32 = 0;
    let mut glyph_id: u32 = 0;
    let mut slen: u32 = 0_u32;
    if current_font < 0i32 {
        panic!("No font selected!");
    }
    font = &mut *loaded_fonts.offset(current_font as isize) as *mut loaded_font;
    if do_actual_text != 0 {
        slen = get_buffered_unsigned_pair();
        if lr_mode >= 2i32 {
            i = 0_u32;
            while i < slen {
                dvi_page_buf_index = dvi_page_buf_index.wrapping_add(2_u32);
                i = i.wrapping_add(1)
            }
        } else {
            let mut unicodes: *mut u16 = new((slen as u64)
                .wrapping_mul(::std::mem::size_of::<u16>() as u64)
                as u32) as *mut u16;
            i = 0_u32;
            while i < slen {
                *unicodes.offset(i as isize) = get_buffered_unsigned_pair() as u16;
                i = i.wrapping_add(1)
            }
            pdf_dev_begin_actualtext(unicodes, slen as i32);
            free(unicodes as *mut libc::c_void);
        }
    }
    width = get_buffered_signed_quad();
    if lr_mode >= 2i32 {
        lr_width = (lr_width as u32).wrapping_add(width as u32) as u32;
        skip_glyphs();
        return;
    }
    if lr_mode == 1i32 {
        dvi_right(width);
    }
    slen = get_buffered_unsigned_pair();
    xloc =
        new((slen as u64).wrapping_mul(::std::mem::size_of::<spt_t>() as u64) as u32) as *mut spt_t;
    yloc =
        new((slen as u64).wrapping_mul(::std::mem::size_of::<spt_t>() as u64) as u32) as *mut spt_t;
    i = 0_u32;
    while i < slen {
        *xloc.offset(i as isize) = get_buffered_signed_quad();
        *yloc.offset(i as isize) = get_buffered_signed_quad();
        i = i.wrapping_add(1)
    }
    if (*font).rgba_color != 0xffffffffu32 {
        let mut color: pdf_color = pdf_color {
            num_components: 0,
            spot_color_name: None,
            values: [0.; 4],
        };
        pdf_color_rgbcolor(
            &mut color,
            (((*font).rgba_color >> 24i32) as u8 as i32 & 0xffi32) as f64 / 255i32 as f64,
            (((*font).rgba_color >> 16i32) as u8 as i32 & 0xffi32) as f64 / 255i32 as f64,
            (((*font).rgba_color >> 8i32) as u8 as i32 & 0xffi32) as f64 / 255i32 as f64,
        );
        let color_clone = color.clone();
        pdf_color_push(&mut color, &color_clone);
    }
    i = 0_u32;
    while i < slen {
        glyph_id = get_buffered_unsigned_pair();
        if glyph_id < (*font).numGlyphs {
            let mut advance: u32 = 0;
            let mut ascent: f64 = (*font).ascent as f64;
            let mut descent: f64 = (*font).descent as f64;
            if !(*font).cffont.is_null() {
                let mut cstrings: *mut cff_index = (*(*font).cffont).cstrings;
                let mut gm = t1_ginfo::new();
                /* If .notdef is not the 1st glyph in CharStrings, glyph_id given by
                FreeType should be increased by 1 */
                if (*(*font).cffont).is_notdef_notzero != 0 {
                    glyph_id = glyph_id.wrapping_add(1_u32)
                }
                t1char_get_metrics(
                    (*cstrings)
                        .data
                        .offset(*(*cstrings).offset.offset(glyph_id as isize) as isize)
                        .offset(-1),
                    (*(*cstrings)
                        .offset
                        .offset(glyph_id.wrapping_add(1_u32) as isize))
                    .wrapping_sub(*(*cstrings).offset.offset(glyph_id as isize))
                        as i32,
                    *(*(*font).cffont).subrs.offset(0),
                    &mut gm,
                );
                advance = (if (*font).layout_dir == 0i32 {
                    gm.wx
                } else {
                    gm.wy
                }) as u32;
                ascent = gm.bbox.ury;
                descent = gm.bbox.lly
            } else {
                advance = (*(*font).hvmt.offset(glyph_id as isize)).advance as u32
            }
            glyph_width =
                ((*font).size as f64 * advance as f64 / (*font).unitsPerEm as f64) as spt_t;
            glyph_width = (glyph_width as f32 * (*font).extend) as spt_t;
            if dvi_is_tracking_boxes() {
                let mut rect = pdf_rect::new();
                height = ((*font).size as f64 * ascent / (*font).unitsPerEm as f64) as spt_t;
                depth = ((*font).size as f64 * -descent / (*font).unitsPerEm as f64) as spt_t;
                pdf_dev_set_rect(
                    &mut rect,
                    dvi_state.h + *xloc.offset(i as isize),
                    -dvi_state.v - *yloc.offset(i as isize),
                    glyph_width,
                    height,
                    depth,
                );
                pdf_doc_expand_box(&mut rect);
            }
        }
        wbuf[0] = (glyph_id >> 8i32) as u8;
        wbuf[1] = (glyph_id & 0xff_u32) as u8;
        pdf_dev_set_string(
            dvi_state.h + *xloc.offset(i as isize),
            -dvi_state.v - *yloc.offset(i as isize),
            wbuf.as_mut_ptr() as *const libc::c_void,
            2i32 as size_t,
            glyph_width,
            (*font).font_id,
            -1i32,
        );
        i = i.wrapping_add(1)
    }
    if (*font).rgba_color != 0xffffffffu32 {
        pdf_color_pop();
    }
    free(xloc as *mut libc::c_void);
    free(yloc as *mut libc::c_void);
    if do_actual_text != 0 {
        pdf_dev_end_actualtext();
    }
    if lr_mode == 0i32 {
        dvi_right(width);
    };
}
unsafe extern "C" fn check_postamble() {
    let mut code: i32 = 0;
    tt_skip_bytes(28_u32, dvi_handle);
    loop {
        code = tt_get_unsigned_byte(dvi_handle) as i32;
        if !(code != 249i32) {
            break;
        }
        match code {
            243 | 244 | 245 | 246 => {
                tt_skip_bytes((code + 1i32 - 243i32) as u32, dvi_handle);
                skip_fntdef();
            }
            252 => {
                tt_skip_bytes(4_u32, dvi_handle);
                skip_native_font_def();
            }
            _ => {
                panic!("Unexpected op code ({}) in postamble", code);
            }
        }
    }
    tt_skip_bytes(4_u32, dvi_handle);
    post_id_byte = tt_get_unsigned_byte(dvi_handle) as i32;
    if !(post_id_byte == 2i32
        || post_id_byte == 3i32
        || post_id_byte == 7i32
        || post_id_byte == 6i32)
    {
        info!("DVI ID = {}\n", post_id_byte);
        _tt_abort(invalid_signature.as_ptr());
    }
    check_id_bytes();
    if has_ptex != 0 && post_id_byte != 3i32 {
        panic!("Saw opcode {} in DVI file not for Ascii pTeX", 255i32,);
    }
    num_pages = 0_u32;
    /* force loop to terminate */
}
/* Most of the work of actually interpreting
 * the dvi file is here.
 */
#[no_mangle]
pub unsafe extern "C" fn dvi_do_page(
    mut page_paper_height: f64,
    mut hmargin: f64,
    mut vmargin: f64,
) {
    let mut opcode: u8 = 0;
    /* before this is called, we have scanned the page for papersize specials
    and the complete DVI data is now in dvi_page_buffer */
    dvi_page_buf_index = 0_u32;
    /* DVI coordinate */
    dev_origin_x = hmargin;
    dev_origin_y = page_paper_height - vmargin;
    dvi_stack_depth = 0i32;
    loop {
        opcode = get_buffered_unsigned_byte() as u8;
        if opcode as i32 <= 127i32 {
            dvi_set(opcode as i32);
        } else if opcode as i32 >= 171i32 && opcode as i32 <= 234i32 {
            do_fnt((opcode as i32 - 171i32) as u32);
        } else {
            let mut current_block_59: u64;
            match opcode as i32 {
                128 | 129 | 130 => {
                    dvi_set(get_buffered_unsigned_num((opcode as i32 - 128i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                131 => {
                    panic!("Multibyte (>24 bits) character not supported!");
                }
                132 => {
                    do_setrule();
                    current_block_59 = 6471821049853688503;
                }
                133 | 134 | 135 => {
                    dvi_put(get_buffered_unsigned_num((opcode as i32 - 133i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                136 => {
                    panic!("Multibyte (>24 bits) character not supported!");
                }
                137 => {
                    do_putrule();
                    current_block_59 = 6471821049853688503;
                }
                139 => {
                    do_bop();
                    current_block_59 = 6471821049853688503;
                }
                140 => {
                    do_eop();
                    if linear != 0 {
                        opcode = tt_get_unsigned_byte(dvi_handle);
                        if opcode as i32 == 248i32 {
                            check_postamble();
                        } else {
                            ttstub_input_ungetc(dvi_handle, opcode as i32);
                        }
                    }
                    return;
                }
                141 => {
                    dvi_push();
                    if lr_mode >= 2i32 {
                        lr_width_push();
                    }
                    /* If we are here, we have an opcode that is something
                     * other than SET_CHAR.
                     */
                    /* The following line needs to go here instead of in
                     * dvi_push() since logical structure of document is
                     * oblivous to virtual fonts. For example the last line on a
                     * page could be at stack level 3 and the page footer should
                     * be at stack level 3.  However, if the page footer contains
                     * virtual fonts (or other nested constructions), it could
                     * fool the link breaker into thinking it was a continuation
                     * of the link */
                    dvi_mark_depth();
                    current_block_59 = 6471821049853688503;
                }
                142 => {
                    dpx_dvi_pop();
                    if lr_mode >= 2i32 {
                        lr_width_pop();
                    }
                    /* Above explanation holds for following line too */
                    dvi_mark_depth();
                    current_block_59 = 6471821049853688503;
                }
                143 | 144 | 145 | 146 => {
                    dvi_right(get_buffered_signed_num((opcode as i32 - 143i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                147 => {
                    dvi_w0();
                    current_block_59 = 6471821049853688503;
                }
                148 | 149 | 150 | 151 => {
                    dvi_w(get_buffered_signed_num((opcode as i32 - 148i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                152 => {
                    dvi_x0();
                    current_block_59 = 6471821049853688503;
                }
                153 | 154 | 155 | 156 => {
                    dvi_x(get_buffered_signed_num((opcode as i32 - 153i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                157 | 158 | 159 | 160 => {
                    dvi_down(get_buffered_signed_num((opcode as i32 - 157i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                161 => {
                    dvi_y0();
                    current_block_59 = 6471821049853688503;
                }
                162 | 163 | 164 | 165 => {
                    dvi_y(get_buffered_signed_num((opcode as i32 - 162i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                166 => {
                    dvi_z0();
                    current_block_59 = 6471821049853688503;
                }
                167 | 168 | 169 | 170 => {
                    dvi_z(get_buffered_signed_num((opcode as i32 - 167i32) as u8));
                    current_block_59 = 6471821049853688503;
                }
                235 | 236 | 237 | 238 => {
                    do_fnt(get_buffered_unsigned_num((opcode as i32 - 235i32) as u8) as u32);
                    current_block_59 = 6471821049853688503;
                }
                239 | 240 | 241 | 242 => {
                    /* Specials */
                    let mut size: i32 = get_buffered_unsigned_num((opcode as i32 - 239i32) as u8);
                    if size < 0i32 {
                        warn!("DVI: Special with {} bytes???", size);
                    } else {
                        do_xxx(size);
                    }
                    current_block_59 = 6471821049853688503;
                }
                138 | 243 | 244 | 245 | 246 => {
                    current_block_59 = 6471821049853688503;
                }
                255 => {
                    /* pTeX extension */
                    need_pTeX(opcode as i32);
                    do_dir();
                    current_block_59 = 6471821049853688503;
                }
                253 => {
                    /* XeTeX extensions */
                    need_XeTeX(opcode as i32);
                    do_glyphs(0i32);
                    current_block_59 = 6471821049853688503;
                }
                254 => {
                    need_XeTeX(opcode as i32);
                    do_glyphs(1i32);
                    current_block_59 = 6471821049853688503;
                }
                252 => {
                    /* should not occur - processed during pre-scanning */
                    need_XeTeX(opcode as i32);
                    current_block_59 = 6471821049853688503;
                }
                250 => {
                    need_XeTeX(opcode as i32);
                    dvi_begin_reflect();
                    current_block_59 = 6471821049853688503;
                }
                251 => {
                    need_XeTeX(opcode as i32);
                    dvi_end_reflect();
                    current_block_59 = 6471821049853688503;
                }
                248 => {
                    if linear as i32 != 0 && processing_page == 0 {
                        /* for linear processing, this means there are no more pages */
                        num_pages = 0_u32; /* force loop to terminate */
                        return;
                    }
                    current_block_59 = 17253953464124104480;
                }
                247 | 249 => {
                    current_block_59 = 17253953464124104480;
                }
                _ => {
                    panic!("Unexpected opcode or DVI file ended prematurely");
                }
            }
            match current_block_59 {
                17253953464124104480 =>
                /* else fall through to error case */
                {
                    panic!("Unexpected preamble or postamble in dvi file");
                }
                _ =>
                    /* These should not occur - processed during pre-scanning */
                    {}
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dvi_init(mut dvi_filename: *const i8, mut mag: f64) -> f64 {
    let mut post_location: i32 = 0;
    if dvi_filename.is_null() {
        panic!("filename must be specified");
    }
    dvi_handle = ttstub_input_open(dvi_filename, TTInputFormat::BINARY, 0i32);
    if dvi_handle.is_null() {
        _tt_abort(
            b"cannot open \"%s\"\x00" as *const u8 as *const i8,
            dvi_filename,
        );
    }
    /* DVI files are most easily read backwards by searching for post_post and
     * then post opcode.
     */
    post_location = find_post();
    get_dvi_info(post_location);
    do_scales(mag);
    get_page_info(post_location);
    get_comment();
    get_dvi_fonts(post_location);
    clear_state();
    dvi_page_buf_size = 0x10000u32;
    dvi_page_buffer =
        new((dvi_page_buf_size as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
    dvi2pts
}
#[no_mangle]
pub unsafe extern "C" fn dvi_close() {
    let mut i: u32 = 0;
    if linear != 0 {
        /* probably reading a pipe from xetex; consume any remaining data */
        while ttstub_input_getc(dvi_handle) != -1i32 {}
    }
    /* We add comment in dvi_close instead of dvi_init so user
     * has a change to overwrite it.  The docinfo dictionary is
     * treated as a write-once record.
     */
    /* Do some house cleaning */
    ttstub_input_close(dvi_handle);
    dvi_handle = 0 as *mut libc::c_void;
    if !def_fonts.is_null() {
        i = 0_u32;
        while i < num_def_fonts {
            let ref mut fresh23 = (*def_fonts.offset(i as isize)).font_name;
            *fresh23 =
                mfree((*def_fonts.offset(i as isize)).font_name as *mut libc::c_void) as *mut i8;
            i = i.wrapping_add(1)
        }
        free(def_fonts as *mut libc::c_void);
    }
    def_fonts = 0 as *mut font_def;
    page_loc = mfree(page_loc as *mut libc::c_void) as *mut u32;
    num_pages = 0_u32;
    i = 0_u32;
    while i < num_loaded_fonts {
        free((*loaded_fonts.offset(i as isize)).hvmt as *mut libc::c_void);
        let ref mut fresh24 = (*loaded_fonts.offset(i as isize)).hvmt;
        *fresh24 = 0 as *mut tt_longMetrics;
        if !(*loaded_fonts.offset(i as isize)).cffont.is_null() {
            cff_close((*loaded_fonts.offset(i as isize)).cffont);
        }
        let ref mut fresh25 = (*loaded_fonts.offset(i as isize)).cffont;
        *fresh25 = 0 as *mut cff_font;
        i = i.wrapping_add(1)
    }
    loaded_fonts = mfree(loaded_fonts as *mut libc::c_void) as *mut loaded_font;
    num_loaded_fonts = 0_u32;
    vf_close_all_fonts();
    tfm_close_all();
    if !dvi_page_buffer.is_null() {
        dvi_page_buffer = mfree(dvi_page_buffer as *mut libc::c_void) as *mut u8;
        dvi_page_buf_size = 0_u32
    };
}
/* The following are need to implement virtual fonts
According to documentation, the vf "subroutine"
must have state pushed and must have
w,v,y, and z set to zero.  The current font
is determined by the virtual font header, which
may be undefined */
static mut saved_dvi_font: [i32; 16] = [0; 16];
static mut num_saved_fonts: u32 = 0_u32;
#[no_mangle]
pub unsafe extern "C" fn dvi_vf_init(mut dev_font_id: i32) {
    dvi_push();
    dvi_state.w = 0i32;
    dvi_state.x = 0i32;
    dvi_state.y = 0i32;
    dvi_state.z = 0i32;
    /* do not reset dvi_state.d. */
    if num_saved_fonts < 16u32 {
        let fresh26 = num_saved_fonts;
        num_saved_fonts = num_saved_fonts.wrapping_add(1);
        saved_dvi_font[fresh26 as usize] = current_font
    } else {
        panic!("Virtual fonts nested too deeply!");
    }
    current_font = dev_font_id;
}
/* After VF subroutine is finished, we simply pop the DVI stack */
#[no_mangle]
pub unsafe extern "C" fn dvi_vf_finish() {
    dpx_dvi_pop();
    if num_saved_fonts > 0_u32 {
        num_saved_fonts = num_saved_fonts.wrapping_sub(1);
        current_font = saved_dvi_font[num_saved_fonts as usize]
    } else {
        panic!("Tried to pop an empty font stack");
    };
}
/* Scan various specials */
/* This need to allow 'true' prefix for unit and
 * length value must be divided by current magnification.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn read_length(
    mut vp: *mut f64,
    mut mag: f64,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8; /* remember this for free, because q may be advanced */
    let mut p: *const i8 = *pp; /* inverse magnify */
    let mut v: f64 = 0.;
    let mut u: f64 = 1.0f64;
    let mut _ukeys: [*const i8; 10] = [
        b"pt\x00" as *const u8 as *const i8,
        b"in\x00" as *const u8 as *const i8,
        b"cm\x00" as *const u8 as *const i8,
        b"mm\x00" as *const u8 as *const i8,
        b"bp\x00" as *const u8 as *const i8,
        b"pc\x00" as *const u8 as *const i8,
        b"dd\x00" as *const u8 as *const i8,
        b"cc\x00" as *const u8 as *const i8,
        b"sp\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut k: i32 = 0;
    let mut error: i32 = 0i32;
    q = parse_float_decimal(&mut p, endptr);
    if q.is_null() {
        *vp = 0.0f64;
        *pp = p;
        return -1i32;
    }
    v = atof(q);
    free(q as *mut libc::c_void);
    skip_white(&mut p, endptr);
    q = parse_c_ident(&mut p, endptr);
    if !q.is_null() {
        let mut qq: *mut i8 = q;
        if strlen(q) >= strlen(b"true\x00" as *const u8 as *const i8)
            && memcmp(
                q as *const libc::c_void,
                b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"true\x00" as *const u8 as *const i8),
            ) == 0
        {
            u /= if mag != 0.0f64 { mag } else { 1.0f64 };
            q = q.offset(strlen(b"true\x00" as *const u8 as *const i8) as isize)
        }
        if strlen(q) == 0i32 as u64 {
            /* "true" was a separate word from the units */
            free(qq as *mut libc::c_void);
            skip_white(&mut p, endptr);
            q = parse_c_ident(&mut p, endptr);
            qq = q
        }
        if !q.is_null() {
            k = 0i32;
            while !_ukeys[k as usize].is_null() && strcmp(_ukeys[k as usize], q) != 0 {
                k += 1
            }
            match k {
                0 => u *= 72.0f64 / 72.27f64,
                1 => u *= 72.0f64,
                2 => u *= 72.0f64 / 2.54f64,
                3 => u *= 72.0f64 / 25.4f64,
                4 => u *= 1.0f64,
                5 => u *= 12.0f64 * 72.0f64 / 72.27f64,
                6 => u *= 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                7 => u *= 12.0f64 * 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                8 => u *= 72.0f64 / (72.27f64 * 65536i32 as f64),
                _ => {
                    dpx_warning(
                        b"Unknown unit of measure: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    error = -1i32
                }
            }
            free(qq as *mut libc::c_void);
        } else {
            warn!("Missing unit of measure after \"true\"");
            error = -1i32
        }
    }
    *vp = v * u;
    *pp = p;
    error
}
unsafe extern "C" fn scan_special(
    mut wd: *mut f64,
    mut ht: *mut f64,
    mut xo: *mut f64,
    mut yo: *mut f64,
    mut lm: *mut i32,
    mut majorversion: *mut i32,
    mut minorversion: *mut i32,
    mut do_enc: *mut i32,
    mut key_bits: *mut i32,
    mut permission: *mut i32,
    mut owner_pw: *mut i8,
    mut user_pw: *mut i8,
    mut buf: *const i8,
    mut size: u32,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = buf;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut ns_pdf: i32 = 0i32;
    let mut ns_dvipdfmx: i32 = 0i32;
    let mut error: i32 = 0i32;
    let mut tmp: f64 = 0.;
    endptr = p.offset(size as isize);
    skip_white(&mut p, endptr);
    q = parse_c_ident(&mut p, endptr);
    if streq_ptr(q, b"pdf\x00" as *const u8 as *const i8) {
        skip_white(&mut p, endptr);
        if p < endptr && *p as i32 == ':' as i32 {
            p = p.offset(1);
            skip_white(&mut p, endptr);
            free(q as *mut libc::c_void);
            q = parse_c_ident(&mut p, endptr);
            ns_pdf = 1i32
        }
    } else if streq_ptr(q, b"x\x00" as *const u8 as *const i8) {
        skip_white(&mut p, endptr);
        if p < endptr && *p as i32 == ':' as i32 {
            p = p.offset(1);
            skip_white(&mut p, endptr);
            free(q as *mut libc::c_void);
            q = parse_c_ident(&mut p, endptr)
        }
    } else if streq_ptr(q, b"dvipdfmx\x00" as *const u8 as *const i8) {
        skip_white(&mut p, endptr);
        if p < endptr && *p as i32 == ':' as i32 {
            p = p.offset(1);
            skip_white(&mut p, endptr);
            free(q as *mut libc::c_void);
            q = parse_c_ident(&mut p, endptr);
            ns_dvipdfmx = 1i32
        }
    }
    skip_white(&mut p, endptr);
    if !q.is_null() {
        if streq_ptr(q, b"landscape\x00" as *const u8 as *const i8) {
            *lm = 1i32
        } else if ns_pdf != 0 && streq_ptr(q, b"pagesize\x00" as *const u8 as *const i8) as i32 != 0
        {
            while error == 0 && p < endptr {
                let mut kp: *mut i8 = parse_c_ident(&mut p, endptr);
                if kp.is_null() {
                    break;
                }
                skip_white(&mut p, endptr);
                if streq_ptr(kp, b"width\x00" as *const u8 as *const i8) {
                    error = read_length(&mut tmp, dvi_tell_mag(), &mut p, endptr);
                    if error == 0 {
                        *wd = tmp * dvi_tell_mag()
                    }
                } else if streq_ptr(kp, b"height\x00" as *const u8 as *const i8) {
                    error = read_length(&mut tmp, dvi_tell_mag(), &mut p, endptr);
                    if error == 0 {
                        *ht = tmp * dvi_tell_mag()
                    }
                } else if streq_ptr(kp, b"xoffset\x00" as *const u8 as *const i8) {
                    error = read_length(&mut tmp, dvi_tell_mag(), &mut p, endptr);
                    if error == 0 {
                        *xo = tmp * dvi_tell_mag()
                    }
                } else if streq_ptr(kp, b"yoffset\x00" as *const u8 as *const i8) {
                    error = read_length(&mut tmp, dvi_tell_mag(), &mut p, endptr);
                    if error == 0 {
                        *yo = tmp * dvi_tell_mag()
                    }
                } else if streq_ptr(kp, b"default\x00" as *const u8 as *const i8) {
                    *wd = paper_width;
                    *ht = paper_height;
                    *lm = landscape_mode;
                    *yo = 72.0f64;
                    *xo = *yo
                }
                free(kp as *mut libc::c_void);
                skip_white(&mut p, endptr);
            }
        } else if streq_ptr(q, b"papersize\x00" as *const u8 as *const i8) {
            let mut qchr: i8 = 0_i8;
            if *p as i32 == '=' as i32 {
                p = p.offset(1)
            }
            skip_white(&mut p, endptr);
            if p < endptr && (*p as i32 == '\'' as i32 || *p as i32 == '\"' as i32) {
                qchr = *p;
                p = p.offset(1);
                skip_white(&mut p, endptr);
            }
            error = read_length(&mut tmp, 1.0f64, &mut p, endptr);
            if error == 0 {
                let mut tmp1: f64 = 0.;
                skip_white(&mut p, endptr);
                if p < endptr && *p as i32 == ',' as i32 {
                    p = p.offset(1);
                    skip_white(&mut p, endptr);
                }
                error = read_length(&mut tmp1, 1.0f64, &mut p, endptr);
                if error == 0 {
                    *wd = tmp;
                    *ht = tmp1;
                    skip_white(&mut p, endptr);
                }
            }
            if error == 0 && qchr as i32 != 0 {
                /* Check if properly quoted */
                if p >= endptr || *p as i32 != qchr as i32 {
                    error = -1i32
                }
            }
            if error == 0i32 {
                paper_width = *wd;
                paper_height = *ht
            }
        } else if !minorversion.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"minorversion\x00" as *const u8 as *const i8) as i32 != 0
        {
            let mut kv: *mut i8 = 0 as *mut i8;
            if *p as i32 == '=' as i32 {
                p = p.offset(1)
            }
            skip_white(&mut p, endptr);
            kv = parse_float_decimal(&mut p, endptr);
            if !kv.is_null() {
                *minorversion = strtol(kv, 0 as *mut *mut i8, 10i32) as i32;
                free(kv as *mut libc::c_void);
            }
        } else if !majorversion.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"majorversion\x00" as *const u8 as *const i8) as i32 != 0
        {
            let mut kv_0: *mut i8 = 0 as *mut i8;
            if *p as i32 == '=' as i32 {
                p = p.offset(1)
            }
            skip_white(&mut p, endptr);
            kv_0 = parse_float_decimal(&mut p, endptr);
            if !kv_0.is_null() {
                *majorversion = strtol(kv_0, 0 as *mut *mut i8, 10i32) as i32;
                free(kv_0 as *mut libc::c_void);
            }
        } else if ns_pdf != 0
            && streq_ptr(q, b"encrypt\x00" as *const u8 as *const i8) as i32 != 0
            && !do_enc.is_null()
        {
            *do_enc = 1i32;
            *user_pw = 0_i8;
            *owner_pw = *user_pw;
            while error == 0 && p < endptr {
                let mut kp_0: *mut i8 = parse_c_ident(&mut p, endptr);
                if kp_0.is_null() {
                    break;
                }
                let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
                skip_white(&mut p, endptr);
                if streq_ptr(kp_0, b"ownerpw\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_string(&mut p, endptr);
                    if !obj.is_null() {
                        if !pdf_string_value(obj).is_null() {
                            strncpy(owner_pw, pdf_string_value(obj) as *const i8, 127i32 as u64);
                        }
                        pdf_release_obj(obj);
                    } else {
                        error = -1i32
                    }
                } else if streq_ptr(kp_0, b"userpw\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_string(&mut p, endptr);
                    if !obj.is_null() {
                        if !pdf_string_value(obj).is_null() {
                            strncpy(user_pw, pdf_string_value(obj) as *const i8, 127i32 as u64);
                        }
                        pdf_release_obj(obj);
                    } else {
                        error = -1i32
                    }
                } else if streq_ptr(kp_0, b"length\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_number(&mut p, endptr);
                    if !obj.is_null()
                        && (!obj.is_null() && pdf_obj_typeof(obj) == PdfObjType::NUMBER)
                    {
                        *key_bits = pdf_number_value(obj) as u32 as i32
                    } else {
                        error = -1i32
                    }
                    pdf_release_obj(obj);
                } else if streq_ptr(kp_0, b"perm\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_number(&mut p, endptr);
                    if !obj.is_null()
                        && (!obj.is_null() && pdf_obj_typeof(obj) == PdfObjType::NUMBER)
                    {
                        *permission = pdf_number_value(obj) as u32 as i32
                    } else {
                        error = -1i32
                    }
                    pdf_release_obj(obj);
                } else {
                    error = -1i32
                }
                free(kp_0 as *mut libc::c_void);
                skip_white(&mut p, endptr);
            }
        } else if ns_dvipdfmx != 0
            && streq_ptr(q, b"config\x00" as *const u8 as *const i8) as i32 != 0
        {
            warn!("Tectonic does not support `config\' special. Ignored.");
        }
        free(q as *mut libc::c_void);
    }
    error
}
static mut buffered_page: i32 = -1i32;
/* returns scale (dvi2pts) */
/* may append .dvi or .xdv to filename */
/* Closes data structures created by dvi_open */
/* Renamed to avoid clash with XeTeX */
#[no_mangle]
pub unsafe extern "C" fn dvi_scan_specials(
    mut page_no: i32,
    mut page_width: *mut f64,
    mut page_height: *mut f64,
    mut x_offset: *mut f64,
    mut y_offset: *mut f64,
    mut landscape: *mut i32,
    mut majorversion: *mut i32,
    mut minorversion: *mut i32,
    mut do_enc: *mut i32,
    mut key_bits: *mut i32,
    mut permission: *mut i32,
    mut owner_pw: *mut i8,
    mut user_pw: *mut i8,
) {
    let mut offset: u32 = 0; /* because dvipdfmx wants to scan first page twice! */
    let mut opcode: u8 = 0;
    let mut len: u32 = 0;
    if page_no == buffered_page || num_pages == 0_u32 {
        return;
    }
    buffered_page = page_no;
    dvi_page_buf_index = 0_u32;
    if linear == 0 {
        if page_no as u32 >= num_pages {
            panic!("Invalid page number: {}", page_no);
        }
        offset = *page_loc.offset(page_no as isize);
        ttstub_input_seek(dvi_handle, offset as ssize_t, 0i32);
    }
    loop {
        opcode = get_and_buffer_unsigned_byte(dvi_handle) as u8;
        if !(opcode as i32 != 140i32) {
            break;
        }
        if opcode as i32 <= 127i32 || opcode as i32 >= 171i32 && opcode as i32 <= 234i32 {
            continue;
        }
        if opcode as i32 == 239i32
            || opcode as i32 == 240i32
            || opcode as i32 == 241i32
            || opcode as i32 == 242i32
        {
            let mut size: u32 = get_and_buffer_unsigned_byte(dvi_handle) as u32;
            let mut current_block_14: u64;
            match opcode as i32 {
                242 => {
                    size = size
                        .wrapping_mul(0x100u32)
                        .wrapping_add(get_and_buffer_unsigned_byte(dvi_handle) as u32);
                    if size > 0x7fff_u32 {
                        warn!(
                            "Unsigned number starting with {:x} exceeds 0x7fffffff",
                            size,
                        );
                    }
                    current_block_14 = 2922806634731202080;
                }
                241 => {
                    current_block_14 = 2922806634731202080;
                }
                240 => {
                    current_block_14 = 7135116673376365024;
                }
                _ => {
                    current_block_14 = 26972500619410423;
                }
            }
            match current_block_14 {
                2922806634731202080 => {
                    size = size
                        .wrapping_mul(0x100u32)
                        .wrapping_add(get_and_buffer_unsigned_byte(dvi_handle) as u32);
                    current_block_14 = 7135116673376365024;
                }
                _ => {}
            }
            match current_block_14 {
                7135116673376365024 => {
                    size = size
                        .wrapping_mul(0x100u32)
                        .wrapping_add(get_and_buffer_unsigned_byte(dvi_handle) as u32)
                }
                _ => {}
            }
            if dvi_page_buf_index.wrapping_add(size) >= dvi_page_buf_size {
                dvi_page_buf_size = dvi_page_buf_index
                    .wrapping_add(size)
                    .wrapping_add(0x10000u32);
                dvi_page_buffer = renew(
                    dvi_page_buffer as *mut libc::c_void,
                    (dvi_page_buf_size as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64)
                        as u32,
                ) as *mut u8
            }
            if ttstub_input_read(
                dvi_handle,
                dvi_page_buffer.offset(dvi_page_buf_index as isize) as *mut i8,
                size as size_t,
            ) != size as i64
            {
                panic!("Reading DVI file failed!");
            }
            if scan_special(
                page_width,
                page_height,
                x_offset,
                y_offset,
                landscape,
                majorversion,
                minorversion,
                do_enc,
                key_bits,
                permission,
                owner_pw,
                user_pw,
                dvi_page_buffer.offset(dvi_page_buf_index as isize) as *mut i8,
                size,
            ) != 0
            {
                dpx_warning(
                    b"Reading special command failed: \"%.*s\"\x00" as *const u8 as *const i8,
                    size,
                    dvi_page_buffer.offset(dvi_page_buf_index as isize) as *mut i8,
                );
            }
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(size)
        } else {
            let mut current_block_50: u64;
            /* Skipping... */
            match opcode as i32 {
                139 => {
                    get_and_buffer_bytes(dvi_handle, 44_u32); /* width */
                    current_block_50 = 6033931424626438518; /* glyph count */
                }
                138 | 141 | 142 | 147 | 152 | 161 | 166 => {
                    current_block_50 = 6033931424626438518; /* 2 bytes ID + 8 bytes x,y-location per glyph */
                }
                128 | 133 | 143 | 157 | 148 | 153 | 162 | 167 | 235 => {
                    get_and_buffer_bytes(dvi_handle, 1_u32); /* utf16 code unit count */
                    current_block_50 = 6033931424626438518; /* 2 bytes per code unit */
                }
                129 | 134 | 144 | 158 | 149 | 154 | 163 | 168 | 236 => {
                    get_and_buffer_bytes(dvi_handle, 2_u32); /* width */
                    current_block_50 = 6033931424626438518; /* glyph count */
                }
                130 | 135 | 145 | 159 | 150 | 155 | 164 | 169 | 237 => {
                    get_and_buffer_bytes(dvi_handle, 3_u32); /* 2 bytes ID + 8 bytes x,y-location per glyph */
                    current_block_50 = 6033931424626438518;
                }
                131 | 136 | 146 | 160 | 151 | 156 | 165 | 170 | 238 => {
                    get_and_buffer_bytes(dvi_handle, 4_u32);
                    current_block_50 = 6033931424626438518;
                }
                132 | 137 => {
                    get_and_buffer_bytes(dvi_handle, 8_u32);
                    current_block_50 = 6033931424626438518;
                }
                243 | 244 | 245 | 246 => {
                    do_fntdef(tt_get_unsigned_num(
                        dvi_handle,
                        (opcode as i32 - 243i32) as u8,
                    ));
                    current_block_50 = 6033931424626438518;
                }
                253 => {
                    need_XeTeX(opcode as i32);
                    get_and_buffer_bytes(dvi_handle, 4_u32);
                    len = get_and_buffer_unsigned_pair(dvi_handle);
                    get_and_buffer_bytes(dvi_handle, len.wrapping_mul(10_u32));
                    current_block_50 = 6033931424626438518;
                }
                254 => {
                    need_XeTeX(opcode as i32);
                    len = get_and_buffer_unsigned_pair(dvi_handle);
                    get_and_buffer_bytes(dvi_handle, len.wrapping_mul(2_u32));
                    get_and_buffer_bytes(dvi_handle, 4_u32);
                    len = get_and_buffer_unsigned_pair(dvi_handle);
                    get_and_buffer_bytes(dvi_handle, len.wrapping_mul(10_u32));
                    current_block_50 = 6033931424626438518;
                }
                252 => {
                    need_XeTeX(opcode as i32);
                    do_native_font_def(tt_get_signed_quad(dvi_handle));
                    current_block_50 = 6033931424626438518;
                }
                250 | 251 => {
                    need_XeTeX(opcode as i32);
                    current_block_50 = 6033931424626438518;
                }
                255 => {
                    need_pTeX(opcode as i32);
                    get_and_buffer_bytes(dvi_handle, 1_u32);
                    current_block_50 = 6033931424626438518;
                }
                248 => {
                    if linear as i32 != 0 && dvi_page_buf_index == 1_u32 {
                        /* this is actually an indication that we've reached the end of the input */
                        return;
                    }
                    current_block_50 = 1349400641705233371;
                }
                _ => {
                    current_block_50 = 1349400641705233371;
                }
            }
            match current_block_50 {
                1349400641705233371 =>
                /* else fall through to error case */
                /* case PRE: case POST_POST: and others */
                {
                    panic!("Unexpected opcode {}", opcode as i32);
                }
                _ => {}
            }
        }
    }
}
/* spt_t */
/* instantiated in dvipdfmx.c */
#[no_mangle]
pub unsafe extern "C" fn dvi_reset_global_state() {
    buffered_page = -1i32;
    num_def_fonts = 0_u32;
    max_def_fonts = 0_u32;
    compute_boxes = 0i32;
    link_annot = 1i32;
    verbose = 0i32;
    num_loaded_fonts = 0_u32;
    max_loaded_fonts = 0_u32;
}
