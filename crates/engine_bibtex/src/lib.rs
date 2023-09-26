// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]
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
    auxi::{get_aux_command_and_process, last_check_for_aux_errors, pop_the_aux_stack, AuxData},
    bibs::BibData,
    bst::get_bst_command_and_process,
    buffer::{BufTy, GlobalBuffer},
    cite::CiteInfo,
    entries::EntryData,
    exec::ExecCtx,
    external::*,
    global::GlobalData,
    hash::HashData,
    history::{get_history, History},
    log::{
        bib_close_log, init_log_file, init_standard_output, log_pr_aux_name, print_aux_name,
        print_confusion, sam_wrong_file_name_print, write_log_file, write_logs,
    },
    other::OtherData,
    peekable::{input_ln, PeekableInput},
    pool::{pre_def_certain_strings, StringPool},
    scan::eat_bst_white_space,
};
use std::{
    ffi::{CStr, CString},
    ptr,
};
use tectonic_bridge_core::{
    ttbc_input_close, ttbc_input_open, ttbc_output_close, ttbc_output_open,
    ttbc_output_open_stdout, CoreBridgeLauncher, CoreBridgeState, FileFormat,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::OutputHandle;
use xbuf::SafelyZero;

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
pub(crate) mod history;
pub(crate) mod log;
pub(crate) mod other;
pub(crate) mod peekable;
pub(crate) mod pool;
pub(crate) mod scan;
pub(crate) mod xbuf;

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

pub(crate) struct Bibtex<'a, 'cbs> {
    pub engine: &'a mut CoreBridgeState<'cbs>,
    pub config: BibtexConfig,

    pub bst: Option<File>,

    pub bbl_file: *mut OutputHandle,
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

    pub b_default: HashPointer,
    pub s_null: HashPointer,
    pub s_default: HashPointer,
    pub s_aux_extension: HashPointer,
}

impl<'a, 'cbs> Bibtex<'a, 'cbs> {
    pub(crate) fn new(
        engine: &'a mut CoreBridgeState<'cbs>,
        config: BibtexConfig,
    ) -> Bibtex<'a, 'cbs> {
        Bibtex {
            engine,
            config,
            bst: None,
            bbl_file: ptr::null_mut(),
            bbl_line_num: 0,
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
            b_default: 0,
            s_null: 0,
            s_default: 0,
            s_aux_extension: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct LookupRes {
    /// The location of the string - where it exists, was inserted, of if insert is false,
    /// where it *would* have been inserted
    loc: usize,
    /// Whether the string existed in the hash table already
    exists: bool,
}

#[derive(Debug)]
pub(crate) struct FindCiteLocs {
    cite_loc: CiteNumber,
    lc_cite_loc: CiteNumber,

    cite_found: bool,
    lc_found: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum StrIlk {
    Text,
    Integer,
    AuxCommand,
    AuxFile,
    BstCommand,
    BstFile,
    BibFile,
    FileExt,
    Cite,
    LcCite,
    BstFn,
    BibCommand,
    Macro,
    ControlSeq,
}

// SAFETY: StrIlk is valid at zero as StrIlk::Text
unsafe impl SafelyZero for StrIlk {}

type StrNumber = usize;
type CiteNumber = usize;
type ASCIICode = u8;
type BufPointer = usize;
type PoolPointer = usize;
type HashPointer = usize;
type BibNumber = usize;
type WizFnLoc = usize;
type FieldLoc = usize;
type FnDefLoc = usize;

pub(crate) fn bibtex_main(ctx: &mut Bibtex<'_, '_>, aux_file_name: &CStr) -> History {
    history::reset();
    log::reset();

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
            // SAFETY: bst_file guaranteed valid at this point
            ctx.bst.take().map(|file| file.file.close(ctx));
            ttbc_output_close(ctx.engine, ctx.bbl_file);
        }
        Err(BibtexError::NoBst) => {
            ttbc_output_close(ctx.engine, ctx.bbl_file);
        }
        Err(BibtexError::Fatal) => (),
        Ok(hist) => return hist,
    }

    match get_history() {
        History::Spotless => (),
        History::WarningIssued(warns) => {
            if warns == 1 {
                write_logs("(There was 1 warning)\n")
            } else {
                write_logs(&format!("(There were {} warnings)\n", warns))
            }
        }
        History::ErrorIssued(errs) => {
            if errs == 1 {
                write_logs("(There was 1 error message)\n")
            } else {
                write_logs(&format!("(There were {} error messages)\n", errs))
            }
        }
        History::FatalError => {
            write_logs("(That was a fatal error)\n");
        }
    }

    bib_close_log(ctx);
    get_history()
}

pub(crate) fn inner_bibtex_main(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    aux_file_name: &CStr,
) -> Result<History, BibtexError> {
    if !init_standard_output(ctx) {
        return Ok(History::FatalError);
    }

    if initialize(ctx, globals, aux_file_name)? != 0 {
        return Ok(History::FatalError);
    }

    if ctx.config.verbose {
        write_logs("This is BibTeX, Version 0.99d\n");
    } else {
        write_log_file("This is BibTeX, Version 0.99d\n");
    }

    write_log_file(&format!(
        "Capacity: max_strings={}, hash_size={}, hash_prime={}\n",
        pool::MAX_STRINGS,
        hash::HASH_SIZE,
        hash::HASH_PRIME
    ));

    if ctx.config.verbose {
        write_logs("The top-level auxiliary file: ");
        print_aux_name(globals.pool, globals.aux.top_file().name)?;
    } else {
        write_log_file("The top-level auxiliary file: ");
        log_pr_aux_name(globals.aux, globals.pool)?;
    }

    let last_aux = loop {
        globals.aux.top_file_mut().line += 1;

        if !input_ln(&mut globals.aux.top_file_mut().file, globals.buffers) {
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

    ctx.bbl_line_num = 1;
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.init(BufTy::Base));

    let mut exec = ExecCtx::new(ctx);
    loop {
        if !eat_bst_white_space(exec.glbl_ctx_mut(), globals.buffers) {
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
    let ctx = &mut *ctx;
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
            sam_wrong_file_name_print(aux_file_name);
            return Ok(1);
        }
    };

    set_extension(&mut path, b".blg");
    let log_file = CStr::from_bytes_with_nul(&path).unwrap();
    if !init_log_file(ctx, log_file) {
        sam_wrong_file_name_print(log_file);
        return Ok(1);
    }

    set_extension(&mut path, b".bbl");
    let bbl_file = CStr::from_bytes_with_nul(&path).unwrap();
    // SAFETY: Function sound if provided a valid path pointer
    ctx.bbl_file = unsafe { ttbc_output_open(ctx.engine, bbl_file.as_ptr(), 0) };
    if ctx.bbl_file.is_null() {
        sam_wrong_file_name_print(bbl_file);
        return Ok(1);
    }

    set_extension(&mut path, b".aux");
    let lookup = match pool.lookup_str_insert(hash, &path[..path.len() - 1], StrIlk::AuxFile) {
        Ok(res) => res,
        Err(_) => return Err(BibtexError::Fatal),
    };

    aux.push_file(File {
        name: hash.text(lookup.loc),
        file: aux_file,
        line: 0,
    });

    if lookup.exists {
        write_logs("Already encountered auxiliary file");
        print_confusion();
        return Err(BibtexError::Fatal);
    }

    Ok(0)
}

fn initialize(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    aux_file_name: &CStr,
) -> Result<i32, BibtexError> {
    globals.pool.set_pool_ptr(0);
    globals.pool.set_str_ptr(1);
    globals.pool.set_start(globals.pool.str_ptr(), 0);

    ctx.bib_seen = false;
    ctx.bst_seen = false;
    ctx.citation_seen = false;
    ctx.all_entries = false;

    ctx.entry_seen = false;
    ctx.read_seen = false;
    ctx.read_performed = false;
    ctx.reading_completed = false;
    ctx.impl_fn_num = 0;
    globals.buffers.set_init(BufTy::Out, 0);

    pre_def_certain_strings(ctx, globals)?;
    get_the_top_level_aux_file_name(ctx, globals, aux_file_name)
}

mod external {
    #[allow(improper_ctypes)]
    extern "C" {
        pub(crate) fn xrealloc(ptr: *mut libc::c_void, size: libc::size_t) -> *mut libc::c_void;

        pub(crate) fn xcalloc(elems: libc::size_t, elem_size: libc::size_t) -> *mut libc::c_void;
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
