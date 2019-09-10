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
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    static mut buffer: *mut UnicodeScalar;
    #[no_mangle]
    static mut max_strings: int32_t;
    #[no_mangle]
    static mut pool_size: int32_t;
    #[no_mangle]
    static mut str_pool: *mut packed_UTF16_code;
    #[no_mangle]
    static mut str_start: *mut pool_pointer;
    #[no_mangle]
    static mut pool_ptr: pool_pointer;
    #[no_mangle]
    static mut str_ptr: str_number;
    #[no_mangle]
    static mut init_pool_ptr: pool_pointer;
    #[no_mangle]
    static mut init_str_ptr: str_number;
    #[no_mangle]
    fn overflow(s: *const i8, n: int32_t) -> !;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type UnicodeScalar = int32_t;
pub type pool_pointer = int32_t;
pub type str_number = int32_t;
pub type packed_UTF16_code = u16;
/* tectonic/xetex-stringpool.c: preloaded "string pool" constants
   Copyright 2017-2018 the Tectonic Project
   Licensed under the MIT License.
*/
static mut string_constants: [*const i8; 3] = [
    b"this marks the start of the stringpool\x00" as *const u8 as *const i8,
    b"\x00" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn load_pool_strings(mut spare_size: int32_t) -> libc::c_int {
    let mut s: *const i8 = 0 as *const i8;
    let mut i: libc::c_int = 0i32;
    let mut total_len: size_t = 0i32 as size_t;
    let mut g: str_number = 0i32;
    loop {
        let fresh0 = i;
        i = i + 1;
        s = string_constants[fresh0 as usize];
        if s.is_null() {
            break;
        }
        let mut len: size_t = strlen(s);
        total_len = (total_len as u64).wrapping_add(len) as size_t as size_t;
        if total_len >= spare_size as u64 {
            return 0i32;
        }
        loop {
            let fresh1 = len;
            len = len.wrapping_sub(1);
            if !(fresh1 > 0i32 as u64) {
                break;
            }
            let fresh2 = s;
            s = s.offset(1);
            let fresh3 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh3 as isize) = *fresh2 as packed_UTF16_code
        }
        g = make_string()
        /* Returns 0 on error. */
    }
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn length(mut s: str_number) -> int32_t {
    if s as libc::c_long >= 65536 {
        return *str_start.offset(((s + 1i32) as libc::c_long - 65536) as isize)
            - *str_start.offset((s as libc::c_long - 65536) as isize);
    } else if s >= 32i32 && s < 127i32 {
        return 1i32;
    } else if s <= 127i32 {
        return 3i32;
    } else if s < 256i32 {
        return 4i32;
    } else {
        return 8i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_string() -> str_number {
    if str_ptr == max_strings {
        overflow(
            b"number of strings\x00" as *const u8 as *const i8,
            max_strings - init_str_ptr,
        );
    }
    str_ptr += 1;
    *str_start.offset((str_ptr - 65536i32) as isize) = pool_ptr;
    return str_ptr - 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn append_str(mut s: str_number) {
    let mut i: int32_t = 0;
    let mut j: pool_pointer = 0;
    i = length(s);
    if pool_ptr + i > pool_size {
        overflow(
            b"pool size\x00" as *const u8 as *const i8,
            pool_size - init_pool_ptr,
        );
    }
    j = *str_start.offset((s as libc::c_long - 65536) as isize);
    while i > 0i32 {
        *str_pool.offset(pool_ptr as isize) = *str_pool.offset(j as isize);
        pool_ptr += 1;
        j += 1;
        i -= 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn str_eq_buf(mut s: str_number, mut k: int32_t) -> bool {
    let mut j: pool_pointer = 0;
    j = *str_start.offset((s as libc::c_long - 65536) as isize);
    while j < *str_start.offset(((s + 1i32) as libc::c_long - 65536) as isize) {
        if *buffer.offset(k as isize) as libc::c_long >= 65536 {
            if *str_pool.offset(j as isize) as libc::c_long
                != 55296
                    + (*buffer.offset(k as isize) as libc::c_long - 65536) / 1024 as libc::c_long
            {
                return 0i32 != 0;
            } else {
                if *str_pool.offset((j + 1i32) as isize) as libc::c_long
                    != 56320
                        + (*buffer.offset(k as isize) as libc::c_long - 65536)
                            % 1024 as libc::c_long
                {
                    return 0i32 != 0;
                } else {
                    j += 1
                }
            }
        } else if *str_pool.offset(j as isize) as libc::c_int != *buffer.offset(k as isize) {
            return 0i32 != 0;
        }
        j += 1;
        k += 1
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn str_eq_str(mut s: str_number, mut t: str_number) -> bool {
    let mut j: pool_pointer = 0;
    let mut k: pool_pointer = 0;
    if length(s) != length(t) {
        return 0i32 != 0;
    }
    if length(s) == 1i32 {
        if (s as libc::c_long) < 65536 {
            if (t as libc::c_long) < 65536 {
                if s != t {
                    return 0i32 != 0;
                }
            } else if s
                != *str_pool
                    .offset(*str_start.offset((t as libc::c_long - 65536) as isize) as isize)
                    as libc::c_int
            {
                return 0i32 != 0;
            }
        } else if (t as libc::c_long) < 65536 {
            if *str_pool.offset(*str_start.offset((s as libc::c_long - 65536) as isize) as isize)
                as libc::c_int
                != t
            {
                return 0i32 != 0;
            }
        } else if *str_pool.offset(*str_start.offset((s as libc::c_long - 65536) as isize) as isize)
            as libc::c_int
            != *str_pool.offset(*str_start.offset((t as libc::c_long - 65536) as isize) as isize)
                as libc::c_int
        {
            return 0i32 != 0;
        }
    } else {
        j = *str_start.offset((s as libc::c_long - 65536) as isize);
        k = *str_start.offset((t as libc::c_long - 65536) as isize);
        while j < *str_start.offset(((s + 1i32) as libc::c_long - 65536) as isize) {
            if *str_pool.offset(j as isize) as libc::c_int
                != *str_pool.offset(k as isize) as libc::c_int
            {
                return 0i32 != 0;
            }
            j += 1;
            k += 1
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn search_string(mut search: str_number) -> str_number {
    let mut s: str_number = 0;
    let mut len: int32_t = 0;
    len = length(search);
    if len == 0i32 {
        return (65536 + 1i32 as libc::c_long) as str_number;
    } else {
        s = search - 1i32;
        while s as libc::c_long > 65535 {
            if length(s) == len {
                if str_eq_str(s, search) {
                    return s;
                }
            }
            s -= 1
        }
    }
    return 0i32;
}
/* tectonic/xetex-stringpool.h: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn slow_make_string() -> str_number {
    let mut s: str_number = 0;
    let mut t: str_number = 0;
    t = make_string();
    s = search_string(t);
    if s > 0i32 {
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - 65536i32) as isize);
        return s;
    }
    return t;
}
