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
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
pub struct ht_entry {
    pub key: *mut libc::c_char,
    pub keylen: libc::c_int,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: libc::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_iter {
    pub index: libc::c_int,
    pub curr: *mut libc::c_void,
    pub hash: *mut ht_table,
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
pub unsafe extern "C" fn xtoi(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32 + 10i32;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32 + 10i32;
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn min4(
    mut x1: libc::c_double,
    mut x2: libc::c_double,
    mut x3: libc::c_double,
    mut x4: libc::c_double,
) -> libc::c_double {
    let mut v: libc::c_double = x1;
    if x2 < v {
        v = x2
    }
    if x3 < v {
        v = x3
    }
    if x4 < v {
        v = x4
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn max4(
    mut x1: libc::c_double,
    mut x2: libc::c_double,
    mut x3: libc::c_double,
    mut x4: libc::c_double,
) -> libc::c_double {
    let mut v: libc::c_double = x1;
    if x2 > v {
        v = x2
    }
    if x3 > v {
        v = x3
    }
    if x4 > v {
        v = x4
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn skip_white_spaces(
    mut s: *mut *mut u8,
    mut endptr: *mut u8,
) {
    while *s < endptr {
        if !(**s as libc::c_int == ' ' as i32
            || **s as libc::c_int == '\t' as i32
            || **s as libc::c_int == '\u{c}' as i32
            || **s as libc::c_int == '\r' as i32
            || **s as libc::c_int == '\n' as i32
            || **s as libc::c_int == '\u{0}' as i32)
        {
            break;
        }
        *s = (*s).offset(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn ht_init_table(mut ht: *mut ht_table, mut hval_free_fn: hval_free_func) {
    let mut i: libc::c_int = 0;
    if !ht.is_null() {
    } else {
        __assert_fail(
            b"ht\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            85i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"void ht_init_table(struct ht_table *, hval_free_func)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < 503i32 {
        (*ht).table[i as usize] = 0 as *mut ht_entry;
        i += 1
    }
    (*ht).count = 0i32;
    (*ht).hval_free_fn = hval_free_fn;
}
#[no_mangle]
pub unsafe extern "C" fn ht_clear_table(mut ht: *mut ht_table) {
    let mut i: libc::c_int = 0;
    if !ht.is_null() {
    } else {
        __assert_fail(
            b"ht\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            99i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void ht_clear_table(struct ht_table *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < 503i32 {
        let mut hent: *mut ht_entry = 0 as *mut ht_entry;
        let mut next: *mut ht_entry = 0 as *mut ht_entry;
        hent = (*ht).table[i as usize];
        while !hent.is_null() {
            if !(*hent).value.is_null() && (*ht).hval_free_fn.is_some() {
                (*ht).hval_free_fn.expect("non-null function pointer")((*hent).value);
            }
            (*hent).value = 0 as *mut libc::c_void;
            if !(*hent).key.is_null() {
                free((*hent).key as *mut libc::c_void);
            }
            (*hent).key = 0 as *mut libc::c_char;
            next = (*hent).next;
            free(hent as *mut libc::c_void);
            hent = next
        }
        (*ht).table[i as usize] = 0 as *mut ht_entry;
        i += 1
    }
    (*ht).count = 0i32;
    (*ht).hval_free_fn = None;
}
#[no_mangle]
pub unsafe extern "C" fn ht_table_size(mut ht: *mut ht_table) -> libc::c_int {
    if !ht.is_null() {
    } else {
        __assert_fail(
            b"ht\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            126i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int ht_table_size(struct ht_table *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*ht).count;
}
unsafe extern "C" fn get_hash(
    mut key: *const libc::c_void,
    mut keylen: libc::c_int,
) -> libc::c_uint {
    let mut hkey: libc::c_uint = 0i32 as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < keylen {
        hkey = (hkey << 5i32)
            .wrapping_add(hkey)
            .wrapping_add(*(key as *const libc::c_char).offset(i as isize) as libc::c_uint);
        i += 1
    }
    return hkey.wrapping_rem(503i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ht_lookup_table(
    mut ht: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: libc::c_int,
) -> *mut libc::c_void {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: libc::c_uint = 0;
    if !ht.is_null() && !key.is_null() {
    } else {
        __assert_fail(
            b"ht && key\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            150i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                b"void *ht_lookup_table(struct ht_table *, const void *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp(
                (*hent).key as *const libc::c_void,
                key,
                keylen as u64,
            ) == 0
        {
            return (*hent).value;
        }
        hent = (*hent).next
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ht_remove_table(
    mut ht: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: libc::c_int,
) -> libc::c_int
/* returns 1 if the element was found and removed and 0 otherwise */ {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut prev: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: libc::c_uint = 0;
    if !ht.is_null() && !key.is_null() {
    } else {
        __assert_fail(
            b"ht && key\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            173i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"int ht_remove_table(struct ht_table *, const void *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    prev = 0 as *mut ht_entry;
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp(
                (*hent).key as *const libc::c_void,
                key,
                keylen as u64,
            ) == 0
        {
            break;
        }
        prev = hent;
        hent = (*hent).next
    }
    if !hent.is_null() {
        (*hent).key = mfree((*hent).key as *mut libc::c_void) as *mut libc::c_char;
        (*hent).keylen = 0i32;
        if !(*hent).value.is_null() && (*ht).hval_free_fn.is_some() {
            (*ht).hval_free_fn.expect("non-null function pointer")((*hent).value);
        }
        (*hent).value = 0 as *mut libc::c_void;
        if !prev.is_null() {
            (*prev).next = (*hent).next
        } else {
            (*ht).table[hkey as usize] = (*hent).next
        }
        free(hent as *mut libc::c_void);
        (*ht).count -= 1;
        return 1i32;
    } else {
        return 0i32;
    };
}
/* replace... */
#[no_mangle]
pub unsafe extern "C" fn ht_insert_table(
    mut ht: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: libc::c_int,
    mut value: *mut libc::c_void,
) {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut prev: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: libc::c_uint = 0;
    if !ht.is_null() && !key.is_null() {
    } else {
        __assert_fail(
            b"ht && key\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            213i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                b"void ht_insert_table(struct ht_table *, const void *, int, void *)\x00",
            ))
            .as_ptr(),
        );
    }
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    prev = 0 as *mut ht_entry;
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp(
                (*hent).key as *const libc::c_void,
                key,
                keylen as u64,
            ) == 0
        {
            break;
        }
        prev = hent;
        hent = (*hent).next
    }
    if !hent.is_null() {
        if !(*hent).value.is_null() && (*ht).hval_free_fn.is_some() {
            (*ht).hval_free_fn.expect("non-null function pointer")((*hent).value);
        }
        (*hent).value = value
    } else {
        hent = new((1i32 as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<ht_entry>() as u64)
            as u32) as *mut ht_entry;
        (*hent).key = new((keylen as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32) as *mut libc::c_char;
        memcpy(
            (*hent).key as *mut libc::c_void,
            key,
            keylen as u64,
        );
        (*hent).keylen = keylen;
        (*hent).value = value;
        (*hent).next = 0 as *mut ht_entry;
        if !prev.is_null() {
            (*prev).next = hent
        } else {
            (*ht).table[hkey as usize] = hent
        }
        (*ht).count += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn ht_append_table(
    mut ht: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: libc::c_int,
    mut value: *mut libc::c_void,
) {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut last: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: libc::c_uint = 0;
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    if hent.is_null() {
        hent = new((1i32 as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<ht_entry>() as u64)
            as u32) as *mut ht_entry;
        (*ht).table[hkey as usize] = hent
    } else {
        while !hent.is_null() {
            last = hent;
            hent = (*hent).next
        }
        hent = new((1i32 as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<ht_entry>() as u64)
            as u32) as *mut ht_entry;
        (*last).next = hent
    }
    (*hent).key = new((keylen as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    memcpy(
        (*hent).key as *mut libc::c_void,
        key,
        keylen as u64,
    );
    (*hent).keylen = keylen;
    (*hent).value = value;
    (*hent).next = 0 as *mut ht_entry;
    (*ht).count += 1;
}
#[no_mangle]
pub unsafe extern "C" fn ht_set_iter(mut ht: *mut ht_table, mut iter: *mut ht_iter) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !ht.is_null() && !iter.is_null() {
    } else {
        __assert_fail(
            b"ht && iter\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            280i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int ht_set_iter(struct ht_table *, struct ht_iter *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < 503i32 {
        if !(*ht).table[i as usize].is_null() {
            (*iter).index = i;
            (*iter).curr = (*ht).table[i as usize] as *mut libc::c_void;
            (*iter).hash = ht;
            return 0i32;
        }
        i += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn ht_clear_iter(mut iter: *mut ht_iter) {
    if !iter.is_null() {
        (*iter).index = 503i32;
        (*iter).curr = 0 as *mut libc::c_void;
        (*iter).hash = 0 as *mut ht_table
    };
}
#[no_mangle]
pub unsafe extern "C" fn ht_iter_getkey(
    mut iter: *mut ht_iter,
    mut keylen: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    hent = (*iter).curr as *mut ht_entry;
    if !iter.is_null() && !hent.is_null() {
        *keylen = (*hent).keylen;
        return (*hent).key;
    } else {
        *keylen = 0i32;
        return 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ht_iter_getval(mut iter: *mut ht_iter) -> *mut libc::c_void {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    hent = (*iter).curr as *mut ht_entry;
    if !iter.is_null() && !hent.is_null() {
        return (*hent).value;
    } else {
        return 0 as *mut libc::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ht_iter_next(mut iter: *mut ht_iter) -> libc::c_int {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut ht: *mut ht_table = 0 as *mut ht_table;
    if !iter.is_null() {
    } else {
        __assert_fail(
            b"iter\x00" as *const u8 as *const libc::c_char,
            b"dpx-dpxutil.c\x00" as *const u8 as *const libc::c_char,
            338i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"int ht_iter_next(struct ht_iter *)\x00",
            ))
            .as_ptr(),
        );
    }
    ht = (*iter).hash;
    hent = (*iter).curr as *mut ht_entry;
    hent = (*hent).next;
    while hent.is_null() && {
        (*iter).index += 1;
        (*iter).index < 503i32
    } {
        hent = (*ht).table[(*iter).index as usize]
    }
    (*iter).curr = hent as *mut libc::c_void;
    return if !hent.is_null() { 0i32 } else { -1i32 };
}
unsafe extern "C" fn read_c_escchar(
    mut r: *mut libc::c_char,
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0i32;
    let mut l: libc::c_int = 1i32;
    let mut p: *const libc::c_char = *pp;
    match *p.offset(0) as libc::c_int {
        97 => {
            c = '\u{7}' as i32;
            p = p.offset(1)
        }
        98 => {
            c = '\u{8}' as i32;
            p = p.offset(1)
        }
        102 => {
            c = '\u{c}' as i32;
            p = p.offset(1)
        }
        110 => {
            c = '\n' as i32;
            p = p.offset(1)
        }
        114 => {
            c = '\r' as i32;
            p = p.offset(1)
        }
        116 => {
            c = '\t' as i32;
            p = p.offset(1)
        }
        118 => {
            c = '\u{b}' as i32;
            p = p.offset(1)
        }
        92 | 63 | 39 | 34 => {
            c = *p.offset(0) as libc::c_int;
            p = p.offset(1)
        }
        10 => {
            l = 0i32;
            p = p.offset(1)
        }
        13 => {
            p = p.offset(1);
            if p < endptr && *p.offset(0) as libc::c_int == '\n' as i32 {
                p = p.offset(1)
            }
            l = 0i32
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            let mut i: libc::c_int = 0;
            c = 0i32;
            i = 0i32;
            while i < 3i32
                && p < endptr
                && *p.offset(0) as libc::c_int >= '0' as i32
                && *p.offset(0) as libc::c_int <= '7' as i32
            {
                c = (c << 3i32) + (*p.offset(0) as libc::c_int - '0' as i32);
                i += 1;
                p = p.offset(1)
            }
        }
        120 => {
            let mut i_0: libc::c_int = 0;
            c = 0i32;
            i_0 = 0i32;
            p = p.offset(1);
            while i_0 < 2i32
                && p < endptr
                && *(*__ctype_b_loc()).offset(*p.offset(0) as u8 as libc::c_int as isize)
                    as libc::c_int
                    & _ISxdigit as libc::c_int as u16 as libc::c_int
                    != 0
            {
                c = (c << 4i32)
                    + (if *(*__ctype_b_loc())
                        .offset(*p.offset(0) as u8 as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as u16 as libc::c_int
                        != 0
                    {
                        *p.offset(0) as libc::c_int - '0' as i32
                    } else {
                        (if *(*__ctype_b_loc())
                            .offset(*p.offset(0) as u8 as libc::c_int as isize)
                            as libc::c_int
                            & _ISlower as libc::c_int as u16 as libc::c_int
                            != 0
                        {
                            *p.offset(0) as libc::c_int - 'a' as i32 + 10i32
                        } else {
                            *p.offset(0) as libc::c_int - 'A' as i32 + 10i32
                        })
                    });
                i_0 += 1;
                p = p.offset(1)
            }
        }
        _ => {
            dpx_warning(
                b"Unknown escape char sequence: \\%c\x00" as *const u8 as *const libc::c_char,
                *p.offset(0) as libc::c_int,
            );
            l = 0i32;
            p = p.offset(1)
        }
    }
    if !r.is_null() {
        *r = c as libc::c_char
    }
    *pp = p;
    return l;
}
unsafe extern "C" fn read_c_litstrc(
    mut q: *mut libc::c_char,
    mut len: libc::c_int,
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: libc::c_int = 0i32;
    let mut s: libc::c_int = -1i32;
    l = 0i32;
    p = *pp;
    while s == -1i32 && p < endptr {
        match *p.offset(0) as libc::c_int {
            34 => {
                s = 0i32;
                p = p.offset(1)
            }
            92 => {
                if !q.is_null() && l == len {
                    s = -3i32
                } else {
                    p = p.offset(1);
                    l += read_c_escchar(
                        if !q.is_null() {
                            &mut *q.offset(l as isize)
                        } else {
                            0 as *mut libc::c_char
                        },
                        &mut p,
                        endptr,
                    )
                }
            }
            10 | 13 => s = -2i32,
            _ => {
                if !q.is_null() && l == len {
                    s = -3i32
                } else {
                    if q.is_null() {
                        l += 1
                    } else {
                        let fresh0 = l;
                        l = l + 1;
                        *q.offset(fresh0 as isize) = *p.offset(0)
                    }
                    p = p.offset(1)
                }
            }
        }
    }
    if s == 0i32 {
        if !q.is_null() && l == len {
            s = -3i32
        } else if !q.is_null() {
            let fresh1 = l;
            l = l + 1;
            *q.offset(fresh1 as isize) = '\u{0}' as i32 as libc::c_char
        }
    }
    *pp = p;
    return if s == 0i32 { l } else { s };
}
#[no_mangle]
pub unsafe extern "C" fn parse_c_string(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = *pp;
    let mut l: libc::c_int = 0i32;
    if p >= endptr || *p.offset(0) as libc::c_int != '\"' as i32 {
        return 0 as *mut libc::c_char;
    }
    p = p.offset(1);
    l = read_c_litstrc(0 as *mut libc::c_char, 0i32, &mut p, endptr);
    if l >= 0i32 {
        q = new(((l + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32) as *mut libc::c_char;
        p = (*pp).offset(1);
        l = read_c_litstrc(q, l + 1i32, &mut p, endptr)
    }
    *pp = p;
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn parse_c_ident(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = *pp;
    let mut n: libc::c_int = 0;
    if p >= endptr
        || !(*p as libc::c_int == '_' as i32
            || *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
            || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32)
    {
        return 0 as *mut libc::c_char;
    }
    n = 0i32;
    while p < endptr
        && (*p as libc::c_int == '_' as i32
            || *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
            || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32
            || *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32)
    {
        p = p.offset(1);
        n += 1
    }
    q = new(((n + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    memcpy(
        q as *mut libc::c_void,
        *pp as *const libc::c_void,
        n as u64,
    );
    *q.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    *pp = p;
    return q;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
pub unsafe extern "C" fn parse_float_decimal(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = *pp;
    let mut s: libc::c_int = 0i32;
    let mut n: libc::c_int = 0i32;
    if p >= endptr {
        return 0 as *mut libc::c_char;
    }
    if *p.offset(0) as libc::c_int == '+' as i32 || *p.offset(0) as libc::c_int == '-' as i32 {
        p = p.offset(1)
    }
    /* 1. .01 001 001E-001 */
    s = 0i32;
    n = 0i32;
    while p < endptr && s >= 0i32 {
        match *p.offset(0) as libc::c_int {
            43 | 45 => {
                if s != 2i32 {
                    s = -1i32
                } else {
                    s = 3i32;
                    p = p.offset(1)
                }
            }
            46 => {
                if s > 0i32 {
                    s = -1i32
                } else {
                    s = 1i32;
                    p = p.offset(1)
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                n += 1;
                p = p.offset(1)
            }
            69 | 101 => {
                if n == 0i32 || s == 2i32 {
                    s = -1i32
                } else {
                    s = 2i32;
                    p = p.offset(1)
                }
            }
            _ => s = -1i32,
        }
    }
    if n != 0i32 {
        n = p.wrapping_offset_from(*pp) as libc::c_long as libc::c_int;
        q = new(((n + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32) as *mut libc::c_char;
        memcpy(
            q as *mut libc::c_void,
            *pp as *const libc::c_void,
            n as u64,
        );
        *q.offset(n as isize) = '\u{0}' as i32 as libc::c_char
    }
    *pp = p;
    return q;
}
