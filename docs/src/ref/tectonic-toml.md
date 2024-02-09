# The `Tectonic.toml` File

**Starting with [the V2 interface][v2]**, the `Tectonic.toml` file defines a
Tectonic workspace.

[v2]: ./v2cli.md

## Contents

The `Tectonic.toml` file is written in the [TOML] format. Allowed items in
the file are detailed below.

[TOML]: https://toml.io/

```toml
[doc]
name = <string>  # the document name
bundle = <url or filesystem path>  # the source of the TeX bundle

# The doc.metadata table may contain arbitrary data.
# It does not affect Tectonic in any way.
[doc.metadata]
pubish = false
arr = [1, 2, [6, 7]]


# One (of possibly many) output specifications.
[[output]]

# This output's name. By default, build products for each output will be
# placed in the build directory under subdirectory with this name.
name = "output name"

# The output's type. Right now, only "pdf" is valid.
type = "pdf"

# The TeX "format" of preloaded macros to use when compiling the document.
# This is optional, with a default of "latex" (which corresponds to the
# standard LaTeX format). The exact set of formats that are supported will
# depend on the bundle that is being used.
tex_format = "latex"

# Whether the TeX “shell escape”, AKA `\write18`, mechanism is allowed.
# This is optional and defaults to false.
#
# Shell-escape is insecure, since it give the document access to your shell.
# It also is non-portable, because it requires your document to be built
# is run in an environment where a shell exists.
# Naturally, its use is strongly discouraged, but some packages depend on
# this feature.
shell_escape = false

# The working directory path to use for “shell escape”. The default is a
# temporary directory if `output.shell_escape` is true, else it's disabled.
# The path can be absolute or relative to the root file, but it must exist.
# Specifying this path automatically sets `output.shell_escape` to true.
# This is optional, and defaults to a temporary directory.
shell_escape_cwd = "string"

# The input files we'll use to build this document.
# Usually, this is a path relative to the `./src` directory.
#
# This may also be an array of paths, which are concatenated
# while building. You could, for example, define:
# inputs = ["preamble.tex", "main.tex"]
#
# Finally, you may include an "inline" document as follows:
# inputs = [
#   { inline = "\\documentclass[a4paper]{article}" },
#   "main.tex"
# ]
# This will insert "\documentclass[a4paper]{article}" before main.tex
# (with a newline), allowing you to set options without making a new file.
inputs = "main.tex"
```