# The `tectonic_engine_xetex` crate

[![](http://meritbadge.herokuapp.com/tectonic_engine_xetex)](https://crates.io/crates/tectonic_engine_xetex)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides the
[XeTeX] engine as a reusable crate.

[XeTeX]: http://www.xetex.org/

- [API documentation](https://docs.rs/tectonic_engine_xetex/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).


## Cargo features

This crate provides the following [Cargo features][features]:

[features]: https://doc.rust-lang.org/cargo/reference/features.html

- **`external-harfbuzz`**: activates the same-named feature in
  the `tectonic_bridge_harfbuzz` dependency.


## Updating the generated header

This crate exposes Rust functions to C/C++ code using a header file created by
[cbindgen]. To update the header, run:

[cbindgen]: https://github.com/eqrion/cbindgen/

```sh
cbindgen --output xetex/xetex_bindings.h
```
