# The `Tectonic.toml` File

**Starting with [the V2 interface][v2]**, the `Tectonic.toml` file defines a
Tectonic document.

[v2]: ./v2cli.md

## Contents

The `Tectonic.toml` file is expressed in, yes, [TOML] format. Allowed items in
the file are detailed below.

[TOML]: https://toml.io/

```toml
[doc]
name = <string>  # the document name
bundle = <url or filesystem path>  # the source of the TeX bundle
```

Unexpected items are not allowed.

## Items

### `doc.name`

The name of the document. This is distinct from the document title. This value
will be used to name output files, so it should be relatively short and
filesystem-friendly.

### `doc.bundle`

A string identifying the location of the “bundle” of TeX support files
underyling the processing of the document.

In most circumstances this value should be a URL. The `tectonic -X new` command
will populate this field with the current recommended default.

This field can also be a filesystem path, pointing to either a Zip-format bundle
or a directory of support files. This mode of operation is discouraged because
it limits reproducibility. URLs with a `file:` protocol are also treated
identically to filesystem paths.
