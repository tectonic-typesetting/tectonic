#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
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
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    static mut CSI_IDENTITY: CIDSysInfo;
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
    fn CMap_parse_check_sig(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn CMap_parse(cmap: *mut CMap, handle: rust_input_handle_t) -> i32;
}
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
pub struct CIDSysInfo {
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut u8,
    pub codeHi: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: i32,
    pub len: size_t,
    pub code: *mut u8,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: i32,
}
/* quasi-hack to get the primary input */
/* CID, Code... MEM_ALLOC_SIZE bytes  */
/* Previous mapData data segment      */
/* Position of next free data segment */
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
pub struct CMap_cache {
    pub num: i32,
    pub max: i32,
    pub cmaps: *mut *mut CMap,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
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
    return 0i32 != 0;
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
 * TODO:
 *   Only cid(range|char) allowed for CODE_TO_CID and bf(range|char) for CID_TO_CODE ?
 */
static mut __verbose: i32 = 0i32;
static mut __silent: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CMap_set_verbose(mut level: i32) {
    __verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_silent(mut value: i32) {
    __silent = if value != 0 { 1i32 } else { 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn CMap_new() -> *mut CMap {
    let mut cmap: *mut CMap = 0 as *mut CMap;
    cmap = new((1_u64).wrapping_mul(::std::mem::size_of::<CMap>() as u64) as u32) as *mut CMap;
    (*cmap).name = 0 as *mut i8;
    (*cmap).type_0 = 1i32;
    (*cmap).wmode = 0i32;
    (*cmap).useCMap = 0 as *mut CMap;
    (*cmap).CSI = 0 as *mut CIDSysInfo;
    (*cmap).profile.minBytesIn = 2i32 as size_t;
    (*cmap).profile.maxBytesIn = 2i32 as size_t;
    (*cmap).profile.minBytesOut = 2i32 as size_t;
    (*cmap).profile.maxBytesOut = 2i32 as size_t;
    (*cmap).flags = 0i32;
    (*cmap).codespace.num = 0_u32;
    (*cmap).codespace.max = 10_u32;
    (*cmap).codespace.ranges =
        new((10_u64).wrapping_mul(::std::mem::size_of::<rangeDef>() as u64) as u32)
            as *mut rangeDef;
    (*cmap).mapTbl = 0 as *mut mapDef;
    (*cmap).mapData =
        new((1_u64).wrapping_mul(::std::mem::size_of::<mapData>() as u64) as u32) as *mut mapData;
    (*(*cmap).mapData).prev = 0 as *mut mapData;
    (*(*cmap).mapData).pos = 0i32;
    (*(*cmap).mapData).data =
        new((4096_u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    (*cmap).reverseMap =
        new((65536_u64).wrapping_mul(::std::mem::size_of::<i32>() as u64) as u32) as *mut i32;
    memset(
        (*cmap).reverseMap as *mut libc::c_void,
        0i32,
        (65536i32 as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64),
    );
    return cmap;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_release(mut cmap: *mut CMap) {
    if cmap.is_null() {
        return;
    }
    free((*cmap).name as *mut libc::c_void);
    if !(*cmap).CSI.is_null() {
        free((*(*cmap).CSI).registry as *mut libc::c_void);
        free((*(*cmap).CSI).ordering as *mut libc::c_void);
        free((*cmap).CSI as *mut libc::c_void);
    }
    free((*cmap).codespace.ranges as *mut libc::c_void);
    if !(*cmap).mapTbl.is_null() {
        mapDef_release((*cmap).mapTbl);
    }
    let mut map: *mut mapData = (*cmap).mapData;
    while !map.is_null() {
        let mut prev: *mut mapData = (*map).prev;
        free((*map).data as *mut libc::c_void);
        free(map as *mut libc::c_void);
        map = prev
    }
    free((*cmap).reverseMap as *mut libc::c_void);
    free(cmap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_is_Identity(mut cmap: *mut CMap) -> bool {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            149_u32,
            (*::std::mem::transmute::<&[u8; 31], &[i8; 31]>(b"_Bool CMap_is_Identity(CMap *)\x00"))
                .as_ptr(),
        );
    }
    return streq_ptr((*cmap).name, b"Identity-H\x00" as *const u8 as *const i8) as i32 != 0
        || streq_ptr((*cmap).name, b"Identity-V\x00" as *const u8 as *const i8) as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_is_valid(mut cmap: *mut CMap) -> bool {
    /* Quick check */
    if cmap.is_null()
        || (*cmap).name.is_null()
        || (*cmap).type_0 < 0i32
        || (*cmap).type_0 > 3i32
        || (*cmap).codespace.num < 1_u32
        || (*cmap).type_0 != 0i32 && (*cmap).mapTbl.is_null()
    {
        return 0i32 != 0;
    }
    if !(*cmap).useCMap.is_null() {
        let mut csi1: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
        let mut csi2: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
        csi1 = CMap_get_CIDSysInfo(cmap);
        csi2 = CMap_get_CIDSysInfo((*cmap).useCMap);
        if strcmp((*csi1).registry, (*csi2).registry) != 0
            || strcmp((*csi1).ordering, (*csi2).ordering) != 0
        {
            dpx_warning(
                b"CIDSystemInfo mismatched %s <--> %s\x00" as *const u8 as *const i8,
                CMap_get_name(cmap),
                CMap_get_name((*cmap).useCMap),
            );
            return 0i32 != 0;
        }
    }
    return 1i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_profile(mut cmap: *mut CMap, mut type_0: i32) -> i32 {
    let mut value: i32 = 0i32;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            184_u32,
            (*::std::mem::transmute::<&[u8; 34], &[i8; 34]>(
                b"int CMap_get_profile(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    match type_0 {
        0 => value = (*cmap).profile.minBytesIn as i32,
        1 => value = (*cmap).profile.maxBytesIn as i32,
        2 => value = (*cmap).profile.maxBytesOut as i32,
        3 => value = (*cmap).profile.maxBytesOut as i32,
        _ => {
            _tt_abort(
                b"%s: Unrecognized profile type %d.\x00" as *const u8 as *const i8,
                b"CMap\x00" as *const u8 as *const i8,
                type_0,
            );
        }
    }
    return value;
}
/*
 * Put notdef chars for codes not declared in notdef(range|char)
 */
unsafe extern "C" fn handle_undefined(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const u8,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut u8,
    mut outbytesleft: *mut size_t,
) {
    let mut len: size_t = 0i32 as size_t;
    if *outbytesleft < 2i32 as u64 {
        _tt_abort(
            b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    }
    match (*cmap).type_0 {
        1 => {
            memcpy(
                *outbuf as *mut libc::c_void,
                b"\x00\x00\x00" as *const u8 as *const i8 as *const libc::c_void,
                2i32 as u64,
            );
        }
        2 => {
            memcpy(
                *outbuf as *mut libc::c_void,
                b"\xff\xfd\x00" as *const u8 as *const i8 as *const libc::c_void,
                2i32 as u64,
            );
        }
        _ => {
            dpx_warning(
                b"Cannot handle undefined mapping for this type of CMap mapping: %d\x00"
                    as *const u8 as *const i8,
                (*cmap).type_0,
            );
            dpx_warning(b"<0000> is used for .notdef char.\x00" as *const u8 as *const i8);
            memset(*outbuf as *mut libc::c_void, 0i32, 2i32 as u64);
        }
    }
    *outbuf = (*outbuf).offset(2);
    *outbytesleft = (*outbytesleft as u64).wrapping_sub(2i32 as u64) as size_t as size_t;
    len = bytes_consumed(cmap, *inbuf, *inbytesleft);
    *inbuf = (*inbuf).offset(len as isize);
    *inbytesleft = (*inbytesleft as u64).wrapping_sub(len) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_decode_char(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const u8,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut u8,
    mut outbytesleft: *mut size_t,
) {
    let mut t: *mut mapDef = 0 as *mut mapDef;
    let mut p: *const u8 = 0 as *const u8;
    let mut save: *const u8 = 0 as *const u8;
    let mut c: u8 = 0_u8;
    let mut count: size_t = 0i32 as size_t;
    save = *inbuf;
    p = save;
    /*
     * First handle some special cases:
     */
    if (*cmap).type_0 == 0i32 {
        if (*inbytesleft).wrapping_rem(2i32 as u64) != 0 {
            _tt_abort(
                b"%s: Invalid/truncated input string.\x00" as *const u8 as *const i8,
                b"CMap\x00" as *const u8 as *const i8,
            );
        }
        if *outbytesleft < 2i32 as u64 {
            _tt_abort(
                b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                b"CMap\x00" as *const u8 as *const i8,
            );
        }
        memcpy(
            *outbuf as *mut libc::c_void,
            *inbuf as *const libc::c_void,
            2i32 as u64,
        );
        *inbuf = (*inbuf).offset(2);
        *outbuf = (*outbuf).offset(2);
        *outbytesleft = (*outbytesleft as u64).wrapping_sub(2i32 as u64) as size_t as size_t;
        *inbytesleft = (*inbytesleft as u64).wrapping_sub(2i32 as u64) as size_t as size_t;
        return;
    } else {
        if (*cmap).mapTbl.is_null() {
            if !(*cmap).useCMap.is_null() {
                CMap_decode_char((*cmap).useCMap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            } else {
                /* no mapping available in this CMap */
                dpx_warning(
                    b"No mapping available for this character.\x00" as *const u8 as *const i8,
                );
                handle_undefined(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            }
        }
    }
    if !(*cmap).mapTbl.is_null() {
    } else {
        __assert_fail(b"cmap->mapTbl\x00" as *const u8 as *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      276_u32,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[i8; 92]>(b"void CMap_decode_char(CMap *, const unsigned char **, size_t *, unsigned char **, size_t *)\x00")).as_ptr());
    }
    t = (*cmap).mapTbl;
    while count < *inbytesleft {
        let fresh0 = p;
        p = p.offset(1);
        c = *fresh0;
        count = count.wrapping_add(1);
        if (*t.offset(c as isize)).flag & 1i32 << 4i32 == 0 {
            break;
        }
        t = (*t.offset(c as isize)).next
    }
    if (*t.offset(c as isize)).flag & 1i32 << 4i32 != 0 {
        /* need more bytes */
        _tt_abort(
            b"%s: Premature end of input string.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    } else {
        if if (*t.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } == 0
        {
            if !(*cmap).useCMap.is_null() {
                CMap_decode_char((*cmap).useCMap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            } else {
                /* no mapping available in this CMap */
                dpx_warning(b"No character mapping available.\x00" as *const u8 as *const i8);
                dpx_message(
                    b" CMap name: %s\n\x00" as *const u8 as *const i8,
                    CMap_get_name(cmap),
                );
                dpx_message(b" input str: \x00" as *const u8 as *const i8);
                dpx_message(b"<\x00" as *const u8 as *const i8);
                while save < p {
                    dpx_message(b"%02x\x00" as *const u8 as *const i8, *save as i32);
                    save = save.offset(1)
                }
                dpx_message(b">\n\x00" as *const u8 as *const i8);
                /*
                 * We know partial match found up to `count' bytes,
                 * but we will not use this information for the sake of simplicity.
                 */
                handle_undefined(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
                return;
            }
        } else {
            match (*t.offset(c as isize)).flag & 0xfi32 {
                8 => {
                    dpx_warning(
                        b"Character mapped to .notdef found.\x00" as *const u8 as *const i8,
                    );
                }
                1 | 4 => {}
                2 => {
                    _tt_abort(
                        b"%s: CharName mapping not supported.\x00" as *const u8 as *const i8,
                        b"CMap\x00" as *const u8 as *const i8,
                    );
                }
                _ => {
                    _tt_abort(
                        b"%s: Unknown mapping type.\x00" as *const u8 as *const i8,
                        b"CMap\x00" as *const u8 as *const i8,
                    );
                }
            }
            /* continue */
            if *outbytesleft >= (*t.offset(c as isize)).len {
                memcpy(
                    *outbuf as *mut libc::c_void,
                    (*t.offset(c as isize)).code as *const libc::c_void,
                    (*t.offset(c as isize)).len,
                );
            } else {
                _tt_abort(
                    b"%s: Buffer overflow.\x00" as *const u8 as *const i8,
                    b"CMap\x00" as *const u8 as *const i8,
                );
            }
            *outbuf = (*outbuf).offset((*t.offset(c as isize)).len as isize);
            *outbytesleft = (*outbytesleft as u64).wrapping_sub((*t.offset(c as isize)).len)
                as size_t as size_t;
            if !inbytesleft.is_null() {
                *inbytesleft = (*inbytesleft as u64).wrapping_sub(count) as size_t as size_t
            }
            *inbuf = p
        }
    };
}
/*
 * For convenience, it does not do decoding to CIDs.
 */
#[no_mangle]
pub unsafe extern "C" fn CMap_decode(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const u8,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut u8,
    mut outbytesleft: *mut size_t,
) -> size_t {
    let mut count: size_t = 0;
    if !cmap.is_null() && !inbuf.is_null() && !outbuf.is_null() {
    } else {
        __assert_fail(b"cmap && inbuf && outbuf\x00" as *const u8 as
                          *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      344_u32,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[i8; 89]>(b"size_t CMap_decode(CMap *, const unsigned char **, size_t *, unsigned char **, size_t *)\x00")).as_ptr());
    }
    if !inbytesleft.is_null() && !outbytesleft.is_null() {
    } else {
        __assert_fail(b"inbytesleft && outbytesleft\x00" as *const u8 as
                          *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      345_u32,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[i8; 89]>(b"size_t CMap_decode(CMap *, const unsigned char **, size_t *, unsigned char **, size_t *)\x00")).as_ptr());
    }
    count = 0i32 as size_t;
    while *inbytesleft > 0i32 as u64 && *outbytesleft > 0i32 as u64 {
        CMap_decode_char(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
        count = count.wrapping_add(1)
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_reverse_decode(mut cmap: *mut CMap, mut cid: CID) -> i32 {
    let mut ch: i32 = if !(*cmap).reverseMap.is_null() {
        *(*cmap).reverseMap.offset(cid as isize)
    } else {
        -1i32
    };
    if ch == 0i32 && !(*cmap).useCMap.is_null() {
        return CMap_reverse_decode((*cmap).useCMap, cid);
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_name(mut cmap: *mut CMap) -> *mut i8 {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            363_u32,
            (*::std::mem::transmute::<&[u8; 28], &[i8; 28]>(b"char *CMap_get_name(CMap *)\x00"))
                .as_ptr(),
        );
    }
    return (*cmap).name;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_type(mut cmap: *mut CMap) -> i32 {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            370_u32,
            (*::std::mem::transmute::<&[u8; 26], &[i8; 26]>(b"int CMap_get_type(CMap *)\x00"))
                .as_ptr(),
        );
    }
    return (*cmap).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_wmode(mut cmap: *mut CMap) -> i32 {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            377_u32,
            (*::std::mem::transmute::<&[u8; 27], &[i8; 27]>(b"int CMap_get_wmode(CMap *)\x00"))
                .as_ptr(),
        );
    }
    return (*cmap).wmode;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_get_CIDSysInfo(mut cmap: *mut CMap) -> *mut CIDSysInfo {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            384_u32,
            (*::std::mem::transmute::<&[u8; 40], &[i8; 40]>(
                b"CIDSysInfo *CMap_get_CIDSysInfo(CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*cmap).CSI;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_name(mut cmap: *mut CMap, mut name: *const i8) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            391_u32,
            (*::std::mem::transmute::<&[u8; 41], &[i8; 41]>(
                b"void CMap_set_name(CMap *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    free((*cmap).name as *mut libc::c_void);
    (*cmap).name = new((strlen(name).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*cmap).name, name);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_type(mut cmap: *mut CMap, mut type_0: i32) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            400_u32,
            (*::std::mem::transmute::<&[u8; 32], &[i8; 32]>(
                b"void CMap_set_type(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    (*cmap).type_0 = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_wmode(mut cmap: *mut CMap, mut wmode: i32) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            407_u32,
            (*::std::mem::transmute::<&[u8; 33], &[i8; 33]>(
                b"void CMap_set_wmode(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    (*cmap).wmode = wmode;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_set_CIDSysInfo(mut cmap: *mut CMap, mut csi: *const CIDSysInfo) {
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            414_u32,
            (*::std::mem::transmute::<&[u8; 53], &[i8; 53]>(
                b"void CMap_set_CIDSysInfo(CMap *, const CIDSysInfo *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*cmap).CSI.is_null() {
        free((*(*cmap).CSI).registry as *mut libc::c_void);
        free((*(*cmap).CSI).ordering as *mut libc::c_void);
        free((*cmap).CSI as *mut libc::c_void);
    }
    if !csi.is_null() && !(*csi).registry.is_null() && !(*csi).ordering.is_null() {
        (*cmap).CSI = new((1_u64).wrapping_mul(::std::mem::size_of::<CIDSysInfo>() as u64) as u32)
            as *mut CIDSysInfo;
        (*(*cmap).CSI).registry = new((strlen((*csi).registry).wrapping_add(1i32 as u64) as u32
            as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy((*(*cmap).CSI).registry, (*csi).registry);
        (*(*cmap).CSI).ordering = new((strlen((*csi).ordering).wrapping_add(1i32 as u64) as u32
            as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        strcpy((*(*cmap).CSI).ordering, (*csi).ordering);
        (*(*cmap).CSI).supplement = (*csi).supplement
    } else {
        dpx_warning(b"Invalid CIDSystemInfo.\x00" as *const u8 as *const i8);
        (*cmap).CSI = 0 as *mut CIDSysInfo
    };
}
/*
 * Can have muliple entry ?
 */
#[no_mangle]
pub unsafe extern "C" fn CMap_set_usecmap(mut cmap: *mut CMap, mut ucmap: *mut CMap) {
    let mut i: u32 = 0; /* Maybe if (!ucmap) _tt_abort() is better for this. */
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            443_u32,
            (*::std::mem::transmute::<&[u8; 38], &[i8; 38]>(
                b"void CMap_set_usecmap(CMap *, CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !ucmap.is_null() {
    } else {
        __assert_fail(
            b"ucmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            444_u32,
            (*::std::mem::transmute::<&[u8; 38], &[i8; 38]>(
                b"void CMap_set_usecmap(CMap *, CMap *)\x00",
            ))
            .as_ptr(),
        );
    }
    if cmap == ucmap {
        _tt_abort(
            b"%s: Identical CMap object cannot be used for usecmap CMap: 0x%p=0x%p\x00" as *const u8
                as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
            cmap,
            ucmap,
        );
    }
    /* Check if ucmap have neccesary information. */
    if !CMap_is_valid(ucmap) {
        _tt_abort(
            b"%s: Invalid CMap.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    }
    /*
     *  CMapName of cmap can be undefined when usecmap is executed in CMap parsing.
     *  And it is also possible CSI is not defined at that time.
     */
    if streq_ptr((*cmap).name, (*ucmap).name) {
        _tt_abort(
            b"%s: CMap refering itself not allowed: CMap %s --> %s\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
            (*cmap).name,
            (*ucmap).name,
        );
    }
    if !(*cmap).CSI.is_null()
        && !(*(*cmap).CSI).registry.is_null()
        && !(*(*cmap).CSI).ordering.is_null()
    {
        if strcmp((*(*cmap).CSI).registry, (*(*ucmap).CSI).registry) != 0
            || strcmp((*(*cmap).CSI).ordering, (*(*ucmap).CSI).ordering) != 0
        {
            _tt_abort(
                b"%s: CMap %s required by %s have different CSI.\x00" as *const u8 as *const i8,
                b"CMap\x00" as *const u8 as *const i8,
                CMap_get_name(cmap),
                CMap_get_name(ucmap),
            );
        }
    }
    /* We must copy codespaceranges. */
    i = 0_u32;
    while i < (*ucmap).codespace.num {
        let mut csr: *mut rangeDef = (*ucmap).codespace.ranges.offset(i as isize);
        CMap_add_codespacerange(cmap, (*csr).codeLo, (*csr).codeHi, (*csr).dim);
        i = i.wrapping_add(1)
    }
    (*cmap).useCMap = ucmap;
}
/* Test the validity of character c. */
unsafe extern "C" fn CMap_match_codespace(
    mut cmap: *mut CMap,
    mut c: *const u8,
    mut dim: size_t,
) -> i32 {
    let mut i: u32 = 0;
    let mut pos: u32 = 0;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            484_u32,
            (*::std::mem::transmute::<&[u8; 64], &[i8; 64]>(
                b"int CMap_match_codespace(CMap *, const unsigned char *, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0_u32;
    while i < (*cmap).codespace.num {
        let mut csr: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
        if !((*csr).dim != dim) {
            pos = 0_u32;
            while (pos as u64) < dim {
                if *c.offset(pos as isize) as i32 > *(*csr).codeHi.offset(pos as isize) as i32
                    || (*c.offset(pos as isize) as i32) < *(*csr).codeLo.offset(pos as isize) as i32
                {
                    break;
                }
                pos = pos.wrapping_add(1)
            }
            if pos as u64 == dim {
                return 0i32;
            }
        }
        i = i.wrapping_add(1)
        /* Valid */
    }
    return -1i32;
    /* Invalid */
}
/*
 * No overlapping codespace ranges are allowed, otherwise mapping is ambiguous.
 */
#[no_mangle]
pub unsafe extern "C" fn CMap_add_codespacerange(
    mut cmap: *mut CMap,
    mut codelo: *const u8,
    mut codehi: *const u8,
    mut dim: size_t,
) -> i32 {
    let mut csr: *mut rangeDef = 0 as *mut rangeDef;
    let mut i: u32 = 0;
    if !cmap.is_null() && dim > 0i32 as u64 {
    } else {
        __assert_fail(b"cmap && dim > 0\x00" as *const u8 as
                          *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      510_u32,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[i8; 90]>(b"int CMap_add_codespacerange(CMap *, const unsigned char *, const unsigned char *, size_t)\x00")).as_ptr());
    }
    i = 0_u32;
    while i < (*cmap).codespace.num {
        let mut j: size_t = 0;
        let mut overlap: bool = 1i32 != 0;
        csr = (*cmap).codespace.ranges.offset(i as isize);
        j = 0i32 as size_t;
        while j < (if (*csr).dim < dim { (*csr).dim } else { dim }) && overlap as i32 != 0 {
            if *codelo.offset(j as isize) as i32 >= *(*csr).codeLo.offset(j as isize) as i32
                && *codelo.offset(j as isize) as i32 <= *(*csr).codeHi.offset(j as isize) as i32
                || *codehi.offset(j as isize) as i32 >= *(*csr).codeLo.offset(j as isize) as i32
                    && *codehi.offset(j as isize) as i32 <= *(*csr).codeHi.offset(j as isize) as i32
            {
                overlap = 1i32 != 0
            } else {
                overlap = 0i32 != 0
            }
            j = j.wrapping_add(1)
        }
        if overlap {
            dpx_warning(b"Overlapping codespace found. (ingored)\x00" as *const u8 as *const i8);
            return -1i32;
        }
        i = i.wrapping_add(1)
    }
    if dim < (*cmap).profile.minBytesIn {
        (*cmap).profile.minBytesIn = dim
    }
    if dim > (*cmap).profile.maxBytesIn {
        (*cmap).profile.maxBytesIn = dim
    }
    if (*cmap).codespace.num.wrapping_add(1_u32) > (*cmap).codespace.max {
        (*cmap).codespace.max = (*cmap).codespace.max.wrapping_add(10_u32);
        (*cmap).codespace.ranges = renew(
            (*cmap).codespace.ranges as *mut libc::c_void,
            ((*cmap).codespace.max as u64).wrapping_mul(::std::mem::size_of::<rangeDef>() as u64)
                as u32,
        ) as *mut rangeDef
    }
    csr = (*cmap)
        .codespace
        .ranges
        .offset((*cmap).codespace.num as isize);
    (*csr).dim = dim;
    (*csr).codeHi = get_mem(cmap, dim as i32);
    (*csr).codeLo = get_mem(cmap, dim as i32);
    memcpy(
        (*csr).codeHi as *mut libc::c_void,
        codehi as *const libc::c_void,
        dim,
    );
    memcpy(
        (*csr).codeLo as *mut libc::c_void,
        codelo as *const libc::c_void,
        dim,
    );
    (*cmap).codespace.num = (*cmap).codespace.num.wrapping_add(1);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_notdefchar(
    mut cmap: *mut CMap,
    mut src: *const u8,
    mut srcdim: size_t,
    mut dst: CID,
) -> i32 {
    return CMap_add_notdefrange(cmap, src, src, srcdim, dst);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_notdefrange(
    mut cmap: *mut CMap,
    mut srclo: *const u8,
    mut srchi: *const u8,
    mut srcdim: size_t,
    mut dst: CID,
) -> i32 {
    let mut c: i32 = 0;
    let mut cur: *mut mapDef = 0 as *mut mapDef;
    if !cmap.is_null() {
    } else {
        __assert_fail(b"cmap\x00" as *const u8 as *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      564_u32,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[i8; 92]>(b"int CMap_add_notdefrange(CMap *, const unsigned char *, const unsigned char *, size_t, CID)\x00")).as_ptr());
    }
    /* dst not used here */
    /* FIXME */
    if check_range(
        cmap,
        srclo,
        srchi,
        srcdim,
        &mut dst as *mut CID as *const u8,
        2i32 as size_t,
    ) < 0i32
    {
        return -1i32;
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new()
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&mut cur, srclo, srcdim as i32) < 0i32 {
        return -1i32;
    }
    c = *srclo.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32;
    while c <= *srchi.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32 {
        if if (*cur.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } != 0
        {
            if __silent == 0 {
                dpx_warning(
                    b"Trying to redefine already defined code mapping. (ignored)\x00" as *const u8
                        as *const i8,
                );
            }
        } else {
            (*cur.offset(c as isize)).flag = 0i32 | 1i32 << 3i32;
            let ref mut fresh1 = (*cur.offset(c as isize)).code;
            *fresh1 = get_mem(cmap, 2i32);
            (*cur.offset(c as isize)).len = 2i32 as size_t;
            *(*cur.offset(c as isize)).code.offset(0) = (dst as i32 >> 8i32) as u8;
            *(*cur.offset(c as isize)).code.offset(1) = (dst as i32 & 0xffi32) as u8
        }
        c += 1
        /* Do not do dst++ for notdefrange  */
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_bfchar(
    mut cmap: *mut CMap,
    mut src: *const u8,
    mut srcdim: size_t,
    mut dst: *const u8,
    mut dstdim: size_t,
) -> i32 {
    return CMap_add_bfrange(cmap, src, src, srcdim, dst, dstdim);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_bfrange(
    mut cmap: *mut CMap,
    mut srclo: *const u8,
    mut srchi: *const u8,
    mut srcdim: size_t,
    mut base: *const u8,
    mut dstdim: size_t,
) -> i32 {
    let mut c: i32 = 0;
    let mut last_byte: i32 = 0;
    let mut i: i32 = 0;
    let mut cur: *mut mapDef = 0 as *mut mapDef;
    if !cmap.is_null() {
    } else {
        __assert_fail(b"cmap\x00" as *const u8 as *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      610_u32,
                      (*::std::mem::transmute::<&[u8; 114],
                                                &[i8; 114]>(b"int CMap_add_bfrange(CMap *, const unsigned char *, const unsigned char *, size_t, const unsigned char *, size_t)\x00")).as_ptr());
    }
    if check_range(cmap, srclo, srchi, srcdim, base, dstdim) < 0i32 {
        return -1i32;
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new()
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&mut cur, srclo, srcdim as i32) < 0i32 {
        return -1i32;
    }
    c = *srclo.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32;
    while c <= *srchi.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32 {
        /* According to 5014.CIDFont_Spec.pdf (p.52),
         * Code mappings (unlike codespace ranges) may overlap,
         * but succeeding maps superceded preceding maps.
         * (reported and patched by Luo Jie on 2007/12/2)
         */
        if (if (*cur.offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        }) == 0
            || (*cur.offset(c as isize)).len < dstdim
        {
            (*cur.offset(c as isize)).flag = 0i32 | 1i32 << 2i32;
            let ref mut fresh2 = (*cur.offset(c as isize)).code;
            *fresh2 = get_mem(cmap, dstdim as i32)
        }
        /*
         * We assume restriction to code ranges also applied here.
         * Addition <00FF> + 1 is undefined.
         *
         * Changed on 2004-03-20:
         *
         *  Should be treated as <0100> in Acrobat's "ToUnicode" CMap.
         */
        (*cur.offset(c as isize)).len = dstdim;
        memcpy(
            (*cur.offset(c as isize)).code as *mut libc::c_void,
            base as *const libc::c_void,
            dstdim,
        );
        last_byte = c - *srclo.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32
            + *base.offset(dstdim.wrapping_sub(1i32 as u64) as isize) as i32;
        *(*cur.offset(c as isize))
            .code
            .offset(dstdim.wrapping_sub(1i32 as u64) as isize) = (last_byte & 0xffi32) as u8;
        i = dstdim.wrapping_sub(2i32 as u64) as i32;
        while i >= 0i32 && last_byte > 255i32 {
            last_byte = *(*cur.offset(c as isize)).code.offset(i as isize) as i32 + 1i32;
            *(*cur.offset(c as isize)).code.offset(i as isize) = (last_byte & 0xffi32) as u8;
            i -= 1
        }
        c += 1
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_cidchar(
    mut cmap: *mut CMap,
    mut src: *const u8,
    mut srcdim: size_t,
    mut dst: CID,
) -> i32 {
    return CMap_add_cidrange(cmap, src, src, srcdim, dst);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_add_cidrange(
    mut cmap: *mut CMap,
    mut srclo: *const u8,
    mut srchi: *const u8,
    mut srcdim: size_t,
    mut base: CID,
) -> i32 {
    let mut i: size_t = 0;
    let mut c: size_t = 0;
    let mut v: size_t = 0;
    let mut cur: *mut mapDef = 0 as *mut mapDef;
    if !cmap.is_null() {
    } else {
        __assert_fail(b"cmap\x00" as *const u8 as *const i8,
                      b"dpx-cmap.c\x00" as *const u8 as *const i8,
                      666_u32,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[i8; 89]>(b"int CMap_add_cidrange(CMap *, const unsigned char *, const unsigned char *, size_t, CID)\x00")).as_ptr());
    }
    /* base not used here */
    if check_range(
        cmap,
        srclo,
        srchi,
        srcdim,
        &mut base as *mut CID as *const u8,
        2i32 as size_t,
    ) < 0i32
    {
        /* FIXME */
        return -1i32;
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new()
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&mut cur, srclo, srcdim as i32) < 0i32 {
        return -1i32;
    }
    v = 0i32 as size_t;
    i = 0i32 as size_t;
    while i < srcdim.wrapping_sub(1i32 as u64) {
        v = (v << 8i32).wrapping_add(*srclo.offset(i as isize) as u64);
        i = i.wrapping_add(1)
    }
    *(*cmap).reverseMap.offset(base as isize) = v as i32;
    c = *srclo.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as size_t;
    while c <= *srchi.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as u64 {
        if (*cur.offset(c as isize)).flag != 0i32 {
            if __silent == 0 {
                dpx_warning(
                    b"Trying to redefine already defined CID mapping. (ignored)\x00" as *const u8
                        as *const i8,
                );
            }
        } else {
            (*cur.offset(c as isize)).flag = 0i32 | 1i32 << 0i32;
            (*cur.offset(c as isize)).len = 2i32 as size_t;
            let ref mut fresh3 = (*cur.offset(c as isize)).code;
            *fresh3 = get_mem(cmap, 2i32);
            *(*cur.offset(c as isize)).code.offset(0) = (base as i32 >> 8i32) as u8;
            *(*cur.offset(c as isize)).code.offset(1) = (base as i32 & 0xffi32) as u8;
            *(*cmap).reverseMap.offset(base as isize) = (v << 8i32).wrapping_add(c) as i32
        }
        if base as i32 >= 65535i32 {
            dpx_warning(b"CID number too large.\x00" as *const u8 as *const i8);
        }
        base = base.wrapping_add(1);
        c = c.wrapping_add(1)
    }
    return 0i32;
}
unsafe extern "C" fn mapDef_release(mut t: *mut mapDef) {
    let mut c: i32 = 0;
    if !t.is_null() {
    } else {
        __assert_fail(
            b"t\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            709_u32,
            (*::std::mem::transmute::<&[u8; 30], &[i8; 30]>(b"void mapDef_release(mapDef *)\x00"))
                .as_ptr(),
        );
    }
    c = 0i32;
    while c < 256i32 {
        if (*t.offset(c as isize)).flag & 1i32 << 4i32 != 0 {
            mapDef_release((*t.offset(c as isize)).next);
        }
        c += 1
    }
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn mapDef_new() -> *mut mapDef {
    let mut t: *mut mapDef = 0 as *mut mapDef;
    let mut c: i32 = 0;
    t = new((256_u64).wrapping_mul(::std::mem::size_of::<mapDef>() as u64) as u32) as *mut mapDef;
    c = 0i32;
    while c < 256i32 {
        (*t.offset(c as isize)).flag = 0i32 | 0i32;
        let ref mut fresh4 = (*t.offset(c as isize)).code;
        *fresh4 = 0 as *mut u8;
        let ref mut fresh5 = (*t.offset(c as isize)).next;
        *fresh5 = 0 as *mut mapDef;
        c += 1
    }
    return t;
}
unsafe extern "C" fn get_mem(mut cmap: *mut CMap, mut size: i32) -> *mut u8 {
    let mut map: *mut mapData = 0 as *mut mapData;
    let mut p: *mut u8 = 0 as *mut u8;
    if !cmap.is_null() && !(*cmap).mapData.is_null() && size >= 0i32 {
    } else {
        __assert_fail(
            b"cmap && cmap->mapData && size >= 0\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            739_u32,
            (*::std::mem::transmute::<&[u8; 36], &[i8; 36]>(
                b"unsigned char *get_mem(CMap *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    map = (*cmap).mapData;
    if (*map).pos + size >= 4096i32 {
        let mut prev: *mut mapData = map;
        map = new((1_u64).wrapping_mul(::std::mem::size_of::<mapData>() as u64) as u32)
            as *mut mapData;
        (*map).data =
            new((4096_u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        (*map).prev = prev;
        (*map).pos = 0i32;
        (*cmap).mapData = map
    }
    p = (*map).data.offset((*map).pos as isize);
    (*map).pos += size;
    return p;
}
unsafe extern "C" fn locate_tbl(
    mut cur: *mut *mut mapDef,
    mut code: *const u8,
    mut dim: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    if !cur.is_null() && !(*cur).is_null() {
    } else {
        __assert_fail(
            b"cur && *cur\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            760_u32,
            (*::std::mem::transmute::<&[u8; 54], &[i8; 54]>(
                b"int locate_tbl(mapDef **, const unsigned char *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < dim - 1i32 {
        c = *code.offset(i as isize) as i32;
        if if (*(*cur).offset(c as isize)).flag & 0xfi32 != 0i32 {
            1i32
        } else {
            0i32
        } != 0
        {
            dpx_warning(b"Ambiguous CMap entry.\x00" as *const u8 as *const i8);
            return -1i32;
        }
        if (*(*cur).offset(c as isize)).next.is_null() {
            /* create new node */
            let ref mut fresh6 = (*(*cur).offset(c as isize)).next;
            *fresh6 = mapDef_new()
        }
        (*(*cur).offset(c as isize)).flag |= 1i32 << 4i32;
        *cur = (*(*cur).offset(c as isize)).next;
        i += 1
    }
    return 0i32;
}
/* Private funcs. */
/*
 * Guess how many bytes consumed as a `single' character:
 * Substring of length bytesconsumed bytes of input string is interpreted as
 * a `single' character by CMap_decode().
 */
unsafe extern "C" fn bytes_consumed(
    mut cmap: *mut CMap,
    mut instr: *const u8,
    mut inbytes: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    let mut longest: size_t = 0i32 as size_t;
    let mut bytesconsumed: size_t = 0;
    if !cmap.is_null() {
    } else {
        __assert_fail(
            b"cmap\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            786_u32,
            (*::std::mem::transmute::<&[u8; 61], &[i8; 61]>(
                b"size_t bytes_consumed(CMap *, const unsigned char *, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32 as size_t;
    while i < (*cmap).codespace.num as u64 {
        let mut csr: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
        pos = 0i32 as size_t;
        while pos
            < (if (*csr).dim < inbytes {
                (*csr).dim
            } else {
                inbytes
            })
        {
            if *instr.offset(pos as isize) as i32 > *(*csr).codeHi.offset(pos as isize) as i32
                || (*instr.offset(pos as isize) as i32) < *(*csr).codeLo.offset(pos as isize) as i32
            {
                break;
            }
            pos = pos.wrapping_add(1)
        }
        if pos == (*csr).dim {
            /* part of instr is totally valid in this codespace. */
            return (*csr).dim;
        }
        if pos > longest {
            longest = pos
        }
        i = i.wrapping_add(1)
    }
    if i == (*cmap).codespace.num as u64 {
        /* No matching at all */
        bytesconsumed = (*cmap).profile.minBytesIn
    } else {
        bytesconsumed = (*cmap).profile.maxBytesIn;
        i = 0i32 as size_t;
        while i < (*cmap).codespace.num as u64 {
            let mut csr_0: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
            if (*csr_0).dim > longest && (*csr_0).dim < bytesconsumed {
                bytesconsumed = (*csr_0).dim
            }
            i = i.wrapping_add(1)
        }
    }
    return bytesconsumed;
}
unsafe extern "C" fn check_range(
    mut cmap: *mut CMap,
    mut srclo: *const u8,
    mut srchi: *const u8,
    mut srcdim: size_t,
    mut dst: *const u8,
    mut dstdim: size_t,
) -> i32 {
    if srcdim < 1i32 as u64
        || dstdim < 1i32 as u64
        || (srclo.is_null() || srchi.is_null() || dst.is_null())
        || memcmp(
            srclo as *const libc::c_void,
            srchi as *const libc::c_void,
            srcdim.wrapping_sub(1i32 as u64),
        ) != 0
        || *srclo.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32
            > *srchi.offset(srcdim.wrapping_sub(1i32 as u64) as isize) as i32
    {
        dpx_warning(b"Invalid CMap mapping entry. (ignored)\x00" as *const u8 as *const i8);
        return -1i32;
    }
    if CMap_match_codespace(cmap, srclo, srcdim) < 0i32
        || CMap_match_codespace(cmap, srchi, srcdim) < 0i32
    {
        dpx_warning(b"Invalid CMap mapping entry. (ignored)\x00" as *const u8 as *const i8);
        return -1i32;
    }
    if srcdim < (*cmap).profile.minBytesIn {
        (*cmap).profile.minBytesIn = srcdim
    }
    if srcdim > (*cmap).profile.maxBytesIn {
        (*cmap).profile.maxBytesIn = srcdim
    }
    if dstdim < (*cmap).profile.minBytesOut {
        (*cmap).profile.minBytesOut = dstdim
    }
    if dstdim > (*cmap).profile.maxBytesOut {
        (*cmap).profile.maxBytesOut = dstdim
    }
    return 0i32;
}
static mut __cache: *mut CMap_cache = 0 as *const CMap_cache as *mut CMap_cache;
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_init() {
    static mut range_min: [u8; 2] = [0; 2];
    static mut range_max: [u8; 2] = [0xff_u8, 0xff_u8];
    if !__cache.is_null() {
        _tt_abort(
            b"%s: Already initialized.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    }
    __cache = new((1_u64).wrapping_mul(::std::mem::size_of::<CMap_cache>() as u64) as u32)
        as *mut CMap_cache;
    (*__cache).max = 16u32 as i32;
    (*__cache).cmaps = new(((*__cache).max as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*mut CMap>() as u64) as u32)
        as *mut *mut CMap;
    (*__cache).num = 0i32;
    /* Create Identity mapping */
    let ref mut fresh7 = *(*__cache).cmaps.offset(0);
    *fresh7 = CMap_new();
    CMap_set_name(
        *(*__cache).cmaps.offset(0),
        b"Identity-H\x00" as *const u8 as *const i8,
    );
    CMap_set_type(*(*__cache).cmaps.offset(0), 0i32);
    CMap_set_wmode(*(*__cache).cmaps.offset(0), 0i32);
    CMap_set_CIDSysInfo(*(*__cache).cmaps.offset(0), &mut CSI_IDENTITY);
    CMap_add_codespacerange(
        *(*__cache).cmaps.offset(0),
        range_min.as_mut_ptr(),
        range_max.as_mut_ptr(),
        2i32 as size_t,
    );
    let ref mut fresh8 = *(*__cache).cmaps.offset(1);
    *fresh8 = CMap_new();
    CMap_set_name(
        *(*__cache).cmaps.offset(1),
        b"Identity-V\x00" as *const u8 as *const i8,
    );
    CMap_set_type(*(*__cache).cmaps.offset(1), 0i32);
    CMap_set_wmode(*(*__cache).cmaps.offset(1), 1i32);
    CMap_set_CIDSysInfo(*(*__cache).cmaps.offset(1), &mut CSI_IDENTITY);
    CMap_add_codespacerange(
        *(*__cache).cmaps.offset(1),
        range_min.as_mut_ptr(),
        range_max.as_mut_ptr(),
        2i32 as size_t,
    );
    (*__cache).num += 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_get(mut id: i32) -> *mut CMap {
    if __cache.is_null() {
        _tt_abort(
            b"%s: CMap cache not initialized.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    }
    if id < 0i32 || id >= (*__cache).num {
        _tt_abort(b"Invalid CMap ID %d\x00" as *const u8 as *const i8, id);
    }
    return *(*__cache).cmaps.offset(id as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_find(mut cmap_name: *const i8) -> i32 {
    let mut id: i32 = 0i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if __cache.is_null() {
        CMap_cache_init();
    }
    if !__cache.is_null() {
    } else {
        __assert_fail(
            b"__cache\x00" as *const u8 as *const i8,
            b"dpx-cmap.c\x00" as *const u8 as *const i8,
            914_u32,
            (*::std::mem::transmute::<&[u8; 34], &[i8; 34]>(
                b"int CMap_cache_find(const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    id = 0i32;
    while id < (*__cache).num {
        let mut name: *mut i8 = 0 as *mut i8;
        /* CMapName may be undefined when processing usecmap. */
        name = CMap_get_name(*(*__cache).cmaps.offset(id as isize));
        if !name.is_null() && streq_ptr(cmap_name, name) as i32 != 0 {
            return id;
        }
        id += 1
    }
    handle = ttstub_input_open(cmap_name, TTIF_CMAP, 0i32);
    if handle.is_null() {
        return -1i32;
    }
    if CMap_parse_check_sig(handle) < 0i32 {
        ttstub_input_close(handle);
        return -1i32;
    }
    if __verbose != 0 {
        dpx_message(b"(CMap:%s\x00" as *const u8 as *const i8, cmap_name);
    }
    if (*__cache).num >= (*__cache).max {
        (*__cache).max = ((*__cache).max as u32).wrapping_add(16u32) as i32 as i32;
        (*__cache).cmaps = renew(
            (*__cache).cmaps as *mut libc::c_void,
            ((*__cache).max as u32 as u64).wrapping_mul(::std::mem::size_of::<*mut CMap>() as u64)
                as u32,
        ) as *mut *mut CMap
    }
    id = (*__cache).num;
    (*__cache).num += 1;
    let ref mut fresh9 = *(*__cache).cmaps.offset(id as isize);
    *fresh9 = CMap_new();
    if CMap_parse(*(*__cache).cmaps.offset(id as isize), handle) < 0i32 {
        _tt_abort(
            b"%s: Parsing CMap file failed.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    }
    ttstub_input_close(handle);
    if __verbose != 0 {
        dpx_message(b")\x00" as *const u8 as *const i8);
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_add(mut cmap: *mut CMap) -> i32 {
    let mut id: i32 = 0;
    let mut cmap_name0: *mut i8 = 0 as *mut i8;
    let mut cmap_name1: *mut i8 = 0 as *mut i8;
    if !CMap_is_valid(cmap) {
        _tt_abort(
            b"%s: Invalid CMap.\x00" as *const u8 as *const i8,
            b"CMap\x00" as *const u8 as *const i8,
        );
    }
    id = 0i32;
    while id < (*__cache).num {
        cmap_name0 = CMap_get_name(cmap);
        cmap_name1 = CMap_get_name(*(*__cache).cmaps.offset(id as isize));
        if streq_ptr(cmap_name0, cmap_name1) {
            _tt_abort(
                b"%s: CMap \"%s\" already defined.\x00" as *const u8 as *const i8,
                b"CMap\x00" as *const u8 as *const i8,
                cmap_name0,
            );
        }
        id += 1
    }
    if (*__cache).num >= (*__cache).max {
        (*__cache).max = ((*__cache).max as u32).wrapping_add(16u32) as i32 as i32;
        (*__cache).cmaps = renew(
            (*__cache).cmaps as *mut libc::c_void,
            ((*__cache).max as u32 as u64).wrapping_mul(::std::mem::size_of::<*mut CMap>() as u64)
                as u32,
        ) as *mut *mut CMap
    }
    id = (*__cache).num;
    (*__cache).num += 1;
    let ref mut fresh10 = *(*__cache).cmaps.offset(id as isize);
    *fresh10 = cmap;
    return id;
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
/* Limits */
/*
 * TYPE_IDENTITY and TYPE_CID_TO_CODE is not defined in the CMap spec.
 */
/* ************************* CMAP_MAIN **************************/
/* charName not supported */
#[no_mangle]
pub unsafe extern "C" fn CMap_cache_close() {
    if !__cache.is_null() {
        let mut id: i32 = 0;
        id = 0i32;
        while id < (*__cache).num {
            CMap_release(*(*__cache).cmaps.offset(id as isize));
            id += 1
        }
        free((*__cache).cmaps as *mut libc::c_void);
        __cache = mfree(__cache as *mut libc::c_void) as *mut CMap_cache
    };
}
