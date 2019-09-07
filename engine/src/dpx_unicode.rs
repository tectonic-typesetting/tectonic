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
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn UC_is_valid(mut ucv: int32_t) -> bool {
    return !(ucv < 0i32
        || ucv as libc::c_long > 0x10ffff
        || ucv as libc::c_long >= 0xd800 && ucv as libc::c_long <= 0xdfff);
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF16BE_is_valid_string(
    mut p: *const libc::c_uchar,
    mut endptr: *const libc::c_uchar,
) -> bool {
    if p.offset(1) >= endptr {
        return 0i32 != 0;
    }
    while p < endptr {
        let mut ucv: int32_t = UC_UTF16BE_decode_char(&mut p, endptr);
        if !UC_is_valid(ucv) {
            return 0i32 != 0;
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF8_is_valid_string(
    mut p: *const libc::c_uchar,
    mut endptr: *const libc::c_uchar,
) -> bool {
    if p.offset(1) >= endptr {
        return 0i32 != 0;
    }
    while p < endptr {
        let mut ucv: int32_t = UC_UTF8_decode_char(&mut p, endptr);
        if !UC_is_valid(ucv) {
            return 0i32 != 0;
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF16BE_decode_char(
    mut pp: *mut *const libc::c_uchar,
    mut endptr: *const libc::c_uchar,
) -> int32_t {
    let mut p: *const libc::c_uchar = *pp;
    let mut ucv: int32_t = -1i32;
    let mut first: uint16_t = 0;
    let mut second: uint16_t = 0;
    if p.offset(1) >= endptr {
        return -1i32;
    }
    first = ((*p.offset(0) as libc::c_int) << 8i32 | *p.offset(1) as libc::c_int) as uint16_t;
    p = p.offset(2);
    if first as libc::c_uint >= 0xd800u32 && (first as libc::c_uint) < 0xdc00u32 {
        if p.offset(1) >= endptr {
            return -1i32;
        }
        second = ((*p.offset(0) as libc::c_int) << 8i32 | *p.offset(1) as libc::c_int) as uint16_t;
        p = p.offset(2);
        ucv = (second as libc::c_uint & 0x3ffu32) as int32_t;
        ucv = (ucv as libc::c_uint | (first as libc::c_uint & 0x3ffu32) << 10i32) as int32_t;
        ucv += 0x10000i32
    } else if first as libc::c_uint >= 0xdc00u32 && (first as libc::c_uint) < 0xe000u32 {
        return -1i32;
    } else {
        ucv = first as int32_t
    }
    *pp = p;
    return ucv;
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF16BE_encode_char(
    mut ucv: int32_t,
    mut pp: *mut *mut libc::c_uchar,
    mut endptr: *mut libc::c_uchar,
) -> size_t {
    let mut count: libc::c_int = 0i32;
    let mut p: *mut libc::c_uchar = *pp;
    if ucv >= 0i32 && ucv <= 0xffffi32 {
        if p.offset(2) >= endptr {
            return 0i32 as size_t;
        }
        *p.offset(0) = (ucv >> 8i32 & 0xffi32) as libc::c_uchar;
        *p.offset(1) = (ucv & 0xffi32) as libc::c_uchar;
        count = 2i32
    } else if ucv >= 0x10000i32 && ucv <= 0x10ffffi32 {
        let mut high: libc::c_ushort = 0;
        let mut low: libc::c_ushort = 0;
        if p.offset(4) >= endptr {
            return 0i32 as size_t;
        }
        ucv -= 0x10000i32;
        high = ((ucv >> 10i32) as libc::c_uint).wrapping_add(0xd800u32) as libc::c_ushort;
        low = (ucv as libc::c_uint & 0x3ffu32).wrapping_add(0xdc00u32) as libc::c_ushort;
        *p.offset(0) = (high as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
        *p.offset(1) = (high as libc::c_int & 0xffi32) as libc::c_uchar;
        *p.offset(2) = (low as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
        *p.offset(3) = (low as libc::c_int & 0xffi32) as libc::c_uchar;
        count = 4i32
    } else {
        if p.offset(2) >= endptr {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xfffdi32 >> 8i32 & 0xffi32) as libc::c_uchar;
        *p.offset(1) = (0xfffdi32 & 0xffi32) as libc::c_uchar;
        count = 2i32
    }
    *pp = (*pp).offset(count as isize);
    return count as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn UC_UTF8_decode_char(
    mut pp: *mut *const libc::c_uchar,
    mut endptr: *const libc::c_uchar,
) -> int32_t {
    let mut p: *const libc::c_uchar = *pp;
    let mut ucv: int32_t = 0;
    let fresh0 = p;
    p = p.offset(1);
    let mut c: libc::c_uchar = *fresh0;
    let mut nbytes: libc::c_int = 0;
    if c as libc::c_int <= 0x7fi32 {
        ucv = c as int32_t;
        nbytes = 0i32
    } else if c as libc::c_int & 0xe0i32 == 0xc0i32 {
        /* 110x xxxx */
        ucv = c as libc::c_int & 31i32;
        nbytes = 1i32
    } else if c as libc::c_int & 0xf0i32 == 0xe0i32 {
        /* 1110 xxxx */
        ucv = c as libc::c_int & 0xfi32;
        nbytes = 2i32
    } else if c as libc::c_int & 0xf8i32 == 0xf0i32 {
        /* 1111 0xxx */
        ucv = c as libc::c_int & 0x7i32;
        nbytes = 3i32
    } else if c as libc::c_int & 0xfci32 == 0xf8i32 {
        /* 1111 10xx */
        ucv = c as libc::c_int & 0x3i32;
        nbytes = 4i32
    } else if c as libc::c_int & 0xfei32 == 0xfci32 {
        /* 1111 110x */
        ucv = c as libc::c_int & 0x1i32;
        nbytes = 5i32
    } else {
        return -1i32;
    }
    if p.offset(nbytes as isize) > endptr {
        return -1i32;
    }
    loop {
        let fresh1 = nbytes;
        nbytes = nbytes - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        let fresh2 = p;
        p = p.offset(1);
        c = *fresh2;
        if c as libc::c_int & 0xc0i32 != 0x80i32 {
            return -1i32;
        }
        ucv = ucv << 6i32 | c as libc::c_int & 0x3fi32
    }
    *pp = p;
    return ucv;
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
pub unsafe extern "C" fn UC_UTF8_encode_char(
    mut ucv: int32_t,
    mut pp: *mut *mut libc::c_uchar,
    mut endptr: *mut libc::c_uchar,
) -> size_t {
    let mut count: libc::c_int = 0i32;
    let mut p: *mut libc::c_uchar = *pp;
    if !pp.is_null() && !(*pp).is_null() && !endptr.is_null() {
    } else {
        __assert_fail(
            b"pp && *pp && endptr\x00" as *const u8 as *const libc::c_char,
            b"dpx-unicode.c\x00" as *const u8 as *const libc::c_char,
            197i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 71], &[libc::c_char; 71]>(
                b"size_t UC_UTF8_encode_char(int32_t, unsigned char **, unsigned char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !UC_is_valid(ucv) {
        return 0i32 as size_t;
    }
    if ucv < 0x7fi32 {
        if p >= endptr.offset(-1) {
            return 0i32 as size_t;
        }
        *p.offset(0) = ucv as libc::c_uchar;
        count = 1i32
    } else if ucv <= 0x7ffi32 {
        if p >= endptr.offset(-2) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xc0i32 | ucv >> 6i32) as libc::c_uchar;
        *p.offset(1) = (0x80i32 | ucv & 0x3fi32) as libc::c_uchar;
        count = 2i32
    } else if ucv <= 0xffffi32 {
        if p >= endptr.offset(-3) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xe0i32 | ucv >> 12i32) as libc::c_uchar;
        *p.offset(1) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(2) = (0x80i32 | ucv & 0x3fi32) as libc::c_uchar;
        count = 3i32
    } else if ucv <= 0x1fffffi32 {
        if p >= endptr.offset(-4) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xf0i32 | ucv >> 18i32) as libc::c_uchar;
        *p.offset(1) = (0x80i32 | ucv >> 12i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(2) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(3) = (0x80i32 | ucv & 0x3fi32) as libc::c_uchar;
        count = 4i32
    } else if ucv <= 0x3ffffffi32 {
        if p >= endptr.offset(-5) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xf8i32 | ucv >> 24i32) as libc::c_uchar;
        *p.offset(1) = (0x80i32 | ucv >> 18i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(2) = (0x80i32 | ucv >> 12i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(3) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(4) = (0x80i32 | ucv & 0x3fi32) as libc::c_uchar;
        count = 5i32
    } else if ucv <= 0x7fffffffi32 {
        if p >= endptr.offset(-6) {
            return 0i32 as size_t;
        }
        *p.offset(0) = (0xfci32 | ucv >> 30i32) as libc::c_uchar;
        *p.offset(1) = (0x80i32 | ucv >> 24i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(2) = (0x80i32 | ucv >> 18i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(3) = (0x80i32 | ucv >> 12i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(4) = (0x80i32 | ucv >> 6i32 & 0x3fi32) as libc::c_uchar;
        *p.offset(5) = (0x80i32 | ucv & 0x3fi32) as libc::c_uchar;
        count = 6i32
    }
    *pp = (*pp).offset(count as isize);
    return count as size_t;
}
