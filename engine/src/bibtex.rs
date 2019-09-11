#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
use libc::free;
extern "C" {
    /* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    /* Both XeTeX and bibtex use this enum: */
    /* The weird enum values are historical and could be rationalized. But it is
     * good to write them explicitly since they must be kept in sync with
     * `src/engines/mod.rs`.
     */
    /* quasi-hack to get the primary input */
    /* Bridge API. Keep synchronized with src/engines/mod.rs. */
    /* These functions are not meant to be used in the C/C++ code. They define the
     * API that we expose to the Rust side of things. */
    /* The internal, C/C++ interface: */
    /* Global symbols that route through the global API variable. Hopefully we
     * will one day eliminate all of the global state and get rid of all of
     * these. */
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> i32;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const i8,
        format: tt_input_format_type,
        is_gz: i32,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_output_close(handle: rust_output_handle_t) -> i32;
    #[no_mangle]
    fn ttstub_output_write(handle: rust_output_handle_t, data: *const i8, len: size_t) -> size_t;
    #[no_mangle]
    fn ttstub_output_putc(handle: rust_output_handle_t, c: i32) -> i32;
    #[no_mangle]
    fn ttstub_output_open_stdout() -> rust_output_handle_t;
    #[no_mangle]
    fn ttstub_output_open(path: *const i8, is_gz: i32) -> rust_output_handle_t;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::std::ffi::VaList) -> i32;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type tt_history_t = u32;
pub const HISTORY_FATAL_ERROR: tt_history_t = 3;
pub const HISTORY_ERROR_ISSUED: tt_history_t = 2;
pub const HISTORY_WARNING_ISSUED: tt_history_t = 1;
pub const HISTORY_SPOTLESS: tt_history_t = 0;
pub type tt_input_format_type = u32;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_output_handle_t = *mut libc::c_void;
pub type rust_input_handle_t = *mut libc::c_void;
pub type str_number = i32;
/*22: */
pub type ASCII_code = u8;
pub type pool_pointer = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [i64; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type bib_number = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct peekable_input_t {
    pub handle: rust_input_handle_t,
    pub peek_char: i32,
    pub saw_eof: bool,
}
pub type buf_pointer = i32;
pub type lex_type = u8;
pub type buf_type = *mut ASCII_code;
pub type hash_loc = i32;
pub type fn_class = u8;
pub type str_ilk = u8;
pub type hash_pointer = i32;
pub type id_type = u8;
pub type cite_number = i32;
pub type str_ent_loc = i32;
pub type stk_type = u8;
pub type lit_stk_loc = i32;
pub type int_ent_loc = i32;
pub type field_loc = i32;
pub type wiz_fn_loc = i32;
pub type hash_ptr2 = i32;
pub type fn_def_loc = i32;
pub type aux_number = i32;
pub type pds_len = u8;
pub type pds_type = *const i8;
pub type blt_in_range = i32;
unsafe extern "C" fn peekable_open(
    mut path: *const i8,
    mut format: tt_input_format_type,
) -> *mut peekable_input_t {
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut peekable: *mut peekable_input_t = 0 as *mut peekable_input_t;
    handle = ttstub_input_open(path, format, 0i32);
    if handle.is_null() {
        return 0 as *mut peekable_input_t;
    }
    peekable = xmalloc((1i32 as u64).wrapping_mul(::std::mem::size_of::<peekable_input_t>() as u64))
        as *mut peekable_input_t;
    (*peekable).handle = handle;
    (*peekable).peek_char = -1i32;
    (*peekable).saw_eof = false;
    peekable
}
unsafe extern "C" fn peekable_close(mut peekable: *mut peekable_input_t) -> i32 {
    let mut rv: i32 = 0;
    if peekable.is_null() {
        return 0i32;
    }
    rv = ttstub_input_close((*peekable).handle);
    free(peekable as *mut libc::c_void);
    rv
}
unsafe extern "C" fn peekable_getc(mut peekable: *mut peekable_input_t) -> i32 {
    let mut rv: i32 = 0;
    if (*peekable).peek_char != -1i32 {
        rv = (*peekable).peek_char;
        (*peekable).peek_char = -1i32;
        return rv;
    }
    rv = ttstub_input_getc((*peekable).handle);
    if rv == -1i32 {
        (*peekable).saw_eof = true
    }
    rv
}
unsafe extern "C" fn peekable_ungetc(mut peekable: *mut peekable_input_t, mut c: i32) {
    /*last_lex */
    /*last_fn_class */
    /*last_ilk */
    /*last_lit_type */
    /*longest_pds */
    /* TODO: assert c != EOF */
    (*peekable).peek_char = c;
}
/* eofeoln.c, adapted for Rusty I/O */
unsafe extern "C" fn tectonic_eof(mut peekable: *mut peekable_input_t) -> bool {
    /* Check for EOF following Pascal semantics. */
    let mut c: i32 = 0;
    if peekable.is_null() {
        return true;
    }
    if (*peekable).saw_eof {
        return true;
    }
    c = peekable_getc(peekable);
    if c == -1i32 {
        return true;
    }
    peekable_ungetc(peekable, c);
    false
}
unsafe extern "C" fn eoln(mut peekable: *mut peekable_input_t) -> bool {
    let mut c: i32 = 0;
    if (*peekable).saw_eof {
        return true;
    }
    c = peekable_getc(peekable);
    if c != -1i32 {
        peekable_ungetc(peekable, c);
    }
    c == '\n' as i32 || c == '\r' as i32 || c == -1i32
}
/* end eofeoln.c */
static mut error_jmpbuf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut recover_jmpbuf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut standard_output: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut pool_size: i32 = 0;
static mut max_bib_files: i32 = 0;
static mut max_cites: i32 = 0;
static mut wiz_fn_space: i32 = 0;
static mut ent_str_size: i32 = 0;
static mut glob_str_size: i32 = 0;
static mut max_glob_strs: i32 = 0;
static mut max_fields: i32 = 0;
static mut lit_stk_size: i32 = 0;
static mut max_strings: i32 = 0;
static mut hash_size: i32 = 0;
static mut hash_prime: i32 = 0;
static mut hash_max: i32 = 0;
static mut end_of_def: i32 = 0;
static mut undefined: i32 = 0;
static mut bad: i32 = 0;
/*fatal_message */
static mut history: u8 = 0;
static mut err_count: i32 = 0;
static mut lex_class: [lex_type; 256] = [0; 256];
static mut id_class: [id_type; 256] = [0; 256];
static mut char_width: [i32; 256] = [0; 256];
static mut string_width: i32 = 0;
static mut name_of_file: *mut ASCII_code = 0 as *const ASCII_code as *mut ASCII_code;
static mut name_length: i32 = 0;
static mut name_ptr: i32 = 0;
static mut buf_size: i32 = 0;
static mut buffer: buf_type = 0 as *const ASCII_code as *mut ASCII_code;
static mut last: buf_pointer = 0;
static mut sv_buffer: buf_type = 0 as *const ASCII_code as *mut ASCII_code;
static mut sv_ptr1: buf_pointer = 0;
static mut sv_ptr2: buf_pointer = 0;
static mut tmp_ptr: i32 = 0;
static mut tmp_end_ptr: i32 = 0;
static mut str_pool: *mut ASCII_code = 0 as *const ASCII_code as *mut ASCII_code;
static mut str_start: *mut pool_pointer = 0 as *const pool_pointer as *mut pool_pointer;
static mut pool_ptr: pool_pointer = 0;
static mut str_ptr: str_number = 0;
static mut p_ptr1: pool_pointer = 0;
static mut p_ptr2: pool_pointer = 0;
static mut hash_next: *mut hash_pointer = 0 as *const hash_pointer as *mut hash_pointer;
static mut hash_text: *mut str_number = 0 as *const str_number as *mut str_number;
static mut hash_ilk: *mut str_ilk = 0 as *const str_ilk as *mut str_ilk;
static mut ilk_info: *mut i32 = 0 as *const i32 as *mut i32;
static mut hash_used: i32 = 0;
static mut hash_found: bool = false;
static mut dummy_loc: hash_loc = 0;
static mut s_aux_extension: str_number = 0;
static mut s_log_extension: str_number = 0;
static mut s_bbl_extension: str_number = 0;
static mut s_bst_extension: str_number = 0;
static mut s_bib_extension: str_number = 0;
static mut s_bst_area: str_number = 0;
static mut s_bib_area: str_number = 0;
static mut pre_def_loc: hash_loc = 0;
static mut command_num: i32 = 0;
static mut buf_ptr1: buf_pointer = 0;
static mut buf_ptr2: buf_pointer = 0;
/*white_adjacent */
static mut scan_result: u8 = 0;
static mut token_value: i32 = 0;
static mut aux_name_length: i32 = 0;
static mut aux_file: [*mut peekable_input_t; 21] =
    [0 as *const peekable_input_t as *mut peekable_input_t; 21];
static mut aux_list: [str_number; 21] = [0; 21];
static mut aux_ptr: aux_number = 0;
static mut aux_ln_stack: [i32; 21] = [0; 21];
static mut top_lev_str: str_number = 0;
static mut log_file: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut bbl_file: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut bib_list: *mut str_number = 0 as *const str_number as *mut str_number;
static mut bib_ptr: bib_number = 0;
static mut num_bib_files: bib_number = 0;
static mut bib_seen: bool = false;
static mut bib_file: *mut *mut peekable_input_t =
    0 as *const *mut peekable_input_t as *mut *mut peekable_input_t;
static mut bst_seen: bool = false;
static mut bst_str: str_number = 0;
static mut bst_file: *mut peekable_input_t = 0 as *const peekable_input_t as *mut peekable_input_t;
static mut cite_list: *mut str_number = 0 as *const str_number as *mut str_number;
static mut cite_ptr: cite_number = 0;
static mut entry_cite_ptr: cite_number = 0;
static mut num_cites: cite_number = 0;
static mut old_num_cites: cite_number = 0;
static mut citation_seen: bool = false;
static mut cite_loc: hash_loc = 0;
static mut lc_cite_loc: hash_loc = 0;
static mut lc_xcite_loc: hash_loc = 0;
static mut all_entries: bool = false;
static mut all_marker: cite_number = 0;
static mut bbl_line_num: i32 = 0;
static mut bst_line_num: i32 = 0;
static mut fn_loc: hash_loc = 0;
static mut wiz_loc: hash_loc = 0;
static mut literal_loc: hash_loc = 0;
static mut macro_name_loc: hash_loc = 0;
static mut macro_def_loc: hash_loc = 0;
static mut fn_type: *mut fn_class = 0 as *const fn_class as *mut fn_class;
static mut wiz_def_ptr: wiz_fn_loc = 0;
static mut wiz_functions: *mut hash_ptr2 = 0 as *const hash_ptr2 as *mut hash_ptr2;
static mut int_ent_ptr: int_ent_loc = 0;
static mut entry_ints: *mut i32 = 0 as *const i32 as *mut i32;
static mut num_ent_ints: int_ent_loc = 0;
static mut str_ent_ptr: str_ent_loc = 0;
static mut entry_strs: *mut ASCII_code = 0 as *const ASCII_code as *mut ASCII_code;
static mut num_ent_strs: str_ent_loc = 0;
static mut str_glb_ptr: i32 = 0;
static mut glb_str_ptr: *mut str_number = 0 as *const str_number as *mut str_number;
static mut global_strs: *mut ASCII_code = 0 as *const ASCII_code as *mut ASCII_code;
static mut glb_str_end: *mut i32 = 0 as *const i32 as *mut i32;
static mut num_glb_strs: i32 = 0;
static mut field_ptr: field_loc = 0;
static mut field_parent_ptr: field_loc = 0;
static mut field_end_ptr: field_loc = 0;
static mut cite_parent_ptr: cite_number = 0;
static mut cite_xptr: cite_number = 0;
static mut field_info: *mut str_number = 0 as *const str_number as *mut str_number;
static mut num_fields: field_loc = 0;
static mut num_pre_defined_fields: field_loc = 0;
static mut crossref_num: field_loc = 0;
static mut entry_seen: bool = false;
static mut read_seen: bool = false;
static mut read_performed: bool = false;
static mut reading_completed: bool = false;
static mut read_completed: bool = false;
static mut impl_fn_num: i32 = 0;
static mut bib_line_num: i32 = 0;
static mut entry_type_loc: hash_loc = 0;
static mut type_list: *mut hash_ptr2 = 0 as *const hash_ptr2 as *mut hash_ptr2;
static mut type_exists: bool = false;
static mut entry_exists: *mut bool = 0 as *const bool as *mut bool;
static mut store_entry: bool = false;
static mut field_name_loc: hash_loc = 0;
static mut field_val_loc: hash_loc = 0;
static mut store_field: bool = false;
static mut store_token: bool = false;
static mut right_outer_delim: ASCII_code = 0;
static mut right_str_delim: ASCII_code = 0;
static mut at_bib_command: bool = false;
static mut cur_macro_loc: hash_loc = 0;
static mut cite_info: *mut str_number = 0 as *const str_number as *mut str_number;
static mut cite_hash_found: bool = false;
static mut preamble_ptr: bib_number = 0;
static mut num_preamble_strings: bib_number = 0;
static mut bib_brace_level: i32 = 0;
static mut lit_stack: *mut i32 = 0 as *const i32 as *mut i32;
static mut lit_stk_type: *mut stk_type = 0 as *const stk_type as *mut stk_type;
static mut lit_stk_ptr: lit_stk_loc = 0;
static mut cmd_str_ptr: str_number = 0;
static mut ent_chr_ptr: i32 = 0;
static mut glob_chr_ptr: i32 = 0;
static mut ex_buf: buf_type = 0 as *const ASCII_code as *mut ASCII_code;
static mut ex_buf_ptr: buf_pointer = 0;
static mut ex_buf_length: buf_pointer = 0;
static mut out_buf: buf_type = 0 as *const ASCII_code as *mut ASCII_code;
static mut out_buf_ptr: buf_pointer = 0;
static mut out_buf_length: buf_pointer = 0;
static mut mess_with_entries: bool = false;
static mut sort_cite_ptr: cite_number = 0;
static mut sort_key_num: str_ent_loc = 0;
static mut brace_level: i32 = 0;
static mut b_equals: hash_loc = 0;
static mut b_greater_than: hash_loc = 0;
static mut b_less_than: hash_loc = 0;
static mut b_plus: hash_loc = 0;
static mut b_minus: hash_loc = 0;
static mut b_concatenate: hash_loc = 0;
static mut b_gets: hash_loc = 0;
static mut b_add_period: hash_loc = 0;
static mut b_call_type: hash_loc = 0;
static mut b_change_case: hash_loc = 0;
static mut b_chr_to_int: hash_loc = 0;
static mut b_cite: hash_loc = 0;
static mut b_duplicate: hash_loc = 0;
static mut b_empty: hash_loc = 0;
static mut b_format_name: hash_loc = 0;
static mut b_if: hash_loc = 0;
static mut b_int_to_chr: hash_loc = 0;
static mut b_int_to_str: hash_loc = 0;
static mut b_missing: hash_loc = 0;
static mut b_newline: hash_loc = 0;
static mut b_num_names: hash_loc = 0;
static mut b_pop: hash_loc = 0;
static mut b_preamble: hash_loc = 0;
static mut b_purify: hash_loc = 0;
static mut b_quote: hash_loc = 0;
static mut b_skip: hash_loc = 0;
static mut b_stack: hash_loc = 0;
static mut b_substring: hash_loc = 0;
static mut b_swap: hash_loc = 0;
static mut b_text_length: hash_loc = 0;
static mut b_text_prefix: hash_loc = 0;
static mut b_top_stack: hash_loc = 0;
static mut b_type: hash_loc = 0;
static mut b_warning: hash_loc = 0;
static mut b_while: hash_loc = 0;
static mut b_width: hash_loc = 0;
static mut b_write: hash_loc = 0;
static mut b_default: hash_loc = 0;
static mut s_null: str_number = 0;
static mut s_default: str_number = 0;
static mut s_preamble: *mut str_number = 0 as *const str_number as *mut str_number;
static mut pop_lit1: i32 = 0;
static mut pop_lit2: i32 = 0;
static mut pop_lit3: i32 = 0;
static mut pop_typ1: stk_type = 0;
static mut pop_typ2: stk_type = 0;
static mut pop_typ3: stk_type = 0;
static mut sp_ptr: pool_pointer = 0;
static mut sp_xptr1: pool_pointer = 0;
static mut sp_xptr2: pool_pointer = 0;
static mut sp_end: pool_pointer = 0;
static mut sp_length: pool_pointer = 0;
static mut sp2_length: pool_pointer = 0;
static mut sp_brace_level: i32 = 0;
static mut ex_buf_xptr: buf_pointer = 0;
static mut ex_buf_yptr: buf_pointer = 0;
static mut control_seq_loc: hash_loc = 0;
static mut preceding_white: bool = false;
static mut and_found: bool = false;
static mut num_names: i32 = 0;
static mut name_bf_ptr: buf_pointer = 0;
static mut name_bf_xptr: buf_pointer = 0;
static mut name_bf_yptr: buf_pointer = 0;
static mut nm_brace_level: i32 = 0;
static mut name_tok: *mut buf_pointer = 0 as *const buf_pointer as *mut buf_pointer;
static mut name_sep_char: *mut ASCII_code = 0 as *const ASCII_code as *mut ASCII_code;
static mut num_tokens: buf_pointer = 0;
static mut token_starting: bool = false;
static mut alpha_found: bool = false;
static mut double_letter: bool = false;
static mut end_of_group: bool = false;
static mut to_be_written: bool = false;
static mut first_start: buf_pointer = 0;
static mut first_end: buf_pointer = 0;
static mut last_end: buf_pointer = 0;
static mut von_start: buf_pointer = 0;
static mut von_end: buf_pointer = 0;
static mut jr_end: buf_pointer = 0;
static mut cur_token: buf_pointer = 0;
static mut last_token: buf_pointer = 0;
static mut use_default: bool = false;
static mut num_commas: buf_pointer = 0;
static mut comma1: buf_pointer = 0;
static mut comma2: buf_pointer = 0;
static mut num_text_chars: buf_pointer = 0;
/*bad_conversion */
static mut conversion_type: u8 = 0;
static mut prev_colon: bool = false;
static mut verbose: i32 = 0;
static mut min_crossrefs: i32 = 0;
/*:473*/
/*12: *//*3: */
unsafe extern "C" fn putc_log(c: i32) {
    ttstub_output_putc(log_file, c); /* note: global! */
    ttstub_output_putc(standard_output, c);
}
unsafe extern "C" fn puts_log(mut s: *const i8) {
    let mut len: size_t = strlen(s);
    ttstub_output_write(log_file, s, len);
    ttstub_output_write(standard_output, s, len);
}
unsafe extern "C" fn ttstub_puts(mut handle: rust_output_handle_t, mut s: *const i8) {
    ttstub_output_write(handle, s, strlen(s));
}
static mut fmt_buf: [i8; 1024] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
unsafe extern "C" fn printf_log(mut fmt: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(fmt_buf.as_mut_ptr(), 1024i32 as u64, fmt, ap.as_va_list());
    puts_log(fmt_buf.as_mut_ptr());
}
unsafe extern "C" fn mark_warning() {
    if history as i32 == HISTORY_WARNING_ISSUED as i32 {
        err_count += 1
    } else if history as i32 == HISTORY_SPOTLESS as i32 {
        history = HISTORY_WARNING_ISSUED as i32 as u8;
        err_count = 1i32
    };
}
unsafe extern "C" fn mark_error() {
    if (history as i32) < HISTORY_ERROR_ISSUED as i32 {
        history = HISTORY_ERROR_ISSUED as i32 as u8;
        err_count = 1i32
    } else {
        err_count += 1
    };
}
unsafe extern "C" fn mark_fatal() {
    history = HISTORY_FATAL_ERROR as i32 as u8;
}
unsafe extern "C" fn print_overflow() {
    puts_log(b"Sorry---you\'ve exceeded BibTeX\'s \x00" as *const u8 as *const i8);
    mark_fatal();
}
unsafe extern "C" fn print_confusion() {
    puts_log(b"---this can\'t happen\n\x00" as *const u8 as *const i8);
    puts_log(b"*Please notify the BibTeX maintainer*\n\x00" as *const u8 as *const i8);
    mark_fatal();
}
unsafe extern "C" fn buffer_overflow() {
    buffer = xrealloc(
        buffer as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    sv_buffer = xrealloc(
        sv_buffer as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    ex_buf = xrealloc(
        ex_buf as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    out_buf = xrealloc(
        out_buf as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    name_tok = xrealloc(
        name_tok as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<buf_pointer>() as u64),
    ) as *mut buf_pointer;
    name_sep_char = xrealloc(
        name_sep_char as *mut libc::c_void,
        ((buf_size + 20000i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    buf_size = buf_size + 20000i32;
}
unsafe extern "C" fn input_ln(mut peekable: *mut peekable_input_t) -> bool {
    last = 0i32;
    if tectonic_eof(peekable) {
        return false;
    }
    while !eoln(peekable) {
        if last >= buf_size {
            buffer_overflow();
        }
        *buffer.offset(last as isize) = peekable_getc(peekable) as ASCII_code;
        last += 1
    }
    peekable_getc(peekable);
    while last > 0i32 {
        if !(lex_class[*buffer.offset((last - 1i32) as isize) as usize] as i32 == 1i32) {
            break;
        }
        /*white_space */
        last -= 1
    }
    true
}
unsafe extern "C" fn out_pool_str(mut handle: rust_output_handle_t, mut s: str_number) {
    let mut i: pool_pointer = 0;
    if s < 0i32 || s >= str_ptr + 3i32 || s >= max_strings {
        printf_log(
            b"Illegal string number:%ld\x00" as *const u8 as *const i8,
            s as i64,
        );
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    i = *str_start.offset(s as isize);
    while i < *str_start.offset((s + 1i32) as isize) {
        ttstub_output_putc(handle, *str_pool.offset(i as isize) as i32);
        i += 1
    }
}
unsafe extern "C" fn print_a_pool_str(mut s: str_number) {
    out_pool_str(standard_output, s);
    out_pool_str(log_file, s);
}
unsafe extern "C" fn pool_overflow() {
    str_pool = xrealloc(
        str_pool as *mut libc::c_void,
        ((pool_size as i64 + 65000 + 1i32 as i64) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    pool_size = (pool_size as i64 + 65000) as i32;
}
unsafe extern "C" fn out_token(mut handle: rust_output_handle_t) {
    let mut i: buf_pointer = buf_ptr1;
    while i < buf_ptr2 {
        let fresh0 = i;
        i = i + 1;
        ttstub_output_putc(handle, *buffer.offset(fresh0 as isize) as i32);
    }
}
unsafe extern "C" fn print_a_token() {
    out_token(standard_output);
    out_token(log_file);
}
unsafe extern "C" fn print_bad_input_line() {
    let mut bf_ptr: buf_pointer = 0;
    puts_log(b" : \x00" as *const u8 as *const i8);
    bf_ptr = 0i32;
    while bf_ptr < buf_ptr2 {
        if lex_class[*buffer.offset(bf_ptr as isize) as usize] as i32 == 1i32 {
            /*white_space */
            putc_log(' ' as i32);
        } else {
            putc_log(*buffer.offset(bf_ptr as isize) as i32);
        }
        bf_ptr += 1
    }
    putc_log('\n' as i32);
    puts_log(b" : \x00" as *const u8 as *const i8);
    bf_ptr = 0i32;
    loop {
        let fresh1 = bf_ptr;
        bf_ptr = bf_ptr + 1;
        if !(fresh1 < buf_ptr2) {
            break;
        }
        putc_log(' ' as i32);
    }
    bf_ptr = buf_ptr2;
    while bf_ptr < last {
        if lex_class[*buffer.offset(bf_ptr as isize) as usize] as i32 == 1i32 {
            /*white_space */
            putc_log(' ' as i32);
        } else {
            putc_log(*buffer.offset(bf_ptr as isize) as i32);
        }
        bf_ptr += 1
    }
    putc_log('\n' as i32);
    bf_ptr = 0i32;
    while bf_ptr < buf_ptr2 && lex_class[*buffer.offset(bf_ptr as isize) as usize] as i32 == 1i32 {
        /*white_space */
        bf_ptr += 1
    } /*empty */
    if bf_ptr == buf_ptr2 {
        puts_log(b"(Error may have been on previous line)\n\x00" as *const u8 as *const i8);
        /*any_value */
    }
    mark_error();
}
unsafe extern "C" fn print_skipping_whatever_remains() {
    puts_log(b"I\'m skipping whatever remains of this \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn sam_wrong_file_name_print() {
    ttstub_puts(
        standard_output,
        b"I couldn\'t open file name `\x00" as *const u8 as *const i8,
    );
    name_ptr = 0i32;
    while name_ptr <= name_length {
        let fresh2 = name_ptr;
        name_ptr = name_ptr + 1;
        ttstub_output_putc(
            standard_output,
            *name_of_file.offset(fresh2 as isize) as i32,
        );
    }
    ttstub_output_putc(standard_output, '\'' as i32);
    ttstub_output_putc(standard_output, '\n' as i32);
}
unsafe extern "C" fn print_aux_name() {
    print_a_pool_str(aux_list[aux_ptr as usize]);
    putc_log('\n' as i32);
}
unsafe extern "C" fn log_pr_aux_name() {
    out_pool_str(log_file, aux_list[aux_ptr as usize]);
    ttstub_output_putc(log_file, '\n' as i32);
}
unsafe extern "C" fn aux_err_print() {
    printf_log(
        b"---line %ld of file \x00" as *const u8 as *const i8,
        aux_ln_stack[aux_ptr as usize] as i64,
    );
    print_aux_name();
    print_bad_input_line();
    print_skipping_whatever_remains();
    puts_log(b"command\n\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn aux_err_illegal_another_print(mut cmd_num: i32) {
    puts_log(b"Illegal, another \\bib\x00" as *const u8 as *const i8);
    match cmd_num {
        0 => {
            puts_log(b"data\x00" as *const u8 as *const i8);
        }
        1 => {
            puts_log(b"style\x00" as *const u8 as *const i8);
        }
        _ => {
            puts_log(b"Illegal auxiliary-file command\x00" as *const u8 as *const i8);
            print_confusion();
            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
        }
    }
    puts_log(b" command\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn aux_err_no_right_brace_print() {
    puts_log(b"No \"}\"\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn aux_err_stuff_after_right_brace_print() {
    puts_log(b"Stuff after \"}\"\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn aux_err_white_space_in_argument_print() {
    puts_log(b"White space in argument\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn print_bib_name() {
    print_a_pool_str(*bib_list.offset(bib_ptr as isize));
    print_a_pool_str(s_bib_extension);
    putc_log('\n' as i32);
}
unsafe extern "C" fn log_pr_bib_name() {
    out_pool_str(log_file, *bib_list.offset(bib_ptr as isize));
    out_pool_str(log_file, s_bib_extension);
    ttstub_output_putc(log_file, '\n' as i32);
}
unsafe extern "C" fn print_bst_name() {
    print_a_pool_str(bst_str);
    print_a_pool_str(s_bst_extension);
    putc_log('\n' as i32);
}
unsafe extern "C" fn log_pr_bst_name() {
    out_pool_str(log_file, bst_str);
    out_pool_str(log_file, s_bst_extension);
    ttstub_output_putc(log_file, '\n' as i32);
}
unsafe extern "C" fn hash_cite_confusion() {
    puts_log(b"Cite hash error\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn check_cite_overflow(mut last_cite: cite_number) {
    if last_cite == max_cites {
        cite_list = xrealloc(
            cite_list as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
        ) as *mut str_number;
        type_list = xrealloc(
            type_list as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
        ) as *mut hash_ptr2;
        entry_exists = xrealloc(
            entry_exists as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64),
        ) as *mut bool;
        cite_info = xrealloc(
            cite_info as *mut libc::c_void,
            ((max_cites + 750i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
        ) as *mut str_number;
        max_cites = max_cites + 750i32;
        while last_cite < max_cites {
            *type_list.offset(last_cite as isize) = 0i32;
            *cite_info.offset(last_cite as isize) = 0i32;
            last_cite = last_cite + 1i32
        }
    };
}
unsafe extern "C" fn aux_end1_err_print() {
    puts_log(b"I found no \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn aux_end2_err_print() {
    puts_log(b"---while reading file \x00" as *const u8 as *const i8);
    print_aux_name();
    mark_error();
}
unsafe extern "C" fn bst_ln_num_print() {
    printf_log(
        b"--line %ld of file \x00" as *const u8 as *const i8,
        bst_line_num as i64,
    );
    print_bst_name();
}
unsafe extern "C" fn bst_err_print_and_look_for_blank_line() {
    putc_log('-' as i32);
    bst_ln_num_print();
    print_bad_input_line();
    while last != 0i32 {
        if !input_ln(bst_file) {
            longjmp(recover_jmpbuf.as_mut_ptr(), 1i32);
        } else {
            bst_line_num = bst_line_num + 1i32
        }
    }
    buf_ptr2 = last;
}
unsafe extern "C" fn bst_warn_print() {
    bst_ln_num_print();
    mark_warning();
}
unsafe extern "C" fn eat_bst_print() {
    puts_log(b"Illegal end of style file in command: \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn unknwn_function_class_confusion() {
    puts_log(b"Unknown function class\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn print_fn_class(mut fn_loc_0: hash_loc) {
    match *fn_type.offset(fn_loc_0 as isize) as i32 {
        0 => {
            puts_log(b"built-in\x00" as *const u8 as *const i8);
        }
        1 => {
            puts_log(b"wizard-defined\x00" as *const u8 as *const i8);
        }
        2 => {
            puts_log(b"integer-literal\x00" as *const u8 as *const i8);
        }
        3 => {
            puts_log(b"string-literal\x00" as *const u8 as *const i8);
        }
        4 => {
            puts_log(b"field\x00" as *const u8 as *const i8);
        }
        5 => {
            puts_log(b"integer-entry-variable\x00" as *const u8 as *const i8);
        }
        6 => {
            puts_log(b"string-entry-variable\x00" as *const u8 as *const i8);
        }
        7 => {
            puts_log(b"integer-global-variable\x00" as *const u8 as *const i8);
        }
        8 => {
            puts_log(b"string-global-variable\x00" as *const u8 as *const i8);
        }
        _ => {
            unknwn_function_class_confusion();
        }
    };
}
/*:159*/
/*160: */
unsafe extern "C" fn id_scanning_confusion() {
    puts_log(b"Identifier scanning error\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn bst_id_print() {
    if scan_result as i32 == 0i32 {
        /*id_null */
        printf_log(
            b"\"%c\" begins identifier, command: \x00" as *const u8 as *const i8,
            *buffer.offset(buf_ptr2 as isize) as i32,
        );
    } else if scan_result as i32 == 2i32 {
        /*other_char_adjacent */
        printf_log(
            b"\"%c\" immediately follows identifier, command: \x00" as *const u8 as *const i8,
            *buffer.offset(buf_ptr2 as isize) as i32,
        );
    } else {
        id_scanning_confusion();
    };
}
unsafe extern "C" fn bst_left_brace_print() {
    puts_log(b"\"{\" is missing in command: \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn bst_right_brace_print() {
    puts_log(b"\"}\" is missing in command: \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn already_seen_function_print(mut seen_fn_loc: hash_loc) {
    print_a_pool_str(*hash_text.offset(seen_fn_loc as isize));
    puts_log(b" is already a type \"\x00" as *const u8 as *const i8);
    print_fn_class(seen_fn_loc);
    puts_log(b"\" function name\n\x00" as *const u8 as *const i8);
    bst_err_print_and_look_for_blank_line();
}
unsafe extern "C" fn bib_ln_num_print() {
    printf_log(
        b"--line %ld of file\x00" as *const u8 as *const i8,
        bib_line_num as i64,
    );
    print_bib_name();
}
unsafe extern "C" fn bib_err_print() {
    putc_log('-' as i32);
    bib_ln_num_print();
    print_bad_input_line();
    print_skipping_whatever_remains();
    if at_bib_command {
        puts_log(b"command\n\x00" as *const u8 as *const i8);
    } else {
        puts_log(b"entry\n\x00" as *const u8 as *const i8);
    };
}
unsafe extern "C" fn bib_warn_print() {
    bib_ln_num_print();
    mark_warning();
}
unsafe extern "C" fn check_field_overflow(mut total_fields: i32) {
    let mut f_ptr: field_loc = 0;
    let mut start_fields: field_loc = 0;
    if total_fields > max_fields {
        start_fields = max_fields;
        field_info = xrealloc(
            field_info as *mut libc::c_void,
            ((total_fields + 17250i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
        ) as *mut str_number;
        max_fields = total_fields + 17250i32;
        let mut for_end: i32 = 0;
        f_ptr = start_fields;
        for_end = max_fields - 1i32;
        if f_ptr <= for_end {
            loop {
                *field_info.offset(f_ptr as isize) = 0i32;
                let fresh3 = f_ptr;
                f_ptr = f_ptr + 1;
                if !(fresh3 < for_end) {
                    break;
                }
                /*missing */
            }
        }
    };
}
unsafe extern "C" fn eat_bib_print() {
    puts_log(b"Illegal end of database file\x00" as *const u8 as *const i8);
    bib_err_print();
}
unsafe extern "C" fn bib_one_of_two_print(mut char1: ASCII_code, mut char2: ASCII_code) {
    printf_log(
        b"I was expecting a `%c\' or a `%c\'\x00" as *const u8 as *const i8,
        char1 as i32,
        char2 as i32,
    );
    bib_err_print();
}
unsafe extern "C" fn bib_equals_sign_print() {
    printf_log(b"I was expecting an \"=\"\x00" as *const u8 as *const i8);
    bib_err_print();
}
unsafe extern "C" fn bib_unbalanced_braces_print() {
    puts_log(b"Unbalanced braces\x00" as *const u8 as *const i8);
    bib_err_print();
}
unsafe extern "C" fn bib_field_too_long_print() {
    printf_log(
        b"Your field is more than %ld characters\x00" as *const u8 as *const i8,
        buf_size as i64,
    );
    bib_err_print();
}
unsafe extern "C" fn macro_warn_print() {
    puts_log(b"Warning--string name \"\x00" as *const u8 as *const i8);
    print_a_token();
    puts_log(b"\" is \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn bib_id_print() {
    if scan_result as i32 == 0i32 {
        /*id_null */
        puts_log(b"You\'re missing \x00" as *const u8 as *const i8);
    } else if scan_result as i32 == 2i32 {
        /*other_char_adjacent */
        printf_log(
            b"\"%c\" immediately follows \x00" as *const u8 as *const i8,
            *buffer.offset(buf_ptr2 as isize) as i32,
        );
    } else {
        id_scanning_confusion();
    };
}
unsafe extern "C" fn bib_cmd_confusion() {
    puts_log(b"Unknown database-file command\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn cite_key_disappeared_confusion() {
    puts_log(b"A cite key disappeared\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn bad_cross_reference_print(mut s: str_number) {
    puts_log(b"--entry \"\x00" as *const u8 as *const i8);
    print_a_pool_str(*cite_list.offset(cite_ptr as isize));
    putc_log('\"' as i32);
    putc_log('\n' as i32);
    puts_log(b"refers to entry \"\x00" as *const u8 as *const i8);
    print_a_pool_str(s);
}
unsafe extern "C" fn nonexistent_cross_reference_error() {
    puts_log(b"A bad cross reference-\x00" as *const u8 as *const i8);
    bad_cross_reference_print(*field_info.offset(field_ptr as isize));
    puts_log(b"\", which doesn\'t exist\n\x00" as *const u8 as *const i8);
    mark_error();
}
unsafe extern "C" fn print_missing_entry(mut s: str_number) {
    puts_log(b"Warning--I didn\'t find a database entry for \"\x00" as *const u8 as *const i8);
    print_a_pool_str(s);
    putc_log('\"' as i32);
    putc_log('\n' as i32);
    mark_warning();
}
unsafe extern "C" fn bst_ex_warn_print() {
    if mess_with_entries {
        puts_log(b" for entry \x00" as *const u8 as *const i8);
        print_a_pool_str(*cite_list.offset(cite_ptr as isize));
    }
    putc_log('\n' as i32);
    puts_log(b"while executing-\x00" as *const u8 as *const i8);
    bst_ln_num_print();
    mark_error();
}
unsafe extern "C" fn bst_mild_ex_warn_print() {
    if mess_with_entries {
        puts_log(b" for entry \x00" as *const u8 as *const i8);
        print_a_pool_str(*cite_list.offset(cite_ptr as isize));
    }
    putc_log('\n' as i32);
    puts_log(b"while executing\x00" as *const u8 as *const i8);
    bst_warn_print();
}
unsafe extern "C" fn bst_cant_mess_with_entries_print() {
    puts_log(b"You can\'t mess with entries here\x00" as *const u8 as *const i8);
    bst_ex_warn_print();
}
unsafe extern "C" fn illegl_literal_confusion() {
    puts_log(b"Illegal literal type\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn unknwn_literal_confusion() {
    puts_log(b"Unknown literal type\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn print_stk_lit(mut stk_lt: i32, mut stk_tp: stk_type) {
    match stk_tp as i32 {
        0 => {
            printf_log(
                b"%ld is an integer literal\x00" as *const u8 as *const i8,
                stk_lt as i64,
            );
        }
        1 => {
            putc_log('\"' as i32);
            print_a_pool_str(stk_lt);
            puts_log(b"\" is a string literal\x00" as *const u8 as *const i8);
        }
        2 => {
            putc_log('`' as i32);
            print_a_pool_str(*hash_text.offset(stk_lt as isize));
            puts_log(b"\' is a function literal\x00" as *const u8 as *const i8);
        }
        3 => {
            putc_log('`' as i32);
            print_a_pool_str(stk_lt);
            puts_log(b"\' is a missing field\x00" as *const u8 as *const i8);
        }
        4 => {
            illegl_literal_confusion();
        }
        _ => {
            unknwn_literal_confusion();
        }
    };
}
unsafe extern "C" fn print_lit(mut stk_lt: i32, mut stk_tp: stk_type) {
    match stk_tp as i32 {
        0 => {
            printf_log(b"%ld\n\x00" as *const u8 as *const i8, stk_lt as i64);
        }
        1 => {
            print_a_pool_str(stk_lt);
            putc_log('\n' as i32);
        }
        2 => {
            print_a_pool_str(*hash_text.offset(stk_lt as isize));
            putc_log('\n' as i32);
        }
        3 => {
            print_a_pool_str(stk_lt);
            putc_log('\n' as i32);
        }
        4 => {
            illegl_literal_confusion();
        }
        _ => {
            unknwn_literal_confusion();
        }
    };
}
unsafe extern "C" fn output_bbl_line() {
    if out_buf_length != 0i32 {
        while out_buf_length > 0i32 {
            if !(lex_class[*out_buf.offset((out_buf_length - 1i32) as isize) as usize] as i32
                == 1i32)
            {
                break;
            }
            /*white_space */
            out_buf_length = out_buf_length - 1i32
        }
        if out_buf_length == 0i32 {
            return;
        }
        out_buf_ptr = 0i32;
        while out_buf_ptr < out_buf_length {
            ttstub_output_putc(bbl_file, *out_buf.offset(out_buf_ptr as isize) as i32);
            out_buf_ptr += 1
        }
    }
    ttstub_output_putc(bbl_file, '\n' as i32);
    bbl_line_num += 1;
    out_buf_length = 0i32;
}
unsafe extern "C" fn bst_1print_string_size_exceeded() {
    puts_log(b"Warning--you\'ve exceeded \x00" as *const u8 as *const i8);
}
unsafe extern "C" fn bst_2print_string_size_exceeded() {
    puts_log(b"-string-size,\x00" as *const u8 as *const i8);
    bst_mild_ex_warn_print();
    puts_log(b"*Please notify the bibstyle designer*\n\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn braces_unbalanced_complaint(mut pop_lit_var: str_number) {
    puts_log(b"Warning--\"\x00" as *const u8 as *const i8);
    print_a_pool_str(pop_lit_var);
    puts_log(b"\" isn\'t a brace-balanced string\x00" as *const u8 as *const i8);
    bst_mild_ex_warn_print();
}
unsafe extern "C" fn case_conversion_confusion() {
    puts_log(b"Unknown type of case conversion\x00" as *const u8 as *const i8);
    print_confusion();
    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn start_name(mut file_name: str_number) {
    let mut p_ptr: pool_pointer = 0;
    free(name_of_file as *mut libc::c_void);
    name_of_file = xmalloc(
        ((*str_start.offset((file_name + 1i32) as isize) - *str_start.offset(file_name as isize)
            + 1i32
            + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    name_ptr = 0i32;
    p_ptr = *str_start.offset(file_name as isize);
    while p_ptr < *str_start.offset((file_name + 1i32) as isize) {
        *name_of_file.offset(name_ptr as isize) = *str_pool.offset(p_ptr as isize);
        name_ptr += 1;
        p_ptr += 1
    }
    name_length =
        *str_start.offset((file_name + 1i32) as isize) - *str_start.offset(file_name as isize);
    *name_of_file.offset(name_length as isize) = 0i32 as ASCII_code;
}
unsafe extern "C" fn add_extension(mut ext: str_number) {
    let mut p_ptr: pool_pointer = 0;
    name_ptr = name_length;
    p_ptr = *str_start.offset(ext as isize);
    while p_ptr < *str_start.offset((ext + 1i32) as isize) {
        *name_of_file.offset(name_ptr as isize) = *str_pool.offset(p_ptr as isize);
        name_ptr += 1;
        p_ptr += 1
    }
    name_length += *str_start.offset((ext + 1i32) as isize) - *str_start.offset(ext as isize);
    *name_of_file.offset(name_length as isize) = 0i32 as ASCII_code;
}
unsafe extern "C" fn make_string() -> str_number {
    if str_ptr == max_strings {
        print_overflow();
        printf_log(
            b"number of strings %ld\n\x00" as *const u8 as *const i8,
            max_strings as i64,
        );
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    str_ptr = str_ptr + 1i32;
    *str_start.offset(str_ptr as isize) = pool_ptr;
    str_ptr - 1i32
}
unsafe extern "C" fn str_eq_buf(
    mut s: str_number,
    mut buf: buf_type,
    mut bf_ptr: buf_pointer,
    mut len: buf_pointer,
) -> bool {
    let mut i: buf_pointer = 0;
    let mut j: pool_pointer = 0;
    if *str_start.offset((s + 1i32) as isize) - *str_start.offset(s as isize) != len {
        return false;
    }
    i = bf_ptr;
    j = *str_start.offset(s as isize);
    while j < *str_start.offset((s + 1i32) as isize) {
        if *str_pool.offset(j as isize) as i32 != *buf.offset(i as isize) as i32 {
            return false;
        }
        i = i + 1i32;
        j = j + 1i32
    }
    true
}
unsafe extern "C" fn str_eq_str(mut s1: str_number, mut s2: str_number) -> bool {
    if *str_start.offset((s1 + 1i32) as isize) - *str_start.offset(s1 as isize)
        != *str_start.offset((s2 + 1i32) as isize) - *str_start.offset(s2 as isize)
    {
        return false;
    }
    p_ptr1 = *str_start.offset(s1 as isize);
    p_ptr2 = *str_start.offset(s2 as isize);
    while p_ptr1 < *str_start.offset((s1 + 1i32) as isize) {
        if *str_pool.offset(p_ptr1 as isize) as i32 != *str_pool.offset(p_ptr2 as isize) as i32 {
            return false;
        }
        p_ptr1 = p_ptr1 + 1i32;
        p_ptr2 = p_ptr2 + 1i32
    }
    true
}
unsafe extern "C" fn lower_case(mut buf: buf_type, mut bf_ptr: buf_pointer, mut len: buf_pointer) {
    let mut i: buf_pointer = 0;
    if len > 0i32 {
        let mut for_end: i32 = 0;
        i = bf_ptr;
        for_end = bf_ptr + len - 1i32;
        if i <= for_end {
            loop {
                if *buf.offset(i as isize) as i32 >= 'A' as i32
                    && *buf.offset(i as isize) as i32 <= 'Z' as i32
                {
                    *buf.offset(i as isize) = (*buf.offset(i as isize) as i32 + 32i32) as ASCII_code
                }
                let fresh4 = i;
                i = i + 1;
                if !(fresh4 < for_end) {
                    break;
                }
            }
        }
    };
}
unsafe extern "C" fn upper_case(mut buf: buf_type, mut bf_ptr: buf_pointer, mut len: buf_pointer) {
    let mut i: buf_pointer = 0;
    if len > 0i32 {
        let mut for_end: i32 = 0;
        i = bf_ptr;
        for_end = bf_ptr + len - 1i32;
        if i <= for_end {
            loop {
                if *buf.offset(i as isize) as i32 >= 'a' as i32
                    && *buf.offset(i as isize) as i32 <= 'z' as i32
                {
                    *buf.offset(i as isize) = (*buf.offset(i as isize) as i32 - 32i32) as ASCII_code
                }
                let fresh5 = i;
                i = i + 1;
                if !(fresh5 < for_end) {
                    break;
                }
            }
        }
    };
}
unsafe extern "C" fn str_lookup(
    mut buf: buf_type,
    mut j: buf_pointer,
    mut l: buf_pointer,
    mut ilk: str_ilk,
    mut insert_it: bool,
) -> hash_loc {
    let mut h: i32 = 0;
    let mut p: hash_loc = 0;
    let mut k: buf_pointer = 0;
    let mut str_num: str_number = 0;
    h = 0i32;
    k = j;
    while k < j + l {
        h = h + h + *buf.offset(k as isize) as i32;
        while h >= hash_prime {
            h = h - hash_prime
        }
        k = k + 1i32
    }
    p = h + 1i32;
    hash_found = false;
    str_num = 0i32;
    loop {
        if *hash_text.offset(p as isize) > 0i32 {
            if str_eq_buf(*hash_text.offset(p as isize), buf, j, l) {
                if *hash_ilk.offset(p as isize) as i32 == ilk as i32 {
                    hash_found = true;
                    return p;
                /* str_found */
                } else {
                    str_num = *hash_text.offset(p as isize)
                }
            }
        }
        if *hash_next.offset(p as isize) == 0i32 {
            /*empty */
            if !insert_it {
                return p;
            } /* str_not_found */
            if *hash_text.offset(p as isize) > 0i32 {
                loop {
                    if hash_used == 1i32 {
                        print_overflow();
                        printf_log(
                            b"hash size %ld\n\x00" as *const u8 as *const i8,
                            hash_size as i64,
                        );
                        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                    }
                    hash_used = hash_used - 1i32;
                    if *hash_text.offset(hash_used as isize) == 0i32 {
                        break;
                    }
                }
                *hash_next.offset(p as isize) = hash_used;
                p = hash_used
            }
            if str_num > 0i32 {
                *hash_text.offset(p as isize) = str_num
            } else {
                while pool_ptr + l > pool_size {
                    pool_overflow();
                }
                k = j;
                while k < j + l {
                    *str_pool.offset(pool_ptr as isize) = *buf.offset(k as isize);
                    pool_ptr = pool_ptr + 1i32;
                    k = k + 1i32
                }
                *hash_text.offset(p as isize) = make_string()
            }
            *hash_ilk.offset(p as isize) = ilk;
            return p;
        }
        p = *hash_next.offset(p as isize)
    }
}
unsafe extern "C" fn pre_define(mut pds: pds_type, mut len: pds_len, mut ilk: str_ilk) {
    let mut i: pds_len = 0;
    let mut for_end: i32 = 0;
    i = 1i32 as pds_len;
    for_end = len as i32;
    if i as i32 <= for_end {
        loop {
            *buffer.offset(i as isize) = *pds.offset((i as i32 - 1i32) as isize) as u8;
            let fresh6 = i;
            i = i.wrapping_add(1);
            if !((fresh6 as i32) < for_end) {
                break;
            }
        }
    }
    pre_def_loc = str_lookup(buffer, 1i32, len as buf_pointer, ilk, true);
}
unsafe extern "C" fn int_to_ASCII(
    mut the_int: i32,
    mut int_buf: buf_type,
    mut int_begin: buf_pointer,
    mut int_end: *mut buf_pointer,
) {
    let mut int_ptr: buf_pointer = 0;
    let mut int_xptr: buf_pointer = 0;
    let mut int_tmp_val: ASCII_code = 0;
    int_ptr = int_begin;
    if the_int < 0i32 {
        if int_ptr == buf_size {
            buffer_overflow();
        }
        /* str_found */
        *int_buf.offset(int_ptr as isize) = 45i32 as ASCII_code; /*minus_sign */
        int_ptr = int_ptr + 1i32;
        the_int = -the_int
    }
    int_xptr = int_ptr;
    loop {
        if int_ptr == buf_size {
            buffer_overflow();
        }
        *int_buf.offset(int_ptr as isize) = ('0' as i32 + the_int % 10i32) as ASCII_code;
        int_ptr = int_ptr + 1i32;
        the_int = the_int / 10i32;
        if the_int == 0i32 {
            break;
        }
    }
    *int_end = int_ptr;
    int_ptr = int_ptr - 1i32;
    while int_xptr < int_ptr {
        int_tmp_val = *int_buf.offset(int_xptr as isize);
        *int_buf.offset(int_xptr as isize) = *int_buf.offset(int_ptr as isize);
        *int_buf.offset(int_ptr as isize) = int_tmp_val;
        int_ptr = int_ptr - 1i32;
        int_xptr = int_xptr + 1i32
    }
}
unsafe extern "C" fn add_database_cite(mut new_cite: *mut cite_number) {
    check_cite_overflow(*new_cite);
    check_field_overflow(num_fields * (*new_cite + 1i32));
    *cite_list.offset(*new_cite as isize) = *hash_text.offset(cite_loc as isize);
    *ilk_info.offset(cite_loc as isize) = *new_cite;
    *ilk_info.offset(lc_cite_loc as isize) = cite_loc;
    *new_cite = *new_cite + 1i32;
}
unsafe extern "C" fn find_cite_locs_for_this_cite_key(mut cite_str: str_number) -> bool {
    ex_buf_ptr = 0i32;
    tmp_ptr = *str_start.offset(cite_str as isize);
    tmp_end_ptr = *str_start.offset((cite_str + 1i32) as isize);
    while tmp_ptr < tmp_end_ptr {
        *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(tmp_ptr as isize);
        ex_buf_ptr = ex_buf_ptr + 1i32;
        tmp_ptr = tmp_ptr + 1i32
    }
    cite_loc = str_lookup(
        ex_buf,
        0i32,
        *str_start.offset((cite_str + 1i32) as isize) - *str_start.offset(cite_str as isize),
        9i32 as str_ilk,
        false,
    );
    cite_hash_found = hash_found;
    lower_case(
        ex_buf,
        0i32,
        *str_start.offset((cite_str + 1i32) as isize) - *str_start.offset(cite_str as isize),
    );
    lc_cite_loc = str_lookup(
        ex_buf,
        0i32,
        *str_start.offset((cite_str + 1i32) as isize) - *str_start.offset(cite_str as isize),
        10i32 as str_ilk,
        false,
    );
    hash_found
}
unsafe extern "C" fn swap(mut swap1: cite_number, mut swap2: cite_number) {
    let mut innocent_bystander: cite_number = 0;
    innocent_bystander = *cite_info.offset(swap2 as isize);
    *cite_info.offset(swap2 as isize) = *cite_info.offset(swap1 as isize);
    *cite_info.offset(swap1 as isize) = innocent_bystander;
}
unsafe extern "C" fn less_than(mut arg1: cite_number, mut arg2: cite_number) -> bool {
    let mut char_ptr: i32 = 0;
    let mut ptr1: str_ent_loc = 0;
    let mut ptr2: str_ent_loc = 0;
    let mut char1: ASCII_code = 0;
    let mut char2: ASCII_code = 0;
    ptr1 = arg1 * num_ent_strs + sort_key_num;
    ptr2 = arg2 * num_ent_strs + sort_key_num;
    char_ptr = 0i32;
    loop {
        char1 = *entry_strs.offset((ptr1 * (ent_str_size + 1i32) + char_ptr) as isize);
        char2 = *entry_strs.offset((ptr2 * (ent_str_size + 1i32) + char_ptr) as isize);
        if char1 as i32 == 127i32 {
            /*end_of_string */
            if char2 as i32 == 127i32 {
                /*end_of_string */
                if arg1 < arg2 {
                    return true;
                } else if arg1 > arg2 {
                    return false;
                } else {
                    puts_log(b"Duplicate sort key\x00" as *const u8 as *const i8);
                    print_confusion();
                    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                }
            } else {
                return true;
            }
        } else {
            if char2 as i32 == 127i32 {
                /*end_of_string */
                return false;
            } else {
                if (char1 as i32) < char2 as i32 {
                    return true;
                } else {
                    if char1 as i32 > char2 as i32 {
                        return false;
                    }
                }
            }
        }
        char_ptr = char_ptr + 1i32
    }
}
unsafe extern "C" fn quick_sort(mut left_end: cite_number, mut right_end: cite_number) {
    let mut left: cite_number = 0;
    let mut right: cite_number = 0;
    let mut insert_ptr: cite_number = 0;
    let mut middle: cite_number = 0;
    let mut partition: cite_number = 0;
    if right_end - left_end < 10i32 {
        /*short_list */
        /*305: */
        let mut for_end: i32 = 0; /*built_in */
        insert_ptr = left_end + 1i32; /*n_aux_citation */
        for_end = right_end; /*n_aux_bibdata */
        if insert_ptr <= for_end {
            loop {
                let mut for_end_0: i32 = 0; /*n_aux_bibstyle */
                right = insert_ptr; /*n_aux_input */
                for_end_0 = left_end + 1i32; /*n_bst_entry */
                if right >= for_end_0 {
                    while !less_than(
                        *cite_info.offset((right - 1i32) as isize),
                        *cite_info.offset(right as isize),
                    ) {
                        swap(right - 1i32, right); /*n_bst_execute */
                        let fresh7 = right; /*n_bst_function */
                        right = right - 1; /*n_bst_integers */
                        if !(fresh7 > for_end_0) {
                            break; /*n_bst_iterate */
                        }
                    }
                } /*n_bst_macro */
                let fresh8 = insert_ptr; /*n_bst_read */
                insert_ptr = insert_ptr + 1; /*n_bst_reverse */
                if !(fresh8 < for_end) {
                    break; /*n_bst_sort */
                }
            }
        }
    } else {
        left = left_end + 4i32; /*n_bst_strings */
        middle = (left_end + right_end) / 2i32; /*n_bib_comment */
        right = right_end - 4i32; /*n_bib_preamble */
        if less_than(
            *cite_info.offset(left as isize),
            *cite_info.offset(middle as isize),
        ) {
            if less_than(
                *cite_info.offset(middle as isize),
                *cite_info.offset(right as isize),
            ) {
                swap(left_end, middle); /*n_bib_string */
            } else if less_than(
                *cite_info.offset(left as isize),
                *cite_info.offset(right as isize),
            ) {
                swap(left_end, right); /*str_literal */
            } else {
                swap(left_end, left); /*str_literal */
            }
        } else if less_than(
            *cite_info.offset(right as isize),
            *cite_info.offset(middle as isize),
        ) {
            swap(left_end, middle); /*n_i */
        } else if less_than(
            *cite_info.offset(right as isize),
            *cite_info.offset(left as isize),
        ) {
            swap(left_end, right); /*n_j */
        } else {
            swap(left_end, left); /*n_oe */
        } /*n_oe_upper */
        partition = *cite_info.offset(left_end as isize); /*n_ae */
        left = left_end + 1i32; /*n_ae_upper */
        right = right_end; /*n_aa */
        loop {
            while less_than(*cite_info.offset(left as isize), partition) {
                left = left + 1i32
            } /*n_aa_upper */
            while less_than(partition, *cite_info.offset(right as isize)) {
                right = right - 1i32
            } /*n_o */
            if left < right {
                swap(left, right); /*n_o_upper */
                left = left + 1i32; /*n_l */
                right = right - 1i32
            } /*n_l_upper */
            if left == right + 1i32 {
                break; /*n_ss */
            }
        } /*field */
        swap(left_end, right); /*str_entry_var */
        quick_sort(left_end, right - 1i32); /*int_global_var */
        quick_sort(left, right_end); /*int_global_var */
    };
}
unsafe extern "C" fn build_in(
    mut pds: pds_type,
    mut len: pds_len,
    mut fn_hash_loc: *mut hash_loc,
    mut blt_in_num: blt_in_range,
) {
    pre_define(pds, len, 11i32 as str_ilk);
    *fn_hash_loc = pre_def_loc;
    *fn_type.offset(*fn_hash_loc as isize) = 0i32 as fn_class;
    *ilk_info.offset(*fn_hash_loc as isize) = blt_in_num;
}
unsafe extern "C" fn pre_def_certain_strings() {
    pre_define(
        b".aux        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        7i32 as str_ilk,
    );
    s_aux_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b".bbl        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        7i32 as str_ilk,
    );
    s_bbl_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b".blg        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        7i32 as str_ilk,
    );
    s_log_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b".bst        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        7i32 as str_ilk,
    );
    s_bst_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b".bib        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        7i32 as str_ilk,
    );
    s_bib_extension = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b"texinputs:  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        8i32 as str_ilk,
    );
    s_bst_area = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b"texbib:     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        8i32 as str_ilk,
    );
    s_bib_area = *hash_text.offset(pre_def_loc as isize);
    pre_define(
        b"\\citation   \x00" as *const u8 as *const i8,
        9i32 as pds_len,
        2i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    pre_define(
        b"\\bibdata    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        2i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(
        b"\\bibstyle   \x00" as *const u8 as *const i8,
        9i32 as pds_len,
        2i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(
        b"\\@input     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        2i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 3i32;
    pre_define(
        b"entry       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(
        b"execute     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(
        b"function    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    pre_define(
        b"integers    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 3i32;
    pre_define(
        b"iterate     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 4i32;
    pre_define(
        b"macro       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 5i32;
    pre_define(
        b"read        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 6i32;
    pre_define(
        b"reverse     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 7i32;
    pre_define(
        b"sort        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 8i32;
    pre_define(
        b"strings     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        4i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 9i32;
    pre_define(
        b"comment     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        12i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(
        b"preamble    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        12i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(
        b"string      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        12i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    build_in(
        b"=           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_equals,
        0i32,
    );
    build_in(
        b">           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_greater_than,
        1i32,
    );
    build_in(
        b"<           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_less_than,
        2i32,
    );
    build_in(
        b"+           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_plus,
        3i32,
    );
    build_in(
        b"-           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_minus,
        4i32,
    );
    build_in(
        b"*           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        &mut b_concatenate,
        5i32,
    );
    build_in(
        b":=          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        &mut b_gets,
        6i32,
    );
    build_in(
        b"add.period$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_add_period,
        7i32,
    );
    build_in(
        b"call.type$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_call_type,
        8i32,
    );
    build_in(
        b"change.case$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_change_case,
        9i32,
    );
    build_in(
        b"chr.to.int$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_chr_to_int,
        10i32,
    );
    build_in(
        b"cite$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_cite,
        11i32,
    );
    build_in(
        b"duplicate$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_duplicate,
        12i32,
    );
    build_in(
        b"empty$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_empty,
        13i32,
    );
    build_in(
        b"format.name$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_format_name,
        14i32,
    );
    build_in(
        b"if$         \x00" as *const u8 as *const i8,
        3i32 as pds_len,
        &mut b_if,
        15i32,
    );
    build_in(
        b"int.to.chr$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_int_to_chr,
        16i32,
    );
    build_in(
        b"int.to.str$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        &mut b_int_to_str,
        17i32,
    );
    build_in(
        b"missing$    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        &mut b_missing,
        18i32,
    );
    build_in(
        b"newline$    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        &mut b_newline,
        19i32,
    );
    build_in(
        b"num.names$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_num_names,
        20i32,
    );
    build_in(
        b"pop$        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        &mut b_pop,
        21i32,
    );
    build_in(
        b"preamble$   \x00" as *const u8 as *const i8,
        9i32 as pds_len,
        &mut b_preamble,
        22i32,
    );
    build_in(
        b"purify$     \x00" as *const u8 as *const i8,
        7i32 as pds_len,
        &mut b_purify,
        23i32,
    );
    build_in(
        b"quote$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_quote,
        24i32,
    );
    build_in(
        b"skip$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_skip,
        25i32,
    );
    build_in(
        b"stack$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_stack,
        26i32,
    );
    build_in(
        b"substring$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        &mut b_substring,
        27i32,
    );
    build_in(
        b"swap$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_swap,
        28i32,
    );
    build_in(
        b"text.length$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_text_length,
        29i32,
    );
    build_in(
        b"text.prefix$\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        &mut b_text_prefix,
        30i32,
    );
    build_in(
        b"top$        \x00" as *const u8 as *const i8,
        4i32 as pds_len,
        &mut b_top_stack,
        31i32,
    );
    build_in(
        b"type$       \x00" as *const u8 as *const i8,
        5i32 as pds_len,
        &mut b_type,
        32i32,
    );
    build_in(
        b"warning$    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        &mut b_warning,
        33i32,
    );
    build_in(
        b"while$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_while,
        34i32,
    );
    build_in(
        b"width$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_width,
        35i32,
    );
    build_in(
        b"write$      \x00" as *const u8 as *const i8,
        6i32 as pds_len,
        &mut b_write,
        36i32,
    );
    pre_define(
        b"            \x00" as *const u8 as *const i8,
        0i32 as pds_len,
        0i32 as str_ilk,
    );
    s_null = *hash_text.offset(pre_def_loc as isize);
    *fn_type.offset(pre_def_loc as isize) = 3i32 as fn_class;
    pre_define(
        b"default.type\x00" as *const u8 as *const i8,
        12i32 as pds_len,
        0i32 as str_ilk,
    );
    s_default = *hash_text.offset(pre_def_loc as isize);
    *fn_type.offset(pre_def_loc as isize) = 3i32 as fn_class;
    b_default = b_skip;
    preamble_ptr = 0i32;
    pre_define(
        b"i           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 0i32;
    pre_define(
        b"j           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 1i32;
    pre_define(
        b"oe          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 2i32;
    pre_define(
        b"OE          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 3i32;
    pre_define(
        b"ae          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 4i32;
    pre_define(
        b"AE          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 5i32;
    pre_define(
        b"aa          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 6i32;
    pre_define(
        b"AA          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 7i32;
    pre_define(
        b"o           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 8i32;
    pre_define(
        b"O           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 9i32;
    pre_define(
        b"l           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 10i32;
    pre_define(
        b"L           \x00" as *const u8 as *const i8,
        1i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 11i32;
    pre_define(
        b"ss          \x00" as *const u8 as *const i8,
        2i32 as pds_len,
        14i32 as str_ilk,
    );
    *ilk_info.offset(pre_def_loc as isize) = 12i32;
    pre_define(
        b"crossref    \x00" as *const u8 as *const i8,
        8i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = 4i32 as fn_class;
    *ilk_info.offset(pre_def_loc as isize) = num_fields;
    crossref_num = num_fields;
    num_fields = num_fields + 1i32;
    num_pre_defined_fields = num_fields;
    pre_define(
        b"sort.key$   \x00" as *const u8 as *const i8,
        9i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = 6i32 as fn_class;
    *ilk_info.offset(pre_def_loc as isize) = num_ent_strs;
    sort_key_num = num_ent_strs;
    num_ent_strs = num_ent_strs + 1i32;
    pre_define(
        b"entry.max$  \x00" as *const u8 as *const i8,
        10i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = 7i32 as fn_class;
    *ilk_info.offset(pre_def_loc as isize) = ent_str_size;
    pre_define(
        b"global.max$ \x00" as *const u8 as *const i8,
        11i32 as pds_len,
        11i32 as str_ilk,
    );
    *fn_type.offset(pre_def_loc as isize) = 7i32 as fn_class;
    *ilk_info.offset(pre_def_loc as isize) = glob_str_size;
}
unsafe extern "C" fn scan1(mut char1: ASCII_code) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last && *buffer.offset(buf_ptr2 as isize) as i32 != char1 as i32 {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 < last
}
unsafe extern "C" fn scan1_white(mut char1: ASCII_code) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 != 1i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char1 as i32
    {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 < last
}
unsafe extern "C" fn scan2(mut char1: ASCII_code, mut char2: ASCII_code) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && *buffer.offset(buf_ptr2 as isize) as i32 != char1 as i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char2 as i32
    {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 < last
}
unsafe extern "C" fn scan2_white(mut char1: ASCII_code, mut char2: ASCII_code) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && *buffer.offset(buf_ptr2 as isize) as i32 != char1 as i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char2 as i32
        && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 != 1i32
    {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 < last
}
unsafe extern "C" fn scan3(
    mut char1: ASCII_code,
    mut char2: ASCII_code,
    mut char3: ASCII_code,
) -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last
        && *buffer.offset(buf_ptr2 as isize) as i32 != char1 as i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char2 as i32
        && *buffer.offset(buf_ptr2 as isize) as i32 != char3 as i32
    {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 < last
}
unsafe extern "C" fn scan_alpha() -> bool {
    buf_ptr1 = buf_ptr2;
    while buf_ptr2 < last && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 2i32 {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 - buf_ptr1 != 0i32
}
unsafe extern "C" fn scan_identifier(
    mut char1: ASCII_code,
    mut char2: ASCII_code,
    mut char3: ASCII_code,
) {
    buf_ptr1 = buf_ptr2;
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 != 3i32 {
        /*numeric */
        while buf_ptr2 < last && id_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32
        {
            buf_ptr2 = buf_ptr2 + 1i32
        }
    } /*id_null */
    if buf_ptr2 - buf_ptr1 == 0i32 {
        scan_result = 0_u8
    } else if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32
        || buf_ptr2 == last
    {
        scan_result = 3_u8
    } else if *buffer.offset(buf_ptr2 as isize) as i32 == char1 as i32
        || *buffer.offset(buf_ptr2 as isize) as i32 == char2 as i32
        || *buffer.offset(buf_ptr2 as isize) as i32 == char3 as i32
    {
        /*white_adjacent */
        scan_result = 1_u8
    } else {
        scan_result = 2_u8
    }; /*specified_char_adjacent */
    /*other_char_adjacent */
}
unsafe extern "C" fn scan_nonneg_integer() -> bool {
    buf_ptr1 = buf_ptr2;
    token_value = 0i32;
    while buf_ptr2 < last && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 3i32 {
        token_value = token_value * 10i32 + (*buffer.offset(buf_ptr2 as isize) as i32 - 48i32);
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 - buf_ptr1 != 0i32
}
unsafe extern "C" fn scan_integer() -> bool {
    let mut sign_length: u8 = 0;
    buf_ptr1 = buf_ptr2;
    if *buffer.offset(buf_ptr2 as isize) as i32 == 45i32 {
        /*minus_sign */
        sign_length = 1_u8;
        buf_ptr2 = buf_ptr2 + 1i32
    } else {
        sign_length = 0_u8
    }
    token_value = 0i32;
    while buf_ptr2 < last && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 3i32 {
        token_value = token_value * 10i32 + (*buffer.offset(buf_ptr2 as isize) as i32 - 48i32);
        buf_ptr2 = buf_ptr2 + 1i32
    }
    if sign_length as i32 == 1i32 {
        token_value = -token_value
    }
    buf_ptr2 - buf_ptr1 != sign_length as i32
}
unsafe extern "C" fn scan_white_space() -> bool {
    while buf_ptr2 < last && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32 {
        buf_ptr2 = buf_ptr2 + 1i32
    }
    buf_ptr2 < last
}
unsafe extern "C" fn eat_bst_white_space() -> bool {
    loop {
        if scan_white_space() {
            if *buffer.offset(buf_ptr2 as isize) as i32 != 37i32 {
                /*comment */
                return true;
            }
        }
        if !input_ln(bst_file) {
            return false;
        }
        bst_line_num = bst_line_num + 1i32;
        buf_ptr2 = 0i32
    }
}
unsafe extern "C" fn skip_token_print() {
    putc_log('-' as i32);
    bst_ln_num_print();
    mark_error();
    scan2_white(125i32 as ASCII_code, 37i32 as ASCII_code);
}
unsafe extern "C" fn print_recursion_illegal() {
    puts_log(b"Curse you, wizard, before you recurse me:\n\x00" as *const u8 as *const i8);
    puts_log(b"function \x00" as *const u8 as *const i8);
    print_a_token();
    puts_log(b" is illegal in its own definition\n\x00" as *const u8 as *const i8);
    skip_token_print();
}
unsafe extern "C" fn skp_token_unknown_function_print() {
    print_a_token();
    puts_log(b" is an unknown function\x00" as *const u8 as *const i8);
    skip_token_print();
}
unsafe extern "C" fn skip_illegal_stuff_after_token_print() {
    printf_log(
        b"\"%c\" can\'t follow a literal\x00" as *const u8 as *const i8,
        *buffer.offset(buf_ptr2 as isize) as i32,
    );
    skip_token_print();
}
unsafe extern "C" fn scan_fn_def(mut fn_hash_loc: hash_loc) {
    let mut current_block: u64;
    let mut singl_function: *mut hash_ptr2 = 0 as *mut hash_ptr2;
    let mut single_fn_space: i32 = 0;
    let mut single_ptr: fn_def_loc = 0;
    let mut copy_ptr: fn_def_loc = 0;
    let mut end_of_num: buf_pointer = 0;
    let mut impl_fn_loc: hash_loc = 0;
    single_fn_space = 100i32;
    singl_function = xmalloc(
        ((single_fn_space + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
    ) as *mut hash_ptr2;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
    } else {
        single_ptr = 0i32;
        loop {
            if !(*buffer.offset(buf_ptr2 as isize) as i32 != 125i32) {
                current_block = 355541881813056170;
                break;
            }
            /*right_brace */
            match *buffer.offset(buf_ptr2 as isize) as i32 {
                35 => {
                    buf_ptr2 = buf_ptr2 + 1i32; /*int_literal */
                    if !scan_integer() {
                        puts_log(
                            b"Illegal integer in integer literal\x00" as *const u8 as *const i8,
                        ); /*str_literal */
                        skip_token_print(); /*194: */
                    } else {
                        literal_loc = str_lookup(
                            buffer,
                            buf_ptr1,
                            buf_ptr2 - buf_ptr1,
                            1i32 as str_ilk,
                            true,
                        ); /*single_quote */
                        if !hash_found {
                            *fn_type.offset(literal_loc as isize) = 2i32 as fn_class; /*wiz_defined */
                            *ilk_info.offset(literal_loc as isize) = token_value
                        }
                        if buf_ptr2 < last
                            && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 != 1i32
                            && *buffer.offset(buf_ptr2 as isize) as i32 != 125i32
                            && *buffer.offset(buf_ptr2 as isize) as i32 != 37i32
                        {
                            skip_illegal_stuff_after_token_print();
                        } else {
                            *singl_function.offset(single_ptr as isize) = literal_loc;
                            if single_ptr == single_fn_space {
                                singl_function = xrealloc(
                                    singl_function as *mut libc::c_void,
                                    ((single_fn_space + 100i32 + 1i32) as u64)
                                        .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                                )
                                    as *mut hash_ptr2;
                                single_fn_space = single_fn_space + 100i32
                            }
                            single_ptr = single_ptr + 1i32
                        }
                    }
                }
                34 => {
                    buf_ptr2 = buf_ptr2 + 1i32;
                    if !scan1(34i32 as ASCII_code) {
                        printf_log(b"No `\"\' to end string literal\x00" as *const u8 as *const i8);
                        skip_token_print();
                    } else {
                        literal_loc = str_lookup(
                            buffer,
                            buf_ptr1,
                            buf_ptr2 - buf_ptr1,
                            0i32 as str_ilk,
                            true,
                        );
                        *fn_type.offset(literal_loc as isize) = 3i32 as fn_class;
                        buf_ptr2 = buf_ptr2 + 1i32;
                        if buf_ptr2 < last
                            && lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 != 1i32
                            && *buffer.offset(buf_ptr2 as isize) as i32 != 125i32
                            && *buffer.offset(buf_ptr2 as isize) as i32 != 37i32
                        {
                            skip_illegal_stuff_after_token_print();
                        } else {
                            *singl_function.offset(single_ptr as isize) = literal_loc;
                            if single_ptr == single_fn_space {
                                singl_function = xrealloc(
                                    singl_function as *mut libc::c_void,
                                    ((single_fn_space + 100i32 + 1i32) as u64)
                                        .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                                )
                                    as *mut hash_ptr2;
                                single_fn_space = single_fn_space + 100i32
                            }
                            single_ptr = single_ptr + 1i32
                        }
                    }
                }
                39 => {
                    buf_ptr2 = buf_ptr2 + 1i32;
                    scan2_white(125i32 as ASCII_code, 37i32 as ASCII_code);
                    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                    fn_loc = str_lookup(
                        buffer,
                        buf_ptr1,
                        buf_ptr2 - buf_ptr1,
                        11i32 as str_ilk,
                        false,
                    );
                    if !hash_found {
                        skp_token_unknown_function_print();
                    } else if fn_loc == wiz_loc {
                        print_recursion_illegal();
                    } else {
                        *singl_function.offset(single_ptr as isize) = 1i32 - 1i32;
                        if single_ptr == single_fn_space {
                            singl_function = xrealloc(
                                singl_function as *mut libc::c_void,
                                ((single_fn_space + 100i32 + 1i32) as u64)
                                    .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                            ) as *mut hash_ptr2;
                            single_fn_space = single_fn_space + 100i32
                        }
                        single_ptr = single_ptr + 1i32;
                        *singl_function.offset(single_ptr as isize) = fn_loc;
                        if single_ptr == single_fn_space {
                            singl_function = xrealloc(
                                singl_function as *mut libc::c_void,
                                ((single_fn_space + 100i32 + 1i32) as u64)
                                    .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                            ) as *mut hash_ptr2;
                            single_fn_space = single_fn_space + 100i32
                        }
                        single_ptr = single_ptr + 1i32
                    }
                }
                123 => {
                    *ex_buf.offset(0) = 39i32 as ASCII_code;
                    int_to_ASCII(impl_fn_num, ex_buf, 1i32, &mut end_of_num);
                    impl_fn_loc = str_lookup(ex_buf, 0i32, end_of_num, 11i32 as str_ilk, true);
                    if hash_found {
                        puts_log(
                            b"Already encountered implicit function\x00" as *const u8 as *const i8,
                        );
                        print_confusion();
                        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                    }
                    impl_fn_num = impl_fn_num + 1i32;
                    *fn_type.offset(impl_fn_loc as isize) = 1i32 as fn_class;
                    *singl_function.offset(single_ptr as isize) = 1i32 - 1i32;
                    if single_ptr == single_fn_space {
                        singl_function = xrealloc(
                            singl_function as *mut libc::c_void,
                            ((single_fn_space + 100i32 + 1i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                        ) as *mut hash_ptr2;
                        single_fn_space = single_fn_space + 100i32
                    }
                    single_ptr = single_ptr + 1i32;
                    *singl_function.offset(single_ptr as isize) = impl_fn_loc;
                    if single_ptr == single_fn_space {
                        singl_function = xrealloc(
                            singl_function as *mut libc::c_void,
                            ((single_fn_space + 100i32 + 1i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                        ) as *mut hash_ptr2;
                        single_fn_space = single_fn_space + 100i32
                    }
                    single_ptr = single_ptr + 1i32;
                    buf_ptr2 = buf_ptr2 + 1i32;
                    scan_fn_def(impl_fn_loc);
                }
                _ => {
                    scan2_white(125i32 as ASCII_code, 37i32 as ASCII_code);
                    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                    fn_loc = str_lookup(
                        buffer,
                        buf_ptr1,
                        buf_ptr2 - buf_ptr1,
                        11i32 as str_ilk,
                        false,
                    );
                    if !hash_found {
                        skp_token_unknown_function_print();
                    } else if fn_loc == wiz_loc {
                        print_recursion_illegal();
                    } else {
                        *singl_function.offset(single_ptr as isize) = fn_loc;
                        if single_ptr == single_fn_space {
                            singl_function = xrealloc(
                                singl_function as *mut libc::c_void,
                                ((single_fn_space + 100i32 + 1i32) as u64)
                                    .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                            ) as *mut hash_ptr2;
                            single_fn_space = single_fn_space + 100i32
                        }
                        single_ptr = single_ptr + 1i32
                    }
                }
            }
            /*next_token */
            if eat_bst_white_space() {
                continue; /*space */
            }
            eat_bst_print();
            puts_log(b"function\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            current_block = 623752384954289075;
            break;
        }
        match current_block {
            623752384954289075 => {}
            _ => {
                *singl_function.offset(single_ptr as isize) = end_of_def;
                if single_ptr == single_fn_space {
                    singl_function = xrealloc(
                        singl_function as *mut libc::c_void,
                        ((single_fn_space + 100i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                    ) as *mut hash_ptr2;
                    single_fn_space = single_fn_space + 100i32
                }
                single_ptr = single_ptr + 1i32;
                while single_ptr + wiz_def_ptr > wiz_fn_space {
                    wiz_functions = xrealloc(
                        wiz_functions as *mut libc::c_void,
                        ((wiz_fn_space + 3000i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
                    ) as *mut hash_ptr2;
                    wiz_fn_space = wiz_fn_space + 3000i32
                }
                *ilk_info.offset(fn_hash_loc as isize) = wiz_def_ptr;
                copy_ptr = 0i32;
                while copy_ptr < single_ptr {
                    *wiz_functions.offset(wiz_def_ptr as isize) =
                        *singl_function.offset(copy_ptr as isize);
                    copy_ptr = copy_ptr + 1i32;
                    wiz_def_ptr = wiz_def_ptr + 1i32
                }
                buf_ptr2 = buf_ptr2 + 1i32
            }
        }
    }
    free(singl_function as *mut libc::c_void);
}
unsafe extern "C" fn eat_bib_white_space() -> bool {
    while !scan_white_space() {
        if !input_ln(*bib_file.offset(bib_ptr as isize)) {
            return false;
        }
        bib_line_num = bib_line_num + 1i32;
        buf_ptr2 = 0i32
    }
    true
}
unsafe extern "C" fn compress_bib_white() -> bool {
    if ex_buf_ptr == buf_size {
        bib_field_too_long_print();
        return false;
    } else {
        *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as ASCII_code;
        ex_buf_ptr = ex_buf_ptr + 1i32
    }
    while !scan_white_space() {
        if !input_ln(*bib_file.offset(bib_ptr as isize)) {
            eat_bib_print();
            return false;
        }
        bib_line_num = bib_line_num + 1i32;
        buf_ptr2 = 0i32
    }
    true
}
unsafe extern "C" fn scan_balanced_braces() -> bool {
    buf_ptr2 = buf_ptr2 + 1i32;
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32 || buf_ptr2 == last {
        if !compress_bib_white() {
            return false;
        }
    }
    if ex_buf_ptr > 1i32 {
        if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 32i32 {
            /*space */
            if *ex_buf.offset((ex_buf_ptr - 2i32) as isize) as i32 == 32i32 {
                /*space */
                ex_buf_ptr = ex_buf_ptr - 1i32
            }
        }
    } /*255: */
    bib_brace_level = 0i32;
    if store_field {
        /*257: */
        while *buffer.offset(buf_ptr2 as isize) as i32 != right_str_delim as i32 {
            match *buffer.offset(buf_ptr2 as isize) as i32 {
                123 => {
                    bib_brace_level = bib_brace_level + 1i32; /*left_brace */
                    if ex_buf_ptr == buf_size {
                        bib_field_too_long_print(); /*right_brace */
                        return false;
                    } else {
                        *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as ASCII_code; /*left_brace */
                        ex_buf_ptr = ex_buf_ptr + 1i32
                    }
                    buf_ptr2 = buf_ptr2 + 1i32;
                    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32
                        || buf_ptr2 == last
                    {
                        if !compress_bib_white() {
                            return false;
                        }
                    }
                    loop {
                        match *buffer.offset(buf_ptr2 as isize) as i32 {
                            125 => {
                                bib_brace_level = bib_brace_level - 1i32;
                                if ex_buf_ptr == buf_size {
                                    bib_field_too_long_print();
                                    return false;
                                } else {
                                    *ex_buf.offset(ex_buf_ptr as isize) = 125i32 as ASCII_code;
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                }
                                buf_ptr2 = buf_ptr2 + 1i32;
                                if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32
                                    == 1i32
                                    || buf_ptr2 == last
                                {
                                    if !compress_bib_white() {
                                        return false;
                                    }
                                }
                                if bib_brace_level == 0i32 {
                                    break;
                                }
                            }
                            123 => {
                                bib_brace_level = bib_brace_level + 1i32;
                                if ex_buf_ptr == buf_size {
                                    bib_field_too_long_print();
                                    return false;
                                } else {
                                    *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as ASCII_code;
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                }
                                buf_ptr2 = buf_ptr2 + 1i32;
                                if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32
                                    == 1i32
                                    || buf_ptr2 == last
                                {
                                    if !compress_bib_white() {
                                        return false;
                                    }
                                }
                            }
                            _ => {
                                if ex_buf_ptr == buf_size {
                                    bib_field_too_long_print();
                                    return false;
                                } else {
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *buffer.offset(buf_ptr2 as isize);
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                }
                                buf_ptr2 = buf_ptr2 + 1i32;
                                if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32
                                    == 1i32
                                    || buf_ptr2 == last
                                {
                                    if !compress_bib_white() {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
                125 => {
                    bib_unbalanced_braces_print();
                    return false;
                }
                _ => {
                    if ex_buf_ptr == buf_size {
                        bib_field_too_long_print();
                        return false;
                    } else {
                        *ex_buf.offset(ex_buf_ptr as isize) = *buffer.offset(buf_ptr2 as isize);
                        ex_buf_ptr = ex_buf_ptr + 1i32
                    }
                    buf_ptr2 = buf_ptr2 + 1i32;
                    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32
                        || buf_ptr2 == last
                    {
                        if !compress_bib_white() {
                            return false;
                        }
                    }
                }
            }
        }
    } else {
        while *buffer.offset(buf_ptr2 as isize) as i32 != right_str_delim as i32 {
            if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                /*left_brace */
                bib_brace_level = bib_brace_level + 1i32;
                buf_ptr2 = buf_ptr2 + 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return false;
                }
                while bib_brace_level > 0i32 {
                    /*256: */
                    if *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
                        /*right_brace */
                        bib_brace_level = bib_brace_level - 1i32;
                        buf_ptr2 = buf_ptr2 + 1i32;
                        if !eat_bib_white_space() {
                            eat_bib_print();
                            return false;
                        }
                    } else if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                        /*left_brace */
                        bib_brace_level = bib_brace_level + 1i32;
                        buf_ptr2 = buf_ptr2 + 1i32;
                        if !eat_bib_white_space() {
                            eat_bib_print();
                            return false;
                        }
                    } else {
                        buf_ptr2 = buf_ptr2 + 1i32;
                        if !scan2(125i32 as ASCII_code, 123i32 as ASCII_code) {
                            if !eat_bib_white_space() {
                                eat_bib_print();
                                return false;
                            }
                        }
                    }
                }
            } else if *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
                /*right_brace */
                bib_unbalanced_braces_print(); /*right_brace */
                return false;
            } else {
                buf_ptr2 = buf_ptr2 + 1i32; /*double_quote */
                if !scan3(right_str_delim, 123i32 as ASCII_code, 125i32 as ASCII_code) {
                    if !eat_bib_white_space() {
                        eat_bib_print();
                        return false;
                    }
                }
            }
        }
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    true
}
unsafe extern "C" fn scan_a_field_token_and_eat_white() -> bool {
    match *buffer.offset(buf_ptr2 as isize) as i32 {
        123 => {
            right_str_delim = 125i32 as ASCII_code;
            if !scan_balanced_braces() {
                return false;
            }
        }
        34 => {
            right_str_delim = 34i32 as ASCII_code;
            if !scan_balanced_braces() {
                return false;
            }
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            if !scan_nonneg_integer() {
                puts_log(b"A digit disappeared\x00" as *const u8 as *const i8);
                print_confusion();
                longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
            }
            if store_field {
                tmp_ptr = buf_ptr1;
                while tmp_ptr < buf_ptr2 {
                    if ex_buf_ptr == buf_size {
                        bib_field_too_long_print();
                        return false;
                    } else {
                        *ex_buf.offset(ex_buf_ptr as isize) = *buffer.offset(tmp_ptr as isize);
                        ex_buf_ptr = ex_buf_ptr + 1i32
                    }
                    tmp_ptr = tmp_ptr + 1i32
                }
            }
        }
        _ => {
            scan_identifier(44i32 as ASCII_code, right_outer_delim, 35i32 as ASCII_code);
            if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
            } else {
                bib_id_print();
                puts_log(b"a field part\x00" as *const u8 as *const i8);
                bib_err_print();
                return false;
            }
            if store_field {
                lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                macro_name_loc = str_lookup(
                    buffer,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    13i32 as str_ilk,
                    false,
                );
                store_token = true;
                if at_bib_command {
                    if command_num == 2i32 {
                        /*n_bib_string */
                        if macro_name_loc == cur_macro_loc {
                            store_token = false;
                            macro_warn_print();
                            puts_log(b"used in its own definition\n\x00" as *const u8 as *const i8);
                            bib_warn_print();
                        }
                    }
                }
                if !hash_found {
                    store_token = false;
                    macro_warn_print();
                    puts_log(b"undefined\n\x00" as *const u8 as *const i8);
                    bib_warn_print();
                }
                if store_token {
                    /*261: */
                    tmp_ptr = *str_start.offset(*ilk_info.offset(macro_name_loc as isize) as isize); /*space */
                    tmp_end_ptr = *str_start
                        .offset((*ilk_info.offset(macro_name_loc as isize) + 1i32) as isize);
                    if ex_buf_ptr == 0i32 {
                        if tmp_ptr < tmp_end_ptr
                            && lex_class[*str_pool.offset(tmp_ptr as isize) as usize] as i32 == 1i32
                        {
                            if ex_buf_ptr == buf_size {
                                bib_field_too_long_print();
                                return false;
                            } else {
                                *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as ASCII_code;
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            }
                            tmp_ptr = tmp_ptr + 1i32;
                            while tmp_ptr < tmp_end_ptr
                                && lex_class[*str_pool.offset(tmp_ptr as isize) as usize] as i32
                                    == 1i32
                            {
                                tmp_ptr = tmp_ptr + 1i32
                            }
                        }
                    }
                    while tmp_ptr < tmp_end_ptr {
                        if lex_class[*str_pool.offset(tmp_ptr as isize) as usize] as i32 != 1i32 {
                            /*white_space */
                            if ex_buf_ptr == buf_size {
                                bib_field_too_long_print();
                                return false;
                            } else {
                                *ex_buf.offset(ex_buf_ptr as isize) =
                                    *str_pool.offset(tmp_ptr as isize);
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            }
                        } else if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 != 32i32 {
                            /*space */
                            if ex_buf_ptr == buf_size {
                                bib_field_too_long_print(); /*space */
                                return false;
                            } else {
                                *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as ASCII_code;
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            }
                        }
                        tmp_ptr = tmp_ptr + 1i32
                    }
                }
            }
        }
    }
    if !eat_bib_white_space() {
        eat_bib_print();
        return false;
    }
    true
}
unsafe extern "C" fn scan_and_store_the_field_value_and_eat_white() -> bool {
    ex_buf_ptr = 0i32;
    if !scan_a_field_token_and_eat_white() {
        return false;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 == 35i32 {
        /*concat_char */
        buf_ptr2 = buf_ptr2 + 1i32;
        if !eat_bib_white_space() {
            eat_bib_print();
            return false;
        }
        if !scan_a_field_token_and_eat_white() {
            return false;
        }
    }
    if store_field {
        /*262: */
        if !at_bib_command {
            if ex_buf_ptr > 0i32 {
                if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 32i32 {
                    /*space */
                    ex_buf_ptr = ex_buf_ptr - 1i32
                }
            }
        } /*str_literal */
        if !at_bib_command && *ex_buf.offset(0) as i32 == 32i32 && ex_buf_ptr > 0i32 {
            ex_buf_xptr = 1i32
        } else {
            ex_buf_xptr = 0i32
        } /*264: */
        field_val_loc = str_lookup(
            ex_buf,
            ex_buf_xptr,
            ex_buf_ptr - ex_buf_xptr,
            0i32 as str_ilk,
            true,
        );
        *fn_type.offset(field_val_loc as isize) = 3i32 as fn_class;
        if at_bib_command {
            /*263: */
            match command_num {
                1 => {
                    *s_preamble.offset(preamble_ptr as isize) =
                        *hash_text.offset(field_val_loc as isize);
                    preamble_ptr = preamble_ptr + 1i32
                }
                2 => {
                    *ilk_info.offset(cur_macro_loc as isize) =
                        *hash_text.offset(field_val_loc as isize)
                }
                _ => {
                    bib_cmd_confusion();
                }
            }
        } else {
            field_ptr = entry_cite_ptr * num_fields + *ilk_info.offset(field_name_loc as isize);
            if field_ptr >= max_fields {
                puts_log(b"field_info index is out of range\x00" as *const u8 as *const i8);
                print_confusion();
                longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
            }
            if *field_info.offset(field_ptr as isize) != 0i32 {
                /*missing */
                puts_log(b"Warning--I\'m ignoring \x00" as *const u8 as *const i8);
                print_a_pool_str(*cite_list.offset(entry_cite_ptr as isize));
                puts_log(b"\'s extra \"\x00" as *const u8 as *const i8);
                print_a_pool_str(*hash_text.offset(field_name_loc as isize));
                puts_log(b"\" field\n\x00" as *const u8 as *const i8);
                bib_warn_print();
            } else {
                *field_info.offset(field_ptr as isize) = *hash_text.offset(field_val_loc as isize);
                if *ilk_info.offset(field_name_loc as isize) == crossref_num && !all_entries {
                    /*265: */
                    tmp_ptr = ex_buf_xptr;
                    while tmp_ptr < ex_buf_ptr {
                        *out_buf.offset(tmp_ptr as isize) = *ex_buf.offset(tmp_ptr as isize);
                        tmp_ptr = tmp_ptr + 1i32
                    }
                    lower_case(out_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                    lc_cite_loc = str_lookup(
                        out_buf,
                        ex_buf_xptr,
                        ex_buf_ptr - ex_buf_xptr,
                        10i32 as str_ilk,
                        true,
                    );
                    if hash_found {
                        cite_loc = *ilk_info.offset(lc_cite_loc as isize);
                        if *ilk_info.offset(cite_loc as isize) >= old_num_cites {
                            *cite_info.offset(*ilk_info.offset(cite_loc as isize) as isize) =
                                *cite_info.offset(*ilk_info.offset(cite_loc as isize) as isize)
                                    + 1i32
                        }
                    } else {
                        cite_loc = str_lookup(
                            ex_buf,
                            ex_buf_xptr,
                            ex_buf_ptr - ex_buf_xptr,
                            9i32 as str_ilk,
                            true,
                        );
                        if hash_found {
                            hash_cite_confusion();
                        }
                        add_database_cite(&mut cite_ptr);
                        *cite_info.offset(*ilk_info.offset(cite_loc as isize) as isize) = 1i32
                    }
                }
            }
        }
    }
    true
}
unsafe extern "C" fn decr_brace_level(mut pop_lit_var: str_number) {
    if brace_level == 0i32 {
        braces_unbalanced_complaint(pop_lit_var);
    } else {
        brace_level = brace_level - 1i32
    };
}
unsafe extern "C" fn check_brace_level(mut pop_lit_var: str_number) {
    if brace_level > 0i32 {
        braces_unbalanced_complaint(pop_lit_var);
    };
}
unsafe extern "C" fn name_scan_for_and(mut pop_lit_var: str_number) {
    brace_level = 0i32;
    preceding_white = false;
    and_found = false;
    while !and_found && ex_buf_ptr < ex_buf_length {
        match *ex_buf.offset(ex_buf_ptr as isize) as i32 {
            97 | 65 => {
                ex_buf_ptr = ex_buf_ptr + 1i32;
                if preceding_white {
                    /*387: */
                    if ex_buf_ptr <= ex_buf_length - 3i32 {
                        if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 'n' as i32
                            || *ex_buf.offset(ex_buf_ptr as isize) as i32 == 'N' as i32
                        {
                            if *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 'd' as i32
                                || *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 'D' as i32
                            {
                                if lex_class[*ex_buf.offset((ex_buf_ptr + 2i32) as isize) as usize]
                                    as i32
                                    == 1i32
                                {
                                    /*white_space */
                                    ex_buf_ptr = ex_buf_ptr + 2i32;
                                    and_found = true
                                }
                            }
                        }
                    }
                }
                preceding_white = false
            }
            123 => {
                brace_level = brace_level + 1i32;
                ex_buf_ptr = ex_buf_ptr + 1i32;
                while brace_level > 0i32 && ex_buf_ptr < ex_buf_length {
                    if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        brace_level = brace_level - 1i32
                    } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        brace_level = brace_level + 1i32
                    }
                    ex_buf_ptr = ex_buf_ptr + 1i32
                }
                preceding_white = false
            }
            125 => {
                decr_brace_level(pop_lit_var);
                ex_buf_ptr = ex_buf_ptr + 1i32;
                preceding_white = false
            }
            _ => {
                if lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] as i32 == 1i32 {
                    /*white_space */
                    ex_buf_ptr = ex_buf_ptr + 1i32;
                    preceding_white = true
                } else {
                    ex_buf_ptr = ex_buf_ptr + 1i32;
                    preceding_white = false
                }
            }
        }
    }
    check_brace_level(pop_lit_var);
}
unsafe extern "C" fn von_token_found() -> bool {
    nm_brace_level = 0i32;
    while name_bf_ptr < name_bf_xptr {
        if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'A' as i32
            && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'Z' as i32
        {
            return false;
        } else {
            if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'a' as i32
                && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'z' as i32
            {
                return true;
            } else {
                if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32 {
                    /*left_brace */
                    nm_brace_level = nm_brace_level + 1i32; /*401: */
                    name_bf_ptr = name_bf_ptr + 1i32;
                    if name_bf_ptr + 2i32 < name_bf_xptr
                        && *sv_buffer.offset(name_bf_ptr as isize) as i32 == 92i32
                    {
                        /*399: */
                        name_bf_ptr = name_bf_ptr + 1i32;
                        name_bf_yptr = name_bf_ptr;
                        while name_bf_ptr < name_bf_xptr
                            && lex_class[*sv_buffer.offset(name_bf_ptr as isize) as usize] as i32
                                == 2i32
                        {
                            name_bf_ptr = name_bf_ptr + 1i32
                        }
                        control_seq_loc = str_lookup(
                            sv_buffer,
                            name_bf_yptr,
                            name_bf_ptr - name_bf_yptr,
                            14i32 as str_ilk,
                            false,
                        );
                        if hash_found {
                            /*400: */
                            match *ilk_info.offset(control_seq_loc as isize) {
                                3 | 5 | 7 | 9 | 11 => return false,
                                0 | 1 | 2 | 4 | 6 | 8 | 10 | 12 => return true,
                                _ => {
                                    puts_log(
                                        b"Control-sequence hash error\x00" as *const u8
                                            as *const i8,
                                    );
                                    print_confusion();
                                    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                                }
                            }
                        }
                        while name_bf_ptr < name_bf_xptr && nm_brace_level > 0i32 {
                            if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'A' as i32
                                && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'Z' as i32
                            {
                                return false;
                            } else {
                                if *sv_buffer.offset(name_bf_ptr as isize) as i32 >= 'a' as i32
                                    && *sv_buffer.offset(name_bf_ptr as isize) as i32 <= 'z' as i32
                                {
                                    return true;
                                } else {
                                    if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 125i32 {
                                        /*right_brace */
                                        nm_brace_level = nm_brace_level - 1i32
                                    } else if *sv_buffer.offset(name_bf_ptr as isize) as i32
                                        == 123i32
                                    {
                                        /*left_brace */
                                        nm_brace_level = nm_brace_level + 1i32
                                    }
                                }
                            }
                            name_bf_ptr = name_bf_ptr + 1i32
                        }
                        return false;
                    } else {
                        while nm_brace_level > 0i32 && name_bf_ptr < name_bf_xptr {
                            if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 125i32 {
                                /*right_brace */
                                nm_brace_level = nm_brace_level - 1i32
                            } else if *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32 {
                                /*left_brace */
                                nm_brace_level = nm_brace_level + 1i32
                            }
                            name_bf_ptr = name_bf_ptr + 1i32
                        }
                    }
                } else {
                    name_bf_ptr = name_bf_ptr + 1i32
                }
            }
        }
    }
    false
}
unsafe extern "C" fn von_name_ends_and_last_name_starts_stuff() {
    von_end = last_end - 1i32;
    while von_end > von_start {
        name_bf_ptr = *name_tok.offset((von_end - 1i32) as isize);
        name_bf_xptr = *name_tok.offset(von_end as isize);
        if von_token_found() {
            return;
        }
        von_end = von_end - 1i32
    }
}
unsafe extern "C" fn skip_stuff_at_sp_brace_level_greater_than_one() {
    while sp_brace_level > 1i32 && sp_ptr < sp_end {
        if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
            /*right_brace */
            sp_brace_level = sp_brace_level - 1i32
        } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
            /*left_brace */
            sp_brace_level = sp_brace_level + 1i32
        }
        sp_ptr = sp_ptr + 1i32
    }
}
unsafe extern "C" fn brace_lvl_one_letters_complaint() {
    puts_log(b"The format string \"\x00" as *const u8 as *const i8);
    print_a_pool_str(pop_lit1);
    puts_log(b"\" has an illegal brace-level-1 letter\x00" as *const u8 as *const i8);
    bst_ex_warn_print();
}
unsafe extern "C" fn enough_text_chars(mut enough_chars: buf_pointer) -> bool {
    num_text_chars = 0i32;
    ex_buf_yptr = ex_buf_xptr;
    while ex_buf_yptr < ex_buf_ptr && num_text_chars < enough_chars {
        ex_buf_yptr = ex_buf_yptr + 1i32;
        if *ex_buf.offset((ex_buf_yptr - 1i32) as isize) as i32 == 123i32 {
            /*left_brace */
            brace_level = brace_level + 1i32;
            if brace_level == 1i32 && ex_buf_yptr < ex_buf_ptr {
                if *ex_buf.offset(ex_buf_yptr as isize) as i32 == 92i32 {
                    /*backslash */
                    ex_buf_yptr = ex_buf_yptr + 1i32;
                    while ex_buf_yptr < ex_buf_ptr && brace_level > 0i32 {
                        if *ex_buf.offset(ex_buf_yptr as isize) as i32 == 125i32 {
                            /*right_brace */
                            brace_level = brace_level - 1i32
                        } else if *ex_buf.offset(ex_buf_yptr as isize) as i32 == 123i32 {
                            /*left_brace */
                            brace_level = brace_level + 1i32
                        }
                        ex_buf_yptr = ex_buf_yptr + 1i32
                    }
                }
            }
        } else if *ex_buf.offset((ex_buf_yptr - 1i32) as isize) as i32 == 125i32 {
            /*right_brace */
            brace_level = brace_level - 1i32
        }
        num_text_chars = num_text_chars + 1i32
    }
    num_text_chars >= enough_chars
}
unsafe extern "C" fn figure_out_the_formatted_name() {
    ex_buf_ptr = 0i32;
    sp_brace_level = 0i32;
    sp_ptr = *str_start.offset(pop_lit1 as isize);
    sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
    while sp_ptr < sp_end {
        if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
            /*left_brace */
            sp_brace_level = sp_brace_level + 1i32;
            sp_ptr = sp_ptr + 1i32;
            sp_xptr1 = sp_ptr;
            alpha_found = false;
            double_letter = false;
            end_of_group = false;
            to_be_written = true;
            while !end_of_group && sp_ptr < sp_end {
                if lex_class[*str_pool.offset(sp_ptr as isize) as usize] as i32 == 2i32 {
                    /*alpha */
                    sp_ptr = sp_ptr + 1i32;
                    if alpha_found {
                        brace_lvl_one_letters_complaint();
                        to_be_written = false
                    } else {
                        match *str_pool.offset((sp_ptr - 1i32) as isize) as i32 {
                            102 | 70 => {
                                cur_token = first_start;
                                last_token = first_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'f' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'F' as i32
                                {
                                    double_letter = true
                                }
                            }
                            118 | 86 => {
                                cur_token = von_start;
                                last_token = von_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'v' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'V' as i32
                                {
                                    double_letter = true
                                }
                            }
                            108 | 76 => {
                                cur_token = von_end;
                                last_token = last_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'l' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'L' as i32
                                {
                                    double_letter = true
                                }
                            }
                            106 | 74 => {
                                cur_token = last_end;
                                last_token = jr_end;
                                if cur_token == last_token {
                                    to_be_written = false
                                }
                                if *str_pool.offset(sp_ptr as isize) as i32 == 'j' as i32
                                    || *str_pool.offset(sp_ptr as isize) as i32 == 'J' as i32
                                {
                                    double_letter = true
                                }
                            }
                            _ => {
                                brace_lvl_one_letters_complaint();
                                to_be_written = false
                            }
                        }
                        if double_letter {
                            sp_ptr = sp_ptr + 1i32
                        }
                    }
                    alpha_found = true
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
                    /*right_brace */
                    sp_brace_level = sp_brace_level - 1i32;
                    sp_ptr = sp_ptr + 1i32;
                    end_of_group = true
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                    /*left_brace */
                    sp_brace_level = sp_brace_level + 1i32;
                    sp_ptr = sp_ptr + 1i32;
                    skip_stuff_at_sp_brace_level_greater_than_one();
                } else {
                    sp_ptr = sp_ptr + 1i32
                }
            }
            if !(end_of_group as i32 != 0 && to_be_written as i32 != 0) {
                continue;
            }
            /*412: */
            ex_buf_xptr = ex_buf_ptr;
            sp_ptr = sp_xptr1;
            sp_brace_level = 1i32;
            while sp_brace_level > 0i32 {
                if lex_class[*str_pool.offset(sp_ptr as isize) as usize] as i32 == 2i32
                    && sp_brace_level == 1i32
                {
                    sp_ptr = sp_ptr + 1i32;
                    if double_letter {
                        sp_ptr = sp_ptr + 1i32
                    }
                    use_default = true;
                    sp_xptr2 = sp_ptr;
                    if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        use_default = false; /*416: */
                        sp_brace_level = sp_brace_level + 1i32;
                        sp_ptr = sp_ptr + 1i32;
                        sp_xptr1 = sp_ptr;
                        skip_stuff_at_sp_brace_level_greater_than_one();
                        sp_xptr2 = sp_ptr - 1i32
                    }
                    while cur_token < last_token {
                        if double_letter {
                            /*415: */
                            name_bf_ptr = *name_tok.offset(cur_token as isize);
                            name_bf_xptr = *name_tok.offset((cur_token + 1i32) as isize);
                            if ex_buf_length + (name_bf_xptr - name_bf_ptr) > buf_size {
                                buffer_overflow();
                            }
                            while name_bf_ptr < name_bf_xptr {
                                *ex_buf.offset(ex_buf_ptr as isize) =
                                    *sv_buffer.offset(name_bf_ptr as isize);
                                ex_buf_ptr = ex_buf_ptr + 1i32;
                                name_bf_ptr = name_bf_ptr + 1i32
                            }
                        } else {
                            name_bf_ptr = *name_tok.offset(cur_token as isize);
                            name_bf_xptr = *name_tok.offset((cur_token + 1i32) as isize);
                            while name_bf_ptr < name_bf_xptr {
                                if lex_class[*sv_buffer.offset(name_bf_ptr as isize) as usize]
                                    as i32
                                    == 2i32
                                {
                                    /*alpha */
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow();
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *sv_buffer.offset(name_bf_ptr as isize);
                                    ex_buf_ptr = ex_buf_ptr + 1i32;
                                    break;
                                } else {
                                    if name_bf_ptr + 1i32 < name_bf_xptr
                                        && *sv_buffer.offset(name_bf_ptr as isize) as i32 == 123i32
                                    {
                                        if *sv_buffer.offset((name_bf_ptr + 1i32) as isize) as i32
                                            == 92i32
                                        {
                                            /*backslash */
                                            /*417: */
                                            if ex_buf_ptr + 2i32 > buf_size {
                                                buffer_overflow(); /*left_brace */
                                            } /*backslash */
                                            *ex_buf.offset(ex_buf_ptr as isize) =
                                                123i32 as ASCII_code;
                                            ex_buf_ptr = ex_buf_ptr + 1i32;
                                            *ex_buf.offset(ex_buf_ptr as isize) =
                                                92i32 as ASCII_code;
                                            ex_buf_ptr = ex_buf_ptr + 1i32;
                                            name_bf_ptr = name_bf_ptr + 2i32;
                                            nm_brace_level = 1i32;
                                            while name_bf_ptr < name_bf_xptr
                                                && nm_brace_level > 0i32
                                            {
                                                if *sv_buffer.offset(name_bf_ptr as isize) as i32
                                                    == 125i32
                                                {
                                                    /*right_brace */
                                                    nm_brace_level = nm_brace_level - 1i32
                                                } else if *sv_buffer.offset(name_bf_ptr as isize)
                                                    as i32
                                                    == 123i32
                                                {
                                                    /*left_brace */
                                                    nm_brace_level = nm_brace_level + 1i32
                                                }
                                                if ex_buf_ptr == buf_size {
                                                    buffer_overflow();
                                                }
                                                *ex_buf.offset(ex_buf_ptr as isize) =
                                                    *sv_buffer.offset(name_bf_ptr as isize);
                                                ex_buf_ptr = ex_buf_ptr + 1i32;
                                                name_bf_ptr = name_bf_ptr + 1i32
                                            }
                                            break;
                                        }
                                    }
                                    name_bf_ptr = name_bf_ptr + 1i32
                                }
                            }
                        }
                        cur_token = cur_token + 1i32;
                        if cur_token < last_token {
                            /*418: */
                            if use_default {
                                if !double_letter {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow(); /*period */
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) = 46i32 as ASCII_code;
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                }
                                if lex_class[*name_sep_char.offset(cur_token as isize) as usize]
                                    as i32
                                    == 4i32
                                {
                                    /*sep_char */
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow(); /*tie */
                                    } /*space */
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *name_sep_char.offset(cur_token as isize);
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                } else if cur_token == last_token - 1i32 || !enough_text_chars(3i32)
                                {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow();
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) = 126i32 as ASCII_code;
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                } else {
                                    if ex_buf_ptr == buf_size {
                                        buffer_overflow();
                                    }
                                    *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as ASCII_code;
                                    ex_buf_ptr = ex_buf_ptr + 1i32
                                }
                            } else {
                                if ex_buf_length + (sp_xptr2 - sp_xptr1) > buf_size {
                                    buffer_overflow();
                                }
                                sp_ptr = sp_xptr1;
                                while sp_ptr < sp_xptr2 {
                                    *ex_buf.offset(ex_buf_ptr as isize) =
                                        *str_pool.offset(sp_ptr as isize);
                                    ex_buf_ptr = ex_buf_ptr + 1i32;
                                    sp_ptr = sp_ptr + 1i32
                                }
                            }
                        }
                    }
                    if !use_default {
                        sp_ptr = sp_xptr2 + 1i32
                    }
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
                    /*right_brace */
                    sp_brace_level = sp_brace_level - 1i32; /*right_brace */
                    sp_ptr = sp_ptr + 1i32;
                    if sp_brace_level > 0i32 {
                        if ex_buf_ptr == buf_size {
                            buffer_overflow();
                        }
                        *ex_buf.offset(ex_buf_ptr as isize) = 125i32 as ASCII_code;
                        ex_buf_ptr = ex_buf_ptr + 1i32
                    }
                } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                    /*left_brace */
                    sp_brace_level = sp_brace_level + 1i32; /*left_brace */
                    sp_ptr = sp_ptr + 1i32;
                    if ex_buf_ptr == buf_size {
                        buffer_overflow();
                    }
                    *ex_buf.offset(ex_buf_ptr as isize) = 123i32 as ASCII_code;
                    ex_buf_ptr = ex_buf_ptr + 1i32
                } else {
                    if ex_buf_ptr == buf_size {
                        buffer_overflow();
                    }
                    *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                    ex_buf_ptr = ex_buf_ptr + 1i32;
                    sp_ptr = sp_ptr + 1i32
                }
            }
            if ex_buf_ptr > 0i32 {
                if *ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 126i32 {
                    /*tie */
                    /*420: */
                    ex_buf_ptr = ex_buf_ptr - 1i32; /*space */
                    if !(*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 126i32) {
                        if !enough_text_chars(3i32) {
                            ex_buf_ptr = ex_buf_ptr + 1i32
                        } else {
                            *ex_buf.offset(ex_buf_ptr as isize) = 32i32 as ASCII_code;
                            ex_buf_ptr = ex_buf_ptr + 1i32
                        }
                    }
                }
            }
        } else if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
            /*right_brace */
            braces_unbalanced_complaint(pop_lit1);
            sp_ptr = sp_ptr + 1i32
        } else {
            if ex_buf_ptr == buf_size {
                buffer_overflow();
            }
            *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            ex_buf_ptr = ex_buf_ptr + 1i32;
            sp_ptr = sp_ptr + 1i32
        }
    }
    if sp_brace_level > 0i32 {
        braces_unbalanced_complaint(pop_lit1);
    }
    ex_buf_length = ex_buf_ptr;
}
unsafe extern "C" fn push_lit_stk(mut push_lt: i32, mut push_type: stk_type) {
    *lit_stack.offset(lit_stk_ptr as isize) = push_lt;
    *lit_stk_type.offset(lit_stk_ptr as isize) = push_type;
    if lit_stk_ptr == lit_stk_size {
        lit_stack = xrealloc(
            lit_stack as *mut libc::c_void,
            ((lit_stk_size + 100i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<i32>() as u64),
        ) as *mut i32;
        lit_stk_type = xrealloc(
            lit_stk_type as *mut libc::c_void,
            ((lit_stk_size + 100i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<stk_type>() as u64),
        ) as *mut stk_type;
        lit_stk_size = lit_stk_size + 100i32
    }
    lit_stk_ptr = lit_stk_ptr + 1i32;
}
unsafe extern "C" fn pop_lit_stk(mut pop_lit: *mut i32, mut pop_type: *mut stk_type) {
    if lit_stk_ptr == 0i32 {
        puts_log(b"You can\'t pop an empty literal stack\x00" as *const u8 as *const i8);
        bst_ex_warn_print();
        *pop_type = 4i32 as stk_type
    /*stk_empty */
    } else {
        lit_stk_ptr = lit_stk_ptr - 1i32;
        *pop_lit = *lit_stack.offset(lit_stk_ptr as isize);
        *pop_type = *lit_stk_type.offset(lit_stk_ptr as isize);
        if *pop_type as i32 == 1i32 {
            /*stk_str */
            if *pop_lit >= cmd_str_ptr {
                if *pop_lit != str_ptr - 1i32 {
                    puts_log(b"Nontop top of string stack\x00" as *const u8 as *const i8);
                    print_confusion();
                    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                }
                str_ptr = str_ptr - 1i32;
                pool_ptr = *str_start.offset(str_ptr as isize)
            }
        }
    };
}
unsafe extern "C" fn print_wrong_stk_lit(
    mut stk_lt: i32,
    mut stk_tp1: stk_type,
    mut stk_tp2: stk_type,
) {
    if stk_tp1 as i32 != 4i32 {
        /*stk_empty */
        print_stk_lit(stk_lt, stk_tp1);
        match stk_tp2 as i32 {
            0 => {
                puts_log(b", not an integer,\x00" as *const u8 as *const i8);
            }
            1 => {
                puts_log(b", not a string,\x00" as *const u8 as *const i8);
            }
            2 => {
                puts_log(b", not a function,\x00" as *const u8 as *const i8);
            }
            3 | 4 => {
                illegl_literal_confusion();
            }
            _ => {
                unknwn_literal_confusion();
            }
        }
        bst_ex_warn_print();
    };
}
unsafe extern "C" fn pop_top_and_print() {
    let mut stk_lt: i32 = 0;
    let mut stk_tp: stk_type = 0;
    pop_lit_stk(&mut stk_lt, &mut stk_tp);
    if stk_tp as i32 == 4i32 {
        /*stk_empty */
        puts_log(b"Empty literal\n\x00" as *const u8 as *const i8);
    } else {
        print_lit(stk_lt, stk_tp);
    };
}
unsafe extern "C" fn pop_whole_stack() {
    while lit_stk_ptr > 0i32 {
        pop_top_and_print();
    }
}
unsafe extern "C" fn init_command_execution() {
    lit_stk_ptr = 0i32;
    cmd_str_ptr = str_ptr;
}
unsafe extern "C" fn check_command_execution() {
    if lit_stk_ptr != 0i32 {
        printf_log(
            b"ptr=%ld, stack=\n\x00" as *const u8 as *const i8,
            lit_stk_ptr as i64,
        );
        pop_whole_stack();
        puts_log(b"---the literal stack isn\'t empty\x00" as *const u8 as *const i8);
        bst_ex_warn_print();
    }
    if cmd_str_ptr != str_ptr {
        puts_log(b"Nonempty empty string stack\x00" as *const u8 as *const i8);
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    };
}
unsafe extern "C" fn add_pool_buf_and_push() {
    while pool_ptr + ex_buf_length > pool_size {
        pool_overflow();
    }
    ex_buf_ptr = 0i32;
    while ex_buf_ptr < ex_buf_length {
        *str_pool.offset(pool_ptr as isize) = *ex_buf.offset(ex_buf_ptr as isize);
        pool_ptr = pool_ptr + 1i32;
        ex_buf_ptr = ex_buf_ptr + 1i32
    }
    push_lit_stk(make_string(), 1i32 as stk_type);
}
unsafe extern "C" fn add_buf_pool(mut p_str: str_number) {
    p_ptr1 = *str_start.offset(p_str as isize);
    p_ptr2 = *str_start.offset((p_str + 1i32) as isize);
    if ex_buf_length + (p_ptr2 - p_ptr1) > buf_size {
        buffer_overflow();
    }
    ex_buf_ptr = ex_buf_length;
    while p_ptr1 < p_ptr2 {
        *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(p_ptr1 as isize);
        ex_buf_ptr = ex_buf_ptr + 1i32;
        p_ptr1 = p_ptr1 + 1i32
    }
    ex_buf_length = ex_buf_ptr;
}
unsafe extern "C" fn add_out_pool(mut p_str: str_number) {
    let mut break_ptr: buf_pointer = 0;
    let mut end_ptr: buf_pointer = 0;
    let mut break_pt_found: bool = false;
    let mut unbreakable_tail: bool = false;
    p_ptr1 = *str_start.offset(p_str as isize);
    p_ptr2 = *str_start.offset((p_str + 1i32) as isize);
    while out_buf_length + (p_ptr2 - p_ptr1) > buf_size {
        buffer_overflow();
    }
    out_buf_ptr = out_buf_length;
    while p_ptr1 < p_ptr2 {
        *out_buf.offset(out_buf_ptr as isize) = *str_pool.offset(p_ptr1 as isize);
        p_ptr1 = p_ptr1 + 1i32;
        out_buf_ptr = out_buf_ptr + 1i32
    }
    out_buf_length = out_buf_ptr;
    unbreakable_tail = false;
    while out_buf_length > 79i32 && !unbreakable_tail {
        /*324: */
        end_ptr = out_buf_length;
        out_buf_ptr = 79i32;
        break_pt_found = false;
        while lex_class[*out_buf.offset(out_buf_ptr as isize) as usize] as i32 != 1i32
            && out_buf_ptr >= 3i32
        {
            out_buf_ptr = out_buf_ptr - 1i32
        }
        if out_buf_ptr == 3i32 - 1i32 {
            /*325: */
            out_buf_ptr = 79i32 + 1i32;
            while out_buf_ptr < end_ptr {
                if !(lex_class[*out_buf.offset(out_buf_ptr as isize) as usize] as i32 != 1i32) {
                    break;
                }
                /*white_space */
                out_buf_ptr = out_buf_ptr + 1i32
            }
            /*loop1_exit */
            if out_buf_ptr == end_ptr {
                unbreakable_tail = true
            } else {
                break_pt_found = true;
                while out_buf_ptr + 1i32 < end_ptr {
                    if !(lex_class[*out_buf.offset((out_buf_ptr + 1i32) as isize) as usize] as i32
                        == 1i32)
                    {
                        break;
                    }
                    /*white_space */
                    out_buf_ptr = out_buf_ptr + 1i32
                }
            }
        } else {
            break_pt_found = true
        } /*space */
        if break_pt_found {
            out_buf_length = out_buf_ptr; /*space */
            break_ptr = out_buf_length + 1i32;
            output_bbl_line();
            *out_buf.offset(0) = 32i32 as ASCII_code;
            *out_buf.offset(1) = 32i32 as ASCII_code;
            out_buf_ptr = 2i32;
            tmp_ptr = break_ptr;
            while tmp_ptr < end_ptr {
                *out_buf.offset(out_buf_ptr as isize) = *out_buf.offset(tmp_ptr as isize);
                out_buf_ptr = out_buf_ptr + 1i32;
                tmp_ptr = tmp_ptr + 1i32
            }
            out_buf_length = end_ptr - break_ptr + 2i32
        }
    }
}
unsafe extern "C" fn x_equals() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != pop_typ2 as i32 {
        if pop_typ1 as i32 != 4i32 && pop_typ2 as i32 != 4i32 {
            print_stk_lit(pop_lit1, pop_typ1);
            puts_log(b", \x00" as *const u8 as *const i8);
            print_stk_lit(pop_lit2, pop_typ2);
            putc_log('\n' as i32);
            puts_log(b"---they aren\'t the same literal types\x00" as *const u8 as *const i8);
            bst_ex_warn_print();
        }
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ1 as i32 != 0i32 && pop_typ1 as i32 != 1i32 {
        if pop_typ1 as i32 != 4i32 {
            /*stk_empty */
            print_stk_lit(pop_lit1, pop_typ1);
            puts_log(b", not an integer or a string,\x00" as *const u8 as *const i8);
            bst_ex_warn_print();
        }
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ1 as i32 == 0i32 {
        /*stk_int */
        if pop_lit2 == pop_lit1 {
            push_lit_stk(1i32, 0i32 as stk_type);
        } else {
            push_lit_stk(0i32, 0i32 as stk_type);
        }
    } else if str_eq_str(pop_lit2, pop_lit1) {
        push_lit_stk(1i32, 0i32 as stk_type);
    } else {
        push_lit_stk(0i32, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_greater_than() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_lit2 > pop_lit1 {
        push_lit_stk(1i32, 0i32 as stk_type);
    } else {
        push_lit_stk(0i32, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_less_than() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_lit2 < pop_lit1 {
        push_lit_stk(1i32, 0i32 as stk_type);
    } else {
        push_lit_stk(0i32, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_plus() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else {
        push_lit_stk(pop_lit2 + pop_lit1, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_minus() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else {
        push_lit_stk(pop_lit2 - pop_lit1, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_concatenate() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ2 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 1i32 as stk_type); /*352: */
        push_lit_stk(s_null, 1i32 as stk_type); /*353: */
    } else if pop_lit2 >= cmd_str_ptr {
        if pop_lit1 >= cmd_str_ptr {
            *str_start.offset(pop_lit1 as isize) = *str_start.offset((pop_lit1 + 1i32) as isize); /*354: */
            str_ptr = str_ptr + 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize);
            lit_stk_ptr = lit_stk_ptr + 1i32
        } else if *str_start.offset((pop_lit2 + 1i32) as isize)
            - *str_start.offset(pop_lit2 as isize)
            == 0i32
        {
            push_lit_stk(pop_lit1, 1i32 as stk_type);
        } else {
            pool_ptr = *str_start.offset((pop_lit2 + 1i32) as isize);
            while pool_ptr
                + (*str_start.offset((pop_lit1 + 1i32) as isize)
                    - *str_start.offset(pop_lit1 as isize))
                > pool_size
            {
                pool_overflow();
            }
            sp_ptr = *str_start.offset(pop_lit1 as isize);
            sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr = pool_ptr + 1i32;
                sp_ptr = sp_ptr + 1i32
            }
            push_lit_stk(make_string(), 1i32 as stk_type);
        }
    } else if pop_lit1 >= cmd_str_ptr {
        if *str_start.offset((pop_lit2 + 1i32) as isize) - *str_start.offset(pop_lit2 as isize)
            == 0i32
        {
            str_ptr = str_ptr + 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize);
            *lit_stack.offset(lit_stk_ptr as isize) = pop_lit1;
            lit_stk_ptr = lit_stk_ptr + 1i32
        } else if *str_start.offset((pop_lit1 + 1i32) as isize)
            - *str_start.offset(pop_lit1 as isize)
            == 0i32
        {
            lit_stk_ptr = lit_stk_ptr + 1i32
        } else {
            sp_length = *str_start.offset((pop_lit1 + 1i32) as isize)
                - *str_start.offset(pop_lit1 as isize);
            sp2_length = *str_start.offset((pop_lit2 + 1i32) as isize)
                - *str_start.offset(pop_lit2 as isize);
            while pool_ptr + sp_length + sp2_length > pool_size {
                pool_overflow();
            }
            sp_ptr = *str_start.offset((pop_lit1 + 1i32) as isize);
            sp_end = *str_start.offset(pop_lit1 as isize);
            sp_xptr1 = sp_ptr + sp2_length;
            while sp_ptr > sp_end {
                sp_ptr = sp_ptr - 1i32;
                sp_xptr1 = sp_xptr1 - 1i32;
                *str_pool.offset(sp_xptr1 as isize) = *str_pool.offset(sp_ptr as isize)
            }
            sp_ptr = *str_start.offset(pop_lit2 as isize);
            sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr = pool_ptr + 1i32;
                sp_ptr = sp_ptr + 1i32
            }
            pool_ptr = pool_ptr + sp_length;
            push_lit_stk(make_string(), 1i32 as stk_type);
        }
    } else if *str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize)
        == 0i32
    {
        lit_stk_ptr = lit_stk_ptr + 1i32
    } else if *str_start.offset((pop_lit2 + 1i32) as isize) - *str_start.offset(pop_lit2 as isize)
        == 0i32
    {
        push_lit_stk(pop_lit1, 1i32 as stk_type);
    } else {
        while pool_ptr
            + (*str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize))
            + (*str_start.offset((pop_lit2 + 1i32) as isize) - *str_start.offset(pop_lit2 as isize))
            > pool_size
        {
            pool_overflow();
        }
        sp_ptr = *str_start.offset(pop_lit2 as isize);
        sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
        while sp_ptr < sp_end {
            *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            pool_ptr = pool_ptr + 1i32;
            sp_ptr = sp_ptr + 1i32
        }
        sp_ptr = *str_start.offset(pop_lit1 as isize);
        sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
        while sp_ptr < sp_end {
            *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            pool_ptr = pool_ptr + 1i32;
            sp_ptr = sp_ptr + 1i32
        }
        push_lit_stk(make_string(), 1i32 as stk_type);
    };
}
unsafe extern "C" fn x_gets() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 2i32 {
        /*stk_fn */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
    } else if !mess_with_entries
        && (*fn_type.offset(pop_lit1 as isize) as i32 == 6i32
            || *fn_type.offset(pop_lit1 as isize) as i32 == 5i32)
    {
        bst_cant_mess_with_entries_print();
    } else {
        match *fn_type.offset(pop_lit1 as isize) as i32 {
            5 => {
                /*
                356: */
                if pop_typ2 as i32 != 0i32 {
                    /*stk_int */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
                } else {
                    *entry_ints.offset(
                        (cite_ptr * num_ent_ints + *ilk_info.offset(pop_lit1 as isize)) as isize,
                    ) = pop_lit2
                }
            }
            6 => {
                if pop_typ2 as i32 != 1i32 {
                    /*stk_str */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, 1i32 as stk_type);
                } else {
                    str_ent_ptr = cite_ptr * num_ent_strs + *ilk_info.offset(pop_lit1 as isize);
                    ent_chr_ptr = 0i32;
                    sp_ptr = *str_start.offset(pop_lit2 as isize);
                    sp_xptr1 = *str_start.offset((pop_lit2 + 1i32) as isize);
                    if sp_xptr1 - sp_ptr > ent_str_size {
                        bst_1print_string_size_exceeded();
                        printf_log(
                            b"%ld, the entry\x00" as *const u8 as *const i8,
                            ent_str_size as i64,
                        );
                        bst_2print_string_size_exceeded();
                        sp_xptr1 = sp_ptr + ent_str_size
                    }
                    while sp_ptr < sp_xptr1 {
                        *entry_strs
                            .offset((str_ent_ptr * (ent_str_size + 1i32) + ent_chr_ptr) as isize) =
                            *str_pool.offset(sp_ptr as isize);
                        ent_chr_ptr = ent_chr_ptr + 1i32;
                        sp_ptr = sp_ptr + 1i32
                    }
                    *entry_strs
                        .offset((str_ent_ptr * (ent_str_size + 1i32) + ent_chr_ptr) as isize) =
                        127i32 as ASCII_code
                    /*end_of_string */
                }
            }
            7 => {
                if pop_typ2 as i32 != 0i32 {
                    /*stk_int */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
                } else {
                    *ilk_info.offset(pop_lit1 as isize) = pop_lit2
                }
            }
            8 => {
                if pop_typ2 as i32 != 1i32 {
                    /*stk_str */
                    print_wrong_stk_lit(pop_lit2, pop_typ2, 1i32 as stk_type);
                } else {
                    str_glb_ptr = *ilk_info.offset(pop_lit1 as isize);
                    if pop_lit2 < cmd_str_ptr {
                        *glb_str_ptr.offset(str_glb_ptr as isize) = pop_lit2
                    } else {
                        *glb_str_ptr.offset(str_glb_ptr as isize) = 0i32;
                        glob_chr_ptr = 0i32;
                        sp_ptr = *str_start.offset(pop_lit2 as isize);
                        sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
                        if sp_end - sp_ptr > glob_str_size {
                            bst_1print_string_size_exceeded();
                            printf_log(
                                b"%ld, the global\x00" as *const u8 as *const i8,
                                glob_str_size as i64,
                            );
                            bst_2print_string_size_exceeded();
                            sp_end = sp_ptr + glob_str_size
                        }
                        while sp_ptr < sp_end {
                            *global_strs.offset(
                                (str_glb_ptr * (glob_str_size + 1i32) + glob_chr_ptr) as isize,
                            ) = *str_pool.offset(sp_ptr as isize);
                            glob_chr_ptr = glob_chr_ptr + 1i32;
                            sp_ptr = sp_ptr + 1i32
                        }
                        *glb_str_end.offset(str_glb_ptr as isize) = glob_chr_ptr
                    }
                }
            }
            _ => {
                puts_log(b"You can\'t assign to type \x00" as *const u8 as *const i8);
                print_fn_class(pop_lit1);
                puts_log(b", a nonvariable function class\x00" as *const u8 as *const i8);
                bst_ex_warn_print();
            }
        }
    };
}
unsafe extern "C" fn x_add_period() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if *str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize)
        == 0i32
    {
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        /*362: */
        sp_ptr = *str_start.offset((pop_lit1 + 1i32) as isize);
        sp_end = *str_start.offset(pop_lit1 as isize);
        while sp_ptr > sp_end {
            sp_ptr = sp_ptr - 1i32;
            if *str_pool.offset(sp_ptr as isize) as i32 != 125i32 {
                break;
            }
        }
        /*right_brace */
        match *str_pool.offset(sp_ptr as isize) as i32 {
            46 | 63 | 33 => {
                if *lit_stack.offset(lit_stk_ptr as isize) >= cmd_str_ptr {
                    str_ptr = str_ptr + 1i32; /*period */
                    pool_ptr = *str_start.offset(str_ptr as isize)
                }
                lit_stk_ptr = lit_stk_ptr + 1i32
            }
            _ => {
                if pop_lit1 < cmd_str_ptr {
                    while pool_ptr
                        + (*str_start.offset((pop_lit1 + 1i32) as isize)
                            - *str_start.offset(pop_lit1 as isize))
                        + 1i32
                        > pool_size
                    {
                        pool_overflow();
                    }
                    sp_ptr = *str_start.offset(pop_lit1 as isize);
                    sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
                    while sp_ptr < sp_end {
                        *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                        pool_ptr = pool_ptr + 1i32;
                        sp_ptr = sp_ptr + 1i32
                    }
                } else {
                    pool_ptr = *str_start.offset((pop_lit1 + 1i32) as isize);
                    while pool_ptr + 1i32 > pool_size {
                        pool_overflow();
                    }
                }
                *str_pool.offset(pool_ptr as isize) = 46i32 as ASCII_code;
                pool_ptr = pool_ptr + 1i32;
                push_lit_stk(make_string(), 1i32 as stk_type);
            }
        }
    };
}
unsafe extern "C" fn x_change_case() {
    let mut current_block: u64;
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ2 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 1i32 as stk_type); /*title_lowers */
        push_lit_stk(s_null, 1i32 as stk_type); /*all_lowers */
    } else {
        match *str_pool.offset(*str_start.offset(pop_lit1 as isize) as isize) as i32 {
            116 | 84 => conversion_type = 0_u8,
            108 | 76 => conversion_type = 1_u8,
            117 | 85 => conversion_type = 2_u8,
            _ => {
                /*all_uppers */
                conversion_type = 3_u8
            }
        } /*bad_conversion */
        if *str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize)
            != 1i32
            || conversion_type as i32 == 3i32
        {
            conversion_type = 3_u8; /*bad_conversion */
            print_a_pool_str(pop_lit1);
            puts_log(b" is an illegal case-conversion string\x00" as *const u8 as *const i8);
            bst_ex_warn_print();
        }
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit2);
        brace_level = 0i32;
        ex_buf_ptr = 0i32;
        while ex_buf_ptr < ex_buf_length {
            if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                /*left_brace */
                brace_level = brace_level + 1i32;
                if !(brace_level != 1i32) {
                    if !(ex_buf_ptr + 4i32 > ex_buf_length) {
                        if !(*ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 != 92i32) {
                            if conversion_type as i32 == 0i32 {
                                /*title_lowers */
                                if ex_buf_ptr == 0i32 {
                                    current_block = 17089879097653631793;
                                } else if prev_colon as i32 != 0
                                    && lex_class
                                        [*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as usize]
                                        as i32
                                        == 1i32
                                {
                                    current_block = 17089879097653631793;
                                } else {
                                    current_block = 6417057564578538666;
                                }
                            } else {
                                current_block = 6417057564578538666;
                            }
                            match current_block {
                                17089879097653631793 => {}
                                _ => {
                                    ex_buf_ptr = ex_buf_ptr + 1i32;
                                    while ex_buf_ptr < ex_buf_length && brace_level > 0i32 {
                                        ex_buf_ptr = ex_buf_ptr + 1i32;
                                        ex_buf_xptr = ex_buf_ptr;
                                        while ex_buf_ptr < ex_buf_length
                                            && lex_class
                                                [*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                                as i32
                                                == 2i32
                                        {
                                            ex_buf_ptr = ex_buf_ptr + 1i32
                                        }
                                        control_seq_loc = str_lookup(
                                            ex_buf,
                                            ex_buf_xptr,
                                            ex_buf_ptr - ex_buf_xptr,
                                            14i32 as str_ilk,
                                            false,
                                        );
                                        if hash_found {
                                            /*373: */
                                            match conversion_type as i32 {
                                                0 | 1 => {
                                                    match *ilk_info.offset(control_seq_loc as isize)
                                                    {
                                                        11 | 9 | 3 | 5 | 7 => {
                                                            lower_case(
                                                                ex_buf,
                                                                ex_buf_xptr,
                                                                ex_buf_ptr - ex_buf_xptr,
                                                            );
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                2 => {
                                                    match *ilk_info.offset(control_seq_loc as isize)
                                                    {
                                                        10 | 8 | 2 | 4 | 6 => {
                                                            upper_case(
                                                                ex_buf,
                                                                ex_buf_xptr,
                                                                ex_buf_ptr - ex_buf_xptr,
                                                            );
                                                        }
                                                        0 | 1 | 12 => {
                                                            upper_case(
                                                                ex_buf,
                                                                ex_buf_xptr,
                                                                ex_buf_ptr - ex_buf_xptr,
                                                            );
                                                            while ex_buf_xptr < ex_buf_ptr {
                                                                *ex_buf.offset(
                                                                    (ex_buf_xptr - 1i32) as isize,
                                                                ) = *ex_buf
                                                                    .offset(ex_buf_xptr as isize);
                                                                ex_buf_xptr = ex_buf_xptr + 1i32
                                                            }
                                                            ex_buf_xptr = ex_buf_xptr - 1i32;
                                                            while ex_buf_ptr < ex_buf_length
                                                                && lex_class[*ex_buf
                                                                    .offset(ex_buf_ptr as isize)
                                                                    as usize]
                                                                    as i32
                                                                    == 1i32
                                                            {
                                                                ex_buf_ptr = ex_buf_ptr + 1i32
                                                            }
                                                            tmp_ptr = ex_buf_ptr;
                                                            while tmp_ptr < ex_buf_length {
                                                                *ex_buf.offset(
                                                                    (tmp_ptr
                                                                        - (ex_buf_ptr
                                                                            - ex_buf_xptr))
                                                                        as isize,
                                                                ) = *ex_buf
                                                                    .offset(tmp_ptr as isize);
                                                                tmp_ptr = tmp_ptr + 1i32
                                                            }
                                                            ex_buf_length = tmp_ptr
                                                                - (ex_buf_ptr - ex_buf_xptr);
                                                            ex_buf_ptr = ex_buf_xptr
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                3 => {}
                                                _ => {
                                                    case_conversion_confusion();
                                                }
                                            }
                                        }
                                        ex_buf_xptr = ex_buf_ptr;
                                        while ex_buf_ptr < ex_buf_length
                                            && brace_level > 0i32
                                            && *ex_buf.offset(ex_buf_ptr as isize) as i32 != 92i32
                                        {
                                            if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32
                                            {
                                                /*right_brace */
                                                brace_level = brace_level - 1i32
                                            } else if *ex_buf.offset(ex_buf_ptr as isize) as i32
                                                == 123i32
                                            {
                                                /*left_brace */
                                                brace_level = brace_level + 1i32
                                            }
                                            ex_buf_ptr = ex_buf_ptr + 1i32
                                        }
                                        match conversion_type as i32 {
                                            0 | 1 => {
                                                lower_case(
                                                    ex_buf,
                                                    ex_buf_xptr,
                                                    ex_buf_ptr - ex_buf_xptr,
                                                );
                                            }
                                            2 => {
                                                upper_case(
                                                    ex_buf,
                                                    ex_buf_xptr,
                                                    ex_buf_ptr - ex_buf_xptr,
                                                );
                                            }
                                            3 => {}
                                            _ => {
                                                case_conversion_confusion();
                                            }
                                        }
                                    }
                                    ex_buf_ptr = ex_buf_ptr - 1i32
                                }
                            }
                        }
                    }
                }
                /*backslash */
                /*ok_pascal_i_give_up */
                prev_colon = false
            } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                /*right_brace */
                decr_brace_level(pop_lit2);
                prev_colon = false
            } else if brace_level == 0i32 {
                /*377: */
                match conversion_type as i32 {
                    0 => {
                        if !(ex_buf_ptr == 0i32) {
                            if !(prev_colon as i32 != 0
                                && lex_class[*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as usize]
                                    as i32
                                    == 1i32)
                            {
                                lower_case(ex_buf, ex_buf_ptr, 1i32);
                            }
                        }
                        if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 58i32 {
                            /*colon */
                            prev_colon = true
                        } else if lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] as i32
                            != 1i32
                        {
                            /*white_space */
                            prev_colon = false
                        }
                    }
                    1 => {
                        lower_case(ex_buf, ex_buf_ptr, 1i32);
                    }
                    2 => {
                        upper_case(ex_buf, ex_buf_ptr, 1i32);
                    }
                    3 => {}
                    _ => {
                        case_conversion_confusion();
                    }
                }
            }
            ex_buf_ptr = ex_buf_ptr + 1i32
        }
        check_brace_level(pop_lit2);
        add_pool_buf_and_push();
    };
}
unsafe extern "C" fn x_chr_to_int() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if *str_start.offset((pop_lit1 + 1i32) as isize) - *str_start.offset(pop_lit1 as isize)
        != 1i32
    {
        putc_log('\"' as i32);
        print_a_pool_str(pop_lit1);
        puts_log(b"\" isn\'t a single character\x00" as *const u8 as *const i8);
        bst_ex_warn_print();
        push_lit_stk(0i32, 0i32 as stk_type);
    } else {
        push_lit_stk(
            *str_pool.offset(*str_start.offset(pop_lit1 as isize) as isize) as i32,
            0i32 as stk_type,
        );
    };
}
unsafe extern "C" fn x_cite() {
    if !mess_with_entries {
        bst_cant_mess_with_entries_print();
    } else {
        push_lit_stk(*cite_list.offset(cite_ptr as isize), 1i32 as stk_type);
    };
}
unsafe extern "C" fn x_duplicate() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        push_lit_stk(pop_lit1, pop_typ1);
        push_lit_stk(pop_lit1, pop_typ1);
    } else {
        if *lit_stack.offset(lit_stk_ptr as isize) >= cmd_str_ptr {
            str_ptr = str_ptr + 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize)
        }
        lit_stk_ptr = lit_stk_ptr + 1i32;
        if pop_lit1 < cmd_str_ptr {
            push_lit_stk(pop_lit1, pop_typ1);
        } else {
            while pool_ptr
                + (*str_start.offset((pop_lit1 + 1i32) as isize)
                    - *str_start.offset(pop_lit1 as isize))
                > pool_size
            {
                pool_overflow();
            }
            sp_ptr = *str_start.offset(pop_lit1 as isize);
            sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr = pool_ptr + 1i32;
                sp_ptr = sp_ptr + 1i32
            }
            push_lit_stk(make_string(), 1i32 as stk_type);
        }
    };
}
unsafe extern "C" fn x_empty() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    match pop_typ1 as i32 {
        1 => {
            sp_ptr = *str_start.offset(pop_lit1 as isize);
            sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
            while sp_ptr < sp_end {
                if lex_class[*str_pool.offset(sp_ptr as isize) as usize] as i32 != 1i32 {
                    /*white_space */
                    push_lit_stk(0i32, 0i32 as stk_type);
                    return;
                }
                sp_ptr = sp_ptr + 1i32
            }
            push_lit_stk(1i32, 0i32 as stk_type);
        }
        3 => {
            push_lit_stk(1i32, 0i32 as stk_type);
        }
        4 => {
            push_lit_stk(0i32, 0i32 as stk_type);
        }
        _ => {
            print_stk_lit(pop_lit1, pop_typ1);
            puts_log(b", not a string or missing field,\x00" as *const u8 as *const i8);
            bst_ex_warn_print();
            push_lit_stk(0i32, 0i32 as stk_type);
        }
    };
}
unsafe extern "C" fn x_format_name() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ3 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit3, pop_typ3, 1i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit3);
        ex_buf_ptr = 0i32;
        num_names = 0i32;
        while num_names < pop_lit2 && ex_buf_ptr < ex_buf_length {
            num_names = num_names + 1i32;
            ex_buf_xptr = ex_buf_ptr;
            name_scan_for_and(pop_lit3);
        }
        if ex_buf_ptr < ex_buf_length {
            ex_buf_ptr = ex_buf_ptr - 4i32
        }
        if num_names < pop_lit2 {
            if pop_lit2 == 1i32 {
                puts_log(b"There is no name in \"\x00" as *const u8 as *const i8);
            } else {
                printf_log(
                    b"There aren\'t %ld names in \"\x00" as *const u8 as *const i8,
                    pop_lit2 as i64,
                );
            }
            print_a_pool_str(pop_lit3);
            putc_log('\"' as i32);
            bst_ex_warn_print();
        }
        while ex_buf_ptr > ex_buf_xptr {
            match lex_class[*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as usize] as i32 {
                1 | 4 => ex_buf_ptr = ex_buf_ptr - 1i32,
                _ => {
                    if !(*ex_buf.offset((ex_buf_ptr - 1i32) as isize) as i32 == 44i32) {
                        break;
                    }
                    /*comma */
                    printf_log(
                        b"Name %ld in \"\x00" as *const u8 as *const i8,
                        pop_lit2 as i64,
                    );
                    print_a_pool_str(pop_lit3);
                    puts_log(b"\" has a comma at the end\x00" as *const u8 as *const i8);
                    bst_ex_warn_print();
                    ex_buf_ptr = ex_buf_ptr - 1i32
                }
            }
        }
        name_bf_ptr = 0i32;
        num_commas = 0i32;
        num_tokens = 0i32;
        token_starting = true;
        while ex_buf_xptr < ex_buf_ptr {
            match *ex_buf.offset(ex_buf_xptr as isize) as i32 {
                44 => {
                    if num_commas == 2i32 {
                        printf_log(
                            b"Too many commas in name %ld of \"\x00" as *const u8 as *const i8,
                            pop_lit2 as i64,
                        );
                        print_a_pool_str(pop_lit3);
                        putc_log('\"' as i32);
                        bst_ex_warn_print();
                    } else {
                        num_commas = num_commas + 1i32;
                        if num_commas == 1i32 {
                            comma1 = num_tokens
                        } else {
                            comma2 = num_tokens
                        }
                        *name_sep_char.offset(num_tokens as isize) = 44i32 as ASCII_code
                        /*comma */
                    }
                    ex_buf_xptr = ex_buf_xptr + 1i32;
                    token_starting = true
                }
                123 => {
                    brace_level = brace_level + 1i32;
                    if token_starting {
                        *name_tok.offset(num_tokens as isize) = name_bf_ptr;
                        num_tokens = num_tokens + 1i32
                    }
                    *sv_buffer.offset(name_bf_ptr as isize) = *ex_buf.offset(ex_buf_xptr as isize);
                    name_bf_ptr = name_bf_ptr + 1i32;
                    ex_buf_xptr = ex_buf_xptr + 1i32;
                    while brace_level > 0i32 && ex_buf_xptr < ex_buf_ptr {
                        if *ex_buf.offset(ex_buf_xptr as isize) as i32 == 125i32 {
                            /*right_brace */
                            brace_level = brace_level - 1i32
                        } else if *ex_buf.offset(ex_buf_xptr as isize) as i32 == 123i32 {
                            /*left_brace */
                            brace_level = brace_level + 1i32
                        } /*space */
                        *sv_buffer.offset(name_bf_ptr as isize) =
                            *ex_buf.offset(ex_buf_xptr as isize);
                        name_bf_ptr = name_bf_ptr + 1i32;
                        ex_buf_xptr = ex_buf_xptr + 1i32
                    }
                    token_starting = false
                }
                125 => {
                    if token_starting {
                        *name_tok.offset(num_tokens as isize) = name_bf_ptr;
                        num_tokens = num_tokens + 1i32
                    }
                    printf_log(
                        b"Name %ld of \"\x00" as *const u8 as *const i8,
                        pop_lit2 as i64,
                    );
                    print_a_pool_str(pop_lit3);
                    puts_log(b"\" isn\'t brace balanced\x00" as *const u8 as *const i8);
                    bst_ex_warn_print();
                    ex_buf_xptr = ex_buf_xptr + 1i32;
                    token_starting = false
                }
                _ => match lex_class[*ex_buf.offset(ex_buf_xptr as isize) as usize] as i32 {
                    1 => {
                        if !token_starting {
                            *name_sep_char.offset(num_tokens as isize) = 32i32 as ASCII_code
                        }
                        ex_buf_xptr = ex_buf_xptr + 1i32;
                        token_starting = true
                    }
                    4 => {
                        if !token_starting {
                            *name_sep_char.offset(num_tokens as isize) =
                                *ex_buf.offset(ex_buf_xptr as isize)
                        }
                        ex_buf_xptr = ex_buf_xptr + 1i32;
                        token_starting = true
                    }
                    _ => {
                        if token_starting {
                            *name_tok.offset(num_tokens as isize) = name_bf_ptr;
                            num_tokens = num_tokens + 1i32
                        }
                        *sv_buffer.offset(name_bf_ptr as isize) =
                            *ex_buf.offset(ex_buf_xptr as isize);
                        name_bf_ptr = name_bf_ptr + 1i32;
                        ex_buf_xptr = ex_buf_xptr + 1i32;
                        token_starting = false
                    }
                },
            }
        }
        *name_tok.offset(num_tokens as isize) = name_bf_ptr;
        if num_commas == 0i32 {
            first_start = 0i32;
            last_end = num_tokens;
            jr_end = last_end;
            let mut current_block_127: u64;
            von_start = 0i32;
            loop {
                if !(von_start < last_end - 1i32) {
                    current_block_127 = 248631179418912492;
                    break;
                }
                name_bf_ptr = *name_tok.offset(von_start as isize);
                name_bf_xptr = *name_tok.offset((von_start + 1i32) as isize);
                if von_token_found() {
                    von_name_ends_and_last_name_starts_stuff();
                    current_block_127 = 7590078969446600227;
                    break;
                } else {
                    von_start = von_start + 1i32
                }
            }
            loop {
                match current_block_127 {
                    7590078969446600227 => {
                        /*von_found */
                        first_end = von_start;
                        break;
                    }
                    _ => {
                        if von_start > 0i32 {
                            if !(lex_class[*name_sep_char.offset(von_start as isize) as usize]
                                as i32
                                != 4i32
                                || *name_sep_char.offset(von_start as isize) as i32 == 126i32)
                            {
                                von_start = von_start - 1i32;
                                current_block_127 = 248631179418912492;
                                continue;
                            }
                        }
                        /*loop2_exit */
                        von_end = von_start;
                        current_block_127 = 7590078969446600227;
                    }
                }
            }
        } else if num_commas == 1i32 {
            von_start = 0i32;
            last_end = comma1;
            jr_end = last_end;
            first_start = jr_end;
            first_end = num_tokens;
            von_name_ends_and_last_name_starts_stuff();
        } else if num_commas == 2i32 {
            von_start = 0i32;
            last_end = comma1;
            jr_end = comma2;
            first_start = jr_end;
            first_end = num_tokens;
            von_name_ends_and_last_name_starts_stuff();
        } else {
            puts_log(b"Illegal number of comma,s\x00" as *const u8 as *const i8);
            print_confusion();
            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
        }
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        figure_out_the_formatted_name();
        add_pool_buf_and_push();
    };
}
unsafe extern "C" fn x_int_to_chr() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_lit1 < 0i32 || pop_lit1 > 127i32 {
        printf_log(
            b"%ld isn\'t valid ASCII\x00" as *const u8 as *const i8,
            pop_lit1 as i64,
        );
        bst_ex_warn_print();
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        while pool_ptr + 1i32 > pool_size {
            pool_overflow();
        }
        *str_pool.offset(pool_ptr as isize) = pop_lit1 as ASCII_code;
        pool_ptr = pool_ptr + 1i32;
        push_lit_stk(make_string(), 1i32 as stk_type);
    };
}
unsafe extern "C" fn x_int_to_str() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        int_to_ASCII(pop_lit1, ex_buf, 0i32, &mut ex_buf_length);
        add_pool_buf_and_push();
    };
}
unsafe extern "C" fn x_missing() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if !mess_with_entries {
        bst_cant_mess_with_entries_print();
    } else if pop_typ1 as i32 != 1i32 && pop_typ1 as i32 != 3i32 {
        if pop_typ1 as i32 != 4i32 {
            /*stk_empty */
            print_stk_lit(pop_lit1, pop_typ1);
            puts_log(b", not a string or missing field,\x00" as *const u8 as *const i8);
            bst_ex_warn_print();
        }
        push_lit_stk(0i32, 0i32 as stk_type);
    } else if pop_typ1 as i32 == 3i32 {
        /*stk_field_missing */
        push_lit_stk(1i32, 0i32 as stk_type);
    } else {
        push_lit_stk(0i32, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_num_names() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        ex_buf_ptr = 0i32;
        num_names = 0i32;
        while ex_buf_ptr < ex_buf_length {
            name_scan_for_and(pop_lit1);
            num_names = num_names + 1i32
        }
        push_lit_stk(num_names, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_preamble() {
    ex_buf_length = 0i32;
    preamble_ptr = 0i32;
    while preamble_ptr < num_preamble_strings {
        add_buf_pool(*s_preamble.offset(preamble_ptr as isize));
        preamble_ptr = preamble_ptr + 1i32
    }
    add_pool_buf_and_push();
}
unsafe extern "C" fn x_purify() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type); /*space */
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        brace_level = 0i32;
        ex_buf_xptr = 0i32;
        ex_buf_ptr = 0i32;
        while ex_buf_ptr < ex_buf_length {
            match lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] as i32 {
                1 | 4 => {
                    *ex_buf.offset(ex_buf_xptr as isize) = 32i32 as ASCII_code;
                    ex_buf_xptr = ex_buf_xptr + 1i32
                }
                2 | 3 => {
                    *ex_buf.offset(ex_buf_xptr as isize) = *ex_buf.offset(ex_buf_ptr as isize);
                    ex_buf_xptr = ex_buf_xptr + 1i32
                }
                _ => {
                    if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                        /*left_brace */
                        brace_level = brace_level + 1i32;
                        if brace_level == 1i32 && ex_buf_ptr + 1i32 < ex_buf_length {
                            if *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 92i32 {
                                /*backslash */
                                /*433: */
                                ex_buf_ptr = ex_buf_ptr + 1i32;
                                while ex_buf_ptr < ex_buf_length && brace_level > 0i32 {
                                    ex_buf_ptr = ex_buf_ptr + 1i32;
                                    ex_buf_yptr = ex_buf_ptr;
                                    while ex_buf_ptr < ex_buf_length
                                        && lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                            as i32
                                            == 2i32
                                    {
                                        ex_buf_ptr = ex_buf_ptr + 1i32
                                    }
                                    control_seq_loc = str_lookup(
                                        ex_buf,
                                        ex_buf_yptr,
                                        ex_buf_ptr - ex_buf_yptr,
                                        14i32 as str_ilk,
                                        false,
                                    );
                                    if hash_found {
                                        /*434: */
                                        *ex_buf.offset(ex_buf_xptr as isize) =
                                            *ex_buf.offset(ex_buf_yptr as isize);
                                        ex_buf_xptr = ex_buf_xptr + 1i32;
                                        match *ilk_info.offset(control_seq_loc as isize) {
                                            2 | 3 | 4 | 5 | 12 => {
                                                *ex_buf.offset(ex_buf_xptr as isize) =
                                                    *ex_buf.offset((ex_buf_yptr + 1i32) as isize);
                                                ex_buf_xptr = ex_buf_xptr + 1i32
                                            }
                                            _ => {}
                                        }
                                    }
                                    while ex_buf_ptr < ex_buf_length
                                        && brace_level > 0i32
                                        && *ex_buf.offset(ex_buf_ptr as isize) as i32 != 92i32
                                    {
                                        match lex_class
                                            [*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                            as i32
                                        {
                                            2 | 3 => {
                                                *ex_buf.offset(ex_buf_xptr as isize) =
                                                    *ex_buf.offset(ex_buf_ptr as isize);
                                                ex_buf_xptr = ex_buf_xptr + 1i32
                                            }
                                            _ => {
                                                if *ex_buf.offset(ex_buf_ptr as isize) as i32
                                                    == 125i32
                                                {
                                                    /*right_brace */
                                                    brace_level = brace_level - 1i32
                                                } else if *ex_buf.offset(ex_buf_ptr as isize) as i32
                                                    == 123i32
                                                {
                                                    /*left_brace */
                                                    brace_level = brace_level + 1i32
                                                }
                                            }
                                        }
                                        ex_buf_ptr = ex_buf_ptr + 1i32
                                    }
                                }
                                ex_buf_ptr = ex_buf_ptr - 1i32
                            }
                        }
                    } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                        /*right_brace */
                        if brace_level > 0i32 {
                            brace_level = brace_level - 1i32
                        }
                    }
                }
            } /*double_quote */
            ex_buf_ptr = ex_buf_ptr + 1i32
        }
        ex_buf_length = ex_buf_xptr;
        add_pool_buf_and_push();
    };
}
unsafe extern "C" fn x_quote() {
    while pool_ptr + 1i32 > pool_size {
        pool_overflow();
    }
    *str_pool.offset(pool_ptr as isize) = 34i32 as ASCII_code;
    pool_ptr = pool_ptr + 1i32;
    push_lit_stk(make_string(), 1i32 as stk_type);
}
unsafe extern "C" fn x_substring() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ2 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ3 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit3, pop_typ3, 1i32 as stk_type); /*439: */
        push_lit_stk(s_null, 1i32 as stk_type); /*441: */
    } else {
        sp_length =
            *str_start.offset((pop_lit3 + 1i32) as isize) - *str_start.offset(pop_lit3 as isize);
        if pop_lit1 >= sp_length {
            if pop_lit2 == 1i32 || pop_lit2 == -1i32 {
                if *lit_stack.offset(lit_stk_ptr as isize) >= cmd_str_ptr {
                    str_ptr = str_ptr + 1i32;
                    pool_ptr = *str_start.offset(str_ptr as isize)
                }
                lit_stk_ptr = lit_stk_ptr + 1i32;
                return;
            }
        }
        if pop_lit1 <= 0i32 || pop_lit2 == 0i32 || pop_lit2 > sp_length || pop_lit2 < -sp_length {
            push_lit_stk(s_null, 1i32 as stk_type);
            return;
        } else {
            if pop_lit2 > 0i32 {
                if pop_lit1 > sp_length - (pop_lit2 - 1i32) {
                    pop_lit1 = sp_length - (pop_lit2 - 1i32)
                }
                sp_ptr = *str_start.offset(pop_lit3 as isize) + (pop_lit2 - 1i32);
                sp_end = sp_ptr + pop_lit1;
                if pop_lit2 == 1i32 {
                    if pop_lit3 >= cmd_str_ptr {
                        *str_start.offset((pop_lit3 + 1i32) as isize) = sp_end;
                        str_ptr = str_ptr + 1i32;
                        pool_ptr = *str_start.offset(str_ptr as isize);
                        lit_stk_ptr = lit_stk_ptr + 1i32;
                        return;
                    }
                }
            } else {
                pop_lit2 = -pop_lit2;
                if pop_lit1 > sp_length - (pop_lit2 - 1i32) {
                    pop_lit1 = sp_length - (pop_lit2 - 1i32)
                }
                sp_end = *str_start.offset((pop_lit3 + 1i32) as isize) - (pop_lit2 - 1i32);
                sp_ptr = sp_end - pop_lit1
            }
            while pool_ptr + sp_end - sp_ptr > pool_size {
                pool_overflow();
            }
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr = pool_ptr + 1i32;
                sp_ptr = sp_ptr + 1i32
            }
            push_lit_stk(make_string(), 1i32 as stk_type);
        }
    };
}
unsafe extern "C" fn x_swap() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 1i32 || pop_lit1 < cmd_str_ptr {
        push_lit_stk(pop_lit1, pop_typ1);
        if pop_typ2 as i32 == 1i32 && pop_lit2 >= cmd_str_ptr {
            str_ptr = str_ptr + 1i32;
            pool_ptr = *str_start.offset(str_ptr as isize)
        }
        push_lit_stk(pop_lit2, pop_typ2);
    } else if pop_typ2 as i32 != 1i32 || pop_lit2 < cmd_str_ptr {
        str_ptr = str_ptr + 1i32;
        pool_ptr = *str_start.offset(str_ptr as isize);
        push_lit_stk(pop_lit1, 1i32 as stk_type);
        push_lit_stk(pop_lit2, pop_typ2);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit2);
        sp_ptr = *str_start.offset(pop_lit1 as isize);
        sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
        while sp_ptr < sp_end {
            *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
            pool_ptr = pool_ptr + 1i32;
            sp_ptr = sp_ptr + 1i32
        }
        push_lit_stk(make_string(), 1i32 as stk_type);
        add_pool_buf_and_push();
    };
}
unsafe extern "C" fn x_text_length() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        num_text_chars = 0i32;
        sp_ptr = *str_start.offset(pop_lit1 as isize);
        sp_end = *str_start.offset((pop_lit1 + 1i32) as isize);
        sp_brace_level = 0i32;
        while sp_ptr < sp_end {
            sp_ptr = sp_ptr + 1i32;
            if *str_pool.offset((sp_ptr - 1i32) as isize) as i32 == 123i32 {
                /*left_brace */
                sp_brace_level = sp_brace_level + 1i32;
                if sp_brace_level == 1i32 && sp_ptr < sp_end {
                    if *str_pool.offset(sp_ptr as isize) as i32 == 92i32 {
                        /*backslash */
                        sp_ptr = sp_ptr + 1i32;
                        while sp_ptr < sp_end && sp_brace_level > 0i32 {
                            if *str_pool.offset(sp_ptr as isize) as i32 == 125i32 {
                                /*right_brace */
                                sp_brace_level = sp_brace_level - 1i32
                            } else if *str_pool.offset(sp_ptr as isize) as i32 == 123i32 {
                                /*left_brace */
                                sp_brace_level = sp_brace_level + 1i32
                            }
                            sp_ptr = sp_ptr + 1i32
                        }
                        num_text_chars = num_text_chars + 1i32
                    }
                }
            } else if *str_pool.offset((sp_ptr - 1i32) as isize) as i32 == 125i32 {
                /*right_brace */
                if sp_brace_level > 0i32 {
                    sp_brace_level = sp_brace_level - 1i32
                }
            } else {
                num_text_chars = num_text_chars + 1i32
            }
        }
        push_lit_stk(num_text_chars, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_text_prefix() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
    if pop_typ1 as i32 != 0i32 {
        /*stk_int */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_typ2 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit2, pop_typ2, 1i32 as stk_type); /*445: */
        push_lit_stk(s_null, 1i32 as stk_type);
    } else if pop_lit1 <= 0i32 {
        push_lit_stk(s_null, 1i32 as stk_type);
        return;
    } else {
        sp_ptr = *str_start.offset(pop_lit2 as isize);
        sp_end = *str_start.offset((pop_lit2 + 1i32) as isize);
        num_text_chars = 0i32;
        sp_brace_level = 0i32;
        sp_xptr1 = sp_ptr;
        while sp_xptr1 < sp_end && num_text_chars < pop_lit1 {
            sp_xptr1 = sp_xptr1 + 1i32;
            if *str_pool.offset((sp_xptr1 - 1i32) as isize) as i32 == 123i32 {
                /*left_brace */
                sp_brace_level = sp_brace_level + 1i32;
                if sp_brace_level == 1i32 && sp_xptr1 < sp_end {
                    if *str_pool.offset(sp_xptr1 as isize) as i32 == 92i32 {
                        /*backslash */
                        sp_xptr1 = sp_xptr1 + 1i32;
                        while sp_xptr1 < sp_end && sp_brace_level > 0i32 {
                            if *str_pool.offset(sp_xptr1 as isize) as i32 == 125i32 {
                                /*right_brace */
                                sp_brace_level = sp_brace_level - 1i32
                            } else if *str_pool.offset(sp_xptr1 as isize) as i32 == 123i32 {
                                /*left_brace */
                                sp_brace_level = sp_brace_level + 1i32
                            }
                            sp_xptr1 = sp_xptr1 + 1i32
                        }
                        num_text_chars = num_text_chars + 1i32
                    }
                }
            } else if *str_pool.offset((sp_xptr1 - 1i32) as isize) as i32 == 125i32 {
                /*right_brace */
                if sp_brace_level > 0i32 {
                    sp_brace_level = sp_brace_level - 1i32
                }
            } else {
                num_text_chars = num_text_chars + 1i32
            }
        } /*right_brace */
        sp_end = sp_xptr1;
        while pool_ptr + sp_brace_level + sp_end - sp_ptr > pool_size {
            pool_overflow();
        }
        if pop_lit2 >= cmd_str_ptr {
            pool_ptr = sp_end
        } else {
            while sp_ptr < sp_end {
                *str_pool.offset(pool_ptr as isize) = *str_pool.offset(sp_ptr as isize);
                pool_ptr = pool_ptr + 1i32;
                sp_ptr = sp_ptr + 1i32
            }
        }
        while sp_brace_level > 0i32 {
            *str_pool.offset(pool_ptr as isize) = 125i32 as ASCII_code;
            pool_ptr = pool_ptr + 1i32;
            sp_brace_level = sp_brace_level - 1i32
        }
        push_lit_stk(make_string(), 1i32 as stk_type);
    };
}
unsafe extern "C" fn x_type() {
    if !mess_with_entries {
        bst_cant_mess_with_entries_print();
    } else if *type_list.offset(cite_ptr as isize) == undefined
        || *type_list.offset(cite_ptr as isize) == 0i32
    {
        push_lit_stk(s_null, 1i32 as stk_type);
    } else {
        push_lit_stk(
            *hash_text.offset(*type_list.offset(cite_ptr as isize) as isize),
            1i32 as stk_type,
        );
    };
}
unsafe extern "C" fn x_warning() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
    } else {
        puts_log(b"Warning--\x00" as *const u8 as *const i8);
        print_lit(pop_lit1, pop_typ1);
        mark_warning();
    };
}
unsafe extern "C" fn x_width() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
        push_lit_stk(0i32, 0i32 as stk_type);
    } else {
        ex_buf_length = 0i32;
        add_buf_pool(pop_lit1);
        string_width = 0i32;
        brace_level = 0i32;
        ex_buf_ptr = 0i32;
        while ex_buf_ptr < ex_buf_length {
            if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                /*left_brace */
                brace_level = brace_level + 1i32;
                if brace_level == 1i32 && ex_buf_ptr + 1i32 < ex_buf_length {
                    if *ex_buf.offset((ex_buf_ptr + 1i32) as isize) as i32 == 92i32 {
                        /*backslash */
                        /*453: */
                        ex_buf_ptr = ex_buf_ptr + 1i32;
                        while ex_buf_ptr < ex_buf_length && brace_level > 0i32 {
                            ex_buf_ptr = ex_buf_ptr + 1i32;
                            ex_buf_xptr = ex_buf_ptr;
                            while ex_buf_ptr < ex_buf_length
                                && lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] as i32
                                    == 2i32
                            {
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            }
                            if ex_buf_ptr < ex_buf_length && ex_buf_ptr == ex_buf_xptr {
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            } else {
                                control_seq_loc = str_lookup(
                                    ex_buf,
                                    ex_buf_xptr,
                                    ex_buf_ptr - ex_buf_xptr,
                                    14i32 as str_ilk,
                                    false,
                                );
                                if hash_found {
                                    /*454: */
                                    match *ilk_info.offset(control_seq_loc as isize) {
                                        12 => string_width = string_width + 500i32,
                                        4 => string_width = string_width + 722i32,
                                        2 => string_width = string_width + 778i32,
                                        5 => string_width = string_width + 903i32,
                                        3 => string_width = string_width + 1014i32,
                                        _ => {
                                            string_width = string_width
                                                + char_width
                                                    [*ex_buf.offset(ex_buf_xptr as isize) as usize]
                                        }
                                    }
                                }
                            }
                            while ex_buf_ptr < ex_buf_length
                                && lex_class[*ex_buf.offset(ex_buf_ptr as isize) as usize] as i32
                                    == 1i32
                            {
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            }
                            while ex_buf_ptr < ex_buf_length
                                && brace_level > 0i32
                                && *ex_buf.offset(ex_buf_ptr as isize) as i32 != 92i32
                            {
                                if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                                    /*right_brace */
                                    brace_level = brace_level - 1i32
                                } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 123i32 {
                                    /*left_brace */
                                    brace_level = brace_level + 1i32
                                } else {
                                    string_width = string_width
                                        + char_width[*ex_buf.offset(ex_buf_ptr as isize) as usize]
                                }
                                ex_buf_ptr = ex_buf_ptr + 1i32
                            }
                        }
                        ex_buf_ptr = ex_buf_ptr - 1i32
                    } else {
                        string_width = string_width + char_width[123]
                    }
                } else {
                    string_width = string_width + char_width[123]
                }
            } else if *ex_buf.offset(ex_buf_ptr as isize) as i32 == 125i32 {
                /*right_brace */
                decr_brace_level(pop_lit1);
                string_width = string_width + char_width[125]
            } else {
                string_width =
                    string_width + char_width[*ex_buf.offset(ex_buf_ptr as isize) as usize]
            }
            ex_buf_ptr = ex_buf_ptr + 1i32
        }
        check_brace_level(pop_lit1);
        push_lit_stk(string_width, 0i32 as stk_type);
    };
}
unsafe extern "C" fn x_write() {
    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
    if pop_typ1 as i32 != 1i32 {
        /*stk_str */
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1i32 as stk_type);
    } else {
        add_out_pool(pop_lit1);
    };
}
unsafe extern "C" fn execute_fn(mut ex_fn_loc: hash_loc) {
    let mut current_block: u64;
    let mut r_pop_lt1: i32 = 0;
    let mut r_pop_lt2: i32 = 0;
    let mut r_pop_tp1: stk_type = 0;
    let mut r_pop_tp2: stk_type = 0;
    let mut wiz_ptr: wiz_fn_loc = 0;
    match *fn_type.offset(ex_fn_loc as isize) as i32 {
        0 => {
            match *ilk_info.offset(ex_fn_loc as isize) {
                0 => {
                    current_block = 3427267834250323188;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                /*stk_fn */
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                /*stk_fn */
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        /*stk_int */
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                /*stk_fn */
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                /*stk_fn */
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                /*stk_int */
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                1 => {
                    current_block = 8506478340253986099;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                2 => {
                    current_block = 2992643050629887313;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                3 => {
                    current_block = 3333486971056105332;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                4 => {
                    current_block = 9486769047678124609;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                5 => {
                    current_block = 751934050883067221;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                6 => {
                    current_block = 11311982278797531854;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                7 => {
                    current_block = 17093842530523746;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                8 => {
                    current_block = 12003026128998772082;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                9 => {
                    current_block = 11401095418043589429;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                10 => {
                    current_block = 14433305390625741996;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                11 => {
                    current_block = 11821275415581843219;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                12 => {
                    current_block = 2586387813362916675;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                13 => {
                    current_block = 1299544639425101402;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                14 => {
                    current_block = 12456603346645215998;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                15 => {
                    current_block = 7451931000317828687;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                16 => {
                    current_block = 8655676648363273062;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                17 => {
                    current_block = 4310494265205845711;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                18 => {
                    current_block = 18342201684529422979;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                19 => {
                    current_block = 14095471398735929972;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                20 => {
                    current_block = 1788254067469565360;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                21 => {
                    current_block = 987738563414658848;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                22 => {
                    current_block = 9872068022390718344;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                23 => {
                    current_block = 8468081085890054388;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                24 => {
                    current_block = 17805198275128379845;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                25 => {}
                26 => {
                    current_block = 12179545346928503758;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                27 => {
                    current_block = 227719186661713671;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                28 => {
                    current_block = 6386094465163296590;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                29 => {
                    current_block = 7298725476856358922;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                30 => {
                    current_block = 14071960002833054982;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                31 => {
                    current_block = 8412464758337420148;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                32 => {
                    current_block = 5003375028251918140;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                33 => {
                    current_block = 14559770770887801255;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                34 => {
                    current_block = 9705665520141849625;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                35 => {
                    current_block = 17353911828636475972;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                36 => {
                    current_block = 16007871220680826792;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
                _ => {
                    current_block = 7817847338202672115;
                    match current_block {
                        16007871220680826792 => {
                            x_write();
                        }
                        9705665520141849625 => {
                            pop_lit_stk(&mut r_pop_lt1, &mut r_pop_tp1);
                            pop_lit_stk(&mut r_pop_lt2, &mut r_pop_tp2);
                            if r_pop_tp1 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2i32 as stk_type);
                            } else if r_pop_tp2 as i32 != 2i32 {
                                print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2i32 as stk_type);
                            } else {
                                loop {
                                    execute_fn(r_pop_lt2);
                                    pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                                    if pop_typ1 as i32 != 0i32 {
                                        print_wrong_stk_lit(pop_lit1, pop_typ1, 0i32 as stk_type);
                                        break;
                                    } else {
                                        if !(pop_lit1 > 0i32) {
                                            break;
                                        }
                                        execute_fn(r_pop_lt1);
                                    }
                                }
                            }
                        }
                        3427267834250323188 => {
                            x_equals();
                        }
                        8506478340253986099 => {
                            x_greater_than();
                        }
                        2992643050629887313 => {
                            x_less_than();
                        }
                        3333486971056105332 => {
                            x_plus();
                        }
                        9486769047678124609 => {
                            x_minus();
                        }
                        751934050883067221 => {
                            x_concatenate();
                        }
                        11311982278797531854 => {
                            x_gets();
                        }
                        17093842530523746 => {
                            x_add_period();
                        }
                        12003026128998772082 => {
                            if !mess_with_entries {
                                bst_cant_mess_with_entries_print();
                            } else if *type_list.offset(cite_ptr as isize) == undefined {
                                execute_fn(b_default);
                            } else if !(*type_list.offset(cite_ptr as isize) == 0i32) {
                                execute_fn(*type_list.offset(cite_ptr as isize));
                            }
                        }
                        11401095418043589429 => {
                            x_change_case();
                        }
                        14433305390625741996 => {
                            x_chr_to_int();
                        }
                        11821275415581843219 => {
                            x_cite();
                        }
                        2586387813362916675 => {
                            x_duplicate();
                        }
                        1299544639425101402 => {
                            x_empty();
                        }
                        12456603346645215998 => {
                            x_format_name();
                        }
                        7451931000317828687 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                            pop_lit_stk(&mut pop_lit2, &mut pop_typ2);
                            pop_lit_stk(&mut pop_lit3, &mut pop_typ3);
                            if pop_typ1 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 2i32 as stk_type);
                            } else if pop_typ2 as i32 != 2i32 {
                                print_wrong_stk_lit(pop_lit2, pop_typ2, 2i32 as stk_type);
                            } else if pop_typ3 as i32 != 0i32 {
                                print_wrong_stk_lit(pop_lit3, pop_typ3, 0i32 as stk_type);
                            } else if pop_lit3 > 0i32 {
                                execute_fn(pop_lit2);
                            } else {
                                execute_fn(pop_lit1);
                            }
                        }
                        8655676648363273062 => {
                            x_int_to_chr();
                        }
                        4310494265205845711 => {
                            x_int_to_str();
                        }
                        18342201684529422979 => {
                            x_missing();
                        }
                        14095471398735929972 => {
                            output_bbl_line();
                        }
                        1788254067469565360 => {
                            x_num_names();
                        }
                        987738563414658848 => {
                            pop_lit_stk(&mut pop_lit1, &mut pop_typ1);
                        }
                        9872068022390718344 => {
                            x_preamble();
                        }
                        8468081085890054388 => {
                            x_purify();
                        }
                        17805198275128379845 => {
                            x_quote();
                        }
                        12179545346928503758 => {
                            pop_whole_stack();
                        }
                        227719186661713671 => {
                            x_substring();
                        }
                        6386094465163296590 => {
                            x_swap();
                        }
                        7298725476856358922 => {
                            x_text_length();
                        }
                        14071960002833054982 => {
                            x_text_prefix();
                        }
                        8412464758337420148 => {
                            pop_top_and_print();
                        }
                        5003375028251918140 => {
                            x_type();
                        }
                        14559770770887801255 => {
                            x_warning();
                        }
                        17353911828636475972 => {
                            x_width();
                        }
                        _ => {
                            puts_log(b"Unknown built-in function\x00" as *const u8 as *const i8);
                            print_confusion();
                            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                        }
                    }
                }
            }
        }
        1 => {
            wiz_ptr = *ilk_info.offset(ex_fn_loc as isize);
            while *wiz_functions.offset(wiz_ptr as isize) != end_of_def {
                if *wiz_functions.offset(wiz_ptr as isize) != 1i32 - 1i32 {
                    execute_fn(*wiz_functions.offset(wiz_ptr as isize));
                } else {
                    wiz_ptr = wiz_ptr + 1i32;
                    push_lit_stk(*wiz_functions.offset(wiz_ptr as isize), 2i32 as stk_type);
                }
                wiz_ptr = wiz_ptr + 1i32
            }
        }
        2 => {
            push_lit_stk(*ilk_info.offset(ex_fn_loc as isize), 0i32 as stk_type);
        }
        3 => {
            push_lit_stk(*hash_text.offset(ex_fn_loc as isize), 1i32 as stk_type);
        }
        4 => {
            if !mess_with_entries {
                bst_cant_mess_with_entries_print();
            } else {
                field_ptr = cite_ptr * num_fields + *ilk_info.offset(ex_fn_loc as isize);
                if field_ptr >= max_fields {
                    puts_log(b"field_info index is out of range\x00" as *const u8 as *const i8);
                    print_confusion();
                    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                }
                if *field_info.offset(field_ptr as isize) == 0i32 {
                    /*missing */
                    push_lit_stk(*hash_text.offset(ex_fn_loc as isize), 3i32 as stk_type);
                } else {
                    push_lit_stk(*field_info.offset(field_ptr as isize), 1i32 as stk_type);
                }
            }
        }
        5 => {
            if !mess_with_entries {
                bst_cant_mess_with_entries_print();
            } else {
                push_lit_stk(
                    *entry_ints.offset(
                        (cite_ptr * num_ent_ints + *ilk_info.offset(ex_fn_loc as isize)) as isize,
                    ),
                    0i32 as stk_type,
                );
            }
        }
        6 => {
            if !mess_with_entries {
                bst_cant_mess_with_entries_print();
            } else {
                str_ent_ptr = cite_ptr * num_ent_strs + *ilk_info.offset(ex_fn_loc as isize);
                ex_buf_ptr = 0i32;
                while *entry_strs
                    .offset((str_ent_ptr * (ent_str_size + 1i32) + ex_buf_ptr) as isize)
                    as i32
                    != 127i32
                {
                    /*end_of_string */
                    *ex_buf.offset(ex_buf_ptr as isize) = *entry_strs
                        .offset((str_ent_ptr * (ent_str_size + 1i32) + ex_buf_ptr) as isize); /* strip off the (assumed) ".aux" for subsequent futzing */
                    ex_buf_ptr = ex_buf_ptr + 1i32
                }
                ex_buf_length = ex_buf_ptr;
                add_pool_buf_and_push();
            }
        }
        7 => {
            push_lit_stk(*ilk_info.offset(ex_fn_loc as isize), 0i32 as stk_type);
        }
        8 => {
            str_glb_ptr = *ilk_info.offset(ex_fn_loc as isize);
            if *glb_str_ptr.offset(str_glb_ptr as isize) > 0i32 {
                push_lit_stk(*glb_str_ptr.offset(str_glb_ptr as isize), 1i32 as stk_type);
            } else {
                while pool_ptr + *glb_str_end.offset(str_glb_ptr as isize) > pool_size {
                    pool_overflow();
                }
                glob_chr_ptr = 0i32;
                while glob_chr_ptr < *glb_str_end.offset(str_glb_ptr as isize) {
                    *str_pool.offset(pool_ptr as isize) = *global_strs
                        .offset((str_glb_ptr * (glob_str_size + 1i32) + glob_chr_ptr) as isize);
                    pool_ptr = pool_ptr + 1i32;
                    glob_chr_ptr = glob_chr_ptr + 1i32
                }
                push_lit_stk(make_string(), 1i32 as stk_type);
            }
        }
        _ => {
            unknwn_function_class_confusion();
        }
    };
}
unsafe extern "C" fn get_the_top_level_aux_file_name(mut aux_file_name: *const i8) -> i32 {
    name_of_file = xmalloc(
        strlen(aux_file_name)
            .wrapping_add(1i32 as u64)
            .wrapping_add(1i32 as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    strcpy(name_of_file as *mut i8, aux_file_name);
    aux_name_length = strlen(name_of_file as *mut i8) as i32;
    aux_name_length -= 4i32;
    name_length = aux_name_length;
    /* this code used to auto-add the .aux extension if needed; we don't */
    aux_ptr = 0i32; // preserve pascal-style string semantics
    aux_file[aux_ptr as usize] = peekable_open(name_of_file as *mut i8, TTIF_TEX);
    if aux_file[aux_ptr as usize].is_null() {
        sam_wrong_file_name_print();
        return 1i32;
    }
    add_extension(s_log_extension);
    log_file = ttstub_output_open(name_of_file as *mut i8, 0i32);
    if log_file.is_null() {
        sam_wrong_file_name_print();
        return 1i32;
    }
    name_length = aux_name_length;
    add_extension(s_bbl_extension);
    bbl_file = ttstub_output_open(name_of_file as *mut i8, 0i32);
    if bbl_file.is_null() {
        sam_wrong_file_name_print();
        return 1i32;
    }
    name_length = aux_name_length;
    add_extension(s_aux_extension);
    name_ptr = 0i32;
    while name_ptr < name_length {
        *buffer.offset((name_ptr + 1i32) as isize) = *name_of_file.offset(name_ptr as isize);
        name_ptr = name_ptr + 1i32
    }
    top_lev_str = *hash_text
        .offset(str_lookup(buffer, 1i32, aux_name_length, 0i32 as str_ilk, true) as isize);
    aux_list[aux_ptr as usize] =
        *hash_text.offset(str_lookup(buffer, 1i32, name_length, 3i32 as str_ilk, true) as isize);
    if hash_found {
        puts_log(b"Already encountered auxiliary file\x00" as *const u8 as *const i8);
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    aux_ln_stack[aux_ptr as usize] = 0i32;
    0i32
}
unsafe extern "C" fn aux_bib_data_command() {
    if bib_seen {
        aux_err_illegal_another_print(0i32);
        aux_err_print();
        return;
    }
    bib_seen = true;
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        buf_ptr2 = buf_ptr2 + 1i32;
        if !scan2_white(125i32 as ASCII_code, 44i32 as ASCII_code) {
            aux_err_no_right_brace_print();
            aux_err_print();
            return;
        }
        if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32 {
            /*white_space */
            aux_err_white_space_in_argument_print();
            aux_err_print();
            return;
        }
        if last > buf_ptr2 + 1i32 && *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
            aux_err_stuff_after_right_brace_print();
            aux_err_print();
            return;
        }
        if bib_ptr == max_bib_files {
            bib_list = xrealloc(
                bib_list as *mut libc::c_void,
                ((max_bib_files + 20i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
            ) as *mut str_number;
            bib_file = xrealloc(
                bib_file as *mut libc::c_void,
                ((max_bib_files + 20i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<*mut peekable_input_t>() as u64),
            ) as *mut *mut peekable_input_t;
            s_preamble = xrealloc(
                s_preamble as *mut libc::c_void,
                ((max_bib_files + 20i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
            ) as *mut str_number;
            max_bib_files = max_bib_files + 20i32
        }
        *bib_list.offset(bib_ptr as isize) = *hash_text.offset(str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            6i32 as str_ilk,
            true,
        ) as isize);
        if hash_found {
            puts_log(b"This database file appears more than once: \x00" as *const u8 as *const i8);
            print_bib_name();
            aux_err_print();
            return;
        }
        start_name(*bib_list.offset(bib_ptr as isize));
        let ref mut fresh9 = *bib_file.offset(bib_ptr as isize);
        *fresh9 = peekable_open(name_of_file as *mut i8, TTIF_BIB);
        if (*fresh9).is_null() {
            puts_log(b"I couldn\'t open database file \x00" as *const u8 as *const i8);
            print_bib_name();
            aux_err_print();
            return;
        }
        bib_ptr = bib_ptr + 1i32
    }
}
unsafe extern "C" fn aux_bib_style_command() {
    if bst_seen {
        aux_err_illegal_another_print(1i32);
        aux_err_print();
        return;
    }
    bst_seen = true;
    buf_ptr2 = buf_ptr2 + 1i32;
    if !scan1_white(125i32 as ASCII_code) {
        aux_err_no_right_brace_print();
        aux_err_print();
        return;
    }
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32 {
        /*white_space */
        aux_err_white_space_in_argument_print();
        aux_err_print();
        return;
    }
    if last > buf_ptr2 + 1i32 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print();
        return;
    }
    bst_str =
        *hash_text.offset(
            str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 5i32 as str_ilk, true) as isize,
        );
    if hash_found {
        puts_log(b"Already encountered style file\x00" as *const u8 as *const i8);
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    start_name(bst_str);
    bst_file = peekable_open(name_of_file as *mut i8, TTIF_BST);
    if bst_file.is_null() {
        puts_log(b"I couldn\'t open style file \x00" as *const u8 as *const i8);
        print_bst_name();
        bst_str = 0i32;
        aux_err_print();
        return;
    }
    if verbose != 0 {
        puts_log(b"The style file: \x00" as *const u8 as *const i8);
        print_bst_name();
    } else {
        ttstub_puts(log_file, b"The style file: \x00" as *const u8 as *const i8);
        log_pr_bst_name();
    };
}
unsafe extern "C" fn aux_citation_command() {
    citation_seen = true;
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        let mut current_block_56: u64;
        /*right_brace */
        buf_ptr2 = buf_ptr2 + 1i32;
        if !scan2_white(125i32 as ASCII_code, 44i32 as ASCII_code) {
            aux_err_no_right_brace_print();
            aux_err_print();
            return;
        }
        if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32 {
            /*white_space */
            aux_err_white_space_in_argument_print();
            aux_err_print();
            return;
        }
        if last > buf_ptr2 + 1i32 && *buffer.offset(buf_ptr2 as isize) as i32 == 125i32 {
            aux_err_stuff_after_right_brace_print();
            aux_err_print();
            return;
        }
        if buf_ptr2 - buf_ptr1 == 1i32 {
            if *buffer.offset(buf_ptr1 as isize) as i32 == 42i32 {
                /*star */
                if all_entries {
                    puts_log(
                        b"Multiple inclusions of entire database\n\x00" as *const u8 as *const i8,
                    ); /*137: */
                    aux_err_print();
                    return;
                } else {
                    all_entries = true;
                    all_marker = cite_ptr
                }
                current_block_56 = 10930818133215224067;
            } else {
                current_block_56 = 15925075030174552612;
            }
        } else {
            current_block_56 = 15925075030174552612;
        }
        match current_block_56 {
            15925075030174552612 => {
                tmp_ptr = buf_ptr1;
                while tmp_ptr < buf_ptr2 {
                    *ex_buf.offset(tmp_ptr as isize) = *buffer.offset(tmp_ptr as isize);
                    tmp_ptr = tmp_ptr + 1i32
                }
                lower_case(ex_buf, buf_ptr1, buf_ptr2 - buf_ptr1);
                lc_cite_loc = str_lookup(
                    ex_buf,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    10i32 as str_ilk,
                    true,
                );
                if hash_found {
                    dummy_loc = str_lookup(
                        buffer,
                        buf_ptr1,
                        buf_ptr2 - buf_ptr1,
                        9i32 as str_ilk,
                        false,
                    );
                    if !hash_found {
                        puts_log(
                            b"Case mismatch error between cite keys \x00" as *const u8 as *const i8,
                        );
                        print_a_token();
                        puts_log(b" and \x00" as *const u8 as *const i8);
                        print_a_pool_str(*cite_list.offset(
                            *ilk_info.offset(*ilk_info.offset(lc_cite_loc as isize) as isize)
                                as isize,
                        ));
                        putc_log('\n' as i32);
                        aux_err_print();
                        return;
                    }
                } else {
                    cite_loc =
                        str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 9i32 as str_ilk, true);
                    if hash_found {
                        hash_cite_confusion();
                    }
                    check_cite_overflow(cite_ptr);
                    *cite_list.offset(cite_ptr as isize) = *hash_text.offset(cite_loc as isize);
                    *ilk_info.offset(cite_loc as isize) = cite_ptr;
                    *ilk_info.offset(lc_cite_loc as isize) = cite_loc;
                    cite_ptr = cite_ptr + 1i32
                }
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn aux_input_command() {
    let mut aux_extension_ok: bool = false;
    buf_ptr2 = buf_ptr2 + 1i32;
    if !scan1_white(125i32 as ASCII_code) {
        aux_err_no_right_brace_print();
        aux_err_print();
        return;
    }
    if lex_class[*buffer.offset(buf_ptr2 as isize) as usize] as i32 == 1i32 {
        /*white_space */
        aux_err_white_space_in_argument_print();
        aux_err_print();
        return;
    }
    if last > buf_ptr2 + 1i32 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print();
        return;
    }
    aux_ptr = aux_ptr + 1i32;
    if aux_ptr == 20i32 {
        print_a_token();
        puts_log(b": \x00" as *const u8 as *const i8);
        print_overflow();
        printf_log(
            b"auxiliary file depth %ld\n\x00" as *const u8 as *const i8,
            20i32 as i64,
        );
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    aux_extension_ok = true;
    if buf_ptr2 - buf_ptr1
        < *str_start.offset((s_aux_extension + 1i32) as isize)
            - *str_start.offset(s_aux_extension as isize)
    {
        aux_extension_ok = false
    } else if !str_eq_buf(
        s_aux_extension,
        buffer,
        buf_ptr2
            - (*str_start.offset((s_aux_extension + 1i32) as isize)
                - *str_start.offset(s_aux_extension as isize)),
        *str_start.offset((s_aux_extension + 1i32) as isize)
            - *str_start.offset(s_aux_extension as isize),
    ) {
        aux_extension_ok = false
    }
    if !aux_extension_ok {
        print_a_token();
        puts_log(b" has a wrong extension\x00" as *const u8 as *const i8);
        aux_ptr = aux_ptr - 1i32;
        aux_err_print();
        return;
    }
    aux_list[aux_ptr as usize] =
        *hash_text.offset(
            str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 3i32 as str_ilk, true) as isize,
        );
    if hash_found {
        puts_log(b"Already encountered file \x00" as *const u8 as *const i8);
        print_aux_name();
        aux_ptr = aux_ptr - 1i32;
        aux_err_print();
        return;
    }
    start_name(aux_list[aux_ptr as usize]);
    name_ptr = name_length;
    *name_of_file.offset(name_ptr as isize) = 0i32 as ASCII_code;
    aux_file[aux_ptr as usize] = peekable_open(name_of_file as *mut i8, TTIF_TEX);
    if aux_file[aux_ptr as usize].is_null() {
        puts_log(b"I couldn\'t open auxiliary file \x00" as *const u8 as *const i8);
        print_aux_name();
        aux_ptr = aux_ptr - 1i32;
        aux_err_print();
        return;
    }
    printf_log(
        b"A level-%ld auxiliary file: \x00" as *const u8 as *const i8,
        aux_ptr as i64,
    );
    print_aux_name();
    aux_ln_stack[aux_ptr as usize] = 0i32;
}
unsafe extern "C" fn pop_the_aux_stack() -> i32 {
    peekable_close(aux_file[aux_ptr as usize]);
    aux_file[aux_ptr as usize] = 0 as *mut peekable_input_t;
    if aux_ptr == 0i32 {
        return 1i32;
    }
    aux_ptr -= 1;
    0i32
}
unsafe extern "C" fn get_aux_command_and_process() {
    buf_ptr2 = 0i32;
    if !scan1(123i32 as ASCII_code) {
        return;
    }
    command_num = *ilk_info.offset(str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        2i32 as str_ilk,
        false,
    ) as isize);
    if hash_found {
        match command_num {
            0 => {
                aux_bib_data_command();
            }
            1 => {
                aux_bib_style_command();
            }
            2 => {
                aux_citation_command();
            }
            3 => {
                aux_input_command();
            }
            _ => {
                puts_log(b"Unknown auxiliary-file command\x00" as *const u8 as *const i8);
                print_confusion();
                longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
            }
        }
    };
}
unsafe extern "C" fn last_check_for_aux_errors() {
    num_cites = cite_ptr;
    num_bib_files = bib_ptr;
    if !citation_seen {
        aux_end1_err_print();
        puts_log(b"\\citation commands\x00" as *const u8 as *const i8);
        aux_end2_err_print();
    } else if num_cites == 0i32 && !all_entries {
        aux_end1_err_print();
        puts_log(b"cite keys\x00" as *const u8 as *const i8);
        aux_end2_err_print();
    }
    if !bib_seen {
        aux_end1_err_print();
        puts_log(b"\\bibdata command\x00" as *const u8 as *const i8);
        aux_end2_err_print();
    } else if num_bib_files == 0i32 {
        aux_end1_err_print();
        puts_log(b"database files\x00" as *const u8 as *const i8);
        aux_end2_err_print();
    }
    if !bst_seen {
        aux_end1_err_print();
        puts_log(b"\\bibstyle command\x00" as *const u8 as *const i8);
        aux_end2_err_print();
    } else if bst_str == 0i32 {
        aux_end1_err_print();
        puts_log(b"style file\x00" as *const u8 as *const i8);
        aux_end2_err_print();
    };
}
unsafe extern "C" fn bst_entry_command() {
    if entry_seen {
        puts_log(b"Illegal, another entry command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    entry_seen = true;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        scan_identifier(
            125i32 as ASCII_code,
            37i32 as ASCII_code,
            37i32 as ASCII_code,
        ); /*field */
        if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
        } else {
            bst_id_print();
            puts_log(b"entry\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = 4i32 as fn_class;
        *ilk_info.offset(fn_loc as isize) = num_fields;
        num_fields = num_fields + 1i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            puts_log(b"entry\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if num_fields == num_pre_defined_fields {
        puts_log(b"Warning--I didn\'t find any fields\x00" as *const u8 as *const i8);
        bst_warn_print();
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        scan_identifier(
            125i32 as ASCII_code,
            37i32 as ASCII_code,
            37i32 as ASCII_code,
        ); /*int_entry_var */
        if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
        } else {
            bst_id_print();
            puts_log(b"entry\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = 5i32 as fn_class;
        *ilk_info.offset(fn_loc as isize) = num_ent_ints;
        num_ent_ints = num_ent_ints + 1i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            puts_log(b"entry\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"entry\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        scan_identifier(
            125i32 as ASCII_code,
            37i32 as ASCII_code,
            37i32 as ASCII_code,
        ); /*str_entry_var */
        if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
        } else {
            bst_id_print();
            puts_log(b"entry\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = 6i32 as fn_class;
        *ilk_info.offset(fn_loc as isize) = num_ent_strs;
        num_ent_strs = num_ent_strs + 1i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            puts_log(b"entry\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 = buf_ptr2 + 1i32;
}
unsafe extern "C" fn bad_argument_token() -> bool {
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    fn_loc = str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        11i32 as str_ilk,
        false,
    );
    if !hash_found {
        print_a_token();
        puts_log(b" is an unknown function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return true;
    } else {
        if *fn_type.offset(fn_loc as isize) as i32 != 0i32
            && *fn_type.offset(fn_loc as isize) as i32 != 1i32
        {
            print_a_token();
            puts_log(b" has bad function type \x00" as *const u8 as *const i8);
            print_fn_class(fn_loc);
            bst_err_print_and_look_for_blank_line();
            return true;
        }
    }
    false
}
unsafe extern "C" fn bst_execute_command() {
    if !read_seen {
        puts_log(b"Illegal, execute command before read command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"execute\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"execute\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"execute\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    scan_identifier(
        125i32 as ASCII_code,
        37i32 as ASCII_code,
        37i32 as ASCII_code,
    );
    if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
    } else {
        bst_id_print();
        puts_log(b"execute\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if bad_argument_token() {
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"execute\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        puts_log(b"execute\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    init_command_execution();
    mess_with_entries = false;
    execute_fn(fn_loc);
    check_command_execution();
}
unsafe extern "C" fn bst_function_command() {
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print(); /*wiz_defined */
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    scan_identifier(
        125i32 as ASCII_code,
        37i32 as ASCII_code,
        37i32 as ASCII_code,
    );
    if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
    } else {
        bst_id_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    wiz_loc = str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        11i32 as str_ilk,
        true,
    );
    if hash_found {
        already_seen_function_print(wiz_loc);
        return;
    }
    *fn_type.offset(wiz_loc as isize) = 1i32 as fn_class;
    if *hash_text.offset(wiz_loc as isize) == s_default {
        b_default = wiz_loc
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"function\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    scan_fn_def(wiz_loc);
}
unsafe extern "C" fn bst_integers_command() {
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"integers\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"integers\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"integers\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        scan_identifier(
            125i32 as ASCII_code,
            37i32 as ASCII_code,
            37i32 as ASCII_code,
        ); /*int_global_var */
        if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
        } else {
            bst_id_print();
            puts_log(b"integers\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = 7i32 as fn_class;
        *ilk_info.offset(fn_loc as isize) = 0i32;
        if !eat_bst_white_space() {
            eat_bst_print();
            puts_log(b"integers\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 = buf_ptr2 + 1i32;
}
unsafe extern "C" fn bst_iterate_command() {
    if !read_seen {
        puts_log(b"Illegal, iterate command before read command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"iterate\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"iterate\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"iterate\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    scan_identifier(
        125i32 as ASCII_code,
        37i32 as ASCII_code,
        37i32 as ASCII_code,
    );
    if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
    } else {
        bst_id_print();
        puts_log(b"iterate\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if bad_argument_token() {
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"iterate\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        puts_log(b"iterate\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    init_command_execution();
    mess_with_entries = true;
    sort_cite_ptr = 0i32;
    while sort_cite_ptr < num_cites {
        cite_ptr = *cite_info.offset(sort_cite_ptr as isize);
        execute_fn(fn_loc);
        check_command_execution();
        sort_cite_ptr = sort_cite_ptr + 1i32
    }
}
unsafe extern "C" fn bst_macro_command() {
    if read_seen {
        puts_log(b"Illegal, macro command after read command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    scan_identifier(
        125i32 as ASCII_code,
        37i32 as ASCII_code,
        37i32 as ASCII_code,
    );
    if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
    } else {
        bst_id_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    macro_name_loc = str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        13i32 as str_ilk,
        true,
    );
    if hash_found {
        print_a_token();
        puts_log(b" is already defined as a macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    *ilk_info.offset(macro_name_loc as isize) = *hash_text.offset(macro_name_loc as isize);
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 34i32 {
        /*double_quote */
        puts_log(b"A macro definition must be \"-delimited\x00" as *const u8 as *const i8); /*str_literal */
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !scan1(34i32 as ASCII_code) {
        puts_log(b"There\'s no `\"\' to end macro definition\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    macro_def_loc = str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 0i32 as str_ilk, true);
    *fn_type.offset(macro_def_loc as isize) = 3i32 as fn_class;
    *ilk_info.offset(macro_name_loc as isize) = *hash_text.offset(macro_def_loc as isize);
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        puts_log(b"macro\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
}
unsafe extern "C" fn get_bib_command_or_entry_and_process() {
    let mut current_block: u64;
    at_bib_command = false;
    while !scan1(64i32 as ASCII_code) {
        if !input_ln(*bib_file.offset(bib_ptr as isize)) {
            return;
        }
        bib_line_num = bib_line_num + 1i32;
        buf_ptr2 = 0i32
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 64i32 {
        /*at_sign */
        puts_log(b"An \"@\" disappeared\x00" as *const u8 as *const i8);
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    scan_identifier(
        123i32 as ASCII_code,
        40i32 as ASCII_code,
        40i32 as ASCII_code,
    );
    if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
    } else {
        bib_id_print();
        puts_log(b"an entry type\x00" as *const u8 as *const i8);
        bib_err_print();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    command_num = *ilk_info.offset(str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        12i32 as str_ilk,
        false,
    ) as isize);
    if hash_found {
        /*240: */
        at_bib_command = true;
        match command_num {
            0 => return,
            1 => {
                if preamble_ptr == max_bib_files {
                    bib_list = xrealloc(
                        bib_list as *mut libc::c_void,
                        ((max_bib_files + 20i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
                    ) as *mut str_number;
                    bib_file = xrealloc(
                        bib_file as *mut libc::c_void,
                        ((max_bib_files + 20i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<*mut peekable_input_t>() as u64),
                    ) as *mut *mut peekable_input_t;
                    s_preamble = xrealloc(
                        s_preamble as *mut libc::c_void,
                        ((max_bib_files + 20i32 + 1i32) as u64)
                            .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
                    ) as *mut str_number;
                    max_bib_files = max_bib_files + 20i32
                }
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                    /*left_brace */
                    right_outer_delim = 125i32 as ASCII_code
                } else if *buffer.offset(buf_ptr2 as isize) as i32 == 40i32 {
                    /*right_brace */
                    /*left_paren */
                    right_outer_delim = 41i32 as ASCII_code
                } else {
                    bib_one_of_two_print(123i32 as ASCII_code, 40i32 as ASCII_code); /*right_paren */
                    return;
                }
                buf_ptr2 = buf_ptr2 + 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                store_field = true;
                if !scan_and_store_the_field_value_and_eat_white() {
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 != right_outer_delim as i32 {
                    printf_log(
                        b"Missing \"%c\" in preamble command\x00" as *const u8 as *const i8,
                        right_outer_delim as i32,
                    );
                    bib_err_print();
                    return;
                }
                buf_ptr2 = buf_ptr2 + 1i32;
                return;
            }
            2 => {
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
                    /*left_brace */
                    right_outer_delim = 125i32 as ASCII_code
                } else if *buffer.offset(buf_ptr2 as isize) as i32 == 40i32 {
                    /*right_brace */
                    /*left_paren */
                    right_outer_delim = 41i32 as ASCII_code
                } else {
                    bib_one_of_two_print(123i32 as ASCII_code, 40i32 as ASCII_code); /*right_paren */
                    return;
                }
                buf_ptr2 = buf_ptr2 + 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                scan_identifier(
                    61i32 as ASCII_code,
                    61i32 as ASCII_code,
                    61i32 as ASCII_code,
                );
                if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
                } else {
                    bib_id_print();
                    puts_log(b"a string name\x00" as *const u8 as *const i8);
                    bib_err_print();
                    return;
                }
                lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
                cur_macro_loc = str_lookup(
                    buffer,
                    buf_ptr1,
                    buf_ptr2 - buf_ptr1,
                    13i32 as str_ilk,
                    true,
                );
                *ilk_info.offset(cur_macro_loc as isize) =
                    *hash_text.offset(cur_macro_loc as isize);
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 != 61i32 {
                    /*equals_sign */
                    bib_equals_sign_print();
                    return;
                }
                buf_ptr2 = buf_ptr2 + 1i32;
                if !eat_bib_white_space() {
                    eat_bib_print();
                    return;
                }
                store_field = true;
                if !scan_and_store_the_field_value_and_eat_white() {
                    return;
                }
                if *buffer.offset(buf_ptr2 as isize) as i32 != right_outer_delim as i32 {
                    printf_log(
                        b"Missing \"%c\" in string command\x00" as *const u8 as *const i8,
                        right_outer_delim as i32,
                    );
                    bib_err_print();
                    return;
                }
                buf_ptr2 = buf_ptr2 + 1i32;
                return;
            }
            _ => {
                bib_cmd_confusion();
            }
        }
    } else {
        entry_type_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            false,
        );
        if !hash_found || *fn_type.offset(entry_type_loc as isize) as i32 != 1i32 {
            type_exists = false
        } else {
            type_exists = true
        }
    }
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 == 123i32 {
        /*left_brace */
        right_outer_delim = 125i32 as ASCII_code
    } else if *buffer.offset(buf_ptr2 as isize) as i32 == 40i32 {
        /*right_brace */
        /*left_paren */
        right_outer_delim = 41i32 as ASCII_code
    } else {
        bib_one_of_two_print(123i32 as ASCII_code, 40i32 as ASCII_code); /*right_paren */
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    if right_outer_delim as i32 == 41i32 {
        /*right_paren */
        scan1_white(44i32 as ASCII_code);
    } else {
        scan2_white(44i32 as ASCII_code, 125i32 as ASCII_code);
    }
    tmp_ptr = buf_ptr1;
    while tmp_ptr < buf_ptr2 {
        *ex_buf.offset(tmp_ptr as isize) = *buffer.offset(tmp_ptr as isize);
        tmp_ptr = tmp_ptr + 1i32
    }
    lower_case(ex_buf, buf_ptr1, buf_ptr2 - buf_ptr1);
    if all_entries {
        lc_cite_loc = str_lookup(
            ex_buf,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            10i32 as str_ilk,
            true,
        )
    } else {
        lc_cite_loc = str_lookup(
            ex_buf,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            10i32 as str_ilk,
            false,
        )
    }
    if hash_found {
        entry_cite_ptr = *ilk_info.offset(*ilk_info.offset(lc_cite_loc as isize) as isize);
        if !all_entries || entry_cite_ptr < all_marker || entry_cite_ptr >= old_num_cites {
            if *type_list.offset(entry_cite_ptr as isize) == 0i32 {
                /*empty */
                if !all_entries && entry_cite_ptr >= old_num_cites {
                    cite_loc =
                        str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 9i32 as str_ilk, true);
                    if !hash_found {
                        *ilk_info.offset(lc_cite_loc as isize) = cite_loc;
                        *ilk_info.offset(cite_loc as isize) = entry_cite_ptr;
                        *cite_list.offset(entry_cite_ptr as isize) =
                            *hash_text.offset(cite_loc as isize);
                        hash_found = true
                    }
                }
                current_block = 12387625063048049585;
            } else {
                current_block = 3813860224257983916;
            }
        } else if !*entry_exists.offset(entry_cite_ptr as isize) {
            ex_buf_ptr = 0i32;
            tmp_ptr = *str_start.offset(*cite_info.offset(entry_cite_ptr as isize) as isize);
            tmp_end_ptr =
                *str_start.offset((*cite_info.offset(entry_cite_ptr as isize) + 1i32) as isize);
            while tmp_ptr < tmp_end_ptr {
                *ex_buf.offset(ex_buf_ptr as isize) = *str_pool.offset(tmp_ptr as isize);
                ex_buf_ptr = ex_buf_ptr + 1i32;
                tmp_ptr = tmp_ptr + 1i32
            }
            lower_case(
                ex_buf,
                0i32,
                *str_start.offset((*cite_info.offset(entry_cite_ptr as isize) + 1i32) as isize)
                    - *str_start.offset(*cite_info.offset(entry_cite_ptr as isize) as isize),
            );
            lc_xcite_loc = str_lookup(
                ex_buf,
                0i32,
                *str_start.offset((*cite_info.offset(entry_cite_ptr as isize) + 1i32) as isize)
                    - *str_start.offset(*cite_info.offset(entry_cite_ptr as isize) as isize),
                10i32 as str_ilk,
                false,
            );
            if !hash_found {
                cite_key_disappeared_confusion();
            }
            if lc_xcite_loc == lc_cite_loc {
                current_block = 12387625063048049585;
            } else {
                current_block = 3813860224257983916;
            }
        } else {
            current_block = 3813860224257983916;
        }
        match current_block {
            12387625063048049585 => {}
            _ => {
                if *type_list.offset(entry_cite_ptr as isize) == 0i32 {
                    /*empty */
                    puts_log(b"The cite list is messed up\x00" as *const u8 as *const i8);
                    print_confusion();
                    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                }
                puts_log(b"Repeated entry\x00" as *const u8 as *const i8);
                bib_err_print();
                return;
            }
        }
    }
    /*first_time_entry */
    store_entry = true;
    if all_entries {
        let mut current_block_216: u64;
        /*273: */
        if hash_found {
            if entry_cite_ptr < all_marker {
                current_block_216 = 17170253997621722914;
            } else {
                *entry_exists.offset(entry_cite_ptr as isize) = true;
                cite_loc = *ilk_info.offset(lc_cite_loc as isize);
                current_block_216 = 763224442071743734;
            }
        } else {
            cite_loc = str_lookup(buffer, buf_ptr1, buf_ptr2 - buf_ptr1, 9i32 as str_ilk, true);
            if hash_found {
                hash_cite_confusion();
            }
            current_block_216 = 763224442071743734;
        }
        match current_block_216 {
            763224442071743734 => {
                entry_cite_ptr = cite_ptr;
                add_database_cite(&mut cite_ptr);
            }
            _ => {}
        }
    } else if !hash_found {
        store_entry = false
    }
    if store_entry {
        /*274: */
        if type_exists {
            *type_list.offset(entry_cite_ptr as isize) = entry_type_loc
        } else {
            *type_list.offset(entry_cite_ptr as isize) = undefined;
            puts_log(b"Warning--entry type for \"\x00" as *const u8 as *const i8);
            print_a_token();
            puts_log(b"\" isn\'t style-file defined\n\x00" as *const u8 as *const i8);
            bib_warn_print();
        }
    }
    if !eat_bib_white_space() {
        eat_bib_print();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != right_outer_delim as i32 {
        if *buffer.offset(buf_ptr2 as isize) as i32 != 44i32 {
            /*comma */
            bib_one_of_two_print(44i32 as ASCII_code, right_outer_delim);
            return;
        }
        buf_ptr2 = buf_ptr2 + 1i32;
        if !eat_bib_white_space() {
            eat_bib_print();
            return;
        }
        if *buffer.offset(buf_ptr2 as isize) as i32 == right_outer_delim as i32 {
            break;
        }
        scan_identifier(
            61i32 as ASCII_code,
            61i32 as ASCII_code,
            61i32 as ASCII_code,
        );
        if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
        } else {
            bib_id_print();
            puts_log(b"a field name\x00" as *const u8 as *const i8);
            bib_err_print();
            return;
        }
        store_field = false;
        if store_entry {
            lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
            field_name_loc = str_lookup(
                buffer,
                buf_ptr1,
                buf_ptr2 - buf_ptr1,
                11i32 as str_ilk,
                false,
            );
            if hash_found {
                if *fn_type.offset(field_name_loc as isize) as i32 == 4i32 {
                    /*field */
                    store_field = true
                }
            }
        }
        if !eat_bib_white_space() {
            eat_bib_print();
            return;
        }
        if *buffer.offset(buf_ptr2 as isize) as i32 != 61i32 {
            /*equals_sign */
            bib_equals_sign_print(); /*missing */
            return;
        } /*empty */
        buf_ptr2 = buf_ptr2 + 1i32; /*any_value */
        if !eat_bib_white_space() {
            eat_bib_print();
            return;
        }
        if !scan_and_store_the_field_value_and_eat_white() {
            return;
        }
    }
    buf_ptr2 = buf_ptr2 + 1i32;
}
unsafe extern "C" fn bst_read_command() {
    if read_seen {
        puts_log(b"Illegal, another read command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    read_seen = true;
    if !entry_seen {
        puts_log(b"Illegal, read command before entry command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    sv_ptr1 = buf_ptr2;
    sv_ptr2 = last;
    tmp_ptr = sv_ptr1;
    while tmp_ptr < sv_ptr2 {
        *sv_buffer.offset(tmp_ptr as isize) = *buffer.offset(tmp_ptr as isize);
        tmp_ptr = tmp_ptr + 1i32
    }
    check_field_overflow(num_fields * num_cites);
    field_ptr = 0i32;
    while field_ptr < max_fields {
        *field_info.offset(field_ptr as isize) = 0i32;
        field_ptr = field_ptr + 1i32
    }
    cite_ptr = 0i32;
    while cite_ptr < max_cites {
        *type_list.offset(cite_ptr as isize) = 0i32;
        *cite_info.offset(cite_ptr as isize) = 0i32;
        cite_ptr = cite_ptr + 1i32
    }
    old_num_cites = num_cites;
    if all_entries {
        cite_ptr = all_marker;
        while cite_ptr < old_num_cites {
            *cite_info.offset(cite_ptr as isize) = *cite_list.offset(cite_ptr as isize);
            *entry_exists.offset(cite_ptr as isize) = false;
            cite_ptr = cite_ptr + 1i32
        }
        cite_ptr = all_marker
    } else {
        cite_ptr = num_cites;
        all_marker = 0i32
        /*any_value */
    }
    read_performed = true;
    bib_ptr = 0i32;
    while bib_ptr < num_bib_files {
        if verbose != 0 {
            printf_log(
                b"Database file #%ld: \x00" as *const u8 as *const i8,
                bib_ptr as i64 + 1i32 as i64,
            );
            print_bib_name();
        } else {
            let mut buf: [i8; 512] = [0; 512];
            snprintf(
                buf.as_mut_ptr(),
                (::std::mem::size_of::<[i8; 512]>() as u64).wrapping_sub(1i32 as u64),
                b"Database file #%ld: \x00" as *const u8 as *const i8,
                bib_ptr as i64 + 1i32 as i64,
            );
            ttstub_output_write(log_file, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
            log_pr_bib_name();
        }
        bib_line_num = 0i32;
        buf_ptr2 = last;
        while !tectonic_eof(*bib_file.offset(bib_ptr as isize)) {
            get_bib_command_or_entry_and_process();
        }
        peekable_close(*bib_file.offset(bib_ptr as isize));
        let ref mut fresh10 = *bib_file.offset(bib_ptr as isize);
        *fresh10 = 0 as *mut peekable_input_t;
        bib_ptr = bib_ptr + 1i32
    }
    reading_completed = true;
    num_cites = cite_ptr;
    num_preamble_strings = preamble_ptr;
    if (num_cites - 1i32) * num_fields + crossref_num >= max_fields {
        puts_log(b"field_info index is out of range\x00" as *const u8 as *const i8);
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        field_ptr = cite_ptr * num_fields + crossref_num;
        if *field_info.offset(field_ptr as isize) != 0i32 {
            /*missing */
            if find_cite_locs_for_this_cite_key(*field_info.offset(field_ptr as isize)) {
                cite_loc = *ilk_info.offset(lc_cite_loc as isize);
                *field_info.offset(field_ptr as isize) = *hash_text.offset(cite_loc as isize);
                cite_parent_ptr = *ilk_info.offset(cite_loc as isize);
                field_ptr = cite_ptr * num_fields + num_pre_defined_fields;
                field_end_ptr = field_ptr - num_pre_defined_fields + num_fields;
                field_parent_ptr = cite_parent_ptr * num_fields + num_pre_defined_fields;
                while field_ptr < field_end_ptr {
                    if *field_info.offset(field_ptr as isize) == 0i32 {
                        /*missing */
                        *field_info.offset(field_ptr as isize) =
                            *field_info.offset(field_parent_ptr as isize)
                    }
                    field_ptr = field_ptr + 1i32;
                    field_parent_ptr = field_parent_ptr + 1i32
                }
            }
        }
        cite_ptr = cite_ptr + 1i32
    }
    if (num_cites - 1i32) * num_fields + crossref_num >= max_fields {
        puts_log(b"field_info index is out of range\x00" as *const u8 as *const i8);
        print_confusion();
        longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        field_ptr = cite_ptr * num_fields + crossref_num;
        if *field_info.offset(field_ptr as isize) != 0i32 {
            /*missing */
            if !find_cite_locs_for_this_cite_key(*field_info.offset(field_ptr as isize)) {
                if cite_hash_found {
                    hash_cite_confusion();
                }
                nonexistent_cross_reference_error();
                *field_info.offset(field_ptr as isize) = 0i32
            /*missing */
            } else {
                if cite_loc != *ilk_info.offset(lc_cite_loc as isize) {
                    hash_cite_confusion();
                }
                cite_parent_ptr = *ilk_info.offset(cite_loc as isize);
                if *type_list.offset(cite_parent_ptr as isize) == 0i32 {
                    /*empty */
                    nonexistent_cross_reference_error();
                    *field_info.offset(field_ptr as isize) = 0i32
                /*missing */
                } else {
                    field_parent_ptr = cite_parent_ptr * num_fields + crossref_num;
                    if *field_info.offset(field_parent_ptr as isize) != 0i32 {
                        /*missing */
                        /*missing */
                        /*283: */
                        puts_log(
                            b"Warning--you\'ve nested cross references\x00" as *const u8
                                as *const i8,
                        );
                        bad_cross_reference_print(*cite_list.offset(cite_parent_ptr as isize));
                        puts_log(
                            b"\", which also refers to something\n\x00" as *const u8 as *const i8,
                        );
                        mark_warning();
                    }
                    if !all_entries
                        && cite_parent_ptr >= old_num_cites
                        && *cite_info.offset(cite_parent_ptr as isize) < min_crossrefs
                    {
                        *field_info.offset(field_ptr as isize) = 0i32
                    }
                }
            }
        }
        cite_ptr = cite_ptr + 1i32
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        if *type_list.offset(cite_ptr as isize) == 0i32 {
            /*empty */
            print_missing_entry(*cite_list.offset(cite_ptr as isize));
        } else if all_entries as i32 != 0
            || cite_ptr < old_num_cites
            || *cite_info.offset(cite_ptr as isize) >= min_crossrefs
        {
            if cite_ptr > cite_xptr {
                /*286: */
                if (cite_xptr + 1i32) * num_fields > max_fields {
                    puts_log(b"field_info index is out of range\x00" as *const u8 as *const i8);
                    print_confusion();
                    longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
                }
                *cite_list.offset(cite_xptr as isize) = *cite_list.offset(cite_ptr as isize);
                *type_list.offset(cite_xptr as isize) = *type_list.offset(cite_ptr as isize);
                if !find_cite_locs_for_this_cite_key(*cite_list.offset(cite_ptr as isize)) {
                    cite_key_disappeared_confusion();
                }
                if !cite_hash_found || cite_loc != *ilk_info.offset(lc_cite_loc as isize) {
                    hash_cite_confusion();
                }
                *ilk_info.offset(cite_loc as isize) = cite_xptr;
                field_ptr = cite_xptr * num_fields;
                field_end_ptr = field_ptr + num_fields;
                tmp_ptr = cite_ptr * num_fields;
                while field_ptr < field_end_ptr {
                    *field_info.offset(field_ptr as isize) = *field_info.offset(tmp_ptr as isize);
                    field_ptr = field_ptr + 1i32;
                    tmp_ptr = tmp_ptr + 1i32
                }
            }
            cite_xptr = cite_xptr + 1i32
        }
        cite_ptr = cite_ptr + 1i32
    }
    num_cites = cite_xptr;
    if all_entries {
        /*287: */
        cite_ptr = all_marker; /*end_of_string */
        while cite_ptr < old_num_cites {
            if !*entry_exists.offset(cite_ptr as isize) {
                print_missing_entry(*cite_info.offset(cite_ptr as isize));
            }
            cite_ptr = cite_ptr + 1i32
        }
    }
    entry_ints = xmalloc(
        (((num_ent_ints + 1i32) * (num_cites + 1i32)) as u64)
            .wrapping_mul(::std::mem::size_of::<i32>() as u64),
    ) as *mut i32;
    int_ent_ptr = 0i32;
    while int_ent_ptr < num_ent_ints * num_cites {
        *entry_ints.offset(int_ent_ptr as isize) = 0i32;
        int_ent_ptr = int_ent_ptr + 1i32
    }
    entry_strs = xmalloc(
        (((num_ent_strs + 1i32) * (num_cites + 1i32) * (ent_str_size + 1i32)) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    str_ent_ptr = 0i32;
    while str_ent_ptr < num_ent_strs * num_cites {
        *entry_strs.offset((str_ent_ptr * (ent_str_size + 1i32) + 0i32) as isize) =
            127i32 as ASCII_code;
        str_ent_ptr = str_ent_ptr + 1i32
    }
    cite_ptr = 0i32;
    while cite_ptr < num_cites {
        *cite_info.offset(cite_ptr as isize) = cite_ptr;
        cite_ptr = cite_ptr + 1i32
    }
    read_completed = true;
    buf_ptr2 = sv_ptr1;
    last = sv_ptr2;
    tmp_ptr = buf_ptr2;
    while tmp_ptr < last {
        *buffer.offset(tmp_ptr as isize) = *sv_buffer.offset(tmp_ptr as isize);
        tmp_ptr = tmp_ptr + 1i32
    }
}
unsafe extern "C" fn bst_reverse_command() {
    if !read_seen {
        puts_log(b"Illegal, reverse command before read command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"reverse\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"reverse\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"reverse\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    scan_identifier(
        125i32 as ASCII_code,
        37i32 as ASCII_code,
        37i32 as ASCII_code,
    );
    if scan_result as i32 == 3i32 || scan_result as i32 == 1i32 {
    } else {
        bst_id_print();
        puts_log(b"reverse\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if bad_argument_token() {
        return;
    }
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"reverse\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        bst_right_brace_print();
        puts_log(b"reverse\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 = buf_ptr2 + 1i32;
    init_command_execution();
    mess_with_entries = true;
    if num_cites > 0i32 {
        sort_cite_ptr = num_cites;
        loop {
            sort_cite_ptr = sort_cite_ptr - 1i32;
            cite_ptr = *cite_info.offset(sort_cite_ptr as isize);
            execute_fn(fn_loc);
            check_command_execution();
            if sort_cite_ptr == 0i32 {
                break;
            }
        }
    };
}
unsafe extern "C" fn bst_sort_command() {
    if !read_seen {
        puts_log(b"Illegal, sort command before read command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if num_cites > 1i32 {
        quick_sort(0i32, num_cites - 1i32);
    };
}
unsafe extern "C" fn bst_strings_command() {
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"strings\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    if *buffer.offset(buf_ptr2 as isize) as i32 != 123i32 {
        /*left_brace */
        bst_left_brace_print();
        puts_log(b"strings\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    buf_ptr2 += 1;
    if !eat_bst_white_space() {
        eat_bst_print();
        puts_log(b"strings\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    while *buffer.offset(buf_ptr2 as isize) as i32 != 125i32 {
        /*right_brace */
        scan_identifier(
            125i32 as ASCII_code,
            37i32 as ASCII_code,
            37i32 as ASCII_code,
        );
        if scan_result as i32 != 3i32 && scan_result as i32 != 1i32 {
            /*specified_char_adjacent */
            bst_id_print(); /*str_global_var */
            puts_log(b"strings\x00" as *const u8 as *const i8); /*HASH_SIZE */
            bst_err_print_and_look_for_blank_line();
            return;
        }
        lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
        fn_loc = str_lookup(
            buffer,
            buf_ptr1,
            buf_ptr2 - buf_ptr1,
            11i32 as str_ilk,
            true,
        );
        if hash_found {
            already_seen_function_print(fn_loc);
            return;
        }
        *fn_type.offset(fn_loc as isize) = 8i32 as fn_class;
        *ilk_info.offset(fn_loc as isize) = num_glb_strs;
        if num_glb_strs == max_glob_strs {
            glb_str_ptr = xrealloc(
                glb_str_ptr as *mut libc::c_void,
                ((max_glob_strs + 10i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<str_number>() as u64),
            ) as *mut str_number;
            global_strs = xrealloc(
                global_strs as *mut libc::c_void,
                (((max_glob_strs + 10i32) * (glob_str_size + 1i32)) as u64)
                    .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
            ) as *mut ASCII_code;
            glb_str_end = xrealloc(
                glb_str_end as *mut libc::c_void,
                ((max_glob_strs + 10i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<i32>() as u64),
            ) as *mut i32;
            max_glob_strs = max_glob_strs + 10i32;
            str_glb_ptr = num_glb_strs;
            while str_glb_ptr < max_glob_strs {
                *glb_str_ptr.offset(str_glb_ptr as isize) = 0i32;
                *glb_str_end.offset(str_glb_ptr as isize) = 0i32;
                str_glb_ptr = str_glb_ptr + 1i32
            }
        }
        num_glb_strs += 1;
        if !eat_bst_white_space() {
            eat_bst_print();
            puts_log(b"strings\x00" as *const u8 as *const i8);
            bst_err_print_and_look_for_blank_line();
            return;
        }
    }
    buf_ptr2 += 1;
}
unsafe extern "C" fn get_bst_command_and_process() {
    if !scan_alpha() {
        printf_log(
            b"\"%c\" can\'t start a style-file command\x00" as *const u8 as *const i8,
            *buffer.offset(buf_ptr2 as isize) as i32,
        );
        bst_err_print_and_look_for_blank_line();
        return;
    }
    lower_case(buffer, buf_ptr1, buf_ptr2 - buf_ptr1);
    command_num = *ilk_info.offset(str_lookup(
        buffer,
        buf_ptr1,
        buf_ptr2 - buf_ptr1,
        4i32 as str_ilk,
        false,
    ) as isize);
    if !hash_found {
        print_a_token();
        puts_log(b" is an illegal style-file command\x00" as *const u8 as *const i8);
        bst_err_print_and_look_for_blank_line();
        return;
    }
    match command_num {
        0 => {
            bst_entry_command();
        }
        1 => {
            bst_execute_command();
        }
        2 => {
            bst_function_command();
        }
        3 => {
            bst_integers_command();
        }
        4 => {
            bst_iterate_command();
        }
        5 => {
            bst_macro_command();
        }
        6 => {
            bst_read_command();
        }
        7 => {
            bst_reverse_command();
        }
        8 => {
            bst_sort_command();
        }
        9 => {
            bst_strings_command();
        }
        _ => {
            puts_log(b"Unknown style-file command\x00" as *const u8 as *const i8);
            print_confusion();
            longjmp(error_jmpbuf.as_mut_ptr(), 1i32);
        }
    };
}
unsafe extern "C" fn setup_params() {
    ent_str_size = 250i32;
    glob_str_size = 20000i32;
    max_strings = 35307i32;
    hash_size = max_strings;
    if hash_size < 5000i32 {
        /*HASH_SIZE */
        hash_size = 5000i32
    } /*other_lex */
    hash_max = hash_size + 1i32 - 1i32; /*alpha */
    end_of_def = hash_max + 1i32; /*illegal */
    undefined = hash_max + 1i32; /*illegal */
}
unsafe extern "C" fn compute_hash_prime() {
    let mut hash_want: i32 = 0; /*white_space */
    let mut k: i32 = 0; /*white_space */
    let mut j: i32 = 0; /*white_space */
    let mut o: i32 = 0; /*sep_char */
    let mut n: i32 = 0; /*sep_char */
    let mut square: i32 = 0; /*numeric */
    let mut j_prime: bool = false; /*alpha */
    hash_want = hash_size / 20i32 * 17i32; /*alpha */
    j = 1i32; /*legal_id_char */
    k = 1i32; /*illegal_id_char */
    hash_prime = 2i32; /*illegal_id_char */
    *hash_next.offset(k as isize) = hash_prime; /*illegal_id_char */
    o = 2i32; /*illegal_id_char */
    square = 9i32; /*illegal_id_char */
    while hash_prime < hash_want {
        loop {
            j += 2i32; /*illegal_id_char */
            if j == square {
                *hash_text.offset(o as isize) = j; /*illegal_id_char */
                j += 2i32; /*illegal_id_char */
                o += 1i32; /*illegal_id_char */
                square = *hash_next.offset(o as isize) * *hash_next.offset(o as isize)
            } /*illegal_id_char */
            n = 2i32; /*illegal_id_char */
            j_prime = true; /*illegal_id_char */
            while n < o && j_prime as i32 != 0 {
                while *hash_text.offset(n as isize) < j {
                    let ref mut fresh11 = *hash_text.offset(n as isize); /*illegal_id_char */
                    *fresh11 += 2i32 * *hash_next.offset(n as isize)
                } /*empty */
                if *hash_text.offset(n as isize) == j {
                    j_prime = false
                }
                n = n + 1i32
            }
            if j_prime {
                break;
            }
        }
        k += 1;
        hash_prime = j;
        *hash_next.offset(k as isize) = hash_prime
    }
}
unsafe extern "C" fn initialize(mut aux_file_name: *const i8) -> i32 {
    let mut i: i32 = 0;
    let mut k: hash_loc = 0;
    bad = 0i32;
    if 3i32 < 3i32 {
        bad = 1i32
    }
    if 79i32 <= 3i32 {
        bad = 10i32 * bad + 2i32
    }
    if 79i32 >= buf_size {
        bad = 10i32 * bad + 3i32
    }
    if hash_prime < 128i32 {
        bad = 10i32 * bad + 4i32
    }
    if hash_prime > hash_size {
        bad = 10i32 * bad + 5i32
    }
    if 1i32 != 1i32 {
        bad = 10i32 * bad + 6i32
    }
    if max_strings > hash_size {
        bad = 10i32 * bad + 7i32
    }
    if max_cites > max_strings {
        bad = 10i32 * bad + 8i32
    }
    if 10i32 < 2i32 * 4i32 + 2i32 {
        bad = 100i32 * bad + 22i32
    }
    if bad != 0 {
        return 1i32;
    }
    history = HISTORY_SPOTLESS as i32 as u8;
    i = 0i32;
    while i <= 127i32 {
        lex_class[i as usize] = 5i32 as lex_type;
        i += 1
    }
    i = 128i32;
    while i <= 255i32 {
        lex_class[i as usize] = 2i32 as lex_type;
        i += 1
    }
    i = 0i32;
    while i <= 31i32 {
        lex_class[i as usize] = 0i32 as lex_type;
        i += 1
    }
    lex_class[127] = 0i32 as lex_type;
    lex_class[9] = 1i32 as lex_type;
    lex_class[13] = 1i32 as lex_type;
    lex_class[32] = 1i32 as lex_type;
    lex_class[126] = 4i32 as lex_type;
    lex_class[45] = 4i32 as lex_type;
    i = 48i32;
    while i <= 57i32 {
        lex_class[i as usize] = 3i32 as lex_type;
        i += 1
    }
    i = 65i32;
    while i <= 90i32 {
        lex_class[i as usize] = 2i32 as lex_type;
        i += 1
    }
    i = 97i32;
    while i <= 122i32 {
        lex_class[i as usize] = 2i32 as lex_type;
        i += 1
    }
    i = 0i32;
    while i <= 255i32 {
        id_class[i as usize] = 1i32 as id_type;
        i += 1
    }
    i = 0i32;
    while i <= 31i32 {
        id_class[i as usize] = 0i32 as id_type;
        i += 1
    }
    id_class[32] = 0i32 as id_type;
    id_class[9] = 0i32 as id_type;
    id_class[34] = 0i32 as id_type;
    id_class[35] = 0i32 as id_type;
    id_class[37] = 0i32 as id_type;
    id_class[39] = 0i32 as id_type;
    id_class[40] = 0i32 as id_type;
    id_class[41] = 0i32 as id_type;
    id_class[44] = 0i32 as id_type;
    id_class[61] = 0i32 as id_type;
    id_class[123] = 0i32 as id_type;
    id_class[125] = 0i32 as id_type;
    i = 0i32;
    while i <= 127i32 {
        char_width[i as usize] = 0i32;
        i += 1
    }
    char_width[32] = 278i32;
    char_width[33] = 278i32;
    char_width[34] = 500i32;
    char_width[35] = 833i32;
    char_width[36] = 500i32;
    char_width[37] = 833i32;
    char_width[38] = 778i32;
    char_width[39] = 278i32;
    char_width[40] = 389i32;
    char_width[41] = 389i32;
    char_width[42] = 500i32;
    char_width[43] = 778i32;
    char_width[44] = 278i32;
    char_width[45] = 333i32;
    char_width[46] = 278i32;
    char_width[47] = 500i32;
    char_width[48] = 500i32;
    char_width[49] = 500i32;
    char_width[50] = 500i32;
    char_width[51] = 500i32;
    char_width[52] = 500i32;
    char_width[53] = 500i32;
    char_width[54] = 500i32;
    char_width[55] = 500i32;
    char_width[56] = 500i32;
    char_width[57] = 500i32;
    char_width[58] = 278i32;
    char_width[59] = 278i32;
    char_width[60] = 278i32;
    char_width[61] = 778i32;
    char_width[62] = 472i32;
    char_width[63] = 472i32;
    char_width[64] = 778i32;
    char_width[65] = 750i32;
    char_width[66] = 708i32;
    char_width[67] = 722i32;
    char_width[68] = 764i32;
    char_width[69] = 681i32;
    char_width[70] = 653i32;
    char_width[71] = 785i32;
    char_width[72] = 750i32;
    char_width[73] = 361i32;
    char_width[74] = 514i32;
    char_width[75] = 778i32;
    char_width[76] = 625i32;
    char_width[77] = 917i32;
    char_width[78] = 750i32;
    char_width[79] = 778i32;
    char_width[80] = 681i32;
    char_width[81] = 778i32;
    char_width[82] = 736i32;
    char_width[83] = 556i32;
    char_width[84] = 722i32;
    char_width[85] = 750i32;
    char_width[86] = 750i32;
    char_width[87] = 1028i32;
    char_width[88] = 750i32;
    char_width[89] = 750i32;
    char_width[90] = 611i32;
    char_width[91] = 278i32;
    char_width[92] = 500i32;
    char_width[93] = 278i32;
    char_width[94] = 500i32;
    char_width[95] = 278i32;
    char_width[96] = 278i32;
    char_width[97] = 500i32;
    char_width[98] = 556i32;
    char_width[99] = 444i32;
    char_width[100] = 556i32;
    char_width[101] = 444i32;
    char_width[102] = 306i32;
    char_width[103] = 500i32;
    char_width[104] = 556i32;
    char_width[105] = 278i32;
    char_width[106] = 306i32;
    char_width[107] = 528i32;
    char_width[108] = 278i32;
    char_width[109] = 833i32;
    char_width[110] = 556i32;
    char_width[111] = 500i32;
    char_width[112] = 556i32;
    char_width[113] = 528i32;
    char_width[114] = 392i32;
    char_width[115] = 394i32;
    char_width[116] = 389i32;
    char_width[117] = 556i32;
    char_width[118] = 528i32;
    char_width[119] = 722i32;
    char_width[120] = 528i32;
    char_width[121] = 528i32;
    char_width[122] = 444i32;
    char_width[123] = 500i32;
    char_width[124] = 1000i32;
    char_width[125] = 500i32;
    char_width[126] = 500i32;
    k = 1i32;
    while k <= hash_max {
        *hash_next.offset(k as isize) = 0i32;
        *hash_text.offset(k as isize) = 0i32;
        k += 1
    }
    hash_used = hash_max + 1i32;
    pool_ptr = 0i32;
    str_ptr = 1i32;
    *str_start.offset(str_ptr as isize) = pool_ptr;
    bib_ptr = 0i32;
    bib_seen = false;
    bst_str = 0i32;
    bst_seen = false;
    cite_ptr = 0i32;
    citation_seen = false;
    all_entries = false;
    wiz_def_ptr = 0i32;
    num_ent_ints = 0i32;
    num_ent_strs = 0i32;
    num_fields = 0i32;
    str_glb_ptr = 0i32;
    while str_glb_ptr < max_glob_strs {
        *glb_str_ptr.offset(str_glb_ptr as isize) = 0i32;
        *glb_str_end.offset(str_glb_ptr as isize) = 0i32;
        str_glb_ptr = str_glb_ptr + 1i32
    }
    num_glb_strs = 0i32;
    entry_seen = false;
    read_seen = false;
    read_performed = false;
    reading_completed = false;
    read_completed = false;
    impl_fn_num = 0i32;
    out_buf_length = 0i32;
    pre_def_certain_strings();
    get_the_top_level_aux_file_name(aux_file_name)
}
/* tectonic/bibtex.h
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn bibtex_main(mut aux_file_name: *const i8) -> tt_history_t {
    pool_size = 65000 as i32;
    buf_size = 20000i32;
    max_bib_files = 20i32;
    max_glob_strs = 10i32;
    max_fields = 17250i32;
    max_cites = 750i32;
    wiz_fn_space = 3000i32;
    lit_stk_size = 100i32;
    standard_output = ttstub_output_open_stdout();
    if standard_output.is_null() {
        return HISTORY_FATAL_ERROR;
    }
    setup_params();
    entry_ints = 0 as *mut i32;
    entry_strs = 0 as *mut ASCII_code;
    bib_file = xmalloc(
        ((max_bib_files + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<*mut peekable_input_t>() as u64),
    ) as *mut *mut peekable_input_t;
    bib_list = xmalloc(
        ((max_bib_files + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    wiz_functions = xmalloc(
        ((wiz_fn_space + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
    ) as *mut hash_ptr2;
    field_info = xmalloc(
        ((max_fields + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    s_preamble = xmalloc(
        ((max_bib_files + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    str_pool = xmalloc(
        ((pool_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    buffer = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as buf_type;
    sv_buffer = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as buf_type;
    ex_buf = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as buf_type;
    out_buf = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as buf_type;
    name_tok = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<buf_pointer>() as u64),
    ) as *mut buf_pointer;
    name_sep_char = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    glb_str_ptr =
        xmalloc((max_glob_strs as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64))
            as *mut str_number;
    global_strs = xmalloc(
        ((max_glob_strs * (glob_str_size + 1i32)) as u64)
            .wrapping_mul(::std::mem::size_of::<ASCII_code>() as u64),
    ) as *mut ASCII_code;
    glb_str_end = xmalloc((max_glob_strs as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
        as *mut i32;
    cite_list = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    type_list = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_ptr2>() as u64),
    ) as *mut hash_ptr2;
    entry_exists =
        xmalloc(((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64))
            as *mut bool;
    cite_info = xmalloc(
        ((max_cites + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    str_start = xmalloc(
        ((max_strings + 1i32) as u64).wrapping_mul(::std::mem::size_of::<pool_pointer>() as u64),
    ) as *mut pool_pointer;
    hash_next = xmalloc(
        ((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hash_pointer>() as u64),
    ) as *mut hash_pointer;
    hash_text = xmalloc(
        ((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    hash_ilk =
        xmalloc(((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_ilk>() as u64))
            as *mut str_ilk;
    ilk_info = xmalloc(((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
        as *mut i32;
    fn_type =
        xmalloc(((hash_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<fn_class>() as u64))
            as *mut fn_class;
    lit_stack =
        xmalloc(((lit_stk_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
            as *mut i32;
    lit_stk_type = xmalloc(
        ((lit_stk_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<stk_type>() as u64),
    ) as *mut stk_type;
    compute_hash_prime();
    if initialize(aux_file_name) != 0 {
        /* TODO: log initialization or get_the_..() error */
        return HISTORY_FATAL_ERROR;
    }
    if !(_setjmp(error_jmpbuf.as_mut_ptr()) == 1i32) {
        if verbose != 0 {
            puts_log(b"This is BibTeX, Version 0.99d\n\x00" as *const u8 as *const i8);
        } else {
            ttstub_puts(
                log_file,
                b"This is BibTeX, Version 0.99d\n\x00" as *const u8 as *const i8,
            );
        }
        let mut buf: [i8; 512] = [0; 512];
        snprintf(
            buf.as_mut_ptr(),
            (::std::mem::size_of::<[i8; 512]>() as u64).wrapping_sub(1i32 as u64),
            b"Capacity: max_strings=%ld, hash_size=%ld, hash_prime=%ld\n\x00" as *const u8
                as *const i8,
            max_strings as i64,
            hash_size as i64,
            hash_prime as i64,
        );
        ttstub_output_write(log_file, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
        if verbose != 0 {
            puts_log(b"The top-level auxiliary file: \x00" as *const u8 as *const i8);
            print_aux_name();
        } else {
            ttstub_puts(
                log_file,
                b"The top-level auxiliary file: \x00" as *const u8 as *const i8,
            );
            log_pr_aux_name();
        }
        loop {
            aux_ln_stack[aux_ptr as usize] += 1;
            if !input_ln(aux_file[aux_ptr as usize]) {
                if pop_the_aux_stack() != 0 {
                    break;
                }
            } else {
                get_aux_command_and_process();
            }
        }
        last_check_for_aux_errors();
        if !(bst_str == 0i32) {
            bst_line_num = 0i32;
            bbl_line_num = 1i32;
            buf_ptr2 = last;
            if _setjmp(recover_jmpbuf.as_mut_ptr()) == 0i32 {
                while eat_bst_white_space() {
                    get_bst_command_and_process();
                }
            }
            peekable_close(bst_file);
            bst_file = 0 as *mut peekable_input_t
        }
        ttstub_output_close(bbl_file);
    }
    /*456:*/
    if read_performed as i32 != 0 && !reading_completed {
        printf_log(
            b"Aborted at line %ld of file \x00" as *const u8 as *const i8,
            bib_line_num as i64,
        );
        print_bib_name();
    }
    match history as i32 {
        0 => {}
        1 => {
            if err_count == 1i32 {
                puts_log(b"(There was 1 warning)\n\x00" as *const u8 as *const i8);
            } else {
                printf_log(
                    b"(There were %ld warnings)\n\x00" as *const u8 as *const i8,
                    err_count as i64,
                );
            }
        }
        2 => {
            if err_count == 1i32 {
                puts_log(b"(There was 1 error message)\n\x00" as *const u8 as *const i8);
            } else {
                printf_log(
                    b"(There were %ld error messages)\n\x00" as *const u8 as *const i8,
                    err_count as i64,
                );
            }
        }
        3 => {
            puts_log(b"(That was a fatal error)\n\x00" as *const u8 as *const i8);
        }
        _ => {
            puts_log(b"History is bunk\x00" as *const u8 as *const i8);
            print_confusion();
        }
    }
    ttstub_output_close(log_file);
    history as tt_history_t
}
