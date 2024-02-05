# tectonic_engine_spx2html 0.3.2 (2024-02-05)

- Fix (well, suppress) a new Clippy complaint (#1137, @pkgw).


# tectonic_engine_spx2html 0.3.1 (2023-06-12)

- Avoid emitting negative padding values, which are illegal (#1052, @pkgw)
- Close automatic tags when starting certain automatic tags (#1052, @pkgw)


# tectonic_engine_spx2html 0.3.0 (2023-05-18)

- A massive rework to support more sophisticated HTML output for the
  [Tectonopedia] project (#1016, @pkgw). This crate is still highly unstable so
  we're not going to document them.

[Tectonopedia]: https://github.com/tectonic-typesetting/tectonopedia


# tectonic_engine_spx2html 0.2.1 (2022-10-27)

- Avoid a dumb crash when attempting to compile documents that have not been
  set up for the Tectonic HTML compilation framework (#955, @pkgw). Note,
  however, that generic documents will still fail to build in HTML mode.


# tectonic_engine_spx2html 0.2.0 (2022-10-04)

- Many updates for [tt-weave] (#941, @pkgw). This crate is still highly unstable
  so we're not going to document them.

[tt-weave]: https://github.com/pkgw/tt-weave/


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
