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

// Public sub-modules and reexports.

pub mod bibtex;
pub mod spx2html;
pub mod tex;
pub mod xdvipdfmx;

pub use self::{
    bibtex::BibtexEngine, spx2html::Spx2HtmlEngine, tex::TexEngine, xdvipdfmx::XdvipdfmxEngine,
};

pub use tectonic_bridge_core::{IoEventBackend, NoopIoEventBackend};
