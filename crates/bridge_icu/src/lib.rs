// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

#![allow(nonstandard_style)]

//! This crate exists to export the ICU *C* API into the Cargo framework, as well as
//! provide bindings to other tectonic crates.

mod sys;

mod converter;

pub use sys::{UChar32, UBIDI_DEFAULT_LTR, UBIDI_DEFAULT_RTL};

pub use converter::Converter;

#[derive(PartialEq, Debug)]
pub struct IcuErr(sys::UErrorCode);

impl IcuErr {
    fn from_raw(err: sys::UErrorCode) -> IcuErr {
        IcuErr(err)
    }
}
