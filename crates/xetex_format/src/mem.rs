// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! The TeX dynamic memory array.

use nom::{
    multi::count,
    number::complete::{be_i32, be_u8},
    IResult, Parser,
};

use crate::{
    base::{self, MIN_HALFWORD, SIZEOF_MEMORY_WORD},
    engine::Engine,
    parseutils,
};

pub type MemPointer = i32;

#[derive(Debug)]
pub struct Memory {
    pub mem: Vec<u8>,

    /// This is needed by the format-file parser for a bounds check.
    pub lo_mem_max: MemPointer,
}

const HI_MEM_STAT_USAGE: i32 = 15;
const N_SERIALIZED_SA_ROOTS: usize = 7; // INT_VAL => INTER_CHAR_VAL, inclusive

impl Memory {
    pub(crate) fn parse<'a>(input: &'a [u8], engine: &Engine) -> IResult<&'a [u8], Self> {
        let mem_top = engine.symbols.lookup("MEM_TOP") as usize;

        // lower limit hardcoded (?)
        let (input, lo_mem_max) =
            parseutils::ranged_be_i32(1019, mem_top as i32 - HI_MEM_STAT_USAGE)(input)?;

        // lower limit hardcoded
        let (input, rover) = parseutils::ranged_be_i32(20, lo_mem_max)(input)?;

        let (input, _sa_roots) = count(
            parseutils::ranged_be_i32(MIN_HALFWORD, lo_mem_max),
            N_SERIALIZED_SA_ROOTS,
        )
        .parse(input)?;

        // Compressed memory loading;

        let mut mem = vec![0; (mem_top + 1) * SIZEOF_MEMORY_WORD];
        let mut input = input;
        let mut p = 0;
        let mut q = rover;

        loop {
            let nb = (q + 2 - p) as usize * SIZEOF_MEMORY_WORD;

            // TODO: read straight into mem?
            let (new_input, block) = count(be_u8, nb).parse(input)?;
            input = new_input;
            let idx = p as usize * SIZEOF_MEMORY_WORD;
            mem[idx..idx + nb].copy_from_slice(&block[..]);

            let ofs = base::memword_read_b32_s0(&mem[..], q);
            p = q + ofs;
            assert!(p <= lo_mem_max);

            q = base::memword_read_b32_s1(&mem[..], q + 1);

            if q == rover {
                break;
            }
        }

        // Loading the rest of low memory. TODO: straight into `mem`?
        let nb = (lo_mem_max + 1 - p) as usize * SIZEOF_MEMORY_WORD;
        let (input, block) = count(be_u8, nb).parse(input)?;
        let idx = p as usize * SIZEOF_MEMORY_WORD;
        mem[idx..idx + nb].copy_from_slice(&block[..]);

        let (input, hi_mem_min) =
            parseutils::ranged_be_i32(lo_mem_max + 1, mem_top as i32 - HI_MEM_STAT_USAGE)(input)?;

        let (input, _avail) = parseutils::ranged_be_i32(MIN_HALFWORD, mem_top as i32)(input)?;

        let nb = (mem_top + 1 - hi_mem_min as usize) * SIZEOF_MEMORY_WORD;
        let (input, block) = count(be_u8, nb).parse(input)?;
        mem[hi_mem_min as usize * SIZEOF_MEMORY_WORD
            ..hi_mem_min as usize * SIZEOF_MEMORY_WORD + nb]
            .copy_from_slice(&block[..]);

        let (input, _var_used) = be_i32(input)?;
        let (input, _dyn_used) = be_i32(input)?;
        Ok((input, Memory { mem, lo_mem_max }))
    }

    pub fn decode_toklist(&self, index: MemPointer) -> (i32, MemPointer) {
        let value = base::memword_read_b32_s0(&self.mem[..], index);
        let next = base::memword_read_b32_s1(&self.mem[..], index);
        (value, next)
    }
}
