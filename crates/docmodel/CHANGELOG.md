# tectonic_docmodel 0.2.2 (2024-02-05)

- Add a `metadata` field to the Tectonic.toml file specification, allowing
  open-ended, user-defined metadata (#1120, @rm-dr). This is useful for custom
  tools that build on top of Tectonic, where you might want to have some custom
  piece of information about each document in a group.


# tectonic_docmodel 0.2.1 (2023-05-18)

- Update the `toml` dependency to the 0.7 series (#1038, @CraftSpider)
- Have `shell_escape_cwd` imply `shell_escape = true` (#966, @pkgw)


# tectonic_docmodel 0.2.0 (2022-10-03)

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
