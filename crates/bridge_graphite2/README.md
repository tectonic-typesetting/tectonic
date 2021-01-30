# The `tectonic_bridge_graphite2` crate

[![](http://meritbadge.herokuapp.com/tectonic_bridge_graphite2)](https://crates.io/crates/tectonic_bridge_graphite2)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It exposes the *C* API
of the [graphite2] “smart font” system within the Rust/Cargo build framework,
**with no Rust bindings**. This is why it is not named `graphite2-sys`.

[graphite2]: https://graphite.sil.org/

- [API documentation](https://docs.rs/tectonic_bridge_graphite2/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

The intention is that eventually this crate will provide the option of
“vendoring” the graphite2 library, so that the Tectonic C code can use the
library without it needing to be installed on the system. However, this has not
yet been implemented.

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_GRAPHITE2_INCLUDE_PATH`, which will be a
semicolon-separated list of C incude directories, such that your code can
include the `graphite2/*` C headers. It will also export an environment variable
named `DEP_GRAPHITE2_DEFINE_STATIC`. If non-empty, C/C++ code compiled against
this library should define the C preprocessor symbol `GRAPHITE2_STATIC`.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_bridge_graphite2;
```


## Cargo features

At the moment this crate does not provide any [Cargo features][features]. It is
intended that eventually it will, to allow control over whether the graphite2
library is vendored or not.

[features]: https://doc.rust-lang.org/cargo/reference/features.html
