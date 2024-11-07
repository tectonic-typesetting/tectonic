# The `tectonic_bridge_harfbuzz` crate

[![](http://meritbadge.herokuapp.com/tectonic_bridge_harfbuzz)](https://crates.io/crates/tectonic_bridge_harfbuzz)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It exposes the *C* API
of the [Harfbuzz] text shaping library the Rust/Cargo build framework, **with no
Rust bindings**. By default, a static, local version of the Harfbuzz library is
built.

[Harfbuzz]: https://harfbuzz.github.io/

- [API documentation](https://docs.rs/tectonic_bridge_harfbuzz/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

While there are a variety of other Harfbuzz-related crates in the Rust
ecosystem, Tectonic has specialized needs (e.g. support for the [graphite2]
smart font library). Hence this specialized crate.

[graphite2]: https://graphite.sil.org/

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_HARFBUZZ_INCLUDE_PATH`, which will be a
semicolon-separated list of C include directories enabling your code to include
the `harfbuzz/` headers.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_bridge_harfbuzz;
```


## Cargo features

This crate provides the following [Cargo features][features]:

[features]: https://doc.rust-lang.org/cargo/reference/features.html

- **`external-harfbuzz`**: instead of building a local copy of Harfbuzz,
  discover it as an external dependency using pkg-config or vcpkg, as determined
  by the `tectonic_dep_support` crate. This requires that all libraries upon
  which Harfbuzz depends also be found externally, since the external library can't
  then link against a local vendored version.

