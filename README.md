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

[![Build Status](https://travis-ci.org/tectonic-typesetting/tectonic.svg?branch=master)](https://travis-ci.org/tectonic-typesetting/tectonic)
[![](http://meritbadge.herokuapp.com/tectonic)](https://crates.io/crates/tectonic)
[![codecov](https://codecov.io/gh/tectonic-typesetting/tectonic/branch/master/graph/badge.svg)](https://codecov.io/gh/tectonic-typesetting/tectonic)
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

- You can use Tectonic as a
  [GitHub Action](https://github.com/features/actions) with
  [compile-latex](https://github.com/marketplace/actions/compile-latex),
  thanks to [Vinay Sharma](https://github.com/vinay0410).
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


## Features

The Tectonic build can be customized with the following features:

##### serialization (enabled by default)

This feature enables (de)serialization using the [serde](https://serde.rs/)
crate. At the moment, this is only used to read per-user configuration from a
[TOML](https://github.com/toml-lang/toml) file. If this feature is disabled,
the per-user configuration file will be silently ignored.

This functionality is optional because it requires the `serde_derive` crate,
which in turn uses Rust’s `proc_macro` feature. The `proc_macro` functionality
[is not available on musl targets](https://github.com/rust-lang/rust/issues/40174),
and so must be turned off if you wish to build a completely static Tectonic
executable.
