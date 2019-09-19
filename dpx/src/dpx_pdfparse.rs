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

use crate::strstartswith;
use crate::{info, warn};

use super::dpx_dpxutil::xtoi;
use super::dpx_error::{dpx_message, dpx_warning};
use super::dpx_mem::new;
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_add_stream, pdf_deref_obj, pdf_file, pdf_lookup_dict,
    pdf_merge_dict, pdf_new_array, pdf_new_boolean, pdf_new_dict, pdf_new_indirect, pdf_new_name,
    pdf_new_null, pdf_new_number, pdf_new_stream, pdf_new_string, pdf_number_value, pdf_obj,
    pdf_obj_typeof, pdf_release_obj, pdf_stream_dict, PdfObjType,
};
use crate::specials::spc_lookup_reference;
use libc::{free, memcmp, memcpy, strchr};

pub type size_t = u64;
/* pow() */
/* PDF */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserState {
    pub tainted: i32,
}
static mut parser_state: ParserState = ParserState { tainted: 0i32 };

static mut save: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn dump(mut start: *const i8, mut end: *const i8) {
    let mut p: *const i8 = start;
    info!("\nCurrent input buffer is -->");
    while p < end && p < start.offset(50) {
        let fresh0 = p;
        p = p.offset(1);
        dpx_message(b"%c\x00" as *const u8 as *const i8, *fresh0 as i32);
    }
    if p == start.offset(50) {
        info!("...");
    }
    info!("<--\n");
}
#[no_mangle]
pub unsafe extern "C" fn pdfparse_skip_line(mut start: *mut *const i8, mut end: *const i8) {
    while *start < end && **start as i32 != '\n' as i32 && **start as i32 != '\r' as i32 {
        *start = (*start).offset(1)
    }
    /* The carriage return (CR; \r; 0x0D) and line feed (LF; \n; 0x0A)
     * characters, also called newline characters, are treated as
     * end-of-line (EOL) markers. The combination of a carriage return
     * followed immediately by a line feed is treated as one EOL marker.
     */
    if *start < end && **start as i32 == '\r' as i32 {
        *start = (*start).offset(1)
    }
    if *start < end && **start as i32 == '\n' as i32 {
        *start = (*start).offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn skip_white(mut start: *mut *const i8, mut end: *const i8) {
    /*
     * The null (NUL; 0x00) character is a white-space character in PDF spec
     * but isspace(0x00) returns FALSE; on the other hand, the vertical tab
     * (VT; 0x0B) character is not a white-space character in PDF spec but
     * isspace(0x0B) returns TRUE.
     */
    while *start < end
        && (**start as i32 == ' ' as i32
            || **start as i32 == '\t' as i32
            || **start as i32 == '\u{c}' as i32
            || **start as i32 == '\r' as i32
            || **start as i32 == '\n' as i32
            || **start as i32 == '\u{0}' as i32
            || **start as i32 == '%' as i32)
    {
        if **start as i32 == '%' as i32 {
            pdfparse_skip_line(start, end);
        } else {
            *start = (*start).offset(1)
        }
    }
}
unsafe extern "C" fn parsed_string(mut start: *const i8, mut end: *const i8) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    len = end.wrapping_offset_from(start) as i64 as i32;
    if len > 0i32 {
        result = new(
            ((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        memcpy(
            result as *mut libc::c_void,
            start as *const libc::c_void,
            len as _,
        );
        *result.offset(len as isize) = '\u{0}' as i32 as i8
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn parse_number(mut start: *mut *const i8, mut end: *const i8) -> *mut i8 {
    let mut number: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    skip_white(start, end);
    p = *start;
    if p < end && (*p as i32 == '+' as i32 || *p as i32 == '-' as i32) {
        p = p.offset(1)
    }
    while p < end && libc::isdigit(*p as _) != 0 {
        p = p.offset(1)
    }
    if p < end && *p as i32 == '.' as i32 {
        p = p.offset(1);
        while p < end && libc::isdigit(*p as _) != 0 {
            p = p.offset(1)
        }
    }
    number = parsed_string(*start, p);
    *start = p;
    number
}
#[no_mangle]
pub unsafe extern "C" fn parse_unsigned(mut start: *mut *const i8, mut end: *const i8) -> *mut i8 {
    let mut number: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    skip_white(start, end);
    p = *start;
    while p < end {
        if libc::isdigit(*p as _) == 0 {
            break;
        }
        p = p.offset(1)
    }
    number = parsed_string(*start, p);
    *start = p;
    number
}
unsafe extern "C" fn parse_gen_ident(
    mut start: *mut *const i8,
    mut end: *const i8,
    mut valid_chars: *const i8,
) -> *mut i8 {
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    /* No skip_white(start, end)? */
    p = *start;
    while p < end {
        if strchr(valid_chars, *p as i32).is_null() {
            break;
        }
        p = p.offset(1)
    }
    ident = parsed_string(*start, p);
    *start = p;
    ident
}
#[no_mangle]
pub unsafe extern "C" fn parse_ident(mut start: *mut *const i8, mut end: *const i8) -> *mut i8 {
    static mut valid_chars: *const i8 =
        b"!\"#$&\'*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\x00"
            as *const u8 as *const i8;
    parse_gen_ident(start, end, valid_chars)
}
#[no_mangle]
pub unsafe extern "C" fn parse_val_ident(mut start: *mut *const i8, mut end: *const i8) -> *mut i8 {
    static mut valid_chars: *const i8 =
        b"!\"#$&\'*+,-./0123456789:;?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\x00"
            as *const u8 as *const i8;
    parse_gen_ident(start, end, valid_chars)
}
#[no_mangle]
pub unsafe extern "C" fn parse_opt_ident(mut start: *mut *const i8, mut end: *const i8) -> *mut i8 {
    if *start < end && **start as i32 == '@' as i32 {
        *start = (*start).offset(1);
        return parse_ident(start, end);
    }
    0 as *mut i8
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_number(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut p: *const i8 = 0 as *const i8;
    let mut v: f64 = 0.0f64;
    let mut nddigits: i32 = 0i32;
    let mut sign: i32 = 1i32;
    let mut has_dot: i32 = 0i32;
    p = *pp;
    skip_white(&mut p, endptr);
    if p >= endptr
        || libc::isdigit(*p.offset(0) as _) == 0
            && *p.offset(0) as i32 != '.' as i32
            && *p.offset(0) as i32 != '+' as i32
            && *p.offset(0) as i32 != '-' as i32
    {
        warn!("Could not find a numeric object.");
        return 0 as *mut pdf_obj;
    }
    if *p.offset(0) as i32 == '-' as i32 {
        if p.offset(1) >= endptr {
            warn!("Could not find a numeric object.");
            return 0 as *mut pdf_obj;
        }
        sign = -1i32;
        p = p.offset(1)
    } else if *p.offset(0) as i32 == '+' as i32 {
        if p.offset(1) >= endptr {
            warn!("Could not find a numeric object.");
            return 0 as *mut pdf_obj;
        }
        sign = 1i32;
        p = p.offset(1)
    }
    while p < endptr
        && !(*p.offset(0) as i32 == ' ' as i32
            || *p.offset(0) as i32 == '\t' as i32
            || *p.offset(0) as i32 == '\u{c}' as i32
            || *p.offset(0) as i32 == '\r' as i32
            || *p.offset(0) as i32 == '\n' as i32
            || *p.offset(0) as i32 == '\u{0}' as i32
            || (*p.offset(0) as i32 == '(' as i32
                || *p.offset(0) as i32 == ')' as i32
                || *p.offset(0) as i32 == '/' as i32
                || *p.offset(0) as i32 == '<' as i32
                || *p.offset(0) as i32 == '>' as i32
                || *p.offset(0) as i32 == '[' as i32
                || *p.offset(0) as i32 == ']' as i32
                || *p.offset(0) as i32 == '%' as i32))
    {
        if *p.offset(0) as i32 == '.' as i32 {
            if has_dot != 0 {
                /* Two dots */
                warn!("Could not find a numeric object.");
                return 0 as *mut pdf_obj;
            } else {
                has_dot = 1i32
            }
        } else if libc::isdigit(*p.offset(0) as _) != 0 {
            if has_dot != 0 {
                v +=
                    (*p.offset(0) as i32 - '0' as i32) as f64 / (10f64).powf((nddigits + 1) as f64);
                nddigits += 1
            } else {
                v = v * 10.0f64 + *p.offset(0) as i32 as f64 - '0' as i32 as f64
            }
        } else {
            warn!("Could not find a numeric object.");
            return 0 as *mut pdf_obj;
        }
        p = p.offset(1)
    }
    *pp = p;
    pdf_new_number(sign as f64 * v)
}
/*
 * PDF Name:
 *
 *  PDF-1.2+: Two hexadecimal digits preceded by a number sign.
 */
unsafe extern "C" fn pn_getc(mut pp: *mut *const i8, mut endptr: *const i8) -> i32 {
    let mut ch: i32 = 0i32;
    let mut p: *const i8 = 0 as *const i8;
    p = *pp;
    if *p.offset(0) as i32 == '#' as i32 {
        if p.offset(2) >= endptr {
            *pp = endptr;
            return -1i32;
        }
        if libc::isxdigit(*p.offset(1) as _) == 0 || libc::isxdigit(*p.offset(2) as _) == 0 {
            *pp = (*pp).offset(3);
            return -1i32;
        }
        ch = xtoi(*p.offset(1)) << 4i32;
        ch += xtoi(*p.offset(2));
        *pp = (*pp).offset(3)
    } else {
        ch = *p.offset(0) as i32;
        *pp = (*pp).offset(1)
    }
    ch
}
static mut sbuf: [i8; 65536] = [0; 65536];
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_name(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut name: [i8; 129] = [0; 129];
    let mut ch: i32 = 0;
    let mut len: i32 = 0i32;
    skip_white(pp, endptr);
    if *pp >= endptr || **pp as i32 != '/' as i32 {
        warn!("Could not find a name object.");
        return 0 as *mut pdf_obj;
    }
    *pp = (*pp).offset(1);
    while *pp < endptr
        && !(**pp as i32 == ' ' as i32
            || **pp as i32 == '\t' as i32
            || **pp as i32 == '\u{c}' as i32
            || **pp as i32 == '\r' as i32
            || **pp as i32 == '\n' as i32
            || **pp as i32 == '\u{0}' as i32
            || (**pp as i32 == '(' as i32
                || **pp as i32 == ')' as i32
                || **pp as i32 == '/' as i32
                || **pp as i32 == '<' as i32
                || **pp as i32 == '>' as i32
                || **pp as i32 == '[' as i32
                || **pp as i32 == ']' as i32
                || **pp as i32 == '%' as i32))
    {
        ch = pn_getc(pp, endptr);
        if ch < 0i32 || ch > 0xffi32 {
            warn!("Invalid char in PDF name object. (ignored)");
        } else if ch == 0i32 {
            warn!("Null char not allowed in PDF name object. (ignored)");
        } else if len < 65535i32 + 1i32 {
            if len == 128i32 {
                warn!("PDF name length too long. (>= {} bytes)", 128i32);
            }
            let fresh1 = len;
            len = len + 1;
            name[fresh1 as usize] = ch as i8
        } else {
            warn!(
                "PDF name length too long. (>= {} bytes, truncated)",
                65535 + 1
            );
        }
    }
    if len < 1i32 {
        warn!("No valid name object found.");
        return 0 as *mut pdf_obj;
    }
    name[len as usize] = '\u{0}' as i32 as i8;
    pdf_new_name(name.as_mut_ptr())
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_boolean(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(4) <= endptr
        && !strstartswith(*pp, b"true\x00" as *const u8 as *const i8).is_null()
    {
        if (*pp).offset(4) == endptr
            || (*(*pp).offset(4) as i32 == ' ' as i32
                || *(*pp).offset(4) as i32 == '\t' as i32
                || *(*pp).offset(4) as i32 == '\u{c}' as i32
                || *(*pp).offset(4) as i32 == '\r' as i32
                || *(*pp).offset(4) as i32 == '\n' as i32
                || *(*pp).offset(4) as i32 == '\u{0}' as i32
                || (*(*pp).offset(4) as i32 == '(' as i32
                    || *(*pp).offset(4) as i32 == ')' as i32
                    || *(*pp).offset(4) as i32 == '/' as i32
                    || *(*pp).offset(4) as i32 == '<' as i32
                    || *(*pp).offset(4) as i32 == '>' as i32
                    || *(*pp).offset(4) as i32 == '[' as i32
                    || *(*pp).offset(4) as i32 == ']' as i32
                    || *(*pp).offset(4) as i32 == '%' as i32))
        {
            *pp = (*pp).offset(4);
            return pdf_new_boolean(1_i8);
        }
    } else if (*pp).offset(5) <= endptr
        && !strstartswith(*pp, b"false\x00" as *const u8 as *const i8).is_null()
    {
        if (*pp).offset(5) == endptr
            || (*(*pp).offset(5) as i32 == ' ' as i32
                || *(*pp).offset(5) as i32 == '\t' as i32
                || *(*pp).offset(5) as i32 == '\u{c}' as i32
                || *(*pp).offset(5) as i32 == '\r' as i32
                || *(*pp).offset(5) as i32 == '\n' as i32
                || *(*pp).offset(5) as i32 == '\u{0}' as i32
                || (*(*pp).offset(5) as i32 == '(' as i32
                    || *(*pp).offset(5) as i32 == ')' as i32
                    || *(*pp).offset(5) as i32 == '/' as i32
                    || *(*pp).offset(5) as i32 == '<' as i32
                    || *(*pp).offset(5) as i32 == '>' as i32
                    || *(*pp).offset(5) as i32 == '[' as i32
                    || *(*pp).offset(5) as i32 == ']' as i32
                    || *(*pp).offset(5) as i32 == '%' as i32))
        {
            *pp = (*pp).offset(5);
            return pdf_new_boolean(0_i8);
        }
    }
    warn!("Not a boolean object.");
    0 as *mut pdf_obj
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_null(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(4) > endptr {
        warn!("Not a null object.");
        return 0 as *mut pdf_obj;
    } else {
        if (*pp).offset(4) < endptr
            && !(*(*pp).offset(4) as i32 == ' ' as i32
                || *(*pp).offset(4) as i32 == '\t' as i32
                || *(*pp).offset(4) as i32 == '\u{c}' as i32
                || *(*pp).offset(4) as i32 == '\r' as i32
                || *(*pp).offset(4) as i32 == '\n' as i32
                || *(*pp).offset(4) as i32 == '\u{0}' as i32
                || (*(*pp).offset(4) as i32 == '(' as i32
                    || *(*pp).offset(4) as i32 == ')' as i32
                    || *(*pp).offset(4) as i32 == '/' as i32
                    || *(*pp).offset(4) as i32 == '<' as i32
                    || *(*pp).offset(4) as i32 == '>' as i32
                    || *(*pp).offset(4) as i32 == '[' as i32
                    || *(*pp).offset(4) as i32 == ']' as i32
                    || *(*pp).offset(4) as i32 == '%' as i32))
        {
            warn!("Not a null object.");
            return 0 as *mut pdf_obj;
        } else {
            if !strstartswith(*pp, b"null\x00" as *const u8 as *const i8).is_null() {
                *pp = (*pp).offset(4);
                return pdf_new_null();
            }
        }
    }
    warn!("Not a null object.");
    0 as *mut pdf_obj
}
/*
 * PDF Literal String
 */
unsafe extern "C" fn ps_getescc(mut pp: *mut *const i8, mut endptr: *const i8) -> i32 {
    let mut ch: i32 = 0; /* backslash assumed. */
    let mut i: i32 = 0;
    let mut p: *const i8 = 0 as *const i8;
    p = (*pp).offset(1);
    match *p.offset(0) as i32 {
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
            if p < endptr && *p.offset(0) as i32 == '\n' as i32 {
                p = p.offset(1)
            }
        }
        _ => {
            if *p.offset(0) as i32 == '\\' as i32
                || *p.offset(0) as i32 == '(' as i32
                || *p.offset(0) as i32 == ')' as i32
            {
                ch = *p.offset(0) as i32;
                p = p.offset(1)
            } else if *p.offset(0) as i32 >= '0' as i32 && *p.offset(0) as i32 <= '7' as i32 {
                ch = 0i32;
                /* Ignore overflow. */
                i = 0i32;
                while i < 3i32
                    && p < endptr
                    && (*p.offset(0) as i32 >= '0' as i32 && *p.offset(0) as i32 <= '7' as i32)
                {
                    ch = (ch << 3i32) + (*p.offset(0) as i32 - '0' as i32);
                    p = p.offset(1);
                    i += 1
                }
                ch = ch & 0xffi32
            } else {
                /* Don't forget isodigit() is a macro. */
                ch = *p.offset(0) as u8 as i32; /* Ignore only backslash. */
                p = p.offset(1)
            }
        }
    }
    *pp = p;
    ch
}
unsafe extern "C" fn parse_pdf_literal_string(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut ch: i32 = 0;
    let mut op_count: i32 = 0i32;
    let mut len: i32 = 0i32;
    let mut p: *const i8 = 0 as *const i8;
    p = *pp;
    skip_white(&mut p, endptr);
    if p >= endptr || *p.offset(0) as i32 != '(' as i32 {
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
        ch = *p.offset(0) as i32;
        if ch == ')' as i32 && op_count < 1i32 {
            break;
        }
        if parser_state.tainted != 0 {
            if p.offset(1) < endptr && ch & 0x80i32 != 0 {
                if len + 2i32 >= 65535i32 {
                    warn!("PDF string length too long. (limit: {})", 65535i32);
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
            warn!("PDF string length too long. (limit: {})", 65535i32);
            return 0 as *mut pdf_obj;
        }
        match ch {
            92 => {
                ch = ps_getescc(&mut p, endptr);
                if ch >= 0i32 {
                    let fresh4 = len;
                    len = len + 1;
                    sbuf[fresh4 as usize] = (ch & 0xffi32) as i8
                }
            }
            13 => {
                p = p.offset(1);
                if p < endptr && *p.offset(0) as i32 == '\n' as i32 {
                    p = p.offset(1)
                }
                let fresh5 = len;
                len = len + 1;
                sbuf[fresh5 as usize] = '\n' as i32 as i8
            }
            _ => {
                if ch == '(' as i32 {
                    op_count += 1
                } else if ch == ')' as i32 {
                    op_count -= 1
                }
                let fresh6 = len;
                len = len + 1;
                sbuf[fresh6 as usize] = ch as i8;
                p = p.offset(1)
            }
        }
    }
    if op_count > 0i32 || p >= endptr || *p.offset(0) as i32 != ')' as i32 {
        warn!("Unbalanced parens/truncated PDF literal string.");
        return 0 as *mut pdf_obj;
    }
    *pp = p.offset(1);
    pdf_new_string(sbuf.as_mut_ptr() as *const libc::c_void, len as size_t)
}
/*
 * PDF Hex String
 */
unsafe extern "C" fn parse_pdf_hex_string(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut p: *const i8 = 0 as *const i8;
    let mut len: i32 = 0;
    p = *pp;
    skip_white(&mut p, endptr);
    if p >= endptr || *p.offset(0) as i32 != '<' as i32 {
        return 0 as *mut pdf_obj;
    }
    p = p.offset(1);
    len = 0i32;
    /*
     * PDF Reference does not describe how to treat invalid char.
     * Zero is appended if final hex digit is missing.
     */
    while p < endptr && *p.offset(0) as i32 != '>' as i32 && len < 65535i32 {
        let mut ch: i32 = 0;
        skip_white(&mut p, endptr);
        if p >= endptr || *p.offset(0) as i32 == '>' as i32 {
            break;
        }
        ch = xtoi(*p.offset(0)) << 4i32;
        p = p.offset(1);
        skip_white(&mut p, endptr);
        if p < endptr && *p.offset(0) as i32 != '>' as i32 {
            ch += xtoi(*p.offset(0));
            p = p.offset(1)
        }
        let fresh7 = len;
        len = len + 1;
        sbuf[fresh7 as usize] = (ch & 0xffi32) as i8
    }
    if p >= endptr {
        warn!("Premature end of input hex string.");
        return 0 as *mut pdf_obj;
    } else {
        if *p.offset(0) as i32 != '>' as i32 {
            warn!("PDF string length too long. (limit: {})", 65535i32);
            return 0 as *mut pdf_obj;
        }
    }
    *pp = p.offset(1);
    pdf_new_string(sbuf.as_mut_ptr() as *const libc::c_void, len as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_string(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(2) <= endptr {
        if **pp as i32 == '(' as i32 {
            return parse_pdf_literal_string(pp, endptr);
        } else {
            if **pp as i32 == '<' as i32
                && (*(*pp).offset(1) as i32 == '>' as i32
                    || libc::isxdigit(*(*pp).offset(1) as _) != 0)
            {
                return parse_pdf_hex_string(pp, endptr);
            }
        }
    }
    warn!("Could not find a string object.");
    0 as *mut pdf_obj
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_tainted_dict(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    parser_state.tainted = 1i32;
    result = parse_pdf_dict(pp, endptr, 0 as *mut pdf_file);
    parser_state.tainted = 0i32;
    result
}
/* PDF_PARSE_STRICT */
/* !PDF_PARSE_STRICT */
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_dict(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const i8 = 0 as *const i8;
    p = *pp;
    skip_white(&mut p, endptr);
    /* At least four letter <<>>. */
    if p.offset(4) > endptr
        || *p.offset(0) as i32 != '<' as i32
        || *p.offset(1) as i32 != '<' as i32
    {
        return 0 as *mut pdf_obj;
    } /* skip >> */
    p = p.offset(2); /* skip ] */
    result = pdf_new_dict();
    skip_white(&mut p, endptr);
    while p < endptr && *p.offset(0) as i32 != '>' as i32 {
        let mut key: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
        skip_white(&mut p, endptr);
        key = parse_pdf_name(&mut p, endptr);
        if key.is_null() {
            warn!("Could not find a key in dictionary object.");
            pdf_release_obj(result);
            return 0 as *mut pdf_obj;
        }
        skip_white(&mut p, endptr);
        value = parse_pdf_object(&mut p, endptr, pf);
        if value.is_null() {
            pdf_release_obj(key);
            pdf_release_obj(value);
            pdf_release_obj(result);
            warn!("Could not find a value in dictionary object.");
            return 0 as *mut pdf_obj;
        }
        pdf_add_dict(result, key, value);
        skip_white(&mut p, endptr);
    }
    if p.offset(2) > endptr
        || *p.offset(0) as i32 != '>' as i32
        || *p.offset(1) as i32 != '>' as i32
    {
        warn!("Syntax error: Dictionary object ended prematurely.");
        pdf_release_obj(result);
        return 0 as *mut pdf_obj;
    }
    *pp = p.offset(2);
    result
}
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_array(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const i8 = 0 as *const i8;
    p = *pp;
    skip_white(&mut p, endptr);
    if p.offset(2) > endptr || *p.offset(0) as i32 != '[' as i32 {
        warn!("Could not find an array object.");
        return 0 as *mut pdf_obj;
    }
    result = pdf_new_array();
    p = p.offset(1);
    skip_white(&mut p, endptr);
    while p < endptr && *p.offset(0) as i32 != ']' as i32 {
        let mut elem: *mut pdf_obj = 0 as *mut pdf_obj;
        elem = parse_pdf_object(&mut p, endptr, pf);
        if elem.is_null() {
            pdf_release_obj(result);
            warn!("Could not find a valid object in array object.");
            return 0 as *mut pdf_obj;
        }
        pdf_add_array(result, elem);
        skip_white(&mut p, endptr);
    }
    if p >= endptr || *p.offset(0) as i32 != ']' as i32 {
        warn!("Array object ended prematurely.");
        pdf_release_obj(result);
        return 0 as *mut pdf_obj;
    }
    *pp = p.offset(1);
    result
}
unsafe extern "C" fn parse_pdf_stream(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut dict: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut p: *const i8 = 0 as *const i8;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_length: i32 = 0;
    p = *pp;
    skip_white(&mut p, endptr);
    if p.offset(6) > endptr || strstartswith(p, b"stream\x00" as *const u8 as *const i8).is_null() {
        return 0 as *mut pdf_obj;
    }
    p = p.offset(6);
    /* The keyword stream that follows the stream dictionary
     * should be followed by an end-of-line marker consisting of
     * either a carriage return (0x0D;\r) and a line feed (0x0A;\n)
     * or just a line feed, and not by a carriage return alone.
     * [PDF Reference, 6th ed., version 1.7, pp. 60-61] */
    /* Notice that TeX translates an end-of-line marker to a single space. */
    if p < endptr && *p.offset(0) as i32 == '\n' as i32 {
        p = p.offset(1)
    } else if p.offset(1) < endptr
        && (*p.offset(0) as i32 == '\r' as i32 && *p.offset(1) as i32 == '\n' as i32)
    {
        p = p.offset(2)
    }
    /* Stream length */
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp2: *mut pdf_obj = 0 as *mut pdf_obj;
    tmp = pdf_lookup_dict(dict, b"Length\x00" as *const u8 as *const i8);
    if !tmp.is_null() {
        tmp2 = pdf_deref_obj(tmp);
        if pdf_obj_typeof(tmp2) != PdfObjType::NUMBER {
            stream_length = -1i32
        } else {
            stream_length = pdf_number_value(tmp2) as i32
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
    filters = pdf_lookup_dict(dict, b"Filter\x00" as *const u8 as *const i8);
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
    if p < endptr && *p.offset(0) as i32 == '\r' as i32 {
        p = p.offset(1)
    }
    if p < endptr && *p.offset(0) as i32 == '\n' as i32 {
        p = p.offset(1)
    }
    if p.offset(9) > endptr
        || memcmp(
            p as *const libc::c_void,
            b"endstream\x00" as *const u8 as *const i8 as *const libc::c_void,
            9,
        ) != 0
    {
        pdf_release_obj(result);
        return 0 as *mut pdf_obj;
    }
    p = p.offset(9);
    *pp = p;
    result
}
/* PLEASE REMOVE THIS */
/* This is not PDF indirect reference. */
unsafe extern "C" fn parse_pdf_reference(
    mut start: *mut *const i8,
    mut end: *const i8,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut name: *mut i8 = 0 as *mut i8;
    save = *start;
    skip_white(start, end);
    name = parse_opt_ident(start, end);
    if !name.is_null() {
        result = spc_lookup_reference(name);
        if result.is_null() {
            dpx_warning(
                b"Could not find the named reference (@%s).\x00" as *const u8 as *const i8,
                name,
            );
            dump(save, end);
            *start = save
        }
        free(name as *mut libc::c_void);
    } else {
        warn!("Could not find a reference name.");
        dump(save, end);
        *start = save;
        result = 0 as *mut pdf_obj
    }
    result
}
/* !PDF_PARSE_STRICT */
unsafe extern "C" fn try_pdf_reference(
    mut start: *const i8,
    mut end: *const i8,
    mut endptr: *mut *const i8,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut id: u32 = 0_u32;
    let mut gen: u16 = 0_u16;
    assert!(!pf.is_null());
    if !endptr.is_null() {
        *endptr = start
    }
    skip_white(&mut start, end);
    if start > end.offset(-5) || libc::isdigit(*start as _) == 0 {
        return 0 as *mut pdf_obj;
    }
    while !(*start as i32 == ' ' as i32
        || *start as i32 == '\t' as i32
        || *start as i32 == '\u{c}' as i32
        || *start as i32 == '\r' as i32
        || *start as i32 == '\n' as i32
        || *start as i32 == '\u{0}' as i32)
    {
        if start >= end || libc::isdigit(*start as _) == 0 {
            return 0 as *mut pdf_obj;
        }
        id = id
            .wrapping_mul(10_u32)
            .wrapping_add((*start as i32 - '0' as i32) as u32);
        start = start.offset(1)
    }
    skip_white(&mut start, end);
    if start >= end || libc::isdigit(*start as _) == 0 {
        return 0 as *mut pdf_obj;
    }
    while !(*start as i32 == ' ' as i32
        || *start as i32 == '\t' as i32
        || *start as i32 == '\u{c}' as i32
        || *start as i32 == '\r' as i32
        || *start as i32 == '\n' as i32
        || *start as i32 == '\u{0}' as i32)
    {
        if start >= end || libc::isdigit(*start as _) == 0 {
            return 0 as *mut pdf_obj;
        }
        gen = (gen as i32 * 10i32 + (*start as i32 - '0' as i32)) as u16;
        start = start.offset(1)
    }
    skip_white(&mut start, end);
    if start >= end || *start as i32 != 'R' as i32 {
        return 0 as *mut pdf_obj;
    }
    start = start.offset(1);
    if !(start >= end
        || (*start as i32 == ' ' as i32
            || *start as i32 == '\t' as i32
            || *start as i32 == '\u{c}' as i32
            || *start as i32 == '\r' as i32
            || *start as i32 == '\n' as i32
            || *start as i32 == '\u{0}' as i32)
        || (*start as i32 == '(' as i32
            || *start as i32 == ')' as i32
            || *start as i32 == '/' as i32
            || *start as i32 == '<' as i32
            || *start as i32 == '>' as i32
            || *start as i32 == '[' as i32
            || *start as i32 == ']' as i32
            || *start as i32 == '%' as i32))
    {
        return 0 as *mut pdf_obj;
    }
    if !endptr.is_null() {
        *endptr = start
    }
    pdf_new_indirect(pf, id, gen)
}
/* Please remove this */
#[no_mangle]
pub unsafe extern "C" fn parse_pdf_object(
    mut pp: *mut *const i8,
    mut endptr: *const i8,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj
/* If pf is NULL, then indirect references are not allowed */ {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut nextptr: *const i8 = 0 as *const i8;
    skip_white(pp, endptr);
    if *pp >= endptr {
        warn!("Could not find any valid object.");
        return 0 as *mut pdf_obj;
    }
    match **pp as i32 {
        60 => {
            if *(*pp).offset(1) as i32 != '<' as i32 {
                result = parse_pdf_hex_string(pp, endptr)
            } else {
                let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
                result = parse_pdf_dict(pp, endptr, pf);
                skip_white(pp, endptr);
                if !result.is_null()
                    && *pp <= endptr.offset(-15)
                    && memcmp(
                        *pp as *const libc::c_void,
                        b"stream\x00" as *const u8 as *const i8 as *const libc::c_void,
                        6,
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
            warn!("Unknown PDF object type.");
            result = 0 as *mut pdf_obj
        }
    }
    result
}
