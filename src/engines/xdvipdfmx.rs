// src/engines/xdvipdfmx.rs -- Rustic interface to the xdvipdfmx translator.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString};

use errors::{ErrorKind, Result};
use io::IoStack;
use status::StatusBackend;
use super::{IoEventBackend, ExecutionState, TectonicBridgeApi};


pub struct XdvipdfmxEngine {
}


impl XdvipdfmxEngine {
    pub fn new () -> XdvipdfmxEngine {
        XdvipdfmxEngine {}
    }

    pub fn process (&mut self, io: &mut IoStack,
                    events: &mut IoEventBackend,
                    status: &mut StatusBackend, dvi: &str, pdf: &str) -> Result<i32> {
        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let /*mut*/ state = ExecutionState::new(io, events, status);
        let bridge = TectonicBridgeApi::new(&state);

        unsafe {
            match super::dvipdfmx_simple_main(&bridge, cdvi.as_ptr(), cpdf.as_ptr()) {
                99 => {
                    let ptr = super::tt_get_error_message();
                    let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    Err(ErrorKind::Msg(msg).into())
                },
                x => Ok(x as i32)
            }
        }
    }
}
