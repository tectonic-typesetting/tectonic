#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::warn;

use crate::{ttstub_input_read, ttstub_input_seek};
extern "C" {
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_get_signed_byte(handle: rust_input_handle_t) -> i8;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> i16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn put_big_endian(s: *mut libc::c_void, q: i32, n: i32) -> i32;
    #[no_mangle]
    fn sfnt_find_table_len(sfont: *mut sfnt, tag: *const i8) -> u32;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> u32;
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const i8) -> u32;
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
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type Fixed = u32;
pub type FWord = i16;
pub type uFWord = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub num_kept_tables: u16,
    pub flags: *mut i8,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: i32,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_head_table {
    pub version: Fixed,
    pub fontRevision: Fixed,
    pub checkSumAdjustment: u32,
    pub magicNumber: u32,
    pub flags: u16,
    pub unitsPerEm: u16,
    pub created: [u8; 8],
    pub modified: [u8; 8],
    pub xMin: FWord,
    pub yMin: FWord,
    pub xMax: FWord,
    pub yMax: FWord,
    pub macStyle: u16,
    pub lowestRecPPEM: u16,
    pub fontDirectionHint: i16,
    pub indexToLocFormat: i16,
    pub glyphDataFormat: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_hhea_table {
    pub version: Fixed,
    pub ascent: FWord,
    pub descent: FWord,
    pub lineGap: FWord,
    pub advanceWidthMax: uFWord,
    pub minLeftSideBearing: FWord,
    pub minRightSideBearing: FWord,
    pub xMaxExtent: FWord,
    pub caretSlopeRise: i16,
    pub caretSlopeRun: i16,
    pub caretOffset: FWord,
    pub reserved: [i16; 4],
    pub metricDataFormat: i16,
    pub numOfLongHorMetrics: u16,
    pub numOfExSideBearings: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vhea_table {
    pub version: Fixed,
    pub vertTypoAscender: i16,
    pub vertTypoDescender: i16,
    pub vertTypoLineGap: i16,
    pub advanceHeightMax: i16,
    pub minTopSideBearing: i16,
    pub minBottomSideBearing: i16,
    pub yMaxExtent: i16,
    pub caretSlopeRise: i16,
    pub caretSlopeRun: i16,
    pub caretOffset: i16,
    pub reserved: [i16; 4],
    pub metricDataFormat: i16,
    pub numOfLongVerMetrics: u16,
    pub numOfExSideBearings: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_maxp_table {
    pub version: Fixed,
    pub numGlyphs: u16,
    pub maxPoints: u16,
    pub maxContours: u16,
    pub maxComponentPoints: u16,
    pub maxComponentContours: u16,
    pub maxZones: u16,
    pub maxTwilightPoints: u16,
    pub maxStorage: u16,
    pub maxFunctionDefs: u16,
    pub maxInstructionDefs: u16,
    pub maxStackElements: u16,
    pub maxSizeOfInstructions: u16,
    pub maxComponentElements: u16,
    pub maxComponentDepth: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_os2__table {
    pub version: u16,
    pub xAvgCharWidth: i16,
    pub usWeightClass: u16,
    pub usWidthClass: u16,
    pub fsType: i16,
    pub ySubscriptXSize: i16,
    pub ySubscriptYSize: i16,
    pub ySubscriptXOffset: i16,
    pub ySubscriptYOffset: i16,
    pub ySuperscriptXSize: i16,
    pub ySuperscriptYSize: i16,
    pub ySuperscriptXOffset: i16,
    pub ySuperscriptYOffset: i16,
    pub yStrikeoutSize: i16,
    pub yStrikeoutPosition: i16,
    pub sFamilyClass: i16,
    pub panose: [u8; 10],
    pub ulUnicodeRange1: u32,
    pub ulUnicodeRange2: u32,
    pub ulUnicodeRange3: u32,
    pub ulUnicodeRange4: u32,
    pub achVendID: [i8; 4],
    pub fsSelection: u16,
    pub usFirstCharIndex: u16,
    pub usLastCharIndex: u16,
    pub sTypoAscender: i16,
    pub sTypoDescender: i16,
    pub sTypoLineGap: i16,
    pub usWinAscent: u16,
    pub usWinDescent: u16,
    pub ulCodePageRange1: u32,
    pub ulCodePageRange2: u32,
    pub sxHeight: i16,
    pub sCapHeight: i16,
    pub usDefaultChar: u16,
    pub usBreakChar: u16,
    pub usMaxContext: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vertOriginYMetrics {
    pub glyphIndex: u16,
    pub vertOriginY: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_VORG_table {
    pub defaultVertOriginY: i16,
    pub numVertOriginYMetrics: u16,
    pub vertOriginYMetrics: *mut tt_vertOriginYMetrics,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: u16,
    pub sideBearing: i16,
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
  tables contains information refered by other tables
  maxp->numGlyphs, etc --> loca, etc
  hhea->numOfLongHorMetrics --> hmtx
  head->indexToLocFormat --> loca
  head->glyphDataFormat --> glyf
*/
#[no_mangle]
pub unsafe extern "C" fn tt_pack_head_table(mut table: *mut tt_head_table) -> *mut i8 {
    let mut i: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut data: *mut i8 = 0 as *mut i8;
    if table.is_null() {
        _tt_abort(b"passed NULL pointer\n\x00" as *const u8 as *const i8);
    }
    data = new((54u64 as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    p = data;
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).version as i32, 4i32) as isize);
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).fontRevision as i32, 4i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).checkSumAdjustment as i32,
        4i32,
    ) as isize);
    p = p
        .offset(put_big_endian(p as *mut libc::c_void, (*table).magicNumber as i32, 4i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).flags as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).unitsPerEm as i32, 2i32) as isize);
    i = 0i32;
    while i < 8i32 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = (*table).created[i as usize] as i8;
        i += 1
    }
    i = 0i32;
    while i < 8i32 {
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = (*table).modified[i as usize] as i8;
        i += 1
    }
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).xMin as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).yMin as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).xMax as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).yMax as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).macStyle as i32, 2i32) as isize);
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).lowestRecPPEM as i32, 2i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).fontDirectionHint as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).indexToLocFormat as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).glyphDataFormat as i32,
        2i32,
    ) as isize);
    data
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_head_table(mut sfont: *mut sfnt) -> *mut tt_head_table {
    let mut i: i32 = 0;
    let mut table: *mut tt_head_table = new((1_u64)
        .wrapping_mul(::std::mem::size_of::<tt_head_table>() as u64)
        as u32) as *mut tt_head_table;
    sfnt_locate_table(sfont, b"head\x00" as *const u8 as *const i8);
    (*table).version = tt_get_unsigned_quad((*sfont).handle);
    (*table).fontRevision = tt_get_unsigned_quad((*sfont).handle);
    (*table).checkSumAdjustment = tt_get_unsigned_quad((*sfont).handle);
    (*table).magicNumber = tt_get_unsigned_quad((*sfont).handle);
    (*table).flags = tt_get_unsigned_pair((*sfont).handle);
    (*table).unitsPerEm = tt_get_unsigned_pair((*sfont).handle);
    i = 0i32;
    while i < 8i32 {
        (*table).created[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i += 1
    }
    i = 0i32;
    while i < 8i32 {
        (*table).modified[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i += 1
    }
    (*table).xMin = tt_get_signed_pair((*sfont).handle);
    (*table).yMin = tt_get_signed_pair((*sfont).handle);
    (*table).xMax = tt_get_signed_pair((*sfont).handle);
    (*table).yMax = tt_get_signed_pair((*sfont).handle);
    (*table).macStyle = tt_get_signed_pair((*sfont).handle) as u16;
    (*table).lowestRecPPEM = tt_get_signed_pair((*sfont).handle) as u16;
    (*table).fontDirectionHint = tt_get_signed_pair((*sfont).handle);
    (*table).indexToLocFormat = tt_get_signed_pair((*sfont).handle);
    (*table).glyphDataFormat = tt_get_signed_pair((*sfont).handle);
    table
}
#[no_mangle]
pub unsafe extern "C" fn tt_pack_maxp_table(mut table: *mut tt_maxp_table) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut data: *mut i8 = 0 as *mut i8;
    data = new((32u64 as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    p = data;
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).version as i32, 4i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).numGlyphs as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).maxPoints as i32, 2i32) as isize);
    p = p
        .offset(put_big_endian(p as *mut libc::c_void, (*table).maxContours as i32, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentPoints as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentContours as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).maxZones as i32, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxTwilightPoints as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).maxStorage as i32, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxFunctionDefs as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxInstructionDefs as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxStackElements as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxSizeOfInstructions as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentElements as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentDepth as i32,
        2i32,
    ) as isize);
    data
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_maxp_table(mut sfont: *mut sfnt) -> *mut tt_maxp_table {
    let mut table: *mut tt_maxp_table = new((1_u64)
        .wrapping_mul(::std::mem::size_of::<tt_maxp_table>() as u64)
        as u32) as *mut tt_maxp_table;
    sfnt_locate_table(sfont, b"maxp\x00" as *const u8 as *const i8);
    (*table).version = tt_get_unsigned_quad((*sfont).handle);
    (*table).numGlyphs = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxPoints = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxContours = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentPoints = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentContours = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxZones = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxTwilightPoints = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxStorage = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxFunctionDefs = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxInstructionDefs = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxStackElements = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxSizeOfInstructions = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentElements = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentDepth = tt_get_unsigned_pair((*sfont).handle);
    table
}
#[no_mangle]
pub unsafe extern "C" fn tt_pack_hhea_table(mut table: *mut tt_hhea_table) -> *mut i8 {
    let mut i: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut data: *mut i8 = 0 as *mut i8;
    data = new((36u64 as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    p = data;
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).version as i32, 4i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).ascent as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).descent as i32, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).lineGap as i32, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).advanceWidthMax as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).minLeftSideBearing as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).minRightSideBearing as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).xMaxExtent as i32, 2i32) as isize);
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).caretSlopeRise as i32, 2i32) as isize,
    );
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).caretSlopeRun as i32, 2i32) as isize,
    );
    p = p
        .offset(put_big_endian(p as *mut libc::c_void, (*table).caretOffset as i32, 2i32) as isize);
    i = 0i32;
    while i < 4i32 {
        p = p.offset(put_big_endian(
            p as *mut libc::c_void,
            (*table).reserved[i as usize] as i32,
            2i32,
        ) as isize);
        i += 1
    }
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).metricDataFormat as i32,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).numOfLongHorMetrics as i32,
        2i32,
    ) as isize);
    data
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_hhea_table(mut sfont: *mut sfnt) -> *mut tt_hhea_table {
    let mut i: i32 = 0;
    let mut len: u32 = 0;
    let mut table: *mut tt_hhea_table = new((1_u64)
        .wrapping_mul(::std::mem::size_of::<tt_hhea_table>() as u64)
        as u32) as *mut tt_hhea_table;
    sfnt_locate_table(sfont, b"hhea\x00" as *const u8 as *const i8);
    (*table).version = tt_get_unsigned_quad((*sfont).handle);
    (*table).ascent = tt_get_signed_pair((*sfont).handle);
    (*table).descent = tt_get_signed_pair((*sfont).handle);
    (*table).lineGap = tt_get_signed_pair((*sfont).handle);
    (*table).advanceWidthMax = tt_get_unsigned_pair((*sfont).handle);
    (*table).minLeftSideBearing = tt_get_signed_pair((*sfont).handle);
    (*table).minRightSideBearing = tt_get_signed_pair((*sfont).handle);
    (*table).xMaxExtent = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRise = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRun = tt_get_signed_pair((*sfont).handle);
    (*table).caretOffset = tt_get_signed_pair((*sfont).handle);
    i = 0i32;
    while i < 4i32 {
        (*table).reserved[i as usize] = tt_get_signed_pair((*sfont).handle);
        i += 1
    }
    (*table).metricDataFormat = tt_get_signed_pair((*sfont).handle);
    if (*table).metricDataFormat as i32 != 0i32 {
        _tt_abort(b"unknown metricDataFormat\x00" as *const u8 as *const i8);
    }
    (*table).numOfLongHorMetrics = tt_get_unsigned_pair((*sfont).handle);
    len = sfnt_find_table_len(sfont, b"hmtx\x00" as *const u8 as *const i8);
    (*table).numOfExSideBearings = len
        .wrapping_sub(((*table).numOfLongHorMetrics as i32 * 4i32) as u32)
        .wrapping_div(2_u32) as u16;
    table
}
/* vhea */
#[no_mangle]
pub unsafe extern "C" fn tt_read_vhea_table(mut sfont: *mut sfnt) -> *mut tt_vhea_table {
    let mut i: i32 = 0; /* ushort ? */
    let mut len: u32 = 0;
    let mut table: *mut tt_vhea_table = new((1_u64)
        .wrapping_mul(::std::mem::size_of::<tt_vhea_table>() as u64)
        as u32) as *mut tt_vhea_table;
    sfnt_locate_table(sfont, b"vhea\x00" as *const u8 as *const i8);
    (*table).version = tt_get_unsigned_quad((*sfont).handle);
    (*table).vertTypoAscender = tt_get_signed_pair((*sfont).handle);
    (*table).vertTypoDescender = tt_get_signed_pair((*sfont).handle);
    (*table).vertTypoLineGap = tt_get_signed_pair((*sfont).handle);
    (*table).advanceHeightMax = tt_get_signed_pair((*sfont).handle);
    (*table).minTopSideBearing = tt_get_signed_pair((*sfont).handle);
    (*table).minBottomSideBearing = tt_get_signed_pair((*sfont).handle);
    (*table).yMaxExtent = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRise = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRun = tt_get_signed_pair((*sfont).handle);
    (*table).caretOffset = tt_get_signed_pair((*sfont).handle);
    i = 0i32;
    while i < 4i32 {
        (*table).reserved[i as usize] = tt_get_signed_pair((*sfont).handle);
        i += 1
    }
    (*table).metricDataFormat = tt_get_signed_pair((*sfont).handle);
    (*table).numOfLongVerMetrics = tt_get_unsigned_pair((*sfont).handle);
    len = sfnt_find_table_len(sfont, b"vmtx\x00" as *const u8 as *const i8);
    (*table).numOfExSideBearings = len
        .wrapping_sub(((*table).numOfLongVerMetrics as i32 * 4i32) as u32)
        .wrapping_div(2_u32) as u16;
    table
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_VORG_table(mut sfont: *mut sfnt) -> *mut tt_VORG_table {
    let mut vorg: *mut tt_VORG_table = 0 as *mut tt_VORG_table;
    let mut offset: u32 = 0;
    let mut i: u16 = 0;
    offset = sfnt_find_table_pos(sfont, b"VORG\x00" as *const u8 as *const i8);
    if offset > 0_u32 {
        vorg = new((1_u64).wrapping_mul(::std::mem::size_of::<tt_VORG_table>() as u64) as u32)
            as *mut tt_VORG_table;
        sfnt_locate_table(sfont, b"VORG\x00" as *const u8 as *const i8);
        if tt_get_unsigned_pair((*sfont).handle) as i32 != 1i32
            || tt_get_unsigned_pair((*sfont).handle) as i32 != 0i32
        {
            _tt_abort(b"Unsupported VORG version.\x00" as *const u8 as *const i8);
        }
        (*vorg).defaultVertOriginY = tt_get_signed_pair((*sfont).handle);
        (*vorg).numVertOriginYMetrics = tt_get_unsigned_pair((*sfont).handle);
        (*vorg).vertOriginYMetrics = new(((*vorg).numVertOriginYMetrics as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<tt_vertOriginYMetrics>() as u64)
            as u32) as *mut tt_vertOriginYMetrics;
        /*
         * The vertOriginYMetrics array must be sorted in increasing
         * glyphIndex order.
         */
        i = 0_u16;
        while (i as i32) < (*vorg).numVertOriginYMetrics as i32 {
            (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex =
                tt_get_unsigned_pair((*sfont).handle);
            (*(*vorg).vertOriginYMetrics.offset(i as isize)).vertOriginY =
                tt_get_signed_pair((*sfont).handle);
            i = i.wrapping_add(1)
        }
    } else {
        vorg = 0 as *mut tt_VORG_table
    }
    vorg
}
/*
 * hmtx and vmtx
 *
 *  Reading/writing hmtx and vmtx depend on other tables, maxp and hhea/vhea.
 */
#[no_mangle]
pub unsafe extern "C" fn tt_read_longMetrics(
    mut sfont: *mut sfnt,
    mut numGlyphs: u16,
    mut numLongMetrics: u16,
    mut numExSideBearings: u16,
) -> *mut tt_longMetrics {
    let mut m: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut gid: u16 = 0;
    let mut last_adv: u16 = 0_u16;
    let mut last_esb: i16 = 0_i16;
    m = new(
        (numGlyphs as u32 as u64).wrapping_mul(::std::mem::size_of::<tt_longMetrics>() as u64)
            as u32,
    ) as *mut tt_longMetrics;
    gid = 0_u16;
    while (gid as i32) < numGlyphs as i32 {
        if (gid as i32) < numLongMetrics as i32 {
            last_adv = tt_get_unsigned_pair((*sfont).handle)
        }
        if (gid as i32) < numLongMetrics as i32 + numExSideBearings as i32 {
            last_esb = tt_get_signed_pair((*sfont).handle)
        }
        (*m.offset(gid as isize)).advance = last_adv;
        (*m.offset(gid as isize)).sideBearing = last_esb;
        gid = gid.wrapping_add(1)
    }
    m
}
/* OS/2 table */
/* this table may not exist */
#[no_mangle]
pub unsafe extern "C" fn tt_read_os2__table(mut sfont: *mut sfnt) -> *mut tt_os2__table {
    let mut table: *mut tt_os2__table = 0 as *mut tt_os2__table;
    let mut i: i32 = 0;
    table = new((1_u64).wrapping_mul(::std::mem::size_of::<tt_os2__table>() as u64) as u32)
        as *mut tt_os2__table;
    if sfnt_find_table_pos(sfont, b"OS/2\x00" as *const u8 as *const i8) > 0_u32 {
        sfnt_locate_table(sfont, b"OS/2\x00" as *const u8 as *const i8);
        (*table).version = tt_get_unsigned_pair((*sfont).handle);
        (*table).xAvgCharWidth = tt_get_signed_pair((*sfont).handle);
        (*table).usWeightClass = tt_get_unsigned_pair((*sfont).handle);
        (*table).usWidthClass = tt_get_unsigned_pair((*sfont).handle);
        (*table).fsType = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptXSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptYSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptXOffset = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptYOffset = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptXSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptYSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptXOffset = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptYOffset = tt_get_signed_pair((*sfont).handle);
        (*table).yStrikeoutSize = tt_get_signed_pair((*sfont).handle);
        (*table).yStrikeoutPosition = tt_get_signed_pair((*sfont).handle);
        (*table).sFamilyClass = tt_get_signed_pair((*sfont).handle);
        i = 0i32;
        while i < 10i32 {
            (*table).panose[i as usize] = tt_get_unsigned_byte((*sfont).handle);
            i += 1
        }
        (*table).ulUnicodeRange1 = tt_get_unsigned_quad((*sfont).handle);
        (*table).ulUnicodeRange2 = tt_get_unsigned_quad((*sfont).handle);
        (*table).ulUnicodeRange3 = tt_get_unsigned_quad((*sfont).handle);
        (*table).ulUnicodeRange4 = tt_get_unsigned_quad((*sfont).handle);
        i = 0i32;
        while i < 4i32 {
            (*table).achVendID[i as usize] = tt_get_signed_byte((*sfont).handle);
            i += 1
        }
        (*table).fsSelection = tt_get_unsigned_pair((*sfont).handle);
        (*table).usFirstCharIndex = tt_get_unsigned_pair((*sfont).handle);
        (*table).usLastCharIndex = tt_get_unsigned_pair((*sfont).handle);
        if sfnt_find_table_len(sfont, b"OS/2\x00" as *const u8 as *const i8) >= 78_u32 {
            /* these fields are not present in the original Apple spec (68-byte table),
            but Microsoft's version of "format 0" does include them... grr! */
            (*table).sTypoAscender = tt_get_signed_pair((*sfont).handle);
            (*table).sTypoDescender = tt_get_signed_pair((*sfont).handle);
            (*table).sTypoLineGap = tt_get_signed_pair((*sfont).handle);
            (*table).usWinAscent = tt_get_unsigned_pair((*sfont).handle);
            (*table).usWinDescent = tt_get_unsigned_pair((*sfont).handle);
            if (*table).version as i32 > 0i32 {
                /* format 1 adds the following 2 fields */
                (*table).ulCodePageRange1 = tt_get_unsigned_quad((*sfont).handle);
                (*table).ulCodePageRange2 = tt_get_unsigned_quad((*sfont).handle);
                if (*table).version as i32 > 1i32 {
                    /* and formats 2 and 3 (current) include 5 more.... these share the
                    same fields, only the precise definition of some was changed */
                    (*table).sxHeight = tt_get_signed_pair((*sfont).handle);
                    (*table).sCapHeight = tt_get_signed_pair((*sfont).handle);
                    (*table).usDefaultChar = tt_get_unsigned_pair((*sfont).handle);
                    (*table).usBreakChar = tt_get_unsigned_pair((*sfont).handle);
                    (*table).usMaxContext = tt_get_unsigned_pair((*sfont).handle)
                }
            }
        }
    } else {
        /* used in add_CIDVMetrics() of cidtype0.c */
        (*table).sTypoAscender = 880_i16;
        (*table).sTypoDescender = -120_i16;
        /* used in tt_get_fontdesc() of tt_aux.c */
        (*table).usWeightClass = 400_u16; /* Normal(Regular) */
        (*table).xAvgCharWidth = 0_i16; /* ignore */
        (*table).version = 0_u16; /* TrueType rev 1.5 */
        (*table).fsType = 0_i16; /* Installable Embedding */
        (*table).fsSelection = 0_u16; /* All undefined */
        (*table).sFamilyClass = 0_i16; /* No Classification */
        i = 0i32;
        while i < 10i32 {
            (*table).panose[i as usize] = 0_u8;
            i += 1
            /* All Any */
        }
    }
    table
}
unsafe extern "C" fn tt_get_name(
    mut sfont: *mut sfnt,
    mut dest: *mut i8,
    mut destlen: u16,
    mut plat_id: u16,
    mut enco_id: u16,
    mut lang_id: u16,
    mut name_id: u16,
) -> u16 {
    let mut length: u16 = 0_u16;
    let mut num_names: u16 = 0;
    let mut string_offset: u16 = 0;
    let mut name_offset: u32 = 0;
    let mut i: i32 = 0;
    name_offset = sfnt_locate_table(sfont, b"name\x00" as *const u8 as *const i8);
    if tt_get_unsigned_pair((*sfont).handle) != 0 {
        _tt_abort(b"Expecting zero\x00" as *const u8 as *const i8);
    }
    num_names = tt_get_unsigned_pair((*sfont).handle);
    string_offset = tt_get_unsigned_pair((*sfont).handle);
    i = 0i32;
    while i < num_names as i32 {
        let mut p_id: u16 = 0;
        let mut e_id: u16 = 0;
        let mut n_id: u16 = 0;
        let mut l_id: u16 = 0;
        let mut offset: u16 = 0;
        p_id = tt_get_unsigned_pair((*sfont).handle);
        e_id = tt_get_unsigned_pair((*sfont).handle);
        l_id = tt_get_unsigned_pair((*sfont).handle);
        n_id = tt_get_unsigned_pair((*sfont).handle);
        length = tt_get_unsigned_pair((*sfont).handle);
        offset = tt_get_unsigned_pair((*sfont).handle);
        /* language ID value 0xffffu for `accept any language ID' */
        if p_id as i32 == plat_id as i32
            && e_id as i32 == enco_id as i32
            && (lang_id as u32 == 0xffffu32 || l_id as i32 == lang_id as i32)
            && n_id as i32 == name_id as i32
        {
            if length as i32 > destlen as i32 - 1i32 {
                warn!(
                    "Name string too long ({}), truncating to {}",
                    length, destlen,
                );
                length = (destlen as i32 - 1i32) as u16
            }
            ttstub_input_seek(
                (*sfont).handle,
                name_offset
                    .wrapping_add(string_offset as u32)
                    .wrapping_add(offset as u32) as ssize_t,
                0i32,
            );
            ttstub_input_read(
                (*sfont).handle,
                dest as *mut u8 as *mut i8,
                length as size_t,
            );
            *dest.offset(length as isize) = '\u{0}' as i32 as i8;
            break;
        } else {
            i += 1
        }
    }
    if i == num_names as i32 {
        length = 0_u16
    }
    length
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
/* set to 0 */
/* extra information */
/* v.1.1 name */
/* v.1.1 name */
/* v.1.1 name */
/* set to 0 */
/* extra information */
/* 0x0001 or 0x0002 */
/* if (faType & 0x08) editable_embedding */
/* TTF spec. from MS is wrong */
/* TTF spec. from MS is wrong */
/* TTF spec. from MS is wrong */
/* version 0x0002 */
/* hmtx and vmtx */
/* head, hhea, maxp */
/* vhea */
/* VORG */
/* hmtx and vmtx */
/* OS/2 table */
/* name table */
#[no_mangle]
pub unsafe extern "C" fn tt_get_ps_fontname(
    mut sfont: *mut sfnt,
    mut dest: *mut i8,
    mut destlen: u16,
) -> u16 {
    let mut namelen: u16 = 0_u16;
    /* First try Mac-Roman PS name and then Win-Unicode PS name */
    namelen = tt_get_name(sfont, dest, destlen, 1_u16, 0_u16, 0_u16, 6_u16);
    if namelen as i32 != 0i32
        || {
            namelen = tt_get_name(sfont, dest, destlen, 3_u16, 1_u16, 0x409_u16, 6_u16);
            namelen as i32 != 0i32
        }
        || {
            namelen = tt_get_name(sfont, dest, destlen, 3_u16, 5_u16, 0x412_u16, 6_u16);
            namelen as i32 != 0i32
        }
    {
        return namelen;
    }
    warn!("No valid PostScript name available");
    /*
      Workaround for some bad TTfonts:
      Language ID value 0xffffu for `accept any language ID'
    */
    namelen = tt_get_name(sfont, dest, destlen, 1_u16, 0_u16, 0xffff_u16, 6_u16);
    if namelen as i32 == 0i32 {
        /*
          Finally falling back to Mac Roman name field.
          Warning: Some bad Japanese TTfonts using SJIS encoded string in the
          Mac Roman name field.
        */
        namelen = tt_get_name(sfont, dest, destlen, 1_u16, 0_u16, 0_u16, 1_u16)
    }
    namelen
}
