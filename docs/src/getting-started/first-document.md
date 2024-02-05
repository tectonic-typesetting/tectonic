# Getting Started: Build Your First Document

Now that [you’ve installed Tectonic][install], let’s create and build your first
[document].

[install]: ./install.md
[document]: ../ref/documents.md

**Important:** *From here on out, this [Getting Started][gs-index] guide will
use what we call the [“V2” interface][v2cli] to the Tectonic program. The V2
interface coexists with, but has a fairly different approach than, the [“V1”
interface][v1cli]. We are gradually migrating from V1 to V2. Neither interface
(V1 or V2) is the same as the one exposed by classic TeX tools such as
`pdflatex`.*

[gs-index]: ./index.md
[v2cli]: ../ref/v2cli.md
[v1cli]: ../ref/v1cli.md


## Create a new document

The Tectonic [V2 interface][v2cli] has a “multitool” structure similar to that
of other powerful tools such as [git] and [cargo]. To create a new document, we
use a [`new`][cli-new] subcommand that looks like this:

```sh
$ tectonic -X new myfirstdoc
```

[git]: https://git-scm.com/
[cargo]: https://doc.rust-lang.org/cargo/
[cli-new]: ../v2cli/new.md

This will create a new [Tectonic workspace][workspace] directory named
`myfirstdoc` containing a file `Tectonic.toml` and a sub-directory named `src`.
Enter this new directory in your command prompt.

[workspace]: ../ref/workspaces.md

```sh
$ cd myfirstdoc
```

**Note:** *The `-X` flag activates the V2 interface. Don’t forget it! Eventually
it will become unnecessary and you’ll just be able to write `tectonic new`, but
that changeover hasn’t happened yet.*

If you’ve got an existing TeX file, you can process it in one-off fashion with:

```sh
$ tectonic -X compile myfile.tex
```

See [the `tectonic -X compile` documentation][cli-compile] for all of the options.

[cli-compile]: ../v2cli/compile.md


## Basic document source structure

The source code to your document is stored in the `src` subdirectory of your new
document. Check it out:

```sh
$ ls src
```

You’ll see three files that were created by the [`new`][cli-new] command:

- `_preamble.tex`
- `index.tex`
- `_postamble.tex`

These files are pre-populated with extremely basic contents following this
suggested source structure:

- The “preamble” file should contain all of your (La)TeX initialization
  boilerplate, up to and including the LaTeX `\begin{document}` command.
- The “index” file contains all of your actual document content, without any of
  the annoying boilerplate. When you create a new Tectonic document, it just
  contains the text `Hello, world.`
- The “postamble” file should contain all of your cleanup code, starting with
  the LaTeX `\end{document}` command. There will almost never need to be any
  other content in this file.

When Tectonic builds your document, it processes these files in the order listed
above, so all three of them need to be available. But the breakdown suggested
above is only a suggestion, nothing more. If you want all of your boilerplate
and content to be in a single file, we recommend putting it all in `index.tex`
and making your preamble and postamble empty.

The motivation for this separation is partially stylistic, but not entirely so.
In the future, we anticipate that there might be different ways to build the
same document that invoke different preamble or postamble contents.


## Building your document

To compile your document, run:

```sh
$ tectonic -X build
```

If you haven’t run Tectonic on your computer before, this command will take a
minute or two as it downloads the support files that it needs and generates the
LaTeX “format file” storing the default macro collection. Tectonic will [cache](#cache)
these files and avoid downloading them again. Test it out by running the build
again:

```sh
$ tectonic -X build
```

This time the command should finish much more quickly, with no messages about
downloading files. The output PDF document will be placed at the path
`build/default/default.pdf` relative to your document directory:

```sh
$ ls -l build/default/
```

If you’re familiar with traditional TeX engines, you’ll have noticed that
Tectonic’s “user experience” is substantially different from those engines:

1. Tectonic doesn’t print out the usual chatter — unless there’s an error.
2. Tectonic automatically reruns the TeX stage until its output stabilizes.
3. By default, Tectonic doesn’t write out intermediate files such as
   (`texput.aux`, `texput.log`).
4. You ought not have seen this yet, but if you make a mistake in your TeX,
   Tectonic will quit with an error message, rather than asking you to type `X2`
   or whatever.

We hope that you’ll agree that these changes make for a program that is much
more pleasant to use than the traditional tools.


## Cache

The location of the cache depends on your operating system. You can use the
[V2 Interface][v2cli-ref] to find the exact cache location on your machine
or take a look [at the implementation][user-cache-impl].

If you need to change the location of the cache, you can do that by setting
the environment variable `TECTONIC_CACHE_DIR` to the path of a directory.
We recommend leaving the cache location at the default unless there is a
compelling reason to change it.

[v2cli-ref]: ../ref/v2cli.md
[user-cache-impl]: https://docs.rs/tectonic_io_base/latest/tectonic_io_base/app_dirs/fn.ensure_user_cache_dir.html
