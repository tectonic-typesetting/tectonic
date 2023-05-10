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

use std::ffi::CString;
use tectonic_bridge_core::{CoreBridgeLauncher, EngineAbortedError};
use tectonic_errors::prelude::*;

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
    config: c_api::BibtexConfig,
}

impl BibtexEngine {
    /// Set the BibTeX `min_crossrefs` parameter.
    ///
    /// The default value is 2.
    ///
    /// This needs verifying, but I believe that this setting affects how many
    /// times an item needs to be referenced in directly-referenced BibTeX
    /// entries before it gets its own standalone entry.
    pub fn min_crossrefs(&mut self, value: i32) -> &mut Self {
        self.config.min_crossrefs = value as libc::c_int;
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
        launcher: &mut CoreBridgeLauncher,
        aux: &str,
    ) -> Result<BibtexOutcome> {
        let caux = CString::new(aux)?;

        launcher.with_global_lock(|state| {
            let hist = unsafe { c_api::tt_engine_bibtex_main(state, &self.config, caux.as_ptr()) };

            match hist {
                c_api::History::Spotless => Ok(BibtexOutcome::Spotless),
                c_api::History::WarningIssued => Ok(BibtexOutcome::Warnings),
                c_api::History::ErrorIssued => Ok(BibtexOutcome::Errors),
                c_api::History::FatalError => Err(anyhow!("unspecified fatal bibtex error")),
                c_api::History::Aborted => Err(EngineAbortedError::new_abort_indicator().into()),
            }
        })
    }
}

#[doc(hidden)]
pub mod c_api {
    use crate::c_api::buffer::{bib_buf, bib_buf_size, buffer_overflow, BufTy};
    use std::slice;
    use tectonic_bridge_core::{CoreBridgeState, FileFormat};
    use tectonic_io_base::InputHandle;

    mod buffer;
    mod log;
    mod peekable;

    unsafe fn buf_to_slice<'a>(
        buf: BufType,
        start: BufPointer,
        len: BufPointer,
    ) -> &'a [ASCIICode] {
        slice::from_raw_parts(buf.offset(start as isize), len as usize)
    }

    unsafe fn buf_to_slice_mut<'a>(
        buf: BufType,
        start: BufPointer,
        len: BufPointer,
    ) -> &'a mut [ASCIICode] {
        slice::from_raw_parts_mut(buf.offset(start as isize), len as usize)
    }

    unsafe fn str_to_slice<'a>(
        str_pool: *mut ASCIICode,
        str_start: *mut PoolPointer,
        str: StrNumber,
    ) -> &'a [ASCIICode] {
        let str = str as isize;
        slice::from_raw_parts(
            str_pool.offset(*str_start.offset(str) as isize),
            (*str_start.offset(str + 1) - *str_start.offset(str)) as usize,
        )
    }

    /// cbindgen:rename-all=ScreamingSnakeCase
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[repr(C)]
    pub enum History {
        Spotless = 0,
        WarningIssued = 1,
        ErrorIssued = 2,
        FatalError = 3,
        Aborted = 4,
    }

    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct BibtexConfig {
        pub min_crossrefs: libc::c_int,
    }

    impl Default for BibtexConfig {
        fn default() -> Self {
            BibtexConfig { min_crossrefs: 2 }
        }
    }

    type StrNumber = i32;
    type CiteNumber = i32;
    type ASCIICode = u8;
    type BufType = *mut ASCIICode;
    type BufPointer = i32;
    type PoolPointer = i32;
    type LexType = u8;

    // #[no_mangle]
    // pub unsafe extern "C" fn buffer_overflow() {
    //     bib_xretalloc_noset!(buffer, ASCIICode, buf_size + BUF_SIZE);
    //     bib_xretalloc_noset!(sv_buffer, ASCIICode, buf_size + BUF_SIZE);
    //     bib_xretalloc_noset!(ex_buf, ASCIICode, buf_size + BUF_SIZE);
    //     bib_xretalloc_noset!(out_buf, ASCIICode, buf_size + BUF_SIZE);
    //     bib_xretalloc_noset!(name_tok, BufPointer, buf_size + BUF_SIZE);
    //     bib_xretalloc!(name_sep_char, ASCIICode, buf_size, buf_size + BUF_SIZE);
    // }

    #[no_mangle]
    pub unsafe extern "C" fn str_ends_with(
        str_pool: *mut ASCIICode,
        str_start: *mut PoolPointer,
        s: StrNumber,
        ext: StrNumber,
    ) -> bool {
        let str = str_to_slice(str_pool, str_start, s);
        let ext = str_to_slice(str_pool, str_start, ext);
        str.ends_with(ext)
    }

    #[no_mangle]
    pub unsafe extern "C" fn bib_str_eq_buf(
        str_pool: *mut ASCIICode,
        str_start: *mut PoolPointer,
        s: StrNumber,
        buf: BufType,
        bf_ptr: BufPointer,
        len: BufPointer,
    ) -> bool {
        let buf = buf_to_slice(buf, bf_ptr, len);
        let str = str_to_slice(str_pool, str_start, s);
        buf == str
    }

    #[no_mangle]
    pub unsafe extern "C" fn bib_str_eq_str(
        str_pool: *mut ASCIICode,
        str_start: *mut PoolPointer,
        s1: StrNumber,
        s2: StrNumber,
    ) -> bool {
        let str1 = str_to_slice(str_pool, str_start, s1);
        let str2 = str_to_slice(str_pool, str_start, s2);
        str1 == str2
    }

    #[no_mangle]
    pub unsafe extern "C" fn lower_case(buf: BufType, bf_ptr: BufPointer, len: BufPointer) {
        let buf = buf_to_slice_mut(buf, bf_ptr, len);
        buf.make_ascii_lowercase();
    }

    #[no_mangle]
    pub unsafe extern "C" fn upper_case(buf: BufType, bf_ptr: BufPointer, len: BufPointer) {
        let buf = buf_to_slice_mut(buf, bf_ptr, len);
        buf.make_ascii_uppercase();
    }

    /// # Safety
    ///
    /// Passed pointer must point to a valid array that we have exclusive access to for the duration
    /// of this call, that is at least as long as `right_end`, and initialized for the range
    /// `ptr[left_end..right_end]`
    #[no_mangle]
    pub unsafe extern "C" fn quick_sort(
        cite_info: *mut StrNumber,
        left_end: CiteNumber,
        right_end: CiteNumber,
    ) {
        let slice = slice::from_raw_parts_mut(
            cite_info.add(left_end as usize),
            (right_end - left_end) as usize,
        );
        slice.sort();
    }

    #[no_mangle]
    pub unsafe extern "C" fn int_to_ascii(
        mut the_int: i32,
        int_buf: BufTy,
        int_begin: BufPointer,
        int_end: *mut BufPointer,
    ) {
        let buf = bib_buf(int_buf);
        let mut int_ptr = int_begin;
        let mut int_xptr = int_begin;

        if the_int < 0 {
            {
                if int_ptr == bib_buf_size() {
                    buffer_overflow();
                }
                *buf.offset(int_ptr as isize) = 45 /*minus_sign */ ;
                int_ptr += 1;
            }
            the_int = -the_int;
        }

        loop {
            if int_ptr == bib_buf_size() {
                buffer_overflow();
            }
            *buf.offset(int_ptr as isize) = b'0' + (the_int % 10) as u8;
            int_ptr += 1;
            the_int /= 10;
            if the_int == 0 {
                break;
            }
        }

        *int_end = int_ptr;
        int_ptr -= 1;
        while int_xptr < int_ptr {
            let int_tmp_val = *buf.offset(int_xptr as isize);
            *buf.offset(int_xptr as isize) = *buf.offset(int_ptr as isize);
            *buf.offset(int_ptr as isize) = int_tmp_val;
            int_ptr -= 1;
            int_xptr += 1;
        }
    }

    #[allow(improper_ctypes)] // for CoreBridgeState
    extern "C" {
        pub fn tt_engine_bibtex_main(
            api: &mut CoreBridgeState,
            cfg: &BibtexConfig,
            aux_name: *const libc::c_char,
        ) -> History;
    }

    /// cbindgen:ignore
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn ttstub_input_open(
            path: *const libc::c_char,
            format: FileFormat,
            is_gz: libc::c_int,
        ) -> *mut InputHandle;
        pub fn ttstub_input_close(input: *mut InputHandle) -> libc::c_int;
        pub fn ttstub_input_getc(input: *mut InputHandle) -> libc::c_int;

        pub fn xrealloc(ptr: *mut libc::c_void, size: libc::size_t) -> *mut libc::c_void;

        pub fn xcalloc(elems: libc::size_t, elem_size: libc::size_t) -> *mut libc::c_void;
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
