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
#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use super::dpx_sfnt::{
    sfnt_close, sfnt_find_table_pos, sfnt_locate_table, sfnt_open, sfnt_read_table_directory,
};
use crate::{info, warn};

use super::dpx_agl::{
    agl_chop_suffix, agl_lookup_list, agl_name_convert_unicode, agl_name_is_unicode,
    agl_sput_UTF16BE,
};
use super::dpx_cff::{
    cff_add_string, cff_charsets_lookup_inverse, cff_close, cff_get_index_header, cff_get_name,
    cff_get_sid, cff_get_string, cff_glyph_lookup, cff_index_size, cff_new_index, cff_open,
    cff_pack_charsets, cff_pack_fdselect, cff_pack_index, cff_put_header, cff_read_charsets,
    cff_read_fdselect, cff_read_subrs, cff_release_index, cff_set_name, cff_update_string,
};
use super::dpx_cff::{
    cff_charsets_lookup, cff_fdselect_lookup, cff_read_fdarray, cff_read_private,
    cff_release_charsets, cff_release_fdselect,
};
use super::dpx_cff_dict::{
    cff_dict_add, cff_dict_get, cff_dict_known, cff_dict_pack, cff_dict_remove, cff_dict_set,
    cff_dict_update, cff_new_dict,
};
use super::dpx_cid::{
    CIDFont_get_embedding, CIDFont_get_opt_index, CIDFont_get_parent_id, CIDFont_is_BaseFont,
};
use super::dpx_cid::{CSI_IDENTITY, CSI_UNICODE};
use super::dpx_cmap::{
    CMap_add_bfchar, CMap_add_cidchar, CMap_add_codespacerange, CMap_cache_add, CMap_cache_find,
    CMap_new, CMap_release, CMap_set_CIDSysInfo, CMap_set_name, CMap_set_type, CMap_set_wmode,
};
use super::dpx_cmap_write::CMap_create_stream;
use super::dpx_cs_type2::cs_copy_charstring;
use super::dpx_dpxfile::{dpx_open_opentype_file, dpx_open_truetype_file, dpx_open_type1_file};
use super::dpx_mfileio::work_buffer;
use super::dpx_pdffont::pdf_font_make_uniqueTag;
use super::dpx_t1_char::{t1char_convert_charstring, t1char_get_metrics};
use super::dpx_t1_load::t1_load_font;
use super::dpx_tt_aux::tt_get_fontdesc;
use super::dpx_tt_aux::ttc_read_offset;
use super::dpx_tt_table::{
    tt_read_VORG_table, tt_read_head_table, tt_read_hhea_table, tt_read_longMetrics,
    tt_read_maxp_table, tt_read_os2__table, tt_read_vhea_table,
};
use super::dpx_type0::{
    Type0Font, Type0Font_cache_get, Type0Font_get_usedchars, Type0Font_set_ToUnicode,
};
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_add_stream, pdf_array_length, pdf_new_array, pdf_new_dict,
    pdf_new_name, pdf_new_number, pdf_new_stream, pdf_new_string, pdf_obj, pdf_ref_obj,
    pdf_release_obj, pdf_stream_dict,
};
use crate::{ttstub_input_close, ttstub_input_read, ttstub_input_seek};
use libc::free;
extern "C" {
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;

use super::dpx_cid::{cid_opt, CIDFont, CIDSysInfo};

use super::dpx_cs_type2::cs_ginfo;
use super::dpx_sfnt::sfnt;

use super::dpx_cff::cff_charsets;
pub type card8 = u8;
pub type card16 = u16;
pub type s_SID = u16;

use super::dpx_cff::cff_index;
pub type l_offset = u32;
pub type c_offsize = u8;
use super::dpx_cff::cff_fdselect;
use super::dpx_cff::cff_range3;
use super::dpx_tt_table::tt_longMetrics;
use super::dpx_tt_table::tt_vhea_table;
pub type Fixed = u32;

use super::dpx_cff::cff_dict;
use super::dpx_cff::cff_font;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDType0Info {
    pub handle: rust_input_handle_t,
    pub sfont: *mut sfnt,
    pub cffont: *mut cff_font,
}
use super::dpx_tt_table::{tt_head_table, tt_hhea_table, tt_maxp_table, tt_os2__table};
/* 16.16-bit signed fixed-point number */
pub type FWord = i16;
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */

pub type uFWord = u16;
use super::dpx_tt_table::tt_VORG_table;

pub type CID = u16;
pub type CIDType0Error = i32;
pub const CID_OPEN_ERROR_IS_CIDFONT: CIDType0Error = -6;
pub const CID_OPEN_ERROR_NOT_CIDFONT: CIDType0Error = -5;
pub const CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT: CIDType0Error = -4;
pub const CID_OPEN_ERROR_NO_CFF_TABLE: CIDType0Error = -3;
pub const CID_OPEN_ERROR_NOT_SFNT_FONT: CIDType0Error = -2;
pub const CID_OPEN_ERROR_CANNOT_OPEN_FILE: CIDType0Error = -1;
pub const CID_OPEN_ERROR_NO_ERROR: CIDType0Error = 0;
use super::dpx_cmap::CMap;

/* Mapping types, MAP_IS_NAME is not supported. */
/* Lookup flags */
/* DEBUG */
/* Codespacerange */
/* Dimension of this codespacerange */
/* Lower bounds of valid input code */
/* Upper bounds of valid input code */

use super::dpx_agl::agl_name;
use super::dpx_t1_char::t1_ginfo;

/*
 * CID-Keyed Font support:
 *
 *  Only CFF/OpenType CID-Keyed Font with Type 2 charstrings is supported.
 *
 */
/* typedef CID in cmap.h */
/* pseudo unique tag */
/* Font info. from OpenType tables */
/* Metrics */
static mut verbose: i32 = 0i32;
static mut opt_flags: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_set_verbose(mut level: i32) {
    verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_set_flags(mut flags: i32) {
    opt_flags = flags;
}
/*
 * PDF Reference 3rd. ed., p.340, "Glyph Metrics in CID Fonts".
 */
unsafe extern "C" fn add_CIDHMetrics(
    mut fontdict: *mut pdf_obj,
    mut CIDToGIDMap: *mut u8,
    mut last_cid: u16,
    mut maxp: *mut tt_maxp_table,
    mut head: *mut tt_head_table,
    mut hmtx: *mut tt_longMetrics,
) {
    let mut w_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut an_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cid: i32 = 0;
    let mut start: i32 = 0i32;
    let mut prev: i32 = 0i32;
    let mut defaultAdvanceWidth: f64 = 0.;
    let mut empty: i32 = 1i32;
    defaultAdvanceWidth = (1000.0f64 * (*hmtx.offset(0)).advance as i32 as f64
        / (*head).unitsPerEm as i32 as f64
        / 1i32 as f64
        + 0.5f64)
        .floor()
        * 1i32 as f64;
    /*
     * We alway use format:
     *  c [w_1 w_2 ... w_n]
     */
    w_array = pdf_new_array();
    cid = 0i32;
    while cid <= last_cid as i32 {
        let mut gid: u16 = 0;
        let mut advanceWidth: f64 = 0.;
        gid = (if !CIDToGIDMap.is_null() {
            (*CIDToGIDMap.offset((2i32 * cid) as isize) as i32) << 8i32
                | *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) as i32
        } else {
            cid
        }) as u16;
        if !(gid as i32 >= (*maxp).numGlyphs as i32 || cid != 0i32 && gid as i32 == 0i32) {
            advanceWidth = (1000.0f64 * (*hmtx.offset(gid as isize)).advance as i32 as f64
                / (*head).unitsPerEm as i32 as f64
                / 1i32 as f64
                + 0.5f64)
                .floor()
                * 1i32 as f64;
            if advanceWidth == defaultAdvanceWidth {
                if !an_array.is_null() {
                    pdf_add_array(w_array, pdf_new_number(start as f64));
                    pdf_add_array(w_array, an_array);
                    an_array = 0 as *mut pdf_obj;
                    empty = 0i32
                }
            } else {
                if cid != prev + 1i32 && !an_array.is_null() {
                    pdf_add_array(w_array, pdf_new_number(start as f64));
                    pdf_add_array(w_array, an_array);
                    an_array = 0 as *mut pdf_obj;
                    empty = 0i32
                }
                if an_array.is_null() {
                    an_array = pdf_new_array();
                    start = cid
                }
                pdf_add_array(an_array, pdf_new_number(advanceWidth));
                prev = cid
            }
        }
        cid += 1
    }
    if !an_array.is_null() {
        pdf_add_array(w_array, pdf_new_number(start as f64));
        pdf_add_array(w_array, an_array);
        empty = 0i32
    }
    /*
     * We always write DW for older MacOS X's preview app.
     * PDF Reference 2nd. ed, wrongly described default value of DW as 0, and
     * MacOS X's (up to 10.2.8) preview app. implements this wrong description.
     */
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"DW\x00" as *const u8 as *const i8),
        pdf_new_number(defaultAdvanceWidth),
    );
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W\x00" as *const u8 as *const i8),
            pdf_ref_obj(w_array),
        );
    }
    pdf_release_obj(w_array);
}
unsafe extern "C" fn add_CIDVMetrics(
    mut sfont: *mut sfnt,
    mut fontdict: *mut pdf_obj,
    mut CIDToGIDMap: *mut u8,
    mut last_cid: u16,
    mut maxp: *mut tt_maxp_table,
    mut head: *mut tt_head_table,
    mut hmtx: *mut tt_longMetrics,
) {
    let mut w2_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut an_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cid: i32 = 0;
    let mut vorg: *mut tt_VORG_table = 0 as *mut tt_VORG_table;
    let mut vhea: *mut tt_vhea_table = 0 as *mut tt_vhea_table;
    let mut vmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut defaultAdvanceHeight: f64 = 0.;
    let mut defaultVertOriginY: f64 = 0.;
    let mut empty: i32 = 1i32;
    /*
     * No accurate vertical metrics can be obtained by simple way if the
     * font does not have VORG table. Only CJK fonts may have VORG.
     */
    if sfnt_find_table_pos(sfont, b"VORG\x00" as *const u8 as *const i8) <= 0_u32 {
        return;
    }
    vorg = tt_read_VORG_table(sfont);
    defaultVertOriginY = (1000.0f64 * (*vorg).defaultVertOriginY as i32 as f64
        / (*head).unitsPerEm as i32 as f64
        / 1i32 as f64
        + 0.5f64)
        .floor()
        * 1i32 as f64;
    if sfnt_find_table_pos(sfont, b"vhea\x00" as *const u8 as *const i8) > 0_u32 {
        vhea = tt_read_vhea_table(sfont)
    }
    if !vhea.is_null() && sfnt_find_table_pos(sfont, b"vmtx\x00" as *const u8 as *const i8) > 0_u32
    {
        sfnt_locate_table(sfont, b"vmtx\x00" as *const u8 as *const i8);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        )
    }
    if sfnt_find_table_pos(sfont, b"OS/2\x00" as *const u8 as *const i8) <= 0_u32 {
        let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
        /* OpenType font must have OS/2 table. */
        os2 = tt_read_os2__table(sfont);
        defaultVertOriginY = (1000.0f64 * (*os2).sTypoAscender as i32 as f64
            / (*head).unitsPerEm as i32 as f64
            / 1i32 as f64
            + 0.5f64)
            .floor()
            * 1i32 as f64;
        defaultAdvanceHeight = (1000.0f64
            * ((*os2).sTypoAscender as i32 - (*os2).sTypoDescender as i32) as f64
            / (*head).unitsPerEm as i32 as f64
            / 1i32 as f64
            + 0.5f64)
            .floor()
            * 1i32 as f64;
        free(os2 as *mut libc::c_void);
    } else {
        /* Some TrueType fonts used in Macintosh does not have OS/2 table. */
        defaultAdvanceHeight = 1000i32 as f64
    }
    w2_array = pdf_new_array();
    cid = 0i32;
    while cid <= last_cid as i32 {
        let mut i: u16 = 0;
        let mut gid: u16 = 0;
        let mut advanceHeight: f64 = 0.;
        let mut vertOriginX: f64 = 0.;
        let mut vertOriginY: f64 = 0.;
        gid = (if !CIDToGIDMap.is_null() {
            (*CIDToGIDMap.offset((2i32 * cid) as isize) as i32) << 8i32
                | *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) as i32
        } else {
            cid
        }) as u16;
        if !(gid as i32 >= (*maxp).numGlyphs as i32 || cid != 0i32 && gid as i32 == 0i32) {
            advanceHeight = if !vmtx.is_null() {
                (1000.0f64 * (*vmtx.offset(gid as isize)).advance as i32 as f64
                    / (*head).unitsPerEm as i32 as f64
                    / 1i32 as f64
                    + 0.5f64)
                    .floor()
                    * 1i32 as f64
            } else {
                defaultAdvanceHeight
            };
            vertOriginX = (1000.0f64
                * ((*hmtx.offset(gid as isize)).advance as i32 as f64 * 0.5f64)
                / (*head).unitsPerEm as i32 as f64
                / 1i32 as f64
                + 0.5f64)
                .floor()
                * 1i32 as f64;
            vertOriginY = defaultVertOriginY;
            i = 0_u16;
            while (i as i32) < (*vorg).numVertOriginYMetrics as i32
                && gid as i32 > (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex as i32
            {
                if gid as i32 == (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex as i32
                {
                    vertOriginY = (1000.0f64
                        * (*(*vorg).vertOriginYMetrics.offset(i as isize)).vertOriginY as i32
                            as f64
                        / (*head).unitsPerEm as i32 as f64
                        / 1i32 as f64
                        + 0.5f64)
                        .floor()
                        * 1i32 as f64
                }
                i = i.wrapping_add(1)
            }
            /*
             * c_first c_last w1_y v_x v_y
             * This form may hit Acrobat's implementation limit of array element size, 8192.
             * AFPL GhostScript 8.11 stops with rangecheck error with this. Maybe GS's bug?
             */
            if vertOriginY != defaultVertOriginY || advanceHeight != defaultAdvanceHeight {
                pdf_add_array(w2_array, pdf_new_number(cid as f64));
                pdf_add_array(w2_array, pdf_new_number(cid as f64));
                pdf_add_array(w2_array, pdf_new_number(-advanceHeight));
                pdf_add_array(w2_array, pdf_new_number(vertOriginX));
                pdf_add_array(w2_array, pdf_new_number(vertOriginY));
                empty = 0i32
            }
        }
        cid += 1
    }
    if defaultVertOriginY != 880i32 as f64 || defaultAdvanceHeight != 1000i32 as f64 {
        an_array = pdf_new_array();
        pdf_add_array(an_array, pdf_new_number(defaultVertOriginY));
        pdf_add_array(an_array, pdf_new_number(-defaultAdvanceHeight));
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"DW2\x00" as *const u8 as *const i8),
            an_array,
        );
    }
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W2\x00" as *const u8 as *const i8),
            pdf_ref_obj(w2_array),
        );
    }
    pdf_release_obj(w2_array);
    free((*vorg).vertOriginYMetrics as *mut libc::c_void);
    free(vorg as *mut libc::c_void);
    free(vmtx as *mut libc::c_void);
    free(vhea as *mut libc::c_void);
}
unsafe extern "C" fn add_CIDMetrics(
    mut sfont: *mut sfnt,
    mut fontdict: *mut pdf_obj,
    mut CIDToGIDMap: *mut u8,
    mut last_cid: u16,
    mut need_vmetrics: i32,
) {
    let mut hmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut hhea: *mut tt_hhea_table = 0 as *mut tt_hhea_table;
    let mut maxp: *mut tt_maxp_table = 0 as *mut tt_maxp_table;
    /*
     * Read head, hhea, maxp:
     *
     *   unitsPerEm       --> head
     *   numHMetrics      --> hhea
     *   numGlyphs        --> maxp
     */
    head = tt_read_head_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const i8);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    add_CIDHMetrics(fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);
    if need_vmetrics != 0 {
        add_CIDVMetrics(sfont, fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);
    }
    free(hmtx as *mut libc::c_void);
    free(hhea as *mut libc::c_void);
    free(maxp as *mut libc::c_void);
    free(head as *mut libc::c_void);
}
/*
 * Create an instance of embeddable font.
 */
unsafe extern "C" fn write_fontfile(mut font: *mut CIDFont, mut cffont: *mut cff_font) -> i32 {
    let mut topdict: *mut cff_index = 0 as *mut cff_index;
    let mut fdarray: *mut cff_index = 0 as *mut cff_index;
    let mut private: *mut cff_index = 0 as *mut cff_index;
    let mut dest: *mut u8 = 0 as *mut u8;
    let mut destlen: i32 = 0i32;
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut offset: i32 = 0;
    let mut topdict_offset: i32 = 0;
    let mut fdarray_offset: i32 = 0;
    /*  DICT sizes (offset set to long int) */
    topdict = cff_new_index(1i32 as card16); /* some bad font may have */
    fdarray = cff_new_index((*cffont).num_fds as card16); /* some bad font may have */
    private = cff_new_index((*cffont).num_fds as card16);
    cff_dict_remove((*cffont).topdict, b"UniqueID\x00" as *const u8 as *const i8);
    cff_dict_remove((*cffont).topdict, b"XUID\x00" as *const u8 as *const i8);
    cff_dict_remove((*cffont).topdict, b"Private\x00" as *const u8 as *const i8);
    cff_dict_remove((*cffont).topdict, b"Encoding\x00" as *const u8 as *const i8);
    *(*topdict).offset.offset(1) = (cff_dict_pack(
        (*cffont).topdict,
        work_buffer.as_mut_ptr() as *mut card8,
        1024i32,
    ) + 1i32) as l_offset;
    i = 0i32;
    while i < (*cffont).num_fds as i32 {
        size = 0i32;
        if !(*cffont).private.is_null() && !(*(*cffont).private.offset(i as isize)).is_null() {
            size = cff_dict_pack(
                *(*cffont).private.offset(i as isize),
                work_buffer.as_mut_ptr() as *mut card8,
                1024i32,
            );
            if size < 1i32 {
                /* Private had contained only Subr */
                cff_dict_remove(
                    *(*cffont).fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                ); /* header size */
            }
        } /* charset format 0 */
        *(*private).offset.offset((i + 1i32) as isize) =
            (*(*private).offset.offset(i as isize)).wrapping_add(size as u32); /* fdselect format 3 */
        *(*fdarray).offset.offset((i + 1i32) as isize) = (*(*fdarray).offset.offset(i as isize))
            .wrapping_add(cff_dict_pack(
                *(*cffont).fdarray.offset(i as isize),
                work_buffer.as_mut_ptr() as *mut card8,
                1024i32,
            ) as u32); /* Private is not INDEX */
        i += 1
    }
    destlen = 4i32;
    destlen += cff_set_name(cffont, (*font).fontname);
    destlen += cff_index_size(topdict);
    destlen += cff_index_size((*cffont).string);
    destlen += cff_index_size((*cffont).gsubr);
    destlen += (*(*cffont).charsets).num_entries as i32 * 2i32 + 1i32;
    destlen += (*(*cffont).fdselect).num_entries as i32 * 3i32 + 5i32;
    destlen += cff_index_size((*cffont).cstrings);
    destlen += cff_index_size(fdarray);
    destlen = (destlen as u32)
        .wrapping_add((*(*private).offset.offset((*private).count as isize)).wrapping_sub(1_u32))
        as i32 as i32;
    dest = new((destlen as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32)
        as *mut card8;
    offset = 0i32;
    /* Header */
    offset += cff_put_header(cffont, dest.offset(offset as isize), destlen - offset);
    /* Name */
    offset += cff_pack_index(
        (*cffont).name,
        dest.offset(offset as isize),
        destlen - offset,
    );
    /* Top DICT */
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    /* Strings */
    offset += cff_pack_index(
        (*cffont).string,
        dest.offset(offset as isize),
        destlen - offset,
    );
    /* Global Subrs */
    offset += cff_pack_index(
        (*cffont).gsubr,
        dest.offset(offset as isize),
        destlen - offset,
    );
    /* charset */
    cff_dict_set(
        (*cffont).topdict,
        b"charset\x00" as *const u8 as *const i8,
        0i32,
        offset as f64,
    );
    offset += cff_pack_charsets(cffont, dest.offset(offset as isize), destlen - offset);
    /* FDSelect */
    cff_dict_set(
        (*cffont).topdict,
        b"FDSelect\x00" as *const u8 as *const i8,
        0i32,
        offset as f64,
    );
    offset += cff_pack_fdselect(cffont, dest.offset(offset as isize), destlen - offset);
    /* CharStrings */
    cff_dict_set(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        0i32,
        offset as f64,
    ); /* Charstrings cosumes huge memory */
    offset += cff_pack_index(
        (*cffont).cstrings,
        dest.offset(offset as isize),
        cff_index_size((*cffont).cstrings),
    );
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = 0 as *mut cff_index;
    /* FDArray and Private */
    cff_dict_set(
        (*cffont).topdict,
        b"FDArray\x00" as *const u8 as *const i8,
        0i32,
        offset as f64,
    );
    fdarray_offset = offset;
    offset += cff_index_size(fdarray);
    (*fdarray).data = new(((*(*fdarray).offset.offset((*fdarray).count as isize))
        .wrapping_sub(1_u32) as u64)
        .wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32)
        as *mut card8;
    i = 0i32;
    while i < (*cffont).num_fds as i32 {
        size = (*(*private).offset.offset((i + 1i32) as isize))
            .wrapping_sub(*(*private).offset.offset(i as isize)) as i32;
        if !(*(*cffont).private.offset(i as isize)).is_null() && size > 0i32 {
            cff_dict_pack(
                *(*cffont).private.offset(i as isize),
                dest.offset(offset as isize),
                size,
            );
            cff_dict_set(
                *(*cffont).fdarray.offset(i as isize),
                b"Private\x00" as *const u8 as *const i8,
                0i32,
                size as f64,
            );
            cff_dict_set(
                *(*cffont).fdarray.offset(i as isize),
                b"Private\x00" as *const u8 as *const i8,
                1i32,
                offset as f64,
            );
        }
        cff_dict_pack(
            *(*cffont).fdarray.offset(i as isize),
            (*fdarray)
                .data
                .offset(*(*fdarray).offset.offset(i as isize) as isize)
                .offset(-1),
            (*(*fdarray).offset.offset((*fdarray).count as isize)).wrapping_sub(1_u32) as i32,
        );
        offset += size;
        i += 1
    }
    cff_pack_index(
        fdarray,
        dest.offset(fdarray_offset as isize),
        cff_index_size(fdarray),
    );
    cff_release_index(fdarray);
    cff_release_index(private);
    /* Finally Top DICT */
    (*topdict).data = new(((*(*topdict).offset.offset((*topdict).count as isize))
        .wrapping_sub(1_u32) as u64)
        .wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32)
        as *mut card8;
    cff_dict_pack(
        (*cffont).topdict,
        (*topdict).data,
        (*(*topdict).offset.offset((*topdict).count as isize)).wrapping_sub(1_u32) as i32,
    );
    cff_pack_index(
        topdict,
        dest.offset(topdict_offset as isize),
        cff_index_size(topdict),
    );
    cff_release_index(topdict);
    /*
     * FontFile
     */
    let mut fontfile: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    fontfile = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontFile3\x00" as *const u8 as *const i8),
        pdf_ref_obj(fontfile),
    );
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"CIDFontType0C\x00" as *const u8 as *const i8),
    );
    pdf_add_stream(fontfile, dest as *mut i8 as *const libc::c_void, offset);
    pdf_release_obj(fontfile);
    free(dest as *mut libc::c_void);
    destlen
}
unsafe extern "C" fn CIDFont_type0_get_used_chars(mut font: *mut CIDFont) -> *mut i8 {
    let mut parent_id: i32 = 0;
    let mut used_chars: *mut i8 = 0 as *mut i8;
    parent_id = CIDFont_get_parent_id(font, 0i32);
    if parent_id < 0i32 && {
        parent_id = CIDFont_get_parent_id(font, 1i32);
        parent_id < 0i32
    } {
        panic!("No parent Type 0 font !");
    }
    used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
    if used_chars.is_null() {
        panic!("Unexpected error: Font not actually used???");
    }
    used_chars
}
unsafe extern "C" fn CIDType0Error_Show(mut error: CIDType0Error, mut name: *const i8) {
    match error as i32 {
        -1 => {
            _tt_abort(
                b"Could not open OpenType font file: %s\x00" as *const u8 as *const i8,
                name,
            );
        }
        -2 => {
            _tt_abort(
                b"Could not open SFNT font file: %s\x00" as *const u8 as *const i8,
                name,
            );
        }
        -3 => {
            _tt_abort(
                b"Not a CFF/OpenType font: %s\x00" as *const u8 as *const i8,
                name,
            );
        }
        -4 => {
            _tt_abort(
                b"Could not open CFF font: %s\x00" as *const u8 as *const i8,
                name,
            );
        }
        -5 => {
            _tt_abort(b"Not a CIDFont: %s\x00" as *const u8 as *const i8, name);
        }
        -6 => {
            _tt_abort(
                b"Should not be a CIDFont: %s\x00" as *const u8 as *const i8,
                name,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn CIDFontInfo_init(mut info: *mut CIDType0Info) {
    memset(
        info as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<CIDType0Info>() as u64,
    );
}
unsafe extern "C" fn CIDFontInfo_close(mut info: *mut CIDType0Info) {
    if !(*info).cffont.is_null() {
        cff_close((*info).cffont);
    }
    if !(*info).sfont.is_null() {
        sfnt_close((*info).sfont);
    }
    if !(*info).handle.is_null() {
        ttstub_input_close((*info).handle);
    }
    CIDFontInfo_init(info);
}
unsafe extern "C" fn CIDFont_type0_try_open(
    mut name: *const i8,
    mut index: i32,
    mut required_cid: i32,
    mut info: *mut CIDType0Info,
) -> CIDType0Error {
    let mut offset: u32 = 0_u32;
    let mut is_cid: i32 = 0;
    CIDFontInfo_init(info);
    (*info).handle = dpx_open_opentype_file(name);
    if (*info).handle.is_null() {
        (*info).handle = dpx_open_truetype_file(name);
        if (*info).handle.is_null() {
            return CID_OPEN_ERROR_CANNOT_OPEN_FILE;
        }
    }
    (*info).sfont = sfnt_open((*info).handle);
    if (*info).sfont.is_null() {
        return CID_OPEN_ERROR_NOT_SFNT_FONT;
    }
    if (*(*info).sfont).type_0 == 1i32 << 4i32 {
        offset = ttc_read_offset((*info).sfont, index)
    }
    if (*(*info).sfont).type_0 != 1i32 << 4i32 && (*(*info).sfont).type_0 != 1i32 << 2i32
        || sfnt_read_table_directory((*info).sfont, offset) < 0i32
        || {
            offset = sfnt_find_table_pos((*info).sfont, b"CFF \x00" as *const u8 as *const i8);
            offset == 0_u32
        }
    {
        CIDFontInfo_close(info);
        return CID_OPEN_ERROR_NO_CFF_TABLE;
    }
    (*info).cffont = cff_open((*(*info).sfont).handle, offset as i32, 0i32);
    if (*info).cffont.is_null() {
        return CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT;
    }
    is_cid = (*(*info).cffont).flag & 1i32 << 0i32;
    if required_cid != is_cid {
        CIDFontInfo_close(info);
        return (if required_cid != 0 {
            CID_OPEN_ERROR_NOT_CIDFONT as i32
        } else {
            CID_OPEN_ERROR_IS_CIDFONT as i32
        }) as CIDType0Error;
    }
    CID_OPEN_ERROR_NO_ERROR
}
unsafe extern "C" fn CIDFont_type0_add_CIDSet(
    mut font: *mut CIDFont,
    mut used_chars: *mut i8,
    mut last_cid: card16,
) {
    /*
     * CIDSet:
     * Length of CIDSet stream is not clear. Must be 8192 bytes long?
     */
    let mut cidset: *mut pdf_obj = 0 as *mut pdf_obj;
    cidset = pdf_new_stream(1i32 << 0i32);
    pdf_add_stream(
        cidset,
        used_chars as *const libc::c_void,
        last_cid as i32 / 8i32 + 1i32,
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"CIDSet\x00" as *const u8 as *const i8),
        pdf_ref_obj(cidset),
    );
    pdf_release_obj(cidset);
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_dofont(mut font: *mut CIDFont) {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut charstrings: *mut cff_index = 0 as *mut cff_index;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut fdselect: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut charstring_len: i32 = 0;
    let mut max_len: i32 = 0;
    let mut destlen: i32 = 0i32;
    let mut size: i32 = 0;
    let mut offset: i32 = 0i32;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut num_glyphs: card16 = 0i32 as card16;
    let mut gid: card16 = 0;
    let mut cid: i32 = 0;
    let mut cs_count: card16 = 0;
    let mut last_cid: card16 = 0i32 as card16;
    let mut fd: i32 = 0;
    let mut prev_fd: i32 = 0;
    let mut used_chars: *mut i8 = 0 as *mut i8;
    let mut CIDToGIDMap: *mut u8 = 0 as *mut u8;
    let mut error: CIDType0Error = CID_OPEN_ERROR_NO_ERROR;
    let mut info: CIDType0Info = CIDType0Info {
        handle: 0 as *mut libc::c_void,
        sfont: 0 as *mut sfnt,
        cffont: 0 as *mut cff_font,
    };
    assert!(!font.is_null());
    if (*font).indirect.is_null() {
        return;
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
        pdf_ref_obj((*font).descriptor),
    );
    if CIDFont_is_BaseFont(font) {
        return;
    } else {
        if CIDFont_get_embedding(font) == 0 && opt_flags & 1i32 << 1i32 != 0 {
            /* No metrics needed. */
            pdf_add_dict(
                (*font).fontdict,
                pdf_new_name(b"DW\x00" as *const u8 as *const i8),
                pdf_new_number(1000.0f64),
            );
            return;
        }
    }
    used_chars = CIDFont_type0_get_used_chars(font);
    error = CIDFont_type0_try_open((*font).ident, CIDFont_get_opt_index(font), 1i32, &mut info);
    if error as i32 != CID_OPEN_ERROR_NO_ERROR as i32 {
        CIDType0Error_Show(error, (*font).ident);
        return;
    }
    cffont = info.cffont;
    cff_read_charsets(cffont);
    /*
     * DW, W, DW2 and W2:
     * Those values are obtained from OpenType table (not TFM).
     */
    if opt_flags & 1i32 << 1i32 != 0 {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"DW\x00" as *const u8 as *const i8),
            pdf_new_number(1000.0f64),
        );
    } else {
        let mut cid_count: i32 = 0;
        if cff_dict_known((*cffont).topdict, b"CIDCount\x00" as *const u8 as *const i8) != 0 {
            cid_count = cff_dict_get(
                (*cffont).topdict,
                b"CIDCount\x00" as *const u8 as *const i8,
                0i32,
            ) as i32
        } else {
            cid_count = 65535i32 + 1i32
        }
        CIDToGIDMap = new(((2i32 * cid_count) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        memset(
            CIDToGIDMap as *mut libc::c_void,
            0i32,
            (2i32 * cid_count) as u64,
        );
        let ref mut fresh0 = *used_chars.offset((0i32 / 8i32) as isize);
        *fresh0 = (*fresh0 as i32 | 1i32 << 7i32 - 0i32 % 8i32) as i8;
        /* .notdef */
        cid = 0i32;
        while cid <= 65535i32 {
            if *used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 != 0 {
                gid = cff_charsets_lookup(cffont, cid as card16);
                if cid != 0i32 && gid as i32 == 0i32 {
                    dpx_warning(
                        b"Glyph for CID %u missing in font \"%s\".\x00" as *const u8 as *const i8,
                        cid as CID as i32,
                        (*font).ident,
                    );
                    let ref mut fresh1 = *used_chars.offset((cid / 8i32) as isize);
                    *fresh1 = (*fresh1 as i32 & !(1i32 << 7i32 - cid % 8i32)) as i8
                } else {
                    *CIDToGIDMap.offset((2i32 * cid) as isize) =
                        (gid as i32 >> 8i32 & 0xffi32) as u8;
                    *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) =
                        (gid as i32 & 0xffi32) as u8;
                    last_cid = cid as card16;
                    num_glyphs = num_glyphs.wrapping_add(1)
                }
            }
            cid += 1
        }
        add_CIDMetrics(
            info.sfont,
            (*font).fontdict,
            CIDToGIDMap,
            last_cid,
            if CIDFont_get_parent_id(font, 1i32) < 0i32 {
                0i32
            } else {
                1i32
            },
        );
    }
    if CIDFont_get_embedding(font) == 0 {
        free(CIDToGIDMap as *mut libc::c_void);
        CIDFontInfo_close(&mut info);
        return;
    }
    /*
     * Embed font subset.
     */
    cff_read_fdselect(cffont);
    cff_read_fdarray(cffont);
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    offset = cff_dict_get(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    ttstub_input_seek(
        (*cffont).handle,
        (*cffont).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    idx = cff_get_index_header(cffont);
    /* offset is now absolute offset ... bad */
    offset = ttstub_input_seek((*cffont).handle, 0i32 as ssize_t, 1i32) as i32;
    cs_count = (*idx).count;
    if (cs_count as i32) < 2i32 {
        panic!("No valid charstring data found.");
    }
    /* New Charsets data */
    charset = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_charsets>() as u64) as u32)
        as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = 0i32 as card16;
    (*charset).data.glyphs = new((num_glyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<s_SID>() as u64)
        as u32) as *mut s_SID;
    /* New FDSelect data */
    fdselect = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_fdselect>() as u64) as u32)
        as *mut cff_fdselect;
    (*fdselect).format = 3i32 as card8;
    (*fdselect).num_entries = 0i32 as card16;
    (*fdselect).data.ranges = new((num_glyphs as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<cff_range3>() as u64)
        as u32) as *mut cff_range3;
    /* New CharStrings INDEX */
    charstrings = cff_new_index((num_glyphs as i32 + 1i32) as card16);
    max_len = 2i32 * 65536i32;
    (*charstrings).data =
        new((max_len as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32)
            as *mut card8;
    charstring_len = 0i32;
    /*
     * TODO: Re-assign FD number.
     */
    prev_fd = -1i32;
    gid = 0i32 as card16;
    data =
        new((65536_u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32) as *mut card8;
    cid = 0i32;
    while cid <= last_cid as i32 {
        let mut gid_org: u16 = 0;
        if !(*used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 == 0) {
            gid_org = ((*CIDToGIDMap.offset((2i32 * cid) as isize) as i32) << 8i32
                | *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) as i32)
                as u16;
            size = (*(*idx).offset.offset((gid_org as i32 + 1i32) as isize))
                .wrapping_sub(*(*idx).offset.offset(gid_org as isize)) as i32;
            if size > 65536i32 {
                panic!("Charstring too long: gid={}", gid_org);
            }
            if charstring_len + 65536i32 >= max_len {
                max_len = charstring_len + 2i32 * 65536i32;
                (*charstrings).data = renew(
                    (*charstrings).data as *mut libc::c_void,
                    (max_len as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64)
                        as u32,
                ) as *mut card8
            }
            *(*charstrings).offset.offset(gid as isize) = (charstring_len + 1i32) as l_offset;
            ttstub_input_seek(
                (*cffont).handle,
                (offset as u32)
                    .wrapping_add(*(*idx).offset.offset(gid_org as isize))
                    .wrapping_sub(1_u32) as ssize_t,
                0i32,
            );
            ttstub_input_read((*cffont).handle, data as *mut i8, size as size_t);
            fd = cff_fdselect_lookup(cffont, gid_org) as i32;
            charstring_len += cs_copy_charstring(
                (*charstrings).data.offset(charstring_len as isize),
                max_len - charstring_len,
                data,
                size,
                (*cffont).gsubr,
                *(*cffont).subrs.offset(fd as isize),
                0i32 as f64,
                0i32 as f64,
                0 as *mut cs_ginfo,
            );
            if cid > 0i32 && gid_org as i32 > 0i32 {
                *(*charset)
                    .data
                    .glyphs
                    .offset((*charset).num_entries as isize) = cid as s_SID;
                (*charset).num_entries = ((*charset).num_entries as i32 + 1i32) as card16
            }
            if fd != prev_fd {
                (*(*fdselect)
                    .data
                    .ranges
                    .offset((*fdselect).num_entries as isize))
                .first = gid;
                (*(*fdselect)
                    .data
                    .ranges
                    .offset((*fdselect).num_entries as isize))
                .fd = fd as card8;
                (*fdselect).num_entries = ((*fdselect).num_entries as i32 + 1i32) as card16;
                prev_fd = fd
            }
            gid = gid.wrapping_add(1)
        }
        cid += 1
    }
    if gid as i32 != num_glyphs as i32 {
        panic!("Unexpeced error: ?????");
    }
    free(data as *mut libc::c_void);
    cff_release_index(idx);
    free(CIDToGIDMap as *mut libc::c_void);
    *(*charstrings).offset.offset(num_glyphs as isize) = (charstring_len + 1i32) as l_offset;
    (*charstrings).count = num_glyphs;
    (*cffont).num_glyphs = num_glyphs;
    (*cffont).cstrings = charstrings;
    /* discard old one, set new data */
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    cff_release_fdselect((*cffont).fdselect);
    (*cffont).fdselect = fdselect;
    /* no Global subr */
    if !(*cffont).gsubr.is_null() {
        cff_release_index((*cffont).gsubr);
    }
    (*cffont).gsubr = cff_new_index(0i32 as card16);
    fd = 0i32;
    while fd < (*cffont).num_fds as i32 {
        if !(*cffont).subrs.is_null() && !(*(*cffont).subrs.offset(fd as isize)).is_null() {
            cff_release_index(*(*cffont).subrs.offset(fd as isize));
            let ref mut fresh2 = *(*cffont).subrs.offset(fd as isize);
            *fresh2 = 0 as *mut cff_index
        }
        if !(*cffont).private.is_null() && !(*(*cffont).private.offset(fd as isize)).is_null() {
            cff_dict_remove(
                *(*cffont).private.offset(fd as isize),
                b"Subrs\x00" as *const u8 as *const i8,
            );
            /* no Subrs */
        }
        fd += 1
    }
    destlen = write_fontfile(font, cffont);
    CIDFontInfo_close(&mut info);
    if verbose > 1i32 {
        info!("[{}/{} glyphs][{} bytes]", num_glyphs, cs_count, destlen,);
    }
    CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_open(
    mut font: *mut CIDFont,
    mut name: *const i8,
    mut cmap_csi: *mut CIDSysInfo,
    mut opt: *mut cid_opt,
    mut expected_flag: i32,
) -> i32 {
    let mut csi: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut offset: u32 = 0_u32;
    let mut is_cid_font: i32 = 0i32;
    let mut expect_cid_font: i32 = (expected_flag == 0i32) as i32;
    let mut expect_type1_font: i32 = expected_flag & 1i32 << 8i32;
    assert!(!font.is_null());
    if expect_type1_font != 0 {
        if !cmap_csi.is_null()
            && (strcmp((*cmap_csi).registry, b"Adobe\x00" as *const u8 as *const i8) != 0i32
                || strcmp(
                    (*cmap_csi).ordering,
                    b"Identity\x00" as *const u8 as *const i8,
                ) != 0i32)
        {
            return -1i32;
        }
    }
    if expect_type1_font != 0 {
        handle = dpx_open_type1_file(name)
    } else {
        handle = dpx_open_opentype_file(name)
    }
    if expect_type1_font == 0 {
        if handle.is_null() {
            handle = dpx_open_truetype_file(name);
            if handle.is_null() {
                return -1i32;
            }
        }
        sfont = sfnt_open(handle);
        if sfont.is_null() {
            _tt_abort(
                b"Not a CFF/OpenType font: %s\x00" as *const u8 as *const i8,
                name,
            );
        }
        if (*sfont).type_0 == 1i32 << 4i32 {
            offset = ttc_read_offset(sfont, (*opt).index)
        }
        if (*sfont).type_0 != 1i32 << 4i32 && (*sfont).type_0 != 1i32 << 2i32
            || sfnt_read_table_directory(sfont, offset) < 0i32
            || {
                offset = sfnt_find_table_pos(sfont, b"CFF \x00" as *const u8 as *const i8);
                offset == 0_u32
            }
        {
            sfnt_close(sfont);
            if !handle.is_null() {
                ttstub_input_close(handle);
            }
            return -1i32;
        }
        cffont = cff_open((*sfont).handle, offset as i32, 0i32);
        if cffont.is_null() {
            panic!("Cannot read CFF font data");
        }
        is_cid_font = (*cffont).flag & 1i32 << 0i32;
        if expect_cid_font != is_cid_font {
            cff_close(cffont);
            sfnt_close(sfont);
            if !handle.is_null() {
                ttstub_input_close(handle);
            }
            return -1i32;
        }
        if is_cid_font != 0 {
            cff_read_charsets(cffont);
            (*opt).cff_charsets = (*cffont).charsets as *mut libc::c_void;
            (*cffont).charsets = 0 as *mut cff_charsets
        }
    } else {
        if handle.is_null() {
            return -1i32;
        }
        cffont = t1_load_font(0 as *mut *mut i8, 1i32, handle);
        if cffont.is_null() {
            ttstub_input_close(handle);
            return -1i32;
        }
        ttstub_input_close(handle);
    }
    csi = new((1_u64).wrapping_mul(::std::mem::size_of::<CIDSysInfo>() as u64) as u32)
        as *mut CIDSysInfo;
    if is_cid_font != 0 {
        (*csi).registry = cff_get_string(
            cffont,
            cff_dict_get(
                (*cffont).topdict,
                b"ROS\x00" as *const u8 as *const i8,
                0i32,
            ) as s_SID,
        );
        (*csi).ordering = cff_get_string(
            cffont,
            cff_dict_get(
                (*cffont).topdict,
                b"ROS\x00" as *const u8 as *const i8,
                1i32,
            ) as s_SID,
        );
        (*csi).supplement = cff_dict_get(
            (*cffont).topdict,
            b"ROS\x00" as *const u8 as *const i8,
            2i32,
        ) as i32
    } else {
        (*csi).registry = new((strlen(b"Adobe\x00" as *const u8 as *const i8)
            .wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        strcpy((*csi).registry, b"Adobe\x00" as *const u8 as *const i8);
        (*csi).ordering = new((strlen(b"Identity\x00" as *const u8 as *const i8)
            .wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        strcpy((*csi).ordering, b"Identity\x00" as *const u8 as *const i8);
        (*csi).supplement = 0i32
    }
    if expect_type1_font == 0 && !cmap_csi.is_null() {
        if strcmp((*csi).registry, (*cmap_csi).registry) != 0i32
            || strcmp((*csi).ordering, (*cmap_csi).ordering) != 0i32
        {
            info!("\nCharacter collection mismatched:\n");
            dpx_message(
                b"\tFont: %s-%s-%d\n\x00" as *const u8 as *const i8,
                (*csi).registry,
                (*csi).ordering,
                (*csi).supplement,
            );
            dpx_message(
                b"\tCMap: %s-%s-%d\n\x00" as *const u8 as *const i8,
                (*cmap_csi).registry,
                (*cmap_csi).ordering,
                (*cmap_csi).supplement,
            );
            panic!("Inconsistent CMap specified for this font.");
        }
        if (*csi).supplement < (*cmap_csi).supplement {
            warn!("CMap have higher supplmement number.");
            warn!("Some characters may not be displayed or printed.");
        }
    }
    let mut shortname: *mut i8 = 0 as *mut i8;
    let mut fontname_len: i32 = 8i32;
    shortname = cff_get_name(cffont);
    if shortname.is_null() {
        panic!("No valid FontName found.");
    }
    /*
     * Mangled name requires more 7 bytes.
     * Style requires more 11 bytes.
     */
    if is_cid_font != 0 {
        fontname_len += 11i32
    }
    fontname = new(
        (strlen(shortname).wrapping_add(fontname_len as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    memset(
        fontname as *mut libc::c_void,
        0i32,
        strlen(shortname).wrapping_add(fontname_len as u64),
    );
    strcpy(fontname, shortname);
    free(shortname as *mut libc::c_void);
    cff_close(cffont);
    if is_cid_font != 0 {
        if (*opt).embed != 0 && (*opt).style != 0i32 {
            dpx_warning(
                b"Embedding disabled due to style option for %s.\x00" as *const u8 as *const i8,
                name,
            );
            (*opt).embed = 0i32
        }
        match (*opt).style {
            1 => {
                strcat(fontname, b",Bold\x00" as *const u8 as *const i8);
            }
            2 => {
                strcat(fontname, b",Italic\x00" as *const u8 as *const i8);
            }
            3 => {
                strcat(fontname, b",BoldItalic\x00" as *const u8 as *const i8);
            }
            _ => {}
        }
    } else if expect_type1_font != 0 {
        if (*opt).style != 0i32 {
            warn!(",Bold, ,Italic, ... not supported for this type of font...");
            (*opt).style = 0i32
        }
    } else {
        (*opt).embed = 1i32
    }
    (*font).fontname = fontname;
    (*font).subtype = 1i32;
    (*font).csi = csi;
    (*font).flags |= expected_flag;
    (*font).fontdict = pdf_new_dict();
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Font\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"CIDFontType0\x00" as *const u8 as *const i8),
    );
    if expect_type1_font != 0 || (*opt).embed != 0 {
        memmove(
            fontname.offset(7) as *mut libc::c_void,
            fontname as *const libc::c_void,
            strlen(fontname).wrapping_add(1i32 as u64),
        );
        pdf_font_make_uniqueTag(fontname);
        *fontname.offset(6) = '+' as i32 as i8
    }
    if expect_type1_font != 0 {
        (*font).descriptor = pdf_new_dict()
    } else {
        /* getting font info. from TrueType tables */
        (*font).descriptor = tt_get_fontdesc(sfont, &mut (*opt).embed, (*opt).stemv, 0i32, name);
        if (*font).descriptor.is_null() {
            panic!("Could not obtain necessary font info.");
        }
    }
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
        pdf_new_name(fontname),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"BaseFont\x00" as *const u8 as *const i8),
        pdf_new_name(fontname),
    );
    let mut csi_dict: *mut pdf_obj = pdf_new_dict();
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Registry\x00" as *const u8 as *const i8),
        pdf_new_string(
            (*csi).registry as *const libc::c_void,
            strlen((*csi).registry),
        ),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Ordering\x00" as *const u8 as *const i8),
        pdf_new_string(
            (*csi).ordering as *const libc::c_void,
            strlen((*csi).ordering),
        ),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Supplement\x00" as *const u8 as *const i8),
        pdf_new_number((*csi).supplement as f64),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"CIDSystemInfo\x00" as *const u8 as *const i8),
        csi_dict,
    );
    if is_cid_font != 0 {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"DW\x00" as *const u8 as *const i8),
            pdf_new_number(1000i32 as f64),
        );
        /* not sure */
    }
    if expect_type1_font == 0 {
        sfnt_close(sfont);
        if !handle.is_null() {
            ttstub_input_close(handle);
        }
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_t1cdofont(mut font: *mut CIDFont) {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut charstrings: *mut cff_index = 0 as *mut cff_index;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut charstring_len: i32 = 0;
    let mut max_len: i32 = 0;
    let mut destlen: i32 = 0i32;
    let mut size: i32 = 0;
    let mut offset: i32 = 0i32;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut num_glyphs: card16 = 0;
    let mut gid: card16 = 0;
    let mut last_cid: card16 = 0;
    let mut i: i32 = 0;
    let mut cid: i32 = 0;
    let mut used_chars: *mut i8 = 0 as *mut i8;
    let mut default_width: f64 = 0.;
    let mut nominal_width: f64 = 0.;
    let mut error: CIDType0Error = CID_OPEN_ERROR_NO_ERROR;
    let mut info: CIDType0Info = CIDType0Info {
        handle: 0 as *mut libc::c_void,
        sfont: 0 as *mut sfnt,
        cffont: 0 as *mut cff_font,
    };
    assert!(!font.is_null());
    if (*font).indirect.is_null() {
        return;
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
        pdf_ref_obj((*font).descriptor),
    );
    used_chars = CIDFont_type0_get_used_chars(font);
    error = CIDFont_type0_try_open((*font).ident, CIDFont_get_opt_index(font), 0i32, &mut info);
    if error as i32 != CID_OPEN_ERROR_NO_ERROR as i32 {
        CIDType0Error_Show(error, (*font).ident);
        return;
    }
    cffont = info.cffont;
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    if !(*(*cffont).private.offset(0)).is_null()
        && cff_dict_known(
            *(*cffont).private.offset(0),
            b"StdVW\x00" as *const u8 as *const i8,
        ) != 0
    {
        let mut stemv: f64 = 0.;
        stemv = cff_dict_get(
            *(*cffont).private.offset(0),
            b"StdVW\x00" as *const u8 as *const i8,
            0i32,
        );
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"StemV\x00" as *const u8 as *const i8),
            pdf_new_number(stemv),
        );
    }
    if !(*(*cffont).private.offset(0)).is_null()
        && cff_dict_known(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const i8,
        ) != 0
    {
        default_width = cff_dict_get(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const i8,
            0i32,
        )
    } else {
        default_width = 0.0f64
    }
    if !(*(*cffont).private.offset(0)).is_null()
        && cff_dict_known(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const i8,
        ) != 0
    {
        nominal_width = cff_dict_get(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const i8,
            0i32,
        )
    } else {
        nominal_width = 0.0f64
    }
    num_glyphs = 0i32 as card16;
    last_cid = 0i32 as card16;
    let ref mut fresh3 = *used_chars.offset((0i32 / 8i32) as isize);
    *fresh3 = (*fresh3 as i32 | 1i32 << 7i32 - 0i32 % 8i32) as i8;
    /* .notdef */
    i = 0i32;
    while i < ((*cffont).num_glyphs as i32 + 7i32) / 8i32 {
        let mut c: i32 = 0;
        let mut j: i32 = 0;
        c = *used_chars.offset(i as isize) as i32;
        j = 7i32;
        while j >= 0i32 {
            if c & 1i32 << j != 0 {
                num_glyphs = num_glyphs.wrapping_add(1);
                last_cid = ((i + 1i32) * 8i32 - j - 1i32) as card16
            }
            j -= 1
        }
        i += 1
    }
    let mut fdselect: *mut cff_fdselect = 0 as *mut cff_fdselect;
    fdselect = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_fdselect>() as u64) as u32)
        as *mut cff_fdselect;
    (*fdselect).format = 3i32 as card8;
    (*fdselect).num_entries = 1i32 as card16;
    (*fdselect).data.ranges =
        new((1_u64).wrapping_mul(::std::mem::size_of::<cff_range3>() as u64) as u32)
            as *mut cff_range3;
    (*(*fdselect).data.ranges.offset(0)).first = 0i32 as card16;
    (*(*fdselect).data.ranges.offset(0)).fd = 0i32 as card8;
    (*cffont).fdselect = fdselect;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    charset = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_charsets>() as u64) as u32)
        as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = (num_glyphs as i32 - 1i32) as card16;
    (*charset).data.glyphs = new(((num_glyphs as i32 - 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<s_SID>() as u64)
        as u32) as *mut s_SID;
    gid = 0i32 as card16;
    cid = 0i32;
    while cid <= last_cid as i32 {
        if *used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 != 0 {
            if gid as i32 > 0i32 {
                *(*charset).data.glyphs.offset((gid as i32 - 1i32) as isize) = cid as s_SID
            }
            gid = gid.wrapping_add(1)
        }
        cid += 1
    }
    /* cff_release_charsets(cffont->charsets); */
    (*cffont).charsets = charset; /* FIXME: Skip XXXXXX+ */
    cff_dict_add(
        (*cffont).topdict,
        b"CIDCount\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CIDCount\x00" as *const u8 as *const i8,
        0i32,
        (last_cid as i32 + 1i32) as f64,
    );
    (*cffont).fdarray =
        new((1_u64).wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64) as u32)
            as *mut *mut cff_dict;
    let ref mut fresh4 = *(*cffont).fdarray.offset(0);
    *fresh4 = cff_new_dict();
    cff_dict_add(
        *(*cffont).fdarray.offset(0),
        b"FontName\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0),
        b"FontName\x00" as *const u8 as *const i8,
        0i32,
        cff_add_string(cffont, (*font).fontname.offset(7), 1i32) as f64,
    );
    cff_dict_add(
        *(*cffont).fdarray.offset(0),
        b"Private\x00" as *const u8 as *const i8,
        2i32,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0),
        b"Private\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0),
        b"Private\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    /* FDArray  - index offset, not known yet */
    cff_dict_add(
        (*cffont).topdict,
        b"FDArray\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDArray\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    /* FDSelect - offset, not known yet */
    cff_dict_add(
        (*cffont).topdict,
        b"FDSelect\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDSelect\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    cff_dict_remove((*cffont).topdict, b"UniqueID\x00" as *const u8 as *const i8);
    cff_dict_remove((*cffont).topdict, b"XUID\x00" as *const u8 as *const i8);
    cff_dict_remove((*cffont).topdict, b"Private\x00" as *const u8 as *const i8);
    cff_dict_remove((*cffont).topdict, b"Encoding\x00" as *const u8 as *const i8);
    /* */
    offset = cff_dict_get(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    ttstub_input_seek(
        (*cffont).handle,
        (*cffont).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    idx = cff_get_index_header(cffont);
    /* offset is now absolute offset ... bad */
    offset = ttstub_input_seek((*cffont).handle, 0i32 as ssize_t, 1i32) as i32;
    if ((*idx).count as i32) < 2i32 {
        panic!("No valid charstring data found.");
    }
    /* New CharStrings INDEX */
    charstrings = cff_new_index((num_glyphs as i32 + 1i32) as card16);
    max_len = 2i32 * 65536i32;
    (*charstrings).data =
        new((max_len as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32)
            as *mut card8;
    charstring_len = 0i32;
    gid = 0i32 as card16;
    data =
        new((65536_u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32) as *mut card8;
    cid = 0i32;
    while cid <= last_cid as i32 {
        if !(*used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 == 0) {
            size = (*(*idx).offset.offset((cid + 1i32) as isize))
                .wrapping_sub(*(*idx).offset.offset(cid as isize)) as i32;
            if size > 65536i32 {
                panic!("Charstring too long: gid={}", cid);
            }
            if charstring_len + 65536i32 >= max_len {
                max_len = charstring_len + 2i32 * 65536i32;
                (*charstrings).data = renew(
                    (*charstrings).data as *mut libc::c_void,
                    (max_len as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64)
                        as u32,
                ) as *mut card8
            }
            *(*charstrings).offset.offset(gid as isize) = (charstring_len + 1i32) as l_offset;
            ttstub_input_seek(
                (*cffont).handle,
                (offset as u32)
                    .wrapping_add(*(*idx).offset.offset(cid as isize))
                    .wrapping_sub(1_u32) as ssize_t,
                0i32,
            );
            ttstub_input_read((*cffont).handle, data as *mut i8, size as size_t);
            charstring_len += cs_copy_charstring(
                (*charstrings).data.offset(charstring_len as isize),
                max_len - charstring_len,
                data,
                size,
                (*cffont).gsubr,
                *(*cffont).subrs.offset(0),
                default_width,
                nominal_width,
                0 as *mut cs_ginfo,
            );
            gid = gid.wrapping_add(1)
        }
        cid += 1
    }
    if gid as i32 != num_glyphs as i32 {
        panic!("Unexpeced error: ?????");
    }
    free(data as *mut libc::c_void);
    cff_release_index(idx);
    *(*charstrings).offset.offset(num_glyphs as isize) = (charstring_len + 1i32) as l_offset;
    (*charstrings).count = num_glyphs;
    (*cffont).num_glyphs = num_glyphs;
    (*cffont).cstrings = charstrings;
    /* no Global subr */
    if !(*cffont).gsubr.is_null() {
        cff_release_index((*cffont).gsubr);
    }
    (*cffont).gsubr = cff_new_index(0i32 as card16);
    if !(*cffont).subrs.is_null() && !(*(*cffont).subrs.offset(0)).is_null() {
        cff_release_index(*(*cffont).subrs.offset(0));
        let ref mut fresh5 = *(*cffont).subrs.offset(0);
        *fresh5 = 0 as *mut cff_index
    }
    if !(*cffont).private.is_null() && !(*(*cffont).private.offset(0)).is_null() {
        cff_dict_remove(
            *(*cffont).private.offset(0),
            b"Subrs\x00" as *const u8 as *const i8,
        );
        /* no Subrs */
    }
    cff_add_string(cffont, b"Adobe\x00" as *const u8 as *const i8, 1i32);
    cff_add_string(cffont, b"Identity\x00" as *const u8 as *const i8, 1i32);
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(*(*cffont).private.offset(0), cffont);
    cff_update_string(cffont);
    /* CFF code need to be rewrote... */
    cff_dict_add(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        3i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        0i32,
        cff_get_sid(cffont, b"Adobe\x00" as *const u8 as *const i8) as f64,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        1i32,
        cff_get_sid(cffont, b"Identity\x00" as *const u8 as *const i8) as f64,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        2i32,
        0.0f64,
    );
    destlen = write_fontfile(font, cffont);
    /*
     * DW, W, DW2 and W2:
     * Those values are obtained from OpenType table (not TFM).
     */
    let mut CIDToGIDMap: *mut u8 = 0 as *mut u8;
    CIDToGIDMap = new(((2i32 * (last_cid as i32 + 1i32)) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    memset(
        CIDToGIDMap as *mut libc::c_void,
        0i32,
        (2i32 * (last_cid as i32 + 1i32)) as u64,
    );
    cid = 0i32;
    while cid <= last_cid as i32 {
        if *used_chars.offset((cid / 8i32) as isize) as i32 & 1i32 << 7i32 - cid % 8i32 != 0 {
            *CIDToGIDMap.offset((2i32 * cid) as isize) = (cid >> 8i32 & 0xffi32) as u8;
            *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) = (cid & 0xffi32) as u8
        }
        cid += 1
    }
    add_CIDMetrics(
        info.sfont,
        (*font).fontdict,
        CIDToGIDMap,
        last_cid,
        if CIDFont_get_parent_id(font, 1i32) < 0i32 {
            0i32
        } else {
            1i32
        },
    );
    free(CIDToGIDMap as *mut libc::c_void);
    CIDFontInfo_close(&mut info);
    if verbose > 1i32 {
        info!("[{} glyphs][{} bytes]", num_glyphs, destlen);
    }
    CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
unsafe extern "C" fn load_base_CMap(
    mut font_name: *const i8,
    mut wmode: i32,
    mut cffont: *mut cff_font,
) -> i32 {
    let mut cmap_id: i32 = -1i32;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut cmap_name: *mut i8 = 0 as *mut i8;
    let mut gid: card16 = 0;
    let mut range_min: [u8; 4] = [0; 4];
    let mut range_max: [u8; 4] = [0x7f, 0xff, 0xff, 0xff];
    cmap_name = new((strlen(font_name)
        .wrapping_add(strlen(b"-UCS4-H\x00" as *const u8 as *const i8))
        .wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    if wmode != 0 {
        sprintf(
            cmap_name,
            b"%s-UCS4-V\x00" as *const u8 as *const i8,
            font_name,
        );
    } else {
        sprintf(
            cmap_name,
            b"%s-UCS4-H\x00" as *const u8 as *const i8,
            font_name,
        );
    }
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0i32 {
        free(cmap_name as *mut libc::c_void);
        return cmap_id;
    }
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_type(cmap, 1i32);
    CMap_set_wmode(cmap, wmode);
    CMap_add_codespacerange(
        cmap,
        range_min.as_mut_ptr(),
        range_max.as_mut_ptr(),
        4i32 as size_t,
    );
    CMap_set_CIDSysInfo(cmap, &mut CSI_IDENTITY);
    free(cmap_name as *mut libc::c_void);
    gid = 1i32 as card16;
    while (gid as i32) < (*cffont).num_glyphs as i32 {
        let mut ucv: i32 = 0;
        let mut sid: s_SID = 0;
        let mut glyph: *mut i8 = 0 as *mut i8;
        let mut name: *mut i8 = 0 as *mut i8;
        let mut suffix: *mut i8 = 0 as *mut i8;
        let mut srcCode: [u8; 4] = [0; 4];
        sid = cff_charsets_lookup_inverse(cffont, gid);
        glyph = cff_get_string(cffont, sid);
        name = agl_chop_suffix(glyph, &mut suffix);
        if name.is_null() {
            free(suffix as *mut libc::c_void);
            free(glyph as *mut libc::c_void);
        } else if !suffix.is_null() {
            free(name as *mut libc::c_void);
            free(suffix as *mut libc::c_void);
            free(glyph as *mut libc::c_void);
        } else {
            if agl_name_is_unicode(name) {
                ucv = agl_name_convert_unicode(name);
                srcCode[0] = (ucv >> 24i32 & 0xffi32) as u8;
                srcCode[1] = (ucv >> 16i32 & 0xffi32) as u8;
                srcCode[2] = (ucv >> 8i32 & 0xffi32) as u8;
                srcCode[3] = (ucv & 0xffi32) as u8;
                CMap_add_cidchar(cmap, srcCode.as_mut_ptr(), 4i32 as size_t, gid);
            } else {
                let mut agln: *mut agl_name = 0 as *mut agl_name;
                agln = agl_lookup_list(name);
                if agln.is_null() {
                    dpx_warning(
                        b"Glyph \"%s\" inaccessible (no Unicode mapping)\x00" as *const u8
                            as *const i8,
                        glyph,
                    );
                }
                while !agln.is_null() {
                    if (*agln).n_components > 1i32 {
                        dpx_warning(
                            b"Glyph \"%s\" inaccessible (composite character)\x00" as *const u8
                                as *const i8,
                            glyph,
                        );
                    } else if (*agln).n_components == 1i32 {
                        ucv = (*agln).unicodes[0];
                        srcCode[0] = (ucv >> 24i32 & 0xffi32) as u8;
                        srcCode[1] = (ucv >> 16i32 & 0xffi32) as u8;
                        srcCode[2] = (ucv >> 8i32 & 0xffi32) as u8;
                        srcCode[3] = (ucv & 0xffi32) as u8;
                        CMap_add_cidchar(cmap, srcCode.as_mut_ptr(), 4i32 as size_t, gid);
                    }
                    agln = (*agln).alternate
                }
            }
            free(name as *mut libc::c_void);
            free(suffix as *mut libc::c_void);
            free(glyph as *mut libc::c_void);
        }
        gid = gid.wrapping_add(1)
    }
    cmap_id = CMap_cache_add(cmap);
    cmap_id
}
#[no_mangle]
pub unsafe extern "C" fn t1_load_UnicodeCMap(
    mut font_name: *const i8,
    mut otl_tags: *const i8,
    mut wmode: i32,
) -> i32 {
    let mut cmap_id: i32 = -1i32;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    if font_name.is_null() {
        return -1i32;
    }
    handle = dpx_open_type1_file(font_name) as *mut rust_input_handle_t;
    if handle.is_null() {
        return -1i32;
    }
    cffont = t1_load_font(0 as *mut *mut i8, 1i32, handle as rust_input_handle_t);
    ttstub_input_close(handle as rust_input_handle_t);
    if cffont.is_null() {
        return -1i32;
    }
    cmap_id = load_base_CMap(font_name, wmode, cffont);
    cff_close(cffont);
    if cmap_id < 0i32 {
        _tt_abort(
            b"Failed to create Unicode charmap for font \"%s\".\x00" as *const u8 as *const i8,
            font_name,
        );
    }
    if !otl_tags.is_null() {
        warn!("Glyph substitution not supported for Type1 font yet...");
    }
    cmap_id
}
/*
 * ToUnicode CMap
 */
unsafe extern "C" fn create_ToUnicode_stream(
    mut cffont: *mut cff_font,
    mut font_name: *const i8,
    mut used_glyphs: *const i8,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut cid: CID = 0;
    let mut gid: card16 = 0;
    let mut glyph_count: i32 = 0;
    let mut total_fail_count: i32 = 0;
    let mut cmap_name: *mut i8 = 0 as *mut i8;
    let mut wbuf: [u8; 1024] = [0; 1024];
    let mut p: *mut u8 = 0 as *mut u8;
    let mut endptr: *mut u8 = 0 as *mut u8;
    static mut range_min: [u8; 2] = [0; 2];
    static mut range_max: [u8; 2] = [0xff, 0xff];
    if font_name.is_null() || used_glyphs.is_null() {
        return 0 as *mut pdf_obj;
    }
    cmap = CMap_new();
    cmap_name = new((strlen(font_name)
        .wrapping_add(strlen(b"-UTF16\x00" as *const u8 as *const i8))
        .wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy(cmap_name, font_name);
    strcat(cmap_name, b"-UTF16\x00" as *const u8 as *const i8);
    CMap_set_name(cmap, cmap_name);
    free(cmap_name as *mut libc::c_void);
    CMap_set_wmode(cmap, 0i32);
    CMap_set_type(cmap, 2i32);
    CMap_set_CIDSysInfo(cmap, &mut CSI_UNICODE);
    CMap_add_codespacerange(
        cmap,
        range_min.as_mut_ptr(),
        range_max.as_mut_ptr(),
        2i32 as size_t,
    );
    total_fail_count = 0i32;
    glyph_count = total_fail_count;
    p = wbuf.as_mut_ptr();
    endptr = wbuf.as_mut_ptr().offset(1024);
    cid = 1i32 as CID;
    while (cid as i32) < (*cffont).num_glyphs as i32 {
        /* Skip .notdef */
        if *used_glyphs.offset((cid as i32 / 8i32) as isize) as i32
            & 1i32 << 7i32 - cid as i32 % 8i32
            != 0
        {
            let mut glyph: *mut i8 = 0 as *mut i8;
            let mut len: i32 = 0;
            let mut fail_count: i32 = 0;
            wbuf[0] = (cid as i32 >> 8i32 & 0xffi32) as u8;
            wbuf[1] = (cid as i32 & 0xffi32) as u8;
            p = wbuf.as_mut_ptr().offset(2);
            gid = cff_charsets_lookup_inverse(cffont, cid);
            if !(gid as i32 == 0i32) {
                glyph = cff_get_string(cffont, gid);
                if !glyph.is_null() {
                    len = agl_sput_UTF16BE(glyph, &mut p, endptr, &mut fail_count);
                    if len < 1i32 || fail_count != 0 {
                        total_fail_count += fail_count
                    } else {
                        CMap_add_bfchar(
                            cmap,
                            wbuf.as_mut_ptr(),
                            2i32 as size_t,
                            wbuf.as_mut_ptr().offset(2),
                            len as size_t,
                        );
                    }
                    free(glyph as *mut libc::c_void);
                }
                glyph_count += 1
            }
        }
        cid = cid.wrapping_add(1)
    }
    if total_fail_count != 0i32 && total_fail_count >= glyph_count / 10i32 {
        warn!(
            "{} glyph names (out of {}) missing Unicode mapping.",
            total_fail_count, glyph_count,
        );
        dpx_warning(
            b"ToUnicode CMap \"%s-UTF16\" removed.\x00" as *const u8 as *const i8,
            font_name,
        );
    } else {
        stream = CMap_create_stream(cmap)
    }
    CMap_release(cmap);
    stream
}
/* Force bold at small text sizes */
/* pdf_font --> CIDFont */
unsafe extern "C" fn get_font_attr(mut font: *mut CIDFont, mut cffont: *mut cff_font) {
    let mut capheight: f64 = 0.;
    let mut ascent: f64 = 0.;
    let mut descent: f64 = 0.;
    let mut italicangle: f64 = 0.;
    let mut stemv: f64 = 0.;
    let mut defaultwidth: f64 = 0.;
    let mut nominalwidth: f64 = 0.;
    let mut flags: i32 = 0i32;
    let mut gid: i32 = 0;
    let mut i: i32 = 0;
    static mut L_c: [*const i8; 5] = [
        b"H\x00" as *const u8 as *const i8,
        b"P\x00" as *const u8 as *const i8,
        b"Pi\x00" as *const u8 as *const i8,
        b"Rho\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut L_d: [*const i8; 5] = [
        b"p\x00" as *const u8 as *const i8,
        b"q\x00" as *const u8 as *const i8,
        b"mu\x00" as *const u8 as *const i8,
        b"eta\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut L_a: [*const i8; 4] = [
        b"b\x00" as *const u8 as *const i8,
        b"h\x00" as *const u8 as *const i8,
        b"lambda\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut gm = t1_ginfo::new();
    defaultwidth = 500.0f64;
    nominalwidth = 0.0f64;
    /*
     * CapHeight, Ascent, and Descent is meaningfull only for Latin/Greek/Cyrillic.
     * The BlueValues and OtherBlues also have those information.
     */
    if cff_dict_known((*cffont).topdict, b"FontBBox\x00" as *const u8 as *const i8) != 0 {
        /* Default values */
        ascent = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const i8,
            3i32,
        );
        capheight = ascent;
        descent = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const i8,
            1i32,
        )
    } else {
        capheight = 680.0f64;
        ascent = 690.0f64;
        descent = -190.0f64
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"StdVW\x00" as *const u8 as *const i8,
    ) != 0
    {
        stemv = cff_dict_get(
            *(*cffont).private.offset(0),
            b"StdVW\x00" as *const u8 as *const i8,
            0i32,
        )
    } else {
        /*
         * We may use the following values for StemV:
         *  Thin - ExtraLight: <= 50
         *  Light: 71
         *  Regular(Normal): 88
         *  Medium: 109
         *  SemiBold(DemiBold): 135
         *  Bold - Heavy: >= 166
         */
        stemv = 88.0f64
    }
    if cff_dict_known(
        (*cffont).topdict,
        b"ItalicAngle\x00" as *const u8 as *const i8,
    ) != 0
    {
        italicangle = cff_dict_get(
            (*cffont).topdict,
            b"ItalicAngle\x00" as *const u8 as *const i8,
            0i32,
        );
        if italicangle != 0.0f64 {
            flags |= 1i32 << 6i32
        }
    } else {
        italicangle = 0.0f64
    }
    /*
     * Use "space", "H", "p", and "b" for various values.
     * Those characters should not "seac". (no accent)
     */
    gid = cff_glyph_lookup(cffont, b"space\x00" as *const u8 as *const i8) as i32;
    if gid >= 0i32 && gid < (*(*cffont).cstrings).count as i32 {
        t1char_get_metrics(
            (*(*cffont).cstrings)
                .data
                .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                .offset(-1),
            (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                as i32,
            *(*cffont).subrs.offset(0),
            &mut gm,
        );
        defaultwidth = gm.wx
    }
    i = 0i32;
    while !L_c[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_c[i as usize]) as i32;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as i32 {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as i32,
                *(*cffont).subrs.offset(0),
                &mut gm,
            );
            capheight = gm.bbox.ury;
            break;
        } else {
            i += 1
        }
    }
    i = 0i32;
    while !L_d[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_d[i as usize]) as i32;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as i32 {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as i32,
                *(*cffont).subrs.offset(0),
                &mut gm,
            );
            descent = gm.bbox.lly;
            break;
        } else {
            i += 1
        }
    }
    i = 0i32;
    while !L_a[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_a[i as usize]) as i32;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as i32 {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings).offset.offset((gid + 1i32) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as i32,
                *(*cffont).subrs.offset(0),
                &mut gm,
            );
            ascent = gm.bbox.ury;
            break;
        } else {
            i += 1
        }
    }
    if defaultwidth != 0.0f64 {
        cff_dict_add(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const i8,
            1i32,
        );
        cff_dict_set(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const i8,
            0i32,
            defaultwidth,
        );
    }
    if nominalwidth != 0.0f64 {
        cff_dict_add(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const i8,
            1i32,
        );
        cff_dict_set(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const i8,
            0i32,
            nominalwidth,
        );
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"ForceBold\x00" as *const u8 as *const i8,
    ) != 0
        && cff_dict_get(
            *(*cffont).private.offset(0),
            b"ForceBold\x00" as *const u8 as *const i8,
            0i32,
        ) != 0.
    {
        flags |= 1i32 << 18i32
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"IsFixedPitch\x00" as *const u8 as *const i8,
    ) != 0
        && cff_dict_get(
            *(*cffont).private.offset(0),
            b"IsFixedPitch\x00" as *const u8 as *const i8,
            0i32,
        ) != 0.
    {
        flags |= 1i32 << 0i32
    }
    if !(*font).fontname.is_null()
        && strstr((*font).fontname, b"Sans\x00" as *const u8 as *const i8).is_null()
    {
        flags |= 1i32 << 1i32
    }
    flags |= 1i32 << 2i32;
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"CapHeight\x00" as *const u8 as *const i8),
        pdf_new_number(capheight),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"Ascent\x00" as *const u8 as *const i8),
        pdf_new_number(ascent),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"Descent\x00" as *const u8 as *const i8),
        pdf_new_number(descent),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"ItalicAngle\x00" as *const u8 as *const i8),
        pdf_new_number(italicangle),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"StemV\x00" as *const u8 as *const i8),
        pdf_new_number(stemv),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"Flags\x00" as *const u8 as *const i8),
        pdf_new_number(flags as f64),
    );
}
unsafe extern "C" fn add_metrics(
    mut font: *mut CIDFont,
    mut cffont: *mut cff_font,
    mut CIDToGIDMap: *mut u8,
    mut widths: *mut f64,
    mut default_width: f64,
    mut last_cid: CID,
) {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut val: f64 = 0.;
    let mut cid: card16 = 0;
    let mut gid: card16 = 0;
    let mut used_chars: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut parent_id: i32 = 0;
    /*
     * The original FontBBox of the font is preserved, instead
     * of replacing it with tight bounding box calculated from
     * charstrings, to prevent Acrobat 4 from greeking text as
     * much as possible.
     */
    if cff_dict_known((*cffont).topdict, b"FontBBox\x00" as *const u8 as *const i8) == 0 {
        panic!("No FontBBox?");
    }
    tmp = pdf_new_array();
    i = 0i32;
    while i < 4i32 {
        val = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\x00" as *const u8 as *const i8,
            i,
        );
        pdf_add_array(
            tmp,
            pdf_new_number((val / 1.0f64 + 0.5f64).floor() * 1.0f64),
        );
        i += 1
    }
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontBBox\x00" as *const u8 as *const i8),
        tmp,
    );
    parent_id = CIDFont_get_parent_id(font, 0i32);
    if parent_id < 0i32 && {
        parent_id = CIDFont_get_parent_id(font, 1i32);
        parent_id < 0i32
    } {
        panic!("No parent Type 0 font !");
    }
    used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
    if used_chars.is_null() {
        panic!("Unexpected error: Font not actually used???");
    }
    /* FIXME:
     * This writes "CID CID width".
     * I think it's better to handle each 8 char block
     * and to use "CID_start [ w0 w1 ...]".
     */
    tmp = pdf_new_array();
    cid = 0i32 as card16;
    while cid as i32 <= last_cid as i32 {
        if *used_chars.offset((cid as i32 / 8i32) as isize) as i32
            & 1i32 << 7i32 - cid as i32 % 8i32
            != 0
        {
            gid = ((*CIDToGIDMap.offset((2i32 * cid as i32) as isize) as i32) << 8i32
                | *CIDToGIDMap.offset((2i32 * cid as i32 + 1i32) as isize) as i32)
                as card16;
            if *widths.offset(gid as isize) != default_width {
                pdf_add_array(tmp, pdf_new_number(cid as f64));
                pdf_add_array(tmp, pdf_new_number(cid as f64));
                pdf_add_array(
                    tmp,
                    pdf_new_number(
                        (*widths.offset(gid as isize) / 1.0f64 + 0.5f64).floor() * 1.0f64,
                    ),
                );
            }
        }
        cid = cid.wrapping_add(1)
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"DW\x00" as *const u8 as *const i8),
        pdf_new_number(default_width),
    );
    if pdf_array_length(tmp) > 0_u32 {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"W\x00" as *const u8 as *const i8),
            pdf_ref_obj(tmp),
        );
    }
    pdf_release_obj(tmp);
}
/* Type1 --> CFF CIDFont */
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_t1dofont(mut font: *mut CIDFont) {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut defaultwidth: f64 = 0.;
    let mut nominalwidth: f64 = 0.;
    let mut num_glyphs: i32 = 0i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut i: i32 = 0;
    let mut offset: i32 = 0;
    let mut used_chars: *mut i8 = 0 as *mut i8;
    let mut last_cid: card16 = 0;
    let mut gid: card16 = 0;
    let mut cid: card16 = 0;
    let mut CIDToGIDMap: *mut u8 = 0 as *mut u8;
    assert!(!font.is_null());
    if (*font).indirect.is_null() {
        return;
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
        pdf_ref_obj((*font).descriptor),
    );
    handle = dpx_open_type1_file((*font).ident);
    if handle.is_null() {
        panic!("Type1: Could not open Type1 font.");
    }
    cffont = t1_load_font(0 as *mut *mut i8, 0i32, handle);
    if cffont.is_null() {
        panic!("Could not read Type 1 font...");
    }
    ttstub_input_close(handle);
    if (*font).fontname.is_null() {
        panic!("Fontname undefined...");
    }
    let mut hparent: *mut Type0Font = 0 as *mut Type0Font;
    let mut vparent: *mut Type0Font = 0 as *mut Type0Font;
    let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut vparent_id: i32 = 0;
    let mut hparent_id: i32 = 0;
    hparent_id = CIDFont_get_parent_id(font, 0i32);
    vparent_id = CIDFont_get_parent_id(font, 1i32);
    if hparent_id < 0i32 && vparent_id < 0i32 {
        panic!("No parent Type 0 font !");
    }
    /* usedchars is same for h and v */
    if hparent_id < 0i32 {
        hparent = 0 as *mut Type0Font
    } else {
        hparent = Type0Font_cache_get(hparent_id);
        used_chars = Type0Font_get_usedchars(hparent)
    }
    if vparent_id < 0i32 {
        vparent = 0 as *mut Type0Font
    } else {
        vparent = Type0Font_cache_get(vparent_id);
        used_chars = Type0Font_get_usedchars(vparent)
    }
    if used_chars.is_null() {
        panic!("Unexpected error: Font not actually used???");
    }
    tounicode = create_ToUnicode_stream(cffont, (*font).fontname, used_chars);
    if !hparent.is_null() {
        Type0Font_set_ToUnicode(hparent, pdf_ref_obj(tounicode));
    }
    if !vparent.is_null() {
        Type0Font_set_ToUnicode(vparent, pdf_ref_obj(tounicode));
    }
    pdf_release_obj(tounicode);
    cff_set_name(cffont, (*font).fontname);
    /* defaultWidthX, CapHeight, etc. */
    get_font_attr(font, cffont);
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"defaultWidthX\x00" as *const u8 as *const i8,
    ) != 0
    {
        defaultwidth = cff_dict_get(
            *(*cffont).private.offset(0),
            b"defaultWidthX\x00" as *const u8 as *const i8,
            0i32,
        )
    } else {
        defaultwidth = 0.0f64
    }
    if cff_dict_known(
        *(*cffont).private.offset(0),
        b"nominalWidthX\x00" as *const u8 as *const i8,
    ) != 0
    {
        nominalwidth = cff_dict_get(
            *(*cffont).private.offset(0),
            b"nominalWidthX\x00" as *const u8 as *const i8,
            0i32,
        )
    } else {
        nominalwidth = 0.0f64
    }
    num_glyphs = 0i32;
    last_cid = 0i32 as card16;
    let ref mut fresh6 = *used_chars.offset((0i32 / 8i32) as isize);
    *fresh6 = (*fresh6 as i32 | 1i32 << 7i32 - 0i32 % 8i32) as i8;
    /* .notdef */
    i = 0i32; /* FIXME: Skip XXXXXX+ */
    while i < ((*cffont).num_glyphs as i32 + 7i32) / 8i32 {
        let mut c: i32 = 0;
        let mut j: i32 = 0;
        c = *used_chars.offset(i as isize) as i32;
        j = 7i32;
        while j >= 0i32 {
            if c & 1i32 << j != 0 {
                num_glyphs += 1;
                last_cid = ((i + 1i32) * 8i32 - j - 1i32) as card16
            }
            j -= 1
        }
        i += 1
    }
    let mut fdselect: *mut cff_fdselect = 0 as *mut cff_fdselect;
    fdselect = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_fdselect>() as u64) as u32)
        as *mut cff_fdselect;
    (*fdselect).format = 3i32 as card8;
    (*fdselect).num_entries = 1i32 as card16;
    (*fdselect).data.ranges =
        new((1_u64).wrapping_mul(::std::mem::size_of::<cff_range3>() as u64) as u32)
            as *mut cff_range3;
    (*(*fdselect).data.ranges.offset(0)).first = 0i32 as card16;
    (*(*fdselect).data.ranges.offset(0)).fd = 0i32 as card8;
    (*cffont).fdselect = fdselect;
    CIDToGIDMap = new(((2i32 * (last_cid as i32 + 1i32)) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    memset(
        CIDToGIDMap as *mut libc::c_void,
        0i32,
        (2i32 * (last_cid as i32 + 1i32)) as u64,
    );
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    charset = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_charsets>() as u64) as u32)
        as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = (num_glyphs - 1i32) as card16;
    (*charset).data.glyphs = new(((num_glyphs - 1i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<s_SID>() as u64)
        as u32) as *mut s_SID;
    gid = 0i32 as card16;
    cid = 0i32 as card16;
    while cid as i32 <= last_cid as i32 {
        if *used_chars.offset((cid as i32 / 8i32) as isize) as i32
            & 1i32 << 7i32 - cid as i32 % 8i32
            != 0
        {
            if gid as i32 > 0i32 {
                *(*charset).data.glyphs.offset((gid as i32 - 1i32) as isize) = cid
            }
            *CIDToGIDMap.offset((2i32 * cid as i32) as isize) =
                (gid as i32 >> 8i32 & 0xffi32) as u8;
            *CIDToGIDMap.offset((2i32 * cid as i32 + 1i32) as isize) = (gid as i32 & 0xffi32) as u8;
            gid = gid.wrapping_add(1)
        }
        cid = cid.wrapping_add(1)
    }
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    cff_dict_add(
        (*cffont).topdict,
        b"CIDCount\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CIDCount\x00" as *const u8 as *const i8,
        0i32,
        (last_cid as i32 + 1i32) as f64,
    );
    (*cffont).fdarray =
        new((1_u64).wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64) as u32)
            as *mut *mut cff_dict;
    let ref mut fresh7 = *(*cffont).fdarray.offset(0);
    *fresh7 = cff_new_dict();
    cff_dict_add(
        *(*cffont).fdarray.offset(0),
        b"FontName\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0),
        b"FontName\x00" as *const u8 as *const i8,
        0i32,
        cff_add_string(cffont, (*font).fontname.offset(7), 1i32) as f64,
    );
    cff_dict_add(
        *(*cffont).fdarray.offset(0),
        b"Private\x00" as *const u8 as *const i8,
        2i32,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0),
        b"Private\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0),
        b"Private\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    /* FDArray  - index offset, not known yet */
    cff_dict_add(
        (*cffont).topdict,
        b"FDArray\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDArray\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    /* FDSelect - offset, not known yet */
    cff_dict_add(
        (*cffont).topdict,
        b"FDSelect\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDSelect\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"charset\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"charset\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        1i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        0i32,
        0.0f64,
    );
    let mut cstring: *mut cff_index = 0 as *mut cff_index;
    let mut gm = t1_ginfo::new();
    let mut max: i32 = 0i32;
    let mut widths: *mut f64 = 0 as *mut f64;
    let mut w_stat: [i32; 1001] = [0; 1001];
    let mut max_count: i32 = 0;
    let mut dw: i32 = 0;
    widths =
        new((num_glyphs as u32 as u64).wrapping_mul(::std::mem::size_of::<f64>() as u64) as u32)
            as *mut f64;
    memset(
        w_stat.as_mut_ptr() as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<i32>() as u64).wrapping_mul(1001i32 as u64),
    );
    offset = 0i64 as i32;
    cstring = cff_new_index(num_glyphs as card16);
    (*cstring).data = 0 as *mut card8;
    *(*cstring).offset.offset(0) = 1i32 as l_offset;
    gid = 0i32 as card16;
    cid = 0i32 as card16;
    while cid as i32 <= last_cid as i32 {
        if !(*used_chars.offset((cid as i32 / 8i32) as isize) as i32
            & 1i32 << 7i32 - cid as i32 % 8i32
            == 0)
        {
            if offset + 65536i32 >= max {
                max += 65536i32 * 2i32;
                (*cstring).data = renew(
                    (*cstring).data as *mut libc::c_void,
                    (max as u32 as u64).wrapping_mul(::std::mem::size_of::<card8>() as u64) as u32,
                ) as *mut card8
            }
            offset += t1char_convert_charstring(
                (*cstring)
                    .data
                    .offset(*(*cstring).offset.offset(gid as isize) as isize)
                    .offset(-1),
                65536i32,
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(cid as isize) as isize)
                    .offset(-1),
                (*(*(*cffont).cstrings)
                    .offset
                    .offset((cid as i32 + 1i32) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(cid as isize))
                    as i32,
                *(*cffont).subrs.offset(0),
                defaultwidth,
                nominalwidth,
                &mut gm,
            );
            *(*cstring).offset.offset((gid as i32 + 1i32) as isize) = (offset + 1i32) as l_offset;
            if gm.use_seac != 0 {
                panic!("This font using the \"seac\" command for accented characters...");
            }
            *widths.offset(gid as isize) = gm.wx;
            if gm.wx >= 0.0f64 && gm.wx <= 1000.0f64 {
                w_stat[gm.wx as i32 as usize] += 1i32
            }
            gid = gid.wrapping_add(1)
        }
        cid = cid.wrapping_add(1)
    }
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = cstring;
    max_count = 0i32;
    dw = -1i32;
    i = 0i32;
    while i <= 1000i32 {
        if w_stat[i as usize] > max_count {
            dw = i;
            max_count = w_stat[i as usize]
        }
        i += 1
    }
    if dw >= 0i32 {
        add_metrics(font, cffont, CIDToGIDMap, widths, dw as f64, last_cid);
    } else {
        add_metrics(font, cffont, CIDToGIDMap, widths, defaultwidth, last_cid);
    }
    free(widths as *mut libc::c_void);
    cff_release_index(*(*cffont).subrs.offset(0));
    let ref mut fresh8 = *(*cffont).subrs.offset(0);
    *fresh8 = 0 as *mut cff_index;
    free(CIDToGIDMap as *mut libc::c_void);
    cff_add_string(cffont, b"Adobe\x00" as *const u8 as *const i8, 1i32);
    cff_add_string(cffont, b"Identity\x00" as *const u8 as *const i8, 1i32);
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(*(*cffont).private.offset(0), cffont);
    cff_update_string(cffont);
    /* CFF code need to be rewrote... */
    cff_dict_add(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        3i32,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        0i32,
        cff_get_sid(cffont, b"Adobe\x00" as *const u8 as *const i8) as f64,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        1i32,
        cff_get_sid(cffont, b"Identity\x00" as *const u8 as *const i8) as f64,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\x00" as *const u8 as *const i8,
        2i32,
        0.0f64,
    );
    (*cffont).num_glyphs = num_glyphs as card16;
    offset = write_fontfile(font, cffont);
    cff_close(cffont);
    CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
