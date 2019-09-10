#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    /* Global symbols that route through the global API variable. Hopefully we
     * will one day eliminate all of the global state and get rid of all of
     * these. */
    #[no_mangle]
    fn ttstub_issue_warning(format: *const i8, _: ...);
    #[no_mangle]
    fn ttstub_output_open_stdout() -> rust_output_handle_t;
    #[no_mangle]
    fn ttstub_output_write(
        handle: rust_output_handle_t,
        data: *const i8,
        len: size_t,
    ) -> size_t;
    #[no_mangle]
    fn vsnprintf(
        _: *mut i8,
        _: u64,
        _: *const i8,
        _: ::std::ffi::VaList,
    ) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = u64;
pub type rust_output_handle_t = *mut libc::c_void;
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
pub type message_type_t = _message_type;
pub type _message_type = u32;
pub const DPX_MESG_WARN: _message_type = 1;
pub const DPX_MESG_INFO: _message_type = 0;
static mut _last_message_type: message_type_t = DPX_MESG_INFO;
static mut _dpx_quietness: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn shut_up(mut quietness: i32) {
    _dpx_quietness = quietness;
}
static mut _dpx_message_handle: rust_output_handle_t =
    0 as *const libc::c_void as *mut libc::c_void;
static mut _dpx_message_buf: [i8; 1024] = [0; 1024];
unsafe extern "C" fn _dpx_ensure_output_handle() -> rust_output_handle_t {
    _dpx_message_handle = ttstub_output_open_stdout();
    if _dpx_message_handle.is_null() {
        _tt_abort(
            b"xdvipdfmx cannot get output logging handle?!\x00" as *const u8 as *const i8,
        );
    }
    return _dpx_message_handle;
}
unsafe extern "C" fn _dpx_print_to_stdout(
    mut fmt: *const i8,
    mut argp: ::std::ffi::VaList,
    mut warn: i32,
) {
    let mut n: i32 = 0;
    n = vsnprintf(
        _dpx_message_buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as u64,
        fmt,
        argp.as_va_list(),
    );
    /* n is the number of bytes the vsnprintf() wanted to write -- it might be
     * bigger than sizeof(buf). */
    if n as u64 >= ::std::mem::size_of::<[i8; 1024]>() as u64 {
        n = (::std::mem::size_of::<[i8; 1024]>() as u64)
            .wrapping_sub(1i32 as u64) as i32;
        _dpx_message_buf[n as usize] = '\u{0}' as i32 as i8
    }
    if warn != 0 {
        ttstub_issue_warning(
            b"%s\x00" as *const u8 as *const i8,
            _dpx_message_buf.as_mut_ptr(),
        );
    }
    ttstub_output_write(
        _dpx_ensure_output_handle(),
        _dpx_message_buf.as_mut_ptr(),
        n as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dpx_message(mut fmt: *const i8, mut args: ...) {
    let mut argp: ::std::ffi::VaListImpl;
    if _dpx_quietness > 0i32 {
        return;
    }
    argp = args.clone();
    _dpx_print_to_stdout(fmt, argp.as_va_list(), 0i32);
    _last_message_type = DPX_MESG_INFO;
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
pub unsafe extern "C" fn dpx_warning(mut fmt: *const i8, mut args: ...) {
    let mut argp: ::std::ffi::VaListImpl;
    if _dpx_quietness > 1i32 {
        return;
    }
    if _last_message_type as u32 == DPX_MESG_INFO as i32 as u32 {
        ttstub_output_write(
            _dpx_ensure_output_handle(),
            b"\n\x00" as *const u8 as *const i8,
            1i32 as size_t,
        );
    }
    ttstub_output_write(
        _dpx_ensure_output_handle(),
        b"warning: \x00" as *const u8 as *const i8,
        9i32 as size_t,
    );
    argp = args.clone();
    _dpx_print_to_stdout(fmt, argp.as_va_list(), 1i32);
    ttstub_output_write(
        _dpx_ensure_output_handle(),
        b"\n\x00" as *const u8 as *const i8,
        1i32 as size_t,
    );
    _last_message_type = DPX_MESG_WARN;
}
