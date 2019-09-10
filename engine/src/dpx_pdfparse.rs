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
    pub type pdf_file;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_indirect(
        pf: *mut pdf_file,
        label: libc::c_uint,
        generation: libc::c_ushort,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_null() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_boolean(value: libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_number_value(number: *mut pdf_obj) -> libc::c_double;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    #[no_mangle]
    fn xtoi(c: libc::c_char) -> libc::c_int;
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
    fn dpx_message(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
    /* PDF parser shouldn't depend on this...
     */
    #[no_mangle]
    fn spc_lookup_reference(ident: *const libc::c_char) -> *mut pdf_obj;
}
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
pub type size_t = u64;
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
/* pow() */
/* PDF */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tainted: libc::c_int,
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
static mut parser_state: C2RustUnnamed_0 = {
    let mut init = C2RustUnnamed_0 { tainted: 0i32 };
    init
};
static mut save: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn dump(mut start: *const libc::c_char, mut end: *const libc::c_char) {
    let mut p: *const libc::c_char = start;
    dpx_message(b"\nCurrent input buffer is -->\x00" as *const u8 as *const libc::c_char);
    while p < end && p < start.offset(50) {
        let fresh0 = p;
        p = p.offset(1);
        dpx_message(
            b"%c\x00" as *const u8 as *const libc::c_char,
            *fresh0 as libc::c_int,
        );
    }
    if p == start.offset(50) {
        dpx_message(b"...\x00" as *const u8 as *const libc::c_char);
    }
    dpx_message(b"<--\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn pdfparse_skip_line(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) {
    while *start < end
        && **start as libc::c_int != '\n' as i32
        && **start as libc::c_int != '\r' as i32
    {
        *start = (*start).offset(1)
    }
    /* The carriage return (CR; \r; 0x0D) and line feed (LF; \n; 0x0A)
     * characters, also called newline characters, are treated as
     * end-of-line (EOL) markers. The combination of a carriage return
     * followed immediately by a line feed is treated as one EOL marker.
     */
    if *start < end && **start as libc::c_int == '\r' as i32 {
        *start = (*start).offset(1)
    }
    if *start < end && **start as libc::c_int == '\n' as i32 {
        *start = (*start).offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn skip_white(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) {
    /*
     * The null (NUL; 0x00) character is a white-space character in PDF spec
     * but isspace(0x00) returns FALSE; on the other hand, the vertical tab
     * (VT; 0x0B) character is not a white-space character in PDF spec but
     * isspace(0x0B) returns TRUE.
     */
    while *start < end
        && (**start as libc::c_int == ' ' as i32
            || **start as libc::c_int == '\t' as i32
            || **start as libc::c_int == '\u{c}' as i32
            || **start as libc::c_int == '\r' as i32
            || **start as libc::c_int == '\n' as i32
            || **start as libc::c_int == '\u{0}' as i32
            || **start as libc::c_int == '%' as i32)
    {
        if **start as libc::c_int == '%' as i32 {
            pdfparse_skip_line(start, end);
        } else {
            *start = (*start).offset(1)
        }
    }
}
unsafe extern "C" fn parsed_string(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = end.wrapping_offset_from(start) as libc::c_long as libc::c_int;
    if len > 0i32 {
        result = new(((len + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
            as u32) as *mut libc::c_char;
        memcpy(
            result as *mut libc::c_void,
            start as *const libc::c_void,
            len as u64,
        );
        *result.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn parse_number(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    let mut number: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    skip_white(start, end);
    p = *start;
    if p < end && (*p as libc::c_int == '+' as i32 || *p as libc::c_int == '-' as i32) {
        p = p.offset(1)
    }
    while p < end
        && *(*__ctype_b_loc()).offset(*p as u8 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1)
    }
    if p < end && *p as libc::c_int == '.' as i32 {
        p = p.offset(1);
        while p < end
            && *(*__ctype_b_loc()).offset(*p as u8 as libc::c_int as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1)
        }
    }
    number = parsed_string(*start, p);
    *start = p;
    return number;
}
#[no_mangle]
pub unsafe extern "C" fn parse_unsigned(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    let mut number: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    skip_white(start, end);
    p = *start;
    while p < end {
        if *(*__ctype_b_loc()).offset(*p as u8 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            break;
        }
        p = p.offset(1)
    }
    number = parsed_string(*start, p);
    *start = p;
    return number;
}
unsafe extern "C" fn parse_gen_ident(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
    mut valid_chars: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ident: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* No skip_white(start, end)? */
    p = *start;
    while p < end {
        if strchr(valid_chars, *p as libc::c_int).is_null() {
            break;
        }
        p = p.offset(1)
    }
    ident = parsed_string(*start, p);
    *start = p;
    return ident;
}
#[no_mangle]
pub unsafe extern "C" fn parse_ident(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    static mut valid_chars: *const libc::c_char =
        b"!\"#$&\'*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\x00"
            as *const u8 as *const libc::c_char;
    return parse_gen_ident(start, end, valid_chars);
}
#[no_mangle]
pub unsafe extern "C" fn parse_val_ident(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    static mut valid_chars: *const libc::c_char =
        b"!\"#$&\'*+,-./0123456789:;?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\x00"
            as *const u8 as *const libc::c_char;
    return parse_gen_ident(start, end, valid_chars);
}
#[no_mangle]
pub unsafe extern "C" fn parse_opt_ident(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    if *start < end && **start as libc::c_int == '@' as i32 {
        *start = (*start).offset(1);
        return parse_ident(start, end);
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_number(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut v: libc::c_double = 0.0f64;
    let mut nddigits: libc::c_int = 0i32;
    let mut sign: libc::c_int = 1i32;
    let mut has_dot: libc::c_int = 0i32;
    p = *pp;
    skip_white(&mut p, endptr);
    if p >= endptr
        || *(*__ctype_b_loc()).offset(*p.offset(0) as u8 as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
            && *p.offset(0) as libc::c_int != '.' as i32
            && *p.offset(0) as libc::c_int != '+' as i32
            && *p.offset(0) as libc::c_int != '-' as i32
    {
        dpx_warning(b"Could not find a numeric object.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    }
    if *p.offset(0) as libc::c_int == '-' as i32 {
        if p.offset(1) >= endptr {
            dpx_warning(
                b"Could not find a numeric object.\x00" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut pdf_obj;
        }
        sign = -1i32;
        p = p.offset(1)
    } else if *p.offset(0) as libc::c_int == '+' as i32 {
        if p.offset(1) >= endptr {
            dpx_warning(
                b"Could not find a numeric object.\x00" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut pdf_obj;
        }
        sign = 1i32;
        p = p.offset(1)
    }
    while p < endptr
        && !(*p.offset(0) as libc::c_int == ' ' as i32
            || *p.offset(0) as libc::c_int == '\t' as i32
            || *p.offset(0) as libc::c_int == '\u{c}' as i32
            || *p.offset(0) as libc::c_int == '\r' as i32
            || *p.offset(0) as libc::c_int == '\n' as i32
            || *p.offset(0) as libc::c_int == '\u{0}' as i32
            || (*p.offset(0) as libc::c_int == '(' as i32
                || *p.offset(0) as libc::c_int == ')' as i32
                || *p.offset(0) as libc::c_int == '/' as i32
                || *p.offset(0) as libc::c_int == '<' as i32
                || *p.offset(0) as libc::c_int == '>' as i32
                || *p.offset(0) as libc::c_int == '[' as i32
                || *p.offset(0) as libc::c_int == ']' as i32
                || *p.offset(0) as libc::c_int == '%' as i32))
    {
        if *p.offset(0) as libc::c_int == '.' as i32 {
            if has_dot != 0 {
                /* Two dots */
                dpx_warning(
                    b"Could not find a numeric object.\x00" as *const u8 as *const libc::c_char,
                );
                return 0 as *mut pdf_obj;
            } else {
                has_dot = 1i32
            }
        } else if *(*__ctype_b_loc()).offset(*p.offset(0) as u8 as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            if has_dot != 0 {
                v += (*p.offset(0) as libc::c_int - '0' as i32) as libc::c_double
                    / pow(10i32 as libc::c_double, (nddigits + 1i32) as libc::c_double);
                nddigits += 1
            } else {
                v = v * 10.0f64 + *p.offset(0) as libc::c_int as libc::c_double
                    - '0' as i32 as libc::c_double
            }
        } else {
            dpx_warning(
                b"Could not find a numeric object.\x00" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut pdf_obj;
        }
        p = p.offset(1)
    }
    *pp = p;
    return pdf_new_number(sign as libc::c_double * v);
}
/*
 * PDF Name:
 *
 *  PDF-1.2+: Two hexadecimal digits preceded by a number sign.
 */
unsafe extern "C" fn pn_getc(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> libc::c_int {
    let mut ch: libc::c_int = 0i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = *pp;
    if *p.offset(0) as libc::c_int == '#' as i32 {
        if p.offset(2) >= endptr {
            *pp = endptr;
            return -1i32;
        }
        if *(*__ctype_b_loc()).offset(*p.offset(1) as u8 as libc::c_int as isize)
            as libc::c_int
            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
            || *(*__ctype_b_loc()).offset(*p.offset(2) as u8 as libc::c_int as isize)
                as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            *pp = (*pp).offset(3);
            return -1i32;
        }
        ch = xtoi(*p.offset(1)) << 4i32;
        ch += xtoi(*p.offset(2));
        *pp = (*pp).offset(3)
    } else {
        ch = *p.offset(0) as libc::c_int;
        *pp = (*pp).offset(1)
    }
    return ch;
}
static mut sbuf: [libc::c_char; 65536] = [0; 65536];
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_name(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    let mut name: [libc::c_char; 129] = [0; 129];
    let mut ch: libc::c_int = 0;
    let mut len: libc::c_int = 0i32;
    skip_white(pp, endptr);
    if *pp >= endptr || **pp as libc::c_int != '/' as i32 {
        dpx_warning(b"Could not find a name object.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    }
    *pp = (*pp).offset(1);
    while *pp < endptr
        && !(**pp as libc::c_int == ' ' as i32
            || **pp as libc::c_int == '\t' as i32
            || **pp as libc::c_int == '\u{c}' as i32
            || **pp as libc::c_int == '\r' as i32
            || **pp as libc::c_int == '\n' as i32
            || **pp as libc::c_int == '\u{0}' as i32
            || (**pp as libc::c_int == '(' as i32
                || **pp as libc::c_int == ')' as i32
                || **pp as libc::c_int == '/' as i32
                || **pp as libc::c_int == '<' as i32
                || **pp as libc::c_int == '>' as i32
                || **pp as libc::c_int == '[' as i32
                || **pp as libc::c_int == ']' as i32
                || **pp as libc::c_int == '%' as i32))
    {
        ch = pn_getc(pp, endptr);
        if ch < 0i32 || ch > 0xffi32 {
            dpx_warning(
                b"Invalid char in PDF name object. (ignored)\x00" as *const u8
                    as *const libc::c_char,
            );
        } else if ch == 0i32 {
            dpx_warning(
                b"Null char not allowed in PDF name object. (ignored)\x00" as *const u8
                    as *const libc::c_char,
            );
        } else if len < 65535i32 + 1i32 {
            if len == 128i32 {
                dpx_warning(
                    b"PDF name length too long. (>= %d bytes)\x00" as *const u8
                        as *const libc::c_char,
                    128i32,
                );
            }
            let fresh1 = len;
            len = len + 1;
            name[fresh1 as usize] = ch as libc::c_char
        } else {
            dpx_warning(
                b"PDF name length too long. (>= %d bytes, truncated)\x00" as *const u8
                    as *const libc::c_char,
                65535i32 + 1i32,
            );
        }
    }
    if len < 1i32 {
        dpx_warning(b"No valid name object found.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    }
    name[len as usize] = '\u{0}' as i32 as libc::c_char;
    return pdf_new_name(name.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_boolean(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(4) <= endptr
        && !strstartswith(*pp, b"true\x00" as *const u8 as *const libc::c_char).is_null()
    {
        if (*pp).offset(4) == endptr
            || (*(*pp).offset(4) as libc::c_int == ' ' as i32
                || *(*pp).offset(4) as libc::c_int == '\t' as i32
                || *(*pp).offset(4) as libc::c_int == '\u{c}' as i32
                || *(*pp).offset(4) as libc::c_int == '\r' as i32
                || *(*pp).offset(4) as libc::c_int == '\n' as i32
                || *(*pp).offset(4) as libc::c_int == '\u{0}' as i32
                || (*(*pp).offset(4) as libc::c_int == '(' as i32
                    || *(*pp).offset(4) as libc::c_int == ')' as i32
                    || *(*pp).offset(4) as libc::c_int == '/' as i32
                    || *(*pp).offset(4) as libc::c_int == '<' as i32
                    || *(*pp).offset(4) as libc::c_int == '>' as i32
                    || *(*pp).offset(4) as libc::c_int == '[' as i32
                    || *(*pp).offset(4) as libc::c_int == ']' as i32
                    || *(*pp).offset(4) as libc::c_int == '%' as i32))
        {
            *pp = (*pp).offset(4);
            return pdf_new_boolean(1i32 as libc::c_char);
        }
    } else if (*pp).offset(5) <= endptr
        && !strstartswith(*pp, b"false\x00" as *const u8 as *const libc::c_char).is_null()
    {
        if (*pp).offset(5) == endptr
            || (*(*pp).offset(5) as libc::c_int == ' ' as i32
                || *(*pp).offset(5) as libc::c_int == '\t' as i32
                || *(*pp).offset(5) as libc::c_int == '\u{c}' as i32
                || *(*pp).offset(5) as libc::c_int == '\r' as i32
                || *(*pp).offset(5) as libc::c_int == '\n' as i32
                || *(*pp).offset(5) as libc::c_int == '\u{0}' as i32
                || (*(*pp).offset(5) as libc::c_int == '(' as i32
                    || *(*pp).offset(5) as libc::c_int == ')' as i32
                    || *(*pp).offset(5) as libc::c_int == '/' as i32
                    || *(*pp).offset(5) as libc::c_int == '<' as i32
                    || *(*pp).offset(5) as libc::c_int == '>' as i32
                    || *(*pp).offset(5) as libc::c_int == '[' as i32
                    || *(*pp).offset(5) as libc::c_int == ']' as i32
                    || *(*pp).offset(5) as libc::c_int == '%' as i32))
        {
            *pp = (*pp).offset(5);
            return pdf_new_boolean(0i32 as libc::c_char);
        }
    }
    dpx_warning(b"Not a boolean object.\x00" as *const u8 as *const libc::c_char);
    return 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_null(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(4) > endptr {
        dpx_warning(b"Not a null object.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    } else {
        if (*pp).offset(4) < endptr
            && !(*(*pp).offset(4) as libc::c_int == ' ' as i32
                || *(*pp).offset(4) as libc::c_int == '\t' as i32
                || *(*pp).offset(4) as libc::c_int == '\u{c}' as i32
                || *(*pp).offset(4) as libc::c_int == '\r' as i32
                || *(*pp).offset(4) as libc::c_int == '\n' as i32
                || *(*pp).offset(4) as libc::c_int == '\u{0}' as i32
                || (*(*pp).offset(4) as libc::c_int == '(' as i32
                    || *(*pp).offset(4) as libc::c_int == ')' as i32
                    || *(*pp).offset(4) as libc::c_int == '/' as i32
                    || *(*pp).offset(4) as libc::c_int == '<' as i32
                    || *(*pp).offset(4) as libc::c_int == '>' as i32
                    || *(*pp).offset(4) as libc::c_int == '[' as i32
                    || *(*pp).offset(4) as libc::c_int == ']' as i32
                    || *(*pp).offset(4) as libc::c_int == '%' as i32))
        {
            dpx_warning(b"Not a null object.\x00" as *const u8 as *const libc::c_char);
            return 0 as *mut pdf_obj;
        } else {
            if !strstartswith(*pp, b"null\x00" as *const u8 as *const libc::c_char).is_null() {
                *pp = (*pp).offset(4);
                return pdf_new_null();
            }
        }
    }
    dpx_warning(b"Not a null object.\x00" as *const u8 as *const libc::c_char);
    return 0 as *mut pdf_obj;
}
/*
 * PDF Literal String
 */
unsafe extern "C" fn ps_getescc(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> libc::c_int {
    let mut ch: libc::c_int = 0; /* backslash assumed. */
    let mut i: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = (*pp).offset(1);
    match *p.offset(0) as libc::c_int {
        110 => {
            ch = '\n' as i32;
            p = p.offset(1)
        }
        114 => {
            ch = '\r' as i32;
            p = p.offset(1)
        }
        116 => {
            ch = '\t' as i32;
            p = p.offset(1)
        }
        98 => {
            ch = '\u{8}' as i32;
            p = p.offset(1)
        }
        102 => {
            ch = '\u{c}' as i32;
            p = p.offset(1)
        }
        10 => {
            /*
             * An end-of-line marker preceded by a backslash must be ignored.
             */
            ch = -1i32;
            p = p.offset(1)
        }
        13 => {
            ch = -1i32;
            p = p.offset(1);
            if p < endptr && *p.offset(0) as libc::c_int == '\n' as i32 {
                p = p.offset(1)
            }
        }
        _ => {
            if *p.offset(0) as libc::c_int == '\\' as i32
                || *p.offset(0) as libc::c_int == '(' as i32
                || *p.offset(0) as libc::c_int == ')' as i32
            {
                ch = *p.offset(0) as libc::c_int;
                p = p.offset(1)
            } else if *p.offset(0) as libc::c_int >= '0' as i32
                && *p.offset(0) as libc::c_int <= '7' as i32
            {
                ch = 0i32;
                /* Ignore overflow. */
                i = 0i32;
                while i < 3i32
                    && p < endptr
                    && (*p.offset(0) as libc::c_int >= '0' as i32
                        && *p.offset(0) as libc::c_int <= '7' as i32)
                {
                    ch = (ch << 3i32) + (*p.offset(0) as libc::c_int - '0' as i32);
                    p = p.offset(1);
                    i += 1
                }
                ch = ch & 0xffi32
            } else {
                /* Don't forget isodigit() is a macro. */
                ch = *p.offset(0) as u8 as libc::c_int; /* Ignore only backslash. */
                p = p.offset(1)
            }
        }
    }
    *pp = p;
    return ch;
}
unsafe extern "C" fn parse_pdf_literal_string(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    let mut ch: libc::c_int = 0;
    let mut op_count: libc::c_int = 0i32;
    let mut len: libc::c_int = 0i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = *pp;
    skip_white(&mut p, endptr);
    if p >= endptr || *p.offset(0) as libc::c_int != '(' as i32 {
        return 0 as *mut pdf_obj;
    }
    p = p.offset(1);
    /* The carriage return (CR, 0x0d) and line feed (LF, 0x0a) characters,
     * also called newline characters, are treated as end-of-line (EOL)
     * markers. The combination of a carriage return followed immediately
     * by a line feed is treated as one EOL marker.
     * [PDF Reference, 6th ed., version 1.7, p. 50] */
    /* If an end-of-line marker appears within a literal string
     * without a preceding backslash, the result is equivalent to
     * \n (regardless of whether the end-of-line marker was
     * a carriage return, a line feed, or both).
     * [PDF Reference, 6th ed., version 1.7, p. 55] */
    while p < endptr {
        ch = *p.offset(0) as libc::c_int;
        if ch == ')' as i32 && op_count < 1i32 {
            break;
        }
        if parser_state.tainted != 0 {
            if p.offset(1) < endptr && ch & 0x80i32 != 0 {
                if len + 2i32 >= 65535i32 {
                    dpx_warning(
                        b"PDF string length too long. (limit: %d)\x00" as *const u8
                            as *const libc::c_char,
                        65535i32,
                    );
                    return 0 as *mut pdf_obj;
                }
                let fresh2 = len;
                len = len + 1;
                sbuf[fresh2 as usize] = *p.offset(0);
                let fresh3 = len;
                len = len + 1;
                sbuf[fresh3 as usize] = *p.offset(1);
                p = p.offset(2);
                continue;
            }
        }
        /* !PDF_PARSE_STRICT */
        if len + 1i32 >= 65535i32 {
            dpx_warning(
                b"PDF string length too long. (limit: %d)\x00" as *const u8 as *const libc::c_char,
                65535i32,
            );
            return 0 as *mut pdf_obj;
        }
        match ch {
            92 => {
                ch = ps_getescc(&mut p, endptr);
                if ch >= 0i32 {
                    let fresh4 = len;
                    len = len + 1;
                    sbuf[fresh4 as usize] = (ch & 0xffi32) as libc::c_char
                }
            }
            13 => {
                p = p.offset(1);
                if p < endptr && *p.offset(0) as libc::c_int == '\n' as i32 {
                    p = p.offset(1)
                }
                let fresh5 = len;
                len = len + 1;
                sbuf[fresh5 as usize] = '\n' as i32 as libc::c_char
            }
            _ => {
                if ch == '(' as i32 {
                    op_count += 1
                } else if ch == ')' as i32 {
                    op_count -= 1
                }
                let fresh6 = len;
                len = len + 1;
                sbuf[fresh6 as usize] = ch as libc::c_char;
                p = p.offset(1)
            }
        }
    }
    if op_count > 0i32 || p >= endptr || *p.offset(0) as libc::c_int != ')' as i32 {
        dpx_warning(
            b"Unbalanced parens/truncated PDF literal string.\x00" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut pdf_obj;
    }
    *pp = p.offset(1);
    return pdf_new_string(sbuf.as_mut_ptr() as *const libc::c_void, len as size_t);
}
/*
 * PDF Hex String
 */
unsafe extern "C" fn parse_pdf_hex_string(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    p = *pp;
    skip_white(&mut p, endptr);
    if p >= endptr || *p.offset(0) as libc::c_int != '<' as i32 {
        return 0 as *mut pdf_obj;
    }
    p = p.offset(1);
    len = 0i32;
    /*
     * PDF Reference does not describe how to treat invalid char.
     * Zero is appended if final hex digit is missing.
     */
    while p < endptr && *p.offset(0) as libc::c_int != '>' as i32 && len < 65535i32 {
        let mut ch: libc::c_int = 0;
        skip_white(&mut p, endptr);
        if p >= endptr || *p.offset(0) as libc::c_int == '>' as i32 {
            break;
        }
        ch = xtoi(*p.offset(0)) << 4i32;
        p = p.offset(1);
        skip_white(&mut p, endptr);
        if p < endptr && *p.offset(0) as libc::c_int != '>' as i32 {
            ch += xtoi(*p.offset(0));
            p = p.offset(1)
        }
        let fresh7 = len;
        len = len + 1;
        sbuf[fresh7 as usize] = (ch & 0xffi32) as libc::c_char
    }
    if p >= endptr {
        dpx_warning(b"Premature end of input hex string.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    } else {
        if *p.offset(0) as libc::c_int != '>' as i32 {
            dpx_warning(
                b"PDF string length too long. (limit: %d)\x00" as *const u8 as *const libc::c_char,
                65535i32,
            );
            return 0 as *mut pdf_obj;
        }
    }
    *pp = p.offset(1);
    return pdf_new_string(sbuf.as_mut_ptr() as *const libc::c_void, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_string(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(2) <= endptr {
        if **pp as libc::c_int == '(' as i32 {
            return parse_pdf_literal_string(pp, endptr);
        } else {
            if **pp as libc::c_int == '<' as i32
                && (*(*pp).offset(1) as libc::c_int == '>' as i32
                    || *(*__ctype_b_loc())
                        .offset(*(*pp).offset(1) as u8 as libc::c_int as isize)
                        as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0)
            {
                return parse_pdf_hex_string(pp, endptr);
            }
        }
    }
    dpx_warning(b"Could not find a string object.\x00" as *const u8 as *const libc::c_char);
    return 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_tainted_dict(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    parser_state.tainted = 1i32;
    result = parse_pdf_dict(pp, endptr, 0 as *mut pdf_file);
    parser_state.tainted = 0i32;
    return result;
}
/* PDF_PARSE_STRICT */
/* !PDF_PARSE_STRICT */
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_dict(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = *pp;
    skip_white(&mut p, endptr);
    /* At least four letter <<>>. */
    if p.offset(4) > endptr
        || *p.offset(0) as libc::c_int != '<' as i32
        || *p.offset(1) as libc::c_int != '<' as i32
    {
        return 0 as *mut pdf_obj;
    } /* skip >> */
    p = p.offset(2); /* skip ] */
    result = pdf_new_dict();
    skip_white(&mut p, endptr);
    while p < endptr && *p.offset(0) as libc::c_int != '>' as i32 {
        let mut key: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
        skip_white(&mut p, endptr);
        key = parse_pdf_name(&mut p, endptr);
        if key.is_null() {
            dpx_warning(
                b"Could not find a key in dictionary object.\x00" as *const u8
                    as *const libc::c_char,
            );
            pdf_release_obj(result);
            return 0 as *mut pdf_obj;
        }
        skip_white(&mut p, endptr);
        value = parse_pdf_object(&mut p, endptr, pf);
        if value.is_null() {
            pdf_release_obj(key);
            pdf_release_obj(value);
            pdf_release_obj(result);
            dpx_warning(
                b"Could not find a value in dictionary object.\x00" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut pdf_obj;
        }
        pdf_add_dict(result, key, value);
        skip_white(&mut p, endptr);
    }
    if p.offset(2) > endptr
        || *p.offset(0) as libc::c_int != '>' as i32
        || *p.offset(1) as libc::c_int != '>' as i32
    {
        dpx_warning(
            b"Syntax error: Dictionary object ended prematurely.\x00" as *const u8
                as *const libc::c_char,
        );
        pdf_release_obj(result);
        return 0 as *mut pdf_obj;
    }
    *pp = p.offset(2);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_array(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = *pp;
    skip_white(&mut p, endptr);
    if p.offset(2) > endptr || *p.offset(0) as libc::c_int != '[' as i32 {
        dpx_warning(b"Could not find an array object.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    }
    result = pdf_new_array();
    p = p.offset(1);
    skip_white(&mut p, endptr);
    while p < endptr && *p.offset(0) as libc::c_int != ']' as i32 {
        let mut elem: *mut pdf_obj = 0 as *mut pdf_obj;
        elem = parse_pdf_object(&mut p, endptr, pf);
        if elem.is_null() {
            pdf_release_obj(result);
            dpx_warning(
                b"Could not find a valid object in array object.\x00" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut pdf_obj;
        }
        pdf_add_array(result, elem);
        skip_white(&mut p, endptr);
    }
    if p >= endptr || *p.offset(0) as libc::c_int != ']' as i32 {
        dpx_warning(b"Array object ended prematurely.\x00" as *const u8 as *const libc::c_char);
        pdf_release_obj(result);
        return 0 as *mut pdf_obj;
    }
    *pp = p.offset(1);
    return result;
}
unsafe extern "C" fn parse_pdf_stream(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut dict: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_length: libc::c_int = 0;
    p = *pp;
    skip_white(&mut p, endptr);
    if p.offset(6) > endptr
        || strstartswith(p, b"stream\x00" as *const u8 as *const libc::c_char).is_null()
    {
        return 0 as *mut pdf_obj;
    }
    p = p.offset(6);
    /* The keyword stream that follows the stream dictionary
     * should be followed by an end-of-line marker consisting of
     * either a carriage return (0x0D;\r) and a line feed (0x0A;\n)
     * or just a line feed, and not by a carriage return alone.
     * [PDF Reference, 6th ed., version 1.7, pp. 60-61] */
    /* Notice that TeX translates an end-of-line marker to a single space. */
    if p < endptr && *p.offset(0) as libc::c_int == '\n' as i32 {
        p = p.offset(1)
    } else if p.offset(1) < endptr
        && (*p.offset(0) as libc::c_int == '\r' as i32
            && *p.offset(1) as libc::c_int == '\n' as i32)
    {
        p = p.offset(2)
    }
    /* Stream length */
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp2: *mut pdf_obj = 0 as *mut pdf_obj;
    tmp = pdf_lookup_dict(dict, b"Length\x00" as *const u8 as *const libc::c_char);
    if !tmp.is_null() {
        tmp2 = pdf_deref_obj(tmp);
        if pdf_obj_typeof(tmp2) != 2i32 {
            stream_length = -1i32
        } else {
            stream_length = pdf_number_value(tmp2) as libc::c_int
        }
        pdf_release_obj(tmp2);
    } else {
        return 0 as *mut pdf_obj;
    }
    if stream_length < 0i32 || p.offset(stream_length as isize) > endptr {
        return 0 as *mut pdf_obj;
    }
    /*
     * If Filter is not applied, set STREAM_COMPRESS flag.
     * Should we use filter for ASCIIHexEncode/ASCII85Encode-ed streams?
     */
    let mut filters: *mut pdf_obj = 0 as *mut pdf_obj;
    filters = pdf_lookup_dict(dict, b"Filter\x00" as *const u8 as *const libc::c_char);
    if filters.is_null() && stream_length > 10i32 {
        result = pdf_new_stream(1i32 << 0i32)
    } else {
        result = pdf_new_stream(0i32)
    }
    stream_dict = pdf_stream_dict(result);
    pdf_merge_dict(stream_dict, dict);
    pdf_add_stream(result, p as *const libc::c_void, stream_length);
    p = p.offset(stream_length as isize);
    /* Check "endsteam" */
    /* It is recommended that there be an end-of-line marker
     * after the data and before endstream; this marker is not included
     * in the stream length.
     * [PDF Reference, 6th ed., version 1.7, pp. 61] */
    if p < endptr && *p.offset(0) as libc::c_int == '\r' as i32 {
        p = p.offset(1)
    }
    if p < endptr && *p.offset(0) as libc::c_int == '\n' as i32 {
        p = p.offset(1)
    }
    if p.offset(9) > endptr
        || memcmp(
            p as *const libc::c_void,
            b"endstream\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            9i32 as u64,
        ) != 0
    {
        pdf_release_obj(result);
        return 0 as *mut pdf_obj;
    }
    p = p.offset(9);
    *pp = p;
    return result;
}
/* PLEASE REMOVE THIS */
/* This is not PDF indirect reference. */
unsafe extern "C" fn parse_pdf_reference(
    mut start: *mut *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    save = *start;
    skip_white(start, end);
    name = parse_opt_ident(start, end);
    if !name.is_null() {
        result = spc_lookup_reference(name);
        if result.is_null() {
            dpx_warning(
                b"Could not find the named reference (@%s).\x00" as *const u8
                    as *const libc::c_char,
                name,
            );
            dump(save, end);
            *start = save
        }
        free(name as *mut libc::c_void);
    } else {
        dpx_warning(b"Could not find a reference name.\x00" as *const u8 as *const libc::c_char);
        dump(save, end);
        *start = save;
        result = 0 as *mut pdf_obj
    }
    return result;
}
/* !PDF_PARSE_STRICT */
unsafe extern "C" fn try_pdf_reference(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut endptr: *mut *const libc::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut id: libc::c_uint = 0i32 as libc::c_uint;
    let mut gen: libc::c_ushort = 0i32 as libc::c_ushort;
    if !pf.is_null() {
    } else {
        __assert_fail(b"pf\x00" as *const u8 as *const libc::c_char,
                      b"dpx-pdfparse.c\x00" as *const u8 as
                          *const libc::c_char, 883i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"pdf_obj *try_pdf_reference(const char *, const char *, const char **, pdf_file *)\x00")).as_ptr());
    }
    if !endptr.is_null() {
        *endptr = start
    }
    skip_white(&mut start, end);
    if start > end.offset(-5)
        || *(*__ctype_b_loc()).offset(*start as u8 as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 0 as *mut pdf_obj;
    }
    while !(*start as libc::c_int == ' ' as i32
        || *start as libc::c_int == '\t' as i32
        || *start as libc::c_int == '\u{c}' as i32
        || *start as libc::c_int == '\r' as i32
        || *start as libc::c_int == '\n' as i32
        || *start as libc::c_int == '\u{0}' as i32)
    {
        if start >= end
            || *(*__ctype_b_loc()).offset(*start as u8 as libc::c_int as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            return 0 as *mut pdf_obj;
        }
        id = id
            .wrapping_mul(10i32 as libc::c_uint)
            .wrapping_add((*start as libc::c_int - '0' as i32) as libc::c_uint);
        start = start.offset(1)
    }
    skip_white(&mut start, end);
    if start >= end
        || *(*__ctype_b_loc()).offset(*start as u8 as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 0 as *mut pdf_obj;
    }
    while !(*start as libc::c_int == ' ' as i32
        || *start as libc::c_int == '\t' as i32
        || *start as libc::c_int == '\u{c}' as i32
        || *start as libc::c_int == '\r' as i32
        || *start as libc::c_int == '\n' as i32
        || *start as libc::c_int == '\u{0}' as i32)
    {
        if start >= end
            || *(*__ctype_b_loc()).offset(*start as u8 as libc::c_int as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            return 0 as *mut pdf_obj;
        }
        gen = (gen as libc::c_int * 10i32 + (*start as libc::c_int - '0' as i32)) as libc::c_ushort;
        start = start.offset(1)
    }
    skip_white(&mut start, end);
    if start >= end || *start as libc::c_int != 'R' as i32 {
        return 0 as *mut pdf_obj;
    }
    start = start.offset(1);
    if !(start >= end
        || (*start as libc::c_int == ' ' as i32
            || *start as libc::c_int == '\t' as i32
            || *start as libc::c_int == '\u{c}' as i32
            || *start as libc::c_int == '\r' as i32
            || *start as libc::c_int == '\n' as i32
            || *start as libc::c_int == '\u{0}' as i32)
        || (*start as libc::c_int == '(' as i32
            || *start as libc::c_int == ')' as i32
            || *start as libc::c_int == '/' as i32
            || *start as libc::c_int == '<' as i32
            || *start as libc::c_int == '>' as i32
            || *start as libc::c_int == '[' as i32
            || *start as libc::c_int == ']' as i32
            || *start as libc::c_int == '%' as i32))
    {
        return 0 as *mut pdf_obj;
    }
    if !endptr.is_null() {
        *endptr = start
    }
    return pdf_new_indirect(pf, id, gen);
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
/* Please remove this */
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_object(
    mut pp: *mut *const libc::c_char,
    mut endptr: *const libc::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj
/* If pf is NULL, then indirect references are not allowed */ {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut nextptr: *const libc::c_char = 0 as *const libc::c_char;
    skip_white(pp, endptr);
    if *pp >= endptr {
        dpx_warning(b"Could not find any valid object.\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    }
    match **pp as libc::c_int {
        60 => {
            if *(*pp).offset(1) as libc::c_int != '<' as i32 {
                result = parse_pdf_hex_string(pp, endptr)
            } else {
                let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
                result = parse_pdf_dict(pp, endptr, pf);
                skip_white(pp, endptr);
                if !result.is_null()
                    && *pp <= endptr.offset(-15)
                    && memcmp(
                        *pp as *const libc::c_void,
                        b"stream\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        6i32 as u64,
                    ) == 0
                {
                    dict = result;
                    result = parse_pdf_stream(pp, endptr, dict);
                    pdf_release_obj(dict);
                }
            }
        }
        40 => result = parse_pdf_string(pp, endptr),
        91 => result = parse_pdf_array(pp, endptr, pf),
        47 => result = parse_pdf_name(pp, endptr),
        110 => result = parse_pdf_null(pp, endptr),
        116 | 102 => result = parse_pdf_boolean(pp, endptr),
        43 | 45 | 46 => result = parse_pdf_number(pp, endptr),
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            /*
             * If pf != NULL, then we are parsing a PDF file,
             * and indirect references are allowed.
             */
            if !pf.is_null() && {
                result = try_pdf_reference(*pp, endptr, &mut nextptr, pf);
                !result.is_null()
            } {
                *pp = nextptr
            } else {
                result = parse_pdf_number(pp, endptr)
            }
        }
        64 => result = parse_pdf_reference(pp, endptr),
        _ => {
            dpx_warning(b"Unknown PDF object type.\x00" as *const u8 as *const libc::c_char);
            result = 0 as *mut pdf_obj
        }
    }
    return result;
}
