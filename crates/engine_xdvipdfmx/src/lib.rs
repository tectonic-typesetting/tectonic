// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! The `xdvipdfmx` program from [XeTeX] as a reusable crate.
//!
//! [XeTeX]: http://xetex.sourceforge.net/
//!
//! The `xdvipdfmx` progam converts XeTeX "XDV" intermediate files into PDF
//! output files.
//!
//! This crate provides the `xdvipdfmx` implementation used by [Tectonic].
//! However, in order to obtain the full Tectonic user experience, it must be
//! combined with a variety of other utilities: the main XeTeX engine, code to
//! fetch support files, and so on. Rather than using this crate directly you
//! should probably use the main [`tectonic`] crate, which combines all of these
//! pieces into a (semi) coherent whole.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/
//! [`tectonic`]: https://docs.rs/tectonic/

use std::{ffi::CString, time::SystemTime};
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError};
use tectonic_errors::prelude::*;

/// A struct for invoking the `xdvipdfmx` engine.
///
/// This struct has a fairly straightforward “builder” interface: you create it,
/// apply any settings that you wish, and eventually run the
/// [`process()`](Self::process) method.
///
/// Due to constraints of the gnarly C/C++ code underlying the engine
/// implementation, only one engine may run at once in one process. The engine
/// execution framework uses a global mutex to ensure that this is the case.
/// This restriction applies not only to the [`XdvipdfmxEngine`] type but to
/// *all* Tectonic engines. I.e., you can't run this engine and the XeTeX engine
/// at the same time.
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
    /// Set whether compression will be enabled in the output PDF.
    ///
    /// The default value is true. You might want to set this to false to
    /// improve the reproducibility of your generated PDFs, since different
    /// environments may create different compressed outputs even if their
    /// inputs and algorithms are the same. If this is your interest,
    /// see also [`enable_deterministic_tags`](Self::enable_deterministic_tags).
    pub fn enable_compression(&mut self, enable_compression: bool) -> &mut Self {
        self.enable_compression = enable_compression;
        self
    }

    /// Set whether font tags will be generated deterministically.
    ///
    /// The default is false: the engine includes some random characters when
    /// creating font tags. Changing this to true helps create byte-for-byte
    /// reproducible PDF outputs.
    pub fn enable_deterministic_tags(&mut self, deterministic_tags: bool) -> &mut Self {
        self.deterministic_tags = deterministic_tags;
        self
    }

    /// Sets the build date embedded in the output artifacts
    ///
    /// The default value is the Unix epoch, which is almost certainly not what
    /// you want. This value is used as a source of entropy and is written to
    /// the output PDF.
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

    /// Run xdvipdfmx.
    ///
    /// The *launcher* parameter gives overarching environmental context in
    /// which the engine will be run.
    ///
    /// The *dvi* parameter gives the name of the DVI file, created by the TeX
    /// engine, that will be processed. In Tectonic this is actually an XDV
    /// file, containing extended features needed for XeTeX Unicode processing.
    ///
    /// The *pdf* parameter gives the name of the output PDF file to create.
    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher,
        dvi: &str,
        pdf: &str,
    ) -> Result<()> {
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
                .as_secs(),
        };

        let cdvi = CString::new(dvi)?;
        let cpdf = CString::new(pdf)?;

        launcher.with_global_lock(|state| {
            let r = unsafe {
                c_api::tt_engine_xdvipdfmx_main(state, &config, cdvi.as_ptr(), cpdf.as_ptr())
            };

            // At the moment, the only possible return codes are 0 and 99 (= abort).
            if r == 99 {
                Err(EngineAbortedError::new_abort_indicator().into())
            } else {
                Ok(())
            }
        })
    }
}

#[doc(hidden)]
pub mod c_api {
    // If you change the interfaces here, rerun cbindgen as described in the README!

    use tectonic_bridge_core::CoreBridgeState;

    #[derive(Debug)]
    #[repr(C)]
    pub struct XdvipdfmxConfig {
        pub paperspec: *const libc::c_char,
        pub enable_compression: libc::c_uchar,
        pub deterministic_tags: libc::c_uchar,
        pub build_date: u64,
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
    use tectonic_pdf_io as clipyrenamehack;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
