# Installing Tectonic

One of the big advantages that Tectonic offers compared to the traditional TeX
stack is that all of Tectonic’s functionality is delivered in a single
executable file — not the usual tree of thousands of interlocking data files and
binary tools.

You have several options for installing the Tectonic executable. The best choice
depends on your computing environment and your needs.

- [Pre-built binary packages](#pre-built-binary-packages)
  - Currently available for Arch Linux, [nixpkgs,
    nixOS](https://nixos.org/nixos/packages.html#tectonic) and MacPorts/Homebrew
    on macOS/OS X
- [The `cargo install` method](#the-cargo-install-method)
  - Easiest if you already have C++ and Rust development tools installed
- [The Anaconda method](#the-anaconda-method)
  - Reliable and cross-platform

If unsure, try [the Anaconda method](#the-anaconda-method). It involves a few
steps in the command line and a few downloads, but it is straightforward and
reliable.


## Pre-built binary packages

Tectonic may be available in precompiled packages offered by either your
operating system or standalone package managers. Check out [the Tectonic source
repository][repo] for an up-to-date listing.

[repo]: https://github.com/tectonic-typesetting/tectonic/#readme

In most cases, the package name will be `tectonic` and it will provide a
command-line tool also named `tectonic`.


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

```
sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
```

Once that is done, the following should be sufficient to download and install
the latest Tectonic release:

```
cargo install tectonic
```

### RHEL, CentOS, or Fedora Linux

Ensure that the requisite packages are installed with the following command:

```
sudo dnf install fontconfig-devel graphite2-devel harfbuzz-devel libicu-devel openssl-devel zlib-devel
```

Once that is done, the following should be sufficient to download and install
the latest Tectonic release:

```
cargo install tectonic
```

### MacOS / OS X, with Homebrew

To `cargo install` Tectonic on macOS, the recommended way to install the
required dependencies is with [Homebrew](http://brew.sh). Note, however, that
you can also just directly install `tectonic`  with Homebrew as well. If you
want to compile it yourself, the following commands will install the required
formulae:

```
brew install --only-dependencies tectonic
brew install pkg-config
```

If the Rust build program `cargo` not available, also run:

```
brew install rust
```

The `cargo` invocation is then as normal, but you must set up some auxiliary
environment variables as well:

```
export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
export PKG_CONFIG_PATH=/usr/local/opt/icu4c/lib/pkgconfig
cargo install tectonic
```

Alternatively, you can build tectonic from source by running

```
brew install --build-from-source tectonic
```


## The Anaconda method

If the other installation methods do not or cannot work for you, another
reliable installation method is based on the Anaconda Python framework, even
though nothing in Tectonic uses Python.

If you don’t already have an Anaconda environment set up, we recommend that you
use Continuum’s “Miniconda” installer to get set up quickly. First, install
Miniconda [according to these official
instructions](https://conda.io/docs/install/quick.html). Once Miniconda is
installed you may need to open a new terminal in order for its changes to take
effect. If all went well, the command `conda` will now be available in your
terminal.

Once the `conda` command is available, installation is straightforward. Tectonic
and its support libraries are provided through
[conda-forge](http://conda-forge.github.io/), a community-led project that
emulates and updates the official Anaconda system. To install Tectonic you must
activate conda-forge, which can be done temporarily like so:

```
conda install -c conda-forge tectonic
```



## Other methods

See also some pages in the “Cookbooks” section of this book for more specialized
instructions.