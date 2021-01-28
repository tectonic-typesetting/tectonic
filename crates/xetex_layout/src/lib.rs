// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! No Rust API, for now. This crate exports a *C* API to C++ font loading and
//! layout code. Ideally, it will migrate to become a cbindgen C API to a Rust
//! implementation.

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_core;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_freetype2;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_graphite2;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_harfbuzz;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_icu;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
