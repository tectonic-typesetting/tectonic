// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! The [bibtex] program as a reusable crate.
//!
//! [bibtex]: http://www.bibtex.org/

use std::ffi::CString;
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError};
use tectonic_errors::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BibtexOutcome {
    Spotless = 0,
    Warnings = 1,
    Errors = 2,
}

#[derive(Debug, Default)]
pub struct BibtexEngine {
    config: c_api::BibtexConfig,
}

impl BibtexEngine {
    pub fn min_crossrefs(&mut self, value: i32) -> &mut Self {
        self.config.min_crossrefs = value as libc::c_int;
        self
    }

    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher,
        aux: &str,
    ) -> Result<BibtexOutcome> {
        let caux = CString::new(aux)?;

        launcher.with_global_lock(|state| {
            let hist = unsafe { c_api::tt_engine_bibtex_main(state, &self.config, caux.as_ptr()) };

            match hist {
                c_api::History::Spotless => Ok(BibtexOutcome::Spotless),
                c_api::History::WarningIssued => Ok(BibtexOutcome::Warnings),
                c_api::History::ErrorIssued => Ok(BibtexOutcome::Errors),
                c_api::History::FatalError => Err(anyhow!("unspecified fatal bibtex error")),
                c_api::History::Aborted => Err(EngineAbortedError::new_abort_indicator().into()),
            }
        })
    }
}

#[doc(hidden)]
pub mod c_api {
    use tectonic_bridge_core::CoreBridgeState;

    /// cbindgen:rename-all=ScreamingSnakeCase
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[repr(C)]
    pub enum History {
        Spotless = 0,
        WarningIssued = 1,
        ErrorIssued = 2,
        FatalError = 3,
        Aborted = 4,
    }

    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct BibtexConfig {
        pub min_crossrefs: libc::c_int,
    }

    impl Default for BibtexConfig {
        fn default() -> Self {
            BibtexConfig { min_crossrefs: 2 }
        }
    }

    #[allow(improper_ctypes)] // for CoreBridgeState
    extern "C" {
        pub fn tt_engine_bibtex_main(
            api: &mut CoreBridgeState,
            cfg: &BibtexConfig,
            aux_name: *const libc::c_char,
        ) -> History;
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
