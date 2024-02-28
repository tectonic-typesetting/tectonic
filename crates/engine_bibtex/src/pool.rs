use crate::{
    auxi::AuxCommand,
    bibs::BibCommand,
    bst::BstCommand,
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    entries::ENT_STR_SIZE,
    exec::ControlSeq,
    global::GLOB_STR_SIZE,
    hash,
    hash::{BstBuiltin, BstFn, HashData, HashExtra},
    log::{output_bbl_line, print_overflow},
    ASCIICode, Bibtex, BibtexError, GlobalItems, HashPointer, LookupRes, PoolPointer, StrIlk,
    StrNumber,
};
use std::ops::Range;

const POOL_SIZE: usize = 65000;
pub(crate) const MAX_PRINT_LINE: usize = 79;
pub(crate) const MIN_PRINT_LINE: usize = 3;
pub(crate) const MAX_STRINGS: usize = 35307;

#[derive(Debug, PartialEq)]
pub(crate) enum LookupErr {
    Invalid,
    DoesntExist,
}

pub(crate) struct StringPool {
    strings: Vec<u8>,
    // Stores string starting locations in the string pool
    // length of string `s` is offsets[s + 1] - offsets[s]
    offsets: Vec<usize>,
    pool_ptr: PoolPointer,
    str_ptr: StrNumber,
}

impl StringPool {
    pub(crate) fn new() -> StringPool {
        StringPool {
            strings: vec![0; POOL_SIZE + 1],
            offsets: vec![0; MAX_STRINGS + 1],
            pool_ptr: 0,
            str_ptr: 1,
        }
    }

    pub fn try_get_str(&self, s: usize) -> Result<&[u8], LookupErr> {
        // TODO: Why plus three? Should probably find if somewhere relies on that
        if s == 0 || s >= self.str_ptr + 3 {
            Err(LookupErr::DoesntExist)
        } else if s >= MAX_STRINGS {
            Err(LookupErr::Invalid)
        } else {
            Ok(&self.strings[self.offsets[s]..self.offsets[s + 1]])
        }
    }

    pub fn get_str(&self, s: usize) -> &[u8] {
        self.try_get_str(s).unwrap_or_else(|e| match e {
            LookupErr::DoesntExist => panic!("String number {} doesn't exist", s),
            LookupErr::Invalid => panic!("Invalid string number {}", s),
        })
    }

    pub fn grow(&mut self) {
        self.strings.resize(self.strings.len() + POOL_SIZE, 0);
    }

    /// Used while defining strings - declare the current `pool_ptr` as the end of the current
    /// string, increment the `str_ptr`, and return the new string's `StrNumber`
    pub fn make_string(&mut self, ctx: &mut Bibtex<'_, '_>) -> Result<StrNumber, BibtexError> {
        if self.str_ptr == MAX_STRINGS {
            print_overflow(ctx);
            ctx.write_logs(&format!("number of strings {}\n", MAX_STRINGS));
            return Err(BibtexError::Fatal);
        }
        self.str_ptr += 1;
        self.offsets[self.str_ptr] = self.pool_ptr;
        Ok(self.str_ptr - 1)
    }

    fn hash_str(hash: &HashData, str: &[ASCIICode]) -> usize {
        let prime = hash.prime();
        str.iter()
            .fold(0, |acc, &c| ((2 * acc) + c as usize) % prime)
    }

    pub fn lookup_str(&self, hash: &HashData, str: &[ASCIICode], ilk: StrIlk) -> LookupRes {
        let h = Self::hash_str(hash, str);
        let mut p = h as HashPointer + hash::HASH_BASE as HashPointer;

        let exists = loop {
            let existing = hash.text(p);

            if existing > 0 && self.get_str(existing) == str && hash.node(p).kind() == ilk {
                break true;
            }

            if hash.next(p) == 0 {
                break false;
            }

            p = hash.next(p);
        };

        LookupRes { loc: p, exists }
    }

    /// Lookup a string, inserting it if it isn't found. Note that this returns `Ok` whether the
    /// string is found or not, only returning `Err` if a called function fails.
    pub(crate) fn lookup_str_insert(
        &mut self,
        ctx: &mut Bibtex<'_, '_>,
        hash: &mut HashData,
        str: &[ASCIICode],
        ilk: HashExtra,
    ) -> Result<LookupRes, BibtexError> {
        // Hash string using simple hash function. This hash is capped to HASH_PRIME
        let h = Self::hash_str(hash, str);
        let mut str_num = 0;
        // Get position by adding HASH_BASE
        let mut p = (h + hash::HASH_BASE) as HashPointer;

        // Look for an existing match, or the last slot
        let existing = loop {
            // Get the current text at the position
            let existing = hash.text(p);
            // If the text exists and is the same as the text we're adding
            if self.try_get_str(existing) == Ok(str) {
                // If an existing hash entry exists for this type, return it
                if hash.node(p).kind() == ilk.kind() {
                    return Ok(LookupRes {
                        loc: p,
                        exists: true,
                    });
                } else {
                    str_num = existing;
                }
            }

            if hash.next(p) == 0 {
                break existing;
            }

            p = hash.next(p);
        };

        // If we hit the end and the slot is already in use
        if existing > 0 {
            // Walk backwards from our current len to our first empty slot.
            // If all slots are full, error
            loop {
                if hash.len() == hash::HASH_BASE {
                    print_overflow(ctx);
                    ctx.write_logs(&format!("hash size {}\n", hash::HASH_SIZE));
                    return Err(BibtexError::Fatal);
                }
                hash.set_len(hash.len() - 1);

                if hash.text(hash.len()) == 0 {
                    break;
                }
            }
            // Set the next item to our new lowest open slot
            hash.set_next(p, hash.len());
            // Operate on the new empty slot
            p = hash.len();
        }

        // We found the string in the string pool while hunting for a slot
        if str_num > 0 {
            hash.set_text(p, str_num);
        // The string isn't in the string pool - add it
        } else {
            while self.pool_ptr + str.len() > self.strings.len() {
                self.grow();
            }
            self.strings[self.pool_ptr..self.pool_ptr + str.len()].copy_from_slice(str);
            self.pool_ptr += str.len();

            match self.make_string(ctx) {
                Ok(str) => hash.set_text(p, str),
                Err(err) => return Err(err),
            }
        }

        // Set the type of this slot
        hash.node_mut(p).extra = ilk;

        Ok(LookupRes {
            loc: p,
            exists: false,
        })
    }

    pub fn str_ptr(&self) -> usize {
        self.str_ptr
    }

    pub fn set_str_ptr(&mut self, val: usize) {
        self.str_ptr = val;
    }

    pub fn pool_ptr(&self) -> usize {
        self.pool_ptr
    }

    pub fn set_pool_ptr(&mut self, val: usize) {
        self.pool_ptr = val;
    }

    pub fn str_start(&self, str: StrNumber) -> usize {
        self.offsets[str]
    }

    // TODO: Encapsulate better
    pub fn set_start(&mut self, str: StrNumber, start: usize) {
        self.offsets[str] = start;
    }

    pub fn copy_raw(&mut self, str: StrNumber, pos: usize) {
        let start = self.offsets[str];
        let end = self.offsets[str + 1];

        while pos + (end - start) > self.strings.len() {
            self.grow();
        }

        self.strings.copy_within(start..end, pos);
    }

    pub fn copy_range_raw(&mut self, range: Range<usize>, pos: usize) {
        while pos + (range.end - range.start) > self.strings.len() {
            self.grow();
        }
        self.strings.copy_within(range, pos)
    }

    pub fn append(&mut self, c: ASCIICode) {
        self.strings[self.pool_ptr] = c;
        self.pool_ptr += 1;
    }

    pub fn add_string_raw(
        &mut self,
        ctx: &mut Bibtex<'_, '_>,
        str: &[ASCIICode],
    ) -> Result<PoolPointer, BibtexError> {
        while self.pool_ptr + str.len() > self.strings.len() {
            self.grow();
        }
        self.strings[self.pool_ptr..self.pool_ptr + str.len()].copy_from_slice(str);
        self.pool_ptr += str.len();
        self.make_string(ctx)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

pub(crate) fn add_buf_pool(pool: &StringPool, buffers: &mut GlobalBuffer, str: StrNumber) {
    let str = pool.get_str(str);

    if buffers.init(BufTy::Ex) + str.len() > buffers.len() {
        buffers.grow_all();
    }

    let start = buffers.init(BufTy::Ex);
    buffers.copy_from(BufTy::Ex, start, str);
    buffers.set_offset(BufTy::Ex, 1, start + str.len());
    buffers.set_init(BufTy::Ex, start + str.len());
}

pub(crate) fn pre_def_certain_strings(
    ctx: &mut Bibtex<'_, '_>,
    GlobalItems {
        pool,
        hash,
        other,
        entries,
        ..
    }: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    let res = pool.lookup_str_insert(ctx, hash, b".aux", HashExtra::FileExt)?;
    ctx.s_aux_extension = hash.text(res.loc);

    pool.lookup_str_insert(
        ctx,
        hash,
        b"\\bibdata",
        HashExtra::AuxCommand(AuxCommand::Data),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"\\bibstyle",
        HashExtra::AuxCommand(AuxCommand::Style),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"\\citation",
        HashExtra::AuxCommand(AuxCommand::Citation),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"\\@input",
        HashExtra::AuxCommand(AuxCommand::Input),
    )?;

    pool.lookup_str_insert(
        ctx,
        hash,
        b"entry",
        HashExtra::BstCommand(BstCommand::Entry),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"execute",
        HashExtra::BstCommand(BstCommand::Execute),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"function",
        HashExtra::BstCommand(BstCommand::Function),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"integers",
        HashExtra::BstCommand(BstCommand::Integers),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"iterate",
        HashExtra::BstCommand(BstCommand::Iterate),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"macro",
        HashExtra::BstCommand(BstCommand::Macro),
    )?;
    pool.lookup_str_insert(ctx, hash, b"read", HashExtra::BstCommand(BstCommand::Read))?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"reverse",
        HashExtra::BstCommand(BstCommand::Reverse),
    )?;
    pool.lookup_str_insert(ctx, hash, b"sort", HashExtra::BstCommand(BstCommand::Sort))?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"strings",
        HashExtra::BstCommand(BstCommand::Strings),
    )?;

    pool.lookup_str_insert(
        ctx,
        hash,
        b"comment",
        HashExtra::BibCommand(BibCommand::Comment),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"preamble",
        HashExtra::BibCommand(BibCommand::Preamble),
    )?;
    pool.lookup_str_insert(
        ctx,
        hash,
        b"string",
        HashExtra::BibCommand(BibCommand::String),
    )?;

    let mut build_in = |pds: &[ASCIICode], builtin| {
        let res =
            pool.lookup_str_insert(ctx, hash, pds, HashExtra::BstFn(BstFn::Builtin(builtin)))?;
        Ok(res.loc)
    };

    build_in(b"=", BstBuiltin::Eq)?;
    build_in(b">", BstBuiltin::Gt)?;
    build_in(b"<", BstBuiltin::Lt)?;
    build_in(b"+", BstBuiltin::Plus)?;
    build_in(b"-", BstBuiltin::Minus)?;
    build_in(b"*", BstBuiltin::Concat)?;
    build_in(b":=", BstBuiltin::Set)?;
    build_in(b"add.period$", BstBuiltin::AddPeriod)?;
    build_in(b"call.type$", BstBuiltin::CallType)?;
    build_in(b"change.case$", BstBuiltin::ChangeCase)?;
    build_in(b"chr.to.int$", BstBuiltin::ChrToInt)?;
    build_in(b"cite$", BstBuiltin::Cite)?;
    build_in(b"duplicate$", BstBuiltin::Duplicate)?;
    build_in(b"empty$", BstBuiltin::Empty)?;
    build_in(b"format.name$", BstBuiltin::FormatName)?;
    build_in(b"if$", BstBuiltin::If)?;
    build_in(b"int.to.chr$", BstBuiltin::IntToChr)?;
    build_in(b"int.to.str$", BstBuiltin::IntToStr)?;
    build_in(b"missing$", BstBuiltin::Missing)?;
    build_in(b"newline$", BstBuiltin::Newline)?;
    build_in(b"num.names$", BstBuiltin::NumNames)?;
    build_in(b"pop$", BstBuiltin::Pop)?;
    build_in(b"preamble$", BstBuiltin::Preamble)?;
    build_in(b"purify$", BstBuiltin::Purify)?;
    build_in(b"quote$", BstBuiltin::Quote)?;
    let skip_loc = build_in(b"skip$", BstBuiltin::Skip)?;
    build_in(b"stack$", BstBuiltin::Stack)?;
    build_in(b"substring$", BstBuiltin::Substring)?;
    build_in(b"swap$", BstBuiltin::Swap)?;
    build_in(b"text.length$", BstBuiltin::TextLength)?;
    build_in(b"text.prefix$", BstBuiltin::TextPrefix)?;
    build_in(b"top$", BstBuiltin::Top)?;
    build_in(b"type$", BstBuiltin::Type)?;
    build_in(b"warning$", BstBuiltin::Warning)?;
    build_in(b"while$", BstBuiltin::While)?;
    build_in(b"width$", BstBuiltin::Width)?;
    build_in(b"write$", BstBuiltin::Write)?;

    let res = pool.lookup_str_insert(ctx, hash, b"", HashExtra::Text)?;
    ctx.s_null = hash.text(res.loc);
    let res = pool.lookup_str_insert(ctx, hash, b"default.type", HashExtra::Text)?;
    ctx.s_default = hash.text(res.loc);
    ctx.b_default = skip_loc;

    pool.lookup_str_insert(ctx, hash, b"i", HashExtra::ControlSeq(ControlSeq::LowerI))?;
    pool.lookup_str_insert(ctx, hash, b"j", HashExtra::ControlSeq(ControlSeq::LowerJ))?;
    pool.lookup_str_insert(ctx, hash, b"oe", HashExtra::ControlSeq(ControlSeq::LowerOE))?;
    pool.lookup_str_insert(ctx, hash, b"OE", HashExtra::ControlSeq(ControlSeq::UpperOE))?;
    pool.lookup_str_insert(ctx, hash, b"ae", HashExtra::ControlSeq(ControlSeq::LowerAE))?;
    pool.lookup_str_insert(ctx, hash, b"AE", HashExtra::ControlSeq(ControlSeq::UpperAE))?;
    pool.lookup_str_insert(ctx, hash, b"aa", HashExtra::ControlSeq(ControlSeq::LowerAA))?;
    pool.lookup_str_insert(ctx, hash, b"AA", HashExtra::ControlSeq(ControlSeq::UpperAA))?;
    pool.lookup_str_insert(ctx, hash, b"o", HashExtra::ControlSeq(ControlSeq::LowerO))?;
    pool.lookup_str_insert(ctx, hash, b"O", HashExtra::ControlSeq(ControlSeq::UpperO))?;
    pool.lookup_str_insert(ctx, hash, b"l", HashExtra::ControlSeq(ControlSeq::LowerL))?;
    pool.lookup_str_insert(ctx, hash, b"L", HashExtra::ControlSeq(ControlSeq::UpperL))?;
    pool.lookup_str_insert(ctx, hash, b"ss", HashExtra::ControlSeq(ControlSeq::LowerSS))?;

    let num_fields = other.num_fields();
    pool.lookup_str_insert(
        ctx,
        hash,
        b"crossref",
        HashExtra::BstFn(BstFn::Field(num_fields)),
    )?;
    other.set_crossref_num(num_fields);
    other.set_num_fields(num_fields + 1);
    other.set_pre_defined_fields(num_fields + 1);

    let num_ent_strs = entries.num_ent_strs();
    pool.lookup_str_insert(
        ctx,
        hash,
        b"sort.key$",
        HashExtra::BstFn(BstFn::StrEntry(num_ent_strs)),
    )?;
    entries.set_sort_key_num(num_ent_strs);
    entries.set_num_ent_strs(num_ent_strs + 1);

    pool.lookup_str_insert(
        ctx,
        hash,
        b"entry.max$",
        HashExtra::BstFn(BstFn::IntGlbl(ENT_STR_SIZE as i32)),
    )?;

    pool.lookup_str_insert(
        ctx,
        hash,
        b"global.max$",
        HashExtra::BstFn(BstFn::IntGlbl(GLOB_STR_SIZE as i32)),
    )?;

    Ok(())
}

pub(crate) fn add_out_pool(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    str: StrNumber,
) {
    let str = pool.get_str(str);

    while buffers.init(BufTy::Out) + str.len() > buffers.len() {
        buffers.grow_all();
    }

    let out_offset = buffers.init(BufTy::Out);
    buffers.copy_from(BufTy::Out, out_offset, str);
    buffers.set_init(BufTy::Out, out_offset + str.len());

    let mut unbreakable_tail = false;
    while buffers.init(BufTy::Out) > MAX_PRINT_LINE && !unbreakable_tail {
        let end_ptr = buffers.init(BufTy::Out);
        let mut out_offset = MAX_PRINT_LINE;
        let mut break_pt_found = false;

        while LexClass::of(buffers.at(BufTy::Out, out_offset)) != LexClass::Whitespace
            && out_offset >= MIN_PRINT_LINE
        {
            out_offset -= 1;
        }

        if out_offset == MIN_PRINT_LINE - 1 {
            out_offset = MAX_PRINT_LINE + 1;
            while out_offset < end_ptr {
                if LexClass::of(buffers.at(BufTy::Out, out_offset)) != LexClass::Whitespace {
                    out_offset += 1;
                } else {
                    break;
                }
            }

            if out_offset == end_ptr {
                unbreakable_tail = true;
            } else {
                break_pt_found = true;
                while out_offset + 1 < end_ptr {
                    if LexClass::of(buffers.at(BufTy::Out, out_offset + 1)) == LexClass::Whitespace
                    {
                        out_offset += 1;
                    } else {
                        break;
                    }
                }
            }
        } else {
            break_pt_found = true;
        }

        if break_pt_found {
            buffers.set_init(BufTy::Out, out_offset);
            let break_ptr = buffers.init(BufTy::Out) + 1;
            output_bbl_line(ctx, buffers);
            buffers.set_at(BufTy::Out, 0, b' ');
            buffers.set_at(BufTy::Out, 1, b' ');
            let len = end_ptr - break_ptr;
            buffers.copy_within(BufTy::Out, BufTy::Out, break_ptr, 2, len);
            buffers.set_init(BufTy::Out, len + 2);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::BibtexConfig;
    // use tectonic_bridge_core::{CoreBridgeLauncher, CoreBridgeState, DriverHooks, MinimalDriver};

    // TODO: Create context without backend? Use custom backend-like type?
    //       Implement the relevant interfaces ourself?
    // #[test]
    // fn test_pool() {
    //     with_cbs(|cbs| {
    //         let mut ctx = Bibtex::new(cbs, BibtexConfig::default());
    //         let mut hash = HashData::new();
    //         let mut new_pool = StringPool::new();
    //         let res = new_pool
    //             .lookup_str_insert(&mut ctx, &mut hash, b"a cool string", HashExtra::Text)
    //             .unwrap();
    //         assert!(!res.exists);
    //         assert_eq!(
    //             new_pool.try_get_str(hash.text(res.loc)),
    //             Ok(b"a cool string" as &[_])
    //         );
    //
    //         let res2 = new_pool
    //             .lookup_str_insert(&mut ctx, &mut hash, b"a cool string", HashExtra::Text)
    //             .unwrap();
    //         assert!(res2.exists);
    //         assert_eq!(
    //             new_pool.try_get_str(hash.text(res2.loc)),
    //             Ok(b"a cool string" as &[_])
    //         );
    //
    //         let res3 = new_pool.lookup_str(&hash, b"a cool string", StrIlk::Text);
    //         assert!(res3.exists);
    //         assert_eq!(
    //             new_pool.try_get_str(hash.text(res3.loc)),
    //             Ok(b"a cool string" as &[_])
    //         );
    //
    //         let res4 = new_pool.lookup_str(&hash, b"a bad string", StrIlk::Text);
    //         assert!(!res4.exists);
    //         assert_eq!(
    //             new_pool.try_get_str(hash.text(res4.loc)),
    //             Err(LookupErr::DoesntExist)
    //         );
    //     })
    //     .unwrap()
    // }
}
