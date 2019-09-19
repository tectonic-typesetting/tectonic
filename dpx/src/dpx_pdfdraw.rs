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

use crate::warn;

use super::dpx_pdfcolor::{
    pdf_color_compare, pdf_color_copycolor, pdf_color_graycolor_new, pdf_color_is_valid,
    pdf_color_to_string, pdf_color_type,
};
use super::dpx_pdfdev::{
    graphics_mode, pdf_dev_get_param, pdf_dev_reset_fonts, pdf_sprint_coord, pdf_sprint_length,
    pdf_sprint_matrix, pdf_sprint_rect,
};
use super::dpx_pdfdoc::pdf_doc_add_page_content;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! { // TODO move to context structure
    static ref gs_stack: Mutex<Vec<pdf_gstate>> = Mutex::new({
        let mut v = vec![];
        v
    });
}

use libc::sprintf;

pub use super::dpx_pdfcolor::pdf_color;

use super::dpx_pdfdev::{pdf_coord, pdf_rect, pdf_tmatrix};

/* Graphics State */
#[derive(Clone)]
#[repr(C)]
pub struct pdf_gstate {
    pub cp: pdf_coord,
    pub matrix: pdf_tmatrix,
    pub strokecolor: pdf_color,
    pub fillcolor: pdf_color,
    pub linedash: LineDash,
    pub linewidth: f64,
    pub linecap: i32,
    pub linejoin: i32,
    pub miterlimit: f64,
    pub flatness: i32,
    pub path: pdf_path,
    pub flags: i32,
    pub pt_fixee: pdf_coord,
}
#[derive(Clone)]
pub struct pdf_path {
    pub path: Vec<pa_elem>, /* cm,  - */
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

impl pdf_path {
    pub fn new() -> Self {
        Self { path: vec![] }
    }
    pub fn len(&self) -> usize {
        self.path.len()
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct pa_elem {
    pub typ: PeType,
    pub p: [pdf_coord; 3],
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct LineDash {
    pub num_dash: i32,
    pub pattern: [f64; 16],
    pub offset: f64,
}

fn pdf_coord__equal(p1: &pdf_coord, p2: &pdf_coord) -> bool {
    ((p1.x - p2.x).abs() < 1e-7) && ((p1.y - p2.y).abs() < 1e-7)
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
    let pdf_coord { x, y } = *p;
    p.x = x * M.a + y * M.c;
    p.y = x * M.b + y * M.d;
    0i32
}
unsafe extern "C" fn pdf_coord__idtransform(p: &mut pdf_coord, M: &pdf_tmatrix) -> i32 {
    let mut W = pdf_tmatrix::new();
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
    *M = W;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PeType {
    MOVETO = 0,
    LINETO = 1,
    CURVETO = 2,
    CURVETO_V = 3,
    CURVETO_Y = 4,
    CLOSEPATH = 5,
    TERMINATE = 6,
}

impl Default for PeType {
    fn default() -> Self {
        PeType::MOVETO
    }
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

fn clear_a_path(p: &mut pdf_path) {
    p.path = vec![];
}
/* start new subpath */
unsafe extern "C" fn pdf_path__moveto(
    pa: &mut pdf_path,
    cp: &mut pdf_coord,
    p0: &pdf_coord,
) -> i32 {
    if !pa.path.is_empty() {
        let len = pa.len();
        let pe = &mut pa.path[len - 1];
        if pe.typ == PeType::MOVETO {
            *cp = *p0;
            pe.p[0] = *cp;
            return 0i32;
        }
    }
    *cp = *p0;
    let pe = pa_elem {
        typ: PeType::MOVETO,
        p: [*cp, pdf_coord::zero(), pdf_coord::zero()],
    };
    pa.path.push(pe);
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
unsafe extern "C" fn pdf_path__next_pe<'a>(
    pa: &'a mut pdf_path,
    cp: &pdf_coord,
) -> &'a mut pa_elem {
    if pa.path.is_empty() {
        let mut pe = pa_elem::default();
        pe.p[0] = *cp;
        pa.path.push(pe);
        pa.path.push(pa_elem::default());
        let len = pa.len();
        return &mut pa.path[len - 1];
    }
    let len = pa.len();
    let mut pe = &mut pa.path[len - 1];
    match pe.typ {
        PeType::MOVETO => {
            pe.p[0] = *cp;
        }
        PeType::LINETO => {
            if !pdf_coord__equal(&pe.p[0], cp) {
                let mut pe = pa_elem::default();
                pe.p[0] = *cp;
                pa.path.push(pe);
            }
        }
        PeType::CURVETO => {
            if !pdf_coord__equal(&pe.p[2], cp) {
                let mut pe = pa_elem::default();
                pe.p[0] = *cp;
                pa.path.push(pe);
            }
        }
        PeType::CURVETO_Y | PeType::CURVETO_V => {
            if !pdf_coord__equal(&pe.p[1], cp) {
                let mut pe = pa_elem::default();
                pe.p[0] = *cp;
                pa.path.push(pe);
            }
        }
        PeType::CLOSEPATH => {
            let mut pe = pa_elem::default();
            pe.p[0] = *cp;
            pa.path.push(pe);
        }
        _ => {}
    }
    pa.path.push(pa_elem::default());
    let len = pa.len();
    return &mut pa.path[len - 1];
}
unsafe extern "C" fn pdf_path__transform(pa: &mut pdf_path, M: &pdf_tmatrix) -> i32 {
    let mut n = 0;
    let mut i = 0;
    while i < pa.len() {
        let pe = &mut pa.path[i];
        n = if pe.typ != PeType::TERMINATE {
            pe.typ.n_pts()
        } else {
            0
        };
        loop {
            let fresh8 = n;
            n += 1;
            if !(fresh8 > 0) {
                break;
            }
            pdf_coord__transform(&mut pe.p[n], M);
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
    if pdf_coord__equal(cp, p0) {
        pe.typ = PeType::CURVETO_V;
        pe.p[0] = *p1;
        *cp = *p2;
        pe.p[1] = *cp;
    } else if pdf_coord__equal(p1, p2) {
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
    let mut p0 = pdf_coord::new(r_x * c, r_y * s);
    pdf_coord__transform(&mut p0, &mut T);
    p0.x += ca.x;
    p0.y += ca.y;
    if pa.path.is_empty() {
        pdf_path__moveto(pa, cp, &mut p0);
    } else if !pdf_coord__equal(cp, &p0) {
        pdf_path__lineto(pa, cp, &mut p0);
        /* add line seg */
    }
    i = 0i32;
    while error == 0 && i < n_c {
        q = a_0 + i as f64 * d_a;
        let (s, c) = q.sin_cos();
        let e0 = pdf_coord::new(c, s);
        let (s, c) = (q + d_a).sin_cos();
        let e1 = pdf_coord::new(c, s);
        /* Condition for tangent vector requirs
         *  d1 = p1 - p0 = f ( sin a, -cos a)
         *  d2 = p2 - p3 = g ( sin b, -cos b)
         * and from symmetry
         *  g^2 = f^2
         */
        /* s.p. *//* e.p. */
        let mut p0 = pdf_coord::new(r_x * e0.x, r_y * e0.y);
        let mut p3 = pdf_coord::new(r_x * e1.x, r_y * e1.y);
        let mut p1 = pdf_coord::new(-b_x * e0.y, b_y * e0.x);
        let mut p2 = pdf_coord::new(b_x * e1.y, -b_y * e1.x);
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
    /* search for start point of the last subpath */
    let pe = pa.path.iter().rev().find(|pe| pe.typ == PeType::MOVETO);

    if let Some(pe) = pe {
        *cp = pe.p[0].clone();
        /* NOTE:
         *  Manually closed path without closepath is not
         *  affected by linejoin. A path with coincidental
         *  starting and ending point is not the same as
         *  'closed' path.
         */
        let mut pe = pa_elem::default();
        pe.typ = PeType::CLOSEPATH;
        pa.path.push(pe);
        0i32
    } else {
        -1i32
    }
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
    if pa.len() == 5 {
        let pe0 = &pa.path[0];
        let pe1 = &pa.path[1];
        let pe2 = &pa.path[2];
        let pe3 = &pa.path[3];
        let pe4 = &pa.path[4];
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
unsafe extern "C" fn pdf_dev__rectshape(r: &pdf_rect, M: Option<&pdf_tmatrix>, opchr: i8) -> i32 {
    let buf = &mut fmt_buf;
    let mut len = 0;
    let mut isclip: i32 = 0i32;
    let mut wd: f64 = 0.;
    let mut ht: f64 = 0.;
    assert!(b"fFsSbBW ".contains(&(opchr as u8)));
    isclip = if opchr == b'W' as i8 || opchr == b' ' as i8 {
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
    buf[len] = b' ' as i8;
    len += 1;
    if isclip == 0 {
        buf[len] = b'q' as i8;
        len += 1;
        if let Some(m) = M {
            buf[len] = b' ' as i8;
            len += 1;
            len += pdf_sprint_matrix(&mut buf[len..], m) as usize;
            buf[len] = b' ' as i8;
            len += 1;
            buf[len] = b'c' as i8;
            len += 1;
            buf[len] = b'm' as i8;
            len += 1;
        }
        buf[len] = b' ' as i8;
        len += 1;
    }
    buf[len] = b'n' as i8;
    len += 1;
    let p = pdf_coord::new(r.llx, r.lly);
    wd = r.urx - r.llx;
    ht = r.ury - r.lly;
    buf[len] = b' ' as i8;
    len += 1;
    len += pdf_sprint_coord(buf[len..].as_mut_ptr(), &p) as usize;
    buf[len] = b' ' as i8;
    len += 1;
    len += pdf_sprint_length(buf[len..].as_mut_ptr(), wd) as usize;
    buf[len] = b' ' as i8;
    len += 1;
    len += pdf_sprint_length(buf[len..].as_mut_ptr(), ht) as usize;
    buf[len] = b' ' as i8;
    len += 1;
    buf[len] = b'r' as i8;
    len += 1;
    buf[len] = b'e' as i8;
    len += 1;
    if opchr != b' ' as i8 {
        buf[len] = b' ' as i8;
        len += 1;
        buf[len] = opchr;
        len += 1;
        buf[len] = b' ' as i8;
        len += 1;
        buf[len] = (if isclip != 0 { b'n' } else { b'Q' }) as i8;
        len += 1;
    }
    pdf_doc_add_page_content(buf.as_mut_ptr(), len as u32);
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
    let mut b: *mut i8 = fmt_buf.as_mut_ptr(); /* height... */
    let mut b_len: i32 = 1024i32; /* op: re */
    let mut r = pdf_rect::new(); /* op: m l c v y h */
    let mut n_seg: i32 = 0; /* default to 1 in PDF */
    let mut len: i32 = 0i32;
    let mut isrect: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    assert!(b"fFsSbBW ".contains(&(opchr as u8)));
    let isclip = if opchr == b'W' as i8 { true } else { false };
    if
    /*pa.num_paths <= 0_u32 &&*/
    path_added == 0i32 {
        return 0i32;
    }
    path_added = 0i32;
    graphics_mode();
    isrect = pdf_path__isarect(pa, ignore_rule);
    if isrect != 0 {
        let pe = &pa.path[0];
        let pe1 = &pa.path[2];
        r.llx = pe.p[0].x;
        r.lly = pe.p[0].y;
        r.urx = pe1.p[0].x - pe.p[0].x;
        r.ury = pe1.p[0].y - pe.p[0].y;
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
        for pe in pa.path.iter_mut() {
            /* op: f F s S b B W f* F* s* S* b* B* W* */
            let n_pts = if pe.typ != PeType::TERMINATE {
                pe.typ.n_pts() as i32
            } else {
                0i32
            };
            for (_j, pt) in (0..n_pts).zip(pe.p.iter_mut()) {
                /* op: m l c v y h */
                let fresh32 = len;
                len = len + 1;
                *b.offset(fresh32 as isize) = ' ' as i32 as i8;
                len += pdf_sprint_coord(b.offset(len as isize), &mut *pt);
            }
            let fresh33 = len;
            len = len + 1;
            *b.offset(fresh33 as isize) = ' ' as i32 as i8;
            let fresh34 = len;
            len = len + 1;
            *b.offset(fresh34 as isize) = if
            /* !pe.is_null() &&*/
            pe.typ != PeType::TERMINATE {
                pe.typ.opchr()
            } else {
                b' ' as i8
            };
            if len + 128i32 > b_len {
                pdf_doc_add_page_content(b, len as u32);
                len = 0i32
            }
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
    if isclip {
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

trait Top<T> {
    fn top(&mut self) -> &mut T;
}

impl<T> Top<T> for Vec<T> {
    fn top(&mut self) -> &mut T {
        let last = self.len() - 1;
        &mut self[last]
    }
}

impl pdf_gstate {
    pub fn init() -> Self {
        Self {
            cp: pdf_coord::zero(),
            matrix: pdf_tmatrix::identity(),
            strokecolor: pdf_color_graycolor_new(0.).unwrap(),
            fillcolor: pdf_color_graycolor_new(0.).unwrap(),
            linedash: LineDash::default(),
            linecap: 0,  // TODO make enum
            linejoin: 0, // TODO make enum
            linewidth: 1.,
            miterlimit: 10.,
            flatness: 1,
            /* Internal variables */
            flags: 0,
            path: pdf_path::new(),
            pt_fixee: pdf_coord::zero(),
        }
    }
}

unsafe extern "C" fn copy_a_gstate(gs1: &mut pdf_gstate, gs2: &pdf_gstate) {
    let mut i: i32 = 0;
    gs1.cp = gs2.cp;
    gs1.matrix = gs2.matrix;
    /* TODO:
     * Path should be linked list and gsave only
     * record starting point within path rather than
     * copying whole path.
     */
    gs1.path = gs2.path.clone(); /* Initial state */
    gs1.linedash.num_dash = gs2.linedash.num_dash;
    i = 0i32;
    while i < gs2.linedash.num_dash {
        gs1.linedash.pattern[i as usize] = gs2.linedash.pattern[i as usize];
        i += 1
    }
    gs1.linedash.offset = gs2.linedash.offset;
    gs1.linecap = gs2.linecap;
    gs1.linejoin = gs2.linejoin;
    gs1.linewidth = gs2.linewidth;
    gs1.miterlimit = gs2.miterlimit;
    gs1.flatness = gs2.flatness;
    pdf_color_copycolor(&mut gs1.fillcolor, &gs2.fillcolor);
    pdf_color_copycolor(&mut gs1.strokecolor, &gs2.strokecolor);
    gs1.pt_fixee.x = gs2.pt_fixee.x;
    gs1.pt_fixee.y = gs2.pt_fixee.y;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_init_gstates() {
    let mut stack = gs_stack.lock().unwrap();
    *stack = vec![];
    let gs = pdf_gstate::init();
    stack.push(gs);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_clear_gstates() {
    let mut stack = gs_stack.lock().unwrap();

    if stack.len() > 1 {
        /* at least 1 elem. */
        warn!("GS stack depth is not zero at the end of the document."); /* op: q */
    }
    *stack = vec![];
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_gsave() -> i32 {
    let mut stack = gs_stack.lock().unwrap();
    let gs0 = stack.top();

    let mut gs1 = pdf_gstate::init();
    copy_a_gstate(&mut gs1, gs0);
    stack.push(gs1);

    pdf_doc_add_page_content(b" q\x00" as *const u8 as *const i8, 2_u32);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_grestore() -> i32 {
    let mut stack = gs_stack.lock().unwrap();
    if stack.len() <= 1 {
        /* Initial state at bottom */
        warn!("Too many grestores."); /* op: Q */
        return -1i32;
    }
    let _gs = stack.pop();
    pdf_doc_add_page_content(b" Q\x00" as *const u8 as *const i8, 2_u32);
    pdf_dev_reset_fonts(0i32);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_push_gstate() -> i32 {
    let mut stack = gs_stack.lock().unwrap();

    let gs0 = pdf_gstate::init();
    stack.push(gs0);

    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_pop_gstate() -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    if gss.len() <= 1 {
        /* Initial state at bottom */
        warn!("Too many grestores.");
        return -1i32;
    }
    let _gs = gss.pop();
    0i32
}
#[no_mangle]
pub extern "C" fn pdf_dev_current_depth() -> usize {
    let stack = gs_stack.lock().unwrap();
    stack.len() - 1
    /* 0 means initial state */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_grestore_to(mut depth: usize) {
    let mut gss = gs_stack.lock().unwrap(); /* op: Q */
    assert!(depth >= 0);
    if gss.len() > depth + 1 {
        warn!("Closing pending transformations at end of page/XObject.");
    }
    while gss.len() > depth + 1 {
        pdf_doc_add_page_content(b" Q\x00" as *const u8 as *const i8, 2_u32);
        let _gs = gss.pop();
    }
    pdf_dev_reset_fonts(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_currentpoint(p: &mut pdf_coord) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    *p = gs.cp.clone();
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_currentmatrix(M: &mut pdf_tmatrix) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    *M = gs.matrix.clone();
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
    let mut stack = gs_stack.lock().unwrap();
    let mut len: i32 = 0;
    let mut gs = stack.top();
    let current = if mask as i32 != 0 {
        &mut gs.fillcolor
    } else {
        &mut gs.strokecolor
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let CTM = &mut gs.matrix;
    let mut W = pdf_tmatrix::new();
    let mut buf = &mut fmt_buf;
    let mut len = 0;
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
        buf[len] = b' ' as i8;
        len += 1;
        len += pdf_sprint_matrix(&mut buf[len..], M) as usize;
        buf[len] = b' ' as i8;
        len += 1;
        buf[len] = b'c' as i8;
        len += 1;
        buf[len] = b'm' as i8;
        len += 1;
        pdf_doc_add_page_content(buf.as_mut_ptr(), len as u32);
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
    let mut gss = gs_stack.lock().unwrap(); /* op: M */
    let gs = gss.top(); /* op: J */
    let mut len: i32 = 0i32; /* op: j */
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr(); /* op: w */
    if gs.miterlimit != mlimit {
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
        gs.miterlimit = mlimit
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinecap(mut capstyle: i32) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if gs.linecap != capstyle {
        len = sprintf(buf, b" %d J\x00" as *const u8 as *const i8, capstyle);
        pdf_doc_add_page_content(buf, len as u32);
        gs.linecap = capstyle
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinejoin(mut joinstyle: i32) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if gs.linejoin != joinstyle {
        len = sprintf(buf, b" %d j\x00" as *const u8 as *const i8, joinstyle);
        pdf_doc_add_page_content(buf, len as u32);
        gs.linejoin = joinstyle
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinewidth(mut width: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if gs.linewidth != width {
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
        gs.linewidth = width
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setdash(
    mut count: i32,
    mut pattern: *mut f64,
    mut offset: f64,
) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let mut len: i32 = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut i: i32 = 0;
    gs.linedash.num_dash = count;
    gs.linedash.offset = offset;
    pdf_doc_add_page_content(b" [\x00" as *const u8 as *const i8, 2_u32);
    i = 0i32;
    while i < count {
        *buf.offset(0) = ' ' as i32 as i8;
        len = pdf_sprint_length(buf.offset(1), *pattern.offset(i as isize));
        pdf_doc_add_page_content(buf, (len + 1i32) as u32);
        gs.linedash.pattern[i as usize] = *pattern.offset(i as isize);
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    pdf_dev__flushpath(cpa, 'W' as i32 as i8, 0i32, 0i32)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_eoclip() -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    pdf_dev__flushpath(cpa, 'W' as i32 as i8, 1i32, 0i32)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_flushpath(mut p_op: i8, mut fill_rule: i32) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let mut error: i32 = 0i32;
    /* last arg 'ignore_rule' is only for single object
     * that can be converted to a rect where fill rule
     * is inessential.
     */
    error = pdf_dev__flushpath(cpa, p_op, fill_rule, 1i32);
    cpa.path.clear();
    gs.flags &= !(1i32 << 0i32);
    error
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_newpath() -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let p = &mut gs.path;
    if !p.path.is_empty() {
        p.path.clear();
    }
    /* The following is required for "newpath" operator in mpost.c. */
    pdf_doc_add_page_content(b" n\x00" as *const u8 as *const i8, 2_u32); /* op: n */
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_moveto(mut x: f64, mut y: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p = pdf_coord::new(x, y);
    pdf_path__moveto(cpa, cpt, &p)
    /* cpt updated */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rmoveto(mut x: f64, mut y: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p = pdf_coord::new(cpt.x + x, cpt.y + y);
    pdf_path__moveto(cpa, cpt, &p)
    /* cpt updated */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_lineto(mut x: f64, mut y: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p0 = pdf_coord::new(x, y);
    pdf_path__lineto(cpa, cpt, &p0)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rlineto(mut x: f64, mut y: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p0 = pdf_coord::new(x + cpt.x, y + cpt.y);
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p0 = pdf_coord::new(x0, y0);
    let p1 = pdf_coord::new(x1, y1);
    let p2 = pdf_coord::new(x2, y2);
    pdf_path__curveto(cpa, cpt, &p0, &p1, &p2)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_vcurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let cpt_clone = cpt.clone();
    let p0 = pdf_coord::new(x0, y0);
    let p1 = pdf_coord::new(x1, y1);
    pdf_path__curveto(cpa, cpt, &cpt_clone, &p0, &p1)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_ycurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p0 = pdf_coord::new(x0, y0);
    let p1 = pdf_coord::new(x1, y1);
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p0 = pdf_coord::new(x0 + cpt.x, y0 + cpt.y);
    let p1 = pdf_coord::new(x1 + cpt.x, y1 + cpt.y);
    let p2 = pdf_coord::new(x2 + cpt.x, y2 + cpt.y);
    pdf_path__curveto(cpa, cpt, &p0, &p1, &p2)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_closepath() -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpt = &mut gs.cp;
    let cpa = &mut gs.path;
    pdf_path__closepath(cpa, cpt)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_dtransform(p: &mut pdf_coord, mut M: Option<&pdf_tmatrix>) {
    if let Some(m) = M {
        pdf_coord__dtransform(p, m);
    } else {
        let mut gss = gs_stack.lock().unwrap();
        let gs = gss.top();
        pdf_coord__dtransform(p, &mut gs.matrix);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_idtransform(p: &mut pdf_coord, M: Option<&pdf_tmatrix>) {
    if let Some(m) = M {
        pdf_coord__idtransform(p, m);
    } else {
        let mut gss = gs_stack.lock().unwrap();
        let gs = gss.top();
        pdf_coord__idtransform(p, &mut gs.matrix);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_transform(p: &mut pdf_coord, M: Option<&pdf_tmatrix>) {
    if let Some(m) = M {
        pdf_coord__transform(p, m);
    } else {
        let mut gss = gs_stack.lock().unwrap();
        let gs = gss.top();
        pdf_coord__transform(p, &mut gs.matrix);
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arc(c_x: f64, c_y: f64, r: f64, a_0: f64, a_1: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let mut c = pdf_coord::new(c_x, c_y);
    pdf_path__elliptarc(cpa, cpt, &mut c, r, r, 0.0f64, a_0, a_1, 1i32)
}
/* *negative* arc */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arcn(c_x: f64, c_y: f64, r: f64, a_0: f64, a_1: f64) -> i32 {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let mut c = pdf_coord::new(c_x, c_y);
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let mut c = pdf_coord::new(c_x, c_y);
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    let cpa = &mut gs.path;
    let cpt = &mut gs.cp;
    let p1 = pdf_coord::new(x0 + 2. * (x1 - x0) / 3., y0 + 2. * (y1 - y0) / 3.);
    let p2 = pdf_coord::new(x1 + (x2 - x1) / 3., y1 + (y2 - y1) / 3.);
    let p3 = pdf_coord::new(x2, y2);
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
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    gs.pt_fixee.x = x;
    gs.pt_fixee.y = y;
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
pub fn pdf_dev_get_fixed_point() -> pdf_coord {
    let mut gss = gs_stack.lock().unwrap();
    let gs = gss.top();
    gs.pt_fixee.clone()
}
