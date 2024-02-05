# tectonic_bridge_harfbuzz 0.2.9 (2024-02-05)

- Update the vendored Harfbuzz to 8.3.0.


# tectonic_bridge_harfbuzz 0.2.8 (2023-05-22)

- Update the vendored Harfbuzz to 7.3.0.


# tectonic_bridge_harfbuzz 0.2.7 (2022-10-04)

- On macOS, build with the preprocessor directive
  `__ASSERT_MACROS_DEFINE_VERSIONS_WITHOUT_UNDERSCORES=0`, which should
  hopefully fix compile failures with old SDKs (#944, @pkgw).


# tectonic_bridge_harfbuzz 0.2.6 (2022-10-03)

- Update the vendored harfbuzz to 5.2.0.


# tectonic_bridge_harfbuzz 0.2.5 (2022-04-26)

- Update the vendored harfbuzz to 4.1.0.


# tectonic_bridge_harfbuzz 0.2.4 (2022-02-28)

- Update the vendored Harfbuzz to 3.0.0.


# tectonic_bridge_harfbuzz 0.2.3 (2021-10-11)

- Update the vendored Harfbuzz source to
- Fixes for Clippy 1.53.0


# tectonic_bridge_harfbuzz 0.2.2 (2021-06-07)

- Still no code changes
- 0.2.1 didn't publish to Crates.io because our package file was too big for it;
  we've eliminated some of the Harfbuzz support files that should fix this
  ([#781], [@pkgw])

[#781]: https://github.com/tectonic-typesetting/tectonic/pull/781
[@pkgw]: https://github.com/pkgw


# tectonic_bridge_harfbuzz 0.2.1 (2021-06-07)

- No code changes
- The 0.2.0 package didn't actually contain the vendored Harfbuzz source files,
  breaking `cargo` builds. We’ve modified the CI scripts to attempt to fix this
  ([#777], [@pkgw])

[#777]: https://github.com/tectonic-typesetting/tectonic/pull/777
[@pkgw]: https://github.com/pkgw


# tectonic_bridge_harfbuzz 0.2.0 (2021-06-03)

- Update the vendored Harfbuzz to 2.8.1.
- Fix up handling of how C/C++ header file paths are exported to dependent
  crates. This is a breaking change: we've moved from a single include directory
  to a list of them.


# tectonic_bridge_harfbuzz 0.1.0 (2021-01-15)

Initial release of Harfbuzz "bridge" crate for Tectonic. Includes the ability to
vendor Harfbuzz.
