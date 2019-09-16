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

use super::dpx_dvi::{
    dvi_close, dvi_comment, dvi_do_page, dvi_init, dvi_npages, dvi_reset_global_state,
    dvi_scan_specials, dvi_set_verbose,
};
use super::dpx_pdfdev::{pdf_dev_reset_global_state, pdf_dev_set_verbose};
use super::dpx_pdfdoc::pdf_doc_set_mediabox;
use super::dpx_pdfdoc::{
    pdf_close_document, pdf_doc_set_creator, pdf_doc_set_verbose, pdf_open_document,
};
use super::dpx_pdffont::{
    pdf_font_reset_unique_tag_state, pdf_font_set_deterministic_unique_tags, pdf_font_set_dpi,
};
use super::dpx_tt_aux::tt_aux_set_verbose;
use crate::dpx_pdfparse::parse_unsigned;
use crate::{info, warn};

use libc::free;
extern "C" {
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn atoi(__nptr: *const i8) -> i32;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    #[no_mangle]
    fn pdf_obj_set_verbose(level: i32);
    #[no_mangle]
    fn pdf_obj_reset_global_state();
    #[no_mangle]
    fn pdf_set_version(version: u32);
    #[no_mangle]
    fn pdf_get_version() -> u32;
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
    fn pdf_files_init();
    #[no_mangle]
    fn CIDFont_set_flags(flags: i32);
    #[no_mangle]
    fn pdf_set_use_predictor(bval: i32);
    #[no_mangle]
    fn pdf_files_close();
    #[no_mangle]
    fn pdf_close_fontmaps();
    #[no_mangle]
    fn pdf_load_fontmap_file(filename: *const i8, mode: i32) -> i32;
    #[no_mangle]
    fn pdf_fontmap_set_verbose(level: i32);
    #[no_mangle]
    fn pdf_set_compression(level: i32);
    #[no_mangle]
    fn pdf_init_fontmaps();
    #[no_mangle]
    fn paperinfo(ppformat: *const i8) -> *const paper;
    #[no_mangle]
    fn dpx_file_set_verbose(level: i32);
    #[no_mangle]
    fn dpx_delete_old_cache(life: i32);
    #[no_mangle]
    fn parse_float_decimal(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    #[no_mangle]
    fn parse_c_ident(pp: *mut *const i8, endptr: *const i8) -> *mut i8;
    /* The name transform_info is misleading.
     * I'll put this here for a moment...
     */
    /* Physical dimensions
     *
     * If those values are given, images will be scaled
     * and/or shifted to fit within a box described by
     * those values.
     */
    /* transform matrix */
    /* user_bbox */
    #[no_mangle]
    fn shut_up(quietness: i32);
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn pdf_init_device(unit_conv: f64, precision: i32, is_bw: i32);
    #[no_mangle]
    fn pdf_close_device();
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_enc_set_verbose(level: i32);
    #[no_mangle]
    fn pdf_enc_compute_id_string(dviname: *const i8, pdfname: *const i8);
    #[no_mangle]
    fn pdf_enc_set_passwd(size: u32, perm: u32, owner: *const i8, user: *const i8);
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    #[no_mangle]
    fn tpic_set_fill_mode(mode: i32);
    /* current page in PDF */
    /* This should not use pdf_. */
    /* PDF parser shouldn't depend on this...
     */
    #[no_mangle]
    fn spc_exec_at_end_document() -> i32;
    #[no_mangle]
    fn spc_exec_at_begin_document() -> i32;
    #[no_mangle]
    fn tfm_reset_global_state();
    #[no_mangle]
    fn vf_reset_global_state();
}

pub type PageRange = page_range;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_range {
    pub first: i32,
    pub last: i32,
}
use super::dpx_pdfdev::pdf_rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paper {
    pub name: *const i8,
    pub pswidth: f64,
    pub psheight: f64,
}
#[no_mangle]
pub static mut is_xdv: i32 = 0i32;
#[no_mangle]
pub static mut translate_origin: i32 = 0i32;
static mut ignore_colors: i8 = 0_i8;
static mut annot_grow: f64 = 0.0f64;
static mut bookmark_open: i32 = 0i32;
static mut mag: f64 = 1.0f64;
static mut font_dpi: i32 = 600i32;
/*
 * Precision is essentially limited to 0.01pt.
 * See, dev_set_string() in pdfdev.c.
 */
static mut pdfdecimaldigits: i32 = 3i32;
/* Image cache life in hours */
/*  0 means erase all old images and leave new images */
/* -1 means erase all old images and also erase new images */
/* -2 means ignore image cache (default) */
static mut image_cache_life: i32 = -2i32;
/* Encryption */
static mut do_encryption: i32 = 0i32;
static mut key_bits: i32 = 40i32;
static mut permission: i32 = 0x3ci32;
/* Page device */
#[no_mangle]
pub static mut paper_width: f64 = 595.0f64;
#[no_mangle]
pub static mut paper_height: f64 = 842.0f64;
static mut x_offset: f64 = 72.0f64;
static mut y_offset: f64 = 72.0f64;
#[no_mangle]
pub static mut landscape_mode: i32 = 0i32;
#[no_mangle]
pub static mut always_embed: i32 = 0i32;
/* always embed fonts, regardless of licensing flags */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
unsafe extern "C" fn read_length(
    mut vp: *mut f64,
    mut pp: *mut *const i8,
    mut endptr: *const i8,
) -> i32 {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = *pp;
    let mut v: f64 = 0.;
    let mut u: f64 = 1.0f64;
    let mut _ukeys: [*const i8; 10] = [
        b"pt\x00" as *const u8 as *const i8,
        b"in\x00" as *const u8 as *const i8,
        b"cm\x00" as *const u8 as *const i8,
        b"mm\x00" as *const u8 as *const i8,
        b"bp\x00" as *const u8 as *const i8,
        b"pc\x00" as *const u8 as *const i8,
        b"dd\x00" as *const u8 as *const i8,
        b"cc\x00" as *const u8 as *const i8,
        b"sp\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut k: i32 = 0;
    let mut error: i32 = 0i32;
    q = parse_float_decimal(&mut p, endptr);
    if q.is_null() {
        *vp = 0.0f64;
        *pp = p;
        return -1i32;
    }
    v = atof(q);
    free(q as *mut libc::c_void);
    skip_white(&mut p, endptr);
    q = parse_c_ident(&mut p, endptr);
    if !q.is_null() {
        let mut qq: *mut i8 = q;
        if strlen(q) >= strlen(b"true\x00" as *const u8 as *const i8)
            && memcmp(
                q as *const libc::c_void,
                b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
                strlen(b"true\x00" as *const u8 as *const i8),
            ) == 0
        {
            q = q.offset(strlen(b"true\x00" as *const u8 as *const i8) as isize)
            /* just skip "true" */
        }
        if strlen(q) == 0i32 as u64 {
            free(qq as *mut libc::c_void);
            skip_white(&mut p, endptr);
            q = parse_c_ident(&mut p, endptr);
            qq = q
        }
        if !q.is_null() {
            k = 0i32;
            while !_ukeys[k as usize].is_null() && strcmp(_ukeys[k as usize], q) != 0 {
                k += 1
            }
            match k {
                0 => u *= 72.0f64 / 72.27f64,
                1 => u *= 72.0f64,
                2 => u *= 72.0f64 / 2.54f64,
                3 => u *= 72.0f64 / 25.4f64,
                4 => u *= 1.0f64,
                5 => u *= 12.0f64 * 72.0f64 / 72.27f64,
                6 => u *= 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                7 => u *= 12.0f64 * 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64,
                8 => u *= 72.0f64 / (72.27f64 * 65536i32 as f64),
                _ => {
                    dpx_warning(
                        b"Unknown unit of measure: %s\x00" as *const u8 as *const i8,
                        q,
                    );
                    error = -1i32
                }
            }
            free(qq as *mut libc::c_void);
        } else {
            warn!("Missing unit of measure after \"true\"");
            error = -1i32
        }
    }
    *vp = v * u;
    *pp = p;
    error
}
unsafe extern "C" fn select_paper(mut paperspec: *const i8) {
    let mut pi: *const paper = 0 as *const paper;
    let mut error: i32 = 0i32;
    pi = paperinfo(paperspec);
    if !pi.is_null()
        && !(if !pi.is_null() && !(*pi).name.is_null() {
            (*pi).name
        } else {
            0 as *const i8
        })
        .is_null()
    {
        paper_width = if !pi.is_null() && !(*pi).name.is_null() {
            (*pi).pswidth
        } else {
            0.0f64
        };
        paper_height = if !pi.is_null() && !(*pi).name.is_null() {
            (*pi).psheight
        } else {
            0.0f64
        }
    } else {
        let mut p: *const i8 = paperspec;
        let mut endptr: *const i8 = 0 as *const i8;
        let mut comma: *const i8 = 0 as *const i8;
        comma = strchr(p, ',' as i32);
        endptr = p.offset(strlen(p) as isize);
        if comma.is_null() {
            _tt_abort(
                b"Unrecognized paper format: %s\x00" as *const u8 as *const i8,
                paperspec,
            );
        }
        error = read_length(&mut paper_width, &mut p, comma);
        p = comma.offset(1);
        error = read_length(&mut paper_height, &mut p, endptr)
    }
    if error != 0 || paper_width <= 0.0f64 || paper_height <= 0.0f64 {
        _tt_abort(
            b"Invalid paper size: %s (%.2fx%.2f)\x00" as *const u8 as *const i8,
            paperspec,
            paper_width,
            paper_height,
        );
    };
}
unsafe extern "C" fn select_pages(
    mut pagespec: *const i8,
    mut ret_page_ranges: *mut *mut PageRange,
    mut ret_num_page_ranges: *mut u32,
) {
    let mut page_ranges: *mut PageRange = 0 as *mut PageRange;
    let mut num_page_ranges: u32 = 0_u32;
    let mut max_page_ranges: u32 = 0_u32;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = pagespec;
    while *p as i32 != '\u{0}' as i32 {
        /* Enlarge page range table if necessary */
        if num_page_ranges >= max_page_ranges {
            max_page_ranges = max_page_ranges.wrapping_add(4_u32); /* Can't be signed. */
            page_ranges = renew(
                page_ranges as *mut libc::c_void,
                (max_page_ranges as u64).wrapping_mul(::std::mem::size_of::<PageRange>() as u64)
                    as u32,
            ) as *mut PageRange
        }
        (*page_ranges.offset(num_page_ranges as isize)).first = 0i32;
        (*page_ranges.offset(num_page_ranges as isize)).last = 0i32;
        while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
        q = parse_unsigned(&mut p, p.offset(strlen(p) as isize));
        if !q.is_null() {
            /* '-' is allowed here */
            (*page_ranges.offset(num_page_ranges as isize)).first = atoi(q) - 1i32; /* Root node */
            (*page_ranges.offset(num_page_ranges as isize)).last =
                (*page_ranges.offset(num_page_ranges as isize)).first;
            free(q as *mut libc::c_void);
        }
        while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
            p = p.offset(1)
        }
        if *p as i32 == '-' as i32 {
            p = p.offset(1);
            while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
                p = p.offset(1)
            }
            (*page_ranges.offset(num_page_ranges as isize)).last = -1i32;
            if *p != 0 {
                q = parse_unsigned(&mut p, p.offset(strlen(p) as isize));
                if !q.is_null() {
                    (*page_ranges.offset(num_page_ranges as isize)).last = atoi(q) - 1i32;
                    free(q as *mut libc::c_void);
                }
                while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
                    p = p.offset(1)
                }
            }
        } else {
            (*page_ranges.offset(num_page_ranges as isize)).last =
                (*page_ranges.offset(num_page_ranges as isize)).first
        }
        num_page_ranges = num_page_ranges.wrapping_add(1);
        if *p as i32 == ',' as i32 {
            p = p.offset(1)
        } else {
            while *p as i32 != 0 && libc::isspace(*p as _) != 0 {
                p = p.offset(1)
            }
            if *p != 0 {
                _tt_abort(
                    b"Bad page range specification: %s\x00" as *const u8 as *const i8,
                    p,
                );
            }
        }
    }
    *ret_page_ranges = page_ranges;
    *ret_num_page_ranges = num_page_ranges;
}
unsafe extern "C" fn system_default() {
    if !(b"a4\x00" as *const u8 as *const i8).is_null() {
        select_paper(b"a4\x00" as *const u8 as *const i8);
    } else if !(b"a4\x00" as *const u8 as *const i8).is_null() {
        select_paper(b"a4\x00" as *const u8 as *const i8);
    };
}
unsafe extern "C" fn do_dvi_pages(mut page_ranges: *mut PageRange, mut num_page_ranges: u32) {
    let mut page_no: i32 = 0;
    let mut step: i32 = 0;
    let mut page_count: u32 = 0;
    let mut i: u32 = 0;
    let mut page_width: f64 = 0.;
    let mut page_height: f64 = 0.;
    let mut init_paper_width: f64 = 0.;
    let mut init_paper_height: f64 = 0.;
    let mut mediabox = pdf_rect::new();
    spc_exec_at_begin_document();
    page_width = paper_width;
    init_paper_width = page_width;
    page_height = paper_height;
    init_paper_height = page_height;
    page_count = 0_u32;
    mediabox.llx = 0.0f64;
    mediabox.lly = 0.0f64;
    mediabox.urx = paper_width;
    mediabox.ury = paper_height;
    pdf_doc_set_mediabox(0_u32, &mut mediabox);
    i = 0_u32;
    while i < num_page_ranges && dvi_npages() != 0 {
        if (*page_ranges.offset(i as isize)).last < 0i32 {
            let ref mut fresh0 = (*page_ranges.offset(i as isize)).last;
            *fresh0 = (*fresh0 as u32).wrapping_add(dvi_npages()) as i32 as i32
        }
        step = if (*page_ranges.offset(i as isize)).first <= (*page_ranges.offset(i as isize)).last
        {
            1i32
        } else {
            -1i32
        };
        page_no = (*page_ranges.offset(i as isize)).first;
        while dvi_npages() != 0 {
            if (page_no as u32) < dvi_npages() {
                let mut w: f64 = 0.;
                let mut h: f64 = 0.;
                let mut xo: f64 = 0.;
                let mut yo: f64 = 0.;
                let mut lm: i32 = 0;
                info!("[{}", page_no + 1i32);
                /* Users want to change page size even after page is started! */
                page_width = paper_width;
                page_height = paper_height;
                w = page_width;
                h = page_height;
                lm = landscape_mode;
                xo = x_offset;
                yo = y_offset;
                dvi_scan_specials(
                    page_no,
                    &mut w,
                    &mut h,
                    &mut xo,
                    &mut yo,
                    &mut lm,
                    0 as *mut i32,
                    0 as *mut i32,
                    0 as *mut i32,
                    0 as *mut i32,
                    0 as *mut i32,
                    0 as *mut i8,
                    0 as *mut i8,
                );
                if lm != landscape_mode {
                    let mut _tmp: f64 = w;
                    w = h;
                    h = _tmp;
                    landscape_mode = lm
                }
                if page_width != w || page_height != h {
                    page_width = w;
                    page_height = h
                }
                if x_offset != xo || y_offset != yo {
                    x_offset = xo;
                    y_offset = yo
                }
                if page_width != init_paper_width || page_height != init_paper_height {
                    mediabox.llx = 0.0f64;
                    mediabox.lly = 0.0f64;
                    mediabox.urx = page_width;
                    mediabox.ury = page_height;
                    pdf_doc_set_mediabox(page_count.wrapping_add(1_u32), &mut mediabox);
                }
                dvi_do_page(page_height, x_offset, y_offset);
                page_count = page_count.wrapping_add(1);
                info!("]");
            }
            if step > 0i32 && page_no >= (*page_ranges.offset(i as isize)).last {
                break;
            }
            if step < 0i32 && page_no <= (*page_ranges.offset(i as isize)).last {
                break;
            }
            page_no += step
        }
        i = i.wrapping_add(1)
    }
    if page_count < 1_u32 {
        panic!("No pages fall in range!");
    }
    spc_exec_at_end_document();
}
#[no_mangle]
pub unsafe extern "C" fn dvipdfmx_main(
    mut pdf_filename: *const i8,
    mut dvi_filename: *const i8,
    mut pagespec: *const i8,
    mut opt_flags: i32,
    mut translate: bool,
    mut compress: bool,
    mut deterministic_tags: bool,
    mut quiet: bool,
    mut verbose: u32,
) -> i32 {
    let mut enable_object_stream: bool = true; /* This must come before parsing options... */
    let mut dvi2pts: f64 = 0.;
    let mut num_page_ranges: u32 = 0_u32;
    let mut page_ranges: *mut PageRange = 0 as *mut PageRange;
    assert!(!pdf_filename.is_null());
    assert!(!dvi_filename.is_null());
    translate_origin = translate as i32;
    dvi_reset_global_state();
    tfm_reset_global_state();
    vf_reset_global_state();
    pdf_dev_reset_global_state();
    pdf_obj_reset_global_state();
    pdf_font_reset_unique_tag_state();
    if quiet {
        shut_up(2i32);
    } else {
        dvi_set_verbose(verbose as i32);
        pdf_dev_set_verbose(verbose as i32);
        pdf_doc_set_verbose(verbose as i32);
        pdf_enc_set_verbose(verbose as i32);
        pdf_obj_set_verbose(verbose as i32);
        pdf_fontmap_set_verbose(verbose as i32);
        dpx_file_set_verbose(verbose as i32);
        tt_aux_set_verbose(verbose as i32);
    }
    pdf_set_compression(if compress as i32 != 0 { 9i32 } else { 0i32 });
    pdf_font_set_deterministic_unique_tags(if deterministic_tags as i32 != 0 {
        1i32
    } else {
        0i32
    });
    system_default();
    pdf_init_fontmaps();
    /* We used to read the config file here. It synthesized command-line
     * arguments, so we emulate the default TeXLive config file by copying those
     * code bits. */
    pdf_set_version(5_u32); /* last page */
    select_paper(b"letter\x00" as *const u8 as *const i8);
    annot_grow = 0i32 as f64;
    bookmark_open = 0i32;
    key_bits = 40i32;
    permission = 0x3ci32;
    font_dpi = 600i32;
    pdfdecimaldigits = 5i32;
    image_cache_life = -2i32;
    pdf_load_fontmap_file(b"pdftex.map\x00" as *const u8 as *const i8, '+' as i32);
    pdf_load_fontmap_file(b"kanjix.map\x00" as *const u8 as *const i8, '+' as i32);
    pdf_load_fontmap_file(b"ckx.map\x00" as *const u8 as *const i8, '+' as i32);
    if !pagespec.is_null() {
        select_pages(pagespec, &mut page_ranges, &mut num_page_ranges);
    }
    if page_ranges.is_null() {
        page_ranges = new((1_u64).wrapping_mul(::std::mem::size_of::<PageRange>() as u64) as u32)
            as *mut PageRange
    }
    if num_page_ranges == 0_u32 {
        (*page_ranges.offset(0)).first = 0i32;
        (*page_ranges.offset(0)).last = -1i32;
        num_page_ranges = 1_u32
    }
    /*kpse_init_prog("", font_dpi, NULL, NULL);
    kpse_set_program_enabled(kpse_pk_format, true, kpse_src_texmf_cnf);*/
    pdf_font_set_dpi(font_dpi);
    dpx_delete_old_cache(image_cache_life);
    pdf_enc_compute_id_string(dvi_filename, pdf_filename);
    let mut ver_major: i32 = 0i32;
    let mut ver_minor: i32 = 0i32;
    let mut owner_pw: [i8; 127] = [0; 127];
    let mut user_pw: [i8; 127] = [0; 127];
    /* Dependency between DVI and PDF side is rather complicated... */
    dvi2pts = dvi_init(dvi_filename, mag);
    if dvi2pts == 0.0f64 {
        panic!("dvi_init() failed!");
    }
    pdf_doc_set_creator(dvi_comment());
    dvi_scan_specials(
        0i32,
        &mut paper_width,
        &mut paper_height,
        &mut x_offset,
        &mut y_offset,
        &mut landscape_mode,
        &mut ver_major,
        &mut ver_minor,
        &mut do_encryption,
        &mut key_bits,
        &mut permission,
        owner_pw.as_mut_ptr(),
        user_pw.as_mut_ptr(),
    );
    if ver_minor >= 3i32 && ver_minor <= 7i32 {
        pdf_set_version(ver_minor as u32);
    }
    if do_encryption != 0 {
        if !(key_bits >= 40i32 && key_bits <= 128i32 && key_bits % 8i32 == 0i32)
            && key_bits != 256i32
        {
            panic!("Invalid encryption key length specified: {}", key_bits);
        } else {
            if key_bits > 40i32 && pdf_get_version() < 4_u32 {
                panic!("Chosen key length requires at least PDF 1.4. Use \"-V 4\" to change.");
            }
        }
        do_encryption = 1i32;
        pdf_enc_set_passwd(
            key_bits as u32,
            permission as u32,
            owner_pw.as_mut_ptr(),
            user_pw.as_mut_ptr(),
        );
    }
    if landscape_mode != 0 {
        let mut _tmp: f64 = paper_width;
        paper_width = paper_height;
        paper_height = _tmp
    }
    pdf_files_init();
    if opt_flags & 1i32 << 6i32 != 0 {
        enable_object_stream = false
    }
    /* Set default paper size here so that all page's can inherite it.
     * annot_grow:    Margin of annotation.
     * bookmark_open: Miximal depth of open bookmarks.
     */
    pdf_open_document(
        pdf_filename,
        do_encryption != 0,
        enable_object_stream,
        paper_width,
        paper_height,
        annot_grow,
        bookmark_open,
        (opt_flags & 1i32 << 4i32 == 0) as i32,
    );
    /* Ignore_colors placed here since
     * they are considered as device's capacity.
     */
    pdf_init_device(dvi2pts, pdfdecimaldigits, ignore_colors as i32);
    if opt_flags & 1i32 << 2i32 != 0 {
        CIDFont_set_flags(1i32 << 1i32);
    }
    /* Please move this to spc_init_specials(). */
    if opt_flags & 1i32 << 1i32 != 0 {
        tpic_set_fill_mode(1i32); /* No prediction */
    }
    if opt_flags & 1i32 << 5i32 != 0 {
        pdf_set_use_predictor(0i32);
    }
    do_dvi_pages(page_ranges, num_page_ranges);
    pdf_files_close();
    /* Order of close... */
    pdf_close_device();
    /* pdf_close_document flushes XObject (image) and other resources. */
    pdf_close_document(); /* pdf_font may depend on fontmap. */
    pdf_close_fontmaps();
    dvi_close();
    info!("\n");
    free(page_ranges as *mut libc::c_void);
    0i32
}
