// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! Dealing with the TeX string table.

use nom::{
    multi::count,
    number::complete::{be_i32, be_u16},
    Err as NomErr, IResult, Parser,
};

use crate::parseutils;

pub type StrPointer = i32;

#[derive(Debug)]
pub struct StringTable {
    pub strings: Vec<String>,
}

pub(crate) fn len_utf16(s: &str) -> usize {
    let mut len = s.len();

    for c in s.chars() {
        if c as u32 > 0xFFFF {
            len += 1;
        }
    }

    len
}

impl StringTable {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn all_sps(&self) -> impl Iterator<Item = StrPointer> {
        0x10000..0x10000 + self.strings.len() as i32
    }

    pub fn lookup(&self, sp: StrPointer) -> &str {
        assert!(sp > 0xFFFF);
        &self.strings[sp as usize - 0x10000]
    }

    pub fn utf16_length(&self, sp: StrPointer) -> usize {
        if sp > 0xFFFF {
            len_utf16(self.lookup(sp))
        } else if (32..127).contains(&sp) {
            1
        } else if sp <= 127 {
            3
        } else if sp < 256 {
            4
        } else {
            8
        }
    }

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], StringTable> {
        const TOO_BIG_CHAR: i32 = 0x0001_0000;

        let (input, pool_ptr) = be_i32(input)?;
        let (input, str_ptr) = be_i32(input)?;
        let n_strings = str_ptr - TOO_BIG_CHAR + 1;

        let (input, str_starts) =
            count(parseutils::ranged_be_i32(0, pool_ptr), n_strings as usize).parse(input)?;

        let (input, str_pool) = count(be_u16, pool_ptr as usize).parse(input)?;
        let mut strings = Vec::new();

        for i in 0..(n_strings as usize) {
            let idx0 = str_starts[i] as usize;
            let sl = if i == n_strings as usize - 1 {
                &str_pool[idx0..]
            } else {
                &str_pool[idx0..str_starts[i + 1] as usize]
            };
            let s = String::from_utf16(sl).map_err(|_| {
                use nom::error::ParseError;
                NomErr::Error(nom::error::Error::from_error_kind(
                    input,
                    nom::error::ErrorKind::Satisfy,
                ))
            })?;
            strings.push(s);
        }

        Ok((input, StringTable { strings }))
    }
}
