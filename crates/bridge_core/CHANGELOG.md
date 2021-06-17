# rc: micro bump

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
