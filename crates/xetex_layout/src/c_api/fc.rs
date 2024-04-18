use std::convert::TryFrom;

mod font_set;
mod object_set;
pub mod pat;
pub mod sys;

pub use font_set::{FontSet, OwnFontSet};
pub use object_set::{ObjectSet, OwnObjectSet};
pub use pat::{OwnPattern, Pattern};

pub fn init() -> bool {
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
