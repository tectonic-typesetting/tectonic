#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]
#![feature(c_variadic)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type size_t = u64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;

extern "C" {
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    #[no_mangle]
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::std::ffi::VaList) -> i32;
    /* tectonic/bibtex.h
       Copyright 2017 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn bibtex_main(aux_file_name: *const i8) -> tt_history_t;
    /*  DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
        the DVIPDFMx project team.

        Copyright (c) 2006 SIL. (xdvipdfmx extensions for XeTeX support)

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn dvipdfmx_main(
        pdfname: *const i8,
        dviname: *const i8,
        pagespec: *const i8,
        opt_flags: i32,
        translate: bool,
        compress: bool,
        deterministic_tags: bool,
        quiet: bool,
        verbose: u32,
    ) -> i32;
    /* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
       Copyright 2016-2018 The Tectonic Project
       Licensed under the MIT License.
    */
    /* Extra stuff used in various change files for various reasons.  */
    /* Array allocations. Add 1 to size to account for Pascal indexing convention. */
    /*11:*/
    /*18: */
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
    /*WORDS_BIGENDIAN*/
    /* ## THE ORIGINAL SITUATION (archived for posterity)
     *
     * In XeTeX, a "quarterword" is 16 bits. Who knows why. A "halfword" is,
     * sensibly, 32 bits. A "memory word" is a full word: either four quarters or
     * two halves: i.e., 64 bits. The memory word union also has options for
     * doubles (called `gr`), `integer` which is an int32_t (called `cint`), and a
     * pointer (`ptr`).
     *
     * Original struct definition, LITTLE ENDIAN (condensed):
     *
     *   typedef union {
     *       struct { int32_t LH, RH; } v;
     *       struct { short B1, B0; } u;
     *   } two_halves;
     *
     *   typedef struct {
     *       struct { uint16_t B3, B2, B1, B0; } u;
     *   } four_quarters;
     *
     *   typedef union {
     *       two_halves hh;
     *
     *       struct {
     *           int32_t junk;
     *           int32_t CINT;
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
     *       struct { int32_t RH, LH; } v;
     *       struct {
     *           int32_t junk;
     *           short B0, B1;
     *       } u;
     *   } two_halves;
     *
     *   typedef struct {
     *       struct { uint16_t B0, B1, B2, B3; } u;
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
    /* enum: normal .. filll */
    /* range: 0 .. 0x1FF */
    /* which mode we are: horz, vert, etc. */
    /* pointer to head of list being built */
    /* pointer to tail of list being built */
    /* LR_save or LR_box or delim_ptr */
    /* number of lines that have already been put into the current vlist */
    /* source line number at which this level was entered */
    /* prev_depth or space_factor/clang or incompleat_noad */
    /* tokenizer state: mid_line, skip_blanks, new_line */
    /* index of this level of input in input_file array */
    /* position of beginning of current line in `buffer` */
    /* position of next character to read in `buffer` */
    /* position of end of line in `buffer` */
    /* string number: name of current file or magic value for terminal, etc. */
    /* Functions originating in texmfmp.c */
    /* Needed here for UFILE */
    /* variables! */
    /* All the following variables are defined in xetexini.c */
    /* should be internal to shipout, but accessed by synctex */
    /*:1683*/
    /* It looks like these arrays are set up so that they can be safely indexed
     * with negative indices. The underlying arrays used to be named "zzzaa" and
     * "zzzbb". */
    /* the former xetexcoerce.h: */
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
    #[no_mangle]
    fn tt_run_engine(dump_name: *const i8, input_file_name: *const i8) -> tt_history_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_bridge_api_t {
    pub context: *mut libc::c_void,
    pub issue_warning: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8) -> ()>,
    pub issue_error: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8) -> ()>,
    pub get_file_md5:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: *mut i8) -> i32>,
    pub get_data_md5: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: size_t, _: *mut i8) -> i32,
    >,
    pub output_open: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: i32) -> rust_output_handle_t,
    >,
    pub output_open_stdout:
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> rust_output_handle_t>,
    pub output_putc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_output_handle_t, _: i32) -> i32>,
    pub output_write: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: rust_output_handle_t,
            _: *const i8,
            _: size_t,
        ) -> size_t,
    >,
    pub output_flush:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_output_handle_t) -> i32>,
    pub output_close:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_output_handle_t) -> i32>,
    pub input_open: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const i8,
            _: tt_input_format_type,
            _: i32,
        ) -> rust_input_handle_t,
    >,
    pub input_open_primary:
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> rust_input_handle_t>,
    pub input_get_size:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t) -> size_t>,
    pub input_seek: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: rust_input_handle_t,
            _: ssize_t,
            _: i32,
            _: *mut i32,
        ) -> size_t,
    >,
    pub input_read: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: rust_input_handle_t,
            _: *mut i8,
            _: size_t,
        ) -> ssize_t,
    >,
    pub input_getc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t) -> i32>,
    pub input_ungetc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t, _: i32) -> i32>,
    pub input_close:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t) -> i32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [i64; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
/* tectonic/core-bridge.c: the C/C++ => Rust bridge
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/
/*vsnprintf*/
/* TODO: these are needed for the various *_main routines which should
 * probably be moved out into other files. */
/* The global variable that represents the Rust API. Some fine day we'll get
 * rid of all of the globals ... */
static mut tectonic_global_bridge: *const tt_bridge_api_t = std::ptr::null();
static mut jump_buffer: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut error_buf: [i8; 1024] = [
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
#[no_mangle]
pub unsafe extern "C" fn _tt_abort(mut format: *const i8, mut args: ...) -> ! {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        error_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    longjmp(jump_buffer.as_mut_ptr(), 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_error_message() -> *const i8 {
    return error_buf.as_mut_ptr();
}
/* Running the actual engines. Those code needs to be centralized for unified
 * setjmp aborts and error message extraction. */
#[no_mangle]
pub unsafe extern "C" fn tex_simple_main(
    mut api: *const tt_bridge_api_t,
    mut dump_name: *const i8,
    mut input_file_name: *const i8,
) -> i32 {
    let mut rv: i32 = 0;
    tectonic_global_bridge = api;
    if _setjmp(jump_buffer.as_mut_ptr()) != 0 {
        tectonic_global_bridge = 0 as *mut tt_bridge_api_t;
        return HISTORY_FATAL_ERROR as i32;
    }
    rv = tt_run_engine(dump_name, input_file_name) as i32;
    tectonic_global_bridge = 0 as *mut tt_bridge_api_t;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn dvipdfmx_simple_main(
    mut api: *const tt_bridge_api_t,
    mut dviname: *const i8,
    mut pdfname: *const i8,
    mut compress: bool,
    mut deterministic_tags: bool,
) -> i32 {
    let mut rv: i32 = 0;
    tectonic_global_bridge = api;
    if _setjmp(jump_buffer.as_mut_ptr()) != 0 {
        tectonic_global_bridge = 0 as *mut tt_bridge_api_t;
        return 99i32;
    }
    rv = dvipdfmx_main(
        pdfname,
        dviname,
        0 as *const i8,
        0i32,
        0i32 != 0,
        compress,
        deterministic_tags,
        0i32 != 0,
        0i32 as u32,
    );
    tectonic_global_bridge = 0 as *mut tt_bridge_api_t;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn bibtex_simple_main(
    mut api: *const tt_bridge_api_t,
    mut aux_file_name: *const i8,
) -> i32 {
    let mut rv: i32 = 0;
    tectonic_global_bridge = api;
    if _setjmp(jump_buffer.as_mut_ptr()) != 0 {
        tectonic_global_bridge = 0 as *const tt_bridge_api_t;
        return 99i32;
    }
    rv = bibtex_main(aux_file_name) as i32;
    tectonic_global_bridge = 0 as *const tt_bridge_api_t;
    return rv;
}
/* Global symbols that route through the global API variable. Hopefully we
 * will one day eliminate all of the global state and get rid of all of
 * these. */
/* Global symbols that route through the global API */
#[no_mangle]
pub unsafe extern "C" fn ttstub_issue_warning(mut format: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl; /* Not ideal to (ab)use error_buf here */
    ap = args.clone(); /* Not ideal to (ab)use error_buf here */
    vsnprintf(
        error_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    (*tectonic_global_bridge)
        .issue_warning
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        error_buf.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_issue_error(mut format: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        error_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    (*tectonic_global_bridge)
        .issue_error
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        error_buf.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_fprintf(
    mut handle: rust_output_handle_t,
    mut format: *const i8,
    mut args: ...
) -> i32 {
    static mut fprintf_buf: [i8; 1024] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut len: i32 = vsnprintf(
        fprintf_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    if len >= 1024i32 {
        len = 1024i32 - 1i32;
        fprintf_buf[len as usize] = '\u{0}' as i32 as i8
    }
    if len >= 0i32 {
        ttstub_output_write(handle, fprintf_buf.as_mut_ptr(), len as size_t);
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_get_file_md5(mut path: *const i8, mut digest: *mut i8) -> i32 {
    return (*tectonic_global_bridge)
        .get_file_md5
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, path, digest
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_get_data_md5(
    mut data: *const i8,
    mut len: size_t,
    mut digest: *mut i8,
) -> i32 {
    return (*tectonic_global_bridge)
        .get_data_md5
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, data, len, digest
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_open(
    mut path: *const i8,
    mut is_gz: i32,
) -> rust_output_handle_t {
    return (*tectonic_global_bridge)
        .output_open
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, path, is_gz
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_open_stdout() -> rust_output_handle_t {
    return (*tectonic_global_bridge)
        .output_open_stdout
        .expect("non-null function pointer")((*tectonic_global_bridge).context);
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_putc(mut handle: rust_output_handle_t, mut c: i32) -> i32 {
    return (*tectonic_global_bridge)
        .output_putc
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, handle, c
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_write(
    mut handle: rust_output_handle_t,
    mut data: *const i8,
    mut len: size_t,
) -> size_t {
    return (*tectonic_global_bridge)
        .output_write
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, handle, data, len
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_flush(mut handle: rust_output_handle_t) -> i32 {
    return (*tectonic_global_bridge)
        .output_flush
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle);
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_close(mut handle: rust_output_handle_t) -> i32 {
    return (*tectonic_global_bridge)
        .output_close
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle);
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_open(
    mut path: *const i8,
    mut format: tt_input_format_type,
    mut is_gz: i32,
) -> rust_input_handle_t {
    return (*tectonic_global_bridge)
        .input_open
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        path,
        format,
        is_gz,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_open_primary() -> rust_input_handle_t {
    return (*tectonic_global_bridge)
        .input_open_primary
        .expect("non-null function pointer")((*tectonic_global_bridge).context);
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_get_size(mut handle: rust_input_handle_t) -> size_t {
    return (*tectonic_global_bridge)
        .input_get_size
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle);
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_seek(
    mut handle: rust_input_handle_t,
    mut offset: ssize_t,
    mut whence: i32,
) -> size_t {
    let mut internal_error: i32 = 0i32;
    let mut rv: size_t = (*tectonic_global_bridge)
        .input_seek
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        handle,
        offset,
        whence,
        &mut internal_error,
    );
    if internal_error != 0 {
        // Nonzero indicates a serious internal error.
        longjmp(jump_buffer.as_mut_ptr(), 1i32);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_read(
    mut handle: rust_input_handle_t,
    mut data: *mut i8,
    mut len: size_t,
) -> ssize_t {
    return (*tectonic_global_bridge)
        .input_read
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, handle, data, len
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_getc(mut handle: rust_input_handle_t) -> i32 {
    return (*tectonic_global_bridge)
        .input_getc
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle);
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_ungetc(mut handle: rust_input_handle_t, mut ch: i32) -> i32 {
    return (*tectonic_global_bridge)
        .input_ungetc
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, handle, ch
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_close(mut handle: rust_input_handle_t) -> i32 {
    if (*tectonic_global_bridge)
        .input_close
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle)
        != 0
    {
        // Nonzero return value indicates a serious internal error.
        longjmp(jump_buffer.as_mut_ptr(), 1i32);
    }
    return 0i32;
}

extern "C" {
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
}

/* tectonic/core-memory.c: basic C dynamic memory helpers

Copyright 1993, 1994, 1995, 2008, 2009, 2010, 2011 Karl Berry.
Copyright 1997, 2002, 2005 Olaf Weber.

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 2.1 of the License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this library; if not, see <http://www.gnu.org/licenses/>.  */
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut nelem: size_t, mut elsize: size_t) -> *mut libc::c_void {
    let mut new_mem: *mut libc::c_void = calloc(
        if nelem != 0 { nelem } else { 1i32 as u64 },
        if elsize != 0 { elsize } else { 1i32 as u64 },
    );
    if new_mem.is_null() {
        _tt_abort(
            b"xcalloc request for %lu elements of size %lu failed\x00" as *const u8 as *const i8,
            nelem,
            elsize,
        );
    }
    return new_mem;
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut new_mem: *mut libc::c_void = malloc(if size != 0 { size } else { 1i32 as u64 });
    if new_mem.is_null() {
        _tt_abort(
            b"xmalloc request for %lu bytes failed\x00" as *const u8 as *const i8,
            size,
        );
    }
    return new_mem;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut old_ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
    if old_ptr.is_null() {
        new_mem = xmalloc(size)
    } else {
        new_mem = realloc(old_ptr, if size != 0 { size } else { 1i32 as u64 });
        if new_mem.is_null() {
            _tt_abort(
                b"xrealloc() to %lu bytes failed\x00" as *const u8 as *const i8,
                size,
            );
        }
    }
    return new_mem;
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut s: *const i8) -> *mut i8 {
    let mut new_string: *mut i8 = xmalloc(strlen(s).wrapping_add(1i32 as u64)) as *mut i8;
    return strcpy(new_string, s);
}

mod bibtex;
mod dpx_agl;
mod dpx_bmpimage;
mod dpx_cff;
mod dpx_cff_dict;
mod dpx_cid;
mod dpx_cidtype0;
mod dpx_cidtype2;
mod dpx_cmap;
mod dpx_cmap_read;
mod dpx_cmap_write;
mod dpx_cs_type2;
mod dpx_dpxconf;
mod dpx_dpxcrypt;
mod dpx_dpxfile;
mod dpx_dpxutil;
mod dpx_dvi;
mod dpx_dvipdfmx;
mod dpx_epdf;
mod dpx_error;
mod dpx_fontmap;
mod dpx_jp2image;
mod dpx_jpegimage;
mod dpx_mem;
mod dpx_mfileio;
mod dpx_mpost;
mod dpx_numbers;
mod dpx_otl_conf;
mod dpx_otl_opt;
mod dpx_pdfcolor;
mod dpx_pdfdev;
mod dpx_pdfdoc;
mod dpx_pdfdraw;
mod dpx_pdfencoding;
mod dpx_pdfencrypt;
mod dpx_pdffont;
mod dpx_pdfnames;
mod dpx_pdfobj;
mod dpx_pdfparse;
mod dpx_pdfresource;
mod dpx_pdfximage;
mod dpx_pkfont;
mod dpx_pngimage;
mod dpx_pst;
mod dpx_pst_obj;
mod dpx_sfnt;
mod dpx_spc_color;
mod dpx_spc_dvipdfmx;
mod dpx_spc_dvips;
mod dpx_spc_html;
mod dpx_spc_misc;
mod dpx_spc_pdfm;
mod dpx_spc_tpic;
mod dpx_spc_util;
mod dpx_spc_xtx;
mod dpx_specials;
mod dpx_subfont;
mod dpx_t1_char;
mod dpx_t1_load;
mod dpx_tfm;
mod dpx_truetype;
mod dpx_tt_aux;
mod dpx_tt_cmap;
mod dpx_tt_glyf;
mod dpx_tt_gsub;
mod dpx_tt_post;
mod dpx_tt_table;
mod dpx_type0;
mod dpx_type1;
mod dpx_type1c;
mod dpx_unicode;
mod dpx_vf;
mod xetex_engine_interface;
mod xetex_errors;
mod xetex_ext;
mod xetex_ini;
mod xetex_io;
mod xetex_linebreak;
mod xetex_math;
mod xetex_output;
mod xetex_pagebuilder;
mod xetex_pic;
mod xetex_scaledmath;
mod xetex_shipout;
mod xetex_stringpool;
mod xetex_synctex;
mod xetex_texmfmp;
mod xetex_xetex0;

mod stub_icu;

pub use xetex_engine_interface::tt_xetex_set_int_variable;
