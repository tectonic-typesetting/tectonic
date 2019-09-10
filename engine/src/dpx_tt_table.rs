#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut i8,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    #[no_mangle]
    fn tt_get_signed_byte(handle: rust_input_handle_t) -> libc::c_schar;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> libc::c_short;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn put_big_endian(s: *mut libc::c_void, q: SFNT_LONG, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sfnt_find_table_len(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const i8) -> SFNT_ULONG;
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
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = i64;
pub type int32_t = __int32_t;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type BYTE = u8;
pub type SFNT_CHAR = libc::c_schar;
pub type USHORT = u16;
pub type SHORT = libc::c_short;
pub type SFNT_ULONG = u32;
pub type SFNT_LONG = int32_t;
pub type Fixed = u32;
pub type FWord = libc::c_short;
pub type uFWord = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut i8,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_head_table {
    pub version: Fixed,
    pub fontRevision: Fixed,
    pub checkSumAdjustment: SFNT_ULONG,
    pub magicNumber: SFNT_ULONG,
    pub flags: USHORT,
    pub unitsPerEm: USHORT,
    pub created: [BYTE; 8],
    pub modified: [BYTE; 8],
    pub xMin: FWord,
    pub yMin: FWord,
    pub xMax: FWord,
    pub yMax: FWord,
    pub macStyle: USHORT,
    pub lowestRecPPEM: USHORT,
    pub fontDirectionHint: SHORT,
    pub indexToLocFormat: SHORT,
    pub glyphDataFormat: SHORT,
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
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: FWord,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongHorMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vhea_table {
    pub version: Fixed,
    pub vertTypoAscender: SHORT,
    pub vertTypoDescender: SHORT,
    pub vertTypoLineGap: SHORT,
    pub advanceHeightMax: SHORT,
    pub minTopSideBearing: SHORT,
    pub minBottomSideBearing: SHORT,
    pub yMaxExtent: SHORT,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: SHORT,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongVerMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_maxp_table {
    pub version: Fixed,
    pub numGlyphs: USHORT,
    pub maxPoints: USHORT,
    pub maxContours: USHORT,
    pub maxComponentPoints: USHORT,
    pub maxComponentContours: USHORT,
    pub maxZones: USHORT,
    pub maxTwilightPoints: USHORT,
    pub maxStorage: USHORT,
    pub maxFunctionDefs: USHORT,
    pub maxInstructionDefs: USHORT,
    pub maxStackElements: USHORT,
    pub maxSizeOfInstructions: USHORT,
    pub maxComponentElements: USHORT,
    pub maxComponentDepth: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_os2__table {
    pub version: USHORT,
    pub xAvgCharWidth: SHORT,
    pub usWeightClass: USHORT,
    pub usWidthClass: USHORT,
    pub fsType: SHORT,
    pub ySubscriptXSize: SHORT,
    pub ySubscriptYSize: SHORT,
    pub ySubscriptXOffset: SHORT,
    pub ySubscriptYOffset: SHORT,
    pub ySuperscriptXSize: SHORT,
    pub ySuperscriptYSize: SHORT,
    pub ySuperscriptXOffset: SHORT,
    pub ySuperscriptYOffset: SHORT,
    pub yStrikeoutSize: SHORT,
    pub yStrikeoutPosition: SHORT,
    pub sFamilyClass: SHORT,
    pub panose: [BYTE; 10],
    pub ulUnicodeRange1: SFNT_ULONG,
    pub ulUnicodeRange2: SFNT_ULONG,
    pub ulUnicodeRange3: SFNT_ULONG,
    pub ulUnicodeRange4: SFNT_ULONG,
    pub achVendID: [SFNT_CHAR; 4],
    pub fsSelection: USHORT,
    pub usFirstCharIndex: USHORT,
    pub usLastCharIndex: USHORT,
    pub sTypoAscender: SHORT,
    pub sTypoDescender: SHORT,
    pub sTypoLineGap: SHORT,
    pub usWinAscent: USHORT,
    pub usWinDescent: USHORT,
    pub ulCodePageRange1: SFNT_ULONG,
    pub ulCodePageRange2: SFNT_ULONG,
    pub sxHeight: SHORT,
    pub sCapHeight: SHORT,
    pub usDefaultChar: USHORT,
    pub usBreakChar: USHORT,
    pub usMaxContext: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vertOriginYMetrics {
    pub glyphIndex: USHORT,
    pub vertOriginY: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_VORG_table {
    pub defaultVertOriginY: SHORT,
    pub numVertOriginYMetrics: USHORT,
    pub vertOriginYMetrics: *mut tt_vertOriginYMetrics,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: USHORT,
    pub sideBearing: SHORT,
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
    let mut i: libc::c_int = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut data: *mut i8 = 0 as *mut i8;
    if table.is_null() {
        _tt_abort(b"passed NULL pointer\n\x00" as *const u8 as *const i8);
    }
    data = new((54u64 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    p = data;
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).version as SFNT_LONG, 4i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).fontRevision as SFNT_LONG,
        4i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).checkSumAdjustment as SFNT_LONG,
        4i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).magicNumber as SFNT_LONG,
        4i32,
    ) as isize);
    p = p
        .offset(put_big_endian(p as *mut libc::c_void, (*table).flags as SFNT_LONG, 2i32) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).unitsPerEm as SFNT_LONG,
        2i32,
    ) as isize);
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
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).xMin as SFNT_LONG, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).yMin as SFNT_LONG, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).xMax as SFNT_LONG, 2i32) as isize);
    p = p.offset(put_big_endian(p as *mut libc::c_void, (*table).yMax as SFNT_LONG, 2i32) as isize);
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).macStyle as SFNT_LONG, 2i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).lowestRecPPEM as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).fontDirectionHint as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).indexToLocFormat as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).glyphDataFormat as SFNT_LONG,
        2i32,
    ) as isize);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_head_table(mut sfont: *mut sfnt) -> *mut tt_head_table {
    let mut i: libc::c_int = 0;
    let mut table: *mut tt_head_table = new((1i32 as u32 as u64)
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
    (*table).macStyle = tt_get_signed_pair((*sfont).handle) as USHORT;
    (*table).lowestRecPPEM = tt_get_signed_pair((*sfont).handle) as USHORT;
    (*table).fontDirectionHint = tt_get_signed_pair((*sfont).handle);
    (*table).indexToLocFormat = tt_get_signed_pair((*sfont).handle);
    (*table).glyphDataFormat = tt_get_signed_pair((*sfont).handle);
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn tt_pack_maxp_table(mut table: *mut tt_maxp_table) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut data: *mut i8 = 0 as *mut i8;
    data = new((32u64 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    p = data;
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).version as SFNT_LONG, 4i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).numGlyphs as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxPoints as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxContours as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentPoints as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentContours as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).maxZones as SFNT_LONG, 2i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxTwilightPoints as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxStorage as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxFunctionDefs as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxInstructionDefs as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxStackElements as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxSizeOfInstructions as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentElements as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).maxComponentDepth as SFNT_LONG,
        2i32,
    ) as isize);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_maxp_table(mut sfont: *mut sfnt) -> *mut tt_maxp_table {
    let mut table: *mut tt_maxp_table = new((1i32 as u32 as u64)
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
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn tt_pack_hhea_table(mut table: *mut tt_hhea_table) -> *mut i8 {
    let mut i: libc::c_int = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut data: *mut i8 = 0 as *mut i8;
    data = new((36u64 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    p = data;
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).version as SFNT_LONG, 4i32) as isize,
    );
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).ascent as SFNT_LONG, 2i32) as isize,
    );
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).descent as SFNT_LONG, 2i32) as isize,
    );
    p = p.offset(
        put_big_endian(p as *mut libc::c_void, (*table).lineGap as SFNT_LONG, 2i32) as isize,
    );
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).advanceWidthMax as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).minLeftSideBearing as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).minRightSideBearing as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).xMaxExtent as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).caretSlopeRise as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).caretSlopeRun as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).caretOffset as SFNT_LONG,
        2i32,
    ) as isize);
    i = 0i32;
    while i < 4i32 {
        p = p.offset(put_big_endian(
            p as *mut libc::c_void,
            (*table).reserved[i as usize] as SFNT_LONG,
            2i32,
        ) as isize);
        i += 1
    }
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).metricDataFormat as SFNT_LONG,
        2i32,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut libc::c_void,
        (*table).numOfLongHorMetrics as SFNT_LONG,
        2i32,
    ) as isize);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_hhea_table(mut sfont: *mut sfnt) -> *mut tt_hhea_table {
    let mut i: libc::c_int = 0;
    let mut len: SFNT_ULONG = 0;
    let mut table: *mut tt_hhea_table = new((1i32 as u32 as u64)
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
    if (*table).metricDataFormat as libc::c_int != 0i32 {
        _tt_abort(b"unknown metricDataFormat\x00" as *const u8 as *const i8);
    }
    (*table).numOfLongHorMetrics = tt_get_unsigned_pair((*sfont).handle);
    len = sfnt_find_table_len(sfont, b"hmtx\x00" as *const u8 as *const i8);
    (*table).numOfExSideBearings = len
        .wrapping_sub(((*table).numOfLongHorMetrics as libc::c_int * 4i32) as libc::c_uint)
        .wrapping_div(2i32 as libc::c_uint) as USHORT;
    return table;
}
/* vhea */
#[no_mangle]
pub unsafe extern "C" fn tt_read_vhea_table(mut sfont: *mut sfnt) -> *mut tt_vhea_table {
    let mut i: libc::c_int = 0; /* ushort ? */
    let mut len: SFNT_ULONG = 0;
    let mut table: *mut tt_vhea_table = new((1i32 as u32 as u64)
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
        .wrapping_sub(((*table).numOfLongVerMetrics as libc::c_int * 4i32) as libc::c_uint)
        .wrapping_div(2i32 as libc::c_uint) as USHORT;
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn tt_read_VORG_table(mut sfont: *mut sfnt) -> *mut tt_VORG_table {
    let mut vorg: *mut tt_VORG_table = 0 as *mut tt_VORG_table;
    let mut offset: SFNT_ULONG = 0;
    let mut i: USHORT = 0;
    offset = sfnt_find_table_pos(sfont, b"VORG\x00" as *const u8 as *const i8);
    if offset > 0i32 as libc::c_uint {
        vorg = new((1i32 as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<tt_VORG_table>() as u64)
            as u32) as *mut tt_VORG_table;
        sfnt_locate_table(sfont, b"VORG\x00" as *const u8 as *const i8);
        if tt_get_unsigned_pair((*sfont).handle) as libc::c_int != 1i32
            || tt_get_unsigned_pair((*sfont).handle) as libc::c_int != 0i32
        {
            _tt_abort(b"Unsupported VORG version.\x00" as *const u8 as *const i8);
        }
        (*vorg).defaultVertOriginY = tt_get_signed_pair((*sfont).handle);
        (*vorg).numVertOriginYMetrics = tt_get_unsigned_pair((*sfont).handle);
        (*vorg).vertOriginYMetrics = new(((*vorg).numVertOriginYMetrics as u32
            as u64)
            .wrapping_mul(::std::mem::size_of::<tt_vertOriginYMetrics>() as u64)
            as u32) as *mut tt_vertOriginYMetrics;
        /*
         * The vertOriginYMetrics array must be sorted in increasing
         * glyphIndex order.
         */
        i = 0i32 as USHORT;
        while (i as libc::c_int) < (*vorg).numVertOriginYMetrics as libc::c_int {
            (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex =
                tt_get_unsigned_pair((*sfont).handle);
            (*(*vorg).vertOriginYMetrics.offset(i as isize)).vertOriginY =
                tt_get_signed_pair((*sfont).handle);
            i = i.wrapping_add(1)
        }
    } else {
        vorg = 0 as *mut tt_VORG_table
    }
    return vorg;
}
/*
 * hmtx and vmtx
 *
 *  Reading/writing hmtx and vmtx depend on other tables, maxp and hhea/vhea.
 */
#[no_mangle]
pub unsafe extern "C" fn tt_read_longMetrics(
    mut sfont: *mut sfnt,
    mut numGlyphs: USHORT,
    mut numLongMetrics: USHORT,
    mut numExSideBearings: USHORT,
) -> *mut tt_longMetrics {
    let mut m: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut gid: USHORT = 0;
    let mut last_adv: USHORT = 0i32 as USHORT;
    let mut last_esb: SHORT = 0i32 as SHORT;
    m = new((numGlyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<tt_longMetrics>() as u64)
        as u32) as *mut tt_longMetrics;
    gid = 0i32 as USHORT;
    while (gid as libc::c_int) < numGlyphs as libc::c_int {
        if (gid as libc::c_int) < numLongMetrics as libc::c_int {
            last_adv = tt_get_unsigned_pair((*sfont).handle)
        }
        if (gid as libc::c_int) < numLongMetrics as libc::c_int + numExSideBearings as libc::c_int {
            last_esb = tt_get_signed_pair((*sfont).handle)
        }
        (*m.offset(gid as isize)).advance = last_adv;
        (*m.offset(gid as isize)).sideBearing = last_esb;
        gid = gid.wrapping_add(1)
    }
    return m;
}
/* OS/2 table */
/* this table may not exist */
#[no_mangle]
pub unsafe extern "C" fn tt_read_os2__table(mut sfont: *mut sfnt) -> *mut tt_os2__table {
    let mut table: *mut tt_os2__table = 0 as *mut tt_os2__table;
    let mut i: libc::c_int = 0;
    table = new((1i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<tt_os2__table>() as u64)
        as u32) as *mut tt_os2__table;
    if sfnt_find_table_pos(sfont, b"OS/2\x00" as *const u8 as *const i8)
        > 0i32 as libc::c_uint
    {
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
        if sfnt_find_table_len(sfont, b"OS/2\x00" as *const u8 as *const i8)
            >= 78i32 as libc::c_uint
        {
            /* these fields are not present in the original Apple spec (68-byte table),
            but Microsoft's version of "format 0" does include them... grr! */
            (*table).sTypoAscender = tt_get_signed_pair((*sfont).handle);
            (*table).sTypoDescender = tt_get_signed_pair((*sfont).handle);
            (*table).sTypoLineGap = tt_get_signed_pair((*sfont).handle);
            (*table).usWinAscent = tt_get_unsigned_pair((*sfont).handle);
            (*table).usWinDescent = tt_get_unsigned_pair((*sfont).handle);
            if (*table).version as libc::c_int > 0i32 {
                /* format 1 adds the following 2 fields */
                (*table).ulCodePageRange1 = tt_get_unsigned_quad((*sfont).handle);
                (*table).ulCodePageRange2 = tt_get_unsigned_quad((*sfont).handle);
                if (*table).version as libc::c_int > 1i32 {
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
        (*table).sTypoAscender = 880i32 as SHORT;
        (*table).sTypoDescender = -120i32 as SHORT;
        /* used in tt_get_fontdesc() of tt_aux.c */
        (*table).usWeightClass = 400u32 as USHORT; /* Normal(Regular) */
        (*table).xAvgCharWidth = 0i32 as SHORT; /* ignore */
        (*table).version = 0i32 as USHORT; /* TrueType rev 1.5 */
        (*table).fsType = 0i32 as SHORT; /* Installable Embedding */
        (*table).fsSelection = 0u32 as USHORT; /* All undefined */
        (*table).sFamilyClass = 0i32 as SHORT; /* No Classification */
        i = 0i32;
        while i < 10i32 {
            (*table).panose[i as usize] = 0i32 as BYTE;
            i += 1
            /* All Any */
        }
    }
    return table;
}
unsafe extern "C" fn tt_get_name(
    mut sfont: *mut sfnt,
    mut dest: *mut i8,
    mut destlen: USHORT,
    mut plat_id: USHORT,
    mut enco_id: USHORT,
    mut lang_id: USHORT,
    mut name_id: USHORT,
) -> USHORT {
    let mut length: USHORT = 0i32 as USHORT;
    let mut num_names: USHORT = 0;
    let mut string_offset: USHORT = 0;
    let mut name_offset: SFNT_ULONG = 0;
    let mut i: libc::c_int = 0;
    name_offset = sfnt_locate_table(sfont, b"name\x00" as *const u8 as *const i8);
    if tt_get_unsigned_pair((*sfont).handle) != 0 {
        _tt_abort(b"Expecting zero\x00" as *const u8 as *const i8);
    }
    num_names = tt_get_unsigned_pair((*sfont).handle);
    string_offset = tt_get_unsigned_pair((*sfont).handle);
    i = 0i32;
    while i < num_names as libc::c_int {
        let mut p_id: USHORT = 0;
        let mut e_id: USHORT = 0;
        let mut n_id: USHORT = 0;
        let mut l_id: USHORT = 0;
        let mut offset: USHORT = 0;
        p_id = tt_get_unsigned_pair((*sfont).handle);
        e_id = tt_get_unsigned_pair((*sfont).handle);
        l_id = tt_get_unsigned_pair((*sfont).handle);
        n_id = tt_get_unsigned_pair((*sfont).handle);
        length = tt_get_unsigned_pair((*sfont).handle);
        offset = tt_get_unsigned_pair((*sfont).handle);
        /* language ID value 0xffffu for `accept any language ID' */
        if p_id as libc::c_int == plat_id as libc::c_int
            && e_id as libc::c_int == enco_id as libc::c_int
            && (lang_id as libc::c_uint == 0xffffu32
                || l_id as libc::c_int == lang_id as libc::c_int)
            && n_id as libc::c_int == name_id as libc::c_int
        {
            if length as libc::c_int > destlen as libc::c_int - 1i32 {
                dpx_warning(
                    b"Name string too long (%u), truncating to %u\x00" as *const u8
                        as *const i8,
                    length as libc::c_int,
                    destlen as libc::c_int,
                );
                length = (destlen as libc::c_int - 1i32) as USHORT
            }
            ttstub_input_seek(
                (*sfont).handle,
                name_offset
                    .wrapping_add(string_offset as libc::c_uint)
                    .wrapping_add(offset as libc::c_uint) as ssize_t,
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
    if i == num_names as libc::c_int {
        length = 0i32 as USHORT
    }
    return length;
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
    mut destlen: USHORT,
) -> USHORT {
    let mut namelen: USHORT = 0i32 as USHORT;
    /* First try Mac-Roman PS name and then Win-Unicode PS name */
    namelen = tt_get_name(
        sfont,
        dest,
        destlen,
        1i32 as USHORT,
        0i32 as USHORT,
        0i32 as USHORT,
        6i32 as USHORT,
    );
    if namelen as libc::c_int != 0i32
        || {
            namelen = tt_get_name(
                sfont,
                dest,
                destlen,
                3i32 as USHORT,
                1i32 as USHORT,
                0x409u32 as USHORT,
                6i32 as USHORT,
            );
            namelen as libc::c_int != 0i32
        }
        || {
            namelen = tt_get_name(
                sfont,
                dest,
                destlen,
                3i32 as USHORT,
                5i32 as USHORT,
                0x412u32 as USHORT,
                6i32 as USHORT,
            );
            namelen as libc::c_int != 0i32
        }
    {
        return namelen;
    }
    dpx_warning(b"No valid PostScript name available\x00" as *const u8 as *const i8);
    /*
      Workaround for some bad TTfonts:
      Language ID value 0xffffu for `accept any language ID'
    */
    namelen = tt_get_name(
        sfont,
        dest,
        destlen,
        1i32 as USHORT,
        0i32 as USHORT,
        0xffffu32 as USHORT,
        6i32 as USHORT,
    );
    if namelen as libc::c_int == 0i32 {
        /*
          Finally falling back to Mac Roman name field.
          Warning: Some bad Japanese TTfonts using SJIS encoded string in the
          Mac Roman name field.
        */
        namelen = tt_get_name(
            sfont,
            dest,
            destlen,
            1i32 as USHORT,
            0i32 as USHORT,
            0i32 as USHORT,
            1i32 as USHORT,
        )
    }
    return namelen;
}
