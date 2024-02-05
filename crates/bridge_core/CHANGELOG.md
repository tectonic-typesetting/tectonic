# tectonic_bridge_core 0.4.1 (2024-02-05)

- Add a safety comment and fix some new Clippy complaints (#1076, @CraftSpider)


# tectonic_bridge_core 0.4.0 (2023-06-12)

- Add new APIs, `CoreBridgeLauncher::with_expose_absolute_paths` and
  `CoreBridgeLauncher::with_mtime_override` (#1036, @Mrmaxmeier). These help
  enable the new `-Z deterministic-mode` mode.


# tectonic_bridge_core 0.3.2 (2023-05-18)

- Bump the `md-5` dep to the 0.10 series (#1038, @CraftSpider)
- Tidy up recent Clippy warnings.


# tectonic_bridge_core 0.3.1 (2022-10-03)

- Remove C's `time_t` from internal FFI APIs to avoid portability issues. This
  should avoid issues with Linux Musl builds.


# tectonic_bridge_core 0.3.0 (2021-10-11)

- Add `SecuritySettings::allow_extra_search_paths()` (#814, @ralismark).


# tectonic_bridge_core 0.2.2 (2021-06-17)

- Switch from running [cbindgen] at build time to having the developer run it
  manually. This really ought to fix the crate builds on docs.rs ([#788]), and
  should speed builds too.

[cbindgen]: https://github.com/eqrion/cbindgen
[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788


# tectonic_bridge_core 0.2.1 (2021-06-17)

- Attempt to fix crate builds on docs.rs — see [#788]. This works around an
  issue in Tectonic’s usage of [cbindgen] by configuring Cargo to operate in
  offline mode when building on docs.rs, which builds crates with network access
  turned off.

[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788
[cbindgen]: https://github.com/eqrion/cbindgen


# tectonic_bridge_core 0.2.0 (2021-06-15)

- Add a security infrastructure that gives a systematic way to control whether
  features that can be abused by untrusted inputs, like shell-escape, are
  enabled. The default is to disable all such features. Callers can request to
  allow their use, but we use a centralized approach that ensures that such
  requests will always be denied if the environment variable
  `$TECTONIC_UNTRUSTED_MODE` is set to a nonempty value (@pkgw, #787).
- Add a C API allowing us to expose the filesystem paths for just-opened
  inputs. This is needed for correct SyncTeX support (@hullanson, @pkgw, #762).


# tectonic_bridge_core 0.1.0 (2021-06-03)

This is the first release of the "core" bridge crate. It provides a baseline of
APIs for C/C++ code to interact with an underlying "driver" implemented in Rust.
Those APIs mainly revolve around basic I/O and diagnostics, although we do have
a specialized "system request" to implement the TeX shell-escape feature.
