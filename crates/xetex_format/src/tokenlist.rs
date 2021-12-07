// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Handling TeX token lists.
//!
//! Token lists are linked lists of compressed entries that can express either
//! a character or a control sequence.

use crate::{
    base::TEX_NULL,
    commands::CommandCode,
    cshash::ControlSeqHash,
    eqtb::EqtbPointer,
    mem::{MemPointer, Memory},
    stringtable::StringTable,
};

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

pub fn print_toklist(
    mut p: MemPointer,
    mem: &Memory,
    cshash: &ControlSeqHash,
    strings: &StringTable,
    is_macro: bool,
) {
    if is_macro {
        // Skip the reference count
        p = mem.decode_toklist(p).1;
        println!("~~ macro template: ~~");
    }

    const CCDESCS: &[&str] = &[
        "ESC", "BGR", "EGR", "MTH", "TAB", "CAR", "MAC", "SUP", "SUB", "IGN", "SPC", "LET", "OTH",
        "ACT", "COM", "INV",
    ];

    while p != TEX_NULL {
        let (value, next) = mem.decode_toklist(p);
        let tok = Token::from(value);

        match tok {
            Token::Char { cmd, chr } => match (is_macro, cmd) {
                (true, 14) => {
                    println!("~~ macro expansion: ~~");
                }

                (true, 13) => {
                    println!("<macro parameter>");
                }

                (true, 5) => {
                    println!("#{}", chr);
                }

                _ => {
                    if let Some(c) = char::from_u32(chr as u32) {
                        println!("{} {{{}}}", c, CCDESCS[cmd as usize]);
                    } else {
                        println!(
                            "[illegal USV char code 0x{:08x}] {{{}}}",
                            chr, CCDESCS[cmd as usize]
                        );
                    }
                }
            },

            Token::ControlSeq { ptr } => {
                if let Some(text) = cshash.stringify(ptr, strings) {
                    if text == " " {
                        println!("\\  (slash-space)");
                    } else {
                        println!("\\{}", text);
                    }
                } else {
                    println!("\\[undecodable token cseq pointer {}]", ptr);
                }
            }
        }

        p = next;
    }
}
