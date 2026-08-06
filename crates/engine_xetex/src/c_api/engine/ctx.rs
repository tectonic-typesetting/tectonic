use crate::c_api::engine::{
    B32x2, CatCode, History, InputState, IntPar, InteractionMode, ListStateRecord, Local,
    MemoryWord, Node, NodeBase, NodeError, Selector, CAT_CODE_BASE, EQTB_SIZE, INT_BASE,
    LOCAL_BASE, PRIM_SIZE,
};
use crate::c_api::globals::{Globals, ALL_CTX};
use crate::c_api::inputs::UFile;
use crate::ty::{Scaled, StrNumber};
use std::ffi::{CStr, CString};
use std::{ptr, slice};

pub struct EngineCtx {
    pub(crate) selector: Selector,
    pub(crate) old_setting: Selector,
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
    pub(crate) mag_set: i32,
    pub(crate) max_h: Scaled,
    pub(crate) max_v: Scaled,
    pub(crate) max_push: i32,
    pub(crate) semantic_pagination_enabled: bool,
    pub(crate) tex_format_default: CString,
    pub(crate) nest_cur: i32,
    pub(crate) cur_chr: i32,
    pub(crate) cur_cmd: u8,
    pub(crate) cur_cs: i32,
    pub(crate) cur_tok: i32,
    pub(crate) scanner_status: u8,
    pub(crate) param_ptr: i32,
    pub(crate) align_state: i32,
    pub(crate) avail: i32,
    pub(crate) rover: i32,

    pub(crate) eqtb: Vec<MemoryWord>,
    pub(crate) prim: Box<[B32x2; PRIM_SIZE + 1]>,
    /// An arena of TeX nodes
    pub(crate) mem: Vec<MemoryWord>,
    pub(crate) buffer: Vec<char>,
    pub(crate) xeq_level_array: Vec<u16>,
    pub(crate) nest: Vec<ListStateRecord>,
    pub(crate) save_stack: Vec<MemoryWord>,
    pub(crate) input_file: Vec<*mut UFile>,
    pub(crate) eof_seen: Vec<bool>,
    pub(crate) grp_stack: Vec<i32>, // save pointer
    pub(crate) if_stack: Vec<i32>,
    pub(crate) param_stack: Vec<i32>,
    pub(crate) hyph_word: Vec<StrNumber>,
    pub(crate) hyph_list: Vec<i32>,
    pub(crate) hyph_link: Vec<u16>, // hyph pointer
    pub(crate) native_text: Vec<u16>,
    pub(crate) yhash: Vec<B32x2>,
    pub(crate) trie_trl: Vec<i32>, // trie pointer
    pub(crate) trie_tro: Vec<i32>, // trie pointer
    pub(crate) trie_trc: Vec<u16>,
}

impl EngineCtx {
    pub(crate) fn new() -> EngineCtx {
        EngineCtx {
            selector: Selector::File(0),
            old_setting: Selector::File(0),
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
            mag_set: 0,
            max_h: 0,
            max_v: 0,
            max_push: 0,
            semantic_pagination_enabled: false,
            tex_format_default: CString::default(),
            nest_cur: 0,
            cur_chr: 0,
            cur_cmd: 0,
            cur_cs: 0,
            cur_tok: 0,
            scanner_status: 0,
            param_ptr: 0,
            align_state: 0,
            avail: 0,
            rover: 0,

            eqtb: Vec::new(),
            prim: Box::new([B32x2 { s0: 0, s1: 0 }; PRIM_SIZE + 1]),
            mem: Vec::new(),
            buffer: Vec::new(),
            xeq_level_array: vec![0; EQTB_SIZE - INT_BASE + 1],
            nest: Vec::new(),
            save_stack: Vec::new(),
            input_file: Vec::new(),
            eof_seen: Vec::new(),
            grp_stack: Vec::new(),
            if_stack: Vec::new(),
            param_stack: Vec::new(),
            hyph_word: Vec::new(),
            hyph_list: Vec::new(),
            hyph_link: Vec::new(),
            native_text: Vec::new(),
            yhash: Vec::new(),
            trie_trl: Vec::new(),
            trie_tro: Vec::new(),
            trie_trc: Vec::new(),
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut EngineCtx) -> T) -> T {
        Globals::token(|tok| {
            ALL_CTX.with(|(engine, _, _, _, _, _, _, _, _)| f(engine.borrow_mut(tok)))
        })
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

    pub fn set_xeq_level(&mut self, idx: usize, val: u16) {
        self.xeq_level_array[idx - INT_BASE] = val;
    }
}

c_var!(EngineCtx => selector: into u32);
c_var!(EngineCtx => tally: i32);
c_var!(EngineCtx => error_line: i32);
c_var!(EngineCtx => trick_count: i32);
c_arr!(EngineCtx => trick_buf[_]: u16);
c_var!(EngineCtx => eqtb_top: i32);

#[no_mangle]
pub extern "C" fn name_length() -> usize {
    EngineCtx::with(|engine| {
        engine
            .name_of_file
            .as_ref()
            .map(|s| s.count_bytes())
            .unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn name_of_file() -> *const libc::c_char {
    EngineCtx::with(|engine| {
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
    EngineCtx::with(|engine| engine.name_of_file = s.map(CStr::to_owned))
}

#[no_mangle]
pub extern "C" fn name_length16() -> usize {
    EngineCtx::with(|engine| {
        engine
            .name_of_file_utf16
            .as_ref()
            .map(|s| s.len())
            .unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn name_of_file16() -> *const u16 {
    EngineCtx::with(|engine| {
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
    EngineCtx::with(|engine| engine.name_of_file_utf16 = s.map(<[u16]>::to_owned))
}

c_var!(EngineCtx => cur_name: StrNumber);
c_var!(EngineCtx => cur_area: StrNumber);
c_var!(EngineCtx => cur_ext: StrNumber);
c_var!(EngineCtx => job_name: StrNumber);
c_var!(EngineCtx => area_delimiter: usize);
c_var!(EngineCtx => ext_delimiter: usize);
c_var!(EngineCtx => name_in_progress: bool);
c_var!(EngineCtx => stop_at_space: bool);
c_var!(EngineCtx => file_name_quote_char: u16);
c_var!(EngineCtx => quoted_filename: bool);
c_var!(EngineCtx => texmf_log_name: StrNumber);
c_var!(EngineCtx => log_opened: bool);
c_arr!(EngineCtx => input_stack: InputState);
c_var!(EngineCtx => input_ptr: usize);

#[no_mangle]
pub extern "C" fn cur_input() -> InputState {
    EngineCtx::with(|engine| engine.cur_input.clone())
}

#[no_mangle]
pub extern "C" fn cur_input_ptr() -> *mut InputState {
    EngineCtx::with(|engine| ptr::from_mut(&mut engine.cur_input))
}

#[no_mangle]
pub extern "C" fn set_cur_input(val: InputState) {
    EngineCtx::with(|engine| engine.cur_input = val)
}

c_var!(EngineCtx => interaction: into u8);
c_var!(EngineCtx => history: into u8);
c_var!(EngineCtx => total_pages: i32);
c_var!(EngineCtx => last_bop: i32);
c_var!(EngineCtx => base_ptr: usize);
c_var!(EngineCtx => first_count: i32);
c_var!(EngineCtx => half_error_line: i32);
c_var!(EngineCtx => hi_mem_min: i32);
c_var!(EngineCtx => mem_end: i32);
c_var!(EngineCtx => halt_on_error_p: i32);
c_var!(EngineCtx => error_count: i8);
c_var!(EngineCtx => use_err_help: bool);
c_var!(EngineCtx => help_ptr: usize);
c_arr!(EngineCtx => help_line[_]: *const libc::c_char);
c_var!(EngineCtx => mag_set: i32);
c_var!(EngineCtx => max_h: i32);
c_var!(EngineCtx => max_v: i32);
c_var!(EngineCtx => max_push: i32);
c_var!(EngineCtx => semantic_pagination_enabled: bool);

#[no_mangle]
pub extern "C" fn tex_format_default() -> *const libc::c_char {
    EngineCtx::with(|engine| engine.tex_format_default.as_ptr())
}

#[no_mangle]
pub extern "C" fn set_tex_format_default(val: *const libc::c_char) {
    if val.is_null() {
        EngineCtx::with(|engine| engine.tex_format_default = CString::default());
    } else {
        let val = unsafe { CStr::from_ptr(val) };
        EngineCtx::with(|engine| engine.tex_format_default = val.to_owned());
    }
}

c_var!(EngineCtx => nest_cur: i32);
c_var!(EngineCtx => cur_chr: i32);
c_var!(EngineCtx => cur_cmd: u8);
c_var!(EngineCtx => cur_cs: i32);
c_var!(EngineCtx => cur_tok: i32);
c_var!(EngineCtx => scanner_status: u8);
c_var!(EngineCtx => param_ptr: i32);
c_var!(EngineCtx => align_state: i32);
c_var!(EngineCtx => avail: i32);
c_var!(EngineCtx => rover: i32);

c_arr!(EngineCtx => eqtb: MemoryWord);
c_arr!(EngineCtx => mem: MemoryWord);
c_arr!(EngineCtx => prim[_]: B32x2);

#[no_mangle]
pub extern "C" fn resize_buffer(len: usize) {
    EngineCtx::with(|engine| engine.buffer.resize(len, '\0'))
}

#[no_mangle]
pub extern "C" fn buffer_ptr() -> *mut char {
    EngineCtx::with(|engine| engine.buffer.as_mut_ptr())
}

#[no_mangle]
pub extern "C" fn buffer(idx: usize) -> u32 {
    EngineCtx::with(|engine| engine.buffer[idx]) as u32
}

#[no_mangle]
pub extern "C" fn set_buffer(idx: usize, val: u32) {
    EngineCtx::with(|engine| {
        engine.buffer[idx] = char::from_u32(val).unwrap_or(char::REPLACEMENT_CHARACTER)
    })
}

#[no_mangle]
pub extern "C" fn clear_buffer() {
    EngineCtx::with(|engine| engine.buffer.clear())
}

#[no_mangle]
pub extern "C" fn xeq_level_array_ptr(idx: usize) -> *mut u16 {
    EngineCtx::with(|engine| ptr::from_mut(&mut engine.xeq_level_array[idx]))
}

c_arr!(EngineCtx => nest: ListStateRecord);
c_arr!(EngineCtx => save_stack: MemoryWord);
c_arr!(EngineCtx => input_file: *mut UFile);
c_arr!(EngineCtx => eof_seen: bool);
c_arr!(EngineCtx => grp_stack: i32);
c_arr!(EngineCtx => if_stack: i32);
c_arr!(EngineCtx => param_stack: i32);
c_arr!(EngineCtx => hyph_word: i32);
c_arr!(EngineCtx => hyph_list: i32);
c_arr!(EngineCtx => hyph_link: u16);
c_arr!(EngineCtx => native_text: u16);
c_arr!(EngineCtx => yhash: B32x2);
c_arr!(EngineCtx => trie_trl: i32);
c_arr!(EngineCtx => trie_tro: i32);
c_arr!(EngineCtx => trie_trc: u16);
