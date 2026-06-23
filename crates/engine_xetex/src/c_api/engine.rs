use crate::c_api::globals::Globals;
use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::{ptr, slice};

mod memory;

use crate::c_api::is_dir_sep;
use crate::c_api::output::{
    rs_print, rs_print_bytes, rs_print_char, rs_print_cs, rs_print_esc_bytes, rs_print_int,
    rs_print_ln, rs_print_nl, rs_print_nl_bytes, rs_print_raw_char,
};
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

pub const LEFT_BRACE: i32 = 1;
pub const RIGHT_BRACE: i32 = 2;
pub const MATH_SHIFT: i32 = 3;
pub const TAB_MARK: i32 = 4;
pub const CAR_RET: i32 = 5;
pub const OUT_PARAM: i32 = 5;
pub const MAC_PARAM: i32 = 6;
pub const SUP_MARK: i32 = 7;
pub const SUB_MARK: i32 = 8;
pub const ENDV: i32 = 9;
pub const IGNORE: i32 = 9;
pub const SPACER: i32 = 10;
pub const LETTER: i32 = 11;
pub const OTHER_CHAR: i32 = 12;
pub const PAR_END: i32 = 13;
pub const ACTIVE_CHAR: i32 = 13;
pub const MATCH: i32 = 13;
pub const STOP: i32 = 14;
pub const COMMENT: i32 = 14;
pub const END_MATCH: i32 = 14;
pub const DELIM_NUM: i32 = 15;
pub const INVALID_CHAR: i32 = 15;
pub const CHAR_NUM: i32 = 16;
pub const MAX_CHAR_CODE: i32 = 15;
pub const MATH_CHAR_NUM: i32 = 17;
pub const MARK: i32 = 18;
pub const XRAY: i32 = 19;
pub const MAKE_BOX: i32 = 20;
pub const HMOVE: i32 = 21;
pub const VMOVE: i32 = 22;
pub const UN_HBOX: i32 = 23;
pub const UN_VBOX: i32 = 24;
pub const REMOVE_ITEM: i32 = 25;
pub const HSKIP: i32 = 26;
pub const VSKIP: i32 = 27;
pub const MSKIP: i32 = 28;
pub const KERN: i32 = 29;
pub const MKERN: i32 = 30;
pub const LEADER_SHIP: i32 = 31;
pub const HALIGN: i32 = 32;
pub const VALIGN: i32 = 33;
pub const NO_ALIGN: i32 = 34;
pub const VRULE: i32 = 35;
pub const HRULE: i32 = 36;
pub const INSERT: i32 = 37;
pub const VADJUST: i32 = 38;
pub const IGNORE_SPACES: i32 = 39;
pub const AFTER_ASSIGNMENT: i32 = 40;
pub const AFTER_GROUP: i32 = 41;
pub const BREAK_PENALTY: i32 = 42;
pub const START_PAR: i32 = 43;
pub const ITAL_CORR: i32 = 44;
pub const ACCENT: i32 = 45;
pub const MATH_ACCENT: i32 = 46;
pub const DISCRETIONARY: i32 = 47;
pub const EQ_NO: i32 = 48;
pub const LEFT_RIGHT: i32 = 49;
pub const MATH_COMP: i32 = 50;
pub const LIMIT_SWITCH: i32 = 51;
pub const ABOVE: i32 = 52;
pub const MATH_STYLE: i32 = 53;
pub const MATH_CHOICE: i32 = 54;
pub const NON_SCRIPT: i32 = 55;
pub const VCENTER: i32 = 56;
pub const CASE_SHIFT: i32 = 57;
pub const MESSAGE: i32 = 58;
pub const EXTENSION: i32 = 59;
pub const IN_STREAM: i32 = 60;
pub const BEGIN_GROUP: i32 = 61;
pub const END_GROUP: i32 = 62;
pub const OMIT: i32 = 63;
pub const EX_SPACE: i32 = 64;
pub const NO_BOUNDARY: i32 = 65;
pub const RADICAL: i32 = 66;
pub const END_CS_NAME: i32 = 67;
pub const CHAR_GIVEN: i32 = 68;
pub const MIN_INTERNAL: i32 = 68;
pub const MATH_GIVEN: i32 = 69;
pub const XETEX_MATH_GIVEN: i32 = 70;
pub const LAST_ITEM: i32 = 71;
pub const TOKS_REGISTER: i32 = 72;
pub const MAX_NON_PREFIXED_COMMAND: i32 = 71;
pub const ASSIGN_TOKS: i32 = 73;
pub const ASSIGN_INT: i32 = 74;
pub const ASSIGN_DIMEN: i32 = 75;
pub const ASSIGN_GLUE: i32 = 76;
pub const ASSIGN_MU_GLUE: i32 = 77;
pub const ASSIGN_FONT_DIMEN: i32 = 78;
pub const ASSIGN_FONT_INT: i32 = 79;
pub const SET_AUX: i32 = 80;
pub const SET_PREV_GRAF: i32 = 81;
pub const SET_PAGE_DIMEN: i32 = 82;
pub const SET_PAGE_INT: i32 = 83;
pub const SET_BOX_DIMEN: i32 = 84;
pub const SET_SHAPE: i32 = 85;
pub const DEF_CODE: i32 = 86;
pub const XETEX_DEF_CODE: i32 = 87;
pub const DEF_FAMILY: i32 = 88;
pub const SET_FONT: i32 = 89;
pub const DEF_FONT: i32 = 90;
pub const REGISTER: i32 = 91;
pub const ADVANCE: i32 = 92;
pub const MAX_INTERNAL: i32 = 91;
pub const MULTIPLY: i32 = 93;
pub const DIVIDE: i32 = 94;
pub const PREFIX: i32 = 95;
pub const LET: i32 = 96;
pub const SHORTHAND_DEF: i32 = 97;
pub const READ_TO_CS: i32 = 98;
pub const DEF: i32 = 99;
pub const SET_BOX: i32 = 100;
pub const HYPH_DATA: i32 = 101;
pub const SET_INTERACTION: i32 = 102;
pub const UNDEFINED_CS: i32 = 103;
pub const MAX_COMMAND: i32 = 102;
pub const EXPAND_AFTER: i32 = 104;
pub const NO_EXPAND: i32 = 105;
pub const INPUT: i32 = 106;
pub const IF_TEST: i32 = 107;
pub const FI_OR_ELSE: i32 = 108;
pub const CS_NAME: i32 = 109;
pub const CONVERT: i32 = 110;
pub const THE: i32 = 111;
pub const TOP_BOT_MARK: i32 = 112;
pub const CALL: i32 = 113;
pub const LONG_CALL: i32 = 114;
pub const OUTER_CALL: i32 = 115;
pub const LONG_OUTER_CALL: i32 = 116;
pub const END_TEMPLATE: i32 = 117;
pub const DONT_EXPAND: i32 = 118;
pub const GLUE_REF: i32 = 119;
pub const SHAPE_REF: i32 = 120;
pub const BOX_REF: i32 = 121;
pub const DATA: i32 = 122;

pub const MIN_HALFWORD: i32 = -0x0FFFFFFF;
pub const MAX_HALFWORD: i32 = 0x3FFFFFFF;

pub const TEX_NULL: i32 = MIN_HALFWORD;
/// The largest positive value that TeX knows
pub const TEX_INFINITY: i32 = 0x7FFFFFFF;

/* begin_token_list() types */
pub const PARAMETER: u16 = 0;
pub const U_TEMPLATE: u16 = 1;
pub const V_TEMPLATE: u16 = 2;
pub const BACKED_UP: u16 = 3;
pub const BACKED_UP_CHAR: u16 = 4;
pub const INSERTED: u16 = 5;
pub const MACRO: u16 = 6;
pub const OUTPUT_TEXT: u16 = 7;
pub const EVERY_PAR_TEXT: u16 = 8;
pub const EVERY_MATH_TEXT: u16 = 9;
pub const EVERY_DISPLAY_TEXT: u16 = 10;
pub const EVERY_HBOX_TEXT: u16 = 11;
pub const EVERY_VBOX_TEXT: u16 = 12;
pub const EVERY_JOB_TEXT: u16 = 13;
pub const EVERY_CR_TEXT: u16 = 14;
pub const MARK_TEXT: u16 = 15;
pub const EVERY_EOF_TEXT: u16 = 16;
pub const INTER_CHAR_TEXT: u16 = 17;
pub const WRITE_TEXT: u16 = 18;
pub const TECTONIC_CODA_TEXT: u16 = 19;

pub const POP: u8 = 142;

pub const TOKEN_LIST: u16 = 0;

pub const MAX_CHAR_VAL: i32 = 0x200000;
pub const CS_TOKEN_FLAG: i32 = 0x1FFFFFF;

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
    pub(crate) texmf_log_name: StrNumber,
    pub(crate) log_opened: bool,
    pub(crate) input_stack: Vec<InputState>,
    pub(crate) input_ptr: usize,
    pub(crate) cur_input: InputState,
    pub(crate) interaction: InteractionMode,
    pub(crate) history: History,
    pub(crate) total_pages: i32,
    pub(crate) last_bop: i32,
    pub(crate) base_ptr: usize,
    pub(crate) first_count: i32,
    pub(crate) half_error_line: i32,
    pub(crate) hi_mem_min: i32,
    pub(crate) mem_end: i32,
    pub(crate) halt_on_error_p: i32,
    pub(crate) error_count: i8,
    pub(crate) use_err_help: bool,
    pub(crate) help_ptr: usize,
    pub(crate) help_line: [*const libc::c_char; 6],

    pub(crate) eqtb: Vec<MemoryWord>,
    pub(crate) prim: Box<[B32x2; PRIM_SIZE + 1]>,
    /// An arena of TeX nodes
    pub(crate) mem: Vec<MemoryWord>,
    pub(crate) buffer: Vec<char>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InteractionMode {
    Batch = 0,
    Nonstop,
    Scroll,
    ErrorStop,
}

impl TryFrom<u8> for InteractionMode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => InteractionMode::Batch,
            1 => InteractionMode::Nonstop,
            2 => InteractionMode::Scroll,
            3 => InteractionMode::ErrorStop,
            _ => return Err(value),
        })
    }
}

#[derive(Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputState {
    /// tokenizer state: mid_line, skip_blanks, new_line
    state: u16,
    /// index of this level of input in input_file array
    index: u16,
    /// position of beginning of current line in `buffer`
    start: i32,
    /// position of next character to read in `buffer`
    loc: i32,
    /// position of end of line in `buffer`
    limit: i32,
    /// name of current file or magic value for terminal, etc.
    name: StrNumber,
    synctex_tag: i32,
}

struct NodeError {
    ty: u16,
    subty: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
#[repr(C)]
pub enum History {
    Spotless = 0,
    WarningIssued = 1,
    ErrorIssued = 2,
    FatalError = 3,
}

impl TryFrom<u8> for History {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => History::Spotless,
            1 => History::WarningIssued,
            2 => History::ErrorIssued,
            3 => History::FatalError,
            _ => return Err(value),
        })
    }
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
            texmf_log_name: 0,
            log_opened: false,
            input_stack: Vec::new(),
            input_ptr: 0,
            cur_input: InputState::default(),
            interaction: InteractionMode::Batch,
            history: History::Spotless,
            total_pages: 0,
            last_bop: 0,
            base_ptr: 0,
            first_count: 0,
            half_error_line: 0,
            hi_mem_min: 0,
            mem_end: 0,
            halt_on_error_p: 0,
            error_count: 0,
            use_err_help: false,
            help_ptr: 0,
            help_line: [ptr::null(); 6],

            eqtb: Vec::new(),
            prim: Box::new([B32x2 { s0: 0, s1: 0 }; PRIM_SIZE + 1]),
            mem: Vec::new(),
            buffer: Vec::new(),
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

    pub fn local(&self, local: Local) -> i32 {
        unsafe { self.eqtb[LOCAL_BASE + local as usize].b32.s1 }
    }

    pub fn set_local(&mut self, local: Local, val: i32) {
        self.eqtb[LOCAL_BASE + local as usize].b32.s1 = val
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
pub extern "C" fn texmf_log_name() -> StrNumber {
    ENGINE_CTX.with_borrow(|engine| engine.texmf_log_name)
}

#[no_mangle]
pub extern "C" fn set_texmf_log_name(val: StrNumber) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.texmf_log_name = val)
}

#[no_mangle]
pub extern "C" fn log_opened() -> bool {
    ENGINE_CTX.with_borrow(|engine| engine.log_opened)
}

#[no_mangle]
pub extern "C" fn set_log_opened(val: bool) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.log_opened = val)
}

#[no_mangle]
pub extern "C" fn resize_input_stack(len: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.input_stack.resize(len, InputState::default()))
}

#[no_mangle]
pub extern "C" fn input_stack(idx: usize) -> InputState {
    ENGINE_CTX.with_borrow(|engine| engine.input_stack[idx].clone())
}

#[no_mangle]
pub extern "C" fn set_input_stack(idx: usize, state: InputState) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.input_stack[idx] = state)
}

#[no_mangle]
pub extern "C" fn clear_input_stack() {
    ENGINE_CTX.with_borrow_mut(|engine| engine.input_stack.clear())
}

#[no_mangle]
pub extern "C" fn input_ptr() -> usize {
    ENGINE_CTX.with_borrow(|engine| engine.input_ptr)
}

#[no_mangle]
pub extern "C" fn set_input_ptr(val: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.input_ptr = val)
}

#[no_mangle]
pub extern "C" fn cur_input() -> InputState {
    ENGINE_CTX.with_borrow(|engine| engine.cur_input.clone())
}

#[no_mangle]
pub extern "C" fn cur_input_ptr() -> *mut InputState {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.cur_input))
}

#[no_mangle]
pub extern "C" fn set_cur_input(val: InputState) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.cur_input = val)
}

#[no_mangle]
pub extern "C" fn interaction() -> u8 {
    ENGINE_CTX.with_borrow(|engine| engine.interaction as u8)
}

#[no_mangle]
pub extern "C" fn set_interaction(val: u8) {
    ENGINE_CTX
        .with_borrow_mut(|engine| engine.interaction = InteractionMode::try_from(val).unwrap())
}

#[no_mangle]
pub extern "C" fn history() -> History {
    ENGINE_CTX.with_borrow(|engine| engine.history)
}

#[no_mangle]
pub extern "C" fn set_history(val: u8) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.history = History::try_from(val).unwrap())
}

#[no_mangle]
pub extern "C" fn total_pages() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.total_pages)
}

#[no_mangle]
pub extern "C" fn set_total_pages(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.total_pages = val)
}

#[no_mangle]
pub extern "C" fn last_bop() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.last_bop)
}

#[no_mangle]
pub extern "C" fn set_last_bop(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.last_bop = val)
}

#[no_mangle]
pub extern "C" fn base_ptr() -> usize {
    ENGINE_CTX.with_borrow(|engine| engine.base_ptr)
}

#[no_mangle]
pub extern "C" fn set_base_ptr(val: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.base_ptr = val)
}

#[no_mangle]
pub extern "C" fn first_count() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.first_count)
}

#[no_mangle]
pub extern "C" fn set_first_count(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.first_count = val)
}

#[no_mangle]
pub extern "C" fn half_error_line() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.half_error_line)
}

#[no_mangle]
pub extern "C" fn set_half_error_line(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.half_error_line = val)
}

#[no_mangle]
pub extern "C" fn hi_mem_min() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.hi_mem_min)
}

#[no_mangle]
pub extern "C" fn set_hi_mem_min(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.hi_mem_min = val)
}

#[no_mangle]
pub extern "C" fn mem_end() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.mem_end)
}

#[no_mangle]
pub extern "C" fn set_mem_end(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.mem_end = val)
}

#[no_mangle]
pub extern "C" fn halt_on_error_p() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.halt_on_error_p)
}

#[no_mangle]
pub extern "C" fn set_halt_on_error_p(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.halt_on_error_p = val)
}

#[no_mangle]
pub extern "C" fn error_count() -> i8 {
    ENGINE_CTX.with_borrow(|engine| engine.error_count)
}

#[no_mangle]
pub extern "C" fn set_error_count(val: i8) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.error_count = val)
}

#[no_mangle]
pub extern "C" fn use_err_help() -> bool {
    ENGINE_CTX.with_borrow(|engine| engine.use_err_help)
}

#[no_mangle]
pub extern "C" fn set_use_err_help(val: bool) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.use_err_help = val)
}

#[no_mangle]
pub extern "C" fn help_ptr() -> usize {
    ENGINE_CTX.with_borrow(|engine| engine.help_ptr)
}

#[no_mangle]
pub extern "C" fn set_help_ptr(val: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.help_ptr = val)
}

#[no_mangle]
pub extern "C" fn help_line(idx: usize) -> *const libc::c_char {
    ENGINE_CTX.with_borrow(|engine| engine.help_line[idx])
}

#[no_mangle]
pub extern "C" fn set_help_line(idx: usize, ptr: *const libc::c_char) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.help_line[idx] = ptr)
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

#[no_mangle]
pub extern "C" fn resize_buffer(len: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.buffer.resize(len, '\0'))
}

#[no_mangle]
pub extern "C" fn buffer_ptr() -> *mut char {
    ENGINE_CTX.with_borrow_mut(|engine| engine.buffer.as_mut_ptr())
}

#[no_mangle]
pub extern "C" fn buffer(idx: usize) -> char {
    ENGINE_CTX.with_borrow(|engine| engine.buffer[idx])
}

#[no_mangle]
pub extern "C" fn set_buffer(idx: usize, val: u32) {
    ENGINE_CTX.with_borrow_mut(|engine| {
        engine.buffer[idx] = char::from_u32(val).unwrap_or(char::REPLACEMENT_CHARACTER)
    })
}

#[no_mangle]
pub extern "C" fn clear_buffer() {
    ENGINE_CTX.with_borrow_mut(|engine| engine.buffer.clear())
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

pub fn rs_more_name(globals: &mut Globals<'_, '_>, c: u16) -> bool {
    if globals.engine.stop_at_space && globals.engine.file_name_quote_char == 0 && c == ' ' as u16 {
        return false;
    }

    if globals.engine.stop_at_space
        && globals.engine.file_name_quote_char != 0
        && c == globals.engine.file_name_quote_char
    {
        globals.engine.file_name_quote_char = 0;
        return true;
    }

    if globals.engine.stop_at_space
        && globals.engine.file_name_quote_char == 0
        && (c == '"' as u16 || c == '\'' as u16)
    {
        globals.engine.file_name_quote_char = c;
        globals.engine.quoted_filename = true;
        return true;
    }

    if globals.strings.pool_ptr + 1 > globals.strings.pool_size {
        todo!("overflow(\"pool size\", pool_size() - init_pool_ptr)");
    }

    globals.strings.str_pool[globals.strings.pool_ptr as usize] = c;
    globals.strings.pool_ptr += 1;

    if is_dir_sep(char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER)) {
        globals.engine.area_delimiter = globals.strings.cur_length();
        globals.engine.ext_delimiter = 0;
    } else if c == '.' as u16 {
        globals.engine.ext_delimiter = globals.strings.cur_length();
    }

    true
}

pub fn rs_make_name_string(globals: &mut Globals<'_, '_>) -> StrNumber {
    if globals.strings.pool_ptr
        + globals
            .engine
            .name_of_file
            .as_ref()
            .map(|n| n.count_bytes())
            .unwrap_or(0)
        > globals.strings.pool_size
        || globals.strings.str_ptr == globals.strings.max_strings
    {
        return '?' as StrNumber;
    }

    rs_make_utf16_name(globals.engine);

    // TODO: Don't allocate and set name_of_file, just use encode_utf16
    if let Some(s) = globals.engine.name_of_file_utf16.as_deref() {
        for &c in s {
            globals.strings.str_pool[globals.strings.pool_ptr] = c;
            globals.strings.pool_ptr += 1;
        }
    }

    let res = rs_make_string(globals.strings);

    let save = (
        globals.engine.area_delimiter,
        globals.engine.ext_delimiter,
        globals.engine.name_in_progress,
        globals.engine.stop_at_space,
    );
    globals.engine.name_in_progress = true;
    rs_begin_name(globals);
    globals.engine.stop_at_space = false;

    let name = globals.engine.name_of_file_utf16.take();
    // Needed for side-effects
    name.as_deref()
        .and_then(|s| s.iter().find(|&c| !rs_more_name(globals, *c)));
    globals.engine.name_of_file_utf16 = name;

    globals.engine.stop_at_space = save.3;
    rs_end_name(globals);
    globals.engine.name_in_progress = save.2;
    globals.engine.ext_delimiter = save.1;
    globals.engine.area_delimiter = save.0;

    res
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

#[no_mangle]
pub extern "C" fn more_name(c: u16) -> bool {
    Globals::with(|globals| rs_more_name(globals, c))
}

#[no_mangle]
pub extern "C" fn make_name_string() -> StrNumber {
    Globals::with(|globals| rs_make_name_string(globals))
}

pub fn rs_open_log_file(globals: &mut Globals<'_, '_>) {
    let old = globals.engine.selector;

    if globals.engine.job_name == 0 {
        globals.engine.job_name = rs_maketexstring(globals, "texput");
    }

    rs_pack_job_name(globals, ".log");

    let file_name = globals
        .engine
        .name_of_file
        .as_ref()
        .unwrap()
        .to_string_lossy();
    match globals.state.output_open(&file_name, false) {
        Some(file) => globals.out.log_file = Some(file),
        None => panic!("cannot open log file output \"{}\"", file_name),
    }

    globals.engine.texmf_log_name = rs_make_name_string(globals);
    globals.engine.selector = Selector::LogOnly;
    globals.engine.log_opened = true;

    globals.engine.input_stack[globals.engine.input_ptr] = globals.engine.cur_input.clone();

    rs_print_nl_bytes(globals, b"**");
    let mut l = globals.engine.input_stack[0].limit as usize;
    if globals.engine.buffer[l] as i32 == globals.engine.int_par(IntPar::EndLineChar) {
        l -= 1;
    }

    for k in 1..=l {
        rs_print(globals, globals.engine.buffer[k] as i32)
    }

    rs_print_ln(globals);
    globals.engine.selector = match old {
        Selector::NoPrint => Selector::LogOnly,
        Selector::TermOnly => Selector::TermAndLog,
        _ => panic!(),
    };
}

#[no_mangle]
pub extern "C" fn open_log_file() {
    Globals::with(|globals| rs_open_log_file(globals))
}

pub fn rs_show_token_list(globals: &mut Globals<'_, '_>, mut p: usize, q: usize, l: i32) {
    let mut match_chr = '#' as i32;
    let mut n = b'0';
    globals.engine.tally = 0;

    while p as i32 != TEX_NULL && globals.engine.tally < l {
        if p == q {
            globals.engine.first_count = globals.engine.tally;
            globals.engine.trick_count = globals.engine.tally + 1 + globals.engine.error_line
                - globals.engine.half_error_line;
            if globals.engine.trick_count < globals.engine.error_line {
                globals.engine.trick_count = globals.engine.error_line;
            }
        }
        if (p as i32) < globals.engine.hi_mem_min || (p as i32) > globals.engine.mem_end {
            rs_print_esc_bytes(globals, b"CLOBBERED.");
            return;
        }

        // TODO: Use semantic accessors maybe?
        if unsafe { globals.engine.mem[p].b32.s0 } >= CS_TOKEN_FLAG {
            rs_print_cs(
                globals,
                unsafe { globals.engine.mem[p].b32.s0 } - CS_TOKEN_FLAG,
            );
        } else {
            let temp = unsafe { globals.engine.mem[p].b32.s0 };
            let m = temp / MAX_CHAR_VAL;
            let c = temp % MAX_CHAR_VAL;

            if temp < 0 {
                rs_print_esc_bytes(globals, b"BAD.");
            } else {
                match m {
                    LEFT_BRACE | RIGHT_BRACE | MATH_SHIFT | TAB_MARK | SUP_MARK | SUB_MARK
                    | SPACER | LETTER | OTHER_CHAR => {
                        rs_print_char(globals, c);
                    }
                    MAC_PARAM => {
                        rs_print_char(globals, c);
                        rs_print_char(globals, c);
                    }
                    OUT_PARAM => {
                        rs_print_char(globals, match_chr);
                        if c <= 9 {
                            rs_print_char(globals, c + 48);
                        } else {
                            rs_print_char(globals, '!' as i32);
                            return;
                        }
                    }
                    MATCH => {
                        match_chr = c;
                        rs_print_char(globals, c);
                        n += 1;
                        rs_print_char(globals, n as i32);
                        if n > b'9' {
                            return;
                        }
                    }
                    END_MATCH => {
                        if c == 0 {
                            rs_print_bytes(globals, b"->");
                        }
                    }
                    _ => {
                        rs_print_esc_bytes(globals, b"BAD.");
                    }
                }
            }
        }

        p = globals.engine.base_node(p).next();
    }

    if p as i32 != TEX_NULL {
        rs_print_esc_bytes(globals, b"ETC.");
    }
}

#[no_mangle]
pub extern "C" fn show_token_list(p: i32, q: i32, l: i32) {
    Globals::with(|globals| rs_show_token_list(globals, p as usize, q as usize, l))
}

pub fn rs_show_context(globals: &mut Globals<'_, '_>) {
    globals.engine.base_ptr = globals.engine.input_ptr;
    globals.engine.input_stack[globals.engine.base_ptr] = globals.engine.cur_input.clone();
    let mut nn = -1;
    let mut bottom_line = false;
    loop {
        globals.engine.cur_input = globals.engine.input_stack[globals.engine.base_ptr].clone();
        if globals.engine.cur_input.state != TOKEN_LIST {
            if globals.engine.cur_input.name > 19 || globals.engine.base_ptr == 0 {
                bottom_line = true;
            }
        }

        if globals.engine.base_ptr == globals.engine.input_ptr
            || bottom_line
            || nn < globals.engine.int_par(IntPar::ErrorContextLines)
        {
            let l;
            if globals.engine.base_ptr == globals.engine.input_ptr
                || globals.engine.cur_input.state != TOKEN_LIST
                || globals.engine.cur_input.index != BACKED_UP
                || globals.engine.cur_input.loc != TEX_NULL
            {
                globals.engine.tally = 0;
                let old_setting = globals.engine.selector;
                if globals.engine.cur_input.state != TOKEN_LIST {
                    if globals.engine.cur_input.name <= 17 {
                        if globals.engine.cur_input.name == 0 {
                            if globals.engine.base_ptr == 0 {
                                rs_print_nl_bytes(globals, b"<*>")
                            } else {
                                rs_print_nl_bytes(globals, b"<insert> ")
                            }
                        } else {
                            rs_print_nl_bytes(globals, b"<read ");
                            if globals.engine.cur_input.name == 17 {
                                rs_print_char(globals, '*' as i32);
                            } else {
                                rs_print_int(globals, globals.engine.cur_input.name - 1);
                            }
                            rs_print_char(globals, '>' as i32);
                        }
                    } else {
                        rs_print_nl_bytes(globals, b"l.");
                        if globals.engine.cur_input.index as i32 == globals.files.in_open {
                            rs_print_int(globals, globals.files.line);
                        } else {
                            rs_print_int(
                                globals,
                                globals.files.line_stack
                                    [(globals.engine.cur_input.index + 1) as usize],
                            );
                        }
                    }
                    rs_print_char(globals, b' ' as i32);

                    l = globals.engine.tally;
                    globals.engine.tally = 0;
                    globals.engine.selector = Selector::Pseudo;
                    globals.engine.trick_count = 1000000;

                    let j;
                    if globals.engine.buffer[globals.engine.cur_input.limit as usize] as i32
                        == globals.engine.int_par(IntPar::EndLineChar)
                    {
                        j = globals.engine.cur_input.limit;
                    } else {
                        j = globals.engine.cur_input.limit + 1;
                    }
                    if j > 0 {
                        let mut i = globals.engine.cur_input.start;
                        let for_end = j - 1;
                        if i <= for_end {
                            loop {
                                if i == globals.engine.cur_input.loc {
                                    globals.engine.first_count = globals.engine.tally;
                                    globals.engine.trick_count =
                                        globals.engine.tally + 1 + globals.engine.error_line
                                            - globals.engine.half_error_line;
                                    if globals.engine.trick_count < globals.engine.error_line {
                                        globals.engine.trick_count = globals.engine.error_line;
                                    }
                                }
                                rs_print_char(globals, globals.engine.buffer[i as usize] as i32);
                                if i >= for_end {
                                    break;
                                }
                                i += 1;
                            }
                        }
                    }
                } else {
                    match globals.engine.cur_input.index {
                        PARAMETER => rs_print_nl_bytes(globals, b"<argument> "),
                        U_TEMPLATE | V_TEMPLATE => rs_print_nl_bytes(globals, b"<template> "),
                        BACKED_UP | BACKED_UP_CHAR => {
                            if globals.engine.cur_input.loc == TEX_NULL {
                                rs_print_nl_bytes(globals, b"<recently read> ")
                            } else {
                                rs_print_nl_bytes(globals, b"<to be read again> ")
                            }
                        }
                        INSERTED => rs_print_nl_bytes(globals, b"<inserted text> "),
                        MACRO => {
                            rs_print_ln(globals);
                            rs_print_cs(globals, globals.engine.cur_input.name);
                        }
                        OUTPUT_TEXT => rs_print_nl_bytes(globals, b"<output> "),
                        EVERY_PAR_TEXT => rs_print_nl_bytes(globals, b"<everypar> "),
                        EVERY_MATH_TEXT => rs_print_nl_bytes(globals, b"<everymath> "),
                        EVERY_DISPLAY_TEXT => rs_print_nl_bytes(globals, b"<everydisplay> "),
                        EVERY_HBOX_TEXT => rs_print_nl_bytes(globals, b"<everyhbox> "),
                        EVERY_VBOX_TEXT => rs_print_nl_bytes(globals, b"<everyvbox> "),
                        EVERY_JOB_TEXT => rs_print_nl_bytes(globals, b"<everyjob> "),
                        EVERY_CR_TEXT => rs_print_nl_bytes(globals, b"<everycr> "),
                        MARK_TEXT => rs_print_nl_bytes(globals, b"<mark> "),
                        EVERY_EOF_TEXT => rs_print_nl_bytes(globals, b"<everyeof> "),
                        INTER_CHAR_TEXT => rs_print_nl_bytes(globals, b"<XeTeXinterchartoks> "),
                        WRITE_TEXT => rs_print_nl_bytes(globals, b"<write> "),
                        TECTONIC_CODA_TEXT => rs_print_nl_bytes(globals, b"<TectonicCodaTokens> "),
                        _ => rs_print_nl(globals, '?' as i32),
                    }

                    l = globals.engine.tally;
                    globals.engine.tally = 0;
                    globals.engine.selector = Selector::Pseudo;
                    globals.engine.trick_count = 1000000;

                    if globals.engine.cur_input.index < MACRO {
                        rs_show_token_list(
                            globals,
                            globals.engine.cur_input.start as usize,
                            globals.engine.cur_input.loc as usize,
                            100000,
                        );
                    } else {
                        rs_show_token_list(
                            globals,
                            unsafe {
                                globals.engine.mem[globals.engine.cur_input.start as usize]
                                    .b32
                                    .s1
                            } as usize,
                            globals.engine.cur_input.loc as usize,
                            100000,
                        );
                    }
                }
                globals.engine.selector = old_setting;
                if globals.engine.trick_count == 1000000 {
                    globals.engine.first_count = globals.engine.tally;
                    globals.engine.trick_count = globals.engine.tally
                        + 1
                        + globals.engine.error_line
                        + globals.engine.half_error_line;
                    if globals.engine.trick_count < globals.engine.error_line {
                        globals.engine.trick_count = globals.engine.error_line;
                    }
                }

                let m;
                if globals.engine.tally < globals.engine.trick_count {
                    m = globals.engine.tally - globals.engine.first_count;
                } else {
                    m = globals.engine.trick_count - globals.engine.first_count;
                }

                let n;
                let p;
                if l + globals.engine.first_count <= globals.engine.half_error_line {
                    p = 0;
                    n = l + globals.engine.first_count;
                } else {
                    rs_print_bytes(globals, b"...");
                    p = l + globals.engine.first_count - globals.engine.half_error_line + 3;
                    n = globals.engine.half_error_line;
                }

                let mut q = p;
                let for_end = globals.engine.first_count - 1;
                if q <= for_end {
                    loop {
                        rs_print_char(
                            globals,
                            globals.engine.trick_buf[(q % globals.engine.error_line) as usize]
                                as i32,
                        );
                        if q >= for_end {
                            break;
                        }
                        q += 1;
                    }
                }

                rs_print_ln(globals);

                let mut q = 1;
                let for_end = n;
                if q <= for_end {
                    loop {
                        rs_print_raw_char(globals, b' ' as u16, true);
                        if q >= for_end {
                            break;
                        }
                        q += 1;
                    }
                }

                let p;
                if m + n <= globals.engine.error_line {
                    p = globals.engine.first_count + m;
                } else {
                    p = globals.engine.first_count + (globals.engine.error_line - n - 3);
                }

                let mut q = globals.engine.first_count;
                let for_end = p - 1;
                if q <= for_end {
                    loop {
                        rs_print_char(
                            globals,
                            globals.engine.trick_buf[(q % globals.engine.error_line) as usize]
                                as i32,
                        );
                        if q >= for_end {
                            break;
                        }
                        q += 1;
                    }
                }

                if m + n > globals.engine.error_line {
                    rs_print_bytes(globals, b"...");
                }
                nn += 1;
            }
        } else if nn == globals.engine.int_par(IntPar::ErrorContextLines) {
            rs_print_nl_bytes(globals, b"...");
            nn += 1;
        }

        if bottom_line {
            break;
        }

        globals.engine.base_ptr -= 1;
    }
    globals.engine.cur_input = globals.engine.input_stack[globals.engine.input_ptr].clone();
}

#[no_mangle]
pub extern "C" fn show_context() {
    Globals::with(|globals| rs_show_context(globals))
}

pub fn rs_token_show(globals: &mut Globals<'_, '_>, p: usize) {
    if p as i32 != TEX_NULL {
        rs_show_token_list(
            globals,
            unsafe { globals.engine.mem[p].b32.s1 } as usize,
            TEX_NULL as usize,
            10000000,
        );
    }
}

#[no_mangle]
pub extern "C" fn token_show(p: i32) {
    Globals::with(|globals| rs_token_show(globals, p as usize))
}
