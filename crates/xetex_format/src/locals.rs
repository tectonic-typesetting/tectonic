// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! "Local" parameters defined by the engine.
//!
//! These are mostly token lists, but there is also the `parshape` parameter
//! which is handled specially by the `SET_SHAPE` command. In non-vanilla TeX,
//! there are a few more parameters controlled by the `SET_SHAPE` command:
//! "e-TeX penalties" parameters defined in their own section of the eqtb.

use std::io::{Result, Write};

use super::FormatVersion;

/// Different kinds of "local" parameters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalParKind {
    /// A token list.
    TokenList,

    /// A "shape" parameter.
    Shape,
}

/// Information about "local" parameters.
#[derive(Clone, Copy, Debug)]
pub struct LocalPar {
    /// The name of the parameter.
    name: &'static str,

    /// The kind of the parameter.
    kind: LocalParKind,

    /// A custom name for the primitive associated with this parameter.
    custom_primitive_name: Option<&'static str>,

    /// The first format version in which the parameter was introduced.
    since: FormatVersion,
}

const LOCAL_PARS: &[LocalPar] = &[
    LocalPar {
        name: "par_shape",
        kind: LocalParKind::Shape,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "output_routine",
        kind: LocalParKind::TokenList,
        custom_primitive_name: Some("output"),
        since: 0,
    },
    LocalPar {
        name: "every_par",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_math",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_display",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_hbox",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_vbox",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_job",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_cr",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "err_help",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "every_eof",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "XeTeX_inter_char_toks",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
    LocalPar {
        name: "Tectonic_Coda_Tokens",
        kind: LocalParKind::TokenList,
        custom_primitive_name: None,
        since: 0,
    },
];

/// Get information about the local parameters used in the latest
/// engine format.
pub fn get_latest_local_pars() -> &'static [LocalPar] {
    LOCAL_PARS
}

/// Get information about the local parameters used in a specific
/// engine format version.
pub fn get_local_pars_for_version(version: FormatVersion) -> Vec<LocalPar> {
    let mut r = Vec::new();

    for p in LOCAL_PARS {
        if version >= p.since {
            r.push(*p)
        }
    }

    r
}

/// Emit C header information for the "locals" parameters.
pub fn emit_c_header_stanza<W: Write>(pars: &[LocalPar], mut stream: W) -> Result<()> {
    writeln!(stream, "/* \"Local\" parameters (mostly token lists) */\n")?;

    for (index, par) in pars.iter().enumerate() {
        writeln!(
            stream,
            "#define LOCAL__{} {}",
            par.name.to_lowercase(),
            index
        )?;
    }

    writeln!(stream, "#define NUM_LOCALS {}\n", pars.len())?;
    Ok(())
}

/// Emit initializers for gluepar primitives in the C header.
pub fn emit_c_header_primitives<W: Write>(pars: &[LocalPar], mut stream: W) -> Result<()> {
    for par in pars {
        let cmd = match par.kind {
            LocalParKind::Shape => "SET_SHAPE",
            LocalParKind::TokenList => "ASSIGN_TOKS",
        };

        let prim_name = match par.custom_primitive_name {
            Some(s) => s,
            None => par.name,
        };

        writeln!(
            stream,
            "    {{ \"{}\", {}, LOCAL_BASE + LOCAL__{} }}, \\",
            prim_name.replace("_", ""),
            cmd,
            par.name.to_lowercase(),
        )?;
    }

    Ok(())
}
