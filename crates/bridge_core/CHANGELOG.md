# tectonic_bridge_core 0.1.0 (2021-06-03)

This is the first release of the "core" bridge crate. It provides a baseline of
APIs for C/C++ code to interact with an underlying "driver" implemented in Rust.
Those APIs mainly revolve around basic I/O and diagnostics, although we do have
a specialized "system request" to implement the TeX shell-escape feature.
