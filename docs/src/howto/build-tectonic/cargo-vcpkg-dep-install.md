# How To: Build Tectonic: Install Dependencies With cargo-vcpkg

A convenient, fairly cross-platform way to install Tectonic’s dependencies is
using the [cargo-vcpkg] tool. It will take care of setting up [vcpkg], providing
the needed dependencies, and informing Tectonic’s build system about how to find
them. It will, however, generally require that the dependencies be compiled from
scratch.

[cargo-vcpkg]: https://crates.io/crates/cargo-vcpkg
[vcpkg]: https://vcpkg.readthedocs.io/

First, install [cargo-vcpkg] if needed:

```sh
cargo install cargo-vcpkg
```

Then, in the Tectonic source tree, run the command to obtain all of the needed
dependencies:

```sh
cargo vcpkg build
```

Then set the `VCPKG_ROOT` environment variable to tell the build system where
the dependencies may be found. In a [bash] shell, a good command is:

```sh
export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
```

[bash]: https://www.gnu.org/software/bash/
