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
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ::std::ffi::VaList)
        -> libc::c_int;
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn dvi_dev_xpos() -> libc::c_double;
    #[no_mangle]
    fn dvi_dev_ypos() -> libc::c_double;
    #[no_mangle]
    fn dvi_untag_depth();
    #[no_mangle]
    fn dvi_tag_depth();
    #[no_mangle]
    fn dvi_link_annot(flag: libc::c_int);
    /* They just return PDF dictionary object.
     * Callers are completely responsible for doing right thing...
     */
    #[no_mangle]
    fn pdf_doc_get_dictionary(category: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_get_reference(category: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_current_page_number() -> libc::c_int;
    #[no_mangle]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_ref_page(page_no: libc::c_uint) -> *mut pdf_obj;
    /* Annotation with auto- clip and line (or page) break */
    #[no_mangle]
    fn pdf_doc_begin_annot(dict: *mut pdf_obj);
    #[no_mangle]
    fn pdf_doc_end_annot();
    #[no_mangle]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
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
        keylen: libc::c_int,
        object: *mut pdf_obj,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_names_lookup_reference(
        names: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_names_lookup_object(
        names: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_names_close_object(
        names: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
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
    /* Please remove this */
    #[no_mangle]
    fn dump(start: *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
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
    fn spc_color_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_color_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
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
    fn spc_dvipdfmx_check_special(buf: *const libc::c_char, len: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_dvipdfmx_setup_handler(
        sph: *mut spc_handler,
        spe: *mut spc_env,
        ap: *mut spc_arg,
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
    fn spc_dvips_at_begin_document() -> libc::c_int;
    #[no_mangle]
    fn spc_dvips_at_end_document() -> libc::c_int;
    #[no_mangle]
    fn spc_dvips_at_begin_page() -> libc::c_int;
    #[no_mangle]
    fn spc_dvips_at_end_page() -> libc::c_int;
    #[no_mangle]
    fn spc_dvips_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_dvips_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
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
    fn spc_html_at_begin_page() -> libc::c_int;
    #[no_mangle]
    fn spc_html_at_end_page() -> libc::c_int;
    #[no_mangle]
    fn spc_html_at_begin_document() -> libc::c_int;
    #[no_mangle]
    fn spc_html_at_end_document() -> libc::c_int;
    #[no_mangle]
    fn spc_html_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_html_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
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
    fn spc_misc_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_misc_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
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
    fn spc_pdfm_at_begin_document() -> libc::c_int;
    #[no_mangle]
    fn spc_pdfm_at_end_document() -> libc::c_int;
    #[no_mangle]
    fn spc_pdfm_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_pdfm_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> libc::c_int;
    #[no_mangle]
    fn spc_tpic_at_begin_page() -> libc::c_int;
    #[no_mangle]
    fn spc_tpic_at_end_page() -> libc::c_int;
    #[no_mangle]
    fn spc_tpic_at_begin_document() -> libc::c_int;
    #[no_mangle]
    fn spc_tpic_at_end_document() -> libc::c_int;
    #[no_mangle]
    fn spc_tpic_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_tpic_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> libc::c_int;
    /*  This is xdvipdfmx, an extended version of dvipdfmx,
        an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2013-2016 by the dvipdfmx project team.

        Copyright (c) 2006 SIL International
        Originally written by Jonathan Kew

        Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn spc_xtx_check_special(buffer: *const libc::c_char, size: libc::c_int) -> bool;
    #[no_mangle]
    fn spc_xtx_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
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
pub type va_list = __builtin_va_list;
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
    pub curptr: *const libc::c_char,
    pub endptr: *const libc::c_char,
    pub base: *const libc::c_char,
    pub command: *const libc::c_char,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const libc::c_char,
    pub exec: spc_handler_fn_ptr,
}
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
pub struct ht_table {
    pub count: libc::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut libc::c_char,
    pub keylen: libc::c_int,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub key: *const libc::c_char,
    pub bodhk_func: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub eodhk_func: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub bophk_func: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub eophk_func: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub check_func: Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool>,
    pub setup_func: Option<
        unsafe extern "C" fn(_: *mut spc_handler, _: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
    >,
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
static mut verbose: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn spc_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn spc_warn(
    mut spe: *mut spc_env,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    ap = args.clone();
    vsprintf(buf.as_mut_ptr(), fmt, ap.as_va_list());
    dpx_warning(
        b"%s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    );
}
/* This is currently just to make other spc_xxx to not directly
 * call dvi_xxx.
 */
#[no_mangle]
pub unsafe extern "C" fn spc_begin_annot(
    mut spe: *mut spc_env,
    mut dict: *mut pdf_obj,
) -> libc::c_int {
    pdf_doc_begin_annot(dict); /* Tell dvi interpreter to handle line-break. */
    dvi_tag_depth();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_end_annot(mut spe: *mut spc_env) -> libc::c_int {
    dvi_untag_depth();
    pdf_doc_end_annot();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_resume_annot(mut spe: *mut spc_env) -> libc::c_int {
    dvi_link_annot(1i32);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_suspend_annot(mut spe: *mut spc_env) -> libc::c_int {
    dvi_link_annot(0i32);
    return 0i32;
}
static mut named_objects: *mut ht_table = 0 as *const ht_table as *mut ht_table;
/* reserved keys */
static mut _rkeys: [*const libc::c_char; 11] = [
    b"xpos\x00" as *const u8 as *const libc::c_char,
    b"ypos\x00" as *const u8 as *const libc::c_char,
    b"thispage\x00" as *const u8 as *const libc::c_char,
    b"prevpage\x00" as *const u8 as *const libc::c_char,
    b"nextpage\x00" as *const u8 as *const libc::c_char,
    b"resources\x00" as *const u8 as *const libc::c_char,
    b"pages\x00" as *const u8 as *const libc::c_char,
    b"names\x00" as *const u8 as *const libc::c_char,
    b"catalog\x00" as *const u8 as *const libc::c_char,
    b"docinfo\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/* pageN where N is a positive integer.
 * Note that page need not exist at this time.
 */
unsafe extern "C" fn ispageref(mut key: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if strlen(key) <= strlen(b"page\x00" as *const u8 as *const libc::c_char)
        || memcmp(
            key as *const libc::c_void,
            b"page\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"page\x00" as *const u8 as *const libc::c_char),
        ) != 0
    {
        return 0i32;
    } else {
        p = key.offset(4);
        while *p as libc::c_int != 0
            && *p as libc::c_int >= '0' as i32
            && *p as libc::c_int <= '9' as i32
        {
            p = p.offset(1)
        }
        if *p as libc::c_int != '\u{0}' as i32 {
            return 0i32;
        }
    }
    return 1i32;
}
/*
 * The following routine returns copies, not the original object.
 */
#[no_mangle]
pub unsafe extern "C" fn spc_lookup_reference(mut key: *const libc::c_char) -> *mut pdf_obj {
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut k: libc::c_int = 0;
    if !named_objects.is_null() {
    } else {
        __assert_fail(
            b"named_objects\x00" as *const u8 as *const libc::c_char,
            b"dpx-specials.c\x00" as *const u8 as *const libc::c_char,
            162i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"pdf_obj *spc_lookup_reference(const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if key.is_null() {
        return 0 as *mut pdf_obj;
    }
    k = 0i32;
    while !_rkeys[k as usize].is_null() && strcmp(key, _rkeys[k as usize]) != 0 {
        k += 1
    }
    match k {
        0 => {
            /* xpos and ypos must be position in device space here. */
            cp.x = dvi_dev_xpos();
            cp.y = 0.0f64;
            pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
            value = pdf_new_number(floor(cp.x / 0.01f64 + 0.5f64) * 0.01f64)
        }
        1 => {
            cp.x = 0.0f64;
            cp.y = dvi_dev_ypos();
            pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
            value = pdf_new_number(floor(cp.y / 0.01f64 + 0.5f64) * 0.01f64)
        }
        2 => value = pdf_doc_get_reference(b"@THISPAGE\x00" as *const u8 as *const libc::c_char),
        3 => value = pdf_doc_get_reference(b"@PREVPAGE\x00" as *const u8 as *const libc::c_char),
        4 => value = pdf_doc_get_reference(b"@NEXTPAGE\x00" as *const u8 as *const libc::c_char),
        6 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Pages\x00" as *const u8 as *const libc::c_char,
            ))
        }
        7 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Names\x00" as *const u8 as *const libc::c_char,
            ))
        }
        5 => value = pdf_ref_obj(pdf_doc_current_page_resources()),
        8 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Catalog\x00" as *const u8 as *const libc::c_char,
            ))
        }
        9 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Info\x00" as *const u8 as *const libc::c_char,
            ))
        }
        _ => {
            if ispageref(key) != 0 {
                value = pdf_doc_ref_page(atoi(key.offset(4)) as libc::c_uint)
            } else {
                value = pdf_names_lookup_reference(
                    named_objects,
                    key as *const libc::c_void,
                    strlen(key) as libc::c_int,
                )
            }
        }
    }
    if value.is_null() {
        _tt_abort(
            b"Object reference %s not exist.\x00" as *const u8 as *const libc::c_char,
            key,
        );
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn spc_lookup_object(mut key: *const libc::c_char) -> *mut pdf_obj {
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut k: libc::c_int = 0;
    if !named_objects.is_null() {
    } else {
        __assert_fail(
            b"named_objects\x00" as *const u8 as *const libc::c_char,
            b"dpx-specials.c\x00" as *const u8 as *const libc::c_char,
            227i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"pdf_obj *spc_lookup_object(const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if key.is_null() {
        return 0 as *mut pdf_obj;
    }
    k = 0i32;
    while !_rkeys[k as usize].is_null() && strcmp(key, _rkeys[k as usize]) != 0 {
        k += 1
    }
    match k {
        0 => {
            cp.x = dvi_dev_xpos();
            cp.y = 0.0f64;
            pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
            value = pdf_new_number(floor(cp.x / 0.01f64 + 0.5f64) * 0.01f64)
        }
        1 => {
            cp.x = 0.0f64;
            cp.y = dvi_dev_ypos();
            pdf_dev_transform(&mut cp, 0 as *const pdf_tmatrix);
            value = pdf_new_number(floor(cp.y / 0.01f64 + 0.5f64) * 0.01f64)
        }
        2 => value = pdf_doc_get_dictionary(b"@THISPAGE\x00" as *const u8 as *const libc::c_char),
        6 => value = pdf_doc_get_dictionary(b"Pages\x00" as *const u8 as *const libc::c_char),
        7 => value = pdf_doc_get_dictionary(b"Names\x00" as *const u8 as *const libc::c_char),
        5 => value = pdf_doc_current_page_resources(),
        8 => value = pdf_doc_get_dictionary(b"Catalog\x00" as *const u8 as *const libc::c_char),
        9 => value = pdf_doc_get_dictionary(b"Info\x00" as *const u8 as *const libc::c_char),
        _ => {
            value = pdf_names_lookup_object(
                named_objects,
                key as *const libc::c_void,
                strlen(key) as libc::c_int,
            )
        }
    }
    /* spc_handler_pdfm_bead() in spc_pdfm.c controls NULL too.
      if (!value) {
        _tt_abort("Object reference %s not exist.", key);
      }
    */
    return value; /* _FIXME_ */
}
#[no_mangle]
pub unsafe extern "C" fn spc_push_object(mut key: *const libc::c_char, mut value: *mut pdf_obj) {
    if !named_objects.is_null() {
    } else {
        __assert_fail(
            b"named_objects\x00" as *const u8 as *const libc::c_char,
            b"dpx-specials.c\x00" as *const u8 as *const libc::c_char,
            279i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void spc_push_object(const char *, pdf_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    if key.is_null() || value.is_null() {
        return;
    }
    pdf_names_add_object(
        named_objects,
        key as *const libc::c_void,
        strlen(key) as libc::c_int,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spc_flush_object(mut key: *const libc::c_char) {
    pdf_names_close_object(
        named_objects,
        key as *const libc::c_void,
        strlen(key) as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spc_clear_objects() {
    pdf_delete_name_tree(&mut named_objects);
    named_objects = pdf_new_name_tree();
}
unsafe extern "C" fn spc_handler_unknown(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    if !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(
            b"spe && args\x00" as *const u8 as *const libc::c_char,
            b"dpx-specials.c\x00" as *const u8 as *const libc::c_char,
            305i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                b"int spc_handler_unknown(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*args).curptr = (*args).endptr;
    return -1i32;
}
unsafe extern "C" fn init_special(
    mut special: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    mut p: *const libc::c_char,
    mut size: u32,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
    mut mag: libc::c_double,
) {
    (*special).key = 0 as *const libc::c_char;
    (*special).exec = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int>,
        spc_handler_fn_ptr,
    >(Some(
        spc_handler_unknown
            as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
    ));
    (*spe).x_user = x_user;
    (*spe).y_user = y_user;
    (*spe).mag = mag;
    (*spe).pg = pdf_doc_current_page_number();
    (*args).curptr = p;
    (*args).endptr = (*args).curptr.offset(size as isize);
    (*args).base = (*args).curptr;
    (*args).command = 0 as *const libc::c_char;
}
unsafe extern "C" fn check_garbage(mut args: *mut spc_arg) {
    if !args.is_null() {
    } else {
        __assert_fail(
            b"args\x00" as *const u8 as *const libc::c_char,
            b"dpx-specials.c\x00" as *const u8 as *const libc::c_char,
            339i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"void check_garbage(struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*args).curptr >= (*args).endptr {
        return;
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        dpx_warning(
            b"Unparsed material at end of special ignored.\x00" as *const u8 as *const libc::c_char,
        );
        dump((*args).curptr, (*args).endptr);
    };
}
static mut known_specials: [C2RustUnnamed_0; 9] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_0 {
                key: b"pdf:\x00" as *const u8 as *const libc::c_char,
                bodhk_func: Some(
                    spc_pdfm_at_begin_document as unsafe extern "C" fn() -> libc::c_int,
                ),
                eodhk_func: Some(spc_pdfm_at_end_document as unsafe extern "C" fn() -> libc::c_int),
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_pdfm_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_pdfm_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"x:\x00" as *const u8 as *const libc::c_char,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_xtx_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_xtx_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"dvipdfmx:\x00" as *const u8 as *const libc::c_char,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_dvipdfmx_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_dvipdfmx_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"ps:\x00" as *const u8 as *const libc::c_char,
                bodhk_func: Some(
                    spc_dvips_at_begin_document as unsafe extern "C" fn() -> libc::c_int,
                ),
                eodhk_func: Some(
                    spc_dvips_at_end_document as unsafe extern "C" fn() -> libc::c_int,
                ),
                bophk_func: Some(spc_dvips_at_begin_page as unsafe extern "C" fn() -> libc::c_int),
                eophk_func: Some(spc_dvips_at_end_page as unsafe extern "C" fn() -> libc::c_int),
                check_func: Some(
                    spc_dvips_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_dvips_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"color\x00" as *const u8 as *const libc::c_char,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_color_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_color_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"tpic\x00" as *const u8 as *const libc::c_char,
                bodhk_func: Some(
                    spc_tpic_at_begin_document as unsafe extern "C" fn() -> libc::c_int,
                ),
                eodhk_func: Some(spc_tpic_at_end_document as unsafe extern "C" fn() -> libc::c_int),
                bophk_func: Some(spc_tpic_at_begin_page as unsafe extern "C" fn() -> libc::c_int),
                eophk_func: Some(spc_tpic_at_end_page as unsafe extern "C" fn() -> libc::c_int),
                check_func: Some(
                    spc_tpic_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_tpic_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"html:\x00" as *const u8 as *const libc::c_char,
                bodhk_func: Some(
                    spc_html_at_begin_document as unsafe extern "C" fn() -> libc::c_int,
                ),
                eodhk_func: Some(spc_html_at_end_document as unsafe extern "C" fn() -> libc::c_int),
                bophk_func: Some(spc_html_at_begin_page as unsafe extern "C" fn() -> libc::c_int),
                eophk_func: Some(spc_html_at_end_page as unsafe extern "C" fn() -> libc::c_int),
                check_func: Some(
                    spc_html_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_html_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"unknown\x00" as *const u8 as *const libc::c_char,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_misc_check_special
                        as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> bool,
                ),
                setup_func: Some(
                    spc_misc_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: 0 as *const libc::c_char,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: None,
                setup_func: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_exec_at_begin_page() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while !known_specials[i as usize].key.is_null() {
        if known_specials[i as usize].bophk_func.is_some() {
            error = known_specials[i as usize]
                .bophk_func
                .expect("non-null function pointer")()
        }
        i = i.wrapping_add(1)
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn spc_exec_at_end_page() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while !known_specials[i as usize].key.is_null() {
        if known_specials[i as usize].eophk_func.is_some() {
            error = known_specials[i as usize]
                .eophk_func
                .expect("non-null function pointer")()
        }
        i = i.wrapping_add(1)
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn spc_exec_at_begin_document() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut i: libc::c_uint = 0;
    if named_objects.is_null() {
    } else {
        __assert_fail(
            b"!named_objects\x00" as *const u8 as *const libc::c_char,
            b"dpx-specials.c\x00" as *const u8 as *const libc::c_char,
            474i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int spc_exec_at_begin_document(void)\x00",
            ))
            .as_ptr(),
        );
    }
    named_objects = pdf_new_name_tree();
    i = 0i32 as libc::c_uint;
    while !known_specials[i as usize].key.is_null() {
        if known_specials[i as usize].bodhk_func.is_some() {
            error = known_specials[i as usize]
                .bodhk_func
                .expect("non-null function pointer")()
        }
        i = i.wrapping_add(1)
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn spc_exec_at_end_document() -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while !known_specials[i as usize].key.is_null() {
        if known_specials[i as usize].eodhk_func.is_some() {
            error = known_specials[i as usize]
                .eodhk_func
                .expect("non-null function pointer")()
        }
        i = i.wrapping_add(1)
    }
    if !named_objects.is_null() {
        pdf_delete_name_tree(&mut named_objects);
    }
    return error;
}
unsafe extern "C" fn print_error(
    mut name: *const libc::c_char,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ebuf: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut pg: libc::c_int = (*spe).pg;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = (*spe).x_user;
    c.y = (*spe).y_user;
    pdf_dev_transform(&mut c, 0 as *const pdf_tmatrix);
    if !(*ap).command.is_null() && !name.is_null() {
        dpx_warning(
            b"Interpreting special command %s (%s) failed.\x00" as *const u8 as *const libc::c_char,
            (*ap).command,
            name,
        );
        dpx_warning(
            b">> at page=\"%d\" position=\"(%g, %g)\" (in PDF)\x00" as *const u8
                as *const libc::c_char,
            pg,
            c.x,
            c.y,
        );
    }
    i = 0i32;
    p = (*ap).base;
    while i < 63i32 && p < (*ap).endptr {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let fresh0 = i;
            i = i + 1;
            ebuf[fresh0 as usize] = *p
        } else {
            if !(i + 4i32 < 63i32) {
                break;
            }
            i += sprintf(
                ebuf.as_mut_ptr().offset(i as isize),
                b"\\x%02x\x00" as *const u8 as *const libc::c_char,
                *p as libc::c_uchar as libc::c_int,
            )
        }
        p = p.offset(1)
    }
    ebuf[i as usize] = '\u{0}' as i32 as libc::c_char;
    if (*ap).curptr < (*ap).endptr {
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 > 60i32) {
                break;
            }
            ebuf[i as usize] = '.' as i32 as libc::c_char
        }
    }
    dpx_warning(
        b">> xxx \"%s\"\x00" as *const u8 as *const libc::c_char,
        ebuf.as_mut_ptr(),
    );
    if (*ap).curptr < (*ap).endptr {
        i = 0i32;
        p = (*ap).curptr;
        while i < 63i32 && p < (*ap).endptr {
            if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                let fresh2 = i;
                i = i + 1;
                ebuf[fresh2 as usize] = *p
            } else {
                if !(i + 4i32 < 63i32) {
                    break;
                }
                i += sprintf(
                    ebuf.as_mut_ptr().offset(i as isize),
                    b"\\x%02x\x00" as *const u8 as *const libc::c_char,
                    *p as libc::c_uchar as libc::c_int,
                )
            }
            p = p.offset(1)
        }
        ebuf[i as usize] = '\u{0}' as i32 as libc::c_char;
        if (*ap).curptr < (*ap).endptr {
            loop {
                let fresh3 = i;
                i = i - 1;
                if !(fresh3 > 60i32) {
                    break;
                }
                ebuf[i as usize] = '.' as i32 as libc::c_char
            }
        }
        dpx_warning(
            b">> Reading special command stopped around >>%s<<\x00" as *const u8
                as *const libc::c_char,
            ebuf.as_mut_ptr(),
        );
        (*ap).curptr = (*ap).endptr
    };
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
/* current page in PDF */
/* This should not use pdf_. */
/* PDF parser shouldn't depend on this...
 */
#[no_mangle]
pub unsafe extern "C" fn spc_exec_special(
    mut buffer: *const libc::c_char,
    mut size: int32_t,
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
    mut mag: libc::c_double,
) -> libc::c_int {
    let mut error: libc::c_int = -1i32;
    let mut i: libc::c_int = 0;
    let mut found: bool = false;
    let mut spe: spc_env = spc_env {
        x_user: 0.,
        y_user: 0.,
        mag: 0.,
        pg: 0,
    };
    let mut args: spc_arg = spc_arg {
        curptr: 0 as *const libc::c_char,
        endptr: 0 as *const libc::c_char,
        base: 0 as *const libc::c_char,
        command: 0 as *const libc::c_char,
    };
    let mut special: spc_handler = spc_handler {
        key: 0 as *const libc::c_char,
        exec: None,
    };
    if verbose > 3i32 {
        dump(buffer, buffer.offset(size as isize));
    }
    init_special(
        &mut special,
        &mut spe,
        &mut args,
        buffer,
        size as u32,
        x_user,
        y_user,
        mag,
    );
    i = 0i32;
    while !known_specials[i as usize].key.is_null() {
        found = known_specials[i as usize]
            .check_func
            .expect("non-null function pointer")(buffer, size);
        if found {
            error = known_specials[i as usize]
                .setup_func
                .expect("non-null function pointer")(
                &mut special, &mut spe, &mut args
            );
            if error == 0 {
                error = special.exec.expect("non-null function pointer")(&mut spe, &mut args)
            }
            if error != 0 {
                print_error(known_specials[i as usize].key, &mut spe, &mut args);
            }
            break;
        } else {
            i += 1
        }
    }
    check_garbage(&mut args);
    return error;
}
