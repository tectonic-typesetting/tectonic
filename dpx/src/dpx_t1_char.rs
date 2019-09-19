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

use libc::free;
extern "C" {
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;
pub type card8 = u8;

use super::dpx_cff::cff_index;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_ginfo {
    pub use_seac: i32,
    pub wx: f64,
    pub wy: f64,
    pub bbox: C2RustUnnamed_0,
    pub seac: C2RustUnnamed,
}
impl t1_ginfo {
    pub fn new() -> Self {
        Self {
            use_seac: 0,
            wx: 0.,
            wy: 0.,
            bbox: C2RustUnnamed_0 {
                llx: 0.,
                lly: 0.,
                urx: 0.,
                ury: 0.,
            },
            seac: C2RustUnnamed {
                asb: 0.,
                adx: 0.,
                ady: 0.,
                bchar: 0,
                achar: 0,
            },
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub asb: f64,
    pub adx: f64,
    pub ady: f64,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_chardesc {
    pub flags: i32,
    pub sbw: C2RustUnnamed_3,
    pub bbox: C2RustUnnamed_2,
    pub seac: C2RustUnnamed_1,
    pub num_stems: i32,
    pub stems: [t1_stem; 96],
    pub charpath: *mut t1_cpath,
    pub lastpath: *mut t1_cpath,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_cpath {
    pub type_0: i32,
    pub num_args: i32,
    pub args: [f64; 48],
    pub next: *mut t1_cpath,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_stem {
    pub id: i32,
    pub dir: i32,
    pub pos: f64,
    pub del: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub asb: f64,
    pub adx: f64,
    pub ady: f64,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub sbx: f64,
    pub sby: f64,
    pub wx: f64,
    pub wy: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_stemgroup {
    pub num_stems: i32,
    pub stems: [f64; 96],
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
static mut status: i32 = -1i32;
static mut phase: i32 = -1i32;
static mut nest: i32 = -1i32;
static mut cs_stack_top: i32 = 0i32;
static mut ps_stack_top: i32 = 0i32;
/* [vh]stem support require one more stack size. */
static mut cs_arg_stack: [f64; 49] = [0.; 49];
static mut ps_arg_stack: [f64; 194] = [0.; 194];
/*
 * Stem:
 *
 *   1. Stems must be sorted in the increasing bottom/left edge order.
 *   2. The encoded values are all relative; The value x(y) of the first
 *      stem is relative to 0 for Type 2 charstring and is relative to
 *      the left(bottom) side-bearing for Type 1 charstring.
 *   3. A width of -20(-21) specifies the top/right(bottom/left) edge
 *      of an edge hint in Type 2 charstring. But the width of 'ghost'
 *      hint in Type 1 charstring is positive with value 20 or 21.
 *   4. The h(v)stemhm MUST be used instead of h(v)stem if charstring
 *      contains hintmask operator.
 *
 * TODO:
 *
 *  Convert ghost hint to edge hint, Counter control for hstem3/vstem3.
 */
#[inline]
unsafe extern "C" fn stem_compare(mut v1: *const libc::c_void, mut v2: *const libc::c_void) -> i32 {
    let mut cmp: i32 = 0i32;
    let mut s1: *const t1_stem = 0 as *const t1_stem;
    let mut s2: *const t1_stem = 0 as *const t1_stem;
    s1 = v1 as *const t1_stem;
    s2 = v2 as *const t1_stem;
    if (*s1).dir == (*s2).dir {
        if (*s1).pos == (*s2).pos {
            if (*s1).del == (*s2).del {
                cmp = 0i32
            } else {
                cmp = if (*s1).del < (*s2).del { -1i32 } else { 1i32 }
            }
        } else {
            cmp = if (*s1).pos < (*s2).pos { -1i32 } else { 1i32 }
        }
    } else {
        cmp = if (*s1).dir == 0i32 { -1i32 } else { 1i32 }
    }
    cmp
}
unsafe extern "C" fn get_stem(mut cd: *mut t1_chardesc, mut stem_id: i32) -> i32 {
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*cd).num_stems {
        if (*cd).stems[i as usize].id == stem_id {
            break;
        }
        i += 1
    }
    if i < (*cd).num_stems {
        i
    } else {
        -1i32
    }
}
unsafe extern "C" fn add_stem(
    mut cd: *mut t1_chardesc,
    mut pos: f64,
    mut del: f64,
    mut dir: i32,
) -> i32 {
    let mut i: i32 = 0;
    assert!(!cd.is_null());
    pos += if dir == 0i32 {
        (*cd).sbw.sby
    } else {
        (*cd).sbw.sbx
    };
    i = 0i32;
    while i < (*cd).num_stems {
        if (*cd).stems[i as usize].dir == dir
            && (*cd).stems[i as usize].pos == pos
            && (*cd).stems[i as usize].del == del
        {
            break;
        }
        i += 1
    }
    if i == (*cd).num_stems {
        if (*cd).num_stems == 96i32 {
            return -1i32;
        }
        (*cd).stems[i as usize].dir = dir;
        (*cd).stems[i as usize].pos = pos;
        (*cd).stems[i as usize].del = del;
        (*cd).stems[i as usize].id = (*cd).num_stems;
        (*cd).num_stems += 1
    }
    (*cd).stems[i as usize].id
}
unsafe extern "C" fn copy_args(mut args1: *mut f64, mut args2: *mut f64, mut count: i32) {
    loop {
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 > 0i32) {
            break;
        }
        *args1 = *args2;
        args1 = args1.offset(1);
        args2 = args2.offset(1)
    }
}
/*
 * Stack:
 */
/*
 * Path construction:
 */
/* Get operands from cs_arg_stack[] */
unsafe extern "C" fn add_charpath(
    mut cd: *mut t1_chardesc,
    mut type_0: i32,
    mut argv: *mut f64,
    mut argn: i32,
) {
    let mut p: *mut t1_cpath = 0 as *mut t1_cpath;
    assert!(!cd.is_null());
    assert!(argn <= 48i32);
    p = new((1_u64).wrapping_mul(::std::mem::size_of::<t1_cpath>() as u64) as u32) as *mut t1_cpath;
    (*p).type_0 = type_0;
    (*p).num_args = argn;
    (*p).next = 0 as *mut t1_cpath;
    loop {
        let fresh1 = argn;
        argn = argn - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        (*p).args[argn as usize] = *argv.offset(argn as isize)
    }
    if (*cd).charpath.is_null() {
        (*cd).charpath = p
    }
    if !(*cd).lastpath.is_null() {
        (*(*cd).lastpath).next = p
    }
    (*cd).lastpath = p;
    if type_0 >= 0i32
        && phase != 3i32
        && (type_0 >= 4i32 && type_0 <= 9i32
            || type_0 >= 21i32
                && type_0 <= 31i32
                && type_0 != 23i32
                && type_0 != 29i32
                && type_0 != 28i32)
    {
        phase = 2i32
    };
}
unsafe extern "C" fn init_charpath(mut cd: *mut t1_chardesc) {
    (*cd).flags = 0i32;
    (*cd).num_stems = 0i32;
    (*cd).sbw.wy = 0.0f64;
    (*cd).sbw.wx = (*cd).sbw.wy;
    (*cd).sbw.sby = 0.0f64;
    (*cd).sbw.sbx = (*cd).sbw.sby;
    (*cd).bbox.ury = 0.0f64;
    (*cd).bbox.urx = (*cd).bbox.ury;
    (*cd).bbox.lly = (*cd).bbox.urx;
    (*cd).bbox.llx = (*cd).bbox.lly;
    (*cd).lastpath = 0 as *mut t1_cpath;
    (*cd).charpath = (*cd).lastpath;
}
unsafe extern "C" fn release_charpath(mut cd: *mut t1_chardesc) {
    let mut curr: *mut t1_cpath = 0 as *mut t1_cpath;
    let mut next: *mut t1_cpath = 0 as *mut t1_cpath;
    assert!(!cd.is_null());
    curr = (*cd).charpath;
    while !curr.is_null() {
        next = (*curr).next;
        free(curr as *mut libc::c_void);
        curr = next
    }
    (*cd).lastpath = 0 as *mut t1_cpath;
    (*cd).charpath = (*cd).lastpath;
}
/*
 * Type 1 charstring operators:
 */
/*
 * Single byte operators:
 */
unsafe extern "C" fn do_operator1(mut cd: *mut t1_chardesc, mut data: *mut *mut card8) {
    let mut op: card8 = **data;
    *data = (*data).offset(1);
    match op as i32 {
        9 => {
            /*
             * From T1 spec.:
             *  Note that, unlike the closepath command in the PostScript language,
             *  this command does not reposition the current point. Any subsequent
             *  rmoveto must be relative to the current point in force before the
             *  Type 1 font format closepath command was given.
             */
            /* noop */
            cs_stack_top = 0i32
        }
        13 => {
            if cs_stack_top < 2i32 {
                status = -2i32;
                return;
            }
            cs_stack_top -= 1;
            (*cd).sbw.wx = cs_arg_stack[cs_stack_top as usize];
            (*cd).sbw.wy = 0i32 as f64;
            cs_stack_top -= 1;
            (*cd).sbw.sbx = cs_arg_stack[cs_stack_top as usize];
            (*cd).sbw.sby = 0i32 as f64;
            cs_stack_top = 0i32
        }
        1 | 3 => {
            if cs_stack_top < 2i32 {
                status = -2i32;
                return;
            }
            let mut stem_id: i32 = 0;
            stem_id = add_stem(
                cd,
                cs_arg_stack[(cs_stack_top - 2i32) as usize],
                cs_arg_stack[(cs_stack_top - 1i32) as usize],
                if op as i32 == 1i32 { 0i32 } else { 1i32 },
            );
            if stem_id < 0i32 {
                warn!("Too many hints...");
                status = -1i32;
                return;
            }
            /* Put stem_id onto the stack... */
            let fresh2 = cs_stack_top;
            cs_stack_top = cs_stack_top + 1;
            cs_arg_stack[fresh2 as usize] = stem_id as f64;
            add_charpath(
                cd,
                -1i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - 1i32) as isize),
                1i32,
            );
            cs_stack_top = 0i32
        }
        21 => {
            /*
             * Reference point is (0, 0) in Type 2 charstring.
             */
            if cs_stack_top < 2i32 {
                status = -2i32;
                return;
            }
            if phase < 2i32 {
                cs_arg_stack[(cs_stack_top - 2i32) as usize] += (*cd).sbw.sbx;
                cs_arg_stack[(cs_stack_top - 1i32) as usize] += (*cd).sbw.sby
            }
            add_charpath(
                cd,
                op as i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - 2i32) as isize),
                2i32,
            );
            cs_stack_top = 0i32
        }
        22 | 4 => {
            if cs_stack_top < 1i32 {
                status = -2i32;
                return;
            }
            let mut argn: i32 = 1i32;
            if phase < 2i32 {
                /*
                 * The reference point for the first moveto operator is diferrent
                 * between Type 1 charstring and Type 2 charstring. We compensate it.
                 */
                if op as i32 == 22i32 {
                    cs_arg_stack[(cs_stack_top - 1i32) as usize] += (*cd).sbw.sbx;
                    if (*cd).sbw.sby != 0.0f64 {
                        let fresh3 = cs_stack_top;
                        cs_stack_top = cs_stack_top + 1;
                        cs_arg_stack[fresh3 as usize] = (*cd).sbw.sby;
                        argn = 2i32;
                        op = 21i32 as card8
                    }
                } else {
                    cs_arg_stack[(cs_stack_top - 1i32) as usize] += (*cd).sbw.sby;
                    if (*cd).sbw.sbx != 0.0f64 {
                        cs_arg_stack[cs_stack_top as usize] =
                            cs_arg_stack[(cs_stack_top - 1i32) as usize];
                        cs_arg_stack[(cs_stack_top - 1i32) as usize] = (*cd).sbw.sbx;
                        cs_stack_top += 1;
                        argn = 2i32;
                        op = 21i32 as card8
                    }
                }
            }
            add_charpath(
                cd,
                op as i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - argn) as isize),
                argn,
            );
            cs_stack_top = 0i32
        }
        14 => {
            status = 3i32;
            cs_stack_top = 0i32
        }
        5 => {
            /* above oprators are candidate for first stack-clearing operator */
            if cs_stack_top < 2i32 {
                status = -2i32;
                return;
            }
            add_charpath(
                cd,
                op as i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - 2i32) as isize),
                2i32,
            );
            cs_stack_top = 0i32
        }
        6 | 7 => {
            if cs_stack_top < 1i32 {
                status = -2i32;
                return;
            }
            add_charpath(
                cd,
                op as i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - 1i32) as isize),
                1i32,
            );
            cs_stack_top = 0i32
        }
        8 => {
            if cs_stack_top < 6i32 {
                status = -2i32;
                return;
            }
            add_charpath(
                cd,
                op as i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - 6i32) as isize),
                6i32,
            );
            cs_stack_top = 0i32
        }
        30 | 31 => {
            if cs_stack_top < 4i32 {
                status = -2i32;
                return;
            }
            add_charpath(
                cd,
                op as i32,
                &mut *cs_arg_stack
                    .as_mut_ptr()
                    .offset((cs_stack_top - 4i32) as isize),
                4i32,
            );
            cs_stack_top = 0i32
        }
        11 => {}
        10 => {
            panic!("Unexpected callsubr.");
        }
        _ => {
            /* no-op ? */
            warn!("Unknown charstring operator: 0x{:02x}", op as i32,);
            status = -1i32
        }
    };
}
/*
 * OtherSubrs:
 *
 *  arg0 arg1 ... argn n othersubr# callothersubr
 *
 *   0: Build flex:
 *      fd x y 3 0 callothersubr
 *      Othersubr #0 pushes x and y to PostScript interpreter operand stack.
 *   1: Start flex
 *      0 1 callothersubr
 *   2: Mark flex control points
 *      0 2 callothersubr
 *   3: Discard hint
 *      subr# 1 3 callothersubr pop callsubr
 *      Subroutine subr# (only) contains stem declaration.
 *      Othersubr #3 pushes subr# to PostScript interpreter operand stack.
 *  12: Counter control
 *      A subr to avoid stack overflow.
 *  13: Counter control
 */
/*
 * Convert six control points marked as CS_FLEX_CTRL to a flex path.
 */
unsafe extern "C" fn do_othersubr0(mut cd: *mut t1_chardesc) {
    let mut flex: *mut t1_cpath = 0 as *mut t1_cpath;
    let mut cur: *mut t1_cpath = 0 as *mut t1_cpath;
    let mut next: *mut t1_cpath = 0 as *mut t1_cpath;
    if ps_stack_top < 1i32 {
        status = -1i32;
        return;
    }
    /* Seek first CS_FLEX_CTRL mark */
    cur = (*cd).charpath;
    while !cur.is_null() && (*cur).type_0 != -2i32 {
        cur = (*cur).next
    }
    flex = cur;
    let mut i: u32 = 0;
    cur = (*cur).next;
    i = 1_u32;
    while i < 7_u32 {
        if cur.is_null() || (*cur).type_0 != -2i32 || (*cur).num_args != 2i32 {
            status = -1i32;
            return;
        }
        if i == 1_u32 {
            (*flex).args[0] += (*cur).args[0];
            (*flex).args[1] += (*cur).args[1]
        } else {
            copy_args(
                &mut *(*flex)
                    .args
                    .as_mut_ptr()
                    .offset((2_u32).wrapping_mul(i).wrapping_sub(2_u32) as isize),
                (*cur).args.as_mut_ptr(),
                2i32,
            );
        }
        next = (*cur).next;
        free(cur as *mut libc::c_void);
        cur = next;
        i = i.wrapping_add(1)
    }
    if !cur.is_null() {
        status = -1i32;
        return;
    }
    /*
     * Now 'flex' have all six control points, the first pair is relative
     * from starting point.
     */
    (*flex).type_0 = 35i32; /* flex depth */
    ps_stack_top -= 1;
    (*flex).args[12] = ps_arg_stack[ps_stack_top as usize];
    (*flex).num_args = 13i32;
    (*flex).next = 0 as *mut t1_cpath;
    (*cd).lastpath = flex;
    phase = 2i32;
}
/* Start flex */
unsafe extern "C" fn do_othersubr1() {
    phase = 3i32;
}
/* Mark flex control point */
unsafe extern "C" fn do_othersubr2(mut cd: *mut t1_chardesc) {
    if phase != 3i32 || (*cd).lastpath.is_null() {
        status = -1i32;
        return;
    }
    match (*(*cd).lastpath).type_0 {
        21 => {}
        22 => {
            (*(*cd).lastpath).num_args = 2i32;
            (*(*cd).lastpath).args[1] = 0.0f64
        }
        4 => {
            (*(*cd).lastpath).num_args = 2i32;
            (*(*cd).lastpath).args[1] = (*(*cd).lastpath).args[0];
            (*(*cd).lastpath).args[0] = 0.0f64
        }
        _ => {
            status = -1i32;
            return;
        }
    }
    (*(*cd).lastpath).type_0 = -2i32;
}
/*
 * Hint Replacement:
 *  "Adobe Type 1 Font Format", Chapter 8.
 */
unsafe extern "C" fn do_othersubr3(mut cd: *mut t1_chardesc) {
    (*cd).flags |= 1i32 << 0i32;
}
unsafe extern "C" fn do_othersubr12() {
    /* Othersubr12 call must immediately follow the hsbw or sbw. */
    if phase != 0i32 {
        status = -1i32;
        return;
    };
    /* noop */
}
unsafe extern "C" fn do_othersubr13(mut cd: *mut t1_chardesc) {
    let mut stemgroups: [t1_stemgroup; 96] = [t1_stemgroup {
        num_stems: 0,
        stems: [0.; 96],
    }; 96];
    let mut num_hgroups: i32 = 0;
    let mut num_vgroups: i32 = 0;
    let mut n: i32 = 0;
    let mut stem_id: i32 = 0;
    let mut pos: f64 = 0.;
    let mut del: f64 = 0.;
    /* After #12 callothersubr or hsbw or sbw. */
    if phase != 0i32 {
        status = -1i32;
        return;
    }
    n = 0i32;
    while n < 96i32 {
        stemgroups[n as usize].num_stems = 0i32;
        n += 1
    }
    ps_stack_top -= 1;
    num_hgroups = ps_arg_stack[ps_stack_top as usize] as i32;
    if num_hgroups < 0i32 || num_hgroups > 96i32 {
        status = -1i32;
        return;
    }
    n = 0i32;
    pos = 0.0f64;
    while ps_stack_top >= 2i32 && n < num_hgroups {
        /* add_stem() add sidebearing */
        ps_stack_top -= 1;
        pos += ps_arg_stack[ps_stack_top as usize];
        ps_stack_top -= 1;
        del = ps_arg_stack[ps_stack_top as usize];
        stem_id = add_stem(
            cd,
            if del < 0.0f64 { pos + del } else { pos },
            if del < 0.0f64 { -del } else { del },
            0i32,
        );
        stemgroups[n as usize].stems[stemgroups[n as usize].num_stems as usize] = stem_id as f64;
        stemgroups[n as usize].num_stems += 1i32;
        pos += del;
        if del < 0.0f64 {
            pos = 0.0f64;
            n += 1
        }
    }
    if n != num_hgroups {
        status = -2i32;
        return;
    }
    ps_stack_top -= 1;
    num_vgroups = ps_arg_stack[ps_stack_top as usize] as i32;
    if num_vgroups < 0i32 || num_vgroups > 96i32 {
        status = -1i32;
        return;
    }
    n = 0i32;
    pos = 0.0f64;
    while ps_stack_top >= 2i32 && n < num_vgroups {
        /* add_stem() add sidebearing */
        ps_stack_top -= 1;
        pos += ps_arg_stack[ps_stack_top as usize];
        ps_stack_top -= 1;
        del = ps_arg_stack[ps_stack_top as usize];
        stem_id = add_stem(
            cd,
            if del < 0.0f64 { pos + del } else { pos },
            if del < 0.0f64 { -del } else { del },
            1i32,
        );
        stemgroups[n as usize].stems[stemgroups[n as usize].num_stems as usize] = stem_id as f64;
        stemgroups[n as usize].num_stems += 1i32;
        pos += del;
        if del < 0.0f64 {
            pos = 0.0f64;
            n += 1
        }
    }
    if n != num_vgroups {
        status = -2i32;
        return;
    }
    n = 0i32;
    while n
        < (if num_hgroups > num_vgroups {
            num_hgroups
        } else {
            num_vgroups
        })
    {
        add_charpath(
            cd,
            20i32,
            stemgroups[n as usize].stems.as_mut_ptr(),
            stemgroups[n as usize].num_stems,
        );
        n += 1
    }
    (*cd).flags |= 1i32 << 1i32;
}
unsafe extern "C" fn do_callothersubr(mut cd: *mut t1_chardesc) {
    let mut argn: i32 = 0;
    let mut subrno: i32 = 0;
    if cs_stack_top < 2i32 {
        status = -2i32;
        return;
    }
    cs_stack_top -= 1;
    subrno = cs_arg_stack[cs_stack_top as usize] as i32;
    cs_stack_top -= 1;
    argn = cs_arg_stack[cs_stack_top as usize] as i32;
    if cs_stack_top < argn {
        status = -2i32;
        return;
    }
    if ps_stack_top + argn > 96i32 * 2i32 + 2i32 {
        status = -1i32;
        return;
    }
    loop {
        let fresh4 = argn;
        argn = argn - 1;
        if !(fresh4 > 0i32) {
            break;
        }
        cs_stack_top -= 1;
        let fresh5 = ps_stack_top;
        ps_stack_top = ps_stack_top + 1;
        ps_arg_stack[fresh5 as usize] = cs_arg_stack[cs_stack_top as usize]
    }
    match subrno {
        0 => {
            do_othersubr0(cd);
        }
        1 => {
            do_othersubr1();
        }
        2 => {
            do_othersubr2(cd);
        }
        3 => {
            do_othersubr3(cd);
        }
        12 => {
            do_othersubr12();
        }
        13 => {
            do_othersubr13(cd);
        }
        _ => {
            panic!("Unknown othersubr #{}.", subrno);
        }
    };
}
/*
 * Double byte operators:
 */
unsafe extern "C" fn do_operator2(
    mut cd: *mut t1_chardesc,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
) {
    let mut op: card8 = 0;
    *data = (*data).offset(1);
    if endptr < (*data).offset(1) {
        status = -1i32;
        return;
    }
    op = **data;
    *data = (*data).offset(1);
    match op as i32 {
        7 => {
            if cs_stack_top < 4i32 {
                status = -2i32;
                return;
            }
            cs_stack_top -= 1;
            (*cd).sbw.wy = cs_arg_stack[cs_stack_top as usize];
            cs_stack_top -= 1;
            (*cd).sbw.wx = cs_arg_stack[cs_stack_top as usize];
            cs_stack_top -= 1;
            (*cd).sbw.sby = cs_arg_stack[cs_stack_top as usize];
            cs_stack_top -= 1;
            (*cd).sbw.sbx = cs_arg_stack[cs_stack_top as usize];
            cs_stack_top = 0i32
        }
        2 | 1 => {
            /*
             * TODO:
             *  The counter control can be used for hstem3 and vstem3
             *  operator if LanguageGroup is not equal to 1.
             */
            if cs_stack_top < 6i32 {
                status = -2i32;
                return;
            }
            let mut i: i32 = 0;
            i = 2i32;
            while i >= 0i32 {
                let mut stem_id: i32 = 0;
                stem_id = add_stem(
                    cd,
                    cs_arg_stack[(cs_stack_top - 2i32 * i - 2i32) as usize],
                    cs_arg_stack[(cs_stack_top - 2i32 * i - 1i32) as usize],
                    if op as i32 == 2i32 { 0i32 } else { 1i32 },
                );
                if stem_id < 0i32 {
                    warn!("Too many hints...");
                    status = -1i32;
                    return;
                }
                /* Put stem_id onto the stack... */
                let fresh6 = cs_stack_top;
                cs_stack_top = cs_stack_top + 1;
                cs_arg_stack[fresh6 as usize] = stem_id as f64;
                add_charpath(
                    cd,
                    -1i32,
                    &mut *cs_arg_stack
                        .as_mut_ptr()
                        .offset((cs_stack_top - 1i32) as isize),
                    1i32,
                );
                cs_stack_top -= 1;
                i -= 1
            }
            cs_stack_top = 0i32
        }
        33 => {
            if cs_stack_top < 2i32 {
                status = -2i32;
                return;
            }
            /* noop */
            cs_stack_top = 0i32
        }
        17 => {
            /* all operator above are stack-clearing */
            /*
             * Transfer a operand from PS interpreter operand stack to BuildChar
             * operand stack.
             */
            if ps_stack_top < 1i32 {
                status = -1i32;
                return;
            }
            if cs_stack_top + 1i32 > 48i32 {
                status = -2i32;
                return;
            }
            ps_stack_top -= 1;
            let fresh7 = cs_stack_top;
            cs_stack_top = cs_stack_top + 1;
            cs_arg_stack[fresh7 as usize] = ps_arg_stack[ps_stack_top as usize]
        }
        0 => {}
        12 => {
            /* TODO: check overflow */
            if cs_stack_top < 2i32 {
                status = -2i32;
                return;
            }
            cs_arg_stack[(cs_stack_top - 2i32) as usize] /=
                cs_arg_stack[(cs_stack_top - 1i32) as usize];
            cs_stack_top -= 1
        }
        16 => {
            do_callothersubr(cd);
        }
        6 => {
            if cs_stack_top < 5i32 {
                status = -2i32;
                return;
            }
            (*cd).flags |= 1i32 << 2i32;
            cs_stack_top -= 1;
            (*cd).seac.achar = cs_arg_stack[cs_stack_top as usize] as card8;
            cs_stack_top -= 1;
            (*cd).seac.bchar = cs_arg_stack[cs_stack_top as usize] as card8;
            cs_stack_top -= 1;
            (*cd).seac.ady = cs_arg_stack[cs_stack_top as usize];
            cs_stack_top -= 1;
            (*cd).seac.adx = cs_arg_stack[cs_stack_top as usize];
            /* We must compensate the difference of the glyph origin. */
            (*cd).seac.ady += (*cd).sbw.sby;
            cs_stack_top -= 1;
            (*cd).seac.adx += (*cd).sbw.sbx - cs_arg_stack[cs_stack_top as usize];
            cs_stack_top = 0i32
        }
        _ => {
            /* no-op ? */
            warn!("Unknown charstring operator: 0x0c{:02x}", op as i32,);
            status = -1i32
        }
    };
}
/*
 * Charstring encoding:
 *  Copied from cs_type2.c
 *  Note:
 *   The Type 2 interpretation of a number encoded in five-bytes (those with
 *   an initial byte value of 255) differs from how it is interpreted in the
 *   Type 1 format.
 */
/* Type 2 5-bytes encoding used. */
unsafe extern "C" fn put_numbers(
    mut argv: *mut f64,
    mut argn: i32,
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
) {
    let mut i: i32 = 0;
    i = 0i32;
    while i < argn {
        let mut value: f64 = 0.;
        let mut ivalue: i32 = 0;
        value = *argv.offset(i as isize);
        /* Nearest integer value */
        ivalue = (value + 0.5).floor() as i32;
        if value >= 0x8000i64 as f64 || value <= (-0x8000 - 1i32 as i64) as f64 {
            /*
             * This number cannot be represented as a single operand.
             * We must use `a b mul ...' or `a c div' to represent large values.
             */
            panic!("Argument value too large. (This is bug)");
        } else {
            if (value - ivalue as f64).abs() > 3.0e-5f64 {
                /* 16.16-bit signed fixed value  */
                if limit < (*dest).offset(5) {
                    status = -3i32;
                    return;
                }
                let fresh8 = *dest;
                *dest = (*dest).offset(1);
                *fresh8 = 255i32 as card8;
                /* Everything else are integers. */
                ivalue = value.floor() as i32; /* mantissa */
                let fresh9 = *dest; /* fraction */
                *dest = (*dest).offset(1); /* Shouldn't come here */
                *fresh9 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh10 = *dest;
                *dest = (*dest).offset(1);
                *fresh10 = (ivalue & 0xffi32) as card8;
                ivalue = ((value - ivalue as f64) * 0x10000i64 as f64) as i32;
                let fresh11 = *dest;
                *dest = (*dest).offset(1);
                *fresh11 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh12 = *dest;
                *dest = (*dest).offset(1);
                *fresh12 = (ivalue & 0xffi32) as card8
            } else if ivalue >= -107i32 && ivalue <= 107i32 {
                if limit < (*dest).offset(1) {
                    status = -3i32;
                    return;
                }
                let fresh13 = *dest;
                *dest = (*dest).offset(1);
                *fresh13 = (ivalue + 139i32) as card8
            } else if ivalue >= 108i32 && ivalue <= 1131i32 {
                if limit < (*dest).offset(2) {
                    status = -3i32;
                    return;
                }
                ivalue = 0xf700u32.wrapping_add(ivalue as u32).wrapping_sub(108_u32) as i32;
                let fresh14 = *dest;
                *dest = (*dest).offset(1);
                *fresh14 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh15 = *dest;
                *dest = (*dest).offset(1);
                *fresh15 = (ivalue & 0xffi32) as card8
            } else if ivalue >= -1131i32 && ivalue <= -108i32 {
                if limit < (*dest).offset(2) {
                    status = -3i32;
                    return;
                }
                ivalue = 0xfb00u32.wrapping_sub(ivalue as u32).wrapping_sub(108_u32) as i32;
                let fresh16 = *dest;
                *dest = (*dest).offset(1);
                *fresh16 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh17 = *dest;
                *dest = (*dest).offset(1);
                *fresh17 = (ivalue & 0xffi32) as card8
            } else if ivalue >= -32768i32 && ivalue <= 32767i32 {
                /* shortint */
                if limit < (*dest).offset(3) {
                    status = -3i32;
                    return;
                }
                let fresh18 = *dest;
                *dest = (*dest).offset(1);
                *fresh18 = 28i32 as card8;
                let fresh19 = *dest;
                *dest = (*dest).offset(1);
                *fresh19 = (ivalue >> 8i32 & 0xffi32) as card8;
                let fresh20 = *dest;
                *dest = (*dest).offset(1);
                *fresh20 = (ivalue & 0xffi32) as card8
            } else {
                panic!("Unexpected error.");
            }
        }
        i += 1
    }
}
unsafe extern "C" fn get_integer(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let mut result: i32 = 0i32;
    let mut b0: card8 = **data;
    let mut b1: card8 = 0;
    let mut b2: card8 = 0;
    *data = (*data).offset(1);
    if b0 as i32 == 28i32 {
        /* shortint */
        if endptr < (*data).offset(2) {
            status = -1i32;
            return;
        }
        b1 = **data;
        b2 = *(*data).offset(1);
        result = b1 as i32 * 256i32 + b2 as i32;
        if result > 0x7fffi32 {
            result = (result as i64 - 0x10000) as i32
        }
        *data = (*data).offset(2)
    } else if b0 as i32 >= 32i32 && b0 as i32 <= 246i32 {
        /* int (1) */
        result = b0 as i32 - 139i32
    } else if b0 as i32 >= 247i32 && b0 as i32 <= 250i32 {
        /* int (2) */
        if endptr < (*data).offset(1) {
            status = -1i32;
            return;
        }
        b1 = **data;
        result = (b0 as i32 - 247i32) * 256i32 + b1 as i32 + 108i32;
        *data = (*data).offset(1)
    } else if b0 as i32 >= 251i32 && b0 as i32 <= 254i32 {
        if endptr < (*data).offset(1) {
            status = -1i32;
            return;
        }
        b1 = **data;
        result = -(b0 as i32 - 251i32) * 256i32 - b1 as i32 - 108i32;
        *data = (*data).offset(1)
    } else {
        status = -1i32;
        return;
    }
    if cs_stack_top + 1i32 > 48i32 {
        status = -2i32;
        return;
    }
    let fresh21 = cs_stack_top;
    cs_stack_top = cs_stack_top + 1;
    cs_arg_stack[fresh21 as usize] = result as f64;
}
/* Type 1 */
unsafe extern "C" fn get_longint(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let mut result: i32 = 0i32;
    let mut i: u32 = 0;
    *data = (*data).offset(1);
    if endptr < (*data).offset(4) {
        status = -1i32;
        return;
    }
    result = **data as i32;
    if result as i64 >= 0x80 {
        result = (result as i64 - 0x100) as i32
    }
    *data = (*data).offset(1);
    i = 1_u32;
    while i < 4_u32 {
        result = result * 256i32 + **data as i32;
        *data = (*data).offset(1);
        i = i.wrapping_add(1)
    }
    if cs_stack_top + 1i32 > 48i32 {
        status = -2i32;
        return;
    }
    let fresh22 = cs_stack_top;
    cs_stack_top = cs_stack_top + 1;
    cs_arg_stack[fresh22 as usize] = result as f64;
}
/*
 * TODO:
 *  Check "seac"
 *   We cannot do backword parsing due to subroutine, div etc.
 */
/* Parse charstring and build charpath. */
unsafe extern "C" fn t1char_build_charpath(
    mut cd: *mut t1_chardesc,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut subrs: *mut cff_index,
) {
    let mut b0: card8 = 0i32 as card8;
    let mut subr: *mut card8 = 0 as *mut card8;
    let mut len: i32 = 0;
    if nest > 10i32 {
        panic!("Subroutine nested too deeply.");
    }
    nest += 1;
    while *data < endptr && status == 0i32 {
        b0 = **data;
        if b0 as i32 == 255i32 {
            get_longint(data, endptr);
        /* Type 1 */
        } else if b0 as i32 == 11i32 {
            status = 2i32
        } else if b0 as i32 == 10i32 {
            if cs_stack_top < 1i32 {
                status = -2i32
            } else {
                let mut idx: i32 = 0;
                cs_stack_top -= 1;
                idx = cs_arg_stack[cs_stack_top as usize] as i32;
                if subrs.is_null() || idx >= (*subrs).count as i32 {
                    panic!("Invalid Subr#.");
                }
                subr = (*subrs)
                    .data
                    .offset(*(*subrs).offset.offset(idx as isize) as isize)
                    .offset(-1);
                len = (*(*subrs).offset.offset((idx + 1i32) as isize))
                    .wrapping_sub(*(*subrs).offset.offset(idx as isize))
                    as i32;
                t1char_build_charpath(cd, &mut subr, subr.offset(len as isize), subrs);
                *data = (*data).offset(1)
            }
        } else if b0 as i32 == 12i32 {
            do_operator2(cd, data, endptr);
        } else if (b0 as i32) < 32i32 && b0 as i32 != 28i32 {
            /* 19, 20 need mask */
            do_operator1(cd, data);
        } else if b0 as i32 >= 22i32 && b0 as i32 <= 27i32 || b0 as i32 == 31i32 {
            /* reserved */
            status = -1i32
        /* not an error ? */
        } else {
            get_integer(data, endptr);
        }
    }
    if status == 2i32 {
        status = 0i32
    } else if status == 3i32 && *data < endptr {
        if !(*data == endptr.offset(-1) && **data as i32 == 11i32) {
            warn!(
                "Garbage after endchar. ({} bytes)",
                endptr.wrapping_offset_from(*data) as i64 as i32
            );
        }
    } else if status < 0i32 {
        /* error */
        panic!(
            "Parsing charstring failed: (status={}, stack={})",
            status, cs_stack_top
        );
    }
    nest -= 1;
}
/*
 * Calculate BoundingBox and compress path.
 *  The essentials of PDF size reduction is not Type 2 charstring compression
 *  but Type 1 charstring encryption. Encryption makes lossless compression
 *  useless. We will only do very simple charstring compression.
 */
unsafe extern "C" fn do_postproc(mut cd: *mut t1_chardesc) {
    let mut i: u32 = 0;
    let mut cur: *mut t1_cpath = 0 as *mut t1_cpath;
    let mut prev: *mut t1_cpath = 0 as *mut t1_cpath;
    let mut next: *mut t1_cpath = 0 as *mut t1_cpath;
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    if (*cd).charpath.is_null() {
        return;
    }
    /* Set dummy large value. */
    (*cd).bbox.lly = 100000.0f64;
    (*cd).bbox.llx = (*cd).bbox.lly;
    (*cd).bbox.ury = -100000.0f64;
    (*cd).bbox.urx = (*cd).bbox.ury;
    cur = (*cd).charpath;
    prev = 0 as *mut t1_cpath;
    y = 0.0f64;
    x = y;
    while !cur.is_null() {
        next = (*cur).next;
        match (*cur).type_0 {
            21 => {
                x += (*cur).args[0];
                y += (*cur).args[1];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
            }
            5 => {
                x += (*cur).args[0];
                y += (*cur).args[1];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                if !prev.is_null() && !cur.is_null() && (*prev).num_args + (*cur).num_args < 48i32 {
                    if (*prev).type_0 == 5i32 {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    } else if (*prev).type_0 == 8i32 {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).type_0 = 24i32;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    }
                }
            }
            22 => {
                x += (*cur).args[0];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
            }
            6 => {
                x += (*cur).args[0];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                if !prev.is_null() && !cur.is_null() && (*prev).num_args + (*cur).num_args < 48i32 {
                    if (*prev).type_0 == 7i32 && (*prev).num_args % 2i32 == 1i32
                        || (*prev).type_0 == 6i32 && (*prev).num_args % 2i32 == 0i32
                    {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    }
                }
            }
            4 => {
                y += (*cur).args[0];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
            }
            7 => {
                y += (*cur).args[0];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                if !prev.is_null() && !cur.is_null() && (*prev).num_args + (*cur).num_args < 48i32 {
                    if (*prev).type_0 == 6i32 && (*prev).num_args % 2i32 == 1i32
                        || (*prev).type_0 == 7i32 && (*prev).num_args % 2i32 == 0i32
                    {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    }
                }
            }
            8 => {
                i = 0_u32;
                while i < 3_u32 {
                    x += (*cur).args[(2_u32).wrapping_mul(i) as usize];
                    y += (*cur).args[(2_u32).wrapping_mul(i).wrapping_add(1_u32) as usize];
                    if (*cd).bbox.llx > x {
                        (*cd).bbox.llx = x
                    }
                    if (*cd).bbox.urx < x {
                        (*cd).bbox.urx = x
                    }
                    if (*cd).bbox.lly > y {
                        (*cd).bbox.lly = y
                    }
                    if (*cd).bbox.ury < y {
                        (*cd).bbox.ury = y
                    }
                    i = i.wrapping_add(1)
                }
                if !prev.is_null() && !cur.is_null() && (*prev).num_args + (*cur).num_args < 48i32 {
                    if (*prev).type_0 == 8i32 {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    } else if (*prev).type_0 == 5i32 {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).type_0 = 25i32;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    }
                }
            }
            30 => {
                y += (*cur).args[0];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                x += (*cur).args[1];
                y += (*cur).args[2];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                x += (*cur).args[3];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                if !prev.is_null() && !cur.is_null() && (*prev).num_args + (*cur).num_args < 48i32 {
                    if (*prev).type_0 == 31i32 && (*prev).num_args / 4i32 % 2i32 == 1i32
                        || (*prev).type_0 == 30i32 && (*prev).num_args / 4i32 % 2i32 == 0i32
                    {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    }
                }
            }
            31 => {
                x += (*cur).args[0];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                x += (*cur).args[1];
                y += (*cur).args[2];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                y += (*cur).args[3];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y
                }
                if !prev.is_null() && !cur.is_null() && (*prev).num_args + (*cur).num_args < 48i32 {
                    if (*prev).type_0 == 30i32 && (*prev).num_args / 4i32 % 2i32 == 1i32
                        || (*prev).type_0 == 31i32 && (*prev).num_args / 4i32 % 2i32 == 0i32
                    {
                        copy_args(
                            (*prev).args.as_mut_ptr().offset((*prev).num_args as isize),
                            (*cur).args.as_mut_ptr(),
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next;
                        cur = mfree(cur as *mut libc::c_void) as *mut t1_cpath
                    }
                }
            }
            35 => {
                i = 0_u32;
                while i < 6_u32 {
                    x += (*cur).args[(2_u32).wrapping_mul(i) as usize];
                    y += (*cur).args[(2i32 * 1i32 + 1i32) as usize];
                    if (*cd).bbox.llx > x {
                        (*cd).bbox.llx = x
                    }
                    if (*cd).bbox.urx < x {
                        (*cd).bbox.urx = x
                    }
                    if (*cd).bbox.lly > y {
                        (*cd).bbox.lly = y
                    }
                    if (*cd).bbox.ury < y {
                        (*cd).bbox.ury = y
                    }
                    i = i.wrapping_add(1)
                }
                if (*cur).args[12] == 50.0f64 {
                    if (*cur).args[1] == 0.0f64
                        && (*cur).args[11] == 0.0f64
                        && (*cur).args[5] == 0.0f64
                        && (*cur).args[7] == 0.0f64
                        && (*cur).args[3] + (*cur).args[9] == 0.0f64
                    {
                        /* cur->args[0] = cur->args[0];  dx1 */
                        (*cur).args[1] = (*cur).args[2]; /* dx2 */
                        (*cur).args[2] = (*cur).args[3]; /* dy2 */
                        (*cur).args[3] = (*cur).args[4]; /* dx3 */
                        (*cur).args[4] = (*cur).args[6]; /* dx4 */
                        (*cur).args[5] = (*cur).args[8]; /* dx5 */
                        (*cur).args[6] = (*cur).args[10]; /* dx6 */
                        (*cur).num_args = 7i32;
                        (*cur).type_0 = 34i32
                    } else if (*cur).args[5] == 0.0f64
                        && (*cur).args[7] == 0.0f64
                        && (*cur).args[1] + (*cur).args[3] + (*cur).args[9] + (*cur).args[11]
                            == 0i32 as f64
                    {
                        /* cur->args[0] = cur->args[0];  dx1 */
                        /* cur->args[1] = cur->args[1];  dy1 */
                        /* cur->args[2] = cur->args[2];  dx2 */
                        /* cur->args[3] = cur->args[3];  dy2 */
                        /* cur->args[4] = cur->args[4];  dx3 */
                        (*cur).args[5] = (*cur).args[6]; /* dx4 */
                        (*cur).args[6] = (*cur).args[8]; /* dx5 */
                        (*cur).args[7] = (*cur).args[9]; /* dy5 */
                        (*cur).args[8] = (*cur).args[10]; /* dx6 */
                        (*cur).num_args = 9i32;
                        (*cur).type_0 = 36i32
                    }
                }
            }
            -1 | 20 => {}
            _ => {
                panic!("Unexpected Type 2 charstring command {}.", (*cur).type_0);
            }
        }
        if !cur.is_null() {
            prev = cur
        }
        cur = next
    }
    /* Had no path. Fix lower-left point. */
    if (*cd).bbox.llx > (*cd).bbox.urx {
        (*cd).bbox.urx = (*cd).sbw.wx;
        (*cd).bbox.llx = (*cd).bbox.urx
    }
    if (*cd).bbox.lly > (*cd).bbox.ury {
        (*cd).bbox.ury = (*cd).sbw.wy;
        (*cd).bbox.lly = (*cd).bbox.ury
    };
}
#[no_mangle]
pub unsafe extern "C" fn t1char_get_metrics(
    mut src: *mut card8,
    mut srclen: i32,
    mut subrs: *mut cff_index,
    mut ginfo: *mut t1_ginfo,
) -> i32 {
    let mut t1char: t1_chardesc = t1_chardesc {
        flags: 0,
        sbw: C2RustUnnamed_3 {
            sbx: 0.,
            sby: 0.,
            wx: 0.,
            wy: 0.,
        },
        bbox: C2RustUnnamed_2 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_1 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
        num_stems: 0,
        stems: [t1_stem {
            id: 0,
            dir: 0,
            pos: 0.,
            del: 0.,
        }; 96],
        charpath: 0 as *mut t1_cpath,
        lastpath: 0 as *mut t1_cpath,
    };
    let mut cd: *mut t1_chardesc = 0 as *mut t1_chardesc;
    cd = &mut t1char;
    init_charpath(cd);
    status = 0i32;
    phase = 0i32;
    nest = 0i32;
    ps_stack_top = 0i32;
    cs_stack_top = 0i32;
    t1char_build_charpath(cd, &mut src, src.offset(srclen as isize), subrs);
    if cs_stack_top != 0i32 || ps_stack_top != 0i32 {
        warn!("Stack not empty. ({}, {})", cs_stack_top, ps_stack_top,);
    }
    do_postproc(cd);
    if !ginfo.is_null() {
        (*ginfo).wx = (*cd).sbw.wx;
        (*ginfo).wy = (*cd).sbw.wy;
        (*ginfo).bbox.llx = (*cd).bbox.llx;
        (*ginfo).bbox.lly = (*cd).bbox.lly;
        (*ginfo).bbox.urx = (*cd).bbox.urx;
        (*ginfo).bbox.ury = (*cd).bbox.ury;
        if (*cd).flags & 1i32 << 2i32 != 0 {
            (*ginfo).use_seac = 1i32;
            (*ginfo).seac.adx = (*cd).seac.adx;
            (*ginfo).seac.ady = (*cd).seac.ady;
            (*ginfo).seac.bchar = (*cd).seac.bchar;
            (*ginfo).seac.achar = (*cd).seac.achar
        } else {
            (*ginfo).use_seac = 0i32
        }
    }
    release_charpath(cd);
    0i32
}
/*
 * Encode Charpath as a Type 2 Charstring
 */
unsafe extern "C" fn t1char_encode_charpath(
    mut cd: *mut t1_chardesc,
    mut default_width: f64,
    mut nominal_width: f64,
    mut dst: *mut card8,
    mut endptr: *mut card8,
) -> i32 {
    let mut save: *mut card8 = 0 as *mut card8;
    let mut curr: *mut t1_cpath = 0 as *mut t1_cpath;
    assert!(!cd.is_null());
    save = dst;
    curr = (*cd).charpath;
    status = 0i32;
    phase = 0i32;
    nest = 0i32;
    ps_stack_top = 0i32;
    cs_stack_top = 0i32;
    /*
     * Advance Width
     */
    if (*cd).sbw.wx != default_width {
        let mut wx: f64 = (*cd).sbw.wx - nominal_width;
        put_numbers(&mut wx, 1i32, &mut dst, endptr);
        if status != 0i32 {
            panic!("Charstring encoder error: {}", status);
        }
    }
    /*
     * Hint Declaration
     */
    let mut num_hstems: i32 = 0i32;
    let mut num_vstems: i32 = 0i32;
    let mut i: i32 = 0;
    let mut reset: i32 = 1i32;
    let mut stem: [f64; 2] = [0.; 2];
    i = 0i32;
    while i < (*cd).num_stems && (*cd).stems[i as usize].dir == 0i32 {
        num_hstems += 1;
        stem[0] = if reset != 0 {
            (*cd).stems[i as usize].pos
        } else {
            (*cd).stems[i as usize].pos
                - ((*cd).stems[(i - 1i32) as usize].pos + (*cd).stems[(i - 1i32) as usize].del)
        };
        stem[1] = (*cd).stems[i as usize].del;
        put_numbers(stem.as_mut_ptr(), 2i32, &mut dst, endptr);
        if status != 0i32 {
            panic!("Charstring encoder error: {}", status);
        }
        reset = 0i32;
        if 2i32 * num_hstems > 48i32 - 3i32 {
            if dst.offset(1) >= endptr {
                panic!("Buffer overflow.");
            }
            let fresh23 = dst;
            dst = dst.offset(1);
            *fresh23 = (if (*cd).flags & 1i32 << 0i32 != 0 {
                18i32
            } else {
                1i32
            }) as card8;
            reset = 1i32
        }
        i += 1
    }
    if reset == 0i32 {
        if dst.offset(1) >= endptr {
            panic!("Buffer overflow.");
        }
        let fresh24 = dst;
        dst = dst.offset(1);
        *fresh24 = (if (*cd).flags & 1i32 << 0i32 != 0 {
            18i32
        } else {
            1i32
        }) as card8
    }
    reset = 1i32;
    if (*cd).num_stems - num_hstems > 0i32 {
        i = num_hstems;
        while i < (*cd).num_stems {
            num_vstems += 1;
            stem[0] = if reset != 0 {
                (*cd).stems[i as usize].pos
            } else {
                (*cd).stems[i as usize].pos
                    - ((*cd).stems[(i - 1i32) as usize].pos + (*cd).stems[(i - 1i32) as usize].del)
            };
            stem[1] = (*cd).stems[i as usize].del;
            put_numbers(stem.as_mut_ptr(), 2i32, &mut dst, endptr);
            if status != 0i32 {
                panic!("Charstring encoder error: {}", status);
            }
            reset = 0i32;
            if 2i32 * num_vstems > 48i32 - 3i32 {
                if dst.offset(1) >= endptr {
                    panic!("Buffer overflow.");
                }
                let fresh25 = dst;
                dst = dst.offset(1);
                *fresh25 = (if (*cd).flags & 1i32 << 0i32 != 0 {
                    23i32
                } else {
                    3i32
                }) as card8;
                reset = 1i32
            }
            i += 1
        }
        if reset == 0i32 {
            if dst.offset(1) >= endptr {
                panic!("Buffer overflow.");
            }
            if (*cd).flags & 1i32 << 0i32 != 0 || (*cd).flags & 1i32 << 1i32 != 0 {
                /*
                 * The vstem hint operator can be ommited if hstem and vstem hints
                 * are both declared at the beginning of a charstring, and is
                 * followed directly by the hintmask or cntrmask operators.
                 */
                if (*curr).type_0 != -1i32 && (*curr).type_0 != 20i32 {
                    let fresh26 = dst;
                    dst = dst.offset(1);
                    *fresh26 = 23i32 as card8
                }
            } else {
                let fresh27 = dst;
                dst = dst.offset(1);
                *fresh27 = 3i32 as card8
            }
        }
    }
    /*
     * Path Construction and Hint Replacement
     */
    while !curr.is_null() && (*curr).type_0 != 14i32 {
        match (*curr).type_0 {
            -1 => {
                let mut hintmask: [card8; 12] = [0; 12];
                memset(
                    hintmask.as_mut_ptr() as *mut libc::c_void,
                    0i32,
                    (((*cd).num_stems + 7i32) / 8i32) as u64,
                );
                while !curr.is_null() && (*curr).type_0 == -1i32 {
                    let mut stem_idx: i32 = 0;
                    stem_idx = get_stem(cd, (*curr).args[0] as i32);
                    assert!(stem_idx < (*cd).num_stems);
                    hintmask[(stem_idx / 8i32) as usize] =
                        (hintmask[(stem_idx / 8i32) as usize] as i32
                            | 1i32 << 7i32 - stem_idx % 8i32) as card8;
                    curr = (*curr).next
                }
                if (*cd).flags & 1i32 << 0i32 != 0 {
                    if dst.offset((((*cd).num_stems + 7i32) / 8i32 + 1i32) as isize) >= endptr {
                        panic!("Buffer overflow.");
                    }
                    let fresh28 = dst;
                    dst = dst.offset(1);
                    *fresh28 = 19i32 as card8;
                    memcpy(
                        dst as *mut libc::c_void,
                        hintmask.as_mut_ptr() as *const libc::c_void,
                        (((*cd).num_stems + 7i32) / 8i32) as u64,
                    );
                    dst = dst.offset((((*cd).num_stems + 7i32) / 8i32) as isize)
                }
            }
            20 => {
                let mut cntrmask: [card8; 12] = [0; 12];
                let mut stem_idx_0: i32 = 0;
                let mut i_0: i32 = 0;
                memset(
                    cntrmask.as_mut_ptr() as *mut libc::c_void,
                    0i32,
                    (((*cd).num_stems + 7i32) / 8i32) as u64,
                );
                i_0 = 0i32;
                while i_0 < (*curr).num_args {
                    stem_idx_0 = get_stem(cd, (*curr).args[i_0 as usize] as i32);
                    assert!(stem_idx_0 < (*cd).num_stems);
                    cntrmask[(stem_idx_0 / 8i32) as usize] =
                        (cntrmask[(stem_idx_0 / 8i32) as usize] as i32
                            | 1i32 << 7i32 - stem_idx_0 % 8i32) as card8;
                    i_0 += 1
                }
                if dst.offset((((*cd).num_stems + 7i32) / 8i32 + 1i32) as isize) >= endptr {
                    panic!("Buffer overflow.");
                }
                let fresh29 = dst;
                dst = dst.offset(1);
                *fresh29 = 20i32 as card8;
                memcpy(
                    dst as *mut libc::c_void,
                    cntrmask.as_mut_ptr() as *const libc::c_void,
                    (((*cd).num_stems + 7i32) / 8i32) as u64,
                );
                dst = dst.offset((((*cd).num_stems + 7i32) / 8i32) as isize);
                curr = (*curr).next
            }
            21 | 22 | 4 | 5 | 6 | 7 | 8 | 31 | 30 | 25 | 24 => {
                put_numbers(
                    (*curr).args.as_mut_ptr(),
                    (*curr).num_args,
                    &mut dst,
                    endptr,
                );
                if status != 0i32 {
                    panic!("Charstring encoder error: {}", status);
                }
                if dst.offset(1) >= endptr {
                    panic!("Buffer overflow.");
                }
                let fresh30 = dst;
                dst = dst.offset(1);
                *fresh30 = (*curr).type_0 as card8;
                curr = (*curr).next
            }
            35 | 34 | 36 => {
                put_numbers(
                    (*curr).args.as_mut_ptr(),
                    (*curr).num_args,
                    &mut dst,
                    endptr,
                );
                if status != 0i32 {
                    panic!("Charstring encoder error: {}", status);
                }
                if dst.offset(2) >= endptr {
                    panic!("Buffer overflow.");
                }
                let fresh31 = dst;
                dst = dst.offset(1);
                *fresh31 = 12i32 as card8;
                let fresh32 = dst;
                dst = dst.offset(1);
                *fresh32 = (*curr).type_0 as card8;
                curr = (*curr).next
            }
            _ => {
                panic!("Unknown Type 2 charstring command: {}", (*curr).type_0);
            }
        }
    }
    /*
     * (adx ady bchar achar) endchar
     */
    if (*cd).flags & 1i32 << 2i32 != 0 {
        let mut seac: [f64; 4] = [0.; 4];
        seac[0] = (*cd).seac.adx;
        seac[1] = (*cd).seac.ady;
        seac[2] = (*cd).seac.bchar as f64;
        seac[3] = (*cd).seac.achar as f64;
        put_numbers(seac.as_mut_ptr(), 4i32, &mut dst, endptr);
        if status != 0i32 {
            panic!("Charstring encoder error: {}", status);
        }
        if dst.offset(2) >= endptr {
            panic!("Buffer overflow.");
        }
        warn!("Obsolete four arguments of \"endchar\" will be used for Type 1 \"seac\" operator.");
    }
    if dst.offset(1) >= endptr {
        panic!("Buffer overflow.");
    }
    let fresh33 = dst;
    dst = dst.offset(1);
    *fresh33 = 14i32 as card8;
    dst.wrapping_offset_from(save) as i64 as i32
}
#[no_mangle]
pub unsafe extern "C" fn t1char_convert_charstring(
    mut dst: *mut card8,
    mut dstlen: i32,
    mut src: *mut card8,
    mut srclen: i32,
    mut subrs: *mut cff_index,
    mut default_width: f64,
    mut nominal_width: f64,
    mut ginfo: *mut t1_ginfo,
) -> i32 {
    let mut length: i32 = 0;
    let mut t1char: t1_chardesc = t1_chardesc {
        flags: 0,
        sbw: C2RustUnnamed_3 {
            sbx: 0.,
            sby: 0.,
            wx: 0.,
            wy: 0.,
        },
        bbox: C2RustUnnamed_2 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_1 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
        num_stems: 0,
        stems: [t1_stem {
            id: 0,
            dir: 0,
            pos: 0.,
            del: 0.,
        }; 96],
        charpath: 0 as *mut t1_cpath,
        lastpath: 0 as *mut t1_cpath,
    };
    let mut cd: *mut t1_chardesc = 0 as *mut t1_chardesc;
    cd = &mut t1char;
    init_charpath(cd);
    status = 0i32;
    phase = 0i32;
    nest = 0i32;
    ps_stack_top = 0i32;
    cs_stack_top = 0i32;
    t1char_build_charpath(cd, &mut src, src.offset(srclen as isize), subrs);
    if cs_stack_top != 0i32 || ps_stack_top != 0i32 {
        warn!("Stack not empty. ({}, {})", cs_stack_top, ps_stack_top,);
    }
    do_postproc(cd);
    qsort(
        (*cd).stems.as_mut_ptr() as *mut libc::c_void,
        (*cd).num_stems as size_t,
        ::std::mem::size_of::<t1_stem>() as u64,
        Some(
            stem_compare
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
    length = t1char_encode_charpath(
        cd,
        default_width,
        nominal_width,
        dst,
        dst.offset(dstlen as isize),
    );
    if !ginfo.is_null() {
        (*ginfo).wx = (*cd).sbw.wx;
        (*ginfo).wy = (*cd).sbw.wy;
        (*ginfo).bbox.llx = (*cd).bbox.llx;
        (*ginfo).bbox.lly = (*cd).bbox.lly;
        (*ginfo).bbox.urx = (*cd).bbox.urx;
        (*ginfo).bbox.ury = (*cd).bbox.ury;
        if (*cd).flags & 1i32 << 2i32 != 0 {
            (*ginfo).use_seac = 1i32;
            (*ginfo).seac.adx = (*cd).seac.adx;
            (*ginfo).seac.ady = (*cd).seac.ady;
            (*ginfo).seac.bchar = (*cd).seac.bchar;
            (*ginfo).seac.achar = (*cd).seac.achar
        } else {
            (*ginfo).use_seac = 0i32
        }
    }
    release_charpath(cd);
    length
}
