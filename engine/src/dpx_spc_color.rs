#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: libc::c_uint,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> libc::c_int;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn parse_c_ident(
        pp: *mut *const i8,
        endptr: *const i8,
    ) -> *mut i8;
    /* Color special
     * See remark in spc_color.c.
     */
    #[no_mangle]
    fn pdf_color_set(sc: *mut pdf_color, fc: *mut pdf_color);
    #[no_mangle]
    fn pdf_color_push(sc: *mut pdf_color, fc: *mut pdf_color);
    #[no_mangle]
    fn pdf_color_pop();
    /* Color stack
     */
    #[no_mangle]
    fn pdf_color_clear_stack();
    #[no_mangle]
    fn spc_util_read_colorspec(
        spe: *mut spc_env,
        colorspec: *mut pdf_color,
        args: *mut spc_arg,
        syntax: libc::c_int,
    ) -> libc::c_int;
    /* Similar to bop_content */
    #[no_mangle]
    fn pdf_doc_set_bgcolor(color: *const pdf_color);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: f64,
    pub y_user: f64,
    pub mag: f64,
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
pub struct pdf_color {
    pub num_components: libc::c_int,
    pub spot_color_name: *mut i8,
    pub values: [f64; 4],
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
/* Color stack is actually placed into pdfcolor.c.
 * The reason why we need to place stack there is
 * that we must reinstall color after grestore and
 * other operations that can change current color
 * implicitely.
 */
unsafe extern "C" fn spc_handler_color_push(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 1i32);
    if error == 0 {
        pdf_color_push(&mut colorspec, &mut colorspec);
    }
    return error;
}
unsafe extern "C" fn spc_handler_color_pop(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    pdf_color_pop();
    return 0i32;
}
/* Invoked by the special command "color rgb .625 0 0".
 * DVIPS clears the color stack, and then saves and sets the given color.
 */
unsafe extern "C" fn spc_handler_color_default(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 1i32);
    if error == 0 {
        pdf_color_clear_stack();
        pdf_color_set(&mut colorspec, &mut colorspec);
    }
    return error;
}
/* This is from color special? */
unsafe extern "C" fn spc_handler_background(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 1i32);
    if error == 0 {
        pdf_doc_set_bgcolor(&mut colorspec);
    }
    return error;
}
unsafe extern "C" fn skip_blank(mut pp: *mut *const i8, mut endptr: *const i8) {
    let mut p: *const i8 = *pp;
    while p < endptr
        && (*p as libc::c_int & !0x7fi32 == 0i32
            && *(*__ctype_b_loc()).offset(*p as u8 as libc::c_int as isize)
                as libc::c_int
                & _ISblank as libc::c_int as u16 as libc::c_int
                != 0)
    {
        p = p.offset(1)
    }
    *pp = p;
}
#[no_mangle]
pub unsafe extern "C" fn spc_color_check_special(
    mut buf: *const i8,
    mut len: libc::c_int,
) -> bool {
    let mut r: bool = 0i32 != 0;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    p = buf;
    endptr = p.offset(len as isize);
    skip_blank(&mut p, endptr);
    q = parse_c_ident(&mut p, endptr);
    if q.is_null() {
        return 0i32 != 0;
    } else {
        if streq_ptr(q, b"color\x00" as *const u8 as *const i8) {
            r = 1i32 != 0
        } else if streq_ptr(q, b"background\x00" as *const u8 as *const i8) {
            r = 1i32 != 0
        }
    }
    free(q as *mut libc::c_void);
    return r;
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
pub unsafe extern "C" fn spc_color_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    if !sph.is_null() && !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(b"sph && spe && ap\x00" as *const u8 as
                          *const i8,
                      b"dpx-spc_color.c\x00" as *const u8 as
                          *const i8, 141i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[i8; 86]>(b"int spc_color_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        return -1i32;
    }
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if streq_ptr(q, b"background\x00" as *const u8 as *const i8) {
        (*ap).command = b"background\x00" as *const u8 as *const i8;
        (*sph).exec = Some(
            spc_handler_background
                as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
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
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                );
                (*ap).curptr = p
            } else if streq_ptr(q, b"pop\x00" as *const u8 as *const i8) {
                (*ap).command = b"pop\x00" as *const u8 as *const i8;
                (*sph).exec = Some(
                    spc_handler_color_pop
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                );
                (*ap).curptr = p
            } else {
                (*ap).command = b"\x00" as *const u8 as *const i8;
                (*sph).exec = Some(
                    spc_handler_color_default
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
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
    return 0i32;
}
