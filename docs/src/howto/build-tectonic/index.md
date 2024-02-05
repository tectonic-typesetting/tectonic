# How To: Build Tectonic

This document lays out the options available for building the Tectonic [Rust
crate][rust-crate] and/or [executable] from source code. Because Tectonic relies
on a large number of system libraries and tightly-integrated C/C++ code, it can
be more challenging to compile than most Rust code.

[rust-crate]: https://doc.rust-lang.org/rust-by-example/crates.html
[executable]: https://en.wikipedia.org/wiki/Executable

For this reason, if you just want to *run* Tectonic, we recommend that you start
by [installing][install] a pre-built version if possible. Using a pre-compiled
binary will save you time and, possibly, headaches.

[install]: ../installation/index.md


## Basic Prerequisites

To build Tectonic you will need Rust, C, and C++ compilers installed. It is
beyond the scope of this document to give instructions on this topic, besides
pointing you to the [Rust installation
page](https://www.rust-lang.org/tools/install).

You do *not* necessarily need to download a copy of the Tectonic source code, if
the [cargo install] command will meet your needs.

[cargo install]: https://doc.rust-lang.org/cargo/commands/cargo-install.html


## Third-Party Dependencies

Tectonic relies on a number of well-established third-party libraries that deal
with fonts, Unicode, text shaping, and so on. Specifically:

- [fontconfig](https://fontconfig.org/) for discovering system fonts (except on
  macOS)
- [freetype2](https://www.freetype.org/) for parsing font files
- [graphite2](https://graphite.sil.org/) for shaping certain unusual scripts
- [Harfbuzz](https://harfbuzz.github.io/) for text shaping
- [ICU4C](http://site.icu-project.org/home) for Unicode data and algorithms
- [libpng](http://www.libpng.org/) for parsing PNG images
- [OpenSSL](https://www.openssl.org/) for HTTPS if you’re not on a Mac or
  Windows machine (or whichever SSL library is required for your system by the
  [rust-native-tls] crate)
- [zlib](https://zlib.net/) for compression algorithms

[Harfbuzz]: https://harfbuzz.github.io/
[rust-native-tls]: https://github.com/sfackler/rust-native-tls/#readme

To build Tectonic, your first task is to decide where you want these library
dependencies to come from.

- Tectonic can provide some dependencies **internally** (“vendor” them). This is
  the default for [Harfbuzz]. You can use [Cargo features][cargo-features],
  described below, to control when this happens. For some third-party libraries
  needed by Tectonic, vendoring is not possible.
- You can install the dependencies **externally**, with a system such as your
  OS’s package manager, and tell the Tectonic build system how to access them.
  Read [how to install Tectonic’s dependencies externally][external-dep-install]
  for quick recipes as to how to do that.
- As an intermediate approach, you can **use cargo-vcpkg** to compile the
  dependencies for Tectonic’s use with [vcpkg]. Read [this
  page][cargo-vcpkg-dep-install] to learn how to do that.

[external-dep-install]: ./external-dep-install.md
[vcpkg]: https://vcpkg.readthedocs.io
[cargo-vcpkg-dep-install]: ./cargo-vcpkg-dep-install.md

You’ll have to set up one of two ways for the Tectonic build system to gather
the appropriate information about how to compile against the external
dependencies:

- **[pkg-config]**, the default system, is the appropriate choice in most cases.
  Generally all you need to do is make sure that the `pkg-config` program is
  installed using the same framework as you used to install the library
  dependencies. You can force the Tectonic build system to use pkg-config for
  dependency discovery by setting the environment variable
  `TECTONIC_DEP_BACKEND` to the value `pkg-config`.
- **[vcpkg]** is the choice to use if you installed your dependencies this way,
  either [using cargo-vcpkg][cargo-vcpkg-dep-install] or separately. Activate
  this mode by setting the environment variable `TECTONIC_DEP_BACKEND` to the
  value `vcpkg`.

[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/

If using [pkg-config], setting the environment variable
`TECTONIC_PKGCONFIG_FORCE_SEMI_STATIC` will cause the build system to attempt to
link with external libraries statically rather than dynamically. System
libraries, such as `libc` and `libm` on Unix systems, will still be linked
dynamically. This mode is planned to be superseded by better support for
“vendoring” dependent libraries.


## Choose Cargo Features

The Cargo build framework offers the concept of [features][cargo-features] to
control build options. Tectonic offers the following features:

[cargo-features]: https://doc.rust-lang.org/cargo/reference/features.html

- **`external-harfbuzz`**. By default, the Tectonic crates will build and link
  to a “vendored” (static, internal) version of the [Harfbuzz] text shaping
  library. If you would like to link to an externally-supplied version instead,
  enable this feature.
- **`geturl-curl`**. Uses the [curl] crate to get URLs. In order for this to
  take effect, you must use `--no-default-features`, because `geturl-reqwest` is
  a default feature and takes precedence.
- **`geturl-reqwest`** (enabled by default). Uses the [reqwest] crate to get
  URLs. This is a good portable default.
- **`native-tls-vendored`**. If using [reqwest], activate the `vendored` option
  in the [native-tls] crate, causing OpenSSL to be vendored. This can be useful
  when cross-compiling or building static binaries, but is discouraged because
  that means that the resulting binaries won’t benefit from security fixes to
  system TLS libraries.

[curl]: https://docs.rs/curl/
[reqwest]: https://docs.rs/reqwest/
[native-tls]: https://github.com/sfackler/rust-native-tls

Some lesser-used features are:

- **`serialization`** (enabled by default). Disabling this feature turns off all
  Tectonic features that require the [serde] crate. This option is provided
  because Tectonic’s use of serde requires [procedural macro][proc-macro]
  support, which is not available by default on static-only compilation
  environments. However, it is likely that serialization support will become
  mandatory in the future, and one can still produce static `tectonic`
  executables using a cross-compilation approach. Therefore we do not recommend
  that you rely on this feature.
- **`profile`**. Compile Tectonic code in such a way as to make it profileable.
  In particular, this forces the C/C++ compiler to include frame pointer
  information unless it is known that such information is not needed for
  profiling on the target platform.

[serde]: https://crates.io/crates/serde
[proc-macro]: https://doc.rust-lang.org/reference/procedural-macros.html

To avoid activating a feature that is enabled by default, you must pass the
`--no-default-features` flag to the `cargo` command that you run. Features are
enabled with a flag such as `--features "serialization profile"`.


## Compile the Code

To build the latest released version of Tectonic without needing to download its
source code, first *ensure that your build environment variables are set up
properly* and determine what feature flags you need. Then run:

```sh
cargo install tectonic
```

inserting any feature flags after the `install`. To install the latest version
from Git, do the same but use:

```sh
cargo install --git https://github.com/tectonic-typesetting/tectonic.git
```

Many other variations are possible. See the [cargo install] documentation for
more.

If you have downloaded its source code (perhaps because you wish to make your
own improvements), make sure that you’re inside the Tectonic source tree and run:

```sh
cargo build
```

once again adding any feature flags and ensuring that any necessary build
environment variables are set up properly. Read [The Cargo Book][cargo-book] for
vastly more information about where you can go from there.

[cargo-book]: https://doc.rust-lang.org/cargo/index.html
