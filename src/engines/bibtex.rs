// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

use tectonic_bridge_core::CoreBridgeLauncher;
use tectonic_engine_bibtex::{BibtexEngine as RealBibtexEngine, BibtexOutcome};

use super::tex::TexOutcome;
use crate::{errors::Result, unstable_opts::UnstableOptions};

#[derive(Default)]
pub struct BibtexEngine {}

impl BibtexEngine {
    pub fn new() -> BibtexEngine {
        Default::default()
    }

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
