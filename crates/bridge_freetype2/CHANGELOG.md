# rc: micro bump


# tectonic_bridge_freetype2 0.3.1 (2026-04-11)

- Fix for publish not going through


# tectonic_bridge_freetype2 0.3.0 (2026-04-11)

- Global lint fixes
- Update to edition 2021
- Expose the C API in Rust for use in `tectonic_xetex_layout` Rust rewrite


# tectonic_bridge_freetype2 0.2.0 (2021-06-03)

Fix up handling of how C/C++ header file paths are exported to dependent crates.
This is a breaking change: we've moved from a single include directory to a list
of them.


# tectonic_bridge_freetype2 0.1.0 (2021-01-15)

Initial release of FreeType "bridge" crate for Tectonic.
