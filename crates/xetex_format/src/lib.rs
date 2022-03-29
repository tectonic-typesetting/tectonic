// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Introspection into the internal data structures of the Tectonic/XeTeX TeX
//! engine, with decoding of "format" files.
//!
//! There are currently two main entrypoints to this crate. To introspect the
//! various internal structures provided by the engine, you can create an
//! [`engine::Engine`] instance. One of its main capabilities is that it can
//! emit a C header defining the various magic constants needed for the engine
//! implementation:
//!
//! ```rust
//! let engine = tectonic_xetex_format::engine::Engine::default();
//! engine.emit_c_header(std::io::stdout()).unwrap();
//! ```
//!
//! You can also parse a TeX "format file" into a [`format::Format`] struct to
//! examine saved engine state. This functionality isn't yet fully implemented,
//! but many of the key pieces are present:
//!
//! ```no_run
//! use std::{io::Read, fs::File};
//! use tectonic_errors::prelude::*;
//! use tectonic_xetex_format::format::Format;
//!
//! # fn main() -> Result<()> {
//! let mut file = File::open("path-to-format-file.fmt")?;
//! let mut data = Vec::new();
//! file.read_to_end(&mut data)?;
//! let format = Format::parse(&data[..])?;
//! # Ok(())
//! # }
//! ```
//!
//! The intention is to add enough infrastructure so that all saved macros and
//! control strings can be decoded. On Linux systems, Tectonic's auto-generated
//! format files are saved in the `~/.cache/Tectonic/formats/` directory.

/// A type for format file version numbers.
///
/// This is `usize`. Version numbers increment monotonically as engine commands
/// and primitives evolve
pub type FormatVersion = usize;

/// The latest format version number supported by this version of the crate.
pub const LATEST_VERSION: FormatVersion = 32;

mod parseutils;

pub mod base;
pub mod catcodes;
pub mod commands;
pub mod cshash;
pub mod dimenpars;
pub mod engine;
pub mod enums;
pub mod eqtb;
pub mod etexpenalties;
pub mod format;
pub mod gluepars;
pub mod intpars;
pub mod locals;
pub mod mem;
pub mod stringtable;
pub mod symbols;
pub mod tokenlist;
