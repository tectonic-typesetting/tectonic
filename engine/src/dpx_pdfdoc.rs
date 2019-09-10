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
    pub type pdf_file;
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
    fn pdf_color_set_verbose(level: i32);
    #[no_mangle]
    fn floor(_: f64) -> f64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: i32) -> i32;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
        format: tt_input_format_type,
        is_gz: i32,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn pdf_out_init(filename: *const i8, enable_encrypt: bool, enable_object_stream: bool);
    #[no_mangle]
    fn pdf_out_flush();
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: f64) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_number_value(number: *mut pdf_obj) -> f64;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_string_length(object: *mut pdf_obj) -> u32;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut i8;
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
    fn pdf_get_array(array: *mut pdf_obj, idx: i32) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_array_length(array: *mut pdf_obj) -> u32;
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_remove_dict(dict: *mut pdf_obj, key: *const i8);
    #[no_mangle]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const i8) -> *mut pdf_obj;
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
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_stream_length(stream: *mut pdf_obj) -> i32;
    /* Compare label of two indirect reference object.
     */
    #[no_mangle]
    fn pdf_compare_reference(ref1: *mut pdf_obj, ref2: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn pdf_set_info(obj: *mut pdf_obj);
    #[no_mangle]
    fn pdf_set_root(obj: *mut pdf_obj);
    #[no_mangle]
    fn pdf_set_id(id: *mut pdf_obj);
    #[no_mangle]
    fn pdf_set_encrypt(encrypt: *mut pdf_obj);
    #[no_mangle]
    fn pdf_file_get_catalog(pf: *mut pdf_file) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn get_unique_time_if_given() -> time_t;
    /* Accessor to various device parameters.
     */
    #[no_mangle]
    fn pdf_dev_get_param(param_type: i32) -> i32;
    /* Text composition mode is ignored (always same as font's
     * writing mode) and glyph rotation is not enabled if
     * auto_rotate is unset.
     */
    /*
     * For pdf_doc, pdf_draw and others.
     */
    /* Force reselecting font and color:
     * XFrom (content grabbing) and Metapost support want them.
     */
    #[no_mangle]
    fn pdf_dev_reset_fonts(newpage: i32);
    #[no_mangle]
    fn pdf_dev_reset_color(force: i32);
    /* Initialization of transformation matrix with M and others.
     * They are called within pdf_doc_begin_page() and pdf_doc_end_page().
     */
    #[no_mangle]
    fn pdf_dev_bop(M: *const pdf_tmatrix);
    #[no_mangle]
    fn pdf_dev_eop();
    #[no_mangle]
    fn pdf_dev_get_coord(xpos: *mut f64, ypos: *mut f64);
    #[no_mangle]
    fn pdf_color_graycolor(color: *mut pdf_color, g: f64) -> i32;
    #[no_mangle]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    #[no_mangle]
    fn pdf_close_colors();
    #[no_mangle]
    fn pdf_init_colors();
    #[no_mangle]
    fn pdf_color_is_white(color: *const pdf_color) -> bool;
    #[no_mangle]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[no_mangle]
    fn ht_clear_table(ht: *mut ht_table);
    #[no_mangle]
    fn ht_table_size(ht: *mut ht_table) -> i32;
    #[no_mangle]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
        value: *mut libc::c_void,
    );
    #[no_mangle]
    fn ht_set_iter(ht: *mut ht_table, iter: *mut ht_iter) -> i32;
    #[no_mangle]
    fn ht_clear_iter(iter: *mut ht_iter);
    #[no_mangle]
    fn ht_iter_getkey(iter: *mut ht_iter, keylen: *mut i32) -> *mut i8;
    #[no_mangle]
    fn ht_iter_next(iter: *mut ht_iter) -> i32;
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
    static mut is_xdv: i32;
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
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn pdf_ximage_set_verbose(level: i32);
    #[no_mangle]
    fn pdf_init_images();
    #[no_mangle]
    fn pdf_close_images();
    #[no_mangle]
    fn pdf_ximage_get_reference(xobj_id: i32) -> *mut pdf_obj;
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const i8, options: load_options) -> i32;
    #[no_mangle]
    fn pdf_ximage_defineresource(
        ident: *const i8,
        subtype: i32,
        cdata: *mut libc::c_void,
        resource: *mut pdf_obj,
    ) -> i32;
    #[no_mangle]
    fn pdf_ximage_init_form_info(info: *mut xform_info);
    #[no_mangle]
    fn check_for_jpeg(handle: rust_input_handle_t) -> i32;
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
    #[no_mangle]
    fn pdf_dev_rectfill(x: f64, y: f64, w: f64, h: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_gsave() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore() -> i32;
    /* Requires from mpost.c because new MetaPost graphics must initialize
     * the current gstate. */
    #[no_mangle]
    fn pdf_dev_push_gstate() -> i32;
    #[no_mangle]
    fn pdf_dev_pop_gstate() -> i32;
    /* The depth here is the depth of q/Q nesting.
     * We must remember current depth of nesting when starting a page or xform,
     * and must recover until that depth at the end of page/xform.
     */
    #[no_mangle]
    fn pdf_dev_current_depth() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore_to(depth: i32);
    #[no_mangle]
    fn pdf_dev_set_color(color: *const pdf_color, mask: i8, force: i32);
    #[no_mangle]
    fn pdf_enc_id_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_encrypt_obj() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_init_fonts();
    #[no_mangle]
    fn pdf_close_fonts();
    #[no_mangle]
    fn pdf_font_set_verbose(level: i32);
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
    /* Hash */
    /* Not actually tree... */
    #[no_mangle]
    fn pdf_new_name_tree() -> *mut ht_table;
    #[no_mangle]
    fn pdf_delete_name_tree(names: *mut *mut ht_table);
    #[no_mangle]
    fn pdf_names_add_object(
        names: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
        object: *mut pdf_obj,
    ) -> i32;
    /* Really create name tree... */
    #[no_mangle]
    fn pdf_names_create_tree(
        names: *mut ht_table,
        count: *mut i32,
        filter: *mut ht_table,
    ) -> *mut pdf_obj;
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
    fn pdf_init_resources();
    #[no_mangle]
    fn pdf_close_resources();
    #[no_mangle]
    fn check_for_png(handle: rust_input_handle_t) -> i32;
}
pub type __time_t = i64;
pub type size_t = u64;
pub type time_t = __time_t;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = u32;
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
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub num_components: i32,
    pub spot_color_name: *mut i8,
    pub values: [f64; 4],
}
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
pub struct form_list_node {
    pub q_depth: i32,
    pub form: pdf_form,
    pub prev: *mut form_list_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_form {
    pub ident: *mut i8,
    pub matrix: pdf_tmatrix,
    pub cropbox: pdf_rect,
    pub resources: *mut pdf_obj,
    pub contents: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: i32,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut i8,
    pub keylen: i32,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_article {
    pub id: *mut i8,
    pub info: *mut pdf_obj,
    pub num_beads: u32,
    pub max_beads: u32,
    pub beads: *mut pdf_bead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_bead {
    pub id: *mut i8,
    pub page_no: i32,
    pub rect: pdf_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_olitem {
    pub dict: *mut pdf_obj,
    pub is_open: i32,
    pub first: *mut pdf_olitem,
    pub parent: *mut pdf_olitem,
    pub next: *mut pdf_olitem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_page {
    pub page_obj: *mut pdf_obj,
    pub page_ref: *mut pdf_obj,
    pub flags: i32,
    pub ref_x: f64,
    pub ref_y: f64,
    pub cropbox: pdf_rect,
    pub resources: *mut pdf_obj,
    pub background: *mut pdf_obj,
    pub contents: *mut pdf_obj,
    pub content_refs: [*mut pdf_obj; 4],
    pub annots: *mut pdf_obj,
    pub beads: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_doc {
    pub root: C2RustUnnamed_3,
    pub info: *mut pdf_obj,
    pub pages: C2RustUnnamed_2,
    pub outlines: C2RustUnnamed_1,
    pub articles: C2RustUnnamed_0,
    pub names: *mut name_dict,
    pub check_gotos: i32,
    pub gotos: ht_table,
    pub opt: C2RustUnnamed,
    pub pending_forms: *mut form_list_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub outline_open_depth: i32,
    pub annot_grow: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_dict {
    pub category: *const i8,
    pub data: *mut ht_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num_entries: u32,
    pub max_entries: u32,
    pub entries: *mut pdf_article,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub first: *mut pdf_olitem,
    pub current: *mut pdf_olitem,
    pub current_depth: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub mediabox: pdf_rect,
    pub bop: *mut pdf_obj,
    pub eop: *mut pdf_obj,
    pub num_entries: u32,
    pub max_entries: u32,
    pub entries: *mut pdf_page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub dict: *mut pdf_obj,
    pub viewerpref: *mut pdf_obj,
    pub pagelabels: *mut pdf_obj,
    pub pages: *mut pdf_obj,
    pub names: *mut pdf_obj,
    pub threads: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_iter {
    pub index: i32,
    pub curr: *mut libc::c_void,
    pub hash: *mut ht_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: i32,
    pub bbox_type: i32,
    pub dict: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xform_info {
    pub flags: i32,
    pub bbox: pdf_rect,
    pub matrix: pdf_tmatrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub dirty: i32,
    pub broken: i32,
    pub annot_dict: *mut pdf_obj,
    pub rect: pdf_rect,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
/* quasi-hack to get the primary input */
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
static mut verbose: i32 = 0i32;
static mut manual_thumb_enabled: i8 = 0_i8;
static mut thumb_basename: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_enable_manual_thumbnails() {
    manual_thumb_enabled = 1_i8;
    // without HAVE_LIBPNG:
    // dpx_warning("Manual thumbnail is not supported without the libpng library.");
}
unsafe extern "C" fn read_thumbnail(mut thumb_filename: *const i8) -> *mut pdf_obj {
    let mut image_ref: *mut pdf_obj = 0 as *mut pdf_obj; /* Maybe reference */
    let mut xobj_id: i32 = 0;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    handle = ttstub_input_open(thumb_filename, TTIF_PICT, 0i32) as *mut rust_input_handle_t;
    if handle.is_null() {
        dpx_warning(
            b"Could not open thumbnail file \"%s\"\x00" as *const u8 as *const i8,
            thumb_filename,
        );
        return 0 as *mut pdf_obj;
    }
    if check_for_png(handle as rust_input_handle_t) == 0
        && check_for_jpeg(handle as rust_input_handle_t) == 0
    {
        dpx_warning(
            b"Thumbnail \"%s\" not a png/jpeg file!\x00" as *const u8 as *const i8,
            thumb_filename,
        );
        ttstub_input_close(handle as rust_input_handle_t);
        return 0 as *mut pdf_obj;
    }
    ttstub_input_close(handle as rust_input_handle_t);
    xobj_id = pdf_ximage_findresource(thumb_filename, options);
    if xobj_id < 0i32 {
        dpx_warning(
            b"Could not read thumbnail file \"%s\".\x00" as *const u8 as *const i8,
            thumb_filename,
        );
        image_ref = 0 as *mut pdf_obj
    } else {
        image_ref = pdf_ximage_get_reference(xobj_id)
    }
    return image_ref;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_set_verbose(mut level: i32) {
    verbose = level;
    pdf_font_set_verbose(level);
    pdf_color_set_verbose(level);
    pdf_ximage_set_verbose(level);
}
static mut pdoc: pdf_doc = pdf_doc {
    root: C2RustUnnamed_3 {
        dict: 0 as *const pdf_obj as *mut pdf_obj,
        viewerpref: 0 as *const pdf_obj as *mut pdf_obj,
        pagelabels: 0 as *const pdf_obj as *mut pdf_obj,
        pages: 0 as *const pdf_obj as *mut pdf_obj,
        names: 0 as *const pdf_obj as *mut pdf_obj,
        threads: 0 as *const pdf_obj as *mut pdf_obj,
    },
    info: 0 as *const pdf_obj as *mut pdf_obj,
    pages: C2RustUnnamed_2 {
        mediabox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        bop: 0 as *const pdf_obj as *mut pdf_obj,
        eop: 0 as *const pdf_obj as *mut pdf_obj,
        num_entries: 0,
        max_entries: 0,
        entries: 0 as *const pdf_page as *mut pdf_page,
    },
    outlines: C2RustUnnamed_1 {
        first: 0 as *const pdf_olitem as *mut pdf_olitem,
        current: 0 as *const pdf_olitem as *mut pdf_olitem,
        current_depth: 0,
    },
    articles: C2RustUnnamed_0 {
        num_entries: 0,
        max_entries: 0,
        entries: 0 as *const pdf_article as *mut pdf_article,
    },
    names: 0 as *const name_dict as *mut name_dict,
    check_gotos: 0,
    gotos: ht_table {
        count: 0,
        hval_free_fn: None,
        table: [0 as *const ht_entry as *mut ht_entry; 503],
    },
    opt: C2RustUnnamed {
        outline_open_depth: 0,
        annot_grow: 0.,
    },
    pending_forms: 0 as *const form_list_node as *mut form_list_node,
};
unsafe extern "C" fn pdf_doc_init_catalog(mut p: *mut pdf_doc) {
    (*p).root.viewerpref = 0 as *mut pdf_obj;
    (*p).root.pagelabels = 0 as *mut pdf_obj;
    (*p).root.pages = 0 as *mut pdf_obj;
    (*p).root.names = 0 as *mut pdf_obj;
    (*p).root.threads = 0 as *mut pdf_obj;
    (*p).root.dict = pdf_new_dict();
    pdf_set_root((*p).root.dict);
}
unsafe extern "C" fn pdf_doc_close_catalog(mut p: *mut pdf_doc) {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    if !(*p).root.viewerpref.is_null() {
        tmp = pdf_lookup_dict(
            (*p).root.dict,
            b"ViewerPreferences\x00" as *const u8 as *const i8,
        );
        if tmp.is_null() {
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"ViewerPreferences\x00" as *const u8 as *const i8),
                pdf_ref_obj((*p).root.viewerpref),
            );
        } else if !tmp.is_null() && pdf_obj_typeof(tmp) == 6i32 {
            pdf_merge_dict((*p).root.viewerpref, tmp);
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"ViewerPreferences\x00" as *const u8 as *const i8),
                pdf_ref_obj((*p).root.viewerpref),
            );
        } else {
            /* What should I do? */
            dpx_warning(b"Could not modify ViewerPreferences.\x00" as *const u8 as *const i8);
            /* Maybe reference */
        }
        pdf_release_obj((*p).root.viewerpref);
        (*p).root.viewerpref = 0 as *mut pdf_obj
    }
    if !(*p).root.pagelabels.is_null() {
        tmp = pdf_lookup_dict((*p).root.dict, b"PageLabels\x00" as *const u8 as *const i8);
        if tmp.is_null() {
            tmp = pdf_new_dict();
            pdf_add_dict(
                tmp,
                pdf_new_name(b"Nums\x00" as *const u8 as *const i8),
                pdf_link_obj((*p).root.pagelabels),
            );
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"PageLabels\x00" as *const u8 as *const i8),
                pdf_ref_obj(tmp),
            );
            pdf_release_obj(tmp);
        } else {
            /* What should I do? */
            dpx_warning(b"Could not modify PageLabels.\x00" as *const u8 as *const i8);
        }
        pdf_release_obj((*p).root.pagelabels);
        (*p).root.pagelabels = 0 as *mut pdf_obj
    }
    pdf_add_dict(
        (*p).root.dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Catalog\x00" as *const u8 as *const i8),
    );
    pdf_release_obj((*p).root.dict);
    (*p).root.dict = 0 as *mut pdf_obj;
}
/*
 * Pages are starting at 1.
 * The page count does not increase until the page is finished.
 */
unsafe extern "C" fn doc_resize_page_entries(mut p: *mut pdf_doc, mut size: u32) {
    if size > (*p).pages.max_entries {
        let mut i: u32 = 0; /* global bop */
        (*p).pages.entries = renew(
            (*p).pages.entries as *mut libc::c_void,
            (size as u64).wrapping_mul(::std::mem::size_of::<pdf_page>() as u64) as u32,
        ) as *mut pdf_page; /* background */
        i = (*p).pages.max_entries; /* page body  */
        while i < size {
            let ref mut fresh0 = (*(*p).pages.entries.offset(i as isize)).page_obj; /* global eop */
            *fresh0 = 0 as *mut pdf_obj;
            let ref mut fresh1 = (*(*p).pages.entries.offset(i as isize)).page_ref;
            *fresh1 = 0 as *mut pdf_obj;
            (*(*p).pages.entries.offset(i as isize)).flags = 0i32;
            let ref mut fresh2 = (*(*p).pages.entries.offset(i as isize)).resources;
            *fresh2 = 0 as *mut pdf_obj;
            let ref mut fresh3 = (*(*p).pages.entries.offset(i as isize)).background;
            *fresh3 = 0 as *mut pdf_obj;
            let ref mut fresh4 = (*(*p).pages.entries.offset(i as isize)).contents;
            *fresh4 = 0 as *mut pdf_obj;
            let ref mut fresh5 = (*(*p).pages.entries.offset(i as isize)).content_refs[0];
            *fresh5 = 0 as *mut pdf_obj;
            let ref mut fresh6 = (*(*p).pages.entries.offset(i as isize)).content_refs[1];
            *fresh6 = 0 as *mut pdf_obj;
            let ref mut fresh7 = (*(*p).pages.entries.offset(i as isize)).content_refs[2];
            *fresh7 = 0 as *mut pdf_obj;
            let ref mut fresh8 = (*(*p).pages.entries.offset(i as isize)).content_refs[3];
            *fresh8 = 0 as *mut pdf_obj;
            let ref mut fresh9 = (*(*p).pages.entries.offset(i as isize)).annots;
            *fresh9 = 0 as *mut pdf_obj;
            let ref mut fresh10 = (*(*p).pages.entries.offset(i as isize)).beads;
            *fresh10 = 0 as *mut pdf_obj;
            i = i.wrapping_add(1)
        }
        (*p).pages.max_entries = size
    };
}
unsafe extern "C" fn doc_get_page_entry(mut p: *mut pdf_doc, mut page_no: u32) -> *mut pdf_page {
    let mut page: *mut pdf_page = 0 as *mut pdf_page;
    if page_no as u64 > 65535 {
        _tt_abort(
            b"Page number %ul too large!\x00" as *const u8 as *const i8,
            page_no,
        );
    } else {
        if page_no == 0_u32 {
            _tt_abort(
                b"Invalid Page number %ul.\x00" as *const u8 as *const i8,
                page_no,
            );
        }
    }
    if page_no > (*p).pages.max_entries {
        doc_resize_page_entries(p, page_no.wrapping_add(128u32));
    }
    page = &mut *(*p)
        .pages
        .entries
        .offset(page_no.wrapping_sub(1_u32) as isize) as *mut pdf_page;
    return page;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_set_bop_content(mut content: *const i8, mut length: u32) {
    let mut p: *mut pdf_doc = &mut pdoc;
    assert!(!p.is_null());
    if !(*p).pages.bop.is_null() {
        pdf_release_obj((*p).pages.bop);
        (*p).pages.bop = 0 as *mut pdf_obj
    }
    if length > 0_u32 {
        (*p).pages.bop = pdf_new_stream(1i32 << 0i32);
        pdf_add_stream(
            (*p).pages.bop,
            content as *const libc::c_void,
            length as i32,
        );
    } else {
        (*p).pages.bop = 0 as *mut pdf_obj
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_set_eop_content(mut content: *const i8, mut length: u32) {
    let mut p: *mut pdf_doc = &mut pdoc;
    if !(*p).pages.eop.is_null() {
        pdf_release_obj((*p).pages.eop);
        (*p).pages.eop = 0 as *mut pdf_obj
    }
    if length > 0_u32 {
        (*p).pages.eop = pdf_new_stream(1i32 << 0i32);
        pdf_add_stream(
            (*p).pages.eop,
            content as *const libc::c_void,
            length as i32,
        );
    } else {
        (*p).pages.eop = 0 as *mut pdf_obj
    };
}
/* auxiliary function to compute timezone offset on
systems that do not support the tm_gmtoff in struct tm,
or have a timezone variable.  Such as i386-solaris.  */
unsafe extern "C" fn compute_timezone_offset() -> i32 {
    let mut now: time_t = 0;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    let mut local: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    now = get_unique_time_if_given();
    if now == -1i32 as time_t {
        now = time(0 as *mut time_t);
        localtime_r(&mut now, &mut local);
        gmtime_r(&mut now, &mut tm);
        return (mktime(&mut local) - mktime(&mut tm)) as i32;
    } else {
        return 0i32;
    };
}
/* HAVE_TIMEZONE */
/* HAVE_TM_GMTOFF */
/*
 * Docinfo
 */
unsafe extern "C" fn asn_date(mut date_string: *mut i8) -> i32 {
    let mut tz_offset: i32 = 0;
    let mut current_time: time_t = 0;
    let mut bd_time: *mut tm = 0 as *mut tm;
    current_time = get_unique_time_if_given();
    if current_time == -1i32 as time_t {
        time(&mut current_time);
        bd_time = localtime(&mut current_time);
        tz_offset = compute_timezone_offset()
    /* HAVE_TIMEZONE */
    /* HAVE_TM_GMTOFF */
    } else {
        bd_time = gmtime(&mut current_time);
        tz_offset = 0i32
    }
    sprintf(
        date_string,
        b"D:%04d%02d%02d%02d%02d%02d%c%02d\'%02d\'\x00" as *const u8 as *const i8,
        (*bd_time).tm_year + 1900i32,
        (*bd_time).tm_mon + 1i32,
        (*bd_time).tm_mday,
        (*bd_time).tm_hour,
        (*bd_time).tm_min,
        (*bd_time).tm_sec,
        if tz_offset > 0i32 {
            '+' as i32
        } else {
            '-' as i32
        },
        abs(tz_offset) / 3600i32,
        abs(tz_offset) / 60i32 % 60i32,
    );
    return strlen(date_string) as i32;
}
unsafe extern "C" fn pdf_doc_init_docinfo(mut p: *mut pdf_doc) {
    (*p).info = pdf_new_dict();
    pdf_set_info((*p).info);
}
unsafe extern "C" fn pdf_doc_close_docinfo(mut p: *mut pdf_doc) {
    let mut docinfo: *mut pdf_obj = (*p).info;
    /*
     * Excerpt from PDF Reference 4th ed., sec. 10.2.1.
     *
     * Any entry whose value is not known should be omitted from the dictionary,
     * rather than included with an empty string as its value.
     *
     * ....
     *
     * Note: Although viewer applications can store custom metadata in the document
     * information dictionary, it is inappropriate to store private content or
     * structural information there; such information should be stored in the
     * document catalog instead (see Section 3.6.1,  Document Catalog ).
     */
    let mut keys: [*const i8; 9] = [
        b"Title\x00" as *const u8 as *const i8,
        b"Author\x00" as *const u8 as *const i8,
        b"Subject\x00" as *const u8 as *const i8,
        b"Keywords\x00" as *const u8 as *const i8,
        b"Creator\x00" as *const u8 as *const i8,
        b"Producer\x00" as *const u8 as *const i8,
        b"CreationDate\x00" as *const u8 as *const i8,
        b"ModDate\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: u32 = 0;
    i = 0_u32;
    while !keys[i as usize].is_null() {
        value = pdf_lookup_dict(docinfo, keys[i as usize]);
        if !value.is_null() {
            if !(!value.is_null() && pdf_obj_typeof(value) == 3i32) {
                dpx_warning(
                    b"\"%s\" in DocInfo dictionary not string type.\x00" as *const u8 as *const i8,
                    keys[i as usize],
                );
                pdf_remove_dict(docinfo, keys[i as usize]);
                dpx_warning(
                    b"\"%s\" removed from DocInfo.\x00" as *const u8 as *const i8,
                    keys[i as usize],
                );
            } else if pdf_string_length(value) == 0_u32 {
                /* The hyperref package often uses emtpy strings. */
                pdf_remove_dict(docinfo, keys[i as usize]);
            }
        }
        i = i.wrapping_add(1)
    }
    if pdf_lookup_dict(docinfo, b"Producer\x00" as *const u8 as *const i8).is_null() {
        let mut banner: [i8; 16] =
            *::std::mem::transmute::<&[u8; 16], &mut [i8; 16]>(b"xdvipdfmx (0.1)\x00");
        pdf_add_dict(
            docinfo,
            pdf_new_name(b"Producer\x00" as *const u8 as *const i8),
            pdf_new_string(
                banner.as_mut_ptr() as *const libc::c_void,
                strlen(banner.as_mut_ptr()),
            ),
        );
    }
    if pdf_lookup_dict(docinfo, b"CreationDate\x00" as *const u8 as *const i8).is_null() {
        let mut now: [i8; 80] = [0; 80];
        asn_date(now.as_mut_ptr());
        pdf_add_dict(
            docinfo,
            pdf_new_name(b"CreationDate\x00" as *const u8 as *const i8),
            pdf_new_string(
                now.as_mut_ptr() as *const libc::c_void,
                strlen(now.as_mut_ptr()),
            ),
        );
    }
    pdf_release_obj(docinfo);
    (*p).info = 0 as *mut pdf_obj;
}
unsafe extern "C" fn pdf_doc_get_page_resources(
    mut p: *mut pdf_doc,
    mut category: *const i8,
) -> *mut pdf_obj {
    let mut resources: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
    let mut res_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    if p.is_null() || category.is_null() {
        return 0 as *mut pdf_obj;
    }
    if !(*p).pending_forms.is_null() {
        if !(*(*p).pending_forms).form.resources.is_null() {
            res_dict = (*(*p).pending_forms).form.resources
        } else {
            (*(*p).pending_forms).form.resources = pdf_new_dict();
            res_dict = (*(*p).pending_forms).form.resources
        }
    } else {
        currentpage =
            &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        if !(*currentpage).resources.is_null() {
            res_dict = (*currentpage).resources
        } else {
            (*currentpage).resources = pdf_new_dict();
            res_dict = (*currentpage).resources
        }
    }
    resources = pdf_lookup_dict(res_dict, category);
    if resources.is_null() {
        resources = pdf_new_dict();
        pdf_add_dict(res_dict, pdf_new_name(category), resources);
    }
    return resources;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_add_page_resource(
    mut category: *const i8,
    mut resource_name: *const i8,
    mut resource_ref: *mut pdf_obj,
) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut resources: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut duplicate: *mut pdf_obj = 0 as *mut pdf_obj;
    if !(!resource_ref.is_null() && pdf_obj_typeof(resource_ref) == 9i32) {
        dpx_warning(b"Passed non indirect reference...\x00" as *const u8 as *const i8);
        resource_ref = pdf_ref_obj(resource_ref)
        /* leak */
    }
    resources = pdf_doc_get_page_resources(p, category);
    duplicate = pdf_lookup_dict(resources, resource_name);
    if !duplicate.is_null() && pdf_compare_reference(duplicate, resource_ref) != 0 {
        dpx_warning(
            b"Conflicting page resource found (page: %d, category: %s, name: %s).\x00" as *const u8
                as *const i8,
            pdf_doc_current_page_number(),
            category,
            resource_name,
        );
        dpx_warning(b"Ignoring...\x00" as *const u8 as *const i8);
        pdf_release_obj(resource_ref);
    } else {
        pdf_add_dict(resources, pdf_new_name(resource_name), resource_ref);
    };
}
unsafe extern "C" fn doc_flush_page(
    mut p: *mut pdf_doc,
    mut page: *mut pdf_page,
    mut parent_ref: *mut pdf_obj,
) {
    let mut contents_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut count: u32 = 0;
    pdf_add_dict(
        (*page).page_obj,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Page\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        (*page).page_obj,
        pdf_new_name(b"Parent\x00" as *const u8 as *const i8),
        parent_ref,
    );
    /*
     * Clipping area specified by CropBox is affected by MediaBox which
     * might be inherit from parent node. If MediaBox of the root node
     * does not have enough size to cover all page's imaging area, using
     * CropBox here gives incorrect result.
     */
    if (*page).flags & 1i32 << 0i32 != 0 {
        let mut mediabox: *mut pdf_obj = 0 as *mut pdf_obj;
        mediabox = pdf_new_array();
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.llx / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.lly / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.urx / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.ury / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_dict(
            (*page).page_obj,
            pdf_new_name(b"MediaBox\x00" as *const u8 as *const i8),
            mediabox,
        );
    }
    count = 0_u32;
    contents_array = pdf_new_array();
    if !(*page).content_refs[0].is_null() {
        /* global bop */
        pdf_add_array(contents_array, (*page).content_refs[0]);
        count = count.wrapping_add(1)
    } else if !(*p).pages.bop.is_null() && pdf_stream_length((*p).pages.bop) > 0i32 {
        pdf_add_array(contents_array, pdf_ref_obj((*p).pages.bop));
        count = count.wrapping_add(1)
    }
    if !(*page).content_refs[1].is_null() {
        /* background */
        pdf_add_array(contents_array, (*page).content_refs[1]);
        count = count.wrapping_add(1)
    }
    if !(*page).content_refs[2].is_null() {
        /* page body */
        pdf_add_array(contents_array, (*page).content_refs[2]);
        count = count.wrapping_add(1)
    }
    if !(*page).content_refs[3].is_null() {
        /* global eop */
        pdf_add_array(contents_array, (*page).content_refs[3]);
        count = count.wrapping_add(1)
    } else if !(*p).pages.eop.is_null() && pdf_stream_length((*p).pages.eop) > 0i32 {
        pdf_add_array(contents_array, pdf_ref_obj((*p).pages.eop));
        count = count.wrapping_add(1)
    }
    if count == 0_u32 {
        dpx_warning(b"Page with empty content found!!!\x00" as *const u8 as *const i8);
    }
    (*page).content_refs[0] = 0 as *mut pdf_obj;
    (*page).content_refs[1] = 0 as *mut pdf_obj;
    (*page).content_refs[2] = 0 as *mut pdf_obj;
    (*page).content_refs[3] = 0 as *mut pdf_obj;
    pdf_add_dict(
        (*page).page_obj,
        pdf_new_name(b"Contents\x00" as *const u8 as *const i8),
        contents_array,
    );
    if !(*page).annots.is_null() {
        pdf_add_dict(
            (*page).page_obj,
            pdf_new_name(b"Annots\x00" as *const u8 as *const i8),
            pdf_ref_obj((*page).annots),
        );
        pdf_release_obj((*page).annots);
    }
    if !(*page).beads.is_null() {
        pdf_add_dict(
            (*page).page_obj,
            pdf_new_name(b"B\x00" as *const u8 as *const i8),
            pdf_ref_obj((*page).beads),
        );
        pdf_release_obj((*page).beads);
    }
    pdf_release_obj((*page).page_obj);
    pdf_release_obj((*page).page_ref);
    (*page).page_obj = 0 as *mut pdf_obj;
    (*page).page_ref = 0 as *mut pdf_obj;
    (*page).annots = 0 as *mut pdf_obj;
    (*page).beads = 0 as *mut pdf_obj;
}
unsafe extern "C" fn build_page_tree(
    mut p: *mut pdf_doc,
    mut firstpage: *mut pdf_page,
    mut num_pages: i32,
    mut parent_ref: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut self_0: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut self_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut kids: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: i32 = 0;
    self_0 = pdf_new_dict();
    /*
     * This is a slight kludge which allow the subtree dictionary
     * generated by this routine to be merged with the real
     * page_tree dictionary, while keeping the indirect object
     * references right.
     */
    self_ref = if !parent_ref.is_null() {
        pdf_ref_obj(self_0)
    } else {
        pdf_ref_obj((*p).root.pages)
    };
    pdf_add_dict(
        self_0,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Pages\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        self_0,
        pdf_new_name(b"Count\x00" as *const u8 as *const i8),
        pdf_new_number(num_pages as f64),
    );
    if !parent_ref.is_null() {
        pdf_add_dict(
            self_0,
            pdf_new_name(b"Parent\x00" as *const u8 as *const i8),
            parent_ref,
        );
    }
    kids = pdf_new_array();
    if num_pages > 0i32 && num_pages <= 4i32 {
        i = 0i32;
        while i < num_pages {
            let mut page: *mut pdf_page = 0 as *mut pdf_page;
            page = firstpage.offset(i as isize);
            if (*page).page_ref.is_null() {
                (*page).page_ref = pdf_ref_obj((*page).page_obj)
            }
            pdf_add_array(kids, pdf_link_obj((*page).page_ref));
            doc_flush_page(p, page, pdf_link_obj(self_ref));
            i += 1
        }
    } else if num_pages > 0i32 {
        i = 0i32;
        while i < 4i32 {
            let mut start: i32 = 0;
            let mut end: i32 = 0;
            start = i * num_pages / 4i32;
            end = (i + 1i32) * num_pages / 4i32;
            if end - start > 1i32 {
                let mut subtree: *mut pdf_obj = 0 as *mut pdf_obj;
                subtree = build_page_tree(
                    p,
                    firstpage.offset(start as isize),
                    end - start,
                    pdf_link_obj(self_ref),
                );
                pdf_add_array(kids, pdf_ref_obj(subtree));
                pdf_release_obj(subtree);
            } else {
                let mut page_0: *mut pdf_page = 0 as *mut pdf_page;
                page_0 = firstpage.offset(start as isize);
                if (*page_0).page_ref.is_null() {
                    (*page_0).page_ref = pdf_ref_obj((*page_0).page_obj)
                }
                pdf_add_array(kids, pdf_link_obj((*page_0).page_ref));
                doc_flush_page(p, page_0, pdf_link_obj(self_ref));
            }
            i += 1
        }
    }
    pdf_add_dict(
        self_0,
        pdf_new_name(b"Kids\x00" as *const u8 as *const i8),
        kids,
    );
    pdf_release_obj(self_ref);
    return self_0;
}
unsafe extern "C" fn pdf_doc_init_page_tree(
    mut p: *mut pdf_doc,
    mut media_width: f64,
    mut media_height: f64,
) {
    /*
     * Create empty page tree.
     * The docroot.pages is kept open until the document is closed.
     * This allows the user to write to pages if he so choses.
     */
    (*p).root.pages = pdf_new_dict();
    (*p).pages.num_entries = 0_u32;
    (*p).pages.max_entries = 0_u32;
    (*p).pages.entries = 0 as *mut pdf_page;
    (*p).pages.bop = 0 as *mut pdf_obj;
    (*p).pages.eop = 0 as *mut pdf_obj;
    (*p).pages.mediabox.llx = 0.0f64;
    (*p).pages.mediabox.lly = 0.0f64;
    (*p).pages.mediabox.urx = media_width;
    (*p).pages.mediabox.ury = media_height;
}
unsafe extern "C" fn pdf_doc_close_page_tree(mut p: *mut pdf_doc) {
    let mut page_tree_root: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut mediabox: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut page_no: u32 = 0;
    /*
     * Do consistency check on forward references to pages.
     */
    page_no = (*p).pages.num_entries.wrapping_add(1_u32);
    while page_no <= (*p).pages.max_entries {
        let mut page: *mut pdf_page = 0 as *mut pdf_page;
        page = doc_get_page_entry(p, page_no);
        if !(*page).page_obj.is_null() {
            dpx_warning(
                b"Nonexistent page #%u refered.\x00" as *const u8 as *const i8,
                page_no,
            );
            pdf_release_obj((*page).page_ref);
            (*page).page_ref = 0 as *mut pdf_obj
        }
        if !(*page).page_obj.is_null() {
            dpx_warning(
                b"Entry for a nonexistent page #%u created.\x00" as *const u8 as *const i8,
                page_no,
            );
            pdf_release_obj((*page).page_obj);
            (*page).page_obj = 0 as *mut pdf_obj
        }
        if !(*page).annots.is_null() {
            dpx_warning(
                b"Annotation attached to a nonexistent page #%u.\x00" as *const u8 as *const i8,
                page_no,
            );
            pdf_release_obj((*page).annots);
            (*page).annots = 0 as *mut pdf_obj
        }
        if !(*page).beads.is_null() {
            dpx_warning(
                b"Article beads attached to a nonexistent page #%u.\x00" as *const u8 as *const i8,
                page_no,
            );
            pdf_release_obj((*page).beads);
            (*page).beads = 0 as *mut pdf_obj
        }
        if !(*page).resources.is_null() {
            pdf_release_obj((*page).resources);
            (*page).resources = 0 as *mut pdf_obj
        }
        page_no = page_no.wrapping_add(1)
    }
    /*
     * Connect page tree to root node.
     */
    page_tree_root = build_page_tree(
        p,
        &mut *(*p).pages.entries.offset(0),
        (*p).pages.num_entries as i32,
        0 as *mut pdf_obj,
    );
    pdf_merge_dict((*p).root.pages, page_tree_root);
    pdf_release_obj(page_tree_root);
    /* They must be after build_page_tree() */
    if !(*p).pages.bop.is_null() {
        pdf_add_stream(
            (*p).pages.bop,
            b"\n\x00" as *const u8 as *const i8 as *const libc::c_void,
            1i32,
        );
        pdf_release_obj((*p).pages.bop);
        (*p).pages.bop = 0 as *mut pdf_obj
    }
    if !(*p).pages.eop.is_null() {
        pdf_add_stream(
            (*p).pages.eop,
            b"\n\x00" as *const u8 as *const i8 as *const libc::c_void,
            1i32,
        );
        pdf_release_obj((*p).pages.eop);
        (*p).pages.eop = 0 as *mut pdf_obj
    }
    /* Create media box at root node and let the other pages inherit it. */
    mediabox = pdf_new_array();
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.llx / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.lly / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.urx / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.ury / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_dict(
        (*p).root.pages,
        pdf_new_name(b"MediaBox\x00" as *const u8 as *const i8),
        mediabox,
    );
    pdf_add_dict(
        (*p).root.dict,
        pdf_new_name(b"Pages\x00" as *const u8 as *const i8),
        pdf_ref_obj((*p).root.pages),
    );
    pdf_release_obj((*p).root.pages);
    (*p).root.pages = 0 as *mut pdf_obj;
    (*p).pages.entries = mfree((*p).pages.entries as *mut libc::c_void) as *mut pdf_page;
    (*p).pages.num_entries = 0_u32;
    (*p).pages.max_entries = 0_u32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_get_page_count(mut pf: *mut pdf_file) -> i32 {
    let mut count: i32 = 0i32;
    let mut page_tree: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut catalog: *mut pdf_obj = 0 as *mut pdf_obj;
    catalog = pdf_file_get_catalog(pf);
    page_tree = pdf_deref_obj(pdf_lookup_dict(
        catalog,
        b"Pages\x00" as *const u8 as *const i8,
    ));
    if !(!page_tree.is_null() && pdf_obj_typeof(page_tree) == 6i32) {
        return 0i32;
    }
    let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_lookup_dict(
        page_tree,
        b"Count\x00" as *const u8 as *const i8,
    ));
    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
        pdf_release_obj(tmp);
        return 0i32;
    }
    count = pdf_number_value(tmp) as i32;
    pdf_release_obj(tmp);
    return count;
}
/*
 * From PDFReference15_v6.pdf (p.119 and p.834)
 *
 * MediaBox rectangle (Required; inheritable)
 *
 * The media box defines the boundaries of the physical medium on which the
 * page is to be printed. It may include any extended area surrounding the
 * finished page for bleed, printing marks, or other such purposes. It may
 * also include areas close to the edges of the medium that cannot be marked
 * because of physical limitations of the output device. Content falling
 * outside this boundary can safely be discarded without affecting the
 * meaning of the PDF file.
 *
 * CropBox rectangle (Optional; inheritable)
 *
 * The crop box defines the region to which the contents of the page are to be
 * clipped (cropped) when displayed or printed. Unlike the other boxes, the
 * crop box has no defined meaning in terms of physical page geometry or
 * intended use; it merely imposes clipping on the page contents. However,
 * in the absence of additional information (such as imposition instructions
 * specified in a JDF or PJTF job ticket), the crop box will determine how
 * the page's contents are to be positioned on the output medium. The default
 * value is the page's media box.
 *
 * BleedBox rectangle (Optional; PDF 1.3)
 *
 * The bleed box (PDF 1.3) defines the region to which the contents of the
 * page should be clipped when output in a production environment. This may
 * include any extra "bleed area" needed to accommodate the physical
 * limitations of cutting, folding, and trimming equipment. The actual printed
 * page may include printing marks that fall outside the bleed box.
 * The default value is the page's crop box.
 *
 * TrimBox rectangle (Optional; PDF 1.3)
 *
 * The trim box (PDF 1.3) defines the intended dimensions of the finished page
 * after trimming. It may be smaller than the media box, to allow for
 * production-related content such as printing instructions, cut marks, or
 * color bars. The default value is the page's crop box.
 *
 * ArtBox rectangle (Optional; PDF 1.3)
 *
 * The art box (PDF 1.3) defines the extent of the page's meaningful content
 * (including potential white space) as intended by the page's creator.
 * The default value is the page's crop box.
 *
 * Rotate integer (Optional; inheritable)
 *
 * The number of degrees by which the page should be rotated clockwise when
 * displayed or printed. The value must be a multiple of 90. Default value: 0.
 */
/* count_p removed: Please use different interface if you want to get total page
 * number. pdf_doc_get_page() is obviously not an interface to do such.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_get_page(
    mut pf: *mut pdf_file,
    mut page_no: i32,
    mut options: i32,
    mut bbox: *mut pdf_rect,
    mut matrix: *mut pdf_tmatrix,
    mut resources_p: *mut *mut pdf_obj,
) -> *mut pdf_obj
/* returned values */ {
    let mut current_block: u64;
    let mut page_tree: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut resources: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut box_0: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut rotate: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut medbox: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut catalog: *mut pdf_obj = 0 as *mut pdf_obj;
    catalog = pdf_file_get_catalog(pf);
    page_tree = pdf_deref_obj(pdf_lookup_dict(
        catalog,
        b"Pages\x00" as *const u8 as *const i8,
    ));
    if !(!page_tree.is_null() && pdf_obj_typeof(page_tree) == 6i32) {
        current_block = 7715203803291643663;
    } else {
        let mut count: i32 = 0;
        let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"Count\x00" as *const u8 as *const i8,
        ));
        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 2i32) {
            pdf_release_obj(tmp);
            current_block = 7715203803291643663;
        } else {
            count = pdf_number_value(tmp) as i32;
            pdf_release_obj(tmp);
            if page_no <= 0i32 || page_no > count {
                dpx_warning(
                    b"Page %d does not exist.\x00" as *const u8 as *const i8,
                    page_no,
                );
                current_block = 5059794928954228255;
            } else {
                /*
                 * Seek correct page. Get MediaBox, CropBox and Resources.
                 * (Note that these entries can be inherited.)
                 */
                let mut art_box: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut trim_box: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut bleed_box: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut media_box: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut crop_box: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut kids: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut tmp_0: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut depth: i32 = 30i32;
                let mut page_idx: i32 = page_no - 1i32;
                let mut kids_length: i32 = 1i32;
                let mut i: i32 = 0i32;
                's_83: loop {
                    depth -= 1;
                    if !(depth != 0 && i != kids_length) {
                        current_block = 13707613154239713890;
                        break;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"MediaBox\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(media_box);
                        media_box = tmp_0
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"CropBox\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(crop_box);
                        crop_box = tmp_0
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"ArtBox\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(art_box);
                        art_box = tmp_0
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"TrimBox\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(trim_box);
                        trim_box = tmp_0
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"BleedBox\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(bleed_box);
                        bleed_box = tmp_0
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"Rotate\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(rotate);
                        rotate = tmp_0
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"Resources\x00" as *const u8 as *const i8,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(resources);
                        resources = tmp_0
                    }
                    kids = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"Kids\x00" as *const u8 as *const i8,
                    ));
                    if kids.is_null() {
                        current_block = 13707613154239713890;
                        break;
                    }
                    if !(!kids.is_null() && pdf_obj_typeof(kids) == 5i32) {
                        pdf_release_obj(kids);
                        current_block = 7715203803291643663;
                        break;
                    } else {
                        kids_length = pdf_array_length(kids) as i32;
                        i = 0i32;
                        while i < kids_length {
                            let mut count_0: i32 = 0;
                            pdf_release_obj(page_tree);
                            page_tree = pdf_deref_obj(pdf_get_array(kids, i));
                            if !(!page_tree.is_null() && pdf_obj_typeof(page_tree) == 6i32) {
                                current_block = 7715203803291643663;
                                break 's_83;
                            }
                            tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                                page_tree,
                                b"Count\x00" as *const u8 as *const i8,
                            ));
                            if !tmp_0.is_null() && pdf_obj_typeof(tmp_0) == 2i32 {
                                /* Pages object */
                                count_0 = pdf_number_value(tmp_0) as i32;
                                pdf_release_obj(tmp_0);
                            } else if tmp_0.is_null() {
                                /* Page object */
                                count_0 = 1i32
                            } else {
                                pdf_release_obj(tmp_0);
                                current_block = 7715203803291643663;
                                break 's_83;
                            }
                            if page_idx < count_0 {
                                break;
                            }
                            page_idx -= count_0;
                            i += 1
                        }
                        pdf_release_obj(kids);
                    }
                }
                match current_block {
                    7715203803291643663 => {}
                    _ => {
                        if depth == 0 || kids_length == i {
                            pdf_release_obj(media_box);
                            pdf_release_obj(crop_box);
                        } else {
                            /* Nasty BBox selection... */
                            if options == 0i32 || options == 1i32 {
                                if !crop_box.is_null() {
                                    box_0 = crop_box
                                } else {
                                    box_0 = media_box;
                                    if box_0.is_null()
                                        && {
                                            box_0 = bleed_box;
                                            box_0.is_null()
                                        }
                                        && {
                                            box_0 = trim_box;
                                            box_0.is_null()
                                        }
                                        && !art_box.is_null()
                                    {
                                        box_0 = art_box
                                    }
                                }
                            } else if options == 2i32 {
                                if !media_box.is_null() {
                                    box_0 = media_box
                                } else {
                                    box_0 = crop_box;
                                    if box_0.is_null()
                                        && {
                                            box_0 = bleed_box;
                                            box_0.is_null()
                                        }
                                        && {
                                            box_0 = trim_box;
                                            box_0.is_null()
                                        }
                                        && !art_box.is_null()
                                    {
                                        box_0 = art_box
                                    }
                                }
                            } else if options == 3i32 {
                                if !art_box.is_null() {
                                    box_0 = art_box
                                } else {
                                    box_0 = crop_box;
                                    if box_0.is_null()
                                        && {
                                            box_0 = media_box;
                                            box_0.is_null()
                                        }
                                        && {
                                            box_0 = bleed_box;
                                            box_0.is_null()
                                        }
                                        && !trim_box.is_null()
                                    {
                                        box_0 = trim_box
                                    }
                                }
                            } else if options == 4i32 {
                                if !trim_box.is_null() {
                                    box_0 = trim_box
                                } else {
                                    box_0 = crop_box;
                                    if box_0.is_null()
                                        && {
                                            box_0 = media_box;
                                            box_0.is_null()
                                        }
                                        && {
                                            box_0 = bleed_box;
                                            box_0.is_null()
                                        }
                                        && !art_box.is_null()
                                    {
                                        box_0 = art_box
                                    }
                                }
                            } else if options == 5i32 {
                                if !bleed_box.is_null() {
                                    box_0 = bleed_box
                                } else {
                                    box_0 = crop_box;
                                    if box_0.is_null()
                                        && {
                                            box_0 = media_box;
                                            box_0.is_null()
                                        }
                                        && {
                                            box_0 = trim_box;
                                            box_0.is_null()
                                        }
                                        && !art_box.is_null()
                                    {
                                        box_0 = art_box
                                    }
                                }
                            }
                            medbox = media_box;
                            if !(!(!box_0.is_null() && pdf_obj_typeof(box_0) == 5i32)
                                || pdf_array_length(box_0) != 4_u32
                                || !(!resources.is_null() && pdf_obj_typeof(resources) == 6i32))
                            {
                                let mut i_0: i32 = 0;
                                i_0 = 4i32;
                                loop {
                                    let fresh11 = i_0;
                                    i_0 = i_0 - 1;
                                    if !(fresh11 != 0) {
                                        current_block = 13014351284863956202;
                                        break;
                                    }
                                    let mut x: f64 = 0.;
                                    let mut tmp_1: *mut pdf_obj =
                                        pdf_deref_obj(pdf_get_array(box_0, i_0));
                                    if !(!tmp_1.is_null() && pdf_obj_typeof(tmp_1) == 2i32) {
                                        pdf_release_obj(tmp_1);
                                        current_block = 7715203803291643663;
                                        break;
                                    } else {
                                        x = pdf_number_value(tmp_1);
                                        match i_0 {
                                            0 => (*bbox).llx = x,
                                            1 => (*bbox).lly = x,
                                            2 => (*bbox).urx = x,
                                            3 => (*bbox).ury = x,
                                            _ => {}
                                        }
                                        pdf_release_obj(tmp_1);
                                    }
                                }
                                match current_block {
                                    7715203803291643663 => {}
                                    _ =>
                                    /* New scheme only for XDV files */
                                    {
                                        if !medbox.is_null() && (is_xdv != 0 || options != 0) {
                                            i_0 = 4i32;
                                            loop {
                                                let fresh12 = i_0;
                                                i_0 = i_0 - 1;
                                                if !(fresh12 != 0) {
                                                    current_block = 10570719081292997246;
                                                    break;
                                                }
                                                let mut x_0: f64 = 0.;
                                                let mut tmp_2: *mut pdf_obj =
                                                    pdf_deref_obj(pdf_get_array(medbox, i_0));
                                                if !(!tmp_2.is_null()
                                                    && pdf_obj_typeof(tmp_2) == 2i32)
                                                {
                                                    pdf_release_obj(tmp_2);
                                                    current_block = 7715203803291643663;
                                                    break;
                                                } else {
                                                    x_0 = pdf_number_value(tmp_2);
                                                    match i_0 {
                                                        0 => {
                                                            if (*bbox).llx < x_0 {
                                                                (*bbox).llx = x_0
                                                            }
                                                        }
                                                        1 => {
                                                            if (*bbox).lly < x_0 {
                                                                (*bbox).lly = x_0
                                                            }
                                                        }
                                                        2 => {
                                                            if (*bbox).urx > x_0 {
                                                                (*bbox).urx = x_0
                                                            }
                                                        }
                                                        3 => {
                                                            if (*bbox).ury > x_0 {
                                                                (*bbox).ury = x_0
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                    pdf_release_obj(tmp_2);
                                                }
                                            }
                                        } else {
                                            current_block = 10570719081292997246;
                                        }
                                        match current_block {
                                            7715203803291643663 => {}
                                            _ => {
                                                pdf_release_obj(box_0);
                                                (*matrix).d = 1.0f64;
                                                (*matrix).a = (*matrix).d;
                                                (*matrix).c = 0.0f64;
                                                (*matrix).b = (*matrix).c;
                                                (*matrix).f = 0.0f64;
                                                (*matrix).e = (*matrix).f;
                                                if !rotate.is_null()
                                                    && pdf_obj_typeof(rotate) == 2i32
                                                {
                                                    let mut deg: f64 = pdf_number_value(rotate);
                                                    if deg - deg as i32 as f64 != 0.0f64 {
                                                        dpx_warning(b"Invalid value specified for /Rotate: %f\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const i8,
                                                                    deg);
                                                    } else if deg != 0.0f64 {
                                                        let mut rot: i32 = deg as i32;
                                                        if (rot % 90i32) as f64 != 0.0f64 {
                                                            dpx_warning(b"Invalid value specified for /Rotate: %f\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const i8,
                                                                        deg);
                                                        } else {
                                                            rot = rot % 360i32;
                                                            if rot < 0i32 {
                                                                rot += 360i32
                                                            }
                                                            match rot {
                                                                90 => {
                                                                    (*matrix).d = 0i32 as f64;
                                                                    (*matrix).a = (*matrix).d;
                                                                    (*matrix).b = -1i32 as f64;
                                                                    (*matrix).c = 1i32 as f64;
                                                                    (*matrix).e =
                                                                        (*bbox).llx - (*bbox).lly;
                                                                    (*matrix).f =
                                                                        (*bbox).lly + (*bbox).urx
                                                                }
                                                                180 => {
                                                                    (*matrix).d = -1i32 as f64;
                                                                    (*matrix).a = (*matrix).d;
                                                                    (*matrix).c = 0i32 as f64;
                                                                    (*matrix).b = (*matrix).c;
                                                                    (*matrix).e =
                                                                        (*bbox).llx + (*bbox).urx;
                                                                    (*matrix).f =
                                                                        (*bbox).lly + (*bbox).ury
                                                                }
                                                                270 => {
                                                                    (*matrix).d = 0i32 as f64;
                                                                    (*matrix).a = (*matrix).d;
                                                                    (*matrix).b = 1i32 as f64;
                                                                    (*matrix).c = -1i32 as f64;
                                                                    (*matrix).e =
                                                                        (*bbox).llx + (*bbox).ury;
                                                                    (*matrix).f =
                                                                        (*bbox).lly - (*bbox).llx
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                    pdf_release_obj(rotate);
                                                    rotate = 0 as *mut pdf_obj;
                                                    current_block = 3151994457458062110;
                                                } else if !rotate.is_null() {
                                                    current_block = 7715203803291643663;
                                                } else {
                                                    current_block = 3151994457458062110;
                                                }
                                                match current_block {
                                                    7715203803291643663 => {}
                                                    _ => {
                                                        if !resources_p.is_null() {
                                                            *resources_p = resources
                                                        } else {
                                                            pdf_release_obj(resources);
                                                        }
                                                        return page_tree;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        current_block = 7715203803291643663;
                    }
                }
            }
        }
    }
    match current_block {
        7715203803291643663 => {
            dpx_warning(b"Cannot parse document. Broken PDF file?\x00" as *const u8 as *const i8);
        }
        _ => {}
    }
    pdf_release_obj(box_0);
    pdf_release_obj(rotate);
    pdf_release_obj(resources);
    pdf_release_obj(page_tree);
    return 0 as *mut pdf_obj;
}
unsafe extern "C" fn pdf_doc_init_bookmarks(mut p: *mut pdf_doc, mut bm_open_depth: i32) {
    let mut item: *mut pdf_olitem = 0 as *mut pdf_olitem;
    (*p).opt.outline_open_depth = (if bm_open_depth >= 0i32 {
        bm_open_depth as u32
    } else {
        256u32.wrapping_sub(bm_open_depth as u32)
    }) as i32;
    (*p).outlines.current_depth = 1i32;
    item = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_olitem>() as u64) as u32)
        as *mut pdf_olitem;
    (*item).dict = 0 as *mut pdf_obj;
    (*item).next = 0 as *mut pdf_olitem;
    (*item).first = 0 as *mut pdf_olitem;
    (*item).parent = 0 as *mut pdf_olitem;
    (*item).is_open = 1i32;
    (*p).outlines.current = item;
    (*p).outlines.first = item;
}
unsafe extern "C" fn clean_bookmarks(mut item: *mut pdf_olitem) -> i32 {
    let mut next: *mut pdf_olitem = 0 as *mut pdf_olitem;
    while !item.is_null() {
        next = (*item).next;
        pdf_release_obj((*item).dict);
        if !(*item).first.is_null() {
            clean_bookmarks((*item).first);
        }
        free(item as *mut libc::c_void);
        item = next
    }
    return 0i32;
}
unsafe extern "C" fn flush_bookmarks(
    mut node: *mut pdf_olitem,
    mut parent_ref: *mut pdf_obj,
    mut parent_dict: *mut pdf_obj,
) -> i32 {
    let mut retval: i32 = 0;
    let mut count: i32 = 0;
    let mut item: *mut pdf_olitem = 0 as *mut pdf_olitem;
    let mut this_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut prev_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut next_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!(*node).dict.is_null());
    this_ref = pdf_ref_obj((*node).dict);
    pdf_add_dict(
        parent_dict,
        pdf_new_name(b"First\x00" as *const u8 as *const i8),
        pdf_link_obj(this_ref),
    );
    retval = 0i32;
    item = node;
    prev_ref = 0 as *mut pdf_obj;
    while !item.is_null() && !(*item).dict.is_null() {
        if !(*item).first.is_null() && !(*(*item).first).dict.is_null() {
            count = flush_bookmarks((*item).first, this_ref, (*item).dict);
            if (*item).is_open != 0 {
                pdf_add_dict(
                    (*item).dict,
                    pdf_new_name(b"Count\x00" as *const u8 as *const i8),
                    pdf_new_number(count as f64),
                );
                retval += count
            } else {
                pdf_add_dict(
                    (*item).dict,
                    pdf_new_name(b"Count\x00" as *const u8 as *const i8),
                    pdf_new_number(-count as f64),
                );
            }
        }
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"Parent\x00" as *const u8 as *const i8),
            pdf_link_obj(parent_ref),
        );
        if !prev_ref.is_null() {
            pdf_add_dict(
                (*item).dict,
                pdf_new_name(b"Prev\x00" as *const u8 as *const i8),
                prev_ref,
            );
        }
        if !(*item).next.is_null() && !(*(*item).next).dict.is_null() {
            next_ref = pdf_ref_obj((*(*item).next).dict);
            pdf_add_dict(
                (*item).dict,
                pdf_new_name(b"Next\x00" as *const u8 as *const i8),
                pdf_link_obj(next_ref),
            );
        } else {
            next_ref = 0 as *mut pdf_obj
        }
        pdf_release_obj((*item).dict);
        (*item).dict = 0 as *mut pdf_obj;
        prev_ref = this_ref;
        this_ref = next_ref;
        retval += 1;
        item = (*item).next
    }
    pdf_add_dict(
        parent_dict,
        pdf_new_name(b"Last\x00" as *const u8 as *const i8),
        pdf_link_obj(prev_ref),
    );
    pdf_release_obj(prev_ref);
    pdf_release_obj((*node).dict);
    (*node).dict = 0 as *mut pdf_obj;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_bookmarks_up() -> i32 {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut parent: *mut pdf_olitem = 0 as *mut pdf_olitem;
    let mut item: *mut pdf_olitem = 0 as *mut pdf_olitem;
    item = (*p).outlines.current;
    if item.is_null() || (*item).parent.is_null() {
        dpx_warning(b"Can\'t go up above the bookmark root node!\x00" as *const u8 as *const i8);
        return -1i32;
    }
    parent = (*item).parent;
    item = (*parent).next;
    if (*parent).next.is_null() {
        item = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_olitem>() as u64) as u32)
            as *mut pdf_olitem;
        (*parent).next = item;
        (*item).dict = 0 as *mut pdf_obj;
        (*item).first = 0 as *mut pdf_olitem;
        (*item).next = 0 as *mut pdf_olitem;
        (*item).is_open = 0i32;
        (*item).parent = (*parent).parent
    }
    (*p).outlines.current = item;
    (*p).outlines.current_depth -= 1;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_bookmarks_down() -> i32 {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut item: *mut pdf_olitem = 0 as *mut pdf_olitem;
    let mut first: *mut pdf_olitem = 0 as *mut pdf_olitem;
    item = (*p).outlines.current;
    if (*item).dict.is_null() {
        let mut tcolor: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut action: *mut pdf_obj = 0 as *mut pdf_obj;
        dpx_warning(b"Empty bookmark node!\x00" as *const u8 as *const i8);
        dpx_warning(b"You have tried to jump more than 1 level.\x00" as *const u8 as *const i8);
        (*item).dict = pdf_new_dict();
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"Title\x00" as *const u8 as *const i8),
            pdf_new_string(
                b"<No Title>\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"<No Title>\x00" as *const u8 as *const i8),
            ),
        );
        tcolor = pdf_new_array();
        pdf_add_array(tcolor, pdf_new_number(1.0f64));
        pdf_add_array(tcolor, pdf_new_number(0.0f64));
        pdf_add_array(tcolor, pdf_new_number(0.0f64));
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"C\x00" as *const u8 as *const i8),
            pdf_link_obj(tcolor),
        );
        pdf_release_obj(tcolor);
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"F\x00" as *const u8 as *const i8),
            pdf_new_number(1.0f64),
        );
        action = pdf_new_dict();
        pdf_add_dict(
            action,
            pdf_new_name(b"S\x00" as *const u8 as *const i8),
            pdf_new_name(b"JavaScript\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(action,
                     pdf_new_name(b"JS\x00" as *const u8 as
                                      *const i8),
                     pdf_new_string(b"app.alert(\"The author of this document made this bookmark item empty!\", 3, 0)\x00"
                                        as *const u8 as *const i8 as
                                        *const libc::c_void,
                                    strlen(b"app.alert(\"The author of this document made this bookmark item empty!\", 3, 0)\x00"
                                               as *const u8 as
                                               *const i8)));
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"A\x00" as *const u8 as *const i8),
            pdf_link_obj(action),
        );
        pdf_release_obj(action);
    }
    first = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_olitem>() as u64) as u32)
        as *mut pdf_olitem;
    (*item).first = first;
    (*first).dict = 0 as *mut pdf_obj;
    (*first).is_open = 0i32;
    (*first).parent = item;
    (*first).next = 0 as *mut pdf_olitem;
    (*first).first = 0 as *mut pdf_olitem;
    (*p).outlines.current = first;
    (*p).outlines.current_depth += 1;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_bookmarks_depth() -> i32 {
    let mut p: *mut pdf_doc = &mut pdoc;
    return (*p).outlines.current_depth;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_bookmarks_add(mut dict: *mut pdf_obj, mut is_open: i32) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut item: *mut pdf_olitem = 0 as *mut pdf_olitem;
    let mut next: *mut pdf_olitem = 0 as *mut pdf_olitem;
    assert!(!p.is_null() && !dict.is_null());
    item = (*p).outlines.current;
    if item.is_null() {
        item = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_olitem>() as u64) as u32)
            as *mut pdf_olitem;
        (*item).parent = 0 as *mut pdf_olitem;
        (*p).outlines.first = item
    } else if !(*item).dict.is_null() {
        /* go to next item */
        item = (*item).next
    }
    (*item).dict = dict;
    (*item).first = 0 as *mut pdf_olitem;
    (*item).is_open = if is_open < 0i32 {
        if (*p).outlines.current_depth > (*p).opt.outline_open_depth {
            0i32
        } else {
            1i32
        }
    } else {
        is_open
    };
    next = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_olitem>() as u64) as u32)
        as *mut pdf_olitem;
    (*item).next = next;
    (*next).dict = 0 as *mut pdf_obj;
    (*next).parent = (*item).parent;
    (*next).first = 0 as *mut pdf_olitem;
    (*next).is_open = -1i32;
    (*next).next = 0 as *mut pdf_olitem;
    (*p).outlines.current = item;
    pdf_doc_add_goto(dict);
}
unsafe extern "C" fn pdf_doc_close_bookmarks(mut p: *mut pdf_doc) {
    let mut catalog: *mut pdf_obj = (*p).root.dict;
    let mut item: *mut pdf_olitem = 0 as *mut pdf_olitem;
    let mut count: i32 = 0;
    let mut bm_root: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut bm_root_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    item = (*p).outlines.first;
    if !(*item).dict.is_null() {
        bm_root = pdf_new_dict();
        bm_root_ref = pdf_ref_obj(bm_root);
        count = flush_bookmarks(item, bm_root_ref, bm_root);
        pdf_add_dict(
            bm_root,
            pdf_new_name(b"Count\x00" as *const u8 as *const i8),
            pdf_new_number(count as f64),
        );
        pdf_add_dict(
            catalog,
            pdf_new_name(b"Outlines\x00" as *const u8 as *const i8),
            bm_root_ref,
        );
        pdf_release_obj(bm_root);
    }
    clean_bookmarks(item);
    (*p).outlines.first = 0 as *mut pdf_olitem;
    (*p).outlines.current = 0 as *mut pdf_olitem;
    (*p).outlines.current_depth = 0i32;
}
static mut name_dict_categories: [*const i8; 10] = [
    b"Dests\x00" as *const u8 as *const i8,
    b"AP\x00" as *const u8 as *const i8,
    b"JavaScript\x00" as *const u8 as *const i8,
    b"Pages\x00" as *const u8 as *const i8,
    b"Templates\x00" as *const u8 as *const i8,
    b"IDS\x00" as *const u8 as *const i8,
    b"URLS\x00" as *const u8 as *const i8,
    b"EmbeddedFiles\x00" as *const u8 as *const i8,
    b"AlternatePresentations\x00" as *const u8 as *const i8,
    b"Renditions\x00" as *const u8 as *const i8,
];
unsafe extern "C" fn pdf_doc_init_names(mut p: *mut pdf_doc, mut check_gotos: i32) {
    let mut i: u32 = 0;
    (*p).root.names = 0 as *mut pdf_obj;
    (*p).names = new(((::std::mem::size_of::<[*const i8; 10]>() as u64)
        .wrapping_div(::std::mem::size_of::<*const i8>() as u64)
        .wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<name_dict>() as u64) as u32)
        as *mut name_dict;
    i = 0_u32;
    while (i as u64)
        < (::std::mem::size_of::<[*const i8; 10]>() as u64)
            .wrapping_div(::std::mem::size_of::<*const i8>() as u64)
    {
        let ref mut fresh13 = (*(*p).names.offset(i as isize)).category;
        *fresh13 = name_dict_categories[i as usize];
        let ref mut fresh14 = (*(*p).names.offset(i as isize)).data;
        *fresh14 = if strcmp(
            name_dict_categories[i as usize],
            b"Dests\x00" as *const u8 as *const i8,
        ) != 0
        {
            0 as *mut ht_table
        } else {
            pdf_new_name_tree()
        };
        i = i.wrapping_add(1)
        /*
         * We need a non-null entry for PDF destinations in order to find
         * broken links even if no destination is defined in the DVI file.
         */
    }
    let ref mut fresh15 = (*(*p).names.offset(
        (::std::mem::size_of::<[*const i8; 10]>() as u64)
            .wrapping_div(::std::mem::size_of::<*const i8>() as u64) as isize,
    ))
    .category;
    *fresh15 = 0 as *const i8;
    let ref mut fresh16 = (*(*p).names.offset(
        (::std::mem::size_of::<[*const i8; 10]>() as u64)
            .wrapping_div(::std::mem::size_of::<*const i8>() as u64) as isize,
    ))
    .data;
    *fresh16 = 0 as *mut ht_table;
    (*p).check_gotos = check_gotos;
    ht_init_table(
        &mut (*p).gotos,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut pdf_obj) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            pdf_release_obj as unsafe extern "C" fn(_: *mut pdf_obj) -> (),
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_add_names(
    mut category: *const i8,
    mut key: *const libc::c_void,
    mut keylen: i32,
    mut value: *mut pdf_obj,
) -> i32 {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut i: u32 = 0;
    i = 0_u32;
    while !(*(*p).names.offset(i as isize)).category.is_null() {
        if streq_ptr((*(*p).names.offset(i as isize)).category, category) {
            break;
        }
        i = i.wrapping_add(1)
    }
    if (*(*p).names.offset(i as isize)).category.is_null() {
        dpx_warning(
            b"Unknown name dictionary category \"%s\".\x00" as *const u8 as *const i8,
            category,
        );
        return -1i32;
    }
    if (*(*p).names.offset(i as isize)).data.is_null() {
        let ref mut fresh17 = (*(*p).names.offset(i as isize)).data;
        *fresh17 = pdf_new_name_tree()
    }
    return pdf_names_add_object((*(*p).names.offset(i as isize)).data, key, keylen, value);
}
unsafe extern "C" fn pdf_doc_add_goto(mut annot_dict: *mut pdf_obj) {
    let mut current_block: u64;
    let mut subtype: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut A: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut S: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut D: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut D_new: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dest: *const i8 = 0 as *const i8;
    let mut key: *const i8 = 0 as *const i8;
    let mut destlen: i32 = 0i32;
    if pdoc.check_gotos == 0 {
        return;
    }
    /*
     * An annotation dictionary coming from an annotation special
     * must have a "Subtype". An annotation dictionary coming from
     * an outline special has none.
     */
    subtype = pdf_deref_obj(pdf_lookup_dict(
        annot_dict,
        b"Subtype\x00" as *const u8 as *const i8,
    ));
    if !subtype.is_null() {
        if !subtype.is_null() && pdf_obj_typeof(subtype) == 10i32 {
            current_block = 14825911176647684745;
        } else if !(!subtype.is_null() && pdf_obj_typeof(subtype) == 4i32) {
            current_block = 10743935136377679094;
        } else if strcmp(
            pdf_name_value(subtype),
            b"Link\x00" as *const u8 as *const i8,
        ) != 0
        {
            current_block = 6401626691277551363;
        } else {
            current_block = 3276175668257526147;
        }
    } else {
        current_block = 3276175668257526147;
    }
    match current_block {
        3276175668257526147 => {
            dict = annot_dict;
            key = b"Dest\x00" as *const u8 as *const i8;
            D = pdf_deref_obj(pdf_lookup_dict(annot_dict, key));
            if !D.is_null() && pdf_obj_typeof(D) == 10i32 {
                current_block = 14825911176647684745;
            } else {
                A = pdf_deref_obj(pdf_lookup_dict(
                    annot_dict,
                    b"A\x00" as *const u8 as *const i8,
                ));
                if !A.is_null() {
                    if !A.is_null() && pdf_obj_typeof(A) == 10i32 {
                        current_block = 14825911176647684745;
                    } else if !D.is_null() || !(!A.is_null() && pdf_obj_typeof(A) == 6i32) {
                        current_block = 10743935136377679094;
                    } else {
                        S = pdf_deref_obj(pdf_lookup_dict(A, b"S\x00" as *const u8 as *const i8));
                        if !S.is_null() && pdf_obj_typeof(S) == 10i32 {
                            current_block = 14825911176647684745;
                        } else if !(!S.is_null() && pdf_obj_typeof(S) == 4i32) {
                            current_block = 10743935136377679094;
                        } else if strcmp(pdf_name_value(S), b"GoTo\x00" as *const u8 as *const i8)
                            != 0
                        {
                            current_block = 6401626691277551363;
                        } else {
                            dict = A;
                            key = b"D\x00" as *const u8 as *const i8;
                            D = pdf_deref_obj(pdf_lookup_dict(A, key));
                            current_block = 9828876828309294594;
                        }
                    }
                } else {
                    current_block = 9828876828309294594;
                }
                match current_block {
                    14825911176647684745 => {}
                    10743935136377679094 => {}
                    6401626691277551363 => {}
                    _ => {
                        if !D.is_null() && pdf_obj_typeof(D) == 3i32 {
                            dest = pdf_string_value(D) as *mut i8;
                            destlen = pdf_string_length(D) as i32;
                            D_new = ht_lookup_table(
                                &mut pdoc.gotos,
                                dest as *const libc::c_void,
                                destlen,
                            ) as *mut pdf_obj;
                            if D_new.is_null() {
                                let mut buf: [i8; 10] = [0; 10];
                                /* We use hexadecimal notation for our numeric destinations.
                                 * Other bases (e.g., 10+26 or 10+2*26) would be more efficient.
                                 */
                                sprintf(
                                    buf.as_mut_ptr(),
                                    b"%x\x00" as *const u8 as *const i8,
                                    ht_table_size(&mut pdoc.gotos),
                                ); /* Maybe reference */
                                D_new = pdf_new_string(
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    strlen(buf.as_mut_ptr()),
                                );
                                ht_append_table(
                                    &mut pdoc.gotos,
                                    dest as *const libc::c_void,
                                    destlen,
                                    D_new as *mut libc::c_void,
                                );
                            }
                            let mut key_obj: *mut pdf_obj = pdf_new_name(key);
                            if pdf_add_dict(dict, key_obj, pdf_link_obj(D_new)) == 0 {
                                pdf_release_obj(key_obj);
                            }
                            current_block = 6401626691277551363;
                        } else if !D.is_null() && pdf_obj_typeof(D) == 5i32 {
                            current_block = 6401626691277551363;
                        } else if !D.is_null() && pdf_obj_typeof(D) == 10i32 {
                            current_block = 14825911176647684745;
                        } else {
                            current_block = 10743935136377679094;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        14825911176647684745 => {
            dpx_warning(b"Cannot optimize PDF annotations. Output file may be broken. Please restart with option \"-C 0x10\"\n\x00"
                            as *const u8 as *const i8);
        }
        10743935136377679094 => {
            dpx_warning(
                b"Unknown PDF annotation format. Output file may be broken.\x00" as *const u8
                    as *const i8,
            );
        }
        _ => {}
    }
    pdf_release_obj(subtype);
    pdf_release_obj(A);
    pdf_release_obj(S);
    pdf_release_obj(D);
}
unsafe extern "C" fn warn_undef_dests(mut dests: *mut ht_table, mut gotos: *mut ht_table) {
    let mut iter: ht_iter = ht_iter {
        index: 0,
        curr: 0 as *mut libc::c_void,
        hash: 0 as *mut ht_table,
    };
    if ht_set_iter(gotos, &mut iter) < 0i32 {
        return;
    }
    loop {
        let mut keylen: i32 = 0;
        let mut key: *mut i8 = ht_iter_getkey(&mut iter, &mut keylen);
        if ht_lookup_table(dests, key as *const libc::c_void, keylen).is_null() {
            let mut dest: *mut i8 = new(((keylen + 1i32) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            memcpy(
                dest as *mut libc::c_void,
                key as *const libc::c_void,
                keylen as u64,
            );
            *dest.offset(keylen as isize) = 0_i8;
            dpx_warning(
                b"PDF destination \"%s\" not defined.\x00" as *const u8 as *const i8,
                dest,
            );
            free(dest as *mut libc::c_void);
        }
        if !(ht_iter_next(&mut iter) >= 0i32) {
            break;
        }
    }
    ht_clear_iter(&mut iter);
}
unsafe extern "C" fn pdf_doc_close_names(mut p: *mut pdf_doc) {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: u32 = 0;
    i = 0_u32;
    while !(*(*p).names.offset(i as isize)).category.is_null() {
        if !(*(*p).names.offset(i as isize)).data.is_null() {
            let mut data: *mut ht_table = (*(*p).names.offset(i as isize)).data;
            let mut name_tree: *mut pdf_obj = 0 as *mut pdf_obj;
            let mut count: i32 = 0;
            if pdoc.check_gotos == 0
                || strcmp(
                    (*(*p).names.offset(i as isize)).category,
                    b"Dests\x00" as *const u8 as *const i8,
                ) != 0
            {
                name_tree = pdf_names_create_tree(data, &mut count, 0 as *mut ht_table)
            } else {
                name_tree = pdf_names_create_tree(data, &mut count, &mut pdoc.gotos);
                if verbose != 0 && count < (*data).count {
                    dpx_message(
                        b"\nRemoved %d unused PDF destinations\n\x00" as *const u8 as *const i8,
                        (*data).count - count,
                    );
                }
                if count < pdoc.gotos.count {
                    warn_undef_dests(data, &mut pdoc.gotos);
                }
            }
            if !name_tree.is_null() {
                if (*p).root.names.is_null() {
                    (*p).root.names = pdf_new_dict()
                }
                pdf_add_dict(
                    (*p).root.names,
                    pdf_new_name((*(*p).names.offset(i as isize)).category),
                    pdf_ref_obj(name_tree),
                );
                pdf_release_obj(name_tree);
            }
            pdf_delete_name_tree(&mut (*(*p).names.offset(i as isize)).data);
        }
        i = i.wrapping_add(1)
    }
    if !(*p).root.names.is_null() {
        tmp = pdf_lookup_dict((*p).root.dict, b"Names\x00" as *const u8 as *const i8);
        if tmp.is_null() {
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"Names\x00" as *const u8 as *const i8),
                pdf_ref_obj((*p).root.names),
            );
        } else if !tmp.is_null() && pdf_obj_typeof(tmp) == 6i32 {
            pdf_merge_dict((*p).root.names, tmp);
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"Names\x00" as *const u8 as *const i8),
                pdf_ref_obj((*p).root.names),
            );
        } else {
            /* What should I do? */
            dpx_warning(b"Could not modify Names dictionary.\x00" as *const u8 as *const i8);
        }
        pdf_release_obj((*p).root.names);
        (*p).root.names = 0 as *mut pdf_obj
    }
    (*p).names = mfree((*p).names as *mut libc::c_void) as *mut name_dict;
    ht_clear_table(&mut (*p).gotos);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_add_annot(
    mut page_no: u32,
    mut rect: *const pdf_rect,
    mut annot_dict: *mut pdf_obj,
    mut new_annot: i32,
) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut page: *mut pdf_page = 0 as *mut pdf_page;
    let mut rect_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut annot_grow: f64 = (*p).opt.annot_grow;
    let mut xpos: f64 = 0.;
    let mut ypos: f64 = 0.;
    let mut annbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    page = doc_get_page_entry(p, page_no);
    if (*page).annots.is_null() {
        (*page).annots = pdf_new_array()
    }
    let mut mediabox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    pdf_doc_get_mediabox(page_no, &mut mediabox);
    pdf_dev_get_coord(&mut xpos, &mut ypos);
    annbox.llx = (*rect).llx - xpos;
    annbox.lly = (*rect).lly - ypos;
    annbox.urx = (*rect).urx - xpos;
    annbox.ury = (*rect).ury - ypos;
    if annbox.llx < mediabox.llx
        || annbox.urx > mediabox.urx
        || annbox.lly < mediabox.lly
        || annbox.ury > mediabox.ury
    {
        dpx_warning(b"Annotation out of page boundary.\x00" as *const u8 as *const i8);
        dpx_warning(
            b"Current page\'s MediaBox: [%g %g %g %g]\x00" as *const u8 as *const i8,
            mediabox.llx,
            mediabox.lly,
            mediabox.urx,
            mediabox.ury,
        );
        dpx_warning(
            b"Annotation: [%g %g %g %g]\x00" as *const u8 as *const i8,
            annbox.llx,
            annbox.lly,
            annbox.urx,
            annbox.ury,
        );
        dpx_warning(b"Maybe incorrect paper size specified.\x00" as *const u8 as *const i8);
    }
    if annbox.llx > annbox.urx || annbox.lly > annbox.ury {
        dpx_warning(
            b"Rectangle with negative width/height: [%g %g %g %g]\x00" as *const u8 as *const i8,
            annbox.llx,
            annbox.lly,
            annbox.urx,
            annbox.ury,
        );
    }
    rect_array = pdf_new_array();
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((annbox.llx - annot_grow) / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((annbox.lly - annot_grow) / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((annbox.urx + annot_grow) / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((annbox.ury + annot_grow) / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_dict(
        annot_dict,
        pdf_new_name(b"Rect\x00" as *const u8 as *const i8),
        rect_array,
    );
    pdf_add_array((*page).annots, pdf_ref_obj(annot_dict));
    if new_annot != 0 {
        pdf_doc_add_goto(annot_dict);
    };
}
/*
 * PDF Article Thread
 */
unsafe extern "C" fn pdf_doc_init_articles(mut p: *mut pdf_doc) {
    (*p).root.threads = 0 as *mut pdf_obj;
    (*p).articles.num_entries = 0_u32;
    (*p).articles.max_entries = 0_u32;
    (*p).articles.entries = 0 as *mut pdf_article;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_begin_article(
    mut article_id: *const i8,
    mut article_info: *mut pdf_obj,
) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut article: *mut pdf_article = 0 as *mut pdf_article;
    if article_id.is_null() || strlen(article_id) == 0i32 as u64 {
        _tt_abort(b"Article thread without internal identifier.\x00" as *const u8 as *const i8);
    }
    if (*p).articles.num_entries >= (*p).articles.max_entries {
        (*p).articles.max_entries = (*p).articles.max_entries.wrapping_add(16_u32);
        (*p).articles.entries = renew(
            (*p).articles.entries as *mut libc::c_void,
            ((*p).articles.max_entries as u64)
                .wrapping_mul(::std::mem::size_of::<pdf_article>() as u64) as u32,
        ) as *mut pdf_article
    }
    article = &mut *(*p)
        .articles
        .entries
        .offset((*p).articles.num_entries as isize) as *mut pdf_article;
    (*article).id = new((strlen(article_id).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*article).id, article_id);
    (*article).info = article_info;
    (*article).num_beads = 0_u32;
    (*article).max_beads = 0_u32;
    (*article).beads = 0 as *mut pdf_bead;
    (*p).articles.num_entries = (*p).articles.num_entries.wrapping_add(1);
}
unsafe extern "C" fn find_bead(
    mut article: *mut pdf_article,
    mut bead_id: *const i8,
) -> *mut pdf_bead {
    let mut bead: *mut pdf_bead = 0 as *mut pdf_bead;
    let mut i: u32 = 0;
    bead = 0 as *mut pdf_bead;
    i = 0_u32;
    while i < (*article).num_beads {
        if streq_ptr((*(*article).beads.offset(i as isize)).id, bead_id) {
            bead = &mut *(*article).beads.offset(i as isize) as *mut pdf_bead;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    return bead;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_add_bead(
    mut article_id: *const i8,
    mut bead_id: *const i8,
    mut page_no: i32,
    mut rect: *const pdf_rect,
) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut article: *mut pdf_article = 0 as *mut pdf_article;
    let mut bead: *mut pdf_bead = 0 as *mut pdf_bead;
    let mut i: u32 = 0;
    if article_id.is_null() {
        _tt_abort(b"No article identifier specified.\x00" as *const u8 as *const i8);
    }
    article = 0 as *mut pdf_article;
    i = 0_u32;
    while i < (*p).articles.num_entries {
        if streq_ptr((*(*p).articles.entries.offset(i as isize)).id, article_id) {
            article = &mut *(*p).articles.entries.offset(i as isize) as *mut pdf_article;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    if article.is_null() {
        _tt_abort(b"Specified article thread that doesn\'t exist.\x00" as *const u8 as *const i8);
    }
    bead = if !bead_id.is_null() {
        find_bead(article, bead_id)
    } else {
        0 as *mut pdf_bead
    };
    if bead.is_null() {
        if (*article).num_beads >= (*article).max_beads {
            (*article).max_beads = (*article).max_beads.wrapping_add(16_u32);
            (*article).beads = renew(
                (*article).beads as *mut libc::c_void,
                ((*article).max_beads as u64).wrapping_mul(::std::mem::size_of::<pdf_bead>() as u64)
                    as u32,
            ) as *mut pdf_bead;
            i = (*article).num_beads;
            while i < (*article).max_beads {
                let ref mut fresh18 = (*(*article).beads.offset(i as isize)).id;
                *fresh18 = 0 as *mut i8;
                (*(*article).beads.offset(i as isize)).page_no = -1i32;
                i = i.wrapping_add(1)
            }
        }
        bead = &mut *(*article).beads.offset((*article).num_beads as isize) as *mut pdf_bead;
        if !bead_id.is_null() {
            (*bead).id = new((strlen(bead_id).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            strcpy((*bead).id, bead_id);
        } else {
            (*bead).id = 0 as *mut i8
        }
        (*article).num_beads = (*article).num_beads.wrapping_add(1)
    }
    (*bead).rect.llx = (*rect).llx;
    (*bead).rect.lly = (*rect).lly;
    (*bead).rect.urx = (*rect).urx;
    (*bead).rect.ury = (*rect).ury;
    (*bead).page_no = page_no;
}
unsafe extern "C" fn make_article(
    mut p: *mut pdf_doc,
    mut article: *mut pdf_article,
    mut bead_ids: *mut *const i8,
    mut num_beads: u32,
    mut article_info: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut art_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut first: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut prev: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut last: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    if article.is_null() {
        return 0 as *mut pdf_obj;
    }
    art_dict = pdf_new_dict();
    last = 0 as *mut pdf_obj;
    prev = last;
    first = prev;
    /*
     * The bead_ids represents logical order of beads in an article thread.
     * If bead_ids is not given, we create an article thread in the order of
     * beads appeared.
     */
    n = (if !bead_ids.is_null() {
        num_beads
    } else {
        (*article).num_beads
    }) as i32;
    i = 0i32;
    while i < n {
        let mut bead: *mut pdf_bead = 0 as *mut pdf_bead;
        bead = if !bead_ids.is_null() {
            find_bead(article, *bead_ids.offset(i as isize))
        } else {
            &mut *(*article).beads.offset(i as isize) as *mut pdf_bead
        };
        if !(bead.is_null() || (*bead).page_no < 0i32) {
            last = pdf_new_dict();
            if prev.is_null() {
                first = last;
                pdf_add_dict(
                    first,
                    pdf_new_name(b"T\x00" as *const u8 as *const i8),
                    pdf_ref_obj(art_dict),
                );
            } else {
                pdf_add_dict(
                    prev,
                    pdf_new_name(b"N\x00" as *const u8 as *const i8),
                    pdf_ref_obj(last),
                );
                pdf_add_dict(
                    last,
                    pdf_new_name(b"V\x00" as *const u8 as *const i8),
                    pdf_ref_obj(prev),
                );
                /* We must link first to last. */
                if prev != first {
                    pdf_release_obj(prev);
                }
            }
            /* Realize bead now. */
            let mut page: *mut pdf_page = 0 as *mut pdf_page;
            let mut rect: *mut pdf_obj = 0 as *mut pdf_obj;
            page = doc_get_page_entry(p, (*bead).page_no as u32);
            if (*page).beads.is_null() {
                (*page).beads = pdf_new_array()
            }
            pdf_add_dict(
                last,
                pdf_new_name(b"P\x00" as *const u8 as *const i8),
                pdf_link_obj((*page).page_ref),
            );
            rect = pdf_new_array();
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.llx / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.lly / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.urx / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.ury / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_dict(last, pdf_new_name(b"R\x00" as *const u8 as *const i8), rect);
            pdf_add_array((*page).beads, pdf_ref_obj(last));
            prev = last
        }
        i += 1
    }
    if !first.is_null() && !last.is_null() {
        pdf_add_dict(
            last,
            pdf_new_name(b"N\x00" as *const u8 as *const i8),
            pdf_ref_obj(first),
        );
        pdf_add_dict(
            first,
            pdf_new_name(b"V\x00" as *const u8 as *const i8),
            pdf_ref_obj(last),
        );
        if first != last {
            pdf_release_obj(last);
        }
        pdf_add_dict(
            art_dict,
            pdf_new_name(b"F\x00" as *const u8 as *const i8),
            pdf_ref_obj(first),
        );
        /* If article_info is supplied, we override article->info. */
        if !article_info.is_null() {
            pdf_add_dict(
                art_dict,
                pdf_new_name(b"I\x00" as *const u8 as *const i8),
                article_info,
            );
        } else if !(*article).info.is_null() {
            pdf_add_dict(
                art_dict,
                pdf_new_name(b"I\x00" as *const u8 as *const i8),
                pdf_ref_obj((*article).info),
            );
            pdf_release_obj((*article).info);
            (*article).info = 0 as *mut pdf_obj
            /* We do not write as object reference. */
        }
        pdf_release_obj(first);
    } else {
        pdf_release_obj(art_dict);
        art_dict = 0 as *mut pdf_obj
    }
    return art_dict;
}
unsafe extern "C" fn clean_article(mut article: *mut pdf_article) {
    if article.is_null() {
        return;
    }
    if !(*article).beads.is_null() {
        let mut i: u32 = 0;
        i = 0_u32;
        while i < (*article).num_beads {
            free((*(*article).beads.offset(i as isize)).id as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        (*article).beads = mfree((*article).beads as *mut libc::c_void) as *mut pdf_bead
    }
    (*article).id = mfree((*article).id as *mut libc::c_void) as *mut i8;
    (*article).num_beads = 0_u32;
    (*article).max_beads = 0_u32;
}
unsafe extern "C" fn pdf_doc_close_articles(mut p: *mut pdf_doc) {
    let mut i: u32 = 0;
    i = 0_u32;
    while i < (*p).articles.num_entries {
        let mut article: *mut pdf_article = 0 as *mut pdf_article;
        article = &mut *(*p).articles.entries.offset(i as isize) as *mut pdf_article;
        if !(*article).beads.is_null() {
            let mut art_dict: *mut pdf_obj = 0 as *mut pdf_obj;
            art_dict = make_article(p, article, 0 as *mut *const i8, 0_u32, 0 as *mut pdf_obj);
            if (*p).root.threads.is_null() {
                (*p).root.threads = pdf_new_array()
            }
            pdf_add_array((*p).root.threads, pdf_ref_obj(art_dict));
            pdf_release_obj(art_dict);
        }
        clean_article(article);
        i = i.wrapping_add(1)
    }
    (*p).articles.entries = mfree((*p).articles.entries as *mut libc::c_void) as *mut pdf_article;
    (*p).articles.num_entries = 0_u32;
    (*p).articles.max_entries = 0_u32;
    if !(*p).root.threads.is_null() {
        pdf_add_dict(
            (*p).root.dict,
            pdf_new_name(b"Threads\x00" as *const u8 as *const i8),
            pdf_ref_obj((*p).root.threads),
        );
        pdf_release_obj((*p).root.threads);
        (*p).root.threads = 0 as *mut pdf_obj
    };
}
/* page_no = 0 for root page tree node. */
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_set_mediabox(mut page_no: u32, mut mediabox: *const pdf_rect) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut page: *mut pdf_page = 0 as *mut pdf_page;
    if page_no == 0_u32 {
        (*p).pages.mediabox.llx = (*mediabox).llx;
        (*p).pages.mediabox.lly = (*mediabox).lly;
        (*p).pages.mediabox.urx = (*mediabox).urx;
        (*p).pages.mediabox.ury = (*mediabox).ury
    } else {
        page = doc_get_page_entry(p, page_no);
        (*page).cropbox.llx = (*mediabox).llx;
        (*page).cropbox.lly = (*mediabox).lly;
        (*page).cropbox.urx = (*mediabox).urx;
        (*page).cropbox.ury = (*mediabox).ury;
        (*page).flags |= 1i32 << 0i32
    };
}
unsafe extern "C" fn pdf_doc_get_mediabox(mut page_no: u32, mut mediabox: *mut pdf_rect) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut page: *mut pdf_page = 0 as *mut pdf_page;
    if page_no == 0_u32 {
        (*mediabox).llx = (*p).pages.mediabox.llx;
        (*mediabox).lly = (*p).pages.mediabox.lly;
        (*mediabox).urx = (*p).pages.mediabox.urx;
        (*mediabox).ury = (*p).pages.mediabox.ury
    } else {
        page = doc_get_page_entry(p, page_no);
        if (*page).flags & 1i32 << 0i32 != 0 {
            (*mediabox).llx = (*page).cropbox.llx;
            (*mediabox).lly = (*page).cropbox.lly;
            (*mediabox).urx = (*page).cropbox.urx;
            (*mediabox).ury = (*page).cropbox.ury
        } else {
            (*mediabox).llx = (*p).pages.mediabox.llx;
            (*mediabox).lly = (*p).pages.mediabox.lly;
            (*mediabox).urx = (*p).pages.mediabox.urx;
            (*mediabox).ury = (*p).pages.mediabox.ury
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_current_page_resources() -> *mut pdf_obj {
    let mut resources: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
    if !(*p).pending_forms.is_null() {
        if !(*(*p).pending_forms).form.resources.is_null() {
            resources = (*(*p).pending_forms).form.resources
        } else {
            (*(*p).pending_forms).form.resources = pdf_new_dict();
            resources = (*(*p).pending_forms).form.resources
        }
    } else {
        currentpage =
            &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        if !(*currentpage).resources.is_null() {
            resources = (*currentpage).resources
        } else {
            (*currentpage).resources = pdf_new_dict();
            resources = (*currentpage).resources
        }
    }
    return resources;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_get_dictionary(mut category: *const i8) -> *mut pdf_obj {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!category.is_null());
    if streq_ptr(category, b"Names\x00" as *const u8 as *const i8) {
        if (*p).root.names.is_null() {
            (*p).root.names = pdf_new_dict()
        }
        dict = (*p).root.names
    } else if streq_ptr(category, b"Pages\x00" as *const u8 as *const i8) {
        if (*p).root.pages.is_null() {
            (*p).root.pages = pdf_new_dict()
        }
        dict = (*p).root.pages
    } else if streq_ptr(category, b"Catalog\x00" as *const u8 as *const i8) {
        if (*p).root.dict.is_null() {
            (*p).root.dict = pdf_new_dict()
        }
        dict = (*p).root.dict
    } else if streq_ptr(category, b"Info\x00" as *const u8 as *const i8) {
        if (*p).info.is_null() {
            (*p).info = pdf_new_dict()
        }
        dict = (*p).info
    } else if streq_ptr(category, b"@THISPAGE\x00" as *const u8 as *const i8) {
        /* Sorry for this... */
        let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
        currentpage =
            &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        dict = (*currentpage).page_obj
    }
    if dict.is_null() {
        _tt_abort(
            b"Document dict. \"%s\" not exist. \x00" as *const u8 as *const i8,
            category,
        );
    }
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_current_page_number() -> i32 {
    let mut p: *mut pdf_doc = &mut pdoc;
    return (*p).pages.num_entries.wrapping_add(1_u32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_ref_page(mut page_no: u32) -> *mut pdf_obj {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut page: *mut pdf_page = 0 as *mut pdf_page;
    page = doc_get_page_entry(p, page_no);
    if (*page).page_obj.is_null() {
        (*page).page_obj = pdf_new_dict();
        (*page).page_ref = pdf_ref_obj((*page).page_obj)
    }
    return pdf_link_obj((*page).page_ref);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_get_reference(mut category: *const i8) -> *mut pdf_obj {
    let mut ref_0: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut page_no: i32 = 0;
    assert!(!category.is_null());
    page_no = pdf_doc_current_page_number();
    if streq_ptr(category, b"@THISPAGE\x00" as *const u8 as *const i8) {
        ref_0 = pdf_doc_ref_page(page_no as u32)
    } else if streq_ptr(category, b"@PREVPAGE\x00" as *const u8 as *const i8) {
        if page_no <= 1i32 {
            _tt_abort(
                b"Reference to previous page, but no pages have been completed yet.\x00"
                    as *const u8 as *const i8,
            );
        }
        ref_0 = pdf_doc_ref_page((page_no - 1i32) as u32)
    } else if streq_ptr(category, b"@NEXTPAGE\x00" as *const u8 as *const i8) {
        ref_0 = pdf_doc_ref_page((page_no + 1i32) as u32)
    }
    if ref_0.is_null() {
        _tt_abort(
            b"Reference to \"%s\" not exist. \x00" as *const u8 as *const i8,
            category,
        );
    }
    return ref_0;
}
unsafe extern "C" fn pdf_doc_new_page(mut p: *mut pdf_doc) {
    let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
    if (*p).pages.num_entries >= (*p).pages.max_entries {
        doc_resize_page_entries(p, (*p).pages.max_entries.wrapping_add(128u32));
    }
    /*
     * This is confusing. pdf_doc_finish_page() have increased page count!
     */
    currentpage = &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
    /* Was this page already instantiated by a forward reference to it? */
    if (*currentpage).page_ref.is_null() {
        (*currentpage).page_obj = pdf_new_dict();
        (*currentpage).page_ref = pdf_ref_obj((*currentpage).page_obj)
    }
    (*currentpage).background = 0 as *mut pdf_obj;
    (*currentpage).contents = pdf_new_stream(1i32 << 0i32);
    (*currentpage).resources = pdf_new_dict();
    (*currentpage).annots = 0 as *mut pdf_obj;
    (*currentpage).beads = 0 as *mut pdf_obj;
}
/* This only closes contents and resources. */
unsafe extern "C" fn pdf_doc_finish_page(mut p: *mut pdf_doc) {
    let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
    if !(*p).pending_forms.is_null() {
        _tt_abort(b"A pending form XObject at the end of page.\x00" as *const u8 as *const i8);
    }
    currentpage = &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
    if (*currentpage).page_obj.is_null() {
        (*currentpage).page_obj = pdf_new_dict()
    }
    /*
     * Make Contents array.
     */
    /*
     * Global BOP content stream.
     * pdf_ref_obj() returns reference itself when the object is
     * indirect reference, not reference to the indirect reference.
     * We keep bop itself but not reference to it since it is
     * expected to be small.
     */
    if !(*p).pages.bop.is_null() && pdf_stream_length((*p).pages.bop) > 0i32 {
        (*currentpage).content_refs[0] = pdf_ref_obj((*p).pages.bop)
    } else {
        (*currentpage).content_refs[0] = 0 as *mut pdf_obj
    }
    /*
     * Current page background content stream.
     */
    if !(*currentpage).background.is_null() {
        if pdf_stream_length((*currentpage).background) > 0i32 {
            (*currentpage).content_refs[1] = pdf_ref_obj((*currentpage).background);
            pdf_add_stream(
                (*currentpage).background,
                b"\n\x00" as *const u8 as *const i8 as *const libc::c_void,
                1i32,
            );
        }
        pdf_release_obj((*currentpage).background);
        (*currentpage).background = 0 as *mut pdf_obj
    } else {
        (*currentpage).content_refs[1] = 0 as *mut pdf_obj
    }
    /* Content body of current page */
    (*currentpage).content_refs[2] = pdf_ref_obj((*currentpage).contents);
    pdf_add_stream(
        (*currentpage).contents,
        b"\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        1i32,
    );
    pdf_release_obj((*currentpage).contents);
    (*currentpage).contents = 0 as *mut pdf_obj;
    /*
     * Global EOP content stream.
     */
    if !(*p).pages.eop.is_null() && pdf_stream_length((*p).pages.eop) > 0i32 {
        (*currentpage).content_refs[3] = pdf_ref_obj((*p).pages.eop)
    } else {
        (*currentpage).content_refs[3] = 0 as *mut pdf_obj
    }
    /*
     * Page resources.
     */
    if !(*currentpage).resources.is_null() {
        let mut procset: *mut pdf_obj = 0 as *mut pdf_obj;
        /*
         * ProcSet is obsolete in PDF-1.4 but recommended for compatibility.
         */
        procset = pdf_new_array();
        pdf_add_array(procset, pdf_new_name(b"PDF\x00" as *const u8 as *const i8));
        pdf_add_array(procset, pdf_new_name(b"Text\x00" as *const u8 as *const i8));
        pdf_add_array(
            procset,
            pdf_new_name(b"ImageC\x00" as *const u8 as *const i8),
        );
        pdf_add_array(
            procset,
            pdf_new_name(b"ImageB\x00" as *const u8 as *const i8),
        );
        pdf_add_array(
            procset,
            pdf_new_name(b"ImageI\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            (*currentpage).resources,
            pdf_new_name(b"ProcSet\x00" as *const u8 as *const i8),
            procset,
        );
        pdf_add_dict(
            (*currentpage).page_obj,
            pdf_new_name(b"Resources\x00" as *const u8 as *const i8),
            pdf_ref_obj((*currentpage).resources),
        );
        pdf_release_obj((*currentpage).resources);
        (*currentpage).resources = 0 as *mut pdf_obj
    }
    if manual_thumb_enabled != 0 {
        let mut thumb_filename: *mut i8 = 0 as *mut i8;
        let mut thumb_ref: *mut pdf_obj = 0 as *mut pdf_obj;
        thumb_filename = new(
            (strlen(thumb_basename).wrapping_add(7i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        sprintf(
            thumb_filename,
            b"%s.%ld\x00" as *const u8 as *const i8,
            thumb_basename,
            (*p).pages.num_entries.wrapping_rem(99999_u32) as i64 + 1,
        );
        thumb_ref = read_thumbnail(thumb_filename);
        free(thumb_filename as *mut libc::c_void);
        if !thumb_ref.is_null() {
            pdf_add_dict(
                (*currentpage).page_obj,
                pdf_new_name(b"Thumb\x00" as *const u8 as *const i8),
                thumb_ref,
            );
        }
    }
    (*p).pages.num_entries = (*p).pages.num_entries.wrapping_add(1);
}
static mut bgcolor: pdf_color = {
    let mut init = pdf_color {
        num_components: 1i32,
        spot_color_name: 0 as *const i8 as *mut i8,
        values: [1.0f64, 0., 0., 0.],
    };
    init
};
/* Manual thumbnail */
/* Similar to bop_content */
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_set_bgcolor(mut color: *const pdf_color) {
    if !color.is_null() {
        pdf_color_copycolor(&mut bgcolor, color);
    } else {
        /* as clear... */
        pdf_color_graycolor(&mut bgcolor, 1.0f64);
    };
}
unsafe extern "C" fn doc_fill_page_background(mut p: *mut pdf_doc) {
    let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut cm: i32 = 0;
    let mut saved_content: *mut pdf_obj = 0 as *mut pdf_obj;
    cm = pdf_dev_get_param(2i32);
    if cm == 0 || pdf_color_is_white(&mut bgcolor) as i32 != 0 {
        return;
    }
    pdf_doc_get_mediabox(pdf_doc_current_page_number() as u32, &mut r);
    currentpage = &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
    assert!(!currentpage.is_null());
    if (*currentpage).background.is_null() {
        (*currentpage).background = pdf_new_stream(1i32 << 0i32)
    }
    saved_content = (*currentpage).contents;
    (*currentpage).contents = (*currentpage).background;
    pdf_dev_gsave();
    pdf_dev_set_color(&mut bgcolor, 0x20_i8, 0i32);
    pdf_dev_rectfill(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
    pdf_dev_grestore();
    (*currentpage).contents = saved_content;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_begin_page(mut scale: f64, mut x_origin: f64, mut y_origin: f64) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.a = scale;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = scale;
    M.e = x_origin;
    M.f = y_origin;
    /* pdf_doc_new_page() allocates page content stream. */
    pdf_doc_new_page(p);
    pdf_dev_bop(&mut M);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_end_page() {
    let mut p: *mut pdf_doc = &mut pdoc;
    pdf_dev_eop();
    doc_fill_page_background(p);
    pdf_doc_finish_page(p);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_add_page_content(mut buffer: *const i8, mut length: u32) {
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut currentpage: *mut pdf_page = 0 as *mut pdf_page;
    if !(*p).pending_forms.is_null() {
        pdf_add_stream(
            (*(*p).pending_forms).form.contents,
            buffer as *const libc::c_void,
            length as i32,
        );
    } else {
        currentpage =
            &mut *(*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        pdf_add_stream(
            (*currentpage).contents,
            buffer as *const libc::c_void,
            length as i32,
        );
    };
}
static mut doccreator: *mut i8 = 0 as *const i8 as *mut i8;
/* Ugh */
#[no_mangle]
pub unsafe extern "C" fn pdf_open_document(
    mut filename: *const i8,
    mut enable_encrypt: bool,
    mut enable_object_stream: bool,
    mut media_width: f64,
    mut media_height: f64,
    mut annot_grow_amount: f64,
    mut bookmark_open_depth: i32,
    mut check_gotos: i32,
) {
    let mut p: *mut pdf_doc = &mut pdoc;
    pdf_out_init(filename, enable_encrypt, enable_object_stream);
    pdf_doc_init_catalog(p);
    (*p).opt.annot_grow = annot_grow_amount;
    (*p).opt.outline_open_depth = bookmark_open_depth;
    pdf_init_resources();
    pdf_init_colors();
    pdf_init_fonts();
    /* Thumbnail want this to be initialized... */
    pdf_init_images();
    pdf_doc_init_docinfo(p);
    if !doccreator.is_null() {
        pdf_add_dict(
            (*p).info,
            pdf_new_name(b"Creator\x00" as *const u8 as *const i8),
            pdf_new_string(doccreator as *const libc::c_void, strlen(doccreator)),
        );
        doccreator = mfree(doccreator as *mut libc::c_void) as *mut i8
    }
    pdf_doc_init_bookmarks(p, bookmark_open_depth);
    pdf_doc_init_articles(p);
    pdf_doc_init_names(p, check_gotos);
    pdf_doc_init_page_tree(p, media_width, media_height);
    pdf_doc_set_bgcolor(0 as *const pdf_color);
    if enable_encrypt {
        let mut encrypt: *mut pdf_obj = pdf_encrypt_obj();
        pdf_set_encrypt(encrypt);
        pdf_release_obj(encrypt);
    }
    pdf_set_id(pdf_enc_id_array());
    /* Create a default name for thumbnail image files */
    if manual_thumb_enabled != 0 {
        let mut fn_len: size_t = strlen(filename);
        if fn_len > 4i32 as u64
            && strncmp(
                b".pdf\x00" as *const u8 as *const i8,
                filename.offset(fn_len as isize).offset(-4),
                4i32 as u64,
            ) == 0
        {
            thumb_basename = new(
                (fn_len.wrapping_sub(4i32 as u64).wrapping_add(1i32 as u64) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
            ) as *mut i8;
            strncpy(thumb_basename, filename, fn_len.wrapping_sub(4i32 as u64));
            *thumb_basename.offset(fn_len.wrapping_sub(4i32 as u64) as isize) = 0_i8
        } else {
            thumb_basename = new((fn_len.wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            strcpy(thumb_basename, filename);
        }
    }
    (*p).pending_forms = 0 as *mut form_list_node;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_set_creator(mut creator: *const i8) {
    if creator.is_null() || *creator.offset(0) as i32 == '\u{0}' as i32 {
        return;
    }
    doccreator = new((strlen(creator).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(doccreator, creator);
    /* Ugh */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close_document() {
    let mut p: *mut pdf_doc = &mut pdoc;
    /*
     * Following things were kept around so user can add dictionary items.
     */
    pdf_doc_close_articles(p); /* Should be at last. */
    pdf_doc_close_names(p);
    pdf_doc_close_bookmarks(p);
    pdf_doc_close_page_tree(p);
    pdf_doc_close_docinfo(p);
    pdf_doc_close_catalog(p);
    pdf_close_images();
    pdf_close_fonts();
    pdf_close_colors();
    pdf_close_resources();
    pdf_out_flush();
    free(thumb_basename as *mut libc::c_void);
}
/*
 * All this routine does is give the form a name and add a unity scaling matrix.
 * It fills in required fields.  The caller must initialize the stream.
 */
unsafe extern "C" fn pdf_doc_make_xform(
    mut xform: *mut pdf_obj,
    mut bbox: *mut pdf_rect,
    mut matrix: *mut pdf_tmatrix,
    mut resources: *mut pdf_obj,
    mut attrib: *mut pdf_obj,
) {
    let mut xform_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    xform_dict = pdf_stream_dict(xform);
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"XObject\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"Form\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"FormType\x00" as *const u8 as *const i8),
        pdf_new_number(1.0f64),
    );
    if bbox.is_null() {
        _tt_abort(b"No BoundingBox supplied.\x00" as *const u8 as *const i8);
    }
    tmp = pdf_new_array();
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).llx / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).lly / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).urx / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).ury / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"BBox\x00" as *const u8 as *const i8),
        tmp,
    );
    if !matrix.is_null() {
        tmp = pdf_new_array();
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).a / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).b / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).c / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).d / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).e / 0.001f64 + 0.5f64) * 0.001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).f / 0.001f64 + 0.5f64) * 0.001f64),
        );
        pdf_add_dict(
            xform_dict,
            pdf_new_name(b"Matrix\x00" as *const u8 as *const i8),
            tmp,
        );
    }
    if !attrib.is_null() {
        pdf_merge_dict(xform_dict, attrib);
    }
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"Resources\x00" as *const u8 as *const i8),
        resources,
    );
}
/*
 * begin_form_xobj creates an xobject with its "origin" at
 * xpos and ypos that is clipped to the specified bbox. Note
 * that the origin is not the lower left corner of the bbox.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_begin_grabbing(
    mut ident: *const i8,
    mut ref_x: f64,
    mut ref_y: f64,
    mut cropbox: *const pdf_rect,
) -> i32 {
    let mut xobj_id: i32 = -1i32;
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut form: *mut pdf_form = 0 as *mut pdf_form;
    let mut fnode: *mut form_list_node = 0 as *mut form_list_node;
    let mut info: xform_info = xform_info {
        flags: 0,
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
    };
    pdf_dev_push_gstate();
    fnode = new((1_u64).wrapping_mul(::std::mem::size_of::<form_list_node>() as u64) as u32)
        as *mut form_list_node;
    (*fnode).prev = (*p).pending_forms;
    (*fnode).q_depth = pdf_dev_current_depth();
    form = &mut (*fnode).form;
    /*
     * The reference point of an Xobject is at the lower left corner
     * of the bounding box.  Since we would like to have an arbitrary
     * reference point, we use a transformation matrix, translating
     * the reference point to (0,0).
     */
    (*form).matrix.a = 1.0f64;
    (*form).matrix.b = 0.0f64;
    (*form).matrix.c = 0.0f64;
    (*form).matrix.d = 1.0f64;
    (*form).matrix.e = -ref_x;
    (*form).matrix.f = -ref_y;
    (*form).cropbox.llx = ref_x + (*cropbox).llx;
    (*form).cropbox.lly = ref_y + (*cropbox).lly;
    (*form).cropbox.urx = ref_x + (*cropbox).urx;
    (*form).cropbox.ury = ref_y + (*cropbox).ury;
    (*form).contents = pdf_new_stream(1i32 << 0i32);
    (*form).resources = pdf_new_dict();
    pdf_ximage_init_form_info(&mut info);
    info.matrix.a = 1.0f64;
    info.matrix.b = 0.0f64;
    info.matrix.c = 0.0f64;
    info.matrix.d = 1.0f64;
    info.matrix.e = -ref_x;
    info.matrix.f = -ref_y;
    info.bbox.llx = (*cropbox).llx;
    info.bbox.lly = (*cropbox).lly;
    info.bbox.urx = (*cropbox).urx;
    info.bbox.ury = (*cropbox).ury;
    /* Use reference since content itself isn't available yet. */
    xobj_id = pdf_ximage_defineresource(
        ident,
        0i32,
        &mut info as *mut xform_info as *mut libc::c_void,
        pdf_ref_obj((*form).contents),
    );
    (*p).pending_forms = fnode;
    /*
     * Make sure the object is self-contained by adding the
     * current font and color to the object stream.
     */
    pdf_dev_reset_fonts(1i32); /* force color operators to be added to stream */
    pdf_dev_reset_color(1i32);
    return xobj_id;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_end_grabbing(mut attrib: *mut pdf_obj) {
    let mut form: *mut pdf_form = 0 as *mut pdf_form;
    let mut procset: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *mut pdf_doc = &mut pdoc;
    let mut fnode: *mut form_list_node = 0 as *mut form_list_node;
    if (*p).pending_forms.is_null() {
        dpx_warning(b"Tried to close a nonexistent form XOject.\x00" as *const u8 as *const i8);
        return;
    }
    fnode = (*p).pending_forms;
    form = &mut (*fnode).form;
    pdf_dev_grestore_to((*fnode).q_depth);
    /*
     * ProcSet is obsolete in PDF-1.4 but recommended for compatibility.
     */
    procset = pdf_new_array();
    pdf_add_array(procset, pdf_new_name(b"PDF\x00" as *const u8 as *const i8));
    pdf_add_array(procset, pdf_new_name(b"Text\x00" as *const u8 as *const i8));
    pdf_add_array(
        procset,
        pdf_new_name(b"ImageC\x00" as *const u8 as *const i8),
    );
    pdf_add_array(
        procset,
        pdf_new_name(b"ImageB\x00" as *const u8 as *const i8),
    );
    pdf_add_array(
        procset,
        pdf_new_name(b"ImageI\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        (*form).resources,
        pdf_new_name(b"ProcSet\x00" as *const u8 as *const i8),
        procset,
    );
    pdf_doc_make_xform(
        (*form).contents,
        &mut (*form).cropbox,
        &mut (*form).matrix,
        pdf_ref_obj((*form).resources),
        attrib,
    );
    pdf_release_obj((*form).resources);
    pdf_release_obj((*form).contents);
    pdf_release_obj(attrib);
    (*p).pending_forms = (*fnode).prev;
    pdf_dev_pop_gstate();
    pdf_dev_reset_fonts(1i32);
    pdf_dev_reset_color(0i32);
    free(fnode as *mut libc::c_void);
}
static mut breaking_state: C2RustUnnamed_4 = {
    let mut init = C2RustUnnamed_4 {
        dirty: 0i32,
        broken: 0i32,
        annot_dict: 0 as *const pdf_obj as *mut pdf_obj,
        rect: {
            let mut init = pdf_rect {
                llx: 0.0f64,
                lly: 0.0f64,
                urx: 0.0f64,
                ury: 0.0f64,
            };
            init
        },
    };
    init
};
unsafe extern "C" fn reset_box() {
    breaking_state.rect.lly = ::std::f64::INFINITY;
    breaking_state.rect.llx = breaking_state.rect.lly;
    breaking_state.rect.ury = -::std::f64::INFINITY;
    breaking_state.rect.urx = breaking_state.rect.ury;
    breaking_state.dirty = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_begin_annot(mut dict: *mut pdf_obj) {
    breaking_state.annot_dict = dict;
    breaking_state.broken = 0i32;
    reset_box();
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_end_annot() {
    pdf_doc_break_annot();
    breaking_state.annot_dict = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_break_annot() {
    if breaking_state.dirty != 0 {
        let mut annot_dict: *mut pdf_obj = 0 as *mut pdf_obj;
        /* Copy dict */
        annot_dict = pdf_new_dict();
        pdf_merge_dict(annot_dict, breaking_state.annot_dict);
        pdf_doc_add_annot(
            pdf_doc_current_page_number() as u32,
            &mut breaking_state.rect,
            annot_dict,
            (breaking_state.broken == 0) as i32,
        );
        pdf_release_obj(annot_dict);
        breaking_state.broken = 1i32
    }
    reset_box();
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
/* PDF document metadata */
/* They just return PDF dictionary object.
 * Callers are completely responsible for doing right thing...
 */
/* Not really managing tree...
 * There should be something for number tree.
 */
/* Page */
/* Article thread */
/* Bookmarks */
/* Returns xobj_id of started xform. */
/* Annotation */
/* Annotation with auto- clip and line (or page) break */
#[no_mangle]
pub unsafe extern "C" fn pdf_doc_expand_box(mut rect: *const pdf_rect) {
    breaking_state.rect.llx = if breaking_state.rect.llx < (*rect).llx {
        breaking_state.rect.llx
    } else {
        (*rect).llx
    };
    breaking_state.rect.lly = if breaking_state.rect.lly < (*rect).lly {
        breaking_state.rect.lly
    } else {
        (*rect).lly
    };
    breaking_state.rect.urx = if breaking_state.rect.urx > (*rect).urx {
        breaking_state.rect.urx
    } else {
        (*rect).urx
    };
    breaking_state.rect.ury = if breaking_state.rect.ury > (*rect).ury {
        breaking_state.rect.ury
    } else {
        (*rect).ury
    };
    breaking_state.dirty = 1i32;
}
