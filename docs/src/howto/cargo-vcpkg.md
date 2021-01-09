# build tectonic with vcpkg on Windows, Linux or macOS

## prerequisites

This guide assumes you have [git](https://git-scm.com/) installed and are
comfortable using the command-line.

## setting up your environment

If you are running macOS prior to 10.15 (Catalina), or a version of Xcode that is not up to date, you may need to [install gcc](#Install-gcc-on-macOS). Windows and Linux should be ready to go if you have a recent compiler installed.

## Install cargo-vcpkg

```sh
cargo install cargo-vcpkg
```

## Install tectonic dependencies using vcpkg

```sh
$ cargo vcpkg build
		Cloning https://github.com/microsoft/vcpkg
	cargo build
		Checkout rev 6d36e2a
	Compiling vcpkg
	Installing freetype harfbuzz[icu,graphite2]
	Compiling bzip2 (triplet x64-osx)
	Compiling zlib (triplet x64-osx)
	Compiling libpng (triplet x64-osx)
	Compiling freetype (triplet x64-osx)
	Compiling libiconv (triplet x64-osx)
	Compiling gettext (triplet x64-osx)
	Compiling graphite2 (triplet x64-osx)
	Compiling icu (triplet x64-osx)
	Compiling ragel (triplet x64-osx)
	Compiling harfbuzz (triplet x64-osx)
		Finished in 247.89s

```

## Build tectonic

Now we need to configure tectonic so it knows we are using `vcpkg` to build
the binary. This is done by setting the `TECTONIC_DEP_BACKEND` environment variable. `TECTONIC_DEP_BACKEND="vcpkg"` tells `tectonic` to use `vcpkg` instead of trying to resolve the libraries using `pkgconfig`.

For Linux and macOS, run `cargo build` with the appropriate environment variables, like this:

```sh
TECTONIC_DEP_BACKEND="vcpkg" cargo build --release
```

Or on Windows, like this for powershell:

```bat
$Env:TECTONIC_DEP_BACKEND="vcpkg"
$Env:RUSTFLAGS="-Ctarget-feature=+crt-static"
cargo build --release
```

Or like this for cmd:

```bat
set TECTONIC_DEP_BACKEND="vcpkg"
set RUSTFLAGS="-Ctarget-feature=+crt-static"
cargo build --release
```

This will take a couple of minutes but should eventually print something like:

```sh
    Finished release [optimized] target(s) in 3m 39s
```

The binary should be in `target/release`.

## Install gcc on macOS

If the `cargo vcpkg build` step fails on macOS, you may need to install another compiler. One way to do this is using [homebrew](https://brew.sh/) to install gcc.

```sh
brew install gcc
```
