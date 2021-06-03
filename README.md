[![Build Status](https://dev.azure.com/tectonic-typesetting/tectonic/_apis/build/status/tectonic-typesetting.tectonic?branchName=master)](https://dev.azure.com/tectonic-typesetting/tectonic/_build/latest?definitionId=11&branchName=master)
[![](http://meritbadge.herokuapp.com/tectonic)](https://crates.io/crates/tectonic)
[![codecov](https://codecov.io/gh/tectonic-typesetting/tectonic/branch/master/graph/badge.svg)](https://codecov.io/gh/tectonic-typesetting/tectonic)

# Tectonic

Tectonic is a modernized, complete, self-contained
[TeX](https://en.wikipedia.org/wiki/TeX)/[LaTeX](https://www.latex-project.org/)
engine, powered by [XeTeX](http://xetex.sourceforge.net/) and
[TeXLive](https://www.tug.org/texlive/).

## Read this first

If you just want to compile TeX documents, you should probably **click through to
[the main Tectonic website](https://tectonic-typesetting.github.io/)**. This
page is primarily aimed at folks interested in how Tectonic works “under the hood.”

## Developer dashboard

<a href="https://repology.org/metapackage/tectonic">
    <img src="https://repology.org/badge/vertical-allrepos/tectonic.svg" alt="Packaging status" align="right">
</a>

- [User website](https://tectonic-typesetting.github.io/).
- [Community discussion forum](https://tectonic.newton.cx/).
- [Installation](https://tectonic-typesetting.github.io/install.html).
- [Developer documentation, including build instructions](https://tectonic-typesetting.github.io/develop.html).
- [API documentation](https://docs.rs/tectonic/).
- [Issues](https://github.com/tectonic-typesetting/tectonic/issues/).
- [Changelog](./CHANGELOG.md).

## Technical ecosystem

If you’re interested in Tectonic as a software tool, you might also want to check out:

- One of the following [GitHub Actions](https://github.com/features/actions)
    - [setup-tectonic](https://github.com/marketplace/actions/setup-tectonic), which lets you use tectonic directly in your workflows (supports caching + optionally biber)
    - [compile-latex](https://github.com/marketplace/actions/compile-latex) contributed by [Vinay
  Sharma](https://github.com/vinay0410), which is powered by Tectonic.
- [tt.ente.ninja](https://tt.ente.ninja), which runs Tectonic against a subset
  of the [arxiv.org](https://arxiv.org/) corpus à la the Rust tool
  [Crater](https://github.com/rust-lang/crater) — a
  [project](https://github.com/Mrmaxmeier/tectonic-on-arXiv) by
  [@Mrmaxmeier](https://github.com/Mrmaxmeier/)

## The “reference sources”

Much of the core code of Tectonic is derived from
[XeTeX](http://xetex.sourceforge.net/), and we strive to track and maintain
compatibility with upstream as much as possible. However, the nature of the
Tectonic project is such that its source code is going to diverge from that of
XeTeX over time. We can do our best to track the *semantics* of changes to
XeTeX, but the expression of those changes in source form may well change
greatly over time.

In this repository, the Git submodule `reference_sources` links to the
[“staging repository”](https://github.com/tectonic-typesetting/tectonic-staging)
that tracks the XeTeX source
code that we use as a reference. In particular, the version of the reference
code in the submodule is the most recent code whose semantics are *guaranteed*
to be expressed in Tectonic, to the best of our efforts. You don’t need to
clone `reference_sources` to build Tectonic (which is good because everyone is
always super confused by how Git submodules work!). It just provides a
convenient way for Git to track the exact reference code that we are using at
any given time.

Please see
[the tectonic-staging README](https://github.com/tectonic-typesetting/tectonic-staging#readme)
for more information. (Or at least, more words on the topic.)


## Cargo Features

The Tectonic build can be customized with the following [Cargo features]:

[Cargo features]: https://doc.rust-lang.org/cargo/reference/features.html

##### `external-harfbuzz`

By default, the Tectonic crates will build and link to a “vendored” (static,
internal) version of the [Harfbuzz] text shaping library. If you would like to
link to an externally-supplied version instead, enable this feature. The
external library can be detected with either [pkg-config] or [vcpkg]. See the
[How To Build Tectonic][howto-build] guide for more details.

[Harfbuzz]: https://harfbuzz.github.io/
[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/
[vcpkg]: https://vcpkg.readthedocs.io/
[howto-build]: https://tectonic-typesetting.github.io/book/latest/#update-link-when-published

##### `geturl-curl`

Use the [curl] crate to implement HTTP requests. In order for this to take
effect, you must use `--no-default-features` because `geturl-reqwest` is a
default feature and it takes precedence.

[curl]: https://docs.rs/curl/

##### `geturl-reqwest` (enabled by default)

Use the [reqwest] crate to implement HTTP requests. This is the default
selection.

[reqwest]: https://docs.rs/reqwest/

##### `native-tls-vendored`

If using [reqwest], activate the `vendored` option in the [native-tls] crate,
causing OpenSSL to be vendored. This can be useful when cross-compiling or
building static binaries, but is discouraged because that means that the
resulting binaries won't benefit from security fixes to system TLS libraries.

[native-tls]: https://github.com/sfackler/rust-native-tls

##### `serialization` (enabled by default)

This feature enables (de)serialization using the [serde](https://serde.rs/)
crate. At the moment, this is only used to read per-user configuration from a
[TOML](https://github.com/toml-lang/toml) file. If this feature is disabled, the
per-user configuration file will be silently ignored. This feature is provided
because serialization requires the `serde_derive` crate, which in turn uses
Rust’s `proc_macro` feature. The `proc_macro` functionality is difficult to
build with statically linked targets, although Tectonic’s CI system
[demonstrates how to make it work][static-proc-macro].

[static-proc-macro]: https://github.com/tectonic-typesetting/tectonic-ci-support/tree/master/cross-images#readme
