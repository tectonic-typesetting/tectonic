# Workspaces

***This concept only applies to Tectonic’s [V2 interface][v2cli-ref]. It is not
relevant to [the original (“V1”) interface][v1cli-ref].***

[v2cli-ref]: ./v2cli.md
[v1cli-ref]: ./v1cli.md

A Tectonic *workspace* is a directory tree for building documents. The top-level
directory of a workspace contains a [Tectonic.toml][tectonic-toml] file.

[tectonic-toml]: ./tectonic-toml.md

At the moment, a workspace contains exactly one [document], having its source
rooted in the workspace root. In the future, Tectonic may be extended so that a
workspace might contain more than one document.

[document]: ./documents.md
