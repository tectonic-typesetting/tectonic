// src/engines/tex.rs -- Rustic interface to the core TeX engine.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use libc;
use std::ffi::{CStr, CString};

use errors::{ErrorKind, Result};
use io::IOStack;
use super::{assign_global_state, c_api, ExecutionState};


pub enum OutputFormat {
    Xdv,
    Pdf,
}


pub enum TexResult {
    // The Errors possibility should only occur if halt_on_error_p is false --
    // otherwise, errors get upgraded to fatals. The fourth TeX "history"
    // option, "HISTORY_FATAL_ERROR" results in an Err result, not
    // Ok(TexResult).
    Spotless,
    Warnings,
    Errors,
}


pub struct TexEngine {
    // One day, the engine will hold its own state. For the time being,
    // though, it's just a proxy for the global constants in the C code.
}


impl TexEngine {
    pub fn new () -> TexEngine {
        TexEngine {}
    }

    pub fn set_output_format (&mut self, out_fmt: OutputFormat) {
        match out_fmt {
            OutputFormat::Xdv => unsafe { c_api::tt_set_int_variable(b"no_pdf_output\0".as_ptr(), 1); },
            OutputFormat::Pdf => {}
        };
    }

    pub fn set_halt_on_error_mode (&mut self, halt_on_error: bool) {
        let v = if halt_on_error { 1 } else { 0 };
        unsafe { c_api::tt_set_int_variable(b"halt_on_error_p\0".as_ptr(), v); }
    }

    // This function can't be generic across the IOProvider trait, for now,
    // since the global pointer that stashes the ExecutionState must have a
    // complete type.

    pub fn process_tex (&mut self, io: &mut IOStack, format_file_name: &str, input_file_name: &str) -> Result<TexResult> {
        let cformat = CString::new(format_file_name)?;
        let cinput = CString::new(input_file_name)?;

        let mut state = ExecutionState::new(io);

        unsafe {
            assign_global_state (&mut state, || {
                c_api::tt_misc_initialize(cformat.as_ptr());
                match c_api::tt_run_engine(cinput.as_ptr()) {
                    0 => Ok(TexResult::Spotless),
                    1 => Ok(TexResult::Warnings),
                    2 => Ok(TexResult::Errors),
                    3 => {
                        let ptr = c_api::tt_get_error_message();
                        let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                        Err(ErrorKind::TexError(msg).into())
                    },
                    x => Err(ErrorKind::TexError(format!("internal error: unexpected 'history' value {}", x)).into())
                }
            })
        }
    }

    pub fn process_xdvipdfmx (&mut self, io: &mut IOStack, dvi: &str, pdf: &str) -> Result<libc::c_int> {
        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let mut state = ExecutionState::new(io);

        unsafe {
            assign_global_state (&mut state, || {
                match c_api::dvipdfmx_simple_main(cdvi.as_ptr(), cpdf.as_ptr()) {
                    99 => {
                        let ptr = c_api::tt_get_error_message();
                        let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                        Err(ErrorKind::DpxError(msg).into())
                    },
                    x => Ok(x)
                }
            })
        }
    }
}
