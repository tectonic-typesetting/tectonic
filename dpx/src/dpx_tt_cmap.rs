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

use super::dpx_sfnt::{
    dfont_open, sfnt_close, sfnt_find_table_pos, sfnt_locate_table, sfnt_open,
    sfnt_read_table_directory,
};
use crate::{info, warn};

use super::dpx_agl::agl_get_unicodes;
use super::dpx_cff::{
    cff_charsets_lookup_inverse, cff_close, cff_get_glyphname, cff_get_string, cff_open,
    cff_read_charsets,
};
use super::dpx_cff_dict::{cff_dict_get, cff_dict_known};
use super::dpx_cid::{CSI_IDENTITY, CSI_UNICODE};
use super::dpx_cmap::{
    CMap_add_bfchar, CMap_add_cidchar, CMap_add_codespacerange, CMap_cache_add, CMap_cache_find,
    CMap_cache_get, CMap_decode, CMap_get_type, CMap_new, CMap_release, CMap_reverse_decode,
    CMap_set_CIDSysInfo, CMap_set_name, CMap_set_silent, CMap_set_type, CMap_set_wmode,
};
use super::dpx_cmap_write::CMap_create_stream;
use super::dpx_dpxfile::{dpx_open_dfont_file, dpx_open_opentype_file, dpx_open_truetype_file};
use super::dpx_numbers::{
    tt_get_signed_pair, tt_get_unsigned_byte, tt_get_unsigned_pair, tt_get_unsigned_quad,
};
use super::dpx_pdfresource::{pdf_defineresource, pdf_findresource, pdf_get_resource_reference};
use super::dpx_tt_aux::ttc_read_offset;
use super::dpx_tt_gsub::{
    otl_gsub, otl_gsub_add_feat, otl_gsub_add_feat_list, otl_gsub_apply, otl_gsub_apply_chain,
    otl_gsub_new, otl_gsub_release, otl_gsub_select, otl_gsub_set_chain, otl_gsub_set_verbose,
};
use super::dpx_tt_post::{tt_get_glyphname, tt_read_post_table, tt_release_post_table};
use super::dpx_tt_table::tt_read_maxp_table;
use super::dpx_unicode::UC_UTF16BE_encode_char;
use crate::dpx_pdfobj::pdf_obj;
use crate::mfree;
use crate::{ttstub_input_close, ttstub_input_seek};
use libc::free;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */
pub type Fixed = u32;
/* 16.16-bit signed fixed-point number */
pub type FWord = i16;

use super::dpx_sfnt::sfnt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_cmap {
    pub format: u16,
    pub platform: u16,
    pub encoding: u16,
    pub language: u32,
    pub map: *mut libc::c_void,
}

use super::dpx_cid::CIDSysInfo;

use super::dpx_cff::cff_font;
pub type card16 = u16;
use super::dpx_cff::cff_charsets;
use super::dpx_cff::cff_range2;
/* CFF Data Types */
/* SID SID number */
/* offset(0) */
/* size offset(0) */
/* 1-byte unsigned number */
/* 2-byte unsigned number */
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
/* 1, 2, 3, or 4-byte offset */
pub type s_SID = u16;
use super::dpx_cff::cff_range1;
/* 2-byte string identifier  */
/* number of objects stored in INDEX */
/* Offset array element size, 1-4    */
/* Offset array, count + 1 offsets   */
/* Object data                       */
/* format major version (starting at 1) */
/* format minor version (starting at 0) */
/* Header size (bytes)                  */
/* Absolute offset (0) size             */
/* Dictionary */
/* encoded data value (as card8 or card16) */
/* opname                                 */
/* number of values                        */
/* values                                  */
/* Encoding, Charset and FDSelect */
/* SID or CID, or card8 for Encoding  */
/* no. of remaining gids/codes in this range */
/* SID or CID (card16)      */
/* card16-version of range1 */

use super::dpx_tt_post::tt_post_table;

/* Mapping types, MAP_IS_NAME is not supported. */
/* Lookup flags */
/* DEBUG */
/* Codespacerange */
/* Dimension of this codespacerange */
/* Lower bounds of valid input code */
/* Upper bounds of valid input code */
/* 2 for CID, variable for Code..  */
/* CID (as 16-bit BE), Code ...    */
/* Next Subtbl for LOOKUP_CONTINUE */
/* CID, Code... MEM_ALLOC_SIZE bytes  */
/* Previous mapData data segment      */
/* Position of next free data segment */
use super::dpx_cmap::CMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap12 {
    pub nGroups: u32,
    pub groups: *mut charGroup,
}
/* Format 8 and 10 not supported...
 *
 *  format  8: mixed 16-bit and 32-bit coverage
 *  format 10: trimmed array
 */
/*
 * format 12: segmented coverage
 *
 * startGlyphID is 32-bit long, however, GlyphID is still 16-bit long !
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charGroup {
    pub startCharCode: u32,
    pub endCharCode: u32,
    pub startGlyphID: u32,
}
/* format 6: trimmed table mapping */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap6 {
    pub firstCode: u16,
    pub entryCount: u16,
    pub glyphIndexArray: *mut u16,
}
/*
 * format 4: segment mapping to delta values
 * - Microsoft standard character to glyph index mapping table
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap4 {
    pub segCountX2: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub endCount: *mut u16,
    pub reservedPad: u16,
    pub startCount: *mut u16,
    pub idDelta: *mut u16,
    pub idRangeOffset: *mut u16,
    pub glyphIndexArray: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap2 {
    pub subHeaderKeys: [u16; 256],
    pub subHeaders: *mut SubHeader,
    pub glyphIndexArray: *mut u16,
}
/* format 2: high-byte mapping through table */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubHeader {
    pub firstCode: u16,
    pub entryCount: u16,
    pub idDelta: i16,
    pub idRangeOffset: u16,
}
/* format 0: byte encoding table */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap0 {
    pub glyphIndexArray: [u8; 256],
}
pub type CID = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap_plat_enc_rec {
    pub platform: i16,
    pub encoding: i16,
}
use super::dpx_tt_table::tt_maxp_table;
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn otf_cmap_set_verbose(mut level: i32) {
    otl_gsub_set_verbose(level);
    verbose = level;
}
unsafe extern "C" fn read_cmap0(mut sfont: *mut sfnt, mut len: u32) -> *mut cmap0 {
    let mut map: *mut cmap0 = 0 as *mut cmap0;
    let mut i: u32 = 0;
    if len < 256_u32 {
        panic!("invalid cmap subtable");
    }
    map = new((1_u64).wrapping_mul(::std::mem::size_of::<cmap0>() as u64) as u32) as *mut cmap0;
    i = 0_u32;
    while i < 256_u32 {
        (*map).glyphIndexArray[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i = i.wrapping_add(1)
    }
    map
}
unsafe extern "C" fn release_cmap0(mut map: *mut cmap0) {
    free(map as *mut libc::c_void);
}
unsafe extern "C" fn lookup_cmap0(mut map: *mut cmap0, mut cc: u16) -> u16 {
    return (if cc as i32 > 255i32 {
        0i32
    } else {
        (*map).glyphIndexArray[cc as usize] as i32
    }) as u16;
}
unsafe extern "C" fn read_cmap2(mut sfont: *mut sfnt, mut len: u32) -> *mut cmap2 {
    let mut map: *mut cmap2 = 0 as *mut cmap2;
    let mut i: u16 = 0;
    let mut n: u16 = 0;
    if len < 512_u32 {
        panic!("invalid cmap subtable");
    }
    map = new((1_u64).wrapping_mul(::std::mem::size_of::<cmap2>() as u64) as u32) as *mut cmap2;
    i = 0_u16;
    while (i as i32) < 256i32 {
        (*map).subHeaderKeys[i as usize] = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    n = 0_u16;
    i = 0_u16;
    while (i as i32) < 256i32 {
        (*map).subHeaderKeys[i as usize] = ((*map).subHeaderKeys[i as usize] as i32 / 8i32) as u16;
        if (n as i32) < (*map).subHeaderKeys[i as usize] as i32 {
            n = (*map).subHeaderKeys[i as usize]
        }
        i = i.wrapping_add(1)
    }
    n = (n as i32 + 1i32) as u16;
    (*map).subHeaders =
        new((n as u32 as u64).wrapping_mul(::std::mem::size_of::<SubHeader>() as u64) as u32)
            as *mut SubHeader;
    i = 0_u16;
    while (i as i32) < n as i32 {
        (*(*map).subHeaders.offset(i as isize)).firstCode = tt_get_unsigned_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).entryCount = tt_get_unsigned_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).idDelta = tt_get_signed_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).idRangeOffset =
            tt_get_unsigned_pair((*sfont).handle);
        /* It makes things easier to let the offset starts from
         * the beginning of glyphIndexArray.
         */
        if (*(*map).subHeaders.offset(i as isize)).idRangeOffset as i32 != 0i32 {
            let ref mut fresh0 = (*(*map).subHeaders.offset(i as isize)).idRangeOffset;
            *fresh0 = (*fresh0 as i32 - (2i32 + (n as i32 - i as i32 - 1i32) * 8i32)) as u16
        }
        i = i.wrapping_add(1)
    }
    /* Caculate the length of glyphIndexArray, this is ugly,
     * there should be a better way to get this information.
     */
    n = (len
        .wrapping_sub(518_u32)
        .wrapping_sub((n as i32 * 8i32) as u32) as u16 as i32
        / 2i32) as u16;
    (*map).glyphIndexArray =
        new((n as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32) as *mut u16;
    i = 0_u16;
    while (i as i32) < n as i32 {
        *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    map
}
unsafe extern "C" fn release_cmap2(mut map: *mut cmap2) {
    if !map.is_null() {
        free((*map).subHeaders as *mut libc::c_void);
        free((*map).glyphIndexArray as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap2(mut map: *mut cmap2, mut cc: u16) -> u16 {
    let mut idx: u16 = 0_u16;
    let mut idDelta: i16 = 0;
    let mut firstCode: u16 = 0;
    let mut entryCount: u16 = 0;
    let mut idRangeOffset: u16 = 0;
    let mut hi: i32 = 0;
    let mut lo: i32 = 0;
    let mut i: u16 = 0;
    hi = cc as i32 >> 8i32 & 0xffi32;
    lo = cc as i32 & 0xffi32;
    /* select which subHeader to use */
    i = (*map).subHeaderKeys[hi as usize];
    firstCode = (*(*map).subHeaders.offset(i as isize)).firstCode;
    entryCount = (*(*map).subHeaders.offset(i as isize)).entryCount;
    idDelta = (*(*map).subHeaders.offset(i as isize)).idDelta;
    idRangeOffset = ((*(*map).subHeaders.offset(i as isize)).idRangeOffset as i32 / 2i32) as u16;
    if lo >= firstCode as i32 && lo < firstCode as i32 + entryCount as i32 {
        idRangeOffset = (idRangeOffset as i32 + (lo - firstCode as i32)) as u16;
        idx = *(*map).glyphIndexArray.offset(idRangeOffset as isize);
        if idx as i32 != 0i32 {
            idx = (idx as i32 + idDelta as i32 & 0xffffi32) as u16
        }
    }
    idx
}
unsafe extern "C" fn read_cmap4(mut sfont: *mut sfnt, mut len: u32) -> *mut cmap4 {
    let mut map: *mut cmap4 = 0 as *mut cmap4;
    let mut i: u16 = 0;
    let mut n: u16 = 0;
    let mut segCount: u16 = 0;
    if len < 8_u32 {
        panic!("invalid cmap subtable");
    }
    map = new((1_u64).wrapping_mul(::std::mem::size_of::<cmap4>() as u64) as u32) as *mut cmap4;
    segCount = tt_get_unsigned_pair((*sfont).handle);
    (*map).segCountX2 = segCount;
    (*map).searchRange = tt_get_unsigned_pair((*sfont).handle);
    (*map).entrySelector = tt_get_unsigned_pair((*sfont).handle);
    (*map).rangeShift = tt_get_unsigned_pair((*sfont).handle);
    segCount = (segCount as i32 / 2i32) as u16;
    (*map).endCount =
        new((segCount as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32)
            as *mut u16;
    i = 0_u16;
    while (i as i32) < segCount as i32 {
        *(*map).endCount.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    (*map).reservedPad = tt_get_unsigned_pair((*sfont).handle);
    (*map).startCount =
        new((segCount as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32)
            as *mut u16;
    i = 0_u16;
    while (i as i32) < segCount as i32 {
        *(*map).startCount.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    (*map).idDelta =
        new((segCount as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32)
            as *mut u16;
    i = 0_u16;
    while (i as i32) < segCount as i32 {
        *(*map).idDelta.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    (*map).idRangeOffset =
        new((segCount as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32)
            as *mut u16;
    i = 0_u16;
    while (i as i32) < segCount as i32 {
        *(*map).idRangeOffset.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    n = len
        .wrapping_sub(16_u32)
        .wrapping_sub((8i32 * segCount as i32) as u32)
        .wrapping_div(2_u32) as u16;
    if n as i32 == 0i32 {
        (*map).glyphIndexArray = 0 as *mut u16
    } else {
        (*map).glyphIndexArray = new((n as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u16>() as u64)
            as u32) as *mut u16;
        i = 0_u16;
        while (i as i32) < n as i32 {
            *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
            i = i.wrapping_add(1)
        }
    }
    map
}
unsafe extern "C" fn release_cmap4(mut map: *mut cmap4) {
    if !map.is_null() {
        free((*map).endCount as *mut libc::c_void);
        free((*map).startCount as *mut libc::c_void);
        free((*map).idDelta as *mut libc::c_void);
        free((*map).idRangeOffset as *mut libc::c_void);
        free((*map).glyphIndexArray as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap4(mut map: *mut cmap4, mut cc: u16) -> u16 {
    let mut gid: u16 = 0_u16;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut segCount: u16 = 0;
    /*
     * Segments are sorted in order of increasing endCode values.
     * Last segment maps 0xffff to gid 0 (?)
     */
    segCount = ((*map).segCountX2 as i32 / 2i32) as u16;
    i = segCount;
    loop {
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 as i32 > 0i32 && cc as i32 <= *(*map).endCount.offset(i as isize) as i32) {
            break;
        }
        if !(cc as i32 >= *(*map).startCount.offset(i as isize) as i32) {
            continue;
        }
        if *(*map).idRangeOffset.offset(i as isize) as i32 == 0i32 {
            gid = (cc as i32 + *(*map).idDelta.offset(i as isize) as i32 & 0xffffi32) as u16
        } else if cc as i32 == 0xffffi32
            && *(*map).idRangeOffset.offset(i as isize) as i32 == 0xffffi32
        {
            /* this is for protection against some old broken fonts... */
            gid = 0_u16
        } else {
            j = (*(*map).idRangeOffset.offset(i as isize) as i32
                - (segCount as i32 - i as i32) * 2i32) as u16;
            j = (cc as i32 - *(*map).startCount.offset(i as isize) as i32 + j as i32 / 2i32) as u16;
            gid = *(*map).glyphIndexArray.offset(j as isize);
            if gid as i32 != 0i32 {
                gid = (gid as i32 + *(*map).idDelta.offset(i as isize) as i32 & 0xffffi32) as u16
            }
        }
        break;
    }
    gid
}
unsafe extern "C" fn read_cmap6(mut sfont: *mut sfnt, mut len: u32) -> *mut cmap6 {
    let mut map: *mut cmap6 = 0 as *mut cmap6;
    let mut i: u16 = 0;
    if len < 4_u32 {
        panic!("invalid cmap subtable");
    }
    map = new((1_u64).wrapping_mul(::std::mem::size_of::<cmap6>() as u64) as u32) as *mut cmap6;
    (*map).firstCode = tt_get_unsigned_pair((*sfont).handle);
    (*map).entryCount = tt_get_unsigned_pair((*sfont).handle);
    (*map).glyphIndexArray = new(
        ((*map).entryCount as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32
    ) as *mut u16;
    i = 0_u16;
    while (i as i32) < (*map).entryCount as i32 {
        *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1)
    }
    map
}
unsafe extern "C" fn release_cmap6(mut map: *mut cmap6) {
    if !map.is_null() {
        free((*map).glyphIndexArray as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap6(mut map: *mut cmap6, mut cc: u16) -> u16 {
    let mut idx: u16 = 0;
    idx = (cc as i32 - (*map).firstCode as i32) as u16;
    if (idx as i32) < (*map).entryCount as i32 {
        return *(*map).glyphIndexArray.offset(idx as isize);
    }
    0_u16
}
/* ULONG length */
unsafe extern "C" fn read_cmap12(mut sfont: *mut sfnt, mut len: u32) -> *mut cmap12 {
    let mut map: *mut cmap12 = 0 as *mut cmap12;
    let mut i: u32 = 0;
    if len < 4_u32 {
        panic!("invalid cmap subtable");
    }
    map = new((1_u64).wrapping_mul(::std::mem::size_of::<cmap12>() as u64) as u32) as *mut cmap12;
    (*map).nGroups = tt_get_unsigned_quad((*sfont).handle);
    (*map).groups =
        new(((*map).nGroups as u64).wrapping_mul(::std::mem::size_of::<charGroup>() as u64) as u32)
            as *mut charGroup;
    i = 0_u32;
    while i < (*map).nGroups {
        (*(*map).groups.offset(i as isize)).startCharCode = tt_get_unsigned_quad((*sfont).handle);
        (*(*map).groups.offset(i as isize)).endCharCode = tt_get_unsigned_quad((*sfont).handle);
        (*(*map).groups.offset(i as isize)).startGlyphID = tt_get_unsigned_quad((*sfont).handle);
        i = i.wrapping_add(1)
    }
    map
}
unsafe extern "C" fn release_cmap12(mut map: *mut cmap12) {
    if !map.is_null() {
        free((*map).groups as *mut libc::c_void);
        free(map as *mut libc::c_void);
    };
}
unsafe extern "C" fn lookup_cmap12(mut map: *mut cmap12, mut cccc: u32) -> u16 {
    let mut gid: u16 = 0_u16;
    let mut i: i32 = 0;
    i = (*map).nGroups as i32;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 >= 0i32 && cccc <= (*(*map).groups.offset(i as isize)).endCharCode) {
            break;
        }
        if !(cccc >= (*(*map).groups.offset(i as isize)).startCharCode) {
            continue;
        }
        gid = (cccc
            .wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode)
            .wrapping_add((*(*map).groups.offset(i as isize)).startGlyphID)
            & 0xffff_u32) as u16;
        break;
    }
    gid
}
/* read cmap */
#[no_mangle]
pub unsafe extern "C" fn tt_cmap_read(
    mut sfont: *mut sfnt,
    mut platform: u16,
    mut encoding: u16,
) -> *mut tt_cmap {
    let mut cmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut offset: u32 = 0;
    let mut length: u32 = 0_u32;
    let mut p_id: u16 = 0;
    let mut e_id: u16 = 0;
    let mut i: u16 = 0;
    let mut n_subtabs: u16 = 0;
    assert!(!sfont.is_null());
    offset = sfnt_locate_table(sfont, b"cmap\x00" as *const u8 as *const i8);
    tt_get_unsigned_pair((*sfont).handle);
    n_subtabs = tt_get_unsigned_pair((*sfont).handle);
    i = 0_u16;
    while (i as i32) < n_subtabs as i32 {
        p_id = tt_get_unsigned_pair((*sfont).handle);
        e_id = tt_get_unsigned_pair((*sfont).handle);
        if p_id as i32 != platform as i32 || e_id as i32 != encoding as i32 {
            tt_get_unsigned_quad((*sfont).handle);
            i = i.wrapping_add(1)
        } else {
            offset =
                (offset as u32).wrapping_add(tt_get_unsigned_quad((*sfont).handle)) as u32 as u32;
            break;
        }
    }
    if i as i32 == n_subtabs as i32 {
        return 0 as *mut tt_cmap;
    }
    cmap =
        new((1_u64).wrapping_mul(::std::mem::size_of::<tt_cmap>() as u64) as u32) as *mut tt_cmap;
    (*cmap).map = 0 as *mut libc::c_void;
    (*cmap).platform = platform;
    (*cmap).encoding = encoding;
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    (*cmap).format = tt_get_unsigned_pair((*sfont).handle);
    /* Length and version (language) is ULONG for
     * format 8, 10, 12 !
     */
    if (*cmap).format as i32 <= 6i32 {
        length = tt_get_unsigned_pair((*sfont).handle) as u32;
        (*cmap).language = tt_get_unsigned_pair((*sfont).handle) as u32
    /* language (Mac) */
    } else if tt_get_unsigned_pair((*sfont).handle) as i32 != 0i32 {
        /* reverved - 0 */
        warn!("Unrecognized cmap subtable format.");
        tt_cmap_release(cmap);
        return 0 as *mut tt_cmap;
    } else {
        length = tt_get_unsigned_quad((*sfont).handle);
        (*cmap).language = tt_get_unsigned_quad((*sfont).handle)
    }
    match (*cmap).format as i32 {
        0 => (*cmap).map = read_cmap0(sfont, length) as *mut libc::c_void,
        2 => (*cmap).map = read_cmap2(sfont, length) as *mut libc::c_void,
        4 => (*cmap).map = read_cmap4(sfont, length) as *mut libc::c_void,
        6 => (*cmap).map = read_cmap6(sfont, length) as *mut libc::c_void,
        12 => {
            /* dpx_warning("UCS-4 TrueType cmap table..."); */
            (*cmap).map = read_cmap12(sfont, length) as *mut libc::c_void
        }
        _ => {
            warn!("Unrecognized OpenType/TrueType cmap format.");
            tt_cmap_release(cmap);
            return 0 as *mut tt_cmap;
        }
    }
    if (*cmap).map.is_null() {
        tt_cmap_release(cmap);
        cmap = 0 as *mut tt_cmap
    }
    cmap
}
#[no_mangle]
pub unsafe extern "C" fn tt_cmap_release(mut cmap: *mut tt_cmap) {
    if !cmap.is_null() {
        if !(*cmap).map.is_null() {
            match (*cmap).format as i32 {
                0 => {
                    release_cmap0((*cmap).map as *mut cmap0);
                }
                2 => {
                    release_cmap2((*cmap).map as *mut cmap2);
                }
                4 => {
                    release_cmap4((*cmap).map as *mut cmap4);
                }
                6 => {
                    release_cmap6((*cmap).map as *mut cmap6);
                }
                12 => {
                    release_cmap12((*cmap).map as *mut cmap12);
                }
                _ => {
                    panic!("Unrecognized OpenType/TrueType cmap format.");
                }
            }
        }
        free(cmap as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tt_cmap_lookup(mut cmap: *mut tt_cmap, mut cc: u32) -> u16 {
    let mut gid: u16 = 0_u16;
    assert!(!cmap.is_null());
    if cc as i64 > 0xffff && ((*cmap).format as i32) < 12i32 {
        warn!("Four bytes charcode not supported in OpenType/TrueType cmap format 0...6.");
        return 0_u16;
    }
    match (*cmap).format as i32 {
        0 => gid = lookup_cmap0((*cmap).map as *mut cmap0, cc as u16),
        2 => gid = lookup_cmap2((*cmap).map as *mut cmap2, cc as u16),
        4 => gid = lookup_cmap4((*cmap).map as *mut cmap4, cc as u16),
        6 => gid = lookup_cmap6((*cmap).map as *mut cmap6, cc as u16),
        12 => gid = lookup_cmap12((*cmap).map as *mut cmap12, cc),
        _ => {
            panic!("Unrecognized OpenType/TrueType cmap subtable format");
        }
    }
    gid
}
static mut wbuf: [u8; 1024] = [0; 1024];
static mut srange_min: [u8; 2] = [0; 2];
static mut srange_max: [u8; 2] = [0xff_u8, 0xff_u8];
static mut lrange_min: [u8; 4] = [0; 4];
static mut lrange_max: [u8; 4] = [0x7f_u8, 0xff_u8, 0xff_u8, 0xff_u8];
unsafe extern "C" fn load_cmap4(
    mut map: *mut cmap4,
    mut GIDToCIDMap: *mut u8,
    mut gsub_vert: *mut otl_gsub,
    mut gsub_list: *mut otl_gsub,
    mut cmap: *mut CMap,
    mut tounicode_add: *mut CMap,
) {
    let mut c0: u16 = 0;
    let mut c1: u16 = 0;
    let mut gid: u16 = 0;
    let mut cid: u16 = 0;
    let mut j: u16 = 0;
    let mut d: u16 = 0;
    let mut segCount: u16 = 0;
    let mut ch: u16 = 0;
    let mut i: i32 = 0;
    segCount = ((*map).segCountX2 as i32 / 2i32) as u16;
    i = segCount as i32 - 1i32;
    while i >= 0i32 {
        c0 = *(*map).startCount.offset(i as isize);
        c1 = *(*map).endCount.offset(i as isize);
        d = (*(*map).idRangeOffset.offset(i as isize) as i32 / 2i32 - (segCount as i32 - i)) as u16;
        j = 0_u16;
        while j as i32 <= c1 as i32 - c0 as i32 {
            ch = (c0 as i32 + j as i32) as u16;
            if *(*map).idRangeOffset.offset(i as isize) as i32 == 0i32 {
                gid = (ch as i32 + *(*map).idDelta.offset(i as isize) as i32 & 0xffffi32) as u16
            } else if c0 as i32 == 0xffffi32
                && c1 as i32 == 0xffffi32
                && *(*map).idRangeOffset.offset(i as isize) as i32 == 0xffffi32
            {
                /* this is for protection against some old broken fonts... */
                gid = 0_u16
            } else {
                gid = (*(*map)
                    .glyphIndexArray
                    .offset((j as i32 + d as i32) as isize) as i32
                    + *(*map).idDelta.offset(i as isize) as i32
                    & 0xffffi32) as u16
            } /* LONG ? */
            if gid as i32 != 0i32 && gid as i32 != 0xffffi32 {
                if !gsub_list.is_null() {
                    otl_gsub_apply_chain(gsub_list, &mut gid);
                }
                if !gsub_vert.is_null() {
                    otl_gsub_apply(gsub_vert, &mut gid);
                }
                if !GIDToCIDMap.is_null() {
                    cid = ((*GIDToCIDMap.offset((2i32 * gid as i32) as isize) as i32) << 8i32
                        | *GIDToCIDMap.offset((2i32 * gid as i32 + 1i32) as isize) as i32)
                        as u16;
                    if cid as i32 == 0i32 {
                        warn!("GID {} does not have corresponding CID {}.", gid, cid,);
                    }
                } else {
                    cid = gid
                }
                wbuf[0] = 0_u8;
                wbuf[1] = 0_u8;
                wbuf[2] = (ch as i32 >> 8i32 & 0xffi32) as u8;
                wbuf[3] = (ch as i32 & 0xffi32) as u8;
                wbuf[4] = (cid as i32 >> 8i32 & 0xffi32) as u8;
                wbuf[5] = (cid as i32 & 0xffi32) as u8;
                CMap_add_cidchar(cmap, wbuf.as_mut_ptr(), 4i32 as size_t, cid);
                if !tounicode_add.is_null() {
                    let mut p: *mut u8 = wbuf.as_mut_ptr().offset(6);
                    let mut uc_len: size_t = 0;
                    uc_len = UC_UTF16BE_encode_char(
                        ch as i32,
                        &mut p,
                        wbuf.as_mut_ptr().offset(1024).offset(-1),
                    );
                    CMap_add_bfchar(
                        tounicode_add,
                        wbuf.as_mut_ptr().offset(4),
                        2i32 as size_t,
                        wbuf.as_mut_ptr().offset(6),
                        uc_len,
                    );
                }
            }
            j = j.wrapping_add(1)
        }
        i -= 1
    }
}
unsafe extern "C" fn load_cmap12(
    mut map: *mut cmap12,
    mut GIDToCIDMap: *mut u8,
    mut gsub_vert: *mut otl_gsub,
    mut gsub_list: *mut otl_gsub,
    mut cmap: *mut CMap,
    mut tounicode_add: *mut CMap,
) {
    let mut i: u32 = 0;
    let mut ch: u32 = 0;
    let mut gid: u16 = 0;
    let mut cid: u16 = 0;
    i = 0_u32;
    while i < (*map).nGroups {
        ch = (*(*map).groups.offset(i as isize)).startCharCode;
        while ch <= (*(*map).groups.offset(i as isize)).endCharCode {
            let mut d: i32 =
                ch.wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode) as i32;
            gid = ((*(*map).groups.offset(i as isize))
                .startGlyphID
                .wrapping_add(d as u32)
                & 0xffff_u32) as u16;
            if !gsub_list.is_null() {
                otl_gsub_apply_chain(gsub_list, &mut gid);
            }
            if !gsub_vert.is_null() {
                otl_gsub_apply(gsub_vert, &mut gid);
            }
            if !GIDToCIDMap.is_null() {
                cid = ((*GIDToCIDMap.offset((2i32 * gid as i32) as isize) as i32) << 8i32
                    | *GIDToCIDMap.offset((2i32 * gid as i32 + 1i32) as isize) as i32)
                    as u16;
                if cid as i32 == 0i32 {
                    warn!("GID {} does not have corresponding CID {}.", gid, cid,);
                }
            } else {
                cid = gid
            }
            wbuf[0] = (ch >> 24i32 & 0xff_u32) as u8;
            wbuf[1] = (ch >> 16i32 & 0xff_u32) as u8;
            wbuf[2] = (ch >> 8i32 & 0xff_u32) as u8;
            wbuf[3] = (ch & 0xff_u32) as u8;
            wbuf[4] = (cid as i32 >> 8i32 & 0xffi32) as u8;
            wbuf[5] = (cid as i32 & 0xffi32) as u8;
            CMap_add_cidchar(cmap, wbuf.as_mut_ptr(), 4i32 as size_t, cid);
            if !tounicode_add.is_null() {
                let mut p: *mut u8 = wbuf.as_mut_ptr().offset(6);
                let mut uc_len: size_t = 0;
                uc_len = UC_UTF16BE_encode_char(
                    ch as i32,
                    &mut p,
                    wbuf.as_mut_ptr().offset(1024).offset(-1),
                );
                CMap_add_bfchar(
                    tounicode_add,
                    wbuf.as_mut_ptr().offset(4),
                    2i32 as size_t,
                    wbuf.as_mut_ptr().offset(6),
                    uc_len,
                );
            }
            ch = ch.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
}
/* OpenType CIDFont:
 *
 *  We don't use GID for them. OpenType cmap table is for
 *  charcode to GID mapping rather than to-CID mapping.
 */
unsafe extern "C" fn handle_CIDFont(
    mut sfont: *mut sfnt,
    mut GIDToCIDMap: *mut *mut u8,
    mut csi: *mut CIDSysInfo,
) -> i32 {
    let mut cffont: *mut cff_font = 0 as *mut cff_font; /* CID... */
    let mut offset: i32 = 0; /* card8 */
    let mut i: i32 = 0;
    let mut num_glyphs: card16 = 0;
    let mut gid: card16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut map: *mut u8 = 0 as *mut u8;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    assert!(!csi.is_null());
    offset = sfnt_find_table_pos(sfont, b"CFF \x00" as *const u8 as *const i8) as i32;
    if offset == 0i32 {
        (*csi).registry = 0 as *mut i8;
        (*csi).ordering = 0 as *mut i8;
        *GIDToCIDMap = 0 as *mut u8;
        return 0i32;
    }
    maxp = tt_read_maxp_table(sfont);
    num_glyphs = (*maxp).numGlyphs;
    free(maxp as *mut libc::c_void);
    if (num_glyphs as i32) < 1i32 {
        panic!("No glyph contained in this font...");
    }
    cffont = cff_open((*sfont).handle, offset, 0i32);
    if cffont.is_null() {
        panic!("Could not open CFF font...");
    }
    if (*cffont).flag & 1i32 << 0i32 == 0 {
        cff_close(cffont);
        (*csi).registry = 0 as *mut i8;
        (*csi).ordering = 0 as *mut i8;
        *GIDToCIDMap = 0 as *mut u8;
        return 0i32;
    }
    if cff_dict_known((*cffont).topdict, b"ROS\x00" as *const u8 as *const i8) == 0 {
        panic!("No CIDSystemInfo???");
    } else {
        let mut reg: card16 = 0;
        let mut ord: card16 = 0;
        reg = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const i8,
            0i32,
        ) as card16;
        ord = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const i8,
            1i32,
        ) as card16;
        (*csi).registry = cff_get_string(cffont, reg);
        (*csi).ordering = cff_get_string(cffont, ord);
        (*csi).supplement = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const i8,
            2i32,
        ) as i32
    }
    cff_read_charsets(cffont);
    charset = (*cffont).charsets;
    if charset.is_null() {
        panic!("No CFF charset data???");
    }
    map = new(((num_glyphs as i32 * 2i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    memset(
        map as *mut libc::c_void,
        0i32,
        (num_glyphs as i32 * 2i32) as u64,
    );
    match (*charset).format as i32 {
        0 => {
            let mut cids: *mut s_SID = 0 as *mut s_SID;
            cids = (*charset).data.glyphs;
            gid = 1i32 as card16;
            i = 0i32;
            while i < (*charset).num_entries as i32 {
                *map.offset((2i32 * gid as i32) as isize) =
                    (*cids.offset(i as isize) as i32 >> 8i32 & 0xffi32) as u8;
                *map.offset((2i32 * gid as i32 + 1i32) as isize) =
                    (*cids.offset(i as isize) as i32 & 0xffi32) as u8;
                gid = gid.wrapping_add(1);
                i += 1
            }
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            let mut cid: card16 = 0;
            let mut count: card16 = 0;
            ranges = (*charset).data.range1;
            gid = 1i32 as card16;
            i = 0i32;
            while i < (*charset).num_entries as i32 {
                cid = (*ranges.offset(i as isize)).first;
                count = ((*ranges.offset(i as isize)).n_left as i32 + 1i32) as card16;
                loop {
                    let fresh3 = count;
                    count = count.wrapping_sub(1);
                    if !(fresh3 as i32 > 0i32 && gid as i32 <= num_glyphs as i32) {
                        break;
                    }
                    *map.offset((2i32 * gid as i32) as isize) =
                        (cid as i32 >> 8i32 & 0xffi32) as u8;
                    *map.offset((2i32 * gid as i32 + 1i32) as isize) = (cid as i32 & 0xffi32) as u8;
                    gid = gid.wrapping_add(1);
                    cid = cid.wrapping_add(1)
                }
                i += 1
            }
        }
        2 => {
            let mut ranges_0: *mut cff_range2 = 0 as *mut cff_range2;
            let mut cid_0: card16 = 0;
            let mut count_0: card16 = 0;
            ranges_0 = (*charset).data.range2;
            if (*charset).num_entries as i32 == 1i32 && (*ranges_0.offset(0)).first as i32 == 1i32 {
                /* "Complete" CIDFont */
                map = mfree(map as *mut libc::c_void) as *mut u8
            } else {
                /* Not trivial mapping */
                gid = 1i32 as card16;
                i = 0i32;
                while i < (*charset).num_entries as i32 {
                    cid_0 = (*ranges_0.offset(i as isize)).first;
                    count_0 = ((*ranges_0.offset(i as isize)).n_left as i32 + 1i32) as card16;
                    loop {
                        let fresh4 = count_0;
                        count_0 = count_0.wrapping_sub(1);
                        if !(fresh4 as i32 > 0i32 && gid as i32 <= num_glyphs as i32) {
                            break;
                        }
                        *map.offset((2i32 * gid as i32) as isize) =
                            (cid_0 as i32 >> 8i32 & 0xffi32) as u8;
                        *map.offset((2i32 * gid as i32 + 1i32) as isize) =
                            (cid_0 as i32 & 0xffi32) as u8;
                        gid = gid.wrapping_add(1);
                        cid_0 = cid_0.wrapping_add(1)
                    }
                    i += 1
                }
            }
        }
        _ => {
            map = mfree(map as *mut libc::c_void) as *mut u8;
            panic!(
                "Unknown CFF charset format...: {}",
                (*charset).format as i32
            );
        }
    }
    cff_close(cffont);
    *GIDToCIDMap = map;
    1i32
}
unsafe extern "C" fn is_PUA_or_presentation(mut uni: u32) -> bool {
    /* KANGXI RADICALs are commonly double encoded. */
    return uni >= 0x2f00_u32 && uni <= 0x2fd5_u32
        || uni >= 0xe000_u32 && uni <= 0xf8ff_u32
        || uni >= 0xfb00_u32 && uni <= 0xfb4f_u32
        || uni >= 0xf0000_u32 && uni <= 0xffffd_u32
        || uni >= 0x100000_u32 && uni <= 0x10fffd_u32;
}
unsafe extern "C" fn sfnt_get_glyphname(
    mut post: *mut tt_post_table,
    mut cffont: *mut cff_font,
    mut gid: u16,
) -> *mut i8 {
    let mut name: *mut i8 = 0 as *mut i8;
    if !post.is_null() {
        name = tt_get_glyphname(post, gid)
    }
    if name.is_null() && !cffont.is_null() {
        name = cff_get_glyphname(cffont, gid)
    }
    name
}
/*
 * Substituted glyphs:
 *
 *  Mapping information stored in cmap_add.
 */
unsafe extern "C" fn handle_subst_glyphs(
    mut cmap: *mut CMap,
    mut cmap_add: *mut CMap,
    mut used_glyphs: *const i8,
    mut sfont: *mut sfnt,
    mut cffont: *mut cff_font,
) -> u16 {
    let mut count: u16 = 0;
    let mut i: u16 = 0;
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    if cmap_add.is_null() {
        post = tt_read_post_table(sfont)
    }
    count = 0_u16;
    i = 0_u16;
    while (i as i32) < 8192i32 {
        let mut j: u32 = 0;
        let mut len: size_t = 0;
        let mut inbytesleft: size_t = 0;
        let mut outbytesleft: size_t = 0;
        let mut inbuf: *const u8 = 0 as *const u8;
        let mut outbuf: *mut u8 = 0 as *mut u8;
        if !(*used_glyphs.offset(i as isize) as i32 == 0i32) {
            j = 0_u32;
            while j < 8_u32 {
                let mut gid: u16 = ((8i32 * i as i32) as u32).wrapping_add(j) as u16;
                if !(*used_glyphs.offset((gid as i32 / 8i32) as isize) as i32
                    & 1i32 << 7i32 - gid as i32 % 8i32
                    == 0)
                {
                    if cmap_add.is_null() {
                        /* try to look up Unicode values from the glyph name... */
                        let mut name: *mut i8 = 0 as *mut i8;
                        let mut unicodes: [i32; 16] = [0; 16];
                        let mut unicode_count: i32 = -1i32;
                        name = sfnt_get_glyphname(post, cffont, gid);
                        if !name.is_null() {
                            unicode_count = agl_get_unicodes(name, unicodes.as_mut_ptr(), 16i32)
                        }
                        if unicode_count == -1i32 {
                            if !name.is_null() {
                                dpx_message(
                                    b"No Unicode mapping available: GID=%u, name=%s\n\x00"
                                        as *const u8
                                        as *const i8,
                                    gid as i32,
                                    name,
                                );
                            } else {
                                info!("No Unicode mapping available: GID={}\n", gid);
                            }
                        } else {
                            /* the Unicode characters go into wbuf[2] and following, in UTF16BE */
                            /* we rely on WBUF_SIZE being more than adequate for MAX_UNICODES  */
                            let mut p: *mut u8 = wbuf.as_mut_ptr().offset(2);
                            let mut k: i32 = 0;
                            len = 0i32 as size_t;
                            k = 0i32;
                            while k < unicode_count {
                                len = (len as u64).wrapping_add(UC_UTF16BE_encode_char(
                                    unicodes[k as usize],
                                    &mut p,
                                    wbuf.as_mut_ptr().offset(1024),
                                )) as size_t as size_t;
                                k += 1
                            }
                            wbuf[0] = (gid as i32 >> 8i32 & 0xffi32) as u8;
                            wbuf[1] = (gid as i32 & 0xffi32) as u8;
                            CMap_add_bfchar(
                                cmap,
                                wbuf.as_mut_ptr(),
                                2i32 as size_t,
                                wbuf.as_mut_ptr().offset(2),
                                len,
                            );
                        }
                        free(name as *mut libc::c_void);
                    } else {
                        wbuf[0] = (gid as i32 >> 8i32 & 0xffi32) as u8;
                        wbuf[1] = (gid as i32 & 0xffi32) as u8;
                        inbuf = wbuf.as_mut_ptr();
                        inbytesleft = 2i32 as size_t;
                        outbuf = wbuf.as_mut_ptr().offset(2);
                        outbytesleft = (1024i32 - 2i32) as size_t;
                        CMap_decode(
                            cmap_add,
                            &mut inbuf,
                            &mut inbytesleft,
                            &mut outbuf,
                            &mut outbytesleft,
                        );
                        if inbytesleft != 0i32 as u64 {
                            warn!("CMap conversion failed...");
                        } else {
                            len = ((1024i32 - 2i32) as u64).wrapping_sub(outbytesleft);
                            CMap_add_bfchar(
                                cmap,
                                wbuf.as_mut_ptr(),
                                2i32 as size_t,
                                wbuf.as_mut_ptr().offset(2),
                                len,
                            );
                            count = count.wrapping_add(1);
                            if verbose > 0i32 {
                                let mut _i: size_t = 0;
                                info!(
                                    "otf_cmap>> Additional ToUnicode mapping: <{:04X}> <",
                                    gid as i32,
                                );
                                _i = 0i32 as size_t;
                                while _i < len {
                                    info!(
                                        "{:02X}",
                                        wbuf[(2i32 as u64).wrapping_add(_i) as usize] as i32,
                                    );
                                    _i = _i.wrapping_add(1)
                                }
                                info!(">\n");
                            }
                        }
                    }
                }
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    if !post.is_null() {
        tt_release_post_table(post);
    }
    count
}
unsafe extern "C" fn prepare_CIDFont_from_sfnt(mut sfont: *mut sfnt) -> *mut cff_font {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut offset: u32 = 0_u32;
    if (*sfont).type_0 != 1i32 << 2i32 || sfnt_read_table_directory(sfont, 0_u32) < 0i32 || {
        offset = sfnt_find_table_pos(sfont, b"CFF \x00" as *const u8 as *const i8);
        offset == 0_u32
    } {
        return 0 as *mut cff_font;
    }
    cffont = cff_open((*sfont).handle, offset as i32, 0i32);
    if cffont.is_null() {
        return 0 as *mut cff_font;
    }
    cff_read_charsets(cffont);
    cffont
}
unsafe extern "C" fn add_to_cmap_if_used(
    mut cmap: *mut CMap,
    mut cffont: *mut cff_font,
    mut used_chars: *mut i8,
    mut gid: u16,
    mut ch: u32,
) -> u16 {
    let mut count: u16 = 0_u16;
    let mut cid: u16 = (if !cffont.is_null() {
        cff_charsets_lookup_inverse(cffont, gid) as i32
    } else {
        gid as i32
    }) as u16;
    /* Skip PUA characters and alphabetic presentation forms, allowing
     * handle_subst_glyphs() as it might find better mapping. Fixes the
     * mapping of ligatures encoded in PUA in fonts like Linux Libertine
     * and old Adobe fonts.
     */
    if *used_chars.offset((cid as i32 / 8i32) as isize) as i32 & 1i32 << 7i32 - cid as i32 % 8i32
        != 0
        && !is_PUA_or_presentation(ch)
    {
        let mut len: i32 = 0;
        let mut p: *mut u8 = wbuf.as_mut_ptr().offset(2);
        count = count.wrapping_add(1);
        wbuf[0] = (cid as i32 >> 8i32 & 0xffi32) as u8;
        wbuf[1] = (cid as i32 & 0xffi32) as u8;
        len = UC_UTF16BE_encode_char(ch as i32, &mut p, wbuf.as_mut_ptr().offset(1024)) as i32;
        CMap_add_bfchar(
            cmap,
            wbuf.as_mut_ptr(),
            2i32 as size_t,
            wbuf.as_mut_ptr().offset(2),
            len as size_t,
        );
        /* Avoid duplicate entry
         * There are problem when two Unicode code is mapped to
         * single glyph...
         */
        let ref mut fresh5 = *used_chars.offset((cid as i32 / 8i32) as isize);
        *fresh5 = (*fresh5 as i32 & !(1i32 << 7i32 - cid as i32 % 8i32)) as i8
    }
    count
}
unsafe extern "C" fn create_ToUnicode_cmap4(
    mut cmap: *mut CMap,
    mut map: *mut cmap4,
    mut used_chars: *mut i8,
    mut cffont: *mut cff_font,
) -> u16 {
    let mut count: u16 = 0_u16;
    let mut segCount: u16 = ((*map).segCountX2 as i32 / 2i32) as u16;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0_u16;
    while (i as i32) < segCount as i32 {
        let mut c0: u16 = *(*map).startCount.offset(i as isize);
        let mut c1: u16 = *(*map).endCount.offset(i as isize);
        let mut d: u16 = (*(*map).idRangeOffset.offset(i as isize) as i32 / 2i32
            - (segCount as i32 - i as i32)) as u16;
        j = 0_u16;
        while j as i32 <= c1 as i32 - c0 as i32 {
            let mut ch: u16 = (c0 as i32 + j as i32) as u16;
            let mut gid: u16 = 0;
            if *(*map).idRangeOffset.offset(i as isize) as i32 == 0i32 {
                gid = (ch as i32 + *(*map).idDelta.offset(i as isize) as i32 & 0xffffi32) as u16
            } else if c0 as i32 == 0xffffi32
                && c1 as i32 == 0xffffi32
                && *(*map).idRangeOffset.offset(i as isize) as i32 == 0xffffi32
            {
                /* this is for protection against some old broken fonts... */
                gid = 0_u16
            } else {
                gid = (*(*map)
                    .glyphIndexArray
                    .offset((j as i32 + d as i32) as isize) as i32
                    + *(*map).idDelta.offset(i as isize) as i32
                    & 0xffffi32) as u16
            }
            count = (count as i32
                + add_to_cmap_if_used(cmap, cffont, used_chars, gid, ch as u32) as i32)
                as u16;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    count
}
unsafe extern "C" fn create_ToUnicode_cmap12(
    mut cmap: *mut CMap,
    mut map: *mut cmap12,
    mut used_chars: *mut i8,
    mut cffont: *mut cff_font,
) -> u16 {
    let mut i: u32 = 0;
    let mut ch: u32 = 0;
    let mut count: u32 = 0_u32;
    i = 0_u32;
    while i < (*map).nGroups {
        ch = (*(*map).groups.offset(i as isize)).startCharCode;
        while ch <= (*(*map).groups.offset(i as isize)).endCharCode {
            let mut d: i32 =
                ch.wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode) as i32;
            let mut gid: u16 = ((*(*map).groups.offset(i as isize))
                .startGlyphID
                .wrapping_add(d as u32)
                & 0xffff_u32) as u16;
            count = (count as u32)
                .wrapping_add(add_to_cmap_if_used(cmap, cffont, used_chars, gid, ch) as u32)
                as u32 as u32;
            ch = ch.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    count as u16
}
unsafe extern "C" fn create_ToUnicode_cmap(
    mut ttcmap: *mut tt_cmap,
    mut cmap_name: *const i8,
    mut cmap_add: *mut CMap,
    mut used_chars: *const i8,
    mut sfont: *mut sfnt,
    mut code_to_cid_cmap: *mut CMap,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut count: u16 = 0_u16;
    let mut cffont: *mut cff_font = prepare_CIDFont_from_sfnt(sfont);
    let mut is_cidfont: i8 = (!cffont.is_null() && (*cffont).flag & 1i32 << 0i32 != 0) as i32 as i8;
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_wmode(cmap, 0i32);
    CMap_set_type(cmap, 2i32);
    CMap_set_CIDSysInfo(cmap, &mut CSI_UNICODE);
    CMap_add_codespacerange(
        cmap,
        srange_min.as_mut_ptr(),
        srange_max.as_mut_ptr(),
        2i32 as size_t,
    );
    /* cmap_add here stores information about all unencoded glyphs which can be
     * accessed only through OT Layout GSUB table.
     */
    if !code_to_cid_cmap.is_null()
        && !cffont.is_null()
        && is_cidfont as i32 != 0
        && cmap_add.is_null()
    {
        let mut i: u16 = 0;
        i = 0_u16;
        while (i as i32) < 8192i32 {
            let mut j: i32 = 0;
            if !(*used_chars.offset(i as isize) as i32 == 0i32) {
                j = 0i32;
                while j < 8i32 {
                    let mut cid: u16 = (8i32 * i as i32 + j) as u16;
                    let mut ch: i32 = 0;
                    if !(*used_chars.offset((cid as i32 / 8i32) as isize) as i32
                        & 1i32 << 7i32 - cid as i32 % 8i32
                        == 0)
                    {
                        ch = CMap_reverse_decode(code_to_cid_cmap, cid);
                        if ch >= 0i32 {
                            let mut len: i32 = 0;
                            let mut p: *mut u8 = wbuf.as_mut_ptr().offset(2);
                            wbuf[0] = (cid as i32 >> 8i32 & 0xffi32) as u8;
                            wbuf[1] = (cid as i32 & 0xffi32) as u8;
                            len = UC_UTF16BE_encode_char(ch, &mut p, wbuf.as_mut_ptr().offset(1024))
                                as i32;
                            CMap_add_bfchar(
                                cmap,
                                wbuf.as_mut_ptr(),
                                2i32 as size_t,
                                wbuf.as_mut_ptr().offset(2),
                                len as size_t,
                            );
                            count = count.wrapping_add(1)
                        }
                    }
                    j += 1
                }
            }
            i = i.wrapping_add(1)
        }
    } else {
        let mut used_chars_copy: [i8; 8192] = [0; 8192];
        memcpy(
            used_chars_copy.as_mut_ptr() as *mut libc::c_void,
            used_chars as *const libc::c_void,
            8192i32 as u64,
        );
        /* For create_ToUnicode_cmap{4,12}(), cffont is for GID -> CID lookup,
         * so it is only needed for CID fonts. */
        match (*ttcmap).format as i32 {
            4 => {
                count = create_ToUnicode_cmap4(
                    cmap,
                    (*ttcmap).map as *mut cmap4,
                    used_chars_copy.as_mut_ptr(),
                    if is_cidfont as i32 != 0 {
                        cffont
                    } else {
                        0 as *mut cff_font
                    },
                )
            }
            12 => {
                count = create_ToUnicode_cmap12(
                    cmap,
                    (*ttcmap).map as *mut cmap12,
                    used_chars_copy.as_mut_ptr(),
                    if is_cidfont as i32 != 0 {
                        cffont
                    } else {
                        0 as *mut cff_font
                    },
                )
            }
            _ => {}
        }
        /* For handle_subst_glyphs(), cffont is for GID -> glyph name lookup, so
         * it is only needed for non-CID fonts. */
        count = (count as i32
            + handle_subst_glyphs(
                cmap,
                cmap_add,
                used_chars_copy.as_mut_ptr(),
                sfont,
                if is_cidfont as i32 != 0 {
                    0 as *mut cff_font
                } else {
                    cffont
                },
            ) as i32) as u16
    }
    if (count as i32) < 1i32 {
        stream = 0 as *mut pdf_obj
    } else {
        stream = CMap_create_stream(cmap)
    }
    CMap_release(cmap);
    if !cffont.is_null() {
        cff_close(cffont);
    }
    stream
}
static mut cmap_plat_encs: [cmap_plat_enc_rec; 5] = [
    {
        let mut init = cmap_plat_enc_rec {
            platform: 3_i16,
            encoding: 10_i16,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 0_i16,
            encoding: 3_i16,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 0_i16,
            encoding: 0_i16,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 3_i16,
            encoding: 1_i16,
        };
        init
    },
    {
        let mut init = cmap_plat_enc_rec {
            platform: 0_i16,
            encoding: 1_i16,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn otf_create_ToUnicode_stream(
    mut font_name: *const i8,
    mut ttc_index: i32,
    mut used_chars: *const i8,
    mut cmap_id: i32,
) -> *mut pdf_obj {
    let mut cmap_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut res_id: i32 = 0;
    let mut cmap_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap_add: *mut CMap = 0 as *mut CMap;
    let mut code_to_cid_cmap: *mut CMap = 0 as *mut CMap;
    let mut cmap_add_id: i32 = 0;
    let mut ttcmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut normalized_font_name: *mut i8 = 0 as *mut i8;
    let mut cmap_name: *mut i8 = 0 as *mut i8;
    let mut cmap_add_name: *mut i8 = 0 as *mut i8;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: u32 = 0_u32;
    let mut cmap_type: i32 = 0;
    let mut i: size_t = 0;
    /* replace slash in map name with dash to make the output cmap name valid,
     * happens when XeTeX embeds full font path
     * https://sourceforge.net/p/xetex/bugs/52/
     */
    normalized_font_name = new((strlen(font_name).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8; /* many warnings without this... */
    strcpy(normalized_font_name, font_name);
    i = 0i32 as size_t;
    while i < strlen(font_name) {
        if *normalized_font_name.offset(i as isize) as i32 == '/' as i32 {
            *normalized_font_name.offset(i as isize) = '-' as i32 as i8
        }
        i = i.wrapping_add(1)
    }
    cmap_name = new((strlen(font_name)
        .wrapping_add(strlen(b"-UTF16\x00" as *const u8 as *const i8))
        .wrapping_add(5i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    sprintf(
        cmap_name,
        b"%s,%03d-UTF16\x00" as *const u8 as *const i8,
        normalized_font_name,
        ttc_index,
    );
    free(normalized_font_name as *mut libc::c_void);
    res_id = pdf_findresource(b"CMap\x00" as *const u8 as *const i8, cmap_name);
    if res_id >= 0i32 {
        free(cmap_name as *mut libc::c_void);
        cmap_ref = pdf_get_resource_reference(res_id);
        return cmap_ref;
    }
    if verbose > 0i32 {
        info!("\n");
        dpx_message(
            b"otf_cmap>> Creating ToUnicode CMap for \"%s\"...\n\x00" as *const u8 as *const i8,
            font_name,
        );
    }
    handle = dpx_open_truetype_file(font_name);
    if !handle.is_null() || {
        handle = dpx_open_opentype_file(font_name);
        !handle.is_null()
    } {
        sfont = sfnt_open(handle)
    } else {
        handle = dpx_open_dfont_file(font_name);
        if !handle.is_null() {
            sfont = dfont_open(handle, ttc_index)
        } else {
            free(cmap_name as *mut libc::c_void);
            return 0 as *mut pdf_obj;
        }
    }
    if sfont.is_null() {
        _tt_abort(
            b"Could not open OpenType/TrueType font file \"%s\"\x00" as *const u8 as *const i8,
            font_name,
        );
    }
    match (*sfont).type_0 {
        256 => offset = (*sfont).offset,
        16 => {
            offset = ttc_read_offset(sfont, ttc_index);
            if offset == 0_u32 {
                panic!("Invalid TTC index");
            }
        }
        _ => offset = 0_u32,
    }
    if sfnt_read_table_directory(sfont, offset) < 0i32 {
        panic!("Could not read OpenType/TrueType table directory.");
    }
    code_to_cid_cmap = CMap_cache_get(cmap_id);
    cmap_type = CMap_get_type(code_to_cid_cmap);
    if cmap_type != 1i32 {
        code_to_cid_cmap = 0 as *mut CMap
    }
    cmap_add_name = new((strlen(font_name)
        .wrapping_add(strlen(b",000-UCS32-Add\x00" as *const u8 as *const i8))
        .wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    sprintf(
        cmap_add_name,
        b"%s,%03d-UCS32-Add\x00" as *const u8 as *const i8,
        font_name,
        ttc_index,
    );
    cmap_add_id = CMap_cache_find(cmap_add_name);
    free(cmap_add_name as *mut libc::c_void);
    if cmap_add_id < 0i32 {
        cmap_add = 0 as *mut CMap
    } else {
        cmap_add = CMap_cache_get(cmap_add_id)
    }
    CMap_set_silent(1i32);
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[cmap_plat_enc_rec; 5]>() as u64)
            .wrapping_div(::std::mem::size_of::<cmap_plat_enc_rec>() as u64)
    {
        ttcmap = tt_cmap_read(
            sfont,
            cmap_plat_encs[i as usize].platform as u16,
            cmap_plat_encs[i as usize].encoding as u16,
        );
        if !ttcmap.is_null() {
            if (*ttcmap).format as i32 == 4i32 || (*ttcmap).format as i32 == 12i32 {
                cmap_obj = create_ToUnicode_cmap(
                    ttcmap,
                    cmap_name,
                    cmap_add,
                    used_chars,
                    sfont,
                    code_to_cid_cmap,
                );
                break;
            }
        }
        i = i.wrapping_add(1)
    }
    if cmap_obj.is_null() {
        warn!("Unable to read OpenType/TrueType Unicode cmap table.");
    }
    tt_cmap_release(ttcmap);
    CMap_set_silent(0i32);
    if !cmap_obj.is_null() {
        res_id = pdf_defineresource(
            b"CMap\x00" as *const u8 as *const i8,
            cmap_name,
            cmap_obj,
            1i32,
        );
        cmap_ref = pdf_get_resource_reference(res_id)
    } else {
        cmap_ref = 0 as *mut pdf_obj
    }
    free(cmap_name as *mut libc::c_void);
    sfnt_close(sfont);
    if !handle.is_null() {
        ttstub_input_close(handle);
    }
    cmap_ref
}
unsafe extern "C" fn load_base_CMap(
    mut cmap_name: *const i8,
    mut tounicode_add: *mut CMap,
    mut wmode: i32,
    mut csi: *mut CIDSysInfo,
    mut GIDToCIDMap: *mut u8,
    mut gsub_vert: *mut otl_gsub,
    mut gsub_list: *mut otl_gsub,
    mut ttcmap: *mut tt_cmap,
) -> i32 {
    let mut cmap_id: i32 = 0;
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id < 0i32 {
        let mut cmap: *mut CMap = 0 as *mut CMap;
        cmap = CMap_new();
        CMap_set_name(cmap, cmap_name);
        CMap_set_type(cmap, 1i32);
        CMap_set_wmode(cmap, wmode);
        CMap_add_codespacerange(
            cmap,
            lrange_min.as_mut_ptr(),
            lrange_max.as_mut_ptr(),
            4i32 as size_t,
        );
        if !csi.is_null() {
            /* CID */
            CMap_set_CIDSysInfo(cmap, csi);
        } else {
            CMap_set_CIDSysInfo(cmap, &mut CSI_IDENTITY);
        }
        if (*ttcmap).format as i32 == 12i32 {
            load_cmap12(
                (*ttcmap).map as *mut cmap12,
                GIDToCIDMap,
                gsub_vert,
                gsub_list,
                cmap,
                tounicode_add,
            );
        } else if (*ttcmap).format as i32 == 4i32 {
            load_cmap4(
                (*ttcmap).map as *mut cmap4,
                GIDToCIDMap,
                gsub_vert,
                gsub_list,
                cmap,
                tounicode_add,
            );
        }
        cmap_id = CMap_cache_add(cmap)
    }
    cmap_id
}
/* TrueType cmap table */
/* or version, only for Mac */
/* Paltform ID */
/* Platform-specific encoding ID */
/* Windows */
/* Mac */
/* Indirect reference */
/* CMap ID */
#[no_mangle]
pub unsafe extern "C" fn otf_load_Unicode_CMap(
    mut map_name: *const i8,
    mut ttc_index: i32,
    mut otl_tags: *const i8,
    mut wmode: i32,
) -> i32 {
    let mut cmap_id: i32 = -1i32;
    /* Additional ToUnicode mappings required by OTL GSUB substitusion */
    let mut tounicode_add_id: i32 = -1i32;
    let mut tounicode_add: *mut CMap = 0 as *mut CMap;
    let mut tounicode_add_name: *mut i8 = 0 as *mut i8;
    let mut is_cidfont: i32 = 0i32;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut offset: u32 = 0_u32;
    let mut base_name: *mut i8 = 0 as *mut i8;
    let mut cmap_name: *mut i8 = 0 as *mut i8;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut gsub_vert: *mut otl_gsub = 0 as *mut otl_gsub;
    let mut gsub_list: *mut otl_gsub = 0 as *mut otl_gsub;
    let mut ttcmap: *mut tt_cmap = 0 as *mut tt_cmap;
    let mut csi: CIDSysInfo = {
        let mut init = CIDSysInfo {
            registry: 0 as *mut i8,
            ordering: 0 as *mut i8,
            supplement: 0i32,
        };
        init
    };
    let mut GIDToCIDMap: *mut u8 = 0 as *mut u8;
    if map_name.is_null() {
        return -1i32;
    }
    if ttc_index > 999i32 || ttc_index < 0i32 {
        return -1i32;
        /* Sorry for this... */
    }
    handle = dpx_open_truetype_file(map_name) as *mut rust_input_handle_t;
    if handle.is_null() {
        handle = dpx_open_opentype_file(map_name) as *mut rust_input_handle_t
    }
    if handle.is_null() {
        handle = dpx_open_dfont_file(map_name) as *mut rust_input_handle_t;
        if handle.is_null() {
            return -1i32;
        }
        sfont = dfont_open(handle as rust_input_handle_t, ttc_index)
    } else {
        sfont = sfnt_open(handle as rust_input_handle_t)
    }
    if sfont.is_null() {
        _tt_abort(
            b"Could not open OpenType/TrueType/dfont font file \"%s\"\x00" as *const u8
                as *const i8,
            map_name,
        );
    }
    match (*sfont).type_0 {
        16 => {
            offset = ttc_read_offset(sfont, ttc_index);
            if offset == 0_u32 {
                panic!("Invalid TTC index");
            }
        }
        1 | 4 => offset = 0_u32,
        256 => offset = (*sfont).offset,
        _ => {
            _tt_abort(
                b"Not a OpenType/TrueType/TTC font?: %s\x00" as *const u8 as *const i8,
                map_name,
            );
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0i32 {
        panic!("Could not read OpenType/TrueType table directory.");
    }
    base_name = new((strlen(map_name)
        .wrapping_add(strlen(b"-UCS4-H\x00" as *const u8 as *const i8))
        .wrapping_add(5i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    if wmode != 0 {
        sprintf(
            base_name,
            b"%s,%03d-UCS4-V\x00" as *const u8 as *const i8,
            map_name,
            ttc_index,
        );
    } else {
        sprintf(
            base_name,
            b"%s,%03d-UCS4-H\x00" as *const u8 as *const i8,
            map_name,
            ttc_index,
        );
    }
    if !otl_tags.is_null() {
        cmap_name = new((strlen(map_name)
            .wrapping_add(strlen(otl_tags))
            .wrapping_add(strlen(b"-UCS4-H\x00" as *const u8 as *const i8))
            .wrapping_add(6i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        if wmode != 0 {
            sprintf(
                cmap_name,
                b"%s,%03d,%s-UCS4-V\x00" as *const u8 as *const i8,
                map_name,
                ttc_index,
                otl_tags,
            );
        } else {
            sprintf(
                cmap_name,
                b"%s,%03d,%s-UCS4-H\x00" as *const u8 as *const i8,
                map_name,
                ttc_index,
                otl_tags,
            );
        }
        /* tounicode_add here is later refered by otf_create_ToUnicode_stream()
         * for finding additional CID to Unicode mapping entries required by
         * OTL gsub substitution.
         */
        tounicode_add_name = new((strlen(map_name)
            .wrapping_add(strlen(b",000-UCS32-Add\x00" as *const u8 as *const i8))
            .wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8; /* Microsoft UCS4 */
        sprintf(
            tounicode_add_name,
            b"%s,%03d-UCS32-Add\x00" as *const u8 as *const i8,
            map_name,
            ttc_index,
        ); /* Microsoft UCS2 */
        tounicode_add_id = CMap_cache_find(tounicode_add_name); /* Unicode 2.0 or later */
        if tounicode_add_id >= 0i32 {
            tounicode_add = CMap_cache_get(tounicode_add_id)
        } else {
            tounicode_add = CMap_new();
            CMap_set_name(tounicode_add, tounicode_add_name);
            CMap_set_type(tounicode_add, 2i32);
            CMap_set_wmode(tounicode_add, 0i32);
            CMap_add_codespacerange(
                tounicode_add,
                srange_min.as_mut_ptr(),
                srange_max.as_mut_ptr(),
                2i32 as size_t,
            );
            CMap_set_CIDSysInfo(tounicode_add, &mut CSI_UNICODE);
            CMap_add_bfchar(
                tounicode_add,
                srange_min.as_mut_ptr(),
                2i32 as size_t,
                srange_max.as_mut_ptr(),
                2i32 as size_t,
            );
            tounicode_add_id = CMap_cache_add(tounicode_add)
        }
        free(tounicode_add_name as *mut libc::c_void);
    } else {
        cmap_name = new((strlen(base_name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        strcpy(cmap_name, base_name);
    }
    if (*sfont).type_0 == 1i32 << 2i32 {
        is_cidfont = handle_CIDFont(sfont, &mut GIDToCIDMap, &mut csi)
    } else {
        is_cidfont = 0i32
    }
    if verbose > 0i32 {
        info!("\n");
        dpx_message(
            b"otf_cmap>> Unicode charmap for font=\"%s\" layout=\"%s\"\n\x00" as *const u8
                as *const i8,
            map_name,
            if !otl_tags.is_null() {
                otl_tags
            } else {
                b"none\x00" as *const u8 as *const i8
            },
        );
    }
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0i32 {
        free(cmap_name as *mut libc::c_void);
        free(base_name as *mut libc::c_void);
        free(GIDToCIDMap as *mut libc::c_void);
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        if verbose > 0i32 {
            info!("otf_cmap>> Found at cmap_id={}.\n", cmap_id);
        }
        return cmap_id;
    }
    ttcmap = tt_cmap_read(sfont, 3_u16, 10_u16);
    if ttcmap.is_null() {
        ttcmap = tt_cmap_read(sfont, 3_u16, 1_u16);
        if ttcmap.is_null() {
            ttcmap = tt_cmap_read(sfont, 0_u16, 3_u16);
            if ttcmap.is_null() {
                panic!("Unable to read OpenType/TrueType Unicode cmap table.");
            }
        }
    }
    if wmode == 1i32 {
        gsub_vert = otl_gsub_new();
        if otl_gsub_add_feat(
            gsub_vert,
            b"*\x00" as *const u8 as *const i8,
            b"*\x00" as *const u8 as *const i8,
            b"vrt2\x00" as *const u8 as *const i8,
            sfont,
        ) < 0i32
        {
            if otl_gsub_add_feat(
                gsub_vert,
                b"*\x00" as *const u8 as *const i8,
                b"*\x00" as *const u8 as *const i8,
                b"vert\x00" as *const u8 as *const i8,
                sfont,
            ) < 0i32
            {
                warn!("GSUB feature vrt2/vert not found.");
                otl_gsub_release(gsub_vert);
                gsub_vert = 0 as *mut otl_gsub
            } else {
                otl_gsub_select(
                    gsub_vert,
                    b"*\x00" as *const u8 as *const i8,
                    b"*\x00" as *const u8 as *const i8,
                    b"vert\x00" as *const u8 as *const i8,
                );
            }
        } else {
            otl_gsub_select(
                gsub_vert,
                b"*\x00" as *const u8 as *const i8,
                b"*\x00" as *const u8 as *const i8,
                b"vrt2\x00" as *const u8 as *const i8,
            );
        }
    } else {
        gsub_vert = 0 as *mut otl_gsub
    }
    if !otl_tags.is_null() {
        gsub_list = otl_gsub_new();
        if otl_gsub_add_feat_list(gsub_list, otl_tags, sfont) < 0i32 {
            dpx_warning(
                b"Reading GSUB feature table(s) failed for \"%s\"\x00" as *const u8 as *const i8,
                otl_tags,
            );
        } else {
            otl_gsub_set_chain(gsub_list, otl_tags);
        }
    } else {
        gsub_list = 0 as *mut otl_gsub
    }
    cmap_id = load_base_CMap(
        cmap_name,
        tounicode_add,
        wmode,
        if is_cidfont != 0 {
            &mut csi
        } else {
            0 as *mut CIDSysInfo
        },
        GIDToCIDMap,
        gsub_vert,
        gsub_list,
        ttcmap,
    );
    if cmap_id < 0i32 {
        panic!("Failed to read OpenType/TrueType cmap table.");
    }
    if !gsub_vert.is_null() {
        otl_gsub_release(gsub_vert);
    }
    gsub_vert = 0 as *mut otl_gsub;
    if !gsub_list.is_null() {
        otl_gsub_release(gsub_list);
    }
    gsub_list = 0 as *mut otl_gsub;
    free(cmap_name as *mut libc::c_void);
    free(base_name as *mut libc::c_void);
    free(GIDToCIDMap as *mut libc::c_void);
    if is_cidfont != 0 {
        free(csi.registry as *mut libc::c_void);
        free(csi.ordering as *mut libc::c_void);
    }
    tt_cmap_release(ttcmap);
    sfnt_close(sfont);
    ttstub_input_close(handle as rust_input_handle_t);
    cmap_id
}
