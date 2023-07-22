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

If you’re building on Windows, you’ll likely want to make sure that your
[`RUSTFLAGS`] variable includes a `+crt-static` [target feature] and set the
`VCPKGRS_TRIPLET` variable to `x64-windows-static-release`. This is a custom
[vcpkg triplet] provided by Tectonic’s build system (in the directory
`dist/vcpkg-triplets`) that is automatically activated by its [cargo-vcpkg]
integration. If you don’t use [cargo-vcpkg], the default triplet is
`x64-windows-static` if the `+crt-static` feature is activated, or
`x64-windows-static-md` if it is not.

If you’ve done the full vcpkg install, you might as well build with [an external
Harfbuzz][external-harfbuzz]. Therefore a full Windows build invocation —
launched from bash — might look like this:

[`RUSTFLAGS`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[target feature]: https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html
[vcpkg triplet]: https://vcpkg.readthedocs.io/en/latest/users/triplets/
[external-harfbuzz]: ./index.md#choose-cargo-features

```sh
cargo vcpkg build
export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
export RUSTFLAGS='-Ctarget-feature=+crt-static'  # Windows only
export VCPKGRS_TRIPLET='x64-windows-static-release'  # Windows only
export TECTONIC_DEP_BACKEND=vcpkg
cargo build --features external-harfbuzz
```

Note that if you are going to run additional commands such as `cargo test`,
you’re going to need to ensure that the same environment variables *and feature
flags* are used consistently.
