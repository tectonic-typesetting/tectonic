#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types)]
extern crate libc;
extern "C" {
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(path: *const libc::c_char,
                         format: tt_input_format_type, is_gz: libc::c_int)
     -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t,
                         whence: libc::c_int) -> size_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
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
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn check_for_pdf(handle: rust_input_handle_t) -> libc::c_int;
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
    static mut work_buffer: [libc::c_char; 0];
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(buffer: *mut libc::c_char, length: libc::c_int,
                 file: rust_input_handle_t) -> *mut libc::c_char;
    #[no_mangle]
    fn bmp_include_image(ximage: *mut pdf_ximage, handle: rust_input_handle_t)
     -> libc::c_int;
    #[no_mangle]
    fn check_for_bmp(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn dpx_delete_temp_file(tmp: *mut libc::c_char, force: libc::c_int);
    /* tmp freed here */
    #[no_mangle]
    static mut keep_cache: libc::c_int;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn min4(v1: libc::c_double, v2: libc::c_double, v3: libc::c_double,
            v4: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn max4(v1: libc::c_double, v2: libc::c_double, v3: libc::c_double,
            v4: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pdf_include_page(ximage: *mut pdf_ximage, handle: rust_input_handle_t,
                        ident: *const libc::c_char, options: load_options)
     -> libc::c_int;
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
    fn check_for_jpeg(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn jpeg_include_image(ximage: *mut pdf_ximage,
                          handle: rust_input_handle_t) -> libc::c_int;
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
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
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
    fn png_include_image(ximage: *mut pdf_ximage, handle: rust_input_handle_t)
     -> libc::c_int;
    #[no_mangle]
    fn check_for_png(handle: rust_input_handle_t) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub f: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct pdf_coord {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct transform_info {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub depth: libc::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ximage_info {
    pub flags: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bits_per_component: libc::c_int,
    pub num_components: libc::c_int,
    pub min_dpi: libc::c_int,
    pub xdensity: libc::c_double,
    pub ydensity: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct xform_info {
    pub flags: libc::c_int,
    pub bbox: pdf_rect,
    pub matrix: pdf_tmatrix,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct load_options {
    pub page_no: libc::c_int,
    pub bbox_type: libc::c_int,
    pub dict: *mut pdf_obj,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct pdf_ximage_ {
    pub ident: *mut libc::c_char,
    pub res_name: [libc::c_char; 16],
    pub subtype: libc::c_int,
    pub attr: attr_,
    pub filename: *mut libc::c_char,
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct attr_ {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub xdensity: libc::c_double,
    pub ydensity: libc::c_double,
    pub bbox: pdf_rect,
    pub page_no: libc::c_int,
    pub page_count: libc::c_int,
    pub bbox_type: libc::c_int,
    pub dict: *mut pdf_obj,
    pub tempfile: libc::c_char,
}
pub type pdf_ximage = pdf_ximage_;
/* quasi-hack to get the primary input */
/* verbose, verbose, verbose... */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct opt_ {
    pub verbose: libc::c_int,
    pub cmdtmpl: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ic_ {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub ximages: *mut pdf_ximage,
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
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char,
                               mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32
    } /* unsafe? */
    return 0i32 != 0;
}
#[inline]
unsafe extern "C" fn strstartswith(mut s: *const libc::c_char,
                                   mut prefix: *const libc::c_char)
 -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 { return s.offset(length as isize) }
    return 0 as *const libc::c_char;
}
static mut _opts: opt_ =
    {
        let mut init =
            opt_{verbose: 0i32,
                 cmdtmpl: 0 as *const libc::c_char as *mut libc::c_char,};
        init
    };
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_verbose(mut level: libc::c_int) {
    _opts.verbose = level;
}
static mut _ic: ic_ =
    {
        let mut init =
            ic_{count: 0i32,
                capacity: 0i32,
                ximages: 0 as *const pdf_ximage as *mut pdf_ximage,};
        init
    };
unsafe extern "C" fn pdf_init_ximage_struct(mut I: *mut pdf_ximage) {
    (*I).ident = 0 as *mut libc::c_char;
    (*I).filename = 0 as *mut libc::c_char;
    (*I).subtype = -1i32;
    memset((*I).res_name.as_mut_ptr() as *mut libc::c_void, 0i32,
           16i32 as libc::c_ulong);
    (*I).reference = 0 as *mut pdf_obj;
    (*I).resource = 0 as *mut pdf_obj;
    (*I).attr.height = 0i32;
    (*I).attr.width = (*I).attr.height;
    (*I).attr.ydensity = 1.0f64;
    (*I).attr.xdensity = (*I).attr.ydensity;
    (*I).attr.bbox.lly = 0i32 as libc::c_double;
    (*I).attr.bbox.llx = (*I).attr.bbox.lly;
    (*I).attr.bbox.ury = 0i32 as libc::c_double;
    (*I).attr.bbox.urx = (*I).attr.bbox.ury;
    (*I).attr.page_no = 1i32;
    (*I).attr.page_count = 1i32;
    (*I).attr.bbox_type = 0i32;
    (*I).attr.dict = 0 as *mut pdf_obj;
    (*I).attr.tempfile = 0i32 as libc::c_char;
}
unsafe extern "C" fn pdf_clean_ximage_struct(mut I: *mut pdf_ximage) {
    free((*I).ident as *mut libc::c_void);
    free((*I).filename as *mut libc::c_void);
    pdf_release_obj((*I).reference);
    pdf_release_obj((*I).resource);
    pdf_release_obj((*I).attr.dict);
    pdf_init_ximage_struct(I);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_images() {
    let mut ic: *mut ic_ = &mut _ic;
    (*ic).count = 0i32;
    (*ic).capacity = 0i32;
    (*ic).ximages = 0 as *mut pdf_ximage;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close_images() {
    let mut ic: *mut ic_ = &mut _ic;
    if !(*ic).ximages.is_null() {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < (*ic).count {
            let mut I: *mut pdf_ximage = (*ic).ximages.offset(i as isize);
            if (*I).attr.tempfile != 0 {
                /*
                 * It is important to remove temporary files at the end because
                 * we cache file names. Since we use mkstemp to create them, we
                 * might get the same file name again if we delete the first file.
                 * (This happens on NetBSD, reported by Jukka Salmi.)
                 * We also use this to convert a PS file only once if multiple
                 * pages are imported from that file.
                 */
                if _opts.verbose > 1i32 && keep_cache != 1i32 {
                    dpx_message(b"pdf_image>> deleting temporary file \"%s\"\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*I).filename); /* temporary filename freed here */
                }
                dpx_delete_temp_file((*I).filename, 0i32);
                (*I).filename = 0 as *mut libc::c_char
            }
            pdf_clean_ximage_struct(I);
            i += 1
        }
        (*ic).ximages =
            mfree((*ic).ximages as *mut libc::c_void) as *mut pdf_ximage;
        (*ic).capacity = 0i32;
        (*ic).count = (*ic).capacity
    }
    _opts.cmdtmpl =
        mfree(_opts.cmdtmpl as *mut libc::c_void) as *mut libc::c_char;
}
unsafe extern "C" fn source_image_type(mut handle: rust_input_handle_t)
 -> libc::c_int {
    let mut format: libc::c_int = -1i32;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    /* Original check order: jpeg, jp2, png, bmp, pdf, ps */
    if check_for_jpeg(handle) != 0 {
        format = 1i32
    } else if check_for_png(handle) != 0 {
        format = 2i32
    } else if check_for_bmp(handle) != 0 {
        format = 6i32
    } else if check_for_pdf(handle) != 0 {
        format = 0i32
    } else if check_for_ps(handle) != 0 {
        format = 5i32
    } else {
        dpx_warning(b"Tectonic was unable to detect an image\'s format\x00" as
                        *const u8 as *const libc::c_char);
        format = -1i32
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    return format;
}
unsafe extern "C" fn load_image(mut ident: *const libc::c_char,
                                mut fullname: *const libc::c_char,
                                mut format: libc::c_int,
                                mut handle: rust_input_handle_t,
                                mut options: load_options) -> libc::c_int {
    let mut current_block: u64;
    let mut ic: *mut ic_ = &mut _ic;
    let mut id: libc::c_int = -1i32;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    id = (*ic).count;
    if (*ic).count >= (*ic).capacity {
        (*ic).capacity += 16i32;
        (*ic).ximages =
            renew((*ic).ximages as *mut libc::c_void,
                  ((*ic).capacity as uint32_t as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<pdf_ximage>()
                                                       as libc::c_ulong) as
                      uint32_t) as *mut pdf_ximage
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    pdf_init_ximage_struct(I);
    if !ident.is_null() {
        (*I).ident =
            new((strlen(ident).wrapping_add(1i32 as libc::c_ulong) as uint32_t
                     as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy((*I).ident, ident);
    }
    if !fullname.is_null() {
        (*I).filename =
            new((strlen(fullname).wrapping_add(1i32 as libc::c_ulong) as
                     uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy((*I).filename, fullname);
    }
    (*I).attr.page_no = options.page_no;
    (*I).attr.bbox_type = options.bbox_type;
    /* else if (check_for_jp2(fp))
     *    format = IMAGE_TYPE_JP2; */
    (*I).attr.dict = options.dict; /* unsafe? */
    match format {
        1 => {
            if _opts.verbose != 0 {
                dpx_message(b"[JPEG]\x00" as *const u8 as
                                *const libc::c_char);
            }
            if jpeg_include_image(I, handle) < 0i32 {
                current_block = 15386155914718490365;
            } else {
                (*I).subtype = 1i32;
                current_block = 14945149239039849694;
            }
        }
        7 => {
            if _opts.verbose != 0 {
                dpx_message(b"[JP2]\x00" as *const u8 as *const libc::c_char);
            }
            /*if (jp2_include_image(I, fp) < 0)*/
            dpx_warning(b"Tectonic: JP2 not yet supported\x00" as *const u8 as
                            *const libc::c_char);
            current_block = 15386155914718490365;
        }
        2 => {
            /*I->subtype = PDF_XOBJECT_TYPE_IMAGE;
          break;*/
            if _opts.verbose != 0 {
                dpx_message(b"[PNG]\x00" as *const u8 as *const libc::c_char);
            }
            if png_include_image(I, handle) < 0i32 {
                current_block = 15386155914718490365;
            } else {
                (*I).subtype = 1i32;
                current_block = 14945149239039849694;
            }
        }
        6 => {
            if _opts.verbose != 0 {
                dpx_message(b"[BMP]\x00" as *const u8 as *const libc::c_char);
            }
            if bmp_include_image(I, handle) < 0i32 {
                current_block = 15386155914718490365;
            } else {
                (*I).subtype = 1i32;
                current_block = 14945149239039849694;
            }
        }
        0 => {
            if _opts.verbose != 0 {
                dpx_message(b"[PDF]\x00" as *const u8 as *const libc::c_char);
            }
            let mut result: libc::c_int =
                pdf_include_page(I, handle, fullname, options);
            /* Tectonic: this used to try ps_include_page() */
            if result != 0i32 {
                current_block = 15386155914718490365;
            } else {
                if _opts.verbose != 0 {
                    dpx_message(b",Page:%d\x00" as *const u8 as
                                    *const libc::c_char, (*I).attr.page_no);
                }
                (*I).subtype = 0i32;
                current_block = 14945149239039849694;
            }
        }
        5 => {
            if _opts.verbose != 0 {
                dpx_message(b"[EPS]\x00" as *const u8 as *const libc::c_char);
            }
            dpx_warning(b"sorry, PostScript images are not supported by Tectonic\x00"
                            as *const u8 as *const libc::c_char);
            dpx_warning(b"for details, please see https://github.com/tectonic-typesetting/tectonic/issues/27\x00"
                            as *const u8 as *const libc::c_char);
            current_block = 15386155914718490365;
        }
        _ => {
            if _opts.verbose != 0 {
                dpx_message(b"[UNKNOWN]\x00" as *const u8 as
                                *const libc::c_char);
            }
            current_block = 15386155914718490365;
        }
    }
    match current_block {
        15386155914718490365 =>
        /* Tectonic: this used to try ps_include_page() */
        {
            pdf_clean_ximage_struct(I);
            return -1i32
        }
        _ => {
            match (*I).subtype {
                1 => {
                    sprintf((*I).res_name.as_mut_ptr(),
                            b"Im%d\x00" as *const u8 as *const libc::c_char,
                            id);
                }
                0 => {
                    sprintf((*I).res_name.as_mut_ptr(),
                            b"Fm%d\x00" as *const u8 as *const libc::c_char,
                            id);
                }
                _ => {
                    _tt_abort(b"Unknown XObject subtype: %d\x00" as *const u8
                                  as *const libc::c_char, (*I).subtype);
                }
            }
            (*ic).count += 1;
            return id
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_findresource(mut ident:
                                                     *const libc::c_char,
                                                 mut options: load_options)
 -> libc::c_int {
    let mut ic: *mut ic_ = &mut _ic;
    let mut id: libc::c_int = -1i32;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    let mut format: libc::c_int = 0;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    /* "I don't understand why there is comparision against I->attr.dict here...
     * I->attr.dict and options.dict are simply pointers to PDF dictionaries."
     */
    id = 0i32;
    while id < (*ic).count {
        I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
        if !(*I).ident.is_null() &&
               streq_ptr(ident, (*I).ident) as libc::c_int != 0 {
            if (*I).attr.page_no == options.page_no &&
                   (*I).attr.dict == options.dict &&
                   (*I).attr.bbox_type == options.bbox_type {
                return id
            }
        }
        id += 1
    }
    /* This happens if we've already inserted the image into the PDF output.
     * In my one test case, it seems to just work to plunge along merrily
     * ahead ...
     *
     * if (f) {
     *   <"we already have converted this file; f is the temporary file name">
     *   fullname = NEW(strlen(f)+1, char);
     *   strcpy(fullname, f);
     * } else { kpse_find_file() }
     */
    handle = ttstub_input_open(ident, TTIF_PICT, 0i32);
    if handle.is_null() {
        dpx_warning(b"Error locating image file \"%s\"\x00" as *const u8 as
                        *const libc::c_char, ident);
        return -1i32
    }
    if _opts.verbose != 0 {
        dpx_message(b"(Image:%s\x00" as *const u8 as *const libc::c_char,
                    ident);
    }
    format = source_image_type(handle);
    id = load_image(ident, ident, format, handle, options);
    ttstub_input_close(handle);
    if _opts.verbose != 0 {
        dpx_message(b")\x00" as *const u8 as *const libc::c_char);
    }
    if id < 0i32 {
        dpx_warning(b"pdf: image inclusion failed for \"%s\".\x00" as
                        *const u8 as *const libc::c_char, ident);
    }
    return id;
}
/* Reference: PDF Reference 1.5 v6, pp.321--322
 *
 * TABLE 4.42 Additional entries specific to a type 1 form dictionary
 *
 * BBox rectangle (Required) An array of four numbers in the form coordinate
 *                system, giving the coordinates of the left, bottom, right,
 *                and top edges, respectively, of the form XObject's bounding
 *                box. These boundaries are used to clip the form XObject and
 *                to determine its size for caching.
 *
 * Matrix array   (Optional) An array of six numbers specifying the form
 *                matrix, which maps form space into user space.
 *                Default value: the identity matrix [1 0 0 1 0 0].
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_init_form_info(mut info:
                                                       *mut xform_info) {
    (*info).flags = 0i32;
    (*info).bbox.llx = 0i32 as libc::c_double;
    (*info).bbox.lly = 0i32 as libc::c_double;
    (*info).bbox.urx = 0i32 as libc::c_double;
    (*info).bbox.ury = 0i32 as libc::c_double;
    (*info).matrix.a = 1.0f64;
    (*info).matrix.b = 0.0f64;
    (*info).matrix.c = 0.0f64;
    (*info).matrix.d = 1.0f64;
    (*info).matrix.e = 0.0f64;
    (*info).matrix.f = 0.0f64;
}
/* Reference: PDF Reference 1.5 v6, pp.303--306
 *
 * TABLE 4.42 Additional entries specific to an image dictionary
 *
 * Width integer  (Required) The width of the image, in samples.
 *
 * Height integer (Required) The height of the image, in samples.
 *
 * ColorSpace name or array
 *                (Required for images, except those that use the JPXDecode
 *                filter; not allowed for image masks) The color space in
 *                which image samples are specified. This may be any type
 *                of color space except Patter.
 *
 *                If the image uses the JPXDecode filter, this entry is
 *                optional.
 *
 * BitsPerComponent integer
 *                (Required except for image masks and images that use the
 *                JPXDecode filter) The number of bits used to represent
 *                each color component. Only a single value may be specified;
 *                the number of bits is the same for all color components.
 *                Valid values are 1,2,4,8, and (in PDF1.5) 16. If ImageMask
 *                is true, this entry is optional, and if speficified, its
 *                value must be 1.
 *
 *                If the image stream uses the JPXDecode filter, this entry
 *                is optional and ignored if present. The bit depth is
 *                determined in the process of decoding the JPEG2000 image.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_init_image_info(mut info:
                                                        *mut ximage_info) {
    (*info).flags = 0i32; /* The width of the image, in samples */
    (*info).width = 0i32; /* The height of the image, in samples */
    (*info).height = 0i32;
    (*info).bits_per_component = 0i32;
    (*info).num_components = 0i32;
    (*info).min_dpi = 0i32;
    (*info).ydensity = 1.0f64;
    (*info).xdensity = (*info).ydensity;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_image(mut I: *mut pdf_ximage,
                                              mut image_info:
                                                  *mut libc::c_void,
                                              mut resource: *mut pdf_obj) {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut info: *mut ximage_info = image_info as *mut ximage_info;
    if !(!resource.is_null() && pdf_obj_typeof(resource) == 7i32) {
        _tt_abort(b"Image XObject must be of stream type.\x00" as *const u8 as
                      *const libc::c_char);
    }
    (*I).subtype = 1i32;
    (*I).attr.width = (*info).width;
    (*I).attr.height = (*info).height;
    (*I).attr.xdensity = (*info).xdensity;
    (*I).attr.ydensity = (*info).ydensity;
    (*I).reference = pdf_ref_obj(resource);
    dict = pdf_stream_dict(resource);
    pdf_add_dict(dict,
                 pdf_new_name(b"Type\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"XObject\x00" as *const u8 as
                                  *const libc::c_char));
    pdf_add_dict(dict,
                 pdf_new_name(b"Subtype\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_name(b"Image\x00" as *const u8 as
                                  *const libc::c_char));
    pdf_add_dict(dict,
                 pdf_new_name(b"Width\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number((*info).width as libc::c_double));
    pdf_add_dict(dict,
                 pdf_new_name(b"Height\x00" as *const u8 as
                                  *const libc::c_char),
                 pdf_new_number((*info).height as libc::c_double));
    if (*info).bits_per_component > 0i32 {
        /* Ignored for JPXDecode filter. FIXME */
        pdf_add_dict(dict,
                     pdf_new_name(b"BitsPerComponent\x00" as *const u8 as
                                      *const libc::c_char),
                     pdf_new_number((*info).bits_per_component as
                                        libc::c_double)); /* Caller don't know we are using reference. */
    }
    if !(*I).attr.dict.is_null() { pdf_merge_dict(dict, (*I).attr.dict); }
    pdf_release_obj(resource);
    (*I).resource = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_form(mut I: *mut pdf_ximage,
                                             mut form_info: *mut libc::c_void,
                                             mut resource: *mut pdf_obj) {
    let mut info: *mut xform_info = form_info as *mut xform_info;
    let mut p1: pdf_coord = pdf_coord{x: 0., y: 0.,};
    let mut p2: pdf_coord = pdf_coord{x: 0., y: 0.,};
    let mut p3: pdf_coord = pdf_coord{x: 0., y: 0.,};
    let mut p4: pdf_coord = pdf_coord{x: 0., y: 0.,};
    (*I).subtype = 0i32;
    /* Image's attribute "bbox" here is affected by /Rotate entry of included
     * PDF page.
     */
    p1.x = (*info).bbox.llx; /* Caller don't know we are using reference. */
    p1.y = (*info).bbox.lly;
    pdf_dev_transform(&mut p1, &mut (*info).matrix);
    p2.x = (*info).bbox.urx;
    p1.y = (*info).bbox.lly;
    pdf_dev_transform(&mut p2, &mut (*info).matrix);
    p3.x = (*info).bbox.urx;
    p3.y = (*info).bbox.ury;
    pdf_dev_transform(&mut p3, &mut (*info).matrix);
    p4.x = (*info).bbox.llx;
    p4.y = (*info).bbox.ury;
    pdf_dev_transform(&mut p4, &mut (*info).matrix);
    (*I).attr.bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
    (*I).attr.bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
    (*I).attr.bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
    (*I).attr.bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);
    (*I).reference = pdf_ref_obj(resource);
    pdf_release_obj(resource);
    (*I).resource = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_page(mut I: *mut pdf_ximage)
 -> libc::c_int {
    return (*I).attr.page_no;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_reference(mut id: libc::c_int)
 -> *mut pdf_obj {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as
                      *const libc::c_char, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    if (*I).reference.is_null() {
        (*I).reference = pdf_ref_obj((*I).resource)
    }
    return pdf_link_obj((*I).reference);
}
/* called from pdfdoc.c only for late binding */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_defineresource(mut ident:
                                                       *const libc::c_char,
                                                   mut subtype: libc::c_int,
                                                   mut info:
                                                       *mut libc::c_void,
                                                   mut resource: *mut pdf_obj)
 -> libc::c_int {
    let mut ic: *mut ic_ = &mut _ic;
    let mut id: libc::c_int = 0;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    id = (*ic).count;
    if (*ic).count >= (*ic).capacity {
        (*ic).capacity += 16i32;
        (*ic).ximages =
            renew((*ic).ximages as *mut libc::c_void,
                  ((*ic).capacity as uint32_t as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<pdf_ximage>()
                                                       as libc::c_ulong) as
                      uint32_t) as *mut pdf_ximage
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    pdf_init_ximage_struct(I);
    if !ident.is_null() {
        (*I).ident =
            new((strlen(ident).wrapping_add(1i32 as libc::c_ulong) as uint32_t
                     as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy((*I).ident, ident);
    }
    match subtype {
        1 => {
            pdf_ximage_set_image(I, info, resource);
            sprintf((*I).res_name.as_mut_ptr(),
                    b"Im%d\x00" as *const u8 as *const libc::c_char, id);
        }
        0 => {
            pdf_ximage_set_form(I, info, resource);
            sprintf((*I).res_name.as_mut_ptr(),
                    b"Fm%d\x00" as *const u8 as *const libc::c_char, id);
        }
        _ => {
            _tt_abort(b"Unknown XObject subtype: %d\x00" as *const u8 as
                          *const libc::c_char, subtype);
        }
    }
    (*ic).count += 1;
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_resname(mut id: libc::c_int)
 -> *mut libc::c_char {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as
                      *const libc::c_char, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    return (*I).res_name.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_subtype(mut id: libc::c_int)
 -> libc::c_int {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as
                      *const libc::c_char, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    return (*I).subtype;
}
/* from spc_pdfm.c */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_attr(mut id: libc::c_int,
                                             mut width: libc::c_int,
                                             mut height: libc::c_int,
                                             mut xdensity: libc::c_double,
                                             mut ydensity: libc::c_double,
                                             mut llx: libc::c_double,
                                             mut lly: libc::c_double,
                                             mut urx: libc::c_double,
                                             mut ury: libc::c_double) {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as
                      *const libc::c_char, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    (*I).attr.width = width;
    (*I).attr.height = height;
    (*I).attr.xdensity = xdensity;
    (*I).attr.ydensity = ydensity;
    (*I).attr.bbox.llx = llx;
    (*I).attr.bbox.lly = lly;
    (*I).attr.bbox.urx = urx;
    (*I).attr.bbox.ury = ury;
}
/* depth...
 * Dvipdfm treat "depth" as "yoffset" for pdf:image and pdf:uxobj
 * not as vertical dimension of scaled image. (And there are bugs.)
 * This part contains incompatibile behaviour than dvipdfm!
 */
unsafe extern "C" fn scale_to_fit_I(mut T: *mut pdf_tmatrix,
                                    mut p: *mut transform_info,
                                    mut I: *mut pdf_ximage) {
    let mut s_x: libc::c_double = 0.;
    let mut s_y: libc::c_double = 0.;
    let mut d_x: libc::c_double = 0.;
    let mut d_y: libc::c_double = 0.;
    let mut wd0: libc::c_double = 0.;
    let mut ht0: libc::c_double = 0.;
    let mut dp: libc::c_double = 0.;
    let mut xscale: libc::c_double = 0.;
    let mut yscale: libc::c_double = 0.;
    if (*p).flags & 1i32 << 0i32 != 0 {
        wd0 = (*p).bbox.urx - (*p).bbox.llx;
        ht0 = (*p).bbox.ury - (*p).bbox.lly;
        xscale = (*I).attr.width as libc::c_double * (*I).attr.xdensity / wd0;
        yscale =
            (*I).attr.height as libc::c_double * (*I).attr.ydensity / ht0;
        d_x = -(*p).bbox.llx / wd0;
        d_y = -(*p).bbox.lly / ht0
    } else {
        wd0 = (*I).attr.width as libc::c_double * (*I).attr.xdensity;
        ht0 = (*I).attr.height as libc::c_double * (*I).attr.ydensity;
        yscale = 1.0f64;
        xscale = yscale;
        d_x = 0.0f64;
        d_y = 0.0f64
    }
    if wd0 == 0.0f64 {
        dpx_warning(b"Image width=0.0!\x00" as *const u8 as
                        *const libc::c_char);
        wd0 = 1.0f64
    }
    if ht0 == 0.0f64 {
        dpx_warning(b"Image height=0.0!\x00" as *const u8 as
                        *const libc::c_char);
        ht0 = 1.0f64
    }
    if (*p).flags & 1i32 << 1i32 != 0 && (*p).flags & 1i32 << 2i32 != 0 {
        s_x = (*p).width * xscale;
        s_y = ((*p).height + (*p).depth) * yscale;
        dp = (*p).depth * yscale
    } else if (*p).flags & 1i32 << 1i32 != 0 {
        s_x = (*p).width * xscale;
        s_y =
            s_x *
                ((*I).attr.height as libc::c_double /
                     (*I).attr.width as libc::c_double);
        dp = 0.0f64
    } else if (*p).flags & 1i32 << 2i32 != 0 {
        s_y = ((*p).height + (*p).depth) * yscale;
        s_x =
            s_y *
                ((*I).attr.width as libc::c_double /
                     (*I).attr.height as libc::c_double);
        dp = (*p).depth * yscale
    } else { s_x = wd0; s_y = ht0; dp = 0.0f64 }
    (*T).a = s_x;
    (*T).c = 0.0f64;
    (*T).b = 0.0f64;
    (*T).d = s_y;
    (*T).e = d_x * s_x / xscale;
    (*T).f = d_y * s_y / yscale - dp;
}
unsafe extern "C" fn scale_to_fit_F(mut T: *mut pdf_tmatrix,
                                    mut p: *mut transform_info,
                                    mut I: *mut pdf_ximage) {
    let mut s_x: libc::c_double = 0.;
    let mut s_y: libc::c_double = 0.;
    let mut d_x: libc::c_double = 0.;
    let mut d_y: libc::c_double = 0.;
    let mut wd0: libc::c_double = 0.;
    let mut ht0: libc::c_double = 0.;
    let mut dp: libc::c_double = 0.;
    if (*p).flags & 1i32 << 0i32 != 0 {
        wd0 = (*p).bbox.urx - (*p).bbox.llx;
        ht0 = (*p).bbox.ury - (*p).bbox.lly;
        d_x = -(*p).bbox.llx;
        d_y = -(*p).bbox.lly
    } else {
        wd0 = (*I).attr.bbox.urx - (*I).attr.bbox.llx;
        ht0 = (*I).attr.bbox.ury - (*I).attr.bbox.lly;
        d_x = 0.0f64;
        d_y = 0.0f64
    }
    if wd0 == 0.0f64 {
        dpx_warning(b"Image width=0.0!\x00" as *const u8 as
                        *const libc::c_char);
        wd0 = 1.0f64
    }
    if ht0 == 0.0f64 {
        dpx_warning(b"Image height=0.0!\x00" as *const u8 as
                        *const libc::c_char);
        ht0 = 1.0f64
    }
    if (*p).flags & 1i32 << 1i32 != 0 && (*p).flags & 1i32 << 2i32 != 0 {
        s_x = (*p).width / wd0;
        s_y = ((*p).height + (*p).depth) / ht0;
        dp = (*p).depth
    } else if (*p).flags & 1i32 << 1i32 != 0 {
        s_x = (*p).width / wd0;
        s_y = s_x;
        dp = 0.0f64
    } else if (*p).flags & 1i32 << 2i32 != 0 {
        s_y = ((*p).height + (*p).depth) / ht0;
        s_x = s_y;
        dp = (*p).depth
    } else { s_y = 1.0f64; s_x = s_y; dp = 0.0f64 }
    (*T).a = s_x;
    (*T).c = 0.0f64;
    (*T).b = 0.0f64;
    (*T).d = s_y;
    (*T).e = s_x * d_x;
    (*T).f = s_y * d_y - dp;
}
/* called from pdfdev.c and spc_html.c */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_scale_image(mut id: libc::c_int,
                                                mut M: *mut pdf_tmatrix,
                                                mut r: *mut pdf_rect,
                                                mut p: *mut transform_info)
 -> libc::c_int 
 /* argument from specials */
 {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as
                      *const libc::c_char, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    (*M).a = 1.0f64;
    (*M).b = 0.0f64;
    (*M).c = 0.0f64;
    (*M).d = 1.0f64;
    (*M).e = 0.0f64;
    (*M).f = 0.0f64;
    match (*I).subtype {
        1 => {
            /* Reference: PDF Reference 1.5 v6, p.302
         *
         * An image can be placed on the output page in any desired position,
         * orientation, and size by using the cm operator to modify the current
         * transformation matrix (CTM) so as to map the unit square of user space
         * to the rectangle or parallelogram in which the image is to be painted.
         *
         * There is neither BBox nor Matrix key in the image XObject.
         * Everything must be controlled by the cm operator.
         *
         * The argument [p] contains the user-defined bounding box, the scailing
         * factor of which is bp as EPS and PDF. On the other hand, I->attr
         * contains the (sampling) width and the (sampling) height of the image.
         *
         * There is no problem if a bitmap image has density information.
         * Otherwise, DVIPDFM's ebb generates bounding box as 100px = 72bp = 1in.
         * In this case, screen captured images look bad. Moreover, DVIPDFM's ebb
         * ignores all density information and use just 100px = 72bp = 1in.
         *
         * On the other hand, pdfTeX uses 100px = 100bp to get a better quality
         * for screen captured images.
         *
         * DVIPDFMx's xbb generates bounding box as 100px = 100bp in the same
         * way as pdfTeX. Furthermore, it takes care of density information too.
         */
            scale_to_fit_I(M, p, I);
            if (*p).flags & 1i32 << 0i32 != 0 {
                (*r).llx =
                    (*p).bbox.llx /
                        ((*I).attr.width as libc::c_double *
                             (*I).attr.xdensity);
                (*r).lly =
                    (*p).bbox.lly /
                        ((*I).attr.height as libc::c_double *
                             (*I).attr.ydensity);
                (*r).urx =
                    (*p).bbox.urx /
                        ((*I).attr.width as libc::c_double *
                             (*I).attr.xdensity);
                (*r).ury =
                    (*p).bbox.ury /
                        ((*I).attr.height as libc::c_double *
                             (*I).attr.ydensity)
            } else {
                (*r).llx = 0.0f64;
                (*r).lly = 0.0f64;
                (*r).urx = 1.0f64;
                (*r).ury = 1.0f64
            }
        }
        0 => {
            /* User-defined transformation and clipping are controlled by
         * the cm operator and W operator, explicitly */
            scale_to_fit_F(M, p,
                           I); /* I->attr.bbox from the image bounding box */
            if (*p).flags & 1i32 << 0i32 != 0 {
                (*r).llx = (*p).bbox.llx;
                (*r).lly = (*p).bbox.lly;
                (*r).urx = (*p).bbox.urx;
                (*r).ury = (*p).bbox.ury
            } else {
                (*r).llx = (*I).attr.bbox.llx;
                (*r).lly = (*I).attr.bbox.lly;
                (*r).urx = (*I).attr.bbox.urx;
                (*r).ury = (*I).attr.bbox.ury
            }
        }
        _ => { }
    }
    return 0i32;
}
/* Migrated from psimage.c */
#[no_mangle]
pub unsafe extern "C" fn set_distiller_template(mut s: *mut libc::c_char) {
    free(_opts.cmdtmpl as *mut libc::c_void);
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 {
        _opts.cmdtmpl = 0 as *mut libc::c_char
    } else {
        _opts.cmdtmpl =
            new((strlen(s).wrapping_add(1i32 as libc::c_ulong) as uint32_t as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                     as libc::c_ulong) as
                    uint32_t) as *mut libc::c_char;
        strcpy(_opts.cmdtmpl, s);
    };
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
/* NOT USED YET */
/* scale factor for bp */
/* Please use different interface than findresource...
 * This is not intended to be used for specifying page number and others.
 * Only pdf:image special in spc_pdfm.c want optinal dict!
 */
/* Called by pngimage, jpegimage, epdf, mpost, etc. */
/* from pdfximage.c */
#[no_mangle]
pub unsafe extern "C" fn get_distiller_template() -> *mut libc::c_char {
    return _opts.cmdtmpl;
}
unsafe extern "C" fn check_for_ps(mut handle: rust_input_handle_t)
 -> libc::c_int {
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    tt_mfgets(work_buffer.as_mut_ptr(), 1024i32, handle);
    if !strstartswith(work_buffer.as_mut_ptr(),
                      b"%!\x00" as *const u8 as *const libc::c_char).is_null()
       {
        return 1i32
    }
    return 0i32;
}
