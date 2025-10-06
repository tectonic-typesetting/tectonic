// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! Character category codes.

use std::fmt;
use tectonic_errors::prelude::*;

/// A character category code.
///
/// You can cast category codes as integers with a simple `c as i32` (e.g.).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CatCode {
    Escape = 0,
    BeginGroup = 1,
    EndGroup = 2,
    MathShift = 3,
    Tab = 4,
    CarriageReturn = 5,
    MacroParam = 6,
    Superscript = 7,
    Subscript = 8,
    Ignored = 9,
    Space = 10,
    Letter = 11,
    Other = 12,
    Active = 13,
    Comment = 14,
    Invalid = 15,
}

impl CatCode {
    pub fn abbrev(&self) -> &'static str {
        match self {
            CatCode::Escape => "esc",
            CatCode::BeginGroup => "bgr",
            CatCode::EndGroup => "egr",
            CatCode::MathShift => "mth",
            CatCode::Tab => "tab",
            CatCode::CarriageReturn => "car",
            CatCode::MacroParam => "mac",
            CatCode::Superscript => "sup",
            CatCode::Subscript => "sub",
            CatCode::Ignored => "ign",
            CatCode::Space => "spc",
            CatCode::Letter => "let",
            CatCode::Other => "oth",
            CatCode::Active => "act",
            CatCode::Comment => "com",
            CatCode::Invalid => "inv",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CatCode::Escape => "control-sequence escape",
            CatCode::BeginGroup => "begin group",
            CatCode::EndGroup => "end group",
            CatCode::MathShift => "math shift",
            CatCode::Tab => "tab separator",
            CatCode::CarriageReturn => "carriage return",
            CatCode::MacroParam => "macro parameter",
            CatCode::Superscript => "superscript",
            CatCode::Subscript => "subscript",
            CatCode::Ignored => "ignored",
            CatCode::Space => "space",
            CatCode::Letter => "letter",
            CatCode::Other => "other",
            CatCode::Active => "active character",
            CatCode::Comment => "comment",
            CatCode::Invalid => "invalid",
        }
    }

    pub fn from_i32(n: i32) -> Result<Self> {
        match n {
            0 => Ok(CatCode::Escape),
            1 => Ok(CatCode::BeginGroup),
            2 => Ok(CatCode::EndGroup),
            3 => Ok(CatCode::MathShift),
            4 => Ok(CatCode::Tab),
            5 => Ok(CatCode::CarriageReturn),
            6 => Ok(CatCode::MacroParam),
            7 => Ok(CatCode::Superscript),
            8 => Ok(CatCode::Subscript),
            9 => Ok(CatCode::Ignored),
            10 => Ok(CatCode::Space),
            11 => Ok(CatCode::Letter),
            12 => Ok(CatCode::Other),
            13 => Ok(CatCode::Active),
            14 => Ok(CatCode::Comment),
            15 => Ok(CatCode::Invalid),
            _ => {
                bail!(
                    "category codes must be between 0 and 15 (inclusive); got {}",
                    n
                );
            }
        }
    }
}

impl fmt::Display for CatCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.abbrev())
    }
}
