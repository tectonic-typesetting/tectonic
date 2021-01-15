# tectonic -X compile

Compile a standalone TeX document.

***This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref].***

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

#### Example

Generate `myfile.pdf` from `myfile.tex`:

```sh
tectonic -X compile myfile.tex
```

#### Usage Synopsis

```sh
tectonic -X compile  # full form
  [--bundle PATH] [-b PATH]
  [--chatter LEVEL] [-c LEVEL]
  [--color WHEN]
  [--format PATH] [-f]
  [--hide PATH...]
  [--keep-intermediates] [-k]
  [--keep-logs]
  [--makefile-rules PATH]
  [--only-cached] [-C]
  [--open]
  [--outdir DIR] [-o]
  [--outfmt FORMAT]
  [--pass PASS]
  [--print] [-p]
  [--reruns COUNT] [-r COUNT]
  [--synctex]
  [--web-bundle URL] [-w]
  [-Z UNSTABLE-OPTION]
  TEXPATH
```

#### Remarks

This command compiles a freestanding TeX document, outside of the Tectonic
[document][docs-ref] framework. It is virtually the same interface as the [“V1”
command-line interface][v1cli-ref], with lots of options. In comparison, the V2
interface is “[cargo]-like”, with a variety of subcommands anchored around a
[Tectonic.toml] file defining a document to build.

[docs-ref]: ../ref/documents.md
[cargo]: https://doc.rust-lang.org/cargo/
[Tectonic.toml]: ./tectonic-toml.md

Basic usage of this command is often as simple as:

```sh
tectonic -X compile myfile.tex
```

This will compile the file and create `myfile.pdf` if nothing went wrong. You
can use an input filename of `-` to have Tectonic process standard input. (In
this case, the output file will be named `texput.pdf`.)


#### Options

The following are the available flags.

| Short | Full                      | Explanation                                                                                    |
|:------|:--------------------------|:-----------------------------------------------------------------------------------------------|
| `-b`  | `--bundle <PATH>`         | Use this Zip-format bundle file to find resource files instead of the default |
| `-c`  | `--chatter <LEVEL>`       | How much chatter to print when running. Possible values: `default`, `minimal` |
|       | `--color <WHEN>`          | When to colorize the program’s output: `always`, `auto`, or `never` |
|       | `--format <PATH>`         | The name of the "format" file used to initialize the TeX engine. Default: `latex` |
| `-h`  | `--help`                  | Prints help information |
|       | `--hide <PATH>...`        | Tell the engine that no file at `<PATH>` exists, if it tries to read it |
| `-k`  | `--keep-intermediates`    | Keep the intermediate files generated during processing |
|       | `--keep-logs`             | Keep the log files generated during processing |
|       | `--makefile-rules <PATH>` | Write Makefile-format rules expressing the dependencies of this run to `<PATH>` |
| `-C`  | `--only-cached`           | Use only resource files cached locally |
|       | `--open`                  | Open the output PDF after it is built |
| `-o`  | `--outdir <OUTDIR>`       | The directory in which to place output files. Default: the directory containing INPUT |
|       | `--outfmt <FORMAT>`       | The kind of output to generate. Possible values: `pdf` (the default), `html`, `xdv`, `aux`, `format` |
|       | `--pass <PASS>`           | Which engines to run. Possible values: `default`, `tex`, `bibtex_first` |
| `-p`  | `--print`                 | Print the engine's chatter during processing |
| `-r`  | `--reruns <COUNT>`        | Rerun the TeX engine exactly this many times after the first |
|       | `--synctex`               | Generate SyncTeX data |
| `-V`  | `--version`               | Prints version information |
| `-w`  | `--web-bundle <URL>`      | Use this URL find resource files instead of the default |
| `-Z`  | `-Z <UNSTABLE-OPTION>`    | Activate experimental “unstable” options |

#### Unstable options

The following unstable options may be available. As the name aims to indicate,
the set of unstable options is subject to change at any time.

| Expression               | Explanation |
|:-------------------------|:------------|
| `-Z help`                | List all unstable options |
| `-Z continue-on-errors`  | Keep compiling even when severe errors occur |
| `-Z min-crossrefs=<num>` | Equivalent to bibtex's `-min-crossrefs` flag. Default vaue: 2 |
| `-Z paper-size=<spec>`   | Change the initial paper size. Default: `letter` |

