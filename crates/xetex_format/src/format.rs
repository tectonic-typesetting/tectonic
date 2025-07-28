// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! Decode a format file.

use nom::{
    multi::count,
    number::complete::{be_i16, be_i32, be_i64, be_u16},
    Err as NomErr, IResult, Parser,
};
use std::io::Write;
use tectonic_errors::prelude::*;

use crate::{
    base::{MAX_HALFWORD, MIN_HALFWORD, TEX_NULL},
    catcodes::CatCode,
    commands::CommandCode,
    cshash,
    engine::Engine,
    eqtb::{self, EqtbPointer},
    mem, parseutils, stringtable,
    tokenlist::Token,
    FormatVersion,
};

/// Saved Tectonic/XeTeX engine state, decoded into memory.
///
/// This public API of this structure isn't yet complete. It parses format files
/// but does not yet provide proper runtime introspection of the results.
#[derive(Debug)]
#[allow(dead_code)] // TEMPORARY!
pub struct Format {
    engine: Engine,
    strings: stringtable::StringTable,
    mem: mem::Memory,
    eqtb: eqtb::EquivalenciesTable,
    cshash: cshash::ControlSeqHash,
}

// Parsing

const HEADER_MAGIC: i32 = 0x54_54_4E_43; // ASCII "TTNC"
const FOOTER_MAGIC: i32 = 0x00_00_02_9A;
const TOO_BIG_CHAR: i32 = 0x0001_0000;

const HYPH_SIZE: usize = 8191;

const TRIE_OP_SIZE: i32 = 35111;

const BIGGEST_LANG: usize = 255;

const MAX_USV: i32 = crate::base::NUMBER_USVS as i32;

impl Format {
    pub fn parse(input: &[u8]) -> Result<Self> {
        let (input, serial) = match parse_header(input) {
            Ok(t) => t,
            Err(NomErr::Error(inner)) => bail!("parse error: {}", inner.code.description()),
            Err(NomErr::Failure(inner)) => bail!("parse failure: {}", inner.code.description()),
            Err(NomErr::Incomplete(_)) => bail!("incomplete input"),
        };

        let engine = Engine::new_for_version(serial as FormatVersion)?;

        match parse_body(engine, input) {
            Ok((_remainder, result)) => Ok(result),
            Err(NomErr::Error(inner)) => bail!("parse error: {}", inner.code.description()),
            Err(NomErr::Failure(inner)) => bail!("parse failure: {}", inner.code.description()),
            Err(NomErr::Incomplete(_)) => bail!("incomplete input"),
        }
    }

    pub fn dump_string_table<W: Write>(&self, stream: &mut W) -> Result<()> {
        for sp in self.strings.all_sps() {
            let value = self.strings.lookup(sp);
            writeln!(stream, "{sp} = \"{value}\"")?;
        }

        Ok(())
    }

    pub fn dump_actives<W: Write>(&self, stream: &mut W) -> Result<()> {
        let undefined_cs_cmd = self.engine.symbols.lookup("UNDEFINED_CS") as CommandCode;

        for chr in valid_usvs() {
            let entry = self.eqtb_active(chr);

            if entry.ty == undefined_cs_cmd {
                continue;
            }

            let cur_cat = self.eqtb_catcode(chr)?;
            let cmd_desc = self.engine.commands.describe(entry.ty, entry.value);

            writeln!(
                stream,
                "{} ({}) => {}",
                fmt_usv(chr),
                cur_cat.abbrev(),
                cmd_desc
            )?;
        }

        Ok(())
    }

    pub fn dump_catcodes<W: Write>(&self, stream: &mut W) -> Result<()> {
        let mut blocks = vec![Vec::new(); 16];
        let mut start = 0;
        let mut prev = start;
        let mut cur_cat = self.eqtb_catcode(start)?;

        for chr in valid_usvs().skip(1) {
            let cat = self.eqtb_catcode(chr)?;

            if cat != cur_cat {
                blocks[cur_cat as usize].push((start, prev));
                start = chr;
                cur_cat = cat;
            }

            prev = chr;
        }

        blocks[cur_cat as usize].push((start, prev));

        for cat in 0..16 {
            if cat > 0 {
                writeln!(stream)?;
            }

            writeln!(stream, "{}:", CatCode::from_i32(cat).unwrap().description())?;

            for block in &blocks[cat as usize] {
                let (start, end) = *block;

                if end == start {
                    writeln!(stream, "    {}", fmt_usv(start))?;
                } else {
                    writeln!(stream, "    {} - {}", fmt_usv(start), fmt_usv(end))?;
                }
            }
        }

        Ok(())
    }

    pub fn dump_cseqs<W: Write>(&self, stream: &mut W, extended: bool) -> Result<()> {
        let undefined_cs_cmd = self.engine.symbols.lookup("UNDEFINED_CS") as CommandCode;

        for (name, ptr) in self.cseqs() {
            let entry = self.eqtb.decode(ptr);

            if entry.ty == undefined_cs_cmd {
                continue;
            }

            let cs_desc = fmt_csname(name);

            let (cmd_desc, extended) = if extended {
                self.engine
                    .commands
                    .describe_extended(entry.ty, entry.value, self)
            } else {
                (self.engine.commands.describe(entry.ty, entry.value), None)
            };

            writeln!(stream, "{cs_desc} => {cmd_desc}")?;

            if let Some(e) = extended {
                writeln!(stream, "--------\n{e}\n--------")?;
            }
        }

        Ok(())
    }

    fn cseqs(&self) -> impl Iterator<Item = (String, EqtbPointer)> {
        // This is lame; we shouldn't need to make a big buffer, but I'm too
        // lazy to write real iterater implementation right now.

        let null_cs = (
            "".to_owned(),
            self.engine.symbols.lookup("NULL_CS") as EqtbPointer,
        );
        let null_cs = std::iter::once(null_cs);

        let single_base = self.engine.symbols.lookup("SINGLE_BASE");
        let single_letters = valid_usvs().map(move |usv| {
            (
                char::from_u32(usv as u32).unwrap().to_string(),
                single_base as i32 + usv,
            )
        });

        let ml_data: Vec<(String, EqtbPointer)> = self
            .strings
            .all_sps()
            .filter_map(|sp| {
                let name = self.strings.lookup(sp).to_owned();
                self.cshash
                    .lookup(&name, &self.strings)
                    .map(|ptr| (name, ptr))
            })
            .collect();

        null_cs.chain(single_letters).chain(ml_data)
    }

    // Various stringifications that depend on the format data

    pub fn fmt_toklist(&self, mut p: mem::MemPointer, is_macro: bool) -> String {
        use std::fmt::Write;
        let mut result = String::new();

        if is_macro {
            // Skip the reference count
            p = self.mem.decode_toklist(p).1;
            writeln!(result, "~~ macro template: ~~").unwrap();
        }

        const CCDESCS: &[&str] = &[
            "ESC", "BGR", "EGR", "MTH", "TAB", "CAR", "MAC", "SUP", "SUB", "IGN", "SPC", "LET",
            "OTH", "ACT", "COM", "INV",
        ];

        while p != TEX_NULL {
            let (value, next) = self.mem.decode_toklist(p);
            let tok = Token::from(value);

            // TODO: condense output for runs of characters with same catcode
            match tok {
                Token::Char { cmd, chr } => match (is_macro, cmd) {
                    (true, 14 /* END_MATCH */) => {
                        writeln!(result, "~~ macro expansion: ~~").unwrap();
                    }

                    (true, 13 /* MATCH */) => {
                        writeln!(result, "<macro parameter>").unwrap();
                    }

                    (true, 5 /* OUT_PARAM */) => {
                        writeln!(result, "#{chr}").unwrap();
                    }

                    _ => {
                        // TODO: consider using fmt_csv
                        if let Some(c) = char::from_u32(chr as u32) {
                            writeln!(result, "{} {{{}}}", c, CCDESCS[cmd as usize]).unwrap();
                        } else {
                            writeln!(
                                result,
                                "[illegal USV char code 0x{:08x}] {{{}}}",
                                chr, CCDESCS[cmd as usize]
                            )
                            .unwrap();
                        }
                    }
                },

                Token::ControlSeq { ptr } => {
                    writeln!(result, "{}", self.fmt_cs_pointer(ptr)).unwrap();
                }
            }

            p = next;
        }

        result
    }

    fn fmt_cs_pointer(&self, ptr: EqtbPointer) -> String {
        if let Some(text) = self.cshash.stringify(ptr, &self.strings) {
            fmt_csname(text)
        } else {
            format!("[undecodable cseq pointer {ptr}]")
        }
    }

    // Decoding various eqtb bits. These could just as well be methods on the Eqtb
    // type, except it doesn't actually hold onto all of the magic offsets needed
    // to index into it properly.

    fn eqtb_active(&self, c: i32) -> eqtb::EqtbEntry {
        assert!((0..MAX_USV).contains(&c));
        self.eqtb
            .decode(self.engine.symbols.lookup("ACTIVE_BASE") as EqtbPointer + c)
    }

    fn eqtb_catcode(&self, c: i32) -> Result<CatCode> {
        assert!((0..MAX_USV).contains(&c));
        CatCode::from_i32(
            self.eqtb
                .decode(self.engine.symbols.lookup("CAT_CODE_BASE") as EqtbPointer + c)
                .value,
        )
    }
}

fn parse_header(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, _) = parseutils::satisfy_be_i32(HEADER_MAGIC)(input)?;
    be_i32(input)
}

fn parse_body(engine: Engine, input: &[u8]) -> IResult<&[u8], Format> {
    let mem_top = engine.symbols.lookup("MEM_TOP") as i32;
    let eqtb_size = engine.symbols.lookup("EQTB_SIZE") as i32;
    let hash_prime = engine.symbols.lookup("HASH_PRIME") as i32;
    let hash_base = engine.symbols.lookup("HASH_BASE") as i32;
    let eqtb_top = engine.symbols.lookup("EQTB_TOP") as i32;
    let prim_size = engine.symbols.lookup("PRIM_SIZE") as i32;
    let max_fonts = engine.symbols.lookup("MAX_FONT_MAX") as i32;

    let (input, hash_high) = be_i32(input)?;
    let (input, _mem_top) = parseutils::satisfy_be_i32(mem_top)(input)?;
    let (input, _eqtb_size) = parseutils::satisfy_be_i32(eqtb_size)(input)?;
    let (input, _hash_prime) = parseutils::satisfy_be_i32(hash_prime)(input)?;
    let (input, _hyph_prime) = be_i32(input)?;

    // string table

    let (input, strings) = stringtable::StringTable::parse(input)?;

    // "mem" array

    let (input, mem) = mem::Memory::parse(input, &engine)?;

    // eqtb

    let (input, eqtb) = eqtb::EquivalenciesTable::parse(input, &engine, hash_high)?;

    // nominally hash_top, but hash_top = eqtb_top since hash_extra is nonzero
    let (input, _par_loc) = parseutils::ranged_be_i32(hash_base, eqtb_top)(input)?;

    let (input, _write_loc) = parseutils::ranged_be_i32(hash_base, eqtb_top)(input)?;

    // Primitives. TODO: figure out best type for `prims`.

    let (input, _prims) = count(be_i64, prim_size as usize + 1).parse(input)?;

    // Control sequence names -- the hash table.

    let (input, cshash) = cshash::ControlSeqHash::parse(input, &engine, hash_high)?;

    // font info

    let (input, fmem_ptr) = parseutils::ranged_be_i32(7, 147483647)(input)?;

    let (input, _font_info) = count(be_i64, fmem_ptr as usize).parse(input)?;

    // NB: FONT_BASE = 0
    let (input, font_ptr) = parseutils::ranged_be_i32(0, max_fonts)(input)?;

    let n_fonts = font_ptr as usize + 1;
    let (input, _font_check) = count(be_i64, n_fonts).parse(input)?;
    let (input, _font_size) = count(be_i32, n_fonts).parse(input)?;
    let (input, _font_dsize) = count(be_i32, n_fonts).parse(input)?;
    let (input, _font_params) = count(
        parseutils::ranged_be_i32(MIN_HALFWORD, MAX_HALFWORD),
        n_fonts,
    )
    .parse(input)?;
    let (input, _hyphen_char) = count(be_i32, n_fonts).parse(input)?;
    let (input, _skew_char) = count(be_i32, n_fonts).parse(input)?;
    let (input, _font_name) = count(be_i32, n_fonts).parse(input)?;
    let (input, _font_area) = count(be_i32, n_fonts).parse(input)?;
    let (input, _font_bc) = count(be_i16, n_fonts).parse(input)?;
    let (input, _font_ec) = count(be_i16, n_fonts).parse(input)?;
    let (input, _char_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _width_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _height_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _depth_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _italic_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _lig_kern_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _kern_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _exten_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _param_base) = count(be_i32, n_fonts).parse(input)?;
    let (input, _font_glue) = count(
        parseutils::ranged_be_i32(MIN_HALFWORD, mem.lo_mem_max),
        n_fonts,
    )
    .parse(input)?;
    let (input, _bchar_label) =
        count(parseutils::ranged_be_i32(0, fmem_ptr - 1), n_fonts).parse(input)?;
    let (input, _font_bchar) =
        count(parseutils::ranged_be_i32(0, TOO_BIG_CHAR), n_fonts).parse(input)?;
    let (input, _font_false_bchar) =
        count(parseutils::ranged_be_i32(0, TOO_BIG_CHAR), n_fonts).parse(input)?;

    // Hyphenations!

    let (input, hyph_count) = be_i32(input)?;

    let (input, mut _hyph_next) = be_i32(input)?;

    let mut hyph_next;
    let mut hyph_link = vec![0u16; HYPH_SIZE];
    let mut hyph_word = vec![0i32; HYPH_SIZE];
    let mut hyph_list = vec![0i32; HYPH_SIZE];
    let mut input = input;
    let max_word = strings.len() as i32 + TOO_BIG_CHAR - 1;

    for _ in 0..hyph_count {
        let (ii, mut j) = be_i32(input)?;

        if j > 0xFFFF {
            hyph_next = j / 0x10000;
            j -= hyph_next * 0x10000;
        } else {
            hyph_next = 0;
        }

        hyph_link[j as usize] = hyph_next as u16;

        let (ii, w) = parseutils::ranged_be_i32(0, max_word)(ii)?;
        hyph_word[j as usize] = w;

        let (ii, l) = parseutils::ranged_be_i32(MIN_HALFWORD, MAX_HALFWORD)(ii)?;
        hyph_list[j as usize] = l;

        input = ii;
    }

    // trie

    let (input, trie_max) = be_i32(input)?;

    let (input, _hyph_start) = parseutils::ranged_be_i32(0, trie_max)(input)?;

    let n_trie = trie_max as usize + 1;
    let (input, _trie_trl) = count(be_i32, n_trie).parse(input)?;
    let (input, _trie_tro) = count(be_i32, n_trie).parse(input)?;
    let (input, _trie_trc) = count(be_u16, n_trie).parse(input)?;

    let (input, _max_hyph_char) = be_i32(input)?;

    let (input, trie_op_ptr) = parseutils::ranged_be_i32(0, TRIE_OP_SIZE)(input)?;

    // IMPORTANT!!! XeTeX loads these into 1-based indices!
    let (input, _hyf_distance) = count(be_i16, trie_op_ptr as usize).parse(input)?;
    let (input, _hyf_num) = count(be_i16, trie_op_ptr as usize).parse(input)?;
    let (input, _hyf_next) = count(be_u16, trie_op_ptr as usize).parse(input)?;

    let mut trie_used = vec![0i32; BIGGEST_LANG + 1];
    let mut op_start = vec![0i32; BIGGEST_LANG + 1];

    let mut k = BIGGEST_LANG + 1;
    let mut j = trie_op_ptr;
    let mut input = input;

    while j > 0 {
        let (ii, new_k) = parseutils::ranged_be_i32(0, k as i32 - 1)(input)?;
        k = new_k as usize;
        let (ii, u) = parseutils::ranged_be_i32(1, j)(ii)?;
        trie_used[k] = u;
        j -= u;
        op_start[k] = j;
        input = ii;
    }

    // All done!

    let (input, _) = parseutils::satisfy_be_i32(FOOTER_MAGIC)(input)?;

    let fmt = Format {
        engine,
        strings,
        mem,
        eqtb,
        cshash,
    };
    Ok((input, fmt))
}

fn valid_usvs() -> impl Iterator<Item = i32> {
    (0..0xD800).chain(0xE000..0x11_0000)
}

pub fn fmt_usv(c: i32) -> String {
    // Valid inputs are valid USVs, which are as per the Unicode Glossary: "Any
    // Unicode code point except high-surrogate and low-surrogate code points.
    // In other words, the ranges of integers 0x0 to 0xD7FF and 0xE000 to
    // 0x10FFFF, inclusive."
    let maybe_chr = char::from_u32(c as u32);

    if let Some(chr) = maybe_chr {
        if chr == ' ' {
            format!("' ' (0x{c:06x})")
        } else if chr == '\'' {
            format!("\\' (0x{c:06x})")
        } else if chr == '\"' {
            format!("\\\" (0x{c:06x})")
        } else if chr.is_control() || chr.is_whitespace() {
            format!("{} (0x{:06x})", chr.escape_default(), c)
        } else {
            format!("{chr} (0x{c:06x})")
        }
    } else {
        format!("*invalid* (0x{c:06x})")
    }
}

pub fn fmt_csname<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    let has_ws = name.contains(char::is_whitespace);

    match (name.len(), has_ws) {
        (0, _) => "[null CS]".to_owned(),
        (1, _) => format!("'\\{}'", fmt_usv(name.chars().next().unwrap() as i32)),
        (_, false) => format!("\\{name}"),
        (_, true) => format!("\"\\{name}\""),
    }
}
