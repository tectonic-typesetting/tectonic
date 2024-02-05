# tectonic_xetex_format 0.3.2 (2024-02-05)

- Update for latest Clippy complaints (#1119, @pkgw).


# tectonic_xetex_format 0.3.1 (2023-05-18)

- Tidy up recent Clippy warnings.


# tectonic_xetex_format 0.3.0 (2022-10-03)

- Define version 33 of the format in support of TeXLive 2022.0 (#936, @pkgw)
  - Remove CHAR_SUB_CODE_BASE from here on out. It is only needed for MLTeX,
    which is disabled in XeTeX. This section of the equivalents table had one
    entry for every USV, which is a lot of space.
  - Synchronize PRIM_SIZE with TeXLive 2022, and provide PRIM_PRIME

# tectonic_xetex_format 0.2.0 (2022-04-26)

Update for TeXLive 2021 (#882, @pkgw):

- There is one new integer parameter: `\tracingstacklevels`
- Bump `PRIM_SIZE` to 510, since we have passed 500 primitives!


# tectonic_xetex_format 0.1.0 (2022-02-28)

The new `tectonic_xetex_format` crate defines metadata about the Tectonic/XeTeX
engine implementation. It has two major use cases:

- Generate the C headers used by `tectonic_engine_xetex` for its implementation
- Allow introspection of Tectonic/XeTeX "format files"

This latter functionality will allow use to answer questions such as "what
control strings are defined in this LaTeX format?" or "what is the built-in
definition of this macro?"

The elements of the format definition are all versioned, so that as the engine
evolves we should retain the ability to introspect older formats.
