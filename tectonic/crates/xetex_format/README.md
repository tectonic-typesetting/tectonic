# The `tectonic_xetex_format` crate

[![](http://meritbadge.herokuapp.com/tectonic_xetex_format)](https://crates.io/crates/tectonic_xetex_format)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides
introspection of the internal data structures of the Tectonic/[XeTeX] engine and
their serialization into "format files".

[XeTeX]: http://xetex.sourceforge.net/

- [API documentation](https://docs.rs/tectonic_xetex_format/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

This crate has two main uses: you can use it to decode an existing format file
and introspect the detailed setup that it encodes; or you can use it to emit a C
header file defining magic constants in the engine implementation. The former
usage isn't fully developed yet, but many of the key pieces have been
implemented.


## Cargo features

This crate currently provides no [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
