# tectonic_xetex_layout 0.2.4 (2024-02-05)

- Remove a hack related to `libicudata` from the build script; this is now
  handled in the proper place, in the `tectonic_bridge_icu` crate (#1092,
  @pkgw).


# tectonic_xetex_layout 0.2.3 (2023-06-12)

- Fix a potential crash on invalid font files (#1035, @Mrmaxmeier).


# tectonic_xetex_layout 0.2.2 (2023-05-18)

- Tidy up recent Clippy warnings.


# tectonic_xetex_layout 0.2.1 (2022-10-03)

- Work around ICU limitations in Alpine 3.16. The latest version of Alpine Linux
  seems to provide a static ICU that no longer has the "macintosh" converter
  built in. So don't error out if it fails to load; just hope that everything
  will be OK.


# tectonic_xetex_layout 0.2.0 (2022-04-26)

Update for TeXLive 2021 (#882, @pkgw).

- Add new C API needed for TeXLive 2021: `ttxl_font_get_point_size`.


# tectonic_xetex_layout 0.1.1 (2021-10-11)

- Require the latest version of `tectonic_bridge_graphite2`, which contains a
  Windows build fix.
- Fixes for Clippy 1.53.0


# tectonic_xetex_layout 0.1.0 (2021-06-03)

This new crate encapsulates the font selection and layout code used by the
`tectonic_engine_xetex` crate. While it mostly consists of C/C++ code at the
moment and does not expose a Rust API, there is a hope that it can be made more
flexible and that its implementation can be migrated to be more Rust-based.
