# rc: minor bump

- Update the vendored Harfbuzz to 2.8.1.
- Fix up handling of how C/C++ header file paths are exported to dependent
  crates. This is a breaking change: we've moved from a single include directory
  to a list of them.

# tectonic_bridge_harfbuzz 0.1.0 (2021-01-15)

Initial release of Harfbuzz "bridge" crate for Tectonic. Includes the ability to
vendor Harfbuzz.
