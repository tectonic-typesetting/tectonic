# rc: minor bump

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
