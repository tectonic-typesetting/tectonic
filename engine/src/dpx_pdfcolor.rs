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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_get_version() -> libc::c_uint;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
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
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn sget_unsigned_pair(_: *mut u8) -> libc::c_ushort;
    #[no_mangle]
    fn pdf_dev_get_param(param_type: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_reset_color(force: libc::c_int);
    #[no_mangle]
    fn MD5_init(ctx: *mut MD5_CONTEXT);
    #[no_mangle]
    fn MD5_write(ctx: *mut MD5_CONTEXT, inbuf: *const u8, inlen: libc::c_uint);
    #[no_mangle]
    fn MD5_final(outbuf: *mut u8, ctx: *mut MD5_CONTEXT);
}
pub type __int32_t = libc::c_int;
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
pub type int32_t = __int32_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub num_components: libc::c_int,
    pub spot_color_name: *mut libc::c_char,
    pub values: [libc::c_double; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_colorspace {
    pub ident: *mut libc::c_char,
    pub subtype: libc::c_int,
    pub resource: *mut pdf_obj,
    pub reference: *mut pdf_obj,
    pub cdata: *mut libc::c_void,
}
pub type iccSig = u32;
/*
 * In ICC profile stream dicrionary, there is /Range whose values must
 * "match the information in the profile". But where is those values in?
 *
 * How should I treat rendering intent?
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccbased_cdata {
    pub sig: int32_t,
    pub checksum: [u8; 16],
    pub colorspace: libc::c_int,
    pub alternate: libc::c_int,
    /* alternate colorspace (id), unused */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub count: libc::c_uint,
    pub capacity: libc::c_uint,
    pub colorspaces: *mut pdf_colorspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccHeader {
    pub size: libc::c_int,
    pub CMMType: iccSig,
    pub version: int32_t,
    pub devClass: iccSig,
    pub colorSpace: iccSig,
    pub PCS: iccSig,
    pub creationDate: [libc::c_char; 12],
    pub acsp: iccSig,
    pub platform: iccSig,
    pub flags: [libc::c_char; 4],
    pub devMnfct: iccSig,
    pub devModel: iccSig,
    pub devAttr: [libc::c_char; 8],
    pub intent: int32_t,
    pub illuminant: iccXYZNumber,
    pub creator: iccSig,
    pub ID: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccXYZNumber {
    pub X: int32_t,
    pub Y: int32_t,
    pub Z: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CONTEXT {
    pub A: u32,
    pub B: u32,
    pub C: u32,
    pub D: u32,
    pub nblocks: size_t,
    pub buf: [u8; 64],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub current: libc::c_int,
    pub stroke: [pdf_color; 128],
    pub fill: [pdf_color; 128],
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
/* No page independence here...
 */
static mut verbose: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_color_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
/* This function returns PDF_COLORSPACE_TYPE_GRAY,
 * PDF_COLORSPACE_TYPE_RGB, PDF_COLORSPACE_TYPE_CMYK or
 * PDF_COLORSPACE_TYPE_SPOT.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_color_type(mut color: *const pdf_color) -> libc::c_int {
    if !color.is_null() {
    } else {
        __assert_fail(
            b"color\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            54i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"int pdf_color_type(const pdf_color *)\x00",
            ))
            .as_ptr(),
        ); /* Dummy */
    }
    return -(*color).num_components;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_rgbcolor(
    mut color: *mut pdf_color,
    mut r: libc::c_double,
    mut g: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_int {
    if !color.is_null() {
    } else {
        __assert_fail(
            b"color\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            62i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                b"int pdf_color_rgbcolor(pdf_color *, double, double, double)\x00",
            ))
            .as_ptr(),
        );
    }
    if r < 0.0f64 || r > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: red=%g\x00" as *const u8 as *const libc::c_char,
            r,
        );
        return -1i32;
    }
    if g < 0.0f64 || g > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: green=%g\x00" as *const u8 as *const libc::c_char,
            g,
        );
        return -1i32;
    }
    if b < 0.0f64 || b > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: blue=%g\x00" as *const u8 as *const libc::c_char,
            b,
        );
        return -1i32;
    }
    (*color).values[0] = r;
    (*color).values[1] = g;
    (*color).values[2] = b;
    (*color).num_components = 3i32;
    (*color).spot_color_name = 0 as *mut libc::c_char;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_cmykcolor(
    mut color: *mut pdf_color,
    mut c: libc::c_double,
    mut m: libc::c_double,
    mut y: libc::c_double,
    mut k: libc::c_double,
) -> libc::c_int {
    if !color.is_null() {
    } else {
        __assert_fail(
            b"color\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            91i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                b"int pdf_color_cmykcolor(pdf_color *, double, double, double, double)\x00",
            ))
            .as_ptr(),
        );
    }
    if c < 0.0f64 || c > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: cyan=%g\x00" as *const u8 as *const libc::c_char,
            c,
        );
        return -1i32;
    }
    if m < 0.0f64 || m > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: magenta=%g\x00" as *const u8 as *const libc::c_char,
            m,
        );
        return -1i32;
    }
    if y < 0.0f64 || y > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: yellow=%g\x00" as *const u8 as *const libc::c_char,
            y,
        );
        return -1i32;
    }
    if k < 0.0f64 || k > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: black=%g\x00" as *const u8 as *const libc::c_char,
            k,
        );
        return -1i32;
    }
    (*color).values[0] = c;
    (*color).values[1] = m;
    (*color).values[2] = y;
    (*color).values[3] = k;
    (*color).num_components = 4i32;
    (*color).spot_color_name = 0 as *mut libc::c_char;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_graycolor(
    mut color: *mut pdf_color,
    mut g: libc::c_double,
) -> libc::c_int {
    if !color.is_null() {
    } else {
        __assert_fail(
            b"color\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            125i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"int pdf_color_graycolor(pdf_color *, double)\x00",
            ))
            .as_ptr(),
        );
    }
    if g < 0.0f64 || g > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: gray=%g\x00" as *const u8 as *const libc::c_char,
            g,
        );
        return -1i32;
    }
    (*color).values[0] = g;
    (*color).num_components = 1i32;
    (*color).spot_color_name = 0 as *mut libc::c_char;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_spotcolor(
    mut color: *mut pdf_color,
    mut name: *mut libc::c_char,
    mut c: libc::c_double,
) -> libc::c_int {
    if !color.is_null() {
    } else {
        __assert_fail(
            b"color\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            144i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int pdf_color_spotcolor(pdf_color *, char *, double)\x00",
            ))
            .as_ptr(),
        );
    }
    if c < 0.0f64 || c > 1.0f64 {
        dpx_warning(
            b"Invalid color value specified: grade=%g\x00" as *const u8 as *const libc::c_char,
            c,
        );
        return -1i32;
    }
    (*color).values[0] = c;
    (*color).values[1] = 0.0f64;
    (*color).num_components = 2i32;
    (*color).spot_color_name = name;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_copycolor(
    mut color1: *mut pdf_color,
    mut color2: *const pdf_color,
) {
    if !color1.is_null() && !color2.is_null() {
    } else {
        __assert_fail(
            b"color1 && color2\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            165i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"void pdf_color_copycolor(pdf_color *, const pdf_color *)\x00",
            ))
            .as_ptr(),
        );
    }
    memcpy(
        color1 as *mut libc::c_void,
        color2 as *const libc::c_void,
        ::std::mem::size_of::<pdf_color>() as u64,
    );
}
/* Brighten up a color. f == 0 means no change, f == 1 means white. */
#[no_mangle]
pub unsafe extern "C" fn pdf_color_brighten_color(
    mut dst: *mut pdf_color,
    mut src: *const pdf_color,
    mut f: libc::c_double,
) {
    if !dst.is_null() && !src.is_null() {
    } else {
        __assert_fail(
            b"dst && src\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            174i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 70], &[libc::c_char; 70]>(
                b"void pdf_color_brighten_color(pdf_color *, const pdf_color *, double)\x00",
            ))
            .as_ptr(),
        ); /* n == 4 is CMYK, others are RGB and Gray */
    }
    if f == 1.0f64 {
        pdf_color_graycolor(dst, 1.0f64);
    } else {
        let mut f0: libc::c_double = 0.;
        let mut f1: libc::c_double = 0.;
        let mut n: libc::c_int = 0;
        (*dst).num_components = (*src).num_components;
        n = (*dst).num_components;
        f1 = if n == 4i32 { 0.0f64 } else { f };
        f0 = 1.0f64 - f;
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 != 0) {
                break;
            }
            (*dst).values[n as usize] = f0 * (*src).values[n as usize] + f1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_is_white(mut color: *const pdf_color) -> bool {
    let mut n: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    if !color.is_null() {
    } else {
        __assert_fail(
            b"color\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            197i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"_Bool pdf_color_is_white(const pdf_color *)\x00",
            ))
            .as_ptr(),
        );
    }
    n = (*color).num_components;
    match n {
        1 | 3 => {
            /* Gray */
            /* RGB */
            f = 1.0f64
        }
        4 => {
            /* CMYK */
            f = 0.0f64
        }
        _ => return 0i32 != 0,
    }
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 != 0) {
            break;
        }
        if (*color).values[n as usize] != f {
            return 0i32 != 0;
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_to_string(
    mut color: *const pdf_color,
    mut buffer: *mut libc::c_char,
    mut mask: libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0i32;
    if pdf_color_type(color) == -2i32 {
        len = sprintf(
            buffer,
            b" /%s %c%c %g %c%c\x00" as *const u8 as *const libc::c_char,
            (*color).spot_color_name,
            'C' as i32 | mask as libc::c_int,
            'S' as i32 | mask as libc::c_int,
            floor((*color).values[0] / 0.001f64 + 0.5f64) * 0.001f64,
            'S' as i32 | mask as libc::c_int,
            'C' as i32 | mask as libc::c_int,
        )
    } else {
        i = 0i32;
        while i < (*color).num_components {
            len += sprintf(
                buffer.offset(len as isize),
                b" %g\x00" as *const u8 as *const libc::c_char,
                floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
            );
            i += 1
        }
    }
    return len;
}
/*
 * This routine is not a real color matching.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_color_compare(
    mut color1: *const pdf_color,
    mut color2: *const pdf_color,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = (*color1).num_components;
    let mut current_block_1: u64;
    match n {
        1 => {
            current_block_1 = 715039052867723359;
        }
        2 => {
            /* Spot */
            current_block_1 = 1982130065057554431;
        }
        3 => {
            current_block_1 = 1982130065057554431;
        }
        4 => {
            current_block_1 = 15718257842624222162;
        }
        _ => return -1i32,
    }
    match current_block_1 {
        1982130065057554431 =>
        /* RGB */
        {
            current_block_1 = 15718257842624222162;
        }
        _ => {}
    }
    match current_block_1 {
        15718257842624222162 =>
            /* CMYK */
            {}
        _ => {}
    }
    if n != (*color2).num_components {
        return -1i32;
    }
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0) {
            break;
        }
        if (*color1).values[n as usize] != (*color2).values[n as usize] {
            return -1i32;
        }
    }
    if !(*color1).spot_color_name.is_null() && !(*color2).spot_color_name.is_null() {
        return strcmp((*color1).spot_color_name, (*color2).spot_color_name);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_is_valid(mut color: *const pdf_color) -> bool {
    let mut n: libc::c_int = 0;
    n = (*color).num_components;
    let mut current_block_1: u64;
    match n {
        1 => {
            current_block_1 = 715039052867723359;
        }
        2 => {
            /* Spot */
            current_block_1 = 17490471542129831839;
        }
        3 => {
            current_block_1 = 17490471542129831839;
        }
        4 => {
            current_block_1 = 7844836989092399584;
        }
        _ => return 0i32 != 0,
    }
    match current_block_1 {
        17490471542129831839 =>
        /* RGB */
        {
            current_block_1 = 7844836989092399584;
        }
        _ => {}
    }
    match current_block_1 {
        7844836989092399584 =>
            /* CMYK */
            {}
        _ => {}
    }
    loop {
        let fresh3 = n;
        n = n - 1;
        if !(fresh3 != 0) {
            break;
        }
        if (*color).values[n as usize] < 0.0f64 || (*color).values[n as usize] > 1.0f64 {
            dpx_warning(
                b"Invalid color value: %g\x00" as *const u8 as *const libc::c_char,
                (*color).values[n as usize],
            );
            return 0i32 != 0;
        }
    }
    if pdf_color_type(color) == -2i32 {
        if (*color).spot_color_name.is_null()
            || *(*color).spot_color_name.offset(0) as libc::c_int == '\u{0}' as i32
        {
            dpx_warning(b"Invalid spot color: empty name\x00" as *const u8 as *const libc::c_char);
            return 0i32 != 0;
        }
    }
    return 1i32 != 0;
}
static mut color_stack: C2RustUnnamed_2 = C2RustUnnamed_2 {
    current: 0,
    stroke: [pdf_color {
        num_components: 0,
        spot_color_name: 0 as *const libc::c_char as *mut libc::c_char,
        values: [0.; 4],
    }; 128],
    fill: [pdf_color {
        num_components: 0,
        spot_color_name: 0 as *const libc::c_char as *mut libc::c_char,
        values: [0.; 4],
    }; 128],
};
#[no_mangle]
pub unsafe extern "C" fn pdf_color_clear_stack() {
    if color_stack.current > 0i32 {
        dpx_warning(
            b"You\'ve mistakenly made a global color change within nested colors.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    loop {
        let fresh4 = color_stack.current;
        color_stack.current = color_stack.current - 1;
        if !(fresh4 != 0) {
            break;
        }
        free(color_stack.stroke[color_stack.current as usize].spot_color_name as *mut libc::c_void);
        free(color_stack.fill[color_stack.current as usize].spot_color_name as *mut libc::c_void);
    }
    color_stack.current = 0i32;
    pdf_color_graycolor(color_stack.stroke.as_mut_ptr(), 0.0f64);
    pdf_color_graycolor(color_stack.fill.as_mut_ptr(), 0.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_set(mut sc: *mut pdf_color, mut fc: *mut pdf_color) {
    pdf_color_copycolor(
        &mut *color_stack
            .stroke
            .as_mut_ptr()
            .offset(color_stack.current as isize),
        sc,
    );
    pdf_color_copycolor(
        &mut *color_stack
            .fill
            .as_mut_ptr()
            .offset(color_stack.current as isize),
        fc,
    );
    pdf_dev_reset_color(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_push(mut sc: *mut pdf_color, mut fc: *mut pdf_color) {
    if color_stack.current >= 128i32 - 1i32 {
        dpx_warning(b"Color stack overflow. Just ignore.\x00" as *const u8 as *const libc::c_char);
    } else {
        color_stack.current += 1;
        pdf_color_set(sc, fc);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_color_pop() {
    if color_stack.current <= 0i32 {
        dpx_warning(b"Color stack underflow. Just ignore.\x00" as *const u8 as *const libc::c_char);
    } else {
        color_stack.current -= 1;
        pdf_dev_reset_color(0i32);
    };
}
/* Color special
 * See remark in spc_color.c.
 */
/* Color stack
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_color_get_current(
    mut sc: *mut *mut pdf_color,
    mut fc: *mut *mut pdf_color,
) {
    *sc = &mut *color_stack
        .stroke
        .as_mut_ptr()
        .offset(color_stack.current as isize) as *mut pdf_color;
    *fc = &mut *color_stack
        .fill
        .as_mut_ptr()
        .offset(color_stack.current as isize) as *mut pdf_color;
}
static mut nullbytes16: [u8; 16] = [
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
    0i32 as u8,
];
static mut icc_versions: [C2RustUnnamed_1; 8] = [
    {
        let mut init = C2RustUnnamed_1 {
            major: 0i32,
            minor: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0i32,
            minor: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0i32,
            minor: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0x2i32,
            minor: 0x10i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0x2i32,
            minor: 0x20i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0x4i32,
            minor: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0x4i32,
            minor: 0i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            major: 0x4i32,
            minor: 0x20i32,
        };
        init
    },
];
unsafe extern "C" fn iccp_version_supported(
    mut major: libc::c_int,
    mut minor: libc::c_int,
) -> libc::c_int {
    let mut pdf_ver: libc::c_int = 0;
    pdf_ver = pdf_get_version() as libc::c_int;
    if pdf_ver < 8i32 {
        if icc_versions[pdf_ver as usize].major < major {
            return 0i32;
        } else if icc_versions[pdf_ver as usize].major == major
            && icc_versions[pdf_ver as usize].minor < minor
        {
            return 0i32;
        } else {
            return 1i32;
        }
    }
    return 0i32;
}
unsafe extern "C" fn str2iccSig(mut s: *const libc::c_void) -> iccSig {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = s as *const libc::c_char;
    return ((*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int) as iccSig;
}
unsafe extern "C" fn iccp_init_iccHeader(mut icch: *mut iccHeader) {
    if !icch.is_null() {
    } else {
        __assert_fail(
            b"icch\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            460i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void iccp_init_iccHeader(iccHeader *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*icch).size = 0i32;
    (*icch).CMMType = 0i32 as iccSig;
    (*icch).version = 0xffffffi32;
    (*icch).devClass = 0i32 as iccSig;
    (*icch).colorSpace = 0i32 as iccSig;
    (*icch).PCS = 0i32 as iccSig;
    memset(
        (*icch).creationDate.as_mut_ptr() as *mut libc::c_void,
        0i32,
        12i32 as u64,
    );
    (*icch).acsp =
        str2iccSig(b"ascp\x00" as *const u8 as *const libc::c_char as *const libc::c_void);
    (*icch).platform = 0i32 as iccSig;
    memset(
        (*icch).flags.as_mut_ptr() as *mut libc::c_void,
        0i32,
        4i32 as u64,
    );
    (*icch).devMnfct = 0i32 as iccSig;
    (*icch).devModel = 0i32 as iccSig;
    memset(
        (*icch).devAttr.as_mut_ptr() as *mut libc::c_void,
        0i32,
        8i32 as u64,
    );
    (*icch).intent = 0i32;
    (*icch).illuminant.X = 0i32;
    (*icch).illuminant.Y = 0i32;
    (*icch).illuminant.Z = 0i32;
    (*icch).creator = 0i32 as iccSig;
    memset(
        (*icch).ID.as_mut_ptr() as *mut libc::c_void,
        0i32,
        16i32 as u64,
    );
}
unsafe extern "C" fn init_iccbased_cdata(mut cdata: *mut iccbased_cdata) {
    if !cdata.is_null() {
    } else {
        __assert_fail(
            b"cdata\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            511i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void init_iccbased_cdata(struct iccbased_cdata *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*cdata).sig =
        ('i' as i32) << 24i32 | ('c' as i32) << 16i32 | ('c' as i32) << 8i32 | 'b' as i32;
    memset(
        (*cdata).checksum.as_mut_ptr() as *mut libc::c_void,
        0i32,
        16i32 as u64,
    );
    (*cdata).colorspace = 0i32;
    (*cdata).alternate = -1i32;
}
unsafe extern "C" fn release_iccbased_cdata(mut cdata: *mut iccbased_cdata) {
    if !cdata.is_null()
        && (*cdata).sig
            == ('i' as i32) << 24i32 | ('c' as i32) << 16i32 | ('c' as i32) << 8i32 | 'b' as i32
    {
    } else {
        __assert_fail(
            b"check_sig(cdata, \'i\', \'c\', \'c\', \'b\')\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            524i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"void release_iccbased_cdata(struct iccbased_cdata *)\x00",
            ))
            .as_ptr(),
        );
    }
    free(cdata as *mut libc::c_void);
}
unsafe extern "C" fn get_num_components_iccbased(mut cdata: *const iccbased_cdata) -> libc::c_int {
    let mut num_components: libc::c_int = 0i32;
    if !cdata.is_null()
        && (*cdata).sig
            == ('i' as i32) << 24i32 | ('c' as i32) << 16i32 | ('c' as i32) << 8i32 | 'b' as i32
    {
    } else {
        __assert_fail(
            b"check_sig(cdata, \'i\', \'c\', \'c\', \'b\')\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            534i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"int get_num_components_iccbased(const struct iccbased_cdata *)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*cdata).colorspace {
        -3 => num_components = 3i32,
        -4 => num_components = 4i32,
        -1 => num_components = 1i32,
        2 => num_components = 3i32,
        _ => {}
    }
    return num_components;
}
unsafe extern "C" fn compare_iccbased(
    mut ident1: *const libc::c_char,
    mut cdata1: *const iccbased_cdata,
    mut ident2: *const libc::c_char,
    mut cdata2: *const iccbased_cdata,
) -> libc::c_int {
    if !cdata1.is_null() && !cdata2.is_null() {
        if !cdata1.is_null()
            && (*cdata1).sig
                == ('i' as i32) << 24i32 | ('c' as i32) << 16i32 | ('c' as i32) << 8i32 | 'b' as i32
        {
        } else {
            __assert_fail(b"check_sig(cdata1, \'i\', \'c\', \'c\', \'b\')\x00"
                              as *const u8 as *const libc::c_char,
                          b"dpx-pdfcolor.c\x00" as *const u8 as
                              *const libc::c_char, 560i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 111],
                                                    &[libc::c_char; 111]>(b"int compare_iccbased(const char *, const struct iccbased_cdata *, const char *, const struct iccbased_cdata *)\x00")).as_ptr());
        }
        if !cdata2.is_null()
            && (*cdata2).sig
                == ('i' as i32) << 24i32 | ('c' as i32) << 16i32 | ('c' as i32) << 8i32 | 'b' as i32
        {
        } else {
            __assert_fail(b"check_sig(cdata2, \'i\', \'c\', \'c\', \'b\')\x00"
                              as *const u8 as *const libc::c_char,
                          b"dpx-pdfcolor.c\x00" as *const u8 as
                              *const libc::c_char, 561i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 111],
                                                    &[libc::c_char; 111]>(b"int compare_iccbased(const char *, const struct iccbased_cdata *, const char *, const struct iccbased_cdata *)\x00")).as_ptr());
        }
        if memcmp(
            (*cdata1).checksum.as_ptr() as *const libc::c_void,
            nullbytes16.as_mut_ptr() as *const libc::c_void,
            16i32 as u64,
        ) != 0
            && memcmp(
                (*cdata2).checksum.as_ptr() as *const libc::c_void,
                nullbytes16.as_mut_ptr() as *const libc::c_void,
                16i32 as u64,
            ) != 0
        {
            return memcmp(
                (*cdata1).checksum.as_ptr() as *const libc::c_void,
                (*cdata2).checksum.as_ptr() as *const libc::c_void,
                16i32 as u64,
            );
        }
        if (*cdata1).colorspace != (*cdata2).colorspace {
            return (*cdata1).colorspace - (*cdata2).colorspace;
        }
        /* Continue if checksum unknown and colorspace is same. */
    }
    if !ident1.is_null() && !ident2.is_null() {
        return strcmp(ident1, ident2);
    }
    /* No way to compare */
    return -1i32; /* acsp */
}
#[no_mangle]
pub unsafe extern "C" fn iccp_check_colorspace(
    mut colortype: libc::c_int,
    mut profile: *const libc::c_void,
    mut proflen: libc::c_int,
) -> libc::c_int {
    let mut colorspace: iccSig = 0;
    let mut p: *const u8 = 0 as *const u8;
    if profile.is_null() || proflen < 128i32 {
        return -1i32;
    }
    p = profile as *const u8;
    colorspace = str2iccSig(p.offset(16) as *const libc::c_void);
    match colortype {
        3 | -3 => {
            if colorspace
                != str2iccSig(
                    b"RGB \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                )
            {
                return -1i32;
            }
        }
        1 | -1 => {
            if colorspace
                != str2iccSig(
                    b"GRAY\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                )
            {
                return -1i32;
            }
        }
        -4 => {
            if colorspace
                != str2iccSig(
                    b"CMYK\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                )
            {
                return -1i32;
            }
        }
        _ => return -1i32,
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn iccp_get_rendering_intent(
    mut profile: *const libc::c_void,
    mut proflen: libc::c_int,
) -> *mut pdf_obj {
    let mut ri: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const u8 = 0 as *const u8;
    let mut intent: int32_t = 0;
    if profile.is_null() || proflen < 128i32 {
        return 0 as *mut pdf_obj;
    }
    p = profile as *const u8;
    intent = (*p.offset(64) as libc::c_int) << 24i32
        | (*p.offset(65) as libc::c_int) << 16i32
        | (*p.offset(66) as libc::c_int) << 8i32
        | *p.offset(67) as libc::c_int;
    match intent >> 16i32 & 0xffi32 {
        2 => ri = pdf_new_name(b"Saturation\x00" as *const u8 as *const libc::c_char),
        0 => ri = pdf_new_name(b"Perceptual\x00" as *const u8 as *const libc::c_char),
        3 => ri = pdf_new_name(b"AbsoluteColorimetric\x00" as *const u8 as *const libc::c_char),
        1 => ri = pdf_new_name(b"RelativeColorimetric\x00" as *const u8 as *const libc::c_char),
        _ => {
            dpx_warning(
                b"Invalid rendering intent type: %d\x00" as *const u8 as *const libc::c_char,
                intent >> 16i32 & 0xffi32,
            );
            ri = 0 as *mut pdf_obj
        }
    }
    return ri;
}
unsafe extern "C" fn iccp_unpack_header(
    mut icch: *mut iccHeader,
    mut profile: *const libc::c_void,
    mut proflen: libc::c_int,
    mut check_size: libc::c_int,
) -> libc::c_int {
    let mut p: *const u8 = 0 as *const u8;
    let mut endptr: *const u8 = 0 as *const u8;
    if check_size != 0 {
        if profile.is_null() || proflen < 128i32 || proflen % 4i32 != 0i32 {
            dpx_warning(
                b"Profile size: %d\x00" as *const u8 as *const libc::c_char,
                proflen,
            );
            return -1i32;
        }
    }
    p = profile as *const u8;
    endptr = p.offset(128);
    (*icch).size = (*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int;
    if check_size != 0 {
        if (*icch).size != proflen {
            dpx_warning(
                b"ICC Profile size: %d(header) != %d\x00" as *const u8 as *const libc::c_char,
                (*icch).size,
                proflen,
            );
            return -1i32;
        }
    }
    p = p.offset(4);
    (*icch).CMMType = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    (*icch).version = (*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int;
    p = p.offset(4);
    (*icch).devClass = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    (*icch).colorSpace = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    (*icch).PCS = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        (*icch).creationDate.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        12i32 as u64,
    );
    p = p.offset(12);
    (*icch).acsp = str2iccSig(p as *const libc::c_void);
    if (*icch).acsp
        != str2iccSig(b"acsp\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
    {
        dpx_warning(
            b"Invalid ICC profile: not \"acsp\" - %c%c%c%c \x00" as *const u8
                as *const libc::c_char,
            *p.offset(0) as libc::c_int,
            *p.offset(1) as libc::c_int,
            *p.offset(2) as libc::c_int,
            *p.offset(3) as libc::c_int,
        );
        return -1i32;
    }
    p = p.offset(4);
    (*icch).platform = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        (*icch).flags.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        4i32 as u64,
    );
    p = p.offset(4);
    (*icch).devMnfct = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    (*icch).devModel = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        (*icch).devAttr.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        8i32 as u64,
    );
    p = p.offset(8);
    (*icch).intent = (*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int;
    p = p.offset(4);
    (*icch).illuminant.X = (*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int;
    p = p.offset(4);
    (*icch).illuminant.Y = (*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int;
    p = p.offset(4);
    (*icch).illuminant.Z = (*p.offset(0) as libc::c_int) << 24i32
        | (*p.offset(1) as libc::c_int) << 16i32
        | (*p.offset(2) as libc::c_int) << 8i32
        | *p.offset(3) as libc::c_int;
    p = p.offset(4);
    (*icch).creator = str2iccSig(p as *const libc::c_void);
    p = p.offset(4);
    memcpy(
        (*icch).ID.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        16i32 as u64,
    );
    p = p.offset(16);
    /* 28 bytes reserved - must be set to zeros */
    while p < endptr {
        if *p as libc::c_int != '\u{0}' as i32 {
            dpx_warning(
                b"Reserved pad not zero: %02x (at offset %d in ICC profile header.)\x00"
                    as *const u8 as *const libc::c_char,
                *p as libc::c_int,
                128i32 - endptr.wrapping_offset_from(p) as libc::c_long as libc::c_int,
            );
            return -1i32;
        }
        p = p.offset(1)
    }
    return 0i32;
}
unsafe extern "C" fn iccp_get_checksum(
    mut checksum: *mut u8,
    mut profile: *const libc::c_void,
    mut proflen: libc::c_int,
) {
    let mut p: *const u8 = 0 as *const u8;
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    p = profile as *const u8;
    MD5_init(&mut md5);
    MD5_write(&mut md5, p.offset(0), 56i32 as libc::c_uint);
    MD5_write(&mut md5, nullbytes16.as_mut_ptr(), 12i32 as libc::c_uint);
    MD5_write(&mut md5, p.offset(68), 16i32 as libc::c_uint);
    MD5_write(&mut md5, nullbytes16.as_mut_ptr(), 16i32 as libc::c_uint);
    MD5_write(&mut md5, p.offset(100), 28i32 as libc::c_uint);
    /* body */
    MD5_write(&mut md5, p.offset(128), (proflen - 128i32) as libc::c_uint);
    MD5_final(checksum, &mut md5);
}
unsafe extern "C" fn print_iccp_header(mut icch: *mut iccHeader, mut checksum: *mut u8) {
    let mut i: libc::c_int = 0;
    if !icch.is_null() {
    } else {
        __assert_fail(
            b"icch\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            778i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"void print_iccp_header(iccHeader *, unsigned char *)\x00",
            ))
            .as_ptr(),
        );
    }
    dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    dpx_message(b"pdf_color>> ICC Profile Info\n\x00" as *const u8 as *const libc::c_char);
    dpx_message(
        b"pdf_color>> Profile Size:\t%d bytes\n\x00" as *const u8 as *const libc::c_char,
        (*icch).size,
    );
    if (*icch).CMMType == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"CMM Type\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).CMMType >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).CMMType >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).CMMType >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).CMMType & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"CMM Type\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"CMM Type\x00" as *const u8 as *const libc::c_char,
            (*icch).CMMType >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).CMMType >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).CMMType >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).CMMType & 0xffi32 as libc::c_uint,
        );
    }
    dpx_message(
        b"pdf_color>> Profile Version:\t%d.%01d.%01d\n\x00" as *const u8 as *const libc::c_char,
        (*icch).version >> 24i32 & 0xffi32,
        (*icch).version >> 20i32 & 0xfi32,
        (*icch).version >> 16i32 & 0xfi32,
    );
    if (*icch).devClass == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Device Class\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).devClass >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devClass >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devClass >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devClass & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Device Class\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Device Class\x00" as *const u8 as *const libc::c_char,
            (*icch).devClass >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).devClass >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).devClass >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).devClass & 0xffi32 as libc::c_uint,
        );
    }
    if (*icch).colorSpace == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Color Space\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).colorSpace >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).colorSpace >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).colorSpace >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).colorSpace & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Color Space\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Color Space\x00" as *const u8 as *const libc::c_char,
            (*icch).colorSpace >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).colorSpace >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).colorSpace >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).colorSpace & 0xffi32 as libc::c_uint,
        );
    }
    if (*icch).PCS == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Connection Space\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).PCS >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).PCS >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).PCS >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).PCS & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Connection Space\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Connection Space\x00" as *const u8 as *const libc::c_char,
            (*icch).PCS >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).PCS >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).PCS >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).PCS & 0xffi32 as libc::c_uint,
        );
    }
    dpx_message(b"pdf_color>> Creation Date:\t\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < 12i32 {
        if i == 0i32 {
            dpx_message(
                b"%04u\x00" as *const u8 as *const libc::c_char,
                sget_unsigned_pair((*icch).creationDate.as_mut_ptr() as *mut u8)
                    as libc::c_int,
            );
        } else {
            dpx_message(
                b":%02u\x00" as *const u8 as *const libc::c_char,
                sget_unsigned_pair(&mut *(*icch).creationDate.as_mut_ptr().offset(i as isize)
                    as *mut libc::c_char as *mut u8) as libc::c_int,
            );
        }
        i += 2i32
    }
    dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    if (*icch).platform == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Primary Platform\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).platform >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).platform >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).platform >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).platform & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Primary Platform\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Primary Platform\x00" as *const u8 as *const libc::c_char,
            (*icch).platform >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).platform >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).platform >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).platform & 0xffi32 as libc::c_uint,
        );
    }
    dpx_message(
        b"pdf_color>> Profile Flags:\t%02x:%02x:%02x:%02x\n\x00" as *const u8
            as *const libc::c_char,
        (*icch).flags[0] as libc::c_int,
        (*icch).flags[1] as libc::c_int,
        (*icch).flags[2] as libc::c_int,
        (*icch).flags[3] as libc::c_int,
    );
    if (*icch).devMnfct == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Device Mnfct\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).devMnfct >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devMnfct >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devMnfct >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devMnfct & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Device Mnfct\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Device Mnfct\x00" as *const u8 as *const libc::c_char,
            (*icch).devMnfct >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).devMnfct >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).devMnfct >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).devMnfct & 0xffi32 as libc::c_uint,
        );
    }
    if (*icch).devModel == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Device Model\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).devModel >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devModel >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devModel >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).devModel & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Device Model\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Device Model\x00" as *const u8 as *const libc::c_char,
            (*icch).devModel >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).devModel >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).devModel >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).devModel & 0xffi32 as libc::c_uint,
        );
    }
    dpx_message(b"pdf_color>> Device Attr:\t\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < 8i32 {
        if i == 0i32 {
            dpx_message(
                b"%02x\x00" as *const u8 as *const libc::c_char,
                (*icch).devAttr[i as usize] as libc::c_int,
            );
        } else {
            dpx_message(
                b":%02x\x00" as *const u8 as *const libc::c_char,
                (*icch).devAttr[i as usize] as libc::c_int,
            );
        }
        i += 1
    }
    dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    dpx_message(b"pdf_color>> Rendering Intent:\t\x00" as *const u8 as *const libc::c_char);
    match (*icch).intent >> 16i32 & 0xffi32 {
        2 => {
            dpx_message(b"Saturation\x00" as *const u8 as *const libc::c_char);
        }
        0 => {
            dpx_message(b"Perceptual\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            dpx_message(b"Absolute Colorimetric\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            dpx_message(b"Relative Colorimetric\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
            dpx_message(b"(invalid)\x00" as *const u8 as *const libc::c_char);
        }
    }
    dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    if (*icch).creator == 0i32 as libc::c_uint {
        dpx_message(
            b"pdf_color>> %s:\t(null)\n\x00" as *const u8 as *const libc::c_char,
            b"Creator\x00" as *const u8 as *const libc::c_char,
        );
    } else if *(*__ctype_b_loc())
        .offset(((*icch).creator >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
        as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).creator >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).creator >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(((*icch).creator & 0xffi32 as libc::c_uint) as libc::c_int as isize)
            as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        dpx_message(
            b"pdf_color>> %s:\t(invalid)\n\x00" as *const u8 as *const libc::c_char,
            b"Creator\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        dpx_message(
            b"pdf_color>> %s:\t%c%c%c%c\n\x00" as *const u8 as *const libc::c_char,
            b"Creator\x00" as *const u8 as *const libc::c_char,
            (*icch).creator >> 24i32 & 0xffi32 as libc::c_uint,
            (*icch).creator >> 16i32 & 0xffi32 as libc::c_uint,
            (*icch).creator >> 8i32 & 0xffi32 as libc::c_uint,
            (*icch).creator & 0xffi32 as libc::c_uint,
        );
    }
    dpx_message(b"pdf_color>> Illuminant (XYZ):\t\x00" as *const u8 as *const libc::c_char);
    dpx_message(
        b"%.3f %.3f %.3f\n\x00" as *const u8 as *const libc::c_char,
        (*icch).illuminant.X as libc::c_double / 0x10000i32 as libc::c_double,
        (*icch).illuminant.Y as libc::c_double / 0x10000i32 as libc::c_double,
        (*icch).illuminant.Z as libc::c_double / 0x10000i32 as libc::c_double,
    );
    dpx_message(b"pdf_color>> Checksum:\t\x00" as *const u8 as *const libc::c_char);
    if memcmp(
        (*icch).ID.as_mut_ptr() as *const libc::c_void,
        nullbytes16.as_mut_ptr() as *const libc::c_void,
        16i32 as u64,
    ) == 0
    {
        dpx_message(b"(null)\x00" as *const u8 as *const libc::c_char);
    } else {
        i = 0i32;
        while i < 16i32 {
            if i == 0i32 {
                dpx_message(
                    b"%02x\x00" as *const u8 as *const libc::c_char,
                    (*icch).ID[i as usize] as libc::c_int,
                );
            } else {
                dpx_message(
                    b":%02x\x00" as *const u8 as *const libc::c_char,
                    (*icch).ID[i as usize] as libc::c_int,
                );
            }
            i += 1
        }
    }
    dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    if !checksum.is_null() {
        dpx_message(b"pdf_color>> Calculated:\t\x00" as *const u8 as *const libc::c_char);
        i = 0i32;
        while i < 16i32 {
            if i == 0i32 {
                dpx_message(
                    b"%02x\x00" as *const u8 as *const libc::c_char,
                    *checksum.offset(i as isize) as libc::c_int,
                );
            } else {
                dpx_message(
                    b":%02x\x00" as *const u8 as *const libc::c_char,
                    *checksum.offset(i as isize) as libc::c_int,
                );
            }
            i += 1
        }
        dpx_message(b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn iccp_devClass_allowed(mut dev_class: libc::c_int) -> libc::c_int {
    let mut colormode: libc::c_int = 0;
    colormode = pdf_dev_get_param(2i32);
    match colormode {
        _ => {}
    }
    if dev_class as libc::c_uint
        != str2iccSig(b"scnr\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
        && dev_class as libc::c_uint
            != str2iccSig(b"mntr\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
        && dev_class as libc::c_uint
            != str2iccSig(b"prtr\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
        && dev_class as libc::c_uint
            != str2iccSig(b"spac\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
    {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn iccp_load_profile(
    mut ident: *const libc::c_char,
    mut profile: *const libc::c_void,
    mut proflen: libc::c_int,
) -> libc::c_int {
    let mut cspc_id: libc::c_int = 0;
    let mut resource: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut icch: iccHeader = iccHeader {
        size: 0,
        CMMType: 0,
        version: 0,
        devClass: 0,
        colorSpace: 0,
        PCS: 0,
        creationDate: [0; 12],
        acsp: 0,
        platform: 0,
        flags: [0; 4],
        devMnfct: 0,
        devModel: 0,
        devAttr: [0; 8],
        intent: 0,
        illuminant: iccXYZNumber { X: 0, Y: 0, Z: 0 },
        creator: 0,
        ID: [0; 16],
    };
    let mut colorspace: libc::c_int = 0;
    let mut checksum: [u8; 16] = [0; 16];
    let mut cdata: *mut iccbased_cdata = 0 as *mut iccbased_cdata;
    iccp_init_iccHeader(&mut icch);
    if iccp_unpack_header(&mut icch, profile, proflen, 1i32) < 0i32 {
        /* check size */
        dpx_warning(
            b"Invalid ICC profile header in \"%s\"\x00" as *const u8 as *const libc::c_char,
            ident,
        );
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    if iccp_version_supported(
        icch.version >> 24i32 & 0xffi32,
        icch.version >> 16i32 & 0xffi32,
    ) == 0
    {
        dpx_warning(b"ICC profile format spec. version %d.%01d.%01d not supported in current PDF version setting.\x00"
                        as *const u8 as *const libc::c_char,
                    icch.version >> 24i32 & 0xffi32,
                    icch.version >> 20i32 & 0xfi32,
                    icch.version >> 16i32 & 0xfi32);
        dpx_warning(b"ICC profile not embedded.\x00" as *const u8 as *const libc::c_char);
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    if iccp_devClass_allowed(icch.devClass as libc::c_int) == 0 {
        dpx_warning(
            b"Unsupported ICC Profile Device Class:\x00" as *const u8 as *const libc::c_char,
        );
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    if icch.colorSpace
        == str2iccSig(b"RGB \x00" as *const u8 as *const libc::c_char as *const libc::c_void)
    {
        colorspace = -3i32
    } else if icch.colorSpace
        == str2iccSig(b"GRAY\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
    {
        colorspace = -1i32
    } else if icch.colorSpace
        == str2iccSig(b"CMYK\x00" as *const u8 as *const libc::c_char as *const libc::c_void)
    {
        colorspace = -4i32
    } else {
        dpx_warning(b"Unsupported input color space.\x00" as *const u8 as *const libc::c_char);
        print_iccp_header(&mut icch, 0 as *mut u8);
        return -1i32;
    }
    iccp_get_checksum(checksum.as_mut_ptr(), profile, proflen);
    if memcmp(
        icch.ID.as_mut_ptr() as *const libc::c_void,
        nullbytes16.as_mut_ptr() as *const libc::c_void,
        16i32 as u64,
    ) != 0
        && memcmp(
            icch.ID.as_mut_ptr() as *const libc::c_void,
            checksum.as_mut_ptr() as *const libc::c_void,
            16i32 as u64,
        ) != 0
    {
        dpx_warning(
            b"Invalid ICC profile: Inconsistent checksum.\x00" as *const u8 as *const libc::c_char,
        );
        print_iccp_header(&mut icch, checksum.as_mut_ptr());
        return -1i32;
    }
    cdata = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<iccbased_cdata>() as u64)
        as u32) as *mut iccbased_cdata;
    init_iccbased_cdata(cdata);
    (*cdata).colorspace = colorspace;
    memcpy(
        (*cdata).checksum.as_mut_ptr() as *mut libc::c_void,
        checksum.as_mut_ptr() as *const libc::c_void,
        16i32 as u64,
    );
    cspc_id = pdf_colorspace_findresource(ident, 4i32, cdata as *const libc::c_void);
    if cspc_id >= 0i32 {
        if verbose != 0 {
            dpx_message(
                b"(ICCP:[id=%d])\x00" as *const u8 as *const libc::c_char,
                cspc_id,
            );
        }
        release_iccbased_cdata(cdata);
        return cspc_id;
    }
    if verbose > 1i32 {
        print_iccp_header(&mut icch, checksum.as_mut_ptr());
    }
    resource = pdf_new_array();
    stream = pdf_new_stream(1i32 << 0i32);
    pdf_add_array(
        resource,
        pdf_new_name(b"ICCBased\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_array(resource, pdf_ref_obj(stream));
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"N\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(get_num_components_iccbased(cdata) as libc::c_double),
    );
    pdf_add_stream(stream, profile, proflen);
    pdf_release_obj(stream);
    cspc_id = pdf_colorspace_defineresource(ident, 4i32, cdata as *mut libc::c_void, resource);
    return cspc_id;
}
static mut cspc_cache: C2RustUnnamed_0 = {
    let mut init = C2RustUnnamed_0 {
        count: 0i32 as libc::c_uint,
        capacity: 0i32 as libc::c_uint,
        colorspaces: 0 as *const pdf_colorspace as *mut pdf_colorspace,
    };
    init
};
unsafe extern "C" fn pdf_colorspace_findresource(
    mut ident: *const libc::c_char,
    mut type_0: libc::c_int,
    mut cdata: *const libc::c_void,
) -> libc::c_int {
    let mut colorspace: *mut pdf_colorspace = 0 as *mut pdf_colorspace;
    let mut cspc_id: libc::c_int = 0;
    let mut cmp: libc::c_int = -1i32;
    cspc_id = 0i32;
    while cmp != 0 && (cspc_id as libc::c_uint) < cspc_cache.count {
        colorspace = &mut *cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
        if !((*colorspace).subtype != type_0) {
            match (*colorspace).subtype {
                4 => {
                    cmp = compare_iccbased(
                        ident,
                        cdata as *const iccbased_cdata,
                        (*colorspace).ident,
                        (*colorspace).cdata as *const iccbased_cdata,
                    )
                }
                _ => {}
            }
            if cmp == 0 {
                return cspc_id;
            }
        }
        cspc_id += 1
    }
    return -1i32;
    /* not found */
}
unsafe extern "C" fn pdf_init_colorspace_struct(mut colorspace: *mut pdf_colorspace) {
    if !colorspace.is_null() {
    } else {
        __assert_fail(
            b"colorspace\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            1044i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void pdf_init_colorspace_struct(pdf_colorspace *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*colorspace).ident = 0 as *mut libc::c_char;
    (*colorspace).subtype = 0i32;
    (*colorspace).resource = 0 as *mut pdf_obj;
    (*colorspace).reference = 0 as *mut pdf_obj;
    (*colorspace).cdata = 0 as *mut libc::c_void;
}
unsafe extern "C" fn pdf_clean_colorspace_struct(mut colorspace: *mut pdf_colorspace) {
    if !colorspace.is_null() {
    } else {
        __assert_fail(
            b"colorspace\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            1059i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void pdf_clean_colorspace_struct(pdf_colorspace *)\x00",
            ))
            .as_ptr(),
        );
    }
    free((*colorspace).ident as *mut libc::c_void);
    pdf_release_obj((*colorspace).resource);
    pdf_release_obj((*colorspace).reference);
    (*colorspace).resource = 0 as *mut pdf_obj;
    (*colorspace).reference = 0 as *mut pdf_obj;
    if !(*colorspace).cdata.is_null() {
        match (*colorspace).subtype {
            4 => {
                release_iccbased_cdata((*colorspace).cdata as *mut iccbased_cdata);
            }
            _ => {}
        }
    }
    (*colorspace).cdata = 0 as *mut libc::c_void;
    (*colorspace).subtype = 0i32;
}
unsafe extern "C" fn pdf_flush_colorspace(mut colorspace: *mut pdf_colorspace) {
    if !colorspace.is_null() {
    } else {
        __assert_fail(
            b"colorspace\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfcolor.c\x00" as *const u8 as *const libc::c_char,
            1083i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void pdf_flush_colorspace(pdf_colorspace *)\x00",
            ))
            .as_ptr(),
        );
    }
    pdf_release_obj((*colorspace).resource);
    pdf_release_obj((*colorspace).reference);
    (*colorspace).resource = 0 as *mut pdf_obj;
    (*colorspace).reference = 0 as *mut pdf_obj;
}
/* **************************** COLOR SPACE *****************************/
unsafe extern "C" fn pdf_colorspace_defineresource(
    mut ident: *const libc::c_char,
    mut subtype: libc::c_int,
    mut cdata: *mut libc::c_void,
    mut resource: *mut pdf_obj,
) -> libc::c_int {
    let mut cspc_id: libc::c_int = 0; /* .... */
    let mut colorspace: *mut pdf_colorspace = 0 as *mut pdf_colorspace;
    if cspc_cache.count >= cspc_cache.capacity {
        cspc_cache.capacity = cspc_cache.capacity.wrapping_add(16i32 as libc::c_uint);
        cspc_cache.colorspaces = renew(
            cspc_cache.colorspaces as *mut libc::c_void,
            (cspc_cache.capacity as u64)
                .wrapping_mul(::std::mem::size_of::<pdf_colorspace>() as u64)
                as u32,
        ) as *mut pdf_colorspace
    }
    cspc_id = cspc_cache.count as libc::c_int;
    colorspace = &mut *cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    pdf_init_colorspace_struct(colorspace);
    if !ident.is_null() {
        (*colorspace).ident = new(
            (strlen(ident).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
                as u32,
        ) as *mut libc::c_char;
        strcpy((*colorspace).ident, ident);
    }
    (*colorspace).subtype = subtype;
    (*colorspace).cdata = cdata;
    (*colorspace).resource = resource;
    if verbose != 0 {
        dpx_message(
            b"(ColorSpace:%s\x00" as *const u8 as *const libc::c_char,
            ident,
        );
        if verbose > 1i32 {
            match subtype {
                4 => {
                    dpx_message(b"[ICCBased]\x00" as *const u8 as *const libc::c_char);
                }
                3 => {
                    dpx_message(b"[CalRGB]\x00" as *const u8 as *const libc::c_char);
                }
                1 => {
                    dpx_message(b"[CalGray]\x00" as *const u8 as *const libc::c_char);
                }
                _ => {}
            }
        }
        dpx_message(b")\x00" as *const u8 as *const libc::c_char);
    }
    cspc_cache.count = cspc_cache.count.wrapping_add(1);
    return cspc_id;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_colorspace_reference(mut cspc_id: libc::c_int) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_colorspace = 0 as *mut pdf_colorspace;
    colorspace = &mut *cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    if (*colorspace).reference.is_null() {
        (*colorspace).reference = pdf_ref_obj((*colorspace).resource);
        pdf_release_obj((*colorspace).resource);
        (*colorspace).resource = 0 as *mut pdf_obj
    }
    return pdf_link_obj((*colorspace).reference);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_colors() {
    cspc_cache.count = 0i32 as libc::c_uint;
    cspc_cache.capacity = 0i32 as libc::c_uint;
    cspc_cache.colorspaces = 0 as *mut pdf_colorspace;
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
/* Not check size */
/* returns colorspace ID */
#[no_mangle]
pub unsafe extern "C" fn pdf_close_colors() {
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < cspc_cache.count {
        let mut colorspace: *mut pdf_colorspace = 0 as *mut pdf_colorspace;
        colorspace = &mut *cspc_cache.colorspaces.offset(i as isize) as *mut pdf_colorspace;
        pdf_flush_colorspace(colorspace);
        pdf_clean_colorspace_struct(colorspace);
        i = i.wrapping_add(1)
    }
    cspc_cache.colorspaces =
        mfree(cspc_cache.colorspaces as *mut libc::c_void) as *mut pdf_colorspace;
    cspc_cache.capacity = 0i32 as libc::c_uint;
    cspc_cache.count = cspc_cache.capacity;
}
