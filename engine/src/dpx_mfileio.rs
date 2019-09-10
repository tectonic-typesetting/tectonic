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
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_ungetc(handle: rust_input_handle_t, ch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
    _tt_abort(
        b"io:  An OS command failed that should not have.\n\x00" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn seek_relative(mut file: *mut FILE, mut pos: int32_t) {
    if fseek(file, pos as libc::c_long, 1i32) != 0 {
        os_error();
    };
}
unsafe extern "C" fn seek_end(mut file: *mut FILE) {
    if fseek(file, 0, 2i32) != 0 {
        os_error();
    };
}
unsafe extern "C" fn tell_position(mut file: *mut FILE) -> int32_t {
    let mut size: libc::c_long = ftell(file);
    if size < 0i32 as libc::c_long {
        os_error();
    }
    if size > 0x7fffffffi32 as libc::c_long {
        _tt_abort(
            b"ftell: file size %ld exceeds 0x7fffffff.\n\x00" as *const u8 as *const libc::c_char,
            size,
        );
    }
    return size as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn file_size(mut file: *mut FILE) -> int32_t {
    let mut size: int32_t = 0;
    seek_end(file);
    size = tell_position(file);
    rewind(file);
    return size;
}
/* Unlike fgets, mfgets works with \r, \n, or \r\n end of lines. */
#[no_mangle]
pub unsafe extern "C" fn mfgets(
    mut buffer: *mut libc::c_char,
    mut length: libc::c_int,
    mut file: *mut FILE,
) -> *mut libc::c_char {
    let mut ch: libc::c_int = 0i32;
    let mut i: libc::c_int = 0i32;
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
        *buffer.offset(fresh0 as isize) = ch as libc::c_char
    }
    *buffer.offset(i as isize) = 0i32 as libc::c_char;
    if ch < 0i32 && i == 0i32 {
        return 0 as *mut libc::c_char;
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
    return buffer;
}
/* Note: this is really just a random array used in other files. */
#[no_mangle]
pub static mut work_buffer: [libc::c_char; 1024] = [0; 1024];
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
    mut buffer: *mut libc::c_char,
    mut length: libc::c_int,
    mut file: rust_input_handle_t,
) -> *mut libc::c_char {
    let mut ch: libc::c_int = 0i32;
    let mut i: libc::c_int = 0i32;
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
        *buffer.offset(fresh1 as isize) = ch as libc::c_char
    }
    *buffer.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    if ch < 0i32 && i == 0i32 {
        return 0 as *mut libc::c_char;
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
    return buffer;
}
