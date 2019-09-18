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

use super::{spc_arg, spc_env};
use crate::dpx_dpxutil::parse_c_ident;
use crate::dpx_pdfparse::skip_white;
use crate::streq_ptr;
use libc::free;
extern "C" {
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
}
pub type size_t = u64;

pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
use super::spc_handler;
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */

unsafe extern "C" fn spc_handler_null(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    (*args).curptr = (*args).endptr;
    0i32
}
static mut dvipdfmx_handlers: [spc_handler; 1] = [{
    let mut init = spc_handler {
        key: b"config\x00" as *const u8 as *const i8,
        exec: Some(
            spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
        ),
    };
    init
}];

#[no_mangle]
pub unsafe extern "C" fn spc_dvipdfmx_check_special(mut buf: *const i8, mut len: i32) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p.offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const i8) as isize) <= endptr
        && memcmp(
            p as *const libc::c_void,
            b"dvipdfmx:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"dvipdfmx:\x00" as *const u8 as *const i8),
        ) == 0
    {
        return true;
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvipdfmx_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut error: i32 = -1i32;
    let mut i: size_t = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    assert!(!sph.is_null() && !spe.is_null() && !ap.is_null());
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap)
        .curptr
        .offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const i8) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const libc::c_void,
            b"dvipdfmx:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"dvipdfmx:\x00" as *const u8 as *const i8),
        ) != 0
    {
        spc_warn(
            spe,
            b"Not dvipdfmx: special???\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"dvipdfmx:\x00" as *const u8 as *const i8) as isize);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        i = 0i32 as size_t;
        while i
            < (::std::mem::size_of::<[spc_handler; 1]>() as u64)
                .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
        {
            if streq_ptr(q, dvipdfmx_handlers[i as usize].key) {
                (*ap).command = dvipdfmx_handlers[i as usize].key;
                (*sph).key = b"dvipdfmx:\x00" as *const u8 as *const i8;
                (*sph).exec = dvipdfmx_handlers[i as usize].exec;
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                error = 0i32;
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
        free(q as *mut libc::c_void);
    }
    error
}
