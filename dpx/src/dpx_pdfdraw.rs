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
use crate::warn;

use super::dpx_pdfcolor::{
    pdf_color_compare, pdf_color_copycolor, pdf_color_graycolor, pdf_color_is_valid,
    pdf_color_to_string, pdf_color_type,
};
use super::dpx_pdfdev::{pdf_sprint_coord, pdf_sprint_matrix, pdf_sprint_rect};
use libc::free;
extern "C" {
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    fn pdf_dev_reset_fonts(newpage: i32);
    #[no_mangle]
    fn pdf_dev_get_param(param_type: i32) -> i32;
    #[no_mangle]
    fn pdf_sprint_length(buf: *mut i8, value: f64) -> i32;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const i8, length: u32);
}

pub use super::dpx_pdfcolor::pdf_color;

use super::dpx_pdfdev::{pdf_coord, pdf_rect, pdf_tmatrix};

/* Graphics State */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_gstate {
    pub cp: pdf_coord,
    pub matrix: pdf_tmatrix,
    pub strokecolor: pdf_color,
    pub fillcolor: pdf_color,
    pub linedash: C2RustUnnamed,
    pub linewidth: f64,
    pub linecap: i32,
    pub linejoin: i32,
    pub miterlimit: f64,
    pub flatness: i32,
    pub path: pdf_path,
    pub flags: i32,
    pub pt_fixee: pdf_coord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_path {
    pub num_paths: u32,
    pub max_paths: u32,
    pub path: *mut pa_elem,
    /* cm,  - */
    /* colorspace here */
    /* d,  D  */
    /* w,  LW */
    /* J,  LC */
    /* j,  LJ */
    /* M,  ML */
    /* i,  FL, 0 to 100 (0 for use device-default) */
    /* internal */
    /* bookkeeping the origin of the last transform applied */
    /* _PDF_DRAW_H_ */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_elem {
    pub typ: PeType,
    pub p: [pdf_coord; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub num_dash: i32,
    pub pattern: [f64; 16],
    pub offset: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m_stack {
    pub size: i32,
    pub top: *mut m_stack_elem,
    pub bottom: *mut m_stack_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m_stack_elem {
    pub data: *mut libc::c_void,
    pub prev: *mut m_stack_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub opchr: i8,
    pub n_pts: i32,
    pub strkey: *const i8,
}
unsafe extern "C" fn inversematrix(mut W: &mut pdf_tmatrix, mut M: &pdf_tmatrix) -> i32 {
    let mut det: f64 = 0.;
    det = M.a * M.d - M.b * M.c;
    if det.abs() < 2.5e-16f64 {
        warn!("Inverting matrix with zero determinant...");
        return -1i32;
    }
    W.a = M.d / det;
    W.b = -M.b / det;
    W.c = -M.c / det;
    W.d = M.a / det;
    W.e = M.c * M.f - M.d * M.e;
    W.f = M.b * M.e - M.a * M.f;
    0i32
}

extern "C" fn pdf_coord__transform(p: &mut pdf_coord, M: &pdf_tmatrix) -> i32 {
    let pdf_coord { x, y } = *p;
    p.x = x * M.a + y * M.c + M.e;
    p.y = x * M.b + y * M.d + M.f;
    0i32
}
extern "C" fn pdf_coord__dtransform(p: &mut pdf_coord, M: &pdf_tmatrix) -> i32 {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let pdf_coord { x, y } = *p;
    p.x = x * M.a + y * M.c;
    p.y = x * M.b + y * M.d;
    0i32
}
unsafe extern "C" fn pdf_coord__idtransform(p: &mut pdf_coord, M: &pdf_tmatrix) -> i32 {
    let mut W = pdf_tmatrix::new();
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut error: i32 = 0;
    error = inversematrix(&mut W, M);
    if error != 0 {
        return error;
    }
    let pdf_coord { x, y } = *p;
    p.x = x * W.a + y * W.c;
    p.y = x * W.b + y * W.d;
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_invertmatrix(M: &mut pdf_tmatrix) {
    let mut W = pdf_tmatrix::new();
    let mut det: f64 = 0.;
    det = M.a * M.d - M.b * M.c;
    if det.abs() < 2.5e-16f64 {
        warn!("Inverting matrix with zero determinant...");
        W.a = 1.0f64;
        W.c = 0.0f64;
        W.b = 0.0f64;
        W.d = 1.0f64;
        W.e = 0.0f64;
        W.f = 0.0f64
    } else {
        W.a = M.d / det;
        W.b = -M.b / det;
        W.c = -M.c / det;
        W.d = M.a / det;
        W.e = M.c * M.f - M.d * M.e;
        W.f = M.b * M.e - M.a * M.f;
        W.e /= det;
        W.f /= det
    }
    M.a = W.a;
    M.b = W.b;
    M.c = W.c;
    M.d = W.d;
    M.e = W.e;
    M.f = W.f;
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum PeType {
    MOVETO = 0,
    LINETO = 1,
    CURVETO = 2,
    CURVETO_V = 3,
    CURVETO_Y = 4,
    CLOSEPATH = 5,
    TERMINATE = 6,
}

impl PeType {
    pub fn opchr(&self) -> i8 {
        use PeType::*;
        (match *self {
            MOVETO => b'm',
            LINETO => b'l',
            CURVETO => b'c',
            CURVETO_V => b'v',
            CURVETO_Y => b'y',
            CLOSEPATH => b'h',
            TERMINATE => b' ',
        }) as i8
    }
    pub fn n_pts(&self) -> usize {
        use PeType::*;
        match *self {
            MOVETO => 1,
            LINETO => 1,
            CURVETO => 3,
            CURVETO_V => 2,
            CURVETO_Y => 2,
            CLOSEPATH => 0,
            TERMINATE => 0,
        }
    }
}

static mut fmt_buf: [i8; 1024] = [0; 1024];
unsafe extern "C" fn init_a_path(p: &mut pdf_path) {
    p.num_paths = 0_u32;
    p.max_paths = 0_u32;
    p.path = 0 as *mut pa_elem;
}
extern "C" fn pdf_path__clearpath(mut p: &mut pdf_path) {
    p.num_paths = 0_u32;
}
unsafe extern "C" fn pdf_path__growpath(p: &mut pdf_path, mut max_pe: u32) -> i32 {
    if max_pe < p.max_paths {
        return 0i32;
    }
    p.max_paths = if p.max_paths + 8 > max_pe {
        p.max_paths + 8_u32
    } else {
        max_pe
    };
    p.path = renew(
        p.path as *mut libc::c_void,
        (p.max_paths as u64).wrapping_mul(::std::mem::size_of::<pa_elem>() as u64) as u32,
    ) as *mut pa_elem;
    0i32
}
unsafe extern "C" fn clear_a_path(p: &mut pdf_path) {
    p.path = mfree(p.path as *mut libc::c_void) as *mut pa_elem;
    p.num_paths = 0_u32;
    p.max_paths = 0_u32;
}
unsafe extern "C" fn pdf_path__copypath(p1: &mut pdf_path, p0: &pdf_path) -> i32 {
    let mut i: u32 = 0;
    pdf_path__growpath(p1, p0.num_paths);
    i = 0_u32;
    while i < p0.num_paths {
        let pe1 = &mut *(*p1).path.offset(i as isize);
        let pe0 = &mut *p0.path.offset(i as isize);
        /* FIXME */
        *pe1 = *pe0;
        i = i + 1
    }
    p1.num_paths = p0.num_paths;
    0i32
}
/* start new subpath */
unsafe extern "C" fn pdf_path__moveto(
    pa: &mut pdf_path,
    cp: &mut pdf_coord,
    p0: &pdf_coord,
) -> i32 {
    pdf_path__growpath(pa, pa.num_paths + 1);
    if pa.num_paths > 0_u32 {
        let pe = &mut *(*pa)
            .path
            .offset((pa.num_paths - 1) as isize);
        if pe.typ == PeType::MOVETO {
            *cp = *p0;
            pe.p[0] = *cp;
            return 0i32;
        }
    }
    let fresh0 = pa.num_paths;
    pa.num_paths += 1;
    let pe = &mut *pa.path.offset(fresh0 as isize);
    pe.typ = PeType::MOVETO;
    *cp = *p0;
    pe.p[0] = *cp;
    0i32
}
/* Do 'compression' of path while adding new path elements.
 * Sequantial moveto command will be replaced with a
 * single moveto. If cp is not equal to the last point in pa,
 * then moveto is inserted (starting new subpath).
 * FIXME:
 * 'moveto' must be used to enforce starting new path.
 * This affects how 'closepath' is treated.
 */
unsafe extern "C" fn pdf_path__next_pe<'a>(pa: &'a mut pdf_path, cp: &pdf_coord) -> &'a mut pa_elem {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    pdf_path__growpath(pa, pa.num_paths + 2);
    if pa.num_paths == 0_u32 {
        let fresh1 = pa.num_paths;
        pa.num_paths += 1;
        let pe = &mut *pa.path.offset(fresh1 as isize);
        pe.typ = PeType::MOVETO;
        pe.p[0].x = cp.x;
        pe.p[0].y = cp.y;
        let fresh2 = pa.num_paths;
        pa.num_paths += 1;
        return &mut *pa.path.offset(fresh2 as isize);
    }
    let mut pe = &mut *(*pa)
        .path
        .offset((pa.num_paths - 1) as isize);
    match pe.typ {
        PeType::MOVETO => {
            pe.p[0].x = cp.x;
            pe.p[0].y = cp.y
        }
        PeType::LINETO => {
            if &mut *pe.p.as_mut_ptr().offset(0) != cp {
                let fresh3 = pa.num_paths;
                pa.num_paths += 1;
                pe = &mut *pa.path.offset(fresh3 as isize);
                pe.typ = PeType::MOVETO;
                pe.p[0].x = cp.x;
                pe.p[0].y = cp.y
            }
        }
        PeType::CURVETO => {
            if &mut *pe.p.as_mut_ptr().offset(2) != cp {
                let fresh4 = pa.num_paths;
                pa.num_paths += 1;
                pe = &mut *pa.path.offset(fresh4 as isize);
                pe.typ = PeType::MOVETO;
                pe.p[0].x = cp.x;
                pe.p[0].y = cp.y
            }
        }
        PeType::CURVETO_Y | PeType::CURVETO_V => {
            if &mut *pe.p.as_mut_ptr().offset(1) != cp {
                let fresh5 = pa.num_paths;
                pa.num_paths += 1;
                pe = &mut *pa.path.offset(fresh5 as isize);
                pe.typ = PeType::MOVETO;
                pe.p[0].x = cp.x;
                pe.p[0].y = cp.y
            }
        }
        PeType::CLOSEPATH => {
            let fresh6 = pa.num_paths;
            pa.num_paths += 1;
            pe = &mut *pa.path.offset(fresh6 as isize);
            pe.typ = PeType::MOVETO;
            pe.p[0].x = cp.x;
            pe.p[0].y = cp.y
        }
        _ => {}
    }
    let fresh7 = pa.num_paths;
    pa.num_paths += 1;
    &mut *pa.path.offset(fresh7 as isize)
}
unsafe extern "C" fn pdf_path__transform(pa: &mut pdf_path, M: &pdf_tmatrix) -> i32 {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    let mut n: u32 = 0_u32;
    let mut i: u32 = 0;
    i = 0_u32;
    while i < pa.num_paths {
        let pe = &mut *pa.path.offset(i as isize);
        n = (if pe.typ != PeType::TERMINATE {
            (*pe).typ.n_pts() as i32
        } else {
            0i32
        }) as u32;
        loop {
            let fresh8 = n;
            n += 1;
            if !(fresh8 > 0_u32) {
                break;
            }
            pdf_coord__transform(&mut *(*pe).p.as_mut_ptr().offset(n as isize), M);
        }
        i += 1
    }
    0i32
}
/* Path Construction */
unsafe extern "C" fn pdf_path__lineto(
    pa: &mut pdf_path,
    cp: &mut pdf_coord,
    p0: &pdf_coord,
) -> i32 {
    let pe = pdf_path__next_pe(pa, cp);
    pe.typ = PeType::LINETO;
    cp.x = p0.x;
    pe.p[0].x = cp.x;
    cp.y = p0.y;
    pe.p[0].y = cp.y;
    0i32
}
unsafe extern "C" fn pdf_path__curveto(
    pa: &mut pdf_path,
    cp: &mut pdf_coord,
    p0: &pdf_coord,
    p1: &pdf_coord,
    p2: &pdf_coord,
) -> i32 {
    let pe = pdf_path__next_pe(pa, cp);
    if cp == p0 {
        pe.typ = PeType::CURVETO_V;
        pe.p[0] = *p1;
        *cp = *p2;
        pe.p[1] = *cp;
    } else if p1 == p2 {
        pe.typ = PeType::CURVETO_Y;
        pe.p[0] = *p0;
        *cp = *p1;
        pe.p[1] = *cp;
    } else {
        pe.typ = PeType::CURVETO;
        pe.p[0] = *p0;
        pe.p[1] = *p1;
        *cp = *p2;
        pe.p[2] = *cp;
    }
    0i32
}
/* This isn't specified as cp to somewhere. */
unsafe extern "C" fn pdf_path__elliptarc(
    pa: &mut pdf_path,
    cp: &mut pdf_coord,
    ca: &pdf_coord,
    mut r_x: f64,
    mut r_y: f64,
    mut xar: f64,
    mut a_0: f64,
    mut a_1: f64,
    mut a_d: i32,
) -> i32
/* arc orientation        */ {
    let mut b: f64 = 0.; /* number of segments */
    let mut b_x: f64 = 0.;
    let mut b_y: f64 = 0.;
    let mut d_a: f64 = 0.;
    let mut q: f64 = 0.;
    let mut p0 = pdf_coord::new();
    let mut p1 = pdf_coord::new();
    let mut p2 = pdf_coord::new();
    let mut p3 = pdf_coord::new();
    let mut e0 = pdf_coord::new();
    let mut e1 = pdf_coord::new();
    let mut T = pdf_tmatrix::new();
    let mut n_c: i32 = 0;
    let mut i: i32 = 0;
    let mut error: i32 = 0i32;
    if r_x.abs() < 2.5e-16f64 || r_y.abs() < 2.5e-16f64 {
        return -1i32;
    }
    if a_d < 0i32 {
        while a_1 > a_0 {
            a_1 -= 360.0f64
        }
    } else {
        while a_1 < a_0 {
            a_0 -= 360.0f64
        }
    }
    d_a = a_1 - a_0;
    n_c = 1i32;
    while d_a.abs() > 90.0f64 * n_c as f64 {
        n_c += 1
    }
    d_a /= n_c as f64;
    if d_a.abs() < 2.5e-16f64 {
        return -1i32;
    }
    a_0 *= core::f64::consts::PI / 180.;
    a_1 *= core::f64::consts::PI / 180.;
    d_a *= core::f64::consts::PI / 180.;
    xar *= core::f64::consts::PI / 180.;
    let (s, c) = xar.sin_cos();
    T.a = c;
    T.b = s;
    T.c = -s;
    T.d = c;
    T.e = 0.;
    T.f = 0.;
    /* A parameter that controls cb-curve (off-curve) points */
    b = 4.0f64 * (1.0f64 - (0.5f64 * d_a).cos()) / (3.0f64 * (0.5f64 * d_a).sin());
    b_x = r_x * b;
    b_y = r_y * b;
    let (s, c) = a_0.sin_cos();
    p0.x = r_x * c;
    p0.y = r_y * s;
    pdf_coord__transform(&mut p0, &mut T);
    p0.x += ca.x;
    p0.y += ca.y;
    if pa.num_paths == 0_u32 {
        pdf_path__moveto(pa, cp, &mut p0);
    } else if cp != &p0 {
        pdf_path__lineto(pa, cp, &mut p0);
        /* add line seg */
    }
    i = 0i32;
    while error == 0 && i < n_c {
        q = a_0 + i as f64 * d_a;
        let (s, c) = q.sin_cos();
        e0.x = c;
        e0.y = s;
        let (s, c) = (q + d_a).sin_cos();
        e1.x = c;
        e1.y = s;
        /* Condition for tangent vector requirs
         *  d1 = p1 - p0 = f ( sin a, -cos a)
         *  d2 = p2 - p3 = g ( sin b, -cos b)
         * and from symmetry
         *  g^2 = f^2
         */
        p0.x = r_x * e0.x; /* s.p. */
        p0.y = r_y * e0.y; /* e.p. */
        p3.x = r_x * e1.x;
        p3.y = r_y * e1.y;
        p1.x = -b_x * e0.y;
        p1.y = b_y * e0.x;
        p2.x = b_x * e1.y;
        p2.y = -b_y * e1.x;
        pdf_coord__transform(&mut p0, &mut T);
        pdf_coord__transform(&mut p1, &mut T);
        pdf_coord__transform(&mut p2, &mut T);
        pdf_coord__transform(&mut p3, &mut T);
        p0.x += ca.x;
        p0.y += ca.y;
        p3.x += ca.x;
        p3.y += ca.y;
        p1.x += p0.x;
        p1.y += p0.y;
        p2.x += p3.x;
        p2.y += p3.y;
        error = pdf_path__curveto(pa, &mut p0, &mut p1, &mut p2, &mut p3);
        *cp = p3.clone();
        i += 1
    }
    error
}
unsafe extern "C" fn pdf_path__closepath(pa: &mut pdf_path, cp: &mut pdf_coord) -> i32
/* no arg */ {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    let mut i: i32 = 0;
    /* search for start point of the last subpath */
    i = (pa.num_paths - 1) as i32; /* No path or no start point(!) */
    while i >= 0i32 {
        pe = &mut *pa.path.offset(i as isize) as *mut pa_elem;
        if (*pe).typ == PeType::MOVETO {
            break;
        }
        i -= 1
    }
    if pe.is_null() || i < 0i32 {
        return -1i32;
    }
    *cp = (*pe).p[0].clone();
    pdf_path__growpath(pa, pa.num_paths + 1);
    /* NOTE:
     *  Manually closed path without closepath is not
     *  affected by linejoin. A path with coincidental
     *  starting and ending point is not the same as
     *  'closed' path.
     */
    let fresh9 = pa.num_paths;
    pa.num_paths += 1;
    pe = &mut *pa.path.offset(fresh9 as isize) as *mut pa_elem;
    (*pe).typ = PeType::CLOSEPATH;
    0i32
}
/*
 *  x y width height re
 *
 * is equivalent to
 *
 *  x y m
 *  (x + width) y l
 *  (x + width) (y + height) l
 *  x (y + height) l
 *  h
 */
/* Just for quick test */
unsafe extern "C" fn pdf_path__isarect(pa: &mut pdf_path, mut f_ir: i32) -> i32
/* fill-rule is ignorable */ {
    if pa.num_paths == 5_u32 {
        let pe0 = &mut *pa.path.offset(0);
        let pe1 = &mut *pa.path.offset(1);
        let pe2 = &mut *pa.path.offset(2);
        let pe3 = &mut *pa.path.offset(3);
        let pe4 = &mut *pa.path.offset(4);
        if pe0.typ == PeType::MOVETO
            && pe1.typ == PeType::LINETO
            && pe2.typ == PeType::LINETO
            && pe3.typ == PeType::LINETO
            && pe4.typ == PeType::CLOSEPATH
        {
            if pe1.p[0].y - pe0.p[0].y == 0.
                && pe2.p[0].x - pe1.p[0].x == 0.
                && pe3.p[0].y - pe2.p[0].y == 0.
            {
                if pe1.p[0].x - pe0.p[0].x == pe2.p[0].x - pe3.p[0].x {
                    return 1i32;
                }
            /* Winding number is different but ignore it here. */
            } else if f_ir != 0
                && pe1.p[0].x - pe0.p[0].x == 0.
                && pe2.p[0].y - pe1.p[0].y == 0.
                && pe3.p[0].x - pe2.p[0].x == 0.
            {
                if pe1.p[0].y - pe0.p[0].y == pe2.p[0].y - pe3.p[0].y {
                    return 1i32;
                }
            }
        }
    }
    0i32
}
/* Path Painting */
/* F is obsoleted */
unsafe extern "C" fn INVERTIBLE_MATRIX(M: &pdf_tmatrix) -> i32 {
    if (M.a * M.d - M.b * M.c).abs() < 2.5e-16f64 {
        warn!("Transformation matrix not invertible.");
        warn!("--- M = [{} {} {} {} {} {}]", M.a, M.b, M.c, M.d, M.e, M.f,);
        return -1i32;
    }
    0i32
}
/* rectfill, rectstroke, rectclip, recteoclip
 *
 * Draw isolated rectangle without actually doing
 * gsave/grestore operation.
 *
 * TODO:
 *  linestyle, fill-opacity, stroke-opacity,....
 *  As this routine draw a single graphics object
 *  each time, there should be options for specifying
 *  various drawing styles, which might inherite
 *  current graphcs state parameter.
 */
unsafe extern "C" fn pdf_dev__rectshape(
    r: &pdf_rect,
    M: Option<&pdf_tmatrix>,
    mut opchr: i8,
) -> i32 {
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut len: i32 = 0i32;
    let mut isclip: i32 = 0i32;
    let mut p = pdf_coord::new();
    let mut wd: f64 = 0.;
    let mut ht: f64 = 0.;
    assert!(
        (opchr as i32 == 'f' as i32
            || opchr as i32 == 'F' as i32
            || opchr as i32 == 's' as i32
            || opchr as i32 == 'S' as i32
            || opchr as i32 == 'b' as i32
            || opchr as i32 == 'B' as i32
            || opchr as i32 == 'W' as i32
            || opchr as i32 == ' ' as i32)
    );
    isclip = if opchr as i32 == 'W' as i32 || opchr as i32 == ' ' as i32 {
        1i32
    } else {
        0i32
    };
    /* disallow matrix for clipping.
     * q ... clip Q does nothing and
     * n M cm ... clip n alter CTM.
     */
    if M.is_some() && (isclip != 0 || INVERTIBLE_MATRIX(M.unwrap()) == 0) {
        return -1i32;
    } /* op: q cm n re Q */
    graphics_mode();
    let fresh10 = len;
    len = len + 1;
    *buf.offset(fresh10 as isize) = ' ' as i32 as i8;
    if isclip == 0 {
        let fresh11 = len;
        len = len + 1;
        *buf.offset(fresh11 as isize) = 'q' as i32 as i8;
        if let Some(m) = M {
            let fresh12 = len;
            len = len + 1;
            *buf.offset(fresh12 as isize) = ' ' as i32 as i8;
            len += pdf_sprint_matrix(buf.offset(len as isize), m);
            let fresh13 = len;
            len = len + 1;
            *buf.offset(fresh13 as isize) = ' ' as i32 as i8;
            let fresh14 = len;
            len = len + 1;
            *buf.offset(fresh14 as isize) = 'c' as i32 as i8;
            let fresh15 = len;
            len = len + 1;
            *buf.offset(fresh15 as isize) = 'm' as i32 as i8
        }
        let fresh16 = len;
        len = len + 1;
        *buf.offset(fresh16 as isize) = ' ' as i32 as i8
    }
    let fresh17 = len;
    len = len + 1;
    *buf.offset(fresh17 as isize) = 'n' as i32 as i8;
    p.x = r.llx;
    p.y = r.lly;
    wd = r.urx - r.llx;
    ht = r.ury - r.lly;
    let fresh18 = len;
    len = len + 1;
    *buf.offset(fresh18 as isize) = ' ' as i32 as i8;
    len += pdf_sprint_coord(buf.offset(len as isize), &mut p);
    let fresh19 = len;
    len = len + 1;
    *buf.offset(fresh19 as isize) = ' ' as i32 as i8;
    len += pdf_sprint_length(buf.offset(len as isize), wd);
    let fresh20 = len;
    len = len + 1;
    *buf.offset(fresh20 as isize) = ' ' as i32 as i8;
    len += pdf_sprint_length(buf.offset(len as isize), ht);
    let fresh21 = len;
    len = len + 1;
    *buf.offset(fresh21 as isize) = ' ' as i32 as i8;
    let fresh22 = len;
    len = len + 1;
    *buf.offset(fresh22 as isize) = 'r' as i32 as i8;
    let fresh23 = len;
    len = len + 1;
    *buf.offset(fresh23 as isize) = 'e' as i32 as i8;
    if opchr as i32 != ' ' as i32 {
        let fresh24 = len;
        len = len + 1;
        *buf.offset(fresh24 as isize) = ' ' as i32 as i8;
        let fresh25 = len;
        len = len + 1;
        *buf.offset(fresh25 as isize) = opchr;
        let fresh26 = len;
        len = len + 1;
        *buf.offset(fresh26 as isize) = ' ' as i32 as i8;
        let fresh27 = len;
        len = len + 1;
        *buf.offset(fresh27 as isize) = (if isclip != 0 { 'n' as i32 } else { 'Q' as i32 }) as i8
    }
    pdf_doc_add_page_content(buf, len as u32);
    0i32
}
static mut path_added: i32 = 0i32;
/* FIXME */
unsafe extern "C" fn pdf_dev__flushpath(
    pa: &mut pdf_path,
    mut opchr: i8,
    mut rule: i32,
    mut ignore_rule: i32,
) -> i32 {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem; /* FIXME */
    let mut pe1: *mut pa_elem = 0 as *mut pa_elem; /* width...  */
    let mut b: *mut i8 = fmt_buf.as_mut_ptr(); /* height... */
    let mut b_len: i32 = 1024i32; /* op: re */
    let mut r = pdf_rect::new(); /* op: m l c v y h */
    let mut pt: *mut pdf_coord = 0 as *mut pdf_coord; /* op: m l c v y h */
    let mut n_pts: i32 = 0; /* op: f F s S b B W f* F* s* S* b* B* W* */
    let mut n_seg: i32 = 0; /* default to 1 in PDF */
    let mut len: i32 = 0i32;
    let mut isclip: i32 = 0i32;
    let mut isrect: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    assert!((opchr as i32 == 'f' as i32
                || opchr as i32 == 'F' as i32
                || opchr as i32 == 's' as i32
                || opchr as i32 == 'S' as i32
                || opchr as i32 == 'b' as i32
                || opchr as i32 == 'B' as i32
                || opchr as i32 == 'W' as i32
                || opchr as i32 == ' ' as i32)
    );
    isclip = if opchr as i32 == 'W' as i32 {
        1i32
    } else {
        0i32
    };
    if pa.num_paths <= 0_u32 && path_added == 0i32 {
        return 0i32;
    }
    path_added = 0i32;
    graphics_mode();
    isrect = pdf_path__isarect(pa, ignore_rule);
    if isrect != 0 {
        pe = &mut *pa.path.offset(0) as *mut pa_elem;
        pe1 = &mut *pa.path.offset(2) as *mut pa_elem;
        r.llx = (*pe).p[0].x;
        r.lly = (*pe).p[0].y;
        r.urx = (*pe1).p[0].x - (*pe).p[0].x;
        r.ury = (*pe1).p[0].y - (*pe).p[0].y;
        let fresh28 = len;
        len = len + 1;
        *b.offset(fresh28 as isize) = ' ' as i32 as i8;
        len += pdf_sprint_rect(b.offset(len as isize), &r);
        let fresh29 = len;
        len = len + 1;
        *b.offset(fresh29 as isize) = ' ' as i32 as i8;
        let fresh30 = len;
        len = len + 1;
        *b.offset(fresh30 as isize) = 'r' as i32 as i8;
        let fresh31 = len;
        len = len + 1;
        *b.offset(fresh31 as isize) = 'e' as i32 as i8;
        pdf_doc_add_page_content(b, len as u32);
        len = 0i32
    } else {
        n_seg = pa.num_paths as i32;
        i = 0i32;
        len = 0i32;
        pe = &mut *pa.path.offset(0) as *mut pa_elem;
        while i < n_seg {
            n_pts = if !pe.is_null() && (*pe).typ != PeType::TERMINATE {
                (*pe).typ.n_pts() as i32
            } else {
                0i32
            };
            j = 0i32;
            pt = &mut *(*pe).p.as_mut_ptr().offset(0) as *mut pdf_coord;
            while j < n_pts {
                let fresh32 = len;
                len = len + 1;
                *b.offset(fresh32 as isize) = ' ' as i32 as i8;
                len += pdf_sprint_coord(b.offset(len as isize), &mut *pt);
                j += 1;
                pt = pt.offset(1)
            }
            let fresh33 = len;
            len = len + 1;
            *b.offset(fresh33 as isize) = ' ' as i32 as i8;
            let fresh34 = len;
            len = len + 1;
            *b.offset(fresh34 as isize) =
                if !pe.is_null() && (*pe).typ != PeType::TERMINATE {
                    (*pe).typ.opchr()
                } else {
                    b' ' as i8
                };
            if len + 128i32 > b_len {
                pdf_doc_add_page_content(b, len as u32);
                len = 0i32
            }
            pe = pe.offset(1);
            i += 1
        }
        if len > 0i32 {
            pdf_doc_add_page_content(b, len as u32);
            len = 0i32
        }
    }
    let fresh35 = len;
    len = len + 1;
    *b.offset(fresh35 as isize) = ' ' as i32 as i8;
    let fresh36 = len;
    len = len + 1;
    *b.offset(fresh36 as isize) = opchr;
    if rule == 1i32 {
        let fresh37 = len;
        len = len + 1;
        *b.offset(fresh37 as isize) = '*' as i32 as i8
    }
    if isclip != 0 {
        let fresh38 = len;
        len = len + 1;
        *b.offset(fresh38 as isize) = ' ' as i32 as i8;
        let fresh39 = len;
        len = len + 1;
        *b.offset(fresh39 as isize) = 'n' as i32 as i8
    }
    pdf_doc_add_page_content(b, len as u32);
    0i32
}
unsafe extern "C" fn m_stack_init(mut stack: *mut m_stack) {
    assert!(!stack.is_null());
    (*stack).size = 0i32;
    (*stack).top = 0 as *mut m_stack_elem;
    (*stack).bottom = 0 as *mut m_stack_elem;
}
unsafe extern "C" fn m_stack_push(mut stack: *mut m_stack, mut data: *mut libc::c_void) {
    let mut elem: *mut m_stack_elem = 0 as *mut m_stack_elem;
    assert!(!stack.is_null());
    elem = new((1_u64).wrapping_mul(::std::mem::size_of::<m_stack_elem>() as u64) as u32)
        as *mut m_stack_elem;
    (*elem).prev = (*stack).top;
    (*elem).data = data;
    (*stack).top = elem;
    if (*stack).size == 0i32 {
        (*stack).bottom = elem
    }
    (*stack).size += 1;
}
unsafe extern "C" fn m_stack_pop(mut stack: *mut m_stack) -> *mut libc::c_void {
    let mut elem: *mut m_stack_elem = 0 as *mut m_stack_elem;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    assert!(!stack.is_null());
    if (*stack).size == 0i32 {
        return 0 as *mut libc::c_void;
    }
    data = (*(*stack).top).data;
    elem = (*stack).top;
    (*stack).top = (*elem).prev;
    if (*stack).size == 1i32 {
        (*stack).bottom = 0 as *mut m_stack_elem
    }
    free(elem as *mut libc::c_void);
    (*stack).size -= 1;
    data
}
unsafe extern "C" fn m_stack_top(mut stack: *mut m_stack) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    assert!(!stack.is_null());
    if (*stack).size == 0i32 {
        return 0 as *mut libc::c_void;
    }
    data = (*(*stack).top).data;
    data
}
static mut gs_stack: m_stack = m_stack {
    size: 0,
    top: 0 as *const m_stack_elem as *mut m_stack_elem,
    bottom: 0 as *const m_stack_elem as *mut m_stack_elem,
};
unsafe extern "C" fn init_a_gstate(mut gs: *mut pdf_gstate) {
    (*gs).cp.x = 0.0f64;
    (*gs).cp.y = 0.0f64;
    (*gs).matrix.a = 1.0f64;
    (*gs).matrix.b = 0.0f64;
    (*gs).matrix.c = 0.0f64;
    (*gs).matrix.d = 1.0f64;
    (*gs).matrix.e = 0.0f64;
    (*gs).matrix.f = 0.0f64;
    pdf_color_graycolor(&mut (*gs).strokecolor, 0.0f64);
    pdf_color_graycolor(&mut (*gs).fillcolor, 0.0f64);
    (*gs).linedash.num_dash = 0i32;
    (*gs).linedash.offset = 0i32 as f64;
    (*gs).linecap = 0i32;
    (*gs).linejoin = 0i32;
    (*gs).linewidth = 1.0f64;
    (*gs).miterlimit = 10.0f64;
    (*gs).flatness = 1i32;
    /* Internal variables */
    (*gs).flags = 0i32;
    init_a_path(&mut (*gs).path);
    (*gs).pt_fixee.x = 0i32 as f64;
    (*gs).pt_fixee.y = 0i32 as f64;
}
unsafe extern "C" fn clear_a_gstate(mut gs: *mut pdf_gstate) {
    clear_a_path(&mut (*gs).path);
    memset(
        gs as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<pdf_gstate>() as u64,
    );
}
unsafe extern "C" fn copy_a_gstate(mut gs1: *mut pdf_gstate, mut gs2: *mut pdf_gstate) {
    let mut i: i32 = 0;
    assert!(!gs1.is_null() && !gs2.is_null());
    (*gs1).cp.x = (*gs2).cp.x;
    (*gs1).cp.y = (*gs2).cp.y;
    (*gs1).matrix.a = (*gs2).matrix.a;
    (*gs1).matrix.b = (*gs2).matrix.b;
    (*gs1).matrix.c = (*gs2).matrix.c;
    (*gs1).matrix.d = (*gs2).matrix.d;
    (*gs1).matrix.e = (*gs2).matrix.e;
    (*gs1).matrix.f = (*gs2).matrix.f;
    /* TODO:
     * Path should be linked list and gsave only
     * record starting point within path rather than
     * copying whole path.
     */
    pdf_path__copypath(&mut (*gs1).path, &mut (*gs2).path); /* Initial state */
    (*gs1).linedash.num_dash = (*gs2).linedash.num_dash;
    i = 0i32;
    while i < (*gs2).linedash.num_dash {
        (*gs1).linedash.pattern[i as usize] = (*gs2).linedash.pattern[i as usize];
        i += 1
    }
    (*gs1).linedash.offset = (*gs2).linedash.offset;
    (*gs1).linecap = (*gs2).linecap;
    (*gs1).linejoin = (*gs2).linejoin;
    (*gs1).linewidth = (*gs2).linewidth;
    (*gs1).miterlimit = (*gs2).miterlimit;
    (*gs1).flatness = (*gs2).flatness;
    pdf_color_copycolor(&mut (*gs1).fillcolor, &mut (*gs2).fillcolor);
    pdf_color_copycolor(&mut (*gs1).strokecolor, &mut (*gs2).strokecolor);
    (*gs1).pt_fixee.x = (*gs2).pt_fixee.x;
    (*gs1).pt_fixee.y = (*gs2).pt_fixee.y;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_init_gstates() {
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    m_stack_init(&mut gs_stack);
    gs = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_gstate>() as u64) as u32)
        as *mut pdf_gstate;
    init_a_gstate(gs);
    m_stack_push(&mut gs_stack, gs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_clear_gstates() {
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if gs_stack.size > 1i32 {
        /* at least 1 elem. */
        warn!("GS stack depth is not zero at the end of the document."); /* op: q */
    }
    loop {
        gs = m_stack_pop(&mut gs_stack) as *mut pdf_gstate;
        if gs.is_null() {
            break;
        }
        clear_a_gstate(gs);
        free(gs as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_gsave() -> i32 {
    let mut gs0: *mut pdf_gstate = 0 as *mut pdf_gstate;
    let mut gs1: *mut pdf_gstate = 0 as *mut pdf_gstate;
    gs0 = m_stack_top(&mut gs_stack) as *mut pdf_gstate;
    gs1 = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_gstate>() as u64) as u32)
        as *mut pdf_gstate;
    init_a_gstate(gs1);
    copy_a_gstate(gs1, gs0);
    m_stack_push(&mut gs_stack, gs1 as *mut libc::c_void);
    pdf_doc_add_page_content(b" q\x00" as *const u8 as *const i8, 2_u32);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_grestore() -> i32 {
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if gs_stack.size <= 1i32 {
        /* Initial state at bottom */
        warn!("Too many grestores."); /* op: Q */
        return -1i32;
    }
    gs = m_stack_pop(&mut gs_stack) as *mut pdf_gstate;
    clear_a_gstate(gs);
    free(gs as *mut libc::c_void);
    pdf_doc_add_page_content(b" Q\x00" as *const u8 as *const i8, 2_u32);
    pdf_dev_reset_fonts(0i32);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_push_gstate() -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs0: *mut pdf_gstate = 0 as *mut pdf_gstate;
    gs0 = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_gstate>() as u64) as u32)
        as *mut pdf_gstate;
    init_a_gstate(gs0);
    m_stack_push(gss, gs0 as *mut libc::c_void);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_pop_gstate() -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if (*gss).size <= 1i32 {
        /* Initial state at bottom */
        warn!("Too many grestores.");
        return -1i32;
    }
    gs = m_stack_pop(gss) as *mut pdf_gstate;
    clear_a_gstate(gs);
    free(gs as *mut libc::c_void);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_current_depth() -> i32 {
    return gs_stack.size - 1i32;
    /* 0 means initial state */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_grestore_to(mut depth: i32) {
    let mut gss: *mut m_stack = &mut gs_stack; /* op: Q */
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    assert!(depth >= 0i32);
    if (*gss).size > depth + 1i32 {
        warn!("Closing pending transformations at end of page/XObject.");
    }
    while (*gss).size > depth + 1i32 {
        pdf_doc_add_page_content(b" Q\x00" as *const u8 as *const i8, 2_u32);
        gs = m_stack_pop(gss) as *mut pdf_gstate;
        clear_a_gstate(gs);
        free(gs as *mut libc::c_void);
    }
    pdf_dev_reset_fonts(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_currentpoint(p: &mut pdf_coord) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpt = &mut (*gs).cp;
    *p = cpt.clone();
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_currentmatrix(M: &mut pdf_tmatrix) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let CTM = &mut (*gs).matrix;
    M.a = CTM.a;
    M.b = CTM.b;
    M.c = CTM.c;
    M.d = CTM.d;
    M.e = CTM.e;
    M.f = CTM.f;
    0i32
}
/*
 * mask == 0 means stroking color, mask == 0x20 nonstroking color
 *
 * force == 1 means that operators will be generated even if
 *   the color is the same as the current graphics state color
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_color(color: &pdf_color, mut mask: i8, mut force: i32) {
    let mut len: i32 = 0;
    let mut gs: *mut pdf_gstate = m_stack_top(&mut gs_stack) as *mut pdf_gstate;
    let mut current = if mask as i32 != 0 {
        &mut (*gs).fillcolor
    } else {
        &mut (*gs).strokecolor
    };
    assert!(pdf_color_is_valid(color));
    if !(pdf_dev_get_param(2i32) != 0 && (force != 0 || pdf_color_compare(color, current) != 0)) {
        /* If "color" is already the current color, then do nothing
         * unless a color operator is forced
         */
        return;
    } /* op: RG K G rg k g etc. */
    graphics_mode(); /* Init to avoid compiler warning */
    len = pdf_color_to_string(color, fmt_buf.as_mut_ptr(), mask);
    let fresh40 = len;
    len = len + 1;
    fmt_buf[fresh40 as usize] = ' ' as i32 as i8;
    match pdf_color_type(color) {
        -3 => {
            let fresh41 = len;
            len = len + 1;
            fmt_buf[fresh41 as usize] = ('R' as i32 | mask as i32) as i8;
            let fresh42 = len;
            len = len + 1;
            fmt_buf[fresh42 as usize] = ('G' as i32 | mask as i32) as i8
        }
        -4 => {
            let fresh43 = len;
            len = len + 1;
            fmt_buf[fresh43 as usize] = ('K' as i32 | mask as i32) as i8
        }
        -1 => {
            let fresh44 = len;
            len = len + 1;
            fmt_buf[fresh44 as usize] = ('G' as i32 | mask as i32) as i8
        }
        _ => {}
    }
    pdf_doc_add_page_content(fmt_buf.as_mut_ptr(), len as u32);
    pdf_color_copycolor(current, color);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_concat(M: &pdf_tmatrix) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let cpt = &mut (*gs).cp;
    let mut CTM = &mut (*gs).matrix;
    let mut W = pdf_tmatrix::new();
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut len: i32 = 0i32;
    /* Adobe Reader erases page content if there are
     * non invertible transformation.
     */
    if (M.a * M.d - M.b * M.c).abs() < 2.5e-16f64 {
        warn!("Transformation matrix not invertible."); /* op: cm */
        warn!("--- M = [{} {} {} {} {} {}]", M.a, M.b, M.c, M.d, M.e, M.f,);
        return -1i32;
    }
    if (M.a - 1.0f64).abs() > 2.5e-16f64
        || M.b.abs() > 2.5e-16f64
        || M.c.abs() > 2.5e-16f64
        || (M.d - 1.0f64).abs() > 2.5e-16f64
        || M.e.abs() > 2.5e-16f64
        || M.f.abs() > 2.5e-16f64
    {
        let fresh45 = len;
        len = len + 1;
        *buf.offset(fresh45 as isize) = ' ' as i32 as i8;
        len += pdf_sprint_matrix(buf.offset(len as isize), M);
        let fresh46 = len;
        len = len + 1;
        *buf.offset(fresh46 as isize) = ' ' as i32 as i8;
        let fresh47 = len;
        len = len + 1;
        *buf.offset(fresh47 as isize) = 'c' as i32 as i8;
        let fresh48 = len;
        len = len + 1;
        *buf.offset(fresh48 as isize) = 'm' as i32 as i8;
        pdf_doc_add_page_content(buf, len as u32);
        let pdf_tmatrix {
            a: _tmp_a,
            b: _tmp_b,
            c: _tmp_c,
            d: _tmp_d,
            ..
        } = *CTM;
        CTM.a = M.a * _tmp_a + M.b * _tmp_c;
        CTM.b = M.a * _tmp_b + M.b * _tmp_d;
        CTM.c = M.c * _tmp_a + M.d * _tmp_c;
        CTM.d = M.c * _tmp_b + M.d * _tmp_d;
        CTM.e += M.e * _tmp_a + M.f * _tmp_c;
        CTM.f += M.e * _tmp_b + M.f * _tmp_d
    }
    inversematrix(&mut W, M);
    pdf_path__transform(cpa, &mut W);
    pdf_coord__transform(cpt, &mut W);
    0i32
}
/*
 * num w        LW  linewidth (g.t. 0)
 * int J        LC  linecap
 * int j        LJ  linejoin
 * num M        ML  miter limit (g.t. 0)
 * array num d  D   line dash
 * int ri       RI  renderint intnet
 * int i        FL  flatness tolerance (0-100)
 * name gs      --  name: res. name of ExtGState dict.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setmiterlimit(mut mlimit: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack; /* op: M */
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate; /* op: J */
    let mut len: i32 = 0i32; /* op: j */
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr(); /* op: w */
    if (*gs).miterlimit != mlimit {
        let fresh49 = len; /* op: */
        len = len + 1; /* op: */
        *buf.offset(fresh49 as isize) = ' ' as i32 as i8; /* op: */
        len += pdf_sprint_length(buf.offset(len as isize), mlimit); /* op: */
        let fresh50 = len; /* op: d */
        len = len + 1;
        *buf.offset(fresh50 as isize) = ' ' as i32 as i8;
        let fresh51 = len;
        len = len + 1;
        *buf.offset(fresh51 as isize) = 'M' as i32 as i8;
        pdf_doc_add_page_content(buf, len as u32);
        (*gs).miterlimit = mlimit
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinecap(mut capstyle: i32) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if (*gs).linecap != capstyle {
        len = sprintf(buf, b" %d J\x00" as *const u8 as *const i8, capstyle);
        pdf_doc_add_page_content(buf, len as u32);
        (*gs).linecap = capstyle
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinejoin(mut joinstyle: i32) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if (*gs).linejoin != joinstyle {
        len = sprintf(buf, b" %d j\x00" as *const u8 as *const i8, joinstyle);
        pdf_doc_add_page_content(buf, len as u32);
        (*gs).linejoin = joinstyle
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinewidth(mut width: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if (*gs).linewidth != width {
        let fresh52 = len;
        len = len + 1;
        *buf.offset(fresh52 as isize) = ' ' as i32 as i8;
        len += pdf_sprint_length(buf.offset(len as isize), width);
        let fresh53 = len;
        len = len + 1;
        *buf.offset(fresh53 as isize) = ' ' as i32 as i8;
        let fresh54 = len;
        len = len + 1;
        *buf.offset(fresh54 as isize) = 'w' as i32 as i8;
        pdf_doc_add_page_content(buf, len as u32);
        (*gs).linewidth = width
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setdash(
    mut count: i32,
    mut pattern: *mut f64,
    mut offset: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut i: i32 = 0;
    (*gs).linedash.num_dash = count;
    (*gs).linedash.offset = offset;
    pdf_doc_add_page_content(b" [\x00" as *const u8 as *const i8, 2_u32);
    i = 0i32;
    while i < count {
        *buf.offset(0) = ' ' as i32 as i8;
        len = pdf_sprint_length(buf.offset(1), *pattern.offset(i as isize));
        pdf_doc_add_page_content(buf, (len + 1i32) as u32);
        (*gs).linedash.pattern[i as usize] = *pattern.offset(i as isize);
        i += 1
    }
    pdf_doc_add_page_content(b"] \x00" as *const u8 as *const i8, 2_u32);
    len = pdf_sprint_length(buf, offset);
    pdf_doc_add_page_content(buf, len as u32);
    pdf_doc_add_page_content(b" d\x00" as *const u8 as *const i8, 2_u32);
    0i32
}
/* ZSYUEDVEDEOF */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_clip() -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    pdf_dev__flushpath(cpa, 'W' as i32 as i8, 0i32, 0i32)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_eoclip() -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    pdf_dev__flushpath(cpa, 'W' as i32 as i8, 1i32, 0i32)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_flushpath(mut p_op: i8, mut fill_rule: i32) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut error: i32 = 0i32;
    /* last arg 'ignore_rule' is only for single object
     * that can be converted to a rect where fill rule
     * is inessential.
     */
    error = pdf_dev__flushpath(cpa, p_op, fill_rule, 1i32);
    pdf_path__clearpath(cpa);
    (*gs).flags &= !(1i32 << 0i32);
    error
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_newpath() -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut p = &mut (*gs).path;
    if p.num_paths > 0_u32 {
        pdf_path__clearpath(p);
    }
    /* The following is required for "newpath" operator in mpost.c. */
    pdf_doc_add_page_content(b" n\x00" as *const u8 as *const i8, 2_u32); /* op: n */
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_moveto(mut x: f64, mut y: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let mut p = pdf_coord::new();
    p.x = x;
    p.y = y;
    return pdf_path__moveto(cpa, cpt, &mut p);
    /* cpt updated */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rmoveto(mut x: f64, mut y: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let p = pdf_coord {
        x: cpt.x + x,
        y: cpt.y + y,
    };
    pdf_path__moveto(cpa, cpt, &p)
    /* cpt updated */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_lineto(mut x: f64, mut y: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let p0 = pdf_coord { x, y };
    pdf_path__lineto(cpa, cpt, &p0)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rlineto(mut x: f64, mut y: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let mut p0 = pdf_coord {
        x: x + cpt.x,
        y: y + cpt.y,
    };
    pdf_path__lineto(cpa, cpt, &p0)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_curveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let p0 = pdf_coord { x: x0, y: y0 };
    let p1 = pdf_coord { x: x1, y: y1 };
    let p2 = pdf_coord { x: x2, y: y2 };
    pdf_path__curveto(cpa, cpt, &p0, &p1, &p2)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_vcurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let cpt_clone = cpt.clone();
    let p0 = pdf_coord { x: x0, y: y0 };
    let p1 = pdf_coord { x: x1, y: y1 };
    pdf_path__curveto(cpa, cpt, &cpt_clone, &p0, &p1)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_ycurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let p0 = pdf_coord { x: x0, y: y0 };
    let p1 = pdf_coord { x: x1, y: y1 };
    pdf_path__curveto(cpa, cpt, &p0, &p1, &p1)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rcurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let p0 = pdf_coord {
        x: x0 + cpt.x,
        y: y0 + cpt.y,
    };
    let p1 = pdf_coord {
        x: x1 + cpt.x,
        y: y1 + cpt.y,
    };
    let p2 = pdf_coord {
        x: x2 + cpt.x,
        y: y2 + cpt.y,
    };
    pdf_path__curveto(cpa, cpt, &p0, &p1, &p2)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_closepath() -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpt = &mut (*gs).cp;
    let mut cpa = &mut (*gs).path;
    pdf_path__closepath(cpa, cpt)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_dtransform(p: &mut pdf_coord, mut M: Option<&pdf_tmatrix>) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM = &mut (*gs).matrix;
    pdf_coord__dtransform(p, if let Some(m) = M { m } else { CTM });
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_idtransform(p: &mut pdf_coord, M: Option<&pdf_tmatrix>) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM = &mut (*gs).matrix;
    pdf_coord__idtransform(p, if let Some(m) = M { m } else { CTM });
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_transform(p: &mut pdf_coord, M: Option<&pdf_tmatrix>) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM = &mut (*gs).matrix;
    pdf_coord__transform(p, if let Some(m) = M { m } else { CTM });
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arc(c_x: f64, c_y: f64, r: f64, a_0: f64, a_1: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let mut c = pdf_coord { x: c_x, y: c_y };
    pdf_path__elliptarc(cpa, cpt, &mut c, r, r, 0.0f64, a_0, a_1, 1i32)
}
/* *negative* arc */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arcn(c_x: f64, c_y: f64, r: f64, a_0: f64, a_1: f64) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let mut c = pdf_coord { x: c_x, y: c_y };
    pdf_path__elliptarc(cpa, cpt, &mut c, r, r, 0.0f64, a_0, a_1, -1i32)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arcx(
    c_x: f64,
    c_y: f64,
    r_x: f64,
    r_y: f64,
    a_0: f64,
    a_1: f64,
    a_d: i32,
    xar: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let mut c = pdf_coord { x: c_x, y: c_y };
    pdf_path__elliptarc(cpa, cpt, &mut c, r_x, r_y, xar, a_0, a_1, a_d)
}
/* Required by Tpic */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_bspline(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> i32 {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa = &mut (*gs).path;
    let mut cpt = &mut (*gs).cp;
    let p1 = pdf_coord {
        x: x0 + 2. * (x1 - x0) / 3.,
        y: y0 + 2. * (y1 - y0) / 3.,
    };
    let p2 = pdf_coord {
        x: x1 + (x2 - x1) / 3.,
        y: y1 + (y2 - y1) / 3.,
    };
    let p3 = pdf_coord { x: x2, y: y2 };
    pdf_path__curveto(cpa, cpt, &p1, &p2, &p3)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rectfill(x: f64, y: f64, w: f64, h: f64) -> i32 {
    let mut r = pdf_rect {
        llx: x,
        lly: y,
        urx: x + w,
        ury: y + h,
    };
    pdf_dev__rectshape(&mut r, None, 'f' as i32 as i8)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rectclip(x: f64, y: f64, w: f64, h: f64) -> i32 {
    let mut r = pdf_rect {
        llx: x,
        lly: y,
        urx: x + w,
        ury: y + h,
    };
    pdf_dev__rectshape(&mut r, None, 'W' as i32 as i8)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rectadd(x: f64, y: f64, w: f64, h: f64) -> i32 {
    let mut r = pdf_rect {
        llx: x,
        lly: y,
        urx: x + w,
        ury: y + h,
    };
    path_added = 1i32;
    pdf_dev__rectshape(&mut r, None, ' ' as i32 as i8)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_fixed_point(mut x: f64, mut y: f64) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    (*gs).pt_fixee.x = x;
    (*gs).pt_fixee.y = y;
}
/* m -> n x m */
/* Path Construction */
/* Path Painting */
/* NULL pointer of M mean apply current transformation */
/* Requires from mpost.c because new MetaPost graphics must initialize
 * the current gstate. */
/* extension */
/* arc direction   */
/* x-axis-rotation */
/* The depth here is the depth of q/Q nesting.
 * We must remember current depth of nesting when starting a page or xform,
 * and must recover until that depth at the end of page/xform.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_get_fixed_point(p: &mut pdf_coord) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    p.x = (*gs).pt_fixee.x;
    p.y = (*gs).pt_fixee.y;
}
