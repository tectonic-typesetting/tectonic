#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    pub type pst_obj;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn skip_white_spaces(s: *mut *mut u8, endptr: *mut u8);
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
    fn new(size: u32) -> *mut libc::c_void;
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
    fn pst_parse_null(inbuf: *mut *mut u8, inbufend: *mut u8) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_boolean(inbuf: *mut *mut u8, inbufend: *mut u8) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_name(inbuf: *mut *mut u8, inbufend: *mut u8) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_number(inbuf: *mut *mut u8, inbufend: *mut u8) -> *mut pst_obj;
    #[no_mangle]
    fn pst_parse_string(inbuf: *mut *mut u8, inbufend: *mut u8) -> *mut pst_obj;
}

pub type pst_type = i32;
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
unsafe extern "C" fn pst_parse_any(mut inbuf: *mut *mut u8, mut inbufend: *mut u8) -> *mut pst_obj {
    let mut data: *mut u8 = 0 as *mut u8;
    let mut cur: *mut u8 = *inbuf;
    let mut len: u32 = 0;
    while cur < inbufend
        && !(cur == inbufend
            || (*cur as i32 == '(' as i32
                || *cur as i32 == ')' as i32
                || *cur as i32 == '/' as i32
                || *cur as i32 == '<' as i32
                || *cur as i32 == '>' as i32
                || *cur as i32 == '[' as i32
                || *cur as i32 == ']' as i32
                || *cur as i32 == '{' as i32
                || *cur as i32 == '}' as i32
                || *cur as i32 == '%' as i32)
            || (*cur as i32 == ' ' as i32
                || *cur as i32 == '\t' as i32
                || *cur as i32 == '\u{c}' as i32
                || *cur as i32 == '\r' as i32
                || *cur as i32 == '\n' as i32
                || *cur as i32 == '\u{0}' as i32))
    {
        cur = cur.offset(1)
    }
    len = cur.wrapping_offset_from(*inbuf) as i64 as u32;
    data = new(
        (len.wrapping_add(1_u32) as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
    ) as *mut u8;
    memcpy(
        data as *mut libc::c_void,
        *inbuf as *const libc::c_void,
        len as u64,
    );
    *data.offset(len as isize) = '\u{0}' as i32 as u8;
    *inbuf = cur;
    pst_new_obj(-1i32, data as *mut libc::c_void)
}
unsafe extern "C" fn skip_line(mut inbuf: *mut *mut u8, mut inbufend: *mut u8) {
    while *inbuf < inbufend && **inbuf as i32 != '\n' as i32 && **inbuf as i32 != '\r' as i32 {
        *inbuf = (*inbuf).offset(1)
    }
    if *inbuf < inbufend && **inbuf as i32 == '\r' as i32 {
        *inbuf = (*inbuf).offset(1)
    }
    if *inbuf < inbufend && **inbuf as i32 == '\n' as i32 {
        *inbuf = (*inbuf).offset(1)
    };
}
unsafe extern "C" fn skip_comments(mut inbuf: *mut *mut u8, mut inbufend: *mut u8) {
    while *inbuf < inbufend && **inbuf as i32 == '%' as i32 {
        skip_line(inbuf, inbufend);
        skip_white_spaces(inbuf, inbufend);
    }
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
pub unsafe extern "C" fn pst_get_token(
    mut inbuf: *mut *mut u8,
    mut inbufend: *mut u8,
) -> *mut pst_obj {
    let mut obj: *mut pst_obj = 0 as *mut pst_obj;
    let mut c: u8 = 0;
    assert!(*inbuf <= inbufend && *inbufend == 0);
    skip_white_spaces(inbuf, inbufend);
    skip_comments(inbuf, inbufend);
    if *inbuf >= inbufend {
        return 0 as *mut pst_obj;
    }
    c = **inbuf;
    match c as i32 {
        47 => obj = pst_parse_name(inbuf, inbufend),
        91 | 123 => {
            /* This is wrong */
            obj = pst_new_mark();
            *inbuf = (*inbuf).offset(1)
        }
        60 => {
            if (*inbuf).offset(1) >= inbufend {
                return 0 as *mut pst_obj;
            }
            c = *(*inbuf).offset(1);
            if c as i32 == '<' as i32 {
                obj = pst_new_mark();
                *inbuf = (*inbuf).offset(2)
            } else if libc::isxdigit(c as _) != 0 {
                obj = pst_parse_string(inbuf, inbufend)
            } else if c as i32 == '~' as i32 {
                /* ASCII85 */
                obj = pst_parse_string(inbuf, inbufend)
            }
        }
        40 => obj = pst_parse_string(inbuf, inbufend),
        62 => {
            if (*inbuf).offset(1) >= inbufend || *(*inbuf).offset(1) as i32 != '>' as i32 {
                _tt_abort(
                    b"Unexpected end of ASCII hex string marker.\x00" as *const u8 as *const i8,
                );
            } else {
                let mut mark: *mut i8 = 0 as *mut i8;
                mark =
                    new((3_u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
                *mark.offset(0) = '>' as i32 as i8;
                *mark.offset(1) = '>' as i32 as i8;
                *mark.offset(2) = '\u{0}' as i32 as i8;
                obj = pst_new_obj(-1i32, mark as *mut libc::c_void);
                *inbuf = (*inbuf).offset(2)
            }
        }
        93 | 125 => {
            let mut mark_0: *mut i8 = 0 as *mut i8;
            mark_0 =
                new((2_u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
            *mark_0.offset(0) = c as i8;
            *mark_0.offset(1) = '\u{0}' as i32 as i8;
            obj = pst_new_obj(-1i32, mark_0 as *mut libc::c_void);
            *inbuf = (*inbuf).offset(1)
        }
        _ => {
            if c as i32 == 't' as i32 || c as i32 == 'f' as i32 {
                obj = pst_parse_boolean(inbuf, inbufend)
            } else if c as i32 == 'n' as i32 {
                obj = pst_parse_null(inbuf, inbufend)
            } else if c as i32 == '+' as i32
                || c as i32 == '-' as i32
                || libc::isdigit(c as _) != 0
                || c as i32 == '.' as i32
            {
                obj = pst_parse_number(inbuf, inbufend)
            }
        }
    }
    if obj.is_null() {
        obj = pst_parse_any(inbuf, inbufend)
    }
    obj
}
