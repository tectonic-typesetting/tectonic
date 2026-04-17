# rc: micro bump

- Fix SIGBUS crash on macOS arm64 for any `\setmainfont{}` call

Still at least one known issue with CFString methods, but it should only show up for weirder strings with null bytes or invalid UTF-8.

# tectonic_mac_core 0.1.1 (2026-04-11)

- Update mac_core for publish

# tectonic_mac_core 0.1.0 (2026-04-11)

A new bridge for use in the `tectonic_xetex_layout` rewrite to Rust. Exposes macOS framework APIs.

