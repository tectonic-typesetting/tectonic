# The `tectonic_geturl` create

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides an
interface for fetching URLs using one of several HTTP backends.

[![](http://meritbadge.herokuapp.com/tectonic_geturl)](https://crates.io/crates/tectonic_geturl)

- [API documentation](https://docs.rs/tectonic_geturl/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).


## Cargo features

This crate provides the following [Cargo features][features]:

[features]: https://doc.rust-lang.org/cargo/reference/features.html

- **`reqwest`** (enabled by default): use the [reqwest] crate as the backend for
  performing URL gets.

[reqwest]: https://docs.rs/reqwest/

If no backend is enabled, a “null” backend will be used that will always return errors.
