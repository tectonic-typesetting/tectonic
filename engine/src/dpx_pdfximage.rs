#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::{info, warn};

use super::dpx_pdfdraw::pdf_dev_transform;
use crate::dpx_pdfobj::{
    pdf_add_dict, pdf_link_obj, pdf_merge_dict, pdf_new_name, pdf_new_number, pdf_obj,
    pdf_obj_typeof, pdf_ref_obj, pdf_release_obj, pdf_stream_dict,
};
use crate::{ttstub_input_close, ttstub_input_open, ttstub_input_seek};
use libc::free;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    /* Name does not include the / */
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn check_for_pdf(handle: rust_input_handle_t) -> i32;
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
    static mut work_buffer: [i8; 0];
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(buffer: *mut i8, length: i32, file: rust_input_handle_t) -> *mut i8;
    #[no_mangle]
    fn bmp_include_image(ximage: *mut pdf_ximage, handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn check_for_bmp(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn dpx_delete_temp_file(tmp: *mut i8, force: i32);
    /* tmp freed here */
    #[no_mangle]
    static mut keep_cache: i32;
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
    fn min4(v1: f64, v2: f64, v3: f64, v4: f64) -> f64;
    #[no_mangle]
    fn max4(v1: f64, v2: f64, v3: f64, v4: f64) -> f64;
    #[no_mangle]
    fn pdf_include_page(
        ximage: *mut pdf_ximage,
        handle: rust_input_handle_t,
        ident: *const i8,
        options: load_options,
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
    fn check_for_jpeg(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn jpeg_include_image(ximage: *mut pdf_ximage, handle: rust_input_handle_t) -> i32;
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
    fn png_include_image(ximage: *mut pdf_ximage, handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn check_for_png(handle: rust_input_handle_t) -> i32;
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;

use crate::TTInputFormat;

pub type rust_input_handle_t = *mut libc::c_void;

use super::dpx_pdfdev::pdf_tmatrix;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}

use super::dpx_pdfdev::pdf_coord;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ximage_info {
    pub flags: i32,
    pub width: i32,
    pub height: i32,
    pub bits_per_component: i32,
    pub num_components: i32,
    pub min_dpi: i32,
    pub xdensity: f64,
    pub ydensity: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xform_info {
    pub flags: i32,
    pub bbox: pdf_rect,
    pub matrix: pdf_tmatrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: i32,
    pub bbox_type: i32,
    pub dict: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_ximage {
    pub ident: *mut i8,
    pub res_name: [i8; 16],
    pub subtype: i32,
    pub attr: attr_,
    pub filename: *mut i8,
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_ {
    pub width: i32,
    pub height: i32,
    pub xdensity: f64,
    pub ydensity: f64,
    pub bbox: pdf_rect,
    pub page_no: i32,
    pub page_count: i32,
    pub bbox_type: i32,
    pub dict: *mut pdf_obj,
    pub tempfile: i8,
}
/* quasi-hack to get the primary input */
/* verbose, verbose, verbose... */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opt_ {
    pub verbose: i32,
    pub cmdtmpl: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ic_ {
    pub count: i32,
    pub capacity: i32,
    pub ximages: *mut pdf_ximage,
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
    } /* unsafe? */
    false
}
#[inline]
unsafe extern "C" fn strstartswith(mut s: *const i8, mut prefix: *const i8) -> *const i8 {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    0 as *const i8
}
static mut _opts: opt_ = {
    let mut init = opt_ {
        verbose: 0i32,
        cmdtmpl: 0 as *const i8 as *mut i8,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_verbose(mut level: i32) {
    _opts.verbose = level;
}
static mut _ic: ic_ = {
    let mut init = ic_ {
        count: 0i32,
        capacity: 0i32,
        ximages: 0 as *const pdf_ximage as *mut pdf_ximage,
    };
    init
};
unsafe extern "C" fn pdf_init_ximage_struct(mut I: *mut pdf_ximage) {
    (*I).ident = 0 as *mut i8;
    (*I).filename = 0 as *mut i8;
    (*I).subtype = -1i32;
    memset(
        (*I).res_name.as_mut_ptr() as *mut libc::c_void,
        0i32,
        16i32 as u64,
    );
    (*I).reference = 0 as *mut pdf_obj;
    (*I).resource = 0 as *mut pdf_obj;
    (*I).attr.height = 0i32;
    (*I).attr.width = (*I).attr.height;
    (*I).attr.ydensity = 1.0f64;
    (*I).attr.xdensity = (*I).attr.ydensity;
    (*I).attr.bbox.lly = 0i32 as f64;
    (*I).attr.bbox.llx = (*I).attr.bbox.lly;
    (*I).attr.bbox.ury = 0i32 as f64;
    (*I).attr.bbox.urx = (*I).attr.bbox.ury;
    (*I).attr.page_no = 1i32;
    (*I).attr.page_count = 1i32;
    (*I).attr.bbox_type = 0i32;
    (*I).attr.dict = 0 as *mut pdf_obj;
    (*I).attr.tempfile = 0_i8;
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
        let mut i: i32 = 0;
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
                    dpx_message(
                        b"pdf_image>> deleting temporary file \"%s\"\n\x00" as *const u8
                            as *const i8,
                        (*I).filename,
                    ); /* temporary filename freed here */
                }
                dpx_delete_temp_file((*I).filename, 0i32);
                (*I).filename = 0 as *mut i8
            }
            pdf_clean_ximage_struct(I);
            i += 1
        }
        (*ic).ximages = mfree((*ic).ximages as *mut libc::c_void) as *mut pdf_ximage;
        (*ic).capacity = 0i32;
        (*ic).count = (*ic).capacity
    }
    _opts.cmdtmpl = mfree(_opts.cmdtmpl as *mut libc::c_void) as *mut i8;
}
unsafe extern "C" fn source_image_type(mut handle: rust_input_handle_t) -> i32 {
    let mut format: i32 = -1i32;
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
        warn!("Tectonic was unable to detect an image\'s format");
        format = -1i32
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    format
}
unsafe extern "C" fn load_image(
    mut ident: *const i8,
    mut fullname: *const i8,
    mut format: i32,
    mut handle: rust_input_handle_t,
    mut options: load_options,
) -> i32 {
    let mut current_block: u64;
    let mut ic: *mut ic_ = &mut _ic;
    let mut id: i32 = -1i32;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    id = (*ic).count;
    if (*ic).count >= (*ic).capacity {
        (*ic).capacity += 16i32;
        (*ic).ximages = renew(
            (*ic).ximages as *mut libc::c_void,
            ((*ic).capacity as u32 as u64).wrapping_mul(::std::mem::size_of::<pdf_ximage>() as u64)
                as u32,
        ) as *mut pdf_ximage
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    pdf_init_ximage_struct(I);
    if !ident.is_null() {
        (*I).ident = new((strlen(ident).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        strcpy((*I).ident, ident);
    }
    if !fullname.is_null() {
        (*I).filename = new((strlen(fullname).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
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
                info!("[JPEG]");
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
                info!("[JP2]");
            }
            /*if (jp2_include_image(I, fp) < 0)*/
            warn!("Tectonic: JP2 not yet supported");
            current_block = 15386155914718490365;
        }
        2 => {
            /*I->subtype = PDF_XOBJECT_TYPE_IMAGE;
            break;*/
            if _opts.verbose != 0 {
                info!("[PNG]");
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
                info!("[BMP]");
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
                info!("[PDF]");
            }
            let mut result: i32 = pdf_include_page(I, handle, fullname, options);
            /* Tectonic: this used to try ps_include_page() */
            if result != 0i32 {
                current_block = 15386155914718490365;
            } else {
                if _opts.verbose != 0 {
                    info!(",Page:{}", (*I).attr.page_no);
                }
                (*I).subtype = 0i32;
                current_block = 14945149239039849694;
            }
        }
        5 => {
            if _opts.verbose != 0 {
                info!("[EPS]");
            }
            warn!("sorry, PostScript images are not supported by Tectonic");
            warn!("for details, please see https://github.com/tectonic-typesetting/tectonic/issues/27");
            current_block = 15386155914718490365;
        }
        _ => {
            if _opts.verbose != 0 {
                info!("[UNKNOWN]");
            }
            current_block = 15386155914718490365;
        }
    }
    match current_block {
        15386155914718490365 =>
        /* Tectonic: this used to try ps_include_page() */
        {
            pdf_clean_ximage_struct(I);
            return -1i32;
        }
        _ => {
            match (*I).subtype {
                1 => {
                    sprintf(
                        (*I).res_name.as_mut_ptr(),
                        b"Im%d\x00" as *const u8 as *const i8,
                        id,
                    );
                }
                0 => {
                    sprintf(
                        (*I).res_name.as_mut_ptr(),
                        b"Fm%d\x00" as *const u8 as *const i8,
                        id,
                    );
                }
                _ => {
                    _tt_abort(
                        b"Unknown XObject subtype: %d\x00" as *const u8 as *const i8,
                        (*I).subtype,
                    );
                }
            }
            (*ic).count += 1;
            return id;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_findresource(
    mut ident: *const i8,
    mut options: load_options,
) -> i32 {
    let mut ic: *mut ic_ = &mut _ic;
    let mut id: i32 = -1i32;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    let mut format: i32 = 0;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    /* "I don't understand why there is comparision against I->attr.dict here...
     * I->attr.dict and options.dict are simply pointers to PDF dictionaries."
     */
    id = 0i32;
    while id < (*ic).count {
        I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
        if !(*I).ident.is_null() && streq_ptr(ident, (*I).ident) as i32 != 0 {
            if (*I).attr.page_no == options.page_no
                && (*I).attr.dict == options.dict
                && (*I).attr.bbox_type == options.bbox_type
            {
                return id;
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
    handle = ttstub_input_open(ident, TTInputFormat::PICT, 0i32);
    if handle.is_null() {
        dpx_warning(
            b"Error locating image file \"%s\"\x00" as *const u8 as *const i8,
            ident,
        );
        return -1i32;
    }
    if _opts.verbose != 0 {
        dpx_message(b"(Image:%s\x00" as *const u8 as *const i8, ident);
    }
    format = source_image_type(handle);
    id = load_image(ident, ident, format, handle, options);
    ttstub_input_close(handle);
    if _opts.verbose != 0 {
        info!(")");
    }
    if id < 0i32 {
        dpx_warning(
            b"pdf: image inclusion failed for \"%s\".\x00" as *const u8 as *const i8,
            ident,
        );
    }
    id
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
pub unsafe extern "C" fn pdf_ximage_init_form_info(mut info: *mut xform_info) {
    (*info).flags = 0i32;
    (*info).bbox.llx = 0i32 as f64;
    (*info).bbox.lly = 0i32 as f64;
    (*info).bbox.urx = 0i32 as f64;
    (*info).bbox.ury = 0i32 as f64;
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
pub unsafe extern "C" fn pdf_ximage_init_image_info(mut info: *mut ximage_info) {
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
pub unsafe extern "C" fn pdf_ximage_set_image(
    mut I: *mut pdf_ximage,
    mut image_info: *mut libc::c_void,
    mut resource: *mut pdf_obj,
) {
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut info: *mut ximage_info = image_info as *mut ximage_info;
    if !(!resource.is_null() && pdf_obj_typeof(resource) == 7i32) {
        _tt_abort(b"Image XObject must be of stream type.\x00" as *const u8 as *const i8);
    }
    (*I).subtype = 1i32;
    (*I).attr.width = (*info).width;
    (*I).attr.height = (*info).height;
    (*I).attr.xdensity = (*info).xdensity;
    (*I).attr.ydensity = (*info).ydensity;
    (*I).reference = pdf_ref_obj(resource);
    dict = pdf_stream_dict(resource);
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"XObject\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"Image\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Width\x00" as *const u8 as *const i8),
        pdf_new_number((*info).width as f64),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Height\x00" as *const u8 as *const i8),
        pdf_new_number((*info).height as f64),
    );
    if (*info).bits_per_component > 0i32 {
        /* Ignored for JPXDecode filter. FIXME */
        pdf_add_dict(
            dict,
            pdf_new_name(b"BitsPerComponent\x00" as *const u8 as *const i8),
            pdf_new_number((*info).bits_per_component as f64),
        ); /* Caller don't know we are using reference. */
    }
    if !(*I).attr.dict.is_null() {
        pdf_merge_dict(dict, (*I).attr.dict);
    }
    pdf_release_obj(resource);
    (*I).resource = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_form(
    mut I: *mut pdf_ximage,
    mut form_info: *mut libc::c_void,
    mut resource: *mut pdf_obj,
) {
    let mut info: *mut xform_info = form_info as *mut xform_info;
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p4: pdf_coord = pdf_coord { x: 0., y: 0. };
    (*I).subtype = 0i32;
    /* Image's attribute "bbox" here is affected by /Rotate entry of included
     * PDF page.
     */
    p1.x = (*info).bbox.llx; /* Caller don't know we are using reference. */
    p1.y = (*info).bbox.lly;
    pdf_dev_transform(&mut p1, Some(&(*info).matrix));
    p2.x = (*info).bbox.urx;
    p1.y = (*info).bbox.lly;
    pdf_dev_transform(&mut p2, Some(&(*info).matrix));
    p3.x = (*info).bbox.urx;
    p3.y = (*info).bbox.ury;
    pdf_dev_transform(&mut p3, Some(&(*info).matrix));
    p4.x = (*info).bbox.llx;
    p4.y = (*info).bbox.ury;
    pdf_dev_transform(&mut p4, Some(&(*info).matrix));
    (*I).attr.bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
    (*I).attr.bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
    (*I).attr.bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
    (*I).attr.bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);
    (*I).reference = pdf_ref_obj(resource);
    pdf_release_obj(resource);
    (*I).resource = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_page(mut I: *mut pdf_ximage) -> i32 {
    (*I).attr.page_no
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_reference(mut id: i32) -> *mut pdf_obj {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as *const i8, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    if (*I).reference.is_null() {
        (*I).reference = pdf_ref_obj((*I).resource)
    }
    pdf_link_obj((*I).reference)
}
/* called from pdfdoc.c only for late binding */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_defineresource(
    mut ident: *const i8,
    mut subtype: i32,
    mut info: *mut libc::c_void,
    mut resource: *mut pdf_obj,
) -> i32 {
    let mut ic: *mut ic_ = &mut _ic;
    let mut id: i32 = 0;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    id = (*ic).count;
    if (*ic).count >= (*ic).capacity {
        (*ic).capacity += 16i32;
        (*ic).ximages = renew(
            (*ic).ximages as *mut libc::c_void,
            ((*ic).capacity as u32 as u64).wrapping_mul(::std::mem::size_of::<pdf_ximage>() as u64)
                as u32,
        ) as *mut pdf_ximage
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    pdf_init_ximage_struct(I);
    if !ident.is_null() {
        (*I).ident = new((strlen(ident).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        strcpy((*I).ident, ident);
    }
    match subtype {
        1 => {
            pdf_ximage_set_image(I, info, resource);
            sprintf(
                (*I).res_name.as_mut_ptr(),
                b"Im%d\x00" as *const u8 as *const i8,
                id,
            );
        }
        0 => {
            pdf_ximage_set_form(I, info, resource);
            sprintf(
                (*I).res_name.as_mut_ptr(),
                b"Fm%d\x00" as *const u8 as *const i8,
                id,
            );
        }
        _ => {
            _tt_abort(
                b"Unknown XObject subtype: %d\x00" as *const u8 as *const i8,
                subtype,
            );
        }
    }
    (*ic).count += 1;
    id
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_resname(mut id: i32) -> *mut i8 {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as *const i8, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    (*I).res_name.as_mut_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_get_subtype(mut id: i32) -> i32 {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as *const i8, id);
    }
    I = &mut *(*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    (*I).subtype
}
/* from spc_pdfm.c */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_set_attr(
    mut id: i32,
    mut width: i32,
    mut height: i32,
    mut xdensity: f64,
    mut ydensity: f64,
    mut llx: f64,
    mut lly: f64,
    mut urx: f64,
    mut ury: f64,
) {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as *const i8, id);
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
unsafe extern "C" fn scale_to_fit_I(
    mut T: *mut pdf_tmatrix,
    mut p: *mut transform_info,
    mut I: *mut pdf_ximage,
) {
    let mut s_x: f64 = 0.;
    let mut s_y: f64 = 0.;
    let mut d_x: f64 = 0.;
    let mut d_y: f64 = 0.;
    let mut wd0: f64 = 0.;
    let mut ht0: f64 = 0.;
    let mut dp: f64 = 0.;
    let mut xscale: f64 = 0.;
    let mut yscale: f64 = 0.;
    if (*p).flags & 1i32 << 0i32 != 0 {
        wd0 = (*p).bbox.urx - (*p).bbox.llx;
        ht0 = (*p).bbox.ury - (*p).bbox.lly;
        xscale = (*I).attr.width as f64 * (*I).attr.xdensity / wd0;
        yscale = (*I).attr.height as f64 * (*I).attr.ydensity / ht0;
        d_x = -(*p).bbox.llx / wd0;
        d_y = -(*p).bbox.lly / ht0
    } else {
        wd0 = (*I).attr.width as f64 * (*I).attr.xdensity;
        ht0 = (*I).attr.height as f64 * (*I).attr.ydensity;
        yscale = 1.0f64;
        xscale = yscale;
        d_x = 0.0f64;
        d_y = 0.0f64
    }
    if wd0 == 0.0f64 {
        warn!("Image width=0.0!");
        wd0 = 1.0f64
    }
    if ht0 == 0.0f64 {
        warn!("Image height=0.0!");
        ht0 = 1.0f64
    }
    if (*p).flags & 1i32 << 1i32 != 0 && (*p).flags & 1i32 << 2i32 != 0 {
        s_x = (*p).width * xscale;
        s_y = ((*p).height + (*p).depth) * yscale;
        dp = (*p).depth * yscale
    } else if (*p).flags & 1i32 << 1i32 != 0 {
        s_x = (*p).width * xscale;
        s_y = s_x * ((*I).attr.height as f64 / (*I).attr.width as f64);
        dp = 0.0f64
    } else if (*p).flags & 1i32 << 2i32 != 0 {
        s_y = ((*p).height + (*p).depth) * yscale;
        s_x = s_y * ((*I).attr.width as f64 / (*I).attr.height as f64);
        dp = (*p).depth * yscale
    } else {
        s_x = wd0;
        s_y = ht0;
        dp = 0.0f64
    }
    (*T).a = s_x;
    (*T).c = 0.0f64;
    (*T).b = 0.0f64;
    (*T).d = s_y;
    (*T).e = d_x * s_x / xscale;
    (*T).f = d_y * s_y / yscale - dp;
}
unsafe extern "C" fn scale_to_fit_F(
    mut T: *mut pdf_tmatrix,
    mut p: *mut transform_info,
    mut I: *mut pdf_ximage,
) {
    let mut s_x: f64 = 0.;
    let mut s_y: f64 = 0.;
    let mut d_x: f64 = 0.;
    let mut d_y: f64 = 0.;
    let mut wd0: f64 = 0.;
    let mut ht0: f64 = 0.;
    let mut dp: f64 = 0.;
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
        warn!("Image width=0.0!");
        wd0 = 1.0f64
    }
    if ht0 == 0.0f64 {
        warn!("Image height=0.0!");
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
    } else {
        s_y = 1.0f64;
        s_x = s_y;
        dp = 0.0f64
    }
    (*T).a = s_x;
    (*T).c = 0.0f64;
    (*T).b = 0.0f64;
    (*T).d = s_y;
    (*T).e = s_x * d_x;
    (*T).f = s_y * d_y - dp;
}
/* called from pdfdev.c and spc_html.c */
#[no_mangle]
pub unsafe extern "C" fn pdf_ximage_scale_image(
    mut id: i32,
    mut M: *mut pdf_tmatrix,
    mut r: *mut pdf_rect,
    mut p: *mut transform_info,
) -> i32
/* argument from specials */ {
    let mut ic: *mut ic_ = &mut _ic;
    let mut I: *mut pdf_ximage = 0 as *mut pdf_ximage;
    if id < 0i32 || id >= (*ic).count {
        _tt_abort(b"Invalid XObject ID: %d\x00" as *const u8 as *const i8, id);
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
                (*r).llx = (*p).bbox.llx / ((*I).attr.width as f64 * (*I).attr.xdensity);
                (*r).lly = (*p).bbox.lly / ((*I).attr.height as f64 * (*I).attr.ydensity);
                (*r).urx = (*p).bbox.urx / ((*I).attr.width as f64 * (*I).attr.xdensity);
                (*r).ury = (*p).bbox.ury / ((*I).attr.height as f64 * (*I).attr.ydensity)
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
            scale_to_fit_F(M, p, I); /* I->attr.bbox from the image bounding box */
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
        _ => {}
    }
    0i32
}
/* Migrated from psimage.c */
#[no_mangle]
pub unsafe extern "C" fn set_distiller_template(mut s: *mut i8) {
    free(_opts.cmdtmpl as *mut libc::c_void);
    if s.is_null() || *s as i32 == '\u{0}' as i32 {
        _opts.cmdtmpl = 0 as *mut i8
    } else {
        _opts.cmdtmpl = new((strlen(s).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
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
pub unsafe extern "C" fn get_distiller_template() -> *mut i8 {
    _opts.cmdtmpl
}
unsafe extern "C" fn check_for_ps(mut handle: rust_input_handle_t) -> i32 {
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    tt_mfgets(work_buffer.as_mut_ptr(), 1024i32, handle);
    if !strstartswith(
        work_buffer.as_mut_ptr(),
        b"%!\x00" as *const u8 as *const i8,
    )
    .is_null()
    {
        return 1i32;
    }
    0i32
}
