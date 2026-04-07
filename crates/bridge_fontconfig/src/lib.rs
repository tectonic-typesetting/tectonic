//! This crate exists to both export the FreeType2 *C* API into the
//! Cargo framework, and provide a safe wrapper around it for use in Rust code.

#![deny(clippy::undocumented_unsafe_blocks)]

use std::convert::TryFrom;
use std::ffi::CStr;

mod font_set;
mod object_set;
pub mod pat;
pub mod sys;

pub use font_set::{FontSet, FontSetRef};
pub use object_set::{ObjectSet, ObjectSetRef};
pub use pat::{Pattern, PatternRef};

/// Initialize fontconfig. Should be called automatically by any API that requires it.
pub fn init() -> bool {
    // SAFETY: This is always safe to call
    (unsafe { sys::FcInit() }) == sys::FcTrue
}

/// Error returned by a fallible operation
#[derive(Debug, PartialEq)]
pub enum FcErr {
    /// No matching found for pattern or search
    NoMatch,
    /// Invalid type used - for example, attempted to access a string property as an integer
    TypeMismatch,
    /// No ID exists for the request
    ResultNoId,
    /// Allocation failed due to out-of-memory error
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

/// Properties a font can have
#[derive(Copy, Clone)]
pub enum Property {
    /// Font family
    Family,
    /// Font style
    Style,
    /// Font slant
    Slant,
    /// Font weight
    Weight,
    /// Font width
    Width,
    /// Font file
    File,
    /// Font index
    Index,
    /// Font full name
    FullName,
    /// Font format
    FontFormat,
}

impl Property {
    fn to_raw(self) -> &'static CStr {
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

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_png as clippyrenamehack1;
}
