# tectonic_bridge_flate 0.1.2 (2021-06-16)

- Try some workarounds to get docs building on docs.rs, both for this crate on
  its own and for the toplevel `tectonic` crate.


# tectonic_bridge_flate 0.1.1 (2021-01-16)

- Fix a Clippy complaint


# tectonic_bridge_flate 0.1.0 (2021-01-03)

Initial release of the `tectonic_bridge_flate` crate. This crate provides a
simple C API to the flate2 crate — even though flate2 often wraps zlib, which
has its own C API. This is the first step towards segmenting Tectonic's
native-library dependencies and starting to be able to vendor them. This new
crate doesn't change anything dramatic yet, but starts that process.
