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
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
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
}
pub type uint8_t = u8;
pub type size_t = u64;
/* This is DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2003-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
/* libgcrypt md5 */
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
    pub h0: u64,
    pub h1: u64,
    pub h2: u64,
    pub h3: u64,
    pub h4: u64,
    pub h5: u64,
    pub h6: u64,
    pub h7: u64,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AES_CONTEXT {
    pub nrounds: libc::c_int,
    pub rk: [u32; 60],
    pub iv: [libc::c_uchar; 16],
}
/* This is DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2003-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
unsafe extern "C" fn _gcry_burn_stack(mut bytes: libc::c_int) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 64]>() as u64,
    );
    bytes = (bytes as u64)
        .wrapping_sub(::std::mem::size_of::<[libc::c_char; 64]>() as u64)
        as libc::c_int as libc::c_int;
    if bytes > 0i32 {
        _gcry_burn_stack(bytes);
    };
}
/* Rotate a 32 bit integer by n bytes */
/*
 * The following codes for MD5 Message-Digest Algorithm were modified
 * by Jin-Hwan Cho on August 5, 2003 based on libgrypt-1.1.42.
 *
 * Copyright (C) 1995,1996,1998,1999,2001,2002,2003 Free Software Foundation, Inc.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 *
 * According to the definition of MD5 in RFC 1321 from April 1992.
 * NOTE: This is *not* the same file as the one from glibc.
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995.
 * heavily modified for GnuPG by Werner Koch <wk@gnupg.org>
 */
#[no_mangle]
pub unsafe extern "C" fn MD5_init(mut ctx: *mut MD5_CONTEXT) {
    (*ctx).A = 0x67452301u32;
    (*ctx).B = 0xefcdab89u32;
    (*ctx).C = 0x98badcfeu32;
    (*ctx).D = 0x10325476u32;
    (*ctx).nblocks = 0i32 as size_t;
    (*ctx).count = 0i32;
}
/* These are the four functions used in the four steps of the MD5 algorithm
 * and defined in the RFC 1321. The first function is a little bit optimized
 * (as found in Colin Plumbs public domain implementation). */
/* #define FF(b, c, d) ((b & c) | (~b & d)) */
/* transform n*64 bytes */
unsafe extern "C" fn transform(mut ctx: *mut MD5_CONTEXT, mut data: *const libc::c_uchar) {
    let mut correct_words: [u32; 16] = [0; 16];
    let mut A: u32 = (*ctx).A;
    let mut B: u32 = (*ctx).B;
    let mut C: u32 = (*ctx).C;
    let mut D: u32 = (*ctx).D;
    let mut cwp: *mut u32 = correct_words.as_mut_ptr();
    memcpy(
        correct_words.as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        (::std::mem::size_of::<u32>() as u64).wrapping_mul(16i32 as u64),
    );
    /* Before we start, one word about the strange constants.
     * They are defined in RFC 1321 as
     *
     *   T[i] = (int) (4294967296.0 * fabs (sin (i))), i=1..64
     */
    /* Round 1. */
    let fresh0 = cwp;
    cwp = cwp.offset(1);
    A = (A as libc::c_uint).wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh0)
            .wrapping_add(0xd76aa478u32),
    ) as u32;
    A = A << 7i32 | A >> 32i32 - 7i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    let fresh1 = cwp;
    cwp = cwp.offset(1);
    D = (D as libc::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh1)
            .wrapping_add(0xe8c7b756u32),
    ) as u32;
    D = D << 12i32 | D >> 32i32 - 12i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    let fresh2 = cwp;
    cwp = cwp.offset(1);
    C = (C as libc::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh2)
            .wrapping_add(0x242070dbi32 as libc::c_uint),
    ) as u32;
    C = C << 17i32 | C >> 32i32 - 17i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    let fresh3 = cwp;
    cwp = cwp.offset(1);
    B = (B as libc::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh3)
            .wrapping_add(0xc1bdceeeu32),
    ) as u32;
    B = B << 22i32 | B >> 32i32 - 22i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    let fresh4 = cwp;
    cwp = cwp.offset(1);
    A = (A as libc::c_uint).wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh4)
            .wrapping_add(0xf57c0fafu32),
    ) as u32;
    A = A << 7i32 | A >> 32i32 - 7i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    let fresh5 = cwp;
    cwp = cwp.offset(1);
    D = (D as libc::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh5)
            .wrapping_add(0x4787c62ai32 as libc::c_uint),
    ) as u32;
    D = D << 12i32 | D >> 32i32 - 12i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    let fresh6 = cwp;
    cwp = cwp.offset(1);
    C = (C as libc::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh6)
            .wrapping_add(0xa8304613u32),
    ) as u32;
    C = C << 17i32 | C >> 32i32 - 17i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    let fresh7 = cwp;
    cwp = cwp.offset(1);
    B = (B as libc::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh7)
            .wrapping_add(0xfd469501u32),
    ) as u32;
    B = B << 22i32 | B >> 32i32 - 22i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    let fresh8 = cwp;
    cwp = cwp.offset(1);
    A = (A as libc::c_uint).wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh8)
            .wrapping_add(0x698098d8i32 as libc::c_uint),
    ) as u32;
    A = A << 7i32 | A >> 32i32 - 7i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    let fresh9 = cwp;
    cwp = cwp.offset(1);
    D = (D as libc::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh9)
            .wrapping_add(0x8b44f7afu32),
    ) as u32;
    D = D << 12i32 | D >> 32i32 - 12i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    let fresh10 = cwp;
    cwp = cwp.offset(1);
    C = (C as libc::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh10)
            .wrapping_add(0xffff5bb1u32),
    ) as u32;
    C = C << 17i32 | C >> 32i32 - 17i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    let fresh11 = cwp;
    cwp = cwp.offset(1);
    B = (B as libc::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh11)
            .wrapping_add(0x895cd7beu32),
    ) as u32;
    B = B << 22i32 | B >> 32i32 - 22i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    let fresh12 = cwp;
    cwp = cwp.offset(1);
    A = (A as libc::c_uint).wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh12)
            .wrapping_add(0x6b901122i32 as libc::c_uint),
    ) as u32;
    A = A << 7i32 | A >> 32i32 - 7i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    let fresh13 = cwp;
    cwp = cwp.offset(1);
    D = (D as libc::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh13)
            .wrapping_add(0xfd987193u32),
    ) as u32;
    D = D << 12i32 | D >> 32i32 - 12i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    let fresh14 = cwp;
    cwp = cwp.offset(1);
    C = (C as libc::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh14)
            .wrapping_add(0xa679438eu32),
    ) as u32;
    C = C << 17i32 | C >> 32i32 - 17i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    let fresh15 = cwp;
    cwp = cwp.offset(1);
    B = (B as libc::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh15)
            .wrapping_add(0x49b40821i32 as libc::c_uint),
    ) as u32;
    B = B << 22i32 | B >> 32i32 - 22i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    /* Round 2. */
    A = (A as libc::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[1])
            .wrapping_add(0xf61e2562u32),
    ) as u32;
    A = A << 5i32 | A >> 32i32 - 5i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[6])
            .wrapping_add(0xc040b340u32),
    ) as u32;
    D = D << 9i32 | D >> 32i32 - 9i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[11])
            .wrapping_add(0x265e5a51i32 as libc::c_uint),
    ) as u32;
    C = C << 14i32 | C >> 32i32 - 14i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[0])
            .wrapping_add(0xe9b6c7aau32),
    ) as u32;
    B = B << 20i32 | B >> 32i32 - 20i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[5])
            .wrapping_add(0xd62f105du32),
    ) as u32;
    A = A << 5i32 | A >> 32i32 - 5i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[10])
            .wrapping_add(0x2441453i32 as libc::c_uint),
    ) as u32;
    D = D << 9i32 | D >> 32i32 - 9i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[15])
            .wrapping_add(0xd8a1e681u32),
    ) as u32;
    C = C << 14i32 | C >> 32i32 - 14i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[4])
            .wrapping_add(0xe7d3fbc8u32),
    ) as u32;
    B = B << 20i32 | B >> 32i32 - 20i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[9])
            .wrapping_add(0x21e1cde6i32 as libc::c_uint),
    ) as u32;
    A = A << 5i32 | A >> 32i32 - 5i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[14])
            .wrapping_add(0xc33707d6u32),
    ) as u32;
    D = D << 9i32 | D >> 32i32 - 9i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[3])
            .wrapping_add(0xf4d50d87u32),
    ) as u32;
    C = C << 14i32 | C >> 32i32 - 14i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[8])
            .wrapping_add(0x455a14edi32 as libc::c_uint),
    ) as u32;
    B = B << 20i32 | B >> 32i32 - 20i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[13])
            .wrapping_add(0xa9e3e905u32),
    ) as u32;
    A = A << 5i32 | A >> 32i32 - 5i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[2])
            .wrapping_add(0xfcefa3f8u32),
    ) as u32;
    D = D << 9i32 | D >> 32i32 - 9i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[7])
            .wrapping_add(0x676f02d9i32 as libc::c_uint),
    ) as u32;
    C = C << 14i32 | C >> 32i32 - 14i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[12])
            .wrapping_add(0x8d2a4c8au32),
    ) as u32;
    B = B << 20i32 | B >> 32i32 - 20i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    /* Round 3. */
    A = (A as libc::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[5])
            .wrapping_add(0xfffa3942u32),
    ) as u32;
    A = A << 4i32 | A >> 32i32 - 4i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[8])
            .wrapping_add(0x8771f681u32),
    ) as u32;
    D = D << 11i32 | D >> 32i32 - 11i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[11])
            .wrapping_add(0x6d9d6122i32 as libc::c_uint),
    ) as u32;
    C = C << 16i32 | C >> 32i32 - 16i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[14])
            .wrapping_add(0xfde5380cu32),
    ) as u32;
    B = B << 23i32 | B >> 32i32 - 23i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[1])
            .wrapping_add(0xa4beea44u32),
    ) as u32;
    A = A << 4i32 | A >> 32i32 - 4i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[4])
            .wrapping_add(0x4bdecfa9i32 as libc::c_uint),
    ) as u32;
    D = D << 11i32 | D >> 32i32 - 11i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[7])
            .wrapping_add(0xf6bb4b60u32),
    ) as u32;
    C = C << 16i32 | C >> 32i32 - 16i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[10])
            .wrapping_add(0xbebfbc70u32),
    ) as u32;
    B = B << 23i32 | B >> 32i32 - 23i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[13])
            .wrapping_add(0x289b7ec6i32 as libc::c_uint),
    ) as u32;
    A = A << 4i32 | A >> 32i32 - 4i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[0])
            .wrapping_add(0xeaa127fau32),
    ) as u32;
    D = D << 11i32 | D >> 32i32 - 11i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[3])
            .wrapping_add(0xd4ef3085u32),
    ) as u32;
    C = C << 16i32 | C >> 32i32 - 16i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[6])
            .wrapping_add(0x4881d05i32 as libc::c_uint),
    ) as u32;
    B = B << 23i32 | B >> 32i32 - 23i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[9])
            .wrapping_add(0xd9d4d039u32),
    ) as u32;
    A = A << 4i32 | A >> 32i32 - 4i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[12])
            .wrapping_add(0xe6db99e5u32),
    ) as u32;
    D = D << 11i32 | D >> 32i32 - 11i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[15])
            .wrapping_add(0x1fa27cf8i32 as libc::c_uint),
    ) as u32;
    C = C << 16i32 | C >> 32i32 - 16i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[2])
            .wrapping_add(0xc4ac5665u32),
    ) as u32;
    B = B << 23i32 | B >> 32i32 - 23i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    /* Round 4.  */
    A = (A as libc::c_uint).wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[0])
            .wrapping_add(0xf4292244u32),
    ) as u32;
    A = A << 6i32 | A >> 32i32 - 6i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[7])
            .wrapping_add(0x432aff97i32 as libc::c_uint),
    ) as u32;
    D = D << 10i32 | D >> 32i32 - 10i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[14])
            .wrapping_add(0xab9423a7u32),
    ) as u32;
    C = C << 15i32 | C >> 32i32 - 15i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[5])
            .wrapping_add(0xfc93a039u32),
    ) as u32;
    B = B << 21i32 | B >> 32i32 - 21i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[12])
            .wrapping_add(0x655b59c3i32 as libc::c_uint),
    ) as u32;
    A = A << 6i32 | A >> 32i32 - 6i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[3])
            .wrapping_add(0x8f0ccc92u32),
    ) as u32;
    D = D << 10i32 | D >> 32i32 - 10i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[10])
            .wrapping_add(0xffeff47du32),
    ) as u32;
    C = C << 15i32 | C >> 32i32 - 15i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[1])
            .wrapping_add(0x85845dd1u32),
    ) as u32;
    B = B << 21i32 | B >> 32i32 - 21i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[8])
            .wrapping_add(0x6fa87e4fi32 as libc::c_uint),
    ) as u32;
    A = A << 6i32 | A >> 32i32 - 6i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[15])
            .wrapping_add(0xfe2ce6e0u32),
    ) as u32;
    D = D << 10i32 | D >> 32i32 - 10i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[6])
            .wrapping_add(0xa3014314u32),
    ) as u32;
    C = C << 15i32 | C >> 32i32 - 15i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[13])
            .wrapping_add(0x4e0811a1i32 as libc::c_uint),
    ) as u32;
    B = B << 21i32 | B >> 32i32 - 21i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    A = (A as libc::c_uint).wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[4])
            .wrapping_add(0xf7537e82u32),
    ) as u32;
    A = A << 6i32 | A >> 32i32 - 6i32;
    A = (A as libc::c_uint).wrapping_add(B) as u32;
    D = (D as libc::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[11])
            .wrapping_add(0xbd3af235u32),
    ) as u32;
    D = D << 10i32 | D >> 32i32 - 10i32;
    D = (D as libc::c_uint).wrapping_add(A) as u32;
    C = (C as libc::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[2])
            .wrapping_add(0x2ad7d2bbi32 as libc::c_uint),
    ) as u32;
    C = C << 15i32 | C >> 32i32 - 15i32;
    C = (C as libc::c_uint).wrapping_add(D) as u32;
    B = (B as libc::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[9])
            .wrapping_add(0xeb86d391u32),
    ) as u32;
    B = B << 21i32 | B >> 32i32 - 21i32;
    B = (B as libc::c_uint).wrapping_add(C) as u32;
    /* Put checksum in context given as argument. */
    (*ctx).A = ((*ctx).A as libc::c_uint).wrapping_add(A) as u32;
    (*ctx).B = ((*ctx).B as libc::c_uint).wrapping_add(B) as u32;
    (*ctx).C = ((*ctx).C as libc::c_uint).wrapping_add(C) as u32;
    (*ctx).D = ((*ctx).D as libc::c_uint).wrapping_add(D) as u32;
}
/* The routine updates the message-digest context to
 * account for the presence of each of the characters inBuf[0..inLen-1]
 * in the message whose digest is being computed. */
#[no_mangle]
pub unsafe extern "C" fn MD5_write(
    mut hd: *mut MD5_CONTEXT,
    mut inbuf: *const libc::c_uchar,
    mut inlen: libc::c_uint,
) {
    if (*hd).count == 64i32 {
        /* flush the buffer */
        transform(hd, (*hd).buf.as_mut_ptr());
        _gcry_burn_stack(
            (80i32 as u64).wrapping_add(
                (6i32 as u64)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
            ) as libc::c_int,
        );
        (*hd).count = 0i32;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1)
    }
    if inbuf.is_null() {
        return;
    }
    if (*hd).count != 0 {
        while inlen != 0 && (*hd).count < 64i32 {
            let fresh16 = inbuf;
            inbuf = inbuf.offset(1);
            let fresh17 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh17 as usize] = *fresh16;
            inlen = inlen.wrapping_sub(1)
        }
        MD5_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
        if inlen == 0 {
            return;
        }
    }
    _gcry_burn_stack(
        (80i32 as u64).wrapping_add(
            (6i32 as u64)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
        ) as libc::c_int,
    );
    while inlen >= 64i32 as libc::c_uint {
        transform(hd, inbuf);
        (*hd).count = 0i32;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
        inlen = inlen.wrapping_sub(64i32 as libc::c_uint);
        inbuf = inbuf.offset(64)
    }
    while inlen != 0 && (*hd).count < 64i32 {
        let fresh18 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh19 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh19 as usize] = *fresh18;
        inlen = inlen.wrapping_sub(1)
    }
}
/* The routine final terminates the message-digest computation and
 * ends with the desired message digest in mdContext->digest[0...15].
 * The handle is prepared for a new MD5 cycle.
 * Returns 16 bytes representing the digest. */
#[no_mangle]
pub unsafe extern "C" fn MD5_final(mut outbuf: *mut libc::c_uchar, mut hd: *mut MD5_CONTEXT) {
    let mut t: u32 = 0; /* flush */
    let mut msb: u32 = 0;
    let mut lsb: u32 = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    MD5_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
    t = (*hd).nblocks as u32;
    /* multiply by 64 to make a byte count */
    lsb = t << 6i32;
    msb = t >> 26i32;
    /* add the count */
    t = lsb;
    lsb = (lsb as libc::c_uint).wrapping_add((*hd).count as libc::c_uint) as u32;
    if lsb < t {
        msb = msb.wrapping_add(1)
    }
    /* multiply by 8 to make a bit count */
    t = lsb;
    lsb <<= 3i32;
    msb <<= 3i32;
    msb |= t >> 29i32;
    if (*hd).count < 56i32 {
        /* enough room */
        let fresh20 = (*hd).count; /* pad */
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh20 as usize] = 0x80i32 as libc::c_uchar;
        while (*hd).count < 56i32 {
            let fresh21 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh21 as usize] = 0i32 as libc::c_uchar
        }
    /* pad */
    } else {
        let fresh22 = (*hd).count; /* pad character */
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh22 as usize] = 0x80i32 as libc::c_uchar;
        while (*hd).count < 64i32 {
            let fresh23 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh23 as usize] = 0i32 as libc::c_uchar
        }
        /* fill next block with zeroes */
        MD5_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint); /* flush */
        memset(
            (*hd).buf.as_mut_ptr() as *mut libc::c_void,
            0i32,
            56i32 as u64,
        );
    }
    /* append the 64 bit count */
    (*hd).buf[56] = (lsb & 0xffi32 as libc::c_uint) as libc::c_uchar; /* p = hd->buf; */
    (*hd).buf[57] = (lsb >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
    (*hd).buf[58] = (lsb >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
    (*hd).buf[59] = (lsb >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
    (*hd).buf[60] = (msb & 0xffi32 as libc::c_uint) as libc::c_uchar;
    (*hd).buf[61] = (msb >> 8i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
    (*hd).buf[62] = (msb >> 16i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
    (*hd).buf[63] = (msb >> 24i32 & 0xffi32 as libc::c_uint) as libc::c_uchar;
    transform(hd, (*hd).buf.as_mut_ptr());
    _gcry_burn_stack(
        (80i32 as u64).wrapping_add(
            (6i32 as u64)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
        ) as libc::c_int,
    );
    p = outbuf;
    /* little endian */
    *(p as *mut u32) = (*hd).A;
    p = p.offset(::std::mem::size_of::<u32>() as u64 as isize);
    *(p as *mut u32) = (*hd).B;
    p = p.offset(::std::mem::size_of::<u32>() as u64 as isize);
    *(p as *mut u32) = (*hd).C;
    p = p.offset(::std::mem::size_of::<u32>() as u64 as isize);
    *(p as *mut u32) = (*hd).D;
    p = p.offset(::std::mem::size_of::<u32>() as u64 as isize);
}
/*
 * The following codes for the SHA256 hash function are taken from
 * libgrypt-1.6.3. (slightly modified)
 *
 * sha256.c - SHA256 hash function
 * Copyright (C) 2003, 2006, 2008, 2009 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 */
unsafe extern "C" fn _gcry_bswap32(mut x: u32) -> u32 {
    return ((x << 8i32 | x >> 32i32 - 8i32) as libc::c_long & 0xff00ffi64
        | (x >> (8i32 & 32i32 - 1i32) | x << (32i32 - 8i32 & 32i32 - 1i32)) as libc::c_long
            & 0xff00ff00i64) as u32;
}
unsafe extern "C" fn _gcry_bswap64(mut x: u64) -> u64 {
    return (_gcry_bswap32(x as u32) as u64) << 32i32
        | _gcry_bswap32((x >> 32i32) as u32) as u64;
}
/* Endian dependent byte swap operations.  */
unsafe extern "C" fn buf_get_be32(mut _buf: *const libc::c_void) -> u32 {
    let mut in_0: *const uint8_t = _buf as *const uint8_t;
    return (*in_0.offset(0) as u32) << 24i32
        | (*in_0.offset(1) as u32) << 16i32
        | (*in_0.offset(2) as u32) << 8i32
        | *in_0.offset(3) as u32;
}
unsafe extern "C" fn buf_put_be32(mut _buf: *mut libc::c_void, mut val: u32) {
    let mut out: *mut uint8_t = _buf as *mut uint8_t;
    *out.offset(0) = (val >> 24i32) as uint8_t;
    *out.offset(1) = (val >> 16i32) as uint8_t;
    *out.offset(2) = (val >> 8i32) as uint8_t;
    *out.offset(3) = val as uint8_t;
}
unsafe extern "C" fn buf_get_be64(mut _buf: *const libc::c_void) -> u64 {
    let mut in_0: *const uint8_t = _buf as *const uint8_t;
    return (*in_0.offset(0) as u64) << 56i32
        | (*in_0.offset(1) as u64) << 48i32
        | (*in_0.offset(2) as u64) << 40i32
        | (*in_0.offset(3) as u64) << 32i32
        | (*in_0.offset(4) as u64) << 24i32
        | (*in_0.offset(5) as u64) << 16i32
        | (*in_0.offset(6) as u64) << 8i32
        | *in_0.offset(7) as u64;
}
unsafe extern "C" fn buf_put_be64(mut _buf: *mut libc::c_void, mut val: u64) {
    let mut out: *mut uint8_t = _buf as *mut uint8_t;
    *out.offset(0) = (val >> 56i32) as uint8_t;
    *out.offset(1) = (val >> 48i32) as uint8_t;
    *out.offset(2) = (val >> 40i32) as uint8_t;
    *out.offset(3) = (val >> 32i32) as uint8_t;
    *out.offset(4) = (val >> 24i32) as uint8_t;
    *out.offset(5) = (val >> 16i32) as uint8_t;
    *out.offset(6) = (val >> 8i32) as uint8_t;
    *out.offset(7) = val as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn SHA256_init(mut hd: *mut SHA256_CONTEXT) {
    (*hd).h0 = 0x6a09e667i32 as u32;
    (*hd).h1 = 0xbb67ae85u32;
    (*hd).h2 = 0x3c6ef372i32 as u32;
    (*hd).h3 = 0xa54ff53au32;
    (*hd).h4 = 0x510e527fi32 as u32;
    (*hd).h5 = 0x9b05688cu32;
    (*hd).h6 = 0x1f83d9abi32 as u32;
    (*hd).h7 = 0x5be0cd19i32 as u32;
    (*hd).nblocks = 0i32 as size_t;
    (*hd).count = 0i32;
}
/*
Transform the message X which consists of 16 32-bit-words. See FIPS
180-2 for details.  */
/* (4.6) */
/* (4.7) */
unsafe extern "C" fn _SHA256_transform(
    mut hd: *mut SHA256_CONTEXT,
    mut data: *const libc::c_uchar,
) -> libc::c_uint {
    static mut K: [u32; 64] = [
        0x428a2f98i32 as u32,
        0x71374491i32 as u32,
        0xb5c0fbcfu32,
        0xe9b5dba5u32,
        0x3956c25bi32 as u32,
        0x59f111f1i32 as u32,
        0x923f82a4u32,
        0xab1c5ed5u32,
        0xd807aa98u32,
        0x12835b01i32 as u32,
        0x243185bei32 as u32,
        0x550c7dc3i32 as u32,
        0x72be5d74i32 as u32,
        0x80deb1feu32,
        0x9bdc06a7u32,
        0xc19bf174u32,
        0xe49b69c1u32,
        0xefbe4786u32,
        0xfc19dc6i32 as u32,
        0x240ca1cci32 as u32,
        0x2de92c6fi32 as u32,
        0x4a7484aai32 as u32,
        0x5cb0a9dci32 as u32,
        0x76f988dai32 as u32,
        0x983e5152u32,
        0xa831c66du32,
        0xb00327c8u32,
        0xbf597fc7u32,
        0xc6e00bf3u32,
        0xd5a79147u32,
        0x6ca6351i32 as u32,
        0x14292967i32 as u32,
        0x27b70a85i32 as u32,
        0x2e1b2138i32 as u32,
        0x4d2c6dfci32 as u32,
        0x53380d13i32 as u32,
        0x650a7354i32 as u32,
        0x766a0abbi32 as u32,
        0x81c2c92eu32,
        0x92722c85u32,
        0xa2bfe8a1u32,
        0xa81a664bu32,
        0xc24b8b70u32,
        0xc76c51a3u32,
        0xd192e819u32,
        0xd6990624u32,
        0xf40e3585u32,
        0x106aa070i32 as u32,
        0x19a4c116i32 as u32,
        0x1e376c08i32 as u32,
        0x2748774ci32 as u32,
        0x34b0bcb5i32 as u32,
        0x391c0cb3i32 as u32,
        0x4ed8aa4ai32 as u32,
        0x5b9cca4fi32 as u32,
        0x682e6ff3i32 as u32,
        0x748f82eei32 as u32,
        0x78a5636fi32 as u32,
        0x84c87814u32,
        0x8cc70208u32,
        0x90befffau32,
        0xa4506cebu32,
        0xbef9a3f7u32,
        0xc67178f2u32,
    ];
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut e: u32 = 0;
    let mut f: u32 = 0;
    let mut g: u32 = 0;
    let mut h: u32 = 0;
    let mut t1: u32 = 0;
    let mut t2: u32 = 0;
    let mut w: [u32; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    a = (*hd).h0;
    b = (*hd).h1;
    c = (*hd).h2;
    d = (*hd).h3;
    e = (*hd).h4;
    f = (*hd).h5;
    g = (*hd).h6;
    h = (*hd).h7;
    i = 0i32;
    while i < 16i32 {
        w[i as usize] = buf_get_be32(data.offset((i * 4i32) as isize) as *const libc::c_void);
        i += 1
    }
    while i < 64i32 {
        w[i as usize] = ((w[(i - 2i32) as usize] >> (17i32 & 32i32 - 1i32)
            | w[(i - 2i32) as usize] << (32i32 - 17i32 & 32i32 - 1i32))
            ^ (w[(i - 2i32) as usize] >> (19i32 & 32i32 - 1i32)
                | w[(i - 2i32) as usize] << (32i32 - 19i32 & 32i32 - 1i32))
            ^ w[(i - 2i32) as usize] >> 10i32)
            .wrapping_add(w[(i - 7i32) as usize])
            .wrapping_add(
                (w[(i - 15i32) as usize] >> (7i32 & 32i32 - 1i32)
                    | w[(i - 15i32) as usize] << (32i32 - 7i32 & 32i32 - 1i32))
                    ^ (w[(i - 15i32) as usize] >> (18i32 & 32i32 - 1i32)
                        | w[(i - 15i32) as usize] << (32i32 - 18i32 & 32i32 - 1i32))
                    ^ w[(i - 15i32) as usize] >> 3i32,
            )
            .wrapping_add(w[(i - 16i32) as usize]);
        i += 1
    }
    i = 0i32;
    while i < 64i32 {
        t1 = h
            .wrapping_add(
                (e >> (6i32 & 32i32 - 1i32) | e << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (e >> (11i32 & 32i32 - 1i32) | e << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (e >> (25i32 & 32i32 - 1i32) | e << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(K[i as usize])
            .wrapping_add(w[i as usize]);
        t2 = ((a >> (2i32 & 32i32 - 1i32) | a << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (a >> (13i32 & 32i32 - 1i32) | a << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (a >> (22i32 & 32i32 - 1i32) | a << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(a & b | c & (a | b));
        d = (d as libc::c_uint).wrapping_add(t1) as u32;
        h = t1.wrapping_add(t2);
        t1 = g
            .wrapping_add(
                (d >> (6i32 & 32i32 - 1i32) | d << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (d >> (11i32 & 32i32 - 1i32) | d << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (d >> (25i32 & 32i32 - 1i32) | d << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(K[(i + 1i32) as usize])
            .wrapping_add(w[(i + 1i32) as usize]);
        t2 = ((h >> (2i32 & 32i32 - 1i32) | h << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (h >> (13i32 & 32i32 - 1i32) | h << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (h >> (22i32 & 32i32 - 1i32) | h << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(h & a | b & (h | a));
        c = (c as libc::c_uint).wrapping_add(t1) as u32;
        g = t1.wrapping_add(t2);
        t1 = f
            .wrapping_add(
                (c >> (6i32 & 32i32 - 1i32) | c << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (c >> (11i32 & 32i32 - 1i32) | c << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (c >> (25i32 & 32i32 - 1i32) | c << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(K[(i + 2i32) as usize])
            .wrapping_add(w[(i + 2i32) as usize]);
        t2 = ((g >> (2i32 & 32i32 - 1i32) | g << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (g >> (13i32 & 32i32 - 1i32) | g << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (g >> (22i32 & 32i32 - 1i32) | g << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(g & h | a & (g | h));
        b = (b as libc::c_uint).wrapping_add(t1) as u32;
        f = t1.wrapping_add(t2);
        t1 = e
            .wrapping_add(
                (b >> (6i32 & 32i32 - 1i32) | b << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (b >> (11i32 & 32i32 - 1i32) | b << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (b >> (25i32 & 32i32 - 1i32) | b << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(K[(i + 3i32) as usize])
            .wrapping_add(w[(i + 3i32) as usize]);
        t2 = ((f >> (2i32 & 32i32 - 1i32) | f << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (f >> (13i32 & 32i32 - 1i32) | f << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (f >> (22i32 & 32i32 - 1i32) | f << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(f & g | h & (f | g));
        a = (a as libc::c_uint).wrapping_add(t1) as u32;
        e = t1.wrapping_add(t2);
        t1 = d
            .wrapping_add(
                (a >> (6i32 & 32i32 - 1i32) | a << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (a >> (11i32 & 32i32 - 1i32) | a << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (a >> (25i32 & 32i32 - 1i32) | a << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(K[(i + 4i32) as usize])
            .wrapping_add(w[(i + 4i32) as usize]);
        t2 = ((e >> (2i32 & 32i32 - 1i32) | e << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (e >> (13i32 & 32i32 - 1i32) | e << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (e >> (22i32 & 32i32 - 1i32) | e << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(e & f | g & (e | f));
        h = (h as libc::c_uint).wrapping_add(t1) as u32;
        d = t1.wrapping_add(t2);
        t1 = c
            .wrapping_add(
                (h >> (6i32 & 32i32 - 1i32) | h << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (h >> (11i32 & 32i32 - 1i32) | h << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (h >> (25i32 & 32i32 - 1i32) | h << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(K[(i + 5i32) as usize])
            .wrapping_add(w[(i + 5i32) as usize]);
        t2 = ((d >> (2i32 & 32i32 - 1i32) | d << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (d >> (13i32 & 32i32 - 1i32) | d << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (d >> (22i32 & 32i32 - 1i32) | d << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(d & e | f & (d | e));
        g = (g as libc::c_uint).wrapping_add(t1) as u32;
        c = t1.wrapping_add(t2);
        t1 = b
            .wrapping_add(
                (g >> (6i32 & 32i32 - 1i32) | g << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (g >> (11i32 & 32i32 - 1i32) | g << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (g >> (25i32 & 32i32 - 1i32) | g << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(K[(i + 6i32) as usize])
            .wrapping_add(w[(i + 6i32) as usize]);
        t2 = ((c >> (2i32 & 32i32 - 1i32) | c << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (c >> (13i32 & 32i32 - 1i32) | c << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (c >> (22i32 & 32i32 - 1i32) | c << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(c & d | e & (c | d));
        f = (f as libc::c_uint).wrapping_add(t1) as u32;
        b = t1.wrapping_add(t2);
        t1 = a
            .wrapping_add(
                (f >> (6i32 & 32i32 - 1i32) | f << (32i32 - 6i32 & 32i32 - 1i32))
                    ^ (f >> (11i32 & 32i32 - 1i32) | f << (32i32 - 11i32 & 32i32 - 1i32))
                    ^ (f >> (25i32 & 32i32 - 1i32) | f << (32i32 - 25i32 & 32i32 - 1i32)),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(K[(i + 7i32) as usize])
            .wrapping_add(w[(i + 7i32) as usize]);
        t2 = ((b >> (2i32 & 32i32 - 1i32) | b << (32i32 - 2i32 & 32i32 - 1i32))
            ^ (b >> (13i32 & 32i32 - 1i32) | b << (32i32 - 13i32 & 32i32 - 1i32))
            ^ (b >> (22i32 & 32i32 - 1i32) | b << (32i32 - 22i32 & 32i32 - 1i32)))
            .wrapping_add(b & c | d & (b | c));
        e = (e as libc::c_uint).wrapping_add(t1) as u32;
        a = t1.wrapping_add(t2);
        i += 8i32
    }
    (*hd).h0 = ((*hd).h0 as libc::c_uint).wrapping_add(a) as u32;
    (*hd).h1 = ((*hd).h1 as libc::c_uint).wrapping_add(b) as u32;
    (*hd).h2 = ((*hd).h2 as libc::c_uint).wrapping_add(c) as u32;
    (*hd).h3 = ((*hd).h3 as libc::c_uint).wrapping_add(d) as u32;
    (*hd).h4 = ((*hd).h4 as libc::c_uint).wrapping_add(e) as u32;
    (*hd).h5 = ((*hd).h5 as libc::c_uint).wrapping_add(f) as u32;
    (*hd).h6 = ((*hd).h6 as libc::c_uint).wrapping_add(g) as u32;
    (*hd).h7 = ((*hd).h7 as libc::c_uint).wrapping_add(h) as u32;
    return (74i32 * 4i32 + 32i32) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn SHA256_write(
    mut hd: *mut SHA256_CONTEXT,
    mut inbuf: *const libc::c_uchar,
    mut inlen: libc::c_uint,
) {
    let mut stack_burn: libc::c_uint = 0i32 as libc::c_uint;
    if (*hd).count == 64i32 {
        /* flush the buffer */
        stack_burn = _SHA256_transform(hd, (*hd).buf.as_mut_ptr());
        _gcry_burn_stack(stack_burn as libc::c_int);
        (*hd).count = 0i32;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1)
    }
    if inbuf.is_null() {
        return;
    }
    if (*hd).count != 0 {
        while inlen != 0 && (*hd).count < 64i32 {
            let fresh24 = inbuf;
            inbuf = inbuf.offset(1);
            let fresh25 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh25 as usize] = *fresh24;
            inlen = inlen.wrapping_sub(1)
        }
        SHA256_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
        if inlen == 0 {
            return;
        }
    }
    _gcry_burn_stack(stack_burn as libc::c_int);
    while inlen >= 64i32 as libc::c_uint {
        stack_burn = _SHA256_transform(hd, inbuf);
        (*hd).count = 0i32;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
        inlen = inlen.wrapping_sub(64i32 as libc::c_uint);
        inbuf = inbuf.offset(64)
    }
    while inlen != 0 && (*hd).count < 64i32 {
        let fresh26 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh27 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh27 as usize] = *fresh26;
        inlen = inlen.wrapping_sub(1)
    }
}
/*
The routine finally terminates the computation and returns the
digest.  The handle is prepared for a new cycle, but adding bytes
to the handle will the destroy the returned buffer.  Returns: 32
bytes with the message the digest.  */
#[no_mangle]
pub unsafe extern "C" fn SHA256_final(mut outbuf: *mut libc::c_uchar, mut hd: *mut SHA256_CONTEXT) {
    let mut t: u32 = 0;
    let mut msb: u32 = 0;
    let mut lsb: u32 = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut burn: libc::c_uint = 0;
    SHA256_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
    /* flush */
    t = (*hd).nblocks as u32;
    /* multiply by 64 to make a byte count */
    lsb = t << 6i32;
    msb = t >> 26i32;
    /* add the count */
    t = lsb;
    lsb = (lsb as libc::c_uint).wrapping_add((*hd).count as libc::c_uint) as u32;
    if lsb < t {
        msb = msb.wrapping_add(1)
    }
    /* multiply by 8 to make a bit count */
    t = lsb;
    lsb <<= 3i32;
    msb <<= 3i32;
    msb |= t >> 29i32;
    if (*hd).count < 56i32 {
        /* enough room */
        let fresh28 = (*hd).count; /* pad */
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh28 as usize] = 0x80i32 as libc::c_uchar;
        while (*hd).count < 56i32 {
            let fresh29 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh29 as usize] = 0i32 as libc::c_uchar
        }
    /* pad */
    } else {
        /* need one extra block */
        let fresh30 = (*hd).count; /* pad character */
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh30 as usize] = 0x80i32 as libc::c_uchar;
        while (*hd).count < 64i32 {
            let fresh31 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh31 as usize] = 0i32 as libc::c_uchar
        }
        SHA256_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
        /* fill next block with zeroes */
        memset(
            (*hd).buf.as_mut_ptr() as *mut libc::c_void,
            0i32,
            56i32 as u64,
        );
    }
    /* flush */
    /* append the 64 bit count */
    buf_put_be32((*hd).buf.as_mut_ptr().offset(56) as *mut libc::c_void, msb);
    buf_put_be32((*hd).buf.as_mut_ptr().offset(60) as *mut libc::c_void, lsb);
    burn = _SHA256_transform(hd, (*hd).buf.as_mut_ptr());
    _gcry_burn_stack(burn as libc::c_int);
    p = outbuf;
    *(p as *mut u32) = _gcry_bswap32((*hd).h0);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h1);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h2);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h3);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h4);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h5);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h6);
    p = p.offset(4);
    *(p as *mut u32) = _gcry_bswap32((*hd).h7);
    p = p.offset(4);
}
/* The following code are taken from libgcrypt-1.6.3. (slightly modified):
 *
 * sha512.c - SHA384 and SHA512 hash functions
 * Copyright (C) 2003, 2008, 2009 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser general Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 */
#[no_mangle]
pub unsafe extern "C" fn SHA512_init(mut ctx: *mut SHA512_CONTEXT) {
    let mut hd: *mut SHA512_STATE = &mut (*ctx).state;
    (*hd).h0 = 0x6a09e667f3bcc908u64;
    (*hd).h1 = 0xbb67ae8584caa73bu64;
    (*hd).h2 = 0x3c6ef372fe94f82bu64;
    (*hd).h3 = 0xa54ff53a5f1d36f1u64;
    (*hd).h4 = 0x510e527fade682d1u64;
    (*hd).h5 = 0x9b05688c2b3e6c1fu64;
    (*hd).h6 = 0x1f83d9abfb41bd6bu64;
    (*hd).h7 = 0x5be0cd19137e2179u64;
    (*ctx).nblocks = 0i32 as size_t;
    (*ctx).count = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn SHA384_init(mut ctx: *mut SHA512_CONTEXT) {
    let mut hd: *mut SHA512_STATE = &mut (*ctx).state;
    (*hd).h0 = 0xcbbb9d5dc1059ed8u64;
    (*hd).h1 = 0x629a292a367cd507u64;
    (*hd).h2 = 0x9159015a3070dd17u64;
    (*hd).h3 = 0x152fecd8f70e5939u64;
    (*hd).h4 = 0x67332667ffc00b31u64;
    (*hd).h5 = 0x8eb44a8768581511u64;
    (*hd).h6 = 0xdb0c2e0d64f98fa7u64;
    (*hd).h7 = 0x47b5481dbefa4fa4u64;
    (*ctx).nblocks = 0i32 as size_t;
    (*ctx).count = 0i32;
}
static mut k: [u64; 80] = [
    0x428a2f98d728ae22u64,
    0x7137449123ef65cdu64,
    0xb5c0fbcfec4d3b2fu64,
    0xe9b5dba58189dbbcu64,
    0x3956c25bf348b538u64,
    0x59f111f1b605d019u64,
    0x923f82a4af194f9bu64,
    0xab1c5ed5da6d8118u64,
    0xd807aa98a3030242u64,
    0x12835b0145706fbeu64,
    0x243185be4ee4b28cu64,
    0x550c7dc3d5ffb4e2u64,
    0x72be5d74f27b896fu64,
    0x80deb1fe3b1696b1u64,
    0x9bdc06a725c71235u64,
    0xc19bf174cf692694u64,
    0xe49b69c19ef14ad2u64,
    0xefbe4786384f25e3u64,
    0xfc19dc68b8cd5b5u64,
    0x240ca1cc77ac9c65u64,
    0x2de92c6f592b0275u64,
    0x4a7484aa6ea6e483u64,
    0x5cb0a9dcbd41fbd4u64,
    0x76f988da831153b5u64,
    0x983e5152ee66dfabu64,
    0xa831c66d2db43210u64,
    0xb00327c898fb213fu64,
    0xbf597fc7beef0ee4u64,
    0xc6e00bf33da88fc2u64,
    0xd5a79147930aa725u64,
    0x6ca6351e003826fu64,
    0x142929670a0e6e70u64,
    0x27b70a8546d22ffcu64,
    0x2e1b21385c26c926u64,
    0x4d2c6dfc5ac42aedu64,
    0x53380d139d95b3dfu64,
    0x650a73548baf63deu64,
    0x766a0abb3c77b2a8u64,
    0x81c2c92e47edaee6u64,
    0x92722c851482353bu64,
    0xa2bfe8a14cf10364u64,
    0xa81a664bbc423001u64,
    0xc24b8b70d0f89791u64,
    0xc76c51a30654be30u64,
    0xd192e819d6ef5218u64,
    0xd69906245565a910u64,
    0xf40e35855771202au64,
    0x106aa07032bbd1b8u64,
    0x19a4c116b8d2d0c8u64,
    0x1e376c085141ab53u64,
    0x2748774cdf8eeb99u64,
    0x34b0bcb5e19b48a8u64,
    0x391c0cb3c5c95a63u64,
    0x4ed8aa4ae3418acbu64,
    0x5b9cca4f7763e373u64,
    0x682e6ff3d6b2b8a3u64,
    0x748f82ee5defb2fcu64,
    0x78a5636f43172f60u64,
    0x84c87814a1f0ab72u64,
    0x8cc702081a6439ecu64,
    0x90befffa23631e28u64,
    0xa4506cebde82bde9u64,
    0xbef9a3f7b2c67915u64,
    0xc67178f2e372532bu64,
    0xca273eceea26619cu64,
    0xd186b8c721c0c207u64,
    0xeada7dd6cde0eb1eu64,
    0xf57d4f7fee6ed178u64,
    0x6f067aa72176fbau64,
    0xa637dc5a2c898a6u64,
    0x113f9804bef90daeu64,
    0x1b710b35131c471bu64,
    0x28db77f523047d84u64,
    0x32caab7b40c72493u64,
    0x3c9ebe0a15c9bebcu64,
    0x431d67c49c100d4cu64,
    0x4cc5d4becb3e42b6u64,
    0x597f299cfc657e2au64,
    0x5fcb6fab3ad6faecu64,
    0x6c44198c4a475817u64,
];
/* ***************
 * Transform the message W which consists of 16 64-bit-words
 */
unsafe extern "C" fn __transform(
    mut hd: *mut SHA512_STATE,
    mut data: *const libc::c_uchar,
) -> libc::c_uint {
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut d: u64 = 0;
    let mut e: u64 = 0;
    let mut f: u64 = 0;
    let mut g: u64 = 0;
    let mut h: u64 = 0;
    let mut w: [u64; 16] = [0; 16];
    let mut t: libc::c_int = 0;
    /* get values from the chaining vars */
    a = (*hd).h0;
    b = (*hd).h1;
    c = (*hd).h2;
    d = (*hd).h3;
    e = (*hd).h4;
    f = (*hd).h5;
    g = (*hd).h6;
    h = (*hd).h7;
    t = 0i32;
    while t < 16i32 {
        w[t as usize] = buf_get_be64(data.offset((t * 8i32) as isize) as *const libc::c_void);
        t += 1
    }
    t = 0i32;
    while t < 80i32 - 16i32 {
        let mut t1: u64 = 0;
        let mut t2: u64 = 0;
        /* Performance on a AMD Athlon(tm) Dual Core Processor 4050e
           with gcc 4.3.3 using gcry_md_hash_buffer of each 10000 bytes
           initialized to 0,1,2,3...255,0,... and 1000 iterations:

           Not unrolled with macros:  440ms
           Unrolled with macros:      350ms
           Unrolled with inline:      330ms
        */
        /* Not unrolled.  */
        /* Unrolled to interweave the chain variables.  */
        t1 = h
            .wrapping_add(
                (e >> 14i32 | e << 64i32 - 14i32)
                    ^ (e >> 18i32 | e << 64i32 - 18i32)
                    ^ (e >> 41i32 | e << 64i32 - 41i32),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[t as usize])
            .wrapping_add(w[0]);
        w[0] = (w[0] as u64).wrapping_add(
            ((w[14] >> 19i32 | w[14] << 64i32 - 19i32)
                ^ (w[14] >> 61i32 | w[14] << 64i32 - 61i32)
                ^ w[14] >> 6i32)
                .wrapping_add(w[9])
                .wrapping_add(
                    (w[1] >> 1i32 | w[1] << 64i32 - 1i32)
                        ^ (w[1] >> 8i32 | w[1] << 64i32 - 8i32)
                        ^ w[1] >> 7i32,
                ),
        ) as u64;
        t2 = ((a >> 28i32 | a << 64i32 - 28i32)
            ^ (a >> 34i32 | a << 64i32 - 34i32)
            ^ (a >> 39i32 | a << 64i32 - 39i32))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = (d as u64).wrapping_add(t1) as u64;
        h = t1.wrapping_add(t2);
        t1 = g
            .wrapping_add(
                (d >> 14i32 | d << 64i32 - 14i32)
                    ^ (d >> 18i32 | d << 64i32 - 18i32)
                    ^ (d >> 41i32 | d << 64i32 - 41i32),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 1i32) as usize])
            .wrapping_add(w[1]);
        w[1] = (w[1] as u64).wrapping_add(
            ((w[15] >> 19i32 | w[15] << 64i32 - 19i32)
                ^ (w[15] >> 61i32 | w[15] << 64i32 - 61i32)
                ^ w[15] >> 6i32)
                .wrapping_add(w[10])
                .wrapping_add(
                    (w[2] >> 1i32 | w[2] << 64i32 - 1i32)
                        ^ (w[2] >> 8i32 | w[2] << 64i32 - 8i32)
                        ^ w[2] >> 7i32,
                ),
        ) as u64;
        t2 = ((h >> 28i32 | h << 64i32 - 28i32)
            ^ (h >> 34i32 | h << 64i32 - 34i32)
            ^ (h >> 39i32 | h << 64i32 - 39i32))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = (c as u64).wrapping_add(t1) as u64;
        g = t1.wrapping_add(t2);
        t1 = f
            .wrapping_add(
                (c >> 14i32 | c << 64i32 - 14i32)
                    ^ (c >> 18i32 | c << 64i32 - 18i32)
                    ^ (c >> 41i32 | c << 64i32 - 41i32),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 2i32) as usize])
            .wrapping_add(w[2]);
        w[2] = (w[2] as u64).wrapping_add(
            ((w[0] >> 19i32 | w[0] << 64i32 - 19i32)
                ^ (w[0] >> 61i32 | w[0] << 64i32 - 61i32)
                ^ w[0] >> 6i32)
                .wrapping_add(w[11])
                .wrapping_add(
                    (w[3] >> 1i32 | w[3] << 64i32 - 1i32)
                        ^ (w[3] >> 8i32 | w[3] << 64i32 - 8i32)
                        ^ w[3] >> 7i32,
                ),
        ) as u64;
        t2 = ((g >> 28i32 | g << 64i32 - 28i32)
            ^ (g >> 34i32 | g << 64i32 - 34i32)
            ^ (g >> 39i32 | g << 64i32 - 39i32))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = (b as u64).wrapping_add(t1) as u64;
        f = t1.wrapping_add(t2);
        t1 = e
            .wrapping_add(
                (b >> 14i32 | b << 64i32 - 14i32)
                    ^ (b >> 18i32 | b << 64i32 - 18i32)
                    ^ (b >> 41i32 | b << 64i32 - 41i32),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 3i32) as usize])
            .wrapping_add(w[3]);
        w[3] = (w[3] as u64).wrapping_add(
            ((w[1] >> 19i32 | w[1] << 64i32 - 19i32)
                ^ (w[1] >> 61i32 | w[1] << 64i32 - 61i32)
                ^ w[1] >> 6i32)
                .wrapping_add(w[12])
                .wrapping_add(
                    (w[4] >> 1i32 | w[4] << 64i32 - 1i32)
                        ^ (w[4] >> 8i32 | w[4] << 64i32 - 8i32)
                        ^ w[4] >> 7i32,
                ),
        ) as u64;
        t2 = ((f >> 28i32 | f << 64i32 - 28i32)
            ^ (f >> 34i32 | f << 64i32 - 34i32)
            ^ (f >> 39i32 | f << 64i32 - 39i32))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = a.wrapping_add(t1) as u64;
        e = t1.wrapping_add(t2);
        t1 = d
            .wrapping_add(
                (a >> 14i32 | a << 64i32 - 14i32)
                    ^ (a >> 18i32 | a << 64i32 - 18i32)
                    ^ (a >> 41i32 | a << 64i32 - 41i32),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 4i32) as usize])
            .wrapping_add(w[4]);
        w[4] = (w[4] as u64).wrapping_add(
            ((w[2] >> 19i32 | w[2] << 64i32 - 19i32)
                ^ (w[2] >> 61i32 | w[2] << 64i32 - 61i32)
                ^ w[2] >> 6i32)
                .wrapping_add(w[13])
                .wrapping_add(
                    (w[5] >> 1i32 | w[5] << 64i32 - 1i32)
                        ^ (w[5] >> 8i32 | w[5] << 64i32 - 8i32)
                        ^ w[5] >> 7i32,
                ),
        ) as u64;
        t2 = ((e >> 28i32 | e << 64i32 - 28i32)
            ^ (e >> 34i32 | e << 64i32 - 34i32)
            ^ (e >> 39i32 | e << 64i32 - 39i32))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = h.wrapping_add(t1) as u64;
        d = t1.wrapping_add(t2);
        t1 = c
            .wrapping_add(
                (h >> 14i32 | h << 64i32 - 14i32)
                    ^ (h >> 18i32 | h << 64i32 - 18i32)
                    ^ (h >> 41i32 | h << 64i32 - 41i32),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 5i32) as usize])
            .wrapping_add(w[5]);
        w[5] = (w[5] as u64).wrapping_add(
            ((w[3] >> 19i32 | w[3] << 64i32 - 19i32)
                ^ (w[3] >> 61i32 | w[3] << 64i32 - 61i32)
                ^ w[3] >> 6i32)
                .wrapping_add(w[14])
                .wrapping_add(
                    (w[6] >> 1i32 | w[6] << 64i32 - 1i32)
                        ^ (w[6] >> 8i32 | w[6] << 64i32 - 8i32)
                        ^ w[6] >> 7i32,
                ),
        ) as u64;
        t2 = ((d >> 28i32 | d << 64i32 - 28i32)
            ^ (d >> 34i32 | d << 64i32 - 34i32)
            ^ (d >> 39i32 | d << 64i32 - 39i32))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = g.wrapping_add(t1) as u64;
        c = t1.wrapping_add(t2);
        t1 = b
            .wrapping_add(
                (g >> 14i32 | g << 64i32 - 14i32)
                    ^ (g >> 18i32 | g << 64i32 - 18i32)
                    ^ (g >> 41i32 | g << 64i32 - 41i32),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 6i32) as usize])
            .wrapping_add(w[6]);
        w[6] = (w[6] as u64).wrapping_add(
            ((w[4] >> 19i32 | w[4] << 64i32 - 19i32)
                ^ (w[4] >> 61i32 | w[4] << 64i32 - 61i32)
                ^ w[4] >> 6i32)
                .wrapping_add(w[15])
                .wrapping_add(
                    (w[7] >> 1i32 | w[7] << 64i32 - 1i32)
                        ^ (w[7] >> 8i32 | w[7] << 64i32 - 8i32)
                        ^ w[7] >> 7i32,
                ),
        ) as u64;
        t2 = ((c >> 28i32 | c << 64i32 - 28i32)
            ^ (c >> 34i32 | c << 64i32 - 34i32)
            ^ (c >> 39i32 | c << 64i32 - 39i32))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = (f as u64).wrapping_add(t1) as u64;
        b = t1.wrapping_add(t2);
        t1 = a
            .wrapping_add(
                (f >> 14i32 | f << 64i32 - 14i32)
                    ^ (f >> 18i32 | f << 64i32 - 18i32)
                    ^ (f >> 41i32 | f << 64i32 - 41i32),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 7i32) as usize])
            .wrapping_add(w[7]);
        w[7] = (w[7] as u64).wrapping_add(
            ((w[5] >> 19i32 | w[5] << 64i32 - 19i32)
                ^ (w[5] >> 61i32 | w[5] << 64i32 - 61i32)
                ^ w[5] >> 6i32)
                .wrapping_add(w[0])
                .wrapping_add(
                    (w[8] >> 1i32 | w[8] << 64i32 - 1i32)
                        ^ (w[8] >> 8i32 | w[8] << 64i32 - 8i32)
                        ^ w[8] >> 7i32,
                ),
        ) as u64;
        t2 = ((b >> 28i32 | b << 64i32 - 28i32)
            ^ (b >> 34i32 | b << 64i32 - 34i32)
            ^ (b >> 39i32 | b << 64i32 - 39i32))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = (e as u64).wrapping_add(t1) as u64;
        a = t1.wrapping_add(t2);
        t1 = h
            .wrapping_add(
                (e >> 14i32 | e << 64i32 - 14i32)
                    ^ (e >> 18i32 | e << 64i32 - 18i32)
                    ^ (e >> 41i32 | e << 64i32 - 41i32),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[(t + 8i32) as usize])
            .wrapping_add(w[8]);
        w[8] = (w[8] as u64).wrapping_add(
            ((w[6] >> 19i32 | w[6] << 64i32 - 19i32)
                ^ (w[6] >> 61i32 | w[6] << 64i32 - 61i32)
                ^ w[6] >> 6i32)
                .wrapping_add(w[1])
                .wrapping_add(
                    (w[9] >> 1i32 | w[9] << 64i32 - 1i32)
                        ^ (w[9] >> 8i32 | w[9] << 64i32 - 8i32)
                        ^ w[9] >> 7i32,
                ),
        ) as u64;
        t2 = ((a >> 28i32 | a << 64i32 - 28i32)
            ^ (a >> 34i32 | a << 64i32 - 34i32)
            ^ (a >> 39i32 | a << 64i32 - 39i32))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = (d as u64).wrapping_add(t1) as u64;
        h = t1.wrapping_add(t2);
        t1 = g
            .wrapping_add(
                (d >> 14i32 | d << 64i32 - 14i32)
                    ^ (d >> 18i32 | d << 64i32 - 18i32)
                    ^ (d >> 41i32 | d << 64i32 - 41i32),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 9i32) as usize])
            .wrapping_add(w[9]);
        w[9] = (w[9] as u64).wrapping_add(
            ((w[7] >> 19i32 | w[7] << 64i32 - 19i32)
                ^ (w[7] >> 61i32 | w[7] << 64i32 - 61i32)
                ^ w[7] >> 6i32)
                .wrapping_add(w[2])
                .wrapping_add(
                    (w[10] >> 1i32 | w[10] << 64i32 - 1i32)
                        ^ (w[10] >> 8i32 | w[10] << 64i32 - 8i32)
                        ^ w[10] >> 7i32,
                ),
        ) as u64;
        t2 = ((h >> 28i32 | h << 64i32 - 28i32)
            ^ (h >> 34i32 | h << 64i32 - 34i32)
            ^ (h >> 39i32 | h << 64i32 - 39i32))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = (c as u64).wrapping_add(t1) as u64;
        g = t1.wrapping_add(t2);
        t1 = f
            .wrapping_add(
                (c >> 14i32 | c << 64i32 - 14i32)
                    ^ (c >> 18i32 | c << 64i32 - 18i32)
                    ^ (c >> 41i32 | c << 64i32 - 41i32),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 10i32) as usize])
            .wrapping_add(w[10]);
        w[10] = (w[10] as u64).wrapping_add(
            ((w[8] >> 19i32 | w[8] << 64i32 - 19i32)
                ^ (w[8] >> 61i32 | w[8] << 64i32 - 61i32)
                ^ w[8] >> 6i32)
                .wrapping_add(w[3])
                .wrapping_add(
                    (w[11] >> 1i32 | w[11] << 64i32 - 1i32)
                        ^ (w[11] >> 8i32 | w[11] << 64i32 - 8i32)
                        ^ w[11] >> 7i32,
                ),
        ) as u64;
        t2 = ((g >> 28i32 | g << 64i32 - 28i32)
            ^ (g >> 34i32 | g << 64i32 - 34i32)
            ^ (g >> 39i32 | g << 64i32 - 39i32))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = (b as u64).wrapping_add(t1) as u64;
        f = t1.wrapping_add(t2);
        t1 = e
            .wrapping_add(
                (b >> 14i32 | b << 64i32 - 14i32)
                    ^ (b >> 18i32 | b << 64i32 - 18i32)
                    ^ (b >> 41i32 | b << 64i32 - 41i32),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 11i32) as usize])
            .wrapping_add(w[11]);
        w[11] = (w[11] as u64).wrapping_add(
            ((w[9] >> 19i32 | w[9] << 64i32 - 19i32)
                ^ (w[9] >> 61i32 | w[9] << 64i32 - 61i32)
                ^ w[9] >> 6i32)
                .wrapping_add(w[4])
                .wrapping_add(
                    (w[12] >> 1i32 | w[12] << 64i32 - 1i32)
                        ^ (w[12] >> 8i32 | w[12] << 64i32 - 8i32)
                        ^ w[12] >> 7i32,
                ),
        ) as u64;
        t2 = ((f >> 28i32 | f << 64i32 - 28i32)
            ^ (f >> 34i32 | f << 64i32 - 34i32)
            ^ (f >> 39i32 | f << 64i32 - 39i32))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = (a as u64).wrapping_add(t1) as u64;
        e = t1.wrapping_add(t2);
        t1 = d
            .wrapping_add(
                (a >> 14i32 | a << 64i32 - 14i32)
                    ^ (a >> 18i32 | a << 64i32 - 18i32)
                    ^ (a >> 41i32 | a << 64i32 - 41i32),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 12i32) as usize])
            .wrapping_add(w[12]);
        w[12] = (w[12] as u64).wrapping_add(
            ((w[10] >> 19i32 | w[10] << 64i32 - 19i32)
                ^ (w[10] >> 61i32 | w[10] << 64i32 - 61i32)
                ^ w[10] >> 6i32)
                .wrapping_add(w[5])
                .wrapping_add(
                    (w[13] >> 1i32 | w[13] << 64i32 - 1i32)
                        ^ (w[13] >> 8i32 | w[13] << 64i32 - 8i32)
                        ^ w[13] >> 7i32,
                ),
        ) as u64;
        t2 = ((e >> 28i32 | e << 64i32 - 28i32)
            ^ (e >> 34i32 | e << 64i32 - 34i32)
            ^ (e >> 39i32 | e << 64i32 - 39i32))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = (h as u64).wrapping_add(t1) as u64;
        d = t1.wrapping_add(t2);
        t1 = c
            .wrapping_add(
                (h >> 14i32 | h << 64i32 - 14i32)
                    ^ (h >> 18i32 | h << 64i32 - 18i32)
                    ^ (h >> 41i32 | h << 64i32 - 41i32),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 13i32) as usize])
            .wrapping_add(w[13]);
        w[13] = (w[13] as u64).wrapping_add(
            ((w[11] >> 19i32 | w[11] << 64i32 - 19i32)
                ^ (w[11] >> 61i32 | w[11] << 64i32 - 61i32)
                ^ w[11] >> 6i32)
                .wrapping_add(w[6])
                .wrapping_add(
                    (w[14] >> 1i32 | w[14] << 64i32 - 1i32)
                        ^ (w[14] >> 8i32 | w[14] << 64i32 - 8i32)
                        ^ w[14] >> 7i32,
                ),
        ) as u64;
        t2 = ((d >> 28i32 | d << 64i32 - 28i32)
            ^ (d >> 34i32 | d << 64i32 - 34i32)
            ^ (d >> 39i32 | d << 64i32 - 39i32))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = (g as u64).wrapping_add(t1) as u64;
        c = t1.wrapping_add(t2);
        t1 = b
            .wrapping_add(
                (g >> 14i32 | g << 64i32 - 14i32)
                    ^ (g >> 18i32 | g << 64i32 - 18i32)
                    ^ (g >> 41i32 | g << 64i32 - 41i32),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 14i32) as usize])
            .wrapping_add(w[14]);
        w[14] = (w[14] as u64).wrapping_add(
            ((w[12] >> 19i32 | w[12] << 64i32 - 19i32)
                ^ (w[12] >> 61i32 | w[12] << 64i32 - 61i32)
                ^ w[12] >> 6i32)
                .wrapping_add(w[7])
                .wrapping_add(
                    (w[15] >> 1i32 | w[15] << 64i32 - 1i32)
                        ^ (w[15] >> 8i32 | w[15] << 64i32 - 8i32)
                        ^ w[15] >> 7i32,
                ),
        ) as u64;
        t2 = ((c >> 28i32 | c << 64i32 - 28i32)
            ^ (c >> 34i32 | c << 64i32 - 34i32)
            ^ (c >> 39i32 | c << 64i32 - 39i32))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = (f as u64).wrapping_add(t1) as u64;
        b = t1.wrapping_add(t2);
        t1 = a
            .wrapping_add(
                (f >> 14i32 | f << 64i32 - 14i32)
                    ^ (f >> 18i32 | f << 64i32 - 18i32)
                    ^ (f >> 41i32 | f << 64i32 - 41i32),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 15i32) as usize])
            .wrapping_add(w[15]);
        w[15] = (w[15] as u64).wrapping_add(
            ((w[13] >> 19i32 | w[13] << 64i32 - 19i32)
                ^ (w[13] >> 61i32 | w[13] << 64i32 - 61i32)
                ^ w[13] >> 6i32)
                .wrapping_add(w[8])
                .wrapping_add(
                    (w[0] >> 1i32 | w[0] << 64i32 - 1i32)
                        ^ (w[0] >> 8i32 | w[0] << 64i32 - 8i32)
                        ^ w[0] >> 7i32,
                ),
        ) as u64;
        t2 = ((b >> 28i32 | b << 64i32 - 28i32)
            ^ (b >> 34i32 | b << 64i32 - 34i32)
            ^ (b >> 39i32 | b << 64i32 - 39i32))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = (e as u64).wrapping_add(t1) as u64;
        a = t1.wrapping_add(t2);
        t += 16i32
    }
    while t < 80i32 {
        let mut t1_0: u64 = 0;
        let mut t2_0: u64 = 0;
        /* Not unrolled.  */
        /* Unrolled to interweave the chain variables.  */
        t1_0 = h
            .wrapping_add(
                (e >> 14i32 | e << 64i32 - 14i32)
                    ^ (e >> 18i32 | e << 64i32 - 18i32)
                    ^ (e >> 41i32 | e << 64i32 - 41i32),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[t as usize])
            .wrapping_add(w[0]);
        t2_0 = ((a >> 28i32 | a << 64i32 - 28i32)
            ^ (a >> 34i32 | a << 64i32 - 34i32)
            ^ (a >> 39i32 | a << 64i32 - 39i32))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = (d as u64).wrapping_add(t1_0) as u64;
        h = t1_0.wrapping_add(t2_0);
        t1_0 = g
            .wrapping_add(
                (d >> 14i32 | d << 64i32 - 14i32)
                    ^ (d >> 18i32 | d << 64i32 - 18i32)
                    ^ (d >> 41i32 | d << 64i32 - 41i32),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 1i32) as usize])
            .wrapping_add(w[1]);
        t2_0 = ((h >> 28i32 | h << 64i32 - 28i32)
            ^ (h >> 34i32 | h << 64i32 - 34i32)
            ^ (h >> 39i32 | h << 64i32 - 39i32))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = (c as u64).wrapping_add(t1_0) as u64;
        g = t1_0.wrapping_add(t2_0);
        t1_0 = f
            .wrapping_add(
                (c >> 14i32 | c << 64i32 - 14i32)
                    ^ (c >> 18i32 | c << 64i32 - 18i32)
                    ^ (c >> 41i32 | c << 64i32 - 41i32),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 2i32) as usize])
            .wrapping_add(w[2]);
        t2_0 = ((g >> 28i32 | g << 64i32 - 28i32)
            ^ (g >> 34i32 | g << 64i32 - 34i32)
            ^ (g >> 39i32 | g << 64i32 - 39i32))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = (b as u64).wrapping_add(t1_0) as u64;
        f = t1_0.wrapping_add(t2_0);
        t1_0 = e
            .wrapping_add(
                (b >> 14i32 | b << 64i32 - 14i32)
                    ^ (b >> 18i32 | b << 64i32 - 18i32)
                    ^ (b >> 41i32 | b << 64i32 - 41i32),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 3i32) as usize])
            .wrapping_add(w[3]);
        t2_0 = ((f >> 28i32 | f << 64i32 - 28i32)
            ^ (f >> 34i32 | f << 64i32 - 34i32)
            ^ (f >> 39i32 | f << 64i32 - 39i32))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = (a as u64).wrapping_add(t1_0) as u64;
        e = t1_0.wrapping_add(t2_0);
        t1_0 = d
            .wrapping_add(
                (a >> 14i32 | a << 64i32 - 14i32)
                    ^ (a >> 18i32 | a << 64i32 - 18i32)
                    ^ (a >> 41i32 | a << 64i32 - 41i32),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 4i32) as usize])
            .wrapping_add(w[4]);
        t2_0 = ((e >> 28i32 | e << 64i32 - 28i32)
            ^ (e >> 34i32 | e << 64i32 - 34i32)
            ^ (e >> 39i32 | e << 64i32 - 39i32))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = (h as u64).wrapping_add(t1_0) as u64;
        d = t1_0.wrapping_add(t2_0);
        t1_0 = c
            .wrapping_add(
                (h >> 14i32 | h << 64i32 - 14i32)
                    ^ (h >> 18i32 | h << 64i32 - 18i32)
                    ^ (h >> 41i32 | h << 64i32 - 41i32),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 5i32) as usize])
            .wrapping_add(w[5]);
        t2_0 = ((d >> 28i32 | d << 64i32 - 28i32)
            ^ (d >> 34i32 | d << 64i32 - 34i32)
            ^ (d >> 39i32 | d << 64i32 - 39i32))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = (g as u64).wrapping_add(t1_0) as u64;
        c = t1_0.wrapping_add(t2_0);
        t1_0 = b
            .wrapping_add(
                (g >> 14i32 | g << 64i32 - 14i32)
                    ^ (g >> 18i32 | g << 64i32 - 18i32)
                    ^ (g >> 41i32 | g << 64i32 - 41i32),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 6i32) as usize])
            .wrapping_add(w[6]);
        t2_0 = ((c >> 28i32 | c << 64i32 - 28i32)
            ^ (c >> 34i32 | c << 64i32 - 34i32)
            ^ (c >> 39i32 | c << 64i32 - 39i32))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = (f as u64).wrapping_add(t1_0) as u64;
        b = t1_0.wrapping_add(t2_0);
        t1_0 = a
            .wrapping_add(
                (f >> 14i32 | f << 64i32 - 14i32)
                    ^ (f >> 18i32 | f << 64i32 - 18i32)
                    ^ (f >> 41i32 | f << 64i32 - 41i32),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 7i32) as usize])
            .wrapping_add(w[7]);
        t2_0 = ((b >> 28i32 | b << 64i32 - 28i32)
            ^ (b >> 34i32 | b << 64i32 - 34i32)
            ^ (b >> 39i32 | b << 64i32 - 39i32))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = (e as u64).wrapping_add(t1_0) as u64;
        a = t1_0.wrapping_add(t2_0);
        t1_0 = h
            .wrapping_add(
                (e >> 14i32 | e << 64i32 - 14i32)
                    ^ (e >> 18i32 | e << 64i32 - 18i32)
                    ^ (e >> 41i32 | e << 64i32 - 41i32),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[(t + 8i32) as usize])
            .wrapping_add(w[8]);
        t2_0 = ((a >> 28i32 | a << 64i32 - 28i32)
            ^ (a >> 34i32 | a << 64i32 - 34i32)
            ^ (a >> 39i32 | a << 64i32 - 39i32))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = (d as u64).wrapping_add(t1_0) as u64;
        h = t1_0.wrapping_add(t2_0);
        t1_0 = g
            .wrapping_add(
                (d >> 14i32 | d << 64i32 - 14i32)
                    ^ (d >> 18i32 | d << 64i32 - 18i32)
                    ^ (d >> 41i32 | d << 64i32 - 41i32),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 9i32) as usize])
            .wrapping_add(w[9]);
        t2_0 = ((h >> 28i32 | h << 64i32 - 28i32)
            ^ (h >> 34i32 | h << 64i32 - 34i32)
            ^ (h >> 39i32 | h << 64i32 - 39i32))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = (c as u64).wrapping_add(t1_0) as u64;
        g = t1_0.wrapping_add(t2_0);
        t1_0 = f
            .wrapping_add(
                (c >> 14i32 | c << 64i32 - 14i32)
                    ^ (c >> 18i32 | c << 64i32 - 18i32)
                    ^ (c >> 41i32 | c << 64i32 - 41i32),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 10i32) as usize])
            .wrapping_add(w[10]);
        t2_0 = ((g >> 28i32 | g << 64i32 - 28i32)
            ^ (g >> 34i32 | g << 64i32 - 34i32)
            ^ (g >> 39i32 | g << 64i32 - 39i32))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = (b as u64).wrapping_add(t1_0) as u64;
        f = t1_0.wrapping_add(t2_0);
        t1_0 = e
            .wrapping_add(
                (b >> 14i32 | b << 64i32 - 14i32)
                    ^ (b >> 18i32 | b << 64i32 - 18i32)
                    ^ (b >> 41i32 | b << 64i32 - 41i32),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 11i32) as usize])
            .wrapping_add(w[11]);
        t2_0 = ((f >> 28i32 | f << 64i32 - 28i32)
            ^ (f >> 34i32 | f << 64i32 - 34i32)
            ^ (f >> 39i32 | f << 64i32 - 39i32))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = (a as u64).wrapping_add(t1_0) as u64;
        e = t1_0.wrapping_add(t2_0);
        t1_0 = d
            .wrapping_add(
                (a >> 14i32 | a << 64i32 - 14i32)
                    ^ (a >> 18i32 | a << 64i32 - 18i32)
                    ^ (a >> 41i32 | a << 64i32 - 41i32),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 12i32) as usize])
            .wrapping_add(w[12]);
        t2_0 = ((e >> 28i32 | e << 64i32 - 28i32)
            ^ (e >> 34i32 | e << 64i32 - 34i32)
            ^ (e >> 39i32 | e << 64i32 - 39i32))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = (h as u64).wrapping_add(t1_0) as u64;
        d = t1_0.wrapping_add(t2_0);
        t1_0 = c
            .wrapping_add(
                (h >> 14i32 | h << 64i32 - 14i32)
                    ^ (h >> 18i32 | h << 64i32 - 18i32)
                    ^ (h >> 41i32 | h << 64i32 - 41i32),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 13i32) as usize])
            .wrapping_add(w[13]);
        t2_0 = ((d >> 28i32 | d << 64i32 - 28i32)
            ^ (d >> 34i32 | d << 64i32 - 34i32)
            ^ (d >> 39i32 | d << 64i32 - 39i32))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = (g as u64).wrapping_add(t1_0) as u64;
        c = t1_0.wrapping_add(t2_0);
        t1_0 = b
            .wrapping_add(
                (g >> 14i32 | g << 64i32 - 14i32)
                    ^ (g >> 18i32 | g << 64i32 - 18i32)
                    ^ (g >> 41i32 | g << 64i32 - 41i32),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 14i32) as usize])
            .wrapping_add(w[14]);
        t2_0 = ((c >> 28i32 | c << 64i32 - 28i32)
            ^ (c >> 34i32 | c << 64i32 - 34i32)
            ^ (c >> 39i32 | c << 64i32 - 39i32))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = (f as u64).wrapping_add(t1_0) as u64;
        b = t1_0.wrapping_add(t2_0);
        t1_0 = a
            .wrapping_add(
                (f >> 14i32 | f << 64i32 - 14i32)
                    ^ (f >> 18i32 | f << 64i32 - 18i32)
                    ^ (f >> 41i32 | f << 64i32 - 41i32),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 15i32) as usize])
            .wrapping_add(w[15]);
        t2_0 = ((b >> 28i32 | b << 64i32 - 28i32)
            ^ (b >> 34i32 | b << 64i32 - 34i32)
            ^ (b >> 39i32 | b << 64i32 - 39i32))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = (e as u64).wrapping_add(t1_0) as u64;
        a = t1_0.wrapping_add(t2_0);
        t += 16i32
    }
    /* Update chaining vars.  */
    (*hd).h0 = ((*hd).h0 as u64).wrapping_add(a) as u64;
    (*hd).h1 = ((*hd).h1 as u64).wrapping_add(b) as u64;
    (*hd).h2 = ((*hd).h2 as u64).wrapping_add(c) as u64;
    (*hd).h3 = ((*hd).h3 as u64).wrapping_add(d) as u64;
    (*hd).h4 = ((*hd).h4 as u64).wrapping_add(e) as u64;
    (*hd).h5 = ((*hd).h5 as u64).wrapping_add(f) as u64;
    (*hd).h6 = ((*hd).h6 as u64).wrapping_add(g) as u64;
    (*hd).h7 = ((*hd).h7 as u64).wrapping_add(h) as u64;
    return ((8i32 + 16i32) as u64)
        .wrapping_mul(::std::mem::size_of::<u64>() as u64)
        .wrapping_add(::std::mem::size_of::<u32>() as u64)
        .wrapping_add(
            (3i32 as u64)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
        ) as libc::c_uint;
}
unsafe extern "C" fn _SHA512_transform(
    mut ctx: *mut SHA512_CONTEXT,
    mut data: *const libc::c_uchar,
) -> libc::c_uint {
    return (__transform(&mut (*ctx).state, data) as u64).wrapping_add(
        (3i32 as u64)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
    ) as libc::c_uint;
}
/* The routine final terminates the computation and
 * returns the digest.
 * The handle is prepared for a new cycle, but adding bytes to the
 * handle will the destroy the returned buffer.
 * Returns: 64 bytes representing the digest.  When used for sha384,
 * we take the leftmost 48 of those bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn SHA512_write(
    mut hd: *mut SHA512_CONTEXT,
    mut inbuf: *const libc::c_uchar,
    mut inlen: libc::c_uint,
) {
    let mut stack_burn: libc::c_uint = 0i32 as libc::c_uint;
    if (*hd).count == 128i32 {
        /* flush the buffer */
        stack_burn = _SHA512_transform(hd, (*hd).buf.as_mut_ptr());
        _gcry_burn_stack(stack_burn as libc::c_int);
        (*hd).count = 0i32;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1)
    }
    if inbuf.is_null() {
        return;
    }
    if (*hd).count != 0 {
        while inlen != 0 && (*hd).count < 128i32 {
            let fresh32 = inbuf;
            inbuf = inbuf.offset(1);
            let fresh33 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh33 as usize] = *fresh32;
            inlen = inlen.wrapping_sub(1)
        }
        SHA512_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
        if inlen == 0 {
            return;
        }
    }
    _gcry_burn_stack(stack_burn as libc::c_int);
    while inlen >= 128i32 as libc::c_uint {
        stack_burn = _SHA512_transform(hd, inbuf);
        (*hd).count = 0i32;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
        inlen = inlen.wrapping_sub(128i32 as libc::c_uint);
        inbuf = inbuf.offset(128)
    }
    while inlen != 0 && (*hd).count < 128i32 {
        let fresh34 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh35 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh35 as usize] = *fresh34;
        inlen = inlen.wrapping_sub(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn SHA512_final(mut outbuf: *mut libc::c_uchar, mut hd: *mut SHA512_CONTEXT) {
    let mut stack_burn_depth: libc::c_uint = 0;
    let mut t: u64 = 0;
    let mut msb: u64 = 0;
    let mut lsb: u64 = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    SHA512_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
    /* flush */
    t = (*hd).nblocks;
    /* multiply by 128 to make a byte count */
    lsb = t << 7i32;
    msb = t >> 57i32;
    /* add the count */
    t = lsb;
    lsb = (lsb as u64).wrapping_add((*hd).count as u64) as u64;
    if lsb < t {
        msb = msb.wrapping_add(1)
    }
    /* multiply by 8 to make a bit count */
    t = lsb;
    lsb <<= 3i32;
    msb <<= 3i32;
    msb |= t >> 61i32;
    if (*hd).count < 112i32 {
        /* enough room */
        let fresh36 = (*hd).count; /* pad */
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh36 as usize] = 0x80i32 as libc::c_uchar;
        while (*hd).count < 112i32 {
            let fresh37 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh37 as usize] = 0i32 as libc::c_uchar
        }
    /* pad */
    } else {
        /* need one extra block */
        let fresh38 = (*hd).count; /* pad character */
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh38 as usize] = 0x80i32 as libc::c_uchar;
        while (*hd).count < 128i32 {
            let fresh39 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh39 as usize] = 0i32 as libc::c_uchar
        }
        SHA512_write(hd, 0 as *const libc::c_uchar, 0i32 as libc::c_uint);
        /* fill next block with zeroes */
        memset(
            (*hd).buf.as_mut_ptr() as *mut libc::c_void,
            0i32,
            112i32 as u64,
        );
    }
    /* flush */
    /* append the 128 bit count */
    buf_put_be64((*hd).buf.as_mut_ptr().offset(112) as *mut libc::c_void, msb);
    buf_put_be64((*hd).buf.as_mut_ptr().offset(120) as *mut libc::c_void, lsb);
    stack_burn_depth = _SHA512_transform(hd, (*hd).buf.as_mut_ptr());
    _gcry_burn_stack(stack_burn_depth as libc::c_int);
    p = outbuf;
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h0);
    p = p.offset(8);
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h1);
    p = p.offset(8);
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h2);
    p = p.offset(8);
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h3);
    p = p.offset(8);
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h4);
    p = p.offset(8);
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h5);
    p = p.offset(8);
    /* Note that these last two chunks are included even for SHA384.
    We just ignore them. */
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h6);
    p = p.offset(8);
    *(p as *mut u64) = _gcry_bswap64((*hd).state.h7);
    p = p.offset(8);
}
/*
 * The following codes for the arcfour stream cipher were modified
 * by Jin-Hwan Cho on August 5, 2003 based on libgrypt-1.1.42.
 *
 * Copyright (C) 2000,2001,2002,2003 Free Software Foundation, Inc.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 *
 * According to the definition of MD5 in RFC 1321 from April 1992.
 * NOTE: This is *not* the same file as the one from glibc.
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995.
 * heavily modified for GnuPG by Werner Koch <wk@gnupg.org>
 */
unsafe extern "C" fn do_encrypt_stream(
    mut ctx: *mut ARC4_CONTEXT,
    mut outbuf: *mut libc::c_uchar,
    mut inbuf: *const libc::c_uchar,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_int = (*ctx).idx_i; /* and seems to be faster than mod */
    let mut j: libc::c_int = (*ctx).idx_j;
    let mut sbox: *mut libc::c_uchar = (*ctx).sbox.as_mut_ptr();
    let mut t: libc::c_int = 0;
    loop {
        let fresh40 = len;
        len = len.wrapping_sub(1);
        if !(fresh40 != 0) {
            break;
        }
        i += 1;
        i = i & 255i32;
        j += *sbox.offset(i as isize) as libc::c_int;
        j &= 255i32;
        t = *sbox.offset(i as isize) as libc::c_int;
        *sbox.offset(i as isize) = *sbox.offset(j as isize);
        *sbox.offset(j as isize) = t as libc::c_uchar;
        let fresh41 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh42 = outbuf;
        outbuf = outbuf.offset(1);
        *fresh42 = (*fresh41 as libc::c_int
            ^ *sbox.offset(
                (*sbox.offset(i as isize) as libc::c_int + *sbox.offset(j as isize) as libc::c_int
                    & 255i32) as isize,
            ) as libc::c_int) as libc::c_uchar
    }
    (*ctx).idx_i = i;
    (*ctx).idx_j = j;
}
#[no_mangle]
pub unsafe extern "C" fn ARC4(
    mut ctx: *mut ARC4_CONTEXT,
    mut len: libc::c_uint,
    mut inbuf: *const libc::c_uchar,
    mut outbuf: *mut libc::c_uchar,
) {
    do_encrypt_stream(ctx, outbuf, inbuf, len);
    _gcry_burn_stack(64i32);
}
unsafe extern "C" fn do_arcfour_setkey(
    mut ctx: *mut ARC4_CONTEXT,
    mut key: *const libc::c_uchar,
    mut keylen: libc::c_uint,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut karr: [libc::c_uchar; 256] = [0; 256];
    (*ctx).idx_j = 0i32;
    (*ctx).idx_i = (*ctx).idx_j;
    i = 0i32;
    while i < 256i32 {
        (*ctx).sbox[i as usize] = i as libc::c_uchar;
        i += 1
    }
    i = 0i32;
    while i < 256i32 {
        karr[i as usize] = *key.offset((i as libc::c_uint).wrapping_rem(keylen) as isize);
        i += 1
    }
    j = 0i32;
    i = j;
    while i < 256i32 {
        let mut t: libc::c_int = 0;
        j = (j + (*ctx).sbox[i as usize] as libc::c_int + karr[i as usize] as libc::c_int) % 256i32;
        t = (*ctx).sbox[i as usize] as libc::c_int;
        (*ctx).sbox[i as usize] = (*ctx).sbox[j as usize];
        (*ctx).sbox[j as usize] = t as libc::c_uchar;
        i += 1
    }
    memset(
        karr.as_mut_ptr() as *mut libc::c_void,
        0i32,
        256i32 as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ARC4_set_key(
    mut ctx: *mut ARC4_CONTEXT,
    mut keylen: libc::c_uint,
    mut key: *const libc::c_uchar,
) {
    do_arcfour_setkey(ctx, key, keylen);
    _gcry_burn_stack(300i32);
}
#[no_mangle]
pub unsafe extern "C" fn AES_ecb_encrypt(
    mut key: *const libc::c_uchar,
    mut key_len: size_t,
    mut plain: *const libc::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut libc::c_uchar,
    mut cipher_len: *mut size_t,
) {
    let mut ctx: *mut AES_CONTEXT = 0 as *mut AES_CONTEXT;
    let mut aes: AES_CONTEXT = AES_CONTEXT {
        nrounds: 0,
        rk: [0; 60],
        iv: [0; 16],
    };
    let mut inptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut outptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    ctx = &mut aes;
    *cipher_len = plain_len;
    *cipher = new((*cipher_len as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
        as u32) as *mut libc::c_uchar;
    (*ctx).nrounds = rijndaelSetupEncrypt(
        (*ctx).rk.as_mut_ptr(),
        key,
        key_len.wrapping_mul(8i32 as u64) as libc::c_int,
    );
    inptr = plain;
    outptr = *cipher;
    len = plain_len;
    while len >= 16i32 as u64 {
        rijndaelEncrypt((*ctx).rk.as_mut_ptr(), (*ctx).nrounds, inptr, outptr);
        inptr = inptr.offset(16);
        outptr = outptr.offset(16);
        len = (len as u64).wrapping_sub(16i32 as u64) as size_t as size_t
    }
    if len > 0i32 as u64 {
        let mut block: [libc::c_uchar; 16] = [0; 16];
        memcpy(
            block.as_mut_ptr() as *mut libc::c_void,
            inptr as *const libc::c_void,
            len,
        );
        rijndaelEncrypt(
            (*ctx).rk.as_mut_ptr(),
            (*ctx).nrounds,
            block.as_mut_ptr() as *const uint8_t,
            outptr,
        );
        inptr = inptr.offset(len as isize);
        outptr = outptr.offset(16)
    };
}
/* libgcrypt arcfour */
/* NULL iv means here "use random IV". */
#[no_mangle]
pub unsafe extern "C" fn AES_cbc_encrypt_tectonic(
    mut key: *const libc::c_uchar,
    mut key_len: size_t,
    mut iv: *const libc::c_uchar,
    mut padding: libc::c_int,
    mut plain: *const libc::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut libc::c_uchar,
    mut cipher_len: *mut size_t,
) {
    let mut ctx: *mut AES_CONTEXT = 0 as *mut AES_CONTEXT;
    let mut aes: AES_CONTEXT = AES_CONTEXT {
        nrounds: 0,
        rk: [0; 60],
        iv: [0; 16],
    };
    let mut inptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut outptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut block: [libc::c_uchar; 16] = [0; 16];
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut padbytes: libc::c_int = 0;
    ctx = &mut aes;
    if !iv.is_null() {
        memcpy(
            (*ctx).iv.as_mut_ptr() as *mut libc::c_void,
            iv as *const libc::c_void,
            16i32 as u64,
        );
    } else {
        i = 0i32 as size_t;
        while i < 16i32 as u64 {
            (*ctx).iv[i as usize] = (rand() % 256i32) as libc::c_uchar;
            i = i.wrapping_add(1)
        }
    }
    /* 16 bytes aligned.
     * Note that when padding is enabled there can be excess 16-byte
     * filled with 0x10. It occurs when size of the input data is multiple
     * of 16.
     */
    padbytes = (if padding != 0 {
        (16i32 as u64).wrapping_sub(plain_len.wrapping_rem(16i32 as u64))
    } else if plain_len.wrapping_rem(16i32 as u64) != 0 {
        (16i32 as u64).wrapping_sub(plain_len.wrapping_rem(16i32 as u64))
    } else {
        0i32 as u64
    }) as libc::c_int;
    /* We do NOT write IV to the output stream if IV is explicitly specified. */
    *cipher_len = plain_len
        .wrapping_add((if !iv.is_null() { 0i32 } else { 16i32 }) as u64)
        .wrapping_add(padbytes as u64);
    *cipher = new((*cipher_len as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as u64)
        as u32) as *mut libc::c_uchar;
    (*ctx).nrounds = rijndaelSetupEncrypt(
        (*ctx).rk.as_mut_ptr(),
        key,
        key_len.wrapping_mul(8i32 as u64) as libc::c_int,
    );
    inptr = plain;
    outptr = *cipher;
    if iv.is_null() {
        memcpy(
            outptr as *mut libc::c_void,
            (*ctx).iv.as_mut_ptr() as *const libc::c_void,
            16i32 as u64,
        );
        outptr = outptr.offset(16)
    }
    len = plain_len;
    while len >= 16i32 as u64 {
        i = 0i32 as size_t;
        while i < 16i32 as u64 {
            block[i as usize] = (*inptr.offset(i as isize) as libc::c_int
                ^ (*ctx).iv[i as usize] as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1)
        }
        rijndaelEncrypt(
            (*ctx).rk.as_mut_ptr(),
            (*ctx).nrounds,
            block.as_mut_ptr() as *const uint8_t,
            outptr,
        );
        memcpy(
            (*ctx).iv.as_mut_ptr() as *mut libc::c_void,
            outptr as *const libc::c_void,
            16i32 as u64,
        );
        inptr = inptr.offset(16);
        outptr = outptr.offset(16);
        len = (len as u64).wrapping_sub(16i32 as u64) as size_t as size_t
    }
    if len > 0i32 as u64 || padding != 0 {
        i = 0i32 as size_t;
        while i < len {
            block[i as usize] = (*inptr.offset(i as isize) as libc::c_int
                ^ (*ctx).iv[i as usize] as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1)
        }
        i = len;
        while i < 16i32 as u64 {
            block[i as usize] = (padbytes ^ (*ctx).iv[i as usize] as libc::c_int) as libc::c_uchar;
            i = i.wrapping_add(1)
        }
        rijndaelEncrypt(
            (*ctx).rk.as_mut_ptr(),
            (*ctx).nrounds,
            block.as_mut_ptr() as *const uint8_t,
            outptr,
        );
        memcpy(
            (*ctx).iv.as_mut_ptr() as *mut libc::c_void,
            outptr as *const libc::c_void,
            16i32 as u64,
        );
        inptr = inptr.offset(16);
        outptr = outptr.offset(16)
    };
}
/* The following section contains a Rijndael encryption implementation
 * based on code from Philip J. Erdelsky's public domain one.
 * They can be obtained from
 *
 *   http://www.efgh.com/software/rijndael.htm
 *   http://www.efgh.com/software/rijndael.txt
 */
static mut Te0: [u32; 256] = [
    0xc66363a5u32,
    0xf87c7c84u32,
    0xee777799u32,
    0xf67b7b8du32,
    0xfff2f20du32,
    0xd66b6bbdu32,
    0xde6f6fb1u32,
    0x91c5c554u32,
    0x60303050u32,
    0x2010103u32,
    0xce6767a9u32,
    0x562b2b7du32,
    0xe7fefe19u32,
    0xb5d7d762u32,
    0x4dababe6u32,
    0xec76769au32,
    0x8fcaca45u32,
    0x1f82829du32,
    0x89c9c940u32,
    0xfa7d7d87u32,
    0xeffafa15u32,
    0xb25959ebu32,
    0x8e4747c9u32,
    0xfbf0f00bu32,
    0x41adadecu32,
    0xb3d4d467u32,
    0x5fa2a2fdu32,
    0x45afafeau32,
    0x239c9cbfu32,
    0x53a4a4f7u32,
    0xe4727296u32,
    0x9bc0c05bu32,
    0x75b7b7c2u32,
    0xe1fdfd1cu32,
    0x3d9393aeu32,
    0x4c26266au32,
    0x6c36365au32,
    0x7e3f3f41u32,
    0xf5f7f702u32,
    0x83cccc4fu32,
    0x6834345cu32,
    0x51a5a5f4u32,
    0xd1e5e534u32,
    0xf9f1f108u32,
    0xe2717193u32,
    0xabd8d873u32,
    0x62313153u32,
    0x2a15153fu32,
    0x804040cu32,
    0x95c7c752u32,
    0x46232365u32,
    0x9dc3c35eu32,
    0x30181828u32,
    0x379696a1u32,
    0xa05050fu32,
    0x2f9a9ab5u32,
    0xe070709u32,
    0x24121236u32,
    0x1b80809bu32,
    0xdfe2e23du32,
    0xcdebeb26u32,
    0x4e272769u32,
    0x7fb2b2cdu32,
    0xea75759fu32,
    0x1209091bu32,
    0x1d83839eu32,
    0x582c2c74u32,
    0x341a1a2eu32,
    0x361b1b2du32,
    0xdc6e6eb2u32,
    0xb45a5aeeu32,
    0x5ba0a0fbu32,
    0xa45252f6u32,
    0x763b3b4du32,
    0xb7d6d661u32,
    0x7db3b3ceu32,
    0x5229297bu32,
    0xdde3e33eu32,
    0x5e2f2f71u32,
    0x13848497u32,
    0xa65353f5u32,
    0xb9d1d168u32,
    0u32,
    0xc1eded2cu32,
    0x40202060u32,
    0xe3fcfc1fu32,
    0x79b1b1c8u32,
    0xb65b5bedu32,
    0xd46a6abeu32,
    0x8dcbcb46u32,
    0x67bebed9u32,
    0x7239394bu32,
    0x944a4adeu32,
    0x984c4cd4u32,
    0xb05858e8u32,
    0x85cfcf4au32,
    0xbbd0d06bu32,
    0xc5efef2au32,
    0x4faaaae5u32,
    0xedfbfb16u32,
    0x864343c5u32,
    0x9a4d4dd7u32,
    0x66333355u32,
    0x11858594u32,
    0x8a4545cfu32,
    0xe9f9f910u32,
    0x4020206u32,
    0xfe7f7f81u32,
    0xa05050f0u32,
    0x783c3c44u32,
    0x259f9fbau32,
    0x4ba8a8e3u32,
    0xa25151f3u32,
    0x5da3a3feu32,
    0x804040c0u32,
    0x58f8f8au32,
    0x3f9292adu32,
    0x219d9dbcu32,
    0x70383848u32,
    0xf1f5f504u32,
    0x63bcbcdfu32,
    0x77b6b6c1u32,
    0xafdada75u32,
    0x42212163u32,
    0x20101030u32,
    0xe5ffff1au32,
    0xfdf3f30eu32,
    0xbfd2d26du32,
    0x81cdcd4cu32,
    0x180c0c14u32,
    0x26131335u32,
    0xc3ecec2fu32,
    0xbe5f5fe1u32,
    0x359797a2u32,
    0x884444ccu32,
    0x2e171739u32,
    0x93c4c457u32,
    0x55a7a7f2u32,
    0xfc7e7e82u32,
    0x7a3d3d47u32,
    0xc86464acu32,
    0xba5d5de7u32,
    0x3219192bu32,
    0xe6737395u32,
    0xc06060a0u32,
    0x19818198u32,
    0x9e4f4fd1u32,
    0xa3dcdc7fu32,
    0x44222266u32,
    0x542a2a7eu32,
    0x3b9090abu32,
    0xb888883u32,
    0x8c4646cau32,
    0xc7eeee29u32,
    0x6bb8b8d3u32,
    0x2814143cu32,
    0xa7dede79u32,
    0xbc5e5ee2u32,
    0x160b0b1du32,
    0xaddbdb76u32,
    0xdbe0e03bu32,
    0x64323256u32,
    0x743a3a4eu32,
    0x140a0a1eu32,
    0x924949dbu32,
    0xc06060au32,
    0x4824246cu32,
    0xb85c5ce4u32,
    0x9fc2c25du32,
    0xbdd3d36eu32,
    0x43acacefu32,
    0xc46262a6u32,
    0x399191a8u32,
    0x319595a4u32,
    0xd3e4e437u32,
    0xf279798bu32,
    0xd5e7e732u32,
    0x8bc8c843u32,
    0x6e373759u32,
    0xda6d6db7u32,
    0x18d8d8cu32,
    0xb1d5d564u32,
    0x9c4e4ed2u32,
    0x49a9a9e0u32,
    0xd86c6cb4u32,
    0xac5656fau32,
    0xf3f4f407u32,
    0xcfeaea25u32,
    0xca6565afu32,
    0xf47a7a8eu32,
    0x47aeaee9u32,
    0x10080818u32,
    0x6fbabad5u32,
    0xf0787888u32,
    0x4a25256fu32,
    0x5c2e2e72u32,
    0x381c1c24u32,
    0x57a6a6f1u32,
    0x73b4b4c7u32,
    0x97c6c651u32,
    0xcbe8e823u32,
    0xa1dddd7cu32,
    0xe874749cu32,
    0x3e1f1f21u32,
    0x964b4bddu32,
    0x61bdbddcu32,
    0xd8b8b86u32,
    0xf8a8a85u32,
    0xe0707090u32,
    0x7c3e3e42u32,
    0x71b5b5c4u32,
    0xcc6666aau32,
    0x904848d8u32,
    0x6030305u32,
    0xf7f6f601u32,
    0x1c0e0e12u32,
    0xc26161a3u32,
    0x6a35355fu32,
    0xae5757f9u32,
    0x69b9b9d0u32,
    0x17868691u32,
    0x99c1c158u32,
    0x3a1d1d27u32,
    0x279e9eb9u32,
    0xd9e1e138u32,
    0xebf8f813u32,
    0x2b9898b3u32,
    0x22111133u32,
    0xd26969bbu32,
    0xa9d9d970u32,
    0x78e8e89u32,
    0x339494a7u32,
    0x2d9b9bb6u32,
    0x3c1e1e22u32,
    0x15878792u32,
    0xc9e9e920u32,
    0x87cece49u32,
    0xaa5555ffu32,
    0x50282878u32,
    0xa5dfdf7au32,
    0x38c8c8fu32,
    0x59a1a1f8u32,
    0x9898980u32,
    0x1a0d0d17u32,
    0x65bfbfdau32,
    0xd7e6e631u32,
    0x844242c6u32,
    0xd06868b8u32,
    0x824141c3u32,
    0x299999b0u32,
    0x5a2d2d77u32,
    0x1e0f0f11u32,
    0x7bb0b0cbu32,
    0xa85454fcu32,
    0x6dbbbbd6u32,
    0x2c16163au32,
];
static mut Te1: [u32; 256] = [
    0xa5c66363u32,
    0x84f87c7cu32,
    0x99ee7777u32,
    0x8df67b7bu32,
    0xdfff2f2u32,
    0xbdd66b6bu32,
    0xb1de6f6fu32,
    0x5491c5c5u32,
    0x50603030u32,
    0x3020101u32,
    0xa9ce6767u32,
    0x7d562b2bu32,
    0x19e7fefeu32,
    0x62b5d7d7u32,
    0xe64dababu32,
    0x9aec7676u32,
    0x458fcacau32,
    0x9d1f8282u32,
    0x4089c9c9u32,
    0x87fa7d7du32,
    0x15effafau32,
    0xebb25959u32,
    0xc98e4747u32,
    0xbfbf0f0u32,
    0xec41adadu32,
    0x67b3d4d4u32,
    0xfd5fa2a2u32,
    0xea45afafu32,
    0xbf239c9cu32,
    0xf753a4a4u32,
    0x96e47272u32,
    0x5b9bc0c0u32,
    0xc275b7b7u32,
    0x1ce1fdfdu32,
    0xae3d9393u32,
    0x6a4c2626u32,
    0x5a6c3636u32,
    0x417e3f3fu32,
    0x2f5f7f7u32,
    0x4f83ccccu32,
    0x5c683434u32,
    0xf451a5a5u32,
    0x34d1e5e5u32,
    0x8f9f1f1u32,
    0x93e27171u32,
    0x73abd8d8u32,
    0x53623131u32,
    0x3f2a1515u32,
    0xc080404u32,
    0x5295c7c7u32,
    0x65462323u32,
    0x5e9dc3c3u32,
    0x28301818u32,
    0xa1379696u32,
    0xf0a0505u32,
    0xb52f9a9au32,
    0x90e0707u32,
    0x36241212u32,
    0x9b1b8080u32,
    0x3ddfe2e2u32,
    0x26cdebebu32,
    0x694e2727u32,
    0xcd7fb2b2u32,
    0x9fea7575u32,
    0x1b120909u32,
    0x9e1d8383u32,
    0x74582c2cu32,
    0x2e341a1au32,
    0x2d361b1bu32,
    0xb2dc6e6eu32,
    0xeeb45a5au32,
    0xfb5ba0a0u32,
    0xf6a45252u32,
    0x4d763b3bu32,
    0x61b7d6d6u32,
    0xce7db3b3u32,
    0x7b522929u32,
    0x3edde3e3u32,
    0x715e2f2fu32,
    0x97138484u32,
    0xf5a65353u32,
    0x68b9d1d1u32,
    0u32,
    0x2cc1ededu32,
    0x60402020u32,
    0x1fe3fcfcu32,
    0xc879b1b1u32,
    0xedb65b5bu32,
    0xbed46a6au32,
    0x468dcbcbu32,
    0xd967bebeu32,
    0x4b723939u32,
    0xde944a4au32,
    0xd4984c4cu32,
    0xe8b05858u32,
    0x4a85cfcfu32,
    0x6bbbd0d0u32,
    0x2ac5efefu32,
    0xe54faaaau32,
    0x16edfbfbu32,
    0xc5864343u32,
    0xd79a4d4du32,
    0x55663333u32,
    0x94118585u32,
    0xcf8a4545u32,
    0x10e9f9f9u32,
    0x6040202u32,
    0x81fe7f7fu32,
    0xf0a05050u32,
    0x44783c3cu32,
    0xba259f9fu32,
    0xe34ba8a8u32,
    0xf3a25151u32,
    0xfe5da3a3u32,
    0xc0804040u32,
    0x8a058f8fu32,
    0xad3f9292u32,
    0xbc219d9du32,
    0x48703838u32,
    0x4f1f5f5u32,
    0xdf63bcbcu32,
    0xc177b6b6u32,
    0x75afdadau32,
    0x63422121u32,
    0x30201010u32,
    0x1ae5ffffu32,
    0xefdf3f3u32,
    0x6dbfd2d2u32,
    0x4c81cdcdu32,
    0x14180c0cu32,
    0x35261313u32,
    0x2fc3ececu32,
    0xe1be5f5fu32,
    0xa2359797u32,
    0xcc884444u32,
    0x392e1717u32,
    0x5793c4c4u32,
    0xf255a7a7u32,
    0x82fc7e7eu32,
    0x477a3d3du32,
    0xacc86464u32,
    0xe7ba5d5du32,
    0x2b321919u32,
    0x95e67373u32,
    0xa0c06060u32,
    0x98198181u32,
    0xd19e4f4fu32,
    0x7fa3dcdcu32,
    0x66442222u32,
    0x7e542a2au32,
    0xab3b9090u32,
    0x830b8888u32,
    0xca8c4646u32,
    0x29c7eeeeu32,
    0xd36bb8b8u32,
    0x3c281414u32,
    0x79a7dedeu32,
    0xe2bc5e5eu32,
    0x1d160b0bu32,
    0x76addbdbu32,
    0x3bdbe0e0u32,
    0x56643232u32,
    0x4e743a3au32,
    0x1e140a0au32,
    0xdb924949u32,
    0xa0c0606u32,
    0x6c482424u32,
    0xe4b85c5cu32,
    0x5d9fc2c2u32,
    0x6ebdd3d3u32,
    0xef43acacu32,
    0xa6c46262u32,
    0xa8399191u32,
    0xa4319595u32,
    0x37d3e4e4u32,
    0x8bf27979u32,
    0x32d5e7e7u32,
    0x438bc8c8u32,
    0x596e3737u32,
    0xb7da6d6du32,
    0x8c018d8du32,
    0x64b1d5d5u32,
    0xd29c4e4eu32,
    0xe049a9a9u32,
    0xb4d86c6cu32,
    0xfaac5656u32,
    0x7f3f4f4u32,
    0x25cfeaeau32,
    0xafca6565u32,
    0x8ef47a7au32,
    0xe947aeaeu32,
    0x18100808u32,
    0xd56fbabau32,
    0x88f07878u32,
    0x6f4a2525u32,
    0x725c2e2eu32,
    0x24381c1cu32,
    0xf157a6a6u32,
    0xc773b4b4u32,
    0x5197c6c6u32,
    0x23cbe8e8u32,
    0x7ca1ddddu32,
    0x9ce87474u32,
    0x213e1f1fu32,
    0xdd964b4bu32,
    0xdc61bdbdu32,
    0x860d8b8bu32,
    0x850f8a8au32,
    0x90e07070u32,
    0x427c3e3eu32,
    0xc471b5b5u32,
    0xaacc6666u32,
    0xd8904848u32,
    0x5060303u32,
    0x1f7f6f6u32,
    0x121c0e0eu32,
    0xa3c26161u32,
    0x5f6a3535u32,
    0xf9ae5757u32,
    0xd069b9b9u32,
    0x91178686u32,
    0x5899c1c1u32,
    0x273a1d1du32,
    0xb9279e9eu32,
    0x38d9e1e1u32,
    0x13ebf8f8u32,
    0xb32b9898u32,
    0x33221111u32,
    0xbbd26969u32,
    0x70a9d9d9u32,
    0x89078e8eu32,
    0xa7339494u32,
    0xb62d9b9bu32,
    0x223c1e1eu32,
    0x92158787u32,
    0x20c9e9e9u32,
    0x4987ceceu32,
    0xffaa5555u32,
    0x78502828u32,
    0x7aa5dfdfu32,
    0x8f038c8cu32,
    0xf859a1a1u32,
    0x80098989u32,
    0x171a0d0du32,
    0xda65bfbfu32,
    0x31d7e6e6u32,
    0xc6844242u32,
    0xb8d06868u32,
    0xc3824141u32,
    0xb0299999u32,
    0x775a2d2du32,
    0x111e0f0fu32,
    0xcb7bb0b0u32,
    0xfca85454u32,
    0xd66dbbbbu32,
    0x3a2c1616u32,
];
static mut Te2: [u32; 256] = [
    0x63a5c663u32,
    0x7c84f87cu32,
    0x7799ee77u32,
    0x7b8df67bu32,
    0xf20dfff2u32,
    0x6bbdd66bu32,
    0x6fb1de6fu32,
    0xc55491c5u32,
    0x30506030u32,
    0x1030201u32,
    0x67a9ce67u32,
    0x2b7d562bu32,
    0xfe19e7feu32,
    0xd762b5d7u32,
    0xabe64dabu32,
    0x769aec76u32,
    0xca458fcau32,
    0x829d1f82u32,
    0xc94089c9u32,
    0x7d87fa7du32,
    0xfa15effau32,
    0x59ebb259u32,
    0x47c98e47u32,
    0xf00bfbf0u32,
    0xadec41adu32,
    0xd467b3d4u32,
    0xa2fd5fa2u32,
    0xafea45afu32,
    0x9cbf239cu32,
    0xa4f753a4u32,
    0x7296e472u32,
    0xc05b9bc0u32,
    0xb7c275b7u32,
    0xfd1ce1fdu32,
    0x93ae3d93u32,
    0x266a4c26u32,
    0x365a6c36u32,
    0x3f417e3fu32,
    0xf702f5f7u32,
    0xcc4f83ccu32,
    0x345c6834u32,
    0xa5f451a5u32,
    0xe534d1e5u32,
    0xf108f9f1u32,
    0x7193e271u32,
    0xd873abd8u32,
    0x31536231u32,
    0x153f2a15u32,
    0x40c0804u32,
    0xc75295c7u32,
    0x23654623u32,
    0xc35e9dc3u32,
    0x18283018u32,
    0x96a13796u32,
    0x50f0a05u32,
    0x9ab52f9au32,
    0x7090e07u32,
    0x12362412u32,
    0x809b1b80u32,
    0xe23ddfe2u32,
    0xeb26cdebu32,
    0x27694e27u32,
    0xb2cd7fb2u32,
    0x759fea75u32,
    0x91b1209u32,
    0x839e1d83u32,
    0x2c74582cu32,
    0x1a2e341au32,
    0x1b2d361bu32,
    0x6eb2dc6eu32,
    0x5aeeb45au32,
    0xa0fb5ba0u32,
    0x52f6a452u32,
    0x3b4d763bu32,
    0xd661b7d6u32,
    0xb3ce7db3u32,
    0x297b5229u32,
    0xe33edde3u32,
    0x2f715e2fu32,
    0x84971384u32,
    0x53f5a653u32,
    0xd168b9d1u32,
    0u32,
    0xed2cc1edu32,
    0x20604020u32,
    0xfc1fe3fcu32,
    0xb1c879b1u32,
    0x5bedb65bu32,
    0x6abed46au32,
    0xcb468dcbu32,
    0xbed967beu32,
    0x394b7239u32,
    0x4ade944au32,
    0x4cd4984cu32,
    0x58e8b058u32,
    0xcf4a85cfu32,
    0xd06bbbd0u32,
    0xef2ac5efu32,
    0xaae54faau32,
    0xfb16edfbu32,
    0x43c58643u32,
    0x4dd79a4du32,
    0x33556633u32,
    0x85941185u32,
    0x45cf8a45u32,
    0xf910e9f9u32,
    0x2060402u32,
    0x7f81fe7fu32,
    0x50f0a050u32,
    0x3c44783cu32,
    0x9fba259fu32,
    0xa8e34ba8u32,
    0x51f3a251u32,
    0xa3fe5da3u32,
    0x40c08040u32,
    0x8f8a058fu32,
    0x92ad3f92u32,
    0x9dbc219du32,
    0x38487038u32,
    0xf504f1f5u32,
    0xbcdf63bcu32,
    0xb6c177b6u32,
    0xda75afdau32,
    0x21634221u32,
    0x10302010u32,
    0xff1ae5ffu32,
    0xf30efdf3u32,
    0xd26dbfd2u32,
    0xcd4c81cdu32,
    0xc14180cu32,
    0x13352613u32,
    0xec2fc3ecu32,
    0x5fe1be5fu32,
    0x97a23597u32,
    0x44cc8844u32,
    0x17392e17u32,
    0xc45793c4u32,
    0xa7f255a7u32,
    0x7e82fc7eu32,
    0x3d477a3du32,
    0x64acc864u32,
    0x5de7ba5du32,
    0x192b3219u32,
    0x7395e673u32,
    0x60a0c060u32,
    0x81981981u32,
    0x4fd19e4fu32,
    0xdc7fa3dcu32,
    0x22664422u32,
    0x2a7e542au32,
    0x90ab3b90u32,
    0x88830b88u32,
    0x46ca8c46u32,
    0xee29c7eeu32,
    0xb8d36bb8u32,
    0x143c2814u32,
    0xde79a7deu32,
    0x5ee2bc5eu32,
    0xb1d160bu32,
    0xdb76addbu32,
    0xe03bdbe0u32,
    0x32566432u32,
    0x3a4e743au32,
    0xa1e140au32,
    0x49db9249u32,
    0x60a0c06u32,
    0x246c4824u32,
    0x5ce4b85cu32,
    0xc25d9fc2u32,
    0xd36ebdd3u32,
    0xacef43acu32,
    0x62a6c462u32,
    0x91a83991u32,
    0x95a43195u32,
    0xe437d3e4u32,
    0x798bf279u32,
    0xe732d5e7u32,
    0xc8438bc8u32,
    0x37596e37u32,
    0x6db7da6du32,
    0x8d8c018du32,
    0xd564b1d5u32,
    0x4ed29c4eu32,
    0xa9e049a9u32,
    0x6cb4d86cu32,
    0x56faac56u32,
    0xf407f3f4u32,
    0xea25cfeau32,
    0x65afca65u32,
    0x7a8ef47au32,
    0xaee947aeu32,
    0x8181008u32,
    0xbad56fbau32,
    0x7888f078u32,
    0x256f4a25u32,
    0x2e725c2eu32,
    0x1c24381cu32,
    0xa6f157a6u32,
    0xb4c773b4u32,
    0xc65197c6u32,
    0xe823cbe8u32,
    0xdd7ca1ddu32,
    0x749ce874u32,
    0x1f213e1fu32,
    0x4bdd964bu32,
    0xbddc61bdu32,
    0x8b860d8bu32,
    0x8a850f8au32,
    0x7090e070u32,
    0x3e427c3eu32,
    0xb5c471b5u32,
    0x66aacc66u32,
    0x48d89048u32,
    0x3050603u32,
    0xf601f7f6u32,
    0xe121c0eu32,
    0x61a3c261u32,
    0x355f6a35u32,
    0x57f9ae57u32,
    0xb9d069b9u32,
    0x86911786u32,
    0xc15899c1u32,
    0x1d273a1du32,
    0x9eb9279eu32,
    0xe138d9e1u32,
    0xf813ebf8u32,
    0x98b32b98u32,
    0x11332211u32,
    0x69bbd269u32,
    0xd970a9d9u32,
    0x8e89078eu32,
    0x94a73394u32,
    0x9bb62d9bu32,
    0x1e223c1eu32,
    0x87921587u32,
    0xe920c9e9u32,
    0xce4987ceu32,
    0x55ffaa55u32,
    0x28785028u32,
    0xdf7aa5dfu32,
    0x8c8f038cu32,
    0xa1f859a1u32,
    0x89800989u32,
    0xd171a0du32,
    0xbfda65bfu32,
    0xe631d7e6u32,
    0x42c68442u32,
    0x68b8d068u32,
    0x41c38241u32,
    0x99b02999u32,
    0x2d775a2du32,
    0xf111e0fu32,
    0xb0cb7bb0u32,
    0x54fca854u32,
    0xbbd66dbbu32,
    0x163a2c16u32,
];
static mut Te3: [u32; 256] = [
    0x6363a5c6u32,
    0x7c7c84f8u32,
    0x777799eeu32,
    0x7b7b8df6u32,
    0xf2f20dffu32,
    0x6b6bbdd6u32,
    0x6f6fb1deu32,
    0xc5c55491u32,
    0x30305060u32,
    0x1010302u32,
    0x6767a9ceu32,
    0x2b2b7d56u32,
    0xfefe19e7u32,
    0xd7d762b5u32,
    0xababe64du32,
    0x76769aecu32,
    0xcaca458fu32,
    0x82829d1fu32,
    0xc9c94089u32,
    0x7d7d87fau32,
    0xfafa15efu32,
    0x5959ebb2u32,
    0x4747c98eu32,
    0xf0f00bfbu32,
    0xadadec41u32,
    0xd4d467b3u32,
    0xa2a2fd5fu32,
    0xafafea45u32,
    0x9c9cbf23u32,
    0xa4a4f753u32,
    0x727296e4u32,
    0xc0c05b9bu32,
    0xb7b7c275u32,
    0xfdfd1ce1u32,
    0x9393ae3du32,
    0x26266a4cu32,
    0x36365a6cu32,
    0x3f3f417eu32,
    0xf7f702f5u32,
    0xcccc4f83u32,
    0x34345c68u32,
    0xa5a5f451u32,
    0xe5e534d1u32,
    0xf1f108f9u32,
    0x717193e2u32,
    0xd8d873abu32,
    0x31315362u32,
    0x15153f2au32,
    0x4040c08u32,
    0xc7c75295u32,
    0x23236546u32,
    0xc3c35e9du32,
    0x18182830u32,
    0x9696a137u32,
    0x5050f0au32,
    0x9a9ab52fu32,
    0x707090eu32,
    0x12123624u32,
    0x80809b1bu32,
    0xe2e23ddfu32,
    0xebeb26cdu32,
    0x2727694eu32,
    0xb2b2cd7fu32,
    0x75759feau32,
    0x9091b12u32,
    0x83839e1du32,
    0x2c2c7458u32,
    0x1a1a2e34u32,
    0x1b1b2d36u32,
    0x6e6eb2dcu32,
    0x5a5aeeb4u32,
    0xa0a0fb5bu32,
    0x5252f6a4u32,
    0x3b3b4d76u32,
    0xd6d661b7u32,
    0xb3b3ce7du32,
    0x29297b52u32,
    0xe3e33eddu32,
    0x2f2f715eu32,
    0x84849713u32,
    0x5353f5a6u32,
    0xd1d168b9u32,
    0u32,
    0xeded2cc1u32,
    0x20206040u32,
    0xfcfc1fe3u32,
    0xb1b1c879u32,
    0x5b5bedb6u32,
    0x6a6abed4u32,
    0xcbcb468du32,
    0xbebed967u32,
    0x39394b72u32,
    0x4a4ade94u32,
    0x4c4cd498u32,
    0x5858e8b0u32,
    0xcfcf4a85u32,
    0xd0d06bbbu32,
    0xefef2ac5u32,
    0xaaaae54fu32,
    0xfbfb16edu32,
    0x4343c586u32,
    0x4d4dd79au32,
    0x33335566u32,
    0x85859411u32,
    0x4545cf8au32,
    0xf9f910e9u32,
    0x2020604u32,
    0x7f7f81feu32,
    0x5050f0a0u32,
    0x3c3c4478u32,
    0x9f9fba25u32,
    0xa8a8e34bu32,
    0x5151f3a2u32,
    0xa3a3fe5du32,
    0x4040c080u32,
    0x8f8f8a05u32,
    0x9292ad3fu32,
    0x9d9dbc21u32,
    0x38384870u32,
    0xf5f504f1u32,
    0xbcbcdf63u32,
    0xb6b6c177u32,
    0xdada75afu32,
    0x21216342u32,
    0x10103020u32,
    0xffff1ae5u32,
    0xf3f30efdu32,
    0xd2d26dbfu32,
    0xcdcd4c81u32,
    0xc0c1418u32,
    0x13133526u32,
    0xecec2fc3u32,
    0x5f5fe1beu32,
    0x9797a235u32,
    0x4444cc88u32,
    0x1717392eu32,
    0xc4c45793u32,
    0xa7a7f255u32,
    0x7e7e82fcu32,
    0x3d3d477au32,
    0x6464acc8u32,
    0x5d5de7bau32,
    0x19192b32u32,
    0x737395e6u32,
    0x6060a0c0u32,
    0x81819819u32,
    0x4f4fd19eu32,
    0xdcdc7fa3u32,
    0x22226644u32,
    0x2a2a7e54u32,
    0x9090ab3bu32,
    0x8888830bu32,
    0x4646ca8cu32,
    0xeeee29c7u32,
    0xb8b8d36bu32,
    0x14143c28u32,
    0xdede79a7u32,
    0x5e5ee2bcu32,
    0xb0b1d16u32,
    0xdbdb76adu32,
    0xe0e03bdbu32,
    0x32325664u32,
    0x3a3a4e74u32,
    0xa0a1e14u32,
    0x4949db92u32,
    0x6060a0cu32,
    0x24246c48u32,
    0x5c5ce4b8u32,
    0xc2c25d9fu32,
    0xd3d36ebdu32,
    0xacacef43u32,
    0x6262a6c4u32,
    0x9191a839u32,
    0x9595a431u32,
    0xe4e437d3u32,
    0x79798bf2u32,
    0xe7e732d5u32,
    0xc8c8438bu32,
    0x3737596eu32,
    0x6d6db7dau32,
    0x8d8d8c01u32,
    0xd5d564b1u32,
    0x4e4ed29cu32,
    0xa9a9e049u32,
    0x6c6cb4d8u32,
    0x5656faacu32,
    0xf4f407f3u32,
    0xeaea25cfu32,
    0x6565afcau32,
    0x7a7a8ef4u32,
    0xaeaee947u32,
    0x8081810u32,
    0xbabad56fu32,
    0x787888f0u32,
    0x25256f4au32,
    0x2e2e725cu32,
    0x1c1c2438u32,
    0xa6a6f157u32,
    0xb4b4c773u32,
    0xc6c65197u32,
    0xe8e823cbu32,
    0xdddd7ca1u32,
    0x74749ce8u32,
    0x1f1f213eu32,
    0x4b4bdd96u32,
    0xbdbddc61u32,
    0x8b8b860du32,
    0x8a8a850fu32,
    0x707090e0u32,
    0x3e3e427cu32,
    0xb5b5c471u32,
    0x6666aaccu32,
    0x4848d890u32,
    0x3030506u32,
    0xf6f601f7u32,
    0xe0e121cu32,
    0x6161a3c2u32,
    0x35355f6au32,
    0x5757f9aeu32,
    0xb9b9d069u32,
    0x86869117u32,
    0xc1c15899u32,
    0x1d1d273au32,
    0x9e9eb927u32,
    0xe1e138d9u32,
    0xf8f813ebu32,
    0x9898b32bu32,
    0x11113322u32,
    0x6969bbd2u32,
    0xd9d970a9u32,
    0x8e8e8907u32,
    0x9494a733u32,
    0x9b9bb62du32,
    0x1e1e223cu32,
    0x87879215u32,
    0xe9e920c9u32,
    0xcece4987u32,
    0x5555ffaau32,
    0x28287850u32,
    0xdfdf7aa5u32,
    0x8c8c8f03u32,
    0xa1a1f859u32,
    0x89898009u32,
    0xd0d171au32,
    0xbfbfda65u32,
    0xe6e631d7u32,
    0x4242c684u32,
    0x6868b8d0u32,
    0x4141c382u32,
    0x9999b029u32,
    0x2d2d775au32,
    0xf0f111eu32,
    0xb0b0cb7bu32,
    0x5454fca8u32,
    0xbbbbd66du32,
    0x16163a2cu32,
];
static mut Te4: [u32; 256] = [
    0x63636363u32,
    0x7c7c7c7cu32,
    0x77777777u32,
    0x7b7b7b7bu32,
    0xf2f2f2f2u32,
    0x6b6b6b6bu32,
    0x6f6f6f6fu32,
    0xc5c5c5c5u32,
    0x30303030u32,
    0x1010101u32,
    0x67676767u32,
    0x2b2b2b2bu32,
    0xfefefefeu32,
    0xd7d7d7d7u32,
    0xababababu32,
    0x76767676u32,
    0xcacacacau32,
    0x82828282u32,
    0xc9c9c9c9u32,
    0x7d7d7d7du32,
    0xfafafafau32,
    0x59595959u32,
    0x47474747u32,
    0xf0f0f0f0u32,
    0xadadadadu32,
    0xd4d4d4d4u32,
    0xa2a2a2a2u32,
    0xafafafafu32,
    0x9c9c9c9cu32,
    0xa4a4a4a4u32,
    0x72727272u32,
    0xc0c0c0c0u32,
    0xb7b7b7b7u32,
    0xfdfdfdfdu32,
    0x93939393u32,
    0x26262626u32,
    0x36363636u32,
    0x3f3f3f3fu32,
    0xf7f7f7f7u32,
    0xccccccccu32,
    0x34343434u32,
    0xa5a5a5a5u32,
    0xe5e5e5e5u32,
    0xf1f1f1f1u32,
    0x71717171u32,
    0xd8d8d8d8u32,
    0x31313131u32,
    0x15151515u32,
    0x4040404u32,
    0xc7c7c7c7u32,
    0x23232323u32,
    0xc3c3c3c3u32,
    0x18181818u32,
    0x96969696u32,
    0x5050505u32,
    0x9a9a9a9au32,
    0x7070707u32,
    0x12121212u32,
    0x80808080u32,
    0xe2e2e2e2u32,
    0xebebebebu32,
    0x27272727u32,
    0xb2b2b2b2u32,
    0x75757575u32,
    0x9090909u32,
    0x83838383u32,
    0x2c2c2c2cu32,
    0x1a1a1a1au32,
    0x1b1b1b1bu32,
    0x6e6e6e6eu32,
    0x5a5a5a5au32,
    0xa0a0a0a0u32,
    0x52525252u32,
    0x3b3b3b3bu32,
    0xd6d6d6d6u32,
    0xb3b3b3b3u32,
    0x29292929u32,
    0xe3e3e3e3u32,
    0x2f2f2f2fu32,
    0x84848484u32,
    0x53535353u32,
    0xd1d1d1d1u32,
    0u32,
    0xededededu32,
    0x20202020u32,
    0xfcfcfcfcu32,
    0xb1b1b1b1u32,
    0x5b5b5b5bu32,
    0x6a6a6a6au32,
    0xcbcbcbcbu32,
    0xbebebebeu32,
    0x39393939u32,
    0x4a4a4a4au32,
    0x4c4c4c4cu32,
    0x58585858u32,
    0xcfcfcfcfu32,
    0xd0d0d0d0u32,
    0xefefefefu32,
    0xaaaaaaaau32,
    0xfbfbfbfbu32,
    0x43434343u32,
    0x4d4d4d4du32,
    0x33333333u32,
    0x85858585u32,
    0x45454545u32,
    0xf9f9f9f9u32,
    0x2020202u32,
    0x7f7f7f7fu32,
    0x50505050u32,
    0x3c3c3c3cu32,
    0x9f9f9f9fu32,
    0xa8a8a8a8u32,
    0x51515151u32,
    0xa3a3a3a3u32,
    0x40404040u32,
    0x8f8f8f8fu32,
    0x92929292u32,
    0x9d9d9d9du32,
    0x38383838u32,
    0xf5f5f5f5u32,
    0xbcbcbcbcu32,
    0xb6b6b6b6u32,
    0xdadadadau32,
    0x21212121u32,
    0x10101010u32,
    0xffffffffu32,
    0xf3f3f3f3u32,
    0xd2d2d2d2u32,
    0xcdcdcdcdu32,
    0xc0c0c0cu32,
    0x13131313u32,
    0xececececu32,
    0x5f5f5f5fu32,
    0x97979797u32,
    0x44444444u32,
    0x17171717u32,
    0xc4c4c4c4u32,
    0xa7a7a7a7u32,
    0x7e7e7e7eu32,
    0x3d3d3d3du32,
    0x64646464u32,
    0x5d5d5d5du32,
    0x19191919u32,
    0x73737373u32,
    0x60606060u32,
    0x81818181u32,
    0x4f4f4f4fu32,
    0xdcdcdcdcu32,
    0x22222222u32,
    0x2a2a2a2au32,
    0x90909090u32,
    0x88888888u32,
    0x46464646u32,
    0xeeeeeeeeu32,
    0xb8b8b8b8u32,
    0x14141414u32,
    0xdedededeu32,
    0x5e5e5e5eu32,
    0xb0b0b0bu32,
    0xdbdbdbdbu32,
    0xe0e0e0e0u32,
    0x32323232u32,
    0x3a3a3a3au32,
    0xa0a0a0au32,
    0x49494949u32,
    0x6060606u32,
    0x24242424u32,
    0x5c5c5c5cu32,
    0xc2c2c2c2u32,
    0xd3d3d3d3u32,
    0xacacacacu32,
    0x62626262u32,
    0x91919191u32,
    0x95959595u32,
    0xe4e4e4e4u32,
    0x79797979u32,
    0xe7e7e7e7u32,
    0xc8c8c8c8u32,
    0x37373737u32,
    0x6d6d6d6du32,
    0x8d8d8d8du32,
    0xd5d5d5d5u32,
    0x4e4e4e4eu32,
    0xa9a9a9a9u32,
    0x6c6c6c6cu32,
    0x56565656u32,
    0xf4f4f4f4u32,
    0xeaeaeaeau32,
    0x65656565u32,
    0x7a7a7a7au32,
    0xaeaeaeaeu32,
    0x8080808u32,
    0xbabababau32,
    0x78787878u32,
    0x25252525u32,
    0x2e2e2e2eu32,
    0x1c1c1c1cu32,
    0xa6a6a6a6u32,
    0xb4b4b4b4u32,
    0xc6c6c6c6u32,
    0xe8e8e8e8u32,
    0xddddddddu32,
    0x74747474u32,
    0x1f1f1f1fu32,
    0x4b4b4b4bu32,
    0xbdbdbdbdu32,
    0x8b8b8b8bu32,
    0x8a8a8a8au32,
    0x70707070u32,
    0x3e3e3e3eu32,
    0xb5b5b5b5u32,
    0x66666666u32,
    0x48484848u32,
    0x3030303u32,
    0xf6f6f6f6u32,
    0xe0e0e0eu32,
    0x61616161u32,
    0x35353535u32,
    0x57575757u32,
    0xb9b9b9b9u32,
    0x86868686u32,
    0xc1c1c1c1u32,
    0x1d1d1d1du32,
    0x9e9e9e9eu32,
    0xe1e1e1e1u32,
    0xf8f8f8f8u32,
    0x98989898u32,
    0x11111111u32,
    0x69696969u32,
    0xd9d9d9d9u32,
    0x8e8e8e8eu32,
    0x94949494u32,
    0x9b9b9b9bu32,
    0x1e1e1e1eu32,
    0x87878787u32,
    0xe9e9e9e9u32,
    0xcecececeu32,
    0x55555555u32,
    0x28282828u32,
    0xdfdfdfdfu32,
    0x8c8c8c8cu32,
    0xa1a1a1a1u32,
    0x89898989u32,
    0xd0d0d0du32,
    0xbfbfbfbfu32,
    0xe6e6e6e6u32,
    0x42424242u32,
    0x68686868u32,
    0x41414141u32,
    0x99999999u32,
    0x2d2d2d2du32,
    0xf0f0f0fu32,
    0xb0b0b0b0u32,
    0x54545454u32,
    0xbbbbbbbbu32,
    0x16161616u32,
];
static mut rcon: [u32; 10] = [
    0x1000000i32 as u32,
    0x2000000i32 as u32,
    0x4000000i32 as u32,
    0x8000000i32 as u32,
    0x10000000i32 as u32,
    0x20000000i32 as u32,
    0x40000000i32 as u32,
    0x80000000u32,
    0x1b000000i32 as u32,
    0x36000000i32 as u32,
];
/* AES Support */
/* *
 * Expand the cipher key into the encryption key schedule.
 *
 * @return the number of rounds for the given cipher key size.
 */
unsafe extern "C" fn rijndaelSetupEncrypt(
    mut rk: *mut u32,
    mut key: *const uint8_t,
    mut keybits: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    let mut temp: u32 = 0;
    *rk.offset(0) = (*key.offset(0) as u32) << 24i32
        ^ (*key.offset(1) as u32) << 16i32
        ^ (*key.offset(2) as u32) << 8i32
        ^ *key.offset(3) as u32;
    *rk.offset(1) = (*key.offset(4).offset(0) as u32) << 24i32
        ^ (*key.offset(4).offset(1) as u32) << 16i32
        ^ (*key.offset(4).offset(2) as u32) << 8i32
        ^ *key.offset(4).offset(3) as u32;
    *rk.offset(2) = (*key.offset(8).offset(0) as u32) << 24i32
        ^ (*key.offset(8).offset(1) as u32) << 16i32
        ^ (*key.offset(8).offset(2) as u32) << 8i32
        ^ *key.offset(8).offset(3) as u32;
    *rk.offset(3) = (*key.offset(12).offset(0) as u32) << 24i32
        ^ (*key.offset(12).offset(1) as u32) << 16i32
        ^ (*key.offset(12).offset(2) as u32) << 8i32
        ^ *key.offset(12).offset(3) as u32;
    if keybits == 128i32 {
        loop {
            temp = *rk.offset(3);
            *rk.offset(4) = *rk.offset(0)
                ^ Te4[(temp >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff000000u32
                ^ Te4[(temp >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                    & 0xff0000i32 as libc::c_uint
                ^ Te4[(temp & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
                ^ Te4[(temp >> 24i32) as usize] & 0xffi32 as libc::c_uint
                ^ rcon[i as usize];
            *rk.offset(5) = *rk.offset(1) ^ *rk.offset(4);
            *rk.offset(6) = *rk.offset(2) ^ *rk.offset(5);
            *rk.offset(7) = *rk.offset(3) ^ *rk.offset(6);
            i = i.wrapping_add(1);
            if i == 10i32 as libc::c_uint {
                return 10i32;
            }
            rk = rk.offset(4)
        }
    }
    *rk.offset(4) = (*key.offset(16).offset(0) as u32) << 24i32
        ^ (*key.offset(16).offset(1) as u32) << 16i32
        ^ (*key.offset(16).offset(2) as u32) << 8i32
        ^ *key.offset(16).offset(3) as u32;
    *rk.offset(5) = (*key.offset(20).offset(0) as u32) << 24i32
        ^ (*key.offset(20).offset(1) as u32) << 16i32
        ^ (*key.offset(20).offset(2) as u32) << 8i32
        ^ *key.offset(20).offset(3) as u32;
    if keybits == 192i32 {
        loop {
            temp = *rk.offset(5);
            *rk.offset(6) = *rk.offset(0)
                ^ Te4[(temp >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff000000u32
                ^ Te4[(temp >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                    & 0xff0000i32 as libc::c_uint
                ^ Te4[(temp & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
                ^ Te4[(temp >> 24i32) as usize] & 0xffi32 as libc::c_uint
                ^ rcon[i as usize];
            *rk.offset(7) = *rk.offset(1) ^ *rk.offset(6);
            *rk.offset(8) = *rk.offset(2) ^ *rk.offset(7);
            *rk.offset(9) = *rk.offset(3) ^ *rk.offset(8);
            i = i.wrapping_add(1);
            if i == 8i32 as libc::c_uint {
                return 12i32;
            }
            *rk.offset(10) = *rk.offset(4) ^ *rk.offset(9);
            *rk.offset(11) = *rk.offset(5) ^ *rk.offset(10);
            rk = rk.offset(6)
        }
    }
    *rk.offset(6) = (*key.offset(24).offset(0) as u32) << 24i32
        ^ (*key.offset(24).offset(1) as u32) << 16i32
        ^ (*key.offset(24).offset(2) as u32) << 8i32
        ^ *key.offset(24).offset(3) as u32;
    *rk.offset(7) = (*key.offset(28).offset(0) as u32) << 24i32
        ^ (*key.offset(28).offset(1) as u32) << 16i32
        ^ (*key.offset(28).offset(2) as u32) << 8i32
        ^ *key.offset(28).offset(3) as u32;
    if keybits == 256i32 {
        loop {
            temp = *rk.offset(7);
            *rk.offset(8) = *rk.offset(0)
                ^ Te4[(temp >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff000000u32
                ^ Te4[(temp >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                    & 0xff0000i32 as libc::c_uint
                ^ Te4[(temp & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
                ^ Te4[(temp >> 24i32) as usize] & 0xffi32 as libc::c_uint
                ^ rcon[i as usize];
            *rk.offset(9) = *rk.offset(1) ^ *rk.offset(8);
            *rk.offset(10) = *rk.offset(2) ^ *rk.offset(9);
            *rk.offset(11) = *rk.offset(3) ^ *rk.offset(10);
            i = i.wrapping_add(1);
            if i == 7i32 as libc::c_uint {
                return 14i32;
            }
            temp = *rk.offset(11);
            *rk.offset(12) = *rk.offset(4)
                ^ Te4[(temp >> 24i32) as usize] & 0xff000000u32
                ^ Te4[(temp >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                    & 0xff0000i32 as libc::c_uint
                ^ Te4[(temp >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                    & 0xff00i32 as libc::c_uint
                ^ Te4[(temp & 0xffi32 as libc::c_uint) as usize] & 0xffi32 as libc::c_uint;
            *rk.offset(13) = *rk.offset(5) ^ *rk.offset(12);
            *rk.offset(14) = *rk.offset(6) ^ *rk.offset(13);
            *rk.offset(15) = *rk.offset(7) ^ *rk.offset(14);
            rk = rk.offset(8)
        }
    }
    return 0i32;
}
unsafe extern "C" fn rijndaelEncrypt(
    mut rk: *const u32,
    mut nrounds: libc::c_int,
    mut plaintext: *const uint8_t,
    mut ciphertext: *mut uint8_t,
) {
    let mut s0: u32 = 0;
    let mut s1: u32 = 0;
    let mut s2: u32 = 0;
    let mut s3: u32 = 0;
    let mut t0: u32 = 0;
    let mut t1: u32 = 0;
    let mut t2: u32 = 0;
    let mut t3: u32 = 0;
    /* ?FULL_UNROLL */
    /*
     * map byte array block to cipher state
     * and add initial round key:
     */
    s0 = (*plaintext.offset(0) as u32) << 24i32
        ^ (*plaintext.offset(1) as u32) << 16i32
        ^ (*plaintext.offset(2) as u32) << 8i32
        ^ *plaintext.offset(3) as u32
        ^ *rk.offset(0);
    s1 = (*plaintext.offset(4).offset(0) as u32) << 24i32
        ^ (*plaintext.offset(4).offset(1) as u32) << 16i32
        ^ (*plaintext.offset(4).offset(2) as u32) << 8i32
        ^ *plaintext.offset(4).offset(3) as u32
        ^ *rk.offset(1);
    s2 = (*plaintext.offset(8).offset(0) as u32) << 24i32
        ^ (*plaintext.offset(8).offset(1) as u32) << 16i32
        ^ (*plaintext.offset(8).offset(2) as u32) << 8i32
        ^ *plaintext.offset(8).offset(3) as u32
        ^ *rk.offset(2);
    s3 = (*plaintext.offset(12).offset(0) as u32) << 24i32
        ^ (*plaintext.offset(12).offset(1) as u32) << 16i32
        ^ (*plaintext.offset(12).offset(2) as u32) << 8i32
        ^ *plaintext.offset(12).offset(3) as u32
        ^ *rk.offset(3);
    /* round 1: */
    t0 = Te0[(s0 >> 24i32) as usize]
        ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(4);
    t1 = Te0[(s1 >> 24i32) as usize]
        ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(5);
    t2 = Te0[(s2 >> 24i32) as usize]
        ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(6);
    t3 = Te0[(s3 >> 24i32) as usize]
        ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(7);
    /* round 2: */
    s0 = Te0[(t0 >> 24i32) as usize]
        ^ Te1[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(8);
    s1 = Te0[(t1 >> 24i32) as usize]
        ^ Te1[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(9);
    s2 = Te0[(t2 >> 24i32) as usize]
        ^ Te1[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(10);
    s3 = Te0[(t3 >> 24i32) as usize]
        ^ Te1[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(11);
    /* round 3: */
    t0 = Te0[(s0 >> 24i32) as usize]
        ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(12);
    t1 = Te0[(s1 >> 24i32) as usize]
        ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(13);
    t2 = Te0[(s2 >> 24i32) as usize]
        ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(14);
    t3 = Te0[(s3 >> 24i32) as usize]
        ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(15);
    /* round 4: */
    s0 = Te0[(t0 >> 24i32) as usize]
        ^ Te1[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(16);
    s1 = Te0[(t1 >> 24i32) as usize]
        ^ Te1[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(17);
    s2 = Te0[(t2 >> 24i32) as usize]
        ^ Te1[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(18);
    s3 = Te0[(t3 >> 24i32) as usize]
        ^ Te1[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(19);
    /* round 5: */
    t0 = Te0[(s0 >> 24i32) as usize]
        ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(20);
    t1 = Te0[(s1 >> 24i32) as usize]
        ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(21);
    t2 = Te0[(s2 >> 24i32) as usize]
        ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(22);
    t3 = Te0[(s3 >> 24i32) as usize]
        ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(23);
    /* round 6: */
    s0 = Te0[(t0 >> 24i32) as usize]
        ^ Te1[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(24);
    s1 = Te0[(t1 >> 24i32) as usize]
        ^ Te1[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(25);
    s2 = Te0[(t2 >> 24i32) as usize]
        ^ Te1[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(26);
    s3 = Te0[(t3 >> 24i32) as usize]
        ^ Te1[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(27);
    /* round 7: */
    t0 = Te0[(s0 >> 24i32) as usize]
        ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(28);
    t1 = Te0[(s1 >> 24i32) as usize]
        ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(29);
    t2 = Te0[(s2 >> 24i32) as usize]
        ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(30);
    t3 = Te0[(s3 >> 24i32) as usize]
        ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(31);
    /* round 8: */
    s0 = Te0[(t0 >> 24i32) as usize]
        ^ Te1[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(32);
    s1 = Te0[(t1 >> 24i32) as usize]
        ^ Te1[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(33);
    s2 = Te0[(t2 >> 24i32) as usize]
        ^ Te1[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(34);
    s3 = Te0[(t3 >> 24i32) as usize]
        ^ Te1[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(t2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(35);
    /* round 9: */
    t0 = Te0[(s0 >> 24i32) as usize]
        ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(36);
    t1 = Te0[(s1 >> 24i32) as usize]
        ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(37);
    t2 = Te0[(s2 >> 24i32) as usize]
        ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(38);
    t3 = Te0[(s3 >> 24i32) as usize]
        ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
        ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
        ^ *rk.offset(39);
    if nrounds > 10i32 {
        /* round 10: */
        s0 = Te0[(t0 >> 24i32) as usize]
            ^ Te1[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(t3 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(40);
        s1 = Te0[(t1 >> 24i32) as usize]
            ^ Te1[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(t0 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(41);
        s2 = Te0[(t2 >> 24i32) as usize]
            ^ Te1[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(t1 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(42);
        s3 = Te0[(t3 >> 24i32) as usize]
            ^ Te1[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(t2 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(43);
        /* round 11: */
        t0 = Te0[(s0 >> 24i32) as usize]
            ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(44);
        t1 = Te0[(s1 >> 24i32) as usize]
            ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(45);
        t2 = Te0[(s2 >> 24i32) as usize]
            ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(46);
        t3 = Te0[(s3 >> 24i32) as usize]
            ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
            ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
            ^ *rk.offset(47);
        if nrounds > 12i32 {
            /* round 12: */
            s0 = Te0[(t0 >> 24i32) as usize]
                ^ Te1[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(t3 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(48);
            s1 = Te0[(t1 >> 24i32) as usize]
                ^ Te1[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(t0 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(49);
            s2 = Te0[(t2 >> 24i32) as usize]
                ^ Te1[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(t1 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(50);
            s3 = Te0[(t3 >> 24i32) as usize]
                ^ Te1[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(t2 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(51);
            /* round 13: */
            t0 = Te0[(s0 >> 24i32) as usize]
                ^ Te1[(s1 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(s2 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(s3 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(52);
            t1 = Te0[(s1 >> 24i32) as usize]
                ^ Te1[(s2 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(s3 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(s0 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(53);
            t2 = Te0[(s2 >> 24i32) as usize]
                ^ Te1[(s3 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(s0 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(s1 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(54);
            t3 = Te0[(s3 >> 24i32) as usize]
                ^ Te1[(s0 >> 16i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te2[(s1 >> 8i32 & 0xffi32 as libc::c_uint) as usize]
                ^ Te3[(s2 & 0xffi32 as libc::c_uint) as usize]
                ^ *rk.offset(55)
        }
    }
    rk = rk.offset((nrounds << 2i32) as isize);
    /* !FULL_UNROLL */
    /* ?FULL_UNROLL */
    /*
     * apply last round and
     * map cipher state to byte array block:
     */
    s0 = Te4[(t0 >> 24i32) as usize] & 0xff000000u32
        ^ Te4[(t1 >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff0000i32 as libc::c_uint
        ^ Te4[(t2 >> 8i32 & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
        ^ Te4[(t3 & 0xffi32 as libc::c_uint) as usize] & 0xffi32 as libc::c_uint
        ^ *rk.offset(0);
    *ciphertext.offset(0) = (s0 >> 24i32) as uint8_t;
    *ciphertext.offset(1) = (s0 >> 16i32) as uint8_t;
    *ciphertext.offset(2) = (s0 >> 8i32) as uint8_t;
    *ciphertext.offset(3) = s0 as uint8_t;
    s1 = Te4[(t1 >> 24i32) as usize] & 0xff000000u32
        ^ Te4[(t2 >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff0000i32 as libc::c_uint
        ^ Te4[(t3 >> 8i32 & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
        ^ Te4[(t0 & 0xffi32 as libc::c_uint) as usize] & 0xffi32 as libc::c_uint
        ^ *rk.offset(1);
    *ciphertext.offset(4).offset(0) = (s1 >> 24i32) as uint8_t;
    *ciphertext.offset(4).offset(1) = (s1 >> 16i32) as uint8_t;
    *ciphertext.offset(4).offset(2) = (s1 >> 8i32) as uint8_t;
    *ciphertext.offset(4).offset(3) = s1 as uint8_t;
    s2 = Te4[(t2 >> 24i32) as usize] & 0xff000000u32
        ^ Te4[(t3 >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff0000i32 as libc::c_uint
        ^ Te4[(t0 >> 8i32 & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
        ^ Te4[(t1 & 0xffi32 as libc::c_uint) as usize] & 0xffi32 as libc::c_uint
        ^ *rk.offset(2);
    *ciphertext.offset(8).offset(0) = (s2 >> 24i32) as uint8_t;
    *ciphertext.offset(8).offset(1) = (s2 >> 16i32) as uint8_t;
    *ciphertext.offset(8).offset(2) = (s2 >> 8i32) as uint8_t;
    *ciphertext.offset(8).offset(3) = s2 as uint8_t;
    s3 = Te4[(t3 >> 24i32) as usize] & 0xff000000u32
        ^ Te4[(t0 >> 16i32 & 0xffi32 as libc::c_uint) as usize] & 0xff0000i32 as libc::c_uint
        ^ Te4[(t1 >> 8i32 & 0xffi32 as libc::c_uint) as usize] & 0xff00i32 as libc::c_uint
        ^ Te4[(t2 & 0xffi32 as libc::c_uint) as usize] & 0xffi32 as libc::c_uint
        ^ *rk.offset(3);
    *ciphertext.offset(12).offset(0) = (s3 >> 24i32) as uint8_t;
    *ciphertext.offset(12).offset(1) = (s3 >> 16i32) as uint8_t;
    *ciphertext.offset(12).offset(2) = (s3 >> 8i32) as uint8_t;
    *ciphertext.offset(12).offset(3) = s3 as uint8_t;
}
