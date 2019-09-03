#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types)]
extern crate libc;
extern "C" {
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    /* Here is the complete list of PDF object types */
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut always_embed: libc::c_int;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t,
                         whence: libc::c_int) -> size_t;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t)
     -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    /* pdf_add_dict requires key but pdf_add_array does not.
 * pdf_add_array always append elements to array.
 * They should be pdf_put_array(array, idx, element) and
 * pdf_put_dict(dict, key, value)
 */
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
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
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj,
                    value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table;
    #[no_mangle]
    fn tt_release_post_table(post: *mut tt_post_table);
    #[no_mangle]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    /* OS/2 table */
    #[no_mangle]
    fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table;
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type BYTE = libc::c_uchar;
pub type SFNT_CHAR = libc::c_schar;
pub type USHORT = libc::c_ushort;
pub type SHORT = libc::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
pub type FWord = libc::c_short;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [libc::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut libc::c_char,
    pub tables: *mut sfnt_table,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const libc::c_char,
    pub names: *mut *mut libc::c_char,
    pub count: USHORT,
    /* Number of glyph names in names[] */
}
#[derive ( Copy , Clone )]
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
#[derive ( Copy , Clone )]
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
static mut verbose: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn tt_aux_set_verbose(mut level: libc::c_int) {
    verbose = level; /* skip version tag */
}
#[no_mangle]
pub unsafe extern "C" fn ttc_read_offset(mut sfont: *mut sfnt,
                                         mut ttc_idx: libc::c_int)
 -> SFNT_ULONG {
    let mut offset: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut num_dirs: SFNT_ULONG = 0i32 as SFNT_ULONG;
    if sfont.is_null() || (*sfont).handle.is_null() {
        _tt_abort(b"file not opened\x00" as *const u8 as *const libc::c_char);
    }
    if (*sfont).type_0 != 1i32 << 4i32 {
        _tt_abort(b"ttc_read_offset(): invalid font type\x00" as *const u8 as
                      *const libc::c_char);
    }
    ttstub_input_seek((*sfont).handle, 4i32 as ssize_t, 0i32);
    /* version = */
    tt_get_unsigned_quad((*sfont).handle);
    num_dirs = tt_get_unsigned_quad((*sfont).handle);
    if ttc_idx < 0i32 ||
           ttc_idx as libc::c_uint >
               num_dirs.wrapping_sub(1i32 as libc::c_uint) {
        _tt_abort(b"Invalid TTC index number\x00" as *const u8 as
                      *const libc::c_char);
    }
    ttstub_input_seek((*sfont).handle, (12i32 + ttc_idx * 4i32) as ssize_t,
                      0i32);
    offset = tt_get_unsigned_quad((*sfont).handle);
    return offset;
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
/* flag declared in dvipdfmx.c */
/* TTC (TrueType Collection) */
/* FontDescriptor */
/* Force bold at small text sizes */
#[no_mangle]
pub unsafe extern "C" fn tt_get_fontdesc(mut sfont: *mut sfnt,
                                         mut embed: *mut libc::c_int,
                                         mut stemv: libc::c_int,
                                         mut type_0: libc::c_int,
                                         mut fontname: *const libc::c_char)
 -> *mut pdf_obj {
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut bbox: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut flag: libc::c_int = 1i32 << 2i32;
    /* TrueType tables */
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    if sfont.is_null() {
        _tt_abort(b"font file not opened\x00" as *const u8 as
                      *const libc::c_char);
    }
    os2 = tt_read_os2__table(sfont);
    head = tt_read_head_table(sfont);
    post = tt_read_post_table(sfont);
    if post.is_null() {
        free(os2 as *mut libc::c_void);
        free(head as *mut libc::c_void);
        return 0 as *mut pdf_obj
    }
    descriptor = pdf_new_dict();
    pdf_add_dict(descriptor,
                 pdf_new_name(b"Type\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"FontDescriptor\x00" as *const u8 as
                                  *const libc::c_char));
    if *embed != 0 && !os2.is_null() {
        /*
      License:

       "Preview & Print embedding" (0x004) requires the document containing
       Preview & Print font to be opened in read-only mode. However, licensing
       information are lost when fonts are embedded in PDF document and
       the only way to make the PDF document "read-only" is to encrypt it.
       But we have no support for encryption yet. We do not embed any fonts
       with "Preview & Print embedding" setting.

       2001/11/22: Changed to allow `Preview & Print' only fonts embedding

       2006/04/19: Added support for always_embed option
    */
        if (*os2).fsType as libc::c_int == 0i32 ||
               (*os2).fsType as libc::c_int & 0x8i32 != 0 {
            /* the least restrictive license granted takes precedence. */
            *embed = 1i32
        } else if (*os2).fsType as libc::c_int & 0x4i32 != 0 {
            if verbose > 0i32 {
                dpx_warning(b"Font \"%s\" permits \"Preview & Print\" embedding only **\n\x00"
                                as *const u8 as *const libc::c_char,
                            fontname);
            }
            *embed = 1i32
        } else if always_embed != 0 {
            if verbose > 0i32 {
                dpx_warning(b"Font \"%s\" may be subject to embedding restrictions **\n\x00"
                                as *const u8 as *const libc::c_char,
                            fontname);
            }
            *embed = 1i32
        } else {
            if verbose > 0i32 {
                dpx_warning(b"Embedding of font \"%s\" disabled due to license restrictions\x00"
                                as *const u8 as *const libc::c_char,
                            fontname);
            }
            *embed = 0i32
        }
    }
    if !os2.is_null() {
        pdf_add_dict(descriptor,
                     pdf_new_name(b"Ascent\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(floor(1000.0f64 *
                                              (*os2).sTypoAscender as
                                                  libc::c_int as
                                                  libc::c_double /
                                              (*head).unitsPerEm as
                                                  libc::c_int as
                                                  libc::c_double /
                                              1i32 as libc::c_double + 0.5f64)
                                        * 1i32 as libc::c_double));
        pdf_add_dict(descriptor,
                     pdf_new_name(b"Descent\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(floor(1000.0f64 *
                                              (*os2).sTypoDescender as
                                                  libc::c_int as
                                                  libc::c_double /
                                              (*head).unitsPerEm as
                                                  libc::c_int as
                                                  libc::c_double /
                                              1i32 as libc::c_double + 0.5f64)
                                        * 1i32 as libc::c_double));
        if stemv < 0i32 {
            /* if not given by the option '-v' */
            stemv =
                ((*os2).usWeightClass as libc::c_int as libc::c_double /
                     65.0f64 *
                     ((*os2).usWeightClass as libc::c_int as libc::c_double /
                          65.0f64) + 50i32 as libc::c_double) as libc::c_int
        } /* arbitrary */
        pdf_add_dict(descriptor,
                     pdf_new_name(b"StemV\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(stemv as libc::c_double));
        if (*os2).version as libc::c_int == 0x2i32 {
            pdf_add_dict(descriptor,
                         pdf_new_name(b"CapHeight\x00" as *const u8 as
                                          *const libc::c_char),
                         pdf_new_number(floor(1000.0f64 *
                                                  (*os2).sCapHeight as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  (*head).unitsPerEm as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  1i32 as libc::c_double +
                                                  0.5f64) *
                                            1i32 as libc::c_double));
            /* optional */
            pdf_add_dict(descriptor,
                         pdf_new_name(b"XHeight\x00" as *const u8 as
                                          *const libc::c_char),
                         pdf_new_number(floor(1000.0f64 *
                                                  (*os2).sxHeight as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  (*head).unitsPerEm as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  1i32 as libc::c_double +
                                                  0.5f64) *
                                            1i32 as libc::c_double));
        } else {
            pdf_add_dict(descriptor,
                         pdf_new_name(b"CapHeight\x00" as *const u8 as
                                          *const libc::c_char),
                         pdf_new_number(floor(1000.0f64 *
                                                  (*os2).sTypoAscender as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  (*head).unitsPerEm as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  1i32 as libc::c_double +
                                                  0.5f64) *
                                            1i32 as libc::c_double));
        }
        /* optional */
        if (*os2).xAvgCharWidth as libc::c_int != 0i32 {
            pdf_add_dict(descriptor,
                         pdf_new_name(b"AvgWidth\x00" as *const u8 as
                                          *const libc::c_char),
                         pdf_new_number(floor(1000.0f64 *
                                                  (*os2).xAvgCharWidth as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  (*head).unitsPerEm as
                                                      libc::c_int as
                                                      libc::c_double /
                                                  1i32 as libc::c_double +
                                                  0.5f64) *
                                            1i32 as libc::c_double));
        }
    }
    /* BoundingBox (array) */
    bbox = pdf_new_array();
    pdf_add_array(bbox,
                  pdf_new_number(floor(1000.0f64 *
                                           (*head).xMin as libc::c_int as
                                               libc::c_double /
                                           (*head).unitsPerEm as libc::c_int
                                               as libc::c_double /
                                           1i32 as libc::c_double + 0.5f64) *
                                     1i32 as libc::c_double));
    pdf_add_array(bbox,
                  pdf_new_number(floor(1000.0f64 *
                                           (*head).yMin as libc::c_int as
                                               libc::c_double /
                                           (*head).unitsPerEm as libc::c_int
                                               as libc::c_double /
                                           1i32 as libc::c_double + 0.5f64) *
                                     1i32 as libc::c_double));
    pdf_add_array(bbox,
                  pdf_new_number(floor(1000.0f64 *
                                           (*head).xMax as libc::c_int as
                                               libc::c_double /
                                           (*head).unitsPerEm as libc::c_int
                                               as libc::c_double /
                                           1i32 as libc::c_double + 0.5f64) *
                                     1i32 as libc::c_double));
    pdf_add_array(bbox,
                  pdf_new_number(floor(1000.0f64 *
                                           (*head).yMax as libc::c_int as
                                               libc::c_double /
                                           (*head).unitsPerEm as libc::c_int
                                               as libc::c_double /
                                           1i32 as libc::c_double + 0.5f64) *
                                     1i32 as libc::c_double));
    pdf_add_dict(descriptor,
                 pdf_new_name(b"FontBBox\x00" as *const u8 as
                                  *const libc::c_char), bbox);
    /* post */
    pdf_add_dict(descriptor,
                 pdf_new_name(b"ItalicAngle\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(((*post).italicAngle as libc::c_long %
                                     0x10000i64) as libc::c_double /
                                    0x10000i64 as libc::c_double +
                                    ((*post).italicAngle as libc::c_long /
                                         0x10000i64) as libc::c_double -
                                    (if (*post).italicAngle as libc::c_long /
                                            0x10000i64 > 0x7fffi64 {
                                         0x10000i64
                                     } else { 0i32 as libc::c_long }) as
                                        libc::c_double));
    /* Flags */
    if !os2.is_null() {
        if (*os2).fsSelection as libc::c_int & 1i32 << 0i32 != 0 {
            flag |= 1i32 << 6i32
        }
        if (*os2).fsSelection as libc::c_int & 1i32 << 5i32 != 0 {
            flag |= 1i32 << 18i32
        }
        if (*os2).sFamilyClass as libc::c_int >> 8i32 & 0xffi32 != 8i32 {
            flag |= 1i32 << 1i32
        }
        if (*os2).sFamilyClass as libc::c_int >> 8i32 & 0xffi32 == 10i32 {
            flag |= 1i32 << 3i32
        }
        if (*post).isFixedPitch != 0 { flag |= 1i32 << 0i32 }
    }
    pdf_add_dict(descriptor,
                 pdf_new_name(b"Flags\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(flag as libc::c_double));
    /* insert panose if you want */
    if type_0 == 0i32 && !os2.is_null() {
        /* cid-keyed font - add panose */
        let mut styledict: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut panose: [libc::c_uchar; 12] = [0; 12];
        panose[0] =
            ((*os2).sFamilyClass as libc::c_int >> 8i32) as libc::c_uchar;
        panose[1] =
            ((*os2).sFamilyClass as libc::c_int & 0xffi32) as libc::c_uchar;
        memcpy(panose.as_mut_ptr().offset(2) as *mut libc::c_void,
               (*os2).panose.as_mut_ptr() as *const libc::c_void,
               10i32 as libc::c_ulong);
        styledict = pdf_new_dict();
        pdf_add_dict(styledict,
                     pdf_new_name(b"Panose\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_string(panose.as_mut_ptr() as
                                        *const libc::c_void,
                                    12i32 as size_t));
        pdf_add_dict(descriptor,
                     pdf_new_name(b"Style\x00" as *const u8 as
                                      *const libc::c_char), styledict);
    }
    free(head as *mut libc::c_void);
    free(os2 as *mut libc::c_void);
    tt_release_post_table(post);
    return descriptor;
}
