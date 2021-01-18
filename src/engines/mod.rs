// src/engines/mod.rs -- interface to Tectonic engines written in C
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

//! Access to Tectonic’s processing backends.
//!
//! These backends subsume the functionality of programs such as `bibtex`,
//! `xetex`, and `xdvipdfmx`. The API for each of these is defined in a
//! sub-module with the corresponding name.
//!
//! Due to the way Rust's visibility rules work, this module contains a
//! substantial private API that defines the interface between Tectonic's Rust
//! code and the C/C++ code that the backends are (currently) implemented in.

use tectonic_bridge_core::CoreBridgeState;

// Public sub-modules and reexports.

pub mod bibtex;
pub mod spx2html;
pub mod tex;
pub mod xdvipdfmx;

pub use self::{
    bibtex::BibtexEngine, spx2html::Spx2HtmlEngine, tex::TexEngine, xdvipdfmx::XdvipdfmxEngine,
};

pub use tectonic_bridge_core::{IoEventBackend, NoopIoEventBackend};

// This silences the warning that ExecutionState is not FFI-safe. The C side only passes the
// pointer around and doesn't actually look into the struct, so we can ignore this warning.
#[allow(improper_ctypes)]
extern "C" {
    fn tt_xetex_set_int_variable(var_name: *const libc::c_char, value: libc::c_int) -> libc::c_int;

    #[allow(dead_code)] // currently unused
    fn tt_xetex_set_string_variable(
        var_name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;

    fn tex_simple_main(
        api: &mut CoreBridgeState,
        dump_name: *const libc::c_char,
        input_file_name: *const libc::c_char,
        build_date: libc::time_t,
    ) -> libc::c_int;

    fn dvipdfmx_simple_main(
        api: &mut CoreBridgeState,
        config: &xdvipdfmx::XdvipdfmxConfig,
        dviname: *const libc::c_char,
        pdfname: *const libc::c_char,
        enable_compression: bool,
        deterministic_tags: bool,
        build_date: libc::time_t,
    ) -> libc::c_int;
}
