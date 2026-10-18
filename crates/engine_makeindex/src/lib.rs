// Copyright 2025 the Tectonic Project
// Licensed under the MIT License.

//! A pure-Rust implementation of the `makeindex` index processor.
//!
//! This crate is intentionally independent of Tectonic's I/O stack: it takes
//! `.idx` bytes and an optional `.ist` style file, and returns the generated
//! `.ind` and `.ilg` contents. The Tectonic driver decides when to invoke it
//! and how to place the outputs in the processing session.
//!
//! The implementation is a clean-room effort based on the documented
//! makeindex input/output formats. It covers the standard LaTeX `\makeindex`
//! pipeline and the common `nomencl` style. See `README.md` for the current
//! scope and known follow-up work.

mod generator;
mod idx;
mod page;
mod sort;
mod style;

use anyhow::Result;

/// The engine used to run makeindex-style index processing.
#[derive(Debug, Default)]
pub struct MakeindexEngine;

/// The result of one makeindex run.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MakeindexOutcome {
    /// The generated `.ind` (or `.nls`) file contents.
    pub ind: Vec<u8>,

    /// The generated `.ilg` transcript.
    pub ilg: Vec<u8>,

    /// The number of entries accepted from the input.
    pub accepted: usize,

    /// The number of non-fatal warnings.
    pub warnings: usize,
}

impl MakeindexEngine {
    /// Create a new engine instance.
    pub fn new() -> Self {
        Self
    }

    /// Process an `.idx` file.
    ///
    /// `style` is the optional contents of an `.ist` file. If it is `None`,
    /// makeindex's default LaTeX style is used.
    pub fn process(&self, idx: &[u8], style: Option<&[u8]>) -> Result<MakeindexOutcome> {
        let style = style.map(style::Style::parse).unwrap_or_default();
        let (entries, mut warnings) = idx::parse(idx, &style);
        let accepted = entries.len();
        let generated = generator::generate(entries, &style);
        warnings.extend(generated.warnings);

        let ilg = format!(
            "This is makeindex, version 2.12 [tectonic].\n\
             Scanning input file ... done ({} entries accepted, {} warnings).\n\
             Sorting entries... done.\n\
             Output written in the index file.\n",
            accepted,
            warnings.len()
        );

        Ok(MakeindexOutcome {
            ind: generated.ind.into_bytes(),
            ilg: ilg.into_bytes(),
            accepted,
            warnings: warnings.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MakeindexEngine;

    #[test]
    fn processes_a_simple_index() {
        let input = b"\\indexentry{alpha}{1}\n\\indexentry{alpha!beta}{3}\n";
        let outcome = MakeindexEngine::new().process(input, None).unwrap();
        let text = String::from_utf8(outcome.ind).unwrap();

        assert_eq!(outcome.accepted, 2);
        assert!(text.contains("\\item alpha, 1"));
        assert!(text.contains("\\subitem beta, 3"));
    }

    #[test]
    fn processes_glossaries_style() {
        let style = br#"
actual '?'
keyword "\\glossaryentry"
preamble "\\begin{theglossary}\n"
postamble "\\end{theglossary}\n"
item_0 "\%\n"
delim_0 "{\\glossaryentrynumbers{\\relax "
delim_t "}}"
headings_flag 1
heading_prefix "\\glsgroupheading{"
heading_suffix "}"
group_skip "\\glsgroupskip\n"
"#;
        let input = b"\\glossaryentry{LaTeX?\\glossentry{latex}|setentrycounter[]{page}\"\\glsnumberformat}{1}\n";
        let outcome = MakeindexEngine::new().process(input, Some(style)).unwrap();
        let text = String::from_utf8(outcome.ind).unwrap();

        assert!(text.contains("\\glossentry{latex}"));
        assert!(text.contains("\\setentrycounter[]{page}\\glsnumberformat{1}"));
    }

    #[test]
    fn processes_nomencl_style() {
        let style = br#"
keyword "\\nomenclatureentry"
quote '%'
preamble "\n\\begin{thenomenclature}\n"
postamble "\n\\end{thenomenclature}\n"
delim_0 ""
"#;
        let input = b"\\nomenclatureentry{alpha}{1}\n";
        let outcome = MakeindexEngine::new().process(input, Some(style)).unwrap();
        let text = String::from_utf8(outcome.ind).unwrap();

        assert!(text.contains("\\begin{thenomenclature}"));
        assert!(text.contains("\\item alpha1"));
    }
}
