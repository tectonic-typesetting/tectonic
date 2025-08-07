// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! The hash table for multi-letter control sequences.

use nom::{
    multi::count,
    number::complete::{be_i32, be_u8},
    IResult, Parser,
};
use tectonic_errors::prelude::*;

use crate::{
    base::{self, SIZEOF_MEMORY_WORD},
    engine::Engine,
    eqtb::EqtbPointer,
    parseutils,
    stringtable::{StrPointer, StringTable},
    symbols::{SymbolCategory, SymbolTable},
};

pub fn initialize_cshash_symbols(symbols: &mut SymbolTable) -> Result<()> {
    symbols.add(SymbolCategory::CsHash, "HASH_SIZE", 15_000)?;
    symbols.add(SymbolCategory::CsHash, "HASH_EXTRA", 600_000)?;
    symbols.add(SymbolCategory::CsHash, "HASH_OFFSET", 514)?;
    symbols.add(SymbolCategory::CsHash, "HASH_PRIME", 8501)?;
    Ok(())
}

#[derive(Debug)]
pub struct ControlSeqHash {
    need_offset_hash: Vec<u8>,

    // To keep this type self-contained, it's easiest just to copy out the
    // settings that we need to do our various computations.
    hash_base: EqtbPointer,
    hash_prime: u32,
    hash_offset: i32,
    single_base: EqtbPointer,
    null_cs_loc: EqtbPointer,
    undefined_cs_loc: EqtbPointer,
    eqtb_size: EqtbPointer,
    eqtb_top: EqtbPointer,
    prim_eqtb_base: EqtbPointer,
    frozen_null_font_loc: EqtbPointer,
}

impl ControlSeqHash {
    pub(crate) fn parse<'a>(
        input: &'a [u8],
        engine: &Engine,
        hash_high: i32,
    ) -> IResult<&'a [u8], Self> {
        let hash_base = engine.symbols.lookup("HASH_BASE") as EqtbPointer;
        let hash_prime = engine.symbols.lookup("HASH_PRIME") as u32;
        let hash_offset = engine.symbols.lookup("HASH_OFFSET") as i32;
        let single_base = engine.symbols.lookup("SINGLE_BASE") as EqtbPointer;
        let null_cs_loc = engine.symbols.lookup("NULL_CS") as EqtbPointer;
        let undefined_cs_loc = engine.symbols.lookup("UNDEFINED_CONTROL_SEQUENCE") as EqtbPointer;
        let eqtb_size = engine.symbols.lookup("EQTB_SIZE") as EqtbPointer;
        let eqtb_top = engine.symbols.lookup("EQTB_TOP") as EqtbPointer;
        let prim_eqtb_base = engine.symbols.lookup("PRIM_EQTB_BASE") as EqtbPointer;
        let frozen_null_font_loc = engine.symbols.lookup("FROZEN_NULL_FONT") as EqtbPointer;

        let index = |i: i32| (i - hash_offset) as usize * SIZEOF_MEMORY_WORD;

        let high_hash_size = eqtb_top + 1 - hash_offset;
        let mut need_offset_hash = vec![0u8; high_hash_size as usize * SIZEOF_MEMORY_WORD];

        let (input, hash_used) = parseutils::ranged_be_i32(
            hash_base,
            engine.symbols.lookup("FROZEN_CONTROL_SEQUENCE") as i32,
        )(input)?;

        let mut p = hash_base - 1;
        let mut input = input;

        loop {
            let (ii, new_p) = parseutils::ranged_be_i32(p + 1, hash_used)(input)?;
            p = new_p;

            // TODO: load directly into `hash`?
            let (ii, block) = count(be_u8, 8).parse(ii)?;
            let ofs = index(p);
            need_offset_hash[ofs..ofs + 8].copy_from_slice(&block[..]);

            input = ii;

            if p == hash_used {
                break;
            }
        }

        // TODO: load directly into `hash`?
        let nb = ((engine.symbols.lookup("UNDEFINED_CONTROL_SEQUENCE") as i32 - 1) - hash_used)
            as usize
            * SIZEOF_MEMORY_WORD;
        let (input, block) = count(be_u8, nb).parse(input)?;
        let ofs = index(hash_used + 1);
        need_offset_hash[ofs..ofs + nb].copy_from_slice(&block[..]);

        let mut input = input;

        if hash_high > 0 {
            let nb = hash_high as usize * SIZEOF_MEMORY_WORD;
            let (new_input, block) = count(be_u8, nb).parse(input)?;
            input = new_input;
            let ofs = index(eqtb_size + 1);
            need_offset_hash[ofs..ofs + nb].copy_from_slice(&block[..]);
        }

        let (input, _cs_count) = be_i32(input)?;

        Ok((
            input,
            ControlSeqHash {
                need_offset_hash,
                hash_base,
                hash_prime,
                hash_offset,
                single_base,
                null_cs_loc,
                undefined_cs_loc,
                eqtb_size,
                eqtb_top,
                prim_eqtb_base,
                frozen_null_font_loc,
            },
        ))
    }

    fn decode(&self, index: i32) -> (StrPointer, i32) {
        let index = index - self.hash_offset;
        let text_ptr = base::memword_read_b32_s1(&self.need_offset_hash[..], index);
        let next_ptr = base::memword_read_b32_s0(&self.need_offset_hash[..], index);
        (text_ptr, next_ptr)
    }

    pub fn lookup(&self, csname: &str, strings: &StringTable) -> Option<EqtbPointer> {
        let csname_len_utf16 = crate::stringtable::len_utf16(csname);

        let mut h = 0;

        for c in csname.chars() {
            h = h + h + c as u32;
            while h >= self.hash_prime {
                h -= self.hash_prime;
            }
        }

        let mut p = h as i32 + self.hash_base;

        loop {
            let (str_ptr, next_ptr) = self.decode(p);

            if str_ptr > 0 {
                let len = strings.utf16_length(str_ptr);

                if len == csname_len_utf16 {
                    let s = strings.lookup(str_ptr);

                    if s == csname {
                        return Some(p);
                    }
                }
            }

            if next_ptr == 0 {
                return None;
            }

            p = next_ptr;
        }
    }

    /// Similar to TeX's `print_cs`
    pub fn stringify(&self, p: EqtbPointer, strings: &StringTable) -> Option<String> {
        if p < self.hash_base {
            // Single-character control sequence, or active character, or the
            // null CS.

            if p >= self.single_base {
                if p == self.null_cs_loc {
                    return Some("".to_owned());
                } else {
                    let usv = (p - self.single_base) as u32;
                    return char::from_u32(usv).map(|c| c.to_string());
                }
            } else {
                // The 1 here is formally ACTIVE_BASE
                return Some(format!(
                    "[active character {}]",
                    crate::format::fmt_usv(p - 1)
                ));
            }
        }

        if p >= self.undefined_cs_loc && p <= self.eqtb_size {
            return None;
        }

        if p > self.eqtb_top {
            return None;
        }

        if p >= self.prim_eqtb_base && p < self.frozen_null_font_loc {
            //TODO: print_esc(prim[p - PRIM_EQTB_BASE].s1 - 1);
            return None;
        }

        // `if (text(p) >= str_ptr) => "NONEXISTENT."`

        let (text_ptr, _next_ptr) = self.decode(p);
        Some(strings.lookup(text_ptr).to_owned())
    }
}
