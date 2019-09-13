#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::mfree;
use libc::free;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut i8,
    pub keylen: i32,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: i32,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_iter {
    pub index: i32,
    pub curr: *mut libc::c_void,
    pub hash: *mut ht_table,
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
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
pub unsafe extern "C" fn xtoi(mut c: i8) -> i32 {
    if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
        return c as i32 - '0' as i32;
    }
    if c as i32 >= 'A' as i32 && c as i32 <= 'F' as i32 {
        return c as i32 - 'A' as i32 + 10i32;
    }
    if c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32 {
        return c as i32 - 'a' as i32 + 10i32;
    }
    -1i32
}
#[no_mangle]
pub unsafe extern "C" fn min4(mut x1: f64, mut x2: f64, mut x3: f64, mut x4: f64) -> f64 {
    let mut v: f64 = x1;
    if x2 < v {
        v = x2
    }
    if x3 < v {
        v = x3
    }
    if x4 < v {
        v = x4
    }
    v
}
#[no_mangle]
pub unsafe extern "C" fn max4(mut x1: f64, mut x2: f64, mut x3: f64, mut x4: f64) -> f64 {
    let mut v: f64 = x1;
    if x2 > v {
        v = x2
    }
    if x3 > v {
        v = x3
    }
    if x4 > v {
        v = x4
    }
    v
}
#[no_mangle]
pub unsafe extern "C" fn skip_white_spaces(mut s: *mut *mut u8, mut endptr: *mut u8) {
    while *s < endptr {
        if !(**s as i32 == ' ' as i32
            || **s as i32 == '\t' as i32
            || **s as i32 == '\u{c}' as i32
            || **s as i32 == '\r' as i32
            || **s as i32 == '\n' as i32
            || **s as i32 == '\u{0}' as i32)
        {
            break;
        }
        *s = (*s).offset(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn ht_init_table(mut ht: *mut ht_table, mut hval_free_fn: hval_free_func) {
    let mut i: i32 = 0;
    assert!(!ht.is_null());
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
    let mut i: i32 = 0;
    assert!(!ht.is_null());
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
            (*hent).key = 0 as *mut i8;
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
pub unsafe extern "C" fn ht_table_size(mut ht: *mut ht_table) -> i32 {
    assert!(!ht.is_null());
    (*ht).count
}
unsafe extern "C" fn get_hash(mut key: *const libc::c_void, mut keylen: i32) -> u32 {
    let mut hkey: u32 = 0_u32;
    let mut i: i32 = 0;
    i = 0i32;
    while i < keylen {
        hkey = (hkey << 5i32)
            .wrapping_add(hkey)
            .wrapping_add(*(key as *const i8).offset(i as isize) as u32);
        i += 1
    }
    hkey.wrapping_rem(503_u32)
}
#[no_mangle]
pub unsafe extern "C" fn ht_lookup_table(
    mut ht: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: i32,
) -> *mut libc::c_void {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: u32 = 0;
    assert!(!ht.is_null() && !key.is_null());
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp((*hent).key as *const libc::c_void, key, keylen as u64) == 0
        {
            return (*hent).value;
        }
        hent = (*hent).next
    }
    0 as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn ht_remove_table(
    mut ht: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: i32,
) -> i32
/* returns 1 if the element was found and removed and 0 otherwise */ {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut prev: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: u32 = 0;
    assert!(!ht.is_null() && !key.is_null());
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    prev = 0 as *mut ht_entry;
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp((*hent).key as *const libc::c_void, key, keylen as u64) == 0
        {
            break;
        }
        prev = hent;
        hent = (*hent).next
    }
    if !hent.is_null() {
        (*hent).key = mfree((*hent).key as *mut libc::c_void) as *mut i8;
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
    mut keylen: i32,
    mut value: *mut libc::c_void,
) {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut prev: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: u32 = 0;
    assert!(!ht.is_null() && !key.is_null());
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    prev = 0 as *mut ht_entry;
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp((*hent).key as *const libc::c_void, key, keylen as u64) == 0
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
        hent = new((1_u64).wrapping_mul(::std::mem::size_of::<ht_entry>() as u64) as u32)
            as *mut ht_entry;
        (*hent).key =
            new((keylen as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
                as *mut i8;
        memcpy((*hent).key as *mut libc::c_void, key, keylen as u64);
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
    mut keylen: i32,
    mut value: *mut libc::c_void,
) {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut last: *mut ht_entry = 0 as *mut ht_entry;
    let mut hkey: u32 = 0;
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    if hent.is_null() {
        hent = new((1_u64).wrapping_mul(::std::mem::size_of::<ht_entry>() as u64) as u32)
            as *mut ht_entry;
        (*ht).table[hkey as usize] = hent
    } else {
        while !hent.is_null() {
            last = hent;
            hent = (*hent).next
        }
        hent = new((1_u64).wrapping_mul(::std::mem::size_of::<ht_entry>() as u64) as u32)
            as *mut ht_entry;
        (*last).next = hent
    }
    (*hent).key =
        new((keylen as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    memcpy((*hent).key as *mut libc::c_void, key, keylen as u64);
    (*hent).keylen = keylen;
    (*hent).value = value;
    (*hent).next = 0 as *mut ht_entry;
    (*ht).count += 1;
}
#[no_mangle]
pub unsafe extern "C" fn ht_set_iter(mut ht: *mut ht_table, mut iter: *mut ht_iter) -> i32 {
    let mut i: i32 = 0;
    assert!(!ht.is_null() && !iter.is_null());
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
    -1i32
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
pub unsafe extern "C" fn ht_iter_getkey(mut iter: *mut ht_iter, mut keylen: *mut i32) -> *mut i8 {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    hent = (*iter).curr as *mut ht_entry;
    if !iter.is_null() && !hent.is_null() {
        *keylen = (*hent).keylen;
        return (*hent).key;
    } else {
        *keylen = 0i32;
        return 0 as *mut i8;
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
pub unsafe extern "C" fn ht_iter_next(mut iter: *mut ht_iter) -> i32 {
    let mut hent: *mut ht_entry = 0 as *mut ht_entry;
    let mut ht: *mut ht_table = 0 as *mut ht_table;
    assert!(!iter.is_null());
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
    if !hent.is_null() {
        0i32
    } else {
        -1i32
    }
}
unsafe extern "C" fn read_c_escchar(
    mut r: *mut i8,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut c: i32 = 0i32;
    let mut l: i32 = 1i32;
    let mut p: *const i8 = *pp;
    match *p.offset(0) as i32 {
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
            c = *p.offset(0) as i32;
            p = p.offset(1)
        }
        10 => {
            l = 0i32;
            p = p.offset(1)
        }
        13 => {
            p = p.offset(1);
            if p < endptr && *p.offset(0) as i32 == '\n' as i32 {
                p = p.offset(1)
            }
            l = 0i32
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            let mut i: i32 = 0;
            c = 0i32;
            i = 0i32;
            while i < 3i32
                && p < endptr
                && *p.offset(0) as i32 >= '0' as i32
                && *p.offset(0) as i32 <= '7' as i32
            {
                c = (c << 3i32) + (*p.offset(0) as i32 - '0' as i32);
                i += 1;
                p = p.offset(1)
            }
        }
        120 => {
            let mut i_0: i32 = 0;
            c = 0i32;
            i_0 = 0i32;
            p = p.offset(1);
            while i_0 < 2i32 && p < endptr && libc::isxdigit(*p.offset(0) as _) != 0 {
                c = (c << 4i32)
                    + (if libc::isdigit(*p.offset(0) as _) != 0 {
                        *p.offset(0) as i32 - '0' as i32
                    } else {
                        (if libc::islower(*p.offset(0) as _) != 0 {
                            *p.offset(0) as i32 - 'a' as i32 + 10i32
                        } else {
                            *p.offset(0) as i32 - 'A' as i32 + 10i32
                        })
                    });
                i_0 += 1;
                p = p.offset(1)
            }
        }
        _ => {
            dpx_warning(
                b"Unknown escape char sequence: \\%c\x00" as *const u8 as *const i8,
                *p.offset(0) as i32,
            );
            l = 0i32;
            p = p.offset(1)
        }
    }
    if !r.is_null() {
        *r = c as i8
    }
    *pp = p;
    l
}
unsafe extern "C" fn read_c_litstrc(
    mut q: *mut i8,
    mut len: i32,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut l: i32 = 0i32;
    let mut s: i32 = -1i32;
    l = 0i32;
    p = *pp;
    while s == -1i32 && p < endptr {
        match *p.offset(0) as i32 {
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
                            0 as *mut i8
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
            *q.offset(fresh1 as isize) = '\u{0}' as i32 as i8
        }
    }
    *pp = p;
    if s == 0i32 {
        l
    } else {
        s
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_c_string(mut pp: *mut *const i8, mut endptr: *const i8) -> *mut i8 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = *pp;
    let mut l: i32 = 0i32;
    if p >= endptr || *p.offset(0) as i32 != '\"' as i32 {
        return 0 as *mut i8;
    }
    p = p.offset(1);
    l = read_c_litstrc(0 as *mut i8, 0i32, &mut p, endptr);
    if l >= 0i32 {
        q = new(((l + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        p = (*pp).offset(1);
        l = read_c_litstrc(q, l + 1i32, &mut p, endptr)
    }
    *pp = p;
    q
}
#[no_mangle]
pub unsafe extern "C" fn parse_c_ident(mut pp: *mut *const i8, mut endptr: *const i8) -> *mut i8 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = *pp;
    let mut n: i32 = 0;
    if p >= endptr
        || !(*p as i32 == '_' as i32
            || *p as i32 >= 'a' as i32 && *p as i32 <= 'z' as i32
            || *p as i32 >= 'A' as i32 && *p as i32 <= 'Z' as i32)
    {
        return 0 as *mut i8;
    }
    n = 0i32;
    while p < endptr
        && (*p as i32 == '_' as i32
            || *p as i32 >= 'a' as i32 && *p as i32 <= 'z' as i32
            || *p as i32 >= 'A' as i32 && *p as i32 <= 'Z' as i32
            || *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32)
    {
        p = p.offset(1);
        n += 1
    }
    q = new(((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    memcpy(q as *mut libc::c_void, *pp as *const libc::c_void, n as u64);
    *q.offset(n as isize) = '\u{0}' as i32 as i8;
    *pp = p;
    q
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
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut i8 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = *pp;
    let mut s: i32 = 0i32;
    let mut n: i32 = 0i32;
    if p >= endptr {
        return 0 as *mut i8;
    }
    if *p.offset(0) as i32 == '+' as i32 || *p.offset(0) as i32 == '-' as i32 {
        p = p.offset(1)
    }
    /* 1. .01 001 001E-001 */
    s = 0i32;
    n = 0i32;
    while p < endptr && s >= 0i32 {
        match *p.offset(0) as i32 {
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
        n = p.wrapping_offset_from(*pp) as i64 as i32;
        q = new(((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        memcpy(q as *mut libc::c_void, *pp as *const libc::c_void, n as u64);
        *q.offset(n as isize) = '\u{0}' as i32 as i8
    }
    *pp = p;
    q
}
