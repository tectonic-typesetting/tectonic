#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::warn;

extern crate libc;
use libc::free;
extern "C" {
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn sfnt_set_table(sfont: *mut sfnt, tag: *const i8, data: *mut libc::c_void, length: u32);
    #[no_mangle]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const i8) -> u32;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> u32;
    #[no_mangle]
    fn put_big_endian(s: *mut libc::c_void, q: i32, n: i32) -> i32;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> i16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
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
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    /* head, hhea, maxp */
    #[no_mangle]
    fn tt_pack_head_table(table: *mut tt_head_table) -> *mut i8;
    #[no_mangle]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[no_mangle]
    fn tt_pack_hhea_table(table: *mut tt_hhea_table) -> *mut i8;
    #[no_mangle]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[no_mangle]
    fn tt_pack_maxp_table(table: *mut tt_maxp_table) -> *mut i8;
    #[no_mangle]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    /* vhea */
    #[no_mangle]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    /* hmtx and vmtx */
    #[no_mangle]
    fn tt_read_longMetrics(
        sfont: *mut sfnt,
        numGlyphs: u16,
        numLongMetrics: u16,
        numExSideBearings: u16,
    ) -> *mut tt_longMetrics;
    /* OS/2 table */
    #[no_mangle]
    fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;
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
pub struct tt_glyph_desc {
    pub gid: u16,
    pub ogid: u16,
    pub advw: u16,
    pub advh: u16,
    pub lsb: i16,
    pub tsb: i16,
    pub llx: i16,
    pub lly: i16,
    pub urx: i16,
    pub ury: i16,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyphs {
    pub num_glyphs: u16,
    pub max_glyphs: u16,
    pub last_gid: u16,
    pub emsize: u16,
    pub dw: u16,
    pub default_advh: u16,
    pub default_tsb: i16,
    pub gd: *mut tt_glyph_desc,
    pub used_slot: *mut u8,
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
    /* extra information */
}
/* hmtx and vmtx */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: u16,
    pub sideBearing: i16,
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
    /* extra information */
}
unsafe extern "C" fn find_empty_slot(mut g: *mut tt_glyphs) -> u16 {
    let mut gid: u16 = 0;
    assert!(!g.is_null());
    gid = 0_u16;
    while (gid as i32) < 65534i32 {
        if *(*g).used_slot.offset((gid as i32 / 8i32) as isize) as i32
            & 1i32 << 7i32 - gid as i32 % 8i32
            == 0
        {
            break;
        }
        gid = gid.wrapping_add(1)
    }
    if gid as i32 == 65534i32 {
        _tt_abort(b"No empty glyph slot available.\x00" as *const u8 as *const i8);
    }
    gid
}
#[no_mangle]
pub unsafe extern "C" fn tt_find_glyph(mut g: *mut tt_glyphs, mut gid: u16) -> u16 {
    let mut idx: u16 = 0;
    let mut new_gid: u16 = 0_u16;
    assert!(!g.is_null());
    idx = 0_u16;
    while (idx as i32) < (*g).num_glyphs as i32 {
        if gid as i32 == (*(*g).gd.offset(idx as isize)).ogid as i32 {
            new_gid = (*(*g).gd.offset(idx as isize)).gid;
            break;
        } else {
            idx = idx.wrapping_add(1)
        }
    }
    new_gid
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_index(mut g: *mut tt_glyphs, mut gid: u16) -> u16 {
    let mut idx: u16 = 0;
    assert!(!g.is_null());
    idx = 0_u16;
    while (idx as i32) < (*g).num_glyphs as i32 {
        if gid as i32 == (*(*g).gd.offset(idx as isize)).gid as i32 {
            break;
        }
        idx = idx.wrapping_add(1)
    }
    if idx as i32 == (*g).num_glyphs as i32 {
        idx = 0_u16
    }
    idx
}
#[no_mangle]
pub unsafe extern "C" fn tt_add_glyph(
    mut g: *mut tt_glyphs,
    mut gid: u16,
    mut new_gid: u16,
) -> u16 {
    assert!(!g.is_null());
    if *(*g).used_slot.offset((new_gid as i32 / 8i32) as isize) as i32
        & 1i32 << 7i32 - new_gid as i32 % 8i32
        != 0
    {
        warn!("Slot {} already used.", new_gid as i32);
    } else {
        if (*g).num_glyphs as i32 + 1i32 >= 65534i32 {
            _tt_abort(b"Too many glyphs.\x00" as *const u8 as *const i8);
        }
        if (*g).num_glyphs as i32 >= (*g).max_glyphs as i32 {
            (*g).max_glyphs = ((*g).max_glyphs as i32 + 256i32) as u16;
            (*g).gd = renew(
                (*g).gd as *mut libc::c_void,
                ((*g).max_glyphs as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<tt_glyph_desc>() as u64)
                    as u32,
            ) as *mut tt_glyph_desc
        }
        (*(*g).gd.offset((*g).num_glyphs as isize)).gid = new_gid;
        (*(*g).gd.offset((*g).num_glyphs as isize)).ogid = gid;
        (*(*g).gd.offset((*g).num_glyphs as isize)).length = 0_u32;
        let ref mut fresh0 = (*(*g).gd.offset((*g).num_glyphs as isize)).data;
        *fresh0 = 0 as *mut u8;
        let ref mut fresh1 = *(*g).used_slot.offset((new_gid as i32 / 8i32) as isize);
        *fresh1 = (*fresh1 as i32 | 1i32 << 7i32 - new_gid as i32 % 8i32) as u8;
        (*g).num_glyphs = ((*g).num_glyphs as i32 + 1i32) as u16
    }
    if new_gid as i32 > (*g).last_gid as i32 {
        (*g).last_gid = new_gid
    }
    new_gid
}
/*
 * Initialization
 */
#[no_mangle]
pub unsafe extern "C" fn tt_build_init() -> *mut tt_glyphs {
    let mut g: *mut tt_glyphs = 0 as *mut tt_glyphs;
    g = new((1_u64).wrapping_mul(::std::mem::size_of::<tt_glyphs>() as u64) as u32)
        as *mut tt_glyphs;
    (*g).num_glyphs = 0_u16;
    (*g).max_glyphs = 0_u16;
    (*g).last_gid = 0_u16;
    (*g).emsize = 1_u16;
    (*g).default_advh = 0_u16;
    (*g).default_tsb = 0_i16;
    (*g).gd = 0 as *mut tt_glyph_desc;
    (*g).used_slot =
        new((8192_u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    memset((*g).used_slot as *mut libc::c_void, 0i32, 8192i32 as u64);
    tt_add_glyph(g, 0_u16, 0_u16);
    g
}
#[no_mangle]
pub unsafe extern "C" fn tt_build_finish(mut g: *mut tt_glyphs) {
    if !g.is_null() {
        if !(*g).gd.is_null() {
            let mut idx: u16 = 0;
            idx = 0_u16;
            while (idx as i32) < (*g).num_glyphs as i32 {
                free((*(*g).gd.offset(idx as isize)).data as *mut libc::c_void);
                idx = idx.wrapping_add(1)
            }
            free((*g).gd as *mut libc::c_void);
        }
        free((*g).used_slot as *mut libc::c_void);
        free(g as *mut libc::c_void);
    };
}
#[inline]
unsafe extern "C" fn glyf_cmp(mut v1: *const libc::c_void, mut v2: *const libc::c_void) -> i32 {
    let mut cmp: i32 = 0i32;
    let mut sv1: *const tt_glyph_desc = 0 as *const tt_glyph_desc;
    let mut sv2: *const tt_glyph_desc = 0 as *const tt_glyph_desc;
    sv1 = v1 as *const tt_glyph_desc;
    sv2 = v2 as *const tt_glyph_desc;
    if (*sv1).gid as i32 == (*sv2).gid as i32 {
        cmp = 0i32
    } else if ((*sv1).gid as i32) < (*sv2).gid as i32 {
        cmp = -1i32
    } else {
        cmp = 1i32
    }
    cmp
}
#[no_mangle]
pub unsafe extern "C" fn tt_build_tables(mut sfont: *mut sfnt, mut g: *mut tt_glyphs) -> i32 {
    let mut hmtx_table_data: *mut i8 = 0 as *mut i8;
    let mut loca_table_data: *mut i8 = 0 as *mut i8;
    let mut glyf_table_data: *mut i8 = 0 as *mut i8;
    let mut hmtx_table_size: u32 = 0;
    let mut loca_table_size: u32 = 0;
    let mut glyf_table_size: u32 = 0;
    /* some information available from other TrueType table */
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    let mut hmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut vmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
    /* temp */
    let mut location: *mut u32 = 0 as *mut u32; /* Estimate most frequently appeared width */
    let mut offset: u32 = 0;
    let mut i: i32 = 0;
    let mut w_stat: *mut u16 = 0 as *mut u16;
    assert!(!g.is_null());
    if sfont.is_null() || (*sfont).handle.is_null() {
        _tt_abort(b"File not opened.\x00" as *const u8 as *const i8);
    }
    if (*sfont).type_0 != 1i32 << 0i32
        && (*sfont).type_0 != 1i32 << 4i32
        && (*sfont).type_0 != 1i32 << 8i32
    {
        _tt_abort(b"Invalid font type\x00" as *const u8 as *const i8);
    }
    if (*g).num_glyphs as i32 > 65534i32 {
        _tt_abort(b"Too many glyphs.\x00" as *const u8 as *const i8);
    }
    /*
     * Read head, hhea, maxp, loca:
     *
     *   unitsPerEm       --> head
     *   numHMetrics      --> hhea
     *   indexToLocFormat --> head
     *   numGlyphs        --> maxp
     */
    head = tt_read_head_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    if (*hhea).metricDataFormat as i32 != 0i32 {
        _tt_abort(b"Unknown metricDataFormat.\x00" as *const u8 as *const i8);
    }
    (*g).emsize = (*head).unitsPerEm;
    sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const i8);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    os2 = tt_read_os2__table(sfont);
    if !os2.is_null() {
        (*g).default_advh = ((*os2).sTypoAscender as i32 - (*os2).sTypoDescender as i32) as u16;
        (*g).default_tsb = ((*g).default_advh as i32 - (*os2).sTypoAscender as i32) as i16
    }
    if sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8) > 0_u32 {
        let mut vhea: *mut tt_vhea_table = 0 as *mut tt_vhea_table;
        vhea = tt_read_vhea_table(sfont);
        sfnt_locate_table(sfont, b"vmtx\x00" as *const u8 as *const i8);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        );
        free(vhea as *mut libc::c_void);
    } else {
        vmtx = 0 as *mut tt_longMetrics
    }
    sfnt_locate_table(sfont, b"loca\x00" as *const u8 as *const i8);
    location = new((((*maxp).numGlyphs as i32 + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u32>() as u64) as u32) as *mut u32;
    if (*head).indexToLocFormat as i32 == 0i32 {
        i = 0i32;
        while i <= (*maxp).numGlyphs as i32 {
            *location.offset(i as isize) =
                (2_u32).wrapping_mul(tt_get_unsigned_pair((*sfont).handle) as u32);
            i += 1
        }
    } else if (*head).indexToLocFormat as i32 == 1i32 {
        i = 0i32;
        while i <= (*maxp).numGlyphs as i32 {
            *location.offset(i as isize) = tt_get_unsigned_quad((*sfont).handle);
            i += 1
        }
    } else {
        _tt_abort(b"Unknown IndexToLocFormat.\x00" as *const u8 as *const i8);
    }
    w_stat = new((((*g).emsize as i32 + 2i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32) as *mut u16;
    memset(
        w_stat as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<u16>() as u64).wrapping_mul(((*g).emsize as i32 + 2i32) as u64),
    );
    /*
     * Read glyf table.
     */
    offset = sfnt_locate_table(sfont, b"glyf\x00" as *const u8 as *const i8);
    /*
     * The num_glyphs may grow when composite glyph is found.
     * A component of glyph refered by a composite glyph is appended
     * to used_glyphs if it is not already registered in used_glyphs.
     * Glyph programs of composite glyphs are modified so that it
     * correctly refer to new gid of their components.
     */
    i = 0i32; /* old gid */
    while i < 65534i32 {
        let mut gid: u16 = 0;
        let mut loc: u32 = 0;
        let mut len: u32 = 0;
        let mut p: *mut u8 = 0 as *mut u8;
        let mut endptr: *mut u8 = 0 as *mut u8;
        let mut number_of_contours: i16 = 0;
        if i >= (*g).num_glyphs as i32 {
            break;
        }
        gid = (*(*g).gd.offset(i as isize)).ogid;
        if gid as i32 >= (*maxp).numGlyphs as i32 {
            _tt_abort(
                b"Invalid glyph index (gid %u)\x00" as *const u8 as *const i8,
                gid as i32,
            );
        }
        loc = *location.offset(gid as isize);
        len = (*location.offset((gid as i32 + 1i32) as isize)).wrapping_sub(loc);
        (*(*g).gd.offset(i as isize)).advw = (*hmtx.offset(gid as isize)).advance;
        (*(*g).gd.offset(i as isize)).lsb = (*hmtx.offset(gid as isize)).sideBearing;
        if !vmtx.is_null() {
            (*(*g).gd.offset(i as isize)).advh = (*vmtx.offset(gid as isize)).advance;
            (*(*g).gd.offset(i as isize)).tsb = (*vmtx.offset(gid as isize)).sideBearing
        } else {
            (*(*g).gd.offset(i as isize)).advh = (*g).default_advh;
            (*(*g).gd.offset(i as isize)).tsb = (*g).default_tsb
        }
        (*(*g).gd.offset(i as isize)).length = len;
        let ref mut fresh2 = (*(*g).gd.offset(i as isize)).data;
        *fresh2 = 0 as *mut u8;
        if (*(*g).gd.offset(i as isize)).advw as i32 <= (*g).emsize as i32 {
            let ref mut fresh3 = *w_stat.offset((*(*g).gd.offset(i as isize)).advw as isize);
            *fresh3 = (*fresh3 as i32 + 1i32) as u16
        } else {
            let ref mut fresh4 = *w_stat.offset(((*g).emsize as i32 + 1i32) as isize);
            *fresh4 = (*fresh4 as i32 + 1i32) as u16
            /* larger than em */
        }
        if !(len == 0_u32) {
            if len < 10_u32 {
                _tt_abort(
                    b"Invalid TrueType glyph data (gid %u).\x00" as *const u8 as *const i8,
                    gid as i32,
                );
            }
            p = new((len as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
                as *mut u8;
            let ref mut fresh5 = (*(*g).gd.offset(i as isize)).data;
            *fresh5 = p;
            endptr = p.offset(len as isize);
            ttstub_input_seek((*sfont).handle, offset.wrapping_add(loc) as ssize_t, 0i32);
            number_of_contours = tt_get_signed_pair((*sfont).handle);
            p = p.offset(
                put_big_endian(p as *mut libc::c_void, number_of_contours as i32, 2i32) as isize,
            );
            /* BoundingBox: FWord x 4 */
            (*(*g).gd.offset(i as isize)).llx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).lly = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).urx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).ury = tt_get_signed_pair((*sfont).handle);
            /* _FIXME_ */
            if vmtx.is_null() {
                /* vertOriginY == sTypeAscender */
                (*(*g).gd.offset(i as isize)).tsb = ((*g).default_advh as i32
                    - (*g).default_tsb as i32
                    - (*(*g).gd.offset(i as isize)).ury as i32)
                    as i16
            }
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).llx as i32,
                2i32,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).lly as i32,
                2i32,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).urx as i32,
                2i32,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).ury as i32,
                2i32,
            ) as isize);
            /* Read evrything else. */
            ttstub_input_read(
                (*sfont).handle,
                p as *mut i8,
                len.wrapping_sub(10_u32) as size_t,
            );
            /*
             * Fix GIDs of composite glyphs.
             */
            if (number_of_contours as i32) < 0i32 {
                let mut flags: u16 = 0; /* flag, gid of a component */
                let mut cgid: u16 = 0;
                let mut new_gid: u16 = 0;
                loop {
                    if p >= endptr {
                        _tt_abort(
                            b"Invalid TrueType glyph data (gid %u): %u bytes\x00" as *const u8
                                as *const i8,
                            gid as i32,
                            len,
                        );
                    }
                    /*
                     * Flags and gid of component glyph are both u16.
                     */
                    flags = ((*p as i32) << 8i32 | *p.offset(1) as i32) as u16;
                    p = p.offset(2);
                    cgid = ((*p as i32) << 8i32 | *p.offset(1) as i32) as u16;
                    if cgid as i32 >= (*maxp).numGlyphs as i32 {
                        _tt_abort(
                            b"Invalid gid (%u > %u) in composite glyph %u.\x00" as *const u8
                                as *const i8,
                            cgid as i32,
                            (*maxp).numGlyphs as i32,
                            gid as i32,
                        );
                    }
                    new_gid = tt_find_glyph(g, cgid);
                    if new_gid as i32 == 0i32 {
                        new_gid = tt_add_glyph(g, cgid, find_empty_slot(g))
                    }
                    p = p.offset(
                        put_big_endian(p as *mut libc::c_void, new_gid as i32, 2i32) as isize
                    );
                    /*
                     * Just skip remaining part.
                     */
                    p = p.offset(
                        (if flags as i32 & 1i32 << 0i32 != 0 {
                            4i32
                        } else {
                            2i32
                        }) as isize,
                    );
                    if flags as i32 & 1i32 << 3i32 != 0 {
                        /* F2Dot14 */
                        p = p.offset(2)
                    } else if flags as i32 & 1i32 << 6i32 != 0 {
                        /* F2Dot14 x 2 */
                        p = p.offset(4)
                    } else if flags as i32 & 1i32 << 7i32 != 0 {
                        /* F2Dot14 x 4 */
                        p = p.offset(8)
                    }
                    if !(flags as i32 & 1i32 << 5i32 != 0) {
                        break;
                    }
                }
            }
        }
        /* Does not contains any data. */
        i += 1
    }
    free(location as *mut libc::c_void);
    free(hmtx as *mut libc::c_void);
    free(vmtx as *mut libc::c_void);
    let mut max_count: i32 = -1i32;
    (*g).dw = (*(*g).gd.offset(0)).advw;
    i = 0i32;
    while i < (*g).emsize as i32 + 1i32 {
        if *w_stat.offset(i as isize) as i32 > max_count {
            max_count = *w_stat.offset(i as isize) as i32;
            (*g).dw = i as u16
        }
        i += 1
    }
    free(w_stat as *mut libc::c_void);
    qsort(
        (*g).gd as *mut libc::c_void,
        (*g).num_glyphs as size_t,
        ::std::mem::size_of::<tt_glyph_desc>() as u64,
        Some(
            glyf_cmp as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
    let mut prev: u16 = 0;
    let mut last_advw: u16 = 0;
    let mut p_0: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut padlen: i32 = 0;
    let mut num_hm_known: i32 = 0;
    glyf_table_size = 0u64 as u32;
    num_hm_known = 0i32;
    last_advw = (*(*g).gd.offset(((*g).num_glyphs as i32 - 1i32) as isize)).advw;
    i = (*g).num_glyphs as i32 - 1i32;
    while i >= 0i32 {
        padlen = (if (*(*g).gd.offset(i as isize)).length.wrapping_rem(4_u32) != 0 {
            (4_u32).wrapping_sub((*(*g).gd.offset(i as isize)).length.wrapping_rem(4_u32))
        } else {
            0_u32
        }) as i32;
        glyf_table_size = (glyf_table_size as u32).wrapping_add(
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as u32),
        ) as u32 as u32;
        if num_hm_known == 0 && last_advw as i32 != (*(*g).gd.offset(i as isize)).advw as i32 {
            (*hhea).numOfLongHorMetrics = ((*(*g).gd.offset(i as isize)).gid as i32 + 2i32) as u16;
            num_hm_known = 1i32
        }
        i -= 1
    }
    /* All advance widths are same. */
    if num_hm_known == 0 {
        (*hhea).numOfLongHorMetrics = 1_u16
    }
    hmtx_table_size =
        ((*hhea).numOfLongHorMetrics as i32 * 2i32 + ((*g).last_gid as i32 + 1i32) * 2i32) as u32;
    /*
     * Choosing short format does not always give good result
     * when compressed. Sometimes increases size.
     */
    if (glyf_table_size as u64) < 0x20000 {
        (*head).indexToLocFormat = 0_i16;
        loca_table_size = (((*g).last_gid as i32 + 2i32) * 2i32) as u32
    } else {
        (*head).indexToLocFormat = 1_i16;
        loca_table_size = (((*g).last_gid as i32 + 2i32) * 4i32) as u32
    }
    p_0 = new((hmtx_table_size as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    hmtx_table_data = p_0;
    q = new((loca_table_size as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
        as *mut i8;
    loca_table_data = q;
    glyf_table_data =
        new((glyf_table_size as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
    offset = 0u64 as u32;
    prev = 0_u16;
    i = 0i32;
    while i < (*g).num_glyphs as i32 {
        let mut gap: i32 = 0;
        let mut j: i32 = 0;
        gap = (*(*g).gd.offset(i as isize)).gid as i32 - prev as i32 - 1i32;
        j = 1i32;
        while j <= gap {
            if prev as i32 + j == (*hhea).numOfLongHorMetrics as i32 - 1i32 {
                p_0 = p_0.offset(
                    put_big_endian(p_0 as *mut libc::c_void, last_advw as i32, 2i32) as isize,
                )
            } else if prev as i32 + j < (*hhea).numOfLongHorMetrics as i32 {
                p_0 = p_0.offset(put_big_endian(p_0 as *mut libc::c_void, 0i32, 2i32) as isize)
            }
            p_0 = p_0.offset(put_big_endian(p_0 as *mut libc::c_void, 0i32, 2i32) as isize);
            if (*head).indexToLocFormat as i32 == 0i32 {
                q = q.offset(put_big_endian(
                    q as *mut libc::c_void,
                    offset.wrapping_div(2_u32) as u16 as i32,
                    2i32,
                ) as isize)
            } else {
                q = q.offset(put_big_endian(q as *mut libc::c_void, offset as i32, 4i32) as isize)
            }
            j += 1
        }
        padlen = (if (*(*g).gd.offset(i as isize)).length.wrapping_rem(4_u32) != 0 {
            (4_u32).wrapping_sub((*(*g).gd.offset(i as isize)).length.wrapping_rem(4_u32))
        } else {
            0_u32
        }) as i32;
        if ((*(*g).gd.offset(i as isize)).gid as i32) < (*hhea).numOfLongHorMetrics as i32 {
            p_0 = p_0.offset(put_big_endian(
                p_0 as *mut libc::c_void,
                (*(*g).gd.offset(i as isize)).advw as i32,
                2i32,
            ) as isize)
        }
        p_0 = p_0.offset(put_big_endian(
            p_0 as *mut libc::c_void,
            (*(*g).gd.offset(i as isize)).lsb as i32,
            2i32,
        ) as isize);
        if (*head).indexToLocFormat as i32 == 0i32 {
            q = q.offset(put_big_endian(
                q as *mut libc::c_void,
                offset.wrapping_div(2_u32) as u16 as i32,
                2i32,
            ) as isize)
        } else {
            q = q.offset(put_big_endian(q as *mut libc::c_void, offset as i32, 4i32) as isize)
        }
        memset(
            glyf_table_data.offset(offset as isize) as *mut libc::c_void,
            0i32,
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as u32) as u64,
        );
        memcpy(
            glyf_table_data.offset(offset as isize) as *mut libc::c_void,
            (*(*g).gd.offset(i as isize)).data as *const libc::c_void,
            (*(*g).gd.offset(i as isize)).length as u64,
        );
        offset = (offset as u32).wrapping_add(
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as u32),
        ) as u32 as u32;
        prev = (*(*g).gd.offset(i as isize)).gid;
        /* free data here since it consume much memory */
        free((*(*g).gd.offset(i as isize)).data as *mut libc::c_void);
        (*(*g).gd.offset(i as isize)).length = 0_u32;
        let ref mut fresh6 = (*(*g).gd.offset(i as isize)).data;
        *fresh6 = 0 as *mut u8;
        i += 1
    }
    if (*head).indexToLocFormat as i32 == 0i32 {
        q = q.offset(put_big_endian(
            q as *mut libc::c_void,
            offset.wrapping_div(2_u32) as u16 as i32,
            2i32,
        ) as isize)
    } else {
        q = q.offset(put_big_endian(q as *mut libc::c_void, offset as i32, 4i32) as isize)
    }
    sfnt_set_table(
        sfont,
        b"hmtx\x00" as *const u8 as *const i8,
        hmtx_table_data as *mut libc::c_void,
        hmtx_table_size,
    );
    sfnt_set_table(
        sfont,
        b"loca\x00" as *const u8 as *const i8,
        loca_table_data as *mut libc::c_void,
        loca_table_size,
    );
    sfnt_set_table(
        sfont,
        b"glyf\x00" as *const u8 as *const i8,
        glyf_table_data as *mut libc::c_void,
        glyf_table_size,
    );
    (*head).checkSumAdjustment = 0_u32;
    (*maxp).numGlyphs = ((*g).last_gid as i32 + 1i32) as u16;
    /* TODO */
    sfnt_set_table(
        sfont,
        b"maxp\x00" as *const u8 as *const i8,
        tt_pack_maxp_table(maxp) as *mut libc::c_void,
        32u64 as u32,
    );
    sfnt_set_table(
        sfont,
        b"hhea\x00" as *const u8 as *const i8,
        tt_pack_hhea_table(hhea) as *mut libc::c_void,
        36u64 as u32,
    );
    sfnt_set_table(
        sfont,
        b"head\x00" as *const u8 as *const i8,
        tt_pack_head_table(head) as *mut libc::c_void,
        54u64 as u32,
    );
    free(maxp as *mut libc::c_void);
    free(hhea as *mut libc::c_void);
    free(head as *mut libc::c_void);
    free(os2 as *mut libc::c_void);
    0i32
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
/* GID in original font */
/* optimal value for DW */
/* default value */
/* default value */
#[no_mangle]
pub unsafe extern "C" fn tt_get_metrics(mut sfont: *mut sfnt, mut g: *mut tt_glyphs) -> i32 {
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    let mut hmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut vmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
    /* temp */
    let mut location: *mut u32 = 0 as *mut u32;
    let mut offset: u32 = 0;
    let mut i: u32 = 0;
    let mut w_stat: *mut u16 = 0 as *mut u16;
    assert!(!g.is_null());
    if sfont.is_null() || (*sfont).handle.is_null() {
        _tt_abort(b"File not opened.\x00" as *const u8 as *const i8);
    }
    if (*sfont).type_0 != 1i32 << 0i32
        && (*sfont).type_0 != 1i32 << 4i32
        && (*sfont).type_0 != 1i32 << 8i32
    {
        _tt_abort(b"Invalid font type\x00" as *const u8 as *const i8);
    }
    /*
     * Read head, hhea, maxp, loca:
     *
     *   unitsPerEm       --> head
     *   numHMetrics      --> hhea
     *   indexToLocFormat --> head
     *   numGlyphs        --> maxp
     */
    head = tt_read_head_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    if (*hhea).metricDataFormat as i32 != 0i32 {
        _tt_abort(b"Unknown metricDataFormat.\x00" as *const u8 as *const i8);
    }
    (*g).emsize = (*head).unitsPerEm;
    sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const i8);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    os2 = tt_read_os2__table(sfont);
    (*g).default_advh = ((*os2).sTypoAscender as i32 - (*os2).sTypoDescender as i32) as u16;
    (*g).default_tsb = ((*g).default_advh as i32 - (*os2).sTypoAscender as i32) as i16;
    if sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8) > 0_u32 {
        let mut vhea: *mut tt_vhea_table = 0 as *mut tt_vhea_table;
        vhea = tt_read_vhea_table(sfont);
        sfnt_locate_table(sfont, b"vmtx\x00" as *const u8 as *const i8);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        );
        free(vhea as *mut libc::c_void);
    } else {
        vmtx = 0 as *mut tt_longMetrics
    }
    sfnt_locate_table(sfont, b"loca\x00" as *const u8 as *const i8);
    location = new((((*maxp).numGlyphs as i32 + 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u32>() as u64) as u32) as *mut u32;
    if (*head).indexToLocFormat as i32 == 0i32 {
        i = 0_u32;
        while i <= (*maxp).numGlyphs as u32 {
            *location.offset(i as isize) =
                (2_u32).wrapping_mul(tt_get_unsigned_pair((*sfont).handle) as u32);
            i = i.wrapping_add(1)
        }
    } else if (*head).indexToLocFormat as i32 == 1i32 {
        i = 0_u32;
        while i <= (*maxp).numGlyphs as u32 {
            *location.offset(i as isize) = tt_get_unsigned_quad((*sfont).handle);
            i = i.wrapping_add(1)
        }
    } else {
        _tt_abort(b"Unknown IndexToLocFormat.\x00" as *const u8 as *const i8);
    }
    w_stat = new((((*g).emsize as i32 + 2i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32) as *mut u16;
    memset(
        w_stat as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<u16>() as u64).wrapping_mul(((*g).emsize as i32 + 2i32) as u64),
    );
    /*
     * Read glyf table.
     */
    offset = sfnt_locate_table(sfont, b"glyf\x00" as *const u8 as *const i8); /* old gid */
    i = 0_u32;
    while i < (*g).num_glyphs as u32 {
        let mut gid: u16 = 0;
        let mut loc: u32 = 0;
        let mut len: u32 = 0;
        gid = (*(*g).gd.offset(i as isize)).ogid;
        if gid as i32 >= (*maxp).numGlyphs as i32 {
            _tt_abort(
                b"Invalid glyph index (gid %u)\x00" as *const u8 as *const i8,
                gid as i32,
            );
        }
        loc = *location.offset(gid as isize);
        len = (*location.offset((gid as i32 + 1i32) as isize)).wrapping_sub(loc);
        (*(*g).gd.offset(i as isize)).advw = (*hmtx.offset(gid as isize)).advance;
        (*(*g).gd.offset(i as isize)).lsb = (*hmtx.offset(gid as isize)).sideBearing;
        if !vmtx.is_null() {
            (*(*g).gd.offset(i as isize)).advh = (*vmtx.offset(gid as isize)).advance;
            (*(*g).gd.offset(i as isize)).tsb = (*vmtx.offset(gid as isize)).sideBearing
        } else {
            (*(*g).gd.offset(i as isize)).advh = (*g).default_advh;
            (*(*g).gd.offset(i as isize)).tsb = (*g).default_tsb
        }
        (*(*g).gd.offset(i as isize)).length = len;
        let ref mut fresh7 = (*(*g).gd.offset(i as isize)).data;
        *fresh7 = 0 as *mut u8;
        if (*(*g).gd.offset(i as isize)).advw as i32 <= (*g).emsize as i32 {
            let ref mut fresh8 = *w_stat.offset((*(*g).gd.offset(i as isize)).advw as isize);
            *fresh8 = (*fresh8 as i32 + 1i32) as u16
        } else {
            let ref mut fresh9 = *w_stat.offset(((*g).emsize as i32 + 1i32) as isize);
            *fresh9 = (*fresh9 as i32 + 1i32) as u16
            /* larger than em */
        }
        if !(len == 0_u32) {
            if len < 10_u32 {
                _tt_abort(
                    b"Invalid TrueType glyph data (gid %u).\x00" as *const u8 as *const i8,
                    gid as i32,
                );
            }
            ttstub_input_seek((*sfont).handle, offset.wrapping_add(loc) as ssize_t, 0i32);
            tt_get_signed_pair((*sfont).handle);
            /* BoundingBox: FWord x 4 */
            (*(*g).gd.offset(i as isize)).llx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).lly = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).urx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).ury = tt_get_signed_pair((*sfont).handle);
            /* _FIXME_ */
            if vmtx.is_null() {
                /* vertOriginY == sTypeAscender */
                (*(*g).gd.offset(i as isize)).tsb = ((*g).default_advh as i32
                    - (*g).default_tsb as i32
                    - (*(*g).gd.offset(i as isize)).ury as i32)
                    as i16
            }
        }
        /* Does not contains any data. */
        i = i.wrapping_add(1)
    }
    free(location as *mut libc::c_void);
    free(hmtx as *mut libc::c_void);
    free(maxp as *mut libc::c_void);
    free(hhea as *mut libc::c_void);
    free(head as *mut libc::c_void);
    free(os2 as *mut libc::c_void);
    free(vmtx as *mut libc::c_void);
    let mut max_count: i32 = -1i32;
    (*g).dw = (*(*g).gd.offset(0)).advw;
    i = 0_u32;
    while i < ((*g).emsize as i32 + 1i32) as u32 {
        if *w_stat.offset(i as isize) as i32 > max_count {
            max_count = *w_stat.offset(i as isize) as i32;
            (*g).dw = i as u16
        }
        i = i.wrapping_add(1)
    }
    free(w_stat as *mut libc::c_void);
    0i32
}
