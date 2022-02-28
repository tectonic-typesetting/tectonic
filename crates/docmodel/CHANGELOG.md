# rc: micro bump

- Define HTML options for build output (#865, @pkgw)
- Fixes for newer versions of Clippy


# tectonic_docmodel 0.1.1 (2021-10-11)

- Fix the error message given when a "V2" command is run outside of a Tectonic
  document workspace (#813, @ralismark)
- Fixes for Clippy >=1.53.0 (@pkgw)


# tectonic_docmodel 0.1.0 (2021-06-15)

This crate isolates the file formats used by the Tectonic “document model”,
primarily `Tectonic.toml`. This makes it possible to interact with these data
formats without needing to link in with the full Tectonic dependency stack.
