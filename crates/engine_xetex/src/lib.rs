// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! The [XeTeX] program as a reusable crate.
//!
//! [XeTeX]: http://www.xetex.org/
//!
//! This crate provides the core TeX engine implementation used by [Tectonic].
//! However, in order to obtain the full Tectonic user experience, it must be
//! combined with a variety of other utilities: the `xdvipdfmx` engine, code to
//! fetch support files, and so on. Rather than using this crate directly you
//! should probably use the main [`tectonic`] crate, which combines all of these
//! pieces into a (semi) coherent whole.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/
//! [`tectonic`]: https://docs.rs/tectonic/

// TODO: the internal interface we're using here is pretty janky. The bibtex
// engine has a nicer approach that we should probably start using.

use std::{ffi::CString, time::SystemTime};
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError};
use tectonic_errors::prelude::*;

/// A serial number describing the detailed binary layout of the TeX “format
/// files” used by this crate. This number will occasionally increment,
/// indicating that the format file structure has changed. There is no provision
/// for partial forwards or backwards compatibility: if the number changes, you
/// need to regenerate your format files. If you’re generating format files, you
/// should munge this serial number in the filename, or something along those
/// lines, to make sure that when the engine is updated you don’t attempt to
/// reuse old files.
//
// DEVELOPER NOTE: if you change this, rerun cbindgen! This value is exported
// into the C/C++ code as a #define.
pub const FORMAT_SERIAL: u32 = 32;

/// A possible outcome from a (Xe)TeX engine invocation.
///
/// The classic TeX implementation provides a fourth outcome: “fatal error”. In
/// Tectonic, this outcome is represented as an `Err` result rather than a
/// [`TexOutcome`].
///
/// The `Errors` possibility will only occur if the `halt_on_error` engine
/// option is false: if it’s true, errors get upgraded to fatals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TexOutcome {
    /// Nothing bad happened.
    Spotless,

    /// Warnings were issued by the TeX engine. Note that, due to the ways that
    /// people are used to using TeX, warnings are *extremely* common in the
    /// wild. It’s rare to find a real document that *doesn’t* compile with
    /// warnings.
    Warnings,

    /// Errors were issued by the TeX engine. Note that, in TeX terminology,
    /// errors are not necessarily *fatal* errors: the engine will try extremely
    /// hard to proceed when it encounters them. It is not uncommon to find TeX
    /// documents in the wild that produce errors.
    Errors,
}

/// A struct for invoking the (Xe)TeX engine.
///
/// This struct has a fairly straightforward “builder” interface: you create it,
/// apply any settings that you wish, and eventually run the
/// [`process()`](Self::process) method.
///
/// Due to constraints of the gnarly C/C++ code underlying the engine
/// implementation, only one engine may run at once in one process. The engine
/// execution framework uses a global mutex to ensure that this is the case.
/// This restriction applies not only to the [`TexEngine`] type but to *all*
/// Tectonic engines. I.e., you can't run this engine and the BibTeX engine at
/// the same time.
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
    /// Configure whether the engine will halt on errors.
    ///
    /// The default setting is true. If false, the engine will plunge on ahead
    /// in the face of all but the most catastrophic problems. It’s really quite
    /// impressive!
    pub fn halt_on_error_mode(&mut self, halt_on_error: bool) -> &mut Self {
        self.halt_on_error = halt_on_error;
        self
    }

    /// Configure the engine to run in "initex" mode, in which it generates a
    /// "format" file that serializes the engine state rather than a PDF
    /// document. The default is false.
    pub fn initex_mode(&mut self, initex: bool) -> &mut Self {
        self.initex_mode = initex;
        self
    }

    /// Configure the engine to produce SyncTeX data.
    ///
    /// The default is false.
    pub fn synctex(&mut self, synctex_enabled: bool) -> &mut Self {
        self.synctex_enabled = synctex_enabled;
        self
    }

    /// Configure the engine to use “semantic pagination”.
    ///
    /// **Important:** this mode is essentially unimplemented.
    ///
    /// The goal of this mode is to set up the engine to create HTML-friendly
    /// output by altering how paragraphs and pages are constructed. When this
    /// mode is activated, the engine output type changes from XDV to SPX
    /// (although the two formats are quite similar).
    ///
    /// The default is false.
    pub fn semantic_pagination(&mut self, enabled: bool) -> &mut Self {
        self.semantic_pagination_enabled = enabled;
        self
    }

    /// Configure whether the "shell escape" TeX feature is enabled.
    ///
    /// The default is false.
    pub fn shell_escape(&mut self, shell_escape_enabled: bool) -> &mut Self {
        self.shell_escape_enabled = shell_escape_enabled;
        self
    }

    /// Sets the date and time used by the TeX engine. This affects things like
    /// LaTeX's \today command.
    ///
    /// The default vaue is the Unix epoch, so you should almost always override
    /// this setting. If you are aiming to achieve reproducible builds, you will
    /// need a way to fix this parameter from one engine invocation to the next.
    pub fn build_date(&mut self, date: SystemTime) -> &mut Self {
        self.build_date = date;
        self
    }

    /// Process a document using the current engine configuration.
    ///
    /// The *launcher* parameter gives overarching environmental context in
    /// which the engine will be run.
    ///
    /// The *format_file_name* is the name for the TeX “format file” giving
    /// preloaded engine state. It must be findable in the I/O stack, using the
    /// special hooks that are provided for handing format files, which allow
    /// updates to the file format to be handed (see [`FORMAT_SERIAL`]). If in
    /// “initex” mode, this parameter will be ignored.
    ///
    /// The *input_file_name* is used to name the “primary input file”. The I/O
    /// system has special hooks for opening this primary input, so be aware
    /// that this filename is *not* opened using the usual mechanisms. This
    /// setting affects some of the names used by the engine internally,
    /// including the name it uses to create its main output files. The
    /// traditional default value is `"texput"`.
    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher,
        format_file_name: &str,
        input_file_name: &str,
    ) -> Result<TexOutcome> {
        let cformat = CString::new(format_file_name)?;
        let cinput = CString::new(input_file_name)?;

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
                        .as_secs(),
                )
            };

            match r {
                0 => Ok(TexOutcome::Spotless),
                1 => Ok(TexOutcome::Warnings),
                2 => Ok(TexOutcome::Errors),
                3 => Err(EngineAbortedError::new_abort_indicator().into()),
                x => Err(anyhow!("internal error: unexpected 'history' value {}", x)),
            }
        })
    }
}

#[doc(hidden)]
pub mod c_api {
    // If you change the interfaces here, rerun cbindgen as described in the README!

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
            build_date: u64,
        ) -> libc::c_int;
    }
}

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_pdf_io as clipyrenamehack1;

    #[allow(unused_imports)]
    use tectonic_xetex_layout as clipyrenamehack2;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
