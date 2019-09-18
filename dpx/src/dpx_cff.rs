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

use crate::streq_ptr;
use crate::warn;

use super::dpx_cff_dict::{cff_dict_get, cff_dict_known, cff_dict_unpack, cff_release_dict};
use super::dpx_error::dpx_warning;
use super::dpx_mem::{new, renew};
use super::dpx_numbers::{tt_get_unsigned_byte, tt_get_unsigned_pair};
use crate::{ttstub_input_read, ttstub_input_seek};
use libc::{free, memcmp, memcpy, memmove, memset, strlen};

pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
/* CFF Data Types */
/* SID SID number */
/* offset(0) */
/* size offset(0) */
pub type c_offsize = u8;
/* 1-byte unsigned number specifies the size
of an Offset field or fields, range 1-4 */
pub type l_offset = u32;
/* 1, 2, 3, or 4-byte offset */
pub type s_SID = u16;
/* 2-byte string identifier  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: u16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut u8,
    /* Object data                       */
}

pub trait Pack {
    fn pack(&mut self, dest: &mut [u8]) -> usize;
}

/// Rewrittened cff_index
#[derive(Clone)]
pub struct CffIndex {
    pub count: u16, // ??
    pub offsize: c_offsize,
    pub offset: Vec<l_offset>,
    pub data: Vec<u8>,
}
impl CffIndex {
    // cff_index_new
    pub fn new(count: u16) -> Box<Self> {
        let offset = if count > 0 {
            let mut offset = vec![0; count as usize + 1];
            offset[0] = 1;
            offset
        } else {
            vec![]
        };
        Box::new(CffIndex {
            count,
            offsize: 0,
            offset,
            data: vec![],
        })
    }
    // cff_index_size
    pub fn size(&mut self) -> usize {
        if self.count > 0 {
            let datalen: l_offset = self.offset[self.count as usize] as l_offset - 1;
            self.offsize = if (datalen as u64) < 0xff {
                1
            } else if (datalen as u64) < 0xffff {
                2
            } else if (datalen as u64) < 0xffffff {
                3
            } else {
                4
            };
            (((3 + self.offsize as i32 * (self.count as i32 + 1)) as u32) + datalen) as usize
        } else {
            2
        }
    }
}

// cff_pack_index
impl Pack for CffIndex {
    fn pack(&mut self, mut dest: &mut [u8]) -> usize {
        let destlen = dest.len();
        let mut datalen: size_t = 0;
        let mut i: u16 = 0;
        if self.count < 1 {
            if destlen < 2 {
                panic!("Not enough space available...");
            }
            unsafe {
                memset(dest.as_mut_ptr() as *mut libc::c_void, 0, 2);
            }
            return 2;
        }
        let mut len = self.size();
        datalen = (self.offset[self.count as usize] - 1) as size_t;
        if destlen < len {
            panic!("Not enough space available...");
        }
        dest[0..2].copy_from_slice(&self.count.to_be_bytes());
        dest = &mut dest[2..];
        if datalen < 0xff {
            self.offsize = 1 as c_offsize;
            dest[0] = 1;
            dest = &mut dest[1..];
            for i in 0..=self.count as usize {
                dest[0] = self.offset[i] as u8;
                dest = &mut dest[1..];
            }
        } else if datalen < 0xffff {
            self.offsize = 2 as c_offsize;
            dest[0] = 2;
            dest = &mut dest[1..];
            for i in 0..=self.count as usize {
                dest[0..2].copy_from_slice(&(self.offset[i] as u16).to_be_bytes());
                dest = &mut dest[2..];
            }
        } else if datalen < 0xffffff {
            self.offsize = 3 as c_offsize;
            dest[0] = 3;
            dest = &mut dest[1..];
            for i in 0..=self.count as usize {
                dest[0..3].copy_from_slice(&self.offset[i].to_be_bytes()[1..4]);
                dest = &mut dest[3..];
            }
        } else {
            self.offsize = 4 as c_offsize;
            dest[0] = 4;
            dest = &mut dest[1..];
            for i in 0..=self.count as usize {
                dest[0..4].copy_from_slice(&self.offset[i].to_be_bytes());
                dest = &mut dest[4..];
            }
        }
        dest[..self.offset[self.count as usize] as usize - 1].copy_from_slice(&self.data[..]);
        len
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: u8,
    pub minor: u8,
    pub hdr_size: u8,
    pub offsize: c_offsize,
    /* Absolute offset (0) size             */
}
/* Dictionary */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: i32,
    pub key: *const i8,
    pub count: i32,
    pub values: *mut f64,
    /* values                                  */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: i32,
    pub count: i32,
    pub entries: *mut cff_dict_entry,
}
/* Encoding, Charset and FDSelect */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: u8,
    /* no. of remaining gids/codes in this range */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: u16,
    /* u16-version of range1 */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: u8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: u8,
    pub num_entries: u8,
    pub data: C2RustUnnamed,
    pub num_supps: u8,
    pub supp: *mut cff_map,
    /* supplement */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub codes: *mut u8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: u8,
    pub num_entries: u16,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
/* CID-Keyed font specific */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: u16,
    pub fd: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: u8,
    pub num_entries: u16,
    pub data: C2RustUnnamed_1,
    /* u16 sentinel; */
    /* format 3 only, must be equals to num_glyphs */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub fds: *mut u8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut i8,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: u16,
    pub num_fds: u8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: i32,
    pub index: i32,
    pub flag: i32,
    pub is_notdef_notzero: i32,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
static mut cff_stdstr: [&[u8]; 391] = [
    b".notdef\x00",
    b"space\x00",
    b"exclam\x00",
    b"quotedbl\x00",
    b"numbersign\x00",
    b"dollar\x00",
    b"percent\x00",
    b"ampersand\x00",
    b"quoteright\x00",
    b"parenleft\x00",
    b"parenright\x00",
    b"asterisk\x00",
    b"plus\x00",
    b"comma\x00",
    b"hyphen\x00",
    b"period\x00",
    b"slash\x00",
    b"zero\x00",
    b"one\x00",
    b"two\x00",
    b"three\x00",
    b"four\x00",
    b"five\x00",
    b"six\x00",
    b"seven\x00",
    b"eight\x00",
    b"nine\x00",
    b"colon\x00",
    b"semicolon\x00",
    b"less\x00",
    b"equal\x00",
    b"greater\x00",
    b"question\x00",
    b"at\x00",
    b"A\x00",
    b"B\x00",
    b"C\x00",
    b"D\x00",
    b"E\x00",
    b"F\x00",
    b"G\x00",
    b"H\x00",
    b"I\x00",
    b"J\x00",
    b"K\x00",
    b"L\x00",
    b"M\x00",
    b"N\x00",
    b"O\x00",
    b"P\x00",
    b"Q\x00",
    b"R\x00",
    b"S\x00",
    b"T\x00",
    b"U\x00",
    b"V\x00",
    b"W\x00",
    b"X\x00",
    b"Y\x00",
    b"Z\x00",
    b"bracketleft\x00",
    b"backslash\x00",
    b"bracketright\x00",
    b"asciicircum\x00",
    b"underscore\x00",
    b"quoteleft\x00",
    b"a\x00",
    b"b\x00",
    b"c\x00",
    b"d\x00",
    b"e\x00",
    b"f\x00",
    b"g\x00",
    b"h\x00",
    b"i\x00",
    b"j\x00",
    b"k\x00",
    b"l\x00",
    b"m\x00",
    b"n\x00",
    b"o\x00",
    b"p\x00",
    b"q\x00",
    b"r\x00",
    b"s\x00",
    b"t\x00",
    b"u\x00",
    b"v\x00",
    b"w\x00",
    b"x\x00",
    b"y\x00",
    b"z\x00",
    b"braceleft\x00",
    b"bar\x00",
    b"braceright\x00",
    b"asciitilde\x00",
    b"exclamdown\x00",
    b"cent\x00",
    b"sterling\x00",
    b"fraction\x00",
    b"yen\x00",
    b"florin\x00",
    b"section\x00",
    b"currency\x00",
    b"quotesingle\x00",
    b"quotedblleft\x00",
    b"guillemotleft\x00",
    b"guilsinglleft\x00",
    b"guilsinglright\x00",
    b"fi\x00",
    b"fl\x00",
    b"endash\x00",
    b"dagger\x00",
    b"daggerdbl\x00",
    b"periodcentered\x00",
    b"paragraph\x00",
    b"bullet\x00",
    b"quotesinglbase\x00",
    b"quotedblbase\x00",
    b"quotedblright\x00",
    b"guillemotright\x00",
    b"ellipsis\x00",
    b"perthousand\x00",
    b"questiondown\x00",
    b"grave\x00",
    b"acute\x00",
    b"circumflex\x00",
    b"tilde\x00",
    b"macron\x00",
    b"breve\x00",
    b"dotaccent\x00",
    b"dieresis\x00",
    b"ring\x00",
    b"cedilla\x00",
    b"hungarumlaut\x00",
    b"ogonek\x00",
    b"caron\x00",
    b"emdash\x00",
    b"AE\x00",
    b"ordfeminine\x00",
    b"Lslash\x00",
    b"Oslash\x00",
    b"OE\x00",
    b"ordmasculine\x00",
    b"ae\x00",
    b"dotlessi\x00",
    b"lslash\x00",
    b"oslash\x00",
    b"oe\x00",
    b"germandbls\x00",
    b"onesuperior\x00",
    b"logicalnot\x00",
    b"mu\x00",
    b"trademark\x00",
    b"Eth\x00",
    b"onehalf\x00",
    b"plusminus\x00",
    b"Thorn\x00",
    b"onequarter\x00",
    b"divide\x00",
    b"brokenbar\x00",
    b"degree\x00",
    b"thorn\x00",
    b"threequarters\x00",
    b"twosuperior\x00",
    b"registered\x00",
    b"minus\x00",
    b"eth\x00",
    b"multiply\x00",
    b"threesuperior\x00",
    b"copyright\x00",
    b"Aacute\x00",
    b"Acircumflex\x00",
    b"Adieresis\x00",
    b"Agrave\x00",
    b"Aring\x00",
    b"Atilde\x00",
    b"Ccedilla\x00",
    b"Eacute\x00",
    b"Ecircumflex\x00",
    b"Edieresis\x00",
    b"Egrave\x00",
    b"Iacute\x00",
    b"Icircumflex\x00",
    b"Idieresis\x00",
    b"Igrave\x00",
    b"Ntilde\x00",
    b"Oacute\x00",
    b"Ocircumflex\x00",
    b"Odieresis\x00",
    b"Ograve\x00",
    b"Otilde\x00",
    b"Scaron\x00",
    b"Uacute\x00",
    b"Ucircumflex\x00",
    b"Udieresis\x00",
    b"Ugrave\x00",
    b"Yacute\x00",
    b"Ydieresis\x00",
    b"Zcaron\x00",
    b"aacute\x00",
    b"acircumflex\x00",
    b"adieresis\x00",
    b"agrave\x00",
    b"aring\x00",
    b"atilde\x00",
    b"ccedilla\x00",
    b"eacute\x00",
    b"ecircumflex\x00",
    b"edieresis\x00",
    b"egrave\x00",
    b"iacute\x00",
    b"icircumflex\x00",
    b"idieresis\x00",
    b"igrave\x00",
    b"ntilde\x00",
    b"oacute\x00",
    b"ocircumflex\x00",
    b"odieresis\x00",
    b"ograve\x00",
    b"otilde\x00",
    b"scaron\x00",
    b"uacute\x00",
    b"ucircumflex\x00",
    b"udieresis\x00",
    b"ugrave\x00",
    b"yacute\x00",
    b"ydieresis\x00",
    b"zcaron\x00",
    b"exclamsmall\x00",
    b"Hungarumlautsmall\x00",
    b"dollaroldstyle\x00",
    b"dollarsuperior\x00",
    b"ampersandsmall\x00",
    b"Acutesmall\x00",
    b"parenleftsuperior\x00",
    b"parenrightsuperior\x00",
    b"twodotenleader\x00",
    b"onedotenleader\x00",
    b"zerooldstyle\x00",
    b"oneoldstyle\x00",
    b"twooldstyle\x00",
    b"threeoldstyle\x00",
    b"fouroldstyle\x00",
    b"fiveoldstyle\x00",
    b"sixoldstyle\x00",
    b"sevenoldstyle\x00",
    b"eightoldstyle\x00",
    b"nineoldstyle\x00",
    b"commasuperior\x00",
    b"threequartersemdash\x00",
    b"periodsuperior\x00",
    b"questionsmall\x00",
    b"asuperior\x00",
    b"bsuperior\x00",
    b"centsuperior\x00",
    b"dsuperior\x00",
    b"esuperior\x00",
    b"isuperior\x00",
    b"lsuperior\x00",
    b"msuperior\x00",
    b"nsuperior\x00",
    b"osuperior\x00",
    b"rsuperior\x00",
    b"ssuperior\x00",
    b"tsuperior\x00",
    b"ff\x00",
    b"ffi\x00",
    b"ffl\x00",
    b"parenleftinferior\x00",
    b"parenrightinferior\x00",
    b"Circumflexsmall\x00",
    b"hyphensuperior\x00",
    b"Gravesmall\x00",
    b"Asmall\x00",
    b"Bsmall\x00",
    b"Csmall\x00",
    b"Dsmall\x00",
    b"Esmall\x00",
    b"Fsmall\x00",
    b"Gsmall\x00",
    b"Hsmall\x00",
    b"Ismall\x00",
    b"Jsmall\x00",
    b"Ksmall\x00",
    b"Lsmall\x00",
    b"Msmall\x00",
    b"Nsmall\x00",
    b"Osmall\x00",
    b"Psmall\x00",
    b"Qsmall\x00",
    b"Rsmall\x00",
    b"Ssmall\x00",
    b"Tsmall\x00",
    b"Usmall\x00",
    b"Vsmall\x00",
    b"Wsmall\x00",
    b"Xsmall\x00",
    b"Ysmall\x00",
    b"Zsmall\x00",
    b"colonmonetary\x00",
    b"onefitted\x00",
    b"rupiah\x00",
    b"Tildesmall\x00",
    b"exclamdownsmall\x00",
    b"centoldstyle\x00",
    b"Lslashsmall\x00",
    b"Scaronsmall\x00",
    b"Zcaronsmall\x00",
    b"Dieresissmall\x00",
    b"Brevesmall\x00",
    b"Caronsmall\x00",
    b"Dotaccentsmall\x00",
    b"Macronsmall\x00",
    b"figuredash\x00",
    b"hypheninferior\x00",
    b"Ogoneksmall\x00",
    b"Ringsmall\x00",
    b"Cedillasmall\x00",
    b"questiondownsmall\x00",
    b"oneeighth\x00",
    b"threeeighths\x00",
    b"fiveeighths\x00",
    b"seveneighths\x00",
    b"onethird\x00",
    b"twothirds\x00",
    b"zerosuperior\x00",
    b"foursuperior\x00",
    b"fivesuperior\x00",
    b"sixsuperior\x00",
    b"sevensuperior\x00",
    b"eightsuperior\x00",
    b"ninesuperior\x00",
    b"zeroinferior\x00",
    b"oneinferior\x00",
    b"twoinferior\x00",
    b"threeinferior\x00",
    b"fourinferior\x00",
    b"fiveinferior\x00",
    b"sixinferior\x00",
    b"seveninferior\x00",
    b"eightinferior\x00",
    b"nineinferior\x00",
    b"centinferior\x00",
    b"dollarinferior\x00",
    b"periodinferior\x00",
    b"commainferior\x00",
    b"Agravesmall\x00",
    b"Aacutesmall\x00",
    b"Acircumflexsmall\x00",
    b"Atildesmall\x00",
    b"Adieresissmall\x00",
    b"Aringsmall\x00",
    b"AEsmall\x00",
    b"Ccedillasmall\x00",
    b"Egravesmall\x00",
    b"Eacutesmall\x00",
    b"Ecircumflexsmall\x00",
    b"Edieresissmall\x00",
    b"Igravesmall\x00",
    b"Iacutesmall\x00",
    b"Icircumflexsmall\x00",
    b"Idieresissmall\x00",
    b"Ethsmall\x00",
    b"Ntildesmall\x00",
    b"Ogravesmall\x00",
    b"Oacutesmall\x00",
    b"Ocircumflexsmall\x00",
    b"Otildesmall\x00",
    b"Odieresissmall\x00",
    b"OEsmall\x00",
    b"Oslashsmall\x00",
    b"Ugravesmall\x00",
    b"Uacutesmall\x00",
    b"Ucircumflexsmall\x00",
    b"Udieresissmall\x00",
    b"Yacutesmall\x00",
    b"Thornsmall\x00",
    b"Ydieresissmall\x00",
    b"001.000\x00",
    b"001.001\x00",
    b"001.002\x00",
    b"001.003\x00",
    b"Black\x00",
    b"Bold\x00",
    b"Book\x00",
    b"Light\x00",
    b"Medium\x00",
    b"Regular\x00",
    b"Roman\x00",
    b"Semibold\x00",
];
unsafe extern "C" fn get_unsigned(mut handle: rust_input_handle_t, mut n: i32) -> u32 {
    let mut v: u32 = 0_u32;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0i32) {
            break;
        }
        v = v
            .wrapping_mul(0x100u32)
            .wrapping_add(tt_get_unsigned_byte(handle) as u32)
    }
    v
}
/*
 * Read Header, Name INDEX, Top DICT INDEX, and String INDEX.
 */
#[no_mangle]
pub unsafe extern "C" fn cff_open(
    mut handle: rust_input_handle_t,
    mut offset: i32,
    mut n: i32,
) -> *mut cff_font {
    let mut cff: *mut cff_font = 0 as *mut cff_font; /* not used */
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    cff =
        new((1_u64).wrapping_mul(::std::mem::size_of::<cff_font>() as u64) as u32) as *mut cff_font;
    (*cff).fontname = 0 as *mut i8;
    (*cff).index = n;
    (*cff).handle = handle;
    (*cff).offset = offset as l_offset;
    (*cff).filter = 0i32;
    (*cff).flag = 0i32;
    (*cff).name = 0 as *mut cff_index;
    (*cff).topdict = 0 as *mut cff_dict;
    (*cff).gsubr = 0 as *mut cff_index;
    (*cff).encoding = 0 as *mut cff_encoding;
    (*cff).charsets = 0 as *mut cff_charsets;
    (*cff).fdselect = 0 as *mut cff_fdselect;
    (*cff).cstrings = 0 as *mut cff_index;
    (*cff).fdarray = 0 as *mut *mut cff_dict;
    (*cff).private = 0 as *mut *mut cff_dict;
    (*cff).subrs = 0 as *mut *mut cff_index;
    (*cff).num_glyphs = 0i32 as u16;
    (*cff).num_fds = 0i32 as u8;
    (*cff).string = 0 as *mut cff_index;
    (*cff)._string = 0 as *mut cff_index;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(0_u32) as ssize_t,
        0i32,
    );
    (*cff).header.major = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.minor = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.hdr_size = tt_get_unsigned_byte((*cff).handle);
    (*cff).header.offsize = tt_get_unsigned_byte((*cff).handle);
    if ((*cff).header.offsize as i32) < 1i32 || (*cff).header.offsize as i32 > 4i32 {
        panic!("invalid offsize data");
    }
    if (*cff).header.major as i32 > 1i32 || (*cff).header.minor as i32 > 0i32 {
        dpx_warning(
            b"%s: CFF version %u.%u not supported.\x00" as *const u8 as *const i8,
            b"CFF\x00" as *const u8 as *const i8,
            (*cff).header.major as i32,
            (*cff).header.minor as i32,
        );
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add((*cff).header.hdr_size as u32) as ssize_t,
        0i32,
    );
    /* Name INDEX */
    idx = cff_get_index(&*cff);
    if n > (*idx).count as i32 - 1i32 {
        warn!("{}: Invalid CFF fontset index number.", "CFF");
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    (*cff).name = idx;
    (*cff).fontname = cff_get_name(&*cff);
    /* Top DICT INDEX */
    idx = cff_get_index(&*cff);
    if n > (*idx).count as i32 - 1i32 {
        panic!("CFF Top DICT not exist...");
    }
    (*cff).topdict = cff_dict_unpack(
        (*idx)
            .data
            .offset(*(*idx).offset.offset(n as isize) as isize)
            .offset(-1),
        (*idx)
            .data
            .offset(*(*idx).offset.offset((n + 1i32) as isize) as isize)
            .offset(-1),
    );
    if (*cff).topdict.is_null() {
        panic!("Parsing CFF Top DICT data failed...");
    }
    cff_release_index(idx);
    if cff_dict_known(
        (*cff).topdict,
        b"CharstringType\x00" as *const u8 as *const i8,
    ) != 0
        && cff_dict_get(
            (*cff).topdict,
            b"CharstringType\x00" as *const u8 as *const i8,
            0i32,
        ) != 2i32 as f64
    {
        warn!("Only Type 2 Charstrings supported...");
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    if cff_dict_known(
        (*cff).topdict,
        b"SyntheticBase\x00" as *const u8 as *const i8,
    ) != 0
    {
        warn!("CFF Synthetic font not supported.");
        cff_close(cff);
        return 0 as *mut cff_font;
    }
    /* String INDEX */
    (*cff).string = cff_get_index(&*cff);
    /* offset to GSubr */
    (*cff).gsubr_offset = ttstub_input_seek((*cff).handle, 0i32 as ssize_t, 1i32)
        .wrapping_sub(offset as u64) as l_offset;
    /* Number of glyphs */
    offset = cff_dict_get(
        (*cff).topdict,
        b"CharStrings\x00" as *const u8 as *const i8,
        0i32,
    ) as i32;
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    (*cff).num_glyphs = tt_get_unsigned_pair((*cff).handle);
    /* Check for font type */
    if cff_dict_known((*cff).topdict, b"ROS\x00" as *const u8 as *const i8) != 0 {
        (*cff).flag |= 1i32 << 0i32
    } else {
        (*cff).flag |= 1i32 << 1i32
    }
    /* Check for encoding */
    if cff_dict_known((*cff).topdict, b"Encoding\x00" as *const u8 as *const i8) != 0 {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Encoding\x00" as *const u8 as *const i8,
            0i32,
        ) as i32;
        if offset == 0i32 {
            /* predefined */
            (*cff).flag |= 1i32 << 3i32
        } else if offset == 1i32 {
            (*cff).flag |= 1i32 << 4i32
        }
    } else {
        (*cff).flag |= 1i32 << 3i32
    }
    /* Check for charset */
    if cff_dict_known((*cff).topdict, b"charset\x00" as *const u8 as *const i8) != 0 {
        offset = cff_dict_get(
            (*cff).topdict,
            b"charset\x00" as *const u8 as *const i8,
            0i32,
        ) as i32;
        if offset == 0i32 {
            /* predefined */
            (*cff).flag |= 1i32 << 5i32
        } else if offset == 1i32 {
            (*cff).flag |= 1i32 << 6i32
        } else if offset == 2i32 {
            (*cff).flag |= 1i32 << 7i32
        }
    } else {
        (*cff).flag |= 1i32 << 5i32
    } /* seek back to GSubr */
    ttstub_input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add((*cff).gsubr_offset) as ssize_t,
        0i32,
    ); /* no trailing '\0' */
    return cff; /* Additional data in between header and
                 * Name INDEX ignored.
                 */
}
#[no_mangle]
pub unsafe extern "C" fn cff_close(mut cff: *mut cff_font) {
    let mut i: u16 = 0;
    if !cff.is_null() {
        free((*cff).fontname as *mut libc::c_void);
        if !(*cff).name.is_null() {
            cff_release_index((*cff).name);
        }
        if !(*cff).topdict.is_null() {
            cff_release_dict((*cff).topdict);
        }
        if !(*cff).string.is_null() {
            cff_release_index((*cff).string);
        }
        if !(*cff).gsubr.is_null() {
            cff_release_index((*cff).gsubr);
        }
        if !(*cff).encoding.is_null() {
            cff_release_encoding((*cff).encoding);
        }
        if !(*cff).charsets.is_null() {
            cff_release_charsets((*cff).charsets);
        }
        if !(*cff).fdselect.is_null() {
            cff_release_fdselect((*cff).fdselect);
        }
        if !(*cff).cstrings.is_null() {
            cff_release_index((*cff).cstrings);
        }
        if !(*cff).fdarray.is_null() {
            i = 0i32 as u16;
            while (i as i32) < (*cff).num_fds as i32 {
                if !(*(*cff).fdarray.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).fdarray.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).fdarray as *mut libc::c_void);
        }
        if !(*cff).private.is_null() {
            i = 0i32 as u16;
            while (i as i32) < (*cff).num_fds as i32 {
                if !(*(*cff).private.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).private.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).private as *mut libc::c_void);
        }
        if !(*cff).subrs.is_null() {
            i = 0i32 as u16;
            while (i as i32) < (*cff).num_fds as i32 {
                if !(*(*cff).subrs.offset(i as isize)).is_null() {
                    cff_release_index(*(*cff).subrs.offset(i as isize));
                }
                i = i.wrapping_add(1)
            }
            free((*cff).subrs as *mut libc::c_void);
        }
        if !(*cff)._string.is_null() {
            cff_release_index((*cff)._string);
        }
        free(cff as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_name(cff: &cff_font) -> *mut i8 {
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut len: l_offset = 0;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    idx = cff.name;
    len = (*(*idx).offset.offset((cff.index + 1i32) as isize))
        .wrapping_sub(*(*idx).offset.offset(cff.index as isize));
    fontname = new(
        (len.wrapping_add(1_u32) as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
    ) as *mut i8;
    memcpy(
        fontname as *mut libc::c_void,
        (*idx)
            .data
            .offset(*(*idx).offset.offset(cff.index as isize) as isize)
            .offset(-1) as *const libc::c_void,
        len as _,
    );
    *fontname.offset(len as isize) = '\u{0}' as i32 as i8;
    fontname
}
#[no_mangle]
pub unsafe extern "C" fn cff_set_name(mut cff: *mut cff_font, mut name: *mut i8) -> i32 {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    if strlen(name) > 127 {
        panic!("FontName string length too large...");
    }
    if !(*cff).name.is_null() {
        cff_release_index((*cff).name);
    }
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    (*cff).name = idx;
    (*idx).count = 1i32 as u16;
    (*idx).offsize = 1i32 as c_offsize;
    (*idx).offset =
        new((2_u64).wrapping_mul(::std::mem::size_of::<l_offset>() as u64) as u32) as *mut l_offset;
    *(*idx).offset.offset(0) = 1i32 as l_offset;
    *(*idx).offset.offset(1) = strlen(name).wrapping_add(1) as l_offset;
    (*idx).data =
        new((strlen(name) as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
    memmove(
        (*idx).data as *mut libc::c_void,
        name as *const libc::c_void,
        strlen(name),
    );
    (5usize).wrapping_add(strlen(name)) as _
}
#[no_mangle]
pub unsafe extern "C" fn cff_put_header(cff: &cff_font, mut dest: &mut [u8]) -> usize {
    /* We will set all offset (0) to four-byte integer. */
    dest[0..4].copy_from_slice(&[cff.header.major, cff.header.minor, 4, 4]);
    4
}
/* Only read header part but not body */
#[no_mangle]
pub unsafe extern "C" fn cff_get_index_header(cff: &cff_font) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    count = tt_get_unsigned_pair(cff.handle);
    (*idx).count = count;
    if count as i32 > 0i32 {
        (*idx).offsize = tt_get_unsigned_byte(cff.handle);
        if ((*idx).offsize as i32) < 1i32 || (*idx).offsize as i32 > 4i32 {
            panic!("invalid offsize data");
        }
        (*idx).offset = new(((count as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64)
            as u32) as *mut l_offset;
        i = 0i32 as u16;
        while (i as i32) < count as i32 {
            *(*idx).offset.offset(i as isize) = get_unsigned(cff.handle, (*idx).offsize as i32);
            i = i.wrapping_add(1)
        }
        if count as i32 == 0xffffi32 {
            ttstub_input_seek(
                cff.handle,
                ttstub_input_seek(cff.handle, 0i32 as ssize_t, 1i32)
                    .wrapping_add((*idx).offsize as u64) as ssize_t,
                0i32,
            );
        } else {
            *(*idx).offset.offset(i as isize) = get_unsigned(cff.handle, (*idx).offsize as i32)
        }
        if *(*idx).offset.offset(0) != 1_u32 {
            panic!("cff_get_index(): invalid index data");
        }
        (*idx).data = 0 as *mut u8
    } else {
        (*idx).offsize = 0i32 as c_offsize;
        (*idx).offset = 0 as *mut l_offset;
        (*idx).data = 0 as *mut u8
    }
    idx
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_index(cff: &cff_font) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    let mut length: i32 = 0;
    let mut nb_read: i32 = 0;
    let mut offset: i32 = 0;
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    count = tt_get_unsigned_pair(cff.handle);
    (*idx).count = count;
    if count as i32 > 0i32 {
        (*idx).offsize = tt_get_unsigned_byte(cff.handle);
        if ((*idx).offsize as i32) < 1i32 || (*idx).offsize as i32 > 4i32 {
            panic!("invalid offsize data");
        }
        (*idx).offset = new(((count as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64)
            as u32) as *mut l_offset;
        i = 0i32 as u16;
        while (i as i32) < count as i32 + 1i32 {
            *(*idx).offset.offset(i as isize) = get_unsigned(cff.handle, (*idx).offsize as i32);
            i = i.wrapping_add(1)
        }
        if *(*idx).offset.offset(0) != 1_u32 {
            panic!("Invalid CFF Index offset data");
        }
        length =
            (*(*idx).offset.offset(count as isize)).wrapping_sub(*(*idx).offset.offset(0)) as i32;
        (*idx).data =
            new((length as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
                as *mut u8;
        offset = 0i32;
        while length > 0i32 {
            nb_read = ttstub_input_read(
                cff.handle,
                ((*idx).data as *mut i8).offset(offset as isize),
                length as size_t,
            ) as i32;
            offset += nb_read;
            length -= nb_read
        }
    } else {
        (*idx).offsize = 0i32 as c_offsize;
        (*idx).offset = 0 as *mut l_offset;
        (*idx).data = 0 as *mut u8
    }
    idx
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_index(mut idx: *mut cff_index, mut dest: &mut [u8]) -> usize {
    let destlen = dest.len();
    let mut len = 0;
    let mut datalen: size_t = 0;
    let mut i: u16 = 0;
    if ((*idx).count as i32) < 1i32 {
        if destlen < 2 {
            panic!("Not enough space available...");
        }
        memset(dest.as_mut_ptr() as *mut libc::c_void, 0, 2);
        return 2;
    }
    len = cff_index_size(idx);
    datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1_u32) as size_t;
    if destlen < len {
        panic!("Not enough space available...");
    }
    dest[0] = ((*idx).count as i32 >> 8i32 & 0xffi32) as u8;
    dest = &mut dest[1..];
    dest[0] = ((*idx).count as i32 & 0xffi32) as u8;
    dest = &mut dest[1..];
    if datalen < 0xff {
        (*idx).offsize = 1i32 as c_offsize;
        dest[0] = 1i32 as u8;
        dest = &mut dest[1..];
        i = 0i32 as u16;
        while i as i32 <= (*idx).count as i32 {
            dest[0] = (*(*idx).offset.offset(i as isize) & 0xff_u32) as u8;
            dest = &mut dest[1..];
            i = i.wrapping_add(1)
        }
    } else if datalen < 0xffff {
        (*idx).offsize = 2i32 as c_offsize;
        dest[0] = 2i32 as u8;
        dest = &mut dest[1..];
        i = 0i32 as u16;
        while i as i32 <= (*idx).count as i32 {
            dest[0] = (*(*idx).offset.offset(i as isize) >> 8i32 & 0xff_u32) as u8;
            dest = &mut dest[1..];
            dest[0] = (*(*idx).offset.offset(i as isize) & 0xff_u32) as u8;
            dest = &mut dest[1..];
            i = i.wrapping_add(1)
        }
    } else if datalen < 0xffffff {
        (*idx).offsize = 3i32 as c_offsize;
        dest[0] = 3i32 as u8;
        dest = &mut dest[1..];
        i = 0i32 as u16;
        while i as i32 <= (*idx).count as i32 {
            dest[0] = (*(*idx).offset.offset(i as isize) >> 16i32 & 0xff_u32) as u8;
            dest = &mut dest[1..];
            dest[0] = (*(*idx).offset.offset(i as isize) >> 8i32 & 0xff_u32) as u8;
            dest = &mut dest[1..];
            dest[0] = (*(*idx).offset.offset(i as isize) & 0xff_u32) as u8;
            dest = &mut dest[1..];
            i = i.wrapping_add(1)
        }
    } else {
        (*idx).offsize = 4i32 as c_offsize;
        dest[0] = 4i32 as u8;
        dest = &mut dest[1..];
        i = 0i32 as u16;
        while i as i32 <= (*idx).count as i32 {
            dest[0] = (*(*idx).offset.offset(i as isize) >> 24i32 & 0xff_u32) as u8;
            dest = &mut dest[1..];
            dest[0] = (*(*idx).offset.offset(i as isize) >> 16i32 & 0xff_u32) as u8;
            dest = &mut dest[1..];
            dest[0] = (*(*idx).offset.offset(i as isize) >> 8i32 & 0xff_u32) as u8;
            dest = &mut dest[1..];
            dest[0] = (*(*idx).offset.offset(i as isize) & 0xff_u32) as u8;
            dest = &mut dest[1..];
            i = i.wrapping_add(1)
        }
    }
    memmove(
        dest.as_mut_ptr() as *mut libc::c_void,
        (*idx).data as *const libc::c_void,
        (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1) as _,
    );
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_index_size(mut idx: *mut cff_index) -> usize {
    if (*idx).count as i32 > 0 {
        let mut datalen: l_offset = 0;
        datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1_u32);
        if (datalen as u64) < 0xff {
            (*idx).offsize = 1 as c_offsize
        } else if (datalen as u64) < 0xffff {
            (*idx).offsize = 2 as c_offsize
        } else if (datalen as u64) < 0xffffff {
            (*idx).offsize = 3 as c_offsize
        } else {
            (*idx).offsize = 4 as c_offsize
        }
        ((3 + (*idx).offsize as i32 * ((*idx).count as i32 + 1)) as u32).wrapping_add(datalen)
            as usize
    } else {
        2
    }
}
#[no_mangle]
pub unsafe extern "C" fn cff_new_index(mut count: u16) -> *mut cff_index {
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    idx = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_index>() as u64) as u32)
        as *mut cff_index;
    (*idx).count = count;
    (*idx).offsize = 0i32 as c_offsize;
    if count as i32 > 0i32 {
        (*idx).offset = new(((count as i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64)
            as u32) as *mut l_offset;
        *(*idx).offset.offset(0) = 1i32 as l_offset
    } else {
        (*idx).offset = 0 as *mut l_offset
    }
    (*idx).data = 0 as *mut u8;
    idx
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_index(mut idx: *mut cff_index) {
    if !idx.is_null() {
        if !(*idx).data.is_null() {
            free((*idx).data as *mut libc::c_void);
        }
        if !(*idx).offset.is_null() {
            free((*idx).offset as *mut libc::c_void);
        }
        free(idx as *mut libc::c_void);
    };
}
/* Strings */
#[no_mangle]
pub unsafe extern "C" fn cff_get_string(mut cff: *const cff_font, mut id: s_SID) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    if (id as i32) < 391i32 {
        len = strlen(cff_stdstr[id as usize].as_ptr() as *const i8) as i32;
        result = new(
            ((len + 1i32) as u32 as u64).wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32,
        ) as *mut i8;
        memcpy(
            result as *mut libc::c_void,
            cff_stdstr[id as usize].as_ptr() as *const libc::c_void,
            len as _,
        );
        *result.offset(len as isize) = '\u{0}' as i32 as i8
    } else if !cff.is_null() && !(*cff).string.is_null() {
        let mut strings: *mut cff_index = (*cff).string;
        id = (id as i32 - 391i32) as s_SID;
        if (id as i32) < (*strings).count as i32 {
            len = (*(*strings).offset.offset((id as i32 + 1i32) as isize))
                .wrapping_sub(*(*strings).offset.offset(id as isize)) as i32;
            result = new(((len + 1i32) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64) as u32)
                as *mut i8;
            memmove(
                result as *mut libc::c_void,
                (*strings)
                    .data
                    .offset(*(*strings).offset.offset(id as isize) as isize)
                    .offset(-1) as *const libc::c_void,
                len as _,
            );
            *result.offset(len as isize) = '\u{0}' as i32 as i8
        }
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_sid(mut cff: *mut cff_font, mut str: *const i8) -> i32 {
    let mut i: u16 = 0;
    if cff.is_null() || str.is_null() {
        return -1i32;
    }
    /* I search String INDEX first. */
    if !cff.is_null() && !(*cff).string.is_null() {
        let mut idx: *mut cff_index = (*cff).string;
        i = 0i32 as u16;
        while (i as i32) < (*idx).count as i32 {
            if strlen(str)
                == (*(*idx).offset.offset((i + 1) as isize))
                    .wrapping_sub(*(*idx).offset.offset(i as isize)) as _
                && memcmp(
                    str as *const libc::c_void,
                    (*idx)
                        .data
                        .offset(*(*idx).offset.offset(i as isize) as isize)
                        .offset(-1) as *const libc::c_void,
                    strlen(str),
                ) == 0
            {
                return i as i32 + 391i32;
            }
            i = i.wrapping_add(1)
        }
    }
    i = 0i32 as u16;
    while (i as i32) < 391i32 {
        if streq_ptr(str, cff_stdstr[i as usize].as_ptr() as *const i8) {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    -1i32
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_seac_sid(mut cff: *mut cff_font, mut str: *const i8) -> i32 {
    let mut i: u16 = 0;
    if cff.is_null() || str.is_null() {
        return -1i32;
    }
    i = 0i32 as u16;
    while (i as i32) < 391i32 {
        if streq_ptr(str, cff_stdstr[i as usize].as_ptr() as *const i8) {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    -1i32
}
unsafe extern "C" fn cff_match_string(cff: &cff_font, mut str: *const i8, mut sid: s_SID) -> i32 {
    let mut i: u16 = 0;
    if (sid as i32) < 391i32 {
        return if streq_ptr(str, cff_stdstr[sid as usize].as_ptr() as *const i8) as i32 != 0 {
            1i32
        } else {
            0i32
        };
    } else {
        i = (sid as i32 - 391i32) as u16;
        if cff.string.is_null() || i as i32 >= (*cff.string).count as i32 {
            panic!("Invalid SID");
        }
        if strlen(str)
            == (*(*cff.string).offset.offset((i + 1) as isize))
                .wrapping_sub(*(*cff.string).offset.offset(i as isize)) as _
        {
            return if memcmp(
                str as *const libc::c_void,
                (*cff.string)
                    .data
                    .offset(*(*cff.string).offset.offset(i as isize) as isize)
                    .offset(-1) as *const libc::c_void,
                strlen(str),
            ) == 0
            {
                1i32
            } else {
                0i32
            };
        }
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn cff_update_string(cff: &mut cff_font) {
    /*if cff.is_null() {
        panic!("CFF font not opened.");
    }*/
    if !cff.string.is_null() {
        cff_release_index((*cff).string);
    }
    cff.string = (*cff)._string;
    cff._string = 0 as *mut cff_index;
}
/* String */
#[no_mangle]
pub unsafe extern "C" fn cff_add_string(
    mut cff: &mut cff_font,
    mut str: *const i8,
    mut unique: i32,
) -> s_SID
/* Setting unique == 1 eliminates redundant or predefined strings. */ {
    let mut idx: u16 = 0;
    let mut strings: *mut cff_index = 0 as *mut cff_index;
    let mut offset: l_offset = 0;
    let mut size: l_offset = 0;
    let mut len: size_t = strlen(str) as _;
    if cff._string.is_null() {
        cff._string = cff_new_index(0i32 as u16)
    }
    strings = cff._string;
    if unique != 0 {
        /* TODO: do binary search to speed things up */
        idx = 0i32 as u16;
        while (idx as i32) < 391i32 {
            if streq_ptr(cff_stdstr[idx as usize].as_ptr() as *const i8, str) {
                return idx;
            }
            idx = idx.wrapping_add(1)
        }
        idx = 0i32 as u16;
        while (idx as i32) < (*strings).count as i32 {
            size = (*(*strings).offset.offset((idx as i32 + 1i32) as isize))
                .wrapping_sub(*(*strings).offset.offset(idx as isize));
            offset = *(*strings).offset.offset(idx as isize);
            if size as u64 == len
                && memcmp(
                    (*strings).data.offset(offset as isize).offset(-1) as *const libc::c_void,
                    str as *const libc::c_void,
                    len as _,
                ) == 0
            {
                return (idx as i32 + 391i32) as s_SID;
            }
            idx = idx.wrapping_add(1)
        }
    }
    offset = if (*strings).count as i32 > 0i32 {
        *(*strings).offset.offset((*strings).count as isize)
    } else {
        1_u32
    };
    (*strings).offset = renew(
        (*strings).offset as *mut libc::c_void,
        (((*strings).count as i32 + 2i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<l_offset>() as u64) as u32,
    ) as *mut l_offset;
    if (*strings).count as i32 == 0i32 {
        *(*strings).offset.offset(0) = 1i32 as l_offset
    }
    idx = (*strings).count;
    (*strings).count = ((*strings).count as i32 + 1i32) as u16;
    *(*strings).offset.offset((*strings).count as isize) =
        (offset as u64).wrapping_add(len) as l_offset;
    (*strings).data = renew(
        (*strings).data as *mut libc::c_void,
        ((offset as u64).wrapping_add(len).wrapping_sub(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
    ) as *mut u8;
    memcpy(
        (*strings).data.offset(offset as isize).offset(-1) as *mut libc::c_void,
        str as *const libc::c_void,
        len as _,
    );
    (idx as i32 + 391i32) as s_SID
}
/*
 * Encoding and Charset
 *
 *  Encoding and Charset arrays always begin with GID = 1.
 */
#[no_mangle]
pub unsafe extern "C" fn cff_read_encoding(cff: &mut cff_font) -> i32 {
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut i: u8 = 0;
    if cff.topdict.is_null() {
        panic!("Top DICT data not found");
    }
    if cff_dict_known(cff.topdict, b"Encoding\x00" as *const u8 as *const i8) == 0 {
        cff.flag |= 1i32 << 3i32;
        cff.encoding = 0 as *mut cff_encoding;
        return 0i32;
    }
    offset = cff_dict_get(cff.topdict, b"Encoding\x00" as *const u8 as *const i8, 0i32) as i32;
    if offset == 0i32 {
        /* predefined */
        cff.flag |= 1i32 << 3i32;
        cff.encoding = 0 as *mut cff_encoding;
        return 0i32;
    } else {
        if offset == 1i32 {
            cff.flag |= 1i32 << 4i32;
            cff.encoding = 0 as *mut cff_encoding;
            return 0i32;
        }
    }
    ttstub_input_seek(
        cff.handle,
        cff.offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    encoding = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_encoding>() as u64) as u32)
        as *mut cff_encoding;
    cff.encoding = encoding;
    (*encoding).format = tt_get_unsigned_byte(cff.handle);
    length = 1i32;
    match (*encoding).format as i32 & !0x80i32 {
        0 => {
            (*encoding).num_entries = tt_get_unsigned_byte(cff.handle);
            (*encoding).data.codes = new(((*encoding).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                as u32) as *mut u8;
            i = 0i32 as u8;
            while (i as i32) < (*encoding).num_entries as i32 {
                *(*encoding).data.codes.offset(i as isize) = tt_get_unsigned_byte(cff.handle);
                i = i.wrapping_add(1)
            }
            length += (*encoding).num_entries as i32 + 1i32
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            (*encoding).num_entries = tt_get_unsigned_byte(cff.handle);
            ranges = new(((*encoding).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<cff_range1>() as u64)
                as u32) as *mut cff_range1;
            (*encoding).data.range1 = ranges;
            i = 0i32 as u8;
            while (i as i32) < (*encoding).num_entries as i32 {
                (*ranges.offset(i as isize)).first = tt_get_unsigned_byte(cff.handle) as s_SID;
                (*ranges.offset(i as isize)).n_left = tt_get_unsigned_byte(cff.handle);
                i = i.wrapping_add(1)
            }
            length += (*encoding).num_entries as i32 * 2i32 + 1i32
        }
        _ => {
            free(encoding as *mut libc::c_void);
            panic!("Unknown Encoding format");
        }
    }
    /* Supplementary data */
    if (*encoding).format as i32 & 0x80i32 != 0 {
        let mut map: *mut cff_map = 0 as *mut cff_map;
        (*encoding).num_supps = tt_get_unsigned_byte(cff.handle);
        map = new(((*encoding).num_supps as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<cff_map>() as u64) as u32)
            as *mut cff_map;
        (*encoding).supp = map;
        i = 0i32 as u8;
        while (i as i32) < (*encoding).num_supps as i32 {
            (*map.offset(i as isize)).code = tt_get_unsigned_byte(cff.handle);
            (*map.offset(i as isize)).glyph = tt_get_unsigned_pair(cff.handle);
            i = i.wrapping_add(1)
            /* SID */
        }
        length += (*encoding).num_supps as i32 * 3i32 + 1i32
    } else {
        (*encoding).num_supps = 0i32 as u8;
        (*encoding).supp = 0 as *mut cff_map
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_encoding(cff: &cff_font, dest: &mut [u8]) -> usize {
    let destlen = dest.len();
    let mut len = 0_usize;
    if cff.flag & (1i32 << 3i32 | 1i32 << 4i32) != 0 || cff.encoding.is_null() {
        return 0;
    }
    let encoding = &*cff.encoding;
    dest[len] = encoding.format;
    len += 1;
    dest[len] = encoding.num_entries;
    len += 1;
    match encoding.format as i32 & !0x80i32 {
        0 => {
            for i in 0..encoding.num_entries as isize {
                dest[len] = *encoding.data.codes.offset(i);
                len += 1;
            }
        }
        1 => {
            for i in 0..encoding.num_entries as isize {
                dest[len] = ((*encoding.data.range1.offset(i)).first as i32 & 0xffi32) as u8;
                len += 1;
                dest[len] = (*encoding.data.range1.offset(i)).n_left;
                len += 1;
            }
        }
        _ => {
            panic!("Unknown Encoding format");
        }
    }
    if encoding.format as i32 & 0x80 != 0 {
        dest[len] = encoding.num_supps;
        len += 1;
        for i in 0..encoding.num_supps as isize {
            dest[len] = (*encoding.supp.offset(i)).code;
            len += 1;
            dest[len..len + 2].copy_from_slice(&(*encoding.supp.offset(i)).glyph.to_be_bytes());
            len += 2;
        }
    }
    len
}
/* input: code, output: glyph index */
#[no_mangle]
pub unsafe extern "C" fn cff_encoding_lookup(cff: &cff_font, mut code: u8) -> u16 {
    let mut gid: u16 = 0i32 as u16;
    let mut encoding: *mut cff_encoding = 0 as *mut cff_encoding;
    let mut i: u16 = 0;
    if cff.flag & (1i32 << 3i32 | 1i32 << 4i32) != 0 {
        panic!("Predefined CFF encoding not supported yet");
    } else {
        if cff.encoding.is_null() {
            panic!("Encoding data not available");
        }
    }
    encoding = cff.encoding;
    gid = 0i32 as u16;
    match (*encoding).format as i32 & !0x80i32 {
        0 => {
            i = 0i32 as u16;
            while (i as i32) < (*encoding).num_entries as i32 {
                if code as i32 == *(*encoding).data.codes.offset(i as isize) as i32 {
                    gid = (i as i32 + 1i32) as u16;
                    break;
                } else {
                    i = i.wrapping_add(1)
                }
            }
        }
        1 => {
            i = 0i32 as u16;
            while (i as i32) < (*encoding).num_entries as i32 {
                if code as i32 >= (*(*encoding).data.range1.offset(i as isize)).first as i32
                    && code as i32
                        <= (*(*encoding).data.range1.offset(i as isize)).first as i32
                            + (*(*encoding).data.range1.offset(i as isize)).n_left as i32
                {
                    gid = (gid as i32
                        + (code as i32
                            - (*(*encoding).data.range1.offset(i as isize)).first as i32
                            + 1i32)) as u16;
                    break;
                } else {
                    gid = (gid as i32
                        + ((*(*encoding).data.range1.offset(i as isize)).n_left as i32 + 1i32))
                        as u16;
                    i = i.wrapping_add(1)
                }
            }
            if i as i32 == (*encoding).num_entries as i32 {
                gid = 0i32 as u16
            }
        }
        _ => {
            panic!("Unknown Encoding format.");
        }
    }
    /* Supplementary data */
    if gid as i32 == 0i32 && (*encoding).format as i32 & 0x80i32 != 0 {
        let mut map: *mut cff_map = 0 as *mut cff_map;
        if (*encoding).supp.is_null() {
            panic!("No CFF supplementary encoding data read.");
        }
        map = (*encoding).supp;
        i = 0i32 as u16;
        while (i as i32) < (*encoding).num_supps as i32 {
            if code as i32 == (*map.offset(i as isize)).code as i32 {
                gid = cff_charsets_lookup(cff, (*map.offset(i as isize)).glyph);
                break;
            } else {
                i = i.wrapping_add(1)
            }
        }
    }
    gid
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_encoding(mut encoding: *mut cff_encoding) {
    if !encoding.is_null() {
        match (*encoding).format as i32 & !0x80i32 {
            0 => {
                free((*encoding).data.codes as *mut libc::c_void);
            }
            1 => {
                free((*encoding).data.range1 as *mut libc::c_void);
            }
            _ => {
                panic!("Unknown Encoding format.");
            }
        }
        if (*encoding).format as i32 & 0x80i32 != 0 {
            free((*encoding).supp as *mut libc::c_void);
        }
        free(encoding as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_charsets(cff: &mut cff_font) -> i32 {
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut count: u16 = 0;
    let mut i: u16 = 0;
    if cff.topdict.is_null() {
        panic!("Top DICT not available");
    }
    if cff_dict_known(cff.topdict, b"charset\x00" as *const u8 as *const i8) == 0 {
        cff.flag |= 1i32 << 5i32;
        cff.charsets = 0 as *mut cff_charsets;
        return 0i32;
    }
    offset = cff_dict_get(cff.topdict, b"charset\x00" as *const u8 as *const i8, 0i32) as i32;
    if offset == 0i32 {
        /* predefined */
        cff.flag |= 1i32 << 5i32;
        cff.charsets = 0 as *mut cff_charsets;
        return 0i32;
    } else {
        if offset == 1i32 {
            cff.flag |= 1i32 << 6i32;
            cff.charsets = 0 as *mut cff_charsets;
            return 0i32;
        } else {
            if offset == 2i32 {
                cff.flag |= 1i32 << 7i32;
                cff.charsets = 0 as *mut cff_charsets;
                return 0i32;
            }
        }
    }
    ttstub_input_seek(
        cff.handle,
        cff.offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    charset = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_charsets>() as u64) as u32)
        as *mut cff_charsets;
    cff.charsets = charset;
    (*charset).format = tt_get_unsigned_byte(cff.handle);
    (*charset).num_entries = 0i32 as u16;
    count = (cff.num_glyphs as i32 - 1i32) as u16;
    length = 1i32;
    /* Not sure. Not well documented. */
    match (*charset).format as i32 {
        0 => {
            (*charset).num_entries = (cff.num_glyphs as i32 - 1i32) as u16; /* no .notdef */
            (*charset).data.glyphs = new(((*charset).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<s_SID>() as u64)
                as u32) as *mut s_SID; /* no-overrap */
            length += (*charset).num_entries as i32 * 2i32; /* non-overrapping */
            i = 0i32 as u16; /* or CID */
            while (i as i32) < (*charset).num_entries as i32 {
                *(*charset).data.glyphs.offset(i as isize) = tt_get_unsigned_pair(cff.handle);
                i = i.wrapping_add(1)
            }
            count = 0i32 as u16
        }
        1 => {
            let mut ranges: *mut cff_range1 = 0 as *mut cff_range1;
            while count as i32 > 0i32 && ((*charset).num_entries as i32) < cff.num_glyphs as i32 {
                ranges = renew(
                    ranges as *mut libc::c_void,
                    (((*charset).num_entries as i32 + 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<cff_range1>() as u64)
                        as u32,
                ) as *mut cff_range1;
                (*ranges.offset((*charset).num_entries as isize)).first =
                    tt_get_unsigned_pair(cff.handle);
                (*ranges.offset((*charset).num_entries as isize)).n_left =
                    tt_get_unsigned_byte(cff.handle);
                count = (count as i32
                    - ((*ranges.offset((*charset).num_entries as isize)).n_left as i32 + 1i32))
                    as u16;
                (*charset).num_entries = ((*charset).num_entries as i32 + 1i32) as u16;
                (*charset).data.range1 = ranges
            }
            length += (*charset).num_entries as i32 * 3i32
        }
        2 => {
            let mut ranges_0: *mut cff_range2 = 0 as *mut cff_range2;
            while count as i32 > 0i32 && ((*charset).num_entries as i32) < cff.num_glyphs as i32 {
                ranges_0 = renew(
                    ranges_0 as *mut libc::c_void,
                    (((*charset).num_entries as i32 + 1i32) as u32 as u64)
                        .wrapping_mul(::std::mem::size_of::<cff_range2>() as u64)
                        as u32,
                ) as *mut cff_range2;
                (*ranges_0.offset((*charset).num_entries as isize)).first =
                    tt_get_unsigned_pair(cff.handle);
                (*ranges_0.offset((*charset).num_entries as isize)).n_left =
                    tt_get_unsigned_pair(cff.handle);
                count = (count as i32
                    - ((*ranges_0.offset((*charset).num_entries as isize)).n_left as i32 + 1i32))
                    as u16;
                (*charset).num_entries = ((*charset).num_entries as i32 + 1i32) as u16
            }
            (*charset).data.range2 = ranges_0;
            length += (*charset).num_entries as i32 * 4i32
        }
        _ => {
            free(charset as *mut libc::c_void);
            panic!("Unknown Charset format");
        }
    }
    if count as i32 > 0i32 {
        panic!("Charset data possibly broken");
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_charsets(cff: &cff_font, dest: &mut [u8]) -> usize {
    let destlen = dest.len();
    let mut len = 0;
    let mut i: u16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    if cff.flag & (1 << 5 | 1 << 6 | 1 << 7) != 0 || cff.charsets.is_null() {
        return 0;
    }
    if destlen < 1 {
        panic!("in cff_pack_charsets(): Buffer overflow");
    }
    charset = cff.charsets;
    dest[len] = (*charset).format;
    len += 1;
    match (*charset).format as i32 {
        0 => {
            if destlen < len + (*charset).num_entries as usize * 2 {
                panic!("in cff_pack_charsets(): Buffer overflow");
            }
            for i in 0..((*charset).num_entries as isize) {
                let mut sid: s_SID = *(*charset).data.glyphs.offset(i);
                dest[len..len + 2].copy_from_slice(&sid.to_be_bytes());
                len += 2;
            }
        }
        1 => {
            if destlen < len + (*charset).num_entries as usize * 3 {
                panic!("in cff_pack_charsets(): Buffer overflow");
            }
            for i in 0..((*charset).num_entries as isize) {
                let range = *(*charset).data.range1.offset(i);
                dest[len..len + 2].copy_from_slice(&range.first.to_be_bytes());
                len += 2;
                dest[len] = range.n_left;
                len += 1;
            }
        }
        2 => {
            if destlen < len + (*charset).num_entries as usize * 4 {
                panic!("in cff_pack_charsets(): Buffer overflow");
            }
            for i in 0..((*charset).num_entries as isize) {
                let range = *(*charset).data.range2.offset(i);
                dest[len..len + 2].copy_from_slice(&range.first.to_be_bytes());
                len += 2;
                dest[len..len + 2].copy_from_slice(&range.n_left.to_be_bytes());
                len += 2;
            }
        }
        _ => {
            panic!("Unknown Charset format");
        }
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_get_glyphname(cff: &cff_font, gid: u16) -> *mut i8 {
    let mut sid: s_SID = 0;
    sid = cff_charsets_lookup_inverse(cff, gid);
    cff_get_string(cff, sid)
}
#[no_mangle]
pub unsafe extern "C" fn cff_glyph_lookup(cff: &cff_font, mut glyph: *const i8) -> u16 {
    let mut gid: u16 = 0;
    let mut charset: *mut cff_charsets = 0 as *mut cff_charsets;
    let mut i: u16 = 0;
    let mut n: u16 = 0;
    if cff.flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        panic!("Predefined CFF charsets not supported yet");
    } else {
        if cff.charsets.is_null() {
            panic!("Charsets data not available");
        }
    }
    /* .notdef always have glyph index 0 */
    if glyph.is_null() || streq_ptr(glyph, b".notdef\x00" as *const u8 as *const i8) as i32 != 0 {
        return 0i32 as u16;
    }
    charset = cff.charsets;
    gid = 0i32 as u16;
    match (*charset).format as i32 {
        0 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                gid = gid.wrapping_add(1);
                if cff_match_string(cff, glyph, *(*charset).data.glyphs.offset(i as isize)) != 0 {
                    return gid;
                }
                i = i.wrapping_add(1)
            }
        }
        1 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                n = 0i32 as u16;
                while n as i32 <= (*(*charset).data.range1.offset(i as isize)).n_left as i32 {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range1.offset(i as isize)).first as i32 + n as i32)
                            as s_SID,
                    ) != 0
                    {
                        return gid;
                    }
                    n = n.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
        2 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                n = 0i32 as u16;
                while n as i32 <= (*(*charset).data.range2.offset(i as isize)).n_left as i32 {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range2.offset(i as isize)).first as i32 + n as i32)
                            as s_SID,
                    ) != 0
                    {
                        return gid;
                    }
                    n = n.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
        _ => {
            panic!("Unknown Charset format");
        }
    }
    return 0i32 as u16;
    /* not found, returns .notdef */
}
/* Input : SID or CID (16-bit unsigned int)
 * Output: glyph index
 */
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup(cff: &cff_font, mut cid: u16) -> u16 {
    if cff.flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        panic!("Predefined CFF charsets not supported yet");
    } else {
        if cff.charsets.is_null() {
            panic!("Charsets data not available");
        }
    }
    cff_charsets_lookup_gid(cff.charsets, cid)
}
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_gid(
    mut charset: *mut cff_charsets,
    mut cid: u16,
) -> u16 {
    let mut gid: u16 = 0i32 as u16;
    let mut i: u16 = 0;
    if cid as i32 == 0i32 {
        return 0i32 as u16;
        /* GID 0 (.notdef) */
    }
    match (*charset).format as i32 {
        0 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                if cid as i32 == *(*charset).data.glyphs.offset(i as isize) as i32 {
                    gid = (i as i32 + 1i32) as u16;
                    return gid;
                }
                i = i.wrapping_add(1)
            }
        }
        1 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                if cid as i32 >= (*(*charset).data.range1.offset(i as isize)).first as i32
                    && cid as i32
                        <= (*(*charset).data.range1.offset(i as isize)).first as i32
                            + (*(*charset).data.range1.offset(i as isize)).n_left as i32
                {
                    gid = (gid as i32
                        + (cid as i32 - (*(*charset).data.range1.offset(i as isize)).first as i32
                            + 1i32)) as u16;
                    return gid;
                }
                gid = (gid as i32
                    + ((*(*charset).data.range1.offset(i as isize)).n_left as i32 + 1i32))
                    as u16;
                i = i.wrapping_add(1)
            }
        }
        2 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                if cid as i32 >= (*(*charset).data.range2.offset(i as isize)).first as i32
                    && cid as i32
                        <= (*(*charset).data.range2.offset(i as isize)).first as i32
                            + (*(*charset).data.range2.offset(i as isize)).n_left as i32
                {
                    gid = (gid as i32
                        + (cid as i32 - (*(*charset).data.range2.offset(i as isize)).first as i32
                            + 1i32)) as u16;
                    return gid;
                }
                gid = (gid as i32
                    + ((*(*charset).data.range2.offset(i as isize)).n_left as i32 + 1i32))
                    as u16;
                i = i.wrapping_add(1)
            }
        }
        _ => {
            panic!("Unknown Charset format");
        }
    }
    return 0i32 as u16;
    /* not found */
}
/* Input : GID
 * Output: SID/CID (u16)
 */
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_inverse(cff: &cff_font, gid: u16) -> u16 {
    if cff.flag & (1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) != 0 {
        panic!("Predefined CFF charsets not supported yet");
    } else {
        if cff.charsets.is_null() {
            panic!("Charsets data not available");
        }
    }
    if gid as i32 == 0i32 {
        return 0i32 as u16;
        /* .notdef */
    }
    cff_charsets_lookup_cid(cff.charsets, gid)
}
#[no_mangle]
pub unsafe extern "C" fn cff_charsets_lookup_cid(
    mut charset: *mut cff_charsets,
    mut gid: u16,
) -> u16 {
    let mut sid: u16 = 0i32 as u16;
    let mut i: u16 = 0;
    match (*charset).format as i32 {
        0 => {
            if gid as i32 - 1i32 >= (*charset).num_entries as i32 {
                panic!("Invalid GID.");
            }
            sid = *(*charset).data.glyphs.offset((gid as i32 - 1i32) as isize)
        }
        1 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                if gid as i32 <= (*(*charset).data.range1.offset(i as isize)).n_left as i32 + 1i32 {
                    sid = (gid as i32 + (*(*charset).data.range1.offset(i as isize)).first as i32
                        - 1i32) as u16;
                    break;
                } else {
                    gid = (gid as i32
                        - ((*(*charset).data.range1.offset(i as isize)).n_left as i32 + 1i32))
                        as u16;
                    i = i.wrapping_add(1)
                }
            }
            if i as i32 == (*charset).num_entries as i32 {
                panic!("Invalid GID");
            }
        }
        2 => {
            i = 0i32 as u16;
            while (i as i32) < (*charset).num_entries as i32 {
                if gid as i32 <= (*(*charset).data.range2.offset(i as isize)).n_left as i32 + 1i32 {
                    sid = (gid as i32 + (*(*charset).data.range2.offset(i as isize)).first as i32
                        - 1i32) as u16;
                    break;
                } else {
                    gid = (gid as i32
                        - ((*(*charset).data.range2.offset(i as isize)).n_left as i32 + 1i32))
                        as u16;
                    i = i.wrapping_add(1)
                }
            }
            if i as i32 == (*charset).num_entries as i32 {
                panic!("Invalid GID");
            }
        }
        _ => {
            panic!("Unknown Charset format");
        }
    }
    sid
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_charsets(mut charset: *mut cff_charsets) {
    if !charset.is_null() {
        match (*charset).format as i32 {
            0 => {
                free((*charset).data.glyphs as *mut libc::c_void);
            }
            1 => {
                free((*charset).data.range1 as *mut libc::c_void);
            }
            2 => {
                free((*charset).data.range2 as *mut libc::c_void);
            }
            _ => {}
        }
        free(charset as *mut libc::c_void);
    };
}
/* CID-Keyed font specific */
#[no_mangle]
pub unsafe extern "C" fn cff_read_fdselect(cff: &mut cff_font) -> i32 {
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    let mut offset: i32 = 0;
    let mut length: i32 = 0;
    let mut i: u16 = 0;
    if cff.topdict.is_null() {
        panic!("Top DICT not available");
    }
    if cff.flag & 1i32 << 0i32 == 0 {
        return 0i32;
    }
    offset = cff_dict_get(cff.topdict, b"FDSelect\x00" as *const u8 as *const i8, 0i32) as i32;
    ttstub_input_seek(
        cff.handle,
        cff.offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    fdsel = new((1_u64).wrapping_mul(::std::mem::size_of::<cff_fdselect>() as u64) as u32)
        as *mut cff_fdselect;
    cff.fdselect = fdsel;
    (*fdsel).format = tt_get_unsigned_byte(cff.handle);
    length = 1i32;
    match (*fdsel).format as i32 {
        0 => {
            (*fdsel).num_entries = cff.num_glyphs;
            (*fdsel).data.fds = new(((*fdsel).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                as u32) as *mut u8;
            i = 0i32 as u16;
            while (i as i32) < (*fdsel).num_entries as i32 {
                *(*fdsel).data.fds.offset(i as isize) = tt_get_unsigned_byte(cff.handle);
                i = i.wrapping_add(1)
            }
            length += (*fdsel).num_entries as i32
        }
        3 => {
            let mut ranges: *mut cff_range3 = 0 as *mut cff_range3;
            (*fdsel).num_entries = tt_get_unsigned_pair(cff.handle);
            ranges = new(((*fdsel).num_entries as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<cff_range3>() as u64)
                as u32) as *mut cff_range3;
            (*fdsel).data.ranges = ranges;
            i = 0i32 as u16;
            while (i as i32) < (*fdsel).num_entries as i32 {
                (*ranges.offset(i as isize)).first = tt_get_unsigned_pair(cff.handle);
                (*ranges.offset(i as isize)).fd = tt_get_unsigned_byte(cff.handle);
                i = i.wrapping_add(1)
            }
            if (*ranges.offset(0)).first as i32 != 0i32 {
                panic!("Range not starting with 0.");
            }
            if cff.num_glyphs as i32 != tt_get_unsigned_pair(cff.handle) as i32 {
                panic!("Sentinel value mismatched with number of glyphs.");
            }
            length += (*fdsel).num_entries as i32 * 3i32 + 4i32
        }
        _ => {
            free(fdsel as *mut libc::c_void);
            panic!("Unknown FDSelect format.");
        }
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn cff_pack_fdselect(cff: &cff_font, dest: &mut [u8]) -> usize {
    let destlen = dest.len();
    let mut len = 0;
    let mut i: u16 = 0;
    if cff.fdselect.is_null() {
        return 0;
    }
    let fdsel = &*cff.fdselect;
    dest[len] = fdsel.format;
    len += 1;
    match fdsel.format as i32 {
        0 => {
            if fdsel.num_entries != cff.num_glyphs {
                panic!("in cff_pack_fdselect(): Invalid data");
            }
            for i in 0..fdsel.num_entries as isize {
                dest[len] = *fdsel.data.fds.offset(i);
                len += 1;
            }
        }
        3 => {
            len += 2;
            for i in 0..fdsel.num_entries as isize {
                dest[len..len + 2]
                    .copy_from_slice(&(*fdsel.data.ranges.offset(i)).first.to_be_bytes());
                len += 2;
                dest[len] = (*fdsel.data.ranges.offset(i)).fd;
                len += 1;
            }
            dest[len..len + 2].copy_from_slice(&cff.num_glyphs.to_be_bytes());
            len += 2;
            dest[1] = (len / 3 - 1 >> 8 & 0xff) as u8;
            dest[2] = (len / 3 - 1 & 0xff) as u8
        }
        _ => {
            panic!("Unknown FDSelect format.");
        }
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_release_fdselect(mut fdselect: *mut cff_fdselect) {
    if !fdselect.is_null() {
        if (*fdselect).format as i32 == 0i32 {
            free((*fdselect).data.fds as *mut libc::c_void);
        } else if (*fdselect).format as i32 == 3i32 {
            free((*fdselect).data.ranges as *mut libc::c_void);
        }
        free(fdselect as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_fdselect_lookup(cff: &cff_font, mut gid: u16) -> u8 {
    let mut fd: u8 = 0xffi32 as u8;
    let mut fdsel: *mut cff_fdselect = 0 as *mut cff_fdselect;
    if cff.fdselect.is_null() {
        panic!("in cff_fdselect_lookup(): FDSelect not available");
    }
    fdsel = cff.fdselect;
    if gid as i32 >= cff.num_glyphs as i32 {
        panic!("in cff_fdselect_lookup(): Invalid glyph index");
    }
    match (*fdsel).format as i32 {
        0 => fd = *(*fdsel).data.fds.offset(gid as isize),
        3 => {
            if gid as i32 == 0i32 {
                fd = (*(*fdsel).data.ranges.offset(0)).fd
            } else {
                let mut i: u16 = 0;
                i = 1i32 as u16;
                while (i as i32) < (*fdsel).num_entries as i32 {
                    if (gid as i32) < (*(*fdsel).data.ranges.offset(i as isize)).first as i32 {
                        break;
                    }
                    i = i.wrapping_add(1)
                }
                fd = (*(*fdsel).data.ranges.offset((i as i32 - 1i32) as isize)).fd
            }
        }
        _ => {
            panic!("in cff_fdselect_lookup(): Invalid FDSelect format");
        }
    }
    if fd as i32 >= cff.num_fds as i32 {
        panic!("in cff_fdselect_lookup(): Invalid Font DICT index");
    }
    fd
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_subrs(cff: &mut cff_font) -> i32 {
    let mut len: i32 = 0i32;
    let mut offset: i32 = 0;
    let mut i: i32 = 0;
    if cff.flag & 1i32 << 0i32 != 0 && cff.fdarray.is_null() {
        cff_read_fdarray(cff);
    }
    if cff.private.is_null() {
        cff_read_private(cff);
    }
    if cff.gsubr.is_null() {
        ttstub_input_seek(
            cff.handle,
            cff.offset.wrapping_add(cff.gsubr_offset) as ssize_t,
            0i32,
        );
        cff.gsubr = cff_get_index(cff)
    }
    cff.subrs = new((cff.num_fds as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*mut cff_index>() as u64) as u32)
        as *mut *mut cff_index;
    if cff.flag & 1i32 << 0i32 != 0 {
        i = 0i32;
        while i < cff.num_fds as i32 {
            if (*cff.private.offset(i as isize)).is_null()
                || cff_dict_known(
                    *cff.private.offset(i as isize),
                    b"Subrs\x00" as *const u8 as *const i8,
                ) == 0
            {
                let ref mut fresh47 = *cff.subrs.offset(i as isize);
                *fresh47 = 0 as *mut cff_index
            } else {
                offset = cff_dict_get(
                    *cff.fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                    1i32,
                ) as i32;
                offset = (offset as f64
                    + cff_dict_get(
                        *cff.private.offset(i as isize),
                        b"Subrs\x00" as *const u8 as *const i8,
                        0i32,
                    )) as i32;
                ttstub_input_seek(
                    cff.handle,
                    cff.offset.wrapping_add(offset as u32) as ssize_t,
                    0i32,
                );
                let ref mut fresh48 = *cff.subrs.offset(i as isize);
                *fresh48 = cff_get_index(cff);
                len += cff_index_size(*cff.subrs.offset(i as isize)) as i32
            }
            i += 1
        }
    } else if (*cff.private.offset(0)).is_null()
        || cff_dict_known(
            *cff.private.offset(0),
            b"Subrs\x00" as *const u8 as *const i8,
        ) == 0
    {
        let ref mut fresh49 = *cff.subrs.offset(0);
        *fresh49 = 0 as *mut cff_index
    } else {
        offset = cff_dict_get(cff.topdict, b"Private\x00" as *const u8 as *const i8, 1i32) as i32;
        offset = (offset as f64
            + cff_dict_get(
                *cff.private.offset(0),
                b"Subrs\x00" as *const u8 as *const i8,
                0i32,
            )) as i32;
        ttstub_input_seek(
            cff.handle,
            cff.offset.wrapping_add(offset as u32) as ssize_t,
            0i32,
        );
        let ref mut fresh50 = *cff.subrs.offset(0);
        *fresh50 = cff_get_index(cff);
        len += cff_index_size(*cff.subrs.offset(0)) as i32
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn cff_read_fdarray(cff: &mut cff_font) -> i32 {
    let mut len: i32 = 0i32;
    let mut idx: *mut cff_index = 0 as *mut cff_index;
    let mut offset: i32 = 0;
    let mut size: i32 = 0;
    let mut i: u16 = 0;
    if cff.topdict.is_null() {
        panic!("in cff_read_fdarray(): Top DICT not found");
    }
    if cff.flag & 1i32 << 0i32 == 0 {
        return 0i32;
    }
    /* must exist */
    offset = cff_dict_get(cff.topdict, b"FDArray\x00" as *const u8 as *const i8, 0i32) as i32;
    ttstub_input_seek(
        cff.handle,
        cff.offset.wrapping_add(offset as u32) as ssize_t,
        0i32,
    );
    idx = cff_get_index(cff);
    cff.num_fds = (*idx).count as u8;
    cff.fdarray = new(((*idx).count as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64) as u32)
        as *mut *mut cff_dict;
    i = 0i32 as u16;
    while (i as i32) < (*idx).count as i32 {
        let mut data: *mut u8 = (*idx)
            .data
            .offset(*(*idx).offset.offset(i as isize) as isize)
            .offset(-1);
        size = (*(*idx).offset.offset((i as i32 + 1i32) as isize))
            .wrapping_sub(*(*idx).offset.offset(i as isize)) as i32;
        if size > 0i32 {
            let ref mut fresh51 = *cff.fdarray.offset(i as isize);
            *fresh51 = cff_dict_unpack(data, data.offset(size as isize))
        } else {
            let ref mut fresh52 = *cff.fdarray.offset(i as isize);
            *fresh52 = 0 as *mut cff_dict
        }
        i = i.wrapping_add(1)
    }
    len = cff_index_size(idx) as i32;
    cff_release_index(idx);
    len
}
/* Flag */
/* FontName */
/* - CFF structure - */
/* CFF Header */
/* Name INDEX */
/* Top DICT (single) */
/* String INDEX */
/* Global Subr INDEX */
/* Encodings */
/* Charsets  */
/* FDSelect, CIDFont only */
/* CharStrings */
/* CIDFont only */
/* per-Font DICT */
/* Local Subr INDEX, per-Private DICT */
/* -- extra data -- */
/* non-zero for OpenType or PostScript wrapped */
/* number of glyphs (CharString INDEX count) */
/* number of Font DICT */
/* Updated String INDEX.
 * Please fix this. We should separate input and output.
 */
/* not used, ASCII Hex filter if needed */
/* CFF fontset index */
/* Flag: see above */
/* 1 if .notdef is not the 1st glyph */
/* CFF Header */
/* CFF INDEX */
/* Name INDEX */
/* Global and Local Subrs INDEX */
/* Encoding */
/* Charsets */
/* Returns GID of PS name "glyph" */
/* Return PS name of "gid" */
/* Returns GID of glyph with SID/CID "cid" */
/* Returns SID or CID */
/* FDSelect */
/* Font DICT(s) */
/* Private DICT(s) */
#[no_mangle]
pub unsafe extern "C" fn cff_read_private(cff: &mut cff_font) -> i32 {
    let mut len: i32 = 0i32;
    let mut data: *mut u8 = 0 as *mut u8;
    let mut offset: i32 = 0;
    let mut size: i32 = 0;
    if cff.flag & 1i32 << 0i32 != 0 {
        let mut i: i32 = 0;
        if cff.fdarray.is_null() {
            cff_read_fdarray(cff);
        }
        cff.private = new((cff.num_fds as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64)
            as u32) as *mut *mut cff_dict;
        i = 0i32;
        while i < cff.num_fds as i32 {
            if !(*cff.fdarray.offset(i as isize)).is_null()
                && cff_dict_known(
                    *cff.fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                ) != 0
                && {
                    size = cff_dict_get(
                        *cff.fdarray.offset(i as isize),
                        b"Private\x00" as *const u8 as *const i8,
                        0i32,
                    ) as i32;
                    size > 0i32
                }
            {
                offset = cff_dict_get(
                    *cff.fdarray.offset(i as isize),
                    b"Private\x00" as *const u8 as *const i8,
                    1i32,
                ) as i32;
                ttstub_input_seek(
                    cff.handle,
                    cff.offset.wrapping_add(offset as u32) as ssize_t,
                    0i32,
                );
                data = new(
                    (size as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32,
                ) as *mut u8;
                if ttstub_input_read(cff.handle, data as *mut i8, size as size_t) != size as i64 {
                    panic!("reading file failed");
                }
                let ref mut fresh53 = *cff.private.offset(i as isize);
                *fresh53 = cff_dict_unpack(data, data.offset(size as isize));
                free(data as *mut libc::c_void);
                len += size
            } else {
                let ref mut fresh54 = *cff.private.offset(i as isize);
                *fresh54 = 0 as *mut cff_dict
            }
            i += 1
        }
    } else {
        cff.num_fds = 1i32 as u8;
        cff.private =
            new((1_u64).wrapping_mul(::std::mem::size_of::<*mut cff_dict>() as u64) as u32)
                as *mut *mut cff_dict;
        if cff_dict_known(cff.topdict, b"Private\x00" as *const u8 as *const i8) != 0 && {
            size = cff_dict_get(cff.topdict, b"Private\x00" as *const u8 as *const i8, 0i32) as i32;
            size > 0i32
        } {
            offset =
                cff_dict_get(cff.topdict, b"Private\x00" as *const u8 as *const i8, 1i32) as i32;
            ttstub_input_seek(
                cff.handle,
                cff.offset.wrapping_add(offset as u32) as ssize_t,
                0i32,
            );
            data = new((size as u32 as u64).wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
                as *mut u8;
            if ttstub_input_read(cff.handle, data as *mut i8, size as size_t) != size as i64 {
                panic!("reading file failed");
            }
            let ref mut fresh55 = *cff.private.offset(0);
            *fresh55 = cff_dict_unpack(data, data.offset(size as isize));
            free(data as *mut libc::c_void);
            len += size
        } else {
            let ref mut fresh56 = *cff.private.offset(0);
            *fresh56 = 0 as *mut cff_dict;
            len = 0i32
        }
    }
    len
}
