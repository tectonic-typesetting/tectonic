#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn ttstub_input_ungetc(handle: rust_input_handle_t, ch: i32) -> i32;
}
use libc::{fgetc, fseek, ftell, rewind, ungetc, FILE};
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
pub type rust_input_handle_t = *mut libc::c_void;
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
unsafe extern "C" fn os_error() {
    _tt_abort(b"io:  An OS command failed that should not have.\n\x00" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn seek_relative(mut file: *mut FILE, mut pos: i32) {
    if fseek(file, pos as _, 1i32) != 0 {
        os_error();
    };
}
unsafe extern "C" fn seek_end(mut file: *mut FILE) {
    if fseek(file, 0, 2i32) != 0 {
        os_error();
    };
}
unsafe extern "C" fn tell_position(mut file: *mut FILE) -> i32 {
    let mut size = ftell(file);
    if size < 0 {
        os_error();
    }
    if size as i64 > 0x7fffffffi32 as i64 {
        _tt_abort(
            b"ftell: file size %ld exceeds 0x7fffffff.\n\x00" as *const u8 as *const i8,
            size,
        );
    }
    size as i32
}
#[no_mangle]
pub unsafe extern "C" fn file_size(mut file: *mut FILE) -> i32 {
    let mut size: i32 = 0;
    seek_end(file);
    size = tell_position(file);
    rewind(file);
    size
}
/* Unlike fgets, mfgets works with \r, \n, or \r\n end of lines. */
#[no_mangle]
pub unsafe extern "C" fn mfgets(
    mut buffer: *mut i8,
    mut length: i32,
    mut file: *mut FILE,
) -> *mut i8 {
    let mut ch: i32 = 0i32;
    let mut i: i32 = 0i32;
    while i < length - 1i32
        && {
            ch = fgetc(file);
            ch >= 0i32
        }
        && ch != '\n' as i32
        && ch != '\r' as i32
    {
        let fresh0 = i;
        i = i + 1;
        *buffer.offset(fresh0 as isize) = ch as i8
    }
    *buffer.offset(i as isize) = 0_i8;
    if ch < 0i32 && i == 0i32 {
        return 0 as *mut i8;
    }
    if ch == '\r' as i32
        && {
            ch = fgetc(file);
            ch >= 0i32
        }
        && ch != '\n' as i32
    {
        ungetc(ch, file);
    }
    buffer
}
/* Note: this is really just a random array used in other files. */
#[no_mangle]
pub static mut work_buffer: [i8; 1024] = [0; 1024];
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
/* Tectonic-enabled versions */
/* Modified versions of the above functions based on the Tectonic I/O system. */
#[no_mangle]
pub unsafe extern "C" fn tt_mfgets(
    mut buffer: *mut i8,
    mut length: i32,
    mut file: rust_input_handle_t,
) -> *mut i8 {
    let mut ch: i32 = 0i32;
    let mut i: i32 = 0i32;
    while i < length - 1i32
        && {
            ch = ttstub_input_getc(file);
            ch >= 0i32
        }
        && ch != '\n' as i32
        && ch != '\r' as i32
    {
        let fresh1 = i;
        i = i + 1;
        *buffer.offset(fresh1 as isize) = ch as i8
    }
    *buffer.offset(i as isize) = '\u{0}' as i32 as i8;
    if ch < 0i32 && i == 0i32 {
        return 0 as *mut i8;
    }
    if ch == '\r' as i32
        && {
            ch = ttstub_input_getc(file);
            ch >= 0i32
        }
        && ch != '\n' as i32
    {
        ttstub_input_ungetc(file, ch);
    }
    buffer
}
