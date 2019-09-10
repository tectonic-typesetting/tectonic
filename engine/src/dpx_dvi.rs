#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64)
        -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> libc::c_int;
    #[no_mangle]
    fn atof(__nptr: *const i8) -> libc::c_double;
    #[no_mangle]
    fn strtol(_: *const i8, _: *mut *mut i8, _: libc::c_int) -> i64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
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
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut i8,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_ungetc(handle: rust_input_handle_t, ch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> libc::c_int;
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    /* Tectonic enabled */
    #[no_mangle]
    fn tt_skip_bytes(n: libc::c_uint, handle: rust_input_handle_t);
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn tt_get_signed_quad(handle: rust_input_handle_t) -> int32_t;
    #[no_mangle]
    fn tt_get_unsigned_num(handle: rust_input_handle_t, num: u8) -> u32;
    #[no_mangle]
    fn tt_get_positive_quad(
        handle: rust_input_handle_t,
        type_0: *const i8,
        name: *const i8,
    ) -> u32;
    #[no_mangle]
    fn sqxfw(sq: int32_t, fw: fixword) -> int32_t;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_number_value(number: *mut pdf_obj) -> libc::c_double;
    #[no_mangle]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_color_rgbcolor(
        color: *mut pdf_color,
        r: libc::c_double,
        g: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_push(sc: *mut pdf_color, fc: *mut pdf_color);
    #[no_mangle]
    fn pdf_color_pop();
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
        font_id: libc::c_int,
        ctype: libc::c_int,
    );
    #[no_mangle]
    fn pdf_dev_set_rule(xpos: spt_t, ypos: spt_t, width: spt_t, height: spt_t);
    /* The design_size and ptsize required by PK font support...
     */
    #[no_mangle]
    fn pdf_dev_locate_font(font_name: *const i8, ptsize: spt_t) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_set_dirmode(dir_mode: libc::c_int);
    /* Set rect to rectangle in device space.
     * Unit conversion spt_t to bp and transformation applied within it.
     */
    #[no_mangle]
    fn pdf_dev_set_rect(
        rect: *mut pdf_rect,
        x_pos: spt_t,
        y_pos: spt_t,
        width: spt_t,
        height: spt_t,
        depth: spt_t,
    );
    /* Text is normal and line art is not normal in dvipdfmx. So we don't have
     * begin_text (BT in PDF) and end_text (ET), but instead we have graphics_mode()
     * to terminate text section. pdf_dev_flushpath() and others call this.
     */
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    fn pdf_dev_begin_actualtext(unicodes: *mut u16, len: libc::c_int);
    #[no_mangle]
    fn pdf_dev_end_actualtext();
    #[no_mangle]
    static mut paper_width: libc::c_double;
    #[no_mangle]
    static mut paper_height: libc::c_double;
    #[no_mangle]
    static mut landscape_mode: libc::c_int;
    #[no_mangle]
    fn cff_close(cff: *mut cff_font);
    #[no_mangle]
    fn cff_dict_get(
        dict: *mut cff_dict,
        key: *const i8,
        idx: libc::c_int,
    ) -> libc::c_double;
    #[no_mangle]
    fn cff_dict_known(dict: *mut cff_dict, key: *const i8) -> libc::c_int;
    #[no_mangle]
    fn dpx_open_type1_file(filename: *const i8) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_truetype_file(filename: *const i8) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_opentype_file(filename: *const i8) -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_dfont_file(filename: *const i8) -> rust_input_handle_t;
    /*  DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
        the DVIPDFMx project team.

        Copyright (c) 2006 SIL. (xdvipdfmx extensions for XeTeX support)

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
    static mut is_xdv: libc::c_int;
    #[no_mangle]
    fn pdf_lookup_fontmap_record(kp: *const i8) -> *mut fontmap_rec;
    #[no_mangle]
    fn pdf_insert_native_fontmap_record(
        filename: *const i8,
        index: u32,
        layout_dir: libc::c_int,
        extend: libc::c_int,
        slant: libc::c_int,
        embolden: libc::c_int,
    ) -> *mut fontmap_rec;
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
    /* Page */
    #[no_mangle]
    fn pdf_doc_begin_page(
        scale: libc::c_double,
        x_origin: libc::c_double,
        y_origin: libc::c_double,
    );
    #[no_mangle]
    fn pdf_doc_end_page();
    #[no_mangle]
    fn pdf_doc_break_annot();
    #[no_mangle]
    fn pdf_doc_expand_box(rect: *const pdf_rect);
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
    /* Please remove this */
    #[no_mangle]
    fn dump(start: *const i8, end: *const i8);
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    #[no_mangle]
    fn parse_pdf_number(pp: *mut *const i8, endptr: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_string(pp: *mut *const i8, endptr: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[no_mangle]
    fn dfont_open(handle: rust_input_handle_t, index: libc::c_int) -> *mut sfnt;
    #[no_mangle]
    fn sfnt_close(sfont: *mut sfnt);
    /* table directory */
    #[no_mangle]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> libc::c_int;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    /* This should not use pdf_. */
    #[no_mangle]
    fn spc_set_verbose(level: libc::c_int);
    #[no_mangle]
    fn spc_exec_at_begin_page() -> libc::c_int;
    #[no_mangle]
    fn spc_exec_at_end_page() -> libc::c_int;
    #[no_mangle]
    fn spc_exec_special(
        p: *const i8,
        size: int32_t,
        x_user: libc::c_double,
        y_user: libc::c_double,
        mag: libc::c_double,
    ) -> libc::c_int;
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
    fn subfont_set_verbose(level: libc::c_int);
    #[no_mangle]
    fn lookup_sfd_record(rec_id: libc::c_int, code: u8) -> u16;
    #[no_mangle]
    fn sfd_load_record(
        sfd_name: *const i8,
        subfont_id: *const i8,
    ) -> libc::c_int;
    #[no_mangle]
    fn t1char_get_metrics(
        src: *mut card8,
        srclen: libc::c_int,
        subrs: *mut cff_index,
        ginfo: *mut t1_ginfo,
    ) -> libc::c_int;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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
    fn t1_load_font(
        enc_vec: *mut *mut i8,
        mode: libc::c_int,
        handle: rust_input_handle_t,
    ) -> *mut cff_font;
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
    fn tfm_set_verbose(level: libc::c_int);
    #[no_mangle]
    fn tfm_open(tex_name: *const i8, must_exist: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tfm_close_all();
    #[no_mangle]
    fn tfm_get_fw_width(font_id: libc::c_int, ch: int32_t) -> fixword;
    #[no_mangle]
    fn tfm_get_fw_height(font_id: libc::c_int, ch: int32_t) -> fixword;
    #[no_mangle]
    fn tfm_get_fw_depth(font_id: libc::c_int, ch: int32_t) -> fixword;
    /* TTC (TrueType Collection) */
    #[no_mangle]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: libc::c_int) -> SFNT_ULONG;
    #[no_mangle]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[no_mangle]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[no_mangle]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    /* vhea */
    #[no_mangle]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    /* hmtx and vmtx */
    #[no_mangle]
    fn tt_read_longMetrics(
        sfont: *mut sfnt,
        numGlyphs: USHORT,
        numLongMetrics: USHORT,
        numExSideBearings: USHORT,
    ) -> *mut tt_longMetrics;
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
    fn vf_set_verbose(level: libc::c_int);
    #[no_mangle]
    fn vf_locate_font(tex_name: *const i8, ptsize: spt_t) -> libc::c_int;
    #[no_mangle]
    fn vf_set_char(ch: int32_t, vf_font: libc::c_int);
    #[no_mangle]
    fn vf_close_all_fonts();
    #[no_mangle]
    fn parse_float_decimal(
        pp: *mut *const i8,
        endptr: *const i8,
    ) -> *mut i8;
    #[no_mangle]
    fn parse_c_ident(
        pp: *mut *const i8,
        endptr: *const i8,
    ) -> *mut i8;
}
use crate::*;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type fixword = int32_t;
/* quasi-hack to get the primary input */
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
pub struct pdf_color {
    pub num_components: libc::c_int,
    pub spot_color_name: *mut i8,
    pub values: [libc::c_double; 4],
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
pub type spt_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
/*
 * The section below this line deals with the actual processing of the
 * dvi file.
 *
 * The dvi file processor state is contained in the following variables:
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_registers {
    pub h: int32_t,
    pub v: int32_t,
    pub w: int32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub z: int32_t,
    pub d: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_def {
    pub tex_id: u32,
    pub point_size: spt_t,
    pub design_size: spt_t,
    pub font_name: *mut i8,
    pub font_id: libc::c_int,
    pub used: libc::c_int,
    pub native: libc::c_int,
    pub rgba_color: u32,
    pub face_index: u32,
    pub layout_dir: libc::c_int,
    pub extend: libc::c_int,
    pub slant: libc::c_int,
    pub embolden: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loaded_font {
    pub type_0: libc::c_int,
    pub font_id: libc::c_int,
    pub subfont_id: libc::c_int,
    pub tfm_id: libc::c_int,
    pub size: spt_t,
    pub source: libc::c_int,
    pub rgba_color: u32,
    pub hvmt: *mut tt_longMetrics,
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
    pub unitsPerEm: libc::c_uint,
    pub cffont: *mut cff_font,
    pub numGlyphs: libc::c_uint,
    pub layout_dir: libc::c_int,
    pub extend: libc::c_float,
    pub slant: libc::c_float,
    pub embolden: libc::c_float,
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
    pub filter: libc::c_int,
    pub index: libc::c_int,
    pub flag: libc::c_int,
    pub is_notdef_notzero: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
pub type card8 = u8;
pub type l_offset = u32;
pub type c_offsize = u8;
pub type card16 = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: libc::c_int,
    pub count: libc::c_int,
    pub entries: *mut cff_dict_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: libc::c_int,
    pub key: *const i8,
    pub count: libc::c_int,
    pub values: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
pub type s_SID = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed_1,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
}
/* hmtx and vmtx */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: USHORT,
    pub sideBearing: SHORT,
}
pub type SHORT = libc::c_short;
pub type USHORT = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut i8,
    pub font_name: *mut i8,
    pub enc_name: *mut i8,
    pub charmap: C2RustUnnamed_2,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: libc::c_double,
    pub extend: libc::c_double,
    pub bold: libc::c_double,
    pub mapc: libc::c_int,
    pub flags: libc::c_int,
    pub otl_tags: *mut i8,
    pub tounicode: *mut i8,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: libc::c_double,
    pub charcoll: *mut i8,
    pub index: libc::c_int,
    pub style: libc::c_int,
    pub stemv: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub sfd_name: *mut i8,
    pub subfont_id: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_header {
    pub unit_num: u32,
    pub unit_den: u32,
    pub mag: u32,
    pub media_width: u32,
    pub media_height: u32,
    pub stackdepth: libc::c_uint,
    pub comment: [i8; 257],
}
/* skimming through reflected segment measuring its width */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_lr {
    pub state: libc::c_int,
    pub font: libc::c_int,
    pub buf_index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_ginfo {
    pub use_seac: libc::c_int,
    pub wx: libc::c_double,
    pub wy: libc::c_double,
    pub bbox: C2RustUnnamed_3,
    pub seac: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub asb: libc::c_double,
    pub adx: libc::c_double,
    pub ady: libc::c_double,
    pub bchar: card8,
    pub achar: card8,
}
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
    /* table data */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_head_table {
    pub version: Fixed,
    pub fontRevision: Fixed,
    pub checkSumAdjustment: SFNT_ULONG,
    pub magicNumber: SFNT_ULONG,
    pub flags: USHORT,
    pub unitsPerEm: USHORT,
    pub created: [BYTE; 8],
    pub modified: [BYTE; 8],
    pub xMin: FWord,
    pub yMin: FWord,
    pub xMax: FWord,
    pub yMax: FWord,
    pub macStyle: USHORT,
    pub lowestRecPPEM: USHORT,
    pub fontDirectionHint: SHORT,
    pub indexToLocFormat: SHORT,
    pub glyphDataFormat: SHORT,
}
/* 16.16-bit signed fixed-point number */
pub type FWord = libc::c_short;
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
pub type BYTE = u8;
pub type Fixed = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_maxp_table {
    pub version: Fixed,
    pub numGlyphs: USHORT,
    pub maxPoints: USHORT,
    pub maxContours: USHORT,
    pub maxComponentPoints: USHORT,
    pub maxComponentContours: USHORT,
    pub maxZones: USHORT,
    pub maxTwilightPoints: USHORT,
    pub maxStorage: USHORT,
    pub maxFunctionDefs: USHORT,
    pub maxInstructionDefs: USHORT,
    pub maxStackElements: USHORT,
    pub maxSizeOfInstructions: USHORT,
    pub maxComponentElements: USHORT,
    pub maxComponentDepth: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_hhea_table {
    pub version: Fixed,
    pub ascent: FWord,
    pub descent: FWord,
    pub lineGap: FWord,
    pub advanceWidthMax: uFWord,
    pub minLeftSideBearing: FWord,
    pub minRightSideBearing: FWord,
    pub xMaxExtent: FWord,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: FWord,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongHorMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
    /* extra information */
}
pub type uFWord = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vhea_table {
    pub version: Fixed,
    pub vertTypoAscender: SHORT,
    pub vertTypoDescender: SHORT,
    pub vertTypoLineGap: SHORT,
    pub advanceHeightMax: SHORT,
    pub minTopSideBearing: SHORT,
    pub minBottomSideBearing: SHORT,
    pub yMaxExtent: SHORT,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: SHORT,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongVerMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
    /* extra information */
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
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* UTF-32 over U+FFFF -> UTF-16 surrogate pair */
/* Interal Variables */
static mut dvi_handle: rust_input_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut linear: i8 = 0i32 as i8;
/* set to 1 for strict linear processing of the input */
static mut page_loc: *mut u32 = 0 as *const u32 as *mut u32;
static mut num_pages: libc::c_uint = 0i32 as libc::c_uint;
static mut dvi_file_size: u32 = 0i32 as u32;
static mut dvi_info: dvi_header = {
    let mut init = dvi_header {
        unit_num: 25400000i32 as u32,
        unit_den: 473628672i32 as u32,
        mag: 1000i32 as u32,
        media_width: 0i32 as u32,
        media_height: 0i32 as u32,
        stackdepth: 0i32 as libc::c_uint,
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
static mut dev_origin_x: libc::c_double = 72.0f64;
static mut dev_origin_y: libc::c_double = 770.0f64;
#[no_mangle]
pub unsafe extern "C" fn get_origin(mut x: libc::c_int) -> libc::c_double {
    return if x != 0 { dev_origin_x } else { dev_origin_y };
}
static mut lr_state: dvi_lr = dvi_lr {
    state: 0,
    font: 0,
    buf_index: 0,
};
/* state at start of current skimming  */
static mut lr_mode: libc::c_int = 0;
/* current direction or skimming depth */
static mut lr_width: u32 = 0;
/* total width of reflected segment    */
static mut lr_width_stack: [u32; 256] = [0; 256];
static mut lr_width_stack_depth: libc::c_uint = 0i32 as libc::c_uint;
static mut loaded_fonts: *mut loaded_font = 0 as *const loaded_font as *mut loaded_font;
static mut num_loaded_fonts: libc::c_uint = 0i32 as libc::c_uint;
static mut max_loaded_fonts: libc::c_uint = 0i32 as libc::c_uint;
unsafe extern "C" fn need_more_fonts(mut n: libc::c_uint) {
    if num_loaded_fonts.wrapping_add(n) > max_loaded_fonts {
        max_loaded_fonts = max_loaded_fonts.wrapping_add(16u32);
        loaded_fonts = renew(
            loaded_fonts as *mut libc::c_void,
            (max_loaded_fonts as u64)
                .wrapping_mul(::std::mem::size_of::<loaded_font>() as u64)
                as u32,
        ) as *mut loaded_font
    };
}
static mut def_fonts: *mut font_def = 0 as *const font_def as *mut font_def;
static mut num_def_fonts: libc::c_uint = 0i32 as libc::c_uint;
static mut max_def_fonts: libc::c_uint = 0i32 as libc::c_uint;
static mut compute_boxes: libc::c_int = 0i32;
static mut link_annot: libc::c_int = 1i32;
static mut verbose: libc::c_int = 0i32;
/* 64K should be plenty for most pages */
static mut dvi_page_buffer: *mut u8 = 0 as *const u8 as *mut u8;
static mut dvi_page_buf_size: libc::c_uint = 0;
static mut dvi_page_buf_index: libc::c_uint = 0;
/* functions to read numbers from the dvi file and store them in dvi_page_buffer */
unsafe extern "C" fn get_and_buffer_unsigned_byte(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    ch = ttstub_input_getc(handle);
    if ch < 0i32 {
        _tt_abort(b"File ended prematurely\n\x00" as *const u8 as *const i8);
    }
    if dvi_page_buf_index >= dvi_page_buf_size {
        dvi_page_buf_size = dvi_page_buf_size.wrapping_add(0x10000u32);
        dvi_page_buffer = renew(
            dvi_page_buffer as *mut libc::c_void,
            (dvi_page_buf_size as u64)
                .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                as u32,
        ) as *mut u8
    }
    let fresh0 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    *dvi_page_buffer.offset(fresh0 as isize) = ch as u8;
    return ch;
}
unsafe extern "C" fn get_and_buffer_unsigned_pair(mut handle: rust_input_handle_t) -> libc::c_uint {
    let mut pair: libc::c_uint = get_and_buffer_unsigned_byte(handle) as libc::c_uint;
    pair = pair << 8i32 | get_and_buffer_unsigned_byte(handle) as libc::c_uint;
    return pair;
}
unsafe extern "C" fn get_and_buffer_bytes(
    mut handle: rust_input_handle_t,
    mut count: libc::c_uint,
) {
    if dvi_page_buf_index.wrapping_add(count) >= dvi_page_buf_size {
        dvi_page_buf_size = dvi_page_buf_index
            .wrapping_add(count)
            .wrapping_add(0x10000u32);
        dvi_page_buffer = renew(
            dvi_page_buffer as *mut libc::c_void,
            (dvi_page_buf_size as u64)
                .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                as u32,
        ) as *mut u8
    }
    if ttstub_input_read(
        handle,
        (dvi_page_buffer as *mut i8).offset(dvi_page_buf_index as isize),
        count as size_t,
    ) != count as i64
    {
        _tt_abort(b"File ended prematurely\n\x00" as *const u8 as *const i8);
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(count);
}
/* functions to fetch values from dvi_page_buffer */
unsafe extern "C" fn get_buffered_unsigned_byte() -> libc::c_int {
    let fresh1 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    return *dvi_page_buffer.offset(fresh1 as isize) as libc::c_int;
}
unsafe extern "C" fn get_buffered_unsigned_pair() -> libc::c_uint {
    let fresh2 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut pair: libc::c_uint = *dvi_page_buffer.offset(fresh2 as isize) as libc::c_uint;
    let fresh3 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    pair = pair << 8i32 | *dvi_page_buffer.offset(fresh3 as isize) as libc::c_uint;
    return pair;
}
unsafe extern "C" fn get_buffered_signed_quad() -> int32_t {
    let mut i: libc::c_uint = 0;
    let fresh4 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut quad: int32_t = *dvi_page_buffer.offset(fresh4 as isize) as int32_t;
    /* Check sign on first byte before reading others */
    if quad >= 0x80i32 {
        quad -= 0x100i32
    }
    i = 0i32 as libc::c_uint;
    while i < 3i32 as libc::c_uint {
        let fresh5 = dvi_page_buf_index;
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
        quad = quad << 8i32 | *dvi_page_buffer.offset(fresh5 as isize) as libc::c_int;
        i = i.wrapping_add(1)
    }
    return quad;
}
unsafe extern "C" fn get_buffered_signed_num(mut num: u8) -> int32_t {
    let fresh6 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut quad: int32_t = *dvi_page_buffer.offset(fresh6 as isize) as int32_t;
    if quad > 0x7fi32 {
        quad -= 0x100i32
    }
    let mut current_block_4: u64;
    match num as libc::c_int {
        3 => {
            let fresh7 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh7 as isize) as libc::c_int;
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
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh8 as isize) as libc::c_int;
            current_block_4 = 7573734397012025053;
        }
        _ => {}
    }
    match current_block_4 {
        7573734397012025053 => {
            let fresh9 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh9 as isize) as libc::c_int
        }
        _ => {}
    }
    return quad;
}
unsafe extern "C" fn get_buffered_unsigned_num(mut num: u8) -> int32_t {
    let fresh10 = dvi_page_buf_index;
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
    let mut quad: int32_t = *dvi_page_buffer.offset(fresh10 as isize) as int32_t;
    let mut current_block_4: u64;
    match num as libc::c_int {
        3 => {
            if quad > 0x7fi32 {
                quad -= 0x100i32
            }
            let fresh11 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh11 as isize) as libc::c_int;
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
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh12 as isize) as libc::c_int;
            current_block_4 = 10786445149178684808;
        }
        _ => {}
    }
    match current_block_4 {
        10786445149178684808 => {
            let fresh13 = dvi_page_buf_index;
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8i32 | *dvi_page_buffer.offset(fresh13 as isize) as libc::c_int
        }
        _ => {}
    }
    return quad;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_set_verbose(mut level: libc::c_int) {
    verbose = level;
    subfont_set_verbose(level);
    tfm_set_verbose(level);
    vf_set_verbose(level);
    spc_set_verbose(level);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_npages() -> libc::c_uint {
    return num_pages;
}
static mut invalid_signature: [i8; 53] = [
    83, 111, 109, 101, 116, 104, 105, 110, 103, 32, 105, 115, 32, 119, 114, 111, 110, 103, 46, 32,
    65, 114, 101, 32, 121, 111, 117, 32, 115, 117, 114, 101, 32, 116, 104, 105, 115, 32, 105, 115,
    32, 97, 32, 68, 86, 73, 32, 102, 105, 108, 101, 63, 0,
];
static mut pre_id_byte: libc::c_int = 0;
static mut post_id_byte: libc::c_int = 0;
static mut is_ptex: libc::c_int = 0i32;
static mut has_ptex: libc::c_int = 0i32;
unsafe extern "C" fn check_id_bytes() {
    if pre_id_byte != post_id_byte && (pre_id_byte != 2i32 || post_id_byte != 3i32) {
        _tt_abort(
            b"Inconsistent DVI id_bytes %d (pre) and %d (post)\x00" as *const u8
                as *const i8,
            pre_id_byte,
            post_id_byte,
        );
    };
}
unsafe extern "C" fn need_XeTeX(mut c: libc::c_int) {
    if is_xdv == 0 {
        _tt_abort(
            b"DVI opcode %i only valid for XeTeX\x00" as *const u8 as *const i8,
            c,
        );
    };
}
unsafe extern "C" fn need_pTeX(mut c: libc::c_int) {
    if is_ptex == 0 {
        _tt_abort(
            b"DVI opcode %i only valid for Ascii pTeX\x00" as *const u8 as *const i8,
            c,
        );
    }
    has_ptex = 1i32;
}
unsafe extern "C" fn find_post() -> int32_t {
    let mut dvi_size: off_t = 0;
    let mut current: int32_t = 0;
    let mut ch: libc::c_int = 0;
    dvi_size = ttstub_input_get_size(dvi_handle) as off_t;
    if dvi_size > 0x7fffffffi32 as i64 {
        _tt_abort(b"DVI file size exceeds 31-bit\x00" as *const u8 as *const i8);
    }
    dvi_file_size = dvi_size as u32;
    ttstub_input_seek(dvi_handle, 0i32 as ssize_t, 2i32);
    current = dvi_size as int32_t;
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
    if dvi_file_size.wrapping_sub(current as libc::c_uint) < 4i32 as libc::c_uint
        || current == 0i32
        || !(ch == 2i32 || ch == 3i32 || ch == 7i32 || ch == 6i32)
    {
        dpx_message(b"DVI ID = %d\n\x00" as *const u8 as *const i8, ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    post_id_byte = ch;
    is_xdv = (ch == 7i32 || ch == 6i32) as libc::c_int;
    is_ptex = (ch == 3i32) as libc::c_int;
    /* Make sure post_post is really there */
    current = current - 5i32;
    ttstub_input_seek(dvi_handle, current as ssize_t, 0i32);
    ch = ttstub_input_getc(dvi_handle);
    if ch != 249i32 {
        dpx_message(
            b"Found %d where post_post opcode should be\n\x00" as *const u8 as *const i8,
            ch,
        );
        _tt_abort(invalid_signature.as_ptr());
    }
    current = tt_get_signed_quad(dvi_handle);
    ttstub_input_seek(dvi_handle, current as ssize_t, 0i32);
    ch = ttstub_input_getc(dvi_handle);
    if ch != 248i32 {
        dpx_message(
            b"Found %d where post_post opcode should be\n\x00" as *const u8 as *const i8,
            ch,
        );
        _tt_abort(invalid_signature.as_ptr());
    }
    /* Finally check the ID byte in the preamble */
    /* An Ascii pTeX DVI file has id_byte DVI_ID in the preamble but DVIV_ID in the postamble. */
    ttstub_input_seek(dvi_handle, 0i32 as ssize_t, 0i32);
    ch = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    if ch != 247i32 {
        dpx_message(
            b"Found %d where PRE was expected\n\x00" as *const u8 as *const i8,
            ch,
        );
        _tt_abort(invalid_signature.as_ptr());
    }
    ch = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    if !(ch == 2i32 || ch == 7i32 || ch == 6i32) {
        dpx_message(b"DVI ID = %d\n\x00" as *const u8 as *const i8, ch);
        _tt_abort(invalid_signature.as_ptr());
    }
    pre_id_byte = ch;
    check_id_bytes();
    return current;
}
unsafe extern "C" fn get_page_info(mut post_location: int32_t) {
    let mut i: libc::c_int = 0;
    ttstub_input_seek(dvi_handle, (post_location + 27i32) as ssize_t, 0i32);
    num_pages = tt_get_unsigned_pair(dvi_handle) as libc::c_uint;
    if num_pages == 0i32 as libc::c_uint {
        _tt_abort(b"Page count is 0!\x00" as *const u8 as *const i8);
    }
    if verbose > 2i32 {
        dpx_message(
            b"Page count:\t %4d\n\x00" as *const u8 as *const i8,
            num_pages,
        );
    }
    page_loc = new((num_pages as u64)
        .wrapping_mul(::std::mem::size_of::<u32>() as u64)
        as u32) as *mut u32;
    ttstub_input_seek(dvi_handle, (post_location + 1i32) as ssize_t, 0i32);
    *page_loc.offset(num_pages.wrapping_sub(1i32 as libc::c_uint) as isize) =
        tt_get_unsigned_quad(dvi_handle);
    if (*page_loc.offset(num_pages.wrapping_sub(1i32 as libc::c_uint) as isize))
        .wrapping_add(41i32 as libc::c_uint)
        > dvi_file_size
    {
        _tt_abort(invalid_signature.as_ptr());
    }
    i = num_pages.wrapping_sub(2i32 as libc::c_uint) as libc::c_int;
    while i >= 0i32 {
        ttstub_input_seek(
            dvi_handle,
            (*page_loc.offset((i + 1i32) as isize)).wrapping_add(41i32 as libc::c_uint) as ssize_t,
            0i32,
        );
        *page_loc.offset(i as isize) = tt_get_unsigned_quad(dvi_handle);
        if (*page_loc.offset(num_pages.wrapping_sub(1i32 as libc::c_uint) as isize))
            .wrapping_add(41i32 as libc::c_uint)
            > dvi_file_size
        {
            _tt_abort(invalid_signature.as_ptr());
        }
        i -= 1
    }
}
/* Following are computed "constants" used for unit conversion */
static mut dvi2pts: libc::c_double = 1.52018f64;
static mut total_mag: libc::c_double = 1.0f64;
#[no_mangle]
pub unsafe extern "C" fn dvi_tell_mag() -> libc::c_double {
    return total_mag; /* unused */
}
unsafe extern "C" fn do_scales(mut mag: libc::c_double) {
    total_mag = dvi_info.mag as libc::c_double / 1000.0f64 * mag; /* 1.0 */
    dvi2pts = dvi_info.unit_num as libc::c_double / dvi_info.unit_den as libc::c_double; /* font name length */
    dvi2pts *= 72.0f64 / 254000.0f64; /* hard-code as 10pt for now, not used anyway */
}
unsafe extern "C" fn get_dvi_info(mut post_location: int32_t) {
    ttstub_input_seek(dvi_handle, (post_location + 5i32) as ssize_t, 0i32); /* direction */
    dvi_info.unit_num = tt_get_unsigned_quad(dvi_handle);
    dvi_info.unit_den = tt_get_unsigned_quad(dvi_handle);
    dvi_info.mag = tt_get_unsigned_quad(dvi_handle);
    dvi_info.media_height = tt_get_unsigned_quad(dvi_handle);
    dvi_info.media_width = tt_get_unsigned_quad(dvi_handle);
    dvi_info.stackdepth = tt_get_unsigned_pair(dvi_handle) as libc::c_uint;
    if dvi_info.stackdepth > 256u32 {
        dpx_warning(
            b"DVI need stack depth of %d,\x00" as *const u8 as *const i8,
            dvi_info.stackdepth,
        );
        dpx_warning(
            b"but DVI_STACK_DEPTH_MAX is %d.\x00" as *const u8 as *const i8,
            256u32,
        );
        _tt_abort(b"Capacity exceeded.\x00" as *const u8 as *const i8);
    }
    if verbose > 2i32 {
        dpx_message(b"DVI File Info\n\x00" as *const u8 as *const i8);
        dpx_message(
            b"Unit: %u / %u\n\x00" as *const u8 as *const i8,
            dvi_info.unit_num,
            dvi_info.unit_den,
        );
        dpx_message(
            b"Magnification: %u\n\x00" as *const u8 as *const i8,
            dvi_info.mag,
        );
        dpx_message(
            b"Media Height: %u\n\x00" as *const u8 as *const i8,
            dvi_info.media_height,
        );
        dpx_message(
            b"Media Width: %u\n\x00" as *const u8 as *const i8,
            dvi_info.media_width,
        );
        dpx_message(
            b"Stack Depth: %u\n\x00" as *const u8 as *const i8,
            dvi_info.stackdepth,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_comment() -> *const i8 {
    return dvi_info.comment.as_mut_ptr();
}
unsafe extern "C" fn read_font_record(mut tex_id: u32) {
    let mut dir_length: libc::c_int = 0;
    let mut name_length: libc::c_int = 0;
    let mut point_size: u32 = 0;
    let mut design_size: u32 = 0;
    let mut directory: *mut i8 = 0 as *mut i8;
    let mut font_name: *mut i8 = 0 as *mut i8;
    if num_def_fonts >= max_def_fonts {
        max_def_fonts = max_def_fonts.wrapping_add(16u32);
        def_fonts = renew(
            def_fonts as *mut libc::c_void,
            (max_def_fonts as u64)
                .wrapping_mul(::std::mem::size_of::<font_def>() as u64)
                as u32,
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
    dir_length = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    name_length = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    directory = new(((dir_length + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    if ttstub_input_read(dvi_handle, directory, dir_length as size_t) != dir_length as i64
    {
        _tt_abort(invalid_signature.as_ptr());
    }
    *directory.offset(dir_length as isize) = '\u{0}' as i32 as i8;
    free(directory as *mut libc::c_void);
    font_name = new(((name_length + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    if ttstub_input_read(dvi_handle, font_name, name_length as size_t)
        != name_length as i64
    {
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
    (*def_fonts.offset(num_def_fonts as isize)).face_index = 0i32 as u32;
    (*def_fonts.offset(num_def_fonts as isize)).layout_dir = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).extend = 0x10000i32;
    (*def_fonts.offset(num_def_fonts as isize)).slant = 0i32;
    (*def_fonts.offset(num_def_fonts as isize)).embolden = 0i32;
    num_def_fonts = num_def_fonts.wrapping_add(1);
}
unsafe extern "C" fn read_native_font_record(mut tex_id: u32) {
    let mut flags: libc::c_uint = 0;
    let mut point_size: u32 = 0;
    let mut font_name: *mut i8 = 0 as *mut i8;
    let mut len: libc::c_int = 0;
    let mut index: u32 = 0;
    if num_def_fonts >= max_def_fonts {
        max_def_fonts = max_def_fonts.wrapping_add(16u32);
        def_fonts = renew(
            def_fonts as *mut libc::c_void,
            (max_def_fonts as u64)
                .wrapping_mul(::std::mem::size_of::<font_def>() as u64)
                as u32,
        ) as *mut font_def
    }
    point_size = tt_get_positive_quad(
        dvi_handle,
        b"DVI\x00" as *const u8 as *const i8,
        b"point_size\x00" as *const u8 as *const i8,
    );
    flags = tt_get_unsigned_pair(dvi_handle) as libc::c_uint;
    len = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    font_name = new(((len + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
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
    if flags & 0x100i32 as libc::c_uint != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).layout_dir = 1i32
    }
    if flags & 0x200i32 as libc::c_uint != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).rgba_color = tt_get_unsigned_quad(dvi_handle)
    }
    if flags & 0x1000i32 as libc::c_uint != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).extend = tt_get_signed_quad(dvi_handle)
    }
    if flags & 0x2000i32 as libc::c_uint != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).slant = tt_get_signed_quad(dvi_handle)
    }
    if flags & 0x4000i32 as libc::c_uint != 0 {
        (*def_fonts.offset(num_def_fonts as isize)).embolden = tt_get_signed_quad(dvi_handle)
    }
    num_def_fonts = num_def_fonts.wrapping_add(1);
}
unsafe extern "C" fn get_dvi_fonts(mut post_location: int32_t) {
    let mut code: libc::c_int = 0;
    ttstub_input_seek(dvi_handle, (post_location + 29i32) as ssize_t, 0i32);
    loop {
        code = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
        if !(code != 249i32) {
            break;
        }
        match code {
            243 | 244 | 245 | 246 => {
                read_font_record(tt_get_unsigned_num(
                    dvi_handle,
                    (code - 243i32) as u8,
                ));
            }
            252 => {
                need_XeTeX(code);
                read_native_font_record(tt_get_signed_quad(dvi_handle) as u32);
            }
            _ => {
                dpx_message(
                    b"Unexpected op code: %3d\n\x00" as *const u8 as *const i8,
                    code,
                );
                _tt_abort(invalid_signature.as_ptr());
            }
        }
    }
    if verbose > 2i32 {
        let mut i: libc::c_uint = 0;
        dpx_message(b"\n\x00" as *const u8 as *const i8);
        dpx_message(b"DVI file font info\n\x00" as *const u8 as *const i8);
        i = 0i32 as libc::c_uint;
        while i < num_def_fonts {
            dpx_message(
                b"TeX Font: %10s loaded at ID=%5d, \x00" as *const u8 as *const i8,
                (*def_fonts.offset(i as isize)).font_name,
                (*def_fonts.offset(i as isize)).tex_id,
            );
            dpx_message(
                b"size=%5.2fpt (scaled %4.1f%%)\x00" as *const u8 as *const i8,
                (*def_fonts.offset(i as isize)).point_size as libc::c_double * dvi2pts,
                100.0f64
                    * ((*def_fonts.offset(i as isize)).point_size as libc::c_double
                        / (*def_fonts.offset(i as isize)).design_size as libc::c_double),
            );
            dpx_message(b"\n\x00" as *const u8 as *const i8);
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn get_comment() {
    let mut length: libc::c_int = 0;
    ttstub_input_seek(dvi_handle, 14i32 as ssize_t, 0i32);
    length = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
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
static mut dvi_stack_depth: libc::c_int = 0i32;
static mut current_font: libc::c_int = -1i32;
static mut processing_page: libc::c_int = 0i32;
unsafe extern "C" fn clear_state() {
    dvi_state.h = 0i32;
    dvi_state.v = 0i32;
    dvi_state.w = 0i32;
    dvi_state.x = 0i32;
    dvi_state.y = 0i32;
    dvi_state.z = 0i32;
    dvi_state.d = 0i32 as libc::c_uint;
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
static mut marked_depth: libc::c_int = 0i32;
static mut tagged_depth: libc::c_int = -1i32;
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
pub unsafe extern "C" fn dvi_compute_boxes(mut flag: libc::c_int) {
    compute_boxes = flag;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_link_annot(mut flag: libc::c_int) {
    link_annot = flag;
}
/* allow other modules (pdfdev) to ask whether we're collecting box areas */
#[no_mangle]
pub unsafe extern "C" fn dvi_is_tracking_boxes() -> bool {
    return compute_boxes != 0 && link_annot != 0 && marked_depth >= tagged_depth;
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
pub unsafe extern "C" fn dvi_do_special(mut buffer: *const libc::c_void, mut size: int32_t) {
    let mut x_user: libc::c_double = 0.; /* VF or device font ID */
    let mut y_user: libc::c_double = 0.;
    let mut mag: libc::c_double = 0.;
    let mut p: *const i8 = 0 as *const i8;
    graphics_mode();
    p = buffer as *const i8;
    x_user = dvi_state.h as libc::c_double * dvi2pts;
    y_user = -dvi_state.v as libc::c_double * dvi2pts;
    mag = dvi_tell_mag();
    if spc_exec_special(p, size, x_user, y_user, mag) < 0i32 {
        if verbose != 0 {
            dump(p, p.offset(size as isize));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dvi_unit_size() -> libc::c_double {
    return dvi2pts;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_locate_font(
    mut tfm_name: *const i8,
    mut ptsize: spt_t,
) -> libc::c_uint {
    let mut cur_id: libc::c_uint = 0;
    let mut name: *const i8 = tfm_name;
    let mut subfont_id: libc::c_int = -1i32;
    let mut font_id: libc::c_int = 0;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    if verbose != 0 {
        dpx_message(
            b"<%s@%.2fpt\x00" as *const u8 as *const i8,
            tfm_name,
            ptsize as libc::c_double * dvi2pts,
        );
    }
    need_more_fonts(1i32 as libc::c_uint);
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
                dpx_message(b"(VF)>\x00" as *const u8 as *const i8);
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
                    dpx_message(b"(OVF)>\x00" as *const u8 as *const i8);
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
                b">> This font is mapped to a physical font \"%s\".\x00" as *const u8
                    as *const i8,
                (*mrec).font_name,
            );
            dpx_warning(
                b">> Please check if kpathsea library can find this font: %s\x00" as *const u8
                    as *const i8,
                (*mrec).font_name,
            );
        } else {
            dpx_warning(
                b">> There are no valid font mapping entry for this font.\x00" as *const u8
                    as *const i8,
            );
            dpx_warning(
                b">> Font file name \"%s\" was assumed but failed to locate that font.\x00"
                    as *const u8 as *const i8,
                tfm_name,
            );
        }
        _tt_abort(
            b"Cannot proceed without .vf or \"physical\" font for PDF output...\x00" as *const u8
                as *const i8,
        );
    }
    (*loaded_fonts.offset(cur_id as isize)).type_0 = 1i32;
    (*loaded_fonts.offset(cur_id as isize)).font_id = font_id;
    if verbose != 0 {
        dpx_message(b">\x00" as *const u8 as *const i8);
    }
    return cur_id;
}
unsafe extern "C" fn dvi_locate_native_font(
    mut filename: *const i8,
    mut index: u32,
    mut ptsize: spt_t,
    mut layout_dir: libc::c_int,
    mut extend: libc::c_int,
    mut slant: libc::c_int,
    mut embolden: libc::c_int,
) -> libc::c_int {
    let mut cur_id: libc::c_int = -1i32;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut fontmap_key: *mut i8 = 0 as *mut i8;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut is_dfont: libc::c_int = 0i32;
    let mut is_type1: libc::c_int = 0i32;
    if verbose != 0 {
        dpx_message(
            b"<%s@%.2fpt\x00" as *const u8 as *const i8,
            filename,
            ptsize as libc::c_double * dvi2pts,
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
    need_more_fonts(1i32 as libc::c_uint);
    let fresh17 = num_loaded_fonts;
    num_loaded_fonts = num_loaded_fonts.wrapping_add(1);
    cur_id = fresh17 as libc::c_int;
    fontmap_key =
        xmalloc(strlen(filename).wrapping_add(40i32 as u64)) as *mut i8;
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
                b"Failed to insert font record for font: %s\x00" as *const u8
                    as *const i8,
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
        dpx_warning(
            b"skipping PFB sanity check -- needs Tectonic I/O update\x00" as *const u8
                as *const i8,
        );
        memset(
            enc_vec.as_mut_ptr() as *mut libc::c_void,
            0i32,
            (256i32 as u64)
                .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
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
        if cff_dict_known(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const i8,
        ) != 0
        {
            (*loaded_fonts.offset(cur_id as isize)).ascent = cff_dict_get(
                (*cffont).topdict,
                b"FontBBox\x00" as *const u8 as *const i8,
                3i32,
            ) as libc::c_int;
            (*loaded_fonts.offset(cur_id as isize)).descent = cff_dict_get(
                (*cffont).topdict,
                b"FontBBox\x00" as *const u8 as *const i8,
                1i32,
            ) as libc::c_int
        } else {
            (*loaded_fonts.offset(cur_id as isize)).ascent = 690i32;
            (*loaded_fonts.offset(cur_id as isize)).descent = -190i32
        }
        (*loaded_fonts.offset(cur_id as isize)).unitsPerEm = 1000i32 as libc::c_uint;
        (*loaded_fonts.offset(cur_id as isize)).numGlyphs = (*cffont).num_glyphs as libc::c_uint;
        ttstub_input_close(handle);
    } else {
        if is_dfont != 0 {
            sfont = dfont_open(handle, index as libc::c_int)
        } else {
            sfont = sfnt_open(handle)
        }
        if (*sfont).type_0 == 1i32 << 4i32 {
            offset = ttc_read_offset(sfont, index as libc::c_int)
        } else if (*sfont).type_0 == 1i32 << 8i32 {
            offset = (*sfont).offset
        }
        sfnt_read_table_directory(sfont, offset);
        head = tt_read_head_table(sfont);
        maxp = tt_read_maxp_table(sfont);
        hhea = tt_read_hhea_table(sfont);
        (*loaded_fonts.offset(cur_id as isize)).ascent = (*hhea).ascent as libc::c_int;
        (*loaded_fonts.offset(cur_id as isize)).descent = (*hhea).descent as libc::c_int;
        (*loaded_fonts.offset(cur_id as isize)).unitsPerEm = (*head).unitsPerEm as libc::c_uint;
        (*loaded_fonts.offset(cur_id as isize)).numGlyphs = (*maxp).numGlyphs as libc::c_uint;
        if layout_dir == 1i32
            && sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8)
                > 0i32 as libc::c_uint
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
    (*loaded_fonts.offset(cur_id as isize)).extend = (*mrec).opt.extend as libc::c_float;
    (*loaded_fonts.offset(cur_id as isize)).slant = (*mrec).opt.slant as libc::c_float;
    (*loaded_fonts.offset(cur_id as isize)).embolden = (*mrec).opt.bold as libc::c_float;
    if verbose != 0 {
        dpx_message(b">\x00" as *const u8 as *const i8);
    }
    return cur_id;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_dev_xpos() -> libc::c_double {
    return dvi_state.h as libc::c_double * dvi2pts;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_dev_ypos() -> libc::c_double {
    return -(dvi_state.v as libc::c_double * dvi2pts);
}
unsafe extern "C" fn do_moveto(mut x: int32_t, mut y: int32_t) {
    dvi_state.h = x;
    dvi_state.v = y;
}
/* FIXME: dvi_forward() might be a better name */
#[no_mangle]
pub unsafe extern "C" fn dvi_right(mut x: int32_t) {
    if lr_mode >= 2i32 {
        lr_width =
            (lr_width as libc::c_uint).wrapping_add(x as libc::c_uint) as u32;
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
pub unsafe extern "C" fn dvi_down(mut y: int32_t) {
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
pub unsafe extern "C" fn dvi_set(mut ch: int32_t) {
    let mut font: *mut loaded_font = 0 as *mut loaded_font;
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut wbuf: [u8; 4] = [0; 4];
    if current_font < 0i32 {
        _tt_abort(b"No font selected!\x00" as *const u8 as *const i8);
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
        lr_width =
            (lr_width as libc::c_uint).wrapping_add(width as libc::c_uint) as u32;
        return;
    }
    if lr_mode == 1i32 {
        dvi_right(width);
    }
    match (*font).type_0 {
        1 => {
            if ch > 65535i32 {
                /* _FIXME_ */
                wbuf[0] = (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) >> 8i32 & 0xffi32)
                    as u8;
                wbuf[1] =
                    (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) & 0xffi32) as u8;
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
                let mut uch: u16 =
                    lookup_sfd_record((*font).subfont_id, ch as u8);
                wbuf[0] = (uch as libc::c_int >> 8i32 & 0xffi32) as u8;
                wbuf[1] = (uch as libc::c_int & 0xffi32) as u8;
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
                let mut rect: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
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
pub unsafe extern "C" fn dvi_put(mut ch: int32_t) {
    let mut font: *mut loaded_font = 0 as *mut loaded_font;
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut wbuf: [u8; 4] = [0; 4];
    if current_font < 0i32 {
        _tt_abort(b"No font selected!\x00" as *const u8 as *const i8);
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
                wbuf[0] = (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) >> 8i32 & 0xffi32)
                    as u8;
                wbuf[1] =
                    (0xd800i32 + (ch - 0x10000i32 >> 10i32 & 0x3ffi32) & 0xffi32) as u8;
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
                let mut uch: libc::c_uint = 0;
                uch = lookup_sfd_record((*font).subfont_id, ch as u8) as libc::c_uint;
                wbuf[0] = (uch >> 8i32 & 0xffi32 as libc::c_uint) as u8;
                wbuf[1] = (uch & 0xffi32 as libc::c_uint) as u8;
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
                let mut rect: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
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
pub unsafe extern "C" fn dvi_rule(mut width: int32_t, mut height: int32_t) {
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
        fprintf(
            stderr,
            b"  > dvi_dirchg %d\n\x00" as *const u8 as *const i8,
            dir as libc::c_int,
        );
    }
    dvi_state.d = dir as libc::c_uint;
    pdf_dev_set_dirmode(dvi_state.d as libc::c_int);
    /* 0: horizontal, 1,3: vertical */
}
unsafe extern "C" fn do_setrule() {
    let mut width: int32_t = 0;
    let mut height: int32_t = 0;
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
        _ => {
            lr_width = (lr_width as libc::c_uint).wrapping_add(width as libc::c_uint) as u32
                as u32
        }
    };
}
unsafe extern "C" fn do_putrule() {
    let mut width: int32_t = 0;
    let mut height: int32_t = 0;
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
    if dvi_stack_depth as libc::c_uint >= 256u32 {
        _tt_abort(b"DVI stack exceeded limit.\x00" as *const u8 as *const i8);
    }
    let fresh21 = dvi_stack_depth;
    dvi_stack_depth = dvi_stack_depth + 1;
    dvi_stack[fresh21 as usize] = dvi_state;
}
#[no_mangle]
pub unsafe extern "C" fn dpx_dvi_pop() {
    if dvi_stack_depth <= 0i32 {
        _tt_abort(b"Tried to pop an empty stack.\x00" as *const u8 as *const i8);
    }
    dvi_stack_depth -= 1;
    dvi_state = dvi_stack[dvi_stack_depth as usize];
    do_moveto(dvi_state.h, dvi_state.v);
    pdf_dev_set_dirmode(dvi_state.d as libc::c_int);
    /* 0: horizontal, 1,3: vertical */
}
#[no_mangle]
pub unsafe extern "C" fn dvi_w(mut ch: int32_t) {
    dvi_state.w = ch;
    dvi_right(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_w0() {
    dvi_right(dvi_state.w);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_x(mut ch: int32_t) {
    dvi_state.x = ch;
    dvi_right(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_x0() {
    dvi_right(dvi_state.x);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_y(mut ch: int32_t) {
    dvi_state.y = ch;
    dvi_down(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_y0() {
    dvi_down(dvi_state.y);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_z(mut ch: int32_t) {
    dvi_state.z = ch;
    dvi_down(ch);
}
#[no_mangle]
pub unsafe extern "C" fn dvi_z0() {
    dvi_down(dvi_state.z);
}
unsafe extern "C" fn skip_fntdef() {
    let mut area_len: libc::c_int = 0;
    let mut name_len: libc::c_int = 0;
    tt_skip_bytes(12i32 as libc::c_uint, dvi_handle);
    area_len = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    name_len = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    tt_skip_bytes((area_len + name_len) as libc::c_uint, dvi_handle);
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
pub unsafe extern "C" fn dvi_set_font(mut font_id: libc::c_int) {
    current_font = font_id;
}
unsafe extern "C" fn do_fnt(mut tex_id: u32) {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < num_def_fonts {
        if (*def_fonts.offset(i as isize)).tex_id == tex_id {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == num_def_fonts {
        _tt_abort(
            b"Tried to select a font that hasn\'t been defined: id=%d\x00" as *const u8
                as *const i8,
            tex_id,
        );
    }
    if (*def_fonts.offset(i as isize)).used == 0 {
        let mut font_id: libc::c_uint = 0;
        if (*def_fonts.offset(i as isize)).native != 0 {
            font_id = dvi_locate_native_font(
                (*def_fonts.offset(i as isize)).font_name,
                (*def_fonts.offset(i as isize)).face_index,
                (*def_fonts.offset(i as isize)).point_size,
                (*def_fonts.offset(i as isize)).layout_dir,
                (*def_fonts.offset(i as isize)).extend,
                (*def_fonts.offset(i as isize)).slant,
                (*def_fonts.offset(i as isize)).embolden,
            ) as libc::c_uint
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
        (*def_fonts.offset(i as isize)).font_id = font_id as libc::c_int
    }
    current_font = (*def_fonts.offset(i as isize)).font_id;
}
unsafe extern "C" fn do_xxx(mut size: int32_t) {
    if lr_mode < 2i32 {
        dvi_do_special(
            dvi_page_buffer.offset(dvi_page_buf_index as isize) as *const libc::c_void,
            size,
        );
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(size as libc::c_uint);
}
unsafe extern "C" fn do_bop() {
    let mut i: libc::c_uint = 0;
    if processing_page != 0 {
        _tt_abort(b"Got a bop in the middle of a page!\x00" as *const u8 as *const i8);
    }
    /* For now, ignore TeX's count registers */
    i = 0i32 as libc::c_uint;
    while i < 10i32 as libc::c_uint {
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4i32 as libc::c_uint);
        i = i.wrapping_add(1)
    }
    /* Ignore previous page pointer since we have already
     * saved this information
     */
    dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4i32 as libc::c_uint);
    clear_state();
    processing_page = 1i32;
    pdf_doc_begin_page(dvi_tell_mag(), dev_origin_x, dev_origin_y);
    spc_exec_at_begin_page();
}
unsafe extern "C" fn do_eop() {
    processing_page = 0i32;
    if dvi_stack_depth != 0i32 {
        _tt_abort(
            b"DVI stack depth is not zero at end of page\x00" as *const u8 as *const i8,
        );
    }
    spc_exec_at_end_page();
    pdf_doc_end_page();
}
unsafe extern "C" fn do_dir() {
    dvi_state.d = get_buffered_unsigned_byte() as libc::c_uint;
    pdf_dev_set_dirmode(dvi_state.d as libc::c_int);
    /* 0: horizontal, 1,3: vertical */
}
unsafe extern "C" fn lr_width_push() {
    if lr_width_stack_depth >= 256u32 {
        _tt_abort(b"Segment width stack exceeded limit.\x00" as *const u8 as *const i8);
        /* must precede dvi_right */
    }
    let fresh22 = lr_width_stack_depth;
    lr_width_stack_depth = lr_width_stack_depth.wrapping_add(1);
    lr_width_stack[fresh22 as usize] = lr_width;
}
unsafe extern "C" fn lr_width_pop() {
    if lr_width_stack_depth <= 0i32 as libc::c_uint {
        _tt_abort(
            b"Tried to pop an empty segment width stack.\x00" as *const u8 as *const i8,
        );
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
        lr_width = 0i32 as u32
    };
}
unsafe extern "C" fn dvi_end_reflect() {
    match lr_mode {
        2 => {
            current_font = lr_state.font;
            dvi_page_buf_index = lr_state.buf_index;
            lr_mode = 0i32 + 1i32 - lr_state.state;
            dvi_right(-(lr_width as int32_t));
            lr_width_push();
        }
        0 | 1 => {
            lr_width_pop();
            dvi_right(-(lr_width as int32_t));
            lr_mode = 0i32 + 1i32 - lr_mode
        }
        _ => {
            /* lr_mode > SKIMMING */
            lr_mode -= 1
        }
    }; /* skip point size */
}
unsafe extern "C" fn skip_native_font_def() {
    let mut flags: libc::c_uint = 0;
    let mut name_length: libc::c_int = 0;
    tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
    flags = tt_get_unsigned_pair(dvi_handle) as libc::c_uint;
    name_length = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    tt_skip_bytes((name_length + 4i32) as libc::c_uint, dvi_handle);
    if flags & 0x200i32 as libc::c_uint != 0 {
        tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
    }
    if flags & 0x1000i32 as libc::c_uint != 0 {
        tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
    }
    if flags & 0x2000i32 as libc::c_uint != 0 {
        tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
    }
    if flags & 0x4000i32 as libc::c_uint != 0 {
        tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
    };
}
unsafe extern "C" fn do_native_font_def(mut tex_id: int32_t) {
    if linear != 0 {
        read_native_font_record(tex_id as u32);
    } else {
        skip_native_font_def();
    }
    dvi_page_buf_index = dvi_page_buf_index.wrapping_sub(1);
    /* don't buffer the opcode */
}
unsafe extern "C" fn skip_glyphs() {
    let mut i: libc::c_uint = 0; /* Will actually move left */
    let mut slen: libc::c_uint = 0i32 as libc::c_uint; /* freetype glyph index */
    slen = get_buffered_unsigned_pair();
    i = 0i32 as libc::c_uint;
    while i < slen {
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4i32 as libc::c_uint);
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(4i32 as libc::c_uint);
        dvi_page_buf_index = dvi_page_buf_index.wrapping_add(2i32 as libc::c_uint);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn do_glyphs(mut do_actual_text: libc::c_int) {
    let mut font: *mut loaded_font = 0 as *mut loaded_font;
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut xloc: *mut spt_t = 0 as *mut spt_t;
    let mut yloc: *mut spt_t = 0 as *mut spt_t;
    let mut glyph_width: spt_t = 0i32;
    let mut wbuf: [u8; 2] = [0; 2];
    let mut i: libc::c_uint = 0;
    let mut glyph_id: libc::c_uint = 0;
    let mut slen: libc::c_uint = 0i32 as libc::c_uint;
    if current_font < 0i32 {
        _tt_abort(b"No font selected!\x00" as *const u8 as *const i8);
    }
    font = &mut *loaded_fonts.offset(current_font as isize) as *mut loaded_font;
    if do_actual_text != 0 {
        slen = get_buffered_unsigned_pair();
        if lr_mode >= 2i32 {
            i = 0i32 as libc::c_uint;
            while i < slen {
                dvi_page_buf_index = dvi_page_buf_index.wrapping_add(2i32 as libc::c_uint);
                i = i.wrapping_add(1)
            }
        } else {
            let mut unicodes: *mut u16 = new((slen as u64)
                .wrapping_mul(::std::mem::size_of::<u16>() as u64)
                as u32) as *mut u16;
            i = 0i32 as libc::c_uint;
            while i < slen {
                *unicodes.offset(i as isize) = get_buffered_unsigned_pair() as u16;
                i = i.wrapping_add(1)
            }
            pdf_dev_begin_actualtext(unicodes, slen as libc::c_int);
            free(unicodes as *mut libc::c_void);
        }
    }
    width = get_buffered_signed_quad();
    if lr_mode >= 2i32 {
        lr_width =
            (lr_width as libc::c_uint).wrapping_add(width as libc::c_uint) as u32;
        skip_glyphs();
        return;
    }
    if lr_mode == 1i32 {
        dvi_right(width);
    }
    slen = get_buffered_unsigned_pair();
    xloc = new(
        (slen as u64).wrapping_mul(::std::mem::size_of::<spt_t>() as u64)
            as u32,
    ) as *mut spt_t;
    yloc = new(
        (slen as u64).wrapping_mul(::std::mem::size_of::<spt_t>() as u64)
            as u32,
    ) as *mut spt_t;
    i = 0i32 as libc::c_uint;
    while i < slen {
        *xloc.offset(i as isize) = get_buffered_signed_quad();
        *yloc.offset(i as isize) = get_buffered_signed_quad();
        i = i.wrapping_add(1)
    }
    if (*font).rgba_color != 0xffffffffu32 {
        let mut color: pdf_color = pdf_color {
            num_components: 0,
            spot_color_name: 0 as *mut i8,
            values: [0.; 4],
        };
        pdf_color_rgbcolor(
            &mut color,
            (((*font).rgba_color >> 24i32) as u8 as libc::c_int & 0xffi32)
                as libc::c_double
                / 255i32 as libc::c_double,
            (((*font).rgba_color >> 16i32) as u8 as libc::c_int & 0xffi32)
                as libc::c_double
                / 255i32 as libc::c_double,
            (((*font).rgba_color >> 8i32) as u8 as libc::c_int & 0xffi32)
                as libc::c_double
                / 255i32 as libc::c_double,
        );
        pdf_color_push(&mut color, &mut color);
    }
    i = 0i32 as libc::c_uint;
    while i < slen {
        glyph_id = get_buffered_unsigned_pair();
        if glyph_id < (*font).numGlyphs {
            let mut advance: libc::c_uint = 0;
            let mut ascent: libc::c_double = (*font).ascent as libc::c_double;
            let mut descent: libc::c_double = (*font).descent as libc::c_double;
            if !(*font).cffont.is_null() {
                let mut cstrings: *mut cff_index = (*(*font).cffont).cstrings;
                let mut gm: t1_ginfo = t1_ginfo {
                    use_seac: 0,
                    wx: 0.,
                    wy: 0.,
                    bbox: C2RustUnnamed_3 {
                        llx: 0.,
                        lly: 0.,
                        urx: 0.,
                        ury: 0.,
                    },
                    seac: C2RustUnnamed_4 {
                        asb: 0.,
                        adx: 0.,
                        ady: 0.,
                        bchar: 0,
                        achar: 0,
                    },
                };
                /* If .notdef is not the 1st glyph in CharStrings, glyph_id given by
                FreeType should be increased by 1 */
                if (*(*font).cffont).is_notdef_notzero != 0 {
                    glyph_id = glyph_id.wrapping_add(1i32 as libc::c_uint)
                }
                t1char_get_metrics(
                    (*cstrings)
                        .data
                        .offset(*(*cstrings).offset.offset(glyph_id as isize) as isize)
                        .offset(-1),
                    (*(*cstrings)
                        .offset
                        .offset(glyph_id.wrapping_add(1i32 as libc::c_uint) as isize))
                    .wrapping_sub(*(*cstrings).offset.offset(glyph_id as isize))
                        as libc::c_int,
                    *(*(*font).cffont).subrs.offset(0),
                    &mut gm,
                );
                advance = (if (*font).layout_dir == 0i32 {
                    gm.wx
                } else {
                    gm.wy
                }) as libc::c_uint;
                ascent = gm.bbox.ury;
                descent = gm.bbox.lly
            } else {
                advance = (*(*font).hvmt.offset(glyph_id as isize)).advance as libc::c_uint
            }
            glyph_width = ((*font).size as libc::c_double * advance as libc::c_double
                / (*font).unitsPerEm as libc::c_double) as spt_t;
            glyph_width = (glyph_width as libc::c_float * (*font).extend) as spt_t;
            if dvi_is_tracking_boxes() {
                let mut rect: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
                height = ((*font).size as libc::c_double * ascent
                    / (*font).unitsPerEm as libc::c_double) as spt_t;
                depth = ((*font).size as libc::c_double * -descent
                    / (*font).unitsPerEm as libc::c_double) as spt_t;
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
        wbuf[1] = (glyph_id & 0xffi32 as libc::c_uint) as u8;
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
    let mut code: libc::c_int = 0;
    tt_skip_bytes(28i32 as libc::c_uint, dvi_handle);
    loop {
        code = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
        if !(code != 249i32) {
            break;
        }
        match code {
            243 | 244 | 245 | 246 => {
                tt_skip_bytes((code + 1i32 - 243i32) as libc::c_uint, dvi_handle);
                skip_fntdef();
            }
            252 => {
                tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
                skip_native_font_def();
            }
            _ => {
                _tt_abort(
                    b"Unexpected op code (%u) in postamble\x00" as *const u8 as *const i8,
                    code,
                );
            }
        }
    }
    tt_skip_bytes(4i32 as libc::c_uint, dvi_handle);
    post_id_byte = tt_get_unsigned_byte(dvi_handle) as libc::c_int;
    if !(post_id_byte == 2i32
        || post_id_byte == 3i32
        || post_id_byte == 7i32
        || post_id_byte == 6i32)
    {
        dpx_message(
            b"DVI ID = %d\n\x00" as *const u8 as *const i8,
            post_id_byte,
        );
        _tt_abort(invalid_signature.as_ptr());
    }
    check_id_bytes();
    if has_ptex != 0 && post_id_byte != 3i32 {
        _tt_abort(
            b"Saw opcode %i in DVI file not for Ascii pTeX\x00" as *const u8 as *const i8,
            255i32,
        );
    }
    num_pages = 0i32 as libc::c_uint;
    /* force loop to terminate */
}
/* Most of the work of actually interpreting
 * the dvi file is here.
 */
#[no_mangle]
pub unsafe extern "C" fn dvi_do_page(
    mut page_paper_height: libc::c_double,
    mut hmargin: libc::c_double,
    mut vmargin: libc::c_double,
) {
    let mut opcode: u8 = 0;
    /* before this is called, we have scanned the page for papersize specials
    and the complete DVI data is now in dvi_page_buffer */
    dvi_page_buf_index = 0i32 as libc::c_uint;
    /* DVI coordinate */
    dev_origin_x = hmargin;
    dev_origin_y = page_paper_height - vmargin;
    dvi_stack_depth = 0i32;
    loop {
        opcode = get_buffered_unsigned_byte() as u8;
        if opcode as libc::c_int <= 127i32 {
            dvi_set(opcode as int32_t);
        } else if opcode as libc::c_int >= 171i32 && opcode as libc::c_int <= 234i32 {
            do_fnt((opcode as libc::c_int - 171i32) as u32);
        } else {
            let mut current_block_59: u64;
            match opcode as libc::c_int {
                128 | 129 | 130 => {
                    dvi_set(get_buffered_unsigned_num(
                        (opcode as libc::c_int - 128i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                131 => {
                    _tt_abort(
                        b"Multibyte (>24 bits) character not supported!\x00" as *const u8
                            as *const i8,
                    );
                }
                132 => {
                    do_setrule();
                    current_block_59 = 6471821049853688503;
                }
                133 | 134 | 135 => {
                    dvi_put(get_buffered_unsigned_num(
                        (opcode as libc::c_int - 133i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                136 => {
                    _tt_abort(
                        b"Multibyte (>24 bits) character not supported!\x00" as *const u8
                            as *const i8,
                    );
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
                        if opcode as libc::c_int == 248i32 {
                            check_postamble();
                        } else {
                            ttstub_input_ungetc(dvi_handle, opcode as libc::c_int);
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
                    dvi_right(get_buffered_signed_num(
                        (opcode as libc::c_int - 143i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                147 => {
                    dvi_w0();
                    current_block_59 = 6471821049853688503;
                }
                148 | 149 | 150 | 151 => {
                    dvi_w(get_buffered_signed_num(
                        (opcode as libc::c_int - 148i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                152 => {
                    dvi_x0();
                    current_block_59 = 6471821049853688503;
                }
                153 | 154 | 155 | 156 => {
                    dvi_x(get_buffered_signed_num(
                        (opcode as libc::c_int - 153i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                157 | 158 | 159 | 160 => {
                    dvi_down(get_buffered_signed_num(
                        (opcode as libc::c_int - 157i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                161 => {
                    dvi_y0();
                    current_block_59 = 6471821049853688503;
                }
                162 | 163 | 164 | 165 => {
                    dvi_y(get_buffered_signed_num(
                        (opcode as libc::c_int - 162i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                166 => {
                    dvi_z0();
                    current_block_59 = 6471821049853688503;
                }
                167 | 168 | 169 | 170 => {
                    dvi_z(get_buffered_signed_num(
                        (opcode as libc::c_int - 167i32) as u8,
                    ));
                    current_block_59 = 6471821049853688503;
                }
                235 | 236 | 237 | 238 => {
                    do_fnt(get_buffered_unsigned_num(
                        (opcode as libc::c_int - 235i32) as u8,
                    ) as u32);
                    current_block_59 = 6471821049853688503;
                }
                239 | 240 | 241 | 242 => {
                    /* Specials */
                    let mut size: int32_t = get_buffered_unsigned_num(
                        (opcode as libc::c_int - 239i32) as u8,
                    );
                    if size < 0i32 {
                        dpx_warning(
                            b"DVI: Special with %d bytes???\x00" as *const u8
                                as *const i8,
                            size,
                        );
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
                    need_pTeX(opcode as libc::c_int);
                    do_dir();
                    current_block_59 = 6471821049853688503;
                }
                253 => {
                    /* XeTeX extensions */
                    need_XeTeX(opcode as libc::c_int);
                    do_glyphs(0i32);
                    current_block_59 = 6471821049853688503;
                }
                254 => {
                    need_XeTeX(opcode as libc::c_int);
                    do_glyphs(1i32);
                    current_block_59 = 6471821049853688503;
                }
                252 => {
                    /* should not occur - processed during pre-scanning */
                    need_XeTeX(opcode as libc::c_int);
                    current_block_59 = 6471821049853688503;
                }
                250 => {
                    need_XeTeX(opcode as libc::c_int);
                    dvi_begin_reflect();
                    current_block_59 = 6471821049853688503;
                }
                251 => {
                    need_XeTeX(opcode as libc::c_int);
                    dvi_end_reflect();
                    current_block_59 = 6471821049853688503;
                }
                248 => {
                    if linear as libc::c_int != 0 && processing_page == 0 {
                        /* for linear processing, this means there are no more pages */
                        num_pages = 0i32 as libc::c_uint; /* force loop to terminate */
                        return;
                    }
                    current_block_59 = 17253953464124104480;
                }
                247 | 249 => {
                    current_block_59 = 17253953464124104480;
                }
                _ => {
                    _tt_abort(
                        b"Unexpected opcode or DVI file ended prematurely\x00" as *const u8
                            as *const i8,
                    );
                }
            }
            match current_block_59 {
                17253953464124104480 =>
                /* else fall through to error case */
                {
                    _tt_abort(
                        b"Unexpected preamble or postamble in dvi file\x00" as *const u8
                            as *const i8,
                    );
                }
                _ =>
                    /* These should not occur - processed during pre-scanning */
                    {}
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dvi_init(
    mut dvi_filename: *const i8,
    mut mag: libc::c_double,
) -> libc::c_double {
    let mut post_location: int32_t = 0;
    if dvi_filename.is_null() {
        _tt_abort(b"filename must be specified\x00" as *const u8 as *const i8);
    }
    dvi_handle = ttstub_input_open(dvi_filename, TTIF_BINARY, 0i32);
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
    dvi_page_buffer = new((dvi_page_buf_size as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64)
        as u32) as *mut u8;
    return dvi2pts;
}
#[no_mangle]
pub unsafe extern "C" fn dvi_close() {
    let mut i: libc::c_uint = 0;
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
        i = 0i32 as libc::c_uint;
        while i < num_def_fonts {
            let ref mut fresh23 = (*def_fonts.offset(i as isize)).font_name;
            *fresh23 = mfree((*def_fonts.offset(i as isize)).font_name as *mut libc::c_void)
                as *mut i8;
            i = i.wrapping_add(1)
        }
        free(def_fonts as *mut libc::c_void);
    }
    def_fonts = 0 as *mut font_def;
    page_loc = mfree(page_loc as *mut libc::c_void) as *mut u32;
    num_pages = 0i32 as libc::c_uint;
    i = 0i32 as libc::c_uint;
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
    num_loaded_fonts = 0i32 as libc::c_uint;
    vf_close_all_fonts();
    tfm_close_all();
    if !dvi_page_buffer.is_null() {
        dvi_page_buffer = mfree(dvi_page_buffer as *mut libc::c_void) as *mut u8;
        dvi_page_buf_size = 0i32 as libc::c_uint
    };
}
/* The following are need to implement virtual fonts
According to documentation, the vf "subroutine"
must have state pushed and must have
w,v,y, and z set to zero.  The current font
is determined by the virtual font header, which
may be undefined */
static mut saved_dvi_font: [libc::c_int; 16] = [0; 16];
static mut num_saved_fonts: libc::c_uint = 0i32 as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn dvi_vf_init(mut dev_font_id: libc::c_int) {
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
        _tt_abort(b"Virtual fonts nested too deeply!\x00" as *const u8 as *const i8);
    }
    current_font = dev_font_id;
}
/* After VF subroutine is finished, we simply pop the DVI stack */
#[no_mangle]
pub unsafe extern "C" fn dvi_vf_finish() {
    dpx_dvi_pop();
    if num_saved_fonts > 0i32 as libc::c_uint {
        num_saved_fonts = num_saved_fonts.wrapping_sub(1);
        current_font = saved_dvi_font[num_saved_fonts as usize]
    } else {
        _tt_abort(b"Tried to pop an empty font stack\x00" as *const u8 as *const i8);
    };
}
/* Scan various specials */
/* This need to allow 'true' prefix for unit and
 * length value must be divided by current magnification.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn read_length(
    mut vp: *mut libc::c_double,
    mut mag: libc::c_double,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> libc::c_int {
    let mut q: *mut i8 = 0 as *mut i8; /* remember this for free, because q may be advanced */
    let mut p: *const i8 = *pp; /* inverse magnify */
    let mut v: libc::c_double = 0.;
    let mut u: libc::c_double = 1.0f64;
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
    let mut k: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
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
                8 => u *= 72.0f64 / (72.27f64 * 65536i32 as libc::c_double),
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
            dpx_warning(
                b"Missing unit of measure after \"true\"\x00" as *const u8 as *const i8,
            );
            error = -1i32
        }
    }
    *vp = v * u;
    *pp = p;
    return error;
}
unsafe extern "C" fn scan_special(
    mut wd: *mut libc::c_double,
    mut ht: *mut libc::c_double,
    mut xo: *mut libc::c_double,
    mut yo: *mut libc::c_double,
    mut lm: *mut libc::c_int,
    mut majorversion: *mut libc::c_int,
    mut minorversion: *mut libc::c_int,
    mut do_enc: *mut libc::c_int,
    mut key_bits: *mut libc::c_int,
    mut permission: *mut int32_t,
    mut owner_pw: *mut i8,
    mut user_pw: *mut i8,
    mut buf: *const i8,
    mut size: u32,
) -> libc::c_int {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = buf;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut ns_pdf: libc::c_int = 0i32;
    let mut ns_dvipdfmx: libc::c_int = 0i32;
    let mut error: libc::c_int = 0i32;
    let mut tmp: libc::c_double = 0.;
    endptr = p.offset(size as isize);
    skip_white(&mut p, endptr);
    q = parse_c_ident(&mut p, endptr);
    if streq_ptr(q, b"pdf\x00" as *const u8 as *const i8) {
        skip_white(&mut p, endptr);
        if p < endptr && *p as libc::c_int == ':' as i32 {
            p = p.offset(1);
            skip_white(&mut p, endptr);
            free(q as *mut libc::c_void);
            q = parse_c_ident(&mut p, endptr);
            ns_pdf = 1i32
        }
    } else if streq_ptr(q, b"x\x00" as *const u8 as *const i8) {
        skip_white(&mut p, endptr);
        if p < endptr && *p as libc::c_int == ':' as i32 {
            p = p.offset(1);
            skip_white(&mut p, endptr);
            free(q as *mut libc::c_void);
            q = parse_c_ident(&mut p, endptr)
        }
    } else if streq_ptr(q, b"dvipdfmx\x00" as *const u8 as *const i8) {
        skip_white(&mut p, endptr);
        if p < endptr && *p as libc::c_int == ':' as i32 {
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
        } else if ns_pdf != 0
            && streq_ptr(q, b"pagesize\x00" as *const u8 as *const i8) as libc::c_int != 0
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
            let mut qchr: i8 = 0i32 as i8;
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1)
            }
            skip_white(&mut p, endptr);
            if p < endptr && (*p as libc::c_int == '\'' as i32 || *p as libc::c_int == '\"' as i32)
            {
                qchr = *p;
                p = p.offset(1);
                skip_white(&mut p, endptr);
            }
            error = read_length(&mut tmp, 1.0f64, &mut p, endptr);
            if error == 0 {
                let mut tmp1: libc::c_double = 0.;
                skip_white(&mut p, endptr);
                if p < endptr && *p as libc::c_int == ',' as i32 {
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
            if error == 0 && qchr as libc::c_int != 0 {
                /* Check if properly quoted */
                if p >= endptr || *p as libc::c_int != qchr as libc::c_int {
                    error = -1i32
                }
            }
            if error == 0i32 {
                paper_width = *wd;
                paper_height = *ht
            }
        } else if !minorversion.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"minorversion\x00" as *const u8 as *const i8) as libc::c_int
                != 0
        {
            let mut kv: *mut i8 = 0 as *mut i8;
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1)
            }
            skip_white(&mut p, endptr);
            kv = parse_float_decimal(&mut p, endptr);
            if !kv.is_null() {
                *minorversion = strtol(kv, 0 as *mut *mut i8, 10i32) as libc::c_int;
                free(kv as *mut libc::c_void);
            }
        } else if !majorversion.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"majorversion\x00" as *const u8 as *const i8) as libc::c_int
                != 0
        {
            let mut kv_0: *mut i8 = 0 as *mut i8;
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1)
            }
            skip_white(&mut p, endptr);
            kv_0 = parse_float_decimal(&mut p, endptr);
            if !kv_0.is_null() {
                *majorversion = strtol(kv_0, 0 as *mut *mut i8, 10i32) as libc::c_int;
                free(kv_0 as *mut libc::c_void);
            }
        } else if ns_pdf != 0
            && streq_ptr(q, b"encrypt\x00" as *const u8 as *const i8) as libc::c_int != 0
            && !do_enc.is_null()
        {
            *do_enc = 1i32;
            *user_pw = 0i32 as i8;
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
                            strncpy(
                                owner_pw,
                                pdf_string_value(obj) as *const i8,
                                127i32 as u64,
                            );
                        }
                        pdf_release_obj(obj);
                    } else {
                        error = -1i32
                    }
                } else if streq_ptr(kp_0, b"userpw\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_string(&mut p, endptr);
                    if !obj.is_null() {
                        if !pdf_string_value(obj).is_null() {
                            strncpy(
                                user_pw,
                                pdf_string_value(obj) as *const i8,
                                127i32 as u64,
                            );
                        }
                        pdf_release_obj(obj);
                    } else {
                        error = -1i32
                    }
                } else if streq_ptr(kp_0, b"length\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_number(&mut p, endptr);
                    if !obj.is_null() && (!obj.is_null() && pdf_obj_typeof(obj) == 2i32) {
                        *key_bits = pdf_number_value(obj) as libc::c_uint as libc::c_int
                    } else {
                        error = -1i32
                    }
                    pdf_release_obj(obj);
                } else if streq_ptr(kp_0, b"perm\x00" as *const u8 as *const i8) {
                    obj = parse_pdf_number(&mut p, endptr);
                    if !obj.is_null() && (!obj.is_null() && pdf_obj_typeof(obj) == 2i32) {
                        *permission = pdf_number_value(obj) as libc::c_uint as int32_t
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
            && streq_ptr(q, b"config\x00" as *const u8 as *const i8) as libc::c_int != 0
        {
            dpx_warning(
                b"Tectonic does not support `config\' special. Ignored.\x00" as *const u8
                    as *const i8,
            );
        }
        free(q as *mut libc::c_void);
    }
    return error;
}
static mut buffered_page: libc::c_int = -1i32;
/* returns scale (dvi2pts) */
/* may append .dvi or .xdv to filename */
/* Closes data structures created by dvi_open */
/* Renamed to avoid clash with XeTeX */
#[no_mangle]
pub unsafe extern "C" fn dvi_scan_specials(
    mut page_no: libc::c_int,
    mut page_width: *mut libc::c_double,
    mut page_height: *mut libc::c_double,
    mut x_offset: *mut libc::c_double,
    mut y_offset: *mut libc::c_double,
    mut landscape: *mut libc::c_int,
    mut majorversion: *mut libc::c_int,
    mut minorversion: *mut libc::c_int,
    mut do_enc: *mut libc::c_int,
    mut key_bits: *mut libc::c_int,
    mut permission: *mut int32_t,
    mut owner_pw: *mut i8,
    mut user_pw: *mut i8,
) {
    let mut offset: u32 = 0; /* because dvipdfmx wants to scan first page twice! */
    let mut opcode: u8 = 0;
    let mut len: libc::c_uint = 0;
    if page_no == buffered_page || num_pages == 0i32 as libc::c_uint {
        return;
    }
    buffered_page = page_no;
    dvi_page_buf_index = 0i32 as libc::c_uint;
    if linear == 0 {
        if page_no as libc::c_uint >= num_pages {
            _tt_abort(
                b"Invalid page number: %u\x00" as *const u8 as *const i8,
                page_no,
            );
        }
        offset = *page_loc.offset(page_no as isize);
        ttstub_input_seek(dvi_handle, offset as ssize_t, 0i32);
    }
    loop {
        opcode = get_and_buffer_unsigned_byte(dvi_handle) as u8;
        if !(opcode as libc::c_int != 140i32) {
            break;
        }
        if opcode as libc::c_int <= 127i32
            || opcode as libc::c_int >= 171i32 && opcode as libc::c_int <= 234i32
        {
            continue;
        }
        if opcode as libc::c_int == 239i32
            || opcode as libc::c_int == 240i32
            || opcode as libc::c_int == 241i32
            || opcode as libc::c_int == 242i32
        {
            let mut size: u32 = get_and_buffer_unsigned_byte(dvi_handle) as u32;
            let mut current_block_14: u64;
            match opcode as libc::c_int {
                242 => {
                    size = size
                        .wrapping_mul(0x100u32)
                        .wrapping_add(get_and_buffer_unsigned_byte(dvi_handle) as libc::c_uint);
                    if size > 0x7fffi32 as libc::c_uint {
                        dpx_warning(
                            b"Unsigned number starting with %x exceeds 0x7fffffff\x00" as *const u8
                                as *const i8,
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
                        .wrapping_add(get_and_buffer_unsigned_byte(dvi_handle) as libc::c_uint);
                    current_block_14 = 7135116673376365024;
                }
                _ => {}
            }
            match current_block_14 {
                7135116673376365024 => {
                    size = size
                        .wrapping_mul(0x100u32)
                        .wrapping_add(get_and_buffer_unsigned_byte(dvi_handle) as libc::c_uint)
                }
                _ => {}
            }
            if dvi_page_buf_index.wrapping_add(size) >= dvi_page_buf_size {
                dvi_page_buf_size = dvi_page_buf_index
                    .wrapping_add(size)
                    .wrapping_add(0x10000u32);
                dvi_page_buffer = renew(
                    dvi_page_buffer as *mut libc::c_void,
                    (dvi_page_buf_size as u64)
                        .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                        as u32,
                ) as *mut u8
            }
            if ttstub_input_read(
                dvi_handle,
                dvi_page_buffer.offset(dvi_page_buf_index as isize) as *mut i8,
                size as size_t,
            ) != size as i64
            {
                _tt_abort(b"Reading DVI file failed!\x00" as *const u8 as *const i8);
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
                    b"Reading special command failed: \"%.*s\"\x00" as *const u8
                        as *const i8,
                    size,
                    dvi_page_buffer.offset(dvi_page_buf_index as isize) as *mut i8,
                );
            }
            dvi_page_buf_index = dvi_page_buf_index.wrapping_add(size)
        } else {
            let mut current_block_50: u64;
            /* Skipping... */
            match opcode as libc::c_int {
                139 => {
                    get_and_buffer_bytes(dvi_handle, 44i32 as libc::c_uint); /* width */
                    current_block_50 = 6033931424626438518; /* glyph count */
                }
                138 | 141 | 142 | 147 | 152 | 161 | 166 => {
                    current_block_50 = 6033931424626438518; /* 2 bytes ID + 8 bytes x,y-location per glyph */
                }
                128 | 133 | 143 | 157 | 148 | 153 | 162 | 167 | 235 => {
                    get_and_buffer_bytes(dvi_handle, 1i32 as libc::c_uint); /* utf16 code unit count */
                    current_block_50 = 6033931424626438518; /* 2 bytes per code unit */
                }
                129 | 134 | 144 | 158 | 149 | 154 | 163 | 168 | 236 => {
                    get_and_buffer_bytes(dvi_handle, 2i32 as libc::c_uint); /* width */
                    current_block_50 = 6033931424626438518; /* glyph count */
                }
                130 | 135 | 145 | 159 | 150 | 155 | 164 | 169 | 237 => {
                    get_and_buffer_bytes(dvi_handle, 3i32 as libc::c_uint); /* 2 bytes ID + 8 bytes x,y-location per glyph */
                    current_block_50 = 6033931424626438518;
                }
                131 | 136 | 146 | 160 | 151 | 156 | 165 | 170 | 238 => {
                    get_and_buffer_bytes(dvi_handle, 4i32 as libc::c_uint);
                    current_block_50 = 6033931424626438518;
                }
                132 | 137 => {
                    get_and_buffer_bytes(dvi_handle, 8i32 as libc::c_uint);
                    current_block_50 = 6033931424626438518;
                }
                243 | 244 | 245 | 246 => {
                    do_fntdef(tt_get_unsigned_num(
                        dvi_handle,
                        (opcode as libc::c_int - 243i32) as u8,
                    ));
                    current_block_50 = 6033931424626438518;
                }
                253 => {
                    need_XeTeX(opcode as libc::c_int);
                    get_and_buffer_bytes(dvi_handle, 4i32 as libc::c_uint);
                    len = get_and_buffer_unsigned_pair(dvi_handle);
                    get_and_buffer_bytes(dvi_handle, len.wrapping_mul(10i32 as libc::c_uint));
                    current_block_50 = 6033931424626438518;
                }
                254 => {
                    need_XeTeX(opcode as libc::c_int);
                    len = get_and_buffer_unsigned_pair(dvi_handle);
                    get_and_buffer_bytes(dvi_handle, len.wrapping_mul(2i32 as libc::c_uint));
                    get_and_buffer_bytes(dvi_handle, 4i32 as libc::c_uint);
                    len = get_and_buffer_unsigned_pair(dvi_handle);
                    get_and_buffer_bytes(dvi_handle, len.wrapping_mul(10i32 as libc::c_uint));
                    current_block_50 = 6033931424626438518;
                }
                252 => {
                    need_XeTeX(opcode as libc::c_int);
                    do_native_font_def(tt_get_signed_quad(dvi_handle));
                    current_block_50 = 6033931424626438518;
                }
                250 | 251 => {
                    need_XeTeX(opcode as libc::c_int);
                    current_block_50 = 6033931424626438518;
                }
                255 => {
                    need_pTeX(opcode as libc::c_int);
                    get_and_buffer_bytes(dvi_handle, 1i32 as libc::c_uint);
                    current_block_50 = 6033931424626438518;
                }
                248 => {
                    if linear as libc::c_int != 0 && dvi_page_buf_index == 1i32 as libc::c_uint {
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
                    _tt_abort(
                        b"Unexpected opcode %d\x00" as *const u8 as *const i8,
                        opcode as libc::c_int,
                    );
                }
                _ => {}
            }
        }
    }
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
/* spt_t */
/* instantiated in dvipdfmx.c */
#[no_mangle]
pub unsafe extern "C" fn dvi_reset_global_state() {
    buffered_page = -1i32;
    num_def_fonts = 0i32 as libc::c_uint;
    max_def_fonts = 0i32 as libc::c_uint;
    compute_boxes = 0i32;
    link_annot = 1i32;
    verbose = 0i32;
    num_loaded_fonts = 0i32 as libc::c_uint;
    max_loaded_fonts = 0i32 as libc::c_uint;
}
