// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! The `xdvipdfmx` program from [XeTeX] as a reusable crate.
//!
//! [XeTeX]: http://xetex.sourceforge.net/

use std::{
    ffi::{CStr, CString},
    time::SystemTime,
};
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError, IoEventBackend};
use tectonic_errors::prelude::*;
use tectonic_io_base::stack::IoStack;
use tectonic_status_base::StatusBackend;

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
        paperspec: Option<&str>,
    ) -> Result<i32> {
        // This conversion is probably way too complex, because we need to convert String to
        // something which holds a CStr (which needs to be a local so it doesn't disappear). And
        // all of this happens in an Option.

        // Keep a local reference so the string doesn't get dropped too early
        let paperspec_str = paperspec.and_then(|s| CString::new(s).ok());

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

        launcher.with_global_lock(|state| {
            let r = unsafe {
                c_api::tt_engine_xdvipdfmx_main(
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
    }
}

impl Default for XdvipdfmxEngine {
    fn default() -> Self {
        XdvipdfmxEngine::new()
    }
}

#[doc(hidden)]
pub mod c_api {
    use tectonic_bridge_core::CoreBridgeState;

    #[allow(improper_ctypes)] // for CoreBridgeState
    extern "C" {
        pub fn tt_engine_xdvipdfmx_main(
            api: &mut CoreBridgeState,
            cfg: &super::XdvipdfmxConfig,
            dviname: *const libc::c_char,
            pdfname: *const libc::c_char,
            enable_compression: bool,
            deterministic_tags: bool,
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
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
