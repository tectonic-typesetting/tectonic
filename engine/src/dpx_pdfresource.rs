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
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_res {
    pub ident: *mut libc::c_char,
    pub flags: libc::c_int,
    pub category: libc::c_int,
    pub cdata: *mut libc::c_void,
    pub object: *mut pdf_obj,
    pub reference: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct res_cache {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub resources: *mut pdf_res,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *const libc::c_char,
    pub cat_id: libc::c_int,
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
static mut pdf_resource_categories: [C2RustUnnamed; 9] = [
    {
        let mut init = C2RustUnnamed {
            name: b"Font\x00" as *const u8 as *const libc::c_char,
            cat_id: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"CIDFont\x00" as *const u8 as *const libc::c_char,
            cat_id: 1i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"Encoding\x00" as *const u8 as *const libc::c_char,
            cat_id: 2i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"CMap\x00" as *const u8 as *const libc::c_char,
            cat_id: 3i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"XObject\x00" as *const u8 as *const libc::c_char,
            cat_id: 4i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"ColorSpace\x00" as *const u8 as *const libc::c_char,
            cat_id: 5i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"Shading\x00" as *const u8 as *const libc::c_char,
            cat_id: 6i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"Pattern\x00" as *const u8 as *const libc::c_char,
            cat_id: 7i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            name: b"ExtGState\x00" as *const u8 as *const libc::c_char,
            cat_id: 8i32,
        };
        init
    },
];
static mut resources: [res_cache; 9] = [res_cache {
    count: 0,
    capacity: 0,
    resources: 0 as *const pdf_res as *mut pdf_res,
}; 9];
unsafe extern "C" fn pdf_init_resource(mut res: *mut pdf_res) {
    if !res.is_null() {
    } else {
        __assert_fail(
            b"res\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfresource.c\x00" as *const u8 as *const libc::c_char,
            94i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void pdf_init_resource(pdf_res *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*res).ident = 0 as *mut libc::c_char;
    (*res).category = -1i32;
    (*res).flags = 0i32;
    (*res).cdata = 0 as *mut libc::c_void;
    (*res).object = 0 as *mut pdf_obj;
    (*res).reference = 0 as *mut pdf_obj;
}
unsafe extern "C" fn pdf_flush_resource(mut res: *mut pdf_res) {
    if !res.is_null() {
        pdf_release_obj((*res).reference);
        pdf_release_obj((*res).object);
        (*res).reference = 0 as *mut pdf_obj;
        (*res).object = 0 as *mut pdf_obj
    };
}
unsafe extern "C" fn pdf_clean_resource(mut res: *mut pdf_res) {
    if !res.is_null() {
        if !(*res).reference.is_null() || !(*res).object.is_null() {
            dpx_warning(
                b"Trying to release un-flushed object.\x00" as *const u8 as *const libc::c_char,
            );
        }
        pdf_release_obj((*res).reference);
        pdf_release_obj((*res).object);
        (*res).ident = mfree((*res).ident as *mut libc::c_void) as *mut libc::c_char;
        (*res).category = -1i32;
        (*res).flags = 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_resources() {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while (i as u64)
        < (::std::mem::size_of::<[C2RustUnnamed; 9]>() as u64)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as u64)
    {
        resources[i as usize].count = 0i32;
        resources[i as usize].capacity = 0i32;
        resources[i as usize].resources = 0 as *mut pdf_res;
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close_resources() {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while (i as u64)
        < (::std::mem::size_of::<[C2RustUnnamed; 9]>() as u64)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as u64)
    {
        let mut rc: *mut res_cache = 0 as *mut res_cache;
        let mut j: libc::c_int = 0;
        rc = &mut *resources.as_mut_ptr().offset(i as isize) as *mut res_cache;
        j = 0i32;
        while j < (*rc).count {
            pdf_flush_resource(&mut *(*rc).resources.offset(j as isize));
            pdf_clean_resource(&mut *(*rc).resources.offset(j as isize));
            j += 1
        }
        free((*rc).resources as *mut libc::c_void);
        (*rc).count = 0i32;
        (*rc).capacity = 0i32;
        (*rc).resources = 0 as *mut pdf_res;
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn get_category(mut category: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while (i as u64)
        < (::std::mem::size_of::<[C2RustUnnamed; 9]>() as u64)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as u64)
    {
        if streq_ptr(category, pdf_resource_categories[i as usize].name) {
            return pdf_resource_categories[i as usize].cat_id;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_defineresource(
    mut category: *const libc::c_char,
    mut resname: *const libc::c_char,
    mut object: *mut pdf_obj,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut res_id: libc::c_int = 0;
    let mut rc: *mut res_cache = 0 as *mut res_cache;
    let mut cat_id: libc::c_int = 0;
    let mut res: *mut pdf_res = 0 as *mut pdf_res;
    if !category.is_null() && !object.is_null() {
    } else {
        __assert_fail(
            b"category && object\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfresource.c\x00" as *const u8 as *const libc::c_char,
            192i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                b"int pdf_defineresource(const char *, const char *, pdf_obj *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    cat_id = get_category(category);
    if cat_id < 0i32 {
        _tt_abort(
            b"Unknown resource category: %s\x00" as *const u8 as *const libc::c_char,
            category,
        );
    }
    rc = &mut *resources.as_mut_ptr().offset(cat_id as isize) as *mut res_cache;
    if !resname.is_null() {
        res_id = 0i32;
        while res_id < (*rc).count {
            res = &mut *(*rc).resources.offset(res_id as isize) as *mut pdf_res;
            if streq_ptr(resname, (*res).ident) {
                dpx_warning(
                    b"Resource %s (category: %s) already defined...\x00" as *const u8
                        as *const libc::c_char,
                    resname,
                    category,
                );
                pdf_flush_resource(res);
                (*res).flags = flags;
                if flags & 1i32 != 0 {
                    (*res).reference = pdf_ref_obj(object);
                    pdf_release_obj(object);
                } else {
                    (*res).object = object
                }
                return cat_id << 16i32 | res_id;
            }
            res_id += 1
        }
    } else {
        res_id = (*rc).count
    }
    if res_id == (*rc).count {
        if (*rc).count >= (*rc).capacity {
            (*rc).capacity =
                ((*rc).capacity as libc::c_uint).wrapping_add(16u32) as libc::c_int as libc::c_int;
            (*rc).resources = renew(
                (*rc).resources as *mut libc::c_void,
                ((*rc).capacity as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<pdf_res>() as u64)
                    as u32,
            ) as *mut pdf_res
        }
        res = &mut *(*rc).resources.offset(res_id as isize) as *mut pdf_res;
        pdf_init_resource(res);
        if !resname.is_null() && *resname.offset(0) as libc::c_int != '\u{0}' as i32 {
            (*res).ident = new(
                (strlen(resname).wrapping_add(1i32 as u64) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
                    as u32,
            ) as *mut libc::c_char;
            strcpy((*res).ident, resname);
        }
        (*res).category = cat_id;
        (*res).flags = flags;
        if flags & 1i32 != 0 {
            (*res).reference = pdf_ref_obj(object);
            pdf_release_obj(object);
        } else {
            (*res).object = object
        }
        (*rc).count += 1
    }
    return cat_id << 16i32 | res_id;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_findresource(
    mut category: *const libc::c_char,
    mut resname: *const libc::c_char,
) -> libc::c_int {
    let mut res: *mut pdf_res = 0 as *mut pdf_res;
    let mut res_id: libc::c_int = 0;
    let mut cat_id: libc::c_int = 0;
    let mut rc: *mut res_cache = 0 as *mut res_cache;
    if !resname.is_null() && !category.is_null() {
    } else {
        __assert_fail(
            b"resname && category\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfresource.c\x00" as *const u8 as *const libc::c_char,
            254i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"int pdf_findresource(const char *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    cat_id = get_category(category);
    if cat_id < 0i32 {
        _tt_abort(
            b"Unknown resource category: %s\x00" as *const u8 as *const libc::c_char,
            category,
        );
    }
    rc = &mut *resources.as_mut_ptr().offset(cat_id as isize) as *mut res_cache;
    res_id = 0i32;
    while res_id < (*rc).count {
        res = &mut *(*rc).resources.offset(res_id as isize) as *mut pdf_res;
        if streq_ptr(resname, (*res).ident) {
            return cat_id << 16i32 | res_id;
        }
        res_id += 1
    }
    return -1i32;
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
pub unsafe extern "C" fn pdf_get_resource_reference(mut rc_id: libc::c_int) -> *mut pdf_obj {
    let mut cat_id: libc::c_int = 0;
    let mut res_id: libc::c_int = 0;
    let mut rc: *mut res_cache = 0 as *mut res_cache;
    let mut res: *mut pdf_res = 0 as *mut pdf_res;
    cat_id = rc_id >> 16i32 & 0xffffi32;
    res_id = rc_id & 0xffffi32;
    if cat_id < 0i32
        || cat_id as u64
            >= (::std::mem::size_of::<[C2RustUnnamed; 9]>() as u64)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as u64)
    {
        _tt_abort(
            b"Invalid category ID: %d\x00" as *const u8 as *const libc::c_char,
            cat_id,
        );
    }
    rc = &mut *resources.as_mut_ptr().offset(cat_id as isize) as *mut res_cache;
    if res_id < 0i32 || res_id >= (*rc).count {
        _tt_abort(
            b"Invalid resource ID: %d\x00" as *const u8 as *const libc::c_char,
            res_id,
        );
    }
    res = &mut *(*rc).resources.offset(res_id as isize) as *mut pdf_res;
    if (*res).reference.is_null() {
        if (*res).object.is_null() {
            _tt_abort(b"Undefined object...\x00" as *const u8 as *const libc::c_char);
        } else {
            (*res).reference = pdf_ref_obj((*res).object)
        }
    }
    return pdf_link_obj((*res).reference);
}
