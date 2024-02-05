# tectonic_io_base 0.4.3 (2024-02-05)

- Once again, tidy up recent Clippy warnings (#1076, @CraftSpider).


# tectonic_io_base 0.4.2 (2023-05-18)

- Tidy up recent Clippy warnings.
- Update the `sha2` dependency to the 0.10 series (#1038, @CraftSpider)


# tectonic_io_base 0.4.1 (2022-10-03)

- Print a warning when absolute paths are accessed (#806, #911, @ralismark,
  @pkgw). Any such access represents an aspect of the build that won't
  necessarily be reproducible on other machines.


# tectonic_io_base 0.4.0 (2022-02-28)

- Implement `Seek` for `InputHandle` (#865, @pkgw)
- Fixes for the latest versions of Clippy


# tectonic_io_base 0.3.1 (2021-10-11)

- No code changes; fixing a couple of docstring typos.


# tectonic_io_base 0.3.0 (2021-06-15)

- Add new "abspath" methods to the IoProvider trait. We need a new API to
  generate proper SyncTeX output in the XeTeX engine, and this is the best
  approach that we could devise that does a good job of maintaining backwards
  compatibility. However, implementors of the IoProvider trait that delegate to
  inner implementations will need to make sure to explicitly implement the new
  methods in order to provide correct behavior (#762).
- Add a new `app_dirs` module for system-wide knowledge of per-user directories
  (@pkgw, #768). It's valuable to put this low in the dependency stack so that
  higher-level crates can just "know" where to go for per-user files such as the
  bundle cache.
- Correct some broken internal links in the docs.


# tectonic_io_base 0.2.0 (2021-06-03)

- BREAKING: use `&str` for TeX paths rather than `OsStr`. In principle this
  prevents users from asking the TeX engine to load up files whose names aren't
  expressible in Unicode, but that whole use case really meshes poorly with
  Tectonic's goal to provide a portable, uniform user experience. And using
  `str` just makes many parts of life much easier.
- Expose a new interface for TeX path normalization.
- If an engine requests to open a file from a filesystem provider, and that name
  exists but is a directory, pretend that it's not found. This is sensible behavior
  and prevents some hard-to-understand failures (#754)
- Add `FilesystemIo::root()` for users that want to query the root directory of
  a filesystem I/O provider.
- Work on the docs a bit


# tectonic_io_base 0.1.0 (2021-01-15)

Initial release: a new crate for basic Tectonic I/O types and traits.
