// src/engines/xdvipdfmx.rs -- Rustic interface to the xdvipdfmx translator.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString};

use errors::{ErrorKind, Result};
use io::IoStack;
use status::termcolor::TermcolorStatusBackend;
use super::{assign_global_state, c_api, ExecutionState};


pub struct XdvipdfmxEngine {
}


impl XdvipdfmxEngine {
    pub fn new () -> XdvipdfmxEngine {
        XdvipdfmxEngine {}
    }

    pub fn process (&mut self, io: &mut IoStack, status: &mut TermcolorStatusBackend, dvi: &str, pdf: &str) -> Result<i32> {
        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let mut state = ExecutionState::new(io, status);

        unsafe {
            assign_global_state (&mut state, || {
                match c_api::dvipdfmx_simple_main(cdvi.as_ptr(), cpdf.as_ptr()) {
                    99 => {
                        let ptr = c_api::tt_get_error_message();
                        let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                        Err(ErrorKind::DpxError(msg).into())
                    },
                    x => Ok(x as i32)
                }
            })
        }
    }
}
