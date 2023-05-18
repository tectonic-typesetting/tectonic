# tectonic_engine_xdvipdfmx 0.4.1 (2023-05-18)

- Tidy up recent Clippy warnings.


# tectonic_engine_xdvipdfmx 0.4.0 (2022-10-27)

- Use new support in the `pdf_io` backend to handle the `dvipdfmx:config`
  special (#904, #953, @vlasakm). This should fix some aspects of PDF generation,
  including named anchors created by `hyperref`.


# tectonic_engine_xdvipdfmx 0.3.0 (2022-10-03)

- Synchronize with TeXLive 2022.0 (#936, @pkgw). There are no changes in this
  crate related to the update, but some in `tectonic_pdf_io`.
- Remove C's `time_t` from internal FFI APIs to avoid portability issues. This
  should avoid issues with Linux Musl builds.


# tectonic_engine_xdvipdfmx 0.2.0 (2022-04-26)

Update xdvipdfmx for TeXLive 2021 (#882, @pkgw). A brief summary of changes
based on the TeXLive 2021 release notes:

- Not applicable to Tectonic: GhostScript safety improvements.
- If an image ﬁle is not found, exit with bad status.
- Extended special syntax for color support.
- Specials for manipulating ExtGState.
- Compatibility specials `pdfcolorstack` and `pdffontattr`.
- Experimental support for dviluatex’s extended `fnt_def`.
- Support new feature of virtual font to fallback Japanese font deﬁnition.

There are other internal changes; see [the low-level ChangeLog][xdvcl]. Note
that most of these changes are actually found in the `tectonic_pdf_io` crate,
which contains most of the actual exported xdvipdfmx code.

[xdvcl]: https://github.com/TeX-Live/texlive-source/blob/404d2e476949c1e225e6b94ff92e3a113ab6b413/texk/dvipdfm-x/ChangeLog#L107-L557


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
