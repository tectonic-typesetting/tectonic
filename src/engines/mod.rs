// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! Access to Tectonic’s processing backends.
//!
//! These backends subsume the functionality of programs such as `bibtex`,
//! `xetex`, and `xdvipdfmx`. This module is historical — the API for each of
//! these is defined in crates with names like `tectonic_engine_xetex`.

// Public sub-modules and reexports.

pub mod bibtex;
pub mod spx2html;
pub mod tex;
pub mod xdvipdfmx;

pub use self::{
    bibtex::BibtexEngine, spx2html::Spx2HtmlEngine, tex::TexEngine, xdvipdfmx::XdvipdfmxEngine,
};
