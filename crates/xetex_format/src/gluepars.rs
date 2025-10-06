// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Glue parameters defined by the engine.

use std::io::Write;
use tectonic_errors::prelude::*;

use crate::{
    symbols::{SymbolCategory, SymbolTable},
    FormatVersion,
};

/// Different kinds of glue parameters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlueParKind {
    /// A regular glue parameter.
    Regular,

    /// A math glue parameter.
    Math,
}

/// Information about glue parameters.
#[derive(Clone, Copy, Debug)]
pub struct GluePar {
    /// The name of the parameter.
    name: &'static str,

    /// The kind of the parameter.
    kind: GlueParKind,

    /// The first format version in which the parameter was introduced.
    since: FormatVersion,
}

const GLUE_PARS: &[GluePar] = &[
    GluePar {
        name: "line_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "baseline_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "par_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "above_display_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "below_display_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "above_display_short_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "below_display_short_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "left_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "right_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "top_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "split_top_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "tab_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "space_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "xspace_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "par_fill_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "XeTeX_linebreak_skip",
        kind: GlueParKind::Regular,
        since: 0,
    },
    GluePar {
        name: "thin_mu_skip",
        kind: GlueParKind::Math,
        since: 0,
    },
    GluePar {
        name: "med_mu_skip",
        kind: GlueParKind::Math,
        since: 0,
    },
    GluePar {
        name: "thick_mu_skip",
        kind: GlueParKind::Math,
        since: 0,
    },
];

/// Get information about the glue parameters used in a specific engine format
/// version.
pub fn get_gluepars_for_version(
    version: FormatVersion,
    symbols: &mut SymbolTable,
) -> Result<Vec<GluePar>> {
    let mut r = Vec::new();
    let mut n = 0;

    for p in GLUE_PARS {
        if version >= p.since {
            r.push(*p);
            symbols.add(
                SymbolCategory::GluePars,
                format!("GLUE_PAR__{}", p.name.to_lowercase()),
                n,
            )?;
            n += 1;
        }
    }

    symbols.add(SymbolCategory::GluePars, "GLUE_PARS", n)?;
    Ok(r)
}

/// Emit initializers for gluepar primitives in the C header.
pub fn emit_c_header_primitives<W: Write>(pars: &[GluePar], mut stream: W) -> Result<()> {
    for par in pars {
        let cmd = match par.kind {
            GlueParKind::Regular => "ASSIGN_GLUE",
            GlueParKind::Math => "ASSIGN_MU_GLUE",
        };

        writeln!(
            stream,
            "    {{ \"{}\", {}, GLUE_BASE + GLUE_PAR__{}, xf_prim_init_none }}, \\",
            par.name.replace('_', ""),
            cmd,
            par.name.to_lowercase(),
        )?;
    }

    Ok(())
}
