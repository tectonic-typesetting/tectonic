// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

#![recursion_limit = "1024"] // "error_chain can recurse deeply"

//! Tectonic is a complete
//! [TeX](https://www.tug.org/)/[LaTeX](https://www.latex-project.org/) engine
//! converted into a standalone library. It is derived from the
//! [XeTeX](http://xetex.sourceforge.net/) variant of TeX and uses the support
//! files packages by the [TeX Live](https://www.tug.org/texlive/) project.
//! Tectonic would not be possible without the hard work that has gone into
//! these projects.
//!
//! Because Tectonic is based on the XeTeX engine, it can take advantage of
//! the features of modern fonts (TrueType, OpenType, etc.), outputs directly
//! to the PDF file format, and supports Unicode inputs. Tectonic differs from
//! other TeX engines in the following ways:
//!
//! - Dependencies on environment variables and configuration files have been
//!   eliminated.
//! - All I/O is routed through pluggable backends. Support data can be fetched
//!   from a single “bundle” file, and the engine’s (copious) output can be
//!   hidden or postprocessed.
//! - The command-line frontend, `tectonic`, has a modernized user interface that
//!   never asks for user input.
//! - The frontend is just a thin shim over the Tectonic Rust crate, so that
//!   the full engine can be embedded anywhere you can run Rust code.
//!
//! Rust API documentation for Tectonic is currently very incomplete. As a
//! stopgap, please see [the source to the CLI
//! frontend](https://github.com/tectonic-typesetting/tectonic/blob/master/src/cli_driver.rs)
//! for a demonstration of how to run the engine.

extern crate aho_corasick;
extern crate app_dirs;
#[macro_use] extern crate error_chain;
extern crate flate2;
extern crate fs2;
extern crate hyper;
extern crate hyper_native_tls;
extern crate libc;
extern crate md5;
extern crate tempfile;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate sha2;
extern crate tectonic_xdv;
extern crate termcolor;
extern crate toml;
extern crate zip;

#[macro_use] pub mod status;
#[macro_use] pub mod errors;
pub mod config;
pub mod digest;
pub mod driver;
pub mod engines;
pub mod io;

pub use engines::bibtex::BibtexEngine;
pub use engines::spx2html::Spx2HtmlEngine;
pub use engines::tex::{TexEngine, TexResult};
pub use engines::xdvipdfmx::XdvipdfmxEngine;
pub use errors::{Error, ErrorKind, Result};

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {name: "Tectonic", author: "TectonicProject"};

const FORMAT_SERIAL: u32 = 28; // keep synchronized with tectonic/constants.h!!
