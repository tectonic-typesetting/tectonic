#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           label_break_value,
           ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn parse_c_ident(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_init_fontmap_record(mrec: *mut fontmap_rec);
    #[no_mangle]
    fn pdf_clear_fontmap_record(mrec: *mut fontmap_rec);
    #[no_mangle]
    fn pdf_load_fontmap_file(filename: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_read_fontmap_line(
        mrec: *mut fontmap_rec,
        mline: *const libc::c_char,
        mline_strlen: libc::c_int,
        format: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_append_fontmap_record(kp: *const libc::c_char, mrec: *const fontmap_rec) -> libc::c_int;
    #[no_mangle]
    fn pdf_remove_fontmap_record(kp: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pdf_insert_fontmap_record(
        kp: *const libc::c_char,
        mrec: *const fontmap_rec,
    ) -> *mut fontmap_rec;
    #[no_mangle]
    fn is_pdfm_mapline(mline: *const libc::c_char) -> libc::c_int;
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
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    static mut work_buffer: [libc::c_char; 0];
    /* Text composition mode is ignored (always same as font's
     * writing mode) and glyph rotation is not enabled if
     * auto_rotate is unset.
     */
    /*
     * For pdf_doc, pdf_draw and others.
     */
    /* Force reselecting font and color:
     * XFrom (content grabbing) and Metapost support want them.
     */
    #[no_mangle]
    fn pdf_dev_reset_fonts(newpage: libc::c_int);
    #[no_mangle]
    fn pdf_dev_reset_color(force: libc::c_int);
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const libc::c_char, length: libc::c_uint);
    /* Similar to bop_content */
    #[no_mangle]
    fn pdf_doc_set_bgcolor(color: *const pdf_color);
    #[no_mangle]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_gsave() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_grestore() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_set_fixed_point(x: libc::c_double, y: libc::c_double);
    #[no_mangle]
    fn pdf_dev_get_fixed_point(p: *mut pdf_coord);
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn parse_ident(start: *mut *const libc::c_char, end: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn parse_val_ident(
        start: *mut *const libc::c_char,
        end: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    /* syntax 1: ((rgb|cmyk|hsb|gray) colorvalues)|colorname
     * syntax 0: pdf_number|pdf_array
     *
     * This is for reading *single* color specification.
     */
    #[no_mangle]
    fn spc_util_read_colorspec(
        spe: *mut spc_env,
        colorspec: *mut pdf_color,
        args: *mut spc_arg,
        syntax: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn spc_util_read_numbers(
        values: *mut libc::c_double,
        num_values: libc::c_int,
        args: *mut spc_arg,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: libc::c_double,
    pub y_user: libc::c_double,
    pub mag: libc::c_double,
    pub pg: libc::c_int,
    /* current page in PDF */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const libc::c_char,
    pub endptr: *const libc::c_char,
    pub base: *const libc::c_char,
    pub command: *const libc::c_char,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const libc::c_char,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut libc::c_char,
    pub font_name: *mut libc::c_char,
    pub enc_name: *mut libc::c_char,
    pub charmap: C2RustUnnamed,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: libc::c_double,
    pub extend: libc::c_double,
    pub bold: libc::c_double,
    pub mapc: libc::c_int,
    pub flags: libc::c_int,
    pub otl_tags: *mut libc::c_char,
    pub tounicode: *mut libc::c_char,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: libc::c_double,
    pub charcoll: *mut libc::c_char,
    pub index: libc::c_int,
    pub style: libc::c_int,
    pub stemv: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sfd_name: *mut libc::c_char,
    pub subfont_id: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub f: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub num_components: libc::c_int,
    pub spot_color_name: *mut libc::c_char,
    pub values: [libc::c_double; 4],
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/*  This is xdvipdfmx, an extended version of dvipdfmx,
    an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2013-2016 by the dvipdfmx project team.

    Copyright (c) 2006 SIL International
    Originally written by Jonathan Kew

    This file based on spc_pdfm.c, part of the dvipdfmx project:

    Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata.

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
pub unsafe extern "C" fn spc_handler_xtx_do_transform(
    mut x_user: libc::c_double,
    mut y_user: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut d: libc::c_double,
    mut e: libc::c_double,
    mut f: libc::c_double,
) -> libc::c_int {
    let mut M: pdf_tmatrix = {
        let mut init = pdf_tmatrix {
            a: 0i32 as libc::c_double,
            b: 0i32 as libc::c_double,
            c: 0i32 as libc::c_double,
            d: 0i32 as libc::c_double,
            e: 0i32 as libc::c_double,
            f: 0i32 as libc::c_double,
        };
        init
    };
    let mut pt: pdf_coord = pdf_coord { x: 0., y: 0. };
    /* Create transformation matrix */
    M.a = a;
    M.b = b;
    M.c = c;
    M.d = d;
    M.e = (1.0f64 - M.a) * x_user - M.c * y_user + e;
    M.f = (1.0f64 - M.d) * y_user - M.b * x_user + f;
    pdf_dev_concat(&mut M);
    pdf_dev_get_fixed_point(&mut pt);
    pdf_dev_set_fixed_point(x_user - pt.x, y_user - pt.y);
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_scale(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut values: [libc::c_double; 2] = [0.; 2];
    if spc_util_read_numbers(&mut *values.as_mut_ptr().offset(0), 2i32, args) < 2i32 {
        return -1i32;
    }
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        (*spe).x_user,
        (*spe).y_user,
        values[0],
        0i32 as libc::c_double,
        0i32 as libc::c_double,
        values[1],
        0i32 as libc::c_double,
        0i32 as libc::c_double,
    );
}
/* Scaling without gsave/grestore. */
static mut scaleFactors: *mut pdf_coord = 0 as *const pdf_coord as *mut pdf_coord;
static mut scaleFactorCount: libc::c_int = -1i32;
unsafe extern "C" fn spc_handler_xtx_bscale(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut values: [libc::c_double; 2] = [0.; 2];
    scaleFactorCount += 1;
    if scaleFactorCount & 0xfi32 == 0 {
        scaleFactors = xrealloc(
            scaleFactors as *mut libc::c_void,
            ((scaleFactorCount + 16i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pdf_coord>() as libc::c_ulong),
        ) as *mut pdf_coord
    }
    if spc_util_read_numbers(&mut *values.as_mut_ptr().offset(0), 2i32, args) < 2i32 {
        return -1i32;
    }
    if fabs(values[0]) < 1.0e-7f64 || fabs(values[1]) < 1.0e-7f64 {
        return -1i32;
    }
    (*scaleFactors.offset(scaleFactorCount as isize)).x = 1i32 as libc::c_double / values[0];
    (*scaleFactors.offset(scaleFactorCount as isize)).y = 1i32 as libc::c_double / values[1];
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        (*spe).x_user,
        (*spe).y_user,
        values[0],
        0i32 as libc::c_double,
        0i32 as libc::c_double,
        values[1],
        0i32 as libc::c_double,
        0i32 as libc::c_double,
    );
}
unsafe extern "C" fn spc_handler_xtx_escale(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let fresh0 = scaleFactorCount;
    scaleFactorCount = scaleFactorCount - 1;
    let mut factor: pdf_coord = *scaleFactors.offset(fresh0 as isize);
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        (*spe).x_user,
        (*spe).y_user,
        factor.x,
        0i32 as libc::c_double,
        0i32 as libc::c_double,
        factor.y,
        0i32 as libc::c_double,
        0i32 as libc::c_double,
    );
}
unsafe extern "C" fn spc_handler_xtx_rotate(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut value: libc::c_double = 0.;
    if spc_util_read_numbers(&mut value, 1i32, args) < 1i32 {
        return -1i32;
    }
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        (*spe).x_user,
        (*spe).y_user,
        cos(value * 3.14159265358979323846f64 / 180i32 as libc::c_double),
        sin(value * 3.14159265358979323846f64 / 180i32 as libc::c_double),
        -sin(value * 3.14159265358979323846f64 / 180i32 as libc::c_double),
        cos(value * 3.14159265358979323846f64 / 180i32 as libc::c_double),
        0i32 as libc::c_double,
        0i32 as libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn spc_handler_xtx_gsave(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    pdf_dev_gsave();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_handler_xtx_grestore(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    pdf_dev_grestore();
    /*
     * Unfortunately, the following line is necessary in case
     * of a font or color change inside of the save/restore pair.
     * Anything that was done there must be redone, so in effect,
     * we make no assumptions about what fonts. We act like we are
     * starting a new page.
     */
    pdf_dev_reset_fonts(0i32);
    pdf_dev_reset_color(0i32);
    return 0i32;
}
/* Please remove this.
 * This should be handled before processing pages!
 */
unsafe extern "C" fn spc_handler_xtx_papersize(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_backgroundcolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut colorspec: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut libc::c_char,
        values: [0.; 4],
    };
    error = spc_util_read_colorspec(spe, &mut colorspec, args, 0i32);
    if error != 0 {
        spc_warn(
            spe,
            b"No valid color specified?\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        pdf_doc_set_bgcolor(&mut colorspec);
    }
    return error;
}
/* FIXME: xdv2pdf's x:fontmapline and x:fontmapfile may have slightly different syntax/semantics */
unsafe extern "C" fn spc_handler_xtx_fontmapline(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut map_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opchr: libc::c_char = 0;
    let mut error: libc::c_int = 0i32;
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        spc_warn(
            spe,
            b"Empty fontmapline special?\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    opchr = *(*ap).curptr.offset(0);
    if opchr as libc::c_int == '-' as i32 || opchr as libc::c_int == '+' as i32 {
        (*ap).curptr = (*ap).curptr.offset(1)
    }
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    match opchr as libc::c_int {
        45 => {
            map_name = parse_ident(&mut (*ap).curptr, (*ap).endptr);
            if !map_name.is_null() {
                pdf_remove_fontmap_record(map_name);
                free(map_name as *mut libc::c_void);
            } else {
                spc_warn(
                    spe,
                    b"Invalid fontmap line: Missing TFM name.\x00" as *const u8
                        as *const libc::c_char,
                );
                error = -1i32
            }
        }
        _ => {
            p = (*ap).curptr;
            q = buffer.as_mut_ptr();
            while p < (*ap).endptr {
                let fresh1 = p;
                p = p.offset(1);
                let fresh2 = q;
                q = q.offset(1);
                *fresh2 = *fresh1
            }
            *q = '\u{0}' as i32 as libc::c_char;
            mrec = new((1i32 as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fontmap_rec>() as libc::c_ulong)
                as uint32_t) as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            error = pdf_read_fontmap_line(
                mrec,
                buffer.as_mut_ptr(),
                (*ap).endptr.wrapping_offset_from((*ap).curptr) as libc::c_long as libc::c_int,
                is_pdfm_mapline(buffer.as_mut_ptr()),
            );
            if error != 0 {
                spc_warn(
                    spe,
                    b"Invalid fontmap line.\x00" as *const u8 as *const libc::c_char,
                );
            } else if opchr as libc::c_int == '+' as i32 {
                pdf_append_fontmap_record((*mrec).map_name, mrec);
            } else {
                pdf_insert_fontmap_record((*mrec).map_name, mrec);
            }
            pdf_clear_fontmap_record(mrec);
            free(mrec as *mut libc::c_void);
        }
    }
    if error == 0 {
        (*ap).curptr = (*ap).endptr
    }
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_fontmapfile(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut mapfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return 0i32;
    }
    match *(*args).curptr.offset(0) as libc::c_int {
        45 => {
            mode = '-' as i32;
            (*args).curptr = (*args).curptr.offset(1)
        }
        43 => {
            mode = '+' as i32;
            (*args).curptr = (*args).curptr.offset(1)
        }
        _ => mode = 0i32,
    }
    mapfile = parse_val_ident(&mut (*args).curptr, (*args).endptr);
    if mapfile.is_null() {
        spc_warn(
            spe,
            b"No fontmap file specified.\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    } else {
        error = pdf_load_fontmap_file(mapfile, mode)
    }
    return error;
}
static mut overlay_name: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn spc_handler_xtx_initoverlay(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -1i32;
    }
    strncpy(
        overlay_name.as_mut_ptr(),
        (*args).curptr,
        (*args).endptr.wrapping_offset_from((*args).curptr) as libc::c_long as libc::c_ulong,
    );
    overlay_name[(*args).endptr.wrapping_offset_from((*args).curptr) as libc::c_long as usize] =
        0i32 as libc::c_char;
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_clipoverlay(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -1i32;
    }
    pdf_dev_grestore();
    pdf_dev_gsave();
    if strncmp(
        overlay_name.as_mut_ptr(),
        (*args).curptr,
        strlen(overlay_name.as_mut_ptr()),
    ) != 0i32
        && strncmp(
            b"all\x00" as *const u8 as *const libc::c_char,
            (*args).curptr,
            strlen(b"all\x00" as *const u8 as *const libc::c_char),
        ) != 0i32
    {
        pdf_doc_add_page_content(
            b" 0 0 m W n\x00" as *const u8 as *const libc::c_char,
            10i32 as libc::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_renderingmode(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut value: libc::c_double = 0.;
    if spc_util_read_numbers(&mut value, 1i32, args) < 1i32 {
        return -1i32;
    }
    if (value as libc::c_int) < 0i32 || value as libc::c_int > 7i32 {
        spc_warn(
            spe,
            b"Invalid text rendering mode %d.\n\x00" as *const u8 as *const libc::c_char,
            value as libc::c_int,
        );
        return -1i32;
    }
    sprintf(
        work_buffer.as_mut_ptr(),
        b" %d Tr\x00" as *const u8 as *const libc::c_char,
        value as libc::c_int,
    );
    pdf_doc_add_page_content(
        work_buffer.as_mut_ptr(),
        strlen(work_buffer.as_mut_ptr()) as libc::c_uint,
    );
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        pdf_doc_add_page_content(
            b" \x00" as *const u8 as *const libc::c_char,
            1i32 as libc::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.wrapping_offset_from((*args).curptr) as libc::c_long as libc::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_unsupportedcolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    spc_warn(spe,
             b"xetex-style \\special{x:%s} is not supported by this driver;\nupdate document or driver to use \\special{color} instead.\x00"
                 as *const u8 as *const libc::c_char, (*args).command);
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn spc_handler_xtx_unsupported(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    spc_warn(
        spe,
        b"xetex-style \\special{x:%s} is not supported by this driver.\x00" as *const u8
            as *const libc::c_char,
        (*args).command,
    );
    (*args).curptr = (*args).endptr;
    return 0i32;
}
static mut xtx_handlers: [spc_handler; 21] = unsafe {
    [
        {
            let mut init = spc_handler {
                key: b"textcolor\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupportedcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"textcolorpush\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupportedcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"textcolorpop\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupportedcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"rulecolor\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupportedcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"rulecolorpush\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupportedcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"rulecolorpop\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupportedcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"papersize\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_papersize
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"backgroundcolor\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_backgroundcolor
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"gsave\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_gsave
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"grestore\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_grestore
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"scale\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_scale
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bscale\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_bscale
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"escale\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_escale
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"rotate\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_rotate
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"fontmapline\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_fontmapline
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"fontmapfile\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_fontmapfile
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"shadow\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupported
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"colorshadow\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_unsupported
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"renderingmode\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_renderingmode
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"initoverlay\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_initoverlay
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"clipoverlay\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_xtx_clipoverlay
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_xtx_check_special(
    mut buf: *const libc::c_char,
    mut len: libc::c_int,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p.offset(strlen(b"x:\x00" as *const u8 as *const libc::c_char) as isize) <= endptr
        && memcmp(
            p as *const libc::c_void,
            b"x:\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"x:\x00" as *const u8 as *const libc::c_char),
        ) == 0
    {
        return 1i32 != 0;
    }
    return 0i32 != 0;
}
/*  This is xdvipdfmx, an extended version of dvipdfmx,
    an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2013-2016 by the dvipdfmx project team.

    Copyright (c) 2006 SIL International
    Originally written by Jonathan Kew

    Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata,
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
pub unsafe extern "C" fn spc_xtx_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = -1i32;
    let mut i: libc::c_uint = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if !sph.is_null() && !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(b"sph && spe && ap\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-spc_xtx.c\x00" as *const u8 as
                          *const libc::c_char, 413i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"int spc_xtx_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    if (*ap)
        .curptr
        .offset(strlen(b"x:\x00" as *const u8 as *const libc::c_char) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const libc::c_void,
            b"x:\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"x:\x00" as *const u8 as *const libc::c_char),
        ) != 0
    {
        spc_warn(
            spe,
            b"Not x: special???\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"x:\x00" as *const u8 as *const libc::c_char) as isize);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        i = 0i32 as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[spc_handler; 21]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<spc_handler>() as libc::c_ulong)
        {
            if streq_ptr(q, xtx_handlers[i as usize].key) {
                (*ap).command = xtx_handlers[i as usize].key;
                (*sph).key = b"x:\x00" as *const u8 as *const libc::c_char;
                (*sph).exec = xtx_handlers[i as usize].exec;
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                error = 0i32;
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
        free(q as *mut libc::c_void);
    }
    return error;
}
