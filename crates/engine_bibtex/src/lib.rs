// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

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
//!
//! If you change the interfaces here, rerun cbindgen as described in the README!

use crate::c_api::history::History;
use std::ffi::CString;
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError};
use tectonic_errors::prelude::*;

#[doc(hidden)]
#[derive(Debug)]
pub enum BibtexError {
    Fatal,
    Recover,
}

/// A possible outcome from a BibTeX engine invocation.
///
/// The classic TeX implementation provides a fourth outcome: “fatal error”. In
/// Tectonic, this outcome is represented as an `Err` result rather than a
/// [`BibtexOutcome`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BibtexOutcome {
    /// Nothing bad happened.
    Spotless = 0,

    /// Warnings were issued.
    Warnings = 1,

    /// Errors occurred. Note that, in TeX usage, “errors” are not necessarily
    /// *fatal* errors: the engine will proceed and work around errors as best
    /// it can.
    Errors = 2,
}

/// A struct for invoking the BibTeX engine.
///
/// This struct has a fairly straightforward “builder” interface: you create it,
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
    ctx: c_api::Bibtex,
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
        self.ctx.config.min_crossrefs = value;
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
            // SAFETY: We're in the global lock, so this is unique access
            let hist = unsafe { c_api::tt_engine_bibtex_main(state, &mut self.ctx, caux.as_ptr()) };

            match hist {
                History::Spotless => Ok(BibtexOutcome::Spotless),
                History::WarningIssued => Ok(BibtexOutcome::Warnings),
                History::ErrorIssued => Ok(BibtexOutcome::Errors),
                History::FatalError => Err(anyhow!("unspecified fatal bibtex error")),
                History::Aborted => Err(EngineAbortedError::new_abort_indicator().into()),
            }
        })
    }
}

#[doc(hidden)]
#[allow(clippy::assertions_on_constants)]
pub mod c_api {
    use crate::BibtexError;
    use auxi::with_aux_mut;
    use buffer::{with_buffers, BufTy};
    use hash::{with_hash, with_hash_mut};
    use history::History;
    use log::{init_log_file, print_confusion, sam_wrong_file_name_print, write_logs};
    use peekable::PeekableInput;
    use pool::{with_pool, with_pool_mut};
    use std::{
        ffi::CStr,
        ptr::{self, NonNull},
    };
    use tectonic_bridge_core::{CoreBridgeState, FileFormat};
    use tectonic_io_base::{InputHandle, OutputHandle};
    use xbuf::{xcalloc_zeroed, SafelyZero};

    pub mod auxi;
    pub mod bibs;
    pub mod buffer;
    pub mod char_info;
    pub mod cite;
    pub mod entries;
    pub mod exec;
    pub mod global;
    pub mod hash;
    pub mod history;
    pub mod log;
    pub mod other;
    pub mod peekable;
    pub mod pool;
    pub mod scan;
    pub mod xbuf;

    pub use external::*;

    // These used to be 'bad' checks at the start of a program, now we can ensure them at comptime
    const _: () = assert!(hash::HASH_PRIME >= 128);
    const _: () = assert!(pool::MAX_PRINT_LINE > pool::MIN_PRINT_LINE);
    const _: () = assert!(pool::MIN_PRINT_LINE >= 3);
    const _: () = assert!(pool::MAX_PRINT_LINE < buffer::BUF_SIZE + 1);
    const _: () = assert!(hash::HASH_PRIME <= hash::HASH_SIZE);
    const _: () = assert!(pool::MAX_STRINGS <= hash::HASH_SIZE);
    const _: () = assert!(cite::MAX_CITES <= pool::MAX_STRINGS);

    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct BibtexConfig {
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

    #[derive(Debug)]
    #[repr(C)]
    pub struct Bibtex {
        pub config: BibtexConfig,
        pub bst_file: Option<NonNull<PeekableInput>>,
        pub bst_str: StrNumber,
        pub bst_line_num: usize,

        pub bbl_file: *mut OutputHandle,
        pub bbl_line_num: usize,

        pub num_bib_files: usize,
        pub num_preamble_strings: usize,
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

    impl Default for Bibtex {
        fn default() -> Self {
            Bibtex {
                config: Default::default(),
                bst_file: None,
                bst_str: 0,
                bst_line_num: 0,
                bbl_file: ptr::null_mut(),
                bbl_line_num: 0,
                num_bib_files: 0,
                num_preamble_strings: 0,
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

    /// cbindgen:field-names=[name_of_file, name_length]
    #[repr(C)]
    pub struct NameAndLen {
        name: *mut ASCIICode,
        len: i32,
    }

    #[repr(C)]
    #[derive(PartialEq)]
    pub enum CResult {
        Error,
        Recover,
        Ok,
    }

    impl From<CResult> for Result<(), BibtexError> {
        fn from(value: CResult) -> Self {
            match value {
                CResult::Error => Err(BibtexError::Fatal),
                CResult::Recover => Err(BibtexError::Recover),
                CResult::Ok => Ok(()),
            }
        }
    }

    impl From<Result<(), BibtexError>> for CResult {
        fn from(value: Result<(), BibtexError>) -> Self {
            match value {
                Ok(()) => CResult::Ok,
                Err(BibtexError::Fatal) => CResult::Error,
                Err(BibtexError::Recover) => CResult::Recover,
            }
        }
    }

    #[repr(C)]
    pub enum CResultStr {
        Error,
        Recover,
        Ok(StrNumber),
    }

    #[repr(C)]
    pub enum CResultBool {
        Error,
        Recover,
        Ok(bool),
    }

    impl From<Result<bool, BibtexError>> for CResultBool {
        fn from(value: Result<bool, BibtexError>) -> Self {
            match value {
                Ok(val) => CResultBool::Ok(val),
                Err(BibtexError::Fatal) => CResultBool::Error,
                Err(BibtexError::Recover) => CResultBool::Recover,
            }
        }
    }

    #[repr(C)]
    pub struct LookupRes {
        /// The location of the string - where it exists, was inserted, of if insert is false,
        /// where it *would* have been inserted
        loc: usize,
        /// Whether the string existed in the hash table already
        exists: bool,
    }

    #[repr(C)]
    pub enum CResultLookup {
        Error,
        Ok(LookupRes),
    }

    impl From<Result<LookupRes, BibtexError>> for CResultLookup {
        fn from(value: Result<LookupRes, BibtexError>) -> Self {
            match value {
                Ok(val) => CResultLookup::Ok(val),
                Err(_) => CResultLookup::Error,
            }
        }
    }

    #[repr(C)]
    pub struct FindCiteLocs {
        cite_loc: CiteNumber,
        lc_cite_loc: CiteNumber,

        cite_found: bool,
        lc_found: bool,
    }

    /// cbindgen:rename-all=ScreamingSnakeCase
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum StrIlk {
        Text = 0,
        Integer = 1,
        AuxCommand = 2,
        AuxFile = 3,
        BstCommand = 4,
        BstFile = 5,
        BibFile = 6,
        FileExt = 7,
        // 8 isn't used
        Cite = 9,
        LcCite = 10,
        BstFn = 11,
        BibCommand = 12,
        Macro = 13,
        ControlSeq = 14,
    }

    // SAFETY: StrIlk is valid at zero as StrIlk::Text
    unsafe impl SafelyZero for StrIlk {}

    type StrNumber = usize;
    type CiteNumber = usize;
    type ASCIICode = u8;
    type BufPointer = usize;
    type PoolPointer = usize;
    type HashPointer = usize;
    type AuxNumber = usize;
    type BibNumber = usize;
    type WizFnLoc = usize;
    type FieldLoc = usize;
    type FnDefLoc = usize;

    #[no_mangle]
    pub unsafe extern "C" fn reset_all() {
        log::reset();
        pool::reset();
        history::reset();
        buffer::reset();
        cite::reset();
        auxi::reset();
        bibs::reset();
        hash::reset();
        other::reset();
        entries::reset();
        global::reset();
    }

    #[no_mangle]
    pub extern "C" fn bib_str_eq_buf(
        s: StrNumber,
        buf: BufTy,
        ptr: BufPointer,
        len: BufPointer,
    ) -> bool {
        with_buffers(|buffers| {
            with_pool(|pool| {
                let buf = &buffers.buffer(buf)[ptr..(ptr + len)];
                let str = pool.get_str(s);
                buf == str
            })
        })
    }

    #[no_mangle]
    pub extern "C" fn start_name(file_name: StrNumber) -> NameAndLen {
        with_pool(|pool| {
            let file_name = pool.get_str(file_name);
            let new_name = xcalloc_zeroed::<ASCIICode>(file_name.len() + 1).unwrap();
            new_name[..file_name.len()].copy_from_slice(file_name);
            new_name[file_name.len()] = 0;
            NameAndLen {
                name: (new_name as *mut [_]).cast(),
                len: file_name.len() as i32,
            }
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_the_top_level_aux_file_name(
        ctx: *mut Bibtex,
        aux_file_name: *const libc::c_char,
    ) -> CResultStr {
        let ctx = &mut *ctx;
        let aux_file_name = CStr::from_ptr(aux_file_name);
        let aux_bytes = aux_file_name.to_bytes_with_nul();

        // This will be our scratch space for CStr filenames
        let mut path = vec![0; aux_bytes.len()];
        // Copy in all but the presumed trailing `.aux`
        path[..aux_bytes.len() - 5].copy_from_slice(&aux_bytes[..aux_bytes.len() - 5]);

        let set_extension = |path: &mut Vec<_>, extension: &[u8]| {
            let range = path.len() - 5..path.len() - 1;
            path[range].copy_from_slice(extension);
        };

        with_aux_mut(|aux| {
            aux.set_ptr(0);

            let aux_file = match PeekableInput::open(aux_file_name, FileFormat::Tex) {
                Ok(file) => file,
                Err(_) => {
                    sam_wrong_file_name_print(aux_file_name);
                    return CResultStr::Ok(1);
                }
            };
            aux.set_file_at_ptr(Box::into_raw(aux_file));

            set_extension(&mut path, b".blg");
            let log_file = CStr::from_bytes_with_nul(&path).unwrap();
            if !init_log_file(log_file) {
                sam_wrong_file_name_print(log_file);
                return CResultStr::Ok(1);
            }

            set_extension(&mut path, b".bbl");
            let bbl_file = CStr::from_bytes_with_nul(&path).unwrap();
            ctx.bbl_file = ttstub_output_open(bbl_file.as_ptr(), 0);
            if ctx.bbl_file.is_null() {
                sam_wrong_file_name_print(bbl_file);
                return CResultStr::Ok(1);
            }

            set_extension(&mut path, b".aux");
            let lookup = match with_hash_mut(|hash| {
                with_pool_mut(|pool| {
                    pool.lookup_str_insert(hash, &path[..path.len() - 1], StrIlk::AuxFile)
                })
            }) {
                Ok(res) => res,
                Err(_) => return CResultStr::Error,
            };
            aux.set_at_ptr(with_hash(|hash| hash.text(lookup.loc)));

            if lookup.exists {
                write_logs("Already encountered auxiliary file");
                print_confusion();
                return CResultStr::Error;
            }

            CResultStr::Ok(0)
        })
    }

    #[allow(improper_ctypes)] // for CoreBridgeState
    extern "C" {
        pub fn tt_engine_bibtex_main(
            api: &mut CoreBridgeState<'_>,
            ctx: &mut Bibtex,
            aux_name: *const libc::c_char,
        ) -> History;
    }

    /// cbindgen:ignore
    mod external {
        use super::*;

        #[allow(improper_ctypes)]
        extern "C" {
            pub fn ttstub_input_open(
                path: *const libc::c_char,
                format: FileFormat,
                is_gz: libc::c_int,
            ) -> *mut InputHandle;
            pub fn ttstub_input_close(input: *mut InputHandle) -> libc::c_int;
            pub fn ttstub_output_open_stdout() -> *mut OutputHandle;
            pub fn ttstub_output_open(
                path: *const libc::c_char,
                is_gz: libc::c_int,
            ) -> *mut OutputHandle;
            pub fn ttstub_output_close(handle: *mut OutputHandle) -> libc::c_int;

            pub fn xrealloc(ptr: *mut libc::c_void, size: libc::size_t) -> *mut libc::c_void;

            pub fn xcalloc(elems: libc::size_t, elem_size: libc::size_t) -> *mut libc::c_void;
        }
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
