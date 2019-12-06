# The `tectonic_cfg_support` create

This create is a support create for `build.rs` files that helps deal with
`CARGO_CFG_TARGET_*` variables. When cross-compiling, these variables must be
used instead of constructs such as `cfg!(target_arch = ...)` because the
`build.rs` script is compiled to target the build host environment, not the
true target environment. This crate is part of
[the Tectonic project](https://tectonic-typesetting.github.io/).

[![](http://meritbadge.herokuapp.com/tectonic_cfg_support)](https://crates.io/crates/tectonic_cfg_support)

- [API documentation](https://docs.rs/tectonic_cfg_support/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).
