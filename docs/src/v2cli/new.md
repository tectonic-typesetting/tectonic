# tectonic -X new

Create a new Tectonic workspace.

**_This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref]._**

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

#### Usage Synopsis

```sh
tectonic -X new [path]
```

If `[path]` is unspecified, the workspace is created in the current directory.

#### Remarks

This command will create a bare-bones [Tectonic.toml][tectonic-toml] file in the
target directory. The project’s name will be initialized to the name of the
workspace directory.

[tectonic-toml]: ../ref/tectonic-toml.md

It will also create placeholder source files in a `src` subdirectory:
`index.tex`, `_preamble.tex`, and `_postamble.tex`. The default build command
will process these files in the expected order:

1. `src/_preamble.tex`
2. `src/index.tex`
3. `src/_postamble.tex`

The intention of this framework is to allow you to isolate the main content of
your document from the usual LaTeX boilerplate. There are no restrictions on
what kind of content may be placed in each file, though.

#### See Also

- [`tectonic -X init`](./init.md)
