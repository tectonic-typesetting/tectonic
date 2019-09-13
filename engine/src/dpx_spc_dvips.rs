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
use crate::strstartswith;
use crate::warn;

use super::dpx_pdfdraw::pdf_dev_concat;
use crate::dpx_pdfobj::pdf_obj;
use crate::{ttstub_input_close, ttstub_input_open};
use libc::free;
extern "C" {
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
    /* Here is the complete list of PDF object types */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn pdf_dev_put_image(xobj_id: i32, p: *mut transform_info, ref_x: f64, ref_y: f64) -> i32;
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const i8, options: load_options) -> i32;
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
    fn mps_exec_inline(buffer: *mut *const i8, endptr: *const i8, x_user: f64, y_user: f64) -> i32;
    #[no_mangle]
    fn mps_stack_depth() -> i32;
    #[no_mangle]
    fn mps_eop_cleanup();
    #[no_mangle]
    fn pdf_dev_gsave() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore() -> i32;
    /* The depth here is the depth of q/Q nesting.
     * We must remember current depth of nesting when starting a page or xform,
     * and must recover until that depth at the end of page/xform.
     */
    #[no_mangle]
    fn pdf_dev_current_depth() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore_to(depth: i32);
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    #[no_mangle]
    fn spc_util_read_dimtrns(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        args: *mut spc_arg,
        syntax: i32,
    ) -> i32;
}
pub type size_t = u64;

use crate::TTInputFormat;

pub type rust_input_handle_t = *mut libc::c_void;
/* quasi-hack to get the primary input */
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

use super::dpx_specials::{spc_arg, spc_env};

pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const i8,
    pub exec: spc_handler_fn_ptr,
}

use super::dpx_pdfdev::pdf_tmatrix;

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
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: i32,
    pub bbox_type: i32,
    pub dict: *mut pdf_obj,
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
static mut block_pending: i32 = 0i32;
static mut pending_x: f64 = 0.0f64;
static mut pending_y: f64 = 0.0f64;
static mut position_set: i32 = 0i32;
static mut ps_headers: *mut *mut i8 = 0 as *const *mut i8 as *mut *mut i8;
static mut num_ps_headers: i32 = 0i32;
unsafe extern "C" fn spc_handler_ps_header(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut pro: *mut i8 = 0 as *mut i8;
    let mut ps_header: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(1) >= (*args).endptr || *(*args).curptr.offset(0) as i32 != '=' as i32
    {
        spc_warn(
            spe,
            b"No filename specified for PSfile special.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*args).curptr = (*args).curptr.offset(1);
    pro = xmalloc(
        ((*args).endptr.wrapping_offset_from((*args).curptr) as i64 + 1i32 as i64) as size_t,
    ) as *mut i8;
    strncpy(
        pro,
        (*args).curptr,
        (*args).endptr.wrapping_offset_from((*args).curptr) as i64 as u64,
    );
    *pro.offset((*args).endptr.wrapping_offset_from((*args).curptr) as i64 as isize) = 0_i8;
    ps_header =
        ttstub_input_open(pro, TTInputFormat::TEX_PS_HEADER, 0i32) as *mut rust_input_handle_t;
    if ps_header.is_null() {
        spc_warn(
            spe,
            b"PS header %s not found.\x00" as *const u8 as *const i8,
            pro,
        );
        free(pro as *mut libc::c_void);
        return -1i32;
    }
    ttstub_input_close(ps_header as rust_input_handle_t);
    if num_ps_headers & 0xfi32 == 0 {
        ps_headers = xrealloc(
            ps_headers as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64).wrapping_mul((num_ps_headers + 16i32) as u64),
        ) as *mut *mut i8
    }
    let fresh0 = num_ps_headers;
    num_ps_headers = num_ps_headers + 1;
    let ref mut fresh1 = *ps_headers.offset(fresh0 as isize);
    *fresh1 = pro;
    (*args).curptr = (*args).endptr;
    0i32
}
unsafe extern "C" fn parse_filename(mut pp: *mut *const i8, mut endptr: *const i8) -> *mut i8 {
    let mut r: *mut i8 = 0 as *mut i8;
    let mut q: *const i8 = 0 as *const i8;
    let mut p: *const i8 = *pp;
    let mut qchar: i8 = 0;
    let mut n: i32 = 0;
    if p.is_null() || p >= endptr {
        return 0 as *mut i8;
    } else {
        if *p as i32 == '\"' as i32 || *p as i32 == '\'' as i32 {
            let fresh2 = p;
            p = p.offset(1);
            qchar = *fresh2
        } else {
            qchar = ' ' as i32 as i8
        }
    }
    n = 0i32;
    q = p;
    while p < endptr && *p as i32 != qchar as i32 {
        /* nothing */
        n += 1;
        p = p.offset(1)
    }
    if qchar as i32 != ' ' as i32 {
        if *p as i32 != qchar as i32 {
            return 0 as *mut i8;
        }
        p = p.offset(1)
    }
    if q.is_null() || n == 0i32 {
        return 0 as *mut i8;
    }
    r = new(((n + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    memcpy(r as *mut libc::c_void, q as *const libc::c_void, n as u64);
    *r.offset(n as isize) = '\u{0}' as i32 as i8;
    *pp = p;
    r
}
/* =filename ... */
unsafe extern "C" fn spc_handler_ps_file(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut form_id: i32 = 0;
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    assert!(!spe.is_null() && !args.is_null());
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(1) >= (*args).endptr || *(*args).curptr.offset(0) as i32 != '=' as i32
    {
        spc_warn(
            spe,
            b"No filename specified for PSfile special.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*args).curptr = (*args).curptr.offset(1);
    filename = parse_filename(&mut (*args).curptr, (*args).endptr);
    if filename.is_null() {
        spc_warn(
            spe,
            b"No filename specified for PSfile special.\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    transform_info_clear(&mut ti);
    if spc_util_read_dimtrns(spe, &mut ti, args, 1i32) < 0i32 {
        free(filename as *mut libc::c_void);
        return -1i32;
    }
    form_id = pdf_ximage_findresource(filename, options);
    if form_id < 0i32 {
        spc_warn(
            spe,
            b"Failed to read image file: %s\x00" as *const u8 as *const i8,
            filename,
        );
        free(filename as *mut libc::c_void);
        return -1i32;
    }
    free(filename as *mut libc::c_void);
    pdf_dev_put_image(form_id, &mut ti, (*spe).x_user, (*spe).y_user);
    0i32
}
/* This isn't correct implementation but dvipdfm supports... */
unsafe extern "C" fn spc_handler_ps_plotfile(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut error: i32 = 0i32; /* xscale = 1.0, yscale = -1.0 */
    let mut form_id: i32 = 0;
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut p: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    assert!(!spe.is_null() && !args.is_null());
    spc_warn(
        spe,
        b"\"ps: plotfile\" found (not properly implemented)\x00" as *const u8 as *const i8,
    );
    skip_white(&mut (*args).curptr, (*args).endptr);
    filename = parse_filename(&mut (*args).curptr, (*args).endptr);
    if filename.is_null() {
        spc_warn(
            spe,
            b"Expecting filename but not found...\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    form_id = pdf_ximage_findresource(filename, options);
    if form_id < 0i32 {
        spc_warn(
            spe,
            b"Could not open PS file: %s\x00" as *const u8 as *const i8,
            filename,
        );
        error = -1i32
    } else {
        transform_info_clear(&mut p);
        p.matrix.d = -1.0f64;
        pdf_dev_put_image(form_id, &mut p, 0i32 as f64, 0i32 as f64);
    }
    free(filename as *mut libc::c_void);
    error
}
unsafe extern "C" fn spc_handler_ps_literal(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut error: i32 = 0i32;
    let mut st_depth: i32 = 0;
    let mut gs_depth: i32 = 0;
    let mut x_user: f64 = 0.;
    let mut y_user: f64 = 0.;
    assert!(!spe.is_null() && !args.is_null() && (*args).curptr <= (*args).endptr);
    if (*args)
        .curptr
        .offset(strlen(b":[begin]\x00" as *const u8 as *const i8) as isize)
        <= (*args).endptr
        && !strstartswith((*args).curptr, b":[begin]\x00" as *const u8 as *const i8).is_null()
    {
        block_pending += 1;
        position_set = 1i32;
        pending_x = (*spe).x_user;
        x_user = pending_x;
        pending_y = (*spe).y_user;
        y_user = pending_y;
        (*args).curptr = (*args)
            .curptr
            .offset(strlen(b":[begin]\x00" as *const u8 as *const i8) as isize)
    } else if (*args)
        .curptr
        .offset(strlen(b":[end]\x00" as *const u8 as *const i8) as isize)
        <= (*args).endptr
        && !strstartswith((*args).curptr, b":[end]\x00" as *const u8 as *const i8).is_null()
    {
        if block_pending <= 0i32 {
            spc_warn(
                spe,
                b"No corresponding ::[begin] found.\x00" as *const u8 as *const i8,
            );
            return -1i32;
        }
        block_pending -= 1;
        position_set = 0i32;
        x_user = pending_x;
        y_user = pending_y;
        (*args).curptr = (*args)
            .curptr
            .offset(strlen(b":[end]\x00" as *const u8 as *const i8) as isize)
    } else if (*args).curptr < (*args).endptr && *(*args).curptr.offset(0) as i32 == ':' as i32 {
        x_user = if position_set != 0 {
            pending_x
        } else {
            (*spe).x_user
        };
        y_user = if position_set != 0 {
            pending_y
        } else {
            (*spe).y_user
        };
        (*args).curptr = (*args).curptr.offset(1)
    } else {
        position_set = 1i32;
        pending_x = (*spe).x_user;
        x_user = pending_x;
        pending_y = (*spe).y_user;
        y_user = pending_y
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        st_depth = mps_stack_depth();
        gs_depth = pdf_dev_current_depth();
        error = mps_exec_inline(&mut (*args).curptr, (*args).endptr, x_user, y_user);
        if error != 0 {
            spc_warn(
                spe,
                b"Interpreting PS code failed!!! Output might be broken!!!\x00" as *const u8
                    as *const i8,
            );
            pdf_dev_grestore_to(gs_depth);
        } else if st_depth != mps_stack_depth() {
            spc_warn(
                spe,
                b"Stack not empty after execution of inline PostScript code.\x00" as *const u8
                    as *const i8,
            );
            spc_warn(spe,
                     b">> Your macro package makes some assumption on internal behaviour of DVI drivers.\x00"
                         as *const u8 as *const i8);
            spc_warn(
                spe,
                b">> It may not compatible with dvipdfmx.\x00" as *const u8 as *const i8,
            );
        }
    }
    error
}
unsafe extern "C" fn spc_handler_ps_trickscmd(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    warn!("PSTricks commands are disallowed in Tectonic");
    (*args).curptr = (*args).endptr;
    -1i32
}
unsafe extern "C" fn spc_handler_ps_tricksobj(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    warn!("PSTricks commands are disallowed in Tectonic");
    (*args).curptr = (*args).endptr;
    -1i32
}
unsafe extern "C" fn spc_handler_ps_default(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    let mut error: i32 = 0;
    let mut st_depth: i32 = 0;
    let mut gs_depth: i32 = 0;
    assert!(!spe.is_null() && !args.is_null());
    pdf_dev_gsave();
    st_depth = mps_stack_depth();
    gs_depth = pdf_dev_current_depth();
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.d = 1.0f64;
    M.a = M.d;
    M.c = 0.0f64;
    M.b = M.c;
    M.e = (*spe).x_user;
    M.f = (*spe).y_user;
    pdf_dev_concat(&mut M);
    error = mps_exec_inline(
        &mut (*args).curptr,
        (*args).endptr,
        (*spe).x_user,
        (*spe).y_user,
    );
    M.e = -(*spe).x_user;
    M.f = -(*spe).y_user;
    pdf_dev_concat(&mut M);
    if error != 0 {
        spc_warn(
            spe,
            b"Interpreting PS code failed!!! Output might be broken!!!\x00" as *const u8
                as *const i8,
        );
    } else if st_depth != mps_stack_depth() {
        spc_warn(
            spe,
            b"Stack not empty after execution of inline PostScript code.\x00" as *const u8
                as *const i8,
        );
        spc_warn(
            spe,
            b">> Your macro package makes some assumption on internal behaviour of DVI drivers.\x00"
                as *const u8 as *const i8,
        );
        spc_warn(
            spe,
            b">> It may not compatible with dvipdfmx.\x00" as *const u8 as *const i8,
        );
    }
    pdf_dev_grestore_to(gs_depth);
    pdf_dev_grestore();
    error
}
static mut dvips_handlers: [spc_handler; 10] = [
    {
        let mut init = spc_handler {
            key: b"header\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_header
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"PSfile\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_file
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"psfile\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_file
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"ps: plotfile \x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_plotfile
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"PS: plotfile \x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_plotfile
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"PS:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_literal
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"ps:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_literal
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"PST:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_trickscmd
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"pst:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_tricksobj
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"\" \x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_ps_default
                    as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
];

#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_begin_document() -> i32 {
    /* This function used to start the global_defs temp file. */
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_end_document() -> i32 {
    if !ps_headers.is_null() {
        while num_ps_headers > 0i32 {
            num_ps_headers -= 1;
            free(*ps_headers.offset(num_ps_headers as isize) as *mut libc::c_void);
        }
        ps_headers = mfree(ps_headers as *mut libc::c_void) as *mut *mut i8
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_begin_page() -> i32 {
    /* This function used do some things related to now-removed PSTricks functionality. */
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_end_page() -> i32 {
    mps_eop_cleanup();
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_check_special(mut buf: *const i8, mut len: i32) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut i: size_t = 0;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p >= endptr {
        return false;
    }
    len = endptr.wrapping_offset_from(p) as i64 as i32;
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 10]>() as u64)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
    {
        if len as u64 >= strlen(dvips_handlers[i as usize].key)
            && memcmp(
                p as *const libc::c_void,
                dvips_handlers[i as usize].key as *const libc::c_void,
                strlen(dvips_handlers[i as usize].key),
            ) == 0
        {
            return true;
        }
        i = i.wrapping_add(1)
    }
    false
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
pub unsafe extern "C" fn spc_dvips_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    let mut key: *const i8 = 0 as *const i8;
    let mut keylen: i32 = 0;
    let mut i: size_t = 0;
    assert!(!handle.is_null() && !spe.is_null() && !args.is_null());
    skip_white(&mut (*args).curptr, (*args).endptr);
    key = (*args).curptr;
    while (*args).curptr < (*args).endptr && libc::isalpha(*(*args).curptr.offset(0) as _) != 0 {
        (*args).curptr = (*args).curptr.offset(1)
    }
    /* Test for "ps:". The "ps::" special is subsumed under this case.  */
    if (*args).curptr < (*args).endptr && *(*args).curptr.offset(0) as i32 == ':' as i32 {
        (*args).curptr = (*args).curptr.offset(1);
        if (*args)
            .curptr
            .offset(strlen(b" plotfile \x00" as *const u8 as *const i8) as isize)
            <= (*args).endptr
            && !strstartswith((*args).curptr, b" plotfile \x00" as *const u8 as *const i8).is_null()
        {
            (*args).curptr = (*args)
                .curptr
                .offset(strlen(b" plotfile \x00" as *const u8 as *const i8) as isize)
        }
    } else if (*args).curptr.offset(1) < (*args).endptr
        && *(*args).curptr.offset(0) as i32 == '\"' as i32
        && *(*args).curptr.offset(1) as i32 == ' ' as i32
    {
        (*args).curptr = (*args).curptr.offset(2)
    }
    keylen = (*args).curptr.wrapping_offset_from(key) as i64 as i32;
    if keylen < 1i32 {
        spc_warn(spe, b"Not ps: special???\x00" as *const u8 as *const i8);
        return -1i32;
    }
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 10]>() as u64)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
    {
        if keylen as u64 == strlen(dvips_handlers[i as usize].key)
            && strncmp(key, dvips_handlers[i as usize].key, keylen as u64) == 0
        {
            skip_white(&mut (*args).curptr, (*args).endptr);
            (*args).command = dvips_handlers[i as usize].key;
            (*handle).key = b"ps:\x00" as *const u8 as *const i8;
            (*handle).exec = dvips_handlers[i as usize].exec;
            return 0i32;
        }
        i = i.wrapping_add(1)
    }
    -1i32
}
