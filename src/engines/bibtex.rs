// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Engine for invoking `bibtex`.

use tectonic_bridge_core::CoreBridgeLauncher;
use tectonic_engine_bibtex::{BibtexEngine as RealBibtexEngine, BibtexOutcome};

use super::tex::TexOutcome;
use crate::{errors::Result, unstable_opts::UnstableOptions};

/// A struct for invoking the `bibtex` engine.
///
/// This struct has a fairly straightforward "builder" interface: you create it,
/// apply any settings that you wish, and eventually run the
/// [`process()`](Self::process) method.
#[derive(Default)]
pub struct BibtexEngine {}

impl BibtexEngine {
    /// Create a new, default engine for running `bibtex`.
    pub fn new() -> BibtexEngine {
        Default::default()
    }

    /// Process a document using the current engine configuration.
    ///
    /// The *launcher* parameter gives overarching environmental context in
    /// which the engine will be run.
    ///
    /// The *aux* parameter gives the name of the "aux" file, created by the TeX
    /// engine, that BibTeX will process.
    ///
    /// The *unstables* parameter controls unstable options that may change the behavior of
    /// `bibtex`.
    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher,
        aux: &str,
        unstables: &UnstableOptions,
    ) -> Result<TexOutcome> {
        let mut real_engine = RealBibtexEngine::default();

        if let Some(x) = unstables.min_crossrefs {
            real_engine.min_crossrefs(x);
        }

        let real_outcome = real_engine.process(launcher, aux)?;

        match real_outcome {
            BibtexOutcome::Spotless => Ok(TexOutcome::Spotless),
            BibtexOutcome::Warnings => Ok(TexOutcome::Warnings),
            BibtexOutcome::Errors => Ok(TexOutcome::Errors),
        }
    }
}
