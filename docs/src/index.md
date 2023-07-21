# The Tectonic Typesetting System

Tectonic is a modernized, complete, self-contained
[TeX](https://en.wikipedia.org/wiki/TeX)/[LaTeX](https://www.latex-project.org/)
engine, powered by [XeTeX](http://xetex.sourceforge.net/) and
[TeXLive](https://www.tug.org/texlive/). This book aims to document the core
elements of the Tectonic software system.

Without further ado, we suggest you start with the [Introduction]!

[Introduction]: ./introduction/index.md


## Contributions are welcome!

This book is, clearly, a work in progress! Contributions of any kind are most
welcome — please see the discussion in
[GitHub issue #62](https://github.com/tectonic-typesetting/tectonic/issues/62)
for some ideas of things that should be documented here.

The documentation is written in [Markdown] (specifically, CommonMark using
[pulldown-cmark]) and rendered into HTML using [mdbook]. The source code lives
in the `docs/` subdirectory of [the main tectonic repository]. To make and view
changes, all you need to do is [install mdbook], then run the command:

```sh
$ mdbook serve
```

in the `docs/` directory.

[Markdown]: https://commonmark.org/
[pulldown-cmark]: https://crates.io/crates/pulldown-cmark
[mdbook]: https://rust-lang-nursery.github.io/mdBook/
[the main tectonic repository]: https://github.com/tectonic-typesetting/tectonic
[install mdbook]: https://github.com/rust-lang-nursery/mdBook#installation

Members of the Tectonic community are eager to help if you run into any issues —
please launch a discussion on [the Tectonic forum] if you’d like to get
involved!

[the Tectonic forum]: https://github.com/tectonic-typesetting/tectonic/discussions
