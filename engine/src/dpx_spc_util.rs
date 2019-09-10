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
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn cos(_: f64) -> f64;
    #[no_mangle]
    fn sin(_: f64) -> f64;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    #[no_mangle]
    fn pdf_color_spotcolor(color: *mut pdf_color, color_name: *mut i8, c: f64) -> i32;
    #[no_mangle]
    fn pdf_color_graycolor(color: *mut pdf_color, g: f64) -> i32;
    #[no_mangle]
    fn pdf_color_cmykcolor(color: *mut pdf_color, c: f64, m: f64, y: f64, k: f64) -> i32;
    #[no_mangle]
    fn pdf_color_rgbcolor(color: *mut pdf_color, r: f64, g: f64, b: f64) -> i32;
    #[no_mangle]
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn parse_float_decimal(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
}
pub type C2RustUnnamed = u32;
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
pub struct pdf_color {
    pub num_components: i32,
    pub spot_color_name: *mut i8,
    pub values: [f64; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: f64,
    pub y_user: f64,
    pub mag: f64,
    pub pg: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const i8,
    pub endptr: *const i8,
    pub base: *const i8,
    pub command: *const i8,
}
/* Color names */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colordef_ {
    pub key: *const i8,
    pub color: pdf_color,
}
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return false;
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

    Copyright (C) 2007-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
unsafe extern "C" fn skip_blank(mut pp: *mut *const i8, mut endptr: *const i8) {
    let mut p: *const i8 = *pp; /* 360 / 60 */
    while p < endptr
        && (*p as i32 & !0x7fi32 == 0i32
            && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                & _ISblank as i32 as u16 as i32
                != 0)
    {
        p = p.offset(1)
    }
    *pp = p;
}
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_numbers(
    mut values: *mut f64,
    mut num_values: i32,
    mut args: *mut spc_arg,
) -> i32 {
    let mut count: i32 = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    skip_blank(&mut (*args).curptr, (*args).endptr);
    count = 0i32;
    while count < num_values && (*args).curptr < (*args).endptr {
        q = parse_float_decimal(&mut (*args).curptr, (*args).endptr);
        if q.is_null() {
            break;
        }
        *values.offset(count as isize) = atof(q);
        free(q as *mut libc::c_void);
        skip_blank(&mut (*args).curptr, (*args).endptr);
        count += 1
    }
    return count;
}
unsafe extern "C" fn rgb_color_from_hsv(
    mut color: *mut pdf_color,
    mut h: f64,
    mut s: f64,
    mut v: f64,
) {
    let mut r: f64 = 0.;
    let mut g: f64 = 0.;
    let mut b: f64 = 0.;
    assert!(!color.is_null());
    b = v;
    g = b;
    r = g;
    if s != 0.0f64 {
        let mut h6: f64 = 0.;
        let mut f: f64 = 0.;
        let mut v1: f64 = 0.;
        let mut v2: f64 = 0.;
        let mut v3: f64 = 0.;
        let mut i: i32 = 0;
        h6 = h * 6i32 as f64;
        i = h6 as i32;
        f = h6 - i as f64;
        v1 = v * (1i32 as f64 - s);
        v2 = v * (1i32 as f64 - s * f);
        v3 = v * (1i32 as f64 - s * (1i32 as f64 - f));
        match i {
            0 => {
                r = v;
                g = v3;
                b = v1
            }
            1 => {
                r = v2;
                g = v;
                b = v1
            }
            2 => {
                r = v1;
                g = v;
                b = v3
            }
            3 => {
                r = v1;
                g = v2;
                b = v
            }
            4 => {
                r = v3;
                g = v1;
                b = v
            }
            5 => {
                r = v;
                g = v1;
                b = v2
            }
            6 => {
                r = v;
                g = v1;
                b = v2
            }
            _ => {}
        }
    }
    pdf_color_rgbcolor(color, r, g, b);
}
unsafe extern "C" fn spc_read_color_color(
    mut spe: *mut spc_env,
    mut colorspec: *mut pdf_color,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut cv: [f64; 4] = [0.; 4];
    let mut nc: i32 = 0;
    let mut error: i32 = 0i32;
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        spc_warn(
            spe,
            b"No valid color specified?\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if streq_ptr(q, b"rgb\x00" as *const u8 as *const i8) {
        /* Handle rgb color */
        nc = spc_util_read_numbers(cv.as_mut_ptr(), 3i32, ap);
        if nc != 3i32 {
            spc_warn(
                spe,
                b"Invalid value for RGB color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            pdf_color_rgbcolor(colorspec, cv[0], cv[1], cv[2]);
        }
    } else if streq_ptr(q, b"cmyk\x00" as *const u8 as *const i8) {
        /* Handle cmyk color */
        nc = spc_util_read_numbers(cv.as_mut_ptr(), 4i32, ap);
        if nc != 4i32 {
            spc_warn(
                spe,
                b"Invalid value for CMYK color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            pdf_color_cmykcolor(colorspec, cv[0], cv[1], cv[2], cv[3]);
        }
    } else if streq_ptr(q, b"gray\x00" as *const u8 as *const i8) {
        /* Handle gray */
        nc = spc_util_read_numbers(cv.as_mut_ptr(), 1i32, ap);
        if nc != 1i32 {
            spc_warn(
                spe,
                b"Invalid value for gray color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            pdf_color_graycolor(colorspec, cv[0]);
        }
    } else if streq_ptr(q, b"spot\x00" as *const u8 as *const i8) {
        /* Handle spot colors */
        let mut color_name: *mut i8 = parse_c_ident(&mut (*ap).curptr, (*ap).endptr); /* Must be a "named" color */
        if color_name.is_null() {
            spc_warn(
                spe,
                b"No valid spot color name specified?\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        nc = spc_util_read_numbers(cv.as_mut_ptr(), 1i32, ap);
        if nc != 1i32 {
            spc_warn(
                spe,
                b"Invalid value for spot color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32;
            free(color_name as *mut libc::c_void);
        } else {
            pdf_color_spotcolor(colorspec, color_name, cv[0]);
        }
    } else if streq_ptr(q, b"hsb\x00" as *const u8 as *const i8) {
        nc = spc_util_read_numbers(cv.as_mut_ptr(), 3i32, ap);
        if nc != 3i32 {
            spc_warn(
                spe,
                b"Invalid value for HSB color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            rgb_color_from_hsv(colorspec, cv[0], cv[1], cv[2]);
            spc_warn(
                spe,
                b"HSB color converted to RGB: hsb: <%g, %g, %g> ==> rgb: <%g, %g, %g>\x00"
                    as *const u8 as *const i8,
                cv[0],
                cv[1],
                cv[2],
                (*colorspec).values[0],
                (*colorspec).values[1],
                (*colorspec).values[2],
            );
        }
    } else {
        error = pdf_color_namedcolor(colorspec, q);
        if error != 0 {
            spc_warn(
                spe,
                b"Unrecognized color name: %s\x00" as *const u8 as *const i8,
                q,
            );
        }
    }
    free(q as *mut libc::c_void);
    return error;
}
/* Argument for this is PDF_Number or PDF_Array.
 * But we ignore that since we don't want to add
 * dependency to pdfxxx and @foo can not be
 * allowed for color specification. "pdf" here
 * means pdf: special syntax.
 */
unsafe extern "C" fn spc_read_color_pdf(
    mut spe: *mut spc_env,
    mut colorspec: *mut pdf_color,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut cv: [f64; 4] = [0.; 4]; /* at most four */
    let mut nc: i32 = 0;
    let mut isarry: i32 = 0i32;
    let mut error: i32 = 0i32;
    let mut q: *mut i8 = 0 as *mut i8;
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if *(*ap).curptr.offset(0) as i32 == '[' as i32 {
        (*ap).curptr = (*ap).curptr.offset(1);
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        isarry = 1i32
    }
    nc = spc_util_read_numbers(cv.as_mut_ptr(), 4i32, ap);
    match nc {
        1 => {
            pdf_color_graycolor(colorspec, cv[0]);
        }
        3 => {
            pdf_color_rgbcolor(colorspec, cv[0], cv[1], cv[2]);
        }
        4 => {
            pdf_color_cmykcolor(colorspec, cv[0], cv[1], cv[2], cv[3]);
        }
        _ => {
            /* Try to read the color names defined in dvipsname.def */
            q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
            if q.is_null() {
                spc_warn(
                    spe,
                    b"No valid color specified?\x00" as *const u8 as *const i8,
                );
                return -1i32;
            }
            error = pdf_color_namedcolor(colorspec, q);
            if error != 0 {
                spc_warn(
                    spe,
                    b"Unrecognized color name: %s, keep the current color\x00" as *const u8
                        as *const i8,
                    q,
                );
            }
            free(q as *mut libc::c_void);
        }
    }
    if isarry != 0 {
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        if (*ap).curptr >= (*ap).endptr || *(*ap).curptr.offset(0) as i32 != ']' as i32 {
            spc_warn(
                spe,
                b"Unbalanced \'[\' and \']\' in color specification.\x00" as *const u8 as *const i8,
            );
            error = -1i32
        } else {
            (*ap).curptr = (*ap).curptr.offset(1)
        }
    }
    return error;
}
/* This is for reading *single* color specification. */
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_colorspec(
    mut spe: *mut spc_env,
    mut colorspec: *mut pdf_color,
    mut ap: *mut spc_arg,
    mut syntax: i32,
) -> i32 {
    assert!(!colorspec.is_null() && !spe.is_null() && !ap.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -1i32;
    }
    if syntax != 0 {
        return spc_read_color_color(spe, colorspec, ap);
    } else {
        return spc_read_color_pdf(spe, colorspec, ap);
    };
}
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_pdfcolor(
    mut spe: *mut spc_env,
    mut colorspec: *mut pdf_color,
    mut ap: *mut spc_arg,
    mut defaultcolor: *mut pdf_color,
) -> i32 {
    let mut error: i32 = 0i32;
    assert!(!colorspec.is_null() && !spe.is_null() && !ap.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -1i32;
    }
    error = spc_read_color_pdf(spe, colorspec, ap);
    if error < 0i32 && !defaultcolor.is_null() {
        pdf_color_copycolor(colorspec, defaultcolor);
        error = 0i32
    }
    return error;
}
/* This need to allow 'true' prefix for unit and
 * length value must be divided by current magnification.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn spc_util_read_length(
    mut spe: *mut spc_env,
    mut vp: *mut f64,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8; /* inverse magnify */
    let mut v: f64 = 0.;
    let mut u: f64 = 1.0f64;
    let mut ukeys: [*const i8; 10] = [
        b"pt\x00" as *const u8 as *const i8,
        b"in\x00" as *const u8 as *const i8,
        b"cm\x00" as *const u8 as *const i8,
        b"mm\x00" as *const u8 as *const i8,
        b"bp\x00" as *const u8 as *const i8,
        b"pc\x00" as *const u8 as *const i8,
        b"dd\x00" as *const u8 as *const i8,
        b"cc\x00" as *const u8 as *const i8,
        b"sp\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut k: i32 = 0;
    let mut error: i32 = 0i32;
    q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        return -1i32;
    }
    v = atof(q);
    free(q as *mut libc::c_void);
    skip_white(&mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        let mut qq: *mut i8 = q;
        if strlen(q) >= strlen(b"true\x00" as *const u8 as *const i8)
            && memcmp(
                q as *const libc::c_void,
                b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"true\x00" as *const u8 as *const i8),
            ) == 0
        {
            u /= if (*spe).mag != 0.0f64 {
                (*spe).mag
            } else {
                1.0f64
            };
            q = q.offset(strlen(b"true\x00" as *const u8 as *const i8) as isize);
            if *q == 0 {
                free(qq as *mut libc::c_void);
                skip_white(&mut (*ap).curptr, (*ap).endptr);
                q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
                qq = q
            }
        }
        if !q.is_null() {
            k = 0i32;
            while !ukeys[k as usize].is_null() && strcmp(ukeys[k as usize], q) != 0 {
                k += 1
            }
            match k {
                0 => u *= 72.0f64 / 72.27f64,
                1 => u *= 72.0f64,
                2 => u *= 72.0f64 / 2.54f64,
                3 => u *= 72.0f64 / 25.4f64,
                4 => u *= 1.0f64,
                5 => u *= 12.0f64 * 72.0f64 / 72.27f64,
                6 => u *= 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                7 => u *= 12.0f64 * 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                8 => u *= 72.0f64 / (72.27f64 * 65536i32 as f64),
                _ => {
                    spc_warn(
                        spe,
                        b"Unknown unit of measure: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    error = -1i32
                }
            }
            free(qq as *mut libc::c_void);
        } else {
            spc_warn(
                spe,
                b"Missing unit of measure after \"true\"\x00" as *const u8 as *const i8,
            );
            error = -1i32
        }
    }
    *vp = v * u;
    return error;
}
/*
 * Compute a transformation matrix
 * transformations are applied in the following
 * order: scaling, rotate, displacement.
 */
unsafe extern "C" fn make_transmatrix(
    mut M: *mut pdf_tmatrix,
    mut xoffset: f64,
    mut yoffset: f64,
    mut xscale: f64,
    mut yscale: f64,
    mut rotate: f64,
) {
    let mut c: f64 = 0.;
    let mut s: f64 = 0.;
    c = cos(rotate);
    s = sin(rotate);
    (*M).a = xscale * c;
    (*M).b = xscale * s;
    (*M).c = -yscale * s;
    (*M).d = yscale * c;
    (*M).e = xoffset;
    (*M).f = yoffset;
}
unsafe extern "C" fn spc_read_dimtrns_dvips(
    mut spe: *mut spc_env,
    mut t: *mut transform_info,
    mut ap: *mut spc_arg,
) -> i32 {
    static mut _dtkeys: [*const i8; 15] = [
        b"hoffset\x00" as *const u8 as *const i8,
        b"voffset\x00" as *const u8 as *const i8,
        b"hsize\x00" as *const u8 as *const i8,
        b"vsize\x00" as *const u8 as *const i8,
        b"hscale\x00" as *const u8 as *const i8,
        b"vscale\x00" as *const u8 as *const i8,
        b"angle\x00" as *const u8 as *const i8,
        b"clip\x00" as *const u8 as *const i8,
        b"llx\x00" as *const u8 as *const i8,
        b"lly\x00" as *const u8 as *const i8,
        b"urx\x00" as *const u8 as *const i8,
        b"ury\x00" as *const u8 as *const i8,
        b"rwi\x00" as *const u8 as *const i8,
        b"rhi\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut xoffset: f64 = 0.;
    let mut yoffset: f64 = 0.;
    let mut xscale: f64 = 0.;
    let mut yscale: f64 = 0.;
    let mut rotate: f64 = 0.;
    let mut error: i32 = 0i32;
    rotate = 0.0f64;
    yoffset = rotate;
    xoffset = yoffset;
    yscale = 1.0f64;
    xscale = yscale;
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let mut kp: *mut i8 = 0 as *mut i8;
        let mut vp: *mut i8 = 0 as *mut i8;
        let mut k: i32 = 0;
        kp = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        k = 0i32;
        while !_dtkeys[k as usize].is_null() && strcmp(kp, _dtkeys[k as usize]) != 0 {
            k += 1
        }
        if _dtkeys[k as usize].is_null() {
            spc_warn(
                spe,
                b"Unrecognized dimension/transformation key: %s\x00" as *const u8 as *const i8,
                kp,
            );
            error = -1i32;
            free(kp as *mut libc::c_void);
            break;
        } else {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
            if k == 7i32 {
                (*t).flags |= 1i32 << 3i32;
                free(kp as *mut libc::c_void);
            /* not key-value */
            } else {
                if (*ap).curptr < (*ap).endptr && *(*ap).curptr.offset(0) as i32 == '=' as i32 {
                    (*ap).curptr = (*ap).curptr.offset(1);
                    skip_blank(&mut (*ap).curptr, (*ap).endptr);
                }
                vp = 0 as *mut i8;
                if *(*ap).curptr.offset(0) as i32 == '\'' as i32
                    || *(*ap).curptr.offset(0) as i32 == '\"' as i32
                {
                    let mut qchr: i8 = *(*ap).curptr.offset(0);
                    (*ap).curptr = (*ap).curptr.offset(1);
                    skip_blank(&mut (*ap).curptr, (*ap).endptr);
                    vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                    skip_blank(&mut (*ap).curptr, (*ap).endptr);
                    if !vp.is_null() && qchr as i32 != *(*ap).curptr.offset(0) as i32 {
                        spc_warn(
                            spe,
                            b"Syntax error in dimension/transformation specification.\x00"
                                as *const u8 as *const i8,
                        );
                        error = -1i32;
                        vp = mfree(vp as *mut libc::c_void) as *mut i8
                    }
                    (*ap).curptr = (*ap).curptr.offset(1)
                } else {
                    vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr)
                }
                if error == 0 && vp.is_null() {
                    spc_warn(
                        spe,
                        b"Missing value for dimension/transformation: %s\x00" as *const u8
                            as *const i8,
                        kp,
                    );
                    error = -1i32
                }
                free(kp as *mut libc::c_void);
                if vp.is_null() || error != 0 {
                    break;
                }
                match k {
                    0 => xoffset = atof(vp),
                    1 => yoffset = atof(vp),
                    2 => {
                        (*t).width = atof(vp);
                        (*t).flags |= 1i32 << 1i32
                    }
                    3 => {
                        (*t).height = atof(vp);
                        (*t).flags |= 1i32 << 2i32
                    }
                    4 => xscale = atof(vp) / 100.0f64,
                    5 => yscale = atof(vp) / 100.0f64,
                    6 => rotate = 3.14159265358979323846f64 * atof(vp) / 180.0f64,
                    8 => {
                        (*t).bbox.llx = atof(vp);
                        (*t).flags |= 1i32 << 0i32
                    }
                    9 => {
                        (*t).bbox.lly = atof(vp);
                        (*t).flags |= 1i32 << 0i32
                    }
                    10 => {
                        (*t).bbox.urx = atof(vp);
                        (*t).flags |= 1i32 << 0i32
                    }
                    11 => {
                        (*t).bbox.ury = atof(vp);
                        (*t).flags |= 1i32 << 0i32
                    }
                    12 => {
                        (*t).width = atof(vp) / 10.0f64;
                        (*t).flags |= 1i32 << 1i32
                    }
                    13 => {
                        (*t).height = atof(vp) / 10.0f64;
                        (*t).flags |= 1i32 << 2i32
                    }
                    _ => {}
                }
                skip_blank(&mut (*ap).curptr, (*ap).endptr);
                free(vp as *mut libc::c_void);
            }
        }
    }
    make_transmatrix(&mut (*t).matrix, xoffset, yoffset, xscale, yscale, rotate);
    return error;
}
/* "page" and "pagebox" are not dimension nor transformation nor
 * something acceptable to put into here.
 * PLEASE DONT ADD HERE!
 */
unsafe extern "C" fn spc_read_dimtrns_pdfm(
    mut spe: *mut spc_env,
    mut p: *mut transform_info,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut has_scale: i32 = 0; /* default: do clipping */
    let mut has_xscale: i32 = 0; /* default: do clipping */
    let mut has_yscale: i32 = 0;
    let mut has_rotate: i32 = 0;
    let mut has_matrix: i32 = 0;
    let mut _dtkeys: [*const i8; 12] = [
        b"width\x00" as *const u8 as *const i8,
        b"height\x00" as *const u8 as *const i8,
        b"depth\x00" as *const u8 as *const i8,
        b"scale\x00" as *const u8 as *const i8,
        b"xscale\x00" as *const u8 as *const i8,
        b"yscale\x00" as *const u8 as *const i8,
        b"rotate\x00" as *const u8 as *const i8,
        b"bbox\x00" as *const u8 as *const i8,
        b"matrix\x00" as *const u8 as *const i8,
        b"clip\x00" as *const u8 as *const i8,
        b"hide\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut xscale: f64 = 0.;
    let mut yscale: f64 = 0.;
    let mut rotate: f64 = 0.;
    let mut error: i32 = 0i32;
    has_matrix = 0i32;
    has_rotate = has_matrix;
    has_scale = has_rotate;
    has_yscale = has_scale;
    has_xscale = has_yscale;
    yscale = 1.0f64;
    xscale = yscale;
    rotate = 0.0f64;
    (*p).flags |= 1i32 << 3i32;
    (*p).flags &= !(1i32 << 4i32);
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let mut kp: *mut i8 = 0 as *mut i8;
        let mut vp: *mut i8 = 0 as *mut i8;
        let mut k: i32 = 0;
        kp = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        k = 0i32;
        while !_dtkeys[k as usize].is_null() && strcmp(_dtkeys[k as usize], kp) != 0 {
            k += 1
        }
        match k {
            0 => {
                error = spc_util_read_length(spe, &mut (*p).width, ap);
                (*p).flags |= 1i32 << 1i32
            }
            1 => {
                error = spc_util_read_length(spe, &mut (*p).height, ap);
                (*p).flags |= 1i32 << 2i32
            }
            2 => {
                error = spc_util_read_length(spe, &mut (*p).depth, ap);
                (*p).flags |= 1i32 << 2i32
            }
            3 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    xscale = yscale;
                    has_scale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            4 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    xscale = atof(vp);
                    has_xscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            5 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    has_yscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            6 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    rotate = 3.14159265358979323846f64 * atof(vp) / 180.0f64;
                    has_rotate = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            7 => {
                let mut v: [f64; 4] = [0.; 4];
                if spc_util_read_numbers(v.as_mut_ptr(), 4i32, ap) != 4i32 {
                    error = -1i32
                } else {
                    (*p).bbox.llx = v[0];
                    (*p).bbox.lly = v[1];
                    (*p).bbox.urx = v[2];
                    (*p).bbox.ury = v[3];
                    (*p).flags |= 1i32 << 0i32
                }
            }
            8 => {
                let mut v_0: [f64; 6] = [0.; 6];
                if spc_util_read_numbers(v_0.as_mut_ptr(), 6i32, ap) != 6i32 {
                    error = -1i32
                } else {
                    (*p).matrix.a = v_0[0];
                    (*p).matrix.b = v_0[1];
                    (*p).matrix.c = v_0[2];
                    (*p).matrix.d = v_0[3];
                    (*p).matrix.e = v_0[4];
                    (*p).matrix.f = v_0[5];
                    has_matrix = 1i32
                }
            }
            9 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    if atof(vp) != 0. {
                        (*p).flags |= 1i32 << 3i32
                    } else {
                        (*p).flags &= !(1i32 << 3i32)
                    }
                    free(vp as *mut libc::c_void);
                }
            }
            10 => (*p).flags |= 1i32 << 4i32,
            _ => error = -1i32,
        }
        if error != 0 {
            spc_warn(
                spe,
                b"Unrecognized key or invalid value for dimension/transformation: %s\x00"
                    as *const u8 as *const i8,
                kp,
            );
        } else {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
        }
        free(kp as *mut libc::c_void);
    }
    if error == 0 {
        /* Check consistency */
        if has_xscale != 0 && (*p).flags & 1i32 << 1i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both width and xscale. Ignore xscale.\x00" as *const u8
                    as *const i8,
            );
            xscale = 1.0f64
        } else if has_yscale != 0 && (*p).flags & 1i32 << 2i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both height/depth and yscale. Ignore yscale.\x00" as *const u8
                    as *const i8,
            );
            yscale = 1.0f64
        } else if has_scale != 0 && (has_xscale != 0 || has_yscale != 0) {
            spc_warn(
                spe,
                b"Can\'t supply overall scale along with axis scales.\x00" as *const u8
                    as *const i8,
            );
            error = -1i32
        } else if has_matrix != 0
            && (has_scale != 0 || has_xscale != 0 || has_yscale != 0 || has_rotate != 0)
        {
            spc_warn(spe,
                     b"Can\'t supply transform matrix along with scales or rotate. Ignore scales and rotate.\x00"
                         as *const u8 as *const i8);
        }
    }
    if has_matrix == 0 {
        make_transmatrix(&mut (*p).matrix, 0.0f64, 0.0f64, xscale, yscale, rotate);
    }
    if (*p).flags & 1i32 << 0i32 == 0 {
        (*p).flags &= !(1i32 << 3i32)
        /* no clipping needed */
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_dimtrns(
    mut spe: *mut spc_env,
    mut ti: *mut transform_info,
    mut args: *mut spc_arg,
    mut syntax: i32,
) -> i32 {
    if ti.is_null() || spe.is_null() || args.is_null() {
        return -1i32;
    }
    if syntax != 0 {
        return spc_read_dimtrns_dvips(spe, ti, args);
    } else {
        return spc_read_dimtrns_pdfm(spe, ti, args);
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
/* syntax 1: ((rgb|cmyk|hsb|gray) colorvalues)|colorname
 * syntax 0: pdf_number|pdf_array
 *
 * This is for reading *single* color specification.
 */
#[no_mangle]
pub unsafe extern "C" fn spc_util_read_blahblah(
    mut spe: *mut spc_env,
    mut p: *mut transform_info,
    mut page_no: *mut i32,
    mut bbox_type: *mut i32,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut has_scale: i32 = 0; /* default: do clipping */
    let mut has_xscale: i32 = 0; /* default: do clipping */
    let mut has_yscale: i32 = 0;
    let mut has_rotate: i32 = 0;
    let mut has_matrix: i32 = 0;
    let mut _dtkeys: [*const i8; 14] = [
        b"width\x00" as *const u8 as *const i8,
        b"height\x00" as *const u8 as *const i8,
        b"depth\x00" as *const u8 as *const i8,
        b"scale\x00" as *const u8 as *const i8,
        b"xscale\x00" as *const u8 as *const i8,
        b"yscale\x00" as *const u8 as *const i8,
        b"rotate\x00" as *const u8 as *const i8,
        b"bbox\x00" as *const u8 as *const i8,
        b"matrix\x00" as *const u8 as *const i8,
        b"clip\x00" as *const u8 as *const i8,
        b"hide\x00" as *const u8 as *const i8,
        b"page\x00" as *const u8 as *const i8,
        b"pagebox\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut xscale: f64 = 0.;
    let mut yscale: f64 = 0.;
    let mut rotate: f64 = 0.;
    let mut error: i32 = 0i32;
    has_matrix = 0i32;
    has_rotate = has_matrix;
    has_scale = has_rotate;
    has_yscale = has_scale;
    has_xscale = has_yscale;
    yscale = 1.0f64;
    xscale = yscale;
    rotate = 0.0f64;
    (*p).flags |= 1i32 << 3i32;
    (*p).flags &= !(1i32 << 4i32);
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let mut kp: *mut i8 = 0 as *mut i8;
        let mut vp: *mut i8 = 0 as *mut i8;
        let mut k: i32 = 0;
        kp = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        k = 0i32;
        while !_dtkeys[k as usize].is_null() && strcmp(_dtkeys[k as usize], kp) != 0 {
            k += 1
        }
        match k {
            0 => {
                error = spc_util_read_length(spe, &mut (*p).width, ap);
                (*p).flags |= 1i32 << 1i32
            }
            1 => {
                error = spc_util_read_length(spe, &mut (*p).height, ap);
                (*p).flags |= 1i32 << 2i32
            }
            2 => {
                error = spc_util_read_length(spe, &mut (*p).depth, ap);
                (*p).flags |= 1i32 << 2i32
            }
            3 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    xscale = yscale;
                    has_scale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            4 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    xscale = atof(vp);
                    has_xscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            5 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    yscale = atof(vp);
                    has_yscale = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            6 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    rotate = 3.14159265358979323846f64 * atof(vp) / 180.0f64;
                    has_rotate = 1i32;
                    free(vp as *mut libc::c_void);
                }
            }
            7 => {
                let mut v: [f64; 4] = [0.; 4];
                if spc_util_read_numbers(v.as_mut_ptr(), 4i32, ap) != 4i32 {
                    error = -1i32
                } else {
                    (*p).bbox.llx = v[0];
                    (*p).bbox.lly = v[1];
                    (*p).bbox.urx = v[2];
                    (*p).bbox.ury = v[3];
                    (*p).flags |= 1i32 << 0i32
                }
            }
            8 => {
                let mut v_0: [f64; 6] = [0.; 6];
                if spc_util_read_numbers(v_0.as_mut_ptr(), 6i32, ap) != 6i32 {
                    error = -1i32
                } else {
                    (*p).matrix.a = v_0[0];
                    (*p).matrix.b = v_0[1];
                    (*p).matrix.c = v_0[2];
                    (*p).matrix.d = v_0[3];
                    (*p).matrix.e = v_0[4];
                    (*p).matrix.f = v_0[5];
                    has_matrix = 1i32
                }
            }
            9 => {
                vp = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    if atof(vp) != 0. {
                        (*p).flags |= 1i32 << 3i32
                    } else {
                        (*p).flags &= !(1i32 << 3i32)
                    }
                    free(vp as *mut libc::c_void);
                }
            }
            11 => {
                let mut page: f64 = 0.;
                if !page_no.is_null() && spc_util_read_numbers(&mut page, 1i32, ap) == 1i32 {
                    *page_no = page as i32
                } else {
                    error = -1i32
                }
            }
            10 => (*p).flags |= 1i32 << 4i32,
            12 => {
                let mut q: *mut i8 = 0 as *mut i8;
                q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
                if !q.is_null() {
                    if !bbox_type.is_null() {
                        if strcasecmp(q, b"cropbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 1i32
                        } else if strcasecmp(q, b"mediabox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 2i32
                        } else if strcasecmp(q, b"artbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 3i32
                        } else if strcasecmp(q, b"trimbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 4i32
                        } else if strcasecmp(q, b"bleedbox\x00" as *const u8 as *const i8) == 0i32 {
                            *bbox_type = 5i32
                        }
                    }
                    free(q as *mut libc::c_void);
                } else if !bbox_type.is_null() {
                    *bbox_type = 0i32
                }
            }
            _ => error = -1i32,
        }
        if error != 0 {
            spc_warn(
                spe,
                b"Unrecognized key or invalid value for dimension/transformation: %s\x00"
                    as *const u8 as *const i8,
                kp,
            );
        } else {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
        }
        free(kp as *mut libc::c_void);
    }
    if error == 0 {
        /* Check consistency */
        if has_xscale != 0 && (*p).flags & 1i32 << 1i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both width and xscale. Ignore xscale.\x00" as *const u8
                    as *const i8,
            );
            xscale = 1.0f64
        } else if has_yscale != 0 && (*p).flags & 1i32 << 2i32 != 0 {
            spc_warn(
                spe,
                b"Can\'t supply both height/depth and yscale. Ignore yscale.\x00" as *const u8
                    as *const i8,
            );
            yscale = 1.0f64
        } else if has_scale != 0 && (has_xscale != 0 || has_yscale != 0) {
            spc_warn(
                spe,
                b"Can\'t supply overall scale along with axis scales.\x00" as *const u8
                    as *const i8,
            );
            error = -1i32
        } else if has_matrix != 0
            && (has_scale != 0 || has_xscale != 0 || has_yscale != 0 || has_rotate != 0)
        {
            spc_warn(spe,
                     b"Can\'t supply transform matrix along with scales or rotate. Ignore scales and rotate.\x00"
                         as *const u8 as *const i8);
        }
    }
    if has_matrix == 0 {
        make_transmatrix(&mut (*p).matrix, 0.0f64, 0.0f64, xscale, yscale, rotate);
    }
    if (*p).flags & 1i32 << 0i32 == 0 {
        (*p).flags &= !(1i32 << 3i32)
        /* no clipping needed */
    }
    return error;
}
static mut colordefs: [colordef_; 69] = [
    {
        let mut init = colordef_ {
            key: b"GreenYellow\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.15f64, 0.00f64, 0.69f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Yellow\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Goldenrod\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.10f64, 0.84f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Dandelion\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.29f64, 0.84f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Apricot\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.32f64, 0.52f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Peach\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.50f64, 0.70f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Melon\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.46f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"YellowOrange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.42f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Orange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.61f64, 0.87f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BurntOrange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.51f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Bittersweet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.75f64, 1.00f64, 0.24f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RedOrange\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.77f64, 0.87f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Mahogany\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.85f64, 0.87f64, 0.35f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Maroon\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.87f64, 0.68f64, 0.32f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BrickRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.89f64, 0.94f64, 0.28f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Red\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 1.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"OrangeRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 1.00f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RubineRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 1.00f64, 0.13f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"WildStrawberry\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.96f64, 0.39f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Salmon\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.53f64, 0.38f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"CarnationPink\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.63f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Magenta\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 1.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"VioletRed\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.81f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Rhodamine\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.82f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Mulberry\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.34f64, 0.90f64, 0.00f64, 0.02f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RedViolet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.07f64, 0.90f64, 0.00f64, 0.34f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Fuchsia\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.47f64, 0.91f64, 0.00f64, 0.08f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Lavender\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.48f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Thistle\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.12f64, 0.59f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Orchid\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.32f64, 0.64f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"DarkOrchid\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.40f64, 0.80f64, 0.20f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Purple\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.45f64, 0.86f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Plum\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.50f64, 1.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Violet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.79f64, 0.88f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RoyalPurple\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.75f64, 0.90f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BlueViolet\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.86f64, 0.91f64, 0.00f64, 0.04f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Periwinkle\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.57f64, 0.55f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"CadetBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.62f64, 0.57f64, 0.23f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"CornflowerBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.65f64, 0.13f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"MidnightBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.98f64, 0.13f64, 0.00f64, 0.43f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"NavyBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.94f64, 0.54f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RoyalBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [1.00f64, 0.50f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Blue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [1.00f64, 1.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Cerulean\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.94f64, 0.11f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Cyan\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [1.00f64, 0.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"ProcessBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.96f64, 0.00f64, 0.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"SkyBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.62f64, 0.00f64, 0.12f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Turquoise\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.85f64, 0.00f64, 0.20f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"TealBlue\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.86f64, 0.00f64, 0.34f64, 0.02f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Aquamarine\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.82f64, 0.00f64, 0.30f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"BlueGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.85f64, 0.00f64, 0.33f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Emerald\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [1.00f64, 0.00f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"JungleGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.99f64, 0.00f64, 0.52f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"SeaGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.69f64, 0.00f64, 0.50f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Green\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [1.00f64, 0.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"ForestGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.91f64, 0.00f64, 0.88f64, 0.12f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"PineGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.92f64, 0.00f64, 0.59f64, 0.25f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"LimeGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.50f64, 0.00f64, 1.00f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"YellowGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.44f64, 0.00f64, 0.74f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"SpringGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.26f64, 0.00f64, 0.76f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"OliveGreen\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.64f64, 0.00f64, 0.95f64, 0.40f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"RawSienna\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.72f64, 1.00f64, 0.45f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Sepia\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.83f64, 1.00f64, 0.70f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Brown\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.00f64, 0.81f64, 1.00f64, 0.60f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Tan\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 4i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.14f64, 0.42f64, 0.56f64, 0.00f64],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Gray\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 1i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.5f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"Black\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 1i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.0f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: b"White\x00" as *const u8 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 1i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [1.0f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
    {
        let mut init = colordef_ {
            key: 0 as *const i8,
            color: {
                let mut init = pdf_color {
                    num_components: 0i32,
                    spot_color_name: 0 as *const i8 as *mut i8,
                    values: [0.0f64, 0., 0., 0.],
                };
                init
            },
        };
        init
    },
];
/* From pdfcolor.c */
unsafe extern "C" fn pdf_color_namedcolor(mut color: *mut pdf_color, mut name: *const i8) -> i32 {
    let mut i: i32 = 0;
    i = 0i32;
    while !colordefs[i as usize].key.is_null() {
        if streq_ptr(colordefs[i as usize].key, name) {
            pdf_color_copycolor(
                color,
                &mut (*colordefs.as_mut_ptr().offset(i as isize)).color,
            );
            return 0i32;
        }
        i += 1
    }
    return -1i32;
}
