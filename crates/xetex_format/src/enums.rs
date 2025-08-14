// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! Enumerations within the engine.
//!
//! We divide these into two categories: "simple" enums that basically have
//! static behavior, and "dynamic" enums that have version-dependent variants
//! and/or numerical values.
//!
//! The dynamic enums can gain values across different engine format versions,
//! and the numerical values associated with different enumeration variants
//! might evolve in non-trivial ways. This framework sets up an infrastructure
//! to define "abstract" Rust enums for these dynamic types and "resolvers" that
//! map numerical values to the abstract enums in the way that is appropriate
//! for a given engine version.

use tectonic_errors::prelude::*;

use crate::{
    symbols::{DeclaresSymbols, SymbolTable},
    FormatVersion,
};

/// A trait for a type whose values may correspond to named TeX engine
/// primitives.
pub trait HasPrimitive {
    /// Get the symbol associated with this value.
    fn primitive(&self) -> Option<&'static str>;
}

pub mod dynamic;
pub mod simple;

pub fn initialize_enum_symbols(version: FormatVersion, symbols: &mut SymbolTable) -> Result<()> {
    simple::AboveCodes::declare_symbols(symbols)?;
    simple::CharacterConstants::declare_symbols(symbols)?;
    simple::FiOrElseCodes::declare_symbols(symbols)?;
    simple::GlueNodeSubtypes::declare_symbols(symbols)?;
    simple::IfCodes::declare_symbols(symbols)?;
    simple::InteractionModes::declare_symbols(symbols)?;
    simple::KernNodeSubtypes::declare_symbols(symbols)?;
    simple::MathFontSizes::declare_symbols(symbols)?;
    simple::MathNodeSubtypes::declare_symbols(symbols)?;
    simple::NodeTypes::declare_symbols(symbols)?;
    simple::OpNoadSubtypes::declare_symbols(symbols)?;
    simple::SetBoxDimenCodes::declare_symbols(symbols)?;
    simple::ShorthandDefCodes::declare_symbols(symbols)?;
    simple::SkipCodes::declare_symbols(symbols)?;
    simple::StyleNodeSubtypes::declare_symbols(symbols)?;
    simple::TopBotMarkCodes::declare_symbols(symbols)?;
    simple::WhatsitNodeSubtypes::declare_symbols(symbols)?;
    simple::XrayCodes::declare_symbols(symbols)?;

    // Do some of the dynamic enums that declare important symbols that we need
    // early.
    dynamic::Modes::build_resolver(version, symbols)?;
    dynamic::MathNoadTypes::build_resolver(version, symbols)?;
    dynamic::TabCrCodes::build_resolver(version, symbols)?;

    Ok(())
}
