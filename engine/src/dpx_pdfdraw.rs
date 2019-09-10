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
    fn cos(_: f64) -> f64;
    #[no_mangle]
    fn sin(_: f64) -> f64;
    #[no_mangle]
    fn fabs(_: f64) -> f64;
    #[no_mangle]
    fn graphics_mode();
    #[no_mangle]
    fn pdf_dev_reset_fonts(newpage: libc::c_int);
    #[no_mangle]
    fn pdf_dev_get_param(param_type: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pdf_sprint_length(buf: *mut i8, value: f64) -> libc::c_int;
    #[no_mangle]
    fn pdf_sprint_coord(buf: *mut i8, p: *const pdf_coord) -> libc::c_int;
    #[no_mangle]
    fn pdf_sprint_rect(buf: *mut i8, p: *const pdf_rect) -> libc::c_int;
    #[no_mangle]
    fn pdf_sprint_matrix(buf: *mut i8, p: *const pdf_tmatrix) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: libc::c_uint,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_graycolor(color: *mut pdf_color, g: f64) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    #[no_mangle]
    fn pdf_color_type(color: *const pdf_color) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_compare(color1: *const pdf_color, color2: *const pdf_color) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_to_string(
        color: *const pdf_color,
        buffer: *mut i8,
        mask: i8,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_color_is_valid(color: *const pdf_color) -> bool;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
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
    fn pdf_doc_add_page_content(buffer: *const i8, length: libc::c_uint);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub num_components: libc::c_int,
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
pub struct pdf_coord {
    pub x: f64,
    pub y: f64,
}
/* Graphics State */
pub type pdf_gstate = pdf_gstate_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_gstate_ {
    pub cp: pdf_coord,
    pub matrix: pdf_tmatrix,
    pub strokecolor: pdf_color,
    pub fillcolor: pdf_color,
    pub linedash: C2RustUnnamed,
    pub linewidth: f64,
    pub linecap: libc::c_int,
    pub linejoin: libc::c_int,
    pub miterlimit: f64,
    pub flatness: libc::c_int,
    pub path: pdf_path,
    pub flags: libc::c_int,
    pub pt_fixee: pdf_coord,
}
pub type pdf_path = pdf_path_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_path_ {
    pub num_paths: libc::c_uint,
    pub max_paths: libc::c_uint,
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
pub type pa_elem = pa_elem_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_elem_ {
    pub type_0: libc::c_int,
    pub p: [pdf_coord; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub num_dash: libc::c_int,
    pub pattern: [f64; 16],
    pub offset: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m_stack {
    pub size: libc::c_int,
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
    pub n_pts: libc::c_int,
    pub strkey: *const i8,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn inversematrix(
    mut W: *mut pdf_tmatrix,
    mut M: *const pdf_tmatrix,
) -> libc::c_int {
    let mut det: f64 = 0.;
    det = (*M).a * (*M).d - (*M).b * (*M).c;
    if fabs(det) < 2.5e-16f64 {
        dpx_warning(
            b"Inverting matrix with zero determinant...\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*W).a = (*M).d / det;
    (*W).b = -(*M).b / det;
    (*W).c = -(*M).c / det;
    (*W).d = (*M).a / det;
    (*W).e = (*M).c * (*M).f - (*M).d * (*M).e;
    (*W).f = (*M).b * (*M).e - (*M).a * (*M).f;
    return 0i32;
}
unsafe extern "C" fn pdf_coord__equal(
    mut p1: *const pdf_coord,
    mut p2: *const pdf_coord,
) -> libc::c_int {
    if fabs((*p1).x - (*p2).x) < 1.0e-7f64 && fabs((*p1).y - (*p2).y) < 1.0e-7f64 {
        return 1i32;
    }
    return 0i32;
}
unsafe extern "C" fn pdf_coord__transform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) -> libc::c_int {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    x = (*p).x;
    y = (*p).y;
    (*p).x = x * (*M).a + y * (*M).c + (*M).e;
    (*p).y = x * (*M).b + y * (*M).d + (*M).f;
    return 0i32;
}
unsafe extern "C" fn pdf_coord__dtransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) -> libc::c_int {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    x = (*p).x;
    y = (*p).y;
    (*p).x = x * (*M).a + y * (*M).c;
    (*p).y = x * (*M).b + y * (*M).d;
    return 0i32;
}
unsafe extern "C" fn pdf_coord__idtransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) -> libc::c_int {
    let mut W: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut error: libc::c_int = 0;
    error = inversematrix(&mut W, M);
    if error != 0 {
        return error;
    }
    x = (*p).x;
    y = (*p).y;
    (*p).x = x * W.a + y * W.c;
    (*p).y = x * W.b + y * W.d;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_invertmatrix(mut M: *mut pdf_tmatrix) {
    let mut W: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut det: f64 = 0.;
    if !M.is_null() {
    } else {
        __assert_fail(
            b"M\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            137i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
                b"void pdf_invertmatrix(pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    det = (*M).a * (*M).d - (*M).b * (*M).c;
    if fabs(det) < 2.5e-16f64 {
        dpx_warning(
            b"Inverting matrix with zero determinant...\x00" as *const u8 as *const i8,
        );
        W.a = 1.0f64;
        W.c = 0.0f64;
        W.b = 0.0f64;
        W.d = 1.0f64;
        W.e = 0.0f64;
        W.f = 0.0f64
    } else {
        W.a = (*M).d / det;
        W.b = -(*M).b / det;
        W.c = -(*M).c / det;
        W.d = (*M).a / det;
        W.e = (*M).c * (*M).f - (*M).d * (*M).e;
        W.f = (*M).b * (*M).e - (*M).a * (*M).f;
        W.e /= det;
        W.f /= det
    }
    (*M).a = W.a;
    (*M).b = W.b;
    (*M).c = W.c;
    (*M).d = W.d;
    (*M).e = W.e;
    (*M).f = W.f;
}
static mut petypes: [C2RustUnnamed_0; 7] = [
    {
        let mut init = C2RustUnnamed_0 {
            opchr: 'm' as i32 as i8,
            n_pts: 1i32,
            strkey: b"moveto\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            opchr: 'l' as i32 as i8,
            n_pts: 1i32,
            strkey: b"lineto\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            opchr: 'c' as i32 as i8,
            n_pts: 3i32,
            strkey: b"curveto\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            opchr: 'v' as i32 as i8,
            n_pts: 2i32,
            strkey: b"vcurveto\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            opchr: 'y' as i32 as i8,
            n_pts: 2i32,
            strkey: b"ycurveto\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            opchr: 'h' as i32 as i8,
            n_pts: 0i32,
            strkey: b"closepath\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            opchr: ' ' as i32 as i8,
            n_pts: 0i32,
            strkey: 0 as *const i8,
        };
        init
    },
];
static mut fmt_buf: [i8; 1024] = [0; 1024];
unsafe extern "C" fn init_a_path(mut p: *mut pdf_path) {
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            212i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[i8; 29]>(
                b"void init_a_path(pdf_path *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*p).num_paths = 0i32 as libc::c_uint;
    (*p).max_paths = 0i32 as libc::c_uint;
    (*p).path = 0 as *mut pa_elem;
}
unsafe extern "C" fn pdf_path__clearpath(mut p: *mut pdf_path) {
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            224i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
                b"void pdf_path__clearpath(pdf_path *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*p).num_paths = 0i32 as libc::c_uint;
}
unsafe extern "C" fn pdf_path__growpath(
    mut p: *mut pdf_path,
    mut max_pe: libc::c_uint,
) -> libc::c_int {
    if max_pe < (*p).max_paths {
        return 0i32;
    }
    (*p).max_paths = if (*p).max_paths.wrapping_add(8i32 as libc::c_uint) > max_pe {
        (*p).max_paths.wrapping_add(8i32 as libc::c_uint)
    } else {
        max_pe
    };
    (*p).path = renew(
        (*p).path as *mut libc::c_void,
        ((*p).max_paths as u64)
            .wrapping_mul(::std::mem::size_of::<pa_elem>() as u64) as u32,
    ) as *mut pa_elem;
    return 0i32;
}
unsafe extern "C" fn clear_a_path(mut p: *mut pdf_path) {
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            246i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[i8; 30]>(
                b"void clear_a_path(pdf_path *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*p).path = mfree((*p).path as *mut libc::c_void) as *mut pa_elem;
    (*p).num_paths = 0i32 as libc::c_uint;
    (*p).max_paths = 0i32 as libc::c_uint;
}
unsafe extern "C" fn pdf_path__copypath(
    mut p1: *mut pdf_path,
    mut p0: *const pdf_path,
) -> libc::c_int {
    let mut pe0: *mut pa_elem = 0 as *mut pa_elem;
    let mut pe1: *mut pa_elem = 0 as *mut pa_elem;
    let mut i: libc::c_uint = 0;
    pdf_path__growpath(p1, (*p0).num_paths);
    i = 0i32 as libc::c_uint;
    while i < (*p0).num_paths {
        pe1 = &mut *(*p1).path.offset(i as isize) as *mut pa_elem;
        pe0 = &mut *(*p0).path.offset(i as isize) as *mut pa_elem;
        /* FIXME */
        (*pe1).type_0 = (*pe0).type_0;
        (*pe1).p[0].x = (*pe0).p[0].x;
        (*pe1).p[0].y = (*pe0).p[0].y;
        (*pe1).p[1].x = (*pe0).p[1].x;
        (*pe1).p[1].y = (*pe0).p[1].y;
        (*pe1).p[2].x = (*pe0).p[2].x;
        (*pe1).p[2].y = (*pe0).p[2].y;
        i = i.wrapping_add(1)
    }
    (*p1).num_paths = (*p0).num_paths;
    return 0i32;
}
/* start new subpath */
unsafe extern "C" fn pdf_path__moveto(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut p0: *const pdf_coord,
) -> libc::c_int {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    pdf_path__growpath(pa, (*pa).num_paths.wrapping_add(1i32 as libc::c_uint));
    if (*pa).num_paths > 0i32 as libc::c_uint {
        pe = &mut *(*pa)
            .path
            .offset((*pa).num_paths.wrapping_sub(1i32 as libc::c_uint) as isize)
            as *mut pa_elem;
        if (*pe).type_0 == 0i32 {
            (*cp).x = (*p0).x;
            (*pe).p[0].x = (*cp).x;
            (*cp).y = (*p0).y;
            (*pe).p[0].y = (*cp).y;
            return 0i32;
        }
    }
    let fresh0 = (*pa).num_paths;
    (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
    pe = &mut *(*pa).path.offset(fresh0 as isize) as *mut pa_elem;
    (*pe).type_0 = 0i32;
    (*cp).x = (*p0).x;
    (*pe).p[0].x = (*cp).x;
    (*cp).y = (*p0).y;
    (*pe).p[0].y = (*cp).y;
    return 0i32;
}
/* Do 'compression' of path while adding new path elements.
 * Sequantial moveto command will be replaced with a
 * single moveto. If cp is not equal to the last point in pa,
 * then moveto is inserted (starting new subpath).
 * FIXME:
 * 'moveto' must be used to enforce starting new path.
 * This affects how 'closepath' is treated.
 */
unsafe extern "C" fn pdf_path__next_pe(
    mut pa: *mut pdf_path,
    mut cp: *const pdf_coord,
) -> *mut pa_elem {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    pdf_path__growpath(pa, (*pa).num_paths.wrapping_add(2i32 as libc::c_uint));
    if (*pa).num_paths == 0i32 as libc::c_uint {
        let fresh1 = (*pa).num_paths;
        (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
        pe = &mut *(*pa).path.offset(fresh1 as isize) as *mut pa_elem;
        (*pe).type_0 = 0i32;
        (*pe).p[0].x = (*cp).x;
        (*pe).p[0].y = (*cp).y;
        let fresh2 = (*pa).num_paths;
        (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
        return &mut *(*pa).path.offset(fresh2 as isize) as *mut pa_elem;
    }
    pe = &mut *(*pa)
        .path
        .offset((*pa).num_paths.wrapping_sub(1i32 as libc::c_uint) as isize)
        as *mut pa_elem;
    match (*pe).type_0 {
        0 => {
            (*pe).p[0].x = (*cp).x;
            (*pe).p[0].y = (*cp).y
        }
        1 => {
            if pdf_coord__equal(&mut *(*pe).p.as_mut_ptr().offset(0), cp) == 0 {
                let fresh3 = (*pa).num_paths;
                (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
                pe = &mut *(*pa).path.offset(fresh3 as isize) as *mut pa_elem;
                (*pe).type_0 = 0i32;
                (*pe).p[0].x = (*cp).x;
                (*pe).p[0].y = (*cp).y
            }
        }
        2 => {
            if pdf_coord__equal(&mut *(*pe).p.as_mut_ptr().offset(2), cp) == 0 {
                let fresh4 = (*pa).num_paths;
                (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
                pe = &mut *(*pa).path.offset(fresh4 as isize) as *mut pa_elem;
                (*pe).type_0 = 0i32;
                (*pe).p[0].x = (*cp).x;
                (*pe).p[0].y = (*cp).y
            }
        }
        4 | 3 => {
            if pdf_coord__equal(&mut *(*pe).p.as_mut_ptr().offset(1), cp) == 0 {
                let fresh5 = (*pa).num_paths;
                (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
                pe = &mut *(*pa).path.offset(fresh5 as isize) as *mut pa_elem;
                (*pe).type_0 = 0i32;
                (*pe).p[0].x = (*cp).x;
                (*pe).p[0].y = (*cp).y
            }
        }
        5 => {
            let fresh6 = (*pa).num_paths;
            (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
            pe = &mut *(*pa).path.offset(fresh6 as isize) as *mut pa_elem;
            (*pe).type_0 = 0i32;
            (*pe).p[0].x = (*cp).x;
            (*pe).p[0].y = (*cp).y
        }
        _ => {}
    }
    let fresh7 = (*pa).num_paths;
    (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
    return &mut *(*pa).path.offset(fresh7 as isize) as *mut pa_elem;
}
unsafe extern "C" fn pdf_path__transform(
    mut pa: *mut pdf_path,
    mut M: *const pdf_tmatrix,
) -> libc::c_int {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    let mut n: libc::c_uint = 0i32 as libc::c_uint;
    let mut i: libc::c_uint = 0;
    if !pa.is_null() && !M.is_null() {
    } else {
        __assert_fail(
            b"pa && M\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            376i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                b"int pdf_path__transform(pdf_path *, const pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32 as libc::c_uint;
    while i < (*pa).num_paths {
        pe = &mut *(*pa).path.offset(i as isize) as *mut pa_elem;
        n = (if !pe.is_null() && (*pe).type_0 > -1i32 && (*pe).type_0 < 6i32 {
            petypes[(*pe).type_0 as usize].n_pts
        } else {
            0i32
        }) as libc::c_uint;
        loop {
            let fresh8 = n;
            n = n.wrapping_sub(1);
            if !(fresh8 > 0i32 as libc::c_uint) {
                break;
            }
            pdf_coord__transform(&mut *(*pe).p.as_mut_ptr().offset(n as isize), M);
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
/* Path Construction */
unsafe extern "C" fn pdf_path__lineto(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut p0: *const pdf_coord,
) -> libc::c_int {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    pe = pdf_path__next_pe(pa, cp);
    (*pe).type_0 = 1i32;
    (*cp).x = (*p0).x;
    (*pe).p[0].x = (*cp).x;
    (*cp).y = (*p0).y;
    (*pe).p[0].y = (*cp).y;
    return 0i32;
}
unsafe extern "C" fn pdf_path__curveto(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut p0: *const pdf_coord,
    mut p1: *const pdf_coord,
    mut p2: *const pdf_coord,
) -> libc::c_int {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    pe = pdf_path__next_pe(pa, cp);
    if pdf_coord__equal(cp, p0) != 0 {
        (*pe).type_0 = 3i32;
        (*pe).p[0].x = (*p1).x;
        (*pe).p[0].y = (*p1).y;
        (*cp).x = (*p2).x;
        (*pe).p[1].x = (*cp).x;
        (*cp).y = (*p2).y;
        (*pe).p[1].y = (*cp).y
    } else if pdf_coord__equal(p1, p2) != 0 {
        (*pe).type_0 = 4i32;
        (*pe).p[0].x = (*p0).x;
        (*pe).p[0].y = (*p0).y;
        (*cp).x = (*p1).x;
        (*pe).p[1].x = (*cp).x;
        (*cp).y = (*p1).y;
        (*pe).p[1].y = (*cp).y
    } else {
        (*pe).type_0 = 2i32;
        (*pe).p[0].x = (*p0).x;
        (*pe).p[0].y = (*p0).y;
        (*pe).p[1].x = (*p1).x;
        (*pe).p[1].y = (*p1).y;
        (*cp).x = (*p2).x;
        (*pe).p[2].x = (*cp).x;
        (*cp).y = (*p2).y;
        (*pe).p[2].y = (*cp).y
    }
    return 0i32;
}
/* This isn't specified as cp to somewhere. */
unsafe extern "C" fn pdf_path__elliptarc(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut ca: *const pdf_coord,
    mut r_x: f64,
    mut r_y: f64,
    mut xar: f64,
    mut a_0: f64,
    mut a_1: f64,
    mut a_d: libc::c_int,
) -> libc::c_int
/* arc orientation        */ {
    let mut b: f64 = 0.; /* number of segments */
    let mut b_x: f64 = 0.;
    let mut b_y: f64 = 0.;
    let mut d_a: f64 = 0.;
    let mut q: f64 = 0.;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut e0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut e1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut T: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut n_c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    if fabs(r_x) < 2.5e-16f64 || fabs(r_y) < 2.5e-16f64 {
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
    while fabs(d_a) > 90.0f64 * n_c as f64 {
        n_c += 1
    }
    d_a /= n_c as f64;
    if fabs(d_a) < 2.5e-16f64 {
        return -1i32;
    }
    a_0 *= 3.14159265358979323846f64 / 180.0f64;
    a_1 *= 3.14159265358979323846f64 / 180.0f64;
    d_a *= 3.14159265358979323846f64 / 180.0f64;
    xar *= 3.14159265358979323846f64 / 180.0f64;
    T.a = cos(xar);
    T.c = -sin(xar);
    T.b = -T.c;
    T.d = T.a;
    T.e = 0.0f64;
    T.f = 0.0f64;
    /* A parameter that controls cb-curve (off-curve) points */
    b = 4.0f64 * (1.0f64 - cos(0.5f64 * d_a)) / (3.0f64 * sin(0.5f64 * d_a));
    b_x = r_x * b;
    b_y = r_y * b;
    p0.x = r_x * cos(a_0);
    p0.y = r_y * sin(a_0);
    pdf_coord__transform(&mut p0, &mut T);
    p0.x += (*ca).x;
    p0.y += (*ca).y;
    if (*pa).num_paths == 0i32 as libc::c_uint {
        pdf_path__moveto(pa, cp, &mut p0);
    } else if pdf_coord__equal(cp, &mut p0) == 0 {
        pdf_path__lineto(pa, cp, &mut p0);
        /* add line seg */
    }
    i = 0i32;
    while error == 0 && i < n_c {
        q = a_0 + i as f64 * d_a;
        e0.x = cos(q);
        e0.y = sin(q);
        e1.x = cos(q + d_a);
        e1.y = sin(q + d_a);
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
        p0.x += (*ca).x;
        p0.y += (*ca).y;
        p3.x += (*ca).x;
        p3.y += (*ca).y;
        p1.x += p0.x;
        p1.y += p0.y;
        p2.x += p3.x;
        p2.y += p3.y;
        error = pdf_path__curveto(pa, &mut p0, &mut p1, &mut p2, &mut p3);
        (*cp).x = p3.x;
        (*cp).y = p3.y;
        i += 1
    }
    return error;
}
unsafe extern "C" fn pdf_path__closepath(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
) -> libc::c_int
/* no arg */ {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem;
    let mut i: libc::c_int = 0;
    /* search for start point of the last subpath */
    i = (*pa).num_paths.wrapping_sub(1i32 as libc::c_uint) as libc::c_int; /* No path or no start point(!) */
    while i >= 0i32 {
        pe = &mut *(*pa).path.offset(i as isize) as *mut pa_elem;
        if (*pe).type_0 == 0i32 {
            break;
        }
        i -= 1
    }
    if pe.is_null() || i < 0i32 {
        return -1i32;
    }
    (*cp).x = (*pe).p[0].x;
    (*cp).y = (*pe).p[0].y;
    pdf_path__growpath(pa, (*pa).num_paths.wrapping_add(1i32 as libc::c_uint));
    /* NOTE:
     *  Manually closed path without closepath is not
     *  affected by linejoin. A path with coincidental
     *  starting and ending point is not the same as
     *  'closed' path.
     */
    let fresh9 = (*pa).num_paths;
    (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
    pe = &mut *(*pa).path.offset(fresh9 as isize) as *mut pa_elem;
    (*pe).type_0 = 5i32;
    return 0i32;
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
unsafe extern "C" fn pdf_path__isarect(
    mut pa: *mut pdf_path,
    mut f_ir: libc::c_int,
) -> libc::c_int
/* fill-rule is ignorable */ {
    let mut pe0: *mut pa_elem = 0 as *mut pa_elem;
    let mut pe1: *mut pa_elem = 0 as *mut pa_elem;
    let mut pe2: *mut pa_elem = 0 as *mut pa_elem;
    let mut pe3: *mut pa_elem = 0 as *mut pa_elem;
    let mut pe4: *mut pa_elem = 0 as *mut pa_elem;
    if (*pa).num_paths == 5i32 as libc::c_uint {
        pe0 = &mut *(*pa).path.offset(0) as *mut pa_elem;
        pe1 = &mut *(*pa).path.offset(1) as *mut pa_elem;
        pe2 = &mut *(*pa).path.offset(2) as *mut pa_elem;
        pe3 = &mut *(*pa).path.offset(3) as *mut pa_elem;
        pe4 = &mut *(*pa).path.offset(4) as *mut pa_elem;
        if (*pe0).type_0 == 0i32
            && (*pe1).type_0 == 1i32
            && (*pe2).type_0 == 1i32
            && (*pe3).type_0 == 1i32
            && (*pe4).type_0 == 5i32
        {
            if (*pe1).p[0].y - (*pe0).p[0].y == 0i32 as f64
                && (*pe2).p[0].x - (*pe1).p[0].x == 0i32 as f64
                && (*pe3).p[0].y - (*pe2).p[0].y == 0i32 as f64
            {
                if (*pe1).p[0].x - (*pe0).p[0].x == (*pe2).p[0].x - (*pe3).p[0].x {
                    return 1i32;
                }
            /* Winding number is different but ignore it here. */
            } else if f_ir != 0
                && (*pe1).p[0].x - (*pe0).p[0].x == 0i32 as f64
                && (*pe2).p[0].y - (*pe1).p[0].y == 0i32 as f64
                && (*pe3).p[0].x - (*pe2).p[0].x == 0i32 as f64
            {
                if (*pe1).p[0].y - (*pe0).p[0].y == (*pe2).p[0].y - (*pe3).p[0].y {
                    return 1i32;
                }
            }
        }
    }
    return 0i32;
}
/* Path Painting */
/* F is obsoleted */
unsafe extern "C" fn INVERTIBLE_MATRIX(mut M: *const pdf_tmatrix) -> libc::c_int {
    if fabs((*M).a * (*M).d - (*M).b * (*M).c) < 2.5e-16f64 {
        dpx_warning(
            b"Transformation matrix not invertible.\x00" as *const u8 as *const i8,
        );
        dpx_warning(
            b"--- M = [%g %g %g %g %g %g]\x00" as *const u8 as *const i8,
            (*M).a,
            (*M).b,
            (*M).c,
            (*M).d,
            (*M).e,
            (*M).f,
        );
        return -1i32;
    }
    return 0i32;
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
    mut r: *const pdf_rect,
    mut M: *const pdf_tmatrix,
    mut opchr: i8,
) -> libc::c_int {
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut len: libc::c_int = 0i32;
    let mut isclip: libc::c_int = 0i32;
    let mut p: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut wd: f64 = 0.;
    let mut ht: f64 = 0.;
    if !r.is_null()
        && (opchr as libc::c_int == 'f' as i32
            || opchr as libc::c_int == 'F' as i32
            || opchr as libc::c_int == 's' as i32
            || opchr as libc::c_int == 'S' as i32
            || opchr as libc::c_int == 'b' as i32
            || opchr as libc::c_int == 'B' as i32
            || opchr as libc::c_int == 'W' as i32
            || opchr as libc::c_int == ' ' as i32)
    {
    } else {
        __assert_fail(
            b"r && PT_OP_VALID(opchr)\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            667i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 68], &[i8; 68]>(
                b"int pdf_dev__rectshape(const pdf_rect *, const pdf_tmatrix *, char)\x00",
            ))
            .as_ptr(),
        );
    }
    isclip = if opchr as libc::c_int == 'W' as i32 || opchr as libc::c_int == ' ' as i32 {
        1i32
    } else {
        0i32
    };
    /* disallow matrix for clipping.
     * q ... clip Q does nothing and
     * n M cm ... clip n alter CTM.
     */
    if !M.is_null() && (isclip != 0 || INVERTIBLE_MATRIX(M) == 0) {
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
        if !M.is_null() {
            let fresh12 = len;
            len = len + 1;
            *buf.offset(fresh12 as isize) = ' ' as i32 as i8;
            len += pdf_sprint_matrix(buf.offset(len as isize), M);
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
    p.x = (*r).llx;
    p.y = (*r).lly;
    wd = (*r).urx - (*r).llx;
    ht = (*r).ury - (*r).lly;
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
    if opchr as libc::c_int != ' ' as i32 {
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
        *buf.offset(fresh27 as isize) =
            (if isclip != 0 { 'n' as i32 } else { 'Q' as i32 }) as i8
    }
    pdf_doc_add_page_content(buf, len as libc::c_uint);
    return 0i32;
}
static mut path_added: libc::c_int = 0i32;
/* FIXME */
unsafe extern "C" fn pdf_dev__flushpath(
    mut pa: *mut pdf_path,
    mut opchr: i8,
    mut rule: libc::c_int,
    mut ignore_rule: libc::c_int,
) -> libc::c_int {
    let mut pe: *mut pa_elem = 0 as *mut pa_elem; /* FIXME */
    let mut pe1: *mut pa_elem = 0 as *mut pa_elem; /* width...  */
    let mut b: *mut i8 = fmt_buf.as_mut_ptr(); /* height... */
    let mut b_len: libc::c_int = 1024i32; /* op: re */
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    }; /* op: m l c v y h */
    let mut pt: *mut pdf_coord = 0 as *mut pdf_coord; /* op: m l c v y h */
    let mut n_pts: libc::c_int = 0; /* op: f F s S b B W f* F* s* S* b* B* W* */
    let mut n_seg: libc::c_int = 0; /* default to 1 in PDF */
    let mut len: libc::c_int = 0i32;
    let mut isclip: libc::c_int = 0i32;
    let mut isrect: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !pa.is_null()
        && (opchr as libc::c_int == 'f' as i32
            || opchr as libc::c_int == 'F' as i32
            || opchr as libc::c_int == 's' as i32
            || opchr as libc::c_int == 'S' as i32
            || opchr as libc::c_int == 'b' as i32
            || opchr as libc::c_int == 'B' as i32
            || opchr as libc::c_int == 'W' as i32
            || opchr as libc::c_int == ' ' as i32)
    {
    } else {
        __assert_fail(
            b"pa && PT_OP_VALID(opchr)\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            738i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[i8; 51]>(
                b"int pdf_dev__flushpath(pdf_path *, char, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    isclip = if opchr as libc::c_int == 'W' as i32 {
        1i32
    } else {
        0i32
    };
    if (*pa).num_paths <= 0i32 as libc::c_uint && path_added == 0i32 {
        return 0i32;
    }
    path_added = 0i32;
    graphics_mode();
    isrect = pdf_path__isarect(pa, ignore_rule);
    if isrect != 0 {
        pe = &mut *(*pa).path.offset(0) as *mut pa_elem;
        pe1 = &mut *(*pa).path.offset(2) as *mut pa_elem;
        r.llx = (*pe).p[0].x;
        r.lly = (*pe).p[0].y;
        r.urx = (*pe1).p[0].x - (*pe).p[0].x;
        r.ury = (*pe1).p[0].y - (*pe).p[0].y;
        let fresh28 = len;
        len = len + 1;
        *b.offset(fresh28 as isize) = ' ' as i32 as i8;
        len += pdf_sprint_rect(b.offset(len as isize), &mut r);
        let fresh29 = len;
        len = len + 1;
        *b.offset(fresh29 as isize) = ' ' as i32 as i8;
        let fresh30 = len;
        len = len + 1;
        *b.offset(fresh30 as isize) = 'r' as i32 as i8;
        let fresh31 = len;
        len = len + 1;
        *b.offset(fresh31 as isize) = 'e' as i32 as i8;
        pdf_doc_add_page_content(b, len as libc::c_uint);
        len = 0i32
    } else {
        n_seg = (*pa).num_paths as libc::c_int;
        i = 0i32;
        len = 0i32;
        pe = &mut *(*pa).path.offset(0) as *mut pa_elem;
        while i < n_seg {
            n_pts = if !pe.is_null() && (*pe).type_0 > -1i32 && (*pe).type_0 < 6i32 {
                petypes[(*pe).type_0 as usize].n_pts
            } else {
                0i32
            };
            j = 0i32;
            pt = &mut *(*pe).p.as_mut_ptr().offset(0) as *mut pdf_coord;
            while j < n_pts {
                let fresh32 = len;
                len = len + 1;
                *b.offset(fresh32 as isize) = ' ' as i32 as i8;
                len += pdf_sprint_coord(b.offset(len as isize), pt);
                j += 1;
                pt = pt.offset(1)
            }
            let fresh33 = len;
            len = len + 1;
            *b.offset(fresh33 as isize) = ' ' as i32 as i8;
            let fresh34 = len;
            len = len + 1;
            *b.offset(fresh34 as isize) =
                (if !pe.is_null() && (*pe).type_0 > -1i32 && (*pe).type_0 < 6i32 {
                    petypes[(*pe).type_0 as usize].opchr as libc::c_int
                } else {
                    ' ' as i32
                }) as i8;
            if len + 128i32 > b_len {
                pdf_doc_add_page_content(b, len as libc::c_uint);
                len = 0i32
            }
            pe = pe.offset(1);
            i += 1
        }
        if len > 0i32 {
            pdf_doc_add_page_content(b, len as libc::c_uint);
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
    pdf_doc_add_page_content(b, len as libc::c_uint);
    return 0i32;
}
unsafe extern "C" fn m_stack_init(mut stack: *mut m_stack) {
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            850i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[i8; 29]>(
                b"void m_stack_init(m_stack *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*stack).size = 0i32;
    (*stack).top = 0 as *mut m_stack_elem;
    (*stack).bottom = 0 as *mut m_stack_elem;
}
unsafe extern "C" fn m_stack_push(mut stack: *mut m_stack, mut data: *mut libc::c_void) {
    let mut elem: *mut m_stack_elem = 0 as *mut m_stack_elem;
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            864i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
                b"void m_stack_push(m_stack *, void *)\x00",
            ))
            .as_ptr(),
        );
    }
    elem = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<m_stack_elem>() as u64)
        as u32) as *mut m_stack_elem;
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
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            885i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[i8; 29]>(
                b"void *m_stack_pop(m_stack *)\x00",
            ))
            .as_ptr(),
        );
    }
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
    return data;
}
unsafe extern "C" fn m_stack_top(mut stack: *mut m_stack) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if !stack.is_null() {
    } else {
        __assert_fail(
            b"stack\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            907i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[i8; 29]>(
                b"void *m_stack_top(m_stack *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*stack).size == 0i32 {
        return 0 as *mut libc::c_void;
    }
    data = (*(*stack).top).data;
    return data;
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
    let mut i: libc::c_int = 0;
    if !gs1.is_null() && !gs2.is_null() {
    } else {
        __assert_fail(
            b"gs1 && gs2\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            964i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                b"void copy_a_gstate(pdf_gstate *, pdf_gstate *)\x00",
            ))
            .as_ptr(),
        );
    }
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
    gs = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pdf_gstate>() as u64)
        as u32) as *mut pdf_gstate;
    init_a_gstate(gs);
    m_stack_push(&mut gs_stack, gs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_clear_gstates() {
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if gs_stack.size > 1i32 {
        /* at least 1 elem. */
        dpx_warning(
            b"GS stack depth is not zero at the end of the document.\x00" as *const u8
                as *const i8,
        ); /* op: q */
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
pub unsafe extern "C" fn pdf_dev_gsave() -> libc::c_int {
    let mut gs0: *mut pdf_gstate = 0 as *mut pdf_gstate;
    let mut gs1: *mut pdf_gstate = 0 as *mut pdf_gstate;
    gs0 = m_stack_top(&mut gs_stack) as *mut pdf_gstate;
    gs1 = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pdf_gstate>() as u64)
        as u32) as *mut pdf_gstate;
    init_a_gstate(gs1);
    copy_a_gstate(gs1, gs0);
    m_stack_push(&mut gs_stack, gs1 as *mut libc::c_void);
    pdf_doc_add_page_content(
        b" q\x00" as *const u8 as *const i8,
        2i32 as libc::c_uint,
    );
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_grestore() -> libc::c_int {
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if gs_stack.size <= 1i32 {
        /* Initial state at bottom */
        dpx_warning(b"Too many grestores.\x00" as *const u8 as *const i8); /* op: Q */
        return -1i32;
    }
    gs = m_stack_pop(&mut gs_stack) as *mut pdf_gstate;
    clear_a_gstate(gs);
    free(gs as *mut libc::c_void);
    pdf_doc_add_page_content(
        b" Q\x00" as *const u8 as *const i8,
        2i32 as libc::c_uint,
    );
    pdf_dev_reset_fonts(0i32);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_push_gstate() -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs0: *mut pdf_gstate = 0 as *mut pdf_gstate;
    gs0 = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pdf_gstate>() as u64)
        as u32) as *mut pdf_gstate;
    init_a_gstate(gs0);
    m_stack_push(gss, gs0 as *mut libc::c_void);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_pop_gstate() -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if (*gss).size <= 1i32 {
        /* Initial state at bottom */
        dpx_warning(b"Too many grestores.\x00" as *const u8 as *const i8);
        return -1i32;
    }
    gs = m_stack_pop(gss) as *mut pdf_gstate;
    clear_a_gstate(gs);
    free(gs as *mut libc::c_void);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_current_depth() -> libc::c_int {
    return gs_stack.size - 1i32;
    /* 0 means initial state */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_grestore_to(mut depth: libc::c_int) {
    let mut gss: *mut m_stack = &mut gs_stack; /* op: Q */
    let mut gs: *mut pdf_gstate = 0 as *mut pdf_gstate;
    if depth >= 0i32 {
    } else {
        __assert_fail(
            b"depth >= 0\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1113i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[i8; 30]>(
                b"void pdf_dev_grestore_to(int)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*gss).size > depth + 1i32 {
        dpx_warning(
            b"Closing pending transformations at end of page/XObject.\x00" as *const u8
                as *const i8,
        );
    }
    while (*gss).size > depth + 1i32 {
        pdf_doc_add_page_content(
            b" Q\x00" as *const u8 as *const i8,
            2i32 as libc::c_uint,
        );
        gs = m_stack_pop(gss) as *mut pdf_gstate;
        clear_a_gstate(gs);
        free(gs as *mut libc::c_void);
    }
    pdf_dev_reset_fonts(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_currentpoint(mut p: *mut pdf_coord) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1137i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[i8; 38]>(
                b"int pdf_dev_currentpoint(pdf_coord *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*p).x = (*cpt).x;
    (*p).y = (*cpt).y;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_currentmatrix(mut M: *mut pdf_tmatrix) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &mut (*gs).matrix;
    if !M.is_null() {
    } else {
        __assert_fail(
            b"M\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1151i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[i8; 41]>(
                b"int pdf_dev_currentmatrix(pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*M).a = (*CTM).a;
    (*M).b = (*CTM).b;
    (*M).c = (*CTM).c;
    (*M).d = (*CTM).d;
    (*M).e = (*CTM).e;
    (*M).f = (*CTM).f;
    return 0i32;
}
/*
 * mask == 0 means stroking color, mask == 0x20 nonstroking color
 *
 * force == 1 means that operators will be generated even if
 *   the color is the same as the current graphics state color
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_color(
    mut color: *const pdf_color,
    mut mask: i8,
    mut force: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    let mut gs: *mut pdf_gstate = m_stack_top(&mut gs_stack) as *mut pdf_gstate;
    let mut current: *mut pdf_color = if mask as libc::c_int != 0 {
        &mut (*gs).fillcolor
    } else {
        &mut (*gs).strokecolor
    };
    if pdf_color_is_valid(color) {
    } else {
        __assert_fail(
            b"pdf_color_is_valid(color)\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1172i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[i8; 53]>(
                b"void pdf_dev_set_color(const pdf_color *, char, int)\x00",
            ))
            .as_ptr(),
        );
    }
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
            fmt_buf[fresh41 as usize] = ('R' as i32 | mask as libc::c_int) as i8;
            let fresh42 = len;
            len = len + 1;
            fmt_buf[fresh42 as usize] = ('G' as i32 | mask as libc::c_int) as i8
        }
        -4 => {
            let fresh43 = len;
            len = len + 1;
            fmt_buf[fresh43 as usize] = ('K' as i32 | mask as libc::c_int) as i8
        }
        -1 => {
            let fresh44 = len;
            len = len + 1;
            fmt_buf[fresh44 as usize] = ('G' as i32 | mask as libc::c_int) as i8
        }
        _ => {}
    }
    pdf_doc_add_page_content(fmt_buf.as_mut_ptr(), len as libc::c_uint);
    pdf_color_copycolor(current, color);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_concat(mut M: *const pdf_tmatrix) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut CTM: *mut pdf_tmatrix = &mut (*gs).matrix;
    let mut W: pdf_tmatrix = {
        let mut init = pdf_tmatrix {
            a: 0i32 as f64,
            b: 0i32 as f64,
            c: 0i32 as f64,
            d: 0i32 as f64,
            e: 0i32 as f64,
            f: 0i32 as f64,
        };
        init
    };
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut len: libc::c_int = 0i32;
    if !M.is_null() {
    } else {
        __assert_fail(
            b"M\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1215i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[i8; 40]>(
                b"int pdf_dev_concat(const pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    /* Adobe Reader erases page content if there are
     * non invertible transformation.
     */
    if fabs((*M).a * (*M).d - (*M).b * (*M).c) < 2.5e-16f64 {
        dpx_warning(
            b"Transformation matrix not invertible.\x00" as *const u8 as *const i8,
        ); /* op: cm */
        dpx_warning(
            b"--- M = [%g %g %g %g %g %g]\x00" as *const u8 as *const i8,
            (*M).a,
            (*M).b,
            (*M).c,
            (*M).d,
            (*M).e,
            (*M).f,
        );
        return -1i32;
    }
    if fabs((*M).a - 1.0f64) > 2.5e-16f64
        || fabs((*M).b) > 2.5e-16f64
        || fabs((*M).c) > 2.5e-16f64
        || fabs((*M).d - 1.0f64) > 2.5e-16f64
        || fabs((*M).e) > 2.5e-16f64
        || fabs((*M).f) > 2.5e-16f64
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
        pdf_doc_add_page_content(buf, len as libc::c_uint);
        let mut _tmp_a: f64 = 0.;
        let mut _tmp_b: f64 = 0.;
        let mut _tmp_c: f64 = 0.;
        let mut _tmp_d: f64 = 0.;
        _tmp_a = (*CTM).a;
        _tmp_b = (*CTM).b;
        _tmp_c = (*CTM).c;
        _tmp_d = (*CTM).d;
        (*CTM).a = (*M).a * _tmp_a + (*M).b * _tmp_c;
        (*CTM).b = (*M).a * _tmp_b + (*M).b * _tmp_d;
        (*CTM).c = (*M).c * _tmp_a + (*M).d * _tmp_c;
        (*CTM).d = (*M).c * _tmp_b + (*M).d * _tmp_d;
        (*CTM).e += (*M).e * _tmp_a + (*M).f * _tmp_c;
        (*CTM).f += (*M).e * _tmp_b + (*M).f * _tmp_d
    }
    inversematrix(&mut W, M);
    pdf_path__transform(cpa, &mut W);
    pdf_coord__transform(cpt, &mut W);
    return 0i32;
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
pub unsafe extern "C" fn pdf_dev_setmiterlimit(mut mlimit: f64) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack; /* op: M */
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate; /* op: J */
    let mut len: libc::c_int = 0i32; /* op: j */
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
        pdf_doc_add_page_content(buf, len as libc::c_uint);
        (*gs).miterlimit = mlimit
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinecap(mut capstyle: libc::c_int) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: libc::c_int = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if (*gs).linecap != capstyle {
        len = sprintf(
            buf,
            b" %d J\x00" as *const u8 as *const i8,
            capstyle,
        );
        pdf_doc_add_page_content(buf, len as libc::c_uint);
        (*gs).linecap = capstyle
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinejoin(mut joinstyle: libc::c_int) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: libc::c_int = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    if (*gs).linejoin != joinstyle {
        len = sprintf(
            buf,
            b" %d j\x00" as *const u8 as *const i8,
            joinstyle,
        );
        pdf_doc_add_page_content(buf, len as libc::c_uint);
        (*gs).linejoin = joinstyle
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setlinewidth(mut width: f64) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: libc::c_int = 0i32;
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
        pdf_doc_add_page_content(buf, len as libc::c_uint);
        (*gs).linewidth = width
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_setdash(
    mut count: libc::c_int,
    mut pattern: *mut f64,
    mut offset: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut len: libc::c_int = 0i32;
    let mut buf: *mut i8 = fmt_buf.as_mut_ptr();
    let mut i: libc::c_int = 0;
    (*gs).linedash.num_dash = count;
    (*gs).linedash.offset = offset;
    pdf_doc_add_page_content(
        b" [\x00" as *const u8 as *const i8,
        2i32 as libc::c_uint,
    );
    i = 0i32;
    while i < count {
        *buf.offset(0) = ' ' as i32 as i8;
        len = pdf_sprint_length(buf.offset(1), *pattern.offset(i as isize));
        pdf_doc_add_page_content(buf, (len + 1i32) as libc::c_uint);
        (*gs).linedash.pattern[i as usize] = *pattern.offset(i as isize);
        i += 1
    }
    pdf_doc_add_page_content(
        b"] \x00" as *const u8 as *const i8,
        2i32 as libc::c_uint,
    );
    len = pdf_sprint_length(buf, offset);
    pdf_doc_add_page_content(buf, len as libc::c_uint);
    pdf_doc_add_page_content(
        b" d\x00" as *const u8 as *const i8,
        2i32 as libc::c_uint,
    );
    return 0i32;
}
/* ZSYUEDVEDEOF */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_clip() -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    return pdf_dev__flushpath(cpa, 'W' as i32 as i8, 0i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_eoclip() -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    return pdf_dev__flushpath(cpa, 'W' as i32 as i8, 1i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_flushpath(
    mut p_op: i8,
    mut fill_rule: libc::c_int,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut error: libc::c_int = 0i32;
    /* last arg 'ignore_rule' is only for single object
     * that can be converted to a rect where fill rule
     * is inessential.
     */
    error = pdf_dev__flushpath(cpa, p_op, fill_rule, 1i32);
    pdf_path__clearpath(cpa);
    (*gs).flags &= !(1i32 << 0i32);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_newpath() -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut p: *mut pdf_path = &mut (*gs).path;
    if (*p).num_paths > 0i32 as libc::c_uint {
        pdf_path__clearpath(p);
    }
    /* The following is required for "newpath" operator in mpost.c. */
    pdf_doc_add_page_content(
        b" n\x00" as *const u8 as *const i8,
        2i32 as libc::c_uint,
    ); /* op: n */
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_moveto(
    mut x: f64,
    mut y: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p: pdf_coord = pdf_coord { x: 0., y: 0. };
    p.x = x;
    p.y = y;
    return pdf_path__moveto(cpa, cpt, &mut p);
    /* cpt updated */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rmoveto(
    mut x: f64,
    mut y: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p: pdf_coord = pdf_coord { x: 0., y: 0. };
    p.x = (*cpt).x + x;
    p.y = (*cpt).y + y;
    return pdf_path__moveto(cpa, cpt, &mut p);
    /* cpt updated */
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_lineto(
    mut x: f64,
    mut y: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x;
    p0.y = y;
    return pdf_path__lineto(cpa, cpt, &mut p0);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rlineto(
    mut x: f64,
    mut y: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x + (*cpt).x;
    p0.y = y + (*cpt).y;
    return pdf_path__lineto(cpa, cpt, &mut p0);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_curveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x0;
    p0.y = y0;
    p1.x = x1;
    p1.y = y1;
    p2.x = x2;
    p2.y = y2;
    return pdf_path__curveto(cpa, cpt, &mut p0, &mut p1, &mut p2);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_vcurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x0;
    p0.y = y0;
    p1.x = x1;
    p1.y = y1;
    return pdf_path__curveto(cpa, cpt, cpt, &mut p0, &mut p1);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_ycurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x0;
    p0.y = y0;
    p1.x = x1;
    p1.y = y1;
    return pdf_path__curveto(cpa, cpt, &mut p0, &mut p1, &mut p1);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rcurveto(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x0 + (*cpt).x;
    p0.y = y0 + (*cpt).y;
    p1.x = x1 + (*cpt).x;
    p1.y = y1 + (*cpt).y;
    p2.x = x2 + (*cpt).x;
    p2.y = y2 + (*cpt).y;
    return pdf_path__curveto(cpa, cpt, &mut p0, &mut p1, &mut p2);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_closepath() -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    return pdf_path__closepath(cpa, cpt);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_dtransform(mut p: *mut pdf_coord, mut M: *const pdf_tmatrix) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &mut (*gs).matrix;
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1557i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[i8; 58]>(
                b"void pdf_dev_dtransform(pdf_coord *, const pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    pdf_coord__dtransform(
        p,
        if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_idtransform(mut p: *mut pdf_coord, mut M: *const pdf_tmatrix) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &mut (*gs).matrix;
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1571i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"void pdf_dev_idtransform(pdf_coord *, const pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    pdf_coord__idtransform(
        p,
        if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_transform(mut p: *mut pdf_coord, mut M: *const pdf_tmatrix) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &mut (*gs).matrix;
    if !p.is_null() {
    } else {
        __assert_fail(
            b"p\x00" as *const u8 as *const i8,
            b"dpx-pdfdraw.c\x00" as *const u8 as *const i8,
            1585i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                b"void pdf_dev_transform(pdf_coord *, const pdf_tmatrix *)\x00",
            ))
            .as_ptr(),
        );
    }
    pdf_coord__transform(
        p,
        if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arc(
    mut c_x: f64,
    mut c_y: f64,
    mut r: f64,
    mut a_0: f64,
    mut a_1: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = c_x;
    c.y = c_y;
    return pdf_path__elliptarc(cpa, cpt, &mut c, r, r, 0.0f64, a_0, a_1, 1i32);
}
/* *negative* arc */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arcn(
    mut c_x: f64,
    mut c_y: f64,
    mut r: f64,
    mut a_0: f64,
    mut a_1: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = c_x;
    c.y = c_y;
    return pdf_path__elliptarc(cpa, cpt, &mut c, r, r, 0.0f64, a_0, a_1, -1i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_arcx(
    mut c_x: f64,
    mut c_y: f64,
    mut r_x: f64,
    mut r_y: f64,
    mut a_0: f64,
    mut a_1: f64,
    mut a_d: libc::c_int,
    mut xar: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = c_x;
    c.y = c_y;
    return pdf_path__elliptarc(cpa, cpt, &mut c, r_x, r_y, xar, a_0, a_1, a_d);
}
/* Required by Tpic */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_bspline(
    mut x0: f64,
    mut y0: f64,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> libc::c_int {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &mut (*gs).path;
    let mut cpt: *mut pdf_coord = &mut (*gs).cp;
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    p1.x = x0 + 2.0f64 * (x1 - x0) / 3.0f64;
    p1.y = y0 + 2.0f64 * (y1 - y0) / 3.0f64;
    p2.x = x1 + (x2 - x1) / 3.0f64;
    p2.y = y1 + (y2 - y1) / 3.0f64;
    p3.x = x2;
    p3.y = y2;
    return pdf_path__curveto(cpa, cpt, &mut p1, &mut p2, &mut p3);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rectfill(
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) -> libc::c_int {
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    r.llx = x;
    r.lly = y;
    r.urx = x + w;
    r.ury = y + h;
    return pdf_dev__rectshape(&mut r, 0 as *const pdf_tmatrix, 'f' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rectclip(
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) -> libc::c_int {
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    r.llx = x;
    r.lly = y;
    r.urx = x + w;
    r.ury = y + h;
    return pdf_dev__rectshape(&mut r, 0 as *const pdf_tmatrix, 'W' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_rectadd(
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) -> libc::c_int {
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    r.llx = x;
    r.lly = y;
    r.urx = x + w;
    r.ury = y + h;
    path_added = 1i32;
    return pdf_dev__rectshape(&mut r, 0 as *const pdf_tmatrix, ' ' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_fixed_point(mut x: f64, mut y: f64) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    (*gs).pt_fixee.x = x;
    (*gs).pt_fixee.y = y;
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
pub unsafe extern "C" fn pdf_dev_get_fixed_point(mut p: *mut pdf_coord) {
    let mut gss: *mut m_stack = &mut gs_stack;
    let mut gs: *mut pdf_gstate = m_stack_top(gss) as *mut pdf_gstate;
    (*p).x = (*gs).pt_fixee.x;
    (*p).y = (*gs).pt_fixee.y;
}
