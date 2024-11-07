# The `tectonic_dep_support` crate

[![](http://meritbadge.herokuapp.com/tectonic_dep_support)](https://crates.io/crates/tectonic_dep_support)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides build-time
utilities for finding external library dependencies, allowing either
[pkg-config] or [vcpkg] to be used as the dep-finding backend, and providing
whatever fiddly features are needed to enable the Tectonic build process.

- [API documentation](https://docs.rs/tectonic_dep_support/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/
[vcpkg]: https://vcpkg.readthedocs.io/
