# rc: minor bump

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
