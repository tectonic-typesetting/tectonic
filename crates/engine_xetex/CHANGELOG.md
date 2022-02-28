# rc: minor bump

- Use the new `tectonic_xetex_format` crate as part of the build process (#851,
  #848, @pkgw). This crate defines all of the metadata about the XeTeX engine
  internals, with versioning, and generates the necessary header files and
  macros. It also contains code for decoding XeTeX/Tectonic format files, so
  that we'll be able to introspect engine data structures such as macro
  definitions.
- Plumb in some specials that will be used by the prototype HTML output
  mode (#865, @pkgw)
- Tidy up some of the auto-generated C code
- Fix an internal transcription error: `pre_display_direction`, not
  `pre_display_correction`
- Fix a long-standing test issue with PNG image dimensions occasionally leading
  to not-quite-reproducible output (#847, @pkgw)


# tectonic_engine_xetex 0.1.4 (2021-07-04)

- Avoid misplaced newlines in warning output ([#803], [@ralismark])
- Fix new warnings reported by Clippy 1.53.0

[#803]: https://github.com/tectonic-typesetting/tectonic/pull/803
[@ralismark]: https://github.com/ralismark


# tectonic_engine_xetex 0.1.3 (2021-06-17)

- Switch from running [cbindgen] at build time to having the developer run it
  manually. This really ought to fix the crate builds on docs.rs ([#788]), and
  should speed builds too.

[cbindgen]: https://github.com/eqrion/cbindgen
[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788


# tectonic_engine_xetex 0.1.2 (2021-06-17)

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
