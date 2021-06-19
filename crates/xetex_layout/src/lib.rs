// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate contains no Rust code. It exists to export a *C* API to C++ font
//! loading and layout code in the Cargo build framework used by [Tectonic].
//! Ideally, it will migrate to become a cbindgen C API to a Rust
//! implementation.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_core as clipyrenamehack1;

    #[allow(unused_imports)]
    use tectonic_bridge_freetype2 as clipyrenamehack2;

    #[allow(unused_imports)]
    use tectonic_bridge_graphite2 as clipyrenamehack3;

    #[allow(unused_imports)]
    use tectonic_bridge_harfbuzz as clipyrenamehack4;

    #[allow(unused_imports)]
    use tectonic_bridge_icu as clipyrenamehack5;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
