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
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
        format: tt_input_format_type,
        is_gz: i32,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(
        buffer: *mut i8,
        length: i32,
        file: rust_input_handle_t,
    ) -> *mut i8;
    #[no_mangle]
    fn pdf_dev_put_image(
        xobj_id: i32,
        p: *mut transform_info,
        ref_x: f64,
        ref_y: f64,
    ) -> i32;
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const i8, options: load_options) -> i32;
    #[no_mangle]
    fn mps_scan_bbox(
        pp: *mut *const i8,
        endptr: *const i8,
        bbox: *mut pdf_rect,
    ) -> i32;
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
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
pub type size_t = u64;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = u32;
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
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const i8,
    pub exec: spc_handler_fn_ptr,
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
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
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
pub struct load_options {
    pub page_no: i32,
    pub bbox_type: i32,
    pub dict: *mut pdf_obj,
}
/* quasi-hack to get the primary input */
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
unsafe extern "C" fn spc_handler_postscriptbox(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> i32 {
    let mut form_id: i32 = 0;
    let mut len: i32 = 0;
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
    let mut filename: [i8; 256] = [0; 256];
    let mut buf: [i8; 512] = [0; 512];
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(
            b"spe && ap\x00" as *const u8 as *const i8,
            b"dpx-spc_misc.c\x00" as *const u8 as *const i8,
            51i32 as u32,
            (*::std::mem::transmute::<&[u8; 66], &[i8; 66]>(
                b"int spc_handler_postscriptbox(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*ap).curptr >= (*ap).endptr {
        spc_warn(
            spe,
            b"No width/height/filename given for postscriptbox special.\x00" as *const u8
                as *const i8,
        );
        return -1i32;
    }
    /* input is not NULL terminated */
    len = (*ap).endptr.wrapping_offset_from((*ap).curptr) as i64 as i32;
    len = if 511i32 < len { 511i32 } else { len };
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        (*ap).curptr as *const libc::c_void,
        len as u64,
    );
    buf[len as usize] = '\u{0}' as i32 as i8;
    transform_info_clear(&mut ti);
    spc_warn(
        spe,
        b"%s\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
    );
    if sscanf(
        buf.as_mut_ptr(),
        b"{%lfpt}{%lfpt}{%255[^}]}\x00" as *const u8 as *const i8,
        &mut ti.width as *mut f64,
        &mut ti.height as *mut f64,
        filename.as_mut_ptr(),
    ) != 3i32
    {
        spc_warn(
            spe,
            b"Syntax error in postscriptbox special?\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    (*ap).curptr = (*ap).endptr;
    ti.width *= 72.0f64 / 72.27f64;
    ti.height *= 72.0f64 / 72.27f64;
    handle = ttstub_input_open(filename.as_mut_ptr(), TTIF_PICT, 0i32);
    if handle.is_null() {
        spc_warn(
            spe,
            b"Could not open image file: %s\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return -1i32;
    }
    ti.flags |= 1i32 << 1i32 | 1i32 << 2i32;
    loop {
        let mut p: *const i8 = tt_mfgets(buf.as_mut_ptr(), 512i32, handle);
        if p.is_null() {
            break;
        }
        if !(mps_scan_bbox(&mut p, p.offset(strlen(p) as isize), &mut ti.bbox) >= 0i32) {
            continue;
        }
        ti.flags |= 1i32 << 0i32;
        break;
    }
    ttstub_input_close(handle);
    form_id = pdf_ximage_findresource(filename.as_mut_ptr(), options);
    if form_id < 0i32 {
        spc_warn(
            spe,
            b"Failed to load image file: %s\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return -1i32;
    }
    pdf_dev_put_image(form_id, &mut ti, (*spe).x_user, (*spe).y_user);
    return 0i32;
}
unsafe extern "C" fn spc_handler_null(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    (*args).curptr = (*args).endptr;
    return 0i32;
}
static mut misc_handlers: [spc_handler; 6] = unsafe {
    [
        {
            let mut init = spc_handler {
                key: b"postscriptbox\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_postscriptbox
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"landscape\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"papersize\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"src:\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"pos:\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"om:\x00" as *const u8 as *const i8,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_misc_check_special(
    mut buffer: *const i8,
    mut size: i32,
) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut i: size_t = 0;
    p = buffer;
    endptr = p.offset(size as isize);
    skip_white(&mut p, endptr);
    size = endptr.wrapping_offset_from(p) as i64 as i32;
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 6]>() as u64)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
    {
        if size as u64 >= strlen(misc_handlers[i as usize].key)
            && strncmp(
                p,
                misc_handlers[i as usize].key,
                strlen(misc_handlers[i as usize].key),
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
pub unsafe extern "C" fn spc_misc_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> i32 {
    let mut key: *const i8 = 0 as *const i8;
    let mut keylen: i32 = 0;
    let mut i: size_t = 0;
    if !handle.is_null() && !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(b"handle && spe && args\x00" as *const u8 as
                          *const i8,
                      b"dpx-spc_misc.c\x00" as *const u8 as
                          *const i8, 156i32 as u32,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[i8; 85]>(b"int spc_misc_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    key = (*args).curptr;
    while (*args).curptr < (*args).endptr
        && *(*__ctype_b_loc())
            .offset(*(*args).curptr.offset(0) as u8 as i32 as isize)
            as i32
            & _ISalpha as i32 as u16 as i32
            != 0
    {
        (*args).curptr = (*args).curptr.offset(1)
    }
    if (*args).curptr < (*args).endptr && *(*args).curptr.offset(0) as i32 == ':' as i32 {
        (*args).curptr = (*args).curptr.offset(1)
    }
    keylen = (*args).curptr.wrapping_offset_from(key) as i64 as i32;
    if keylen < 1i32 {
        return -1i32;
    }
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 6]>() as u64)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as u64)
    {
        if keylen as u64 == strlen(misc_handlers[i as usize].key)
            && strncmp(key, misc_handlers[i as usize].key, keylen as u64) == 0
        {
            skip_white(&mut (*args).curptr, (*args).endptr);
            (*args).command = misc_handlers[i as usize].key;
            (*handle).key = b"???:\x00" as *const u8 as *const i8;
            (*handle).exec = misc_handlers[i as usize].exec;
            return 0i32;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}
