#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
use libc::free;
extern "C" {
    pub type pst_obj;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const i8) -> i32;
    #[no_mangle]
    fn CMap_cache_get(id: i32) -> *mut CMap;
    #[no_mangle]
    fn CMap_add_codespacerange(
        cmap: *mut CMap,
        codelo: *const u8,
        codehi: *const u8,
        dim: size_t,
    ) -> i32;
    #[no_mangle]
    fn CMap_add_notdefrange(
        cmap: *mut CMap,
        srclo: *const u8,
        srchi: *const u8,
        srcdim: size_t,
        dst: CID,
    ) -> i32;
    #[no_mangle]
    fn CMap_add_notdefchar(cmap: *mut CMap, src: *const u8, srcdim: size_t, dst: CID) -> i32;
    #[no_mangle]
    fn CMap_add_cidrange(
        cmap: *mut CMap,
        srclo: *const u8,
        hi: *const u8,
        srcdim: size_t,
        base: CID,
    ) -> i32;
    #[no_mangle]
    fn CMap_add_bfrange(
        cmap: *mut CMap,
        srclo: *const u8,
        srchi: *const u8,
        srcdim: size_t,
        dest: *const u8,
        destdim: size_t,
    ) -> i32;
    #[no_mangle]
    fn CMap_add_cidchar(cmap: *mut CMap, src: *const u8, srcdim: size_t, dest: CID) -> i32;
    #[no_mangle]
    fn CMap_add_bfchar(
        cmap: *mut CMap,
        src: *const u8,
        srcdim: size_t,
        dest: *const u8,
        destdim: size_t,
    ) -> i32;
    #[no_mangle]
    fn CMap_set_CIDSysInfo(cmap: *mut CMap, csi: *const CIDSysInfo);
    #[no_mangle]
    fn CMap_set_usecmap(cmap: *mut CMap, ucmap: *mut CMap);
    #[no_mangle]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: i32);
    #[no_mangle]
    fn CMap_set_type(cmap: *mut CMap, type_0: i32);
    #[no_mangle]
    fn CMap_set_name(cmap: *mut CMap, name: *const i8);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn CMap_is_valid(cmap: *mut CMap) -> bool;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
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
    fn pst_get_token(inbuf: *mut *mut u8, inbufend: *mut u8) -> *mut pst_obj;
    #[no_mangle]
    fn pst_release_obj(obj: *mut pst_obj);
    #[no_mangle]
    fn pst_type_of(obj: *mut pst_obj) -> pst_type;
    #[no_mangle]
    fn pst_length_of(obj: *mut pst_obj) -> i32;
    #[no_mangle]
    fn pst_getIV(obj: *mut pst_obj) -> i32;
    #[no_mangle]
    fn pst_getSV(obj: *mut pst_obj) -> *mut u8;
    #[no_mangle]
    fn pst_data_ptr(obj: *mut pst_obj) -> *mut libc::c_void;
}
pub type __ssize_t = i64;
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
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: i32,
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
    pub codeLo: *mut u8,
    pub codeHi: *mut u8,
    /* Upper bounds of valid input code */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: i32,
    pub len: size_t,
    pub code: *mut u8,
    pub next: *mut mapDef,
    /* Next Subtbl for LOOKUP_CONTINUE */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: i32,
    /* Position of next free data segment */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut i8,
    pub type_0: i32,
    pub wmode: i32,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: i32,
    pub profile: C2RustUnnamed,
    pub reverseMap: *mut i32,
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
    pub num: u32,
    pub max: u32,
    pub ranges: *mut rangeDef,
}
pub type CID = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreader {
    pub cursor: *mut u8,
    pub endptr: *mut u8,
    pub buf: *mut u8,
    pub max: size_t,
    pub handle: rust_input_handle_t,
    pub unread: size_t,
}
pub type pst_type = i32;
#[inline]
unsafe extern "C" fn strstartswith(mut s: *const i8, mut prefix: *const i8) -> *const i8 {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    0 as *const i8
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
static mut __verbose: i32 = 0i32;
unsafe extern "C" fn ifreader_create(
    mut handle: rust_input_handle_t,
    mut size: size_t,
    mut bufsize: size_t,
) -> *mut ifreader {
    let mut reader: *mut ifreader = 0 as *mut ifreader;
    reader =
        new((1_u64).wrapping_mul(::std::mem::size_of::<ifreader>() as u64) as u32) as *mut ifreader;
    (*reader).buf = new((bufsize.wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    (*reader).max = bufsize;
    (*reader).handle = handle;
    (*reader).unread = size;
    (*reader).endptr = (*reader).buf;
    (*reader).cursor = (*reader).endptr;
    *(*reader).endptr = 0_u8;
    reader
}
unsafe extern "C" fn ifreader_destroy(mut reader: *mut ifreader) {
    assert!(!reader.is_null());
    free((*reader).buf as *mut libc::c_void);
    free(reader as *mut libc::c_void);
}
unsafe extern "C" fn ifreader_read(mut reader: *mut ifreader, mut size: size_t) -> size_t {
    let mut bytesread: size_t = 0i32 as size_t;
    let mut bytesrem: size_t = 0i32 as size_t;
    assert!(!reader.is_null());
    bytesrem = ((*reader).endptr as size_t).wrapping_sub((*reader).cursor as size_t);
    if size > (*reader).max {
        if __verbose != 0 {
            dpx_message(
                b"\nExtending buffer (%zu bytes)...\n\x00" as *const u8 as *const i8,
                size,
            );
        }
        (*reader).buf = renew(
            (*reader).buf as *mut libc::c_void,
            (size.wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
        ) as *mut u8;
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
        if ttstub_input_read((*reader).handle, (*reader).endptr as *mut i8, bytesread) as u64
            != bytesread
        {
            _tt_abort(b"Reading file failed.\x00" as *const u8 as *const i8);
        }
        (*reader).endptr = (*reader).endptr.offset(bytesread as isize);
        (*reader).unread = ((*reader).unread as u64).wrapping_sub(bytesread) as size_t as size_t;
        if __verbose != 0 {
            dpx_message(
                b"Reading more %zu bytes (%zu bytes remains in buffer)...\n\x00" as *const u8
                    as *const i8,
                bytesread,
                bytesrem,
            );
        }
    }
    *(*reader).endptr = 0_u8;
    bytesread.wrapping_add(bytesrem)
}
unsafe extern "C" fn check_next_token(mut input: *mut ifreader, mut key: *const i8) -> i32 {
    let mut cmp: i32 = 0;
    let mut token: *mut pst_obj = 0 as *mut pst_obj;
    let mut str: *mut i8 = 0 as *mut i8;
    if ifreader_read(input, strlen(key)) == 0i32 as u64 {
        return -1i32;
    }
    token = pst_get_token(&mut (*input).cursor, (*input).endptr);
    if token.is_null() {
        return -1i32;
    }
    str = pst_getSV(token) as *mut i8;
    cmp = if strcmp(str, key) != 0 { -1i32 } else { 0i32 };
    free(str as *mut libc::c_void);
    pst_release_obj(token);
    cmp
}
unsafe extern "C" fn get_coderange(
    mut input: *mut ifreader,
    mut codeLo: *mut u8,
    mut codeHi: *mut u8,
    mut dim: *mut i32,
    mut maxlen: i32,
) -> i32 {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut dim1: i32 = 0;
    let mut dim2: i32 = 0;
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
    memcpy(codeLo as *mut libc::c_void, pst_data_ptr(tok1), dim1 as u64);
    memcpy(codeHi as *mut libc::c_void, pst_data_ptr(tok2), dim2 as u64);
    pst_release_obj(tok1);
    pst_release_obj(tok2);
    *dim = dim1;
    0i32
}
unsafe extern "C" fn do_codespacerange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
    let mut codeLo: [u8; 127] = [0; 127];
    let mut codeHi: [u8; 127] = [0; 127];
    let mut dim: i32 = 0;
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
    check_next_token(input, b"endcodespacerange\x00" as *const u8 as *const i8)
}
/*
 * bfrange
 *  <codeLo> <codeHi> [destCode1 destCode2 ...]
 */
unsafe extern "C" fn handle_codearray(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut codeLo: *mut u8,
    mut dim: i32,
    mut count: i32,
) -> i32 {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    if dim < 1i32 {
        _tt_abort(b"Invalid code range.\x00" as *const u8 as *const i8);
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
                    pst_data_ptr(tok) as *mut u8,
                    pst_length_of(tok) as size_t,
                );
            } else if pst_type_of(tok) == 7i32 || !(pst_type_of(tok) == 6i32) {
                _tt_abort(
                    b"%s: Invalid CMap mapping record.\x00" as *const u8 as *const i8,
                    b"CMap_parse:\x00" as *const u8 as *const i8,
                );
            } else {
                _tt_abort(
                    b"%s: Mapping to charName not supported.\x00" as *const u8 as *const i8,
                    b"CMap_parse:\x00" as *const u8 as *const i8,
                );
            }
        }
        pst_release_obj(tok);
        let ref mut fresh2 = *codeLo.offset((dim - 1i32) as isize);
        *fresh2 = (*fresh2 as i32 + 1i32) as u8
    }
    check_next_token(input, b"]\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_notdefrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut codeLo: [u8; 127] = [0; 127];
    let mut codeHi: [u8; 127] = [0; 127];
    let mut dstCID: i32 = 0;
    let mut dim: i32 = 0;
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
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        }
        pst_release_obj(tok);
    }
    check_next_token(input, b"endnotdefrange\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_bfrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut codeLo: [u8; 127] = [0; 127];
    let mut codeHi: [u8; 127] = [0; 127];
    let mut srcdim: i32 = 0;
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
                pst_data_ptr(tok) as *mut u8,
                pst_length_of(tok) as size_t,
            );
        } else if pst_type_of(tok) == 7i32 {
            if handle_codearray(
                cmap,
                input,
                codeLo.as_mut_ptr(),
                srcdim,
                codeHi[(srcdim - 1i32) as usize] as i32 - codeLo[(srcdim - 1i32) as usize] as i32
                    + 1i32,
            ) < 0i32
            {
                pst_release_obj(tok);
                return -1i32;
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        }
        pst_release_obj(tok);
    }
    check_next_token(input, b"endbfrange\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_cidrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
    let mut tok: *mut pst_obj = 0 as *mut pst_obj;
    let mut codeLo: [u8; 127] = [0; 127];
    let mut codeHi: [u8; 127] = [0; 127];
    let mut dstCID: i32 = 0;
    let mut dim: i32 = 0;
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
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        }
        pst_release_obj(tok);
    }
    check_next_token(input, b"endcidrange\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_notdefchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut dstCID: i32 = 0;
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
                    pst_data_ptr(tok1) as *const u8,
                    pst_length_of(tok1) as size_t,
                    dstCID as CID,
                );
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    check_next_token(input, b"endnotdefchar\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_bfchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
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
                pst_data_ptr(tok1) as *mut u8,
                pst_length_of(tok1) as size_t,
                pst_data_ptr(tok2) as *mut u8,
                pst_length_of(tok2) as size_t,
            );
        } else if pst_type_of(tok2) == 6i32 {
            _tt_abort(
                b"%s: Mapping to charName not supported.\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    check_next_token(input, b"endbfchar\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_cidchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: i32,
) -> i32 {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut dstCID: i32 = 0;
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
                    pst_data_ptr(tok1) as *const u8,
                    pst_length_of(tok1) as size_t,
                    dstCID as CID,
                );
            }
        } else {
            dpx_warning(
                b"%s: Invalid CMap mapping record. (ignored)\x00" as *const u8 as *const i8,
                b"CMap_parse:\x00" as *const u8 as *const i8,
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    check_next_token(input, b"endcidchar\x00" as *const u8 as *const i8)
}
unsafe extern "C" fn do_cidsysteminfo(mut cmap: *mut CMap, mut input: *mut ifreader) -> i32 {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj;
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut csi: CIDSysInfo = {
        let mut init = CIDSysInfo {
            registry: 0 as *mut i8,
            ordering: 0 as *mut i8,
            supplement: -1i32,
        };
        init
    };
    let mut simpledict: i32 = 0i32;
    let mut error: i32 = 0i32;
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
                b"begin\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"begin\x00" as *const u8 as *const i8),
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
                b">>\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b">>\x00" as *const u8 as *const i8),
            ) == 0
            && simpledict != 0
        {
            pst_release_obj(tok1);
            break;
        } else if pst_type_of(tok1) < 0i32
            && memcmp(
                pst_data_ptr(tok1),
                b"end\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"end\x00" as *const u8 as *const i8),
            ) == 0
            && simpledict == 0
        {
            pst_release_obj(tok1);
            break;
        } else {
            if pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Registry\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"Registry\x00" as *const u8 as *const i8),
                ) == 0
                && {
                    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == 5i32) {
                    error = -1i32
                } else if simpledict == 0
                    && check_next_token(input, b"def\x00" as *const u8 as *const i8) != 0
                {
                    error = -1i32
                }
                if error == 0 {
                    csi.registry = pst_getSV(tok2) as *mut i8
                }
            } else if pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Ordering\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"Ordering\x00" as *const u8 as *const i8),
                ) == 0
                && {
                    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == 5i32) {
                    error = -1i32
                } else if simpledict == 0
                    && check_next_token(input, b"def\x00" as *const u8 as *const i8) != 0
                {
                    error = -1i32
                }
                if error == 0 {
                    csi.ordering = pst_getSV(tok2) as *mut i8
                }
            } else if pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Supplement\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"Supplement\x00" as *const u8 as *const i8),
                ) == 0
                && {
                    tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == 2i32) {
                    error = -1i32
                } else if simpledict == 0
                    && check_next_token(input, b"def\x00" as *const u8 as *const i8) != 0
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
    if error == 0 && check_next_token(input, b"def\x00" as *const u8 as *const i8) != 0 {
        error = -1i32
    }
    if error == 0 && !csi.registry.is_null() && !csi.ordering.is_null() && csi.supplement >= 0i32 {
        CMap_set_CIDSysInfo(cmap, &mut csi);
    }
    free(csi.registry as *mut libc::c_void);
    free(csi.ordering as *mut libc::c_void);
    error
}
#[no_mangle]
pub unsafe extern "C" fn CMap_parse_check_sig(mut handle: rust_input_handle_t) -> i32 {
    let mut result: i32 = -1i32;
    let mut sig: [i8; 65] = [0; 65];
    if handle.is_null() {
        return -1i32;
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(handle, sig.as_mut_ptr(), 64i32 as size_t) != 64i32 as i64 {
        result = -1i32
    } else {
        sig[64] = 0_i8;
        if strstartswith(sig.as_mut_ptr(), b"%!PS\x00" as *const u8 as *const i8).is_null() {
            result = -1i32
        } else if !strstr(
            sig.as_mut_ptr().offset(4),
            b"Resource-CMap\x00" as *const u8 as *const i8,
        )
        .is_null()
        {
            result = 0i32
        }
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    result
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
pub unsafe extern "C" fn CMap_parse(mut cmap: *mut CMap, mut handle: rust_input_handle_t) -> i32 {
    let mut tok1: *mut pst_obj = 0 as *mut pst_obj; /* else Simply ignore */
    let mut tok2: *mut pst_obj = 0 as *mut pst_obj;
    let mut input: *mut ifreader = 0 as *mut ifreader;
    let mut status: i32 = 0i32;
    let mut tmpint: i32 = -1i32;
    assert!(!cmap.is_null() && !handle.is_null());
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
                b"CMapName\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"CMapName\x00" as *const u8 as *const i8),
            ) == 0
        {
            tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == 6i32 || pst_type_of(tok2) == 5i32)
                || check_next_token(input, b"def\x00" as *const u8 as *const i8) < 0i32
            {
                status = -1i32
            } else {
                CMap_set_name(cmap, pst_data_ptr(tok2) as *const i8);
            }
        } else if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"CMapType\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"CMapType\x00" as *const u8 as *const i8),
            ) == 0
        {
            tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == 2i32)
                || check_next_token(input, b"def\x00" as *const u8 as *const i8) < 0i32
            {
                status = -1i32
            } else {
                CMap_set_type(cmap, pst_getIV(tok2));
            }
        } else if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"WMode\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"WMode\x00" as *const u8 as *const i8),
            ) == 0
        {
            tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == 2i32)
                || check_next_token(input, b"def\x00" as *const u8 as *const i8) < 0i32
            {
                status = -1i32
            } else {
                CMap_set_wmode(cmap, pst_getIV(tok2));
            }
        } else if pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"CIDSystemInfo\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"CIDSystemInfo\x00" as *const u8 as *const i8),
            ) == 0
        {
            status = do_cidsysteminfo(cmap, input)
        } else if !(pst_type_of(tok1) == 6i32
            && memcmp(
                pst_data_ptr(tok1),
                b"Version\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"Version\x00" as *const u8 as *const i8),
            ) == 0
            || pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"UIDOffset\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"UIDOffset\x00" as *const u8 as *const i8),
                ) == 0
            || pst_type_of(tok1) == 6i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"XUID\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"XUID\x00" as *const u8 as *const i8),
                ) == 0)
        {
            if pst_type_of(tok1) == 6i32 {
                /* Possibly usecmap comes next */
                tok2 = pst_get_token(&mut (*input).cursor, (*input).endptr);
                if !tok2.is_null()
                    && (pst_type_of(tok2) < 0i32
                        && memcmp(
                            pst_data_ptr(tok2),
                            b"usecmap\x00" as *const u8 as *const i8 as *const libc::c_void,
                            strlen(b"usecmap\x00" as *const u8 as *const i8),
                        ) == 0)
                {
                    let mut id: i32 = 0;
                    let mut ucmap: *mut CMap = 0 as *mut CMap;
                    id = CMap_cache_find(pst_data_ptr(tok1) as *const i8);
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
                    b"begincodespacerange\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"begincodespacerange\x00" as *const u8 as *const i8),
                ) == 0
            {
                status = do_codespacerange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginnotdefrange\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"beginnotdefrange\x00" as *const u8 as *const i8),
                ) == 0
            {
                status = do_notdefrange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginnotdefchar\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"beginnotdefchar\x00" as *const u8 as *const i8),
                ) == 0
            {
                status = do_notdefchar(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginbfrange\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"beginbfrange\x00" as *const u8 as *const i8),
                ) == 0
            {
                status = do_bfrange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginbfchar\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"beginbfchar\x00" as *const u8 as *const i8),
                ) == 0
            {
                status = do_bfchar(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincidrange\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"begincidrange\x00" as *const u8 as *const i8),
                ) == 0
            {
                status = do_cidrange(cmap, input, tmpint)
            } else if pst_type_of(tok1) < 0i32
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincidchar\x00" as *const u8 as *const i8 as *const libc::c_void,
                    strlen(b"begincidchar\x00" as *const u8 as *const i8),
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
    if status < 0i32 {
        -1i32
    } else {
        CMap_is_valid(cmap) as i32
    }
}
