# The Tectonic Typesetting System

The `tectonic` crate delivers a modernized, complete, self-contained
[TeX](https://en.wikipedia.org/wiki/TeX)/[LaTeX](https://www.latex-project.org/)
engine, powered by [XeTeX](http://xetex.sourceforge.net/) and
[TeXLive](https://www.tug.org/texlive/). It provides a CLI and ties together the
functionality of a large collection of sub-crates.

- [User Website](https://tectonic-typesetting.github.io/)
- [The Tectonic Book](https://tectonic-typesetting.github.io/book/latest/)
- [Installation](https://tectonic-typesetting.github.io/book/latest/installation/) of the CLI
- [Detailed Build Instructions][build-inst]
- [API Documentation](https://docs.rs/tectonic/)
- [Issues](https://github.com/tectonic-typesetting/tectonic/issues/)
- [Changelog](https://github.com/tectonic-typesetting/tectonic/blob/release/CHANGELOG.md)

[build-inst]: https://tectonic-typesetting.github.io/book/latest/howto/build-tectonic/


## Building Tectonic

The Tectonic crates collectively depend on a relatively large number of
third-party libraries. See [the Book’s build instructions][build-inst] for a
listing and guidance on setting up the build.

The crate build can be customized with the following [Cargo features]:

[Cargo features]: https://doc.rust-lang.org/cargo/reference/features.html

- `external-harfbuzz`: build against an external version of the [Harfbuzz] text
  shaping library, rather than a vendored version (the default)
- `geturl-curl`: use the [curl] crate to implement HTTP requests. In order for
  this to take effect, you must use `--no-default-features` because
  `geturl-reqwest` is a default feature and it takes precedence
- `geturl-reqwest`: use the [reqwest] crate to implement HTTP requests (enabled
  by default)
- `native-tls-vendored`: if using [reqwest], activate the `vendored` option in
  the [native-tls] crate, causing OpenSSL to be vendored

[Harfbuzz]: https://harfbuzz.github.io/
[curl]: https://docs.rs/curl/
[reqwest]: https://docs.rs/reqwest/
[native-tls]: https://github.com/sfackler/rust-native-tls

[The Book][build-inst] describes some less-used features and provides a bit more
detail.


## Sub-crates

The main `tectonic` crate ties together the functionality of a number of
sub-crates:

- [`tectonic_bridge_core`](https://crates.io/crates/tectonic_bridge_core)
- [`tectonic_bridge_flate`](https://crates.io/crates/tectonic_bridge_flate)
- [`tectonic_bridge_freetype2`](https://crates.io/crates/tectonic_bridge_freetype2)
- [`tectonic_bridge_graphite2`](https://crates.io/crates/tectonic_bridge_graphite2)
- [`tectonic_bridge_harfbuzz`](https://crates.io/crates/tectonic_bridge_harfbuzz)
- [`tectonic_bridge_icu`](https://crates.io/crates/tectonic_bridge_icu)
- [`tectonic_cfg_support`](https://crates.io/crates/tectonic_cfg_support)
- [`tectonic_dep_support`](https://crates.io/crates/tectonic_dep_support)
- [`tectonic_engine_bibtex`](https://crates.io/crates/tectonic_engine_bibtex)
- [`tectonic_engine_xdvipdfmx`](https://crates.io/crates/tectonic_engine_xdvipdfmx)
- [`tectonic_engine_xetex`](https://crates.io/crates/tectonic_engine_xetex)
- [`tectonic_errors`](https://crates.io/crates/tectonic_errors)
- [`tectonic_geturl`](https://crates.io/crates/tectonic_geturl)
- [`tectonic_io_base`](https://crates.io/crates/tectonic_io_base)
- [`tectonic_pdf_io`](https://crates.io/crates/tectonic_pdf_io)
- [`tectonic_status_base`](https://crates.io/crates/tectonic_status_base)
- [`tectonic_xdv`](https://crates.io/crates/tectonic_xdv)
- [`tectonic_xetex_layout`](https://crates.io/crates/tectonic_xetex_layout)
