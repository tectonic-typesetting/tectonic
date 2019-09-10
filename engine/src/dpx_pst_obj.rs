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
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn xtoi(c: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn skip_white_spaces(s: *mut *mut libc::c_uchar, endptr: *mut libc::c_uchar);
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
pub struct pst_obj {
    pub type_0: pst_type,
    pub data: *mut libc::c_void,
}
pub type pst_type = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_string {
    pub length: libc::c_uint,
    pub value: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_name {
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_real {
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_integer {
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_boolean {
    pub value: libc::c_char,
}
static mut pst_const_null: *const libc::c_char = b"null\x00" as *const u8 as *const libc::c_char;
static mut pst_const_mark: *const libc::c_char = b"mark\x00" as *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn pst_new_obj(
    mut type_0: pst_type,
    mut data: *mut libc::c_void,
) -> *mut pst_obj {
    let mut obj: *mut pst_obj = 0 as *mut pst_obj;
    obj = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pst_obj>() as u64) as u32)
        as *mut pst_obj;
    (*obj).type_0 = type_0;
    (*obj).data = data;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn pst_new_mark() -> *mut pst_obj {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    q = new(
        (strlen(pst_const_mark).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32,
    ) as *mut libc::c_char;
    strcpy(q, pst_const_mark);
    return pst_new_obj(7i32, q as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pst_release_obj(mut obj: *mut pst_obj) {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            138i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void pst_release_obj(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
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
                b"Unrecognized object type: %d\x00" as *const u8 as *const libc::c_char,
                (*obj).type_0,
            );
        }
    }
    free(obj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pst_type_of(mut obj: *mut pst_obj) -> pst_type {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            159i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"pst_type pst_type_of(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn pst_length_of(mut obj: *mut pst_obj) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            168i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"int pst_length_of(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*obj).type_0 {
        1 => len = pst_boolean_length() as libc::c_int,
        2 => len = pst_integer_length() as libc::c_int,
        3 => len = pst_real_length() as libc::c_int,
        6 => len = pst_name_length((*obj).data as *mut pst_name) as libc::c_int,
        5 => len = pst_string_length((*obj).data as *mut pst_string) as libc::c_int,
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -1 => len = strlen((*obj).data as *const libc::c_char) as libc::c_int,
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const libc::c_char,
                (*obj).type_0,
            );
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn pst_getIV(mut obj: *mut pst_obj) -> libc::c_int {
    let mut iv: libc::c_int = 0i32;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            194i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"int pst_getIV(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*obj).type_0 {
        1 => iv = pst_boolean_IV((*obj).data as *mut pst_boolean),
        2 => iv = pst_integer_IV((*obj).data as *mut pst_integer),
        3 => iv = pst_real_IV((*obj).data as *mut pst_real),
        6 => iv = pst_name_IV(),
        5 => iv = pst_string_IV((*obj).data as *mut pst_string),
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -1 => {
            _tt_abort(
                b"Cannot convert object of type UNKNOWN to integer value.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const libc::c_char,
                (*obj).type_0,
            );
        }
    }
    return iv;
}
#[no_mangle]
pub unsafe extern "C" fn pst_getRV(mut obj: *mut pst_obj) -> libc::c_double {
    let mut rv: libc::c_double = 0.0f64;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            220i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"double pst_getRV(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*obj).type_0 {
        1 => rv = pst_boolean_RV((*obj).data as *mut pst_boolean),
        2 => rv = pst_integer_RV((*obj).data as *mut pst_integer),
        3 => rv = pst_real_RV((*obj).data as *mut pst_real),
        6 => rv = pst_name_RV(),
        5 => rv = pst_string_RV((*obj).data as *mut pst_string),
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -1 => {
            _tt_abort(
                b"Cannot convert object of type UNKNOWN to real value.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const libc::c_char,
                (*obj).type_0,
            );
        }
    }
    return rv;
}
/* Length can be obtained by pst_length_of(). */
#[no_mangle]
pub unsafe extern "C" fn pst_getSV(mut obj: *mut pst_obj) -> *mut libc::c_uchar {
    let mut sv: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            247i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"unsigned char *pst_getSV(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*obj).type_0 {
        1 => sv = pst_boolean_SV((*obj).data as *mut pst_boolean),
        2 => sv = pst_integer_SV((*obj).data as *mut pst_integer),
        3 => sv = pst_real_SV((*obj).data as *mut pst_real),
        6 => sv = pst_name_SV((*obj).data as *mut pst_name),
        5 => sv = pst_string_SV((*obj).data as *mut pst_string),
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -1 => {
            let mut len: libc::c_int = 0;
            len = strlen((*obj).data as *mut libc::c_char) as libc::c_int;
            if len > 0i32 {
                sv = new(((len + 1i32) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
                    as u32) as *mut libc::c_uchar;
                memcpy(sv as *mut libc::c_void, (*obj).data, len as u64);
                *sv.offset(len as isize) = '\u{0}' as i32 as libc::c_uchar
            } else {
                sv = 0 as *mut libc::c_uchar
            }
        }
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const libc::c_char,
                (*obj).type_0,
            );
        }
    }
    return sv;
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
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            284i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void *pst_data_ptr(pst_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*obj).type_0 {
        1 => p = pst_boolean_data_ptr((*obj).data as *mut pst_boolean) as *mut libc::c_char,
        2 => p = pst_integer_data_ptr((*obj).data as *mut pst_integer) as *mut libc::c_char,
        3 => p = pst_real_data_ptr((*obj).data as *mut pst_real) as *mut libc::c_char,
        6 => p = pst_name_data_ptr((*obj).data as *mut pst_name) as *mut libc::c_char,
        5 => p = pst_string_data_ptr((*obj).data as *mut pst_string) as *mut libc::c_char,
        0 | 7 => {
            _tt_abort(
                b"Operation not defined for this type of object.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -1 => p = (*obj).data as *mut libc::c_char,
        _ => {
            _tt_abort(
                b"Unrecognized object type: %d\x00" as *const u8 as *const libc::c_char,
                (*obj).type_0,
            );
        }
    }
    return p as *mut libc::c_void;
}
/* BOOLEAN */
/* BOOLEAN */
unsafe extern "C" fn pst_boolean_new(mut value: libc::c_char) -> *mut pst_boolean {
    let mut obj: *mut pst_boolean = 0 as *mut pst_boolean;
    obj = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pst_boolean>() as u64)
        as u32) as *mut pst_boolean;
    (*obj).value = value;
    return obj;
}
unsafe extern "C" fn pst_boolean_release(mut obj: *mut pst_boolean) {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            318i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void pst_boolean_release(pst_boolean *)\x00",
            ))
            .as_ptr(),
        );
    }
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn pst_boolean_IV(mut obj: *mut pst_boolean) -> libc::c_int {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            325i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"int pst_boolean_IV(pst_boolean *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value as libc::c_int;
}
unsafe extern "C" fn pst_boolean_RV(mut obj: *mut pst_boolean) -> libc::c_double {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            332i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"double pst_boolean_RV(pst_boolean *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value as libc::c_double;
}
unsafe extern "C" fn pst_boolean_SV(mut obj: *mut pst_boolean) -> *mut libc::c_uchar {
    let mut str: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            341i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"unsigned char *pst_boolean_SV(pst_boolean *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*obj).value != 0 {
        str = new((5i32 as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
            as u32) as *mut libc::c_uchar;
        memcpy(
            str as *mut libc::c_void,
            b"true\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4i32 as u64,
        );
        *str.offset(4) = '\u{0}' as i32 as libc::c_uchar
    } else {
        str = new((6i32 as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
            as u32) as *mut libc::c_uchar;
        memcpy(
            str as *mut libc::c_void,
            b"false\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            5i32 as u64,
        );
        *str.offset(5) = '\u{0}' as i32 as libc::c_uchar
    }
    return str;
}
unsafe extern "C" fn pst_boolean_length() -> libc::c_uint {
    _tt_abort(
        b"Operation not defined for this type of object.\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn pst_boolean_data_ptr(mut obj: *mut pst_boolean) -> *mut libc::c_void {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            366i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void *pst_boolean_data_ptr(pst_boolean *)\x00",
            ))
            .as_ptr(),
        );
    }
    return &mut (*obj).value as *mut libc::c_char as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn pst_parse_boolean(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_obj {
    if (*inbuf).offset(4) <= inbufend
        && memcmp(
            *inbuf as *const libc::c_void,
            b"true\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4i32 as u64,
        ) == 0i32
        && ((*inbuf).offset(4) == inbufend
            || (*(*inbuf).offset(4) as libc::c_int == '(' as i32
                || *(*inbuf).offset(4) as libc::c_int == ')' as i32
                || *(*inbuf).offset(4) as libc::c_int == '/' as i32
                || *(*inbuf).offset(4) as libc::c_int == '<' as i32
                || *(*inbuf).offset(4) as libc::c_int == '>' as i32
                || *(*inbuf).offset(4) as libc::c_int == '[' as i32
                || *(*inbuf).offset(4) as libc::c_int == ']' as i32
                || *(*inbuf).offset(4) as libc::c_int == '{' as i32
                || *(*inbuf).offset(4) as libc::c_int == '}' as i32
                || *(*inbuf).offset(4) as libc::c_int == '%' as i32)
            || (*(*inbuf).offset(4) as libc::c_int == ' ' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\t' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\u{c}' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\r' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\n' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\u{0}' as i32))
    {
        *inbuf = (*inbuf).offset(4);
        return pst_new_obj(
            1i32,
            pst_boolean_new(1i32 as libc::c_char) as *mut libc::c_void,
        );
    } else if (*inbuf).offset(5) <= inbufend
        && memcmp(
            *inbuf as *const libc::c_void,
            b"false\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            5i32 as u64,
        ) == 0i32
        && ((*inbuf).offset(5) == inbufend
            || (*(*inbuf).offset(5) as libc::c_int == '(' as i32
                || *(*inbuf).offset(5) as libc::c_int == ')' as i32
                || *(*inbuf).offset(5) as libc::c_int == '/' as i32
                || *(*inbuf).offset(5) as libc::c_int == '<' as i32
                || *(*inbuf).offset(5) as libc::c_int == '>' as i32
                || *(*inbuf).offset(5) as libc::c_int == '[' as i32
                || *(*inbuf).offset(5) as libc::c_int == ']' as i32
                || *(*inbuf).offset(5) as libc::c_int == '{' as i32
                || *(*inbuf).offset(5) as libc::c_int == '}' as i32
                || *(*inbuf).offset(5) as libc::c_int == '%' as i32)
            || (*(*inbuf).offset(5) as libc::c_int == ' ' as i32
                || *(*inbuf).offset(5) as libc::c_int == '\t' as i32
                || *(*inbuf).offset(5) as libc::c_int == '\u{c}' as i32
                || *(*inbuf).offset(5) as libc::c_int == '\r' as i32
                || *(*inbuf).offset(5) as libc::c_int == '\n' as i32
                || *(*inbuf).offset(5) as libc::c_int == '\u{0}' as i32))
    {
        *inbuf = (*inbuf).offset(5);
        return pst_new_obj(
            1i32,
            pst_boolean_new(0i32 as libc::c_char) as *mut libc::c_void,
        );
    } else {
        return 0 as *mut pst_obj;
    };
}
/* NULL */
#[no_mangle]
pub unsafe extern "C" fn pst_parse_null(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_obj {
    if (*inbuf).offset(4) <= inbufend
        && memcmp(
            *inbuf as *const libc::c_void,
            b"null\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4i32 as u64,
        ) == 0i32
        && ((*inbuf).offset(4) == inbufend
            || (*(*inbuf).offset(4) as libc::c_int == '(' as i32
                || *(*inbuf).offset(4) as libc::c_int == ')' as i32
                || *(*inbuf).offset(4) as libc::c_int == '/' as i32
                || *(*inbuf).offset(4) as libc::c_int == '<' as i32
                || *(*inbuf).offset(4) as libc::c_int == '>' as i32
                || *(*inbuf).offset(4) as libc::c_int == '[' as i32
                || *(*inbuf).offset(4) as libc::c_int == ']' as i32
                || *(*inbuf).offset(4) as libc::c_int == '{' as i32
                || *(*inbuf).offset(4) as libc::c_int == '}' as i32
                || *(*inbuf).offset(4) as libc::c_int == '%' as i32)
            || (*(*inbuf).offset(4) as libc::c_int == ' ' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\t' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\u{c}' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\r' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\n' as i32
                || *(*inbuf).offset(4) as libc::c_int == '\u{0}' as i32))
    {
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        *inbuf = (*inbuf).offset(4);
        q = new(
            (strlen(pst_const_null).wrapping_add(1i32 as u64) as u32
                as u64)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
                as u32,
        ) as *mut libc::c_char;
        strcpy(q, pst_const_null);
        return pst_new_obj(0i32, q as *mut libc::c_void);
    } else {
        return 0 as *mut pst_obj;
    };
}
/* NUMBERS */
/* INTEGER */
unsafe extern "C" fn pst_integer_new(mut value: libc::c_int) -> *mut pst_integer {
    let mut obj: *mut pst_integer = 0 as *mut pst_integer;
    obj = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pst_integer>() as u64)
        as u32) as *mut pst_integer;
    (*obj).value = value;
    return obj;
}
unsafe extern "C" fn pst_integer_release(mut obj: *mut pst_integer) {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            418i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void pst_integer_release(pst_integer *)\x00",
            ))
            .as_ptr(),
        );
    }
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn pst_integer_IV(mut obj: *mut pst_integer) -> libc::c_int {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            425i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"int pst_integer_IV(pst_integer *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value;
}
unsafe extern "C" fn pst_integer_RV(mut obj: *mut pst_integer) -> libc::c_double {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            432i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"double pst_integer_RV(pst_integer *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value as libc::c_double;
}
unsafe extern "C" fn pst_integer_SV(mut obj: *mut pst_integer) -> *mut libc::c_uchar {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut fmt_buf: [libc::c_char; 15] = [0; 15];
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            443i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"unsigned char *pst_integer_SV(pst_integer *)\x00",
            ))
            .as_ptr(),
        );
    }
    len = sprintf(
        fmt_buf.as_mut_ptr(),
        b"%d\x00" as *const u8 as *const libc::c_char,
        (*obj).value,
    );
    value = new(((len + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    strcpy(value, fmt_buf.as_mut_ptr());
    return value as *mut libc::c_uchar;
}
unsafe extern "C" fn pst_integer_data_ptr(mut obj: *mut pst_integer) -> *mut libc::c_void {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            456i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void *pst_integer_data_ptr(pst_integer *)\x00",
            ))
            .as_ptr(),
        );
    }
    return &mut (*obj).value as *mut libc::c_int as *mut libc::c_void;
}
unsafe extern "C" fn pst_integer_length() -> libc::c_uint {
    _tt_abort(
        b"Operation not defined for this type of object.\x00" as *const u8 as *const libc::c_char,
    );
}
/* REAL */
unsafe extern "C" fn pst_real_new(mut value: libc::c_double) -> *mut pst_real {
    let mut obj: *mut pst_real = 0 as *mut pst_real;
    obj = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pst_real>() as u64) as u32)
        as *mut pst_real;
    (*obj).value = value;
    return obj;
}
unsafe extern "C" fn pst_real_release(mut obj: *mut pst_real) {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            482i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void pst_real_release(pst_real *)\x00",
            ))
            .as_ptr(),
        );
    }
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn pst_real_IV(mut obj: *mut pst_real) -> libc::c_int {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            489i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"int pst_real_IV(pst_real *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value as libc::c_int;
}
unsafe extern "C" fn pst_real_RV(mut obj: *mut pst_real) -> libc::c_double {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            496i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"double pst_real_RV(pst_real *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value;
}
unsafe extern "C" fn pst_real_SV(mut obj: *mut pst_real) -> *mut libc::c_uchar {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut fmt_buf: [libc::c_char; 15] = [0; 15];
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            507i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"unsigned char *pst_real_SV(pst_real *)\x00",
            ))
            .as_ptr(),
        );
    }
    len = sprintf(
        fmt_buf.as_mut_ptr(),
        b"%.5g\x00" as *const u8 as *const libc::c_char,
        (*obj).value,
    );
    value = new((len as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    strcpy(value, fmt_buf.as_mut_ptr());
    return value as *mut libc::c_uchar;
}
unsafe extern "C" fn pst_real_data_ptr(mut obj: *mut pst_real) -> *mut libc::c_void {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            520i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"void *pst_real_data_ptr(pst_real *)\x00",
            ))
            .as_ptr(),
        );
    }
    return &mut (*obj).value as *mut libc::c_double as *mut libc::c_void;
}
unsafe extern "C" fn pst_real_length() -> libc::c_uint {
    _tt_abort(
        b"Operation not defined for this type of object.\x00" as *const u8 as *const libc::c_char,
    );
}
/* NOTE: the input buffer must be null-terminated, i.e., *inbufend == 0 */
/* leading white-space is ignored */
#[no_mangle]
pub unsafe extern "C" fn pst_parse_number(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_obj {
    let mut cur: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut lval: libc::c_int = 0;
    let mut dval: libc::c_double = 0.;
    *__errno_location() = 0i32;
    lval = strtol(
        *inbuf as *mut libc::c_char,
        &mut cur as *mut *mut libc::c_uchar as *mut libc::c_void as *mut *mut libc::c_char,
        10i32,
    ) as libc::c_int;
    if *__errno_location() != 0
        || *cur as libc::c_int == '.' as i32
        || *cur as libc::c_int == 'e' as i32
        || *cur as libc::c_int == 'E' as i32
    {
        /* real */
        *__errno_location() = 0i32;
        dval = strtod(
            *inbuf as *mut libc::c_char,
            &mut cur as *mut *mut libc::c_uchar as *mut libc::c_void as *mut *mut libc::c_char,
        );
        if *__errno_location() == 0
            && (cur == inbufend
                || (*cur as libc::c_int == '(' as i32
                    || *cur as libc::c_int == ')' as i32
                    || *cur as libc::c_int == '/' as i32
                    || *cur as libc::c_int == '<' as i32
                    || *cur as libc::c_int == '>' as i32
                    || *cur as libc::c_int == '[' as i32
                    || *cur as libc::c_int == ']' as i32
                    || *cur as libc::c_int == '{' as i32
                    || *cur as libc::c_int == '}' as i32
                    || *cur as libc::c_int == '%' as i32)
                || (*cur as libc::c_int == ' ' as i32
                    || *cur as libc::c_int == '\t' as i32
                    || *cur as libc::c_int == '\u{c}' as i32
                    || *cur as libc::c_int == '\r' as i32
                    || *cur as libc::c_int == '\n' as i32
                    || *cur as libc::c_int == '\u{0}' as i32))
        {
            *inbuf = cur;
            return pst_new_obj(3i32, pst_real_new(dval) as *mut libc::c_void);
        }
    } else if cur != *inbuf
        && (cur == inbufend
            || (*cur as libc::c_int == '(' as i32
                || *cur as libc::c_int == ')' as i32
                || *cur as libc::c_int == '/' as i32
                || *cur as libc::c_int == '<' as i32
                || *cur as libc::c_int == '>' as i32
                || *cur as libc::c_int == '[' as i32
                || *cur as libc::c_int == ']' as i32
                || *cur as libc::c_int == '{' as i32
                || *cur as libc::c_int == '}' as i32
                || *cur as libc::c_int == '%' as i32)
            || (*cur as libc::c_int == ' ' as i32
                || *cur as libc::c_int == '\t' as i32
                || *cur as libc::c_int == '\u{c}' as i32
                || *cur as libc::c_int == '\r' as i32
                || *cur as libc::c_int == '\n' as i32
                || *cur as libc::c_int == '\u{0}' as i32))
    {
        /* integer */
        *inbuf = cur;
        return pst_new_obj(2i32, pst_integer_new(lval) as *mut libc::c_void);
    } else {
        if lval >= 2i32
            && lval <= 36i32
            && *cur as libc::c_int == '#' as i32
            && {
                cur = cur.offset(1);
                *(*__ctype_b_loc()).offset(*cur as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            }
            && (lval != 16i32
                || *cur.offset(1) as libc::c_int != 'x' as i32
                    && *cur.offset(1) as libc::c_int != 'X' as i32)
        {
            /* integer with radix */
            /* Can the base have a (plus) sign? I think yes. */
            *__errno_location() = 0i32;
            lval = strtol(
                cur as *mut libc::c_char,
                &mut cur as *mut *mut libc::c_uchar as *mut libc::c_void as *mut *mut libc::c_char,
                lval,
            ) as libc::c_int;
            if *__errno_location() == 0
                && (cur == inbufend
                    || (*cur as libc::c_int == '(' as i32
                        || *cur as libc::c_int == ')' as i32
                        || *cur as libc::c_int == '/' as i32
                        || *cur as libc::c_int == '<' as i32
                        || *cur as libc::c_int == '>' as i32
                        || *cur as libc::c_int == '[' as i32
                        || *cur as libc::c_int == ']' as i32
                        || *cur as libc::c_int == '{' as i32
                        || *cur as libc::c_int == '}' as i32
                        || *cur as libc::c_int == '%' as i32)
                    || (*cur as libc::c_int == ' ' as i32
                        || *cur as libc::c_int == '\t' as i32
                        || *cur as libc::c_int == '\u{c}' as i32
                        || *cur as libc::c_int == '\r' as i32
                        || *cur as libc::c_int == '\n' as i32
                        || *cur as libc::c_int == '\u{0}' as i32))
            {
                *inbuf = cur;
                return pst_new_obj(2i32, pst_integer_new(lval) as *mut libc::c_void);
            }
        }
    }
    /* error */
    return 0 as *mut pst_obj;
}
/* NAME */
/* NAME */
/*
 * \0 is not allowed for name object.
 */
unsafe extern "C" fn pst_name_new(mut name: *const libc::c_char) -> *mut pst_name {
    let mut obj: *mut pst_name = 0 as *mut pst_name;
    obj = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pst_name>() as u64) as u32)
        as *mut pst_name;
    (*obj).value = new(
        (strlen(name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32,
    ) as *mut libc::c_char;
    strcpy((*obj).value, name);
    return obj;
}
unsafe extern "C" fn pst_name_release(mut obj: *mut pst_name) {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            592i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void pst_name_release(pst_name *)\x00",
            ))
            .as_ptr(),
        );
    }
    free((*obj).value as *mut libc::c_void);
    free(obj as *mut libc::c_void);
}
unsafe extern "C" fn getxpair(mut s: *mut *mut libc::c_uchar) -> libc::c_int {
    let mut hi: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    hi = xtoi(**s as libc::c_char);
    if hi < 0i32 {
        return hi;
    }
    *s = (*s).offset(1);
    lo = xtoi(**s as libc::c_char);
    if lo < 0i32 {
        return lo;
    }
    *s = (*s).offset(1);
    return hi << 4i32 | lo;
}
#[no_mangle]
pub unsafe extern "C" fn pst_parse_name(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_obj
/* / is required */ {
    let mut wbuf: [libc::c_uchar; 128] = [0; 128];
    let mut c: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = wbuf.as_mut_ptr();
    let mut cur: *mut libc::c_uchar = *inbuf;
    let mut len: libc::c_int = 0i32;
    if *cur as libc::c_int != '/' as i32 {
        return 0 as *mut pst_obj;
    }
    cur = cur.offset(1);
    while !(cur == inbufend
        || (*cur as libc::c_int == '(' as i32
            || *cur as libc::c_int == ')' as i32
            || *cur as libc::c_int == '/' as i32
            || *cur as libc::c_int == '<' as i32
            || *cur as libc::c_int == '>' as i32
            || *cur as libc::c_int == '[' as i32
            || *cur as libc::c_int == ']' as i32
            || *cur as libc::c_int == '{' as i32
            || *cur as libc::c_int == '}' as i32
            || *cur as libc::c_int == '%' as i32)
        || (*cur as libc::c_int == ' ' as i32
            || *cur as libc::c_int == '\t' as i32
            || *cur as libc::c_int == '\u{c}' as i32
            || *cur as libc::c_int == '\r' as i32
            || *cur as libc::c_int == '\n' as i32
            || *cur as libc::c_int == '\u{0}' as i32))
    {
        let fresh0 = cur;
        cur = cur.offset(1);
        c = *fresh0;
        if c as libc::c_int == '#' as i32 {
            let mut val: libc::c_int = 0;
            if cur.offset(2) >= inbufend {
                dpx_warning(
                    b"Premature end of input name string.\x00" as *const u8 as *const libc::c_char,
                );
                break;
            } else {
                val = getxpair(&mut cur);
                if val <= 0i32 {
                    dpx_warning(
                        b"Invalid char for name object. (ignored)\x00" as *const u8
                            as *const libc::c_char,
                    );
                    continue;
                } else {
                    c = val as libc::c_uchar
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
    *p = '\u{0}' as i32 as libc::c_uchar;
    if len > 127i32 {
        dpx_warning(
            b"String too long for name object. Output will be truncated.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    *inbuf = cur;
    return pst_new_obj(
        6i32,
        pst_name_new(wbuf.as_mut_ptr() as *mut libc::c_char) as *mut libc::c_void,
    );
}
unsafe extern "C" fn pst_name_IV() -> libc::c_int {
    _tt_abort(
        b"Operation not defined for this type of object.\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn pst_name_RV() -> libc::c_double {
    _tt_abort(
        b"Operation not defined for this type of object.\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn pst_name_SV(mut obj: *mut pst_name) -> *mut libc::c_uchar {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = new(
        (strlen((*obj).value).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32,
    ) as *mut libc::c_char;
    strcpy(value, (*obj).value);
    return value as *mut libc::c_uchar;
}
unsafe extern "C" fn pst_name_data_ptr(mut obj: *mut pst_name) -> *mut libc::c_void {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            679i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"void *pst_name_data_ptr(pst_name *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value as *mut libc::c_void;
}
unsafe extern "C" fn pst_name_length(mut obj: *mut pst_name) -> libc::c_uint {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            686i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"unsigned int pst_name_length(pst_name *)\x00",
            ))
            .as_ptr(),
        );
    }
    return strlen((*obj).value) as libc::c_uint;
}
/* STRING */
/*
 * TODO: ascii85 string <~ .... ~>
 */
unsafe extern "C" fn pst_string_new(
    mut str: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> *mut pst_string {
    let mut obj: *mut pst_string = 0 as *mut pst_string;
    obj = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pst_string>() as u64)
        as u32) as *mut pst_string;
    (*obj).length = len;
    (*obj).value = 0 as *mut libc::c_uchar;
    if len > 0i32 as libc::c_uint {
        (*obj).value = new((len as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
            as u32) as *mut libc::c_uchar;
        if !str.is_null() {
            memcpy(
                (*obj).value as *mut libc::c_void,
                str as *const libc::c_void,
                len as u64,
            );
        }
    }
    return obj;
}
unsafe extern "C" fn pst_string_release(mut obj: *mut pst_string) {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            714i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void pst_string_release(pst_string *)\x00",
            ))
            .as_ptr(),
        );
    }
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
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_obj {
    if (*inbuf).offset(2) >= inbufend {
        return 0 as *mut pst_obj;
    } else {
        if **inbuf as libc::c_int == '(' as i32 {
            return pst_new_obj(
                5i32,
                pst_string_parse_literal(inbuf, inbufend) as *mut libc::c_void,
            );
        } else {
            if **inbuf as libc::c_int == '<' as i32
                && *(*inbuf).offset(1) as libc::c_int == '~' as i32
            {
                _tt_abort(
                    b"ASCII85 string not supported yet.\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                if **inbuf as libc::c_int == '<' as i32 {
                    return pst_new_obj(
                        5i32,
                        pst_string_parse_hex(inbuf, inbufend) as *mut libc::c_void,
                    );
                }
            }
        }
    }
    return 0 as *mut pst_obj;
}
/* Overflowed value is set to invalid char.  */
unsafe extern "C" fn ostrtouc(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
    mut valid: *mut libc::c_uchar,
) -> libc::c_uchar {
    let mut cur: *mut libc::c_uchar = *inbuf;
    let mut val: libc::c_uint = 0i32 as libc::c_uint;
    while cur < inbufend
        && cur < (*inbuf).offset(3)
        && (*cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '7' as i32)
    {
        val = val << 3i32 | (*cur as libc::c_int - '0' as i32) as libc::c_uint;
        cur = cur.offset(1)
    }
    if val > 255i32 as libc::c_uint || cur == *inbuf {
        *valid = 0i32 as libc::c_uchar
    } else {
        *valid = 1i32 as libc::c_uchar
    }
    *inbuf = cur;
    return val as libc::c_uchar;
}
unsafe extern "C" fn esctouc(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
    mut valid: *mut libc::c_uchar,
) -> libc::c_uchar {
    let mut unescaped: libc::c_uchar = 0;
    let mut escaped: libc::c_uchar = 0;
    escaped = **inbuf;
    *valid = 1i32 as libc::c_uchar;
    match escaped as libc::c_int {
        92 | 41 | 40 => {
            /* Backslash, unbalanced paranthes */
            unescaped = escaped;
            *inbuf = (*inbuf).offset(1)
        }
        110 => {
            /* Other escaped char */
            unescaped = '\n' as i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(1)
        }
        114 => {
            unescaped = '\r' as i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(1)
        }
        116 => {
            unescaped = '\t' as i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(1)
        }
        98 => {
            unescaped = '\u{8}' as i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(1)
        }
        102 => {
            unescaped = '\u{c}' as i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(1)
        }
        13 => {
            /*
             * An end-of-line marker preceeded by backslash is not part of a
             * literal string
             */
            unescaped = 0i32 as libc::c_uchar;
            *valid = 0i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(
                (if *inbuf < inbufend.offset(-1)
                    && *(*inbuf).offset(1) as libc::c_int == '\n' as i32
                {
                    2i32
                } else {
                    1i32
                }) as isize,
            )
        }
        10 => {
            unescaped = 0i32 as libc::c_uchar;
            *valid = 0i32 as libc::c_uchar;
            *inbuf = (*inbuf).offset(1)
        }
        _ => {
            /* Possibly octal notion */
            unescaped = ostrtouc(inbuf, inbufend, valid)
        }
    }
    return unescaped;
}
/* STRING */
unsafe extern "C" fn pst_string_parse_literal(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_string {
    let mut wbuf: [libc::c_uchar; 4096] = [0; 4096];
    let mut cur: *mut libc::c_uchar = *inbuf;
    let mut c: libc::c_uchar = 0i32 as libc::c_uchar;
    let mut len: libc::c_int = 0i32;
    let mut balance: libc::c_int = 1i32;
    if cur.offset(2) > inbufend || *cur as libc::c_int != '(' as i32 {
        return 0 as *mut pst_string;
    }
    cur = cur.offset(1);
    while cur < inbufend && len < 4096i32 && balance > 0i32 {
        let fresh2 = cur;
        cur = cur.offset(1);
        c = *fresh2;
        match c as libc::c_int {
            92 => {
                let mut unescaped: libc::c_uchar = 0;
                let mut valid: libc::c_uchar = 0;
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
                wbuf[fresh4 as usize] = '(' as i32 as libc::c_uchar
            }
            41 => {
                balance -= 1;
                if balance > 0i32 {
                    let fresh5 = len;
                    len = len + 1;
                    wbuf[fresh5 as usize] = ')' as i32 as libc::c_uchar
                }
            }
            13 => {
                /*
                 * An end-of-line marker (\n, \r or \r\n), not preceeded by a backslash,
                 * must be converted to single \n.
                 */
                if cur < inbufend && *cur as libc::c_int == '\n' as i32 {
                    cur = cur.offset(1)
                }
                let fresh6 = len;
                len = len + 1;
                wbuf[fresh6 as usize] = '\n' as i32 as libc::c_uchar
            }
            _ => {
                let fresh7 = len;
                len = len + 1;
                wbuf[fresh7 as usize] = c
            }
        }
    }
    if c as libc::c_int != ')' as i32 {
        return 0 as *mut pst_string;
    }
    *inbuf = cur;
    return pst_string_new(wbuf.as_mut_ptr(), len as libc::c_uint);
}
unsafe extern "C" fn pst_string_parse_hex(
    mut inbuf: *mut *mut libc::c_uchar,
    mut inbufend: *mut libc::c_uchar,
) -> *mut pst_string {
    let mut wbuf: [libc::c_uchar; 4096] = [0; 4096];
    let mut cur: *mut libc::c_uchar = *inbuf;
    let mut len: libc::c_uint = 0i32 as libc::c_uint;
    if cur.offset(2) > inbufend
        || *cur as libc::c_int != '<' as i32
        || *cur as libc::c_int == '<' as i32 && *cur.offset(1) as libc::c_int == '<' as i32
    {
        return 0 as *mut pst_string;
    }
    cur = cur.offset(1);
    /* PDF Reference does not specify how to treat invalid char */
    while cur < inbufend && len < 4096i32 as libc::c_uint {
        let mut hi: libc::c_int = 0;
        let mut lo: libc::c_int = 0;
        skip_white_spaces(&mut cur, inbufend);
        if *cur as libc::c_int == '>' as i32 {
            break;
        }
        let fresh8 = cur;
        cur = cur.offset(1);
        hi = xtoi(*fresh8 as libc::c_char);
        if hi < 0i32 {
            dpx_warning(
                b"Invalid char for hex string <%x> treated as <0>.\x00" as *const u8
                    as *const libc::c_char,
                *cur.offset(-1) as libc::c_int,
            );
            hi = 0i32
        }
        skip_white_spaces(&mut cur, inbufend);
        if *cur as libc::c_int == '>' as i32 {
            break;
        }
        /* 0 is appended if final hex digit is missing */
        lo = if cur < inbufend {
            let fresh9 = cur;
            cur = cur.offset(1);
            xtoi(*fresh9 as libc::c_char)
        } else {
            0i32
        };
        if lo < 0i32 {
            dpx_warning(
                b"Invalid char for hex string <%x> treated as <0>.\x00" as *const u8
                    as *const libc::c_char,
                *cur.offset(-1) as libc::c_int,
            );
            lo = 0i32
        }
        let fresh10 = len;
        len = len.wrapping_add(1);
        wbuf[fresh10 as usize] = (hi << 4i32 | lo) as libc::c_uchar
    }
    let fresh11 = cur;
    cur = cur.offset(1);
    if *fresh11 as libc::c_int != '>' as i32 {
        return 0 as *mut pst_string;
    }
    *inbuf = cur;
    return pst_string_new(wbuf.as_mut_ptr(), len);
}
unsafe extern "C" fn pst_string_IV(mut obj: *mut pst_string) -> libc::c_int {
    return pst_string_RV(obj) as libc::c_int;
}
unsafe extern "C" fn pst_string_RV(mut obj: *mut pst_string) -> libc::c_double {
    let mut nobj: *mut pst_obj = 0 as *mut pst_obj;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rv: libc::c_double = 0.;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            900i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"double pst_string_RV(pst_string *)\x00",
            ))
            .as_ptr(),
        );
    }
    p = (*obj).value;
    end = p.offset((*obj).length as isize);
    nobj = pst_parse_number(&mut p, end);
    if nobj.is_null() || p != end {
        _tt_abort(
            b"Cound not convert string to real value.\x00" as *const u8 as *const libc::c_char,
        );
    }
    rv = pst_getRV(nobj);
    pst_release_obj(nobj);
    return rv;
}
unsafe extern "C" fn pst_string_SV(mut obj: *mut pst_string) -> *mut libc::c_uchar {
    let mut str: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            916i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"unsigned char *pst_string_SV(pst_string *)\x00",
            ))
            .as_ptr(),
        );
    }
    str = new(
        ((*obj).length.wrapping_add(1i32 as libc::c_uint) as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
            as u32,
    ) as *mut libc::c_uchar;
    memcpy(
        str as *mut libc::c_void,
        (*obj).value as *const libc::c_void,
        (*obj).length as u64,
    );
    *str.offset((*obj).length as isize) = '\u{0}' as i32 as libc::c_uchar;
    return str;
}
unsafe extern "C" fn pst_string_data_ptr(mut obj: *mut pst_string) -> *mut libc::c_void {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            926i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void *pst_string_data_ptr(pst_string *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).value as *mut libc::c_void;
}
unsafe extern "C" fn pst_string_length(mut obj: *mut pst_string) -> libc::c_uint {
    if !obj.is_null() {
    } else {
        __assert_fail(
            b"obj\x00" as *const u8 as *const libc::c_char,
            b"dpx-pst_obj.c\x00" as *const u8 as *const libc::c_char,
            933i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"unsigned int pst_string_length(pst_string *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*obj).length;
}
