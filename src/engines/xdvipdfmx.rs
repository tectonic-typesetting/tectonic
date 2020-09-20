// src/engines/xdvipdfmx.rs -- Rustic interface to the xdvipdfmx translator.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString};
use std::time::SystemTime;

use super::{ExecutionState, IoEventBackend, TectonicBridgeApi};
use crate::errors::{ErrorKind, Result};
use crate::io::IoStack;
use crate::status::StatusBackend;

pub struct XdvipdfmxEngine {
    enable_compression: bool,
    deterministic_tags: bool,
    build_date: SystemTime,
}

impl XdvipdfmxEngine {
    pub fn new() -> XdvipdfmxEngine {
        XdvipdfmxEngine {
            enable_compression: true,
            deterministic_tags: false,
            build_date: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn with_compression(mut self, enable_compression: bool) -> Self {
        self.enable_compression = enable_compression;
        self
    }

    pub fn with_deterministic_tags(mut self, flag: bool) -> Self {
        self.deterministic_tags = flag;
        self
    }

    /// Sets the date and time used by the xdvipdfmx engine. This value is used
    /// as a source of entropy and is written to the output PDF. When expecting
    /// reproducible builds, this should be set to a static value, like its
    /// default value UNIX_EPOCH.
    pub fn with_date(mut self, date: SystemTime) -> Self {
        self.build_date = date;
        self
    }

    pub fn process(
        &mut self,
        io: &mut IoStack,
        events: &mut dyn IoEventBackend,
        status: &mut dyn StatusBackend,
        dvi: &str,
        pdf: &str,
    ) -> Result<i32> {
        let _guard = super::ENGINE_LOCK.lock().unwrap(); // until we're thread-safe ...

        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let mut state = ExecutionState::new(io, events, status);
        let bridge = TectonicBridgeApi::new(&mut state);

        unsafe {
            match super::dvipdfmx_simple_main(
                &bridge,
                cdvi.as_ptr(),
                cpdf.as_ptr(),
                self.enable_compression,
                self.deterministic_tags,
                self.build_date
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("invalid build date")
                    .as_secs() as libc::time_t,
            ) {
                99 => {
                    let ptr = super::tt_get_error_message();
                    let msg = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    Err(ErrorKind::Msg(msg).into())
                }
                x => Ok(x as i32),
            }
        }
    }
}

impl Default for XdvipdfmxEngine {
    fn default() -> Self {
        XdvipdfmxEngine::new()
    }
}
