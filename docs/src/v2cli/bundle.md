# tectonic -X bundle

Commands relating to Tectonic’s “bundles” of support files.

***This is a [V2 CLI][v2cli-ref] command. For information on the original (“V1”
CLI), see [its reference page][v1cli-ref].***

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

The `bundle` subcommands are:

- [`tectonic -X bundle cat`](#tectonic--x-bundle-cat)
- [`tectonic -X bundle search`](#tectonic--x-bundle-search)


## tectonic -X bundle cat

Print out a file stored in the current document’s backing bundle.

#### Usage Synopsis

```sh
tectonic -X bundle cat <filename>
```

#### Example

```sh
$ tectonic -X bundle cat latex.ltx
%%
%% This is file `latex.ltx',
%% generated with the docstrip utility.
...
```

#### Remarks

If this command is run outside of a [document workspace](../ref/workspaces.md),
the system default bundle will be used.


## tectonic -X bundle search

Print out the names of files in the current document’s backing bundle,
potentially with filtering.

#### Usage Synopsis

```sh
tectonic -X bundle search [term]
```

#### Example

```sh
$ tectonic -X bundle search minted
minted1.sty
tcbminted.code.tex
minted.4ht
minted.sty
```

#### Remarks

If no term is specified, *all* of the files in the bundle are printed. The
ordering of those filenames is unspecified.

The default search method is to use simple substring matching. Other methods may
be added in the future, activated by additional options.

If this command is run outside of a [document workspace](../ref/workspaces.md),
the system default bundle will be used.
