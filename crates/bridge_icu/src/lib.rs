// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! No Rust code. This crate exists to export the ICU *C* API into the
//! Cargo framework.

pub const UBIDI_DEFAULT_LTR: u8 = 0xFE;
pub const UBIDI_DEFAULT_RTL: u8 = 0xFF;

pub type UChar32 = i32;
