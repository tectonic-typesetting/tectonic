#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(label_break_value)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const libc::c_char,
                     endptr: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct spc_env {
    pub x_user: libc::c_double,
    pub y_user: libc::c_double,
    pub mag: libc::c_double,
    pub pg: libc::c_int,
    /* current page in PDF */
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const libc::c_char,
    pub endptr: *const libc::c_char,
    pub base: *const libc::c_char,
    pub command: *const libc::c_char,
}
pub type spc_handler_fn_ptr
    =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct spc_handler {
    pub key: *const libc::c_char,
    pub exec: spc_handler_fn_ptr,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char,
                               mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() { return strcmp(s1, s2) == 0i32 }
    return 0i32 != 0;
}
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
unsafe extern "C" fn spc_handler_null(mut spe: *mut spc_env,
                                      mut args: *mut spc_arg) -> libc::c_int {
    (*args).curptr = (*args).endptr;
    return 0i32;
}
static mut dvipdfmx_handlers: [spc_handler; 1] =
    unsafe {
        [{
             let mut init =
                 spc_handler{key:
                                 b"config\x00" as *const u8 as
                                     *const libc::c_char,
                             exec:
                                 Some(spc_handler_null as
                                          unsafe extern "C" fn(_:
                                                                   *mut spc_env,
                                                               _:
                                                                   *mut spc_arg)
                                              -> libc::c_int),};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn spc_dvipdfmx_check_special(mut buf:
                                                        *const libc::c_char,
                                                    mut len: libc::c_int)
 -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p.offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const libc::c_char)
                    as isize) <= endptr &&
           memcmp(p as *const libc::c_void,
                  b"dvipdfmx:\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  strlen(b"dvipdfmx:\x00" as *const u8 as
                             *const libc::c_char)) == 0 {
        return 1i32 != 0
    }
    return 0i32 != 0;
}
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
pub unsafe extern "C" fn spc_dvipdfmx_setup_handler(mut sph: *mut spc_handler,
                                                    mut spe: *mut spc_env,
                                                    mut ap: *mut spc_arg)
 -> libc::c_int {
    let mut error: libc::c_int = -1i32;
    let mut i: size_t = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if !sph.is_null() && !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(b"sph && spe && ap\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-spc_dvipdfmx.c\x00" as *const u8 as
                          *const libc::c_char, 69i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[libc::c_char; 89]>(b"int spc_dvipdfmx_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr.offset(strlen(b"dvipdfmx:\x00" as *const u8 as
                                      *const libc::c_char) as isize) >=
           (*ap).endptr ||
           memcmp((*ap).curptr as *const libc::c_void,
                  b"dvipdfmx:\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  strlen(b"dvipdfmx:\x00" as *const u8 as
                             *const libc::c_char)) != 0 {
        spc_warn(spe,
                 b"Not dvipdfmx: special???\x00" as *const u8 as
                     *const libc::c_char);
        return -1i32
    }
    (*ap).curptr =
        (*ap).curptr.offset(strlen(b"dvipdfmx:\x00" as *const u8 as
                                       *const libc::c_char) as isize);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        i = 0i32 as size_t;
        while i <
                  (::std::mem::size_of::<[spc_handler; 1]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<spc_handler>()
                                                       as libc::c_ulong) {
            if streq_ptr(q, dvipdfmx_handlers[i as usize].key) {
                (*ap).command = dvipdfmx_handlers[i as usize].key;
                (*sph).key =
                    b"dvipdfmx:\x00" as *const u8 as *const libc::c_char;
                (*sph).exec = dvipdfmx_handlers[i as usize].exec;
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                error = 0i32;
                break ;
            } else { i = i.wrapping_add(1) }
        }
        free(q as *mut libc::c_void);
    }
    return error;
}
