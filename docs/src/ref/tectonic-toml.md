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
shell_escape = [bool]  # optional, defaults to false: whether "shell escape" (\write18) is allowed
preamble = [string] # optional, defaults to "_preamble.tex": the preamble file to use (within `src`)
index = [string] # optional, defaults to "index.tex": the index file to use (within `src`)
postamble = [string] # optional, defaults to "_postamble.tex": the postamble file to use (within `src`)
```

Unexpected items are not allowed.

## Items

### `doc.name`

The name of the document. This is distinct from the document title. This value
will be used to name output files, so it should be relatively short and
filesystem-friendly.

### `doc.bundle`

A string identifying the location of the “bundle” of TeX support files
underlying the processing of the document.

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

### `output.shell_escape`

Whether the TeX “shell escape”, AKA `\write18`, mechanism is allowed. The
default is false. Shell-escape is inherently insecure, because its usage
requires that text from the document compilation is passed directly to the
operating system shell. It also is inherently unportable, because it requires
that your document compilation is run in an environment where an operating
system shell exists and can be invoked. Its use is therefore strongly
discouraged, but some packages require it.

### `output.preamble`

The preamble file to build the document with for this output. This defaults to
`"_preamble.tex"` within the `src` directory. Typically this file will contain
document setup steps.

### `output.index`

The index file to build the document with for this output. This defaults to
`"index.tex"` within the `src` directory. Typically this file will contain
the body of the document.

### `output.postamble`

The postamble file to build the document with for this output. This defaults to
`"_postamble.tex"` within the `src` directory. Typically this file will contain
document closing steps.
