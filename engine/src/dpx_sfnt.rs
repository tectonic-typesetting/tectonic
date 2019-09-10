#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pdf_obj;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
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
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type BYTE = u8;
pub type USHORT = u16;
pub type SFNT_ULONG = u32;
pub type SFNT_LONG = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [libc::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut libc::c_char,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr); /* tag name */
    return 0 as *mut libc::c_void; /* typefaces number */
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_open(mut handle: rust_input_handle_t) -> *mut sfnt {
    let mut sfont: *mut sfnt = 0 as *mut sfnt; /* typefaces position */
    let mut type_0: SFNT_ULONG = 0; /* resource id */
    if !handle.is_null() {
    } else {
        __assert_fail(
            b"handle\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            52i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"sfnt *sfnt_open(rust_input_handle_t)\x00",
            ))
            .as_ptr(),
        ); /* resource name position from name_list */
    } /* resource flag (byte) + resource offset */
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32); /* mbz */
    sfont = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<sfnt>() as u64) as u32)
        as *mut sfnt;
    (*sfont).handle = handle;
    type_0 = tt_get_unsigned_quad((*sfont).handle);
    if type_0 as u64 == 0x10000 || type_0 as u64 == 0x74727565 {
        (*sfont).type_0 = 1i32 << 0i32
    } else if type_0 as u64 == 0x10000 {
        (*sfont).type_0 = 1i32 << 1i32
    } else if type_0 as u64 == 0x4f54544f {
        (*sfont).type_0 = 1i32 << 2i32
    } else if type_0 as u64 == 0x74746366 {
        (*sfont).type_0 = 1i32 << 4i32
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    (*sfont).directory = 0 as *mut sfnt_table_directory;
    (*sfont).offset = 0u64 as SFNT_ULONG;
    return sfont;
}
#[no_mangle]
pub unsafe extern "C" fn dfont_open(
    mut handle: rust_input_handle_t,
    mut index: libc::c_int,
) -> *mut sfnt {
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut rdata_pos: SFNT_ULONG = 0;
    let mut map_pos: SFNT_ULONG = 0;
    let mut tags_pos: SFNT_ULONG = 0;
    let mut types_pos: SFNT_ULONG = 0;
    let mut res_pos: SFNT_ULONG = 0;
    let mut tag: SFNT_ULONG = 0;
    let mut tags_num: USHORT = 0;
    let mut types_num: USHORT = 0;
    let mut i: USHORT = 0;
    if !handle.is_null() {
    } else {
        __assert_fail(
            b"handle\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            87i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"sfnt *dfont_open(rust_input_handle_t, int)\x00",
            ))
            .as_ptr(),
        );
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    sfont = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<sfnt>() as u64) as u32)
        as *mut sfnt;
    (*sfont).handle = handle;
    rdata_pos = tt_get_unsigned_quad((*sfont).handle);
    map_pos = tt_get_unsigned_quad((*sfont).handle);
    ttstub_input_seek(
        (*sfont).handle,
        map_pos.wrapping_add(0x18i32 as libc::c_uint) as ssize_t,
        0i32,
    );
    tags_pos = map_pos.wrapping_add(tt_get_unsigned_pair((*sfont).handle) as libc::c_uint);
    ttstub_input_seek((*sfont).handle, tags_pos as ssize_t, 0i32);
    tags_num = tt_get_unsigned_pair((*sfont).handle);
    i = 0i32 as USHORT;
    while i as libc::c_int <= tags_num as libc::c_int {
        tag = tt_get_unsigned_quad((*sfont).handle);
        types_num = tt_get_unsigned_pair((*sfont).handle);
        types_pos = tags_pos.wrapping_add(tt_get_unsigned_pair((*sfont).handle) as libc::c_uint);
        if tag as u64 == 0x73666e74 {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i as libc::c_int > tags_num as libc::c_int {
        free(sfont as *mut libc::c_void);
        return 0 as *mut sfnt;
    }
    ttstub_input_seek((*sfont).handle, types_pos as ssize_t, 0i32);
    if index > types_num as libc::c_int {
        _tt_abort(
            b"Invalid index %d for dfont.\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    i = 0i32 as USHORT;
    while i as libc::c_int <= types_num as libc::c_int {
        tt_get_unsigned_pair((*sfont).handle);
        tt_get_unsigned_pair((*sfont).handle);
        res_pos = tt_get_unsigned_quad((*sfont).handle);
        tt_get_unsigned_quad((*sfont).handle);
        if i as libc::c_int == index {
            break;
        }
        i = i.wrapping_add(1)
    }
    ttstub_input_seek((*sfont).handle, 0i32 as ssize_t, 0i32);
    (*sfont).type_0 = 1i32 << 8i32;
    (*sfont).directory = 0 as *mut sfnt_table_directory;
    (*sfont).offset = (res_pos as u64 & 0xffffff)
        .wrapping_add(rdata_pos as u64)
        .wrapping_add(4i32 as u64) as SFNT_ULONG;
    return sfont;
}
unsafe extern "C" fn release_directory(mut td: *mut sfnt_table_directory) {
    let mut i: libc::c_uint = 0;
    if !td.is_null() {
        if !(*td).tables.is_null() {
            i = 0i32 as libc::c_uint;
            while i < (*td).num_tables as libc::c_uint {
                free((*(*td).tables.offset(i as isize)).data as *mut libc::c_void);
                i = i.wrapping_add(1)
            }
            free((*td).tables as *mut libc::c_void);
        }
        free((*td).flags as *mut libc::c_void);
        free(td as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_close(mut sfont: *mut sfnt) {
    if !sfont.is_null() {
        if !(*sfont).directory.is_null() {
            release_directory((*sfont).directory);
        }
        free(sfont as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn put_big_endian(
    mut s: *mut libc::c_void,
    mut q: SFNT_LONG,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = s as *mut libc::c_char;
    i = n - 1i32;
    while i >= 0i32 {
        *p.offset(i as isize) = (q & 0xffi32) as libc::c_char;
        q >>= 8i32;
        i -= 1
    }
    return n;
}
/* Convert four-byte number to big endianess
 * in a machine independent way.
 */
unsafe extern "C" fn convert_tag(mut tag: *mut libc::c_char, mut u_tag: u32) {
    let mut i: libc::c_int = 0;
    i = 3i32;
    while i >= 0i32 {
        *tag.offset(i as isize) = u_tag.wrapping_rem(256i32 as libc::c_uint) as libc::c_char;
        u_tag =
            (u_tag as libc::c_uint).wrapping_div(256i32 as libc::c_uint) as u32;
        i -= 1
    }
}
/*
 * Computes the max power of 2 <= n
 */
unsafe extern "C" fn max2floor(mut n: libc::c_uint) -> libc::c_uint {
    let mut val: libc::c_int = 1i32;
    while n > 1i32 as libc::c_uint {
        n = n.wrapping_div(2i32 as libc::c_uint);
        val *= 2i32
    }
    return val as libc::c_uint;
}
/*
 * Computes the log2 of the max power of 2 <= n
 */
unsafe extern "C" fn log2floor(mut n: libc::c_uint) -> libc::c_uint {
    let mut val: libc::c_uint = 0i32 as libc::c_uint;
    while n > 1i32 as libc::c_uint {
        n = n.wrapping_div(2i32 as libc::c_uint);
        val = val.wrapping_add(1)
    }
    return val;
}
unsafe extern "C" fn sfnt_calc_checksum(
    mut data: *mut libc::c_void,
    mut length: SFNT_ULONG,
) -> SFNT_ULONG {
    let mut chksum: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut p: *mut BYTE = 0 as *mut BYTE;
    let mut endptr: *mut BYTE = 0 as *mut BYTE;
    let mut count: libc::c_int = 0i32;
    p = data as *mut BYTE;
    endptr = p.offset(length as isize);
    while p < endptr {
        chksum = (chksum as libc::c_uint)
            .wrapping_add(((*p.offset(0) as libc::c_int) << 8i32 * (3i32 - count)) as libc::c_uint)
            as SFNT_ULONG as SFNT_ULONG;
        count = count + 1i32 & 3i32;
        p = p.offset(1)
    }
    return chksum;
}
unsafe extern "C" fn find_table_index(
    mut td: *mut sfnt_table_directory,
    mut tag: *const libc::c_char,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    if td.is_null() {
        return -1i32;
    }
    idx = 0i32;
    while idx < (*td).num_tables as libc::c_int {
        if memcmp(
            (*(*td).tables.offset(idx as isize)).tag.as_mut_ptr() as *const libc::c_void,
            tag as *const libc::c_void,
            4i32 as u64,
        ) == 0
        {
            return idx;
        }
        idx += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_set_table(
    mut sfont: *mut sfnt,
    mut tag: *const libc::c_char,
    mut data: *mut libc::c_void,
    mut length: SFNT_ULONG,
) {
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: libc::c_int = 0;
    if !sfont.is_null() {
    } else {
        __assert_fail(
            b"sfont\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            272i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                b"void sfnt_set_table(sfnt *, const char *, void *, SFNT_ULONG)\x00",
            ))
            .as_ptr(),
        );
    }
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        idx = (*td).num_tables as libc::c_int;
        (*td).num_tables = (*td).num_tables.wrapping_add(1);
        (*td).tables = renew(
            (*td).tables as *mut libc::c_void,
            ((*td).num_tables as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<sfnt_table>() as u64)
                as u32,
        ) as *mut sfnt_table;
        memcpy(
            (*(*td).tables.offset(idx as isize)).tag.as_mut_ptr() as *mut libc::c_void,
            tag as *const libc::c_void,
            4i32 as u64,
        );
    }
    (*(*td).tables.offset(idx as isize)).check_sum = sfnt_calc_checksum(data, length);
    (*(*td).tables.offset(idx as isize)).offset = 0i64 as SFNT_ULONG;
    (*(*td).tables.offset(idx as isize)).length = length;
    let ref mut fresh0 = (*(*td).tables.offset(idx as isize)).data;
    *fresh0 = data as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_find_table_len(
    mut sfont: *mut sfnt,
    mut tag: *const libc::c_char,
) -> SFNT_ULONG {
    let mut length: SFNT_ULONG = 0;
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: libc::c_int = 0;
    if !sfont.is_null() && !tag.is_null() {
    } else {
        __assert_fail(
            b"sfont && tag\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            299i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"SFNT_ULONG sfnt_find_table_len(sfnt *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        length = 0i32 as SFNT_ULONG
    } else {
        length = (*(*td).tables.offset(idx as isize)).length
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_find_table_pos(
    mut sfont: *mut sfnt,
    mut tag: *const libc::c_char,
) -> SFNT_ULONG {
    let mut offset: SFNT_ULONG = 0;
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: libc::c_int = 0;
    if !sfont.is_null() && !tag.is_null() {
    } else {
        __assert_fail(
            b"sfont && tag\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            319i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"SFNT_ULONG sfnt_find_table_pos(sfnt *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        offset = 0i32 as SFNT_ULONG
    } else {
        offset = (*(*td).tables.offset(idx as isize)).offset
    }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_locate_table(
    mut sfont: *mut sfnt,
    mut tag: *const libc::c_char,
) -> SFNT_ULONG {
    let mut offset: SFNT_ULONG = 0;
    if !sfont.is_null() && !tag.is_null() {
    } else {
        __assert_fail(
            b"sfont && tag\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            337i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"SFNT_ULONG sfnt_locate_table(sfnt *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    offset = sfnt_find_table_pos(sfont, tag);
    if offset == 0i32 as libc::c_uint {
        _tt_abort(b"sfnt: table not found...\x00" as *const u8 as *const libc::c_char);
    }
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_read_table_directory(
    mut sfont: *mut sfnt,
    mut offset: SFNT_ULONG,
) -> libc::c_int {
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut i: libc::c_uint = 0;
    let mut u_tag: u32 = 0;
    if !sfont.is_null() {
    } else {
        __assert_fail(
            b"sfont\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            355i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"int sfnt_read_table_directory(sfnt *, SFNT_ULONG)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*sfont).directory.is_null() {
        release_directory((*sfont).directory);
    }
    td = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<sfnt_table_directory>() as u64)
        as u32) as *mut sfnt_table_directory;
    (*sfont).directory = td;
    if !(*sfont).handle.is_null() {
    } else {
        __assert_fail(
            b"sfont->handle\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            362i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"int sfnt_read_table_directory(sfnt *, SFNT_ULONG)\x00",
            ))
            .as_ptr(),
        );
    }
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    (*td).version = tt_get_unsigned_quad((*sfont).handle);
    (*td).num_tables = tt_get_unsigned_pair((*sfont).handle);
    (*td).search_range = tt_get_unsigned_pair((*sfont).handle);
    (*td).entry_selector = tt_get_unsigned_pair((*sfont).handle);
    (*td).range_shift = tt_get_unsigned_pair((*sfont).handle);
    (*td).flags = new(((*td).num_tables as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    (*td).tables = new(((*td).num_tables as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<sfnt_table>() as u64)
        as u32) as *mut sfnt_table;
    i = 0i32 as libc::c_uint;
    while i < (*td).num_tables as libc::c_uint {
        u_tag = tt_get_unsigned_quad((*sfont).handle);
        convert_tag((*(*td).tables.offset(i as isize)).tag.as_mut_ptr(), u_tag);
        (*(*td).tables.offset(i as isize)).check_sum = tt_get_unsigned_quad((*sfont).handle);
        (*(*td).tables.offset(i as isize)).offset =
            tt_get_unsigned_quad((*sfont).handle).wrapping_add((*sfont).offset);
        (*(*td).tables.offset(i as isize)).length = tt_get_unsigned_quad((*sfont).handle);
        let ref mut fresh1 = (*(*td).tables.offset(i as isize)).data;
        *fresh1 = 0 as *mut libc::c_char;
        //fprintf(stderr, "[%4s:%x]", td->tables[i].tag, td->tables[i].offset);
        *(*td).flags.offset(i as isize) = 0i32 as libc::c_char;
        i = i.wrapping_add(1)
    }
    (*td).num_kept_tables = 0i32 as USHORT;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_require_table(
    mut sfont: *mut sfnt,
    mut tag: *const libc::c_char,
    mut must_exist: libc::c_int,
) -> libc::c_int {
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: libc::c_int = 0;
    if !sfont.is_null() && !(*sfont).directory.is_null() {
    } else {
        __assert_fail(
            b"sfont && sfont->directory\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            399i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"int sfnt_require_table(sfnt *, const char *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        if must_exist != 0 {
            return -1i32;
        }
    } else {
        let ref mut fresh2 = *(*td).flags.offset(idx as isize);
        *fresh2 = (*fresh2 as libc::c_int | 1i32 << 0i32) as libc::c_char;
        (*td).num_kept_tables = (*td).num_kept_tables.wrapping_add(1)
    }
    return 0i32;
}
/*
 * o All tables begin on four byte boundries, and pad any remaining space
 *   between tables with zeros
 *
 * o Entries in the Table Directory must be sorted in ascending order by tag
 *
 * o The head table contains checksum of the whole font file.
 *   To compute:  first set it to 0, sum the entire font as ULONG,
 *   then store 0xB1B0AFBA - sum.
 */
static mut wbuf: [u8; 1024] = [0; 1024];
static mut padbytes: [u8; 4] = [0; 4];
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

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
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */
/* 16.16-bit signed fixed-point number */
/* table header */
/* table data */
/* Fixed for Win */
/* number of kept tables */
/* keep or omit */
/* sfnt resource */
/* Convert sfnt "fixed" type to double */
/* get_***_*** from numbers.h */
/* table directory */
#[no_mangle]
pub unsafe extern "C" fn sfnt_create_FontFile_stream(mut sfont: *mut sfnt) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut offset: libc::c_int = 0;
    let mut nb_read: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sr: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !sfont.is_null() && !(*sfont).directory.is_null() {
    } else {
        __assert_fail(
            b"sfont && sfont->directory\x00" as *const u8 as *const libc::c_char,
            b"dpx-sfnt.c\x00" as *const u8 as *const libc::c_char,
            439i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"pdf_obj *sfnt_create_FontFile_stream(sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    stream = pdf_new_stream(1i32 << 0i32);
    td = (*sfont).directory;
    /* Header */
    p = wbuf.as_mut_ptr() as *mut libc::c_char;
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*td).version as SFNT_LONG, 4i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*td).num_kept_tables as SFNT_LONG,
        2i32,
    ) as isize);
    sr = max2floor((*td).num_kept_tables as libc::c_uint).wrapping_mul(16i32 as libc::c_uint)
        as libc::c_int;
    p = p.offset(put_big_endian(p as *mut libc::c_void, sr, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        log2floor((*td).num_kept_tables as libc::c_uint) as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*td).num_kept_tables as libc::c_int * 16i32 - sr,
        2i32,
    ) as isize);
    pdf_add_stream(stream, wbuf.as_mut_ptr() as *const libc::c_void, 12i32);
    /*
     * Compute start of actual tables (after headers).
     */
    offset = 12i32 + 16i32 * (*td).num_kept_tables as libc::c_int;
    i = 0i32;
    while i < (*td).num_tables as libc::c_int {
        /* This table must exist in FontFile */
        if *(*td).flags.offset(i as isize) as libc::c_int & 1i32 << 0i32 != 0 {
            if offset % 4i32 != 0i32 {
                offset += 4i32 - offset % 4i32
            }
            p = wbuf.as_mut_ptr() as *mut libc::c_char;
            memcpy(
                p as *mut libc::c_void,
                (*(*td).tables.offset(i as isize)).tag.as_mut_ptr() as *const libc::c_void,
                4i32 as u64,
            );
            p = p.offset(4);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*td).tables.offset(i as isize)).check_sum as SFNT_LONG,
                4i32,
            ) as isize);
            p = p.offset(put_big_endian(p as *mut libc::c_void, offset, 4i32) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*td).tables.offset(i as isize)).length as SFNT_LONG,
                4i32,
            ) as isize);
            pdf_add_stream(stream, wbuf.as_mut_ptr() as *const libc::c_void, 16i32);
            offset = (offset as libc::c_uint)
                .wrapping_add((*(*td).tables.offset(i as isize)).length)
                as libc::c_int as libc::c_int
        }
        i += 1
    }
    offset = 12i32 + 16i32 * (*td).num_kept_tables as libc::c_int;
    i = 0i32;
    while i < (*td).num_tables as libc::c_int {
        if *(*td).flags.offset(i as isize) as libc::c_int & 1i32 << 0i32 != 0 {
            if offset % 4i32 != 0i32 {
                length = 4i32 - offset % 4i32;
                pdf_add_stream(stream, padbytes.as_mut_ptr() as *const libc::c_void, length);
                offset += length
            }
            if (*(*td).tables.offset(i as isize)).data.is_null() {
                if (*sfont).handle.is_null() {
                    pdf_release_obj(stream);
                    _tt_abort(
                        b"Font file not opened or already closed...\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                length = (*(*td).tables.offset(i as isize)).length as libc::c_int;
                ttstub_input_seek(
                    (*sfont).handle,
                    (*(*td).tables.offset(i as isize)).offset as ssize_t,
                    0i32,
                );
                while length > 0i32 {
                    nb_read = ttstub_input_read(
                        (*sfont).handle,
                        wbuf.as_mut_ptr() as *mut libc::c_char,
                        (if length < 1024i32 { length } else { 1024i32 }) as size_t,
                    ) as libc::c_int;
                    if nb_read < 0i32 {
                        pdf_release_obj(stream);
                        _tt_abort(
                            b"Reading file failed...\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        if nb_read > 0i32 {
                            pdf_add_stream(
                                stream,
                                wbuf.as_mut_ptr() as *const libc::c_void,
                                nb_read,
                            );
                        }
                    }
                    length -= nb_read
                }
            } else {
                pdf_add_stream(
                    stream,
                    (*(*td).tables.offset(i as isize)).data as *const libc::c_void,
                    (*(*td).tables.offset(i as isize)).length as libc::c_int,
                );
                let ref mut fresh3 = (*(*td).tables.offset(i as isize)).data;
                *fresh3 = mfree((*(*td).tables.offset(i as isize)).data as *mut libc::c_void)
                    as *mut libc::c_char
            }
            /* Set offset for next table */
            offset = (offset as libc::c_uint)
                .wrapping_add((*(*td).tables.offset(i as isize)).length)
                as libc::c_int as libc::c_int
        }
        i += 1
    }
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Length1\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(offset as libc::c_double),
    );
    return stream;
}
