# The `tectonic_bridge_core` crate

[![](http://meritbadge.herokuapp.com/tectonic_bridge_core)](https://crates.io/crates/tectonic_bridge_core)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides a C API to
core I/O services provided by the Tectonic Rust code. This API is then consumed
by the various C/C++ “engines” implementing the TeX software.

- [API documentation](https://docs.rs/tectonic_bridge_core/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_TECTONIC_BRIDGE_CORE_INCLUDE`, which will be
the name of a directory containing the generated `tectonic_bridge_core.h` file
that exposes this crate’s C API.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include the C API symbols. If you are not actively using
its Rust API, a `use` statement will suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_bridge_core;
```


## Cargo features

This crate does not currently provides any [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
