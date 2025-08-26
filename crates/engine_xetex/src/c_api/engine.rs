use crate::c_api::globals::Globals;
use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::{ptr, slice};

mod memory;

use crate::c_api::pool::{
    rs_make_string, rs_search_string, rs_slow_make_string, StringPool, EMPTY_STRING, TOO_BIG_CHAR,
};
pub use memory::*;

pub const NULL_CS: usize = 0x220001;
pub const PRIM_SIZE: usize = 2100;
pub const UNDEFINED_CONTROL_SEQUENCE: usize = 0x226603;
pub const FROZEN_NULL_FONT: usize = 0x2242da;
pub const DIMEN_VAL_LIMIT: usize = 128;

pub const TEXT_SIZE: usize = 0;
pub const SCRIPT_SIZE: usize = 256;
pub const SCRIPT_SCRIPT_SIZE: usize = 512;

thread_local! {
    pub static ENGINE_CTX: RefCell<EngineCtx> = RefCell::new(EngineCtx::new())
}

pub struct EngineCtx {
    pub(crate) selector: Selector,
    pub(crate) tally: i32,
    pub(crate) error_line: i32,
    pub(crate) trick_count: i32,
    pub(crate) trick_buf: [u16; 256],
    pub(crate) eqtb_top: i32,
    pub(crate) name_of_file: Option<CString>,
    pub(crate) name_of_file_utf16: Option<Vec<u16>>,
    pub(crate) file_name_quote_char: u16,
    pub(crate) cur_name: StrNumber,
    pub(crate) cur_area: StrNumber,
    pub(crate) cur_ext: StrNumber,
    pub(crate) job_name: StrNumber,
    pub(crate) area_delimiter: usize,
    pub(crate) ext_delimiter: usize,
    pub(crate) name_in_progress: bool,
    pub(crate) stop_at_space: bool,
    pub(crate) quoted_filename: bool,

    pub(crate) eqtb: Vec<MemoryWord>,
    pub(crate) prim: Box<[B32x2; PRIM_SIZE + 1]>,
    /// An arena of TeX nodes
    pub(crate) mem: Vec<MemoryWord>,
}

struct NodeError {
    ty: u16,
    subty: u16,
}

impl EngineCtx {
    fn new() -> EngineCtx {
        EngineCtx {
            selector: Selector::File(0),
            tally: 0,
            error_line: 0,
            trick_count: 0,
            trick_buf: [0; 256],
            eqtb_top: 0,
            name_of_file: None,
            name_of_file_utf16: None,
            file_name_quote_char: 0,
            cur_area: 0,
            cur_ext: 0,
            cur_name: 0,
            job_name: 0,
            area_delimiter: 0,
            ext_delimiter: 0,
            name_in_progress: false,
            stop_at_space: false,
            quoted_filename: false,

            eqtb: Vec::new(),
            prim: Box::new([B32x2 { s0: 0, s1: 0 }; PRIM_SIZE + 1]),
            mem: Vec::new(),
        }
    }

    pub fn raw_mem(&self, idx: usize) -> MemoryWord {
        self.mem[idx]
    }

    pub fn try_node<T: ?Sized + Node>(&self, idx: usize) -> Result<&T, NodeError> {
        let ptr = self.mem.as_ptr().wrapping_add(idx);
        let base = unsafe { &*NodeBase::from_ptr(ptr) };

        if T::ty() != base.ty() || T::subty().is_some_and(|subty| subty != base.subty()) {
            return Err(NodeError {
                ty: base.ty(),
                subty: base.subty(),
            });
        }

        let ptr = unsafe { T::from_ptr(ptr) };
        Ok(unsafe { &*ptr })
    }

    pub fn base_node(&self, idx: usize) -> &NodeBase {
        let ptr = self.mem.as_ptr().wrapping_add(idx);
        let ptr = NodeBase::from_ptr(ptr);
        unsafe { &*ptr }
    }

    pub fn node<T: ?Sized + Node>(&self, idx: usize) -> &T {
        match self.try_node::<T>(idx) {
            Ok(node) => node,
            Err(e) => {
                panic!(
                    "Invalid node type. expected {}:{:?}, found {}:{}",
                    T::ty(),
                    T::subty(),
                    e.ty,
                    e.subty,
                );
            }
        }
    }

    pub fn int_par(&self, par: IntPar) -> i32 {
        unsafe { self.eqtb[INT_BASE + par as usize].b32.s1 }
    }

    pub fn set_int_par(&mut self, par: IntPar, val: i32) {
        self.eqtb[INT_BASE + par as usize].b32.s1 = val
    }

    pub fn cat_code(&self, p: usize) -> Result<CatCode, i32> {
        let val = unsafe { self.eqtb[CAT_CODE_BASE + p].b32.s1 };
        CatCode::try_from(val)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Selector {
    File(u8),
    NoPrint,
    TermOnly,
    LogOnly,
    TermAndLog,
    Pseudo,
    NewString,
}

impl From<Selector> for u32 {
    fn from(value: Selector) -> Self {
        match value {
            Selector::File(val) => val as u32,
            Selector::NoPrint => 16,
            Selector::TermOnly => 17,
            Selector::LogOnly => 18,
            Selector::TermAndLog => 19,
            Selector::Pseudo => 20,
            Selector::NewString => 21,
        }
    }
}

impl TryFrom<u32> for Selector {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            val @ 0..16 => Ok(Selector::File(val as u8)),
            16 => Ok(Selector::NoPrint),
            17 => Ok(Selector::TermOnly),
            18 => Ok(Selector::LogOnly),
            19 => Ok(Selector::TermAndLog),
            20 => Ok(Selector::Pseudo),
            21 => Ok(Selector::NewString),
            _ => Err(()),
        }
    }
}

#[no_mangle]
pub extern "C" fn selector() -> u32 {
    ENGINE_CTX.with_borrow(|engine| engine.selector.into())
}

#[no_mangle]
pub extern "C" fn set_selector(val: u32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.selector = Selector::try_from(val).unwrap());
}

#[no_mangle]
pub extern "C" fn tally() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.tally)
}

#[no_mangle]
pub extern "C" fn set_tally(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.tally = val)
}

#[no_mangle]
pub extern "C" fn error_line() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.error_line)
}

#[no_mangle]
pub extern "C" fn set_error_line(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.error_line = val)
}

#[no_mangle]
pub extern "C" fn trick_count() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.trick_count)
}

#[no_mangle]
pub extern "C" fn set_trick_count(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.trick_count = val)
}

#[no_mangle]
pub extern "C" fn trick_buf(idx: usize) -> u16 {
    ENGINE_CTX.with_borrow(|engine| engine.trick_buf[idx])
}

#[no_mangle]
pub extern "C" fn set_trick_buf(idx: usize, val: u16) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.trick_buf[idx] = val)
}

#[no_mangle]
pub extern "C" fn eqtb_top() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.eqtb_top)
}

#[no_mangle]
pub extern "C" fn set_eqtb_top(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.eqtb_top = val)
}

#[no_mangle]
pub extern "C" fn name_length() -> usize {
    ENGINE_CTX.with_borrow(|engine| {
        engine
            .name_of_file
            .as_ref()
            .map(|s| s.count_bytes())
            .unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn name_of_file() -> *const libc::c_char {
    ENGINE_CTX.with_borrow(|engine| {
        engine
            .name_of_file
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(ptr::null())
    })
}

#[no_mangle]
pub extern "C" fn set_name_of_file(val: *const libc::c_char) {
    let s = if val.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(val) })
    };
    ENGINE_CTX.with_borrow_mut(|engine| engine.name_of_file = s.map(CStr::to_owned))
}

#[no_mangle]
pub extern "C" fn name_length16() -> usize {
    ENGINE_CTX.with_borrow(|engine| {
        engine
            .name_of_file_utf16
            .as_ref()
            .map(|s| s.len())
            .unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn name_of_file16() -> *const u16 {
    ENGINE_CTX.with_borrow(|engine| {
        engine
            .name_of_file_utf16
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(ptr::null())
    })
}

#[no_mangle]
pub extern "C" fn set_name_of_file16(val: *const u16, len: usize) {
    let s = if val.is_null() {
        None
    } else {
        Some(unsafe { slice::from_raw_parts(val, len) })
    };
    ENGINE_CTX.with_borrow_mut(|engine| engine.name_of_file_utf16 = s.map(<[u16]>::to_owned))
}

#[no_mangle]
pub extern "C" fn cur_name() -> StrNumber {
    ENGINE_CTX.with_borrow(|engine| engine.cur_name)
}

#[no_mangle]
pub extern "C" fn set_cur_name(val: StrNumber) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.cur_name = val)
}

#[no_mangle]
pub extern "C" fn cur_area() -> StrNumber {
    ENGINE_CTX.with_borrow(|engine| engine.cur_area)
}

#[no_mangle]
pub extern "C" fn set_cur_area(val: StrNumber) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.cur_area = val)
}

#[no_mangle]
pub extern "C" fn cur_ext() -> StrNumber {
    ENGINE_CTX.with_borrow(|engine| engine.cur_ext)
}

#[no_mangle]
pub extern "C" fn set_cur_ext(val: StrNumber) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.cur_ext = val)
}

#[no_mangle]
pub extern "C" fn job_name() -> StrNumber {
    ENGINE_CTX.with_borrow(|engine| engine.job_name)
}

#[no_mangle]
pub extern "C" fn set_job_name(val: StrNumber) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.job_name = val)
}

#[no_mangle]
pub extern "C" fn area_delimiter() -> usize {
    ENGINE_CTX.with_borrow(|engine| engine.area_delimiter)
}

#[no_mangle]
pub extern "C" fn set_area_delimiter(val: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.area_delimiter = val)
}

#[no_mangle]
pub extern "C" fn ext_delimiter() -> usize {
    ENGINE_CTX.with_borrow(|engine| engine.ext_delimiter)
}

#[no_mangle]
pub extern "C" fn set_ext_delimiter(val: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.ext_delimiter = val)
}

#[no_mangle]
pub extern "C" fn name_in_progress() -> bool {
    ENGINE_CTX.with_borrow(|engine| engine.name_in_progress)
}

#[no_mangle]
pub extern "C" fn set_name_in_progress(val: bool) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.name_in_progress = val)
}

#[no_mangle]
pub extern "C" fn stop_at_space() -> bool {
    ENGINE_CTX.with_borrow(|engine| engine.stop_at_space)
}

#[no_mangle]
pub extern "C" fn set_stop_at_space(val: bool) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.stop_at_space = val)
}

#[no_mangle]
pub extern "C" fn file_name_quote_char() -> u16 {
    ENGINE_CTX.with_borrow(|engine| engine.file_name_quote_char)
}

#[no_mangle]
pub extern "C" fn set_file_name_quote_char(val: u16) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.file_name_quote_char = val)
}

#[no_mangle]
pub extern "C" fn quoted_filename() -> bool {
    ENGINE_CTX.with_borrow(|engine| engine.quoted_filename)
}

#[no_mangle]
pub extern "C" fn set_quoted_filename(val: bool) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.quoted_filename = val)
}

#[no_mangle]
pub extern "C" fn eqtb(idx: usize) -> MemoryWord {
    ENGINE_CTX.with_borrow(|engine| engine.eqtb[idx])
}

#[no_mangle]
pub extern "C" fn set_eqtb(idx: usize, val: MemoryWord) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.eqtb[idx] = val)
}

#[no_mangle]
pub extern "C" fn eqtb_ptr(idx: usize) -> *mut MemoryWord {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.eqtb[idx]))
}

#[no_mangle]
pub extern "C" fn resize_eqtb(len: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| {
        engine.eqtb.resize(
            len,
            MemoryWord {
                ptr: ptr::null_mut(),
            },
        )
    })
}

#[no_mangle]
pub extern "C" fn clear_eqtb() {
    ENGINE_CTX.with_borrow_mut(|engine| engine.eqtb.clear())
}

#[no_mangle]
pub extern "C" fn mem(idx: usize) -> MemoryWord {
    ENGINE_CTX.with_borrow(|engine| engine.mem[idx])
}

#[no_mangle]
pub extern "C" fn set_mem(idx: usize, val: MemoryWord) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.mem[idx] = val)
}

#[no_mangle]
pub extern "C" fn mem_ptr(idx: usize) -> *mut MemoryWord {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.mem[idx]))
}

#[no_mangle]
pub extern "C" fn resize_mem(len: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| {
        engine.mem.resize(
            len,
            MemoryWord {
                ptr: ptr::null_mut(),
            },
        )
    })
}

#[no_mangle]
pub extern "C" fn clear_mem() {
    ENGINE_CTX.with_borrow_mut(|engine| engine.mem.clear())
}

#[no_mangle]
pub extern "C" fn prim(idx: usize) -> B32x2 {
    ENGINE_CTX.with_borrow(|engine| engine.prim[idx])
}

#[no_mangle]
pub extern "C" fn set_prim(idx: usize, val: B32x2) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.prim[idx] = val)
}

#[no_mangle]
pub extern "C" fn prim_ptr(idx: usize) -> *mut B32x2 {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.prim[idx]))
}

fn checkpool_pointer(pool: &mut StringPool, pool_ptr: usize, len: usize) {
    if pool_ptr + len >= pool.pool_size {
        panic!("string pool overflow [{} bytes]", pool.pool_size);
    }
}

pub fn rs_maketexstring(globals: &mut Globals<'_, '_>, str: &str) -> StrNumber {
    if str.len() == 0 {
        return EMPTY_STRING;
    }

    checkpool_pointer(globals.strings, globals.strings.pool_ptr, str.len());

    for b in str.encode_utf16() {
        globals.strings.str_pool[globals.strings.pool_ptr] = b;
        globals.strings.pool_ptr += 1;
    }

    rs_make_string(globals.strings)
}

pub fn rs_gettexstring(globals: &mut Globals<'_, '_>, s: StrNumber) -> String {
    if s < 0x10000 {
        return String::new();
    }

    let str = globals.strings.str(s - 0x10000);

    String::from_utf16_lossy(str)
}

pub fn rs_pack_file_name(globals: &mut Globals<'_, '_>, n: StrNumber, a: StrNumber, e: StrNumber) {
    let n = globals.strings.tex_str(n);
    let a = globals.strings.tex_str(a);
    let e = globals.strings.tex_str(e);
    let mut buffer = String::with_capacity(n.len() + a.len() + e.len());

    let iter = a.iter().chain(n).chain(e).copied();
    for c in char::decode_utf16(iter) {
        let c = c.unwrap_or(char::REPLACEMENT_CHARACTER);
        buffer.push(c);
    }

    globals.engine.name_of_file = Some(CString::new(buffer).unwrap());
}

pub fn rs_pack_job_name(globals: &mut Globals<'_, '_>, s: &str) {
    globals.engine.cur_area = EMPTY_STRING;
    globals.engine.cur_ext = rs_maketexstring(globals, s);
    globals.engine.cur_name = globals.engine.job_name;
    rs_pack_file_name(
        globals,
        globals.engine.cur_name,
        globals.engine.cur_area,
        globals.engine.cur_ext,
    );
}

pub fn rs_make_utf16_name(engine: &mut EngineCtx) {
    engine.name_of_file_utf16 = engine
        .name_of_file
        .as_ref()
        .and_then(|name| name.to_str().ok())
        .map(|s| s.encode_utf16().collect());
}

pub fn rs_begin_name(globals: &mut Globals<'_, '_>) {
    globals.engine.area_delimiter = 0;
    globals.engine.ext_delimiter = 0;
    globals.engine.quoted_filename = false;
    globals.engine.file_name_quote_char = 0;
}

pub fn rs_end_name(globals: &mut Globals<'_, '_>) {
    if globals.strings.str_ptr + 3 > globals.strings.max_strings {
        todo!("overflow(\"number of strings\", max_strings() - init_str_ptr)");
    }

    /* area_delimiter is the length from the start of the filename to the
     * directory seperator "/", which we use to construct the stringpool
     * string `cur_area`. If there was already a string in the stringpool for
     * the area, reuse it. */

    if globals.engine.area_delimiter == 0 {
        globals.engine.cur_area = EMPTY_STRING;
    } else {
        globals.engine.cur_area = globals.strings.str_ptr as StrNumber;
        globals.strings.str_start[globals.strings.str_ptr + 1 - 0x10000] =
            globals.strings.str_start[globals.strings.str_ptr - TOO_BIG_CHAR]
                + globals.engine.area_delimiter as u32;
        globals.strings.str_ptr += 1;

        let temp_str = rs_search_string(globals.strings, globals.engine.cur_area);

        if temp_str > 0 {
            globals.engine.cur_area = temp_str;
            globals.strings.str_ptr -= 1;

            for j in (globals.strings.str_start[globals.strings.str_ptr + 1 - 0x10000] as usize)
                ..globals.strings.pool_ptr
            {
                globals.strings.str_pool[j - globals.engine.area_delimiter] =
                    globals.strings.str_pool[j];
            }

            globals.strings.pool_ptr -= globals.engine.area_delimiter;
        }
    }

    /* ext_delimiter is the length from the start of the filename to the
     * extension '.' delimiter, which we use to construct the stringpool
     * strings `cur_ext` and `cur_name`. */

    if globals.engine.ext_delimiter == 0 {
        globals.engine.cur_ext = EMPTY_STRING;
        globals.engine.cur_name = rs_slow_make_string(globals.strings);
    } else {
        globals.engine.cur_name = globals.strings.str_ptr as StrNumber;
        globals.strings.str_start[globals.strings.str_ptr + 1 - 0x10000] =
            globals.strings.str_start[globals.strings.str_ptr - TOO_BIG_CHAR]
                + globals.engine.ext_delimiter as u32
                - globals.engine.area_delimiter as u32
                - 1;
        globals.strings.str_ptr += 1;

        globals.engine.cur_ext = rs_make_string(globals.strings);
        globals.strings.str_ptr -= 1;

        let temp_str = rs_search_string(globals.strings, globals.engine.cur_name);

        if temp_str > 0 {
            globals.engine.cur_name = temp_str;
            globals.strings.str_ptr -= 1;

            for j in (globals.strings.str_start[globals.strings.str_ptr + 1 - 0x10000] as usize)
                ..globals.strings.pool_ptr
            {
                globals.strings.str_pool
                    [j - globals.engine.ext_delimiter + globals.engine.area_delimiter + 1] =
                    globals.strings.str_pool[j];
            }

            globals.strings.pool_ptr -=
                globals.engine.ext_delimiter - globals.engine.area_delimiter - 1;
        }
    }

    globals.engine.cur_ext = rs_slow_make_string(globals.strings);
}

#[no_mangle]
pub unsafe extern "C" fn maketexstring(str: *const libc::c_char) -> StrNumber {
    if str.is_null() {
        return EMPTY_STRING;
    }
    let str = unsafe { CStr::from_ptr(str) }.to_string_lossy();
    Globals::with(|globals| rs_maketexstring(globals, &str))
}

#[no_mangle]
pub unsafe extern "C" fn gettexstring(s: StrNumber) -> *mut libc::c_char {
    let str = Globals::with(|globals| rs_gettexstring(globals, s));
    let out = unsafe { libc::malloc(str.len() + 1) }.cast::<libc::c_char>();
    unsafe { ptr::copy_nonoverlapping(str.as_ptr().cast(), out, str.len()) };
    unsafe { out.add(str.len()).write(0) };
    out
}

#[no_mangle]
pub extern "C" fn pack_file_name(n: StrNumber, a: StrNumber, e: StrNumber) {
    Globals::with(|globals| rs_pack_file_name(globals, n, a, e))
}

#[no_mangle]
pub extern "C" fn pack_job_name(s: *const libc::c_char) {
    let s = unsafe { CStr::from_ptr(s) }.to_str().unwrap();
    Globals::with(|globals| rs_pack_job_name(globals, s))
}

#[no_mangle]
pub extern "C" fn make_utf16_name() {
    ENGINE_CTX.with_borrow_mut(|engine| rs_make_utf16_name(engine))
}

#[no_mangle]
pub extern "C" fn begin_name() {
    Globals::with(|globals| rs_begin_name(globals))
}

#[no_mangle]
pub extern "C" fn end_name() {
    Globals::with(|globals| rs_end_name(globals))
}
