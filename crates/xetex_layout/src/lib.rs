// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate provides font loading and layout code, as well as C bindings to it.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

pub mod engine;
pub mod font;
pub mod manager;
mod utils;

mod c_api;

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
