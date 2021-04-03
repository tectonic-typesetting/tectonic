// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! The `xdvipdfmx` program from [XeTeX] as a reusable crate.
//!
//! [XeTeX]: http://xetex.sourceforge.net/

use std::{ffi::CString, time::SystemTime};
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError};
use tectonic_errors::prelude::*;

pub struct XdvipdfmxEngine {
    paper_spec: String,
    enable_compression: bool,
    deterministic_tags: bool,
    build_date: SystemTime,
}

impl Default for XdvipdfmxEngine {
    fn default() -> Self {
        XdvipdfmxEngine {
            paper_spec: "letter".to_owned(),
            enable_compression: true,
            deterministic_tags: false,
            build_date: SystemTime::UNIX_EPOCH,
        }
    }
}

impl XdvipdfmxEngine {
    pub fn enable_compression(&mut self, enable_compression: bool) -> &mut Self {
        self.enable_compression = enable_compression;
        self
    }

    pub fn enable_deterministic_tags(&mut self, flag: bool) -> &mut Self {
        self.deterministic_tags = flag;
        self
    }

    /// Sets the date and time used by the xdvipdfmx engine. This value is used
    /// as a source of entropy and is written to the output PDF. When expecting
    /// reproducible builds, this should be set to a static value, like its
    /// default value UNIX_EPOCH.
    pub fn build_date(&mut self, date: SystemTime) -> &mut Self {
        self.build_date = date;
        self
    }

    /// Set the initial paper size specification to be used.
    ///
    /// The default is `"letter"`, regardless of current locale.
    pub fn paper_spec(&mut self, paper_spec: String) -> &mut Self {
        self.paper_spec = paper_spec;
        self
    }

    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher,
        dvi: &str,
        pdf: &str,
    ) -> Result<i32> {
        let paperspec_str = atry!(
            CString::new(self.paper_spec.as_str());
            ["paper_spec may not contain internal NULs"]
        );

        let config = c_api::XdvipdfmxConfig {
            paperspec: paperspec_str.as_c_str().as_ptr(),
            enable_compression: if self.enable_compression { 1 } else { 0 },
            deterministic_tags: if self.deterministic_tags { 1 } else { 0 },
            build_date: self
                .build_date
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("invalid build date")
                .as_secs() as libc::time_t,
        };

        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        launcher.with_global_lock(|state| {
            let r = unsafe {
                c_api::tt_engine_xdvipdfmx_main(state, &config, cdvi.as_ptr(), cpdf.as_ptr())
            };

            match r {
                99 => Err(EngineAbortedError::new_abort_indicator().into()),
                x => Ok(x as i32),
            }
        })
    }
}

#[doc(hidden)]
pub mod c_api {
    use tectonic_bridge_core::CoreBridgeState;

    #[derive(Debug)]
    #[repr(C)]
    pub struct XdvipdfmxConfig {
        pub paperspec: *const libc::c_char,
        pub enable_compression: libc::c_uchar,
        pub deterministic_tags: libc::c_uchar,
        pub build_date: libc::time_t,
    }

    #[allow(improper_ctypes)] // for CoreBridgeState
    extern "C" {
        pub fn tt_engine_xdvipdfmx_main(
            api: &mut CoreBridgeState,
            cfg: &XdvipdfmxConfig,
            dviname: *const libc::c_char,
            pdfname: *const libc::c_char,
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
