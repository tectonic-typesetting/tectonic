# tectonic -X watch

Build the current document and rebuild it as input files change.

***This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref].***

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

#### Usage Synopsis

```sh
tectonic -X watch
  [--exec <execute>...] [-x <execute>...]
```

#### Remarks

This command builds the current document in the same fashion as [`tectonic -X
build`](./build.md), and then stays running and watches for changes to the input
files. It rebuilds the document when changes are detected.

#### Command-Line Options

The `--exec` option (or `-x` for short) configures the command used to run the
document build. The value of this option is appended to `tectonic -X` and
defaults to `build`. If you want to pass options to the build command, this is
the way to do so.
