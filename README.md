Tectonic
========

*A standalone TeX typesetting engine, powered by TeXLive and XeTeX.*

[![Build Status](https://travis-ci.org/pkgw/tectonic.svg?branch=master)](https://travis-ci.org/pkgw/tectonic)

Tectonic is a [TeX](https://en.wikipedia.org/wiki/TeX) processer provided as a
reusable library. The goal of Tectonic is to provide a *standalone* TeX/LaTeX
engine that can be embedded anywhere, bringing the power of TeX typesetting to
any application that needs it. It is forked from the
[XeTeX](http://xetex.sourceforge.net/) extension of the classic “Web2C” TeX
implementation.

There are two key differences between Tectonic and other TeX engines:

- The engine has been turned into a well-behaved library. Tectonic’s I/O
  backend obtains resources from a Zip file called a “bundle” so that you
  don’t need a specialized filesystem tree containing thousands of files in
  order to process realistic documents.
- The implementation has been fully detached from the classic implementation
  in the [WEB](https://en.wikipedia.org/wiki/WEB) language and from the
  associated nest of support tools. Tectonic is written in C, C++, and Rust,
  and its implementation is self-contained.

Tectonic is almost completely derived from XeTeX and relies upon the
infrastructure developed by the [TeXLive](https://www.tug.org/texlive/)
project. XeTeX was chosen because it can load modern system fonts and process
Unicode input files.


Current Status and Roadmap
--------------------------

The Tectonic codebase currently builds a standalone program that works almost
identically to XeTeX. The most important difference is that it looks for
support files in a Zip file, rather than searching filesystem trees using the
[kpathsea](https://www.tug.org/kpathsea/) framework. Tools to generate bundle
files are provided in the
[tectonic-staging](https://github.com/pkgw/tectonic-staging) repository.

Tectonic should build on both Linux and Mac machines.

The current task is a general rearrangement of the code to isolate and tidy
the I/O-related layers. These will be replaced with new code that will allow
the possibility of processing LaTeX documents in fully-embedded fashion,
namely, without producing the usual assortment of output files (`file.log`,
`file.aux`, etc.).

There is also a significant need to tidy up the machine-generated C code and
develop a test suite to validate the system. (Tectonic does pass the TRIP and
eTRIP tests.)

In the long term, the goal is to tidy up the code as much as possible and
package the full functionality into an easy-to-use library. Ideally the
implementation will be ported to pure [Rust](https://www.rust-lang.org/) so
that it can be compiled for fun targets like
[WebAssembly](http://webassembly.org/). That will take a great deal of effort,
however. Modern documentation of the internals (i.e., not literate-programming
style) would also be nice.


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

For now, just run `tests/run.sh`. Hopefully we’ll be moving to a better test
rig soon.


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
