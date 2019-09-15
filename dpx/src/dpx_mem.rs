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
#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use libc::free;
extern "C" {
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;

#[no_mangle]
pub unsafe extern "C" fn new(mut size: u32) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = malloc(size as size_t);
    if result.is_null() {
        panic!("Out of memory - asked for {} bytes\n", size);
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn renew(mut mem: *mut libc::c_void, mut size: u32) -> *mut libc::c_void {
    if size != 0 {
        let mut result: *mut libc::c_void = realloc(mem, size as size_t);
        if result.is_null() {
            panic!("Out of memory - asked for {} bytes\n", size);
        }
        return result;
    } else {
        /* realloc may not return NULL if size == 0 */
        free(mem);
        return 0 as *mut libc::c_void;
    };
}
