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

<!-- Keep options alphabetized -->

```sh
tectonic -X compile  # full form
  [--bundle <file_path>] [-b <file_path>]
  [--format <path>] [-f <path>]
  [--hide <hide_path>...]
  [--keep-intermediates] [-k]
  [--keep-logs]
  [--makefile-rules <dest_path>]
  [--only-cached] [-C]
  [--outdir <outdir>] [-o <outdir>]
  [--outfmt <format>]
  [--pass <pass>]
  [--print] [-p]
  [--reruns <count>] [-r <count>]
  [--synctex]
  [--untrusted]
  [--web-bundle <url>] [-w <url>]
  [-Z <option>...]
  <input>
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

##### Security

By default, the document is compiled in a “trusted” mode. This means that the
calling user can request to enable certain engine features that could raise
security concerns if used with untrusted input: the classic example of this
being TeX’s “shell-escape” functionality. These features are *not* enabled by
default, but they can be enabled on the command line; in the case of
shell-escape, this is done with `-Z shell-escape`.

If the command-line argument `--untrusted` is provided, these features cannot be
enabled, regardless of other settings such as `-Z shell-escape`. So if you are
going to process untrusted input in a command-line script, as long as you make
sure that `--untrusted` is provided, the known-dangerous features will be
disabled.

Furthermore, if the environment variable `TECTONIC_UNTRUSTED_MODE` is set to a
non-empty value, Tectonic will behave as if `--untrusted` were specified,
regardless of the actual command-line arguments. Setting this variable can
provide a modest extra layer of protection if the Tectonic engine is being run
outside of its CLI form. Keep in mind that untrusted shell scripts and the like
can trivially defeat this by explicitly clearing the environment variable.

#### Options

The following are the available flags.

<!-- Keep alphabetized by full name: -->

| Short | Full                           | Explanation                                                                                            |
|:------|:-------------------------------|:-------------------------------------------------------------------------------------------------------|
| `-b`  | `--bundle <file_path>`         | Use this directory or Zip-format bundle file to find resource files instead of the default             |
| `-f`  | `--format <path>`              | The name of the “format” file used to initialize the TeX engine [default: `latex`]                     |
| `-h`  | `--help`                       | Prints help information                                                                                |
|       | `--hide <hide_path>...`        | Tell the engine that no file at `<hide_path>` exists, if it tries to read it                           |
| `-k`  | `--keep-intermediates`         | Keep the intermediate files generated during processing                                                |
|       | `--keep-logs`                  | Keep the log files generated during processing                                                         |
|       | `--makefile-rules <dest_path>` | Write Makefile-format rules expressing the dependencies of this run to `<dest_path>`                   |
| `-C`  | `--only-cached`                | Use only resource files cached locally                                                                 |
| `-o`  | `--outdir <outdir>`            | The directory in which to place output files [default: the directory containing `<input>`]             |
|       | `--outfmt <format>`            | The kind of output to generate [default: `pdf`]  [possible values: `pdf`, `html`, `xdv`, `aux`, `fmt`] |
|       | `--pass <pass>`                | Which engines to run [default: `default`]  [possible values: `default`, `tex`, `bibtex_first`]         |
| `-p`  | `--print`                      | Print the engine’s chatter during processing                                                           |
| `-r`  | `--reruns <count>`             | Rerun the TeX engine exactly this many times after the first                                           |
|       | `--synctex`                    | Generate SyncTeX data                                                                                  |
|       | `--untrusted`                  | Input is untrusted — disable all known-insecure features                                               |
| `-V`  | `--version`                    | Prints version information                                                                             |
| `-w`  | `--web-bundle <url>`           | Use this URL to find resource files instead of the default                                             |
| `-Z`  | `-Z <option>...`               | Unstable options. Pass `-Zhelp` to show a list                                                         |

#### Unstable options

The following unstable options may be available. As the name aims to indicate,
the set of unstable options is subject to change at any time.

<!-- Keep alphabetized: -->

| Expression                   | Explanation |
|:-----------------------------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `-Z help`                    | List all unstable options                                                                                                                                                                                                                                                                                  |
| `-Z continue-on-errors`      | Keep compiling even when severe errors occur                                                                                                                                                                                                                                                               |
| `-Z min-crossrefs=<num>`     | Equivalent to bibtex’s `-min-crossrefs` flag - “include after `<num>` crossrefs” [default: `2`]                                                                                                                                                                                                            |
| `-Z paper-size=<spec>`       | Change the initial paper size [default: `letter`]                                                                                                                                                                                                                                                          |
| `-Z search-path=<path>`      | Also look in `<path>` for files (unless `--untrusted` has been specified), like `TEXINPUTS`. Can be specified multiple times.                                                                                                                                                                              |
| `-Z shell-escape`            | Enable `\write18` (unless `--untrusted` has been specified)                                                                                                                                                                                                                                                |
| `-Z shell-escape-cwd=<path>` | Working directory to use for `\write18`. Use `$(pwd)` for same behaviour as most other engines (e.g. for relative paths in `\inputminted`). Implies `-Z shell-escape`                                                                                                                                      |
| `-Z deterministic-mode`      | Force a deterministic build environment. Note that setting `SOURCE_DATE_EPOCH` is usually sufficient for reproducible builds, and this option makes some extra functionality trade-offs. Specifically, deterministic mode breaks SyncTeX’s auxiliary files as they include and rely on absolute file paths |
