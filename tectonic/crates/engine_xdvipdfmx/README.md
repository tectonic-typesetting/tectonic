# The `tectonic_engine_xdvipdfmx` crate

[![](http://meritbadge.herokuapp.com/tectonic_engine_xdvipdfmx)](https://crates.io/crates/tectonic_engine_xdvipdfmx)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides
[XeTeX]’s `xdvipdfmx` program as a reusable crate.

[XeTeX]: http://xetex.sourceforge.net/

- [API documentation](https://docs.rs/tectonic_engine_xdvipdfmx/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).


## Cargo features

This crate does not currently provides any [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html


## Updating the generated header

This crate exposes Rust functions to C/C++ code using a header file created by
[cbindgen]. To update the header, run:

[cbindgen]: https://github.com/eqrion/cbindgen/

```sh
cbindgen --output xdvipdfmx/xdvipdfmx_bindings.h
```
