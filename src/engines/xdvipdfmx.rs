// src/engines/xdvipdfmx.rs -- Rustic interface to the xdvipdfmx translator.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashMap;
use std::ffi::{CStr, CString, OsString};

use errors::{ErrorKind, Result};
use io::IoStack;
use status::StatusBackend;
use super::{assign_global_state, c_api, ExecutionState, FileSummary};


pub struct XdvipdfmxEngine {
}


impl XdvipdfmxEngine {
    pub fn new () -> XdvipdfmxEngine {
        XdvipdfmxEngine {}
    }

    pub fn process (&mut self, io: &mut IoStack,
                    summaries: Option<&mut HashMap<OsString, FileSummary>>,
                    status: &mut StatusBackend, dvi: &str, pdf: &str) -> Result<i32> {
        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let mut state = ExecutionState::new(io, summaries, status);

        unsafe {
            assign_global_state (&mut state, || {
                match c_api::dvipdfmx_simple_main(cdvi.as_ptr(), cpdf.as_ptr()) {
                    99 => {
                        let ptr = c_api::tt_get_error_message();
                        let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                        Err(ErrorKind::Msg(msg).into())
                    },
                    x => Ok(x as i32)
                }
            })
        }
    }
}
