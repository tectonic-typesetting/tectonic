#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
use crate::dpx_pdfobj::pdf_obj;
use libc::free;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_number(value: f64) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn pdf_new_stream(flags: i32) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: i32,
    );
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
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
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub num_kept_tables: u16,
    pub flags: *mut i8,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: i32,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: u32,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr); /* tag name */
    return 0 as *mut libc::c_void; /* typefaces number */
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_open(mut handle: rust_input_handle_t) -> *mut sfnt {
    let mut sfont: *mut sfnt = 0 as *mut sfnt; /* typefaces position */
    let mut type_0: u32 = 0; /* resource id */
    assert!(!handle.is_null());
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32); /* mbz */
    sfont =
        new((1_u32 as u64).wrapping_mul(::std::mem::size_of::<sfnt>() as u64) as u32) as *mut sfnt;
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
    (*sfont).offset = 0u64 as u32;
    sfont
}
#[no_mangle]
pub unsafe extern "C" fn dfont_open(mut handle: rust_input_handle_t, mut index: i32) -> *mut sfnt {
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut rdata_pos: u32 = 0;
    let mut map_pos: u32 = 0;
    let mut tags_pos: u32 = 0;
    let mut types_pos: u32 = 0;
    let mut res_pos: u32 = 0;
    let mut tag: u32 = 0;
    let mut tags_num: u16 = 0;
    let mut types_num: u16 = 0;
    let mut i: u16 = 0;
    assert!(!handle.is_null());
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    sfont =
        new((1_u32 as u64).wrapping_mul(::std::mem::size_of::<sfnt>() as u64) as u32) as *mut sfnt;
    (*sfont).handle = handle;
    rdata_pos = tt_get_unsigned_quad((*sfont).handle);
    map_pos = tt_get_unsigned_quad((*sfont).handle);
    ttstub_input_seek(
        (*sfont).handle,
        map_pos.wrapping_add(0x18_u32) as ssize_t,
        0i32,
    );
    tags_pos = map_pos.wrapping_add(tt_get_unsigned_pair((*sfont).handle) as u32);
    ttstub_input_seek((*sfont).handle, tags_pos as ssize_t, 0i32);
    tags_num = tt_get_unsigned_pair((*sfont).handle);
    i = 0_u16;
    while i as i32 <= tags_num as i32 {
        tag = tt_get_unsigned_quad((*sfont).handle);
        types_num = tt_get_unsigned_pair((*sfont).handle);
        types_pos = tags_pos.wrapping_add(tt_get_unsigned_pair((*sfont).handle) as u32);
        if tag as u64 == 0x73666e74 {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i as i32 > tags_num as i32 {
        free(sfont as *mut libc::c_void);
        return 0 as *mut sfnt;
    }
    ttstub_input_seek((*sfont).handle, types_pos as ssize_t, 0i32);
    if index > types_num as i32 {
        _tt_abort(
            b"Invalid index %d for dfont.\x00" as *const u8 as *const i8,
            index,
        );
    }
    i = 0_u16;
    while i as i32 <= types_num as i32 {
        tt_get_unsigned_pair((*sfont).handle);
        tt_get_unsigned_pair((*sfont).handle);
        res_pos = tt_get_unsigned_quad((*sfont).handle);
        tt_get_unsigned_quad((*sfont).handle);
        if i as i32 == index {
            break;
        }
        i = i.wrapping_add(1)
    }
    ttstub_input_seek((*sfont).handle, 0i32 as ssize_t, 0i32);
    (*sfont).type_0 = 1i32 << 8i32;
    (*sfont).directory = 0 as *mut sfnt_table_directory;
    (*sfont).offset = (res_pos as u64 & 0xffffff)
        .wrapping_add(rdata_pos as u64)
        .wrapping_add(4i32 as u64) as u32;
    sfont
}
unsafe extern "C" fn release_directory(mut td: *mut sfnt_table_directory) {
    let mut i: u32 = 0;
    if !td.is_null() {
        if !(*td).tables.is_null() {
            i = 0_u32;
            while i < (*td).num_tables as u32 {
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
pub unsafe extern "C" fn put_big_endian(mut s: *mut libc::c_void, mut q: i32, mut n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    p = s as *mut i8;
    i = n - 1i32;
    while i >= 0i32 {
        *p.offset(i as isize) = (q & 0xffi32) as i8;
        q >>= 8i32;
        i -= 1
    }
    n
}
/* Convert four-byte number to big endianess
 * in a machine independent way.
 */
unsafe extern "C" fn convert_tag(mut tag: *mut i8, mut u_tag: u32) {
    let mut i: i32 = 0;
    i = 3i32;
    while i >= 0i32 {
        *tag.offset(i as isize) = u_tag.wrapping_rem(256_u32) as i8;
        u_tag = (u_tag as u32).wrapping_div(256_u32) as u32;
        i -= 1
    }
}
/*
 * Computes the max power of 2 <= n
 */
unsafe extern "C" fn max2floor(mut n: u32) -> u32 {
    let mut val: i32 = 1i32;
    while n > 1_u32 {
        n = n.wrapping_div(2_u32);
        val *= 2i32
    }
    val as u32
}
/*
 * Computes the log2 of the max power of 2 <= n
 */
unsafe extern "C" fn log2floor(mut n: u32) -> u32 {
    let mut val: u32 = 0_u32;
    while n > 1_u32 {
        n = n.wrapping_div(2_u32);
        val = val.wrapping_add(1)
    }
    val
}
unsafe extern "C" fn sfnt_calc_checksum(mut data: *mut libc::c_void, mut length: u32) -> u32 {
    let mut chksum: u32 = 0_u32;
    let mut p: *mut u8 = 0 as *mut u8;
    let mut endptr: *mut u8 = 0 as *mut u8;
    let mut count: i32 = 0i32;
    p = data as *mut u8;
    endptr = p.offset(length as isize);
    while p < endptr {
        chksum = (chksum as u32)
            .wrapping_add(((*p.offset(0) as i32) << 8i32 * (3i32 - count)) as u32)
            as u32 as u32;
        count = count + 1i32 & 3i32;
        p = p.offset(1)
    }
    chksum
}
unsafe extern "C" fn find_table_index(
    mut td: *mut sfnt_table_directory,
    mut tag: *const i8,
) -> i32 {
    let mut idx: i32 = 0;
    if td.is_null() {
        return -1i32;
    }
    idx = 0i32;
    while idx < (*td).num_tables as i32 {
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
    -1i32
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_set_table(
    mut sfont: *mut sfnt,
    mut tag: *const i8,
    mut data: *mut libc::c_void,
    mut length: u32,
) {
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: i32 = 0;
    assert!(!sfont.is_null());
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        idx = (*td).num_tables as i32;
        (*td).num_tables = (*td).num_tables.wrapping_add(1);
        (*td).tables = renew(
            (*td).tables as *mut libc::c_void,
            ((*td).num_tables as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<sfnt_table>() as u64) as u32,
        ) as *mut sfnt_table;
        memcpy(
            (*(*td).tables.offset(idx as isize)).tag.as_mut_ptr() as *mut libc::c_void,
            tag as *const libc::c_void,
            4i32 as u64,
        );
    }
    (*(*td).tables.offset(idx as isize)).check_sum = sfnt_calc_checksum(data, length);
    (*(*td).tables.offset(idx as isize)).offset = 0i64 as u32;
    (*(*td).tables.offset(idx as isize)).length = length;
    let ref mut fresh0 = (*(*td).tables.offset(idx as isize)).data;
    *fresh0 = data as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_find_table_len(mut sfont: *mut sfnt, mut tag: *const i8) -> u32 {
    let mut length: u32 = 0;
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: i32 = 0;
    assert!(!sfont.is_null() && !tag.is_null());
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        length = 0_u32
    } else {
        length = (*(*td).tables.offset(idx as isize)).length
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_find_table_pos(mut sfont: *mut sfnt, mut tag: *const i8) -> u32 {
    let mut offset: u32 = 0;
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: i32 = 0;
    assert!(!sfont.is_null() && !tag.is_null());
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        offset = 0_u32
    } else {
        offset = (*(*td).tables.offset(idx as isize)).offset
    }
    offset
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_locate_table(mut sfont: *mut sfnt, mut tag: *const i8) -> u32 {
    let mut offset: u32 = 0;
    assert!(!sfont.is_null() && !tag.is_null());
    offset = sfnt_find_table_pos(sfont, tag);
    if offset == 0_u32 {
        _tt_abort(b"sfnt: table not found...\x00" as *const u8 as *const i8);
    }
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    offset
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_read_table_directory(mut sfont: *mut sfnt, mut offset: u32) -> i32 {
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut i: u32 = 0;
    let mut u_tag: u32 = 0;
    assert!(!sfont.is_null());
    if !(*sfont).directory.is_null() {
        release_directory((*sfont).directory);
    }
    td = new(
        (1_u32 as u64).wrapping_mul(::std::mem::size_of::<sfnt_table_directory>() as u64) as u32,
    ) as *mut sfnt_table_directory;
    (*sfont).directory = td;
    assert!(!(*sfont).handle.is_null());
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    (*td).version = tt_get_unsigned_quad((*sfont).handle);
    (*td).num_tables = tt_get_unsigned_pair((*sfont).handle);
    (*td).search_range = tt_get_unsigned_pair((*sfont).handle);
    (*td).entry_selector = tt_get_unsigned_pair((*sfont).handle);
    (*td).range_shift = tt_get_unsigned_pair((*sfont).handle);
    (*td).flags = new(
        ((*td).num_tables as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32
    ) as *mut i8;
    (*td).tables = new(((*td).num_tables as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<sfnt_table>() as u64) as u32)
        as *mut sfnt_table;
    i = 0_u32;
    while i < (*td).num_tables as u32 {
        u_tag = tt_get_unsigned_quad((*sfont).handle);
        convert_tag((*(*td).tables.offset(i as isize)).tag.as_mut_ptr(), u_tag);
        (*(*td).tables.offset(i as isize)).check_sum = tt_get_unsigned_quad((*sfont).handle);
        (*(*td).tables.offset(i as isize)).offset =
            tt_get_unsigned_quad((*sfont).handle).wrapping_add((*sfont).offset);
        (*(*td).tables.offset(i as isize)).length = tt_get_unsigned_quad((*sfont).handle);
        let ref mut fresh1 = (*(*td).tables.offset(i as isize)).data;
        *fresh1 = 0 as *mut i8;
        //fprintf(stderr, "[%4s:%x]", td->tables[i].tag, td->tables[i].offset);
        *(*td).flags.offset(i as isize) = 0_i8;
        i = i.wrapping_add(1)
    }
    (*td).num_kept_tables = 0_u16;
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn sfnt_require_table(
    mut sfont: *mut sfnt,
    mut tag: *const i8,
    mut must_exist: i32,
) -> i32 {
    let mut td: *mut sfnt_table_directory = 0 as *mut sfnt_table_directory;
    let mut idx: i32 = 0;
    assert!(!sfont.is_null() && !(*sfont).directory.is_null());
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0i32 {
        if must_exist != 0 {
            return -1i32;
        }
    } else {
        let ref mut fresh2 = *(*td).flags.offset(idx as isize);
        *fresh2 = (*fresh2 as i32 | 1i32 << 0i32) as i8;
        (*td).num_kept_tables = (*td).num_kept_tables.wrapping_add(1)
    }
    0i32
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
    let mut offset: i32 = 0;
    let mut nb_read: i32 = 0;
    let mut length: i32 = 0;
    let mut i: i32 = 0;
    let mut sr: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    assert!(!sfont.is_null() && !(*sfont).directory.is_null());
    stream = pdf_new_stream(1i32 << 0i32);
    td = (*sfont).directory;
    /* Header */
    p = wbuf.as_mut_ptr() as *mut i8;
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*td).version as i32, 4i32) as isize);
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*td).num_kept_tables as i32, 2i32) as isize,
    );
    sr = max2floor((*td).num_kept_tables as u32).wrapping_mul(16_u32) as i32;
    p = p.offset(put_big_endian(p as *mut libc::c_void, sr, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        log2floor((*td).num_kept_tables as u32) as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*td).num_kept_tables as i32 * 16i32 - sr,
        2i32,
    ) as isize);
    pdf_add_stream(stream, wbuf.as_mut_ptr() as *const libc::c_void, 12i32);
    /*
     * Compute start of actual tables (after headers).
     */
    offset = 12i32 + 16i32 * (*td).num_kept_tables as i32;
    i = 0i32;
    while i < (*td).num_tables as i32 {
        /* This table must exist in FontFile */
        if *(*td).flags.offset(i as isize) as i32 & 1i32 << 0i32 != 0 {
            if offset % 4i32 != 0i32 {
                offset += 4i32 - offset % 4i32
            }
            p = wbuf.as_mut_ptr() as *mut i8;
            memcpy(
                p as *mut libc::c_void,
                (*(*td).tables.offset(i as isize)).tag.as_mut_ptr() as *const libc::c_void,
                4i32 as u64,
            );
            p = p.offset(4);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*td).tables.offset(i as isize)).check_sum as i32,
                4i32,
            ) as isize);
            p = p.offset(put_big_endian(p as *mut libc::c_void, offset, 4i32) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*td).tables.offset(i as isize)).length as i32,
                4i32,
            ) as isize);
            pdf_add_stream(stream, wbuf.as_mut_ptr() as *const libc::c_void, 16i32);
            offset = (offset as u32).wrapping_add((*(*td).tables.offset(i as isize)).length) as i32
                as i32
        }
        i += 1
    }
    offset = 12i32 + 16i32 * (*td).num_kept_tables as i32;
    i = 0i32;
    while i < (*td).num_tables as i32 {
        if *(*td).flags.offset(i as isize) as i32 & 1i32 << 0i32 != 0 {
            if offset % 4i32 != 0i32 {
                length = 4i32 - offset % 4i32;
                pdf_add_stream(stream, padbytes.as_mut_ptr() as *const libc::c_void, length);
                offset += length
            }
            if (*(*td).tables.offset(i as isize)).data.is_null() {
                if (*sfont).handle.is_null() {
                    pdf_release_obj(stream);
                    _tt_abort(
                        b"Font file not opened or already closed...\x00" as *const u8 as *const i8,
                    );
                }
                length = (*(*td).tables.offset(i as isize)).length as i32;
                ttstub_input_seek(
                    (*sfont).handle,
                    (*(*td).tables.offset(i as isize)).offset as ssize_t,
                    0i32,
                );
                while length > 0i32 {
                    nb_read = ttstub_input_read(
                        (*sfont).handle,
                        wbuf.as_mut_ptr() as *mut i8,
                        (if length < 1024i32 { length } else { 1024i32 }) as size_t,
                    ) as i32;
                    if nb_read < 0i32 {
                        pdf_release_obj(stream);
                        _tt_abort(b"Reading file failed...\x00" as *const u8 as *const i8);
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
                    (*(*td).tables.offset(i as isize)).length as i32,
                );
                let ref mut fresh3 = (*(*td).tables.offset(i as isize)).data;
                *fresh3 =
                    mfree((*(*td).tables.offset(i as isize)).data as *mut libc::c_void) as *mut i8
            }
            /* Set offset for next table */
            offset = (offset as u32).wrapping_add((*(*td).tables.offset(i as isize)).length) as i32
                as i32
        }
        i += 1
    }
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Length1\x00" as *const u8 as *const i8),
        pdf_new_number(offset as f64),
    );
    stream
}
