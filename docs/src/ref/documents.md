# Documents

***This concept only applies to Tectonic’s [V2 interface][v2cli-ref]. It is not
relevant to [the original ("V1") interface][v1cli-ref].***

[v2cli-ref]: ./v2cli.md
[v1cli-ref]: ./v1cli.md

The fundamental unit of processing in Tectonic is the *document*. The main
purpose of Tectonic is to compile documents from their TeX source to one or more
output formats.


## Source structure

Every Tectonic document is defined by a [Tectonic.toml][tectonic-toml] file,
which is found at the root of its source tree. This directory is also the root
of the current Tectonic [workspace]. At the moment, "workspaces" and "documents"
are the same thing, but in the future it might become possible to define
multiple documents inside a single workspace.

[tectonic-toml]: ./tectonic-toml.md
[workspace]: ./workspaces.md

The TeX sources are stored in a `src` subdirectory of the document root.
Fresh workspaces will contain a file named `main.tex`, but this may be
configured in [Tectonic.toml][tectonic-toml]. The [`build` command][cli-build] will process these files in the order they're provided in the `inputs` array.

[cli-build]: ../v2cli/build.md


## Build structure

Build outputs are placed in the document’s build directory. By default, this is
a `build` subdirectory of the document root.
