# rc: micro bump

- Attempt to fix crate builds on docs.rs — see [#788]. This works around an
  issue in Tectonic’s usage of [cbindgen] by configuring Cargo to operate in
  offline mode when building on docs.rs, which builds crates with network access
  turned off.

[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788
[cbindgen]: https://github.com/eqrion/cbindgen


# tectonic_engine_xetex 0.1.1 (2021-06-15)

- Fix SyncTeX output (@hulloanson, @pkgw, #720, #744). We needed to include
  absolute paths and properly deal with file renames, etc. The only way to
  really do this right is to have the I/O backend provide filesystem paths when
  it has them, so we've extended the lower-level crates to make this possible.
- Fix the implementation of some special XeTeX commands, reported by @burrbull
  (@pkgw, #714, #783). This requires a bump in the format file serial number. We
  believe that this fix includes a fix to an upstream XeTeX bug, which has been
  reported.


# tectonic_engine_xetex 0.1.0 (2021-06-03)

This crate introduces the XeTeX engine as a standalone crate, building on the
new "core bridge" functionality.

Compared to the implementation previously provided in the main `tectonic` crate,
it also adds shell-escape functionality and iterates the Rust API somewhat.
