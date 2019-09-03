#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types, label_break_value, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type pst_obj;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn skip_white_spaces(s: *mut *mut libc::c_uchar,
                         endptr: *mut libc::c_uchar);
    #[no_mangle]
    fn pst_new_obj(type_0: pst_type, data: *mut libc::c_void) -> *mut pst_obj;
    #[no_mangle]
    fn pst_new_mark() -> *mut pst_obj;
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
    fn pst_parse_null(inbuf: *mut *mut libc::c_uchar,
                      inbufend: *mut libc::c_uchar) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_boolean(inbuf: *mut *mut libc::c_uchar,
                         inbufend: *mut libc::c_uchar) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_name(inbuf: *mut *mut libc::c_uchar,
                      inbufend: *mut libc::c_uchar) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_number(inbuf: *mut *mut libc::c_uchar,
                        inbufend: *mut libc::c_uchar) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_string(inbuf: *mut *mut libc::c_uchar,
                        inbufend: *mut libc::c_uchar) -> *mut pst_obj;
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
pub type pst_type = libc::c_int;
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
unsafe extern "C" fn pst_parse_any(mut inbuf: *mut *mut libc::c_uchar,
                                   mut inbufend: *mut libc::c_uchar)
 -> *mut pst_obj {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cur: *mut libc::c_uchar = *inbuf;
    let mut len: libc::c_uint = 0;
    while cur < inbufend &&
              !(cur == inbufend ||
                    (*cur as libc::c_int == '(' as i32 ||
                         *cur as libc::c_int == ')' as i32 ||
                         *cur as libc::c_int == '/' as i32 ||
                         *cur as libc::c_int == '<' as i32 ||
                         *cur as libc::c_int == '>' as i32 ||
                         *cur as libc::c_int == '[' as i32 ||
                         *cur as libc::c_int == ']' as i32 ||
                         *cur as libc::c_int == '{' as i32 ||
                         *cur as libc::c_int == '}' as i32 ||
                         *cur as libc::c_int == '%' as i32) ||
                    (*cur as libc::c_int == ' ' as i32 ||
                         *cur as libc::c_int == '\t' as i32 ||
                         *cur as libc::c_int == '\u{c}' as i32 ||
                         *cur as libc::c_int == '\r' as i32 ||
                         *cur as libc::c_int == '\n' as i32 ||
                         *cur as libc::c_int == '\u{0}' as i32)) {
        cur = cur.offset(1)
    }
    len = cur.wrapping_offset_from(*inbuf) as libc::c_long as libc::c_uint;
    data =
        new((len.wrapping_add(1i32 as libc::c_uint) as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_uchar;
    memcpy(data as *mut libc::c_void, *inbuf as *const libc::c_void,
           len as libc::c_ulong);
    *data.offset(len as isize) = '\u{0}' as i32 as libc::c_uchar;
    *inbuf = cur;
    return pst_new_obj(-1i32, data as *mut libc::c_void);
}
unsafe extern "C" fn skip_line(mut inbuf: *mut *mut libc::c_uchar,
                               mut inbufend: *mut libc::c_uchar) {
    while *inbuf < inbufend && **inbuf as libc::c_int != '\n' as i32 &&
              **inbuf as libc::c_int != '\r' as i32 {
        *inbuf = (*inbuf).offset(1)
    }
    if *inbuf < inbufend && **inbuf as libc::c_int == '\r' as i32 {
        *inbuf = (*inbuf).offset(1)
    }
    if *inbuf < inbufend && **inbuf as libc::c_int == '\n' as i32 {
        *inbuf = (*inbuf).offset(1)
    };
}
unsafe extern "C" fn skip_comments(mut inbuf: *mut *mut libc::c_uchar,
                                   mut inbufend: *mut libc::c_uchar) {
    while *inbuf < inbufend && **inbuf as libc::c_int == '%' as i32 {
        skip_line(inbuf, inbufend);
        skip_white_spaces(inbuf, inbufend);
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
/* NOTE: the input buffer must be null-terminated, i.e., *inbufend == 0 */
#[no_mangle]
pub unsafe extern "C" fn pst_get_token(mut inbuf: *mut *mut libc::c_uchar,
                                       mut inbufend: *mut libc::c_uchar)
 -> *mut pst_obj {
    let mut obj: *mut pst_obj = 0 as *mut pst_obj;
    let mut c: libc::c_uchar = 0;
    if *inbuf <= inbufend && *inbufend == 0 {
    } else {
        __assert_fail(b"*inbuf <= inbufend && !*inbufend\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-pst.c\x00" as *const u8 as *const libc::c_char,
                      87i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"pst_obj *pst_get_token(unsigned char **, unsigned char *)\x00")).as_ptr());
    }
    skip_white_spaces(inbuf, inbufend);
    skip_comments(inbuf, inbufend);
    if *inbuf >= inbufend { return 0 as *mut pst_obj }
    c = **inbuf;
    match c as libc::c_int {
        47 => { obj = pst_parse_name(inbuf, inbufend) }
        91 | 123 => {
            /* This is wrong */
            obj = pst_new_mark();
            *inbuf = (*inbuf).offset(1)
        }
        60 => {
            if (*inbuf).offset(1) >= inbufend { return 0 as *mut pst_obj }
            c = *(*inbuf).offset(1);
            if c as libc::c_int == '<' as i32 {
                obj = pst_new_mark();
                *inbuf = (*inbuf).offset(2)
            } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                          libc::c_int &
                          _ISxdigit as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
                obj = pst_parse_string(inbuf, inbufend)
            } else if c as libc::c_int == '~' as i32 {
                /* ASCII85 */
                obj = pst_parse_string(inbuf, inbufend)
            }
        }
        40 => { obj = pst_parse_string(inbuf, inbufend) }
        62 => {
            if (*inbuf).offset(1) >= inbufend ||
                   *(*inbuf).offset(1) as libc::c_int != '>' as i32 {
                _tt_abort(b"Unexpected end of ASCII hex string marker.\x00" as
                              *const u8 as *const libc::c_char);
            } else {
                let mut mark: *mut libc::c_char = 0 as *mut libc::c_char;
                mark =
                    new((3i32 as uint32_t as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                             as libc::c_ulong)
                            as uint32_t) as *mut libc::c_char;
                *mark.offset(0) = '>' as i32 as libc::c_char;
                *mark.offset(1) = '>' as i32 as libc::c_char;
                *mark.offset(2) = '\u{0}' as i32 as libc::c_char;
                obj = pst_new_obj(-1i32, mark as *mut libc::c_void);
                *inbuf = (*inbuf).offset(2)
            }
        }
        93 | 125 => {
            let mut mark_0: *mut libc::c_char = 0 as *mut libc::c_char;
            mark_0 =
                new((2i32 as uint32_t as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                         as libc::c_ulong) as
                        uint32_t) as *mut libc::c_char;
            *mark_0.offset(0) = c as libc::c_char;
            *mark_0.offset(1) = '\u{0}' as i32 as libc::c_char;
            obj = pst_new_obj(-1i32, mark_0 as *mut libc::c_void);
            *inbuf = (*inbuf).offset(1)
        }
        _ => {
            if c as libc::c_int == 't' as i32 ||
                   c as libc::c_int == 'f' as i32 {
                obj = pst_parse_boolean(inbuf, inbufend)
            } else if c as libc::c_int == 'n' as i32 {
                obj = pst_parse_null(inbuf, inbufend)
            } else if c as libc::c_int == '+' as i32 ||
                          c as libc::c_int == '-' as i32 ||
                          *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                         isize) as libc::c_int
                              &
                              _ISdigit as libc::c_int as libc::c_ushort as
                                  libc::c_int != 0 ||
                          c as libc::c_int == '.' as i32 {
                obj = pst_parse_number(inbuf, inbufend)
            }
        }
    }
    if obj.is_null() { obj = pst_parse_any(inbuf, inbufend) }
    return obj;
}
