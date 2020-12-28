# Installing Tectonic

One of the big advantages that Tectonic offers compared to the traditional TeX
stack is that all of Tectonic’s functionality is delivered in a single
executable file — not the usual tree of thousands of interlocking data files and
binary tools.

***Know what you want? [Download the latest pre-built Tectonic binaries
here][gh-latest].***

You have several options for installing the Tectonic executable. The best choice
depends on your computing environment and your needs.

- [Direct download](#direct-download)
- [Pre-built binary packages](#pre-built-binary-packages)
- [The `cargo install` method](#the-cargo-install-method)

The [direct download](#direct-download) method should meet your needs unless
you’re hoping to run Tectonic on an unusual platform, in which case you will
probably need to [install Tectonic using Cargo](#the-cargo-install-method).


## Direct download

You can [download the latest release of Tectonic][gh-latest] from GitHub. Each
release is published with precompiled executables attached. Because Tectonic is
distributed as a single executable, all you need to do is download the
appropriate archive for your platform.

[gh-latest]: https://github.com/tectonic-typesetting/tectonic/releases/latest

For instance, on most Linux systems, you’ll want to download the file with the
name looking like `tectonic-<VERSION>-x86_64-unknown-linux-gnu.tar.gz`. This
tarball will unpack to a single file, `tectonic`, that is the Tectonic
executable.


## Pre-built binary packages

Tectonic may be available in precompiled packages offered by either your
operating system or standalone package managers. Check out [the Tectonic source
repository][repo] for an up-to-date listing.

[repo]: https://github.com/tectonic-typesetting/tectonic/#readme

In most cases, the package name will be `tectonic` and it will provide a
command-line tool also named `tectonic`.

### Pre-built packages for Anaconda

Tectonic is available for the [Conda package manager][conda], which has the
advantages of (1) not requiring any administrator privileges and (2) supporting
Windows, macOS, and Linux. If you’d like to install Tectonic using a package
manager and you’re not aware of a different option that’s a better fit for your
needs, we recommend using Conda.

[conda]: https://docs.conda.io/

If you don’t already have Conda installed, we recommend that you [install the
“Miniconda” package][miniconda] provided by [Anaconda, Inc.][anaconda]. Once
complete, the command `conda` will now be available in your terminal.

[miniconda]: https://docs.conda.io/en/latest/miniconda.html
[anaconda]: https://www.anaconda.com/

Once the `conda` command is available, you can install Tectonic and its support
libraries using [conda-forge](http://conda-forge.github.io/), a community-led
Conda packaing project. To install Tectonic you must activate conda-forge, which
can be done temporarily like so:

```sh
conda install -c conda-forge tectonic
```


## The `cargo install` method

If your system has C++ and Rust compilers installed, you can install Tectonic
using Rust’s packaging tool, [cargo](http://doc.crates.io/index.html). Make sure
that Rust's `bin` folder (`$HOME/.cargo/bin` by default) is in your `$PATH`.

Tectonic currently requires various system support libraries to be installed.
This is the most likely cause of difficulty installing Tectonic. These libraries are:

- [fontconfig](https://fontconfig.org/) (except on macOS)
- [freetype2](https://www.freetype.org/)
- [graphite2](https://graphite.sil.org/)
- [harfbuzz](https://harfbuzz.github.io/)
- [ICU4C](http://site.icu-project.org/home)
- [libpng](http://www.libpng.org/)
- [zlib](https://zlib.net/)

There is significant interest in reducing the number of external dependencies to
ease the build process, but the above is where things currently stand.

### Debian and Ubuntu Linux

Ensure that the requisite packages are installed with the following command:

```sh
sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
```

Once that is done, the following should be sufficient to download and install
the latest Tectonic release:

```sh
cargo install tectonic
```

### RHEL, CentOS, or Fedora Linux

Ensure that the requisite packages are installed with the following command:

```sh
sudo dnf install fontconfig-devel graphite2-devel harfbuzz-devel libicu-devel openssl-devel zlib-devel
```

Once that is done, the following should be sufficient to download and install
the latest Tectonic release:

```sh
cargo install tectonic
```

### MacOS / OS X, with Homebrew

To `cargo install` Tectonic on macOS, the recommended way to install the
required dependencies is with [Homebrew](http://brew.sh). Note, however, that
you can also just directly install `tectonic`  with Homebrew as well. If you
want to compile it yourself, the following commands will install the required
formulae:

```sh
brew install --only-dependencies tectonic
brew install pkg-config
```

If the Rust build program `cargo` not available, also run:

```sh
brew install rust
```

The `cargo` invocation is then as normal, but you must set up some auxiliary
environment variables as well:

```sh
export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
export PKG_CONFIG_PATH=/usr/local/opt/icu4c/lib/pkgconfig
cargo install tectonic
```

Alternatively, you can build tectonic from source by running

```sh
brew install --build-from-source tectonic
```


## Other methods

See also some pages in the “Cookbooks” section of this book for more specialized
instructions.