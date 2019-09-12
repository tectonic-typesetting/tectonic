#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::{info, warn};
use crate::stub_errno as errno;
extern crate libc;
use crate::dpx_pdfobj::{
    pdf_add_dict, pdf_link_obj, pdf_lookup_dict, pdf_new_dict, pdf_new_name, pdf_obj,
    pdf_obj_typeof, pdf_ref_obj, pdf_release_obj, pdf_stream_length,
};
use libc::free;
extern "C" {
    pub type Type0Font;
    #[no_mangle]
    #[no_mangle]
    fn strtoll(_: *const i8, _: *mut *mut i8, _: i32) -> libc::c_longlong;
    #[no_mangle]
    fn rand() -> i32;
    #[no_mangle]
    fn srand(__seed: u32);
    #[no_mangle]
    fn getenv(__name: *const i8) -> *mut i8;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn pdf_lookup_fontmap_record(kp: *const i8) -> *mut fontmap_rec;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn agl_set_verbose(level: i32);
    #[no_mangle]
    fn agl_init_map();
    #[no_mangle]
    fn agl_close_map();
    #[no_mangle]
    fn CIDFont_set_verbose(level: i32);
    /* ******************************* CACHE ********************************/
    #[no_mangle]
    fn Type0Font_cache_find(map_name: *const i8, cmap_id: i32, fmap_opt: *mut fontmap_opt) -> i32;
    #[no_mangle]
    fn Type0Font_set_verbose(level: i32);
    #[no_mangle]
    fn Type0Font_get_wmode(font: *mut Type0Font) -> i32;
    #[no_mangle]
    fn Type0Font_get_usedchars(font: *mut Type0Font) -> *mut i8;
    #[no_mangle]
    fn Type0Font_get_resource(font: *mut Type0Font) -> *mut pdf_obj;
    #[no_mangle]
    fn Type0Font_cache_init();
    #[no_mangle]
    fn Type0Font_cache_get(id: i32) -> *mut Type0Font;
    #[no_mangle]
    fn Type0Font_cache_close();
    /* Type1 --> CFF CIDFont */
    #[no_mangle]
    fn t1_load_UnicodeCMap(font_name: *const i8, otl_tags: *const i8, wmode: i32) -> i32;
    /* ************************* CMAP_MAIN **************************/
    #[no_mangle]
    fn CMap_set_verbose(level: i32);
    #[no_mangle]
    fn CMap_get_profile(cmap: *mut CMap, type_0: i32) -> i32;
    #[no_mangle]
    fn CMap_get_name(cmap: *mut CMap) -> *mut i8;
    #[no_mangle]
    fn CMap_get_type(cmap: *mut CMap) -> i32;
    #[no_mangle]
    fn CMap_cache_init();
    #[no_mangle]
    fn CMap_cache_get(id: i32) -> *mut CMap;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const i8) -> i32;
    #[no_mangle]
    fn CMap_cache_close();
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
    #[no_mangle]
    fn pdf_encoding_set_verbose(level: i32);
    #[no_mangle]
    fn pdf_init_encodings();
    #[no_mangle]
    fn pdf_close_encodings();
    /* Creates Encoding resource and ToUnicode CMap
     * for all non-predefined encodings.
     */
    #[no_mangle]
    fn pdf_encoding_complete();
    /* enc_name here is .enc file name or the name of predefined
     * encodings.
     */
    #[no_mangle]
    fn pdf_encoding_findresource(enc_name: *const i8) -> i32;
    /* Returns the Encoding resource object.
     */
    #[no_mangle]
    fn pdf_get_encoding_obj(enc_id: i32) -> *mut pdf_obj;
    /* WARNING:
     * Pointer(s) may change after another encoding is loaded.
     */
    #[no_mangle]
    fn pdf_encoding_get_name(enc_id: i32) -> *mut i8;
    /* pdf_encoding_copy_usedchars adds the given vector of used characters
     * to the corresponding vector of the encoding.
     */
    #[no_mangle]
    fn pdf_encoding_add_usedchars(encoding_id: i32, is_used: *const i8);
    #[no_mangle]
    fn pdf_encoding_get_tounicode(encoding_id: i32) -> *mut pdf_obj;
    /* Just load CMap identified with 'ident'. (parsed)
     * PDF stream object (not reference) returned.
     */
    #[no_mangle]
    fn pdf_load_ToUnicode_stream(ident: *const i8) -> *mut pdf_obj;
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
    fn pdf_font_open_pkfont(font: *mut pdf_font) -> i32;
    #[no_mangle]
    fn pdf_font_load_pkfont(font: *mut pdf_font) -> i32;
    #[no_mangle]
    fn PKFont_set_dpi(dpi: i32);
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
    fn pdf_font_open_truetype(font: *mut pdf_font) -> i32;
    #[no_mangle]
    fn pdf_font_load_truetype(font: *mut pdf_font) -> i32;
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
    #[no_mangle]
    fn otf_cmap_set_verbose(level: i32);
    /* CMap ID */
    #[no_mangle]
    fn otf_load_Unicode_CMap(
        map_name: *const i8,
        ttc_index: i32,
        otl_opts: *const i8,
        wmode: i32,
    ) -> i32;
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
    fn pdf_font_open_type1(font: *mut pdf_font) -> i32;
    #[no_mangle]
    fn pdf_font_load_type1(font: *mut pdf_font) -> i32;
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
    fn pdf_font_open_type1c(font: *mut pdf_font) -> i32;
    #[no_mangle]
    fn pdf_font_load_type1c(font: *mut pdf_font) -> i32;
}
pub type __time_t = i64;
pub type size_t = u64;
pub type time_t = __time_t;
/* Options */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: f64,
    pub extend: f64,
    pub bold: f64,
    pub mapc: i32,
    pub flags: i32,
    pub otl_tags: *mut i8,
    pub tounicode: *mut i8,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: f64,
    pub charcoll: *mut i8,
    pub index: i32,
    pub style: i32,
    pub stemv: i32,
    /* StemV value especially for CJK fonts */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut i8,
    pub font_name: *mut i8,
    pub enc_name: *mut i8,
    pub charmap: C2RustUnnamed,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sfd_name: *mut i8,
    pub subfont_id: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_font {
    pub ident: *mut i8,
    pub subtype: i32,
    pub map_name: *mut i8,
    pub encoding_id: i32,
    pub font_id: i32,
    pub index: i32,
    pub fontname: *mut i8,
    pub uniqueID: [i8; 7],
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
    pub usedchars: *mut i8,
    pub flags: i32,
    pub point_size: f64,
    pub design_size: f64,
    /* _PDFFONT_H_ */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub count: i32,
    pub capacity: i32,
    pub fonts: *mut pdf_font,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut i8,
    pub type_0: i32,
    pub wmode: i32,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_2,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: i32,
    pub profile: C2RustUnnamed_1,
    pub reverseMap: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: i32,
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
pub struct C2RustUnnamed_2 {
    pub num: u32,
    pub max: u32,
    pub ranges: *mut rangeDef,
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
pub struct CIDSysInfo {
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: i32,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    0 as *mut libc::c_void
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
    false
}
static mut __verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_font_set_verbose(mut level: i32) {
    __verbose = level;
    CMap_set_verbose(level);
    Type0Font_set_verbose(level);
    CIDFont_set_verbose(level);
    pdf_encoding_set_verbose(level);
    agl_set_verbose(level);
    otf_cmap_set_verbose(level);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_verbose() -> i32 {
    __verbose
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_set_dpi(mut font_dpi: i32) {
    PKFont_set_dpi(font_dpi);
}
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
/* External interface to pdf routines */
/* Name does not include the / */
/* pdf_add_dict requires key but pdf_add_array does not.
 * pdf_add_array always append elements to array.
 * They should be pdf_put_array(array, idx, element) and
 * pdf_put_dict(dict, key, value)
 */
/* pdf_add_dict() want pdf_obj as key, however, key must always be name
 * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
 * key. This strange difference seems come from pdfdoc that first allocate
 * name objects frequently used (maybe 1000 times) such as /Type and does
 * pdf_link_obj() it rather than allocate/free-ing them each time. But I
 * already removed that.
 */
/* Apply proc(key, value, pdata) for each key-value pairs in dict, stop if proc()
 * returned non-zero value (and that value is returned). PDF object is passed for
 * key to allow modification (fix) of key.
 */
/* Compare label of two indirect reference object.
 */
/* The following routines are not appropriate for pdfobj.
 */
#[no_mangle]
pub unsafe extern "C" fn get_unique_time_if_given() -> time_t {
    let mut epoch: i64 = 0; /* Type0 ID */
    let mut endptr: *mut i8 = 0 as *mut i8;
    let mut ret: time_t = -1i32 as time_t;
    let mut got_it: i32 = 0;
    let mut source_date_epoch: *const i8 = 0 as *const i8;
    source_date_epoch = getenv(b"SOURCE_DATE_EPOCH\x00" as *const u8 as *const i8);
    got_it = (source_date_epoch != 0 as *mut libc::c_void as *const i8) as i32;
    if got_it != 0 {
        errno::set_errno(errno::ZERO);
        epoch = strtoll(source_date_epoch, &mut endptr, 10i32) as i64;
        if !(*endptr as i32 != '\u{0}' as i32 || errno::errno() != errno::ZERO) {
            ret = epoch
        }
    }
    ret
}
static mut unique_tag_state: i32 = 1i32;
static mut unique_tags_deterministic: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_font_reset_unique_tag_state() {
    unique_tag_state = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_set_deterministic_unique_tags(mut value: i32) {
    unique_tags_deterministic = value;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_make_uniqueTag(mut tag: *mut i8) {
    let mut i: i32 = 0;
    let mut ch: i8 = 0;
    if unique_tags_deterministic != 0 {
        snprintf(
            tag,
            7i32 as u64,
            b"%06d\x00" as *const u8 as *const i8,
            unique_tag_state,
        );
        unique_tag_state += 1;
        return;
    }
    if unique_tag_state != 0 {
        let mut current_time: time_t = 0;
        current_time = get_unique_time_if_given();
        if current_time == -1i32 as time_t {
            current_time = time(0 as *mut time_t)
        }
        srand(current_time as u32);
        unique_tag_state = 0i32
    }
    i = 0i32;
    while i < 6i32 {
        ch = (rand() % 26i32) as i8;
        *tag.offset(i as isize) = (ch as i32 + 'A' as i32) as i8;
        i += 1
    }
    *tag.offset(6) = '\u{0}' as i32 as i8;
}
unsafe extern "C" fn pdf_init_font_struct(mut font: *mut pdf_font) {
    assert!(!font.is_null());
    (*font).ident = 0 as *mut i8;
    (*font).map_name = 0 as *mut i8;
    (*font).subtype = -1i32;
    (*font).font_id = -1i32;
    (*font).fontname = 0 as *mut i8;
    memset(
        (*font).uniqueID.as_mut_ptr() as *mut libc::c_void,
        0i32,
        7i32 as u64,
    );
    (*font).index = 0i32;
    (*font).encoding_id = -1i32;
    (*font).reference = 0 as *mut pdf_obj;
    (*font).resource = 0 as *mut pdf_obj;
    (*font).descriptor = 0 as *mut pdf_obj;
    (*font).point_size = 0i32 as f64;
    (*font).design_size = 0i32 as f64;
    (*font).usedchars = 0 as *mut i8;
    (*font).flags = 0i32;
}
unsafe extern "C" fn pdf_flush_font(mut font: *mut pdf_font) {
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut uniqueTag: *mut i8 = 0 as *mut i8;
    if font.is_null() {
        return;
    }
    if !(*font).resource.is_null() && !(*font).reference.is_null() {
        if (*font).subtype != 2i32 {
            if pdf_font_get_flag(font, 1i32 << 0i32) != 0 {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"BaseFont\x00" as *const u8 as *const i8),
                    pdf_new_name((*font).fontname),
                );
                if !(*font).descriptor.is_null() {
                    pdf_add_dict(
                        (*font).descriptor,
                        pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
                        pdf_new_name((*font).fontname),
                    );
                }
            } else {
                if (*font).fontname.is_null() {
                    _tt_abort(
                        b"Undefined in fontname... (%s)\x00" as *const u8 as *const i8,
                        (*font).ident,
                    );
                }
                fontname = new(((7i32 as u64)
                    .wrapping_add(strlen((*font).fontname))
                    .wrapping_add(1i32 as u64) as u32 as u64)
                    .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                    as u32) as *mut i8;
                uniqueTag = pdf_font_get_uniqueTag(font);
                sprintf(
                    fontname,
                    b"%6s+%s\x00" as *const u8 as *const i8,
                    uniqueTag,
                    (*font).fontname,
                );
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"BaseFont\x00" as *const u8 as *const i8),
                    pdf_new_name(fontname),
                );
                if !(*font).descriptor.is_null() {
                    pdf_add_dict(
                        (*font).descriptor,
                        pdf_new_name(b"FontName\x00" as *const u8 as *const i8),
                        pdf_new_name(fontname),
                    );
                }
                free(fontname as *mut libc::c_void);
            }
            if !(*font).descriptor.is_null() {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
                    pdf_ref_obj((*font).descriptor),
                );
            }
        }
    }
    pdf_release_obj((*font).resource);
    pdf_release_obj((*font).descriptor);
    pdf_release_obj((*font).reference);
    (*font).reference = 0 as *mut pdf_obj;
    (*font).resource = 0 as *mut pdf_obj;
    (*font).descriptor = 0 as *mut pdf_obj;
}
unsafe extern "C" fn pdf_clean_font_struct(mut font: *mut pdf_font) {
    if !font.is_null() {
        free((*font).ident as *mut libc::c_void);
        free((*font).map_name as *mut libc::c_void);
        free((*font).fontname as *mut libc::c_void);
        free((*font).usedchars as *mut libc::c_void);
        if !(*font).reference.is_null() {
            _tt_abort(b"pdf_font>> Object not flushed.\x00" as *const u8 as *const i8);
        }
        if !(*font).resource.is_null() {
            _tt_abort(b"pdf_font> Object not flushed.\x00" as *const u8 as *const i8);
        }
        if !(*font).descriptor.is_null() {
            _tt_abort(b"pdf_font>> Object not flushed.\x00" as *const u8 as *const i8);
        }
        (*font).ident = 0 as *mut i8;
        (*font).map_name = 0 as *mut i8;
        (*font).fontname = 0 as *mut i8;
        (*font).usedchars = 0 as *mut i8
    };
}
static mut font_cache: C2RustUnnamed_0 = {
    let mut init = C2RustUnnamed_0 {
        count: 0i32,
        capacity: 0i32,
        fonts: 0 as *const pdf_font as *mut pdf_font,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn pdf_init_fonts() {
    assert!(font_cache.fonts.is_null());
    agl_init_map();
    CMap_cache_init();
    pdf_init_encodings();
    Type0Font_cache_init();
    font_cache.count = 0i32;
    font_cache.capacity = 16u32 as i32;
    font_cache.fonts = new((font_cache.capacity as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<pdf_font>() as u64) as u32)
        as *mut pdf_font;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_font_reference(mut font_id: i32) -> *mut pdf_obj {
    let mut font: *mut pdf_font = 0 as *mut pdf_font;
    if font_id < 0i32 || font_id >= font_cache.count {
        _tt_abort(
            b"Invalid font ID: %d\x00" as *const u8 as *const i8,
            font_id,
        );
    }
    font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    if (*font).subtype == 4i32 {
        let mut t0font: *mut Type0Font = 0 as *mut Type0Font;
        t0font = Type0Font_cache_get((*font).font_id);
        return Type0Font_get_resource(t0font);
    } else {
        if (*font).reference.is_null() {
            (*font).reference = pdf_ref_obj(pdf_font_get_resource(font))
        }
    }
    pdf_link_obj((*font).reference)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_font_usedchars(mut font_id: i32) -> *mut i8 {
    let mut font: *mut pdf_font = 0 as *mut pdf_font;
    if font_id < 0i32 || font_id >= font_cache.count {
        _tt_abort(
            b"Invalid font ID: %d\x00" as *const u8 as *const i8,
            font_id,
        );
    }
    font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    if (*font).subtype == 4i32 {
        let mut t0font: *mut Type0Font = 0 as *mut Type0Font;
        t0font = Type0Font_cache_get((*font).font_id);
        return Type0Font_get_usedchars(t0font);
    } else {
        if (*font).usedchars.is_null() {
            (*font).usedchars =
                new((256_u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
            memset(
                (*font).usedchars as *mut libc::c_void,
                0i32,
                (256i32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64),
            );
        }
        return (*font).usedchars;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_font_wmode(mut font_id: i32) -> i32 {
    let mut font: *mut pdf_font = 0 as *mut pdf_font;
    if font_id < 0i32 || font_id >= font_cache.count {
        _tt_abort(
            b"Invalid font ID: %d\x00" as *const u8 as *const i8,
            font_id,
        );
    }
    font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    if (*font).subtype == 4i32 {
        let mut t0font: *mut Type0Font = 0 as *mut Type0Font;
        t0font = Type0Font_cache_get((*font).font_id);
        return Type0Font_get_wmode(t0font);
    } else {
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_font_subtype(mut font_id: i32) -> i32 {
    let mut font: *mut pdf_font = 0 as *mut pdf_font;
    if font_id < 0i32 || font_id >= font_cache.count {
        _tt_abort(
            b"Invalid font ID: %d\x00" as *const u8 as *const i8,
            font_id,
        );
    }
    font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    (*font).subtype
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_font_encoding(mut font_id: i32) -> i32 {
    let mut font: *mut pdf_font = 0 as *mut pdf_font;
    if font_id < 0i32 || font_id >= font_cache.count {
        _tt_abort(
            b"Invalid font ID: %d\x00" as *const u8 as *const i8,
            font_id,
        );
    }
    font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    (*font).encoding_id
}
/* The rule for ToUnicode creation is:
 *
 *  If "tounicode" option is specified in fontmap, use that.
 *  If there is ToUnicode CMap with same name as TFM, use that.
 *  If no "tounicode" option is used and no ToUnicode CMap with
 *  same name as TFM is found, create ToUnicode CMap from glyph
 *  names and AGL file.
 */
unsafe extern "C" fn try_load_ToUnicode_CMap(mut font: *mut pdf_font) -> i32 {
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj; /* Be sure fontmap is still alive here */
    let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap_name: *const i8 = 0 as *const i8;
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    assert!(!font.is_null());
    /* We are using different encoding for Type0 font.
     * This feature is unavailable for them.
     */
    if (*font).subtype == 4i32 {
        return 0i32;
    } /* _FIXME_ */
    assert!(!(*font).map_name.is_null());
    mrec = pdf_lookup_fontmap_record((*font).map_name);
    if !mrec.is_null() && !(*mrec).opt.tounicode.is_null() {
        cmap_name = (*mrec).opt.tounicode
    } else {
        cmap_name = (*font).map_name
    }
    fontdict = pdf_font_get_resource(font);
    tounicode = pdf_load_ToUnicode_stream(cmap_name);
    if tounicode.is_null() && (!mrec.is_null() && !(*mrec).opt.tounicode.is_null()) {
        dpx_warning(
            b"Failed to read ToUnicode mapping \"%s\"...\x00" as *const u8 as *const i8,
            (*mrec).opt.tounicode,
        );
    } else if !tounicode.is_null() {
        if pdf_obj_typeof(tounicode) != 7i32 {
            _tt_abort(b"Object returned by pdf_load_ToUnicode_stream() not stream object! (This must be bug)\x00"
                          as *const u8 as *const i8);
        } else {
            if pdf_stream_length(tounicode) > 0i32 {
                pdf_add_dict(
                    fontdict,
                    pdf_new_name(b"ToUnicode\x00" as *const u8 as *const i8),
                    pdf_ref_obj(tounicode),
                );
                if __verbose != 0 {
                    dpx_message(
                        b"pdf_font>> ToUnicode CMap \"%s\" attached to font id=\"%s\".\n\x00"
                            as *const u8 as *const i8,
                        cmap_name,
                        (*font).map_name,
                    );
                }
            }
        }
        pdf_release_obj(tounicode);
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close_fonts() {
    let mut font_id: i32 = 0;
    font_id = 0i32;
    while font_id < font_cache.count {
        let mut font: *mut pdf_font = 0 as *mut pdf_font;
        font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        if __verbose != 0 {
            if (*font).subtype != 4i32 {
                dpx_message(
                    b"(%s\x00" as *const u8 as *const i8,
                    pdf_font_get_ident(font),
                );
                if __verbose > 2i32 && pdf_font_get_flag(font, 1i32 << 0i32) == 0 {
                    dpx_message(
                        b"[%s+%s]\x00" as *const u8 as *const i8,
                        pdf_font_get_uniqueTag(font),
                        pdf_font_get_fontname(font),
                    );
                } else if __verbose > 1i32 {
                    dpx_message(
                        b"[%s]\x00" as *const u8 as *const i8,
                        pdf_font_get_fontname(font),
                    );
                }
                if __verbose > 1i32 {
                    if pdf_font_get_encoding(font) >= 0i32 {
                        dpx_message(
                            b"[%s]\x00" as *const u8 as *const i8,
                            pdf_encoding_get_name(pdf_font_get_encoding(font)),
                        );
                    } else {
                        info!("[built-in]");
                    }
                }
            }
        }
        /* Must come before load_xxx */
        try_load_ToUnicode_CMap(font);
        /* Type 0 is handled separately... */
        match (*font).subtype {
            0 => {
                if __verbose != 0 {
                    info!("[Type1]");
                }
                if pdf_font_get_flag(font, 1i32 << 2i32) == 0 {
                    pdf_font_load_type1(font);
                }
            }
            1 => {
                if __verbose != 0 {
                    info!("[Type1C]");
                }
                pdf_font_load_type1c(font);
            }
            3 => {
                if __verbose != 0 {
                    info!("[TrueType]");
                }
                pdf_font_load_truetype(font);
            }
            2 => {
                if __verbose != 0 {
                    info!("[Type3/PK]");
                }
                pdf_font_load_pkfont(font);
            }
            4 => {}
            _ => {
                _tt_abort(
                    b"Unknown font type: %d\x00" as *const u8 as *const i8,
                    (*font).subtype,
                );
            }
        }
        if (*font).encoding_id >= 0i32 && (*font).subtype != 4i32 {
            pdf_encoding_add_usedchars((*font).encoding_id, (*font).usedchars);
        }
        if __verbose != 0 {
            if (*font).subtype != 4i32 {
                info!(")");
            }
        }
        font_id += 1
    }
    pdf_encoding_complete();
    font_id = 0i32;
    while font_id < font_cache.count {
        let mut font_0: *mut pdf_font =
            &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        if (*font_0).encoding_id >= 0i32 && (*font_0).subtype != 4i32 {
            let mut enc_obj: *mut pdf_obj = pdf_get_encoding_obj((*font_0).encoding_id);
            let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
            /* Predefined encodings (and those simplified to them) are embedded
            as direct objects, but this is purely a matter of taste. */
            if !enc_obj.is_null() {
                pdf_add_dict(
                    (*font_0).resource,
                    pdf_new_name(b"Encoding\x00" as *const u8 as *const i8),
                    if !enc_obj.is_null() && pdf_obj_typeof(enc_obj) == 4i32 {
                        pdf_link_obj(enc_obj)
                    } else {
                        pdf_ref_obj(enc_obj)
                    },
                );
            }
            if pdf_lookup_dict(
                (*font_0).resource,
                b"ToUnicode\x00" as *const u8 as *const i8,
            )
            .is_null()
                && {
                    tounicode = pdf_encoding_get_tounicode((*font_0).encoding_id);
                    !tounicode.is_null()
                }
            {
                pdf_add_dict(
                    (*font_0).resource,
                    pdf_new_name(b"ToUnicode\x00" as *const u8 as *const i8),
                    pdf_ref_obj(tounicode),
                );
            }
        } else if (*font_0).subtype == 3i32 {
            /* encoding_id < 0 means MacRoman here (but not really)
             * We use MacRoman as "default" encoding. */
            pdf_add_dict(
                (*font_0).resource,
                pdf_new_name(b"Encoding\x00" as *const u8 as *const i8),
                pdf_new_name(b"MacRomanEncoding\x00" as *const u8 as *const i8),
            ); /* After encoding */
        }
        pdf_flush_font(font_0);
        pdf_clean_font_struct(font_0);
        font_id += 1
    }
    font_cache.fonts = mfree(font_cache.fonts as *mut libc::c_void) as *mut pdf_font;
    font_cache.count = 0i32;
    font_cache.capacity = 0i32;
    Type0Font_cache_close();
    CMap_cache_close();
    pdf_close_encodings();
    agl_close_map();
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_findresource(
    mut tex_name: *const i8,
    mut font_scale: f64,
    mut mrec: *mut fontmap_rec,
) -> i32 {
    let mut font_id: i32 = -1i32;
    let mut font: *mut pdf_font = 0 as *mut pdf_font;
    let mut encoding_id: i32 = -1i32;
    let mut cmap_id: i32 = -1i32;
    let mut fontname: *const i8 = 0 as *const i8;
    /*
     * Get appropriate info from map file. (PK fonts at two different
     * point sizes would be looked up twice unecessarily.)
     */
    fontname = if !mrec.is_null() {
        (*mrec).font_name as *const i8
    } else {
        tex_name
    };
    if !mrec.is_null() && !(*mrec).enc_name.is_null() {
        if strstr((*mrec).enc_name, b".enc\x00" as *const u8 as *const i8).is_null()
            || !strstr((*mrec).enc_name, b".cmap\x00" as *const u8 as *const i8).is_null()
        {
            cmap_id = CMap_cache_find((*mrec).enc_name);
            if cmap_id >= 0i32 {
                let mut cmap: *mut CMap = 0 as *mut CMap;
                let mut cmap_type: i32 = 0;
                let mut minbytes: i32 = 0;
                cmap = CMap_cache_get(cmap_id);
                cmap_type = CMap_get_type(cmap);
                minbytes = CMap_get_profile(cmap, 0i32);
                /*
                 * Check for output encoding.
                 */
                if cmap_type != 0i32 && cmap_type != 1i32 && cmap_type != 2i32 {
                    warn!("Only 16-bit encoding supported for output encoding.");
                }
                /*
                 * Turn on map option.
                 */
                if minbytes == 2i32 && (*mrec).opt.mapc < 0i32 {
                    if __verbose != 0 {
                        info!("\n");
                        dpx_message(
                            b"pdf_font>> Input encoding \"%s\" requires at least 2 bytes.\n\x00"
                                as *const u8 as *const i8,
                            CMap_get_name(cmap),
                        );
                        dpx_message(
                            b"pdf_font>> The -m <00> option will be assumed for \"%s\".\n\x00"
                                as *const u8 as *const i8,
                            (*mrec).font_name,
                        );
                    }
                    (*mrec).opt.mapc = 0i32
                    /* _FIXME_ */
                }
            } else if streq_ptr((*mrec).enc_name, b"unicode\x00" as *const u8 as *const i8) {
                cmap_id = otf_load_Unicode_CMap(
                    (*mrec).font_name,
                    (*mrec).opt.index,
                    (*mrec).opt.otl_tags,
                    if (*mrec).opt.flags & 1i32 << 2i32 != 0 {
                        1i32
                    } else {
                        0i32
                    },
                );
                if cmap_id < 0i32 {
                    cmap_id = t1_load_UnicodeCMap(
                        (*mrec).font_name,
                        (*mrec).opt.otl_tags,
                        if (*mrec).opt.flags & 1i32 << 2i32 != 0 {
                            1i32
                        } else {
                            0i32
                        },
                    )
                }
                if cmap_id < 0i32 {
                    _tt_abort(
                        b"Failed to read UCS2/UCS4 TrueType cmap...\x00" as *const u8 as *const i8,
                    );
                }
            }
        }
        if cmap_id < 0i32 {
            encoding_id = pdf_encoding_findresource((*mrec).enc_name);
            if encoding_id < 0i32 {
                _tt_abort(
                    b"Could not find encoding file \"%s\".\x00" as *const u8 as *const i8,
                    (*mrec).enc_name,
                );
            }
        }
    }
    if !mrec.is_null() && cmap_id >= 0i32 {
        /*
         * Composite Font
         */
        let mut type0_id: i32 = 0;
        let mut found: i32 = 0i32;
        type0_id = Type0Font_cache_find((*mrec).font_name, cmap_id, &mut (*mrec).opt);
        if type0_id < 0i32 {
            return -1i32;
        }
        font_id = 0i32;
        while font_id < font_cache.count {
            font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
            if (*font).subtype == 4i32
                && (*font).font_id == type0_id
                && (*font).encoding_id == cmap_id
            {
                found = 1i32;
                if __verbose != 0 {
                    dpx_message(
                        b"\npdf_font>> Type0 font \"%s\" (cmap_id=%d) found at font_id=%d.\n\x00"
                            as *const u8 as *const i8,
                        (*mrec).font_name,
                        cmap_id,
                        font_id,
                    );
                }
                break;
            } else {
                font_id += 1
            }
        }
        if found == 0 {
            font_id = font_cache.count;
            if font_cache.count >= font_cache.capacity {
                font_cache.capacity =
                    (font_cache.capacity as u32).wrapping_add(16u32) as i32 as i32;
                font_cache.fonts = renew(
                    font_cache.fonts as *mut libc::c_void,
                    (font_cache.capacity as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<pdf_font>() as u64)
                        as u32,
                ) as *mut pdf_font
            }
            font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
            pdf_init_font_struct(font);
            (*font).font_id = type0_id;
            (*font).subtype = 4i32;
            (*font).encoding_id = cmap_id;
            font_cache.count += 1;
            if __verbose != 0 {
                dpx_message(
                    b"\npdf_font>> Type0 font \"%s\"\x00" as *const u8 as *const i8,
                    fontname,
                );
                dpx_message(
                    b" cmap_id=<%s,%d>\x00" as *const u8 as *const i8,
                    (*mrec).enc_name,
                    (*font).encoding_id,
                );
                dpx_message(
                    b" opened at font_id=<%s,%d>.\n\x00" as *const u8 as *const i8,
                    tex_name,
                    font_id,
                );
            }
        }
    } else {
        /*
         * Simple Font - always embed.
         */
        let mut found_0: i32 = 0i32;
        font_id = 0i32;
        while font_id < font_cache.count {
            font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
            match (*font).subtype {
                0 | 1 | 3 => {
                    /* fontname here is font file name.
                     * We must compare both font file name and encoding
                     *
                     * TODO: Embed a font only once if it is used
                     *       with two different encodings
                     */
                    if streq_ptr(fontname, (*font).ident) as i32 != 0
                        && encoding_id == (*font).encoding_id
                    {
                        if !mrec.is_null() && (*mrec).opt.index == (*font).index {
                            found_0 = 1i32
                        }
                    }
                }
                2 => {
                    /* There shouldn't be any encoding specified for PK font.
                     * It must be always font's build-in encoding.
                     *
                     * TODO: a PK font with two encodings makes no sense. Change?
                     */
                    if streq_ptr(fontname, (*font).ident) as i32 != 0
                        && font_scale == (*font).point_size
                    {
                        found_0 = 1i32
                    }
                }
                4 => {}
                _ => {
                    _tt_abort(
                        b"Unknown font type: %d\x00" as *const u8 as *const i8,
                        (*font).subtype,
                    );
                }
            }
            if found_0 != 0 {
                if __verbose != 0 {
                    dpx_message(
                        b"\npdf_font>> Simple font \"%s\" (enc_id=%d) found at id=%d.\n\x00"
                            as *const u8 as *const i8,
                        fontname,
                        encoding_id,
                        font_id,
                    );
                }
                break;
            } else {
                font_id += 1
            }
        }
        if found_0 == 0 {
            font_id = font_cache.count;
            if font_cache.count >= font_cache.capacity {
                font_cache.capacity =
                    (font_cache.capacity as u32).wrapping_add(16u32) as i32 as i32;
                font_cache.fonts = renew(
                    font_cache.fonts as *mut libc::c_void,
                    (font_cache.capacity as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<pdf_font>() as u64)
                        as u32,
                ) as *mut pdf_font
            }
            font = &mut *font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
            pdf_init_font_struct(font);
            (*font).point_size = font_scale;
            (*font).encoding_id = encoding_id;
            (*font).ident = new((strlen(fontname).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            strcpy((*font).ident, fontname);
            (*font).map_name = new((strlen(tex_name).wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            strcpy((*font).map_name, tex_name);
            (*font).index = if !mrec.is_null() && (*mrec).opt.index != 0 {
                (*mrec).opt.index
            } else {
                0i32
            };
            if pdf_font_open_type1(font) >= 0i32 {
                (*font).subtype = 0i32
            } else if pdf_font_open_type1c(font) >= 0i32 {
                (*font).subtype = 1i32
            } else if pdf_font_open_truetype(font) >= 0i32 {
                (*font).subtype = 3i32
            } else if pdf_font_open_pkfont(font) >= 0i32 {
                (*font).subtype = 2i32
            } else {
                pdf_clean_font_struct(font);
                return -1i32;
            }
            font_cache.count += 1;
            if __verbose != 0 {
                dpx_message(
                    b"\npdf_font>> Simple font \"%s\"\x00" as *const u8 as *const i8,
                    fontname,
                );
                dpx_message(
                    b" enc_id=<%s,%d>\x00" as *const u8 as *const i8,
                    if !mrec.is_null() && !(*mrec).enc_name.is_null() {
                        (*mrec).enc_name as *const i8
                    } else {
                        b"builtin\x00" as *const u8 as *const i8
                    },
                    (*font).encoding_id,
                );
                dpx_message(
                    b" opened at font_id=<%s,%d>.\n\x00" as *const u8 as *const i8,
                    tex_name,
                    font_id,
                );
            }
        }
    }
    font_id
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_is_in_use(mut font: *mut pdf_font) -> bool {
    assert!(!font.is_null());
    return if !(*font).reference.is_null() {
        1i32
    } else {
        0i32
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_index(mut font: *mut pdf_font) -> i32 {
    assert!(!font.is_null());
    (*font).index
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_ident(mut font: *mut pdf_font) -> *mut i8 {
    assert!(!font.is_null());
    (*font).ident
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_mapname(mut font: *mut pdf_font) -> *mut i8 {
    assert!(!font.is_null());
    (*font).map_name
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_fontname(mut font: *mut pdf_font) -> *mut i8 {
    assert!(!font.is_null());
    (*font).fontname
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_resource(mut font: *mut pdf_font) -> *mut pdf_obj {
    assert!(!font.is_null());
    if (*font).resource.is_null() {
        (*font).resource = pdf_new_dict();
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"Type\x00" as *const u8 as *const i8),
            pdf_new_name(b"Font\x00" as *const u8 as *const i8),
        );
        match (*font).subtype {
            0 | 1 => {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
                    pdf_new_name(b"Type1\x00" as *const u8 as *const i8),
                );
            }
            2 => {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
                    pdf_new_name(b"Type3\x00" as *const u8 as *const i8),
                );
            }
            3 => {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
                    pdf_new_name(b"TrueType\x00" as *const u8 as *const i8),
                );
            }
            _ => {}
        }
    }
    (*font).resource
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_descriptor(mut font: *mut pdf_font) -> *mut pdf_obj {
    assert!(!font.is_null());
    if (*font).descriptor.is_null() {
        (*font).descriptor = pdf_new_dict();
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"Type\x00" as *const u8 as *const i8),
            pdf_new_name(b"FontDescriptor\x00" as *const u8 as *const i8),
        );
    }
    (*font).descriptor
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_usedchars(mut font: *mut pdf_font) -> *mut i8 {
    assert!(!font.is_null());
    (*font).usedchars
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_encoding(mut font: *mut pdf_font) -> i32 {
    assert!(!font.is_null());
    (*font).encoding_id
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_flag(mut font: *mut pdf_font, mut mask: i32) -> i32 {
    assert!(!font.is_null());
    return if (*font).flags & mask != 0 {
        1i32
    } else {
        0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_param(mut font: *mut pdf_font, mut param_type: i32) -> f64 {
    let mut param: f64 = 0.0f64;
    assert!(!font.is_null());
    match param_type {
        1 => param = (*font).design_size,
        2 => param = (*font).point_size,
        _ => {}
    }
    param
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_get_uniqueTag(mut font: *mut pdf_font) -> *mut i8 {
    assert!(!font.is_null());
    if (*font).uniqueID[0] as i32 == '\u{0}' as i32 {
        pdf_font_make_uniqueTag((*font).uniqueID.as_mut_ptr());
    }
    (*font).uniqueID.as_mut_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_set_fontname(
    mut font: *mut pdf_font,
    mut fontname: *const i8,
) -> i32 {
    assert!(!font.is_null() && !fontname.is_null());
    if strlen(fontname) > 127i32 as u64 {
        _tt_abort(b"Unexpected error...\x00" as *const u8 as *const i8);
    }
    if !(*font).fontname.is_null() {
        free((*font).fontname as *mut libc::c_void);
    }
    (*font).fontname = new((strlen(fontname).wrapping_add(1i32 as u64) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32) as *mut i8;
    strcpy((*font).fontname, fontname);
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_set_subtype(mut font: *mut pdf_font, mut subtype: i32) -> i32 {
    assert!(!font.is_null());
    (*font).subtype = subtype;
    0i32
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
/* pdf_open_document() call them. */
/* font_name is used when mrec is NULL.
 * font_scale (point size) used by PK font.
 * It might be necessary if dvipdfmx supports font format with
 * various optical sizes supported in the future.
 */
/* Each font drivers use the followings. */
/* without unique tag */
#[no_mangle]
pub unsafe extern "C" fn pdf_font_set_flags(mut font: *mut pdf_font, mut flags: i32) -> i32 {
    assert!(!font.is_null());
    (*font).flags |= flags;
    0i32
}
