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

use crate::dpx_pdfparse::{parse_number, parse_pdf_dict, parse_pdf_object, parse_unsigned};
use crate::mfree;
use crate::{info, warn};
use crate::{streq_ptr, strstartswith};
use std::ffi::CStr;

use super::dpx_dpxutil::{ht_append_table, ht_clear_table, ht_init_table, ht_lookup_table};
use super::dpx_mfileio::work_buffer;
use crate::{
    ttstub_input_get_size, ttstub_input_getc, ttstub_input_read, ttstub_input_seek,
    ttstub_input_ungetc, ttstub_output_close, ttstub_output_open, ttstub_output_open_stdout,
    ttstub_output_putc, ttstub_output_write,
};
use libc::free;
extern "C" {
    #[no_mangle]
    fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    fn atoi(__nptr: *const i8) -> i32;
    #[no_mangle]
    fn strtoul(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
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
    #[no_mangle]
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(buffer: *mut i8, length: i32, file: rust_input_handle_t) -> *mut i8;
    #[no_mangle]
    fn pdf_encrypt_data(
        plain: *const u8,
        plain_len: size_t,
        cipher: *mut *mut u8,
        cipher_len: *mut size_t,
    );
    #[no_mangle]
    fn pdf_enc_set_label(label: u32);
    #[no_mangle]
    fn pdf_enc_set_generation(generation: u32);
    #[no_mangle]
    fn skip_white(start: *mut *const i8, end: *const i8);
    #[no_mangle]
    fn pdf_sprint_number(buf: *mut i8, value: f64) -> i32;
}

use libz_sys as libz;

pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_output_handle_t = *mut libc::c_void;
pub type rust_input_handle_t = *mut libc::c_void;

pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;

use super::dpx_dpxutil::ht_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_obj {
    pub typ: i32,
    pub label: u32,
    pub generation: u16,
    pub refcount: u32,
    pub flags: i32,
    pub data: *mut libc::c_void,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PdfObjType {
    BOOLEAN = 1,
    NUMBER = 2,
    STRING = 3,
    NAME = 4,
    ARRAY = 5,
    DICT = 6,
    STREAM = 7,
    NULL = 8,
    INDIRECT = 9,
    UNDEFINED = 10,
    OBJ_INVALID = 0,
}

impl From<i32> for PdfObjType {
    fn from(t: i32) -> Self {
        use PdfObjType::*;
        match t {
            1 => BOOLEAN,
            2 => NUMBER,
            3 => STRING,
            4 => NAME,
            5 => ARRAY,
            6 => DICT,
            7 => STREAM,
            8 => NULL,
            9 => INDIRECT,
            10 => UNDEFINED,
            0 => OBJ_INVALID,
            _ => panic!("Invalid object type: {}", t),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_file {
    pub handle: rust_input_handle_t,
    pub trailer: *mut pdf_obj,
    pub xref_table: *mut xref_entry,
    pub catalog: *mut pdf_obj,
    pub num_obj: i32,
    pub file_size: i32,
    pub version: u32,
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xref_entry {
    pub typ: u8,
    pub field2: u32,
    pub field3: u16,
    pub direct: *mut pdf_obj,
    pub indirect: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_dict {
    pub key: *mut pdf_obj,
    pub value: *mut pdf_obj,
    pub next: *mut pdf_dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_stream {
    pub dict: *mut pdf_obj,
    pub stream: *mut u8,
    pub objstm_data: *mut i32,
    pub stream_length: u32,
    pub max_length: u32,
    pub _flags: i32,
    pub decodeparms: decode_parms,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decode_parms {
    pub predictor: i32,
    pub colors: i32,
    pub bits_per_component: i32,
    pub columns: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_name {
    pub name: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_indirect {
    pub pf: *mut pdf_file,
    pub obj: *mut pdf_obj,
    pub label: u32,
    pub generation: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_array {
    pub max: u32,
    pub size: u32,
    pub values: *mut *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_string {
    pub string: *mut u8,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_number {
    pub value: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_boolean {
    pub value: i8,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
static mut pdf_output_handle: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut pdf_output_file_position: i32 = 0i32;
static mut pdf_output_line_position: i32 = 0i32;
static mut compression_saved: i32 = 0i32;
static mut format_buffer: [i8; 4096] = [0; 4096];
static mut output_xref: *mut xref_entry = 0 as *const xref_entry as *mut xref_entry;
static mut pdf_max_ind_objects: u32 = 0;
static mut next_label: u32 = 0;
static mut startxref: u32 = 0;
static mut output_stream: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
/* the limit is only 100 for linearized PDF */
static mut enc_mode: bool = false;
static mut doc_enc_mode: bool = false;
static mut trailer_dict: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
static mut xref_stream: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
static mut verbose: i32 = 0i32;
static mut compression_level: i8 = 9_i8;
static mut compression_use_predictor: i8 = 1_i8;
#[no_mangle]
pub unsafe extern "C" fn pdf_set_compression(mut level: i32) {
    if cfg!(not(feature = "libz-sys")) {
        panic!(
            "You don\'t have compression compiled in. Possibly libz wasn\'t found by configure."
        );
    }
    if cfg!(feature = "legacy-libz") && level != 0i32 {
        warn!("Unable to set compression level -- your zlib doesn\'t have compress2().");
    }
    if level >= 0i32 && level <= 9i32 {
        compression_level = level as i8
    } else {
        panic!("set_compression: invalid compression level: {}", level);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_use_predictor(mut bval: i32) {
    compression_use_predictor = (if bval != 0 { 1i32 } else { 0i32 }) as i8;
}
static mut pdf_version: u32 = 5_u32;
#[no_mangle]
pub unsafe extern "C" fn pdf_set_version(mut version: u32) {
    /* Don't forget to update CIDFont_stdcc_def[] in cid.c too! */
    if version >= 3_u32 && version <= 7_u32 {
        pdf_version = version
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_version() -> u32 {
    pdf_version
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_get_verbose() -> i32 {
    verbose
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_set_verbose(mut level: i32) {
    verbose = level;
}
static mut current_objstm: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
static mut do_objstm: i32 = 0;
unsafe extern "C" fn add_xref_entry(mut label: u32, mut typ: u8, mut field2: u32, mut field3: u16) {
    if label >= pdf_max_ind_objects {
        pdf_max_ind_objects = label
            .wrapping_div(512_u32)
            .wrapping_add(1_u32)
            .wrapping_mul(512_u32);
        output_xref = renew(
            output_xref as *mut libc::c_void,
            (pdf_max_ind_objects as u64).wrapping_mul(::std::mem::size_of::<xref_entry>() as u64)
                as u32,
        ) as *mut xref_entry
    }
    (*output_xref.offset(label as isize)).typ = typ;
    (*output_xref.offset(label as isize)).field2 = field2;
    (*output_xref.offset(label as isize)).field3 = field3;
    let ref mut fresh0 = (*output_xref.offset(label as isize)).direct;
    *fresh0 = 0 as *mut pdf_obj;
    let ref mut fresh1 = (*output_xref.offset(label as isize)).indirect;
    *fresh1 = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_out_init(
    mut filename: *const i8,
    mut do_encryption: bool,
    mut enable_object_stream: bool,
) {
    let mut v: i8 = 0;
    output_xref = 0 as *mut xref_entry;
    pdf_max_ind_objects = 0_u32;
    add_xref_entry(0_u32, 0_u8, 0_u32, 0xffff_u16);
    next_label = 1_u32;
    if pdf_version >= 5_u32 {
        if enable_object_stream {
            xref_stream = pdf_new_stream(1i32 << 0i32);
            (*xref_stream).flags |= 1i32 << 1i32;
            trailer_dict = pdf_stream_dict(xref_stream);
            pdf_add_dict(
                trailer_dict,
                pdf_new_name(b"Type\x00" as *const u8 as *const i8),
                pdf_new_name(b"XRef\x00" as *const u8 as *const i8),
            );
            do_objstm = 1i32
        } else {
            trailer_dict = pdf_new_dict();
            do_objstm = 0i32
        }
    } else {
        xref_stream = 0 as *mut pdf_obj;
        trailer_dict = pdf_new_dict();
        do_objstm = 0i32
    }
    output_stream = 0 as *mut pdf_obj;
    if filename.is_null() {
        panic!("stdout PDF output not supported");
    }
    pdf_output_handle = ttstub_output_open(filename, 0i32);
    if pdf_output_handle.is_null() {
        if strlen(filename) < 128i32 as u64 {
            _tt_abort(
                b"Unable to open \"%s\".\x00" as *const u8 as *const i8,
                filename,
            );
        } else {
            panic!("Unable to open file.");
        }
    }
    pdf_out(
        pdf_output_handle,
        b"%PDF-1.\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"%PDF-1.\x00" as *const u8 as *const i8) as i32,
    );
    v = ('0' as i32 as u32).wrapping_add(pdf_version) as i8;
    pdf_out(
        pdf_output_handle,
        &mut v as *mut i8 as *const libc::c_void,
        1i32,
    );
    pdf_out(
        pdf_output_handle,
        b"\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        1i32,
    );
    pdf_out(
        pdf_output_handle,
        b"%\xe4\xf0\xed\xf8\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"%\xe4\xf0\xed\xf8\n\x00" as *const u8 as *const i8) as i32,
    );
    enc_mode = false;
    doc_enc_mode = do_encryption;
}
unsafe extern "C" fn dump_xref_table() {
    let mut length: i32 = 0;
    let mut i: u32 = 0;
    pdf_out(
        pdf_output_handle,
        b"xref\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        5i32,
    );
    length = sprintf(
        format_buffer.as_mut_ptr(),
        b"%d %u\n\x00" as *const u8 as *const i8,
        0i32,
        next_label,
    );
    pdf_out(
        pdf_output_handle,
        format_buffer.as_mut_ptr() as *const libc::c_void,
        length,
    );
    /*
     * Every space counts.  The space after the 'f' and 'n' is * *essential*.
     * The PDF spec says the lines must be 20 characters long including the
     * end of line character.
     */
    i = 0_u32;
    while i < next_label {
        let mut typ: u8 = (*output_xref.offset(i as isize)).typ;
        if typ as i32 > 1i32 {
            _tt_abort(
                b"object type %c not allowed in xref table\x00" as *const u8 as *const i8,
                typ as i32,
            );
        }
        length = sprintf(
            format_buffer.as_mut_ptr(),
            b"%010u %05hu %c \n\x00" as *const u8 as *const i8,
            (*output_xref.offset(i as isize)).field2,
            (*output_xref.offset(i as isize)).field3 as i32,
            if typ as i32 != 0 {
                'n' as i32
            } else {
                'f' as i32
            },
        );
        pdf_out(
            pdf_output_handle,
            format_buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn dump_trailer_dict() {
    pdf_out(
        pdf_output_handle,
        b"trailer\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        8i32,
    );
    enc_mode = false;
    write_dict((*trailer_dict).data as *mut pdf_dict, pdf_output_handle);
    pdf_release_obj(trailer_dict);
    pdf_out_char(pdf_output_handle, '\n' as i32 as i8);
}
/*
 * output a PDF 1.5 cross-reference stream;
 * contributed by Matthias Franz (March 21, 2007)
 */
unsafe extern "C" fn dump_xref_stream() {
    let mut pos: u32 = 0;
    let mut i: u32 = 0;
    let mut poslen: u32 = 0;
    let mut buf: [u8; 7] = [0; 7];
    let mut w: *mut pdf_obj = 0 as *mut pdf_obj;
    /* determine the necessary size of the offset field */
    pos = startxref; /* maximal offset value */
    poslen = 1_u32; /* type                */
    loop {
        pos >>= 8i32; /* offset (big-endian) */
        if !(pos != 0) {
            break; /* generation          */
        }
        poslen = poslen.wrapping_add(1)
    }
    w = pdf_new_array();
    pdf_add_array(w, pdf_new_number(1i32 as f64));
    pdf_add_array(w, pdf_new_number(poslen as f64));
    pdf_add_array(w, pdf_new_number(2i32 as f64));
    pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"W\x00" as *const u8 as *const i8),
        w,
    );
    /* We need the xref entry for the xref stream right now */
    add_xref_entry(next_label.wrapping_sub(1_u32), 1_u8, startxref, 0_u16);
    i = 0_u32;
    while i < next_label {
        let mut j: u32 = 0;
        let mut f3: u16 = 0;
        buf[0] = (*output_xref.offset(i as isize)).typ;
        pos = (*output_xref.offset(i as isize)).field2;
        j = poslen;
        loop {
            let fresh2 = j;
            j = j.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            buf[(1_u32).wrapping_add(j) as usize] = pos as u8;
            pos >>= 8i32
        }
        f3 = (*output_xref.offset(i as isize)).field3;
        buf[poslen.wrapping_add(1_u32) as usize] = (f3 as i32 >> 8i32) as u8;
        buf[poslen.wrapping_add(2_u32) as usize] = f3 as u8;
        pdf_add_stream(
            xref_stream,
            &mut buf as *mut [u8; 7] as *const libc::c_void,
            poslen.wrapping_add(3_u32) as i32,
        );
        i = i.wrapping_add(1)
    }
    pdf_release_obj(xref_stream);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_out_flush() {
    if !pdf_output_handle.is_null() {
        let mut length: i32 = 0;
        /* Flush current object stream */
        if !current_objstm.is_null() {
            release_objstm(current_objstm);
            current_objstm = 0 as *mut pdf_obj
        }
        /*
         * Label xref stream - we need the number of correct objects
         * for the xref stream dictionary (= trailer).
         * Labelling it in pdf_out_init (with 1)  does not work (why?).
         */
        if !xref_stream.is_null() {
            pdf_label_obj(xref_stream);
        }
        /* Record where this xref is for trailer */
        startxref = pdf_output_file_position as u32;
        pdf_add_dict(
            trailer_dict,
            pdf_new_name(b"Size\x00" as *const u8 as *const i8),
            pdf_new_number(next_label as f64),
        );
        if !xref_stream.is_null() {
            dump_xref_stream();
        } else {
            dump_xref_table();
            dump_trailer_dict();
        }
        /* Done with xref table */
        free(output_xref as *mut libc::c_void);
        pdf_out(
            pdf_output_handle,
            b"startxref\n\x00" as *const u8 as *const i8 as *const libc::c_void,
            10i32,
        );
        length = sprintf(
            format_buffer.as_mut_ptr(),
            b"%u\n\x00" as *const u8 as *const i8,
            startxref,
        );
        pdf_out(
            pdf_output_handle,
            format_buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
        pdf_out(
            pdf_output_handle,
            b"%%EOF\n\x00" as *const u8 as *const i8 as *const libc::c_void,
            6i32,
        );
        if verbose != 0 {
            if compression_level as i32 > 0i32 {
                info!(
                    "Compression saved {} bytes{}\n",
                    compression_saved,
                    if pdf_version < 5_u32 {
                        ". Try \"-V 5\" for better compression"
                    } else {
                        ""
                    },
                );
            }
        }
        ttstub_output_close(pdf_output_handle);
        pdf_output_handle = 0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_error_cleanup() {
    /*
     * This routine is the cleanup required for an abnormal exit.
     * For now, simply close the file.
     */
    if !pdf_output_handle.is_null() {
        ttstub_output_close(pdf_output_handle);
        pdf_output_handle = 0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_root(mut object: *mut pdf_obj) {
    if pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"Root\x00" as *const u8 as *const i8),
        pdf_ref_obj(object),
    ) != 0
    {
        panic!("Root object already set!");
    }
    /* Adobe Readers don't like a document catalog inside an encrypted
     * object stream, although the PDF v1.5 spec seems to allow this.
     * Note that we don't set OBJ_NO_ENCRYPT since the name dictionary in
     * a document catalog may contain strings, which should be encrypted.
     */
    if doc_enc_mode {
        (*object).flags |= 1i32 << 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_info(mut object: *mut pdf_obj) {
    if pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"Info\x00" as *const u8 as *const i8),
        pdf_ref_obj(object),
    ) != 0
    {
        panic!("Info object already set!");
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_id(mut id: *mut pdf_obj) {
    if pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"ID\x00" as *const u8 as *const i8),
        id,
    ) != 0
    {
        panic!("ID already set!");
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_encrypt(mut encrypt: *mut pdf_obj) {
    if pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"Encrypt\x00" as *const u8 as *const i8),
        pdf_ref_obj(encrypt),
    ) != 0
    {
        panic!("Encrypt object already set!");
    }
    (*encrypt).flags |= 1i32 << 1i32;
}
unsafe extern "C" fn pdf_out_char(mut handle: rust_output_handle_t, mut c: i8) {
    if !output_stream.is_null() && handle == pdf_output_handle {
        pdf_add_stream(
            output_stream,
            &mut c as *mut i8 as *const libc::c_void,
            1i32,
        );
    } else {
        ttstub_output_putc(handle, c as i32);
        /* Keep tallys for xref table *only* if writing a pdf file. */
        if handle == pdf_output_handle {
            pdf_output_file_position += 1i32;
            if c as i32 == '\n' as i32 {
                pdf_output_line_position = 0i32
            } else {
                pdf_output_line_position += 1i32
            }
        }
    };
}
static mut xchar: [i8; 17] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 0,
];
unsafe extern "C" fn pdf_out(
    mut handle: rust_output_handle_t,
    mut buffer: *const libc::c_void,
    mut length: i32,
) {
    if !output_stream.is_null() && handle == pdf_output_handle {
        pdf_add_stream(output_stream, buffer, length);
    } else {
        ttstub_output_write(handle, buffer as *const i8, length as size_t);
        /* Keep tallys for xref table *only* if writing a pdf file */
        if handle == pdf_output_handle {
            pdf_output_file_position += length;
            pdf_output_line_position += length;
            /* "foo\nbar\n "... */
            if length > 0i32
                && *(buffer as *const i8).offset((length - 1i32) as isize) as i32 == '\n' as i32
            {
                pdf_output_line_position = 0i32
            }
        }
    };
}
/*  returns 1 if a white-space character is necessary to separate
an object of type1 followed by an object of type2              */
unsafe extern "C" fn pdf_need_white(mut type1: i32, mut type2: i32) -> i32 {
    return !(type1 == 3i32
        || type1 == 5i32
        || type1 == 6i32
        || type2 == 3i32
        || type2 == 4i32
        || type2 == 5i32
        || type2 == 6i32) as i32;
}
unsafe extern "C" fn pdf_out_white(mut handle: rust_output_handle_t) {
    if handle == pdf_output_handle && pdf_output_line_position >= 80i32 {
        pdf_out_char(handle, '\n' as i32 as i8);
    } else {
        pdf_out_char(handle, ' ' as i32 as i8);
    };
}
unsafe extern "C" fn pdf_new_obj(mut typ: i32) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(typ >= 0 && typ <= 10, "Invalid object type: {}", typ);
    result =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_obj>() as u64) as u32) as *mut pdf_obj;
    (*result).typ = typ;
    (*result).data = 0 as *mut libc::c_void;
    (*result).label = 0_u32;
    (*result).generation = 0_u16;
    (*result).refcount = 1_u32;
    (*result).flags = 0i32;
    result
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_typeof(object: *mut pdf_obj) -> PdfObjType {
    if (*object).typ <= 0i32 || (*object).typ > 10i32 {
        PdfObjType::OBJ_INVALID
    } else {
        PdfObjType::from((*object).typ)
    }
}
unsafe extern "C" fn pdf_label_obj(mut object: *mut pdf_obj) {
    if object.is_null() || (*object).typ <= 0i32 || (*object).typ > 10i32 {
        panic!("pdf_label_obj(): passed invalid object.");
    }
    /*
     * Don't change label on an already labeled object. Ignore such calls.
     */
    if (*object).label == 0_u32 {
        let fresh3 = next_label;
        next_label = next_label.wrapping_add(1);
        (*object).label = fresh3;
        (*object).generation = 0_u16
    };
}
/*
 * Transfer the label assigned to the object src to the object dst.
 * The object dst must not yet have been labeled.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_transfer_label(mut dst: *mut pdf_obj, mut src: *mut pdf_obj) {
    assert!(!dst.is_null() && (*dst).label == 0 && !src.is_null());
    (*dst).label = (*src).label;
    (*dst).generation = (*src).generation;
    (*src).label = 0_u32;
    (*src).generation = 0_u16;
}
/*
 * This doesn't really copy the object, but allows it to be used without
 * fear that somebody else will free it.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_link_obj(mut object: *mut pdf_obj) -> *mut pdf_obj {
    if object.is_null() || (*object).typ <= 0i32 || (*object).typ > 10i32 {
        panic!("pdf_link_obj(): passed invalid object.");
    }
    (*object).refcount = (*object).refcount.wrapping_add(1_u32);
    object
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ref_obj(mut object: *mut pdf_obj) -> *mut pdf_obj {
    if object.is_null() || (*object).typ <= 0i32 || (*object).typ > 10i32 {
        panic!("pdf_ref_obj(): passed invalid object.");
    }
    if (*object).refcount == 0_u32 {
        info!("\nTrying to refer already released object!!!\n");
        pdf_write_obj(object, ttstub_output_open_stdout());
        panic!("Cannot continue...");
    }
    if !object.is_null() && pdf_obj_typeof(object) == PdfObjType::INDIRECT {
        return pdf_link_obj(object);
    } else {
        return pdf_new_ref(object);
    };
}
unsafe extern "C" fn release_indirect(mut data: *mut pdf_indirect) {
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn write_indirect(
    mut indirect: *mut pdf_indirect,
    mut handle: rust_output_handle_t,
) {
    let mut length: i32 = 0;
    assert!((*indirect).pf.is_null());
    length = sprintf(
        format_buffer.as_mut_ptr(),
        b"%u %hu R\x00" as *const u8 as *const i8,
        (*indirect).label,
        (*indirect).generation as i32,
    );
    pdf_out(
        handle,
        format_buffer.as_mut_ptr() as *const libc::c_void,
        length,
    );
}
/* The undefined object is used as a placeholder in pdfnames.c
 * for objects which are referenced before they are defined.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_new_undefined() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    result = pdf_new_obj(10i32);
    (*result).data = 0 as *mut libc::c_void;
    result
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_null() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    result = pdf_new_obj(8i32);
    (*result).data = 0 as *mut libc::c_void;
    result
}
unsafe extern "C" fn write_null(mut handle: rust_output_handle_t) {
    pdf_out(
        handle,
        b"null\x00" as *const u8 as *const i8 as *const libc::c_void,
        4i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_boolean(mut value: i8) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_boolean = 0 as *mut pdf_boolean;
    result = pdf_new_obj(1i32);
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_boolean>() as u64) as u32)
        as *mut pdf_boolean;
    (*data).value = value;
    (*result).data = data as *mut libc::c_void;
    result
}
unsafe extern "C" fn release_boolean(mut data: *mut pdf_obj) {
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn write_boolean(mut data: *mut pdf_boolean, mut handle: rust_output_handle_t) {
    if (*data).value != 0 {
        pdf_out(
            handle,
            b"true\x00" as *const u8 as *const i8 as *const libc::c_void,
            4i32,
        );
    } else {
        pdf_out(
            handle,
            b"false\x00" as *const u8 as *const i8 as *const libc::c_void,
            5i32,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_boolean_value(mut object: *mut pdf_obj) -> i8 {
    let mut data: *mut pdf_boolean = 0 as *mut pdf_boolean;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::BOOLEAN {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            1i32,
            808i32,
        );
    }
    data = (*object).data as *mut pdf_boolean;
    (*data).value
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_number(mut value: f64) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_number = 0 as *mut pdf_number;
    result = pdf_new_obj(2i32);
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_number>() as u64) as u32)
        as *mut pdf_number;
    (*data).value = value;
    (*result).data = data as *mut libc::c_void;
    result
}
unsafe extern "C" fn release_number(mut data: *mut pdf_number) {
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn write_number(mut number: *mut pdf_number, mut handle: rust_output_handle_t) {
    let mut count: i32 = 0;
    count = pdf_sprint_number(format_buffer.as_mut_ptr(), (*number).value);
    pdf_out(
        handle,
        format_buffer.as_mut_ptr() as *const libc::c_void,
        count,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_number(mut object: *mut pdf_obj, mut value: f64) {
    let mut data: *mut pdf_number = 0 as *mut pdf_number;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::NUMBER {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            2i32,
            851i32,
        );
    }
    data = (*object).data as *mut pdf_number;
    (*data).value = value;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_number_value(mut object: *mut pdf_obj) -> f64 {
    let mut data: *mut pdf_number = 0 as *mut pdf_number;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::NUMBER {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            2i32,
            862i32,
        );
    }
    data = (*object).data as *mut pdf_number;
    (*data).value
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_string(
    mut str: *const libc::c_void,
    mut length: size_t,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    assert!(!str.is_null());
    result = pdf_new_obj(3i32);
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_string>() as u64) as u32)
        as *mut pdf_string;
    (*result).data = data as *mut libc::c_void;
    (*data).length = length;
    if length != 0 {
        (*data).string = new((length.wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        libc::memcpy((*data).string as *mut libc::c_void, str, length as usize);
        /* Shouldn't assume NULL terminated. */
        *(*data).string.offset(length as isize) = '\u{0}' as i32 as u8
    } else {
        (*data).string = 0 as *mut u8
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn pdf_string_value(mut object: *mut pdf_obj) -> *mut libc::c_void {
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::STRING {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            3i32,
            898i32,
        );
    }
    data = (*object).data as *mut pdf_string;
    (*data).string as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn pdf_string_length(mut object: *mut pdf_obj) -> u32 {
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::STRING {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            3i32,
            910i32,
        );
    }
    data = (*object).data as *mut pdf_string;
    (*data).length as u32
}
/*
 * This routine escapes non printable characters and control
 * characters in an output string.
 */
#[no_mangle]
pub unsafe extern "C" fn pdfobj_escape_str(
    mut buffer: *mut i8,
    mut bufsize: size_t,
    mut s: *const u8,
    mut len: size_t,
) -> size_t {
    let mut result: size_t = 0i32 as size_t;
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < len {
        let mut ch: u8 = 0;
        ch = *s.offset(i as isize);
        if result > bufsize.wrapping_sub(4i32 as u64) {
            panic!("pdfobj_escape_str: Buffer overflow");
        }
        /*
         * We always write three octal digits. Optimization only gives few Kb
         * smaller size for most documents when zlib compressed.
         */
        if (ch as i32) < 32i32 || ch as i32 > 126i32 {
            let fresh4 = result; /* Shouldn't use format_buffer[]. */
            result = result.wrapping_add(1);
            *buffer.offset(fresh4 as isize) = '\\' as i32 as i8;
            result = (result as u64).wrapping_add(sprintf(
                buffer.offset(result as isize),
                b"%03o\x00" as *const u8 as *const i8,
                ch as i32,
            ) as u64) as size_t as size_t
        } else {
            match ch as i32 {
                40 => {
                    let fresh5 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh5 as isize) = '\\' as i32 as i8;
                    let fresh6 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh6 as isize) = '(' as i32 as i8
                }
                41 => {
                    let fresh7 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh7 as isize) = '\\' as i32 as i8;
                    let fresh8 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh8 as isize) = ')' as i32 as i8
                }
                92 => {
                    let fresh9 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh9 as isize) = '\\' as i32 as i8;
                    let fresh10 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh10 as isize) = '\\' as i32 as i8
                }
                _ => {
                    let fresh11 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh11 as isize) = ch as i8
                }
            }
        }
        i = i.wrapping_add(1)
    }
    result
}
unsafe extern "C" fn write_string(mut str: *mut pdf_string, mut handle: rust_output_handle_t) {
    let mut s: *mut u8 = 0 as *mut u8;
    let mut wbuf: [i8; 4096] = [0; 4096];
    let mut nescc: i32 = 0i32;
    let mut count: i32 = 0;
    let mut i: size_t = 0;
    let mut len: size_t = 0i32 as size_t;
    if enc_mode {
        pdf_encrypt_data((*str).string, (*str).length, &mut s, &mut len);
    } else {
        s = (*str).string;
        len = (*str).length
    }
    /*
     * Count all ASCII non-printable characters.
     */
    i = 0i32 as size_t;
    while i < len {
        if libc::isprint(*s.offset(i as isize) as _) == 0 {
            nescc += 1
        }
        i = i.wrapping_add(1)
    }
    /*
     * If the string contains much escaped chars, then we write it as
     * ASCII hex string.
     */
    if nescc as u64 > len.wrapping_div(3i32 as u64) {
        pdf_out_char(handle, '<' as i32 as i8);
        i = 0i32 as size_t;
        while i < len {
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as i32 >> 4i32 & 0xfi32) as usize],
            );
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as i32 & 0xfi32) as usize],
            );
            i = i.wrapping_add(1)
        }
        pdf_out_char(handle, '>' as i32 as i8);
    } else {
        pdf_out_char(handle, '(' as i32 as i8);
        /*
         * This section of code probably isn't speed critical.  Escaping the
         * characters in the string one at a time may seem slow, but it's
         * safe if the formatted string length exceeds FORMAT_BUF_SIZE.
         * Occasionally you see some long strings in PDF.  pdfobj_escape_str
         * is also used for strings of text with no kerning.  These must be
         * handled as quickly as possible since there are so many of them.
         */
        i = 0i32 as size_t;
        while i < len {
            count = pdfobj_escape_str(
                wbuf.as_mut_ptr(),
                4096i32 as size_t,
                &mut *s.offset(i as isize),
                1i32 as size_t,
            ) as i32;
            pdf_out(handle, wbuf.as_mut_ptr() as *const libc::c_void, count);
            i = i.wrapping_add(1)
        }
        pdf_out_char(handle, ')' as i32 as i8);
    }
    if enc_mode as i32 != 0 && !s.is_null() {
        free(s as *mut libc::c_void);
    };
}
unsafe extern "C" fn release_string(mut data: *mut pdf_string) {
    (*data).string = mfree((*data).string as *mut libc::c_void) as *mut u8;
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_string(
    mut object: *mut pdf_obj,
    mut str: *mut u8,
    mut length: size_t,
) {
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::STRING {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            3i32,
            1029i32,
        );
    }
    data = (*object).data as *mut pdf_string;
    if !(*data).string.is_null() {
        free((*data).string as *mut libc::c_void);
    }
    if length != 0i32 as u64 {
        (*data).length = length;
        (*data).string = new((length.wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        libc::memcpy(
            (*data).string as *mut libc::c_void,
            str as *const libc::c_void,
            length as usize,
        );
        *(*data).string.offset(length as isize) = '\u{0}' as i32 as u8
    } else {
        (*data).length = 0i32 as size_t;
        (*data).string = 0 as *mut u8
    };
}
/* Name does *not* include the /. */
#[no_mangle]
pub unsafe extern "C" fn pdf_new_name(mut name: *const i8) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut length: u32 = 0;
    let mut data: *mut pdf_name = 0 as *mut pdf_name;
    result = pdf_new_obj(4i32);
    data =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_name>() as u64) as u32) as *mut pdf_name;
    (*result).data = data as *mut libc::c_void;
    length = strlen(name) as u32;
    if length != 0_u32 {
        (*data).name = new((length.wrapping_add(1_u32) as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
            as *mut i8;
        libc::memcpy(
            (*data).name as *mut libc::c_void,
            name as *const libc::c_void,
            length as usize,
        );
        *(*data).name.offset(length as isize) = '\u{0}' as i32 as i8
    } else {
        (*data).name = 0 as *mut i8
    }
    result
}
unsafe extern "C" fn write_name(mut name: *mut pdf_name, mut handle: rust_output_handle_t) {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut length: i32 = 0;
    s = (*name).name;
    length = (if !(*name).name.is_null() {
        strlen((*name).name)
    } else {
        0i32 as u64
    }) as i32;
    /*
     * From PDF Reference, 3rd ed., p.33:
     *
     *  Beginning with PDF 1.2, any character except null (character code 0)
     *  may be included in a name by writing its 2-digit hexadecimal code,
     *  preceded bythe number sign character (#); see implementation notes 3
     *  and 4 in Appendix H. This syntax is required in order to represent
     *  any of the delimiter or white-space characters or the number sign
     *  character itself; it is recommended but not required for characters
     *  whose codes are outside the range 33 (!) to 126 (~).
     */
    pdf_out_char(handle, '/' as i32 as i8);
    i = 0i32;
    while i < length {
        if (*s.offset(i as isize) as i32) < '!' as i32
            || *s.offset(i as isize) as i32 > '~' as i32
            || *s.offset(i as isize) as i32 == '#' as i32
            || (*s.offset(i as isize) as i32 == '(' as i32
                || *s.offset(i as isize) as i32 == ')' as i32
                || *s.offset(i as isize) as i32 == '/' as i32
                || *s.offset(i as isize) as i32 == '<' as i32
                || *s.offset(i as isize) as i32 == '>' as i32
                || *s.offset(i as isize) as i32 == '[' as i32
                || *s.offset(i as isize) as i32 == ']' as i32
                || *s.offset(i as isize) as i32 == '{' as i32
                || *s.offset(i as isize) as i32 == '}' as i32
                || *s.offset(i as isize) as i32 == '%' as i32)
        {
            /*     ^ "space" is here. */
            pdf_out_char(handle, '#' as i32 as i8);
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as i32 >> 4i32 & 0xfi32) as usize],
            );
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as i32 & 0xfi32) as usize],
            );
        } else {
            pdf_out_char(handle, *s.offset(i as isize));
        }
        i += 1
    }
}
unsafe extern "C" fn release_name(mut data: *mut pdf_name) {
    (*data).name = mfree((*data).name as *mut libc::c_void) as *mut i8;
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_name_value(mut object: *mut pdf_obj) -> *mut i8 {
    let mut data: *mut pdf_name = 0 as *mut pdf_name;
    if object.is_null() || PdfObjType::from((*object).typ) != PdfObjType::NAME {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !object.is_null() {
                (*object).typ
            } else {
                -1i32
            },
            4i32,
            1121i32,
        );
    }
    data = (*object).data as *mut pdf_name;
    (*data).name
}
/*
 * We do not have pdf_name_length() since '\0' is not allowed
 * in PDF name object.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_new_array() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    result = pdf_new_obj(5i32);
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_array>() as u64) as u32)
        as *mut pdf_array;
    (*data).values = 0 as *mut *mut pdf_obj;
    (*data).max = 0_u32;
    (*data).size = 0_u32;
    (*result).data = data as *mut libc::c_void;
    result
}
unsafe extern "C" fn write_array(mut array: *mut pdf_array, mut handle: rust_output_handle_t) {
    pdf_out_char(handle, '[' as i32 as i8);
    if (*array).size > 0_u32 {
        let mut i: u32 = 0;
        let mut type1: i32 = 10i32;
        let mut type2: i32 = 0;
        i = 0_u32;
        while i < (*array).size {
            if !(*(*array).values.offset(i as isize)).is_null() {
                type2 = (**(*array).values.offset(i as isize)).typ;
                if type1 != 10i32 && pdf_need_white(type1, type2) != 0 {
                    pdf_out_white(handle);
                }
                type1 = type2;
                pdf_write_obj(*(*array).values.offset(i as isize), handle);
            } else {
                warn!("PDF array element {} undefined.", i);
            }
            i = i.wrapping_add(1)
        }
    }
    pdf_out_char(handle, ']' as i32 as i8);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_array(mut array: *mut pdf_obj, mut idx: i32) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    if array.is_null() || PdfObjType::from((*array).typ) != PdfObjType::ARRAY {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !array.is_null() {
                (*array).typ
            } else {
                -1i32
            },
            5i32,
            1177i32,
        );
    }
    data = (*array).data as *mut pdf_array;
    if idx < 0i32 {
        result = *(*data)
            .values
            .offset((idx as u32).wrapping_add((*data).size) as isize)
    } else if (idx as u32) < (*data).size {
        result = *(*data).values.offset(idx as isize)
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn pdf_array_length(mut array: *mut pdf_obj) -> u32 {
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    if array.is_null() || PdfObjType::from((*array).typ) != PdfObjType::ARRAY {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !array.is_null() {
                (*array).typ
            } else {
                -1i32
            },
            5i32,
            1194i32,
        );
    }
    data = (*array).data as *mut pdf_array;
    (*data).size
}
unsafe extern "C" fn release_array(mut data: *mut pdf_array) {
    let mut i: u32 = 0;
    if !(*data).values.is_null() {
        i = 0_u32;
        while i < (*data).size {
            pdf_release_obj(*(*data).values.offset(i as isize));
            let ref mut fresh12 = *(*data).values.offset(i as isize);
            *fresh12 = 0 as *mut pdf_obj;
            i = i.wrapping_add(1)
        }
        (*data).values = mfree((*data).values as *mut libc::c_void) as *mut *mut pdf_obj
    }
    free(data as *mut libc::c_void);
}
/*
 * The name pdf_add_array is misleading. It behaves differently than
 * pdf_add_dict(). This should be pdf_push_array().
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_add_array(mut array: *mut pdf_obj, mut object: *mut pdf_obj) {
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    if array.is_null() || PdfObjType::from((*array).typ) != PdfObjType::ARRAY {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !array.is_null() {
                (*array).typ
            } else {
                -1i32
            },
            5i32,
            1225i32,
        );
    }
    data = (*array).data as *mut pdf_array;
    if (*data).size >= (*data).max {
        (*data).max = (*data).max.wrapping_add(256_u32);
        (*data).values = renew(
            (*data).values as *mut libc::c_void,
            ((*data).max as u64).wrapping_mul(::std::mem::size_of::<*mut pdf_obj>() as u64) as u32,
        ) as *mut *mut pdf_obj
    }
    let ref mut fresh13 = *(*data).values.offset((*data).size as isize);
    *fresh13 = object;
    (*data).size = (*data).size.wrapping_add(1);
}
/* Prepend an object to an array */
unsafe extern "C" fn pdf_unshift_array(mut array: *mut pdf_obj, mut object: *mut pdf_obj) {
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    if array.is_null() || PdfObjType::from((*array).typ) != PdfObjType::ARRAY {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !array.is_null() {
                (*array).typ
            } else {
                -1i32
            },
            5i32,
            1245i32,
        );
    }
    data = (*array).data as *mut pdf_array;
    if (*data).size >= (*data).max {
        (*data).max = (*data).max.wrapping_add(256i32 as libc::c_uint);
        (*data).values = renew(
            (*data).values as *mut libc::c_void,
            ((*data).max as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut pdf_obj>() as libc::c_ulong)
                as u32,
        ) as *mut *mut pdf_obj
    }
    libc::memmove(
        &mut *(*data).values.offset(1) as *mut *mut pdf_obj as *mut libc::c_void,
        (*data).values as *const libc::c_void,
        ((*data).size as usize).wrapping_mul(::std::mem::size_of::<*mut pdf_obj>() as usize),
    );
    let ref mut fresh14 = *(*data).values.offset(0);
    *fresh14 = object;
    (*data).size = (*data).size.wrapping_add(1);
}
unsafe extern "C" fn write_dict(mut dict: *mut pdf_dict, mut handle: rust_output_handle_t) {
    pdf_out(
        handle,
        b"<<\x00" as *const u8 as *const i8 as *const libc::c_void,
        2i32,
    );
    while !(*dict).key.is_null() {
        pdf_write_obj((*dict).key, handle);
        if pdf_need_white(4i32, (*(*dict).value).typ) != 0 {
            pdf_out_white(handle);
        }
        pdf_write_obj((*dict).value, handle);
        dict = (*dict).next
    }
    pdf_out(
        handle,
        b">>\x00" as *const u8 as *const i8 as *const libc::c_void,
        2i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_dict() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    result = pdf_new_obj(6i32);
    data =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_dict>() as u64) as u32) as *mut pdf_dict;
    (*data).key = 0 as *mut pdf_obj;
    (*data).value = 0 as *mut pdf_obj;
    (*data).next = 0 as *mut pdf_dict;
    (*result).data = data as *mut libc::c_void;
    result
}
unsafe extern "C" fn release_dict(mut data: *mut pdf_dict) {
    let mut next: *mut pdf_dict = 0 as *mut pdf_dict;
    while !data.is_null() && !(*data).key.is_null() {
        pdf_release_obj((*data).key);
        pdf_release_obj((*data).value);
        (*data).key = 0 as *mut pdf_obj;
        (*data).value = 0 as *mut pdf_obj;
        next = (*data).next;
        free(data as *mut libc::c_void);
        data = next
    }
    free(data as *mut libc::c_void);
}
/* Array is ended by a node with NULL this pointer */
/* pdf_add_dict returns 0 if the key is new and non-zero otherwise */
#[no_mangle]
pub unsafe extern "C" fn pdf_add_dict(
    mut dict: *mut pdf_obj,
    mut key: *mut pdf_obj,
    mut value: *mut pdf_obj,
) -> i32 {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    let mut new_node: *mut pdf_dict = 0 as *mut pdf_dict;
    if dict.is_null() || PdfObjType::from((*dict).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict.is_null() { (*dict).typ } else { -1i32 },
            6i32,
            1313i32,
        );
    }
    if key.is_null() || PdfObjType::from((*key).typ) != PdfObjType::NAME {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !key.is_null() { (*key).typ } else { -1i32 },
            4i32,
            1314i32,
        );
    }
    /* It seems that NULL is sometimes used for null object... */
    if !value.is_null() && (value.is_null() || (*value).typ <= 0i32 || (*value).typ > 10i32) {
        panic!("pdf_add_dict(): Passed invalid value");
    }
    /* If this key already exists, simply replace the value */
    data = (*dict).data as *mut pdf_dict;
    while !(*data).key.is_null() {
        if streq_ptr(pdf_name_value(key), pdf_name_value((*data).key)) {
            /* Release the old value */
            pdf_release_obj((*data).value);
            /* Release the new key (we don't need it) */
            pdf_release_obj(key);
            (*data).value = value;
            return 1i32;
        }
        data = (*data).next
    }
    /*
     * We didn't find the key. We build a new "end" node and add
     * the new key just before the end
     */
    new_node =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_dict>() as u64) as u32) as *mut pdf_dict;
    (*new_node).key = 0 as *mut pdf_obj;
    (*new_node).value = 0 as *mut pdf_obj;
    (*new_node).next = 0 as *mut pdf_dict;
    (*data).next = new_node;
    (*data).key = key;
    (*data).value = value;
    0i32
}
/* pdf_merge_dict makes a link for each item in dict2 before stealing it */
#[no_mangle]
pub unsafe extern "C" fn pdf_merge_dict(mut dict1: *mut pdf_obj, mut dict2: *mut pdf_obj) {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    if dict1.is_null() || PdfObjType::from((*dict1).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict1.is_null() {
                (*dict1).typ
            } else {
                -1i32
            },
            6i32,
            1352i32,
        );
    }
    if dict2.is_null() || PdfObjType::from((*dict2).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict2.is_null() {
                (*dict2).typ
            } else {
                -1i32
            },
            6i32,
            1353i32,
        );
    }
    data = (*dict2).data as *mut pdf_dict;
    while !(*data).key.is_null() {
        pdf_add_dict(
            dict1,
            pdf_link_obj((*data).key),
            pdf_link_obj((*data).value),
        );
        data = (*data).next
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_foreach_dict(
    mut dict: *mut pdf_obj,
    mut proc_0: Option<
        unsafe extern "C" fn(_: *mut pdf_obj, _: *mut pdf_obj, _: *mut libc::c_void) -> i32,
    >,
    mut pdata: *mut libc::c_void,
) -> i32 {
    let mut error: i32 = 0i32;
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    assert!(proc_0.is_some());
    if dict.is_null() || PdfObjType::from((*dict).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict.is_null() { (*dict).typ } else { -1i32 },
            6i32,
            1371i32,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    while error == 0 && !(*data).key.is_null() {
        error = proc_0.expect("non-null function pointer")((*data).key, (*data).value, pdata);
        data = (*data).next
    }
    error
}
#[no_mangle]
pub unsafe extern "C" fn pdf_lookup_dict(
    mut dict: *mut pdf_obj,
    mut name: *const i8,
) -> *mut pdf_obj {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    assert!(!name.is_null());
    if dict.is_null() || PdfObjType::from((*dict).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict.is_null() { (*dict).typ } else { -1i32 },
            6i32,
            1391i32,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    while !(*data).key.is_null() {
        if streq_ptr(name, pdf_name_value((*data).key)) {
            return (*data).value;
        }
        data = (*data).next
    }
    0 as *mut pdf_obj
}
/* Returns array of dictionary keys */
#[no_mangle]
pub unsafe extern "C" fn pdf_dict_keys(mut dict: *mut pdf_obj) -> *mut pdf_obj {
    let mut keys: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    if dict.is_null() || PdfObjType::from((*dict).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict.is_null() { (*dict).typ } else { -1i32 },
            6i32,
            1411i32,
        );
    }
    keys = pdf_new_array();
    data = (*dict).data as *mut pdf_dict;
    while !data.is_null() && !(*data).key.is_null() {
        /* We duplicate name object rather than linking keys.
         * If we forget to free keys, broken PDF is generated.
         */
        pdf_add_array(keys, pdf_new_name(pdf_name_value((*data).key)));
        data = (*data).next
    }
    keys
}
#[no_mangle]
pub unsafe extern "C" fn pdf_remove_dict(mut dict: *mut pdf_obj, mut name: *const i8) {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    let mut data_p: *mut *mut pdf_dict = 0 as *mut *mut pdf_dict;
    if dict.is_null() || PdfObjType::from((*dict).typ) != PdfObjType::DICT {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !dict.is_null() { (*dict).typ } else { -1i32 },
            6i32,
            1430i32,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    data_p = &mut (*dict).data as *mut *mut libc::c_void as *mut libc::c_void as *mut *mut pdf_dict;
    while !(*data).key.is_null() {
        if !(*data).key.is_null()
            && !name.is_null()
            && streq_ptr((*((*(*data).key).data as *mut pdf_name)).name, name) as i32 != 0
        {
            pdf_release_obj((*data).key);
            pdf_release_obj((*data).value);
            *data_p = (*data).next;
            free(data as *mut libc::c_void);
            break;
        } else {
            data_p = &mut (*data).next;
            data = (*data).next
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_stream(mut flags: i32) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    result = pdf_new_obj(7i32);
    data = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_stream>() as u64) as u32)
        as *mut pdf_stream;
    /*
     * Although we are using an arbitrary pdf_object here, it must have
     * type=PDF_DICT and cannot be an indirect reference.  This will be
     * checked by the output routine.
     */
    (*data).dict = pdf_new_dict();
    (*data)._flags = flags;
    (*data).stream = 0 as *mut u8;
    (*data).stream_length = 0_u32;
    (*data).max_length = 0_u32;
    (*data).objstm_data = 0 as *mut i32;
    (*data).decodeparms.predictor = 2i32;
    (*data).decodeparms.columns = 0i32;
    (*data).decodeparms.bits_per_component = 0i32;
    (*data).decodeparms.colors = 0i32;
    (*result).data = data as *mut libc::c_void;
    (*result).flags |= 1i32 << 0i32;
    result
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_set_predictor(
    mut stream: *mut pdf_obj,
    mut predictor: i32,
    mut columns: i32,
    mut bpc: i32,
    mut colors: i32,
) {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if pdf_obj_typeof(stream) != PdfObjType::STREAM {
        return;
    } else {
        if columns < 0i32 || bpc < 0i32 || colors < 0i32 {
            return;
        }
    }
    data = (*stream).data as *mut pdf_stream;
    (*data).decodeparms.predictor = predictor;
    (*data).decodeparms.columns = columns;
    (*data).decodeparms.bits_per_component = bpc;
    (*data).decodeparms.colors = colors;
    (*data)._flags |= 1i32 << 1i32;
}
/* Adaptive PNG filter
 * We use the "minimum sum of absolute differences" heuristic approach
 * for finding the most optimal filter to be used.
 *
 * From http://www.libpng.org/pub/png/book/chapter09.html
 *
 *   For grayscale and truecolor images of 8 or more bits per sample, with or
 *   without alpha channels, dynamic filtering is almost always beneficial. The
 *   approach that has by now become standard is known as the minimum sum of
 *   absolute differences heuristic and was first proposed by Lee Daniel
 *   Crocker in February 1995.
 */
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn filter_PNG15_apply_filter(
    mut raster: *mut libc::c_uchar,
    mut columns: i32,
    mut rows: i32,
    mut bpc: i8,
    mut colors: i8,
    mut length: *mut i32,
) -> *mut libc::c_uchar {
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bits_per_pixel: libc::c_int = colors as libc::c_int * bpc as libc::c_int;
    let mut bytes_per_pixel: libc::c_int = (bits_per_pixel + 7i32) / 8i32;
    let mut rowbytes: i32 = columns * bytes_per_pixel;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    assert!(!raster.is_null() && !length.is_null());
    /* Result */
    dst = new((((rowbytes + 1i32) * rows) as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as u32)
        as *mut libc::c_uchar;
    *length = (rowbytes + 1i32) * rows;
    j = 0i32;
    while j < rows {
        let mut typ: libc::c_int = 0i32;
        let mut pp: *mut libc::c_uchar = dst.offset((j * (rowbytes + 1i32)) as isize);
        let mut p: *mut libc::c_uchar = raster.offset((j * rowbytes) as isize);
        let mut sum: [u32; 5] = [
            0i32 as u32,
            0i32 as u32,
            0i32 as u32,
            0i32 as u32,
            0i32 as u32,
        ];
        /* First calculated sum of values to make a heuristic guess
         * of optimal predictor function.
         */
        i = 0i32;
        while i < rowbytes {
            let mut left: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                *p.offset((i - bytes_per_pixel) as isize) as libc::c_int
            } else {
                0i32
            };
            let mut up: libc::c_int = if j > 0i32 {
                *p.offset(i as isize).offset(-(rowbytes as isize)) as libc::c_int
            } else {
                0i32
            };
            let mut uplft: libc::c_int = if j > 0i32 {
                if i - bytes_per_pixel >= 0i32 {
                    *p.offset(i as isize)
                        .offset(-(rowbytes as isize))
                        .offset(-(bytes_per_pixel as isize)) as libc::c_int
                } else {
                    0i32
                }
            } else {
                0i32
            };
            /* Type 0 -- None */
            sum[0] = (sum[0] as libc::c_uint).wrapping_add(*p.offset(i as isize) as libc::c_uint)
                as u32 as u32;
            /* Type 1 -- Sub */
            sum[1] = (sum[1] as libc::c_uint)
                .wrapping_add((*p.offset(i as isize) as libc::c_int - left).abs() as libc::c_uint)
                as u32 as u32;
            /* Type 2 -- Up */
            sum[2] = (sum[2] as libc::c_uint)
                .wrapping_add((*p.offset(i as isize) as libc::c_int - up).abs() as libc::c_uint)
                as u32 as u32;
            /* Type 3 -- Average */
            let mut tmp: libc::c_int = (((up + left) / 2i32) as f64).floor() as libc::c_int;
            sum[3] = (sum[3] as libc::c_uint)
                .wrapping_add((*p.offset(i as isize) as libc::c_int - tmp).abs() as libc::c_uint)
                as u32 as u32;
            /* Type 4 -- Peath */
            let mut q: libc::c_int = left + up - uplft;
            let mut qa: libc::c_int = (q - left).abs();
            let mut qb: libc::c_int = (q - up).abs();
            let mut qc: libc::c_int = (q - uplft).abs();
            if qa <= qb && qa <= qc {
                sum[4] = (sum[4] as libc::c_uint).wrapping_add(
                    (*p.offset(i as isize) as libc::c_int - left).abs() as libc::c_uint,
                ) as u32 as u32
            } else if qb <= qc {
                sum[4] = (sum[4] as libc::c_uint)
                    .wrapping_add((*p.offset(i as isize) as libc::c_int - up).abs() as libc::c_uint)
                    as u32 as u32
            } else {
                sum[4] = (sum[4] as libc::c_uint).wrapping_add(
                    (*p.offset(i as isize) as libc::c_int - uplft).abs() as libc::c_uint,
                ) as u32 as u32
            }
            i += 1
        }
        let mut min: libc::c_int = sum[0] as libc::c_int;
        let mut min_idx: libc::c_int = 0i32;
        i = 0i32;
        while i < 5i32 {
            if sum[i as usize] < min as libc::c_uint {
                min = sum[i as usize] as libc::c_int;
                min_idx = i
            }
            i += 1
        }
        typ = min_idx;
        /* Now we actually apply filter. */
        *pp.offset(0) = typ as libc::c_uchar;
        match typ {
            0 => {
                libc::memcpy(
                    pp.offset(1) as *mut libc::c_void,
                    p as *const libc::c_void,
                    rowbytes as usize,
                );
            }
            1 => {
                i = 0i32;
                while i < rowbytes {
                    let mut left_0: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                        *p.offset((i - bytes_per_pixel) as isize) as libc::c_int
                    } else {
                        0i32
                    };
                    *pp.offset((i + 1i32) as isize) =
                        (*p.offset(i as isize) as libc::c_int - left_0) as libc::c_uchar;
                    i += 1
                }
            }
            2 => {
                i = 0i32;
                while i < rowbytes {
                    let mut up_0: libc::c_int = if j > 0i32 {
                        *p.offset(i as isize).offset(-(rowbytes as isize)) as libc::c_int
                    } else {
                        0i32
                    };
                    *pp.offset((i + 1i32) as isize) =
                        (*p.offset(i as isize) as libc::c_int - up_0) as libc::c_uchar;
                    i += 1
                }
            }
            3 => {
                i = 0i32;
                while i < rowbytes {
                    let mut up_1: libc::c_int = if j > 0i32 {
                        *p.offset(i as isize).offset(-(rowbytes as isize)) as libc::c_int
                    } else {
                        0i32
                    };
                    let mut left_1: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                        *p.offset((i - bytes_per_pixel) as isize) as libc::c_int
                    } else {
                        0i32
                    };
                    let mut tmp_0: libc::c_int =
                        (((up_1 + left_1) / 2i32) as f64).floor() as libc::c_int;
                    *pp.offset((i + 1i32) as isize) =
                        (*p.offset(i as isize) as libc::c_int - tmp_0) as libc::c_uchar;
                    i += 1
                }
            }
            4 => {
                /* Peath */
                i = 0i32;
                while i < rowbytes {
                    let mut up_2: libc::c_int = if j > 0i32 {
                        *p.offset(i as isize).offset(-(rowbytes as isize)) as libc::c_int
                    } else {
                        0i32
                    };
                    let mut left_2: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                        *p.offset((i - bytes_per_pixel) as isize) as libc::c_int
                    } else {
                        0i32
                    };
                    let mut uplft_0: libc::c_int = if j > 0i32 {
                        if i - bytes_per_pixel >= 0i32 {
                            *p.offset(i as isize)
                                .offset(-(rowbytes as isize))
                                .offset(-(bytes_per_pixel as isize))
                                as libc::c_int
                        } else {
                            0i32
                        }
                    } else {
                        0i32
                    };
                    let mut q_0: libc::c_int = left_2 + up_2 - uplft_0;
                    let mut qa_0: libc::c_int = (q_0 - left_2).abs();
                    let mut qb_0: libc::c_int = (q_0 - up_2).abs();
                    let mut qc_0: libc::c_int = (q_0 - uplft_0).abs();
                    if qa_0 <= qb_0 && qa_0 <= qc_0 {
                        *pp.offset((i + 1i32) as isize) =
                            (*p.offset(i as isize) as libc::c_int - left_2) as libc::c_uchar
                    } else if qb_0 <= qc_0 {
                        *pp.offset((i + 1i32) as isize) =
                            (*p.offset(i as isize) as libc::c_int - up_2) as libc::c_uchar
                    } else {
                        *pp.offset((i + 1i32) as isize) =
                            (*p.offset(i as isize) as libc::c_int - uplft_0) as libc::c_uchar
                    }
                    i += 1
                }
            }
            _ => {}
        }
        j += 1
    }
    return dst;
}
/* TIFF predictor filter support
 *
 * Many PDF viewers seems to have broken TIFF 2 predictor support?
 * Ony GhostScript and MuPDF render 4bpc grayscale image with TIFF 2 predictor
 * filter applied correctly.
 *
 *  Acrobat Reader DC  2015.007.20033  NG
 *  Adobe Acrobat X    10.1.13         NG
 *  Foxit Reader       4.1.5.425       NG
 *  GhostScript        9.16            OK
 *  SumatraPDF(MuPDF)  v3.0            OK
 *  Evince(poppler)    2.32.0.145      NG (1bit and 4bit broken)
 */
/* This modifies "raster" itself! */
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn apply_filter_TIFF2_1_2_4(
    mut raster: *mut libc::c_uchar,
    mut width: i32,
    mut height: i32,
    mut bpc: i8,
    mut num_comp: i8,
) {
    let mut rowbytes: i32 = (bpc as libc::c_int * num_comp as libc::c_int * width + 7i32) / 8i32;
    let mut mask: u8 = ((1i32 << bpc as libc::c_int) - 1i32) as u8;
    let mut prev: *mut u16 = 0 as *mut u16;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    assert!(!raster.is_null());
    assert!(bpc as libc::c_int > 0i32 && bpc as libc::c_int <= 8i32);
    prev = new((num_comp as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<u16>() as libc::c_ulong) as u32) as *mut u16;
    /* Generic routine for 1 to 16 bit.
     * It supports, e.g., 7 bpc images too.
     * Actually, it is not necessary to have 16 bit inbuf and outbuf
     * since we only need 1, 2, and 4 bit support here. 8 bit is enough.
     */
    j = 0i32;
    while j < height {
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        let mut inbits: i32 = 0;
        let mut outbits: i32 = 0;
        let mut inbuf: u16 = 0;
        let mut outbuf: u16 = 0;
        let mut c: libc::c_int = 0;
        memset(
            prev as *mut libc::c_void,
            0i32,
            (::std::mem::size_of::<u16>() as libc::c_ulong).wrapping_mul(num_comp as libc::c_ulong),
        );
        outbuf = 0i32 as u16;
        inbuf = outbuf;
        outbits = 0i32;
        inbits = outbits;
        k = j * rowbytes;
        l = k;
        i = 0i32;
        while i < width {
            c = 0i32;
            while c < num_comp as libc::c_int {
                let mut cur: u8 = 0;
                let mut sub: i8 = 0;
                if inbits < bpc as libc::c_int {
                    /* need more byte */
                    inbuf = ((inbuf as libc::c_int) << 8i32
                        | *raster.offset(l as isize) as libc::c_int)
                        as u16; /* consumed bpc bits */
                    l += 1;
                    inbits += 8i32
                }
                cur = (inbuf as libc::c_int >> inbits - bpc as libc::c_int & mask as libc::c_int)
                    as u8;
                inbits -= bpc as libc::c_int;
                sub = (cur as libc::c_int - *prev.offset(c as isize) as libc::c_int) as i8;
                *prev.offset(c as isize) = cur as u16;
                if (sub as libc::c_int) < 0i32 {
                    sub = (sub as libc::c_int + (1i32 << bpc as libc::c_int)) as i8
                }
                /* Append newly filtered component value */
                outbuf =
                    ((outbuf as libc::c_int) << bpc as libc::c_int | sub as libc::c_int) as u16;
                outbits += bpc as libc::c_int;
                /* flush */
                if outbits >= 8i32 {
                    *raster.offset(k as isize) =
                        (outbuf as libc::c_int >> outbits - 8i32) as libc::c_uchar;
                    k += 1;
                    outbits -= 8i32
                }
                c += 1
            }
            i += 1
        }
        if outbits > 0i32 {
            *raster.offset(k as isize) =
                ((outbuf as libc::c_int) << 8i32 - outbits) as libc::c_uchar
        }
        j += 1
    }
    free(prev as *mut libc::c_void);
}
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn filter_TIFF2_apply_filter(
    mut raster: *mut libc::c_uchar,
    mut columns: i32,
    mut rows: i32,
    mut bpc: i8,
    mut colors: i8,
    mut length: *mut i32,
) -> *mut libc::c_uchar {
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prev: *mut u16 = 0 as *mut u16;
    let mut rowbytes: i32 = (bpc as libc::c_int * colors as libc::c_int * columns + 7i32) / 8i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    assert!(!raster.is_null() && !length.is_null());
    dst = new(((rowbytes * rows) as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as u32)
        as *mut libc::c_uchar;
    libc::memcpy(
        dst as *mut libc::c_void,
        raster as *const libc::c_void,
        (rowbytes * rows) as usize,
    );
    *length = rowbytes * rows;
    match bpc as libc::c_int {
        1 | 2 | 4 => {
            apply_filter_TIFF2_1_2_4(dst, columns, rows, bpc, colors);
        }
        8 => {
            prev = new((colors as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<u16>() as libc::c_ulong)
                as u32) as *mut u16;
            j = 0i32;
            while j < rows {
                memset(
                    prev as *mut libc::c_void,
                    0i32,
                    (::std::mem::size_of::<u16>() as libc::c_ulong)
                        .wrapping_mul(colors as libc::c_ulong),
                );
                i = 0i32;
                while i < columns {
                    let mut c: libc::c_int = 0;
                    let mut pos: i32 = colors as libc::c_int * (columns * j + i);
                    c = 0i32;
                    while c < colors as libc::c_int {
                        let mut cur: u8 = *raster.offset((pos + c) as isize);
                        let mut sub: i32 =
                            cur as libc::c_int - *prev.offset(c as isize) as libc::c_int;
                        *prev.offset(c as isize) = cur as u16;
                        *dst.offset((pos + c) as isize) = sub as libc::c_uchar;
                        c += 1
                    }
                    i += 1
                }
                j += 1
            }
            free(prev as *mut libc::c_void);
        }
        16 => {
            prev = new((colors as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<u16>() as libc::c_ulong)
                as u32) as *mut u16;
            j = 0i32;
            while j < rows {
                memset(
                    prev as *mut libc::c_void,
                    0i32,
                    (::std::mem::size_of::<u16>() as libc::c_ulong)
                        .wrapping_mul(colors as libc::c_ulong),
                );
                i = 0i32;
                while i < columns {
                    let mut c_0: libc::c_int = 0;
                    let mut pos_0: i32 = 2i32 * colors as libc::c_int * (columns * j + i);
                    c_0 = 0i32;
                    while c_0 < colors as libc::c_int {
                        let mut cur_0: u16 =
                            (*raster.offset((pos_0 + 2i32 * c_0) as isize) as libc::c_int * 256i32
                                + *raster.offset((pos_0 + 2i32 * c_0 + 1i32) as isize)
                                    as libc::c_int) as u16;
                        let mut sub_0: u16 = (cur_0 as libc::c_int
                            - *prev.offset(c_0 as isize) as libc::c_int)
                            as u16;
                        *prev.offset(c_0 as isize) = cur_0;
                        *dst.offset((pos_0 + 2i32 * c_0) as isize) =
                            (sub_0 as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
                        *dst.offset((pos_0 + 2i32 * c_0 + 1i32) as isize) =
                            (sub_0 as libc::c_int & 0xffi32) as libc::c_uchar;
                        c_0 += 1
                    }
                    i += 1
                }
                j += 1
            }
            free(prev as *mut libc::c_void);
        }
        _ => {}
    }
    return dst;
}
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn filter_create_predictor_dict(
    mut predictor: libc::c_int,
    mut columns: i32,
    mut bpc: libc::c_int,
    mut colors: libc::c_int,
) -> *mut pdf_obj {
    let mut parms: *mut pdf_obj = 0 as *mut pdf_obj;
    parms = pdf_new_dict();
    pdf_add_dict(
        parms,
        pdf_new_name(b"BitsPerComponent\x00" as *const u8 as *const i8),
        pdf_new_number(bpc as f64),
    );
    pdf_add_dict(
        parms,
        pdf_new_name(b"Colors\x00" as *const u8 as *const i8),
        pdf_new_number(colors as f64),
    );
    pdf_add_dict(
        parms,
        pdf_new_name(b"Columns\x00" as *const u8 as *const i8),
        pdf_new_number(columns as f64),
    );
    pdf_add_dict(
        parms,
        pdf_new_name(b"Predictor\x00" as *const u8 as *const i8),
        pdf_new_number(predictor as f64),
    );
    return parms;
}
unsafe extern "C" fn write_stream(mut stream: *mut pdf_stream, mut handle: rust_output_handle_t) {
    let mut filtered: *mut u8 = 0 as *mut u8;
    let mut filtered_length: u32 = 0;
    /*
     * Always work from a copy of the stream. All filters read from
     * "filtered" and leave their result in "filtered".
     */
    filtered = new(
        ((*stream).stream_length as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
    ) as *mut u8;
    libc::memcpy(
        filtered as *mut libc::c_void,
        (*stream).stream as *const libc::c_void,
        (*stream).stream_length as usize,
    );
    filtered_length = (*stream).stream_length;
    /* PDF/A requires Metadata to be not filtered. */
    let mut typ: *mut pdf_obj = 0 as *mut pdf_obj;
    typ = pdf_lookup_dict((*stream).dict, b"Type\x00" as *const u8 as *const i8);
    if !typ.is_null()
        && streq_ptr(
            b"Metadata\x00" as *const u8 as *const i8,
            pdf_name_value(typ),
        ) as i32
            != 0
    {
        (*stream)._flags &= !(1i32 << 0i32)
    }
    /* Apply compression filter if requested */
    #[cfg(feature = "libz-sys")]
    {
        if (*stream).stream_length > 0i32 as libc::c_uint
            && (*stream)._flags & 1i32 << 0i32 != 0
            && compression_level as libc::c_int > 0i32
        {
            let mut filters: *mut pdf_obj = 0 as *mut pdf_obj;
            /* First apply predictor filter if requested. */
            if compression_use_predictor as libc::c_int != 0
                && (*stream)._flags & 1i32 << 1i32 != 0
                && pdf_lookup_dict((*stream).dict, b"DecodeParms\x00" as *const u8 as *const i8)
                    .is_null()
            {
                let mut bits_per_pixel: libc::c_int =
                    (*stream).decodeparms.colors * (*stream).decodeparms.bits_per_component;
                let mut len: i32 = ((*stream).decodeparms.columns * bits_per_pixel + 7i32) / 8i32;
                let mut rows: i32 =
                    (*stream).stream_length.wrapping_div(len as libc::c_uint) as i32;
                let mut filtered2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut length2: i32 = (*stream).stream_length as i32;
                let mut parms: *mut pdf_obj = 0 as *mut pdf_obj;
                parms = filter_create_predictor_dict(
                    (*stream).decodeparms.predictor,
                    (*stream).decodeparms.columns,
                    (*stream).decodeparms.bits_per_component,
                    (*stream).decodeparms.colors,
                );
                match (*stream).decodeparms.predictor {
                    2 => {
                        /* TIFF2 */
                        filtered2 = filter_TIFF2_apply_filter(
                            filtered,
                            (*stream).decodeparms.columns,
                            rows,
                            (*stream).decodeparms.bits_per_component as i8,
                            (*stream).decodeparms.colors as i8,
                            &mut length2,
                        )
                    }
                    15 => {
                        /* PNG optimun */
                        filtered2 = filter_PNG15_apply_filter(
                            filtered,
                            (*stream).decodeparms.columns,
                            rows,
                            (*stream).decodeparms.bits_per_component as i8,
                            (*stream).decodeparms.colors as i8,
                            &mut length2,
                        )
                    }
                    _ => {
                        warn!(
                            "Unknown/unsupported Predictor function {}.",
                            (*stream).decodeparms.predictor
                        );
                    }
                }
                if !parms.is_null() && !filtered2.is_null() {
                    free(filtered as *mut libc::c_void);
                    filtered = filtered2;
                    filtered_length = length2 as libc::c_uint;
                    pdf_add_dict(
                        (*stream).dict,
                        pdf_new_name(b"DecodeParms\x00" as *const u8 as *const i8),
                        parms,
                    );
                }
            }
            filters = pdf_lookup_dict((*stream).dict, b"Filter\x00" as *const u8 as *const i8);
            let mut buffer_length: libz::uLong;
            let mut buffer: *mut u8 = 0 as *mut u8;
            buffer_length = filtered_length
                .wrapping_add(filtered_length.wrapping_div(1000i32 as libc::c_uint))
                .wrapping_add(14i32 as libc::c_uint) as libz::uLong;
            buffer = new((buffer_length as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as u32) as *mut libc::c_uchar;
            let mut filter_name: *mut pdf_obj =
                pdf_new_name(b"FlateDecode\x00" as *const u8 as *const i8);
            if !filters.is_null() {
                /*
                 * FlateDecode is the first filter to be applied to the stream.
                 */
                pdf_unshift_array(filters, filter_name);
            } else {
                /*
                 * Adding the filter as a name instead of a one-element array
                 * is crucial because otherwise Adobe Reader cannot read the
                 * cross-reference stream any more, cf. the PDF v1.5 Errata.
                 */
                pdf_add_dict(
                    (*stream).dict,
                    pdf_new_name(b"Filter\x00" as *const u8 as *const i8),
                    filter_name,
                );
            }

            #[cfg(not(feature = "legacy-libz"))]
            {
                if libz::compress2(
                    buffer,
                    &mut buffer_length,
                    filtered,
                    filtered_length as libz::uLong,
                    compression_level as libc::c_int,
                ) != 0
                {
                    panic!("Zlib error");
                }
            }
            #[cfg(feature = "legacy-libz")]
            {
                if libz::compress(
                    buffer,
                    &mut buffer_length,
                    filtered,
                    filtered_length as libz::uLong,
                ) != 0
                {
                    panic!("Zlib error");
                }
            }
            free(filtered as *mut libc::c_void);
            compression_saved = (compression_saved as libc::c_ulong).wrapping_add(
                (filtered_length as libc::c_ulong)
                    .wrapping_sub(buffer_length)
                    .wrapping_sub(if !filters.is_null() {
                        strlen(b"/FlateDecode \x00" as *const u8 as *const i8)
                    } else {
                        strlen(b"/Filter/FlateDecode\n\x00" as *const u8 as *const i8)
                    } as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
            filtered = buffer;
            filtered_length = buffer_length as libc::c_uint
        }
    }
    /* HAVE_ZLIB */
    /* AES will change the size of data! */
    if enc_mode {
        let mut cipher: *mut u8 = 0 as *mut u8;
        let mut cipher_len: size_t = 0i32 as size_t;
        pdf_encrypt_data(
            filtered,
            filtered_length as size_t,
            &mut cipher,
            &mut cipher_len,
        );
        free(filtered as *mut libc::c_void);
        filtered = cipher;
        filtered_length = cipher_len as u32
    }
    pdf_add_dict(
        (*stream).dict,
        pdf_new_name(b"Length\x00" as *const u8 as *const i8),
        pdf_new_number(filtered_length as f64),
    );
    pdf_write_obj((*stream).dict, handle);
    pdf_out(
        handle,
        b"\nstream\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        8i32,
    );
    if filtered_length > 0_u32 {
        pdf_out(
            handle,
            filtered as *const libc::c_void,
            filtered_length as i32,
        );
    }
    free(filtered as *mut libc::c_void);
    /*
     * This stream length "object" gets reset every time write_stream is
     * called for the stream object.
     * If this stream gets written more than once with different
     * filters, this could be a problem.
     */
    pdf_out(
        handle,
        b"\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        1i32,
    );
    pdf_out(
        handle,
        b"endstream\x00" as *const u8 as *const i8 as *const libc::c_void,
        9i32,
    );
}
unsafe extern "C" fn release_stream(mut stream: *mut pdf_stream) {
    pdf_release_obj((*stream).dict);
    (*stream).dict = 0 as *mut pdf_obj;
    (*stream).stream = mfree((*stream).stream as *mut libc::c_void) as *mut u8;
    (*stream).objstm_data = mfree((*stream).objstm_data as *mut libc::c_void) as *mut i32;
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_dict(mut stream: *mut pdf_obj) -> *mut pdf_obj {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || PdfObjType::from((*stream).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !stream.is_null() {
                (*stream).typ
            } else {
                -1i32
            },
            7i32,
            1961i32,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    (*data).dict
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_dataptr(mut stream: *mut pdf_obj) -> *const libc::c_void {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || PdfObjType::from((*stream).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !stream.is_null() {
                (*stream).typ
            } else {
                -1i32
            },
            7i32,
            1973i32,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    (*data).stream as *const libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_length(mut stream: *mut pdf_obj) -> i32 {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || PdfObjType::from((*stream).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !stream.is_null() {
                (*stream).typ
            } else {
                -1i32
            },
            7i32,
            1985i32,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    (*data).stream_length as i32
}
unsafe extern "C" fn set_objstm_data(mut objstm: *mut pdf_obj, mut data: *mut i32) {
    if objstm.is_null() || PdfObjType::from((*objstm).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !objstm.is_null() {
                (*objstm).typ
            } else {
                -1i32
            },
            7i32,
            1994i32,
        );
    }
    let ref mut fresh14 = (*((*objstm).data as *mut pdf_stream)).objstm_data;
    *fresh14 = data;
}
unsafe extern "C" fn get_objstm_data(objstm: &pdf_obj) -> *mut i32 {
    if PdfObjType::from(objstm.typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            objstm.typ, 7i32, 2001i32,
        );
    }
    (*(objstm.data as *mut pdf_stream)).objstm_data
}
#[no_mangle]
pub unsafe extern "C" fn pdf_add_stream(
    mut stream: *mut pdf_obj,
    mut stream_data: *const libc::c_void,
    mut length: i32,
) {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || PdfObjType::from((*stream).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !stream.is_null() {
                (*stream).typ
            } else {
                -1i32
            },
            7i32,
            2011i32,
        );
    }
    if length < 1i32 {
        return;
    }
    data = (*stream).data as *mut pdf_stream;
    if (*data).stream_length.wrapping_add(length as u32) > (*data).max_length {
        (*data).max_length = (*data)
            .max_length
            .wrapping_add((length as u32).wrapping_add(4096u32));
        (*data).stream = renew(
            (*data).stream as *mut libc::c_void,
            ((*data).max_length as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
        ) as *mut u8
    }
    libc::memcpy(
        (*data).stream.offset((*data).stream_length as isize) as *mut libc::c_void,
        stream_data,
        length as usize,
    );
    (*data).stream_length = (*data).stream_length.wrapping_add(length as u32);
}
#[no_mangle]
#[cfg(feature = "libz-sys")]
pub unsafe extern "C" fn pdf_add_stream_flate(
    mut dst: *mut pdf_obj,
    mut data: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut z: libz::z_stream = std::mem::zeroed();
    let mut wbuf: [libz::Bytef; 4096] = [0; 4096];
    // FIXME: Bug in libpng-sys
    // z.zalloc = null_mut();
    // z.zfree = null_mut();
    z.opaque = 0 as libz::voidpf;
    z.next_in = data as *mut libz::Bytef;
    z.avail_in = len as libz::uInt;
    z.next_out = wbuf.as_mut_ptr();
    z.avail_out = 4096i32 as libz::uInt;
    if libz::inflateInit_(
        &mut z,
        b"1.2.11\x00" as *const u8 as *const i8,
        ::std::mem::size_of::<libz::z_stream>() as libc::c_ulong as libc::c_int,
    ) != 0i32
    {
        warn!("inflateInit() failed.");
        return -1i32;
    }
    loop {
        let mut status: libc::c_int = 0;
        status = libz::inflate(&mut z, 0i32);
        if status == 1i32 {
            break;
        }
        if status != 0i32 {
            warn!("inflate() failed. Broken PDF file?");
            libz::inflateEnd(&mut z);
            return -1i32;
        }
        if z.avail_out == 0i32 as libc::c_uint {
            pdf_add_stream(dst, wbuf.as_mut_ptr() as *const libc::c_void, 4096i32);
            z.next_out = wbuf.as_mut_ptr();
            z.avail_out = 4096i32 as libz::uInt
        }
    }
    if (4096i32 as libc::c_uint).wrapping_sub(z.avail_out) > 0i32 as libc::c_uint {
        pdf_add_stream(
            dst,
            wbuf.as_mut_ptr() as *const libc::c_void,
            (4096i32 as libc::c_uint).wrapping_sub(z.avail_out) as libc::c_int,
        );
    }
    return if libz::inflateEnd(&mut z) == 0i32 {
        0i32
    } else {
        -1i32
    };
}

#[cfg(feature = "libz-sys")]
unsafe extern "C" fn get_decode_parms(
    parms: &mut decode_parms,
    mut dict: *mut pdf_obj,
) -> libc::c_int {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!dict.is_null());
    assert!(!dict.is_null() && pdf_obj_typeof(dict) == PdfObjType::DICT);
    /* Fill with default values */
    parms.predictor = 1i32;
    parms.colors = 1i32;
    parms.bits_per_component = 8i32;
    parms.columns = 1i32;
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"Predictor\x00" as *const u8 as *const i8,
    ));
    if !tmp.is_null() {
        parms.predictor = pdf_number_value(tmp) as libc::c_int
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"Colors\x00" as *const u8 as *const i8,
    ));
    if !tmp.is_null() {
        parms.colors = pdf_number_value(tmp) as libc::c_int
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"BitsPerComponent\x00" as *const u8 as *const i8,
    ));
    if !tmp.is_null() {
        parms.bits_per_component = pdf_number_value(tmp) as libc::c_int
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"Columns\x00" as *const u8 as *const i8,
    ));
    if !tmp.is_null() {
        parms.columns = pdf_number_value(tmp) as i32
    }
    if parms.bits_per_component != 1i32
        && parms.bits_per_component != 2i32
        && parms.bits_per_component != 4i32
        && parms.bits_per_component != 8i32
        && parms.bits_per_component != 16i32
    {
        warn!(
            "Invalid BPC value in DecodeParms: {}",
            parms.bits_per_component,
        );
        return -1i32;
    } else {
        if parms.predictor <= 0i32 || parms.colors <= 0i32 || parms.columns <= 0i32 {
            return -1i32;
        }
    }
    return 0i32;
}
/* From Xpdf version 3.04
 * I'm not sure if I properly ported... Untested.
 */
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn filter_row_TIFF2(
    mut dst: *mut libc::c_uchar,
    mut src: *const libc::c_uchar,
    parms: &mut decode_parms,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = src;
    let mut col: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* bits_per_component < 8 here */
    let mut mask: libc::c_int = (1i32 << parms.bits_per_component) - 1i32; /* 2 bytes buffer */
    let mut inbuf: libc::c_int = 0;
    let mut outbuf: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut inbits: libc::c_int = 0;
    let mut outbits: libc::c_int = 0;
    col = new((parms.colors as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as u32)
        as *mut libc::c_uchar;
    memset(
        col as *mut libc::c_void,
        0i32,
        parms.colors as libc::c_ulong,
    );
    outbuf = 0i32;
    inbuf = outbuf;
    outbits = 0i32;
    inbits = outbits;
    k = 0i32;
    j = k;
    i = 0i32;
    while i < parms.columns {
        /* expanding each color component into an 8-bits bytes array */
        ci = 0i32;
        while ci < parms.colors {
            if inbits < parms.bits_per_component {
                /* need more byte */
                let fresh16 = j;
                j = j + 1;
                inbuf = inbuf << 8i32 | *p.offset(fresh16 as isize) as libc::c_int;
                inbits += 8i32
            }
            /* predict current color component */
            *col.offset(ci as isize) = (*col.offset(ci as isize) as libc::c_int
                + (inbuf >> inbits - parms.bits_per_component)
                & mask) as libc::c_uchar; /* consumed bpc bits */
            inbits -= parms.bits_per_component;
            /* append newly predicted color component value */
            outbuf = outbuf << parms.bits_per_component | *col.offset(ci as isize) as libc::c_int;
            outbits += parms.bits_per_component;
            if outbits >= 8i32 {
                /* flush */
                let fresh17 = k;
                k = k + 1;
                *dst.offset(fresh17 as isize) = (outbuf >> outbits - 8i32) as libc::c_uchar;
                outbits -= 8i32
            }
            ci += 1
        }
        i += 1
    }
    if outbits > 0i32 {
        *dst.offset(k as isize) = (outbuf << 8i32 - outbits) as libc::c_uchar
    }
    free(col as *mut libc::c_void);
    return 0i32;
}
/* This routine is inefficient. Length is typically 4 for Xref streams.
 * Especially, calling pdf_add_stream() for each 4 bytes append is highly
 * inefficient.
 */
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn filter_decoded(
    mut dst: *mut pdf_obj,
    mut src: *const libc::c_void,
    mut srclen: libc::c_int,
    parms: &mut decode_parms,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = src as *const libc::c_uchar; /* Just copy */
    let mut endptr: *const libc::c_uchar = p.offset(srclen as isize);
    let mut prev: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bits_per_pixel: libc::c_int = parms.colors * parms.bits_per_component;
    let mut bytes_per_pixel: libc::c_int = (bits_per_pixel + 7i32) / 8i32;
    let mut length: libc::c_int = (parms.columns * bits_per_pixel + 7i32) / 8i32;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0i32;
    prev = new((length as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as u32) as *mut libc::c_uchar;
    buf = new((length as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as u32)
        as *mut libc::c_uchar;
    memset(prev as *mut libc::c_void, 0i32, length as libc::c_ulong);
    let mut current_block_77: u64;
    match parms.predictor {
        1 => {
            /* No prediction */
            pdf_add_stream(dst, src, srclen);
            current_block_77 = 6040267449472925966;
        }
        2 => {
            /* TIFF Predictor 2 */
            if parms.bits_per_component == 8i32 {
                while p.offset(length as isize) < endptr {
                    /* Same as PNG Sub */
                    i = 0i32; /* bits per component 1, 2, 4 */
                    while i < length {
                        let mut pv: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                            *buf.offset((i - bytes_per_pixel) as isize) as libc::c_int
                        } else {
                            0i32
                        };
                        *buf.offset(i as isize) =
                            (*p.offset(i as isize) as libc::c_int + pv & 0xffi32) as libc::c_uchar;
                        i += 1
                    }
                    pdf_add_stream(dst, buf as *const libc::c_void, length);
                    p = p.offset(length as isize)
                }
            } else if parms.bits_per_component == 16i32 {
                while p.offset(length as isize) < endptr {
                    i = 0i32;
                    while i < length {
                        let mut b: libc::c_int = i - bytes_per_pixel;
                        let mut hi: i8 = (if b >= 0i32 {
                            *buf.offset(b as isize) as libc::c_int
                        } else {
                            0i32
                        }) as i8;
                        let mut lo: i8 = (if b >= 0i32 {
                            *buf.offset((b + 1i32) as isize) as libc::c_int
                        } else {
                            0i32
                        }) as i8;
                        let mut pv_0: libc::c_int = (hi as libc::c_int) << 8i32 | lo as libc::c_int;
                        let mut cv: libc::c_int = (*p.offset(i as isize) as libc::c_int) << 8i32
                            | *p.offset((i + 1i32) as isize) as libc::c_int;
                        let mut c: libc::c_int = pv_0 + cv;
                        *buf.offset(i as isize) = (c >> 8i32) as libc::c_uchar;
                        *buf.offset((i + 1i32) as isize) = (c & 0xffi32) as libc::c_uchar;
                        i += 2i32
                    }
                    pdf_add_stream(dst, buf as *const libc::c_void, length);
                    p = p.offset(length as isize)
                }
            } else {
                while error == 0 && p.offset(length as isize) < endptr {
                    error = filter_row_TIFF2(buf, p, parms);
                    if error == 0 {
                        pdf_add_stream(dst, buf as *const libc::c_void, length);
                        p = p.offset(length as isize)
                    }
                }
            }
            current_block_77 = 6040267449472925966;
        }
        10 => {
            /* PNG None */
            current_block_77 = 18089190442011260268;
        }
        11 => {
            current_block_77 = 18089190442011260268;
        }
        12 => {
            current_block_77 = 15842817987810867823;
        }
        13 => {
            current_block_77 = 6139367728676434155;
        }
        14 | 15 => {
            current_block_77 = 6912830033131235815;
        }
        _ => {
            warn!("Unknown Predictor type value :{}", parms.predictor,);
            error = -1i32;
            current_block_77 = 6040267449472925966;
        }
    }
    match current_block_77 {
        18089190442011260268 =>
        /* PNG Sub on all rows */
        {
            current_block_77 = 15842817987810867823;
        }
        _ => {}
    }
    match current_block_77 {
        15842817987810867823 =>
        /* PNG UP on all rows */
        {
            current_block_77 = 6139367728676434155;
        }
        _ => {}
    }
    match current_block_77 {
        6139367728676434155 =>
        /* PNG Average on all rows */
        {
            current_block_77 = 6912830033131235815;
        }
        _ => {}
    }
    match current_block_77 {
        6912830033131235815 =>
        /* PNG Paeth on all rows */
        /* PNG optimun: prediction algorithm can change from line to line. */
        {
            let mut typ: libc::c_int = parms.predictor - 10i32;
            while error == 0 && p.offset(length as isize) < endptr {
                if parms.predictor == 15i32 {
                    typ = *p as libc::c_int
                } else if *p as libc::c_int != typ {
                    warn!("Mismatched Predictor type in data stream.",);
                    error = -1i32
                }
                p = p.offset(1);
                match typ {
                    0 => {
                        /* Do nothing just skip first byte */
                        libc::memcpy(
                            buf as *mut libc::c_void,
                            p as *const libc::c_void,
                            length as usize,
                        ); /* left */
                    }
                    1 => {
                        i = 0i32; /* above */
                        while i < length {
                            let mut pv_1: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                                *buf.offset((i - bytes_per_pixel) as isize) as libc::c_int
                            } else {
                                0i32
                            }; /* upper left */
                            *buf.offset(i as isize) = (*p.offset(i as isize) as libc::c_int + pv_1
                                & 0xffi32)
                                as libc::c_uchar; /* highly inefficient */
                            i += 1
                        }
                    }
                    2 => {
                        i = 0i32;
                        while i < length {
                            *buf.offset(i as isize) = (*p.offset(i as isize) as libc::c_int
                                + *prev.offset(i as isize) as libc::c_int
                                & 0xffi32)
                                as libc::c_uchar;
                            i += 1
                        }
                    }
                    3 => {
                        i = 0i32;
                        while i < length {
                            let mut up: libc::c_int = *prev.offset(i as isize) as libc::c_int;
                            let mut left: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                                *buf.offset((i - bytes_per_pixel) as isize) as libc::c_int
                            } else {
                                0i32
                            };
                            let mut tmp: libc::c_int =
                                (((up + left) / 2i32) as f64).floor() as libc::c_int;
                            *buf.offset(i as isize) = (*p.offset(i as isize) as libc::c_int + tmp
                                & 0xffi32)
                                as libc::c_uchar;
                            i += 1
                        }
                    }
                    4 => {
                        i = 0i32;
                        while i < length {
                            let mut a: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                                *buf.offset((i - bytes_per_pixel) as isize) as libc::c_int
                            } else {
                                0i32
                            };
                            let mut b_0: libc::c_int = *prev.offset(i as isize) as libc::c_int;
                            let mut c_0: libc::c_int = if i - bytes_per_pixel >= 0i32 {
                                *prev.offset((i - bytes_per_pixel) as isize) as libc::c_int
                            } else {
                                0i32
                            };
                            let mut q: libc::c_int = a + b_0 - c_0;
                            let mut qa: libc::c_int = q - a;
                            let mut qb: libc::c_int = q - b_0;
                            let mut qc: libc::c_int = q - c_0;
                            qa = if qa < 0i32 { -qa } else { qa };
                            qb = if qb < 0i32 { -qb } else { qb };
                            qc = if qc < 0i32 { -qc } else { qc };
                            if qa <= qb && qa <= qc {
                                *buf.offset(i as isize) = (*p.offset(i as isize) as libc::c_int + a
                                    & 0xffi32)
                                    as libc::c_uchar
                            } else if qb <= qc {
                                *buf.offset(i as isize) =
                                    (*p.offset(i as isize) as libc::c_int + b_0 & 0xffi32)
                                        as libc::c_uchar
                            } else {
                                *buf.offset(i as isize) =
                                    (*p.offset(i as isize) as libc::c_int + c_0 & 0xffi32)
                                        as libc::c_uchar
                            }
                            i += 1
                        }
                    }
                    _ => {
                        warn!("Unknown PNG predictor type: {}", typ,);
                        error = -1i32
                    }
                }
                if error == 0 {
                    pdf_add_stream(dst, buf as *const libc::c_void, length);
                    libc::memcpy(
                        prev as *mut libc::c_void,
                        buf as *const libc::c_void,
                        length as usize,
                    );
                    p = p.offset(length as isize)
                }
            }
        }
        _ => {}
    }
    free(prev as *mut libc::c_void);
    free(buf as *mut libc::c_void);
    error
}
#[cfg(feature = "libz-sys")]
unsafe extern "C" fn pdf_add_stream_flate_filtered(
    mut dst: *mut pdf_obj,
    mut data: *const libc::c_void,
    mut len: libc::c_int,
    parms: &mut decode_parms,
) -> libc::c_int {
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut z: libz::z_stream = std::mem::zeroed();
    let mut wbuf: [libz::Bytef; 4096] = [0; 4096];
    let mut error: libc::c_int = 0;
    // FIXME: Bug in libpng-sys
    // z.zalloc = null_mut();
    // z.zfree = null_mut();
    z.opaque = 0 as libz::voidpf;
    z.next_in = data as *mut libz::Bytef;
    z.avail_in = len as libz::uInt;
    z.next_out = wbuf.as_mut_ptr();
    z.avail_out = 4096i32 as libz::uInt;
    if libz::inflateInit_(
        &mut z,
        b"1.2.11\x00" as *const u8 as *const i8,
        ::std::mem::size_of::<libz::z_stream>() as libc::c_ulong as libc::c_int,
    ) != 0i32
    {
        warn!("inflateInit() failed.");
        return -1i32;
    }
    tmp = pdf_new_stream(0i32);
    loop {
        let mut status: libc::c_int = 0;
        status = libz::inflate(&mut z, 0i32);
        if status == 1i32 {
            break;
        }
        if status != 0i32 {
            warn!("inflate() failed. Broken PDF file?");
            libz::inflateEnd(&mut z);
            return -1i32;
        }
        if z.avail_out == 0i32 as libc::c_uint {
            pdf_add_stream(tmp, wbuf.as_mut_ptr() as *const libc::c_void, 4096i32);
            z.next_out = wbuf.as_mut_ptr();
            z.avail_out = 4096i32 as libz::uInt
        }
    }
    if (4096i32 as libc::c_uint).wrapping_sub(z.avail_out) > 0i32 as libc::c_uint {
        pdf_add_stream(
            tmp,
            wbuf.as_mut_ptr() as *const libc::c_void,
            (4096i32 as libc::c_uint).wrapping_sub(z.avail_out) as libc::c_int,
        );
    }
    error = filter_decoded(dst, pdf_stream_dataptr(tmp), pdf_stream_length(tmp), parms);
    pdf_release_obj(tmp);
    if error == 0 && libz::inflateEnd(&mut z) == 0i32 {
        0i32
    } else {
        -1i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn pdf_concat_stream(mut dst: *mut pdf_obj, mut src: *mut pdf_obj) -> i32 {
    let mut stream_data: *const i8 = 0 as *const i8;
    let mut stream_length: i32 = 0;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut filter: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: i32 = 0i32;
    if !(!dst.is_null() && pdf_obj_typeof(dst) == PdfObjType::STREAM)
        || !(!src.is_null() && pdf_obj_typeof(src) == PdfObjType::STREAM)
    {
        panic!("Invalid type.");
    }
    stream_data = pdf_stream_dataptr(src) as *const i8;
    stream_length = pdf_stream_length(src);
    stream_dict = pdf_stream_dict(src);
    filter = pdf_lookup_dict(stream_dict, b"Filter\x00" as *const u8 as *const i8);
    if filter.is_null() {
        pdf_add_stream(dst, stream_data as *const libc::c_void, stream_length);
    } else {
        #[cfg(feature = "libz-sys")]
        {
            let mut parms = decode_parms {
                predictor: 0,
                colors: 0,
                bits_per_component: 0,
                columns: 0,
            };
            let mut have_parms: libc::c_int = 0i32;
            if !pdf_lookup_dict(stream_dict, b"DecodeParms\x00" as *const u8 as *const i8).is_null()
            {
                let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
                /* Dictionary or array */
                tmp = pdf_deref_obj(pdf_lookup_dict(
                    stream_dict,
                    b"DecodeParms\x00" as *const u8 as *const i8,
                ));
                if !tmp.is_null() && pdf_obj_typeof(tmp) == PdfObjType::ARRAY {
                    if pdf_array_length(tmp) > 1i32 as libc::c_uint {
                        warn!("Unexpected size for DecodeParms array.");
                        return -1i32;
                    }
                    tmp = pdf_deref_obj(pdf_get_array(tmp, 0i32))
                }
                if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PdfObjType::DICT) {
                    warn!("PDF dict expected for DecodeParms...");
                    return -1i32;
                }
                error = get_decode_parms(&mut parms, tmp);
                if error != 0 {
                    panic!("Invalid value(s) in DecodeParms dictionary.");
                }
                have_parms = 1i32
            }
            if !filter.is_null() && pdf_obj_typeof(filter) == PdfObjType::ARRAY {
                if pdf_array_length(filter) > 1i32 as libc::c_uint {
                    warn!("Multiple DecodeFilter not supported.");
                    return -1i32;
                }
                filter = pdf_get_array(filter, 0i32)
            }
            if !filter.is_null() && pdf_obj_typeof(filter) == PdfObjType::NAME {
                let filter_name = CStr::from_ptr(pdf_name_value(filter)).to_str().unwrap();
                if filter_name == "FlateDecode" {
                    if have_parms != 0 {
                        error = pdf_add_stream_flate_filtered(
                            dst,
                            stream_data as *const libc::c_void,
                            stream_length,
                            &mut parms,
                        )
                    } else {
                        error = pdf_add_stream_flate(
                            dst,
                            stream_data as *const libc::c_void,
                            stream_length,
                        )
                    }
                } else {
                    warn!("DecodeFilter \"{}\" not supported.", filter_name,);
                    error = -1i32
                }
            } else {
                panic!("Broken PDF file?");
            }
        }
    }
    /* HAVE_ZLIB */
    error
}
unsafe extern "C" fn pdf_stream_uncompress(mut src: *mut pdf_obj) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = pdf_new_stream(0i32);
    if src.is_null() || PdfObjType::from((*src).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            if !src.is_null() { (*src).typ } else { -1i32 },
            7i32,
            2420i32,
        );
    }
    pdf_merge_dict(pdf_stream_dict(dst), pdf_stream_dict(src));
    pdf_remove_dict(
        pdf_stream_dict(dst),
        b"Length\x00" as *const u8 as *const i8,
    );
    pdf_concat_stream(dst, src);
    dst
}
unsafe extern "C" fn pdf_write_obj(mut object: *mut pdf_obj, mut handle: rust_output_handle_t) {
    if object.is_null() {
        write_null(handle);
        return;
    }
    if object.is_null()
        || (*object).typ <= 0i32
        || (*object).typ > 10i32
        || !object.is_null() && pdf_obj_typeof(object) == PdfObjType::UNDEFINED
    {
        panic!("pdf_write_obj: Invalid object, type = {}\n", (*object).typ);
    }
    match (*object).typ {
        1 => {
            write_boolean((*object).data as *mut pdf_boolean, handle);
        }
        2 => {
            write_number((*object).data as *mut pdf_number, handle);
        }
        3 => {
            write_string((*object).data as *mut pdf_string, handle);
        }
        4 => {
            write_name((*object).data as *mut pdf_name, handle);
        }
        5 => {
            write_array((*object).data as *mut pdf_array, handle);
        }
        6 => {
            write_dict((*object).data as *mut pdf_dict, handle);
        }
        7 => {
            write_stream((*object).data as *mut pdf_stream, handle);
        }
        8 => {
            write_null(handle);
        }
        9 => {
            write_indirect((*object).data as *mut pdf_indirect, handle);
        }
        _ => {}
    };
}
/* Write the object to the file */
unsafe extern "C" fn pdf_flush_obj(mut object: *mut pdf_obj, mut handle: rust_output_handle_t) {
    let mut length: i32 = 0;
    /*
     * Record file position
     */
    add_xref_entry(
        (*object).label,
        1_u8,
        pdf_output_file_position as u32,
        (*object).generation,
    );
    length = sprintf(
        format_buffer.as_mut_ptr(),
        b"%u %hu obj\n\x00" as *const u8 as *const i8,
        (*object).label,
        (*object).generation as i32,
    );
    enc_mode = doc_enc_mode as i32 != 0 && (*object).flags & 1i32 << 1i32 == 0;
    pdf_enc_set_label((*object).label);
    pdf_enc_set_generation((*object).generation as u32);
    pdf_out(
        handle,
        format_buffer.as_mut_ptr() as *const libc::c_void,
        length,
    );
    pdf_write_obj(object, handle);
    pdf_out(
        handle,
        b"\nendobj\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        8i32,
    );
}
unsafe extern "C" fn pdf_add_objstm(objstm: *mut pdf_obj, mut object: *mut pdf_obj) -> i32 {
    let mut data: *mut i32 = 0 as *mut i32;
    let mut pos: i32 = 0;
    if PdfObjType::from((*objstm).typ) != PdfObjType::STREAM {
        panic!(
            "typecheck: Invalid object type: {} {} (line {})",
            (*objstm).typ,
            7i32,
            2497i32,
        );
    }
    data = get_objstm_data(&*objstm);
    let ref mut fresh15 = *data.offset(0);
    *fresh15 += 1;
    pos = *fresh15;
    *data.offset((2i32 * pos) as isize) = (*object).label as i32;
    *data.offset((2i32 * pos + 1i32) as isize) = pdf_stream_length(objstm);
    add_xref_entry((*object).label, 2_u8, (*objstm).label, (pos - 1i32) as u16);
    /* redirect output into objstm */
    output_stream = objstm;
    enc_mode = false;
    pdf_write_obj(object, pdf_output_handle);
    pdf_out_char(pdf_output_handle, '\n' as i32 as i8);
    output_stream = 0 as *mut pdf_obj;
    pos
}
unsafe extern "C" fn release_objstm(objstm: *mut pdf_obj) {
    let mut data: *mut i32 = get_objstm_data(&*objstm);
    let mut pos: i32 = *data.offset(0);
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream: *mut pdf_stream = 0 as *mut pdf_stream;
    let mut old_buf: *mut u8 = 0 as *mut u8;
    let mut old_length: u32 = 0;
    stream = (*objstm).data as *mut pdf_stream;
    /* Precede stream data by offset table */
    old_buf = (*stream).stream;
    old_length = (*stream).stream_length;
    /* Reserve 22 bytes for each entry (two 10 digit numbers plus two spaces) */
    (*stream).stream = new((old_length.wrapping_add((22i32 * pos) as u32) as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
    (*stream).stream_length = 0_u32;
    let mut i: i32 = 2i32 * pos;
    let mut val: *mut i32 = data.offset(2);
    loop {
        let fresh16 = i;
        i = i - 1;
        if !(fresh16 != 0) {
            break;
        }
        let fresh17 = val;
        val = val.offset(1);
        let mut length: i32 = sprintf(
            format_buffer.as_mut_ptr(),
            b"%d \x00" as *const u8 as *const i8,
            *fresh17,
        );
        pdf_add_stream(
            objstm,
            format_buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
    }
    dict = pdf_stream_dict(objstm);
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"ObjStm\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"N\x00" as *const u8 as *const i8),
        pdf_new_number(pos as f64),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"First\x00" as *const u8 as *const i8),
        pdf_new_number((*stream).stream_length as f64),
    );
    pdf_add_stream(objstm, old_buf as *const libc::c_void, old_length as i32);
    free(old_buf as *mut libc::c_void);
    pdf_release_obj(objstm);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_release_obj(mut object: *mut pdf_obj) {
    if object.is_null() {
        return;
    }
    if object.is_null()
        || (*object).typ <= 0i32
        || (*object).typ > 10i32
        || (*object).refcount <= 0_u32
    {
        dpx_message(
            b"\npdf_release_obj: object=%p, type=%d, refcount=%d\n\x00" as *const u8 as *const i8,
            object,
            (*object).typ,
            (*object).refcount,
        );
        pdf_write_obj(object, ttstub_output_open_stdout());
        panic!("pdf_release_obj:  Called with invalid object.");
    }
    (*object).refcount = (*object).refcount.wrapping_sub(1_u32);
    if (*object).refcount == 0_u32 {
        /*
         * Nothing is using this object so it's okay to remove it.
         * Nonzero "label" means object needs to be written before it's destroyed.
         */
        if (*object).label != 0 && !pdf_output_handle.is_null() {
            if do_objstm == 0
                || (*object).flags & 1i32 << 0i32 != 0
                || doc_enc_mode as i32 != 0 && (*object).flags & 1i32 << 1i32 != 0
                || (*object).generation as i32 != 0
            {
                pdf_flush_obj(object, pdf_output_handle);
            } else {
                if current_objstm.is_null() {
                    let mut data: *mut i32 = new(((2i32 * 200i32 + 2i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<i32>() as u64)
                        as u32) as *mut i32;
                    let ref mut fresh18 = *data.offset(1);
                    *fresh18 = 0i32;
                    *data.offset(0) = *fresh18;
                    current_objstm = pdf_new_stream(1i32 << 0i32);
                    set_objstm_data(current_objstm, data);
                    pdf_label_obj(current_objstm);
                }
                if pdf_add_objstm(current_objstm, object) == 200i32 {
                    release_objstm(current_objstm);
                    current_objstm = 0 as *mut pdf_obj
                }
            }
        }
        match PdfObjType::from((*object).typ) {
            PdfObjType::BOOLEAN => {
                release_boolean((*object).data as *mut pdf_obj);
            }
            PdfObjType::NUMBER => {
                release_number((*object).data as *mut pdf_number);
            }
            PdfObjType::STRING => {
                release_string((*object).data as *mut pdf_string);
            }
            PdfObjType::NAME => {
                release_name((*object).data as *mut pdf_name);
            }
            PdfObjType::ARRAY => {
                release_array((*object).data as *mut pdf_array);
            }
            PdfObjType::DICT => {
                release_dict((*object).data as *mut pdf_dict);
            }
            PdfObjType::STREAM => {
                release_stream((*object).data as *mut pdf_stream);
            }
            PdfObjType::INDIRECT => {
                release_indirect((*object).data as *mut pdf_indirect);
            }
            PdfObjType::NULL | _ => {}
        }
        /* This might help detect freeing already freed objects */
        (*object).typ = -1i32;
        (*object).data = 0 as *mut libc::c_void;
        free(object as *mut libc::c_void);
    };
}
/* PDF reading starts around here */
/* As each lines may contain null-characters, so outptr here is NOT
 * null-terminated string. Returns -1 for when EOF is already reached, and -2
 * if buffer has no enough space.
 */
unsafe extern "C" fn tt_mfreadln(
    mut buf: *mut i8,
    mut size: i32,
    mut handle: rust_input_handle_t,
) -> i32 {
    let mut c: i32 = 0;
    let mut len: i32 = 0i32;
    loop {
        c = ttstub_input_getc(handle);
        if !(c != -1i32 && c != '\n' as i32 && c != '\r' as i32) {
            break;
        }
        if len >= size {
            return -2i32;
        }
        let fresh19 = len;
        len = len + 1;
        *buf.offset(fresh19 as isize) = c as i8
    }
    if c == -1i32 && len == 0i32 {
        return -1i32;
    }
    if c == '\r' as i32
        && {
            c = ttstub_input_getc(handle);
            c >= 0i32
        }
        && c != '\n' as i32
    {
        ttstub_input_ungetc(handle, c);
    }
    len
}
unsafe extern "C" fn backup_line(mut handle: rust_input_handle_t) -> i32 {
    let mut ch: i32 = -1i32;
    /* Note: this code should work even if \r\n is eol. It could fail on a
     * machine where \n is eol and there is a \r in the stream --- Highly
     * unlikely in the last few bytes where this is likely to be used.
     */
    if ttstub_input_seek(handle, 0i32 as ssize_t, 1i32) > 1i32 as u64 {
        loop {
            ttstub_input_seek(handle, -2i32 as ssize_t, 1i32);
            if !(ttstub_input_seek(handle, 0i32 as ssize_t, 1i32) > 0i32 as u64
                && {
                    ch = ttstub_input_getc(handle);
                    ch >= 0i32
                }
                && (ch != '\n' as i32 && ch != '\r' as i32))
            {
                break;
            }
        }
    }
    if ch < 0i32 {
        return 0i32;
    }
    1i32
}
unsafe extern "C" fn find_xref(mut handle: rust_input_handle_t, mut file_size: i32) -> i32 {
    let mut xref_pos: i32 = 0i32;
    let mut len: i32 = 0;
    let mut tries: i32 = 10i32;
    let mut start: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut number: *mut i8 = 0 as *mut i8;
    loop {
        let mut currentpos: i32 = 0;
        let mut n: i32 = 0;
        if backup_line(handle) == 0 {
            tries = 0i32;
            break;
        } else {
            currentpos = ttstub_input_seek(handle, 0i32 as ssize_t, 1i32) as i32;
            n = (if strlen(b"startxref\x00" as *const u8 as *const i8)
                < (file_size - currentpos) as u64
            {
                strlen(b"startxref\x00" as *const u8 as *const i8)
            } else {
                (file_size - currentpos) as u64
            }) as i32;
            ttstub_input_read(handle, work_buffer.as_mut_ptr(), n as size_t);
            ttstub_input_seek(handle, currentpos as ssize_t, 0i32);
            tries -= 1;
            if !(tries > 0i32
                && strstartswith(
                    work_buffer.as_mut_ptr(),
                    b"startxref\x00" as *const u8 as *const i8,
                )
                .is_null())
            {
                break;
            }
        }
    }
    if tries <= 0i32 {
        return 0i32;
    }
    /* Skip rest of this line */
    tt_mfgets(work_buffer.as_mut_ptr(), 1024i32, handle);
    /* Next line of input file should contain actual xref location */
    len = tt_mfreadln(work_buffer.as_mut_ptr(), 1024i32, handle);
    if len <= 0i32 {
        warn!("Reading xref location data failed... Not a PDF file?");
        return 0i32;
    }
    start = work_buffer.as_mut_ptr();
    end = start.offset(len as isize);
    skip_white(&mut start, end);
    number = parse_number(&mut start, end);
    xref_pos = atof(number) as i32;
    free(number as *mut libc::c_void);
    xref_pos
}
/*
 * This routine must be called with the file pointer located
 * at the start of the trailer.
 */
unsafe extern "C" fn parse_trailer(mut pf: *mut pdf_file) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cur_pos: i32 = 0;
    let mut nmax: i32 = 0;
    let mut nread: i32 = 0;
    /*
     * Fill work_buffer and hope trailer fits. This should
     * be made a bit more robust sometime.
     */
    cur_pos = ttstub_input_seek((*pf).handle, 0i32 as ssize_t, 1i32) as i32;
    nmax = if (*pf).file_size - cur_pos < 1024i32 {
        (*pf).file_size - cur_pos
    } else {
        1024i32
    };
    nread = ttstub_input_read((*pf).handle, work_buffer.as_mut_ptr(), nmax as size_t) as i32;
    if nread == 0i32
        || strstartswith(
            work_buffer.as_mut_ptr(),
            b"trailer\x00" as *const u8 as *const i8,
        )
        .is_null()
    {
        warn!("No trailer.  Are you sure this is a PDF file?");
        dpx_warning(
            b"buffer:\n->%s<-\n\x00" as *const u8 as *const i8,
            work_buffer.as_mut_ptr(),
        );
        result = 0 as *mut pdf_obj
    } else {
        let mut p: *const i8 = work_buffer
            .as_mut_ptr()
            .offset(strlen(b"trailer\x00" as *const u8 as *const i8) as isize);
        skip_white(&mut p, work_buffer.as_mut_ptr().offset(nread as isize));
        result = parse_pdf_dict(&mut p, work_buffer.as_mut_ptr().offset(nread as isize), pf)
    }
    result
}
/*
 * This routine tries to estimate an upper bound for character position
 * of the end of the object, so it knows how big the buffer must be.
 * The parsing routines require that the entire object be read into
 * memory. It would be a major pain to rewrite them.  The worst case
 * is that an object before an xref table will grab the whole table
 * :-(
 */
unsafe extern "C" fn next_object_offset(mut pf: *mut pdf_file, mut obj_num: u32) -> i32 {
    let mut next: i32 = (*pf).file_size; /* Worst case */
    let mut i: i32 = 0;
    let mut curr: i32 = 0;
    curr = (*(*pf).xref_table.offset(obj_num as isize)).field2 as i32;
    /* Check all other type 1 objects to find next one */
    i = 0i32;
    while i < (*pf).num_obj {
        if (*(*pf).xref_table.offset(i as isize)).typ as i32 == 1i32
            && (*(*pf).xref_table.offset(i as isize)).field2 > curr as u32
            && (*(*pf).xref_table.offset(i as isize)).field2 < next as u32
        {
            next = (*(*pf).xref_table.offset(i as isize)).field2 as i32
        }
        i += 1
    }
    next
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_indirect(
    mut pf: *mut pdf_file,
    mut obj_num: u32,
    mut obj_gen: u16,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut indirect: *mut pdf_indirect = 0 as *mut pdf_indirect;
    indirect = new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_indirect>() as u64) as u32)
        as *mut pdf_indirect;
    (*indirect).pf = pf;
    (*indirect).obj = 0 as *mut pdf_obj;
    (*indirect).label = obj_num;
    (*indirect).generation = obj_gen;
    result = pdf_new_obj(9i32);
    (*result).data = indirect as *mut libc::c_void;
    result
}
unsafe extern "C" fn pdf_read_object(
    mut obj_num: u32,
    mut obj_gen: u16,
    mut pf: *mut pdf_file,
    mut offset: i32,
    mut limit: i32,
) -> *mut pdf_obj {
    let mut length: i32 = 0;
    let mut buffer: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    length = limit - offset;
    if length <= 0i32 {
        return 0 as *mut pdf_obj;
    }
    buffer = new(
        ((length + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    ttstub_input_seek((*pf).handle, offset as ssize_t, 0i32);
    ttstub_input_read((*pf).handle, buffer, length as size_t);
    p = buffer;
    endptr = p.offset(length as isize);
    /* Check for obj_num and obj_gen */
    let mut q: *const i8 = p; /* <== p */
    let mut sp: *mut i8 = 0 as *mut i8;
    let mut n: u32 = 0;
    let mut g: u32 = 0;
    skip_white(&mut q, endptr);
    sp = parse_unsigned(&mut q, endptr);
    if sp.is_null() {
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    n = strtoul(sp, 0 as *mut *mut i8, 10i32) as u32;
    free(sp as *mut libc::c_void);
    skip_white(&mut q, endptr);
    sp = parse_unsigned(&mut q, endptr);
    if sp.is_null() {
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    g = strtoul(sp, 0 as *mut *mut i8, 10i32) as u32;
    free(sp as *mut libc::c_void);
    if obj_num != 0 && (n != obj_num || g != obj_gen as u32) {
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    p = q;
    skip_white(&mut p, endptr);
    if memcmp(
        p as *const libc::c_void,
        b"obj\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"obj\x00" as *const u8 as *const i8),
    ) != 0
    {
        warn!("Didn\'t find \"obj\".");
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    p = p.offset(strlen(b"obj\x00" as *const u8 as *const i8) as isize);
    result = parse_pdf_object(&mut p, endptr, pf);
    skip_white(&mut p, endptr);
    if memcmp(
        p as *const libc::c_void,
        b"endobj\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"endobj\x00" as *const u8 as *const i8),
    ) != 0
    {
        warn!("Didn\'t find \"endobj\".");
        pdf_release_obj(result);
        result = 0 as *mut pdf_obj
    }
    free(buffer as *mut libc::c_void);
    result
}
unsafe extern "C" fn read_objstm(mut pf: *mut pdf_file, mut num: u32) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut offset: u32 = (*(*pf).xref_table.offset(num as isize)).field2;
    let mut gen: u16 = (*(*pf).xref_table.offset(num as isize)).field3;
    let mut limit: i32 = next_object_offset(pf, num);
    let mut n: i32 = 0;
    let mut first: i32 = 0;
    let mut header: *mut i32 = 0 as *mut i32;
    let mut data: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    let mut objstm: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut typ: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut n_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut first_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    objstm = pdf_read_object(num, gen, pf, offset as i32, limit);
    if !objstm.is_null() && pdf_obj_typeof(objstm) == PdfObjType::STREAM {
        let mut tmp: *mut pdf_obj = pdf_stream_uncompress(objstm);
        if !tmp.is_null() {
            pdf_release_obj(objstm);
            objstm = tmp;
            dict = pdf_stream_dict(objstm);
            typ = pdf_lookup_dict(dict, b"Type\x00" as *const u8 as *const i8);
            if !(!(!typ.is_null() && pdf_obj_typeof(typ) == PdfObjType::NAME)
                || strcmp(pdf_name_value(typ), b"ObjStm\x00" as *const u8 as *const i8) != 0)
            {
                n_obj = pdf_lookup_dict(dict, b"N\x00" as *const u8 as *const i8);
                if !n_obj.is_null() && pdf_obj_typeof(n_obj) == PdfObjType::NUMBER {
                    n = pdf_number_value(n_obj) as i32;
                    first_obj = pdf_lookup_dict(dict, b"First\x00" as *const u8 as *const i8);
                    if !first_obj.is_null() && pdf_obj_typeof(first_obj) == PdfObjType::NUMBER {
                        first = pdf_number_value(first_obj) as i32;
                        /* reject object streams without object data */
                        if !(first >= pdf_stream_length(objstm)) {
                            header = new(((2i32 * (n + 1i32)) as u32 as u64)
                                .wrapping_mul(::std::mem::size_of::<i32>() as u64)
                                as u32) as *mut i32;
                            set_objstm_data(objstm, header);
                            let fresh20 = header;
                            header = header.offset(1);
                            *fresh20 = n;
                            let fresh21 = header;
                            header = header.offset(1);
                            *fresh21 = first;
                            /* avoid parsing beyond offset table */
                            data = new(((first + 1i32) as u32 as u64)
                                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                                as u32) as *mut i8;
                            libc::memcpy(
                                data as *mut libc::c_void,
                                pdf_stream_dataptr(objstm),
                                first as usize,
                            );
                            *data.offset(first as isize) = 0_i8;
                            p = data;
                            endptr = p.offset(first as isize);
                            i = 2i32 * n;
                            loop {
                                let fresh22 = i;
                                i = i - 1;
                                if !(fresh22 != 0) {
                                    current_block = 3275366147856559585;
                                    break;
                                }
                                let fresh23 = header;
                                header = header.offset(1);
                                *fresh23 = strtoul(p, &mut q, 10i32) as i32;
                                if q == p as *mut i8 {
                                    current_block = 13429587009686472387;
                                    break;
                                }
                                p = q
                            }
                            match current_block {
                                13429587009686472387 => {}
                                _ => {
                                    /* Any garbage after last entry? */
                                    skip_white(&mut p, endptr);
                                    if !(p != endptr) {
                                        free(data as *mut libc::c_void);
                                        let ref mut fresh24 =
                                            (*(*pf).xref_table.offset(num as isize)).direct;
                                        *fresh24 = objstm;
                                        return *fresh24;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    warn!("Cannot parse object stream.");
    free(data as *mut libc::c_void);
    pdf_release_obj(objstm);
    0 as *mut pdf_obj
}
/* Label without corresponding object definition are replaced by the
 * null object, as required by the PDF spec. This is important to parse
 * several cross-reference sections.
 */
unsafe extern "C" fn pdf_get_object(
    mut pf: *mut pdf_file,
    mut obj_num: u32,
    mut obj_gen: u16,
) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    if !(obj_num > 0_u32
        && obj_num < (*pf).num_obj as u32
        && ((*(*pf).xref_table.offset(obj_num as isize)).typ as i32 == 1i32
            && (*(*pf).xref_table.offset(obj_num as isize)).field3 as i32 == obj_gen as i32
            || (*(*pf).xref_table.offset(obj_num as isize)).typ as i32 == 2i32 && obj_gen == 0))
    {
        dpx_warning(
            b"Trying to read nonexistent or deleted object: %u %hu\x00" as *const u8 as *const i8,
            obj_num,
            obj_gen as i32,
        );
        return pdf_new_null();
    }
    result = (*(*pf).xref_table.offset(obj_num as isize)).direct;
    if !result.is_null() {
        return pdf_link_obj(result);
    }
    if (*(*pf).xref_table.offset(obj_num as isize)).typ as i32 == 1i32 {
        /* type == 1 */
        let mut offset: u32 = 0;
        let mut limit: i32 = 0;
        offset = (*(*pf).xref_table.offset(obj_num as isize)).field2;
        limit = next_object_offset(pf, obj_num);
        result = pdf_read_object(obj_num, obj_gen, pf, offset as i32, limit)
    } else {
        /* type == 2 */
        let mut objstm_num: u32 = (*(*pf).xref_table.offset(obj_num as isize)).field2;
        let mut index: u16 = (*(*pf).xref_table.offset(obj_num as isize)).field3;
        let mut objstm: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut data: *mut i32 = 0 as *mut i32;
        let mut n: i32 = 0;
        let mut first: i32 = 0;
        let mut length: i32 = 0;
        let mut p: *const i8 = 0 as *const i8;
        let mut q: *const i8 = 0 as *const i8;
        if objstm_num >= (*pf).num_obj as u32
            || (*(*pf).xref_table.offset(objstm_num as isize)).typ as i32 != 1i32
            || {
                objstm = (*(*pf).xref_table.offset(objstm_num as isize)).direct;
                !(!objstm.is_null() || {
                    objstm = read_objstm(pf, objstm_num);
                    !objstm.is_null()
                })
            }
        {
            current_block = 17536737673648832705;
        } else {
            data = get_objstm_data(&*objstm);
            let fresh25 = data;
            data = data.offset(1);
            n = *fresh25;
            let fresh26 = data;
            data = data.offset(1);
            first = *fresh26;
            if index as i32 >= n || *data.offset((2i32 * index as i32) as isize) as u32 != obj_num {
                current_block = 17536737673648832705;
            } else {
                length = pdf_stream_length(objstm);
                p = (pdf_stream_dataptr(objstm) as *const i8)
                    .offset(first as isize)
                    .offset(*data.offset((2i32 * index as i32 + 1i32) as isize) as isize);
                q = p.offset(
                    (if index as i32 == n - 1i32 {
                        length
                    } else {
                        first + *data.offset((2i32 * index as i32 + 3i32) as isize)
                    }) as isize,
                );
                result = parse_pdf_object(&mut p, q, pf);
                if result.is_null() {
                    current_block = 17536737673648832705;
                } else {
                    current_block = 13472856163611868459;
                }
            }
        }
        match current_block {
            13472856163611868459 => {}
            _ => {
                warn!("Could not read object from object stream.");
                return pdf_new_null();
            }
        }
    }
    /* Make sure the caller doesn't free this object */
    let ref mut fresh27 = (*(*pf).xref_table.offset(obj_num as isize)).direct;
    *fresh27 = pdf_link_obj(result);
    result
}
unsafe extern "C" fn pdf_new_ref(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    if (*object).label == 0_u32 {
        pdf_label_obj(object);
    }
    result = pdf_new_indirect(0 as *mut pdf_file, (*object).label, (*object).generation);
    let ref mut fresh28 = (*((*result).data as *mut pdf_indirect)).obj;
    *fresh28 = object;
    result
}
/* pdf_deref_obj always returns a link instead of the original   */
/* It never return the null object, but the NULL pointer instead */
#[no_mangle]
pub unsafe extern "C" fn pdf_deref_obj(mut obj: *mut pdf_obj) -> *mut pdf_obj {
    let mut count: i32 = 30i32;
    if !obj.is_null() {
        obj = pdf_link_obj(obj)
    }
    while !obj.is_null() && pdf_obj_typeof(obj) == PdfObjType::INDIRECT && {
        count -= 1;
        count != 0
    } {
        let mut pf: *mut pdf_file = (*((*obj).data as *mut pdf_indirect)).pf;
        if !pf.is_null() {
            let mut obj_num: u32 = (*((*obj).data as *mut pdf_indirect)).label;
            let mut obj_gen: u16 = (*((*obj).data as *mut pdf_indirect)).generation;
            pdf_release_obj(obj);
            obj = pdf_get_object(pf, obj_num, obj_gen)
        } else {
            let mut next_obj: *mut pdf_obj = (*((*obj).data as *mut pdf_indirect)).obj;
            if next_obj.is_null() {
                panic!("Undefined object reference");
            }
            pdf_release_obj(obj);
            obj = pdf_link_obj(next_obj)
        }
    }
    if count == 0 {
        panic!("Loop in object hierarchy detected. Broken PDF file?");
    }
    if !obj.is_null() && pdf_obj_typeof(obj) == PdfObjType::NULL {
        pdf_release_obj(obj);
        return 0 as *mut pdf_obj;
    } else {
        return obj;
    };
}
unsafe extern "C" fn extend_xref(mut pf: *mut pdf_file, mut new_size: i32) {
    let mut i: u32 = 0;
    (*pf).xref_table = renew(
        (*pf).xref_table as *mut libc::c_void,
        (new_size as u32 as u64).wrapping_mul(::std::mem::size_of::<xref_entry>() as u64) as u32,
    ) as *mut xref_entry;
    i = (*pf).num_obj as u32;
    while i < new_size as u32 {
        let ref mut fresh29 = (*(*pf).xref_table.offset(i as isize)).direct;
        *fresh29 = 0 as *mut pdf_obj;
        let ref mut fresh30 = (*(*pf).xref_table.offset(i as isize)).indirect;
        *fresh30 = 0 as *mut pdf_obj;
        (*(*pf).xref_table.offset(i as isize)).typ = 0_u8;
        (*(*pf).xref_table.offset(i as isize)).field3 = 0_u16;
        (*(*pf).xref_table.offset(i as isize)).field2 = 0i64 as u32;
        i = i.wrapping_add(1)
    }
    (*pf).num_obj = new_size;
}
/* Returns < 0 for error, 1 for success, and 0 when xref stream found. */
unsafe extern "C" fn parse_xref_table(mut pf: *mut pdf_file, mut xref_pos: i32) -> i32 {
    let mut p: *const i8 = 0 as *const i8; /* See, PDF ref. v.1.7, p.91 for "255+1" here. */
    let mut endptr: *const i8 = 0 as *const i8;
    let mut buf: [i8; 256] = [0; 256];
    let mut len: i32 = 0;
    /*
     * This routine reads one xref segment. It may be called multiple times
     * on the same file.  xref tables sometimes come in pieces.
     */
    ttstub_input_seek((*pf).handle, xref_pos as ssize_t, 0i32);
    len = tt_mfreadln(buf.as_mut_ptr(), 255i32, (*pf).handle);
    /* We should have already checked that "startxref" section exists. So, EOF
     * here (len = -1) is impossible. We don't treat too long line case
     * seriously.
     */
    if len < 0i32 {
        warn!("Something went wrong while reading xref table...giving up.");
        return -1i32;
    }
    p = buf.as_mut_ptr();
    endptr = buf.as_mut_ptr().offset(len as isize);
    /* No skip_white() here. There should not be any white-spaces here. */
    if memcmp(
        p as *const libc::c_void,
        b"xref\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"xref\x00" as *const u8 as *const i8),
    ) != 0
    {
        /* Might be an xref stream and not an xref table */
        return 0i32;
    }
    p = p.offset(strlen(b"xref\x00" as *const u8 as *const i8) as isize);
    skip_white(&mut p, endptr);
    if p != endptr {
        warn!("Garbage after \"xref\" keyword found.");
        return -1i32;
    }
    loop
    /* Next line in file has first item and size of table */
    {
        let mut flag: i8 = 0;
        let mut current_pos: u32 = 0;
        let mut i: i32 = 0;
        let mut first: u32 = 0;
        let mut size: u32 = 0;
        let mut offset: u32 = 0;
        let mut obj_gen: u32 = 0;
        current_pos = ttstub_input_seek((*pf).handle, 0i32 as ssize_t, 1i32) as u32;
        len = tt_mfreadln(buf.as_mut_ptr(), 255i32, (*pf).handle);
        if !(len == 0i32) {
            if len < 0i32 {
                warn!("Reading a line failed in xref table.");
                return -1i32;
            }
            p = buf.as_mut_ptr();
            endptr = buf.as_mut_ptr().offset(len as isize);
            skip_white(&mut p, endptr);
            if !(p == endptr) {
                if !strstartswith(p, b"trailer\x00" as *const u8 as *const i8).is_null() {
                    /* Backup... This is ugly, but it seems like the safest thing to
                     * do. It is possible the trailer dictionary starts on the same
                     * logical line as the word trailer. In that case, the mfgets call
                     * might have started to read the trailer dictionary and
                     * parse_trailer would fail.
                     */
                    current_pos = (current_pos as i64
                        + p.wrapping_offset_from(buf.as_mut_ptr()) as i64)
                        as u32; /* Jump to the beginning of "trailer" keyword. */
                    ttstub_input_seek((*pf).handle, current_pos as ssize_t, 0i32);
                    break;
                } else {
                    /* Line containing something other than white-space characters found.
                     *
                     * Start reading xref subsection
                     *
                     * This section just reads two nusigned integers, namely, the object number
                     * of first object and the size of the xref subsection. PDF reference says
                     * that only "a space" is allowed between those two numbers but we allow
                     * more white-space characters.
                     */
                    let mut q: *mut i8 = 0 as *mut i8;
                    /* Object number of the first object whithin this xref subsection. */
                    q = parse_unsigned(&mut p, endptr);
                    if q.is_null() {
                        warn!("An unsigned integer expected but could not find. (xref)");
                        return -1i32;
                    }
                    first = atoi(q) as u32;
                    free(q as *mut libc::c_void);
                    skip_white(&mut p, endptr);
                    /* Nnumber of objects in this xref subsection. */
                    q = parse_unsigned(&mut p, endptr);
                    if q.is_null() {
                        warn!("An unsigned integer expected but could not find. (xref)");
                        return -1i32;
                    }
                    size = atoi(q) as u32;
                    free(q as *mut libc::c_void);
                    skip_white(&mut p, endptr);
                    /* Check for unrecognized tokens */
                    if p != endptr {
                        warn!("Unexpected token found in xref table.");
                        return -1i32;
                    }
                    /* The first line of a xref subsection OK. */
                    if ((*pf).num_obj as u32) < first.wrapping_add(size) {
                        extend_xref(pf, first.wrapping_add(size) as i32);
                    }
                    /* Start parsing xref subsection body... */
                    i = first as i32;
                    /* Only white-spaces and/or comment. */
                    while (i as u32) < first.wrapping_add(size) {
                        /* PDF spec. requires each xref subsection lines being exactly 20 bytes
                         * long [including end-of-line marker(s)], offset 10 decimal digits,
                         * generation number being 5 decimal digits, and each entries delimitted
                         * by "a single space". However, we don't srtictly follow this rule:
                         * More than one "white-spaces" allowed, can be ended with a comment,
                         * and so on.
                         */
                        len = tt_mfreadln(buf.as_mut_ptr(), 255i32, (*pf).handle);
                        if !(len == 0i32) {
                            if len < 0i32 {
                                warn!("Something went wrong while reading xref subsection...");
                                return -1i32;
                            }
                            p = buf.as_mut_ptr();
                            endptr = buf.as_mut_ptr().offset(len as isize);
                            skip_white(&mut p, endptr);
                            if p == endptr {
                                continue;
                            }
                            /*
                             * Don't overwrite positions that have already been set by a
                             * modified xref table.  We are working our way backwards
                             * through the reference table, so we only set "position"
                             * if it hasn't been set yet.
                             */
                            offset = 0u64 as u32;
                            obj_gen = 0_u32;
                            flag = 0_i8;
                            let mut q_0: *mut i8 = 0 as *mut i8;
                            /* Offset value -- 10 digits (0 padded) */
                            q_0 = parse_unsigned(&mut p, endptr);
                            if q_0.is_null() {
                                warn!("An unsigned integer expected but could not find. (xref)");
                                return -1i32;
                            } else {
                                if strlen(q_0) != 10i32 as u64 {
                                    /* exactly 10 digits */
                                    warn!("Offset must be a 10 digits number. (xref)");
                                    free(q_0 as *mut libc::c_void);
                                    return -1i32;
                                }
                            }
                            /* FIXME: Possible overflow here. Consider using strtoll(). */
                            offset = atoi(q_0) as u32;
                            free(q_0 as *mut libc::c_void);
                            skip_white(&mut p, endptr);
                            /* Generation number -- 5 digits (0 padded) */
                            q_0 = parse_unsigned(&mut p, endptr);
                            if q_0.is_null() {
                                warn!("An unsigned integer expected but could not find. (xref)");
                                return -1i32;
                            } else {
                                if strlen(q_0) != 5i32 as u64 {
                                    /* exactly 5 digits */
                                    warn!("Expecting a 5 digits number. (xref)");
                                    free(q_0 as *mut libc::c_void);
                                    return -1i32;
                                }
                            }
                            obj_gen = atoi(q_0) as u32;
                            free(q_0 as *mut libc::c_void);
                            skip_white(&mut p, endptr);
                            if p == endptr {
                                warn!(
                                    "Unexpected EOL reached while reading a xref subsection entry."
                                );
                                return -1i32;
                            }
                            /* Flag -- a char */
                            flag = *p;
                            p = p.offset(1);
                            skip_white(&mut p, endptr);
                            if p < endptr {
                                warn!("Garbage in xref subsection entry found...");
                                return -1i32;
                            } else {
                                if flag as i32 != 'n' as i32 && flag as i32 != 'f' as i32
                                    || flag as i32 == 'n' as i32
                                        && (offset >= (*pf).file_size as u32
                                            || offset > 0_u32 && offset < 4_u32)
                                {
                                    warn!(
                                        "Invalid xref table entry [{}]. PDF file is corrupt...",
                                        i,
                                    );
                                    return -1i32;
                                }
                            }
                            /* Everything seems to be OK. */
                            if (*(*pf).xref_table.offset(i as isize)).field2 == 0 {
                                (*(*pf).xref_table.offset(i as isize)).typ =
                                    (flag as i32 == 'n' as i32) as i32 as u8; /* TODO: change! why? */
                                (*(*pf).xref_table.offset(i as isize)).field2 = offset;
                                (*(*pf).xref_table.offset(i as isize)).field3 = obj_gen as u16
                            }
                            i += 1
                        }
                    }
                }
            }
        }
    }
    1i32
}
unsafe extern "C" fn parse_xrefstm_field(
    mut p: *mut *const i8,
    mut length: i32,
    mut def: u32,
) -> u32 {
    let mut val: u32 = 0_u32;
    if length == 0 {
        return def;
    }
    loop {
        let fresh31 = length;
        length = length - 1;
        if !(fresh31 != 0) {
            break;
        }
        val <<= 8i32;
        let fresh32 = *p;
        *p = (*p).offset(1);
        val |= *fresh32 as u8 as u32
    }
    val
}
unsafe extern "C" fn parse_xrefstm_subsec(
    mut pf: *mut pdf_file,
    mut p: *mut *const i8,
    mut length: *mut i32,
    mut W: *mut i32,
    mut wsum: i32,
    mut first: i32,
    mut size: i32,
) -> i32 {
    *length -= wsum * size;
    if *length < 0i32 {
        return -1i32;
    }
    if (*pf).num_obj < first + size {
        extend_xref(pf, first + size);
    }
    let mut e: *mut xref_entry = 0 as *mut xref_entry;
    loop {
        let fresh33 = size;
        size = size - 1;
        if !(fresh33 != 0) {
            break;
        }
        let mut typ: u8 = 0;
        let mut field2: u32 = 0;
        let mut field3: u16 = 0;
        typ = parse_xrefstm_field(p, *W.offset(0), 1_u32) as u8;
        if typ as i32 > 2i32 {
            warn!("Unknown cross-reference stream entry type.");
        }
        field2 = parse_xrefstm_field(p, *W.offset(1), 0_u32);
        field3 = parse_xrefstm_field(p, *W.offset(2), 0_u32) as u16;
        if (*e).field2 == 0 {
            (*e).typ = typ;
            (*e).field2 = field2;
            (*e).field3 = field3
        }
        e = e.offset(1)
    }
    0i32
}
unsafe extern "C" fn parse_xref_stream(
    mut pf: *mut pdf_file,
    mut xref_pos: i32,
    mut trailer: *mut *mut pdf_obj,
) -> i32 {
    let mut current_block: u64;
    let mut xrefstm: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut size_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut W_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut index_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut size: u32 = 0;
    let mut length: i32 = 0;
    let mut W: [i32; 3] = [0; 3];
    let mut i: i32 = 0;
    let mut wsum: i32 = 0i32;
    let mut p: *const i8 = 0 as *const i8;
    xrefstm = pdf_read_object(0_u32, 0_u16, pf, xref_pos, (*pf).file_size);
    if !xrefstm.is_null() && pdf_obj_typeof(xrefstm) == PdfObjType::STREAM {
        let mut tmp: *mut pdf_obj = pdf_stream_uncompress(xrefstm);
        if !tmp.is_null() {
            pdf_release_obj(xrefstm);
            xrefstm = tmp;
            *trailer = pdf_link_obj(pdf_stream_dict(xrefstm));
            size_obj = pdf_lookup_dict(*trailer, b"Size\x00" as *const u8 as *const i8);
            if !size_obj.is_null() && pdf_obj_typeof(size_obj) == PdfObjType::NUMBER {
                size = pdf_number_value(size_obj) as u32;
                length = pdf_stream_length(xrefstm);
                W_obj = pdf_lookup_dict(*trailer, b"W\x00" as *const u8 as *const i8);
                if !(!(!W_obj.is_null() && pdf_obj_typeof(W_obj) == PdfObjType::ARRAY)
                    || pdf_array_length(W_obj) != 3_u32)
                {
                    i = 0i32;
                    loop {
                        if !(i < 3i32) {
                            current_block = 12147880666119273379;
                            break;
                        }
                        let mut tmp_0: *mut pdf_obj = pdf_get_array(W_obj, i);
                        if !(!tmp_0.is_null() && pdf_obj_typeof(tmp_0) == PdfObjType::NUMBER) {
                            current_block = 5131529843719913080;
                            break;
                        }
                        W[i as usize] = pdf_number_value(tmp_0) as i32;
                        wsum += W[i as usize];
                        i += 1
                    }
                    match current_block {
                        5131529843719913080 => {}
                        _ => {
                            p = pdf_stream_dataptr(xrefstm) as *const i8;
                            index_obj =
                                pdf_lookup_dict(*trailer, b"Index\x00" as *const u8 as *const i8);
                            if !index_obj.is_null() {
                                let mut index_len: u32 = 0;
                                if !(!index_obj.is_null()
                                    && pdf_obj_typeof(index_obj) == PdfObjType::ARRAY)
                                    || {
                                        index_len = pdf_array_length(index_obj);
                                        index_len.wrapping_rem(2_u32) != 0
                                    }
                                {
                                    current_block = 5131529843719913080;
                                } else {
                                    i = 0i32;
                                    loop {
                                        if !((i as u32) < index_len) {
                                            current_block = 652864300344834934;
                                            break;
                                        }
                                        let fresh34 = i;
                                        i = i + 1;
                                        let mut first: *mut pdf_obj =
                                            pdf_get_array(index_obj, fresh34);
                                        let fresh35 = i;
                                        i = i + 1;
                                        size_obj = pdf_get_array(index_obj, fresh35);
                                        if !(!first.is_null()
                                            && pdf_obj_typeof(first) == PdfObjType::NUMBER)
                                            || !(!size_obj.is_null()
                                                && pdf_obj_typeof(size_obj) == PdfObjType::NUMBER)
                                            || parse_xrefstm_subsec(
                                                pf,
                                                &mut p,
                                                &mut length,
                                                W.as_mut_ptr(),
                                                wsum,
                                                pdf_number_value(first) as i32,
                                                pdf_number_value(size_obj) as i32,
                                            ) != 0
                                        {
                                            current_block = 5131529843719913080;
                                            break;
                                        }
                                    }
                                }
                            } else if parse_xrefstm_subsec(
                                pf,
                                &mut p,
                                &mut length,
                                W.as_mut_ptr(),
                                wsum,
                                0i32,
                                size as i32,
                            ) != 0
                            {
                                current_block = 5131529843719913080;
                            } else {
                                current_block = 652864300344834934;
                            }
                            match current_block {
                                5131529843719913080 => {}
                                _ => {
                                    if length != 0 {
                                        warn!("Garbage in xref stream.");
                                    }
                                    pdf_release_obj(xrefstm);
                                    return 1i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    warn!("Cannot parse cross-reference stream.");
    pdf_release_obj(xrefstm);
    if !(*trailer).is_null() {
        pdf_release_obj(*trailer);
        *trailer = 0 as *mut pdf_obj
    }
    0i32
}
/* TODO: parse Version entry */
unsafe extern "C" fn read_xref(mut pf: *mut pdf_file) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut trailer: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut main_trailer: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut xref_pos: i32 = 0;
    xref_pos = find_xref((*pf).handle, (*pf).file_size);
    if xref_pos == 0 {
        current_block = 13794981049891343809;
    } else {
        current_block = 14916268686031723178;
    }
    loop {
        match current_block {
            14916268686031723178 => {
                if xref_pos != 0 {
                    let mut prev: *mut pdf_obj = 0 as *mut pdf_obj;
                    let mut res: i32 = parse_xref_table(pf, xref_pos);
                    if res > 0i32 {
                        /* cross-reference table */
                        let mut xrefstm: *mut pdf_obj = 0 as *mut pdf_obj;
                        trailer = parse_trailer(pf);
                        if trailer.is_null() {
                            current_block = 13794981049891343809;
                            continue;
                        }
                        if main_trailer.is_null() {
                            main_trailer = pdf_link_obj(trailer)
                        }
                        xrefstm =
                            pdf_lookup_dict(trailer, b"XRefStm\x00" as *const u8 as *const i8);
                        if !xrefstm.is_null() {
                            let mut new_trailer: *mut pdf_obj = 0 as *mut pdf_obj;
                            if !xrefstm.is_null()
                                && pdf_obj_typeof(xrefstm) == PdfObjType::NUMBER
                                && parse_xref_stream(
                                    pf,
                                    pdf_number_value(xrefstm) as i32,
                                    &mut new_trailer,
                                ) != 0
                            {
                                pdf_release_obj(new_trailer);
                            } else {
                                warn!("Skipping hybrid reference section.");
                            }
                            /* Many PDF 1.5 xref streams use DecodeParms, which we cannot
                               parse. This way we can use at least xref tables in hybrid
                               documents. Or should we better stop parsing the file?
                            */
                        }
                    } else {
                        if !(res == 0 && parse_xref_stream(pf, xref_pos, &mut trailer) != 0) {
                            current_block = 13794981049891343809;
                            continue;
                        }
                        /* cross-reference stream */
                        if main_trailer.is_null() {
                            main_trailer = pdf_link_obj(trailer)
                        }
                    }
                    prev = pdf_lookup_dict(trailer, b"Prev\x00" as *const u8 as *const i8);
                    if !prev.is_null() {
                        if !(!prev.is_null() && pdf_obj_typeof(prev) == PdfObjType::NUMBER) {
                            current_block = 13794981049891343809;
                            continue;
                        }
                        xref_pos = pdf_number_value(prev) as i32
                    } else {
                        xref_pos = 0i32
                    }
                    pdf_release_obj(trailer);
                    current_block = 14916268686031723178;
                } else {
                    return main_trailer;
                }
            }
            _ => {
                warn!("Error while parsing PDF file.");
                pdf_release_obj(trailer);
                pdf_release_obj(main_trailer);
                return 0 as *mut pdf_obj;
            }
        }
    }
}
static mut pdf_files: *mut ht_table = 0 as *const ht_table as *mut ht_table;
unsafe extern "C" fn pdf_file_new(mut handle: rust_input_handle_t) -> *mut pdf_file {
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    assert!(!handle.is_null());
    pf =
        new((1_u64).wrapping_mul(::std::mem::size_of::<pdf_file>() as u64) as u32) as *mut pdf_file;
    (*pf).handle = handle;
    (*pf).trailer = 0 as *mut pdf_obj;
    (*pf).xref_table = 0 as *mut xref_entry;
    (*pf).catalog = 0 as *mut pdf_obj;
    (*pf).num_obj = 0i32;
    (*pf).version = 0_u32;
    (*pf).file_size = ttstub_input_get_size(handle) as i32;
    ttstub_input_seek(handle, 0i32 as ssize_t, 2i32);
    pf
}
unsafe extern "C" fn pdf_file_free(mut pf: *mut pdf_file) {
    let mut i: u32 = 0;
    if pf.is_null() {
        return;
    }
    i = 0_u32;
    while i < (*pf).num_obj as u32 {
        pdf_release_obj((*(*pf).xref_table.offset(i as isize)).direct);
        pdf_release_obj((*(*pf).xref_table.offset(i as isize)).indirect);
        i = i.wrapping_add(1)
    }
    free((*pf).xref_table as *mut libc::c_void);
    pdf_release_obj((*pf).trailer);
    pdf_release_obj((*pf).catalog);
    free(pf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_files_init() {
    pdf_files =
        new((1_u64).wrapping_mul(::std::mem::size_of::<ht_table>() as u64) as u32) as *mut ht_table;
    ht_init_table(
        pdf_files,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut pdf_file) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            pdf_file_free as unsafe extern "C" fn(_: *mut pdf_file) -> (),
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_file_get_version(mut pf: *mut pdf_file) -> u32 {
    assert!(!pf.is_null());
    (*pf).version
}
#[no_mangle]
pub unsafe extern "C" fn pdf_file_get_trailer(mut pf: *mut pdf_file) -> *mut pdf_obj {
    assert!(!pf.is_null());
    pdf_link_obj((*pf).trailer)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_file_get_catalog(mut pf: *mut pdf_file) -> *mut pdf_obj {
    assert!(!pf.is_null());
    (*pf).catalog
}
#[no_mangle]
pub unsafe extern "C" fn pdf_open(
    mut ident: *const i8,
    mut handle: rust_input_handle_t,
) -> *mut pdf_file {
    let mut current_block: u64;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    assert!(!pdf_files.is_null());
    if !ident.is_null() {
        pf = ht_lookup_table(
            pdf_files,
            ident as *const libc::c_void,
            strlen(ident) as i32,
        ) as *mut pdf_file
    }
    if !pf.is_null() {
        (*pf).handle = handle
    } else {
        let mut new_version: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut version: u32 = 0_u32;
        let mut r: i32 = parse_pdf_version(handle, &mut version);
        if r < 0i32 || version < 1_u32 || version > pdf_version {
            warn!("pdf_open: Not a PDF 1.[1-{}] file.", pdf_version,);
            /*
              Try to embed the PDF image, even if the PDF version is newer than
              the setting.
              return NULL;
            */
        }
        pf = pdf_file_new(handle);
        (*pf).version = version;
        (*pf).trailer = read_xref(pf);
        if (*pf).trailer.is_null() {
            current_block = 14455231216035570027;
        } else if !pdf_lookup_dict((*pf).trailer, b"Encrypt\x00" as *const u8 as *const i8)
            .is_null()
        {
            warn!("PDF document is encrypted.");
            current_block = 14455231216035570027;
        } else {
            (*pf).catalog = pdf_deref_obj(pdf_lookup_dict(
                (*pf).trailer,
                b"Root\x00" as *const u8 as *const i8,
            ));
            if !(!(*pf).catalog.is_null() && pdf_obj_typeof((*pf).catalog) == PdfObjType::DICT) {
                warn!("Cannot read PDF document catalog. Broken PDF file?");
                current_block = 14455231216035570027;
            } else {
                new_version = pdf_deref_obj(pdf_lookup_dict(
                    (*pf).catalog,
                    b"Version\x00" as *const u8 as *const i8,
                ));
                if !new_version.is_null() {
                    let mut minor: u32 = 0;
                    if !(!new_version.is_null() && pdf_obj_typeof(new_version) == PdfObjType::NAME)
                        || sscanf(
                            pdf_name_value(new_version),
                            b"1.%u\x00" as *const u8 as *const i8,
                            &mut minor as *mut u32,
                        ) != 1i32
                    {
                        pdf_release_obj(new_version);
                        warn!("Illegal Version entry in document catalog. Broken PDF file?");
                        current_block = 14455231216035570027;
                    } else {
                        if (*pf).version < minor {
                            (*pf).version = minor
                        }
                        pdf_release_obj(new_version);
                        current_block = 15345278821338558188;
                    }
                } else {
                    current_block = 15345278821338558188;
                }
                match current_block {
                    14455231216035570027 => {}
                    _ => {
                        if !ident.is_null() {
                            ht_append_table(
                                pdf_files,
                                ident as *const libc::c_void,
                                strlen(ident) as i32,
                                pf as *mut libc::c_void,
                            );
                        }
                        current_block = 8693738493027456495;
                    }
                }
            }
        }
        match current_block {
            8693738493027456495 => {}
            _ => {
                pdf_file_free(pf);
                return 0 as *mut pdf_file;
            }
        }
    }
    pf
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close(mut pf: *mut pdf_file) {
    if !pf.is_null() {
        (*pf).handle = 0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_files_close() {
    assert!(!pdf_files.is_null());
    ht_clear_table(pdf_files);
    free(pdf_files as *mut libc::c_void);
}
/* Internal static routines */
unsafe extern "C" fn parse_pdf_version(
    mut handle: rust_input_handle_t,
    mut ret_version: *mut u32,
) -> i32 {
    let mut buffer: [i8; 10] = *::std::mem::transmute::<&[u8; 10], &mut [i8; 10]>(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
    );
    let mut minor: u32 = 0;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(
        handle,
        buffer.as_mut_ptr(),
        (::std::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1i32 as u64),
    ) as u64
        != (::std::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1i32 as u64)
    {
        return -1i32;
    }
    if sscanf(
        buffer.as_mut_ptr(),
        b"%%PDF-1.%u\x00" as *const u8 as *const i8,
        &mut minor as *mut u32,
    ) != 1i32
    {
        return -1i32;
    }
    *ret_version = minor;
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn check_for_pdf(mut handle: rust_input_handle_t) -> i32 {
    let mut r: i32 = 0;
    let mut version: u32 = 0;
    r = parse_pdf_version(handle, &mut version);
    if r < 0i32 {
        /* not a PDF file */
        return 0i32;
    }
    if version <= pdf_version {
        return 1i32;
    }
    warn!(
        "Version of PDF file (1.{}) is newer than version limit specification.",
        version
    );
    1i32
}
#[inline]
unsafe extern "C" fn import_dict(
    mut key: *mut pdf_obj,
    mut value: *mut pdf_obj,
    mut pdata: *mut libc::c_void,
) -> i32 {
    let mut copy: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    copy = pdata as *mut pdf_obj;
    tmp = pdf_import_object(value);
    if tmp.is_null() {
        return -1i32;
    }
    pdf_add_dict(copy, pdf_link_obj(key), tmp);
    0i32
}
static mut loop_marker: pdf_obj = {
    let mut init = pdf_obj {
        typ: 0i32,
        label: 0_u32,
        generation: 0_u16,
        refcount: 0_u32,
        flags: 0i32,
        data: 0 as *const libc::c_void as *mut libc::c_void,
    };
    init
};
unsafe extern "C" fn pdf_import_indirect(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut pf: *mut pdf_file = (*((*object).data as *mut pdf_indirect)).pf;
    let mut obj_num: u32 = (*((*object).data as *mut pdf_indirect)).label;
    let mut obj_gen: u16 = (*((*object).data as *mut pdf_indirect)).generation;
    let mut ref_0: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!pf.is_null());
    if !(obj_num > 0_u32
        && obj_num < (*pf).num_obj as u32
        && ((*(*pf).xref_table.offset(obj_num as isize)).typ as i32 == 1i32
            && (*(*pf).xref_table.offset(obj_num as isize)).field3 as i32 == obj_gen as i32
            || (*(*pf).xref_table.offset(obj_num as isize)).typ as i32 == 2i32 && obj_gen == 0))
    {
        warn!("Can\'t resolve object: {} {}", obj_num, obj_gen as i32,);
        return pdf_new_null();
    }
    ref_0 = (*(*pf).xref_table.offset(obj_num as isize)).indirect;
    if !ref_0.is_null() {
        if ref_0 == &mut loop_marker as *mut pdf_obj {
            panic!("Loop in object hierarchy detected. Broken PDF file?");
        }
        return pdf_link_obj(ref_0);
    } else {
        let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
        obj = pdf_get_object(pf, obj_num, obj_gen);
        if obj.is_null() {
            warn!("Could not read object: {} {}", obj_num, obj_gen as i32,);
            return 0 as *mut pdf_obj;
        }
        /* We mark the reference to be able to detect loops */
        let ref mut fresh36 = (*(*pf).xref_table.offset(obj_num as isize)).indirect;
        *fresh36 = &mut loop_marker;
        tmp = pdf_import_object(obj);
        ref_0 = pdf_ref_obj(tmp);
        let ref mut fresh37 = (*(*pf).xref_table.offset(obj_num as isize)).indirect;
        *fresh37 = ref_0;
        pdf_release_obj(tmp);
        pdf_release_obj(obj);
        return pdf_link_obj(ref_0);
    };
}
/*
 * pdf_import_object recursively copies the object and those
 * referenced by it and changes the indirect references so that
 * they refer to the current output file. New indirect references
 * are remembered, which avoids duplicating objects when they
 * are imported several times.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_import_object(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut imported: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: u32 = 0;
    match pdf_obj_typeof(object) {
        PdfObjType::INDIRECT => {
            if !(*((*object).data as *mut pdf_indirect)).pf.is_null() {
                imported = pdf_import_indirect(object)
            } else {
                imported = pdf_link_obj(object)
            }
        }
        PdfObjType::STREAM => {
            let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
            tmp = pdf_import_object(pdf_stream_dict(object));
            if tmp.is_null() {
                return 0 as *mut pdf_obj;
            }
            imported = pdf_new_stream(0i32);
            stream_dict = pdf_stream_dict(imported);
            pdf_merge_dict(stream_dict, tmp);
            pdf_release_obj(tmp);
            pdf_add_stream(
                imported,
                pdf_stream_dataptr(object),
                pdf_stream_length(object),
            );
        }
        PdfObjType::DICT => {
            imported = pdf_new_dict();
            if pdf_foreach_dict(
                object,
                Some(
                    import_dict
                        as unsafe extern "C" fn(
                            _: *mut pdf_obj,
                            _: *mut pdf_obj,
                            _: *mut libc::c_void,
                        ) -> i32,
                ),
                imported as *mut libc::c_void,
            ) < 0i32
            {
                pdf_release_obj(imported);
                return 0 as *mut pdf_obj;
            }
        }
        PdfObjType::ARRAY => {
            imported = pdf_new_array();
            i = 0_u32;
            while i < pdf_array_length(object) {
                tmp = pdf_import_object(pdf_get_array(object, i as i32));
                if tmp.is_null() {
                    pdf_release_obj(imported);
                    return 0 as *mut pdf_obj;
                }
                pdf_add_array(imported, tmp);
                i = i.wrapping_add(1)
            }
        }
        _ => imported = pdf_link_obj(object),
    }
    imported
}
/* returns 0 if indirect references point to the same object */
#[no_mangle]
pub unsafe extern "C" fn pdf_compare_reference(
    mut ref1: *mut pdf_obj,
    mut ref2: *mut pdf_obj,
) -> i32 {
    let mut data1: *mut pdf_indirect = 0 as *mut pdf_indirect;
    let mut data2: *mut pdf_indirect = 0 as *mut pdf_indirect;
    assert!(
        !ref1.is_null()
            && pdf_obj_typeof(ref1) == PdfObjType::INDIRECT
            && (!ref2.is_null() && pdf_obj_typeof(ref2) == PdfObjType::INDIRECT)
    );
    data1 = (*ref1).data as *mut pdf_indirect;
    data2 = (*ref2).data as *mut pdf_indirect;
    return ((*data1).pf != (*data2).pf
        || (*data1).label != (*data2).label
        || (*data1).generation as i32 != (*data2).generation as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_reset_global_state() {
    pdf_output_handle = 0 as *mut libc::c_void;
    pdf_output_file_position = 0i32;
    pdf_output_line_position = 0i32;
    compression_saved = 0i32;
}
