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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
}
pub type size_t = libc::c_ulong;
/* tectonic/core-memory.c: basic C dynamic memory helpers

   Copyright 1993, 1994, 1995, 2008, 2009, 2010, 2011 Karl Berry.
   Copyright 1997, 2002, 2005 Olaf Weber.

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this library; if not, see <http://www.gnu.org/licenses/>.  */
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut nelem: size_t, mut elsize: size_t)
 -> *mut libc::c_void {
    let mut new_mem: *mut libc::c_void =
        calloc(if nelem != 0 { nelem } else { 1i32 as libc::c_ulong },
               if elsize != 0 { elsize } else { 1i32 as libc::c_ulong });
    if new_mem.is_null() {
        _tt_abort(b"xcalloc request for %lu elements of size %lu failed\x00"
                      as *const u8 as *const libc::c_char, nelem, elsize);
    }
    return new_mem;
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut new_mem: *mut libc::c_void =
        malloc(if size != 0 { size } else { 1i32 as libc::c_ulong });
    if new_mem.is_null() {
        _tt_abort(b"xmalloc request for %lu bytes failed\x00" as *const u8 as
                      *const libc::c_char, size);
    }
    return new_mem;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(mut old_ptr: *mut libc::c_void,
                                  mut size: size_t) -> *mut libc::c_void {
    let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if old_ptr.is_null() {
        new_mem = xmalloc(size)
    } else {
        new_mem =
            realloc(old_ptr,
                    if size != 0 { size } else { 1i32 as libc::c_ulong });
        if new_mem.is_null() {
            _tt_abort(b"xrealloc() to %lu bytes failed\x00" as *const u8 as
                          *const libc::c_char, size);
        }
    }
    return new_mem;
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut new_string: *mut libc::c_char =
        xmalloc(strlen(s).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    return strcpy(new_string, s);
}
