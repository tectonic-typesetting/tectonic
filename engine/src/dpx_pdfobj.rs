#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_output_open(path: *const libc::c_char, is_gz: libc::c_int) -> rust_output_handle_t;
    #[no_mangle]
    fn ttstub_output_open_stdout() -> rust_output_handle_t;
    #[no_mangle]
    fn ttstub_output_putc(handle: rust_output_handle_t, c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ttstub_output_write(
        handle: rust_output_handle_t,
        data: *const libc::c_char,
        len: size_t,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_output_close(handle: rust_output_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_ungetc(handle: rust_input_handle_t, ch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[no_mangle]
    fn ht_clear_table(ht: *mut ht_table);
    #[no_mangle]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: libc::c_int,
        value: *mut libc::c_void,
    );
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dpx_message(fmt: *const libc::c_char, _: ...);
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
    #[no_mangle]
    static mut work_buffer: [libc::c_char; 0];
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(
        buffer: *mut libc::c_char,
        length: libc::c_int,
        file: rust_input_handle_t,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_encrypt_data(
        plain: *const libc::c_uchar,
        plain_len: size_t,
        cipher: *mut *mut libc::c_uchar,
        cipher_len: *mut size_t,
    );
    #[no_mangle]
    fn pdf_enc_set_label(label: libc::c_uint);
    #[no_mangle]
    fn pdf_enc_set_generation(generation: libc::c_uint);
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
    #[no_mangle]
    fn parse_number(start: *mut *const libc::c_char, end: *const libc::c_char)
        -> *mut libc::c_char;
    #[no_mangle]
    fn parse_unsigned(
        start: *mut *const libc::c_char,
        end: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn parse_pdf_dict(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn parse_pdf_object(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_sprint_number(buf: *mut libc::c_char, value: libc::c_double) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_output_handle_t = *mut libc::c_void;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut libc::c_char,
    pub keylen: libc::c_int,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: libc::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_obj {
    pub type_0: libc::c_int,
    pub label: libc::c_uint,
    pub generation: libc::c_ushort,
    pub refcount: libc::c_uint,
    pub flags: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_file {
    pub handle: rust_input_handle_t,
    pub trailer: *mut pdf_obj,
    pub xref_table: *mut xref_entry,
    pub catalog: *mut pdf_obj,
    pub num_obj: libc::c_int,
    pub file_size: libc::c_int,
    pub version: libc::c_uint,
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
    pub type_0: libc::c_uchar,
    pub field2: libc::c_uint,
    pub field3: libc::c_ushort,
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
    pub stream: *mut libc::c_uchar,
    pub objstm_data: *mut libc::c_int,
    pub stream_length: libc::c_uint,
    pub max_length: libc::c_uint,
    pub _flags: int32_t,
    pub decodeparms: decode_parms,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decode_parms {
    pub predictor: libc::c_int,
    pub colors: libc::c_int,
    pub bits_per_component: libc::c_int,
    pub columns: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_name {
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_indirect {
    pub pf: *mut pdf_file,
    pub obj: *mut pdf_obj,
    pub label: libc::c_uint,
    pub generation: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_array {
    pub max: libc::c_uint,
    pub size: libc::c_uint,
    pub values: *mut *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_string {
    pub string: *mut libc::c_uchar,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_number {
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_boolean {
    pub value: libc::c_char,
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
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const libc::c_char;
}
static mut pdf_output_handle: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut pdf_output_file_position: libc::c_int = 0i32;
static mut pdf_output_line_position: libc::c_int = 0i32;
static mut compression_saved: libc::c_int = 0i32;
static mut format_buffer: [libc::c_char; 4096] = [0; 4096];
static mut output_xref: *mut xref_entry = 0 as *const xref_entry as *mut xref_entry;
static mut pdf_max_ind_objects: libc::c_uint = 0;
static mut next_label: libc::c_uint = 0;
static mut startxref: libc::c_uint = 0;
static mut output_stream: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
/* the limit is only 100 for linearized PDF */
static mut enc_mode: bool = false;
static mut doc_enc_mode: bool = false;
static mut trailer_dict: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
static mut xref_stream: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
static mut verbose: libc::c_int = 0i32;
static mut compression_level: libc::c_char = 9i32 as libc::c_char;
static mut compression_use_predictor: libc::c_char = 1i32 as libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn pdf_set_compression(mut level: libc::c_int) {
    _tt_abort(
        b"You don\'t have compression compiled in. Possibly libz wasn\'t found by configure.\x00"
            as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_use_predictor(mut bval: libc::c_int) {
    compression_use_predictor = (if bval != 0 { 1i32 } else { 0i32 }) as libc::c_char;
}
static mut pdf_version: libc::c_uint = 5i32 as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn pdf_set_version(mut version: libc::c_uint) {
    /* Don't forget to update CIDFont_stdcc_def[] in cid.c too! */
    if version >= 3i32 as libc::c_uint && version <= 7i32 as libc::c_uint {
        pdf_version = version
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_version() -> libc::c_uint {
    return pdf_version;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_get_verbose() -> libc::c_int {
    return verbose;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_set_verbose(mut level: libc::c_int) {
    verbose = level;
}
static mut current_objstm: *mut pdf_obj = 0 as *const pdf_obj as *mut pdf_obj;
static mut do_objstm: libc::c_int = 0;
unsafe extern "C" fn add_xref_entry(
    mut label: libc::c_uint,
    mut type_0: libc::c_uchar,
    mut field2: libc::c_uint,
    mut field3: libc::c_ushort,
) {
    if label >= pdf_max_ind_objects {
        pdf_max_ind_objects = label
            .wrapping_div(512i32 as libc::c_uint)
            .wrapping_add(1i32 as libc::c_uint)
            .wrapping_mul(512i32 as libc::c_uint);
        output_xref = renew(
            output_xref as *mut libc::c_void,
            (pdf_max_ind_objects as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xref_entry>() as libc::c_ulong)
                as u32,
        ) as *mut xref_entry
    }
    (*output_xref.offset(label as isize)).type_0 = type_0;
    (*output_xref.offset(label as isize)).field2 = field2;
    (*output_xref.offset(label as isize)).field3 = field3;
    let ref mut fresh0 = (*output_xref.offset(label as isize)).direct;
    *fresh0 = 0 as *mut pdf_obj;
    let ref mut fresh1 = (*output_xref.offset(label as isize)).indirect;
    *fresh1 = 0 as *mut pdf_obj;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_out_init(
    mut filename: *const libc::c_char,
    mut do_encryption: bool,
    mut enable_object_stream: bool,
) {
    let mut v: libc::c_char = 0;
    output_xref = 0 as *mut xref_entry;
    pdf_max_ind_objects = 0i32 as libc::c_uint;
    add_xref_entry(
        0i32 as libc::c_uint,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uint,
        0xffffi32 as libc::c_ushort,
    );
    next_label = 1i32 as libc::c_uint;
    if pdf_version >= 5i32 as libc::c_uint {
        if enable_object_stream {
            xref_stream = pdf_new_stream(1i32 << 0i32);
            (*xref_stream).flags |= 1i32 << 1i32;
            trailer_dict = pdf_stream_dict(xref_stream);
            pdf_add_dict(
                trailer_dict,
                pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
                pdf_new_name(b"XRef\x00" as *const u8 as *const libc::c_char),
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
        _tt_abort(b"stdout PDF output not supported\x00" as *const u8 as *const libc::c_char);
    }
    pdf_output_handle = ttstub_output_open(filename, 0i32);
    if pdf_output_handle.is_null() {
        if strlen(filename) < 128i32 as libc::c_ulong {
            _tt_abort(
                b"Unable to open \"%s\".\x00" as *const u8 as *const libc::c_char,
                filename,
            );
        } else {
            _tt_abort(b"Unable to open file.\x00" as *const u8 as *const libc::c_char);
        }
    }
    pdf_out(
        pdf_output_handle,
        b"%PDF-1.\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"%PDF-1.\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    v = ('0' as i32 as libc::c_uint).wrapping_add(pdf_version) as libc::c_char;
    pdf_out(
        pdf_output_handle,
        &mut v as *mut libc::c_char as *const libc::c_void,
        1i32,
    );
    pdf_out(
        pdf_output_handle,
        b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1i32,
    );
    pdf_out(
        pdf_output_handle,
        b"%\xe4\xf0\xed\xf8\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"%\xe4\xf0\xed\xf8\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    enc_mode = 0i32 != 0;
    doc_enc_mode = do_encryption;
}
unsafe extern "C" fn dump_xref_table() {
    let mut length: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    pdf_out(
        pdf_output_handle,
        b"xref\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        5i32,
    );
    length = sprintf(
        format_buffer.as_mut_ptr(),
        b"%d %u\n\x00" as *const u8 as *const libc::c_char,
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
    i = 0i32 as libc::c_uint;
    while i < next_label {
        let mut type_0: libc::c_uchar = (*output_xref.offset(i as isize)).type_0;
        if type_0 as libc::c_int > 1i32 {
            _tt_abort(
                b"object type %c not allowed in xref table\x00" as *const u8 as *const libc::c_char,
                type_0 as libc::c_int,
            );
        }
        length = sprintf(
            format_buffer.as_mut_ptr(),
            b"%010u %05hu %c \n\x00" as *const u8 as *const libc::c_char,
            (*output_xref.offset(i as isize)).field2,
            (*output_xref.offset(i as isize)).field3 as libc::c_int,
            if type_0 as libc::c_int != 0 {
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
        b"trailer\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8i32,
    );
    enc_mode = 0i32 != 0;
    write_dict((*trailer_dict).data as *mut pdf_dict, pdf_output_handle);
    pdf_release_obj(trailer_dict);
    pdf_out_char(pdf_output_handle, '\n' as i32 as libc::c_char);
}
/*
 * output a PDF 1.5 cross-reference stream;
 * contributed by Matthias Franz (March 21, 2007)
 */
unsafe extern "C" fn dump_xref_stream() {
    let mut pos: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut poslen: libc::c_uint = 0;
    let mut buf: [libc::c_uchar; 7] = [
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0,
        0,
    ];
    let mut w: *mut pdf_obj = 0 as *mut pdf_obj;
    /* determine the necessary size of the offset field */
    pos = startxref; /* maximal offset value */
    poslen = 1i32 as libc::c_uint; /* type                */
    loop {
        pos >>= 8i32; /* offset (big-endian) */
        if !(pos != 0) {
            break; /* generation          */
        }
        poslen = poslen.wrapping_add(1)
    }
    w = pdf_new_array();
    pdf_add_array(w, pdf_new_number(1i32 as libc::c_double));
    pdf_add_array(w, pdf_new_number(poslen as libc::c_double));
    pdf_add_array(w, pdf_new_number(2i32 as libc::c_double));
    pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"W\x00" as *const u8 as *const libc::c_char),
        w,
    );
    /* We need the xref entry for the xref stream right now */
    add_xref_entry(
        next_label.wrapping_sub(1i32 as libc::c_uint),
        1i32 as libc::c_uchar,
        startxref,
        0i32 as libc::c_ushort,
    );
    i = 0i32 as libc::c_uint;
    while i < next_label {
        let mut j: libc::c_uint = 0;
        let mut f3: libc::c_ushort = 0;
        buf[0] = (*output_xref.offset(i as isize)).type_0;
        pos = (*output_xref.offset(i as isize)).field2;
        j = poslen;
        loop {
            let fresh2 = j;
            j = j.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            buf[(1i32 as libc::c_uint).wrapping_add(j) as usize] = pos as libc::c_uchar;
            pos >>= 8i32
        }
        f3 = (*output_xref.offset(i as isize)).field3;
        buf[poslen.wrapping_add(1i32 as libc::c_uint) as usize] =
            (f3 as libc::c_int >> 8i32) as libc::c_uchar;
        buf[poslen.wrapping_add(2i32 as libc::c_uint) as usize] = f3 as libc::c_uchar;
        pdf_add_stream(
            xref_stream,
            &mut buf as *mut [libc::c_uchar; 7] as *const libc::c_void,
            poslen.wrapping_add(3i32 as libc::c_uint) as libc::c_int,
        );
        i = i.wrapping_add(1)
    }
    pdf_release_obj(xref_stream);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_out_flush() {
    if !pdf_output_handle.is_null() {
        let mut length: libc::c_int = 0;
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
        startxref = pdf_output_file_position as libc::c_uint;
        pdf_add_dict(
            trailer_dict,
            pdf_new_name(b"Size\x00" as *const u8 as *const libc::c_char),
            pdf_new_number(next_label as libc::c_double),
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
            b"startxref\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            10i32,
        );
        length = sprintf(
            format_buffer.as_mut_ptr(),
            b"%u\n\x00" as *const u8 as *const libc::c_char,
            startxref,
        );
        pdf_out(
            pdf_output_handle,
            format_buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
        pdf_out(
            pdf_output_handle,
            b"%%EOF\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            6i32,
        );
        if verbose != 0 {
            if compression_level as libc::c_int > 0i32 {
                dpx_message(
                    b"Compression saved %d bytes%s\n\x00" as *const u8 as *const libc::c_char,
                    compression_saved,
                    if pdf_version < 5i32 as libc::c_uint {
                        b". Try \"-V 5\" for better compression\x00" as *const u8
                            as *const libc::c_char
                    } else {
                        b"\x00" as *const u8 as *const libc::c_char
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
        pdf_new_name(b"Root\x00" as *const u8 as *const libc::c_char),
        pdf_ref_obj(object),
    ) != 0
    {
        _tt_abort(b"Root object already set!\x00" as *const u8 as *const libc::c_char);
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
        pdf_new_name(b"Info\x00" as *const u8 as *const libc::c_char),
        pdf_ref_obj(object),
    ) != 0
    {
        _tt_abort(b"Info object already set!\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_id(mut id: *mut pdf_obj) {
    if pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"ID\x00" as *const u8 as *const libc::c_char),
        id,
    ) != 0
    {
        _tt_abort(b"ID already set!\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_encrypt(mut encrypt: *mut pdf_obj) {
    if pdf_add_dict(
        trailer_dict,
        pdf_new_name(b"Encrypt\x00" as *const u8 as *const libc::c_char),
        pdf_ref_obj(encrypt),
    ) != 0
    {
        _tt_abort(b"Encrypt object already set!\x00" as *const u8 as *const libc::c_char);
    }
    (*encrypt).flags |= 1i32 << 1i32;
}
unsafe extern "C" fn pdf_out_char(mut handle: rust_output_handle_t, mut c: libc::c_char) {
    if !output_stream.is_null() && handle == pdf_output_handle {
        pdf_add_stream(
            output_stream,
            &mut c as *mut libc::c_char as *const libc::c_void,
            1i32,
        );
    } else {
        ttstub_output_putc(handle, c as libc::c_int);
        /* Keep tallys for xref table *only* if writing a pdf file. */
        if handle == pdf_output_handle {
            pdf_output_file_position += 1i32;
            if c as libc::c_int == '\n' as i32 {
                pdf_output_line_position = 0i32
            } else {
                pdf_output_line_position += 1i32
            }
        }
    };
}
static mut xchar: [libc::c_char; 17] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 0,
];
unsafe extern "C" fn pdf_out(
    mut handle: rust_output_handle_t,
    mut buffer: *const libc::c_void,
    mut length: libc::c_int,
) {
    if !output_stream.is_null() && handle == pdf_output_handle {
        pdf_add_stream(output_stream, buffer, length);
    } else {
        ttstub_output_write(handle, buffer as *const libc::c_char, length as size_t);
        /* Keep tallys for xref table *only* if writing a pdf file */
        if handle == pdf_output_handle {
            pdf_output_file_position += length;
            pdf_output_line_position += length;
            /* "foo\nbar\n "... */
            if length > 0i32
                && *(buffer as *const libc::c_char).offset((length - 1i32) as isize) as libc::c_int
                    == '\n' as i32
            {
                pdf_output_line_position = 0i32
            }
        }
    };
}
/*  returns 1 if a white-space character is necessary to separate
an object of type1 followed by an object of type2              */
unsafe extern "C" fn pdf_need_white(mut type1: libc::c_int, mut type2: libc::c_int) -> libc::c_int {
    return !(type1 == 3i32
        || type1 == 5i32
        || type1 == 6i32
        || type2 == 3i32
        || type2 == 4i32
        || type2 == 5i32
        || type2 == 6i32) as libc::c_int;
}
unsafe extern "C" fn pdf_out_white(mut handle: rust_output_handle_t) {
    if handle == pdf_output_handle && pdf_output_line_position >= 80i32 {
        pdf_out_char(handle, '\n' as i32 as libc::c_char);
    } else {
        pdf_out_char(handle, ' ' as i32 as libc::c_char);
    };
}
unsafe extern "C" fn pdf_new_obj(mut type_0: libc::c_int) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    if type_0 > 10i32 || type_0 < 0i32 {
        _tt_abort(
            b"Invalid object type: %d\x00" as *const u8 as *const libc::c_char,
            type_0,
        );
    }
    result = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_obj>() as libc::c_ulong)
        as u32) as *mut pdf_obj;
    (*result).type_0 = type_0;
    (*result).data = 0 as *mut libc::c_void;
    (*result).label = 0i32 as libc::c_uint;
    (*result).generation = 0i32 as libc::c_ushort;
    (*result).refcount = 1i32 as libc::c_uint;
    (*result).flags = 0i32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_typeof(mut object: *mut pdf_obj) -> libc::c_int {
    if object.is_null() || (*object).type_0 <= 0i32 || (*object).type_0 > 10i32 {
        return 0i32;
    }
    return (*object).type_0;
}
unsafe extern "C" fn pdf_label_obj(mut object: *mut pdf_obj) {
    if object.is_null() || (*object).type_0 <= 0i32 || (*object).type_0 > 10i32 {
        _tt_abort(
            b"pdf_label_obj(): passed invalid object.\x00" as *const u8 as *const libc::c_char,
        );
    }
    /*
     * Don't change label on an already labeled object. Ignore such calls.
     */
    if (*object).label == 0i32 as libc::c_uint {
        let fresh3 = next_label;
        next_label = next_label.wrapping_add(1);
        (*object).label = fresh3;
        (*object).generation = 0i32 as libc::c_ushort
    };
}
/*
 * Transfer the label assigned to the object src to the object dst.
 * The object dst must not yet have been labeled.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_transfer_label(mut dst: *mut pdf_obj, mut src: *mut pdf_obj) {
    if !dst.is_null() && (*dst).label == 0 && !src.is_null() {
    } else {
        __assert_fail(
            b"dst && !dst->label && src\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            682i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void pdf_transfer_label(pdf_obj *, pdf_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*dst).label = (*src).label;
    (*dst).generation = (*src).generation;
    (*src).label = 0i32 as libc::c_uint;
    (*src).generation = 0i32 as libc::c_ushort;
}
/*
 * This doesn't really copy the object, but allows it to be used without
 * fear that somebody else will free it.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_link_obj(mut object: *mut pdf_obj) -> *mut pdf_obj {
    if object.is_null() || (*object).type_0 <= 0i32 || (*object).type_0 > 10i32 {
        _tt_abort(
            b"pdf_link_obj(): passed invalid object.\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*object).refcount = (*object).refcount.wrapping_add(1i32 as libc::c_uint);
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_ref_obj(mut object: *mut pdf_obj) -> *mut pdf_obj {
    if object.is_null() || (*object).type_0 <= 0i32 || (*object).type_0 > 10i32 {
        _tt_abort(b"pdf_ref_obj(): passed invalid object.\x00" as *const u8 as *const libc::c_char);
    }
    if (*object).refcount == 0i32 as libc::c_uint {
        dpx_message(
            b"\nTrying to refer already released object!!!\n\x00" as *const u8
                as *const libc::c_char,
        );
        pdf_write_obj(object, ttstub_output_open_stdout());
        _tt_abort(b"Cannot continue...\x00" as *const u8 as *const libc::c_char);
    }
    if !object.is_null() && pdf_obj_typeof(object) == 9i32 {
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
    let mut length: libc::c_int = 0;
    if (*indirect).pf.is_null() {
    } else {
        __assert_fail(
            b"!indirect->pf\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            736i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"void write_indirect(pdf_indirect *, rust_output_handle_t)\x00",
            ))
            .as_ptr(),
        );
    }
    length = sprintf(
        format_buffer.as_mut_ptr(),
        b"%u %hu R\x00" as *const u8 as *const libc::c_char,
        (*indirect).label,
        (*indirect).generation as libc::c_int,
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
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_null() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    result = pdf_new_obj(8i32);
    (*result).data = 0 as *mut libc::c_void;
    return result;
}
unsafe extern "C" fn write_null(mut handle: rust_output_handle_t) {
    pdf_out(
        handle,
        b"null\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_boolean(mut value: libc::c_char) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_boolean = 0 as *mut pdf_boolean;
    result = pdf_new_obj(1i32);
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_boolean>() as libc::c_ulong)
        as u32) as *mut pdf_boolean;
    (*data).value = value;
    (*result).data = data as *mut libc::c_void;
    return result;
}
unsafe extern "C" fn release_boolean(mut data: *mut pdf_obj) {
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn write_boolean(mut data: *mut pdf_boolean, mut handle: rust_output_handle_t) {
    if (*data).value != 0 {
        pdf_out(
            handle,
            b"true\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4i32,
        );
    } else {
        pdf_out(
            handle,
            b"false\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            5i32,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_boolean_value(mut object: *mut pdf_obj) -> libc::c_char {
    let mut data: *mut pdf_boolean = 0 as *mut pdf_boolean;
    if object.is_null() || (*object).type_0 != 1i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -1i32
            },
            1i32,
            808i32,
        );
    }
    data = (*object).data as *mut pdf_boolean;
    return (*data).value;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_number(mut value: libc::c_double) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_number = 0 as *mut pdf_number;
    result = pdf_new_obj(2i32);
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_number>() as libc::c_ulong)
        as u32) as *mut pdf_number;
    (*data).value = value;
    (*result).data = data as *mut libc::c_void;
    return result;
}
unsafe extern "C" fn release_number(mut data: *mut pdf_number) {
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn write_number(mut number: *mut pdf_number, mut handle: rust_output_handle_t) {
    let mut count: libc::c_int = 0;
    count = pdf_sprint_number(format_buffer.as_mut_ptr(), (*number).value);
    pdf_out(
        handle,
        format_buffer.as_mut_ptr() as *const libc::c_void,
        count,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_number(mut object: *mut pdf_obj, mut value: libc::c_double) {
    let mut data: *mut pdf_number = 0 as *mut pdf_number;
    if object.is_null() || (*object).type_0 != 2i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
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
pub unsafe extern "C" fn pdf_number_value(mut object: *mut pdf_obj) -> libc::c_double {
    let mut data: *mut pdf_number = 0 as *mut pdf_number;
    if object.is_null() || (*object).type_0 != 2i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -1i32
            },
            2i32,
            862i32,
        );
    }
    data = (*object).data as *mut pdf_number;
    return (*data).value;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_string(
    mut str: *const libc::c_void,
    mut length: size_t,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if !str.is_null() {
    } else {
        __assert_fail(
            b"str\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            875i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"pdf_obj *pdf_new_string(const void *, size_t)\x00",
            ))
            .as_ptr(),
        );
    }
    result = pdf_new_obj(3i32);
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_string>() as libc::c_ulong)
        as u32) as *mut pdf_string;
    (*result).data = data as *mut libc::c_void;
    (*data).length = length;
    if length != 0 {
        (*data).string = new((length.wrapping_add(1i32 as libc::c_ulong) as u32
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32) as *mut libc::c_uchar;
        memcpy((*data).string as *mut libc::c_void, str, length);
        /* Shouldn't assume NULL terminated. */
        *(*data).string.offset(length as isize) = '\u{0}' as i32 as libc::c_uchar
    } else {
        (*data).string = 0 as *mut libc::c_uchar
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_string_value(mut object: *mut pdf_obj) -> *mut libc::c_void {
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if object.is_null() || (*object).type_0 != 3i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -1i32
            },
            3i32,
            898i32,
        );
    }
    data = (*object).data as *mut pdf_string;
    return (*data).string as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_string_length(mut object: *mut pdf_obj) -> libc::c_uint {
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if object.is_null() || (*object).type_0 != 3i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -1i32
            },
            3i32,
            910i32,
        );
    }
    data = (*object).data as *mut pdf_string;
    return (*data).length as libc::c_uint;
}
/*
 * This routine escapes non printable characters and control
 * characters in an output string.
 */
#[no_mangle]
pub unsafe extern "C" fn pdfobj_escape_str(
    mut buffer: *mut libc::c_char,
    mut bufsize: size_t,
    mut s: *const libc::c_uchar,
    mut len: size_t,
) -> size_t {
    let mut result: size_t = 0i32 as size_t;
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < len {
        let mut ch: libc::c_uchar = 0;
        ch = *s.offset(i as isize);
        if result > bufsize.wrapping_sub(4i32 as libc::c_ulong) {
            _tt_abort(
                b"pdfobj_escape_str: Buffer overflow\x00" as *const u8 as *const libc::c_char,
            );
        }
        /*
         * We always write three octal digits. Optimization only gives few Kb
         * smaller size for most documents when zlib compressed.
         */
        if (ch as libc::c_int) < 32i32 || ch as libc::c_int > 126i32 {
            let fresh4 = result; /* Shouldn't use format_buffer[]. */
            result = result.wrapping_add(1);
            *buffer.offset(fresh4 as isize) = '\\' as i32 as libc::c_char;
            result = (result as libc::c_ulong).wrapping_add(sprintf(
                buffer.offset(result as isize),
                b"%03o\x00" as *const u8 as *const libc::c_char,
                ch as libc::c_int,
            ) as libc::c_ulong) as size_t as size_t
        } else {
            match ch as libc::c_int {
                40 => {
                    let fresh5 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh5 as isize) = '\\' as i32 as libc::c_char;
                    let fresh6 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh6 as isize) = '(' as i32 as libc::c_char
                }
                41 => {
                    let fresh7 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh7 as isize) = '\\' as i32 as libc::c_char;
                    let fresh8 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh8 as isize) = ')' as i32 as libc::c_char
                }
                92 => {
                    let fresh9 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh9 as isize) = '\\' as i32 as libc::c_char;
                    let fresh10 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh10 as isize) = '\\' as i32 as libc::c_char
                }
                _ => {
                    let fresh11 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh11 as isize) = ch as libc::c_char
                }
            }
        }
        i = i.wrapping_add(1)
    }
    return result;
}
unsafe extern "C" fn write_string(mut str: *mut pdf_string, mut handle: rust_output_handle_t) {
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wbuf: [libc::c_char; 4096] = [0; 4096];
    let mut nescc: libc::c_int = 0i32;
    let mut count: libc::c_int = 0;
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
        if *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            nescc += 1
        }
        i = i.wrapping_add(1)
    }
    /*
     * If the string contains much escaped chars, then we write it as
     * ASCII hex string.
     */
    if nescc as libc::c_ulong > len.wrapping_div(3i32 as libc::c_ulong) {
        pdf_out_char(handle, '<' as i32 as libc::c_char);
        i = 0i32 as size_t;
        while i < len {
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as libc::c_int >> 4i32 & 0xfi32) as usize],
            );
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as libc::c_int & 0xfi32) as usize],
            );
            i = i.wrapping_add(1)
        }
        pdf_out_char(handle, '>' as i32 as libc::c_char);
    } else {
        pdf_out_char(handle, '(' as i32 as libc::c_char);
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
            ) as libc::c_int;
            pdf_out(handle, wbuf.as_mut_ptr() as *const libc::c_void, count);
            i = i.wrapping_add(1)
        }
        pdf_out_char(handle, ')' as i32 as libc::c_char);
    }
    if enc_mode as libc::c_int != 0 && !s.is_null() {
        free(s as *mut libc::c_void);
    };
}
unsafe extern "C" fn release_string(mut data: *mut pdf_string) {
    (*data).string = mfree((*data).string as *mut libc::c_void) as *mut libc::c_uchar;
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_set_string(
    mut object: *mut pdf_obj,
    mut str: *mut libc::c_uchar,
    mut length: size_t,
) {
    let mut data: *mut pdf_string = 0 as *mut pdf_string;
    if object.is_null() || (*object).type_0 != 3i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
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
    if length != 0i32 as libc::c_ulong {
        (*data).length = length;
        (*data).string = new((length.wrapping_add(1i32 as libc::c_ulong) as u32
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32) as *mut libc::c_uchar;
        memcpy(
            (*data).string as *mut libc::c_void,
            str as *const libc::c_void,
            length,
        );
        *(*data).string.offset(length as isize) = '\u{0}' as i32 as libc::c_uchar
    } else {
        (*data).length = 0i32 as size_t;
        (*data).string = 0 as *mut libc::c_uchar
    };
}
/* Name does *not* include the /. */
#[no_mangle]
pub unsafe extern "C" fn pdf_new_name(mut name: *const libc::c_char) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut length: libc::c_uint = 0;
    let mut data: *mut pdf_name = 0 as *mut pdf_name;
    result = pdf_new_obj(4i32);
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_name>() as libc::c_ulong)
        as u32) as *mut pdf_name;
    (*result).data = data as *mut libc::c_void;
    length = strlen(name) as libc::c_uint;
    if length != 0i32 as libc::c_uint {
        (*data).name = new((length.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as u32) as *mut libc::c_char;
        memcpy(
            (*data).name as *mut libc::c_void,
            name as *const libc::c_void,
            length as libc::c_ulong,
        );
        *(*data).name.offset(length as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        (*data).name = 0 as *mut libc::c_char
    }
    return result;
}
unsafe extern "C" fn write_name(mut name: *mut pdf_name, mut handle: rust_output_handle_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    s = (*name).name;
    length = (if !(*name).name.is_null() {
        strlen((*name).name)
    } else {
        0i32 as libc::c_ulong
    }) as libc::c_int;
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
    pdf_out_char(handle, '/' as i32 as libc::c_char);
    i = 0i32;
    while i < length {
        if (*s.offset(i as isize) as libc::c_int) < '!' as i32
            || *s.offset(i as isize) as libc::c_int > '~' as i32
            || *s.offset(i as isize) as libc::c_int == '#' as i32
            || (*s.offset(i as isize) as libc::c_int == '(' as i32
                || *s.offset(i as isize) as libc::c_int == ')' as i32
                || *s.offset(i as isize) as libc::c_int == '/' as i32
                || *s.offset(i as isize) as libc::c_int == '<' as i32
                || *s.offset(i as isize) as libc::c_int == '>' as i32
                || *s.offset(i as isize) as libc::c_int == '[' as i32
                || *s.offset(i as isize) as libc::c_int == ']' as i32
                || *s.offset(i as isize) as libc::c_int == '{' as i32
                || *s.offset(i as isize) as libc::c_int == '}' as i32
                || *s.offset(i as isize) as libc::c_int == '%' as i32)
        {
            /*     ^ "space" is here. */
            pdf_out_char(handle, '#' as i32 as libc::c_char);
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as libc::c_int >> 4i32 & 0xfi32) as usize],
            );
            pdf_out_char(
                handle,
                xchar[(*s.offset(i as isize) as libc::c_int & 0xfi32) as usize],
            );
        } else {
            pdf_out_char(handle, *s.offset(i as isize));
        }
        i += 1
    }
}
unsafe extern "C" fn release_name(mut data: *mut pdf_name) {
    (*data).name = mfree((*data).name as *mut libc::c_void) as *mut libc::c_char;
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_name_value(mut object: *mut pdf_obj) -> *mut libc::c_char {
    let mut data: *mut pdf_name = 0 as *mut pdf_name;
    if object.is_null() || (*object).type_0 != 4i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -1i32
            },
            4i32,
            1121i32,
        );
    }
    data = (*object).data as *mut pdf_name;
    return (*data).name;
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
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_array>() as libc::c_ulong)
        as u32) as *mut pdf_array;
    (*data).values = 0 as *mut *mut pdf_obj;
    (*data).max = 0i32 as libc::c_uint;
    (*data).size = 0i32 as libc::c_uint;
    (*result).data = data as *mut libc::c_void;
    return result;
}
unsafe extern "C" fn write_array(mut array: *mut pdf_array, mut handle: rust_output_handle_t) {
    pdf_out_char(handle, '[' as i32 as libc::c_char);
    if (*array).size > 0i32 as libc::c_uint {
        let mut i: libc::c_uint = 0;
        let mut type1: libc::c_int = 10i32;
        let mut type2: libc::c_int = 0;
        i = 0i32 as libc::c_uint;
        while i < (*array).size {
            if !(*(*array).values.offset(i as isize)).is_null() {
                type2 = (**(*array).values.offset(i as isize)).type_0;
                if type1 != 10i32 && pdf_need_white(type1, type2) != 0 {
                    pdf_out_white(handle);
                }
                type1 = type2;
                pdf_write_obj(*(*array).values.offset(i as isize), handle);
            } else {
                dpx_warning(
                    b"PDF array element %d undefined.\x00" as *const u8 as *const libc::c_char,
                    i,
                );
            }
            i = i.wrapping_add(1)
        }
    }
    pdf_out_char(handle, ']' as i32 as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_get_array(
    mut array: *mut pdf_obj,
    mut idx: libc::c_int,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    if array.is_null() || (*array).type_0 != 5i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !array.is_null() {
                (*array).type_0
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
            .offset((idx as libc::c_uint).wrapping_add((*data).size) as isize)
    } else if (idx as libc::c_uint) < (*data).size {
        result = *(*data).values.offset(idx as isize)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_array_length(mut array: *mut pdf_obj) -> libc::c_uint {
    let mut data: *mut pdf_array = 0 as *mut pdf_array;
    if array.is_null() || (*array).type_0 != 5i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !array.is_null() {
                (*array).type_0
            } else {
                -1i32
            },
            5i32,
            1194i32,
        );
    }
    data = (*array).data as *mut pdf_array;
    return (*data).size;
}
unsafe extern "C" fn release_array(mut data: *mut pdf_array) {
    let mut i: libc::c_uint = 0;
    if !(*data).values.is_null() {
        i = 0i32 as libc::c_uint;
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
    if array.is_null() || (*array).type_0 != 5i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !array.is_null() {
                (*array).type_0
            } else {
                -1i32
            },
            5i32,
            1225i32,
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
    let ref mut fresh13 = *(*data).values.offset((*data).size as isize);
    *fresh13 = object;
    (*data).size = (*data).size.wrapping_add(1);
}
unsafe extern "C" fn write_dict(mut dict: *mut pdf_dict, mut handle: rust_output_handle_t) {
    pdf_out(
        handle,
        b"<<\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        2i32,
    );
    while !(*dict).key.is_null() {
        pdf_write_obj((*dict).key, handle);
        if pdf_need_white(4i32, (*(*dict).value).type_0) != 0 {
            pdf_out_white(handle);
        }
        pdf_write_obj((*dict).value, handle);
        dict = (*dict).next
    }
    pdf_out(
        handle,
        b">>\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        2i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_dict() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    result = pdf_new_obj(6i32);
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_dict>() as libc::c_ulong)
        as u32) as *mut pdf_dict;
    (*data).key = 0 as *mut pdf_obj;
    (*data).value = 0 as *mut pdf_obj;
    (*data).next = 0 as *mut pdf_dict;
    (*result).data = data as *mut libc::c_void;
    return result;
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
) -> libc::c_int {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    let mut new_node: *mut pdf_dict = 0 as *mut pdf_dict;
    if dict.is_null() || (*dict).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -1i32
            },
            6i32,
            1313i32,
        );
    }
    if key.is_null() || (*key).type_0 != 4i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !key.is_null() { (*key).type_0 } else { -1i32 },
            4i32,
            1314i32,
        );
    }
    /* It seems that NULL is sometimes used for null object... */
    if !value.is_null() && (value.is_null() || (*value).type_0 <= 0i32 || (*value).type_0 > 10i32) {
        _tt_abort(b"pdf_add_dict(): Passed invalid value\x00" as *const u8 as *const libc::c_char);
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
    new_node = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_dict>() as libc::c_ulong)
        as u32) as *mut pdf_dict;
    (*new_node).key = 0 as *mut pdf_obj;
    (*new_node).value = 0 as *mut pdf_obj;
    (*new_node).next = 0 as *mut pdf_dict;
    (*data).next = new_node;
    (*data).key = key;
    (*data).value = value;
    return 0i32;
}
/* pdf_merge_dict makes a link for each item in dict2 before stealing it */
#[no_mangle]
pub unsafe extern "C" fn pdf_merge_dict(mut dict1: *mut pdf_obj, mut dict2: *mut pdf_obj) {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    if dict1.is_null() || (*dict1).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict1.is_null() {
                (*dict1).type_0
            } else {
                -1i32
            },
            6i32,
            1352i32,
        );
    }
    if dict2.is_null() || (*dict2).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict2.is_null() {
                (*dict2).type_0
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
        unsafe extern "C" fn(_: *mut pdf_obj, _: *mut pdf_obj, _: *mut libc::c_void) -> libc::c_int,
    >,
    mut pdata: *mut libc::c_void,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    if proc_0.is_some() {
    } else {
        __assert_fail(b"proc\x00" as *const u8 as *const libc::c_char,
                      b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
                      1369i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 79],
                                                &[libc::c_char; 79]>(b"int pdf_foreach_dict(pdf_obj *, int (*)(pdf_obj *, pdf_obj *, void *), void *)\x00")).as_ptr());
    }
    if dict.is_null() || (*dict).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -1i32
            },
            6i32,
            1371i32,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    while error == 0 && !(*data).key.is_null() {
        error = proc_0.expect("non-null function pointer")((*data).key, (*data).value, pdata);
        data = (*data).next
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_lookup_dict(
    mut dict: *mut pdf_obj,
    mut name: *const libc::c_char,
) -> *mut pdf_obj {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    if !name.is_null() {
    } else {
        __assert_fail(
            b"name\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            1389i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"pdf_obj *pdf_lookup_dict(pdf_obj *, const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    if dict.is_null() || (*dict).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -1i32
            },
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
    return 0 as *mut pdf_obj;
}
/* Returns array of dictionary keys */
#[no_mangle]
pub unsafe extern "C" fn pdf_dict_keys(mut dict: *mut pdf_obj) -> *mut pdf_obj {
    let mut keys: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    if dict.is_null() || (*dict).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -1i32
            },
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
    return keys;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_remove_dict(mut dict: *mut pdf_obj, mut name: *const libc::c_char) {
    let mut data: *mut pdf_dict = 0 as *mut pdf_dict;
    let mut data_p: *mut *mut pdf_dict = 0 as *mut *mut pdf_dict;
    if dict.is_null() || (*dict).type_0 != 6i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -1i32
            },
            6i32,
            1430i32,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    data_p = &mut (*dict).data as *mut *mut libc::c_void as *mut libc::c_void as *mut *mut pdf_dict;
    while !(*data).key.is_null() {
        if !(*data).key.is_null()
            && !name.is_null()
            && streq_ptr((*((*(*data).key).data as *mut pdf_name)).name, name) as libc::c_int != 0
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
pub unsafe extern "C" fn pdf_new_stream(mut flags: libc::c_int) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    result = pdf_new_obj(7i32);
    data = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_stream>() as libc::c_ulong)
        as u32) as *mut pdf_stream;
    /*
     * Although we are using an arbitrary pdf_object here, it must have
     * type=PDF_DICT and cannot be an indirect reference.  This will be
     * checked by the output routine.
     */
    (*data).dict = pdf_new_dict();
    (*data)._flags = flags;
    (*data).stream = 0 as *mut libc::c_uchar;
    (*data).stream_length = 0i32 as libc::c_uint;
    (*data).max_length = 0i32 as libc::c_uint;
    (*data).objstm_data = 0 as *mut libc::c_int;
    (*data).decodeparms.predictor = 2i32;
    (*data).decodeparms.columns = 0i32;
    (*data).decodeparms.bits_per_component = 0i32;
    (*data).decodeparms.colors = 0i32;
    (*result).data = data as *mut libc::c_void;
    (*result).flags |= 1i32 << 0i32;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_set_predictor(
    mut stream: *mut pdf_obj,
    mut predictor: libc::c_int,
    mut columns: int32_t,
    mut bpc: libc::c_int,
    mut colors: libc::c_int,
) {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if pdf_obj_typeof(stream) != 7i32 {
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
unsafe extern "C" fn write_stream(mut stream: *mut pdf_stream, mut handle: rust_output_handle_t) {
    let mut filtered: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut filtered_length: libc::c_uint = 0;
    let mut buffer_length: libc::c_uint = 0;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /*
     * Always work from a copy of the stream. All filters read from
     * "filtered" and leave their result in "filtered".
     */
    filtered = new(((*stream).stream_length as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as u32) as *mut libc::c_uchar;
    memcpy(
        filtered as *mut libc::c_void,
        (*stream).stream as *const libc::c_void,
        (*stream).stream_length as libc::c_ulong,
    );
    filtered_length = (*stream).stream_length;
    /* PDF/A requires Metadata to be not filtered. */
    let mut type_0: *mut pdf_obj = 0 as *mut pdf_obj;
    type_0 = pdf_lookup_dict(
        (*stream).dict,
        b"Type\x00" as *const u8 as *const libc::c_char,
    );
    if !type_0.is_null()
        && streq_ptr(
            b"Metadata\x00" as *const u8 as *const libc::c_char,
            pdf_name_value(type_0),
        ) as libc::c_int
            != 0
    {
        (*stream)._flags &= !(1i32 << 0i32)
    }
    /* HAVE_ZLIB */
    /* AES will change the size of data! */
    if enc_mode {
        let mut cipher: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut cipher_len: size_t = 0i32 as size_t;
        pdf_encrypt_data(
            filtered,
            filtered_length as size_t,
            &mut cipher,
            &mut cipher_len,
        );
        free(filtered as *mut libc::c_void);
        filtered = cipher;
        filtered_length = cipher_len as libc::c_uint
    }
    pdf_add_dict(
        (*stream).dict,
        pdf_new_name(b"Length\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(filtered_length as libc::c_double),
    );
    pdf_write_obj((*stream).dict, handle);
    pdf_out(
        handle,
        b"\nstream\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8i32,
    );
    if filtered_length > 0i32 as libc::c_uint {
        pdf_out(
            handle,
            filtered as *const libc::c_void,
            filtered_length as libc::c_int,
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
        b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1i32,
    );
    pdf_out(
        handle,
        b"endstream\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        9i32,
    );
}
unsafe extern "C" fn release_stream(mut stream: *mut pdf_stream) {
    pdf_release_obj((*stream).dict);
    (*stream).dict = 0 as *mut pdf_obj;
    (*stream).stream = mfree((*stream).stream as *mut libc::c_void) as *mut libc::c_uchar;
    (*stream).objstm_data = mfree((*stream).objstm_data as *mut libc::c_void) as *mut libc::c_int;
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_dict(mut stream: *mut pdf_obj) -> *mut pdf_obj {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || (*stream).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -1i32
            },
            7i32,
            1961i32,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    return (*data).dict;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_dataptr(mut stream: *mut pdf_obj) -> *const libc::c_void {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || (*stream).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -1i32
            },
            7i32,
            1973i32,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    return (*data).stream as *const libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_stream_length(mut stream: *mut pdf_obj) -> libc::c_int {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || (*stream).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -1i32
            },
            7i32,
            1985i32,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    return (*data).stream_length as libc::c_int;
}
unsafe extern "C" fn set_objstm_data(mut objstm: *mut pdf_obj, mut data: *mut libc::c_int) {
    if objstm.is_null() || (*objstm).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !objstm.is_null() {
                (*objstm).type_0
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
unsafe extern "C" fn get_objstm_data(mut objstm: *mut pdf_obj) -> *mut libc::c_int {
    if objstm.is_null() || (*objstm).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !objstm.is_null() {
                (*objstm).type_0
            } else {
                -1i32
            },
            7i32,
            2001i32,
        );
    }
    return (*((*objstm).data as *mut pdf_stream)).objstm_data;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_add_stream(
    mut stream: *mut pdf_obj,
    mut stream_data: *const libc::c_void,
    mut length: libc::c_int,
) {
    let mut data: *mut pdf_stream = 0 as *mut pdf_stream;
    if stream.is_null() || (*stream).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !stream.is_null() {
                (*stream).type_0
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
    if (*data).stream_length.wrapping_add(length as libc::c_uint) > (*data).max_length {
        (*data).max_length = (*data)
            .max_length
            .wrapping_add((length as libc::c_uint).wrapping_add(4096u32));
        (*data).stream = renew(
            (*data).stream as *mut libc::c_void,
            ((*data).max_length as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as u32,
        ) as *mut libc::c_uchar
    }
    memcpy(
        (*data).stream.offset((*data).stream_length as isize) as *mut libc::c_void,
        stream_data,
        length as libc::c_ulong,
    );
    (*data).stream_length = (*data).stream_length.wrapping_add(length as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_concat_stream(
    mut dst: *mut pdf_obj,
    mut src: *mut pdf_obj,
) -> libc::c_int {
    let mut stream_data: *const libc::c_char = 0 as *const libc::c_char;
    let mut stream_length: libc::c_int = 0;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut filter: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut error: libc::c_int = 0i32;
    if !(!dst.is_null() && pdf_obj_typeof(dst) == 7i32)
        || !(!src.is_null() && pdf_obj_typeof(src) == 7i32)
    {
        _tt_abort(b"Invalid type.\x00" as *const u8 as *const libc::c_char);
    }
    stream_data = pdf_stream_dataptr(src) as *const libc::c_char;
    stream_length = pdf_stream_length(src);
    stream_dict = pdf_stream_dict(src);
    filter = pdf_lookup_dict(
        stream_dict,
        b"Filter\x00" as *const u8 as *const libc::c_char,
    );
    if filter.is_null() {
        pdf_add_stream(dst, stream_data as *const libc::c_void, stream_length);
    }
    /* HAVE_ZLIB */
    return error;
}
unsafe extern "C" fn pdf_stream_uncompress(mut src: *mut pdf_obj) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = pdf_new_stream(0i32);
    if src.is_null() || (*src).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !src.is_null() { (*src).type_0 } else { -1i32 },
            7i32,
            2420i32,
        );
    }
    pdf_merge_dict(pdf_stream_dict(dst), pdf_stream_dict(src));
    pdf_remove_dict(
        pdf_stream_dict(dst),
        b"Length\x00" as *const u8 as *const libc::c_char,
    );
    pdf_concat_stream(dst, src);
    return dst;
}
unsafe extern "C" fn pdf_write_obj(mut object: *mut pdf_obj, mut handle: rust_output_handle_t) {
    if object.is_null() {
        write_null(handle);
        return;
    }
    if object.is_null()
        || (*object).type_0 <= 0i32
        || (*object).type_0 > 10i32
        || !object.is_null() && pdf_obj_typeof(object) == 10i32
    {
        _tt_abort(
            b"pdf_write_obj: Invalid object, type = %d\n\x00" as *const u8 as *const libc::c_char,
            (*object).type_0,
        );
    }
    match (*object).type_0 {
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
    let mut length: libc::c_int = 0;
    /*
     * Record file position
     */
    add_xref_entry(
        (*object).label,
        1i32 as libc::c_uchar,
        pdf_output_file_position as libc::c_uint,
        (*object).generation,
    );
    length = sprintf(
        format_buffer.as_mut_ptr(),
        b"%u %hu obj\n\x00" as *const u8 as *const libc::c_char,
        (*object).label,
        (*object).generation as libc::c_int,
    );
    enc_mode = doc_enc_mode as libc::c_int != 0 && (*object).flags & 1i32 << 1i32 == 0;
    pdf_enc_set_label((*object).label);
    pdf_enc_set_generation((*object).generation as libc::c_uint);
    pdf_out(
        handle,
        format_buffer.as_mut_ptr() as *const libc::c_void,
        length,
    );
    pdf_write_obj(object, handle);
    pdf_out(
        handle,
        b"\nendobj\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8i32,
    );
}
unsafe extern "C" fn pdf_add_objstm(
    mut objstm: *mut pdf_obj,
    mut object: *mut pdf_obj,
) -> libc::c_int {
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pos: libc::c_int = 0;
    if objstm.is_null() || (*objstm).type_0 != 7i32 {
        _tt_abort(
            b"typecheck: Invalid object type: %d %d (line %d)\x00" as *const u8
                as *const libc::c_char,
            if !objstm.is_null() {
                (*objstm).type_0
            } else {
                -1i32
            },
            7i32,
            2497i32,
        );
    }
    data = get_objstm_data(objstm);
    let ref mut fresh15 = *data.offset(0);
    *fresh15 += 1;
    pos = *fresh15;
    *data.offset((2i32 * pos) as isize) = (*object).label as libc::c_int;
    *data.offset((2i32 * pos + 1i32) as isize) = pdf_stream_length(objstm);
    add_xref_entry(
        (*object).label,
        2i32 as libc::c_uchar,
        (*objstm).label,
        (pos - 1i32) as libc::c_ushort,
    );
    /* redirect output into objstm */
    output_stream = objstm;
    enc_mode = 0i32 != 0;
    pdf_write_obj(object, pdf_output_handle);
    pdf_out_char(pdf_output_handle, '\n' as i32 as libc::c_char);
    output_stream = 0 as *mut pdf_obj;
    return pos;
}
unsafe extern "C" fn release_objstm(mut objstm: *mut pdf_obj) {
    let mut data: *mut libc::c_int = get_objstm_data(objstm);
    let mut pos: libc::c_int = *data.offset(0);
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream: *mut pdf_stream = 0 as *mut pdf_stream;
    let mut old_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut old_length: libc::c_uint = 0;
    stream = (*objstm).data as *mut pdf_stream;
    /* Precede stream data by offset table */
    old_buf = (*stream).stream;
    old_length = (*stream).stream_length;
    /* Reserve 22 bytes for each entry (two 10 digit numbers plus two spaces) */
    (*stream).stream = new(
        (old_length.wrapping_add((22i32 * pos) as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32,
    ) as *mut libc::c_uchar;
    (*stream).stream_length = 0i32 as libc::c_uint;
    let mut i: libc::c_int = 2i32 * pos;
    let mut val: *mut libc::c_int = data.offset(2);
    loop {
        let fresh16 = i;
        i = i - 1;
        if !(fresh16 != 0) {
            break;
        }
        let fresh17 = val;
        val = val.offset(1);
        let mut length: libc::c_int = sprintf(
            format_buffer.as_mut_ptr(),
            b"%d \x00" as *const u8 as *const libc::c_char,
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
        pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"ObjStm\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"N\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(pos as libc::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"First\x00" as *const u8 as *const libc::c_char),
        pdf_new_number((*stream).stream_length as libc::c_double),
    );
    pdf_add_stream(
        objstm,
        old_buf as *const libc::c_void,
        old_length as libc::c_int,
    );
    free(old_buf as *mut libc::c_void);
    pdf_release_obj(objstm);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_release_obj(mut object: *mut pdf_obj) {
    if object.is_null() {
        return;
    }
    if object.is_null()
        || (*object).type_0 <= 0i32
        || (*object).type_0 > 10i32
        || (*object).refcount <= 0i32 as libc::c_uint
    {
        dpx_message(
            b"\npdf_release_obj: object=%p, type=%d, refcount=%d\n\x00" as *const u8
                as *const libc::c_char,
            object,
            (*object).type_0,
            (*object).refcount,
        );
        pdf_write_obj(object, ttstub_output_open_stdout());
        _tt_abort(
            b"pdf_release_obj:  Called with invalid object.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    (*object).refcount = (*object).refcount.wrapping_sub(1i32 as libc::c_uint);
    if (*object).refcount == 0i32 as libc::c_uint {
        /*
         * Nothing is using this object so it's okay to remove it.
         * Nonzero "label" means object needs to be written before it's destroyed.
         */
        if (*object).label != 0 && !pdf_output_handle.is_null() {
            if do_objstm == 0
                || (*object).flags & 1i32 << 0i32 != 0
                || doc_enc_mode as libc::c_int != 0 && (*object).flags & 1i32 << 1i32 != 0
                || (*object).generation as libc::c_int != 0
            {
                pdf_flush_obj(object, pdf_output_handle);
            } else {
                if current_objstm.is_null() {
                    let mut data: *mut libc::c_int =
                        new(((2i32 * 200i32 + 2i32) as u32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            as u32) as *mut libc::c_int;
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
        match (*object).type_0 {
            1 => {
                release_boolean((*object).data as *mut pdf_obj);
            }
            2 => {
                release_number((*object).data as *mut pdf_number);
            }
            3 => {
                release_string((*object).data as *mut pdf_string);
            }
            4 => {
                release_name((*object).data as *mut pdf_name);
            }
            5 => {
                release_array((*object).data as *mut pdf_array);
            }
            6 => {
                release_dict((*object).data as *mut pdf_dict);
            }
            7 => {
                release_stream((*object).data as *mut pdf_stream);
            }
            9 => {
                release_indirect((*object).data as *mut pdf_indirect);
            }
            8 | _ => {}
        }
        /* This might help detect freeing already freed objects */
        (*object).type_0 = -1i32;
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
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0i32;
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
        *buf.offset(fresh19 as isize) = c as libc::c_char
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
    return len;
}
unsafe extern "C" fn backup_line(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut ch: libc::c_int = -1i32;
    /* Note: this code should work even if \r\n is eol. It could fail on a
     * machine where \n is eol and there is a \r in the stream --- Highly
     * unlikely in the last few bytes where this is likely to be used.
     */
    if ttstub_input_seek(handle, 0i32 as ssize_t, 1i32) > 1i32 as libc::c_ulong {
        loop {
            ttstub_input_seek(handle, -2i32 as ssize_t, 1i32);
            if !(ttstub_input_seek(handle, 0i32 as ssize_t, 1i32) > 0i32 as libc::c_ulong
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
    return 1i32;
}
unsafe extern "C" fn find_xref(
    mut handle: rust_input_handle_t,
    mut file_size: libc::c_int,
) -> libc::c_int {
    let mut xref_pos: libc::c_int = 0i32;
    let mut len: libc::c_int = 0;
    let mut tries: libc::c_int = 10i32;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut number: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        let mut currentpos: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        if backup_line(handle) == 0 {
            tries = 0i32;
            break;
        } else {
            currentpos = ttstub_input_seek(handle, 0i32 as ssize_t, 1i32) as libc::c_int;
            n = (if strlen(b"startxref\x00" as *const u8 as *const libc::c_char)
                < (file_size - currentpos) as libc::c_ulong
            {
                strlen(b"startxref\x00" as *const u8 as *const libc::c_char)
            } else {
                (file_size - currentpos) as libc::c_ulong
            }) as libc::c_int;
            ttstub_input_read(handle, work_buffer.as_mut_ptr(), n as size_t);
            ttstub_input_seek(handle, currentpos as ssize_t, 0i32);
            tries -= 1;
            if !(tries > 0i32
                && strstartswith(
                    work_buffer.as_mut_ptr(),
                    b"startxref\x00" as *const u8 as *const libc::c_char,
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
        dpx_warning(
            b"Reading xref location data failed... Not a PDF file?\x00" as *const u8
                as *const libc::c_char,
        );
        return 0i32;
    }
    start = work_buffer.as_mut_ptr();
    end = start.offset(len as isize);
    skip_white(&mut start, end);
    number = parse_number(&mut start, end);
    xref_pos = atof(number) as libc::c_int;
    free(number as *mut libc::c_void);
    return xref_pos;
}
/*
 * This routine must be called with the file pointer located
 * at the start of the trailer.
 */
unsafe extern "C" fn parse_trailer(mut pf: *mut pdf_file) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cur_pos: libc::c_int = 0;
    let mut nmax: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    /*
     * Fill work_buffer and hope trailer fits. This should
     * be made a bit more robust sometime.
     */
    cur_pos = ttstub_input_seek((*pf).handle, 0i32 as ssize_t, 1i32) as libc::c_int;
    nmax = if (*pf).file_size - cur_pos < 1024i32 {
        (*pf).file_size - cur_pos
    } else {
        1024i32
    };
    nread =
        ttstub_input_read((*pf).handle, work_buffer.as_mut_ptr(), nmax as size_t) as libc::c_int;
    if nread == 0i32
        || strstartswith(
            work_buffer.as_mut_ptr(),
            b"trailer\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
    {
        dpx_warning(
            b"No trailer.  Are you sure this is a PDF file?\x00" as *const u8
                as *const libc::c_char,
        );
        dpx_warning(
            b"buffer:\n->%s<-\n\x00" as *const u8 as *const libc::c_char,
            work_buffer.as_mut_ptr(),
        );
        result = 0 as *mut pdf_obj
    } else {
        let mut p: *const libc::c_char = work_buffer
            .as_mut_ptr()
            .offset(strlen(b"trailer\x00" as *const u8 as *const libc::c_char) as isize);
        skip_white(&mut p, work_buffer.as_mut_ptr().offset(nread as isize));
        result = parse_pdf_dict(&mut p, work_buffer.as_mut_ptr().offset(nread as isize), pf)
    }
    return result;
}
/*
 * This routine tries to estimate an upper bound for character position
 * of the end of the object, so it knows how big the buffer must be.
 * The parsing routines require that the entire object be read into
 * memory. It would be a major pain to rewrite them.  The worst case
 * is that an object before an xref table will grab the whole table
 * :-(
 */
unsafe extern "C" fn next_object_offset(
    mut pf: *mut pdf_file,
    mut obj_num: libc::c_uint,
) -> libc::c_int {
    let mut next: libc::c_int = (*pf).file_size; /* Worst case */
    let mut i: libc::c_int = 0;
    let mut curr: libc::c_int = 0;
    curr = (*(*pf).xref_table.offset(obj_num as isize)).field2 as libc::c_int;
    /* Check all other type 1 objects to find next one */
    i = 0i32;
    while i < (*pf).num_obj {
        if (*(*pf).xref_table.offset(i as isize)).type_0 as libc::c_int == 1i32
            && (*(*pf).xref_table.offset(i as isize)).field2 > curr as libc::c_uint
            && (*(*pf).xref_table.offset(i as isize)).field2 < next as libc::c_uint
        {
            next = (*(*pf).xref_table.offset(i as isize)).field2 as libc::c_int
        }
        i += 1
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_indirect(
    mut pf: *mut pdf_file,
    mut obj_num: libc::c_uint,
    mut obj_gen: libc::c_ushort,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut indirect: *mut pdf_indirect = 0 as *mut pdf_indirect;
    indirect = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_indirect>() as libc::c_ulong)
        as u32) as *mut pdf_indirect;
    (*indirect).pf = pf;
    (*indirect).obj = 0 as *mut pdf_obj;
    (*indirect).label = obj_num;
    (*indirect).generation = obj_gen;
    result = pdf_new_obj(9i32);
    (*result).data = indirect as *mut libc::c_void;
    return result;
}
unsafe extern "C" fn pdf_read_object(
    mut obj_num: libc::c_uint,
    mut obj_gen: libc::c_ushort,
    mut pf: *mut pdf_file,
    mut offset: libc::c_int,
    mut limit: libc::c_int,
) -> *mut pdf_obj {
    let mut length: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    length = limit - offset;
    if length <= 0i32 {
        return 0 as *mut pdf_obj;
    }
    buffer = new(((length + 1i32) as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as u32) as *mut libc::c_char;
    ttstub_input_seek((*pf).handle, offset as ssize_t, 0i32);
    ttstub_input_read((*pf).handle, buffer, length as size_t);
    p = buffer;
    endptr = p.offset(length as isize);
    /* Check for obj_num and obj_gen */
    let mut q: *const libc::c_char = p; /* <== p */
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_uint = 0;
    let mut g: libc::c_uint = 0;
    skip_white(&mut q, endptr);
    sp = parse_unsigned(&mut q, endptr);
    if sp.is_null() {
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    n = strtoul(sp, 0 as *mut *mut libc::c_char, 10i32) as libc::c_uint;
    free(sp as *mut libc::c_void);
    skip_white(&mut q, endptr);
    sp = parse_unsigned(&mut q, endptr);
    if sp.is_null() {
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    g = strtoul(sp, 0 as *mut *mut libc::c_char, 10i32) as libc::c_uint;
    free(sp as *mut libc::c_void);
    if obj_num != 0 && (n != obj_num || g != obj_gen as libc::c_uint) {
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    p = q;
    skip_white(&mut p, endptr);
    if memcmp(
        p as *const libc::c_void,
        b"obj\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"obj\x00" as *const u8 as *const libc::c_char),
    ) != 0
    {
        dpx_warning(b"Didn\'t find \"obj\".\x00" as *const u8 as *const libc::c_char);
        free(buffer as *mut libc::c_void);
        return 0 as *mut pdf_obj;
    }
    p = p.offset(strlen(b"obj\x00" as *const u8 as *const libc::c_char) as isize);
    result = parse_pdf_object(&mut p, endptr, pf);
    skip_white(&mut p, endptr);
    if memcmp(
        p as *const libc::c_void,
        b"endobj\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"endobj\x00" as *const u8 as *const libc::c_char),
    ) != 0
    {
        dpx_warning(b"Didn\'t find \"endobj\".\x00" as *const u8 as *const libc::c_char);
        pdf_release_obj(result);
        result = 0 as *mut pdf_obj
    }
    free(buffer as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn read_objstm(mut pf: *mut pdf_file, mut num: libc::c_uint) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut offset: libc::c_uint = (*(*pf).xref_table.offset(num as isize)).field2;
    let mut gen: libc::c_ushort = (*(*pf).xref_table.offset(num as isize)).field3;
    let mut limit: libc::c_int = next_object_offset(pf, num);
    let mut n: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut header: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut objstm: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut type_0: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut n_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut first_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    objstm = pdf_read_object(num, gen, pf, offset as libc::c_int, limit);
    if !objstm.is_null() && pdf_obj_typeof(objstm) == 7i32 {
        let mut tmp: *mut pdf_obj = pdf_stream_uncompress(objstm);
        if !tmp.is_null() {
            pdf_release_obj(objstm);
            objstm = tmp;
            dict = pdf_stream_dict(objstm);
            type_0 = pdf_lookup_dict(dict, b"Type\x00" as *const u8 as *const libc::c_char);
            if !(!(!type_0.is_null() && pdf_obj_typeof(type_0) == 4i32)
                || strcmp(
                    pdf_name_value(type_0),
                    b"ObjStm\x00" as *const u8 as *const libc::c_char,
                ) != 0)
            {
                n_obj = pdf_lookup_dict(dict, b"N\x00" as *const u8 as *const libc::c_char);
                if !n_obj.is_null() && pdf_obj_typeof(n_obj) == 2i32 {
                    n = pdf_number_value(n_obj) as libc::c_int;
                    first_obj =
                        pdf_lookup_dict(dict, b"First\x00" as *const u8 as *const libc::c_char);
                    if !first_obj.is_null() && pdf_obj_typeof(first_obj) == 2i32 {
                        first = pdf_number_value(first_obj) as libc::c_int;
                        /* reject object streams without object data */
                        if !(first >= pdf_stream_length(objstm)) {
                            header = new(((2i32 * (n + 1i32)) as u32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                as u32)
                                as *mut libc::c_int;
                            set_objstm_data(objstm, header);
                            let fresh20 = header;
                            header = header.offset(1);
                            *fresh20 = n;
                            let fresh21 = header;
                            header = header.offset(1);
                            *fresh21 = first;
                            /* avoid parsing beyond offset table */
                            data =
                                new(((first + 1i32) as u32 as libc::c_ulong).wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as u32) as *mut libc::c_char;
                            memcpy(
                                data as *mut libc::c_void,
                                pdf_stream_dataptr(objstm),
                                first as libc::c_ulong,
                            );
                            *data.offset(first as isize) = 0i32 as libc::c_char;
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
                                *fresh23 = strtoul(p, &mut q, 10i32) as libc::c_int;
                                if q == p as *mut libc::c_char {
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
    dpx_warning(b"Cannot parse object stream.\x00" as *const u8 as *const libc::c_char);
    free(data as *mut libc::c_void);
    pdf_release_obj(objstm);
    return 0 as *mut pdf_obj;
}
/* Label without corresponding object definition are replaced by the
 * null object, as required by the PDF spec. This is important to parse
 * several cross-reference sections.
 */
unsafe extern "C" fn pdf_get_object(
    mut pf: *mut pdf_file,
    mut obj_num: libc::c_uint,
    mut obj_gen: libc::c_ushort,
) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    if !(obj_num > 0i32 as libc::c_uint
        && obj_num < (*pf).num_obj as libc::c_uint
        && ((*(*pf).xref_table.offset(obj_num as isize)).type_0 as libc::c_int == 1i32
            && (*(*pf).xref_table.offset(obj_num as isize)).field3 as libc::c_int
                == obj_gen as libc::c_int
            || (*(*pf).xref_table.offset(obj_num as isize)).type_0 as libc::c_int == 2i32
                && obj_gen == 0))
    {
        dpx_warning(
            b"Trying to read nonexistent or deleted object: %u %hu\x00" as *const u8
                as *const libc::c_char,
            obj_num,
            obj_gen as libc::c_int,
        );
        return pdf_new_null();
    }
    result = (*(*pf).xref_table.offset(obj_num as isize)).direct;
    if !result.is_null() {
        return pdf_link_obj(result);
    }
    if (*(*pf).xref_table.offset(obj_num as isize)).type_0 as libc::c_int == 1i32 {
        /* type == 1 */
        let mut offset: libc::c_uint = 0;
        let mut limit: libc::c_int = 0;
        offset = (*(*pf).xref_table.offset(obj_num as isize)).field2;
        limit = next_object_offset(pf, obj_num);
        result = pdf_read_object(obj_num, obj_gen, pf, offset as libc::c_int, limit)
    } else {
        /* type == 2 */
        let mut objstm_num: libc::c_uint = (*(*pf).xref_table.offset(obj_num as isize)).field2;
        let mut index: libc::c_ushort = (*(*pf).xref_table.offset(obj_num as isize)).field3;
        let mut objstm: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut n: libc::c_int = 0;
        let mut first: libc::c_int = 0;
        let mut length: libc::c_int = 0;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut q: *const libc::c_char = 0 as *const libc::c_char;
        if objstm_num >= (*pf).num_obj as libc::c_uint
            || (*(*pf).xref_table.offset(objstm_num as isize)).type_0 as libc::c_int != 1i32
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
            data = get_objstm_data(objstm);
            let fresh25 = data;
            data = data.offset(1);
            n = *fresh25;
            let fresh26 = data;
            data = data.offset(1);
            first = *fresh26;
            if index as libc::c_int >= n
                || *data.offset((2i32 * index as libc::c_int) as isize) as libc::c_uint != obj_num
            {
                current_block = 17536737673648832705;
            } else {
                length = pdf_stream_length(objstm);
                p = (pdf_stream_dataptr(objstm) as *const libc::c_char)
                    .offset(first as isize)
                    .offset(*data.offset((2i32 * index as libc::c_int + 1i32) as isize) as isize);
                q = p.offset(
                    (if index as libc::c_int == n - 1i32 {
                        length
                    } else {
                        first + *data.offset((2i32 * index as libc::c_int + 3i32) as isize)
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
                dpx_warning(
                    b"Could not read object from object stream.\x00" as *const u8
                        as *const libc::c_char,
                );
                return pdf_new_null();
            }
        }
    }
    /* Make sure the caller doesn't free this object */
    let ref mut fresh27 = (*(*pf).xref_table.offset(obj_num as isize)).direct;
    *fresh27 = pdf_link_obj(result);
    return result;
}
unsafe extern "C" fn pdf_new_ref(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    if (*object).label == 0i32 as libc::c_uint {
        pdf_label_obj(object);
    }
    result = pdf_new_indirect(0 as *mut pdf_file, (*object).label, (*object).generation);
    let ref mut fresh28 = (*((*result).data as *mut pdf_indirect)).obj;
    *fresh28 = object;
    return result;
}
/* pdf_deref_obj always returns a link instead of the original   */
/* It never return the null object, but the NULL pointer instead */
#[no_mangle]
pub unsafe extern "C" fn pdf_deref_obj(mut obj: *mut pdf_obj) -> *mut pdf_obj {
    let mut count: libc::c_int = 30i32;
    if !obj.is_null() {
        obj = pdf_link_obj(obj)
    }
    while !obj.is_null() && pdf_obj_typeof(obj) == 9i32 && {
        count -= 1;
        count != 0
    } {
        let mut pf: *mut pdf_file = (*((*obj).data as *mut pdf_indirect)).pf;
        if !pf.is_null() {
            let mut obj_num: libc::c_uint = (*((*obj).data as *mut pdf_indirect)).label;
            let mut obj_gen: libc::c_ushort = (*((*obj).data as *mut pdf_indirect)).generation;
            pdf_release_obj(obj);
            obj = pdf_get_object(pf, obj_num, obj_gen)
        } else {
            let mut next_obj: *mut pdf_obj = (*((*obj).data as *mut pdf_indirect)).obj;
            if next_obj.is_null() {
                _tt_abort(b"Undefined object reference\x00" as *const u8 as *const libc::c_char);
            }
            pdf_release_obj(obj);
            obj = pdf_link_obj(next_obj)
        }
    }
    if count == 0 {
        _tt_abort(
            b"Loop in object hierarchy detected. Broken PDF file?\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if !obj.is_null() && pdf_obj_typeof(obj) == 8i32 {
        pdf_release_obj(obj);
        return 0 as *mut pdf_obj;
    } else {
        return obj;
    };
}
unsafe extern "C" fn extend_xref(mut pf: *mut pdf_file, mut new_size: libc::c_int) {
    let mut i: libc::c_uint = 0;
    (*pf).xref_table = renew(
        (*pf).xref_table as *mut libc::c_void,
        (new_size as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<xref_entry>() as libc::c_ulong) as u32,
    ) as *mut xref_entry;
    i = (*pf).num_obj as libc::c_uint;
    while i < new_size as libc::c_uint {
        let ref mut fresh29 = (*(*pf).xref_table.offset(i as isize)).direct;
        *fresh29 = 0 as *mut pdf_obj;
        let ref mut fresh30 = (*(*pf).xref_table.offset(i as isize)).indirect;
        *fresh30 = 0 as *mut pdf_obj;
        (*(*pf).xref_table.offset(i as isize)).type_0 = 0i32 as libc::c_uchar;
        (*(*pf).xref_table.offset(i as isize)).field3 = 0i32 as libc::c_ushort;
        (*(*pf).xref_table.offset(i as isize)).field2 = 0i64 as libc::c_uint;
        i = i.wrapping_add(1)
    }
    (*pf).num_obj = new_size;
}
/* Returns < 0 for error, 1 for success, and 0 when xref stream found. */
unsafe extern "C" fn parse_xref_table(
    mut pf: *mut pdf_file,
    mut xref_pos: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char; /* See, PDF ref. v.1.7, p.91 for "255+1" here. */
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut len: libc::c_int = 0;
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
        dpx_warning(
            b"Something went wrong while reading xref table...giving up.\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    p = buf.as_mut_ptr();
    endptr = buf.as_mut_ptr().offset(len as isize);
    /* No skip_white() here. There should not be any white-spaces here. */
    if memcmp(
        p as *const libc::c_void,
        b"xref\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        strlen(b"xref\x00" as *const u8 as *const libc::c_char),
    ) != 0
    {
        /* Might be an xref stream and not an xref table */
        return 0i32;
    }
    p = p.offset(strlen(b"xref\x00" as *const u8 as *const libc::c_char) as isize);
    skip_white(&mut p, endptr);
    if p != endptr {
        dpx_warning(
            b"Garbage after \"xref\" keyword found.\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    loop
    /* Next line in file has first item and size of table */
    {
        let mut flag: libc::c_char = 0;
        let mut current_pos: libc::c_uint = 0;
        let mut i: libc::c_int = 0;
        let mut first: u32 = 0;
        let mut size: u32 = 0;
        let mut offset: u32 = 0;
        let mut obj_gen: u32 = 0;
        current_pos = ttstub_input_seek((*pf).handle, 0i32 as ssize_t, 1i32) as libc::c_uint;
        len = tt_mfreadln(buf.as_mut_ptr(), 255i32, (*pf).handle);
        if !(len == 0i32) {
            if len < 0i32 {
                dpx_warning(
                    b"Reading a line failed in xref table.\x00" as *const u8 as *const libc::c_char,
                );
                return -1i32;
            }
            p = buf.as_mut_ptr();
            endptr = buf.as_mut_ptr().offset(len as isize);
            skip_white(&mut p, endptr);
            if !(p == endptr) {
                if !strstartswith(p, b"trailer\x00" as *const u8 as *const libc::c_char).is_null() {
                    /* Backup... This is ugly, but it seems like the safest thing to
                     * do. It is possible the trailer dictionary starts on the same
                     * logical line as the word trailer. In that case, the mfgets call
                     * might have started to read the trailer dictionary and
                     * parse_trailer would fail.
                     */
                    current_pos = (current_pos as libc::c_long
                        + p.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long)
                        as libc::c_uint; /* Jump to the beginning of "trailer" keyword. */
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
                    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
                    /* Object number of the first object whithin this xref subsection. */
                    q = parse_unsigned(&mut p, endptr);
                    if q.is_null() {
                        dpx_warning(
                            b"An unsigned integer expected but could not find. (xref)\x00"
                                as *const u8 as *const libc::c_char,
                        );
                        return -1i32;
                    }
                    first = atoi(q) as u32;
                    free(q as *mut libc::c_void);
                    skip_white(&mut p, endptr);
                    /* Nnumber of objects in this xref subsection. */
                    q = parse_unsigned(&mut p, endptr);
                    if q.is_null() {
                        dpx_warning(
                            b"An unsigned integer expected but could not find. (xref)\x00"
                                as *const u8 as *const libc::c_char,
                        );
                        return -1i32;
                    }
                    size = atoi(q) as u32;
                    free(q as *mut libc::c_void);
                    skip_white(&mut p, endptr);
                    /* Check for unrecognized tokens */
                    if p != endptr {
                        dpx_warning(
                            b"Unexpected token found in xref table.\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return -1i32;
                    }
                    /* The first line of a xref subsection OK. */
                    if ((*pf).num_obj as libc::c_uint) < first.wrapping_add(size) {
                        extend_xref(pf, first.wrapping_add(size) as libc::c_int);
                    }
                    /* Start parsing xref subsection body... */
                    i = first as libc::c_int;
                    /* Only white-spaces and/or comment. */
                    while (i as libc::c_uint) < first.wrapping_add(size) {
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
                                dpx_warning(
                                    b"Something went wrong while reading xref subsection...\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
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
                            obj_gen = 0i32 as u32;
                            flag = 0i32 as libc::c_char;
                            let mut q_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            /* Offset value -- 10 digits (0 padded) */
                            q_0 = parse_unsigned(&mut p, endptr);
                            if q_0.is_null() {
                                dpx_warning(
                                    b"An unsigned integer expected but could not find. (xref)\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                return -1i32;
                            } else {
                                if strlen(q_0) != 10i32 as libc::c_ulong {
                                    /* exactly 10 digits */
                                    dpx_warning(
                                        b"Offset must be a 10 digits number. (xref)\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
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
                                dpx_warning(
                                    b"An unsigned integer expected but could not find. (xref)\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                return -1i32;
                            } else {
                                if strlen(q_0) != 5i32 as libc::c_ulong {
                                    /* exactly 5 digits */
                                    dpx_warning(
                                        b"Expecting a 5 digits number. (xref)\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    free(q_0 as *mut libc::c_void);
                                    return -1i32;
                                }
                            }
                            obj_gen = atoi(q_0) as u32;
                            free(q_0 as *mut libc::c_void);
                            skip_white(&mut p, endptr);
                            if p == endptr {
                                dpx_warning(b"Unexpected EOL reached while reading a xref subsection entry.\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                return -1i32;
                            }
                            /* Flag -- a char */
                            flag = *p;
                            p = p.offset(1);
                            skip_white(&mut p, endptr);
                            if p < endptr {
                                dpx_warning(
                                    b"Garbage in xref subsection entry found...\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                return -1i32;
                            } else {
                                if flag as libc::c_int != 'n' as i32
                                    && flag as libc::c_int != 'f' as i32
                                    || flag as libc::c_int == 'n' as i32
                                        && (offset >= (*pf).file_size as libc::c_uint
                                            || offset > 0i32 as libc::c_uint
                                                && offset < 4i32 as libc::c_uint)
                                {
                                    dpx_warning(
                                        b"Invalid xref table entry [%u]. PDF file is corrupt...\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                        i,
                                    );
                                    return -1i32;
                                }
                            }
                            /* Everything seems to be OK. */
                            if (*(*pf).xref_table.offset(i as isize)).field2 == 0 {
                                (*(*pf).xref_table.offset(i as isize)).type_0 =
                                    (flag as libc::c_int == 'n' as i32) as libc::c_int
                                        as libc::c_uchar; /* TODO: change! why? */
                                (*(*pf).xref_table.offset(i as isize)).field2 = offset;
                                (*(*pf).xref_table.offset(i as isize)).field3 =
                                    obj_gen as libc::c_ushort
                            }
                            i += 1
                        }
                    }
                }
            }
        }
    }
    return 1i32;
}
unsafe extern "C" fn parse_xrefstm_field(
    mut p: *mut *const libc::c_char,
    mut length: libc::c_int,
    mut def: libc::c_uint,
) -> libc::c_uint {
    let mut val: libc::c_uint = 0i32 as libc::c_uint;
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
        val |= *fresh32 as libc::c_uchar as libc::c_uint
    }
    return val;
}
unsafe extern "C" fn parse_xrefstm_subsec(
    mut pf: *mut pdf_file,
    mut p: *mut *const libc::c_char,
    mut length: *mut libc::c_int,
    mut W: *mut libc::c_int,
    mut wsum: libc::c_int,
    mut first: libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut e: *mut xref_entry = 0 as *mut xref_entry;
    *length -= wsum * size;
    if *length < 0i32 {
        return -1i32;
    }
    if (*pf).num_obj < first + size {
        extend_xref(pf, first + size);
    }
    e = (*pf).xref_table.offset(first as isize);
    loop {
        let fresh33 = size;
        size = size - 1;
        if !(fresh33 != 0) {
            break;
        }
        let mut type_0: libc::c_uchar = 0;
        let mut field2: libc::c_uint = 0;
        let mut field3: libc::c_ushort = 0;
        type_0 = parse_xrefstm_field(p, *W.offset(0), 1i32 as libc::c_uint) as libc::c_uchar;
        if type_0 as libc::c_int > 2i32 {
            dpx_warning(
                b"Unknown cross-reference stream entry type.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        field2 = parse_xrefstm_field(p, *W.offset(1), 0i32 as libc::c_uint);
        field3 = parse_xrefstm_field(p, *W.offset(2), 0i32 as libc::c_uint) as libc::c_ushort;
        if (*e).field2 == 0 {
            (*e).type_0 = type_0;
            (*e).field2 = field2;
            (*e).field3 = field3
        }
        e = e.offset(1)
    }
    return 0i32;
}
unsafe extern "C" fn parse_xref_stream(
    mut pf: *mut pdf_file,
    mut xref_pos: libc::c_int,
    mut trailer: *mut *mut pdf_obj,
) -> libc::c_int {
    let mut current_block: u64;
    let mut xrefstm: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut size_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut W_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut index_obj: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut size: libc::c_uint = 0;
    let mut length: libc::c_int = 0;
    let mut W: [libc::c_int; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut wsum: libc::c_int = 0i32;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    xrefstm = pdf_read_object(
        0i32 as libc::c_uint,
        0i32 as libc::c_ushort,
        pf,
        xref_pos,
        (*pf).file_size,
    );
    if !xrefstm.is_null() && pdf_obj_typeof(xrefstm) == 7i32 {
        let mut tmp: *mut pdf_obj = pdf_stream_uncompress(xrefstm);
        if !tmp.is_null() {
            pdf_release_obj(xrefstm);
            xrefstm = tmp;
            *trailer = pdf_link_obj(pdf_stream_dict(xrefstm));
            size_obj = pdf_lookup_dict(*trailer, b"Size\x00" as *const u8 as *const libc::c_char);
            if !size_obj.is_null() && pdf_obj_typeof(size_obj) == 2i32 {
                size = pdf_number_value(size_obj) as libc::c_uint;
                length = pdf_stream_length(xrefstm);
                W_obj = pdf_lookup_dict(*trailer, b"W\x00" as *const u8 as *const libc::c_char);
                if !(!(!W_obj.is_null() && pdf_obj_typeof(W_obj) == 5i32)
                    || pdf_array_length(W_obj) != 3i32 as libc::c_uint)
                {
                    i = 0i32;
                    loop {
                        if !(i < 3i32) {
                            current_block = 12147880666119273379;
                            break;
                        }
                        let mut tmp_0: *mut pdf_obj = pdf_get_array(W_obj, i);
                        if !(!tmp_0.is_null() && pdf_obj_typeof(tmp_0) == 2i32) {
                            current_block = 5131529843719913080;
                            break;
                        }
                        W[i as usize] = pdf_number_value(tmp_0) as libc::c_int;
                        wsum += W[i as usize];
                        i += 1
                    }
                    match current_block {
                        5131529843719913080 => {}
                        _ => {
                            p = pdf_stream_dataptr(xrefstm) as *const libc::c_char;
                            index_obj = pdf_lookup_dict(
                                *trailer,
                                b"Index\x00" as *const u8 as *const libc::c_char,
                            );
                            if !index_obj.is_null() {
                                let mut index_len: libc::c_uint = 0;
                                if !(!index_obj.is_null() && pdf_obj_typeof(index_obj) == 5i32) || {
                                    index_len = pdf_array_length(index_obj);
                                    index_len.wrapping_rem(2i32 as libc::c_uint) != 0
                                } {
                                    current_block = 5131529843719913080;
                                } else {
                                    i = 0i32;
                                    loop {
                                        if !((i as libc::c_uint) < index_len) {
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
                                        if !(!first.is_null() && pdf_obj_typeof(first) == 2i32)
                                            || !(!size_obj.is_null()
                                                && pdf_obj_typeof(size_obj) == 2i32)
                                            || parse_xrefstm_subsec(
                                                pf,
                                                &mut p,
                                                &mut length,
                                                W.as_mut_ptr(),
                                                wsum,
                                                pdf_number_value(first) as libc::c_int,
                                                pdf_number_value(size_obj) as libc::c_int,
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
                                size as libc::c_int,
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
                                        dpx_warning(
                                            b"Garbage in xref stream.\x00" as *const u8
                                                as *const libc::c_char,
                                        );
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
    dpx_warning(b"Cannot parse cross-reference stream.\x00" as *const u8 as *const libc::c_char);
    pdf_release_obj(xrefstm);
    if !(*trailer).is_null() {
        pdf_release_obj(*trailer);
        *trailer = 0 as *mut pdf_obj
    }
    return 0i32;
}
/* TODO: parse Version entry */
unsafe extern "C" fn read_xref(mut pf: *mut pdf_file) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut trailer: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut main_trailer: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut xref_pos: libc::c_int = 0;
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
                    let mut res: libc::c_int = parse_xref_table(pf, xref_pos);
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
                        xrefstm = pdf_lookup_dict(
                            trailer,
                            b"XRefStm\x00" as *const u8 as *const libc::c_char,
                        );
                        if !xrefstm.is_null() {
                            let mut new_trailer: *mut pdf_obj = 0 as *mut pdf_obj;
                            if !xrefstm.is_null()
                                && pdf_obj_typeof(xrefstm) == 2i32
                                && parse_xref_stream(
                                    pf,
                                    pdf_number_value(xrefstm) as libc::c_int,
                                    &mut new_trailer,
                                ) != 0
                            {
                                pdf_release_obj(new_trailer);
                            } else {
                                dpx_warning(
                                    b"Skipping hybrid reference section.\x00" as *const u8
                                        as *const libc::c_char,
                                );
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
                    prev =
                        pdf_lookup_dict(trailer, b"Prev\x00" as *const u8 as *const libc::c_char);
                    if !prev.is_null() {
                        if !(!prev.is_null() && pdf_obj_typeof(prev) == 2i32) {
                            current_block = 13794981049891343809;
                            continue;
                        }
                        xref_pos = pdf_number_value(prev) as libc::c_int
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
                dpx_warning(
                    b"Error while parsing PDF file.\x00" as *const u8 as *const libc::c_char,
                );
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
    if !handle.is_null() {
    } else {
        __assert_fail(
            b"handle\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3507i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"pdf_file *pdf_file_new(rust_input_handle_t)\x00",
            ))
            .as_ptr(),
        );
    }
    pf = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<pdf_file>() as libc::c_ulong) as u32)
        as *mut pdf_file;
    (*pf).handle = handle;
    (*pf).trailer = 0 as *mut pdf_obj;
    (*pf).xref_table = 0 as *mut xref_entry;
    (*pf).catalog = 0 as *mut pdf_obj;
    (*pf).num_obj = 0i32;
    (*pf).version = 0i32 as libc::c_uint;
    (*pf).file_size = ttstub_input_get_size(handle) as libc::c_int;
    ttstub_input_seek(handle, 0i32 as ssize_t, 2i32);
    return pf;
}
unsafe extern "C" fn pdf_file_free(mut pf: *mut pdf_file) {
    let mut i: libc::c_uint = 0;
    if pf.is_null() {
        return;
    }
    i = 0i32 as libc::c_uint;
    while i < (*pf).num_obj as libc::c_uint {
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
    pdf_files = new((1i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<ht_table>() as libc::c_ulong)
        as u32) as *mut ht_table;
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
pub unsafe extern "C" fn pdf_file_get_version(mut pf: *mut pdf_file) -> libc::c_uint {
    if !pf.is_null() {
    } else {
        __assert_fail(
            b"pf\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3554i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"unsigned int pdf_file_get_version(pdf_file *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*pf).version;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_file_get_trailer(mut pf: *mut pdf_file) -> *mut pdf_obj {
    if !pf.is_null() {
    } else {
        __assert_fail(
            b"pf\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3561i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"pdf_obj *pdf_file_get_trailer(pdf_file *)\x00",
            ))
            .as_ptr(),
        );
    }
    return pdf_link_obj((*pf).trailer);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_file_get_catalog(mut pf: *mut pdf_file) -> *mut pdf_obj {
    if !pf.is_null() {
    } else {
        __assert_fail(
            b"pf\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3568i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"pdf_obj *pdf_file_get_catalog(pdf_file *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*pf).catalog;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_open(
    mut ident: *const libc::c_char,
    mut handle: rust_input_handle_t,
) -> *mut pdf_file {
    let mut current_block: u64;
    let mut pf: *mut pdf_file = 0 as *mut pdf_file;
    if !pdf_files.is_null() {
    } else {
        __assert_fail(
            b"pdf_files\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3577i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"pdf_file *pdf_open(const char *, rust_input_handle_t)\x00",
            ))
            .as_ptr(),
        );
    }
    if !ident.is_null() {
        pf = ht_lookup_table(
            pdf_files,
            ident as *const libc::c_void,
            strlen(ident) as libc::c_int,
        ) as *mut pdf_file
    }
    if !pf.is_null() {
        (*pf).handle = handle
    } else {
        let mut new_version: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut version: libc::c_uint = 0i32 as libc::c_uint;
        let mut r: libc::c_int = parse_pdf_version(handle, &mut version);
        if r < 0i32 || version < 1i32 as libc::c_uint || version > pdf_version {
            dpx_warning(
                b"pdf_open: Not a PDF 1.[1-%u] file.\x00" as *const u8 as *const libc::c_char,
                pdf_version,
            );
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
        } else if !pdf_lookup_dict(
            (*pf).trailer,
            b"Encrypt\x00" as *const u8 as *const libc::c_char,
        )
        .is_null()
        {
            dpx_warning(b"PDF document is encrypted.\x00" as *const u8 as *const libc::c_char);
            current_block = 14455231216035570027;
        } else {
            (*pf).catalog = pdf_deref_obj(pdf_lookup_dict(
                (*pf).trailer,
                b"Root\x00" as *const u8 as *const libc::c_char,
            ));
            if !(!(*pf).catalog.is_null() && pdf_obj_typeof((*pf).catalog) == 6i32) {
                dpx_warning(
                    b"Cannot read PDF document catalog. Broken PDF file?\x00" as *const u8
                        as *const libc::c_char,
                );
                current_block = 14455231216035570027;
            } else {
                new_version = pdf_deref_obj(pdf_lookup_dict(
                    (*pf).catalog,
                    b"Version\x00" as *const u8 as *const libc::c_char,
                ));
                if !new_version.is_null() {
                    let mut minor: libc::c_uint = 0;
                    if !(!new_version.is_null() && pdf_obj_typeof(new_version) == 4i32)
                        || sscanf(
                            pdf_name_value(new_version),
                            b"1.%u\x00" as *const u8 as *const libc::c_char,
                            &mut minor as *mut libc::c_uint,
                        ) != 1i32
                    {
                        pdf_release_obj(new_version);
                        dpx_warning(
                            b"Illegal Version entry in document catalog. Broken PDF file?\x00"
                                as *const u8 as *const libc::c_char,
                        );
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
                                strlen(ident) as libc::c_int,
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
    return pf;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close(mut pf: *mut pdf_file) {
    if !pf.is_null() {
        (*pf).handle = 0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_files_close() {
    if !pdf_files.is_null() {
    } else {
        __assert_fail(
            b"pdf_files\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3653i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void pdf_files_close(void)\x00",
            ))
            .as_ptr(),
        );
    }
    ht_clear_table(pdf_files);
    free(pdf_files as *mut libc::c_void);
}
/* Internal static routines */
unsafe extern "C" fn parse_pdf_version(
    mut handle: rust_input_handle_t,
    mut ret_version: *mut libc::c_uint,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 10] = *::std::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
    );
    let mut minor: libc::c_uint = 0;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(
        handle,
        buffer.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    ) as libc::c_ulong
        != (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
    {
        return -1i32;
    }
    if sscanf(
        buffer.as_mut_ptr(),
        b"%%PDF-1.%u\x00" as *const u8 as *const libc::c_char,
        &mut minor as *mut libc::c_uint,
    ) != 1i32
    {
        return -1i32;
    }
    *ret_version = minor;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn check_for_pdf(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut version: libc::c_uint = 0;
    r = parse_pdf_version(handle, &mut version);
    if r < 0i32 {
        /* not a PDF file */
        return 0i32;
    }
    if version <= pdf_version {
        return 1i32;
    }
    dpx_warning(
        b"Version of PDF file (1.%d) is newer than version limit specification.\x00" as *const u8
            as *const libc::c_char,
        version,
    );
    return 1i32;
}
#[inline]
unsafe extern "C" fn import_dict(
    mut key: *mut pdf_obj,
    mut value: *mut pdf_obj,
    mut pdata: *mut libc::c_void,
) -> libc::c_int {
    let mut copy: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
    copy = pdata as *mut pdf_obj;
    tmp = pdf_import_object(value);
    if tmp.is_null() {
        return -1i32;
    }
    pdf_add_dict(copy, pdf_link_obj(key), tmp);
    return 0i32;
}
static mut loop_marker: pdf_obj = {
    let mut init = pdf_obj {
        type_0: 0i32,
        label: 0i32 as libc::c_uint,
        generation: 0i32 as libc::c_ushort,
        refcount: 0i32 as libc::c_uint,
        flags: 0i32,
        data: 0 as *const libc::c_void as *mut libc::c_void,
    };
    init
};
unsafe extern "C" fn pdf_import_indirect(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut pf: *mut pdf_file = (*((*object).data as *mut pdf_indirect)).pf;
    let mut obj_num: libc::c_uint = (*((*object).data as *mut pdf_indirect)).label;
    let mut obj_gen: libc::c_ushort = (*((*object).data as *mut pdf_indirect)).generation;
    let mut ref_0: *mut pdf_obj = 0 as *mut pdf_obj;
    if !pf.is_null() {
    } else {
        __assert_fail(
            b"pf\x00" as *const u8 as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3721i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"pdf_obj *pdf_import_indirect(pdf_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(obj_num > 0i32 as libc::c_uint
        && obj_num < (*pf).num_obj as libc::c_uint
        && ((*(*pf).xref_table.offset(obj_num as isize)).type_0 as libc::c_int == 1i32
            && (*(*pf).xref_table.offset(obj_num as isize)).field3 as libc::c_int
                == obj_gen as libc::c_int
            || (*(*pf).xref_table.offset(obj_num as isize)).type_0 as libc::c_int == 2i32
                && obj_gen == 0))
    {
        dpx_warning(
            b"Can\'t resolve object: %u %u\x00" as *const u8 as *const libc::c_char,
            obj_num,
            obj_gen as libc::c_int,
        );
        return pdf_new_null();
    }
    ref_0 = (*(*pf).xref_table.offset(obj_num as isize)).indirect;
    if !ref_0.is_null() {
        if ref_0 == &mut loop_marker as *mut pdf_obj {
            _tt_abort(
                b"Loop in object hierarchy detected. Broken PDF file?\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return pdf_link_obj(ref_0);
    } else {
        let mut obj: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut tmp: *mut pdf_obj = 0 as *mut pdf_obj;
        obj = pdf_get_object(pf, obj_num, obj_gen);
        if obj.is_null() {
            dpx_warning(
                b"Could not read object: %u %u\x00" as *const u8 as *const libc::c_char,
                obj_num,
                obj_gen as libc::c_int,
            );
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
    let mut i: libc::c_uint = 0;
    match pdf_obj_typeof(object) {
        9 => {
            if !(*((*object).data as *mut pdf_indirect)).pf.is_null() {
                imported = pdf_import_indirect(object)
            } else {
                imported = pdf_link_obj(object)
            }
        }
        7 => {
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
        6 => {
            imported = pdf_new_dict();
            if pdf_foreach_dict(
                object,
                Some(
                    import_dict
                        as unsafe extern "C" fn(
                            _: *mut pdf_obj,
                            _: *mut pdf_obj,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                imported as *mut libc::c_void,
            ) < 0i32
            {
                pdf_release_obj(imported);
                return 0 as *mut pdf_obj;
            }
        }
        5 => {
            imported = pdf_new_array();
            i = 0i32 as libc::c_uint;
            while i < pdf_array_length(object) {
                tmp = pdf_import_object(pdf_get_array(object, i as libc::c_int));
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
    return imported;
}
/* returns 0 if indirect references point to the same object */
#[no_mangle]
pub unsafe extern "C" fn pdf_compare_reference(
    mut ref1: *mut pdf_obj,
    mut ref2: *mut pdf_obj,
) -> libc::c_int {
    let mut data1: *mut pdf_indirect = 0 as *mut pdf_indirect;
    let mut data2: *mut pdf_indirect = 0 as *mut pdf_indirect;
    if !ref1.is_null()
        && pdf_obj_typeof(ref1) == 9i32
        && (!ref2.is_null() && pdf_obj_typeof(ref2) == 9i32)
    {
    } else {
        __assert_fail(
            b"PDF_OBJ_INDIRECTTYPE(ref1) && PDF_OBJ_INDIRECTTYPE(ref2)\x00" as *const u8
                as *const libc::c_char,
            b"dpx-pdfobj.c\x00" as *const u8 as *const libc::c_char,
            3834i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"int pdf_compare_reference(pdf_obj *, pdf_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    data1 = (*ref1).data as *mut pdf_indirect;
    data2 = (*ref2).data as *mut pdf_indirect;
    return ((*data1).pf != (*data2).pf
        || (*data1).label != (*data2).label
        || (*data1).generation as libc::c_int != (*data2).generation as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_obj_reset_global_state() {
    pdf_output_handle = 0 as *mut libc::c_void;
    pdf_output_file_position = 0i32;
    pdf_output_line_position = 0i32;
    compression_saved = 0i32;
}
