# The `tectonic_xetex_layout` crate

[![](http://meritbadge.herokuapp.com/tectonic_xetex_layout)](https://crates.io/crates/tectonic_xetex_layout)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides the font
loading and layout routines of [XeTeX] as a crate, *currently providing only a C
API*.

[XeTeX]: http://xetex.sourceforge.net/

- [API documentation](https://docs.rs/tectonic_xetex_layout/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_TECTONIC_XETEX_LAYOUT_INCLUDE_PATH`, which
will be a semicolon-separated list of directories enabling your code to include
the `tectonic_xetex_layout.h` header.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_xetex_layout;
```


## Cargo features

This crate provides the following [Cargo features][features]:

[features]: https://doc.rust-lang.org/cargo/reference/features.html

- **`external-harfbuzz`**: activates the same-named feature in
  the `tectonic_bridge_harfbuzz` dependency.
