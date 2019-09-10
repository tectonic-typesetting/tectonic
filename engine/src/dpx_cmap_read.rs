#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pst_obj;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn CMap_cache_get(id: libc::c_int) -> *mut CMap;
    #[no_mangle]
    fn CMap_add_codespacerange(
        cmap: *mut CMap,
        codelo: *const libc::c_uchar,
        codehi: *const libc::c_uchar,
        dim: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_notdefrange(
        cmap: *mut CMap,
        srclo: *const libc::c_uchar,
        srchi: *const libc::c_uchar,
        srcdim: size_t,
        dst: CID,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_notdefchar(
        cmap: *mut CMap,
        src: *const libc::c_uchar,
        srcdim: size_t,
        dst: CID,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_cidrange(
        cmap: *mut CMap,
        srclo: *const libc::c_uchar,
        hi: *const libc::c_uchar,
        srcdim: size_t,
        base: CID,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_bfrange(
        cmap: *mut CMap,
        srclo: *const libc::c_uchar,
        srchi: *const libc::c_uchar,
        srcdim: size_t,
        dest: *const libc::c_uchar,
        destdim: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_cidchar(
        cmap: *mut CMap,
        src: *const libc::c_uchar,
        srcdim: size_t,
        dest: CID,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_bfchar(
        cmap: *mut CMap,
        src: *const libc::c_uchar,
        srcdim: size_t,
        dest: *const libc::c_uchar,
        destdim: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn CMap_set_CIDSysInfo(cmap: *mut CMap, csi: *const CIDSysInfo);
    #[no_mangle]
    fn CMap_set_usecmap(cmap: *mut CMap, ucmap: *mut CMap);
    #[no_mangle]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: libc::c_int);
    #[no_mangle]
    fn CMap_set_type(cmap: *mut CMap, type_0: libc::c_int);
    #[no_mangle]
    fn CMap_set_name(cmap: *mut CMap, name: *const libc::c_char);
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: u64) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn CMap_is_valid(cmap: *mut CMap) -> bool;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
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
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pst_get_token(inbuf: *mut *mut libc::c_uchar, inbufend: *mut libc::c_uchar) -> *mut pst_obj;
    #[no_mangle]
    fn pst_release_obj(obj: *mut pst_obj);
    #[no_mangle]
    fn pst_type_of(obj: *mut pst_obj) -> pst_type;
    #[no_mangle]
    fn pst_length_of(obj: *mut pst_obj) -> libc::c_int;
    #[no_mangle]
    fn pst_getIV(obj: *mut pst_obj) -> libc::c_int;
    #[no_mangle]
    fn pst_getSV(obj: *mut pst_obj) -> *mut libc::c_uchar;
    #[no_mangle]
    fn pst_data_ptr(obj: *mut pst_obj) -> *mut libc::c_void;
}
pub type __ssize_t = libc::c_long;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
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
/* Mapping types, MAP_IS_NAME is not supported. */
/* Lookup flags */
/* DEBUG */
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
pub type CID = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreader {
    pub cursor: *mut libc::c_uchar,
    pub endptr: *mut libc::c_uchar,
    pub buf: *mut libc::c_uchar,
    pub max: size_t,
    pub handle: rust_input_handle_t,
    pub unread: size_t,
}
pub type pst_type = libc::c_int;
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
static mut __verbose: libc::c_int = 0i32;
unsafe extern "C" fn ifreader_create(
    mut handle: rust_input_handle_t,
    mut size: size_t,
    mut bufsize: size_t,
) -> *mut ifreader {
    let mut reader: *mut ifreader = 0 as *mut ifreader;
    reader = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<ifreader>() as u64)
        as u32) as *mut ifreader;
    (*reader).buf = new(
        (bufsize.wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
            as u32,
    ) as *mut libc::c_uchar;
    (*reader).max = bufsize;
    (*reader).handle = handle;
    (*reader).unread = size;
    (*reader).endptr = (*reader).buf;
    (*reader).cursor = (*reader).endptr;
    *(*reader).endptr = 0i32 as libc::c_uchar;
    return reader;
}
unsafe extern "C" fn ifreader_destroy(mut reader: *mut ifreader) {
    if !reader.is_null() {
    } else {
        __assert_fail(
            b"reader\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap_read.c\x00" as *const u8 as *const libc::c_char,
            77i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void ifreader_destroy(ifreader *)\x00",
            ))
            .as_ptr(),
        );
    }
    free((*reader).buf as *mut libc::c_void);
    free(reader as *mut libc::c_void);
}
unsafe extern "C" fn ifreader_read(mut reader: *mut ifreader, mut size: size_t) -> size_t {
    let mut bytesread: size_t = 0i32 as size_t;
    let mut bytesrem: size_t = 0i32 as size_t;
    if !reader.is_null() {
    } else {
        __assert_fail(
            b"reader\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap_read.c\x00" as *const u8 as *const libc::c_char,
            88i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"size_t ifreader_read(ifreader *, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    bytesrem = ((*reader).endptr as size_t).wrapping_sub((*reader).cursor as size_t);
    if size > (*reader).max {
        if __verbose != 0 {
            dpx_message(
                b"\nExtending buffer (%zu bytes)...\n\x00" as *const u8 as *const libc::c_char,
                size,
            );
        }
        (*reader).buf = renew(
            (*reader).buf as *mut libc::c_void,
            (size.wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
                as u32,
        ) as *mut libc::c_uchar;
        (*reader).max = size
    }
    if (*reader).unread > 0i32 as u64 && bytesrem < size {
        bytesread = if (*reader).max.wrapping_sub(bytesrem) < (*reader).unread {
            (*reader).max.wrapping_sub(bytesrem)
        } else {
            (*reader).unread
        };
        memmove(
            (*reader).buf as *mut libc::c_void,
            (*reader).cursor as *const libc::c_void,
            bytesrem,
        );
        (*reader).cursor = (*reader).buf;
        (*reader).endptr = (*reader).buf.offset(bytesrem as isize);
        if ttstub_input_read(
            (*reader).handle,
            (*reader).endptr as *mut libc::c_char,
            bytesread,
        ) as u64
            != bytesread
        {
            _tt_abort(b"Reading file failed.\x00" as *const u8 as *const libc::c_char);
        }
        (*reader).endptr = (*reader).endptr.offset(bytesread as isize);
        (*reader).unread =
            ((*reader).unread as u64).wrapping_sub(bytesread) as size_t as size_t;
        if __verbose != 0 {
            dpx_message(
                b"Reading more %zu bytes (%zu bytes remains in buffer)...\n\x00" as *const u8
                    as *const libc::c_char,
                bytesread,
                bytesrem,
            );
        }
    }
    *(*reader).endptr = 0i32 as libc::c_uchar;
    return bytesread.wrapping_add(bytesrem);
}
unsafe extern "C" fn check_next_token(
    mut input: *mut ifreader,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut cmp: libc::c_int = 0;
    let mut token: *mut pst_obj = 0 as *mut pst_obj;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if ifreader_read(input, strlen(key)) == 0i32 as u64 {
        return -1i32;
    }
    token = pst_get_token(&mut (*input).cursor, (*input).endptr);
    if token.is_null() {
        return -1i32;
    }
    str = pst_getSV(token) as *mut libc::c_char;
    cmp = if strcmp(str, key) != 0 { -1i32 } else { 0i32 };
    free(str as *mut libc::c_void);
    pst_release_obj(token);
    return cmp;
}
unsafe extern "C" fn get_coderange(
    mut input: *mut ifreader,
    mut codeLo: *mut libc::c_uchar,
    mut codeHi: *mut libc::c_uchar,
    mut dim: *mut libc::c_int,
    mut maxlen: libc::c_int,
) -> libc::c_int {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut dim1: libc::c_int = 0;
    let mut dim2: libc::c_int = 0;
    tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
    if tok1.is_null() {
        return -1i32;
    }
    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
    if tok2.is_null() {
        pst_release_obj(tok1);
        return -1i32;
    }
    if !(pst_type_of(tok1) == 5i32) || !(pst_type_of(tok2) == 5i32) {
        pst_release_obj(tok1);
        pst_release_obj(tok2);
        return -1i32;
    }
    dim1 = pst_length_of(tok1);
    dim2 = pst_length_of(tok2);
    if dim1 != dim2 || dim1 > maxlen {
        pst_release_obj(tok1);
        pst_release_obj(tok2);
        return -1i32;
    }
    memcpy(
        codeLo as *mut libc::c_void,
        pst_data_ptr(tok1),
        dim1 as u64,
    );
    memcpy(
        codeHi as *mut libc::c_void,
        pst_data_ptr(tok2),
        dim2 as u64,
    );
    pst_release_obj(tok1);
    pst_release_obj(tok2);
    *dim = dim1;
    return 0i32;
}
unsafe extern "C" fn do_codespacerange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut codeLo: [libc::c_uchar; 127] = [0; 127];
    let mut codeHi: [libc::c_uchar; 127] = [0; 127];
    let mut dim: libc::c_int = 0;
    loop {
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 > 0i32) {
            break;
        }
        if get_coderange(
            input,
            codeLo.as_mut_ptr(),
            codeHi.as_mut_ptr(),
            &mut dim,
            127i32,
        ) < 0i32
        {
            return -1i32;
        }
        CMap_add_codespacerange(
            cmap,
            codeLo.as_mut_ptr(),
            codeHi.as_mut_ptr(),
            dim as size_t,
        );
    }
    return check_next_token(
        input,
        b"endcodespacerange\x00" as *const u8 as *const libc::c_char,
    );
}
/*
 * bfrange
 *  <codeLo> <codeHi> [destCode1 destCode2 ...]
 */
unsafe extern "C" fn handle_codearray(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut codeLo: *mut libc::c_uchar,
    mut dim: libc::c_int,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    if dim < 1i32 {
        _tt_abort(b"Invalid code range.\x00" as *const u8 as *const libc::c_char);
    }
    loop {
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        tok = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok.is_null() {
            return -1i32;
        } else {
            if pst_type_of(tok) == 5i32 {
                CMap_add_bfchar(
                    cmap,
                    codeLo,
                    dim as size_t,
                    pst_data_ptr(tok) as *mut libc::c_uchar,
                    pst_length_of(tok) as size_t,
                );
            } else if pst_type_of(tok) == 7i32 || !(pst_type_of(tok) == 6i32) {
                _tt_abort(
                    b"%s: Invalid CMap mapping record.\x00" as *const u8 as *const libc::c_char,
                    b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                _tt_abort(
                    b"%s: Mapping to charName not supported.\x00" as *const u8
                        as *const libc::c_char,
                    b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        pst_release_obj(tok);
        let ref mut fresh2 = *codeLo.offset((dim - 1i32) as isize);
        *fresh2 = (*fresh2 as libc::c_int + 1i32) as libc::c_uchar
    }
    return check_next_token(input, b"]\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn do_notdefrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut codeLo: [libc::c_uchar; 127] = [0; 127];
    let mut codeHi: [libc::c_uchar; 127] = [0; 127];
    let mut dstCID: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    loop {
        let fresh3 = count;
        count = count - 1;
        if !(fresh3 > 0i32) {
            break;
        }
        if ifreader_read(input, (127i32 * 3i32) as size_t) == 0i32 as u64 {
            return -1i32;
        }
        if get_coderange(
            input,
            codeLo.as_mut_ptr(),
            codeHi.as_mut_ptr(),
            &mut dim,
            127i32,
        ) < 0i32
            || {
                tok = pst_get_token(&mut (*input).cursor, (*input).endptr);
                tok.is_null()
            }
        {
            return -1i32;
        }
        if pst_type_of(tok) == 2i32 {
            dstCID = pst_getIV(tok);
            if dstCID >= 0i32 && dstCID <= 65535i32 {
                CMap_add_notdefrange(
                    cmap,
                    codeLo.as_mut_ptr(),
                    codeHi.as_mut_ptr(),
                    dim as size_t,
                    dstCID as CID,
                );
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8
                    as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        }
        pst_release_obj(tok);
    }
    return check_next_token(
        input,
        b"endnotdefrange\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn do_bfrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut codeLo: [libc::c_uchar; 127] = [0; 127];
    let mut codeHi: [libc::c_uchar; 127] = [0; 127];
    let mut srcdim: libc::c_int = 0;
    loop {
        let fresh4 = count;
        count = count - 1;
        if !(fresh4 > 0i32) {
            break;
        }
        if ifreader_read(input, (127i32 * 3i32) as size_t) == 0i32 as u64 {
            return -1i32;
        }
        if get_coderange(
            input,
            codeLo.as_mut_ptr(),
            codeHi.as_mut_ptr(),
            &mut srcdim,
            127i32,
        ) < 0i32
            || {
                tok = pst_get_token(&mut (*input).cursor, (*input).endptr);
                tok.is_null()
            }
        {
            return -1i32;
        }
        if pst_type_of(tok) == 5i32 {
            CMap_add_bfrange(
                cmap,
                codeLo.as_mut_ptr(),
                codeHi.as_mut_ptr(),
                srcdim as size_t,
                pst_data_ptr(tok) as *mut libc::c_uchar,
                pst_length_of(tok) as size_t,
            );
        } else if pst_type_of(tok) == 7i32 {
            if handle_codearray(
                cmap,
                input,
                codeLo.as_mut_ptr(),
                srcdim,
                codeHi[(srcdim - 1i32) as usize] as libc::c_int
                    - codeLo[(srcdim - 1i32) as usize] as libc::c_int
                    + 1i32,
            ) < 0i32
            {
                pst_release_obj(tok);
                return -1i32;
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8
                    as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        }
        pst_release_obj(tok);
    }
    return check_next_token(input, b"endbfrange\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn do_cidrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut codeLo: [libc::c_uchar; 127] = [0; 127];
    let mut codeHi: [libc::c_uchar; 127] = [0; 127];
    let mut dstCID: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    loop {
        let fresh5 = count;
        count = count - 1;
        if !(fresh5 > 0i32) {
            break;
        }
        if ifreader_read(input, (127i32 * 3i32) as size_t) == 0i32 as u64 {
            return -1i32;
        }
        if get_coderange(
            input,
            codeLo.as_mut_ptr(),
            codeHi.as_mut_ptr(),
            &mut dim,
            127i32,
        ) < 0i32
            || {
                tok = pst_get_token(&mut (*input).cursor, (*input).endptr);
                tok.is_null()
            }
        {
            return -1i32;
        }
        if pst_type_of(tok) == 2i32 {
            dstCID = pst_getIV(tok);
            if dstCID >= 0i32 && dstCID <= 65535i32 {
                CMap_add_cidrange(
                    cmap,
                    codeLo.as_mut_ptr(),
                    codeHi.as_mut_ptr(),
                    dim as size_t,
                    dstCID as CID,
                );
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8
                    as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        }
        pst_release_obj(tok);
    }
    return check_next_token(
        input,
        b"endcidrange\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn do_notdefchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut dstCID: libc::c_int = 0;
    loop {
        let fresh6 = count;
        count = count - 1;
        if !(fresh6 > 0i32) {
            break;
        }
        if ifreader_read(input, (127i32 * 2i32) as size_t) == 0i32 as u64 {
            return -1i32;
        }
        tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            return -1i32;
        }
        tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok2.is_null() {
            pst_release_obj(tok1);
            return -1i32;
        }
        if pst_type_of(tok1) == 5i32 && pst_type_of(tok2) == 2i32 {
            dstCID = pst_getIV(tok2);
            if dstCID >= 0i32 && dstCID <= 65535i32 {
                CMap_add_notdefchar(
                    cmap,
                    pst_data_ptr(tok1) as *const libc::c_uchar,
                    pst_length_of(tok1) as size_t,
                    dstCID as CID,
                );
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8
                    as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    return check_next_token(
        input,
        b"endnotdefchar\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn do_bfchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    loop {
        let fresh7 = count;
        count = count - 1;
        if !(fresh7 > 0i32) {
            break;
        }
        if ifreader_read(input, (127i32 * 2i32) as size_t) == 0i32 as u64 {
            return -1i32;
        }
        tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            return -1i32;
        }
        tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok2.is_null() {
            pst_release_obj(tok1);
            return -1i32;
        }
        /* We only support single CID font as descendant font, charName should not come here. */
        if pst_type_of(tok1) == 5i32 && pst_type_of(tok2) == 5i32 {
            CMap_add_bfchar(
                cmap,
                pst_data_ptr(tok1) as *mut libc::c_uchar,
                pst_length_of(tok1) as size_t,
                pst_data_ptr(tok2) as *mut libc::c_uchar,
                pst_length_of(tok2) as size_t,
            );
        } else if pst_type_of(tok2) == 6i32 {
            _tt_abort(
                b"%s: Mapping to charName not supported.\x00" as *const u8 as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8
                    as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    return check_next_token(input, b"endbfchar\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn do_cidchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut dstCID: libc::c_int = 0;
    loop {
        let fresh8 = count;
        count = count - 1;
        if !(fresh8 > 0i32) {
            break;
        }
        if ifreader_read(input, (127i32 * 2i32) as size_t) == 0i32 as u64 {
            return -1i32;
        }
        tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            return -1i32;
        }
        tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok2.is_null() {
            pst_release_obj(tok1);
            return -1i32;
        }
        if pst_type_of(tok1) == 5i32 && pst_type_of(tok2) == 2i32 {
            dstCID = pst_getIV(tok2);
            if dstCID >= 0i32 && dstCID <= 65535i32 {
                CMap_add_cidchar(
                    cmap,
                    pst_data_ptr(tok1) as *const libc::c_uchar,
                    pst_length_of(tok1) as size_t,
                    dstCID as CID,
                );
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8
                    as *const libc::c_char,
                b"CMap_parse:\x00" as *const u8 as *const libc::c_char,
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    return check_next_token(input, b"endcidchar\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn do_cidsysteminfo(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
) -> libc::c_int {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut csi: CIDSysInfo = {
        let mut init = CIDSysInfo {
            registry: 0 as *mut libc::c_char,
            ordering: 0 as *mut libc::c_char,
            supplement: -1i32,
        };
        init
    };
    let mut simpledict: libc::c_int = 0i32;
    let mut error: libc::c_int = 0i32;
    ifreader_read(input, (127i32 * 2i32) as size_t);
    loop
    /*
     * Assuming /CIDSystemInfo 3 dict dup begin .... end def
     * or /CIDSystemInfo << ... >> def
     */
    {
        tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            break;
        }
        if pst_type_of(tok1) == 7i32 {
            simpledict = 1i32;
            pst_release_obj(tok1);
            break;
        } else if pst_type_of(tok1) < 0i32
            && memcmp(
                pst_data_ptr(tok1),
                b"begin\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"begin\x00" as *const u8 as *const libc::c_char),
            ) == 0
        {
            simpledict = 0i32;
            pst_release_obj(tok1);
            break;
        } else {
            pst_release_obj(tok1);
            /* continue */
        }
    }
    tok2 = 0 as *mut pst_obj;
    tok1 = tok2;
    while error == 0 && {
        tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        !tok1.is_null()
    } {
        if pst_type_of(tok1) < 0i32
            && memcmp(
                pst_data_ptr(tok1),
                b">>\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b">>\x00" as *const u8 as *const libc::c_char),
            ) == 0
            && simpledict != 0
        {
            pst_release_obj(tok1);
            break;
        } else if pst_type_of(tok1) < 0i32
            && memcmp(
                pst_data_ptr(tok1),
                b"end\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"end\x00" as *const u8 as *const libc::c_char),
            ) == 0
            && simpledict == 0
        {
            pst_release_obj(tok1);
            break;
        } else {
            if pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Registry\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"Registry\x00" as *const u8 as *const libc::c_char),
                ) == 0
                && {
                    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == 5i32) {
                    error = -1i32
                } else if simpledict == 0
                    && check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) != 0
                {
                    error = -1i32
                }
                if error == 0 {
                    csi.registry = pst_getSV(tok2) as *mut libc::c_char
                }
            } else if pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Ordering\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"Ordering\x00" as *const u8 as *const libc::c_char),
                ) == 0
                && {
                    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == 5i32) {
                    error = -1i32
                } else if simpledict == 0
                    && check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) != 0
                {
                    error = -1i32
                }
                if error == 0 {
                    csi.ordering = pst_getSV(tok2) as *mut libc::c_char
                }
            } else if pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Supplement\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"Supplement\x00" as *const u8 as *const libc::c_char),
                ) == 0
                && {
                    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == 2i32) {
                    error = -1i32
                } else if simpledict == 0
                    && check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) != 0
                {
                    error = -1i32
                }
                if error == 0 {
                    csi.supplement = pst_getIV(tok2)
                }
            }
            if !tok2.is_null() {
                pst_release_obj(tok2);
            }
            if !tok1.is_null() {
                pst_release_obj(tok1);
            }
            tok2 = 0 as *mut pst_obj;
            tok1 = tok2
        }
    }
    if error == 0 && check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) != 0 {
        error = -1i32
    }
    if error == 0 && !csi.registry.is_null() && !csi.ordering.is_null() && csi.supplement >= 0i32 {
        CMap_set_CIDSysInfo(cmap, &mut csi);
    }
    free(csi.registry as *mut libc::c_void);
    free(csi.ordering as *mut libc::c_void);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_parse_check_sig(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut result: libc::c_int = -1i32;
    let mut sig: [libc::c_char; 65] = [0; 65];
    if handle.is_null() {
        return -1i32;
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(handle, sig.as_mut_ptr(), 64i32 as size_t) != 64i32 as libc::c_long {
        result = -1i32
    } else {
        sig[64] = 0i32 as libc::c_char;
        if strstartswith(
            sig.as_mut_ptr(),
            b"%!PS\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
            result = -1i32
        } else if !strstr(
            sig.as_mut_ptr().offset(4),
            b"Resource-CMap\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
            result = 0i32
        }
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    return result;
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
pub unsafe extern "C" fn CMap_parse(
    mut cmap: *mut CMap,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj; /* else Simply ignore */
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut input: *mut ifreader = 0 as *mut ifreader;
    let mut status: libc::c_int = 0i32;
    let mut tmpint: libc::c_int = -1i32;
    if !cmap.is_null() && !handle.is_null() {
    } else {
        __assert_fail(
            b"cmap && handle\x00" as *const u8 as *const libc::c_char,
            b"dpx-cmap_read.c\x00" as *const u8 as *const libc::c_char,
            519i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"int CMap_parse(CMap *, rust_input_handle_t)\x00",
            ))
            .as_ptr(),
        );
    }
    input = ifreader_create(
        handle,
        ttstub_input_get_size(handle),
        (4096i32 - 1i32) as size_t,
    );
    while status >= 0i32 {
        tok2 = 0 as *mut pst_obj;
        tok1 = tok2;
        ifreader_read(input, (4096i32 / 2i32) as size_t);
        tok1 = pst_get_token(&mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            break;
        }
        if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"CMapName\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"CMapName\x00" as *const u8 as *const libc::c_char),
            ) == 0
        {
            tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == 6i32 || pst_type_of(tok2) == 5i32)
                || check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) < 0i32
            {
                status = -1i32
            } else {
                CMap_set_name(cmap, pst_data_ptr(tok2) as *const libc::c_char);
            }
        } else if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"CMapType\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"CMapType\x00" as *const u8 as *const libc::c_char),
            ) == 0
        {
            tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == 2i32)
                || check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) < 0i32
            {
                status = -1i32
            } else {
                CMap_set_type(cmap, pst_getIV(tok2));
            }
        } else if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"WMode\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"WMode\x00" as *const u8 as *const libc::c_char),
            ) == 0
        {
            tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == 2i32)
                || check_next_token(input, b"def\x00" as *const u8 as *const libc::c_char) < 0i32
            {
                status = -1i32
            } else {
                CMap_set_wmode(cmap, pst_getIV(tok2));
            }
        } else if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"CIDSystemInfo\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"CIDSystemInfo\x00" as *const u8 as *const libc::c_char),
            ) == 0
        {
            status = do_cidsysteminfo(cmap, input)
        } else if !(pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"Version\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"Version\x00" as *const u8 as *const libc::c_char),
            ) == 0
            || pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"UIDOffset\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"UIDOffset\x00" as *const u8 as *const libc::c_char),
                ) == 0
            || pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"XUID\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"XUID\x00" as *const u8 as *const libc::c_char),
                ) == 0)
        {
            if pst_type_of(tok1) == 6i32 {
                /* Possibly usecmap comes next */
                tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                if !tok2.is_null()
                    && (pst_type_of(tok2) < 0i32
                        && memcmp(
                            pst_data_ptr(tok2),
                            b"usecmap\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            strlen(b"usecmap\x00" as *const u8 as *const libc::c_char),
                        ) == 0)
                {
                    let mut id: libc::c_int = 0;
                    let mut ucmap: *mut CMap = 0 as *mut CMap;
                    id = CMap_cache_find(pst_data_ptr(tok1) as *const libc::c_char);
                    if id < 0i32 {
                        status = -1i32
                    } else {
                        ucmap = CMap_cache_get(id);
                        CMap_set_usecmap(cmap, ucmap);
                    }
                }
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincodespacerange\x00" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    strlen(b"begincodespacerange\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_codespacerange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginnotdefrange\x00" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    strlen(b"beginnotdefrange\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_notdefrange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginnotdefchar\x00" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    strlen(b"beginnotdefchar\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_notdefchar(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginbfrange\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"beginbfrange\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_bfrange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginbfchar\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"beginbfchar\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_bfchar(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincidrange\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"begincidrange\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_cidrange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincidchar\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    strlen(b"begincidchar\x00" as *const u8 as *const libc::c_char),
                ) == 0
            {
                status = do_cidchar(cmap, input, tmpint)
            } else if pst_type_of(tok1) == 2i32 {
                tmpint = pst_getIV(tok1)
            }
        }
        if !tok1.is_null() {
            pst_release_obj(tok1);
        }
        if !tok2.is_null() {
            pst_release_obj(tok2);
        }
    }
    ifreader_destroy(input);
    return if status < 0i32 {
        -1i32
    } else {
        CMap_is_valid(cmap) as libc::c_int
    };
}
