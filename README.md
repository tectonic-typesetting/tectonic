Tectonic
========

*A standalone TeX typesetting engine, powered by TeXLive and XeTeX.*

[![Build Status](https://travis-ci.org/tectonic-typesetting/tectonic.svg?branch=master)](https://travis-ci.org/tectonic-typesetting/tectonic)
[![](http://meritbadge.herokuapp.com/tectonic)](https://crates.io/crates/tectonic)

- [User website](https://tectonic-typesetting.github.io/).
- [API docs](https://tectonic-typesetting.github.io/api-docs/tectonic/).

Tectonic is a [TeX](https://en.wikipedia.org/wiki/TeX) processor provided as a
reusable library. The goal of Tectonic is to provide a *standalone* TeX/LaTeX
engine that can be embedded anywhere, bringing the power of TeX typesetting to
any application that needs it. It is forked from the
[XeTeX](http://xetex.sourceforge.net/) extension of the classic “Web2C” TeX
implementation.

For more information, see
[the Tectonic website](https://tectonic-typesetting.github.io/).


Building the Program
--------------------

To build Tectonic requires:

- C and C++ compilers
- The [Rust](https://www.rust-lang.org/) compiler and its
  [Cargo](https://crates.io/) package manager. These are easily installed
  through [rustup.rs](https://www.rustup.rs).
- [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/)
- For the time being, the following system libraries. On Macs these can all be
  installed with [Homebrew](http://brew.sh), but `harfbuzz` needs to have the
  `--with-graphite2` option enabled.
  - freetype2
  - graphite2
  - harfbuzz
  - ICU
  - libpng
  - poppler
  - zlib
- On Linux only:
  - fontconfig

Once you have these, you should be able to build the executable just by
running `cargo build`.


Testing Your Build
------------------

The `cargo test` command will run a small test suite, including the classic
TeX “TRIP” test.


About the Name
==============

The name of the project is “Tectonic,” spelled and pronounced like a regular
word because it is one. Enough with the cutesy obscurantism.

In cases where the name might lead to ambiguities, it should be expanded to
“Tectonic typesetting.”

If you’re feeling expansive, you can interpret the name as suggesting a large
change in the TeX world. Or you can think of it as suggesting a salubrious
offering for weary TeX users. Either way, the root of the word does go back to
the ancient Greek τέκτων, ”carpenter,” which Donald Knuth — a devout Christian
— might appreciate.


Copyright and Licensing
=======================

Tectonic is licensed under the MIT License. This is the license under which
the codebase from which it is primarily derived, XeTeX, is distributed.
Various other elements of the TeX system on which Tectonic is based are
licensed under other open-source licenses.
