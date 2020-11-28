# “V1” (Default) Command Line Interface

Tectonic is distributed as a single executable, `tectonic`, that is meant to be
invoked from the command line. We’re starting to refer to this program’s
command-line interface as the “V1” interface, because a new [“V2”
interface](./v2cli.md) is currently under development.

The V1 interface is “[rustc]-like”, offering a single primary workflow with lots
of options controlling its behavior. In comparison, the V2 interface is
“[cargo]-like”, with a variety of subcommands anchored around a [Tectonic.toml]
file defining a document to build.

[cargo]: https://doc.rust-lang.org/cargo/
[Tectonic.toml]: ./tectonic-toml.md
[rustc]: https://doc.rust-lang.org/rustc/command-line-arguments.html


## Current status

**The V1 interface is the default**. If you want to use the V2 interface, you
need to take special steps, as described in [its documentation](./v2cli.md).


## Basic usage

The V1 interface takes an input TeX file and compiles it. Basic usage is often
as simple as:

```sh
tectonic myfile.tex
```

This will compile the file and create `myfile.pdf` if nothing went wrong. You
can use an input filename of `-` to have Tectonic process standard input. (In
this case, the output file will be named `texput.pdf`.)


## Options

In the V1 interface there are a variety of options that control the engine’s
behavior. If you have Tectonic installed, you can view them with `tectonic
--help`.

The following are the available flags.

| Short | Full                      | Explanation                                                                                    |
|:------|:--------------------------|:-----------------------------------------------------------------------------------------------|
| `-b`  | `--bundle <PATH>`         | Use this Zip-format bundle file to find resource files instead of the default                  |
| `-c`  | `--chatter <LEVEL>`       | How much chatter to print when running [default: default]  [possible values: default, minimal] |
|       | `--format <PATH>`         | The name of the "format" file used to initialize the TeX engine [default: latex]               |
| `-h`  | `--help`                  | Prints help information                                                                        |
|       | `--hide <PATH>...`        | Tell the engine that no file at `<PATH>` exists, if it tries to read it                          |
| `-k`  | `--keep-intermediates`    | Keep the intermediate files generated during processing                                        |
|       | `--keep-logs`             | Keep the log files generated during processing                                                |
|       | `--makefile-rules <PATH>` | Write Makefile-format rules expressing the dependencies of this run to <PATH>                  |
| `-C`  | `--only-cached`           | Use only resource files cached locally                                                         |
| `-o`  | `--outdir <OUTDIR>`       | The directory in which to place output files [default: the directory containing INPUT]         |
|       | `--outfmt <FORMAT>`       | The kind of output to generate [default: pdf]  [possible values: pdf, html, xdv, aux, format]  |
|       | `--pass <PASS>`           | Which engines to run [default: default]  [possible values: default, tex, bibtex_first]         |
| `-p`  | `--print`                 | Print the engine's chatter during processing                                                   |
| `-r`  | `--reruns <COUNT>`        | Rerun the TeX engine exactly this many times after the first                                   |
|       | `--synctex`               | Generate SyncTeX data                                                                          |
| `-V`  | `--version`               | Prints version information                                                                     |
| `-w`  | `--web-bundle <URL>`      | Use this URL find resource files instead of the default                                        |
