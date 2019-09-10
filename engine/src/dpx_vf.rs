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
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
        format: tt_input_format_type,
        is_gz: i32,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    /* Tectonic enabled */
    #[no_mangle]
    fn tt_skip_bytes(n: u32, handle: rust_input_handle_t);
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn tt_get_unsigned_num(handle: rust_input_handle_t, num: u8) -> u32;
    #[no_mangle]
    fn tt_get_positive_quad(handle: rust_input_handle_t, type_0: *const i8, name: *const i8)
        -> u32;
    #[no_mangle]
    fn sqxfw(sq: i32, fw: fixword) -> i32;
    #[no_mangle]
    fn dvi_set_font(font_id: i32);
    #[no_mangle]
    fn dvi_set(ch: i32);
    #[no_mangle]
    fn dvi_put(ch: i32);
    #[no_mangle]
    fn dvi_dirchg(dir: u8);
    #[no_mangle]
    fn dvi_do_special(buffer: *const libc::c_void, size: i32);
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
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dvi_z(ch: i32);
    #[no_mangle]
    fn dvi_z0();
    #[no_mangle]
    fn dvi_vf_init(dev_font_id: i32);
    #[no_mangle]
    fn dvi_y(ch: i32);
    #[no_mangle]
    fn dvi_push();
    #[no_mangle]
    fn dpx_dvi_pop();
    #[no_mangle]
    fn dvi_y0();
    #[no_mangle]
    fn dvi_locate_font(name: *const i8, ptsize: spt_t) -> u32;
    #[no_mangle]
    fn dvi_vf_finish();
    #[no_mangle]
    fn dvi_down(y: i32);
    #[no_mangle]
    fn dvi_right(x: i32);
    #[no_mangle]
    fn dvi_x(ch: i32);
    #[no_mangle]
    fn dvi_w0();
    #[no_mangle]
    fn dvi_x0();
    #[no_mangle]
    fn dvi_rule(width: i32, height: i32);
    #[no_mangle]
    fn dvi_w(ch: i32);
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
    #[no_mangle]
    fn tfm_open(tex_name: *const i8, must_exist: i32) -> i32;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = u32;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
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
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type fixword = i32;
pub type spt_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf {
    pub tex_name: *mut i8,
    pub ptsize: spt_t,
    pub design_size: u32,
    pub num_dev_fonts: u32,
    pub max_dev_fonts: u32,
    pub dev_fonts: *mut font_def,
    pub ch_pkt: *mut *mut u8,
    pub pkt_len: *mut u32,
    pub num_chars: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_def {
    pub font_id: i32,
    pub checksum: u32,
    pub size: u32,
    pub design_size: u32,
    pub directory: *mut i8,
    pub name: *mut i8,
    pub tfm_id: i32,
    pub dev_id: i32,
    /* quasi-hack to get the primary input */
    /* id returned by DEV module */
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
static mut verbose: u8 = 0_u8;
#[no_mangle]
pub unsafe extern "C" fn vf_set_verbose(mut level: i32) {
    verbose = level as u8;
}
static mut vf_fonts: *mut vf = 0 as *const vf as *mut vf;
static mut num_vf_fonts: u32 = 0_u32;
static mut max_vf_fonts: u32 = 0_u32;
#[no_mangle]
pub unsafe extern "C" fn vf_reset_global_state() {
    num_vf_fonts = 0_u32;
    max_vf_fonts = 0_u32;
    vf_fonts = 0 as *mut vf;
}
unsafe extern "C" fn read_header(mut vf_handle: rust_input_handle_t, mut thisfont: i32) {
    if tt_get_unsigned_byte(vf_handle) as i32 != 247i32
        || tt_get_unsigned_byte(vf_handle) as i32 != 202i32
    {
        fprintf(
            stderr,
            b"VF file may be corrupt\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    /* skip comment */
    tt_skip_bytes(tt_get_unsigned_byte(vf_handle) as u32, vf_handle);
    /* Skip checksum */
    tt_skip_bytes(4_u32, vf_handle);
    (*vf_fonts.offset(thisfont as isize)).design_size = tt_get_positive_quad(
        vf_handle,
        b"VF\x00" as *const u8 as *const i8,
        b"design_size\x00" as *const u8 as *const i8,
    );
}
unsafe extern "C" fn resize_vf_fonts(mut size: i32) {
    let mut i: i32 = 0;
    if size as u32 > max_vf_fonts {
        vf_fonts = renew(
            vf_fonts as *mut libc::c_void,
            (size as u32 as u64).wrapping_mul(::std::mem::size_of::<vf>() as u64) as u32,
        ) as *mut vf;
        i = max_vf_fonts as i32;
        while i < size {
            (*vf_fonts.offset(i as isize)).num_dev_fonts = 0_u32;
            (*vf_fonts.offset(i as isize)).max_dev_fonts = 0_u32;
            let ref mut fresh0 = (*vf_fonts.offset(i as isize)).dev_fonts;
            *fresh0 = 0 as *mut font_def;
            i += 1
        }
        max_vf_fonts = size as u32
    };
}
unsafe extern "C" fn resize_one_vf_font(mut a_vf: *mut vf, mut size: u32) {
    let mut i: u32 = 0;
    if size > (*a_vf).num_chars {
        size = if size > (*a_vf).num_chars.wrapping_add(256_u32) {
            size
        } else {
            (*a_vf).num_chars.wrapping_add(256_u32)
        };
        (*a_vf).ch_pkt = renew(
            (*a_vf).ch_pkt as *mut libc::c_void,
            (size as u64).wrapping_mul(::std::mem::size_of::<*mut u8>() as u64) as u32,
        ) as *mut *mut u8;
        (*a_vf).pkt_len = renew(
            (*a_vf).pkt_len as *mut libc::c_void,
            (size as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64) as u32,
        ) as *mut u32;
        i = (*a_vf).num_chars;
        while i < size {
            let ref mut fresh1 = *(*a_vf).ch_pkt.offset(i as isize);
            *fresh1 = 0 as *mut u8;
            *(*a_vf).pkt_len.offset(i as isize) = 0_u32;
            i = i.wrapping_add(1)
        }
        (*a_vf).num_chars = size
    };
}
unsafe extern "C" fn read_a_char_def(
    mut vf_handle: rust_input_handle_t,
    mut thisfont: i32,
    mut pkt_len: u32,
    mut ch: u32,
) {
    let mut pkt: *mut u8 = 0 as *mut u8;
    /* Resize and initialize character arrays if necessary */
    if ch >= (*vf_fonts.offset(thisfont as isize)).num_chars {
        resize_one_vf_font(vf_fonts.offset(thisfont as isize), ch.wrapping_add(1_u32));
        /* must exist */
    }
    if pkt_len > 0_u32 {
        pkt = new((pkt_len as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        if ttstub_input_read(vf_handle, pkt as *mut i8, pkt_len as size_t) != pkt_len as i64 {
            _tt_abort(b"VF file ended prematurely.\x00" as *const u8 as *const i8);
        }
        let ref mut fresh2 = *(*vf_fonts.offset(thisfont as isize))
            .ch_pkt
            .offset(ch as isize);
        *fresh2 = pkt
    }
    *(*vf_fonts.offset(thisfont as isize))
        .pkt_len
        .offset(ch as isize) = pkt_len;
}
unsafe extern "C" fn read_a_font_def(
    mut vf_handle: rust_input_handle_t,
    mut font_id: i32,
    mut thisfont: i32,
) {
    let mut dev_font: *mut font_def = 0 as *mut font_def;
    let mut dir_length: i32 = 0;
    let mut name_length: i32 = 0;
    if (*vf_fonts.offset(thisfont as isize)).num_dev_fonts
        >= (*vf_fonts.offset(thisfont as isize)).max_dev_fonts
    {
        let ref mut fresh3 = (*vf_fonts.offset(thisfont as isize)).max_dev_fonts;
        *fresh3 = (*fresh3).wrapping_add(16u32);
        let ref mut fresh4 = (*vf_fonts.offset(thisfont as isize)).dev_fonts;
        *fresh4 = renew(
            (*vf_fonts.offset(thisfont as isize)).dev_fonts as *mut libc::c_void,
            ((*vf_fonts.offset(thisfont as isize)).max_dev_fonts as u64)
                .wrapping_mul(::std::mem::size_of::<font_def>() as u64) as u32,
        ) as *mut font_def
    }
    dev_font = (*vf_fonts.offset(thisfont as isize))
        .dev_fonts
        .offset((*vf_fonts.offset(thisfont as isize)).num_dev_fonts as isize);
    (*dev_font).font_id = font_id;
    (*dev_font).checksum = tt_get_unsigned_quad(vf_handle);
    (*dev_font).size = tt_get_positive_quad(
        vf_handle,
        b"VF\x00" as *const u8 as *const i8,
        b"font_size\x00" as *const u8 as *const i8,
    );
    (*dev_font).design_size = tt_get_positive_quad(
        vf_handle,
        b"VF\x00" as *const u8 as *const i8,
        b"font_design_size\x00" as *const u8 as *const i8,
    );
    dir_length = tt_get_unsigned_byte(vf_handle) as i32;
    name_length = tt_get_unsigned_byte(vf_handle) as i32;
    (*dev_font).directory = new(
        ((dir_length + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32
    ) as *mut i8;
    if ttstub_input_read(vf_handle, (*dev_font).directory, dir_length as size_t)
        != dir_length as i64
    {
        _tt_abort(b"directory read failed\x00" as *const u8 as *const i8);
    }
    (*dev_font).name = new(((name_length + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    if ttstub_input_read(vf_handle, (*dev_font).name, name_length as size_t) != name_length as i64 {
        _tt_abort(b"directory read failed\x00" as *const u8 as *const i8);
    }
    *(*dev_font).directory.offset(dir_length as isize) = 0_i8;
    *(*dev_font).name.offset(name_length as isize) = 0_i8;
    let ref mut fresh5 = (*vf_fonts.offset(thisfont as isize)).num_dev_fonts;
    *fresh5 = (*fresh5).wrapping_add(1_u32);
    (*dev_font).tfm_id = tfm_open((*dev_font).name, 1i32);
    (*dev_font).dev_id = dvi_locate_font(
        (*dev_font).name,
        sqxfw(
            (*vf_fonts.offset(thisfont as isize)).ptsize,
            (*dev_font).size as fixword,
        ),
    ) as i32;
}
unsafe extern "C" fn process_vf_file(mut vf_handle: rust_input_handle_t, mut thisfont: i32) {
    let mut eof: i32 = 0i32;
    let mut code: i32 = 0;
    let mut font_id: u32 = 0;
    while eof == 0 {
        code = tt_get_unsigned_byte(vf_handle) as i32;
        match code {
            243 | 244 | 245 | 246 => {
                font_id = tt_get_unsigned_num(vf_handle, (code - 243i32) as u8);
                read_a_font_def(vf_handle, font_id as i32, thisfont);
            }
            _ => {
                if code < 242i32 {
                    /* For a short packet, code is the pkt_len */
                    let mut ch: u32 = tt_get_unsigned_byte(vf_handle) as u32;
                    /* Skip over TFM width since we already know it */
                    tt_skip_bytes(3_u32, vf_handle);
                    read_a_char_def(vf_handle, thisfont, code as u32, ch);
                } else if code == 242i32 {
                    let mut pkt_len: u32 = tt_get_positive_quad(
                        vf_handle,
                        b"VF\x00" as *const u8 as *const i8,
                        b"pkt_len\x00" as *const u8 as *const i8,
                    );
                    let mut ch_0: u32 = tt_get_unsigned_quad(vf_handle);
                    /* Skip over TFM width since we already know it */
                    tt_skip_bytes(4_u32, vf_handle);
                    if ch_0 < 0x1000000u32 {
                        read_a_char_def(vf_handle, thisfont, pkt_len, ch_0);
                    } else {
                        fprintf(stderr, b"char=%u\n\x00" as *const u8 as *const i8, ch_0);
                        _tt_abort(b"Long character (>24 bits) in VF file.\nI can\'t handle long characters!\n\x00"
                                      as *const u8 as *const i8);
                    }
                } else if code == 248i32 {
                    eof = 1i32
                } else {
                    fprintf(
                        stderr,
                        b"Quitting on code=%d\n\x00" as *const u8 as *const i8,
                        code,
                    );
                    eof = 1i32
                }
            }
        }
    }
}
/* Unfortunately, the following code isn't smart enough
to load the vf only once for multiple point sizes.
You will get a separate copy of each VF in memory (and a separate
opening and reading of the file) for
each point size.  Since VFs are pretty small, I guess
this is tolerable for now.  In any case,
the PDF file will never repeat a physical font name */
/* Note: This code needs to be able to recurse */
/* Global variables such as num_vf_fonts require careful attention */
#[no_mangle]
pub unsafe extern "C" fn vf_locate_font(mut tex_name: *const i8, mut ptsize: spt_t) -> i32 {
    let mut thisfont: i32 = -1i32;
    let mut i: i32 = 0;
    let mut vf_handle: rust_input_handle_t = 0 as *mut libc::c_void;
    /* Has this name and ptsize already been loaded as a VF? */
    i = 0i32;
    while (i as u32) < num_vf_fonts {
        if streq_ptr((*vf_fonts.offset(i as isize)).tex_name, tex_name) as i32 != 0
            && (*vf_fonts.offset(i as isize)).ptsize == ptsize
        {
            break;
        }
        i += 1
    }
    if i as u32 != num_vf_fonts {
        return i;
    }
    vf_handle = ttstub_input_open(tex_name, TTIF_VF, 0i32);
    if vf_handle.is_null() {
        vf_handle = ttstub_input_open(tex_name, TTIF_OVF, 0i32)
    }
    if vf_handle.is_null() {
        return -1i32;
    }
    if verbose as i32 == 1i32 {
        fprintf(stderr, b"(VF:%s\x00" as *const u8 as *const i8, tex_name);
    }
    if num_vf_fonts >= max_vf_fonts {
        resize_vf_fonts(max_vf_fonts.wrapping_add(16u32) as i32);
    }
    let fresh6 = num_vf_fonts;
    num_vf_fonts = num_vf_fonts.wrapping_add(1);
    thisfont = fresh6 as i32;
    /* Initialize some pointers and such */
    let ref mut fresh7 = (*vf_fonts.offset(thisfont as isize)).tex_name;
    *fresh7 = new((strlen(tex_name).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*vf_fonts.offset(thisfont as isize)).tex_name, tex_name);
    (*vf_fonts.offset(thisfont as isize)).ptsize = ptsize;
    (*vf_fonts.offset(thisfont as isize)).num_chars = 0_u32;
    let ref mut fresh8 = (*vf_fonts.offset(thisfont as isize)).ch_pkt;
    *fresh8 = 0 as *mut *mut u8;
    let ref mut fresh9 = (*vf_fonts.offset(thisfont as isize)).pkt_len;
    *fresh9 = 0 as *mut u32;
    read_header(vf_handle, thisfont);
    process_vf_file(vf_handle, thisfont);
    if verbose != 0 {
        fprintf(stderr, b")\x00" as *const u8 as *const i8);
    }
    ttstub_input_close(vf_handle);
    return thisfont;
}
unsafe extern "C" fn unsigned_byte(mut start: *mut *mut u8, mut end: *mut u8) -> i32 {
    let mut byte: i32 = 0i32;
    if *start < end {
        let fresh10 = *start;
        *start = (*start).offset(1);
        byte = *fresh10 as i32
    } else {
        _tt_abort(b"Premature end of DVI byte stream in VF font\n\x00" as *const u8 as *const i8);
    }
    return byte;
}
unsafe extern "C" fn get_pkt_signed_num(
    mut start: *mut *mut u8,
    mut end: *mut u8,
    mut num: u8,
) -> i32 {
    let mut val: i32 = 0i32;
    if end.wrapping_offset_from(*start) as i64 > num as i64 {
        let fresh11 = *start;
        *start = (*start).offset(1);
        val = *fresh11 as i32;
        if val > 0x7fi32 {
            val -= 0x100i32
        }
        let mut current_block_5: u64;
        match num as i32 {
            3 => {
                let fresh12 = *start;
                *start = (*start).offset(1);
                val = val << 8i32 | *fresh12 as i32;
                current_block_5 = 9698575669066167445;
            }
            2 => {
                current_block_5 = 9698575669066167445;
            }
            1 => {
                current_block_5 = 18113473374131038547;
            }
            _ => {
                current_block_5 = 13183875560443969876;
            }
        }
        match current_block_5 {
            9698575669066167445 => {
                let fresh13 = *start;
                *start = (*start).offset(1);
                val = val << 8i32 | *fresh13 as i32;
                current_block_5 = 18113473374131038547;
            }
            _ => {}
        }
        match current_block_5 {
            18113473374131038547 => {
                let fresh14 = *start;
                *start = (*start).offset(1);
                val = val << 8i32 | *fresh14 as i32
            }
            _ => {}
        }
    } else {
        _tt_abort(b"Premature end of DVI byte stream in VF font\n\x00" as *const u8 as *const i8);
    }
    return val;
}
unsafe extern "C" fn get_pkt_unsigned_num(
    mut start: *mut *mut u8,
    mut end: *mut u8,
    mut num: u8,
) -> i32 {
    let mut val: i32 = 0i32;
    if end.wrapping_offset_from(*start) as i64 > num as i64 {
        let fresh15 = *start;
        *start = (*start).offset(1);
        val = *fresh15 as i32;
        let mut current_block_5: u64;
        match num as i32 {
            3 => {
                if val > 0x7fi32 {
                    val -= 0x100i32
                }
                let fresh16 = *start;
                *start = (*start).offset(1);
                val = val << 8i32 | *fresh16 as i32;
                current_block_5 = 5559910912116893431;
            }
            2 => {
                current_block_5 = 5559910912116893431;
            }
            1 => {
                current_block_5 = 15700427407090132107;
            }
            _ => {
                current_block_5 = 13183875560443969876;
            }
        }
        match current_block_5 {
            5559910912116893431 => {
                let fresh17 = *start;
                *start = (*start).offset(1);
                val = val << 8i32 | *fresh17 as i32;
                current_block_5 = 15700427407090132107;
            }
            _ => {}
        }
        match current_block_5 {
            15700427407090132107 => {
                let fresh18 = *start;
                *start = (*start).offset(1);
                val = val << 8i32 | *fresh18 as i32
            }
            _ => {}
        }
    } else {
        _tt_abort(b"Premature end of DVI byte stream in VF font\n\x00" as *const u8 as *const i8);
    }
    return val;
}
unsafe extern "C" fn vf_putrule(mut start: *mut *mut u8, mut end: *mut u8, mut ptsize: spt_t) {
    let mut height: i32 = get_pkt_signed_num(start, end, 3_u8);
    let mut width: i32 = get_pkt_signed_num(start, end, 3_u8);
    dvi_rule(sqxfw(ptsize, width), sqxfw(ptsize, height));
}
unsafe extern "C" fn vf_setrule(mut start: *mut *mut u8, mut end: *mut u8, mut ptsize: spt_t) {
    let mut height: i32 = get_pkt_signed_num(start, end, 3_u8);
    let mut s_width: i32 = sqxfw(ptsize, get_pkt_signed_num(start, end, 3_u8));
    dvi_rule(s_width, sqxfw(ptsize, height));
    dvi_right(s_width);
}
unsafe extern "C" fn vf_fnt(mut font_id: i32, mut vf_font: i32) {
    let mut i: i32 = 0;
    i = 0i32;
    while (i as u32) < (*vf_fonts.offset(vf_font as isize)).num_dev_fonts {
        if font_id
            == (*(*vf_fonts.offset(vf_font as isize))
                .dev_fonts
                .offset(i as isize))
            .font_id
        {
            break;
        }
        i += 1
    }
    if (i as u32) < (*vf_fonts.offset(vf_font as isize)).num_dev_fonts {
        /* Font was found */
        dvi_set_font(
            (*(*vf_fonts.offset(vf_font as isize))
                .dev_fonts
                .offset(i as isize))
            .dev_id,
        );
    } else {
        fprintf(
            stderr,
            b"Font_id: %d not found in VF\n\x00" as *const u8 as *const i8,
            font_id,
        );
    };
}
/* identical to do_xxx in dvi.c */
unsafe extern "C" fn vf_xxx(mut len: i32, mut start: *mut *mut u8, mut end: *mut u8) {
    if *start <= end.offset(-(len as isize)) {
        let mut buffer: *mut u8 = new(((len + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64)
            as u32) as *mut u8;
        memcpy(
            buffer as *mut libc::c_void,
            *start as *const libc::c_void,
            len as u64,
        );
        *buffer.offset(len as isize) = '\u{0}' as i32 as u8;
        let mut p: *mut u8 = buffer;
        while p < buffer.offset(len as isize) && *p as i32 == ' ' as i32 {
            p = p.offset(1)
        }
        /*
         * Warning message from virtual font.
         */
        if memcmp(
            p as *mut i8 as *const libc::c_void,
            b"Warning:\x00" as *const u8 as *const i8 as *const libc::c_void,
            8i32 as u64,
        ) == 0
        {
            if verbose != 0 {
                dpx_warning(b"VF:%s\x00" as *const u8 as *const i8, p.offset(8));
            }
        } else {
            dvi_do_special(buffer as *const libc::c_void, len);
        }
        free(buffer as *mut libc::c_void);
    } else {
        _tt_abort(b"Premature end of DVI byte stream in VF font.\x00" as *const u8 as *const i8);
    }
    *start = (*start).offset(len as isize);
}
#[no_mangle]
pub unsafe extern "C" fn vf_set_char(mut ch: i32, mut vf_font: i32) {
    let mut opcode: u8 = 0;
    let mut start: *mut u8 = 0 as *mut u8;
    let mut end: *mut u8 = 0 as *mut u8;
    let mut ptsize: spt_t = 0;
    let mut default_font: i32 = -1i32;
    if (vf_font as u32) < num_vf_fonts {
        /* Initialize to the first font or -1 if undefined */
        ptsize = (*vf_fonts.offset(vf_font as isize)).ptsize;
        if (*vf_fonts.offset(vf_font as isize)).num_dev_fonts > 0_u32 {
            default_font = (*(*vf_fonts.offset(vf_font as isize)).dev_fonts.offset(0)).dev_id
        }
        dvi_vf_init(default_font);
        if ch as u32 >= (*vf_fonts.offset(vf_font as isize)).num_chars || {
            start = *(*vf_fonts.offset(vf_font as isize))
                .ch_pkt
                .offset(ch as isize);
            start.is_null()
        } {
            fprintf(
                stderr,
                b"\nchar=0x%x(%d)\n\x00" as *const u8 as *const i8,
                ch,
                ch,
            );
            fprintf(
                stderr,
                b"Tried to set a nonexistent character in a virtual font\x00" as *const u8
                    as *const i8,
            );
            end = 0 as *mut u8;
            start = end
        } else {
            end = start.offset(
                *(*vf_fonts.offset(vf_font as isize))
                    .pkt_len
                    .offset(ch as isize) as isize,
            )
        }
        while !start.is_null() && start < end {
            let fresh19 = start;
            start = start.offset(1);
            opcode = *fresh19;
            match opcode as i32 {
                128 | 129 | 130 => {
                    dvi_set(get_pkt_unsigned_num(
                        &mut start,
                        end,
                        (opcode as i32 - 128i32) as u8,
                    ));
                }
                131 => {
                    _tt_abort(
                        b"Multibyte (>24 bits) character in VF packet.\nI can\'t handle this!\x00"
                            as *const u8 as *const i8,
                    );
                }
                132 => {
                    vf_setrule(&mut start, end, ptsize);
                }
                133 | 134 | 135 => {
                    dvi_put(get_pkt_unsigned_num(
                        &mut start,
                        end,
                        (opcode as i32 - 133i32) as u8,
                    ));
                }
                136 => {
                    _tt_abort(
                        b"Multibyte (>24 bits) character in VF packet.\nI can\'t handle this!\x00"
                            as *const u8 as *const i8,
                    );
                }
                137 => {
                    vf_putrule(&mut start, end, ptsize);
                }
                138 => {}
                141 => {
                    dvi_push();
                }
                142 => {
                    dpx_dvi_pop();
                }
                143 | 144 | 145 | 146 => {
                    dvi_right(sqxfw(
                        ptsize,
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 143i32) as u8),
                    ));
                }
                147 => {
                    dvi_w0();
                }
                148 | 149 | 150 | 151 => {
                    dvi_w(sqxfw(
                        ptsize,
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 148i32) as u8),
                    ));
                }
                152 => {
                    dvi_x0();
                }
                153 | 154 | 155 | 156 => {
                    dvi_x(sqxfw(
                        ptsize,
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 153i32) as u8),
                    ));
                }
                157 | 158 | 159 | 160 => {
                    dvi_down(sqxfw(
                        ptsize,
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 157i32) as u8),
                    ));
                }
                161 => {
                    dvi_y0();
                }
                162 | 163 | 164 | 165 => {
                    dvi_y(sqxfw(
                        ptsize,
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 162i32) as u8),
                    ));
                }
                166 => {
                    dvi_z0();
                }
                167 | 168 | 169 | 170 => {
                    dvi_z(sqxfw(
                        ptsize,
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 167i32) as u8),
                    ));
                }
                235 | 236 | 237 | 238 => {
                    vf_fnt(
                        get_pkt_signed_num(&mut start, end, (opcode as i32 - 235i32) as u8),
                        vf_font,
                    );
                }
                239 | 240 | 241 | 242 => {
                    let mut len: i32 =
                        get_pkt_unsigned_num(&mut start, end, (opcode as i32 - 239i32) as u8);
                    if len < 0i32 {
                        dpx_warning(
                            b"VF: Special with %d bytes???\x00" as *const u8 as *const i8,
                            len,
                        );
                    } else {
                        vf_xxx(len, &mut start, end);
                    }
                }
                255 => {
                    dvi_dirchg(unsigned_byte(&mut start, end) as u8);
                }
                _ => {
                    if opcode as i32 <= 127i32 {
                        dvi_set(opcode as i32);
                    } else if opcode as i32 >= 171i32 && opcode as i32 <= 234i32 {
                        vf_fnt(opcode as i32 - 171i32, vf_font);
                    } else {
                        fprintf(
                            stderr,
                            b"Unexpected opcode: %d\n\x00" as *const u8 as *const i8,
                            opcode as i32,
                        );
                        _tt_abort(b"Unexpected opcode in vf file\n\x00" as *const u8 as *const i8);
                    }
                }
            }
        }
        dvi_vf_finish();
    } else {
        fprintf(
            stderr,
            b"vf_set_char: font: %d\x00" as *const u8 as *const i8,
            vf_font,
        );
        _tt_abort(b"Font not loaded\n\x00" as *const u8 as *const i8);
    };
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
pub unsafe extern "C" fn vf_close_all_fonts() {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut one_font: *mut font_def = 0 as *mut font_def;
    i = 0_u32;
    while i < num_vf_fonts {
        /* Release the packet for each character */
        if !(*vf_fonts.offset(i as isize)).ch_pkt.is_null() {
            j = 0_u32;
            while j < (*vf_fonts.offset(i as isize)).num_chars {
                free(*(*vf_fonts.offset(i as isize)).ch_pkt.offset(j as isize) as *mut libc::c_void);
                j = j.wrapping_add(1)
            }
            free((*vf_fonts.offset(i as isize)).ch_pkt as *mut libc::c_void);
        }
        free((*vf_fonts.offset(i as isize)).pkt_len as *mut libc::c_void);
        free((*vf_fonts.offset(i as isize)).tex_name as *mut libc::c_void);
        /* Release each font record */
        j = 0_u32;
        while j < (*vf_fonts.offset(i as isize)).num_dev_fonts {
            one_font =
                &mut *(*vf_fonts.offset(i as isize)).dev_fonts.offset(j as isize) as *mut font_def;
            free((*one_font).directory as *mut libc::c_void);
            free((*one_font).name as *mut libc::c_void);
            j = j.wrapping_add(1)
        }
        free((*vf_fonts.offset(i as isize)).dev_fonts as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free(vf_fonts as *mut libc::c_void);
}
