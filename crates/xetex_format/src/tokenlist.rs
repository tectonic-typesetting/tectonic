// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! Handling TeX token lists.
//!
//! Token lists are linked lists of compressed entries that can express either
//! a character or a control sequence.

use crate::{commands::CommandCode, eqtb::EqtbPointer};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Char { cmd: CommandCode, chr: i32 },
    ControlSeq { ptr: EqtbPointer },
}

const CS_TOKEN_FLAG: i32 = 0x1FF_FFFF;

impl From<i32> for Token {
    fn from(code: i32) -> Token {
        if code > CS_TOKEN_FLAG {
            Token::ControlSeq {
                ptr: code - CS_TOKEN_FLAG,
            }
        } else {
            let cmd = ((code & 0x1E0_0000) >> 21) as CommandCode;
            let chr = code & 0x1F_FFFF;
            Token::Char { cmd, chr }
        }
    }
}
