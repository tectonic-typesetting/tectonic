# tectonic -X show

Display various useful pieces of information.

***This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref].***

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

The `show` subcommands are:

- [`tectonic -X show user-cache-dir`](#tectonic--x-show-user-cache-dir)

## tectonic -X show user-cache-dir

Print out the location of Tectonic’s default per-user cache directory.

#### Usage Synopsis

```sh
tectonic -X show user-cache-dir
```

#### Example

```sh
$ tectonic -X show user-cache-dir
/home/knuth/.cache/Tectonic # Unix

$ tectonic -X show user-cache-dir
/home/knuth/Library/Caches/Tectonic # macOS

$ tectonic -X show user-cache-dir
C:\Users\knuth\AppData\Local\TectonicProject\Tectonic # Windows
```
