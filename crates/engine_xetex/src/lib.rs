// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! The [XeTeX] program as a reusable crate.
//!
//! [XeTeX]: http://www.xetex.org/

use std::{ffi::CString, time::SystemTime};
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError, IoEventBackend};
use tectonic_errors::prelude::*;
use tectonic_io_base::stack::IoStack;
use tectonic_status_base::StatusBackend;

pub const FORMAT_SERIAL: u32 = 29;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    semantic_pagination_enabled: bool,
    shell_escape_enabled: bool,
    build_date: SystemTime,
}

impl Default for TexEngine {
    fn default() -> Self {
        TexEngine {
            halt_on_error: true,
            initex_mode: false,
            synctex_enabled: false,
            semantic_pagination_enabled: false,
            shell_escape_enabled: false,
            build_date: SystemTime::UNIX_EPOCH,
        }
    }
}

impl TexEngine {
    pub fn new() -> TexEngine {
        TexEngine::default()
    }

    pub fn halt_on_error_mode(&mut self, halt_on_error: bool) -> &mut Self {
        self.halt_on_error = halt_on_error;
        self
    }

    /// Configure the engine to run in "initex" mode, in which it generates a
    /// "format" file that serializes the engine state rather than a PDF
    /// document.
    pub fn initex_mode(&mut self, initex: bool) -> &mut Self {
        self.initex_mode = initex;
        self
    }

    /// Configure the engine to produce SyncTeX data.
    pub fn synctex(&mut self, synctex_enabled: bool) -> &mut Self {
        self.synctex_enabled = synctex_enabled;
        self
    }

    /// Configure the engine to use “semantic pagination”.
    ///
    /// In this mode, the TeX page builder is not run, and top-level boxes are
    /// output vertically as they are created. The output file format changes
    /// from XDV to SPX (which is admittedly quite similar). "Page breaks" can
    /// be inserted explicitly in the document, but they only have semantic
    /// (organizational) meaning, rather than affecting the document
    /// rendering.
    ///
    /// This is an essential component of the HTML output process.
    pub fn semantic_pagination(&mut self, enabled: bool) -> &mut Self {
        self.semantic_pagination_enabled = enabled;
        self
    }

    /// Configure whether the "shell escape" TeX feature is enabled.
    pub fn shell_escape(&mut self, shell_escape_enabled: bool) -> &mut Self {
        self.shell_escape_enabled = shell_escape_enabled;
        self
    }

    /// Sets the date and time used by the TeX engine. This affects things like
    /// LaTeX's \today command. When expecting reproducible builds, this should
    /// be set to a static value, like its default value UNIX_EPOCH.
    pub fn build_date(&mut self, date: SystemTime) -> &mut Self {
        self.build_date = date;
        self
    }

    // This function can't be generic across the IoProvider trait, for now,
    // since the global pointer that stashes the ExecutionState must have a
    // complete type.

    pub fn process(
        &mut self,
        io: &mut IoStack,
        events: &mut dyn IoEventBackend,
        status: &mut dyn StatusBackend,
        format_file_name: &str,
        input_file_name: &str,
    ) -> Result<TexResult> {
        let cformat = CString::new(format_file_name)?;
        let cinput = CString::new(input_file_name)?;

        let mut launcher = CoreBridgeLauncher::new(io, events, status);

        launcher.with_global_lock(|state| {
            // Note that we have to do all of this setup while holding the
            // lock, because we're modifying static state variables.

            let v = if self.shell_escape_enabled { 1 } else { 0 };
            unsafe {
                c_api::tt_xetex_set_int_variable(b"shell_escape_enabled\0".as_ptr() as _, v);
            }

            let v = if self.halt_on_error { 1 } else { 0 };
            unsafe {
                c_api::tt_xetex_set_int_variable(b"halt_on_error_p\0".as_ptr() as _, v);
            }

            let v = if self.initex_mode { 1 } else { 0 };
            unsafe {
                c_api::tt_xetex_set_int_variable(b"in_initex_mode\0".as_ptr() as _, v);
            }

            let v = if self.synctex_enabled { 1 } else { 0 };
            unsafe {
                c_api::tt_xetex_set_int_variable(b"synctex_enabled\0".as_ptr() as _, v);
            }

            let v = if self.semantic_pagination_enabled {
                1
            } else {
                0
            };
            unsafe {
                c_api::tt_xetex_set_int_variable(b"semantic_pagination_enabled\0".as_ptr() as _, v);
            }

            let r = unsafe {
                c_api::tt_engine_xetex_main(
                    state,
                    cformat.as_ptr(),
                    cinput.as_ptr(),
                    self.build_date
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("invalid build date")
                        .as_secs() as libc::time_t,
                )
            };

            match r {
                0 => Ok(TexResult::Spotless),
                1 => Ok(TexResult::Warnings),
                2 => Ok(TexResult::Errors),
                3 => Err(EngineAbortedError::new_abort_indicator().into()),
                x => Err(anyhow!("internal error: unexpected 'history' value {}", x)),
            }
        })
    }
}

#[doc(hidden)]
pub mod c_api {
    use tectonic_bridge_core::CoreBridgeState;

    #[allow(improper_ctypes)] // for CoreBridgeState
    extern "C" {
        pub fn tt_xetex_set_int_variable(
            var_name: *const libc::c_char,
            value: libc::c_int,
        ) -> libc::c_int;

        pub fn tt_engine_xetex_main(
            api: &mut CoreBridgeState,
            dump_name: *const libc::c_char,
            input_file_name: *const libc::c_char,
            build_date: libc::time_t,
        ) -> libc::c_int;
    }
}

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_pdf_io;

    #[allow(unused_imports)]
    #[allow(clippy::single_component_path_imports)]
    use tectonic_xetex_layout;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
