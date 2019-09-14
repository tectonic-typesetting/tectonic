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

use super::dpx_pdfdraw::{pdf_dev_concat, pdf_dev_transform};
use super::dpx_pdfximage::{
    pdf_ximage_findresource, pdf_ximage_get_reference, pdf_ximage_get_resname,
    pdf_ximage_scale_image,
};

use super::dpx_pdfdev::{pdf_rect, pdf_tmatrix, transform_info, transform_info_clear};
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_link_obj, pdf_lookup_dict, pdf_new_array, pdf_new_boolean,
    pdf_new_dict, pdf_new_name, pdf_new_null, pdf_new_number, pdf_new_string, pdf_obj,
    pdf_obj_typeof, pdf_ref_obj, pdf_release_obj, pdf_string_value,
};
use crate::mfree;
use crate::streq_ptr;
use libc::free;
extern "C" {
    #[no_mangle]
    fn tan(_: f64) -> f64;
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    /* Name does not include the / */
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn spc_begin_annot(spe: *mut spc_env, annot_dict: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn spc_end_annot(spe: *mut spc_env) -> i32;
    #[no_mangle]
    fn parse_float_decimal(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    fn pdf_doc_get_reference(category: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    /* Not really managing tree...
     * There should be something for number tree.
     */
    #[no_mangle]
    fn pdf_doc_add_names(
        category: *const i8,
        key: *const libc::c_void,
        keylen: i32,
        value: *mut pdf_obj,
    ) -> i32;
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const i8, length: u32);
    #[no_mangle]
    fn pdf_doc_add_page_resource(
        category: *const i8,
        resource_name: *const i8,
        resources: *mut pdf_obj,
    );
    #[no_mangle]
    fn pdf_dev_rectclip(x: f64, y: f64, w: f64, h: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_gsave() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore() -> i32;
}
pub type size_t = u64;

use super::dpx_specials::{spc_arg, spc_env};

pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
use super::dpx_specials::spc_handler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_html_ {
    pub opts: C2RustUnnamed_0,
    pub link_dict: *mut pdf_obj,
    pub baseurl: *mut i8,
    pub pending_type: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub extensions: i32,
}

use crate::dpx_pdfximage::load_options;

use super::dpx_pdfdev::pdf_coord;

/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
static mut _html_state: spc_html_ = {
    let mut init = spc_html_ {
        opts: {
            let mut init = C2RustUnnamed_0 { extensions: 0i32 };
            init
        },
        link_dict: 0 as *const pdf_obj as *mut pdf_obj,
        baseurl: 0 as *const i8 as *mut i8,
        pending_type: -1i32,
    };
    init
};
/* ENABLE_HTML_SVG_TRANSFORM */
unsafe extern "C" fn parse_key_val(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut kp: *mut *mut i8,
    mut vp: *mut *mut i8,
) -> i32 {
    let mut q: *const i8 = 0 as *const i8; /* skip '="' */
                                           
    let mut p: *const i8 = 0 as *const i8; /* include trailing NULL here!!! */
    let mut k: *mut i8 = 0 as *mut i8; /* we may want to add '/' */
    let mut v: *mut i8 = 0 as *mut i8; /* Should be checked somewhere else */
    let mut n: i32 = 0; /* Assume this is URL */
    let mut error: i32 = 0i32;
    p = *pp;
    while p < endptr && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    v = 0 as *mut i8;
    k = v;
    q = p;
    n = 0i32;
    while p < endptr
        && (*p as i32 >= 'a' as i32 && *p as i32 <= 'z' as i32
            || *p as i32 >= 'A' as i32 && *p as i32 <= 'Z' as i32
            || *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32
            || *p as i32 == '-' as i32
            || *p as i32 == ':' as i32)
    {
        n += 1;
        p = p.offset(1)
    }
    if n == 0i32 {
        *vp = 0 as *mut i8;
        *kp = *vp;
        return -1i32;
    }
    k = new(((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    memcpy(k as *mut libc::c_void, q as *const libc::c_void, n as u64);
    *k.offset(n as isize) = '\u{0}' as i32 as i8;
    if p.offset(2) >= endptr
        || *p.offset(0) as i32 != '=' as i32
        || *p.offset(1) as i32 != '\"' as i32 && *p.offset(1) as i32 != '\'' as i32
    {
        k = mfree(k as *mut libc::c_void) as *mut i8;
        *pp = p;
        error = -1i32
    } else {
        let mut qchr: i8 = *p.offset(1);
        p = p.offset(2);
        q = p;
        n = 0i32;
        while p < endptr && *p as i32 != qchr as i32 {
            p = p.offset(1);
            n += 1
        }
        if p == endptr || *p as i32 != qchr as i32 {
            error = -1i32
        } else {
            v = new(
                ((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
            ) as *mut i8;
            memcpy(v as *mut libc::c_void, q as *const libc::c_void, n as u64);
            *v.offset(n as isize) = '\u{0}' as i32 as i8;
            p = p.offset(1)
        }
    }
    *kp = k;
    *vp = v;
    *pp = p;
    error
}
unsafe extern "C" fn read_html_tag(
    mut name: *mut i8,
    mut attr: *mut pdf_obj,
    mut type_0: *mut i32,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut p: *const i8 = *pp;
    let mut n: i32 = 0i32;
    let mut error: i32 = 0i32;
    while p < endptr && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    if p >= endptr || *p as i32 != '<' as i32 {
        return -1i32;
    }
    *type_0 = 1i32;
    p = p.offset(1);
    while p < endptr && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    if p < endptr && *p as i32 == '/' as i32 {
        *type_0 = 2i32;
        p = p.offset(1);
        while p < endptr && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
    }
    n = 0i32;
    while p < endptr
        && n < 127i32
        && !(*p as i32 == '>' as i32 || *p as i32 == '/' as i32 || libc::isspace(*p as _) != 0)
    {
        *name.offset(n as isize) = *p;
        n += 1;
        p = p.offset(1)
    }
    *name.offset(n as isize) = '\u{0}' as i32 as i8;
    if n == 0i32
        || p == endptr
        || !(*p as i32 == '>' as i32 || *p as i32 == '/' as i32 || libc::isspace(*p as _) != 0)
    {
        *pp = p;
        return -1i32;
    }
    while p < endptr && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    while p < endptr && error == 0 && *p as i32 != '/' as i32 && *p as i32 != '>' as i32 {
        let mut kp: *mut i8 = 0 as *mut i8;
        let mut vp: *mut i8 = 0 as *mut i8;
        error = parse_key_val(&mut p, endptr, &mut kp, &mut vp);
        if error == 0 {
            if !kp.is_null() {
                let mut _p: *mut i8 = kp;
                while *_p as i32 != 0i32 {
                    if *_p as i32 >= 'A' as i32 && *_p as i32 <= 'Z' as i32 {
                        *_p = (*_p as i32 - 'A' as i32 + 'a' as i32) as i8
                    }
                    _p = _p.offset(1)
                }
            }
            pdf_add_dict(
                attr,
                pdf_new_name(kp),
                pdf_new_string(
                    vp as *const libc::c_void,
                    strlen(vp).wrapping_add(1i32 as u64),
                ),
            );
            free(kp as *mut libc::c_void);
            free(vp as *mut libc::c_void);
        }
        while p < endptr && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
    }
    if error != 0 {
        *pp = p;
        return error;
    }
    if p < endptr && *p as i32 == '/' as i32 {
        *type_0 = 1i32;
        p = p.offset(1);
        while p < endptr && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
    }
    if p == endptr || *p as i32 != '>' as i32 {
        *pp = p;
        return -1i32;
    }
    p = p.offset(1);
    if !name.is_null() {
        let mut _p_0: *mut i8 = name;
        while *_p_0 as i32 != 0i32 {
            if *_p_0 as i32 >= 'A' as i32 && *_p_0 as i32 <= 'Z' as i32 {
                *_p_0 = (*_p_0 as i32 - 'A' as i32 + 'a' as i32) as i8
            }
            _p_0 = _p_0.offset(1)
        }
    }
    *pp = p;
    0i32
}
unsafe extern "C" fn spc_handler_html__init(mut dp: *mut libc::c_void) -> i32 {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    (*sd).link_dict = 0 as *mut pdf_obj;
    (*sd).baseurl = 0 as *mut i8;
    (*sd).pending_type = -1i32;
    0i32
}
unsafe extern "C" fn spc_handler_html__clean(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    free((*sd).baseurl as *mut libc::c_void);
    if (*sd).pending_type >= 0i32 || !(*sd).link_dict.is_null() {
        spc_warn(
            spe,
            b"Unclosed html anchor found.\x00" as *const u8 as *const i8,
        );
    }
    pdf_release_obj((*sd).link_dict);
    (*sd).pending_type = -1i32;
    (*sd).baseurl = 0 as *mut i8;
    (*sd).link_dict = 0 as *mut pdf_obj;
    0i32
}
unsafe extern "C" fn spc_handler_html__bophook(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    if (*sd).pending_type >= 0i32 {
        spc_warn(
            spe,
            b"...html anchor continues from previous page processed...\x00" as *const u8
                as *const i8,
        );
    }
    0i32
}
unsafe extern "C" fn spc_handler_html__eophook(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    if (*sd).pending_type >= 0i32 {
        spc_warn(
            spe,
            b"Unclosed html anchor at end-of-page!\x00" as *const u8 as *const i8,
        );
    }
    0i32
}
unsafe extern "C" fn fqurl(mut baseurl: *const i8, mut name: *const i8) -> *mut i8 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0i32;
    len = strlen(name) as i32;
    if !baseurl.is_null() {
        len = (len as u64).wrapping_add(strlen(baseurl).wrapping_add(1i32 as u64)) as i32 as i32
    }
    q = new(((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    *q = '\u{0}' as i32 as i8;
    if !baseurl.is_null() && *baseurl.offset(0) as i32 != 0 {
        let mut p: *mut i8 = 0 as *mut i8;
        strcpy(q, baseurl);
        p = q.offset(strlen(q) as isize).offset(-1);
        if *p as i32 == '/' as i32 {
            *p = '\u{0}' as i32 as i8
        }
        if *name.offset(0) as i32 != 0 && *name.offset(0) as i32 != '/' as i32 {
            strcat(q, b"/\x00" as *const u8 as *const i8);
        }
    }
    strcat(q, name);
    q
}
unsafe extern "C" fn html_open_link(
    mut spe: *mut spc_env,
    mut name: *const i8,
    mut sd: *mut spc_html_,
) -> i32 {
    let mut color: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut url: *mut i8 = 0 as *mut i8;
    assert!(!name.is_null());
    assert!((*sd).link_dict.is_null());
    (*sd).link_dict = pdf_new_dict();
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Annot\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"Link\x00" as *const u8 as *const i8),
    );
    color = pdf_new_array();
    pdf_add_array(color, pdf_new_number(0.0f64));
    pdf_add_array(color, pdf_new_number(0.0f64));
    pdf_add_array(color, pdf_new_number(1.0f64));
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"C\x00" as *const u8 as *const i8),
        color,
    );
    url = fqurl((*sd).baseurl, name);
    if *url.offset(0) as i32 == '#' as i32 {
        /* url++; causes memory leak in free(url) */
        pdf_add_dict(
            (*sd).link_dict,
            pdf_new_name(b"Dest\x00" as *const u8 as *const i8),
            pdf_new_string(url.offset(1) as *const libc::c_void, strlen(url.offset(1))),
        ); /* Otherwise must be bug */
    } else {
        let mut action: *mut pdf_obj = pdf_new_dict();
        pdf_add_dict(
            action,
            pdf_new_name(b"Type\x00" as *const u8 as *const i8),
            pdf_new_name(b"Action\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"S\x00" as *const u8 as *const i8),
            pdf_new_name(b"URI\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"URI\x00" as *const u8 as *const i8),
            pdf_new_string(url as *const libc::c_void, strlen(url)),
        );
        pdf_add_dict(
            (*sd).link_dict,
            pdf_new_name(b"A\x00" as *const u8 as *const i8),
            pdf_link_obj(action),
        );
        pdf_release_obj(action);
    }
    free(url as *mut libc::c_void);
    spc_begin_annot(spe, (*sd).link_dict);
    (*sd).pending_type = 0i32;
    0i32
}
unsafe extern "C" fn html_open_dest(
    mut spe: *mut spc_env,
    mut name: *const i8,
    mut sd: *mut spc_html_,
) -> i32 {
    let mut error: i32 = 0;
    let mut array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut page_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cp: pdf_coord = pdf_coord::new();
    cp.x = (*spe).x_user;
    cp.y = (*spe).y_user;
    pdf_dev_transform(&mut cp, None);
    page_ref = pdf_doc_get_reference(b"@THISPAGE\x00" as *const u8 as *const i8);
    assert!(!page_ref.is_null());
    array = pdf_new_array();
    pdf_add_array(array, page_ref);
    pdf_add_array(array, pdf_new_name(b"XYZ\x00" as *const u8 as *const i8));
    pdf_add_array(array, pdf_new_null());
    pdf_add_array(array, pdf_new_number(cp.y + 24.0f64));
    pdf_add_array(array, pdf_new_null());
    error = pdf_doc_add_names(
        b"Dests\x00" as *const u8 as *const i8,
        name as *const libc::c_void,
        strlen(name) as i32,
        array,
    );
    if error != 0 {
        spc_warn(
            spe,
            b"Failed to add named destination: %s\x00" as *const u8 as *const i8,
            name,
        );
    }
    (*sd).pending_type = 1i32;
    error
}
unsafe extern "C" fn spc_html__anchor_open(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
    mut sd: *mut spc_html_,
) -> i32 {
    let mut href: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut name: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: i32 = 0i32;
    if (*sd).pending_type >= 0i32 || !(*sd).link_dict.is_null() {
        spc_warn(
            spe,
            b"Nested html anchors found!\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    href = pdf_lookup_dict(attr, b"href\x00" as *const u8 as *const i8);
    name = pdf_lookup_dict(attr, b"name\x00" as *const u8 as *const i8);
    if !href.is_null() && !name.is_null() {
        spc_warn(
            spe,
            b"Sorry, you can\'t have both \"href\" and \"name\" in anchor tag...\x00" as *const u8
                as *const i8,
        );
        error = -1i32
    } else if !href.is_null() {
        error = html_open_link(spe, pdf_string_value(href) as *const i8, sd)
    } else if !name.is_null() {
        /* name */
        error = html_open_dest(spe, pdf_string_value(name) as *const i8, sd)
    } else {
        spc_warn(
            spe,
            b"You should have \"href\" or \"name\" in anchor tag!\x00" as *const u8 as *const i8,
        );
        error = -1i32
    }
    error
}
unsafe extern "C" fn spc_html__anchor_close(mut spe: *mut spc_env, mut sd: *mut spc_html_) -> i32 {
    let mut error: i32 = 0i32;
    match (*sd).pending_type {
        0 => {
            if !(*sd).link_dict.is_null() {
                spc_end_annot(spe);
                pdf_release_obj((*sd).link_dict);
                (*sd).link_dict = 0 as *mut pdf_obj;
                (*sd).pending_type = -1i32
            } else {
                spc_warn(
                    spe,
                    b"Closing html anchor (link) without starting!\x00" as *const u8 as *const i8,
                );
                error = -1i32
            }
        }
        1 => (*sd).pending_type = -1i32,
        _ => {
            spc_warn(
                spe,
                b"No corresponding opening tag for html anchor.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        }
    }
    error
}
unsafe extern "C" fn spc_html__base_empty(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
    mut sd: *mut spc_html_,
) -> i32 {
    let mut href: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut vp: *mut i8 = 0 as *mut i8;
    href = pdf_lookup_dict(attr, b"href\x00" as *const u8 as *const i8);
    if href.is_null() {
        spc_warn(
            spe,
            b"\"href\" not found for \"base\" tag!\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    vp = pdf_string_value(href) as *mut i8;
    if !(*sd).baseurl.is_null() {
        spc_warn(
            spe,
            b"\"baseurl\" changed: \"%s\" --> \"%s\"\x00" as *const u8 as *const i8,
            (*sd).baseurl,
            vp,
        );
        free((*sd).baseurl as *mut libc::c_void);
    }
    (*sd).baseurl = new((strlen(vp).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*sd).baseurl, vp);
    0i32
}
/* This isn't completed.
 * Please think about placement of images.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn atopt(mut a: *const i8) -> f64 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = a;
    let mut v: f64 = 0.;
    let mut u: f64 = 1.0f64;
    let mut _ukeys: [*const i8; 11] = [
        b"pt\x00" as *const u8 as *const i8,
        b"in\x00" as *const u8 as *const i8,
        b"cm\x00" as *const u8 as *const i8,
        b"mm\x00" as *const u8 as *const i8,
        b"bp\x00" as *const u8 as *const i8,
        b"pc\x00" as *const u8 as *const i8,
        b"dd\x00" as *const u8 as *const i8,
        b"cc\x00" as *const u8 as *const i8,
        b"sp\x00" as *const u8 as *const i8,
        b"px\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut k: i32 = 0;
    q = parse_float_decimal(&mut p, p.offset(strlen(p) as isize));
    if q.is_null() {
        dpx_warning(
            b"Invalid length value: %s (%c)\x00" as *const u8 as *const i8,
            a,
            *p as i32,
        );
        return 0.0f64;
    }
    v = atof(q);
    free(q as *mut libc::c_void);
    q = parse_c_ident(&mut p, p.offset(strlen(p) as isize));
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
            9 => u *= 1.0f64,
            _ => {
                dpx_warning(
                    b"Unknown unit of measure: %s\x00" as *const u8 as *const i8,
                    q,
                );
            }
        }
        free(q as *mut libc::c_void);
    }
    v * u
}
/* Replicated from spc_tpic */
unsafe extern "C" fn create_xgstate(mut a: f64, mut f_ais: i32) -> *mut pdf_obj
/* alpha is shape */ {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    dict = pdf_new_dict();
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"ExtGState\x00" as *const u8 as *const i8),
    );
    if f_ais != 0 {
        pdf_add_dict(
            dict,
            pdf_new_name(b"AIS\x00" as *const u8 as *const i8),
            pdf_new_boolean(1_i8),
        );
    }
    pdf_add_dict(
        dict,
        pdf_new_name(b"ca\x00" as *const u8 as *const i8),
        pdf_new_number(a),
    );
    dict
}
unsafe extern "C" fn check_resourcestatus(mut category: *const i8, mut resname: *const i8) -> i32 {
    let mut dict1: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict2: *mut pdf_obj = 0 as *mut pdf_obj;
    dict1 = pdf_doc_current_page_resources();
    if dict1.is_null() {
        return 0i32;
    }
    dict2 = pdf_lookup_dict(dict1, category);
    if !dict2.is_null() && pdf_obj_typeof(dict2) == 6i32 {
        if !pdf_lookup_dict(dict2, resname).is_null() {
            return 1i32;
        }
    }
    0i32
}
/* ENABLE_HTML_SVG_OPACITY */
unsafe extern "C" fn spc_html__img_empty(mut spe: *mut spc_env, mut attr: *mut pdf_obj) -> i32 {
    let mut src: *mut pdf_obj = 0 as *mut pdf_obj; /* meaning fully opaque */
    let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut ti = transform_info::new();
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    let mut id: i32 = 0;
    let mut error: i32 = 0i32;
    let mut alpha: f64 = 1.0f64;
    /* ENABLE_HTML_SVG_OPACITY */
    let mut M1 = pdf_tmatrix::new();
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 1.,
        b: 0.,
        c: 0.,
        d: 1.,
        e: (*spe).x_user,
        f: (*spe).y_user,
    };
    /* ENABLE_HTML_SVG_TRANSFORM */
    spc_warn(
        spe,
        b"html \"img\" tag found (not completed, plese don\'t use!).\x00" as *const u8 as *const i8,
    );
    src = pdf_lookup_dict(attr, b"src\x00" as *const u8 as *const i8);
    if src.is_null() {
        spc_warn(
            spe,
            b"\"src\" attribute not found for \"img\" tag!\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    transform_info_clear(&mut ti);
    obj = pdf_lookup_dict(attr, b"width\x00" as *const u8 as *const i8);
    if !obj.is_null() {
        ti.width = atopt(pdf_string_value(obj) as *const i8);
        ti.flags |= 1i32 << 1i32
    }
    obj = pdf_lookup_dict(attr, b"height\x00" as *const u8 as *const i8);
    if !obj.is_null() {
        ti.height = atopt(pdf_string_value(obj) as *const i8);
        ti.flags |= 1i32 << 2i32
    }
    obj = pdf_lookup_dict(attr, b"svg:opacity\x00" as *const u8 as *const i8);
    if !obj.is_null() {
        alpha = atof(pdf_string_value(obj) as *const i8);
        if alpha < 0.0f64 || alpha > 1.0f64 {
            spc_warn(
                spe,
                b"Invalid opacity value: %s\x00" as *const u8 as *const i8,
                pdf_string_value(obj) as *mut i8,
            );
            alpha = 1.0f64
        }
    }
    /* ENABLE_HTML_SVG_OPCAITY */
    obj = pdf_lookup_dict(attr, b"svg:transform\x00" as *const u8 as *const i8);
    if !obj.is_null() {
        let mut p: *const i8 = pdf_string_value(obj) as *const i8;
        let mut N = pdf_tmatrix::new();
        while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
        while *p as i32 != 0 && error == 0 {
            N.a = 1.;
            N.b = 0.;
            N.c = 0.;
            N.d = 1.;
            N.e = 0.;
            N.f = 0.;
            error = cvt_a_to_tmatrix(&mut N, p, &mut p);
            if error == 0 {
                N.f = -N.f;
                let mut _tmp_a: f64 = 0.;
                let mut _tmp_b: f64 = 0.;
                let mut _tmp_c: f64 = 0.;
                let mut _tmp_d: f64 = 0.;
                _tmp_a = M.a;
                _tmp_b = M.b;
                _tmp_c = M.c;
                _tmp_d = M.d;
                M.a = N.a * _tmp_a + N.b * _tmp_c;
                M.b = N.a * _tmp_b + N.b * _tmp_d;
                M.c = N.c * _tmp_a + N.d * _tmp_c;
                M.d = N.c * _tmp_b + N.d * _tmp_d;
                M.e += N.e * _tmp_a + N.f * _tmp_c;
                M.f += N.e * _tmp_b + N.f * _tmp_d;
                while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
                    p = p.offset(1)
                }
                if *p as i32 == ',' as i32 {
                    p = p.offset(1);
                    while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
                        p = p.offset(1)
                    }
                }
            }
        }
    }
    /* ENABLE_HTML_SVG_TRANSFORM */
    if error != 0 {
        spc_warn(
            spe,
            b"Error in html \"img\" tag attribute.\x00" as *const u8 as *const i8,
        ); /* Not Tps prefix but... */
        return error;
    } /* op: */
    id = pdf_ximage_findresource(pdf_string_value(src) as *const i8, options); /* op: */
    if id < 0i32 {
        spc_warn(
            spe,
            b"Could not find/load image: %s\x00" as *const u8 as *const i8,
            pdf_string_value(src) as *mut i8,
        ); /* op: gs */
        error = -1i32
    } else {
        let mut res_name: *mut i8 = 0 as *mut i8;
        let mut r = pdf_rect::new();
        graphics_mode();
        pdf_dev_gsave();
        let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut a: i32 = (100.0f64 * alpha).round() as i32;
        if a != 0i32 {
            res_name = new((strlen(b"_Tps_a100_\x00" as *const u8 as *const i8)
                .wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
                as *mut i8;
            sprintf(res_name, b"_Tps_a%03d_\x00" as *const u8 as *const i8, a);
            if check_resourcestatus(b"ExtGState\x00" as *const u8 as *const i8, res_name) == 0 {
                dict = create_xgstate((0.01f64 * a as f64 / 0.01f64).round() * 0.01f64, 0i32);
                pdf_doc_add_page_resource(
                    b"ExtGState\x00" as *const u8 as *const i8,
                    res_name,
                    pdf_ref_obj(dict),
                );
                pdf_release_obj(dict);
            }
            pdf_doc_add_page_content(b" /\x00" as *const u8 as *const i8, 2_u32);
            pdf_doc_add_page_content(res_name, strlen(res_name) as u32);
            pdf_doc_add_page_content(b" gs\x00" as *const u8 as *const i8, 3_u32);
            free(res_name as *mut libc::c_void);
        }
        /* ENABLE_HTML_SVG_OPACITY */
        pdf_ximage_scale_image(id, &mut M1, &mut r, &mut ti); /* op: */
        let mut _tmp_a_0: f64 = 0.; /* op: */
        let mut _tmp_b_0: f64 = 0.; /* op: Do */
        let mut _tmp_c_0: f64 = 0.;
        let mut _tmp_d_0: f64 = 0.;
        _tmp_a_0 = M.a;
        _tmp_b_0 = M.b;
        _tmp_c_0 = M.c;
        _tmp_d_0 = M.d;
        M.a = M1.a * _tmp_a_0 + M1.b * _tmp_c_0;
        M.b = M1.a * _tmp_b_0 + M1.b * _tmp_d_0;
        M.c = M1.c * _tmp_a_0 + M1.d * _tmp_c_0;
        M.d = M1.c * _tmp_b_0 + M1.d * _tmp_d_0;
        M.e += M1.e * _tmp_a_0 + M1.f * _tmp_c_0;
        M.f += M1.e * _tmp_b_0 + M1.f * _tmp_d_0;
        pdf_dev_concat(&mut M);
        pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
        res_name = pdf_ximage_get_resname(id);
        pdf_doc_add_page_content(b" /\x00" as *const u8 as *const i8, 2_u32);
        pdf_doc_add_page_content(res_name, strlen(res_name) as u32);
        pdf_doc_add_page_content(b" Do\x00" as *const u8 as *const i8, 3_u32);
        pdf_dev_grestore();
        pdf_doc_add_page_resource(
            b"XObject\x00" as *const u8 as *const i8,
            res_name,
            pdf_ximage_get_reference(id),
        );
        /* ENABLE_HTML_SVG_XXX */
    }
    error
}
/* ENABLE_HTML_IMG_SUPPORT */
unsafe extern "C" fn spc_handler_html_default(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32 {
    let mut sd: *mut spc_html_ = &mut _html_state; /* treat "open" same as "empty" */
    let mut name: [i8; 128] = [0; 128]; /* treat "open" same as "empty" */
    let mut attr: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: i32 = 0i32;
    let mut type_0: i32 = 1i32;
    if (*ap).curptr >= (*ap).endptr {
        return 0i32;
    }
    attr = pdf_new_dict();
    error = read_html_tag(
        name.as_mut_ptr(),
        attr,
        &mut type_0,
        &mut (*ap).curptr,
        (*ap).endptr,
    );
    if error != 0 {
        pdf_release_obj(attr);
        return error;
    }
    if streq_ptr(name.as_mut_ptr(), b"a\x00" as *const u8 as *const i8) {
        match type_0 {
            1 => error = spc_html__anchor_open(spe, attr, sd),
            2 => error = spc_html__anchor_close(spe, sd),
            _ => {
                spc_warn(
                    spe,
                    b"Empty html anchor tag???\x00" as *const u8 as *const i8,
                );
                error = -1i32
            }
        }
    } else if streq_ptr(name.as_mut_ptr(), b"base\x00" as *const u8 as *const i8) {
        if type_0 == 2i32 {
            spc_warn(
                spe,
                b"Close tag for \"base\"???\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            error = spc_html__base_empty(spe, attr, sd)
        }
    } else if streq_ptr(name.as_mut_ptr(), b"img\x00" as *const u8 as *const i8) {
        if type_0 == 2i32 {
            spc_warn(
                spe,
                b"Close tag for \"img\"???\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            error = spc_html__img_empty(spe, attr)
        }
    }
    pdf_release_obj(attr);
    while (*ap).curptr < (*ap).endptr && libc::isspace(*(*ap).curptr.offset(0) as _) != 0 {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    error
}
/* translate wsp* '(' wsp* number (comma-wsp number)? wsp* ')' */
unsafe extern "C" fn cvt_a_to_tmatrix(
    M: &mut pdf_tmatrix,
    mut ptr: *const i8,
    mut nextptr: *mut *const i8,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = ptr;
    let mut n: i32 = 0;
    let mut v: [f64; 6] = [0.; 6];
    static mut _tkeys: [*const i8; 7] = [
        b"matrix\x00" as *const u8 as *const i8,
        b"translate\x00" as *const u8 as *const i8,
        b"scale\x00" as *const u8 as *const i8,
        b"rotate\x00" as *const u8 as *const i8,
        b"skewX\x00" as *const u8 as *const i8,
        b"skewY\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut k: i32 = 0;
    while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    q = parse_c_ident(&mut p, p.offset(strlen(p) as isize));
    if q.is_null() {
        return -1i32;
    }
    /* parsed transformation key */
    k = 0i32;
    while !_tkeys[k as usize].is_null() && strcmp(q, _tkeys[k as usize]) != 0 {
        k += 1
    }
    free(q as *mut libc::c_void);
    /* handle args */
    while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    if *p as i32 != '(' as i32 || *p.offset(1) as i32 == 0i32 {
        return -1i32;
    }
    p = p.offset(1);
    while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    n = 0i32;
    while n < 6i32 && *p as i32 != 0 && *p as i32 != ')' as i32 {
        q = parse_float_decimal(&mut p, p.offset(strlen(p) as isize));
        if q.is_null() {
            break;
        }
        v[n as usize] = atof(q);
        if *p as i32 == ',' as i32 {
            p = p.offset(1)
        }
        while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
        if *p as i32 == ',' as i32 {
            p = p.offset(1);
            while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
                p = p.offset(1)
            }
        }
        free(q as *mut libc::c_void);
        n += 1
    }
    if *p as i32 != ')' as i32 {
        return -1i32;
    }
    p = p.offset(1);
    match k {
        0 => {
            if n != 6i32 {
                return -1i32;
            }
            M.a = v[0];
            M.c = v[1];
            M.b = v[2];
            M.d = v[3];
            M.e = v[4];
            M.f = v[5]
        }
        1 => {
            if n != 1i32 && n != 2i32 {
                return -1i32;
            }
            M.d = 1.;
            M.a = M.d;
            M.b = 0.;
            M.c = M.b;
            M.e = v[0];
            M.f = if n == 2i32 { v[1] } else { 0. }
        }
        2 => {
            if n != 1i32 && n != 2i32 {
                return -1i32;
            }
            M.a = v[0];
            M.d = if n == 2i32 { v[1] } else { v[0] };
            M.b = 0.;
            M.c = M.b;
            M.f = 0.;
            M.e = M.f
        }
        3 => {
            if n != 1i32 && n != 3i32 {
                return -1i32;
            }
            let (s, c) = (v[0] * core::f64::consts::PI / 180.).sin_cos();
            M.a = c;
            M.c = s;
            M.b = -s;
            M.d = c;
            M.e = if n == 3i32 { v[1] } else { 0.0f64 };
            M.f = if n == 3i32 { v[2] } else { 0.0f64 }
        }
        4 => {
            if n != 1i32 {
                return -1i32;
            }
            M.d = 1.;
            M.a = M.d;
            M.c = 0.;
            M.b = tan(v[0] * core::f64::consts::PI / 180.)
        }
        5 => {
            if n != 1i32 {
                return -1i32;
            }
            M.d = 1.;
            M.a = M.d;
            M.c = tan(v[0] * core::f64::consts::PI / 180.);
            M.b = 0.
        }
        _ => {}
    }
    if !nextptr.is_null() {
        *nextptr = p
    }
    0i32
}
/* ENABLE_HTML_SVG_TRANSFORM */
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_begin_document() -> i32 {
    let mut sd: *mut spc_html_ = &mut _html_state;
    spc_handler_html__init(sd as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_begin_page() -> i32 {
    let mut sd: *mut spc_html_ = &mut _html_state;
    spc_handler_html__bophook(0 as *mut spc_env, sd as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_end_page() -> i32 {
    let mut sd: *mut spc_html_ = &mut _html_state;
    spc_handler_html__eophook(0 as *mut spc_env, sd as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_at_end_document() -> i32 {
    let mut sd: *mut spc_html_ = &mut _html_state;
    spc_handler_html__clean(0 as *mut spc_env, sd as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_html_check_special(mut buffer: *const i8, mut size: i32) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    p = buffer;
    endptr = p.offset(size as isize);
    while p < endptr && libc::isspace(*p as _) != 0 {
        p = p.offset(1)
    }
    size = endptr.wrapping_offset_from(p) as i64 as i32;
    if size as u64 >= strlen(b"html:\x00" as *const u8 as *const i8)
        && memcmp(
            p as *const libc::c_void,
            b"html:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"html:\x00" as *const u8 as *const i8),
        ) == 0
    {
        return true;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn spc_html_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> i32 {
    assert!(!sph.is_null() && !spe.is_null() && !ap.is_null());
    while (*ap).curptr < (*ap).endptr && libc::isspace(*(*ap).curptr.offset(0) as _) != 0 {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    if (*ap)
        .curptr
        .offset(strlen(b"html:\x00" as *const u8 as *const i8) as isize)
        > (*ap).endptr
        || memcmp(
            (*ap).curptr as *const libc::c_void,
            b"html:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"html:\x00" as *const u8 as *const i8),
        ) != 0
    {
        return -1i32;
    }
    (*ap).command = b"\x00" as *const u8 as *const i8;
    (*sph).key = b"html:\x00" as *const u8 as *const i8;
    (*sph).exec = Some(
        spc_handler_html_default as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
    );
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"html:\x00" as *const u8 as *const i8) as isize);
    while (*ap).curptr < (*ap).endptr && libc::isspace(*(*ap).curptr.offset(0) as _) != 0 {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    0i32
}
