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
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::warn;

use super::dpx_pdfdoc::pdf_doc_add_page_content;
use super::dpx_pdfdoc::pdf_doc_get_page;
use super::dpx_pdfdraw::{
    pdf_dev_closepath, pdf_dev_curveto, pdf_dev_flushpath, pdf_dev_lineto, pdf_dev_rectadd,
    pdf_dev_vcurveto, pdf_dev_ycurveto,
};
use super::dpx_pdfdraw::{
    pdf_dev_currentmatrix, pdf_dev_moveto, pdf_dev_transform, pdf_invertmatrix,
};
use super::dpx_pdfparse::skip_white;
use super::dpx_pdfximage::{pdf_ximage_init_form_info, pdf_ximage_set_form};
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_array_length, pdf_boolean_value, pdf_close, pdf_concat_stream,
    pdf_deref_obj, pdf_file, pdf_file_get_catalog, pdf_file_get_trailer, pdf_file_get_version,
    pdf_get_array, pdf_get_version, pdf_import_object, pdf_lookup_dict, pdf_new_array,
    pdf_new_dict, pdf_new_name, pdf_new_number, pdf_new_stream, pdf_number_value, pdf_obj,
    pdf_obj_typeof, pdf_open, pdf_release_obj, pdf_stream_dataptr, pdf_stream_dict,
    pdf_stream_length, PdfObjType,
};
use crate::dpx_pdfparse::{parse_ident, parse_pdf_array};
use crate::streq_ptr;
use libc::free;
extern "C" {
    #[no_mangle]
    fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
pub type rust_input_handle_t = *mut libc::c_void;

use super::dpx_pdfdev::{pdf_coord, pdf_tmatrix};

use crate::dpx_pdfximage::{load_options, pdf_ximage, xform_info};
pub const OP_CURVETO2: C2RustUnnamed_0 = 15;
pub const OP_CURVETO1: C2RustUnnamed_0 = 14;
pub const OP_GRESTORE: C2RustUnnamed_0 = 13;
pub const OP_GSAVE: C2RustUnnamed_0 = 12;
pub const OP_NOOP: C2RustUnnamed_0 = 11;
pub const OP_MOVETO: C2RustUnnamed_0 = 10;
pub const OP_LINETO: C2RustUnnamed_0 = 9;
pub const OP_CLOSEPATH: C2RustUnnamed_0 = 8;
pub const OP_CURVETO: C2RustUnnamed_0 = 7;
pub const OP_RECTANGLE: C2RustUnnamed_0 = 6;
pub const OP_SETCOLORSPACE: C2RustUnnamed_0 = 5;
pub const OP_CONCATMATRIX: C2RustUnnamed_0 = 4;
pub const OP_CLIP: C2RustUnnamed_0 = 3;
pub const OP_CLOSEandCLIP: C2RustUnnamed_0 = 2;
pub const OP_SETCOLOR: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator {
    pub token: *const i8,
    pub opcode: i32,
}
pub type C2RustUnnamed_0 = u32;
pub const OP_UNKNOWN: C2RustUnnamed_0 = 16;
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
/*
 * Concatinating content streams are only supported for streams that only uses
 * single FlateDecode filter, i.e.,
 *
 *   /Filter /FlateDecode or /Filter [/FlateDecode]
 *
 * TrimBox, BleedBox, ArtBox, Rotate ...
 */
/*
 * From PDFReference15_v6.pdf (p.119 and p.834)
 *
 * MediaBox rectangle (Required; inheritable)
 *
 * The media box defines the boundaries of the physical medium on which the
 * page is to be printed. It may include any extended area surrounding the
 * finished page for bleed, printing marks, or other such purposes. It may
 * also include areas close to the edges of the medium that cannot be marked
 * because of physical limitations of the output device. Content falling
 * outside this boundary can safely be discarded without affecting the
 * meaning of the PDF file.
 *
 * CropBox rectangle (Optional; inheritable)
 *
 * The crop box defines the region to which the contents of the page are to be
 * clipped (cropped) when displayed or printed. Unlike the other boxes, the
 * crop box has no defined meaning in terms of physical page geometry or
 * intended use; it merely imposes clipping on the page contents. However,
 * in the absence of additional information (such as imposition instructions
 * specified in a JDF or PJTF job ticket), the crop box will determine how
 * the page's contents are to be positioned on the output medium. The default
 * value is the page's media box.
 *
 * BleedBox rectangle (Optional; PDF 1.3)
 *
 * The bleed box (PDF 1.3) defines the region to which the contents of the
 * page should be clipped when output in a production environment. This may
 * include any extra bleed area needed to accommodate the physical
 * limitations of cutting, folding, and trimming equipment. The actual printed
 * page may include printing marks that fall outside the bleed box.
 * The default value is the page's crop box.
 *
 * TrimBox rectangle (Optional; PDF 1.3)
 *
 * The trim box (PDF 1.3) defines the intended dimensions of the finished page
 * after trimming. It may be smaller than the media box, to allow for
 * production-related content such as printing instructions, cut marks, or
 * color bars. The default value is the pageâ€™s crop box.
 *
 * ArtBox rectangle (Optional; PDF 1.3)
 *
 * The art box (PDF 1.3) defines the extent of the page's meaningful content
 * (including potential white space) as intended by the page's creator.
 * The default value is the page's crop box.
 *
 * Rotate integer (Optional; inheritable)
 *
 * The number of degrees by which the page should be rotated clockwise when
 * displayed or printed. The value must be a multiple of 90. Default value: 0.
 */
unsafe extern "C" fn rect_equal(mut rect1: *mut pdf_obj, mut rect2: *mut pdf_obj) -> i32 {
    let mut i: i32 = 0;
    if rect1.is_null() || rect2.is_null() {
        return 0i32;
    }
    i = 0i32;
    while i < 4i32 {
        if pdf_number_value(pdf_get_array(rect1, i)) != pdf_number_value(pdf_get_array(rect2, i)) {
            return 0i32;
        }
        i += 1
    }
    1i32
}
unsafe extern "C" fn pdf_get_page_obj(
    mut pf: *mut pdf_file,
    mut page_no: i32,
    mut ret_bbox: *mut *mut pdf_obj,
    mut ret_resources: *mut *mut pdf_obj,
) -> *mut pdf_obj {
    let mut page_tree: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut bbox: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut resources: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut rotate: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut page_idx: i32 = 0;
    /*
     * Get Page Tree.
     */
    page_tree = 0 as *mut pdf_obj;
    let mut trailer: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut catalog: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut markinfo: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    trailer = pdf_file_get_trailer(pf);
    if !pdf_lookup_dict(trailer, b"Encrypt\x00" as *const u8 as *const i8).is_null() {
        warn!("This PDF document is encrypted.");
        pdf_release_obj(trailer);
        return 0 as *mut pdf_obj;
    }
    catalog = pdf_deref_obj(pdf_lookup_dict(
        trailer,
        b"Root\x00" as *const u8 as *const i8,
    ));
    if !(!catalog.is_null() && pdf_obj_typeof(catalog) == PdfObjType::DICT) {
        warn!("Can\'t read document catalog.");
        pdf_release_obj(trailer);
        pdf_release_obj(catalog);
        return 0 as *mut pdf_obj;
    }
    pdf_release_obj(trailer);
    markinfo = pdf_deref_obj(pdf_lookup_dict(
        catalog,
        b"MarkInfo\x00" as *const u8 as *const i8,
    ));
    if !markinfo.is_null() {
        tmp = pdf_lookup_dict(markinfo, b"Marked\x00" as *const u8 as *const i8);
        if !tmp.is_null()
            && pdf_obj_typeof(tmp) == PdfObjType::BOOLEAN
            && pdf_boolean_value(tmp) as i32 != 0
        {
            warn!("PDF file is tagged... Ignoring tags.");
        }
        pdf_release_obj(markinfo);
    }
    page_tree = pdf_deref_obj(pdf_lookup_dict(
        catalog,
        b"Pages\x00" as *const u8 as *const i8,
    ));
    pdf_release_obj(catalog);
    if page_tree.is_null() {
        warn!("Page tree not found.");
        return 0 as *mut pdf_obj;
    }
    /*
     * Negative page numbers are counted from the back.
     */
    let mut count: i32 = pdf_number_value(pdf_lookup_dict(
        page_tree,
        b"Count\x00" as *const u8 as *const i8,
    )) as i32;
    page_idx = page_no + (if page_no >= 0i32 { -1i32 } else { count });
    if page_idx < 0i32 || page_idx >= count {
        warn!("Page {} does not exist.", page_no);
        pdf_release_obj(page_tree);
        return 0 as *mut pdf_obj;
    }
    page_no = page_idx + 1i32;
    /*
     * Seek correct page. Get Media/Crop Box.
     * Media box and resources can be inherited.
     */
    let mut kids_ref: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut kids: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut crop_box: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp_0: *mut pdf_obj = 0 as *mut pdf_obj;
    tmp_0 = pdf_lookup_dict(page_tree, b"Resources\x00" as *const u8 as *const i8);
    resources = if !tmp_0.is_null() {
        pdf_deref_obj(tmp_0)
    } else {
        pdf_new_dict()
    };
    loop {
        let mut kids_length: i32 = 0;
        let mut i: i32 = 0;
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"MediaBox\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            pdf_release_obj(bbox);
            bbox = tmp_0
        }
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"BleedBox\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            if rect_equal(tmp_0, bbox) == 0 {
                pdf_release_obj(bbox);
                bbox = tmp_0
            } else {
                pdf_release_obj(tmp_0);
            }
        }
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"TrimBox\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            if rect_equal(tmp_0, bbox) == 0 {
                pdf_release_obj(bbox);
                bbox = tmp_0
            } else {
                pdf_release_obj(tmp_0);
            }
        }
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"ArtBox\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            if rect_equal(tmp_0, bbox) == 0 {
                pdf_release_obj(bbox);
                bbox = tmp_0
            } else {
                pdf_release_obj(tmp_0);
            }
        }
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"CropBox\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            pdf_release_obj(crop_box);
            crop_box = tmp_0
        }
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"Rotate\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            pdf_release_obj(rotate);
            rotate = tmp_0
        }
        tmp_0 = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"Resources\x00" as *const u8 as *const i8,
        ));
        if !tmp_0.is_null() {
            pdf_release_obj(resources);
            resources = tmp_0
        }
        kids_ref = pdf_lookup_dict(page_tree, b"Kids\x00" as *const u8 as *const i8);
        if kids_ref.is_null() {
            break;
        }
        kids = pdf_deref_obj(kids_ref);
        kids_length = pdf_array_length(kids) as i32;
        i = 0i32;
        while i < kids_length {
            let mut count_0: i32 = 0;
            pdf_release_obj(page_tree);
            page_tree = pdf_deref_obj(pdf_get_array(kids, i));
            tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                page_tree,
                b"Count\x00" as *const u8 as *const i8,
            ));
            if !tmp_0.is_null() {
                /* Pages object */
                count_0 = pdf_number_value(tmp_0) as i32;
                pdf_release_obj(tmp_0);
            } else {
                /* Page object */
                count_0 = 1i32
            }
            if page_idx < count_0 {
                break;
            }
            page_idx -= count_0;
            i += 1
        }
        pdf_release_obj(kids);
        if i == kids_length {
            warn!("Page {} not found! Broken PDF file?", page_no);
            pdf_release_obj(bbox);
            pdf_release_obj(crop_box);
            pdf_release_obj(rotate);
            pdf_release_obj(resources);
            pdf_release_obj(page_tree);
            return 0 as *mut pdf_obj;
        }
    }
    if !crop_box.is_null() {
        pdf_release_obj(bbox);
        bbox = crop_box
    }
    if bbox.is_null() {
        warn!("No BoundingBox information available.");
        pdf_release_obj(page_tree);
        pdf_release_obj(resources);
        pdf_release_obj(rotate);
        return 0 as *mut pdf_obj;
    }
    if !rotate.is_null() {
        pdf_release_obj(rotate);
        rotate = 0 as *mut pdf_obj
    }
    if !ret_bbox.is_null() {
        *ret_bbox = bbox
    }
    if !ret_resources.is_null() {
        *ret_resources = resources
    }
    page_tree
}
unsafe extern "C" fn pdf_get_page_content(mut page: *mut pdf_obj) -> *mut pdf_obj {
    let mut contents: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut content_new: *mut pdf_obj = 0 as *mut pdf_obj;
    contents = pdf_deref_obj(pdf_lookup_dict(
        page,
        b"Contents\x00" as *const u8 as *const i8,
    ));
    if contents.is_null() {
        return 0 as *mut pdf_obj;
    }
    if pdf_obj_typeof(contents) == PdfObjType::NULL {
        /* empty page */
        pdf_release_obj(contents);
        /* TODO: better don't include anything if the page is empty */
        contents = pdf_new_stream(0i32)
    } else if !contents.is_null() && pdf_obj_typeof(contents) == PdfObjType::ARRAY {
        /*
         * Concatenate all content streams.
         */
        let mut content_seg: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut idx: i32 = 0i32;
        content_new = pdf_new_stream(1i32 << 0i32);
        loop {
            content_seg = pdf_deref_obj(pdf_get_array(contents, idx));
            if content_seg.is_null() {
                break;
            }
            if !(!content_seg.is_null() && pdf_obj_typeof(content_seg) == PdfObjType::NULL) {
                if !(!content_seg.is_null() && pdf_obj_typeof(content_seg) == PdfObjType::STREAM) {
                    warn!("Page content not a stream object. Broken PDF file?");
                    pdf_release_obj(content_seg);
                    pdf_release_obj(content_new);
                    pdf_release_obj(contents);
                    return 0 as *mut pdf_obj;
                } else {
                    if pdf_concat_stream(content_new, content_seg) < 0i32 {
                        warn!("Could not handle content stream with multiple segments.");
                        pdf_release_obj(content_seg);
                        pdf_release_obj(content_new);
                        pdf_release_obj(contents);
                        return 0 as *mut pdf_obj;
                    }
                }
            }
            pdf_release_obj(content_seg);
            idx += 1
        }
        pdf_release_obj(contents);
        contents = content_new
    } else {
        if !(!contents.is_null() && pdf_obj_typeof(contents) == PdfObjType::STREAM) {
            warn!("Page content not a stream object. Broken PDF file?");
            pdf_release_obj(contents);
            return 0 as *mut pdf_obj;
        }
        /* Flate the contents if necessary. */
        content_new = pdf_new_stream(1i32 << 0i32);
        if pdf_concat_stream(content_new, contents) < 0i32 {
            warn!("Could not handle a content stream.");
            pdf_release_obj(contents);
            pdf_release_obj(content_new);
            return 0 as *mut pdf_obj;
        }
        pdf_release_obj(contents);
        contents = content_new
    }
    contents
}
/* ximage here is the result. DONT USE IT FOR PASSING OPTIONS! */
#[no_mangle]
pub unsafe extern "C" fn pdf_include_page(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
    mut ident: *const i8,
    mut options: load_options,
) -> i32 {
    let mut current_block: u64;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    let mut info = xform_info::default();
    let mut contents: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut catalog: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut page: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut resources: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut markinfo: *mut pdf_obj = 0 as *mut pdf_obj;
    pf = pdf_open(ident, handle);
    if pf.is_null() {
        return -1i32;
    }
    if pdf_file_get_version(pf) > pdf_get_version() {
        warn!(
            "Trying to include PDF file which has newer version number than output PDF: 1.{}.",
            pdf_get_version()
        );
    }
    pdf_ximage_init_form_info(&mut info);
    if options.page_no == 0i32 {
        options.page_no = 1i32
    }
    page = pdf_doc_get_page(
        pf,
        options.page_no,
        options.bbox_type,
        &mut info.bbox,
        &mut info.matrix,
        &mut resources,
    );
    if !page.is_null() {
        catalog = pdf_file_get_catalog(pf);
        markinfo = pdf_deref_obj(pdf_lookup_dict(
            catalog,
            b"MarkInfo\x00" as *const u8 as *const i8,
        ));
        if !markinfo.is_null() {
            let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_lookup_dict(
                markinfo,
                b"Marked\x00" as *const u8 as *const i8,
            ));
            pdf_release_obj(markinfo);
            if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PdfObjType::BOOLEAN) {
                pdf_release_obj(tmp);
                current_block = 3699483483911207084;
            } else {
                if pdf_boolean_value(tmp) != 0 {
                    warn!("PDF file is tagged... Ignoring tags.");
                }
                pdf_release_obj(tmp);
                current_block = 1109700713171191020;
            }
        } else {
            current_block = 1109700713171191020;
        }
        match current_block {
            1109700713171191020 => {
                contents = pdf_deref_obj(pdf_lookup_dict(
                    page,
                    b"Contents\x00" as *const u8 as *const i8,
                ));
                pdf_release_obj(page);
                page = 0 as *mut pdf_obj;
                /*
                 * Handle page content stream.
                 */
                let mut content_new: *mut pdf_obj = 0 as *mut pdf_obj;
                if contents.is_null() {
                    /*
                     * Empty page
                     */
                    content_new = pdf_new_stream(0i32);
                    current_block = 2480299350034459858;
                /* TODO: better don't include anything if the page is empty */
                } else if !contents.is_null() && pdf_obj_typeof(contents) == PdfObjType::STREAM {
                    /*
                     * We must import the stream because its dictionary
                     * may contain indirect references.
                     */
                    content_new = pdf_import_object(contents);
                    current_block = 2480299350034459858;
                } else if !contents.is_null() && pdf_obj_typeof(contents) == PdfObjType::ARRAY {
                    /*
                     * Concatenate all content streams.
                     */
                    let mut idx: i32 = 0;
                    let mut len: i32 = pdf_array_length(contents) as i32;
                    content_new = pdf_new_stream(1i32 << 0i32);
                    idx = 0i32;
                    loop {
                        if !(idx < len) {
                            current_block = 2480299350034459858;
                            break;
                        }
                        let mut content_seg: *mut pdf_obj =
                            pdf_deref_obj(pdf_get_array(contents, idx));
                        if !(!content_seg.is_null()
                            && pdf_obj_typeof(content_seg) == PdfObjType::STREAM)
                            || pdf_concat_stream(content_new, content_seg) < 0i32
                        {
                            pdf_release_obj(content_seg);
                            pdf_release_obj(content_new);
                            current_block = 3699483483911207084;
                            break;
                        } else {
                            pdf_release_obj(content_seg);
                            idx += 1
                        }
                    }
                } else {
                    current_block = 3699483483911207084;
                }
                match current_block {
                    3699483483911207084 => {}
                    _ => {
                        pdf_release_obj(contents);
                        contents = content_new;
                        /*
                         * Add entries to contents stream dictionary.
                         */
                        let mut contents_dict: *mut pdf_obj = 0 as *mut pdf_obj;
                        let mut bbox: *mut pdf_obj = 0 as *mut pdf_obj;
                        let mut matrix: *mut pdf_obj = 0 as *mut pdf_obj;
                        contents_dict = pdf_stream_dict(contents);
                        pdf_add_dict(
                            contents_dict,
                            pdf_new_name(b"Type\x00" as *const u8 as *const i8),
                            pdf_new_name(b"XObject\x00" as *const u8 as *const i8),
                        );
                        pdf_add_dict(
                            contents_dict,
                            pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
                            pdf_new_name(b"Form\x00" as *const u8 as *const i8),
                        );
                        pdf_add_dict(
                            contents_dict,
                            pdf_new_name(b"FormType\x00" as *const u8 as *const i8),
                            pdf_new_number(1.0f64),
                        );
                        bbox = pdf_new_array();
                        pdf_add_array(bbox, pdf_new_number(info.bbox.llx));
                        pdf_add_array(bbox, pdf_new_number(info.bbox.lly));
                        pdf_add_array(bbox, pdf_new_number(info.bbox.urx));
                        pdf_add_array(bbox, pdf_new_number(info.bbox.ury));
                        pdf_add_dict(
                            contents_dict,
                            pdf_new_name(b"BBox\x00" as *const u8 as *const i8),
                            bbox,
                        );
                        matrix = pdf_new_array();
                        pdf_add_array(matrix, pdf_new_number(info.matrix.a));
                        pdf_add_array(matrix, pdf_new_number(info.matrix.b));
                        pdf_add_array(matrix, pdf_new_number(info.matrix.c));
                        pdf_add_array(matrix, pdf_new_number(info.matrix.d));
                        pdf_add_array(matrix, pdf_new_number(info.matrix.e));
                        pdf_add_array(matrix, pdf_new_number(info.matrix.f));
                        pdf_add_dict(
                            contents_dict,
                            pdf_new_name(b"Matrix\x00" as *const u8 as *const i8),
                            matrix,
                        );
                        pdf_add_dict(
                            contents_dict,
                            pdf_new_name(b"Resources\x00" as *const u8 as *const i8),
                            pdf_import_object(resources),
                        );
                        pdf_release_obj(resources);
                        pdf_close(pf);
                        pdf_ximage_set_form(ximage, &mut info, contents);
                        return 0i32;
                    }
                }
            }
            _ => {}
        }
        warn!("Cannot parse document. Broken PDF file?");
    }
    pdf_release_obj(resources);
    pdf_release_obj(markinfo);
    pdf_release_obj(page);
    pdf_release_obj(contents);
    pdf_close(pf);
    -1i32
}
static mut pdf_operators: [operator; 39] = [
    {
        let mut init = operator {
            token: b"SCN\x00" as *const u8 as *const i8,
            opcode: OP_SETCOLOR as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"b*\x00" as *const u8 as *const i8,
            opcode: OP_CLOSEandCLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"B*\x00" as *const u8 as *const i8,
            opcode: OP_CLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"cm\x00" as *const u8 as *const i8,
            opcode: OP_CONCATMATRIX as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"CS\x00" as *const u8 as *const i8,
            opcode: OP_SETCOLORSPACE as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"f*\x00" as *const u8 as *const i8,
            opcode: 0i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"gs\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"re\x00" as *const u8 as *const i8,
            opcode: OP_RECTANGLE as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"rg\x00" as *const u8 as *const i8,
            opcode: -3i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"RG\x00" as *const u8 as *const i8,
            opcode: -3i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"sc\x00" as *const u8 as *const i8,
            opcode: OP_SETCOLOR as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"SC\x00" as *const u8 as *const i8,
            opcode: OP_SETCOLOR as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"W*\x00" as *const u8 as *const i8,
            opcode: OP_CLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"b\x00" as *const u8 as *const i8,
            opcode: OP_CLOSEandCLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"B\x00" as *const u8 as *const i8,
            opcode: OP_CLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"c\x00" as *const u8 as *const i8,
            opcode: OP_CURVETO as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"d\x00" as *const u8 as *const i8,
            opcode: -2i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"f\x00" as *const u8 as *const i8,
            opcode: 0i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"F\x00" as *const u8 as *const i8,
            opcode: 0i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"g\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"G\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"h\x00" as *const u8 as *const i8,
            opcode: OP_CLOSEPATH as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"i\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"j\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"J\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"k\x00" as *const u8 as *const i8,
            opcode: -4i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"K\x00" as *const u8 as *const i8,
            opcode: -4i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"l\x00" as *const u8 as *const i8,
            opcode: OP_LINETO as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"m\x00" as *const u8 as *const i8,
            opcode: OP_MOVETO as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"M\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"n\x00" as *const u8 as *const i8,
            opcode: OP_NOOP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"q\x00" as *const u8 as *const i8,
            opcode: OP_GSAVE as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"Q\x00" as *const u8 as *const i8,
            opcode: OP_GRESTORE as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"s\x00" as *const u8 as *const i8,
            opcode: OP_CLOSEandCLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"S\x00" as *const u8 as *const i8,
            opcode: OP_CLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"v\x00" as *const u8 as *const i8,
            opcode: OP_CURVETO1 as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"w\x00" as *const u8 as *const i8,
            opcode: -1i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"W\x00" as *const u8 as *const i8,
            opcode: OP_CLIP as i32,
        };
        init
    },
    {
        let mut init = operator {
            token: b"y\x00" as *const u8 as *const i8,
            opcode: OP_CURVETO2 as i32,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn pdf_copy_clip(
    mut image_file: *mut libc::FILE,
    mut pageNo: i32,
    mut x_user: f64,
    mut y_user: f64,
) -> i32 {
    let mut page_tree: *mut pdf_obj = 0 as *mut pdf_obj; /* silence uninitialized warning */
    let mut contents: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut depth: i32 = 0i32;
    let mut top: i32 = -1i32;
    let mut clip_path: *const i8 = 0 as *const i8;
    let mut end_path: *const i8 = 0 as *const i8;
    let mut save_path: *mut i8 = 0 as *mut i8;
    let mut temp: *mut i8 = 0 as *mut i8;
    let mut M = pdf_tmatrix::new();
    let mut stack: [f64; 6] = [0.; 6];
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    pf = pdf_open(0 as *const i8, image_file as rust_input_handle_t);
    if pf.is_null() {
        return -1i32;
    }
    pdf_dev_currentmatrix(&mut M);
    pdf_invertmatrix(&mut M);
    M.e += x_user;
    M.f += y_user;
    page_tree = pdf_get_page_obj(pf, pageNo, 0 as *mut *mut pdf_obj, 0 as *mut *mut pdf_obj);
    if page_tree.is_null() {
        pdf_close(pf);
        return -1i32;
    }
    contents = pdf_get_page_content(page_tree);
    pdf_release_obj(page_tree);
    if contents.is_null() {
        pdf_close(pf);
        return -1i32;
    }
    pdf_doc_add_page_content(b" \x00" as *const u8 as *const i8, 1_u32);
    save_path = xmalloc((pdf_stream_length(contents) + 1i32) as size_t) as *mut i8;
    strncpy(
        save_path,
        pdf_stream_dataptr(contents) as *const i8,
        pdf_stream_length(contents) as u64,
    );
    clip_path = save_path;
    end_path = clip_path.offset(pdf_stream_length(contents) as isize);
    depth = 0i32;
    while clip_path < end_path {
        let mut color_dimen: i32 = 0i32;
        let mut token: *mut i8 = 0 as *mut i8;
        skip_white(&mut clip_path, end_path);
        if clip_path == end_path {
            break;
        }
        if depth > 1i32 {
            if *clip_path as i32 == 'q' as i32 {
                depth += 1
            }
            if *clip_path as i32 == 'Q' as i32 {
                depth -= 1
            }
            parse_ident(&mut clip_path, end_path);
        } else if *clip_path as i32 == '-' as i32
            || *clip_path as i32 == '+' as i32
            || *clip_path as i32 == '.' as i32
            || libc::isdigit(*clip_path as _) != 0
        {
            top += 1;
            stack[top as usize] = strtod(clip_path, &mut temp);
            clip_path = temp
        } else if *clip_path as i32 == '[' as i32 {
            /* Ignore, but put a dummy value on the stack (in case of d operator) */
            parse_pdf_array(&mut clip_path, end_path, pf);
            top += 1;
            stack[top as usize] = 0i32 as f64
        } else if *clip_path as i32 == '/' as i32 {
            if strncmp(
                b"/DeviceGray\x00" as *const u8 as *const i8,
                clip_path,
                11i32 as u64,
            ) == 0i32
                || strncmp(
                    b"/Indexed\x00" as *const u8 as *const i8,
                    clip_path,
                    8i32 as u64,
                ) == 0i32
                || strncmp(
                    b"/CalGray\x00" as *const u8 as *const i8,
                    clip_path,
                    8i32 as u64,
                ) == 0i32
            {
                color_dimen = 1i32
            } else if strncmp(
                b"/DeviceRGB\x00" as *const u8 as *const i8,
                clip_path,
                10i32 as u64,
            ) == 0i32
                || strncmp(
                    b"/CalRGB\x00" as *const u8 as *const i8,
                    clip_path,
                    7i32 as u64,
                ) == 0i32
                || strncmp(
                    b"/Lab\x00" as *const u8 as *const i8,
                    clip_path,
                    4i32 as u64,
                ) == 0i32
            {
                color_dimen = 3i32
            } else if strncmp(
                b"/DeviceCMYK\x00" as *const u8 as *const i8,
                clip_path,
                11i32 as u64,
            ) == 0i32
            {
                color_dimen = 4i32
            } else {
                clip_path = clip_path.offset(1);
                parse_ident(&mut clip_path, end_path);
                skip_white(&mut clip_path, end_path);
                token = parse_ident(&mut clip_path, end_path);
                if !streq_ptr(token, b"gs\x00" as *const u8 as *const i8) {
                    return -1i32;
                }
            }
        } else {
            let mut j: u32 = 0;
            let mut T = pdf_tmatrix::new();
            let mut p0 = pdf_coord::zero();
            let mut p1 = pdf_coord::zero();
            let mut p2 = pdf_coord::zero();
            let mut p3 = pdf_coord::zero();
            token = parse_ident(&mut clip_path, end_path);
            j = 0_u32;
            while (j as u64)
                < (::std::mem::size_of::<[operator; 39]>() as u64)
                    .wrapping_div(::std::mem::size_of::<operator>() as u64)
            {
                if streq_ptr(token, pdf_operators[j as usize].token) {
                    break;
                }
                j = j.wrapping_add(1)
            }
            if j as u64
                == (::std::mem::size_of::<[operator; 39]>() as u64)
                    .wrapping_div(::std::mem::size_of::<operator>() as u64)
            {
                return -1i32;
            }
            let mut current_block_157: u64;
            match pdf_operators[j as usize].opcode {
                0 | -1 | -2 | -3 | -4 => {
                    /* Just pop the stack and do nothing. */
                    top += pdf_operators[j as usize].opcode;
                    if top < -1i32 {
                        return -1i32;
                    }
                    current_block_157 = 6328367678128271922;
                }
                1 => {
                    top -= color_dimen;
                    if top < -1i32 {
                        return -1i32;
                    }
                    current_block_157 = 6328367678128271922;
                }
                2 => {
                    pdf_dev_closepath();
                    current_block_157 = 17294711039657812359;
                }
                3 => {
                    current_block_157 = 17294711039657812359;
                }
                4 => {
                    if top < 5i32 {
                        return -1i32;
                    }
                    let fresh0 = top;
                    top = top - 1;
                    T.f = stack[fresh0 as usize];
                    let fresh1 = top;
                    top = top - 1;
                    T.e = stack[fresh1 as usize];
                    let fresh2 = top;
                    top = top - 1;
                    T.d = stack[fresh2 as usize];
                    let fresh3 = top;
                    top = top - 1;
                    T.c = stack[fresh3 as usize];
                    let fresh4 = top;
                    top = top - 1;
                    T.b = stack[fresh4 as usize];
                    let fresh5 = top;
                    top = top - 1;
                    T.a = stack[fresh5 as usize];
                    let mut _tmp_a: f64 = 0.;
                    let mut _tmp_b: f64 = 0.;
                    let mut _tmp_c: f64 = 0.;
                    let mut _tmp_d: f64 = 0.;
                    _tmp_a = M.a;
                    _tmp_b = M.b;
                    _tmp_c = M.c;
                    _tmp_d = M.d;
                    M.a = T.a * _tmp_a + T.b * _tmp_c;
                    M.b = T.a * _tmp_b + T.b * _tmp_d;
                    M.c = T.c * _tmp_a + T.d * _tmp_c;
                    M.d = T.c * _tmp_b + T.d * _tmp_d;
                    M.e += T.e * _tmp_a + T.f * _tmp_c;
                    M.f += T.e * _tmp_b + T.f * _tmp_d;
                    current_block_157 = 6328367678128271922;
                }
                5 => {
                    current_block_157 = 6328367678128271922;
                }
                6 => {
                    if top < 3i32 {
                        return -1i32;
                    }
                    let fresh6 = top;
                    top = top - 1;
                    p1.y = stack[fresh6 as usize];
                    let fresh7 = top;
                    top = top - 1;
                    p1.x = stack[fresh7 as usize];
                    let fresh8 = top;
                    top = top - 1;
                    p0.y = stack[fresh8 as usize];
                    let fresh9 = top;
                    top = top - 1;
                    p0.x = stack[fresh9 as usize];
                    if M.b == 0i32 as f64 && M.c == 0i32 as f64 {
                        let mut M0 = pdf_tmatrix {
                            a: M.a,
                            b: M.b,
                            c: M.c,
                            d: M.d,
                            e: 0.,
                            f: 0.,
                        };
                        pdf_dev_transform(&mut p0, Some(&M));
                        pdf_dev_transform(&mut p1, Some(&M0));
                        pdf_dev_rectadd(p0.x, p0.y, p1.x, p1.y);
                    } else {
                        p2.x = p0.x + p1.x;
                        p2.y = p0.y + p1.y;
                        p3.x = p0.x;
                        p3.y = p0.y + p1.y;
                        p1.x += p0.x;
                        p1.y = p0.y;
                        pdf_dev_transform(&mut p0, Some(&M));
                        pdf_dev_transform(&mut p1, Some(&M));
                        pdf_dev_transform(&mut p2, Some(&M));
                        pdf_dev_transform(&mut p3, Some(&M));
                        pdf_dev_moveto(p0.x, p0.y);
                        pdf_dev_lineto(p1.x, p1.y);
                        pdf_dev_lineto(p2.x, p2.y);
                        pdf_dev_lineto(p3.x, p3.y);
                        pdf_dev_closepath();
                    }
                    current_block_157 = 6328367678128271922;
                }
                7 => {
                    if top < 5i32 {
                        return -1i32;
                    }
                    let fresh10 = top;
                    top = top - 1;
                    p0.y = stack[fresh10 as usize];
                    let fresh11 = top;
                    top = top - 1;
                    p0.x = stack[fresh11 as usize];
                    pdf_dev_transform(&mut p0, Some(&M));
                    let fresh12 = top;
                    top = top - 1;
                    p1.y = stack[fresh12 as usize];
                    let fresh13 = top;
                    top = top - 1;
                    p1.x = stack[fresh13 as usize];
                    pdf_dev_transform(&mut p1, Some(&M));
                    let fresh14 = top;
                    top = top - 1;
                    p2.y = stack[fresh14 as usize];
                    let fresh15 = top;
                    top = top - 1;
                    p2.x = stack[fresh15 as usize];
                    pdf_dev_transform(&mut p2, Some(&M));
                    pdf_dev_curveto(p2.x, p2.y, p1.x, p1.y, p0.x, p0.y);
                    current_block_157 = 6328367678128271922;
                }
                8 => {
                    pdf_dev_closepath();
                    current_block_157 = 6328367678128271922;
                }
                9 => {
                    if top < 1i32 {
                        return -1i32;
                    }
                    let fresh16 = top;
                    top = top - 1;
                    p0.y = stack[fresh16 as usize];
                    let fresh17 = top;
                    top = top - 1;
                    p0.x = stack[fresh17 as usize];
                    pdf_dev_transform(&mut p0, Some(&M));
                    pdf_dev_lineto(p0.x, p0.y);
                    current_block_157 = 6328367678128271922;
                }
                10 => {
                    if top < 1i32 {
                        return -1i32;
                    }
                    let fresh18 = top;
                    top = top - 1;
                    p0.y = stack[fresh18 as usize];
                    let fresh19 = top;
                    top = top - 1;
                    p0.x = stack[fresh19 as usize];
                    pdf_dev_transform(&mut p0, Some(&M));
                    pdf_dev_moveto(p0.x, p0.y);
                    current_block_157 = 6328367678128271922;
                }
                11 => {
                    pdf_doc_add_page_content(b" n\x00" as *const u8 as *const i8, 2_u32);
                    current_block_157 = 6328367678128271922;
                }
                12 => {
                    depth += 1;
                    current_block_157 = 6328367678128271922;
                }
                13 => {
                    depth -= 1;
                    current_block_157 = 6328367678128271922;
                }
                14 => {
                    if top < 3i32 {
                        return -1i32;
                    }
                    let fresh20 = top;
                    top = top - 1;
                    p0.y = stack[fresh20 as usize];
                    let fresh21 = top;
                    top = top - 1;
                    p0.x = stack[fresh21 as usize];
                    pdf_dev_transform(&mut p0, Some(&M));
                    let fresh22 = top;
                    top = top - 1;
                    p1.y = stack[fresh22 as usize];
                    let fresh23 = top;
                    top = top - 1;
                    p1.x = stack[fresh23 as usize];
                    pdf_dev_transform(&mut p1, Some(&M));
                    pdf_dev_vcurveto(p1.x, p1.y, p0.x, p0.y);
                    current_block_157 = 6328367678128271922;
                }
                15 => {
                    if top < 3i32 {
                        return -1i32;
                    }
                    let fresh24 = top;
                    top = top - 1;
                    p0.y = stack[fresh24 as usize];
                    let fresh25 = top;
                    top = top - 1;
                    p0.x = stack[fresh25 as usize];
                    pdf_dev_transform(&mut p0, Some(&M));
                    let fresh26 = top;
                    top = top - 1;
                    p1.y = stack[fresh26 as usize];
                    let fresh27 = top;
                    top = top - 1;
                    p1.x = stack[fresh27 as usize];
                    pdf_dev_transform(&mut p1, Some(&M));
                    pdf_dev_ycurveto(p1.x, p1.y, p0.x, p0.y);
                    current_block_157 = 6328367678128271922;
                }
                _ => return -1i32,
            }
            match current_block_157 {
                17294711039657812359 => {
                    pdf_dev_flushpath('W' as i32 as i8, 0i32);
                }
                _ => {}
            }
        }
        clip_path = clip_path.offset(1)
    }
    free(save_path as *mut libc::c_void);
    pdf_release_obj(contents);
    pdf_close(pf);
    0i32
}
