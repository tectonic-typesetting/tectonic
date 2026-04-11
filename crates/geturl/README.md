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

- **`curl`**: use the [curl] crate as a backend for performing URL gets.
- **`reqwest`** (enabled by default): use the [reqwest] crate as a backend for
  performing URL gets.

[curl]: https://docs.rs/curl/
[reqwest]: https://docs.rs/reqwest/

There is always a "null" backend available, which will always return errors. If
more than one backend is enabled, their prioritization is:

- `reqwest` (most preferred)
- `curl`
- `null` (least preferred)
