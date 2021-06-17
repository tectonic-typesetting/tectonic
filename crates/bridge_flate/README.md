# The `tectonic_bridge_flate` crate

[![](http://meritbadge.herokuapp.com/tectonic_bridge_flate)](https://crates.io/crates/tectonic_bridge_flate)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides a limited C
API to the [flate2] crate. This API is consumed by other Tectonic crates that
include C code that require flate2 functionality.

[flate2]: https://github.com/rust-lang/flate2-rs

- [API documentation](https://docs.rs/tectonic_bridge_flate/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_TECTONIC_BRIDGE_FLATE_INCLUDE`, which will be
the name of a directory containing the generated `tectonic_bridge_flate.h` file
that exposes this crate’s C API.

You will need to ensure that your Rust code actually references this crate in order
for the linker to include the C API symbols. A `use` statement will suffice:

```rust
#[allow(unused_imports)]
use tectonic_bridge_flate::flate2;
```


## Cargo features

The [flate2] crate provides several [Cargo features][features] allowing control
over the zlib implementation that it ultimately uses. This crate intentionally
does *not* expose these features, because it is used by other crates that link
with system libraries that can be expected to link to the system’s libz.
Therefore, the only safe approach at this time is to force the use of the system
libz here as well. Once the linking framework is built up, it will become
possible to allow for more flexibility in this area.

[features]: https://doc.rust-lang.org/cargo/reference/features.html


## Updating the generated header

This crate exposes Rust functions to C/C++ code using a header file created by
[cbindgen]. To update the header, run:

[cbindgen]: https://github.com/eqrion/cbindgen/

```sh
cbindgen --output include/tectonic_bridge_flate.h
```
