#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::warn;

extern crate libc;
use libc::free;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut i32;
    #[no_mangle]
    fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    #[no_mangle]
    fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> i64;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn xtoi(c: i8) -> i32;
    #[no_mangle]
    fn skip_white_spaces(s: *mut *mut u8, endptr: *mut u8);
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
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
    fn new(size: u32) -> *mut libc::c_void;
}
pub type C2RustUnnamed = u32;
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
pub struct pst_obj {
    pub type_0: pst_type,
    pub data: *mut libc::c_void,
}
pub type pst_type = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_string {
    pub length: u32,
    pub value: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_name {
    pub value: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_real {
    pub value: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_integer {
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_boolean {
    pub value: i8,
}
static mut pst_const_null: *const i8 = b"null\x00" as *const u8 as *const i8;
static mut pst_const_mark: *const i8 = b"mark\x00" as *const u8 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn pst_new_obj(
    mut type_0: pst_type,
    mut data: *mut libc::c_void,
) -> *mut pst_obj {
    let mut obj: *mut pst_obj = 0 as *mut pst_obj;
    obj = new((1_u64).wrapping_mul(::std::mem::size_of::<pst_obj>() as u64) as u32) as *mut pst_obj;
    (*obj).type_0 = type_0;
    (*obj).data = data;
    obj
}
#[no_mangle]
pub unsafe extern "C" fn pst_new_mark() -> *mut pst_obj {
    let mut q: *mut i8 = 0 as *mut i8;
    q = new(
        (strlen(pst_const_mark).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    strcpy(q, pst_const_mark);
    pst_new_obj(7i32, q as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn pst_release_obj(mut obj: *mut pst_obj) {
    assert!(!obj.is_null());
    match (*obj).type_0 {
        1 => {
            pst_boolean_release((*obj).data as *mut pst_boolean);
        }
        2 => {
            pst_integer_release((*obj).data as *mut pst_integer);
        }
        3 => {
            pst_real_release((*obj).data as *mut pst_real);
        }
        6 => {
            pst_name_release((*obj).data as *mut pst_name);
        }
        5 => {
            pst_string_release((*obj).data as *mut pst_string);
        }
        0 | 7 | -1 => {
            free((*obj).data);
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const i8,
                (*obj).type_0,
            );
        }
    }
    free(obj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pst_type_of(mut obj: *mut pst_obj) -> pst_type {
    assert!(!obj.is_null());
    (*obj).type_0
}
#[no_mangle]
pub unsafe extern "C" fn pst_length_of(mut obj: *mut pst_obj) -> i32 {
    let mut len: i32 = 0i32;
    assert!(!obj.is_null());
    match (*obj).type_0 {
        1 => len = pst_boolean_length() as i32,
        2 => len = pst_integer_length() as i32,
        3 => len = pst_real_length() as i32,
        6 => len = pst_name_length((*obj).data as *mut pst_name) as i32,
        5 => len = pst_string_length((*obj).data as *mut pst_string) as i32,
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8 as *const i8,
            );
        }
        -1 => len = strlen((*obj).data as *const i8) as i32,
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const i8,
                (*obj).type_0,
            );
        }
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn pst_getIV(mut obj: *mut pst_obj) -> i32 {
    let mut iv: i32 = 0i32;
    assert!(!obj.is_null());
    match (*obj).type_0 {
        1 => iv = pst_boolean_IV((*obj).data as *mut pst_boolean),
        2 => iv = pst_integer_IV((*obj).data as *mut pst_integer),
        3 => iv = pst_real_IV((*obj).data as *mut pst_real),
        6 => iv = pst_name_IV(),
        5 => iv = pst_string_IV((*obj).data as *mut pst_string),
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8 as *const i8,
            );
        }
        -1 => {
            _tt_abort(
                b"Cannot convert object of type UNKNOWN to integer value.\x00" as *const u8
                    as *const i8,
            );
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const i8,
                (*obj).type_0,
            );
        }
    }
    iv
}
#[no_mangle]
pub unsafe extern "C" fn pst_getRV(mut obj: *mut pst_obj) -> f64 {
    let mut rv: f64 = 0.0f64;
    assert!(!obj.is_null());
    match (*obj).type_0 {
        1 => rv = pst_boolean_RV((*obj).data as *mut pst_boolean),
        2 => rv = pst_integer_RV((*obj).data as *mut pst_integer),
        3 => rv = pst_real_RV((*obj).data as *mut pst_real),
        6 => rv = pst_name_RV(),
        5 => rv = pst_string_RV((*obj).data as *mut pst_string),
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8 as *const i8,
            );
        }
        -1 => {
            _tt_abort(
                b"Cannot convert object of type UNKNOWN to real value.\x00" as *const u8
                    as *const i8,
            );
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const i8,
                (*obj).type_0,
            );
        }
    }
    rv
}
/* Length can be obtained by pst_length_of(). */
#[no_mangle]
pub unsafe extern "C" fn pst_getSV(mut obj: *mut pst_obj) -> *mut u8 {
    let mut sv: *mut u8 = 0 as *mut u8;
    assert!(!obj.is_null());
    match (*obj).type_0 {
        1 => sv = pst_boolean_SV((*obj).data as *mut pst_boolean),
        2 => sv = pst_integer_SV((*obj).data as *mut pst_integer),
        3 => sv = pst_real_SV((*obj).data as *mut pst_real),
        6 => sv = pst_name_SV((*obj).data as *mut pst_name),
        5 => sv = pst_string_SV((*obj).data as *mut pst_string),
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8 as *const i8,
            );
        }
        -1 => {
            let mut len: i32 = 0;
            len = strlen((*obj).data as *mut i8) as i32;
            if len > 0i32 {
                sv = new(((len + 1i32) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                    as u32) as *mut u8;
                memcpy(sv as *mut libc::c_void, (*obj).data, len as u64);
                *sv.offset(len as isize) = '\u{0}' as i32 as u8
            } else {
                sv = 0 as *mut u8
            }
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const i8,
                (*obj).type_0,
            );
        }
    }
    sv
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
pub unsafe extern "C" fn pst_data_ptr(mut obj: *mut pst_obj) -> *mut libc::c_void {
    let mut p: *mut i8 = 0 as *mut i8;
    assert!(!obj.is_null());
    match (*obj).type_0 {
        1 => p = pst_boolean_data_ptr((*obj).data as *mut pst_boolean) as *mut i8,
        2 => p = pst_integer_data_ptr((*obj).data as *mut pst_integer) as *mut i8,
        3 => p = pst_real_data_ptr((*obj).data as *mut pst_real) as *mut i8,
        6 => p = pst_name_data_ptr((*obj).data as *mut pst_name) as *mut i8,
        5 => p = pst_string_data_ptr((*obj).data as *mut pst_string) as *mut i8,
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8 as *const i8,
            );
        }
        -1 => p = (*obj).data as *mut i8,
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const i8,
                (*obj).type_0,
            );
        }
    }
    p as *mut libc::c_void
}
/* BOOLEAN */
/* BOOLEAN */
unsafe extern "C" fn pst_boolean_new(mut value: i8) -> *mut pst_boolean {
    let mut obj: *mut pst_boolean = 0 as *mut pst_boolean;
    obj = new((1_u64).wrapping_mul(::std::mem::size_of::<pst_boolean>() as u64) as u32)
        as *mut pst_boolean;
    (*obj).value = value;
    obj
}
unsafe extern "C" fn pst_boolean_release(mut obj: *mut pst_boolean) {
    assert!(!obj.is_null());
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn pst_boolean_IV(mut obj: *mut pst_boolean) -> i32 {
    assert!(!obj.is_null());
    (*obj).value as i32
}
unsafe extern "C" fn pst_boolean_RV(mut obj: *mut pst_boolean) -> f64 {
    assert!(!obj.is_null());
    (*obj).value as f64
}
unsafe extern "C" fn pst_boolean_SV(mut obj: *mut pst_boolean) -> *mut u8 {
    let mut str: *mut u8 = 0 as *mut u8;
    assert!(!obj.is_null());
    if (*obj).value != 0 {
        str = new((5_u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        memcpy(
            str as *mut libc::c_void,
            b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
            4i32 as u64,
        );
        *str.offset(4) = '\u{0}' as i32 as u8
    } else {
        str = new((6_u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        memcpy(
            str as *mut libc::c_void,
            b"false\x00" as *const u8 as *const i8 as *const libc::c_void,
            5i32 as u64,
        );
        *str.offset(5) = '\u{0}' as i32 as u8
    }
    str
}
unsafe extern "C" fn pst_boolean_length() -> u32 {
    _tt_abort(b"Operation not defined for this type of object.\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn pst_boolean_data_ptr(mut obj: *mut pst_boolean) -> *mut libc::c_void {
    assert!(!obj.is_null());
    &mut (*obj).value as *mut i8 as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn pst_parse_boolean(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_obj {
    if (*inbuf).offset(4) <= inbufend
        && memcmp(
            *inbuf as *const libc::c_void,
            b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
            4i32 as u64,
        ) == 0i32
        && ((*inbuf).offset(4) == inbufend
            || (*(*inbuf).offset(4) as i32 == '(' as i32
                || *(*inbuf).offset(4) as i32 == ')' as i32
                || *(*inbuf).offset(4) as i32 == '/' as i32
                || *(*inbuf).offset(4) as i32 == '<' as i32
                || *(*inbuf).offset(4) as i32 == '>' as i32
                || *(*inbuf).offset(4) as i32 == '[' as i32
                || *(*inbuf).offset(4) as i32 == ']' as i32
                || *(*inbuf).offset(4) as i32 == '{' as i32
                || *(*inbuf).offset(4) as i32 == '}' as i32
                || *(*inbuf).offset(4) as i32 == '%' as i32)
            || (*(*inbuf).offset(4) as i32 == ' ' as i32
                || *(*inbuf).offset(4) as i32 == '\t' as i32
                || *(*inbuf).offset(4) as i32 == '\u{c}' as i32
                || *(*inbuf).offset(4) as i32 == '\r' as i32
                || *(*inbuf).offset(4) as i32 == '\n' as i32
                || *(*inbuf).offset(4) as i32 == '\u{0}' as i32))
    {
        *inbuf = (*inbuf).offset(4);
        return pst_new_obj(1i32, pst_boolean_new(1_i8) as *mut libc::c_void);
    } else if (*inbuf).offset(5) <= inbufend
        && memcmp(
            *inbuf as *const libc::c_void,
            b"false\x00" as *const u8 as *const i8 as *const libc::c_void,
            5i32 as u64,
        ) == 0i32
        && ((*inbuf).offset(5) == inbufend
            || (*(*inbuf).offset(5) as i32 == '(' as i32
                || *(*inbuf).offset(5) as i32 == ')' as i32
                || *(*inbuf).offset(5) as i32 == '/' as i32
                || *(*inbuf).offset(5) as i32 == '<' as i32
                || *(*inbuf).offset(5) as i32 == '>' as i32
                || *(*inbuf).offset(5) as i32 == '[' as i32
                || *(*inbuf).offset(5) as i32 == ']' as i32
                || *(*inbuf).offset(5) as i32 == '{' as i32
                || *(*inbuf).offset(5) as i32 == '}' as i32
                || *(*inbuf).offset(5) as i32 == '%' as i32)
            || (*(*inbuf).offset(5) as i32 == ' ' as i32
                || *(*inbuf).offset(5) as i32 == '\t' as i32
                || *(*inbuf).offset(5) as i32 == '\u{c}' as i32
                || *(*inbuf).offset(5) as i32 == '\r' as i32
                || *(*inbuf).offset(5) as i32 == '\n' as i32
                || *(*inbuf).offset(5) as i32 == '\u{0}' as i32))
    {
        *inbuf = (*inbuf).offset(5);
        return pst_new_obj(1i32, pst_boolean_new(0_i8) as *mut libc::c_void);
    } else {
        return 0 as *mut pst_obj;
    };
}
/* NULL */
#[no_mangle]
pub unsafe extern "C" fn pst_parse_null(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_obj {
    if (*inbuf).offset(4) <= inbufend
        && memcmp(
            *inbuf as *const libc::c_void,
            b"null\x00" as *const u8 as *const i8 as *const libc::c_void,
            4i32 as u64,
        ) == 0i32
        && ((*inbuf).offset(4) == inbufend
            || (*(*inbuf).offset(4) as i32 == '(' as i32
                || *(*inbuf).offset(4) as i32 == ')' as i32
                || *(*inbuf).offset(4) as i32 == '/' as i32
                || *(*inbuf).offset(4) as i32 == '<' as i32
                || *(*inbuf).offset(4) as i32 == '>' as i32
                || *(*inbuf).offset(4) as i32 == '[' as i32
                || *(*inbuf).offset(4) as i32 == ']' as i32
                || *(*inbuf).offset(4) as i32 == '{' as i32
                || *(*inbuf).offset(4) as i32 == '}' as i32
                || *(*inbuf).offset(4) as i32 == '%' as i32)
            || (*(*inbuf).offset(4) as i32 == ' ' as i32
                || *(*inbuf).offset(4) as i32 == '\t' as i32
                || *(*inbuf).offset(4) as i32 == '\u{c}' as i32
                || *(*inbuf).offset(4) as i32 == '\r' as i32
                || *(*inbuf).offset(4) as i32 == '\n' as i32
                || *(*inbuf).offset(4) as i32 == '\u{0}' as i32))
    {
        let mut q: *mut i8 = 0 as *mut i8;
        *inbuf = (*inbuf).offset(4);
        q = new(
            (strlen(pst_const_null).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        strcpy(q, pst_const_null);
        return pst_new_obj(0i32, q as *mut libc::c_void);
    } else {
        return 0 as *mut pst_obj;
    };
}
/* NUMBERS */
/* INTEGER */
unsafe extern "C" fn pst_integer_new(mut value: i32) -> *mut pst_integer {
    let mut obj: *mut pst_integer = 0 as *mut pst_integer;
    obj = new((1_u64).wrapping_mul(::std::mem::size_of::<pst_integer>() as u64) as u32)
        as *mut pst_integer;
    (*obj).value = value;
    obj
}
unsafe extern "C" fn pst_integer_release(mut obj: *mut pst_integer) {
    assert!(!obj.is_null());
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn pst_integer_IV(mut obj: *mut pst_integer) -> i32 {
    assert!(!obj.is_null());
    (*obj).value
}
unsafe extern "C" fn pst_integer_RV(mut obj: *mut pst_integer) -> f64 {
    assert!(!obj.is_null());
    (*obj).value as f64
}
unsafe extern "C" fn pst_integer_SV(mut obj: *mut pst_integer) -> *mut u8 {
    let mut value: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut fmt_buf: [i8; 15] = [0; 15];
    assert!(!obj.is_null());
    len = sprintf(
        fmt_buf.as_mut_ptr(),
        b"%d\x00" as *const u8 as *const i8,
        (*obj).value,
    );
    value =
        new(((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    strcpy(value, fmt_buf.as_mut_ptr());
    value as *mut u8
}
unsafe extern "C" fn pst_integer_data_ptr(mut obj: *mut pst_integer) -> *mut libc::c_void {
    assert!(!obj.is_null());
    &mut (*obj).value as *mut i32 as *mut libc::c_void
}
unsafe extern "C" fn pst_integer_length() -> u32 {
    _tt_abort(b"Operation not defined for this type of object.\x00" as *const u8 as *const i8);
}
/* REAL */
unsafe extern "C" fn pst_real_new(mut value: f64) -> *mut pst_real {
    let mut obj: *mut pst_real = 0 as *mut pst_real;
    obj =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pst_real>() as u64) as u32) as *mut pst_real;
    (*obj).value = value;
    obj
}
unsafe extern "C" fn pst_real_release(mut obj: *mut pst_real) {
    assert!(!obj.is_null());
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn pst_real_IV(mut obj: *mut pst_real) -> i32 {
    assert!(!obj.is_null());
    (*obj).value as i32
}
unsafe extern "C" fn pst_real_RV(mut obj: *mut pst_real) -> f64 {
    assert!(!obj.is_null());
    (*obj).value
}
unsafe extern "C" fn pst_real_SV(mut obj: *mut pst_real) -> *mut u8 {
    let mut value: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut fmt_buf: [i8; 15] = [0; 15];
    assert!(!obj.is_null());
    len = sprintf(
        fmt_buf.as_mut_ptr(),
        b"%.5g\x00" as *const u8 as *const i8,
        (*obj).value,
    );
    value =
        new((len as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(value, fmt_buf.as_mut_ptr());
    value as *mut u8
}
unsafe extern "C" fn pst_real_data_ptr(mut obj: *mut pst_real) -> *mut libc::c_void {
    assert!(!obj.is_null());
    &mut (*obj).value as *mut f64 as *mut libc::c_void
}
unsafe extern "C" fn pst_real_length() -> u32 {
    _tt_abort(b"Operation not defined for this type of object.\x00" as *const u8 as *const i8);
}
/* NOTE: the input buffer must be null-terminated, i.e., *inbufend == 0 */
/* leading white-space is ignored */
#[no_mangle]
pub unsafe extern "C" fn pst_parse_number(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_obj {
    let mut cur: *mut u8 = 0 as *mut u8;
    let mut lval: i32 = 0;
    let mut dval: f64 = 0.;
    *__errno_location() = 0i32;
    lval = strtol(
        *inbuf as *mut i8,
        &mut cur as *mut *mut u8 as *mut libc::c_void as *mut *mut i8,
        10i32,
    ) as i32;
    if *__errno_location() != 0
        || *cur as i32 == '.' as i32
        || *cur as i32 == 'e' as i32
        || *cur as i32 == 'E' as i32
    {
        /* real */
        *__errno_location() = 0i32;
        dval = strtod(
            *inbuf as *mut i8,
            &mut cur as *mut *mut u8 as *mut libc::c_void as *mut *mut i8,
        );
        if *__errno_location() == 0
            && (cur == inbufend
                || (*cur as i32 == '(' as i32
                    || *cur as i32 == ')' as i32
                    || *cur as i32 == '/' as i32
                    || *cur as i32 == '<' as i32
                    || *cur as i32 == '>' as i32
                    || *cur as i32 == '[' as i32
                    || *cur as i32 == ']' as i32
                    || *cur as i32 == '{' as i32
                    || *cur as i32 == '}' as i32
                    || *cur as i32 == '%' as i32)
                || (*cur as i32 == ' ' as i32
                    || *cur as i32 == '\t' as i32
                    || *cur as i32 == '\u{c}' as i32
                    || *cur as i32 == '\r' as i32
                    || *cur as i32 == '\n' as i32
                    || *cur as i32 == '\u{0}' as i32))
        {
            *inbuf = cur;
            return pst_new_obj(3i32, pst_real_new(dval) as *mut libc::c_void);
        }
    } else if cur != *inbuf
        && (cur == inbufend
            || (*cur as i32 == '(' as i32
                || *cur as i32 == ')' as i32
                || *cur as i32 == '/' as i32
                || *cur as i32 == '<' as i32
                || *cur as i32 == '>' as i32
                || *cur as i32 == '[' as i32
                || *cur as i32 == ']' as i32
                || *cur as i32 == '{' as i32
                || *cur as i32 == '}' as i32
                || *cur as i32 == '%' as i32)
            || (*cur as i32 == ' ' as i32
                || *cur as i32 == '\t' as i32
                || *cur as i32 == '\u{c}' as i32
                || *cur as i32 == '\r' as i32
                || *cur as i32 == '\n' as i32
                || *cur as i32 == '\u{0}' as i32))
    {
        /* integer */
        *inbuf = cur;
        return pst_new_obj(2i32, pst_integer_new(lval) as *mut libc::c_void);
    } else {
        if lval >= 2i32
            && lval <= 36i32
            && *cur as i32 == '#' as i32
            && {
                cur = cur.offset(1);
                libc::isalnum(*cur as _) != 0
            }
            && (lval != 16i32
                || *cur.offset(1) as i32 != 'x' as i32 && *cur.offset(1) as i32 != 'X' as i32)
        {
            /* integer with radix */
            /* Can the base have a (plus) sign? I think yes. */
            *__errno_location() = 0i32;
            lval = strtol(
                cur as *mut i8,
                &mut cur as *mut *mut u8 as *mut libc::c_void as *mut *mut i8,
                lval,
            ) as i32;
            if *__errno_location() == 0
                && (cur == inbufend
                    || (*cur as i32 == '(' as i32
                        || *cur as i32 == ')' as i32
                        || *cur as i32 == '/' as i32
                        || *cur as i32 == '<' as i32
                        || *cur as i32 == '>' as i32
                        || *cur as i32 == '[' as i32
                        || *cur as i32 == ']' as i32
                        || *cur as i32 == '{' as i32
                        || *cur as i32 == '}' as i32
                        || *cur as i32 == '%' as i32)
                    || (*cur as i32 == ' ' as i32
                        || *cur as i32 == '\t' as i32
                        || *cur as i32 == '\u{c}' as i32
                        || *cur as i32 == '\r' as i32
                        || *cur as i32 == '\n' as i32
                        || *cur as i32 == '\u{0}' as i32))
            {
                *inbuf = cur;
                return pst_new_obj(2i32, pst_integer_new(lval) as *mut libc::c_void);
            }
        }
    }
    /* error */
    0 as *mut pst_obj
}
/* NAME */
/* NAME */
/*
 * \0 is not allowed for name object.
 */
unsafe extern "C" fn pst_name_new(mut name: *const i8) -> *mut pst_name {
    let mut obj: *mut pst_name = 0 as *mut pst_name;
    obj =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pst_name>() as u64) as u32) as *mut pst_name;
    (*obj).value = new((strlen(name).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*obj).value, name);
    obj
}
unsafe extern "C" fn pst_name_release(mut obj: *mut pst_name) {
    assert!(!obj.is_null());
    free((*obj).value as *mut libc::c_void);
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn getxpair(mut s: *mut *mut u8) -> i32 {
    let mut hi: i32 = 0;
    let mut lo: i32 = 0;
    hi = xtoi(**s as i8);
    if hi < 0i32 {
        return hi;
    }
    *s = (*s).offset(1);
    lo = xtoi(**s as i8);
    if lo < 0i32 {
        return lo;
    }
    *s = (*s).offset(1);
    hi << 4i32 | lo
}
#[no_mangle]
pub unsafe extern "C" fn pst_parse_name(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_obj
/* / is required */ {
    let mut wbuf: [u8; 128] = [0; 128];
    let mut c: u8 = 0;
    let mut p: *mut u8 = wbuf.as_mut_ptr();
    let mut cur: *mut u8 = *inbuf;
    let mut len: i32 = 0i32;
    if *cur as i32 != '/' as i32 {
        return 0 as *mut pst_obj;
    }
    cur = cur.offset(1);
    while !(cur == inbufend
        || (*cur as i32 == '(' as i32
            || *cur as i32 == ')' as i32
            || *cur as i32 == '/' as i32
            || *cur as i32 == '<' as i32
            || *cur as i32 == '>' as i32
            || *cur as i32 == '[' as i32
            || *cur as i32 == ']' as i32
            || *cur as i32 == '{' as i32
            || *cur as i32 == '}' as i32
            || *cur as i32 == '%' as i32)
        || (*cur as i32 == ' ' as i32
            || *cur as i32 == '\t' as i32
            || *cur as i32 == '\u{c}' as i32
            || *cur as i32 == '\r' as i32
            || *cur as i32 == '\n' as i32
            || *cur as i32 == '\u{0}' as i32))
    {
        let fresh0 = cur;
        cur = cur.offset(1);
        c = *fresh0;
        if c as i32 == '#' as i32 {
            let mut val: i32 = 0;
            if cur.offset(2) >= inbufend {
                warn!("Premature end of input name string.");
                break;
            } else {
                val = getxpair(&mut cur);
                if val <= 0i32 {
                    warn!("Invalid char for name object. (ignored)");
                    continue;
                } else {
                    c = val as u8
                }
            }
        }
        if len < 127i32 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = c
        }
        len += 1
    }
    *p = '\u{0}' as i32 as u8;
    if len > 127i32 {
        warn!("String too long for name object. Output will be truncated.");
    }
    *inbuf = cur;
    return pst_new_obj(
        6i32,
        pst_name_new(wbuf.as_mut_ptr() as *mut i8) as *mut libc::c_void,
    );
}
unsafe extern "C" fn pst_name_IV() -> i32 {
    _tt_abort(b"Operation not defined for this type of object.\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn pst_name_RV() -> f64 {
    _tt_abort(b"Operation not defined for this type of object.\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn pst_name_SV(mut obj: *mut pst_name) -> *mut u8 {
    let mut value: *mut i8 = 0 as *mut i8;
    value = new(
        (strlen((*obj).value).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    strcpy(value, (*obj).value);
    value as *mut u8
}
unsafe extern "C" fn pst_name_data_ptr(mut obj: *mut pst_name) -> *mut libc::c_void {
    assert!(!obj.is_null());
    (*obj).value as *mut libc::c_void
}
unsafe extern "C" fn pst_name_length(mut obj: *mut pst_name) -> u32 {
    assert!(!obj.is_null());
    strlen((*obj).value) as u32
}
/* STRING */
/*
 * TODO: ascii85 string <~ .... ~>
 */
unsafe extern "C" fn pst_string_new(mut str: *mut u8, mut len: u32) -> *mut pst_string {
    let mut obj: *mut pst_string = 0 as *mut pst_string;
    obj = new((1_u64).wrapping_mul(::std::mem::size_of::<pst_string>() as u64) as u32)
        as *mut pst_string;
    (*obj).length = len;
    (*obj).value = 0 as *mut u8;
    if len > 0_u32 {
        (*obj).value =
            new((len as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        if !str.is_null() {
            memcpy(
                (*obj).value as *mut libc::c_void,
                str as *const libc::c_void,
                len as u64,
            );
        }
    }
    obj
}
unsafe extern "C" fn pst_string_release(mut obj: *mut pst_string) {
    assert!(!obj.is_null());
    free((*obj).value as *mut libc::c_void);
    free(obj as *mut libc::c_void);
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
pub unsafe extern "C" fn pst_parse_string(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_obj {
    if (*inbuf).offset(2) >= inbufend {
        return 0 as *mut pst_obj;
    } else {
        if **inbuf as i32 == '(' as i32 {
            return pst_new_obj(
                5i32,
                pst_string_parse_literal(inbuf, inbufend) as *mut libc::c_void,
            );
        } else {
            if **inbuf as i32 == '<' as i32 && *(*inbuf).offset(1) as i32 == '~' as i32 {
                _tt_abort(b"ASCII85 string not supported yet.\x00" as *const u8 as *const i8);
            } else {
                if **inbuf as i32 == '<' as i32 {
                    return pst_new_obj(
                        5i32,
                        pst_string_parse_hex(inbuf, inbufend) as *mut libc::c_void,
                    );
                }
            }
        }
    }
    0 as *mut pst_obj
}
/* Overflowed value is set to invalid char.  */
unsafe extern "C" fn ostrtouc(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
    mut valid: *mut u8,
) -> u8 {
    let mut cur: *mut u8 = *inbuf;
    let mut val: u32 = 0_u32;
    while cur < inbufend
        && cur < (*inbuf).offset(3)
        && (*cur as i32 >= '0' as i32 && *cur as i32 <= '7' as i32)
    {
        val = val << 3i32 | (*cur as i32 - '0' as i32) as u32;
        cur = cur.offset(1)
    }
    if val > 255_u32 || cur == *inbuf {
        *valid = 0_u8
    } else {
        *valid = 1_u8
    }
    *inbuf = cur;
    val as u8
}
unsafe extern "C" fn esctouc(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
    mut valid: *mut u8,
) -> u8 {
    let mut unescaped: u8 = 0;
    let mut escaped: u8 = 0;
    escaped = **inbuf;
    *valid = 1_u8;
    match escaped as i32 {
        92 | 41 | 40 => {
            /* Backslash, unbalanced paranthes */
            unescaped = escaped;
            *inbuf = (*inbuf).offset(1)
        }
        110 => {
            /* Other escaped char */
            unescaped = '\n' as i32 as u8;
            *inbuf = (*inbuf).offset(1)
        }
        114 => {
            unescaped = '\r' as i32 as u8;
            *inbuf = (*inbuf).offset(1)
        }
        116 => {
            unescaped = '\t' as i32 as u8;
            *inbuf = (*inbuf).offset(1)
        }
        98 => {
            unescaped = '\u{8}' as i32 as u8;
            *inbuf = (*inbuf).offset(1)
        }
        102 => {
            unescaped = '\u{c}' as i32 as u8;
            *inbuf = (*inbuf).offset(1)
        }
        13 => {
            /*
             * An end-of-line marker preceeded by backslash is not part of a
             * literal string
             */
            unescaped = 0_u8;
            *valid = 0_u8;
            *inbuf = (*inbuf).offset(
                (if *inbuf < inbufend.offset(-1) && *(*inbuf).offset(1) as i32 == '\n' as i32 {
                    2i32
                } else {
                    1i32
                }) as isize,
            )
        }
        10 => {
            unescaped = 0_u8;
            *valid = 0_u8;
            *inbuf = (*inbuf).offset(1)
        }
        _ => {
            /* Possibly octal notion */
            unescaped = ostrtouc(inbuf, inbufend, valid)
        }
    }
    unescaped
}
/* STRING */
unsafe extern "C" fn pst_string_parse_literal(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_string {
    let mut wbuf: [u8; 4096] = [0; 4096];
    let mut cur: *mut u8 = *inbuf;
    let mut c: u8 = 0_u8;
    let mut len: i32 = 0i32;
    let mut balance: i32 = 1i32;
    if cur.offset(2) > inbufend || *cur as i32 != '(' as i32 {
        return 0 as *mut pst_string;
    }
    cur = cur.offset(1);
    while cur < inbufend && len < 4096i32 && balance > 0i32 {
        let fresh2 = cur;
        cur = cur.offset(1);
        c = *fresh2;
        match c as i32 {
            92 => {
                let mut unescaped: u8 = 0;
                let mut valid: u8 = 0;
                unescaped = esctouc(&mut cur, inbufend, &mut valid);
                if valid != 0 {
                    let fresh3 = len;
                    len = len + 1;
                    wbuf[fresh3 as usize] = unescaped
                }
            }
            40 => {
                balance += 1;
                let fresh4 = len;
                len = len + 1;
                wbuf[fresh4 as usize] = '(' as i32 as u8
            }
            41 => {
                balance -= 1;
                if balance > 0i32 {
                    let fresh5 = len;
                    len = len + 1;
                    wbuf[fresh5 as usize] = ')' as i32 as u8
                }
            }
            13 => {
                /*
                 * An end-of-line marker (\n, \r or \r\n), not preceeded by a backslash,
                 * must be converted to single \n.
                 */
                if cur < inbufend && *cur as i32 == '\n' as i32 {
                    cur = cur.offset(1)
                }
                let fresh6 = len;
                len = len + 1;
                wbuf[fresh6 as usize] = '\n' as i32 as u8
            }
            _ => {
                let fresh7 = len;
                len = len + 1;
                wbuf[fresh7 as usize] = c
            }
        }
    }
    if c as i32 != ')' as i32 {
        return 0 as *mut pst_string;
    }
    *inbuf = cur;
    pst_string_new(wbuf.as_mut_ptr(), len as u32)
}
unsafe extern "C" fn pst_string_parse_hex(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_string {
    let mut wbuf: [u8; 4096] = [0; 4096];
    let mut cur: *mut u8 = *inbuf;
    let mut len: u32 = 0_u32;
    if cur.offset(2) > inbufend
        || *cur as i32 != '<' as i32
        || *cur as i32 == '<' as i32 && *cur.offset(1) as i32 == '<' as i32
    {
        return 0 as *mut pst_string;
    }
    cur = cur.offset(1);
    /* PDF Reference does not specify how to treat invalid char */
    while cur < inbufend && len < 4096_u32 {
        let mut hi: i32 = 0;
        let mut lo: i32 = 0;
        skip_white_spaces(&mut cur, inbufend);
        if *cur as i32 == '>' as i32 {
            break;
        }
        let fresh8 = cur;
        cur = cur.offset(1);
        hi = xtoi(*fresh8 as i8);
        if hi < 0i32 {
            warn!(
                "Invalid char for hex string <{:x}> treated as <0>.",
                *cur.offset(-1) as i32,
            );
            hi = 0i32
        }
        skip_white_spaces(&mut cur, inbufend);
        if *cur as i32 == '>' as i32 {
            break;
        }
        /* 0 is appended if final hex digit is missing */
        lo = if cur < inbufend {
            let fresh9 = cur;
            cur = cur.offset(1);
            xtoi(*fresh9 as i8)
        } else {
            0i32
        };
        if lo < 0i32 {
            warn!(
                "Invalid char for hex string <{:x}> treated as <0>.",
                *cur.offset(-1) as i32,
            );
            lo = 0i32
        }
        let fresh10 = len;
        len = len.wrapping_add(1);
        wbuf[fresh10 as usize] = (hi << 4i32 | lo) as u8
    }
    let fresh11 = cur;
    cur = cur.offset(1);
    if *fresh11 as i32 != '>' as i32 {
        return 0 as *mut pst_string;
    }
    *inbuf = cur;
    pst_string_new(wbuf.as_mut_ptr(), len)
}
unsafe extern "C" fn pst_string_IV(mut obj: *mut pst_string) -> i32 {
    pst_string_RV(obj) as i32
}
unsafe extern "C" fn pst_string_RV(mut obj: *mut pst_string) -> f64 {
    let mut nobj: *mut pst_obj = 0 as *mut pst_obj;
    let mut p: *mut u8 = 0 as *mut u8;
    let mut end: *mut u8 = 0 as *mut u8;
    let mut rv: f64 = 0.;
    assert!(!obj.is_null());
    p = (*obj).value;
    end = p.offset((*obj).length as isize);
    nobj = pst_parse_number(&mut p, end);
    if nobj.is_null() || p != end {
        _tt_abort(b"Cound not convert string to real value.\x00" as *const u8 as *const i8);
    }
    rv = pst_getRV(nobj);
    pst_release_obj(nobj);
    rv
}
unsafe extern "C" fn pst_string_SV(mut obj: *mut pst_string) -> *mut u8 {
    let mut str: *mut u8 = 0 as *mut u8;
    assert!(!obj.is_null());
    str = new(((*obj).length.wrapping_add(1_u32) as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    memcpy(
        str as *mut libc::c_void,
        (*obj).value as *const libc::c_void,
        (*obj).length as u64,
    );
    *str.offset((*obj).length as isize) = '\u{0}' as i32 as u8;
    str
}
unsafe extern "C" fn pst_string_data_ptr(mut obj: *mut pst_string) -> *mut libc::c_void {
    assert!(!obj.is_null());
    (*obj).value as *mut libc::c_void
}
unsafe extern "C" fn pst_string_length(mut obj: *mut pst_string) -> u32 {
    assert!(!obj.is_null());
    (*obj).length
}
