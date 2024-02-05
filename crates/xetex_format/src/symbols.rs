// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! In this crate, a symbol is a number with an assigned name that is exposed to
//! the engine implementation as a C preprocessor `#define`.
//!
//! The XeTeX engine implementation uses many of these symbols, and many of
//! their values depend on the engine format version. This is because some
//! values ultimately derive from the number of commands, or the number of a
//! certain type of parameter, and those change as features are added to the
//! engine.

use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};
use tectonic_errors::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct SymbolTable {
    by_name: HashMap<String, isize>,
    grouped: BTreeMap<SymbolCategory, Vec<String>>,
}

macro_rules! define_categories {
    ($(#[doc = $doc:expr] $name:ident),+,) => {
        /// Different categories that symbols are grouped into.
        ///
        /// This is used to group the `#defines` emitted in the C header.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum SymbolCategory {
            $(
                #[doc = $doc]
                $name
            ),+
        }

        impl SymbolCategory {
            fn emit_header<W: Write>(&self, stream: &mut W) -> Result<()> {
                let s = match self {
                    $(
                        SymbolCategory::$name => $doc
                    ),+
                };
                Ok(writeln!(stream, "/*{} */\n", s)?)
            }
        }
    }
}

define_categories! {
    /// The format version associated with these symbols.
    FormatVersion,

    /// Parameters associated with the multiletter control string hash table.
    CsHash,

    /// Constants associated with "characters" (namely, Unicode Scalar Values).
    CharacterConstants,

    /// Fixed array sizes.
    FixedArrays,

    /// Offsets for the integer parameters.
    IntPars,

    /// Offsets for the dimensional parameters.
    DimenPars,

    /// Offsets for the glue parameters.
    GluePars,

    /// Offsets for the "local" parameters.
    LocalPars,

    /// Offsets for the e-TeX penalties parameters.
    EtexPenaltiesPars,

    /// Offsets in the equivalents table.
    Eqtb,

    /// Codes for core engine commands.
    Commands,

    /// Major modes of the engine.
    Modes,

    /// Math font sizes.
    MathFontSizes,

    /// Types of nodes that can occur in general lists.
    NodeTypes,

    /// Additional types of "noads" that can occur in math lists.
    MathNoadTypes,

    /// Subtypes for glue nodes.
    GlueNodeSubtypes,

    /// Subtypes for kern nodes.
    KernNodeSubtypes,

    /// Subtypes for math nodes.
    MathNodeSubtypes,

    /// Subtypes for math style nodes.
    StyleNodeSubtypes,

    /// Subtypes for math OP noads.
    OpNoadSubtypes,

    /// Subtypes for whatsit nodes.
    WhatsitNodeSubtypes,

    /// Subcommand codes for the ABOVE command.
    AboveCodes,

    /// Subcommand codes for box-related commands.
    BoxCodes,

    /// Subcommand codes for the CONVERT command.
    ConvertCodes,

    /// Subcommand codes for the EXTENSION command.
    ExtensionCodes,

    /// Subcommand codes for the FI_OR_ELSE command.
    FiOrElseCodes,

    /// Subcommand codes for the IF_TEST command.
    IfCodes,

    /// Subcommand codes for the LAST_ITEM command.
    LastItemCodes,

    /// Subcommand codes for the SET_INTERACTION command.
    InteractionModes,

    /// Subcommand codes for the SET_BOX_DIMEN command.
    SetBoxDimenCodes,

    /// Subcommand codes for the SHORTHAND_DEF command.
    ShorthandDefCodes,

    /// Subcommand codes for skip-related command.
    SkipCodes,

    /// Subcommand codes for the TAB_MARK and CAR_RET commands.
    TabCrCodes,

    /// Subcommand codes for the TOP_BOT_MARK command.
    TopBotMarkCodes,

    /// Subcommand codes for the XRAY command.
    XrayCodes,
}

impl SymbolTable {
    pub fn add<S: Into<String>>(
        &mut self,
        cat: SymbolCategory,
        name: S,
        value: isize,
    ) -> Result<()> {
        let name = name.into();

        if let Some(prev) = self.by_name.insert(name.clone(), value) {
            // We let identical values get re-inserted, mainly for NORMAL.
            ensure!(prev == value, format!("changed symbol name `{name}`"));
        } else {
            let group = self.grouped.entry(cat).or_default();
            group.push(name);
        }

        Ok(())
    }

    pub fn lookup(&self, name: &str) -> isize {
        *self.by_name.get(name).unwrap()
    }

    pub fn emit_c_header_stanza<W: Write>(&self, stream: &mut W) -> Result<()> {
        let mut first = true;

        for (cat, names) in &self.grouped {
            if first {
                first = false;
            } else {
                writeln!(stream)?;
            }

            cat.emit_header(stream)?;

            for name in names {
                let value = self.by_name.get(name).unwrap();
                writeln!(stream, "#define {name} {value} /* = 0x{value:x} */")?;
            }
        }

        Ok(())
    }
}

/// A trait for a type whose values can be expressed with a symbol in the symbol
/// table.
pub trait HasSymbol {
    /// Get the symbol associated with this value.
    fn symbol(&self) -> &'static str;
}

/// A trait for a type that declares associated symbols.
pub trait DeclaresSymbols {
    fn declare_symbols(symbols: &mut SymbolTable) -> Result<()>;
}
