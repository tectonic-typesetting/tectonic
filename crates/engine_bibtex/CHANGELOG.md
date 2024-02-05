# tectonic_engine_bibtex 0.2.2 (2024-02-05)

- Complete the port of BibTeX to pure Rust (#1077, #1083, @CraftSpider)!


# tectonic_engine_bibtex 0.2.1 (2023-06-15)

- Fix a translation bug in the growth of the `global_str` buffer
  (#1055, @CraftSpider, reported in #1054 by @giammirove)


# tectonic_engine_bibtex 0.2.0 (2023-06-12)

This is a big release! This version of the BibTeX engine doesn’t change its
behavior or APIs, but major chunks of its internals have been translated into
Rust by [@CraftSpider] in [#1032], with a bunch of new tests introduced in
[#1037] and [#1039]. This translation replaces about 2,200 lines of C code with
clean, well-organized, (mostly) safe Rust. While there aren’t any major changes
planned for the behavior of the BibTeX engine, this work demonstrates how the
legacy C/C++ code in Tectonic can gradually be replaced with safer, more modern,
more maintainable code.

[@CraftSpider]: https://github.com/CraftSpider
[#1032]: https://github.com/tectonic-typesetting/tectonic/pull/1032
[#1037]: https://github.com/tectonic-typesetting/tectonic/pull/1037
[#1039]: https://github.com/tectonic-typesetting/tectonic/pull/1039

This effort benefited significantly from work by [@Mrmaxmeier] to revitalize the
[tectonic-on-arXiv] service, which runs Tectonic pull requests on a large corpus
of [arxiv.org] preprints and reports any changes in the outputs.

[@Mrmaxmeier]: https://github.com/Mrmaxmeier
[tectonic-on-arXiv]: https://github.com/mrmaxmeier/tectonic-on-arXiv
[arxiv.org]: https://arxiv.org/


# tectonic_engine_bibtex 0.1.5 (2023-05-18)

- Treat `\r\n` sequences as a single unit (#1037, @CraftSpider). This leads to
  more uniform behavior on Windows and non-Windows platforms.


# tectonic_engine_bibtex 0.1.4 (2022-10-03)

No code changes, but some internal documentation improvements about managing FFI
APIs.


# tectonic_engine_bibtex 0.1.3 (2021-06-17)

- Switch from running [cbindgen] at build time to having the developer run it
  manually. This really ought to fix the crate builds on docs.rs ([#788]), and
  should speed builds too.

[cbindgen]: https://github.com/eqrion/cbindgen
[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788


# tectonic_engine_bibtex 0.1.2 (2021-06-17)

- Attempt to fix crate builds on docs.rs — see [#788]. This works around an
  issue in Tectonic’s usage of [cbindgen] by configuring Cargo to operate in
  offline mode when building on docs.rs, which builds crates with network access
  turned off.

[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788
[cbindgen]: https://github.com/eqrion/cbindgen


# tectonic_engine_bibtex 0.1.1 (2021-06-04)

No code changes; the Cargo package didn't publish because I hit the crates.io
rate limit in the previous batch of updates!


# tectonic_engine_bibtex 0.1.0 (2021-06-03)

This crate introduces the `bibtex` engine as a standalone crate, building on
the new "core bridge" functionality.
