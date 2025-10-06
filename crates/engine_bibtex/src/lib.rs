// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(clippy::assertions_on_constants)]

//! The [bibtex] program as a reusable crate.
//!
//! [bibtex]: http://www.bibtex.org/
//!
//! This crate provides the basic BibTeX implementation used by [Tectonic].
//! However, in order to obtain the full Tectonic user experience, it must be
//! combined with a variety of other utilities: the main XeTeX engine, code to
//! fetch support files, and so on. Rather than using this crate directly you
//! should probably use the main [`tectonic`] crate, which combines all of these
//! pieces into a (semi) coherent whole.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/
//! [`tectonic`]: https://docs.rs/tectonic/

use crate::{
    auxi::{
        get_aux_command_and_process, last_check_for_aux_errors, pop_the_aux_stack, AuxCommand,
        AuxData,
    },
    bibs::{BibCommand, BibData},
    bst::{get_bst_command_and_process, BstCommand},
    buffer::{BufTy, GlobalBuffer},
    cite::CiteInfo,
    entries::{EntryData, ENT_STR_SIZE},
    exec::{ControlSeq, ExecCtx},
    global::{GlobalData, GLOB_STR_SIZE},
    hash::{BstBuiltin, BstFn, HashData, HashPointer},
    log::{
        bib_close_log, log_pr_aux_name, print_aux_name, print_confusion, sam_wrong_file_name_print,
        AsBytes,
    },
    other::OtherData,
    peekable::{input_ln, PeekableInput},
    pool::{StrNumber, StringPool},
    scan::eat_bst_white_space,
};
use std::{
    ffi::{CStr, CString},
    io::Write,
};
use tectonic_bridge_core::{CoreBridgeLauncher, CoreBridgeState, FileFormat, OutputId};
use tectonic_errors::prelude::*;

pub(crate) mod auxi;
pub(crate) mod bibs;
pub(crate) mod bst;
pub(crate) mod buffer;
pub(crate) mod char_info;
pub(crate) mod cite;
pub(crate) mod entries;
pub(crate) mod exec;
pub(crate) mod global;
pub(crate) mod hash;
pub(crate) mod log;
pub(crate) mod other;
pub(crate) mod peekable;
pub(crate) mod pool;
pub(crate) mod scan;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum History {
    Spotless,
    WarningIssued(u32),
    ErrorIssued(u32),
    FatalError,
}

#[doc(hidden)]
#[derive(Debug)]
pub(crate) enum BibtexError {
    Fatal,
    Recover,
    NoBst,
}

/// A possible outcome from a BibTeX engine invocation.
///
/// The classic TeX implementation provides a fourth outcome: "fatal error". In
/// Tectonic, this outcome is represented as an `Err` result rather than a
/// [`BibtexOutcome`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BibtexOutcome {
    /// Nothing bad happened.
    Spotless = 0,

    /// Warnings were issued.
    Warnings = 1,

    /// Errors occurred. Note that, in TeX usage, "errors" are not necessarily
    /// *fatal* errors: the engine will proceed and work around errors as best
    /// it can.
    Errors = 2,
}

/// A struct for invoking the BibTeX engine.
///
/// This struct has a fairly straightforward "builder" interface: you create it,
/// apply any settings that you wish, and eventually run the
/// [`process()`](Self::process) method.
///
/// Due to constraints of the gnarly C/C++ code underlying the engine
/// implementation, only one engine may run at once in one process. The engine
/// execution framework uses a global mutex to ensure that this is the case.
/// This restriction applies not only to the [`BibtexEngine`] type but to *all*
/// Tectonic engines. I.e., you can't run this engine and the XeTeX engine at
/// the same time.
#[derive(Debug, Default)]
pub struct BibtexEngine {
    config: BibtexConfig,
}

impl BibtexEngine {
    /// Set the BibTeX `min_crossrefs` parameter.
    ///
    /// The default value is 2.
    ///
    /// This needs verifying, but I believe that this setting affects how many
    /// times an item needs to be referenced in directly-referenced BibTeX
    /// entries before it gets its own standalone entry.
    pub fn min_crossrefs(&mut self, value: u32) -> &mut Self {
        self.config.min_crossrefs = value;
        self
    }

    /// Run BibTeX.
    ///
    /// The *launcher* parameter gives overarching environmental context in
    /// which the engine will be run.
    ///
    /// The *aux* parameter gives the name of the "aux" file, created by the TeX
    /// engine, that BibTeX will process.
    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher<'_>,
        aux: &str,
    ) -> Result<BibtexOutcome> {
        let caux = CString::new(aux)?;

        launcher.with_global_lock(|state| {
            let mut ctx = Bibtex::new(state, self.config.clone());
            let hist = bibtex_main(&mut ctx, &caux);

            match hist {
                History::Spotless => Ok(BibtexOutcome::Spotless),
                History::WarningIssued(_) => Ok(BibtexOutcome::Warnings),
                History::ErrorIssued(_) => Ok(BibtexOutcome::Errors),
                History::FatalError => Err(anyhow!("unspecified fatal bibtex error")),
            }
        })
    }
}

// These used to be 'bad' checks at the start of a program, now we can ensure them at comptime
const _: () = assert!(hash::HASH_PRIME >= 128);
const _: () = assert!(pool::MAX_PRINT_LINE > pool::MIN_PRINT_LINE);
const _: () = assert!(pool::MIN_PRINT_LINE >= 3);
const _: () = assert!(pool::MAX_PRINT_LINE < buffer::BUF_SIZE + 1);
const _: () = assert!(hash::HASH_PRIME <= hash::HASH_SIZE);
const _: () = assert!(pool::MAX_STRINGS <= hash::HASH_SIZE);
const _: () = assert!(cite::MAX_CITES <= pool::MAX_STRINGS);

pub(crate) struct File {
    name: StrNumber,
    file: PeekableInput,
    line: u32,
}

pub(crate) struct GlobalItems<'a> {
    buffers: &'a mut GlobalBuffer,
    pool: &'a mut StringPool,
    hash: &'a mut HashData,
    entries: &'a mut EntryData,
    globals: &'a mut GlobalData,
    bibs: &'a mut BibData,
    aux: &'a mut AuxData,
    cites: &'a mut CiteInfo,
    other: &'a mut OtherData,
}

#[derive(Clone, Debug)]
pub(crate) struct BibtexConfig {
    pub min_crossrefs: u32,
    pub verbose: bool,
}

impl Default for BibtexConfig {
    fn default() -> Self {
        BibtexConfig {
            min_crossrefs: 2,
            verbose: false,
        }
    }
}

#[derive(Default)]
pub(crate) struct Logs {
    stdout: Option<OutputId>,
    file: Option<OutputId>,
}

pub(crate) struct Bibtex<'a, 'cbs> {
    pub engine: &'a mut CoreBridgeState<'cbs>,
    pub config: BibtexConfig,
    pub history: History,
    pub logs: Logs,

    pub bst: Option<File>,

    pub bbl_file: Option<OutputId>,
    pub bbl_line_num: usize,

    pub impl_fn_num: usize,
    pub cite_xptr: usize,

    pub bib_seen: bool,
    pub bst_seen: bool,
    pub citation_seen: bool,
    pub entry_seen: bool,
    pub read_seen: bool,
    pub read_performed: bool,
    pub reading_completed: bool,
    pub all_entries: bool,

    pub b_default: HashPointer<hash::BstFn>,
    pub s_null: StrNumber,
    pub s_default: StrNumber,
    pub s_aux_extension: StrNumber,
}

impl<'a, 'cbs> Bibtex<'a, 'cbs> {
    pub(crate) fn new(
        engine: &'a mut CoreBridgeState<'cbs>,
        config: BibtexConfig,
    ) -> Bibtex<'a, 'cbs> {
        Bibtex {
            engine,
            config,
            history: History::Spotless,
            logs: Logs::default(),
            bst: None,
            bbl_file: None,
            bbl_line_num: 1,
            impl_fn_num: 0,
            cite_xptr: 0,
            bib_seen: false,
            bst_seen: false,
            citation_seen: false,
            entry_seen: false,
            read_seen: false,
            read_performed: false,
            reading_completed: false,
            all_entries: false,
            b_default: HashPointer::default(),
            s_null: StrNumber::invalid(),
            s_default: StrNumber::invalid(),
            s_aux_extension: StrNumber::invalid(),
        }
    }

    pub(crate) fn mark_warning(&mut self) {
        match self.history {
            History::WarningIssued(cur) => self.history = History::WarningIssued(cur + 1),
            History::Spotless => self.history = History::WarningIssued(1),
            _ => (),
        }
    }

    pub(crate) fn mark_error(&mut self) {
        match self.history {
            History::Spotless | History::WarningIssued(_) => self.history = History::ErrorIssued(1),
            History::ErrorIssued(cur) => self.history = History::ErrorIssued(cur + 1),
            _ => (),
        }
    }

    pub(crate) fn mark_fatal(&mut self) {
        self.history = History::FatalError;
    }

    pub(crate) fn write_logs<B: ?Sized + AsBytes>(&mut self, str: &B) {
        let _ = self
            .engine
            .get_output(self.logs.file.unwrap())
            .write_all(str.as_bytes());
        let _ = self
            .engine
            .get_output(self.logs.stdout.unwrap())
            .write_all(str.as_bytes());
    }

    pub(crate) fn write_stdout<B: ?Sized + AsBytes>(&mut self, str: &B) {
        let _ = self
            .engine
            .get_output(self.logs.stdout.unwrap())
            .write_all(str.as_bytes());
    }

    pub(crate) fn write_log_file<B: ?Sized + AsBytes>(&mut self, str: &B) {
        self.engine
            .get_output(self.logs.file.unwrap())
            .write_all(str.as_bytes())
            .unwrap();
    }

    pub(crate) fn init_stdout(&mut self) -> bool {
        if self.logs.stdout.is_none() {
            self.logs.stdout = self.engine.output_open_stdout();
            self.logs.stdout.is_some()
        } else {
            true
        }
    }

    pub(crate) fn init_log_file(&mut self, file: &CStr) -> bool {
        if self.logs.file.is_none() {
            self.logs.file = self.engine.output_open(file.to_str().unwrap(), false);
            self.logs.file.is_some()
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub(crate) struct LookupRes<T> {
    /// The location of the string - where it exists, was inserted, of if insert is false,
    /// where it *would* have been inserted
    loc: HashPointer<T>,
    /// Whether the string existed in the hash table already
    exists: bool,
}

impl<T> Clone for LookupRes<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for LookupRes<T> {}

#[derive(Debug)]
pub(crate) struct FindCiteLocs {
    cite: Option<HashPointer<hash::Cite>>,
    lc_cite: Option<HashPointer<hash::LcCite>>,
}

type CiteNumber = usize;
type ASCIICode = u8;
type BufPointer = usize;
type FieldLoc = usize;

pub(crate) fn bibtex_main(ctx: &mut Bibtex<'_, '_>, aux_file_name: &CStr) -> History {
    let mut buffers = GlobalBuffer::new();
    let mut pool = StringPool::new();
    let mut hash = HashData::new();
    let mut entries = EntryData::new();
    let mut globals = GlobalData::new();
    let mut bibs = BibData::new();
    let mut aux = AuxData::new();
    let mut cites = CiteInfo::new();
    let mut other = OtherData::new();

    let mut globals = GlobalItems {
        buffers: &mut buffers,
        pool: &mut pool,
        hash: &mut hash,
        entries: &mut entries,
        globals: &mut globals,
        bibs: &mut bibs,
        aux: &mut aux,
        cites: &mut cites,
        other: &mut other,
    };

    let res = inner_bibtex_main(ctx, &mut globals, aux_file_name);
    match res {
        Err(BibtexError::Recover) | Ok(History::Spotless) => {
            ctx.bst.take().map(|file| file.file.close(ctx));
            if let Some(bbl) = ctx.bbl_file {
                ctx.engine.output_close(bbl);
            }
        }
        Err(BibtexError::NoBst) => {
            if let Some(bbl) = ctx.bbl_file {
                ctx.engine.output_close(bbl);
            }
        }
        Err(BibtexError::Fatal) => (),
        Ok(hist) => return hist,
    }

    match ctx.history {
        History::Spotless => (),
        History::WarningIssued(warns) => {
            if warns == 1 {
                ctx.write_logs("(There was 1 warning)\n")
            } else {
                ctx.write_logs(&format!("(There were {warns} warnings)\n"))
            }
        }
        History::ErrorIssued(errs) => {
            if errs == 1 {
                ctx.write_logs("(There was 1 error message)\n")
            } else {
                ctx.write_logs(&format!("(There were {errs} error messages)\n"))
            }
        }
        History::FatalError => {
            ctx.write_logs("(That was a fatal error)\n");
        }
    }

    bib_close_log(ctx);
    ctx.history
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
) {
    let res = hash.lookup_str_insert::<hash::FileExt>(pool, b".aux", ());
    ctx.s_aux_extension = hash.get(res.loc).text();

    hash.lookup_str_insert::<AuxCommand>(pool, b"\\bibdata", AuxCommand::Data);
    hash.lookup_str_insert::<AuxCommand>(pool, b"\\bibstyle", AuxCommand::Style);
    hash.lookup_str_insert::<AuxCommand>(pool, b"\\citation", AuxCommand::Citation);
    hash.lookup_str_insert::<AuxCommand>(pool, b"\\@input", AuxCommand::Input);

    hash.lookup_str_insert::<BstCommand>(pool, b"entry", BstCommand::Entry);
    hash.lookup_str_insert::<BstCommand>(pool, b"execute", BstCommand::Execute);
    hash.lookup_str_insert::<BstCommand>(pool, b"function", BstCommand::Function);
    hash.lookup_str_insert::<BstCommand>(pool, b"integers", BstCommand::Integers);
    hash.lookup_str_insert::<BstCommand>(pool, b"iterate", BstCommand::Iterate);
    hash.lookup_str_insert::<BstCommand>(pool, b"macro", BstCommand::Macro);
    hash.lookup_str_insert::<BstCommand>(pool, b"read", BstCommand::Read);
    hash.lookup_str_insert::<BstCommand>(pool, b"reverse", BstCommand::Reverse);
    hash.lookup_str_insert::<BstCommand>(pool, b"sort", BstCommand::Sort);
    hash.lookup_str_insert::<BstCommand>(pool, b"strings", BstCommand::Strings);

    hash.lookup_str_insert::<BibCommand>(pool, b"comment", BibCommand::Comment);
    hash.lookup_str_insert::<BibCommand>(pool, b"preamble", BibCommand::Preamble);
    hash.lookup_str_insert::<BibCommand>(pool, b"string", BibCommand::String);

    let mut build_in = |pds: &[ASCIICode], builtin| {
        hash.lookup_str_insert(pool, pds, BstFn::Builtin(builtin))
            .loc
    };

    build_in(b"=", BstBuiltin::Eq);
    build_in(b">", BstBuiltin::Gt);
    build_in(b"<", BstBuiltin::Lt);
    build_in(b"+", BstBuiltin::Plus);
    build_in(b"-", BstBuiltin::Minus);
    build_in(b"*", BstBuiltin::Concat);
    build_in(b":=", BstBuiltin::Set);
    build_in(b"add.period$", BstBuiltin::AddPeriod);
    build_in(b"call.type$", BstBuiltin::CallType);
    build_in(b"change.case$", BstBuiltin::ChangeCase);
    build_in(b"chr.to.int$", BstBuiltin::ChrToInt);
    build_in(b"cite$", BstBuiltin::Cite);
    build_in(b"duplicate$", BstBuiltin::Duplicate);
    build_in(b"empty$", BstBuiltin::Empty);
    build_in(b"format.name$", BstBuiltin::FormatName);
    build_in(b"if$", BstBuiltin::If);
    build_in(b"int.to.chr$", BstBuiltin::IntToChr);
    build_in(b"int.to.str$", BstBuiltin::IntToStr);
    build_in(b"missing$", BstBuiltin::Missing);
    build_in(b"newline$", BstBuiltin::Newline);
    build_in(b"num.names$", BstBuiltin::NumNames);
    build_in(b"pop$", BstBuiltin::Pop);
    build_in(b"preamble$", BstBuiltin::Preamble);
    build_in(b"purify$", BstBuiltin::Purify);
    build_in(b"quote$", BstBuiltin::Quote);
    let skip_loc = build_in(b"skip$", BstBuiltin::Skip);
    build_in(b"stack$", BstBuiltin::Stack);
    build_in(b"substring$", BstBuiltin::Substring);
    build_in(b"swap$", BstBuiltin::Swap);
    build_in(b"text.length$", BstBuiltin::TextLength);
    build_in(b"text.prefix$", BstBuiltin::TextPrefix);
    build_in(b"top$", BstBuiltin::Top);
    build_in(b"type$", BstBuiltin::Type);
    build_in(b"warning$", BstBuiltin::Warning);
    build_in(b"while$", BstBuiltin::While);
    build_in(b"width$", BstBuiltin::Width);
    build_in(b"write$", BstBuiltin::Write);

    let res = hash.lookup_str_insert::<hash::Text>(pool, b"", ());
    ctx.s_null = hash.get(res.loc).text();
    let res = hash.lookup_str_insert::<hash::Text>(pool, b"default.type", ());
    ctx.s_default = hash.get(res.loc).text();
    ctx.b_default = skip_loc;

    hash.lookup_str_insert::<ControlSeq>(pool, b"i", ControlSeq::LowerI);
    hash.lookup_str_insert::<ControlSeq>(pool, b"j", ControlSeq::LowerJ);
    hash.lookup_str_insert::<ControlSeq>(pool, b"oe", ControlSeq::LowerOE);
    hash.lookup_str_insert::<ControlSeq>(pool, b"OE", ControlSeq::UpperOE);
    hash.lookup_str_insert::<ControlSeq>(pool, b"ae", ControlSeq::LowerAE);
    hash.lookup_str_insert::<ControlSeq>(pool, b"AE", ControlSeq::UpperAE);
    hash.lookup_str_insert::<ControlSeq>(pool, b"aa", ControlSeq::LowerAA);
    hash.lookup_str_insert::<ControlSeq>(pool, b"AA", ControlSeq::UpperAA);
    hash.lookup_str_insert::<ControlSeq>(pool, b"o", ControlSeq::LowerO);
    hash.lookup_str_insert::<ControlSeq>(pool, b"O", ControlSeq::UpperO);
    hash.lookup_str_insert::<ControlSeq>(pool, b"l", ControlSeq::LowerL);
    hash.lookup_str_insert::<ControlSeq>(pool, b"L", ControlSeq::UpperL);
    hash.lookup_str_insert::<ControlSeq>(pool, b"ss", ControlSeq::LowerSS);

    let num_fields = other.num_fields();
    hash.lookup_str_insert::<BstFn>(pool, b"crossref", BstFn::Field(num_fields));
    other.set_crossref_num(num_fields);
    other.set_num_fields(num_fields + 1);
    other.set_pre_defined_fields(num_fields + 1);

    let num_ent_strs = entries.num_ent_strs();
    hash.lookup_str_insert::<BstFn>(pool, b"sort.key$", BstFn::StrEntry(num_ent_strs));
    entries.set_sort_key_num(num_ent_strs);
    entries.set_num_ent_strs(num_ent_strs + 1);

    hash.lookup_str_insert::<BstFn>(pool, b"entry.max$", BstFn::IntGlbl(ENT_STR_SIZE as i64));

    hash.lookup_str_insert::<BstFn>(pool, b"global.max$", BstFn::IntGlbl(GLOB_STR_SIZE as i64));
}

pub(crate) fn inner_bibtex_main(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    aux_file_name: &CStr,
) -> Result<History, BibtexError> {
    if !ctx.init_stdout() {
        return Ok(History::FatalError);
    }

    pre_def_certain_strings(ctx, globals);
    if get_the_top_level_aux_file_name(ctx, globals, aux_file_name)? != 0 {
        return Ok(History::FatalError);
    }

    if ctx.config.verbose {
        ctx.write_logs("This is BibTeX, Version 0.99d\n");
    } else {
        ctx.write_log_file("This is BibTeX, Version 0.99d\n");
    }

    ctx.write_log_file(&format!(
        "Capacity: max_strings={}, hash_size={}, hash_prime={}\n",
        pool::MAX_STRINGS,
        hash::HASH_SIZE,
        hash::HASH_PRIME
    ));

    if ctx.config.verbose {
        ctx.write_logs("The top-level auxiliary file: ");
        print_aux_name(ctx, globals.pool, globals.aux.top_file().name)?;
    } else {
        ctx.write_log_file("The top-level auxiliary file: ");
        log_pr_aux_name(ctx, globals.aux, globals.pool)?;
    }

    let last_aux = loop {
        globals.aux.top_file_mut().line += 1;

        if !input_ln(
            ctx.engine,
            &mut globals.aux.top_file_mut().file,
            globals.buffers,
        ) {
            if let Some(last) = pop_the_aux_stack(ctx, globals.aux) {
                break last;
            }
        } else {
            get_aux_command_and_process(ctx, globals)?;
        }
    };

    last_check_for_aux_errors(ctx, globals.pool, globals.cites, globals.bibs, last_aux)?;

    if ctx.bst.is_none() {
        return Err(BibtexError::NoBst);
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.init(BufTy::Base));

    let mut exec = ExecCtx::new(ctx);
    loop {
        if !eat_bst_white_space(&mut exec, globals.buffers) {
            break;
        }
        get_bst_command_and_process(&mut exec, globals)?;
    }

    Ok(History::Spotless)
}

pub(crate) fn get_the_top_level_aux_file_name(
    ctx: &mut Bibtex<'_, '_>,
    GlobalItems {
        pool, hash, aux, ..
    }: &mut GlobalItems<'_>,
    aux_file_name: &CStr,
) -> Result<i32, BibtexError> {
    let aux_bytes = aux_file_name.to_bytes_with_nul();

    // This will be our scratch space for CStr filenames
    let mut path = vec![0; aux_bytes.len()];
    // Copy in all but the presumed trailing `.aux`
    path[..aux_bytes.len() - 5].copy_from_slice(&aux_bytes[..aux_bytes.len() - 5]);

    let set_extension = |path: &mut Vec<_>, extension: &[u8]| {
        let range = path.len() - 5..path.len() - 1;
        path[range].copy_from_slice(extension);
    };

    let aux_file = match PeekableInput::open(ctx, aux_file_name, FileFormat::Tex) {
        Ok(file) => file,
        Err(_) => {
            sam_wrong_file_name_print(ctx, aux_file_name);
            return Ok(1);
        }
    };

    set_extension(&mut path, b".blg");
    let log_file = CStr::from_bytes_with_nul(&path).unwrap();
    if !ctx.init_log_file(log_file) {
        sam_wrong_file_name_print(ctx, log_file);
        return Ok(1);
    }

    set_extension(&mut path, b".bbl");
    let bbl_file = CStr::from_bytes_with_nul(&path).unwrap();
    ctx.bbl_file = ctx.engine.output_open(bbl_file.to_str().unwrap(), false);
    if ctx.bbl_file.is_none() {
        sam_wrong_file_name_print(ctx, bbl_file);
        return Ok(1);
    }

    set_extension(&mut path, b".aux");
    let lookup = hash.lookup_str_insert::<hash::AuxFile>(pool, &path[..path.len() - 1], ());

    aux.push_file(File {
        name: hash.get(lookup.loc).text(),
        file: aux_file,
        line: 0,
    });

    if lookup.exists {
        ctx.write_logs("Already encountered auxiliary file");
        print_confusion(ctx);
        return Err(BibtexError::Fatal);
    }

    Ok(0)
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
