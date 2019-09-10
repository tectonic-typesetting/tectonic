#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type XeTeXLayoutEngine_rec;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut eqtb: *mut memory_word;
    #[no_mangle]
    static mut file_line_error_style_p: libc::c_int;
    #[no_mangle]
    static mut insert_src_special_every_math: bool;
    #[no_mangle]
    static mut help_line: [*const libc::c_char; 6];
    #[no_mangle]
    static mut help_ptr: u8;
    #[no_mangle]
    static mut tex_remainder: scaled_t;
    #[no_mangle]
    static mut temp_ptr: int32_t;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut hi_mem_min: int32_t;
    #[no_mangle]
    static mut avail: int32_t;
    #[no_mangle]
    static mut nest_ptr: int32_t;
    #[no_mangle]
    static mut cur_list: list_state_record;
    #[no_mangle]
    static mut save_stack: *mut memory_word;
    #[no_mangle]
    static mut save_ptr: int32_t;
    #[no_mangle]
    static mut cur_group: group_code;
    #[no_mangle]
    static mut cur_cmd: eight_bits;
    #[no_mangle]
    static mut cur_chr: int32_t;
    #[no_mangle]
    static mut cur_val: int32_t;
    #[no_mangle]
    static mut cur_val1: int32_t;
    #[no_mangle]
    static mut font_info: *mut memory_word;
    #[no_mangle]
    static mut font_params: *mut font_index;
    #[no_mangle]
    static mut font_area: *mut str_number;
    #[no_mangle]
    static mut font_bc: *mut UTF16_code;
    #[no_mangle]
    static mut font_ec: *mut UTF16_code;
    #[no_mangle]
    static mut skew_char: *mut int32_t;
    #[no_mangle]
    static mut font_layout_engine: *mut *mut libc::c_void;
    #[no_mangle]
    static mut char_base: *mut int32_t;
    #[no_mangle]
    static mut width_base: *mut int32_t;
    #[no_mangle]
    static mut height_base: *mut int32_t;
    #[no_mangle]
    static mut depth_base: *mut int32_t;
    #[no_mangle]
    static mut italic_base: *mut int32_t;
    #[no_mangle]
    static mut lig_kern_base: *mut int32_t;
    #[no_mangle]
    static mut kern_base: *mut int32_t;
    #[no_mangle]
    static mut exten_base: *mut int32_t;
    #[no_mangle]
    static mut param_base: *mut int32_t;
    #[no_mangle]
    static mut null_character: b16x4;
    #[no_mangle]
    static mut total_shrink: [scaled_t; 4];
    #[no_mangle]
    static mut adjust_tail: int32_t;
    #[no_mangle]
    static mut pre_adjust_tail: int32_t;
    #[no_mangle]
    static mut empty: b32x2;
    #[no_mangle]
    static mut cur_f: internal_font_number;
    #[no_mangle]
    static mut cur_c: int32_t;
    #[no_mangle]
    static mut cur_i: b16x4;
    #[no_mangle]
    fn usingOpenType(engine: XeTeXLayoutEngine) -> bool;
    #[no_mangle]
    fn isOpenTypeMathFont(engine: XeTeXLayoutEngine) -> bool;
    #[no_mangle]
    fn measure_native_glyph(node: *mut libc::c_void, use_glyph_metrics: libc::c_int);
    #[no_mangle]
    fn map_char_to_glyph(font: int32_t, ch: int32_t) -> int32_t;
    #[no_mangle]
    fn real_get_native_glyph(pNode: *mut libc::c_void, index: libc::c_uint) -> u16;
    #[no_mangle]
    fn get_native_mathsy_param(f: libc::c_int, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_native_mathex_param(f: libc::c_int, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_ot_math_constant(f: libc::c_int, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_ot_math_variant(
        f: libc::c_int,
        g: libc::c_int,
        v: libc::c_int,
        adv: *mut int32_t,
        horiz: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn get_ot_assembly_ptr(f: libc::c_int, g: libc::c_int, horiz: libc::c_int)
        -> *mut libc::c_void;
    #[no_mangle]
    fn free_ot_assembly(a: *mut GlyphAssembly);
    #[no_mangle]
    fn get_ot_math_ital_corr(f: libc::c_int, g: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_ot_math_accent_pos(f: libc::c_int, g: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn get_ot_math_kern(
        f: libc::c_int,
        g: libc::c_int,
        sf: libc::c_int,
        sg: libc::c_int,
        cmd: libc::c_int,
        shift: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ot_part_count(a: *const GlyphAssembly) -> libc::c_int;
    #[no_mangle]
    fn ot_part_glyph(a: *const GlyphAssembly, i: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ot_part_is_extender(a: *const GlyphAssembly, i: libc::c_int) -> bool;
    #[no_mangle]
    fn ot_part_start_connector(
        f: libc::c_int,
        a: *const GlyphAssembly,
        i: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ot_part_end_connector(
        f: libc::c_int,
        a: *const GlyphAssembly,
        i: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ot_part_full_advance(f: libc::c_int, a: *const GlyphAssembly, i: libc::c_int)
        -> libc::c_int;
    #[no_mangle]
    fn ot_min_connector_overlap(f: libc::c_int) -> libc::c_int;
    /*:1683*/
    /* It looks like these arrays are set up so that they can be safely indexed
     * with negative indices. The underlying arrays used to be named "zzzaa" and
     * "zzzbb". */
    /* the former xetexcoerce.h: */
    #[no_mangle]
    fn just_reverse(p: int32_t);
    #[no_mangle]
    fn get_token();
    #[no_mangle]
    fn get_x_token();
    #[no_mangle]
    fn scan_left_brace();
    #[no_mangle]
    fn scan_keyword(s: *const libc::c_char) -> bool;
    #[no_mangle]
    fn scan_usv_num();
    #[no_mangle]
    fn scan_math_class_int();
    #[no_mangle]
    fn scan_math_fam_int();
    #[no_mangle]
    fn scan_fifteen_bit_int();
    #[no_mangle]
    fn scan_delimiter_int();
    #[no_mangle]
    fn effective_char(err_p: bool, f: internal_font_number, c: u16) -> int32_t;
    #[no_mangle]
    fn scan_dimen(mu: bool, inf: bool, shortcut: bool);
    #[no_mangle]
    fn char_warning(f: internal_font_number, c: int32_t);
    #[no_mangle]
    fn new_native_character(f: internal_font_number, c: UnicodeScalar) -> int32_t;
    #[no_mangle]
    fn new_character(f: internal_font_number, c: UTF16_code) -> int32_t;
    #[no_mangle]
    fn hpack(p: int32_t, w: scaled_t, m: small_number) -> int32_t;
    #[no_mangle]
    fn vpackage(p: int32_t, h: scaled_t, m: small_number, l: scaled_t) -> int32_t;
    #[no_mangle]
    fn append_to_vlist(b: int32_t);
    #[no_mangle]
    fn new_noad() -> int32_t;
    #[no_mangle]
    fn new_choice() -> int32_t;
    #[no_mangle]
    fn line_break(d: bool);
    #[no_mangle]
    fn off_save();
    #[no_mangle]
    fn norm_min(h: int32_t) -> small_number;
    #[no_mangle]
    fn push_math(c: group_code);
    #[no_mangle]
    fn just_copy(p: int32_t, h: int32_t, t: int32_t);
    #[no_mangle]
    fn back_error();
    #[no_mangle]
    fn back_input();
    #[no_mangle]
    fn begin_token_list(p: int32_t, t: u16);
    #[no_mangle]
    fn unsave();
    #[no_mangle]
    fn eq_word_define(p: int32_t, w: int32_t);
    #[no_mangle]
    static mut just_box: int32_t;
    #[no_mangle]
    static mut cur_lang: u8;
    #[no_mangle]
    fn pop_nest();
    #[no_mangle]
    fn push_nest();
    #[no_mangle]
    fn copy_node_list(p: int32_t) -> int32_t;
    #[no_mangle]
    fn flush_node_list(p: int32_t);
    #[no_mangle]
    fn delete_glue_ref(p: int32_t);
    #[no_mangle]
    fn new_penalty(m: int32_t) -> int32_t;
    #[no_mangle]
    fn new_kern(w: scaled_t) -> int32_t;
    #[no_mangle]
    fn new_skip_param(n: small_number) -> int32_t;
    #[no_mangle]
    fn new_glue(q: int32_t) -> int32_t;
    #[no_mangle]
    fn new_param_glue(n: small_number) -> int32_t;
    #[no_mangle]
    fn new_spec(p: int32_t) -> int32_t;
    #[no_mangle]
    fn new_math(w: scaled_t, s: small_number) -> int32_t;
    #[no_mangle]
    fn new_rule() -> int32_t;
    #[no_mangle]
    fn new_null_box() -> int32_t;
    #[no_mangle]
    fn free_node(p: int32_t, s: int32_t);
    #[no_mangle]
    fn get_node(s: int32_t) -> int32_t;
    #[no_mangle]
    fn get_avail() -> int32_t;
    #[no_mangle]
    static mut xtx_ligature_present: bool;
    #[no_mangle]
    static mut cur_dir: small_number;
    #[no_mangle]
    static mut LR_problems: int32_t;
    #[no_mangle]
    static mut LR_ptr: int32_t;
    /* xetex-errors */
    #[no_mangle]
    fn confusion(s: *const libc::c_char) -> !;
    #[no_mangle]
    fn scan_math(p: int32_t);
    #[no_mangle]
    fn insert_src_special();
    #[no_mangle]
    fn error();
    #[no_mangle]
    fn print_esc_cstr(s: *const libc::c_char);
    #[no_mangle]
    fn print_size(s: int32_t);
    #[no_mangle]
    fn print_int(n: int32_t);
    #[no_mangle]
    fn print_cstr(s: *const libc::c_char);
    #[no_mangle]
    fn print(s: int32_t);
    #[no_mangle]
    fn print_char(s: int32_t);
    #[no_mangle]
    fn print_nl_cstr(s: *const libc::c_char);
    /* xetex-pagebuilder */
    /* xetex-scaledmath */
    #[no_mangle]
    fn x_over_n(x: scaled_t, n: int32_t) -> scaled_t;
    #[no_mangle]
    fn mult_and_add(n: int32_t, x: scaled_t, y: scaled_t, max_answer: scaled_t) -> scaled_t;
    #[no_mangle]
    fn print_file_line();
    #[no_mangle]
    fn half(x: int32_t) -> int32_t;
    #[no_mangle]
    fn tex_round(_: libc::c_double) -> int32_t;
    #[no_mangle]
    fn build_page();
    #[no_mangle]
    fn xn_over_d(x: scaled_t, n: int32_t, d: int32_t) -> scaled_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type hb_codepoint_t = u32;
pub type hb_position_t = int32_t;
pub type hb_ot_math_glyph_part_flags_t = libc::c_uint;
pub const HB_OT_MATH_GLYPH_PART_FLAG_EXTENDER: hb_ot_math_glyph_part_flags_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hb_ot_math_glyph_part_t {
    pub glyph: hb_codepoint_t,
    pub start_connector_length: hb_position_t,
    pub end_connector_length: hb_position_t,
    pub full_advance: hb_position_t,
    pub flags: hb_ot_math_glyph_part_flags_t,
}
pub type scaled_t = int32_t;
pub type XeTeXLayoutEngine = *mut XeTeXLayoutEngine_rec;
/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphAssembly {
    pub count: libc::c_uint,
    pub parts: *mut hb_ot_math_glyph_part_t,
}
pub type UTF16_code = libc::c_ushort;
pub type UnicodeScalar = int32_t;
pub type eight_bits = u8;
pub type str_number = int32_t;
pub type small_number = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: int32_t,
    pub s1: int32_t,
}
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
    pub gr: libc::c_double,
    pub ptr: *mut libc::c_void,
}
pub type group_code = u8;
pub type internal_font_number = int32_t;
pub type font_index = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_state_record {
    pub mode: libc::c_short,
    pub head: int32_t,
    pub tail: int32_t,
    pub eTeX_aux: int32_t,
    pub prev_graf: int32_t,
    pub mode_line: int32_t,
    pub aux: memory_word,
}
/* Inlines */
#[inline]
unsafe extern "C" fn is_char_node(p: int32_t) -> bool {
    return p >= hi_mem_min;
}
static mut null_delimiter: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
static mut cur_mlist: int32_t = 0;
static mut cur_style: small_number = 0;
static mut cur_size: int32_t = 0;
static mut cur_mu: scaled_t = 0;
static mut mlist_penalties: bool = false;
#[no_mangle]
pub unsafe extern "C" fn initialize_math_variables() {
    null_delimiter.s3 = 0i32 as u16;
    null_delimiter.s2 = 0i32 as u16;
    null_delimiter.s1 = 0i32 as u16;
    null_delimiter.s0 = 0i32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_math() {
    let mut current_block: u64;
    let mut w: scaled_t = 0;
    let mut j: int32_t = 0;
    let mut x: int32_t = 0;
    let mut l: scaled_t = 0;
    let mut s: scaled_t = 0;
    let mut p: int32_t = 0;
    let mut q: int32_t = 0;
    let mut f: internal_font_number = 0;
    let mut n: int32_t = 0;
    let mut v: scaled_t = 0;
    let mut d: scaled_t = 0;
    get_token();
    if cur_cmd as libc::c_int == 3i32 && cur_list.mode as libc::c_int > 0i32 {
        /*1180: */
        j = -0xfffffffi32;
        w = -0x3fffffffi32;
        if cur_list.head == cur_list.tail {
            /*1520: */
            pop_nest();
            if cur_list.eTeX_aux == -0xfffffffi32 {
                x = 0i32
            } else if (*mem.offset(cur_list.eTeX_aux as isize)).b32.s0 >= 8i32 {
                x = -1i32
            } else {
                x = 1i32
            }
        /*:1519 */
        } else {
            line_break(1i32 != 0);
            /*1528: */
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
                    + 8i32) as isize,
            ))
            .b32
            .s1 == 0i32
            {
                j = new_kern(0i32)
            } else {
                j = new_param_glue(8i32 as small_number)
            } /*:1519 */
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
                    + 7i32) as isize,
            ))
            .b32
            .s1 == 0i32
            {
                p = new_kern(0i32)
            } else {
                p = new_param_glue(7i32 as small_number)
            }
            (*mem.offset(p as isize)).b32.s1 = j;
            j = new_null_box();
            (*mem.offset((j + 1i32) as isize)).b32.s1 =
                (*mem.offset((just_box + 1i32) as isize)).b32.s1;
            (*mem.offset((j + 4i32) as isize)).b32.s1 =
                (*mem.offset((just_box + 4i32) as isize)).b32.s1;
            (*mem.offset((j + 5i32) as isize)).b32.s1 = p;
            (*mem.offset((j + 5i32) as isize)).b16.s0 =
                (*mem.offset((just_box + 5i32) as isize)).b16.s0;
            (*mem.offset((j + 5i32) as isize)).b16.s1 =
                (*mem.offset((just_box + 5i32) as isize)).b16.s1;
            (*mem.offset((j + 6i32) as isize)).gr = (*mem.offset((just_box + 6i32) as isize)).gr;
            v = (*mem.offset((just_box + 4i32) as isize)).b32.s1;
            if cur_list.eTeX_aux == -0xfffffffi32 {
                x = 0i32
            } else if (*mem.offset(cur_list.eTeX_aux as isize)).b32.s0 >= 8i32 {
                x = -1i32
            } else {
                x = 1i32
            }
            if x >= 0i32 {
                p = (*mem.offset((just_box + 5i32) as isize)).b32.s1;
                (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 = -0xfffffffi32
            } else {
                v = -v - (*mem.offset((just_box + 1i32) as isize)).b32.s1;
                p = new_math(0i32, 6i32 as small_number);
                (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 = p;
                just_copy(
                    (*mem.offset((just_box + 5i32) as isize)).b32.s1,
                    p,
                    new_math(0i32, 7i32 as small_number),
                );
                cur_dir = 1i32 as small_number
            }
            v = v + 2i32
                * (*font_info.offset(
                    (6i32
                        + *param_base.offset(
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
                            .s1 as isize,
                        )) as isize,
                ))
                .b32
                .s1;
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
                    + 71i32) as isize,
            ))
            .b32
            .s1 > 0i32
            {
                /*1497: */
                temp_ptr = get_avail(); /*1523:*/
                (*mem.offset(temp_ptr as isize)).b32.s0 = 0i32; /*:1398 */
                (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                LR_ptr = temp_ptr
            }
            while p != -0xfffffffi32 {
                loop {
                    if is_char_node(p) {
                        f = (*mem.offset(p as isize)).b16.s1 as internal_font_number;
                        d = (*font_info.offset(
                            (*width_base.offset(f as isize)
                                + (*font_info.offset(
                                    (*char_base.offset(f as isize)
                                        + effective_char(
                                            1i32 != 0,
                                            f,
                                            (*mem.offset(p as isize)).b16.s0,
                                        )) as isize,
                                ))
                                .b16
                                .s3 as libc::c_int) as isize,
                        ))
                        .b32
                        .s1;
                        current_block = 9427725525305667067;
                        break;
                    } else {
                        match (*mem.offset(p as isize)).b16.s1 as libc::c_int {
                            0 | 1 | 2 => {
                                d = (*mem.offset((p + 1i32) as isize)).b32.s1;
                                current_block = 9427725525305667067;
                                break;
                            }
                            6 => {
                                *mem.offset((4999999i32 - 12i32) as isize) =
                                    *mem.offset((p + 1i32) as isize);
                                (*mem.offset((4999999i32 - 12i32) as isize)).b32.s1 =
                                    (*mem.offset(p as isize)).b32.s1;
                                p = 4999999i32 - 12i32;
                                xtx_ligature_present = 1i32 != 0
                            }
                            11 => {
                                d = (*mem.offset((p + 1i32) as isize)).b32.s1;
                                current_block = 1677945370889843322;
                                break;
                            }
                            40 => {
                                d = (*mem.offset((p + 1i32) as isize)).b32.s1;
                                current_block = 1677945370889843322;
                                break;
                            }
                            9 => {
                                d = (*mem.offset((p + 1i32) as isize)).b32.s1;
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
                                        + 71i32) as isize,
                                ))
                                .b32
                                .s1 > 0i32
                                {
                                    current_block = 13660591889533726445;
                                    break;
                                } else {
                                    current_block = 2631791190359682872;
                                    break;
                                }
                            }
                            14 => {
                                d = (*mem.offset((p + 1i32) as isize)).b32.s1;
                                cur_dir = (*mem.offset(p as isize)).b16.s0 as small_number;
                                current_block = 1677945370889843322;
                                break;
                            }
                            10 => {
                                q = (*mem.offset((p + 1i32) as isize)).b32.s0;
                                d = (*mem.offset((q + 1i32) as isize)).b32.s1;
                                if (*mem.offset((just_box + 5i32) as isize)).b16.s1 as libc::c_int
                                    == 1i32
                                {
                                    if (*mem.offset((just_box + 5i32) as isize)).b16.s0
                                        as libc::c_int
                                        == (*mem.offset(q as isize)).b16.s1 as libc::c_int
                                        && (*mem.offset((q + 2i32) as isize)).b32.s1 != 0i32
                                    {
                                        v = 0x3fffffffi32
                                    }
                                } else if (*mem.offset((just_box + 5i32) as isize)).b16.s1
                                    as libc::c_int
                                    == 2i32
                                {
                                    if (*mem.offset((just_box + 5i32) as isize)).b16.s0
                                        as libc::c_int
                                        == (*mem.offset(q as isize)).b16.s0 as libc::c_int
                                        && (*mem.offset((q + 3i32) as isize)).b32.s1 != 0i32
                                    {
                                        v = 0x3fffffffi32
                                    }
                                }
                                if (*mem.offset(p as isize)).b16.s0 as libc::c_int >= 100i32 {
                                    current_block = 9427725525305667067;
                                    break;
                                } else {
                                    current_block = 1677945370889843322;
                                    break;
                                }
                            }
                            8 => {
                                if (*mem.offset(p as isize)).b16.s0 as libc::c_int == 40i32
                                    || (*mem.offset(p as isize)).b16.s0 as libc::c_int == 41i32
                                    || (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
                                    || (*mem.offset(p as isize)).b16.s0 as libc::c_int == 43i32
                                    || (*mem.offset(p as isize)).b16.s0 as libc::c_int == 44i32
                                {
                                    current_block = 11064061988481400464;
                                    break;
                                } else {
                                    current_block = 5846959088466685742;
                                    break;
                                }
                            }
                            _ => {
                                d = 0i32;
                                current_block = 1677945370889843322;
                                break;
                            }
                        }
                    }
                }
                match current_block {
                    2631791190359682872 => {
                        if (*mem.offset(p as isize)).b16.s0 as libc::c_int >= 4i32 {
                            w = 0x3fffffffi32;
                            break;
                        } else {
                            current_block = 1677945370889843322;
                        }
                    }
                    13660591889533726445 =>
                    /*1525: */
                    {
                        if (*mem.offset(p as isize)).b16.s0 as libc::c_int & 1i32 != 0 {
                            if (*mem.offset(LR_ptr as isize)).b32.s0
                                == 4i32 * ((*mem.offset(p as isize)).b16.s0 as libc::c_int / 4i32)
                                    + 3i32
                            {
                                temp_ptr = LR_ptr;
                                LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                                (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                                avail = temp_ptr
                            } else if (*mem.offset(p as isize)).b16.s0 as libc::c_int > 4i32 {
                                w = 0x3fffffffi32;
                                break;
                            }
                        } else {
                            temp_ptr = get_avail();
                            (*mem.offset(temp_ptr as isize)).b32.s0 = 4i32
                                * ((*mem.offset(p as isize)).b16.s0 as libc::c_int / 4i32)
                                + 3i32;
                            (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                            LR_ptr = temp_ptr;
                            if (*mem.offset(p as isize)).b16.s0 as libc::c_int / 8i32
                                != cur_dir as libc::c_int
                            {
                                just_reverse(p);
                                p = 4999999i32 - 3i32
                            }
                        }
                        current_block = 1677945370889843322;
                    }
                    5846959088466685742 => {
                        d = 0i32;
                        current_block = 1677945370889843322;
                    }
                    11064061988481400464 => {
                        d = (*mem.offset((p + 1i32) as isize)).b32.s1;
                        current_block = 9427725525305667067;
                    }
                    _ => {}
                }
                match current_block {
                    1677945370889843322 => {
                        if v < 0x3fffffffi32 {
                            v = v + d
                        }
                    }
                    _ => {
                        if v < 0x3fffffffi32 {
                            v = v + d;
                            w = v
                        } else {
                            w = 0x3fffffffi32;
                            break;
                        }
                    }
                }
                p = (*mem.offset(p as isize)).b32.s1
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
                    + 71i32) as isize,
            ))
            .b32
            .s1 > 0i32
            {
                while LR_ptr != -0xfffffffi32 {
                    temp_ptr = LR_ptr;
                    LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                    (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                    avail = temp_ptr
                }
                if LR_problems != 0i32 {
                    w = 0x3fffffffi32;
                    LR_problems = 0i32
                }
            }
            cur_dir = 0i32 as small_number;
            flush_node_list((*mem.offset((4999999i32 - 3i32) as isize)).b32.s1);
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
                + 0i32) as isize,
        ))
        .b32
        .s1 == -0xfffffffi32
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
                    + 85i32
                    + 256i32
                    + (0x10ffffi32 + 1i32)
                    + 17i32) as isize,
            ))
            .b32
            .s1 != 0i32
                && ((*eqtb.offset(
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
                .s1 >= 0i32
                    && cur_list.prev_graf + 2i32
                        > (*eqtb.offset(
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
                        .s1
                    || cur_list.prev_graf + 1i32
                        < -(*eqtb.offset(
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
                        .s1)
            {
                l = (*eqtb.offset(
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
                        + 3i32) as isize,
                ))
                .b32
                .s1 - abs((*eqtb.offset(
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
                        + 17i32) as isize,
                ))
                .b32
                .s1);
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
                        + 85i32
                        + 256i32
                        + (0x10ffffi32 + 1i32)
                        + 17i32) as isize,
                ))
                .b32
                .s1 > 0i32
                {
                    s = (*eqtb.offset(
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
                            + 17i32) as isize,
                    ))
                    .b32
                    .s1
                } else {
                    s = 0i32
                }
            } else {
                l = (*eqtb.offset(
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
                        + 3i32) as isize,
                ))
                .b32
                .s1;
                s = 0i32
            }
        } else {
            n = (*mem.offset(
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
                .s1 as isize,
            ))
            .b32
            .s0;
            if cur_list.prev_graf + 2i32 >= n {
                p = (*eqtb.offset(
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
                .s1 + 2i32 * n
            } else {
                p = (*eqtb.offset(
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
                .s1 + 2i32 * (cur_list.prev_graf + 2i32)
            }
            s = (*mem.offset((p - 1i32) as isize)).b32.s1;
            l = (*mem.offset(p as isize)).b32.s1
        }
        push_math(15i32 as group_code);
        cur_list.mode = 207i32 as libc::c_short;
        eq_word_define(
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
            -1i32,
        );
        eq_word_define(
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
            w,
        );
        cur_list.eTeX_aux = j;
        eq_word_define(
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
            x,
        );
        eq_word_define(
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
            l,
        );
        eq_word_define(
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
            s,
        );
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
                + 4i32) as isize,
        ))
        .b32
        .s1 != -0xfffffffi32
        {
            begin_token_list(
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
                        + 4i32) as isize,
                ))
                .b32
                .s1,
                10i32 as u16,
            );
        }
        if nest_ptr == 1i32 {
            build_page();
        }
    } else {
        back_input();
        push_math(15i32 as group_code);
        eq_word_define(
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
            -1i32,
        );
        if insert_src_special_every_math {
            insert_src_special();
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
                + 3i32) as isize,
        ))
        .b32
        .s1 != -0xfffffffi32
        {
            begin_token_list(
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
                        + 3i32) as isize,
                ))
                .b32
                .s1,
                9i32 as u16,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn start_eq_no() {
    (*save_stack.offset((save_ptr + 0i32) as isize)).b32.s1 = cur_chr;
    save_ptr += 1;
    push_math(15i32 as group_code);
    eq_word_define(
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
        -1i32,
    );
    if insert_src_special_every_math {
        insert_src_special();
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
            + 3i32) as isize,
    ))
    .b32
    .s1 != -0xfffffffi32
    {
        begin_token_list(
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
                    + 3i32) as isize,
            ))
            .b32
            .s1,
            9i32 as u16,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn math_limit_switch() {
    if cur_list.head != cur_list.tail {
        if (*mem.offset(cur_list.tail as isize)).b16.s1 as libc::c_int == 17i32 {
            (*mem.offset(cur_list.tail as isize)).b16.s0 = cur_chr as u16;
            return;
        }
    }
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
    }
    print_cstr(
        b"Limit controls must follow a math operator\x00" as *const u8 as *const libc::c_char,
    );
    help_ptr = 1i32 as u8;
    help_line[0] = b"I\'m ignoring this misplaced \\limits or \\nolimits command.\x00" as *const u8
        as *const libc::c_char;
    error();
}
unsafe extern "C" fn scan_delimiter(mut p: int32_t, mut r: bool) {
    if r {
        if cur_chr == 1i32 {
            cur_val1 = 0x40000000i32;
            scan_math_fam_int();
            cur_val1 += cur_val * 0x200000i32;
            scan_usv_num();
            cur_val += cur_val1
        } else {
            scan_delimiter_int();
        }
    } else {
        loop {
            get_x_token();
            if !(cur_cmd as libc::c_int == 10i32 || cur_cmd as libc::c_int == 0i32) {
                break;
            }
        }
        match cur_cmd as libc::c_int {
            11 | 12 => {
                cur_val = (*eqtb.offset(
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
                        + cur_chr) as isize,
                ))
                .b32
                .s1
            }
            15 => {
                if cur_chr == 1i32 {
                    cur_val1 = 0x40000000i32;
                    scan_math_class_int();
                    scan_math_fam_int();
                    cur_val1 += cur_val * 0x20000i32;
                    scan_usv_num();
                    cur_val += cur_val1
                } else {
                    scan_delimiter_int();
                }
            }
            _ => cur_val = -1i32,
        }
    }
    if cur_val < 0i32 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(b"Missing delimiter (. inserted)\x00" as *const u8 as *const libc::c_char);
        help_ptr = 6i32 as u8;
        help_line[5] = b"I was expecting to see something like `(\' or `\\{\' or\x00" as *const u8
            as *const libc::c_char;
        help_line[4] = b"`\\}\' here. If you typed, e.g., `{\' instead of `\\{\', you\x00"
            as *const u8 as *const libc::c_char;
        help_line[3] = b"should probably delete the `{\' by typing `1\' now, so that\x00"
            as *const u8 as *const libc::c_char;
        help_line[2] = b"braces don\'t get unbalanced. Otherwise just proceed.\x00" as *const u8
            as *const libc::c_char;
        help_line[1] = b"Acceptable delimiters are characters whose \\delcode is\x00" as *const u8
            as *const libc::c_char;
        help_line[0] = b"nonnegative, or you can use `\\delimiter <delimiter code>\'.\x00"
            as *const u8 as *const libc::c_char;
        back_error();
        cur_val = 0i32
    }
    if cur_val >= 0x40000000i32 {
        (*mem.offset(p as isize)).b16.s3 = (cur_val % 0x200000i32 / 0x10000i32 * 0x100i32
            + cur_val / 0x200000i32 % 0x100i32)
            as u16;
        (*mem.offset(p as isize)).b16.s2 = (cur_val % 0x10000i32) as u16;
        (*mem.offset(p as isize)).b16.s1 = 0i32 as u16;
        (*mem.offset(p as isize)).b16.s0 = 0i32 as u16
    } else {
        (*mem.offset(p as isize)).b16.s3 = (cur_val / 0x100000i32 % 16i32) as u16;
        (*mem.offset(p as isize)).b16.s2 = (cur_val / 0x1000i32 % 0x100i32) as u16;
        (*mem.offset(p as isize)).b16.s1 = (cur_val / 0x100i32 % 16i32) as u16;
        (*mem.offset(p as isize)).b16.s0 = (cur_val % 0x100i32) as u16
    };
}
#[no_mangle]
pub unsafe extern "C" fn math_radical() {
    (*mem.offset(cur_list.tail as isize)).b32.s1 = get_node(5i32);
    cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
    (*mem.offset(cur_list.tail as isize)).b16.s1 = 24i32 as u16;
    (*mem.offset(cur_list.tail as isize)).b16.s0 = 0i32 as u16;
    (*mem.offset((cur_list.tail + 1i32) as isize)).b32 = empty;
    (*mem.offset((cur_list.tail + 3i32) as isize)).b32 = empty;
    (*mem.offset((cur_list.tail + 2i32) as isize)).b32 = empty;
    scan_delimiter(cur_list.tail + 4i32, 1i32 != 0);
    scan_math(cur_list.tail + 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn math_ac() {
    let mut c: int32_t = 0;
    if cur_cmd as libc::c_int == 45i32 {
        /*1201: */
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(b"Please use \x00" as *const u8 as *const libc::c_char);
        print_esc_cstr(b"mathaccent\x00" as *const u8 as *const libc::c_char);
        print_cstr(b" for accents in math mode\x00" as *const u8 as *const libc::c_char);
        help_ptr = 2i32 as u8;
        help_line[1] = b"I\'m changing \\accent to \\mathaccent here; wish me luck.\x00"
            as *const u8 as *const libc::c_char;
        help_line[0] = b"(Accents are not the same in formulas as they are in text.)\x00"
            as *const u8 as *const libc::c_char;
        error();
    }
    (*mem.offset(cur_list.tail as isize)).b32.s1 = get_node(5i32);
    cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
    (*mem.offset(cur_list.tail as isize)).b16.s1 = 28i32 as u16;
    (*mem.offset(cur_list.tail as isize)).b16.s0 = 0i32 as u16;
    (*mem.offset((cur_list.tail + 1i32) as isize)).b32 = empty;
    (*mem.offset((cur_list.tail + 3i32) as isize)).b32 = empty;
    (*mem.offset((cur_list.tail + 2i32) as isize)).b32 = empty;
    (*mem.offset((cur_list.tail + 4i32) as isize)).b32.s1 = 1i32;
    if cur_chr == 1i32 {
        if scan_keyword(b"fixed\x00" as *const u8 as *const libc::c_char) {
            (*mem.offset(cur_list.tail as isize)).b16.s0 = 1i32 as u16
        } else if scan_keyword(b"bottom\x00" as *const u8 as *const libc::c_char) {
            if scan_keyword(b"fixed\x00" as *const u8 as *const libc::c_char) {
                (*mem.offset(cur_list.tail as isize)).b16.s0 = (2i32 + 1i32) as u16
            } else {
                (*mem.offset(cur_list.tail as isize)).b16.s0 = 2i32 as u16
            }
        }
        scan_math_class_int();
        c = ((cur_val as libc::c_uint & 0x7i32 as libc::c_uint) << 21i32) as int32_t;
        scan_math_fam_int();
        c = (c as libc::c_uint)
            .wrapping_add((cur_val as libc::c_uint & 0xffi32 as libc::c_uint) << 24i32)
            as int32_t;
        scan_usv_num();
        cur_val = cur_val + c
    } else {
        scan_fifteen_bit_int();
        cur_val = (((cur_val / 4096i32) as libc::c_uint & 0x7i32 as libc::c_uint) << 21i32)
            .wrapping_add(
                ((cur_val % 4096i32 / 256i32) as libc::c_uint & 0xffi32 as libc::c_uint) << 24i32,
            )
            .wrapping_add((cur_val % 256i32) as libc::c_uint) as int32_t
    }
    (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s0 =
        (cur_val as libc::c_long % 65536) as u16;
    if cur_val as libc::c_uint >> 21i32 & 0x7i32 as libc::c_uint == 7i32 as libc::c_uint
        && ((*eqtb.offset(
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
                + 44i32) as isize,
        ))
        .b32
        .s1 >= 0i32
            && (*eqtb.offset(
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
                    + 44i32) as isize,
            ))
            .b32
            .s1 < 256i32)
    {
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 = (*eqtb.offset(
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
                + 44i32) as isize,
        ))
        .b32
        .s1 as u16
    } else {
        (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 =
            (cur_val as libc::c_uint >> 24i32 & 0xffi32 as libc::c_uint) as u16
    }
    (*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 =
        ((*mem.offset((cur_list.tail + 4i32) as isize)).b16.s1 as libc::c_long
            + (cur_val as libc::c_uint & 0x1fffffi32 as libc::c_uint) as libc::c_long / 65536
                * 256i32 as libc::c_long) as u16;
    scan_math(cur_list.tail + 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn append_choices() {
    (*mem.offset(cur_list.tail as isize)).b32.s1 = new_choice();
    cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
    save_ptr += 1;
    (*save_stack.offset((save_ptr - 1i32) as isize)).b32.s1 = 0i32;
    push_math(13i32 as group_code);
    scan_left_brace();
}
#[no_mangle]
pub unsafe extern "C" fn fin_mlist(mut p: int32_t) -> int32_t {
    let mut q: int32_t = 0;
    if cur_list.aux.b32.s1 != -0xfffffffi32 {
        /*1220: */
        (*mem.offset((cur_list.aux.b32.s1 + 3i32) as isize)).b32.s1 = 3i32;
        (*mem.offset((cur_list.aux.b32.s1 + 3i32) as isize)).b32.s0 =
            (*mem.offset(cur_list.head as isize)).b32.s1;
        if p == -0xfffffffi32 {
            q = cur_list.aux.b32.s1
        } else {
            q = (*mem.offset((cur_list.aux.b32.s1 + 2i32) as isize)).b32.s0;
            if (*mem.offset(q as isize)).b16.s1 as libc::c_int != 30i32
                || cur_list.eTeX_aux == -0xfffffffi32
            {
                confusion(b"right\x00" as *const u8 as *const libc::c_char);
            }
            (*mem.offset((cur_list.aux.b32.s1 + 2i32) as isize)).b32.s0 =
                (*mem.offset(cur_list.eTeX_aux as isize)).b32.s1;
            (*mem.offset(cur_list.eTeX_aux as isize)).b32.s1 = cur_list.aux.b32.s1;
            (*mem.offset(cur_list.aux.b32.s1 as isize)).b32.s1 = p
        }
    } else {
        (*mem.offset(cur_list.tail as isize)).b32.s1 = p;
        q = (*mem.offset(cur_list.head as isize)).b32.s1
    }
    pop_nest();
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn build_choices() {
    let mut p: int32_t = 0;
    unsave();
    p = fin_mlist(-0xfffffffi32);
    match (*save_stack.offset((save_ptr - 1i32) as isize)).b32.s1 {
        0 => (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s0 = p,
        1 => (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s1 = p,
        2 => (*mem.offset((cur_list.tail + 2i32) as isize)).b32.s0 = p,
        3 => {
            (*mem.offset((cur_list.tail + 2i32) as isize)).b32.s1 = p;
            save_ptr -= 1;
            return;
        }
        _ => {}
    }
    let ref mut fresh0 = (*save_stack.offset((save_ptr - 1i32) as isize)).b32.s1;
    *fresh0 += 1;
    push_math(13i32 as group_code);
    scan_left_brace();
}
#[no_mangle]
pub unsafe extern "C" fn sub_sup() {
    let mut t: small_number = 0;
    let mut p: int32_t = 0;
    t = 0i32 as small_number;
    p = -0xfffffffi32;
    if cur_list.tail != cur_list.head {
        if (*mem.offset(cur_list.tail as isize)).b16.s1 as libc::c_int >= 16i32
            && ((*mem.offset(cur_list.tail as isize)).b16.s1 as libc::c_int) < 30i32
        {
            p = cur_list.tail + 2i32 + cur_cmd as libc::c_int - 7i32;
            t = (*mem.offset(p as isize)).b32.s1 as small_number
        }
    }
    if p == -0xfffffffi32 || t as libc::c_int != 0i32 {
        /*1212: */
        (*mem.offset(cur_list.tail as isize)).b32.s1 = new_noad();
        cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
        p = cur_list.tail + 2i32 + cur_cmd as libc::c_int - 7i32;
        if t as libc::c_int != 0i32 {
            if cur_cmd as libc::c_int == 7i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
                }
                print_cstr(b"Double superscript\x00" as *const u8 as *const libc::c_char);
                help_ptr = 1i32 as u8;
                help_line[0] = b"I treat `x^1^2\' essentially like `x^1{}^2\'.\x00" as *const u8
                    as *const libc::c_char
            } else {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
                }
                print_cstr(b"Double subscript\x00" as *const u8 as *const libc::c_char);
                help_ptr = 1i32 as u8;
                help_line[0] = b"I treat `x_1_2\' essentially like `x_1{}_2\'.\x00" as *const u8
                    as *const libc::c_char
            }
            error();
        }
    }
    scan_math(p);
}
#[no_mangle]
pub unsafe extern "C" fn math_fraction() {
    let mut c: small_number = 0;
    c = cur_chr as small_number;
    if cur_list.aux.b32.s1 != -0xfffffffi32 {
        /*1218:*/
        if c as libc::c_int >= 3i32 {
            scan_delimiter(4999999i32 - 12i32, 0i32 != 0);
            scan_delimiter(4999999i32 - 12i32, 0i32 != 0);
        }
        if c as libc::c_int % 3i32 == 0i32 {
            scan_dimen(0i32 != 0, 0i32 != 0, 0i32 != 0);
        }
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(b"Ambiguous; you need another { and }\x00" as *const u8 as *const libc::c_char);
        help_ptr = 3i32 as u8;
        help_line[2] = b"I\'m ignoring this fraction specification, since I don\'t\x00" as *const u8
            as *const libc::c_char;
        help_line[1] = b"know whether a construction like `x \\over y \\over z\'\x00" as *const u8
            as *const libc::c_char;
        help_line[0] = b"means `{x \\over y} \\over z\' or `x \\over {y \\over z}\'.\x00"
            as *const u8 as *const libc::c_char;
        error();
    } else {
        cur_list.aux.b32.s1 = get_node(6i32);
        (*mem.offset(cur_list.aux.b32.s1 as isize)).b16.s1 = 25i32 as u16;
        (*mem.offset(cur_list.aux.b32.s1 as isize)).b16.s0 = 0i32 as u16;
        (*mem.offset((cur_list.aux.b32.s1 + 2i32) as isize)).b32.s1 = 3i32;
        (*mem.offset((cur_list.aux.b32.s1 + 2i32) as isize)).b32.s0 =
            (*mem.offset(cur_list.head as isize)).b32.s1;
        (*mem.offset((cur_list.aux.b32.s1 + 3i32) as isize)).b32 = empty;
        (*mem.offset((cur_list.aux.b32.s1 + 4i32) as isize)).b16 = null_delimiter;
        (*mem.offset((cur_list.aux.b32.s1 + 5i32) as isize)).b16 = null_delimiter;
        (*mem.offset(cur_list.head as isize)).b32.s1 = -0xfffffffi32;
        cur_list.tail = cur_list.head;
        if c as libc::c_int >= 3i32 {
            scan_delimiter(cur_list.aux.b32.s1 + 4i32, 0i32 != 0);
            scan_delimiter(cur_list.aux.b32.s1 + 5i32, 0i32 != 0);
        }
        match c as libc::c_int % 3i32 {
            0 => {
                scan_dimen(0i32 != 0, 0i32 != 0, 0i32 != 0);
                (*mem.offset((cur_list.aux.b32.s1 + 1i32) as isize)).b32.s1 = cur_val
            }
            1 => (*mem.offset((cur_list.aux.b32.s1 + 1i32) as isize)).b32.s1 = 0x40000000i32,
            2 => (*mem.offset((cur_list.aux.b32.s1 + 1i32) as isize)).b32.s1 = 0i32,
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn math_left_right() {
    let mut t: small_number = 0;
    let mut p: int32_t = 0;
    let mut q: int32_t = 0;
    t = cur_chr as small_number;
    if t as libc::c_int != 30i32 && cur_group as libc::c_int != 16i32 {
        /*1227: */
        if cur_group as libc::c_int == 15i32 {
            scan_delimiter(4999999i32 - 12i32, 0i32 != 0); /*:1530 */
            if file_line_error_style_p != 0 {
                print_file_line(); /*:1530 */
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
            }
            print_cstr(b"Extra \x00" as *const u8 as *const libc::c_char);
            if t as libc::c_int == 1i32 {
                print_esc_cstr(b"middle\x00" as *const u8 as *const libc::c_char);
                help_ptr = 1i32 as u8;
                help_line[0] = b"I\'m ignoring a \\middle that had no matching \\left.\x00"
                    as *const u8 as *const libc::c_char
            } else {
                print_esc_cstr(b"right\x00" as *const u8 as *const libc::c_char);
                help_ptr = 1i32 as u8;
                help_line[0] = b"I\'m ignoring a \\right that had no matching \\left.\x00"
                    as *const u8 as *const libc::c_char
            }
            error();
        } else {
            off_save();
        }
    } else {
        p = new_noad();
        (*mem.offset(p as isize)).b16.s1 = t as u16;
        scan_delimiter(p + 1i32, 0i32 != 0);
        if t as libc::c_int == 1i32 {
            (*mem.offset(p as isize)).b16.s1 = 31i32 as u16;
            (*mem.offset(p as isize)).b16.s0 = 1i32 as u16
        }
        if t as libc::c_int == 30i32 {
            q = p
        } else {
            q = fin_mlist(p);
            unsave();
        }
        if t as libc::c_int != 31i32 {
            push_math(16i32 as group_code);
            (*mem.offset(cur_list.head as isize)).b32.s1 = q;
            cur_list.tail = p;
            cur_list.eTeX_aux = p
        } else {
            (*mem.offset(cur_list.tail as isize)).b32.s1 = new_noad();
            cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
            (*mem.offset(cur_list.tail as isize)).b16.s1 = 23i32 as u16;
            (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s1 = 3i32;
            (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s0 = q
        }
    };
}
unsafe extern "C" fn app_display(mut j: int32_t, mut b: int32_t, mut d: scaled_t) {
    let mut z: scaled_t = 0;
    let mut s: scaled_t = 0;
    let mut e: scaled_t = 0;
    let mut x: int32_t = 0;
    let mut p: int32_t = 0;
    let mut q: int32_t = 0;
    let mut r: int32_t = 0;
    let mut t: int32_t = 0;
    let mut u: int32_t = 0;
    s = (*eqtb.offset(
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
            + 15i32) as isize,
    ))
    .b32
    .s1;
    x = (*eqtb.offset(
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
            + 63i32) as isize,
    ))
    .b32
    .s1;
    if x == 0i32 {
        (*mem.offset((b + 4i32) as isize)).b32.s1 = s + d
    } else {
        z = (*eqtb.offset(
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
                + 14i32) as isize,
        ))
        .b32
        .s1;
        p = b;
        if x > 0i32 {
            e = z - d - (*mem.offset((p + 1i32) as isize)).b32.s1
        } else {
            e = d;
            d = z - e - (*mem.offset((p + 1i32) as isize)).b32.s1
        }
        if j != -0xfffffffi32 {
            b = copy_node_list(j);
            (*mem.offset((b + 3i32) as isize)).b32.s1 = (*mem.offset((p + 3i32) as isize)).b32.s1;
            (*mem.offset((b + 2i32) as isize)).b32.s1 = (*mem.offset((p + 2i32) as isize)).b32.s1;
            s = s - (*mem.offset((b + 4i32) as isize)).b32.s1;
            d = d + s;
            e = e + (*mem.offset((b + 1i32) as isize)).b32.s1 - z - s
        }
        if (*mem.offset(p as isize)).b16.s0 as libc::c_int == 2i32 {
            q = p
        } else {
            r = (*mem.offset((p + 5i32) as isize)).b32.s1;
            free_node(p, 8i32);
            if r == -0xfffffffi32 {
                confusion(b"LR4\x00" as *const u8 as *const libc::c_char);
            }
            if x > 0i32 {
                p = r;
                loop {
                    q = r;
                    r = (*mem.offset(r as isize)).b32.s1;
                    if r == -0xfffffffi32 {
                        break;
                    }
                }
            } else {
                p = -0xfffffffi32;
                q = r;
                loop {
                    t = (*mem.offset(r as isize)).b32.s1;
                    (*mem.offset(r as isize)).b32.s1 = p;
                    p = r;
                    r = t;
                    if r == -0xfffffffi32 {
                        break;
                    }
                }
            }
        }
        if j == -0xfffffffi32 {
            r = new_kern(0i32);
            t = new_kern(0i32)
        } else {
            r = (*mem.offset((b + 5i32) as isize)).b32.s1;
            t = (*mem.offset(r as isize)).b32.s1
        }
        u = new_math(0i32, 3i32 as small_number);
        if (*mem.offset(t as isize)).b16.s1 as libc::c_int == 10i32 {
            j = new_skip_param(8i32 as small_number);
            (*mem.offset(q as isize)).b32.s1 = j;
            (*mem.offset(j as isize)).b32.s1 = u;
            j = (*mem.offset((t + 1i32) as isize)).b32.s0;
            (*mem.offset(temp_ptr as isize)).b16.s1 = (*mem.offset(j as isize)).b16.s1;
            (*mem.offset(temp_ptr as isize)).b16.s0 = (*mem.offset(j as isize)).b16.s0;
            (*mem.offset((temp_ptr + 1i32) as isize)).b32.s1 =
                e - (*mem.offset((j + 1i32) as isize)).b32.s1;
            (*mem.offset((temp_ptr + 2i32) as isize)).b32.s1 =
                -(*mem.offset((j + 2i32) as isize)).b32.s1;
            (*mem.offset((temp_ptr + 3i32) as isize)).b32.s1 =
                -(*mem.offset((j + 3i32) as isize)).b32.s1;
            (*mem.offset(u as isize)).b32.s1 = t
        } else {
            (*mem.offset((t + 1i32) as isize)).b32.s1 = e;
            (*mem.offset(t as isize)).b32.s1 = u;
            (*mem.offset(q as isize)).b32.s1 = t
        }
        u = new_math(0i32, 2i32 as small_number);
        if (*mem.offset(r as isize)).b16.s1 as libc::c_int == 10i32 {
            j = new_skip_param(7i32 as small_number);
            (*mem.offset(u as isize)).b32.s1 = j;
            (*mem.offset(j as isize)).b32.s1 = p;
            j = (*mem.offset((r + 1i32) as isize)).b32.s0;
            (*mem.offset(temp_ptr as isize)).b16.s1 = (*mem.offset(j as isize)).b16.s1;
            (*mem.offset(temp_ptr as isize)).b16.s0 = (*mem.offset(j as isize)).b16.s0;
            (*mem.offset((temp_ptr + 1i32) as isize)).b32.s1 =
                d - (*mem.offset((j + 1i32) as isize)).b32.s1;
            (*mem.offset((temp_ptr + 2i32) as isize)).b32.s1 =
                -(*mem.offset((j + 2i32) as isize)).b32.s1;
            (*mem.offset((temp_ptr + 3i32) as isize)).b32.s1 =
                -(*mem.offset((j + 3i32) as isize)).b32.s1;
            (*mem.offset(r as isize)).b32.s1 = u
        } else {
            (*mem.offset((r + 1i32) as isize)).b32.s1 = d;
            (*mem.offset(r as isize)).b32.s1 = p;
            (*mem.offset(u as isize)).b32.s1 = r;
            if j == -0xfffffffi32 {
                b = hpack(u, 0i32, 1i32 as small_number);
                (*mem.offset((b + 4i32) as isize)).b32.s1 = s
            } else {
                (*mem.offset((b + 5i32) as isize)).b32.s1 = u
            }
        }
    }
    append_to_vlist(b);
}
#[no_mangle]
pub unsafe extern "C" fn after_math() {
    let mut l: bool = false;
    let mut danger: bool = false;
    let mut m: int32_t = 0;
    let mut p: int32_t = 0;
    let mut a: int32_t = 0;
    let mut b: int32_t = 0;
    let mut w: scaled_t = 0;
    let mut z: scaled_t = 0;
    let mut e: scaled_t = 0;
    let mut q: scaled_t = 0;
    let mut d: scaled_t = 0;
    let mut s: scaled_t = 0;
    let mut g1: small_number = 0;
    let mut g2: small_number = 0;
    let mut r: int32_t = 0;
    let mut t: int32_t = 0;
    let mut pre_t: int32_t = 0;
    let mut j: int32_t = -0xfffffffi32;
    danger = 0i32 != 0;
    if cur_list.mode as libc::c_int == 207i32 {
        j = cur_list.eTeX_aux
    }
    if *font_params.offset(
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
                + 2i32) as isize,
        ))
        .b32
        .s1 as isize,
    ) < 22i32
        && !(*font_area.offset(
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
                    + 2i32) as isize,
            ))
            .b32
            .s1 as isize,
        ) as libc::c_uint
            == 0xfffeu32
            && isOpenTypeMathFont(
                *font_layout_engine.offset(
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
                            + 2i32) as isize,
                    ))
                    .b32
                    .s1 as isize,
                ) as XeTeXLayoutEngine,
            ) as libc::c_int
                != 0)
        || *font_params.offset(
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
                    + (2i32 + 256i32)) as isize,
            ))
            .b32
            .s1 as isize,
        ) < 22i32
            && !(*font_area.offset(
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
                        + (2i32 + 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) as libc::c_uint
                == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(
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
                                + (2i32 + 256i32)) as isize,
                        ))
                        .b32
                        .s1 as isize,
                    ) as XeTeXLayoutEngine,
                ) as libc::c_int
                    != 0)
        || *font_params.offset(
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
                    + (2i32 + 2i32 * 256i32)) as isize,
            ))
            .b32
            .s1 as isize,
        ) < 22i32
            && !(*font_area.offset(
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
                        + (2i32 + 2i32 * 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) as libc::c_uint
                == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(
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
                                + (2i32 + 2i32 * 256i32)) as isize,
                        ))
                        .b32
                        .s1 as isize,
                    ) as XeTeXLayoutEngine,
                ) as libc::c_int
                    != 0)
    {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(
            b"Math formula deleted: Insufficient symbol fonts\x00" as *const u8
                as *const libc::c_char,
        );
        help_ptr = 3i32 as u8;
        help_line[2] = b"Sorry, but I can\'t typeset math unless \\textfont 2\x00" as *const u8
            as *const libc::c_char;
        help_line[1] = b"and \\scriptfont 2 and \\scriptscriptfont 2 have all\x00" as *const u8
            as *const libc::c_char;
        help_line[0] = b"the \\fontdimen values needed in math symbol fonts.\x00" as *const u8
            as *const libc::c_char;
        error();
        flush_math();
        danger = 1i32 != 0
    } else if *font_params.offset(
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
                + (3i32 + 0i32)) as isize,
        ))
        .b32
        .s1 as isize,
    ) < 13i32
        && !(*font_area.offset(
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
                    + (3i32 + 0i32)) as isize,
            ))
            .b32
            .s1 as isize,
        ) as libc::c_uint
            == 0xfffeu32
            && isOpenTypeMathFont(
                *font_layout_engine.offset(
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
                            + (3i32 + 0i32)) as isize,
                    ))
                    .b32
                    .s1 as isize,
                ) as XeTeXLayoutEngine,
            ) as libc::c_int
                != 0)
        || *font_params.offset(
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
                    + (3i32 + 256i32)) as isize,
            ))
            .b32
            .s1 as isize,
        ) < 13i32
            && !(*font_area.offset(
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
                        + (3i32 + 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) as libc::c_uint
                == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(
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
                                + (3i32 + 256i32)) as isize,
                        ))
                        .b32
                        .s1 as isize,
                    ) as XeTeXLayoutEngine,
                ) as libc::c_int
                    != 0)
        || *font_params.offset(
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
                    + (3i32 + 2i32 * 256i32)) as isize,
            ))
            .b32
            .s1 as isize,
        ) < 13i32
            && !(*font_area.offset(
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
                        + (3i32 + 2i32 * 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) as libc::c_uint
                == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(
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
                                + (3i32 + 2i32 * 256i32)) as isize,
                        ))
                        .b32
                        .s1 as isize,
                    ) as XeTeXLayoutEngine,
                ) as libc::c_int
                    != 0)
    {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(
            b"Math formula deleted: Insufficient extension fonts\x00" as *const u8
                as *const libc::c_char,
        );
        help_ptr = 3i32 as u8;
        help_line[2] = b"Sorry, but I can\'t typeset math unless \\textfont 3\x00" as *const u8
            as *const libc::c_char;
        help_line[1] = b"and \\scriptfont 3 and \\scriptscriptfont 3 have all\x00" as *const u8
            as *const libc::c_char;
        help_line[0] = b"the \\fontdimen values needed in math extension fonts.\x00" as *const u8
            as *const libc::c_char;
        error();
        flush_math();
        danger = 1i32 != 0
    }
    m = cur_list.mode as int32_t;
    l = 0i32 != 0;
    p = fin_mlist(-0xfffffffi32);
    if cur_list.mode as libc::c_int == -m {
        get_x_token();
        if cur_cmd as libc::c_int != 3i32 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
            }
            print_cstr(b"Display math should end with $$\x00" as *const u8 as *const libc::c_char);
            help_ptr = 2i32 as u8;
            help_line[1] = b"The `$\' that I just saw supposedly matches a previous `$$\'.\x00"
                as *const u8 as *const libc::c_char;
            help_line[0] = b"So I shall assume that you typed `$$\' both times.\x00" as *const u8
                as *const libc::c_char;
            back_error();
        }
        cur_mlist = p;
        cur_style = 2i32 as small_number;
        mlist_penalties = 0i32 != 0;
        mlist_to_hlist();
        a = hpack(
            (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1,
            0i32,
            1i32 as small_number,
        );
        (*mem.offset(a as isize)).b16.s0 = 2i32 as u16;
        unsave();
        save_ptr -= 1;
        if (*save_stack.offset((save_ptr + 0i32) as isize)).b32.s1 == 1i32 {
            l = 1i32 != 0
        }
        danger = 0i32 != 0;
        if cur_list.mode as libc::c_int == 207i32 {
            j = cur_list.eTeX_aux
        }
        if *font_params.offset(
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
                    + 2i32) as isize,
            ))
            .b32
            .s1 as isize,
        ) < 22i32
            && !(*font_area.offset(
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
                        + 2i32) as isize,
                ))
                .b32
                .s1 as isize,
            ) as libc::c_uint
                == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(
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
                                + 2i32) as isize,
                        ))
                        .b32
                        .s1 as isize,
                    ) as XeTeXLayoutEngine,
                ) as libc::c_int
                    != 0)
            || *font_params.offset(
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
                        + (2i32 + 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) < 22i32
                && !(*font_area.offset(
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
                            + (2i32 + 256i32)) as isize,
                    ))
                    .b32
                    .s1 as isize,
                ) as libc::c_uint
                    == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(
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
                                    + (2i32 + 256i32)) as isize,
                            ))
                            .b32
                            .s1 as isize,
                        ) as XeTeXLayoutEngine,
                    ) as libc::c_int
                        != 0)
            || *font_params.offset(
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
                        + (2i32 + 2i32 * 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) < 22i32
                && !(*font_area.offset(
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
                            + (2i32 + 2i32 * 256i32)) as isize,
                    ))
                    .b32
                    .s1 as isize,
                ) as libc::c_uint
                    == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(
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
                                    + (2i32 + 2i32 * 256i32))
                                    as isize,
                            ))
                            .b32
                            .s1 as isize,
                        ) as XeTeXLayoutEngine,
                    ) as libc::c_int
                        != 0)
        {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
            }
            print_cstr(
                b"Math formula deleted: Insufficient symbol fonts\x00" as *const u8
                    as *const libc::c_char,
            );
            help_ptr = 3i32 as u8;
            help_line[2] = b"Sorry, but I can\'t typeset math unless \\textfont 2\x00" as *const u8
                as *const libc::c_char;
            help_line[1] = b"and \\scriptfont 2 and \\scriptscriptfont 2 have all\x00" as *const u8
                as *const libc::c_char;
            help_line[0] = b"the \\fontdimen values needed in math symbol fonts.\x00" as *const u8
                as *const libc::c_char;
            error();
            flush_math();
            danger = 1i32 != 0
        } else if *font_params.offset(
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
                    + (3i32 + 0i32)) as isize,
            ))
            .b32
            .s1 as isize,
        ) < 13i32
            && !(*font_area.offset(
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
                        + (3i32 + 0i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) as libc::c_uint
                == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(
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
                                + (3i32 + 0i32)) as isize,
                        ))
                        .b32
                        .s1 as isize,
                    ) as XeTeXLayoutEngine,
                ) as libc::c_int
                    != 0)
            || *font_params.offset(
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
                        + (3i32 + 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) < 13i32
                && !(*font_area.offset(
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
                            + (3i32 + 256i32)) as isize,
                    ))
                    .b32
                    .s1 as isize,
                ) as libc::c_uint
                    == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(
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
                                    + (3i32 + 256i32)) as isize,
                            ))
                            .b32
                            .s1 as isize,
                        ) as XeTeXLayoutEngine,
                    ) as libc::c_int
                        != 0)
            || *font_params.offset(
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
                        + (3i32 + 2i32 * 256i32)) as isize,
                ))
                .b32
                .s1 as isize,
            ) < 13i32
                && !(*font_area.offset(
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
                            + (3i32 + 2i32 * 256i32)) as isize,
                    ))
                    .b32
                    .s1 as isize,
                ) as libc::c_uint
                    == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(
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
                                    + (3i32 + 2i32 * 256i32))
                                    as isize,
                            ))
                            .b32
                            .s1 as isize,
                        ) as XeTeXLayoutEngine,
                    ) as libc::c_int
                        != 0)
        {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
            }
            print_cstr(
                b"Math formula deleted: Insufficient extension fonts\x00" as *const u8
                    as *const libc::c_char,
            );
            help_ptr = 3i32 as u8;
            help_line[2] = b"Sorry, but I can\'t typeset math unless \\textfont 3\x00" as *const u8
                as *const libc::c_char;
            help_line[1] = b"and \\scriptfont 3 and \\scriptscriptfont 3 have all\x00" as *const u8
                as *const libc::c_char;
            help_line[0] = b"the \\fontdimen values needed in math extension fonts.\x00"
                as *const u8 as *const libc::c_char;
            error();
            flush_math();
            danger = 1i32 != 0
        }
        m = cur_list.mode as int32_t;
        p = fin_mlist(-0xfffffffi32)
    } else {
        a = -0xfffffffi32
    }
    if m < 0i32 {
        /*1231: */
        (*mem.offset(cur_list.tail as isize)).b32.s1 = new_math(
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
                    + (0x10ffffi32 + 1i32)
                    + 1i32) as isize,
            ))
            .b32
            .s1,
            0i32 as small_number,
        );
        cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
        cur_mlist = p;
        cur_style = 2i32 as small_number;
        mlist_penalties = cur_list.mode as libc::c_int > 0i32;
        mlist_to_hlist();
        (*mem.offset(cur_list.tail as isize)).b32.s1 =
            (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1;
        while (*mem.offset(cur_list.tail as isize)).b32.s1 != -0xfffffffi32 {
            cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1
        }
        (*mem.offset(cur_list.tail as isize)).b32.s1 = new_math(
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
                    + (0x10ffffi32 + 1i32)
                    + 1i32) as isize,
            ))
            .b32
            .s1,
            1i32 as small_number,
        );
        cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
        cur_list.aux.b32.s0 = 1000i32;
        unsave();
    } else {
        if a == -0xfffffffi32 {
            /*1232: */
            get_x_token();
            if cur_cmd as libc::c_int != 3i32 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
                }
                print_cstr(
                    b"Display math should end with $$\x00" as *const u8 as *const libc::c_char,
                );
                help_ptr = 2i32 as u8;
                help_line[1] = b"The `$\' that I just saw supposedly matches a previous `$$\'.\x00"
                    as *const u8 as *const libc::c_char;
                help_line[0] = b"So I shall assume that you typed `$$\' both times.\x00"
                    as *const u8 as *const libc::c_char;
                back_error();
            }
        }
        cur_mlist = p;
        cur_style = 0i32 as small_number;
        mlist_penalties = 0i32 != 0;
        mlist_to_hlist();
        p = (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1;
        adjust_tail = 4999999i32 - 5i32;
        pre_adjust_tail = 4999999i32 - 14i32;
        b = hpack(p, 0i32, 1i32 as small_number);
        p = (*mem.offset((b + 5i32) as isize)).b32.s1;
        t = adjust_tail;
        adjust_tail = -0xfffffffi32;
        pre_t = pre_adjust_tail;
        pre_adjust_tail = -0xfffffffi32;
        w = (*mem.offset((b + 1i32) as isize)).b32.s1;
        z = (*eqtb.offset(
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
                + 14i32) as isize,
        ))
        .b32
        .s1;
        s = (*eqtb.offset(
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
                + 15i32) as isize,
        ))
        .b32
        .s1;
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
                + 63i32) as isize,
        ))
        .b32
        .s1 < 0i32
        {
            s = -s - z
        }
        if a == -0xfffffffi32 || danger as libc::c_int != 0 {
            e = 0i32;
            q = 0i32
        } else {
            e = (*mem.offset((a + 1i32) as isize)).b32.s1;
            q = e + math_quad(0i32)
        }
        if w + q > z {
            /*1236: */
            if e != 0i32
                && (w - total_shrink[0] + q <= z
                    || total_shrink[1] != 0i32
                    || total_shrink[2] != 0i32
                    || total_shrink[3] != 0i32)
            {
                free_node(b, 8i32);
                b = hpack(p, z - q, 0i32 as small_number)
            } else {
                e = 0i32;
                if w > z {
                    free_node(b, 8i32);
                    b = hpack(p, z, 0i32 as small_number)
                }
            }
            w = (*mem.offset((b + 1i32) as isize)).b32.s1
        }
        (*mem.offset(b as isize)).b16.s0 = 2i32 as u16;
        d = half(z - w);
        if e > 0i32 && d < 2i32 * e {
            d = half(z - w - e);
            if p != -0xfffffffi32 {
                if !is_char_node(p) {
                    if (*mem.offset(p as isize)).b16.s1 as libc::c_int == 10i32 {
                        d = 0i32
                    }
                }
            }
        }
        (*mem.offset(cur_list.tail as isize)).b32.s1 = new_penalty(
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
                    + 11i32) as isize,
            ))
            .b32
            .s1,
        );
        cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
        if d + s
            <= (*eqtb.offset(
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
                    + 13i32) as isize,
            ))
            .b32
            .s1
            || l as libc::c_int != 0
        {
            g1 = 3i32 as small_number;
            g2 = 4i32 as small_number
        } else {
            g1 = 5i32 as small_number;
            g2 = 6i32 as small_number
        }
        if l as libc::c_int != 0 && e == 0i32 {
            app_display(j, a, 0i32);
            (*mem.offset(cur_list.tail as isize)).b32.s1 = new_penalty(10000i32);
            cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1
        } else {
            (*mem.offset(cur_list.tail as isize)).b32.s1 = new_param_glue(g1);
            cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1
        }
        if e != 0i32 {
            r = new_kern(z - w - e - d);
            if l {
                (*mem.offset(a as isize)).b32.s1 = r;
                (*mem.offset(r as isize)).b32.s1 = b;
                b = a;
                d = 0i32
            } else {
                (*mem.offset(b as isize)).b32.s1 = r;
                (*mem.offset(r as isize)).b32.s1 = a
            }
            b = hpack(b, 0i32, 1i32 as small_number)
        }
        app_display(j, b, d);
        if a != -0xfffffffi32 && e == 0i32 && !l {
            (*mem.offset(cur_list.tail as isize)).b32.s1 = new_penalty(10000i32);
            cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
            app_display(j, a, z - (*mem.offset((a + 1i32) as isize)).b32.s1);
            g2 = 0i32 as small_number
        }
        if t != 4999999i32 - 5i32 {
            (*mem.offset(cur_list.tail as isize)).b32.s1 =
                (*mem.offset((4999999i32 - 5i32) as isize)).b32.s1;
            cur_list.tail = t
        }
        if pre_t != 4999999i32 - 14i32 {
            (*mem.offset(cur_list.tail as isize)).b32.s1 =
                (*mem.offset((4999999i32 - 14i32) as isize)).b32.s1;
            cur_list.tail = pre_t
        }
        (*mem.offset(cur_list.tail as isize)).b32.s1 = new_penalty(
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
                    + 12i32) as isize,
            ))
            .b32
            .s1,
        );
        cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1;
        if g2 as libc::c_int > 0i32 {
            (*mem.offset(cur_list.tail as isize)).b32.s1 = new_param_glue(g2);
            cur_list.tail = (*mem.offset(cur_list.tail as isize)).b32.s1
        }
        flush_node_list(j);
        resume_after_display();
    };
}
#[no_mangle]
pub unsafe extern "C" fn resume_after_display() {
    if cur_group as libc::c_int != 15i32 {
        confusion(b"display\x00" as *const u8 as *const libc::c_char);
    }
    unsave();
    cur_list.prev_graf = cur_list.prev_graf + 3i32;
    push_nest();
    cur_list.mode = 104i32 as libc::c_short;
    cur_list.aux.b32.s0 = 1000i32;
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
        cur_lang = 0i32 as u8
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
        cur_lang = 0i32 as u8
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
    cur_list.aux.b32.s1 = cur_lang as int32_t;
    cur_list.prev_graf = ((norm_min(
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
                + 51i32) as isize,
        ))
        .b32
        .s1,
    ) as libc::c_int
        * 64i32
        + norm_min(
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
                    + 52i32) as isize,
            ))
            .b32
            .s1,
        ) as libc::c_int) as libc::c_long
        * 65536
        + cur_lang as libc::c_long) as int32_t;
    get_x_token();
    if cur_cmd as libc::c_int != 10i32 {
        back_input();
    }
    if nest_ptr == 1i32 {
        build_page();
    };
}
/* Copyright 2016-2018 The Tectonic Project
 * Licensed under the MIT License.
 */
unsafe extern "C" fn math_x_height(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 5i32)
    } else {
        rval = (*font_info.offset((5i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn math_quad(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 6i32)
    } else {
        rval = (*font_info.offset((6i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn num1(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 8i32)
    } else {
        rval = (*font_info.offset((8i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn num2(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 9i32)
    } else {
        rval = (*font_info.offset((9i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn num3(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 10i32)
    } else {
        rval = (*font_info.offset((10i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn denom1(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 11i32)
    } else {
        rval = (*font_info.offset((11i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn denom2(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 12i32)
    } else {
        rval = (*font_info.offset((12i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sup1(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 13i32)
    } else {
        rval = (*font_info.offset((13i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sup2(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 14i32)
    } else {
        rval = (*font_info.offset((14i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sup3(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 15i32)
    } else {
        rval = (*font_info.offset((15i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sub1(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 16i32)
    } else {
        rval = (*font_info.offset((16i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sub2(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 17i32)
    } else {
        rval = (*font_info.offset((17i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sup_drop(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 18i32)
    } else {
        rval = (*font_info.offset((18i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn sub_drop(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 19i32)
    } else {
        rval = (*font_info.offset((19i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn delim1(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 20i32)
    } else {
        rval = (*font_info.offset((20i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn delim2(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 21i32)
    } else {
        rval = (*font_info.offset((21i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn axis_height(mut size_code: int32_t) -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (2i32 + size_code)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathsy_param(f, 22i32)
    } else {
        rval = (*font_info.offset((22i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn default_rule_thickness() -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (3i32 + cur_size)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathex_param(f, 8i32)
    } else {
        rval = (*font_info.offset((8i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn big_op_spacing1() -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (3i32 + cur_size)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathex_param(f, 9i32)
    } else {
        rval = (*font_info.offset((9i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn big_op_spacing2() -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (3i32 + cur_size)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathex_param(f, 10i32)
    } else {
        rval = (*font_info.offset((10i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn big_op_spacing3() -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (3i32 + cur_size)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathex_param(f, 11i32)
    } else {
        rval = (*font_info.offset((11i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn big_op_spacing4() -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (3i32 + cur_size)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathex_param(f, 12i32)
    } else {
        rval = (*font_info.offset((12i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn big_op_spacing5() -> scaled_t {
    let mut f: int32_t = 0;
    let mut rval: scaled_t = 0;
    f = (*eqtb.offset(
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
            + (3i32 + cur_size)) as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rval = get_native_mathex_param(f, 13i32)
    } else {
        rval = (*font_info.offset((13i32 + *param_base.offset(f as isize)) as isize))
            .b32
            .s1
    }
    return rval;
}
unsafe extern "C" fn fraction_rule(mut t: scaled_t) -> int32_t {
    let mut p: int32_t = 0;
    p = new_rule();
    (*mem.offset((p + 3i32) as isize)).b32.s1 = t;
    (*mem.offset((p + 2i32) as isize)).b32.s1 = 0i32;
    return p;
}
unsafe extern "C" fn overbar(mut b: int32_t, mut k: scaled_t, mut t: scaled_t) -> int32_t {
    let mut p: int32_t = 0;
    let mut q: int32_t = 0;
    p = new_kern(k);
    (*mem.offset(p as isize)).b32.s1 = b;
    q = fraction_rule(t);
    (*mem.offset(q as isize)).b32.s1 = p;
    p = new_kern(t);
    (*mem.offset(p as isize)).b32.s1 = q;
    return vpackage(p, 0i32, 1i32 as small_number, 0x3fffffffi32);
}
unsafe extern "C" fn math_glue(mut g: int32_t, mut m: scaled_t) -> int32_t {
    let mut p: int32_t = 0;
    let mut n: int32_t = 0;
    let mut f: scaled_t = 0;
    n = x_over_n(m, 65536 as int32_t);
    f = tex_remainder;
    if f < 0i32 {
        n -= 1;
        f = (f as libc::c_long + 65536) as scaled_t
    }
    p = get_node(4i32);
    (*mem.offset((p + 1i32) as isize)).b32.s1 = mult_and_add(
        n,
        (*mem.offset((g + 1i32) as isize)).b32.s1,
        xn_over_d(
            (*mem.offset((g + 1i32) as isize)).b32.s1,
            f,
            65536 as int32_t,
        ),
        0x3fffffffi32,
    );
    (*mem.offset(p as isize)).b16.s1 = (*mem.offset(g as isize)).b16.s1;
    if (*mem.offset(p as isize)).b16.s1 as libc::c_int == 0i32 {
        (*mem.offset((p + 2i32) as isize)).b32.s1 = mult_and_add(
            n,
            (*mem.offset((g + 2i32) as isize)).b32.s1,
            xn_over_d(
                (*mem.offset((g + 2i32) as isize)).b32.s1,
                f,
                65536 as int32_t,
            ),
            0x3fffffffi32,
        )
    } else {
        (*mem.offset((p + 2i32) as isize)).b32.s1 = (*mem.offset((g + 2i32) as isize)).b32.s1
    }
    (*mem.offset(p as isize)).b16.s0 = (*mem.offset(g as isize)).b16.s0;
    if (*mem.offset(p as isize)).b16.s0 as libc::c_int == 0i32 {
        (*mem.offset((p + 3i32) as isize)).b32.s1 = mult_and_add(
            n,
            (*mem.offset((g + 3i32) as isize)).b32.s1,
            xn_over_d(
                (*mem.offset((g + 3i32) as isize)).b32.s1,
                f,
                65536 as int32_t,
            ),
            0x3fffffffi32,
        )
    } else {
        (*mem.offset((p + 3i32) as isize)).b32.s1 = (*mem.offset((g + 3i32) as isize)).b32.s1
    }
    return p;
}
unsafe extern "C" fn math_kern(mut p: int32_t, mut m: scaled_t) {
    let mut n: int32_t = 0;
    let mut f: scaled_t = 0;
    if (*mem.offset(p as isize)).b16.s0 as libc::c_int == 99i32 {
        n = x_over_n(m, 65536 as int32_t);
        f = tex_remainder;
        if f < 0i32 {
            n -= 1;
            f = (f as libc::c_long + 65536) as scaled_t
        }
        (*mem.offset((p + 1i32) as isize)).b32.s1 = mult_and_add(
            n,
            (*mem.offset((p + 1i32) as isize)).b32.s1,
            xn_over_d(
                (*mem.offset((p + 1i32) as isize)).b32.s1,
                f,
                65536 as int32_t,
            ),
            0x3fffffffi32,
        );
        (*mem.offset(p as isize)).b16.s0 = 1i32 as u16
    };
}
#[no_mangle]
pub unsafe extern "C" fn flush_math() {
    flush_node_list((*mem.offset(cur_list.head as isize)).b32.s1);
    flush_node_list(cur_list.aux.b32.s1);
    (*mem.offset(cur_list.head as isize)).b32.s1 = -0xfffffffi32;
    cur_list.tail = cur_list.head;
    cur_list.aux.b32.s1 = -0xfffffffi32;
}
unsafe extern "C" fn clean_box(mut p: int32_t, mut s: small_number) -> int32_t {
    let mut current_block: u64;
    let mut q: int32_t = 0;
    let mut save_style: small_number = 0;
    let mut x: int32_t = 0;
    let mut r: int32_t = 0;
    match (*mem.offset(p as isize)).b32.s1 {
        1 => {
            cur_mlist = new_noad();
            *mem.offset((cur_mlist + 1i32) as isize) = *mem.offset(p as isize);
            current_block = 12209867499936983673;
        }
        2 => {
            q = (*mem.offset(p as isize)).b32.s0;
            current_block = 3089856285952541829;
        }
        3 => {
            cur_mlist = (*mem.offset(p as isize)).b32.s0;
            current_block = 12209867499936983673;
        }
        _ => {
            q = new_null_box();
            current_block = 3089856285952541829;
        }
    }
    match current_block {
        12209867499936983673 => {
            save_style = cur_style;
            cur_style = s;
            mlist_penalties = 0i32 != 0;
            mlist_to_hlist();
            q = (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1;
            cur_style = save_style;
            if (cur_style as libc::c_int) < 4i32 {
                cur_size = 0i32
            } else {
                cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
            }
            cur_mu = x_over_n(math_quad(cur_size), 18i32)
        }
        _ => {}
    }
    if is_char_node(q) as libc::c_int != 0 || q == -0xfffffffi32 {
        x = hpack(q, 0i32, 1i32 as small_number)
    } else if (*mem.offset(q as isize)).b32.s1 == -0xfffffffi32
        && (*mem.offset(q as isize)).b16.s1 as libc::c_int <= 1i32
        && (*mem.offset((q + 4i32) as isize)).b32.s1 == 0i32
    {
        x = q
    } else {
        x = hpack(q, 0i32, 1i32 as small_number)
    }
    q = (*mem.offset((x + 5i32) as isize)).b32.s1;
    if is_char_node(q) {
        r = (*mem.offset(q as isize)).b32.s1;
        if r != -0xfffffffi32 {
            if (*mem.offset(r as isize)).b32.s1 == -0xfffffffi32 {
                if !is_char_node(r) {
                    if (*mem.offset(r as isize)).b16.s1 as libc::c_int == 11i32 {
                        free_node(r, 3i32);
                        (*mem.offset(q as isize)).b32.s1 = -0xfffffffi32
                    }
                }
            }
        }
    }
    return x;
}
unsafe extern "C" fn fetch(mut a: int32_t) {
    cur_c = (*mem.offset(a as isize)).b16.s0 as int32_t;
    cur_f = (*eqtb.offset(
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
            + ((*mem.offset(a as isize)).b16.s1 as libc::c_int % 256i32 + cur_size))
            as isize,
    ))
    .b32
    .s1;
    cur_c = (cur_c as libc::c_long
        + ((*mem.offset(a as isize)).b16.s1 as libc::c_int / 256i32) as libc::c_long * 65536)
        as int32_t;
    if cur_f == 0i32 {
        /*749: */
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(b"\x00" as *const u8 as *const libc::c_char);
        print_size(cur_size);
        print_char(' ' as i32);
        print_int((*mem.offset(a as isize)).b16.s1 as libc::c_int % 256i32);
        print_cstr(b" is undefined (character \x00" as *const u8 as *const libc::c_char);
        print(cur_c);
        print_char(')' as i32);
        help_ptr = 4i32 as u8;
        help_line[3] = b"Somewhere in the math formula just ended, you used the\x00" as *const u8
            as *const libc::c_char;
        help_line[2] = b"stated character from an undefined font family. For example,\x00"
            as *const u8 as *const libc::c_char;
        help_line[1] = b"plain TeX doesn\'t allow \\it or \\sl in subscripts. Proceed,\x00"
            as *const u8 as *const libc::c_char;
        help_line[0] = b"and I\'ll try to forget that I needed that character.\x00" as *const u8
            as *const libc::c_char;
        error();
        cur_i = null_character;
        (*mem.offset(a as isize)).b32.s1 = 0i32
    } else if *font_area.offset(cur_f as isize) as libc::c_uint == 0xffffu32
        || *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
    {
        cur_i = null_character
    } else {
        if cur_c >= *font_bc.offset(cur_f as isize) as libc::c_int
            && cur_c <= *font_ec.offset(cur_f as isize) as libc::c_int
        {
            cur_i = (*font_info.offset((*char_base.offset(cur_f as isize) + cur_c) as isize)).b16
        } else {
            cur_i = null_character
        }
        if !(cur_i.s3 as libc::c_int > 0i32) {
            char_warning(cur_f, cur_c);
            (*mem.offset(a as isize)).b32.s1 = 0i32
        }
    };
}
unsafe extern "C" fn make_over(mut q: int32_t) {
    (*mem.offset((q + 1i32) as isize)).b32.s0 = overbar(
        clean_box(
            q + 1i32,
            (2i32 * (cur_style as libc::c_int / 2i32) + 1i32) as small_number,
        ),
        3i32 * default_rule_thickness(),
        default_rule_thickness(),
    );
    (*mem.offset((q + 1i32) as isize)).b32.s1 = 2i32;
}
unsafe extern "C" fn make_under(mut q: int32_t) {
    let mut p: int32_t = 0;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut delta: scaled_t = 0;
    x = clean_box(q + 1i32, cur_style);
    p = new_kern(3i32 * default_rule_thickness());
    (*mem.offset(x as isize)).b32.s1 = p;
    (*mem.offset(p as isize)).b32.s1 = fraction_rule(default_rule_thickness());
    y = vpackage(x, 0i32, 1i32 as small_number, 0x3fffffffi32);
    delta = (*mem.offset((y + 3i32) as isize)).b32.s1
        + (*mem.offset((y + 2i32) as isize)).b32.s1
        + default_rule_thickness();
    (*mem.offset((y + 3i32) as isize)).b32.s1 = (*mem.offset((x + 3i32) as isize)).b32.s1;
    (*mem.offset((y + 2i32) as isize)).b32.s1 = delta - (*mem.offset((y + 3i32) as isize)).b32.s1;
    (*mem.offset((q + 1i32) as isize)).b32.s0 = y;
    (*mem.offset((q + 1i32) as isize)).b32.s1 = 2i32;
}
unsafe extern "C" fn make_vcenter(mut q: int32_t) {
    let mut v: int32_t = 0;
    let mut delta: scaled_t = 0;
    v = (*mem.offset((q + 1i32) as isize)).b32.s0;
    if (*mem.offset(v as isize)).b16.s1 as libc::c_int != 1i32 {
        confusion(b"vcenter\x00" as *const u8 as *const libc::c_char);
    }
    delta = (*mem.offset((v + 3i32) as isize)).b32.s1 + (*mem.offset((v + 2i32) as isize)).b32.s1;
    (*mem.offset((v + 3i32) as isize)).b32.s1 = axis_height(cur_size) + half(delta);
    (*mem.offset((v + 2i32) as isize)).b32.s1 = delta - (*mem.offset((v + 3i32) as isize)).b32.s1;
}
unsafe extern "C" fn make_radical(mut q: int32_t) {
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut f: internal_font_number = 0;
    let mut rule_thickness: scaled_t = 0;
    let mut delta: scaled_t = 0;
    let mut clr: scaled_t = 0;
    f = (*eqtb.offset(
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
            + ((*mem.offset((q + 4i32) as isize)).b16.s3 as libc::c_int % 256i32 + cur_size))
            as isize,
    ))
    .b32
    .s1;
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        rule_thickness = get_ot_math_constant(f, 51i32)
    } else {
        rule_thickness = default_rule_thickness()
    }
    x = clean_box(
        q + 1i32,
        (2i32 * (cur_style as libc::c_int / 2i32) + 1i32) as small_number,
    );
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        if (cur_style as libc::c_int) < 2i32 {
            clr = get_ot_math_constant(f, 50i32)
        } else {
            clr = get_ot_math_constant(f, 49i32)
        }
    } else if (cur_style as libc::c_int) < 2i32 {
        clr = rule_thickness + abs(math_x_height(cur_size)) / 4i32
    } else {
        clr = rule_thickness;
        clr = clr + abs(clr) / 4i32
    }
    y = var_delimiter(
        q + 4i32,
        cur_size,
        (*mem.offset((x + 3i32) as isize)).b32.s1
            + (*mem.offset((x + 2i32) as isize)).b32.s1
            + clr
            + rule_thickness,
    );
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
            as libc::c_int
            != 0
    {
        (*mem.offset((y + 2i32) as isize)).b32.s1 = (*mem.offset((y + 3i32) as isize)).b32.s1
            + (*mem.offset((y + 2i32) as isize)).b32.s1
            - rule_thickness;
        (*mem.offset((y + 3i32) as isize)).b32.s1 = rule_thickness
    }
    delta = (*mem.offset((y + 2i32) as isize)).b32.s1
        - ((*mem.offset((x + 3i32) as isize)).b32.s1
            + (*mem.offset((x + 2i32) as isize)).b32.s1
            + clr);
    if delta > 0i32 {
        clr = clr + half(delta)
    }
    (*mem.offset((y + 4i32) as isize)).b32.s1 = -((*mem.offset((x + 3i32) as isize)).b32.s1 + clr);
    (*mem.offset(y as isize)).b32.s1 = overbar(x, clr, (*mem.offset((y + 3i32) as isize)).b32.s1);
    (*mem.offset((q + 1i32) as isize)).b32.s0 = hpack(y, 0i32, 1i32 as small_number);
    (*mem.offset((q + 1i32) as isize)).b32.s1 = 2i32;
}
unsafe extern "C" fn compute_ot_math_accent_pos(mut p: int32_t) -> scaled_t {
    let mut q: int32_t = 0;
    let mut r: int32_t = 0;
    let mut s: scaled_t = 0;
    let mut g: scaled_t = 0;
    if (*mem.offset((p + 1i32) as isize)).b32.s1 == 1i32 {
        fetch(p + 1i32);
        q = new_native_character(cur_f, cur_c);
        g = real_get_native_glyph(
            &mut *mem.offset(q as isize) as *mut memory_word as *mut libc::c_void,
            0i32 as libc::c_uint,
        ) as scaled_t;
        s = get_ot_math_accent_pos(cur_f, g)
    } else if (*mem.offset((p + 1i32) as isize)).b32.s1 == 3i32 {
        r = (*mem.offset((p + 1i32) as isize)).b32.s0;
        if r != -0xfffffffi32 && (*mem.offset(r as isize)).b16.s1 as libc::c_int == 28i32 {
            s = compute_ot_math_accent_pos(r)
        } else {
            s = 0x7fffffffi32
        }
    } else {
        s = 0x7fffffffi32
    }
    return s;
}
unsafe extern "C" fn make_math_accent(mut q: int32_t) {
    let mut p: int32_t = 0;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut a: int32_t = 0;
    let mut c: int32_t = 0;
    let mut g: int32_t = 0;
    let mut f: internal_font_number = 0;
    let mut i: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut s: scaled_t = 0;
    let mut sa: scaled_t = 0;
    let mut h: scaled_t = 0;
    let mut delta: scaled_t = 0;
    let mut w: scaled_t = 0;
    let mut w2: scaled_t = 0;
    let mut ot_assembly_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    fetch(q + 4i32);
    x = -0xfffffffi32;
    ot_assembly_ptr = 0 as *mut libc::c_void;
    if *font_area.offset(cur_f as isize) as libc::c_uint == 0xffffu32
        || *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
    {
        c = cur_c;
        f = cur_f;
        if !((*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32
            || (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32 + 1i32)
        {
            s = compute_ot_math_accent_pos(q)
        } else {
            s = 0i32
        }
        x = clean_box(
            q + 1i32,
            (2i32 * (cur_style as libc::c_int / 2i32) + 1i32) as small_number,
        );
        w = (*mem.offset((x + 1i32) as isize)).b32.s1;
        h = (*mem.offset((x + 3i32) as isize)).b32.s1
    } else if cur_i.s3 as libc::c_int > 0i32 {
        i = cur_i;
        c = cur_c;
        f = cur_f;
        s = 0i32;
        if (*mem.offset((q + 1i32) as isize)).b32.s1 == 1i32 {
            fetch(q + 1i32);
            if cur_i.s1 as libc::c_int % 4i32 == 1i32 {
                a = *lig_kern_base.offset(cur_f as isize) + cur_i.s0 as libc::c_int;
                cur_i = (*font_info.offset(a as isize)).b16;
                if cur_i.s3 as libc::c_int > 128i32 {
                    a = ((*lig_kern_base.offset(cur_f as isize)
                        + 256i32 * cur_i.s1 as libc::c_int
                        + cur_i.s0 as libc::c_int) as libc::c_long
                        + 32768
                        - (256i32 * 128i32) as libc::c_long) as int32_t;
                    cur_i = (*font_info.offset(a as isize)).b16
                }
                loop {
                    if cur_i.s2 as libc::c_int == *skew_char.offset(cur_f as isize) {
                        if cur_i.s1 as libc::c_int >= 128i32 {
                            if cur_i.s3 as libc::c_int <= 128i32 {
                                s = (*font_info.offset(
                                    (*kern_base.offset(cur_f as isize)
                                        + 256i32 * cur_i.s1 as libc::c_int
                                        + cur_i.s0 as libc::c_int)
                                        as isize,
                                ))
                                .b32
                                .s1
                            }
                        }
                        break;
                    } else {
                        if cur_i.s3 as libc::c_int >= 128i32 {
                            break;
                        }
                        a = a + cur_i.s3 as libc::c_int + 1i32;
                        cur_i = (*font_info.offset(a as isize)).b16
                    }
                }
            }
        }
        x = clean_box(
            q + 1i32,
            (2i32 * (cur_style as libc::c_int / 2i32) + 1i32) as small_number,
        );
        w = (*mem.offset((x + 1i32) as isize)).b32.s1;
        h = (*mem.offset((x + 3i32) as isize)).b32.s1;
        while !(i.s1 as libc::c_int % 4i32 != 2i32) {
            y = i.s0 as int32_t;
            i = (*font_info.offset((*char_base.offset(f as isize) + y) as isize)).b16;
            if !(i.s3 as libc::c_int > 0i32) {
                break;
            }
            if (*font_info.offset((*width_base.offset(f as isize) + i.s3 as libc::c_int) as isize))
                .b32
                .s1
                > w
            {
                break;
            }
            c = y
        }
    }
    /*:767*/
    if x != -0xfffffffi32 {
        if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32
                || (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32 + 1i32
            {
                delta = 0i32
            } else if h < get_ot_math_constant(f, 6i32) {
                delta = h
            } else {
                delta = get_ot_math_constant(f, 6i32)
            }
        } else if h
            < (*font_info.offset((5i32 + *param_base.offset(f as isize)) as isize))
                .b32
                .s1
        {
            delta = h
        } else {
            delta = (*font_info.offset((5i32 + *param_base.offset(f as isize)) as isize))
                .b32
                .s1
        }
        if (*mem.offset((q + 2i32) as isize)).b32.s1 != 0i32
            || (*mem.offset((q + 3i32) as isize)).b32.s1 != 0i32
        {
            if (*mem.offset((q + 1i32) as isize)).b32.s1 == 1i32 {
                /*769: */
                flush_node_list(x);
                x = new_noad();
                *mem.offset((x + 1i32) as isize) = *mem.offset((q + 1i32) as isize);
                *mem.offset((x + 2i32) as isize) = *mem.offset((q + 2i32) as isize);
                *mem.offset((x + 3i32) as isize) = *mem.offset((q + 3i32) as isize);
                (*mem.offset((q + 2i32) as isize)).b32 = empty;
                (*mem.offset((q + 3i32) as isize)).b32 = empty;
                (*mem.offset((q + 1i32) as isize)).b32.s1 = 3i32;
                (*mem.offset((q + 1i32) as isize)).b32.s0 = x;
                x = clean_box(q + 1i32, cur_style);
                delta = delta + (*mem.offset((x + 3i32) as isize)).b32.s1 - h;
                h = (*mem.offset((x + 3i32) as isize)).b32.s1
            }
        }
        y = char_box(f, c);
        if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32
            || *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        {
            p = get_node(5i32);
            (*mem.offset(p as isize)).b16.s1 = 8i32 as u16;
            (*mem.offset(p as isize)).b16.s0 = 42i32 as u16;
            (*mem.offset((p + 4i32) as isize)).b16.s2 = f as u16;
            (*mem.offset((p + 4i32) as isize)).b16.s1 = real_get_native_glyph(
                &mut *mem.offset((*mem.offset((y + 5i32) as isize)).b32.s1 as isize)
                    as *mut memory_word as *mut libc::c_void,
                0i32 as libc::c_uint,
            );
            measure_native_glyph(
                &mut *mem.offset(p as isize) as *mut memory_word as *mut libc::c_void,
                1i32,
            );
            free_node(
                (*mem.offset((y + 5i32) as isize)).b32.s1,
                (*mem.offset(((*mem.offset((y + 5i32) as isize)).b32.s1 + 4i32) as isize))
                    .b16
                    .s3 as int32_t,
            );
            (*mem.offset((y + 5i32) as isize)).b32.s1 = p;
            if (*mem.offset(q as isize)).b16.s0 as libc::c_int & 1i32 != 0 {
                measure_native_glyph(
                    &mut *mem.offset(p as isize) as *mut memory_word as *mut libc::c_void,
                    1i32,
                );
            } else {
                c = (*mem.offset((p + 4i32) as isize)).b16.s1 as int32_t;
                a = 0i32;
                loop {
                    g = get_ot_math_variant(f, c, a, &mut w2, 1i32);
                    if w2 > 0i32 && w2 <= w {
                        (*mem.offset((p + 4i32) as isize)).b16.s1 = g as u16;
                        measure_native_glyph(
                            &mut *mem.offset(p as isize) as *mut memory_word as *mut libc::c_void,
                            1i32,
                        );
                        a += 1
                    }
                    if w2 < 0i32 || w2 >= w {
                        break;
                    }
                }
                if w2 < 0i32 {
                    ot_assembly_ptr = get_ot_assembly_ptr(f, c, 1i32);
                    if !ot_assembly_ptr.is_null() {
                        free_node(p, 5i32);
                        p = build_opentype_assembly(f, ot_assembly_ptr, w, 1i32 != 0);
                        (*mem.offset((y + 5i32) as isize)).b32.s1 = p
                    }
                } else {
                    measure_native_glyph(
                        &mut *mem.offset(p as isize) as *mut memory_word as *mut libc::c_void,
                        1i32,
                    );
                }
            }
            (*mem.offset((y + 1i32) as isize)).b32.s1 = (*mem.offset((p + 1i32) as isize)).b32.s1;
            (*mem.offset((y + 3i32) as isize)).b32.s1 = (*mem.offset((p + 3i32) as isize)).b32.s1;
            (*mem.offset((y + 2i32) as isize)).b32.s1 = (*mem.offset((p + 2i32) as isize)).b32.s1;
            if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32
                || (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32 + 1i32
            {
                if (*mem.offset((y + 3i32) as isize)).b32.s1 < 0i32 {
                    (*mem.offset((y + 3i32) as isize)).b32.s1 = 0i32
                }
            } else if (*mem.offset((y + 2i32) as isize)).b32.s1 < 0i32 {
                (*mem.offset((y + 2i32) as isize)).b32.s1 = 0i32
            }
            if p != -0xfffffffi32
                && !is_char_node(p)
                && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
                && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
            {
                sa = get_ot_math_accent_pos(
                    f,
                    (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                );
                if sa == 0x7fffffffi32 {
                    sa = half((*mem.offset((y + 1i32) as isize)).b32.s1)
                }
            } else {
                sa = half((*mem.offset((y + 1i32) as isize)).b32.s1)
            }
            if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32
                || (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32 + 1i32
                || s == 0x7fffffffi32
            {
                s = half(w)
            }
            (*mem.offset((y + 4i32) as isize)).b32.s1 = s - sa
        } else {
            (*mem.offset((y + 4i32) as isize)).b32.s1 =
                s + half(w - (*mem.offset((y + 1i32) as isize)).b32.s1)
        }
        (*mem.offset((y + 1i32) as isize)).b32.s1 = 0i32;
        if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32
            || (*mem.offset(q as isize)).b16.s0 as libc::c_int == 2i32 + 1i32
        {
            (*mem.offset(x as isize)).b32.s1 = y;
            y = vpackage(x, 0i32, 1i32 as small_number, 0x3fffffffi32);
            (*mem.offset((y + 4i32) as isize)).b32.s1 =
                -(h - (*mem.offset((y + 3i32) as isize)).b32.s1)
        } else {
            p = new_kern(-delta);
            (*mem.offset(p as isize)).b32.s1 = x;
            (*mem.offset(y as isize)).b32.s1 = p;
            y = vpackage(y, 0i32, 1i32 as small_number, 0x3fffffffi32);
            if (*mem.offset((y + 3i32) as isize)).b32.s1 < h {
                /*765: */
                p = new_kern(h - (*mem.offset((y + 3i32) as isize)).b32.s1); /*773:*/
                (*mem.offset(p as isize)).b32.s1 = (*mem.offset((y + 5i32) as isize)).b32.s1;
                (*mem.offset((y + 5i32) as isize)).b32.s1 = p;
                (*mem.offset((y + 3i32) as isize)).b32.s1 = h
            }
        }
        (*mem.offset((y + 1i32) as isize)).b32.s1 = (*mem.offset((x + 1i32) as isize)).b32.s1;
        (*mem.offset((q + 1i32) as isize)).b32.s0 = y;
        (*mem.offset((q + 1i32) as isize)).b32.s1 = 2i32
    }
    free_ot_assembly(ot_assembly_ptr as *mut GlyphAssembly);
}
unsafe extern "C" fn make_fraction(mut q: int32_t) {
    let mut p: int32_t = 0;
    let mut v: int32_t = 0;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut z: int32_t = 0;
    let mut delta: scaled_t = 0;
    let mut delta1: scaled_t = 0;
    let mut delta2: scaled_t = 0;
    let mut shift_up: scaled_t = 0;
    let mut shift_down: scaled_t = 0;
    let mut clr: scaled_t = 0;
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == 0x40000000i32 {
        (*mem.offset((q + 1i32) as isize)).b32.s1 = default_rule_thickness()
    }
    x = clean_box(
        q + 2i32,
        (cur_style as libc::c_int + 2i32 - 2i32 * (cur_style as libc::c_int / 6i32))
            as small_number,
    );
    z = clean_box(
        q + 3i32,
        (2i32 * (cur_style as libc::c_int / 2i32) + 3i32 - 2i32 * (cur_style as libc::c_int / 6i32))
            as small_number,
    );
    if (*mem.offset((x + 1i32) as isize)).b32.s1 < (*mem.offset((z + 1i32) as isize)).b32.s1 {
        x = rebox(x, (*mem.offset((z + 1i32) as isize)).b32.s1)
    } else {
        z = rebox(z, (*mem.offset((x + 1i32) as isize)).b32.s1)
    }
    if (cur_style as libc::c_int) < 2i32 {
        shift_up = num1(cur_size);
        shift_down = denom1(cur_size)
    } else {
        shift_down = denom2(cur_size);
        if (*mem.offset((q + 1i32) as isize)).b32.s1 != 0i32 {
            shift_up = num2(cur_size)
        } else {
            shift_up = num3(cur_size)
        }
    }
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == 0i32 {
        /*772:*/
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            if (cur_style as libc::c_int) < 2i32 {
                clr = get_ot_math_constant(cur_f, 27i32)
            } else {
                clr = get_ot_math_constant(cur_f, 26i32)
            }
        } else if (cur_style as libc::c_int) < 2i32 {
            clr = 7i32 * default_rule_thickness()
        } else {
            clr = 3i32 * default_rule_thickness()
        } /*:774*/
        delta = half(
            clr - (shift_up
                - (*mem.offset((x + 2i32) as isize)).b32.s1
                - ((*mem.offset((z + 3i32) as isize)).b32.s1 - shift_down)),
        );
        if delta > 0i32 {
            shift_up = shift_up + delta;
            shift_down = shift_down + delta
        }
    } else {
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            delta = half((*mem.offset((q + 1i32) as isize)).b32.s1);
            if (cur_style as libc::c_int) < 2i32 {
                clr = get_ot_math_constant(cur_f, 37i32)
            } else {
                clr = get_ot_math_constant(cur_f, 36i32)
            }
            delta1 = clr
                - (shift_up
                    - (*mem.offset((x + 2i32) as isize)).b32.s1
                    - (axis_height(cur_size) + delta));
            if (cur_style as libc::c_int) < 2i32 {
                clr = get_ot_math_constant(cur_f, 40i32)
            } else {
                clr = get_ot_math_constant(cur_f, 39i32)
            }
            delta2 = clr
                - (axis_height(cur_size)
                    - delta
                    - ((*mem.offset((z + 3i32) as isize)).b32.s1 - shift_down))
        } else {
            if (cur_style as libc::c_int) < 2i32 {
                clr = 3i32 * (*mem.offset((q + 1i32) as isize)).b32.s1
            } else {
                clr = (*mem.offset((q + 1i32) as isize)).b32.s1
            }
            delta = half((*mem.offset((q + 1i32) as isize)).b32.s1);
            delta1 = clr
                - (shift_up
                    - (*mem.offset((x + 2i32) as isize)).b32.s1
                    - (axis_height(cur_size) + delta));
            delta2 = clr
                - (axis_height(cur_size)
                    - delta
                    - ((*mem.offset((z + 3i32) as isize)).b32.s1 - shift_down))
        }
        if delta1 > 0i32 {
            shift_up = shift_up + delta1
        }
        if delta2 > 0i32 {
            shift_down = shift_down + delta2
        }
    }
    v = new_null_box();
    (*mem.offset(v as isize)).b16.s1 = 1i32 as u16;
    (*mem.offset((v + 3i32) as isize)).b32.s1 =
        shift_up + (*mem.offset((x + 3i32) as isize)).b32.s1;
    (*mem.offset((v + 2i32) as isize)).b32.s1 =
        (*mem.offset((z + 2i32) as isize)).b32.s1 + shift_down;
    (*mem.offset((v + 1i32) as isize)).b32.s1 = (*mem.offset((x + 1i32) as isize)).b32.s1;
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == 0i32 {
        p = new_kern(
            shift_up
                - (*mem.offset((x + 2i32) as isize)).b32.s1
                - ((*mem.offset((z + 3i32) as isize)).b32.s1 - shift_down),
        );
        (*mem.offset(p as isize)).b32.s1 = z
    } else {
        y = fraction_rule((*mem.offset((q + 1i32) as isize)).b32.s1);
        p = new_kern(
            axis_height(cur_size)
                - delta
                - ((*mem.offset((z + 3i32) as isize)).b32.s1 - shift_down),
        );
        (*mem.offset(y as isize)).b32.s1 = p;
        (*mem.offset(p as isize)).b32.s1 = z;
        p = new_kern(
            shift_up - (*mem.offset((x + 2i32) as isize)).b32.s1 - (axis_height(cur_size) + delta),
        );
        (*mem.offset(p as isize)).b32.s1 = y
    }
    (*mem.offset(x as isize)).b32.s1 = p;
    (*mem.offset((v + 5i32) as isize)).b32.s1 = x;
    if (cur_style as libc::c_int) < 2i32 {
        delta = delim1(cur_size)
    } else {
        delta = delim2(cur_size)
    }
    x = var_delimiter(q + 4i32, cur_size, delta);
    (*mem.offset(x as isize)).b32.s1 = v;
    z = var_delimiter(q + 5i32, cur_size, delta);
    (*mem.offset(v as isize)).b32.s1 = z;
    (*mem.offset((q + 1i32) as isize)).b32.s1 = hpack(x, 0i32, 1i32 as small_number);
    /*:775*/
}
unsafe extern "C" fn make_op(mut q: int32_t) -> scaled_t {
    let mut delta: scaled_t = 0;
    let mut p: int32_t = 0;
    let mut v: int32_t = 0;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut z: int32_t = 0;
    let mut c: u16 = 0;
    let mut i: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut shift_up: scaled_t = 0;
    let mut shift_down: scaled_t = 0;
    let mut h1: scaled_t = 0;
    let mut h2: scaled_t = 0;
    let mut n: int32_t = 0;
    let mut g: int32_t = 0;
    let mut ot_assembly_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut save_f: internal_font_number = 0;
    if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 0i32 && (cur_style as libc::c_int) < 2i32
    {
        (*mem.offset(q as isize)).b16.s0 = 1i32 as u16
    }
    delta = 0i32;
    ot_assembly_ptr = 0 as *mut libc::c_void;
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == 1i32 {
        fetch(q + 1i32);
        if !(*font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && usingOpenType(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0)
        {
            if (cur_style as libc::c_int) < 2i32 && cur_i.s1 as libc::c_int % 4i32 == 2i32 {
                c = cur_i.s0;
                i = (*font_info
                    .offset((*char_base.offset(cur_f as isize) + c as libc::c_int) as isize))
                .b16;
                if i.s3 as libc::c_int > 0i32 {
                    cur_c = c as int32_t;
                    cur_i = i;
                    (*mem.offset((q + 1i32) as isize)).b16.s0 = c
                }
            }
            delta = (*font_info.offset(
                (*italic_base.offset(cur_f as isize) + cur_i.s1 as libc::c_int / 4i32) as isize,
            ))
            .b32
            .s1
        }
        x = clean_box(q + 1i32, cur_style);
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            p = (*mem.offset((x + 5i32) as isize)).b32.s1;
            if p != -0xfffffffi32
                && !is_char_node(p)
                && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
                && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
            {
                let mut current_block_41: u64;
                if (cur_style as libc::c_int) < 2i32 {
                    h1 = get_ot_math_constant(cur_f, 3i32);
                    if (h1 as libc::c_double)
                        < (((*mem.offset((p + 3i32) as isize)).b32.s1
                            + (*mem.offset((p + 2i32) as isize)).b32.s1)
                            * 5i32) as libc::c_double
                            / 4i32 as libc::c_double
                    {
                        h1 = ((((*mem.offset((p + 3i32) as isize)).b32.s1
                            + (*mem.offset((p + 2i32) as isize)).b32.s1)
                            * 5i32) as libc::c_double
                            / 4i32 as libc::c_double) as scaled_t
                    }
                    c = (*mem.offset((p + 4i32) as isize)).b16.s1;
                    n = 0i32;
                    loop {
                        g = get_ot_math_variant(cur_f, c as libc::c_int, n, &mut h2, 0i32);
                        if h2 > 0i32 {
                            (*mem.offset((p + 4i32) as isize)).b16.s1 = g as u16;
                            measure_native_glyph(
                                &mut *mem.offset(p as isize) as *mut memory_word
                                    as *mut libc::c_void,
                                1i32,
                            );
                        }
                        n += 1;
                        if h2 < 0i32 || h2 >= h1 {
                            break;
                        }
                    }
                    if h2 < 0i32 {
                        ot_assembly_ptr = get_ot_assembly_ptr(cur_f, c as libc::c_int, 0i32);
                        if !ot_assembly_ptr.is_null() {
                            free_node(p, 5i32);
                            p = build_opentype_assembly(cur_f, ot_assembly_ptr, h1, 0i32 != 0);
                            (*mem.offset((x + 5i32) as isize)).b32.s1 = p;
                            delta = 0i32;
                            current_block_41 = 18116816373875863516;
                        } else {
                            current_block_41 = 6717214610478484138;
                        }
                    } else {
                        measure_native_glyph(
                            &mut *mem.offset(p as isize) as *mut memory_word as *mut libc::c_void,
                            1i32,
                        );
                        current_block_41 = 6717214610478484138;
                    }
                } else {
                    current_block_41 = 6717214610478484138;
                }
                match current_block_41 {
                    6717214610478484138 => {
                        delta = get_ot_math_ital_corr(
                            cur_f,
                            (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                        )
                    }
                    _ => {}
                }
                (*mem.offset((x + 1i32) as isize)).b32.s1 =
                    (*mem.offset((p + 1i32) as isize)).b32.s1;
                (*mem.offset((x + 3i32) as isize)).b32.s1 =
                    (*mem.offset((p + 3i32) as isize)).b32.s1;
                (*mem.offset((x + 2i32) as isize)).b32.s1 =
                    (*mem.offset((p + 2i32) as isize)).b32.s1
            }
        }
        if (*mem.offset((q + 3i32) as isize)).b32.s1 != 0i32
            && (*mem.offset(q as isize)).b16.s0 as libc::c_int != 1i32
        {
            (*mem.offset((x + 1i32) as isize)).b32.s1 =
                (*mem.offset((x + 1i32) as isize)).b32.s1 - delta
        }
        (*mem.offset((x + 4i32) as isize)).b32.s1 = half(
            (*mem.offset((x + 3i32) as isize)).b32.s1 - (*mem.offset((x + 2i32) as isize)).b32.s1,
        ) - axis_height(cur_size);
        (*mem.offset((q + 1i32) as isize)).b32.s1 = 2i32;
        (*mem.offset((q + 1i32) as isize)).b32.s0 = x
    }
    save_f = cur_f;
    if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 1i32 {
        /*777: */
        x = clean_box(
            q + 2i32,
            (2i32 * (cur_style as libc::c_int / 4i32) + 4i32 + cur_style as libc::c_int % 2i32)
                as small_number,
        );
        y = clean_box(q + 1i32, cur_style);
        z = clean_box(
            q + 3i32,
            (2i32 * (cur_style as libc::c_int / 4i32) + 5i32) as small_number,
        );
        v = new_null_box();
        (*mem.offset(v as isize)).b16.s1 = 1i32 as u16;
        (*mem.offset((v + 1i32) as isize)).b32.s1 = (*mem.offset((y + 1i32) as isize)).b32.s1;
        if (*mem.offset((x + 1i32) as isize)).b32.s1 > (*mem.offset((v + 1i32) as isize)).b32.s1 {
            (*mem.offset((v + 1i32) as isize)).b32.s1 = (*mem.offset((x + 1i32) as isize)).b32.s1
        }
        if (*mem.offset((z + 1i32) as isize)).b32.s1 > (*mem.offset((v + 1i32) as isize)).b32.s1 {
            (*mem.offset((v + 1i32) as isize)).b32.s1 = (*mem.offset((z + 1i32) as isize)).b32.s1
        }
        x = rebox(x, (*mem.offset((v + 1i32) as isize)).b32.s1);
        y = rebox(y, (*mem.offset((v + 1i32) as isize)).b32.s1);
        z = rebox(z, (*mem.offset((v + 1i32) as isize)).b32.s1);
        (*mem.offset((x + 4i32) as isize)).b32.s1 = half(delta);
        (*mem.offset((z + 4i32) as isize)).b32.s1 = -(*mem.offset((x + 4i32) as isize)).b32.s1;
        (*mem.offset((v + 3i32) as isize)).b32.s1 = (*mem.offset((y + 3i32) as isize)).b32.s1;
        (*mem.offset((v + 2i32) as isize)).b32.s1 = (*mem.offset((y + 2i32) as isize)).b32.s1;
        cur_f = save_f;
        if (*mem.offset((q + 2i32) as isize)).b32.s1 == 0i32 {
            free_node(x, 8i32);
            (*mem.offset((v + 5i32) as isize)).b32.s1 = y
        } else {
            shift_up = big_op_spacing3() - (*mem.offset((x + 2i32) as isize)).b32.s1;
            if shift_up < big_op_spacing1() {
                shift_up = big_op_spacing1()
            }
            p = new_kern(shift_up);
            (*mem.offset(p as isize)).b32.s1 = y;
            (*mem.offset(x as isize)).b32.s1 = p;
            p = new_kern(big_op_spacing5());
            (*mem.offset(p as isize)).b32.s1 = x;
            (*mem.offset((v + 5i32) as isize)).b32.s1 = p;
            (*mem.offset((v + 3i32) as isize)).b32.s1 = (*mem.offset((v + 3i32) as isize)).b32.s1
                + big_op_spacing5()
                + (*mem.offset((x + 3i32) as isize)).b32.s1
                + (*mem.offset((x + 2i32) as isize)).b32.s1
                + shift_up
        }
        if (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 {
            free_node(z, 8i32);
        } else {
            shift_down = big_op_spacing4() - (*mem.offset((z + 3i32) as isize)).b32.s1;
            if shift_down < big_op_spacing2() {
                shift_down = big_op_spacing2()
            }
            p = new_kern(shift_down);
            (*mem.offset(y as isize)).b32.s1 = p;
            (*mem.offset(p as isize)).b32.s1 = z;
            p = new_kern(big_op_spacing5());
            (*mem.offset(z as isize)).b32.s1 = p;
            (*mem.offset((v + 2i32) as isize)).b32.s1 = (*mem.offset((v + 2i32) as isize)).b32.s1
                + big_op_spacing5()
                + (*mem.offset((z + 3i32) as isize)).b32.s1
                + (*mem.offset((z + 2i32) as isize)).b32.s1
                + shift_down
        }
        (*mem.offset((q + 1i32) as isize)).b32.s1 = v
    }
    free_ot_assembly(ot_assembly_ptr as *mut GlyphAssembly);
    return delta;
}
unsafe extern "C" fn make_ord(mut q: int32_t) {
    let mut a: int32_t = 0;
    let mut p: int32_t = 0;
    let mut r: int32_t = 0;
    while (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 {
        if !((*mem.offset((q + 2i32) as isize)).b32.s1 == 0i32) {
            break;
        }
        if !((*mem.offset((q + 1i32) as isize)).b32.s1 == 1i32) {
            break;
        }
        p = (*mem.offset(q as isize)).b32.s1;
        if !(p != -0xfffffffi32) {
            break;
        }
        if !((*mem.offset(p as isize)).b16.s1 as libc::c_int >= 16i32
            && (*mem.offset(p as isize)).b16.s1 as libc::c_int <= 22i32)
        {
            break;
        }
        if !((*mem.offset((p + 1i32) as isize)).b32.s1 == 1i32) {
            break;
        }
        if !((*mem.offset((p + 1i32) as isize)).b16.s1 as libc::c_int % 256i32
            == (*mem.offset((q + 1i32) as isize)).b16.s1 as libc::c_int % 256i32)
        {
            break;
        }
        (*mem.offset((q + 1i32) as isize)).b32.s1 = 4i32;
        fetch(q + 1i32);
        if !(cur_i.s1 as libc::c_int % 4i32 == 1i32) {
            break;
        }
        a = *lig_kern_base.offset(cur_f as isize) + cur_i.s0 as libc::c_int;
        cur_c = (*mem.offset((p + 1i32) as isize)).b16.s0 as int32_t;
        cur_i = (*font_info.offset(a as isize)).b16;
        if cur_i.s3 as libc::c_int > 128i32 {
            a = ((*lig_kern_base.offset(cur_f as isize)
                + 256i32 * cur_i.s1 as libc::c_int
                + cur_i.s0 as libc::c_int) as libc::c_long
                + 32768
                - (256i32 * 128i32) as libc::c_long) as int32_t;
            cur_i = (*font_info.offset(a as isize)).b16
        }
        loop {
            if cur_i.s2 as libc::c_int == cur_c {
                if cur_i.s3 as libc::c_int <= 128i32 {
                    if cur_i.s1 as libc::c_int >= 128i32 {
                        p = new_kern(
                            (*font_info.offset(
                                (*kern_base.offset(cur_f as isize)
                                    + 256i32 * cur_i.s1 as libc::c_int
                                    + cur_i.s0 as libc::c_int)
                                    as isize,
                            ))
                            .b32
                            .s1,
                        );
                        (*mem.offset(p as isize)).b32.s1 = (*mem.offset(q as isize)).b32.s1;
                        (*mem.offset(q as isize)).b32.s1 = p;
                        return;
                    } else {
                        match cur_i.s1 as libc::c_int {
                            1 | 5 => (*mem.offset((q + 1i32) as isize)).b16.s0 = cur_i.s0,
                            2 | 6 => (*mem.offset((p + 1i32) as isize)).b16.s0 = cur_i.s0,
                            3 | 7 | 11 => {
                                r = new_noad();
                                (*mem.offset((r + 1i32) as isize)).b16.s0 = cur_i.s0;
                                (*mem.offset((r + 1i32) as isize)).b16.s1 =
                                    ((*mem.offset((q + 1i32) as isize)).b16.s1 as libc::c_int
                                        % 256i32) as u16;
                                (*mem.offset(q as isize)).b32.s1 = r;
                                (*mem.offset(r as isize)).b32.s1 = p;
                                if (cur_i.s1 as libc::c_int) < 11i32 {
                                    (*mem.offset((r + 1i32) as isize)).b32.s1 = 1i32
                                } else {
                                    (*mem.offset((r + 1i32) as isize)).b32.s1 = 4i32
                                }
                            }
                            _ => {
                                (*mem.offset(q as isize)).b32.s1 = (*mem.offset(p as isize)).b32.s1;
                                (*mem.offset((q + 1i32) as isize)).b16.s0 = cur_i.s0;
                                *mem.offset((q + 3i32) as isize) = *mem.offset((p + 3i32) as isize);
                                *mem.offset((q + 2i32) as isize) = *mem.offset((p + 2i32) as isize);
                                free_node(p, 4i32);
                            }
                        }
                        if cur_i.s1 as libc::c_int > 3i32 {
                            return;
                        }
                        (*mem.offset((q + 1i32) as isize)).b32.s1 = 1i32;
                        break;
                    }
                }
            }
            if cur_i.s3 as libc::c_int >= 128i32 {
                return;
            }
            a = a + cur_i.s3 as libc::c_int + 1i32;
            cur_i = (*font_info.offset(a as isize)).b16
        }
    }
}
unsafe extern "C" fn attach_hkern_to_new_hlist(mut q: int32_t, mut delta: scaled_t) -> int32_t {
    let mut y: int32_t = 0;
    let mut z: int32_t = 0;
    z = new_kern(delta);
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == -0xfffffffi32 {
        (*mem.offset((q + 1i32) as isize)).b32.s1 = z
    } else {
        y = (*mem.offset((q + 1i32) as isize)).b32.s1;
        while (*mem.offset(y as isize)).b32.s1 != -0xfffffffi32 {
            y = (*mem.offset(y as isize)).b32.s1
        }
        (*mem.offset(y as isize)).b32.s1 = z
    }
    return (*mem.offset((q + 1i32) as isize)).b32.s1;
}
unsafe extern "C" fn make_scripts(mut q: int32_t, mut delta: scaled_t) {
    let mut p: int32_t = 0;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut z: int32_t = 0;
    let mut shift_up: scaled_t = 0;
    let mut shift_down: scaled_t = 0;
    let mut clr: scaled_t = 0;
    let mut sub_kern: scaled_t = 0;
    let mut sup_kern: scaled_t = 0;
    let mut script_c: int32_t = 0;
    let mut script_g: u16 = 0;
    let mut script_f: internal_font_number = 0;
    let mut t: int32_t = 0;
    let mut save_f: internal_font_number = 0;
    p = (*mem.offset((q + 1i32) as isize)).b32.s1;
    script_c = -0xfffffffi32;
    script_g = 0i32 as u16;
    script_f = 0i32;
    sup_kern = 0i32;
    sub_kern = 0i32;
    if is_char_node(p) as libc::c_int != 0
        || p != -0xfffffffi32
            && !is_char_node(p)
            && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
            && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
    {
        shift_up = 0i32;
        shift_down = 0i32
    } else {
        z = hpack(p, 0i32, 1i32 as small_number);
        if (cur_style as libc::c_int) < 4i32 {
            t = 256i32
        } else {
            t = 2i32 * 256i32
        }
        shift_up = (*mem.offset((z + 3i32) as isize)).b32.s1 - sup_drop(t);
        shift_down = (*mem.offset((z + 2i32) as isize)).b32.s1 + sub_drop(t);
        free_node(z, 8i32);
    }
    if (*mem.offset((q + 2i32) as isize)).b32.s1 == 0i32 {
        /*784: */
        save_f = cur_f;
        x = clean_box(
            q + 3i32,
            (2i32 * (cur_style as libc::c_int / 4i32) + 5i32) as small_number,
        );
        cur_f = save_f;
        (*mem.offset((x + 1i32) as isize)).b32.s1 = (*mem.offset((x + 1i32) as isize)).b32.s1
            + (*eqtb.offset(
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
                    + 12i32) as isize,
            ))
            .b32
            .s1;
        if shift_down < sub1(cur_size) {
            shift_down = sub1(cur_size)
        }
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            clr = (*mem.offset((x + 3i32) as isize)).b32.s1 - get_ot_math_constant(cur_f, 9i32)
        } else {
            clr = (*mem.offset((x + 3i32) as isize)).b32.s1
                - abs(math_x_height(cur_size) * 4i32) / 5i32
        }
        if shift_down < clr {
            shift_down = clr
        }
        (*mem.offset((x + 4i32) as isize)).b32.s1 = shift_down;
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            /*787: */
            if (*mem.offset((q + 3i32) as isize)).b32.s1 == 1i32 {
                save_f = cur_f;
                fetch(q + 3i32);
                if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                    ) as libc::c_int
                        != 0
                {
                    script_c = new_native_character(cur_f, cur_c);
                    script_g = real_get_native_glyph(
                        &mut *mem.offset(script_c as isize) as *mut memory_word
                            as *mut libc::c_void,
                        0i32 as libc::c_uint,
                    );
                    script_f = cur_f
                } else {
                    script_g = 0i32 as u16;
                    script_f = 0i32
                }
                cur_f = save_f
            }
            if p != -0xfffffffi32
                && !is_char_node(p)
                && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
                && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
            {
                sub_kern = get_ot_math_kern(
                    (*mem.offset((p + 4i32) as isize)).b16.s2 as libc::c_int,
                    (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                    script_f,
                    script_g as libc::c_int,
                    1i32,
                    shift_down,
                )
            }
            if sub_kern != 0i32 {
                p = attach_hkern_to_new_hlist(q, sub_kern)
            }
        }
    } else {
        save_f = cur_f;
        x = clean_box(
            q + 2i32,
            (2i32 * (cur_style as libc::c_int / 4i32) + 4i32 + cur_style as libc::c_int % 2i32)
                as small_number,
        );
        cur_f = save_f;
        (*mem.offset((x + 1i32) as isize)).b32.s1 = (*mem.offset((x + 1i32) as isize)).b32.s1
            + (*eqtb.offset(
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
                    + 12i32) as isize,
            ))
            .b32
            .s1;
        if cur_style as libc::c_int & 1i32 != 0 {
            clr = sup3(cur_size)
        } else if (cur_style as libc::c_int) < 2i32 {
            clr = sup1(cur_size)
        } else {
            clr = sup2(cur_size)
        }
        if shift_up < clr {
            shift_up = clr
        }
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            clr = (*mem.offset((x + 2i32) as isize)).b32.s1 + get_ot_math_constant(cur_f, 13i32)
        } else {
            clr = (*mem.offset((x + 2i32) as isize)).b32.s1 + abs(math_x_height(cur_size)) / 4i32
        }
        if shift_up < clr {
            shift_up = clr
        }
        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
            && isOpenTypeMathFont(*font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0
        {
            /*788: */
            if (*mem.offset((q + 2i32) as isize)).b32.s1 == 1i32 {
                save_f = cur_f;
                fetch(q + 2i32);
                if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                    ) as libc::c_int
                        != 0
                {
                    script_c = new_native_character(cur_f, cur_c);
                    script_g = real_get_native_glyph(
                        &mut *mem.offset(script_c as isize) as *mut memory_word
                            as *mut libc::c_void,
                        0i32 as libc::c_uint,
                    );
                    script_f = cur_f
                } else {
                    script_g = 0i32 as u16;
                    script_f = 0i32
                }
                cur_f = save_f
            }
            if p != -0xfffffffi32
                && !is_char_node(p)
                && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
                && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
            {
                sup_kern = get_ot_math_kern(
                    (*mem.offset((p + 4i32) as isize)).b16.s2 as libc::c_int,
                    (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                    script_f,
                    script_g as libc::c_int,
                    0i32,
                    shift_up,
                )
            }
            if sup_kern != 0i32 && (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 {
                p = attach_hkern_to_new_hlist(q, sup_kern)
            }
        }
        if (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 {
            (*mem.offset((x + 4i32) as isize)).b32.s1 = -shift_up
        } else {
            /*786: */
            save_f = cur_f;
            y = clean_box(
                q + 3i32,
                (2i32 * (cur_style as libc::c_int / 4i32) + 5i32) as small_number,
            );
            cur_f = save_f;
            (*mem.offset((y + 1i32) as isize)).b32.s1 = (*mem.offset((y + 1i32) as isize)).b32.s1
                + (*eqtb.offset(
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
                        + 12i32) as isize,
                ))
                .b32
                .s1;
            if shift_down < sub2(cur_size) {
                shift_down = sub2(cur_size)
            }
            if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                ) as libc::c_int
                    != 0
            {
                clr = get_ot_math_constant(cur_f, 15i32)
                    - (shift_up
                        - (*mem.offset((x + 2i32) as isize)).b32.s1
                        - ((*mem.offset((y + 3i32) as isize)).b32.s1 - shift_down))
            } else {
                clr = 4i32 * default_rule_thickness()
                    - (shift_up
                        - (*mem.offset((x + 2i32) as isize)).b32.s1
                        - ((*mem.offset((y + 3i32) as isize)).b32.s1 - shift_down))
            }
            if clr > 0i32 {
                shift_down = shift_down + clr;
                if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                    && isOpenTypeMathFont(
                        *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                    ) as libc::c_int
                        != 0
                {
                    clr = get_ot_math_constant(cur_f, 16i32)
                        - (shift_up - (*mem.offset((x + 2i32) as isize)).b32.s1)
                } else {
                    clr = abs(math_x_height(cur_size) * 4i32) / 5i32
                        - (shift_up - (*mem.offset((x + 2i32) as isize)).b32.s1)
                }
                if clr > 0i32 {
                    shift_up = shift_up + clr;
                    shift_down = shift_down - clr
                }
            }
            if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                && isOpenTypeMathFont(
                    *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                ) as libc::c_int
                    != 0
            {
                if (*mem.offset((q + 3i32) as isize)).b32.s1 == 1i32 {
                    save_f = cur_f;
                    fetch(q + 3i32);
                    if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                        && isOpenTypeMathFont(
                            *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                        ) as libc::c_int
                            != 0
                    {
                        script_c = new_native_character(cur_f, cur_c);
                        script_g = real_get_native_glyph(
                            &mut *mem.offset(script_c as isize) as *mut memory_word
                                as *mut libc::c_void,
                            0i32 as libc::c_uint,
                        );
                        script_f = cur_f
                    } else {
                        script_g = 0i32 as u16;
                        script_f = 0i32
                    }
                    cur_f = save_f
                }
                if p != -0xfffffffi32
                    && !is_char_node(p)
                    && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
                    && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
                {
                    sub_kern = get_ot_math_kern(
                        (*mem.offset((p + 4i32) as isize)).b16.s2 as libc::c_int,
                        (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                        script_f,
                        script_g as libc::c_int,
                        1i32,
                        shift_down,
                    )
                }
                if sub_kern != 0i32 {
                    p = attach_hkern_to_new_hlist(q, sub_kern)
                }
                if (*mem.offset((q + 2i32) as isize)).b32.s1 == 1i32 {
                    save_f = cur_f;
                    fetch(q + 2i32);
                    if *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                        && isOpenTypeMathFont(
                            *font_layout_engine.offset(cur_f as isize) as XeTeXLayoutEngine
                        ) as libc::c_int
                            != 0
                    {
                        script_c = new_native_character(cur_f, cur_c);
                        script_g = real_get_native_glyph(
                            &mut *mem.offset(script_c as isize) as *mut memory_word
                                as *mut libc::c_void,
                            0i32 as libc::c_uint,
                        );
                        script_f = cur_f
                    } else {
                        script_g = 0i32 as u16;
                        script_f = 0i32
                    }
                    cur_f = save_f
                }
                if p != -0xfffffffi32
                    && !is_char_node(p)
                    && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32
                    && (*mem.offset(p as isize)).b16.s0 as libc::c_int == 42i32
                {
                    sup_kern = get_ot_math_kern(
                        (*mem.offset((p + 4i32) as isize)).b16.s2 as libc::c_int,
                        (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                        script_f,
                        script_g as libc::c_int,
                        0i32,
                        shift_up,
                    )
                }
                if sup_kern != 0i32 && (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 {
                    p = attach_hkern_to_new_hlist(q, sup_kern)
                }
            }
            (*mem.offset((x + 4i32) as isize)).b32.s1 = sup_kern + delta - sub_kern;
            p = new_kern(
                shift_up
                    - (*mem.offset((x + 2i32) as isize)).b32.s1
                    - ((*mem.offset((y + 3i32) as isize)).b32.s1 - shift_down),
            );
            (*mem.offset(x as isize)).b32.s1 = p;
            (*mem.offset(p as isize)).b32.s1 = y;
            x = vpackage(x, 0i32, 1i32 as small_number, 0x3fffffffi32);
            (*mem.offset((x + 4i32) as isize)).b32.s1 = shift_down
        }
    }
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == -0xfffffffi32 {
        (*mem.offset((q + 1i32) as isize)).b32.s1 = x
    } else {
        p = (*mem.offset((q + 1i32) as isize)).b32.s1;
        while (*mem.offset(p as isize)).b32.s1 != -0xfffffffi32 {
            p = (*mem.offset(p as isize)).b32.s1
        }
        (*mem.offset(p as isize)).b32.s1 = x
    };
}
unsafe extern "C" fn make_left_right(
    mut q: int32_t,
    mut style: small_number,
    mut max_d: scaled_t,
    mut max_h: scaled_t,
) -> small_number {
    let mut delta: scaled_t = 0;
    let mut delta1: scaled_t = 0;
    let mut delta2: scaled_t = 0;
    cur_style = style;
    if (cur_style as libc::c_int) < 4i32 {
        cur_size = 0i32
    } else {
        cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
    }
    cur_mu = x_over_n(math_quad(cur_size), 18i32);
    delta2 = max_d + axis_height(cur_size);
    delta1 = max_h + max_d - delta2;
    if delta2 > delta1 {
        delta1 = delta2
    }
    delta = delta1 / 500i32
        * (*eqtb.offset(
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
                + 18i32) as isize,
        ))
        .b32
        .s1;
    delta2 = delta1 + delta1
        - (*eqtb.offset(
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
                + 10i32) as isize,
        ))
        .b32
        .s1;
    if delta < delta2 {
        delta = delta2
    }
    (*mem.offset((q + 1i32) as isize)).b32.s1 = var_delimiter(q + 1i32, cur_size, delta);
    return ((*mem.offset(q as isize)).b16.s1 as libc::c_int - (30i32 - 20i32)) as small_number;
}
unsafe extern "C" fn mlist_to_hlist() {
    let mut current_block: u64;
    let mut mlist: int32_t = 0;
    let mut penalties: bool = false;
    let mut style: small_number = 0;
    let mut save_style: small_number = 0;
    let mut q: int32_t = 0;
    let mut r: int32_t = 0;
    let mut r_type: small_number = 0;
    let mut t: small_number = 0;
    let mut p: int32_t = -0xfffffffi32;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut z: int32_t = 0;
    let mut pen: int32_t = 0;
    let mut s: small_number = 0;
    let mut max_h: scaled_t = 0;
    let mut max_d: scaled_t = 0;
    let mut delta: scaled_t = 0;
    mlist = cur_mlist;
    penalties = mlist_penalties;
    style = cur_style;
    q = mlist;
    r = -0xfffffffi32;
    r_type = 17i32 as small_number;
    max_h = 0i32;
    max_d = 0i32;
    if (cur_style as libc::c_int) < 4i32 {
        cur_size = 0i32
    } else {
        cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
    }
    cur_mu = x_over_n(math_quad(cur_size), 18i32);
    while q != -0xfffffffi32 {
        loop
        /*753: */
        {
            delta = 0i32; /*:755 */
            match (*mem.offset(q as isize)).b16.s1 as libc::c_int {
                18 => {
                    match r_type as libc::c_int {
                        18 | 17 | 19 | 20 | 22 | 30 => {}
                        _ => {
                            current_block = 1677945370889843322;
                            break;
                        }
                    }
                    (*mem.offset(q as isize)).b16.s1 = 16i32 as u16
                }
                19 | 21 | 22 | 31 => {
                    if r_type as libc::c_int == 18i32 {
                        (*mem.offset(r as isize)).b16.s1 = 16i32 as u16
                    }
                    if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 31i32 {
                        current_block = 2476306051584715158;
                        break;
                    } else {
                        current_block = 1677945370889843322;
                        break;
                    }
                }
                30 => {
                    current_block = 2476306051584715158;
                    break;
                }
                25 => {
                    make_fraction(q);
                    current_block = 454865348394072936;
                    break;
                }
                17 => {
                    delta = make_op(q);
                    if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 1i32 {
                        current_block = 454865348394072936;
                        break;
                    } else {
                        current_block = 1677945370889843322;
                        break;
                    }
                }
                16 => {
                    make_ord(q);
                    current_block = 1677945370889843322;
                    break;
                }
                20 | 23 => {
                    current_block = 1677945370889843322;
                    break;
                }
                24 => {
                    make_radical(q);
                    current_block = 1677945370889843322;
                    break;
                }
                27 => {
                    make_over(q);
                    current_block = 1677945370889843322;
                    break;
                }
                26 => {
                    make_under(q);
                    current_block = 1677945370889843322;
                    break;
                }
                28 => {
                    make_math_accent(q);
                    current_block = 1677945370889843322;
                    break;
                }
                29 => {
                    make_vcenter(q);
                    current_block = 1677945370889843322;
                    break;
                }
                14 => {
                    cur_style = (*mem.offset(q as isize)).b16.s0 as small_number;
                    if (cur_style as libc::c_int) < 4i32 {
                        cur_size = 0i32
                    } else {
                        cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
                    }
                    cur_mu = x_over_n(math_quad(cur_size), 18i32);
                    current_block = 12027452349022962373;
                    break;
                }
                15 => {
                    match cur_style as libc::c_int / 2i32 {
                        0 => {
                            p = (*mem.offset((q + 1i32) as isize)).b32.s0;
                            (*mem.offset((q + 1i32) as isize)).b32.s0 = -0xfffffffi32
                        }
                        1 => {
                            p = (*mem.offset((q + 1i32) as isize)).b32.s1;
                            (*mem.offset((q + 1i32) as isize)).b32.s1 = -0xfffffffi32
                        }
                        2 => {
                            p = (*mem.offset((q + 2i32) as isize)).b32.s0;
                            (*mem.offset((q + 2i32) as isize)).b32.s0 = -0xfffffffi32
                        }
                        3 => {
                            p = (*mem.offset((q + 2i32) as isize)).b32.s1;
                            (*mem.offset((q + 2i32) as isize)).b32.s1 = -0xfffffffi32
                        }
                        _ => {}
                    }
                    flush_node_list((*mem.offset((q + 1i32) as isize)).b32.s0);
                    flush_node_list((*mem.offset((q + 1i32) as isize)).b32.s1);
                    flush_node_list((*mem.offset((q + 2i32) as isize)).b32.s0);
                    flush_node_list((*mem.offset((q + 2i32) as isize)).b32.s1);
                    (*mem.offset(q as isize)).b16.s1 = 14i32 as u16;
                    (*mem.offset(q as isize)).b16.s0 = cur_style as u16;
                    (*mem.offset((q + 1i32) as isize)).b32.s1 = 0i32;
                    (*mem.offset((q + 2i32) as isize)).b32.s1 = 0i32;
                    if p != -0xfffffffi32 {
                        z = (*mem.offset(q as isize)).b32.s1;
                        (*mem.offset(q as isize)).b32.s1 = p;
                        while (*mem.offset(p as isize)).b32.s1 != -0xfffffffi32 {
                            p = (*mem.offset(p as isize)).b32.s1
                        }
                        (*mem.offset(p as isize)).b32.s1 = z
                    }
                    current_block = 12027452349022962373;
                    break;
                }
                3 | 4 | 5 | 8 | 12 | 7 => {
                    current_block = 12027452349022962373;
                    break;
                }
                2 => {
                    if (*mem.offset((q + 3i32) as isize)).b32.s1 > max_h {
                        max_h = (*mem.offset((q + 3i32) as isize)).b32.s1
                    }
                    if (*mem.offset((q + 2i32) as isize)).b32.s1 > max_d {
                        max_d = (*mem.offset((q + 2i32) as isize)).b32.s1
                    }
                    current_block = 12027452349022962373;
                    break;
                }
                10 => {
                    if (*mem.offset(q as isize)).b16.s0 as libc::c_int == 99i32 {
                        x = (*mem.offset((q + 1i32) as isize)).b32.s0;
                        y = math_glue(x, cur_mu);
                        delete_glue_ref(x);
                        (*mem.offset((q + 1i32) as isize)).b32.s0 = y;
                        (*mem.offset(q as isize)).b16.s0 = 0i32 as u16
                    } else if cur_size != 0i32
                        && (*mem.offset(q as isize)).b16.s0 as libc::c_int == 98i32
                    {
                        p = (*mem.offset(q as isize)).b32.s1;
                        if p != -0xfffffffi32 {
                            if (*mem.offset(p as isize)).b16.s1 as libc::c_int == 10i32
                                || (*mem.offset(p as isize)).b16.s1 as libc::c_int == 11i32
                            {
                                (*mem.offset(q as isize)).b32.s1 = (*mem.offset(p as isize)).b32.s1;
                                (*mem.offset(p as isize)).b32.s1 = -0xfffffffi32;
                                flush_node_list(p);
                            }
                        }
                    }
                    current_block = 12027452349022962373;
                    break;
                }
                11 => {
                    math_kern(q, cur_mu);
                    current_block = 12027452349022962373;
                    break;
                }
                _ => {
                    confusion(b"mlist1\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
        match current_block {
            1677945370889843322 => {
                match (*mem.offset((q + 1i32) as isize)).b32.s1 {
                    1 | 4 => {
                        fetch(q + 1i32);
                        if *font_area.offset(cur_f as isize) as libc::c_uint == 0xffffu32
                            || *font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                        {
                            z = new_native_character(cur_f, cur_c);
                            p = get_node(5i32);
                            (*mem.offset(p as isize)).b16.s1 = 8i32 as u16;
                            (*mem.offset(p as isize)).b16.s0 = 42i32 as u16;
                            (*mem.offset((p + 4i32) as isize)).b16.s2 = cur_f as u16;
                            (*mem.offset((p + 4i32) as isize)).b16.s1 = real_get_native_glyph(
                                &mut *mem.offset(z as isize) as *mut memory_word
                                    as *mut libc::c_void,
                                0i32 as libc::c_uint,
                            );
                            measure_native_glyph(
                                &mut *mem.offset(p as isize) as *mut memory_word
                                    as *mut libc::c_void,
                                1i32,
                            );
                            free_node(z, (*mem.offset((z + 4i32) as isize)).b16.s3 as int32_t);
                            delta = get_ot_math_ital_corr(
                                cur_f,
                                (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int,
                            );
                            if (*mem.offset((q + 1i32) as isize)).b32.s1 == 4i32
                                && !(*font_area.offset(cur_f as isize) as libc::c_uint == 0xfffeu32
                                    && isOpenTypeMathFont(
                                        *font_layout_engine.offset(cur_f as isize)
                                            as XeTeXLayoutEngine,
                                    ) as libc::c_int
                                        != 0) as libc::c_int
                                    != 0i32
                            {
                                delta = 0i32
                            }
                            if (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 && delta != 0i32 {
                                (*mem.offset(p as isize)).b32.s1 = new_kern(delta);
                                delta = 0i32
                            }
                        } else if cur_i.s3 as libc::c_int > 0i32 {
                            delta = (*font_info.offset(
                                (*italic_base.offset(cur_f as isize)
                                    + cur_i.s1 as libc::c_int / 4i32)
                                    as isize,
                            ))
                            .b32
                            .s1;
                            p = new_character(cur_f, cur_c as UTF16_code);
                            if (*mem.offset((q + 1i32) as isize)).b32.s1 == 4i32
                                && (*font_info
                                    .offset((2i32 + *param_base.offset(cur_f as isize)) as isize))
                                .b32
                                .s1 != 0i32
                            {
                                delta = 0i32
                            }
                            if (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32 && delta != 0i32 {
                                (*mem.offset(p as isize)).b32.s1 = new_kern(delta);
                                delta = 0i32
                            }
                        } else {
                            p = -0xfffffffi32
                        }
                    }
                    0 => p = -0xfffffffi32,
                    2 => p = (*mem.offset((q + 1i32) as isize)).b32.s0,
                    3 => {
                        cur_mlist = (*mem.offset((q + 1i32) as isize)).b32.s0;
                        save_style = cur_style;
                        mlist_penalties = 0i32 != 0;
                        mlist_to_hlist();
                        cur_style = save_style;
                        if (cur_style as libc::c_int) < 4i32 {
                            cur_size = 0i32
                        } else {
                            cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
                        }
                        cur_mu = x_over_n(math_quad(cur_size), 18i32);
                        p = hpack(
                            (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1,
                            0i32,
                            1i32 as small_number,
                        )
                    }
                    _ => {
                        confusion(b"mlist2\x00" as *const u8 as *const libc::c_char);
                    }
                }
                (*mem.offset((q + 1i32) as isize)).b32.s1 = p;
                if (*mem.offset((q + 3i32) as isize)).b32.s1 == 0i32
                    && (*mem.offset((q + 2i32) as isize)).b32.s1 == 0i32
                {
                    current_block = 454865348394072936;
                } else {
                    make_scripts(q, delta);
                    current_block = 454865348394072936;
                }
            }
            _ => {}
        }
        match current_block {
            454865348394072936 => {
                /*check_dimensions */
                z = hpack(
                    (*mem.offset((q + 1i32) as isize)).b32.s1,
                    0i32,
                    1i32 as small_number,
                );
                if (*mem.offset((z + 3i32) as isize)).b32.s1 > max_h {
                    max_h = (*mem.offset((z + 3i32) as isize)).b32.s1
                }
                if (*mem.offset((z + 2i32) as isize)).b32.s1 > max_d {
                    max_d = (*mem.offset((z + 2i32) as isize)).b32.s1
                }
                free_node(z, 8i32);
                current_block = 2476306051584715158;
            }
            _ => {}
        }
        match current_block {
            2476306051584715158 => {
                /*done_with_noad */
                r = q;
                r_type = (*mem.offset(r as isize)).b16.s1 as small_number;
                if r_type as libc::c_int == 31i32 {
                    r_type = 30i32 as small_number;
                    cur_style = style;
                    if (cur_style as libc::c_int) < 4i32 {
                        cur_size = 0i32
                    } else {
                        cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
                    }
                    cur_mu = x_over_n(math_quad(cur_size), 18i32)
                }
            }
            _ => {}
        }
        /*done_with_node */
        q = (*mem.offset(q as isize)).b32.s1
    } /*ord_noad *//*:755 */
    if r_type as libc::c_int == 18i32 {
        (*mem.offset(r as isize)).b16.s1 = 16i32 as u16
    }
    p = 4999999i32 - 3i32;
    (*mem.offset(p as isize)).b32.s1 = -0xfffffffi32;
    q = mlist;
    r_type = 0i32 as small_number;
    cur_style = style;
    if (cur_style as libc::c_int) < 4i32 {
        cur_size = 0i32
    } else {
        cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
    }
    cur_mu = x_over_n(math_quad(cur_size), 18i32);
    while q != -0xfffffffi32 {
        let mut current_block_236: u64;
        t = 16i32 as small_number;
        s = 4i32 as small_number;
        pen = 10000i32;
        match (*mem.offset(q as isize)).b16.s1 as libc::c_int {
            17 | 20 | 21 | 22 | 23 => {
                t = (*mem.offset(q as isize)).b16.s1 as small_number;
                current_block_236 = 15067367080042895309;
            }
            18 => {
                t = 18i32 as small_number;
                pen = (*eqtb.offset(
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
                        + 9i32) as isize,
                ))
                .b32
                .s1;
                current_block_236 = 15067367080042895309;
            }
            19 => {
                t = 19i32 as small_number;
                pen = (*eqtb.offset(
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
                        + 10i32) as isize,
                ))
                .b32
                .s1;
                current_block_236 = 15067367080042895309;
            }
            16 | 29 | 27 | 26 => {
                current_block_236 = 15067367080042895309;
            }
            24 => {
                s = 5i32 as small_number;
                current_block_236 = 15067367080042895309;
            }
            28 => {
                s = 5i32 as small_number;
                current_block_236 = 15067367080042895309;
            }
            25 => {
                t = 23i32 as small_number;
                s = 6i32 as small_number;
                current_block_236 = 15067367080042895309;
            }
            30 | 31 => {
                t = make_left_right(q, style, max_d, max_h);
                current_block_236 = 15067367080042895309;
            }
            14 => {
                cur_style = (*mem.offset(q as isize)).b16.s0 as small_number;
                s = 3i32 as small_number;
                if (cur_style as libc::c_int) < 4i32 {
                    cur_size = 0i32
                } else {
                    cur_size = 256i32 * ((cur_style as libc::c_int - 2i32) / 2i32)
                }
                cur_mu = x_over_n(math_quad(cur_size), 18i32);
                current_block_236 = 11920828421623439930;
            }
            8 | 12 | 2 | 7 | 5 | 3 | 4 | 10 | 11 => {
                (*mem.offset(p as isize)).b32.s1 = q;
                p = q;
                q = (*mem.offset(q as isize)).b32.s1;
                (*mem.offset(p as isize)).b32.s1 = -0xfffffffi32;
                current_block_236 = 7344615536999694015;
            }
            _ => {
                confusion(b"mlist3\x00" as *const u8 as *const libc::c_char);
            }
        }
        match current_block_236 {
            15067367080042895309 => {
                if r_type as libc::c_int > 0i32 {
                    let mut offset_table: [*const libc::c_char; 8] = [
                        b"02340001\x00" as *const u8 as *const libc::c_char,
                        b"22*40001\x00" as *const u8 as *const libc::c_char,
                        b"33**3**3\x00" as *const u8 as *const libc::c_char,
                        b"44*04004\x00" as *const u8 as *const libc::c_char,
                        b"00*00000\x00" as *const u8 as *const libc::c_char,
                        b"02340001\x00" as *const u8 as *const libc::c_char,
                        b"11*11111\x00" as *const u8 as *const libc::c_char,
                        b"12341011\x00" as *const u8 as *const libc::c_char,
                    ];
                    // The inter-element spacing in math formulas depends on a 8x8 table.
                    // The table indices range from ORD_NOAD to INNER_NOAD.
                    // The chars of this table have the following significance:
                    match *offset_table[(r_type as libc::c_int - 16i32) as usize]
                        .offset((t as libc::c_int - 16i32) as isize)
                        as libc::c_int
                    {
                        48 => {
                            // no space
                            x = 0i32
                        }
                        49 => {
                            // a conditional thin space
                            if (cur_style as libc::c_int) < 4i32 {
                                x = 16i32
                            } else {
                                x = 0i32
                            }
                        }
                        50 => {
                            // a thin space
                            x = 16i32
                        }
                        51 => {
                            // a conditional medium space
                            if (cur_style as libc::c_int) < 4i32 {
                                x = 17i32
                            } else {
                                x = 0i32
                            }
                        }
                        52 => {
                            // a conditional thick space
                            if (cur_style as libc::c_int) < 4i32 {
                                x = 18i32
                            } else {
                                x = 0i32
                            }
                        }
                        _ => {
                            // impossible
                            confusion(b"mlist4\x00" as *const u8 as *const libc::c_char);
                        }
                    }
                    if x != 0i32 {
                        y = math_glue(
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
                                    + x) as isize,
                            ))
                            .b32
                            .s1,
                            cur_mu,
                        );
                        z = new_glue(y);
                        (*mem.offset(y as isize)).b32.s1 = -0xfffffffi32;
                        (*mem.offset(p as isize)).b32.s1 = z;
                        p = z;
                        (*mem.offset(z as isize)).b16.s0 = (x + 1i32) as u16
                    }
                }
                if (*mem.offset((q + 1i32) as isize)).b32.s1 != -0xfffffffi32 {
                    (*mem.offset(p as isize)).b32.s1 = (*mem.offset((q + 1i32) as isize)).b32.s1;
                    loop {
                        p = (*mem.offset(p as isize)).b32.s1;
                        if (*mem.offset(p as isize)).b32.s1 == -0xfffffffi32 {
                            break;
                        }
                    }
                }
                if penalties {
                    if (*mem.offset(q as isize)).b32.s1 != -0xfffffffi32 {
                        if pen < 10000i32 {
                            r_type = (*mem.offset((*mem.offset(q as isize)).b32.s1 as isize))
                                .b16
                                .s1 as small_number;
                            if r_type as libc::c_int != 12i32 {
                                if r_type as libc::c_int != 19i32 {
                                    z = new_penalty(pen);
                                    (*mem.offset(p as isize)).b32.s1 = z;
                                    p = z
                                }
                            }
                        }
                    }
                }
                if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 31i32 {
                    t = 20i32 as small_number
                }
                r_type = t;
                current_block_236 = 11920828421623439930;
            }
            _ => {}
        }
        match current_block_236 {
            11920828421623439930 => {
                /*delete_q */
                r = q;
                q = (*mem.offset(q as isize)).b32.s1;
                free_node(r, s as int32_t);
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn var_delimiter(mut d: int32_t, mut s: int32_t, mut v: scaled_t) -> int32_t {
    let mut b: int32_t = 0;
    let mut ot_assembly_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut f: internal_font_number = 0;
    let mut g: internal_font_number = 0;
    let mut c: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut m: int32_t = 0;
    let mut n: int32_t = 0;
    let mut u: scaled_t = 0;
    let mut w: scaled_t = 0;
    let mut q: b16x4 = {
        let mut init = b16x4_le_t {
            s0: 0i32 as u16,
            s1: 0i32 as u16,
            s2: 0i32 as u16,
            s3: 0i32 as u16,
        };
        init
    };
    let mut r: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut z: int32_t = 0;
    let mut large_attempt: bool = false;
    f = 0i32;
    w = 0i32;
    large_attempt = 0i32 != 0;
    z = (*mem.offset(d as isize)).b16.s3 as libc::c_int % 256i32;
    x = ((*mem.offset(d as isize)).b16.s2 as libc::c_long
        + ((*mem.offset(d as isize)).b16.s3 as libc::c_int / 256i32) as libc::c_long * 65536)
        as u16;
    ot_assembly_ptr = 0 as *mut libc::c_void;
    's_62: loop {
        if z != 0i32 || x as libc::c_int != 0i32 {
            z = z + s + 256i32;
            loop {
                z = z - 256i32;
                g = (*eqtb.offset(
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
                        + z) as isize,
                ))
                .b32
                .s1;
                if g != 0i32 {
                    /*734: */
                    if *font_area.offset(g as isize) as libc::c_uint == 0xfffeu32
                        && usingOpenType(*font_layout_engine.offset(g as isize) as XeTeXLayoutEngine)
                            as libc::c_int
                            != 0
                    {
                        x = map_char_to_glyph(g, x as int32_t) as u16;
                        f = g;
                        c = x;
                        w = 0i32;
                        n = 0i32;
                        loop {
                            y = get_ot_math_variant(g, x as libc::c_int, n, &mut u, 0i32)
                                as u16;
                            if u > w {
                                c = y;
                                w = u;
                                if u >= v {
                                    break 's_62;
                                }
                            }
                            n = n + 1i32;
                            if u < 0i32 {
                                break;
                            }
                        }
                        ot_assembly_ptr = get_ot_assembly_ptr(g, x as libc::c_int, 0i32);
                        if !ot_assembly_ptr.is_null() {
                            break 's_62;
                        }
                    } else {
                        y = x;
                        if y as libc::c_int >= *font_bc.offset(g as isize) as libc::c_int
                            && y as libc::c_int <= *font_ec.offset(g as isize) as libc::c_int
                        {
                            loop {
                                q = (*font_info.offset(
                                    (*char_base.offset(g as isize) + y as libc::c_int) as isize,
                                ))
                                .b16;
                                if !(q.s3 as libc::c_int > 0i32) {
                                    break;
                                }
                                if q.s1 as libc::c_int % 4i32 == 3i32 {
                                    f = g;
                                    c = y;
                                    break 's_62;
                                } else {
                                    u = (*font_info.offset(
                                        (*height_base.offset(g as isize)
                                            + q.s2 as libc::c_int / 16i32)
                                            as isize,
                                    ))
                                    .b32
                                    .s1 + (*font_info.offset(
                                        (*depth_base.offset(g as isize)
                                            + q.s2 as libc::c_int % 16i32)
                                            as isize,
                                    ))
                                    .b32
                                    .s1;
                                    if u > w {
                                        f = g;
                                        c = y;
                                        w = u;
                                        if u >= v {
                                            break 's_62;
                                        }
                                    }
                                    if !(q.s1 as libc::c_int % 4i32 == 2i32) {
                                        break;
                                    }
                                    y = q.s0
                                }
                            }
                        }
                    }
                }
                if z < 256i32 {
                    break;
                }
            }
        }
        if large_attempt {
            break;
        }
        large_attempt = 1i32 != 0;
        z = (*mem.offset(d as isize)).b16.s1 as libc::c_int % 256i32;
        x = ((*mem.offset(d as isize)).b16.s0 as libc::c_long
            + ((*mem.offset(d as isize)).b16.s1 as libc::c_int / 256i32) as libc::c_long * 65536)
            as u16
    }
    if f != 0i32 {
        if !(*font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
            && usingOpenType(*font_layout_engine.offset(f as isize) as XeTeXLayoutEngine)
                as libc::c_int
                != 0)
        {
            /*736: */
            if q.s1 as libc::c_int % 4i32 == 3i32 {
                /*739: */
                b = new_null_box();
                (*mem.offset(b as isize)).b16.s1 = 1i32 as u16;
                r = (*font_info
                    .offset((*exten_base.offset(f as isize) + q.s0 as libc::c_int) as isize))
                .b16;
                c = r.s0;
                u = height_plus_depth(f, c);
                w = 0i32;
                q = (*font_info.offset(
                    (*char_base.offset(f as isize) + effective_char(1i32 != 0, f, c)) as isize,
                ))
                .b16;
                (*mem.offset((b + 1i32) as isize)).b32.s1 = (*font_info
                    .offset((*width_base.offset(f as isize) + q.s3 as libc::c_int) as isize))
                .b32
                .s1 + (*font_info.offset(
                    (*italic_base.offset(f as isize) + q.s1 as libc::c_int / 4i32) as isize,
                ))
                .b32
                .s1;
                c = r.s1;
                if c as libc::c_int != 0i32 {
                    w = w + height_plus_depth(f, c)
                }
                c = r.s2;
                if c as libc::c_int != 0i32 {
                    w = w + height_plus_depth(f, c)
                }
                c = r.s3;
                if c as libc::c_int != 0i32 {
                    w = w + height_plus_depth(f, c)
                }
                n = 0i32;
                if u > 0i32 {
                    while w < v {
                        w = w + u;
                        n += 1;
                        if r.s2 as libc::c_int != 0i32 {
                            w = w + u
                        }
                    }
                }
                c = r.s1;
                if c as libc::c_int != 0i32 {
                    stack_into_box(b, f, c);
                }
                c = r.s0;
                let mut for_end: int32_t = 0;
                m = 1i32;
                for_end = n;
                if m <= for_end {
                    loop {
                        stack_into_box(b, f, c);
                        let fresh1 = m;
                        m = m + 1;
                        if !(fresh1 < for_end) {
                            break;
                        }
                    }
                }
                c = r.s2;
                if c as libc::c_int != 0i32 {
                    stack_into_box(b, f, c);
                    c = r.s0;
                    let mut for_end_0: int32_t = 0;
                    m = 1i32;
                    for_end_0 = n;
                    if m <= for_end_0 {
                        loop {
                            stack_into_box(b, f, c);
                            let fresh2 = m;
                            m = m + 1;
                            if !(fresh2 < for_end_0) {
                                break;
                            }
                        }
                    }
                }
                c = r.s3;
                if c as libc::c_int != 0i32 {
                    stack_into_box(b, f, c);
                }
                (*mem.offset((b + 2i32) as isize)).b32.s1 =
                    w - (*mem.offset((b + 3i32) as isize)).b32.s1
            } else {
                b = char_box(f, c as int32_t)
            }
        /*:736 */
        } else if !ot_assembly_ptr.is_null() {
            b = build_opentype_assembly(f, ot_assembly_ptr, v, 0i32 != 0)
        } else {
            b = new_null_box();
            (*mem.offset(b as isize)).b16.s1 = 1i32 as u16;
            (*mem.offset((b + 5i32) as isize)).b32.s1 = get_node(5i32);
            (*mem.offset((*mem.offset((b + 5i32) as isize)).b32.s1 as isize))
                .b16
                .s1 = 8i32 as u16;
            (*mem.offset((*mem.offset((b + 5i32) as isize)).b32.s1 as isize))
                .b16
                .s0 = 42i32 as u16;
            (*mem.offset(((*mem.offset((b + 5i32) as isize)).b32.s1 + 4i32) as isize))
                .b16
                .s2 = f as u16;
            (*mem.offset(((*mem.offset((b + 5i32) as isize)).b32.s1 + 4i32) as isize))
                .b16
                .s1 = c;
            measure_native_glyph(
                &mut *mem.offset((*mem.offset((b + 5i32) as isize)).b32.s1 as isize)
                    as *mut memory_word as *mut libc::c_void,
                1i32,
            );
            (*mem.offset((b + 1i32) as isize)).b32.s1 = (*mem
                .offset(((*mem.offset((b + 5i32) as isize)).b32.s1 + 1i32) as isize))
            .b32
            .s1;
            (*mem.offset((b + 3i32) as isize)).b32.s1 = (*mem
                .offset(((*mem.offset((b + 5i32) as isize)).b32.s1 + 3i32) as isize))
            .b32
            .s1;
            (*mem.offset((b + 2i32) as isize)).b32.s1 = (*mem
                .offset(((*mem.offset((b + 5i32) as isize)).b32.s1 + 2i32) as isize))
            .b32
            .s1
        }
    } else {
        b = new_null_box();
        (*mem.offset((b + 1i32) as isize)).b32.s1 = (*eqtb.offset(
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
                + 11i32) as isize,
        ))
        .b32
        .s1
    }
    (*mem.offset((b + 4i32) as isize)).b32.s1 =
        half((*mem.offset((b + 3i32) as isize)).b32.s1 - (*mem.offset((b + 2i32) as isize)).b32.s1)
            - axis_height(s);
    free_ot_assembly(ot_assembly_ptr as *mut GlyphAssembly);
    return b;
}
unsafe extern "C" fn char_box(mut f: internal_font_number, mut c: int32_t) -> int32_t {
    let mut q: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut b: int32_t = 0;
    let mut p: int32_t = 0;
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32
        || *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
    {
        b = new_null_box();
        p = new_native_character(f, c);
        (*mem.offset((b + 5i32) as isize)).b32.s1 = p;
        (*mem.offset((b + 3i32) as isize)).b32.s1 = (*mem.offset((p + 3i32) as isize)).b32.s1;
        (*mem.offset((b + 1i32) as isize)).b32.s1 = (*mem.offset((p + 1i32) as isize)).b32.s1;
        if (*mem.offset((p + 2i32) as isize)).b32.s1 < 0i32 {
            (*mem.offset((b + 2i32) as isize)).b32.s1 = 0i32
        } else {
            (*mem.offset((b + 2i32) as isize)).b32.s1 = (*mem.offset((p + 2i32) as isize)).b32.s1
        }
    } else {
        q = (*font_info.offset(
            (*char_base.offset(f as isize) + effective_char(1i32 != 0, f, c as u16)) as isize,
        ))
        .b16;
        b = new_null_box();
        (*mem.offset((b + 1i32) as isize)).b32.s1 = (*font_info
            .offset((*width_base.offset(f as isize) + q.s3 as libc::c_int) as isize))
        .b32
        .s1 + (*font_info
            .offset((*italic_base.offset(f as isize) + q.s1 as libc::c_int / 4i32) as isize))
        .b32
        .s1;
        (*mem.offset((b + 3i32) as isize)).b32.s1 = (*font_info
            .offset((*height_base.offset(f as isize) + q.s2 as libc::c_int / 16i32) as isize))
        .b32
        .s1;
        (*mem.offset((b + 2i32) as isize)).b32.s1 = (*font_info
            .offset((*depth_base.offset(f as isize) + q.s2 as libc::c_int % 16i32) as isize))
        .b32
        .s1;
        p = get_avail();
        (*mem.offset(p as isize)).b16.s0 = c as u16;
        (*mem.offset(p as isize)).b16.s1 = f as u16
    }
    (*mem.offset((b + 5i32) as isize)).b32.s1 = p;
    return b;
}
unsafe extern "C" fn stack_into_box(mut b: int32_t, mut f: internal_font_number, mut c: u16) {
    let mut p: int32_t = 0;
    p = char_box(f, c as int32_t);
    (*mem.offset(p as isize)).b32.s1 = (*mem.offset((b + 5i32) as isize)).b32.s1;
    (*mem.offset((b + 5i32) as isize)).b32.s1 = p;
    (*mem.offset((b + 3i32) as isize)).b32.s1 = (*mem.offset((p + 3i32) as isize)).b32.s1;
}
unsafe extern "C" fn height_plus_depth(mut f: internal_font_number, mut c: u16) -> scaled_t {
    let mut q: b16x4 = (*font_info
        .offset((*char_base.offset(f as isize) + effective_char(1i32 != 0, f, c)) as isize))
    .b16;
    return (*font_info
        .offset((*height_base.offset(f as isize) + q.s2 as libc::c_int / 16i32) as isize))
    .b32
    .s1 + (*font_info
        .offset((*depth_base.offset(f as isize) + q.s2 as libc::c_int % 16i32) as isize))
    .b32
    .s1;
}
unsafe extern "C" fn stack_glyph_into_box(
    mut b: int32_t,
    mut f: internal_font_number,
    mut g: int32_t,
) {
    let mut p: int32_t = 0;
    let mut q: int32_t = 0;
    p = get_node(5i32);
    (*mem.offset(p as isize)).b16.s1 = 8i32 as u16;
    (*mem.offset(p as isize)).b16.s0 = 42i32 as u16;
    (*mem.offset((p + 4i32) as isize)).b16.s2 = f as u16;
    (*mem.offset((p + 4i32) as isize)).b16.s1 = g as u16;
    measure_native_glyph(
        &mut *mem.offset(p as isize) as *mut memory_word as *mut libc::c_void,
        1i32,
    );
    if (*mem.offset(b as isize)).b16.s1 as libc::c_int == 0i32 {
        q = (*mem.offset((b + 5i32) as isize)).b32.s1;
        if q == -0xfffffffi32 {
            (*mem.offset((b + 5i32) as isize)).b32.s1 = p
        } else {
            while (*mem.offset(q as isize)).b32.s1 != -0xfffffffi32 {
                q = (*mem.offset(q as isize)).b32.s1
            }
            (*mem.offset(q as isize)).b32.s1 = p;
            if (*mem.offset((b + 3i32) as isize)).b32.s1 < (*mem.offset((p + 3i32) as isize)).b32.s1
            {
                (*mem.offset((b + 3i32) as isize)).b32.s1 =
                    (*mem.offset((p + 3i32) as isize)).b32.s1
            }
            if (*mem.offset((b + 2i32) as isize)).b32.s1 < (*mem.offset((p + 2i32) as isize)).b32.s1
            {
                (*mem.offset((b + 2i32) as isize)).b32.s1 =
                    (*mem.offset((p + 2i32) as isize)).b32.s1
            }
        }
    } else {
        (*mem.offset(p as isize)).b32.s1 = (*mem.offset((b + 5i32) as isize)).b32.s1;
        (*mem.offset((b + 5i32) as isize)).b32.s1 = p;
        (*mem.offset((b + 3i32) as isize)).b32.s1 = (*mem.offset((p + 3i32) as isize)).b32.s1;
        if (*mem.offset((b + 1i32) as isize)).b32.s1 < (*mem.offset((p + 1i32) as isize)).b32.s1 {
            (*mem.offset((b + 1i32) as isize)).b32.s1 = (*mem.offset((p + 1i32) as isize)).b32.s1
        }
    };
}
unsafe extern "C" fn stack_glue_into_box(mut b: int32_t, mut min: scaled_t, mut max: scaled_t) {
    let mut p: int32_t = 0;
    let mut q: int32_t = 0;
    q = new_spec(0i32);
    (*mem.offset((q + 1i32) as isize)).b32.s1 = min;
    (*mem.offset((q + 2i32) as isize)).b32.s1 = max - min;
    p = new_glue(q);
    if (*mem.offset(b as isize)).b16.s1 as libc::c_int == 0i32 {
        q = (*mem.offset((b + 5i32) as isize)).b32.s1;
        if q == -0xfffffffi32 {
            (*mem.offset((b + 5i32) as isize)).b32.s1 = p
        } else {
            while (*mem.offset(q as isize)).b32.s1 != -0xfffffffi32 {
                q = (*mem.offset(q as isize)).b32.s1
            }
            (*mem.offset(q as isize)).b32.s1 = p
        }
    } else {
        (*mem.offset(p as isize)).b32.s1 = (*mem.offset((b + 5i32) as isize)).b32.s1;
        (*mem.offset((b + 5i32) as isize)).b32.s1 = p;
        (*mem.offset((b + 3i32) as isize)).b32.s1 = (*mem.offset((p + 3i32) as isize)).b32.s1;
        (*mem.offset((b + 1i32) as isize)).b32.s1 = (*mem.offset((p + 1i32) as isize)).b32.s1
    };
}
unsafe extern "C" fn build_opentype_assembly(
    mut f: internal_font_number,
    mut a: *mut libc::c_void,
    mut s: scaled_t,
    mut horiz: bool,
) -> int32_t {
    let mut b: int32_t = 0;
    let mut n: int32_t = 0;
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    let mut g: int32_t = 0;
    let mut p: int32_t = 0;
    let mut s_max: scaled_t = 0;
    let mut o: scaled_t = 0;
    let mut oo: scaled_t = 0;
    let mut prev_o: scaled_t = 0;
    let mut min_o: scaled_t = 0;
    let mut no_extenders: bool = false;
    let mut nat: scaled_t = 0;
    let mut str: scaled_t = 0;
    b = new_null_box();
    if horiz {
        (*mem.offset(b as isize)).b16.s1 = 0i32 as u16
    } else {
        (*mem.offset(b as isize)).b16.s1 = 1i32 as u16
    }
    n = -1i32;
    no_extenders = 1i32 != 0;
    min_o = ot_min_connector_overlap(f);
    loop {
        n = n + 1i32;
        s_max = 0i32;
        prev_o = 0i32;
        let mut for_end: int32_t = 0;
        i = 0i32;
        for_end = ot_part_count(a as *const GlyphAssembly) - 1i32;
        if i <= for_end {
            loop {
                if ot_part_is_extender(a as *const GlyphAssembly, i) {
                    no_extenders = 0i32 != 0;
                    let mut for_end_0: int32_t = 0;
                    j = 1i32;
                    for_end_0 = n;
                    if j <= for_end_0 {
                        loop {
                            o = ot_part_start_connector(f, a as *const GlyphAssembly, i);
                            if min_o < o {
                                o = min_o
                            }
                            if prev_o < o {
                                o = prev_o
                            }
                            s_max =
                                s_max - o + ot_part_full_advance(f, a as *const GlyphAssembly, i);
                            prev_o = ot_part_end_connector(f, a as *const GlyphAssembly, i);
                            let fresh3 = j;
                            j = j + 1;
                            if !(fresh3 < for_end_0) {
                                break;
                            }
                        }
                    }
                } else {
                    o = ot_part_start_connector(f, a as *const GlyphAssembly, i);
                    if min_o < o {
                        o = min_o
                    }
                    if prev_o < o {
                        o = prev_o
                    }
                    s_max = s_max - o + ot_part_full_advance(f, a as *const GlyphAssembly, i);
                    prev_o = ot_part_end_connector(f, a as *const GlyphAssembly, i)
                }
                let fresh4 = i;
                i = i + 1;
                if !(fresh4 < for_end) {
                    break;
                }
            }
        }
        if s_max >= s || no_extenders as libc::c_int != 0 {
            break;
        }
    }
    prev_o = 0i32;
    let mut for_end_1: int32_t = 0;
    i = 0i32;
    for_end_1 = ot_part_count(a as *const GlyphAssembly) - 1i32;
    if i <= for_end_1 {
        loop {
            if ot_part_is_extender(a as *const GlyphAssembly, i) {
                let mut for_end_2: int32_t = 0;
                j = 1i32;
                for_end_2 = n;
                if j <= for_end_2 {
                    loop {
                        o = ot_part_start_connector(f, a as *const GlyphAssembly, i);
                        if prev_o < o {
                            o = prev_o
                        }
                        oo = o;
                        if min_o < o {
                            o = min_o
                        }
                        if oo > 0i32 {
                            stack_glue_into_box(b, -oo, -o);
                        }
                        g = ot_part_glyph(a as *const GlyphAssembly, i);
                        stack_glyph_into_box(b, f, g);
                        prev_o = ot_part_end_connector(f, a as *const GlyphAssembly, i);
                        let fresh5 = j;
                        j = j + 1;
                        if !(fresh5 < for_end_2) {
                            break;
                        }
                    }
                }
            } else {
                o = ot_part_start_connector(f, a as *const GlyphAssembly, i);
                if prev_o < o {
                    o = prev_o
                }
                oo = o;
                if min_o < o {
                    o = min_o
                }
                if oo > 0i32 {
                    stack_glue_into_box(b, -oo, -o);
                }
                g = ot_part_glyph(a as *const GlyphAssembly, i);
                stack_glyph_into_box(b, f, g);
                prev_o = ot_part_end_connector(f, a as *const GlyphAssembly, i)
            }
            let fresh6 = i;
            i = i + 1;
            if !(fresh6 < for_end_1) {
                break;
            }
        }
    }
    p = (*mem.offset((b + 5i32) as isize)).b32.s1;
    nat = 0i32;
    str = 0i32;
    while p != -0xfffffffi32 {
        if (*mem.offset(p as isize)).b16.s1 as libc::c_int == 8i32 {
            if horiz {
                nat = nat + (*mem.offset((p + 1i32) as isize)).b32.s1
            } else {
                nat = nat
                    + (*mem.offset((p + 3i32) as isize)).b32.s1
                    + (*mem.offset((p + 2i32) as isize)).b32.s1
            }
        } else if (*mem.offset(p as isize)).b16.s1 as libc::c_int == 10i32 {
            nat = nat
                + (*mem.offset(((*mem.offset((p + 1i32) as isize)).b32.s0 + 1i32) as isize))
                    .b32
                    .s1;
            str = str
                + (*mem.offset(((*mem.offset((p + 1i32) as isize)).b32.s0 + 2i32) as isize))
                    .b32
                    .s1
        }
        p = (*mem.offset(p as isize)).b32.s1
    }
    o = 0i32;
    if s > nat && str > 0i32 {
        o = s - nat;
        if o > str {
            o = str
        }
        (*mem.offset((b + 5i32) as isize)).b16.s0 = 0i32 as u16;
        (*mem.offset((b + 5i32) as isize)).b16.s1 = 1i32 as u16;
        (*mem.offset((b + 6i32) as isize)).gr = o as libc::c_double / str as libc::c_double;
        if horiz {
            (*mem.offset((b + 1i32) as isize)).b32.s1 =
                nat + tex_round(str as libc::c_double * (*mem.offset((b + 6i32) as isize)).gr)
        } else {
            (*mem.offset((b + 3i32) as isize)).b32.s1 =
                nat + tex_round(str as libc::c_double * (*mem.offset((b + 6i32) as isize)).gr)
        }
    } else if horiz {
        (*mem.offset((b + 1i32) as isize)).b32.s1 = nat
    } else {
        (*mem.offset((b + 3i32) as isize)).b32.s1 = nat
    }
    return b;
}
unsafe extern "C" fn rebox(mut b: int32_t, mut w: scaled_t) -> int32_t {
    let mut p: int32_t = 0;
    let mut f: internal_font_number = 0;
    let mut v: scaled_t = 0;
    if (*mem.offset((b + 1i32) as isize)).b32.s1 != w
        && (*mem.offset((b + 5i32) as isize)).b32.s1 != -0xfffffffi32
    {
        if (*mem.offset(b as isize)).b16.s1 as libc::c_int == 1i32 {
            b = hpack(b, 0i32, 1i32 as small_number)
        }
        p = (*mem.offset((b + 5i32) as isize)).b32.s1;
        if is_char_node(p) as libc::c_int != 0 && (*mem.offset(p as isize)).b32.s1 == -0xfffffffi32
        {
            f = (*mem.offset(p as isize)).b16.s1 as internal_font_number;
            v = (*font_info.offset(
                (*width_base.offset(f as isize)
                    + (*font_info.offset(
                        (*char_base.offset(f as isize)
                            + effective_char(1i32 != 0, f, (*mem.offset(p as isize)).b16.s0))
                            as isize,
                    ))
                    .b16
                    .s3 as libc::c_int) as isize,
            ))
            .b32
            .s1;
            if v != (*mem.offset((b + 1i32) as isize)).b32.s1 {
                (*mem.offset(p as isize)).b32.s1 =
                    new_kern((*mem.offset((b + 1i32) as isize)).b32.s1 - v)
            }
        }
        free_node(b, 8i32);
        b = new_glue(12i32);
        (*mem.offset(b as isize)).b32.s1 = p;
        while (*mem.offset(p as isize)).b32.s1 != -0xfffffffi32 {
            p = (*mem.offset(p as isize)).b32.s1
        }
        (*mem.offset(p as isize)).b32.s1 = new_glue(12i32);
        return hpack(b, w, 0i32 as small_number);
    } else {
        (*mem.offset((b + 1i32) as isize)).b32.s1 = w;
        return b;
    };
}
