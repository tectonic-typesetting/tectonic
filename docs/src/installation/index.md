# How To: Install Tectonic

One of the big advantages that Tectonic offers compared to the traditional TeX
stack is that all of Tectonic’s functionality is delivered in a single
executable file — not the usual tree of thousands of interlocking data files and
binary tools.

***Know what you want? [Download the latest pre-built Tectonic binaries
here][gh-latest].***

You have several options for obtaining the Tectonic executable. The best choice
depends on your computing environment and your needs.

- [Direct download](#direct-download) a Tectonic release
- [Pre-built binary packages](#pre-built-binary-packages) for your favorite
  operating system or package manager
- [Compile it yourself](#compile-tectonic-yourself)

The [direct download](#direct-download) method should cover most use cases, but
if you want better integration with your operating system or computing
environment, [packaged versions](#pre-built-binary-packages) might make more
sense. There should be no need to compile Tectonic yourself unless you want to,
or you’re hoping to run it on an unusual platform.


## Direct download

You can [download the latest release of Tectonic here][gh-latest] on GitHub. Each
release is published with precompiled executables attached. Because Tectonic is
distributed as a single executable, all you need to do is download the
appropriate archive for your platform and unpack it.

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

### Pre-built packages for (Ana)conda

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

### Pre-built packages for Arch Linux

For users of [Arch Linux], there are two Tectonic packages available. The
[tectonic-bin][arch-tectonic-bin] package on [AUR] provides actual pre-compiled
binaries, while the [tectonic][arch-tectonic] will build the binary on your
machine, which may take a while.

[Arch Linux]: https://archlinux.org/
[arch-tectonic-bin]: https://aur.archlinux.org/packages/tectonic-bin/
[AUR]: https://aur.archlinux.org/
[arch-tectonic]: https://aur.archlinux.org/packages/tectonic/


## Compile Tectonic Yourself

You can always compile Tectonic yourself. If your system has C++ and Rust
compilers installed, this may be as simple as running:

```sh
cargo install tectonic
```

However, Tectonic requires various C/C++ support libraries that may not be
available on your system by default. There are also various build options that
you can turn on and off if you have more specialized needs. For all the details,
consult the [How To Build Tectonic][howto-build] guide.

[howto-build]: ../howto/build-tectonic/index.md
