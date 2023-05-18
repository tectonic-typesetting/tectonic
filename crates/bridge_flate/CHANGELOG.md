# tectonic_bridge_flate 0.1.7 (2023-05-18)

- Tidy up recent Clippy warnings.


# tectonic_bridge_flate 0.1.6 (2022-10-03)

No code changes, but some internal documentation improvements about managing FFI
APIs.


# tectonic_bridge_flate 0.1.5 (2021-06-17)

- Switch from running [cbindgen] at build time to having the developer run it
  manually. This really ought to fix the crate builds on docs.rs ([#788]), and
  should speed builds too.

[cbindgen]: https://github.com/eqrion/cbindgen
[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788


# tectonic_bridge_flate 0.1.4 (2021-06-17)

- Attempt to fix crate builds on docs.rs — see [#788]. This works around an
  issue in Tectonic’s usage of [cbindgen] by configuring Cargo to operate in
  offline mode when building on docs.rs, which builds crates with network access
  turned off.

[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788
[cbindgen]: https://github.com/eqrion/cbindgen


# tectonic_bridge_flate 0.1.3 (2021-06-16)

- Try again with our docs.rs workarounds. Looks like we need
  `CARGO_NET_OFFLINE=true`, not `CARGO_NET_OFFLINE=1`.


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
