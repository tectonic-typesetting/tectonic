# tectonic_mac_core 0.1.3 (2026-07-27)

Fixes:

- Fix `SIGBUS` on MacOS for any `\setmainfont` call ([#1345], [@nadimkobeissi])

Internal Improvements:

- Update to more recent dependencies and compiler toolchains ([#1361], [#1355])

[#1363]: https://github.com/tectonic-typesetting/tectonic/pull/1363

[#1361]: https://github.com/tectonic-typesetting/tectonic/pull/1361

[#1345]: https://github.com/tectonic-typesetting/tectonic/pull/1345

[@nadimkobeissi]: https://github.com/nadimkobeissi

# tectonic_mac_core 0.1.2 (2026-04-17)

- Fix SIGBUS crash on macOS arm64 for any `\setmainfont{}` call

Still at least one known issue with CFString methods, but it should only show up for weirder strings with null bytes or invalid UTF-8.

# tectonic_mac_core 0.1.1 (2026-04-11)

- Update mac_core for publish

# tectonic_mac_core 0.1.0 (2026-04-11)

A new bridge for use in the `tectonic_xetex_layout` rewrite to Rust. Exposes macOS framework APIs.

