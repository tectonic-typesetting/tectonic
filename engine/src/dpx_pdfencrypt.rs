#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pdf_obj;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn MD5_init(ctx: *mut MD5_CONTEXT);
    #[no_mangle]
    fn MD5_write(ctx: *mut MD5_CONTEXT, inbuf: *const libc::c_uchar, inlen: libc::c_uint);
    #[no_mangle]
    fn MD5_final(outbuf: *mut libc::c_uchar, ctx: *mut MD5_CONTEXT);
    #[no_mangle]
    fn SHA256_init(ctx: *mut SHA256_CONTEXT);
    #[no_mangle]
    fn SHA256_write(ctx: *mut SHA256_CONTEXT, inbuf: *const libc::c_uchar, inlen: libc::c_uint);
    #[no_mangle]
    fn SHA256_final(outbuf: *mut libc::c_uchar, ctx: *mut SHA256_CONTEXT);
    #[no_mangle]
    fn SHA384_init(ctx: *mut SHA512_CONTEXT);
    #[no_mangle]
    fn SHA512_init(ctx: *mut SHA512_CONTEXT);
    #[no_mangle]
    fn SHA512_write(ctx: *mut SHA512_CONTEXT, inbuf: *const libc::c_uchar, inlen: libc::c_uint);
    #[no_mangle]
    fn SHA512_final(outbuf: *mut libc::c_uchar, ctx: *mut SHA512_CONTEXT);
    #[no_mangle]
    fn ARC4(
        ctx: *mut ARC4_CONTEXT,
        len: libc::c_uint,
        inbuf: *const libc::c_uchar,
        outbuf: *mut libc::c_uchar,
    );
    #[no_mangle]
    fn ARC4_set_key(ctx: *mut ARC4_CONTEXT, keylen: libc::c_uint, key: *const libc::c_uchar);
    #[no_mangle]
    fn AES_ecb_encrypt(
        key: *const libc::c_uchar,
        key_len: size_t,
        plain: *const libc::c_uchar,
        plain_len: size_t,
        cipher: *mut *mut libc::c_uchar,
        cipher_len: *mut size_t,
    );
    #[no_mangle]
    fn AES_cbc_encrypt_tectonic(
        key: *const libc::c_uchar,
        key_len: size_t,
        iv: *const libc::c_uchar,
        padding: libc::c_int,
        plain: *const libc::c_uchar,
        plain_len: size_t,
        cipher: *mut *mut libc::c_uchar,
        cipher_len: *mut size_t,
    );
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
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_get_version() -> libc::c_uint;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
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
    fn pdf_new_dict() -> *mut pdf_obj;
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn get_unique_time_if_given() -> time_t;
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
    fn UC_is_valid(ucv: int32_t) -> bool;
    #[no_mangle]
    fn UC_UTF8_decode_char(pp: *mut *const libc::c_uchar, endptr: *const libc::c_uchar) -> int32_t;
    /* They just return PDF dictionary object.
     * Callers are completely responsible for doing right thing...
     */
    #[no_mangle]
    fn pdf_doc_get_dictionary(category: *const libc::c_char) -> *mut pdf_obj;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CONTEXT {
    pub A: u32,
    pub B: u32,
    pub C: u32,
    pub D: u32,
    pub nblocks: size_t,
    pub buf: [libc::c_uchar; 64],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CONTEXT {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub h5: u32,
    pub h6: u32,
    pub h7: u32,
    pub nblocks: size_t,
    pub buf: [libc::c_uchar; 64],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512_STATE {
    pub h0: uint64_t,
    pub h1: uint64_t,
    pub h2: uint64_t,
    pub h3: uint64_t,
    pub h4: uint64_t,
    pub h5: uint64_t,
    pub h6: uint64_t,
    pub h7: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512_CONTEXT {
    pub state: SHA512_STATE,
    pub nblocks: size_t,
    pub buf: [libc::c_uchar; 128],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARC4_CONTEXT {
    pub idx_i: libc::c_int,
    pub idx_j: libc::c_int,
    pub sbox: [libc::c_uchar; 256],
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
/* Encryption support
 *
 * Supported: 40-128 bit RC4, 128 bit AES, 256 bit AES
 *
 * TODO: Convert password to PDFDocEncoding. SASLPrep stringpref for AESV3.
 */
/* PDF-2.0 is not published yet. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_sec {
    pub key: [libc::c_uchar; 32],
    pub key_size: libc::c_int,
    pub ID: [libc::c_uchar; 16],
    pub O: [libc::c_uchar; 48],
    pub U: [libc::c_uchar; 48],
    pub OE: [libc::c_uchar; 32],
    pub UE: [libc::c_uchar; 32],
    pub V: libc::c_int,
    pub R: libc::c_int,
    pub P: int32_t,
    pub setting: C2RustUnnamed_0,
    pub label: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub objnum: uint64_t,
    pub gennum: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub use_aes: libc::c_int,
    pub encrypt_metadata: libc::c_int,
}
/* Dummy routine for stringprep - NOT IMPLEMENTED YET
 *
 * Preprocessing of a user-provided password consists first of
 * normalizing its representation by applying the "SASLPrep" profile (RFC 4013)
 * of the "stringprep" algorithm (RFC 3454) to the supplied password using the
 * Normalize and BiDi options.
 */
pub type Stringprep_profile_flags = libc::c_int;
static mut sec_data: pdf_sec = pdf_sec {
    key: [0; 32],
    key_size: 0,
    ID: [0; 16],
    O: [0; 48],
    U: [0; 48],
    OE: [0; 32],
    UE: [0; 32],
    V: 0,
    R: 0,
    P: 0,
    setting: C2RustUnnamed_0 {
        use_aes: 0,
        encrypt_metadata: 0,
    },
    label: C2RustUnnamed {
        objnum: 0,
        gennum: 0,
    },
};
static mut padding_bytes: [libc::c_uchar; 32] = [
    0x28i32 as libc::c_uchar,
    0xbfi32 as libc::c_uchar,
    0x4ei32 as libc::c_uchar,
    0x5ei32 as libc::c_uchar,
    0x4ei32 as libc::c_uchar,
    0x75i32 as libc::c_uchar,
    0x8ai32 as libc::c_uchar,
    0x41i32 as libc::c_uchar,
    0x64i32 as libc::c_uchar,
    0i32 as libc::c_uchar,
    0x4ei32 as libc::c_uchar,
    0x56i32 as libc::c_uchar,
    0xffi32 as libc::c_uchar,
    0xfai32 as libc::c_uchar,
    0x1i32 as libc::c_uchar,
    0x8i32 as libc::c_uchar,
    0x2ei32 as libc::c_uchar,
    0x2ei32 as libc::c_uchar,
    0i32 as libc::c_uchar,
    0xb6i32 as libc::c_uchar,
    0xd0i32 as libc::c_uchar,
    0x68i32 as libc::c_uchar,
    0x3ei32 as libc::c_uchar,
    0x80i32 as libc::c_uchar,
    0x2fi32 as libc::c_uchar,
    0xci32 as libc::c_uchar,
    0xa9i32 as libc::c_uchar,
    0xfei32 as libc::c_uchar,
    0x64i32 as libc::c_uchar,
    0x53i32 as libc::c_uchar,
    0x69i32 as libc::c_uchar,
    0x7ai32 as libc::c_uchar,
];
static mut verbose: libc::c_uchar = 0i32 as libc::c_uchar;
/*

    This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

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
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_verbose(mut level: libc::c_int) {
    verbose = level as libc::c_uchar; /* For AES IV */
}
unsafe extern "C" fn pdf_enc_init(mut use_aes: libc::c_int, mut encrypt_metadata: libc::c_int) {
    let mut current_time: time_t = 0;
    let mut p: *mut pdf_sec = &mut sec_data;
    current_time = get_unique_time_if_given();
    if current_time == -1i32 as time_t {
        current_time = time(0 as *mut time_t)
    }
    srand(current_time as libc::c_uint);
    (*p).setting.use_aes = use_aes;
    (*p).setting.encrypt_metadata = encrypt_metadata;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_compute_id_string(
    mut dviname: *const libc::c_char,
    mut pdfname: *const libc::c_char,
) {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut date_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut producer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_time: time_t = 0;
    let mut bd_time: *mut tm = 0 as *mut tm;
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    /* FIXME: This should be placed in main() or somewhere. */
    pdf_enc_init(1i32, 1i32);
    MD5_init(&mut md5);
    date_string = new((15i32 as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as u32) as *mut libc::c_char;
    current_time = get_unique_time_if_given();
    if current_time == -1i32 as time_t {
        time(&mut current_time);
        bd_time = localtime(&mut current_time)
    } else {
        bd_time = gmtime(&mut current_time)
    }
    sprintf(
        date_string,
        b"%04d%02d%02d%02d%02d%02d\x00" as *const u8 as *const libc::c_char,
        (*bd_time).tm_year + 1900i32,
        (*bd_time).tm_mon + 1i32,
        (*bd_time).tm_mday,
        (*bd_time).tm_hour,
        (*bd_time).tm_min,
        (*bd_time).tm_sec,
    );
    MD5_write(
        &mut md5,
        date_string as *mut libc::c_uchar,
        strlen(date_string) as libc::c_uint,
    );
    free(date_string as *mut libc::c_void);
    producer = new((strlen(
        b"%s-%s, Copyright 2002-2015 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata\x00"
            as *const u8 as *const libc::c_char,
    )
    .wrapping_add(strlen(b"xdvipdfmx\x00" as *const u8 as *const libc::c_char))
    .wrapping_add(strlen(b"0.1\x00" as *const u8 as *const libc::c_char))
        as u32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as u32) as *mut libc::c_char;
    sprintf(
        producer,
        b"%s-%s, Copyright 2002-2015 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata\x00"
            as *const u8 as *const libc::c_char,
        b"xdvipdfmx\x00" as *const u8 as *const libc::c_char,
        b"0.1\x00" as *const u8 as *const libc::c_char,
    );
    MD5_write(
        &mut md5,
        producer as *mut libc::c_uchar,
        strlen(producer) as libc::c_uint,
    );
    free(producer as *mut libc::c_void);
    if !dviname.is_null() {
        MD5_write(
            &mut md5,
            dviname as *const libc::c_uchar,
            strlen(dviname) as libc::c_uint,
        );
    }
    if !pdfname.is_null() {
        MD5_write(
            &mut md5,
            pdfname as *const libc::c_uchar,
            strlen(pdfname) as libc::c_uint,
        );
    }
    MD5_final((*p).ID.as_mut_ptr(), &mut md5);
}
unsafe extern "C" fn passwd_padding(mut src: *const libc::c_char, mut dst: *mut libc::c_uchar) {
    let mut len: libc::c_int = 0;
    len = (if (32i32 as libc::c_ulong) < strlen(src) {
        32i32 as libc::c_ulong
    } else {
        strlen(src)
    }) as libc::c_int;
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        len as libc::c_ulong,
    );
    memcpy(
        dst.offset(len as isize) as *mut libc::c_void,
        padding_bytes.as_ptr() as *const libc::c_void,
        (32i32 - len) as libc::c_ulong,
    );
}
unsafe extern "C" fn compute_owner_password(
    mut p: *mut pdf_sec,
    mut opasswd: *const libc::c_char,
    mut upasswd: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut padded: [libc::c_uchar; 32] = [0; 32];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut hash: [libc::c_uchar; 32] = [0; 32];
    passwd_padding(
        if strlen(opasswd) > 0i32 as libc::c_ulong {
            opasswd
        } else {
            upasswd
        },
        padded.as_mut_ptr(),
    );
    MD5_init(&mut md5);
    MD5_write(&mut md5, padded.as_mut_ptr(), 32i32 as libc::c_uint);
    MD5_final(hash.as_mut_ptr(), &mut md5);
    if (*p).R >= 3i32 {
        i = 0i32;
        while i < 50i32 {
            /*
             * NOTE: We truncate each MD5 hash as in the following step.
             *       Otherwise Adobe Reader won't decrypt the PDF file.
             */
            MD5_init(&mut md5);
            MD5_write(&mut md5, hash.as_mut_ptr(), (*p).key_size as libc::c_uint);
            MD5_final(hash.as_mut_ptr(), &mut md5);
            i += 1
        }
    }
    ARC4_set_key(&mut arc4, (*p).key_size as libc::c_uint, hash.as_mut_ptr());
    passwd_padding(upasswd, padded.as_mut_ptr());
    let mut tmp1: [libc::c_uchar; 32] = [0; 32];
    let mut tmp2: [libc::c_uchar; 32] = [0; 32];
    let mut key: [libc::c_uchar; 16] = [0; 16];
    ARC4(
        &mut arc4,
        32i32 as libc::c_uint,
        padded.as_mut_ptr(),
        tmp1.as_mut_ptr(),
    );
    if (*p).R >= 3i32 {
        i = 1i32;
        while i <= 19i32 {
            memcpy(
                tmp2.as_mut_ptr() as *mut libc::c_void,
                tmp1.as_mut_ptr() as *const libc::c_void,
                32i32 as libc::c_ulong,
            );
            j = 0i32;
            while j < (*p).key_size {
                key[j as usize] = (hash[j as usize] as libc::c_int ^ i) as libc::c_uchar;
                j += 1
            }
            ARC4_set_key(&mut arc4, (*p).key_size as libc::c_uint, key.as_mut_ptr());
            ARC4(
                &mut arc4,
                32i32 as libc::c_uint,
                tmp2.as_mut_ptr(),
                tmp1.as_mut_ptr(),
            );
            i += 1
        }
    }
    memcpy(
        (*p).O.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
}
unsafe extern "C" fn compute_encryption_key(mut p: *mut pdf_sec, mut passwd: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut hash: [libc::c_uchar; 32] = [0; 32];
    let mut padded: [libc::c_uchar; 32] = [0; 32];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    passwd_padding(passwd, padded.as_mut_ptr());
    MD5_init(&mut md5);
    MD5_write(&mut md5, padded.as_mut_ptr(), 32i32 as libc::c_uint);
    MD5_write(&mut md5, (*p).O.as_mut_ptr(), 32i32 as libc::c_uint);
    let mut tmp: [libc::c_uchar; 4] = [0; 4];
    tmp[0] = ((*p).P as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[1] = (((*p).P >> 8i32) as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[2] = (((*p).P >> 16i32) as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[3] = (((*p).P >> 24i32) as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    MD5_write(&mut md5, tmp.as_mut_ptr(), 4i32 as libc::c_uint);
    MD5_write(&mut md5, (*p).ID.as_mut_ptr(), 16i32 as libc::c_uint);
    MD5_final(hash.as_mut_ptr(), &mut md5);
    if (*p).R >= 3i32 {
        i = 0i32;
        while i < 50i32 {
            /*
             * NOTE: We truncate each MD5 hash as in the following step.
             *       Otherwise Adobe Reader won't decrypt the PDF file.
             */
            MD5_init(&mut md5);
            MD5_write(&mut md5, hash.as_mut_ptr(), (*p).key_size as libc::c_uint);
            MD5_final(hash.as_mut_ptr(), &mut md5);
            i += 1
        }
    }
    memcpy(
        (*p).key.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        (*p).key_size as libc::c_ulong,
    );
}
unsafe extern "C" fn compute_user_password(mut p: *mut pdf_sec, mut uplain: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut upasswd: [libc::c_uchar; 32] = [0; 32];
    compute_encryption_key(p, uplain);
    match (*p).R {
        2 => {
            ARC4_set_key(
                &mut arc4,
                (*p).key_size as libc::c_uint,
                (*p).key.as_mut_ptr(),
            );
            ARC4(
                &mut arc4,
                32i32 as libc::c_uint,
                padding_bytes.as_ptr(),
                upasswd.as_mut_ptr(),
            );
        }
        3 | 4 => {
            let mut hash: [libc::c_uchar; 32] = [0; 32];
            let mut tmp1: [libc::c_uchar; 32] = [0; 32];
            let mut tmp2: [libc::c_uchar; 32] = [0; 32];
            MD5_init(&mut md5);
            MD5_write(&mut md5, padding_bytes.as_ptr(), 32i32 as libc::c_uint);
            MD5_write(&mut md5, (*p).ID.as_mut_ptr(), 16i32 as libc::c_uint);
            MD5_final(hash.as_mut_ptr(), &mut md5);
            ARC4_set_key(
                &mut arc4,
                (*p).key_size as libc::c_uint,
                (*p).key.as_mut_ptr(),
            );
            ARC4(
                &mut arc4,
                16i32 as libc::c_uint,
                hash.as_mut_ptr(),
                tmp1.as_mut_ptr(),
            );
            i = 1i32;
            while i <= 19i32 {
                let mut key: [libc::c_uchar; 16] = [0; 16];
                memcpy(
                    tmp2.as_mut_ptr() as *mut libc::c_void,
                    tmp1.as_mut_ptr() as *const libc::c_void,
                    16i32 as libc::c_ulong,
                );
                j = 0i32;
                while j < (*p).key_size {
                    key[j as usize] = ((*p).key[j as usize] as libc::c_int ^ i) as libc::c_uchar;
                    j += 1
                }
                ARC4_set_key(&mut arc4, (*p).key_size as libc::c_uint, key.as_mut_ptr());
                ARC4(
                    &mut arc4,
                    16i32 as libc::c_uint,
                    tmp2.as_mut_ptr(),
                    tmp1.as_mut_ptr(),
                );
                i += 1
            }
            memcpy(
                upasswd.as_mut_ptr() as *mut libc::c_void,
                tmp1.as_mut_ptr() as *const libc::c_void,
                32i32 as libc::c_ulong,
            );
        }
        _ => {
            _tt_abort(b"Invalid revision number.\x00" as *const u8 as *const libc::c_char);
        }
    }
    memcpy(
        (*p).U.as_mut_ptr() as *mut libc::c_void,
        upasswd.as_mut_ptr() as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
}
/* Algorithm 2.B from ISO 32000-1 chapter 7 */
unsafe extern "C" fn compute_hash_V5(
    mut hash: *mut libc::c_uchar,
    mut passwd: *const libc::c_char,
    mut salt: *const libc::c_uchar,
    mut user_key: *const libc::c_uchar,
    mut R: libc::c_int,
)
/* revision */
{
    let mut sha: SHA256_CONTEXT = SHA256_CONTEXT {
        h0: 0,
        h1: 0,
        h2: 0,
        h3: 0,
        h4: 0,
        h5: 0,
        h6: 0,
        h7: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut K: [libc::c_uchar; 64] = [0; 64];
    let mut K_len: size_t = 0;
    let mut nround: libc::c_int = 0;
    SHA256_init(&mut sha);
    SHA256_write(
        &mut sha,
        passwd as *const libc::c_uchar,
        strlen(passwd) as libc::c_uint,
    );
    SHA256_write(&mut sha, salt, 8i32 as libc::c_uint);
    if !user_key.is_null() {
        SHA256_write(&mut sha, user_key, 48i32 as libc::c_uint);
    }
    SHA256_final(hash, &mut sha);
    if R == 5i32 || R == 6i32 {
    } else {
        __assert_fail(b"R ==5 || R == 6\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-pdfencrypt.c\x00" as *const u8 as
                          *const libc::c_char, 307i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 103],
                                                &[libc::c_char; 103]>(b"void compute_hash_V5(unsigned char *, const char *, const unsigned char *, const unsigned char *, int)\x00")).as_ptr());
    }
    if R == 5i32 {
        return;
    }
    memcpy(
        K.as_mut_ptr() as *mut libc::c_void,
        hash as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
    K_len = 32i32 as size_t;
    nround = 1i32;
    loop
    /* Initial K count as nround 0. */
    {
        let mut K1: [libc::c_uchar; 256] = [0; 256];
        let mut Kr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut E: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut K1_len: size_t = 0;
        let mut E_len: size_t = 0;
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut E_mod3: libc::c_int = 0i32;
        K1_len = strlen(passwd)
            .wrapping_add(K_len)
            .wrapping_add((if !user_key.is_null() { 48i32 } else { 0i32 }) as libc::c_ulong);
        if K1_len < 240i32 as libc::c_ulong {
        } else {
            __assert_fail(b"K1_len < 240\x00" as *const u8 as
                              *const libc::c_char,
                          b"dpx-pdfencrypt.c\x00" as *const u8 as
                              *const libc::c_char, 319i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 103],
                                                    &[libc::c_char; 103]>(b"void compute_hash_V5(unsigned char *, const char *, const unsigned char *, const unsigned char *, int)\x00")).as_ptr());
        }
        memcpy(
            K1.as_mut_ptr() as *mut libc::c_void,
            passwd as *const libc::c_void,
            strlen(passwd),
        );
        memcpy(
            K1.as_mut_ptr().offset(strlen(passwd) as isize) as *mut libc::c_void,
            K.as_mut_ptr() as *const libc::c_void,
            K_len,
        );
        if !user_key.is_null() {
            memcpy(
                K1.as_mut_ptr()
                    .offset(strlen(passwd) as isize)
                    .offset(K_len as isize) as *mut libc::c_void,
                user_key as *const libc::c_void,
                48i32 as libc::c_ulong,
            );
        }
        Kr = new(
            (K1_len.wrapping_mul(64i32 as libc::c_ulong) as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as u32,
        ) as *mut libc::c_uchar;
        i = 0i32;
        while i < 64i32 {
            memcpy(
                Kr.offset((i as libc::c_ulong).wrapping_mul(K1_len) as isize) as *mut libc::c_void,
                K1.as_mut_ptr() as *const libc::c_void,
                K1_len,
            );
            i += 1
        }
        AES_cbc_encrypt_tectonic(
            K.as_mut_ptr(),
            16i32 as size_t,
            K.as_mut_ptr().offset(16),
            0i32,
            Kr,
            K1_len.wrapping_mul(64i32 as libc::c_ulong),
            &mut E,
            &mut E_len,
        );
        free(Kr as *mut libc::c_void);
        i = 0i32;
        while i < 16i32 {
            E_mod3 += *E.offset(i as isize) as libc::c_int;
            i += 1
        }
        E_mod3 %= 3i32;
        match E_mod3 {
            0 => {
                let mut sha_0: SHA256_CONTEXT = SHA256_CONTEXT {
                    h0: 0,
                    h1: 0,
                    h2: 0,
                    h3: 0,
                    h4: 0,
                    h5: 0,
                    h6: 0,
                    h7: 0,
                    nblocks: 0,
                    buf: [0; 64],
                    count: 0,
                };
                SHA256_init(&mut sha_0);
                SHA256_write(&mut sha_0, E, E_len as libc::c_uint);
                SHA256_final(K.as_mut_ptr(), &mut sha_0);
                K_len = 32i32 as size_t
            }
            1 => {
                let mut sha_1: SHA512_CONTEXT = SHA512_CONTEXT {
                    state: SHA512_STATE {
                        h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        h5: 0,
                        h6: 0,
                        h7: 0,
                    },
                    nblocks: 0,
                    buf: [0; 128],
                    count: 0,
                };
                SHA384_init(&mut sha_1);
                SHA512_write(&mut sha_1, E, E_len as libc::c_uint);
                SHA512_final(K.as_mut_ptr(), &mut sha_1);
                K_len = 48i32 as size_t
            }
            2 => {
                let mut sha_2: SHA512_CONTEXT = SHA512_CONTEXT {
                    state: SHA512_STATE {
                        h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        h5: 0,
                        h6: 0,
                        h7: 0,
                    },
                    nblocks: 0,
                    buf: [0; 128],
                    count: 0,
                };
                SHA512_init(&mut sha_2);
                SHA512_write(&mut sha_2, E, E_len as libc::c_uint);
                SHA512_final(K.as_mut_ptr(), &mut sha_2);
                K_len = 64i32 as size_t
            }
            _ => {}
        }
        c = *E.offset(E_len.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int;
        free(E as *mut libc::c_void);
        if nround >= 64i32 && c <= nround - 32i32 {
            break;
        }
        nround += 1
    }
    memcpy(
        hash as *mut libc::c_void,
        K.as_mut_ptr() as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
}
unsafe extern "C" fn compute_owner_password_V5(
    mut p: *mut pdf_sec,
    mut oplain: *const libc::c_char,
) {
    let mut vsalt: [libc::c_uchar; 8] = [0; 8];
    let mut ksalt: [libc::c_uchar; 8] = [0; 8];
    let mut hash: [libc::c_uchar; 32] = [0; 32];
    let mut OE: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut OE_len: size_t = 0;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8i32 {
        vsalt[i as usize] = (rand() % 256i32) as libc::c_uchar;
        ksalt[i as usize] = (rand() % 256i32) as libc::c_uchar;
        i += 1
    }
    compute_hash_V5(
        hash.as_mut_ptr(),
        oplain,
        vsalt.as_mut_ptr(),
        (*p).U.as_mut_ptr(),
        (*p).R,
    );
    memcpy(
        (*p).O.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
    memcpy(
        (*p).O.as_mut_ptr().offset(32) as *mut libc::c_void,
        vsalt.as_mut_ptr() as *const libc::c_void,
        8i32 as libc::c_ulong,
    );
    memcpy(
        (*p).O.as_mut_ptr().offset(40) as *mut libc::c_void,
        ksalt.as_mut_ptr() as *const libc::c_void,
        8i32 as libc::c_ulong,
    );
    compute_hash_V5(
        hash.as_mut_ptr(),
        oplain,
        ksalt.as_mut_ptr(),
        (*p).U.as_mut_ptr(),
        (*p).R,
    );
    memset(
        iv.as_mut_ptr() as *mut libc::c_void,
        0i32,
        16i32 as libc::c_ulong,
    );
    AES_cbc_encrypt_tectonic(
        hash.as_mut_ptr(),
        32i32 as size_t,
        iv.as_mut_ptr(),
        0i32,
        (*p).key.as_mut_ptr(),
        (*p).key_size as size_t,
        &mut OE,
        &mut OE_len,
    );
    memcpy(
        (*p).OE.as_mut_ptr() as *mut libc::c_void,
        OE as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
    free(OE as *mut libc::c_void);
}
unsafe extern "C" fn compute_user_password_V5(
    mut p: *mut pdf_sec,
    mut uplain: *const libc::c_char,
) {
    let mut vsalt: [libc::c_uchar; 8] = [0; 8];
    let mut ksalt: [libc::c_uchar; 8] = [0; 8];
    let mut hash: [libc::c_uchar; 32] = [0; 32];
    let mut UE: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut UE_len: size_t = 0;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8i32 {
        vsalt[i as usize] = (rand() % 256i32) as libc::c_uchar;
        ksalt[i as usize] = (rand() % 256i32) as libc::c_uchar;
        i += 1
    }
    compute_hash_V5(
        hash.as_mut_ptr(),
        uplain,
        vsalt.as_mut_ptr(),
        0 as *const libc::c_uchar,
        (*p).R,
    );
    memcpy(
        (*p).U.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
    memcpy(
        (*p).U.as_mut_ptr().offset(32) as *mut libc::c_void,
        vsalt.as_mut_ptr() as *const libc::c_void,
        8i32 as libc::c_ulong,
    );
    memcpy(
        (*p).U.as_mut_ptr().offset(40) as *mut libc::c_void,
        ksalt.as_mut_ptr() as *const libc::c_void,
        8i32 as libc::c_ulong,
    );
    compute_hash_V5(
        hash.as_mut_ptr(),
        uplain,
        ksalt.as_mut_ptr(),
        0 as *const libc::c_uchar,
        (*p).R,
    );
    memset(
        iv.as_mut_ptr() as *mut libc::c_void,
        0i32,
        16i32 as libc::c_ulong,
    );
    AES_cbc_encrypt_tectonic(
        hash.as_mut_ptr(),
        32i32 as size_t,
        iv.as_mut_ptr(),
        0i32,
        (*p).key.as_mut_ptr(),
        (*p).key_size as size_t,
        &mut UE,
        &mut UE_len,
    );
    memcpy(
        (*p).UE.as_mut_ptr() as *mut libc::c_void,
        UE as *const libc::c_void,
        32i32 as libc::c_ulong,
    );
    free(UE as *mut libc::c_void);
}
unsafe extern "C" fn check_version(mut p: *mut pdf_sec, mut version: libc::c_int) {
    if (*p).V > 2i32 && version < 4i32 {
        dpx_warning(
            b"Current encryption setting requires PDF version >= 1.4.\x00" as *const u8
                as *const libc::c_char,
        );
        (*p).V = 1i32;
        (*p).key_size = 5i32
    } else if (*p).V == 4i32 && version < 5i32 {
        dpx_warning(
            b"Current encryption setting requires PDF version >= 1.5.\x00" as *const u8
                as *const libc::c_char,
        );
        (*p).V = 2i32
    } else if (*p).V == 5i32 && version < 7i32 {
        dpx_warning(b"Current encryption setting requires PDF version >= 1.7 (plus Adobe Extension Level 3).\x00"
                        as *const u8 as *const libc::c_char);
        (*p).V = 4i32
    };
}
unsafe extern "C" fn stringprep_profile(
    mut input: *const libc::c_char,
    mut output: *mut *mut libc::c_char,
    mut profile: *const libc::c_char,
    mut flags: Stringprep_profile_flags,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    p = input;
    endptr = p.offset(strlen(p) as isize);
    while p < endptr {
        let mut ucv: int32_t = UC_UTF8_decode_char(
            &mut p as *mut *const libc::c_char as *mut *const libc::c_uchar,
            endptr as *const libc::c_uchar,
        );
        if !UC_is_valid(ucv) {
            return -1i32;
        }
    }
    *output = new(
        (strlen(input).wrapping_add(1i32 as libc::c_ulong) as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as u32,
    ) as *mut libc::c_char;
    strcpy(*output, input);
    return 0i32;
}
unsafe extern "C" fn preproc_password(
    mut passwd: *const libc::c_char,
    mut outbuf: *mut libc::c_char,
    mut V: libc::c_int,
) -> libc::c_int {
    let mut saslpwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: libc::c_int = 0i32;
    memset(outbuf as *mut libc::c_void, 0i32, 128i32 as libc::c_ulong);
    match V {
        1 | 2 | 3 | 4 => {
            let mut i: size_t = 0;
            /* Need to be converted to PDFDocEncoding - UNIMPLEMENTED */
            i = 0i32 as size_t;
            while i < strlen(passwd) {
                if (*passwd.offset(i as isize) as libc::c_int) < 0x20i32
                    || *passwd.offset(i as isize) as libc::c_int > 0x7ei32
                {
                    dpx_warning(
                        b"Non-ASCII-printable character found in password.\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                i = i.wrapping_add(1)
            }
            memcpy(
                outbuf as *mut libc::c_void,
                passwd as *const libc::c_void,
                if (127i32 as libc::c_ulong) < strlen(passwd) {
                    127i32 as libc::c_ulong
                } else {
                    strlen(passwd)
                },
            );
        }
        5 => {
            /* This is a dummy routine - not actually stringprep password... */
            if stringprep_profile(
                passwd,
                &mut saslpwd,
                b"SASLprep\x00" as *const u8 as *const libc::c_char,
                0i32,
            ) != 0i32
            {
                return -1i32;
            } else {
                if !saslpwd.is_null() {
                    memcpy(
                        outbuf as *mut libc::c_void,
                        saslpwd as *const libc::c_void,
                        if (127i32 as libc::c_ulong) < strlen(saslpwd) {
                            127i32 as libc::c_ulong
                        } else {
                            strlen(saslpwd)
                        },
                    );
                    free(saslpwd as *mut libc::c_void);
                }
            }
        }
        _ => error = -1i32,
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_passwd(
    mut bits: libc::c_uint,
    mut perm: libc::c_uint,
    mut oplain: *const libc::c_char,
    mut uplain: *const libc::c_char,
) {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut opasswd: [libc::c_char; 128] = [0; 128];
    let mut upasswd: [libc::c_char; 128] = [0; 128];
    let mut version: libc::c_int = 0;
    if !oplain.is_null() {
    } else {
        __assert_fail(b"oplain\x00" as *const u8 as *const libc::c_char,
                      b"dpx-pdfencrypt.c\x00" as *const u8 as
                          *const libc::c_char, 521i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void pdf_enc_set_passwd(unsigned int, unsigned int, const char *, const char *)\x00")).as_ptr());
    }
    if !uplain.is_null() {
    } else {
        __assert_fail(b"uplain\x00" as *const u8 as *const libc::c_char,
                      b"dpx-pdfencrypt.c\x00" as *const u8 as
                          *const libc::c_char, 522i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"void pdf_enc_set_passwd(unsigned int, unsigned int, const char *, const char *)\x00")).as_ptr());
    }
    version = pdf_get_version() as libc::c_int;
    (*p).key_size = bits.wrapping_div(8i32 as libc::c_uint) as libc::c_int;
    if (*p).key_size == 5i32 {
        /* 40bit */
        (*p).V = 1i32
    } else if (*p).key_size <= 16i32 {
        (*p).V = if (*p).setting.use_aes != 0 {
            4i32
        } else {
            2i32
        }
    } else if (*p).key_size == 32i32 {
        (*p).V = 5i32
    } else {
        dpx_warning(
            b"Key length %d unsupported.\x00" as *const u8 as *const libc::c_char,
            bits,
        );
        (*p).key_size = 5i32;
        (*p).V = 2i32
    }
    check_version(p, version);
    (*p).P = (perm | 0xc0u32) as int32_t;
    match (*p).V {
        1 => {
            (*p).R = if ((*p).P as libc::c_long) < 0x100 {
                2i32
            } else {
                3i32
            }
        }
        2 | 3 => (*p).R = 3i32,
        4 => (*p).R = 4i32,
        5 => (*p).R = 6i32,
        _ => (*p).R = 3i32,
    }
    memset(
        opasswd.as_mut_ptr() as *mut libc::c_void,
        0i32,
        128i32 as libc::c_ulong,
    );
    memset(
        upasswd.as_mut_ptr() as *mut libc::c_void,
        0i32,
        128i32 as libc::c_ulong,
    );
    /* Password must be preprocessed. */
    if preproc_password(oplain, opasswd.as_mut_ptr(), (*p).V) < 0i32 {
        dpx_warning(b"Invaid UTF-8 string for password.\x00" as *const u8 as *const libc::c_char);
    }
    if preproc_password(uplain, upasswd.as_mut_ptr(), (*p).V) < 0i32 {
        dpx_warning(b"Invalid UTF-8 string for passowrd.\x00" as *const u8 as *const libc::c_char);
    }
    if (*p).R >= 3i32 {
        (*p).P = ((*p).P as libc::c_uint | 0xfffff000u32) as int32_t
    }
    if (*p).V < 5i32 {
        compute_owner_password(p, opasswd.as_mut_ptr(), upasswd.as_mut_ptr());
        compute_user_password(p, upasswd.as_mut_ptr());
    } else if (*p).V == 5i32 {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < 32i32 {
            (*p).key[i as usize] = (rand() % 256i32) as libc::c_uchar;
            i += 1
        }
        (*p).key_size = 32i32;
        /* uses p->U */
        compute_user_password_V5(p, upasswd.as_mut_ptr());
        compute_owner_password_V5(p, opasswd.as_mut_ptr());
    };
}
unsafe extern "C" fn calculate_key(mut p: *mut pdf_sec, mut key: *mut libc::c_uchar) {
    let mut len: libc::c_int = (*p).key_size + 5i32;
    let mut tmp: [libc::c_uchar; 25] = [0; 25];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    memcpy(
        tmp.as_mut_ptr() as *mut libc::c_void,
        (*p).key.as_mut_ptr() as *const libc::c_void,
        (*p).key_size as libc::c_ulong,
    );
    tmp[(*p).key_size as usize] =
        ((*p).label.objnum as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[((*p).key_size + 1i32) as usize] =
        (((*p).label.objnum >> 8i32) as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[((*p).key_size + 2i32) as usize] =
        (((*p).label.objnum >> 16i32) as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[((*p).key_size + 3i32) as usize] =
        ((*p).label.gennum as libc::c_uchar as libc::c_int & 0xffi32) as libc::c_uchar;
    tmp[((*p).key_size + 4i32) as usize] = (((*p).label.gennum as libc::c_int >> 8i32)
        as libc::c_uchar as libc::c_int
        & 0xffi32) as libc::c_uchar;
    if (*p).V >= 4i32 {
        tmp[((*p).key_size + 5i32) as usize] = 0x73i32 as libc::c_uchar;
        tmp[((*p).key_size + 6i32) as usize] = 0x41i32 as libc::c_uchar;
        tmp[((*p).key_size + 7i32) as usize] = 0x6ci32 as libc::c_uchar;
        tmp[((*p).key_size + 8i32) as usize] = 0x54i32 as libc::c_uchar;
        len += 4i32
    }
    MD5_init(&mut md5);
    MD5_write(&mut md5, tmp.as_mut_ptr(), len as libc::c_uint);
    MD5_final(key, &mut md5);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_encrypt_data(
    mut plain: *const libc::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut libc::c_uchar,
    mut cipher_len: *mut size_t,
) {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut key: [libc::c_uchar; 32] = [0; 32];
    match (*p).V {
        1 | 2 => {
            calculate_key(p, key.as_mut_ptr());
            let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
                idx_i: 0,
                idx_j: 0,
                sbox: [0; 256],
            };
            *cipher_len = plain_len;
            *cipher = new((*cipher_len as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as u32) as *mut libc::c_uchar;
            ARC4_set_key(
                &mut arc4,
                (if 16i32 < (*p).key_size + 5i32 {
                    16i32
                } else {
                    (*p).key_size + 5i32
                }) as libc::c_uint,
                key.as_mut_ptr(),
            );
            ARC4(&mut arc4, plain_len as libc::c_uint, plain, *cipher);
        }
        4 => {
            calculate_key(p, key.as_mut_ptr());
            AES_cbc_encrypt_tectonic(
                key.as_mut_ptr(),
                (if 16i32 < (*p).key_size + 5i32 {
                    16i32
                } else {
                    (*p).key_size + 5i32
                }) as size_t,
                0 as *const libc::c_uchar,
                1i32,
                plain,
                plain_len,
                cipher,
                cipher_len,
            );
        }
        5 => {
            AES_cbc_encrypt_tectonic(
                (*p).key.as_mut_ptr(),
                (*p).key_size as size_t,
                0 as *const libc::c_uchar,
                1i32,
                plain,
                plain_len,
                cipher,
                cipher_len,
            );
        }
        _ => {
            _tt_abort(
                b"pdfencrypt: Unexpected V value: %d\x00" as *const u8 as *const libc::c_char,
                (*p).V,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_encrypt_obj() -> *mut pdf_obj {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut doc_encrypt: *mut pdf_obj = 0 as *mut pdf_obj;
    doc_encrypt = pdf_new_dict();
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Filter\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"Standard\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"V\x00" as *const u8 as *const libc::c_char),
        pdf_new_number((*p).V as libc::c_double),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Length\x00" as *const u8 as *const libc::c_char),
        pdf_new_number(((*p).key_size * 8i32) as libc::c_double),
    );
    if (*p).V >= 4i32 {
        let mut CF: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut StdCF: *mut pdf_obj = 0 as *mut pdf_obj;
        CF = pdf_new_dict();
        StdCF = pdf_new_dict();
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"CFM\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(if (*p).V == 4i32 {
                b"AESV2\x00" as *const u8 as *const libc::c_char
            } else {
                b"AESV3\x00" as *const u8 as *const libc::c_char
            }),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"AuthEvent\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"DocOpen\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"Length\x00" as *const u8 as *const libc::c_char),
            pdf_new_number((*p).key_size as libc::c_double),
        );
        pdf_add_dict(
            CF,
            pdf_new_name(b"StdCF\x00" as *const u8 as *const libc::c_char),
            StdCF,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"CF\x00" as *const u8 as *const libc::c_char),
            CF,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StmF\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"StdCF\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StrF\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"StdCF\x00" as *const u8 as *const libc::c_char),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"R\x00" as *const u8 as *const libc::c_char),
        pdf_new_number((*p).R as libc::c_double),
    );
    if (*p).V < 5i32 {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\x00" as *const u8 as *const libc::c_char),
            pdf_new_string((*p).O.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\x00" as *const u8 as *const libc::c_char),
            pdf_new_string((*p).U.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
    } else if (*p).V == 5i32 {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\x00" as *const u8 as *const libc::c_char),
            pdf_new_string((*p).O.as_mut_ptr() as *const libc::c_void, 48i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\x00" as *const u8 as *const libc::c_char),
            pdf_new_string((*p).U.as_mut_ptr() as *const libc::c_void, 48i32 as size_t),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"P\x00" as *const u8 as *const libc::c_char),
        pdf_new_number((*p).P as libc::c_double),
    );
    if (*p).V == 5i32 {
        let mut perms: [libc::c_uchar; 16] = [0; 16];
        let mut cipher: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut cipher_len: size_t = 0i32 as size_t;
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"OE\x00" as *const u8 as *const libc::c_char),
            pdf_new_string((*p).OE.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"UE\x00" as *const u8 as *const libc::c_char),
            pdf_new_string((*p).UE.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        perms[0] = ((*p).P & 0xffi32) as libc::c_uchar;
        perms[1] = ((*p).P >> 8i32 & 0xffi32) as libc::c_uchar;
        perms[2] = ((*p).P >> 16i32 & 0xffi32) as libc::c_uchar;
        perms[3] = ((*p).P >> 24i32 & 0xffi32) as libc::c_uchar;
        perms[4] = 0xffi32 as libc::c_uchar;
        perms[5] = 0xffi32 as libc::c_uchar;
        perms[6] = 0xffi32 as libc::c_uchar;
        perms[7] = 0xffi32 as libc::c_uchar;
        perms[8] = (if (*p).setting.encrypt_metadata != 0 {
            'T' as i32
        } else {
            'F' as i32
        }) as libc::c_uchar;
        perms[9] = 'a' as i32 as libc::c_uchar;
        perms[10] = 'd' as i32 as libc::c_uchar;
        perms[11] = 'b' as i32 as libc::c_uchar;
        perms[12] = 0i32 as libc::c_uchar;
        perms[13] = 0i32 as libc::c_uchar;
        perms[14] = 0i32 as libc::c_uchar;
        perms[15] = 0i32 as libc::c_uchar;
        AES_ecb_encrypt(
            (*p).key.as_mut_ptr(),
            (*p).key_size as size_t,
            perms.as_mut_ptr(),
            16i32 as size_t,
            &mut cipher,
            &mut cipher_len,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"Perms\x00" as *const u8 as *const libc::c_char),
            pdf_new_string(cipher as *const libc::c_void, cipher_len),
        );
        free(cipher as *mut libc::c_void);
    }
    if (*p).R > 5i32 {
        let mut catalog: *mut pdf_obj =
            pdf_doc_get_dictionary(b"Catalog\x00" as *const u8 as *const libc::c_char);
        let mut ext: *mut pdf_obj = pdf_new_dict();
        let mut adbe: *mut pdf_obj = pdf_new_dict();
        pdf_add_dict(
            adbe,
            pdf_new_name(b"BaseVersion\x00" as *const u8 as *const libc::c_char),
            pdf_new_name(b"1.7\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_dict(
            adbe,
            pdf_new_name(b"ExtensionLevel\x00" as *const u8 as *const libc::c_char),
            pdf_new_number((if (*p).R == 5i32 { 3i32 } else { 8i32 }) as libc::c_double),
        );
        pdf_add_dict(
            ext,
            pdf_new_name(b"ADBE\x00" as *const u8 as *const libc::c_char),
            adbe,
        );
        pdf_add_dict(
            catalog,
            pdf_new_name(b"Extensions\x00" as *const u8 as *const libc::c_char),
            ext,
        );
    }
    return doc_encrypt;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_id_array() -> *mut pdf_obj {
    let mut p: *mut pdf_sec = &mut sec_data;
    let mut id: *mut pdf_obj = pdf_new_array();
    pdf_add_array(
        id,
        pdf_new_string((*p).ID.as_mut_ptr() as *const libc::c_void, 16i32 as size_t),
    );
    pdf_add_array(
        id,
        pdf_new_string((*p).ID.as_mut_ptr() as *const libc::c_void, 16i32 as size_t),
    );
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_label(mut label: libc::c_uint) {
    let mut p: *mut pdf_sec = &mut sec_data;
    (*p).label.objnum = label as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_generation(mut generation: libc::c_uint) {
    let mut p: *mut pdf_sec = &mut sec_data;
    (*p).label.gennum = generation as uint16_t;
}
/* Order is important here */
