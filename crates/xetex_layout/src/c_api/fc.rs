use std::convert::TryFrom;

pub mod pat;
pub mod sys;

pub use pat::{OwnPattern, Pattern};

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
