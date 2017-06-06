// src/engines/tex.rs -- Rustic interface to the core TeX engine.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString};

use errors::{ErrorKind, Result};
use io::IoStack;
use status::StatusBackend;
use super::{IoEventBackend, ExecutionState, TectonicBridgeApi};


#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum TexResult {
    // The Errors possibility should only occur if halt_on_error_p is false --
    // otherwise, errors get upgraded to fatals. The fourth TeX "history"
    // option, "HISTORY_FATAL_ERROR" results in an Err result, not
    // Ok(TexResult).
    Spotless,
    Warnings,
    Errors,
}

#[derive(Debug)]
pub struct TexEngine {
    // One day, the engine will hold its own state. For the time being,
    // though, it's just a proxy for the global constants in the C code.

    halt_on_error: bool,
    initex_mode: bool,
    synctex_enabled: bool,
}

impl Default for TexEngine {
    fn default() -> Self {
        TexEngine {
            halt_on_error: true,
            initex_mode: false,
            synctex_enabled: false,
        }
    }
}

impl TexEngine {
    pub fn new () -> TexEngine {
        TexEngine::default()
    }

    pub fn halt_on_error_mode (&mut self, halt_on_error: bool) -> &mut Self {
        self.halt_on_error = halt_on_error;
        self
    }

    /// Configure the engine to run in "initex" mode, in which it generates a
    /// "format" file that serializes the engine state rather than a PDF
    /// document.
    pub fn initex_mode (&mut self, initex: bool) -> &mut Self {
        self.initex_mode = initex;
        self
    }

    /// Configure the engine to produce SyncTeX data.
    pub fn synctex (&mut self, synctex_enabled: bool) -> &mut Self {
        self.synctex_enabled = synctex_enabled;
        self
    }

    // This function can't be generic across the IoProvider trait, for now,
    // since the global pointer that stashes the ExecutionState must have a
    // complete type.

    pub fn process (&mut self, io: &mut IoStack,
                    events: &mut IoEventBackend,
                    status: &mut StatusBackend,
                    format_file_name: &str, input_file_name: &str) -> Result<TexResult> {
        let cformat = CString::new(format_file_name)?;
        let cinput = CString::new(input_file_name)?;

        let /*mut*/ state = ExecutionState::new(io, events, status);
        let bridge = TectonicBridgeApi::new(&state);

        // initialize globals
        let v = if self.halt_on_error { 1 } else { 0 };
        unsafe { super::tt_set_int_variable(b"halt_on_error_p\0".as_ptr(), v); }
        let v = if self.initex_mode { 1 } else { 0 };
        unsafe { super::tt_set_int_variable(b"in_initex_mode\0".as_ptr(), v); }
        let v = if self.synctex_enabled { 1 } else { 0 };
        unsafe { super::tt_set_int_variable(b"synctex_enabled\0".as_ptr(), v); }

        unsafe {
            match super::tex_simple_main(&bridge, cformat.as_ptr(), cinput.as_ptr()) {
                0 => Ok(TexResult::Spotless),
                1 => Ok(TexResult::Warnings),
                2 => Ok(TexResult::Errors),
                3 => {
                    let ptr = super::tt_get_error_message();
                    let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    Err(ErrorKind::Msg(msg).into())
                },
                x => Err(ErrorKind::Msg(format!("internal error: unexpected 'history' value {}", x)).into())
            }
        }
    }
}
