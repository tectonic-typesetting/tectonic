# The `tectonic_engine_bibtex` crate

[![](http://meritbadge.herokuapp.com/tectonic_engine_bibtex)](https://crates.io/crates/tectonic_engine_bibtex)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides the
[bibtex] program as a reusable crate.

[bibtex]: http://www.bibtex.org/

- [API documentation](https://docs.rs/tectonic_engine_bibtex/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).


## Cargo features

This crate does not currently provides any [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html


## Updating the generated header

This crate exposes Rust functions to C/C++ code using a header file created by
[cbindgen]. To update the header, run:

[cbindgen]: https://github.com/eqrion/cbindgen/

```sh
cbindgen --output bibtex/bibtex_bindings.h
```
