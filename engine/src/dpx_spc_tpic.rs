#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::streq_ptr;
use crate::warn;

use super::dpx_pdfcolor::{pdf_color_brighten_color, pdf_color_get_current};
use super::dpx_pdfdraw::{pdf_dev_concat, pdf_dev_set_color};
use crate::dpx_pdfobj::{
    pdf_add_dict, pdf_lookup_dict, pdf_name_value, pdf_new_boolean, pdf_new_dict, pdf_new_name,
    pdf_new_number, pdf_new_string, pdf_obj, pdf_obj_typeof, pdf_ref_obj, pdf_release_obj,
    pdf_string_value,
};
use crate::dpx_pdfparse::parse_val_ident;
use libc::free;
extern "C" {
    #[no_mangle]
    fn pdf_foreach_dict(
        dict: *mut pdf_obj,
        proc_0: Option<
            unsafe extern "C" fn(_: *mut pdf_obj, _: *mut pdf_obj, _: *mut libc::c_void) -> i32,
        >,
        pdata: *mut libc::c_void,
    ) -> i32;
    #[no_mangle]
    fn pdf_get_version() -> u32;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn parse_float_decimal(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_string(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
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
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    /* The following two routines are NOT WORKING.
     * Dvipdfmx doesn't manage gstate well..
     */
    /* Always returns 1.0, please rename this. */
    #[no_mangle]
    fn pdf_dev_scale() -> f64;
    #[no_mangle]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const i8, length: u32);
    #[no_mangle]
    fn pdf_doc_add_page_resource(
        category: *const i8,
        resource_name: *const i8,
        resources: *mut pdf_obj,
    );
    #[no_mangle]
    fn pdf_dev_setlinewidth(width: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_setmiterlimit(mlimit: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_setlinecap(style: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_setlinejoin(style: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_setdash(count: i32, pattern: *mut f64, offset: f64) -> i32;
    /* Path Construction */
    #[no_mangle]
    fn pdf_dev_moveto(x: f64, y: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_lineto(x0: f64, y0: f64) -> i32;
    #[no_mangle]
    fn pdf_dev_newpath() -> i32;
    #[no_mangle]
    fn pdf_dev_flushpath(p_op: i8, fill_rule: i32) -> i32;
    #[no_mangle]
    fn pdf_dev_gsave() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore() -> i32;
    /* extension */
    #[no_mangle]
    fn pdf_dev_arcx(
        c_x: f64,
        c_y: f64,
        r_x: f64,
        r_y: f64,
        a_0: f64,
        a_1: f64,
        a_d: i32,
        xar: f64,
    ) -> i32;
    #[no_mangle]
    fn pdf_dev_bspline(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> i32;
}
pub type size_t = u64;

use super::dpx_specials::{spc_arg, spc_env};

pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const i8,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fill: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_tpic_ {
    pub mode: C2RustUnnamed_0,
    pub pen_size: f64,
    pub fill_shape: bool,
    pub fill_color: f64,
    pub points: *mut pdf_coord,
    pub num_points: i32,
    pub max_points: i32,
}

use super::dpx_pdfdev::pdf_coord;

pub use super::dpx_pdfcolor::pdf_color;

use super::dpx_pdfdev::pdf_tmatrix;

/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    0 as *mut libc::c_void
}
unsafe extern "C" fn skip_blank(mut pp: *mut *const i8, mut endptr: *const i8) {
    let mut p: *const i8 = *pp;
    while p < endptr && (*p as i32 & !0x7fi32 == 0i32 && crate::isblank(*p as _) != 0) {
        p = p.offset(1)
    }
    *pp = p;
}
static mut _tpic_state: spc_tpic_ = spc_tpic_ {
    mode: C2RustUnnamed_0 { fill: 0 },
    pen_size: 0.,
    fill_shape: false,
    fill_color: 0.,
    points: 0 as *const pdf_coord as *mut pdf_coord,
    num_points: 0,
    max_points: 0,
};
/* We use pdf_doc_add_page_content() here
 * since we always draw isolated graphics.
 */
unsafe extern "C" fn tpic__clear(mut tp: *mut spc_tpic_) {
    (*tp).points = mfree((*tp).points as *mut libc::c_void) as *mut pdf_coord;
    (*tp).num_points = 0i32;
    (*tp).max_points = 0i32;
    (*tp).fill_shape = false;
    (*tp).fill_color = 0.0f64;
}
unsafe extern "C" fn create_xgstate(mut a: f64, mut f_ais: i32) -> *mut pdf_obj
/* alpha is shape */ {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj; /* dash pattern */
    dict = pdf_new_dict();
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"ExtGState\x00" as *const u8 as *const i8),
    );
    if f_ais != 0 {
        pdf_add_dict(
            dict,
            pdf_new_name(b"AIS\x00" as *const u8 as *const i8),
            pdf_new_boolean(1_i8),
        );
    }
    pdf_add_dict(
        dict,
        pdf_new_name(b"ca\x00" as *const u8 as *const i8),
        pdf_new_number(a),
    );
    dict
}
unsafe extern "C" fn check_resourcestatus(mut category: *const i8, mut resname: *const i8) -> i32 {
    let mut dict1: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict2: *mut pdf_obj = 0 as *mut pdf_obj;
    dict1 = pdf_doc_current_page_resources();
    if dict1.is_null() {
        return 0i32;
    }
    dict2 = pdf_lookup_dict(dict1, category);
    if !dict2.is_null() && pdf_obj_typeof(dict2) == 6i32 {
        if !pdf_lookup_dict(dict2, resname).is_null() {
            return 1i32;
        }
    }
    0i32
}
unsafe extern "C" fn set_linestyle(mut pn: f64, mut da: f64) -> i32 {
    let mut dp: [f64; 2] = [0.; 2];
    pdf_dev_setlinejoin(1i32);
    pdf_dev_setmiterlimit(1.4f64);
    pdf_dev_setlinewidth(pn);
    if da > 0.0f64 {
        dp[0] = da * 72.0f64;
        pdf_dev_setdash(1i32, dp.as_mut_ptr(), 0i32 as f64);
        pdf_dev_setlinecap(0i32);
    } else if da < 0.0f64 {
        dp[0] = pn;
        dp[1] = -da * 72.0f64;
        pdf_dev_setdash(2i32, dp.as_mut_ptr(), 0i32 as f64);
        pdf_dev_setlinecap(1i32);
    } else {
        pdf_dev_setlinecap(0i32);
    }
    0i32
}
unsafe extern "C" fn set_fillstyle(mut g: f64, mut a: f64, mut f_ais: i32) -> i32 {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut resname: [i8; 32] = [0; 32];
    let mut buf: [i8; 38] = [0; 38];
    let mut alp: i32 = 0;
    let mut len: i32 = 0i32;
    if a > 0.0f64 {
        alp = (100.0f64 * a).round() as i32;
        sprintf(
            resname.as_mut_ptr(),
            b"_Tps_a%03d_\x00" as *const u8 as *const i8,
            alp,
        );
        if check_resourcestatus(
            b"ExtGState\x00" as *const u8 as *const i8,
            resname.as_mut_ptr(),
        ) == 0
        {
            dict = create_xgstate(
                (0.01f64 * alp as f64 / 0.01f64 + 0.5f64).floor() * 0.01f64,
                f_ais,
            );
            pdf_doc_add_page_resource(
                b"ExtGState\x00" as *const u8 as *const i8,
                resname.as_mut_ptr(),
                pdf_ref_obj(dict),
            );
            pdf_release_obj(dict);
        }
        len += sprintf(
            buf.as_mut_ptr().offset(len as isize),
            b" /%s gs\x00" as *const u8 as *const i8,
            resname.as_mut_ptr(),
        );
        pdf_doc_add_page_content(buf.as_mut_ptr(), len as u32);
        /* op: gs */
    } /* get stroking and fill colors */
    let mut new_fc: pdf_color = pdf_color {
        num_components: 0,
        spot_color_name: 0 as *mut i8,
        values: [0.; 4],
    };
    let (sc, fc) = pdf_color_get_current();
    pdf_color_brighten_color(&mut new_fc, fc, g);
    pdf_dev_set_color(&mut new_fc, 0x20_i8, 0i32);
    0i32
}
unsafe extern "C" fn set_styles(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_fs: bool,
    mut f_vp: bool,
    mut pn: f64,
    mut da: f64,
) {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.a = 1.0f64;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = -1.0f64;
    M.e = (*c).x;
    M.f = (*c).y;
    pdf_dev_concat(&mut M);
    if f_vp {
        set_linestyle(pn, da);
    }
    if f_fs {
        let mut g: f64 = 0.;
        let mut a: f64 = 0.;
        let mut f_ais: i32 = 0;
        if (*tp).mode.fill == 0i32 || (*tp).fill_color == 0. {
            g = 1.0f64 - (*tp).fill_color;
            a = 0.0f64
        } else {
            g = 0.0f64;
            a = (*tp).fill_color
        }
        f_ais = if (*tp).mode.fill == 2i32 { 1i32 } else { 0i32 };
        set_fillstyle(g, a, f_ais);
    };
}
unsafe extern "C" fn showpath(mut f_vp: bool, mut f_fs: bool)
/* visible_path, fill_shape */
{
    if f_vp {
        if f_fs {
            pdf_dev_flushpath('b' as i32 as i8, 0i32);
        } else {
            pdf_dev_flushpath('S' as i32 as i8, 0i32);
        }
    } else if f_fs {
        pdf_dev_flushpath('f' as i32 as i8, 0i32);
    } else {
        pdf_dev_newpath();
    };
}
unsafe extern "C" fn tpic__polyline(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_vp: bool,
    mut da: f64,
) -> i32 {
    let mut pn: f64 = (*tp).pen_size;
    let mut f_fs: bool = (*tp).fill_shape;
    let mut i: i32 = 0;
    let mut error: i32 = 0i32;
    /*
     * Acrobat claims 'Q' as illegal operation when there are unfinished
     * path (a path without path-painting operator applied)?
     */
    /* Shading is applied only to closed path. */
    f_fs = if (*(*tp).points.offset(0)).x
        == (*(*tp).points.offset(((*tp).num_points - 1i32) as isize)).x
        && (*(*tp).points.offset(0)).y
            == (*(*tp).points.offset(((*tp).num_points - 1i32) as isize)).y
    {
        f_fs as i32
    } else {
        0i32
    } != 0;
    f_vp = if pn > 0.0f64 { f_vp as i32 } else { 0i32 } != 0;
    if f_vp as i32 != 0 || f_fs as i32 != 0 {
        pdf_dev_gsave();
        set_styles(tp, c, f_fs, f_vp, pn, da);
        pdf_dev_moveto((*(*tp).points.offset(0)).x, (*(*tp).points.offset(0)).y);
        i = 0i32;
        while i < (*tp).num_points {
            pdf_dev_lineto(
                (*(*tp).points.offset(i as isize)).x,
                (*(*tp).points.offset(i as isize)).y,
            );
            i += 1
        }
        showpath(f_vp, f_fs);
        pdf_dev_grestore();
    }
    tpic__clear(tp);
    error
}
/*
 * Accroding to
 * "Tpic: Pic for TEX", Tim Morgan, Original by Brian Kernighan, p.20:
 *
 *  A spline is a smooth curve guided by a set of straight lines just
 *  like the line above. It begins at the same place, ends at the same
 *  place, and in between is tangent to the mid-point of each guiding
 *  line. The syntax for a spline is identical to a (path) line except
 *  for using spline instead of line.
 *
 * Spline is not a curve drawn by spline-fitting points p0, p1, ..., pn,
 * defined by tpic special "pa" command. Instead, a path defined by set
 * of points p0, p1, ... is guiding line mentioned above.
 *
 * Dvipsk draws them as a straight line from p0 to q1 = (p0 + p1)/2,
 * followed by a quadratic B-spline curve with starting point q1, (off-
 * curve) control point p1, end point q2 = (p1 + p2)/2, ..., and a
 * straight line from qn to pn.
 */
unsafe extern "C" fn tpic__spline(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_vp: bool,
    mut da: f64,
) -> i32 {
    let mut v: [f64; 6] = [0.; 6];
    let mut pn: f64 = (*tp).pen_size;
    let mut f_fs: bool = (*tp).fill_shape;
    let mut i: i32 = 0;
    let mut error: i32 = 0i32;
    f_fs = if (*(*tp).points.offset(0)).x
        == (*(*tp).points.offset(((*tp).num_points - 1i32) as isize)).x
        && (*(*tp).points.offset(0)).y
            == (*(*tp).points.offset(((*tp).num_points - 1i32) as isize)).y
    {
        f_fs as i32
    } else {
        0i32
    } != 0;
    f_vp = if pn > 0.0f64 { f_vp as i32 } else { 0i32 } != 0;
    if f_vp as i32 != 0 || f_fs as i32 != 0 {
        pdf_dev_gsave();
        set_styles(tp, c, f_fs, f_vp, pn, da);
        pdf_dev_moveto((*(*tp).points.offset(0)).x, (*(*tp).points.offset(0)).y);
        v[0] = 0.5f64 * ((*(*tp).points.offset(0)).x + (*(*tp).points.offset(1)).x);
        v[1] = 0.5f64 * ((*(*tp).points.offset(0)).y + (*(*tp).points.offset(1)).y);
        pdf_dev_lineto(v[0], v[1]);
        i = 1i32;
        while i < (*tp).num_points - 1i32 {
            /* B-spline control points */
            v[0] = 0.5f64
                * ((*(*tp).points.offset((i - 1i32) as isize)).x
                    + (*(*tp).points.offset(i as isize)).x);
            v[1] = 0.5f64
                * ((*(*tp).points.offset((i - 1i32) as isize)).y
                    + (*(*tp).points.offset(i as isize)).y);
            v[2] = (*(*tp).points.offset(i as isize)).x;
            v[3] = (*(*tp).points.offset(i as isize)).y;
            v[4] = 0.5f64
                * ((*(*tp).points.offset(i as isize)).x
                    + (*(*tp).points.offset((i + 1i32) as isize)).x);
            v[5] = 0.5f64
                * ((*(*tp).points.offset(i as isize)).y
                    + (*(*tp).points.offset((i + 1i32) as isize)).y);
            pdf_dev_bspline(v[0], v[1], v[2], v[3], v[4], v[5]);
            i += 1
        }
        pdf_dev_lineto(
            (*(*tp).points.offset(i as isize)).x,
            (*(*tp).points.offset(i as isize)).y,
        );
        showpath(f_vp, f_fs);
        pdf_dev_grestore();
    }
    tpic__clear(tp);
    error
}
unsafe extern "C" fn tpic__arc(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_vp: bool,
    mut da: f64,
    mut v: *mut f64,
) -> i32
/* 6 numbers */ {
    let mut pn: f64 = (*tp).pen_size;
    let mut f_fs: bool = (*tp).fill_shape;
    f_fs = if ((*v.offset(4) - *v.offset(5)).abs() + 0.5f64).round() >= 360i32 as f64 {
        f_fs as i32
    } else {
        0i32
    } != 0;
    f_vp = if pn > 0.0f64 { f_vp as i32 } else { 0i32 } != 0;
    if f_vp as i32 != 0 || f_fs as i32 != 0 {
        pdf_dev_gsave();
        set_styles(tp, c, f_fs, f_vp, pn, da);
        /* The arcx operator here draws an excess straight line from current
         * point to the starting point of the arc if they are different, as in
         * PostScript language. It may cuase an unexpected behavior when DVIPS
         * transformation command is inserted before TPIC ar command: it invokes
         * moveto and sets currentpoint which may be different from the starting
         * point of arc to be drawn. We use newpath here to avoid drawing an
         * excess line. I'm not sure if it is proper TPIC implementation but this
         * seems to be DVIPS compatible behavior.
         */
        pdf_dev_newpath();
        pdf_dev_arcx(
            *v.offset(0),
            *v.offset(1),
            *v.offset(2),
            *v.offset(3),
            *v.offset(4),
            *v.offset(5),
            1i32,
            0.0f64,
        );
        showpath(f_vp, f_fs);
        pdf_dev_grestore();
    }
    tpic__clear(tp);
    0i32
}
unsafe extern "C" fn spc_currentpoint(
    mut spe: *mut spc_env,
    mut pg: *mut i32,
    mut cp: *mut pdf_coord,
) -> i32 {
    *pg = 0i32;
    (*cp).x = (*spe).x_user;
    (*cp).y = (*spe).y_user;
    0i32
}
unsafe extern "C" fn spc_handler_tpic_pn(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut q: *mut i8 = 0 as *mut i8;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        spc_warn(
            spe,
            b"Invalid pen size specified?\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*tp).pen_size = atof(q) * (0.072f64 / pdf_dev_scale());
    free(q as *mut libc::c_void);
    0i32
}
unsafe extern "C" fn spc_handler_tpic_pa(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut v: [f64; 2] = [0.; 2];
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    i = 0i32;
    while i < 2i32 && (*ap).curptr < (*ap).endptr {
        q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
        if q.is_null() {
            spc_warn(
                spe,
                b"Missing numbers for TPIC \"pa\" command.\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
        v[i as usize] = atof(q);
        free(q as *mut libc::c_void);
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        i += 1
    }
    if i != 2i32 {
        spc_warn(
            spe,
            b"Invalid arg for TPIC \"pa\" command.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    if (*tp).num_points >= (*tp).max_points {
        (*tp).max_points += 256i32;
        (*tp).points = renew(
            (*tp).points as *mut libc::c_void,
            ((*tp).max_points as u32 as u64).wrapping_mul(::std::mem::size_of::<pdf_coord>() as u64)
                as u32,
        ) as *mut pdf_coord
    }
    (*(*tp).points.offset((*tp).num_points as isize)).x = v[0] * (0.072f64 / pdf_dev_scale());
    (*(*tp).points.offset((*tp).num_points as isize)).y = v[1] * (0.072f64 / pdf_dev_scale());
    (*tp).num_points += 1i32;
    0i32
}
unsafe extern "C" fn spc_handler_tpic_fp(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    if (*tp).num_points <= 1i32 {
        spc_warn(
            spe,
            b"Too few points (< 2) for polyline path.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__polyline(tp, &mut cp, true, 0.0f64)
}
unsafe extern "C" fn spc_handler_tpic_ip(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    if (*tp).num_points <= 1i32 {
        spc_warn(
            spe,
            b"Too few points (< 2) for polyline path.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__polyline(tp, &mut cp, false, 0.0f64)
}
unsafe extern "C" fn spc_handler_tpic_da(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut da: f64 = 0.0f64;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        da = atof(q);
        free(q as *mut libc::c_void);
    }
    if (*tp).num_points <= 1i32 {
        spc_warn(
            spe,
            b"Too few points (< 2) for polyline path.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__polyline(tp, &mut cp, true, da)
}
unsafe extern "C" fn spc_handler_tpic_dt(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut da: f64 = 0.0f64;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        da = -atof(q);
        free(q as *mut libc::c_void);
    }
    if (*tp).num_points <= 1i32 {
        spc_warn(
            spe,
            b"Too few points (< 2) for polyline path.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__polyline(tp, &mut cp, true, da)
}
unsafe extern "C" fn spc_handler_tpic_sp(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut da: f64 = 0.0f64;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        da = atof(q);
        free(q as *mut libc::c_void);
    }
    if (*tp).num_points <= 2i32 {
        spc_warn(
            spe,
            b"Too few points (< 3) for spline path.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__spline(tp, &mut cp, true, da)
}
unsafe extern "C" fn spc_handler_tpic_ar(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut v: [f64; 6] = [0.; 6];
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    i = 0i32;
    while i < 6i32 && (*ap).curptr < (*ap).endptr {
        q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
        if q.is_null() {
            spc_warn(
                spe,
                b"Invalid args. in TPIC \"ar\" command.\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
        v[i as usize] = atof(q);
        free(q as *mut libc::c_void);
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        i += 1
    }
    if i != 6i32 {
        spc_warn(
            spe,
            b"Invalid arg for TPIC \"ar\" command.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    v[0] *= 0.072f64 / pdf_dev_scale();
    v[1] *= 0.072f64 / pdf_dev_scale();
    v[2] *= 0.072f64 / pdf_dev_scale();
    v[3] *= 0.072f64 / pdf_dev_scale();
    v[4] *= 180.0f64 / 3.14159265358979323846f64;
    v[5] *= 180.0f64 / 3.14159265358979323846f64;
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__arc(tp, &mut cp, true, 0.0f64, v.as_mut_ptr())
}
unsafe extern "C" fn spc_handler_tpic_ia(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut v: [f64; 6] = [0.; 6];
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: i32 = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    i = 0i32;
    while i < 6i32 && (*ap).curptr < (*ap).endptr {
        q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
        if q.is_null() {
            spc_warn(
                spe,
                b"Invalid args. in TPIC \"ia\" command.\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
        v[i as usize] = atof(q);
        free(q as *mut libc::c_void);
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        i += 1
    }
    if i != 6i32 {
        spc_warn(
            spe,
            b"Invalid arg for TPIC \"ia\" command.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    v[0] *= 0.072f64 / pdf_dev_scale();
    v[1] *= 0.072f64 / pdf_dev_scale();
    v[2] *= 0.072f64 / pdf_dev_scale();
    v[3] *= 0.072f64 / pdf_dev_scale();
    v[4] *= 180.0f64 / 3.14159265358979323846f64;
    v[5] *= 180.0f64 / 3.14159265358979323846f64;
    spc_currentpoint(spe, &mut pg, &mut cp);
    tpic__arc(tp, &mut cp, false, 0.0f64, v.as_mut_ptr())
}
unsafe extern "C" fn spc_handler_tpic_sh(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut q: *mut i8 = 0 as *mut i8;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    (*tp).fill_shape = true;
    (*tp).fill_color = 0.5f64;
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        let mut g: f64 = atof(q);
        free(q as *mut libc::c_void);
        if g >= 0.0f64 && g <= 1.0f64 {
            (*tp).fill_color = g
        } else {
            warn!("Invalid fill color specified: {}\n", g);
            return -1i32;
        }
    }
    0i32
}
unsafe extern "C" fn spc_handler_tpic_wh(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    (*tp).fill_shape = true;
    (*tp).fill_color = 0.0f64;
    0i32
}
unsafe extern "C" fn spc_handler_tpic_bk(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    (*tp).fill_shape = true;
    (*tp).fill_color = 1.0f64;
    0i32
}
unsafe extern "C" fn spc_handler_tpic_tx(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32
/* , void *dp) */ {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state; /* NULL terminate */
    assert!(!spe.is_null() && !ap.is_null() && !tp.is_null());
    spc_warn(
        spe,
        b"TPIC command \"tx\" not supported.\x00" as *const u8 as *const i8,
    );
    -1i32
}
unsafe extern "C" fn spc_handler_tpic__init(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    (*tp).pen_size = 1.0f64;
    (*tp).fill_shape = false;
    (*tp).fill_color = 0.0f64;
    (*tp).points = 0 as *mut pdf_coord;
    (*tp).num_points = 0i32;
    (*tp).max_points = 0i32;
    if (*tp).mode.fill != 0i32 && pdf_get_version() < 4_u32 {
        spc_warn(
            spe,
            b"Tpic shading support requires PDF version 1.4.\x00" as *const u8 as *const i8,
        );
        (*tp).mode.fill = 0i32
    }
    0i32
}
unsafe extern "C" fn spc_handler_tpic__bophook(mut dp: *mut libc::c_void) -> i32 {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    assert!(!tp.is_null());
    tpic__clear(tp);
    0i32
}
unsafe extern "C" fn spc_handler_tpic__eophook(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    assert!(!tp.is_null());
    if (*tp).num_points > 0i32 {
        spc_warn(
            spe,
            b"Unflushed tpic path at end of the page.\x00" as *const u8 as *const i8,
        );
    }
    tpic__clear(tp);
    0i32
}
unsafe extern "C" fn spc_handler_tpic__clean(
    mut spe: *mut spc_env,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    assert!(!tp.is_null());
    if (*tp).num_points > 0i32 {
        spc_warn(
            spe,
            b"Unflushed tpic path at end of the document.\x00" as *const u8 as *const i8,
        );
    }
    tpic__clear(tp);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn tpic_set_fill_mode(mut mode: i32) {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    (*tp).mode.fill = mode;
}
#[no_mangle]
pub unsafe extern "C" fn spc_tpic_at_begin_page() -> i32 {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    spc_handler_tpic__bophook(tp as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_tpic_at_end_page() -> i32 {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    spc_handler_tpic__eophook(0 as *mut spc_env, tp as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_tpic_at_begin_document() -> i32 {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    spc_handler_tpic__init(0 as *mut spc_env, tp as *mut libc::c_void)
}
#[no_mangle]
pub unsafe extern "C" fn spc_tpic_at_end_document() -> i32 {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    spc_handler_tpic__clean(0 as *mut spc_env, tp as *mut libc::c_void)
}
unsafe extern "C" fn spc_parse_kvpairs(mut ap: *mut spc_arg) -> *mut pdf_obj {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut kp: *mut i8 = 0 as *mut i8;
    let mut vp: *mut i8 = 0 as *mut i8;
    let mut error: i32 = 0i32;
    dict = pdf_new_dict();
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        kp = parse_val_ident(&mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        if (*ap).curptr < (*ap).endptr && *(*ap).curptr.offset(0) as i32 == '=' as i32 {
            (*ap).curptr = (*ap).curptr.offset(1);
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
            if (*ap).curptr == (*ap).endptr {
                free(kp as *mut libc::c_void);
                error = -1i32;
                break;
            } else {
                vp = parse_c_string(&mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -1i32
                } else {
                    pdf_add_dict(
                        dict,
                        pdf_new_name(kp),
                        pdf_new_string(
                            vp as *const libc::c_void,
                            strlen(vp).wrapping_add(1i32 as u64),
                        ),
                    );
                    free(vp as *mut libc::c_void);
                }
            }
        } else {
            /* Treated as 'flag' */
            pdf_add_dict(dict, pdf_new_name(kp), pdf_new_boolean(1_i8));
        }
        free(kp as *mut libc::c_void);
        if error == 0 {
            skip_blank(&mut (*ap).curptr, (*ap).endptr);
        }
    }
    if error != 0 {
        pdf_release_obj(dict);
        dict = 0 as *mut pdf_obj
    }
    dict
}
unsafe extern "C" fn tpic_filter_getopts(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut libc::c_void,
) -> i32 {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    let mut k: *mut i8 = 0 as *mut i8;
    let mut v: *mut i8 = 0 as *mut i8;
    let mut error: i32 = 0i32;
    assert!(!kp.is_null() && !vp.is_null() && !tp.is_null());
    k = pdf_name_value(kp);
    if streq_ptr(k, b"fill-mode\x00" as *const u8 as *const i8) {
        if pdf_obj_typeof(vp) != 3i32 {
            warn!("Invalid value for TPIC option fill-mode...");
            error = -1i32
        } else {
            v = pdf_string_value(vp) as *mut i8;
            if streq_ptr(v, b"shape\x00" as *const u8 as *const i8) {
                (*tp).mode.fill = 2i32
            } else if streq_ptr(v, b"opacity\x00" as *const u8 as *const i8) {
                (*tp).mode.fill = 1i32
            } else if streq_ptr(v, b"solid\x00" as *const u8 as *const i8) {
                (*tp).mode.fill = 0i32
            } else {
                dpx_warning(
                    b"Invalid value for TPIC option fill-mode: %s\x00" as *const u8 as *const i8,
                    v,
                );
                error = -1i32
            }
        }
    } else {
        dpx_warning(
            b"Unrecognized option for TPIC special handler: %s\x00" as *const u8 as *const i8,
            k,
        );
        error = -1i32
    }
    error
}
unsafe extern "C" fn spc_handler_tpic__setopts(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32 {
    let mut tp: *mut spc_tpic_ = &mut _tpic_state;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: i32 = 0i32;
    dict = spc_parse_kvpairs(ap);
    if dict.is_null() {
        return -1i32;
    }
    error = pdf_foreach_dict(
        dict,
        Some(
            tpic_filter_getopts
                as unsafe extern "C" fn(
                    _: *mut pdf_obj,
                    _: *mut pdf_obj,
                    _: *mut libc::c_void,
                ) -> i32,
        ),
        tp as *mut libc::c_void,
    );
    if error == 0 {
        if (*tp).mode.fill != 0i32 && pdf_get_version() < 4_u32 {
            spc_warn(
                spe,
                b"Transparent fill mode requires PDF version 1.4.\x00" as *const u8 as *const i8,
            );
            (*tp).mode.fill = 0i32
        }
    }
    error
}
/* DEBUG */
static mut tpic_handlers: [spc_handler; 13] = {
    [
        {
            let mut init = spc_handler {
                key: b"pn\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_pn
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"pa\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_pa
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"fp\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_fp
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ip\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_ip
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"da\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_da
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"dt\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_dt
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"sp\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_sp
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ar\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_ar
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ia\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_ia
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"sh\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_sh
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"wh\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_wh
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"bk\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_bk
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"tx\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_tpic_tx
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_tpic_check_special(mut buf: *const i8, mut len: i32) -> bool {
    let mut istpic: bool = false;
    let mut hasnsp: bool = false;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut i: size_t = 0;
    p = buf;
    endptr = p.offset(len as isize);
    skip_blank(&mut p, endptr);
    if p.offset(strlen(b"tpic:\x00" as *const u8 as *const i8) as isize) < endptr
        && memcmp(
            p as *const libc::c_void,
            b"tpic:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"tpic:\x00" as *const u8 as *const i8),
        ) == 0
    {
        p = p.offset(strlen(b"tpic:\x00" as *const u8 as *const i8) as isize);
        hasnsp = true
    }
    q = parse_c_ident(&mut p, endptr);
    if q.is_null() {
        istpic = false
    } else if !q.is_null()
        && hasnsp as i32 != 0
        && streq_ptr(q, b"__setopt__\x00" as *const u8 as *const i8) as i32 != 0
    {
        istpic = true;
        free(q as *mut libc::c_void);
    } else {
        i = 0i32 as size_t;
        while i
            < (::std::mem::size_of::<[spc_handler; 13]>() as u64)
                .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
        {
            if streq_ptr(q, tpic_handlers[i as usize].key) {
                istpic = true;
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
        free(q as *mut libc::c_void);
    }
    istpic
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
pub unsafe extern "C" fn spc_tpic_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut i: u32 = 0;
    let mut hasnsp: i32 = 0i32;
    let mut error: i32 = -1i32;
    assert!(!sph.is_null() && !spe.is_null() && !ap.is_null());
    skip_blank(&mut (*ap).curptr, (*ap).endptr);
    if (*ap)
        .curptr
        .offset(strlen(b"tpic:\x00" as *const u8 as *const i8) as isize)
        < (*ap).endptr
        && memcmp(
            (*ap).curptr as *const libc::c_void,
            b"tpic:\x00" as *const u8 as *const i8 as *const libc::c_void,
            strlen(b"tpic:\x00" as *const u8 as *const i8),
        ) == 0
    {
        (*ap).curptr = (*ap)
            .curptr
            .offset(strlen(b"tpic:\x00" as *const u8 as *const i8) as isize);
        hasnsp = 1i32
    }
    q = parse_c_ident(&mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        error = -1i32
    } else if !q.is_null()
        && hasnsp != 0
        && streq_ptr(q, b"__setopt__\x00" as *const u8 as *const i8) as i32 != 0
    {
        (*ap).command = b"__setopt__\x00" as *const u8 as *const i8;
        (*sph).key = b"tpic:\x00" as *const u8 as *const i8;
        (*sph).exec = Some(
            spc_handler_tpic__setopts
                as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
        );
        skip_blank(&mut (*ap).curptr, (*ap).endptr);
        error = 0i32;
        free(q as *mut libc::c_void);
    } else {
        i = 0_u32;
        while (i as u64)
            < (::std::mem::size_of::<[spc_handler; 13]>() as u64)
                .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
        {
            if streq_ptr(q, tpic_handlers[i as usize].key) {
                (*ap).command = tpic_handlers[i as usize].key;
                (*sph).key = b"tpic:\x00" as *const u8 as *const i8;
                (*sph).exec = tpic_handlers[i as usize].exec;
                skip_blank(&mut (*ap).curptr, (*ap).endptr);
                error = 0i32;
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
        free(q as *mut libc::c_void);
    }
    error
}
