#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use super::xetex_texmfmp::get_date_and_time;
use crate::mfree;
use crate::{
    ttstub_input_close, ttstub_input_open, ttstub_input_read, ttstub_output_close,
    ttstub_output_flush, ttstub_output_open, ttstub_output_open_stdout, ttstub_output_write,
};
use libc::free;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn get_avail() -> i32;
    #[no_mangle]
    fn flush_list(p: i32);
    #[no_mangle]
    fn get_node(s: i32) -> i32;
    #[no_mangle]
    fn free_node(p: i32, s: i32);
    #[no_mangle]
    fn delete_token_ref(p: i32);
    #[no_mangle]
    fn delete_glue_ref(p: i32);
    #[no_mangle]
    fn flush_node_list(p: i32);
    #[no_mangle]
    fn begin_diagnostic();
    #[no_mangle]
    fn end_diagnostic(blank_line: bool);
    #[no_mangle]
    fn print_cmd_chr(cmd: u16, chr_code: i32);
    #[no_mangle]
    fn id_lookup(j: i32, l: i32) -> i32;
    #[no_mangle]
    fn prim_lookup(s: str_number) -> i32;
    #[no_mangle]
    fn pseudo_close();
    #[no_mangle]
    fn sa_def(p: i32, e: i32);
    #[no_mangle]
    fn gsa_def(p: i32, e: i32);
    #[no_mangle]
    fn eq_define(p: i32, t: u16, e: i32);
    #[no_mangle]
    fn alter_page_so_far();
    #[no_mangle]
    fn load_pool_strings(spare_size: i32) -> i32;
    #[no_mangle]
    fn alter_prev_graf();
    #[no_mangle]
    fn eq_word_define(p: i32, w: i32);
    #[no_mangle]
    fn geq_define(p: i32, t: u16, e: i32);
    #[no_mangle]
    fn geq_word_define(p: i32, w: i32);
    #[no_mangle]
    fn show_cur_cmd_chr();
    #[no_mangle]
    fn end_token_list();
    #[no_mangle]
    fn back_input();
    #[no_mangle]
    fn back_error();
    #[no_mangle]
    fn end_file_reading();
    #[no_mangle]
    fn get_token();
    #[no_mangle]
    fn find_sa_element(t: small_number, n: i32, w: bool);
    #[no_mangle]
    fn get_x_token();
    #[no_mangle]
    fn scan_left_brace();
    #[no_mangle]
    fn scan_optional_equals();
    #[no_mangle]
    fn scan_keyword(s: *const i8) -> bool;
    #[no_mangle]
    fn scan_glyph_number(f: internal_font_number);
    #[no_mangle]
    fn scan_char_class();
    #[no_mangle]
    fn scan_char_class_not_ignored();
    #[no_mangle]
    fn scan_usv_num();
    #[no_mangle]
    fn scan_char_num();
    #[no_mangle]
    fn scan_xetex_math_char_int();
    #[no_mangle]
    fn scan_math_class_int();
    #[no_mangle]
    fn scan_math_fam_int();
    #[no_mangle]
    fn scan_fifteen_bit_int();
    #[no_mangle]
    fn scan_register_num();
    #[no_mangle]
    fn scan_font_ident();
    #[no_mangle]
    fn find_font_dimen(writing: bool);
    #[no_mangle]
    fn scan_int();
    #[no_mangle]
    fn scan_dimen(mu: bool, inf: bool, shortcut: bool);
    #[no_mangle]
    fn scan_glue(level: small_number);
    #[no_mangle]
    fn scan_toks(macro_def: bool, xpand: bool) -> i32;
    #[no_mangle]
    fn read_toks(n: i32, r: i32, j: i32);
    #[no_mangle]
    fn make_name_string() -> str_number;
    #[no_mangle]
    fn pack_job_name(_: *const i8);
    #[no_mangle]
    fn open_log_file();
    #[no_mangle]
    fn start_input(primary_input_name: *const i8);
    #[no_mangle]
    fn max_hyphenatable_length() -> i32;
    #[no_mangle]
    fn overflow(s: *const i8, n: i32) -> !;
    #[no_mangle]
    fn show_save_groups();
    #[no_mangle]
    fn do_marks(a: small_number, l: small_number, q: i32) -> bool;
    #[no_mangle]
    fn scan_box(box_context: i32);
    #[no_mangle]
    fn get_r_token();
    #[no_mangle]
    fn trap_zero_glue();
    #[no_mangle]
    fn do_register_command(a: small_number);
    #[no_mangle]
    fn destroy_font_manager();
    #[no_mangle]
    fn alter_integer();
    #[no_mangle]
    fn alter_box_dimen();
    #[no_mangle]
    fn new_font(a: small_number);
    #[no_mangle]
    fn new_interaction();
    #[no_mangle]
    fn confusion(s: *const i8) -> !;
    #[no_mangle]
    fn error();
    #[no_mangle]
    fn print_cstr(s: *const i8);
    #[no_mangle]
    fn print_esc_cstr(s: *const i8);
    #[no_mangle]
    fn print_nl_cstr(s: *const i8);
    #[no_mangle]
    fn print_file_line();
    #[no_mangle]
    fn length(s: str_number) -> i32;
    #[no_mangle]
    fn make_string() -> str_number;
    #[no_mangle]
    fn alter_aux();
    #[no_mangle]
    fn print_int(n: i32);
    #[no_mangle]
    fn print(s: i32);
    #[no_mangle]
    fn print_char(s: i32);
    #[no_mangle]
    fn main_control();
    #[no_mangle]
    fn close_files_and_terminate();
    #[no_mangle]
    fn initialize_math_variables();
    #[no_mangle]
    fn print_ln();
    #[no_mangle]
    fn print_nl(s: str_number);
    #[no_mangle]
    fn print_esc(s: str_number);
    #[no_mangle]
    fn print_file_name(n: i32, a: i32, e: i32);
    #[no_mangle]
    fn print_scaled(s: scaled_t);
    #[no_mangle]
    fn initialize_pagebuilder_variables();
    #[no_mangle]
    fn initialize_shipout_variables();
    #[no_mangle]
    fn deinitialize_shipout_variables();
    #[no_mangle]
    fn release_font_engine(engine: *mut libc::c_void, type_flag: i32);
    #[no_mangle]
    fn maketexstring(s: *const i8) -> i32;
    #[no_mangle]
    fn set_cp_code(fontNum: i32, code: u32, side: i32, value: i32);
    /* synctex.h

    Copyright (c) 2008, 2009 jerome DOT laurens AT u-bourgogne DOT fr

    This file is part of the SyncTeX package.

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation
    files (the "Software"), to deal in the Software without
    restriction, including without limitation the rights to use,
    copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
    OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
    HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
    WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE

    Acknowledgments:
    ----------------
    The author received useful remarks from the pdfTeX developers, especially Hahn The Thanh,
    and significant help from XeTeX developer Jonathan Kew

    Nota Bene:
    ----------
    If you include or use a significant part of the synctex package into a software,
    I would appreciate to be listed as contributor and see "SyncTeX" highlighted.

    Version 1
    Latest Revision: Wed Jul  1 08:17:50 UTC 2009

    */
    /*  Send this message to init the synctex command value to the command line option.
     *  Sending this message too early will cause a bus error.  */
    #[no_mangle]
    fn synctex_init_command();
    #[no_mangle]
    fn pdf_files_close();
    #[no_mangle]
    fn pdf_files_init();
}
pub type __ssize_t = i64;
pub type uintptr_t = u64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
/* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

use crate::{TTHistory, TTInputFormat};

pub type rust_output_handle_t = *mut libc::c_void;
pub type rust_input_handle_t = *mut libc::c_void;
/* quasi-hack to get the primary input */
/* tectonic/xetex-core.h: core XeTeX types and #includes.
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/
// defines U_IS_BIG_ENDIAN for us
/* fontconfig */
/* freetype */
/* harfbuzz */
/* Endianness foo */
/* our typedefs */
pub type scaled_t = i32;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Selector {
    FILE_0,
    FILE_15,
    NO_PRINT,
    TERM_ONLY,
    LOG_ONLY,
    TERM_AND_LOG,
    PSEUDO,
    NEW_STRING,
    // Looks like bug in `write_out`, should be deleted after oxidize
    Other(u8),
}

impl From<Selector> for u8 {
    fn from(u: Selector) -> Self {
        use Selector::*;
        match u {
            FILE_0 => 0,
            FILE_15 => 15,
            NO_PRINT => 16,
            TERM_ONLY => 17,
            LOG_ONLY => 18,
            TERM_AND_LOG => 19,
            PSEUDO => 20,
            NEW_STRING => 21,
            Other(u) => u,
        }
    }
}

impl From<u8> for Selector {
    fn from(u: u8) -> Self {
        use Selector::*;
        match u {
            0 => FILE_0,
            15 => FILE_15,
            16 => NO_PRINT,
            17 => TERM_ONLY,
            18 => LOG_ONLY,
            19 => TERM_AND_LOG,
            20 => PSEUDO,
            21 => NEW_STRING,
            n => Other(n),
        }
    }
}

/*18: */
pub type UTF16_code = u16;
pub type UTF8_code = u8;
pub type UnicodeScalar = i32;
pub type eight_bits = u8;
pub type pool_pointer = i32;
pub type str_number = i32;
pub type packed_UTF16_code = u16;
pub type small_number = i16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: i32,
    pub s1: i32,
}
/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */
pub type b32x2 = b32x2_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut libc::c_void,
}
/* ## THE ORIGINAL SITUATION (archived for posterity)
 *
 * In XeTeX, a "quarterword" is 16 bits. Who knows why. A "halfword" is,
 * sensibly, 32 bits. A "memory word" is a full word: either four quarters or
 * two halves: i.e., 64 bits. The memory word union also has options for
 * doubles (called `gr`), `integer` which is an i32 (called `cint`), and a
 * pointer (`ptr`).
 *
 * Original struct definition, LITTLE ENDIAN (condensed):
 *
 *   typedef union {
 *       struct { i32 LH, RH; } v;
 *       struct { short B1, B0; } u;
 *   } two_halves;
 *
 *   typedef struct {
 *       struct { u16 B3, B2, B1, B0; } u;
 *   } four_quarters;
 *
 *   typedef union {
 *       two_halves hh;
 *
 *       struct {
 *           i32 junk;
 *           i32 CINT;
 *       } u;
 *
 *       struct {
 *           four_quarters QQQQ;
 *       } v;
 *   } memory_word;
 *
 *   #  define cint u.CINT
 *   #  define qqqq v.QQQQ
 *
 * Original memory layout, LITTLE ENDIAN:
 *
 *   bytes:    --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   cint:                             [lsb...............msb]
 *   hh.u:     [l..B1...m] [l..B0...m]
 *   hh.v:     [lsb......LH.......msb] [lsb......RH.......msb]
 *   quarters: [l..B3...m] [l..B2...m] [l..B1...m] [l..B0...m]
 *
 * Original struct definition, BIG ENDIAN (condensed):
 *
 *   typedef union {
 *       struct { i32 RH, LH; } v;
 *       struct {
 *           i32 junk;
 *           short B0, B1;
 *       } u;
 *   } two_halves;
 *
 *   typedef struct {
 *       struct { u16 B0, B1, B2, B3; } u;
 *   } four_quarters;
 *
 *   typedef union {
 *       two_halves hh;
 *       four_quarters qqqq;
 *   } memory_word;
 *
 * Original memory layout, BIG ENDIAN:
 *
 *   bytes:    --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   cint:     [msb...............lsb]
 *   hh.u:                             [m..B0...l] [m..B1...l]
 *   hh.v:     [msb......RH.......lsb] [msb......LH.......lsb]
 *   quarters: [m..B0...l] [m..B1...l] [m..B2...l] [m...B3..l]
 *
 * Several things to note that apply to both endiannesses:
 *
 *   1. The different B0 and B1 instances do not line up.
 *   2. `cint` is isomorphic to `hh.v.RH`
 *   3. `hh.u.B0` is isomorphic to `qqqq.u.B2`
 *   4. `hh.u.B1` is isomorphic to `qqqq.u.B3`.
 *   5. The `four_quarters` field `u` serves no discernable purpose.
 *
 * CONVERTING TO THE NEW SYSTEM
 *
 * - `w.cint` => `w.b32.s1`
 * - `w.qqqq.u.B<n>` => `w.b16.s{{3 - <n>}}` !!!!!!!!!!!
 * - similar for `<quarterword_variable>.u.B<n>` => `<quarterword_variable>.s{{3 - <n>}}` !!!
 * - `w.hh.u.B0` => `w.b16.s1`
 * - `w.hh.u.B1` => `w.b16.s0`
 * - `w.hh.v.RH` => `w.b32.s1`
 * - `w.hh.v.LH` => `w.b32.s0`
 * - `four_quarters` => `b16x4`
 * - `two_halves` => `b32x2`
 *
 */
/* Symbolic accessors for various TeX data structures. I would loooove to turn these
 * into actual structs, but the path to doing that is not currently clear. Making
 * field references symbolic seems like a decent start. Sadly I don't see how to do
 * this conversion besides painstakingly annotating things.
 */
/* half of LLIST_info(p) */
/* the other half of LLIST_info(p) */
/* subtype; records L/R direction mode */
/* a scaled; 1 <=> WEB const `width_offset` */
/* a scaled; 2 <=> WEB const `depth_offset` */
/* a scaled; 3 <=> WEB const `height_offset` */
/* a scaled */
/* aka `link` of p+5 */
/* aka `type` of p+5 */
/* aka `subtype` of p+5 */
/* the glue ratio */
/* aka "subtype" of a node */
/* aka "rlink" in double-linked list */
/* aka "llink" in doubly-linked list */
/* was originally the `mem[x+2].int` field */
/* a scaled; "active_short" in the WEB */
/* a scaled */
/* aka "type" of a node */
/* aka "subtype" of a node */
/* the "natural width" difference */
/* the stretch difference in points */
/* the stretch difference in fil */
/* the stretch difference in fill */
/* the stretch difference in fill */
/* the shrink difference */
/* aka "subtype" of a node */
/* aka "llink" in doubly-linked list */
/* aka "rlink" in double-linked list */
/* "new left_edge position relative to cur_h" */
/* aka "llink" in doubly-linked list */
/* aka "rlink" in double-linked list */
/* "the floating_penalty to be used" */
/* a glue pointer */
/* a pointer to a vlist */
/* language number, 0..255 */
/* "minimum left fragment, range 1..63" */
/* "minimum right fragment, range 1..63" */
/* WEB: font(lig_char(p)) */
/* WEB: character(lig_char(p)) */
/* WEB: link(lig_char(p)) */
/* "head of the token list for the mark" */
/* "the mark class" */
/* To check: do these really only apply to MATH_NODEs? */
/* number of UTF16 items in the text */
/* ... or the glyph number, if subtype==GLYPH_NODE */
/* "an insertion for this class will break here if anywhere" */
/* "this insertion might break at broken_ptr" */
/* "the most recent insertion for this subtype" */
/* "the optimum most recent insertion" */
/* aka "llink" in doubly-linked list */
/* siggggghhhhh */
/* aka "rlink" in double-linked list */
/* aka "info" */
/* was originally the `mem[x+1].int` field */
/* number of bytes in the path item */
/* "reference count of token list to write" */
/* Synctex hacks various nodes to add an extra word at the end to store its
 * information, hence the need to know the node size to get the synctex
 * info. */
/* aka "link" of a link-list node */
/* aka "type" of a node */
/* aka "subtype" of a node */
/* a scaled */
/* a scaled */
/* e-TeX extended marks stuff ... not sure where to put these */
/* \topmarks<n> */
/* \firstmarks<n> */
/* \botmarks<n> */
/* \splitfirstmarks<n> */
/* \splitbotmarks<n> */
pub type glue_ord = u8;
/* enum: normal .. filll */
pub type group_code = u8;
pub type internal_font_number = i32;
pub type font_index = i32;
pub type nine_bits = i32;
/* range: 0 .. 0x1FF */
pub type trie_pointer = i32;
pub type trie_opcode = u16;
pub type hyph_pointer = u16;
pub type save_pointer = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_state_record {
    pub mode: i16,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_state_t {
    pub state: u16,
    pub index: u16,
    pub start: i32,
    pub loc: i32,
    pub limit: i32,
    pub name: i32,
    pub synctex_tag: i32,
}
/* tectonic/xetex-io.h: XeTeX-specific low-level I/O routines
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

pub use super::xetex_io::UFILE;
/* xetex-ini.c: WEB initialization code translated to C
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* All the following variables are declared in xetex-xetexd.h */
#[no_mangle]
pub static mut eqtb: *mut memory_word = 0 as *const memory_word as *mut memory_word;
#[no_mangle]
pub static mut bad: i32 = 0;
#[no_mangle]
pub static mut name_of_file: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut name_of_file16: *mut UTF16_code = 0 as *const UTF16_code as *mut UTF16_code;
#[no_mangle]
pub static mut name_length: i32 = 0;
#[no_mangle]
pub static mut name_length16: i32 = 0;
#[no_mangle]
pub static mut buffer: *mut UnicodeScalar = 0 as *const UnicodeScalar as *mut UnicodeScalar;
#[no_mangle]
pub static mut first: i32 = 0;
#[no_mangle]
pub static mut last: i32 = 0;
#[no_mangle]
pub static mut max_buf_stack: i32 = 0;
#[no_mangle]
pub static mut in_initex_mode: bool = false;
#[no_mangle]
pub static mut error_line: i32 = 0;
#[no_mangle]
pub static mut half_error_line: i32 = 0;
#[no_mangle]
pub static mut max_print_line: i32 = 0;
#[no_mangle]
pub static mut max_strings: i32 = 0;
#[no_mangle]
pub static mut strings_free: i32 = 0;
#[no_mangle]
pub static mut string_vacancies: i32 = 0;
#[no_mangle]
pub static mut pool_size: i32 = 0;
#[no_mangle]
pub static mut pool_free: i32 = 0;
#[no_mangle]
pub static mut font_mem_size: i32 = 0;
#[no_mangle]
pub static mut font_max: i32 = 0;
#[no_mangle]
pub static mut hyph_size: i32 = 0;
#[no_mangle]
pub static mut trie_size: i32 = 0;
#[no_mangle]
pub static mut buf_size: i32 = 0;
#[no_mangle]
pub static mut stack_size: i32 = 0;
#[no_mangle]
pub static mut max_in_open: i32 = 0;
#[no_mangle]
pub static mut param_size: i32 = 0;
#[no_mangle]
pub static mut nest_size: i32 = 0;
#[no_mangle]
pub static mut save_size: i32 = 0;
#[no_mangle]
pub static mut expand_depth: i32 = 0;
#[no_mangle]
pub static mut file_line_error_style_p: i32 = 0;
#[no_mangle]
pub static mut halt_on_error_p: i32 = 0;
#[no_mangle]
pub static mut quoted_filename: bool = false;
#[no_mangle]
pub static mut insert_src_special_auto: bool = false;
#[no_mangle]
pub static mut insert_src_special_every_par: bool = false;
#[no_mangle]
pub static mut insert_src_special_every_math: bool = false;
#[no_mangle]
pub static mut insert_src_special_every_vbox: bool = false;
#[no_mangle]
pub static mut str_pool: *mut packed_UTF16_code =
    0 as *const packed_UTF16_code as *mut packed_UTF16_code;
#[no_mangle]
pub static mut str_start: *mut pool_pointer = 0 as *const pool_pointer as *mut pool_pointer;
#[no_mangle]
pub static mut pool_ptr: pool_pointer = 0;
#[no_mangle]
pub static mut str_ptr: str_number = 0;
#[no_mangle]
pub static mut init_pool_ptr: pool_pointer = 0;
#[no_mangle]
pub static mut init_str_ptr: str_number = 0;
#[no_mangle]
pub static mut rust_stdout: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut log_file: rust_output_handle_t = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut selector: Selector = Selector::FILE_0;
#[no_mangle]
pub static mut dig: [u8; 23] = [0; 23];
#[no_mangle]
pub static mut tally: i32 = 0;
#[no_mangle]
pub static mut term_offset: i32 = 0;
#[no_mangle]
pub static mut file_offset: i32 = 0;
#[no_mangle]
pub static mut trick_buf: [UTF16_code; 256] = [0; 256];
#[no_mangle]
pub static mut trick_count: i32 = 0;
#[no_mangle]
pub static mut first_count: i32 = 0;
#[no_mangle]
pub static mut doing_special: bool = false;
#[no_mangle]
pub static mut native_text: *mut UTF16_code = 0 as *const UTF16_code as *mut UTF16_code;
#[no_mangle]
pub static mut native_text_size: i32 = 0;
#[no_mangle]
pub static mut native_len: i32 = 0;
#[no_mangle]
pub static mut save_native_len: i32 = 0;
#[no_mangle]
pub static mut interaction: u8 = 0;
#[no_mangle]
pub static mut deletions_allowed: bool = false;
#[no_mangle]
pub static mut set_box_allowed: bool = false;
#[no_mangle]
pub static mut history: TTHistory = TTHistory::SPOTLESS;
#[no_mangle]
pub static mut error_count: i8 = 0;
#[no_mangle]
pub static mut help_line: [*const i8; 6] = [0 as *const i8; 6];
#[no_mangle]
pub static mut help_ptr: u8 = 0;
#[no_mangle]
pub static mut use_err_help: bool = false;
#[no_mangle]
pub static mut arith_error: bool = false;
#[no_mangle]
pub static mut tex_remainder: scaled_t = 0;
#[no_mangle]
pub static mut temp_ptr: i32 = 0;
#[no_mangle]
pub static mut mem: *mut memory_word = 0 as *const memory_word as *mut memory_word;
#[no_mangle]
pub static mut lo_mem_max: i32 = 0;
#[no_mangle]
pub(crate) static mut hi_mem_min: i32 = 0;
#[no_mangle]
pub static mut var_used: i32 = 0;
#[no_mangle]
pub static mut dyn_used: i32 = 0;
#[no_mangle]
pub static mut avail: i32 = 0;
#[no_mangle]
pub static mut mem_end: i32 = 0;
#[no_mangle]
pub static mut rover: i32 = 0;
#[no_mangle]
pub static mut last_leftmost_char: i32 = 0;
#[no_mangle]
pub static mut last_rightmost_char: i32 = 0;
#[no_mangle]
pub static mut hlist_stack: [i32; 513] = [0; 513];
#[no_mangle]
pub static mut hlist_stack_level: i16 = 0;
#[no_mangle]
pub static mut first_p: i32 = 0;
#[no_mangle]
pub static mut global_prev_p: i32 = 0;
#[no_mangle]
pub static mut font_in_short_display: i32 = 0;
#[no_mangle]
pub static mut depth_threshold: i32 = 0;
#[no_mangle]
pub static mut breadth_max: i32 = 0;
#[no_mangle]
pub static mut nest: *mut list_state_record =
    0 as *const list_state_record as *mut list_state_record;
#[no_mangle]
pub static mut nest_ptr: i32 = 0;
#[no_mangle]
pub static mut max_nest_stack: i32 = 0;
#[no_mangle]
pub static mut cur_list: list_state_record = list_state_record {
    mode: 0,
    head: 0,
    tail: 0,
    eTeX_aux: 0,
    prev_graf: 0,
    mode_line: 0,
    aux: memory_word {
        b32: b32x2 { s0: 0, s1: 0 },
    },
};
#[no_mangle]
pub static mut shown_mode: i16 = 0;
#[no_mangle]
pub static mut old_setting: Selector = Selector::FILE_0;
#[no_mangle]
pub static mut hash: *mut b32x2 = 0 as *const b32x2 as *mut b32x2;
#[no_mangle]
pub static mut hash_used: i32 = 0;
#[no_mangle]
pub static mut hash_extra: i32 = 0;
#[no_mangle]
pub static mut hash_top: i32 = 0;
#[no_mangle]
pub static mut eqtb_top: i32 = 0;
#[no_mangle]
pub static mut hash_high: i32 = 0;
#[no_mangle]
pub static mut no_new_control_sequence: bool = false;
#[no_mangle]
pub static mut cs_count: i32 = 0;
#[no_mangle]
pub static mut prim: [b32x2; 501] = [b32x2 { s0: 0, s1: 0 }; 501];
#[no_mangle]
pub static mut prim_used: i32 = 0;
#[no_mangle]
pub static mut prim_eqtb: [memory_word; 501] = [memory_word {
    b32: b32x2 { s0: 0, s1: 0 },
}; 501];
#[no_mangle]
pub static mut save_stack: *mut memory_word = 0 as *const memory_word as *mut memory_word;
#[no_mangle]
pub static mut save_ptr: i32 = 0;
#[no_mangle]
pub static mut max_save_stack: i32 = 0;
#[no_mangle]
pub static mut cur_level: u16 = 0;
#[no_mangle]
pub static mut cur_group: group_code = 0;
#[no_mangle]
pub static mut cur_boundary: i32 = 0;
#[no_mangle]
pub static mut mag_set: i32 = 0;
#[no_mangle]
pub static mut cur_cmd: eight_bits = 0;
#[no_mangle]
pub static mut cur_chr: i32 = 0;
#[no_mangle]
pub static mut cur_cs: i32 = 0;
#[no_mangle]
pub static mut cur_tok: i32 = 0;
#[no_mangle]
pub static mut input_stack: *mut input_state_t = 0 as *const input_state_t as *mut input_state_t;
#[no_mangle]
pub static mut input_ptr: i32 = 0;
#[no_mangle]
pub static mut max_in_stack: i32 = 0;
#[no_mangle]
pub static mut cur_input: input_state_t = input_state_t {
    state: 0,
    index: 0,
    start: 0,
    loc: 0,
    limit: 0,
    name: 0,
    synctex_tag: 0,
};
#[no_mangle]
pub static mut in_open: i32 = 0;
#[no_mangle]
pub static mut open_parens: i32 = 0;
#[no_mangle]
pub static mut input_file: *mut *mut UFILE = 0 as *const *mut UFILE as *mut *mut UFILE;
#[no_mangle]
pub static mut line: i32 = 0;
#[no_mangle]
pub static mut line_stack: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut source_filename_stack: *mut str_number = 0 as *const str_number as *mut str_number;
#[no_mangle]
pub static mut full_source_filename_stack: *mut str_number =
    0 as *const str_number as *mut str_number;
#[no_mangle]
pub static mut scanner_status: u8 = 0;
#[no_mangle]
pub static mut warning_index: i32 = 0;
#[no_mangle]
pub static mut def_ref: i32 = 0;
#[no_mangle]
pub static mut param_stack: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut param_ptr: i32 = 0;
#[no_mangle]
pub static mut max_param_stack: i32 = 0;
#[no_mangle]
pub static mut align_state: i32 = 0;
#[no_mangle]
pub static mut base_ptr: i32 = 0;
#[no_mangle]
pub static mut par_loc: i32 = 0;
#[no_mangle]
pub static mut par_token: i32 = 0;
#[no_mangle]
pub static mut force_eof: bool = false;
#[no_mangle]
pub static mut expand_depth_count: i32 = 0;
#[no_mangle]
pub static mut is_in_csname: bool = false;
#[no_mangle]
pub static mut cur_mark: [i32; 5] = [0; 5];
#[no_mangle]
pub static mut long_state: u8 = 0;
#[no_mangle]
pub static mut pstack: [i32; 9] = [0; 9];
#[no_mangle]
pub static mut cur_val: i32 = 0;
#[no_mangle]
pub static mut cur_val1: i32 = 0;
#[no_mangle]
pub static mut cur_val_level: u8 = 0;
#[no_mangle]
pub static mut radix: small_number = 0;
#[no_mangle]
pub static mut cur_order: glue_ord = 0;
#[no_mangle]
pub static mut read_file: [*mut UFILE; 16] = [0 as *const UFILE as *mut UFILE; 16];
#[no_mangle]
pub static mut read_open: [u8; 17] = [0; 17];
#[no_mangle]
pub static mut cond_ptr: i32 = 0;
#[no_mangle]
pub static mut if_limit: u8 = 0;
#[no_mangle]
pub static mut cur_if: small_number = 0;
#[no_mangle]
pub static mut if_line: i32 = 0;
#[no_mangle]
pub static mut skip_line: i32 = 0;
#[no_mangle]
pub static mut cur_name: str_number = 0;
#[no_mangle]
pub static mut cur_area: str_number = 0;
#[no_mangle]
pub static mut cur_ext: str_number = 0;
#[no_mangle]
pub static mut area_delimiter: pool_pointer = 0;
#[no_mangle]
pub static mut ext_delimiter: pool_pointer = 0;
#[no_mangle]
pub static mut file_name_quote_char: UTF16_code = 0;
#[no_mangle]
pub static mut format_default_length: i32 = 0;
#[no_mangle]
pub static mut TEX_format_default: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut name_in_progress: bool = false;
#[no_mangle]
pub static mut job_name: str_number = 0;
#[no_mangle]
pub static mut log_opened: bool = false;
#[no_mangle]
pub static mut output_file_extension: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut texmf_log_name: str_number = 0;
#[no_mangle]
pub static mut font_info: *mut memory_word = 0 as *const memory_word as *mut memory_word;
#[no_mangle]
pub static mut fmem_ptr: font_index = 0;
#[no_mangle]
pub static mut font_ptr: internal_font_number = 0;
#[no_mangle]
pub static mut font_check: *mut b16x4 = 0 as *const b16x4 as *mut b16x4;
#[no_mangle]
pub static mut font_size: *mut scaled_t = 0 as *const scaled_t as *mut scaled_t;
#[no_mangle]
pub static mut font_dsize: *mut scaled_t = 0 as *const scaled_t as *mut scaled_t;
#[no_mangle]
pub static mut font_params: *mut font_index = 0 as *const font_index as *mut font_index;
#[no_mangle]
pub static mut font_name: *mut str_number = 0 as *const str_number as *mut str_number;
#[no_mangle]
pub static mut font_area: *mut str_number = 0 as *const str_number as *mut str_number;
#[no_mangle]
pub static mut font_bc: *mut UTF16_code = 0 as *const UTF16_code as *mut UTF16_code;
#[no_mangle]
pub static mut font_ec: *mut UTF16_code = 0 as *const UTF16_code as *mut UTF16_code;
#[no_mangle]
pub static mut font_glue: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut font_used: *mut bool = 0 as *const bool as *mut bool;
#[no_mangle]
pub static mut hyphen_char: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut skew_char: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut bchar_label: *mut font_index = 0 as *const font_index as *mut font_index;
#[no_mangle]
pub static mut font_bchar: *mut nine_bits = 0 as *const nine_bits as *mut nine_bits;
#[no_mangle]
pub static mut font_false_bchar: *mut nine_bits = 0 as *const nine_bits as *mut nine_bits;
#[no_mangle]
pub static mut font_layout_engine: *mut *mut libc::c_void =
    0 as *const *mut libc::c_void as *mut *mut libc::c_void;
#[no_mangle]
pub static mut font_mapping: *mut *mut libc::c_void =
    0 as *const *mut libc::c_void as *mut *mut libc::c_void;
#[no_mangle]
pub static mut font_flags: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut font_letter_space: *mut scaled_t = 0 as *const scaled_t as *mut scaled_t;
#[no_mangle]
pub static mut loaded_font_mapping: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut loaded_font_flags: i8 = 0;
#[no_mangle]
pub static mut loaded_font_letter_space: scaled_t = 0;
#[no_mangle]
pub static mut loaded_font_design_size: scaled_t = 0;
#[no_mangle]
pub static mut mapped_text: *mut UTF16_code = 0 as *const UTF16_code as *mut UTF16_code;
#[no_mangle]
pub static mut xdv_buffer: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut char_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut width_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut height_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut depth_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut italic_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut lig_kern_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut kern_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut exten_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut param_base: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut null_character: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[no_mangle]
pub static mut total_pages: i32 = 0;
#[no_mangle]
pub static mut max_v: scaled_t = 0;
#[no_mangle]
pub static mut max_h: scaled_t = 0;
#[no_mangle]
pub static mut max_push: i32 = 0;
#[no_mangle]
pub static mut last_bop: i32 = 0;
#[no_mangle]
pub static mut dead_cycles: i32 = 0;
#[no_mangle]
pub static mut doing_leaders: bool = false;
#[no_mangle]
pub static mut rule_ht: scaled_t = 0;
#[no_mangle]
pub static mut rule_dp: scaled_t = 0;
#[no_mangle]
pub static mut rule_wd: scaled_t = 0;
#[no_mangle]
pub static mut cur_h: scaled_t = 0;
#[no_mangle]
pub static mut cur_v: scaled_t = 0;
#[no_mangle]
pub static mut total_stretch: [scaled_t; 4] = [0; 4];
#[no_mangle]
pub static mut total_shrink: [scaled_t; 4] = [0; 4];
#[no_mangle]
pub static mut last_badness: i32 = 0;
#[no_mangle]
pub static mut adjust_tail: i32 = 0;
#[no_mangle]
pub static mut pre_adjust_tail: i32 = 0;
#[no_mangle]
pub static mut pack_begin_line: i32 = 0;
#[no_mangle]
pub static mut empty: b32x2 = b32x2 { s0: 0, s1: 0 };
#[no_mangle]
pub static mut cur_f: internal_font_number = 0;
#[no_mangle]
pub static mut cur_c: i32 = 0;
#[no_mangle]
pub static mut cur_i: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[no_mangle]
pub static mut cur_align: i32 = 0;
#[no_mangle]
pub static mut cur_span: i32 = 0;
#[no_mangle]
pub static mut cur_loop: i32 = 0;
#[no_mangle]
pub static mut align_ptr: i32 = 0;
#[no_mangle]
pub static mut cur_head: i32 = 0;
#[no_mangle]
pub static mut cur_tail: i32 = 0;
#[no_mangle]
pub static mut cur_pre_head: i32 = 0;
#[no_mangle]
pub static mut cur_pre_tail: i32 = 0;
#[no_mangle]
pub static mut just_box: i32 = 0;
#[no_mangle]
pub static mut active_width: [scaled_t; 7] = [0; 7];
#[no_mangle]
pub static mut hc: [i32; 4099] = [0; 4099];
#[no_mangle]
pub static mut hf: internal_font_number = 0;
#[no_mangle]
pub static mut hu: [i32; 4097] = [0; 4097];
#[no_mangle]
pub static mut cur_lang: u8 = 0;
#[no_mangle]
pub static mut max_hyph_char: i32 = 0;
#[no_mangle]
pub static mut hyf: [u8; 4097] = [0; 4097];
#[no_mangle]
pub static mut init_list: i32 = 0;
#[no_mangle]
pub static mut init_lig: bool = false;
#[no_mangle]
pub static mut init_lft: bool = false;
#[no_mangle]
pub static mut hyphen_passed: small_number = 0;
#[no_mangle]
pub static mut cur_l: i32 = 0;
#[no_mangle]
pub static mut cur_r: i32 = 0;
#[no_mangle]
pub static mut cur_q: i32 = 0;
#[no_mangle]
pub static mut lig_stack: i32 = 0;
#[no_mangle]
pub static mut ligature_present: bool = false;
#[no_mangle]
pub static mut lft_hit: bool = false;
#[no_mangle]
pub static mut rt_hit: bool = false;
#[no_mangle]
pub static mut trie_trl: *mut trie_pointer = 0 as *const trie_pointer as *mut trie_pointer;
#[no_mangle]
pub static mut trie_tro: *mut trie_pointer = 0 as *const trie_pointer as *mut trie_pointer;
#[no_mangle]
pub static mut trie_trc: *mut u16 = 0 as *const u16 as *mut u16;
#[no_mangle]
pub static mut hyf_distance: [small_number; 35112] = [0; 35112];
#[no_mangle]
pub static mut hyf_num: [small_number; 35112] = [0; 35112];
#[no_mangle]
pub static mut hyf_next: [trie_opcode; 35112] = [0; 35112];
#[no_mangle]
pub static mut op_start: [i32; 256] = [0; 256];
#[no_mangle]
pub static mut hyph_word: *mut str_number = 0 as *const str_number as *mut str_number;
#[no_mangle]
pub static mut hyph_list: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut hyph_link: *mut hyph_pointer = 0 as *const hyph_pointer as *mut hyph_pointer;
#[no_mangle]
pub static mut hyph_count: i32 = 0;
#[no_mangle]
pub static mut hyph_next: i32 = 0;
#[no_mangle]
pub static mut trie_used: [trie_opcode; 256] = [0; 256];
#[no_mangle]
pub static mut trie_op_lang: [u8; 35112] = [0; 35112];
#[no_mangle]
pub static mut trie_op_val: [trie_opcode; 35112] = [0; 35112];
#[no_mangle]
pub static mut trie_op_ptr: i32 = 0;
#[no_mangle]
pub static mut max_op_used: trie_opcode = 0;
#[no_mangle]
pub static mut trie_c: *mut packed_UTF16_code =
    0 as *const packed_UTF16_code as *mut packed_UTF16_code;
#[no_mangle]
pub static mut trie_o: *mut trie_opcode = 0 as *const trie_opcode as *mut trie_opcode;
#[no_mangle]
pub static mut trie_l: *mut trie_pointer = 0 as *const trie_pointer as *mut trie_pointer;
#[no_mangle]
pub static mut trie_r: *mut trie_pointer = 0 as *const trie_pointer as *mut trie_pointer;
#[no_mangle]
pub static mut trie_ptr: trie_pointer = 0;
#[no_mangle]
pub static mut trie_hash: *mut trie_pointer = 0 as *const trie_pointer as *mut trie_pointer;
#[no_mangle]
pub static mut trie_taken: *mut bool = 0 as *const bool as *mut bool;
#[no_mangle]
pub static mut trie_min: [trie_pointer; 65536] = [0; 65536];
#[no_mangle]
pub static mut trie_max: trie_pointer = 0;
#[no_mangle]
pub static mut trie_not_ready: bool = false;
#[no_mangle]
pub static mut best_height_plus_depth: scaled_t = 0;
#[no_mangle]
pub static mut main_f: internal_font_number = 0;
#[no_mangle]
pub static mut main_i: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[no_mangle]
pub static mut main_j: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[no_mangle]
pub static mut main_k: font_index = 0;
#[no_mangle]
pub static mut main_p: i32 = 0;
#[no_mangle]
pub static mut main_pp: i32 = 0;
#[no_mangle]
pub static mut main_ppp: i32 = 0;
#[no_mangle]
pub static mut main_h: i32 = 0;
#[no_mangle]
pub static mut is_hyph: bool = false;
#[no_mangle]
pub static mut space_class: i32 = 0;
#[no_mangle]
pub static mut prev_class: i32 = 0;
#[no_mangle]
pub static mut main_s: i32 = 0;
#[no_mangle]
pub static mut bchar: i32 = 0;
#[no_mangle]
pub static mut false_bchar: i32 = 0;
#[no_mangle]
pub static mut cancel_boundary: bool = false;
#[no_mangle]
pub static mut ins_disc: bool = false;
#[no_mangle]
pub static mut cur_box: i32 = 0;
#[no_mangle]
pub static mut after_token: i32 = 0;
#[no_mangle]
pub static mut long_help_seen: bool = false;
#[no_mangle]
pub static mut format_ident: str_number = 0;
#[no_mangle]
pub static mut write_file: [rust_output_handle_t; 16] =
    [0 as *const libc::c_void as *mut libc::c_void; 16];
#[no_mangle]
pub static mut write_open: [bool; 18] = [false; 18];
#[no_mangle]
pub static mut write_loc: i32 = 0;
#[no_mangle]
pub static mut cur_page_width: scaled_t = 0;
#[no_mangle]
pub static mut cur_page_height: scaled_t = 0;
#[no_mangle]
pub static mut cur_h_offset: scaled_t = 0;
#[no_mangle]
pub static mut cur_v_offset: scaled_t = 0;
#[no_mangle]
pub static mut pdf_last_x_pos: i32 = 0;
#[no_mangle]
pub static mut pdf_last_y_pos: i32 = 0;
#[no_mangle]
pub static mut eof_seen: *mut bool = 0 as *const bool as *mut bool;
#[no_mangle]
pub static mut LR_ptr: i32 = 0;
#[no_mangle]
pub static mut LR_problems: i32 = 0;
#[no_mangle]
pub static mut cur_dir: small_number = 0;
#[no_mangle]
pub static mut pseudo_files: i32 = 0;
#[no_mangle]
pub static mut grp_stack: *mut save_pointer = 0 as *const save_pointer as *mut save_pointer;
#[no_mangle]
pub static mut if_stack: *mut i32 = 0 as *const i32 as *mut i32;
#[no_mangle]
pub static mut max_reg_num: i32 = 0;
#[no_mangle]
pub static mut max_reg_help_line: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut sa_root: [i32; 8] = [0; 8];
#[no_mangle]
pub static mut cur_ptr: i32 = 0;
#[no_mangle]
pub static mut sa_null: memory_word = memory_word {
    b32: b32x2 { s0: 0, s1: 0 },
};
#[no_mangle]
pub static mut sa_chain: i32 = 0;
#[no_mangle]
pub static mut sa_level: u16 = 0;
#[no_mangle]
pub static mut hyph_start: trie_pointer = 0;
#[no_mangle]
pub static mut hyph_index: trie_pointer = 0;
#[no_mangle]
pub static mut disc_ptr: [i32; 4] = [0; 4];
#[no_mangle]
pub static mut edit_name_start: pool_pointer = 0;
#[no_mangle]
pub static mut stop_at_space: bool = false;
#[no_mangle]
pub static mut native_font_type_flag: i32 = 0;
#[no_mangle]
pub static mut xtx_ligature_present: bool = false;
#[no_mangle]
pub static mut delta: scaled_t = 0;
#[no_mangle]
pub static mut synctex_enabled: i32 = 0;
#[no_mangle]
pub static mut used_tectonic_coda_tokens: bool = false;
#[no_mangle]
pub static mut semantic_pagination_enabled: bool = false;
#[no_mangle]
pub static mut gave_char_warning_help: bool = false;
/* These ought to live in xetex-pagebuilder.c but are shared a lot: */
#[no_mangle]
pub static mut page_tail: i32 = 0;
#[no_mangle]
pub static mut page_contents: u8 = 0;
#[no_mangle]
pub static mut page_so_far: [scaled_t; 8] = [0; 8];
#[no_mangle]
pub static mut last_glue: i32 = 0;
#[no_mangle]
pub static mut last_penalty: i32 = 0;
#[no_mangle]
pub static mut last_kern: scaled_t = 0;
#[no_mangle]
pub static mut last_node_type: i32 = 0;
#[no_mangle]
pub static mut insert_penalties: i32 = 0;
#[no_mangle]
pub static mut output_active: bool = false;
#[no_mangle]
pub static mut _xeq_level_array: [u16; 1114732] = [0; 1114732];
static mut _trie_op_hash_array: [i32; 70223] = [0; 70223];
static mut yhash: *mut b32x2 = 0 as *const b32x2 as *mut b32x2;
/* Read and write dump files.  As distributed, these files are
architecture dependent; specifically, BigEndian and LittleEndian
architectures produce different files.  These routines always output
BigEndian files.  This still does not guarantee them to be
architecture-independent, because it is possible to make a format
that dumps a glue ratio, i.e., a floating-point number.  Fortunately,
none of the standard formats do that.  */
/* This macro is always invoked as a statement.  It assumes a variable
`temp'.  */
/* Make the NITEMS items pointed at by P, each of size SIZE, be the
opposite-endianness of whatever they are now.  */
unsafe extern "C" fn swap_items(mut p: *mut i8, mut nitems: size_t, mut size: size_t) {
    let mut temp: i8 = 0;
    match size {
        16 => loop {
            let fresh0 = nitems;
            nitems = nitems.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            temp = *p.offset(0);
            *p.offset(0) = *p.offset(15);
            *p.offset(15) = temp;
            temp = *p.offset(1);
            *p.offset(1) = *p.offset(14);
            *p.offset(14) = temp;
            temp = *p.offset(2);
            *p.offset(2) = *p.offset(13);
            *p.offset(13) = temp;
            temp = *p.offset(3);
            *p.offset(3) = *p.offset(12);
            *p.offset(12) = temp;
            temp = *p.offset(4);
            *p.offset(4) = *p.offset(11);
            *p.offset(11) = temp;
            temp = *p.offset(5);
            *p.offset(5) = *p.offset(10);
            *p.offset(10) = temp;
            temp = *p.offset(6);
            *p.offset(6) = *p.offset(9);
            *p.offset(9) = temp;
            temp = *p.offset(7);
            *p.offset(7) = *p.offset(8);
            *p.offset(8) = temp;
            p = p.offset(size as isize)
        },
        8 => loop {
            let fresh1 = nitems;
            nitems = nitems.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            temp = *p.offset(0);
            *p.offset(0) = *p.offset(7);
            *p.offset(7) = temp;
            temp = *p.offset(1);
            *p.offset(1) = *p.offset(6);
            *p.offset(6) = temp;
            temp = *p.offset(2);
            *p.offset(2) = *p.offset(5);
            *p.offset(5) = temp;
            temp = *p.offset(3);
            *p.offset(3) = *p.offset(4);
            *p.offset(4) = temp;
            p = p.offset(size as isize)
        },
        4 => loop {
            let fresh2 = nitems;
            nitems = nitems.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            temp = *p.offset(0);
            *p.offset(0) = *p.offset(3);
            *p.offset(3) = temp;
            temp = *p.offset(1);
            *p.offset(1) = *p.offset(2);
            *p.offset(2) = temp;
            p = p.offset(size as isize)
        },
        2 => loop {
            let fresh3 = nitems;
            nitems = nitems.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            temp = *p.offset(0);
            *p.offset(0) = *p.offset(1);
            *p.offset(1) = temp;
            p = p.offset(size as isize)
        },
        1 => {}
        _ => {
            _tt_abort(
                b"can\'t swap a %zu-byte item for (un)dumping\x00" as *const u8 as *const i8,
                size,
            );
        }
    };
}
/* not WORDS_BIGENDIAN */
/* Here we write NITEMS items, each item being ITEM_SIZE bytes long.
The pointer to the stuff to write is P, and we write to the file
OUT_FILE.  */
unsafe extern "C" fn do_dump(
    mut p: *mut i8,
    mut item_size: size_t,
    mut nitems: size_t,
    mut out_file: rust_output_handle_t,
) {
    swap_items(p, nitems, item_size);
    let mut r: ssize_t =
        ttstub_output_write(out_file, p, item_size.wrapping_mul(nitems)) as ssize_t;
    if r < 0i32 as i64 || r as size_t != item_size.wrapping_mul(nitems) {
        _tt_abort(
            b"could not write %zu %zu-byte item(s) to %s\x00" as *const u8 as *const i8,
            nitems,
            item_size,
            name_of_file,
        );
    }
    /* Have to restore the old contents of memory, since some of it might
    get used again.  */
    swap_items(p, nitems, item_size);
}
/* Here is the dual of the writing routine.  */
unsafe extern "C" fn do_undump(
    mut p: *mut i8,
    mut item_size: size_t,
    mut nitems: size_t,
    mut in_file: rust_input_handle_t,
) {
    let mut r: ssize_t = ttstub_input_read(in_file, p, item_size.wrapping_mul(nitems));
    if r < 0i32 as i64 || r as size_t != item_size.wrapping_mul(nitems) {
        _tt_abort(
            b"could not undump %zu %zu-byte item(s) from %s\x00" as *const u8 as *const i8,
            nitems,
            item_size,
            name_of_file,
        );
    }
    swap_items(p, nitems, item_size);
}
/*:134*/
/*135: */
unsafe extern "C" fn sort_avail() {
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut old_rover: i32 = 0;
    p = get_node(0x40000000i32);
    p = (*mem.offset((rover + 1i32) as isize)).b32.s1;
    (*mem.offset((rover + 1i32) as isize)).b32.s1 = 0x3fffffffi32;
    old_rover = rover;
    /*136: */
    while p != old_rover {
        if p < rover {
            q = p;
            p = (*mem.offset((q + 1i32) as isize)).b32.s1;
            (*mem.offset((q + 1i32) as isize)).b32.s1 = rover;
            rover = q
        } else {
            q = rover;
            while (*mem.offset((q + 1i32) as isize)).b32.s1 < p {
                q = (*mem.offset((q + 1i32) as isize)).b32.s1
            }
            r = (*mem.offset((p + 1i32) as isize)).b32.s1;
            (*mem.offset((p + 1i32) as isize)).b32.s1 = (*mem.offset((q + 1i32) as isize)).b32.s1;
            (*mem.offset((q + 1i32) as isize)).b32.s1 = p;
            p = r
        }
    }
    p = rover;
    while (*mem.offset((p + 1i32) as isize)).b32.s1 != 0x3fffffffi32 {
        (*mem.offset(((*mem.offset((p + 1i32) as isize)).b32.s1 + 1i32) as isize))
            .b32
            .s0 = p;
        p = (*mem.offset((p + 1i32) as isize)).b32.s1
    }
    (*mem.offset((p + 1i32) as isize)).b32.s1 = rover;
    (*mem.offset((rover + 1i32) as isize)).b32.s0 = p;
}
/*:271*/
/*276: */
unsafe extern "C" fn primitive(mut ident: *const i8, mut c: u16, mut o: i32) {
    let mut prim_val: i32 = 0;
    let mut len: i32 = strlen(ident) as i32;
    if len > 1i32 {
        let mut s: str_number = maketexstring(ident);
        if first + len > buf_size + 1i32 {
            overflow(b"buffer size\x00" as *const u8 as *const i8, buf_size);
        }
        let mut i: i32 = 0i32;
        while i < len {
            *buffer.offset((first + i) as isize) = *ident.offset(i as isize) as UnicodeScalar;
            i += 1
        }
        cur_val = id_lookup(first, len);
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - 65536i32) as isize);
        (*hash.offset(cur_val as isize)).s1 = s;
        prim_val = prim_lookup(s)
    } else {
        cur_val = *ident.offset(0) as i32 + (1i32 + (0x10ffffi32 + 1i32));
        prim_val = prim_lookup(*ident.offset(0) as str_number)
    }
    (*eqtb.offset(cur_val as isize)).b16.s0 = 1_u16;
    (*eqtb.offset(cur_val as isize)).b16.s1 = c;
    (*eqtb.offset(cur_val as isize)).b32.s1 = o;
    prim_eqtb[prim_val as usize].b16.s0 = 1_u16;
    prim_eqtb[prim_val as usize].b16.s1 = c;
    prim_eqtb[prim_val as usize].b32.s1 = o;
}
/*:925*/
/*977: */
#[no_mangle]
pub unsafe extern "C" fn new_trie_op(
    mut d: small_number,
    mut n: small_number,
    mut v: trie_opcode,
) -> trie_opcode {
    let mut h: i32 = 0;
    let mut u: trie_opcode = 0;
    let mut l: i32 = 0;
    h = ((n as i32 + 313i32 * d as i32 + 361i32 * v as i32 + 1009i32 * cur_lang as i32).abs()
        as i64
        % (35111 - -35111)
        + -35111) as i32;
    loop {
        l = _trie_op_hash_array[(h as i64 - -35111) as usize];
        if l == 0i32 {
            if trie_op_ptr as i64 == 35111 {
                overflow(
                    b"pattern memory ops\x00" as *const u8 as *const i8,
                    35111 as i32,
                );
            }
            u = trie_used[cur_lang as usize];
            if u as i64 == 65535 {
                overflow(
                    b"pattern memory ops per language\x00" as *const u8 as *const i8,
                    (65535 - 0i32 as i64) as i32,
                );
            }
            trie_op_ptr += 1;
            u = u.wrapping_add(1);
            trie_used[cur_lang as usize] = u;
            if u as i32 > max_op_used as i32 {
                max_op_used = u
            }
            hyf_distance[trie_op_ptr as usize] = d;
            hyf_num[trie_op_ptr as usize] = n;
            hyf_next[trie_op_ptr as usize] = v;
            trie_op_lang[trie_op_ptr as usize] = cur_lang;
            _trie_op_hash_array[(h as i64 - -35111) as usize] = trie_op_ptr;
            trie_op_val[trie_op_ptr as usize] = u;
            return u;
        }
        if hyf_distance[l as usize] as i32 == d as i32
            && hyf_num[l as usize] as i32 == n as i32
            && hyf_next[l as usize] as i32 == v as i32
            && trie_op_lang[l as usize] as i32 == cur_lang as i32
        {
            return trie_op_val[l as usize];
        }
        if h > -(35111 as i32) {
            h -= 1
        } else {
            h = 35111 as i32
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn trie_node(mut p: trie_pointer) -> trie_pointer {
    let mut h: trie_pointer = 0;
    let mut q: trie_pointer = 0;
    h = (*trie_c.offset(p as isize) as i32
        + 1009i32 * *trie_o.offset(p as isize) as i32
        + 2718i32 * *trie_l.offset(p as isize)
        + 3142i32 * *trie_r.offset(p as isize))
    .abs()
        % trie_size;
    loop {
        q = *trie_hash.offset(h as isize);
        if q == 0i32 {
            *trie_hash.offset(h as isize) = p;
            return p;
        }
        if *trie_c.offset(q as isize) as i32 == *trie_c.offset(p as isize) as i32
            && *trie_o.offset(q as isize) as i32 == *trie_o.offset(p as isize) as i32
            && *trie_l.offset(q as isize) == *trie_l.offset(p as isize)
            && *trie_r.offset(q as isize) == *trie_r.offset(p as isize)
        {
            return q;
        }
        if h > 0i32 {
            h -= 1
        } else {
            h = trie_size
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn compress_trie(mut p: trie_pointer) -> trie_pointer {
    if p == 0i32 {
        0i32
    } else {
        *trie_l.offset(p as isize) = compress_trie(*trie_l.offset(p as isize));
        *trie_r.offset(p as isize) = compress_trie(*trie_r.offset(p as isize));
        trie_node(p)
    }
}
#[no_mangle]
pub unsafe extern "C" fn first_fit(mut p: trie_pointer) {
    let mut h: trie_pointer = 0;
    let mut z: trie_pointer = 0;
    let mut q: trie_pointer = 0;
    let mut c: UTF16_code = 0;
    let mut l: trie_pointer = 0;
    let mut r: trie_pointer = 0;
    let mut ll: i32 = 0;
    c = *trie_c.offset(p as isize);
    z = trie_min[c as usize];
    's_31: loop {
        h = z - c as i32;
        if trie_max < h + max_hyph_char {
            if trie_size <= h + max_hyph_char {
                overflow(b"pattern memory\x00" as *const u8 as *const i8, trie_size);
            }
            loop {
                trie_max += 1;
                *trie_taken.offset(trie_max as isize) = false;
                *trie_trl.offset(trie_max as isize) = trie_max + 1i32;
                *trie_tro.offset(trie_max as isize) = trie_max - 1i32;
                if trie_max == h + max_hyph_char {
                    break;
                }
            }
        }
        if !*trie_taken.offset(h as isize) {
            q = *trie_r.offset(p as isize);
            loop {
                if !(q > 0i32) {
                    break 's_31;
                }
                if *trie_trl.offset((h + *trie_c.offset(q as isize) as i32) as isize) == 0i32 {
                    break;
                }
                q = *trie_r.offset(q as isize)
            }
        }
        /*not_found */
        z = *trie_trl.offset(z as isize)
    }
    /*found *//*991: */
    *trie_taken.offset(h as isize) = true;
    *trie_hash.offset(p as isize) = h;
    q = p;
    loop {
        z = h + *trie_c.offset(q as isize) as i32;
        l = *trie_tro.offset(z as isize);
        r = *trie_trl.offset(z as isize);
        *trie_tro.offset(r as isize) = l;
        *trie_trl.offset(l as isize) = r;
        *trie_trl.offset(z as isize) = 0i32;
        if l < max_hyph_char {
            if z < max_hyph_char {
                ll = z
            } else {
                ll = max_hyph_char
            }
            loop {
                trie_min[l as usize] = r;
                l += 1;
                if l == ll {
                    break;
                }
            }
        }
        q = *trie_r.offset(q as isize);
        if q == 0i32 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn trie_pack(mut p: trie_pointer) {
    let mut q: trie_pointer = 0;
    loop {
        q = *trie_l.offset(p as isize);
        if q > 0i32 && *trie_hash.offset(q as isize) == 0i32 {
            first_fit(q);
            trie_pack(q);
        }
        p = *trie_r.offset(p as isize);
        if p == 0i32 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn trie_fix(mut p: trie_pointer) {
    let mut q: trie_pointer = 0;
    let mut c: UTF16_code = 0;
    let mut z: trie_pointer = 0;
    z = *trie_hash.offset(p as isize);
    loop {
        q = *trie_l.offset(p as isize);
        c = *trie_c.offset(p as isize);
        *trie_trl.offset((z + c as i32) as isize) = *trie_hash.offset(q as isize);
        *trie_trc.offset((z + c as i32) as isize) = c;
        *trie_tro.offset((z + c as i32) as isize) = *trie_o.offset(p as isize) as trie_pointer;
        if q > 0i32 {
            trie_fix(q);
        }
        p = *trie_r.offset(p as isize);
        if p == 0i32 {
            break;
        }
    }
}
unsafe extern "C" fn new_patterns() {
    let mut k: i16 = 0;
    let mut l: i16 = 0;
    let mut digit_sensed: bool = false;
    let mut v: trie_opcode = 0;
    let mut p: trie_pointer = 0;
    let mut q: trie_pointer = 0;
    let mut first_child: bool = false;
    let mut c: UTF16_code = 0;
    if trie_not_ready {
        if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 50i32) as isize,
        ))
        .b32
        .s1 <= 0i32
        {
            cur_lang = 0_u8
        } else if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 50i32) as isize,
        ))
        .b32
        .s1 > 255i32
        {
            cur_lang = 0_u8
        } else {
            cur_lang = (*eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 50i32) as isize,
            ))
            .b32
            .s1 as u8
        }
        scan_left_brace();
        k = 0_i16;
        hyf[0] = 0_u8;
        digit_sensed = false;
        loop {
            get_x_token();
            match cur_cmd as i32 {
                11 | 12 => {
                    if digit_sensed as i32 != 0 || cur_chr < '0' as i32 || cur_chr > '9' as i32 {
                        if cur_chr == '.' as i32 {
                            cur_chr = 0i32
                        } else {
                            cur_chr = (*eqtb.offset(
                                (1i32
                                    + (0x10ffffi32 + 1i32)
                                    + (0x10ffffi32 + 1i32)
                                    + 1i32
                                    + 15000i32
                                    + 12i32
                                    + 9000i32
                                    + 1i32
                                    + 1i32
                                    + 19i32
                                    + 256i32
                                    + 256i32
                                    + 13i32
                                    + 256i32
                                    + 4i32
                                    + 256i32
                                    + 1i32
                                    + 3i32 * 256i32
                                    + (0x10ffffi32 + 1i32)
                                    + cur_chr) as isize,
                            ))
                            .b32
                            .s1;
                            if cur_chr == 0i32 {
                                if file_line_error_style_p != 0 {
                                    print_file_line();
                                } else {
                                    print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                                }
                                print_cstr(b"Nonletter\x00" as *const u8 as *const i8);
                                help_ptr = 1_u8;
                                help_line[0] = b"(See Appendix H.)\x00" as *const u8 as *const i8;
                                error();
                            }
                        }
                        if cur_chr > max_hyph_char {
                            max_hyph_char = cur_chr
                        }
                        if (k as i32) < max_hyphenatable_length() {
                            k += 1;
                            hc[k as usize] = cur_chr;
                            hyf[k as usize] = 0_u8;
                            digit_sensed = false
                        }
                    } else if (k as i32) < max_hyphenatable_length() {
                        hyf[k as usize] = (cur_chr - 48i32) as u8;
                        digit_sensed = true
                    }
                }
                10 | 2 => {
                    if k as i32 > 0i32 {
                        /*998:*/
                        if hc[1] == 0i32 {
                            hyf[0] = 0_u8
                        }
                        if hc[k as usize] == 0i32 {
                            hyf[k as usize] = 0_u8
                        }
                        l = k;
                        v = 0i32 as trie_opcode;
                        loop {
                            if hyf[l as usize] as i32 != 0i32 {
                                v = new_trie_op(
                                    (k as i32 - l as i32) as small_number,
                                    hyf[l as usize] as small_number,
                                    v,
                                )
                            }
                            if !(l as i32 > 0i32) {
                                break;
                            }
                            l -= 1
                        }
                        q = 0i32;
                        hc[0] = cur_lang as i32;
                        while l as i32 <= k as i32 {
                            c = hc[l as usize] as UTF16_code;
                            l += 1;
                            p = *trie_l.offset(q as isize);
                            first_child = true;
                            while p > 0i32 && c as i32 > *trie_c.offset(p as isize) as i32 {
                                q = p;
                                p = *trie_r.offset(q as isize);
                                first_child = false
                            }
                            if p == 0i32 || (c as i32) < *trie_c.offset(p as isize) as i32 {
                                /*999:*/
                                if trie_ptr == trie_size {
                                    overflow(
                                        b"pattern memory\x00" as *const u8 as *const i8,
                                        trie_size,
                                    );
                                }
                                trie_ptr += 1;
                                *trie_r.offset(trie_ptr as isize) = p;
                                p = trie_ptr;
                                *trie_l.offset(p as isize) = 0i32;
                                if first_child {
                                    *trie_l.offset(q as isize) = p
                                } else {
                                    *trie_r.offset(q as isize) = p
                                }
                                *trie_c.offset(p as isize) = c;
                                *trie_o.offset(p as isize) = 0i32 as trie_opcode
                            }
                            q = p
                        }
                        if *trie_o.offset(q as isize) as i32 != 0i32 {
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                            }
                            print_cstr(b"Duplicate pattern\x00" as *const u8 as *const i8);
                            help_ptr = 1_u8;
                            help_line[0] = b"(See Appendix H.)\x00" as *const u8 as *const i8;
                            error();
                        }
                        *trie_o.offset(q as isize) = v
                    }
                    if cur_cmd as i32 == 2i32 {
                        break;
                    }
                    k = 0_i16;
                    hyf[0] = 0_u8;
                    digit_sensed = false
                }
                _ => {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                    }
                    print_cstr(b"Bad \x00" as *const u8 as *const i8);
                    print_esc_cstr(b"patterns\x00" as *const u8 as *const i8);
                    help_ptr = 1_u8;
                    help_line[0] = b"(See Appendix H.)\x00" as *const u8 as *const i8;
                    error();
                }
            }
        }
        /*:996*/
        if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 66i32) as isize,
        ))
        .b32
        .s1 > 0i32
        {
            /*1643:*/
            c = cur_lang as UTF16_code;
            first_child = false;
            p = 0i32;
            loop {
                q = p;
                p = *trie_r.offset(q as isize);
                if p == 0i32 || c as i32 <= *trie_c.offset(p as isize) as i32 {
                    break;
                }
            }
            if p == 0i32 || (c as i32) < *trie_c.offset(p as isize) as i32 {
                /*:1644*/
                /*999:*/
                if trie_ptr == trie_size {
                    overflow(b"pattern memory\x00" as *const u8 as *const i8, trie_size);
                }
                trie_ptr += 1;
                *trie_r.offset(trie_ptr as isize) = p;
                p = trie_ptr;
                *trie_l.offset(p as isize) = 0i32;
                if first_child {
                    *trie_l.offset(q as isize) = p
                } else {
                    *trie_r.offset(q as isize) = p
                }
                *trie_c.offset(p as isize) = c;
                *trie_o.offset(p as isize) = 0i32 as trie_opcode
            }
            q = p;
            p = *trie_l.offset(q as isize);
            first_child = true;
            c = 0i32 as UTF16_code;
            while c as i32 <= 255i32 {
                if (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + c as i32) as isize,
                ))
                .b32
                .s1 > 0i32
                    || c as i32 == 255i32 && first_child as i32 != 0
                {
                    if p == 0i32 {
                        /*999:*/
                        if trie_ptr == trie_size {
                            overflow(b"pattern memory\x00" as *const u8 as *const i8, trie_size);
                            /*:987 */
                        }
                        trie_ptr += 1;
                        *trie_r.offset(trie_ptr as isize) = p;
                        p = trie_ptr;
                        *trie_l.offset(p as isize) = 0i32;
                        if first_child {
                            *trie_l.offset(q as isize) = p
                        } else {
                            *trie_r.offset(q as isize) = p
                        }
                        *trie_c.offset(p as isize) = c;
                        *trie_o.offset(p as isize) = 0i32 as trie_opcode
                    } else {
                        *trie_c.offset(p as isize) = c
                    }
                    *trie_o.offset(p as isize) = (*eqtb.offset(
                        (1i32
                            + (0x10ffffi32 + 1i32)
                            + (0x10ffffi32 + 1i32)
                            + 1i32
                            + 15000i32
                            + 12i32
                            + 9000i32
                            + 1i32
                            + 1i32
                            + 19i32
                            + 256i32
                            + 256i32
                            + 13i32
                            + 256i32
                            + 4i32
                            + 256i32
                            + 1i32
                            + 3i32 * 256i32
                            + (0x10ffffi32 + 1i32)
                            + c as i32) as isize,
                    ))
                    .b32
                    .s1 as trie_opcode;
                    q = p;
                    p = *trie_r.offset(q as isize);
                    first_child = false
                }
                c = c.wrapping_add(1)
            }
            if first_child {
                *trie_l.offset(q as isize) = 0i32
            } else {
                *trie_r.offset(q as isize) = 0i32
            }
        }
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const i8);
        }
        print_cstr(b"Too late for \x00" as *const u8 as *const i8);
        print_esc_cstr(b"patterns\x00" as *const u8 as *const i8);
        help_ptr = 1_u8;
        help_line[0] =
            b"All patterns must be given before typesetting begins.\x00" as *const u8 as *const i8;
        error();
        (*mem.offset((4999999i32 - 12i32) as isize)).b32.s1 = scan_toks(false, false);
        flush_list(def_ref);
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_trie() {
    let mut p: trie_pointer = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut r: trie_pointer = 0;
    let mut s: trie_pointer = 0;
    max_hyph_char += 1;
    op_start[0] = -0i32;
    let mut for_end: i32 = 0;
    j = 1i32;
    for_end = 255i32;
    if j <= for_end {
        loop {
            op_start[j as usize] =
                op_start[(j - 1i32) as usize] + trie_used[(j - 1i32) as usize] as i32;
            let fresh4 = j;
            j = j + 1;
            if !(fresh4 < for_end) {
                break;
            }
        }
    }
    let mut for_end_0: i32 = 0;
    j = 1i32;
    for_end_0 = trie_op_ptr;
    if j <= for_end_0 {
        loop {
            _trie_op_hash_array[(j as i64 - -35111) as usize] =
                op_start[trie_op_lang[j as usize] as usize] + trie_op_val[j as usize] as i32;
            let fresh5 = j;
            j = j + 1;
            if !(fresh5 < for_end_0) {
                break;
            }
        }
    }
    let mut for_end_1: i32 = 0;
    j = 1i32;
    for_end_1 = trie_op_ptr;
    if j <= for_end_1 {
        loop {
            while _trie_op_hash_array[(j as i64 - -35111) as usize] > j {
                k = _trie_op_hash_array[(j as i64 - -35111) as usize];
                t = hyf_distance[k as usize] as i32;
                hyf_distance[k as usize] = hyf_distance[j as usize];
                hyf_distance[j as usize] = t as small_number;
                t = hyf_num[k as usize] as i32;
                hyf_num[k as usize] = hyf_num[j as usize];
                hyf_num[j as usize] = t as small_number;
                t = hyf_next[k as usize] as i32;
                hyf_next[k as usize] = hyf_next[j as usize];
                hyf_next[j as usize] = t as trie_opcode;
                _trie_op_hash_array[(j as i64 - -35111) as usize] =
                    _trie_op_hash_array[(k as i64 - -35111) as usize];
                _trie_op_hash_array[(k as i64 - -35111) as usize] = k
            }
            let fresh6 = j;
            j = j + 1;
            if !(fresh6 < for_end_1) {
                break;
            }
        }
    }
    let mut for_end_2: i32 = 0;
    p = 0i32;
    for_end_2 = trie_size;
    if p <= for_end_2 {
        loop {
            *trie_hash.offset(p as isize) = 0i32;
            let fresh7 = p;
            p = p + 1;
            if !(fresh7 < for_end_2) {
                break;
            }
        }
    }
    *trie_r.offset(0) = compress_trie(*trie_r.offset(0));
    *trie_l.offset(0) = compress_trie(*trie_l.offset(0));
    let mut for_end_3: i32 = 0;
    p = 0i32;
    for_end_3 = trie_ptr;
    if p <= for_end_3 {
        loop {
            *trie_hash.offset(p as isize) = 0i32;
            let fresh8 = p;
            p = p + 1;
            if !(fresh8 < for_end_3) {
                break;
            }
        }
    }
    let mut for_end_4: i32 = 0;
    p = 0i32;
    for_end_4 = 0xffffi32;
    if p <= for_end_4 {
        loop {
            trie_min[p as usize] = p + 1i32;
            let fresh9 = p;
            p = p + 1;
            if !(fresh9 < for_end_4) {
                break;
            }
        }
    }
    *trie_trl.offset(0) = 1i32;
    trie_max = 0i32;
    if *trie_l.offset(0) != 0i32 {
        first_fit(*trie_l.offset(0));
        trie_pack(*trie_l.offset(0));
    }
    if *trie_r.offset(0) != 0i32 {
        /*1645: */
        if *trie_l.offset(0) == 0i32 {
            let mut for_end_5: i32 = 0;
            p = 0i32;
            for_end_5 = 255i32;
            if p <= for_end_5 {
                loop {
                    trie_min[p as usize] = p + 2i32;
                    let fresh10 = p;
                    p = p + 1;
                    if !(fresh10 < for_end_5) {
                        break;
                    }
                }
            }
        }
        first_fit(*trie_r.offset(0));
        trie_pack(*trie_r.offset(0));
        hyph_start = *trie_hash.offset(*trie_r.offset(0) as isize)
    }
    if trie_max == 0i32 {
        let mut for_end_6: i32 = 0;
        r = 0i32;
        for_end_6 = max_hyph_char;
        if r <= for_end_6 {
            loop {
                *trie_trl.offset(r as isize) = 0i32;
                *trie_tro.offset(r as isize) = 0i32;
                *trie_trc.offset(r as isize) = 0_u16;
                let fresh11 = r;
                r = r + 1;
                if !(fresh11 < for_end_6) {
                    break;
                }
            }
        }
        trie_max = max_hyph_char
    } else {
        if *trie_r.offset(0) > 0i32 {
            trie_fix(*trie_r.offset(0));
        }
        if *trie_l.offset(0) > 0i32 {
            trie_fix(*trie_l.offset(0));
        }
        r = 0i32;
        loop {
            s = *trie_trl.offset(r as isize);
            *trie_trl.offset(r as isize) = 0i32;
            *trie_tro.offset(r as isize) = 0i32;
            *trie_trc.offset(r as isize) = 0_u16;
            r = s;
            if r > trie_max {
                break;
            }
        }
    }
    *trie_trc.offset(0) = '?' as i32 as u16;
    trie_not_ready = false;
}
/*:1001*/
unsafe extern "C" fn new_hyph_exceptions() {
    let mut current_block: u64;
    let mut n: i16 = 0;
    let mut j: i16 = 0;
    let mut h: hyph_pointer = 0;
    let mut k: str_number = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut s: str_number = 0;
    let mut u: pool_pointer = 0;
    let mut v: pool_pointer = 0;
    scan_left_brace();
    if (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 50i32) as isize,
    ))
    .b32
    .s1 <= 0i32
    {
        cur_lang = 0_u8
    } else if (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 50i32) as isize,
    ))
    .b32
    .s1 > 255i32
    {
        cur_lang = 0_u8
    } else {
        cur_lang = (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 50i32) as isize,
        ))
        .b32
        .s1 as u8
    }
    if trie_not_ready {
        hyph_index = 0i32
    } else if *trie_trc.offset((hyph_start + cur_lang as i32) as isize) as i32 != cur_lang as i32 {
        hyph_index = 0i32
    } else {
        hyph_index = *trie_trl.offset((hyph_start + cur_lang as i32) as isize)
    }
    /*970:*/
    n = 0_i16;
    p = -0xfffffffi32;
    's_91: loop {
        get_x_token();
        loop {
            match cur_cmd as i32 {
                11 | 12 | 68 => {
                    if cur_chr == '-' as i32 {
                        /*973:*/
                        if (n as i32) < max_hyphenatable_length() {
                            q = get_avail();
                            (*mem.offset(q as isize)).b32.s1 = p;
                            (*mem.offset(q as isize)).b32.s0 = n as i32;
                            p = q
                        }
                    } else {
                        if hyph_index == 0i32 || cur_chr > 255i32 {
                            hc[0] = (*eqtb.offset(
                                (1i32
                                    + (0x10ffffi32 + 1i32)
                                    + (0x10ffffi32 + 1i32)
                                    + 1i32
                                    + 15000i32
                                    + 12i32
                                    + 9000i32
                                    + 1i32
                                    + 1i32
                                    + 19i32
                                    + 256i32
                                    + 256i32
                                    + 13i32
                                    + 256i32
                                    + 4i32
                                    + 256i32
                                    + 1i32
                                    + 3i32 * 256i32
                                    + (0x10ffffi32 + 1i32)
                                    + cur_chr) as isize,
                            ))
                            .b32
                            .s1
                        } else if *trie_trc.offset((hyph_index + cur_chr) as isize) as i32
                            != cur_chr
                        {
                            hc[0] = 0i32
                        } else {
                            hc[0] = *trie_tro.offset((hyph_index + cur_chr) as isize)
                        }
                        if hc[0] == 0i32 {
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                            }
                            print_cstr(b"Not a letter\x00" as *const u8 as *const i8);
                            help_ptr = 2_u8;
                            help_line[1] =
                                b"Letters in \\hyphenation words must have \\lccode>0.\x00"
                                    as *const u8 as *const i8;
                            help_line[0] = b"Proceed; I\'ll ignore the character I just read.\x00"
                                as *const u8
                                as *const i8;
                            error();
                        } else if (n as i32) < max_hyphenatable_length() {
                            n += 1;
                            if (hc[0] as i64) < 65536 {
                                hc[n as usize] = hc[0]
                            } else {
                                hc[n as usize] =
                                    ((hc[0] as i64 - 65536) / 1024i32 as i64 + 55296) as i32;
                                n += 1;
                                hc[n as usize] = ((hc[0] % 1024i32) as i64 + 56320) as i32
                            }
                        }
                    }
                    continue 's_91;
                }
                16 => {
                    scan_char_num();
                    cur_chr = cur_val;
                    cur_cmd = 68i32 as eight_bits
                }
                10 | 2 => {
                    if n as i32 > 1i32 {
                        current_block = 10753070352654377903;
                        break;
                    } else {
                        current_block = 9500030526577190060;
                        break;
                    }
                }
                _ => {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! \x00" as *const u8 as *const i8);
                    }
                    print_cstr(b"Improper \x00" as *const u8 as *const i8);
                    print_esc_cstr(b"hyphenation\x00" as *const u8 as *const i8);
                    print_cstr(b" will be flushed\x00" as *const u8 as *const i8);
                    help_ptr = 2_u8;
                    help_line[1] = b"Hyphenation exceptions must contain only letters\x00"
                        as *const u8 as *const i8;
                    help_line[0] = b"and hyphens. But continue; I\'ll forgive and forget.\x00"
                        as *const u8 as *const i8;
                    error();
                    continue 's_91;
                }
            }
        }
        match current_block {
            10753070352654377903 => {
                /*974:*/
                n += 1;
                hc[n as usize] = cur_lang as i32;
                if pool_ptr + n as i32 > pool_size {
                    overflow(
                        b"pool size\x00" as *const u8 as *const i8,
                        pool_size - init_pool_ptr,
                    );
                }
                h = 0i32 as hyph_pointer;
                j = 1_i16;
                while j as i32 <= n as i32 {
                    h = ((h as i32 + h as i32 + hc[j as usize]) % 607i32) as hyph_pointer;
                    *str_pool.offset(pool_ptr as isize) = hc[j as usize] as packed_UTF16_code;
                    pool_ptr += 1;
                    j += 1
                }
                s = make_string();
                if hyph_next <= 607i32 {
                    while hyph_next > 0i32 && *hyph_word.offset((hyph_next - 1i32) as isize) > 0i32
                    {
                        hyph_next -= 1
                    }
                }
                if hyph_count == hyph_size || hyph_next == 0i32 {
                    overflow(
                        b"exception dictionary\x00" as *const u8 as *const i8,
                        hyph_size,
                    );
                }
                hyph_count += 1;
                while *hyph_word.offset(h as isize) != 0i32 {
                    k = *hyph_word.offset(h as isize);
                    if !(length(k) != length(s)) {
                        u = *str_start.offset((k as i64 - 65536) as isize);
                        v = *str_start.offset((s as i64 - 65536) as isize);
                        loop {
                            if *str_pool.offset(u as isize) as i32
                                != *str_pool.offset(v as isize) as i32
                            {
                                current_block = 876886731760051519;
                                break;
                            }
                            u += 1;
                            v += 1;
                            if !(u != *str_start.offset(((k + 1i32) as i64 - 65536) as isize)) {
                                current_block = 8732226822098929438;
                                break;
                            }
                        }
                        match current_block {
                            876886731760051519 => {}
                            _ => {
                                str_ptr -= 1;
                                pool_ptr = *str_start.offset((str_ptr - 65536i32) as isize);
                                s = *hyph_word.offset(h as isize);
                                hyph_count -= 1;
                                break;
                            }
                        }
                    }
                    /*:975*/
                    /*:976*/
                    if *hyph_link.offset(h as isize) as i32 == 0i32 {
                        *hyph_link.offset(h as isize) = hyph_next as hyph_pointer;
                        if hyph_next >= hyph_size {
                            hyph_next = 607i32
                        }
                        if hyph_next > 607i32 {
                            hyph_next += 1
                        }
                    }
                    h = (*hyph_link.offset(h as isize) as i32 - 1i32) as hyph_pointer
                }
                *hyph_word.offset(h as isize) = s;
                *hyph_list.offset(h as isize) = p
            }
            _ => {}
        }
        if cur_cmd as i32 == 2i32 {
            return;
        }
        n = 0_i16;
        p = -0xfffffffi32
    }
}
#[no_mangle]
pub unsafe extern "C" fn prefixed_command() {
    let mut current_block: u64;
    let mut a: small_number = 0;
    let mut f: internal_font_number = 0;
    let mut j: i32 = 0;
    let mut k: font_index = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut n: i32 = 0;
    let mut e: bool = false;
    a = 0i32 as small_number;
    while cur_cmd as i32 == 95i32 {
        if a as i32 / cur_chr & 1i32 == 0 {
            a = (a as i32 + cur_chr) as small_number
        }
        loop {
            get_x_token();
            if !(cur_cmd as i32 == 10i32 || cur_cmd as i32 == 0i32) {
                break;
            }
        }
        if cur_cmd as i32 <= 71i32 {
            /*1247:*/
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const i8);
            }
            print_cstr(b"You can\'t use a prefix with `\x00" as *const u8 as *const i8);
            print_cmd_chr(cur_cmd as u16, cur_chr);
            print_char('\'' as i32);
            help_ptr = 1_u8;
            help_line[0] =
                b"I\'ll pretend you didn\'t say \\long or \\outer or \\global or \\protected.\x00"
                    as *const u8 as *const i8;
            back_error();
            return;
        }
        if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 36i32) as isize,
        ))
        .b32
        .s1 > 2i32
        {
            show_cur_cmd_chr();
        }
    }
    if a as i32 >= 8i32 {
        j = 0x1c00000i32 + 1i32;
        a = (a as i32 - 8i32) as small_number
    } else {
        j = 0i32
    }
    if cur_cmd as i32 != 99i32 && (a as i32 % 4i32 != 0i32 || j != 0i32) {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const i8);
        }
        print_cstr(b"You can\'t use `\x00" as *const u8 as *const i8);
        print_esc_cstr(b"long\x00" as *const u8 as *const i8);
        print_cstr(b"\' or `\x00" as *const u8 as *const i8);
        print_esc_cstr(b"outer\x00" as *const u8 as *const i8);
        help_ptr = 1_u8;
        help_line[0] = b"I\'ll pretend you didn\'t say \\long or \\outer or \\protected here.\x00"
            as *const u8 as *const i8;
        print_cstr(b"\' or `\x00" as *const u8 as *const i8);
        print_esc_cstr(b"protected\x00" as *const u8 as *const i8);
        print_cstr(b"\' with `\x00" as *const u8 as *const i8);
        print_cmd_chr(cur_cmd as u16, cur_chr);
        print_char('\'' as i32);
        error();
    }
    if (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 43i32) as isize,
    ))
    .b32
    .s1 != 0i32
    {
        if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 43i32) as isize,
        ))
        .b32
        .s1 < 0i32
        {
            if a as i32 >= 4i32 {
                a = (a as i32 - 4i32) as small_number
            }
        } else if (a as i32) < 4i32 {
            a = (a as i32 + 4i32) as small_number
        }
    }
    match cur_cmd as i32 {
        89 => {
            /*1252:*/
            if a as i32 >= 4i32 {
                geq_define(1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32)
                               + 1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 +
                               1i32 + 19i32 + 256i32 + 256i32 + 13i32 + 256i32
                               + 4i32 + 256i32, 122_u16, cur_chr);
            } else {
                eq_define(1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32, 122_u16, cur_chr);
            }
        }
        99 => {
            if cur_chr & 1i32 != 0 && (a as i32) < 4i32 &&
                   (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                      12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                      256i32 + 256i32 + 13i32 + 256i32 + 4i32
                                      + 256i32 + 1i32 + 3i32 * 256i32 +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) + 43i32) as
                                     isize)).b32.s1 >= 0i32 {
                a = (a as i32 + 4i32) as small_number
            }
            e = cur_chr >= 2i32;
            get_r_token();
            p = cur_cs;
            q = scan_toks(1i32 != 0, e);
            if j != 0i32 {
                q = get_avail();
                (*mem.offset(q as isize)).b32.s0 = j;
                (*mem.offset(q as isize)).b32.s1 =
                    (*mem.offset(def_ref as isize)).b32.s1;
                (*mem.offset(def_ref as isize)).b32.s1 = q
            }
            if a as i32 >= 4i32 {
                geq_define(p, (113i32 + a as i32 % 4i32) as u16,
                           def_ref);
            } else {
                eq_define(p, (113i32 + a as i32 % 4i32) as u16,
                          def_ref);
            }
        }
        96 => {
            n = cur_chr;
            get_r_token();
            p = cur_cs;
            if n == 0i32 {
                loop  {
                    get_token();
                    if !(cur_cmd as i32 == 10i32) { break ; }
                }
                if cur_tok == 0x1800000i32 + '=' as i32 {
                    get_token();
                    if cur_cmd as i32 == 10i32 { get_token(); }
                }
            } else {
                get_token();
                q = cur_tok;
                get_token();
                back_input();
                cur_tok = q;
                back_input();
            }
            if cur_cmd as i32 >= 113i32 {
                let ref mut fresh12 = (*mem.offset(cur_chr as isize)).b32.s0;
                *fresh12 += 1
            } else if cur_cmd as i32 == 91i32 ||
                          cur_cmd as i32 == 72i32 {
                if cur_chr < 0i32 || cur_chr > 19i32 {
                    /* 19 = lo_mem_stat_max, I think */
                    let ref mut fresh13 =
                        (*mem.offset((cur_chr + 1i32) as isize)).b32.s0;
                    *fresh13 += 1
                }
            }
            if a as i32 >= 4i32 {
                geq_define(p, cur_cmd as u16, cur_chr);
            } else { eq_define(p, cur_cmd as u16, cur_chr); }
        }
        97 => {
            if cur_chr == 7i32 {
                scan_char_num();
                p =
                    1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32
                        + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                        256i32 + 256i32 + 13i32 + 256i32 + 4i32 + 256i32 +
                        1i32 + 3i32 * 256i32 + (0x10ffffi32 + 1i32) +
                        (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                        (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + cur_val;
                scan_optional_equals();
                scan_char_num();
                n = cur_val;
                scan_char_num();
                if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                      12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                      256i32 + 256i32 + 13i32 + 256i32 + 4i32
                                      + 256i32 + 1i32 + 3i32 * 256i32 +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) + 57i32) as
                                     isize)).b32.s1 > 0i32 {
                    begin_diagnostic();
                    print_nl_cstr(b"New character substitution: \x00" as
                                      *const u8 as *const i8);
                    print(p -
                              (1i32 + (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                   12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                   256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                                   256i32 + 1i32 + 3i32 * 256i32 +
                                   (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32)
                                   + (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32)));
                    print_cstr(b" = \x00" as *const u8 as
                                   *const i8);
                    print(n);
                    print_char(' ' as i32);
                    print(cur_val);
                    end_diagnostic(0i32 != 0);
                }
                n = n * 256i32 + cur_val;
                if a as i32 >= 4i32 {
                    geq_define(p, 122_u16, n);
                } else { eq_define(p, 122_u16, n); }
                if p -
                       (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                            1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 +
                            19i32 + 256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                            256i32 + 1i32 + 3i32 * 256i32 +
                            (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                            (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                            (0x10ffffi32 + 1i32)) <
                       (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) + 1i32 +
                                          15000i32 + 12i32 + 9000i32 + 1i32 +
                                          1i32 + 19i32 + 256i32 + 256i32 +
                                          13i32 + 256i32 + 4i32 + 256i32 +
                                          1i32 + 3i32 * 256i32 +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) + 55i32) as
                                         isize)).b32.s1 {
                    if a as i32 >= 4i32 {
                        geq_word_define(1i32 + (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) + 1i32 +
                                            15000i32 + 12i32 + 9000i32 + 1i32
                                            + 1i32 + 19i32 + 256i32 + 256i32 +
                                            13i32 + 256i32 + 4i32 + 256i32 +
                                            1i32 + 3i32 * 256i32 +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) + 55i32,
                                        p -
                                            (1i32 + (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) + 1i32 +
                                                 15000i32 + 12i32 + 9000i32 +
                                                 1i32 + 1i32 + 19i32 + 256i32
                                                 + 256i32 + 13i32 + 256i32 +
                                                 4i32 + 256i32 + 1i32 +
                                                 3i32 * 256i32 +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32)));
                    } else {
                        eq_word_define(1i32 + (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) + 1i32 +
                                           15000i32 + 12i32 + 9000i32 + 1i32 +
                                           1i32 + 19i32 + 256i32 + 256i32 +
                                           13i32 + 256i32 + 4i32 + 256i32 +
                                           1i32 + 3i32 * 256i32 +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) + 55i32,
                                       p -
                                           (1i32 + (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) + 1i32 +
                                                15000i32 + 12i32 + 9000i32 +
                                                1i32 + 1i32 + 19i32 + 256i32 +
                                                256i32 + 13i32 + 256i32 + 4i32
                                                + 256i32 + 1i32 +
                                                3i32 * 256i32 +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32)));
                    }
                }
                if p -
                       (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                            1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 +
                            19i32 + 256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                            256i32 + 1i32 + 3i32 * 256i32 +
                            (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                            (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                            (0x10ffffi32 + 1i32)) >
                       (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) + 1i32 +
                                          15000i32 + 12i32 + 9000i32 + 1i32 +
                                          1i32 + 19i32 + 256i32 + 256i32 +
                                          13i32 + 256i32 + 4i32 + 256i32 +
                                          1i32 + 3i32 * 256i32 +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) + 56i32) as
                                         isize)).b32.s1 {
                    if a as i32 >= 4i32 {
                        geq_word_define(1i32 + (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) + 1i32 +
                                            15000i32 + 12i32 + 9000i32 + 1i32
                                            + 1i32 + 19i32 + 256i32 + 256i32 +
                                            13i32 + 256i32 + 4i32 + 256i32 +
                                            1i32 + 3i32 * 256i32 +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) +
                                            (0x10ffffi32 + 1i32) + 56i32,
                                        p -
                                            (1i32 + (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) + 1i32 +
                                                 15000i32 + 12i32 + 9000i32 +
                                                 1i32 + 1i32 + 19i32 + 256i32
                                                 + 256i32 + 13i32 + 256i32 +
                                                 4i32 + 256i32 + 1i32 +
                                                 3i32 * 256i32 +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32)));
                    } else {
                        eq_word_define(1i32 + (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) + 1i32 +
                                           15000i32 + 12i32 + 9000i32 + 1i32 +
                                           1i32 + 19i32 + 256i32 + 256i32 +
                                           13i32 + 256i32 + 4i32 + 256i32 +
                                           1i32 + 3i32 * 256i32 +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) + 56i32,
                                       p -
                                           (1i32 + (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) + 1i32 +
                                                15000i32 + 12i32 + 9000i32 +
                                                1i32 + 1i32 + 19i32 + 256i32 +
                                                256i32 + 13i32 + 256i32 + 4i32
                                                + 256i32 + 1i32 +
                                                3i32 * 256i32 +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32)));
                    }
                }
            } else {
                n = cur_chr;
                get_r_token();
                p = cur_cs;
                if a as i32 >= 4i32 {
                    geq_define(p, 0_u16, 0x10ffffi32 + 1i32);
                } else { eq_define(p, 0_u16, 0x10ffffi32 + 1i32); }
                scan_optional_equals();
                match n {
                    0 => {
                        scan_usv_num();
                        if a as i32 >= 4i32 {
                            geq_define(p, 68_u16, cur_val);
                        } else { eq_define(p, 68_u16, cur_val); }
                    }
                    1 => {
                        scan_fifteen_bit_int();
                        if a as i32 >= 4i32 {
                            geq_define(p, 69_u16, cur_val);
                        } else { eq_define(p, 69_u16, cur_val); }
                    }
                    8 => {
                        scan_xetex_math_char_int();
                        if a as i32 >= 4i32 {
                            geq_define(p, 70_u16, cur_val);
                        } else { eq_define(p, 70_u16, cur_val); }
                    }
                    9 => {
                        scan_math_class_int();
                        n =
                            ((cur_val as u32 &
                                  0x7_u32) << 21i32) as
                                i32;
                        scan_math_fam_int();
                        n =
                            (n as
                                 u32).wrapping_add((cur_val as
                                                                 u32
                                                                 &
                                                                 0xffi32 as
                                                                     u32)
                                                                << 24i32) as
                                i32;
                        scan_usv_num();
                        n = n + cur_val;
                        if a as i32 >= 4i32 {
                            geq_define(p, 70_u16, n);
                        } else { eq_define(p, 70_u16, n); }
                    }
                    _ => {
                        scan_register_num();
                        if cur_val > 255i32 {
                            j = n - 2i32;
                            if j > 3i32 { j = 5i32 }
                            find_sa_element(j as small_number, cur_val,
                                            true);
                            let ref mut fresh14 =
                                (*mem.offset((cur_ptr + 1i32) as
                                                 isize)).b32.s0;
                            *fresh14 += 1;
                            if j == 5i32 { j = 72i32 } else { j = 91i32 }
                            if a as i32 >= 4i32 {
                                geq_define(p, j as u16, cur_ptr);
                            } else { eq_define(p, j as u16, cur_ptr); }
                        } else {
                            match n {
                                2 => {
                                    if a as i32 >= 4i32 {
                                        geq_define(p, 74_u16,
                                                   1i32 + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 1i32 + 15000i32 +
                                                       12i32 + 9000i32 + 1i32
                                                       + 1i32 + 19i32 + 256i32
                                                       + 256i32 + 13i32 +
                                                       256i32 + 4i32 + 256i32
                                                       + 1i32 + 3i32 * 256i32
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 85i32 + cur_val);
                                    } else {
                                        eq_define(p, 74_u16,
                                                  1i32 + (0x10ffffi32 + 1i32)
                                                      + (0x10ffffi32 + 1i32) +
                                                      1i32 + 15000i32 + 12i32
                                                      + 9000i32 + 1i32 + 1i32
                                                      + 19i32 + 256i32 +
                                                      256i32 + 13i32 + 256i32
                                                      + 4i32 + 256i32 + 1i32 +
                                                      3i32 * 256i32 +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      85i32 + cur_val);
                                    }
                                }
                                3 => {
                                    if a as i32 >= 4i32 {
                                        geq_define(p, 75_u16,
                                                   1i32 + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 1i32 + 15000i32 +
                                                       12i32 + 9000i32 + 1i32
                                                       + 1i32 + 19i32 + 256i32
                                                       + 256i32 + 13i32 +
                                                       256i32 + 4i32 + 256i32
                                                       + 1i32 + 3i32 * 256i32
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 85i32 + 256i32 +
                                                       (0x10ffffi32 + 1i32) +
                                                       23i32 + cur_val);
                                    } else {
                                        eq_define(p, 75_u16,
                                                  1i32 + (0x10ffffi32 + 1i32)
                                                      + (0x10ffffi32 + 1i32) +
                                                      1i32 + 15000i32 + 12i32
                                                      + 9000i32 + 1i32 + 1i32
                                                      + 19i32 + 256i32 +
                                                      256i32 + 13i32 + 256i32
                                                      + 4i32 + 256i32 + 1i32 +
                                                      3i32 * 256i32 +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      (0x10ffffi32 + 1i32) +
                                                      85i32 + 256i32 +
                                                      (0x10ffffi32 + 1i32) +
                                                      23i32 + cur_val);
                                    }
                                }
                                4 => {
                                    if a as i32 >= 4i32 {
                                        geq_define(p, 76_u16,
                                                   1i32 + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 1i32 + 15000i32 +
                                                       12i32 + 9000i32 + 1i32
                                                       + 1i32 + 19i32 +
                                                       cur_val);
                                    } else {
                                        eq_define(p, 76_u16,
                                                  1i32 + (0x10ffffi32 + 1i32)
                                                      + (0x10ffffi32 + 1i32) +
                                                      1i32 + 15000i32 + 12i32
                                                      + 9000i32 + 1i32 + 1i32
                                                      + 19i32 + cur_val);
                                    }
                                }
                                5 => {
                                    if a as i32 >= 4i32 {
                                        geq_define(p, 77_u16,
                                                   1i32 + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 1i32 + 15000i32 +
                                                       12i32 + 9000i32 + 1i32
                                                       + 1i32 + 19i32 + 256i32
                                                       + cur_val);
                                    } else {
                                        eq_define(p, 77_u16,
                                                  1i32 + (0x10ffffi32 + 1i32)
                                                      + (0x10ffffi32 + 1i32) +
                                                      1i32 + 15000i32 + 12i32
                                                      + 9000i32 + 1i32 + 1i32
                                                      + 19i32 + 256i32 +
                                                      cur_val);
                                    }
                                }
                                6 => {
                                    if a as i32 >= 4i32 {
                                        geq_define(p, 73_u16,
                                                   1i32 + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 1i32 + 15000i32 +
                                                       12i32 + 9000i32 + 1i32
                                                       + 1i32 + 19i32 + 256i32
                                                       + 256i32 + 13i32 +
                                                       cur_val);
                                    } else {
                                        eq_define(p, 73_u16,
                                                  1i32 + (0x10ffffi32 + 1i32)
                                                      + (0x10ffffi32 + 1i32) +
                                                      1i32 + 15000i32 + 12i32
                                                      + 9000i32 + 1i32 + 1i32
                                                      + 19i32 + 256i32 +
                                                      256i32 + 13i32 +
                                                      cur_val);
                                    }
                                }
                                _ => { }
                            }
                        }
                    }
                }
            }
        }
        98 => {
            j = cur_chr;
            scan_int();
            n = cur_val;
            if !scan_keyword(b"to\x00" as *const u8 as *const i8) {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as
                                      *const i8);
                }
                print_cstr(b"Missing `to\' inserted\x00" as *const u8 as
                               *const i8);
                help_ptr = 2_u8;
                help_line[1] =
                    b"You should have said `\\read<number> to \\cs\'.\x00" as
                        *const u8 as *const i8;
                help_line[0] =
                    b"I\'m going to look for the \\cs now.\x00" as *const u8
                        as *const i8;
                error();
            }
            get_r_token();
            p = cur_cs;
            read_toks(n, p, j);
            if a as i32 >= 4i32 {
                geq_define(p, 113_u16, cur_val);
            } else { eq_define(p, 113_u16, cur_val); }
        }
        72 | 73 => {
            q = cur_cs;
            e = false;
            if cur_cmd as i32 == 72i32 {
                if cur_chr == 0i32 {
                    scan_register_num();
                    if cur_val > 255i32 {
                        find_sa_element(5i32 as small_number, cur_val,
                                        true);
                        cur_chr = cur_ptr;
                        e = true
                    } else {
                        cur_chr =
                            1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32)
                                + 1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 +
                                1i32 + 19i32 + 256i32 + 256i32 + 13i32 +
                                cur_val
                    }
                } else { e = true }
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 11i32 {
                scan_char_class_not_ignored();
                cur_ptr = cur_val;
                scan_char_class_not_ignored();
                find_sa_element(6i32 as small_number,
                                cur_ptr * 4096i32 + cur_val, true);
                cur_chr = cur_ptr;
                e = true
            }
            p = cur_chr;
            scan_optional_equals();
            loop  {
                get_x_token();
                if !(cur_cmd as i32 == 10i32 ||
                         cur_cmd as i32 == 0i32) {
                    break ;
                }
            }
            if cur_cmd as i32 != 1i32 {
                /*1262:*/
                if cur_cmd as i32 == 72i32 ||
                       cur_cmd as i32 == 73i32 {
                    if cur_cmd as i32 == 72i32 {
                        if cur_chr == 0i32 {
                            scan_register_num(); /* "extended delimiter code flag" */
                            if cur_val < 256i32 {
                                q =
                                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32)
                                                       + (0x10ffffi32 + 1i32)
                                                       + 1i32 + 15000i32 +
                                                       12i32 + 9000i32 + 1i32
                                                       + 1i32 + 19i32 + 256i32
                                                       + 256i32 + 13i32 +
                                                       cur_val) as
                                                      isize)).b32.s1
                            } else {
                                find_sa_element(5i32 as small_number, cur_val,
                                                0i32 !=
                                                    0); /* "extended delimiter code family */
                                if cur_ptr == -0xfffffffi32 {
                                    q = -0xfffffffi32
                                } else {
                                    q =
                                        (*mem.offset((cur_ptr + 1i32) as
                                                         isize)).b32.s1
                                }
                            }
                        } else {
                            q =
                                (*mem.offset((cur_chr + 1i32) as
                                                 isize)).b32.s1
                        }
                    } else if cur_chr ==
                                  1i32 + (0x10ffffi32 + 1i32) +
                                      (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                      12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                      256i32 + 256i32 + 11i32 {
                        scan_char_class_not_ignored(); /*:1268 */
                        cur_ptr = cur_val;
                        scan_char_class_not_ignored();
                        find_sa_element(6i32 as small_number,
                                        cur_ptr * 4096i32 + cur_val,
                                        false);
                        if cur_ptr == -0xfffffffi32 {
                            q = -0xfffffffi32
                        } else {
                            q =
                                (*mem.offset((cur_ptr + 1i32) as
                                                 isize)).b32.s1
                        }
                    } else { q = (*eqtb.offset(cur_chr as isize)).b32.s1 }
                    if q == -0xfffffffi32 {
                        if e {
                            if a as i32 >= 4i32 {
                                gsa_def(p, -0xfffffffi32);
                            } else { sa_def(p, -0xfffffffi32); }
                        } else if a as i32 >= 4i32 {
                            geq_define(p, 103_u16, -0xfffffffi32);
                        } else {
                            eq_define(p, 103_u16, -0xfffffffi32);
                        }
                    } else {
                        let ref mut fresh15 =
                            (*mem.offset(q as isize)).b32.s0;
                        *fresh15 += 1;
                        if e {
                            if a as i32 >= 4i32 {
                                gsa_def(p, q);
                            } else { sa_def(p, q); }
                        } else if a as i32 >= 4i32 {
                            geq_define(p, 113_u16, q);
                        } else { eq_define(p, 113_u16, q); }
                    }
                    current_block = 1862445865460439639;
                } else { current_block = 15174492983169363256; }
            } else { current_block = 15174492983169363256; }
            match current_block {
                1862445865460439639 => { }
                _ => {
                    back_input();
                    cur_cs = q;
                    q = scan_toks(false, false);
                    if (*mem.offset(def_ref as isize)).b32.s1 == -0xfffffffi32
                       {
                        if e {
                            if a as i32 >= 4i32 {
                                gsa_def(p, -0xfffffffi32);
                            } else { sa_def(p, -0xfffffffi32); }
                        } else if a as i32 >= 4i32 {
                            geq_define(p, 103_u16, -0xfffffffi32);
                        } else {
                            eq_define(p, 103_u16, -0xfffffffi32);
                        }
                        (*mem.offset(def_ref as isize)).b32.s1 = avail;
                        avail = def_ref
                    } else {
                        if p ==
                               1i32 + (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                   12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                   256i32 + 256i32 + 1i32 && !e {
                            (*mem.offset(q as isize)).b32.s1 = get_avail();
                            q = (*mem.offset(q as isize)).b32.s1;
                            (*mem.offset(q as isize)).b32.s0 =
                                0x400000i32 + 125i32;
                            q = get_avail();
                            (*mem.offset(q as isize)).b32.s0 =
                                0x200000i32 + 123i32;
                            (*mem.offset(q as isize)).b32.s1 =
                                (*mem.offset(def_ref as isize)).b32.s1;
                            (*mem.offset(def_ref as isize)).b32.s1 = q
                        }
                        if e {
                            if a as i32 >= 4i32 {
                                gsa_def(p, def_ref);
                            } else { sa_def(p, def_ref); }
                        } else if a as i32 >= 4i32 {
                            geq_define(p, 113_u16, def_ref);
                        } else { eq_define(p, 113_u16, def_ref); }
                    }
                }
            }
        }
        74 => {
            p = cur_chr;
            scan_optional_equals();
            scan_int();
            if a as i32 >= 4i32 {
                geq_word_define(p, cur_val);
            } else { eq_word_define(p, cur_val); }
        }
        75 => {
            p = cur_chr;
            scan_optional_equals();
            scan_dimen(false, false, false);
            if a as i32 >= 4i32 {
                geq_word_define(p, cur_val);
            } else { eq_word_define(p, cur_val); }
        }
        76 | 77 => {
            p = cur_chr;
            n = cur_cmd as i32;
            scan_optional_equals();
            if n == 77i32 {
                scan_glue(3i32 as small_number);
            } else { scan_glue(2i32 as small_number); }
            trap_zero_glue();
            if a as i32 >= 4i32 {
                geq_define(p, 119_u16, cur_val);
            } else { eq_define(p, 119_u16, cur_val); }
        }
        87 => {
            if cur_chr ==
                   1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 +
                       15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                       256i32 + 256i32 + 13i32 + 256i32 + 4i32 + 256i32 + 1i32
                       + 3i32 * 256i32 + (0x10ffffi32 + 1i32) +
                       (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) {
                p = cur_chr;
                scan_usv_num();
                p = p + cur_val;
                n =
                    ((*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                        (0x10ffffi32 + 1i32) + 1i32 + 15000i32
                                        + 12i32 + 9000i32 + 1i32 + 1i32 +
                                        19i32 + 256i32 + 256i32 + 13i32 +
                                        256i32 + 4i32 + 256i32 + 1i32 +
                                        3i32 * 256i32 + (0x10ffffi32 + 1i32) +
                                        (0x10ffffi32 + 1i32) +
                                        (0x10ffffi32 + 1i32) + cur_val) as
                                       isize)).b32.s1 as i64 %
                         65536) as i32;
                scan_optional_equals();
                scan_char_class();
                if a as i32 >= 4i32 {
                    geq_define(p, 122_u16,
                               (cur_val as i64 * 65536 +
                                    n as i64) as i32);
                } else {
                    eq_define(p, 122_u16,
                              (cur_val as i64 * 65536 +
                                   n as i64) as i32);
                }
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) {
                p = cur_chr;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                scan_xetex_math_char_int();
                if a as i32 >= 4i32 {
                    geq_define(p, 122_u16, cur_val);
                } else { eq_define(p, 122_u16, cur_val); }
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 {
                p = cur_chr - 1i32;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                scan_math_class_int();
                n =
                    ((cur_val as u32 & 0x7_u32) <<
                         21i32) as i32;
                scan_math_fam_int();
                n =
                    (n as
                         u32).wrapping_add((cur_val as u32 &
                                                         0xffi32 as
                                                             u32) <<
                                                        24i32) as i32;
                scan_usv_num();
                n = n + cur_val;
                if a as i32 >= 4i32 {
                    geq_define(p, 122_u16, n);
                } else { eq_define(p, 122_u16, n); }
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              85i32 + 256i32 {
                p = cur_chr;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                scan_int();
                if a as i32 >= 4i32 {
                    geq_word_define(p, cur_val);
                } else { eq_word_define(p, cur_val); }
            } else {
                p = cur_chr - 1i32;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                n = 0x40000000i32;
                scan_math_fam_int();
                n = n + cur_val * 0x200000i32;
                scan_usv_num();
                n = n + cur_val;
                if a as i32 >= 4i32 {
                    geq_word_define(p, n);
                } else { eq_word_define(p, n); }
            }
        }
        86 => {
            if cur_chr ==
                   1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 +
                       15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                       256i32 + 256i32 + 13i32 + 256i32 + 4i32 + 256i32 + 1i32
                       + 3i32 * 256i32 {
                n = 15i32
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) {
                n = 0x8000i32
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) {
                n = 0x7fffi32
            } else if cur_chr ==
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              85i32 + 256i32 {
                n = 0xffffffi32
            } else { n = 0x10ffffi32 }
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_int();
            if cur_val < 0i32 &&
                   p <
                       1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 +
                           19i32 + 256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                           256i32 + 1i32 + 3i32 * 256i32 +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 85i32
                           + 256i32 || cur_val > n {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as
                                      *const i8);
                }
                print_cstr(b"Invalid code (\x00" as *const u8 as
                               *const i8);
                print_int(cur_val);
                if p <
                       1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 +
                           19i32 + 256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                           256i32 + 1i32 + 3i32 * 256i32 +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 85i32
                           + 256i32 {
                    print_cstr(b"), should be in the range 0..\x00" as
                                   *const u8 as *const i8);
                } else {
                    print_cstr(b"), should be at most \x00" as *const u8 as
                                   *const i8);
                }
                print_int(n);
                help_ptr = 1_u8;
                help_line[0] =
                    b"I\'m going to use 0 instead of that illegal code value.\x00"
                        as *const u8 as *const i8;
                error();
                cur_val = 0i32
            }
            if p <
                   1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 +
                       15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                       256i32 + 256i32 + 13i32 + 256i32 + 4i32 + 256i32 + 1i32
                       + 3i32 * 256i32 + (0x10ffffi32 + 1i32) +
                       (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                       (0x10ffffi32 + 1i32) {
                if p >=
                       1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32 +
                           19i32 + 256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                           256i32 + 1i32 + 3i32 * 256i32 +
                           (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                           (0x10ffffi32 + 1i32) {
                    n =
                        ((*eqtb.offset(p as isize)).b32.s1 as i64 /
                             65536) as i32;
                    if a as i32 >= 4i32 {
                        geq_define(p, 122_u16,
                                   (n as i64 * 65536 +
                                        cur_val as i64) as i32);
                    } else {
                        eq_define(p, 122_u16,
                                  (n as i64 * 65536 +
                                       cur_val as i64) as i32);
                    }
                } else if a as i32 >= 4i32 {
                    geq_define(p, 122_u16, cur_val);
                } else { eq_define(p, 122_u16, cur_val); }
            } else if p <
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              85i32 + 256i32 {
                if cur_val as i64 == 32768 {
                    cur_val = 0x1fffffi32
                } else {
                    cur_val =
                        (((cur_val / 4096i32) as u32 &
                              0x7_u32) <<
                             21i32).wrapping_add(((cur_val % 4096i32 / 256i32)
                                                      as u32 &
                                                      0xff_u32)
                                                     <<
                                                     24i32).wrapping_add((cur_val
                                                                              %
                                                                              256i32)
                                                                             as
                                                                             u32)
                            as i32
                }
                if a as i32 >= 4i32 {
                    geq_define(p, 122_u16, cur_val);
                } else { eq_define(p, 122_u16, cur_val); }
            } else if a as i32 >= 4i32 {
                geq_word_define(p, cur_val);
            } else { eq_word_define(p, cur_val); }
        }
        88 => {
            p = cur_chr;
            scan_math_fam_int();
            p = p + cur_val;
            scan_optional_equals();
            scan_font_ident();
            if a as i32 >= 4i32 {
                geq_define(p, 122_u16, cur_val);
            } else { eq_define(p, 122_u16, cur_val); }
        }
        91 | 92 | 93 | 94 => { do_register_command(a); }
        100 => {
            scan_register_num();
            if a as i32 >= 4i32 {
                n = 0x40008000i32 + cur_val
            } else { n = 0x40000000i32 + cur_val }
            scan_optional_equals();
            if set_box_allowed {
                scan_box(n);
            } else {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as
                                      *const i8);
                }
                print_cstr(b"Improper \x00" as *const u8 as
                               *const i8);
                print_esc_cstr(b"setbox\x00" as *const u8 as
                                   *const i8);
                help_ptr = 2_u8;
                help_line[1] =
                    b"Sorry, \\setbox is not allowed after \\halign in a display,\x00"
                        as *const u8 as *const i8;
                help_line[0] =
                    b"or between \\accent and an accented character.\x00" as
                        *const u8 as *const i8;
                error();
            }
        }
        80 => { alter_aux(); }
        81 => { alter_prev_graf(); }
        82 => { alter_page_so_far(); }
        83 => { alter_integer(); }
        84 => { alter_box_dimen(); }
        85 => {
            q = cur_chr;
            scan_optional_equals();
            scan_int();
            n = cur_val;
            if n <= 0i32 {
                p = -0xfffffffi32
            } else if q >
                          1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 0i32 {
                n = cur_val / 2i32 + 1i32;
                p = get_node(2i32 * n + 1i32);
                (*mem.offset(p as isize)).b32.s0 = n;
                n = cur_val;
                (*mem.offset((p + 1i32) as isize)).b32.s1 = n;
                j = p + 2i32;
                while j <= p + n + 1i32 {
                    scan_int();
                    (*mem.offset(j as isize)).b32.s1 = cur_val;
                    j += 1
                }
                if n & 1i32 == 0 {
                    (*mem.offset((p + n + 2i32) as isize)).b32.s1 = 0i32
                }
            } else {
                p = get_node(2i32 * n + 1i32);
                (*mem.offset(p as isize)).b32.s0 = n;
                j = 1i32;
                while j <= n {
                    scan_dimen(false, false, false);
                    (*mem.offset((p + 2i32 * j - 1i32) as isize)).b32.s1 =
                        cur_val;
                    scan_dimen(false, false, false);
                    (*mem.offset((p + 2i32 * j) as isize)).b32.s1 = cur_val;
                    j += 1
                }
            }
            if a as i32 >= 4i32 {
                geq_define(q, 120_u16, p);
            } else { eq_define(q, 120_u16, p); }
        }
        101 => {
            if cur_chr == 1i32 {
                if in_initex_mode {
                    new_patterns();
                } else {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! \x00" as *const u8 as
                                          *const i8);
                    }
                    print_cstr(b"Patterns can be loaded only by INITEX\x00" as
                                   *const u8 as *const i8);
                    help_ptr = 0_u8;
                    error();
                    loop  {
                        get_token();
                        if !(cur_cmd as i32 != 2i32) { break ; }
                    }
                    return
                }
            } else { new_hyph_exceptions(); }
        }
        78 => {
            find_font_dimen(1i32 != 0);
            k = cur_val;
            scan_optional_equals();
            scan_dimen(false, false, false);
            (*font_info.offset(k as isize)).b32.s1 = cur_val
        }
        79 => {
            n = cur_chr;
            scan_font_ident();
            f = cur_val;
            if n < 2i32 {
                scan_optional_equals();
                scan_int();
                if n == 0i32 {
                    *hyphen_char.offset(f as isize) = cur_val
                } else { *skew_char.offset(f as isize) = cur_val }
            } else {
                if *font_area.offset(f as isize) as u32 == 0xffffu32
                       ||
                       *font_area.offset(f as isize) as u32 ==
                           0xfffeu32 {
                    scan_glyph_number(f);
                } else { scan_char_num(); }
                p = cur_val;
                scan_optional_equals();
                scan_int();
                match n {
                    2 => { set_cp_code(f, p as u32, 0i32, cur_val); }
                    3 => { set_cp_code(f, p as u32, 1i32, cur_val); }
                    _ => { }
                }
            }
        }
        90 => { new_font(a); }
        102 => { new_interaction(); }
        _ => { confusion(b"prefix\x00" as *const u8 as *const i8); }
    }
    /*1304:*/
    if after_token != 0i32 {
        cur_tok = after_token;
        back_input();
        after_token = 0i32
    };
}
/*:1328*/
/*1337:*/
unsafe extern "C" fn store_fmt_file() {
    let mut current_block: u64;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut x: i32 = 0;
    let mut fmt_out: rust_output_handle_t = 0 as *mut libc::c_void;
    if save_ptr != 0i32 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const i8);
        }
        print_cstr(b"You can\'t dump inside a group\x00" as *const u8 as *const i8);
        help_ptr = 1_u8;
        help_line[0] = b"`{...\\dump}\' is a no-no.\x00" as *const u8 as *const i8;
        if interaction as i32 == 3i32 {
            interaction = 2_u8
        }
        if log_opened {
            error();
        }
        history = TTHistory::FATAL_ERROR;
        close_files_and_terminate();
        ttstub_output_flush(rust_stdout);
        _tt_abort(b"\\dump inside a group\x00" as *const u8 as *const i8);
    }
    selector = Selector::NEW_STRING;
    print_cstr(b" (preloaded format=\x00" as *const u8 as *const i8);
    print(job_name);
    print_char(' ' as i32);
    print_int(
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 23i32) as isize,
        ))
        .b32
        .s1,
    );
    print_char('.' as i32);
    print_int(
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 22i32) as isize,
        ))
        .b32
        .s1,
    );
    print_char('.' as i32);
    print_int(
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 21i32) as isize,
        ))
        .b32
        .s1,
    );
    print_char(')' as i32);
    if interaction as i32 == 0i32 {
        selector = Selector::LOG_ONLY
    } else {
        selector = Selector::TERM_AND_LOG
    }
    if pool_ptr + 1i32 > pool_size {
        overflow(
            b"pool size\x00" as *const u8 as *const i8,
            pool_size - init_pool_ptr,
        );
    }
    format_ident = make_string();
    pack_job_name(b".fmt\x00" as *const u8 as *const i8);
    fmt_out = ttstub_output_open(name_of_file, 0i32);
    if fmt_out.is_null() {
        _tt_abort(
            b"cannot open format output file \"%s\"\x00" as *const u8 as *const i8,
            name_of_file,
        );
    }
    print_nl_cstr(b"Beginning to dump on file \x00" as *const u8 as *const i8);
    print(make_name_string());
    str_ptr -= 1;
    pool_ptr = *str_start.offset((str_ptr - 65536i32) as isize);
    print_nl_cstr(b"\x00" as *const u8 as *const i8);
    print(format_ident);
    /* Header */
    let mut x_val: i32 = 0x54544e43i32; /* TODO: can we move this farther up in this function? */
    do_dump(
        &mut x_val as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_0: i32 = 28i32;
    do_dump(
        &mut x_val_0 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_1: i32 = hash_high;
    do_dump(
        &mut x_val_1 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    while pseudo_files != -0xfffffffi32 {
        pseudo_close();
    }
    let mut x_val_2: i32 = 4999999i32;
    do_dump(
        &mut x_val_2 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_3: i32 = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 256i32
        + 1i32
        + 3i32 * 256i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 85i32
        + 256i32
        + (0x10ffffi32 + 1i32)
        + 23i32
        + 256i32
        - 1i32;
    do_dump(
        &mut x_val_3 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_4: i32 = 8501i32;
    do_dump(
        &mut x_val_4 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_5: i32 = 607i32;
    do_dump(
        &mut x_val_5 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    /* string pool */
    let mut x_val_6: i32 = pool_ptr;
    do_dump(
        &mut x_val_6 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_7: i32 = str_ptr;
    do_dump(
        &mut x_val_7 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    do_dump(
        &mut *str_start.offset(0) as *mut pool_pointer as *mut i8,
        ::std::mem::size_of::<pool_pointer>() as u64,
        (str_ptr - 65536i32 + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *str_pool.offset(0) as *mut packed_UTF16_code as *mut i8,
        ::std::mem::size_of::<packed_UTF16_code>() as u64,
        pool_ptr as size_t,
        fmt_out,
    );
    print_ln();
    print_int(str_ptr);
    print_cstr(b" strings of total length \x00" as *const u8 as *const i8);
    print_int(pool_ptr);
    /* "memory locations" */
    sort_avail();
    var_used = 0i32;
    let mut x_val_8: i32 = lo_mem_max;
    do_dump(
        &mut x_val_8 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_9: i32 = rover;
    do_dump(
        &mut x_val_9 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    k = 0i32;
    while k <= 6i32 {
        let mut x_val_10: i32 = sa_root[k as usize];
        do_dump(
            &mut x_val_10 as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        k += 1
    }
    p = 0i32;
    q = rover;
    x = 0i32;
    loop {
        do_dump(
            &mut *mem.offset(p as isize) as *mut memory_word as *mut i8,
            ::std::mem::size_of::<memory_word>() as u64,
            (q + 2i32 - p) as size_t,
            fmt_out,
        );
        x = x + q + 2i32 - p;
        var_used = var_used + q - p;
        p = q + (*mem.offset(q as isize)).b32.s0;
        q = (*mem.offset((q + 1i32) as isize)).b32.s1;
        if !(q != rover) {
            break;
        }
    }
    var_used = var_used + lo_mem_max - p;
    dyn_used = mem_end + 1i32 - hi_mem_min;
    do_dump(
        &mut *mem.offset(p as isize) as *mut memory_word as *mut i8,
        ::std::mem::size_of::<memory_word>() as u64,
        (lo_mem_max + 1i32 - p) as size_t,
        fmt_out,
    );
    x = x + lo_mem_max + 1i32 - p;
    let mut x_val_11: i32 = hi_mem_min;
    do_dump(
        &mut x_val_11 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_12: i32 = avail;
    do_dump(
        &mut x_val_12 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    do_dump(
        &mut *mem.offset(hi_mem_min as isize) as *mut memory_word as *mut i8,
        ::std::mem::size_of::<memory_word>() as u64,
        (mem_end + 1i32 - hi_mem_min) as size_t,
        fmt_out,
    );
    x = x + mem_end + 1i32 - hi_mem_min;
    p = avail;
    while p != -0xfffffffi32 {
        dyn_used -= 1;
        p = (*mem.offset(p as isize)).b32.s1
    }
    let mut x_val_13: i32 = var_used;
    do_dump(
        &mut x_val_13 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_14: i32 = dyn_used;
    do_dump(
        &mut x_val_14 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    print_ln();
    print_int(x);
    print_cstr(b" memory locations dumped; current usage is \x00" as *const u8 as *const i8);
    print_int(var_used);
    print_char('&' as i32);
    print_int(dyn_used);
    /* equivalents table / primitive */
    k = 1i32; /*:1350*/
    loop {
        j = k;
        loop {
            if !(j < 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                - 1i32)
            {
                current_block = 7923086311623215889;
                break;
            }
            if (*eqtb.offset(j as isize)).b32.s1 == (*eqtb.offset((j + 1i32) as isize)).b32.s1
                && (*eqtb.offset(j as isize)).b16.s1 as i32
                    == (*eqtb.offset((j + 1i32) as isize)).b16.s1 as i32
                && (*eqtb.offset(j as isize)).b16.s0 as i32
                    == (*eqtb.offset((j + 1i32) as isize)).b16.s0 as i32
            {
                current_block = 8379985486002839332;
                break;
            }
            j += 1
        }
        match current_block {
            7923086311623215889 => {
                l = 1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
            }
            _ => {
                j += 1;
                l = j;
                while j < 1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    - 1i32
                {
                    if (*eqtb.offset(j as isize)).b32.s1
                        != (*eqtb.offset((j + 1i32) as isize)).b32.s1
                        || (*eqtb.offset(j as isize)).b16.s1 as i32
                            != (*eqtb.offset((j + 1i32) as isize)).b16.s1 as i32
                        || (*eqtb.offset(j as isize)).b16.s0 as i32
                            != (*eqtb.offset((j + 1i32) as isize)).b16.s0 as i32
                    {
                        break;
                    }
                    j += 1
                }
            }
        }
        let mut x_val_15: i32 = l - k;
        do_dump(
            &mut x_val_15 as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        do_dump(
            &mut *eqtb.offset(k as isize) as *mut memory_word as *mut i8,
            ::std::mem::size_of::<memory_word>() as u64,
            (l - k) as size_t,
            fmt_out,
        );
        k = j + 1i32;
        let mut x_val_16: i32 = k - l;
        do_dump(
            &mut x_val_16 as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        if !(k
            != 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32))
        {
            break;
        }
    }
    loop {
        j = k;
        loop {
            if !(j < 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 85i32
                + 256i32
                + (0x10ffffi32 + 1i32)
                + 23i32
                + 256i32
                - 1i32)
            {
                current_block = 10505255564575309249;
                break;
            }
            if (*eqtb.offset(j as isize)).b32.s1 == (*eqtb.offset((j + 1i32) as isize)).b32.s1 {
                current_block = 18329769178042496632;
                break;
            }
            j += 1
        }
        match current_block {
            10505255564575309249 => {
                l = 1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 85i32
                    + 256i32
                    + (0x10ffffi32 + 1i32)
                    + 23i32
                    + 256i32
                    - 1i32
                    + 1i32
            }
            _ => {
                j += 1;
                l = j;
                while j < 1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 85i32
                    + 256i32
                    + (0x10ffffi32 + 1i32)
                    + 23i32
                    + 256i32
                    - 1i32
                {
                    if (*eqtb.offset(j as isize)).b32.s1
                        != (*eqtb.offset((j + 1i32) as isize)).b32.s1
                    {
                        break;
                    }
                    j += 1
                }
            }
        }
        let mut x_val_17: i32 = l - k;
        do_dump(
            &mut x_val_17 as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        do_dump(
            &mut *eqtb.offset(k as isize) as *mut memory_word as *mut i8,
            ::std::mem::size_of::<memory_word>() as u64,
            (l - k) as size_t,
            fmt_out,
        );
        k = j + 1i32;
        let mut x_val_18: i32 = k - l;
        do_dump(
            &mut x_val_18 as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        if !(k
            <= 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 85i32
                + 256i32
                + (0x10ffffi32 + 1i32)
                + 23i32
                + 256i32
                - 1i32)
        {
            break;
        }
    }
    if hash_high > 0i32 {
        do_dump(
            &mut *eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 85i32
                    + 256i32
                    + (0x10ffffi32 + 1i32)
                    + 23i32
                    + 256i32
                    - 1i32
                    + 1i32) as isize,
            ) as *mut memory_word as *mut i8,
            ::std::mem::size_of::<memory_word>() as u64,
            hash_high as size_t,
            fmt_out,
        );
    }
    let mut x_val_19: i32 = par_loc;
    do_dump(
        &mut x_val_19 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_20: i32 = write_loc;
    do_dump(
        &mut x_val_20 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    p = 0i32;
    while p <= 500i32 {
        do_dump(
            &mut *prim.as_mut_ptr().offset(p as isize) as *mut b32x2 as *mut i8,
            ::std::mem::size_of::<b32x2>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        p += 1
    }
    p = 0i32;
    while p <= 500i32 {
        do_dump(
            &mut *prim_eqtb.as_mut_ptr().offset(p as isize) as *mut memory_word as *mut i8,
            ::std::mem::size_of::<memory_word>() as u64,
            1i32 as size_t,
            fmt_out,
        );
        p += 1
    }
    /* control sequences */
    let mut x_val_21: i32 = hash_used;
    do_dump(
        &mut x_val_21 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    cs_count =
        1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 - 1i32 - hash_used
            + hash_high;
    p = 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32;
    while p <= hash_used {
        if (*hash.offset(p as isize)).s1 != 0i32 {
            let mut x_val_22: i32 = p;
            do_dump(
                &mut x_val_22 as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_out,
            );
            do_dump(
                &mut *hash.offset(p as isize) as *mut b32x2 as *mut i8,
                ::std::mem::size_of::<b32x2>() as u64,
                1i32 as size_t,
                fmt_out,
            );
            cs_count += 1
        }
        p += 1
    }
    do_dump(
        &mut *hash.offset((hash_used + 1i32) as isize) as *mut b32x2 as *mut i8,
        ::std::mem::size_of::<b32x2>() as u64,
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            - 1i32
            - hash_used) as size_t,
        fmt_out,
    );
    if hash_high > 0i32 {
        do_dump(
            &mut *hash.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 85i32
                    + 256i32
                    + (0x10ffffi32 + 1i32)
                    + 23i32
                    + 256i32
                    - 1i32
                    + 1i32) as isize,
            ) as *mut b32x2 as *mut i8,
            ::std::mem::size_of::<b32x2>() as u64,
            hash_high as size_t,
            fmt_out,
        );
    }
    let mut x_val_23: i32 = cs_count;
    do_dump(
        &mut x_val_23 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    print_ln();
    print_int(cs_count);
    print_cstr(b" multiletter control sequences\x00" as *const u8 as *const i8);
    /* fonts */
    let mut x_val_24: i32 = fmem_ptr;
    do_dump(
        &mut x_val_24 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_info.offset(0) as *mut memory_word as *mut i8,
        ::std::mem::size_of::<memory_word>() as u64,
        fmem_ptr as size_t,
        fmt_out,
    );
    let mut x_val_25: i32 = font_ptr;
    do_dump(
        &mut x_val_25 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_check.offset(0) as *mut b16x4 as *mut i8,
        ::std::mem::size_of::<b16x4>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_size.offset(0) as *mut scaled_t as *mut i8,
        ::std::mem::size_of::<scaled_t>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_dsize.offset(0) as *mut scaled_t as *mut i8,
        ::std::mem::size_of::<scaled_t>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_params.offset(0) as *mut font_index as *mut i8,
        ::std::mem::size_of::<font_index>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *hyphen_char.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *skew_char.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_name.offset(0) as *mut str_number as *mut i8,
        ::std::mem::size_of::<str_number>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_area.offset(0) as *mut str_number as *mut i8,
        ::std::mem::size_of::<str_number>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_bc.offset(0) as *mut UTF16_code as *mut i8,
        ::std::mem::size_of::<UTF16_code>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_ec.offset(0) as *mut UTF16_code as *mut i8,
        ::std::mem::size_of::<UTF16_code>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *char_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *width_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *height_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *depth_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *italic_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *lig_kern_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *kern_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *exten_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *param_base.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_glue.offset(0) as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *bchar_label.offset(0) as *mut font_index as *mut i8,
        ::std::mem::size_of::<font_index>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_bchar.offset(0) as *mut nine_bits as *mut i8,
        ::std::mem::size_of::<nine_bits>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *font_false_bchar.offset(0) as *mut nine_bits as *mut i8,
        ::std::mem::size_of::<nine_bits>() as u64,
        (font_ptr + 1i32) as size_t,
        fmt_out,
    );
    k = 0i32;
    while k <= font_ptr {
        print_nl_cstr(b"\\font\x00" as *const u8 as *const i8);
        print_esc(
            (*hash.offset(
                (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 12i32 + k)
                    as isize,
            ))
            .s1,
        );
        print_char('=' as i32);
        if *font_area.offset(k as isize) as u32 == 0xffffu32
            || *font_area.offset(k as isize) as u32 == 0xfffeu32
            || !(*font_mapping.offset(k as isize)).is_null()
        {
            print_file_name(
                *font_name.offset(k as isize),
                (65536 + 1i32 as i64) as i32,
                (65536 + 1i32 as i64) as i32,
            );
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const i8);
            }
            print_cstr(
                b"Can\'t \\dump a format with native fonts or font-mappings\x00" as *const u8
                    as *const i8,
            );
            help_ptr = 3_u8;
            help_line[2] =
                b"You really, really don\'t want to do this.\x00" as *const u8 as *const i8;
            help_line[1] = b"It won\'t work, and only confuses me.\x00" as *const u8 as *const i8;
            help_line[0] = b"(Load them at runtime, not as part of the format file.)\x00"
                as *const u8 as *const i8;
            error();
        } else {
            print_file_name(
                *font_name.offset(k as isize),
                *font_area.offset(k as isize),
                (65536 + 1i32 as i64) as i32,
            );
        }
        if *font_size.offset(k as isize) != *font_dsize.offset(k as isize) {
            print_cstr(b" at \x00" as *const u8 as *const i8);
            print_scaled(*font_size.offset(k as isize));
            print_cstr(b"pt\x00" as *const u8 as *const i8);
        }
        k += 1
    }
    print_ln();
    print_int(fmem_ptr - 7i32);
    print_cstr(b" words of font info for \x00" as *const u8 as *const i8);
    print_int(font_ptr - 0i32);
    if font_ptr != 0i32 + 1i32 {
        print_cstr(b" preloaded fonts\x00" as *const u8 as *const i8);
    } else {
        print_cstr(b" preloaded font\x00" as *const u8 as *const i8);
    }
    /* hyphenation info */
    let mut x_val_26: i32 = hyph_count;
    do_dump(
        &mut x_val_26 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    if hyph_next <= 607i32 {
        hyph_next = hyph_size
    }
    let mut x_val_27: i32 = hyph_next;
    do_dump(
        &mut x_val_27 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    k = 0i32;
    while k <= hyph_size {
        if *hyph_word.offset(k as isize) != 0i32 {
            let mut x_val_28: i32 =
                (k as i64 + 65536 * *hyph_link.offset(k as isize) as i64) as i32;
            do_dump(
                &mut x_val_28 as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_out,
            );
            let mut x_val_29: i32 = *hyph_word.offset(k as isize);
            do_dump(
                &mut x_val_29 as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_out,
            );
            let mut x_val_30: i32 = *hyph_list.offset(k as isize);
            do_dump(
                &mut x_val_30 as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_out,
            );
        }
        k += 1
    }
    print_ln();
    print_int(hyph_count);
    if hyph_count != 1i32 {
        print_cstr(b" hyphenation exceptions\x00" as *const u8 as *const i8);
    } else {
        print_cstr(b" hyphenation exception\x00" as *const u8 as *const i8);
    }
    if trie_not_ready {
        init_trie();
    }
    let mut x_val_31: i32 = trie_max;
    do_dump(
        &mut x_val_31 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_32: i32 = hyph_start;
    do_dump(
        &mut x_val_32 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    do_dump(
        &mut *trie_trl.offset(0) as *mut trie_pointer as *mut i8,
        ::std::mem::size_of::<trie_pointer>() as u64,
        (trie_max + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *trie_tro.offset(0) as *mut trie_pointer as *mut i8,
        ::std::mem::size_of::<trie_pointer>() as u64,
        (trie_max + 1i32) as size_t,
        fmt_out,
    );
    do_dump(
        &mut *trie_trc.offset(0) as *mut u16 as *mut i8,
        ::std::mem::size_of::<u16>() as u64,
        (trie_max + 1i32) as size_t,
        fmt_out,
    );
    let mut x_val_33: i32 = max_hyph_char;
    do_dump(
        &mut x_val_33 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    let mut x_val_34: i32 = trie_op_ptr;
    do_dump(
        &mut x_val_34 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    do_dump(
        &mut *hyf_distance.as_mut_ptr().offset(1) as *mut small_number as *mut i8,
        ::std::mem::size_of::<small_number>() as u64,
        trie_op_ptr as size_t,
        fmt_out,
    );
    do_dump(
        &mut *hyf_num.as_mut_ptr().offset(1) as *mut small_number as *mut i8,
        ::std::mem::size_of::<small_number>() as u64,
        trie_op_ptr as size_t,
        fmt_out,
    );
    do_dump(
        &mut *hyf_next.as_mut_ptr().offset(1) as *mut trie_opcode as *mut i8,
        ::std::mem::size_of::<trie_opcode>() as u64,
        trie_op_ptr as size_t,
        fmt_out,
    );
    print_nl_cstr(b"Hyphenation trie of length \x00" as *const u8 as *const i8);
    print_int(trie_max);
    print_cstr(b" has \x00" as *const u8 as *const i8);
    print_int(trie_op_ptr);
    if trie_op_ptr != 1i32 {
        print_cstr(b" ops\x00" as *const u8 as *const i8);
    } else {
        print_cstr(b" op\x00" as *const u8 as *const i8);
    }
    print_cstr(b" out of \x00" as *const u8 as *const i8);
    print_int(35111 as i32);
    k = 255i32;
    while k >= 0i32 {
        if trie_used[k as usize] as i32 > 0i32 {
            print_nl_cstr(b"  \x00" as *const u8 as *const i8);
            print_int(trie_used[k as usize] as i32);
            print_cstr(b" for language \x00" as *const u8 as *const i8);
            print_int(k);
            let mut x_val_35: i32 = k;
            do_dump(
                &mut x_val_35 as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_out,
            );
            let mut x_val_36: i32 = trie_used[k as usize] as i32;
            do_dump(
                &mut x_val_36 as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_out,
            );
        }
        k -= 1
    }
    /* footer */
    let mut x_val_37: i32 = 0x29ai32; /*:1361*/
    do_dump(
        &mut x_val_37 as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_out,
    );
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 31i32) as isize,
    ))
    .b32
    .s1 = 0i32;
    ttstub_output_close(fmt_out);
}
unsafe extern "C" fn pack_buffered_name(mut n: small_number, mut a: i32, mut b: i32) {
    free(name_of_file as *mut libc::c_void);
    name_of_file = xmalloc(
        ((format_default_length + 1i32 + 1i32) as u64)
            .wrapping_mul(::std::mem::size_of::<UTF8_code>() as u64),
    ) as *mut i8;
    strcpy(name_of_file, TEX_format_default);
    name_length = strlen(name_of_file) as i32;
}
unsafe extern "C" fn load_fmt_file() -> bool {
    let mut current_block: u64;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut x: i32 = 0;
    let mut fmt_in: rust_input_handle_t = 0 as *mut libc::c_void;
    j = cur_input.loc;
    /* This is where a first line starting with "&" used to
     * trigger code that would change the format file. */
    pack_buffered_name((format_default_length - 4i32) as small_number, 1i32, 0i32);
    fmt_in = ttstub_input_open(name_of_file, TTInputFormat::FORMAT, 0i32);
    if fmt_in.is_null() {
        _tt_abort(
            b"cannot open the format file \"%s\"\x00" as *const u8 as *const i8,
            name_of_file,
        );
    }
    cur_input.loc = j;
    if in_initex_mode {
        free(font_info as *mut libc::c_void);
        free(str_pool as *mut libc::c_void);
        free(str_start as *mut libc::c_void);
        free(yhash as *mut libc::c_void);
        free(eqtb as *mut libc::c_void);
        free(mem as *mut libc::c_void);
        mem = 0 as *mut memory_word
    }
    /* start reading the header */
    do_undump(
        &mut x as *mut i32 as *mut i8,
        ::std::mem::size_of::<i32>() as u64,
        1i32 as size_t,
        fmt_in,
    );
    if !(x != 0x54544e43i32) {
        do_undump(
            &mut x as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_in,
        );
        if x != 28i32 {
            _tt_abort(
                b"format file \"%s\" is of the wrong version: expected %d, found %d\x00"
                    as *const u8 as *const i8,
                name_of_file,
                28i32,
                x,
            );
        }
        /* hash table parameters */
        do_undump(
            &mut hash_high as *mut i32 as *mut i8,
            ::std::mem::size_of::<i32>() as u64,
            1i32 as size_t,
            fmt_in,
        );
        if !(hash_high < 0i32 || hash_high as i64 > 2097151) {
            if hash_extra < hash_high {
                hash_extra = hash_high
            }
            eqtb_top = 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 85i32
                + 256i32
                + (0x10ffffi32 + 1i32)
                + 23i32
                + 256i32
                - 1i32
                + hash_extra;
            if hash_extra == 0i32 {
                hash_top = 1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
            } else {
                hash_top = eqtb_top
            }
            yhash = xmalloc(
                ((1i32 + hash_top - 514i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<b32x2>() as u64),
            ) as *mut b32x2;
            hash = yhash.offset(-514);
            (*hash.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32) as isize))
                .s0 = 0i32;
            (*hash.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32) as isize))
                .s1 = 0i32;
            x = 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 1i32;
            while x <= hash_top {
                *hash.offset(x as isize) = *hash
                    .offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32) as isize);
                x += 1
            }
            eqtb = xmalloc(
                ((eqtb_top + 1i32 + 1i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<memory_word>() as u64),
            ) as *mut memory_word;
            (*eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32) as isize,
            ))
            .b16
            .s1 = 103_u16;
            (*eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32) as isize,
            ))
            .b32
            .s1 = -0xfffffffi32;
            (*eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32) as isize,
            ))
            .b16
            .s0 = 0_u16;
            x = 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 85i32
                + 256i32
                + (0x10ffffi32 + 1i32)
                + 23i32
                + 256i32
                - 1i32
                + 1i32;
            while x <= eqtb_top {
                *eqtb.offset(x as isize) = *eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32) as isize,
                );
                x += 1
            }
            max_reg_num = 32767i32;
            max_reg_help_line =
                b"A register number must be between 0 and 32767.\x00" as *const u8 as *const i8;
            /* "memory locations" */
            do_undump(
                &mut x as *mut i32 as *mut i8,
                ::std::mem::size_of::<i32>() as u64,
                1i32 as size_t,
                fmt_in,
            );
            if !(x != 4999999i32) {
                cur_list.head = 4999999i32 - 1i32;
                cur_list.tail = 4999999i32 - 1i32;
                page_tail = 4999999i32 - 2i32;
                mem = xmalloc(
                    ((4999999i32 + 1i32 + 1i32) as u64)
                        .wrapping_mul(::std::mem::size_of::<memory_word>() as u64),
                ) as *mut memory_word;
                do_undump(
                    &mut x as *mut i32 as *mut i8,
                    ::std::mem::size_of::<i32>() as u64,
                    1i32 as size_t,
                    fmt_in,
                );
                if !(x
                    != 1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 85i32
                        + 256i32
                        + (0x10ffffi32 + 1i32)
                        + 23i32
                        + 256i32
                        - 1i32)
                {
                    do_undump(
                        &mut x as *mut i32 as *mut i8,
                        ::std::mem::size_of::<i32>() as u64,
                        1i32 as size_t,
                        fmt_in,
                    );
                    if !(x != 8501i32) {
                        do_undump(
                            &mut x as *mut i32 as *mut i8,
                            ::std::mem::size_of::<i32>() as u64,
                            1i32 as size_t,
                            fmt_in,
                        );
                        if !(x != 607i32) {
                            /* string pool */
                            do_undump(
                                &mut x as *mut i32 as *mut i8,
                                ::std::mem::size_of::<i32>() as u64,
                                1i32 as size_t,
                                fmt_in,
                            ); /*:1345 */
                            if !(x < 0i32) {
                                if x as i64 > 40000000 - pool_free as i64 {
                                    _tt_abort(
                                        b"must increase string_pool_size\x00" as *const u8
                                            as *const i8,
                                    );
                                }
                                pool_ptr = x;
                                if pool_size < pool_ptr + pool_free {
                                    pool_size = pool_ptr + pool_free
                                }
                                do_undump(
                                    &mut x as *mut i32 as *mut i8,
                                    ::std::mem::size_of::<i32>() as u64,
                                    1i32 as size_t,
                                    fmt_in,
                                );
                                if !(x < 0i32) {
                                    if x as i64 > 2097151 - strings_free as i64 {
                                        _tt_abort(
                                            b"must increase sup_strings\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    str_ptr = x;
                                    if max_strings < str_ptr + strings_free {
                                        max_strings = str_ptr + strings_free
                                    }
                                    str_start =
                                        xmalloc(((max_strings + 1i32) as u64).wrapping_mul(
                                            ::std::mem::size_of::<pool_pointer>() as u64,
                                        ))
                                            as *mut pool_pointer;
                                    let mut i: i32 = 0;
                                    do_undump(
                                        &mut *str_start.offset(0) as *mut pool_pointer as *mut i8,
                                        ::std::mem::size_of::<pool_pointer>() as u64,
                                        (str_ptr - 65536i32 + 1i32) as size_t,
                                        fmt_in,
                                    );
                                    i = 0i32;
                                    while i < str_ptr - 65536i32 + 1i32 {
                                        if *(&mut *str_start.offset(0) as *mut pool_pointer)
                                            .offset(i as isize)
                                            < 0i32
                                            || *(&mut *str_start.offset(0) as *mut pool_pointer)
                                                .offset(i as isize)
                                                > pool_ptr
                                        {
                                            _tt_abort(b"item %u (=%ld) of .fmt array at %lx <%ld or >%ld\x00"
                                                          as *const u8 as
                                                          *const i8,
                                                      i,
                                                      *(&mut *str_start.offset(0)
                                                            as
                                                            *mut pool_pointer).offset(i
                                                                                          as
                                                                                          isize)
                                                          as uintptr_t,
                                                      &mut *str_start.offset(0)
                                                          as *mut pool_pointer
                                                          as uintptr_t,
                                                      0i32 as uintptr_t,
                                                      pool_ptr as uintptr_t);
                                        }
                                        i += 1
                                    }
                                    str_pool = xmalloc(((pool_size + 1i32) as u64).wrapping_mul(
                                        ::std::mem::size_of::<packed_UTF16_code>() as u64,
                                    ))
                                        as *mut packed_UTF16_code;
                                    do_undump(
                                        &mut *str_pool.offset(0) as *mut packed_UTF16_code
                                            as *mut i8,
                                        ::std::mem::size_of::<packed_UTF16_code>() as u64,
                                        pool_ptr as size_t,
                                        fmt_in,
                                    );
                                    init_str_ptr = str_ptr;
                                    init_pool_ptr = pool_ptr;
                                    /* "By sorting the list of available spaces in the variable-size portion
                                     * of |mem|, we are usually able to get by without having to dump very
                                     * much of the dynamic memory." */
                                    do_undump(
                                        &mut x as *mut i32 as *mut i8,
                                        ::std::mem::size_of::<i32>() as u64,
                                        1i32 as size_t,
                                        fmt_in,
                                    );
                                    if !(x < 1019i32 || x > 4999999i32 - 15i32) {
                                        lo_mem_max = x;
                                        do_undump(
                                            &mut x as *mut i32 as *mut i8,
                                            ::std::mem::size_of::<i32>() as u64,
                                            1i32 as size_t,
                                            fmt_in,
                                        );
                                        if !(x < 20i32 || x > lo_mem_max) {
                                            rover = x;
                                            k = 0i32;
                                            loop {
                                                if !(k <= 6i32) {
                                                    current_block = 1209030638129645089;
                                                    break;
                                                }
                                                do_undump(
                                                    &mut x as *mut i32 as *mut i8,
                                                    ::std::mem::size_of::<i32>() as u64,
                                                    1i32 as size_t,
                                                    fmt_in,
                                                );
                                                if x < -0xfffffffi32 || x > lo_mem_max {
                                                    current_block = 6442379788293543199;
                                                    break;
                                                }
                                                sa_root[k as usize] = x;
                                                k += 1
                                            }
                                            match current_block {
                                                6442379788293543199 => {}
                                                _ => {
                                                    p = 0i32;
                                                    q = rover;
                                                    loop {
                                                        do_undump(
                                                            &mut *mem.offset(p as isize)
                                                                as *mut memory_word
                                                                as *mut i8,
                                                            ::std::mem::size_of::<memory_word>()
                                                                as u64,
                                                            (q + 2i32 - p) as size_t,
                                                            fmt_in,
                                                        );
                                                        p = q + (*mem.offset(q as isize)).b32.s0;
                                                        if p > lo_mem_max
                                                            || q >= (*mem
                                                                .offset((q + 1i32) as isize))
                                                            .b32
                                                            .s1 && (*mem
                                                                .offset((q + 1i32) as isize))
                                                            .b32
                                                            .s1 != rover
                                                        {
                                                            current_block = 6442379788293543199;
                                                            break;
                                                        }
                                                        q = (*mem.offset((q + 1i32) as isize))
                                                            .b32
                                                            .s1;
                                                        if !(q != rover) {
                                                            current_block = 17395932908762866334;
                                                            break;
                                                        }
                                                    }
                                                    match current_block {
                                                        6442379788293543199 => {}
                                                        _ => {
                                                            do_undump(
                                                                &mut *mem.offset(p as isize)
                                                                    as *mut memory_word
                                                                    as *mut i8,
                                                                ::std::mem::size_of::<memory_word>()
                                                                    as u64,
                                                                (lo_mem_max + 1i32 - p) as size_t,
                                                                fmt_in,
                                                            );
                                                            do_undump(
                                                                &mut x as *mut i32 as *mut i8,
                                                                ::std::mem::size_of::<i32>() as u64,
                                                                1i32 as size_t,
                                                                fmt_in,
                                                            );
                                                            if !(x < lo_mem_max + 1i32
                                                                || x > 4999999i32 - 14i32)
                                                            {
                                                                hi_mem_min = x;
                                                                do_undump(
                                                                    &mut x as *mut i32 as *mut i8,
                                                                    ::std::mem::size_of::<i32>()
                                                                        as u64,
                                                                    1i32 as size_t,
                                                                    fmt_in,
                                                                );
                                                                if !(x < -0xfffffffi32
                                                                    || x > 4999999i32)
                                                                {
                                                                    avail = x;
                                                                    mem_end = 4999999i32;
                                                                    do_undump(
                                                                        &mut *mem.offset(
                                                                            hi_mem_min as isize,
                                                                        )
                                                                            as *mut memory_word
                                                                            as *mut i8,
                                                                        ::std::mem::size_of::<
                                                                            memory_word,
                                                                        >(
                                                                        )
                                                                            as u64,
                                                                        (mem_end + 1i32
                                                                            - hi_mem_min)
                                                                            as size_t,
                                                                        fmt_in,
                                                                    );
                                                                    do_undump(
                                                                        &mut var_used as *mut i32
                                                                            as *mut i8,
                                                                        ::std::mem::size_of::<i32>()
                                                                            as u64,
                                                                        1i32 as size_t,
                                                                        fmt_in,
                                                                    );
                                                                    do_undump(
                                                                        &mut dyn_used as *mut i32
                                                                            as *mut i8,
                                                                        ::std::mem::size_of::<i32>()
                                                                            as u64,
                                                                        1i32 as size_t,
                                                                        fmt_in,
                                                                    );
                                                                    /* equivalents table / primitives
                                                                     *
                                                                     * "The table of equivalents usually contains repeated information, so we
                                                                     * dump it in compressed form: The sequence of $n + 2$ values
                                                                     * $(n, x_1, \ldots, x_n, m)$ in the format file represents $n + m$ consecutive
                                                                     * entries of |eqtb|, with |m| extra copies of $x_n$, namely
                                                                     * $(x_1, \ldots, x_n, x_n, \ldots, x_n)$"
                                                                     */
                                                                    k = 1i32;
                                                                    loop {
                                                                        do_undump(
                                                                            &mut x as *mut i32
                                                                                as *mut i8,
                                                                            ::std::mem::size_of::<i32>(
                                                                            )
                                                                                as u64,
                                                                            1i32 as size_t,
                                                                            fmt_in,
                                                                        );
                                                                        if x < 1i32
                                                                            || k + x
                                                                                > 1i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 1i32
                                                                                    + 15000i32
                                                                                    + 12i32
                                                                                    + 9000i32
                                                                                    + 1i32
                                                                                    + 1i32
                                                                                    + 19i32
                                                                                    + 256i32
                                                                                    + 256i32
                                                                                    + 13i32
                                                                                    + 256i32
                                                                                    + 4i32
                                                                                    + 256i32
                                                                                    + 1i32
                                                                                    + 3i32 * 256i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 85i32
                                                                                    + 256i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 23i32
                                                                                    + 256i32
                                                                                    - 1i32
                                                                                    + 1i32
                                                                        {
                                                                            current_block =
                                                                                6442379788293543199;
                                                                            break;
                                                                        }
                                                                        do_undump(
                                                                            &mut *eqtb
                                                                                .offset(k as isize)
                                                                                as *mut memory_word
                                                                                as *mut i8,
                                                                            ::std::mem::size_of::<
                                                                                memory_word,
                                                                            >(
                                                                            )
                                                                                as u64,
                                                                            x as size_t,
                                                                            fmt_in,
                                                                        );
                                                                        k = k + x;
                                                                        do_undump(
                                                                            &mut x as *mut i32
                                                                                as *mut i8,
                                                                            ::std::mem::size_of::<i32>(
                                                                            )
                                                                                as u64,
                                                                            1i32 as size_t,
                                                                            fmt_in,
                                                                        );
                                                                        if x < 0i32
                                                                            || k + x
                                                                                > 1i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 1i32
                                                                                    + 15000i32
                                                                                    + 12i32
                                                                                    + 9000i32
                                                                                    + 1i32
                                                                                    + 1i32
                                                                                    + 19i32
                                                                                    + 256i32
                                                                                    + 256i32
                                                                                    + 13i32
                                                                                    + 256i32
                                                                                    + 4i32
                                                                                    + 256i32
                                                                                    + 1i32
                                                                                    + 3i32 * 256i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 85i32
                                                                                    + 256i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 23i32
                                                                                    + 256i32
                                                                                    - 1i32
                                                                                    + 1i32
                                                                        {
                                                                            current_block =
                                                                                6442379788293543199;
                                                                            break;
                                                                        }
                                                                        j = k;
                                                                        while j <= k + x - 1i32 {
                                                                            *eqtb.offset(
                                                                                j as isize,
                                                                            ) = *eqtb.offset(
                                                                                (k - 1i32) as isize,
                                                                            );
                                                                            j += 1
                                                                        }
                                                                        k = k + x;
                                                                        if !(k
                                                                            <= 1i32
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + 1i32
                                                                                + 15000i32
                                                                                + 12i32
                                                                                + 9000i32
                                                                                + 1i32
                                                                                + 1i32
                                                                                + 19i32
                                                                                + 256i32
                                                                                + 256i32
                                                                                + 13i32
                                                                                + 256i32
                                                                                + 4i32
                                                                                + 256i32
                                                                                + 1i32
                                                                                + 3i32 * 256i32
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + 85i32
                                                                                + 256i32
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + 23i32
                                                                                + 256i32
                                                                                - 1i32)
                                                                        {
                                                                            current_block
                                                                                =
                                                                                10041771570435381152;
                                                                            break;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        6442379788293543199 => {}
                                                                        _ => {
                                                                            if hash_high > 0i32 {
                                                                                do_undump(&mut *eqtb.offset((1i32
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 1i32
                                                                                                                 +
                                                                                                                 15000i32
                                                                                                                 +
                                                                                                                 12i32
                                                                                                                 +
                                                                                                                 9000i32
                                                                                                                 +
                                                                                                                 1i32
                                                                                                                 +
                                                                                                                 1i32
                                                                                                                 +
                                                                                                                 19i32
                                                                                                                 +
                                                                                                                 256i32
                                                                                                                 +
                                                                                                                 256i32
                                                                                                                 +
                                                                                                                 13i32
                                                                                                                 +
                                                                                                                 256i32
                                                                                                                 +
                                                                                                                 4i32
                                                                                                                 +
                                                                                                                 256i32
                                                                                                                 +
                                                                                                                 1i32
                                                                                                                 +
                                                                                                                 3i32
                                                                                                                     *
                                                                                                                     256i32
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 85i32
                                                                                                                 +
                                                                                                                 256i32
                                                                                                                 +
                                                                                                                 (0x10ffffi32
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                 +
                                                                                                                 23i32
                                                                                                                 +
                                                                                                                 256i32
                                                                                                                 -
                                                                                                                 1i32
                                                                                                                 +
                                                                                                                 1i32)
                                                                                                                as
                                                                                                                isize)
                                                                                              as
                                                                                              *mut memory_word
                                                                                              as
                                                                                              *mut i8,
                                                                                          ::std::mem::size_of::<memory_word>()
                                                                                              as
                                                                                              u64,
                                                                                          hash_high
                                                                                              as
                                                                                              size_t,
                                                                                          fmt_in);
                                                                            }
                                                                            do_undump(
                                                                                &mut x as *mut i32
                                                                                    as *mut i8,
                                                                                ::std::mem::size_of::<
                                                                                    i32,
                                                                                >(
                                                                                )
                                                                                    as u64,
                                                                                1i32 as size_t,
                                                                                fmt_in,
                                                                            );
                                                                            if !(x < 1i32
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + (0x10ffffi32
                                                                                    + 1i32)
                                                                                + 1i32
                                                                                || x > hash_top)
                                                                            {
                                                                                par_loc = x;
                                                                                par_token =
                                                                                    0x1ffffffi32
                                                                                        + par_loc;
                                                                                do_undump(&mut x
                                                                                              as
                                                                                              *mut i32
                                                                                              as
                                                                                              *mut i8,
                                                                                          ::std::mem::size_of::<i32>()
                                                                                              as
                                                                                              u64,
                                                                                          1i32
                                                                                              as
                                                                                              size_t,
                                                                                          fmt_in);
                                                                                if !(x < 1i32
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + (0x10ffffi32
                                                                                        + 1i32)
                                                                                    + 1i32
                                                                                    || x > hash_top)
                                                                                {
                                                                                    write_loc = x;
                                                                                    /* control sequence names
                                                                                     *
                                                                                     * "A different scheme is used to compress the hash table, since its lower
                                                                                     * region is usually sparse. When |text(p) != 0| for |p <= hash_used|, we
                                                                                     * output two words, |p| and |hash[p]|. The hash table is, of course,
                                                                                     * densely packed for |p >= hash_used|, so the remaining entries are
                                                                                     * output in a block."
                                                                                     */
                                                                                    p = 0i32;
                                                                                    while p
                                                                                        <= 500i32
                                                                                    {
                                                                                        do_undump(&mut *prim.as_mut_ptr().offset(p
                                                                                                                                     as
                                                                                                                                     isize)
                                                                                                      as
                                                                                                      *mut b32x2
                                                                                                      as
                                                                                                      *mut i8,
                                                                                                  ::std::mem::size_of::<b32x2>()
                                                                                                      as
                                                                                                      u64,
                                                                                                  1i32
                                                                                                      as
                                                                                                      size_t,
                                                                                                  fmt_in);
                                                                                        p += 1
                                                                                    }
                                                                                    p = 0i32;
                                                                                    while p
                                                                                        <= 500i32
                                                                                    {
                                                                                        do_undump(&mut *prim_eqtb.as_mut_ptr().offset(p
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                      as
                                                                                                      *mut memory_word
                                                                                                      as
                                                                                                      *mut i8,
                                                                                                  ::std::mem::size_of::<memory_word>()
                                                                                                      as
                                                                                                      u64,
                                                                                                  1i32
                                                                                                      as
                                                                                                      size_t,
                                                                                                  fmt_in);
                                                                                        p += 1
                                                                                    }
                                                                                    do_undump(&mut x
                                                                                                  as
                                                                                                  *mut i32
                                                                                                  as
                                                                                                  *mut i8,
                                                                                              ::std::mem::size_of::<i32>()
                                                                                                  as
                                                                                                  u64,
                                                                                              1i32
                                                                                                  as
                                                                                                  size_t,
                                                                                              fmt_in);
                                                                                    if !(x
                                                                                             <
                                                                                             1i32
                                                                                                 +
                                                                                                 (0x10ffffi32
                                                                                                      +
                                                                                                      1i32)
                                                                                                 +
                                                                                                 (0x10ffffi32
                                                                                                      +
                                                                                                      1i32)
                                                                                                 +
                                                                                                 1i32
                                                                                             ||
                                                                                             x
                                                                                                 >
                                                                                                 1i32
                                                                                                     +
                                                                                                     (0x10ffffi32
                                                                                                          +
                                                                                                          1i32)
                                                                                                     +
                                                                                                     (0x10ffffi32
                                                                                                          +
                                                                                                          1i32)
                                                                                                     +
                                                                                                     1i32
                                                                                                     +
                                                                                                     15000i32)
                                                                                       {
                                                                                        hash_used
                                                                                            =
                                                                                            x;
                                                                                        p
                                                                                            =
                                                                                            1i32
                                                                                                +
                                                                                                (0x10ffffi32
                                                                                                     +
                                                                                                     1i32)
                                                                                                +
                                                                                                (0x10ffffi32
                                                                                                     +
                                                                                                     1i32)
                                                                                                +
                                                                                                1i32
                                                                                                -
                                                                                                1i32;
                                                                                        loop 
                                                                                             {
                                                                                            do_undump(&mut x
                                                                                                          as
                                                                                                          *mut i32
                                                                                                          as
                                                                                                          *mut i8,
                                                                                                      ::std::mem::size_of::<i32>()
                                                                                                          as
                                                                                                          u64,
                                                                                                      1i32
                                                                                                          as
                                                                                                          size_t,
                                                                                                      fmt_in);
                                                                                            if x
                                                                                                   <
                                                                                                   p
                                                                                                       +
                                                                                                       1i32
                                                                                                   ||
                                                                                                   x
                                                                                                       >
                                                                                                       hash_used
                                                                                               {
                                                                                                current_block
                                                                                                    =
                                                                                                    6442379788293543199;
                                                                                                break
                                                                                                    ;
                                                                                            }
                                                                                            p
                                                                                                =
                                                                                                x;
                                                                                            do_undump(&mut *hash.offset(p
                                                                                                                            as
                                                                                                                            isize)
                                                                                                          as
                                                                                                          *mut b32x2
                                                                                                          as
                                                                                                          *mut i8,
                                                                                                      ::std::mem::size_of::<b32x2>()
                                                                                                          as
                                                                                                          u64,
                                                                                                      1i32
                                                                                                          as
                                                                                                          size_t,
                                                                                                      fmt_in);
                                                                                            if !(p
                                                                                                     !=
                                                                                                     hash_used)
                                                                                               {
                                                                                                current_block
                                                                                                    =
                                                                                                    2473505634946569239;
                                                                                                break
                                                                                                    ;
                                                                                            }
                                                                                        }
                                                                                        match current_block
                                                                                            {
                                                                                            6442379788293543199
                                                                                            =>
                                                                                            {
                                                                                            }
                                                                                            _
                                                                                            =>
                                                                                            {
                                                                                                do_undump(&mut *hash.offset((hash_used
                                                                                                                                 +
                                                                                                                                 1i32)
                                                                                                                                as
                                                                                                                                isize)
                                                                                                              as
                                                                                                              *mut b32x2
                                                                                                              as
                                                                                                              *mut i8,
                                                                                                          ::std::mem::size_of::<b32x2>()
                                                                                                              as
                                                                                                              u64,
                                                                                                          (1i32
                                                                                                               +
                                                                                                               (0x10ffffi32
                                                                                                                    +
                                                                                                                    1i32)
                                                                                                               +
                                                                                                               (0x10ffffi32
                                                                                                                    +
                                                                                                                    1i32)
                                                                                                               +
                                                                                                               1i32
                                                                                                               +
                                                                                                               15000i32
                                                                                                               +
                                                                                                               12i32
                                                                                                               +
                                                                                                               9000i32
                                                                                                               +
                                                                                                               1i32
                                                                                                               -
                                                                                                               1i32
                                                                                                               -
                                                                                                               hash_used)
                                                                                                              as
                                                                                                              size_t,
                                                                                                          fmt_in);
                                                                                                if hash_high
                                                                                                       >
                                                                                                       0i32
                                                                                                   {
                                                                                                    do_undump(&mut *hash.offset((1i32
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     1i32
                                                                                                                                     +
                                                                                                                                     15000i32
                                                                                                                                     +
                                                                                                                                     12i32
                                                                                                                                     +
                                                                                                                                     9000i32
                                                                                                                                     +
                                                                                                                                     1i32
                                                                                                                                     +
                                                                                                                                     1i32
                                                                                                                                     +
                                                                                                                                     19i32
                                                                                                                                     +
                                                                                                                                     256i32
                                                                                                                                     +
                                                                                                                                     256i32
                                                                                                                                     +
                                                                                                                                     13i32
                                                                                                                                     +
                                                                                                                                     256i32
                                                                                                                                     +
                                                                                                                                     4i32
                                                                                                                                     +
                                                                                                                                     256i32
                                                                                                                                     +
                                                                                                                                     1i32
                                                                                                                                     +
                                                                                                                                     3i32
                                                                                                                                         *
                                                                                                                                         256i32
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     85i32
                                                                                                                                     +
                                                                                                                                     256i32
                                                                                                                                     +
                                                                                                                                     (0x10ffffi32
                                                                                                                                          +
                                                                                                                                          1i32)
                                                                                                                                     +
                                                                                                                                     23i32
                                                                                                                                     +
                                                                                                                                     256i32
                                                                                                                                     -
                                                                                                                                     1i32
                                                                                                                                     +
                                                                                                                                     1i32)
                                                                                                                                    as
                                                                                                                                    isize)
                                                                                                                  as
                                                                                                                  *mut b32x2
                                                                                                                  as
                                                                                                                  *mut i8,
                                                                                                              ::std::mem::size_of::<b32x2>()
                                                                                                                  as
                                                                                                                  u64,
                                                                                                              hash_high
                                                                                                                  as
                                                                                                                  size_t,
                                                                                                              fmt_in);
                                                                                                }
                                                                                                do_undump(&mut cs_count
                                                                                                              as
                                                                                                              *mut i32
                                                                                                              as
                                                                                                              *mut i8,
                                                                                                          ::std::mem::size_of::<i32>()
                                                                                                              as
                                                                                                              u64,
                                                                                                          1i32
                                                                                                              as
                                                                                                              size_t,
                                                                                                          fmt_in);
                                                                                                /* font info */
                                                                                                do_undump(&mut x
                                                                                                              as
                                                                                                              *mut i32
                                                                                                              as
                                                                                                              *mut i8,
                                                                                                          ::std::mem::size_of::<i32>()
                                                                                                              as
                                                                                                              u64,
                                                                                                          1i32
                                                                                                              as
                                                                                                              size_t,
                                                                                                          fmt_in);
                                                                                                if !(x
                                                                                                         <
                                                                                                         7i32)
                                                                                                   {
                                                                                                    if x
                                                                                                           as
                                                                                                           i64
                                                                                                           >
                                                                                                           147483647
                                                                                                       {
                                                                                                        _tt_abort(b"must increase font_mem_size\x00"
                                                                                                                      as
                                                                                                                      *const u8
                                                                                                                      as
                                                                                                                      *const i8);
                                                                                                    }
                                                                                                    fmem_ptr
                                                                                                        =
                                                                                                        x;
                                                                                                    if fmem_ptr
                                                                                                           >
                                                                                                           font_mem_size
                                                                                                       {
                                                                                                        font_mem_size
                                                                                                            =
                                                                                                            fmem_ptr
                                                                                                    }
                                                                                                    font_info
                                                                                                        =
                                                                                                        xmalloc(((font_mem_size
                                                                                                                      +
                                                                                                                      1i32)
                                                                                                                     as
                                                                                                                     u64).wrapping_mul(::std::mem::size_of::<memory_word>()
                                                                                                                                                     as
                                                                                                                                                     u64))
                                                                                                            as
                                                                                                            *mut memory_word;
                                                                                                    do_undump(&mut *font_info.offset(0)
                                                                                                                  as
                                                                                                                  *mut memory_word
                                                                                                                  as
                                                                                                                  *mut i8,
                                                                                                              ::std::mem::size_of::<memory_word>()
                                                                                                                  as
                                                                                                                  u64,
                                                                                                              fmem_ptr
                                                                                                                  as
                                                                                                                  size_t,
                                                                                                              fmt_in);
                                                                                                    do_undump(&mut x
                                                                                                                  as
                                                                                                                  *mut i32
                                                                                                                  as
                                                                                                                  *mut i8,
                                                                                                              ::std::mem::size_of::<i32>()
                                                                                                                  as
                                                                                                                  u64,
                                                                                                              1i32
                                                                                                                  as
                                                                                                                  size_t,
                                                                                                              fmt_in);
                                                                                                    if !(x
                                                                                                             <
                                                                                                             0i32)
                                                                                                       {
                                                                                                        if x
                                                                                                               >
                                                                                                               0i32
                                                                                                                   +
                                                                                                                   9000i32
                                                                                                           {
                                                                                                            _tt_abort(b"must increase font_max\x00"
                                                                                                                          as
                                                                                                                          *const u8
                                                                                                                          as
                                                                                                                          *const i8);
                                                                                                        }
                                                                                                        font_ptr
                                                                                                            =
                                                                                                            x;
                                                                                                        font_mapping
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut *mut libc::c_void;
                                                                                                        font_layout_engine
                                                                                                            =
                                                                                                            xcalloc((font_max
                                                                                                                         +
                                                                                                                         1i32)
                                                                                                                        as
                                                                                                                        size_t,
                                                                                                                    ::std::mem::size_of::<*mut libc::c_void>()
                                                                                                                        as
                                                                                                                        u64)
                                                                                                                as
                                                                                                                *mut *mut libc::c_void;
                                                                                                        font_flags
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i8>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i8;
                                                                                                        font_letter_space
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<scaled_t>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut scaled_t;
                                                                                                        font_check
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<b16x4>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut b16x4;
                                                                                                        font_size
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<scaled_t>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut scaled_t;
                                                                                                        font_dsize
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<scaled_t>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut scaled_t;
                                                                                                        font_params
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<font_index>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut font_index;
                                                                                                        font_name
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<str_number>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut str_number;
                                                                                                        font_area
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<str_number>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut str_number;
                                                                                                        font_bc
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<UTF16_code>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut UTF16_code;
                                                                                                        font_ec
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<UTF16_code>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut UTF16_code;
                                                                                                        font_glue
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        hyphen_char
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        skew_char
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        bchar_label
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<font_index>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut font_index;
                                                                                                        font_bchar
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<nine_bits>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut nine_bits;
                                                                                                        font_false_bchar
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<nine_bits>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut nine_bits;
                                                                                                        char_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        width_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        height_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        depth_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        italic_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        lig_kern_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        kern_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        exten_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        param_base
                                                                                                            =
                                                                                                            xmalloc(((font_max
                                                                                                                          +
                                                                                                                          1i32)
                                                                                                                         as
                                                                                                                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                                                                                                                         as
                                                                                                                                                         u64))
                                                                                                                as
                                                                                                                *mut i32;
                                                                                                        k
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while k
                                                                                                                  <=
                                                                                                                  font_ptr
                                                                                                              {
                                                                                                            let ref mut fresh16 =
                                                                                                                *font_mapping.offset(k
                                                                                                                                         as
                                                                                                                                         isize);
                                                                                                            *fresh16
                                                                                                                =
                                                                                                                0
                                                                                                                    as
                                                                                                                    *mut libc::c_void;
                                                                                                            k
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        do_undump(&mut *font_check.offset(0)
                                                                                                                      as
                                                                                                                      *mut b16x4
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<b16x4>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *font_size.offset(0)
                                                                                                                      as
                                                                                                                      *mut scaled_t
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<scaled_t>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *font_dsize.offset(0)
                                                                                                                      as
                                                                                                                      *mut scaled_t
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<scaled_t>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        let mut i_0:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *font_params.offset(0)
                                                                                                                      as
                                                                                                                      *mut font_index
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<font_index>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_0
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_0
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *font_params.offset(0)
                                                                                                                     as
                                                                                                                     *mut font_index).offset(i_0
                                                                                                                                                 as
                                                                                                                                                 isize)
                                                                                                                   <
                                                                                                                   -0xfffffffi32
                                                                                                                   ||
                                                                                                                   *(&mut *font_params.offset(0)
                                                                                                                         as
                                                                                                                         *mut font_index).offset(i_0
                                                                                                                                                     as
                                                                                                                                                     isize)
                                                                                                                       >
                                                                                                                       0x3fffffffi32
                                                                                                               {
                                                                                                                _tt_abort(b"item %u (=%ld) of .fmt array at %lx <%ld or >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_0,
                                                                                                                          *(&mut *font_params.offset(0)
                                                                                                                                as
                                                                                                                                *mut font_index).offset(i_0
                                                                                                                                                            as
                                                                                                                                                            isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *font_params.offset(0)
                                                                                                                              as
                                                                                                                              *mut font_index
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          -0xfffffffi32
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          0x3fffffffi32
                                                                                                                              as
                                                                                                                              uintptr_t);
                                                                                                            }
                                                                                                            i_0
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        do_undump(&mut *hyphen_char.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *skew_char.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        let mut i_1:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *font_name.offset(0)
                                                                                                                      as
                                                                                                                      *mut str_number
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<str_number>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_1
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_1
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *font_name.offset(0)
                                                                                                                     as
                                                                                                                     *mut str_number).offset(i_1
                                                                                                                                                 as
                                                                                                                                                 isize)
                                                                                                                   >
                                                                                                                   str_ptr
                                                                                                               {
                                                                                                                _tt_abort(b"Item %u (=%ld) of .fmt array at %lx >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_1,
                                                                                                                          *(&mut *font_name.offset(0)
                                                                                                                                as
                                                                                                                                *mut str_number).offset(i_1
                                                                                                                                                            as
                                                                                                                                                            isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *font_name.offset(0)
                                                                                                                              as
                                                                                                                              *mut str_number
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          str_ptr
                                                                                                                              as
                                                                                                                              uintptr_t);
                                                                                                            }
                                                                                                            i_1
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        let mut i_2:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *font_area.offset(0)
                                                                                                                      as
                                                                                                                      *mut str_number
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<str_number>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_2
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_2
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *font_area.offset(0)
                                                                                                                     as
                                                                                                                     *mut str_number).offset(i_2
                                                                                                                                                 as
                                                                                                                                                 isize)
                                                                                                                   >
                                                                                                                   str_ptr
                                                                                                               {
                                                                                                                _tt_abort(b"Item %u (=%ld) of .fmt array at %lx >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_2,
                                                                                                                          *(&mut *font_area.offset(0)
                                                                                                                                as
                                                                                                                                *mut str_number).offset(i_2
                                                                                                                                                            as
                                                                                                                                                            isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *font_area.offset(0)
                                                                                                                              as
                                                                                                                              *mut str_number
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          str_ptr
                                                                                                                              as
                                                                                                                              uintptr_t);
                                                                                                            }
                                                                                                            i_2
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        do_undump(&mut *font_bc.offset(0)
                                                                                                                      as
                                                                                                                      *mut UTF16_code
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<UTF16_code>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *font_ec.offset(0)
                                                                                                                      as
                                                                                                                      *mut UTF16_code
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<UTF16_code>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *char_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *width_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *height_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *depth_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *italic_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *lig_kern_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *kern_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *exten_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        do_undump(&mut *param_base.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        let mut i_3:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *font_glue.offset(0)
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_3
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_3
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *font_glue.offset(0)
                                                                                                                     as
                                                                                                                     *mut i32).offset(i_3
                                                                                                                                              as
                                                                                                                                              isize)
                                                                                                                   <
                                                                                                                   -0xfffffffi32
                                                                                                                   ||
                                                                                                                   *(&mut *font_glue.offset(0)
                                                                                                                         as
                                                                                                                         *mut i32).offset(i_3
                                                                                                                                                  as
                                                                                                                                                  isize)
                                                                                                                       >
                                                                                                                       lo_mem_max
                                                                                                               {
                                                                                                                _tt_abort(b"item %u (=%ld) of .fmt array at %lx <%ld or >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_3,
                                                                                                                          *(&mut *font_glue.offset(0)
                                                                                                                                as
                                                                                                                                *mut i32).offset(i_3
                                                                                                                                                         as
                                                                                                                                                         isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *font_glue.offset(0)
                                                                                                                              as
                                                                                                                              *mut i32
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          -0xfffffffi32
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          lo_mem_max
                                                                                                                              as
                                                                                                                              uintptr_t);
                                                                                                            }
                                                                                                            i_3
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        let mut i_4:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *bchar_label.offset(0)
                                                                                                                      as
                                                                                                                      *mut font_index
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<font_index>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_4
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_4
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *bchar_label.offset(0)
                                                                                                                     as
                                                                                                                     *mut font_index).offset(i_4
                                                                                                                                                 as
                                                                                                                                                 isize)
                                                                                                                   <
                                                                                                                   0i32
                                                                                                                   ||
                                                                                                                   *(&mut *bchar_label.offset(0)
                                                                                                                         as
                                                                                                                         *mut font_index).offset(i_4
                                                                                                                                                     as
                                                                                                                                                     isize)
                                                                                                                       >
                                                                                                                       fmem_ptr
                                                                                                                           -
                                                                                                                           1i32
                                                                                                               {
                                                                                                                _tt_abort(b"item %u (=%ld) of .fmt array at %lx <%ld or >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_4,
                                                                                                                          *(&mut *bchar_label.offset(0)
                                                                                                                                as
                                                                                                                                *mut font_index).offset(i_4
                                                                                                                                                            as
                                                                                                                                                            isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *bchar_label.offset(0)
                                                                                                                              as
                                                                                                                              *mut font_index
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          0i32
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          (fmem_ptr
                                                                                                                               as
                                                                                                                               uintptr_t).wrapping_sub(1i32
                                                                                                                                                           as
                                                                                                                                                           u64));
                                                                                                            }
                                                                                                            i_4
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        let mut i_5:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *font_bchar.offset(0)
                                                                                                                      as
                                                                                                                      *mut nine_bits
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<nine_bits>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_5
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_5
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *font_bchar.offset(0)
                                                                                                                     as
                                                                                                                     *mut nine_bits).offset(i_5
                                                                                                                                                as
                                                                                                                                                isize)
                                                                                                                   <
                                                                                                                   0i32
                                                                                                                   ||
                                                                                                                   *(&mut *font_bchar.offset(0)
                                                                                                                         as
                                                                                                                         *mut nine_bits).offset(i_5
                                                                                                                                                    as
                                                                                                                                                    isize)
                                                                                                                       >
                                                                                                                       65536i32
                                                                                                               {
                                                                                                                _tt_abort(b"item %u (=%ld) of .fmt array at %lx <%ld or >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_5,
                                                                                                                          *(&mut *font_bchar.offset(0)
                                                                                                                                as
                                                                                                                                *mut nine_bits).offset(i_5
                                                                                                                                                           as
                                                                                                                                                           isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *font_bchar.offset(0)
                                                                                                                              as
                                                                                                                              *mut nine_bits
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          0i32
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          65536i32
                                                                                                                              as
                                                                                                                              uintptr_t);
                                                                                                            }
                                                                                                            i_5
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        let mut i_6:
                                                                                                                i32 =
                                                                                                            0;
                                                                                                        do_undump(&mut *font_false_bchar.offset(0)
                                                                                                                      as
                                                                                                                      *mut nine_bits
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<nine_bits>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  (font_ptr
                                                                                                                       +
                                                                                                                       1i32)
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        i_6
                                                                                                            =
                                                                                                            0i32;
                                                                                                        while i_6
                                                                                                                  <
                                                                                                                  font_ptr
                                                                                                                      +
                                                                                                                      1i32
                                                                                                              {
                                                                                                            if *(&mut *font_false_bchar.offset(0)
                                                                                                                     as
                                                                                                                     *mut nine_bits).offset(i_6
                                                                                                                                                as
                                                                                                                                                isize)
                                                                                                                   <
                                                                                                                   0i32
                                                                                                                   ||
                                                                                                                   *(&mut *font_false_bchar.offset(0)
                                                                                                                         as
                                                                                                                         *mut nine_bits).offset(i_6
                                                                                                                                                    as
                                                                                                                                                    isize)
                                                                                                                       >
                                                                                                                       65536i32
                                                                                                               {
                                                                                                                _tt_abort(b"item %u (=%ld) of .fmt array at %lx <%ld or >%ld\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8,
                                                                                                                          i_6,
                                                                                                                          *(&mut *font_false_bchar.offset(0)
                                                                                                                                as
                                                                                                                                *mut nine_bits).offset(i_6
                                                                                                                                                           as
                                                                                                                                                           isize)
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          &mut *font_false_bchar.offset(0)
                                                                                                                              as
                                                                                                                              *mut nine_bits
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          0i32
                                                                                                                              as
                                                                                                                              uintptr_t,
                                                                                                                          65536i32
                                                                                                                              as
                                                                                                                              uintptr_t);
                                                                                                            }
                                                                                                            i_6
                                                                                                                +=
                                                                                                                1
                                                                                                        }
                                                                                                        /* hyphenations */
                                                                                                        do_undump(&mut x
                                                                                                                      as
                                                                                                                      *mut i32
                                                                                                                      as
                                                                                                                      *mut i8,
                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                      as
                                                                                                                      u64,
                                                                                                                  1i32
                                                                                                                      as
                                                                                                                      size_t,
                                                                                                                  fmt_in);
                                                                                                        if !(x
                                                                                                                 <
                                                                                                                 0i32)
                                                                                                           {
                                                                                                            if x
                                                                                                                   >
                                                                                                                   hyph_size
                                                                                                               {
                                                                                                                _tt_abort(b"must increase hyph_size\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const i8);
                                                                                                            }
                                                                                                            hyph_count
                                                                                                                =
                                                                                                                x;
                                                                                                            do_undump(&mut x
                                                                                                                          as
                                                                                                                          *mut i32
                                                                                                                          as
                                                                                                                          *mut i8,
                                                                                                                      ::std::mem::size_of::<i32>()
                                                                                                                          as
                                                                                                                          u64,
                                                                                                                      1i32
                                                                                                                          as
                                                                                                                          size_t,
                                                                                                                      fmt_in);
                                                                                                            if !(x
                                                                                                                     <
                                                                                                                     607i32)
                                                                                                               {
                                                                                                                if x
                                                                                                                       >
                                                                                                                       hyph_size
                                                                                                                   {
                                                                                                                    _tt_abort(b"must increase hyph_size\x00"
                                                                                                                                  as
                                                                                                                                  *const u8
                                                                                                                                  as
                                                                                                                                  *const i8);
                                                                                                                }
                                                                                                                hyph_next
                                                                                                                    =
                                                                                                                    x;
                                                                                                                j
                                                                                                                    =
                                                                                                                    0i32;
                                                                                                                k
                                                                                                                    =
                                                                                                                    1i32;
                                                                                                                loop 
                                                                                                                     {
                                                                                                                    if !(k
                                                                                                                             <=
                                                                                                                             hyph_count)
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            5183402691674069415;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                    do_undump(&mut j
                                                                                                                                  as
                                                                                                                                  *mut i32
                                                                                                                                  as
                                                                                                                                  *mut i8,
                                                                                                                              ::std::mem::size_of::<i32>()
                                                                                                                                  as
                                                                                                                                  u64,
                                                                                                                              1i32
                                                                                                                                  as
                                                                                                                                  size_t,
                                                                                                                              fmt_in);
                                                                                                                    if j
                                                                                                                           <
                                                                                                                           0i32
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            6442379788293543199;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                    if j
                                                                                                                           as
                                                                                                                           i64
                                                                                                                           >
                                                                                                                           65535
                                                                                                                       {
                                                                                                                        hyph_next
                                                                                                                            =
                                                                                                                            (j
                                                                                                                                 as
                                                                                                                                 i64
                                                                                                                                 /
                                                                                                                                 65536)
                                                                                                                                as
                                                                                                                                i32;
                                                                                                                        j
                                                                                                                            =
                                                                                                                            (j
                                                                                                                                 as
                                                                                                                                 i64
                                                                                                                                 -
                                                                                                                                 hyph_next
                                                                                                                                     as
                                                                                                                                     i64
                                                                                                                                     *
                                                                                                                                     65536)
                                                                                                                                as
                                                                                                                                i32
                                                                                                                    } else {
                                                                                                                        hyph_next
                                                                                                                            =
                                                                                                                            0i32
                                                                                                                    }
                                                                                                                    if j
                                                                                                                           >=
                                                                                                                           hyph_size
                                                                                                                           ||
                                                                                                                           hyph_next
                                                                                                                               >
                                                                                                                               hyph_size
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            6442379788293543199;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                    *hyph_link.offset(j
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                                        =
                                                                                                                        hyph_next
                                                                                                                            as
                                                                                                                            hyph_pointer;
                                                                                                                    do_undump(&mut x
                                                                                                                                  as
                                                                                                                                  *mut i32
                                                                                                                                  as
                                                                                                                                  *mut i8,
                                                                                                                              ::std::mem::size_of::<i32>()
                                                                                                                                  as
                                                                                                                                  u64,
                                                                                                                              1i32
                                                                                                                                  as
                                                                                                                                  size_t,
                                                                                                                              fmt_in);
                                                                                                                    if x
                                                                                                                           <
                                                                                                                           0i32
                                                                                                                           ||
                                                                                                                           x
                                                                                                                               >
                                                                                                                               str_ptr
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            6442379788293543199;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                    *hyph_word.offset(j
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                                        =
                                                                                                                        x;
                                                                                                                    do_undump(&mut x
                                                                                                                                  as
                                                                                                                                  *mut i32
                                                                                                                                  as
                                                                                                                                  *mut i8,
                                                                                                                              ::std::mem::size_of::<i32>()
                                                                                                                                  as
                                                                                                                                  u64,
                                                                                                                              1i32
                                                                                                                                  as
                                                                                                                                  size_t,
                                                                                                                              fmt_in);
                                                                                                                    if x
                                                                                                                           <
                                                                                                                           -0xfffffffi32
                                                                                                                           ||
                                                                                                                           x
                                                                                                                               >
                                                                                                                               0x3fffffffi32
                                                                                                                       {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            6442379788293543199;
                                                                                                                        break
                                                                                                                            ;
                                                                                                                    }
                                                                                                                    *hyph_list.offset(j
                                                                                                                                          as
                                                                                                                                          isize)
                                                                                                                        =
                                                                                                                        x;
                                                                                                                    k
                                                                                                                        +=
                                                                                                                        1
                                                                                                                }
                                                                                                                match current_block
                                                                                                                    {
                                                                                                                    6442379788293543199
                                                                                                                    =>
                                                                                                                    {
                                                                                                                    }
                                                                                                                    _
                                                                                                                    =>
                                                                                                                    {
                                                                                                                        j
                                                                                                                            +=
                                                                                                                            1;
                                                                                                                        if j
                                                                                                                               <
                                                                                                                               607i32
                                                                                                                           {
                                                                                                                            j
                                                                                                                                =
                                                                                                                                607i32
                                                                                                                        }
                                                                                                                        hyph_next
                                                                                                                            =
                                                                                                                            j;
                                                                                                                        if hyph_next
                                                                                                                               >=
                                                                                                                               hyph_size
                                                                                                                           {
                                                                                                                            hyph_next
                                                                                                                                =
                                                                                                                                607i32
                                                                                                                        } else if hyph_next
                                                                                                                                      >=
                                                                                                                                      607i32
                                                                                                                         {
                                                                                                                            hyph_next
                                                                                                                                +=
                                                                                                                                1
                                                                                                                        }
                                                                                                                        do_undump(&mut x
                                                                                                                                      as
                                                                                                                                      *mut i32
                                                                                                                                      as
                                                                                                                                      *mut i8,
                                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                                      as
                                                                                                                                      u64,
                                                                                                                                  1i32
                                                                                                                                      as
                                                                                                                                      size_t,
                                                                                                                                  fmt_in);
                                                                                                                        if !(x
                                                                                                                                 <
                                                                                                                                 0i32)
                                                                                                                           {
                                                                                                                            if x
                                                                                                                                   >
                                                                                                                                   trie_size
                                                                                                                               {
                                                                                                                                _tt_abort(b"must increase trie_size\x00"
                                                                                                                                              as
                                                                                                                                              *const u8
                                                                                                                                              as
                                                                                                                                              *const i8);
                                                                                                                            }
                                                                                                                            j
                                                                                                                                =
                                                                                                                                x;
                                                                                                                            trie_max
                                                                                                                                =
                                                                                                                                j;
                                                                                                                            do_undump(&mut x
                                                                                                                                          as
                                                                                                                                          *mut i32
                                                                                                                                          as
                                                                                                                                          *mut i8,
                                                                                                                                      ::std::mem::size_of::<i32>()
                                                                                                                                          as
                                                                                                                                          u64,
                                                                                                                                      1i32
                                                                                                                                          as
                                                                                                                                          size_t,
                                                                                                                                      fmt_in);
                                                                                                                            if !(x
                                                                                                                                     <
                                                                                                                                     0i32
                                                                                                                                     ||
                                                                                                                                     x
                                                                                                                                         >
                                                                                                                                         j)
                                                                                                                               {
                                                                                                                                hyph_start
                                                                                                                                    =
                                                                                                                                    x;
                                                                                                                                if trie_trl.is_null()
                                                                                                                                   {
                                                                                                                                    trie_trl
                                                                                                                                        =
                                                                                                                                        xmalloc(((j
                                                                                                                                                      +
                                                                                                                                                      1i32
                                                                                                                                                      +
                                                                                                                                                      1i32)
                                                                                                                                                     as
                                                                                                                                                     u64).wrapping_mul(::std::mem::size_of::<trie_pointer>()
                                                                                                                                                                                     as
                                                                                                                                                                                     u64))
                                                                                                                                            as
                                                                                                                                            *mut trie_pointer
                                                                                                                                }
                                                                                                                                do_undump(&mut *trie_trl.offset(0)
                                                                                                                                              as
                                                                                                                                              *mut trie_pointer
                                                                                                                                              as
                                                                                                                                              *mut i8,
                                                                                                                                          ::std::mem::size_of::<trie_pointer>()
                                                                                                                                              as
                                                                                                                                              u64,
                                                                                                                                          (j
                                                                                                                                               +
                                                                                                                                               1i32)
                                                                                                                                              as
                                                                                                                                              size_t,
                                                                                                                                          fmt_in);
                                                                                                                                if trie_tro.is_null()
                                                                                                                                   {
                                                                                                                                    trie_tro
                                                                                                                                        =
                                                                                                                                        xmalloc(((j
                                                                                                                                                      +
                                                                                                                                                      1i32
                                                                                                                                                      +
                                                                                                                                                      1i32)
                                                                                                                                                     as
                                                                                                                                                     u64).wrapping_mul(::std::mem::size_of::<trie_pointer>()
                                                                                                                                                                                     as
                                                                                                                                                                                     u64))
                                                                                                                                            as
                                                                                                                                            *mut trie_pointer
                                                                                                                                }
                                                                                                                                do_undump(&mut *trie_tro.offset(0)
                                                                                                                                              as
                                                                                                                                              *mut trie_pointer
                                                                                                                                              as
                                                                                                                                              *mut i8,
                                                                                                                                          ::std::mem::size_of::<trie_pointer>()
                                                                                                                                              as
                                                                                                                                              u64,
                                                                                                                                          (j
                                                                                                                                               +
                                                                                                                                               1i32)
                                                                                                                                              as
                                                                                                                                              size_t,
                                                                                                                                          fmt_in);
                                                                                                                                if trie_trc.is_null()
                                                                                                                                   {
                                                                                                                                    trie_trc
                                                                                                                                        =
                                                                                                                                        xmalloc(((j
                                                                                                                                                      +
                                                                                                                                                      1i32
                                                                                                                                                      +
                                                                                                                                                      1i32)
                                                                                                                                                     as
                                                                                                                                                     u64).wrapping_mul(::std::mem::size_of::<u16>()
                                                                                                                                                                                     as
                                                                                                                                                                                     u64))
                                                                                                                                            as
                                                                                                                                            *mut u16
                                                                                                                                }
                                                                                                                                do_undump(&mut *trie_trc.offset(0)
                                                                                                                                              as
                                                                                                                                              *mut u16
                                                                                                                                              as
                                                                                                                                              *mut i8,
                                                                                                                                          ::std::mem::size_of::<u16>()
                                                                                                                                              as
                                                                                                                                              u64,
                                                                                                                                          (j
                                                                                                                                               +
                                                                                                                                               1i32)
                                                                                                                                              as
                                                                                                                                              size_t,
                                                                                                                                          fmt_in);
                                                                                                                                do_undump(&mut max_hyph_char
                                                                                                                                              as
                                                                                                                                              *mut i32
                                                                                                                                              as
                                                                                                                                              *mut i8,
                                                                                                                                          ::std::mem::size_of::<i32>()
                                                                                                                                              as
                                                                                                                                              u64,
                                                                                                                                          1i32
                                                                                                                                              as
                                                                                                                                              size_t,
                                                                                                                                          fmt_in);
                                                                                                                                do_undump(&mut x
                                                                                                                                              as
                                                                                                                                              *mut i32
                                                                                                                                              as
                                                                                                                                              *mut i8,
                                                                                                                                          ::std::mem::size_of::<i32>()
                                                                                                                                              as
                                                                                                                                              u64,
                                                                                                                                          1i32
                                                                                                                                              as
                                                                                                                                              size_t,
                                                                                                                                          fmt_in);
                                                                                                                                if !(x
                                                                                                                                         <
                                                                                                                                         0i32)
                                                                                                                                   {
                                                                                                                                    if x
                                                                                                                                           as
                                                                                                                                           i64
                                                                                                                                           >
                                                                                                                                           35111
                                                                                                                                       {
                                                                                                                                        _tt_abort(b"must increase TRIE_OP_SIZE\x00"
                                                                                                                                                      as
                                                                                                                                                      *const u8
                                                                                                                                                      as
                                                                                                                                                      *const i8);
                                                                                                                                    }
                                                                                                                                    j
                                                                                                                                        =
                                                                                                                                        x;
                                                                                                                                    trie_op_ptr
                                                                                                                                        =
                                                                                                                                        j;
                                                                                                                                    do_undump(&mut *hyf_distance.as_mut_ptr().offset(1)
                                                                                                                                                  as
                                                                                                                                                  *mut small_number
                                                                                                                                                  as
                                                                                                                                                  *mut i8,
                                                                                                                                              ::std::mem::size_of::<small_number>()
                                                                                                                                                  as
                                                                                                                                                  u64,
                                                                                                                                              j
                                                                                                                                                  as
                                                                                                                                                  size_t,
                                                                                                                                              fmt_in);
                                                                                                                                    do_undump(&mut *hyf_num.as_mut_ptr().offset(1)
                                                                                                                                                  as
                                                                                                                                                  *mut small_number
                                                                                                                                                  as
                                                                                                                                                  *mut i8,
                                                                                                                                              ::std::mem::size_of::<small_number>()
                                                                                                                                                  as
                                                                                                                                                  u64,
                                                                                                                                              j
                                                                                                                                                  as
                                                                                                                                                  size_t,
                                                                                                                                              fmt_in);
                                                                                                                                    let mut i_7:
                                                                                                                                            i32 =
                                                                                                                                        0;
                                                                                                                                    do_undump(&mut *hyf_next.as_mut_ptr().offset(1)
                                                                                                                                                  as
                                                                                                                                                  *mut trie_opcode
                                                                                                                                                  as
                                                                                                                                                  *mut i8,
                                                                                                                                              ::std::mem::size_of::<trie_opcode>()
                                                                                                                                                  as
                                                                                                                                                  u64,
                                                                                                                                              j
                                                                                                                                                  as
                                                                                                                                                  size_t,
                                                                                                                                              fmt_in);
                                                                                                                                    i_7
                                                                                                                                        =
                                                                                                                                        0i32;
                                                                                                                                    while i_7
                                                                                                                                              <
                                                                                                                                              j
                                                                                                                                          {
                                                                                                                                        if *(&mut *hyf_next.as_mut_ptr().offset(1)
                                                                                                                                                 as
                                                                                                                                                 *mut trie_opcode).offset(i_7
                                                                                                                                                                              as
                                                                                                                                                                              isize)
                                                                                                                                               as
                                                                                                                                               i64
                                                                                                                                               >
                                                                                                                                               65535
                                                                                                                                           {
                                                                                                                                            _tt_abort(b"Item %u (=%ld) of .fmt array at %lx >%ld\x00"
                                                                                                                                                          as
                                                                                                                                                          *const u8
                                                                                                                                                          as
                                                                                                                                                          *const i8,
                                                                                                                                                      i_7,
                                                                                                                                                      *(&mut *hyf_next.as_mut_ptr().offset(1)
                                                                                                                                                            as
                                                                                                                                                            *mut trie_opcode).offset(i_7
                                                                                                                                                                                         as
                                                                                                                                                                                         isize)
                                                                                                                                                          as
                                                                                                                                                          uintptr_t,
                                                                                                                                                      &mut *hyf_next.as_mut_ptr().offset(1)
                                                                                                                                                          as
                                                                                                                                                          *mut trie_opcode
                                                                                                                                                          as
                                                                                                                                                          uintptr_t,
                                                                                                                                                      65535
                                                                                                                                                          as
                                                                                                                                                          uintptr_t);
                                                                                                                                        }
                                                                                                                                        i_7
                                                                                                                                            +=
                                                                                                                                            1
                                                                                                                                    }
                                                                                                                                    k
                                                                                                                                        =
                                                                                                                                        0i32;
                                                                                                                                    while k
                                                                                                                                              <=
                                                                                                                                              255i32
                                                                                                                                          {
                                                                                                                                        trie_used[k
                                                                                                                                                      as
                                                                                                                                                      usize]
                                                                                                                                            =
                                                                                                                                            0i32
                                                                                                                                                as
                                                                                                                                                trie_opcode;
                                                                                                                                        k
                                                                                                                                            +=
                                                                                                                                            1
                                                                                                                                    }
                                                                                                                                    k
                                                                                                                                        =
                                                                                                                                        255i32
                                                                                                                                            +
                                                                                                                                            1i32;
                                                                                                                                    loop 
                                                                                                                                         {
                                                                                                                                        if !(j
                                                                                                                                                 >
                                                                                                                                                 0i32)
                                                                                                                                           {
                                                                                                                                            current_block
                                                                                                                                                =
                                                                                                                                                2455569213248551296;
                                                                                                                                            break
                                                                                                                                                ;
                                                                                                                                        }
                                                                                                                                        do_undump(&mut x
                                                                                                                                                      as
                                                                                                                                                      *mut i32
                                                                                                                                                      as
                                                                                                                                                      *mut i8,
                                                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                                                      as
                                                                                                                                                      u64,
                                                                                                                                                  1i32
                                                                                                                                                      as
                                                                                                                                                      size_t,
                                                                                                                                                  fmt_in);
                                                                                                                                        if x
                                                                                                                                               <
                                                                                                                                               0i32
                                                                                                                                               ||
                                                                                                                                               x
                                                                                                                                                   >
                                                                                                                                                   k
                                                                                                                                                       -
                                                                                                                                                       1i32
                                                                                                                                           {
                                                                                                                                            current_block
                                                                                                                                                =
                                                                                                                                                6442379788293543199;
                                                                                                                                            break
                                                                                                                                                ;
                                                                                                                                        }
                                                                                                                                        k
                                                                                                                                            =
                                                                                                                                            x;
                                                                                                                                        do_undump(&mut x
                                                                                                                                                      as
                                                                                                                                                      *mut i32
                                                                                                                                                      as
                                                                                                                                                      *mut i8,
                                                                                                                                                  ::std::mem::size_of::<i32>()
                                                                                                                                                      as
                                                                                                                                                      u64,
                                                                                                                                                  1i32
                                                                                                                                                      as
                                                                                                                                                      size_t,
                                                                                                                                                  fmt_in);
                                                                                                                                        if x
                                                                                                                                               <
                                                                                                                                               1i32
                                                                                                                                               ||
                                                                                                                                               x
                                                                                                                                                   >
                                                                                                                                                   j
                                                                                                                                           {
                                                                                                                                            current_block
                                                                                                                                                =
                                                                                                                                                6442379788293543199;
                                                                                                                                            break
                                                                                                                                                ;
                                                                                                                                        }
                                                                                                                                        trie_used[k
                                                                                                                                                      as
                                                                                                                                                      usize]
                                                                                                                                            =
                                                                                                                                            x
                                                                                                                                                as
                                                                                                                                                trie_opcode;
                                                                                                                                        j
                                                                                                                                            =
                                                                                                                                            j
                                                                                                                                                -
                                                                                                                                                x;
                                                                                                                                        op_start[k
                                                                                                                                                     as
                                                                                                                                                     usize]
                                                                                                                                            =
                                                                                                                                            j
                                                                                                                                    }
                                                                                                                                    match current_block
                                                                                                                                        {
                                                                                                                                        6442379788293543199
                                                                                                                                        =>
                                                                                                                                        {
                                                                                                                                        }
                                                                                                                                        _
                                                                                                                                        =>
                                                                                                                                        {
                                                                                                                                            trie_not_ready
                                                                                                                                                =
                                                                                                                                                0i32
                                                                                                                                                    !=
                                                                                                                                                    0;
                                                                                                                                            /* trailer */
                                                                                                                                            do_undump(&mut x
                                                                                                                                                          as
                                                                                                                                                          *mut i32
                                                                                                                                                          as
                                                                                                                                                          *mut i8,
                                                                                                                                                      ::std::mem::size_of::<i32>()
                                                                                                                                                          as
                                                                                                                                                          u64,
                                                                                                                                                      1i32
                                                                                                                                                          as
                                                                                                                                                          size_t,
                                                                                                                                                      fmt_in);
                                                                                                                                            if !(x
                                                                                                                                                     !=
                                                                                                                                                     0x29ai32)
                                                                                                                                               {
                                                                                                                                                ttstub_input_close(fmt_in);
                                                                                                                                                return 1i32
                                                                                                                                                           !=
                                                                                                                                                           0
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _tt_abort(b"fatal format file error\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn final_cleanup() {
    let mut c: small_number = 0;
    c = cur_chr as small_number;
    if job_name == 0i32 {
        open_log_file();
    }
    while input_ptr > 0i32 {
        if cur_input.state as i32 == 0i32 {
            end_token_list();
        } else {
            end_file_reading();
        }
    }
    while open_parens > 0i32 {
        print_cstr(b" )\x00" as *const u8 as *const i8);
        open_parens -= 1
    }
    if cur_level as i32 > 1i32 {
        print_nl('(' as i32);
        print_esc_cstr(b"end occurred \x00" as *const u8 as *const i8);
        print_cstr(b"inside a group at level \x00" as *const u8 as *const i8);
        print_int(cur_level as i32 - 1i32);
        print_char(')' as i32);
        show_save_groups();
    }
    while cond_ptr != -0xfffffffi32 {
        print_nl('(' as i32);
        print_esc_cstr(b"end occurred \x00" as *const u8 as *const i8);
        print_cstr(b"when \x00" as *const u8 as *const i8);
        print_cmd_chr(107_u16, cur_if as i32);
        if if_line != 0i32 {
            print_cstr(b" on line \x00" as *const u8 as *const i8);
            print_int(if_line);
        }
        print_cstr(b" was incomplete)\x00" as *const u8 as *const i8);
        if_line = (*mem.offset((cond_ptr + 1i32) as isize)).b32.s1;
        cur_if = (*mem.offset(cond_ptr as isize)).b16.s0 as small_number;
        temp_ptr = cond_ptr;
        cond_ptr = (*mem.offset(cond_ptr as isize)).b32.s1;
        free_node(temp_ptr, 2i32);
    }
    if history != TTHistory::SPOTLESS {
        if history == TTHistory::WARNING_ISSUED || (interaction as i32) < 3i32 {
            if selector == Selector::TERM_AND_LOG {
                selector = Selector::TERM_ONLY;
                print_nl_cstr(
                    b"(see the transcript file for additional information)\x00" as *const u8
                        as *const i8,
                );
                selector = Selector::TERM_AND_LOG
            }
        }
    }
    if c as i32 == 1i32 {
        if in_initex_mode {
            let mut for_end: i32 = 0;
            c = 0i32 as small_number;
            for_end = 4i32;
            if c as i32 <= for_end {
                loop {
                    if cur_mark[c as usize] != -0xfffffffi32 {
                        delete_token_ref(cur_mark[c as usize]);
                    }
                    let fresh17 = c;
                    c = c + 1;
                    if !((fresh17 as i32) < for_end) {
                        break;
                    }
                }
            }
            if sa_root[7] != -0xfffffffi32 {
                if do_marks(3i32 as small_number, 0i32 as small_number, sa_root[7]) {
                    sa_root[7] = -0xfffffffi32
                }
            }
            let mut for_end_0: i32 = 0;
            c = 2i32 as small_number;
            for_end_0 = 3i32;
            if c as i32 <= for_end_0 {
                loop {
                    flush_node_list(disc_ptr[c as usize]);
                    let fresh18 = c;
                    c = c + 1;
                    if !((fresh18 as i32) < for_end_0) {
                        break;
                    }
                }
            }
            if last_glue != 0x3fffffffi32 {
                delete_glue_ref(last_glue);
            }
            store_fmt_file();
            return;
        }
        print_nl_cstr(b"(\\dump is performed only by INITEX)\x00" as *const u8 as *const i8);
        return;
    };
}
/* Engine initialization */
static mut stdin_ufile: UFILE = UFILE {
    handle: 0 as *const libc::c_void as *mut libc::c_void,
    savedChar: 0,
    skipNextLF: 0,
    encodingMode: 0,
    conversionData: 0 as *const libc::c_void as *mut libc::c_void,
};
unsafe extern "C" fn init_io() {
    /* This is largely vestigial at this point */
    stdin_ufile.handle = 0 as *mut libc::c_void;
    stdin_ufile.savedChar = -1i32 as i64;
    stdin_ufile.skipNextLF = 0_i16;
    stdin_ufile.encodingMode = 1_i16;
    stdin_ufile.conversionData = 0 as *mut libc::c_void;
    let ref mut fresh19 = *input_file.offset(0);
    *fresh19 = &mut stdin_ufile;
    *buffer.offset(first as isize) = 0i32;
    last = first;
    cur_input.loc = first;
    cur_input.limit = last;
    first = last + 1i32;
}
unsafe extern "C" fn initialize_more_variables() {
    let mut k: i32 = 0;
    let mut z: hyph_pointer = 0;
    doing_special = false;
    native_text_size = 128i32;
    native_text =
        xmalloc((native_text_size as u64).wrapping_mul(::std::mem::size_of::<UTF16_code>() as u64))
            as *mut UTF16_code;
    interaction = 3_u8;
    deletions_allowed = true;
    set_box_allowed = true;
    error_count = 0_i8;
    help_ptr = 0_u8;
    use_err_help = false;
    nest_ptr = 0i32;
    max_nest_stack = 0i32;
    cur_list.mode = 1_i16;
    cur_list.head = 4999999i32 - 1i32;
    cur_list.tail = 4999999i32 - 1i32;
    cur_list.eTeX_aux = -0xfffffffi32;
    cur_list.aux.b32.s1 = -65536000i32;
    cur_list.mode_line = 0i32;
    cur_list.prev_graf = 0i32;
    shown_mode = 0_i16;
    page_contents = 0_u8;
    page_tail = 4999999i32 - 2i32;
    last_glue = 0x3fffffffi32;
    last_penalty = 0i32;
    last_kern = 0i32;
    page_so_far[7] = 0i32;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 256i32
        + 1i32
        + 3i32 * 256i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32);
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 23i32
            + 256i32
            - 1i32
    {
        _xeq_level_array[(k
            - (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32))) as usize] = 1_u16;
        k += 1
    }
    no_new_control_sequence = true;
    prim[0].s0 = 0i32;
    prim[0].s1 = 0i32;
    k = 1i32;
    while k <= 500i32 {
        prim[k as usize] = prim[0];
        k += 1
    }
    prim_eqtb[0].b16.s0 = 0_u16;
    prim_eqtb[0].b16.s1 = 103_u16;
    prim_eqtb[0].b32.s1 = -0xfffffffi32;
    k = 1i32;
    while k <= 500i32 {
        prim_eqtb[k as usize] = prim_eqtb[0];
        k += 1
    }
    save_ptr = 0i32;
    cur_level = 1_u16;
    cur_group = 0i32 as group_code;
    cur_boundary = 0i32;
    max_save_stack = 0i32;
    mag_set = 0i32;
    expand_depth_count = 0i32;
    is_in_csname = false;
    cur_mark[0] = -0xfffffffi32;
    cur_mark[1] = -0xfffffffi32;
    cur_mark[2] = -0xfffffffi32;
    cur_mark[3] = -0xfffffffi32;
    cur_mark[4] = -0xfffffffi32;
    cur_val = 0i32;
    cur_val_level = 0_u8;
    radix = 0i32 as small_number;
    cur_order = 0i32 as glue_ord;
    k = 0i32;
    while k <= 16i32 {
        read_open[k as usize] = 2_u8;
        k += 1
    }
    cond_ptr = -0xfffffffi32;
    if_limit = 0_u8;
    cur_if = 0i32 as small_number;
    if_line = 0i32;
    null_character.s3 = 0_u16;
    null_character.s2 = 0_u16;
    null_character.s1 = 0_u16;
    null_character.s0 = 0_u16;
    total_pages = 0i32;
    max_v = 0i32;
    max_h = 0i32;
    max_push = 0i32;
    last_bop = -1i32;
    doing_leaders = false;
    dead_cycles = 0i32;
    adjust_tail = -0xfffffffi32;
    last_badness = 0i32;
    pre_adjust_tail = -0xfffffffi32;
    pack_begin_line = 0i32;
    empty.s1 = 0i32;
    empty.s0 = -0xfffffffi32;
    align_ptr = -0xfffffffi32;
    cur_align = -0xfffffffi32;
    cur_span = -0xfffffffi32;
    cur_loop = -0xfffffffi32;
    cur_head = -0xfffffffi32;
    cur_tail = -0xfffffffi32;
    cur_pre_head = -0xfffffffi32;
    cur_pre_tail = -0xfffffffi32;
    cur_f = 0i32;
    max_hyph_char = 256i32;
    z = 0i32 as hyph_pointer;
    while z as i32 <= hyph_size {
        *hyph_word.offset(z as isize) = 0i32;
        *hyph_list.offset(z as isize) = -0xfffffffi32;
        *hyph_link.offset(z as isize) = 0i32 as hyph_pointer;
        z = z.wrapping_add(1)
    }
    hyph_count = 0i32;
    hyph_next = 607i32 + 1i32;
    if hyph_next > hyph_size {
        hyph_next = 607i32
    }
    output_active = false;
    insert_penalties = 0i32;
    ligature_present = false;
    cancel_boundary = false;
    lft_hit = false;
    rt_hit = false;
    ins_disc = false;
    after_token = 0i32;
    long_help_seen = false;
    format_ident = 0i32;
    k = 0i32;
    while k <= 17i32 {
        write_open[k as usize] = false;
        k += 1
    }
    LR_ptr = -0xfffffffi32;
    LR_problems = 0i32;
    cur_dir = 0i32 as small_number;
    pseudo_files = -0xfffffffi32;
    sa_root[7] = -0xfffffffi32;
    sa_null.b32.s0 = -0xfffffffi32;
    sa_null.b32.s1 = -0xfffffffi32;
    sa_chain = -0xfffffffi32;
    sa_level = 0_u16;
    disc_ptr[2] = -0xfffffffi32;
    disc_ptr[3] = -0xfffffffi32;
    edit_name_start = 0i32;
    stop_at_space = true;
}
unsafe extern "C" fn initialize_more_initex_variables() {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    k = 1i32;
    while k <= 19i32 {
        (*mem.offset(k as isize)).b32.s1 = 0i32;
        k += 1
    }
    k = 0i32;
    while k <= 19i32 {
        (*mem.offset(k as isize)).b32.s1 = -0xfffffffi32 + 1i32;
        (*mem.offset(k as isize)).b16.s1 = 0_u16;
        (*mem.offset(k as isize)).b16.s0 = 0_u16;
        k += 4i32
    }
    (*mem.offset(6)).b32.s1 = 65536 as i32;
    (*mem.offset(4)).b16.s1 = 1_u16;
    (*mem.offset(10)).b32.s1 = 65536 as i32;
    (*mem.offset(8)).b16.s1 = 2_u16;
    (*mem.offset(14)).b32.s1 = 65536 as i32;
    (*mem.offset(12)).b16.s1 = 1_u16;
    (*mem.offset(15)).b32.s1 = 65536 as i32;
    (*mem.offset(12)).b16.s0 = 1_u16;
    (*mem.offset(18)).b32.s1 = -65536 as i32;
    (*mem.offset(16)).b16.s1 = 1_u16;
    rover = 20i32;
    (*mem.offset(rover as isize)).b32.s1 = 0x3fffffffi32;
    (*mem.offset(rover as isize)).b32.s0 = 1000i32;
    (*mem.offset((rover + 1i32) as isize)).b32.s0 = rover;
    (*mem.offset((rover + 1i32) as isize)).b32.s1 = rover;
    lo_mem_max = rover + 1000i32;
    (*mem.offset(lo_mem_max as isize)).b32.s1 = -0xfffffffi32;
    (*mem.offset(lo_mem_max as isize)).b32.s0 = -0xfffffffi32;
    k = 4999999i32 - 14i32;
    while k <= 4999999i32 {
        *mem.offset(k as isize) = *mem.offset(lo_mem_max as isize);
        k += 1
    }
    (*mem.offset((4999999i32 - 10i32) as isize)).b32.s0 = 0x1ffffffi32
        + (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 5i32);
    (*mem.offset((4999999i32 - 9i32) as isize)).b32.s1 = 65535i32 + 1i32;
    (*mem.offset((4999999i32 - 9i32) as isize)).b32.s0 = -0xfffffffi32;
    (*mem.offset((4999999i32 - 7i32) as isize)).b16.s1 = 1_u16;
    (*mem.offset((4999999i32 - 7i32 + 1i32) as isize)).b32.s0 = 0x3fffffffi32;
    (*mem.offset((4999999i32 - 7i32) as isize)).b16.s0 = 0_u16;
    (*mem.offset(4999999)).b16.s0 = 255_u16;
    (*mem.offset(4999999)).b16.s1 = 1_u16;
    (*mem.offset(4999999)).b32.s1 = 4999999i32;
    (*mem.offset((4999999i32 - 2i32) as isize)).b16.s1 = 10_u16;
    (*mem.offset((4999999i32 - 2i32) as isize)).b16.s0 = 0_u16;
    avail = -0xfffffffi32;
    mem_end = 4999999i32;
    hi_mem_min = 4999999i32 - 14i32;
    var_used = 20i32;
    dyn_used = 15i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32) as isize,
    ))
    .b16
    .s1 = 103_u16;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32) as isize,
    ))
    .b32
    .s1 = -0xfffffffi32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32) as isize,
    ))
    .b16
    .s0 = 0_u16;
    k = 1i32;
    while k <= eqtb_top {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32) as isize,
        );
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32) as isize,
    ))
    .b32
    .s1 = 0i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32) as isize,
    ))
    .b16
    .s1 = 119_u16;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 1i32;
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            - 1i32
    {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32) as isize,
        );
        k += 1
    }
    let ref mut fresh20 = (*mem.offset(0)).b32.s1;
    *fresh20 += 531i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 0i32) as isize,
    ))
    .b32
    .s1 = -0xfffffffi32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 0i32) as isize,
    ))
    .b16
    .s1 = 120_u16;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 0i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32;
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            - 1i32
    {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 0i32) as isize,
        );
        k += 1
    }
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 1i32;
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            - 1i32
    {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32) as isize,
        );
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32) as isize,
    ))
    .b32
    .s1 = -0xfffffffi32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32) as isize,
    ))
    .b16
    .s1 = 121_u16;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 1i32;
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            - 1i32
    {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32) as isize,
        );
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32) as isize,
    ))
    .b32
    .s1 = 0i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32) as isize,
    ))
    .b16
    .s1 = 122_u16;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 256i32
        + 1i32;
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            - 1i32
    {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32) as isize,
        );
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32) as isize,
    ))
    .b32
    .s1 = 0i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32) as isize,
    ))
    .b16
    .s1 = 122_u16;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 256i32
        + 1i32
        + 3i32 * 256i32
        + 1i32;
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            - 1i32
    {
        *eqtb.offset(k as isize) = *eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32) as isize,
        );
        k += 1
    }
    k = 0i32;
    while k <= 0x10ffffi32 + 1i32 - 1i32 {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + k) as isize,
        ))
        .b32
        .s1 = 12i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = k;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = 1000i32;
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + 13i32) as isize,
    ))
    .b32
    .s1 = 5i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + 32i32) as isize,
    ))
    .b32
    .s1 = 10i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + 92i32) as isize,
    ))
    .b32
    .s1 = 0i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + 37i32) as isize,
    ))
    .b32
    .s1 = 14i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + 127i32) as isize,
    ))
    .b32
    .s1 = 15i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32) as isize,
    ))
    .b32
    .s1 = 9i32;
    k = '0' as i32;
    while k <= '9' as i32 {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = (k as u32).wrapping_add((7_u32 & 0x7_u32) << 21i32) as i32;
        k += 1
    }
    k = 'A' as i32;
    while k <= 'Z' as i32 {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + k) as isize,
        ))
        .b32
        .s1 = 11i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (k + 32i32)) as isize,
        ))
        .b32
        .s1 = 11i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = (k as u32)
            .wrapping_add((1_u32 & 0xff_u32) << 24i32)
            .wrapping_add((7_u32 & 0x7_u32) << 21i32) as i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (k + 32i32)) as isize,
        ))
        .b32
        .s1 = ((k + 32i32) as u32)
            .wrapping_add((1_u32 & 0xff_u32) << 24i32)
            .wrapping_add((7_u32 & 0x7_u32) << 21i32) as i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = k + 32i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (k + 32i32)) as isize,
        ))
        .b32
        .s1 = k + 32i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = k;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (k + 32i32)) as isize,
        ))
        .b32
        .s1 = k;
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + k) as isize,
        ))
        .b32
        .s1 = 999i32;
        k += 1
    }
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 256i32
        + 1i32
        + 3i32 * 256i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32);
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            - 1i32
    {
        (*eqtb.offset(k as isize)).b32.s1 = 0i32;
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 55i32) as isize,
    ))
    .b32
    .s1 = 256i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 56i32) as isize,
    ))
    .b32
    .s1 = -1i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 17i32) as isize,
    ))
    .b32
    .s1 = 1000i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32) as isize,
    ))
    .b32
    .s1 = 10000i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 41i32) as isize,
    ))
    .b32
    .s1 = 1i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 40i32) as isize,
    ))
    .b32
    .s1 = 25i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 45i32) as isize,
    ))
    .b32
    .s1 = '\\' as i32;
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 48i32) as isize,
    ))
    .b32
    .s1 = 13i32;
    k = 0i32;
    while k <= 0x10ffffi32 + 1i32 - 1i32 {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 85i32
                + 256i32
                + k) as isize,
        ))
        .b32
        .s1 = -1i32;
        k += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + 46i32) as isize,
    ))
    .b32
    .s1 = 0i32;
    k = 1i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 1i32
        + 15000i32
        + 12i32
        + 9000i32
        + 1i32
        + 1i32
        + 19i32
        + 256i32
        + 256i32
        + 13i32
        + 256i32
        + 4i32
        + 256i32
        + 1i32
        + 3i32 * 256i32
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + (0x10ffffi32 + 1i32)
        + 85i32
        + 256i32
        + (0x10ffffi32 + 1i32);
    while k
        <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 23i32
            + 256i32
            - 1i32
    {
        (*eqtb.offset(k as isize)).b32.s1 = 0i32;
        k += 1
    }
    prim_used = 500i32;
    hash_used = 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32;
    hash_high = 0i32;
    cs_count = 0i32;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 9i32) as isize,
    ))
    .b16
    .s1 = 118_u16;
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 9i32) as isize,
    ))
    .s1 = maketexstring(b"notexpanded:\x00" as *const u8 as *const i8);
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 11i32) as isize,
    ))
    .b16
    .s1 = 39_u16;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 11i32) as isize,
    ))
    .b32
    .s1 = 1i32;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 11i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 11i32) as isize,
    ))
    .s1 = maketexstring(b"primitive\x00" as *const u8 as *const i8);
    k = -(35111 as i32);
    while k as i64 <= 35111 {
        _trie_op_hash_array[(k as i64 - -35111) as usize] = 0i32;
        k += 1
    }
    k = 0i32;
    while k <= 255i32 {
        trie_used[k as usize] = 0i32 as trie_opcode;
        k += 1
    }
    max_op_used = 0i32 as trie_opcode;
    trie_op_ptr = 0i32;
    trie_not_ready = true;
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 0i32) as isize,
    ))
    .s1 = maketexstring(b"inaccessible\x00" as *const u8 as *const i8);
    format_ident = maketexstring(b" (INITEX)\x00" as *const u8 as *const i8);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 8i32) as isize,
    ))
    .s1 = maketexstring(b"endwrite\x00" as *const u8 as *const i8);
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 8i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 8i32) as isize,
    ))
    .b16
    .s1 = 115_u16;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 8i32) as isize,
    ))
    .b32
    .s1 = -0xfffffffi32;
    max_reg_num = 32767i32;
    max_reg_help_line =
        b"A register number must be between 0 and 32767.\x00" as *const u8 as *const i8;
    i = 0i32;
    while i <= 6i32 {
        sa_root[i as usize] = -0xfffffffi32;
        i += 1
    }
    (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 82i32) as isize,
    ))
    .b32
    .s1 = 63i32;
}
/*:1370*/
/*1371: */
unsafe extern "C" fn initialize_primitives() {
    no_new_control_sequence = false;
    first = 0i32;
    primitive(
        b"lineskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 0i32,
    );
    primitive(
        b"baselineskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 1i32,
    );
    primitive(
        b"parskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 2i32,
    );
    primitive(
        b"abovedisplayskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 3i32,
    );
    primitive(
        b"belowdisplayskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 4i32,
    );
    primitive(
        b"abovedisplayshortskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 5i32,
    );
    primitive(
        b"belowdisplayshortskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 6i32,
    );
    primitive(
        b"leftskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 7i32,
    );
    primitive(
        b"rightskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 8i32,
    );
    primitive(
        b"topskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 9i32,
    );
    primitive(
        b"splittopskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 10i32,
    );
    primitive(
        b"tabskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 11i32,
    );
    primitive(
        b"spaceskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 12i32,
    );
    primitive(
        b"xspaceskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 13i32,
    );
    primitive(
        b"parfillskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 14i32,
    );
    primitive(
        b"XeTeXlinebreakskip\x00" as *const u8 as *const i8,
        76_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 15i32,
    );
    primitive(
        b"thinmuskip\x00" as *const u8 as *const i8,
        77_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 16i32,
    );
    primitive(
        b"medmuskip\x00" as *const u8 as *const i8,
        77_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 17i32,
    );
    primitive(
        b"thickmuskip\x00" as *const u8 as *const i8,
        77_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 18i32,
    );
    primitive(
        b"output\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 1i32,
    );
    primitive(
        b"everypar\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 2i32,
    );
    primitive(
        b"everymath\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 3i32,
    );
    primitive(
        b"everydisplay\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 4i32,
    );
    primitive(
        b"everyhbox\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 5i32,
    );
    primitive(
        b"everyvbox\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 6i32,
    );
    primitive(
        b"everyjob\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 7i32,
    );
    primitive(
        b"everycr\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 8i32,
    );
    primitive(
        b"errhelp\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 9i32,
    );
    primitive(
        b"everyeof\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 10i32,
    );
    primitive(
        b"XeTeXinterchartoks\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 11i32,
    );
    primitive(
        b"TectonicCodaTokens\x00" as *const u8 as *const i8,
        73_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 12i32,
    );
    primitive(
        b"pretolerance\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 0i32,
    );
    primitive(
        b"tolerance\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32,
    );
    primitive(
        b"linepenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 2i32,
    );
    primitive(
        b"hyphenpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 3i32,
    );
    primitive(
        b"exhyphenpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 4i32,
    );
    primitive(
        b"clubpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 5i32,
    );
    primitive(
        b"widowpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 6i32,
    );
    primitive(
        b"displaywidowpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 7i32,
    );
    primitive(
        b"brokenpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 8i32,
    );
    primitive(
        b"binoppenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 9i32,
    );
    primitive(
        b"relpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 10i32,
    );
    primitive(
        b"predisplaypenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 11i32,
    );
    primitive(
        b"postdisplaypenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 12i32,
    );
    primitive(
        b"interlinepenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 13i32,
    );
    primitive(
        b"doublehyphendemerits\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 14i32,
    );
    primitive(
        b"finalhyphendemerits\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 15i32,
    );
    primitive(
        b"adjdemerits\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 16i32,
    );
    primitive(
        b"mag\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 17i32,
    );
    primitive(
        b"delimiterfactor\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 18i32,
    );
    primitive(
        b"looseness\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 19i32,
    );
    primitive(
        b"time\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 20i32,
    );
    primitive(
        b"day\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 21i32,
    );
    primitive(
        b"month\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 22i32,
    );
    primitive(
        b"year\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 23i32,
    );
    primitive(
        b"showboxbreadth\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 24i32,
    );
    primitive(
        b"showboxdepth\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 25i32,
    );
    primitive(
        b"hbadness\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 26i32,
    );
    primitive(
        b"vbadness\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 27i32,
    );
    primitive(
        b"pausing\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 28i32,
    );
    primitive(
        b"tracingonline\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 29i32,
    );
    primitive(
        b"tracingmacros\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 30i32,
    );
    primitive(
        b"tracingstats\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 31i32,
    );
    primitive(
        b"tracingparagraphs\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 32i32,
    );
    primitive(
        b"tracingpages\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 33i32,
    );
    primitive(
        b"tracingoutput\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 34i32,
    );
    primitive(
        b"tracinglostchars\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 35i32,
    );
    primitive(
        b"tracingcommands\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 36i32,
    );
    primitive(
        b"tracingrestores\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 37i32,
    );
    primitive(
        b"uchyph\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 38i32,
    );
    primitive(
        b"outputpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 39i32,
    );
    primitive(
        b"maxdeadcycles\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 40i32,
    );
    primitive(
        b"hangafter\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 41i32,
    );
    primitive(
        b"floatingpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 42i32,
    );
    primitive(
        b"globaldefs\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 43i32,
    );
    primitive(
        b"fam\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 44i32,
    );
    primitive(
        b"escapechar\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 45i32,
    );
    primitive(
        b"defaulthyphenchar\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 46i32,
    );
    primitive(
        b"defaultskewchar\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 47i32,
    );
    primitive(
        b"endlinechar\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 48i32,
    );
    primitive(
        b"newlinechar\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 49i32,
    );
    primitive(
        b"language\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 50i32,
    );
    primitive(
        b"lefthyphenmin\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 51i32,
    );
    primitive(
        b"righthyphenmin\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 52i32,
    );
    primitive(
        b"holdinginserts\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 53i32,
    );
    primitive(
        b"errorcontextlines\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 54i32,
    );
    primitive(
        b"XeTeXlinebreakpenalty\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 69i32,
    );
    primitive(
        b"XeTeXprotrudechars\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 70i32,
    );
    primitive(
        b"parindent\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 0i32,
    );
    primitive(
        b"mathsurround\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 1i32,
    );
    primitive(
        b"lineskiplimit\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 2i32,
    );
    primitive(
        b"hsize\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 3i32,
    );
    primitive(
        b"vsize\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 4i32,
    );
    primitive(
        b"maxdepth\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 5i32,
    );
    primitive(
        b"splitmaxdepth\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 6i32,
    );
    primitive(
        b"boxmaxdepth\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 7i32,
    );
    primitive(
        b"hfuzz\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 8i32,
    );
    primitive(
        b"vfuzz\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 9i32,
    );
    primitive(
        b"delimitershortfall\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 10i32,
    );
    primitive(
        b"nulldelimiterspace\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 11i32,
    );
    primitive(
        b"scriptspace\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 12i32,
    );
    primitive(
        b"predisplaysize\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 13i32,
    );
    primitive(
        b"displaywidth\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 14i32,
    );
    primitive(
        b"displayindent\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 15i32,
    );
    primitive(
        b"overfullrule\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 16i32,
    );
    primitive(
        b"hangindent\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 17i32,
    );
    primitive(
        b"hoffset\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 18i32,
    );
    primitive(
        b"voffset\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 19i32,
    );
    primitive(
        b"emergencystretch\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 20i32,
    );
    primitive(
        b"pdfpagewidth\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 21i32,
    );
    primitive(
        b"pdfpageheight\x00" as *const u8 as *const i8,
        75_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 22i32,
    );
    primitive(b" \x00" as *const u8 as *const i8, 64_u16, 0i32);
    primitive(b"/\x00" as *const u8 as *const i8, 44_u16, 0i32);
    primitive(b"accent\x00" as *const u8 as *const i8, 45_u16, 0i32);
    primitive(b"advance\x00" as *const u8 as *const i8, 92_u16, 0i32);
    primitive(
        b"afterassignment\x00" as *const u8 as *const i8,
        40_u16,
        0i32,
    );
    primitive(b"aftergroup\x00" as *const u8 as *const i8, 41_u16, 0i32);
    primitive(b"begingroup\x00" as *const u8 as *const i8, 61_u16, 0i32);
    primitive(b"char\x00" as *const u8 as *const i8, 16_u16, 0i32);
    primitive(b"csname\x00" as *const u8 as *const i8, 109_u16, 0i32);
    primitive(b"delimiter\x00" as *const u8 as *const i8, 15_u16, 0i32);
    primitive(
        b"XeTeXdelimiter\x00" as *const u8 as *const i8,
        15_u16,
        1i32,
    );
    primitive(b"Udelimiter\x00" as *const u8 as *const i8, 15_u16, 1i32);
    primitive(b"divide\x00" as *const u8 as *const i8, 94_u16, 0i32);
    primitive(b"endcsname\x00" as *const u8 as *const i8, 67_u16, 0i32);
    primitive(b"endgroup\x00" as *const u8 as *const i8, 62_u16, 0i32);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 2i32) as isize,
    ))
    .s1 = maketexstring(b"endgroup\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 2i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(b"expandafter\x00" as *const u8 as *const i8, 104_u16, 0i32);
    primitive(b"font\x00" as *const u8 as *const i8, 90_u16, 0i32);
    primitive(b"fontdimen\x00" as *const u8 as *const i8, 78_u16, 0i32);
    primitive(b"halign\x00" as *const u8 as *const i8, 32_u16, 0i32);
    primitive(b"hrule\x00" as *const u8 as *const i8, 36_u16, 0i32);
    primitive(b"ignorespaces\x00" as *const u8 as *const i8, 39_u16, 0i32);
    primitive(b"insert\x00" as *const u8 as *const i8, 37_u16, 0i32);
    primitive(b"mark\x00" as *const u8 as *const i8, 18_u16, 0i32);
    primitive(b"mathaccent\x00" as *const u8 as *const i8, 46_u16, 0i32);
    primitive(
        b"XeTeXmathaccent\x00" as *const u8 as *const i8,
        46_u16,
        1i32,
    );
    primitive(b"Umathaccent\x00" as *const u8 as *const i8, 46_u16, 1i32);
    primitive(b"mathchar\x00" as *const u8 as *const i8, 17_u16, 0i32);
    primitive(
        b"XeTeXmathcharnum\x00" as *const u8 as *const i8,
        17_u16,
        1i32,
    );
    primitive(b"Umathcharnum\x00" as *const u8 as *const i8, 17_u16, 1i32);
    primitive(b"XeTeXmathchar\x00" as *const u8 as *const i8, 17_u16, 2i32);
    primitive(b"Umathchar\x00" as *const u8 as *const i8, 17_u16, 2i32);
    primitive(b"mathchoice\x00" as *const u8 as *const i8, 54_u16, 0i32);
    primitive(b"multiply\x00" as *const u8 as *const i8, 93_u16, 0i32);
    primitive(b"noalign\x00" as *const u8 as *const i8, 34_u16, 0i32);
    primitive(b"noboundary\x00" as *const u8 as *const i8, 65_u16, 0i32);
    primitive(b"noexpand\x00" as *const u8 as *const i8, 105_u16, 0i32);
    primitive(b"primitive\x00" as *const u8 as *const i8, 105_u16, 1i32);
    primitive(b"nonscript\x00" as *const u8 as *const i8, 55_u16, 0i32);
    primitive(b"omit\x00" as *const u8 as *const i8, 63_u16, 0i32);
    primitive(
        b"parshape\x00" as *const u8 as *const i8,
        85_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 0i32,
    );
    primitive(b"penalty\x00" as *const u8 as *const i8, 42_u16, 0i32);
    primitive(b"prevgraf\x00" as *const u8 as *const i8, 81_u16, 0i32);
    primitive(b"radical\x00" as *const u8 as *const i8, 66_u16, 0i32);
    primitive(b"XeTeXradical\x00" as *const u8 as *const i8, 66_u16, 1i32);
    primitive(b"Uradical\x00" as *const u8 as *const i8, 66_u16, 1i32);
    primitive(b"read\x00" as *const u8 as *const i8, 98_u16, 0i32);
    primitive(
        b"relax\x00" as *const u8 as *const i8,
        0_u16,
        0x10ffffi32 + 1i32,
    );
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 7i32) as isize,
    ))
    .s1 = maketexstring(b"relax\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 7i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(b"setbox\x00" as *const u8 as *const i8, 100_u16, 0i32);
    primitive(b"the\x00" as *const u8 as *const i8, 111_u16, 0i32);
    primitive(b"toks\x00" as *const u8 as *const i8, 72_u16, 0i32);
    primitive(b"vadjust\x00" as *const u8 as *const i8, 38_u16, 0i32);
    primitive(b"valign\x00" as *const u8 as *const i8, 33_u16, 0i32);
    primitive(b"vcenter\x00" as *const u8 as *const i8, 56_u16, 0i32);
    primitive(b"vrule\x00" as *const u8 as *const i8, 35_u16, 0i32);
    primitive(
        b"par\x00" as *const u8 as *const i8,
        13_u16,
        0x10ffffi32 + 1i32,
    );
    par_loc = cur_val;
    par_token = 0x1ffffffi32 + par_loc;
    primitive(b"input\x00" as *const u8 as *const i8, 106_u16, 0i32);
    primitive(b"endinput\x00" as *const u8 as *const i8, 106_u16, 1i32);
    primitive(b"topmark\x00" as *const u8 as *const i8, 112_u16, 0i32);
    primitive(b"firstmark\x00" as *const u8 as *const i8, 112_u16, 1i32);
    primitive(b"botmark\x00" as *const u8 as *const i8, 112_u16, 2i32);
    primitive(
        b"splitfirstmark\x00" as *const u8 as *const i8,
        112_u16,
        3i32,
    );
    primitive(b"splitbotmark\x00" as *const u8 as *const i8, 112_u16, 4i32);
    primitive(b"count\x00" as *const u8 as *const i8, 91_u16, 0i32);
    primitive(b"dimen\x00" as *const u8 as *const i8, 91_u16, 1i32);
    primitive(b"skip\x00" as *const u8 as *const i8, 91_u16, 2i32);
    primitive(b"muskip\x00" as *const u8 as *const i8, 91_u16, 3i32);
    primitive(b"spacefactor\x00" as *const u8 as *const i8, 80_u16, 104i32);
    primitive(b"prevdepth\x00" as *const u8 as *const i8, 80_u16, 1i32);
    primitive(b"deadcycles\x00" as *const u8 as *const i8, 83_u16, 0i32);
    primitive(
        b"insertpenalties\x00" as *const u8 as *const i8,
        83_u16,
        1i32,
    );
    primitive(b"wd\x00" as *const u8 as *const i8, 84_u16, 1i32);
    primitive(b"ht\x00" as *const u8 as *const i8, 84_u16, 3i32);
    primitive(b"dp\x00" as *const u8 as *const i8, 84_u16, 2i32);
    primitive(b"lastpenalty\x00" as *const u8 as *const i8, 71_u16, 0i32);
    primitive(b"lastkern\x00" as *const u8 as *const i8, 71_u16, 1i32);
    primitive(b"lastskip\x00" as *const u8 as *const i8, 71_u16, 2i32);
    primitive(b"inputlineno\x00" as *const u8 as *const i8, 71_u16, 4i32);
    primitive(b"badness\x00" as *const u8 as *const i8, 71_u16, 5i32);
    primitive(b"number\x00" as *const u8 as *const i8, 110_u16, 0i32);
    primitive(b"romannumeral\x00" as *const u8 as *const i8, 110_u16, 1i32);
    primitive(b"string\x00" as *const u8 as *const i8, 110_u16, 2i32);
    primitive(b"meaning\x00" as *const u8 as *const i8, 110_u16, 3i32);
    primitive(b"fontname\x00" as *const u8 as *const i8, 110_u16, 4i32);
    primitive(b"jobname\x00" as *const u8 as *const i8, 110_u16, 15i32);
    primitive(
        b"leftmarginkern\x00" as *const u8 as *const i8,
        110_u16,
        11i32,
    );
    primitive(
        b"rightmarginkern\x00" as *const u8 as *const i8,
        110_u16,
        12i32,
    );
    primitive(b"Uchar\x00" as *const u8 as *const i8, 110_u16, 13i32);
    primitive(b"Ucharcat\x00" as *const u8 as *const i8, 110_u16, 14i32);
    primitive(b"if\x00" as *const u8 as *const i8, 107_u16, 0i32);
    primitive(b"ifcat\x00" as *const u8 as *const i8, 107_u16, 1i32);
    primitive(b"ifnum\x00" as *const u8 as *const i8, 107_u16, 2i32);
    primitive(b"ifdim\x00" as *const u8 as *const i8, 107_u16, 3i32);
    primitive(b"ifodd\x00" as *const u8 as *const i8, 107_u16, 4i32);
    primitive(b"ifvmode\x00" as *const u8 as *const i8, 107_u16, 5i32);
    primitive(b"ifhmode\x00" as *const u8 as *const i8, 107_u16, 6i32);
    primitive(b"ifmmode\x00" as *const u8 as *const i8, 107_u16, 7i32);
    primitive(b"ifinner\x00" as *const u8 as *const i8, 107_u16, 8i32);
    primitive(b"ifvoid\x00" as *const u8 as *const i8, 107_u16, 9i32);
    primitive(b"ifhbox\x00" as *const u8 as *const i8, 107_u16, 10i32);
    primitive(b"ifvbox\x00" as *const u8 as *const i8, 107_u16, 11i32);
    primitive(b"ifx\x00" as *const u8 as *const i8, 107_u16, 12i32);
    primitive(b"ifeof\x00" as *const u8 as *const i8, 107_u16, 13i32);
    primitive(b"iftrue\x00" as *const u8 as *const i8, 107_u16, 14i32);
    primitive(b"iffalse\x00" as *const u8 as *const i8, 107_u16, 15i32);
    primitive(b"ifcase\x00" as *const u8 as *const i8, 107_u16, 16i32);
    primitive(b"ifprimitive\x00" as *const u8 as *const i8, 107_u16, 21i32);
    primitive(b"fi\x00" as *const u8 as *const i8, 108_u16, 2i32);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 4i32) as isize,
    ))
    .s1 = maketexstring(b"fi\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 4i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(b"or\x00" as *const u8 as *const i8, 108_u16, 4i32);
    primitive(b"else\x00" as *const u8 as *const i8, 108_u16, 3i32);
    primitive(b"nullfont\x00" as *const u8 as *const i8, 89_u16, 0i32);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 12i32) as isize,
    ))
    .s1 = maketexstring(b"nullfont\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 12i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(
        b"span\x00" as *const u8 as *const i8,
        4_u16,
        0x10ffffi32 + 2i32,
    );
    primitive(
        b"cr\x00" as *const u8 as *const i8,
        5_u16,
        0x10ffffi32 + 3i32,
    );
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 1i32) as isize,
    ))
    .s1 = maketexstring(b"cr\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 1i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(
        b"crcr\x00" as *const u8 as *const i8,
        5_u16,
        0x10ffffi32 + 4i32,
    );
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 5i32) as isize,
    ))
    .s1 = maketexstring(b"endtemplate\x00" as *const u8 as *const i8);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 6i32) as isize,
    ))
    .s1 = maketexstring(b"endtemplate\x00" as *const u8 as *const i8);
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 6i32) as isize,
    ))
    .b16
    .s1 = 9_u16;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 6i32) as isize,
    ))
    .b32
    .s1 = 4999999i32 - 11i32;
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 6i32) as isize,
    ))
    .b16
    .s0 = 1_u16;
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 5i32) as isize,
    ) = *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 6i32) as isize,
    );
    (*eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 5i32) as isize,
    ))
    .b16
    .s1 = 117_u16;
    primitive(b"pagegoal\x00" as *const u8 as *const i8, 82_u16, 0i32);
    primitive(b"pagetotal\x00" as *const u8 as *const i8, 82_u16, 1i32);
    primitive(b"pagestretch\x00" as *const u8 as *const i8, 82_u16, 2i32);
    primitive(
        b"pagefilstretch\x00" as *const u8 as *const i8,
        82_u16,
        3i32,
    );
    primitive(
        b"pagefillstretch\x00" as *const u8 as *const i8,
        82_u16,
        4i32,
    );
    primitive(
        b"pagefilllstretch\x00" as *const u8 as *const i8,
        82_u16,
        5i32,
    );
    primitive(b"pageshrink\x00" as *const u8 as *const i8, 82_u16, 6i32);
    primitive(b"pagedepth\x00" as *const u8 as *const i8, 82_u16, 7i32);
    primitive(b"end\x00" as *const u8 as *const i8, 14_u16, 0i32);
    primitive(b"dump\x00" as *const u8 as *const i8, 14_u16, 1i32);
    primitive(b"hskip\x00" as *const u8 as *const i8, 26_u16, 4i32);
    primitive(b"hfil\x00" as *const u8 as *const i8, 26_u16, 0i32);
    primitive(b"hfill\x00" as *const u8 as *const i8, 26_u16, 1i32);
    primitive(b"hss\x00" as *const u8 as *const i8, 26_u16, 2i32);
    primitive(b"hfilneg\x00" as *const u8 as *const i8, 26_u16, 3i32);
    primitive(b"vskip\x00" as *const u8 as *const i8, 27_u16, 4i32);
    primitive(b"vfil\x00" as *const u8 as *const i8, 27_u16, 0i32);
    primitive(b"vfill\x00" as *const u8 as *const i8, 27_u16, 1i32);
    primitive(b"vss\x00" as *const u8 as *const i8, 27_u16, 2i32);
    primitive(b"vfilneg\x00" as *const u8 as *const i8, 27_u16, 3i32);
    primitive(b"mskip\x00" as *const u8 as *const i8, 28_u16, 5i32);
    primitive(b"kern\x00" as *const u8 as *const i8, 29_u16, 1i32);
    primitive(b"mkern\x00" as *const u8 as *const i8, 30_u16, 99i32);
    primitive(b"moveleft\x00" as *const u8 as *const i8, 21_u16, 1i32);
    primitive(b"moveright\x00" as *const u8 as *const i8, 21_u16, 0i32);
    primitive(b"raise\x00" as *const u8 as *const i8, 22_u16, 1i32);
    primitive(b"lower\x00" as *const u8 as *const i8, 22_u16, 0i32);
    primitive(b"box\x00" as *const u8 as *const i8, 20_u16, 0i32);
    primitive(b"copy\x00" as *const u8 as *const i8, 20_u16, 1i32);
    primitive(b"lastbox\x00" as *const u8 as *const i8, 20_u16, 2i32);
    primitive(b"vsplit\x00" as *const u8 as *const i8, 20_u16, 3i32);
    primitive(b"vtop\x00" as *const u8 as *const i8, 20_u16, 4i32);
    primitive(b"vbox\x00" as *const u8 as *const i8, 20_u16, 4i32 + 1i32);
    primitive(b"hbox\x00" as *const u8 as *const i8, 20_u16, 4i32 + 104i32);
    primitive(
        b"shipout\x00" as *const u8 as *const i8,
        31_u16,
        100i32 - 1i32,
    );
    primitive(b"leaders\x00" as *const u8 as *const i8, 31_u16, 100i32);
    primitive(b"cleaders\x00" as *const u8 as *const i8, 31_u16, 101i32);
    primitive(b"xleaders\x00" as *const u8 as *const i8, 31_u16, 102i32);
    primitive(b"indent\x00" as *const u8 as *const i8, 43_u16, 1i32);
    primitive(b"noindent\x00" as *const u8 as *const i8, 43_u16, 0i32);
    primitive(b"unpenalty\x00" as *const u8 as *const i8, 25_u16, 12i32);
    primitive(b"unkern\x00" as *const u8 as *const i8, 25_u16, 11i32);
    primitive(b"unskip\x00" as *const u8 as *const i8, 25_u16, 10i32);
    primitive(b"unhbox\x00" as *const u8 as *const i8, 23_u16, 0i32);
    primitive(b"unhcopy\x00" as *const u8 as *const i8, 23_u16, 1i32);
    primitive(b"unvbox\x00" as *const u8 as *const i8, 24_u16, 0i32);
    primitive(b"unvcopy\x00" as *const u8 as *const i8, 24_u16, 1i32);
    primitive(b"-\x00" as *const u8 as *const i8, 47_u16, 1i32);
    primitive(b"discretionary\x00" as *const u8 as *const i8, 47_u16, 0i32);
    primitive(b"eqno\x00" as *const u8 as *const i8, 48_u16, 0i32);
    primitive(b"leqno\x00" as *const u8 as *const i8, 48_u16, 1i32);
    primitive(b"mathord\x00" as *const u8 as *const i8, 50_u16, 16i32);
    primitive(b"mathop\x00" as *const u8 as *const i8, 50_u16, 17i32);
    primitive(b"mathbin\x00" as *const u8 as *const i8, 50_u16, 18i32);
    primitive(b"mathrel\x00" as *const u8 as *const i8, 50_u16, 19i32);
    primitive(b"mathopen\x00" as *const u8 as *const i8, 50_u16, 20i32);
    primitive(b"mathclose\x00" as *const u8 as *const i8, 50_u16, 21i32);
    primitive(b"mathpunct\x00" as *const u8 as *const i8, 50_u16, 22i32);
    primitive(b"mathinner\x00" as *const u8 as *const i8, 50_u16, 23i32);
    primitive(b"underline\x00" as *const u8 as *const i8, 50_u16, 26i32);
    primitive(b"overline\x00" as *const u8 as *const i8, 50_u16, 27i32);
    primitive(b"displaylimits\x00" as *const u8 as *const i8, 51_u16, 0i32);
    primitive(b"limits\x00" as *const u8 as *const i8, 51_u16, 1i32);
    primitive(b"nolimits\x00" as *const u8 as *const i8, 51_u16, 2i32);
    primitive(b"displaystyle\x00" as *const u8 as *const i8, 53_u16, 0i32);
    primitive(b"textstyle\x00" as *const u8 as *const i8, 53_u16, 2i32);
    primitive(b"scriptstyle\x00" as *const u8 as *const i8, 53_u16, 4i32);
    primitive(
        b"scriptscriptstyle\x00" as *const u8 as *const i8,
        53_u16,
        6i32,
    );
    primitive(b"above\x00" as *const u8 as *const i8, 52_u16, 0i32);
    primitive(b"over\x00" as *const u8 as *const i8, 52_u16, 1i32);
    primitive(b"atop\x00" as *const u8 as *const i8, 52_u16, 2i32);
    primitive(
        b"abovewithdelims\x00" as *const u8 as *const i8,
        52_u16,
        3i32 + 0i32,
    );
    primitive(
        b"overwithdelims\x00" as *const u8 as *const i8,
        52_u16,
        3i32 + 1i32,
    );
    primitive(
        b"atopwithdelims\x00" as *const u8 as *const i8,
        52_u16,
        3i32 + 2i32,
    );
    primitive(b"left\x00" as *const u8 as *const i8, 49_u16, 30i32);
    primitive(b"right\x00" as *const u8 as *const i8, 49_u16, 31i32);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 3i32) as isize,
    ))
    .s1 = maketexstring(b"right\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 3i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(b"long\x00" as *const u8 as *const i8, 95_u16, 1i32);
    primitive(b"outer\x00" as *const u8 as *const i8, 95_u16, 2i32);
    primitive(b"global\x00" as *const u8 as *const i8, 95_u16, 4i32);
    primitive(b"def\x00" as *const u8 as *const i8, 99_u16, 0i32);
    primitive(b"gdef\x00" as *const u8 as *const i8, 99_u16, 1i32);
    primitive(b"edef\x00" as *const u8 as *const i8, 99_u16, 2i32);
    primitive(b"xdef\x00" as *const u8 as *const i8, 99_u16, 3i32);
    primitive(b"let\x00" as *const u8 as *const i8, 96_u16, 0i32);
    primitive(
        b"futurelet\x00" as *const u8 as *const i8,
        96_u16,
        0i32 + 1i32,
    );
    primitive(b"chardef\x00" as *const u8 as *const i8, 97_u16, 0i32);
    primitive(b"mathchardef\x00" as *const u8 as *const i8, 97_u16, 1i32);
    primitive(
        b"XeTeXmathcharnumdef\x00" as *const u8 as *const i8,
        97_u16,
        8i32,
    );
    primitive(
        b"Umathcharnumdef\x00" as *const u8 as *const i8,
        97_u16,
        8i32,
    );
    primitive(
        b"XeTeXmathchardef\x00" as *const u8 as *const i8,
        97_u16,
        9i32,
    );
    primitive(b"Umathchardef\x00" as *const u8 as *const i8, 97_u16, 9i32);
    primitive(b"countdef\x00" as *const u8 as *const i8, 97_u16, 2i32);
    primitive(b"dimendef\x00" as *const u8 as *const i8, 97_u16, 3i32);
    primitive(b"skipdef\x00" as *const u8 as *const i8, 97_u16, 4i32);
    primitive(b"muskipdef\x00" as *const u8 as *const i8, 97_u16, 5i32);
    primitive(b"toksdef\x00" as *const u8 as *const i8, 97_u16, 6i32);
    primitive(
        b"catcode\x00" as *const u8 as *const i8,
        86_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32,
    );
    primitive(
        b"mathcode\x00" as *const u8 as *const i8,
        86_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"XeTeXmathcodenum\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"Umathcodenum\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"XeTeXmathcode\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32,
    );
    primitive(
        b"Umathcode\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32,
    );
    primitive(
        b"lccode\x00" as *const u8 as *const i8,
        86_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"uccode\x00" as *const u8 as *const i8,
        86_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"sfcode\x00" as *const u8 as *const i8,
        86_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"XeTeXcharclass\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"delcode\x00" as *const u8 as *const i8,
        86_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32,
    );
    primitive(
        b"XeTeXdelcodenum\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32,
    );
    primitive(
        b"Udelcodenum\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32,
    );
    primitive(
        b"XeTeXdelcode\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + 1i32,
    );
    primitive(
        b"Udelcode\x00" as *const u8 as *const i8,
        87_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + 1i32,
    );
    primitive(
        b"textfont\x00" as *const u8 as *const i8,
        88_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 0i32,
    );
    primitive(
        b"scriptfont\x00" as *const u8 as *const i8,
        88_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 256i32,
    );
    primitive(
        b"scriptscriptfont\x00" as *const u8 as *const i8,
        88_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 2i32 * 256i32,
    );
    primitive(b"hyphenation\x00" as *const u8 as *const i8, 101_u16, 0i32);
    primitive(b"patterns\x00" as *const u8 as *const i8, 101_u16, 1i32);
    primitive(b"hyphenchar\x00" as *const u8 as *const i8, 79_u16, 0i32);
    primitive(b"skewchar\x00" as *const u8 as *const i8, 79_u16, 1i32);
    primitive(b"lpcode\x00" as *const u8 as *const i8, 79_u16, 2i32);
    primitive(b"rpcode\x00" as *const u8 as *const i8, 79_u16, 3i32);
    primitive(b"batchmode\x00" as *const u8 as *const i8, 102_u16, 0i32);
    primitive(b"nonstopmode\x00" as *const u8 as *const i8, 102_u16, 1i32);
    primitive(b"scrollmode\x00" as *const u8 as *const i8, 102_u16, 2i32);
    primitive(
        b"errorstopmode\x00" as *const u8 as *const i8,
        102_u16,
        3i32,
    );
    primitive(b"openin\x00" as *const u8 as *const i8, 60_u16, 1i32);
    primitive(b"closein\x00" as *const u8 as *const i8, 60_u16, 0i32);
    primitive(b"message\x00" as *const u8 as *const i8, 58_u16, 0i32);
    primitive(b"errmessage\x00" as *const u8 as *const i8, 58_u16, 1i32);
    primitive(
        b"lowercase\x00" as *const u8 as *const i8,
        57_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32),
    );
    primitive(
        b"uppercase\x00" as *const u8 as *const i8,
        57_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32),
    );
    primitive(b"show\x00" as *const u8 as *const i8, 19_u16, 0i32);
    primitive(b"showbox\x00" as *const u8 as *const i8, 19_u16, 1i32);
    primitive(b"showthe\x00" as *const u8 as *const i8, 19_u16, 2i32);
    primitive(b"showlists\x00" as *const u8 as *const i8, 19_u16, 3i32);
    primitive(b"openout\x00" as *const u8 as *const i8, 59_u16, 0i32);
    primitive(b"write\x00" as *const u8 as *const i8, 59_u16, 1i32);
    write_loc = cur_val;
    primitive(b"closeout\x00" as *const u8 as *const i8, 59_u16, 2i32);
    primitive(b"special\x00" as *const u8 as *const i8, 59_u16, 3i32);
    (*hash.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 10i32) as isize,
    ))
    .s1 = maketexstring(b"special\x00" as *const u8 as *const i8);
    *eqtb.offset(
        (1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 15000i32 + 10i32) as isize,
    ) = *eqtb.offset(cur_val as isize);
    primitive(b"immediate\x00" as *const u8 as *const i8, 59_u16, 4i32);
    primitive(b"setlanguage\x00" as *const u8 as *const i8, 59_u16, 5i32);
    primitive(
        b"synctex\x00" as *const u8 as *const i8,
        74_u16,
        1i32 + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 83i32,
    );
    no_new_control_sequence = true;
}
unsafe extern "C" fn get_strings_started() {
    pool_ptr = 0i32;
    str_ptr = 0i32;
    *str_start.offset(0) = 0i32;
    str_ptr = 65536i32;
    if load_pool_strings(pool_size - string_vacancies) == 0i32 {
        _tt_abort(b"must increase pool_size\x00" as *const u8 as *const i8);
    };
}
/* xetex-errors */
/* xetex-math */
/* xetex-output */
/* xetex-pagebuilder */
/* xetex-scaledmath */
/* xetex-shipout */
/* Inlines */
/* Strings printed this way will end up in the .log as well
 * as the terminal output. */
/*41: The length of the current string in the pool */
/* Tectonic related functions */
/*:1001*/
#[no_mangle]
pub unsafe extern "C" fn tt_run_engine(
    mut dump_name: *mut i8,
    mut input_file_name: *mut i8,
) -> TTHistory {
    let mut font_k: i32 = 0;
    /* Miscellaneous initializations that were mostly originally done in the
     * main() driver routines. */
    /* Get our stdout handle */
    rust_stdout = ttstub_output_open_stdout();
    let mut len: size_t = strlen(dump_name);
    TEX_format_default = xmalloc(len.wrapping_add(1i32 as u64)) as *mut i8;
    strcpy(TEX_format_default, dump_name);
    format_default_length = len as i32;
    /* Not sure why these get custom initializations. */
    if file_line_error_style_p < 0i32 {
        file_line_error_style_p = 0i32
    }
    /* These various parameters were configurable in web2c TeX. We don't
     * bother to allow that. */
    pool_size = 6250000i64 as i32;
    string_vacancies = 90000i64 as i32;
    pool_free = 47500i64 as i32;
    max_strings = 565536i64 as i32;
    strings_free = 100i32;
    font_mem_size = 8000000i64 as i32;
    font_max = 9000i32;
    trie_size = 1000000i64 as i32;
    hyph_size = 8191i32;
    buf_size = 200000i64 as i32;
    nest_size = 500i32;
    max_in_open = 15i32;
    param_size = 10000i32;
    save_size = 80000i64 as i32;
    stack_size = 5000i32;
    error_line = 79i32;
    half_error_line = 50i32;
    max_print_line = 79i32;
    hash_extra = 600000i64 as i32;
    expand_depth = 10000i32;
    /* Allocate many of our big arrays. */
    buffer = xmalloc(
        ((buf_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<UnicodeScalar>() as u64),
    ) as *mut UnicodeScalar;
    nest = xmalloc(
        ((nest_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<list_state_record>() as u64),
    ) as *mut list_state_record;
    save_stack = xmalloc(
        ((save_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<memory_word>() as u64),
    ) as *mut memory_word;
    input_stack = xmalloc(
        ((stack_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<input_state_t>() as u64),
    ) as *mut input_state_t;
    input_file = xmalloc(
        ((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<*mut UFILE>() as u64),
    ) as *mut *mut UFILE;
    line_stack =
        xmalloc(((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
            as *mut i32;
    eof_seen =
        xmalloc(((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64))
            as *mut bool;
    grp_stack = xmalloc(
        ((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<save_pointer>() as u64),
    ) as *mut save_pointer;
    if_stack =
        xmalloc(((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
            as *mut i32;
    source_filename_stack = xmalloc(
        ((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    full_source_filename_stack = xmalloc(
        ((max_in_open + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    param_stack =
        xmalloc(((param_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
            as *mut i32;
    hyph_word = xmalloc(
        ((hyph_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<str_number>() as u64),
    ) as *mut str_number;
    hyph_list =
        xmalloc(((hyph_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64))
            as *mut i32;
    hyph_link = xmalloc(
        ((hyph_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<hyph_pointer>() as u64),
    ) as *mut hyph_pointer;
    /* First bit of initex handling: more allocations. */
    if in_initex_mode {
        mem = xmalloc(
            ((4999999i32 + 1i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<memory_word>() as u64),
        ) as *mut memory_word;
        eqtb_top = 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 23i32
            + 256i32
            - 1i32
            + hash_extra;
        if hash_extra == 0i32 {
            hash_top = 1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
        } else {
            hash_top = eqtb_top
        }
        yhash = xmalloc(
            ((1i32 + hash_top - 514i32 + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<b32x2>() as u64),
        ) as *mut b32x2;
        hash = yhash.offset(-514);
        (*hash.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32) as isize)).s0 =
            0i32;
        (*hash.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32) as isize)).s1 =
            0i32;
        hash_used = 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 + 1i32;
        while hash_used <= hash_top {
            *hash.offset(hash_used as isize) =
                *hash.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32) as isize);
            hash_used += 1
        }
        eqtb = xcalloc(
            (eqtb_top + 1i32) as size_t,
            ::std::mem::size_of::<memory_word>() as u64,
        ) as *mut memory_word;
        str_start = xmalloc(
            ((max_strings + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<pool_pointer>() as u64),
        ) as *mut pool_pointer;
        str_pool = xmalloc(
            ((pool_size + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<packed_UTF16_code>() as u64),
        ) as *mut packed_UTF16_code;
        font_info = xmalloc(
            ((font_mem_size + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<memory_word>() as u64),
        ) as *mut memory_word
    }
    /* Sanity-check various invariants. */
    history = TTHistory::FATAL_ERROR;
    bad = 0i32;
    if half_error_line < 30i32 || half_error_line > error_line - 15i32 {
        bad = 1i32
    }
    if max_print_line < 60i32 {
        bad = 2i32
    }
    if 1100i32 > 4999999i32 {
        bad = 4i32
    }
    if 8501i32 > 15000i32 {
        bad = 5i32
    }
    if max_in_open >= 128i32 {
        bad = 6i32
    }
    if 4999999i32 < 267i32 {
        bad = 7i32
    }
    if -0xfffffffi32 > 0i32 {
        bad = 12i32
    }
    if 9000i32 < -0xfffffffi32 || 9000i32 > 0x3fffffffi32 {
        bad = 15i32
    }
    if font_max > 0i32 + 9000i32 {
        bad = 16i32
    }
    if save_size > 0x3fffffffi32 || max_strings > 0x3fffffffi32 {
        bad = 17i32
    }
    if buf_size > 0x3fffffffi32 {
        bad = 18i32
    }
    if 0x1ffffffi32
        + (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 23i32
            + 256i32
            - 1i32)
        + hash_extra
        > 0x3fffffffi32
    {
        bad = 21i32
    }
    if 514i32 < 0i32 || 514i32 > 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 {
        bad = 42i32
    }
    if format_default_length > 2147483647i32 {
        bad = 31i32
    }
    if 2i32 * 0x3fffffffi32 < 4999999i32 {
        bad = 41i32
    }
    if bad > 0i32 {
        _tt_abort(
            b"failed internal consistency check #%d\x00" as *const u8 as *const i8,
            bad,
        );
    }
    /* OK, ready to keep on initializing. */
    initialize_more_variables();
    if in_initex_mode {
        get_strings_started();
        initialize_more_initex_variables();
        initialize_primitives();
        init_str_ptr = str_ptr;
        init_pool_ptr = pool_ptr
    }
    /*55:*/
    initialize_math_variables();
    initialize_pagebuilder_variables();
    initialize_shipout_variables();
    selector = Selector::TERM_ONLY;
    tally = 0i32;
    term_offset = 0i32;
    file_offset = 0i32;
    job_name = 0i32;
    name_in_progress = false;
    log_opened = false;
    if semantic_pagination_enabled {
        output_file_extension = b".spx\x00" as *const u8 as *const i8
    } else {
        output_file_extension = b".xdv\x00" as *const u8 as *const i8
    }
    input_ptr = 0i32;
    max_in_stack = 0i32;
    *source_filename_stack.offset(0) = 0i32;
    *full_source_filename_stack.offset(0) = 0i32;
    in_open = 0i32;
    open_parens = 0i32;
    max_buf_stack = 0i32;
    *grp_stack.offset(0) = 0i32;
    *if_stack.offset(0) = -0xfffffffi32;
    param_ptr = 0i32;
    max_param_stack = 0i32;
    used_tectonic_coda_tokens = false;
    gave_char_warning_help = false;
    memset(
        buffer as *mut libc::c_void,
        0i32,
        (buf_size as u64).wrapping_mul(::std::mem::size_of::<UnicodeScalar>() as u64),
    );
    first = 0i32;
    scanner_status = 0_u8;
    warning_index = -0xfffffffi32;
    first = 1i32;
    cur_input.state = 33_u16;
    cur_input.start = 1i32;
    cur_input.index = 0_u16;
    line = 0i32;
    cur_input.name = 0i32;
    force_eof = false;
    align_state = 1000000i64 as i32;
    init_io();
    if in_initex_mode {
        no_new_control_sequence = false;
        primitive(b"XeTeXpicfile\x00" as *const u8 as *const i8, 59_u16, 41i32);
        primitive(b"XeTeXpdffile\x00" as *const u8 as *const i8, 59_u16, 42i32);
        primitive(b"XeTeXglyph\x00" as *const u8 as *const i8, 59_u16, 43i32);
        primitive(
            b"XeTeXlinebreaklocale\x00" as *const u8 as *const i8,
            59_u16,
            46i32,
        );
        primitive(
            b"pdfsavepos\x00" as *const u8 as *const i8,
            59_u16,
            6i32 + 0i32,
        );
        primitive(b"lastnodetype\x00" as *const u8 as *const i8, 71_u16, 3i32);
        primitive(b"eTeXversion\x00" as *const u8 as *const i8, 71_u16, 6i32);
        primitive(b"eTeXrevision\x00" as *const u8 as *const i8, 110_u16, 5i32);
        primitive(b"XeTeXversion\x00" as *const u8 as *const i8, 71_u16, 14i32);
        primitive(
            b"XeTeXrevision\x00" as *const u8 as *const i8,
            110_u16,
            6i32,
        );
        primitive(
            b"XeTeXcountglyphs\x00" as *const u8 as *const i8,
            71_u16,
            15i32,
        );
        primitive(
            b"XeTeXcountvariations\x00" as *const u8 as *const i8,
            71_u16,
            16i32,
        );
        primitive(
            b"XeTeXvariation\x00" as *const u8 as *const i8,
            71_u16,
            17i32,
        );
        primitive(
            b"XeTeXfindvariationbyname\x00" as *const u8 as *const i8,
            71_u16,
            18i32,
        );
        primitive(
            b"XeTeXvariationmin\x00" as *const u8 as *const i8,
            71_u16,
            19i32,
        );
        primitive(
            b"XeTeXvariationmax\x00" as *const u8 as *const i8,
            71_u16,
            20i32,
        );
        primitive(
            b"XeTeXvariationdefault\x00" as *const u8 as *const i8,
            71_u16,
            21i32,
        );
        primitive(
            b"XeTeXcountfeatures\x00" as *const u8 as *const i8,
            71_u16,
            22i32,
        );
        primitive(
            b"XeTeXfeaturecode\x00" as *const u8 as *const i8,
            71_u16,
            23i32,
        );
        primitive(
            b"XeTeXfindfeaturebyname\x00" as *const u8 as *const i8,
            71_u16,
            24i32,
        );
        primitive(
            b"XeTeXisexclusivefeature\x00" as *const u8 as *const i8,
            71_u16,
            25i32,
        );
        primitive(
            b"XeTeXcountselectors\x00" as *const u8 as *const i8,
            71_u16,
            26i32,
        );
        primitive(
            b"XeTeXselectorcode\x00" as *const u8 as *const i8,
            71_u16,
            27i32,
        );
        primitive(
            b"XeTeXfindselectorbyname\x00" as *const u8 as *const i8,
            71_u16,
            28i32,
        );
        primitive(
            b"XeTeXisdefaultselector\x00" as *const u8 as *const i8,
            71_u16,
            29i32,
        );
        primitive(
            b"XeTeXvariationname\x00" as *const u8 as *const i8,
            110_u16,
            7i32,
        );
        primitive(
            b"XeTeXfeaturename\x00" as *const u8 as *const i8,
            110_u16,
            8i32,
        );
        primitive(
            b"XeTeXselectorname\x00" as *const u8 as *const i8,
            110_u16,
            9i32,
        );
        primitive(
            b"XeTeXOTcountscripts\x00" as *const u8 as *const i8,
            71_u16,
            30i32,
        );
        primitive(
            b"XeTeXOTcountlanguages\x00" as *const u8 as *const i8,
            71_u16,
            31i32,
        );
        primitive(
            b"XeTeXOTcountfeatures\x00" as *const u8 as *const i8,
            71_u16,
            32i32,
        );
        primitive(
            b"XeTeXOTscripttag\x00" as *const u8 as *const i8,
            71_u16,
            33i32,
        );
        primitive(
            b"XeTeXOTlanguagetag\x00" as *const u8 as *const i8,
            71_u16,
            34i32,
        );
        primitive(
            b"XeTeXOTfeaturetag\x00" as *const u8 as *const i8,
            71_u16,
            35i32,
        );
        primitive(
            b"XeTeXcharglyph\x00" as *const u8 as *const i8,
            71_u16,
            36i32,
        );
        primitive(
            b"XeTeXglyphindex\x00" as *const u8 as *const i8,
            71_u16,
            37i32,
        );
        primitive(
            b"XeTeXglyphbounds\x00" as *const u8 as *const i8,
            71_u16,
            47i32,
        );
        primitive(
            b"XeTeXglyphname\x00" as *const u8 as *const i8,
            110_u16,
            10i32,
        );
        primitive(
            b"XeTeXfonttype\x00" as *const u8 as *const i8,
            71_u16,
            38i32,
        );
        primitive(
            b"XeTeXfirstfontchar\x00" as *const u8 as *const i8,
            71_u16,
            39i32,
        );
        primitive(
            b"XeTeXlastfontchar\x00" as *const u8 as *const i8,
            71_u16,
            40i32,
        );
        primitive(b"pdflastxpos\x00" as *const u8 as *const i8, 71_u16, 41i32);
        primitive(b"pdflastypos\x00" as *const u8 as *const i8, 71_u16, 42i32);
        primitive(b"strcmp\x00" as *const u8 as *const i8, 110_u16, 43i32);
        primitive(b"mdfivesum\x00" as *const u8 as *const i8, 110_u16, 44i32);
        primitive(
            b"pdfmdfivesum\x00" as *const u8 as *const i8,
            110_u16,
            44i32,
        );
        primitive(b"shellescape\x00" as *const u8 as *const i8, 71_u16, 45i32);
        primitive(
            b"XeTeXpdfpagecount\x00" as *const u8 as *const i8,
            71_u16,
            46i32,
        );
        primitive(
            b"tracingassigns\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 58i32,
        );
        primitive(
            b"tracinggroups\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 59i32,
        );
        primitive(
            b"tracingifs\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 60i32,
        );
        primitive(
            b"tracingscantokens\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 61i32,
        );
        primitive(
            b"tracingnesting\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 62i32,
        );
        primitive(
            b"predisplaydirection\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 63i32,
        );
        primitive(
            b"lastlinefit\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 64i32,
        );
        primitive(
            b"savingvdiscards\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 65i32,
        );
        primitive(
            b"savinghyphcodes\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 66i32,
        );
        primitive(
            b"currentgrouplevel\x00" as *const u8 as *const i8,
            71_u16,
            7i32,
        );
        primitive(
            b"currentgrouptype\x00" as *const u8 as *const i8,
            71_u16,
            8i32,
        );
        primitive(
            b"currentiflevel\x00" as *const u8 as *const i8,
            71_u16,
            9i32,
        );
        primitive(
            b"currentiftype\x00" as *const u8 as *const i8,
            71_u16,
            10i32,
        );
        primitive(
            b"currentifbranch\x00" as *const u8 as *const i8,
            71_u16,
            11i32,
        );
        primitive(b"fontcharwd\x00" as *const u8 as *const i8, 71_u16, 48i32);
        primitive(b"fontcharht\x00" as *const u8 as *const i8, 71_u16, 49i32);
        primitive(b"fontchardp\x00" as *const u8 as *const i8, 71_u16, 50i32);
        primitive(b"fontcharic\x00" as *const u8 as *const i8, 71_u16, 51i32);
        primitive(
            b"parshapelength\x00" as *const u8 as *const i8,
            71_u16,
            52i32,
        );
        primitive(
            b"parshapeindent\x00" as *const u8 as *const i8,
            71_u16,
            53i32,
        );
        primitive(
            b"parshapedimen\x00" as *const u8 as *const i8,
            71_u16,
            54i32,
        );
        primitive(b"showgroups\x00" as *const u8 as *const i8, 19_u16, 4i32);
        primitive(b"showtokens\x00" as *const u8 as *const i8, 19_u16, 5i32);
        primitive(b"unexpanded\x00" as *const u8 as *const i8, 111_u16, 1i32);
        primitive(b"detokenize\x00" as *const u8 as *const i8, 111_u16, 5i32);
        primitive(b"showifs\x00" as *const u8 as *const i8, 19_u16, 6i32);
        primitive(
            b"interactionmode\x00" as *const u8 as *const i8,
            83_u16,
            2i32,
        );
        primitive(b"middle\x00" as *const u8 as *const i8, 49_u16, 1i32);
        primitive(
            b"suppressfontnotfounderror\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 67i32,
        );
        primitive(
            b"TeXXeTstate\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 71i32,
        );
        primitive(
            b"XeTeXupwardsmode\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 73i32,
        );
        primitive(
            b"XeTeXuseglyphmetrics\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 74i32,
        );
        primitive(
            b"XeTeXinterchartokenstate\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 75i32,
        );
        primitive(
            b"XeTeXdashbreakstate\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 72i32,
        );
        primitive(
            b"XeTeXinputnormalization\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 76i32,
        );
        primitive(
            b"XeTeXtracingfonts\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 79i32,
        );
        primitive(
            b"XeTeXinterwordspaceshaping\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 80i32,
        );
        primitive(
            b"XeTeXgenerateactualtext\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 81i32,
        );
        primitive(
            b"XeTeXhyphenatablelength\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 82i32,
        );
        primitive(
            b"pdfoutput\x00" as *const u8 as *const i8,
            74_u16,
            1i32 + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 84i32,
        );
        primitive(
            b"XeTeXinputencoding\x00" as *const u8 as *const i8,
            59_u16,
            44i32,
        );
        primitive(
            b"XeTeXdefaultencoding\x00" as *const u8 as *const i8,
            59_u16,
            45i32,
        );
        primitive(b"beginL\x00" as *const u8 as *const i8, 33_u16, 6i32);
        primitive(b"endL\x00" as *const u8 as *const i8, 33_u16, 7i32);
        primitive(b"beginR\x00" as *const u8 as *const i8, 33_u16, 10i32);
        primitive(b"endR\x00" as *const u8 as *const i8, 33_u16, 11i32);
        primitive(b"scantokens\x00" as *const u8 as *const i8, 106_u16, 2i32);
        primitive(b"readline\x00" as *const u8 as *const i8, 98_u16, 1i32);
        primitive(b"unless\x00" as *const u8 as *const i8, 104_u16, 1i32);
        primitive(b"ifdefined\x00" as *const u8 as *const i8, 107_u16, 17i32);
        primitive(b"ifcsname\x00" as *const u8 as *const i8, 107_u16, 18i32);
        primitive(b"iffontchar\x00" as *const u8 as *const i8, 107_u16, 19i32);
        primitive(b"ifincsname\x00" as *const u8 as *const i8, 107_u16, 20i32);
        primitive(b"protected\x00" as *const u8 as *const i8, 95_u16, 8i32);
        primitive(
            b"numexpr\x00" as *const u8 as *const i8,
            71_u16,
            59i32 + 0i32,
        );
        primitive(
            b"dimexpr\x00" as *const u8 as *const i8,
            71_u16,
            59i32 + 1i32,
        );
        primitive(
            b"glueexpr\x00" as *const u8 as *const i8,
            71_u16,
            59i32 + 2i32,
        );
        primitive(
            b"muexpr\x00" as *const u8 as *const i8,
            71_u16,
            59i32 + 3i32,
        );
        primitive(
            b"gluestretchorder\x00" as *const u8 as *const i8,
            71_u16,
            12i32,
        );
        primitive(
            b"glueshrinkorder\x00" as *const u8 as *const i8,
            71_u16,
            13i32,
        );
        primitive(b"gluestretch\x00" as *const u8 as *const i8, 71_u16, 55i32);
        primitive(b"glueshrink\x00" as *const u8 as *const i8, 71_u16, 56i32);
        primitive(b"mutoglue\x00" as *const u8 as *const i8, 71_u16, 57i32);
        primitive(b"gluetomu\x00" as *const u8 as *const i8, 71_u16, 58i32);
        primitive(b"marks\x00" as *const u8 as *const i8, 18_u16, 5i32);
        primitive(
            b"topmarks\x00" as *const u8 as *const i8,
            112_u16,
            0i32 + 5i32,
        );
        primitive(
            b"firstmarks\x00" as *const u8 as *const i8,
            112_u16,
            1i32 + 5i32,
        );
        primitive(
            b"botmarks\x00" as *const u8 as *const i8,
            112_u16,
            2i32 + 5i32,
        );
        primitive(
            b"splitfirstmarks\x00" as *const u8 as *const i8,
            112_u16,
            3i32 + 5i32,
        );
        primitive(
            b"splitbotmarks\x00" as *const u8 as *const i8,
            112_u16,
            4i32 + 5i32,
        );
        primitive(b"pagediscards\x00" as *const u8 as *const i8, 24_u16, 2i32);
        primitive(b"splitdiscards\x00" as *const u8 as *const i8, 24_u16, 3i32);
        primitive(
            b"interlinepenalties\x00" as *const u8 as *const i8,
            85_u16,
            1i32 + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 0,
        );
        primitive(
            b"clubpenalties\x00" as *const u8 as *const i8,
            85_u16,
            1i32 + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 1,
        );
        primitive(
            b"widowpenalties\x00" as *const u8 as *const i8,
            85_u16,
            1i32 + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 2,
        );
        primitive(
            b"displaywidowpenalties\x00" as *const u8 as *const i8,
            85_u16,
            1i32 + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 3,
        );
        max_reg_num = 32767i32;
        max_reg_help_line =
            b"A register number must be between 0 and 32767.\x00" as *const u8 as *const i8
    }
    no_new_control_sequence = true;
    if !in_initex_mode {
        if !load_fmt_file() {
            return history;
        }
    }
    if (*eqtb.offset(
        (1i32
            + (0x10ffff + 1)
            + (0x10ffff + 1)
            + 1
            + 15000
            + 12
            + 9000
            + 1
            + 1
            + 19
            + 256
            + 256
            + 13
            + 256
            + 4
            + 256
            + 1
            + 3 * 256
            + (0x10ffff + 1)
            + (0x10ffff + 1)
            + (0x10ffff + 1)
            + (0x10ffff + 1)
            + (0x10ffff + 1)
            + (0x10ffff + 1)
            + 48) as isize,
    ))
    .b32
    .s1 < 0i32
        || (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 48) as isize,
        ))
        .b32
        .s1 > 0xffffi32
    {
        cur_input.limit -= 1
    } else {
        *buffer.offset(cur_input.limit as isize) = (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 48) as isize,
        ))
        .b32
        .s1
    }
    if in_initex_mode {
        /* TeX initializes with the real date and time, but for format file
         * reproducibility we do this: */
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 20) as isize,
        ))
        .b32
        .s1 = 0i32; /*:79*/
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 21) as isize,
        ))
        .b32
        .s1 = 0i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 22) as isize,
        ))
        .b32
        .s1 = 0i32;
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 23) as isize,
        ))
        .b32
        .s1 = 0i32
    } else {
        let (minutes, day, month, year) = get_date_and_time();
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 20) as isize,
        ))
        .b32
        .s1 = minutes;
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 21) as isize,
        ))
        .b32
        .s1 = day;
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 22) as isize,
        ))
        .b32
        .s1 = month;
        (*eqtb.offset(
            (1i32
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 1
                + 15000
                + 12
                + 9000
                + 1
                + 1
                + 19
                + 256
                + 256
                + 13
                + 256
                + 4
                + 256
                + 1
                + 3 * 256
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + (0x10ffff + 1)
                + 23) as isize,
        ))
        .b32
        .s1 = year;
    }
    if trie_not_ready {
        trie_trl = xmalloc(
            ((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<trie_pointer>() as u64),
        ) as *mut trie_pointer;
        trie_tro = xmalloc(
            ((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<trie_pointer>() as u64),
        ) as *mut trie_pointer;
        trie_trc =
            xmalloc(((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<u16>() as u64))
                as *mut u16;
        trie_c = xmalloc(
            ((trie_size + 1i32) as u64)
                .wrapping_mul(::std::mem::size_of::<packed_UTF16_code>() as u64),
        ) as *mut packed_UTF16_code;
        trie_o = xmalloc(
            ((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<trie_opcode>() as u64),
        ) as *mut trie_opcode;
        trie_l = xmalloc(
            ((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<trie_pointer>() as u64),
        ) as *mut trie_pointer;
        trie_r = xmalloc(
            ((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<trie_pointer>() as u64),
        ) as *mut trie_pointer;
        trie_hash = xmalloc(
            ((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<trie_pointer>() as u64),
        ) as *mut trie_pointer;
        trie_taken =
            xmalloc(((trie_size + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64))
                as *mut bool;
        *trie_l.offset(0) = 0i32;
        *trie_c.offset(0) = 0i32 as packed_UTF16_code;
        trie_ptr = 0i32;
        *trie_r.offset(0) = 0i32;
        hyph_start = 0i32;
        font_mapping = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<*mut libc::c_void>() as u64,
        ) as *mut *mut libc::c_void;
        font_layout_engine = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<*mut libc::c_void>() as u64,
        ) as *mut *mut libc::c_void;
        font_flags = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i8>() as u64,
        ) as *mut i8;
        font_letter_space = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<scaled_t>() as u64,
        ) as *mut scaled_t;
        font_check = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<b16x4>() as u64,
        ) as *mut b16x4;
        font_size = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<scaled_t>() as u64,
        ) as *mut scaled_t;
        font_dsize = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<scaled_t>() as u64,
        ) as *mut scaled_t;
        font_params = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<font_index>() as u64,
        ) as *mut font_index;
        font_name = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<str_number>() as u64,
        ) as *mut str_number;
        font_area = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<str_number>() as u64,
        ) as *mut str_number;
        font_bc = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<UTF16_code>() as u64,
        ) as *mut UTF16_code;
        font_ec = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<UTF16_code>() as u64,
        ) as *mut UTF16_code;
        font_glue = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        hyphen_char = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        skew_char = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        bchar_label = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<font_index>() as u64,
        ) as *mut font_index;
        font_bchar = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<nine_bits>() as u64,
        ) as *mut nine_bits;
        font_false_bchar = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<nine_bits>() as u64,
        ) as *mut nine_bits;
        char_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        width_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        height_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        depth_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        italic_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        lig_kern_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        kern_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        exten_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        param_base = xcalloc(
            (font_max + 1i32) as size_t,
            ::std::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        font_ptr = 0i32;
        fmem_ptr = 7i32;
        *font_name.offset(0) = maketexstring(b"nullfont\x00" as *const u8 as *const i8);
        *font_area.offset(0) = (65536 + 1i32 as i64) as str_number;
        *hyphen_char.offset(0) = '-' as i32;
        *skew_char.offset(0) = -1i32;
        *bchar_label.offset(0) = 0i32;
        *font_bchar.offset(0) = 65536i32;
        *font_false_bchar.offset(0) = 65536i32;
        *font_bc.offset(0) = 1i32 as UTF16_code;
        *font_ec.offset(0) = 0i32 as UTF16_code;
        *font_size.offset(0) = 0i32;
        *font_dsize.offset(0) = 0i32;
        *char_base.offset(0) = 0i32;
        *width_base.offset(0) = 0i32;
        *height_base.offset(0) = 0i32;
        *depth_base.offset(0) = 0i32;
        *italic_base.offset(0) = 0i32;
        *lig_kern_base.offset(0) = 0i32;
        *kern_base.offset(0) = 0i32;
        *exten_base.offset(0) = 0i32;
        *font_glue.offset(0) = -0xfffffffi32;
        *font_params.offset(0) = 7i32;
        let ref mut fresh21 = *font_mapping.offset(0);
        *fresh21 = 0 as *mut libc::c_void;
        *param_base.offset(0) = -1i32;
        font_k = 0i32;
        while font_k <= 6i32 {
            (*font_info.offset(font_k as isize)).b32.s1 = 0i32;
            font_k += 1
        }
    }
    font_used =
        xmalloc(((font_max + 1i32) as u64).wrapping_mul(::std::mem::size_of::<bool>() as u64))
            as *mut bool;
    font_k = 0i32;
    while font_k <= font_max {
        *font_used.offset(font_k as isize) = false;
        font_k += 1
    }
    if interaction as i32 == 0i32 {
        selector = Selector::NO_PRINT
    } else {
        selector = Selector::TERM_ONLY
    }
    if semantic_pagination_enabled {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 81i32) as isize,
        ))
        .b32
        .s1 = 1i32
    }
    pdf_files_init();
    synctex_init_command();
    start_input(input_file_name);
    history = TTHistory::SPOTLESS;
    main_control();
    final_cleanup();
    close_files_and_terminate();
    pdf_files_close();
    free(TEX_format_default as *mut libc::c_void);
    free(font_used as *mut libc::c_void);
    deinitialize_shipout_variables();
    destroy_font_manager();
    font_k = 0i32;
    while font_k < font_max {
        if !(*font_layout_engine.offset(font_k as isize)).is_null() {
            release_font_engine(
                *font_layout_engine.offset(font_k as isize),
                *font_area.offset(font_k as isize),
            );
            let ref mut fresh22 = *font_layout_engine.offset(font_k as isize);
            *fresh22 = 0 as *mut libc::c_void
        }
        font_k += 1
    }
    // Free the big allocated arrays
    free(buffer as *mut libc::c_void);
    free(nest as *mut libc::c_void);
    free(save_stack as *mut libc::c_void);
    free(input_stack as *mut libc::c_void);
    free(input_file as *mut libc::c_void);
    free(line_stack as *mut libc::c_void);
    free(eof_seen as *mut libc::c_void);
    free(grp_stack as *mut libc::c_void);
    free(if_stack as *mut libc::c_void);
    free(source_filename_stack as *mut libc::c_void);
    free(full_source_filename_stack as *mut libc::c_void);
    free(param_stack as *mut libc::c_void);
    free(hyph_word as *mut libc::c_void);
    free(hyph_list as *mut libc::c_void);
    free(hyph_link as *mut libc::c_void);
    // initialize_more_variables @ 3277
    free(native_text as *mut libc::c_void);
    // Free arrays allocated in load_fmt_file
    free(yhash as *mut libc::c_void);
    free(eqtb as *mut libc::c_void);
    free(mem as *mut libc::c_void);
    free(str_start as *mut libc::c_void);
    free(str_pool as *mut libc::c_void);
    free(font_info as *mut libc::c_void);
    free(font_mapping as *mut libc::c_void);
    free(font_layout_engine as *mut libc::c_void);
    free(font_flags as *mut libc::c_void);
    free(font_letter_space as *mut libc::c_void);
    free(font_check as *mut libc::c_void);
    free(font_size as *mut libc::c_void);
    free(font_dsize as *mut libc::c_void);
    free(font_params as *mut libc::c_void);
    free(font_name as *mut libc::c_void);
    free(font_area as *mut libc::c_void);
    free(font_bc as *mut libc::c_void);
    free(font_ec as *mut libc::c_void);
    free(font_glue as *mut libc::c_void);
    free(hyphen_char as *mut libc::c_void);
    free(skew_char as *mut libc::c_void);
    free(bchar_label as *mut libc::c_void);
    free(font_bchar as *mut libc::c_void);
    free(font_false_bchar as *mut libc::c_void);
    free(char_base as *mut libc::c_void);
    free(width_base as *mut libc::c_void);
    free(height_base as *mut libc::c_void);
    free(depth_base as *mut libc::c_void);
    free(italic_base as *mut libc::c_void);
    free(lig_kern_base as *mut libc::c_void);
    free(kern_base as *mut libc::c_void);
    free(exten_base as *mut libc::c_void);
    free(param_base as *mut libc::c_void);
    trie_trl = mfree(trie_trl as *mut libc::c_void) as *mut trie_pointer;
    trie_tro = mfree(trie_tro as *mut libc::c_void) as *mut trie_pointer;
    trie_trc = mfree(trie_trc as *mut libc::c_void) as *mut u16;
    history
}
