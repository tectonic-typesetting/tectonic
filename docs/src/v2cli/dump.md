# tectonic -X dump

Run a partial document build and dump the contents of a TeX intermediate file to
standard output. This can be useful if you have an external tool that parses
such files, and you wish to integrate it into Tectonic-based authoring
workflows.

***This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref].***

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

#### Example

Generate and print the LaTeX `aux` file for the current document:

```sh
tectonic -X dump -s aux
```

This output can, for example, be piped into [bibtools] to generate a `.bib` file
that provides exactly the records needed for the document build.

[bibtools]: https://github.com/pkgw/bibtools/

#### Usage Synopsis

```sh
tectonic -X dump
  [--only-cached] [-C]
  [--profile <profile>] [-p <profile>]
  [--suffix] [-s]
  [--untrusted]
  <filename>
```

#### Remarks

This command runs a partial build of the current document, as identified by
searching for a [Tectonic.toml][tectonic-toml] file in the current directory or
one of its parents.

[tectonic-toml]: ../ref/tectonic-toml.md

The “partial build” consists of one pass of the TeX engine. Future versions of
this tool might gain options allowing you to specify different passes. This
command can be used to dump any file created by TeX during the build (so long
as it’s created on the first pass).

#### Command-Line Options

The `--only-cached` option (or `-C` for short) will configure the engine to
refuse to connect to the network when searching for support files. This can be
useful if you’re working on a document in a context where the Internet is
unavailable (e.g., on an airplane). While the engine generally avoids
unnecessary network connections, referencing a new file in your document will
cause it to look for that file in the online support bundle.

The `--profile` option (or `-p` for short) will select which document output
profile will be used for the build. If unspecified, the profile to use will be
effectively chosen at random.

If the `--suffix` (`-s`) argument is provided, the name of the dumped file
simply has to *end* with the content of the argument `<filename>`, rather than
match it exactly. Therefore `tectonic -X dump -s aux` will dump the LaTeX `aux`
file regardless of its complete name, and `tectonic -X dump -s log` will dump
the log file. If there happens to be more than one file whose name ends with
your specified suffix, they will *all* be dumped, in a pseudo-random order.

Use the `--untrusted` option if working with untrusted content. This is not the
default, because in most cases you *will* trust the document that you’re
building, probably because you have created it yourself, and it would be very
annoying to have to pass `--trusted` every time you dump a document that uses
shell-escape. See the security discussion in the documentation of the
[compile](./compile.md) command for details. In actual usage, it would obviously
be easy to forget to use this option; in cases where untrusted inputs are a
genuine concern, we recommend setting the environment variable
`TECTONIC_UNTRUSTED_MODE` to a non-empty value. This has the same effect as the
`--untrusted` option. Note, however, that a hostile shell user can trivially
clear this variable.
