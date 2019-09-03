#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value)]
extern crate libc;
extern "C" {
    pub type pdf_obj;
    pub type Type0Font;
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
    fn Type0Font_set_ToUnicode(font: *mut Type0Font, cmap_ref: *mut pdf_obj);
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    static mut CSI_IDENTITY: CIDSysInfo;
    #[no_mangle]
    static mut CSI_UNICODE: CIDSysInfo;
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
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(stream: *mut pdf_obj,
                      stream_data_ptr: *const libc::c_void,
                      stream_data_len: libc::c_int);
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn Type0Font_get_usedchars(font: *mut Type0Font) -> *mut libc::c_char;
    /* FIXME */
    #[no_mangle]
    fn CIDFont_get_opt_index(font: *mut CIDFont) -> libc::c_int;
    #[no_mangle]
    fn CIDFont_get_embedding(font: *mut CIDFont) -> libc::c_int;
    #[no_mangle]
    fn CIDFont_get_parent_id(font: *mut CIDFont, wmode: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn CIDFont_is_BaseFont(font: *mut CIDFont) -> bool;
    #[no_mangle]
    fn Type0Font_cache_get(id: libc::c_int) -> *mut Type0Font;
    #[no_mangle]
    fn agl_chop_suffix(glyphname: *const libc::c_char,
                       suffix: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn agl_sput_UTF16BE(name: *const libc::c_char,
                        dstpp: *mut *mut libc::c_uchar,
                        limptr: *mut libc::c_uchar,
                        num_fails: *mut libc::c_int) -> int32_t;
    #[no_mangle]
    fn agl_name_is_unicode(glyphname: *const libc::c_char) -> bool;
    #[no_mangle]
    fn agl_name_convert_unicode(glyphname: *const libc::c_char) -> int32_t;
    #[no_mangle]
    fn agl_lookup_list(glyphname: *const libc::c_char) -> *mut agl_name;
    #[no_mangle]
    fn cff_open(handle: rust_input_handle_t, offset: libc::c_int,
                idx: libc::c_int) -> *mut cff_font;
    #[no_mangle]
    fn cff_close(cff: *mut cff_font);
    /* CFF Header */
    #[no_mangle]
    fn cff_put_header(cff: *mut cff_font, dest: *mut card8,
                      destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_get_index_header(cff: *mut cff_font) -> *mut cff_index;
    #[no_mangle]
    fn cff_release_index(idx: *mut cff_index);
    #[no_mangle]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[no_mangle]
    fn cff_index_size(idx: *mut cff_index) -> libc::c_int;
    #[no_mangle]
    fn cff_pack_index(idx: *mut cff_index, dest: *mut card8,
                      destlen: libc::c_int) -> libc::c_int;
    /* Name INDEX */
    #[no_mangle]
    fn cff_get_name(cff: *mut cff_font) -> *mut libc::c_char;
    #[no_mangle]
    fn cff_set_name(cff: *mut cff_font, name: *mut libc::c_char)
     -> libc::c_int;
    /* Global and Local Subrs INDEX */
    #[no_mangle]
    fn cff_read_subrs(cff: *mut cff_font) -> libc::c_int;
    /* Charsets */
    /* Returns GID of PS name "glyph" */
    /* Return PS name of "gid" */
    /* Returns GID of glyph with SID/CID "cid" */
    /* Returns SID or CID */
    /* FDSelect */
    /* Font DICT(s) */
    /* Private DICT(s) */
    /* String */
    #[no_mangle]
    fn cff_add_string(cff: *mut cff_font, str: *const libc::c_char,
                      unique: libc::c_int) -> s_SID;
    #[no_mangle]
    fn cff_update_string(cff: *mut cff_font);
    #[no_mangle]
    fn cff_get_sid(cff: *mut cff_font, str: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn cff_charsets_lookup_inverse(cff: *mut cff_font, gid: card16) -> card16;
    #[no_mangle]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut libc::c_char;
    #[no_mangle]
    fn cff_read_charsets(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_read_fdselect(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_glyph_lookup(cff: *mut cff_font, glyph: *const libc::c_char)
     -> card16;
    #[no_mangle]
    fn cff_pack_fdselect(cff: *mut cff_font, dest: *mut card8,
                         destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cff_pack_charsets(cff: *mut cff_font, dest: *mut card8,
                         destlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut work_buffer: [libc::c_char; 0];
    #[no_mangle]
    fn cff_release_fdselect(fdselect: *mut cff_fdselect);
    #[no_mangle]
    fn cff_release_charsets(charset: *mut cff_charsets);
    #[no_mangle]
    fn cff_fdselect_lookup(cff: *mut cff_font, gid: card16) -> card8;
    #[no_mangle]
    fn cff_read_private(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_read_fdarray(cff: *mut cff_font) -> libc::c_int;
    #[no_mangle]
    fn cff_charsets_lookup(cff: *mut cff_font, cid: card16) -> card16;
    #[no_mangle]
    fn cff_new_dict() -> *mut cff_dict;
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
    fn CMap_new() -> *mut CMap;
    #[no_mangle]
    fn CMap_release(cmap: *mut CMap);
    #[no_mangle]
    fn CMap_set_name(cmap: *mut CMap, name: *const libc::c_char);
    #[no_mangle]
    fn CMap_set_type(cmap: *mut CMap, type_0: libc::c_int);
    #[no_mangle]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: libc::c_int);
    #[no_mangle]
    fn CMap_set_CIDSysInfo(cmap: *mut CMap, csi: *const CIDSysInfo);
    /* charName not supported */
    #[no_mangle]
    fn CMap_add_bfchar(cmap: *mut CMap, src: *const libc::c_uchar,
                       srcdim: size_t, dest: *const libc::c_uchar,
                       destdim: size_t) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_cidchar(cmap: *mut CMap, src: *const libc::c_uchar,
                        srcdim: size_t, dest: CID) -> libc::c_int;
    #[no_mangle]
    fn CMap_add_codespacerange(cmap: *mut CMap, codelo: *const libc::c_uchar,
                               codehi: *const libc::c_uchar, dim: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn CMap_cache_add(cmap: *mut CMap) -> libc::c_int;
    #[no_mangle]
    fn CMap_cache_find(cmap_name: *const libc::c_char) -> libc::c_int;
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
    fn CMap_create_stream(cmap: *mut CMap) -> *mut pdf_obj;
    #[no_mangle]
    fn cs_copy_charstring(dest: *mut card8, destlen: libc::c_int,
                          src: *mut card8, srclen: libc::c_int,
                          gsubr: *mut cff_index, subr: *mut cff_index,
                          default_width: libc::c_double,
                          nominal_width: libc::c_double, ginfo: *mut cs_ginfo)
     -> libc::c_int;
    #[no_mangle]
    fn dpx_open_type1_file(filename: *const libc::c_char)
     -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_truetype_file(filename: *const libc::c_char)
     -> rust_input_handle_t;
    #[no_mangle]
    fn dpx_open_opentype_file(filename: *const libc::c_char)
     -> rust_input_handle_t;
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
    fn pdf_font_make_uniqueTag(tag: *mut libc::c_char);
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
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const libc::c_char)
     -> SFNT_ULONG;
    #[no_mangle]
    fn t1char_get_metrics(src: *mut card8, srclen: libc::c_int,
                          subrs: *mut cff_index, ginfo: *mut t1_ginfo)
     -> libc::c_int;
    #[no_mangle]
    fn t1char_convert_charstring(dst: *mut card8, dstlen: libc::c_int,
                                 src: *mut card8, srclen: libc::c_int,
                                 subrs: *mut cff_index,
                                 default_width: libc::c_double,
                                 nominal_width: libc::c_double,
                                 ginfo: *mut t1_ginfo) -> libc::c_int;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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
    fn t1_load_font(enc_vec: *mut *mut libc::c_char, mode: libc::c_int,
                    handle: rust_input_handle_t) -> *mut cff_font;
    /* TTC (TrueType Collection) */
    #[no_mangle]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: libc::c_int) -> SFNT_ULONG;
    /* FontDescriptor */
    #[no_mangle]
    fn tt_get_fontdesc(sfont: *mut sfnt, embed: *mut libc::c_int,
                       stemv: libc::c_int, type_0: libc::c_int,
                       fontname: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[no_mangle]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[no_mangle]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    /* vhea */
    #[no_mangle]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    /* VORG */
    #[no_mangle]
    fn tt_read_VORG_table(sfont: *mut sfnt) -> *mut tt_VORG_table;
    /* hmtx and vmtx */
    #[no_mangle]
    fn tt_read_longMetrics(sfont: *mut sfnt, numGlyphs: USHORT,
                           numLongMetrics: USHORT, numExSideBearings: USHORT)
     -> *mut tt_longMetrics;
    /* OS/2 table */
    #[no_mangle]
    fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
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
/* CIDFont types */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut libc::c_char,
    pub ordering: *mut libc::c_char,
    pub supplement: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CIDFont {
    pub ident: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub fontname: *mut libc::c_char,
    pub subtype: libc::c_int,
    pub flags: libc::c_int,
    pub parent: [libc::c_int; 2],
    pub csi: *mut CIDSysInfo,
    pub options: *mut cid_opt,
    pub indirect: *mut pdf_obj,
    pub fontdict: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cid_opt {
    pub name: *mut libc::c_char,
    pub csi: *mut CIDSysInfo,
    pub index: libc::c_int,
    pub style: libc::c_int,
    pub embed: libc::c_int,
    pub stemv: libc::c_int,
    pub cff_charsets: *mut libc::c_void,
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
pub struct cs_ginfo {
    pub flags: libc::c_int,
    pub wx: libc::c_double,
    pub wy: libc::c_double,
    pub bbox: C2RustUnnamed_0,
    pub seac: C2RustUnnamed,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub asb: libc::c_double,
    pub adx: libc::c_double,
    pub ady: libc::c_double,
    pub bchar: card8,
    pub achar: card8,
}
pub type card8 = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
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
pub type card16 = libc::c_ushort;
pub type s_SID = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
pub type l_offset = uint32_t;
pub type c_offsize = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_2 {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
/* hmtx and vmtx */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: USHORT,
    pub sideBearing: SHORT,
}
pub type SHORT = libc::c_short;
#[derive ( Copy , Clone )]
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
    /* extra information */
}
pub type Fixed = uint32_t;
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
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed_3,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
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
pub struct CIDType0Info {
    pub handle: rust_input_handle_t,
    pub sfont: *mut sfnt,
    pub cffont: *mut cff_font,
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
/* 16.16-bit signed fixed-point number */
pub type FWord = libc::c_short;
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
/* Acoid conflict with CHAR ... from <winnt.h>.  */
/* Data Types as described in Apple's TTRefMan */
pub type BYTE = libc::c_uchar;
#[derive ( Copy , Clone )]
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
#[derive ( Copy , Clone )]
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
    /* extra information */
}
pub type uFWord = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tt_VORG_table {
    pub defaultVertOriginY: SHORT,
    pub numVertOriginYMetrics: USHORT,
    pub vertOriginYMetrics: *mut tt_vertOriginYMetrics,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tt_vertOriginYMetrics {
    pub glyphIndex: USHORT,
    pub vertOriginY: SHORT,
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
pub type SFNT_CHAR = libc::c_schar;
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
pub type CID = libc::c_ushort;
pub type CIDType0Error = libc::c_int;
pub const CID_OPEN_ERROR_IS_CIDFONT: CIDType0Error = -6;
pub const CID_OPEN_ERROR_NOT_CIDFONT: CIDType0Error = -5;
pub const CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT: CIDType0Error = -4;
pub const CID_OPEN_ERROR_NO_CFF_TABLE: CIDType0Error = -3;
pub const CID_OPEN_ERROR_NOT_SFNT_FONT: CIDType0Error = -2;
pub const CID_OPEN_ERROR_CANNOT_OPEN_FILE: CIDType0Error = -1;
pub const CID_OPEN_ERROR_NO_ERROR: CIDType0Error = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CMap {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub wmode: libc::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_5,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: libc::c_int,
    pub profile: C2RustUnnamed_4,
    pub reverseMap: *mut libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mapData {
    pub data: *mut libc::c_uchar,
    pub prev: *mut mapData,
    pub pos: libc::c_int,
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
/* Mapping types, MAP_IS_NAME is not supported. */
/* Lookup flags */
/* DEBUG */
/* Codespacerange */
/* Dimension of this codespacerange */
/* Lower bounds of valid input code */
/* Upper bounds of valid input code */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mapDef {
    pub flag: libc::c_int,
    pub len: size_t,
    pub code: *mut libc::c_uchar,
    pub next: *mut mapDef,
    /* Next Subtbl for LOOKUP_CONTINUE */
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
    pub ranges: *mut rangeDef,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut libc::c_uchar,
    pub codeHi: *mut libc::c_uchar,
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct agl_name {
    pub name: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub n_components: libc::c_int,
    pub unicodes: [int32_t; 16],
    pub alternate: *mut agl_name,
    pub is_predef: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct t1_ginfo {
    pub use_seac: libc::c_int,
    pub wx: libc::c_double,
    pub wy: libc::c_double,
    pub bbox: C2RustUnnamed_7,
    pub seac: C2RustUnnamed_6,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub asb: libc::c_double,
    pub adx: libc::c_double,
    pub ady: libc::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
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
static mut verbose: libc::c_int = 0i32;
static mut opt_flags: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_set_flags(mut flags: libc::c_int) {
    opt_flags = flags;
}
/*
 * PDF Reference 3rd. ed., p.340, "Glyph Metrics in CID Fonts".
 */
unsafe extern "C" fn add_CIDHMetrics(mut fontdict: *mut pdf_obj,
                                     mut CIDToGIDMap: *mut libc::c_uchar,
                                     mut last_cid: libc::c_ushort,
                                     mut maxp: *mut tt_maxp_table,
                                     mut head: *mut tt_head_table,
                                     mut hmtx: *mut tt_longMetrics) {
    let mut w_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut an_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cid: libc::c_int = 0;
    let mut start: libc::c_int = 0i32;
    let mut prev: libc::c_int = 0i32;
    let mut defaultAdvanceWidth: libc::c_double = 0.;
    let mut empty: libc::c_int = 1i32;
    defaultAdvanceWidth =
        floor(1000.0f64 *
                  (*hmtx.offset(0)).advance as libc::c_int as libc::c_double /
                  (*head).unitsPerEm as libc::c_int as libc::c_double /
                  1i32 as libc::c_double + 0.5f64) * 1i32 as libc::c_double;
    /*
     * We alway use format:
     *  c [w_1 w_2 ... w_n]
     */
    w_array = pdf_new_array();
    cid = 0i32;
    while cid <= last_cid as libc::c_int {
        let mut gid: USHORT = 0;
        let mut advanceWidth: libc::c_double = 0.;
        gid =
            (if !CIDToGIDMap.is_null() {
                 (*CIDToGIDMap.offset((2i32 * cid) as isize) as libc::c_int)
                     << 8i32 |
                     *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) as
                         libc::c_int
             } else { cid }) as USHORT;
        if !(gid as libc::c_int >= (*maxp).numGlyphs as libc::c_int ||
                 cid != 0i32 && gid as libc::c_int == 0i32) {
            advanceWidth =
                floor(1000.0f64 *
                          (*hmtx.offset(gid as isize)).advance as libc::c_int
                              as libc::c_double /
                          (*head).unitsPerEm as libc::c_int as libc::c_double
                          / 1i32 as libc::c_double + 0.5f64) *
                    1i32 as libc::c_double;
            if advanceWidth == defaultAdvanceWidth {
                if !an_array.is_null() {
                    pdf_add_array(w_array,
                                  pdf_new_number(start as libc::c_double));
                    pdf_add_array(w_array, an_array);
                    an_array = 0 as *mut pdf_obj;
                    empty = 0i32
                }
            } else {
                if cid != prev + 1i32 && !an_array.is_null() {
                    pdf_add_array(w_array,
                                  pdf_new_number(start as libc::c_double));
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
        pdf_add_array(w_array, pdf_new_number(start as libc::c_double));
        pdf_add_array(w_array, an_array);
        empty = 0i32
    }
    /*
     * We always write DW for older MacOS X's preview app.
     * PDF Reference 2nd. ed, wrongly described default value of DW as 0, and
     * MacOS X's (up to 10.2.8) preview app. implements this wrong description.
     */
    pdf_add_dict(fontdict,
                 pdf_new_name(b"DW\x00" as *const u8 as *const libc::c_char),
                 pdf_new_number(defaultAdvanceWidth));
    if empty == 0 {
        pdf_add_dict(fontdict,
                     pdf_new_name(b"W\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_ref_obj(w_array));
    }
    pdf_release_obj(w_array);
}
unsafe extern "C" fn add_CIDVMetrics(mut sfont: *mut sfnt,
                                     mut fontdict: *mut pdf_obj,
                                     mut CIDToGIDMap: *mut libc::c_uchar,
                                     mut last_cid: libc::c_ushort,
                                     mut maxp: *mut tt_maxp_table,
                                     mut head: *mut tt_head_table,
                                     mut hmtx: *mut tt_longMetrics) {
    let mut w2_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut an_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cid: libc::c_int = 0;
    let mut vorg: *mut tt_VORG_table = 0 as *mut tt_VORG_table;
    let mut vhea: *mut tt_vhea_table = 0 as *mut tt_vhea_table;
    let mut vmtx: *mut tt_longMetrics = 0 as *mut tt_longMetrics;
    let mut defaultAdvanceHeight: libc::c_double = 0.;
    let mut defaultVertOriginY: libc::c_double = 0.;
    let mut empty: libc::c_int = 1i32;
    /*
     * No accurate vertical metrics can be obtained by simple way if the
     * font does not have VORG table. Only CJK fonts may have VORG.
     */
    if sfnt_find_table_pos(sfont,
                           b"VORG\x00" as *const u8 as *const libc::c_char) <=
           0i32 as libc::c_uint {
        return
    }
    vorg = tt_read_VORG_table(sfont);
    defaultVertOriginY =
        floor(1000.0f64 *
                  (*vorg).defaultVertOriginY as libc::c_int as libc::c_double
                  / (*head).unitsPerEm as libc::c_int as libc::c_double /
                  1i32 as libc::c_double + 0.5f64) * 1i32 as libc::c_double;
    if sfnt_find_table_pos(sfont,
                           b"vhea\x00" as *const u8 as *const libc::c_char) >
           0i32 as libc::c_uint {
        vhea = tt_read_vhea_table(sfont)
    }
    if !vhea.is_null() &&
           sfnt_find_table_pos(sfont,
                               b"vmtx\x00" as *const u8 as
                                   *const libc::c_char) > 0i32 as libc::c_uint
       {
        sfnt_locate_table(sfont,
                          b"vmtx\x00" as *const u8 as *const libc::c_char);
        vmtx =
            tt_read_longMetrics(sfont, (*maxp).numGlyphs,
                                (*vhea).numOfLongVerMetrics,
                                (*vhea).numOfExSideBearings)
    }
    if sfnt_find_table_pos(sfont,
                           b"OS/2\x00" as *const u8 as *const libc::c_char) <=
           0i32 as libc::c_uint {
        let mut os2: *mut tt_os2__table = 0 as *mut tt_os2__table;
        /* OpenType font must have OS/2 table. */
        os2 = tt_read_os2__table(sfont);
        defaultVertOriginY =
            floor(1000.0f64 *
                      (*os2).sTypoAscender as libc::c_int as libc::c_double /
                      (*head).unitsPerEm as libc::c_int as libc::c_double /
                      1i32 as libc::c_double + 0.5f64) *
                1i32 as libc::c_double;
        defaultAdvanceHeight =
            floor(1000.0f64 *
                      ((*os2).sTypoAscender as libc::c_int -
                           (*os2).sTypoDescender as libc::c_int) as
                          libc::c_double /
                      (*head).unitsPerEm as libc::c_int as libc::c_double /
                      1i32 as libc::c_double + 0.5f64) *
                1i32 as libc::c_double;
        free(os2 as *mut libc::c_void);
    } else {
        /* Some TrueType fonts used in Macintosh does not have OS/2 table. */
        defaultAdvanceHeight = 1000i32 as libc::c_double
    }
    w2_array = pdf_new_array();
    cid = 0i32;
    while cid <= last_cid as libc::c_int {
        let mut i: USHORT = 0;
        let mut gid: USHORT = 0;
        let mut advanceHeight: libc::c_double = 0.;
        let mut vertOriginX: libc::c_double = 0.;
        let mut vertOriginY: libc::c_double = 0.;
        gid =
            (if !CIDToGIDMap.is_null() {
                 (*CIDToGIDMap.offset((2i32 * cid) as isize) as libc::c_int)
                     << 8i32 |
                     *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) as
                         libc::c_int
             } else { cid }) as USHORT;
        if !(gid as libc::c_int >= (*maxp).numGlyphs as libc::c_int ||
                 cid != 0i32 && gid as libc::c_int == 0i32) {
            advanceHeight =
                if !vmtx.is_null() {
                    floor(1000.0f64 *
                              (*vmtx.offset(gid as isize)).advance as
                                  libc::c_int as libc::c_double /
                              (*head).unitsPerEm as libc::c_int as
                                  libc::c_double / 1i32 as libc::c_double +
                              0.5f64) * 1i32 as libc::c_double
                } else { defaultAdvanceHeight };
            vertOriginX =
                floor(1000.0f64 *
                          ((*hmtx.offset(gid as isize)).advance as libc::c_int
                               as libc::c_double * 0.5f64) /
                          (*head).unitsPerEm as libc::c_int as libc::c_double
                          / 1i32 as libc::c_double + 0.5f64) *
                    1i32 as libc::c_double;
            vertOriginY = defaultVertOriginY;
            i = 0i32 as USHORT;
            while (i as libc::c_int) <
                      (*vorg).numVertOriginYMetrics as libc::c_int &&
                      gid as libc::c_int >
                          (*(*vorg).vertOriginYMetrics.offset(i as
                                                                  isize)).glyphIndex
                              as libc::c_int {
                if gid as libc::c_int ==
                       (*(*vorg).vertOriginYMetrics.offset(i as
                                                               isize)).glyphIndex
                           as libc::c_int {
                    vertOriginY =
                        floor(1000.0f64 *
                                  (*(*vorg).vertOriginYMetrics.offset(i as
                                                                          isize)).vertOriginY
                                      as libc::c_int as libc::c_double /
                                  (*head).unitsPerEm as libc::c_int as
                                      libc::c_double / 1i32 as libc::c_double
                                  + 0.5f64) * 1i32 as libc::c_double
                }
                i = i.wrapping_add(1)
            }
            /*
         * c_first c_last w1_y v_x v_y
         * This form may hit Acrobat's implementation limit of array element size, 8192.
         * AFPL GhostScript 8.11 stops with rangecheck error with this. Maybe GS's bug?
         */
            if vertOriginY != defaultVertOriginY ||
                   advanceHeight != defaultAdvanceHeight {
                pdf_add_array(w2_array,
                              pdf_new_number(cid as libc::c_double));
                pdf_add_array(w2_array,
                              pdf_new_number(cid as libc::c_double));
                pdf_add_array(w2_array, pdf_new_number(-advanceHeight));
                pdf_add_array(w2_array, pdf_new_number(vertOriginX));
                pdf_add_array(w2_array, pdf_new_number(vertOriginY));
                empty = 0i32
            }
        }
        cid += 1
    }
    if defaultVertOriginY != 880i32 as libc::c_double ||
           defaultAdvanceHeight != 1000i32 as libc::c_double {
        an_array = pdf_new_array();
        pdf_add_array(an_array, pdf_new_number(defaultVertOriginY));
        pdf_add_array(an_array, pdf_new_number(-defaultAdvanceHeight));
        pdf_add_dict(fontdict,
                     pdf_new_name(b"DW2\x00" as *const u8 as
                                      *const libc::c_char), an_array);
    }
    if empty == 0 {
        pdf_add_dict(fontdict,
                     pdf_new_name(b"W2\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_ref_obj(w2_array));
    }
    pdf_release_obj(w2_array);
    free((*vorg).vertOriginYMetrics as *mut libc::c_void);
    free(vorg as *mut libc::c_void);
    free(vmtx as *mut libc::c_void);
    free(vhea as *mut libc::c_void);
}
unsafe extern "C" fn add_CIDMetrics(mut sfont: *mut sfnt,
                                    mut fontdict: *mut pdf_obj,
                                    mut CIDToGIDMap: *mut libc::c_uchar,
                                    mut last_cid: libc::c_ushort,
                                    mut need_vmetrics: libc::c_int) {
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
    sfnt_locate_table(sfont, b"hmtx\x00" as *const u8 as *const libc::c_char);
    hmtx =
        tt_read_longMetrics(sfont, (*maxp).numGlyphs,
                            (*hhea).numOfLongHorMetrics,
                            (*hhea).numOfExSideBearings);
    add_CIDHMetrics(fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);
    if need_vmetrics != 0 {
        add_CIDVMetrics(sfont, fontdict, CIDToGIDMap, last_cid, maxp, head,
                        hmtx);
    }
    free(hmtx as *mut libc::c_void);
    free(hhea as *mut libc::c_void);
    free(maxp as *mut libc::c_void);
    free(head as *mut libc::c_void);
}
/*
 * Create an instance of embeddable font.
 */
unsafe extern "C" fn write_fontfile(mut font: *mut CIDFont,
                                    mut cffont: *mut cff_font)
 -> libc::c_int {
    let mut topdict: *mut cff_index = 0 as *mut cff_index;
    let mut fdarray: *mut cff_index = 0 as *mut cff_index;
    let mut private: *mut cff_index = 0 as *mut cff_index;
    let mut dest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut destlen: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut topdict_offset: libc::c_int = 0;
    let mut fdarray_offset: libc::c_int = 0;
    /*  DICT sizes (offset set to long int) */
    topdict = cff_new_index(1i32 as card16); /* some bad font may have */
    fdarray =
        cff_new_index((*cffont).num_fds as
                          card16); /* some bad font may have */
    private = cff_new_index((*cffont).num_fds as card16);
    cff_dict_remove((*cffont).topdict,
                    b"UniqueID\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"XUID\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"Private\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"Encoding\x00" as *const u8 as *const libc::c_char);
    *(*topdict).offset.offset(1) =
        (cff_dict_pack((*cffont).topdict,
                       work_buffer.as_mut_ptr() as *mut card8, 1024i32) +
             1i32) as l_offset;
    i = 0i32;
    while i < (*cffont).num_fds as libc::c_int {
        size = 0i32;
        if !(*cffont).private.is_null() &&
               !(*(*cffont).private.offset(i as isize)).is_null() {
            size =
                cff_dict_pack(*(*cffont).private.offset(i as isize),
                              work_buffer.as_mut_ptr() as *mut card8,
                              1024i32);
            if size < 1i32 {
                /* Private had contained only Subr */
                cff_dict_remove(*(*cffont).fdarray.offset(i as isize),
                                b"Private\x00" as *const u8 as
                                    *const libc::c_char); /* header size */
            }
        } /* charset format 0 */
        *(*private).offset.offset((i + 1i32) as isize) =
            (*(*private).offset.offset(i as
                                           isize)).wrapping_add(size as
                                                                    libc::c_uint); /* fdselect format 3 */
        *(*fdarray).offset.offset((i + 1i32) as isize) =
            (*(*fdarray).offset.offset(i as
                                           isize)).wrapping_add(cff_dict_pack(*(*cffont).fdarray.offset(i
                                                                                                            as
                                                                                                            isize),
                                                                              work_buffer.as_mut_ptr()
                                                                                  as
                                                                                  *mut card8,
                                                                              1024i32)
                                                                    as
                                                                    libc::c_uint); /* Private is not INDEX */
        i += 1
    }
    destlen = 4i32;
    destlen += cff_set_name(cffont, (*font).fontname);
    destlen += cff_index_size(topdict);
    destlen += cff_index_size((*cffont).string);
    destlen += cff_index_size((*cffont).gsubr);
    destlen += (*(*cffont).charsets).num_entries as libc::c_int * 2i32 + 1i32;
    destlen += (*(*cffont).fdselect).num_entries as libc::c_int * 3i32 + 5i32;
    destlen += cff_index_size((*cffont).cstrings);
    destlen += cff_index_size(fdarray);
    destlen =
        (destlen as
             libc::c_uint).wrapping_add((*(*private).offset.offset((*private).count
                                                                       as
                                                                       isize)).wrapping_sub(1i32
                                                                                                as
                                                                                                libc::c_uint))
            as libc::c_int as libc::c_int;
    dest =
        new((destlen as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    offset = 0i32;
    /* Header */
    offset +=
        cff_put_header(cffont, dest.offset(offset as isize),
                       destlen - offset);
    /* Name */
    offset +=
        cff_pack_index((*cffont).name, dest.offset(offset as isize),
                       destlen - offset);
    /* Top DICT */
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    /* Strings */
    offset +=
        cff_pack_index((*cffont).string, dest.offset(offset as isize),
                       destlen - offset);
    /* Global Subrs */
    offset +=
        cff_pack_index((*cffont).gsubr, dest.offset(offset as isize),
                       destlen - offset);
    /* charset */
    cff_dict_set((*cffont).topdict,
                 b"charset\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as libc::c_double);
    offset +=
        cff_pack_charsets(cffont, dest.offset(offset as isize),
                          destlen - offset);
    /* FDSelect */
    cff_dict_set((*cffont).topdict,
                 b"FDSelect\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as libc::c_double);
    offset +=
        cff_pack_fdselect(cffont, dest.offset(offset as isize),
                          destlen - offset);
    /* CharStrings */
    cff_dict_set((*cffont).topdict,
                 b"CharStrings\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as
                     libc::c_double); /* Charstrings cosumes huge memory */
    offset +=
        cff_pack_index((*cffont).cstrings, dest.offset(offset as isize),
                       cff_index_size((*cffont).cstrings));
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = 0 as *mut cff_index;
    /* FDArray and Private */
    cff_dict_set((*cffont).topdict,
                 b"FDArray\x00" as *const u8 as *const libc::c_char, 0i32,
                 offset as libc::c_double);
    fdarray_offset = offset;
    offset += cff_index_size(fdarray);
    (*fdarray).data =
        new(((*(*fdarray).offset.offset((*fdarray).count as
                                            isize)).wrapping_sub(1i32 as
                                                                     libc::c_uint)
                 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    i = 0i32;
    while i < (*cffont).num_fds as libc::c_int {
        size =
            (*(*private).offset.offset((i + 1i32) as
                                           isize)).wrapping_sub(*(*private).offset.offset(i
                                                                                              as
                                                                                              isize))
                as libc::c_int;
        if !(*(*cffont).private.offset(i as isize)).is_null() && size > 0i32 {
            cff_dict_pack(*(*cffont).private.offset(i as isize),
                          dest.offset(offset as isize), size);
            cff_dict_set(*(*cffont).fdarray.offset(i as isize),
                         b"Private\x00" as *const u8 as *const libc::c_char,
                         0i32, size as libc::c_double);
            cff_dict_set(*(*cffont).fdarray.offset(i as isize),
                         b"Private\x00" as *const u8 as *const libc::c_char,
                         1i32, offset as libc::c_double);
        }
        cff_dict_pack(*(*cffont).fdarray.offset(i as isize),
                      (*fdarray).data.offset(*(*fdarray).offset.offset(i as
                                                                           isize)
                                                 as isize).offset(-1),
                      (*(*fdarray).offset.offset((*fdarray).count as
                                                     isize)).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_uint)
                          as libc::c_int);
        offset += size;
        i += 1
    }
    cff_pack_index(fdarray, dest.offset(fdarray_offset as isize),
                   cff_index_size(fdarray));
    cff_release_index(fdarray);
    cff_release_index(private);
    /* Finally Top DICT */
    (*topdict).data =
        new(((*(*topdict).offset.offset((*topdict).count as
                                            isize)).wrapping_sub(1i32 as
                                                                     libc::c_uint)
                 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    cff_dict_pack((*cffont).topdict, (*topdict).data,
                  (*(*topdict).offset.offset((*topdict).count as
                                                 isize)).wrapping_sub(1i32 as
                                                                          libc::c_uint)
                      as libc::c_int);
    cff_pack_index(topdict, dest.offset(topdict_offset as isize),
                   cff_index_size(topdict));
    cff_release_index(topdict);
    /*
     * FontFile
     */
    let mut fontfile: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    fontfile = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"FontFile3\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_ref_obj(fontfile));
    pdf_add_dict(stream_dict,
                 pdf_new_name(b"Subtype\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"CIDFontType0C\x00" as *const u8 as
                                  *const libc::c_char));
    pdf_add_stream(fontfile, dest as *mut libc::c_char as *const libc::c_void,
                   offset);
    pdf_release_obj(fontfile);
    free(dest as *mut libc::c_void);
    return destlen;
}
unsafe extern "C" fn CIDFont_type0_get_used_chars(mut font: *mut CIDFont)
 -> *mut libc::c_char {
    let mut parent_id: libc::c_int = 0;
    let mut used_chars: *mut libc::c_char = 0 as *mut libc::c_char;
    parent_id = CIDFont_get_parent_id(font, 0i32);
    if parent_id < 0i32 &&
           { parent_id = CIDFont_get_parent_id(font, 1i32); parent_id < 0i32 }
       {
        _tt_abort(b"No parent Type 0 font !\x00" as *const u8 as
                      *const libc::c_char);
    }
    used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
    if used_chars.is_null() {
        _tt_abort(b"Unexpected error: Font not actually used???\x00" as
                      *const u8 as *const libc::c_char);
    }
    return used_chars;
}
unsafe extern "C" fn CIDType0Error_Show(mut error: CIDType0Error,
                                        mut name: *const libc::c_char) {
    match error as libc::c_int {
        -1 => {
            _tt_abort(b"Could not open OpenType font file: %s\x00" as
                          *const u8 as *const libc::c_char, name);
        }
        -2 => {
            _tt_abort(b"Could not open SFNT font file: %s\x00" as *const u8 as
                          *const libc::c_char, name);
        }
        -3 => {
            _tt_abort(b"Not a CFF/OpenType font: %s\x00" as *const u8 as
                          *const libc::c_char, name);
        }
        -4 => {
            _tt_abort(b"Could not open CFF font: %s\x00" as *const u8 as
                          *const libc::c_char, name);
        }
        -5 => {
            _tt_abort(b"Not a CIDFont: %s\x00" as *const u8 as
                          *const libc::c_char, name);
        }
        -6 => {
            _tt_abort(b"Should not be a CIDFont: %s\x00" as *const u8 as
                          *const libc::c_char, name);
        }
        _ => { }
    };
}
unsafe extern "C" fn CIDFontInfo_init(mut info: *mut CIDType0Info) {
    memset(info as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<CIDType0Info>() as libc::c_ulong);
}
unsafe extern "C" fn CIDFontInfo_close(mut info: *mut CIDType0Info) {
    if !(*info).cffont.is_null() { cff_close((*info).cffont); }
    if !(*info).sfont.is_null() { sfnt_close((*info).sfont); }
    if !(*info).handle.is_null() { ttstub_input_close((*info).handle); }
    CIDFontInfo_init(info);
}
unsafe extern "C" fn CIDFont_type0_try_open(mut name: *const libc::c_char,
                                            mut index: libc::c_int,
                                            mut required_cid: libc::c_int,
                                            mut info: *mut CIDType0Info)
 -> CIDType0Error {
    let mut offset: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut is_cid: libc::c_int = 0;
    CIDFontInfo_init(info);
    (*info).handle = dpx_open_opentype_file(name);
    if (*info).handle.is_null() {
        (*info).handle = dpx_open_truetype_file(name);
        if (*info).handle.is_null() { return CID_OPEN_ERROR_CANNOT_OPEN_FILE }
    }
    (*info).sfont = sfnt_open((*info).handle);
    if (*info).sfont.is_null() { return CID_OPEN_ERROR_NOT_SFNT_FONT }
    if (*(*info).sfont).type_0 == 1i32 << 4i32 {
        offset = ttc_read_offset((*info).sfont, index)
    }
    if (*(*info).sfont).type_0 != 1i32 << 4i32 &&
           (*(*info).sfont).type_0 != 1i32 << 2i32 ||
           sfnt_read_table_directory((*info).sfont, offset) < 0i32 ||
           {
               offset =
                   sfnt_find_table_pos((*info).sfont,
                                       b"CFF \x00" as *const u8 as
                                           *const libc::c_char);
               offset == 0i32 as libc::c_uint
           } {
        CIDFontInfo_close(info);
        return CID_OPEN_ERROR_NO_CFF_TABLE
    }
    (*info).cffont =
        cff_open((*(*info).sfont).handle, offset as libc::c_int, 0i32);
    if (*info).cffont.is_null() { return CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT }
    is_cid = (*(*info).cffont).flag & 1i32 << 0i32;
    if required_cid != is_cid {
        CIDFontInfo_close(info);
        return (if required_cid != 0 {
                    CID_OPEN_ERROR_NOT_CIDFONT as libc::c_int
                } else { CID_OPEN_ERROR_IS_CIDFONT as libc::c_int }) as
                   CIDType0Error
    }
    return CID_OPEN_ERROR_NO_ERROR;
}
unsafe extern "C" fn CIDFont_type0_add_CIDSet(mut font: *mut CIDFont,
                                              mut used_chars:
                                                  *mut libc::c_char,
                                              mut last_cid: card16) {
    /*
     * CIDSet:
     * Length of CIDSet stream is not clear. Must be 8192 bytes long?
     */
    let mut cidset: *mut pdf_obj = 0 as *mut pdf_obj;
    cidset = pdf_new_stream(1i32 << 0i32);
    pdf_add_stream(cidset, used_chars as *const libc::c_void,
                   last_cid as libc::c_int / 8i32 + 1i32);
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"CIDSet\x00" as *const u8 as
                                  *const libc::c_char), pdf_ref_obj(cidset));
    pdf_release_obj(cidset);
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_dofont(mut font: *mut CIDFont) {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut charstrings: *mut cff_index = 0 as *mut cff_index;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut fdselect: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut charstring_len: libc::c_int = 0;
    let mut max_len: libc::c_int = 0;
    let mut destlen: libc::c_int = 0i32;
    let mut size: libc::c_int = 0;
    let mut offset: libc::c_int = 0i32;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut num_glyphs: card16 = 0i32 as card16;
    let mut gid: card16 = 0;
    let mut cid: libc::c_int = 0;
    let mut cs_count: card16 = 0;
    let mut last_cid: card16 = 0i32 as card16;
    let mut fd: libc::c_int = 0;
    let mut prev_fd: libc::c_int = 0;
    let mut used_chars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut CIDToGIDMap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut error: CIDType0Error = CID_OPEN_ERROR_NO_ERROR;
    let mut info: CIDType0Info =
        CIDType0Info{handle: 0 as *mut libc::c_void,
                     sfont: 0 as *mut sfnt,
                     cffont: 0 as *mut cff_font,};
    if !font.is_null() {
    } else {
        __assert_fail(b"font\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cidtype0.c\x00" as *const u8 as
                          *const libc::c_char, 578i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void CIDFont_type0_dofont(CIDFont *)\x00")).as_ptr());
    }
    if (*font).indirect.is_null() { return }
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"FontDescriptor\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_ref_obj((*font).descriptor));
    if CIDFont_is_BaseFont(font) {
        return
    } else {
        if CIDFont_get_embedding(font) == 0 && opt_flags & 1i32 << 1i32 != 0 {
            /* No metrics needed. */
            pdf_add_dict((*font).fontdict,
                         pdf_new_name(b"DW\x00" as *const u8 as
                                          *const libc::c_char),
                         pdf_new_number(1000.0f64));
            return
        }
    }
    used_chars = CIDFont_type0_get_used_chars(font);
    error =
        CIDFont_type0_try_open((*font).ident, CIDFont_get_opt_index(font),
                               1i32, &mut info);
    if error as libc::c_int != CID_OPEN_ERROR_NO_ERROR as libc::c_int {
        CIDType0Error_Show(error, (*font).ident);
        return
    }
    cffont = info.cffont;
    cff_read_charsets(cffont);
    /*
     * DW, W, DW2 and W2:
     * Those values are obtained from OpenType table (not TFM).
     */
    if opt_flags & 1i32 << 1i32 != 0 {
        pdf_add_dict((*font).fontdict,
                     pdf_new_name(b"DW\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(1000.0f64));
    } else {
        let mut cid_count: libc::c_int = 0;
        if cff_dict_known((*cffont).topdict,
                          b"CIDCount\x00" as *const u8 as *const libc::c_char)
               != 0 {
            cid_count =
                cff_dict_get((*cffont).topdict,
                             b"CIDCount\x00" as *const u8 as
                                 *const libc::c_char, 0i32) as libc::c_int
        } else { cid_count = 65535i32 + 1i32 }
        CIDToGIDMap =
            new(((2i32 * cid_count) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_uchar;
        memset(CIDToGIDMap as *mut libc::c_void, 0i32,
               (2i32 * cid_count) as libc::c_ulong);
        let ref mut fresh0 = *used_chars.offset((0i32 / 8i32) as isize);
        *fresh0 =
            (*fresh0 as libc::c_int | 1i32 << 7i32 - 0i32 % 8i32) as
                libc::c_char;
        /* .notdef */
        cid = 0i32;
        while cid <= 65535i32 {
            if *used_chars.offset((cid / 8i32) as isize) as libc::c_int &
                   1i32 << 7i32 - cid % 8i32 != 0 {
                gid = cff_charsets_lookup(cffont, cid as card16);
                if cid != 0i32 && gid as libc::c_int == 0i32 {
                    dpx_warning(b"Glyph for CID %u missing in font \"%s\".\x00"
                                    as *const u8 as *const libc::c_char,
                                cid as CID as libc::c_int, (*font).ident);
                    let ref mut fresh1 =
                        *used_chars.offset((cid / 8i32) as isize);
                    *fresh1 =
                        (*fresh1 as libc::c_int &
                             !(1i32 << 7i32 - cid % 8i32)) as libc::c_char
                } else {
                    *CIDToGIDMap.offset((2i32 * cid) as isize) =
                        (gid as libc::c_int >> 8i32 & 0xffi32) as
                            libc::c_uchar;
                    *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) =
                        (gid as libc::c_int & 0xffi32) as libc::c_uchar;
                    last_cid = cid as card16;
                    num_glyphs = num_glyphs.wrapping_add(1)
                }
            }
            cid += 1
        }
        add_CIDMetrics(info.sfont, (*font).fontdict, CIDToGIDMap, last_cid,
                       if CIDFont_get_parent_id(font, 1i32) < 0i32 {
                           0i32
                       } else { 1i32 });
    }
    if CIDFont_get_embedding(font) == 0 {
        free(CIDToGIDMap as *mut libc::c_void);
        CIDFontInfo_close(&mut info);
        return
    }
    /*
     * Embed font subset.
     */
    cff_read_fdselect(cffont);
    cff_read_fdarray(cffont);
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    offset =
        cff_dict_get((*cffont).topdict,
                     b"CharStrings\x00" as *const u8 as *const libc::c_char,
                     0i32) as libc::c_int;
    ttstub_input_seek((*cffont).handle,
                      (*cffont).offset.wrapping_add(offset as libc::c_uint) as
                          ssize_t, 0i32);
    idx = cff_get_index_header(cffont);
    /* offset is now absolute offset ... bad */
    offset =
        ttstub_input_seek((*cffont).handle, 0i32 as ssize_t, 1i32) as
            libc::c_int;
    cs_count = (*idx).count;
    if (cs_count as libc::c_int) < 2i32 {
        _tt_abort(b"No valid charstring data found.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* New Charsets data */
    charset =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_charsets>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = 0i32 as card16;
    (*charset).data.glyphs =
        new((num_glyphs as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<s_SID>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut s_SID;
    /* New FDSelect data */
    fdselect =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_fdselect>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_fdselect;
    (*fdselect).format = 3i32 as card8;
    (*fdselect).num_entries = 0i32 as card16;
    (*fdselect).data.ranges =
        new((num_glyphs as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_range3>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_range3;
    /* New CharStrings INDEX */
    charstrings = cff_new_index((num_glyphs as libc::c_int + 1i32) as card16);
    max_len = 2i32 * 65536i32;
    (*charstrings).data =
        new((max_len as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    charstring_len = 0i32;
    /*
     * TODO: Re-assign FD number.
     */
    prev_fd = -1i32;
    gid = 0i32 as card16;
    data =
        new((65536i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    cid = 0i32;
    while cid <= last_cid as libc::c_int {
        let mut gid_org: libc::c_ushort = 0;
        if !(*used_chars.offset((cid / 8i32) as isize) as libc::c_int &
                 1i32 << 7i32 - cid % 8i32 == 0) {
            gid_org =
                ((*CIDToGIDMap.offset((2i32 * cid) as isize) as libc::c_int)
                     << 8i32 |
                     *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) as
                         libc::c_int) as libc::c_ushort;
            size =
                (*(*idx).offset.offset((gid_org as libc::c_int + 1i32) as
                                           isize)).wrapping_sub(*(*idx).offset.offset(gid_org
                                                                                          as
                                                                                          isize))
                    as libc::c_int;
            if size > 65536i32 {
                _tt_abort(b"Charstring too long: gid=%u\x00" as *const u8 as
                              *const libc::c_char, gid_org as libc::c_int);
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
            *(*charstrings).offset.offset(gid as isize) =
                (charstring_len + 1i32) as l_offset;
            ttstub_input_seek((*cffont).handle,
                              (offset as
                                   libc::c_uint).wrapping_add(*(*idx).offset.offset(gid_org
                                                                                        as
                                                                                        isize)).wrapping_sub(1i32
                                                                                                                 as
                                                                                                                 libc::c_uint)
                                  as ssize_t, 0i32);
            ttstub_input_read((*cffont).handle, data as *mut libc::c_char,
                              size as size_t);
            fd = cff_fdselect_lookup(cffont, gid_org) as libc::c_int;
            charstring_len +=
                cs_copy_charstring((*charstrings).data.offset(charstring_len
                                                                  as isize),
                                   max_len - charstring_len, data, size,
                                   (*cffont).gsubr,
                                   *(*cffont).subrs.offset(fd as isize),
                                   0i32 as libc::c_double,
                                   0i32 as libc::c_double,
                                   0 as *mut cs_ginfo);
            if cid > 0i32 && gid_org as libc::c_int > 0i32 {
                *(*charset).data.glyphs.offset((*charset).num_entries as
                                                   isize) = cid as s_SID;
                (*charset).num_entries =
                    ((*charset).num_entries as libc::c_int + 1i32) as card16
            }
            if fd != prev_fd {
                (*(*fdselect).data.ranges.offset((*fdselect).num_entries as
                                                     isize)).first = gid;
                (*(*fdselect).data.ranges.offset((*fdselect).num_entries as
                                                     isize)).fd = fd as card8;
                (*fdselect).num_entries =
                    ((*fdselect).num_entries as libc::c_int + 1i32) as card16;
                prev_fd = fd
            }
            gid = gid.wrapping_add(1)
        }
        cid += 1
    }
    if gid as libc::c_int != num_glyphs as libc::c_int {
        _tt_abort(b"Unexpeced error: ?????\x00" as *const u8 as
                      *const libc::c_char);
    }
    free(data as *mut libc::c_void);
    cff_release_index(idx);
    free(CIDToGIDMap as *mut libc::c_void);
    *(*charstrings).offset.offset(num_glyphs as isize) =
        (charstring_len + 1i32) as l_offset;
    (*charstrings).count = num_glyphs;
    (*cffont).num_glyphs = num_glyphs;
    (*cffont).cstrings = charstrings;
    /* discard old one, set new data */
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    cff_release_fdselect((*cffont).fdselect);
    (*cffont).fdselect = fdselect;
    /* no Global subr */
    if !(*cffont).gsubr.is_null() { cff_release_index((*cffont).gsubr); }
    (*cffont).gsubr = cff_new_index(0i32 as card16);
    fd = 0i32;
    while fd < (*cffont).num_fds as libc::c_int {
        if !(*cffont).subrs.is_null() &&
               !(*(*cffont).subrs.offset(fd as isize)).is_null() {
            cff_release_index(*(*cffont).subrs.offset(fd as isize));
            let ref mut fresh2 = *(*cffont).subrs.offset(fd as isize);
            *fresh2 = 0 as *mut cff_index
        }
        if !(*cffont).private.is_null() &&
               !(*(*cffont).private.offset(fd as isize)).is_null() {
            cff_dict_remove(*(*cffont).private.offset(fd as isize),
                            b"Subrs\x00" as *const u8 as *const libc::c_char);
            /* no Subrs */
        }
        fd += 1
    }
    destlen = write_fontfile(font, cffont);
    CIDFontInfo_close(&mut info);
    if verbose > 1i32 {
        dpx_message(b"[%u/%u glyphs][%d bytes]\x00" as *const u8 as
                        *const libc::c_char, num_glyphs as libc::c_int,
                    cs_count as libc::c_int, destlen);
    }
    CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_open(mut font: *mut CIDFont,
                                            mut name: *const libc::c_char,
                                            mut cmap_csi: *mut CIDSysInfo,
                                            mut opt: *mut cid_opt,
                                            mut expected_flag: libc::c_int)
 -> libc::c_int {
    let mut csi: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
    let mut fontname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sfont: *mut sfnt = 0 as *mut sfnt;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut offset: SFNT_ULONG = 0i32 as SFNT_ULONG;
    let mut is_cid_font: libc::c_int = 0i32;
    let mut expect_cid_font: libc::c_int =
        (expected_flag == 0i32) as libc::c_int;
    let mut expect_type1_font: libc::c_int = expected_flag & 1i32 << 8i32;
    if !font.is_null() {
    } else {
        __assert_fail(b"font\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cidtype0.c\x00" as *const u8 as
                          *const libc::c_char, 789i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 78],
                                                &[libc::c_char; 78]>(b"int CIDFont_type0_open(CIDFont *, const char *, CIDSysInfo *, cid_opt *, int)\x00")).as_ptr());
    }
    if expect_type1_font != 0 {
        if !cmap_csi.is_null() &&
               (strcmp((*cmap_csi).registry,
                       b"Adobe\x00" as *const u8 as *const libc::c_char) !=
                    0i32 ||
                    strcmp((*cmap_csi).ordering,
                           b"Identity\x00" as *const u8 as
                               *const libc::c_char) != 0i32) {
            return -1i32
        }
    }
    if expect_type1_font != 0 {
        handle = dpx_open_type1_file(name)
    } else { handle = dpx_open_opentype_file(name) }
    if expect_type1_font == 0 {
        if handle.is_null() {
            handle = dpx_open_truetype_file(name);
            if handle.is_null() { return -1i32 }
        }
        sfont = sfnt_open(handle);
        if sfont.is_null() {
            _tt_abort(b"Not a CFF/OpenType font: %s\x00" as *const u8 as
                          *const libc::c_char, name);
        }
        if (*sfont).type_0 == 1i32 << 4i32 {
            offset = ttc_read_offset(sfont, (*opt).index)
        }
        if (*sfont).type_0 != 1i32 << 4i32 && (*sfont).type_0 != 1i32 << 2i32
               || sfnt_read_table_directory(sfont, offset) < 0i32 ||
               {
                   offset =
                       sfnt_find_table_pos(sfont,
                                           b"CFF \x00" as *const u8 as
                                               *const libc::c_char);
                   offset == 0i32 as libc::c_uint
               } {
            sfnt_close(sfont);
            if !handle.is_null() { ttstub_input_close(handle); }
            return -1i32
        }
        cffont = cff_open((*sfont).handle, offset as libc::c_int, 0i32);
        if cffont.is_null() {
            _tt_abort(b"Cannot read CFF font data\x00" as *const u8 as
                          *const libc::c_char);
        }
        is_cid_font = (*cffont).flag & 1i32 << 0i32;
        if expect_cid_font != is_cid_font {
            cff_close(cffont);
            sfnt_close(sfont);
            if !handle.is_null() { ttstub_input_close(handle); }
            return -1i32
        }
        if is_cid_font != 0 {
            cff_read_charsets(cffont);
            (*opt).cff_charsets = (*cffont).charsets as *mut libc::c_void;
            (*cffont).charsets = 0 as *mut cff_charsets
        }
    } else {
        if handle.is_null() { return -1i32 }
        cffont = t1_load_font(0 as *mut *mut libc::c_char, 1i32, handle);
        if cffont.is_null() { ttstub_input_close(handle); return -1i32 }
        ttstub_input_close(handle);
    }
    csi =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<CIDSysInfo>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut CIDSysInfo;
    if is_cid_font != 0 {
        (*csi).registry =
            cff_get_string(cffont,
                           cff_dict_get((*cffont).topdict,
                                        b"ROS\x00" as *const u8 as
                                            *const libc::c_char, 0i32) as
                               s_SID);
        (*csi).ordering =
            cff_get_string(cffont,
                           cff_dict_get((*cffont).topdict,
                                        b"ROS\x00" as *const u8 as
                                            *const libc::c_char, 1i32) as
                               s_SID);
        (*csi).supplement =
            cff_dict_get((*cffont).topdict,
                         b"ROS\x00" as *const u8 as *const libc::c_char, 2i32)
                as libc::c_int
    } else {
        (*csi).registry =
            new((strlen(b"Adobe\x00" as *const u8 as
                            *const libc::c_char).wrapping_add(1i32 as
                                                                  libc::c_ulong)
                     as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy((*csi).registry,
               b"Adobe\x00" as *const u8 as *const libc::c_char);
        (*csi).ordering =
            new((strlen(b"Identity\x00" as *const u8 as
                            *const libc::c_char).wrapping_add(1i32 as
                                                                  libc::c_ulong)
                     as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy((*csi).ordering,
               b"Identity\x00" as *const u8 as *const libc::c_char);
        (*csi).supplement = 0i32
    }
    if expect_type1_font == 0 && !cmap_csi.is_null() {
        if strcmp((*csi).registry, (*cmap_csi).registry) != 0i32 ||
               strcmp((*csi).ordering, (*cmap_csi).ordering) != 0i32 {
            dpx_message(b"\nCharacter collection mismatched:\n\x00" as
                            *const u8 as *const libc::c_char);
            dpx_message(b"\tFont: %s-%s-%d\n\x00" as *const u8 as
                            *const libc::c_char, (*csi).registry,
                        (*csi).ordering, (*csi).supplement);
            dpx_message(b"\tCMap: %s-%s-%d\n\x00" as *const u8 as
                            *const libc::c_char, (*cmap_csi).registry,
                        (*cmap_csi).ordering, (*cmap_csi).supplement);
            _tt_abort(b"Inconsistent CMap specified for this font.\x00" as
                          *const u8 as *const libc::c_char);
        }
        if (*csi).supplement < (*cmap_csi).supplement {
            dpx_warning(b"CMap have higher supplmement number.\x00" as
                            *const u8 as *const libc::c_char);
            dpx_warning(b"Some characters may not be displayed or printed.\x00"
                            as *const u8 as *const libc::c_char);
        }
    }
    let mut shortname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontname_len: libc::c_int = 8i32;
    shortname = cff_get_name(cffont);
    if shortname.is_null() {
        _tt_abort(b"No valid FontName found.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /*
         * Mangled name requires more 7 bytes.
         * Style requires more 11 bytes.
         */
    if is_cid_font != 0 { fontname_len += 11i32 }
    fontname =
        new((strlen(shortname).wrapping_add(fontname_len as libc::c_ulong) as
                 uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_char;
    memset(fontname as *mut libc::c_void, 0i32,
           strlen(shortname).wrapping_add(fontname_len as libc::c_ulong));
    strcpy(fontname, shortname);
    free(shortname as *mut libc::c_void);
    cff_close(cffont);
    if is_cid_font != 0 {
        if (*opt).embed != 0 && (*opt).style != 0i32 {
            dpx_warning(b"Embedding disabled due to style option for %s.\x00"
                            as *const u8 as *const libc::c_char, name);
            (*opt).embed = 0i32
        }
        match (*opt).style {
            1 => {
                strcat(fontname,
                       b",Bold\x00" as *const u8 as *const libc::c_char);
            }
            2 => {
                strcat(fontname,
                       b",Italic\x00" as *const u8 as *const libc::c_char);
            }
            3 => {
                strcat(fontname,
                       b",BoldItalic\x00" as *const u8 as
                           *const libc::c_char);
            }
            _ => { }
        }
    } else if expect_type1_font != 0 {
        if (*opt).style != 0i32 {
            dpx_warning(b",Bold, ,Italic, ... not supported for this type of font...\x00"
                            as *const u8 as *const libc::c_char);
            (*opt).style = 0i32
        }
    } else { (*opt).embed = 1i32 }
    (*font).fontname = fontname;
    (*font).subtype = 1i32;
    (*font).csi = csi;
    (*font).flags |= expected_flag;
    (*font).fontdict = pdf_new_dict();
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"Type\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"Font\x00" as *const u8 as
                                  *const libc::c_char));
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"Subtype\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"CIDFontType0\x00" as *const u8 as
                                  *const libc::c_char));
    if expect_type1_font != 0 || (*opt).embed != 0 {
        memmove(fontname.offset(7) as *mut libc::c_void,
                fontname as *const libc::c_void,
                strlen(fontname).wrapping_add(1i32 as libc::c_ulong));
        pdf_font_make_uniqueTag(fontname);
        *fontname.offset(6) = '+' as i32 as libc::c_char
    }
    if expect_type1_font != 0 {
        (*font).descriptor = pdf_new_dict()
    } else {
        /* getting font info. from TrueType tables */
        (*font).descriptor =
            tt_get_fontdesc(sfont, &mut (*opt).embed, (*opt).stemv, 0i32,
                            name);
        if (*font).descriptor.is_null() {
            _tt_abort(b"Could not obtain necessary font info.\x00" as
                          *const u8 as *const libc::c_char);
        }
    }
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"FontName\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(fontname));
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"BaseFont\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(fontname));
    let mut csi_dict: *mut pdf_obj = pdf_new_dict();
    pdf_add_dict(csi_dict,
                 pdf_new_name(b"Registry\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_string((*csi).registry as *const libc::c_void,
                                strlen((*csi).registry)));
    pdf_add_dict(csi_dict,
                 pdf_new_name(b"Ordering\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_string((*csi).ordering as *const libc::c_void,
                                strlen((*csi).ordering)));
    pdf_add_dict(csi_dict,
                 pdf_new_name(b"Supplement\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number((*csi).supplement as libc::c_double));
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"CIDSystemInfo\x00" as *const u8 as
                                  *const libc::c_char), csi_dict);
    if is_cid_font != 0 {
        pdf_add_dict((*font).fontdict,
                     pdf_new_name(b"DW\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(1000i32 as libc::c_double));
        /* not sure */
    }
    if expect_type1_font == 0 {
        sfnt_close(sfont);
        if !handle.is_null() { ttstub_input_close(handle); }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_t1cdofont(mut font: *mut CIDFont) {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut charstrings: *mut cff_index = 0 as *mut cff_index;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut charstring_len: libc::c_int = 0;
    let mut max_len: libc::c_int = 0;
    let mut destlen: libc::c_int = 0i32;
    let mut size: libc::c_int = 0;
    let mut offset: libc::c_int = 0i32;
    let mut data: *mut card8 = 0 as *mut card8;
    let mut num_glyphs: card16 = 0;
    let mut gid: card16 = 0;
    let mut last_cid: card16 = 0;
    let mut i: libc::c_int = 0;
    let mut cid: libc::c_int = 0;
    let mut used_chars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut default_width: libc::c_double = 0.;
    let mut nominal_width: libc::c_double = 0.;
    let mut error: CIDType0Error = CID_OPEN_ERROR_NO_ERROR;
    let mut info: CIDType0Info =
        CIDType0Info{handle: 0 as *mut libc::c_void,
                     sfont: 0 as *mut sfnt,
                     cffont: 0 as *mut cff_font,};
    if !font.is_null() {
    } else {
        __assert_fail(b"font\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cidtype0.c\x00" as *const u8 as
                          *const libc::c_char, 1011i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void CIDFont_type0_t1cdofont(CIDFont *)\x00")).as_ptr());
    }
    if (*font).indirect.is_null() { return }
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"FontDescriptor\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_ref_obj((*font).descriptor));
    used_chars = CIDFont_type0_get_used_chars(font);
    error =
        CIDFont_type0_try_open((*font).ident, CIDFont_get_opt_index(font),
                               0i32, &mut info);
    if error as libc::c_int != CID_OPEN_ERROR_NO_ERROR as libc::c_int {
        CIDType0Error_Show(error, (*font).ident);
        return
    }
    cffont = info.cffont;
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    if !(*(*cffont).private.offset(0)).is_null() &&
           cff_dict_known(*(*cffont).private.offset(0),
                          b"StdVW\x00" as *const u8 as *const libc::c_char) !=
               0 {
        let mut stemv: libc::c_double = 0.;
        stemv =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"StdVW\x00" as *const u8 as *const libc::c_char,
                         0i32);
        pdf_add_dict((*font).descriptor,
                     pdf_new_name(b"StemV\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number(stemv));
    }
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
    num_glyphs = 0i32 as card16;
    last_cid = 0i32 as card16;
    let ref mut fresh3 = *used_chars.offset((0i32 / 8i32) as isize);
    *fresh3 =
        (*fresh3 as libc::c_int | 1i32 << 7i32 - 0i32 % 8i32) as libc::c_char;
    /* .notdef */
    i = 0i32;
    while i < ((*cffont).num_glyphs as libc::c_int + 7i32) / 8i32 {
        let mut c: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        c = *used_chars.offset(i as isize) as libc::c_int;
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
    fdselect =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_fdselect>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_fdselect;
    (*fdselect).format = 3i32 as card8;
    (*fdselect).num_entries = 1i32 as card16;
    (*fdselect).data.ranges =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_range3>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_range3;
    (*(*fdselect).data.ranges.offset(0)).first = 0i32 as card16;
    (*(*fdselect).data.ranges.offset(0)).fd = 0i32 as card8;
    (*cffont).fdselect = fdselect;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    charset =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_charsets>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = (num_glyphs as libc::c_int - 1i32) as card16;
    (*charset).data.glyphs =
        new(((num_glyphs as libc::c_int - 1i32) as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<s_SID>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut s_SID;
    gid = 0i32 as card16;
    cid = 0i32;
    while cid <= last_cid as libc::c_int {
        if *used_chars.offset((cid / 8i32) as isize) as libc::c_int &
               1i32 << 7i32 - cid % 8i32 != 0 {
            if gid as libc::c_int > 0i32 {
                *(*charset).data.glyphs.offset((gid as libc::c_int - 1i32) as
                                                   isize) = cid as s_SID
            }
            gid = gid.wrapping_add(1)
        }
        cid += 1
    }
    /* cff_release_charsets(cffont->charsets); */
    (*cffont).charsets = charset; /* FIXME: Skip XXXXXX+ */
    cff_dict_add((*cffont).topdict,
                 b"CIDCount\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"CIDCount\x00" as *const u8 as *const libc::c_char, 0i32,
                 (last_cid as libc::c_int + 1i32) as libc::c_double);
    (*cffont).fdarray =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut cff_dict>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut *mut cff_dict;
    let ref mut fresh4 = *(*cffont).fdarray.offset(0);
    *fresh4 = cff_new_dict();
    cff_dict_add(*(*cffont).fdarray.offset(0),
                 b"FontName\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set(*(*cffont).fdarray.offset(0),
                 b"FontName\x00" as *const u8 as *const libc::c_char, 0i32,
                 cff_add_string(cffont, (*font).fontname.offset(7), 1i32) as
                     libc::c_double);
    cff_dict_add(*(*cffont).fdarray.offset(0),
                 b"Private\x00" as *const u8 as *const libc::c_char, 2i32);
    cff_dict_set(*(*cffont).fdarray.offset(0),
                 b"Private\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    cff_dict_set(*(*cffont).fdarray.offset(0),
                 b"Private\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    /* FDArray  - index offset, not known yet */
    cff_dict_add((*cffont).topdict,
                 b"FDArray\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"FDArray\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    /* FDSelect - offset, not known yet */
    cff_dict_add((*cffont).topdict,
                 b"FDSelect\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"FDSelect\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    cff_dict_remove((*cffont).topdict,
                    b"UniqueID\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"XUID\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"Private\x00" as *const u8 as *const libc::c_char);
    cff_dict_remove((*cffont).topdict,
                    b"Encoding\x00" as *const u8 as *const libc::c_char);
    /* */
    offset =
        cff_dict_get((*cffont).topdict,
                     b"CharStrings\x00" as *const u8 as *const libc::c_char,
                     0i32) as libc::c_int;
    ttstub_input_seek((*cffont).handle,
                      (*cffont).offset.wrapping_add(offset as libc::c_uint) as
                          ssize_t, 0i32);
    idx = cff_get_index_header(cffont);
    /* offset is now absolute offset ... bad */
    offset =
        ttstub_input_seek((*cffont).handle, 0i32 as ssize_t, 1i32) as
            libc::c_int;
    if ((*idx).count as libc::c_int) < 2i32 {
        _tt_abort(b"No valid charstring data found.\x00" as *const u8 as
                      *const libc::c_char);
    }
    /* New CharStrings INDEX */
    charstrings = cff_new_index((num_glyphs as libc::c_int + 1i32) as card16);
    max_len = 2i32 * 65536i32;
    (*charstrings).data =
        new((max_len as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    charstring_len = 0i32;
    gid = 0i32 as card16;
    data =
        new((65536i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut card8;
    cid = 0i32;
    while cid <= last_cid as libc::c_int {
        if !(*used_chars.offset((cid / 8i32) as isize) as libc::c_int &
                 1i32 << 7i32 - cid % 8i32 == 0) {
            size =
                (*(*idx).offset.offset((cid + 1i32) as
                                           isize)).wrapping_sub(*(*idx).offset.offset(cid
                                                                                          as
                                                                                          isize))
                    as libc::c_int;
            if size > 65536i32 {
                _tt_abort(b"Charstring too long: gid=%u\x00" as *const u8 as
                              *const libc::c_char, cid);
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
            *(*charstrings).offset.offset(gid as isize) =
                (charstring_len + 1i32) as l_offset;
            ttstub_input_seek((*cffont).handle,
                              (offset as
                                   libc::c_uint).wrapping_add(*(*idx).offset.offset(cid
                                                                                        as
                                                                                        isize)).wrapping_sub(1i32
                                                                                                                 as
                                                                                                                 libc::c_uint)
                                  as ssize_t, 0i32);
            ttstub_input_read((*cffont).handle, data as *mut libc::c_char,
                              size as size_t);
            charstring_len +=
                cs_copy_charstring((*charstrings).data.offset(charstring_len
                                                                  as isize),
                                   max_len - charstring_len, data, size,
                                   (*cffont).gsubr,
                                   *(*cffont).subrs.offset(0), default_width,
                                   nominal_width, 0 as *mut cs_ginfo);
            gid = gid.wrapping_add(1)
        }
        cid += 1
    }
    if gid as libc::c_int != num_glyphs as libc::c_int {
        _tt_abort(b"Unexpeced error: ?????\x00" as *const u8 as
                      *const libc::c_char);
    }
    free(data as *mut libc::c_void);
    cff_release_index(idx);
    *(*charstrings).offset.offset(num_glyphs as isize) =
        (charstring_len + 1i32) as l_offset;
    (*charstrings).count = num_glyphs;
    (*cffont).num_glyphs = num_glyphs;
    (*cffont).cstrings = charstrings;
    /* no Global subr */
    if !(*cffont).gsubr.is_null() { cff_release_index((*cffont).gsubr); }
    (*cffont).gsubr = cff_new_index(0i32 as card16);
    if !(*cffont).subrs.is_null() && !(*(*cffont).subrs.offset(0)).is_null() {
        cff_release_index(*(*cffont).subrs.offset(0));
        let ref mut fresh5 = *(*cffont).subrs.offset(0);
        *fresh5 = 0 as *mut cff_index
    }
    if !(*cffont).private.is_null() &&
           !(*(*cffont).private.offset(0)).is_null() {
        cff_dict_remove(*(*cffont).private.offset(0),
                        b"Subrs\x00" as *const u8 as *const libc::c_char);
        /* no Subrs */
    }
    cff_add_string(cffont, b"Adobe\x00" as *const u8 as *const libc::c_char,
                   1i32);
    cff_add_string(cffont,
                   b"Identity\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(*(*cffont).private.offset(0), cffont);
    cff_update_string(cffont);
    /* CFF code need to be rewrote... */
    cff_dict_add((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 3i32);
    cff_dict_set((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 0i32,
                 cff_get_sid(cffont,
                             b"Adobe\x00" as *const u8 as *const libc::c_char)
                     as libc::c_double);
    cff_dict_set((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 1i32,
                 cff_get_sid(cffont,
                             b"Identity\x00" as *const u8 as
                                 *const libc::c_char) as libc::c_double);
    cff_dict_set((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 2i32,
                 0.0f64);
    destlen = write_fontfile(font, cffont);
    /*
     * DW, W, DW2 and W2:
     * Those values are obtained from OpenType table (not TFM).
     */
    let mut CIDToGIDMap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    CIDToGIDMap =
        new(((2i32 * (last_cid as libc::c_int + 1i32)) as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_uchar;
    memset(CIDToGIDMap as *mut libc::c_void, 0i32,
           (2i32 * (last_cid as libc::c_int + 1i32)) as libc::c_ulong);
    cid = 0i32;
    while cid <= last_cid as libc::c_int {
        if *used_chars.offset((cid / 8i32) as isize) as libc::c_int &
               1i32 << 7i32 - cid % 8i32 != 0 {
            *CIDToGIDMap.offset((2i32 * cid) as isize) =
                (cid >> 8i32 & 0xffi32) as libc::c_uchar;
            *CIDToGIDMap.offset((2i32 * cid + 1i32) as isize) =
                (cid & 0xffi32) as libc::c_uchar
        }
        cid += 1
    }
    add_CIDMetrics(info.sfont, (*font).fontdict, CIDToGIDMap, last_cid,
                   if CIDFont_get_parent_id(font, 1i32) < 0i32 {
                       0i32
                   } else { 1i32 });
    free(CIDToGIDMap as *mut libc::c_void);
    CIDFontInfo_close(&mut info);
    if verbose > 1i32 {
        dpx_message(b"[%u glyphs][%d bytes]\x00" as *const u8 as
                        *const libc::c_char, num_glyphs as libc::c_int,
                    destlen);
    }
    CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
unsafe extern "C" fn load_base_CMap(mut font_name: *const libc::c_char,
                                    mut wmode: libc::c_int,
                                    mut cffont: *mut cff_font)
 -> libc::c_int {
    let mut cmap_id: libc::c_int = -1i32;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut cmap_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gid: card16 = 0;
    let mut range_min: [libc::c_uchar; 4] =
        [0i32 as libc::c_uchar, 0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar];
    let mut range_max: [libc::c_uchar; 4] =
        [0x7fi32 as libc::c_uchar, 0xffi32 as libc::c_uchar,
         0xffi32 as libc::c_uchar, 0xffi32 as libc::c_uchar];
    cmap_name =
        new((strlen(font_name).wrapping_add(strlen(b"-UCS4-H\x00" as *const u8
                                                       as
                                                       *const libc::c_char)).wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_ulong)
                 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_char;
    if wmode != 0 {
        sprintf(cmap_name,
                b"%s-UCS4-V\x00" as *const u8 as *const libc::c_char,
                font_name);
    } else {
        sprintf(cmap_name,
                b"%s-UCS4-H\x00" as *const u8 as *const libc::c_char,
                font_name);
    }
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0i32 {
        free(cmap_name as *mut libc::c_void);
        return cmap_id
    }
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_type(cmap, 1i32);
    CMap_set_wmode(cmap, wmode);
    CMap_add_codespacerange(cmap, range_min.as_mut_ptr(),
                            range_max.as_mut_ptr(), 4i32 as size_t);
    CMap_set_CIDSysInfo(cmap, &mut CSI_IDENTITY);
    free(cmap_name as *mut libc::c_void);
    gid = 1i32 as card16;
    while (gid as libc::c_int) < (*cffont).num_glyphs as libc::c_int {
        let mut ucv: int32_t = 0;
        let mut sid: s_SID = 0;
        let mut glyph: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut srcCode: [libc::c_uchar; 4] = [0; 4];
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
                srcCode[0] = (ucv >> 24i32 & 0xffi32) as libc::c_uchar;
                srcCode[1] = (ucv >> 16i32 & 0xffi32) as libc::c_uchar;
                srcCode[2] = (ucv >> 8i32 & 0xffi32) as libc::c_uchar;
                srcCode[3] = (ucv & 0xffi32) as libc::c_uchar;
                CMap_add_cidchar(cmap, srcCode.as_mut_ptr(), 4i32 as size_t,
                                 gid);
            } else {
                let mut agln: *mut agl_name = 0 as *mut agl_name;
                agln = agl_lookup_list(name);
                if agln.is_null() {
                    dpx_warning(b"Glyph \"%s\" inaccessible (no Unicode mapping)\x00"
                                    as *const u8 as *const libc::c_char,
                                glyph);
                }
                while !agln.is_null() {
                    if (*agln).n_components > 1i32 {
                        dpx_warning(b"Glyph \"%s\" inaccessible (composite character)\x00"
                                        as *const u8 as *const libc::c_char,
                                    glyph);
                    } else if (*agln).n_components == 1i32 {
                        ucv = (*agln).unicodes[0];
                        srcCode[0] =
                            (ucv >> 24i32 & 0xffi32) as libc::c_uchar;
                        srcCode[1] =
                            (ucv >> 16i32 & 0xffi32) as libc::c_uchar;
                        srcCode[2] = (ucv >> 8i32 & 0xffi32) as libc::c_uchar;
                        srcCode[3] = (ucv & 0xffi32) as libc::c_uchar;
                        CMap_add_cidchar(cmap, srcCode.as_mut_ptr(),
                                         4i32 as size_t, gid);
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
    return cmap_id;
}
#[no_mangle]
pub unsafe extern "C" fn t1_load_UnicodeCMap(mut font_name:
                                                 *const libc::c_char,
                                             mut otl_tags:
                                                 *const libc::c_char,
                                             mut wmode: libc::c_int)
 -> libc::c_int {
    let mut cmap_id: libc::c_int = -1i32;
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut handle: *mut rust_input_handle_t = 0 as *mut rust_input_handle_t;
    if font_name.is_null() { return -1i32 }
    handle = dpx_open_type1_file(font_name) as *mut rust_input_handle_t;
    if handle.is_null() { return -1i32 }
    cffont =
        t1_load_font(0 as *mut *mut libc::c_char, 1i32,
                     handle as rust_input_handle_t);
    ttstub_input_close(handle as rust_input_handle_t);
    if cffont.is_null() { return -1i32 }
    cmap_id = load_base_CMap(font_name, wmode, cffont);
    cff_close(cffont);
    if cmap_id < 0i32 {
        _tt_abort(b"Failed to create Unicode charmap for font \"%s\".\x00" as
                      *const u8 as *const libc::c_char, font_name);
    }
    if !otl_tags.is_null() {
        dpx_warning(b"Glyph substitution not supported for Type1 font yet...\x00"
                        as *const u8 as *const libc::c_char);
    }
    return cmap_id;
}
/*
 * ToUnicode CMap
 */
unsafe extern "C" fn create_ToUnicode_stream(mut cffont: *mut cff_font,
                                             mut font_name:
                                                 *const libc::c_char,
                                             mut used_glyphs:
                                                 *const libc::c_char)
 -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut cid: CID = 0;
    let mut gid: card16 = 0;
    let mut glyph_count: libc::c_int = 0;
    let mut total_fail_count: libc::c_int = 0;
    let mut cmap_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wbuf: [libc::c_uchar; 1024] = [0; 1024];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut endptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    static mut range_min: [libc::c_uchar; 2] =
        [0i32 as libc::c_uchar, 0i32 as libc::c_uchar];
    static mut range_max: [libc::c_uchar; 2] =
        [0xffi32 as libc::c_uchar, 0xffi32 as libc::c_uchar];
    if font_name.is_null() || used_glyphs.is_null() {
        return 0 as *mut pdf_obj
    }
    cmap = CMap_new();
    cmap_name =
        new((strlen(font_name).wrapping_add(strlen(b"-UTF16\x00" as *const u8
                                                       as
                                                       *const libc::c_char)).wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_ulong)
                 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_char;
    strcpy(cmap_name, font_name);
    strcat(cmap_name, b"-UTF16\x00" as *const u8 as *const libc::c_char);
    CMap_set_name(cmap, cmap_name);
    free(cmap_name as *mut libc::c_void);
    CMap_set_wmode(cmap, 0i32);
    CMap_set_type(cmap, 2i32);
    CMap_set_CIDSysInfo(cmap, &mut CSI_UNICODE);
    CMap_add_codespacerange(cmap, range_min.as_mut_ptr(),
                            range_max.as_mut_ptr(), 2i32 as size_t);
    total_fail_count = 0i32;
    glyph_count = total_fail_count;
    p = wbuf.as_mut_ptr();
    endptr = wbuf.as_mut_ptr().offset(1024);
    cid = 1i32 as CID;
    while (cid as libc::c_int) < (*cffont).num_glyphs as libc::c_int {
        /* Skip .notdef */
        if *used_glyphs.offset((cid as libc::c_int / 8i32) as isize) as
               libc::c_int & 1i32 << 7i32 - cid as libc::c_int % 8i32 != 0 {
            let mut glyph: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: int32_t = 0;
            let mut fail_count: libc::c_int = 0;
            wbuf[0] = (cid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
            wbuf[1] = (cid as libc::c_int & 0xffi32) as libc::c_uchar;
            p = wbuf.as_mut_ptr().offset(2);
            gid = cff_charsets_lookup_inverse(cffont, cid);
            if !(gid as libc::c_int == 0i32) {
                glyph = cff_get_string(cffont, gid);
                if !glyph.is_null() {
                    len =
                        agl_sput_UTF16BE(glyph, &mut p, endptr,
                                         &mut fail_count);
                    if len < 1i32 || fail_count != 0 {
                        total_fail_count += fail_count
                    } else {
                        CMap_add_bfchar(cmap, wbuf.as_mut_ptr(),
                                        2i32 as size_t,
                                        wbuf.as_mut_ptr().offset(2),
                                        len as size_t);
                    }
                    free(glyph as *mut libc::c_void);
                }
                glyph_count += 1
            }
        }
        cid = cid.wrapping_add(1)
    }
    if total_fail_count != 0i32 && total_fail_count >= glyph_count / 10i32 {
        dpx_warning(b"%d glyph names (out of %d) missing Unicode mapping.\x00"
                        as *const u8 as *const libc::c_char, total_fail_count,
                    glyph_count);
        dpx_warning(b"ToUnicode CMap \"%s-UTF16\" removed.\x00" as *const u8
                        as *const libc::c_char, font_name);
    } else { stream = CMap_create_stream(cmap) }
    CMap_release(cmap);
    return stream;
}
/* Force bold at small text sizes */
/* pdf_font --> CIDFont */
unsafe extern "C" fn get_font_attr(mut font: *mut CIDFont,
                                   mut cffont: *mut cff_font) {
    let mut capheight: libc::c_double = 0.;
    let mut ascent: libc::c_double = 0.;
    let mut descent: libc::c_double = 0.;
    let mut italicangle: libc::c_double = 0.;
    let mut stemv: libc::c_double = 0.;
    let mut defaultwidth: libc::c_double = 0.;
    let mut nominalwidth: libc::c_double = 0.;
    let mut flags: libc::c_int = 0i32;
    let mut gid: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    static mut L_c: [*const libc::c_char; 5] =
        [b"H\x00" as *const u8 as *const libc::c_char,
         b"P\x00" as *const u8 as *const libc::c_char,
         b"Pi\x00" as *const u8 as *const libc::c_char,
         b"Rho\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    static mut L_d: [*const libc::c_char; 5] =
        [b"p\x00" as *const u8 as *const libc::c_char,
         b"q\x00" as *const u8 as *const libc::c_char,
         b"mu\x00" as *const u8 as *const libc::c_char,
         b"eta\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    static mut L_a: [*const libc::c_char; 4] =
        [b"b\x00" as *const u8 as *const libc::c_char,
         b"h\x00" as *const u8 as *const libc::c_char,
         b"lambda\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut gm: t1_ginfo =
        t1_ginfo{use_seac: 0,
                 wx: 0.,
                 wy: 0.,
                 bbox: C2RustUnnamed_7{llx: 0., lly: 0., urx: 0., ury: 0.,},
                 seac:
                     C2RustUnnamed_6{asb: 0.,
                                     adx: 0.,
                                     ady: 0.,
                                     bchar: 0,
                                     achar: 0,},};
    defaultwidth = 500.0f64;
    nominalwidth = 0.0f64;
    /*
     * CapHeight, Ascent, and Descent is meaningfull only for Latin/Greek/Cyrillic.
     * The BlueValues and OtherBlues also have those information.
     */
    if cff_dict_known((*cffont).topdict,
                      b"FontBBox\x00" as *const u8 as *const libc::c_char) !=
           0 {
        /* Default values */
        ascent =
            cff_dict_get((*cffont).topdict,
                         b"FontBBox\x00" as *const u8 as *const libc::c_char,
                         3i32);
        capheight = ascent;
        descent =
            cff_dict_get((*cffont).topdict,
                         b"FontBBox\x00" as *const u8 as *const libc::c_char,
                         1i32)
    } else { capheight = 680.0f64; ascent = 690.0f64; descent = -190.0f64 }
    if cff_dict_known(*(*cffont).private.offset(0),
                      b"StdVW\x00" as *const u8 as *const libc::c_char) != 0 {
        stemv =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"StdVW\x00" as *const u8 as *const libc::c_char,
                         0i32)
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
    if cff_dict_known((*cffont).topdict,
                      b"ItalicAngle\x00" as *const u8 as *const libc::c_char)
           != 0 {
        italicangle =
            cff_dict_get((*cffont).topdict,
                         b"ItalicAngle\x00" as *const u8 as
                             *const libc::c_char, 0i32);
        if italicangle != 0.0f64 { flags |= 1i32 << 6i32 }
    } else { italicangle = 0.0f64 }
    /*
     * Use "space", "H", "p", and "b" for various values.
     * Those characters should not "seac". (no accent)
     */
    gid =
        cff_glyph_lookup(cffont,
                         b"space\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
        t1char_get_metrics((*(*cffont).cstrings).data.offset(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                      as
                                                                                                      isize)
                                                                 as
                                                                 isize).offset(-1),
                           (*(*(*cffont).cstrings).offset.offset((gid + 1i32)
                                                                     as
                                                                     isize)).wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                                                   as
                                                                                                                                   isize))
                               as libc::c_int, *(*cffont).subrs.offset(0),
                           &mut gm);
        defaultwidth = gm.wx
    }
    i = 0i32;
    while !L_c[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_c[i as usize]) as libc::c_int;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
            t1char_get_metrics((*(*cffont).cstrings).data.offset(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                          as
                                                                                                          isize)
                                                                     as
                                                                     isize).offset(-1),
                               (*(*(*cffont).cstrings).offset.offset((gid +
                                                                          1i32)
                                                                         as
                                                                         isize)).wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                                                       as
                                                                                                                                       isize))
                                   as libc::c_int, *(*cffont).subrs.offset(0),
                               &mut gm);
            capheight = gm.bbox.ury;
            break ;
        } else { i += 1 }
    }
    i = 0i32;
    while !L_d[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_d[i as usize]) as libc::c_int;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
            t1char_get_metrics((*(*cffont).cstrings).data.offset(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                          as
                                                                                                          isize)
                                                                     as
                                                                     isize).offset(-1),
                               (*(*(*cffont).cstrings).offset.offset((gid +
                                                                          1i32)
                                                                         as
                                                                         isize)).wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                                                       as
                                                                                                                                       isize))
                                   as libc::c_int, *(*cffont).subrs.offset(0),
                               &mut gm);
            descent = gm.bbox.lly;
            break ;
        } else { i += 1 }
    }
    i = 0i32;
    while !L_a[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_a[i as usize]) as libc::c_int;
        if gid >= 0i32 && gid < (*(*cffont).cstrings).count as libc::c_int {
            t1char_get_metrics((*(*cffont).cstrings).data.offset(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                          as
                                                                                                          isize)
                                                                     as
                                                                     isize).offset(-1),
                               (*(*(*cffont).cstrings).offset.offset((gid +
                                                                          1i32)
                                                                         as
                                                                         isize)).wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid
                                                                                                                                       as
                                                                                                                                       isize))
                                   as libc::c_int, *(*cffont).subrs.offset(0),
                               &mut gm);
            ascent = gm.bbox.ury;
            break ;
        } else { i += 1 }
    }
    if defaultwidth != 0.0f64 {
        cff_dict_add(*(*cffont).private.offset(0),
                     b"defaultWidthX\x00" as *const u8 as *const libc::c_char,
                     1i32);
        cff_dict_set(*(*cffont).private.offset(0),
                     b"defaultWidthX\x00" as *const u8 as *const libc::c_char,
                     0i32, defaultwidth);
    }
    if nominalwidth != 0.0f64 {
        cff_dict_add(*(*cffont).private.offset(0),
                     b"nominalWidthX\x00" as *const u8 as *const libc::c_char,
                     1i32);
        cff_dict_set(*(*cffont).private.offset(0),
                     b"nominalWidthX\x00" as *const u8 as *const libc::c_char,
                     0i32, nominalwidth);
    }
    if cff_dict_known(*(*cffont).private.offset(0),
                      b"ForceBold\x00" as *const u8 as *const libc::c_char) !=
           0 &&
           cff_dict_get(*(*cffont).private.offset(0),
                        b"ForceBold\x00" as *const u8 as *const libc::c_char,
                        0i32) != 0. {
        flags |= 1i32 << 18i32
    }
    if cff_dict_known(*(*cffont).private.offset(0),
                      b"IsFixedPitch\x00" as *const u8 as *const libc::c_char)
           != 0 &&
           cff_dict_get(*(*cffont).private.offset(0),
                        b"IsFixedPitch\x00" as *const u8 as
                            *const libc::c_char, 0i32) != 0. {
        flags |= 1i32 << 0i32
    }
    if !(*font).fontname.is_null() &&
           strstr((*font).fontname,
                  b"Sans\x00" as *const u8 as *const libc::c_char).is_null() {
        flags |= 1i32 << 1i32
    }
    flags |= 1i32 << 2i32;
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"CapHeight\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(capheight));
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"Ascent\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(ascent));
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"Descent\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(descent));
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"ItalicAngle\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(italicangle));
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"StemV\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(stemv));
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"Flags\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number(flags as libc::c_double));
}
unsafe extern "C" fn add_metrics(mut font: *mut CIDFont,
                                 mut cffont: *mut cff_font,
                                 mut CIDToGIDMap: *mut libc::c_uchar,
                                 mut widths: *mut libc::c_double,
                                 mut default_width: libc::c_double,
                                 mut last_cid: CID) {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut val: libc::c_double = 0.;
    let mut cid: card16 = 0;
    let mut gid: card16 = 0;
    let mut used_chars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut parent_id: libc::c_int = 0;
    /*
     * The original FontBBox of the font is preserved, instead
     * of replacing it with tight bounding box calculated from
     * charstrings, to prevent Acrobat 4 from greeking text as
     * much as possible.
     */
    if cff_dict_known((*cffont).topdict,
                      b"FontBBox\x00" as *const u8 as *const libc::c_char) ==
           0 {
        _tt_abort(b"No FontBBox?\x00" as *const u8 as *const libc::c_char);
    }
    tmp = pdf_new_array();
    i = 0i32;
    while i < 4i32 {
        val =
            cff_dict_get((*cffont).topdict,
                         b"FontBBox\x00" as *const u8 as *const libc::c_char,
                         i);
        pdf_add_array(tmp,
                      pdf_new_number(floor(val / 1.0f64 + 0.5f64) * 1.0f64));
        i += 1
    }
    pdf_add_dict((*font).descriptor,
                 pdf_new_name(b"FontBBox\x00" as *const u8 as
                                  *const libc::c_char), tmp);
    parent_id = CIDFont_get_parent_id(font, 0i32);
    if parent_id < 0i32 &&
           { parent_id = CIDFont_get_parent_id(font, 1i32); parent_id < 0i32 }
       {
        _tt_abort(b"No parent Type 0 font !\x00" as *const u8 as
                      *const libc::c_char);
    }
    used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
    if used_chars.is_null() {
        _tt_abort(b"Unexpected error: Font not actually used???\x00" as
                      *const u8 as *const libc::c_char);
    }
    /* FIXME:
     * This writes "CID CID width".
     * I think it's better to handle each 8 char block
     * and to use "CID_start [ w0 w1 ...]".
     */
    tmp = pdf_new_array();
    cid = 0i32 as card16;
    while cid as libc::c_int <= last_cid as libc::c_int {
        if *used_chars.offset((cid as libc::c_int / 8i32) as isize) as
               libc::c_int & 1i32 << 7i32 - cid as libc::c_int % 8i32 != 0 {
            gid =
                ((*CIDToGIDMap.offset((2i32 * cid as libc::c_int) as isize) as
                      libc::c_int) << 8i32 |
                     *CIDToGIDMap.offset((2i32 * cid as libc::c_int + 1i32) as
                                             isize) as libc::c_int) as card16;
            if *widths.offset(gid as isize) != default_width {
                pdf_add_array(tmp, pdf_new_number(cid as libc::c_double));
                pdf_add_array(tmp, pdf_new_number(cid as libc::c_double));
                pdf_add_array(tmp,
                              pdf_new_number(floor(*widths.offset(gid as
                                                                      isize) /
                                                       1.0f64 + 0.5f64) *
                                                 1.0f64));
            }
        }
        cid = cid.wrapping_add(1)
    }
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"DW\x00" as *const u8 as *const libc::c_char),
                 pdf_new_number(default_width));
    if pdf_array_length(tmp) > 0i32 as libc::c_uint {
        pdf_add_dict((*font).fontdict,
                     pdf_new_name(b"W\x00" as *const u8 as
                                      *const libc::c_char), pdf_ref_obj(tmp));
    }
    pdf_release_obj(tmp);
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
/* Type1 --> CFF CIDFont */
#[no_mangle]
pub unsafe extern "C" fn CIDFont_type0_t1dofont(mut font: *mut CIDFont) {
    let mut cffont: *mut cff_font = 0 as *mut cff_font;
    let mut defaultwidth: libc::c_double = 0.;
    let mut nominalwidth: libc::c_double = 0.;
    let mut num_glyphs: libc::c_int = 0i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut used_chars: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_cid: card16 = 0;
    let mut gid: card16 = 0;
    let mut cid: card16 = 0;
    let mut CIDToGIDMap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !font.is_null() {
    } else {
        __assert_fail(b"font\x00" as *const u8 as *const libc::c_char,
                      b"dpx-cidtype0.c\x00" as *const u8 as
                          *const libc::c_char, 1659i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void CIDFont_type0_t1dofont(CIDFont *)\x00")).as_ptr());
    }
    if (*font).indirect.is_null() { return }
    pdf_add_dict((*font).fontdict,
                 pdf_new_name(b"FontDescriptor\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_ref_obj((*font).descriptor));
    handle = dpx_open_type1_file((*font).ident);
    if handle.is_null() {
        _tt_abort(b"Type1: Could not open Type1 font.\x00" as *const u8 as
                      *const libc::c_char);
    }
    cffont = t1_load_font(0 as *mut *mut libc::c_char, 0i32, handle);
    if cffont.is_null() {
        _tt_abort(b"Could not read Type 1 font...\x00" as *const u8 as
                      *const libc::c_char);
    }
    ttstub_input_close(handle);
    if (*font).fontname.is_null() {
        _tt_abort(b"Fontname undefined...\x00" as *const u8 as
                      *const libc::c_char);
    }
    let mut hparent: *mut Type0Font = 0 as *mut Type0Font;
    let mut vparent: *mut Type0Font = 0 as *mut Type0Font;
    let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut vparent_id: libc::c_int = 0;
    let mut hparent_id: libc::c_int = 0;
    hparent_id = CIDFont_get_parent_id(font, 0i32);
    vparent_id = CIDFont_get_parent_id(font, 1i32);
    if hparent_id < 0i32 && vparent_id < 0i32 {
        _tt_abort(b"No parent Type 0 font !\x00" as *const u8 as
                      *const libc::c_char);
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
        _tt_abort(b"Unexpected error: Font not actually used???\x00" as
                      *const u8 as *const libc::c_char);
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
    if cff_dict_known(*(*cffont).private.offset(0),
                      b"defaultWidthX\x00" as *const u8 as
                          *const libc::c_char) != 0 {
        defaultwidth =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"defaultWidthX\x00" as *const u8 as
                             *const libc::c_char, 0i32)
    } else { defaultwidth = 0.0f64 }
    if cff_dict_known(*(*cffont).private.offset(0),
                      b"nominalWidthX\x00" as *const u8 as
                          *const libc::c_char) != 0 {
        nominalwidth =
            cff_dict_get(*(*cffont).private.offset(0),
                         b"nominalWidthX\x00" as *const u8 as
                             *const libc::c_char, 0i32)
    } else { nominalwidth = 0.0f64 }
    num_glyphs = 0i32;
    last_cid = 0i32 as card16;
    let ref mut fresh6 = *used_chars.offset((0i32 / 8i32) as isize);
    *fresh6 =
        (*fresh6 as libc::c_int | 1i32 << 7i32 - 0i32 % 8i32) as libc::c_char;
    /* .notdef */
    i = 0i32; /* FIXME: Skip XXXXXX+ */
    while i < ((*cffont).num_glyphs as libc::c_int + 7i32) / 8i32 {
        let mut c: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        c = *used_chars.offset(i as isize) as libc::c_int;
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
    fdselect =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_fdselect>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_fdselect;
    (*fdselect).format = 3i32 as card8;
    (*fdselect).num_entries = 1i32 as card16;
    (*fdselect).data.ranges =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_range3>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_range3;
    (*(*fdselect).data.ranges.offset(0)).first = 0i32 as card16;
    (*(*fdselect).data.ranges.offset(0)).fd = 0i32 as card8;
    (*cffont).fdselect = fdselect;
    CIDToGIDMap =
        new(((2i32 * (last_cid as libc::c_int + 1i32)) as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_uchar;
    memset(CIDToGIDMap as *mut libc::c_void, 0i32,
           (2i32 * (last_cid as libc::c_int + 1i32)) as libc::c_ulong);
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    charset =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<cff_charsets>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut cff_charsets;
    (*charset).format = 0i32 as card8;
    (*charset).num_entries = (num_glyphs - 1i32) as card16;
    (*charset).data.glyphs =
        new(((num_glyphs - 1i32) as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<s_SID>() as
                                                 libc::c_ulong) as uint32_t)
            as *mut s_SID;
    gid = 0i32 as card16;
    cid = 0i32 as card16;
    while cid as libc::c_int <= last_cid as libc::c_int {
        if *used_chars.offset((cid as libc::c_int / 8i32) as isize) as
               libc::c_int & 1i32 << 7i32 - cid as libc::c_int % 8i32 != 0 {
            if gid as libc::c_int > 0i32 {
                *(*charset).data.glyphs.offset((gid as libc::c_int - 1i32) as
                                                   isize) = cid
            }
            *CIDToGIDMap.offset((2i32 * cid as libc::c_int) as isize) =
                (gid as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
            *CIDToGIDMap.offset((2i32 * cid as libc::c_int + 1i32) as isize) =
                (gid as libc::c_int & 0xffi32) as libc::c_uchar;
            gid = gid.wrapping_add(1)
        }
        cid = cid.wrapping_add(1)
    }
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    cff_dict_add((*cffont).topdict,
                 b"CIDCount\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"CIDCount\x00" as *const u8 as *const libc::c_char, 0i32,
                 (last_cid as libc::c_int + 1i32) as libc::c_double);
    (*cffont).fdarray =
        new((1i32 as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut cff_dict>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut *mut cff_dict;
    let ref mut fresh7 = *(*cffont).fdarray.offset(0);
    *fresh7 = cff_new_dict();
    cff_dict_add(*(*cffont).fdarray.offset(0),
                 b"FontName\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set(*(*cffont).fdarray.offset(0),
                 b"FontName\x00" as *const u8 as *const libc::c_char, 0i32,
                 cff_add_string(cffont, (*font).fontname.offset(7), 1i32) as
                     libc::c_double);
    cff_dict_add(*(*cffont).fdarray.offset(0),
                 b"Private\x00" as *const u8 as *const libc::c_char, 2i32);
    cff_dict_set(*(*cffont).fdarray.offset(0),
                 b"Private\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    cff_dict_set(*(*cffont).fdarray.offset(0),
                 b"Private\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    /* FDArray  - index offset, not known yet */
    cff_dict_add((*cffont).topdict,
                 b"FDArray\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"FDArray\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    /* FDSelect - offset, not known yet */
    cff_dict_add((*cffont).topdict,
                 b"FDSelect\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"FDSelect\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    cff_dict_add((*cffont).topdict,
                 b"charset\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_set((*cffont).topdict,
                 b"charset\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    cff_dict_add((*cffont).topdict,
                 b"CharStrings\x00" as *const u8 as *const libc::c_char,
                 1i32);
    cff_dict_set((*cffont).topdict,
                 b"CharStrings\x00" as *const u8 as *const libc::c_char, 0i32,
                 0.0f64);
    let mut cstring: *mut cff_index = 0 as *mut cff_index;
    let mut gm: t1_ginfo =
        t1_ginfo{use_seac: 0,
                 wx: 0.,
                 wy: 0.,
                 bbox: C2RustUnnamed_7{llx: 0., lly: 0., urx: 0., ury: 0.,},
                 seac:
                     C2RustUnnamed_6{asb: 0.,
                                     adx: 0.,
                                     ady: 0.,
                                     bchar: 0,
                                     achar: 0,},};
    let mut max: libc::c_int = 0i32;
    let mut widths: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w_stat: [libc::c_int; 1001] = [0; 1001];
    let mut max_count: libc::c_int = 0;
    let mut dw: libc::c_int = 0;
    widths =
        new((num_glyphs as uint32_t as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_double>()
                                                 as libc::c_ulong) as
                uint32_t) as *mut libc::c_double;
    memset(w_stat.as_mut_ptr() as *mut libc::c_void, 0i32,
           (::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong).wrapping_mul(1001i32 as libc::c_ulong));
    offset = 0i64 as libc::c_int;
    cstring = cff_new_index(num_glyphs as card16);
    (*cstring).data = 0 as *mut card8;
    *(*cstring).offset.offset(0) = 1i32 as l_offset;
    gid = 0i32 as card16;
    cid = 0i32 as card16;
    while cid as libc::c_int <= last_cid as libc::c_int {
        if !(*used_chars.offset((cid as libc::c_int / 8i32) as isize) as
                 libc::c_int & 1i32 << 7i32 - cid as libc::c_int % 8i32 == 0)
           {
            if offset + 65536i32 >= max {
                max += 65536i32 * 2i32;
                (*cstring).data =
                    renew((*cstring).data as *mut libc::c_void,
                          (max as uint32_t as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<card8>()
                                                               as
                                                               libc::c_ulong)
                              as uint32_t) as *mut card8
            }
            offset +=
                t1char_convert_charstring((*cstring).data.offset(*(*cstring).offset.offset(gid
                                                                                               as
                                                                                               isize)
                                                                     as
                                                                     isize).offset(-1),
                                          65536i32,
                                          (*(*cffont).cstrings).data.offset(*(*(*cffont).cstrings).offset.offset(cid
                                                                                                                     as
                                                                                                                     isize)
                                                                                as
                                                                                isize).offset(-1),
                                          (*(*(*cffont).cstrings).offset.offset((cid
                                                                                     as
                                                                                     libc::c_int
                                                                                     +
                                                                                     1i32)
                                                                                    as
                                                                                    isize)).wrapping_sub(*(*(*cffont).cstrings).offset.offset(cid
                                                                                                                                                  as
                                                                                                                                                  isize))
                                              as libc::c_int,
                                          *(*cffont).subrs.offset(0),
                                          defaultwidth, nominalwidth,
                                          &mut gm);
            *(*cstring).offset.offset((gid as libc::c_int + 1i32) as isize) =
                (offset + 1i32) as l_offset;
            if gm.use_seac != 0 {
                _tt_abort(b"This font using the \"seac\" command for accented characters...\x00"
                              as *const u8 as *const libc::c_char);
            }
            *widths.offset(gid as isize) = gm.wx;
            if gm.wx >= 0.0f64 && gm.wx <= 1000.0f64 {
                w_stat[gm.wx as libc::c_int as usize] += 1i32
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
        add_metrics(font, cffont, CIDToGIDMap, widths, dw as libc::c_double,
                    last_cid);
    } else {
        add_metrics(font, cffont, CIDToGIDMap, widths, defaultwidth,
                    last_cid);
    }
    free(widths as *mut libc::c_void);
    cff_release_index(*(*cffont).subrs.offset(0));
    let ref mut fresh8 = *(*cffont).subrs.offset(0);
    *fresh8 = 0 as *mut cff_index;
    free(CIDToGIDMap as *mut libc::c_void);
    cff_add_string(cffont, b"Adobe\x00" as *const u8 as *const libc::c_char,
                   1i32);
    cff_add_string(cffont,
                   b"Identity\x00" as *const u8 as *const libc::c_char, 1i32);
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(*(*cffont).private.offset(0), cffont);
    cff_update_string(cffont);
    /* CFF code need to be rewrote... */
    cff_dict_add((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 3i32);
    cff_dict_set((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 0i32,
                 cff_get_sid(cffont,
                             b"Adobe\x00" as *const u8 as *const libc::c_char)
                     as libc::c_double);
    cff_dict_set((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 1i32,
                 cff_get_sid(cffont,
                             b"Identity\x00" as *const u8 as
                                 *const libc::c_char) as libc::c_double);
    cff_dict_set((*cffont).topdict,
                 b"ROS\x00" as *const u8 as *const libc::c_char, 2i32,
                 0.0f64);
    (*cffont).num_glyphs = num_glyphs as card16;
    offset = write_fontfile(font, cffont);
    cff_close(cffont);
    CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
