// src/engines/xdvipdfmx.rs -- Rustic interface to the xdvipdfmx translator.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::{CStr, CString};
use std::time::SystemTime;
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError, IoEventBackend};

use crate::errors::Result;
use crate::io::IoStack;
use crate::status::StatusBackend;
use crate::unstable_opts::UnstableOptions;

#[repr(C)]
pub struct XdvipdfmxConfig {
    paperspec: *const libc::c_char,
}

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
        unstables: &UnstableOptions,
    ) -> Result<i32> {
        // This conversion is probably way too complex, because we need to convert String to
        // something which holds a CStr (which needs to be a local so it doesn't disappear). And
        // all of this happens in an Option.

        // Keep a local reference so the string doesn't get dropped too early
        let paperspec_str = unstables
            .paper_size
            .as_ref()
            .and_then(|s| CString::new(s.clone()).ok());

        // We default to "letter" paper size by default
        let paperspec_default = CStr::from_bytes_with_nul(b"letter\0").unwrap();

        let config = XdvipdfmxConfig {
            paperspec: paperspec_str
                .as_ref()
                .map_or(paperspec_default.as_ptr(), |s| s.as_ptr()),
        };

        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        let mut launcher = CoreBridgeLauncher::new(io, events, status);

        launcher
            .with_global_lock(|state| {
                let r = unsafe {
                    super::dvipdfmx_simple_main(
                        state,
                        &config,
                        cdvi.as_ptr(),
                        cpdf.as_ptr(),
                        self.enable_compression,
                        self.deterministic_tags,
                        self.build_date
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .expect("invalid build date")
                            .as_secs() as libc::time_t,
                    )
                };

                match r {
                    99 => Err(EngineAbortedError::new_abort_indicator().into()),
                    x => Ok(x as i32),
                }
            })
            .map_err(|e| e.into())
    }
}

impl Default for XdvipdfmxEngine {
    fn default() -> Self {
        XdvipdfmxEngine::new()
    }
}
