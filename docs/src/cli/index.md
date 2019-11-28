# Tectonic's Command Line Interface

Tectonic has many command line options.
If you have Tectonic installed, you can view them with `tectonic --help`.

The following are the available flags.

| Short | Full                      | Explanation                                                                                    |
|:------|:--------------------------|:-----------------------------------------------------------------------------------------------|
| `-h`  | `--help`                  | Prints help information                                                                        |
| `-k`  | `--keep-intermediates`    | Keep the intermediate files generated during processing                                        |
|       | `--keep-logs`             | Keep the log files generated during processing                                                |
| `-C`  | `--only-cached`           | Use only resource files cached locally                                                         |
| `-p`  | `--print`                 | Print the engine's chatter during processing                                                   |
|       | `--synctex`               | Generate SyncTeX data                                                                          |
| `-V`  | `--version`               | Prints version information                                                                     |

The following are the available options.

| Short | Full                      | Explanation                                                                                    |
|:------|:--------------------------|:-----------------------------------------------------------------------------------------------|
| `-b`  | `--bundle <PATH>`         | Use this Zip-format bundle file to find resource files instead of the default                  |
| `-c`  | `--chatter <LEVEL>`       | How much chatter to print when running [default: default]  [possible values: default, minimal] |
|       | `--format <PATH>`         | The name of the "format" file used to initialize the TeX engine [default: latex]               |
|       | `--hide <PATH>...`        | Tell the engine that no file at <PATH> exists, if it tries to read it                          |
|       | `--makefile-rules <PATH>` | Write Makefile-format rules expressing the dependencies of this run to <PATH>                  |
| `-o`  | `--outdir <OUTDIR>`       | The directory in which to place output files [default: the directory containing INPUT]         |
|       | `--outfmt <FORMAT>`       | The kind of output to generate [default: pdf]  [possible values: pdf, html, xdv, aux, format]  |
|       | `--pass <PASS>`           | Which engines to run [default: default]  [possible values: default, tex, bibtex_first]         |
| `-r`  | `--reruns <COUNT>`        | Rerun the TeX engine exactly this many times after the first                                   |
| `-w`  | `--web-bundle <URL>`      | Use this URL find resource files instead of the default                                        |

If you specify a path, make sure to put it into brackets when it contains spaces.

Then, after specifying flags and options, you have to provide the actual input to tectonic.
This can be a (path to a) file, or "-" to process the standard input stream.

In short, the usage is `tectonic [FLAGS] [OPTIONS] <INPUT>`.

An example would be `tectonic --synctex --reruns 0 -o ../out/ main.tex`.