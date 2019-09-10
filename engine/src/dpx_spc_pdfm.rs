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
    pub type pdf_file;
    #[no_mangle]
    fn spc_clear_objects();
    #[no_mangle]
    fn spc_flush_object(key: *const i8);
    #[no_mangle]
    fn spc_push_object(key: *const i8, value: *mut pdf_obj);
    #[no_mangle]
    fn spc_suspend_annot(spe: *mut spc_env) -> libc::c_int;
    #[no_mangle]
    fn spc_resume_annot(spe: *mut spc_env) -> libc::c_int;
    #[no_mangle]
    fn spc_end_annot(spe: *mut spc_env) -> libc::c_int;
    #[no_mangle]
    fn spc_begin_annot(spe: *mut spc_env, annot_dict: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn spc_lookup_object(ident: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_foreach_dict(
        dict: *mut pdf_obj,
        proc_0: Option<
            unsafe extern "C" fn(
                _: *mut pdf_obj,
                _: *mut pdf_obj,
                _: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        pdata: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[no_mangle]
    fn pdf_remove_dict(dict: *mut pdf_obj, key: *const i8);
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_array_length(array: *mut pdf_obj) -> libc::c_uint;
    #[no_mangle]
    fn pdf_get_array(array: *mut pdf_obj, idx: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut i8;
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_string_length(object: *mut pdf_obj) -> libc::c_uint;
    #[no_mangle]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_set_string(object: *mut pdf_obj, str: *mut u8, length: size_t);
    #[no_mangle]
    fn pdf_number_value(number: *mut pdf_obj) -> libc::c_double;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut i8,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    static mut work_buffer: [i8; 0];
    #[no_mangle]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[no_mangle]
    fn ht_clear_table(ht: *mut ht_table);
    #[no_mangle]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
        value: *mut libc::c_void,
    );
    #[no_mangle]
    fn parse_c_ident(
        pp: *mut *const i8,
        endptr: *const i8,
    ) -> *mut i8;
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
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn pdf_init_fontmap_record(mrec: *mut fontmap_rec);
    #[no_mangle]
    fn pdf_clear_fontmap_record(mrec: *mut fontmap_rec);
    #[no_mangle]
    fn pdf_load_fontmap_file(filename: *const i8, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_read_fontmap_line(
        mrec: *mut fontmap_rec,
        mline: *const i8,
        mline_strlen: libc::c_int,
        format: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_append_fontmap_record(kp: *const i8, mrec: *const fontmap_rec) -> libc::c_int;
    #[no_mangle]
    fn pdf_remove_fontmap_record(kp: *const i8) -> libc::c_int;
    #[no_mangle]
    fn pdf_insert_fontmap_record(
        kp: *const i8,
        mrec: *const fontmap_rec,
    ) -> *mut fontmap_rec;
    #[no_mangle]
    fn is_pdfm_mapline(mline: *const i8) -> libc::c_int;
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
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    /* Color special
     * See remark in spc_color.c.
     */
    #[no_mangle]
    fn pdf_color_set(sc: *mut pdf_color, fc: *mut pdf_color);
    #[no_mangle]
    fn pdf_color_push(sc: *mut pdf_color, fc: *mut pdf_color);
    #[no_mangle]
    fn pdf_color_pop();
    #[no_mangle]
    fn pdf_color_get_current(sc: *mut *mut pdf_color, fc: *mut *mut pdf_color);
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
    /* Not in spt_t. */
    #[no_mangle]
    fn pdf_sprint_matrix(buf: *mut i8, p: *const pdf_tmatrix) -> libc::c_int;
    /* Place XObject */
    #[no_mangle]
    fn pdf_dev_put_image(
        xobj_id: libc::c_int,
        p: *mut transform_info,
        ref_x: libc::c_double,
        ref_y: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_reset_color(force: libc::c_int);
    #[no_mangle]
    fn pdf_dev_get_coord(xpos: *mut libc::c_double, ypos: *mut libc::c_double);
    #[no_mangle]
    fn pdf_dev_push_coord(xpos: libc::c_double, ypos: libc::c_double);
    #[no_mangle]
    fn pdf_dev_pop_coord();
    /* They just return PDF dictionary object.
     * Callers are completely responsible for doing right thing...
     */
    #[no_mangle]
    fn pdf_doc_get_dictionary(category: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_current_page_number() -> libc::c_int;
    /* Not really managing tree...
     * There should be something for number tree.
     */
    #[no_mangle]
    fn pdf_doc_add_names(
        category: *const i8,
        key: *const libc::c_void,
        keylen: libc::c_int,
        value: *mut pdf_obj,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_set_bop_content(str: *const i8, length: libc::c_uint);
    #[no_mangle]
    fn pdf_doc_set_eop_content(str: *const i8, length: libc::c_uint);
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const i8, length: libc::c_uint);
    /* Article thread */
    #[no_mangle]
    fn pdf_doc_begin_article(article_id: *const i8, info: *mut pdf_obj);
    #[no_mangle]
    fn pdf_doc_add_bead(
        article_id: *const i8,
        bead_id: *const i8,
        page_no: libc::c_int,
        rect: *const pdf_rect,
    );
    /* Bookmarks */
    #[no_mangle]
    fn pdf_doc_bookmarks_up() -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_bookmarks_down() -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_bookmarks_add(dict: *mut pdf_obj, is_open: libc::c_int);
    #[no_mangle]
    fn pdf_doc_bookmarks_depth() -> libc::c_int;
    /* Returns xobj_id of started xform. */
    #[no_mangle]
    fn pdf_doc_begin_grabbing(
        ident: *const i8,
        ref_x: libc::c_double,
        ref_y: libc::c_double,
        cropbox: *const pdf_rect,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_end_grabbing(attrib: *mut pdf_obj);
    /* Annotation */
    #[no_mangle]
    fn pdf_doc_add_annot(
        page_no: libc::c_uint,
        rect: *const pdf_rect,
        annot_dict: *mut pdf_obj,
        new_annot: libc::c_int,
    );
    /* Similar to bop_content */
    #[no_mangle]
    fn pdf_doc_set_bgcolor(color: *const pdf_color);
    #[no_mangle]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[no_mangle]
    fn pdf_dev_gsave() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_grestore() -> libc::c_int;
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    #[no_mangle]
    fn parse_ident(start: *mut *const i8, end: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_val_ident(
        start: *mut *const i8,
        end: *const i8,
    ) -> *mut i8;
    #[no_mangle]
    fn parse_opt_ident(
        start: *mut *const i8,
        end: *const i8,
    ) -> *mut i8;
    #[no_mangle]
    fn parse_pdf_dict(
        pp: *mut *const i8,
        endptr: *const i8,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_object(
        pp: *mut *const i8,
        endptr: *const i8,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_tainted_dict(
        pp: *mut *const i8,
        endptr: *const i8,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ximage_get_reference(xobj_id: libc::c_int) -> *mut pdf_obj;
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const i8, options: load_options) -> libc::c_int;
    #[no_mangle]
    fn spc_util_read_dimtrns(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        args: *mut spc_arg,
        syntax: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn spc_util_read_blahblah(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        page_no: *mut libc::c_int,
        bbox_type: *mut libc::c_int,
        args: *mut spc_arg,
    ) -> libc::c_int;
    #[no_mangle]
    fn spc_util_read_pdfcolor(
        spe: *mut spc_env,
        colorspec: *mut pdf_color,
        args: *mut spc_arg,
        defaultcolor: *mut pdf_color,
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
    fn UC_is_valid(ucv: i32) -> bool;
    #[no_mangle]
    fn UC_UTF16BE_is_valid_string(p: *const u8, endptr: *const u8) -> bool;
    #[no_mangle]
    fn UC_UTF8_is_valid_string(p: *const u8, endptr: *const u8) -> bool;
    #[no_mangle]
    fn UC_UTF16BE_encode_char(
        ucv: i32,
        dstpp: *mut *mut u8,
        endptr: *mut u8,
    ) -> size_t;
    #[no_mangle]
    fn UC_UTF8_decode_char(pp: *mut *const u8, endptr: *const u8) -> i32;
    #[no_mangle]
    fn CMap_decode(
        cmap: *mut CMap,
        inbuf: *mut *const u8,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut u8,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn CMap_cache_get(id: libc::c_int) -> *mut CMap;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const i8) -> libc::c_int;
}
pub type __ssize_t = i64;
pub type size_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: libc::c_double,
    pub y_user: libc::c_double,
    pub mag: libc::c_double,
    pub pg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const i8,
    pub endptr: *const i8,
    pub base: *const i8,
    pub command: *const i8,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const i8,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_pdf_ {
    pub annot_dict: *mut pdf_obj,
    pub lowest_level: libc::c_int,
    pub resourcemap: *mut ht_table,
    pub cd: tounicode,
    /* quasi-hack to get the primary input */
    /* For to-UTF16-BE conversion :( */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tounicode {
    pub cmap_id: libc::c_int,
    pub unescape_backslash: libc::c_int,
    pub taintkeys: *mut pdf_obj,
    /* An array of PDF names. */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: libc::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut i8,
    pub keylen: libc::c_int,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub struct pdf_color {
    pub num_components: libc::c_int,
    pub spot_color_name: *mut i8,
    pub values: [libc::c_double; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut i8,
    pub font_name: *mut i8,
    pub enc_name: *mut i8,
    pub charmap: C2RustUnnamed,
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
pub struct C2RustUnnamed {
    pub sfd_name: *mut i8,
    pub subfont_id: *mut i8,
}
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
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: libc::c_int,
    pub bbox_type: libc::c_int,
    pub dict: *mut pdf_obj,
}
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
/* PLEASE REMOVE THIS */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_map {
    pub type_0: libc::c_int,
    pub res_id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut i8,
    pub type_0: libc::c_int,
    pub wmode: libc::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_1,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: libc::c_int,
    pub profile: C2RustUnnamed_0,
    pub reverseMap: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: libc::c_int,
    pub len: size_t,
    pub code: *mut u8,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
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
pub struct CIDSysInfo {
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: libc::c_double,
    pub y: libc::c_double,
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
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const i8,
    mut prefix: *const i8,
) -> *const i8 {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const i8;
}
static mut _pdf_stat: spc_pdf_ = {
    let mut init = spc_pdf_ {
        annot_dict: 0 as *const pdf_obj as *mut pdf_obj,
        lowest_level: 255i32,
        resourcemap: 0 as *const ht_table as *mut ht_table,
        cd: {
            let mut init = tounicode {
                cmap_id: -1i32,
                unescape_backslash: 0i32,
                taintkeys: 0 as *const pdf_obj as *mut pdf_obj,
            };
            init
        },
    };
    init
};
/* PLEASE REMOVE THIS */
unsafe extern "C" fn hval_free(mut vp: *mut libc::c_void) {
    free(vp); /* unused */
}
unsafe extern "C" fn addresource(
    mut sd: *mut spc_pdf_,
    mut ident: *const i8,
    mut res_id: libc::c_int,
) -> libc::c_int {
    let mut r: *mut resource_map = 0 as *mut resource_map;
    if ident.is_null() || res_id < 0i32 {
        return -1i32;
    }
    r = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<resource_map>() as u64)
        as u32) as *mut resource_map;
    (*r).type_0 = 0i32;
    (*r).res_id = res_id;
    ht_append_table(
        (*sd).resourcemap,
        ident as *const libc::c_void,
        strlen(ident) as libc::c_int,
        r as *mut libc::c_void,
    );
    spc_push_object(ident, pdf_ximage_get_reference(res_id));
    return 0i32;
}
unsafe extern "C" fn findresource(
    mut sd: *mut spc_pdf_,
    mut ident: *const i8,
) -> libc::c_int {
    let mut r: *mut resource_map = 0 as *mut resource_map;
    if ident.is_null() {
        return -1i32;
    }
    r = ht_lookup_table(
        (*sd).resourcemap,
        ident as *const libc::c_void,
        strlen(ident) as libc::c_int,
    ) as *mut resource_map;
    return if !r.is_null() { (*r).res_id } else { -1i32 };
}
unsafe extern "C" fn spc_handler_pdfm__init(mut dp: *mut libc::c_void) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = dp as *mut spc_pdf_;
    /* The folllowing dictionary entry keys are considered as keys for
     * text strings. Be sure that string object is NOT always a text string.
     */
    static mut default_taintkeys: [*const i8; 12] = [
        b"Title\x00" as *const u8 as *const i8,
        b"Author\x00" as *const u8 as *const i8,
        b"Subject\x00" as *const u8 as *const i8,
        b"Keywords\x00" as *const u8 as *const i8,
        b"Creator\x00" as *const u8 as *const i8,
        b"Producer\x00" as *const u8 as *const i8,
        b"Contents\x00" as *const u8 as *const i8,
        b"Subj\x00" as *const u8 as *const i8,
        b"TU\x00" as *const u8 as *const i8,
        b"T\x00" as *const u8 as *const i8,
        b"TM\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut i: libc::c_int = 0;
    (*sd).annot_dict = 0 as *mut pdf_obj;
    (*sd).lowest_level = 255i32;
    (*sd).resourcemap = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<ht_table>() as u64)
        as u32) as *mut ht_table;
    ht_init_table(
        (*sd).resourcemap,
        Some(hval_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    (*sd).cd.taintkeys = pdf_new_array();
    i = 0i32;
    while !default_taintkeys[i as usize].is_null() {
        pdf_add_array(
            (*sd).cd.taintkeys,
            pdf_new_name(default_taintkeys[i as usize]),
        );
        i += 1
    }
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm__clean(mut dp: *mut libc::c_void) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = dp as *mut spc_pdf_;
    if !(*sd).annot_dict.is_null() {
        dpx_warning(b"Unbalanced bann and eann found.\x00" as *const u8 as *const i8);
        pdf_release_obj((*sd).annot_dict);
    }
    (*sd).lowest_level = 255i32;
    (*sd).annot_dict = 0 as *mut pdf_obj;
    if !(*sd).resourcemap.is_null() {
        ht_clear_table((*sd).resourcemap);
        free((*sd).resourcemap as *mut libc::c_void);
    }
    (*sd).resourcemap = 0 as *mut ht_table;
    pdf_release_obj((*sd).cd.taintkeys);
    (*sd).cd.taintkeys = 0 as *mut pdf_obj;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_pdfm_at_begin_document() -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    return spc_handler_pdfm__init(sd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn spc_pdfm_at_end_document() -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    return spc_handler_pdfm__clean(sd as *mut libc::c_void);
}
/* Dvipdfm specials */
unsafe extern "C" fn spc_handler_pdfm_bop(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    if (*args).curptr < (*args).endptr {
        pdf_doc_set_bop_content(
            (*args).curptr,
            (*args).endptr.wrapping_offset_from((*args).curptr) as i64 as libc::c_int
                as libc::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_eop(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    if (*args).curptr < (*args).endptr {
        pdf_doc_set_eop_content(
            (*args).curptr,
            (*args).endptr.wrapping_offset_from((*args).curptr) as i64 as libc::c_int
                as libc::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0i32;
}
/* Why should we have this kind of things? */
unsafe extern "C" fn safeputresdent(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut libc::c_void,
) -> libc::c_int {
    let mut key: *mut i8 = 0 as *mut i8;
    if !kp.is_null() && !vp.is_null() && !dp.is_null() {
    } else {
        __assert_fail(
            b"kp && vp && dp\x00" as *const u8 as *const i8,
            b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
            221i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[i8; 49]>(
                b"int safeputresdent(pdf_obj *, pdf_obj *, void *)\x00",
            ))
            .as_ptr(),
        );
    }
    key = pdf_name_value(kp);
    if !pdf_lookup_dict(dp as *mut pdf_obj, key).is_null() {
        dpx_warning(
            b"Object \"%s\" already defined in dict! (ignored)\x00" as *const u8
                as *const i8,
            key,
        );
    } else {
        pdf_add_dict(dp as *mut pdf_obj, pdf_link_obj(kp), pdf_link_obj(vp));
    }
    return 0i32;
}
unsafe extern "C" fn safeputresdict(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut libc::c_void,
) -> libc::c_int {
    let mut key: *mut i8 = 0 as *mut i8;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    if !kp.is_null() && !vp.is_null() && !dp.is_null() {
    } else {
        __assert_fail(
            b"kp && vp && dp\x00" as *const u8 as *const i8,
            b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
            243i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[i8; 49]>(
                b"int safeputresdict(pdf_obj *, pdf_obj *, void *)\x00",
            ))
            .as_ptr(),
        );
    }
    key = pdf_name_value(kp);
    dict = pdf_lookup_dict(dp as *mut pdf_obj, key);
    if pdf_obj_typeof(vp) == 9i32 {
        pdf_add_dict(dp as *mut pdf_obj, pdf_new_name(key), pdf_link_obj(vp));
    } else if pdf_obj_typeof(vp) == 6i32 {
        if !dict.is_null() {
            pdf_foreach_dict(
                vp,
                Some(
                    safeputresdent
                        as unsafe extern "C" fn(
                            _: *mut pdf_obj,
                            _: *mut pdf_obj,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                dict as *mut libc::c_void,
            );
        } else {
            pdf_add_dict(dp as *mut pdf_obj, pdf_new_name(key), pdf_link_obj(vp));
        }
    } else {
        dpx_warning(
            b"Invalid type (not DICT) for page/form resource dict entry: key=\"%s\"\x00"
                as *const u8 as *const i8,
            key,
        );
        return -1i32;
    }
    return 0i32;
}
/* Think what happens if you do
 *
 *  pdf:put @resources << /Font << >> >>
 *
 */
unsafe extern "C" fn spc_handler_pdfm_put(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut obj1: *mut pdf_obj = 0 as *mut pdf_obj; /* put obj2 into obj1 */
    let mut obj2: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut error: libc::c_int = 0i32;
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    ident = parse_opt_ident(&mut (*ap).curptr, (*ap).endptr);
    if ident.is_null() {
        spc_warn(
            spe,
            b"Missing object identifier.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    obj1 = spc_lookup_object(ident);
    if obj1.is_null() {
        spc_warn(
            spe,
            b"Specified object not exist: %s\x00" as *const u8 as *const i8,
            ident,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    obj2 = parse_pdf_object(&mut (*ap).curptr, (*ap).endptr, 0 as *mut pdf_file);
    if obj2.is_null() {
        spc_warn(
            spe,
            b"Missing (an) object(s) to put into \"%s\"!\x00" as *const u8 as *const i8,
            ident,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    match pdf_obj_typeof(obj1) {
        6 => {
            if pdf_obj_typeof(obj2) != 6i32 {
                spc_warn(
                    spe,
                    b"Inconsistent object type for \"put\" (expecting DICT): %s\x00" as *const u8
                        as *const i8,
                    ident,
                );
                error = -1i32
            } else if streq_ptr(ident, b"resources\x00" as *const u8 as *const i8) {
                error = pdf_foreach_dict(
                    obj2,
                    Some(
                        safeputresdict
                            as unsafe extern "C" fn(
                                _: *mut pdf_obj,
                                _: *mut pdf_obj,
                                _: *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    obj1 as *mut libc::c_void,
                )
            } else {
                pdf_merge_dict(obj1, obj2);
            }
        }
        7 => {
            if pdf_obj_typeof(obj2) == 6i32 {
                pdf_merge_dict(pdf_stream_dict(obj1), obj2);
            } else if pdf_obj_typeof(obj2) == 7i32 {
                spc_warn(
                    spe,
                    b"\"put\" operation not supported for STREAM <- STREAM: %s\x00" as *const u8
                        as *const i8,
                    ident,
                );
                error = -1i32
            } else {
                spc_warn(
                    spe,
                    b"Invalid type: expecting a DICT or STREAM: %s\x00" as *const u8
                        as *const i8,
                    ident,
                );
                error = -1i32
            }
        }
        5 => {
            /* dvipdfm */
            pdf_add_array(obj1, pdf_link_obj(obj2));
            while (*ap).curptr < (*ap).endptr {
                let mut obj3: *mut pdf_obj =
                    parse_pdf_object(&mut (*ap).curptr, (*ap).endptr, 0 as *mut pdf_file);
                if obj3.is_null() {
                    break;
                }
                pdf_add_array(obj1, obj3);
                skip_white(&mut (*ap).curptr, (*ap).endptr);
            }
        }
        _ => {
            spc_warn(
                spe,
                b"Can\'t \"put\" object into non-DICT/STREAM/ARRAY type object: %s\x00" as *const u8
                    as *const i8,
                ident,
            );
            error = -1i32
        }
    }
    pdf_release_obj(obj2);
    free(ident as *mut libc::c_void);
    return error;
}
/* For pdf:tounicode support
 * This feature is provided for convenience. TeX can't do
 * input encoding conversion.
 */
unsafe extern "C" fn reencodestring(
    mut cmap: *mut CMap,
    mut instring: *mut pdf_obj,
) -> libc::c_int {
    let mut wbuf: [u8; 4096] = [0; 4096];
    let mut obufcur: *mut u8 = 0 as *mut u8;
    let mut inbufcur: *const u8 = 0 as *const u8;
    let mut inbufleft: size_t = 0;
    let mut obufleft: size_t = 0;
    if cmap.is_null() || instring.is_null() {
        return 0i32;
    }
    inbufleft = pdf_string_length(instring) as size_t;
    inbufcur = pdf_string_value(instring) as *const u8;
    wbuf[0] = 0xfei32 as u8;
    wbuf[1] = 0xffi32 as u8;
    obufcur = wbuf.as_mut_ptr().offset(2);
    obufleft = (4096i32 - 2i32) as size_t;
    CMap_decode(
        cmap,
        &mut inbufcur,
        &mut inbufleft,
        &mut obufcur,
        &mut obufleft,
    );
    if inbufleft > 0i32 as u64 {
        return -1i32;
    }
    pdf_set_string(
        instring,
        wbuf.as_mut_ptr(),
        (4096i32 as u64).wrapping_sub(obufleft),
    );
    return 0i32;
}
unsafe extern "C" fn maybe_reencode_utf8(mut instring: *mut pdf_obj) -> libc::c_int {
    let mut inbuf: *mut u8 = 0 as *mut u8;
    let mut inlen: libc::c_int = 0;
    let mut non_ascii: libc::c_int = 0i32;
    let mut cp: *const u8 = 0 as *const u8;
    let mut op: *mut u8 = 0 as *mut u8;
    let mut wbuf: [u8; 4096] = [0; 4096];
    if instring.is_null() {
        return 0i32;
    }
    inlen = pdf_string_length(instring) as libc::c_int;
    inbuf = pdf_string_value(instring) as *mut u8;
    /* check if the input string is strictly ASCII */
    cp = inbuf; /* no need to reencode ASCII strings */
    while cp < inbuf.offset(inlen as isize) as *const u8 {
        if *cp as libc::c_int > 127i32 {
            non_ascii = 1i32
        }
        cp = cp.offset(1)
    }
    if non_ascii == 0i32 {
        return 0i32;
    }
    /* Check if the input string is valid UTF8 string
     * This routine may be called against non-text strings.
     * We need to re-encode string only when string is a text string
     * endcoded in UTF8.
     */
    if !UC_UTF8_is_valid_string(inbuf, inbuf.offset(inlen as isize)) {
        return 0i32;
    } else {
        if *inbuf.offset(0) as libc::c_int == 0xfei32
            && *inbuf.offset(1) as libc::c_int == 0xffi32
            && UC_UTF16BE_is_valid_string(inbuf.offset(2), inbuf.offset(inlen as isize))
                as libc::c_int
                != 0
        {
            return 0i32;
        }
    } /* no need to reencode UTF16BE with BOM */
    cp = inbuf; /* out of valid Unicode range, give up (redundant) */
    op = wbuf.as_mut_ptr();
    let fresh0 = op;
    op = op.offset(1);
    *fresh0 = 0xfei32 as u8;
    let fresh1 = op;
    op = op.offset(1);
    *fresh1 = 0xffi32 as u8;
    while cp < inbuf.offset(inlen as isize) as *const u8 {
        let mut usv: i32 = 0;
        let mut len: libc::c_int = 0;
        usv = UC_UTF8_decode_char(&mut cp, inbuf.offset(inlen as isize));
        if !UC_is_valid(usv) {
            return -1i32;
        }
        len = UC_UTF16BE_encode_char(usv, &mut op, wbuf.as_mut_ptr().offset(4096)) as libc::c_int;
        if len == 0i32 {
            return -1i32;
        }
    }
    pdf_set_string(
        instring,
        wbuf.as_mut_ptr(),
        op.wrapping_offset_from(wbuf.as_mut_ptr()) as i64 as size_t,
    );
    return 0i32;
}
/* The purpose of this routine is to check if given string object is
 * surely an object for *text* strings. It does not do a complete check
 * but does a quick check. Please add entries for taintkeys if you have found
 * additional dictionary entries which is considered as a text string.
 */
unsafe extern "C" fn needreencode(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut cd: *mut tounicode,
) -> libc::c_int {
    let mut r: libc::c_int = 0i32;
    let mut i: libc::c_uint = 0;
    let mut tk: *mut pdf_obj = 0 as *mut pdf_obj;
    if !cd.is_null() && !(*cd).taintkeys.is_null() {
    } else {
        __assert_fail(
            b"cd && cd->taintkeys\x00" as *const u8 as *const i8,
            b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
            459i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int needreencode(pdf_obj *, pdf_obj *, struct tounicode *)\x00",
            ))
            .as_ptr(),
        );
    }
    if pdf_obj_typeof(kp) == 4i32 {
    } else {
        __assert_fail(
            b"pdf_obj_typeof(kp) == PDF_NAME\x00" as *const u8 as *const i8,
            b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
            460i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int needreencode(pdf_obj *, pdf_obj *, struct tounicode *)\x00",
            ))
            .as_ptr(),
        );
    }
    if pdf_obj_typeof(vp) == 3i32 {
    } else {
        __assert_fail(
            b"pdf_obj_typeof(vp) == PDF_STRING\x00" as *const u8 as *const i8,
            b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
            461i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int needreencode(pdf_obj *, pdf_obj *, struct tounicode *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32 as libc::c_uint;
    while i < pdf_array_length((*cd).taintkeys) {
        tk = pdf_get_array((*cd).taintkeys, i as libc::c_int);
        if !tk.is_null() && pdf_obj_typeof(tk) == 4i32 {
        } else {
            __assert_fail(
                b"tk && pdf_obj_typeof(tk) == PDF_NAME\x00" as *const u8 as *const i8,
                b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
                465i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                    b"int needreencode(pdf_obj *, pdf_obj *, struct tounicode *)\x00",
                ))
                .as_ptr(),
            );
        }
        if streq_ptr(pdf_name_value(kp), pdf_name_value(tk)) {
            r = 1i32;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    if r != 0 {
        /* Check UTF-16BE BOM. */
        if pdf_string_length(vp) >= 2i32 as libc::c_uint
            && memcmp(
                pdf_string_value(vp),
                b"\xfe\xff\x00" as *const u8 as *const i8 as *const libc::c_void,
                2i32 as u64,
            ) == 0
        {
            r = 0i32
        }
    } /* continue */
    return r;
}
unsafe extern "C" fn modstrings(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut libc::c_void,
) -> libc::c_int {
    let mut r: libc::c_int = 0i32;
    let mut cd: *mut tounicode = dp as *mut tounicode;
    if pdf_obj_typeof(kp) == 4i32 {
    } else {
        __assert_fail(
            b"pdf_obj_typeof(kp) == PDF_NAME\x00" as *const u8 as *const i8,
            b"dpx-spc_pdfm.c\x00" as *const u8 as *const i8,
            487i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[i8; 45]>(
                b"int modstrings(pdf_obj *, pdf_obj *, void *)\x00",
            ))
            .as_ptr(),
        );
    }
    match pdf_obj_typeof(vp) {
        3 => {
            if !cd.is_null() && (*cd).cmap_id >= 0i32 && !(*cd).taintkeys.is_null() {
                let mut cmap: *mut CMap = CMap_cache_get((*cd).cmap_id);
                if needreencode(kp, vp, cd) != 0 {
                    r = reencodestring(cmap, vp)
                }
            } else if is_xdv != 0 && !cd.is_null() && !(*cd).taintkeys.is_null() {
                /* Please fix this... PDF string object is not always a text string.
                 * needreencode() is assumed to do a simple check if given string
                 * object is actually a text string.
                 */
                if needreencode(kp, vp, cd) != 0 {
                    r = maybe_reencode_utf8(vp)
                }
            }
            if r < 0i32 {
                /* error occured... */
                dpx_warning(
                    b"Failed to convert input string to UTF16...\x00" as *const u8
                        as *const i8,
                );
            }
        }
        6 => {
            r = pdf_foreach_dict(
                vp,
                Some(
                    modstrings
                        as unsafe extern "C" fn(
                            _: *mut pdf_obj,
                            _: *mut pdf_obj,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                dp,
            )
        }
        7 => {
            r = pdf_foreach_dict(
                pdf_stream_dict(vp),
                Some(
                    modstrings
                        as unsafe extern "C" fn(
                            _: *mut pdf_obj,
                            _: *mut pdf_obj,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                dp,
            )
        }
        _ => {}
    }
    return r;
}
unsafe extern "C" fn parse_pdf_dict_with_tounicode(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut cd: *mut tounicode,
) -> *mut pdf_obj {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    /* disable this test for XDV files, as we do UTF8 reencoding with no cmap */
    if is_xdv == 0 && (*cd).cmap_id < 0i32 {
        return parse_pdf_dict(pp, endptr, 0 as *mut pdf_file);
    }
    /* :( */
    if !cd.is_null() && (*cd).unescape_backslash != 0 {
        dict = parse_pdf_tainted_dict(pp, endptr)
    } else {
        dict = parse_pdf_dict(pp, endptr, 0 as *mut pdf_file)
    }
    if !dict.is_null() {
        pdf_foreach_dict(
            dict,
            Some(
                modstrings
                    as unsafe extern "C" fn(
                        _: *mut pdf_obj,
                        _: *mut pdf_obj,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            cd as *mut libc::c_void,
        );
    }
    return dict;
}
unsafe extern "C" fn spc_handler_pdfm_annot(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut annot_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut rect: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut ti: transform_info = transform_info {
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
    skip_white(&mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0) as libc::c_int == '@' as i32 {
        ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
        skip_white(&mut (*args).curptr, (*args).endptr);
    }
    transform_info_clear(&mut ti);
    if spc_util_read_dimtrns(spe, &mut ti, args, 0i32) < 0i32 {
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    if ti.flags & 1i32 << 0i32 != 0
        && (ti.flags & 1i32 << 1i32 != 0 || ti.flags & 1i32 << 2i32 != 0)
    {
        spc_warn(
            spe,
            b"You can\'t specify both bbox and width/height.\x00" as *const u8
                as *const i8,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    annot_dict = parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
    if annot_dict.is_null() {
        spc_warn(
            spe,
            b"Could not find dictionary object.\x00" as *const u8 as *const i8,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    } else {
        if !(!annot_dict.is_null() && pdf_obj_typeof(annot_dict) == 6i32) {
            spc_warn(
                spe,
                b"Invalid type: not dictionary object.\x00" as *const u8 as *const i8,
            );
            free(ident as *mut libc::c_void);
            pdf_release_obj(annot_dict);
            return -1i32;
        }
    }
    cp.x = (*spe).x_user;
    cp.y = (*spe).y_user;
    pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
    if ti.flags & 1i32 << 0i32 != 0 {
        rect.llx = ti.bbox.llx + cp.x;
        rect.lly = ti.bbox.lly + cp.y;
        rect.urx = ti.bbox.urx + cp.x;
        rect.ury = ti.bbox.ury + cp.y
    } else {
        rect.llx = cp.x;
        rect.lly = cp.y - (*spe).mag * ti.depth;
        rect.urx = cp.x + (*spe).mag * ti.width;
        rect.ury = cp.y + (*spe).mag * ti.height
    }
    /* Order is important... */
    if !ident.is_null() {
        spc_push_object(ident, pdf_link_obj(annot_dict));
    }
    /* Add this reference. */
    pdf_doc_add_annot(
        pdf_doc_current_page_number() as libc::c_uint,
        &mut rect,
        annot_dict,
        1i32,
    );
    if !ident.is_null() {
        spc_flush_object(ident);
        free(ident as *mut libc::c_void);
    }
    pdf_release_obj(annot_dict);
    return 0i32;
}
/* NOTE: This can't have ident. See "Dvipdfm User's Manual". */
unsafe extern "C" fn spc_handler_pdfm_bann(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut error: libc::c_int = 0i32;
    if !(*sd).annot_dict.is_null() {
        spc_warn(
            spe,
            b"Can\'t begin an annotation when one is pending.\x00" as *const u8
                as *const i8,
        );
        return -1i32;
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    (*sd).annot_dict =
        parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
    if (*sd).annot_dict.is_null() {
        spc_warn(
            spe,
            b"Ignoring annotation with invalid dictionary.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    } else {
        if !(!(*sd).annot_dict.is_null() && pdf_obj_typeof((*sd).annot_dict) == 6i32) {
            spc_warn(
                spe,
                b"Invalid type: not a dictionary object.\x00" as *const u8 as *const i8,
            );
            pdf_release_obj((*sd).annot_dict);
            (*sd).annot_dict = 0 as *mut pdf_obj;
            return -1i32;
        }
    }
    error = spc_begin_annot(spe, (*sd).annot_dict);
    return error;
}
unsafe extern "C" fn spc_handler_pdfm_eann(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut error: libc::c_int = 0i32;
    if (*sd).annot_dict.is_null() {
        spc_warn(
            spe,
            b"Tried to end an annotation without starting one!\x00" as *const u8
                as *const i8,
        );
        return -1i32;
    }
    error = spc_end_annot(spe);
    pdf_release_obj((*sd).annot_dict);
    (*sd).annot_dict = 0 as *mut pdf_obj;
    return error;
}
/* Color:.... */
unsafe extern "C" fn spc_handler_pdfm_bcolor(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut fc: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    let mut sc: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    let mut pfc: *mut pdf_color = 0 as *mut pdf_color;
    let mut psc: *mut pdf_color = 0 as *mut pdf_color;
    pdf_color_get_current(&mut psc, &mut pfc);
    error = spc_util_read_pdfcolor(spe, &mut fc, ap, pfc);
    if error == 0 {
        if (*ap).curptr < (*ap).endptr {
            error = spc_util_read_pdfcolor(spe, &mut sc, ap, psc)
        } else {
            pdf_color_copycolor(&mut sc, &mut fc);
        }
    }
    if error != 0 {
        spc_warn(
            spe,
            b"Invalid color specification?\x00" as *const u8 as *const i8,
        );
    } else {
        pdf_color_push(&mut sc, &mut fc);
        /* save currentcolor */
    }
    return error;
}
/*
 * This special changes the current color without clearing the color stack.
 * It therefore differs from "color rgb 1 0 0".
 */
unsafe extern "C" fn spc_handler_pdfm_scolor(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut fc: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    let mut sc: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    let mut pfc: *mut pdf_color = 0 as *mut pdf_color;
    let mut psc: *mut pdf_color = 0 as *mut pdf_color;
    pdf_color_get_current(&mut psc, &mut pfc);
    error = spc_util_read_pdfcolor(spe, &mut fc, ap, pfc);
    if error == 0 {
        if (*ap).curptr < (*ap).endptr {
            error = spc_util_read_pdfcolor(spe, &mut sc, ap, psc)
        } else {
            pdf_color_copycolor(&mut sc, &mut fc);
        }
    }
    if error != 0 {
        spc_warn(
            spe,
            b"Invalid color specification?\x00" as *const u8 as *const i8,
        );
    } else {
        pdf_color_set(&mut sc, &mut fc);
    }
    return error;
}
unsafe extern "C" fn spc_handler_pdfm_ecolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    pdf_color_pop();
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_btrans(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut ti: transform_info = transform_info {
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
    transform_info_clear(&mut ti);
    if spc_util_read_dimtrns(spe, &mut ti, args, 0i32) < 0i32 {
        return -1i32;
    }
    /* Create transformation matrix */
    M.a = ti.matrix.a;
    M.b = ti.matrix.b;
    M.c = ti.matrix.c;
    M.d = ti.matrix.d;
    M.e = ti.matrix.e;
    M.f = ti.matrix.f;
    M.e += (1.0f64 - M.a) * (*spe).x_user - M.c * (*spe).y_user;
    M.f += (1.0f64 - M.d) * (*spe).y_user - M.b * (*spe).x_user;
    pdf_dev_gsave();
    pdf_dev_concat(&mut M);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_etrans(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    pdf_dev_grestore();
    /*
     * Unfortunately, the following line is necessary in case
     * of a color change inside of the save/restore pair.
     * (Font changes are automatically corrected by pdf_dev_grestore().)
     * Anything that was done there must be redone, so in effect,
     * we make no assumptions about what fonts. We act like we are
     * starting a new page.
     */
    pdf_dev_reset_color(0i32);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_outline(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut item_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut level: libc::c_int = 0;
    let mut is_open: libc::c_int = -1i32;
    let mut current_depth: libc::c_int = 0;
    skip_white(&mut (*args).curptr, (*args).endptr);
    /*
     * pdf:outline is extended to support open/close feature
     *
     * pdf:outline 1 ... (as DVIPDFM)
     * pdf:outline [] 1 ... (open bookmark)
     * pdf:outline [-] 1 ... (closed bookmark)
     */
    if (*args).curptr.offset(3) < (*args).endptr && *(*args).curptr as libc::c_int == '[' as i32 {
        (*args).curptr = (*args).curptr.offset(1);
        if *(*args).curptr as libc::c_int == '-' as i32 {
            (*args).curptr = (*args).curptr.offset(1)
        } else {
            is_open = 1i32
        }
        (*args).curptr = (*args).curptr.offset(1)
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    tmp = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if tmp.is_null() {
        spc_warn(
            spe,
            b"Missing number for outline item depth.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    } else {
        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
            pdf_release_obj(tmp);
            spc_warn(
                spe,
                b"Expecting number for outline item depth.\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
    }
    item_dict = 0 as *mut pdf_obj;
    level = pdf_number_value(tmp) as libc::c_int;
    pdf_release_obj(tmp);
    /* What is this? Starting at level 3 and can go down to level 1?
     *
     * Here is the original comment:
     *  Make sure we know where the starting level is
     *
     * NOTE: added
     *  We need this for converting pages from 3rd to... :(
     */
    (*sd).lowest_level = if (*sd).lowest_level < level {
        (*sd).lowest_level
    } else {
        level
    };
    level += 1i32 - (*sd).lowest_level;
    item_dict = parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
    if item_dict.is_null() {
        spc_warn(
            spe,
            b"Ignoring invalid dictionary.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    current_depth = pdf_doc_bookmarks_depth();
    if current_depth > level {
        loop {
            let fresh2 = current_depth;
            current_depth = current_depth - 1;
            if !(fresh2 > level) {
                break;
            }
            pdf_doc_bookmarks_up();
        }
    } else if current_depth < level {
        loop {
            let fresh3 = current_depth;
            current_depth = current_depth + 1;
            if !(fresh3 < level) {
                break;
            }
            pdf_doc_bookmarks_down();
        }
    }
    pdf_doc_bookmarks_add(item_dict, is_open);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_article(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut info_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    skip_white(&mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        spc_warn(
            spe,
            b"Article name expected but not found.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    info_dict = parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
    if info_dict.is_null() {
        spc_warn(
            spe,
            b"Ignoring article with invalid info dictionary.\x00" as *const u8
                as *const i8,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    pdf_doc_begin_article(ident, pdf_link_obj(info_dict));
    spc_push_object(ident, info_dict);
    free(ident as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_bead(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut article: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut article_info: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut article_name: *mut i8 = 0 as *mut i8;
    let mut rect: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut page_no: libc::c_int = 0;
    let mut ti: transform_info = transform_info {
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
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    skip_white(&mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0) as libc::c_int != '@' as i32 {
        spc_warn(
            spe,
            b"Article identifier expected but not found.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    article_name = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if article_name.is_null() {
        spc_warn(
            spe,
            b"Article reference expected but not found.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    /* If okay so far, try to get a bounding box */
    transform_info_clear(&mut ti);
    if spc_util_read_dimtrns(spe, &mut ti, args, 0i32) < 0i32 {
        free(article_name as *mut libc::c_void);
        return -1i32;
    }
    if ti.flags & 1i32 << 0i32 != 0
        && (ti.flags & 1i32 << 1i32 != 0 || ti.flags & 1i32 << 2i32 != 0)
    {
        spc_warn(
            spe,
            b"You can\'t specify both bbox and width/height.\x00" as *const u8
                as *const i8,
        );
        free(article_name as *mut libc::c_void);
        return -1i32;
    }
    cp.x = (*spe).x_user;
    cp.y = (*spe).y_user;
    pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
    if ti.flags & 1i32 << 0i32 != 0 {
        rect.llx = ti.bbox.llx + cp.x;
        rect.lly = ti.bbox.lly + cp.y;
        rect.urx = ti.bbox.urx + cp.x;
        rect.ury = ti.bbox.ury + cp.y
    } else {
        rect.llx = cp.x;
        rect.lly = cp.y - (*spe).mag * ti.depth;
        rect.urx = cp.x + (*spe).mag * ti.width;
        rect.ury = cp.y + (*spe).mag * ti.height
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0) as libc::c_int != '<' as i32 {
        article_info = pdf_new_dict()
    } else {
        article_info =
            parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
        if article_info.is_null() {
            spc_warn(
                spe,
                b"Error in reading dictionary.\x00" as *const u8 as *const i8,
            );
            free(article_name as *mut libc::c_void);
            return -1i32;
        }
    }
    /* Does this article exist yet */
    article = spc_lookup_object(article_name);
    if !article.is_null() {
        pdf_merge_dict(article, article_info);
        pdf_release_obj(article_info);
    } else {
        pdf_doc_begin_article(article_name, pdf_link_obj(article_info));
        spc_push_object(article_name, article_info);
    }
    page_no = pdf_doc_current_page_number();
    pdf_doc_add_bead(article_name, 0 as *const i8, page_no, &mut rect);
    free(article_name as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_image(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut xobj_id: libc::c_int = 0;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut fspec: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut ti: transform_info = transform_info {
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
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    skip_white(&mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0) as libc::c_int == '@' as i32 {
        ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
        xobj_id = findresource(sd, ident);
        if xobj_id >= 0i32 {
            spc_warn(
                spe,
                b"Object reference name for image \"%s\" already used.\x00" as *const u8
                    as *const i8,
                ident,
            );
            free(ident as *mut libc::c_void);
            return -1i32;
        }
    }
    /* 2015/12/29
     * There should not be "page" and "pagebox" in read_dimtrns().
     * It is for reading "dimensions" and "transformations" and "page" is
     * completely unrelated.
     */
    transform_info_clear(&mut ti);
    if spc_util_read_blahblah(
        spe,
        &mut ti,
        &mut options.page_no,
        &mut options.bbox_type,
        args,
    ) < 0i32
    {
        spc_warn(
            spe,
            b"Reading option field in pdf:image failed.\x00" as *const u8 as *const i8,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    fspec = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if fspec.is_null() {
        spc_warn(
            spe,
            b"Missing filename string for pdf:image.\x00" as *const u8 as *const i8,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    } else {
        if !(!fspec.is_null() && pdf_obj_typeof(fspec) == 3i32) {
            spc_warn(
                spe,
                b"Missing filename string for pdf:image.\x00" as *const u8 as *const i8,
            );
            pdf_release_obj(fspec);
            free(ident as *mut libc::c_void);
            return -1i32;
        }
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        options.dict = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file)
    }
    xobj_id = pdf_ximage_findresource(pdf_string_value(fspec) as *const i8, options);
    if xobj_id < 0i32 {
        spc_warn(
            spe,
            b"Could not find image resource...\x00" as *const u8 as *const i8,
        );
        pdf_release_obj(fspec);
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    if ti.flags & 1i32 << 4i32 == 0 {
        pdf_dev_put_image(xobj_id, &mut ti, (*spe).x_user, (*spe).y_user);
    }
    if !ident.is_null() {
        addresource(sd, ident, xobj_id);
        free(ident as *mut libc::c_void);
    }
    pdf_release_obj(fspec);
    return 0i32;
}
/* Use do_names instead. */
unsafe extern "C" fn spc_handler_pdfm_dest(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut name: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut array: *mut pdf_obj = 0 as *mut pdf_obj;
    skip_white(&mut (*args).curptr, (*args).endptr);
    name = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if name.is_null() {
        spc_warn(
            spe,
            b"PDF string expected for destination name but not found.\x00" as *const u8
                as *const i8,
        );
        return -1i32;
    } else {
        if !(!name.is_null() && pdf_obj_typeof(name) == 3i32) {
            spc_warn(
                spe,
                b"PDF string expected for destination name but invalid type.\x00" as *const u8
                    as *const i8,
            );
            pdf_release_obj(name);
            return -1i32;
        }
    }
    array = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if array.is_null() {
        spc_warn(
            spe,
            b"No destination specified for pdf:dest.\x00" as *const u8 as *const i8,
        );
        pdf_release_obj(name);
        return -1i32;
    } else {
        if !(!array.is_null() && pdf_obj_typeof(array) == 5i32) {
            spc_warn(
                spe,
                b"Destination not specified as an array object!\x00" as *const u8
                    as *const i8,
            );
            pdf_release_obj(name);
            pdf_release_obj(array);
            return -1i32;
        }
    }
    pdf_doc_add_names(
        b"Dests\x00" as *const u8 as *const i8,
        pdf_string_value(name),
        pdf_string_length(name) as libc::c_int,
        array,
    );
    pdf_release_obj(name);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_names(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut category: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut key: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    category = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if category.is_null() {
        spc_warn(
            spe,
            b"PDF name expected but not found.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    } else {
        if !(!category.is_null() && pdf_obj_typeof(category) == 4i32) {
            spc_warn(
                spe,
                b"PDF name expected but not found.\x00" as *const u8 as *const i8,
            );
            pdf_release_obj(category);
            return -1i32;
        }
    }
    tmp = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if tmp.is_null() {
        spc_warn(
            spe,
            b"PDF object expected but not found.\x00" as *const u8 as *const i8,
        );
        pdf_release_obj(category);
        return -1i32;
    } else {
        if !tmp.is_null() && pdf_obj_typeof(tmp) == 5i32 {
            size = pdf_array_length(tmp) as libc::c_int;
            if size % 2i32 != 0i32 {
                spc_warn(
                    spe,
                    b"Array size not multiple of 2 for pdf:names.\x00" as *const u8
                        as *const i8,
                );
                pdf_release_obj(category);
                pdf_release_obj(tmp);
                return -1i32;
            }
            i = 0i32;
            while i < size / 2i32 {
                key = pdf_get_array(tmp, 2i32 * i);
                value = pdf_get_array(tmp, 2i32 * i + 1i32);
                if !(!key.is_null() && pdf_obj_typeof(key) == 3i32) {
                    spc_warn(
                        spe,
                        b"Name tree key must be string.\x00" as *const u8 as *const i8,
                    );
                    pdf_release_obj(category);
                    pdf_release_obj(tmp);
                    return -1i32;
                } else {
                    if pdf_doc_add_names(
                        pdf_name_value(category),
                        pdf_string_value(key),
                        pdf_string_length(key) as libc::c_int,
                        pdf_link_obj(value),
                    ) < 0i32
                    {
                        spc_warn(
                            spe,
                            b"Failed to add Name tree entry...\x00" as *const u8
                                as *const i8,
                        );
                        pdf_release_obj(category);
                        pdf_release_obj(tmp);
                        return -1i32;
                    }
                }
                i += 1
            }
            pdf_release_obj(tmp);
        } else if !tmp.is_null() && pdf_obj_typeof(tmp) == 3i32 {
            key = tmp;
            value = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
            if value.is_null() {
                pdf_release_obj(category);
                pdf_release_obj(key);
                spc_warn(
                    spe,
                    b"PDF object expected but not found.\x00" as *const u8 as *const i8,
                );
                return -1i32;
            }
            if pdf_doc_add_names(
                pdf_name_value(category),
                pdf_string_value(key),
                pdf_string_length(key) as libc::c_int,
                value,
            ) < 0i32
            {
                spc_warn(
                    spe,
                    b"Failed to add Name tree entry...\x00" as *const u8 as *const i8,
                );
                pdf_release_obj(category);
                pdf_release_obj(key);
                return -1i32;
            }
            pdf_release_obj(key);
        } else {
            pdf_release_obj(tmp);
            pdf_release_obj(category);
            spc_warn(
                spe,
                b"Invalid object type for pdf:names.\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
    }
    pdf_release_obj(category);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_docinfo(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut docinfo: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    dict = parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
    if dict.is_null() {
        spc_warn(
            spe,
            b"Dictionary object expected but not found.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    docinfo = pdf_doc_get_dictionary(b"Info\x00" as *const u8 as *const i8);
    pdf_merge_dict(docinfo, dict);
    pdf_release_obj(dict);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_docview(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut catalog: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut pref_old: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut pref_add: *mut pdf_obj = 0 as *mut pdf_obj;
    dict = parse_pdf_dict_with_tounicode(&mut (*args).curptr, (*args).endptr, &mut (*sd).cd);
    if dict.is_null() {
        spc_warn(
            spe,
            b"Dictionary object expected but not found.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    catalog = pdf_doc_get_dictionary(b"Catalog\x00" as *const u8 as *const i8);
    /* Avoid overriding whole ViewerPreferences */
    pref_old = pdf_lookup_dict(
        catalog,
        b"ViewerPreferences\x00" as *const u8 as *const i8,
    ); /* Close all? */
    pref_add = pdf_lookup_dict(
        dict,
        b"ViewerPreferences\x00" as *const u8 as *const i8,
    );
    if !pref_old.is_null() && !pref_add.is_null() {
        pdf_merge_dict(pref_old, pref_add);
        pdf_remove_dict(
            dict,
            b"ViewerPreferences\x00" as *const u8 as *const i8,
        );
    }
    pdf_merge_dict(catalog, dict);
    pdf_release_obj(dict);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_close(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut ident: *mut i8 = 0 as *mut i8;
    skip_white(&mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if !ident.is_null() {
        spc_flush_object(ident);
        free(ident as *mut libc::c_void);
    } else {
        spc_clear_objects();
    }
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_object(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut object: *mut pdf_obj = 0 as *mut pdf_obj;
    skip_white(&mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        spc_warn(
            spe,
            b"Could not find a object identifier.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    object = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if object.is_null() {
        spc_warn(
            spe,
            b"Could not find an object definition for \"%s\".\x00" as *const u8
                as *const i8,
            ident,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    } else {
        spc_push_object(ident, object);
    }
    free(ident as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_content(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        let mut M: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        M.a = 1.0f64;
        M.b = 0.0f64;
        M.c = 0.0f64;
        M.d = 1.0f64;
        M.e = (*spe).x_user;
        M.f = (*spe).y_user;
        let fresh4 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh4 as isize) = ' ' as i32 as i8;
        let fresh5 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh5 as isize) = 'q' as i32 as i8;
        let fresh6 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh6 as isize) = ' ' as i32 as i8;
        len += pdf_sprint_matrix(work_buffer.as_mut_ptr().offset(len as isize), &mut M);
        let fresh7 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh7 as isize) = ' ' as i32 as i8;
        let fresh8 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh8 as isize) = 'c' as i32 as i8;
        let fresh9 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh9 as isize) = 'm' as i32 as i8;
        let fresh10 = len;
        len = len + 1;
        *work_buffer.as_mut_ptr().offset(fresh10 as isize) = ' ' as i32 as i8;
        /* op: Q */
        pdf_doc_add_page_content(work_buffer.as_mut_ptr(), len as libc::c_uint); /* op: q cm */
        len = (*args).endptr.wrapping_offset_from((*args).curptr) as i64 as libc::c_int; /* op: ANY */
        pdf_doc_add_page_content((*args).curptr, len as libc::c_uint); /* op: */
        pdf_doc_add_page_content(
            b" Q\x00" as *const u8 as *const i8,
            2i32 as libc::c_uint,
        ); /* op: ANY */
    } /* op: */
    (*args).curptr = (*args).endptr; /* op: ANY */
    return 0i32; /*kpse_find_pict(instring);*/
}
unsafe extern "C" fn spc_handler_pdfm_literal(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut direct: libc::c_int = 0i32;
    skip_white(&mut (*args).curptr, (*args).endptr);
    while (*args).curptr < (*args).endptr {
        if (*args).curptr.offset(7) <= (*args).endptr
            && !strstartswith(
                (*args).curptr,
                b"reverse\x00" as *const u8 as *const i8,
            )
            .is_null()
        {
            (*args).curptr = (*args).curptr.offset(7);
            dpx_warning(b"The special \"pdf:literal reverse ...\" is no longer supported.\nIgnore the \"reverse\" option.\x00"
                            as *const u8 as *const i8);
        } else {
            if !((*args).curptr.offset(6) <= (*args).endptr
                && !strstartswith(
                    (*args).curptr,
                    b"direct\x00" as *const u8 as *const i8,
                )
                .is_null())
            {
                break;
            }
            direct = 1i32;
            (*args).curptr = (*args).curptr.offset(6)
        }
        skip_white(&mut (*args).curptr, (*args).endptr);
    }
    if (*args).curptr < (*args).endptr {
        let mut M: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        if direct == 0 {
            M.d = 1.0f64;
            M.a = M.d;
            M.c = 0.0f64;
            M.b = M.c;
            M.e = (*spe).x_user;
            M.f = (*spe).y_user;
            pdf_dev_concat(&mut M);
        }
        pdf_doc_add_page_content(
            b" \x00" as *const u8 as *const i8,
            1i32 as libc::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.wrapping_offset_from((*args).curptr) as i64 as libc::c_int
                as libc::c_uint,
        );
        if direct == 0 {
            M.e = -(*spe).x_user;
            M.f = -(*spe).y_user;
            pdf_dev_concat(&mut M);
        }
    }
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_bcontent(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut xpos: libc::c_double = 0.;
    let mut ypos: libc::c_double = 0.;
    pdf_dev_gsave();
    pdf_dev_get_coord(&mut xpos, &mut ypos);
    M.a = 1.0f64;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = 1.0f64;
    M.e = (*spe).x_user - xpos;
    M.f = (*spe).y_user - ypos;
    pdf_dev_concat(&mut M);
    pdf_dev_push_coord((*spe).x_user, (*spe).y_user);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_econtent(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    pdf_dev_pop_coord();
    pdf_dev_grestore();
    pdf_dev_reset_color(0i32);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_code(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        pdf_doc_add_page_content(
            b" \x00" as *const u8 as *const i8,
            1i32 as libc::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.wrapping_offset_from((*args).curptr) as i64 as libc::c_int
                as libc::c_uint,
        );
        (*args).curptr = (*args).endptr
    }
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_do_nothing(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_stream_with_type(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut fstream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut nb_read: ssize_t = 0;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut instring: *mut i8 = 0 as *mut i8;
    let mut fullname: *mut i8 = 0 as *mut i8;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    skip_white(&mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        spc_warn(
            spe,
            b"Missing objname for pdf:(f)stream.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    tmp = parse_pdf_object(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
    if tmp.is_null() {
        spc_warn(
            spe,
            b"Missing input string for pdf:(f)stream.\x00" as *const u8 as *const i8,
        );
        free(ident as *mut libc::c_void);
        return -1i32;
    } else {
        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 3i32) {
            spc_warn(
                spe,
                b"Invalid type of input string for pdf:(f)stream.\x00" as *const u8
                    as *const i8,
            );
            pdf_release_obj(tmp);
            free(ident as *mut libc::c_void);
            return -1i32;
        }
    }
    instring = pdf_string_value(tmp) as *mut i8;
    match type_0 {
        1 => {
            if instring.is_null() {
                spc_warn(
                    spe,
                    b"Missing filename for pdf:fstream.\x00" as *const u8 as *const i8,
                );
                pdf_release_obj(tmp);
                free(ident as *mut libc::c_void);
                return -1i32;
            }
            fullname = 0 as *mut i8;
            if fullname.is_null() {
                spc_warn(
                    spe,
                    b"File \"%s\" not found.\x00" as *const u8 as *const i8,
                    instring,
                );
                pdf_release_obj(tmp);
                free(ident as *mut libc::c_void);
                return -1i32;
            }
            handle = ttstub_input_open(fullname, TTIF_PICT, 0i32) as *mut rust_input_handle_t;
            if handle.is_null() {
                spc_warn(
                    spe,
                    b"Could not open file: %s\x00" as *const u8 as *const i8,
                    instring,
                );
                pdf_release_obj(tmp);
                free(ident as *mut libc::c_void);
                free(fullname as *mut libc::c_void);
                return -1i32;
            }
            fstream = pdf_new_stream(1i32 << 0i32);
            loop {
                nb_read = ttstub_input_read(
                    handle as rust_input_handle_t,
                    work_buffer.as_mut_ptr(),
                    1024i32 as size_t,
                );
                if !(nb_read > 0i32 as i64) {
                    break;
                }
                pdf_add_stream(
                    fstream,
                    work_buffer.as_mut_ptr() as *const libc::c_void,
                    nb_read as libc::c_int,
                );
            }
            ttstub_input_close(handle as rust_input_handle_t);
            free(fullname as *mut libc::c_void);
        }
        0 => {
            fstream = pdf_new_stream(1i32 << 0i32);
            if !instring.is_null() {
                pdf_add_stream(
                    fstream,
                    instring as *const libc::c_void,
                    strlen(instring) as libc::c_int,
                );
            }
        }
        _ => {
            pdf_release_obj(tmp);
            free(ident as *mut libc::c_void);
            return -1i32;
        }
    }
    pdf_release_obj(tmp);
    /*
     * Optional dict.
     *
     *  TODO: check Length, Filter...
     */
    skip_white(&mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0) as libc::c_int == '<' as i32 {
        let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
        stream_dict = pdf_stream_dict(fstream);
        tmp = parse_pdf_dict(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
        if tmp.is_null() {
            spc_warn(
                spe,
                b"Parsing dictionary failed.\x00" as *const u8 as *const i8,
            );
            pdf_release_obj(fstream);
            free(ident as *mut libc::c_void);
            return -1i32;
        }
        if !pdf_lookup_dict(tmp, b"Length\x00" as *const u8 as *const i8).is_null() {
            pdf_remove_dict(tmp, b"Length\x00" as *const u8 as *const i8);
        } else if !pdf_lookup_dict(tmp, b"Filter\x00" as *const u8 as *const i8).is_null()
        {
            pdf_remove_dict(tmp, b"Filter\x00" as *const u8 as *const i8);
        }
        pdf_merge_dict(stream_dict, tmp);
        pdf_release_obj(tmp);
    }
    /* Users should explicitly close this. */
    spc_push_object(ident, fstream);
    free(ident as *mut libc::c_void);
    return 0i32;
}
/*
 * STREAM: Create a PDF stream object from an input string.
 *
 *  pdf: stream @objname (input_string) [PDF_DICT]
 */
unsafe extern "C" fn spc_handler_pdfm_stream(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    return spc_handler_pdfm_stream_with_type(spe, args, 0i32);
}
/*
 * FSTREAM: Create a PDF stream object from an existing file.
 *
 *  pdf: fstream @objname (filename) [PDF_DICT]
 */
unsafe extern "C" fn spc_handler_pdfm_fstream(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    return spc_handler_pdfm_stream_with_type(spe, args, 1i32);
}
/* Grab page content as follows:
 *
 * Reference point = (x_user, y_user)
 *
 * Case 1. \special{pdf:bxobj @obj width WD height HT depth DP}
 *
 *     Grab the box with the lower-left corner (x_user, y_user-DP)
 *     and the upper right corner (x_user+WD, y_user+HT).
 *
 * Case 2. \special{pdf:bxobj @obj bbox LLX LLY URX, URY}
 *
 *     Grab the box with the lower-left corner (x_user+LLX, y_user+LLY)
 *     and the upper right corner (x_user+URX, y_user+URY).
 *
 * Note that scale, xscale, yscale, xoffset, yoffset options are ignored.
 */
unsafe extern "C" fn spc_handler_pdfm_bform(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut xobj_id: libc::c_int = 0;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut cropbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut ti: transform_info = transform_info {
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
    skip_white(&mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        spc_warn(
            spe,
            b"A form XObject must have name.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    transform_info_clear(&mut ti);
    if spc_util_read_dimtrns(spe, &mut ti, args, 0i32) < 0i32 {
        free(ident as *mut libc::c_void);
        return -1i32;
    }
    /* A XForm with zero dimension results in a non-invertible transformation
     * matrix. And it may result in unpredictable behaviour. It might be an
     * error in Acrobat. Bounding box with zero dimension may cause division
     * by zero.
     */
    if ti.flags & 1i32 << 0i32 != 0 {
        if ti.bbox.urx - ti.bbox.llx == 0.0f64 || ti.bbox.ury - ti.bbox.lly == 0.0f64 {
            spc_warn(
                spe,
                b"Bounding box has a zero dimension.\x00" as *const u8 as *const i8,
            );
            free(ident as *mut libc::c_void);
            return -1i32;
        }
        cropbox.llx = ti.bbox.llx;
        cropbox.lly = ti.bbox.lly;
        cropbox.urx = ti.bbox.urx;
        cropbox.ury = ti.bbox.ury
    } else {
        if ti.width == 0.0f64 || ti.depth + ti.height == 0.0f64 {
            spc_warn(
                spe,
                b"Bounding box has a zero dimension.\x00" as *const u8 as *const i8,
            );
            free(ident as *mut libc::c_void);
            return -1i32;
        }
        cropbox.llx = 0.0f64;
        cropbox.lly = -ti.depth;
        cropbox.urx = ti.width;
        cropbox.ury = ti.height
    }
    xobj_id = pdf_doc_begin_grabbing(ident, (*spe).x_user, (*spe).y_user, &mut cropbox);
    if xobj_id < 0i32 {
        free(ident as *mut libc::c_void);
        spc_warn(
            spe,
            b"Couldn\'t start form object.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    spc_push_object(ident, pdf_ximage_get_reference(xobj_id));
    free(ident as *mut libc::c_void);
    return 0i32;
}
/* An extra dictionary after exobj must be merged to the form dictionary,
 * not resource dictionary.
 * Please use pdf:put @resources (before pdf:exobj) instead.
 */
unsafe extern "C" fn spc_handler_pdfm_eform(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut attrib: *mut pdf_obj = 0 as *mut pdf_obj;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        attrib = parse_pdf_dict(&mut (*args).curptr, (*args).endptr, 0 as *mut pdf_file);
        if !attrib.is_null() && !(!attrib.is_null() && pdf_obj_typeof(attrib) == 6i32) {
            pdf_release_obj(attrib);
            attrib = 0 as *mut pdf_obj
        }
    }
    pdf_doc_end_grabbing(attrib);
    return 0i32;
}
/* Saved XObjects can be used as follows:
 *
 * Reference point = (x_user, y_user)
 *
 * Case 1. \special{pdf:uxobj @obj width WD height HT depth DP}
 *
 *     Scale the XObject to fit in the box
 *     [x_user, y_user-DP, x_user+WD, y_user+HT].
 *
 * Case 2. \special{pdf:uxobj @obj xscale XS yscale YS}
 *
 *     Scale the XObject with XS and YS. Note that width and xscale
 *     or height and yscale cannot be used together.
 *
 * Case 3. \special{pdf:bxobj @obj bbox LLX LLY URX, URY}
 *
 *     Scale the XObject to fit in the box
 *     [x_user+LLX, y_user+LLY, x_user+URX, y_user+URY].
 *
 * Note that xoffset and yoffset moves the reference point where the
 * lower-left corner of the XObject will be put.
 */
unsafe extern "C" fn spc_handler_pdfm_uxobj(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut xobj_id: libc::c_int = 0;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut ti: transform_info = transform_info {
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
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    skip_white(&mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        spc_warn(
            spe,
            b"No object identifier given.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    transform_info_clear(&mut ti);
    if (*args).curptr < (*args).endptr {
        if spc_util_read_dimtrns(spe, &mut ti, args, 0i32) < 0i32 {
            free(ident as *mut libc::c_void);
            return -1i32;
        }
    }
    /* Dvipdfmx was suddenly changed to use file name to identify
     * external images. We can't use ident to find image resource
     * here.
     */
    xobj_id = findresource(sd, ident);
    if xobj_id < 0i32 {
        xobj_id = pdf_ximage_findresource(ident, options);
        if xobj_id < 0i32 {
            spc_warn(
                spe,
                b"Specified (image) object doesn\'t exist: %s\x00" as *const u8
                    as *const i8,
                ident,
            );
            free(ident as *mut libc::c_void);
            return -1i32;
        }
    }
    pdf_dev_put_image(xobj_id, &mut ti, (*spe).x_user, (*spe).y_user);
    free(ident as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_link(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    return spc_resume_annot(spe);
}
unsafe extern "C" fn spc_handler_pdfm_nolink(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    return spc_suspend_annot(spe);
}
/* Handled at BOP */
unsafe extern "C" fn spc_handler_pdfm_pagesize(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    (*args).curptr = (*args).endptr;
    return 0i32;
}
/* Please remove this.
 * This should be handled before processing pages!
 */
unsafe extern "C" fn spc_handler_pdfm_bgcolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    error = spc_util_read_pdfcolor(spe, &mut colorspec, args, 0 as *mut pdf_color);
    if error != 0 {
        spc_warn(
            spe,
            b"No valid color specified?\x00" as *const u8 as *const i8,
        );
    } else {
        pdf_doc_set_bgcolor(&mut colorspec);
    }
    return error;
}
unsafe extern "C" fn spc_handler_pdfm_mapline(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut map_name: *mut i8 = 0 as *mut i8;
    let mut opchr: i8 = 0;
    let mut error: libc::c_int = 0i32;
    static mut buffer: [i8; 1024] = [0; 1024];
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        spc_warn(
            spe,
            b"Empty mapline special?\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    opchr = *(*ap).curptr.offset(0);
    if opchr as libc::c_int == '-' as i32 || opchr as libc::c_int == '+' as i32 {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    match opchr as libc::c_int {
        45 => {
            map_name = parse_ident(&mut (*ap).curptr, (*ap).endptr);
            if !map_name.is_null() {
                pdf_remove_fontmap_record(map_name);
                free(map_name as *mut libc::c_void);
            } else {
                spc_warn(
                    spe,
                    b"Invalid fontmap line: Missing TFM name.\x00" as *const u8
                        as *const i8,
                );
                error = -1i32
            }
        }
        _ => {
            p = (*ap).curptr;
            q = buffer.as_mut_ptr();
            while p < (*ap).endptr {
                let fresh11 = p;
                p = p.offset(1);
                let fresh12 = q;
                q = q.offset(1);
                *fresh12 = *fresh11
            }
            *q = '\u{0}' as i32 as i8;
            mrec = new((1i32 as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<fontmap_rec>() as u64)
                as u32) as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            error = pdf_read_fontmap_line(
                mrec,
                buffer.as_mut_ptr(),
                (*ap).endptr.wrapping_offset_from((*ap).curptr) as i64 as libc::c_int,
                is_pdfm_mapline(buffer.as_mut_ptr()),
            );
            if error != 0 {
                spc_warn(
                    spe,
                    b"Invalid fontmap line.\x00" as *const u8 as *const i8,
                );
            } else if opchr as libc::c_int == '+' as i32 {
                pdf_append_fontmap_record((*mrec).map_name, mrec);
            } else {
                pdf_insert_fontmap_record((*mrec).map_name, mrec);
            }
            pdf_clear_fontmap_record(mrec);
            free(mrec as *mut libc::c_void);
        }
    }
    if error == 0 {
        (*ap).curptr = (*ap).endptr
    }
    return 0i32;
}
unsafe extern "C" fn spc_handler_pdfm_mapfile(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut mapfile: *mut i8 = 0 as *mut i8;
    let mut mode: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return 0i32;
    }
    match *(*args).curptr.offset(0) as libc::c_int {
        45 => {
            mode = '-' as i32;
            (*args).curptr = (*args).curptr.offset(1)
        }
        43 => {
            mode = '+' as i32;
            (*args).curptr = (*args).curptr.offset(1)
        }
        _ => mode = 0i32,
    }
    mapfile = parse_val_ident(&mut (*args).curptr, (*args).endptr);
    if mapfile.is_null() {
        spc_warn(
            spe,
            b"No fontmap file specified.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    } else {
        error = pdf_load_fontmap_file(mapfile, mode)
    }
    free(mapfile as *mut libc::c_void);
    return error;
}
unsafe extern "C" fn spc_handler_pdfm_tounicode(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut sd: *mut spc_pdf_ = &mut _pdf_stat;
    let mut cmap_name: *mut i8 = 0 as *mut i8;
    /* First clear */
    (*sd).cd.cmap_id = -1i32;
    (*sd).cd.unescape_backslash = 0i32;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        spc_warn(
            spe,
            b"Missing CMap name for pdf:tounicode.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    /* _FIXME_
     * Any valid char allowed for PDF name object should be allowed here.
     * The argument to this special should be a PDF name obejct.
     * But it's too late to change this special.
     */
    cmap_name = parse_ident(&mut (*args).curptr, (*args).endptr);
    if cmap_name.is_null() {
        spc_warn(
            spe,
            b"Missing ToUnicode mapping name...\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*sd).cd.cmap_id = CMap_cache_find(cmap_name);
    if (*sd).cd.cmap_id < 0i32 {
        spc_warn(
            spe,
            b"Failed to load ToUnicode mapping: %s\x00" as *const u8 as *const i8,
            cmap_name,
        );
        free(cmap_name as *mut libc::c_void);
        return -1i32;
    }
    /* Shift-JIS like encoding may contain backslash in 2nd byte.
     * WARNING: This will add nasty extension to PDF parser.
     */
    if (*sd).cd.cmap_id >= 0i32 {
        if !strstr(cmap_name, b"RKSJ\x00" as *const u8 as *const i8).is_null()
            || !strstr(cmap_name, b"B5\x00" as *const u8 as *const i8).is_null()
            || !strstr(cmap_name, b"GBK\x00" as *const u8 as *const i8).is_null()
            || !strstr(cmap_name, b"KSC\x00" as *const u8 as *const i8).is_null()
        {
            (*sd).cd.unescape_backslash = 1i32
        }
    }
    free(cmap_name as *mut libc::c_void);
    return 0i32;
}
static mut pdfm_handlers: [spc_handler; 80] = unsafe {
    [
        {
            let mut init = spc_handler {
                key: b"annotation\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_annot
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"annotate\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_annot
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"annot\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_annot
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ann\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_annot
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"outline\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_outline
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"out\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_outline
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"article\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_article
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"art\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_article
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bead\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bead
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"thread\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bead
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"destination\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_dest
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"dest\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_dest
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"object\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_object
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"obj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_object
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"docinfo\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_docinfo
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"docview\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_docview
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"content\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_content
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"put\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_put
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"close\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_close
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bop\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bop
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"eop\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eop
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"image\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_image
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"img\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_image
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"epdf\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_image
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"link\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_link
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"nolink\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_nolink
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"begincolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bcolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bc\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"setcolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_scolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"scolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_scolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"sc\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_scolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"endcolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_ecolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ecolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_ecolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ec\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_ecolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"begingray\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bgray\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bg\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"endgray\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_ecolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"egray\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_ecolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"eg\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_ecolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bgcolor\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bgcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bgc\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bgcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bbc\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bgcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bbg\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bgcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"pagesize\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_pagesize
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bannot\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bann
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"beginann\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bann
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bann\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bann
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"eannot\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eann
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"endann\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eann
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"eann\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eann
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"btrans\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_btrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"begintransform\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_btrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"begintrans\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_btrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bt\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_btrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"etrans\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_etrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"endtransform\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_etrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"endtrans\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_etrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"et\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_etrans
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bform\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bform
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"beginxobj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bform
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bxobj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bform
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"eform\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eform
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"endxobj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eform
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"exobj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_eform
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"usexobj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_uxobj
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"uxobj\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_uxobj
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"tounicode\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_tounicode
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"literal\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_literal
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"stream\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_stream
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"fstream\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_fstream
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"names\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_names
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"mapline\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_mapline
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"mapfile\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_mapfile
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bcontent\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_bcontent
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"econtent\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_econtent
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"code\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_code
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"minorversion\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_do_nothing
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"encrypt\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_pdfm_do_nothing
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_pdfm_check_special(
    mut buf: *const i8,
    mut len: libc::c_int,
) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p.offset(strlen(b"pdf:\x00" as *const u8 as *const i8) as isize) <= endptr
        && memcmp(
            p as *const libc::c_void,
            b"pdf:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"pdf:\x00" as *const u8 as *const i8),
        ) == 0
    {
        return 1i32 != 0;
    }
    return 0i32 != 0;
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
pub unsafe extern "C" fn spc_pdfm_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = -1i32;
    let mut i: size_t = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    if !sph.is_null() && !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(b"sph && spe && ap\x00" as *const u8 as
                          *const i8,
                      b"dpx-spc_pdfm.c\x00" as *const u8 as
                          *const i8, 1970i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[i8; 85]>(b"int spc_pdfm_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap)
        .curptr
        .offset(strlen(b"pdf:\x00" as *const u8 as *const i8) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const libc::c_void,
            b"pdf:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"pdf:\x00" as *const u8 as *const i8),
        ) != 0
    {
        spc_warn(
            spe,
            b"Not pdf: special???\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"pdf:\x00" as *const u8 as *const i8) as isize);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        i = 0i32 as size_t;
        while i
            < (::std::mem::size_of::<[spc_handler; 80]>() as u64)
                .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
        {
            if streq_ptr(q, pdfm_handlers[i as usize].key) {
                (*ap).command = pdfm_handlers[i as usize].key;
                (*sph).key = b"pdf:\x00" as *const u8 as *const i8;
                (*sph).exec = pdfm_handlers[i as usize].exec;
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                error = 0i32;
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
        free(q as *mut libc::c_void);
    }
    return error;
}
