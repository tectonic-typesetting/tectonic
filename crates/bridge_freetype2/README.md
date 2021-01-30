# The `tectonic_bridge_freetype2` crate

[![](http://meritbadge.herokuapp.com/tectonic_bridge_freetype2)](https://crates.io/crates/tectonic_bridge_freetype2)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It exposes the *C* API
of the [FreeType] font rendering engine within the Rust/Cargo build framework,
**with no Rust bindings**.

[FreeType]: https://www.freetype.org/

- [API documentation](https://docs.rs/tectonic_bridge_freetype2/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

There are a variety of other low-level FreeType-related crates available, including:

- [freetype-sys](https://crates.io/crates/freetype-sys)
- [embedded-freetype-sys](https://crates.io/crates/embedded-freetype-sys)
- [serve-freetype-sys](https://crates.io/crates/servo-freetype-sys)

This package is distinctive because:

- It uses Tectonic’s dependency-finding framework, which supports both
  [pkg-config] and [vcpkg].
- It ensures that FreeType’s C API is exposed to Cargo.

[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/
[vcpkg]: https://vcpkg.readthedocs.io/

Ideally, one day this crate will be superseded by one of the above crates.

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_FREETYPE2_INCLUDE_PATH`, which will be a
semicolon-separated list of directories containing C headers, such that your
code will be able to successfully include the `ft2build.h` header.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_bridge_freetype2;
```


## Cargo features

At the moment this crate does not provide any [Cargo features][features]. It is
intended that eventually it will, to allow control over whether the FreeType
library is vendored or not.

[features]: https://doc.rust-lang.org/cargo/reference/features.html
