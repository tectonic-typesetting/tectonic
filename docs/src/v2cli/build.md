# tectonic -X build

Build the current document.

***This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref].***

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

#### Usage Synopsis

```sh
tectonic -X build
  [--keep-intermediates]
  [--keep-logs]
  [--only-cached]
  [--print]
  [--open]
  [--untrusted]
```

#### Remarks

This command builds the current document, as identified by searching for a
[Tectonic.toml][tectonic-toml] file in the current directory or one of its
parents. The build artifacts will be placed into the document’s build output
directory, which defaults to a subdirectory `build` of the document source
directory.

[tectonic-toml]: ../ref/tectonic-toml.md

#### Command-Line Options

The `--keep-intermediates` option (or `-k` for short) will cause the engine to
save intermediate files (such as `mydoc.aux` or `mydoc.bbl`) in the build output
directory. By default, these files are stored in memory but not actually written
to disk.

The `--keep-logs` option will cause the engine to save the main TeX log file
(`mydoc.log`) to disk. By default, this information is discarded.

The `--only-cached` option (or `-C` for short) will configure the engine to
refuse to connect to the network when searching for support files. This can be
useful if you’re working on a document in a context where the Internet is
unavailable (e.g., on an airplane). While the engine generally avoids
unnecessary network connections, referencing a new file in your document will
cause it to look for that file in the online support bundle.

The `--print` option (or `-p` for short) will cause the engine to print the
regular terminal output of the TeX engine. This output is similar to, but not
identical to, the contents of the log file. By default, this output is only
printed if the engine encounters a fatal error.

The `--open` option will open the built document using the system handler.

Use the `--untrusted` option if building untrusted content. This is not the
default because in most cases you *will* trust the document that you’re
building, probably because you have created it yourself, and it would be very
annoying to have to pass `--trusted` every time you build a document that uses
shell-escape. See the security discussion in the documentation of the
[compile](./compile.md) command for details. In actual usage, it would obviously
be easy to forget to use this option; in cases where untrusted inputs are a
genuine concern, we recommend setting the environment variable
`TECTONIC_UNTRUSTED_MODE` to a non-empty value. This has the same effect as the
`--untrusted` option. Note, however, that a hostile shell user can trivially
clear this variable.

The `--reproducible` option ensures a fully deterministic build environment.
The most notable difference is a static document build time (`\today`), which can
be configured explicitly by setting the `SOURCE_DATE_EPOCH` environment variable.
There's a few ways to break determinism (shell escape, reading from `/dev/urandom`),
but anything else (especially behaviour in TeXLive packages) is considered a bug.