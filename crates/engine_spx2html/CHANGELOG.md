# tectonic_engine_spx2html 0.1.1 (2022-03-02)

- Use the new pinot 0.1.4, which adds the new APIs needed by Tectonic (#870,
  @pkgw). This should fix `cargo install tectonic`, which didn't work because
  Git repositories can't be used as Crates.io dependencies.


# tectonic_engine_spx2html 0.1.0 (2022-02-28)

Introduce this crate, which starts implementing Tectonic's HTML output. It uses
[tera] for templating and a hacked version of [pinot] for parsing font files,
which turns out to be important for precise typography.

The implementation is still very much preliminary, and the associated HTML and
CSS templates haven't yet been published anywhere, so it's not very useful as
yet, but hopefully that will all be changing soon.

[tera]: https://crates.io/crates/tera
[pinot]: https://crates.io/crates/pinot
