// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Integer parameters defined by the engine.

use std::io::Write;
use tectonic_errors::prelude::*;

use crate::{
    symbols::{SymbolCategory, SymbolTable},
    FormatVersion,
};

/// Information about the primitive associated with an integer parameter
#[derive(Clone, Copy, Debug)]
pub enum IntParPrimitiveKind {
    /// No primitive.
    None,

    /// Primitive with name base on the parameter in the usual way.
    Standard,

    /// Primitive with a different name.
    CustomName(&'static str),
}

/// Information about integer parameters.
#[derive(Clone, Copy, Debug)]
pub struct IntPar {
    /// The name of the parameter.
    name: &'static str,

    /// The kind of primitive associated with this parameter
    primitive_kind: IntParPrimitiveKind,

    /// The first format version in which the parameter was introduced.
    since: FormatVersion,
}

const INT_PARS: &[IntPar] = &[
    IntPar {
        name: "pretolerance",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tolerance",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "line_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "hyphen_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "ex_hyphen_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "club_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "widow_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "display_widow_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "broken_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "bin_op_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "rel_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "pre_display_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "post_display_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "inter_line_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "double_hyphen_demerits",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "final_hyphen_demerits",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "adj_demerits",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "mag",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "delimiter_factor",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "looseness",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "time",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "day",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "month",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "year",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "show_box_breadth",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "show_box_depth",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "hbadness",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "vbadness",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "pausing",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_online",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_macros",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_stats",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_paragraphs",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_pages",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_output",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_lost_chars",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_commands",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_restores",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "uc_hyph",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "output_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "max_dead_cycles",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "hang_after",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "floating_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "global_defs",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "cur_fam",
        primitive_kind: IntParPrimitiveKind::CustomName("fam"),
        since: 0,
    },
    IntPar {
        name: "escape_char",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "default_hyphen_char",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "default_skew_char",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "end_line_char",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "new_line_char",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "language",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "left_hyphen_min",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "right_hyphen_min",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "holding_inserts",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "error_context_lines",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "char_sub_def_min",
        primitive_kind: IntParPrimitiveKind::None,
        since: 0,
    },
    IntPar {
        name: "char_sub_def_max",
        primitive_kind: IntParPrimitiveKind::None,
        since: 0,
    },
    IntPar {
        name: "tracing_char_sub_def",
        primitive_kind: IntParPrimitiveKind::None,
        since: 0,
    },
    IntPar {
        name: "tracing_stack_levels",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 32,
    },
    IntPar {
        name: "tracing_assigns",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_groups",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_ifs",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_scan_tokens",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "tracing_nesting",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "pre_display_direction",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "last_line_fit",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "saving_vdiscards",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "saving_hyph_codes",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "suppress_fontnotfound_error",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "xetex_linebreak_locale",
        primitive_kind: IntParPrimitiveKind::None,
        since: 0,
    },
    IntPar {
        name: "XeTeX_linebreak_penalty",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "XeTeX_protrude_chars",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "texxet",
        primitive_kind: IntParPrimitiveKind::CustomName("TeXXeTstate"),
        since: 0,
    },
    IntPar {
        name: "xetex_dash_break",
        primitive_kind: IntParPrimitiveKind::CustomName("XeTeXdashbreakstate"),
        since: 0,
    },
    IntPar {
        name: "XeTeX_upwards",
        primitive_kind: IntParPrimitiveKind::CustomName("XeTeXupwardsmode"),
        since: 0,
    },
    IntPar {
        name: "XeTeX_use_glyph_metrics",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "XeTeX_inter_char_tokens",
        primitive_kind: IntParPrimitiveKind::CustomName("XeTeXinterchartokenstate"),
        since: 0,
    },
    IntPar {
        name: "XeTeX_input_normalization",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "XeTeX_default_input_mode",
        primitive_kind: IntParPrimitiveKind::None,
        since: 0,
    },
    IntPar {
        name: "XeTeX_default_input_encoding",
        primitive_kind: IntParPrimitiveKind::None,
        since: 0,
    },
    IntPar {
        name: "XeTeX_tracing_fonts",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "XeTeX_interword_space_shaping",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "XeTeX_generate_actual_text",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "XeTeX_hyphenatable_length",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "synctex",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
    IntPar {
        name: "pdfoutput",
        primitive_kind: IntParPrimitiveKind::Standard,
        since: 0,
    },
];

/// Get information about the integer parameters used in a specific engine
/// format version.
pub fn get_intpars_for_version(
    version: FormatVersion,
    symbols: &mut SymbolTable,
) -> Result<Vec<IntPar>> {
    let mut r = Vec::new();
    let mut n = 0;

    for p in INT_PARS {
        if version >= p.since {
            r.push(*p);
            symbols.add(
                SymbolCategory::IntPars,
                format!("INT_PAR__{}", p.name.to_lowercase()),
                n,
            )?;
            n += 1;
        }
    }

    symbols.add(SymbolCategory::IntPars, "INT_PARS", n)?;
    Ok(r)
}

/// Emit initializers for intpar primitives in the C header.
pub fn emit_c_header_primitives<W: Write>(pars: &[IntPar], mut stream: W) -> Result<()> {
    for par in pars {
        let (has_prim, prim_name) = match par.primitive_kind {
            IntParPrimitiveKind::None => (false, ""),
            IntParPrimitiveKind::Standard => (true, par.name),
            IntParPrimitiveKind::CustomName(s) => (true, s),
        };

        if has_prim {
            writeln!(
                stream,
                "    {{ \"{}\", ASSIGN_INT, INT_BASE + INT_PAR__{}, xf_prim_init_none }}, \\",
                prim_name.replace('_', ""),
                par.name.to_lowercase(),
            )?;
        }
    }

    Ok(())
}
