// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Dimensional parameters defined by the engine.

use std::io::Write;
use tectonic_errors::prelude::*;

use crate::{
    symbols::{SymbolCategory, SymbolTable},
    FormatVersion,
};

/// Information about dimensional parameters.
#[derive(Clone, Copy, Debug)]
pub struct DimenPar {
    /// The name of the parameter.
    name: &'static str,

    /// The first format version in which the parameter was introduced.
    since: FormatVersion,
}

const DIMEN_PARS: &[DimenPar] = &[
    DimenPar {
        name: "par_indent",
        since: 0,
    },
    DimenPar {
        name: "math_surround",
        since: 0,
    },
    DimenPar {
        name: "line_skip_limit",
        since: 0,
    },
    DimenPar {
        name: "hsize",
        since: 0,
    },
    DimenPar {
        name: "vsize",
        since: 0,
    },
    DimenPar {
        name: "max_depth",
        since: 0,
    },
    DimenPar {
        name: "split_max_depth",
        since: 0,
    },
    DimenPar {
        name: "box_max_depth",
        since: 0,
    },
    DimenPar {
        name: "hfuzz",
        since: 0,
    },
    DimenPar {
        name: "vfuzz",
        since: 0,
    },
    DimenPar {
        name: "delimiter_shortfall",
        since: 0,
    },
    DimenPar {
        name: "null_delimiter_space",
        since: 0,
    },
    DimenPar {
        name: "script_space",
        since: 0,
    },
    DimenPar {
        name: "pre_display_size",
        since: 0,
    },
    DimenPar {
        name: "display_width",
        since: 0,
    },
    DimenPar {
        name: "display_indent",
        since: 0,
    },
    DimenPar {
        name: "overfull_rule",
        since: 0,
    },
    DimenPar {
        name: "hang_indent",
        since: 0,
    },
    DimenPar {
        name: "h_offset",
        since: 0,
    },
    DimenPar {
        name: "v_offset",
        since: 0,
    },
    DimenPar {
        name: "emergency_stretch",
        since: 0,
    },
    DimenPar {
        name: "pdf_page_width",
        since: 0,
    },
    DimenPar {
        name: "pdf_page_height",
        since: 0,
    },
];

/// Get information about the dimension parameters used in a specific engine
/// format version.
pub fn get_dimenpars_for_version(
    version: FormatVersion,
    symbols: &mut SymbolTable,
) -> Result<Vec<DimenPar>> {
    let mut r = Vec::new();
    let mut n = 0;

    for p in DIMEN_PARS {
        if version >= p.since {
            r.push(*p);
            symbols.add(
                SymbolCategory::DimenPars,
                format!("DIMEN_PAR__{}", p.name.to_lowercase()),
                n,
            )?;
            n += 1;
        }
    }

    symbols.add(SymbolCategory::DimenPars, "DIMEN_PARS", n)?;
    Ok(r)
}

/// Emit initializers for dimenpar primitives in the C header.
pub fn emit_c_header_primitives<W: Write>(pars: &[DimenPar], mut stream: W) -> Result<()> {
    for par in pars {
        writeln!(
            stream,
            "    {{ \"{}\", ASSIGN_DIMEN, DIMEN_BASE + DIMEN_PAR__{}, xf_prim_init_none }}, \\",
            par.name.replace('_', ""),
            par.name.to_lowercase(),
        )?;
    }

    Ok(())
}
