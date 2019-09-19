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

use std::slice::from_raw_parts;

use crate::warn;
use super::dpx_dpxcrypt::{AES_cbc_encrypt_tectonic, AES_ecb_encrypt, ARC4_set_key, ARC4};
use super::dpx_dpxcrypt::ARC4_CONTEXT;
use super::dpx_mem::new;
use super::dpx_pdfdoc::pdf_doc_get_dictionary;
use super::dpx_pdffont::get_unique_time_if_given;
use super::dpx_unicode::{UC_UTF8_decode_char, UC_is_valid};
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_get_version, pdf_new_array, pdf_new_dict, pdf_new_name,
    pdf_new_number, pdf_new_string, pdf_obj,
};
use libc::{
    free, gmtime, localtime, memcpy, memset, srand, strcpy, strlen, time,
};
use md5::{Md5, Digest};
use sha2::{Sha256, Sha384, Sha512};
use rand::prelude::*;

pub type __time_t = i64;
pub type size_t = u64;
pub type time_t = __time_t;

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
    pub key: [u8; 32],
    pub key_size: i32,
    pub ID: [u8; 16],
    pub O: [u8; 48],
    pub U: [u8; 48],
    pub OE: [u8; 32],
    pub UE: [u8; 32],
    pub V: i32,
    pub R: i32,
    pub P: i32,
    pub setting: PdfSecSetting,
    pub label: PdfSecLabel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PdfSecLabel {
    pub objnum: u64,
    pub gennum: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PdfSecSetting {
    pub use_aes: i32,
    pub encrypt_metadata: i32,
}
/* Dummy routine for stringprep - NOT IMPLEMENTED YET
 *
 * Preprocessing of a user-provided password consists first of
 * normalizing its representation by applying the "SASLPrep" profile (RFC 4013)
 * of the "stringprep" algorithm (RFC 3454) to the supplied password using the
 * Normalize and BiDi options.
 */
pub type Stringprep_profile_flags = i32;
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
    setting: PdfSecSetting {
        use_aes: 0,
        encrypt_metadata: 0,
    },
    label: PdfSecLabel {
        objnum: 0,
        gennum: 0,
    },
};
static padding_bytes: [u8; 32] = [
    0x28, 0xbf, 0x4e, 0x5e, 0x4e, 0x75, 0x8a, 0x41, 0x64, 0, 0x4e, 0x56, 0xff, 0xfa, 0x1, 0x8,
    0x2e, 0x2e, 0, 0xb6, 0xd0, 0x68, 0x3e, 0x80, 0x2f, 0xc, 0xa9, 0xfe, 0x64, 0x53, 0x69, 0x7a,
];
static mut verbose: u8 = 0_u8;

#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_verbose(mut level: i32) {
    verbose = level as u8; /* For AES IV */
}

unsafe extern "C" fn pdf_enc_init(mut use_aes: i32, mut encrypt_metadata: i32) {
    let mut current_time: time_t = 0;
    let p = &mut sec_data;
    current_time = get_unique_time_if_given();
    if current_time == -1 {
        current_time = time(std::ptr::null_mut())
    }
    // TODO: libc rand is not used in this module
    srand(current_time as u32);
    p.setting.use_aes = use_aes;
    p.setting.encrypt_metadata = encrypt_metadata;
}

pub unsafe fn pdf_enc_compute_id_string(dviname: Option<&[u8]>, mut pdfname: Option<&[u8]>) {
    let p = &mut sec_data;
    /* FIXME: This should be placed in main() or somewhere. */
    pdf_enc_init(1i32, 1i32);
    let mut current_time = get_unique_time_if_given();
    let bd_time = *(if current_time == -1 as time_t {
        time(&mut current_time);
        localtime(&mut current_time)
    } else {
        gmtime(&mut current_time)
    });
    let mut md5 = Md5::new();
    md5.input(&format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        bd_time.tm_year + 1900,
        bd_time.tm_mon + 1,
        bd_time.tm_mday,
        bd_time.tm_hour,
        bd_time.tm_min,
        bd_time.tm_sec,
    ));
    md5.input(&format!(
        "{}-{}, Copyright 2002-2015 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata",
        // TODO: Are these variables?
        "xdvipdfmx",
        "0.1"
    ));
    if let Some(dviname) = dviname {
        md5.input(dviname);
    }
    if let Some(pdfname) = pdfname {
        md5.input(pdfname);
    }
    p.ID = md5.result().into();
}
unsafe extern "C" fn passwd_padding(mut src: *const i8, mut dst: *mut u8) {
    let mut len: i32 = 0;
    len = (if 32 < strlen(src) { 32 } else { strlen(src) }) as i32;
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        len as _,
    );
    memcpy(
        dst.offset(len as isize) as *mut libc::c_void,
        padding_bytes.as_ptr() as *const libc::c_void,
        (32 - len) as _,
    );
}
unsafe extern "C" fn compute_owner_password(
    p: &mut pdf_sec,
    mut opasswd: *const i8,
    mut upasswd: *const i8,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut padded: [u8; 32] = [0; 32];
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    passwd_padding(
        if strlen(opasswd) > 0 {
            opasswd
        } else {
            upasswd
        },
        padded.as_mut_ptr(),
    );
    let mut md5 = Md5::new();
    md5.input(&padded);
    let mut hash = md5.result();
    if p.R >= 3i32 {
        i = 0i32;
        while i < 50i32 {
            /*
             * NOTE: We truncate each MD5 hash as in the following step.
             *       Otherwise Adobe Reader won't decrypt the PDF file.
             */
            let mut md5 = Md5::new();
            md5.input(&hash[..p.key_size as usize]);
            hash = md5.result();
            i += 1
        }
    }
    ARC4_set_key(&mut arc4, p.key_size as u32, hash.as_mut_ptr());
    passwd_padding(upasswd, padded.as_mut_ptr());
    let mut tmp1: [u8; 32] = [0; 32];
    let mut tmp2: [u8; 32] = [0; 32];
    let mut key: [u8; 16] = [0; 16];
    ARC4(&mut arc4, 32_u32, padded.as_mut_ptr(), tmp1.as_mut_ptr());
    if p.R >= 3i32 {
        i = 1i32;
        while i <= 19i32 {
            memcpy(
                tmp2.as_mut_ptr() as *mut libc::c_void,
                tmp1.as_mut_ptr() as *const libc::c_void,
                32,
            );
            j = 0i32;
            while j < p.key_size {
                key[j as usize] = (hash[j as usize] as i32 ^ i) as u8;
                j += 1
            }
            ARC4_set_key(&mut arc4, p.key_size as u32, key.as_mut_ptr());
            ARC4(&mut arc4, 32_u32, tmp2.as_mut_ptr(), tmp1.as_mut_ptr());
            i += 1
        }
    }
    memcpy(
        p.O.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32,
    );
}

unsafe extern "C" fn compute_encryption_key(p: &mut pdf_sec, mut passwd: *const i8) {
    let mut i: i32 = 0;
    let mut padded: [u8; 32] = [0; 32];
    passwd_padding(passwd, padded.as_mut_ptr());
    let mut md5 = Md5::new();
    md5.input(&padded);
    md5.input(&p.O[..32]);
    let mut tmp: [u8; 4] = [0; 4];
    tmp[0] = (p.P as u8 as i32 & 0xffi32) as u8;
    tmp[1] = ((p.P >> 8i32) as u8 as i32 & 0xffi32) as u8;
    tmp[2] = ((p.P >> 16i32) as u8 as i32 & 0xffi32) as u8;
    tmp[3] = ((p.P >> 24i32) as u8 as i32 & 0xffi32) as u8;
    md5.input(&tmp);
    md5.input(&p.ID);
    let mut hash = md5.result();
    if p.R >= 3i32 {
        i = 0i32;
        while i < 50i32 {
            /*
             * NOTE: We truncate each MD5 hash as in the following step.
             *       Otherwise Adobe Reader won't decrypt the PDF file.
             */
            let mut md5 = Md5::new();
            md5.input(&hash.as_slice()[..p.key_size as usize]);
            hash = md5.result();
            i += 1
        }
    }
    memcpy(
        p.key.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        p.key_size as _,
    );
}

unsafe extern "C" fn compute_user_password(p: &mut pdf_sec, mut uplain: *const i8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut upasswd: [u8; 32] = [0; 32];
    compute_encryption_key(p, uplain);
    match p.R {
        2 => {
            ARC4_set_key(&mut arc4, p.key_size as u32, p.key.as_mut_ptr());
            ARC4(
                &mut arc4,
                32_u32,
                padding_bytes.as_ptr(),
                upasswd.as_mut_ptr(),
            );
        }
        3 | 4 => {
            let mut tmp1: [u8; 32] = [0; 32];
            let mut tmp2: [u8; 32] = [0; 32];
            let mut md5 = Md5::new();
            md5.input(&padding_bytes);
            md5.input(&p.ID);
            let mut hash = md5.result();
            ARC4_set_key(&mut arc4, p.key_size as u32, p.key.as_mut_ptr());
            ARC4(&mut arc4, 16_u32, hash.as_mut_ptr(), tmp1.as_mut_ptr());
            i = 1i32;
            while i <= 19i32 {
                let mut key: [u8; 16] = [0; 16];
                memcpy(
                    tmp2.as_mut_ptr() as *mut libc::c_void,
                    tmp1.as_mut_ptr() as *const libc::c_void,
                    16,
                );
                j = 0i32;
                while j < p.key_size {
                    key[j as usize] = (p.key[j as usize] as i32 ^ i) as u8;
                    j += 1
                }
                ARC4_set_key(&mut arc4, p.key_size as u32, key.as_mut_ptr());
                ARC4(&mut arc4, 16_u32, tmp2.as_mut_ptr(), tmp1.as_mut_ptr());
                i += 1
            }
            memcpy(
                upasswd.as_mut_ptr() as *mut libc::c_void,
                tmp1.as_mut_ptr() as *const libc::c_void,
                32,
            );
        }
        _ => {
            panic!("Invalid revision number.");
        }
    }
    memcpy(
        p.U.as_mut_ptr() as *mut libc::c_void,
        upasswd.as_mut_ptr() as *const libc::c_void,
        32,
    );
}
/* Algorithm 2.B from ISO 32000-1 chapter 7 */
unsafe extern "C" fn compute_hash_V5(
    mut passwd: *const i8,
    mut salt: *const u8,
    mut user_key: *const u8,
    mut R: i32,
) -> [u8; 32]
/* revision */
{
    let mut sha = Sha256::new();
    let mut K: [u8; 64] = [0; 64];
    let mut K_len: size_t = 0;
    let mut nround: i32 = 0;
    sha.input(from_raw_parts(passwd as *const u8, strlen(passwd)));
    sha.input(from_raw_parts(salt, 8));
    if !user_key.is_null() {
        sha.input(from_raw_parts(user_key, 48));
    }
    let mut hash: [u8; 32] = sha.result().into();
    assert!(R == 5i32 || R == 6i32);
    if R == 5i32 {
        return hash;
    }
    for (K_item, hash_item) in K.iter_mut().zip(hash.iter()) {
        *K_item = *hash_item;
    }
    K_len = 32i32 as size_t;
    nround = 1i32;
    loop
    /* Initial K count as nround 0. */
    {
        let mut K1: [u8; 256] = [0; 256];
        let mut Kr: *mut u8 = 0 as *mut u8;
        let mut E: *mut u8 = 0 as *mut u8;
        let mut K1_len: size_t = 0;
        let mut E_len: size_t = 0;
        let mut i: i32 = 0;
        let mut c: i32 = 0;
        let mut E_mod3: i32 = 0i32;
        K1_len = strlen(passwd)
            .wrapping_add(K_len as _)
            .wrapping_add(if !user_key.is_null() { 48 } else { 0 }) as _;
        assert!(K1_len < 240i32 as u64);
        memcpy(
            K1.as_mut_ptr() as *mut libc::c_void,
            passwd as *const libc::c_void,
            strlen(passwd),
        );
        memcpy(
            K1.as_mut_ptr().offset(strlen(passwd) as isize) as *mut libc::c_void,
            K.as_mut_ptr() as *const libc::c_void,
            K_len as _,
        );
        if !user_key.is_null() {
            memcpy(
                K1.as_mut_ptr()
                    .offset(strlen(passwd) as isize)
                    .offset(K_len as isize) as *mut libc::c_void,
                user_key as *const libc::c_void,
                48,
            );
        }
        Kr = new((K1_len.wrapping_mul(64i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        i = 0i32;
        while i < 64i32 {
            memcpy(
                Kr.offset((i as u64).wrapping_mul(K1_len) as isize) as *mut libc::c_void,
                K1.as_mut_ptr() as *const libc::c_void,
                K1_len as _,
            );
            i += 1
        }
        AES_cbc_encrypt_tectonic(
            K.as_mut_ptr(),
            16i32 as size_t,
            K.as_mut_ptr().offset(16),
            0i32,
            Kr,
            K1_len.wrapping_mul(64i32 as u64),
            &mut E,
            &mut E_len,
        );
        free(Kr as *mut libc::c_void);
        i = 0i32;
        while i < 16i32 {
            E_mod3 += *E.offset(i as isize) as i32;
            i += 1
        }
        E_mod3 %= 3i32;
        match E_mod3 {
            0 => {
                let mut sha_0 = Sha256::new();
                sha_0.input(from_raw_parts(E, E_len as usize));
                for (K_item, result_item) in K.iter_mut().zip(sha_0.result()) {
                    *K_item = result_item;
                }
                K_len = 32;
            }
            1 => {
                let mut sha_1 = Sha384::new();
                sha_1.input(from_raw_parts(E, E_len as usize));
                for (K_item, result_item) in K.iter_mut().zip(sha_1.result()) {
                    *K_item = result_item;
                }
                K_len = 48;
            }
            2 => {
                let mut sha_2 = Sha512::new();
                sha_2.input(from_raw_parts(E, E_len as usize));
                for (K_item, result_item) in K.iter_mut().zip(sha_2.result()) {
                    *K_item = result_item;
                }
                K_len = 64;
            }
            _ => {}
        }
        c = *E.offset(E_len.wrapping_sub(1i32 as u64) as isize) as i32;
        free(E as *mut libc::c_void);
        if nround >= 64i32 && c <= nround - 32i32 {
            break;
        }
        nround += 1
    }

    for (hash_item, K_item) in hash.iter_mut().zip(K.iter()) {
        *hash_item = *K_item;
    }
    hash
}
unsafe extern "C" fn compute_owner_password_V5(p: &mut pdf_sec, mut oplain: *const i8) {
    let mut vsalt: [u8; 8] = random();
    let mut ksalt: [u8; 8] = random();
    let mut hash: [u8; 32] = [0; 32];
    let mut OE: *mut u8 = 0 as *mut u8;
    let mut iv: [u8; 16] = [0; 16];
    let mut OE_len: size_t = 0;
    hash = compute_hash_V5(
        oplain,
        vsalt.as_mut_ptr(),
        p.U.as_mut_ptr(),
        p.R,
    );
    memcpy(
        p.O.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32,
    );
    memcpy(
        p.O.as_mut_ptr().offset(32) as *mut libc::c_void,
        vsalt.as_mut_ptr() as *const libc::c_void,
        8,
    );
    memcpy(
        p.O.as_mut_ptr().offset(40) as *mut libc::c_void,
        ksalt.as_mut_ptr() as *const libc::c_void,
        8,
    );
    hash = compute_hash_V5(
        oplain,
        ksalt.as_mut_ptr(),
        p.U.as_mut_ptr(),
        p.R,
    );
    memset(iv.as_mut_ptr() as *mut libc::c_void, 0i32, 16);
    AES_cbc_encrypt_tectonic(
        hash.as_mut_ptr(),
        32i32 as size_t,
        iv.as_mut_ptr(),
        0i32,
        p.key.as_mut_ptr(),
        p.key_size as size_t,
        &mut OE,
        &mut OE_len,
    );
    memcpy(
        p.OE.as_mut_ptr() as *mut libc::c_void,
        OE as *const libc::c_void,
        32,
    );
    free(OE as *mut libc::c_void);
}
unsafe extern "C" fn compute_user_password_V5(p: &mut pdf_sec, mut uplain: *const i8) {
    let mut vsalt: [u8; 8] = random();
    let mut ksalt: [u8; 8] = random();
    let mut hash: [u8; 32] = [0; 32];
    let mut UE: *mut u8 = 0 as *mut u8;
    let mut iv: [u8; 16] = [0; 16];
    let mut UE_len: size_t = 0;
    let mut i: i32 = 0;
    hash = compute_hash_V5(
        uplain,
        vsalt.as_mut_ptr(),
        0 as *const u8,
        p.R,
    );
    memcpy(
        p.U.as_mut_ptr() as *mut libc::c_void,
        hash.as_mut_ptr() as *const libc::c_void,
        32,
    );
    memcpy(
        p.U.as_mut_ptr().offset(32) as *mut libc::c_void,
        vsalt.as_mut_ptr() as *const libc::c_void,
        8,
    );
    memcpy(
        p.U.as_mut_ptr().offset(40) as *mut libc::c_void,
        ksalt.as_mut_ptr() as *const libc::c_void,
        8,
    );
    hash = compute_hash_V5(
        uplain,
        ksalt.as_mut_ptr(),
        0 as *const u8,
        p.R,
    );
    memset(iv.as_mut_ptr() as *mut libc::c_void, 0i32, 16);
    AES_cbc_encrypt_tectonic(
        hash.as_mut_ptr(),
        32i32 as size_t,
        iv.as_mut_ptr(),
        0i32,
        p.key.as_mut_ptr(),
        p.key_size as size_t,
        &mut UE,
        &mut UE_len,
    );
    memcpy(
        p.UE.as_mut_ptr() as *mut libc::c_void,
        UE as *const libc::c_void,
        32,
    );
    free(UE as *mut libc::c_void);
}
unsafe extern "C" fn check_version(p: &mut pdf_sec, mut version: i32) {
    if p.V > 2i32 && version < 4i32 {
        warn!("Current encryption setting requires PDF version >= 1.4.");
        p.V = 1i32;
        p.key_size = 5i32
    } else if p.V == 4i32 && version < 5i32 {
        warn!("Current encryption setting requires PDF version >= 1.5.");
        p.V = 2i32
    } else if p.V == 5i32 && version < 7i32 {
        warn!("Current encryption setting requires PDF version >= 1.7 (plus Adobe Extension Level 3).");
        p.V = 4i32
    };
}
unsafe extern "C" fn stringprep_profile(
    mut input: *const i8,
    mut output: *mut *mut i8,
    mut profile: *const i8,
    mut flags: Stringprep_profile_flags,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    p = input;
    endptr = p.offset(strlen(p) as isize);
    while p < endptr {
        let mut ucv: i32 = UC_UTF8_decode_char(
            &mut p as *mut *const i8 as *mut *const u8,
            endptr as *const u8,
        );
        if !UC_is_valid(ucv) {
            return -1i32;
        }
    }
    *output = new((strlen(input).wrapping_add(1)).wrapping_mul(::std::mem::size_of::<i8>()) as _)
        as *mut i8;
    strcpy(*output, input);
    0i32
}
unsafe extern "C" fn preproc_password(
    mut passwd: *const i8,
    mut outbuf: *mut i8,
    mut V: i32,
) -> i32 {
    let mut saslpwd: *mut i8 = 0 as *mut i8;
    let mut error: i32 = 0i32;
    memset(outbuf as *mut libc::c_void, 0i32, 128);
    match V {
        1 | 2 | 3 | 4 => {
            let mut i: size_t = 0;
            /* Need to be converted to PDFDocEncoding - UNIMPLEMENTED */
            i = 0i32 as size_t;
            while (i as usize) < strlen(passwd) {
                if (*passwd.offset(i as isize) as i32) < 0x20i32
                    || *passwd.offset(i as isize) as i32 > 0x7ei32
                {
                    warn!("Non-ASCII-printable character found in password.");
                }
                i = i.wrapping_add(1)
            }
            memcpy(
                outbuf as *mut libc::c_void,
                passwd as *const libc::c_void,
                if 127 < strlen(passwd) {
                    127
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
                b"SASLprep\x00" as *const u8 as *const i8,
                0i32,
            ) != 0i32
            {
                return -1i32;
            } else {
                if !saslpwd.is_null() {
                    memcpy(
                        outbuf as *mut libc::c_void,
                        saslpwd as *const libc::c_void,
                        if 127 < strlen(saslpwd) {
                            127
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
    error
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_passwd(
    mut bits: u32,
    mut perm: u32,
    mut oplain: *const i8,
    mut uplain: *const i8,
) {
    let p = &mut sec_data;
    assert!(!oplain.is_null());
    assert!(!uplain.is_null());
    let version = pdf_get_version();
    p.key_size = bits.wrapping_div(8_u32) as i32;
    if p.key_size == 5i32 {
        /* 40bit */
        p.V = 1i32
    } else if p.key_size <= 16i32 {
        p.V = if p.setting.use_aes != 0 { 4i32 } else { 2i32 }
    } else if p.key_size == 32i32 {
        p.V = 5i32
    } else {
        warn!("Key length {} unsupported.", bits);
        p.key_size = 5i32;
        p.V = 2i32
    }
    check_version(p, version as i32);
    p.P = (perm | 0xc0u32) as i32;
    match p.V {
        1 => p.R = if (p.P as i64) < 0x100 { 2i32 } else { 3i32 },
        2 | 3 => p.R = 3i32,
        4 => p.R = 4i32,
        5 => p.R = 6i32,
        _ => p.R = 3i32,
    }
    /* Password must be preprocessed. */
    let mut opasswd: [i8; 128] = [0; 128];
    let mut upasswd: [i8; 128] = [0; 128];
    if preproc_password(oplain, opasswd.as_mut_ptr(), p.V) < 0i32 {
        warn!("Invaid UTF-8 string for password.");
    }
    if preproc_password(uplain, upasswd.as_mut_ptr(), p.V) < 0i32 {
        warn!("Invalid UTF-8 string for passowrd.");
    }
    if p.R >= 3 {
        p.P = (p.P as u32 | 0xfffff000u32) as i32
    }
    if p.V < 5 {
        compute_owner_password(p, opasswd.as_mut_ptr(), upasswd.as_mut_ptr());
        compute_user_password(p, upasswd.as_mut_ptr());
    } else if p.V == 5 {
        p.key = random();
        p.key_size = 32;
        /* uses p->U */
        compute_user_password_V5(p, upasswd.as_mut_ptr());
        compute_owner_password_V5(p, opasswd.as_mut_ptr());
    };
}

unsafe extern "C" fn calculate_key(p: &mut pdf_sec) -> [u8; 16] {
    let mut len = p.key_size as usize + 5;
    let mut tmp = [0u8; 25];
    memcpy(
        tmp.as_mut_ptr() as *mut libc::c_void,
        p.key.as_mut_ptr() as *const libc::c_void,
        p.key_size as _,
    );
    tmp[p.key_size as usize] = (p.label.objnum as u8 as i32 & 0xffi32) as u8;
    tmp[(p.key_size + 1i32) as usize] = ((p.label.objnum >> 8i32) as u8 as i32 & 0xffi32) as u8;
    tmp[(p.key_size + 2i32) as usize] = ((p.label.objnum >> 16i32) as u8 as i32 & 0xffi32) as u8;
    tmp[(p.key_size + 3i32) as usize] = (p.label.gennum as u8 as i32 & 0xffi32) as u8;
    tmp[(p.key_size + 4i32) as usize] =
        ((p.label.gennum as i32 >> 8i32) as u8 as i32 & 0xffi32) as u8;
    if p.V >= 4i32 {
        tmp[(p.key_size + 5i32) as usize] = 0x73_u8;
        tmp[(p.key_size + 6i32) as usize] = 0x41_u8;
        tmp[(p.key_size + 7i32) as usize] = 0x6c_u8;
        tmp[(p.key_size + 8i32) as usize] = 0x54_u8;
        len += 4;
    }
    let mut md5 = Md5::new();
    md5.input(&tmp[..len]);
    md5.result().into()
}

#[no_mangle]
pub unsafe extern "C" fn pdf_encrypt_data(
    mut plain: *const u8,
    mut plain_len: size_t,
    mut cipher: *mut *mut u8,
    mut cipher_len: *mut size_t,
) {
    let p = &mut sec_data;
    match p.V {
        1 | 2 => {
            let mut key = calculate_key(p);
            let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
                idx_i: 0,
                idx_j: 0,
                sbox: [0; 256],
            };
            *cipher_len = plain_len;
            *cipher = new(
                (*cipher_len as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32
            ) as *mut u8;
            ARC4_set_key(
                &mut arc4,
                (if 16i32 < p.key_size + 5i32 {
                    16i32
                } else {
                    p.key_size + 5i32
                }) as u32,
                key.as_mut_ptr(),
            );
            ARC4(&mut arc4, plain_len as u32, plain, *cipher);
        }
        4 => {
            let mut key = calculate_key(p);
            AES_cbc_encrypt_tectonic(
                key.as_mut_ptr(),
                (if 16i32 < p.key_size + 5i32 {
                    16i32
                } else {
                    p.key_size + 5i32
                }) as size_t,
                0 as *const u8,
                1i32,
                plain,
                plain_len,
                cipher,
                cipher_len,
            );
        }
        5 => {
            AES_cbc_encrypt_tectonic(
                p.key.as_mut_ptr(),
                p.key_size as size_t,
                0 as *const u8,
                1i32,
                plain,
                plain_len,
                cipher,
                cipher_len,
            );
        }
        _ => {
            panic!("pdfencrypt: Unexpected V value: {}", p.V);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_encrypt_obj() -> *mut pdf_obj {
    let p = &mut sec_data;
    let mut doc_encrypt: *mut pdf_obj = 0 as *mut pdf_obj;
    doc_encrypt = pdf_new_dict();
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Filter\x00" as *const u8 as *const i8),
        pdf_new_name(b"Standard\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"V\x00" as *const u8 as *const i8),
        pdf_new_number(p.V as f64),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Length\x00" as *const u8 as *const i8),
        pdf_new_number((p.key_size * 8i32) as f64),
    );
    if p.V >= 4i32 {
        let mut CF: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut StdCF: *mut pdf_obj = 0 as *mut pdf_obj;
        CF = pdf_new_dict();
        StdCF = pdf_new_dict();
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"CFM\x00" as *const u8 as *const i8),
            pdf_new_name(if p.V == 4i32 {
                b"AESV2\x00" as *const u8 as *const i8
            } else {
                b"AESV3\x00" as *const u8 as *const i8
            }),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"AuthEvent\x00" as *const u8 as *const i8),
            pdf_new_name(b"DocOpen\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"Length\x00" as *const u8 as *const i8),
            pdf_new_number(p.key_size as f64),
        );
        pdf_add_dict(
            CF,
            pdf_new_name(b"StdCF\x00" as *const u8 as *const i8),
            StdCF,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"CF\x00" as *const u8 as *const i8),
            CF,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StmF\x00" as *const u8 as *const i8),
            pdf_new_name(b"StdCF\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StrF\x00" as *const u8 as *const i8),
            pdf_new_name(b"StdCF\x00" as *const u8 as *const i8),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"R\x00" as *const u8 as *const i8),
        pdf_new_number(p.R as f64),
    );
    if p.V < 5i32 {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\x00" as *const u8 as *const i8),
            pdf_new_string(p.O.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\x00" as *const u8 as *const i8),
            pdf_new_string(p.U.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
    } else if p.V == 5i32 {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\x00" as *const u8 as *const i8),
            pdf_new_string(p.O.as_mut_ptr() as *const libc::c_void, 48i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\x00" as *const u8 as *const i8),
            pdf_new_string(p.U.as_mut_ptr() as *const libc::c_void, 48i32 as size_t),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"P\x00" as *const u8 as *const i8),
        pdf_new_number(p.P as f64),
    );
    if p.V == 5i32 {
        let mut perms: [u8; 16] = [0; 16];
        let mut cipher: *mut u8 = 0 as *mut u8;
        let mut cipher_len: size_t = 0i32 as size_t;
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"OE\x00" as *const u8 as *const i8),
            pdf_new_string(p.OE.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"UE\x00" as *const u8 as *const i8),
            pdf_new_string(p.UE.as_mut_ptr() as *const libc::c_void, 32i32 as size_t),
        );
        perms[0] = (p.P & 0xffi32) as u8;
        perms[1] = (p.P >> 8i32 & 0xffi32) as u8;
        perms[2] = (p.P >> 16i32 & 0xffi32) as u8;
        perms[3] = (p.P >> 24i32 & 0xffi32) as u8;
        perms[4] = 0xff_u8;
        perms[5] = 0xff_u8;
        perms[6] = 0xff_u8;
        perms[7] = 0xff_u8;
        perms[8] = (if p.setting.encrypt_metadata != 0 {
            'T' as i32
        } else {
            'F' as i32
        }) as u8;
        perms[9] = 'a' as i32 as u8;
        perms[10] = 'd' as i32 as u8;
        perms[11] = 'b' as i32 as u8;
        perms[12] = 0_u8;
        perms[13] = 0_u8;
        perms[14] = 0_u8;
        perms[15] = 0_u8;
        AES_ecb_encrypt(
            p.key.as_mut_ptr(),
            p.key_size as size_t,
            perms.as_mut_ptr(),
            16i32 as size_t,
            &mut cipher,
            &mut cipher_len,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"Perms\x00" as *const u8 as *const i8),
            pdf_new_string(cipher as *const libc::c_void, cipher_len),
        );
        free(cipher as *mut libc::c_void);
    }
    if p.R > 5i32 {
        let mut catalog: *mut pdf_obj =
            pdf_doc_get_dictionary(b"Catalog\x00" as *const u8 as *const i8);
        let mut ext: *mut pdf_obj = pdf_new_dict();
        let mut adbe: *mut pdf_obj = pdf_new_dict();
        pdf_add_dict(
            adbe,
            pdf_new_name(b"BaseVersion\x00" as *const u8 as *const i8),
            pdf_new_name(b"1.7\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            adbe,
            pdf_new_name(b"ExtensionLevel\x00" as *const u8 as *const i8),
            pdf_new_number((if p.R == 5i32 { 3i32 } else { 8i32 }) as f64),
        );
        pdf_add_dict(
            ext,
            pdf_new_name(b"ADBE\x00" as *const u8 as *const i8),
            adbe,
        );
        pdf_add_dict(
            catalog,
            pdf_new_name(b"Extensions\x00" as *const u8 as *const i8),
            ext,
        );
    }
    doc_encrypt
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_id_array() -> *mut pdf_obj {
    let p = &mut sec_data;
    let mut id: *mut pdf_obj = pdf_new_array();
    pdf_add_array(
        id,
        pdf_new_string(p.ID.as_mut_ptr() as *const libc::c_void, 16i32 as size_t),
    );
    pdf_add_array(
        id,
        pdf_new_string(p.ID.as_mut_ptr() as *const libc::c_void, 16i32 as size_t),
    );
    id
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_label(mut label: u32) {
    let p = &mut sec_data;
    p.label.objnum = label as u64;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_enc_set_generation(mut generation: u32) {
    let p = &mut sec_data;
    p.label.gennum = generation as u16;
}
/* Order is important here */
