#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
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
    pub type otl_opt;
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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn otl_new_opt() -> *mut otl_opt;
    #[no_mangle]
    fn otl_release_opt(opt: *mut otl_opt);
    #[no_mangle]
    fn otl_parse_optstring(opt: *mut otl_opt, optstr: *const i8) -> i32;
    #[no_mangle]
    fn otl_match_optrule(opt: *mut otl_opt, tag: *const i8) -> i32;
    #[no_mangle]
    fn tt_get_signed_byte(handle: rust_input_handle_t) -> i8;
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16;
    #[no_mangle]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> i16;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const i8) -> u32;
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
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type Fixed = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [i8; 4],
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut i8,
    /* table data */
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
/* sfnt resource */
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
pub struct otl_gsub {
    pub num_gsubs: i32,
    pub select: i32,
    pub first: *mut gsub_entry,
    pub gsubs: [otl_gsub_tab; 32],
    /* _TT_GSUB_H_ */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_tab {
    pub script: *mut i8,
    pub language: *mut i8,
    pub feature: *mut i8,
    pub num_subtables: i32,
    pub subtables: *mut otl_gsub_subtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_subtab {
    pub LookupType: u16,
    pub SubstFormat: u16,
    pub table: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub single1: *mut otl_gsub_single1,
    pub single2: *mut otl_gsub_single2,
    pub alternate1: *mut otl_gsub_alternate1,
    pub ligature1: *mut otl_gsub_ligature1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_ligature1 {
    pub LigSetCount: u16,
    pub LigatureSet: *mut otl_gsub_ligset,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_coverage {
    pub format: u16,
    pub count: u16,
    pub list: *mut GlyphID,
    pub range: *mut clt_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_range {
    pub Start: GlyphID,
    pub End: GlyphID,
    pub StartCoverageIndex: u16,
}
pub type GlyphID = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_ligset {
    pub LigatureCount: u16,
    pub Ligature: *mut otl_gsub_ligtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_ligtab {
    pub LigGlyph: GlyphID,
    pub CompCount: u16,
    pub Component: *mut GlyphID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_alternate1 {
    pub AlternateSetCount: u16,
    pub AlternateSet: *mut otl_gsub_altset,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_altset {
    pub GlyphCount: u16,
    pub Alternate: *mut GlyphID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_single2 {
    pub GlyphCount: u16,
    pub Substitute: *mut GlyphID,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_single1 {
    pub DeltaGlyphID: i16,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsub_entry {
    pub index: i32,
    pub next: *mut gsub_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_record {
    pub tag: [i8; 5],
    pub offset: Offset,
}
pub type Offset = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_record_list {
    pub count: u16,
    pub record: *mut clt_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_number_list {
    pub count: u16,
    pub value: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_feature_table {
    pub FeatureParams: Offset,
    pub LookupListIndex: clt_number_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_lookup_table {
    pub LookupType: u16,
    pub LookupFlag: u16,
    pub SubTableList: clt_number_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_header {
    pub version: Fixed,
    pub ScriptList: Offset,
    pub FeatureList: Offset,
    pub LookupList: Offset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_script_table {
    pub DefaultLangSys: Offset,
    pub LangSysRecord: clt_record_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_langsys_table {
    pub LookupOrder: Offset,
    pub ReqFeatureIndex: u16,
    pub FeatureIndex: clt_number_list,
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
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_set_verbose(mut level: i32) {
    verbose = level;
}
unsafe extern "C" fn clt_read_record(mut rec: *mut clt_record, mut sfont: *mut sfnt) -> i32 {
    let mut i: i32 = 0;
    if !rec.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"rec && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            71_u32,
            (*::std::mem::transmute::<&[u8; 49], &[i8; 49]>(
                b"int clt_read_record(struct clt_record *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0i32;
    while i < 4i32 {
        (*rec).tag[i as usize] = tt_get_signed_byte((*sfont).handle) as i8;
        i += 1
    }
    (*rec).tag[4] = '\u{0}' as i32 as i8;
    (*rec).offset = tt_get_unsigned_pair((*sfont).handle);
    return 6i32;
}
unsafe extern "C" fn clt_read_range(mut rec: *mut clt_range, mut sfont: *mut sfnt) -> i32 {
    if !rec.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"rec && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            85_u32,
            (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                b"int clt_read_range(struct clt_range *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*rec).Start = tt_get_unsigned_pair((*sfont).handle);
    (*rec).End = tt_get_unsigned_pair((*sfont).handle);
    (*rec).StartCoverageIndex = tt_get_unsigned_pair((*sfont).handle);
    return 6i32;
}
unsafe extern "C" fn clt_read_record_list(
    mut list: *mut clt_record_list,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    if !list.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"list && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            117_u32,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int clt_read_record_list(struct clt_record_list *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*list).count = tt_get_unsigned_pair((*sfont).handle);
    len = 2i32;
    if (*list).count as i32 == 0i32 {
        (*list).record = 0 as *mut clt_record
    } else {
        (*list).record = new(((*list).count as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<clt_record>() as u64)
            as u32) as *mut clt_record;
        i = 0i32;
        while i < (*list).count as i32 {
            len += clt_read_record(&mut *(*list).record.offset(i as isize), sfont);
            i += 1
        }
    }
    return len;
}
unsafe extern "C" fn clt_release_record_list(mut list: *mut clt_record_list) {
    if !list.is_null() {
        (*list).record = mfree((*list).record as *mut libc::c_void) as *mut clt_record;
        (*list).count = 0_u16
    };
}
unsafe extern "C" fn clt_read_number_list(
    mut list: *mut clt_number_list,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut i: i32 = 0;
    if !list.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"list && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            148_u32,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int clt_read_number_list(struct clt_number_list *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*list).count = tt_get_unsigned_pair((*sfont).handle);
    if (*list).count as i32 == 0i32 {
        (*list).value = 0 as *mut u16
    } else {
        (*list).value = new(
            ((*list).count as u32 as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64) as u32
        ) as *mut u16;
        i = 0i32;
        while i < (*list).count as i32 {
            *(*list).value.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
            i += 1
        }
    }
    return 2i32 + 2i32 * (*list).count as i32;
}
unsafe extern "C" fn clt_release_number_list(mut list: *mut clt_number_list) {
    if !list.is_null() {
        (*list).value = mfree((*list).value as *mut libc::c_void) as *mut u16;
        (*list).count = 0_u16
    };
}
unsafe extern "C" fn clt_read_script_table(
    mut tab: *mut clt_script_table,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    if !tab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"tab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            283_u32,
            (*::std::mem::transmute::<&[u8; 61], &[i8; 61]>(
                b"int clt_read_script_table(struct clt_script_table *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*tab).DefaultLangSys = tt_get_unsigned_pair((*sfont).handle);
    len = 2i32;
    len += clt_read_record_list(&mut (*tab).LangSysRecord, sfont);
    return len;
}
unsafe extern "C" fn clt_release_script_table(mut tab: *mut clt_script_table) {
    if !tab.is_null() {
        clt_release_record_list(&mut (*tab).LangSysRecord);
    };
}
unsafe extern "C" fn clt_read_langsys_table(
    mut tab: *mut clt_langsys_table,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    if !tab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"tab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            314_u32,
            (*::std::mem::transmute::<&[u8; 63], &[i8; 63]>(
                b"int clt_read_langsys_table(struct clt_langsys_table *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*tab).LookupOrder = tt_get_unsigned_pair((*sfont).handle);
    (*tab).ReqFeatureIndex = tt_get_unsigned_pair((*sfont).handle);
    len = 4i32;
    len += clt_read_number_list(&mut (*tab).FeatureIndex, sfont);
    return len;
}
unsafe extern "C" fn clt_release_langsys_table(mut tab: *mut clt_langsys_table) {
    if !tab.is_null() {
        clt_release_number_list(&mut (*tab).FeatureIndex);
    };
}
unsafe extern "C" fn clt_read_feature_table(
    mut tab: *mut clt_feature_table,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    if !tab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"tab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            344_u32,
            (*::std::mem::transmute::<&[u8; 63], &[i8; 63]>(
                b"int clt_read_feature_table(struct clt_feature_table *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*tab).FeatureParams = tt_get_unsigned_pair((*sfont).handle);
    len = 2i32;
    len += clt_read_number_list(&mut (*tab).LookupListIndex, sfont);
    return len;
}
unsafe extern "C" fn clt_release_feature_table(mut tab: *mut clt_feature_table) {
    if !tab.is_null() {
        clt_release_number_list(&mut (*tab).LookupListIndex);
    };
}
unsafe extern "C" fn clt_read_lookup_table(
    mut tab: *mut clt_lookup_table,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    if !tab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"tab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            377_u32,
            (*::std::mem::transmute::<&[u8; 61], &[i8; 61]>(
                b"int clt_read_lookup_table(struct clt_lookup_table *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*tab).LookupType = tt_get_unsigned_pair((*sfont).handle);
    (*tab).LookupFlag = tt_get_unsigned_pair((*sfont).handle);
    len = 4i32;
    len += clt_read_number_list(&mut (*tab).SubTableList, sfont);
    return len;
}
unsafe extern "C" fn clt_release_lookup_table(mut tab: *mut clt_lookup_table) {
    if !tab.is_null() {
        clt_release_number_list(&mut (*tab).SubTableList);
    };
}
unsafe extern "C" fn clt_read_coverage(mut cov: *mut clt_coverage, mut sfont: *mut sfnt) -> i32 {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    if !cov.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"cov && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            399_u32,
            (*::std::mem::transmute::<&[u8; 53], &[i8; 53]>(
                b"int clt_read_coverage(struct clt_coverage *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*cov).format = tt_get_unsigned_pair((*sfont).handle);
    (*cov).count = tt_get_unsigned_pair((*sfont).handle);
    len = 4i32;
    match (*cov).format as i32 {
        1 => {
            if (*cov).count as i32 == 0i32 {
                (*cov).list = 0 as *mut GlyphID
            } else {
                (*cov).list = new(((*cov).count as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<u16>() as u64)
                    as u32) as *mut u16;
                i = 0i32;
                while i < (*cov).count as i32 {
                    *(*cov).list.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
                    i += 1
                }
            }
            (*cov).range = 0 as *mut clt_range;
            len += 2i32 * (*cov).count as i32
        }
        2 => {
            if (*cov).count as i32 == 0i32 {
                (*cov).range = 0 as *mut clt_range
            } else {
                (*cov).range = new(((*cov).count as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<clt_range>() as u64)
                    as u32) as *mut clt_range;
                i = 0i32;
                while i < (*cov).count as i32 {
                    len += clt_read_range(&mut *(*cov).range.offset(i as isize), sfont);
                    i += 1
                }
            }
            (*cov).list = 0 as *mut GlyphID
        }
        _ => {
            _tt_abort(b"Unknown coverage format\x00" as *const u8 as *const i8);
        }
    }
    return len;
}
unsafe extern "C" fn clt_release_coverage(mut cov: *mut clt_coverage) {
    if !cov.is_null() {
        match (*cov).format as i32 {
            1 => (*cov).list = mfree((*cov).list as *mut libc::c_void) as *mut GlyphID,
            2 => (*cov).range = mfree((*cov).range as *mut libc::c_void) as *mut clt_range,
            _ => {
                _tt_abort(b"Unknown coverage format\x00" as *const u8 as *const i8);
            }
        }
    }
    (*cov).count = 0_u16;
}
unsafe extern "C" fn clt_lookup_coverage(mut cov: *mut clt_coverage, mut gid: u16) -> i32 {
    let mut i: i32 = 0;
    if !cov.is_null() {
    } else {
        __assert_fail(
            b"cov\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            460_u32,
            (*::std::mem::transmute::<&[u8; 55], &[i8; 55]>(
                b"int clt_lookup_coverage(struct clt_coverage *, USHORT)\x00",
            ))
            .as_ptr(),
        );
    }
    match (*cov).format as i32 {
        1 => {
            i = 0i32;
            while i < (*cov).count as i32 {
                if *(*cov).list.offset(i as isize) as i32 > gid as i32 {
                    break;
                }
                if *(*cov).list.offset(i as isize) as i32 == gid as i32 {
                    return i;
                }
                i += 1
            }
        }
        2 => {
            i = 0i32;
            while i < (*cov).count as i32 {
                if (gid as i32) < (*(*cov).range.offset(i as isize)).Start as i32 {
                    break;
                }
                if gid as i32 <= (*(*cov).range.offset(i as isize)).End as i32 {
                    return (*(*cov).range.offset(i as isize)).StartCoverageIndex as i32
                        + gid as i32
                        - (*(*cov).range.offset(i as isize)).Start as i32;
                }
                i += 1
            }
        }
        _ => {
            _tt_abort(b"Unknown coverage format\x00" as *const u8 as *const i8);
        }
    }
    return -1i32;
}
unsafe extern "C" fn otl_gsub_read_single(
    mut subtab: *mut otl_gsub_subtab,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    let mut offset: u32 = 0;
    let mut cov_offset: Offset = 0;
    if !subtab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"subtab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            496_u32,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int otl_gsub_read_single(struct otl_gsub_subtab *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    offset = ttstub_input_seek((*sfont).handle, 0i32 as ssize_t, 1i32) as u32;
    (*subtab).LookupType = 1_u16;
    (*subtab).SubstFormat = tt_get_unsigned_pair((*sfont).handle);
    len = 2i32;
    if (*subtab).SubstFormat as i32 == 1i32 {
        let mut data: *mut otl_gsub_single1 = 0 as *mut otl_gsub_single1;
        data = new((1_u64).wrapping_mul(::std::mem::size_of::<otl_gsub_single1>() as u64) as u32)
            as *mut otl_gsub_single1;
        (*subtab).table.single1 = data;
        cov_offset = tt_get_unsigned_pair((*sfont).handle);
        (*data).DeltaGlyphID = tt_get_signed_pair((*sfont).handle);
        len += 4i32;
        ttstub_input_seek(
            (*sfont).handle,
            offset.wrapping_add(cov_offset as u32) as ssize_t,
            0i32,
        );
        len += clt_read_coverage(&mut (*data).coverage, sfont)
    } else if (*subtab).SubstFormat as i32 == 2i32 {
        let mut data_0: *mut otl_gsub_single2 = 0 as *mut otl_gsub_single2;
        let mut count: u16 = 0;
        data_0 = new((1_u64).wrapping_mul(::std::mem::size_of::<otl_gsub_single2>() as u64) as u32)
            as *mut otl_gsub_single2;
        (*subtab).table.single2 = data_0;
        cov_offset = tt_get_unsigned_pair((*sfont).handle);
        (*data_0).GlyphCount = tt_get_unsigned_pair((*sfont).handle);
        len += 4i32;
        if (*data_0).GlyphCount as i32 == 0i32 {
            (*data_0).Substitute = 0 as *mut GlyphID
        } else {
            (*data_0).Substitute = new(((*data_0).GlyphCount as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<GlyphID>() as u64)
                as u32) as *mut GlyphID;
            count = 0_u16;
            while (count as i32) < (*data_0).GlyphCount as i32 {
                *(*data_0).Substitute.offset(count as isize) =
                    tt_get_unsigned_pair((*sfont).handle);
                count = count.wrapping_add(1)
            }
            len += 2i32 * (*data_0).GlyphCount as i32
        }
        ttstub_input_seek(
            (*sfont).handle,
            offset.wrapping_add(cov_offset as u32) as ssize_t,
            0i32,
        );
        len += clt_read_coverage(&mut (*data_0).coverage, sfont)
    } else {
        _tt_abort(b"unexpected SubstFormat\x00" as *const u8 as *const i8);
    }
    return len;
}
unsafe extern "C" fn otl_gsub_read_alternate(
    mut subtab: *mut otl_gsub_subtab,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut offset: u32 = 0;
    let mut cov_offset: Offset = 0;
    let mut altset_offsets: clt_number_list = clt_number_list {
        count: 0,
        value: 0 as *mut u16,
    };
    let mut data: *mut otl_gsub_alternate1 = 0 as *mut otl_gsub_alternate1;
    if !subtab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"subtab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            555_u32,
            (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                b"int otl_gsub_read_alternate(struct otl_gsub_subtab *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    offset = ttstub_input_seek((*sfont).handle, 0i32 as ssize_t, 1i32) as u32;
    (*subtab).LookupType = 3_u16;
    (*subtab).SubstFormat = tt_get_unsigned_pair((*sfont).handle);
    if (*subtab).SubstFormat as i32 != 1i32 {
        dpx_warning(
            b"Unknown GSUB SubstFormat for Alternate: %u\x00" as *const u8 as *const i8,
            (*subtab).SubstFormat as i32,
        );
        return -1i32;
    }
    len = 2i32;
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<otl_gsub_alternate1>() as u64) as u32)
        as *mut otl_gsub_alternate1;
    (*subtab).table.alternate1 = data;
    cov_offset = tt_get_unsigned_pair((*sfont).handle);
    len += 2i32;
    len += clt_read_number_list(&mut altset_offsets, sfont);
    (*data).AlternateSetCount = altset_offsets.count;
    if (*data).AlternateSetCount as i32 == 0i32 {
        (*data).AlternateSet = 0 as *mut otl_gsub_altset;
        (*data).coverage.count = 0_u16;
        (*data).coverage.format = 0_u16;
        (*data).coverage.list = 0 as *mut GlyphID;
        return len;
    }
    (*data).AlternateSet = new(((*data).AlternateSetCount as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<otl_gsub_altset>() as u64)
        as u32) as *mut otl_gsub_altset;
    i = 0_u16;
    while (i as i32) < (*data).AlternateSetCount as i32 {
        let mut altset: *mut otl_gsub_altset = 0 as *mut otl_gsub_altset;
        let mut altset_offset: u32 = 0;
        altset = &mut *(*data).AlternateSet.offset(i as isize) as *mut otl_gsub_altset;
        altset_offset = offset.wrapping_add(*altset_offsets.value.offset(i as isize) as u32);
        ttstub_input_seek((*sfont).handle, altset_offset as ssize_t, 0i32);
        (*altset).GlyphCount = tt_get_unsigned_pair((*sfont).handle);
        len += 2i32;
        if (*altset).GlyphCount as i32 == 0i32 {
            (*altset).Alternate = 0 as *mut GlyphID;
            break;
        } else {
            (*altset).Alternate = new(((*altset).GlyphCount as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<GlyphID>() as u64)
                as u32) as *mut GlyphID;
            j = 0_u16;
            while (j as i32) < (*altset).GlyphCount as i32 {
                *(*altset).Alternate.offset(j as isize) = tt_get_unsigned_pair((*sfont).handle);
                len += 2i32;
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
    }
    clt_release_number_list(&mut altset_offsets);
    ttstub_input_seek(
        (*sfont).handle,
        offset.wrapping_add(cov_offset as u32) as ssize_t,
        0i32,
    );
    len += clt_read_coverage(&mut (*data).coverage, sfont);
    return len;
}
unsafe extern "C" fn otl_gsub_read_ligature(
    mut subtab: *mut otl_gsub_subtab,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut len: i32 = 0;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut offset: u32 = 0;
    let mut cov_offset: Offset = 0;
    let mut ligset_offsets: clt_number_list = clt_number_list {
        count: 0,
        value: 0 as *mut u16,
    };
    let mut data: *mut otl_gsub_ligature1 = 0 as *mut otl_gsub_ligature1;
    if !subtab.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"subtab && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            622_u32,
            (*::std::mem::transmute::<&[u8; 61], &[i8; 61]>(
                b"int otl_gsub_read_ligature(struct otl_gsub_subtab *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    offset = ttstub_input_seek((*sfont).handle, 0i32 as ssize_t, 1i32) as u32;
    (*subtab).LookupType = 4_u16;
    (*subtab).SubstFormat = tt_get_unsigned_pair((*sfont).handle);
    if (*subtab).SubstFormat as i32 != 1i32 {
        dpx_warning(
            b"Unknown GSUB SubstFormat for Ligature: %u\x00" as *const u8 as *const i8,
            (*subtab).SubstFormat as i32,
        );
        return -1i32;
    }
    len = 2i32;
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<otl_gsub_ligature1>() as u64) as u32)
        as *mut otl_gsub_ligature1;
    (*subtab).table.ligature1 = data;
    cov_offset = tt_get_unsigned_pair((*sfont).handle);
    len += 2i32;
    len += clt_read_number_list(&mut ligset_offsets, sfont);
    (*data).LigSetCount = ligset_offsets.count;
    if (*data).LigSetCount as i32 == 0i32 {
        (*data).LigatureSet = 0 as *mut otl_gsub_ligset;
        (*data).coverage.count = 0_u16;
        (*data).coverage.format = 0_u16;
        (*data).coverage.list = 0 as *mut GlyphID;
        return len;
    }
    (*data).LigatureSet = new(((*data).LigSetCount as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<otl_gsub_ligset>() as u64)
        as u32) as *mut otl_gsub_ligset;
    i = 0_u16;
    while (i as i32) < (*data).LigSetCount as i32 {
        let mut ligset_tab: clt_number_list = clt_number_list {
            count: 0,
            value: 0 as *mut u16,
        };
        let mut ligset: *mut otl_gsub_ligset = 0 as *mut otl_gsub_ligset;
        let mut ligset_offset: u32 = 0;
        let mut count: u16 = 0;
        ligset = &mut *(*data).LigatureSet.offset(i as isize) as *mut otl_gsub_ligset;
        ligset_offset = offset.wrapping_add(*ligset_offsets.value.offset(i as isize) as u32);
        ttstub_input_seek((*sfont).handle, ligset_offset as ssize_t, 0i32);
        len += clt_read_number_list(&mut ligset_tab, sfont);
        (*ligset).LigatureCount = ligset_tab.count;
        if ligset_tab.count as i32 == 0i32 {
            (*ligset).Ligature = 0 as *mut otl_gsub_ligtab;
            break;
        } else {
            (*ligset).Ligature = new((ligset_tab.count as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<otl_gsub_ligtab>() as u64)
                as u32) as *mut otl_gsub_ligtab;
            j = 0_u16;
            while (j as i32) < ligset_tab.count as i32 {
                ttstub_input_seek(
                    (*sfont).handle,
                    ligset_offset.wrapping_add(*ligset_tab.value.offset(j as isize) as u32)
                        as ssize_t,
                    0i32,
                );
                (*(*ligset).Ligature.offset(j as isize)).LigGlyph =
                    tt_get_unsigned_pair((*sfont).handle);
                (*(*ligset).Ligature.offset(j as isize)).CompCount =
                    tt_get_unsigned_pair((*sfont).handle);
                if (*(*ligset).Ligature.offset(j as isize)).CompCount as i32 == 0i32 {
                    let ref mut fresh0 = (*(*ligset).Ligature.offset(j as isize)).Component;
                    *fresh0 = 0 as *mut GlyphID;
                    break;
                } else {
                    let ref mut fresh1 = (*(*ligset).Ligature.offset(j as isize)).Component;
                    *fresh1 = new((((*(*ligset).Ligature.offset(j as isize)).CompCount as i32
                        - 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<GlyphID>() as u64)
                        as u32) as *mut GlyphID;
                    count = 0_u16;
                    while (count as i32)
                        < (*(*ligset).Ligature.offset(j as isize)).CompCount as i32 - 1i32
                    {
                        *(*(*ligset).Ligature.offset(j as isize))
                            .Component
                            .offset(count as isize) = tt_get_unsigned_pair((*sfont).handle);
                        count = count.wrapping_add(1)
                    }
                    len += 4i32 + count as i32 * 2i32;
                    j = j.wrapping_add(1)
                }
            }
            clt_release_number_list(&mut ligset_tab);
            i = i.wrapping_add(1)
        }
    }
    clt_release_number_list(&mut ligset_offsets);
    ttstub_input_seek(
        (*sfont).handle,
        offset.wrapping_add(cov_offset as u32) as ssize_t,
        0i32,
    );
    len += clt_read_coverage(&mut (*data).coverage, sfont);
    return len;
}
unsafe extern "C" fn otl_gsub_release_single(mut subtab: *mut otl_gsub_subtab) {
    if !subtab.is_null() {
        match (*subtab).SubstFormat as i32 {
            1 => {
                let mut data: *mut otl_gsub_single1 = 0 as *mut otl_gsub_single1;
                data = (*subtab).table.single1;
                if !data.is_null() {
                    clt_release_coverage(&mut (*data).coverage);
                    free(data as *mut libc::c_void);
                }
                (*subtab).table.single1 = 0 as *mut otl_gsub_single1
            }
            2 => {
                let mut data_0: *mut otl_gsub_single2 = 0 as *mut otl_gsub_single2;
                data_0 = (*subtab).table.single2;
                if !data_0.is_null() {
                    free((*data_0).Substitute as *mut libc::c_void);
                    clt_release_coverage(&mut (*data_0).coverage);
                    free(data_0 as *mut libc::c_void);
                }
                (*subtab).table.single2 = 0 as *mut otl_gsub_single2
            }
            _ => {
                _tt_abort(b"Unknown format for single substitution\x00" as *const u8 as *const i8);
            }
        }
    };
}
unsafe extern "C" fn otl_gsub_release_ligature(mut subtab: *mut otl_gsub_subtab) {
    if !subtab.is_null() {
        let mut data: *mut otl_gsub_ligature1 = 0 as *mut otl_gsub_ligature1;
        let mut i: u16 = 0;
        let mut j: u16 = 0;
        data = (*subtab).table.ligature1;
        if !data.is_null() && !(*data).LigatureSet.is_null() {
            i = 0_u16;
            while (i as i32) < (*data).LigSetCount as i32 {
                let mut ligset: *mut otl_gsub_ligset = 0 as *mut otl_gsub_ligset;
                ligset = &mut *(*data).LigatureSet.offset(i as isize) as *mut otl_gsub_ligset;
                j = 0_u16;
                while (j as i32) < (*ligset).LigatureCount as i32 {
                    let ref mut fresh2 = (*(*ligset).Ligature.offset(j as isize)).Component;
                    *fresh2 = mfree(
                        (*(*ligset).Ligature.offset(j as isize)).Component as *mut libc::c_void,
                    ) as *mut GlyphID;
                    j = j.wrapping_add(1)
                }
                (*ligset).Ligature =
                    mfree((*ligset).Ligature as *mut libc::c_void) as *mut otl_gsub_ligtab;
                i = i.wrapping_add(1)
            }
            free((*data).LigatureSet as *mut libc::c_void);
        }
        clt_release_coverage(&mut (*data).coverage);
        (*data).LigatureSet = 0 as *mut otl_gsub_ligset;
        free(data as *mut libc::c_void);
        (*subtab).table.ligature1 = 0 as *mut otl_gsub_ligature1
    };
}
unsafe extern "C" fn otl_gsub_release_alternate(mut subtab: *mut otl_gsub_subtab) {
    if !subtab.is_null() {
        let mut data: *mut otl_gsub_alternate1 = 0 as *mut otl_gsub_alternate1;
        let mut i: u16 = 0;
        data = (*subtab).table.alternate1;
        if !data.is_null() && !(*data).AlternateSet.is_null() {
            i = 0_u16;
            while (i as i32) < (*data).AlternateSetCount as i32 {
                let mut altset: *mut otl_gsub_altset = 0 as *mut otl_gsub_altset;
                altset = &mut *(*data).AlternateSet.offset(i as isize) as *mut otl_gsub_altset;
                (*altset).Alternate =
                    mfree((*altset).Alternate as *mut libc::c_void) as *mut GlyphID;
                i = i.wrapping_add(1)
            }
            free((*data).AlternateSet as *mut libc::c_void);
        }
        clt_release_coverage(&mut (*data).coverage);
        (*data).AlternateSet = 0 as *mut otl_gsub_altset;
        free(data as *mut libc::c_void);
        (*subtab).table.alternate1 = 0 as *mut otl_gsub_alternate1
    };
}
unsafe extern "C" fn otl_gsub_read_header(
    mut head: *mut otl_gsub_header,
    mut sfont: *mut sfnt,
) -> i32 {
    if !head.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"head && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            787_u32,
            (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                b"int otl_gsub_read_header(struct otl_gsub_header *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*head).version = tt_get_unsigned_quad((*sfont).handle);
    (*head).ScriptList = tt_get_unsigned_pair((*sfont).handle);
    (*head).FeatureList = tt_get_unsigned_pair((*sfont).handle);
    (*head).LookupList = tt_get_unsigned_pair((*sfont).handle);
    return 10i32;
}
unsafe extern "C" fn otl_gsub_read_feat(mut gsub: *mut otl_gsub_tab, mut sfont: *mut sfnt) -> i32 {
    let mut feat_idx: i32 = 0;
    let mut script_idx: i32 = 0;
    let mut gsub_offset: u32 = 0;
    let mut offset: u32 = 0;
    let mut head: otl_gsub_header = otl_gsub_header {
        version: 0,
        ScriptList: 0,
        FeatureList: 0,
        LookupList: 0,
    };
    let mut subtab: *mut otl_gsub_subtab = 0 as *mut otl_gsub_subtab;
    let mut num_subtabs: u16 = 0_u16;
    let mut feat_bits: [u8; 8192] = [0; 8192];
    let mut feature_list: clt_record_list = clt_record_list {
        count: 0,
        record: 0 as *mut clt_record,
    };
    let mut script_list: clt_record_list = clt_record_list {
        count: 0,
        record: 0 as *mut clt_record,
    };
    let mut lookup_list: clt_number_list = clt_number_list {
        count: 0,
        value: 0 as *mut u16,
    };
    let mut script: *mut otl_opt = 0 as *mut otl_opt;
    let mut language: *mut otl_opt = 0 as *mut otl_opt;
    let mut feature: *mut otl_opt = 0 as *mut otl_opt;
    if !gsub.is_null() && !sfont.is_null() {
    } else {
        __assert_fail(
            b"gsub && sfont\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            830_u32,
            (*::std::mem::transmute::<&[u8; 54], &[i8; 54]>(
                b"int otl_gsub_read_feat(struct otl_gsub_tab *, sfnt *)\x00",
            ))
            .as_ptr(),
        );
    }
    gsub_offset = sfnt_find_table_pos(sfont, b"GSUB\x00" as *const u8 as *const i8);
    if gsub_offset == 0_u32 {
        return -1i32;
    }
    script = otl_new_opt();
    otl_parse_optstring(script, (*gsub).script);
    language = otl_new_opt();
    otl_parse_optstring(language, (*gsub).language);
    feature = otl_new_opt();
    otl_parse_optstring(feature, (*gsub).feature);
    memset(
        feat_bits.as_mut_ptr() as *mut libc::c_void,
        0i32,
        8192i32 as u64,
    );
    ttstub_input_seek((*sfont).handle, gsub_offset as ssize_t, 0i32);
    otl_gsub_read_header(&mut head, sfont);
    offset = gsub_offset.wrapping_add(head.ScriptList as u32);
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    clt_read_record_list(&mut script_list, sfont);
    script_idx = 0i32;
    while script_idx < script_list.count as i32 {
        if otl_match_optrule(
            script,
            (*script_list.record.offset(script_idx as isize))
                .tag
                .as_mut_ptr(),
        ) != 0
        {
            let mut script_tab: clt_script_table = clt_script_table {
                DefaultLangSys: 0,
                LangSysRecord: clt_record_list {
                    count: 0,
                    record: 0 as *mut clt_record,
                },
            };
            let mut langsys_idx: i32 = 0;
            offset = gsub_offset
                .wrapping_add(head.ScriptList as u32)
                .wrapping_add((*script_list.record.offset(script_idx as isize)).offset as u32);
            ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
            clt_read_script_table(&mut script_tab, sfont);
            if otl_match_optrule(language, b"dflt\x00" as *const u8 as *const i8) != 0
                && script_tab.DefaultLangSys as i32 != 0i32
            {
                let mut langsys_tab: clt_langsys_table = clt_langsys_table {
                    LookupOrder: 0,
                    ReqFeatureIndex: 0,
                    FeatureIndex: clt_number_list {
                        count: 0,
                        value: 0 as *mut u16,
                    },
                };
                if verbose > 0i32 {
                    dpx_message(
                        b"otl_gsub>> OTL script-language enabled: %c%c%c%c.dflt\n\x00" as *const u8
                            as *const i8,
                        (*script_list.record.offset(script_idx as isize)).tag[0] as i32,
                        (*script_list.record.offset(script_idx as isize)).tag[1] as i32,
                        (*script_list.record.offset(script_idx as isize)).tag[2] as i32,
                        (*script_list.record.offset(script_idx as isize)).tag[3] as i32,
                    );
                }
                ttstub_input_seek(
                    (*sfont).handle,
                    offset.wrapping_add(script_tab.DefaultLangSys as u32) as ssize_t,
                    0i32,
                );
                clt_read_langsys_table(&mut langsys_tab, sfont);
                if otl_match_optrule(feature, b"____\x00" as *const u8 as *const i8) != 0
                    && langsys_tab.ReqFeatureIndex as i32 != 0xffffi32
                {
                    feat_bits[(langsys_tab.ReqFeatureIndex as i32 / 8i32) as usize] =
                        (feat_bits[(langsys_tab.ReqFeatureIndex as i32 / 8i32) as usize] as i32
                            | 1i32 << 7i32 - langsys_tab.ReqFeatureIndex as i32 % 8i32)
                            as u8
                }
                feat_idx = 0i32;
                while feat_idx < langsys_tab.FeatureIndex.count as i32 {
                    feat_bits[(*langsys_tab.FeatureIndex.value.offset(feat_idx as isize) as i32
                        / 8i32) as usize] =
                        (feat_bits[(*langsys_tab.FeatureIndex.value.offset(feat_idx as isize)
                            as i32
                            / 8i32) as usize] as i32
                            | 1i32
                                << 7i32
                                    - *langsys_tab.FeatureIndex.value.offset(feat_idx as isize)
                                        as i32
                                        % 8i32) as u8;
                    feat_idx += 1
                }
                clt_release_langsys_table(&mut langsys_tab);
            }
            langsys_idx = 0i32;
            while langsys_idx < script_tab.LangSysRecord.count as i32 {
                let mut langsys_rec: *mut clt_record = 0 as *mut clt_record;
                langsys_rec = &mut *script_tab.LangSysRecord.record.offset(langsys_idx as isize)
                    as *mut clt_record;
                if otl_match_optrule(language, (*langsys_rec).tag.as_mut_ptr()) != 0 {
                    let mut langsys_tab_0: clt_langsys_table = clt_langsys_table {
                        LookupOrder: 0,
                        ReqFeatureIndex: 0,
                        FeatureIndex: clt_number_list {
                            count: 0,
                            value: 0 as *mut u16,
                        },
                    };
                    if verbose > 0i32 {
                        dpx_message(
                            b"otl_gsub>> OTL script-language enabled: %c%c%c%c.%c%c%c%c\n\x00"
                                as *const u8 as *const i8,
                            (*script_list.record.offset(script_idx as isize)).tag[0] as i32,
                            (*script_list.record.offset(script_idx as isize)).tag[1] as i32,
                            (*script_list.record.offset(script_idx as isize)).tag[2] as i32,
                            (*script_list.record.offset(script_idx as isize)).tag[3] as i32,
                            (*langsys_rec).tag[0] as i32,
                            (*langsys_rec).tag[1] as i32,
                            (*langsys_rec).tag[2] as i32,
                            (*langsys_rec).tag[3] as i32,
                        );
                    }
                    ttstub_input_seek(
                        (*sfont).handle,
                        offset.wrapping_add((*langsys_rec).offset as u32) as ssize_t,
                        0i32,
                    );
                    clt_read_langsys_table(&mut langsys_tab_0, sfont);
                    if otl_match_optrule(feature, b"____\x00" as *const u8 as *const i8) != 0
                        || langsys_tab_0.ReqFeatureIndex as i32 != 0xffffi32
                    {
                        feat_bits[(langsys_tab_0.ReqFeatureIndex as i32 / 8i32) as usize] =
                            (feat_bits[(langsys_tab_0.ReqFeatureIndex as i32 / 8i32) as usize]
                                as i32
                                | 1i32 << 7i32 - langsys_tab_0.ReqFeatureIndex as i32 % 8i32)
                                as u8
                    }
                    feat_idx = 0i32;
                    while feat_idx < langsys_tab_0.FeatureIndex.count as i32 {
                        feat_bits[(*langsys_tab_0.FeatureIndex.value.offset(feat_idx as isize)
                            as i32
                            / 8i32) as usize] =
                            (feat_bits[(*langsys_tab_0.FeatureIndex.value.offset(feat_idx as isize)
                                as i32
                                / 8i32) as usize] as i32
                                | 1i32
                                    << 7i32
                                        - *langsys_tab_0
                                            .FeatureIndex
                                            .value
                                            .offset(feat_idx as isize)
                                            as i32
                                            % 8i32) as u8;
                        feat_idx += 1
                    }
                    clt_release_langsys_table(&mut langsys_tab_0);
                }
                langsys_idx += 1
            }
            clt_release_script_table(&mut script_tab);
        }
        script_idx += 1
    }
    offset = gsub_offset.wrapping_add(head.FeatureList as u32);
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    clt_read_record_list(&mut feature_list, sfont);
    offset = gsub_offset.wrapping_add(head.LookupList as u32);
    ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
    clt_read_number_list(&mut lookup_list, sfont);
    if verbose > 0i32 {
        dpx_message(b"otl_gsub>> Reading OTL feature(s):\x00" as *const u8 as *const i8);
    }
    feat_idx = 0i32;
    while feat_idx < feature_list.count as i32 {
        if feat_bits[(feat_idx / 8i32) as usize] as i32 & 1i32 << 7i32 - feat_idx % 8i32 != 0
            && otl_match_optrule(
                feature,
                (*feature_list.record.offset(feat_idx as isize))
                    .tag
                    .as_mut_ptr(),
            ) != 0
        {
            let mut feature_table: clt_feature_table = clt_feature_table {
                FeatureParams: 0,
                LookupListIndex: clt_number_list {
                    count: 0,
                    value: 0 as *mut u16,
                },
            };
            let mut i: i32 = 0;
            if verbose > 0i32 {
                dpx_message(
                    b" %c%c%c%c\x00" as *const u8 as *const i8,
                    (*feature_list.record.offset(feat_idx as isize)).tag[0] as i32,
                    (*feature_list.record.offset(feat_idx as isize)).tag[1] as i32,
                    (*feature_list.record.offset(feat_idx as isize)).tag[2] as i32,
                    (*feature_list.record.offset(feat_idx as isize)).tag[3] as i32,
                );
            }
            offset = gsub_offset
                .wrapping_add(head.FeatureList as u32)
                .wrapping_add((*feature_list.record.offset(feat_idx as isize)).offset as u32);
            ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
            clt_read_feature_table(&mut feature_table, sfont);
            if feature_table.FeatureParams as i32 != 0i32 {
                _tt_abort(b"unrecognized FeatureParams\x00" as *const u8 as *const i8);
            }
            i = 0i32;
            while i < feature_table.LookupListIndex.count as i32 {
                let mut lookup_table: clt_lookup_table = clt_lookup_table {
                    LookupType: 0,
                    LookupFlag: 0,
                    SubTableList: clt_number_list {
                        count: 0,
                        value: 0 as *mut u16,
                    },
                };
                let mut ll_idx: i32 = 0;
                let mut st_idx: i32 = 0;
                let mut r: i32 = 0;
                let mut n_st: i32 = 0;
                ll_idx = *feature_table.LookupListIndex.value.offset(i as isize) as i32;
                if ll_idx >= lookup_list.count as i32 {
                    _tt_abort(b"invalid Lookup index.\x00" as *const u8 as *const i8);
                }
                offset = gsub_offset
                    .wrapping_add(head.LookupList as u32)
                    .wrapping_add(*lookup_list.value.offset(ll_idx as isize) as u32);
                ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
                clt_read_lookup_table(&mut lookup_table, sfont);
                if lookup_table.LookupType as i32 != 1i32
                    && lookup_table.LookupType as i32 != 3i32
                    && lookup_table.LookupType as i32 != 4i32
                    && lookup_table.LookupType as i32 != 7i32
                {
                    if verbose > 0i32 {
                        dpx_warning(
                            b"Skipping unsupported GSUB subtable: LookupType=%d\x00" as *const u8
                                as *const i8,
                            lookup_table.LookupType as i32,
                        );
                    }
                } else {
                    subtab = renew(
                        subtab as *mut libc::c_void,
                        ((num_subtabs as i32 + lookup_table.SubTableList.count as i32) as u32
                            as u64)
                            .wrapping_mul(::std::mem::size_of::<otl_gsub_subtab>() as u64)
                            as u32,
                    ) as *mut otl_gsub_subtab;
                    n_st = 0i32;
                    st_idx = 0i32;
                    while st_idx < lookup_table.SubTableList.count as i32 {
                        offset = gsub_offset
                            .wrapping_add(head.LookupList as u32)
                            .wrapping_add(*lookup_list.value.offset(ll_idx as isize) as u32)
                            .wrapping_add(
                                *lookup_table.SubTableList.value.offset(st_idx as isize) as u32
                            );
                        ttstub_input_seek((*sfont).handle, offset as ssize_t, 0i32);
                        match lookup_table.LookupType as i32 {
                            1 => {
                                r = otl_gsub_read_single(
                                    &mut *subtab.offset((num_subtabs as i32 + n_st) as isize),
                                    sfont,
                                );
                                if r <= 0i32 {
                                    dpx_warning(
                                        b"Reading GSUB subtable (single) failed...\x00" as *const u8
                                            as *const i8,
                                    );
                                } else {
                                    if verbose > 0i32 {
                                        dpx_message(b"(single)\x00" as *const u8 as *const i8);
                                    }
                                    n_st += 1
                                }
                            }
                            3 => {
                                r = otl_gsub_read_alternate(
                                    &mut *subtab.offset((num_subtabs as i32 + n_st) as isize),
                                    sfont,
                                );
                                if r <= 0i32 {
                                    dpx_warning(
                                        b"Reading GSUB subtable (alternate) failed...\x00"
                                            as *const u8
                                            as *const i8,
                                    );
                                } else {
                                    if verbose > 0i32 {
                                        dpx_message(b"(alternate)\x00" as *const u8 as *const i8);
                                    }
                                    n_st += 1
                                }
                            }
                            4 => {
                                r = otl_gsub_read_ligature(
                                    &mut *subtab.offset((num_subtabs as i32 + n_st) as isize),
                                    sfont,
                                );
                                if r <= 0i32 {
                                    dpx_warning(
                                        b"Reading GSUB subtable (ligature) failed...\x00"
                                            as *const u8
                                            as *const i8,
                                    );
                                } else {
                                    if verbose > 0i32 {
                                        dpx_message(b"(ligature)\x00" as *const u8 as *const i8);
                                    }
                                    n_st += 1
                                }
                            }
                            7 => {
                                let mut SubstFormat: u16 = 0;
                                let mut ExtensionLookupType: u16 = 0;
                                let mut ExtensionOffset: u32 = 0;
                                SubstFormat = tt_get_unsigned_pair((*sfont).handle);
                                if !(SubstFormat as i32 != 1i32) {
                                    ExtensionLookupType = tt_get_unsigned_pair((*sfont).handle);
                                    ExtensionOffset = tt_get_unsigned_quad((*sfont).handle);
                                    ttstub_input_seek(
                                        (*sfont).handle,
                                        offset.wrapping_add(ExtensionOffset) as ssize_t,
                                        0i32,
                                    );
                                    match ExtensionLookupType as i32 {
                                        1 => {
                                            r = otl_gsub_read_single(
                                                &mut *subtab
                                                    .offset((num_subtabs as i32 + n_st) as isize),
                                                sfont,
                                            );
                                            if r <= 0i32 {
                                                dpx_warning(b"Reading GSUB subtable (ext:single) failed...\x00"
                                                                as *const u8
                                                                as
                                                                *const i8);
                                            } else {
                                                if verbose > 0i32 {
                                                    dpx_message(
                                                        b"(ext:single)\x00" as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                n_st += 1
                                            }
                                        }
                                        3 => {
                                            r = otl_gsub_read_alternate(
                                                &mut *subtab
                                                    .offset((num_subtabs as i32 + n_st) as isize),
                                                sfont,
                                            );
                                            if r <= 0i32 {
                                                dpx_warning(b"Reading GSUB subtable (alternate) failed...\x00"
                                                                as *const u8
                                                                as
                                                                *const i8);
                                            } else {
                                                if verbose > 0i32 {
                                                    dpx_message(
                                                        b"(alternate)\x00" as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                n_st += 1
                                            }
                                        }
                                        4 => {
                                            r = otl_gsub_read_ligature(
                                                &mut *subtab
                                                    .offset((num_subtabs as i32 + n_st) as isize),
                                                sfont,
                                            );
                                            if r <= 0i32 {
                                                dpx_warning(b"Reading GSUB subtable (ext:ligature) failed...\x00"
                                                                as *const u8
                                                                as
                                                                *const i8);
                                            } else {
                                                if verbose > 0i32 {
                                                    dpx_message(
                                                        b"(ext:ligature)\x00" as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                n_st += 1
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                        st_idx += 1
                    }
                    num_subtabs = (num_subtabs as i32 + n_st) as u16;
                    clt_release_lookup_table(&mut lookup_table);
                }
                i += 1
            }
            clt_release_feature_table(&mut feature_table);
        }
        feat_idx += 1
    }
    if verbose > 0i32 {
        dpx_message(b"\n\x00" as *const u8 as *const i8);
        dpx_message(
            b"otl_gsub>> %d subtable(s) read.\n\x00" as *const u8 as *const i8,
            num_subtabs as i32,
        );
    }
    clt_release_number_list(&mut lookup_list);
    clt_release_record_list(&mut feature_list);
    clt_release_record_list(&mut script_list);
    otl_release_opt(script);
    otl_release_opt(language);
    otl_release_opt(feature);
    if !subtab.is_null() {
        (*gsub).num_subtables = num_subtabs as i32;
        (*gsub).subtables = subtab
    } else {
        return -1i32;
    }
    return 0i32;
}
unsafe extern "C" fn otl_gsub_apply_single(
    mut subtab: *mut otl_gsub_subtab,
    mut gid: *mut u16,
) -> i32 {
    let mut idx: i32 = 0;
    if !subtab.is_null() && !gid.is_null() {
    } else {
        __assert_fail(
            b"subtab && gid\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            1145_u32,
            (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                b"int otl_gsub_apply_single(struct otl_gsub_subtab *, USHORT *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*subtab).SubstFormat as i32 == 1i32 {
        let mut data: *mut otl_gsub_single1 = 0 as *mut otl_gsub_single1;
        data = (*subtab).table.single1;
        idx = clt_lookup_coverage(&mut (*data).coverage, *gid);
        if idx >= 0i32 {
            *gid = (*gid as i32 + (*data).DeltaGlyphID as i32) as u16;
            return 0i32;
        }
    } else if (*subtab).SubstFormat as i32 == 2i32 {
        let mut data_0: *mut otl_gsub_single2 = 0 as *mut otl_gsub_single2;
        data_0 = (*subtab).table.single2;
        idx = clt_lookup_coverage(&mut (*data_0).coverage, *gid);
        if idx >= 0i32 && idx < (*data_0).GlyphCount as i32 {
            *gid = *(*data_0).Substitute.offset(idx as isize);
            return 0i32;
        }
    }
    return -1i32;
}
unsafe extern "C" fn otl_gsub_apply_alternate(
    mut subtab: *mut otl_gsub_subtab,
    mut alt_idx: u16,
    mut gid: *mut u16,
) -> i32 {
    let mut idx: i32 = 0;
    if !subtab.is_null() && !gid.is_null() {
    } else {
        __assert_fail(
            b"subtab && gid\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            1177_u32,
            (*::std::mem::transmute::<&[u8; 73], &[i8; 73]>(
                b"int otl_gsub_apply_alternate(struct otl_gsub_subtab *, USHORT, USHORT *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*subtab).SubstFormat as i32 == 1i32 {
        let mut data: *mut otl_gsub_alternate1 = 0 as *mut otl_gsub_alternate1;
        data = (*subtab).table.alternate1;
        idx = clt_lookup_coverage(&mut (*data).coverage, *gid);
        if idx < 0i32 || idx >= (*data).AlternateSetCount as i32 {
            return -1i32;
        } else {
            let mut altset: *mut otl_gsub_altset = 0 as *mut otl_gsub_altset;
            altset = &mut *(*data).AlternateSet.offset(idx as isize) as *mut otl_gsub_altset;
            if alt_idx as i32 >= (*altset).GlyphCount as i32 {
                return -1i32;
            } else {
                *gid = *(*altset).Alternate.offset(alt_idx as isize);
                return 0i32;
            }
        }
    }
    return -1i32;
}
unsafe extern "C" fn glyph_seq_cmp(
    mut glyph_seq0: *mut GlyphID,
    mut n_glyphs0: u16,
    mut glyph_seq1: *mut GlyphID,
    mut n_glyphs1: u16,
) -> i32 {
    let mut i: u16 = 0;
    if n_glyphs0 as i32 != n_glyphs1 as i32 {
        return n_glyphs0 as i32 - n_glyphs1 as i32;
    }
    i = 0_u16;
    while (i as i32) < n_glyphs0 as i32 {
        if *glyph_seq0.offset(i as isize) as i32 != *glyph_seq1.offset(i as isize) as i32 {
            return *glyph_seq0.offset(i as isize) as i32 - *glyph_seq1.offset(i as isize) as i32;
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
unsafe extern "C" fn otl_gsub_apply_ligature(
    mut subtab: *mut otl_gsub_subtab,
    mut gid_in: *mut u16,
    mut num_gids: u16,
    mut gid_out: *mut u16,
) -> i32 {
    let mut idx: i32 = 0;
    if !subtab.is_null() && !gid_out.is_null() {
    } else {
        __assert_fail(b"subtab && gid_out\x00" as *const u8 as
                          *const i8,
                      b"dpx-tt_gsub.c\x00" as *const u8 as
                          *const i8, 1226_u32,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[i8; 82]>(b"int otl_gsub_apply_ligature(struct otl_gsub_subtab *, USHORT *, USHORT, USHORT *)\x00")).as_ptr());
    }
    if gid_in.is_null() || (num_gids as i32) < 1i32 {
        return -1i32;
    }
    if (*subtab).SubstFormat as i32 == 1i32 {
        let mut data: *mut otl_gsub_ligature1 = 0 as *mut otl_gsub_ligature1;
        data = (*subtab).table.ligature1;
        idx = clt_lookup_coverage(&mut (*data).coverage, *gid_in.offset(0));
        if idx >= 0i32 && idx < (*data).LigSetCount as i32 {
            let mut ligset: *mut otl_gsub_ligset = 0 as *mut otl_gsub_ligset;
            let mut j: u16 = 0;
            ligset = &mut *(*data).LigatureSet.offset(idx as isize) as *mut otl_gsub_ligset;
            j = 0_u16;
            while (j as i32) < (*ligset).LigatureCount as i32 {
                if glyph_seq_cmp(
                    &mut *gid_in.offset(1),
                    (num_gids as i32 - 1i32) as u16,
                    (*(*ligset).Ligature.offset(j as isize)).Component,
                    ((*(*ligset).Ligature.offset(j as isize)).CompCount as i32 - 1i32) as u16,
                ) == 0
                {
                    *gid_out = (*(*ligset).Ligature.offset(j as isize)).LigGlyph;
                    return 0i32;
                }
                j = j.wrapping_add(1)
            }
        }
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_new() -> *mut otl_gsub {
    let mut gsub_list: *mut otl_gsub = 0 as *mut otl_gsub;
    gsub_list =
        new((1_u64).wrapping_mul(::std::mem::size_of::<otl_gsub>() as u64) as u32) as *mut otl_gsub;
    (*gsub_list).num_gsubs = 0i32;
    (*gsub_list).select = -1i32;
    (*gsub_list).first = 0 as *mut gsub_entry;
    return gsub_list as *mut otl_gsub;
}
unsafe extern "C" fn clear_chain(mut gsub_list: *mut otl_gsub) {
    let mut entry: *mut gsub_entry = 0 as *mut gsub_entry;
    let mut next: *mut gsub_entry = 0 as *mut gsub_entry;
    entry = (*gsub_list).first;
    while !entry.is_null() {
        next = (*entry).next;
        free(entry as *mut libc::c_void);
        entry = next
    }
    (*gsub_list).first = 0 as *mut gsub_entry;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_add_feat(
    mut gsub_list: *mut otl_gsub,
    mut script: *const i8,
    mut language: *const i8,
    mut feature: *const i8,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut retval: i32 = -1i32;
    let mut i: i32 = 0;
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    if (*gsub_list).num_gsubs > 32i32 {
        _tt_abort(b"Too many GSUB features...\x00" as *const u8 as *const i8);
    }
    i = 0i32;
    while i < (*gsub_list).num_gsubs {
        gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
        if streq_ptr(script, (*gsub).script) as i32 != 0
            && streq_ptr(language, (*gsub).language) as i32 != 0
            && streq_ptr(feature, (*gsub).feature) as i32 != 0
        {
            (*gsub_list).select = i;
            return 0i32;
        }
        i += 1
    }
    gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
    (*gsub).script = new((strlen(script).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*gsub).script, script);
    (*gsub).language = new((strlen(language).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*gsub).language, language);
    (*gsub).feature = new((strlen(feature).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*gsub).feature, feature);
    if verbose > 0i32 {
        dpx_message(b"\n\x00" as *const u8 as *const i8);
        dpx_message(
            b"otl_gsub>> Reading \"%s.%s.%s\"...\n\x00" as *const u8 as *const i8,
            script,
            language,
            feature,
        );
    }
    retval = otl_gsub_read_feat(gsub, sfont);
    if retval >= 0i32 {
        (*gsub_list).select = i;
        (*gsub_list).num_gsubs += 1
    } else {
        if verbose > 0i32 {
            dpx_message(b"otl_gsub>> Failed\n\x00" as *const u8 as *const i8);
        }
        free((*gsub).script as *mut libc::c_void);
        free((*gsub).language as *mut libc::c_void);
        free((*gsub).feature as *mut libc::c_void);
    }
    return retval;
}
unsafe extern "C" fn scan_otl_tag(
    mut otl_tags: *const i8,
    mut endptr: *const i8,
    mut script: *mut i8,
    mut language: *mut i8,
    mut feature: *mut i8,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut period: *const i8 = 0 as *const i8;
    if !script.is_null() && !language.is_null() && !feature.is_null() {
    } else {
        __assert_fail(
            b"script && language && feature\x00" as *const u8 as *const i8,
            b"dpx-tt_gsub.c\x00" as *const u8 as *const i8,
            1357_u32,
            (*::std::mem::transmute::<&[u8; 69], &[i8; 69]>(
                b"int scan_otl_tag(const char *, const char *, char *, char *, char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if otl_tags.is_null() || otl_tags >= endptr {
        return -1i32;
    }
    memset(script as *mut libc::c_void, ' ' as i32, 4i32 as u64);
    *script.offset(4) = 0i32 as i8;
    memset(language as *mut libc::c_void, ' ' as i32, 4i32 as u64);
    *language.offset(4) = 0i32 as i8;
    memset(feature as *mut libc::c_void, ' ' as i32, 4i32 as u64);
    *feature.offset(4) = 0i32 as i8;
    /* First parse otl_tags variable */
    p = otl_tags;
    period = strchr(p, '.' as i32);
    if !period.is_null() && period < endptr {
        /* Format scrp.lang.feat */
        if period < p.offset(5) {
            strncpy(script, p, period.wrapping_offset_from(p) as i64 as u64);
        } else {
            dpx_warning(
                b"Invalid OTL script tag found: %s\x00" as *const u8 as *const i8,
                p,
            );
            return -1i32;
        }
        p = period.offset(1);
        period = strchr(p, '.' as i32);
        if !period.is_null() && period < endptr {
            /* Now lang part */
            if period < p.offset(5) {
                strncpy(language, p, period.wrapping_offset_from(p) as i64 as u64);
            } else {
                dpx_warning(
                    b"Invalid OTL lanuage tag found: %s\x00" as *const u8 as *const i8,
                    p,
                );
                return -1i32;
            }
            p = period.offset(1)
        }
    } else {
        strcpy(script, b"*\x00" as *const u8 as *const i8);
        strcpy(language, b"*\x00" as *const u8 as *const i8);
    }
    /* Finally feature */
    if p.offset(4) <= endptr {
        strncpy(feature, p, endptr.wrapping_offset_from(p) as i64 as u64);
        p = endptr
    } else {
        dpx_warning(b"No valid OTL feature tag specified.\x00" as *const u8 as *const i8);
        return -1i32;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_release(mut gsub_list: *mut otl_gsub) {
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    let mut subtab: *mut otl_gsub_subtab = 0 as *mut otl_gsub_subtab;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if gsub_list.is_null() {
        return;
    }
    i = 0i32;
    while i < (*gsub_list).num_gsubs {
        gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
        free((*gsub).script as *mut libc::c_void);
        free((*gsub).language as *mut libc::c_void);
        free((*gsub).feature as *mut libc::c_void);
        j = 0i32;
        while j < (*gsub).num_subtables {
            subtab = &mut *(*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
            match (*subtab).LookupType as i32 {
                1 => {
                    otl_gsub_release_single(subtab);
                }
                3 => {
                    otl_gsub_release_alternate(subtab);
                }
                4 => {
                    otl_gsub_release_ligature(subtab);
                }
                _ => {
                    _tt_abort(b"???\x00" as *const u8 as *const i8);
                }
            }
            j += 1
        }
        free((*gsub).subtables as *mut libc::c_void);
        i += 1
    }
    clear_chain(gsub_list);
    free(gsub_list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_apply(mut gsub_list: *mut otl_gsub, mut gid: *mut u16) -> i32 {
    let mut retval: i32 = -1i32;
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    let mut subtab: *mut otl_gsub_subtab = 0 as *mut otl_gsub_subtab;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if gsub_list.is_null() || gid.is_null() {
        return retval;
    }
    i = (*gsub_list).select;
    if i < 0i32 || i >= (*gsub_list).num_gsubs {
        _tt_abort(b"GSUB not selected...\x00" as *const u8 as *const i8);
    }
    gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
    j = 0i32;
    while retval < 0i32 && j < (*gsub).num_subtables {
        subtab = &mut *(*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
        match (*subtab).LookupType as i32 {
            1 => retval = otl_gsub_apply_single(subtab, gid),
            _ => {}
        }
        j += 1
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_apply_alt(
    mut gsub_list: *mut otl_gsub,
    mut alt_idx: u16,
    mut gid: *mut u16,
) -> i32 {
    let mut retval: i32 = -1i32;
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    let mut subtab: *mut otl_gsub_subtab = 0 as *mut otl_gsub_subtab;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if gsub_list.is_null() || gid.is_null() {
        return retval;
    }
    i = (*gsub_list).select;
    if i < 0i32 || i >= (*gsub_list).num_gsubs {
        _tt_abort(b"GSUB not selected...\x00" as *const u8 as *const i8);
    }
    gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
    j = 0i32;
    while retval < 0i32 && j < (*gsub).num_subtables {
        subtab = &mut *(*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
        match (*subtab).LookupType as i32 {
            3 => retval = otl_gsub_apply_alternate(subtab, alt_idx, gid),
            _ => {}
        }
        j += 1
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_apply_lig(
    mut gsub_list: *mut otl_gsub,
    mut gid_in: *mut u16,
    mut num_gids: u16,
    mut gid_out: *mut u16,
) -> i32 {
    let mut retval: i32 = -1i32;
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    let mut subtab: *mut otl_gsub_subtab = 0 as *mut otl_gsub_subtab;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if gsub_list.is_null() || gid_out.is_null() {
        return retval;
    }
    i = (*gsub_list).select;
    if i < 0i32 || i >= (*gsub_list).num_gsubs {
        _tt_abort(b"GSUB not selected...\x00" as *const u8 as *const i8);
    }
    gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
    j = 0i32;
    while retval < 0i32 && j < (*gsub).num_subtables {
        subtab = &mut *(*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
        match (*subtab).LookupType as i32 {
            4 => retval = otl_gsub_apply_ligature(subtab, gid_in, num_gids, gid_out),
            _ => {}
        }
        j += 1
    }
    return retval;
}
unsafe extern "C" fn gsub_find(
    mut gsub_list: *mut otl_gsub,
    mut script: *const i8,
    mut language: *const i8,
    mut feature: *const i8,
) -> i32 {
    let mut i: i32 = 0;
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    i = 0i32;
    while i < (*gsub_list).num_gsubs {
        gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(i as isize) as *mut otl_gsub_tab;
        if streq_ptr((*gsub).script, script) as i32 != 0
            && streq_ptr((*gsub).language, language) as i32 != 0
            && streq_ptr((*gsub).feature, feature) as i32 != 0
        {
            return i;
        }
        i += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_select(
    mut gsub_list: *mut otl_gsub,
    mut script: *const i8,
    mut language: *const i8,
    mut feature: *const i8,
) -> i32 {
    (*gsub_list).select = gsub_find(gsub_list, script, language, feature);
    return (*gsub_list).select;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_set_chain(
    mut gsub_list: *mut otl_gsub,
    mut otl_tags: *const i8,
) -> i32 {
    let mut prev: *mut gsub_entry = 0 as *mut gsub_entry;
    let mut p: *const i8 = 0 as *const i8;
    let mut nextptr: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut script: [i8; 5] = [0; 5];
    let mut language: [i8; 5] = [0; 5];
    let mut feature: [i8; 5] = [0; 5];
    let mut idx: i32 = 0;
    clear_chain(gsub_list);
    endptr = otl_tags.offset(strlen(otl_tags) as isize);
    p = otl_tags;
    while p < endptr {
        nextptr = strchr(p, ':' as i32);
        if nextptr.is_null() {
            nextptr = endptr
        }
        if scan_otl_tag(
            p,
            nextptr,
            script.as_mut_ptr(),
            language.as_mut_ptr(),
            feature.as_mut_ptr(),
        ) >= 0i32
        {
            idx = gsub_find(
                gsub_list,
                script.as_mut_ptr(),
                language.as_mut_ptr(),
                feature.as_mut_ptr(),
            );
            if idx >= 0i32 && idx <= (*gsub_list).num_gsubs {
                let mut entry: *mut gsub_entry = 0 as *mut gsub_entry;
                entry = new((1_u64).wrapping_mul(::std::mem::size_of::<gsub_entry>() as u64) as u32)
                    as *mut gsub_entry;
                if (*gsub_list).first.is_null() {
                    (*gsub_list).first = entry
                }
                if !prev.is_null() {
                    (*prev).next = entry
                }
                (*entry).index = idx;
                prev = entry
            }
        }
        nextptr = nextptr.offset(1);
        p = nextptr
    }
    if !prev.is_null() {
        (*prev).next = 0 as *mut gsub_entry
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_add_feat_list(
    mut gsub_list: *mut otl_gsub,
    mut otl_tags: *const i8,
    mut sfont: *mut sfnt,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut nextptr: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut script: [i8; 5] = [0; 5];
    let mut language: [i8; 5] = [0; 5];
    let mut feature: [i8; 5] = [0; 5];
    let mut idx: i32 = 0;
    if gsub_list.is_null() || otl_tags.is_null() || sfont.is_null() {
        return -1i32;
    }
    clear_chain(gsub_list);
    endptr = otl_tags.offset(strlen(otl_tags) as isize);
    p = otl_tags;
    while p < endptr {
        nextptr = strchr(p, ':' as i32);
        if nextptr.is_null() {
            nextptr = endptr
        }
        if scan_otl_tag(
            p,
            nextptr,
            script.as_mut_ptr(),
            language.as_mut_ptr(),
            feature.as_mut_ptr(),
        ) >= 0i32
        {
            idx = gsub_find(
                gsub_list,
                script.as_mut_ptr(),
                language.as_mut_ptr(),
                feature.as_mut_ptr(),
            );
            if idx < 0i32 {
                otl_gsub_add_feat(
                    gsub_list,
                    script.as_mut_ptr(),
                    language.as_mut_ptr(),
                    feature.as_mut_ptr(),
                    sfont,
                );
            }
        }
        nextptr = nextptr.offset(1);
        p = nextptr
    }
    return 0i32;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
/* LookupType for GSUB */
/* Handle a list of OTL features */
#[no_mangle]
pub unsafe extern "C" fn otl_gsub_apply_chain(
    mut gsub_list: *mut otl_gsub,
    mut gid: *mut u16,
) -> i32 {
    let mut retval: i32 = -1i32;
    let mut gsub: *mut otl_gsub_tab = 0 as *mut otl_gsub_tab;
    let mut subtab: *mut otl_gsub_subtab = 0 as *mut otl_gsub_subtab;
    let mut entry: *mut gsub_entry = 0 as *mut gsub_entry;
    let mut i: i32 = 0;
    let mut idx: i32 = 0;
    if gsub_list.is_null() || gid.is_null() {
        return retval;
    }
    entry = (*gsub_list).first;
    while !entry.is_null() {
        idx = (*entry).index;
        if !(idx < 0i32 || idx >= (*gsub_list).num_gsubs) {
            gsub = &mut *(*gsub_list).gsubs.as_mut_ptr().offset(idx as isize) as *mut otl_gsub_tab;
            i = 0i32;
            retval = -1i32;
            while retval < 0i32 && i < (*gsub).num_subtables {
                subtab = &mut *(*gsub).subtables.offset(i as isize) as *mut otl_gsub_subtab;
                match (*subtab).LookupType as i32 {
                    1 => retval = otl_gsub_apply_single(subtab, gid),
                    _ => {}
                }
                i += 1
            }
        }
        entry = (*entry).next
    }
    return retval;
}
