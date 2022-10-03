# rc: minor bump

- Define a new TOML item, `shell_escape_cwd`, that can be used to specify the
  directory in which shell-escape state should be managed. The main expected use
  case is to set it to the TeX source directory, to make it possible to work
  around limitations in Tectonic’s encapsulated shell-escape support.


# tectonic_docmodel 0.1.2 (2022-02-28)

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
