# rc: minor bump

- Update for TeXLive 2022.0 (#936, @pkgw)! Mostly very minor changes, with the
  most significant ones involving "base encodings" in the PDF encoding support.
- Fix some size_t/printf warnings on ARM


# tectonic_pdf_io 0.2.0 (2022-04-26)

Update xdvipdfmx for TeXLive 2021 (#882, @pkgw). A brief summary of changes based
on the TeXLive 2021 release notes:

- Not applicable to Tectonic: GhostScript safety improvements
- If an image ﬁle is not found, exit with bad status.
- Extended special syntax for color support.
- Specials for manipulating ExtGState.
- Compatibility specials `pdfcolorstack` and `pdffontattr`.
- Experimental support for dviluatex’s extended `fnt_def`.
- Support new feature of virtual font to fallback Japanese font deﬁnition.

There are other internal changes; see [the low-level ChangeLog][xdvcl]. See also
the `tectonic_xdvipdfmx` crate, which actually contains only a few pieces of
this update.

[xdvcl]: https://github.com/TeX-Live/texlive-source/blob/404d2e476949c1e225e6b94ff92e3a113ab6b413/texk/dvipdfm-x/ChangeLog#L107-L557

Additionally, some new memory leaks have been plugged (@pkgw).


# tectonic_pdf_io 0.1.2 (2021-10-11)

- Fix an incorrect warning issued when reading EXIF data (#822, @korrat)


# tectonic_pdf_io 0.1.1 (2021-07-03)

- Fix the build on Rust 1.46, needed for the conda-forge macOS build (which is
  stuck on this version of Rust for the time being)
- Fixes for complaints from Clippy 1.53.0


# tectonic_pdf_io 0.1.0 (2021-06-03)

This crate contains the vast majority of the C/C++ code from `xdvipdfmx`. It
provides I/O services to both the `tectonic_engine_xdvipdfmx` and
`tectonic_engine_xetex` crates through its C/C++ APIs. It does not provide a
Rust API.

This crate deals with general graphics I/O, not just PDF files, but the majority
of its code is PDF-centric.
