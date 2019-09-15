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

use super::dpx_mpost::mps_scan_bbox;
use super::dpx_pdfdev::{
    pdf_dev_put_image, pdf_rect, pdf_tmatrix, transform_info, transform_info_clear,
};
use super::dpx_pdfximage::pdf_ximage_findresource;
use crate::dpx_pdfobj::pdf_obj;
use crate::{ttstub_input_close, ttstub_input_open};
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const i8, _: ...);
    #[no_mangle]
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(buffer: *mut i8, length: i32, file: rust_input_handle_t) -> *mut i8;
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
}

pub type size_t = u64;

use crate::TTInputFormat;

pub type rust_input_handle_t = *mut libc::c_void;

use super::dpx_specials::{spc_arg, spc_env};

pub type spc_handler_fn_ptr = Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32>;
use super::dpx_specials::spc_handler;

use crate::dpx_pdfximage::load_options;

/* quasi-hack to get the primary input */
unsafe extern "C" fn spc_handler_postscriptbox(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> i32 {
    let mut form_id: i32 = 0;
    let mut len: i32 = 0;
    let mut ti = transform_info::new();
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
    assert!(!spe.is_null() && !ap.is_null());
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
    spc_warn(spe, b"%s\x00" as *const u8 as *const i8, buf.as_mut_ptr());
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
    handle = ttstub_input_open(filename.as_mut_ptr(), TTInputFormat::PICT, 0i32);
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
    0i32
}
unsafe extern "C" fn spc_handler_null(mut spe: *mut spc_env, mut args: *mut spc_arg) -> i32 {
    (*args).curptr = (*args).endptr;
    0i32
}
static mut misc_handlers: [spc_handler; 6] = [
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
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"papersize\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"src:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"pos:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
    {
        let mut init = spc_handler {
            key: b"om:\x00" as *const u8 as *const i8,
            exec: Some(
                spc_handler_null as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> i32,
            ),
        };
        init
    },
];

#[no_mangle]
pub unsafe extern "C" fn spc_misc_check_special(mut buffer: *const i8, mut size: i32) -> bool {
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
            return true;
        }
        i = i.wrapping_add(1)
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn spc_misc_setup_handler(
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
    -1i32
}
