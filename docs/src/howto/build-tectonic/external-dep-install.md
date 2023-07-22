# How To: Build Tectonic: Install System Dependencies

Here are quick commands to install Tectonic’s dependencies using various package
managers:

- [Debian and Ubuntu Linux](#debian-and-ubuntu-linux) and related variants
- [RHEL, CentOS, and Fedora Linux](#rhel-centos-and-fedora-linux) and related variants
- [Homebrew on macOS](#homebrew-on-macos)
- [conda](#conda) on various operating systems
- [vcpkg](#vcpkg) on various operating systems (including Windows)

If none of these fit your needs, you’ll need to figure out the right packages
for your particular setup. Tectonic requires the following libraries:

- [fontconfig](https://fontconfig.org/) (except on macOS)
- [freetype2](https://www.freetype.org/)
- [graphite2](https://graphite.sil.org/)
- [harfbuzz](https://harfbuzz.github.io/)
- [ICU4C](http://site.icu-project.org/home)
- [libpng](http://www.libpng.org/)
- [zlib](https://zlib.net/)
- Whichever SSL library is required for your system by the [rust-native-tls]
  crate: probably [OpenSSL](https://www.openssl.org/)

[rust-native-tls]: https://github.com/sfackler/rust-native-tls/#readme


## Debian and Ubuntu Linux

Install Tectonic’s dependencies with:

```sh
sudo apt-get install \
  libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
```


## RHEL, CentOS, and Fedora Linux

Install Tectonic’s dependencies with:

```sh
sudo dnf install \
  gcc-c++ fontconfig-devel graphite2-devel harfbuzz-devel libicu-devel openssl-devel zlib-devel
```


## Homebrew on macOS

If you use [Homebrew], be aware that you can install Tectonic with it directly!

[Homebrew]: https://brew.sh

```sh
brew install tectonic
```

If you want to compile Tectonic yourself, the following command will install the
dependencies:

```sh
brew install --only-dependencies tectonic
```

You will also need to make sure that your environment has [pkg-config] set up to
find the Homebrew libraries correctly.

[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/


## Conda

If you use [Conda], be aware that you can install Tectonic with it directly,
using the [conda-forge] channel!

[Conda]: https://docs.conda.io/
[conda-forge]: https://conda-forge.org/

```sh
conda install -c conda-forge tectonic
```

But if you want to compile Tectonic yourself, the following command will install
the dependencies:

```sh
conda install fontconfig freetype graphite2 harfbuzz icu libpng openssl zlib
```

You will also need to make sure that your environment has [pkg-config] set up to
find the Conda libraries correctly.


## vcpkg

If you wish to use [vcpkg] to provide Tectonic’s build dependencies, we
recommend that you use [the cargo-vcpkg tool](./cargo-vcpkg-dep-install.md). But
for the record, to install Tectonic’s dependencies through vcpkg directly, you
should probably run:

[vcpkg]: https://vcpkg.readthedocs.io/

```sh
vcpkg install fontconfig freetype "harfbuzz[icu,graphite2]"
```
