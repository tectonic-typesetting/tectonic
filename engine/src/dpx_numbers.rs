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
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type fixword = i32;
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
pub unsafe extern "C" fn get_unsigned_byte(mut file: *mut FILE) -> u8 {
    let mut ch: libc::c_int = 0;
    ch = fgetc(file);
    if ch < 0i32 {
        _tt_abort(b"File ended prematurely\n\x00" as *const u8 as *const i8);
    }
    return ch as u8;
}
#[no_mangle]
pub unsafe extern "C" fn skip_bytes(mut n: libc::c_uint, mut file: *mut FILE) {
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 > 0i32 as libc::c_uint) {
            break;
        }
        get_unsigned_byte(file);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_signed_byte(mut file: *mut FILE) -> libc::c_schar {
    let mut byte: libc::c_int = 0;
    byte = get_unsigned_byte(file) as libc::c_int;
    if byte >= 0x80i32 {
        byte -= 0x100i32
    }
    return byte as libc::c_schar;
}
#[no_mangle]
pub unsafe extern "C" fn get_unsigned_pair(mut file: *mut FILE) -> u16 {
    let mut pair: u16 = get_unsigned_byte(file) as u16;
    pair =
        ((pair as libc::c_int) << 8i32 | get_unsigned_byte(file) as libc::c_int) as u16;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn sget_unsigned_pair(mut s: *mut u8) -> u16 {
    let fresh1 = s;
    s = s.offset(1);
    let mut pair: u16 = *fresh1 as u16;
    let fresh2 = s;
    s = s.offset(1);
    pair = ((pair as libc::c_int) << 8i32 | *fresh2 as libc::c_int) as u16;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn get_signed_pair(mut file: *mut FILE) -> libc::c_short {
    let mut pair: libc::c_short = get_signed_byte(file) as libc::c_short;
    pair =
        ((pair as libc::c_int) << 8i32 | get_unsigned_byte(file) as libc::c_int) as libc::c_short;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn get_unsigned_triple(mut file: *mut FILE) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut triple: libc::c_uint = 0i32 as libc::c_uint;
    i = 0i32;
    while i < 3i32 {
        triple = triple << 8i32 | get_unsigned_byte(file) as libc::c_uint;
        i += 1
    }
    return triple;
}
#[no_mangle]
pub unsafe extern "C" fn get_signed_triple(mut file: *mut FILE) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut triple: libc::c_int = get_signed_byte(file) as libc::c_int;
    i = 0i32;
    while i < 2i32 {
        triple = triple << 8i32 | get_unsigned_byte(file) as libc::c_int;
        i += 1
    }
    return triple;
}
#[no_mangle]
pub unsafe extern "C" fn get_signed_quad(mut file: *mut FILE) -> i32 {
    let mut i: libc::c_int = 0;
    let mut quad: i32 = get_signed_byte(file) as i32;
    i = 0i32;
    while i < 3i32 {
        quad = quad << 8i32 | get_unsigned_byte(file) as libc::c_int;
        i += 1
    }
    return quad;
}
#[no_mangle]
pub unsafe extern "C" fn get_unsigned_quad(mut file: *mut FILE) -> u32 {
    let mut i: libc::c_int = 0;
    let mut quad = 0u32;
    i = 0i32;
    while i < 4i32 {
        quad = quad << 8i32 | get_unsigned_byte(file) as libc::c_uint;
        i += 1
    }
    return quad;
}
#[no_mangle]
pub unsafe extern "C" fn get_unsigned_num(mut file: *mut FILE, mut num: u8) -> u32 {
    let mut val = get_unsigned_byte(file) as u32;
    let mut current_block_4: u64;
    match num as libc::c_int {
        3 => {
            if val > 0x7fi32 as libc::c_uint {
                val = (val as libc::c_uint).wrapping_sub(0x100i32 as libc::c_uint) as u32
            }
            val = val << 8i32 | get_unsigned_byte(file) as libc::c_uint;
            current_block_4 = 10942825333195857913;
        }
        2 => {
            current_block_4 = 10942825333195857913;
        }
        1 => {
            current_block_4 = 17819358871496454702;
        }
        _ => {
            current_block_4 = 7815301370352969686;
        }
    }
    match current_block_4 {
        10942825333195857913 => {
            val = val << 8i32 | get_unsigned_byte(file) as libc::c_uint;
            current_block_4 = 17819358871496454702;
        }
        _ => {}
    }
    match current_block_4 {
        17819358871496454702 => val = val << 8i32 | get_unsigned_byte(file) as libc::c_uint,
        _ => {}
    }
    return val;
}
/* Compute a signed quad that must be positive */
#[no_mangle]
pub unsafe extern "C" fn get_positive_quad(
    mut file: *mut FILE,
    mut type_0: *const i8,
    mut name: *const i8,
) -> u32 {
    let mut val: i32 = get_signed_quad(file);
    if val < 0i32 {
        _tt_abort(
            b"Bad %s: negative %s: %d\x00" as *const u8 as *const i8,
            type_0,
            name,
            val,
        );
    }
    return val as u32;
}
#[no_mangle]
pub unsafe extern "C" fn sqxfw(mut sq: i32, mut fw: fixword) -> i32 {
    let mut sign: libc::c_int = 1i32;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut ad: u32 = 0;
    let mut bd: u32 = 0;
    let mut bc: u32 = 0;
    let mut ac: u32 = 0;
    let mut e: u32 = 0;
    let mut f: u32 = 0;
    let mut g: u32 = 0;
    let mut h: u32 = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut k: u32 = 0;
    let mut result: i32 = 0;
    /* Make positive. */
    if sq < 0i32 {
        sign = -sign; /* 1<<3 is for rounding */
        sq = -sq
    }
    if fw < 0i32 {
        sign = -sign;
        fw = -fw
    }
    a = sq as u32 >> 16i32;
    b = sq as u32 & 0xffffu32;
    c = fw as u32 >> 16i32;
    d = fw as u32 & 0xffffu32;
    ad = a.wrapping_mul(d);
    bd = b.wrapping_mul(d);
    bc = b.wrapping_mul(c);
    ac = a.wrapping_mul(c);
    e = bd >> 16i32;
    f = ad >> 16i32;
    g = ad & 0xffffu32;
    h = bc >> 16i32;
    i = bc & 0xffffu32;
    j = ac >> 16i32;
    k = ac & 0xffffu32;
    result = (e
        .wrapping_add(g)
        .wrapping_add(i)
        .wrapping_add((1i32 << 3i32) as libc::c_uint)
        >> 4i32) as i32;
    result = (result as libc::c_uint).wrapping_add(f.wrapping_add(h).wrapping_add(k) << 12i32)
        as i32 as i32;
    result = (result as libc::c_uint).wrapping_add(j << 28i32) as i32 as i32;
    return if sign > 0i32 { result } else { -result };
}
/* Tectonic-ified versions */
#[no_mangle]
pub unsafe extern "C" fn tt_skip_bytes(mut n: libc::c_uint, mut handle: rust_input_handle_t) {
    loop {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 > 0i32 as libc::c_uint) {
            break;
        }
        tt_get_unsigned_byte(handle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_unsigned_byte(mut handle: rust_input_handle_t) -> u8 {
    let mut ch: libc::c_int = 0;
    ch = ttstub_input_getc(handle);
    if ch < 0i32 {
        _tt_abort(b"File ended prematurely\n\x00" as *const u8 as *const i8);
    }
    return ch as u8;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_signed_byte(mut handle: rust_input_handle_t) -> libc::c_schar {
    let mut byte: libc::c_int = 0;
    byte = tt_get_unsigned_byte(handle) as libc::c_int;
    if byte >= 0x80i32 {
        byte -= 0x100i32
    }
    return byte as libc::c_schar;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_unsigned_pair(mut handle: rust_input_handle_t) -> u16 {
    let mut pair: u16 = tt_get_unsigned_byte(handle) as u16;
    pair = ((pair as libc::c_int) << 8i32 | tt_get_unsigned_byte(handle) as libc::c_int)
        as u16;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_signed_pair(mut handle: rust_input_handle_t) -> libc::c_short {
    let mut pair: libc::c_short = tt_get_signed_byte(handle) as libc::c_short;
    pair = ((pair as libc::c_int) << 8i32 | tt_get_unsigned_byte(handle) as libc::c_int)
        as libc::c_short;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_unsigned_quad(mut handle: rust_input_handle_t) -> u32 {
    let mut i: libc::c_int = 0;
    let mut quad: u32 = 0i32 as u32;
    i = 0i32;
    while i < 4i32 {
        quad = quad << 8i32 | tt_get_unsigned_byte(handle) as libc::c_uint;
        i += 1
    }
    return quad;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_signed_quad(mut handle: rust_input_handle_t) -> i32 {
    let mut i: libc::c_int = 0;
    let mut quad: i32 = tt_get_signed_byte(handle) as i32;
    i = 0i32;
    while i < 3i32 {
        quad = quad << 8i32 | tt_get_unsigned_byte(handle) as libc::c_int;
        i += 1
    }
    return quad;
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_unsigned_num(
    mut handle: rust_input_handle_t,
    mut num: u8,
) -> u32 {
    let mut val: u32 = tt_get_unsigned_byte(handle) as u32;
    let mut current_block_4: u64;
    match num as libc::c_int {
        3 => {
            if val > 0x7fi32 as libc::c_uint {
                val = (val as libc::c_uint).wrapping_sub(0x100i32 as libc::c_uint) as u32
                    as u32
            }
            val = val << 8i32 | tt_get_unsigned_byte(handle) as libc::c_uint;
            current_block_4 = 13589375657124263157;
        }
        2 => {
            current_block_4 = 13589375657124263157;
        }
        1 => {
            current_block_4 = 17178013025578009494;
        }
        _ => {
            current_block_4 = 7815301370352969686;
        }
    }
    match current_block_4 {
        13589375657124263157 =>
        /* fall through */
        {
            val = val << 8i32 | tt_get_unsigned_byte(handle) as libc::c_uint;
            current_block_4 = 17178013025578009494;
        }
        _ => {}
    }
    match current_block_4 {
        17178013025578009494 =>
        /* fall through */
        {
            val = val << 8i32 | tt_get_unsigned_byte(handle) as libc::c_uint
        }
        _ => {}
    }
    return val;
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
/* When reading numbers from binary files 1, 2, or 3 bytes are
   interpreted as either signed or unsigned.

   Four bytes from DVI, PK, TFM, or VF files always yield a signed
   32-bit integer (i32), but some of them must not be negative.

   Four byte numbers from JPEG2000, OpenType, or TrueType files are
   mostly unsigned (u32) and occasionally signed (i32).
*/
/* Tectonic enabled */
#[no_mangle]
pub unsafe extern "C" fn tt_get_positive_quad(
    mut handle: rust_input_handle_t,
    mut type_0: *const i8,
    mut name: *const i8,
) -> u32 {
    let mut val: i32 = tt_get_signed_quad(handle);
    if val < 0i32 {
        _tt_abort(
            b"Bad %s: negative %s: %d\x00" as *const u8 as *const i8,
            type_0,
            name,
            val,
        );
    }
    return val as u32;
}
