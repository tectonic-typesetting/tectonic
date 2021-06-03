// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate contains no Rust code, for now. It exists to export a *C* API for
//! I/O on PDF, XDV, and graphics files, within the Cargo build framework used
//! by [Tectonic]. Ideally, it will migrate to become a cbindgen C API to a Rust
//! implementation.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

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
