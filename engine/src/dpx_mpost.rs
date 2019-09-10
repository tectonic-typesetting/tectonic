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
    pub type pdf_file;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: u64,
        _: u64,
        _: *mut FILE,
    ) -> u64;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
    #[no_mangle]
    fn file_size(file: *mut FILE) -> int32_t;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_set_number(object: *mut pdf_obj, value: libc::c_double);
    #[no_mangle]
    fn pdf_number_value(number: *mut pdf_obj) -> libc::c_double;
    #[no_mangle]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_string_length(object: *mut pdf_obj) -> libc::c_uint;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_get_array(array: *mut pdf_obj, idx: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_array_length(array: *mut pdf_obj) -> libc::c_uint;
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const libc::c_char) -> *mut pdf_obj;
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
    fn pdf_color_rgbcolor(
        color: *mut pdf_color,
        r: libc::c_double,
        g: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_cmykcolor(
        color: *mut pdf_color,
        c: libc::c_double,
        m: libc::c_double,
        y: libc::c_double,
        k: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_graycolor(color: *mut pdf_color, g: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
    /* returns 1.0/unit_conv */
    #[no_mangle]
    fn dev_unit_dviunit() -> libc::c_double;
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
    /* Place XObject */
    #[no_mangle]
    fn pdf_dev_put_image(
        xobj_id: libc::c_int,
        p: *mut transform_info,
        ref_x: libc::c_double,
        ref_y: libc::c_double,
    ) -> libc::c_int;
    /* The design_size and ptsize required by PK font support...
     */
    #[no_mangle]
    fn pdf_dev_locate_font(font_name: *const libc::c_char, ptsize: spt_t) -> libc::c_int;
    /* Access text state parameters. */
    #[no_mangle]
    fn pdf_dev_get_font_wmode(font_id: libc::c_int) -> libc::c_int;
    /* ps: special support want this (pTeX). */
    /* Text composition (direction) mode
     * This affects only when auto_rotate is enabled.
     */
    #[no_mangle]
    fn pdf_dev_get_dirmode() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_set_dirmode(dir_mode: libc::c_int);
    /* Accessor to various device parameters.
     */
    #[no_mangle]
    fn pdf_dev_get_param(param_type: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_set_param(param_type: libc::c_int, value: libc::c_int);
    /* Text is normal and line art is not normal in dvipdfmx. So we don't have
     * begin_text (BT in PDF) and end_text (ET), but instead we have graphics_mode()
     * to terminate text section. pdf_dev_flushpath() and others call this.
     */
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    static mut translate_origin: libc::c_int;
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
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn pdf_lookup_fontmap_record(kp: *const libc::c_char) -> *mut fontmap_rec;
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
    fn pdf_doc_current_page_number() -> libc::c_int;
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
    fn pdf_doc_set_mediabox(page_no: libc::c_uint, mediabox: *const pdf_rect);
    /* Returns xobj_id of started xform. */
    #[no_mangle]
    fn pdf_doc_begin_grabbing(
        ident: *const libc::c_char,
        ref_x: libc::c_double,
        ref_y: libc::c_double,
        cropbox: *const pdf_rect,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_end_grabbing(attrib: *mut pdf_obj);
    #[no_mangle]
    fn pdf_dev_currentmatrix(M: *mut pdf_tmatrix) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_currentpoint(cp: *mut pdf_coord) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_setlinewidth(width: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_setmiterlimit(mlimit: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_setlinecap(style: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_setlinejoin(style: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_setdash(
        count: libc::c_int,
        pattern: *mut libc::c_double,
        offset: libc::c_double,
    ) -> libc::c_int;
    /* Path Construction */
    #[no_mangle]
    fn pdf_dev_moveto(x: libc::c_double, y: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_rmoveto(x: libc::c_double, y: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_closepath() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_lineto(x0: libc::c_double, y0: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_rlineto(x0: libc::c_double, y0: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_curveto(
        x0: libc::c_double,
        y0: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_rcurveto(
        x0: libc::c_double,
        y0: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_arc(
        c_x: libc::c_double,
        c_y: libc::c_double,
        r: libc::c_double,
        a_0: libc::c_double,
        a_1: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_arcn(
        c_x: libc::c_double,
        c_y: libc::c_double,
        r: libc::c_double,
        a_0: libc::c_double,
        a_1: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_newpath() -> libc::c_int;
    /* Path Painting */
    #[no_mangle]
    fn pdf_dev_clip() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_eoclip() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_flushpath(p_op: libc::c_char, fill_rule: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> libc::c_int;
    /* NULL pointer of M mean apply current transformation */
    #[no_mangle]
    fn pdf_dev_dtransform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[no_mangle]
    fn pdf_dev_idtransform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[no_mangle]
    fn pdf_dev_gsave() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_grestore() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_set_color(color: *const pdf_color, mask: libc::c_char, force: libc::c_int);
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
    fn dump(start: *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn pdfparse_skip_line(start: *mut *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn parse_number(start: *mut *const libc::c_char, end: *const libc::c_char)
        -> *mut libc::c_char;
    #[no_mangle]
    fn parse_ident(start: *mut *const libc::c_char, end: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn parse_pdf_name(pp: *mut *const libc::c_char, endptr: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_string(pp: *mut *const libc::c_char, endptr: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_dict(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_array(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn lookup_sfd_record(rec_id: libc::c_int, code: u8) -> libc::c_ushort;
    #[no_mangle]
    fn sfd_load_record(
        sfd_name: *const libc::c_char,
        subfont_id: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn tfm_open(tex_name: *const libc::c_char, must_exist: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tfm_get_width(font_id: libc::c_int, ch: int32_t) -> libc::c_double;
    #[no_mangle]
    fn tfm_string_width(
        font_id: libc::c_int,
        s: *const u8,
        len: libc::c_uint,
    ) -> fixword;
    #[no_mangle]
    fn tfm_exists(tfm_name: *const libc::c_char) -> bool;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type int32_t = __int32_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type fixword = int32_t;
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
    pub spot_color_name: *mut libc::c_char,
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
pub struct pdf_tmatrix {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub f: libc::c_double,
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
/* The name transform_info is misleading.
 * I'll put this here for a moment...
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub depth: libc::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut libc::c_char,
    pub font_name: *mut libc::c_char,
    pub enc_name: *mut libc::c_char,
    pub charmap: C2RustUnnamed_0,
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
    pub otl_tags: *mut libc::c_char,
    pub tounicode: *mut libc::c_char,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: libc::c_double,
    pub charcoll: *mut libc::c_char,
    pub index: libc::c_int,
    pub style: libc::c_int,
    pub stemv: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sfd_name: *mut libc::c_char,
    pub subfont_id: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp_font {
    pub font_name: *mut libc::c_char,
    pub font_id: libc::c_int,
    pub tfm_id: libc::c_int,
    pub subfont_id: libc::c_int,
    pub pt_size: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operators {
    pub token: *const libc::c_char,
    pub opcode: libc::c_int,
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
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const libc::c_char;
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
/*
 * Define the origin as (llx, lly) in order to
 * match the new xetex.def and dvipdfmx.def
 */
static mut Xorigin: libc::c_double = 0.;
static mut Yorigin: libc::c_double = 0.;
static mut font_stack: [mp_font; 256] = [
    {
        let mut init = mp_font {
            font_name: 0 as *const libc::c_char as *mut libc::c_char,
            font_id: -1i32,
            tfm_id: -1i32,
            subfont_id: -1i32,
            pt_size: 0i32 as libc::c_double,
        }; /* No currentfont */
        init
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: 0 as *const libc::c_char as *mut libc::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
];
static mut currentfont: libc::c_int = -1i32;
static mut mp_cmode: libc::c_int = 0i32;
unsafe extern "C" fn mp_setfont(
    mut font_name: *const libc::c_char,
    mut pt_size: libc::c_double,
) -> libc::c_int {
    let mut name: *const libc::c_char = font_name;
    let mut font: *mut mp_font = 0 as *mut mp_font;
    let mut subfont_id: libc::c_int = -1i32;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    font = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if !font.is_null() {
        if streq_ptr((*font).font_name, font_name) as libc::c_int != 0 && (*font).pt_size == pt_size
        {
            return 0i32;
        }
    } else {
        /* ***TODO*** Here some problem exists! */
        font = &mut *font_stack.as_mut_ptr().offset(0) as *mut mp_font;
        (*font).font_name = 0 as *mut libc::c_char;
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
    (*font).font_name = new(
        (strlen(font_name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32,
    ) as *mut libc::c_char;
    strcpy((*font).font_name, font_name);
    (*font).subfont_id = subfont_id;
    (*font).pt_size = pt_size;
    (*font).tfm_id = tfm_open(font_name, 0i32);
    (*font).font_id = pdf_dev_locate_font(name, (pt_size * dev_unit_dviunit()) as spt_t);
    if (*font).font_id < 0i32 {
        _tt_abort(
            b"MPOST: No physical font assigned for \"%s\".\x00" as *const u8 as *const libc::c_char,
            font_name,
        );
    }
    return 0i32;
}
unsafe extern "C" fn save_font() {
    let mut current: *mut mp_font = 0 as *mut mp_font;
    let mut next: *mut mp_font = 0 as *mut mp_font;
    if currentfont < 0i32 {
        font_stack[0].font_name = new((strlen(b"Courier\x00" as *const u8 as *const libc::c_char)
            .wrapping_add(1i32 as u64) as u32
            as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32) as *mut libc::c_char;
        strcpy(
            font_stack[0].font_name,
            b"Courier\x00" as *const u8 as *const libc::c_char,
        );
        font_stack[0].pt_size = 1i32 as libc::c_double;
        font_stack[0].tfm_id = 0i32;
        font_stack[0].subfont_id = 0i32;
        currentfont = 0i32
    }
    let fresh0 = currentfont;
    currentfont = currentfont + 1;
    current = &mut *font_stack.as_mut_ptr().offset(fresh0 as isize) as *mut mp_font;
    next = &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font;
    (*next).font_name = new(
        (strlen((*current).font_name).wrapping_add(1i32 as u64) as u32
            as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32,
    ) as *mut libc::c_char;
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
        (*current).font_name = mfree((*current).font_name as *mut libc::c_void) as *mut libc::c_char
    } else {
        _tt_abort(b"No currentfont...\x00" as *const u8 as *const libc::c_char);
    }
    currentfont -= 1;
}
unsafe extern "C" fn clear_fonts() {
    while currentfont >= 0i32 {
        free(font_stack[currentfont as usize].font_name as *mut libc::c_void);
        currentfont -= 1
    }
}
unsafe extern "C" fn is_fontname(mut token: *const libc::c_char) -> bool {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    mrec = pdf_lookup_fontmap_record(token);
    if !mrec.is_null() {
        return 1i32 != 0;
    }
    return tfm_exists(token);
}
#[no_mangle]
pub unsafe extern "C" fn mps_scan_bbox(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut bbox: *mut pdf_rect,
) -> libc::c_int {
    let mut number: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut values: [libc::c_double; 4] = [0.; 4];
    let mut i: libc::c_int = 0;
    /* skip_white() skips lines starting '%'... */
    while *pp < endptr
        && *(*__ctype_b_loc()).offset(**pp as u8 as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        *pp = (*pp).offset(1)
    }
    /* Scan for bounding box record */
    while *pp < endptr && **pp as libc::c_int == '%' as i32 {
        if (*pp).offset(14) < endptr
            && !strstartswith(
                *pp,
                b"%%BoundingBox:\x00" as *const u8 as *const libc::c_char,
            )
            .is_null()
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
                    (*bbox).llx = 0i32 as libc::c_double;
                    (*bbox).lly = 0i32 as libc::c_double;
                    (*bbox).urx = values[2] - values[0];
                    (*bbox).ury = values[3] - values[1];
                    Xorigin = values[0];
                    Yorigin = values[1]
                } else {
                    (*bbox).llx = values[0];
                    (*bbox).lly = values[1];
                    (*bbox).urx = values[2];
                    (*bbox).ury = values[3];
                    Xorigin = 0.0f64;
                    Yorigin = 0.0f64
                }
                return 0i32;
            }
        }
        pdfparse_skip_line(pp, endptr);
        while *pp < endptr
            && *(*__ctype_b_loc()).offset(**pp as u8 as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            *pp = (*pp).offset(1)
        }
    }
    return -1i32;
}
unsafe extern "C" fn skip_prolog(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut found_prolog: libc::c_int = 0i32;
    let mut save: *const libc::c_char = 0 as *const libc::c_char;
    save = *start;
    while *start < end {
        if **start as libc::c_int != '%' as i32 {
            skip_white(start, end);
        }
        if *start >= end {
            break;
        }
        if !strstartswith(
            *start,
            b"%%EndProlog\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
            found_prolog = 1i32;
            pdfparse_skip_line(start, end);
            break;
        } else if !strstartswith(*start, b"%%Page:\x00" as *const u8 as *const libc::c_char)
            .is_null()
        {
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
            token: b"add\x00" as *const u8 as *const libc::c_char,
            opcode: 1i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"mul\x00" as *const u8 as *const libc::c_char,
            opcode: 3i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"div\x00" as *const u8 as *const libc::c_char,
            opcode: 4i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"neg\x00" as *const u8 as *const libc::c_char,
            opcode: 5i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"sub\x00" as *const u8 as *const libc::c_char,
            opcode: 2i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"truncate\x00" as *const u8 as *const libc::c_char,
            opcode: 6i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"clear\x00" as *const u8 as *const libc::c_char,
            opcode: 10i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"exch\x00" as *const u8 as *const libc::c_char,
            opcode: 11i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"pop\x00" as *const u8 as *const libc::c_char,
            opcode: 12i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"clip\x00" as *const u8 as *const libc::c_char,
            opcode: 44i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"eoclip\x00" as *const u8 as *const libc::c_char,
            opcode: 45i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"closepath\x00" as *const u8 as *const libc::c_char,
            opcode: 32i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"concat\x00" as *const u8 as *const libc::c_char,
            opcode: 52i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"newpath\x00" as *const u8 as *const libc::c_char,
            opcode: 31i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"moveto\x00" as *const u8 as *const libc::c_char,
            opcode: 33i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rmoveto\x00" as *const u8 as *const libc::c_char,
            opcode: 34i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"lineto\x00" as *const u8 as *const libc::c_char,
            opcode: 37i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rlineto\x00" as *const u8 as *const libc::c_char,
            opcode: 38i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"curveto\x00" as *const u8 as *const libc::c_char,
            opcode: 35i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rcurveto\x00" as *const u8 as *const libc::c_char,
            opcode: 36i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"arc\x00" as *const u8 as *const libc::c_char,
            opcode: 39i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"arcn\x00" as *const u8 as *const libc::c_char,
            opcode: 40i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"stroke\x00" as *const u8 as *const libc::c_char,
            opcode: 42i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"fill\x00" as *const u8 as *const libc::c_char,
            opcode: 41i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"show\x00" as *const u8 as *const libc::c_char,
            opcode: 43i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"showpage\x00" as *const u8 as *const libc::c_char,
            opcode: 49i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"gsave\x00" as *const u8 as *const libc::c_char,
            opcode: 50i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"grestore\x00" as *const u8 as *const libc::c_char,
            opcode: 51i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"translate\x00" as *const u8 as *const libc::c_char,
            opcode: 54i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rotate\x00" as *const u8 as *const libc::c_char,
            opcode: 55i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"scale\x00" as *const u8 as *const libc::c_char,
            opcode: 53i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setlinecap\x00" as *const u8 as *const libc::c_char,
            opcode: 62i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setlinejoin\x00" as *const u8 as *const libc::c_char,
            opcode: 63i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setlinewidth\x00" as *const u8 as *const libc::c_char,
            opcode: 60i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setmiterlimit\x00" as *const u8 as *const libc::c_char,
            opcode: 64i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setdash\x00" as *const u8 as *const libc::c_char,
            opcode: 61i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setgray\x00" as *const u8 as *const libc::c_char,
            opcode: 70i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setrgbcolor\x00" as *const u8 as *const libc::c_char,
            opcode: 71i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setcmykcolor\x00" as *const u8 as *const libc::c_char,
            opcode: 72i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"currentpoint\x00" as *const u8 as *const libc::c_char,
            opcode: 80i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"dtransform\x00" as *const u8 as *const libc::c_char,
            opcode: 82i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"idtransform\x00" as *const u8 as *const libc::c_char,
            opcode: 81i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"findfont\x00" as *const u8 as *const libc::c_char,
            opcode: 201i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"scalefont\x00" as *const u8 as *const libc::c_char,
            opcode: 202i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"setfont\x00" as *const u8 as *const libc::c_char,
            opcode: 203i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"currentfont\x00" as *const u8 as *const libc::c_char,
            opcode: 204i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"stringwidth\x00" as *const u8 as *const libc::c_char,
            opcode: 210i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"def\x00" as *const u8 as *const libc::c_char,
            opcode: 999i32,
        };
        init
    },
];
static mut mps_operators: [operators; 28] = [
    {
        let mut init = operators {
            token: b"fshow\x00" as *const u8 as *const libc::c_char,
            opcode: 1001i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"startTexFig\x00" as *const u8 as *const libc::c_char,
            opcode: 1002i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"endTexFig\x00" as *const u8 as *const libc::c_char,
            opcode: 1003i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"hlw\x00" as *const u8 as *const libc::c_char,
            opcode: 1004i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"vlw\x00" as *const u8 as *const libc::c_char,
            opcode: 1005i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"l\x00" as *const u8 as *const libc::c_char,
            opcode: 37i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"r\x00" as *const u8 as *const libc::c_char,
            opcode: 38i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"c\x00" as *const u8 as *const libc::c_char,
            opcode: 35i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"m\x00" as *const u8 as *const libc::c_char,
            opcode: 33i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"p\x00" as *const u8 as *const libc::c_char,
            opcode: 32i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"n\x00" as *const u8 as *const libc::c_char,
            opcode: 31i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"C\x00" as *const u8 as *const libc::c_char,
            opcode: 72i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"G\x00" as *const u8 as *const libc::c_char,
            opcode: 70i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"R\x00" as *const u8 as *const libc::c_char,
            opcode: 71i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"lj\x00" as *const u8 as *const libc::c_char,
            opcode: 63i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"ml\x00" as *const u8 as *const libc::c_char,
            opcode: 64i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"lc\x00" as *const u8 as *const libc::c_char,
            opcode: 62i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"S\x00" as *const u8 as *const libc::c_char,
            opcode: 42i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"F\x00" as *const u8 as *const libc::c_char,
            opcode: 41i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"q\x00" as *const u8 as *const libc::c_char,
            opcode: 50i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"Q\x00" as *const u8 as *const libc::c_char,
            opcode: 51i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"s\x00" as *const u8 as *const libc::c_char,
            opcode: 53i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"t\x00" as *const u8 as *const libc::c_char,
            opcode: 52i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"sd\x00" as *const u8 as *const libc::c_char,
            opcode: 61i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"rd\x00" as *const u8 as *const libc::c_char,
            opcode: 1006i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"P\x00" as *const u8 as *const libc::c_char,
            opcode: 49i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"B\x00" as *const u8 as *const libc::c_char,
            opcode: 1007i32,
        };
        init
    },
    {
        let mut init = operators {
            token: b"W\x00" as *const u8 as *const libc::c_char,
            opcode: 44i32,
        };
        init
    },
];
unsafe extern "C" fn get_opcode(mut token: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while (i as u64)
        < (::std::mem::size_of::<[operators; 48]>() as u64)
            .wrapping_div(::std::mem::size_of::<operators>() as u64)
    {
        if streq_ptr(token, ps_operators[i as usize].token) {
            return ps_operators[i as usize].opcode;
        }
        i = i.wrapping_add(1)
    }
    i = 0i32 as libc::c_uint;
    while (i as u64)
        < (::std::mem::size_of::<[operators; 28]>() as u64)
            .wrapping_div(::std::mem::size_of::<operators>() as u64)
    {
        if streq_ptr(token, mps_operators[i as usize].token) {
            return mps_operators[i as usize].opcode;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}
static mut stack: [*mut pdf_obj; 1024] = [0 as *const pdf_obj as *mut pdf_obj; 1024];
static mut top_stack: libc::c_uint = 0i32 as libc::c_uint;
unsafe extern "C" fn do_exch() -> libc::c_int {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    if top_stack < 2i32 as libc::c_uint {
        return -1i32;
    }
    tmp = stack[top_stack.wrapping_sub(1i32 as libc::c_uint) as usize];
    stack[top_stack.wrapping_sub(1i32 as libc::c_uint) as usize] =
        stack[top_stack.wrapping_sub(2i32 as libc::c_uint) as usize];
    stack[top_stack.wrapping_sub(2i32 as libc::c_uint) as usize] = tmp;
    return 0i32;
}
unsafe extern "C" fn do_clear() -> libc::c_int {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    while top_stack > 0i32 as libc::c_uint {
        tmp = if top_stack > 0i32 as libc::c_uint {
            top_stack = top_stack.wrapping_sub(1);
            stack[top_stack as usize]
        } else {
            0 as *mut pdf_obj
        };
        pdf_release_obj(tmp);
    }
    return 0i32;
}
unsafe extern "C" fn pop_get_numbers(
    mut values: *mut libc::c_double,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    loop {
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        tmp = if top_stack > 0i32 as libc::c_uint {
            top_stack = top_stack.wrapping_sub(1);
            stack[top_stack as usize]
        } else {
            0 as *mut pdf_obj
        };
        if tmp.is_null() {
            dpx_warning(b"mpost: Stack underflow.\x00" as *const u8 as *const libc::c_char);
            break;
        } else if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
            dpx_warning(b"mpost: Not a number!\x00" as *const u8 as *const libc::c_char);
            pdf_release_obj(tmp);
            break;
        } else {
            *values.offset(count as isize) = pdf_number_value(tmp);
            pdf_release_obj(tmp);
        }
    }
    return count + 1i32;
}
unsafe extern "C" fn cvr_array(
    mut array: *mut pdf_obj,
    mut values: *mut libc::c_double,
    mut count: libc::c_int,
) -> libc::c_int {
    if !(!array.is_null() && pdf_obj_typeof(array) == 5i32) {
        dpx_warning(b"mpost: Not an array!\x00" as *const u8 as *const libc::c_char);
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
                dpx_warning(b"mpost: Not a number!\x00" as *const u8 as *const libc::c_char);
                break;
            } else {
                *values.offset(count as isize) = pdf_number_value(tmp)
            }
        }
    }
    pdf_release_obj(array);
    return count + 1i32;
}
unsafe extern "C" fn is_fontdict(mut dict: *mut pdf_obj) -> bool {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    if !(!dict.is_null() && pdf_obj_typeof(dict) == 6i32) {
        return 0i32 != 0;
    }
    tmp = pdf_lookup_dict(dict, b"Type\x00" as *const u8 as *const libc::c_char);
    if tmp.is_null()
        || !(!tmp.is_null() && pdf_obj_typeof(tmp) == 4i32)
        || strcmp(
            pdf_name_value(tmp),
            b"Font\x00" as *const u8 as *const libc::c_char,
        ) != 0
    {
        return 0i32 != 0;
    }
    tmp = pdf_lookup_dict(dict, b"FontName\x00" as *const u8 as *const libc::c_char);
    if tmp.is_null() || !(!tmp.is_null() && pdf_obj_typeof(tmp) == 4i32) {
        return 0i32 != 0;
    }
    tmp = pdf_lookup_dict(dict, b"FontScale\x00" as *const u8 as *const libc::c_char);
    if tmp.is_null() || !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
        return 0i32 != 0;
    }
    return 1i32 != 0;
}
unsafe extern "C" fn do_findfont() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut font_name: *mut pdf_obj = 0 as *mut pdf_obj;
    font_name = if top_stack > 0i32 as libc::c_uint {
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
                pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
                pdf_new_name(b"Font\x00" as *const u8 as *const libc::c_char),
            );
            if !font_name.is_null() && pdf_obj_typeof(font_name) == 3i32 {
                pdf_add_dict(
                    font_dict,
                    pdf_new_name(b"FontName\x00" as *const u8 as *const libc::c_char),
                    pdf_new_name(pdf_string_value(font_name) as *const libc::c_char),
                );
                pdf_release_obj(font_name);
            } else {
                pdf_add_dict(
                    font_dict,
                    pdf_new_name(b"FontName\x00" as *const u8 as *const libc::c_char),
                    font_name,
                );
            }
            pdf_add_dict(
                font_dict,
                pdf_new_name(b"FontScale\x00" as *const u8 as *const libc::c_char),
                pdf_new_number(1.0f64),
            );
            if top_stack < 1024i32 as libc::c_uint {
                let fresh3 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh3 as usize] = font_dict
            } else {
                dpx_warning(
                    b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                        as *const libc::c_char,
                );
                pdf_release_obj(font_dict);
                error = 1i32
            }
        } else {
            error = 1i32
        }
    }
    return error;
}
unsafe extern "C" fn do_scalefont() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut font_scale: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut scale: libc::c_double = 0.;
    error = pop_get_numbers(&mut scale, 1i32);
    if error != 0 {
        return error;
    }
    font_dict = if top_stack > 0i32 as libc::c_uint {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        0 as *mut pdf_obj
    };
    if font_dict.is_null() {
        error = 1i32
    } else if is_fontdict(font_dict) {
        font_scale = pdf_lookup_dict(
            font_dict,
            b"FontScale\x00" as *const u8 as *const libc::c_char,
        );
        pdf_set_number(font_scale, pdf_number_value(font_scale) * scale);
        if top_stack < 1024i32 as libc::c_uint {
            let fresh4 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh4 as usize] = font_dict
        } else {
            dpx_warning(
                b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                    as *const libc::c_char,
            );
            pdf_release_obj(font_dict);
            error = 1i32
        }
    } else {
        error = 1i32
    }
    return error;
}
unsafe extern "C" fn do_setfont() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut font_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut font_scale: libc::c_double = 0.;
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    font_dict = if top_stack > 0i32 as libc::c_uint {
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
            b"FontName\x00" as *const u8 as *const libc::c_char,
        ));
        font_scale = pdf_number_value(pdf_lookup_dict(
            font_dict,
            b"FontScale\x00" as *const u8 as *const libc::c_char,
        ));
        error = mp_setfont(font_name, font_scale)
    }
    pdf_release_obj(font_dict);
    return error;
}
/* Push dummy font dict onto PS stack */
unsafe extern "C" fn do_currentfont() -> libc::c_int {
    let mut error: libc::c_int = 0i32; /* Should not be error... */
    let mut font: *mut mp_font = 0 as *mut mp_font; /* Should not be error... */
    let mut font_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    font = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if font.is_null() {
        dpx_warning(b"Currentfont undefined...\x00" as *const u8 as *const libc::c_char);
        return 1i32;
    } else {
        font_dict = pdf_new_dict();
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"Font\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontName\x00" as *const u8 as *const libc::c_char),
            pdf_new_name((*font).font_name),
        );
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontScale\x00" as *const u8 as *const libc::c_char),
            pdf_new_number((*font).pt_size),
        );
        if top_stack < 1024i32 as libc::c_uint {
            let fresh5 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh5 as usize] = font_dict
        } else {
            dpx_warning(b"PS stack overflow...\x00" as *const u8 as *const libc::c_char);
            pdf_release_obj(font_dict);
            error = 1i32
        }
    }
    return error;
}
unsafe extern "C" fn do_show() -> libc::c_int {
    let mut font: *mut mp_font = 0 as *mut mp_font;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut text_str: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut length: libc::c_int = 0;
    let mut strptr: *mut u8 = 0 as *mut u8;
    let mut text_width: libc::c_double = 0.;
    font = if currentfont < 0i32 {
        0 as *mut mp_font
    } else {
        &mut *font_stack.as_mut_ptr().offset(currentfont as isize) as *mut mp_font
    };
    if font.is_null() {
        dpx_warning(b"Currentfont not set.\x00" as *const u8 as *const libc::c_char);
        return 1i32;
    }
    pdf_dev_currentpoint(&mut cp);
    text_str = if top_stack > 0i32 as libc::c_uint {
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
        dpx_warning(b"mpost: not set.\x00" as *const u8 as *const libc::c_char);
        pdf_release_obj(text_str);
        return 1i32;
    }
    strptr = pdf_string_value(text_str) as *mut u8;
    length = pdf_string_length(text_str) as libc::c_int;
    if (*font).tfm_id < 0i32 {
        dpx_warning(
            b"mpost: TFM not found for \"%s\".\x00" as *const u8 as *const libc::c_char,
            (*font).font_name,
        );
        dpx_warning(b"mpost: Text width not calculated...\x00" as *const u8 as *const libc::c_char);
    }
    text_width = 0.0f64;
    if (*font).subfont_id >= 0i32 {
        let mut uch: libc::c_ushort = 0;
        let mut ustr: *mut u8 = 0 as *mut u8;
        let mut i: libc::c_int = 0;
        ustr = new(((length * 2i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64)
            as u32) as *mut u8;
        i = 0i32;
        while i < length {
            uch = lookup_sfd_record((*font).subfont_id, *strptr.offset(i as isize));
            *ustr.offset((2i32 * i) as isize) = (uch as libc::c_int >> 8i32) as u8;
            *ustr.offset((2i32 * i + 1i32) as isize) =
                (uch as libc::c_int & 0xffi32) as u8;
            if (*font).tfm_id >= 0i32 {
                text_width += tfm_get_width((*font).tfm_id, *strptr.offset(i as isize) as int32_t)
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
            text_width = tfm_string_width((*font).tfm_id, strptr, length as libc::c_uint)
                as libc::c_double
                / (1i32 << 20i32) as libc::c_double;
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
    return 0i32;
}
unsafe extern "C" fn do_mpost_bind_def(
    mut ps_code: *const libc::c_char,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    start = ps_code;
    end = start.offset(strlen(start) as isize);
    error = mp_parse_body(&mut start, end, x_user, y_user);
    return error;
}
unsafe extern "C" fn do_texfig_operator(
    mut opcode: libc::c_int,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
) -> libc::c_int {
    static mut fig_p: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    static mut in_tfig: libc::c_int = 0i32;
    static mut xobj_id: libc::c_int = -1i32;
    static mut count: libc::c_int = 0i32;
    let mut values: [libc::c_double; 6] = [0.; 6];
    let mut error: libc::c_int = 0i32;
    match opcode {
        1002 => {
            error = pop_get_numbers(values.as_mut_ptr(), 6i32);
            if error == 0 {
                let mut dvi2pts: libc::c_double = 0.;
                let mut resname: [libc::c_char; 256] = [0; 256];
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
                    b"__tf%d__\x00" as *const u8 as *const libc::c_char,
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
                _tt_abort(
                    b"endTexFig without valid startTexFig!.\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            pdf_doc_end_grabbing(0 as *mut pdf_obj);
            pdf_dev_put_image(xobj_id, &mut fig_p, x_user, y_user);
            in_tfig = 0i32
        }
        _ => error = 1i32,
    }
    return error;
}
unsafe extern "C" fn ps_dev_CTM(mut M: *mut pdf_tmatrix) -> libc::c_int {
    pdf_dev_currentmatrix(M);
    (*M).a *= 1000i32 as libc::c_double;
    (*M).b *= 1000i32 as libc::c_double;
    (*M).c *= 1000i32 as libc::c_double;
    (*M).d *= 1000i32 as libc::c_double;
    (*M).e *= 1000i32 as libc::c_double;
    (*M).f *= 1000i32 as libc::c_double;
    return 0i32;
}
/*
 * Again, the only piece that needs x_user and y_user is
 * that piece dealing with texfig.
 */
unsafe extern "C" fn do_operator(
    mut token: *const libc::c_char,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut opcode: libc::c_int = 0i32;
    let mut values: [libc::c_double; 12] = [0.; 12];
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut matrix: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut color: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut libc::c_char,
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
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh6 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh6 as usize] = pdf_new_number(values[0] + values[1])
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        3 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh7 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh7 as usize] = pdf_new_number(values[0] * values[1])
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        5 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh8 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh8 as usize] = pdf_new_number(-values[0])
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        2 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh9 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh9 as usize] = pdf_new_number(values[0] - values[1])
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        4 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh10 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh10 as usize] = pdf_new_number(values[0] / values[1])
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        6 => {
            /* Round toward zero. */
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh11 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh11 as usize] =
                        pdf_new_number(if values[0] > 0i32 as libc::c_double {
                            floor(values[0])
                        } else {
                            ceil(values[0])
                        })
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        10 => {
            /* Stack operation */
            error = do_clear()
        }
        12 => {
            tmp = if top_stack > 0i32 as libc::c_uint {
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
            pdf_dev_flushpath('S' as i32 as libc::c_char, 0i32);
        }
        41 => {
            pdf_dev_flushpath('f' as i32 as libc::c_char, 0i32);
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
            tmp = if top_stack > 0i32 as libc::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            error = cvr_array(tmp, values.as_mut_ptr(), 6i32);
            tmp = 0 as *mut pdf_obj;
            if error != 0 {
                dpx_warning(
                    b"Missing array before \"concat\".\x00" as *const u8 as *const libc::c_char,
                );
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
                values[0] = values[0] * 3.14159265358979323846f64 / 180i32 as libc::c_double;
                match mp_cmode {
                    1 | 0 => {
                        /* Really? */
                        matrix.a = cos(values[0]);
                        matrix.b = -sin(values[0]);
                        matrix.c = sin(values[0]);
                        matrix.d = cos(values[0]);
                        matrix.e = 0.0f64;
                        matrix.f = 0.0f64
                    }
                    _ => {
                        matrix.a = cos(values[0]);
                        matrix.b = sin(values[0]);
                        matrix.c = -sin(values[0]);
                        matrix.d = cos(values[0]);
                        matrix.e = 0.0f64;
                        matrix.f = 0.0f64
                    }
                }
                error = pdf_dev_concat(&mut matrix)
            }
        }
        54 => {
            error = pop_get_numbers(values.as_mut_ptr(), 2i32);
            if error == 0 {
                matrix.a = 1.0f64;
                matrix.b = 0.0f64;
                matrix.c = 0.0f64;
                matrix.d = 1.0f64;
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
                let mut i: libc::c_int = 0;
                let mut num_dashes: libc::c_int = 0;
                let mut dash_values: [libc::c_double; 16] = [0.; 16];
                let mut offset: libc::c_double = 0.;
                offset = values[0];
                pattern = if top_stack > 0i32 as libc::c_uint {
                    top_stack = top_stack.wrapping_sub(1);
                    stack[top_stack as usize]
                } else {
                    0 as *mut pdf_obj
                };
                if !(!pattern.is_null() && pdf_obj_typeof(pattern) == 5i32) {
                    pdf_release_obj(pattern);
                    error = 1i32
                } else {
                    num_dashes = pdf_array_length(pattern) as libc::c_int;
                    if num_dashes > 16i32 {
                        dpx_warning(b"Too many dashes...\x00" as *const u8 as *const libc::c_char);
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
                error = pdf_dev_setlinecap(values[0] as libc::c_int)
            }
        }
        63 => {
            error = pop_get_numbers(values.as_mut_ptr(), 1i32);
            if error == 0 {
                error = pdf_dev_setlinejoin(values[0] as libc::c_int)
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
                pdf_dev_set_color(&mut color, 0i32 as libc::c_char, 0i32);
                pdf_dev_set_color(&mut color, 0x20i32 as libc::c_char, 0i32);
            }
        }
        70 => {
            /* Not handled properly */
            error = pop_get_numbers(values.as_mut_ptr(), 1i32); /* This does pdf_release_obj() */
            if error == 0 {
                pdf_color_graycolor(&mut color, values[0]);
                pdf_dev_set_color(&mut color, 0i32 as libc::c_char, 0i32);
                pdf_dev_set_color(&mut color, 0x20i32 as libc::c_char, 0i32);
            }
        }
        71 => {
            error = pop_get_numbers(values.as_mut_ptr(), 3i32);
            if error == 0 {
                pdf_color_rgbcolor(&mut color, values[0], values[1], values[2]);
                pdf_dev_set_color(&mut color, 0i32 as libc::c_char, 0i32);
                pdf_dev_set_color(&mut color, 0x20i32 as libc::c_char, 0i32);
            }
        }
        49 => {}
        80 => {
            error = pdf_dev_currentpoint(&mut cp);
            if error == 0 {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh12 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh12 as usize] = pdf_new_number(cp.x);
                    if top_stack < 1024i32 as libc::c_uint {
                        let fresh13 = top_stack;
                        top_stack = top_stack.wrapping_add(1);
                        stack[fresh13 as usize] = pdf_new_number(cp.y)
                    } else {
                        dpx_warning(
                            b"PS stack overflow including MetaPost file or inline PS code\x00"
                                as *const u8 as *const libc::c_char,
                        );
                        error = 1i32
                    }
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            }
        }
        82 => {
            let mut has_matrix: libc::c_int = 0i32;
            tmp = if top_stack > 0i32 as libc::c_uint {
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
                    tmp = if top_stack > 0i32 as libc::c_uint {
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
                        tmp = if top_stack > 0i32 as libc::c_uint {
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
                            pdf_dev_dtransform(&mut cp, &mut matrix);
                            if top_stack < 1024i32 as libc::c_uint {
                                let fresh14 = top_stack;
                                top_stack = top_stack.wrapping_add(1);
                                stack[fresh14 as usize] = pdf_new_number(cp.x);
                                if top_stack < 1024i32 as libc::c_uint {
                                    let fresh15 = top_stack;
                                    top_stack = top_stack.wrapping_add(1);
                                    stack[fresh15 as usize] = pdf_new_number(cp.y)
                                } else {
                                    dpx_warning(b"PS stack overflow including MetaPost file or inline PS code\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    error = 1i32
                                }
                            } else {
                                dpx_warning(b"PS stack overflow including MetaPost file or inline PS code\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                error = 1i32
                            }
                        }
                    }
                }
            }
        }
        81 => {
            let mut has_matrix_0: libc::c_int = 0i32;
            tmp = if top_stack > 0i32 as libc::c_uint {
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
                    tmp = if top_stack > 0i32 as libc::c_uint {
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
                        tmp = if top_stack > 0i32 as libc::c_uint {
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
                            pdf_dev_idtransform(&mut cp, &mut matrix);
                            if top_stack < 1024i32 as libc::c_uint {
                                let fresh16 = top_stack;
                                top_stack = top_stack.wrapping_add(1);
                                stack[fresh16 as usize] = pdf_new_number(cp.x);
                                if top_stack < 1024i32 as libc::c_uint {
                                    let fresh17 = top_stack;
                                    top_stack = top_stack.wrapping_add(1);
                                    stack[fresh17 as usize] = pdf_new_number(cp.y)
                                } else {
                                    dpx_warning(b"PS stack overflow including MetaPost file or inline PS code\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    error = 1i32
                                }
                            } else {
                                dpx_warning(b"PS stack overflow including MetaPost file or inline PS code\x00"
                                                as *const u8 as
                                                *const libc::c_char);
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
                b"exch findfont exch scalefont setfont show\x00" as *const u8
                    as *const libc::c_char,
                x_user,
                y_user,
            )
        }
        1002 | 1003 => error = do_texfig_operator(opcode, x_user, y_user),
        1004 => {
            error = do_mpost_bind_def(
                b"0 dtransform exch truncate exch idtransform pop setlinewidth\x00" as *const u8
                    as *const libc::c_char,
                x_user,
                y_user,
            )
        }
        1005 => {
            error = do_mpost_bind_def(
                b"0 exch dtransform truncate idtransform setlinewidth pop\x00" as *const u8
                    as *const libc::c_char,
                x_user,
                y_user,
            )
        }
        1006 => {
            error = do_mpost_bind_def(
                b"[] 0 setdash\x00" as *const u8 as *const libc::c_char,
                x_user,
                y_user,
            )
        }
        1007 => {
            error = do_mpost_bind_def(
                b"gsave fill grestore\x00" as *const u8 as *const libc::c_char,
                x_user,
                y_user,
            )
        }
        999 => {
            tmp = if top_stack > 0i32 as libc::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            };
            tmp = if top_stack > 0i32 as libc::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                0 as *mut pdf_obj
            }
        }
        _ => {
            if is_fontname(token) {
                if top_stack < 1024i32 as libc::c_uint {
                    let fresh18 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh18 as usize] = pdf_new_name(token)
                } else {
                    dpx_warning(
                        b"PS stack overflow including MetaPost file or inline PS code\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    error = 1i32
                }
            } else {
                dpx_warning(
                    b"Unknown token \"%s\"\x00" as *const u8 as *const libc::c_char,
                    token,
                );
                error = 1i32
            }
        }
    }
    return error;
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
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
) -> libc::c_int {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: libc::c_int = 0i32;
    skip_white(start, end);
    while *start < end && error == 0 {
        if *(*__ctype_b_loc()).offset(**start as u8 as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || *start < end.offset(-1)
                && (**start as libc::c_int == '+' as i32
                    || **start as libc::c_int == '-' as i32
                    || **start as libc::c_int == '.' as i32)
        {
            let mut value: libc::c_double = 0.;
            let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
            value = strtod(*start, &mut next);
            if next < end as *mut libc::c_char
                && strchr(
                    b"<([{/%\x00" as *const u8 as *const libc::c_char,
                    *next as libc::c_int,
                )
                .is_null()
                && *(*__ctype_b_loc()).offset(*next as u8 as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
            {
                dpx_warning(b"Unkown PostScript operator.\x00" as *const u8 as *const libc::c_char);
                dump(*start, next);
                error = 1i32
            } else if top_stack < 1024i32 as libc::c_uint {
                let fresh19 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh19 as usize] = pdf_new_number(value);
                *start = next
            } else {
                dpx_warning(
                    b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                        as *const libc::c_char,
                );
                error = 1i32;
                break;
            }
        /*
         * PDF parser can't handle PS operator inside arrays.
         * This shouldn't use parse_pdf_array().
         */
        } else if **start as libc::c_int == '[' as i32 && {
            obj = parse_pdf_array(start, end, 0 as *mut pdf_file);
            !obj.is_null()
        } {
            if top_stack < 1024i32 as libc::c_uint {
                let fresh20 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh20 as usize] = obj
            } else {
                dpx_warning(
                    b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                        as *const libc::c_char,
                );
                error = 1i32;
                break;
            }
        /* This cannot handle ASCII85 string. */
        } else if *start < end.offset(-1)
            && (**start as libc::c_int == '<' as i32
                && *(*start).offset(1) as libc::c_int == '<' as i32)
            && {
                obj = parse_pdf_dict(start, end, 0 as *mut pdf_file);
                !obj.is_null()
            }
        {
            if top_stack < 1024i32 as libc::c_uint {
                let fresh21 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh21 as usize] = obj
            } else {
                dpx_warning(
                    b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                        as *const libc::c_char,
                );
                error = 1i32;
                break;
            }
        } else if (**start as libc::c_int == '(' as i32 || **start as libc::c_int == '<' as i32)
            && {
                obj = parse_pdf_string(start, end);
                !obj.is_null()
            }
        {
            if top_stack < 1024i32 as libc::c_uint {
                let fresh22 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh22 as usize] = obj
            } else {
                dpx_warning(
                    b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                        as *const libc::c_char,
                );
                error = 1i32;
                break;
            }
        } else if **start as libc::c_int == '/' as i32 && {
            obj = parse_pdf_name(start, end);
            !obj.is_null()
        } {
            if top_stack < 1024i32 as libc::c_uint {
                let fresh23 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh23 as usize] = obj
            } else {
                dpx_warning(
                    b"PS stack overflow including MetaPost file or inline PS code\x00" as *const u8
                        as *const libc::c_char,
                );
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
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn mps_eop_cleanup() {
    clear_fonts();
    do_clear();
}
#[no_mangle]
pub unsafe extern "C" fn mps_stack_depth() -> libc::c_int {
    return top_stack as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mps_exec_inline(
    mut p: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut dirmode: libc::c_int = 0;
    let mut autorotate: libc::c_int = 0;
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
    return error;
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
pub unsafe extern "C" fn mps_do_page(mut image_file: *mut FILE) -> libc::c_int {
    let mut error: libc::c_int = 0i32; /* scale, xorig, yorig */
    let mut bbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: libc::c_int = 0;
    let mut dir_mode: libc::c_int = 0;
    rewind(image_file);
    size = file_size(image_file);
    if size == 0i32 {
        dpx_warning(
            b"Can\'t read any byte in the MPS file.\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    buffer = new(((size + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    fread(
        buffer as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as u64,
        size as u64,
        image_file,
    );
    *buffer.offset(size as isize) = 0i32 as libc::c_char;
    start = buffer;
    end = buffer.offset(size as isize);
    error = mps_scan_bbox(&mut start, end, &mut bbox);
    if error != 0 {
        dpx_warning(
            b"Error occured while scanning MetaPost file headers: Could not find BoundingBox.\x00"
                as *const u8 as *const libc::c_char,
        );
        free(buffer as *mut libc::c_void);
        return -1i32;
    }
    mp_cmode = 0i32;
    pdf_doc_begin_page(1.0f64, -Xorigin, -Yorigin);
    pdf_doc_set_mediabox(pdf_doc_current_page_number() as libc::c_uint, &mut bbox);
    dir_mode = pdf_dev_get_dirmode();
    pdf_dev_set_param(1i32, 0i32);
    skip_prolog(&mut start, end);
    error = mp_parse_body(&mut start, end, 0.0f64, 0.0f64);
    if error != 0 {
        dpx_warning(
            b"Errors occured while interpreting MetaPost file.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    pdf_dev_set_param(1i32, 1i32);
    pdf_dev_set_dirmode(dir_mode);
    pdf_doc_end_page();
    free(buffer as *mut libc::c_void);
    /*
     * The reason why we don't return XObject itself is
     * PDF inclusion may not be made so.
     */
    return if error != 0 { -1i32 } else { 0i32 };
}
