# rc: minor bump

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
