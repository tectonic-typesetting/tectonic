# The `tectonic_pdf_io` crate

[![](http://meritbadge.herokuapp.com/tectonic_pdf_io)](https://crates.io/crates/tectonic_pdf_io)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides the PDF,
XDV, and image I/O capabilities of [XeTeX]’s `xdvipdfmx` as a crate, *currently
providing only a C API*.

[XeTeX]: http://xetex.sourceforge.net/

- [API documentation](https://docs.rs/tectonic_pdf_io/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_TECTONIC_PDF_IO_INCLUDE_PATH`, which will be
a semicolon-separated list of directories enabling your code to include the
`dpx-*.h` headers.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_pdf_io;
```


## Cargo features

This crate currently provides no [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
