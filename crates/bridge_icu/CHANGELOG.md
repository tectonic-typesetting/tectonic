# tectonic_bridge_icu 0.2.1 (2024-02-05)

- Adjust the build script to try to get proper link ordering with libicudata
  (#1092, @pkgw).


# tectonic_bridge_icu 0.2.0 (2021-06-03)

Fix up handling of how C/C++ header file paths are exported to dependent crates.
This is a breaking change: we've moved from a single include directory to a list
of them.


# tectonic_bridge_icu 0.1.0 (2021-01-15)

Initial release of ICU "bridge" crate for Tectonic.
