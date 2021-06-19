# tectonic_engine_xdvipdfmx 0.1.3 (2021-06-19)

- Fix a `build.rs` typo causing nonstop rebuilds
- Fixes for Clippy 1.53.0


# tectonic_engine_xdvipdfmx 0.1.2 (2021-06-17)

- Switch from running [cbindgen] at build time to having the developer run it
  manually. This really ought to fix the crate builds on docs.rs ([#788]), and
  should speed builds too.

[cbindgen]: https://github.com/eqrion/cbindgen
[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788


# tectonic_engine_xdvipdfmx 0.1.1 (2021-06-17)

- Attempt to fix crate builds on docs.rs — see [#788]. This works around an
  issue in Tectonic’s usage of [cbindgen] by configuring Cargo to operate in
  offline mode when building on docs.rs, which builds crates with network access
  turned off.

[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788
[cbindgen]: https://github.com/eqrion/cbindgen


# tectonic_engine_xdvipdfmx 0.1.0 (2021-06-03)

This crate introduces the `xdvipdfmx` engine as a standalone crate, building on
the new "core bridge" functionality. The API is iterated somewhat from the one
that was previously provided in the main `tectonic` crate.

Note that the vast majority of the `xdvipdfmx` C/C++ code is found in the new
`tectonic_pdf_io` crate, because both this crate and the XeTeX engine need to
share library routines to do I/O on PDF files.
