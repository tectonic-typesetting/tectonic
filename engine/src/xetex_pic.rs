#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::core_memory::xstrdup;
use crate::xetex_errors::error;
use crate::xetex_ext::{D2Fix, Fix2D};
use crate::xetex_ini::memory_word;
use crate::xetex_ini::{
    cur_area, cur_ext, cur_list, cur_name, cur_val, file_line_error_style_p, help_line, help_ptr,
    mem, name_of_file,
};
use crate::xetex_output::{
    print, print_cstr, print_file_line, print_file_name, print_nl_cstr, print_scaled,
};
use crate::xetex_xetex0::{
    new_whatsit, pack_file_name, scan_decimal, scan_dimen, scan_file_name, scan_int, scan_keyword,
};
use crate::TTInputFormat;
use crate::{ttstub_input_close, ttstub_input_open};
use dpx::dpx_bmpimage::{bmp_get_bbox, check_for_bmp};
use dpx::dpx_dpxutil::{max4, min4};
use dpx::dpx_jpegimage::{check_for_jpeg, jpeg_get_bbox};
use dpx::dpx_pdfdoc::{pdf_doc_get_page, pdf_doc_get_page_count};
use dpx::dpx_pdfdraw::pdf_dev_transform;
use dpx::dpx_pdfobj::{pdf_close, pdf_file, pdf_obj, pdf_open, pdf_release_obj};
use dpx::dpx_pngimage::{check_for_png, png_get_bbox};
use libc::{free, memcpy, strlen};

pub type rust_input_handle_t = *mut libc::c_void;
pub type scaled_t = i32;
pub type Fixed = scaled_t;
pub type str_number = i32;
pub type small_number = i16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_t {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct real_point {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct real_rect {
    pub x: f32,
    pub y: f32,
    pub wd: f32,
    pub ht: f32,
}
use dpx::dpx_pdfdev::{pdf_coord, pdf_rect};

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

use dpx::dpx_pdfdev::pdf_tmatrix;

#[no_mangle]
pub unsafe extern "C" fn count_pdf_file_pages() -> i32 {
    let mut pages: i32 = 0;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    handle = ttstub_input_open(name_of_file, TTInputFormat::PICT, 0i32);
    if handle.is_null() {
        return 0i32;
    }
    pf = pdf_open(name_of_file, handle);
    if pf.is_null() {
        /* TODO: issue warning */
        ttstub_input_close(handle);
        return 0i32;
    }
    pages = pdf_doc_get_page_count(pf);
    pdf_close(pf);
    ttstub_input_close(handle);
    pages
}
unsafe extern "C" fn pdf_get_rect(
    mut filename: *mut i8,
    mut handle: rust_input_handle_t,
    mut page_num: i32,
    mut pdf_box: i32,
    mut box_0: *mut real_rect,
) -> i32 {
    let mut pages: i32 = 0;
    let mut dpx_options: i32 = 0;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    let mut page: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut bbox = pdf_rect::new();
    let mut matrix = pdf_tmatrix::new();
    pf = pdf_open(filename, handle);
    if pf.is_null() {
        /* TODO: issue warning */
        return -1i32;
    }
    pages = pdf_doc_get_page_count(pf);
    if page_num > pages {
        page_num = pages
    }
    if page_num < 0i32 {
        page_num = pages + 1i32 + page_num
    }
    if page_num < 1i32 {
        page_num = 1i32
    }
    /* OMG, magic numbers specifying page bound types do not agree between
     * xdvipdfmx code (dpx-pdfdoc.c:pdf_doc_get_page) and XeTeX/Apple's
     * pdfbox_* definitions (xetex-ext.h). */
    match pdf_box {
        2 => dpx_options = 2i32,
        3 => dpx_options = 5i32,
        4 => dpx_options = 4i32,
        5 => dpx_options = 3i32,
        1 | _ => dpx_options = 1i32,
    }
    page = pdf_doc_get_page(
        pf,
        page_num,
        dpx_options,
        &mut bbox,
        &mut matrix,
        0 as *mut *mut pdf_obj,
    );
    pdf_close(pf);
    if page.is_null() {
        /* TODO: issue warning */
        return -1i32;
    }
    pdf_release_obj(page);
    /* Image's attribute "bbox" here is affected by /Rotate entry of included
     * PDF page.
     */
    let mut p1 = pdf_coord::new(bbox.llx, bbox.lly);
    pdf_dev_transform(&mut p1, Some(&matrix));
    let mut p2 = pdf_coord::new(bbox.urx, bbox.lly);
    pdf_dev_transform(&mut p2, Some(&matrix));
    let mut p3 = pdf_coord::new(bbox.urx, bbox.ury);
    pdf_dev_transform(&mut p3, Some(&matrix));
    let mut p4 = pdf_coord::new(bbox.llx, bbox.ury);
    pdf_dev_transform(&mut p4, Some(&matrix));
    bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
    bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
    bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
    bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);
    (*box_0).x = (72.27f64 / 72i32 as f64 * bbox.llx) as f32;
    (*box_0).y = (72.27f64 / 72i32 as f64 * bbox.lly) as f32;
    (*box_0).wd = (72.27f64 / 72i32 as f64 * (bbox.urx - bbox.llx)) as f32;
    (*box_0).ht = (72.27f64 / 72i32 as f64 * (bbox.ury - bbox.lly)) as f32;
    0
}
unsafe extern "C" fn get_image_size_in_inches(
    mut handle: rust_input_handle_t,
    mut width: *mut f32,
    mut height: *mut f32,
) -> i32 {
    let mut err: i32 = 1i32;
    let mut width_pix: u32 = 0;
    let mut height_pix: u32 = 0;
    let mut xdensity: f64 = 0.;
    let mut ydensity: f64 = 0.;
    if check_for_jpeg(handle) != 0 {
        err = jpeg_get_bbox(
            handle,
            &mut width_pix,
            &mut height_pix,
            &mut xdensity,
            &mut ydensity,
        )
    } else if check_for_bmp(handle) != 0 {
        err = bmp_get_bbox(
            handle,
            &mut width_pix,
            &mut height_pix,
            &mut xdensity,
            &mut ydensity,
        )
    } else if check_for_png(handle) != 0 {
        err = png_get_bbox(
            handle,
            &mut width_pix,
            &mut height_pix,
            &mut xdensity,
            &mut ydensity,
        )
    }
    if err != 0 {
        *width = -1i32 as f32;
        *height = -1i32 as f32;
        return err;
    }
    /* xdvipdfmx defines density = 72 / dpi, so ... */
    *width = (width_pix as f64 * xdensity / 72i32 as f64) as f32;
    *height = (height_pix as f64 * ydensity / 72i32 as f64) as f32;
    0
}
/*
  pdfBoxType indicates which pdf bounding box to use (0 for \XeTeXpicfile)
  page indicates which page is wanted (0-based)
  return 0 for success, or non-zero error code for failure
  return full path in *path
  return bounds (tex points) in *bounds
*/
unsafe extern "C" fn find_pic_file(
    mut path: *mut *mut i8,
    mut bounds: *mut real_rect,
    mut pdfBoxType: i32,
    mut page: i32,
) -> i32 {
    let mut err: i32 = -1i32;
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    handle = ttstub_input_open(name_of_file, TTInputFormat::PICT, 0i32);
    (*bounds).ht = 0.0f64 as f32;
    (*bounds).wd = (*bounds).ht;
    (*bounds).y = (*bounds).wd;
    (*bounds).x = (*bounds).y;
    if handle.is_null() {
        return 1i32;
    }
    if pdfBoxType != 0i32 {
        /* if cmd was \XeTeXpdffile, use xpdflib to read it */
        err = pdf_get_rect(name_of_file, handle, page, pdfBoxType, bounds)
    } else {
        err = get_image_size_in_inches(handle, &mut (*bounds).wd, &mut (*bounds).ht);
        (*bounds).wd = ((*bounds).wd as f64 * 72.27f64) as f32;
        (*bounds).ht = ((*bounds).ht as f64 * 72.27f64) as f32
    }
    if err == 0i32 {
        *path = xstrdup(name_of_file)
    }
    ttstub_input_close(handle);
    err
}
unsafe extern "C" fn transform_point(mut p: *mut real_point, mut t: *const transform_t) {
    let mut r: real_point = real_point { x: 0., y: 0. };
    r.x = ((*t).a * (*p).x as f64 + (*t).c * (*p).y as f64 + (*t).x) as f32;
    r.y = ((*t).b * (*p).x as f64 + (*t).d * (*p).y as f64 + (*t).y) as f32;
    *p = r;
}
unsafe extern "C" fn make_identity(mut t: *mut transform_t) {
    (*t).a = 1.0f64;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = 1.0f64;
    (*t).x = 0.0f64;
    (*t).y = 0.0f64;
}
unsafe extern "C" fn make_scale(mut t: *mut transform_t, mut xscale: f64, mut yscale: f64) {
    (*t).a = xscale;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = yscale;
    (*t).x = 0.0f64;
    (*t).y = 0.0f64;
}
unsafe extern "C" fn make_translation(mut t: *mut transform_t, mut dx: f64, mut dy: f64) {
    (*t).a = 1.0f64;
    (*t).b = 0.0f64;
    (*t).c = 0.0f64;
    (*t).d = 1.0f64;
    (*t).x = dx;
    (*t).y = dy;
}
unsafe extern "C" fn make_rotation(mut t: *mut transform_t, mut a: f64) {
    let (s, c) = a.sin_cos();
    (*t).a = c;
    (*t).b = s;
    (*t).c = -s;
    (*t).d = c;
    (*t).x = 0.;
    (*t).y = 0.;
}
unsafe extern "C" fn transform_concat(mut t1: *mut transform_t, mut t2: *const transform_t) {
    let mut r: transform_t = transform_t {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        x: 0.,
        y: 0.,
    };
    r.a = (*t1).a * (*t2).a + (*t1).b * (*t2).c + 0.0f64 * (*t2).x;
    r.b = (*t1).a * (*t2).b + (*t1).b * (*t2).d + 0.0f64 * (*t2).y;
    r.c = (*t1).c * (*t2).a + (*t1).d * (*t2).c + 0.0f64 * (*t2).x;
    r.d = (*t1).c * (*t2).b + (*t1).d * (*t2).d + 0.0f64 * (*t2).y;
    r.x = (*t1).x * (*t2).a + (*t1).y * (*t2).c + 1.0f64 * (*t2).x;
    r.y = (*t1).x * (*t2).b + (*t1).y * (*t2).d + 1.0f64 * (*t2).y;
    *t1 = r;
}
#[no_mangle]
pub unsafe extern "C" fn load_picture(mut is_pdf: bool) {
    let mut pic_path: *mut i8 = 0 as *mut i8;
    let mut bounds: real_rect = real_rect {
        x: 0.,
        y: 0.,
        wd: 0.,
        ht: 0.,
    };
    let mut t: transform_t = transform_t {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        x: 0.,
        y: 0.,
    };
    let mut t2: transform_t = transform_t {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        x: 0.,
        y: 0.,
    };
    let mut corners: [real_point; 4] = [real_point { x: 0., y: 0. }; 4];
    let mut x_size_req: f64 = 0.;
    let mut y_size_req: f64 = 0.;
    let mut check_keywords: bool = false;
    let mut xmin: f64 = 0.;
    let mut xmax: f64 = 0.;
    let mut ymin: f64 = 0.;
    let mut ymax: f64 = 0.;
    let mut i: small_number = 0;
    let mut page: i32 = 0;
    let mut pdf_box_type: i32 = 0;
    let mut result: i32 = 0;
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
    pdf_box_type = 0i32;
    page = 0i32;
    if is_pdf {
        if scan_keyword(b"page\x00" as *const u8 as *const i8) {
            scan_int();
            page = cur_val
        }
        pdf_box_type = 6i32;
        if scan_keyword(b"crop\x00" as *const u8 as *const i8) {
            pdf_box_type = 1i32
        } else if scan_keyword(b"media\x00" as *const u8 as *const i8) {
            pdf_box_type = 2i32
        } else if scan_keyword(b"bleed\x00" as *const u8 as *const i8) {
            pdf_box_type = 3i32
        } else if scan_keyword(b"trim\x00" as *const u8 as *const i8) {
            pdf_box_type = 4i32
        } else if scan_keyword(b"art\x00" as *const u8 as *const i8) {
            pdf_box_type = 5i32
        }
    }
    if pdf_box_type == 6i32 {
        result = find_pic_file(&mut pic_path, &mut bounds, 1i32, page)
    } else {
        result = find_pic_file(&mut pic_path, &mut bounds, pdf_box_type, page)
    }
    corners[0].x = bounds.x;
    corners[0].y = bounds.y;
    corners[1].x = corners[0].x;
    corners[1].y = bounds.y + bounds.ht;
    corners[2].x = bounds.x + bounds.wd;
    corners[2].y = corners[1].y;
    corners[3].x = corners[2].x;
    corners[3].y = corners[0].y;
    x_size_req = 0.0f64;
    y_size_req = 0.0f64;
    make_identity(&mut t);
    check_keywords = true;
    while check_keywords {
        if scan_keyword(b"scaled\x00" as *const u8 as *const i8) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(
                    &mut t2,
                    cur_val as f64 / 1000.0f64,
                    cur_val as f64 / 1000.0f64,
                );
                let mut for_end: i32 = 0;
                i = 0i32 as small_number;
                for_end = 3i32;
                if i as i32 <= for_end {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh0 = i;
                        i = i + 1;
                        if !((fresh0 as i32) < for_end) {
                            break;
                        }
                    }
                }
                transform_concat(&mut t, &mut t2);
            }
        } else if scan_keyword(b"xscaled\x00" as *const u8 as *const i8) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(&mut t2, cur_val as f64 / 1000.0f64, 1.0f64);
                let mut for_end_0: i32 = 0;
                i = 0i32 as small_number;
                for_end_0 = 3i32;
                if i as i32 <= for_end_0 {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh1 = i;
                        i = i + 1;
                        if !((fresh1 as i32) < for_end_0) {
                            break;
                        }
                    }
                }
                transform_concat(&mut t, &mut t2);
            }
        } else if scan_keyword(b"yscaled\x00" as *const u8 as *const i8) {
            scan_int();
            if x_size_req == 0.0f64 && y_size_req == 0.0f64 {
                make_scale(&mut t2, 1.0f64, cur_val as f64 / 1000.0f64);
                let mut for_end_1: i32 = 0;
                i = 0i32 as small_number;
                for_end_1 = 3i32;
                if i as i32 <= for_end_1 {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh2 = i;
                        i = i + 1;
                        if !((fresh2 as i32) < for_end_1) {
                            break;
                        }
                    }
                }
                transform_concat(&mut t, &mut t2);
            }
        } else if scan_keyword(b"width\x00" as *const u8 as *const i8) {
            scan_dimen(false, false, false);
            if cur_val <= 0i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                }
                print_cstr(b"Improper image \x00" as *const u8 as *const i8);
                print_cstr(b"size (\x00" as *const u8 as *const i8);
                print_scaled(cur_val);
                print_cstr(b"pt) will be ignored\x00" as *const u8 as *const i8);
                help_ptr = 2_u8;
                help_line[1] = b"I can\'t scale images to zero or negative sizes,\x00" as *const u8
                    as *const i8;
                help_line[0] = b"so I\'m ignoring this.\x00" as *const u8 as *const i8;
                error();
            } else {
                x_size_req = Fix2D(cur_val)
            }
        } else if scan_keyword(b"height\x00" as *const u8 as *const i8) {
            scan_dimen(false, false, false);
            if cur_val <= 0i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                }
                print_cstr(b"Improper image \x00" as *const u8 as *const i8);
                print_cstr(b"size (\x00" as *const u8 as *const i8);
                print_scaled(cur_val);
                print_cstr(b"pt) will be ignored\x00" as *const u8 as *const i8);
                help_ptr = 2_u8;
                help_line[1] = b"I can\'t scale images to zero or negative sizes,\x00" as *const u8
                    as *const i8;
                help_line[0] = b"so I\'m ignoring this.\x00" as *const u8 as *const i8;
                error();
            } else {
                y_size_req = Fix2D(cur_val)
            }
        } else if scan_keyword(b"rotated\x00" as *const u8 as *const i8) {
            scan_decimal();
            if x_size_req != 0.0f64 || y_size_req != 0.0f64 {
                xmin = 1000000.0f64;
                xmax = -(xmin as i32) as f64;
                ymin = xmin;
                ymax = xmax;
                let mut for_end_2: i32 = 0;
                i = 0i32 as small_number;
                for_end_2 = 3i32;
                if i as i32 <= for_end_2 {
                    loop {
                        if (corners[i as usize].x as f64) < xmin {
                            xmin = corners[i as usize].x as f64
                        }
                        if corners[i as usize].x as f64 > xmax {
                            xmax = corners[i as usize].x as f64
                        }
                        if (corners[i as usize].y as f64) < ymin {
                            ymin = corners[i as usize].y as f64
                        }
                        if corners[i as usize].y as f64 > ymax {
                            ymax = corners[i as usize].y as f64
                        }
                        let fresh3 = i;
                        i = i + 1;
                        if !((fresh3 as i32) < for_end_2) {
                            break;
                        }
                    }
                }
                if x_size_req == 0.0f64 {
                    make_scale(
                        &mut t2,
                        y_size_req / (ymax - ymin),
                        y_size_req / (ymax - ymin),
                    );
                } else if y_size_req == 0.0f64 {
                    make_scale(
                        &mut t2,
                        x_size_req / (xmax - xmin),
                        x_size_req / (xmax - xmin),
                    );
                } else {
                    make_scale(
                        &mut t2,
                        x_size_req / (xmax - xmin),
                        y_size_req / (ymax - ymin),
                    );
                }
                let mut for_end_3: i32 = 0;
                i = 0i32 as small_number;
                for_end_3 = 3i32;
                if i as i32 <= for_end_3 {
                    loop {
                        transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                        let fresh4 = i;
                        i = i + 1;
                        if !((fresh4 as i32) < for_end_3) {
                            break;
                        }
                    }
                }
                x_size_req = 0.0f64;
                y_size_req = 0.0f64;
                transform_concat(&mut t, &mut t2);
            }
            make_rotation(
                &mut t2,
                Fix2D(cur_val) * 3.14159265358979323846f64 / 180.0f64,
            );
            let mut for_end_4: i32 = 0;
            i = 0i32 as small_number;
            for_end_4 = 3i32;
            if i as i32 <= for_end_4 {
                loop {
                    transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                    let fresh5 = i;
                    i = i + 1;
                    if !((fresh5 as i32) < for_end_4) {
                        break;
                    }
                }
            }
            xmin = 1000000.0f64;
            xmax = -(xmin as i32) as f64;
            ymin = xmin;
            ymax = xmax;
            let mut for_end_5: i32 = 0;
            i = 0i32 as small_number;
            for_end_5 = 3i32;
            if i as i32 <= for_end_5 {
                loop {
                    if (corners[i as usize].x as f64) < xmin {
                        xmin = corners[i as usize].x as f64
                    }
                    if corners[i as usize].x as f64 > xmax {
                        xmax = corners[i as usize].x as f64
                    }
                    if (corners[i as usize].y as f64) < ymin {
                        ymin = corners[i as usize].y as f64
                    }
                    if corners[i as usize].y as f64 > ymax {
                        ymax = corners[i as usize].y as f64
                    }
                    let fresh6 = i;
                    i = i + 1;
                    if !((fresh6 as i32) < for_end_5) {
                        break;
                    }
                }
            }
            corners[0].x = xmin as f32;
            corners[0].y = ymin as f32;
            corners[1].x = xmin as f32;
            corners[1].y = ymax as f32;
            corners[2].x = xmax as f32;
            corners[2].y = ymax as f32;
            corners[3].x = xmax as f32;
            corners[3].y = ymin as f32;
            transform_concat(&mut t, &mut t2);
        } else {
            check_keywords = false
        }
    }
    if x_size_req != 0.0f64 || y_size_req != 0.0f64 {
        xmin = 1000000.0f64;
        xmax = -(xmin as i32) as f64;
        ymin = xmin;
        ymax = xmax;
        let mut for_end_6: i32 = 0;
        i = 0i32 as small_number;
        for_end_6 = 3i32;
        if i as i32 <= for_end_6 {
            loop {
                if (corners[i as usize].x as f64) < xmin {
                    xmin = corners[i as usize].x as f64
                }
                if corners[i as usize].x as f64 > xmax {
                    xmax = corners[i as usize].x as f64
                }
                if (corners[i as usize].y as f64) < ymin {
                    ymin = corners[i as usize].y as f64
                }
                if corners[i as usize].y as f64 > ymax {
                    ymax = corners[i as usize].y as f64
                }
                let fresh7 = i;
                i = i + 1;
                if !((fresh7 as i32) < for_end_6) {
                    break;
                }
            }
        }
        if x_size_req == 0.0f64 {
            make_scale(
                &mut t2,
                y_size_req / (ymax - ymin),
                y_size_req / (ymax - ymin),
            );
        } else if y_size_req == 0.0f64 {
            make_scale(
                &mut t2,
                x_size_req / (xmax - xmin),
                x_size_req / (xmax - xmin),
            );
        } else {
            make_scale(
                &mut t2,
                x_size_req / (xmax - xmin),
                y_size_req / (ymax - ymin),
            );
        }
        let mut for_end_7: i32 = 0;
        i = 0i32 as small_number;
        for_end_7 = 3i32;
        if i as i32 <= for_end_7 {
            loop {
                transform_point(&mut *corners.as_mut_ptr().offset(i as isize), &mut t2);
                let fresh8 = i;
                i = i + 1;
                if !((fresh8 as i32) < for_end_7) {
                    break;
                }
            }
        }
        x_size_req = 0.0f64;
        y_size_req = 0.0f64;
        transform_concat(&mut t, &mut t2);
    }
    xmin = 1000000.0f64;
    xmax = -(xmin as i32) as f64;
    ymin = xmin;
    ymax = xmax;
    let mut for_end_8: i32 = 0;
    i = 0i32 as small_number;
    for_end_8 = 3i32;
    if i as i32 <= for_end_8 {
        loop {
            if (corners[i as usize].x as f64) < xmin {
                xmin = corners[i as usize].x as f64
            }
            if corners[i as usize].x as f64 > xmax {
                xmax = corners[i as usize].x as f64
            }
            if (corners[i as usize].y as f64) < ymin {
                ymin = corners[i as usize].y as f64
            }
            if corners[i as usize].y as f64 > ymax {
                ymax = corners[i as usize].y as f64
            }
            let fresh9 = i;
            i = i + 1;
            if !((fresh9 as i32) < for_end_8) {
                break;
            }
        }
    }
    make_translation(
        &mut t2,
        (-(xmin as i32) * 72i32) as f64 / 72.27f64,
        (-(ymin as i32) * 72i32) as f64 / 72.27f64,
    );
    transform_concat(&mut t, &mut t2);
    if result == 0i32 {
        new_whatsit(
            43i32 as small_number,
            (9usize).wrapping_add(
                strlen(pic_path)
                    .wrapping_add(::std::mem::size_of::<memory_word>())
                    .wrapping_sub(1)
                    .wrapping_div(::std::mem::size_of::<memory_word>()),
            ) as small_number,
        );
        if is_pdf {
            (*mem.offset(cur_list.tail as isize)).b16.s0 = 44_u16
        }
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 = strlen(pic_path) as u16;
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s0 = page as u16;
        (*mem.offset((cur_list.tail + 8i32) as isize)).b16.s1 = pdf_box_type as u16;
        (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s1 = D2Fix(xmax - xmin);
        (*mem.offset((cur_list.tail + 3i32) as isize)).b32.s1 = D2Fix(ymax - ymin);
        (*mem.offset((cur_list.tail + 2i32) as isize)).b32.s1 = 0i32;
        (*mem.offset((cur_list.tail + 5i32) as isize)).b32.s0 = D2Fix(t.a);
        (*mem.offset((cur_list.tail + 5i32) as isize)).b32.s1 = D2Fix(t.b);
        (*mem.offset((cur_list.tail + 6i32) as isize)).b32.s0 = D2Fix(t.c);
        (*mem.offset((cur_list.tail + 6i32) as isize)).b32.s1 = D2Fix(t.d);
        (*mem.offset((cur_list.tail + 7i32) as isize)).b32.s0 = D2Fix(t.x);
        (*mem.offset((cur_list.tail + 7i32) as isize)).b32.s1 = D2Fix(t.y);
        memcpy(
            &mut *mem.offset((cur_list.tail + 9i32) as isize) as *mut memory_word as *mut u8
                as *mut libc::c_void,
            pic_path as *const libc::c_void,
            strlen(pic_path),
        );
        free(pic_path as *mut libc::c_void);
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const i8);
        }
        print_cstr(b"Unable to load picture or PDF file \'\x00" as *const u8 as *const i8);
        print_file_name(cur_name, cur_area, cur_ext);
        print('\'' as i32);
        if result == -43i32 {
            help_ptr = 2_u8;
            help_line[1] =
                b"The requested image couldn\'t be read because\x00" as *const u8 as *const i8;
            help_line[0] = b"the file was not found.\x00" as *const u8 as *const i8
        } else {
            help_ptr = 2_u8;
            help_line[1] =
                b"The requested image couldn\'t be read because\x00" as *const u8 as *const i8;
            help_line[0] = b"it was not a recognized image format.\x00" as *const u8 as *const i8
        }
        error();
    };
}
