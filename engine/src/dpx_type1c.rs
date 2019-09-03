#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types, label_break_value)]
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
    pub type pdf_font;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t,
                         whence: libc::c_int) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut libc::c_char,
                         len: size_t) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
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
    fn pdf_array_length(array: *mut pdf_obj) -> libc::c_uint;
    #[no_mangle]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const libc::c_char)
     -> *mut pdf_obj;
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
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(stream: *mut pdf_obj,
                      stream_data_ptr: *const libc::c_void,
                      stream_data_len: libc::c_int);
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_stream_length(stream: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_stream_dataptr(stream: *mut pdf_obj) -> *const libc::c_void;
    #[no_mangle]
    fn pdf_font_get_verbose() -> libc::c_int;
    /* Each font drivers use the followings. */
    #[no_mangle]
    fn pdf_font_is_in_use(font: *mut pdf_font) -> bool;
    #[no_mangle]
    fn pdf_font_get_ident(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_mapname(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_fontname(font: *mut pdf_font) -> *mut libc::c_char;
    /* without unique tag */
    #[no_mangle]
    fn pdf_font_get_uniqueTag(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_descriptor(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_usedchars(font: *mut pdf_font) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_font_get_encoding(font: *mut pdf_font) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_flag(font: *mut pdf_font, mask: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn pdf_font_set_fontname(font: *mut pdf_font,
                             fontname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_set_flags(font: *mut pdf_font, flags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn pdf_font_set_subtype(font: *mut pdf_font, subtype: libc::c_int)
     -> libc::c_int;
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
    /* Flag */
    /* FontName */
    /* - CFF structure - */
    /* CFF Header */
    /* Name INDEX */
    /* Top DICT (single) */
    /* String INDEX */
    /* Global Subr INDEX */
    /* Encodings */
    /* Charsets  */
    /* FDSelect, CIDFont only */
    /* CharStrings */
    /* CIDFont only */
    /* per-Font DICT */
    /* Local Subr INDEX, per-Private DICT */
    /* -- extra data -- */
    /* non-zero for OpenType or PostScript wrapped */
    /* number of glyphs (CharString INDEX count) */
    /* number of Font DICT */
    /* Updated String INDEX.
     * Please fix this. We should separate input and output.
     */
    /* not used, ASCII Hex filter if needed */
    /* CFF fontset index */
    /* Flag: see above */
    /* 1 if .notdef is not the 1st glyph */
    /* CFF Header */
    /* CFF INDEX */
    /* Name INDEX */
    #[no_mangle]
    fn cff_get_name(cff: *mut cff_font) -> *mut libc::c_char;
    #[no_mangle]
    fn cff_open(handle: rust_input_handle_t, offset: libc::c_int,
                idx: libc::c_int) -> *mut cff_font;
    #[no_mangle]
    fn cff_close(cff: *mut cff_font);
    #[no_mangle]
    fn cff_release_index(idx: *mut cff_index);
    #[no_mangle]
    fn cff_index_size(idx: *mut cff_index) -> libc::c_int;
    #[no_mangle]
    fn cff_pack_index(idx: *mut cff_index, dest: *mut card8,
                      destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_put_header(cff: *mut cff_font, dest: *mut card8,
                      destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut work_buffer: [libc::c_char; 0];
    #[no_mangle]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[no_mangle]
    fn cff_get_index_header(cff: *mut cff_font) -> *mut cff_index;
    #[no_mangle]
    fn cff_pack_charsets(cff: *mut cff_font, dest: *mut card8,
                         destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_pack_encoding(cff: *mut cff_font, dest: *mut card8,
                         destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_release_encoding(encoding: *mut cff_encoding);
    #[no_mangle]
    fn cff_set_name(cff: *mut cff_font, name: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn cff_read_charsets(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_read_encoding(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_read_subrs(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_encoding_lookup(cff: *mut cff_font, code: card8) -> card16;
    /* Returns GID of glyph with SID/CID "cid" */
    /* Returns SID or CID */
    /* FDSelect */
    /* Font DICT(s) */
    /* Private DICT(s) */
    /* String */
    #[no_mangle]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut libc::c_char;
    #[no_mangle]
    fn cff_release_charsets(charset: *mut cff_charsets);
    #[no_mangle]
    fn cff_charsets_lookup_inverse(cff: *mut cff_font, gid: card16) -> card16;
    #[no_mangle]
    fn cff_read_private(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_charsets_lookup(cff: *mut cff_font, cid: card16) -> card16;
    #[no_mangle]
    fn cff_get_sid(cff: *mut cff_font, str: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn cff_add_string(cff: *mut cff_font, str: *const libc::c_char,
                      unique: libc::c_int) -> s_SID;
    #[no_mangle]
    fn cff_update_string(cff: *mut cff_font);
    #[no_mangle]
    fn cff_dict_set(dict: *mut cff_dict, key: *const libc::c_char,
                    idx: libc::c_int, value: libc::c_double);
    #[no_mangle]
    fn cff_dict_get(dict: *mut cff_dict, key: *const libc::c_char,
                    idx: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn cff_dict_add(dict: *mut cff_dict, key: *const libc::c_char,
                    count: libc::c_int);
    #[no_mangle]
    fn cff_dict_remove(dict: *mut cff_dict, key: *const libc::c_char);
    #[no_mangle]
    fn cff_dict_known(dict: *mut cff_dict, key: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn cff_dict_pack(dict: *mut cff_dict, dest: *mut card8,
                     destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_dict_update(dict: *mut cff_dict, cff: *mut cff_font);
    #[no_mangle]
    fn cs_copy_charstring(dest: *mut card8, destlen: libc::c_int,
                          src: *mut card8, srclen: libc::c_int,
                          gsubr: *mut cff_index, subr: *mut cff_index,
                          default_width: libc::c_double,
                          nominal_width: libc::c_double, ginfo: *mut cs_ginfo)
     -> libc::c_int;
    #[no_mangle]
    fn dpx_open_opentype_file(filename: *const libc::c_char)
     -> rust_input_handle_t;
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
    fn dpx_message(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_encoding_get_encoding(enc_id: libc::c_int)
     -> *mut *mut libc::c_char;
    /*
 * pdf_create_ToUnicode_CMap() returns stream object but not
 * reference. This need to be renamed to other name like
 * pdf_create_ToUnicode_stream().
 */
    #[no_mangle]
    fn pdf_create_ToUnicode_CMap(enc_name: *const libc::c_char,
                                 enc_vec: *mut *mut libc::c_char,
                                 is_used: *const libc::c_char)
     -> *mut pdf_obj;
    #[no_mangle]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[no_mangle]
    fn sfnt_close(sfont: *mut sfnt);
    /* table directory */
    #[no_mangle]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG)
     -> libc::c_int;
    #[no_mangle]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const libc::c_char)
     -> SFNT_ULONG;
    #[no_mangle]
    fn tfm_open(tex_name: *const libc::c_char, must_exist: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn tfm_get_width(font_id: libc::c_int, ch: int32_t) -> libc::c_double;
    /* FontDescriptor */
    #[no_mangle]
    fn tt_get_fontdesc(sfont: *mut sfnt, embed: *mut libc::c_int,
                       stemv: libc::c_int, type_0: libc::c_int,
                       fontname: *const libc::c_char) -> *mut pdf_obj;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
pub type card8 = libc::c_uchar;
pub type l_offset = uint32_t;
pub type c_offsize = libc::c_uchar;
pub type card16 = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
pub type s_SID = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
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
/* CFF Data Types */
/* SID SID number */
/* offset(0) */
/* size offset(0) */
/* 1-byte unsigned number */
/* 2-byte unsigned number */
/* 1-byte unsigned number specifies the size
                                     of an Offset field or fields, range 1-4 */
/* 1, 2, 3, or 4-byte offset */
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
/* if (format & 0x80) then have supplement */
/* number of entries */
/* format 0 */
/* format 1 */
/* number of supplementary data */
/* supplement */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_0 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct sfnt {
    pub type_0: libc::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
pub type SFNT_ULONG = uint32_t;
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
pub struct sfnt_table {
    pub tag: [libc::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut libc::c_char,
    /* table data */
}
pub type USHORT = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut libc::c_char,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: card16,
    pub num_fds: card8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: libc::c_int,
    pub index: libc::c_int,
    pub flag: libc::c_int,
    pub is_notdef_notzero: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_dict {
    pub max: libc::c_int,
    pub count: libc::c_int,
    pub entries: *mut cff_dict_entry,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: libc::c_int,
    pub key: *const libc::c_char,
    pub count: libc::c_int,
    pub values: *mut libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cs_ginfo {
    pub flags: libc::c_int,
    pub wx: libc::c_double,
    pub wy: libc::c_double,
    pub bbox: C2RustUnnamed_3,
    pub seac: C2RustUnnamed_2,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub asb: libc::c_double,
    pub adx: libc::c_double,
    pub ady: libc::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char,
                               mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() { return strcmp(s1, s2) == 0i32 }
    return 0i32 != 0;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2008-2016 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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
/*
 * CFF/OpenType Font support:
 *
 *  Adobe Technical Note #5176, "The Compact Font Format Specfication"
 *
 * NOTE:
 *
 *  Many CFF/OpenType does not have meaningful/correct CFF encoding.
 *  Encoding should be expilicitly supplied in the fontmap.
 *
 */
/* Font info. from OpenType tables */
#[no_mangle]
pub unsafe extern "C" fn pdf_font_open_type1c(mut font: *mut pdf_font)
 -> libc::c_int {
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ident: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut offset: libc::c_uint = 0i32 as libc::c_uint;
    let mut encoding_id: libc::c_int = 0;
    let mut embedding: libc::c_int = 0;
    if !font.is_null() {
    } else {
        __assert_fail(b"font\x00" as *const u8 as *const libc::c_char,
                      b"dpx-type1c.c\x00" as *const u8 as *const libc::c_char,
                      74i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"int pdf_font_open_type1c(pdf_font *)\x00")).as_ptr());
    }
    ident = pdf_font_get_ident(font);
    encoding_id = pdf_font_get_encoding(font);
    handle = dpx_open_opentype_file(ident) as *mut rust_input_handle_t;
    if handle.is_null() { return -1i32 }
    sfont = sfnt_open(handle as rust_input_handle_t);
    if sfont.is_null() || (*sfont).type_0 != 1i32 << 2i32 ||
           sfnt_read_table_directory(sfont, 0i32 as SFNT_ULONG) < 0i32 {
        _tt_abort(b"Not a CFF/OpenType font (9)?\x00" as *const u8 as
                      *const libc::c_char);
    }
    offset =
        sfnt_find_table_pos(sfont,
                            b"CFF \x00" as *const u8 as *const libc::c_char);
    if offset < 1i32 as libc::c_uint {
        _tt_abort(b"No \"CFF \" table found; not a CFF/OpenType font (10)?\x00"
                      as *const u8 as *const libc::c_char);
    }
    cffont = cff_open((*sfont).handle, offset as libc::c_int, 0i32);
    if cffont.is_null() {
        _tt_abort(b"Could not read CFF font data\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*cffont).flag & 1i32 << 0i32 != 0 {
        cff_close(cffont);
        sfnt_close(sfont);
        ttstub_input_close(handle as rust_input_handle_t);
        return -1i32
    }
    fontname = cff_get_name(cffont);
    if fontname.is_null() {
        _tt_abort(b"No valid FontName found in CFF/OpenType font.\x00" as
                      *const u8 as *const libc::c_char);
    }
    pdf_font_set_fontname(font, fontname);
    free(fontname as *mut libc::c_void);
    cff_close(cffont);
    /*
     * Font like AdobePiStd does not have meaningful built-in encoding.
     * Some software generate CFF/OpenType font with incorrect encoding.
     */
    if encoding_id < 0i32 {
        dpx_warning(b"Built-in encoding used for CFF/OpenType font.\x00" as
                        *const u8 as *const libc::c_char);
        dpx_warning(b"CFF font in OpenType font sometimes have strange built-in encoding.\x00"
                        as *const u8 as *const libc::c_char);
        dpx_warning(b"If you find text is not encoded properly in the generated PDF file,\x00"
                        as *const u8 as *const libc::c_char);
        dpx_warning(b"please specify appropriate \".enc\" file in your fontmap.\x00"
                        as *const u8 as *const libc::c_char);
    }
    pdf_font_set_subtype(font, 1i32);
    embedding =
        if pdf_font_get_flag(font, 1i32 << 0i32) != 0 { 0i32 } else { 1i32 };
    descriptor = pdf_font_get_descriptor(font);
    /*
     * Create font descriptor from OpenType tables.
     * We can also use CFF TOP DICT/Private DICT for this.
     */
    tmp =
        tt_get_fontdesc(sfont, &mut embedding, -1i32, 1i32,
                        fontname); /* copy */
    if tmp.is_null() {
        _tt_abort(b"Could not obtain neccesary font info from OpenType table.\x00"
                      as *const u8 as *const libc::c_char);
    }
    pdf_merge_dict(descriptor, tmp);
    pdf_release_obj(tmp);
    if embedding == 0 {
        /* tt_get_fontdesc may have changed this */
        pdf_font_set_flags(font, 1i32 << 0i32);
    }
    sfnt_close(sfont);
    ttstub_input_close(handle as rust_input_handle_t);
    return 0i32;
}
unsafe extern "C" fn add_SimpleMetrics(mut font: *mut pdf_font,
                                       mut cffont: *mut cff_font,
                                       mut widths: *mut libc::c_double,
                                       mut num_glyphs: card16) {
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut code: libc::c_int = 0;
    let mut firstchar: libc::c_int = 0;
    let mut lastchar: libc::c_int = 0;
    let mut tfm_id: libc::c_int = 0;
    let mut usedchars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut scaling: libc::c_double = 0.;
    fontdict = pdf_font_get_resource(font);
    usedchars = pdf_font_get_usedchars(font);
    /* The widhts array in the font dictionary must be given relative
     * to the default scaling of 1000:1, not relative to the scaling
     * given by the font matrix.
     */
    if cff_dict_known((*cffont).topdict,
                      b"FontMatrix\x00" as *const u8 as *const libc::c_char)
           != 0 {
        scaling =
            1000i32 as libc::c_double *
                cff_dict_get((*cffont).topdict,
                             b"FontMatrix\x00" as *const u8 as
                                 *const libc::c_char, 0i32)
    } else { scaling = 1i32 as libc::c_double }
    tmp_array = pdf_new_array();
    if num_glyphs as libc::c_int <= 1i32 {
        /* This should be error. */
        lastchar = 0i32;
        firstchar = lastchar;
        pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    } else {
        firstchar = 255i32;
        lastchar = 0i32;
        code = 0i32;
        while code < 256i32 {
            if *usedchars.offset(code as isize) != 0 {
                if code < firstchar { firstchar = code }
                if code > lastchar { lastchar = code }
            }
            code += 1
        }
        if firstchar > lastchar {
            pdf_release_obj(tmp_array);
            _tt_abort(b"No glyphs used at all!\x00" as *const u8 as
                          *const libc::c_char);
        }
        tfm_id = tfm_open(pdf_font_get_mapname(font), 0i32);
        code = firstchar;
        while code <= lastchar {
            if *usedchars.offset(code as isize) != 0 {
                let mut width: libc::c_double = 0.;
                if tfm_id < 0i32 {
                    /* tfm is not found */
                    width = scaling * *widths.offset(code as isize)
                } else {
                    let mut diff: libc::c_double = 0.;
                    width = 1000.0f64 * tfm_get_width(tfm_id, code);
                    diff = width - scaling * *widths.offset(code as isize);
                    if fabs(diff) > 1.0f64 {
                        dpx_warning(b"Glyph width mismatch for TFM and font (%s)\x00"
                                        as *const u8 as *const libc::c_char,
                                    pdf_font_get_mapname(font));
                        dpx_warning(b"TFM: %g vs. CFF font: %g\x00" as
                                        *const u8 as *const libc::c_char,
                                    width, *widths.offset(code as isize));
                    }
                    pdf_add_array(tmp_array,
                                  pdf_new_number(floor(width / 0.1f64 +
                                                           0.5f64) * 0.1f64));
                }
            } else { pdf_add_array(tmp_array, pdf_new_number(0.0f64)); }
            code += 1
        }
    }
    if pdf_array_length(tmp_array) > 0i32 as libc::c_uint {
        pdf_add_dict(fontdict,
                     pdf_new_name(b"Widths\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_ref_obj(tmp_array));
    }
    pdf_release_obj(tmp_array);
    pdf_add_dict(fontdict,
                 pdf_new_name(b"FirstChar\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(firstchar as libc::c_double));
    pdf_add_dict(fontdict,
                 pdf_new_name(b"LastChar\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(lastchar as libc::c_double));
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
#[no_mangle]
pub unsafe extern "C" fn pdf_font_load_type1c(mut font: *mut pdf_font)
 -> libc::c_int {
    let mut fontdict: *mut pdf_obj =
        0 as *mut pdf_obj; /* Actually string object */
    let mut descriptor: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut pdfcharset: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut usedchars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uniqueTag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ident: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    let mut encoding_id: libc::c_int = 0;
    let mut fontfile: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut enc_vec: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut charstrings: *mut cff_index = 0 as *mut cff_index;
    let mut topdict: *mut cff_index = 0 as *mut cff_index;
    let mut cs_idx: *mut cff_index = 0 as *mut cff_index;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut topdict_offset: libc::c_int = 0;
    let mut private_size: libc::c_int = 0;
    let mut charstring_len: libc::c_int = 0;
    let mut max_len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut offset: libc::c_int = 0i32;
    let mut stream_data_len: libc::c_int = 0i32;
    let mut stream_data_ptr: *mut card8 = 0 as *mut card8;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut num_glyphs: card16 = 0;
    let mut cs_count: card16 = 0;
    let mut code: card16 = 0;
    let mut ginfo: cs_ginfo =
        cs_ginfo{flags: 0,
                 wx: 0.,
                 wy: 0.,
                 bbox: C2RustUnnamed_3{llx: 0., lly: 0., urx: 0., ury: 0.,},
                 seac:
                     C2RustUnnamed_2{asb: 0.,
                                     adx: 0.,
                                     ady: 0.,
                                     bchar: 0,
                                     achar: 0,},};
    let mut nominal_width: libc::c_double = 0.;
    let mut default_width: libc::c_double = 0.;
    let mut notdef_width: libc::c_double = 0.;
    let mut widths: [libc::c_double; 256] = [0.; 256];
    let mut verbose: libc::c_int = 0;
    if !font.is_null() {
    } else {
        __assert_fail(b"font\x00" as *const u8 as *const libc::c_char,
                      b"dpx-type1c.c\x00" as *const u8 as *const libc::c_char,
                      253i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"int pdf_font_load_type1c(pdf_font *)\x00")).as_ptr());
    }
    verbose = pdf_font_get_verbose();
    if !pdf_font_is_in_use(font) { return 0i32 }
    if pdf_font_get_flag(font, 1i32 << 0i32) != 0 {
        _tt_abort(b"Only embedded font supported for CFF/OpenType font.\x00"
                      as *const u8 as *const libc::c_char);
    }
    usedchars = pdf_font_get_usedchars(font);
    fontname = pdf_font_get_fontname(font);
    ident = pdf_font_get_ident(font);
    uniqueTag = pdf_font_get_uniqueTag(font);
    if usedchars.is_null() || fontname.is_null() || ident.is_null() {
        _tt_abort(b"Unexpected error....\x00" as *const u8 as
                      *const libc::c_char);
    }
    fontdict = pdf_font_get_resource(font);
    descriptor = pdf_font_get_descriptor(font);
    encoding_id = pdf_font_get_encoding(font);
    handle = dpx_open_opentype_file(ident) as *mut rust_input_handle_t;
    if handle.is_null() {
        _tt_abort(b"Could not open OpenType font: %s\x00" as *const u8 as
                      *const libc::c_char, ident);
    }
    sfont = sfnt_open(handle as rust_input_handle_t);
    if sfont.is_null() {
        _tt_abort(b"Could not open OpenType font: %s\x00" as *const u8 as
                      *const libc::c_char, ident);
    }
    if sfnt_read_table_directory(sfont, 0i32 as SFNT_ULONG) < 0i32 {
        _tt_abort(b"Could not read OpenType table directory: %s\x00" as
                      *const u8 as *const libc::c_char, ident);
    }
    if (*sfont).type_0 != 1i32 << 2i32 ||
           {
               offset =
                   sfnt_find_table_pos(sfont,
                                       b"CFF \x00" as *const u8 as
                                           *const libc::c_char) as
                       libc::c_int;
               offset == 0i32
           } {
        _tt_abort(b"Not a CFF/OpenType font (11)?\x00" as *const u8 as
                      *const libc::c_char);
    }
    cffont = cff_open(handle as rust_input_handle_t, offset, 0i32);
    if cffont.is_null() {
        _tt_abort(b"Could not open CFF font.\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*cffont).flag & 1i32 << 0i32 != 0 {
        _tt_abort(b"This is CIDFont...\x00" as *const u8 as
                      *const libc::c_char);
    }
    fullname =
        new((strlen(fontname).wrapping_add(8i32 as libc::c_ulong) as uint32_t
                 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_char;
    sprintf(fullname, b"%6s+%s\x00" as *const u8 as *const libc::c_char,
            uniqueTag, fontname);
    /* Offsets from DICTs */
    cff_read_charsets(cffont);
    if encoding_id < 0i32 { cff_read_encoding(cffont); }
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    /* FIXME */
    (*cffont)._string = cff_new_index(0i32 as card16);
    /* New Charsets data */
    charset =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_charsets>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = 0i32 as card16;
    (*charset).data.glyphs =
        new((256i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<s_SID>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut s_SID;
    /*
     * Encoding related things.
     */
    enc_vec = 0 as *mut *mut libc::c_char;
    if encoding_id >= 0i32 {
        enc_vec = pdf_encoding_get_encoding(encoding_id)
    } else {
        let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
        /*
         * Create enc_vec and ToUnicode CMap for built-in encoding.
         */
        enc_vec =
            new((256i32 as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut *mut libc::c_char;
        code = 0i32 as card16;
        while (code as libc::c_int) < 256i32 {
            if *usedchars.offset(code as isize) != 0 {
                let mut gid: card16 = 0;
                gid = cff_encoding_lookup(cffont, code as card8);
                let ref mut fresh0 = *enc_vec.offset(code as isize);
                *fresh0 =
                    cff_get_string(cffont,
                                   cff_charsets_lookup_inverse(cffont, gid))
            } else {
                let ref mut fresh1 = *enc_vec.offset(code as isize);
                *fresh1 = 0 as *mut libc::c_char
            }
            code = code.wrapping_add(1)
        }
        if pdf_lookup_dict(fontdict,
                           b"ToUnicode\x00" as *const u8 as
                               *const libc::c_char).is_null() {
            tounicode =
                pdf_create_ToUnicode_CMap(fullname, enc_vec, usedchars);
            if !tounicode.is_null() {
                pdf_add_dict(fontdict,
                             pdf_new_name(b"ToUnicode\x00" as *const u8 as
                                              *const libc::c_char),
                             pdf_ref_obj(tounicode));
                pdf_release_obj(tounicode);
            }
        }
    }
    /*
     * New Encoding data:
     *
     *  We should not use format 0 here.
     *  The number of encoded glyphs (num_entries) is limited to 255 in format 0,
     *  and hence it causes problem for encodings that uses full 256 code-points.
     *  As we always sort glyphs by encoding, we can avoid this problem simply
     *  by using format 1; Using full range result in a single range, 0 255.
     *
     *  Creating actual encoding date is delayed to eliminate character codes to
     *  be mapped to .notdef and to handle multiply-encoded glyphs.
     */
    encoding =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_encoding>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_encoding;
    (*encoding).format = 1i32 as card8;
    (*encoding).num_entries = 0i32 as card8;
    (*encoding).data.range1 =
        new((255i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_range1>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_range1;
    (*encoding).num_supps = 0i32 as card8;
    (*encoding).supp =
        new((255i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_map>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_map;
    /*
     * Charastrings.
     */
    offset =
        cff_dict_get((*cffont).topdict,
                     b"CharStrings\x00" as *const u8 as *const libc::c_char,
                     0i32) as libc::c_int;
    ttstub_input_seek((*cffont).handle,
                      (*cffont).offset.wrapping_add(offset as libc::c_uint) as
                          ssize_t, 0i32);
    cs_idx = cff_get_index_header(cffont);
    /* Offset is now absolute offset ... fixme */
    offset =
        ttstub_input_seek((*cffont).handle, 0i32 as ssize_t, 1i32) as
            libc::c_int;
    cs_count = (*cs_idx).count;
    if (cs_count as libc::c_int) < 2i32 {
        _tt_abort(b"No valid charstring data found.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* New CharStrings INDEX */
    charstrings =
        cff_new_index(257i32 as card16); /* 256 + 1 for ".notdef" glyph */
    max_len = 2i32 * 65536i32;
    (*charstrings).data =
        new((max_len as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    charstring_len = 0i32;
    /*
     * Information from OpenType table is rough estimate. Replace with accurate value.
     */
    if !(*(*cffont).private.offset(0)).is_null() &&
           cff_dict_known(*(*cffont).private.offset(0),
                          b"StdVW\x00" as *const u8 as *const libc::c_char) !=
               0 {
        let mut stemv: libc::c_double = 0.;
        stemv =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"StdVW\x00" as *const u8 as *const libc::c_char,
                         0i32);
        pdf_add_dict(descriptor,
                     pdf_new_name(b"StemV\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(stemv));
    }
    /*
     * Widths
     */
    if !(*(*cffont).private.offset(0)).is_null() &&
           cff_dict_known(*(*cffont).private.offset(0),
                          b"defaultWidthX\x00" as *const u8 as
                              *const libc::c_char) != 0 {
        default_width =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"defaultWidthX\x00" as *const u8 as
                             *const libc::c_char, 0i32)
    } else { default_width = 0.0f64 }
    if !(*(*cffont).private.offset(0)).is_null() &&
           cff_dict_known(*(*cffont).private.offset(0),
                          b"nominalWidthX\x00" as *const u8 as
                              *const libc::c_char) != 0 {
        nominal_width =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"nominalWidthX\x00" as *const u8 as
                             *const libc::c_char, 0i32)
    } else { nominal_width = 0.0f64 }
    data =
        new((65536i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    /* First we add .notdef glyph.
     * All Type 1 font requires .notdef glyph to be present.
     */
    if verbose > 2i32 {
        dpx_message(b"[glyphs:/.notdef\x00" as *const u8 as
                        *const libc::c_char);
    }
    size =
        (*(*cs_idx).offset.offset(1)).wrapping_sub(*(*cs_idx).offset.offset(0))
            as libc::c_int;
    if size > 65536i32 {
        _tt_abort(b"Charstring too long: gid=%u, %d bytes\x00" as *const u8 as
                      *const libc::c_char, 0i32, size);
    }
    *(*charstrings).offset.offset(0) = (charstring_len + 1i32) as l_offset;
    ttstub_input_seek((*cffont).handle,
                      (offset as
                           libc::c_uint).wrapping_add(*(*cs_idx).offset.offset(0)).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint)
                          as ssize_t, 0i32);
    ttstub_input_read((*cffont).handle, data as *mut libc::c_char,
                      size as size_t);
    charstring_len +=
        cs_copy_charstring((*charstrings).data.offset(charstring_len as
                                                          isize),
                           max_len - charstring_len, data, size,
                           (*cffont).gsubr, *(*cffont).subrs.offset(0),
                           default_width, nominal_width, &mut ginfo);
    notdef_width = ginfo.wx;
    /*
     * Subset font
     */
    num_glyphs = 1i32 as card16;
    pdfcharset = pdf_new_stream(0i32);
    code = 0i32 as card16;
    while (code as libc::c_int) < 256i32 {
        let mut gid_0: card16 = 0;
        let mut j: card16 = 0;
        let mut sid_orig: s_SID = 0;
        let mut sid: s_SID = 0;
        widths[code as usize] = notdef_width;
        if !(*usedchars.offset(code as isize) == 0 ||
                 (*enc_vec.offset(code as isize)).is_null() ||
                 streq_ptr(*enc_vec.offset(code as isize),
                           b".notdef\x00" as *const u8 as *const libc::c_char)
                     as libc::c_int != 0) {
            /*
         * FIXME:
         *  cff_get_sid() obtain SID from original String INDEX.
         *  It should be cff_string_get_sid(string, ...).
         *  cff_add_string(cff, ...) -> cff_string_add(string, ...).
         */
            sid_orig =
                cff_get_sid(cffont, *enc_vec.offset(code as isize)) as s_SID;
            sid =
                (if (sid_orig as libc::c_int) < 391i32 {
                     sid_orig as libc::c_int
                 } else {
                     cff_add_string(cffont, *enc_vec.offset(code as isize),
                                    0i32) as libc::c_int
                 }) as s_SID;
            /*
         * We use "unique = 0" because duplicate strings are impossible
         * at this stage unless the original font already had duplicates.
         */
            /*
         * Check if multiply-encoded glyph.
         */
            j = 0i32 as card16;
            while (j as libc::c_int) < (*charset).num_entries as libc::c_int {
                if sid as libc::c_int ==
                       *(*charset).data.glyphs.offset(j as isize) as
                           libc::c_int {
                    /* Already have this glyph. */
                    (*(*encoding).supp.offset((*encoding).num_supps as
                                                  isize)).code =
                        code as card8; /* Used but multiply-encoded. */
                    (*(*encoding).supp.offset((*encoding).num_supps as
                                                  isize)).glyph = sid;
                    *usedchars.offset(code as isize) = 0i32 as libc::c_char;
                    (*encoding).num_supps =
                        ((*encoding).num_supps as libc::c_int + 1i32) as
                            card8;
                    break ;
                } else { j = j.wrapping_add(1) }
            }
            if !((j as libc::c_int) < (*charset).num_entries as libc::c_int) {
                /* This is new encoding entry. */
                gid_0 = cff_charsets_lookup(cffont, sid_orig); /* FIXME */
                if gid_0 as libc::c_int == 0i32 {
                    dpx_warning(b"Glyph \"%s\" missing in font \"%s\".\x00" as
                                    *const u8 as *const libc::c_char,
                                *enc_vec.offset(code as isize),
                                fontname); /* Set unused for writing correct encoding */
                    dpx_warning(b"Maybe incorrect encoding specified.\x00" as
                                    *const u8 as *const libc::c_char);
                    *usedchars.offset(code as isize) = 0i32 as libc::c_char
                } else {
                    pdf_add_stream(pdfcharset,
                                   b"/\x00" as *const u8 as
                                       *const libc::c_char as
                                       *const libc::c_void, 1i32);
                    pdf_add_stream(pdfcharset,
                                   *enc_vec.offset(code as isize) as
                                       *const libc::c_void,
                                   strlen(*enc_vec.offset(code as isize)) as
                                       libc::c_int);
                    if verbose > 2i32 {
                        dpx_message(b"/%s\x00" as *const u8 as
                                        *const libc::c_char,
                                    *enc_vec.offset(code as isize));
                    }
                    size =
                        (*(*cs_idx).offset.offset((gid_0 as libc::c_int +
                                                       1i32) as
                                                      isize)).wrapping_sub(*(*cs_idx).offset.offset(gid_0
                                                                                                        as
                                                                                                        isize))
                            as libc::c_int;
                    if size > 65536i32 {
                        _tt_abort(b"Charstring too long: gid=%u, %d bytes\x00"
                                      as *const u8 as *const libc::c_char,
                                  gid_0 as libc::c_int, size);
                    }
                    if charstring_len + 65536i32 >= max_len {
                        max_len = charstring_len + 2i32 * 65536i32;
                        (*charstrings).data =
                            renew((*charstrings).data as *mut libc::c_void,
                                  (max_len as uint32_t as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>()
                                                                       as
                                                                       libc::c_ulong)
                                      as uint32_t) as *mut card8
                    }
                    *(*charstrings).offset.offset(num_glyphs as isize) =
                        (charstring_len + 1i32) as l_offset;
                    ttstub_input_seek((*cffont).handle,
                                      (offset as
                                           libc::c_uint).wrapping_add(*(*cs_idx).offset.offset(gid_0
                                                                                                   as
                                                                                                   isize)).wrapping_sub(1i32
                                                                                                                            as
                                                                                                                            libc::c_uint)
                                          as ssize_t, 0i32);
                    ttstub_input_read((*cffont).handle,
                                      data as *mut libc::c_char,
                                      size as size_t);
                    charstring_len +=
                        cs_copy_charstring((*charstrings).data.offset(charstring_len
                                                                          as
                                                                          isize),
                                           max_len - charstring_len, data,
                                           size, (*cffont).gsubr,
                                           *(*cffont).subrs.offset(0),
                                           default_width, nominal_width,
                                           &mut ginfo);
                    widths[code as usize] = ginfo.wx;
                    *(*charset).data.glyphs.offset((*charset).num_entries as
                                                       isize) = sid;
                    (*charset).num_entries =
                        ((*charset).num_entries as libc::c_int + 1i32) as
                            card16;
                    num_glyphs = num_glyphs.wrapping_add(1)
                }
            }
        }
        code = code.wrapping_add(1)
        /* Prevent duplication. */
    }
    if verbose > 2i32 {
        dpx_message(b"]\x00" as *const u8 as *const libc::c_char);
    }
    free(data as *mut libc::c_void);
    /*
     * Now we create encoding data.
     */
    if (*encoding).num_supps as libc::c_int > 0i32 {
        (*encoding).format =
            ((*encoding).format as libc::c_int | 0x80i32) as card8
    } else {
        free((*encoding).supp as
                 *mut libc::c_void); /* Have supplemantary data. */
        /* FIXME */
    }
    code = 0i32 as card16;
    while (code as libc::c_int) < 256i32 {
        if !(*usedchars.offset(code as isize) == 0 ||
                 (*enc_vec.offset(code as isize)).is_null() ||
                 streq_ptr(*enc_vec.offset(code as isize),
                           b".notdef\x00" as *const u8 as *const libc::c_char)
                     as libc::c_int != 0) {
            (*(*encoding).data.range1.offset((*encoding).num_entries as
                                                 isize)).first = code;
            (*(*encoding).data.range1.offset((*encoding).num_entries as
                                                 isize)).n_left =
                0i32 as card8;
            code = code.wrapping_add(1);
            while (code as libc::c_int) < 256i32 &&
                      *usedchars.offset(code as isize) as libc::c_int != 0 &&
                      !(*enc_vec.offset(code as isize)).is_null() &&
                      strcmp(*enc_vec.offset(code as isize),
                             b".notdef\x00" as *const u8 as
                                 *const libc::c_char) != 0 {
                let ref mut fresh2 =
                    (*(*encoding).data.range1.offset((*encoding).num_entries
                                                         as isize)).n_left;
                *fresh2 = (*fresh2 as libc::c_int + 1i32) as card8;
                code = code.wrapping_add(1)
            }
            (*encoding).num_entries =
                ((*encoding).num_entries as libc::c_int + 1i32) as card8
        }
        code = code.wrapping_add(1)
        /* The above while() loop stopped at unused char or code == 256. */
    }
    /* cleanup */
    if encoding_id < 0i32 && !enc_vec.is_null() {
        code = 0i32 as card16;
        while (code as libc::c_int) < 256i32 {
            if !(*enc_vec.offset(code as isize)).is_null() {
                free(*enc_vec.offset(code as isize) as *mut libc::c_void);
            }
            code = code.wrapping_add(1)
        }
        free(enc_vec as *mut libc::c_void);
    }
    cff_release_index(cs_idx);
    *(*charstrings).offset.offset(num_glyphs as isize) =
        (charstring_len + 1i32) as l_offset;
    (*charstrings).count = num_glyphs;
    charstring_len = cff_index_size(charstrings);
    (*cffont).num_glyphs = num_glyphs;
    /*
     * Discard old one, set new data.
     */
    if !(*cffont).charsets.is_null() {
        cff_release_charsets((*cffont).charsets);
    }
    (*cffont).charsets = charset;
    if !(*cffont).encoding.is_null() {
        cff_release_encoding((*cffont).encoding);
    }
    (*cffont).encoding = encoding;
    /*
     * We don't use subroutines at all.
     */
    if !(*cffont).gsubr.is_null() { cff_release_index((*cffont).gsubr); }
    (*cffont).gsubr = cff_new_index(0i32 as card16);
    if !(*(*cffont).subrs.offset(0)).is_null() {
        cff_release_index(*(*cffont).subrs.offset(0));
    }
    let ref mut fresh3 = *(*cffont).subrs.offset(0);
    *fresh3 = 0 as *mut cff_index;
    /*
     * Flag must be reset since cff_pack_encoding(charset) does not write
     * encoding(charset) if HAVE_STANDARD_ENCODING(CHARSET) is set. We are
     * re-encoding font.
     */
    (*cffont).flag = 1i32 << 1i32;
    /*
     * FIXME:
     *  Update String INDEX to delete unused strings.
     */
    cff_dict_update((*cffont).topdict, cffont);
    if !(*(*cffont).private.offset(0)).is_null() {
        cff_dict_update(*(*cffont).private.offset(0), cffont);
    }
    cff_update_string(cffont);
    /*
     * Calculate sizes of Top DICT and Private DICT.
     * All offset values in DICT are set to long (32-bit) integer
     * in cff_dict_pack(), those values are updated later.
     */
    topdict = cff_new_index(1i32 as card16);
    cff_dict_remove((*cffont).topdict,
                    b"UniqueID\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"XUID\x00" as *const u8 as *const libc::c_char);
    /*
     * Force existence of Encoding.
     */
    if cff_dict_known((*cffont).topdict,
                      b"Encoding\x00" as *const u8 as *const libc::c_char) ==
           0 {
        cff_dict_add((*cffont).topdict,
                     b"Encoding\x00" as *const u8 as *const libc::c_char,
                     1i32); /* no Subrs */
    }
    *(*topdict).offset.offset(1) =
        (cff_dict_pack((*cffont).topdict,
                       work_buffer.as_mut_ptr() as *mut card8, 1024i32) +
             1i32) as l_offset;
    private_size = 0i32;
    if !(*(*cffont).private.offset(0)).is_null() {
        cff_dict_remove(*(*cffont).private.offset(0),
                        b"Subrs\x00" as *const u8 as *const libc::c_char);
        private_size =
            cff_dict_pack(*(*cffont).private.offset(0),
                          work_buffer.as_mut_ptr() as *mut card8, 1024i32)
    }
    /*
     * Estimate total size of fontfile.
     */
    stream_data_len = 4i32; /* header size */
    stream_data_len += cff_set_name(cffont, fullname);
    free(fullname as *mut libc::c_void);
    stream_data_len += cff_index_size(topdict);
    stream_data_len += cff_index_size((*cffont).string);
    stream_data_len += cff_index_size((*cffont).gsubr);
    /* We are using format 1 for Encoding and format 0 for charset.
     * TODO: Should implement cff_xxx_size().
     */
    stream_data_len +=
        2i32 + (*encoding).num_entries as libc::c_int * 2i32 + 1i32 +
            (*encoding).num_supps as libc::c_int * 3i32;
    stream_data_len += 1i32 + (*charset).num_entries as libc::c_int * 2i32;
    stream_data_len += charstring_len;
    stream_data_len += private_size;
    /*
     * Now we create FontFile data.
     */
    stream_data_ptr =
        new((stream_data_len as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    /*
     * Data Layout order as described in CFF spec., sec 2 "Data Layout".
     */
    offset = 0i32;
    /* Header */
    offset +=
        cff_put_header(cffont, stream_data_ptr.offset(offset as isize),
                       stream_data_len - offset);
    /* Name */
    offset +=
        cff_pack_index((*cffont).name,
                       stream_data_ptr.offset(offset as isize),
                       stream_data_len - offset);
    /* Top DICT */
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    /* Strings */
    offset +=
        cff_pack_index((*cffont).string,
                       stream_data_ptr.offset(offset as isize),
                       stream_data_len - offset);
    /* Global Subrs */
    offset +=
        cff_pack_index((*cffont).gsubr,
                       stream_data_ptr.offset(offset as isize),
                       stream_data_len - offset);
    /* Encoding */
    cff_dict_set((*cffont).topdict,
                 b"Encoding\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as libc::c_double);
    offset +=
        cff_pack_encoding(cffont, stream_data_ptr.offset(offset as isize),
                          stream_data_len - offset);
    /* charset */
    cff_dict_set((*cffont).topdict,
                 b"charset\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as libc::c_double);
    offset +=
        cff_pack_charsets(cffont, stream_data_ptr.offset(offset as isize),
                          stream_data_len - offset);
    /* CharStrings */
    cff_dict_set((*cffont).topdict,
                 b"CharStrings\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as libc::c_double);
    offset +=
        cff_pack_index(charstrings, stream_data_ptr.offset(offset as isize),
                       charstring_len);
    cff_release_index(charstrings);
    /* Private */
    cff_dict_set((*cffont).topdict,
                 b"Private\x00" as *const u8 as *const libc::c_char, 1i32,
                 offset as libc::c_double);
    if !(*(*cffont).private.offset(0)).is_null() && private_size > 0i32 {
        private_size =
            cff_dict_pack(*(*cffont).private.offset(0),
                          stream_data_ptr.offset(offset as isize),
                          private_size)
    }
    cff_dict_set((*cffont).topdict,
                 b"Private\x00" as *const u8 as *const libc::c_char, 0i32,
                 private_size as libc::c_double);
    offset += private_size;
    /* Finally Top DICT */
    (*topdict).data =
        new(((*(*topdict).offset.offset(1)).wrapping_sub(1i32 as libc::c_uint)
                 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    cff_dict_pack((*cffont).topdict, (*topdict).data,
                  (*(*topdict).offset.offset(1)).wrapping_sub(1i32 as
                                                                  libc::c_uint)
                      as libc::c_int);
    cff_pack_index(topdict, stream_data_ptr.offset(topdict_offset as isize),
                   cff_index_size(topdict));
    cff_release_index(topdict);
    /* Copyright and Trademark Notice ommited. */
    /* Handle Widths in fontdict. */
    add_SimpleMetrics(font, cffont, widths.as_mut_ptr(), num_glyphs);
    /* Close font */
    cff_close(cffont);
    sfnt_close(sfont);
    ttstub_input_close(handle as rust_input_handle_t);
    if verbose > 1i32 {
        dpx_message(b"[%u/%u glyphs][%d bytes]\x00" as *const u8 as
                        *const libc::c_char, num_glyphs as libc::c_int,
                    cs_count as libc::c_int, offset);
    }
    /*
     * CharSet
     */
    pdf_add_dict(descriptor,
                 pdf_new_name(b"CharSet\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_string(pdf_stream_dataptr(pdfcharset),
                                pdf_stream_length(pdfcharset) as size_t));
    pdf_release_obj(pdfcharset);
    /*
     * Write PDF FontFile data.
     */
    fontfile = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(descriptor,
                 pdf_new_name(b"FontFile3\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_ref_obj(fontfile));
    pdf_add_dict(stream_dict,
                 pdf_new_name(b"Subtype\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"Type1C\x00" as *const u8 as
                                  *const libc::c_char));
    pdf_add_stream(fontfile, stream_data_ptr as *mut libc::c_void, offset);
    pdf_release_obj(fontfile);
    free(stream_data_ptr as *mut libc::c_void);
    return 0i32;
}
