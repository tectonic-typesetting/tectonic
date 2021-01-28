// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! No Rust API, for now. This crate exports a *C* API to C++ font loading and
//! layout code. Ideally, it will migrate to become a cbindgen C API to a Rust
//! implementation.

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
