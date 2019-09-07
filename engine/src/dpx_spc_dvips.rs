#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
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
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn pdf_dev_put_image(
        xobj_id: libc::c_int,
        p: *mut transform_info,
        ref_x: libc::c_double,
        ref_y: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const libc::c_char, options: load_options) -> libc::c_int;
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
    fn mps_exec_inline(
        buffer: *mut *const libc::c_char,
        endptr: *const libc::c_char,
        x_user: libc::c_double,
        y_user: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn mps_stack_depth() -> libc::c_int;
    #[no_mangle]
    fn mps_eop_cleanup();
    #[no_mangle]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_gsave() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_grestore() -> libc::c_int;
    /* The depth here is the depth of q/Q nesting.
     * We must remember current depth of nesting when starting a page or xform,
     * and must recover until that depth at the end of page/xform.
     */
    #[no_mangle]
    fn pdf_dev_current_depth() -> libc::c_int;
    #[no_mangle]
    fn pdf_dev_grestore_to(depth: libc::c_int);
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn spc_util_read_dimtrns(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        args: *mut spc_arg,
        syntax: libc::c_int,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
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
pub struct transform_info {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub depth: libc::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: libc::c_int,
    pub bbox_type: libc::c_int,
    pub dict: *mut pdf_obj,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const libc::c_char;
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
static mut block_pending: libc::c_int = 0i32;
static mut pending_x: libc::c_double = 0.0f64;
static mut pending_y: libc::c_double = 0.0f64;
static mut position_set: libc::c_int = 0i32;
static mut ps_headers: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
static mut num_ps_headers: libc::c_int = 0i32;
unsafe extern "C" fn spc_handler_ps_header(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut pro: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ps_header: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(1) >= (*args).endptr
        || *(*args).curptr.offset(0) as libc::c_int != '=' as i32
    {
        spc_warn(
            spe,
            b"No filename specified for PSfile special.\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    (*args).curptr = (*args).curptr.offset(1);
    pro = xmalloc(
        ((*args).endptr.wrapping_offset_from((*args).curptr) as libc::c_long + 1i32 as libc::c_long)
            as size_t,
    ) as *mut libc::c_char;
    strncpy(
        pro,
        (*args).curptr,
        (*args).endptr.wrapping_offset_from((*args).curptr) as libc::c_long as libc::c_ulong,
    );
    *pro.offset((*args).endptr.wrapping_offset_from((*args).curptr) as libc::c_long as isize) =
        0i32 as libc::c_char;
    ps_header = ttstub_input_open(pro, TTIF_TEX_PS_HEADER, 0i32) as *mut rust_input_handle_t;
    if ps_header.is_null() {
        spc_warn(
            spe,
            b"PS header %s not found.\x00" as *const u8 as *const libc::c_char,
            pro,
        );
        free(pro as *mut libc::c_void);
        return -1i32;
    }
    ttstub_input_close(ps_header as rust_input_handle_t);
    if num_ps_headers & 0xfi32 == 0 {
        ps_headers = xrealloc(
            ps_headers as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((num_ps_headers + 16i32) as libc::c_ulong),
        ) as *mut *mut libc::c_char
    }
    let fresh0 = num_ps_headers;
    num_ps_headers = num_ps_headers + 1;
    let ref mut fresh1 = *ps_headers.offset(fresh0 as isize);
    *fresh1 = pro;
    (*args).curptr = (*args).endptr;
    return 0i32;
}
unsafe extern "C" fn parse_filename(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = *pp;
    let mut qchar: libc::c_char = 0;
    let mut n: libc::c_int = 0;
    if p.is_null() || p >= endptr {
        return 0 as *mut libc::c_char;
    } else {
        if *p as libc::c_int == '\"' as i32 || *p as libc::c_int == '\'' as i32 {
            let fresh2 = p;
            p = p.offset(1);
            qchar = *fresh2
        } else {
            qchar = ' ' as i32 as libc::c_char
        }
    }
    n = 0i32;
    q = p;
    while p < endptr && *p as libc::c_int != qchar as libc::c_int {
        /* nothing */
        n += 1;
        p = p.offset(1)
    }
    if qchar as libc::c_int != ' ' as i32 {
        if *p as libc::c_int != qchar as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        p = p.offset(1)
    }
    if q.is_null() || n == 0i32 {
        return 0 as *mut libc::c_char;
    }
    r = new(((n + 1i32) as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_char;
    memcpy(
        r as *mut libc::c_void,
        q as *const libc::c_void,
        n as libc::c_ulong,
    );
    *r.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    *pp = p;
    return r;
}
/* =filename ... */
unsafe extern "C" fn spc_handler_ps_file(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut form_id: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
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
    if !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(
            b"spe && args\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_dvips.c\x00" as *const u8 as *const libc::c_char,
            140i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                b"int spc_handler_ps_file(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(1) >= (*args).endptr
        || *(*args).curptr.offset(0) as libc::c_int != '=' as i32
    {
        spc_warn(
            spe,
            b"No filename specified for PSfile special.\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    (*args).curptr = (*args).curptr.offset(1);
    filename = parse_filename(&mut (*args).curptr, (*args).endptr);
    if filename.is_null() {
        spc_warn(
            spe,
            b"No filename specified for PSfile special.\x00" as *const u8 as *const libc::c_char,
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
            b"Failed to read image file: %s\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        free(filename as *mut libc::c_void);
        return -1i32;
    }
    free(filename as *mut libc::c_void);
    pdf_dev_put_image(form_id, &mut ti, (*spe).x_user, (*spe).y_user);
    return 0i32;
}
/* This isn't correct implementation but dvipdfm supports... */
unsafe extern "C" fn spc_handler_ps_plotfile(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32; /* xscale = 1.0, yscale = -1.0 */
    let mut form_id: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
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
    if !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(
            b"spe && args\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_dvips.c\x00" as *const u8 as *const libc::c_char,
            185i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"int spc_handler_ps_plotfile(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    spc_warn(
        spe,
        b"\"ps: plotfile\" found (not properly implemented)\x00" as *const u8
            as *const libc::c_char,
    );
    skip_white(&mut (*args).curptr, (*args).endptr);
    filename = parse_filename(&mut (*args).curptr, (*args).endptr);
    if filename.is_null() {
        spc_warn(
            spe,
            b"Expecting filename but not found...\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    form_id = pdf_ximage_findresource(filename, options);
    if form_id < 0i32 {
        spc_warn(
            spe,
            b"Could not open PS file: %s\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        error = -1i32
    } else {
        transform_info_clear(&mut p);
        p.matrix.d = -1.0f64;
        pdf_dev_put_image(
            form_id,
            &mut p,
            0i32 as libc::c_double,
            0i32 as libc::c_double,
        );
    }
    free(filename as *mut libc::c_void);
    return error;
}
unsafe extern "C" fn spc_handler_ps_literal(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut st_depth: libc::c_int = 0;
    let mut gs_depth: libc::c_int = 0;
    let mut x_user: libc::c_double = 0.;
    let mut y_user: libc::c_double = 0.;
    if !spe.is_null() && !args.is_null() && (*args).curptr <= (*args).endptr {
    } else {
        __assert_fail(
            b"spe && args && args->curptr <= args->endptr\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_dvips.c\x00" as *const u8 as *const libc::c_char,
            218i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"int spc_handler_ps_literal(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*args)
        .curptr
        .offset(strlen(b":[begin]\x00" as *const u8 as *const libc::c_char) as isize)
        <= (*args).endptr
        && !strstartswith(
            (*args).curptr,
            b":[begin]\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
    {
        block_pending += 1;
        position_set = 1i32;
        pending_x = (*spe).x_user;
        x_user = pending_x;
        pending_y = (*spe).y_user;
        y_user = pending_y;
        (*args).curptr = (*args)
            .curptr
            .offset(strlen(b":[begin]\x00" as *const u8 as *const libc::c_char) as isize)
    } else if (*args)
        .curptr
        .offset(strlen(b":[end]\x00" as *const u8 as *const libc::c_char) as isize)
        <= (*args).endptr
        && !strstartswith(
            (*args).curptr,
            b":[end]\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
    {
        if block_pending <= 0i32 {
            spc_warn(
                spe,
                b"No corresponding ::[begin] found.\x00" as *const u8 as *const libc::c_char,
            );
            return -1i32;
        }
        block_pending -= 1;
        position_set = 0i32;
        x_user = pending_x;
        y_user = pending_y;
        (*args).curptr = (*args)
            .curptr
            .offset(strlen(b":[end]\x00" as *const u8 as *const libc::c_char) as isize)
    } else if (*args).curptr < (*args).endptr
        && *(*args).curptr.offset(0) as libc::c_int == ':' as i32
    {
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
                    as *const libc::c_char,
            );
            pdf_dev_grestore_to(gs_depth);
        } else if st_depth != mps_stack_depth() {
            spc_warn(
                spe,
                b"Stack not empty after execution of inline PostScript code.\x00" as *const u8
                    as *const libc::c_char,
            );
            spc_warn(spe,
                     b">> Your macro package makes some assumption on internal behaviour of DVI drivers.\x00"
                         as *const u8 as *const libc::c_char);
            spc_warn(
                spe,
                b">> It may not compatible with dvipdfmx.\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    return error;
}
unsafe extern "C" fn spc_handler_ps_trickscmd(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    dpx_warning(
        b"PSTricks commands are disallowed in Tectonic\x00" as *const u8 as *const libc::c_char,
    );
    (*args).curptr = (*args).endptr;
    return -1i32;
}
unsafe extern "C" fn spc_handler_ps_tricksobj(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    dpx_warning(
        b"PSTricks commands are disallowed in Tectonic\x00" as *const u8 as *const libc::c_char,
    );
    (*args).curptr = (*args).endptr;
    return -1i32;
}
unsafe extern "C" fn spc_handler_ps_default(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    let mut st_depth: libc::c_int = 0;
    let mut gs_depth: libc::c_int = 0;
    if !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(
            b"spe && args\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_dvips.c\x00" as *const u8 as *const libc::c_char,
            291i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"int spc_handler_ps_default(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
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
                as *const libc::c_char,
        );
    } else if st_depth != mps_stack_depth() {
        spc_warn(
            spe,
            b"Stack not empty after execution of inline PostScript code.\x00" as *const u8
                as *const libc::c_char,
        );
        spc_warn(
            spe,
            b">> Your macro package makes some assumption on internal behaviour of DVI drivers.\x00"
                as *const u8 as *const libc::c_char,
        );
        spc_warn(
            spe,
            b">> It may not compatible with dvipdfmx.\x00" as *const u8 as *const libc::c_char,
        );
    }
    pdf_dev_grestore_to(gs_depth);
    pdf_dev_grestore();
    return error;
}
static mut dvips_handlers: [spc_handler; 10] = unsafe {
    [
        {
            let mut init = spc_handler {
                key: b"header\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_header
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"PSfile\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_file
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"psfile\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_file
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ps: plotfile \x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_plotfile
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"PS: plotfile \x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_plotfile
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"PS:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_literal
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"ps:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_literal
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"PST:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_trickscmd
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"pst:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_tricksobj
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"\" \x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_ps_default
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_begin_document() -> libc::c_int {
    /* This function used to start the global_defs temp file. */
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_end_document() -> libc::c_int {
    if !ps_headers.is_null() {
        while num_ps_headers > 0i32 {
            num_ps_headers -= 1;
            free(*ps_headers.offset(num_ps_headers as isize) as *mut libc::c_void);
        }
        ps_headers = mfree(ps_headers as *mut libc::c_void) as *mut *mut libc::c_char
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_begin_page() -> libc::c_int {
    /* This function used do some things related to now-removed PSTricks functionality. */
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_at_end_page() -> libc::c_int {
    mps_eop_cleanup();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn spc_dvips_check_special(
    mut buf: *const libc::c_char,
    mut len: libc::c_int,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&mut p, endptr);
    if p >= endptr {
        return 0i32 != 0;
    }
    len = endptr.wrapping_offset_from(p) as libc::c_long as libc::c_int;
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as libc::c_ulong)
    {
        if len as libc::c_ulong >= strlen(dvips_handlers[i as usize].key)
            && memcmp(
                p as *const libc::c_void,
                dvips_handlers[i as usize].key as *const libc::c_void,
                strlen(dvips_handlers[i as usize].key),
            ) == 0
        {
            return 1i32 != 0;
        }
        i = i.wrapping_add(1)
    }
    return 0i32 != 0;
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
) -> libc::c_int {
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut keylen: libc::c_int = 0;
    let mut i: size_t = 0;
    if !handle.is_null() && !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(b"handle && spe && args\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-spc_dvips.c\x00" as *const u8 as
                          *const libc::c_char, 402i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"int spc_dvips_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    key = (*args).curptr;
    while (*args).curptr < (*args).endptr
        && *(*__ctype_b_loc())
            .offset(*(*args).curptr.offset(0) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*args).curptr = (*args).curptr.offset(1)
    }
    /* Test for "ps:". The "ps::" special is subsumed under this case.  */
    if (*args).curptr < (*args).endptr && *(*args).curptr.offset(0) as libc::c_int == ':' as i32 {
        (*args).curptr = (*args).curptr.offset(1);
        if (*args)
            .curptr
            .offset(strlen(b" plotfile \x00" as *const u8 as *const libc::c_char) as isize)
            <= (*args).endptr
            && !strstartswith(
                (*args).curptr,
                b" plotfile \x00" as *const u8 as *const libc::c_char,
            )
            .is_null()
        {
            (*args).curptr = (*args)
                .curptr
                .offset(strlen(b" plotfile \x00" as *const u8 as *const libc::c_char) as isize)
        }
    } else if (*args).curptr.offset(1) < (*args).endptr
        && *(*args).curptr.offset(0) as libc::c_int == '\"' as i32
        && *(*args).curptr.offset(1) as libc::c_int == ' ' as i32
    {
        (*args).curptr = (*args).curptr.offset(2)
    }
    keylen = (*args).curptr.wrapping_offset_from(key) as libc::c_long as libc::c_int;
    if keylen < 1i32 {
        spc_warn(
            spe,
            b"Not ps: special???\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as libc::c_ulong)
    {
        if keylen as libc::c_ulong == strlen(dvips_handlers[i as usize].key)
            && strncmp(key, dvips_handlers[i as usize].key, keylen as libc::c_ulong) == 0
        {
            skip_white(&mut (*args).curptr, (*args).endptr);
            (*args).command = dvips_handlers[i as usize].key;
            (*handle).key = b"ps:\x00" as *const u8 as *const libc::c_char;
            (*handle).exec = dvips_handlers[i as usize].exec;
            return 0i32;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}
