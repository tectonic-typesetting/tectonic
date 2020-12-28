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

[[output]]  # one or more output specifications
name = <string>  # the output's name
type = <"pdf">  # the output's type
tex_format = [string]  # optional, defaults to "latex": the TeX format to use
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

### `output`

A list of dictionaries defining different outputs to be created from the
document source.

### `output.name`

A name given to the output. By default, build products for each output will be
placed in the build directory, in a subdirectory with this name.

### `output.type`

The kind of output to create. Currently, the only allowed option is `"pdf"`,
which creates a [Portable Document Format][pdf] file.

[pdf]: https://en.wikipedia.org/wiki/PDF

### `output.tex_format`

The TeX “format” of preloaded macros to use when compiling the document. The
default is `"latex"`, corresponding to the standard LaTeX format. The exact set
of formats that are supported will depend on the bundle that is being used.
