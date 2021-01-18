// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

use tectonic_bridge_core::{CoreBridgeLauncher, IoEventBackend};
use tectonic_engine_bibtex::{BibtexEngine as RealBibtexEngine, BibtexOutcome};

use super::tex::TexResult;
use crate::{errors::Result, io::IoStack, status::StatusBackend, unstable_opts::UnstableOptions};

#[derive(Default)]
pub struct BibtexEngine {}

impl BibtexEngine {
    pub fn new() -> BibtexEngine {
        Default::default()
    }

    pub fn process(
        &mut self,
        io: &mut IoStack,
        events: &mut dyn IoEventBackend,
        status: &mut dyn StatusBackend,
        aux: &str,
        unstables: &UnstableOptions,
    ) -> Result<TexResult> {
        let mut real_engine = RealBibtexEngine::default();

        if let Some(x) = unstables.min_crossrefs {
            real_engine.min_crossrefs(x);
        }

        let mut launcher = CoreBridgeLauncher::new(io, events, status);
        let real_outcome = real_engine.process(&mut launcher, aux)?;

        match real_outcome {
            BibtexOutcome::Spotless => Ok(TexResult::Spotless),
            BibtexOutcome::Warnings => Ok(TexResult::Warnings),
            BibtexOutcome::Errors => Ok(TexResult::Errors),
        }
    }
}
