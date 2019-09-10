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
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn floor(_: f64) -> f64;
    #[no_mangle]
    fn pdf_new_number(value: f64) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn vsprintf(_: *mut i8, _: *const i8, _: ::std::ffi::VaList) -> i32;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn atoi(__nptr: *const i8) -> i32;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
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
    fn dvi_dev_xpos() -> f64;
    #[no_mangle]
    fn dvi_dev_ypos() -> f64;
    #[no_mangle]
    fn dvi_untag_depth();
    #[no_mangle]
    fn dvi_tag_depth();
    #[no_mangle]
    fn dvi_link_annot(flag: i32);
    /* They just return PDF dictionary object.
     * Callers are completely responsible for doing right thing...
     */
    #[no_mangle]
    fn pdf_doc_get_dictionary(category: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_get_reference(category: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_current_page_number() -> i32;
    #[no_mangle]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_ref_page(page_no: u32) -> *mut pdf_obj;
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
        keylen: i32,
        object: *mut pdf_obj,
    ) -> i32;
    #[no_mangle]
    fn pdf_names_lookup_reference(
        names: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_names_lookup_object(
        names: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_names_close_object(names: *mut ht_table, key: *const libc::c_void, keylen: i32) -> i32;
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
    fn spc_color_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_color_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
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
    fn spc_dvipdfmx_check_special(buf: *const i8, len: i32) -> bool;
    #[no_mangle]
    fn spc_dvipdfmx_setup_handler(
        sph: *mut spc_handler,
        spe: *mut spc_env,
        ap: *mut spc_arg,
    ) -> i32;
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
    fn spc_dvips_at_begin_document() -> i32;
    #[no_mangle]
    fn spc_dvips_at_end_document() -> i32;
    #[no_mangle]
    fn spc_dvips_at_begin_page() -> i32;
    #[no_mangle]
    fn spc_dvips_at_end_page() -> i32;
    #[no_mangle]
    fn spc_dvips_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_dvips_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
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
    fn spc_html_at_begin_page() -> i32;
    #[no_mangle]
    fn spc_html_at_end_page() -> i32;
    #[no_mangle]
    fn spc_html_at_begin_document() -> i32;
    #[no_mangle]
    fn spc_html_at_end_document() -> i32;
    #[no_mangle]
    fn spc_html_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_html_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
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
    fn spc_misc_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_misc_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
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
    fn spc_pdfm_at_begin_document() -> i32;
    #[no_mangle]
    fn spc_pdfm_at_end_document() -> i32;
    #[no_mangle]
    fn spc_pdfm_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_pdfm_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
    #[no_mangle]
    fn spc_tpic_at_begin_page() -> i32;
    #[no_mangle]
    fn spc_tpic_at_end_page() -> i32;
    #[no_mangle]
    fn spc_tpic_at_begin_document() -> i32;
    #[no_mangle]
    fn spc_tpic_at_end_document() -> i32;
    #[no_mangle]
    fn spc_tpic_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_tpic_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
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
    fn spc_xtx_check_special(buffer: *const i8, size: i32) -> bool;
    #[no_mangle]
    fn spc_xtx_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type C2RustUnnamed = u32;
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
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: f64,
    pub y_user: f64,
    pub mag: f64,
    pub pg: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const i8,
    pub endptr: *const i8,
    pub base: *const i8,
    pub command: *const i8,
}
pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const i8,
    pub exec: spc_handler_fn_ptr,
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
pub struct pdf_coord {
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub key: *const i8,
    pub bodhk_func: Option<unsafe extern "C" fn() -> i32>,
    pub eodhk_func: Option<unsafe extern "C" fn() -> i32>,
    pub bophk_func: Option<unsafe extern "C" fn() -> i32>,
    pub eophk_func: Option<unsafe extern "C" fn() -> i32>,
    pub check_func: Option<unsafe extern "C" fn(_: *const i8, _: i32) -> bool>,
    pub setup_func:
        Option<unsafe extern "C" fn(_: *mut spc_handler, _: *mut spc_env, _: *mut spc_arg) -> i32>,
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
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn spc_set_verbose(mut level: i32) {
    verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn spc_warn(mut spe: *mut spc_env, mut fmt: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    static mut buf: [i8; 1024] = [0; 1024];
    ap = args.clone();
    vsprintf(buf.as_mut_ptr(), fmt, ap.as_va_list());
    dpx_warning(b"%s\x00" as *const u8 as *const i8, buf.as_mut_ptr());
}
/* This is currently just to make other spc_xxx to not directly
 * call dvi_xxx.
 */
#[no_mangle]
pub unsafe extern "C" fn spc_begin_annot(mut spe: *mut spc_env, mut dict: *mut pdf_obj) -> i32 {
    pdf_doc_begin_annot(dict); /* Tell dvi interpreter to handle line-break. */
    dvi_tag_depth();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_end_annot(mut spe: *mut spc_env) -> i32 {
    dvi_untag_depth();
    pdf_doc_end_annot();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_resume_annot(mut spe: *mut spc_env) -> i32 {
    dvi_link_annot(1i32);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_suspend_annot(mut spe: *mut spc_env) -> i32 {
    dvi_link_annot(0i32);
    return 0i32;
}
static mut named_objects: *mut ht_table = 0 as *const ht_table as *mut ht_table;
/* reserved keys */
static mut _rkeys: [*const i8; 11] = [
    b"xpos\x00" as *const u8 as *const i8,
    b"ypos\x00" as *const u8 as *const i8,
    b"thispage\x00" as *const u8 as *const i8,
    b"prevpage\x00" as *const u8 as *const i8,
    b"nextpage\x00" as *const u8 as *const i8,
    b"resources\x00" as *const u8 as *const i8,
    b"pages\x00" as *const u8 as *const i8,
    b"names\x00" as *const u8 as *const i8,
    b"catalog\x00" as *const u8 as *const i8,
    b"docinfo\x00" as *const u8 as *const i8,
    0 as *const i8,
];
/* pageN where N is a positive integer.
 * Note that page need not exist at this time.
 */
unsafe extern "C" fn ispageref(mut key: *const i8) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    if strlen(key) <= strlen(b"page\x00" as *const u8 as *const i8)
        || memcmp(
            key as *const libc::c_void,
            b"page\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"page\x00" as *const u8 as *const i8),
        ) != 0
    {
        return 0i32;
    } else {
        p = key.offset(4);
        while *p as i32 != 0 && *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
            p = p.offset(1)
        }
        if *p as i32 != '\u{0}' as i32 {
            return 0i32;
        }
    }
    return 1i32;
}
/*
 * The following routine returns copies, not the original object.
 */
#[no_mangle]
pub unsafe extern "C" fn spc_lookup_reference(mut key: *const i8) -> *mut pdf_obj {
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut k: i32 = 0;
    if !named_objects.is_null() {
    } else {
        __assert_fail(
            b"named_objects\x00" as *const u8 as *const i8,
            b"dpx-specials.c\x00" as *const u8 as *const i8,
            162_u32,
            (*::std::mem::transmute::<&[u8; 44], &[i8; 44]>(
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
        2 => value = pdf_doc_get_reference(b"@THISPAGE\x00" as *const u8 as *const i8),
        3 => value = pdf_doc_get_reference(b"@PREVPAGE\x00" as *const u8 as *const i8),
        4 => value = pdf_doc_get_reference(b"@NEXTPAGE\x00" as *const u8 as *const i8),
        6 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Pages\x00" as *const u8 as *const i8,
            ))
        }
        7 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Names\x00" as *const u8 as *const i8,
            ))
        }
        5 => value = pdf_ref_obj(pdf_doc_current_page_resources()),
        8 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Catalog\x00" as *const u8 as *const i8,
            ))
        }
        9 => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Info\x00" as *const u8 as *const i8,
            ))
        }
        _ => {
            if ispageref(key) != 0 {
                value = pdf_doc_ref_page(atoi(key.offset(4)) as u32)
            } else {
                value = pdf_names_lookup_reference(
                    named_objects,
                    key as *const libc::c_void,
                    strlen(key) as i32,
                )
            }
        }
    }
    if value.is_null() {
        _tt_abort(
            b"Object reference %s not exist.\x00" as *const u8 as *const i8,
            key,
        );
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn spc_lookup_object(mut key: *const i8) -> *mut pdf_obj {
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut k: i32 = 0;
    if !named_objects.is_null() {
    } else {
        __assert_fail(
            b"named_objects\x00" as *const u8 as *const i8,
            b"dpx-specials.c\x00" as *const u8 as *const i8,
            227_u32,
            (*::std::mem::transmute::<&[u8; 41], &[i8; 41]>(
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
        2 => value = pdf_doc_get_dictionary(b"@THISPAGE\x00" as *const u8 as *const i8),
        6 => value = pdf_doc_get_dictionary(b"Pages\x00" as *const u8 as *const i8),
        7 => value = pdf_doc_get_dictionary(b"Names\x00" as *const u8 as *const i8),
        5 => value = pdf_doc_current_page_resources(),
        8 => value = pdf_doc_get_dictionary(b"Catalog\x00" as *const u8 as *const i8),
        9 => value = pdf_doc_get_dictionary(b"Info\x00" as *const u8 as *const i8),
        _ => {
            value = pdf_names_lookup_object(
                named_objects,
                key as *const libc::c_void,
                strlen(key) as i32,
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
pub unsafe extern "C" fn spc_push_object(mut key: *const i8, mut value: *mut pdf_obj) {
    if !named_objects.is_null() {
    } else {
        __assert_fail(
            b"named_objects\x00" as *const u8 as *const i8,
            b"dpx-specials.c\x00" as *const u8 as *const i8,
            279_u32,
            (*::std::mem::transmute::<&[u8; 46], &[i8; 46]>(
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
        strlen(key) as i32,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spc_flush_object(mut key: *const i8) {
    pdf_names_close_object(
        named_objects,
        key as *const libc::c_void,
        strlen(key) as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spc_clear_objects() {
    pdf_delete_name_tree(&mut named_objects);
    named_objects = pdf_new_name_tree();
}
unsafe extern "C" fn spc_handler_unknown(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    if !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(
            b"spe && args\x00" as *const u8 as *const i8,
            b"dpx-specials.c\x00" as *const u8 as *const i8,
            305_u32,
            (*::std::mem::transmute::<&[u8; 60], &[i8; 60]>(
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
    mut p: *const i8,
    mut size: u32,
    mut x_user: f64,
    mut y_user: f64,
    mut mag: f64,
) {
    (*special).key = 0 as *const i8;
    (*special).exec = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>,
        spc_handler_fn_ptr,
    >(Some(
        spc_handler_unknown as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
    ));
    (*spe).x_user = x_user;
    (*spe).y_user = y_user;
    (*spe).mag = mag;
    (*spe).pg = pdf_doc_current_page_number();
    (*args).curptr = p;
    (*args).endptr = (*args).curptr.offset(size as isize);
    (*args).base = (*args).curptr;
    (*args).command = 0 as *const i8;
}
unsafe extern "C" fn check_garbage(mut args: *mut spc_arg) {
    if !args.is_null() {
    } else {
        __assert_fail(
            b"args\x00" as *const u8 as *const i8,
            b"dpx-specials.c\x00" as *const u8 as *const i8,
            339_u32,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
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
        dpx_warning(b"Unparsed material at end of special ignored.\x00" as *const u8 as *const i8);
        dump((*args).curptr, (*args).endptr);
    };
}
static mut known_specials: [C2RustUnnamed_0; 9] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_0 {
                key: b"pdf:\x00" as *const u8 as *const i8,
                bodhk_func: Some(spc_pdfm_at_begin_document as unsafe extern "C" fn() -> i32),
                eodhk_func: Some(spc_pdfm_at_end_document as unsafe extern "C" fn() -> i32),
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_pdfm_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_pdfm_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"x:\x00" as *const u8 as *const i8,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_xtx_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_xtx_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"dvipdfmx:\x00" as *const u8 as *const i8,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_dvipdfmx_check_special
                        as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_dvipdfmx_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"ps:\x00" as *const u8 as *const i8,
                bodhk_func: Some(spc_dvips_at_begin_document as unsafe extern "C" fn() -> i32),
                eodhk_func: Some(spc_dvips_at_end_document as unsafe extern "C" fn() -> i32),
                bophk_func: Some(spc_dvips_at_begin_page as unsafe extern "C" fn() -> i32),
                eophk_func: Some(spc_dvips_at_end_page as unsafe extern "C" fn() -> i32),
                check_func: Some(
                    spc_dvips_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_dvips_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"color\x00" as *const u8 as *const i8,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_color_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_color_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"tpic\x00" as *const u8 as *const i8,
                bodhk_func: Some(spc_tpic_at_begin_document as unsafe extern "C" fn() -> i32),
                eodhk_func: Some(spc_tpic_at_end_document as unsafe extern "C" fn() -> i32),
                bophk_func: Some(spc_tpic_at_begin_page as unsafe extern "C" fn() -> i32),
                eophk_func: Some(spc_tpic_at_end_page as unsafe extern "C" fn() -> i32),
                check_func: Some(
                    spc_tpic_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_tpic_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"html:\x00" as *const u8 as *const i8,
                bodhk_func: Some(spc_html_at_begin_document as unsafe extern "C" fn() -> i32),
                eodhk_func: Some(spc_html_at_end_document as unsafe extern "C" fn() -> i32),
                bophk_func: Some(spc_html_at_begin_page as unsafe extern "C" fn() -> i32),
                eophk_func: Some(spc_html_at_end_page as unsafe extern "C" fn() -> i32),
                check_func: Some(
                    spc_html_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_html_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: b"unknown\x00" as *const u8 as *const i8,
                bodhk_func: None,
                eodhk_func: None,
                bophk_func: None,
                eophk_func: None,
                check_func: Some(
                    spc_misc_check_special as unsafe extern "C" fn(_: *const i8, _: i32) -> bool,
                ),
                setup_func: Some(
                    spc_misc_setup_handler
                        as unsafe extern "C" fn(
                            _: *mut spc_handler,
                            _: *mut spc_env,
                            _: *mut spc_arg,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                key: 0 as *const i8,
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
pub unsafe extern "C" fn spc_exec_at_begin_page() -> i32 {
    let mut error: i32 = 0i32;
    let mut i: u32 = 0;
    i = 0_u32;
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
pub unsafe extern "C" fn spc_exec_at_end_page() -> i32 {
    let mut error: i32 = 0i32;
    let mut i: u32 = 0;
    i = 0_u32;
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
pub unsafe extern "C" fn spc_exec_at_begin_document() -> i32 {
    let mut error: i32 = 0i32;
    let mut i: u32 = 0;
    if named_objects.is_null() {
    } else {
        __assert_fail(
            b"!named_objects\x00" as *const u8 as *const i8,
            b"dpx-specials.c\x00" as *const u8 as *const i8,
            474_u32,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
                b"int spc_exec_at_begin_document(void)\x00",
            ))
            .as_ptr(),
        );
    }
    named_objects = pdf_new_name_tree();
    i = 0_u32;
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
pub unsafe extern "C" fn spc_exec_at_end_document() -> i32 {
    let mut error: i32 = 0i32;
    let mut i: u32 = 0;
    i = 0_u32;
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
unsafe extern "C" fn print_error(mut name: *const i8, mut spe: *mut spc_env, mut ap: *mut spc_arg) {
    let mut p: *const i8 = 0 as *const i8;
    let mut ebuf: [i8; 64] = [0; 64];
    let mut i: i32 = 0;
    let mut pg: i32 = (*spe).pg;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = (*spe).x_user;
    c.y = (*spe).y_user;
    pdf_dev_transform(&mut c, 0 as *const pdf_tmatrix);
    if !(*ap).command.is_null() && !name.is_null() {
        dpx_warning(
            b"Interpreting special command %s (%s) failed.\x00" as *const u8 as *const i8,
            (*ap).command,
            name,
        );
        dpx_warning(
            b">> at page=\"%d\" position=\"(%g, %g)\" (in PDF)\x00" as *const u8 as *const i8,
            pg,
            c.x,
            c.y,
        );
    }
    i = 0i32;
    p = (*ap).base;
    while i < 63i32 && p < (*ap).endptr {
        if *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
            & _ISprint as i32 as u16 as i32
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
                b"\\x%02x\x00" as *const u8 as *const i8,
                *p as u8 as i32,
            )
        }
        p = p.offset(1)
    }
    ebuf[i as usize] = '\u{0}' as i32 as i8;
    if (*ap).curptr < (*ap).endptr {
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 > 60i32) {
                break;
            }
            ebuf[i as usize] = '.' as i32 as i8
        }
    }
    dpx_warning(
        b">> xxx \"%s\"\x00" as *const u8 as *const i8,
        ebuf.as_mut_ptr(),
    );
    if (*ap).curptr < (*ap).endptr {
        i = 0i32;
        p = (*ap).curptr;
        while i < 63i32 && p < (*ap).endptr {
            if *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                & _ISprint as i32 as u16 as i32
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
                    b"\\x%02x\x00" as *const u8 as *const i8,
                    *p as u8 as i32,
                )
            }
            p = p.offset(1)
        }
        ebuf[i as usize] = '\u{0}' as i32 as i8;
        if (*ap).curptr < (*ap).endptr {
            loop {
                let fresh3 = i;
                i = i - 1;
                if !(fresh3 > 60i32) {
                    break;
                }
                ebuf[i as usize] = '.' as i32 as i8
            }
        }
        dpx_warning(
            b">> Reading special command stopped around >>%s<<\x00" as *const u8 as *const i8,
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
    mut buffer: *const i8,
    mut size: i32,
    mut x_user: f64,
    mut y_user: f64,
    mut mag: f64,
) -> i32 {
    let mut error: i32 = -1i32;
    let mut i: i32 = 0;
    let mut found: bool = false;
    let mut spe: spc_env = spc_env {
        x_user: 0.,
        y_user: 0.,
        mag: 0.,
        pg: 0,
    };
    let mut args: spc_arg = spc_arg {
        curptr: 0 as *const i8,
        endptr: 0 as *const i8,
        base: 0 as *const i8,
        command: 0 as *const i8,
    };
    let mut special: spc_handler = spc_handler {
        key: 0 as *const i8,
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
