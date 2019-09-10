#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn CMap_get_CIDSysInfo(cmap: *mut CMap) -> *mut CIDSysInfo;
    #[no_mangle]
    fn CMap_get_wmode(cmap: *mut CMap) -> libc::c_int;
    #[no_mangle]
    fn CMap_get_name(cmap: *mut CMap) -> *mut libc::c_char;
    #[no_mangle]
    fn CMap_is_Identity(cmap: *mut CMap) -> bool;
    #[no_mangle]
    fn CMap_is_valid(cmap: *mut CMap) -> bool;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    static mut CSI_IDENTITY: CIDSysInfo;
    #[no_mangle]
    static mut CSI_UNICODE: CIDSysInfo;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
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
    fn strlen(_: *const libc::c_char) -> u64;
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
    #[no_mangle]
    fn pdf_defineresource(
        category: *const libc::c_char,
        resname: *const libc::c_char,
        object: *mut pdf_obj,
        flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_findresource(category: *const libc::c_char, resname: *const libc::c_char)
        -> libc::c_int;
    #[no_mangle]
    fn pdf_get_resource_reference(res_id: libc::c_int) -> *mut pdf_obj;
}
pub type size_t = u64;
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
/* CIDFont types */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut libc::c_char,
    pub ordering: *mut libc::c_char,
    pub supplement: libc::c_int,
}
/* Codespacerange */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut libc::c_uchar,
    pub codeHi: *mut libc::c_uchar,
    /* Upper bounds of valid input code */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: libc::c_int,
    pub len: size_t,
    pub code: *mut libc::c_uchar,
    pub next: *mut mapDef,
    /* Next Subtbl for LOOKUP_CONTINUE */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut libc::c_uchar,
    pub prev: *mut mapData,
    pub pos: libc::c_int,
    /* Position of next free data segment */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub wmode: libc::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: libc::c_int,
    pub profile: C2RustUnnamed,
    pub reverseMap: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
    pub ranges: *mut rangeDef,
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
/*
 * References:
 *
 *  PostScript Language Reference Manual, 3rd. ed. (Adobe Systems Inc.)
 *    5.11.4 CMap Dictionaries
 *    5.11.5 FMapType 9 Composite Fonts
 *  Building CMap Files for CID-Keyed Fonts, Adobe Technical Note #5099
 *  CID-Keyed Font Technology Overview, Adobe Technical Note #5092
 *  Adobe CMap and CIDFont Files Specification, Adobe Technical Specification #5014
 *
 *  Undefined Character Handling:
 *    PLRM 3rd. ed., sec. 5.11.5., "Handling Undefined Characters"
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbuf {
    pub buf: *mut libc::c_char,
    pub curptr: *mut libc::c_char,
    pub limptr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub start: libc::c_int,
    pub count: libc::c_int,
}
unsafe extern "C" fn block_count(mut mtab: *mut mapDef, mut c: libc::c_int) -> size_t {
    let mut count: size_t = 0i32 as size_t;
    let mut n: size_t = 0;
    n = (*mtab.offset(c as isize))
        .len
        .wrapping_sub(1i32 as u64);
    c += 1i32;
    while c < 256i32 {
        if (*mtab.offset(c as isize)).flag & 1i32 << 4i32 != 0
            || (if (*mtab.offset(c as isize)).flag & 0xfi32 != 0i32 {
                1i32
            } else {
                0i32
            }) == 0
            || (*mtab.offset(c as isize)).flag & 0xfi32 != 1i32 << 0i32
                && (*mtab.offset(c as isize)).flag & 0xfi32 != 1i32 << 2i32
            || (*mtab.offset((c - 1i32) as isize)).len != (*mtab.offset(c as isize)).len
        {
            break;
        }
        if !(memcmp(
            (*mtab.offset((c - 1i32) as isize)).code as *const libc::c_void,
            (*mtab.offset(c as isize)).code as *const libc::c_void,
            n,
        ) == 0
            && (*(*mtab.offset((c - 1i32) as isize)).code.offset(n as isize) as libc::c_int)
                < 255i32
            && *(*mtab.offset((c - 1i32) as isize)).code.offset(n as isize) as libc::c_int + 1i32
                == *(*mtab.offset(c as isize)).code.offset(n as isize) as libc::c_int)
        {
            break;
        }
        count = count.wrapping_add(1);
        c += 1
    }
    return count;
}
unsafe extern "C" fn sputx(
    mut c: libc::c_uchar,
    mut s: *mut *mut libc::c_char,
    mut end: *mut libc::c_char,
) -> libc::c_int {
    let mut hi: libc::c_char = (c as libc::c_int >> 4i32) as libc::c_char;
    let mut lo: libc::c_char = (c as libc::c_int & 0xfi32) as libc::c_char;
    if (*s).offset(2) > end {
        _tt_abort(b"Buffer overflow.\x00" as *const u8 as *const libc::c_char);
    }
    **s = (if (hi as libc::c_int) < 10i32 {
        hi as libc::c_int + '0' as i32
    } else {
        hi as libc::c_int + '7' as i32
    }) as libc::c_char;
    *(*s).offset(1) = (if (lo as libc::c_int) < 10i32 {
        lo as libc::c_int + '0' as i32
    } else {
        lo as libc::c_int + '7' as i32
    }) as libc::c_char;
    *s = (*s).offset(2);
    return 2i32;
}
unsafe extern "C" fn write_map(
    mut mtab: *mut mapDef,
    mut count: size_t,
    mut codestr: *mut libc::c_uchar,
    mut depth: size_t,
    mut wbuf: *mut sbuf,
    mut stream: *mut pdf_obj,
) -> libc::c_int {
    let mut c: size_t = 0;
    let mut i: size_t = 0;
    let mut block_length: size_t = 0;
    let mut mtab1: *mut mapDef = 0 as *mut mapDef;
    /* Must be greater than 1 */
    let mut blocks: [C2RustUnnamed_1; 129] = [C2RustUnnamed_1 { start: 0, count: 0 }; 129];
    let mut num_blocks: size_t = 0i32 as size_t;
    c = 0i32 as size_t;
    while c < 256i32 as u64 {
        *codestr.offset(depth as isize) = (c & 0xffi32 as u64) as libc::c_uchar;
        if (*mtab.offset(c as isize)).flag & 1i32 << 4i32 != 0 {
            mtab1 = (*mtab.offset(c as isize)).next;
            count = write_map(
                mtab1,
                count,
                codestr,
                depth.wrapping_add(1i32 as u64),
                wbuf,
                stream,
            ) as size_t
        } else if if (*mtab.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } != 0
        {
            match (*mtab.offset(c as isize)).flag & 0xfi32 {
                1 | 4 => {
                    block_length = block_count(mtab, c as libc::c_int);
                    if block_length >= 2i32 as u64 {
                        blocks[num_blocks as usize].start = c as libc::c_int;
                        blocks[num_blocks as usize].count = block_length as libc::c_int;
                        num_blocks = num_blocks.wrapping_add(1);
                        c = (c as u64).wrapping_add(block_length) as size_t as size_t
                    } else {
                        let fresh0 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh0 = '<' as i32 as libc::c_char;
                        i = 0i32 as size_t;
                        while i <= depth {
                            sputx(
                                *codestr.offset(i as isize),
                                &mut (*wbuf).curptr,
                                (*wbuf).limptr,
                            );
                            i = i.wrapping_add(1)
                        }
                        let fresh1 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh1 = '>' as i32 as libc::c_char;
                        let fresh2 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh2 = ' ' as i32 as libc::c_char;
                        let fresh3 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh3 = '<' as i32 as libc::c_char;
                        i = 0i32 as size_t;
                        while i < (*mtab.offset(c as isize)).len {
                            sputx(
                                *(*mtab.offset(c as isize)).code.offset(i as isize),
                                &mut (*wbuf).curptr,
                                (*wbuf).limptr,
                            );
                            i = i.wrapping_add(1)
                        }
                        let fresh4 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh4 = '>' as i32 as libc::c_char;
                        let fresh5 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh5 = '\n' as i32 as libc::c_char;
                        count = count.wrapping_add(1)
                    }
                }
                2 => {
                    _tt_abort(
                        b"%s: Unexpected error...\x00" as *const u8 as *const libc::c_char,
                        b"CMap\x00" as *const u8 as *const libc::c_char,
                    );
                }
                8 => {}
                _ => {
                    _tt_abort(
                        b"%s: Unknown mapping type: %d\x00" as *const u8 as *const libc::c_char,
                        b"CMap\x00" as *const u8 as *const libc::c_char,
                        (*mtab.offset(c as isize)).flag & 0xfi32,
                    );
                }
            }
        }
        /* Flush if necessary */
        if count >= 100i32 as u64 || (*wbuf).curptr >= (*wbuf).limptr {
            let mut fmt_buf: [libc::c_char; 32] = [0; 32];
            if count > 100i32 as u64 {
                _tt_abort(
                    b"Unexpected error....: %zu\x00" as *const u8 as *const libc::c_char,
                    count,
                );
            }
            sprintf(
                fmt_buf.as_mut_ptr(),
                b"%zu beginbfchar\n\x00" as *const u8 as *const libc::c_char,
                count,
            );
            pdf_add_stream(
                stream,
                fmt_buf.as_mut_ptr() as *const libc::c_void,
                strlen(fmt_buf.as_mut_ptr()) as libc::c_int,
            );
            pdf_add_stream(
                stream,
                (*wbuf).buf as *const libc::c_void,
                (*wbuf).curptr.wrapping_offset_from((*wbuf).buf) as libc::c_long as libc::c_int,
            );
            (*wbuf).curptr = (*wbuf).buf;
            pdf_add_stream(
                stream,
                b"endbfchar\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"endbfchar\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
            );
            count = 0i32 as size_t
        }
        c = c.wrapping_add(1)
    }
    if num_blocks > 0i32 as u64 {
        let mut fmt_buf_0: [libc::c_char; 32] = [0; 32];
        if count > 0i32 as u64 {
            sprintf(
                fmt_buf_0.as_mut_ptr(),
                b"%zu beginbfchar\n\x00" as *const u8 as *const libc::c_char,
                count,
            );
            pdf_add_stream(
                stream,
                fmt_buf_0.as_mut_ptr() as *const libc::c_void,
                strlen(fmt_buf_0.as_mut_ptr()) as libc::c_int,
            );
            pdf_add_stream(
                stream,
                (*wbuf).buf as *const libc::c_void,
                (*wbuf).curptr.wrapping_offset_from((*wbuf).buf) as libc::c_long as libc::c_int,
            );
            (*wbuf).curptr = (*wbuf).buf;
            pdf_add_stream(
                stream,
                b"endbfchar\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"endbfchar\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
            );
            count = 0i32 as size_t
        }
        sprintf(
            fmt_buf_0.as_mut_ptr(),
            b"%zu beginbfrange\n\x00" as *const u8 as *const libc::c_char,
            num_blocks,
        );
        pdf_add_stream(
            stream,
            fmt_buf_0.as_mut_ptr() as *const libc::c_void,
            strlen(fmt_buf_0.as_mut_ptr()) as libc::c_int,
        );
        i = 0i32 as size_t;
        while i < num_blocks {
            let mut j: size_t = 0;
            c = blocks[i as usize].start as size_t;
            let fresh6 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh6 = '<' as i32 as libc::c_char;
            j = 0i32 as size_t;
            while j < depth {
                sputx(
                    *codestr.offset(j as isize),
                    &mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
                j = j.wrapping_add(1)
            }
            sputx(c as libc::c_uchar, &mut (*wbuf).curptr, (*wbuf).limptr);
            let fresh7 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh7 = '>' as i32 as libc::c_char;
            let fresh8 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh8 = ' ' as i32 as libc::c_char;
            let fresh9 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh9 = '<' as i32 as libc::c_char;
            j = 0i32 as size_t;
            while j < depth {
                sputx(
                    *codestr.offset(j as isize),
                    &mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
                j = j.wrapping_add(1)
            }
            sputx(
                c.wrapping_add(blocks[i as usize].count as u64) as libc::c_uchar,
                &mut (*wbuf).curptr,
                (*wbuf).limptr,
            );
            let fresh10 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh10 = '>' as i32 as libc::c_char;
            let fresh11 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh11 = ' ' as i32 as libc::c_char;
            let fresh12 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh12 = '<' as i32 as libc::c_char;
            j = 0i32 as size_t;
            while j < (*mtab.offset(c as isize)).len {
                sputx(
                    *(*mtab.offset(c as isize)).code.offset(j as isize),
                    &mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
                j = j.wrapping_add(1)
            }
            let fresh13 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh13 = '>' as i32 as libc::c_char;
            let fresh14 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh14 = '\n' as i32 as libc::c_char;
            i = i.wrapping_add(1)
        }
        pdf_add_stream(
            stream,
            (*wbuf).buf as *const libc::c_void,
            (*wbuf).curptr.wrapping_offset_from((*wbuf).buf) as libc::c_long as libc::c_int,
        );
        (*wbuf).curptr = (*wbuf).buf;
        pdf_add_stream(
            stream,
            b"endbfrange\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            strlen(b"endbfrange\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
        );
    }
    return count as libc::c_int;
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
pub unsafe extern "C" fn CMap_create_stream(mut cmap: *mut CMap) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut csi: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
    let mut wbuf: sbuf = sbuf {
        buf: 0 as *mut libc::c_char,
        curptr: 0 as *mut libc::c_char,
        limptr: 0 as *mut libc::c_char,
    };
    let mut ranges: *mut rangeDef = 0 as *mut rangeDef;
    let mut codestr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut count: size_t = 0i32 as size_t;
    if cmap.is_null() || !CMap_is_valid(cmap) {
        dpx_warning(b"Invalid CMap\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut pdf_obj;
    }
    if (*cmap).type_0 == 0i32 {
        return 0 as *mut pdf_obj;
    }
    stream = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(stream);
    csi = CMap_get_CIDSysInfo(cmap);
    if csi.is_null() {
        csi = if (*cmap).type_0 != 2i32 {
            &mut CSI_IDENTITY
        } else {
            &mut CSI_UNICODE
        }
    }
    if (*cmap).type_0 != 2i32 {
        let mut csi_dict: *mut pdf_obj = 0 as *mut pdf_obj;
        csi_dict = pdf_new_dict();
        pdf_add_dict(
            csi_dict,
            pdf_new_name(b"Registry\x00" as *const u8 as *const libc::c_char),
            pdf_new_string(
                (*csi).registry as *const libc::c_void,
                strlen((*csi).registry),
            ),
        );
        pdf_add_dict(
            csi_dict,
            pdf_new_name(b"Ordering\x00" as *const u8 as *const libc::c_char),
            pdf_new_string(
                (*csi).ordering as *const libc::c_void,
                strlen((*csi).ordering),
            ),
        );
        pdf_add_dict(
            csi_dict,
            pdf_new_name(b"Supplement\x00" as *const u8 as *const libc::c_char),
            pdf_new_number((*csi).supplement as libc::c_double),
        );
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"CMap\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"CMapName\x00" as *const u8 as *const libc::c_char),
            pdf_new_name((*cmap).name),
        );
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"CIDSystemInfo\x00" as *const u8 as *const libc::c_char),
            csi_dict,
        );
        if (*cmap).wmode != 0i32 {
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"WMode\x00" as *const u8 as *const libc::c_char),
                pdf_new_number((*cmap).wmode as libc::c_double),
            );
        }
    }
    /* TODO:
     * Predefined CMaps need not to be embedded.
     */
    if !(*cmap).useCMap.is_null() {
        _tt_abort(b"UseCMap found (not supported yet)...\x00" as *const u8 as *const libc::c_char);
    }
    wbuf.buf = new((4096i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as u64)
        as u32) as *mut libc::c_char;
    codestr = new(((*cmap).profile.maxBytesIn as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
        as u32) as *mut libc::c_uchar;
    memset(
        codestr as *mut libc::c_void,
        0i32,
        (*cmap).profile.maxBytesIn,
    );
    wbuf.curptr = wbuf.buf;
    wbuf.limptr = wbuf
        .buf
        .offset(4096)
        .offset(
            -((2i32 as u64).wrapping_mul(
                (*cmap)
                    .profile
                    .maxBytesIn
                    .wrapping_add((*cmap).profile.maxBytesOut),
            ) as isize),
        )
        .offset(16);
    /* Start CMap */
    pdf_add_stream(
        stream,
        b"/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\x00" as *const u8
            as *const libc::c_char as *const libc::c_void,
        strlen(
            b"/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\x00" as *const u8
                as *const libc::c_char,
        ) as libc::c_int,
    );
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CMapName /%s def\n\x00" as *const u8 as *const libc::c_char,
        (*cmap).name,
    ) as isize);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CMapType %d def\n\x00" as *const u8 as *const libc::c_char,
        (*cmap).type_0,
    ) as isize);
    if (*cmap).wmode != 0i32 && (*cmap).type_0 != 2i32 {
        wbuf.curptr = wbuf.curptr.offset(sprintf(
            wbuf.curptr,
            b"/WMode %d def\n\x00" as *const u8 as *const libc::c_char,
            (*cmap).wmode,
        ) as isize)
    }
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CIDSystemInfo <<\n  /Registry (%s)\n  /Ordering (%s)\n  /Supplement %d\n>> def\n\x00"
            as *const u8 as *const libc::c_char,
        (*csi).registry,
        (*csi).ordering,
        (*csi).supplement,
    ) as isize);
    pdf_add_stream(
        stream,
        wbuf.buf as *const libc::c_void,
        wbuf.curptr.wrapping_offset_from(wbuf.buf) as libc::c_long as libc::c_int,
    );
    wbuf.curptr = wbuf.buf;
    /* codespacerange */
    ranges = (*cmap).codespace.ranges;
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"%d begincodespacerange\n\x00" as *const u8 as *const libc::c_char,
        (*cmap).codespace.num,
    ) as isize);
    i = 0i32 as size_t;
    while i < (*cmap).codespace.num as u64 {
        let fresh15 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh15 = '<' as i32 as libc::c_char;
        j = 0i32 as size_t;
        while j < (*ranges.offset(i as isize)).dim {
            sputx(
                *(*ranges.offset(i as isize)).codeLo.offset(j as isize),
                &mut wbuf.curptr,
                wbuf.limptr,
            );
            j = j.wrapping_add(1)
        }
        let fresh16 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh16 = '>' as i32 as libc::c_char;
        let fresh17 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh17 = ' ' as i32 as libc::c_char;
        let fresh18 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh18 = '<' as i32 as libc::c_char;
        j = 0i32 as size_t;
        while j < (*ranges.offset(i as isize)).dim {
            sputx(
                *(*ranges.offset(i as isize)).codeHi.offset(j as isize),
                &mut wbuf.curptr,
                wbuf.limptr,
            );
            j = j.wrapping_add(1)
        }
        let fresh19 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh19 = '>' as i32 as libc::c_char;
        let fresh20 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh20 = '\n' as i32 as libc::c_char;
        i = i.wrapping_add(1)
    }
    pdf_add_stream(
        stream,
        wbuf.buf as *const libc::c_void,
        wbuf.curptr.wrapping_offset_from(wbuf.buf) as libc::c_long as libc::c_int,
    );
    wbuf.curptr = wbuf.buf;
    pdf_add_stream(
        stream,
        b"endcodespacerange\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"endcodespacerange\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    /* CMap body */
    if !(*cmap).mapTbl.is_null() {
        count = write_map(
            (*cmap).mapTbl,
            0i32 as size_t,
            codestr,
            0i32 as size_t,
            &mut wbuf,
            stream,
        ) as size_t; /* Top node */
        if count > 0i32 as u64 {
            /* Flush */
            let mut fmt_buf: [libc::c_char; 32] = [0; 32];
            if count > 100i32 as u64 {
                _tt_abort(
                    b"Unexpected error....: %zu\x00" as *const u8 as *const libc::c_char,
                    count,
                );
            }
            sprintf(
                fmt_buf.as_mut_ptr(),
                b"%zu beginbfchar\n\x00" as *const u8 as *const libc::c_char,
                count,
            );
            pdf_add_stream(
                stream,
                fmt_buf.as_mut_ptr() as *const libc::c_void,
                strlen(fmt_buf.as_mut_ptr()) as libc::c_int,
            );
            pdf_add_stream(
                stream,
                wbuf.buf as *const libc::c_void,
                wbuf.curptr.wrapping_offset_from(wbuf.buf) as libc::c_long as libc::c_int,
            );
            pdf_add_stream(
                stream,
                b"endbfchar\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"endbfchar\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
            );
            count = 0i32 as size_t;
            wbuf.curptr = wbuf.buf
        }
    }
    /* End CMap */
    pdf_add_stream(
        stream,
        b"endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n\x00" as *const u8
            as *const libc::c_char as *const libc::c_void,
        strlen(
            b"endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n\x00" as *const u8
                as *const libc::c_char,
        ) as libc::c_int,
    );
    free(codestr as *mut libc::c_void);
    free(wbuf.buf as *mut libc::c_void);
    return stream;
}
