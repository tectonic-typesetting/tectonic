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

use super::util::spc_util_read_colorspec;
use super::{spc_arg, spc_env, spc_handler, spc_warn};
use crate::dpx_dpxutil::parse_c_ident;
use crate::dpx_pdfcolor::{
    pdf_color, pdf_color_clear_stack, pdf_color_pop, pdf_color_push, pdf_color_set,
};
use crate::dpx_pdfdoc::pdf_doc_set_bgcolor;
use crate::streq_ptr;
use libc::free;

/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
/* Color stack is actually placed into pdfcolor.c.
 * The reason why we need to place stack there is
 * that we must reinstall color after grestore and
 * other operations that can change current color
 * implicitely.
 */
unsafe extern "C" fn spc_handler_color_push(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut error: i32 = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: None,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 1i32);
    if error == 0 {
        let color_clone = colorspec.clone();
        pdf_color_push(&mut colorspec, &color_clone);
    }
    error
}
unsafe extern "C" fn spc_handler_color_pop(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    pdf_color_pop();
    0i32
}
/* Invoked by the special command "color rgb .625 0 0".
 * DVIPS clears the color stack, and then saves and sets the given color.
 */
unsafe extern "C" fn spc_handler_color_default(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    let mut error: i32 = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: None,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 1i32);
    if error == 0 {
        pdf_color_clear_stack();
        pdf_color_set(&colorspec, &colorspec);
    }
    error
}
/* This is from color special? */
unsafe extern "C" fn spc_handler_background(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut error: i32 = 0;
    let mut colorspec = pdf_color {
        num_components: 0,
        spot_color_name: None,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 1i32);
    if error == 0 {
        pdf_doc_set_bgcolor(Some(&colorspec));
    }
    error
}
unsafe extern "C" fn skip_blank(mut pp: *mut *const i8, mut endptr: *const i8) {
    let mut p: *const i8 = *pp;
    while p < endptr && (*p as i32 & !0x7fi32 == 0i32 && crate::isblank(*p as _) != 0) {
        p = p.offset(1)
    }
    *pp = p;
}
#[no_mangle]
pub unsafe extern "C" fn spc_color_check_special(mut buf: *const i8, mut len: i32) -> bool {
    let mut r: bool = false;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    p = buf;
    endptr = p.offset(len as isize);
    skip_blank(&mut p, endptr);
    q = parse_c_ident(&mut p, endptr);
    if q.is_null() {
        return false;
    } else {
        if streq_ptr(q, b"color\x00" as *const u8 as *const i8) {
            r = true
        } else if streq_ptr(q, b"background\x00" as *const u8 as *const i8) {
            r = true
        }
    }
    free(q as *mut libc::c_void);
    r
}
#[no_mangle]
pub unsafe extern "C" fn spc_color_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    assert!(!sph.is_null() && !spe.is_null() && !ap.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        return -1i32;
    }
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if streq_ptr(q, b"background\x00" as *const u8 as *const i8) {
        (*ap).command = b"background\x00" as *const u8 as *const i8;
        (*sph).exec = Some(
            spc_handler_background as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
        );
        free(q as *mut libc::c_void);
    } else if streq_ptr(q, b"color\x00" as *const u8 as *const i8) {
        /* color */
        free(q as *mut libc::c_void); /* cmyk, rgb, ... */
        p = (*ap).curptr;
        q = parse_c_ident(&mut p, (*ap).endptr);
        if q.is_null() {
            return -1i32;
        } else {
            if streq_ptr(q, b"push\x00" as *const u8 as *const i8) {
                (*ap).command = b"push\x00" as *const u8 as *const i8;
                (*sph).exec = Some(
                    spc_handler_color_push
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                );
                (*ap).curptr = p
            } else if streq_ptr(q, b"pop\x00" as *const u8 as *const i8) {
                (*ap).command = b"pop\x00" as *const u8 as *const i8;
                (*sph).exec = Some(
                    spc_handler_color_pop
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                );
                (*ap).curptr = p
            } else {
                (*ap).command = b"\x00" as *const u8 as *const i8;
                (*sph).exec = Some(
                    spc_handler_color_default
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                )
            }
        }
        free(q as *mut libc::c_void);
    } else {
        spc_warn(
            spe,
            b"Not color/background special?\x00" as *const u8 as *const i8,
        );
        free(q as *mut libc::c_void);
        return -1i32;
    }
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    0i32
}
