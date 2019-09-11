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
    fn floor(_: f64) -> f64;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    static mut always_embed: i32;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn pdf_new_number(value: f64) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
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
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
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
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
pub type Fixed = u32;
pub type FWord = i16;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: u32,
    pub minMemType42: u32,
    pub maxMemType42: u32,
    pub minMemType1: u32,
    pub maxMemType1: u32,
    pub numberOfGlyphs: u16,
    pub glyphNamePtr: *mut *const i8,
    pub names: *mut *mut i8,
    pub count: u16,
    /* Number of glyph names in names[] */
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
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn tt_aux_set_verbose(mut level: i32) {
    verbose = level; /* skip version tag */
}
#[no_mangle]
pub unsafe extern "C" fn ttc_read_offset(mut sfont: *mut sfnt, mut ttc_idx: i32) -> u32 {
    let mut offset: u32 = 0_u32;
    let mut num_dirs: u32 = 0_u32;
    if sfont.is_null() || (*sfont).handle.is_null() {
        _tt_abort(b"file not opened\x00" as *const u8 as *const i8);
    }
    if (*sfont).type_0 != 1i32 << 4i32 {
        _tt_abort(b"ttc_read_offset(): invalid font type\x00" as *const u8 as *const i8);
    }
    ttstub_input_seek((*sfont).handle, 4i32 as ssize_t, 0i32);
    /* version = */
    tt_get_unsigned_quad((*sfont).handle);
    num_dirs = tt_get_unsigned_quad((*sfont).handle);
    if ttc_idx < 0i32 || ttc_idx as u32 > num_dirs.wrapping_sub(1_u32) {
        _tt_abort(b"Invalid TTC index number\x00" as *const u8 as *const i8);
    }
    ttstub_input_seek((*sfont).handle, (12i32 + ttc_idx * 4i32) as ssize_t, 0i32);
    offset = tt_get_unsigned_quad((*sfont).handle);
    offset
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
pub unsafe extern "C" fn tt_get_fontdesc(
    mut sfont: *mut sfnt,
    mut embed: *mut i32,
    mut stemv: i32,
    mut type_0: i32,
    mut fontname: *const i8,
) -> *mut pdf_obj {
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut bbox: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut flag: i32 = 1i32 << 2i32;
    /* TrueType tables */
    let mut head: *mut tt_head_table = 0 as *mut tt_head_table;
    let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
    let mut post: *mut tt_post_table = 0 as *mut tt_post_table;
    if sfont.is_null() {
        _tt_abort(b"font file not opened\x00" as *const u8 as *const i8);
    }
    os2 = tt_read_os2__table(sfont);
    head = tt_read_head_table(sfont);
    post = tt_read_post_table(sfont);
    if post.is_null() {
        free(os2 as *mut libc::c_void);
        free(head as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    descriptor = pdf_new_dict();
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
    );
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
        if (*os2).fsType as i32 == 0i32 || (*os2).fsType as i32 & 0x8i32 != 0 {
            /* the least restrictive license granted takes precedence. */
            *embed = 1i32
        } else if (*os2).fsType as i32 & 0x4i32 != 0 {
            if verbose > 0i32 {
                dpx_warning(
                    b"Font \"%s\" permits \"Preview & Print\" embedding only **\n\x00" as *const u8
                        as *const i8,
                    fontname,
                );
            }
            *embed = 1i32
        } else if always_embed != 0 {
            if verbose > 0i32 {
                dpx_warning(
                    b"Font \"%s\" may be subject to embedding restrictions **\n\x00" as *const u8
                        as *const i8,
                    fontname,
                );
            }
            *embed = 1i32
        } else {
            if verbose > 0i32 {
                dpx_warning(
                    b"Embedding of font \"%s\" disabled due to license restrictions\x00"
                        as *const u8 as *const i8,
                    fontname,
                );
            }
            *embed = 0i32
        }
    }
    if !os2.is_null() {
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"Ascent\x00" as *const u8 as *const i8),
            pdf_new_number(
                floor(
                    1000.0f64 * (*os2).sTypoAscender as i32 as f64
                        / (*head).unitsPerEm as i32 as f64
                        / 1i32 as f64
                        + 0.5f64,
                ) * 1i32 as f64,
            ),
        );
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"Descent\x00" as *const u8 as *const i8),
            pdf_new_number(
                floor(
                    1000.0f64 * (*os2).sTypoDescender as i32 as f64
                        / (*head).unitsPerEm as i32 as f64
                        / 1i32 as f64
                        + 0.5f64,
                ) * 1i32 as f64,
            ),
        );
        if stemv < 0i32 {
            /* if not given by the option '-v' */
            stemv = ((*os2).usWeightClass as i32 as f64 / 65.0f64
                * ((*os2).usWeightClass as i32 as f64 / 65.0f64)
                + 50i32 as f64) as i32
        } /* arbitrary */
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"StemV\x00" as *const u8 as *const i8),
            pdf_new_number(stemv as f64),
        );
        if (*os2).version as i32 == 0x2i32 {
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"CapHeight\x00" as *const u8 as *const i8),
                pdf_new_number(
                    floor(
                        1000.0f64 * (*os2).sCapHeight as i32 as f64
                            / (*head).unitsPerEm as i32 as f64
                            / 1i32 as f64
                            + 0.5f64,
                    ) * 1i32 as f64,
                ),
            );
            /* optional */
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"XHeight\x00" as *const u8 as *const i8),
                pdf_new_number(
                    floor(
                        1000.0f64 * (*os2).sxHeight as i32 as f64
                            / (*head).unitsPerEm as i32 as f64
                            / 1i32 as f64
                            + 0.5f64,
                    ) * 1i32 as f64,
                ),
            );
        } else {
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"CapHeight\x00" as *const u8 as *const i8),
                pdf_new_number(
                    floor(
                        1000.0f64 * (*os2).sTypoAscender as i32 as f64
                            / (*head).unitsPerEm as i32 as f64
                            / 1i32 as f64
                            + 0.5f64,
                    ) * 1i32 as f64,
                ),
            );
        }
        /* optional */
        if (*os2).xAvgCharWidth as i32 != 0i32 {
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"AvgWidth\x00" as *const u8 as *const i8),
                pdf_new_number(
                    floor(
                        1000.0f64 * (*os2).xAvgCharWidth as i32 as f64
                            / (*head).unitsPerEm as i32 as f64
                            / 1i32 as f64
                            + 0.5f64,
                    ) * 1i32 as f64,
                ),
            );
        }
    }
    /* BoundingBox (array) */
    bbox = pdf_new_array();
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).xMin as i32 as f64
                    / (*head).unitsPerEm as i32 as f64
                    / 1i32 as f64
                    + 0.5f64,
            ) * 1i32 as f64,
        ),
    );
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).yMin as i32 as f64
                    / (*head).unitsPerEm as i32 as f64
                    / 1i32 as f64
                    + 0.5f64,
            ) * 1i32 as f64,
        ),
    );
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).xMax as i32 as f64
                    / (*head).unitsPerEm as i32 as f64
                    / 1i32 as f64
                    + 0.5f64,
            ) * 1i32 as f64,
        ),
    );
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).yMax as i32 as f64
                    / (*head).unitsPerEm as i32 as f64
                    / 1i32 as f64
                    + 0.5f64,
            ) * 1i32 as f64,
        ),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontBBox\x00" as *const u8 as *const i8),
        bbox,
    );
    /* post */
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"ItalicAngle\x00" as *const u8 as *const i8),
        pdf_new_number(
            ((*post).italicAngle as i64 % 0x10000) as f64 / 0x10000i64 as f64
                + ((*post).italicAngle as i64 / 0x10000) as f64
                - (if (*post).italicAngle as i64 / 0x10000 > 0x7fff {
                    0x10000
                } else {
                    0i32 as i64
                }) as f64,
        ),
    );
    /* Flags */
    if !os2.is_null() {
        if (*os2).fsSelection as i32 & 1i32 << 0i32 != 0 {
            flag |= 1i32 << 6i32
        }
        if (*os2).fsSelection as i32 & 1i32 << 5i32 != 0 {
            flag |= 1i32 << 18i32
        }
        if (*os2).sFamilyClass as i32 >> 8i32 & 0xffi32 != 8i32 {
            flag |= 1i32 << 1i32
        }
        if (*os2).sFamilyClass as i32 >> 8i32 & 0xffi32 == 10i32 {
            flag |= 1i32 << 3i32
        }
        if (*post).isFixedPitch != 0 {
            flag |= 1i32 << 0i32
        }
    }
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Flags\x00" as *const u8 as *const i8),
        pdf_new_number(flag as f64),
    );
    /* insert panose if you want */
    if type_0 == 0i32 && !os2.is_null() {
        /* cid-keyed font - add panose */
        let mut styledict: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut panose: [u8; 12] = [0; 12];
        panose[0] = ((*os2).sFamilyClass as i32 >> 8i32) as u8;
        panose[1] = ((*os2).sFamilyClass as i32 & 0xffi32) as u8;
        memcpy(
            panose.as_mut_ptr().offset(2) as *mut libc::c_void,
            (*os2).panose.as_mut_ptr() as *const libc::c_void,
            10i32 as u64,
        );
        styledict = pdf_new_dict();
        pdf_add_dict(
            styledict,
            pdf_new_name(b"Panose\x00" as *const u8 as *const i8),
            pdf_new_string(panose.as_mut_ptr() as *const libc::c_void, 12i32 as size_t),
        );
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"Style\x00" as *const u8 as *const i8),
            styledict,
        );
    }
    free(head as *mut libc::c_void);
    free(os2 as *mut libc::c_void);
    tt_release_post_table(post);
    descriptor
}
