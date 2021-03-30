// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! No Rust API, for now. This crate exports a *C* API to code that performs I/O
//! on PDF files, XDV files, and other image formats.

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_core;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_flate;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
