#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::info;

extern crate libc;
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_array_length, pdf_dict_keys, pdf_get_array, pdf_link_obj,
    pdf_lookup_dict, pdf_name_value, pdf_new_array, pdf_new_dict, pdf_new_name, pdf_new_null,
    pdf_new_number, pdf_new_string, pdf_obj, pdf_ref_obj, pdf_release_obj, pdf_string_value,
};
use libc::free;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
        format: tt_input_format_type,
        is_gz: i32,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn agl_get_unicodes(glyphstr: *const i8, unicodes: *mut i32, max_uncodes: i32) -> i32;
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
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
    fn skip_white(start: *mut *const i8, end: *const i8);
}
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
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
/* quasi-hack to get the primary input */
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    false
}
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn otl_conf_set_verbose(mut level: i32) {
    verbose = level;
}
unsafe extern "C" fn parse_uc_coverage(
    mut gclass: *mut pdf_obj,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut coverage: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut value: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut ucv: i32 = 0i32;
    let mut glyphname: *mut i8 = 0 as *mut i8;
    let mut glyphclass: *mut i8 = 0 as *mut i8;
    if (*pp).offset(1) >= endptr {
        return 0 as *mut pdf_obj;
    }
    if **pp as i32 == '[' as i32 {
        *pp = (*pp).offset(1)
    }
    coverage = pdf_new_array();
    while *pp < endptr {
        skip_white(pp, endptr);
        match **pp as i32 {
            93 | 59 => {
                *pp = (*pp).offset(1);
                return coverage;
            }
            44 => *pp = (*pp).offset(1),
            64 => {
                let mut cvalues: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut i: i32 = 0;
                let mut size: i32 = 0;
                *pp = (*pp).offset(1);
                glyphclass = parse_c_ident(pp, endptr);
                cvalues = pdf_lookup_dict(gclass, glyphclass);
                if cvalues.is_null() {
                    _tt_abort(
                        b"%s not defined...\x00" as *const u8 as *const i8,
                        glyphclass,
                    );
                }
                size = pdf_array_length(cvalues) as i32;
                i = 0i32;
                while i < size {
                    pdf_add_array(coverage, pdf_link_obj(pdf_get_array(cvalues, i)));
                    i += 1
                }
            }
            _ => {
                glyphname = parse_c_ident(pp, endptr);
                if glyphname.is_null() {
                    _tt_abort(
                        b"Invalid Unicode character specified.\x00" as *const u8 as *const i8,
                    );
                }
                skip_white(pp, endptr);
                if (*pp).offset(1) < endptr && **pp as i32 == '-' as i32 {
                    value = pdf_new_array();
                    if agl_get_unicodes(glyphname, &mut ucv, 1i32) != 1i32 {
                        _tt_abort(
                            b"Invalid Unicode char: %s\x00" as *const u8 as *const i8,
                            glyphname,
                        );
                    }
                    pdf_add_array(value, pdf_new_number(ucv as f64));
                    free(glyphname as *mut libc::c_void);
                    *pp = (*pp).offset(1);
                    skip_white(pp, endptr);
                    glyphname = parse_c_ident(pp, endptr);
                    if glyphname.is_null() {
                        _tt_abort(
                            b"Invalid Unicode char: %s\x00" as *const u8 as *const i8,
                            glyphname,
                        );
                    }
                    if agl_get_unicodes(glyphname, &mut ucv, 1i32) != 1i32 {
                        _tt_abort(
                            b"Invalid Unicode char: %s\x00" as *const u8 as *const i8,
                            glyphname,
                        );
                    }
                    pdf_add_array(value, pdf_new_number(ucv as f64));
                    free(glyphname as *mut libc::c_void);
                } else {
                    if agl_get_unicodes(glyphname, &mut ucv, 1i32) != 1i32 {
                        _tt_abort(
                            b"Invalid Unicode char: %s\x00" as *const u8 as *const i8,
                            glyphname,
                        );
                    }
                    value = pdf_new_number(ucv as f64);
                    free(glyphname as *mut libc::c_void);
                }
                pdf_add_array(coverage, value);
            }
        }
        skip_white(pp, endptr);
    }
    coverage
}
unsafe extern "C" fn add_rule(
    mut rule: *mut pdf_obj,
    mut gclass: *mut pdf_obj,
    mut first: *mut i8,
    mut second: *mut i8,
    mut suffix: *mut i8,
) {
    let mut glyph1: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut glyph2: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut unicodes: [i32; 16] = [0; 16];
    let mut i: i32 = 0;
    let mut n_unicodes: i32 = 0;
    if *first.offset(0) as i32 == '@' as i32 {
        glyph1 = pdf_lookup_dict(gclass, &mut *first.offset(1));
        if glyph1.is_null() {
            dpx_warning(
                b"No glyph class \"%s\" found.\x00" as *const u8 as *const i8,
                &mut *first.offset(1) as *mut i8,
            );
            return;
        }
        pdf_link_obj(glyph1);
        if verbose > 0i32 {
            dpx_message(
                b"otl_conf>> Output glyph sequence: %s\n\x00" as *const u8 as *const i8,
                first,
            );
        }
    } else {
        n_unicodes = agl_get_unicodes(first, unicodes.as_mut_ptr(), 16i32);
        if n_unicodes < 1i32 {
            dpx_warning(
                b"Failed to convert glyph \"%s\" to Unicode sequence.\x00" as *const u8
                    as *const i8,
                first,
            );
            return;
        }
        glyph1 = pdf_new_array();
        if verbose > 0i32 {
            dpx_message(
                b"otl_conf>> Output glyph sequence: %s ->\x00" as *const u8 as *const i8,
                first,
            );
        }
        i = 0i32;
        while i < n_unicodes {
            pdf_add_array(glyph1, pdf_new_number(unicodes[i as usize] as f64));
            if verbose > 0i32 {
                if unicodes[i as usize] < 0x10000i32 {
                    info!(" U+{:04X}", unicodes[i as usize],);
                } else {
                    info!(" U+{:06X}", unicodes[i as usize],);
                }
            }
            i += 1
        }
        if verbose > 0i32 {
            info!("\n");
        }
    }
    if *second.offset(0) as i32 == '@' as i32 {
        glyph2 = pdf_lookup_dict(gclass, &mut *second.offset(1));
        if glyph2.is_null() {
            dpx_warning(
                b"No glyph class \"%s\" found.\x00" as *const u8 as *const i8,
                &mut *second.offset(1) as *mut i8,
            );
            return;
        }
        pdf_link_obj(glyph2);
        if verbose > 0i32 {
            dpx_message(
                b"otl_conf>> Input glyph sequence: %s (%s)\n\x00" as *const u8 as *const i8,
                second,
                suffix,
            );
        }
    } else {
        n_unicodes = agl_get_unicodes(second, unicodes.as_mut_ptr(), 16i32);
        if n_unicodes < 1i32 {
            dpx_warning(
                b"Failed to convert glyph \"%s\" to Unicode sequence.\x00" as *const u8
                    as *const i8,
                second,
            );
            return;
        }
        if verbose > 0i32 {
            if !suffix.is_null() {
                dpx_message(
                    b"otl_conf>> Input glyph sequence: %s.%s ->\x00" as *const u8 as *const i8,
                    second,
                    suffix,
                );
            } else {
                dpx_message(
                    b"otl_conf>> Input glyph sequence: %s ->\x00" as *const u8 as *const i8,
                    second,
                );
            }
        }
        glyph2 = pdf_new_array();
        i = 0i32;
        while i < n_unicodes {
            pdf_add_array(glyph2, pdf_new_number(unicodes[i as usize] as f64));
            if verbose > 0i32 {
                if unicodes[i as usize] < 0x10000i32 {
                    info!(" U+{:04X}", unicodes[i as usize],);
                } else {
                    info!(" U+{:06X}", unicodes[i as usize],);
                }
            }
            i += 1
        }
        if verbose > 0i32 {
            dpx_message(b" (%s)\n\x00" as *const u8 as *const i8, suffix);
        }
    }
    /* OK */
    if !suffix.is_null() {
        pdf_add_array(
            rule,
            pdf_new_string(suffix as *const libc::c_void, strlen(suffix)),
        ); /* allows @ */
    } else {
        pdf_add_array(rule, pdf_new_null());
    }
    pdf_add_array(rule, glyph1);
    pdf_add_array(rule, glyph2);
}
unsafe extern "C" fn parse_substrule(
    mut gclass: *mut pdf_obj,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut substrule: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut token: *mut i8 = 0 as *mut i8;
    skip_white(pp, endptr);
    if *pp < endptr && **pp as i32 == '{' as i32 {
        *pp = (*pp).offset(1)
    }
    skip_white(pp, endptr);
    if *pp >= endptr {
        return 0 as *mut pdf_obj;
    }
    substrule = pdf_new_array();
    while *pp < endptr && **pp as i32 != '}' as i32 {
        skip_white(pp, endptr);
        if *pp >= endptr {
            break;
        }
        if **pp as i32 == '#' as i32 {
            while *pp < endptr {
                if **pp as i32 == '\r' as i32 || **pp as i32 == '\n' as i32 {
                    *pp = (*pp).offset(1);
                    break;
                } else {
                    *pp = (*pp).offset(1)
                }
            }
        } else if **pp as i32 == ';' as i32 {
            *pp = (*pp).offset(1)
        } else {
            skip_white(pp, endptr);
            token = parse_c_ident(pp, endptr);
            if token.is_null() {
                break;
            }
            if streq_ptr(token, b"assign\x00" as *const u8 as *const i8) as i32 != 0
                || streq_ptr(token, b"substitute\x00" as *const u8 as *const i8) as i32 != 0
            {
                let mut tmp: *mut i8 = 0 as *mut i8;
                let mut first: *mut i8 = 0 as *mut i8;
                let mut second: *mut i8 = 0 as *mut i8;
                let mut suffix: *mut i8 = 0 as *mut i8;
                skip_white(pp, endptr);
                first = parse_c_ident(pp, endptr);
                if first.is_null() {
                    _tt_abort(b"Syntax error (1)\x00" as *const u8 as *const i8);
                }
                skip_white(pp, endptr);
                tmp = parse_c_ident(pp, endptr);
                if strcmp(tmp, b"by\x00" as *const u8 as *const i8) != 0
                    && strcmp(tmp, b"to\x00" as *const u8 as *const i8) != 0
                {
                    _tt_abort(b"Syntax error (2): %s\x00" as *const u8 as *const i8, *pp);
                }
                skip_white(pp, endptr);
                second = parse_c_ident(pp, endptr);
                if second.is_null() {
                    _tt_abort(b"Syntax error (3)\x00" as *const u8 as *const i8);
                }
                /* (assign|substitute) tag dst src */
                pdf_add_array(substrule, pdf_new_name(token)); /* = */
                if (*pp).offset(1) < endptr && **pp as i32 == '.' as i32 {
                    *pp = (*pp).offset(1);
                    suffix = parse_c_ident(pp, endptr)
                } else {
                    suffix = 0 as *mut i8
                }
                add_rule(substrule, gclass, first, second, suffix);
                free(first as *mut libc::c_void);
                free(tmp as *mut libc::c_void);
                free(second as *mut libc::c_void);
                free(suffix as *mut libc::c_void);
            } else {
                _tt_abort(b"Unkown command %s.\x00" as *const u8 as *const i8, token);
            }
            free(token as *mut libc::c_void);
            skip_white(pp, endptr);
        }
    }
    if *pp < endptr && **pp as i32 == '}' as i32 {
        *pp = (*pp).offset(1)
    }
    substrule
}
unsafe extern "C" fn parse_block(
    mut gclass: *mut pdf_obj,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> *mut pdf_obj {
    let mut rule: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut token: *mut i8 = 0 as *mut i8;
    let mut tmp: *mut i8 = 0 as *mut i8;
    skip_white(pp, endptr);
    if *pp < endptr && **pp as i32 == '{' as i32 {
        *pp = (*pp).offset(1)
    }
    skip_white(pp, endptr);
    if *pp >= endptr {
        return 0 as *mut pdf_obj;
    }
    rule = pdf_new_dict();
    while *pp < endptr && **pp as i32 != '}' as i32 {
        skip_white(pp, endptr);
        if *pp >= endptr {
            break;
        }
        if **pp as i32 == '#' as i32 {
            while *pp < endptr {
                if **pp as i32 == '\r' as i32 || **pp as i32 == '\n' as i32 {
                    *pp = (*pp).offset(1);
                    break;
                } else {
                    *pp = (*pp).offset(1)
                }
            }
        } else if **pp as i32 == ';' as i32 {
            *pp = (*pp).offset(1)
        } else {
            skip_white(pp, endptr);
            token = parse_c_ident(pp, endptr);
            if token.is_null() {
                break;
            }
            if streq_ptr(token, b"script\x00" as *const u8 as *const i8) as i32 != 0
                || streq_ptr(token, b"language\x00" as *const u8 as *const i8) as i32 != 0
            {
                let mut i: i32 = 0;
                let mut len: i32 = 0;
                skip_white(pp, endptr);
                len = 0i32;
                while (*pp).offset(len as isize) < endptr
                    && *(*pp).offset(len as isize) as i32 != ';' as i32
                {
                    len += 1
                }
                if len > 0i32 {
                    tmp = new(((len + 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                        as u32) as *mut i8;
                    memset(tmp as *mut libc::c_void, 0i32, (len + 1i32) as u64);
                    i = 0i32;
                    while i < len {
                        if libc::isspace(**pp as _) == 0 {
                            *tmp.offset(i as isize) = **pp
                        }
                        *pp = (*pp).offset(1);
                        i += 1
                    }
                    pdf_add_dict(
                        rule,
                        pdf_new_name(token),
                        pdf_new_string(tmp as *const libc::c_void, strlen(tmp)),
                    );
                    if verbose > 0i32 {
                        dpx_message(
                            b"otl_conf>> Current %s set to \"%s\"\n\x00" as *const u8 as *const i8,
                            token,
                            tmp,
                        );
                    }
                    free(tmp as *mut libc::c_void);
                }
            } else if streq_ptr(token, b"option\x00" as *const u8 as *const i8) {
                let mut opt_dict: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut opt_rule: *mut pdf_obj = 0 as *mut pdf_obj;
                opt_dict = pdf_lookup_dict(rule, b"option\x00" as *const u8 as *const i8);
                if opt_dict.is_null() {
                    opt_dict = pdf_new_dict();
                    pdf_add_dict(
                        rule,
                        pdf_new_name(b"option\x00" as *const u8 as *const i8),
                        opt_dict,
                    );
                }
                skip_white(pp, endptr);
                tmp = parse_c_ident(pp, endptr);
                if verbose > 0i32 {
                    dpx_message(
                        b"otl_conf>> Reading option \"%s\"\n\x00" as *const u8 as *const i8,
                        tmp,
                    );
                }
                skip_white(pp, endptr);
                opt_rule = parse_block(gclass, pp, endptr);
                pdf_add_dict(opt_dict, pdf_new_name(tmp), opt_rule);
                free(tmp as *mut libc::c_void);
            } else if streq_ptr(token, b"prefered\x00" as *const u8 as *const i8) as i32 != 0
                || streq_ptr(token, b"required\x00" as *const u8 as *const i8) as i32 != 0
                || streq_ptr(token, b"optional\x00" as *const u8 as *const i8) as i32 != 0
            {
                let mut subst: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut rule_block: *mut pdf_obj = 0 as *mut pdf_obj;
                if verbose > 0i32 {
                    dpx_message(
                        b"otl_conf>> Reading block (%s)\n\x00" as *const u8 as *const i8,
                        token,
                    );
                }
                skip_white(pp, endptr);
                if *pp >= endptr || **pp as i32 != '{' as i32 {
                    _tt_abort(b"Syntax error (1)\x00" as *const u8 as *const i8);
                }
                rule_block = parse_substrule(gclass, pp, endptr);
                subst = pdf_lookup_dict(rule, b"rule\x00" as *const u8 as *const i8);
                if subst.is_null() {
                    subst = pdf_new_array();
                    pdf_add_dict(
                        rule,
                        pdf_new_name(b"rule\x00" as *const u8 as *const i8),
                        subst,
                    );
                }
                pdf_add_array(subst, pdf_new_number(*token.offset(0) as f64));
                pdf_add_array(subst, rule_block);
            } else if *token.offset(0) as i32 == '@' as i32 {
                let mut coverage: *mut pdf_obj = 0 as *mut pdf_obj;
                skip_white(pp, endptr);
                *pp = (*pp).offset(1);
                skip_white(pp, endptr);
                if verbose > 0i32 {
                    dpx_message(
                        b"otl_conf>> Glyph class \"%s\"\n\x00" as *const u8 as *const i8,
                        token,
                    );
                }
                coverage = parse_uc_coverage(gclass, pp, endptr);
                if coverage.is_null() {
                    _tt_abort(b"No valid Unicode characters...\x00" as *const u8 as *const i8);
                }
                pdf_add_dict(gclass, pdf_new_name(&mut *token.offset(1)), coverage);
            }
            free(token as *mut libc::c_void);
            skip_white(pp, endptr);
        }
    }
    if *pp < endptr && **pp as i32 == '}' as i32 {
        *pp = (*pp).offset(1)
    }
    rule
}
unsafe extern "C" fn otl_read_conf(mut conf_name: *const i8) -> *mut pdf_obj {
    let mut rule: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut gclass: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut wbuf: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut endptr: *mut i8 = 0 as *mut i8;
    let mut pp: *const i8 = 0 as *const i8;
    let mut size: i32 = 0;
    let mut len: i32 = 0;
    filename = new((strlen(conf_name)
        .wrapping_add(strlen(b".otl\x00" as *const u8 as *const i8))
        .wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(filename, conf_name);
    strcat(filename, b".otl\x00" as *const u8 as *const i8);
    handle = ttstub_input_open(filename, TTIF_CNF, 0i32) as *mut rust_input_handle_t;
    if handle.is_null() {
        free(filename as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    size = ttstub_input_get_size(handle as rust_input_handle_t) as i32;
    if verbose > 0i32 {
        info!("\n");
        dpx_message(
            b"otl_conf>> Layout config. \"%s\" found: file=\"%s\" (%d bytes)\n\x00" as *const u8
                as *const i8,
            conf_name,
            filename,
            size,
        );
    }
    free(filename as *mut libc::c_void);
    if size < 1i32 {
        return 0 as *mut pdf_obj;
    }
    wbuf = new((size as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    p = wbuf;
    endptr = p.offset(size as isize);
    while size > 0i32 && p < endptr {
        len = ttstub_input_read(handle as rust_input_handle_t, p, size as size_t) as i32;
        if len < 0i32 {
            ttstub_input_close(handle as rust_input_handle_t);
            _tt_abort(
                b"error reading OTL configuration file \"%s\"\x00" as *const u8 as *const i8,
                filename,
            );
        }
        p = p.offset(len as isize);
        size -= len
    }
    ttstub_input_close(handle as rust_input_handle_t);
    pp = wbuf;
    gclass = pdf_new_dict();
    rule = parse_block(gclass, &mut pp, endptr);
    pdf_release_obj(gclass);
    free(wbuf as *mut libc::c_void);
    rule
}
static mut otl_confs: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
#[no_mangle]
pub unsafe extern "C" fn otl_find_conf(mut conf_name: *const i8) -> *mut pdf_obj {
    let mut rule: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut script: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut language: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut options: *mut pdf_obj = 0 as *mut pdf_obj;
    0 as *mut pdf_obj
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_get_script(mut conf: *mut pdf_obj) -> *mut i8 {
    let mut script: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!conf.is_null());
    script = pdf_lookup_dict(conf, b"script\x00" as *const u8 as *const i8);
    pdf_string_value(script) as *mut i8
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_get_language(mut conf: *mut pdf_obj) -> *mut i8 {
    let mut language: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!conf.is_null());
    language = pdf_lookup_dict(conf, b"language\x00" as *const u8 as *const i8);
    pdf_string_value(language) as *mut i8
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_get_rule(mut conf: *mut pdf_obj) -> *mut pdf_obj {
    assert!(!conf.is_null());
    pdf_lookup_dict(conf, b"rule\x00" as *const u8 as *const i8)
}
#[no_mangle]
pub unsafe extern "C" fn otl_conf_find_opt(
    mut conf: *mut pdf_obj,
    mut opt_tag: *const i8,
) -> *mut pdf_obj {
    let mut opt_conf: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut options: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!conf.is_null());
    options = pdf_lookup_dict(conf, b"option\x00" as *const u8 as *const i8);
    if !options.is_null() && !opt_tag.is_null() {
        opt_conf = pdf_lookup_dict(options, opt_tag)
    } else {
        opt_conf = 0 as *mut pdf_obj
    }
    opt_conf
}
#[no_mangle]
pub unsafe extern "C" fn otl_init_conf() {
    pdf_release_obj(otl_confs);
    otl_confs = pdf_new_dict();
    if verbose > 0i32 + 10i32 {
        pdf_release_obj(pdf_ref_obj(otl_confs));
    };
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
pub unsafe extern "C" fn otl_close_conf() {
    pdf_release_obj(otl_confs);
    otl_confs = 0 as *mut pdf_obj;
}
