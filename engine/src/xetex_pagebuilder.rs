#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    /* Needed here for UFILE */
    /* variables! */
    /* All the following variables are defined in xetexini.c */
    #[no_mangle]
    static mut eqtb: *mut memory_word;
    #[no_mangle]
    static mut file_line_error_style_p: i32;
    #[no_mangle]
    static mut help_line: [*const i8; 6];
    #[no_mangle]
    static mut help_ptr: u8;
    #[no_mangle]
    static mut temp_ptr: i32;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut nest: *mut list_state_record;
    #[no_mangle]
    static mut nest_ptr: i32;
    #[no_mangle]
    static mut cur_list: list_state_record;
    #[no_mangle]
    static mut line: i32;
    #[no_mangle]
    static mut cur_mark: [i32; 5];
    #[no_mangle]
    static mut dead_cycles: i32;
    #[no_mangle]
    static mut best_height_plus_depth: scaled_t;
    #[no_mangle]
    static mut page_tail: i32;
    #[no_mangle]
    static mut page_contents: u8;
    #[no_mangle]
    static mut page_so_far: [scaled_t; 8];
    #[no_mangle]
    static mut last_glue: i32;
    #[no_mangle]
    static mut last_penalty: i32;
    #[no_mangle]
    static mut last_kern: scaled_t;
    #[no_mangle]
    static mut last_node_type: i32;
    #[no_mangle]
    static mut insert_penalties: i32;
    #[no_mangle]
    static mut output_active: bool;
    #[no_mangle]
    static mut sa_root: [i32; 8];
    #[no_mangle]
    static mut cur_ptr: i32;
    #[no_mangle]
    static mut disc_ptr: [i32; 4];
    #[no_mangle]
    static mut semantic_pagination_enabled: bool;
    /* the former xetexcoerce.h: */
    #[no_mangle]
    fn badness(t: scaled_t, s: scaled_t) -> i32;
    #[no_mangle]
    fn get_node(s: i32) -> i32;
    #[no_mangle]
    fn free_node(p: i32, s: i32);
    #[no_mangle]
    fn new_null_box() -> i32;
    #[no_mangle]
    fn new_spec(p: i32) -> i32;
    #[no_mangle]
    fn new_skip_param(n: small_number) -> i32;
    #[no_mangle]
    fn delete_token_ref(p: i32);
    #[no_mangle]
    fn delete_glue_ref(p: i32);
    #[no_mangle]
    fn flush_node_list(p: i32);
    #[no_mangle]
    fn push_nest();
    #[no_mangle]
    fn new_save_level(c: group_code);
    #[no_mangle]
    fn geq_word_define(p: i32, w: i32);
    #[no_mangle]
    fn begin_token_list(p: i32, t: u16);
    #[no_mangle]
    fn find_sa_element(t: small_number, n: i32, w: bool);
    #[no_mangle]
    fn scan_left_brace();
    #[no_mangle]
    fn vpackage(p: i32, h: scaled_t, m: small_number, l: scaled_t) -> i32;
    #[no_mangle]
    fn prune_page_top(p: i32, s: bool) -> i32;
    #[no_mangle]
    fn vert_break(p: i32, h: scaled_t, d: scaled_t) -> i32;
    #[no_mangle]
    fn do_marks(a: small_number, l: small_number, q: i32) -> bool;
    #[no_mangle]
    fn box_error(n: eight_bits);
    #[no_mangle]
    fn normal_paragraph();
    #[no_mangle]
    fn error();
    #[no_mangle]
    fn confusion(s: *const i8) -> !;
    #[no_mangle]
    fn print_cstr(s: *const i8);
    #[no_mangle]
    fn print_nl_cstr(s: *const i8);
    #[no_mangle]
    fn print_esc_cstr(s: *const i8);
    #[no_mangle]
    fn print_int(n: i32);
    #[no_mangle]
    fn print_file_line();
    #[no_mangle]
    fn ship_out(p: i32);
    #[no_mangle]
    fn x_over_n(x: scaled_t, n: i32) -> scaled_t;
}
pub type scaled_t = i32;
pub type eight_bits = u8;
pub type small_number = libc::c_short;
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
/* enum: normal .. filll */
pub type group_code = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_state_record {
    pub mode: libc::c_short,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}
#[inline]
unsafe extern "C" fn is_non_discardable_node(p: i32) -> bool {
    return ((*mem.offset(p as isize)).b16.s1 as i32) < 9i32;
}
/* tectonic/xetex-pagebuilder.c: the page builder
   Copyright 2017-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* Customizations for Tectonic:
 *
 * In semantic pagination mode, we don't run the pagebuilder routine. We just
 * directly invoke the shipout code, which in turn writes out the output vlist
 * without worrying about pages. We also behave as if holding_inserts is
 * always true: inserts are kept in the page vlist rather than being
 * processed.
 */
static mut best_page_break: i32 = 0;
static mut best_size: scaled_t = 0;
static mut least_page_cost: i32 = 0;
static mut page_max_depth: scaled_t = 0;
/* XXX other variables belong here but pop up all over the code */
#[no_mangle]
pub unsafe extern "C" fn initialize_pagebuilder_variables() {
    page_max_depth = 0i32;
}
unsafe extern "C" fn freeze_page_specs(mut s: small_number) {
    page_contents = s as u8;
    page_so_far[0] = (*eqtb.offset(
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
            + 4i32) as isize,
    ))
    .b32
    .s1;
    page_max_depth = (*eqtb.offset(
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
            + 5i32) as isize,
    ))
    .b32
    .s1;
    page_so_far[7] = 0i32;
    page_so_far[1] = 0i32;
    page_so_far[2] = 0i32;
    page_so_far[3] = 0i32;
    page_so_far[4] = 0i32;
    page_so_far[5] = 0i32;
    page_so_far[6] = 0i32;
    least_page_cost = 0x3fffffffi32;
}
unsafe extern "C" fn ensure_vbox(mut n: eight_bits) {
    let mut p: i32 = (*eqtb.offset(
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
            + n as i32) as isize,
    ))
    .b32
    .s1;
    if p == -0xfffffffi32 {
        return;
    }
    if (*mem.offset(p as isize)).b16.s1 as i32 != 0i32 {
        return;
    }
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! \x00" as *const u8 as *const i8);
    }
    print_cstr(b"Insertions can only be added to a vbox\x00" as *const u8 as *const i8);
    help_ptr = 3i32 as u8;
    help_line[2] =
        b"Tut tut: You\'re trying to \\insert into a\x00" as *const u8 as *const i8;
    help_line[1] =
        b"\\box register that now contains an \\hbox.\x00" as *const u8 as *const i8;
    help_line[0] =
        b"Proceed, and I\'ll discard its present contents.\x00" as *const u8 as *const i8;
    box_error(n);
}
/*1047: "The fire_up subroutine prepares to output the curent page at the best
 * place; then it fires up the user's output routine, if there is one, or it
 * simple ships out the page. There is one parameter, `c`, which represents
 * the node that was being contributed to the page when the decision to force
 * an output was made." */
unsafe extern "C" fn fire_up(mut c: i32) {
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut s: i32 = 0;
    let mut prev_p: i32 = 0;
    let mut n: u8 = 0;
    let mut wait: bool = false;
    let mut save_vbadness: i32 = 0;
    let mut save_vfuzz: scaled_t = 0;
    let mut save_split_top_skip: i32 = 0;
    let mut process_inserts: bool = false;
    /*1048: "Set the value of output_penalty" */
    if (*mem.offset(best_page_break as isize)).b16.s1 as i32 == 12i32 {
        geq_word_define(
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
            (*mem.offset((best_page_break + 1i32) as isize)).b32.s1,
        );
        (*mem.offset((best_page_break + 1i32) as isize)).b32.s1 = 10000i32
    } else {
        geq_word_define(
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
            10000i32,
        );
    }
    /* ... resuming 1047 ... "We set the values of top_mark, first_mark, and
     * bot_mark. The program uses the fact that `bot_mark != null` implies
     * `first_mark != null`; it also knows that `bot_mark == null` implies
     * `top_mark = first_mark = null`." The do_marks() call basically does the
     * same thing as the code immediately below it, but for all "mark classes"
     * beyond the default one -- a "mark class" being a concept introduced in
     * e-TeX. */
    if sa_root[7] != -0xfffffffi32 {
        if do_marks(1i32 as small_number, 0i32 as small_number, sa_root[7]) {
            sa_root[7] = -0xfffffffi32
        }
    }
    if cur_mark[2] != -0xfffffffi32 {
        if cur_mark[0] != -0xfffffffi32 {
            delete_token_ref(cur_mark[0]);
        }
        cur_mark[0] = cur_mark[2];
        let ref mut fresh0 = (*mem.offset(cur_mark[0] as isize)).b32.s0;
        *fresh0 += 1;
        delete_token_ref(cur_mark[1]);
        cur_mark[1] = -0xfffffffi32
    }
    /*1049: "Put the optimal current page into box 255, update first_mark and
     * bot_mark, append insertions to their boxes, and put the remaining nodes
     * back on the contribution list." */
    if c == best_page_break {
        best_page_break = -0xfffffffi32
    } /* "c not yet linked in" */
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
            + 255i32) as isize,
    ))
    .b32
    .s1 != -0xfffffffi32
    {
        /*1050:*/
        if file_line_error_style_p != 0 {
            print_file_line(); /* "this will count the number of insertions held over" */
        } else {
            print_nl_cstr(b"! \x00" as *const u8 as *const i8);
        }
        print_cstr(b"\x00" as *const u8 as *const i8);
        print_esc_cstr(b"box\x00" as *const u8 as *const i8);
        print_cstr(b"255 is not void\x00" as *const u8 as *const i8);
        help_ptr = 2i32 as u8;
        help_line[1] = b"You shouldn\'t use \\box255 except in \\output routines.\x00" as *const u8
            as *const i8;
        help_line[0] = b"Proceed, and I\'ll discard its present contents.\x00" as *const u8
            as *const i8;
        box_error(255i32 as eight_bits);
    }
    insert_penalties = 0i32;
    save_split_top_skip = (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 10i32) as isize,
    ))
    .b32
    .s1;
    /* Tectonic: in semantic pagination mode, we act as if holding_inserts is
     * always active. */
    process_inserts = (*eqtb.offset(
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
            + 53i32) as isize,
    ))
    .b32
    .s1 <= 0i32
        && !semantic_pagination_enabled;
    if process_inserts {
        /*1053: "Prepare all the boxes involved in insertions to act as
         * queues". Namely: for each insert being tracked, set the
         * `last_ins_ptr` field of its data structure to the last node in its
         * associated vlist. If holding_inserts is positive, the inserts are
         * just kept in the page vlist without any processing, I believe with
         * the expectation that the output routine will do something clever
         * with them. */
        r = (*mem.offset(4999999)).b32.s1; /* 5 = list_offset, "position of the list inside the box" */
        while r != 4999999i32 {
            if (*mem.offset((r + 2i32) as isize)).b32.s0 != -0xfffffffi32 {
                n = (*mem.offset(r as isize)).b16.s0 as u8;
                ensure_vbox(n);
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
                        + n as i32) as isize,
                ))
                .b32
                .s1 == -0xfffffffi32
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
                            + 19i32
                            + 256i32
                            + 256i32
                            + 13i32
                            + 256i32
                            + 4i32
                            + n as i32) as isize,
                    ))
                    .b32
                    .s1 = new_null_box()
                }
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
                        + 13i32
                        + 256i32
                        + 4i32
                        + n as i32) as isize,
                ))
                .b32
                .s1 + 5i32;
                while (*mem.offset(p as isize)).b32.s1 != -0xfffffffi32 {
                    p = (*mem.offset(p as isize)).b32.s1
                }
                (*mem.offset((r + 2i32) as isize)).b32.s1 = p
            }
            r = (*mem.offset(r as isize)).b32.s1
        }
    }
    q = 4999999i32 - 4i32;
    (*mem.offset(q as isize)).b32.s1 = -0xfffffffi32;
    prev_p = 4999999i32 - 2i32;
    p = (*mem.offset(prev_p as isize)).b32.s1;
    while p != best_page_break {
        if (*mem.offset(p as isize)).b16.s1 as i32 == 3i32 {
            if process_inserts {
                /*1055: "Either insert the material specified by node p into
                 * the appropriate box, or hold it for the next page; also
                 * delete node p from the current page." */
                r = (*mem.offset(4999999)).b32.s1;
                while (*mem.offset(r as isize)).b16.s0 as i32
                    != (*mem.offset(p as isize)).b16.s0 as i32
                {
                    r = (*mem.offset(r as isize)).b32.s1
                }
                if (*mem.offset((r + 2i32) as isize)).b32.s0 == -0xfffffffi32 {
                    wait = 1i32 != 0
                } else {
                    wait = 0i32 != 0;
                    s = (*mem.offset((r + 2i32) as isize)).b32.s1;
                    (*mem.offset(s as isize)).b32.s1 = (*mem.offset((p + 4i32) as isize)).b32.s0;
                    if (*mem.offset((r + 2i32) as isize)).b32.s0 == p {
                        /*:1057 */
                        /*1056: "Wrap up the box specified by node r,
                         * splitting node p if called for; set wait = true if
                         * node p holds a remainder after splitting" */
                        if (*mem.offset(r as isize)).b16.s1 as i32 == 1i32 {
                            if (*mem.offset((r + 1i32) as isize)).b32.s0 == p
                                && (*mem.offset((r + 1i32) as isize)).b32.s1 != -0xfffffffi32
                            {
                                while (*mem.offset(s as isize)).b32.s1
                                    != (*mem.offset((r + 1i32) as isize)).b32.s1
                                {
                                    s = (*mem.offset(s as isize)).b32.s1
                                }
                                (*mem.offset(s as isize)).b32.s1 = -0xfffffffi32;
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
                                        + 10i32) as isize,
                                ))
                                .b32
                                .s1 = (*mem.offset((p + 4i32) as isize)).b32.s1;
                                (*mem.offset((p + 4i32) as isize)).b32.s0 = prune_page_top(
                                    (*mem.offset((r + 1i32) as isize)).b32.s1,
                                    0i32 != 0,
                                );
                                if (*mem.offset((p + 4i32) as isize)).b32.s0 != -0xfffffffi32 {
                                    temp_ptr = vpackage(
                                        (*mem.offset((p + 4i32) as isize)).b32.s0,
                                        0i32,
                                        1i32 as small_number,
                                        0x3fffffffi32,
                                    );
                                    (*mem.offset((p + 3i32) as isize)).b32.s1 =
                                        (*mem.offset((temp_ptr + 3i32) as isize)).b32.s1
                                            + (*mem.offset((temp_ptr + 2i32) as isize)).b32.s1;
                                    free_node(temp_ptr, 8i32);
                                    wait = 1i32 != 0
                                }
                            }
                        }
                        (*mem.offset((r + 2i32) as isize)).b32.s0 = -0xfffffffi32;
                        n = (*mem.offset(r as isize)).b16.s0 as u8;
                        temp_ptr = (*mem.offset(
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
                                    + n as i32) as isize,
                            ))
                            .b32
                            .s1 + 5i32) as isize,
                        ))
                        .b32
                        .s1;
                        free_node(
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
                                    + n as i32) as isize,
                            ))
                            .b32
                            .s1,
                            8i32,
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
                                + n as i32) as isize,
                        ))
                        .b32
                        .s1 = vpackage(temp_ptr, 0i32, 1i32 as small_number, 0x3fffffffi32)
                    } else {
                        while (*mem.offset(s as isize)).b32.s1 != -0xfffffffi32 {
                            s = (*mem.offset(s as isize)).b32.s1
                        }
                        (*mem.offset((r + 2i32) as isize)).b32.s1 = s
                    }
                }
                (*mem.offset(prev_p as isize)).b32.s1 = (*mem.offset(p as isize)).b32.s1;
                (*mem.offset(p as isize)).b32.s1 = -0xfffffffi32;
                if wait {
                    (*mem.offset(q as isize)).b32.s1 = p;
                    q = p;
                    insert_penalties += 1
                } else {
                    delete_glue_ref((*mem.offset((p + 4i32) as isize)).b32.s1);
                    free_node(p, 5i32);
                }
                p = prev_p
            }
        } else if (*mem.offset(p as isize)).b16.s1 as i32 == 4i32 {
            if (*mem.offset((p + 1i32) as isize)).b32.s0 != 0i32 {
                /*1057: "Either append the insertion node p after node q, and
                 * remove it from the current page, or delete node(p)" */
                /*1618: "Update the current marks" */
                find_sa_element(
                    7i32 as small_number,
                    (*mem.offset((p + 1i32) as isize)).b32.s0,
                    1i32 != 0,
                );
                if (*mem.offset((cur_ptr + 1i32) as isize)).b32.s1 == -0xfffffffi32 {
                    (*mem.offset((cur_ptr + 1i32) as isize)).b32.s1 =
                        (*mem.offset((p + 1i32) as isize)).b32.s1;
                    let ref mut fresh1 = (*mem
                        .offset((*mem.offset((p + 1i32) as isize)).b32.s1 as isize))
                    .b32
                    .s0;
                    *fresh1 += 1
                }
                if (*mem.offset((cur_ptr + 2i32) as isize)).b32.s0 != -0xfffffffi32 {
                    delete_token_ref((*mem.offset((cur_ptr + 2i32) as isize)).b32.s0);
                }
                (*mem.offset((cur_ptr + 2i32) as isize)).b32.s0 =
                    (*mem.offset((p + 1i32) as isize)).b32.s1;
                let ref mut fresh2 = (*mem
                    .offset((*mem.offset((p + 1i32) as isize)).b32.s1 as isize))
                .b32
                .s0;
                *fresh2 += 1
            } else {
                /*1051: "Update the values of first_mark and bot_mark" */
                if cur_mark[1] == -0xfffffffi32 {
                    cur_mark[1] = (*mem.offset((p + 1i32) as isize)).b32.s1;
                    let ref mut fresh3 = (*mem.offset(cur_mark[1] as isize)).b32.s0;
                    *fresh3 += 1
                }
                if cur_mark[2] != -0xfffffffi32 {
                    delete_token_ref(cur_mark[2]);
                }
                cur_mark[2] = (*mem.offset((p + 1i32) as isize)).b32.s1;
                let ref mut fresh4 = (*mem.offset(cur_mark[2] as isize)).b32.s0;
                *fresh4 += 1
            }
        }
        prev_p = p;
        p = (*mem.offset(prev_p as isize)).b32.s1
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
            + 10i32) as isize,
    ))
    .b32
    .s1 = save_split_top_skip;
    /*1052: "Break the current page at node p, put it in box 255, and put the
     * remaining nodes on the contribution list". */
    if p != -0xfffffffi32 {
        if (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 == -0xfffffffi32 {
            if nest_ptr == 0i32 {
                cur_list.tail = page_tail
            } else {
                (*nest.offset(0)).tail = page_tail
            }
        }
        (*mem.offset(page_tail as isize)).b32.s1 =
            (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1;
        (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 = p;
        (*mem.offset(prev_p as isize)).b32.s1 = -0xfffffffi32
    }
    /* Temporarily futz some variables to inhibit error messages */
    save_vbadness = (*eqtb.offset(
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
            + 27i32) as isize,
    ))
    .b32
    .s1;
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
            + 27i32) as isize,
    ))
    .b32
    .s1 = 10000i32;
    save_vfuzz = (*eqtb.offset(
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
            + 9i32) as isize,
    ))
    .b32
    .s1;
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
            + 9i32) as isize,
    ))
    .b32
    .s1 = 0x3fffffffi32;
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
            + 255i32) as isize,
    ))
    .b32
    .s1 = vpackage(
        (*mem.offset((4999999i32 - 2i32) as isize)).b32.s1,
        best_size,
        0i32 as small_number,
        page_max_depth,
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
            + 27i32) as isize,
    ))
    .b32
    .s1 = save_vbadness;
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
            + 9i32) as isize,
    ))
    .b32
    .s1 = save_vfuzz;
    if last_glue != 0x3fffffffi32 {
        delete_glue_ref(last_glue);
    }
    /*1026: "Start a new current page" */
    page_contents = 0i32 as u8;
    page_tail = 4999999i32 - 2i32;
    (*mem.offset((4999999i32 - 2i32) as isize)).b32.s1 = -0xfffffffi32;
    last_glue = 0x3fffffffi32;
    last_penalty = 0i32;
    last_kern = 0i32;
    last_node_type = -1i32;
    page_so_far[7] = 0i32;
    page_max_depth = 0i32;
    if q != 4999999i32 - 4i32 {
        (*mem.offset((4999999i32 - 2i32) as isize)).b32.s1 =
            (*mem.offset((4999999i32 - 4i32) as isize)).b32.s1;
        page_tail = q
    }
    /*1054: "Delete the page-insertion nodes" */
    r = (*mem.offset(4999999)).b32.s1;
    while r != 4999999i32 {
        q = (*mem.offset(r as isize)).b32.s1;
        free_node(r, 4i32);
        r = q
    }
    (*mem.offset(4999999)).b32.s1 = 4999999i32;
    /* ... resuming 1047 ... */
    if sa_root[7] != -0xfffffffi32 {
        if do_marks(2i32 as small_number, 0i32 as small_number, sa_root[7]) {
            sa_root[7] = -0xfffffffi32
        }
    }
    if cur_mark[0] != -0xfffffffi32 && cur_mark[1] == -0xfffffffi32 {
        cur_mark[1] = cur_mark[0];
        let ref mut fresh5 = (*mem.offset(cur_mark[0] as isize)).b32.s0;
        *fresh5 += 1
    }
    /* Tectonic: in semantic pagination mode, ignore the output routine. */
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
            + 1i32) as isize,
    ))
    .b32
    .s1 != -0xfffffffi32
        && !semantic_pagination_enabled
    {
        if dead_cycles
            >= (*eqtb.offset(
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
            .s1
        {
            /*1059: "Explain that too many dead cycles have happened in a row." */
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \x00" as *const u8 as *const i8);
            }
            print_cstr(b"Output loop---\x00" as *const u8 as *const i8);
            print_int(dead_cycles);
            print_cstr(b" consecutive dead cycles\x00" as *const u8 as *const i8);
            help_ptr = 3i32 as u8;
            help_line[2] = b"I\'ve concluded that your \\output is awry; it never does a\x00"
                as *const u8 as *const i8;
            help_line[1] = b"\\shipout, so I\'m shipping \\box255 out myself. Next time\x00"
                as *const u8 as *const i8;
            help_line[0] = b"increase \\maxdeadcycles if you want me to be more patient!\x00"
                as *const u8 as *const i8;
            error();
        } else {
            /*1060: "Fire up the user's output routine and return" */
            output_active = 1i32 != 0; /* this is `prev_depth` */
            dead_cycles += 1;
            push_nest();
            cur_list.mode = -1i32 as libc::c_short;
            cur_list.aux.b32.s1 = -65536000i32;
            cur_list.mode_line = -line;
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
                        + 1i32) as isize,
                ))
                .b32
                .s1,
                7i32 as u16,
            );
            new_save_level(8i32 as group_code);
            normal_paragraph();
            scan_left_brace();
            return;
        }
    }
    /*1058: "Perform the default output routine." */
    if (*mem.offset((4999999i32 - 2i32) as isize)).b32.s1 != -0xfffffffi32 {
        if (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 == -0xfffffffi32 {
            if nest_ptr == 0i32 {
                cur_list.tail = page_tail
            } else {
                (*nest.offset(0)).tail = page_tail
            }
        } else {
            (*mem.offset(page_tail as isize)).b32.s1 =
                (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1
        }
        (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 =
            (*mem.offset((4999999i32 - 2i32) as isize)).b32.s1;
        (*mem.offset((4999999i32 - 2i32) as isize)).b32.s1 = -0xfffffffi32;
        page_tail = 4999999i32 - 2i32
    }
    flush_node_list(disc_ptr[2]);
    disc_ptr[2] = -0xfffffffi32;
    ship_out(
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
                + 255i32) as isize,
        ))
        .b32
        .s1,
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
            + 255i32) as isize,
    ))
    .b32
    .s1 = -0xfffffffi32;
}
/* xetex-errors */
/* xetex-math */
/* xetex-output */
/* xetex-pagebuilder */
/* XXX redundant with xetex-linebreak.c */
#[no_mangle]
pub unsafe extern "C" fn build_page() {
    let mut current_block: u64;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut pi: i32 = 0;
    let mut n: u8 = 0;
    let mut delta: scaled_t = 0;
    let mut h: scaled_t = 0;
    let mut w: scaled_t = 0;
    if (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 == -0xfffffffi32
        || output_active as i32 != 0
    {
        return;
    }
    loop  {
        p = (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1;
        /*1031: "Update the values of last_glue, last_penalty, and last_kern" */
        if last_glue != 0x3fffffffi32 { delete_glue_ref(last_glue); }
        last_penalty = 0i32;
        last_kern = 0i32;
        last_node_type =
            (*mem.offset(p as isize)).b16.s1 as i32 + 1i32;
        if (*mem.offset(p as isize)).b16.s1 as i32 == 10i32 {
            last_glue = (*mem.offset((p + 1i32) as isize)).b32.s0;
            let ref mut fresh6 = (*mem.offset(last_glue as isize)).b32.s1;
            *fresh6 += 1
        } else {
            last_glue = 0x3fffffffi32;
            if (*mem.offset(p as isize)).b16.s1 as i32 == 12i32 {
                last_penalty = (*mem.offset((p + 1i32) as isize)).b32.s1
            } else if (*mem.offset(p as isize)).b16.s1 as i32 == 11i32
             {
                last_kern = (*mem.offset((p + 1i32) as isize)).b32.s1
            }
        }
        /*1032: "Move node p to the current page; if it is time for a page
         * break, put the nodes following the break back onto the contribution
         * list, and return to the user's output routine if there is one" */
        /* "The code here is an example of a many-way switch into routines
         * that merge together in different places. Some people call this
         * unstructured programming, but the author doesn't see much wrong
         * with it, as long as the various labels have a well-understood
         * meaning." */
        /* 1035: "If the current page is empty and node p is to be deleted,
         * goto done1; otherwise use node p to update the state of the current
         * page; if this node is an insertion, goto contribute; otherwise if
         * this node is not a legal breakpoint, goto contribute or
         * update_heights; otherwise set `pi` to the penalty associated with
         * this breakpoint." ... "The title of this section is already so
         * long, it seems best to avoid making it more accurate but still
         * longer, by mentioning the fact that a kern node at the end of the
         * contribution list will not be contributed until we know its
         * successor." */
        match (*mem.offset(p as isize)).b16.s1 as i32 {
            0 | 1 | 2 => {
                if (page_contents as i32) < 2i32 {
                    /*1036: "Initialize the current page, insert the \topskip glue
                 * ahead of p, and goto continue." */
                    if page_contents as i32 == 0i32 {
                        freeze_page_specs(2i32 as
                                              small_number); /* "now temp_ptr = glue_ptr(q) */
                    } else { page_contents = 2i32 as u8 }
                    q = new_skip_param(9i32 as small_number);
                    if (*mem.offset((temp_ptr + 1i32) as isize)).b32.s1 >
                           (*mem.offset((p + 3i32) as isize)).b32.s1 {
                        let ref mut fresh7 =
                            (*mem.offset((temp_ptr + 1i32) as isize)).b32.s1;
                        *fresh7 -= (*mem.offset((p + 3i32) as isize)).b32.s1
                    } else {
                        (*mem.offset((temp_ptr + 1i32) as isize)).b32.s1 =
                            0i32
                    }
                    (*mem.offset(q as isize)).b32.s1 = p;
                    (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 = q;
                    current_block = 15427931788582360902;
                } else {
                    /*1037: "Prepare to move a box or rule node to the current
                 * page, then goto contribute." */
                    page_so_far[1] +=
                        page_so_far[7] +
                            (*mem.offset((p + 3i32) as isize)).b32.s1;
                    page_so_far[7] =
                        (*mem.offset((p + 2i32) as isize)).b32.s1;
                    current_block = 11918621130838443904;
                }
            }
            8 => {
                /*1401: "Prepare to move whatsit p to the current page, then goto contribute" */
                if (*mem.offset(p as isize)).b16.s0 as i32 == 43i32 ||
                       (*mem.offset(p as isize)).b16.s0 as i32 ==
                           44i32 {
                    page_so_far[1] +=
                        page_so_far[7] +
                            (*mem.offset((p + 3i32) as isize)).b32.s1;
                    page_so_far[7] = (*mem.offset((p + 2i32) as isize)).b32.s1
                }
                current_block = 11918621130838443904;
            }
            10 => {
                if (page_contents as i32) < 2i32 {
                    current_block = 15559656170992153795;
                } else if is_non_discardable_node(page_tail) {
                    pi = 0i32;
                    current_block = 13253659531982233645;
                } else { current_block = 5579886686420104461; }
            }
            11 => {
                if (page_contents as i32) < 2i32 {
                    current_block = 15559656170992153795;
                } else if (*mem.offset(p as isize)).b32.s1 == -0xfffffffi32 {
                    return
                } else if (*mem.offset((*mem.offset(p as isize)).b32.s1 as
                                           isize)).b16.s1 as i32 ==
                              10i32 {
                    pi = 0i32;
                    current_block = 13253659531982233645;
                } else { current_block = 5579886686420104461; }
            }
            12 => {
                if (page_contents as i32) < 2i32 {
                    current_block = 15559656170992153795;
                } else {
                    pi = (*mem.offset((p + 1i32) as isize)).b32.s1;
                    current_block = 13253659531982233645;
                }
            }
            4 => { current_block = 11918621130838443904; }
            3 => {
                /*1043: "Append an insertion to the current page and goto contribute" */
                if page_contents as i32 == 0i32 {
                    freeze_page_specs(1i32 as small_number);
                }
                n = (*mem.offset(p as isize)).b16.s0 as u8;
                r = 4999999i32;
                while n as i32 >=
                          (*mem.offset((*mem.offset(r as isize)).b32.s1 as
                                           isize)).b16.s0 as i32 {
                    r = (*mem.offset(r as isize)).b32.s1
                }
                if (*mem.offset(r as isize)).b16.s0 as i32 !=
                       n as i32 {
                    /*1044: "Create a page insertion node with subtype(r) = n, and
                 * include the glue correction for box `n` in the current page
                 * state" */
                    q = get_node(4i32);
                    (*mem.offset(q as isize)).b32.s1 =
                        (*mem.offset(r as isize)).b32.s1;
                    (*mem.offset(r as isize)).b32.s1 = q;
                    r = q;
                    (*mem.offset(r as isize)).b16.s0 = n as u16;
                    (*mem.offset(r as isize)).b16.s1 = 0i32 as u16;
                    ensure_vbox(n);
                    if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                          (0x10ffffi32 + 1i32) + 1i32 +
                                          15000i32 + 12i32 + 9000i32 + 1i32 +
                                          1i32 + 19i32 + 256i32 + 256i32 +
                                          13i32 + 256i32 + 4i32 +
                                          n as i32) as isize)).b32.s1
                           == -0xfffffffi32 {
                        (*mem.offset((r + 3i32) as isize)).b32.s1 = 0i32
                    } else {
                        (*mem.offset((r + 3i32) as isize)).b32.s1 =
                            (*mem.offset(((*eqtb.offset((1i32 +
                                                             (0x10ffffi32 +
                                                                  1i32) +
                                                             (0x10ffffi32 +
                                                                  1i32) + 1i32
                                                             + 15000i32 +
                                                             12i32 + 9000i32 +
                                                             1i32 + 1i32 +
                                                             19i32 + 256i32 +
                                                             256i32 + 13i32 +
                                                             256i32 + 4i32 +
                                                             n as i32)
                                                            as isize)).b32.s1
                                              + 3i32) as isize)).b32.s1 +
                                (*mem.offset(((*eqtb.offset((1i32 +
                                                                 (0x10ffffi32
                                                                      + 1i32)
                                                                 +
                                                                 (0x10ffffi32
                                                                      + 1i32)
                                                                 + 1i32 +
                                                                 15000i32 +
                                                                 12i32 +
                                                                 9000i32 +
                                                                 1i32 + 1i32 +
                                                                 19i32 +
                                                                 256i32 +
                                                                 256i32 +
                                                                 13i32 +
                                                                 256i32 + 4i32
                                                                 +
                                                                 n as
                                                                     i32)
                                                                as
                                                                isize)).b32.s1
                                                  + 2i32) as isize)).b32.s1
                    }
                    (*mem.offset((r + 2i32) as isize)).b32.s0 = -0xfffffffi32;
                    q =
                        (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                           (0x10ffffi32 + 1i32) + 1i32 +
                                           15000i32 + 12i32 + 9000i32 + 1i32 +
                                           1i32 + 19i32 + n as i32) as
                                          isize)).b32.s1;
                    if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
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
                                          (0x10ffffi32 + 1i32) + 85i32 +
                                          n as i32) as isize)).b32.s1
                           == 1000i32 {
                        h = (*mem.offset((r + 3i32) as isize)).b32.s1
                    } else {
                        h =
                            x_over_n((*mem.offset((r + 3i32) as
                                                      isize)).b32.s1, 1000i32)
                                *
                                (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) + 1i32
                                                   + 15000i32 + 12i32 +
                                                   9000i32 + 1i32 + 1i32 +
                                                   19i32 + 256i32 + 256i32 +
                                                   13i32 + 256i32 + 4i32 +
                                                   256i32 + 1i32 +
                                                   3i32 * 256i32 +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   85i32 + n as i32)
                                                  as isize)).b32.s1
                    }
                    page_so_far[0] -=
                        h + (*mem.offset((q + 1i32) as isize)).b32.s1;
                    page_so_far[(2i32 +
                                     (*mem.offset(q as isize)).b16.s1 as
                                         i32) as usize] +=
                        (*mem.offset((q + 2i32) as isize)).b32.s1;
                    page_so_far[6] +=
                        (*mem.offset((q + 3i32) as isize)).b32.s1;
                    if (*mem.offset(q as isize)).b16.s0 as i32 != 0i32
                           &&
                           (*mem.offset((q + 3i32) as isize)).b32.s1 != 0i32 {
                        if file_line_error_style_p != 0 {
                            print_file_line();
                        } else {
                            print_nl_cstr(b"! \x00" as *const u8 as
                                              *const i8);
                        }
                        print_cstr(b"Infinite glue shrinkage inserted from \x00"
                                       as *const u8 as *const i8);
                        print_esc_cstr(b"skip\x00" as *const u8 as
                                           *const i8);
                        print_int(n as i32);
                        help_ptr = 3i32 as u8;
                        help_line[2] =
                            b"The correction glue for page breaking with insertions\x00"
                                as *const u8 as *const i8;
                        help_line[1] =
                            b"must have finite shrinkability. But you may proceed,\x00"
                                as *const u8 as *const i8;
                        help_line[0] =
                            b"since the offensive shrinkability has been made finite.\x00"
                                as *const u8 as *const i8;
                        error();
                    }
                }
                if (*mem.offset(r as isize)).b16.s1 as i32 == 1i32 {
                    insert_penalties +=
                        (*mem.offset((p + 1i32) as isize)).b32.s1
                } else {
                    (*mem.offset((r + 2i32) as isize)).b32.s1 = p;
                    delta =
                        page_so_far[0] - page_so_far[1] - page_so_far[7] +
                            page_so_far[6];
                    if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
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
                                          (0x10ffffi32 + 1i32) + 85i32 +
                                          n as i32) as isize)).b32.s1
                           == 1000i32 {
                        h = (*mem.offset((p + 3i32) as isize)).b32.s1
                    } else {
                        h =
                            x_over_n((*mem.offset((p + 3i32) as
                                                      isize)).b32.s1, 1000i32)
                                *
                                (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) + 1i32
                                                   + 15000i32 + 12i32 +
                                                   9000i32 + 1i32 + 1i32 +
                                                   19i32 + 256i32 + 256i32 +
                                                   13i32 + 256i32 + 4i32 +
                                                   256i32 + 1i32 +
                                                   3i32 * 256i32 +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   85i32 + n as i32)
                                                  as isize)).b32.s1
                    }
                    if (h <= 0i32 || h <= delta) &&
                           (*mem.offset((p + 3i32) as isize)).b32.s1 +
                               (*mem.offset((r + 3i32) as isize)).b32.s1 <=
                               (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) + 1i32
                                                  + 15000i32 + 12i32 + 9000i32
                                                  + 1i32 + 1i32 + 19i32 +
                                                  256i32 + 256i32 + 13i32 +
                                                  256i32 + 4i32 + 256i32 +
                                                  1i32 + 3i32 * 256i32 +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) + 85i32
                                                  + 256i32 +
                                                  (0x10ffffi32 + 1i32) + 23i32
                                                  + n as i32) as
                                                 isize)).b32.s1 {
                        page_so_far[0] -= h;
                        let ref mut fresh8 =
                            (*mem.offset((r + 3i32) as isize)).b32.s1;
                        *fresh8 += (*mem.offset((p + 3i32) as isize)).b32.s1
                    } else {
                        /*1045: "Find the best way to split the insertion, and
                     * change type(r) to split_up." ... "Here is code that
                     * will split a long footnote between pages, in an
                     * emergency ... Node `p` is an insertion into box `n`;
                     * the insertion will not fit, in its entirety, either
                     * because it would make the total contents of box `n`
                     * greater then `\dimen n`, or because it would make the
                     * incremental amount of growth `h` greater than the
                     * available space `delta`, or both. (This amount `h` has
                     * been weighted by the insertion scaling factor, i.e., by
                     * `\count n` over 1000.) Now we will choose the best way
                     * to break the vlist of the insertion, using the same
                     * criteria as in the `\vsplit` operation." */
                        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
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
                                              (0x10ffffi32 + 1i32) + 85i32 +
                                              n as i32) as
                                             isize)).b32.s1 <= 0i32 {
                            w = 0x3fffffffi32
                        } else {
                            w =
                                page_so_far[0] - page_so_far[1] -
                                    page_so_far[7];
                            if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) + 1i32
                                                  + 15000i32 + 12i32 + 9000i32
                                                  + 1i32 + 1i32 + 19i32 +
                                                  256i32 + 256i32 + 13i32 +
                                                  256i32 + 4i32 + 256i32 +
                                                  1i32 + 3i32 * 256i32 +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) + 85i32
                                                  + n as i32) as
                                                 isize)).b32.s1 != 1000i32 {
                                w =
                                    x_over_n(w,
                                             (*eqtb.offset((1i32 +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                1i32 +
                                                                15000i32 +
                                                                12i32 +
                                                                9000i32 + 1i32
                                                                + 1i32 + 19i32
                                                                + 256i32 +
                                                                256i32 + 13i32
                                                                + 256i32 +
                                                                4i32 + 256i32
                                                                + 1i32 +
                                                                3i32 * 256i32
                                                                +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                (0x10ffffi32 +
                                                                     1i32) +
                                                                85i32 +
                                                                n as
                                                                    i32)
                                                               as
                                                               isize)).b32.s1)
                                        * 1000i32
                            }
                        }
                        if w >
                               (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) + 1i32
                                                  + 15000i32 + 12i32 + 9000i32
                                                  + 1i32 + 1i32 + 19i32 +
                                                  256i32 + 256i32 + 13i32 +
                                                  256i32 + 4i32 + 256i32 +
                                                  1i32 + 3i32 * 256i32 +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) +
                                                  (0x10ffffi32 + 1i32) + 85i32
                                                  + 256i32 +
                                                  (0x10ffffi32 + 1i32) + 23i32
                                                  + n as i32) as
                                                 isize)).b32.s1 -
                                   (*mem.offset((r + 3i32) as isize)).b32.s1 {
                            w =
                                (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) + 1i32
                                                   + 15000i32 + 12i32 +
                                                   9000i32 + 1i32 + 1i32 +
                                                   19i32 + 256i32 + 256i32 +
                                                   13i32 + 256i32 + 4i32 +
                                                   256i32 + 1i32 +
                                                   3i32 * 256i32 +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   (0x10ffffi32 + 1i32) +
                                                   85i32 + 256i32 +
                                                   (0x10ffffi32 + 1i32) +
                                                   23i32 + n as i32)
                                                  as isize)).b32.s1 -
                                    (*mem.offset((r + 3i32) as isize)).b32.s1
                        }
                        q =
                            vert_break((*mem.offset((p + 4i32) as
                                                        isize)).b32.s0, w,
                                       (*mem.offset((p + 2i32) as
                                                        isize)).b32.s1);
                        let ref mut fresh9 =
                            (*mem.offset((r + 3i32) as isize)).b32.s1;
                        *fresh9 += best_height_plus_depth;
                        if (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32) +
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
                                              (0x10ffffi32 + 1i32) + 85i32 +
                                              n as i32) as
                                             isize)).b32.s1 != 1000i32 {
                            best_height_plus_depth =
                                x_over_n(best_height_plus_depth, 1000i32) *
                                    (*eqtb.offset((1i32 + (0x10ffffi32 + 1i32)
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
                                                       + 85i32 +
                                                       n as i32) as
                                                      isize)).b32.s1
                        }
                        page_so_far[0] -= best_height_plus_depth;
                        (*mem.offset(r as isize)).b16.s1 = 1i32 as u16;
                        (*mem.offset((r + 1i32) as isize)).b32.s1 = q;
                        (*mem.offset((r + 1i32) as isize)).b32.s0 = p;
                        if q == -0xfffffffi32 {
                            insert_penalties += -10000i32
                        } else if (*mem.offset(q as isize)).b16.s1 as
                                      i32 == 12i32 {
                            insert_penalties +=
                                (*mem.offset((q + 1i32) as isize)).b32.s1
                        }
                    }
                }
                current_block = 11918621130838443904;
            }
            _ => {
                confusion(b"page\x00" as *const u8 as *const i8);
            }
        }
        match current_block {
            13253659531982233645 =>
            /*1040: "Check if node p is the new champion breakpoint; then if it is
         * time for a page break, prepare for output, and either fire up the
         * user's output routine and return or ship out the page and goto
         * done." We reach this point when p is a glue, kern, or penalty, and
         * there's already content on the page -- so this might be a place to
         * break the page. */
            {
                if pi < 10000i32 {
                    /*1042: "Compute the badness b of the current page, using
             * awful_bad if the box is too full." */
                    if page_so_far[1] < page_so_far[0] {
                        if page_so_far[3] != 0i32 || page_so_far[4] != 0i32 ||
                               page_so_far[5] != 0i32 {
                            b = 0i32
                        } else {
                            b =
                                badness(page_so_far[0] - page_so_far[1],
                                        page_so_far[2])
                        }
                    } else if page_so_far[1] - page_so_far[0] > page_so_far[6]
                     {
                        b = 0x3fffffffi32
                    } else {
                        b =
                            badness(page_so_far[1] - page_so_far[0],
                                    page_so_far[6])
                    }
                    if b < 0x3fffffffi32 {
                        if pi <= -10000i32 {
                            c = pi
                        } else if b < 10000i32 {
                            c = b + pi + insert_penalties
                        } else { c = 100000i64 as i32 }
                        /* DEPLORABLE */
                    } else { c = b }
                    if insert_penalties >= 10000i32 { c = 0x3fffffffi32 }
                    if c <= least_page_cost {
                        best_page_break = p;
                        best_size = page_so_far[0];
                        least_page_cost = c;
                        r = (*mem.offset(4999999)).b32.s1;
                        while r != 4999999i32 {
                            (*mem.offset((r + 2i32) as isize)).b32.s0 =
                                (*mem.offset((r + 2i32) as isize)).b32.s1;
                            r = (*mem.offset(r as isize)).b32.s1
                        }
                    }
                    if c == 0x3fffffffi32 || pi <= -10000i32 {
                        fire_up(p);
                        if output_active {
                            /* "the page has been shipped out by the default output routine" */
                            /* "user's output routine will act" */
                            return
                        }
                        current_block = 15427931788582360902;
                    } else { current_block = 433373112845341403; }
                } else { current_block = 433373112845341403; }
                match current_block {
                    15427931788582360902 => { }
                    _ =>
                    /* ... resuming 1032 ... I believe the "goto" here can only be
         * triggered if p is a penalty node, and we decided not to break. */
                    {
                        if ((*mem.offset(p as isize)).b16.s1 as i32) <
                               10i32 ||
                               (*mem.offset(p as isize)).b16.s1 as i32
                                   > 11i32 {
                            current_block = 11918621130838443904;
                        } else { current_block = 5579886686420104461; }
                    }
                }
            }
            15559656170992153795 => {
                /*1034: "Recycle node p". This codepath is triggered if we encountered
         * something nonprinting (glue, kern, penalty) and there aren't any
         * yes-printing boxes at the top of the page yet. When that happens,
         * we just discard the nonprinting node. */
                (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 =
                    (*mem.offset(p as isize)).b32.s1;
                (*mem.offset(p as isize)).b32.s1 = -0xfffffffi32;
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
                                      (0x10ffffi32 + 1i32) + 65i32) as
                                     isize)).b32.s1 <= 0i32 {
                    flush_node_list(p);
                } else {
                    /* `disc_ptr[LAST_BOX_CODE]` is `tail_page_disc`, the last item
             * removed by the page builder. `disc_ptr[LAST_BOX_CODE]` is
             * `page_disc`, the first item removed by the page builder.
             * `disc_ptr[VSPLIT_CODE]` is `split_disc`, the first item removed
             * by \vsplit. */
                    if disc_ptr[2] == -0xfffffffi32 {
                        disc_ptr[2] = p
                    } else { (*mem.offset(disc_ptr[1] as isize)).b32.s1 = p }
                    disc_ptr[1] = p
                }
                current_block = 15427931788582360902;
            }
            _ => { }
        }
        match current_block {
            5579886686420104461 => {
                /*1039: "Update the current page measurements with respect to the glue or kern
         * specified by node p" */
                if (*mem.offset(p as isize)).b16.s1 as i32 == 11i32 {
                    q = p
                } else {
                    q = (*mem.offset((p + 1i32) as isize)).b32.s0;
                    page_so_far[(2i32 +
                                     (*mem.offset(q as isize)).b16.s1 as
                                         i32) as usize] +=
                        (*mem.offset((q + 2i32) as isize)).b32.s1;
                    page_so_far[6] +=
                        (*mem.offset((q + 3i32) as isize)).b32.s1;
                    if (*mem.offset(q as isize)).b16.s0 as i32 != 0i32
                           &&
                           (*mem.offset((q + 3i32) as isize)).b32.s1 != 0i32 {
                        if file_line_error_style_p != 0 {
                            print_file_line();
                        } else {
                            print_nl_cstr(b"! \x00" as *const u8 as
                                              *const i8);
                        }
                        print_cstr(b"Infinite glue shrinkage found on current page\x00"
                                       as *const u8 as *const i8);
                        help_ptr = 4i32 as u8;
                        help_line[3] =
                            b"The page about to be output contains some infinitely\x00"
                                as *const u8 as *const i8;
                        help_line[2] =
                            b"shrinkable glue, e.g., `\\vss\' or `\\vskip 0pt minus 1fil\'.\x00"
                                as *const u8 as *const i8;
                        help_line[1] =
                            b"Such glue doesn\'t belong there; but you can safely proceed,\x00"
                                as *const u8 as *const i8;
                        help_line[0] =
                            b"since the offensive shrinkability has been made finite.\x00"
                                as *const u8 as *const i8;
                        error();
                        r = new_spec(q);
                        (*mem.offset(r as isize)).b16.s0 = 0i32 as u16;
                        delete_glue_ref(q);
                        (*mem.offset((p + 1i32) as isize)).b32.s0 = r;
                        q = r
                    }
                }
                page_so_far[1] +=
                    page_so_far[7] +
                        (*mem.offset((q + 1i32) as isize)).b32.s1;
                page_so_far[7] = 0i32;
                current_block = 11918621130838443904;
            }
            _ => { }
        }
        match current_block {
            11918621130838443904 => {
                /*1038: "Make sure that page_max_depth is not exceeded." */
                if page_so_far[7] > page_max_depth {
                    page_so_far[1] += page_so_far[7] - page_max_depth;
                    page_so_far[7] = page_max_depth
                }
                /*1033: "Link node p into the current page and goto done." */
                (*mem.offset(page_tail as isize)).b32.s1 =
                    p; /* "vertical mode" */
                page_tail = p;
                (*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 =
                    (*mem.offset(p as isize)).b32.s1;
                (*mem.offset(p as isize)).b32.s1 = -0xfffffffi32
            }
            _ => { }
        }
        if !((*mem.offset((4999999i32 - 1i32) as isize)).b32.s1 !=
                 -0xfffffffi32) {
            break ;
        }
    }
    if nest_ptr == 0i32 {
        cur_list.tail = 4999999i32 - 1i32
    } else {
        (*nest.offset(0)).tail = 4999999i32 - 1i32
    };
    /* "other modes" */
}
