#![deny(clippy::undocumented_unsafe_blocks)]

use std::convert::TryFrom;

macro_rules! c {
    ($lit:literal) => {
        ::std::ptr::from_ref(concat!($lit, "\0")).cast::<::libc::c_char>()
    };
}

mod font_set;
mod object_set;
pub mod pat;
pub mod sys;

pub use font_set::{FontSet, FontSetRef};
pub use object_set::{ObjectSet, ObjectSetRef};
pub use pat::{Pattern, PatternRef};

pub fn init() -> bool {
    // SAFETY: This is always safe to call
    (unsafe { sys::FcInit() }) == sys::FcTrue
}

#[derive(Debug, PartialEq)]
pub enum FcErr {
    NoMatch,
    TypeMismatch,
    ResultNoId,
    OutOfMemory,
}

impl TryFrom<sys::FcResult> for FcErr {
    type Error = ();

    fn try_from(value: sys::FcResult) -> Result<Self, Self::Error> {
        use sys::FcResult;
        Ok(match value {
            FcResult::Match => return Err(()),
            FcResult::NoMatch => FcErr::NoMatch,
            FcResult::TypeMismatch => FcErr::TypeMismatch,
            FcResult::ResultNoId => FcErr::ResultNoId,
            FcResult::OutOfMemory => FcErr::OutOfMemory,
        })
    }
}

#[derive(Copy, Clone)]
pub enum Property {
    Family,
    Style,
    Slant,
    Weight,
    Width,
    File,
    Index,
    FullName,
    FontFormat,
}

impl Property {
    fn to_raw(self) -> *const libc::c_char {
        match self {
            Property::Family => sys::FC_FAMILY,
            Property::Style => sys::FC_STYLE,
            Property::Slant => sys::FC_SLANT,
            Property::Weight => sys::FC_WEIGHT,
            Property::Width => sys::FC_WIDTH,
            Property::File => sys::FC_FILE,
            Property::Index => sys::FC_INDEX,
            Property::FullName => sys::FC_FULLNAME,
            Property::FontFormat => sys::FC_FONTFORMAT,
        }
    }
}
