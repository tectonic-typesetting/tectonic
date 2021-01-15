// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! No Rust code. This crate exists to export the Harfbuzz *C/C++* API into the
//! Cargo framework.

/// Import something from our bridge crates so that we ensure that we actually
/// link with them, to pull in the symbols defined in the C APIs.
mod linkage {
    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_graphite2;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_bridge_icu;
}
