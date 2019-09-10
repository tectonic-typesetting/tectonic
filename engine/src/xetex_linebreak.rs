#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn measure_native_node(node: *mut libc::c_void, use_glyph_metrics: libc::c_int);
    #[no_mangle]
    static mut eqtb: *mut memory_word;
    #[no_mangle]
    static mut file_line_error_style_p: libc::c_int;
    #[no_mangle]
    static mut str_pool: *mut packed_UTF16_code;
    #[no_mangle]
    static mut str_start: *mut pool_pointer;
    #[no_mangle]
    static mut help_line: [*const libc::c_char; 6];
    #[no_mangle]
    static mut help_ptr: libc::c_uchar;
    #[no_mangle]
    static mut arith_error: bool;
    #[no_mangle]
    static mut temp_ptr: int32_t;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut hi_mem_min: int32_t;
    #[no_mangle]
    static mut avail: int32_t;
    #[no_mangle]
    static mut last_leftmost_char: int32_t;
    #[no_mangle]
    static mut last_rightmost_char: int32_t;
    #[no_mangle]
    static mut hlist_stack: [int32_t; 513];
    #[no_mangle]
    static mut hlist_stack_level: libc::c_short;
    #[no_mangle]
    static mut first_p: int32_t;
    #[no_mangle]
    static mut global_prev_p: int32_t;
    #[no_mangle]
    static mut font_in_short_display: int32_t;
    #[no_mangle]
    static mut cur_list: list_state_record;
    #[no_mangle]
    static mut font_info: *mut memory_word;
    #[no_mangle]
    static mut hyphen_char: *mut int32_t;
    #[no_mangle]
    static mut bchar_label: *mut font_index;
    #[no_mangle]
    static mut font_bchar: *mut nine_bits;
    #[no_mangle]
    static mut char_base: *mut int32_t;
    #[no_mangle]
    static mut width_base: *mut int32_t;
    #[no_mangle]
    static mut lig_kern_base: *mut int32_t;
    #[no_mangle]
    static mut kern_base: *mut int32_t;
    #[no_mangle]
    static mut adjust_tail: int32_t;
    #[no_mangle]
    static mut pre_adjust_tail: int32_t;
    #[no_mangle]
    static mut pack_begin_line: int32_t;
    #[no_mangle]
    static mut just_box: int32_t;
    #[no_mangle]
    static mut active_width: [scaled_t; 7];
    #[no_mangle]
    static mut hc: [int32_t; 4099];
    #[no_mangle]
    static mut hf: internal_font_number;
    #[no_mangle]
    static mut hu: [int32_t; 4097];
    #[no_mangle]
    static mut cur_lang: libc::c_uchar;
    #[no_mangle]
    static mut max_hyph_char: int32_t;
    #[no_mangle]
    static mut hyf: [libc::c_uchar; 4097];
    #[no_mangle]
    static mut init_list: int32_t;
    #[no_mangle]
    static mut init_lig: bool;
    #[no_mangle]
    static mut init_lft: bool;
    #[no_mangle]
    static mut hyphen_passed: small_number;
    #[no_mangle]
    static mut cur_l: int32_t;
    #[no_mangle]
    static mut cur_r: int32_t;
    #[no_mangle]
    static mut cur_q: int32_t;
    #[no_mangle]
    static mut lig_stack: int32_t;
    #[no_mangle]
    static mut ligature_present: bool;
    #[no_mangle]
    static mut lft_hit: bool;
    #[no_mangle]
    static mut rt_hit: bool;
    #[no_mangle]
    static mut trie_trl: *mut trie_pointer;
    #[no_mangle]
    static mut trie_tro: *mut trie_pointer;
    #[no_mangle]
    static mut trie_trc: *mut u16;
    #[no_mangle]
    static mut hyf_distance: [small_number; 35112];
    #[no_mangle]
    static mut hyf_num: [small_number; 35112];
    #[no_mangle]
    static mut hyf_next: [trie_opcode; 35112];
    #[no_mangle]
    static mut op_start: [int32_t; 256];
    #[no_mangle]
    static mut hyph_word: *mut str_number;
    #[no_mangle]
    static mut hyph_list: *mut int32_t;
    #[no_mangle]
    static mut hyph_link: *mut hyph_pointer;
    #[no_mangle]
    static mut trie_not_ready: bool;
    #[no_mangle]
    static mut hyph_start: trie_pointer;
    #[no_mangle]
    static mut hyph_index: trie_pointer;
    #[no_mangle]
    static mut xtx_ligature_present: bool;
    #[no_mangle]
    static mut semantic_pagination_enabled: bool;
    #[no_mangle]
    fn badness(t: scaled_t, s: scaled_t) -> int32_t;
    #[no_mangle]
    fn get_avail() -> int32_t;
    #[no_mangle]
    fn flush_list(p: int32_t);
    #[no_mangle]
    fn get_node(s: int32_t) -> int32_t;
    #[no_mangle]
    fn free_node(p: int32_t, s: int32_t);
    #[no_mangle]
    fn new_ligature(f: internal_font_number, c: u16, q: int32_t) -> int32_t;
    #[no_mangle]
    fn new_lig_item(c: u16) -> int32_t;
    #[no_mangle]
    fn new_disc() -> int32_t;
    #[no_mangle]
    fn new_math(w: scaled_t, s: small_number) -> int32_t;
    #[no_mangle]
    fn new_spec(p: int32_t) -> int32_t;
    #[no_mangle]
    fn new_param_glue(n: small_number) -> int32_t;
    #[no_mangle]
    fn new_kern(w: scaled_t) -> int32_t;
    #[no_mangle]
    fn new_penalty(m: int32_t) -> int32_t;
    #[no_mangle]
    fn prev_rightmost(s: int32_t, e: int32_t) -> int32_t;
    #[no_mangle]
    fn delete_glue_ref(p: int32_t);
    #[no_mangle]
    fn flush_node_list(p: int32_t);
    #[no_mangle]
    fn pop_nest();
    #[no_mangle]
    fn length(s: str_number) -> int32_t;
    #[no_mangle]
    fn init_trie();
    #[no_mangle]
    fn max_hyphenatable_length() -> int32_t;
    #[no_mangle]
    fn append_to_vlist(b: int32_t);
    #[no_mangle]
    fn hpack(p: int32_t, w: scaled_t, m: small_number) -> int32_t;
    #[no_mangle]
    fn new_margin_kern(w: scaled_t, p: int32_t, side: small_number) -> int32_t;
    #[no_mangle]
    fn char_pw(p: int32_t, side: small_number) -> scaled_t;
    #[no_mangle]
    fn new_character(f: internal_font_number, c: UTF16_code) -> int32_t;
    #[no_mangle]
    fn new_native_character(f: internal_font_number, c: UnicodeScalar) -> int32_t;
    #[no_mangle]
    fn new_native_word_node(f: internal_font_number, n: int32_t) -> int32_t;
    #[no_mangle]
    fn fract(x: int32_t, n: int32_t, d: int32_t, max_answer: int32_t) -> int32_t;
    #[no_mangle]
    fn effective_char(err_p: bool, f: internal_font_number, c: u16) -> int32_t;
    #[no_mangle]
    fn confusion(s: *const libc::c_char) -> !;
    #[no_mangle]
    fn print_nl_cstr(s: *const libc::c_char);
    #[no_mangle]
    fn error();
    #[no_mangle]
    fn print_cstr(s: *const libc::c_char);
    #[no_mangle]
    fn pdf_error(t: *const libc::c_char, p: *const libc::c_char) -> !;
    #[no_mangle]
    fn print_file_line();
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type scaled_t = int32_t;
pub type UTF16_code = libc::c_ushort;
pub type UnicodeScalar = int32_t;
pub type pool_pointer = int32_t;
pub type str_number = int32_t;
pub type packed_UTF16_code = libc::c_ushort;
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
pub type internal_font_number = int32_t;
pub type font_index = int32_t;
pub type nine_bits = int32_t;
pub type trie_pointer = int32_t;
pub type trie_opcode = libc::c_ushort;
pub type hyph_pointer = libc::c_ushort;
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
#[inline]
unsafe extern "C" fn is_char_node(p: int32_t) -> bool {
    return p >= hi_mem_min;
}
#[inline]
unsafe extern "C" fn is_non_discardable_node(p: int32_t) -> bool {
    return ((*mem.offset(p as isize)).b16.s1 as libc::c_int) < 9i32;
}
static mut passive: int32_t = 0;
static mut cur_active_width: [scaled_t; 7] = [0; 7];
static mut background: [scaled_t; 7] = [0; 7];
static mut break_width: [scaled_t; 7] = [0; 7];
static mut best_place: [int32_t; 4] = [0; 4];
static mut best_pl_line: [int32_t; 4] = [0; 4];
static mut disc_width: scaled_t = 0;
static mut no_shrink_error_yet: bool = false;
static mut cur_p: int32_t = 0;
static mut second_pass: bool = false;
static mut final_pass: bool = false;
static mut threshold: int32_t = 0;
static mut minimal_demerits: [int32_t; 4] = [0; 4];
static mut minimum_demerits: int32_t = 0;
static mut easy_line: int32_t = 0;
static mut last_special_line: int32_t = 0;
static mut first_width: scaled_t = 0;
static mut second_width: scaled_t = 0;
static mut first_indent: scaled_t = 0;
static mut second_indent: scaled_t = 0;
static mut best_bet: int32_t = 0;
static mut fewest_demerits: int32_t = 0;
static mut best_line: int32_t = 0;
static mut actual_looseness: int32_t = 0;
static mut line_diff: int32_t = 0;
static mut hn: small_number = 0;
static mut ha: int32_t = 0;
static mut hb: int32_t = 0;
static mut hyf_char: int32_t = 0;
static mut init_cur_lang: libc::c_uchar = 0;
static mut l_hyf: int32_t = 0;
static mut r_hyf: int32_t = 0;
static mut init_l_hyf: int32_t = 0;
static mut init_r_hyf: int32_t = 0;
static mut hyf_bchar: int32_t = 0;
static mut last_line_fill: int32_t = 0;
static mut do_last_line_fit: bool = false;
static mut active_node_size: small_number = 0;
static mut fill_width: [scaled_t; 3] = [0; 3];
static mut best_pl_short: [scaled_t; 4] = [0; 4];
static mut best_pl_glue: [scaled_t; 4] = [0; 4];
#[inline]
unsafe extern "C" fn get_native_usv(mut p: int32_t, mut i: int32_t) -> UnicodeScalar {
    let mut c: libc::c_ushort = *(&mut *mem.offset((p + 6i32) as isize) as *mut memory_word
        as *mut libc::c_ushort)
        .offset(i as isize);
    if c as libc::c_int >= 0xd800i32 && (c as libc::c_int) < 0xdc00i32 {
        return 0x10000i32
            + (c as libc::c_int - 0xd800i32) * 0x400i32
            + *(&mut *mem.offset((p + 6i32) as isize) as *mut memory_word as *mut libc::c_ushort)
                .offset((i + 1i32) as isize) as libc::c_int
            - 0xdc00i32;
    }
    return c as UnicodeScalar;
}
/* Break a paragraph into lines (XTTP:843).
 *
 * d: true if we are breaking a partial paragraph preceding display math mode
 *
 * Should only be called in horizontal mode. Will leave horizontal and place
 * the output in the enclosing vertical list.
 *
 * `cur_list.head` is the non-empty hlist to be broken. `prev_graf` tells the
 * starting line number (0 unless we're continuing after display math). After
 * completion, `just_box` will point to the final box created.
 */
#[no_mangle]
pub unsafe extern "C" fn line_break(mut d: bool) {
    let mut current_block: u64; /* "this is for over/underfull box messages" */
    let mut auto_breaking: bool = false;
    let mut prev_p: int32_t = 0;
    let mut q: int32_t = 0;
    let mut r: int32_t = 0;
    let mut s: int32_t = 0;
    let mut prev_s: int32_t = 0;
    let mut f: internal_font_number = 0;
    let mut j: small_number = 0;
    let mut c: UnicodeScalar = 0;
    let mut l: int32_t = 0;
    let mut i: int32_t = 0;
    let mut for_end_1: int32_t = 0;
    pack_begin_line = cur_list.mode_line;
    (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 =
        (*mem.offset(cur_list.head as isize)).b32.s1;
    /* Remove trailing space or glue if present; add infinite penalty then par_fill_skip */
    if is_char_node(cur_list.tail) {
        /* is_char_node */
        let ref mut fresh0 = (*mem.offset(cur_list.tail as isize)).b32.s1;
        *fresh0 = new_penalty(10000i32);
        cur_list.tail = *fresh0
    } else if (*mem.offset(cur_list.tail as isize)).b16.s1 as libc::c_int != 10i32 {
        let ref mut fresh1 = (*mem.offset(cur_list.tail as isize)).b32.s1;
        *fresh1 = new_penalty(10000i32);
        cur_list.tail = *fresh1
    } else {
        (*mem.offset(cur_list.tail as isize)).b16.s1 = 12i32 as u16;
        delete_glue_ref((*mem.offset((cur_list.tail + 1i32) as isize)).b32.s0);
        flush_node_list((*mem.offset((cur_list.tail + 1i32) as isize)).b32.s1);
        (*mem.offset((cur_list.tail + 1i32) as isize)).b32.s1 = 10000i32
    }
    let ref mut fresh2 = (*mem.offset(cur_list.tail as isize)).b32.s1;
    *fresh2 = new_param_glue(14i32 as small_number);
    last_line_fill = *fresh2;
    /* Yet more initialization of various kinds */
    init_cur_lang = (cur_list.prev_graf as libc::c_long % 65536) as libc::c_uchar;
    init_l_hyf = cur_list.prev_graf / 0x400000i32;
    init_r_hyf = (cur_list.prev_graf as libc::c_long / 65536 % 64i32 as libc::c_long) as int32_t;
    pop_nest();
    no_shrink_error_yet = 1i32 != 0;
    if (*mem.offset(
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
                + 7i32) as isize,
        ))
        .b32
        .s1 as isize,
    ))
    .b16
    .s0 as libc::c_int
        != 0i32
        && (*mem.offset(
            ((*eqtb.offset(
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
            .s1 + 3i32) as isize,
        ))
        .b32
        .s1 != 0i32
    {
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
                + 7i32) as isize,
        ))
        .b32
        .s1 = finite_shrink(
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
                    + 7i32) as isize,
            ))
            .b32
            .s1,
        )
    }
    if (*mem.offset(
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
                + 8i32) as isize,
        ))
        .b32
        .s1 as isize,
    ))
    .b16
    .s0 as libc::c_int
        != 0i32
        && (*mem.offset(
            ((*eqtb.offset(
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
            .s1 + 3i32) as isize,
        ))
        .b32
        .s1 != 0i32
    {
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
                + 8i32) as isize,
        ))
        .b32
        .s1 = finite_shrink(
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
                    + 8i32) as isize,
            ))
            .b32
            .s1,
        )
    }
    q = (*eqtb.offset(
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
    .s1;
    r = (*eqtb.offset(
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
    .s1;
    background[1] =
        (*mem.offset((q + 1i32) as isize)).b32.s1 + (*mem.offset((r + 1i32) as isize)).b32.s1;
    background[2] = 0i32;
    background[3] = 0i32;
    background[4] = 0i32;
    background[5] = 0i32;
    background[(2i32 + (*mem.offset(q as isize)).b16.s1 as libc::c_int) as usize] =
        (*mem.offset((q + 2i32) as isize)).b32.s1;
    background[(2i32 + (*mem.offset(r as isize)).b16.s1 as libc::c_int) as usize] +=
        (*mem.offset((r + 2i32) as isize)).b32.s1;
    background[6] =
        (*mem.offset((q + 3i32) as isize)).b32.s1 + (*mem.offset((r + 3i32) as isize)).b32.s1;
    /* 1631: "check for special treatment of last line of paragraph" (\lastlinefit > 0) */
    do_last_line_fit = 0i32 != 0; /*863:*/
    active_node_size = 3i32 as small_number;
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
            + 64i32) as isize,
    ))
    .b32
    .s1 > 0i32
    {
        q = (*mem.offset((last_line_fill + 1i32) as isize)).b32.s0;
        if (*mem.offset((q + 2i32) as isize)).b32.s1 > 0i32
            && (*mem.offset(q as isize)).b16.s1 as libc::c_int > 0i32
        {
            if background[3] == 0i32 && background[4] == 0i32 && background[5] == 0i32 {
                do_last_line_fit = 1i32 != 0;
                active_node_size = 5i32 as small_number;
                fill_width[0] = 0i32;
                fill_width[1] = 0i32;
                fill_width[2] = 0i32;
                fill_width[((*mem.offset(q as isize)).b16.s1 as libc::c_int - 1i32) as usize] =
                    (*mem.offset((q + 2i32) as isize)).b32.s1
            }
        }
    }
    minimum_demerits = 0x3fffffffi32;
    minimal_demerits[3] = 0x3fffffffi32;
    minimal_demerits[2] = 0x3fffffffi32;
    minimal_demerits[1] = 0x3fffffffi32;
    minimal_demerits[0] = 0x3fffffffi32;
    /* Prep relating to par_shape (877) */
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
        .s1 == 0i32
        {
            last_special_line = 0i32; /*878:*/
            second_width = (*eqtb.offset(
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
            second_indent = 0i32
        } else {
            last_special_line = abs((*eqtb.offset(
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
                    + 41i32) as isize,
            ))
            .b32
            .s1 < 0i32
            {
                first_width = (*eqtb.offset(
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
                .s1 >= 0i32
                {
                    first_indent = (*eqtb.offset(
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
                    first_indent = 0i32
                }
                second_width = (*eqtb.offset(
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
                second_indent = 0i32
            } else {
                first_width = (*eqtb.offset(
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
                first_indent = 0i32;
                second_width = (*eqtb.offset(
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
                .s1 >= 0i32
                {
                    second_indent = (*eqtb.offset(
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
                    second_indent = 0i32
                }
            }
        }
    } else {
        last_special_line = (*mem.offset(
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
        .s0 - 1i32;
        /* These direct `mem` accesses are in the original WEB code */
        second_width = (*mem.offset(
            ((*eqtb.offset(
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
            .s1 + 2i32 * (last_special_line + 1i32)) as isize,
        ))
        .b32
        .s1; /*:877*/
        second_indent = (*mem.offset(
            ((*eqtb.offset(
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
            .s1 + 2i32 * last_special_line
                + 1i32) as isize,
        ))
        .b32
        .s1
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
            + 19i32) as isize,
    ))
    .b32
    .s1 == 0i32
    {
        easy_line = last_special_line
    } else {
        easy_line = 0x3fffffffi32
    }
    /* Start finding optimal breakpoints (892) */
    threshold = (*eqtb.offset(
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
            + 0i32) as isize,
    ))
    .b32
    .s1;
    if threshold >= 0i32 {
        second_pass = 0i32 != 0;
        final_pass = 0i32 != 0
    } else {
        threshold = (*eqtb.offset(
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
        .s1;
        second_pass = 1i32 != 0;
        final_pass = (*eqtb.offset(
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
                + 20i32) as isize,
        ))
        .b32
        .s1 <= 0i32
    }
    loop {
        if threshold > 10000i32 {
            threshold = 10000i32
        }
        if second_pass {
            /*920:*/
            if trie_not_ready {
                init_trie(); /*893:*/
            }
            cur_lang = init_cur_lang;
            l_hyf = init_l_hyf;
            r_hyf = init_r_hyf;
            if *trie_trc.offset((hyph_start + cur_lang as libc::c_int) as isize) as libc::c_int
                != cur_lang as libc::c_int
            {
                hyph_index = 0i32
            } else {
                hyph_index = *trie_trl.offset((hyph_start + cur_lang as libc::c_int) as isize)
            }
        }
        q = get_node(active_node_size as int32_t);
        (*mem.offset(q as isize)).b16.s1 = 0i32 as u16;
        (*mem.offset(q as isize)).b16.s0 = 2i32 as u16;
        (*mem.offset(q as isize)).b32.s1 = 4999999i32 - 7i32;
        (*mem.offset((q + 1i32) as isize)).b32.s1 = -0xfffffffi32;
        (*mem.offset((q + 1i32) as isize)).b32.s0 = cur_list.prev_graf + 1i32;
        (*mem.offset((q + 2i32) as isize)).b32.s1 = 0i32;
        (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1 = q;
        if do_last_line_fit {
            /*1633:*/
            (*mem.offset((q + 3i32) as isize)).b32.s1 = 0i32; /*:893*/
            (*mem.offset((q + 4i32) as isize)).b32.s1 = 0i32
        }
        active_width[1] = background[1];
        active_width[2] = background[2];
        active_width[3] = background[3];
        active_width[4] = background[4];
        active_width[5] = background[5];
        active_width[6] = background[6];
        passive = -0xfffffffi32;
        font_in_short_display = 0i32;
        cur_p = (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1;
        auto_breaking = 1i32 != 0;
        global_prev_p = cur_p;
        prev_p = global_prev_p;
        first_p = cur_p;
        while cur_p != -0xfffffffi32
            && (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1 != 4999999i32 - 7i32
        {
            /*895: "Call try_break if cur_p is a legal breakpoint; on the
             * second pass, also try to hyphenate the next word, if cur_p is a
             * glue node; then advance cur_p to the next node of the paragraph
             * that could possibly be a legal breakpoint." */
            if is_char_node(cur_p) {
                /*896:*/
                global_prev_p = cur_p;
                prev_p = global_prev_p;
                loop {
                    let mut eff_char: int32_t = 0;
                    f = (*mem.offset(cur_p as isize)).b16.s1 as internal_font_number;
                    eff_char = effective_char(1i32 != 0, f, (*mem.offset(cur_p as isize)).b16.s0);
                    active_width[1] += (*font_info.offset(
                        (*width_base.offset(f as isize)
                            + (*font_info
                                .offset((*char_base.offset(f as isize) + eff_char) as isize))
                            .b16
                            .s3 as libc::c_int) as isize,
                    ))
                    .b32
                    .s1;
                    cur_p = (*mem.offset(cur_p as isize)).b32.s1;
                    if !is_char_node(cur_p) {
                        break;
                    }
                }
            }
            match (*mem.offset(cur_p as isize)).b16.s1 as libc::c_int {
                0 | 1 | 2 => active_width[1] += (*mem.offset((cur_p + 1i32) as isize)).b32.s1,
                8 => {
                    if (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 4i32 {
                        cur_lang = (*mem.offset((cur_p + 1i32) as isize)).b32.s1 as libc::c_uchar;
                        l_hyf = (*mem.offset((cur_p + 1i32) as isize)).b16.s1 as int32_t;
                        r_hyf = (*mem.offset((cur_p + 1i32) as isize)).b16.s0 as int32_t;
                        if *trie_trc.offset((hyph_start + cur_lang as libc::c_int) as isize)
                            as libc::c_int
                            != cur_lang as libc::c_int
                        {
                            hyph_index = 0i32
                        } else {
                            hyph_index =
                                *trie_trl.offset((hyph_start + cur_lang as libc::c_int) as isize)
                        }
                    } else if (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 40i32
                        || (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 41i32
                        || (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 42i32
                        || (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 43i32
                        || (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 44i32
                    {
                        active_width[1] += (*mem.offset((cur_p + 1i32) as isize)).b32.s1
                    }
                }
                10 => {
                    if auto_breaking {
                        if is_char_node(prev_p) {
                            try_break(0i32, 0i32 as small_number);
                        } else if is_non_discardable_node(prev_p) {
                            try_break(0i32, 0i32 as small_number);
                        } else if (*mem.offset(prev_p as isize)).b16.s1 as libc::c_int == 11i32
                            && (*mem.offset(prev_p as isize)).b16.s0 as libc::c_int != 1i32
                        {
                            try_break(0i32, 0i32 as small_number);
                        }
                    }
                    q = (*mem.offset((cur_p + 1i32) as isize)).b32.s0;
                    if (*mem.offset(q as isize)).b16.s0 as libc::c_int != 0i32
                        && (*mem.offset((q + 3i32) as isize)).b32.s1 != 0i32
                    {
                        let ref mut fresh3 = (*mem.offset((cur_p + 1i32) as isize)).b32.s0;
                        *fresh3 = finite_shrink(q);
                        q = *fresh3
                    }
                    active_width[1] += (*mem.offset((q + 1i32) as isize)).b32.s1;
                    active_width
                        [(2i32 + (*mem.offset(q as isize)).b16.s1 as libc::c_int) as usize] +=
                        (*mem.offset((q + 2i32) as isize)).b32.s1;
                    /*:895*/
                    active_width[6] += (*mem.offset((q + 3i32) as isize)).b32.s1; /*:897*/
                    if second_pass as libc::c_int != 0 && auto_breaking as libc::c_int != 0 {
                        /*924: "Try to hyphenate the following word." */
                        prev_s = cur_p;
                        s = (*mem.offset(prev_s as isize)).b32.s1;
                        if s != -0xfffffffi32 {
                            's_786: loop
                            /*930: skip to node ha, or goto done1 if no hyphenation should be attempted */
                            {
                                if is_char_node(s) {
                                    c = (*mem.offset(s as isize)).b16.s0 as UnicodeScalar; /*:930*/
                                    hf = (*mem.offset(s as isize)).b16.s1 as internal_font_number;
                                    current_block = 11202235766349324107;
                                } else if (*mem.offset(s as isize)).b16.s1 as libc::c_int == 6i32 {
                                    if (*mem.offset((s + 1i32) as isize)).b32.s1 == -0xfffffffi32 {
                                        current_block = 13855806088735179493;
                                    } else {
                                        q = (*mem.offset((s + 1i32) as isize)).b32.s1;
                                        c = (*mem.offset(q as isize)).b16.s0 as UnicodeScalar;
                                        hf = (*mem.offset(q as isize)).b16.s1
                                            as internal_font_number;
                                        current_block = 11202235766349324107;
                                    }
                                } else if (*mem.offset(s as isize)).b16.s1 as libc::c_int == 11i32
                                    && (*mem.offset(s as isize)).b16.s0 as libc::c_int == 0i32
                                {
                                    current_block = 13855806088735179493;
                                } else if (*mem.offset(s as isize)).b16.s1 as libc::c_int == 9i32
                                    && (*mem.offset(s as isize)).b16.s0 as libc::c_int >= 4i32
                                {
                                    current_block = 13855806088735179493;
                                } else {
                                    if !((*mem.offset(s as isize)).b16.s1 as libc::c_int == 8i32) {
                                        current_block = 8166967358843938227;
                                        break;
                                    }
                                    if (*mem.offset(s as isize)).b16.s0 as libc::c_int == 40i32
                                        || (*mem.offset(s as isize)).b16.s0 as libc::c_int == 41i32
                                    {
                                        l = 0i32;
                                        while l
                                            < (*mem.offset((s + 4i32) as isize)).b16.s1
                                                as libc::c_int
                                        {
                                            c = get_native_usv(s, l);
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
                                                    + c)
                                                    as isize,
                                            ))
                                            .b32
                                            .s1 != 0i32
                                            {
                                                hf = (*mem.offset((s + 4i32) as isize)).b16.s2
                                                    as internal_font_number;
                                                prev_s = s;
                                                current_block = 16581706250867416845;
                                                break 's_786;
                                            } else {
                                                if c as libc::c_long >= 65536 {
                                                    l += 1
                                                }
                                                l += 1
                                            }
                                        }
                                    }
                                    if (*mem.offset(s as isize)).b16.s0 as libc::c_int == 4i32 {
                                        cur_lang = (*mem.offset((s + 1i32) as isize)).b32.s1
                                            as libc::c_uchar;
                                        l_hyf =
                                            (*mem.offset((s + 1i32) as isize)).b16.s1 as int32_t;
                                        r_hyf =
                                            (*mem.offset((s + 1i32) as isize)).b16.s0 as int32_t;
                                        if *trie_trc
                                            .offset((hyph_start + cur_lang as libc::c_int) as isize)
                                            as libc::c_int
                                            != cur_lang as libc::c_int
                                        {
                                            hyph_index = 0i32
                                        } else {
                                            hyph_index = *trie_trl.offset(
                                                (hyph_start + cur_lang as libc::c_int) as isize,
                                            )
                                        }
                                    }
                                    current_block = 13855806088735179493;
                                }
                                match current_block {
                                    11202235766349324107 => {
                                        if hyph_index == 0i32 || c > 255i32 {
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
                                                    + c)
                                                    as isize,
                                            ))
                                            .b32
                                            .s1
                                        } else if *trie_trc.offset((hyph_index + c) as isize)
                                            as libc::c_int
                                            != c
                                        {
                                            hc[0] = 0i32
                                        } else {
                                            hc[0] = *trie_tro.offset((hyph_index + c) as isize)
                                        }
                                        if hc[0] != 0i32 {
                                            if hc[0] == c
                                                || (*eqtb.offset(
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
                                                        + 38i32)
                                                        as isize,
                                                ))
                                                .b32
                                                .s1 > 0i32
                                            {
                                                current_block = 16581706250867416845;
                                                break;
                                            } else {
                                                current_block = 8166967358843938227;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                prev_s = s;
                                s = (*mem.offset(prev_s as isize)).b32.s1
                            }
                            match current_block {
                                8166967358843938227 => {}
                                _ => {
                                    hyf_char = *hyphen_char.offset(hf as isize);
                                    if !(hyf_char < 0i32) {
                                        if !(hyf_char > 0xffffi32) {
                                            ha = prev_s;
                                            if !(l_hyf + r_hyf > max_hyphenatable_length()) {
                                                if ha != -0xfffffffi32
                                                    && ha < hi_mem_min
                                                    && (*mem.offset(ha as isize)).b16.s1
                                                        as libc::c_int
                                                        == 8i32
                                                    && ((*mem.offset(ha as isize)).b16.s0
                                                        as libc::c_int
                                                        == 40i32
                                                        || (*mem.offset(ha as isize)).b16.s0
                                                            as libc::c_int
                                                            == 41i32)
                                                {
                                                    /*926: check that nodes after native_word permit hyphenation; if not, goto done1 */
                                                    s = (*mem.offset(ha as isize)).b32.s1;
                                                    loop {
                                                        if !is_char_node(s) {
                                                            match (*mem.offset(s as isize)).b16.s1
                                                                as libc::c_int
                                                            {
                                                                6 => {}
                                                                11 => {
                                                                    if (*mem.offset(s as isize))
                                                                        .b16
                                                                        .s0
                                                                        as libc::c_int
                                                                        != 0i32
                                                                    {
                                                                        current_block =
                                                                            2606747282402567793;
                                                                        break;
                                                                    }
                                                                }
                                                                8 | 10 | 12 | 3 | 5 | 4 => {
                                                                    current_block =
                                                                        2606747282402567793;
                                                                    break;
                                                                }
                                                                _ => {
                                                                    current_block =
                                                                        8166967358843938227;
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        s = (*mem.offset(s as isize)).b32.s1
                                                    }
                                                    match current_block {
                                                        8166967358843938227 => {}
                                                        _ => {
                                                            /*927: prepare a native_word_node for hyphenation.
                                                             * "Note that if there are chars with lccode = 0,
                                                             * we split them out into separate native_word
                                                             * nodes." */
                                                            hn = 0i32 as small_number;
                                                            'c_31290: loop {
                                                                /* 'ha' can change in the loop, so for safety: */
                                                                for_end_1 = (*mem
                                                                    .offset((ha + 4i32) as isize))
                                                                .b16
                                                                .s1
                                                                    as int32_t;
                                                                l = 0i32;
                                                                loop {
                                                                    if !(l < for_end_1) {
                                                                        break 'c_31290;
                                                                    }
                                                                    c = get_native_usv(ha, l);
                                                                    if hyph_index == 0i32
                                                                        || c > 255i32
                                                                    {
                                                                        hc[0] = (*eqtb.offset(
                                                                            (1i32
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
                                                                                + c)
                                                                                as isize,
                                                                        ))
                                                                        .b32
                                                                        .s1
                                                                    } else if *trie_trc.offset(
                                                                        (hyph_index + c) as isize,
                                                                    )
                                                                        as libc::c_int
                                                                        != c
                                                                    {
                                                                        hc[0] = 0i32
                                                                    } else {
                                                                        hc[0] = *trie_tro.offset(
                                                                            (hyph_index + c)
                                                                                as isize,
                                                                        )
                                                                    }
                                                                    if hc[0] == 0i32 {
                                                                        if hn as libc::c_int > 0i32
                                                                        {
                                                                            q
                                                                                    =
                                                                                    new_native_word_node(hf,
                                                                                                         (*mem.offset((ha
                                                                                                                           +
                                                                                                                           4i32)
                                                                                                                          as
                                                                                                                          isize)).b16.s1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             -
                                                                                                             l);
                                                                            (*mem.offset(
                                                                                q as isize,
                                                                            ))
                                                                            .b16
                                                                            .s0 = (*mem.offset(
                                                                                ha as isize,
                                                                            ))
                                                                            .b16
                                                                            .s0;
                                                                            i = l;
                                                                            while i
                                                                                < (*mem.offset(
                                                                                    (ha + 4i32)
                                                                                        as isize,
                                                                                ))
                                                                                .b16
                                                                                .s1
                                                                                    as libc::c_int
                                                                            {
                                                                                *(&mut *mem.offset((q
                                                                                                            +
                                                                                                            6i32)
                                                                                                           as
                                                                                                           isize)
                                                                                          as
                                                                                          *mut memory_word
                                                                                          as
                                                                                          *mut libc::c_ushort).offset((i
                                                                                                                           -
                                                                                                                           l)
                                                                                                                          as
                                                                                                                          isize)
                                                                                        =
                                                                                        *(&mut *mem.offset((ha
                                                                                                                +
                                                                                                                6i32)
                                                                                                               as
                                                                                                               isize)
                                                                                              as
                                                                                              *mut memory_word
                                                                                              as
                                                                                              *mut libc::c_ushort).offset(i
                                                                                                                              as
                                                                                                                              isize);
                                                                                i += 1
                                                                            }
                                                                            measure_native_node(&mut *mem.offset(q
                                                                                                                         as
                                                                                                                         isize)
                                                                                                        as
                                                                                                        *mut memory_word
                                                                                                        as
                                                                                                        *mut libc::c_void,
                                                                                                    ((*eqtb.offset((1i32
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
                                                                                                                        74i32)
                                                                                                                       as
                                                                                                                       isize)).b32.s1
                                                                                                         >
                                                                                                         0i32)
                                                                                                        as
                                                                                                        libc::c_int);
                                                                            (*mem.offset(
                                                                                q as isize,
                                                                            ))
                                                                            .b32
                                                                            .s1 = (*mem.offset(
                                                                                ha as isize,
                                                                            ))
                                                                            .b32
                                                                            .s1;
                                                                            (*mem.offset(
                                                                                ha as isize,
                                                                            ))
                                                                            .b32
                                                                            .s1 = q;
                                                                            (*mem.offset(
                                                                                (ha + 4i32)
                                                                                    as isize,
                                                                            ))
                                                                            .b16
                                                                            .s1 = l as u16;
                                                                            measure_native_node(&mut *mem.offset(ha
                                                                                                                         as
                                                                                                                         isize)
                                                                                                        as
                                                                                                        *mut memory_word
                                                                                                        as
                                                                                                        *mut libc::c_void,
                                                                                                    ((*eqtb.offset((1i32
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
                                                                                                                        74i32)
                                                                                                                       as
                                                                                                                       isize)).b32.s1
                                                                                                         >
                                                                                                         0i32)
                                                                                                        as
                                                                                                        libc::c_int);
                                                                            break 'c_31290;
                                                                        }
                                                                    } else if hn as libc::c_int
                                                                        == 0i32
                                                                        && l > 0i32
                                                                    {
                                                                        q = new_native_word_node(
                                                                            hf,
                                                                            (*mem.offset(
                                                                                (ha + 4i32)
                                                                                    as isize,
                                                                            ))
                                                                            .b16
                                                                            .s1
                                                                                as libc::c_int
                                                                                - l,
                                                                        );
                                                                        (*mem.offset(q as isize))
                                                                            .b16
                                                                            .s0 = (*mem
                                                                            .offset(ha as isize))
                                                                        .b16
                                                                        .s0;
                                                                        i = l;
                                                                        while i
                                                                            < (*mem.offset(
                                                                                (ha + 4i32)
                                                                                    as isize,
                                                                            ))
                                                                            .b16
                                                                            .s1
                                                                                as libc::c_int
                                                                        {
                                                                            *(&mut *mem.offset((q
                                                                                                        +
                                                                                                        6i32)
                                                                                                       as
                                                                                                       isize)
                                                                                      as
                                                                                      *mut memory_word
                                                                                      as
                                                                                      *mut libc::c_ushort).offset((i
                                                                                                                       -
                                                                                                                       l)
                                                                                                                      as
                                                                                                                      isize)
                                                                                    =
                                                                                    *(&mut *mem.offset((ha
                                                                                                            +
                                                                                                            6i32)
                                                                                                           as
                                                                                                           isize)
                                                                                          as
                                                                                          *mut memory_word
                                                                                          as
                                                                                          *mut libc::c_ushort).offset(i
                                                                                                                          as
                                                                                                                          isize);
                                                                            i += 1
                                                                        }
                                                                        measure_native_node(&mut *mem.offset(q
                                                                                                                     as
                                                                                                                     isize)
                                                                                                    as
                                                                                                    *mut memory_word
                                                                                                    as
                                                                                                    *mut libc::c_void,
                                                                                                ((*eqtb.offset((1i32
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
                                                                                                                    74i32)
                                                                                                                   as
                                                                                                                   isize)).b32.s1
                                                                                                     >
                                                                                                     0i32)
                                                                                                    as
                                                                                                    libc::c_int);
                                                                        (*mem.offset(q as isize))
                                                                            .b32
                                                                            .s1 = (*mem
                                                                            .offset(ha as isize))
                                                                        .b32
                                                                        .s1;
                                                                        (*mem
                                                                            .offset(ha as isize))
                                                                        .b32
                                                                        .s1 = q;
                                                                        (*mem.offset(
                                                                            (ha + 4i32) as isize,
                                                                        ))
                                                                        .b16
                                                                        .s1 = l as u16;
                                                                        measure_native_node(&mut *mem.offset(ha
                                                                                                                     as
                                                                                                                     isize)
                                                                                                    as
                                                                                                    *mut memory_word
                                                                                                    as
                                                                                                    *mut libc::c_void,
                                                                                                ((*eqtb.offset((1i32
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
                                                                                                                    74i32)
                                                                                                                   as
                                                                                                                   isize)).b32.s1
                                                                                                     >
                                                                                                     0i32)
                                                                                                    as
                                                                                                    libc::c_int);
                                                                        ha = (*mem
                                                                            .offset(ha as isize))
                                                                        .b32
                                                                        .s1;
                                                                        break;
                                                                    } else {
                                                                        if hn
                                                                                   as
                                                                                   libc::c_int
                                                                                   ==
                                                                                   max_hyphenatable_length()
                                                                               {
                                                                                break
                                                                                    'c_31290
                                                                                    ;
                                                                            }
                                                                        hn += 1;
                                                                        if (c as libc::c_long)
                                                                            < 65536
                                                                        {
                                                                            hu[hn as usize] = c;
                                                                            hc[hn as usize] = hc[0]
                                                                        } else {
                                                                            hu[hn as usize] = ((c
                                                                                as libc::c_long
                                                                                - 65536)
                                                                                / 1024i32
                                                                                    as libc::c_long
                                                                                + 0xd800i32
                                                                                    as libc::c_long)
                                                                                as int32_t;
                                                                            hc[hn as usize] = ((hc
                                                                                [0]
                                                                                as libc::c_long
                                                                                - 65536)
                                                                                / 1024i32
                                                                                    as libc::c_long
                                                                                + 0xd800i32
                                                                                    as libc::c_long)
                                                                                as int32_t;
                                                                            hn += 1;
                                                                            hu[hn as usize] = c
                                                                                % 1024i32
                                                                                + 0xdc00i32;
                                                                            hc[hn as usize] = hc[0]
                                                                                % 1024i32
                                                                                + 0xdc00i32;
                                                                            l += 1
                                                                        }
                                                                        hyf_bchar = 65536i32
                                                                    }
                                                                    l += 1
                                                                }
                                                            }
                                                            current_block = 4362442400146949691;
                                                        }
                                                    }
                                                } else {
                                                    /*931: skip to node hb, putting letters into hu and hc */
                                                    hn = 0i32 as small_number;
                                                    's_1342: loop {
                                                        if is_char_node(s) {
                                                            if (*mem.offset(s as isize)).b16.s1
                                                                as libc::c_int
                                                                != hf
                                                            {
                                                                break;
                                                            }
                                                            hyf_bchar =
                                                                (*mem.offset(s as isize)).b16.s0
                                                                    as int32_t;
                                                            c = hyf_bchar;
                                                            if hyph_index == 0i32 || c > 255i32 {
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
                                                                        + c)
                                                                        as isize,
                                                                ))
                                                                .b32
                                                                .s1
                                                            } else if *trie_trc
                                                                .offset((hyph_index + c) as isize)
                                                                as libc::c_int
                                                                != c
                                                            {
                                                                hc[0] = 0i32
                                                            } else {
                                                                hc[0] = *trie_tro.offset(
                                                                    (hyph_index + c) as isize,
                                                                )
                                                            }
                                                            if hc[0] == 0i32 {
                                                                break;
                                                            }
                                                            if hc[0] > max_hyph_char {
                                                                break;
                                                            }
                                                            if hn as libc::c_int
                                                                == max_hyphenatable_length()
                                                            {
                                                                break;
                                                            }
                                                            hb = s;
                                                            hn += 1;
                                                            hu[hn as usize] = c;
                                                            hc[hn as usize] = hc[0];
                                                            hyf_bchar = 65536i32
                                                        } else if (*mem.offset(s as isize)).b16.s1
                                                            as libc::c_int
                                                            == 6i32
                                                        {
                                                            /*932: move the characters of a ligature node to hu and hc; but goto done3
                                                             * if they are not all letters. */
                                                            if (*mem.offset((s + 1i32) as isize))
                                                                .b16
                                                                .s1
                                                                as libc::c_int
                                                                != hf
                                                            {
                                                                break;
                                                            }
                                                            j = hn;
                                                            q = (*mem.offset((s + 1i32) as isize))
                                                                .b32
                                                                .s1;
                                                            if q > -0xfffffffi32 {
                                                                hyf_bchar =
                                                                    (*mem.offset(q as isize)).b16.s0
                                                                        as int32_t
                                                            }
                                                            while q > -0xfffffffi32 {
                                                                c = (*mem.offset(q as isize)).b16.s0
                                                                    as UnicodeScalar;
                                                                if hyph_index == 0i32 || c > 255i32
                                                                {
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
                                                                            + c)
                                                                            as isize,
                                                                    ))
                                                                    .b32
                                                                    .s1
                                                                } else if *trie_trc.offset(
                                                                    (hyph_index + c) as isize,
                                                                )
                                                                    as libc::c_int
                                                                    != c
                                                                {
                                                                    hc[0] = 0i32
                                                                } else {
                                                                    hc[0] = *trie_tro.offset(
                                                                        (hyph_index + c) as isize,
                                                                    )
                                                                }
                                                                if hc[0] == 0i32 {
                                                                    break 's_1342;
                                                                }
                                                                if hc[0] > max_hyph_char {
                                                                    break 's_1342;
                                                                }
                                                                if j as libc::c_int
                                                                    == max_hyphenatable_length()
                                                                {
                                                                    break 's_1342;
                                                                }
                                                                j += 1;
                                                                hu[j as usize] = c;
                                                                hc[j as usize] = hc[0];
                                                                q = (*mem.offset(q as isize)).b32.s1
                                                            }
                                                            hb = s;
                                                            hn = j;
                                                            if (*mem.offset(s as isize)).b16.s0
                                                                as libc::c_int
                                                                & 1i32
                                                                != 0
                                                            {
                                                                hyf_bchar =
                                                                    *font_bchar.offset(hf as isize)
                                                            } else {
                                                                hyf_bchar = 65536i32
                                                            }
                                                        /*:932*/
                                                        } else {
                                                            if !((*mem.offset(s as isize)).b16.s1
                                                                as libc::c_int
                                                                == 11i32
                                                                && (*mem.offset(s as isize)).b16.s0
                                                                    as libc::c_int
                                                                    == 0i32)
                                                            {
                                                                break;
                                                            }
                                                            hb = s;
                                                            hyf_bchar =
                                                                *font_bchar.offset(hf as isize)
                                                        }
                                                        s = (*mem.offset(s as isize)).b32.s1
                                                    }
                                                    current_block = 4362442400146949691;
                                                }
                                                match current_block {
                                                    8166967358843938227 => {}
                                                    _ =>
                                                    /*933: check that the nodes following hb permit
                                                     * hyphenation and that at least l_hyf + r_hyf letters
                                                     * have been found, otherwise goto done1 */
                                                    {
                                                        if !((hn as libc::c_int) < l_hyf + r_hyf) {
                                                            loop {
                                                                if !is_char_node(s) {
                                                                    match (*mem.offset(s as isize))
                                                                        .b16
                                                                        .s1
                                                                        as libc::c_int
                                                                    {
                                                                        6 => {}
                                                                        11 => {
                                                                            current_block =
                                                                                5935670669791948619;
                                                                            match current_block
                                                                                {
                                                                                2529459302156174429
                                                                                =>
                                                                                {
                                                                                    if (*mem.offset(s
                                                                                                        as
                                                                                                        isize)).b16.s0
                                                                                           as
                                                                                           libc::c_int
                                                                                           >=
                                                                                           4i32
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            16848571710846909653;
                                                                                        break
                                                                                            ;
                                                                                    } else {
                                                                                        current_block
                                                                                            =
                                                                                            8166967358843938227;
                                                                                        break
                                                                                            ;
                                                                                    }
                                                                                }
                                                                                _
                                                                                =>
                                                                                {
                                                                                    if (*mem.offset(s
                                                                                                        as
                                                                                                        isize)).b16.s0
                                                                                           as
                                                                                           libc::c_int
                                                                                           !=
                                                                                           0i32
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            16848571710846909653;
                                                                                        break
                                                                                            ;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        8 | 10 | 12 | 3 | 5 | 4 => {
                                                                            current_block
                                                                                =
                                                                                16848571710846909653;
                                                                            break;
                                                                        }
                                                                        9 => {
                                                                            current_block =
                                                                                2529459302156174429;
                                                                            match current_block
                                                                                {
                                                                                2529459302156174429
                                                                                =>
                                                                                {
                                                                                    if (*mem.offset(s
                                                                                                        as
                                                                                                        isize)).b16.s0
                                                                                           as
                                                                                           libc::c_int
                                                                                           >=
                                                                                           4i32
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            16848571710846909653;
                                                                                        break
                                                                                            ;
                                                                                    } else {
                                                                                        current_block
                                                                                            =
                                                                                            8166967358843938227;
                                                                                        break
                                                                                            ;
                                                                                    }
                                                                                }
                                                                                _
                                                                                =>
                                                                                {
                                                                                    if (*mem.offset(s
                                                                                                        as
                                                                                                        isize)).b16.s0
                                                                                           as
                                                                                           libc::c_int
                                                                                           !=
                                                                                           0i32
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            16848571710846909653;
                                                                                        break
                                                                                            ;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            current_block =
                                                                                8166967358843938227;
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                                s = (*mem.offset(s as isize)).b32.s1
                                                            }
                                                            match current_block {
                                                                8166967358843938227 => {}
                                                                _ => {
                                                                    /*:933*/
                                                                    hyphenate();
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
                11 => {
                    /* ... resuming 895 ... */
                    if (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int == 1i32 {
                        if (!is_char_node((*mem.offset(cur_p as isize)).b32.s1) as libc::c_int)
                            < hi_mem_min
                            && auto_breaking as libc::c_int != 0
                        {
                            if (*mem.offset((*mem.offset(cur_p as isize)).b32.s1 as isize))
                                .b16
                                .s1 as libc::c_int
                                == 10i32
                            {
                                try_break(0i32, 0i32 as small_number);
                            }
                        }
                        active_width[1] += (*mem.offset((cur_p + 1i32) as isize)).b32.s1
                    } else {
                        active_width[1] += (*mem.offset((cur_p + 1i32) as isize)).b32.s1
                    }
                }
                6 => {
                    f = (*mem.offset((cur_p + 1i32) as isize)).b16.s1 as internal_font_number;
                    xtx_ligature_present = 1i32 != 0;
                    active_width[1] += (*font_info.offset(
                        (*width_base.offset(f as isize)
                            + (*font_info.offset(
                                (*char_base.offset(f as isize)
                                    + effective_char(
                                        1i32 != 0,
                                        f,
                                        (*mem.offset((cur_p + 1i32) as isize)).b16.s0,
                                    )) as isize,
                            ))
                            .b16
                            .s3 as libc::c_int) as isize,
                    ))
                    .b32
                    .s1
                }
                7 => {
                    /*898: try to break after a discretionary fragment, then goto done5 */
                    s = (*mem.offset((cur_p + 1i32) as isize)).b32.s0;
                    disc_width = 0i32;
                    if s == -0xfffffffi32 {
                        try_break(
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
                                    + 4i32) as isize,
                            ))
                            .b32
                            .s1,
                            1i32 as small_number,
                        );
                    } else {
                        loop {
                            /*899:*/
                            if is_char_node(s) {
                                let mut eff_char_0: int32_t = 0; /*:898 big DISC_NODE case */
                                f = (*mem.offset(s as isize)).b16.s1 as internal_font_number;
                                eff_char_0 =
                                    effective_char(1i32 != 0, f, (*mem.offset(s as isize)).b16.s0);
                                disc_width += (*font_info.offset(
                                    (*width_base.offset(f as isize)
                                        + (*font_info.offset(
                                            (*char_base.offset(f as isize) + eff_char_0) as isize,
                                        ))
                                        .b16
                                        .s3
                                            as libc::c_int)
                                        as isize,
                                ))
                                .b32
                                .s1
                            } else {
                                match (*mem.offset(s as isize)).b16.s1 as libc::c_int {
                                    6 => {
                                        let mut eff_char_1: int32_t = 0;
                                        f = (*mem.offset((s + 1i32) as isize)).b16.s1
                                            as internal_font_number;
                                        xtx_ligature_present = 1i32 != 0;
                                        eff_char_1 = effective_char(
                                            1i32 != 0,
                                            f,
                                            (*mem.offset((s + 1i32) as isize)).b16.s0,
                                        );
                                        disc_width += (*font_info.offset(
                                            (*width_base.offset(f as isize)
                                                + (*font_info.offset(
                                                    (*char_base.offset(f as isize) + eff_char_1)
                                                        as isize,
                                                ))
                                                .b16
                                                .s3
                                                    as libc::c_int)
                                                as isize,
                                        ))
                                        .b32
                                        .s1
                                    }
                                    0 | 1 | 2 | 11 => {
                                        disc_width += (*mem.offset((s + 1i32) as isize)).b32.s1
                                    }
                                    8 => {
                                        if (*mem.offset(s as isize)).b16.s0 as libc::c_int == 40i32
                                            || (*mem.offset(s as isize)).b16.s0 as libc::c_int
                                                == 41i32
                                            || (*mem.offset(s as isize)).b16.s0 as libc::c_int
                                                == 42i32
                                            || (*mem.offset(s as isize)).b16.s0 as libc::c_int
                                                == 43i32
                                            || (*mem.offset(s as isize)).b16.s0 as libc::c_int
                                                == 44i32
                                        {
                                            disc_width += (*mem.offset((s + 1i32) as isize)).b32.s1
                                        } else {
                                            confusion(
                                                b"disc3a\x00" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                    }
                                    _ => {
                                        confusion(b"disc3\x00" as *const u8 as *const libc::c_char);
                                    }
                                }
                            }
                            s = (*mem.offset(s as isize)).b32.s1;
                            if !(s != -0xfffffffi32) {
                                break;
                            }
                        }
                        active_width[1] += disc_width;
                        try_break(
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
                                    + 3i32) as isize,
                            ))
                            .b32
                            .s1,
                            1i32 as small_number,
                        );
                        active_width[1] -= disc_width
                    }
                    r = (*mem.offset(cur_p as isize)).b16.s0 as int32_t;
                    s = (*mem.offset(cur_p as isize)).b32.s1;
                    while r > 0i32 {
                        if is_char_node(s) {
                            let mut eff_char_2: int32_t = 0;
                            f = (*mem.offset(s as isize)).b16.s1 as internal_font_number;
                            eff_char_2 =
                                effective_char(1i32 != 0, f, (*mem.offset(s as isize)).b16.s0);
                            active_width[1] += (*font_info.offset(
                                (*width_base.offset(f as isize)
                                    + (*font_info.offset(
                                        (*char_base.offset(f as isize) + eff_char_2) as isize,
                                    ))
                                    .b16
                                    .s3 as libc::c_int) as isize,
                            ))
                            .b32
                            .s1
                        } else {
                            match (*mem.offset(s as isize)).b16.s1 as libc::c_int {
                                6 => {
                                    let mut eff_char_3: int32_t = 0;
                                    f = (*mem.offset((s + 1i32) as isize)).b16.s1
                                        as internal_font_number;
                                    xtx_ligature_present = 1i32 != 0;
                                    eff_char_3 = effective_char(
                                        1i32 != 0,
                                        f,
                                        (*mem.offset((s + 1i32) as isize)).b16.s0,
                                    );
                                    active_width[1] += (*font_info.offset(
                                        (*width_base.offset(f as isize)
                                            + (*font_info.offset(
                                                (*char_base.offset(f as isize) + eff_char_3)
                                                    as isize,
                                            ))
                                            .b16
                                            .s3
                                                as libc::c_int)
                                            as isize,
                                    ))
                                    .b32
                                    .s1
                                }
                                0 | 1 | 2 | 11 => {
                                    active_width[1] += (*mem.offset((s + 1i32) as isize)).b32.s1
                                }
                                8 => {
                                    if (*mem.offset(s as isize)).b16.s0 as libc::c_int == 40i32
                                        || (*mem.offset(s as isize)).b16.s0 as libc::c_int == 41i32
                                        || (*mem.offset(s as isize)).b16.s0 as libc::c_int == 42i32
                                        || (*mem.offset(s as isize)).b16.s0 as libc::c_int == 43i32
                                        || (*mem.offset(s as isize)).b16.s0 as libc::c_int == 44i32
                                    {
                                        active_width[1] += (*mem.offset((s + 1i32) as isize)).b32.s1
                                    } else {
                                        confusion(
                                            b"disc4a\x00" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                }
                                _ => {
                                    confusion(b"disc4\x00" as *const u8 as *const libc::c_char);
                                }
                            }
                        }
                        r -= 1;
                        s = (*mem.offset(s as isize)).b32.s1
                    }
                    global_prev_p = cur_p;
                    prev_p = global_prev_p;
                    cur_p = s;
                    continue;
                }
                9 => {
                    if ((*mem.offset(cur_p as isize)).b16.s0 as libc::c_int) < 4i32 {
                        auto_breaking =
                            (*mem.offset(cur_p as isize)).b16.s0 as libc::c_int & 1i32 != 0
                    }
                    if !is_char_node((*mem.offset(cur_p as isize)).b32.s1)
                        && auto_breaking as libc::c_int != 0
                    {
                        if (*mem.offset((*mem.offset(cur_p as isize)).b32.s1 as isize))
                            .b16
                            .s1 as libc::c_int
                            == 10i32
                        {
                            try_break(0i32, 0i32 as small_number);
                        }
                    }
                    active_width[1] += (*mem.offset((cur_p + 1i32) as isize)).b32.s1
                }
                12 => {
                    try_break(
                        (*mem.offset((cur_p + 1i32) as isize)).b32.s1,
                        0i32 as small_number,
                    );
                }
                4 | 3 | 5 => {}
                _ => {
                    confusion(b"paragraph\x00" as *const u8 as *const libc::c_char);
                }
            }
            global_prev_p = cur_p;
            prev_p = global_prev_p;
            cur_p = (*mem.offset(cur_p as isize)).b32.s1
        }
        if cur_p == -0xfffffffi32 {
            /*902: "Try the final line break at the end of the paragraph, and
             * goto done if the desired breakpoints have been found." */
            try_break(-10000i32, 1i32 as small_number);
            if (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1 != 4999999i32 - 7i32 {
                /*:902*/
                /*903:*/
                r = (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1; /*:903*/
                fewest_demerits = 0x3fffffffi32; /*904:*/
                loop {
                    if (*mem.offset(r as isize)).b16.s1 as libc::c_int != 2i32 {
                        if (*mem.offset((r + 2i32) as isize)).b32.s1 < fewest_demerits {
                            fewest_demerits = (*mem.offset((r + 2i32) as isize)).b32.s1; /*:904*/
                            best_bet = r
                        }
                    }
                    r = (*mem.offset(r as isize)).b32.s1;
                    if !(r != 4999999i32 - 7i32) {
                        break;
                    }
                }
                best_line = (*mem.offset((best_bet + 1i32) as isize)).b32.s0;
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
                        + 19i32) as isize,
                ))
                .b32
                .s1 == 0i32
                {
                    break;
                }
                r = (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1;
                actual_looseness = 0i32;
                loop {
                    if (*mem.offset(r as isize)).b16.s1 as libc::c_int != 2i32 {
                        line_diff = (*mem.offset((r + 1i32) as isize)).b32.s0 - best_line;
                        if line_diff < actual_looseness
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
                                    + 19i32) as isize,
                            ))
                            .b32
                            .s1 <= line_diff
                            || line_diff > actual_looseness
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
                                        + 19i32) as isize,
                                ))
                                .b32
                                .s1 >= line_diff
                        {
                            best_bet = r;
                            actual_looseness = line_diff;
                            fewest_demerits = (*mem.offset((r + 2i32) as isize)).b32.s1
                        } else if line_diff == actual_looseness
                            && (*mem.offset((r + 2i32) as isize)).b32.s1 < fewest_demerits
                        {
                            best_bet = r;
                            fewest_demerits = (*mem.offset((r + 2i32) as isize)).b32.s1
                        }
                    }
                    r = (*mem.offset(r as isize)).b32.s1;
                    if !(r != 4999999i32 - 7i32) {
                        break;
                    }
                }
                best_line = (*mem.offset((best_bet + 1i32) as isize)).b32.s0;
                if actual_looseness
                    == (*eqtb.offset(
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
                            + 19i32) as isize,
                    ))
                    .b32
                    .s1
                    || final_pass as libc::c_int != 0
                {
                    break;
                }
            }
        }
        /*894: clean up the memory by removing the break nodes */
        q = (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1;
        while q != 4999999i32 - 7i32 {
            cur_p = (*mem.offset(q as isize)).b32.s1;
            if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 2i32 {
                free_node(q, 7i32);
            } else {
                free_node(q, active_node_size as int32_t);
            }
            q = cur_p
        }
        q = passive;
        while q != -0xfffffffi32 {
            cur_p = (*mem.offset(q as isize)).b32.s1;
            free_node(q, 2i32);
            q = cur_p
        }
        /* ... resuming 892 ... */
        if !second_pass {
            threshold = (*eqtb.offset(
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
            .s1;
            second_pass = 1i32 != 0;
            final_pass = (*eqtb.offset(
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
                    + 20i32) as isize,
            ))
            .b32
            .s1 <= 0i32
        } else {
            background[2] = background[2]
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
                        + 20i32) as isize,
                ))
                .b32
                .s1;
            final_pass = 1i32 != 0
        }
    }
    if do_last_line_fit {
        /*1641:*/
        if (*mem.offset((best_bet + 3i32) as isize)).b32.s1 == 0i32 {
            do_last_line_fit = 0i32 != 0
        } else {
            q = new_spec((*mem.offset((last_line_fill + 1i32) as isize)).b32.s0);
            delete_glue_ref((*mem.offset((last_line_fill + 1i32) as isize)).b32.s0);
            let ref mut fresh4 = (*mem.offset((q + 1i32) as isize)).b32.s1;
            *fresh4 += (*mem.offset((best_bet + 3i32) as isize)).b32.s1
                - (*mem.offset((best_bet + 4i32) as isize)).b32.s1;
            (*mem.offset((q + 2i32) as isize)).b32.s1 = 0i32;
            (*mem.offset((last_line_fill + 1i32) as isize)).b32.s0 = q
        }
    }
    post_line_break(d);
    /* Clean up by removing break nodes (894, again) */
    q = (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1;
    while q != 4999999i32 - 7i32 {
        let mut next: int32_t = (*mem.offset(q as isize)).b32.s1;
        if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 2i32 {
            free_node(q, 7i32);
        } else {
            free_node(q, active_node_size as int32_t);
        }
        q = next
    }
    q = passive;
    while q != -0xfffffffi32 {
        let mut next_0: int32_t = (*mem.offset(q as isize)).b32.s1;
        free_node(q, 2i32);
        q = next_0
    }
    /* All done */
    pack_begin_line = 0i32;
}
/* This was just separated out to prevent line_break() from becoming
 * proposterously long. */
unsafe extern "C" fn post_line_break(mut d: bool) {
    let mut q: int32_t = 0;
    let mut r: int32_t = 0;
    let mut s: int32_t = 0;
    let mut p: int32_t = 0;
    let mut k: int32_t = 0;
    let mut w: scaled_t = 0;
    let mut glue_break: bool = false;
    let mut ptmp: int32_t = 0;
    let mut disc_break: bool = false;
    let mut post_disc_break: bool = false;
    let mut cur_width: scaled_t = 0;
    let mut cur_indent: scaled_t = 0;
    let mut t: u16 = 0;
    let mut pen: int32_t = 0;
    let mut cur_line: int32_t = 0;
    let mut LR_ptr: int32_t = 0;
    LR_ptr = cur_list.eTeX_aux;
    /* Reverse the list of break nodes (907) */
    q = (*mem.offset((best_bet + 1i32) as isize)).b32.s1; /*:907*/
    cur_p = -0xfffffffi32;
    loop {
        r = q;
        q = (*mem.offset((q + 1i32) as isize)).b32.s0;
        (*mem.offset((r + 1i32) as isize)).b32.s0 = cur_p;
        cur_p = r;
        if !(q != -0xfffffffi32) {
            break;
        }
    }
    cur_line = cur_list.prev_graf + 1i32;
    loop  {
        /* 909: justify the line ending at breakpoint cur_p and append it to
         * the current vertical list, with associated penalties and
         * insertions. The current line starts a TEMP_HEAD.link and ends at
         * cur_p.cur_break.
         **/
        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              71i32) as isize)).b32.s1 > 0i32 {
            /*1494:*/
            q = (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1;
            if LR_ptr != -0xfffffffi32 {
                temp_ptr = LR_ptr;
                r = q;
                loop  {
                    s =
                        new_math(0i32,
                                 ((*mem.offset(temp_ptr as isize)).b32.s0 -
                                      1i32) as small_number);
                    (*mem.offset(s as isize)).b32.s1 = r;
                    r = s;
                    temp_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                    if !(temp_ptr != -0xfffffffi32) { break ; }
                }
                (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 = r
            }
            while q != (*mem.offset((cur_p + 1i32) as isize)).b32.s1 {
                if q < hi_mem_min &&
                       (*mem.offset(q as isize)).b16.s1 as libc::c_int == 9i32
                   {
                    /*1495:*/
                    if (*mem.offset(q as isize)).b16.s0 as libc::c_int & 1i32
                           != 0 {
                        if LR_ptr != -0xfffffffi32 &&
                               (*mem.offset(LR_ptr as isize)).b32.s0 ==
                                   4i32 *
                                       ((*mem.offset(q as isize)).b16.s0 as
                                            libc::c_int / 4i32) + 3i32 {
                            temp_ptr = LR_ptr;
                            LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                            (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                            avail = temp_ptr
                        }
                    } else {
                        temp_ptr = get_avail();
                        (*mem.offset(temp_ptr as isize)).b32.s0 =
                            4i32 *
                                ((*mem.offset(q as isize)).b16.s0 as
                                     libc::c_int / 4i32) + 3i32;
                        (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                        LR_ptr = temp_ptr
                    }
                }
                q = (*mem.offset(q as isize)).b32.s1
            }
        }
        /* 910: "Modify the end of the line to reflect the nature of the break
         * and to include \rightskip; also set the proper value of
         * disc_break" */
        q = (*mem.offset((cur_p + 1i32) as isize)).b32.s1;
        disc_break = 0i32 != 0;
        post_disc_break = 0i32 != 0;
        glue_break = 0i32 != 0;
        if q == -0xfffffffi32 {
            q = 4999999i32 - 3i32;
            while (*mem.offset(q as isize)).b32.s1 != -0xfffffffi32 {
                q = (*mem.offset(q as isize)).b32.s1
            }
        } else if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 10i32 {
            delete_glue_ref((*mem.offset((q + 1i32) as isize)).b32.s0);
            (*mem.offset((q + 1i32) as isize)).b32.s0 =
                (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                   12i32 + 9000i32 + 1i32 + 1i32 + 8i32) as
                                  isize)).b32.s1;
            (*mem.offset(q as isize)).b16.s0 = (8i32 + 1i32) as u16;
            let ref mut fresh5 =
                (*mem.offset((*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                (0x10ffffi32 + 1i32) + 1i32 +
                                                15000i32 + 12i32 + 9000i32 +
                                                1i32 + 1i32 + 8i32) as
                                               isize)).b32.s1 as
                                 isize)).b32.s1;
            *fresh5 += 1;
            glue_break = 1i32 != 0
        } else if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 7i32 {
            /*911:*/
            t = (*mem.offset(q as isize)).b16.s0;
            if t as libc::c_int == 0i32 {
                r = (*mem.offset(q as isize)).b32.s1
            } else {
                r = q;
                while t as libc::c_int > 1i32 {
                    r = (*mem.offset(r as isize)).b32.s1;
                    t = t.wrapping_sub(1)
                }
                s = (*mem.offset(r as isize)).b32.s1;
                r = (*mem.offset(s as isize)).b32.s1;
                (*mem.offset(s as isize)).b32.s1 = -0xfffffffi32;
                flush_node_list((*mem.offset(q as isize)).b32.s1);
                (*mem.offset(q as isize)).b16.s0 = 0i32 as u16
            }
            if (*mem.offset((q + 1i32) as isize)).b32.s1 != -0xfffffffi32 {
                /*913:*/
                s = (*mem.offset((q + 1i32) as isize)).b32.s1;
                while (*mem.offset(s as isize)).b32.s1 != -0xfffffffi32 {
                    s = (*mem.offset(s as isize)).b32.s1
                }
                (*mem.offset(s as isize)).b32.s1 = r;
                r = (*mem.offset((q + 1i32) as isize)).b32.s1;
                (*mem.offset((q + 1i32) as isize)).b32.s1 = -0xfffffffi32;
                post_disc_break = 1i32 != 0
            }
            if (*mem.offset((q + 1i32) as isize)).b32.s0 != -0xfffffffi32 {
                /*914:*/
                s = (*mem.offset((q + 1i32) as isize)).b32.s0;
                (*mem.offset(q as isize)).b32.s1 = s;
                while (*mem.offset(s as isize)).b32.s1 != -0xfffffffi32 {
                    s = (*mem.offset(s as isize)).b32.s1
                }
                (*mem.offset((q + 1i32) as isize)).b32.s0 = -0xfffffffi32;
                q = s
            }
            (*mem.offset(q as isize)).b32.s1 = r;
            disc_break = 1i32 != 0
        } else if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 11i32 {
            (*mem.offset((q + 1i32) as isize)).b32.s1 = 0i32
        } else if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 9i32 {
            (*mem.offset((q + 1i32) as isize)).b32.s1 = 0i32;
            if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                  (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                  12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                  256i32 + 256i32 + 13i32 + 256i32 + 4i32 +
                                  256i32 + 1i32 + 3i32 * 256i32 +
                                  (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32)
                                  + (0x10ffffi32 + 1i32) +
                                  (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32)
                                  + (0x10ffffi32 + 1i32) + 71i32) as
                                 isize)).b32.s1 > 0i32 {
                /*1495:*/
                if (*mem.offset(q as isize)).b16.s0 as libc::c_int & 1i32 != 0
                   {
                    if LR_ptr != -0xfffffffi32 &&
                           (*mem.offset(LR_ptr as isize)).b32.s0 ==
                               4i32 *
                                   ((*mem.offset(q as isize)).b16.s0 as
                                        libc::c_int / 4i32) + 3i32 {
                        temp_ptr = LR_ptr;
                        LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                        (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                        avail = temp_ptr
                    }
                } else {
                    temp_ptr = get_avail();
                    (*mem.offset(temp_ptr as isize)).b32.s0 =
                        4i32 *
                            ((*mem.offset(q as isize)).b16.s0 as libc::c_int /
                                 4i32) + 3i32;
                    (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                    LR_ptr = temp_ptr
                }
            }
        }
        /* "at this point q is the rightmost breakpoint; the only exception is
         * the case of a discretionary break with non-empty pre_break -- then
         * q has been changed to the last node of the pre-break list" */
        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              70i32) as isize)).b32.s1 > 0i32 {
            if disc_break as libc::c_int != 0 &&
                   (is_char_node(q) as libc::c_int != 0 ||
                        (*mem.offset(q as isize)).b16.s1 as libc::c_int !=
                            7i32) {
                p = q; /*:915*/
                ptmp = p
            } else {
                p =
                    prev_rightmost((*mem.offset((4999999i32 - 3i32) as
                                                    isize)).b32.s1, q);
                ptmp = p;
                p =
                    find_protchar_right((*mem.offset((4999999i32 - 3i32) as
                                                         isize)).b32.s1, p)
            }
            w = char_pw(p, 1i32 as small_number);
            if w != 0i32 {
                k =
                    new_margin_kern(-w, last_rightmost_char,
                                    1i32 as small_number);
                (*mem.offset(k as isize)).b32.s1 =
                    (*mem.offset(ptmp as isize)).b32.s1;
                (*mem.offset(ptmp as isize)).b32.s1 = k;
                if ptmp == q { q = (*mem.offset(q as isize)).b32.s1 }
            }
        }
        if !glue_break {
            r = new_param_glue(8i32 as small_number);
            (*mem.offset(r as isize)).b32.s1 =
                (*mem.offset(q as isize)).b32.s1;
            (*mem.offset(q as isize)).b32.s1 = r;
            q = r
        }
        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              71i32) as isize)).b32.s1 > 0i32 {
            /*1496:*/
            if LR_ptr != -0xfffffffi32 {
                s = 4999999i32 - 3i32;
                r = (*mem.offset(s as isize)).b32.s1;
                while r != q { s = r; r = (*mem.offset(s as isize)).b32.s1 }
                r = LR_ptr;
                while r != -0xfffffffi32 {
                    temp_ptr =
                        new_math(0i32,
                                 (*mem.offset(r as isize)).b32.s0 as
                                     small_number);
                    (*mem.offset(s as isize)).b32.s1 = temp_ptr;
                    s = temp_ptr;
                    r = (*mem.offset(r as isize)).b32.s1
                }
                (*mem.offset(s as isize)).b32.s1 = q
            }
        }
        /* 916: Put \leftskip at the left and detach this line. */
        r = (*mem.offset(q as isize)).b32.s1;
        (*mem.offset(q as isize)).b32.s1 = -0xfffffffi32;
        q = (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1;
        (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 = r;
        /* "at this point q is the leftmost node; all discardable nodes have been discarded */
        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 19i32 + 256i32 + 256i32 + 13i32 + 256i32 +
                              4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              70i32) as isize)).b32.s1 > 0i32 {
            p = q;
            p = find_protchar_left(p, 0i32 != 0);
            w = char_pw(p, 0i32 as small_number);
            if w != 0i32 {
                k =
                    new_margin_kern(-w, last_leftmost_char,
                                    0i32 as small_number);
                (*mem.offset(k as isize)).b32.s1 = q;
                q = k
            }
        }
        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) +
                              1i32 + 15000i32 + 12i32 + 9000i32 + 1i32 + 1i32
                              + 7i32) as isize)).b32.s1 != 0i32 {
            r = new_param_glue(7i32 as small_number);
            (*mem.offset(r as isize)).b32.s1 = q;
            q = r
        }
        /* 918: q points to the hlist that represents the current line. Pack
         * it up at the right width. */
        if cur_line > last_special_line {
            cur_width = second_width;
            cur_indent = second_indent
        } else if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                     (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                     12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                     256i32 + 256i32 + 0i32) as isize)).b32.s1
                      == -0xfffffffi32 {
            cur_width = first_width;
            cur_indent = first_indent
        } else {
            /* These manual `mem` indices are in the original WEB code */
            cur_width =
                (*mem.offset(((*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) + 1i32 +
                                                 15000i32 + 12i32 + 9000i32 +
                                                 1i32 + 1i32 + 19i32 + 256i32
                                                 + 256i32 + 0i32) as
                                                isize)).b32.s1 +
                                  2i32 * cur_line) as isize)).b32.s1;
            cur_indent =
                (*mem.offset(((*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                 (0x10ffffi32 + 1i32) + 1i32 +
                                                 15000i32 + 12i32 + 9000i32 +
                                                 1i32 + 1i32 + 19i32 + 256i32
                                                 + 256i32 + 0i32) as
                                                isize)).b32.s1 +
                                  2i32 * cur_line - 1i32) as isize)).b32.s1
        }
        adjust_tail = 4999999i32 - 5i32;
        pre_adjust_tail = 4999999i32 - 14i32;
        /* Tectonic: in semantic pagination mode, set each "line" (really the
         * whole paragraph) at its natural width. */
        if semantic_pagination_enabled {
            just_box = hpack(q, 0i32, 1i32 as small_number)
        } else {
            just_box = hpack(q, cur_width, 0i32 as small_number)
        } /*:918*/
        (*mem.offset((just_box + 4i32) as isize)).b32.s1 = cur_indent;
        /* 917: append the new box to the current vertical list, followed
         * by any of its special nodes that were taken out */
        if 4999999i32 - 14i32 != pre_adjust_tail {
            (*mem.offset(cur_list.tail as isize)).b32.s1 =
                (*mem.offset((4999999i32 - 14i32) as isize)).b32.s1; /*:917*/
            cur_list.tail = pre_adjust_tail
        }
        pre_adjust_tail = -0xfffffffi32;
        append_to_vlist(just_box);
        if 4999999i32 - 5i32 != adjust_tail {
            (*mem.offset(cur_list.tail as isize)).b32.s1 =
                (*mem.offset((4999999i32 - 5i32) as isize)).b32.s1;
            cur_list.tail = adjust_tail
        }
        adjust_tail = -0xfffffffi32;
        /* 919: Set `pen` to all of the penalties relevant to this line. */
        if cur_line + 1i32 != best_line {
            q =
                (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                   12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                   256i32 + 256i32 + 13i32 + 256i32 + 0i32) as
                                  isize)).b32.s1;
            if q != -0xfffffffi32 {
                r = cur_line;
                if r > (*mem.offset((q + 1i32) as isize)).b32.s1 {
                    r = (*mem.offset((q + 1i32) as isize)).b32.s1
                }
                pen = (*mem.offset((q + r + 1i32) as isize)).b32.s1
            } else {
                pen =
                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 1i32 + 15000i32
                                       + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32
                                       + 256i32 + 256i32 + 13i32 + 256i32 +
                                       4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 13i32) as
                                      isize)).b32.s1
            }
            q =
                (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                   (0x10ffffi32 + 1i32) + 1i32 + 15000i32 +
                                   12i32 + 9000i32 + 1i32 + 1i32 + 19i32 +
                                   256i32 + 256i32 + 13i32 + 256i32 + 1i32) as
                                  isize)).b32.s1;
            if q != -0xfffffffi32 {
                r = cur_line - cur_list.prev_graf;
                if r > (*mem.offset((q + 1i32) as isize)).b32.s1 {
                    r = (*mem.offset((q + 1i32) as isize)).b32.s1
                }
                pen += (*mem.offset((q + r + 1i32) as isize)).b32.s1
            } else if cur_line == cur_list.prev_graf + 1i32 {
                pen +=
                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 1i32 + 15000i32
                                       + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32
                                       + 256i32 + 256i32 + 13i32 + 256i32 +
                                       4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 5i32) as
                                      isize)).b32.s1
            }
            if d {
                q =
                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 1i32 + 15000i32
                                       + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32
                                       + 256i32 + 256i32 + 13i32 + 256i32 +
                                       3i32) as isize)).b32.s1
            } else {
                q =
                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 1i32 + 15000i32
                                       + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32
                                       + 256i32 + 256i32 + 13i32 + 256i32 +
                                       2i32) as isize)).b32.s1
            }
            if q != -0xfffffffi32 {
                r = best_line - cur_line - 1i32;
                if r > (*mem.offset((q + 1i32) as isize)).b32.s1 {
                    r = (*mem.offset((q + 1i32) as isize)).b32.s1
                }
                pen += (*mem.offset((q + r + 1i32) as isize)).b32.s1
            } else if cur_line + 2i32 == best_line {
                if d {
                    pen +=
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
                                           (0x10ffffi32 + 1i32) + 7i32) as
                                          isize)).b32.s1
                } else {
                    pen +=
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
                                           (0x10ffffi32 + 1i32) + 6i32) as
                                          isize)).b32.s1
                }
            }
            if disc_break {
                pen +=
                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 1i32 + 15000i32
                                       + 12i32 + 9000i32 + 1i32 + 1i32 + 19i32
                                       + 256i32 + 256i32 + 13i32 + 256i32 +
                                       4i32 + 256i32 + 1i32 + 3i32 * 256i32 +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) +
                                       (0x10ffffi32 + 1i32) + 8i32) as
                                      isize)).b32.s1
            }
            if pen != 0i32 {
                r = new_penalty(pen);
                (*mem.offset(cur_list.tail as isize)).b32.s1 = r;
                cur_list.tail = r
            }
        }
        /* Done justifying this line. */
        cur_line += 1;
        cur_p = (*mem.offset((cur_p + 1i32) as isize)).b32.s0;
        if cur_p != -0xfffffffi32 {
            if !post_disc_break {
                /* 908: "prune unwanted nodes at the beginning of the next
                 * line". Delete glues, penalties, kerns, and math nodes at
                 * the beginning of the line, unless the node in question is
                 * the chosen breakpoint. */
                r = 4999999i32 - 3i32;
                loop  {
                    q = (*mem.offset(r as isize)).b32.s1;
                    if q == (*mem.offset((cur_p + 1i32) as isize)).b32.s1 {
                        break ;
                    }
                    if is_char_node(q) { break ; }
                    if is_non_discardable_node(q) { break ; }
                    if (*mem.offset(q as isize)).b16.s1 as libc::c_int ==
                           11i32 &&
                           (*mem.offset(q as isize)).b16.s0 as libc::c_int !=
                               1i32 &&
                           (*mem.offset(q as isize)).b16.s0 as libc::c_int !=
                               3i32 {
                        break ;
                    }
                    r = q;
                    if (*mem.offset(q as isize)).b16.s1 as libc::c_int == 9i32
                           &&
                           (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                              (0x10ffffi32 + 1i32) + 1i32 +
                                              15000i32 + 12i32 + 9000i32 +
                                              1i32 + 1i32 + 19i32 + 256i32 +
                                              256i32 + 13i32 + 256i32 + 4i32 +
                                              256i32 + 1i32 + 3i32 * 256i32 +
                                              (0x10ffffi32 + 1i32) +
                                              (0x10ffffi32 + 1i32) +
                                              (0x10ffffi32 + 1i32) +
                                              (0x10ffffi32 + 1i32) +
                                              (0x10ffffi32 + 1i32) +
                                              (0x10ffffi32 + 1i32) + 71i32) as
                                             isize)).b32.s1 > 0i32 {
                        /*1495:*/
                        if (*mem.offset(q as isize)).b16.s0 as libc::c_int &
                               1i32 != 0 {
                            if LR_ptr != -0xfffffffi32 &&
                                   (*mem.offset(LR_ptr as isize)).b32.s0 ==
                                       4i32 *
                                           ((*mem.offset(q as isize)).b16.s0
                                                as libc::c_int / 4i32) + 3i32
                               {
                                temp_ptr = LR_ptr;
                                LR_ptr =
                                    (*mem.offset(temp_ptr as isize)).b32.s1;
                                (*mem.offset(temp_ptr as isize)).b32.s1 =
                                    avail;
                                avail = temp_ptr
                            }
                        } else {
                            temp_ptr = get_avail();
                            (*mem.offset(temp_ptr as isize)).b32.s0 =
                                4i32 *
                                    ((*mem.offset(q as isize)).b16.s0 as
                                         libc::c_int / 4i32) + 3i32;
                            (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                            LR_ptr = temp_ptr
                        }
                    }
                }
                if r != 4999999i32 - 3i32 {
                    (*mem.offset(r as isize)).b32.s1 = -0xfffffffi32;
                    flush_node_list((*mem.offset((4999999i32 - 3i32) as
                                                     isize)).b32.s1);
                    (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 = q
                }
            }
        }
        if !(cur_p != -0xfffffffi32) { break ; }
    }
    if cur_line != best_line || (*mem.offset((4999999i32 - 3i32) as isize)).b32.s1 != -0xfffffffi32
    {
        confusion(b"line breaking\x00" as *const u8 as *const libc::c_char);
    }
    cur_list.prev_graf = best_line - 1i32;
    cur_list.eTeX_aux = LR_ptr;
}
/*858: "The heart of the line-breaking procedure is try_break, a subroutine
 * that tests if the current breakpoint cur_p is feasible, by running through
 * the active list to see what lines of text can be made from active nodes to
 * cur_p. If feasible breaks are possible, new break nodes are created. If
 * cur_p is too far from an active node, that node is deactivated. The
 * parameter pi to try_break is the penalty associated with a break at cur_p;
 * we have pi = eject_penalty if the break is forced, and pi = inf_penalty if
 * the break is illegal. The other parameter, break_type, is set to HYPHENATED
 * or UNHYPHENATED, depending on whether or not the current break is at a
 * disc_node. The end of a paragraph is also regarded as hyphenated; this case
 * is distinguishable by the condition cur_p = null." */
unsafe extern "C" fn try_break(mut pi: int32_t, mut break_type: small_number) {
    let mut current_block: u64;
    let mut r: int32_t = 0;
    let mut prev_r: int32_t = 0;
    let mut old_l: int32_t = 0;
    let mut no_break_yet: bool = false;
    let mut prev_prev_r: int32_t = -0xfffffffi32;
    let mut s: int32_t = 0;
    let mut q: int32_t = 0;
    let mut v: int32_t = 0;
    let mut t: int32_t = 0;
    let mut f: internal_font_number = 0;
    let mut l: int32_t = 0;
    let mut node_r_stays_active: bool = false;
    let mut line_width: scaled_t = 0i32;
    let mut fit_class: libc::c_uchar = 0;
    let mut b: int32_t = 0;
    let mut d: int32_t = 0;
    let mut artificial_demerits: bool = false;
    let mut shortfall: scaled_t = 0;
    let mut g: scaled_t = 0i32;
    /* Tectonic: no-op except at the end of the paragraph. We know we're at
     * the very end of the paragraph when cur_p is TEX_NULL. */
    if semantic_pagination_enabled as libc::c_int != 0 && cur_p != -0xfffffffi32 {
        return;
    }
    if abs(pi) >= 10000i32 {
        if pi > 0i32 {
            return;
        }
        pi = -10000i32
    }
    no_break_yet = 1i32 != 0;
    prev_r = 4999999i32 - 7i32;
    old_l = 0i32;
    cur_active_width[1] = active_width[1];
    cur_active_width[2] = active_width[2];
    cur_active_width[3] = active_width[3];
    cur_active_width[4] = active_width[4];
    cur_active_width[5] = active_width[5];
    cur_active_width[6] = active_width[6];
    loop {
        r = (*mem.offset(prev_r as isize)).b32.s1;
        /*861: "If node r is of type delta_node, update cur_active_width, set
         * prev_r and prev_prev_r, then goto continue" */
        if (*mem.offset(r as isize)).b16.s1 as libc::c_int == 2i32 {
            cur_active_width[1] += (*mem.offset((r + 1i32) as isize)).b32.s1;
            cur_active_width[2] += (*mem.offset((r + 2i32) as isize)).b32.s1;
            cur_active_width[3] += (*mem.offset((r + 3i32) as isize)).b32.s1;
            cur_active_width[4] += (*mem.offset((r + 4i32) as isize)).b32.s1;
            cur_active_width[5] += (*mem.offset((r + 5i32) as isize)).b32.s1;
            cur_active_width[6] += (*mem.offset((r + 6i32) as isize)).b32.s1;
            prev_prev_r = prev_r;
            prev_r = r
        } else {
            /*864: "If a line number class has ended, create new active nodes for
             * the best feasible breaks in that class; then return if r =
             * last_active, otherwise compute the new line_width." */
            l = (*mem.offset((r + 1i32) as isize)).b32.s0;
            if l > old_l {
                /* "now we are no longer in the inner loop" */
                if minimum_demerits < 0x3fffffffi32
                    && (old_l != easy_line || r == 4999999i32 - 7i32)
                {
                    /*865: "Create new active nodes for the best feasible breaks
                     * just found." */
                    if no_break_yet {
                        /*866: "Compute the values of break_width". */
                        no_break_yet = 0i32 != 0;
                        break_width[1] = background[1];
                        break_width[2] = background[2];
                        break_width[3] = background[3];
                        break_width[4] = background[4];
                        break_width[5] = background[5];
                        break_width[6] = background[6];
                        s = cur_p;
                        if break_type as libc::c_int > 0i32 {
                            /*869: "Compute the discretionary break_width values" */
                            if cur_p != -0xfffffffi32 {
                                t = (*mem.offset(cur_p as isize)).b16.s0 as int32_t;
                                v = cur_p;
                                s = (*mem.offset((cur_p + 1i32) as isize)).b32.s1;
                                while t > 0i32 {
                                    t -= 1;
                                    v = (*mem.offset(v as isize)).b32.s1;
                                    /*870: "subtract the width of node v from break_width" */
                                    if is_char_node(v) {
                                        let mut eff_char: int32_t = 0;
                                        f = (*mem.offset(v as isize)).b16.s1
                                            as internal_font_number;
                                        eff_char = effective_char(
                                            1i32 != 0,
                                            f,
                                            (*mem.offset(v as isize)).b16.s0,
                                        );
                                        break_width[1] -= (*font_info.offset(
                                            (*width_base.offset(f as isize)
                                                + (*font_info.offset(
                                                    (*char_base.offset(f as isize) + eff_char)
                                                        as isize,
                                                ))
                                                .b16
                                                .s3
                                                    as libc::c_int)
                                                as isize,
                                        ))
                                        .b32
                                        .s1
                                    } else {
                                        match (*mem.offset(v as isize)).b16.s1 as libc::c_int {
                                            6 => {
                                                let mut eff_char_0: int32_t = 0;
                                                f = (*mem.offset((v + 1i32) as isize)).b16.s1
                                                    as internal_font_number;
                                                xtx_ligature_present = 1i32 != 0;
                                                eff_char_0 = effective_char(
                                                    1i32 != 0,
                                                    f,
                                                    (*mem.offset((v + 1i32) as isize)).b16.s0,
                                                );
                                                break_width[1] -= (*font_info.offset(
                                                    (*width_base.offset(f as isize)
                                                        + (*font_info.offset(
                                                            (*char_base.offset(f as isize)
                                                                + eff_char_0)
                                                                as isize,
                                                        ))
                                                        .b16
                                                        .s3
                                                            as libc::c_int)
                                                        as isize,
                                                ))
                                                .b32
                                                .s1
                                            }
                                            0 | 1 | 2 | 11 => {
                                                break_width[1] -=
                                                    (*mem.offset((v + 1i32) as isize)).b32.s1
                                            }
                                            8 => {
                                                if (*mem.offset(v as isize)).b16.s0 as libc::c_int
                                                    == 40i32
                                                    || (*mem.offset(v as isize)).b16.s0
                                                        as libc::c_int
                                                        == 41i32
                                                    || (*mem.offset(v as isize)).b16.s0
                                                        as libc::c_int
                                                        == 42i32
                                                    || (*mem.offset(v as isize)).b16.s0
                                                        as libc::c_int
                                                        == 43i32
                                                    || (*mem.offset(v as isize)).b16.s0
                                                        as libc::c_int
                                                        == 44i32
                                                {
                                                    break_width[1] -=
                                                        (*mem.offset((v + 1i32) as isize)).b32.s1
                                                } else {
                                                    confusion(
                                                        b"disc1a\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                            }
                                            _ => {
                                                confusion(
                                                    b"disc1\x00" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                        }
                                    }
                                }
                                /*871: "add the width of node s to break_width" */
                                while s != -0xfffffffi32 {
                                    if is_char_node(s) {
                                        let mut eff_char_1: int32_t = 0;
                                        f = (*mem.offset(s as isize)).b16.s1
                                            as internal_font_number;
                                        eff_char_1 = effective_char(
                                            1i32 != 0,
                                            f,
                                            (*mem.offset(s as isize)).b16.s0,
                                        );
                                        break_width[1] += (*font_info.offset(
                                            (*width_base.offset(f as isize)
                                                + (*font_info.offset(
                                                    (*char_base.offset(f as isize) + eff_char_1)
                                                        as isize,
                                                ))
                                                .b16
                                                .s3
                                                    as libc::c_int)
                                                as isize,
                                        ))
                                        .b32
                                        .s1
                                    } else {
                                        match (*mem.offset(s as isize)).b16.s1 as libc::c_int {
                                            6 => {
                                                let mut eff_char_2: int32_t = 0;
                                                f = (*mem.offset((s + 1i32) as isize)).b16.s1
                                                    as internal_font_number;
                                                xtx_ligature_present = 1i32 != 0;
                                                eff_char_2 = effective_char(
                                                    1i32 != 0,
                                                    f,
                                                    (*mem.offset((s + 1i32) as isize)).b16.s0,
                                                );
                                                break_width[1] += (*font_info.offset(
                                                    (*width_base.offset(f as isize)
                                                        + (*font_info.offset(
                                                            (*char_base.offset(f as isize)
                                                                + eff_char_2)
                                                                as isize,
                                                        ))
                                                        .b16
                                                        .s3
                                                            as libc::c_int)
                                                        as isize,
                                                ))
                                                .b32
                                                .s1
                                            }
                                            0 | 1 | 2 | 11 => {
                                                break_width[1] +=
                                                    (*mem.offset((s + 1i32) as isize)).b32.s1
                                            }
                                            8 => {
                                                if (*mem.offset(s as isize)).b16.s0 as libc::c_int
                                                    == 40i32
                                                    || (*mem.offset(s as isize)).b16.s0
                                                        as libc::c_int
                                                        == 41i32
                                                    || (*mem.offset(s as isize)).b16.s0
                                                        as libc::c_int
                                                        == 42i32
                                                    || (*mem.offset(s as isize)).b16.s0
                                                        as libc::c_int
                                                        == 43i32
                                                    || (*mem.offset(s as isize)).b16.s0
                                                        as libc::c_int
                                                        == 44i32
                                                {
                                                    break_width[1] +=
                                                        (*mem.offset((s + 1i32) as isize)).b32.s1
                                                } else {
                                                    confusion(
                                                        b"disc2a\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                            }
                                            _ => {
                                                confusion(
                                                    b"disc2\x00" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                        }
                                    }
                                    s = (*mem.offset(s as isize)).b32.s1
                                }
                                break_width[1] += disc_width;
                                if (*mem.offset((cur_p + 1i32) as isize)).b32.s1 == -0xfffffffi32 {
                                    s = (*mem.offset(v as isize)).b32.s1
                                }
                            }
                        }
                        while s != -0xfffffffi32 {
                            if is_char_node(s) {
                                break;
                            }
                            match (*mem.offset(s as isize)).b16.s1 as libc::c_int {
                                10 => {
                                    v = (*mem.offset((s + 1i32) as isize)).b32.s0;
                                    break_width[1] -= (*mem.offset((v + 1i32) as isize)).b32.s1;
                                    break_width[(2i32
                                        + (*mem.offset(v as isize)).b16.s1 as libc::c_int)
                                        as usize] -= (*mem.offset((v + 2i32) as isize)).b32.s1;
                                    break_width[6] -= (*mem.offset((v + 3i32) as isize)).b32.s1
                                }
                                12 => {}
                                9 => break_width[1] -= (*mem.offset((s + 1i32) as isize)).b32.s1,
                                11 => {
                                    if (*mem.offset(s as isize)).b16.s0 as libc::c_int != 1i32 {
                                        break;
                                    }
                                    break_width[1] -= (*mem.offset((s + 1i32) as isize)).b32.s1
                                }
                                _ => {
                                    break;
                                }
                            }
                            s = (*mem.offset(s as isize)).b32.s1
                        }
                    }
                    /*872: "Insert a delta node to prepare for breaks at cur_p" */
                    if (*mem.offset(prev_r as isize)).b16.s1 as libc::c_int == 2i32 {
                        let ref mut fresh6 = (*mem.offset((prev_r + 1i32) as isize)).b32.s1; /* this is unused */
                        *fresh6 += -cur_active_width[1] + break_width[1];
                        let ref mut fresh7 = (*mem.offset((prev_r + 2i32) as isize)).b32.s1;
                        *fresh7 += -cur_active_width[2] + break_width[2];
                        let ref mut fresh8 = (*mem.offset((prev_r + 3i32) as isize)).b32.s1;
                        *fresh8 += -cur_active_width[3] + break_width[3];
                        let ref mut fresh9 = (*mem.offset((prev_r + 4i32) as isize)).b32.s1;
                        *fresh9 += -cur_active_width[4] + break_width[4];
                        let ref mut fresh10 = (*mem.offset((prev_r + 5i32) as isize)).b32.s1;
                        *fresh10 += -cur_active_width[5] + break_width[5];
                        let ref mut fresh11 = (*mem.offset((prev_r + 6i32) as isize)).b32.s1;
                        *fresh11 += -cur_active_width[6] + break_width[6]
                    } else if prev_r == 4999999i32 - 7i32 {
                        active_width[1] = break_width[1];
                        active_width[2] = break_width[2];
                        active_width[3] = break_width[3];
                        active_width[4] = break_width[4];
                        active_width[5] = break_width[5];
                        active_width[6] = break_width[6]
                    } else {
                        q = get_node(7i32);
                        (*mem.offset(q as isize)).b32.s1 = r;
                        (*mem.offset(q as isize)).b16.s1 = 2i32 as u16;
                        (*mem.offset(q as isize)).b16.s0 = 0i32 as u16;
                        (*mem.offset((q + 1i32) as isize)).b32.s1 =
                            break_width[1] - cur_active_width[1];
                        (*mem.offset((q + 2i32) as isize)).b32.s1 =
                            break_width[2] - cur_active_width[2];
                        (*mem.offset((q + 3i32) as isize)).b32.s1 =
                            break_width[3] - cur_active_width[3];
                        (*mem.offset((q + 4i32) as isize)).b32.s1 =
                            break_width[4] - cur_active_width[4];
                        (*mem.offset((q + 5i32) as isize)).b32.s1 =
                            break_width[5] - cur_active_width[5];
                        (*mem.offset((q + 6i32) as isize)).b32.s1 =
                            break_width[6] - cur_active_width[6];
                        (*mem.offset(prev_r as isize)).b32.s1 = q;
                        prev_prev_r = prev_r;
                        prev_r = q
                    }
                    /* ... resuming 865 ... */
                    if abs((*eqtb.offset(
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
                            + 16i32) as isize,
                    ))
                    .b32
                    .s1) >= 0x3fffffffi32 - minimum_demerits
                    {
                        minimum_demerits = 0x3fffffffi32 - 1i32
                    } else {
                        minimum_demerits = minimum_demerits
                            + abs((*eqtb.offset(
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
                                    + 16i32) as isize,
                            ))
                            .b32
                            .s1)
                    }
                    fit_class = 0i32 as libc::c_uchar;
                    while fit_class as libc::c_int <= 3i32 {
                        if minimal_demerits[fit_class as usize] <= minimum_demerits {
                            /*874: "Insert a new active node from best_place[fit_class] to cur_p" */
                            q = get_node(2i32);
                            (*mem.offset(q as isize)).b32.s1 = passive;
                            passive = q;
                            (*mem.offset((q + 1i32) as isize)).b32.s1 = cur_p;
                            (*mem.offset((q + 1i32) as isize)).b32.s0 =
                                best_place[fit_class as usize];
                            q = get_node(active_node_size as int32_t);
                            (*mem.offset((q + 1i32) as isize)).b32.s1 = passive;
                            (*mem.offset((q + 1i32) as isize)).b32.s0 =
                                best_pl_line[fit_class as usize] + 1i32;
                            (*mem.offset(q as isize)).b16.s0 = fit_class as u16;
                            (*mem.offset(q as isize)).b16.s1 = break_type as u16;
                            (*mem.offset((q + 2i32) as isize)).b32.s1 =
                                minimal_demerits[fit_class as usize];
                            if do_last_line_fit {
                                /*1639: */
                                (*mem.offset((q + 3i32) as isize)).b32.s1 =
                                    best_pl_short[fit_class as usize];
                                (*mem.offset((q + 4i32) as isize)).b32.s1 =
                                    best_pl_glue[fit_class as usize]
                            }
                            (*mem.offset(q as isize)).b32.s1 = r;
                            (*mem.offset(prev_r as isize)).b32.s1 = q;
                            prev_r = q
                        }
                        minimal_demerits[fit_class as usize] = 0x3fffffffi32;
                        fit_class = fit_class.wrapping_add(1)
                    }
                    minimum_demerits = 0x3fffffffi32;
                    /*873: "Insert a delta node to prepare for the next active node" */
                    if r != 4999999i32 - 7i32 {
                        q = get_node(7i32); /* subtype is not used */
                        (*mem.offset(q as isize)).b32.s1 = r;
                        (*mem.offset(q as isize)).b16.s1 = 2i32 as u16;
                        (*mem.offset(q as isize)).b16.s0 = 0i32 as u16;
                        (*mem.offset((q + 1i32) as isize)).b32.s1 =
                            cur_active_width[1] - break_width[1];
                        (*mem.offset((q + 2i32) as isize)).b32.s1 =
                            cur_active_width[2] - break_width[2];
                        (*mem.offset((q + 3i32) as isize)).b32.s1 =
                            cur_active_width[3] - break_width[3];
                        (*mem.offset((q + 4i32) as isize)).b32.s1 =
                            cur_active_width[4] - break_width[4];
                        (*mem.offset((q + 5i32) as isize)).b32.s1 =
                            cur_active_width[5] - break_width[5];
                        (*mem.offset((q + 6i32) as isize)).b32.s1 =
                            cur_active_width[6] - break_width[6];
                        (*mem.offset(prev_r as isize)).b32.s1 = q;
                        prev_prev_r = prev_r;
                        prev_r = q
                    }
                }
                /* ... resuming 864 ... */
                if r == 4999999i32 - 7i32 {
                    return;
                }
                /*879: "Compute the new line width" */
                if l > easy_line {
                    line_width = second_width;
                    old_l = 0x3fffffffi32 - 1i32
                } else {
                    old_l = l;
                    if l > last_special_line {
                        line_width = second_width
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
                            + 0i32) as isize,
                    ))
                    .b32
                    .s1 == -0xfffffffi32
                    {
                        line_width = first_width
                    } else {
                        line_width = (*mem.offset(
                            ((*eqtb.offset(
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
                            .s1 + 2i32 * l) as isize,
                        ))
                        .b32
                        .s1
                    }
                    /* this mem access is in the WEB */
                }
            }
            /*880: "Consider the demerits for a line from r to cur_p; deactivate
             * node r if it should no longer be active; then goto continue if a
             * line from r to cur_p is infeasible; otherwise record a new feasible
             * break" */
            /* Tectonic: if we got here, we must be "considering" a linebreak
             * at the very end of the paragraph. How amazing, it's a perfect fit!
             */
            if semantic_pagination_enabled {
                line_width = cur_active_width[1];
                artificial_demerits = 1i32 != 0;
                shortfall = 0i32
            } else {
                artificial_demerits = 0i32 != 0;
                shortfall = line_width - cur_active_width[1];
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
                        + 70i32) as isize,
                ))
                .b32
                .s1 > 1i32
                {
                    shortfall = shortfall + total_pw(r, cur_p)
                }
            }
            if shortfall > 0i32 {
                /*881: "Set the value of b to the badness for stretching the line,
                 * and compute the corresponding fit_class" */
                if cur_active_width[3] != 0i32
                    || cur_active_width[4] != 0i32
                    || cur_active_width[5] != 0i32
                {
                    if do_last_line_fit {
                        if cur_p == -0xfffffffi32 {
                            /*1634: "Perform computations for the last line and goto found" */
                            if (*mem.offset((r + 3i32) as isize)).b32.s1 == 0i32
                                || (*mem.offset((r + 4i32) as isize)).b32.s1 <= 0i32
                            {
                                current_block = 5565703735569783978;
                            } else if cur_active_width[3] != fill_width[0]
                                || cur_active_width[4] != fill_width[1]
                                || cur_active_width[5] != fill_width[2]
                            {
                                current_block = 5565703735569783978;
                            } else {
                                if (*mem.offset((r + 3i32) as isize)).b32.s1 > 0i32 {
                                    g = cur_active_width[2]
                                } else {
                                    g = cur_active_width[6]
                                }
                                if g <= 0i32 {
                                    current_block = 5565703735569783978;
                                } else {
                                    arith_error = 0i32 != 0;
                                    g = fract(
                                        g,
                                        (*mem.offset((r + 3i32) as isize)).b32.s1,
                                        (*mem.offset((r + 4i32) as isize)).b32.s1,
                                        0x3fffffffi32,
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
                                            + 64i32)
                                            as isize,
                                    ))
                                    .b32
                                    .s1 < 1000i32
                                    {
                                        g = fract(
                                            g,
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
                                                    + 64i32)
                                                    as isize,
                                            ))
                                            .b32
                                            .s1,
                                            1000i32,
                                            0x3fffffffi32,
                                        )
                                    }
                                    if arith_error {
                                        if (*mem.offset((r + 3i32) as isize)).b32.s1 > 0i32 {
                                            g = 0x3fffffffi32
                                        } else {
                                            g = -0x3fffffffi32
                                        }
                                    }
                                    if g > 0i32 {
                                        /*1635: "Set the value of b to the badness of the
                                         * last line for stretching, compute the
                                         * corresponding fit_class, and goto found" */
                                        if g > shortfall {
                                            g = shortfall
                                        }
                                        if g as libc::c_long > 7230584 {
                                            /* XXX: magic number in original WEB code */
                                            if (cur_active_width[2] as libc::c_long) < 1663497 {
                                                /* XXX: magic number in original WEB code */
                                                b = 10000i32;
                                                fit_class = 0i32 as libc::c_uchar;
                                                current_block = 11849408527845460430;
                                            } else {
                                                current_block = 16221891950104054966;
                                            }
                                        } else {
                                            current_block = 16221891950104054966;
                                        }
                                        match current_block {
                                            11849408527845460430 => {}
                                            _ => {
                                                b = badness(g, cur_active_width[2]);
                                                if b > 12i32 {
                                                    if b > 99i32 {
                                                        fit_class = 0i32 as libc::c_uchar
                                                    } else {
                                                        fit_class = 1i32 as libc::c_uchar
                                                    }
                                                } else {
                                                    fit_class = 2i32 as libc::c_uchar
                                                }
                                                current_block = 11849408527845460430;
                                            }
                                        }
                                    } else if g < 0i32 {
                                        /*1636: "Set the value of b to the badness of the
                                         * last line for shrinking, compute the
                                         * corresponding fit_class, and goto found" */
                                        if -g > cur_active_width[6] {
                                            g = -cur_active_width[6]
                                        }
                                        b = badness(-g, cur_active_width[6]);
                                        if b > 12i32 {
                                            /* XXX hardcoded in WEB */
                                            fit_class = 3i32 as libc::c_uchar
                                        } else {
                                            fit_class = 2i32 as libc::c_uchar
                                        }
                                        current_block = 11849408527845460430;
                                    } else {
                                        current_block = 5565703735569783978;
                                    }
                                }
                            }
                        } else {
                            current_block = 5565703735569783978;
                        }
                        match current_block {
                            11849408527845460430 => {}
                            _ => {
                                shortfall = 0i32;
                                current_block = 16988252441985098516;
                            }
                        }
                    } else {
                        current_block = 16988252441985098516;
                    }
                    match current_block {
                        11849408527845460430 => {}
                        _ => {
                            b = 0i32;
                            fit_class = 2i32 as libc::c_uchar;
                            current_block = 8633396468472091231;
                        }
                    }
                } else {
                    let mut current_block_230: u64;
                    if shortfall as libc::c_long > 7230584 {
                        /* XXX: magic number in original WEB code */
                        if (cur_active_width[2] as libc::c_long) < 1663497 {
                            /* XXX: magic number in original WEB code */
                            b = 10000i32;
                            fit_class = 0i32 as libc::c_uchar;
                            current_block_230 = 4001239642700071046;
                        } else {
                            current_block_230 = 15455430299222214173;
                        }
                    } else {
                        current_block_230 = 15455430299222214173;
                    }
                    match current_block_230 {
                        15455430299222214173 => {
                            b = badness(shortfall, cur_active_width[2]);
                            if b > 12i32 {
                                if b > 99i32 {
                                    fit_class = 0i32 as libc::c_uchar
                                } else {
                                    fit_class = 1i32 as libc::c_uchar
                                }
                            } else {
                                fit_class = 2i32 as libc::c_uchar
                            }
                        }
                        _ => {}
                    }
                    current_block = 8633396468472091231;
                }
            } else {
                /*882: "Set the value of b to the badness for shrinking the line,
                 * and compute the corresponding fit_class" */
                if -shortfall > cur_active_width[6] {
                    b = 10000i32 + 1i32
                } else {
                    b = badness(-shortfall, cur_active_width[6])
                }
                if b > 12i32 {
                    fit_class = 3i32 as libc::c_uchar
                } else {
                    fit_class = 2i32 as libc::c_uchar
                }
                current_block = 8633396468472091231;
            }
            match current_block {
                8633396468472091231 => {
                    if do_last_line_fit {
                        /*1637: "Adjust the additional data for last line" */
                        if cur_p == -0xfffffffi32 {
                            shortfall = 0i32
                        }
                        if shortfall > 0i32 {
                            g = cur_active_width[2]
                        } else if shortfall < 0i32 {
                            g = cur_active_width[6]
                        } else {
                            g = 0i32
                        }
                    }
                }
                _ => {}
            }
            if b > 10000i32 || pi == -10000i32 {
                /*883: "Prepare to deactivate node r, and goto deactivate unless
                 * there is a reason to consider lines of text from r to cur_p" */
                if final_pass as libc::c_int != 0
                    && minimum_demerits == 0x3fffffffi32
                    && (*mem.offset(r as isize)).b32.s1 == 4999999i32 - 7i32
                    && prev_r == 4999999i32 - 7i32
                {
                    artificial_demerits = 1i32 != 0;
                    current_block = 8298116646536739282;
                } else if b > threshold {
                    current_block = 4955522990288899513;
                } else {
                    current_block = 8298116646536739282;
                }
                match current_block {
                    4955522990288899513 => {}
                    _ => {
                        node_r_stays_active = 0i32 != 0;
                        current_block = 14114736409816581360;
                    }
                }
            } else {
                prev_r = r;
                if b > threshold {
                    continue;
                }
                node_r_stays_active = 1i32 != 0;
                current_block = 14114736409816581360;
            }
            match current_block {
                14114736409816581360 => {
                    if artificial_demerits {
                        d = 0i32
                    } else {
                        /*888: "Compute the demerits, d, from r to cur_p" */
                        d = (*eqtb.offset(
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
                                + 2i32) as isize,
                        ))
                        .b32
                        .s1 + b; /* algorithmic constant */
                        if abs(d) >= 10000i32 {
                            d = 100000000 as int32_t
                        } else {
                            d = d * d
                        }
                        if pi != 0i32 {
                            if pi > 0i32 {
                                d = d + pi * pi
                            } else if pi > -10000i32 {
                                d = d - pi * pi
                            }
                        }
                        if break_type as libc::c_int == 1i32
                            && (*mem.offset(r as isize)).b16.s1 as libc::c_int == 1i32
                        {
                            if cur_p != -0xfffffffi32 {
                                d = d
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
                                            + 14i32)
                                            as isize,
                                    ))
                                    .b32
                                    .s1
                            } else {
                                d = d
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
                                            + 15i32)
                                            as isize,
                                    ))
                                    .b32
                                    .s1
                            }
                        }
                        if abs(fit_class as libc::c_int
                            - (*mem.offset(r as isize)).b16.s0 as libc::c_int)
                            > 1i32
                        {
                            d = d
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
                                        + 16i32) as isize,
                                ))
                                .b32
                                .s1
                        }
                    }
                    /* resuming 884: */
                    d = d + (*mem.offset((r + 2i32) as isize)).b32.s1;
                    if d <= minimal_demerits[fit_class as usize] {
                        minimal_demerits[fit_class as usize] = d;
                        best_place[fit_class as usize] = (*mem.offset((r + 1i32) as isize)).b32.s1;
                        best_pl_line[fit_class as usize] = l;
                        if do_last_line_fit {
                            /*1638:*/
                            best_pl_short[fit_class as usize] = shortfall;
                            best_pl_glue[fit_class as usize] = g
                        }
                        if d < minimum_demerits {
                            minimum_demerits = d
                        }
                    }
                    if node_r_stays_active {
                        continue;
                    }
                }
                _ => {}
            }
            /*889: "Deactivate node r" */
            (*mem.offset(prev_r as isize)).b32.s1 = (*mem.offset(r as isize)).b32.s1;
            free_node(r, active_node_size as int32_t);
            if prev_r == 4999999i32 - 7i32 {
                /*890: "Update the active widths, since the first active node has been deleted" */
                r = (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1; /*:966 */
                if (*mem.offset(r as isize)).b16.s1 as libc::c_int == 2i32 {
                    active_width[1] += (*mem.offset((r + 1i32) as isize)).b32.s1;
                    active_width[2] += (*mem.offset((r + 2i32) as isize)).b32.s1;
                    active_width[3] += (*mem.offset((r + 3i32) as isize)).b32.s1;
                    active_width[4] += (*mem.offset((r + 4i32) as isize)).b32.s1;
                    active_width[5] += (*mem.offset((r + 5i32) as isize)).b32.s1;
                    active_width[6] += (*mem.offset((r + 6i32) as isize)).b32.s1;
                    cur_active_width[1] = active_width[1];
                    cur_active_width[2] = active_width[2];
                    cur_active_width[3] = active_width[3];
                    cur_active_width[4] = active_width[4];
                    cur_active_width[5] = active_width[5];
                    cur_active_width[6] = active_width[6];
                    (*mem.offset((4999999i32 - 7i32) as isize)).b32.s1 =
                        (*mem.offset(r as isize)).b32.s1;
                    free_node(r, 7i32);
                }
            } else if (*mem.offset(prev_r as isize)).b16.s1 as libc::c_int == 2i32 {
                r = (*mem.offset(prev_r as isize)).b32.s1;
                if r == 4999999i32 - 7i32 {
                    cur_active_width[1] -= (*mem.offset((prev_r + 1i32) as isize)).b32.s1;
                    cur_active_width[2] -= (*mem.offset((prev_r + 2i32) as isize)).b32.s1;
                    cur_active_width[3] -= (*mem.offset((prev_r + 3i32) as isize)).b32.s1;
                    cur_active_width[4] -= (*mem.offset((prev_r + 4i32) as isize)).b32.s1;
                    cur_active_width[5] -= (*mem.offset((prev_r + 5i32) as isize)).b32.s1;
                    cur_active_width[6] -= (*mem.offset((prev_r + 6i32) as isize)).b32.s1;
                    (*mem.offset(prev_prev_r as isize)).b32.s1 = 4999999i32 - 7i32;
                    free_node(prev_r, 7i32);
                    prev_r = prev_prev_r
                } else if (*mem.offset(r as isize)).b16.s1 as libc::c_int == 2i32 {
                    cur_active_width[1] += (*mem.offset((r + 1i32) as isize)).b32.s1;
                    cur_active_width[2] += (*mem.offset((r + 2i32) as isize)).b32.s1;
                    cur_active_width[3] += (*mem.offset((r + 3i32) as isize)).b32.s1;
                    cur_active_width[4] += (*mem.offset((r + 4i32) as isize)).b32.s1;
                    cur_active_width[5] += (*mem.offset((r + 5i32) as isize)).b32.s1;
                    cur_active_width[6] += (*mem.offset((r + 6i32) as isize)).b32.s1;
                    let ref mut fresh12 = (*mem.offset((prev_r + 1i32) as isize)).b32.s1;
                    *fresh12 += (*mem.offset((r + 1i32) as isize)).b32.s1;
                    let ref mut fresh13 = (*mem.offset((prev_r + 2i32) as isize)).b32.s1;
                    *fresh13 += (*mem.offset((r + 2i32) as isize)).b32.s1;
                    let ref mut fresh14 = (*mem.offset((prev_r + 4i32) as isize)).b32.s1;
                    *fresh14 += (*mem.offset((r + 3i32) as isize)).b32.s1;
                    let ref mut fresh15 = (*mem.offset((prev_r + 4i32) as isize)).b32.s1;
                    *fresh15 += (*mem.offset((r + 4i32) as isize)).b32.s1;
                    let ref mut fresh16 = (*mem.offset((prev_r + 5i32) as isize)).b32.s1;
                    *fresh16 += (*mem.offset((r + 5i32) as isize)).b32.s1;
                    let ref mut fresh17 = (*mem.offset((prev_r + 6i32) as isize)).b32.s1;
                    *fresh17 += (*mem.offset((r + 6i32) as isize)).b32.s1;
                    (*mem.offset(prev_r as isize)).b32.s1 = (*mem.offset(r as isize)).b32.s1;
                    free_node(r, 7i32);
                }
            }
        }
    }
}
unsafe extern "C" fn hyphenate() {
    let mut current_block: u64;
    let mut i: libc::c_short = 0;
    let mut j: libc::c_short = 0;
    let mut l: libc::c_short = 0;
    let mut q: int32_t = 0;
    let mut r: int32_t = 0;
    let mut s: int32_t = 0;
    let mut bchar: int32_t = 0;
    let mut major_tail: int32_t = 0;
    let mut minor_tail: int32_t = 0;
    let mut c: UnicodeScalar = 0i32;
    let mut c_loc: libc::c_short = 0;
    let mut r_count: int32_t = 0;
    let mut hyf_node: int32_t = 0;
    let mut z: trie_pointer = 0;
    let mut v: int32_t = 0;
    let mut h: hyph_pointer = 0;
    let mut k: str_number = 0;
    let mut u: pool_pointer = 0;
    let mut for_end: int32_t = 0;
    j = 0i32 as libc::c_short;
    for_end = hn as int32_t;
    if j as libc::c_int <= for_end {
        loop {
            hyf[j as usize] = 0i32 as libc::c_uchar;
            let fresh18 = j;
            j = j + 1;
            if !((fresh18 as libc::c_int) < for_end) {
                break;
            }
        }
    }
    h = hc[1] as hyph_pointer;
    hn += 1;
    hc[hn as usize] = cur_lang as int32_t;
    let mut for_end_0: int32_t = 0;
    j = 2i32 as libc::c_short;
    for_end_0 = hn as int32_t;
    if j as libc::c_int <= for_end_0 {
        loop {
            h = ((h as libc::c_int + h as libc::c_int + hc[j as usize]) % 607i32) as hyph_pointer;
            let fresh19 = j;
            j = j + 1;
            if !((fresh19 as libc::c_int) < for_end_0) {
                break;
            }
        }
    }
    loop {
        k = *hyph_word.offset(h as isize);
        if k == 0i32 {
            current_block = 10027897684796195291;
            break;
        }
        if length(k) == hn as libc::c_int {
            j = 1i32 as libc::c_short;
            u = *str_start.offset((k as libc::c_long - 65536) as isize);
            loop {
                if *str_pool.offset(u as isize) as libc::c_int != hc[j as usize] {
                    current_block = 1763490972649755258;
                    break;
                }
                j += 1;
                u += 1;
                if j as libc::c_int > hn as libc::c_int {
                    current_block = 3275366147856559585;
                    break;
                }
            }
            match current_block {
                1763490972649755258 => {}
                _ => {
                    s = *hyph_list.offset(h as isize);
                    while s != -0xfffffffi32 {
                        hyf[(*mem.offset(s as isize)).b32.s0 as usize] = 1i32 as libc::c_uchar;
                        s = (*mem.offset(s as isize)).b32.s1
                    }
                    hn -= 1;
                    current_block = 15736053877802236303;
                    break;
                }
            }
        }
        h = *hyph_link.offset(h as isize);
        if h as libc::c_int == 0i32 {
            current_block = 10027897684796195291;
            break;
        }
        h = h.wrapping_sub(1)
    }
    match current_block {
        10027897684796195291 => {
            hn -= 1;
            if *trie_trc.offset((cur_lang as libc::c_int + 1i32) as isize) as libc::c_int
                != cur_lang as libc::c_int
            {
                return;
            }
            hc[0] = 0i32;
            hc[(hn as libc::c_int + 1i32) as usize] = 0i32;
            hc[(hn as libc::c_int + 2i32) as usize] = max_hyph_char;
            let mut for_end_1: int32_t = 0;
            j = 0i32 as libc::c_short;
            for_end_1 = hn as libc::c_int - r_hyf + 1i32;
            if j as libc::c_int <= for_end_1 {
                loop {
                    z = *trie_trl.offset((cur_lang as libc::c_int + 1i32) as isize)
                        + hc[j as usize];
                    l = j;
                    while hc[l as usize] == *trie_trc.offset(z as isize) as libc::c_int {
                        if *trie_tro.offset(z as isize) != 0i32 {
                            /*959: */
                            v = *trie_tro.offset(z as isize); /*:958 */
                            loop {
                                v = v + op_start[cur_lang as usize];
                                i = (l as libc::c_int - hyf_distance[v as usize] as libc::c_int)
                                    as libc::c_short;
                                if hyf_num[v as usize] as libc::c_int
                                    > hyf[i as usize] as libc::c_int
                                {
                                    hyf[i as usize] = hyf_num[v as usize] as libc::c_uchar
                                }
                                v = hyf_next[v as usize] as int32_t;
                                if v == 0i32 {
                                    break;
                                }
                            }
                        }
                        l += 1;
                        z = *trie_trl.offset(z as isize) + hc[l as usize]
                    }
                    let fresh20 = j;
                    j = j + 1;
                    if !((fresh20 as libc::c_int) < for_end_1) {
                        break;
                    }
                }
            }
        }
        _ => {}
    }
    let mut for_end_2: int32_t = 0;
    j = 0i32 as libc::c_short;
    for_end_2 = l_hyf - 1i32;
    if j as libc::c_int <= for_end_2 {
        loop {
            hyf[j as usize] = 0i32 as libc::c_uchar;
            let fresh21 = j;
            j = j + 1;
            if !((fresh21 as libc::c_int) < for_end_2) {
                break;
            }
        }
    }
    let mut for_end_3: int32_t = 0;
    j = 0i32 as libc::c_short;
    for_end_3 = r_hyf - 1i32;
    if j as libc::c_int <= for_end_3 {
        loop {
            hyf[(hn as libc::c_int - j as libc::c_int) as usize] = 0i32 as libc::c_uchar;
            let fresh22 = j;
            j = j + 1;
            if !((fresh22 as libc::c_int) < for_end_3) {
                break;
            }
        }
    }
    let mut for_end_4: int32_t = 0;
    j = l_hyf as libc::c_short;
    for_end_4 = hn as libc::c_int - r_hyf;
    if j as libc::c_int <= for_end_4 {
        current_block = 5207889489643863322;
    } else {
        current_block = 8102658916883067714;
    }
    loop {
        match current_block {
            8102658916883067714 => return,
            _ => {
                if hyf[j as usize] as libc::c_int & 1i32 != 0 {
                    break;
                }
                let fresh23 = j;
                j = j + 1;
                if (fresh23 as libc::c_int) < for_end_4 {
                    current_block = 5207889489643863322;
                } else {
                    current_block = 8102658916883067714;
                }
            }
        }
    }
    if ha != -0xfffffffi32
        && !is_char_node(ha)
        && (*mem.offset(ha as isize)).b16.s1 as libc::c_int == 8i32
        && ((*mem.offset(ha as isize)).b16.s0 as libc::c_int == 40i32
            || (*mem.offset(ha as isize)).b16.s0 as libc::c_int == 41i32)
    {
        s = cur_p;
        while (*mem.offset(s as isize)).b32.s1 != ha {
            s = (*mem.offset(s as isize)).b32.s1
        }
        hyphen_passed = 0i32 as small_number;
        let mut for_end_5: int32_t = 0;
        j = l_hyf as libc::c_short;
        for_end_5 = hn as libc::c_int - r_hyf;
        if j as libc::c_int <= for_end_5 {
            loop {
                if hyf[j as usize] as libc::c_int & 1i32 != 0 {
                    q = new_native_word_node(hf, j as libc::c_int - hyphen_passed as libc::c_int);
                    (*mem.offset(q as isize)).b16.s0 = (*mem.offset(ha as isize)).b16.s0;
                    let mut for_end_6: int32_t = 0;
                    i = 0i32 as libc::c_short;
                    for_end_6 = j as libc::c_int - hyphen_passed as libc::c_int - 1i32;
                    if i as libc::c_int <= for_end_6 {
                        loop {
                            *(&mut *mem.offset((q + 6i32) as isize) as *mut memory_word
                                as *mut libc::c_ushort)
                                .offset(i as isize) = *(&mut *mem.offset((ha + 6i32) as isize)
                                as *mut memory_word
                                as *mut libc::c_ushort)
                                .offset((i as libc::c_int + hyphen_passed as libc::c_int) as isize);
                            let fresh24 = i;
                            i = i + 1;
                            if !((fresh24 as libc::c_int) < for_end_6) {
                                break;
                            }
                        }
                    }
                    measure_native_node(
                        &mut *mem.offset(q as isize) as *mut memory_word as *mut libc::c_void,
                        ((*eqtb.offset(
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
                                + 74i32) as isize,
                        ))
                        .b32
                        .s1 > 0i32) as libc::c_int,
                    );
                    (*mem.offset(s as isize)).b32.s1 = q;
                    s = q;
                    q = new_disc();
                    (*mem.offset((q + 1i32) as isize)).b32.s0 = new_native_character(hf, hyf_char);
                    (*mem.offset(s as isize)).b32.s1 = q;
                    s = q;
                    hyphen_passed = j
                }
                let fresh25 = j;
                j = j + 1;
                if !((fresh25 as libc::c_int) < for_end_5) {
                    break;
                }
            }
        }
        hn = (*mem.offset((ha + 4i32) as isize)).b16.s1 as small_number;
        q = new_native_word_node(hf, hn as libc::c_int - hyphen_passed as libc::c_int);
        (*mem.offset(q as isize)).b16.s0 = (*mem.offset(ha as isize)).b16.s0;
        let mut for_end_7: int32_t = 0;
        i = 0i32 as libc::c_short;
        for_end_7 = hn as libc::c_int - hyphen_passed as libc::c_int - 1i32;
        if i as libc::c_int <= for_end_7 {
            loop {
                *(&mut *mem.offset((q + 6i32) as isize) as *mut memory_word
                    as *mut libc::c_ushort)
                    .offset(i as isize) = *(&mut *mem.offset((ha + 6i32) as isize)
                    as *mut memory_word
                    as *mut libc::c_ushort)
                    .offset((i as libc::c_int + hyphen_passed as libc::c_int) as isize);
                let fresh26 = i;
                i = i + 1;
                if !((fresh26 as libc::c_int) < for_end_7) {
                    break;
                }
            }
        }
        measure_native_node(
            &mut *mem.offset(q as isize) as *mut memory_word as *mut libc::c_void,
            ((*eqtb.offset(
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
                    + 74i32) as isize,
            ))
            .b32
            .s1 > 0i32) as libc::c_int,
        );
        (*mem.offset(s as isize)).b32.s1 = q;
        s = q;
        q = (*mem.offset(ha as isize)).b32.s1;
        (*mem.offset(s as isize)).b32.s1 = q;
        (*mem.offset(ha as isize)).b32.s1 = -0xfffffffi32;
        flush_node_list(ha);
    } else {
        q = (*mem.offset(hb as isize)).b32.s1;
        (*mem.offset(hb as isize)).b32.s1 = -0xfffffffi32;
        r = (*mem.offset(ha as isize)).b32.s1;
        (*mem.offset(ha as isize)).b32.s1 = -0xfffffffi32;
        bchar = hyf_bchar;
        if is_char_node(ha) {
            if (*mem.offset(ha as isize)).b16.s1 as libc::c_int != hf {
                current_block = 6826215413708131726;
            } else {
                init_list = ha;
                init_lig = 0i32 != 0;
                hu[0] = (*mem.offset(ha as isize)).b16.s0 as int32_t;
                current_block = 6662862405959679103;
            }
        } else if (*mem.offset(ha as isize)).b16.s1 as libc::c_int == 6i32 {
            if (*mem.offset((ha + 1i32) as isize)).b16.s1 as libc::c_int != hf {
                current_block = 6826215413708131726;
            } else {
                init_list = (*mem.offset((ha + 1i32) as isize)).b32.s1;
                init_lig = 1i32 != 0;
                init_lft = (*mem.offset(ha as isize)).b16.s0 as libc::c_int > 1i32;
                hu[0] = (*mem.offset((ha + 1i32) as isize)).b16.s0 as int32_t;
                if init_list == -0xfffffffi32 {
                    if init_lft {
                        hu[0] = max_hyph_char;
                        init_lig = 0i32 != 0
                    }
                }
                free_node(ha, 2i32);
                current_block = 6662862405959679103;
            }
        } else {
            if !is_char_node(r) {
                if (*mem.offset(r as isize)).b16.s1 as libc::c_int == 6i32 {
                    if (*mem.offset(r as isize)).b16.s0 as libc::c_int > 1i32 {
                        current_block = 6826215413708131726;
                    } else {
                        current_block = 2415422468722899689;
                    }
                } else {
                    current_block = 2415422468722899689;
                }
            } else {
                current_block = 2415422468722899689;
            }
            match current_block {
                6826215413708131726 => {}
                _ => {
                    j = 1i32 as libc::c_short;
                    s = ha;
                    init_list = -0xfffffffi32;
                    current_block = 5209103994167801282;
                }
            }
        }
        match current_block {
            6662862405959679103 => {
                s = cur_p;
                while (*mem.offset(s as isize)).b32.s1 != ha {
                    s = (*mem.offset(s as isize)).b32.s1
                }
                j = 0i32 as libc::c_short
            }
            6826215413708131726 => {
                s = ha;
                j = 0i32 as libc::c_short;
                hu[0] = max_hyph_char;
                init_lig = 0i32 != 0;
                init_list = -0xfffffffi32
            }
            _ => {}
        }
        flush_node_list(r);
        loop {
            l = j;
            j = (reconstitute(j, hn, bchar, hyf_char) as libc::c_int + 1i32) as libc::c_short;
            if hyphen_passed as libc::c_int == 0i32 {
                (*mem.offset(s as isize)).b32.s1 =
                    (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1;
                while (*mem.offset(s as isize)).b32.s1 > -0xfffffffi32 {
                    s = (*mem.offset(s as isize)).b32.s1
                }
                if hyf[(j as libc::c_int - 1i32) as usize] as libc::c_int & 1i32 != 0 {
                    l = j;
                    hyphen_passed = (j as libc::c_int - 1i32) as small_number;
                    (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1 = -0xfffffffi32
                }
            }
            if hyphen_passed as libc::c_int > 0i32 {
                loop
                /*949: */
                {
                    r = get_node(2i32);
                    (*mem.offset(r as isize)).b32.s1 =
                        (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1;
                    (*mem.offset(r as isize)).b16.s1 = 7i32 as u16;
                    major_tail = r;
                    r_count = 0i32;
                    while (*mem.offset(major_tail as isize)).b32.s1 > -0xfffffffi32 {
                        major_tail = (*mem.offset(major_tail as isize)).b32.s1;
                        r_count += 1
                    }
                    i = hyphen_passed;
                    hyf[i as usize] = 0i32 as libc::c_uchar;
                    minor_tail = -0xfffffffi32;
                    (*mem.offset((r + 1i32) as isize)).b32.s0 = -0xfffffffi32;
                    hyf_node = new_character(hf, hyf_char as UTF16_code);
                    if hyf_node != -0xfffffffi32 {
                        i += 1;
                        c = hu[i as usize];
                        hu[i as usize] = hyf_char;
                        (*mem.offset(hyf_node as isize)).b32.s1 = avail;
                        avail = hyf_node
                    }
                    while l as libc::c_int <= i as libc::c_int {
                        l = (reconstitute(l, i, *font_bchar.offset(hf as isize), 65536i32)
                            as libc::c_int
                            + 1i32) as libc::c_short;
                        if (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1 > -0xfffffffi32 {
                            if minor_tail == -0xfffffffi32 {
                                (*mem.offset((r + 1i32) as isize)).b32.s0 =
                                    (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1
                            } else {
                                (*mem.offset(minor_tail as isize)).b32.s1 =
                                    (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1
                            }
                            minor_tail = (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1;
                            while (*mem.offset(minor_tail as isize)).b32.s1 > -0xfffffffi32 {
                                minor_tail = (*mem.offset(minor_tail as isize)).b32.s1
                            }
                        }
                    }
                    if hyf_node != -0xfffffffi32 {
                        hu[i as usize] = c;
                        l = i;
                        i -= 1
                    }
                    minor_tail = -0xfffffffi32;
                    (*mem.offset((r + 1i32) as isize)).b32.s1 = -0xfffffffi32;
                    c_loc = 0i32 as libc::c_short;
                    if *bchar_label.offset(hf as isize) != 0i32 {
                        l -= 1;
                        c = hu[l as usize];
                        c_loc = l;
                        hu[l as usize] = max_hyph_char
                    }
                    while (l as libc::c_int) < j as libc::c_int {
                        loop {
                            l = (reconstitute(l, hn, bchar, 65536i32) as libc::c_int + 1i32)
                                as libc::c_short;
                            if c_loc as libc::c_int > 0i32 {
                                hu[c_loc as usize] = c;
                                c_loc = 0i32 as libc::c_short
                            }
                            if (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1 > -0xfffffffi32 {
                                if minor_tail == -0xfffffffi32 {
                                    (*mem.offset((r + 1i32) as isize)).b32.s1 =
                                        (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1
                                } else {
                                    (*mem.offset(minor_tail as isize)).b32.s1 =
                                        (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1
                                }
                                minor_tail = (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1;
                                while (*mem.offset(minor_tail as isize)).b32.s1 > -0xfffffffi32 {
                                    minor_tail = (*mem.offset(minor_tail as isize)).b32.s1
                                }
                            }
                            if l as libc::c_int >= j as libc::c_int {
                                break;
                            }
                        }
                        while l as libc::c_int > j as libc::c_int {
                            /*952: */
                            j = (reconstitute(j, hn, bchar, 65536i32) as libc::c_int + 1i32)
                                as libc::c_short; /*:944*/
                            (*mem.offset(major_tail as isize)).b32.s1 =
                                (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1;
                            while (*mem.offset(major_tail as isize)).b32.s1 > -0xfffffffi32 {
                                major_tail = (*mem.offset(major_tail as isize)).b32.s1;
                                r_count += 1
                            }
                        }
                    }
                    if r_count > 127i32 {
                        (*mem.offset(s as isize)).b32.s1 = (*mem.offset(r as isize)).b32.s1;
                        (*mem.offset(r as isize)).b32.s1 = -0xfffffffi32;
                        flush_node_list(r);
                    } else {
                        (*mem.offset(s as isize)).b32.s1 = r;
                        (*mem.offset(r as isize)).b16.s0 = r_count as u16
                    }
                    s = major_tail;
                    hyphen_passed = (j as libc::c_int - 1i32) as small_number;
                    (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1 = -0xfffffffi32;
                    if !(hyf[(j as libc::c_int - 1i32) as usize] as libc::c_int & 1i32 != 0) {
                        break;
                    }
                }
            }
            if j as libc::c_int > hn as libc::c_int {
                break;
            }
        }
        (*mem.offset(s as isize)).b32.s1 = q;
        flush_list(init_list);
    };
}
unsafe extern "C" fn finite_shrink(mut p: int32_t) -> int32_t {
    let mut q: int32_t = 0;
    if no_shrink_error_yet {
        no_shrink_error_yet = 0i32 != 0;
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
        }
        print_cstr(
            b"Infinite glue shrinkage found in a paragraph\x00" as *const u8 as *const libc::c_char,
        );
        help_ptr = 5i32 as libc::c_uchar;
        help_line[4] = b"The paragraph just ended includes some glue that has\x00" as *const u8
            as *const libc::c_char;
        help_line[3] = b"infinite shrinkability, e.g., `\\hskip 0pt minus 1fil\'.\x00" as *const u8
            as *const libc::c_char;
        help_line[2] = b"Such glue doesn\'t belong there---it allows a paragraph\x00" as *const u8
            as *const libc::c_char;
        help_line[1] = b"of any length to fit on one line. But it\'s safe to proceed,\x00"
            as *const u8 as *const libc::c_char;
        help_line[0] = b"since the offensive shrinkability has been made finite.\x00" as *const u8
            as *const libc::c_char;
        error();
    }
    q = new_spec(p);
    (*mem.offset(q as isize)).b16.s0 = 0i32 as u16;
    delete_glue_ref(p);
    return q;
}
unsafe extern "C" fn reconstitute(
    mut j: small_number,
    mut n: small_number,
    mut bchar: int32_t,
    mut hchar: int32_t,
) -> small_number {
    let mut current_block: u64;
    let mut p: int32_t = 0;
    let mut t: int32_t = 0;
    let mut q: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut cur_rh: int32_t = 0;
    let mut test_char: int32_t = 0;
    let mut w: scaled_t = 0;
    let mut k: font_index = 0;
    hyphen_passed = 0i32 as small_number;
    t = 4999999i32 - 4i32;
    w = 0i32;
    (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1 = -0xfffffffi32;
    cur_l = hu[j as usize];
    cur_q = t;
    if j as libc::c_int == 0i32 {
        ligature_present = init_lig;
        p = init_list;
        if ligature_present {
            lft_hit = init_lft
        }
        while p > -0xfffffffi32 {
            (*mem.offset(t as isize)).b32.s1 = get_avail();
            t = (*mem.offset(t as isize)).b32.s1;
            (*mem.offset(t as isize)).b16.s1 = hf as u16;
            (*mem.offset(t as isize)).b16.s0 = (*mem.offset(p as isize)).b16.s0;
            p = (*mem.offset(p as isize)).b32.s1
        }
    } else if cur_l < 65536i32 {
        (*mem.offset(t as isize)).b32.s1 = get_avail();
        t = (*mem.offset(t as isize)).b32.s1;
        (*mem.offset(t as isize)).b16.s1 = hf as u16;
        (*mem.offset(t as isize)).b16.s0 = cur_l as u16
    }
    lig_stack = -0xfffffffi32;
    if (j as libc::c_int) < n as libc::c_int {
        cur_r = hu[(j as libc::c_int + 1i32) as usize]
    } else {
        cur_r = bchar
    }
    if hyf[j as usize] as libc::c_int & 1i32 != 0 {
        cur_rh = hchar
    } else {
        cur_rh = 65536i32
    }
    'c_27176: loop {
        if cur_l == 65536i32 {
            k = *bchar_label.offset(hf as isize);
            if k == 0i32 {
                current_block = 4939169394500275451;
            } else {
                q = (*font_info.offset(k as isize)).b16;
                current_block = 1434579379687443766;
            }
        } else {
            q = (*font_info.offset(
                (*char_base.offset(hf as isize) + effective_char(1i32 != 0, hf, cur_l as u16))
                    as isize,
            ))
            .b16;
            if q.s1 as libc::c_int % 4i32 != 1i32 {
                current_block = 4939169394500275451;
            } else {
                k = *lig_kern_base.offset(hf as isize) + q.s0 as libc::c_int;
                q = (*font_info.offset(k as isize)).b16;
                if q.s3 as libc::c_int > 128i32 {
                    k = ((*lig_kern_base.offset(hf as isize)
                        + 256i32 * q.s1 as libc::c_int
                        + q.s0 as libc::c_int) as libc::c_long
                        + 32768
                        - (256i32 * 128i32) as libc::c_long) as font_index;
                    q = (*font_info.offset(k as isize)).b16
                }
                current_block = 1434579379687443766;
            }
        }
        match current_block {
            1434579379687443766 => {
                if cur_rh < 65536i32 {
                    test_char = cur_rh
                } else {
                    test_char = cur_r
                }
                loop {
                    if q.s2 as libc::c_int == test_char {
                        if q.s3 as libc::c_int <= 128i32 {
                            if cur_rh < 65536i32 {
                                hyphen_passed = j;
                                hchar = 65536i32;
                                cur_rh = 65536i32;
                                continue 'c_27176;
                            } else {
                                if hchar < 65536i32 {
                                    if hyf[j as usize] as libc::c_int & 1i32 != 0 {
                                        hyphen_passed = j;
                                        hchar = 65536i32
                                    }
                                }
                                if (q.s1 as libc::c_int) < 128i32 {
                                    /*946: */
                                    if cur_l == 65536i32 {
                                        lft_hit = 1i32 != 0
                                    }
                                    if j as libc::c_int == n as libc::c_int {
                                        if lig_stack == -0xfffffffi32 {
                                            rt_hit = 1i32 != 0
                                        }
                                    }
                                    match q.s1 as libc::c_int {
                                        1 | 5 => {
                                            cur_l = q.s0 as int32_t;
                                            ligature_present = 1i32 != 0
                                        }
                                        2 | 6 => {
                                            cur_r = q.s0 as int32_t;
                                            if lig_stack > -0xfffffffi32 {
                                                (*mem.offset(lig_stack as isize)).b16.s0 =
                                                    cur_r as u16
                                            } else {
                                                lig_stack = new_lig_item(cur_r as u16);
                                                if j as libc::c_int == n as libc::c_int {
                                                    bchar = 65536i32
                                                } else {
                                                    p = get_avail();
                                                    (*mem.offset((lig_stack + 1i32) as isize))
                                                        .b32
                                                        .s1 = p;
                                                    (*mem.offset(p as isize)).b16.s0 = hu
                                                        [(j as libc::c_int + 1i32) as usize]
                                                        as u16;
                                                    (*mem.offset(p as isize)).b16.s1 =
                                                        hf as u16
                                                }
                                            }
                                        }
                                        3 => {
                                            cur_r = q.s0 as int32_t;
                                            p = lig_stack;
                                            lig_stack = new_lig_item(cur_r as u16);
                                            (*mem.offset(lig_stack as isize)).b32.s1 = p
                                        }
                                        7 | 11 => {
                                            if ligature_present {
                                                p = new_ligature(
                                                    hf,
                                                    cur_l as u16,
                                                    (*mem.offset(cur_q as isize)).b32.s1,
                                                );
                                                if lft_hit {
                                                    (*mem.offset(p as isize)).b16.s0 =
                                                        2i32 as u16;
                                                    lft_hit = 0i32 != 0
                                                }
                                                (*mem.offset(cur_q as isize)).b32.s1 = p;
                                                t = p;
                                                ligature_present = 0i32 != 0
                                            }
                                            cur_q = t;
                                            cur_l = q.s0 as int32_t;
                                            ligature_present = 1i32 != 0
                                        }
                                        _ => {
                                            cur_l = q.s0 as int32_t;
                                            ligature_present = 1i32 != 0;
                                            if lig_stack > -0xfffffffi32 {
                                                if (*mem.offset((lig_stack + 1i32) as isize)).b32.s1
                                                    > -0xfffffffi32
                                                {
                                                    (*mem.offset(t as isize)).b32.s1 = (*mem
                                                        .offset((lig_stack + 1i32) as isize))
                                                    .b32
                                                    .s1;
                                                    t = (*mem.offset(t as isize)).b32.s1;
                                                    j += 1
                                                }
                                                p = lig_stack;
                                                lig_stack = (*mem.offset(p as isize)).b32.s1;
                                                free_node(p, 2i32);
                                                if lig_stack == -0xfffffffi32 {
                                                    if (j as libc::c_int) < n as libc::c_int {
                                                        cur_r =
                                                            hu[(j as libc::c_int + 1i32) as usize]
                                                    } else {
                                                        cur_r = bchar
                                                    }
                                                    if hyf[j as usize] as libc::c_int & 1i32 != 0 {
                                                        cur_rh = hchar
                                                    } else {
                                                        cur_rh = 65536i32
                                                    }
                                                } else {
                                                    cur_r = (*mem.offset(lig_stack as isize)).b16.s0
                                                        as int32_t
                                                }
                                            } else {
                                                if j as libc::c_int == n as libc::c_int {
                                                    break;
                                                }
                                                (*mem.offset(t as isize)).b32.s1 = get_avail();
                                                t = (*mem.offset(t as isize)).b32.s1;
                                                (*mem.offset(t as isize)).b16.s1 = hf as u16;
                                                (*mem.offset(t as isize)).b16.s0 =
                                                    cur_r as u16;
                                                j += 1;
                                                if (j as libc::c_int) < n as libc::c_int {
                                                    cur_r = hu[(j as libc::c_int + 1i32) as usize]
                                                } else {
                                                    cur_r = bchar
                                                }
                                                if hyf[j as usize] as libc::c_int & 1i32 != 0 {
                                                    cur_rh = hchar
                                                } else {
                                                    cur_rh = 65536i32
                                                }
                                            }
                                        }
                                    }
                                    if !(q.s1 as libc::c_int > 4i32) {
                                        continue 'c_27176;
                                    }
                                    if q.s1 as libc::c_int != 7i32 {
                                        break;
                                    } else {
                                        continue 'c_27176;
                                    }
                                } else {
                                    w = (*font_info.offset(
                                        (*kern_base.offset(hf as isize)
                                            + 256i32 * q.s1 as libc::c_int
                                            + q.s0 as libc::c_int)
                                            as isize,
                                    ))
                                    .b32
                                    .s1;
                                    break;
                                }
                            }
                        }
                    }
                    if q.s3 as libc::c_int >= 128i32 {
                        if cur_rh == 65536i32 {
                            break;
                        }
                        cur_rh = 65536i32;
                        continue 'c_27176;
                    } else {
                        k = k + q.s3 as libc::c_int + 1i32;
                        q = (*font_info.offset(k as isize)).b16
                    }
                }
            }
            _ => {}
        }
        if ligature_present {
            p = new_ligature(hf, cur_l as u16, (*mem.offset(cur_q as isize)).b32.s1);
            if lft_hit {
                (*mem.offset(p as isize)).b16.s0 = 2i32 as u16;
                lft_hit = 0i32 != 0
            }
            if rt_hit {
                if lig_stack == -0xfffffffi32 {
                    let ref mut fresh28 = (*mem.offset(p as isize)).b16.s0;
                    *fresh28 = (*fresh28).wrapping_add(1);
                    rt_hit = 0i32 != 0
                }
            }
            (*mem.offset(cur_q as isize)).b32.s1 = p;
            t = p;
            ligature_present = 0i32 != 0
        }
        if w != 0i32 {
            (*mem.offset(t as isize)).b32.s1 = new_kern(w);
            t = (*mem.offset(t as isize)).b32.s1;
            w = 0i32;
            (*mem.offset((t + 2i32) as isize)).b32.s0 = 0i32
        }
        if !(lig_stack > -0xfffffffi32) {
            break;
        }
        cur_q = t;
        cur_l = (*mem.offset(lig_stack as isize)).b16.s0 as int32_t;
        ligature_present = 1i32 != 0;
        if (*mem.offset((lig_stack + 1i32) as isize)).b32.s1 > -0xfffffffi32 {
            (*mem.offset(t as isize)).b32.s1 = (*mem.offset((lig_stack + 1i32) as isize)).b32.s1;
            t = (*mem.offset(t as isize)).b32.s1;
            j += 1
        }
        p = lig_stack;
        lig_stack = (*mem.offset(p as isize)).b32.s1;
        free_node(p, 2i32);
        if lig_stack == -0xfffffffi32 {
            if (j as libc::c_int) < n as libc::c_int {
                cur_r = hu[(j as libc::c_int + 1i32) as usize]
            } else {
                cur_r = bchar
            }
            if hyf[j as usize] as libc::c_int & 1i32 != 0 {
                cur_rh = hchar
            } else {
                cur_rh = 65536i32
            }
        } else {
            cur_r = (*mem.offset(lig_stack as isize)).b16.s0 as int32_t
        }
    }
    return j;
}
unsafe extern "C" fn total_pw(mut q: int32_t, mut p: int32_t) -> scaled_t {
    let mut current_block: u64;
    let mut l: int32_t = 0;
    let mut r: int32_t = 0;
    let mut n: int32_t = 0;
    if (*mem.offset((q + 1i32) as isize)).b32.s1 == -0xfffffffi32 {
        l = first_p
    } else {
        l = (*mem.offset(((*mem.offset((q + 1i32) as isize)).b32.s1 + 1i32) as isize))
            .b32
            .s1
    }
    r = prev_rightmost(global_prev_p, p);
    if p != -0xfffffffi32
        && (*mem.offset(p as isize)).b16.s1 as libc::c_int == 7i32
        && (*mem.offset((p + 1i32) as isize)).b32.s0 != -0xfffffffi32
    {
        r = (*mem.offset((p + 1i32) as isize)).b32.s0;
        while (*mem.offset(r as isize)).b32.s1 != -0xfffffffi32 {
            r = (*mem.offset(r as isize)).b32.s1
        }
    } else {
        r = find_protchar_right(l, r)
    }
    if l != -0xfffffffi32 && (*mem.offset(l as isize)).b16.s1 as libc::c_int == 7i32 {
        if (*mem.offset((l + 1i32) as isize)).b32.s1 != -0xfffffffi32 {
            l = (*mem.offset((l + 1i32) as isize)).b32.s1;
            current_block = 15424580701460361554;
        } else {
            n = (*mem.offset(l as isize)).b16.s0 as int32_t;
            l = (*mem.offset(l as isize)).b32.s1;
            while n > 0i32 {
                if (*mem.offset(l as isize)).b32.s1 != -0xfffffffi32 {
                    l = (*mem.offset(l as isize)).b32.s1
                }
                n -= 1
            }
            current_block = 15089075282327824602;
        }
    } else {
        current_block = 15089075282327824602;
    }
    match current_block {
        15089075282327824602 => l = find_protchar_left(l, 1i32 != 0),
        _ => {}
    }
    return char_pw(l, 0i32 as small_number) + char_pw(r, 1i32 as small_number);
}
unsafe extern "C" fn find_protchar_left(mut l: int32_t, mut d: bool) -> int32_t {
    let mut t: int32_t = 0;
    let mut run: bool = false;
    if (*mem.offset(l as isize)).b32.s1 != -0xfffffffi32
        && (*mem.offset(l as isize)).b16.s1 as libc::c_int == 0i32
        && (*mem.offset((l + 1i32) as isize)).b32.s1 == 0i32
        && (*mem.offset((l + 3i32) as isize)).b32.s1 == 0i32
        && (*mem.offset((l + 2i32) as isize)).b32.s1 == 0i32
        && (*mem.offset((l + 5i32) as isize)).b32.s1 == -0xfffffffi32
    {
        l = (*mem.offset(l as isize)).b32.s1
    } else if d {
        while (*mem.offset(l as isize)).b32.s1 != -0xfffffffi32
            && !(is_char_node(l) as libc::c_int != 0
                || is_non_discardable_node(l) as libc::c_int != 0)
        {
            l = (*mem.offset(l as isize)).b32.s1
        }
    }
    hlist_stack_level = 0i32 as libc::c_short;
    run = 1i32 != 0;
    loop {
        t = l;
        while run as libc::c_int != 0
            && (*mem.offset(l as isize)).b16.s1 as libc::c_int == 0i32
            && (*mem.offset((l + 5i32) as isize)).b32.s1 != -0xfffffffi32
        {
            push_node(l);
            l = (*mem.offset((l + 5i32) as isize)).b32.s1
        }
        while run as libc::c_int != 0
            && (!is_char_node(l)
                && ((*mem.offset(l as isize)).b16.s1 as libc::c_int == 3i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 4i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 5i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 12i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 7i32
                        && (*mem.offset((l + 1i32) as isize)).b32.s0 == -0xfffffffi32
                        && (*mem.offset((l + 1i32) as isize)).b32.s1 == -0xfffffffi32
                        && (*mem.offset(l as isize)).b16.s0 as libc::c_int == 0i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 9i32
                        && (*mem.offset((l + 1i32) as isize)).b32.s1 == 0i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 11i32
                        && ((*mem.offset((l + 1i32) as isize)).b32.s1 == 0i32
                            || (*mem.offset(l as isize)).b16.s0 as libc::c_int == 0i32)
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 10i32
                        && (*mem.offset((l + 1i32) as isize)).b32.s0 == 0i32
                    || (*mem.offset(l as isize)).b16.s1 as libc::c_int == 0i32
                        && (*mem.offset((l + 1i32) as isize)).b32.s1 == 0i32
                        && (*mem.offset((l + 3i32) as isize)).b32.s1 == 0i32
                        && (*mem.offset((l + 2i32) as isize)).b32.s1 == 0i32
                        && (*mem.offset((l + 5i32) as isize)).b32.s1 == -0xfffffffi32))
        {
            while (*mem.offset(l as isize)).b32.s1 == -0xfffffffi32
                && hlist_stack_level as libc::c_int > 0i32
            {
                l = pop_node()
            }
            if (*mem.offset(l as isize)).b32.s1 != -0xfffffffi32 {
                l = (*mem.offset(l as isize)).b32.s1
            } else if hlist_stack_level as libc::c_int == 0i32 {
                run = 0i32 != 0
            }
        }
        if t == l {
            break;
        }
    }
    return l;
}
unsafe extern "C" fn find_protchar_right(mut l: int32_t, mut r: int32_t) -> int32_t {
    let mut t: int32_t = 0;
    let mut run: bool = false;
    if r == -0xfffffffi32 {
        return -0xfffffffi32;
    }
    hlist_stack_level = 0i32 as libc::c_short;
    run = 1i32 != 0;
    loop {
        t = r;
        while run as libc::c_int != 0
            && (*mem.offset(r as isize)).b16.s1 as libc::c_int == 0i32
            && (*mem.offset((r + 5i32) as isize)).b32.s1 != -0xfffffffi32
        {
            push_node(l);
            push_node(r);
            l = (*mem.offset((r + 5i32) as isize)).b32.s1;
            r = l;
            while (*mem.offset(r as isize)).b32.s1 != -0xfffffffi32 {
                r = (*mem.offset(r as isize)).b32.s1
            }
        }
        while run as libc::c_int != 0
            && (!is_char_node(r)
                && ((*mem.offset(r as isize)).b16.s1 as libc::c_int == 3i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 4i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 5i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 12i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 7i32
                        && (*mem.offset((r + 1i32) as isize)).b32.s0 == -0xfffffffi32
                        && (*mem.offset((r + 1i32) as isize)).b32.s1 == -0xfffffffi32
                        && (*mem.offset(r as isize)).b16.s0 as libc::c_int == 0i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 9i32
                        && (*mem.offset((r + 1i32) as isize)).b32.s1 == 0i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 11i32
                        && ((*mem.offset((r + 1i32) as isize)).b32.s1 == 0i32
                            || (*mem.offset(r as isize)).b16.s0 as libc::c_int == 0i32)
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 10i32
                        && (*mem.offset((r + 1i32) as isize)).b32.s0 == 0i32
                    || (*mem.offset(r as isize)).b16.s1 as libc::c_int == 0i32
                        && (*mem.offset((r + 1i32) as isize)).b32.s1 == 0i32
                        && (*mem.offset((r + 3i32) as isize)).b32.s1 == 0i32
                        && (*mem.offset((r + 2i32) as isize)).b32.s1 == 0i32
                        && (*mem.offset((r + 5i32) as isize)).b32.s1 == -0xfffffffi32))
        {
            while r == l && hlist_stack_level as libc::c_int > 0i32 {
                r = pop_node();
                l = pop_node()
            }
            if r != l && r != -0xfffffffi32 {
                r = prev_rightmost(l, r)
            } else if r == l && hlist_stack_level as libc::c_int == 0i32 {
                run = 0i32 != 0
            }
        }
        if t == r {
            break;
        }
    }
    return r;
}
unsafe extern "C" fn push_node(mut p: int32_t) {
    if hlist_stack_level as libc::c_int > 512i32 {
        pdf_error(
            b"push_node\x00" as *const u8 as *const libc::c_char,
            b"stack overflow\x00" as *const u8 as *const libc::c_char,
        );
    }
    hlist_stack[hlist_stack_level as usize] = p;
    hlist_stack_level = (hlist_stack_level as libc::c_int + 1i32) as libc::c_short;
}
unsafe extern "C" fn pop_node() -> int32_t {
    hlist_stack_level = (hlist_stack_level as libc::c_int - 1i32) as libc::c_short;
    if (hlist_stack_level as libc::c_int) < 0i32 {
        pdf_error(
            b"pop_node\x00" as *const u8 as *const libc::c_char,
            b"stack underflow (internal error)\x00" as *const u8 as *const libc::c_char,
        );
    }
    return hlist_stack[hlist_stack_level as usize];
}
