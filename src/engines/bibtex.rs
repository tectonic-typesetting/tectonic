// src/engines/bibtex.rs -- Rustic interface to the bibtex processor.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString};

use super::tex::TexResult;
use super::{ExecutionState, IoEventBackend, TectonicBridgeApi};
use crate::errors::{ErrorKind, Result};
use crate::io::IoStack;
use crate::status::StatusBackend;
use crate::unstable_opts::UnstableOptions;

#[no_mangle]
extern "C" {
    static mut min_crossrefs: i32;
}

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
        let _guard = super::ENGINE_LOCK.lock().unwrap(); // until we're thread-safe ...

        let caux = CString::new(aux)?;

        let mut state = ExecutionState::new(io, events, status);
        let bridge = TectonicBridgeApi::new(&mut state);

        if let Some(num) = unstables.min_crossrefs {
            unsafe { min_crossrefs = num }
        }

        unsafe {
            match super::bibtex_simple_main(&bridge, caux.as_ptr()) {
                0 => Ok(TexResult::Spotless),
                1 => Ok(TexResult::Warnings),
                2 => Ok(TexResult::Errors),
                3 => Err(ErrorKind::Msg("unspecified fatal bibtex error".into()).into()),
                99 => {
                    let ptr = super::tt_get_error_message();
                    let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    Err(ErrorKind::Msg(msg).into())
                }
                x => Err(ErrorKind::Msg(format!(
                    "internal error: unexpected 'history' value {}",
                    x
                ))
                .into()),
            }
        }
    }
}
