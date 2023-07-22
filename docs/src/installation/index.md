# How To: Install Tectonic

One of the big advantages that Tectonic offers compared to the traditional TeX
stack is that all of Tectonic’s functionality is delivered in a single
executable file — not the usual tree of thousands of interlocking data files and
binary tools.

You have several options for obtaining the Tectonic executable. The best choice
depends on your computing environment and your needs.

- **For the fastest and easiest installation, [copy-paste a command into your
  terminal](#copy-paste-a-terminal-command)** that will automatically download
  the right Tectonic program for your computer
- [Direct download](#direct-download) a Tectonic release
- [Pre-built binary packages](#pre-built-binary-packages) for your favorite
  operating system or package manager
- [Compile it yourself](#compile-tectonic-yourself)

The [copy-paste method](#copy-paste-a-terminal-command) should cover most use
cases, but if you want better integration with your operating system or
computing environment, [packaged versions](#pre-built-binary-packages) might
make more sense. There should be no need to compile Tectonic yourself unless you
want to, or you’re hoping to run it on an unusual platform.


## Copy-paste a terminal command

This is generally the easiest way to get Tectonic onto your computer. On a
computer running a Unix-like operating system, including macOS, just run the
following command in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -fsSL https://drop-sh.fullyjustified.net |sh
```

This will download the `tectonic` program and place it into the directory where
you ran the command.

On Windows, you can do the same in a PowerShell window, which will unpack
`tectonic.exe` for you:

```ps1
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://drop-ps1.fullyjustified.net'))
```

No matter your operating system, you should probably move the unpacked file into
a directory in your executable search path so that you can run Tectonic from any
working directory. For the time being, the download script doesn’t do this
because it can be tricky to automatically determine what the best installation
destination would be.


## Direct download

You can [download the latest release of Tectonic here][gh-latest] on GitHub. Each
release is published with precompiled executables attached. Because Tectonic is
distributed as a single executable, all you need to do is download the
appropriate archive for your platform and unpack it.

[gh-latest]: https://tectonic-typesetting.github.io/latest.html

For instance, on most Linux systems, you’ll want to download the file with the
name looking like `tectonic-<VERSION>-x86_64-unknown-linux-gnu.tar.gz`. This
tarball will unpack to a single file, `tectonic`, which is the Tectonic
executable.


## Pre-built binary packages

Tectonic may be available in precompiled packages offered by either your
operating system or standalone package managers. Check out [the Tectonic source
repository][repo] for an up-to-date listing.

[repo]: https://github.com/tectonic-typesetting/tectonic/#readme

In most cases, the package name will be `tectonic` and will provide a
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
Conda packaging project. To install Tectonic you must activate conda-forge,
which can be done temporarily like so:

```sh
$ conda install -c conda-forge tectonic
```

### Arch Linux

For users of [Arch Linux], there are two Tectonic packages available: [tectonic][arch-tectonic] from the official repositories, which can be installed with
```sh
$ sudo pacman -S tectonic
```
and [tectonic-git][arch-tectonic-git] from the [AUR].

[Arch Linux]: https://archlinux.org/
[arch-tectonic]: https://archlinux.org/packages/community/x86_64/tectonic/
[AUR]: https://aur.archlinux.org/
[arch-tectonic-git]: https://aur.archlinux.org/packages/tectonic-git

### Homebrew

There is a `tectonic` package in [Homebrew](https://brew.sh/). If you already
have Homebrew installed, installing Tectonic should be as simple as:

```sh
$ brew install tectonic
```

We also have instructions about [installing Tectonic’s dependencies using
Homebrew][homebrew-deps] if you’d like to compile Tectonic yourself on your
Homebrew-based computer.

[homebrew-deps]: /howto/build-tectonic/external-dep-install.md#homebrew-on-macos

### MacPorts

There is a `tectonic` port in [MacPorts](https://www.macports.org/). If you
already have MacPorts installed, installing Tectonic should be as simple as:

```sh
$ sudo port install tectonic
```

### nix or nixOS

If you’re using [nix], you can imperatively install [`tectonic`][nix-tectonic] with:

```sh
$ nix-env -f '<nixpkgs>' -iA tectonic
```

in your shell. You can also create a temporary environment using
[`nix-shell`](https://nixos.org/nix/manual/#sec-nix-shell):

```sh
$ nix-shell '<nixpkgs>' -A tectonic
```

[nix]: https://nixos.org/
[nix-tectonic]: https://nixos.org/nixos/packages.html#tectonic

### Void Linux

Void Linux has a `tectonic` package in the [void-packages] repository. To
install it, run:

```sh
$ sudo xbps-install -S tectonic
```

[void-packages]: https://github.com/void-linux/void-packages/blob/master/srcpkgs/tectonic/template



## Compile Tectonic Yourself

You can always compile Tectonic yourself. If your system has C++ and Rust
compilers installed, this may be as simple as running:

```sh
$ cargo install tectonic
```

However, Tectonic requires various C/C++ support libraries that may not be
available on your system by default. There are also various build options that
you can turn on and off if you have more specialized needs. For all the details,
consult the [How To Build Tectonic][howto-build] guide.

[howto-build]: ../howto/build-tectonic/index.md
