// Copyright 2021-2022 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! The "equivalencies table".
//!
//! Its structure is:
//!
//! 1. Active characters (`active_base`, `NUMBER_USVS` entries)
//! 2. One-letter control sequences (`single_base`, `NUMBER_USVS`)
//! 3. The "null" control sequence (`null_cs`)
//! 4. The control-sequence hash table (`hash_base`, `hash_size` = 15000)
//! 5. Special frozen control sequences (`frozen_control_sequence_base`, 12
//!    entries)
//!    0. `frozen_protection`
//!    1. frozen \cr
//!    2. frozen \endgroup
//!    3. frozen \right
//!    4. frozen \fi
//!    5. frozen \endtemplate, `frozen_end_template`
//!    6. frozen \endtemplate again, `frozen_endv`
//!    7. frozen \relax
//!    8. frozen \endwrite
//!    9. frozen \notexpanded:
//!    10. frozen \special
//!    11. frozen \pdfprimitive
//! 6. Primitives (`prim_eqtb_base`, `prim_size` = 500)
//! 7. Frozen `\nullfont` (`frozen_null_font`)
//! 8. Other fonts (`font_id_base+1`, `max_fonts+1` = 9001)
//! 9. Dummy location for undefined control sequences
//!    (`undefined_control_sequence`)
//! 10. Glue parameters (`glue_base`, ~19 depending on engine version)
//! 11. "Skips" -- glue registers (`skip_base`, `NUMBER_REGS`)
//! 12. "Mu Skips" -- math glue registers (`mu_skip_base`, `NUMBER_REGS`)
//! 13. Locals -- `\parshape` and token list parameters (`local_base`, ~13 depending on engine)
//! 14. Token registers (`toks_base`, `NUMBER_REGS`)
//! 15. e-TeX penalties (`etex_pen_base`, ~4)
//! 16. Box registers (`box_base`, `NUMBER_REGS`)
//! 17. The current font (`cur_font_loc`)
//! 18. Math fonts (`math_font_base`, `NUMBER_MATH_FONTS` = 3 * 256)
//! 19. Character category codes (`cat_code_base`, `NUMBER_USVS`)
//! 20. Character lower-case codes (`lc_code_base`, `NUMBER_USVS`)
//! 21. Character upper-case codes (`uc_code_base`, `NUMBER_USVS`)
//! 22. Character space-factor codes (`sf_code_base`, `NUMBER_USVS`)
//! 23. Character math codes (`math_code_base`, `NUMBER_USVS`)
//! 24. Character substitution codes (`char_sub_code_base`, `NUMBER_USVS`).
//!     This is an MLTeX vestige and is removed in format versions >= 33.
//! 25. Integer parameters (`int_base`, ~85)
//! 26. Integer registers (`count_base`, `NUMBER_REGS`)
//! 27. Delimeter codes (`del_code_base`, `NUMBER_USVS`)
//! 28. Length parameters (`dimen_base`, ~23)
//! 29. Length registers (`scaled_base`, `NUMBER_REGS`)
//! 30. Extra control-sequence hash table (`eqtb_top + 1`, `hash_extra`)
//!
//! Total `eqtb_size` is:
//!
//! - `NUMBER_USVS` * 9
//! - `NUMBER_REGS` * 6
//! - hash_size
//! - 16 one-offs
//! - prim_size
//! - max_fonts + 1
//! - number of engine glue parameters
//! - number of engine token lists
//! - number of engine e-TeX penalties
//! - number of engine integer parameters
//! - number of engine dimension parameters
//! - `NUMBER_MATH_FONTS`
//! - Minus one since eqtb_size is the highest address
//!
//! `eqtb_top` is `eqtb_size + hash_extra` and the total addressed size of the
//! array is `eqtb_top + 1`.

use nom::{multi::count, number::complete::be_u8, IResult, Parser};
use tectonic_errors::prelude::*;

use crate::{
    base::{self, SIZEOF_MEMORY_WORD, TEX_NULL},
    commands::CommandCode,
    engine::Engine,
    parseutils,
    symbols::{SymbolCategory, SymbolTable},
    FormatVersion,
};

#[derive(Debug)]
pub struct EquivalenciesTable {
    eqtb: Vec<u8>,
}

pub type EqtbPointer = i32;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EqtbEntry {
    pub level: i16,
    pub ty: CommandCode,
    pub value: i32,
}

impl EquivalenciesTable {
    pub(crate) fn parse<'a>(
        input: &'a [u8],
        engine: &Engine,
        hash_high: i32,
    ) -> IResult<&'a [u8], Self> {
        let eqtb_size = engine.symbols.lookup("EQTB_SIZE") as usize;
        let eqtb_top = engine.symbols.lookup("EQTB_TOP") as usize;
        let undefined_control_sequence =
            engine.symbols.lookup("UNDEFINED_CONTROL_SEQUENCE") as EqtbPointer;
        let undefined_cs_cmd = engine.symbols.lookup("UNDEFINED_CS") as CommandCode;

        let mut eqtb = vec![0; (eqtb_top + 1) * SIZEOF_MEMORY_WORD];

        write_eqtb_type(&mut eqtb[..], undefined_control_sequence, undefined_cs_cmd);
        write_eqtb_value(&mut eqtb[..], undefined_control_sequence, TEX_NULL);
        write_eqtb_level(&mut eqtb[..], undefined_control_sequence, 0);

        let ucs_ofs = undefined_control_sequence as usize * base::SIZEOF_MEMORY_WORD;

        for x in (eqtb_size + 1)..(eqtb_top + 1) {
            eqtb.copy_within(
                ucs_ofs..ucs_ofs + SIZEOF_MEMORY_WORD,
                x * SIZEOF_MEMORY_WORD,
            );
        }

        let mut k = 1; // really `active_base`, but that will never change
        let mut input = input;

        loop {
            // "The table of equivalents usually contains repeated information, so
            // we dump it in compressed form: The sequence of $n + 2$ values $(n,
            // x_1, \ldots, x_n, m)$ in the format file represents $n + m$
            // consecutive entries of |eqtb|, with |m| extra copies of $x_n$, namely
            // $(x_1, \ldots, x_n, x_n, \ldots, x_n)$"

            let (ii, n) = parseutils::ranged_be_i32(1, (eqtb_size + 1 - k) as i32)(input)?;

            // TODO: read straight into eqtb?
            let nb = n as usize * SIZEOF_MEMORY_WORD;
            let (ii, block) = count(be_u8, nb).parse(ii)?;
            eqtb[k * SIZEOF_MEMORY_WORD..k * SIZEOF_MEMORY_WORD + nb].copy_from_slice(&block[..]);
            k += n as usize;

            let (ii, m) = parseutils::ranged_be_i32(0, (eqtb_size + 1 - k) as i32)(ii)?;

            for j in k..k + m as usize {
                eqtb.copy_within(
                    (k - 1) * SIZEOF_MEMORY_WORD..k * SIZEOF_MEMORY_WORD,
                    j * SIZEOF_MEMORY_WORD,
                );
            }

            input = ii;
            k += m as usize;

            if k > eqtb_size {
                break;
            }
        }

        if hash_high > 0 {
            // TODO: read straight into eqtb?
            let nb = hash_high as usize * SIZEOF_MEMORY_WORD;
            let (new_input, block) = count(be_u8, nb).parse(input)?;
            input = new_input;
            let ofs = (eqtb_size + 1) * SIZEOF_MEMORY_WORD;
            eqtb[ofs..ofs + nb].copy_from_slice(&block[..]);
        }

        Ok((input, EquivalenciesTable { eqtb }))
    }

    pub fn decode(&self, index: EqtbPointer) -> EqtbEntry {
        let level = base::memword_read_b16_s0(&self.eqtb[..], index);
        let ty = base::memword_read_b16_s1(&self.eqtb[..], index);
        let value = base::memword_read_b32_s1(&self.eqtb[..], index);
        EqtbEntry { level, ty, value }
    }
}

/// Equivalent of TeX `eq_level`
#[inline(always)]
fn write_eqtb_level(arr: &mut [u8], index: i32, value: i16) {
    base::memword_write_b16_s0(arr, index, value);
}

/// A command code. Equivalent of TeX `eq_type`.
#[inline(always)]
fn write_eqtb_type(arr: &mut [u8], index: i32, value: CommandCode) {
    base::memword_write_b16_s1(arr, index, value);
}

/// Equivalent of TeX `equiv`.
#[inline(always)]
fn write_eqtb_value(arr: &mut [u8], index: i32, value: i32) {
    base::memword_write_b32_s1(arr, index, value);
}

pub fn initialize_eqtb_symbols(version: FormatVersion, symbols: &mut SymbolTable) -> Result<()> {
    let n_frozen_primitives = 12;
    let n_glue_pars = symbols.lookup("GLUE_PARS");
    let n_locals = symbols.lookup("NUM_LOCALS");
    let n_etex_pens = symbols.lookup("NUM_ETEX_PENALTIES");
    let n_int_pars = symbols.lookup("INT_PARS");
    let n_dimen_pars = symbols.lookup("DIMEN_PARS");
    let hash_size = symbols.lookup("HASH_SIZE");
    let hash_extra = symbols.lookup("HASH_EXTRA");
    let prim_size = symbols.lookup("PRIM_SIZE");
    let max_fonts = symbols.lookup("MAX_FONT_MAX");

    let active_base = 1;
    symbols.add(SymbolCategory::Eqtb, "ACTIVE_BASE", active_base)?;

    let single_base = active_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "SINGLE_BASE", single_base)?;

    let null_cs_loc = single_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "NULL_CS", null_cs_loc)?;

    let hash_base = null_cs_loc + 1;
    symbols.add(SymbolCategory::Eqtb, "HASH_BASE", hash_base)?;

    let frozen_control_sequence_base = hash_base + hash_size;
    symbols.add(
        SymbolCategory::Eqtb,
        "FROZEN_CONTROL_SEQUENCE",
        frozen_control_sequence_base,
    )?;

    let prim_eqtb_base = frozen_control_sequence_base + n_frozen_primitives;
    symbols.add(SymbolCategory::Eqtb, "PRIM_EQTB_BASE", prim_eqtb_base)?;

    let frozen_null_font_loc = prim_eqtb_base + prim_size;
    symbols.add(
        SymbolCategory::Eqtb,
        "FROZEN_NULL_FONT",
        frozen_null_font_loc,
    )?;

    let undefined_control_sequence = frozen_null_font_loc + max_fonts + 1;
    symbols.add(
        SymbolCategory::Eqtb,
        "UNDEFINED_CONTROL_SEQUENCE",
        undefined_control_sequence,
    )?;

    let glue_base = undefined_control_sequence + 1;
    symbols.add(SymbolCategory::Eqtb, "GLUE_BASE", glue_base)?;

    let skip_base = glue_base + n_glue_pars;
    symbols.add(SymbolCategory::Eqtb, "SKIP_BASE", skip_base)?;

    let mu_skip_base = skip_base + base::NUMBER_REGS as isize;
    symbols.add(SymbolCategory::Eqtb, "MU_SKIP_BASE", mu_skip_base)?;

    let local_base = mu_skip_base + base::NUMBER_REGS as isize;
    symbols.add(SymbolCategory::Eqtb, "LOCAL_BASE", local_base)?;

    let toks_base = local_base + n_locals;
    symbols.add(SymbolCategory::Eqtb, "TOKS_BASE", toks_base)?;

    let etex_pen_base = toks_base + base::NUMBER_REGS as isize;
    symbols.add(SymbolCategory::Eqtb, "ETEX_PEN_BASE", etex_pen_base)?;

    let box_base = etex_pen_base + n_etex_pens;
    symbols.add(SymbolCategory::Eqtb, "BOX_BASE", box_base)?;

    let cur_font_loc = box_base + base::NUMBER_REGS as isize;
    symbols.add(SymbolCategory::Eqtb, "CUR_FONT_LOC", cur_font_loc)?;

    let math_font_base = cur_font_loc + 1;
    symbols.add(SymbolCategory::Eqtb, "MATH_FONT_BASE", math_font_base)?;

    let cat_code_base = math_font_base + base::NUMBER_MATH_FONTS as isize;
    symbols.add(SymbolCategory::Eqtb, "CAT_CODE_BASE", cat_code_base)?;

    let lc_code_base = cat_code_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "LC_CODE_BASE", lc_code_base)?;

    let uc_code_base = lc_code_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "UC_CODE_BASE", uc_code_base)?;

    let sf_code_base = uc_code_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "SF_CODE_BASE", sf_code_base)?;

    let math_code_base = sf_code_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "MATH_CODE_BASE", math_code_base)?;

    // As of version 33, we no longer include the char_sub_code chunk, which is
    // unused because Tectonic has removed all MLTeX support.
    let char_sub_code_base = math_code_base + base::NUMBER_USVS as isize;

    let int_base = if version > 32 {
        char_sub_code_base
    } else {
        symbols.add(
            SymbolCategory::Eqtb,
            "CHAR_SUB_CODE_BASE",
            char_sub_code_base,
        )?;

        char_sub_code_base + base::NUMBER_USVS as isize
    };

    symbols.add(SymbolCategory::Eqtb, "INT_BASE", int_base)?;

    let count_base = int_base + n_int_pars;
    symbols.add(SymbolCategory::Eqtb, "COUNT_BASE", count_base)?;

    let del_code_base = count_base + base::NUMBER_REGS as isize;
    symbols.add(SymbolCategory::Eqtb, "DEL_CODE_BASE", del_code_base)?;

    let dimen_base = del_code_base + base::NUMBER_USVS as isize;
    symbols.add(SymbolCategory::Eqtb, "DIMEN_BASE", dimen_base)?;

    let scaled_base = dimen_base + n_dimen_pars;
    symbols.add(SymbolCategory::Eqtb, "SCALED_BASE", scaled_base)?;

    let eqtb_size = scaled_base + base::NUMBER_REGS as isize - 1; // XXXX note the minus-one
    symbols.add(SymbolCategory::Eqtb, "EQTB_SIZE", eqtb_size)?;

    let eqtb_top = eqtb_size + hash_extra;
    symbols.add(SymbolCategory::Eqtb, "EQTB_TOP", eqtb_top)?;

    Ok(())
}
