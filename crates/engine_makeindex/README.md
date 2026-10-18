# The `tectonic_engine_makeindex` crate

This crate provides a pure-Rust implementation of the `makeindex` index
processor used by [Tectonic](https://tectonic-typesetting.github.io/).

The initial implementation targets the standard `.idx`/`.ind` pipeline,
including explicit page ranges, encapsulators, nested subitems, and the
common `.ist` style-file options. It is a clean-room implementation based on
the documented makeindex input/output formats and the makeindex manual.

## Scope

Supported:

- `\indexentry{...}{...}` input, including `@`, `!`, `|`, quote and escape
  characters.
- Default and `nomencl.ist`-style `.ist` parsing for the common output
  directives.
- Sorting, page de-duplication, numeric page-range compression, and the
  standard `\begin{theindex}` output.
- Page encapsulators such as `\textbf{...}`, `see`, and explicit `|(` / `|)`
  ranges.

Not yet supported:

- `makeglossaries` / `bib2gls`, which are separate tools.
- Full makeindex `-g` German ordering and all page-number edge cases.
- Output line wrapping (`line_max` / `indent_space`).
- The `-p` starting-page option and `.mst` default style lookup.

These are tracked as follow-up work; the crate is structured so they can be
added without changing the main processing pipeline.
