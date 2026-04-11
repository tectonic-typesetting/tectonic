// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! e-TeX penalties parameters defined by the engine.
//!
//! These are modified using the `SET_SHAPE` command, which in plain TeX is only
//! used to modify the `parshape` parameter, which is a "local". All of the
//! other locals are token lists.

use std::io::Write;
use tectonic_errors::prelude::*;

use crate::{
    symbols::{SymbolCategory, SymbolTable},
    FormatVersion,
};

/// Information about e-TeX penalties parameters.
#[derive(Clone, Copy, Debug)]
pub struct EtexPenaltiesPar {
    /// The name of the parameter.
    name: &'static str,

    /// The first format version in which the parameter was introduced.
    since: FormatVersion,
}

const ETEX_PENALTIES_PARS: &[EtexPenaltiesPar] = &[
    EtexPenaltiesPar {
        name: "inter_line_penalties",
        since: 0,
    },
    EtexPenaltiesPar {
        name: "club_penalties",
        since: 0,
    },
    EtexPenaltiesPar {
        name: "widow_penalties",
        since: 0,
    },
    EtexPenaltiesPar {
        name: "display_widow_penalties",
        since: 0,
    },
];

/// Get information about the e-TeX penalties parameters used in a specific
/// engine format version.
pub fn get_etex_penalties_pars_for_version(
    version: FormatVersion,
    symbols: &mut SymbolTable,
) -> Result<Vec<EtexPenaltiesPar>> {
    let mut r = Vec::new();
    let mut n = 0;

    for p in ETEX_PENALTIES_PARS {
        if version >= p.since {
            r.push(*p);
            symbols.add(
                SymbolCategory::EtexPenaltiesPars,
                format!("ETEX_PENALTIES_PAR__{}", p.name.to_lowercase()),
                n,
            )?;
            n += 1;
        }
    }

    symbols.add(SymbolCategory::EtexPenaltiesPars, "NUM_ETEX_PENALTIES", n)?;
    Ok(r)
}

/// Emit initializers for gluepar primitives in the C header.
pub fn emit_c_header_primitives<W: Write>(pars: &[EtexPenaltiesPar], mut stream: W) -> Result<()> {
    for par in pars {
        writeln!(
            stream,
            "    {{ \"{}\", SET_SHAPE, ETEX_PEN_BASE + ETEX_PENALTIES_PAR__{}, xf_prim_init_none }}, \\",
            par.name.replace('_', ""),
            par.name.to_lowercase(),
        )?;
    }

    Ok(())
}
