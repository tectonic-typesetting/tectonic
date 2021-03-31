/* Copyright 2016-2018 The Tectonic Project
 * Licensed under the MIT License.
 */

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-synctex.h"
#include "tectonic_bridge_core.h"


static scaled_t math_x_height(int32_t size_code);
static scaled_t math_quad(int32_t size_code);
static scaled_t num1(int32_t size_code);
static scaled_t num2(int32_t size_code);
static scaled_t num3(int32_t size_code);
static scaled_t denom1(int32_t size_code);
static scaled_t denom2(int32_t size_code);
static scaled_t sup1(int32_t size_code);
static scaled_t sup2(int32_t size_code);
static scaled_t sup3(int32_t size_code);
static scaled_t sub1(int32_t size_code);
static scaled_t sub2(int32_t size_code);
static scaled_t sup_drop(int32_t size_code);
static scaled_t sub_drop(int32_t size_code);
static scaled_t delim1(int32_t size_code);
static scaled_t delim2(int32_t size_code);
static scaled_t axis_height(int32_t size_code);
static scaled_t default_rule_thickness(void);
static scaled_t big_op_spacing1(void);
static scaled_t big_op_spacing2(void);
static scaled_t big_op_spacing3(void);
static scaled_t big_op_spacing4(void);
static scaled_t big_op_spacing5(void);
static int32_t fraction_rule(scaled_t t);
static int32_t overbar(int32_t b, scaled_t k, scaled_t t);
static int32_t char_box(internal_font_number f, int32_t c);
static void stack_into_box(int32_t b, internal_font_number f, uint16_t c);
static scaled_t height_plus_depth(internal_font_number f, uint16_t c);
static void stack_glyph_into_box(int32_t b, internal_font_number f, int32_t g);
static void stack_glue_into_box(int32_t b, scaled_t min, scaled_t max);
static int32_t build_opentype_assembly(internal_font_number f, void *a, scaled_t s, bool horiz);
static int32_t var_delimiter(int32_t d, int32_t s, scaled_t v);
static int32_t rebox(int32_t b, scaled_t w);
static int32_t math_glue(int32_t g, scaled_t m);
static void math_kern(int32_t p, scaled_t m);
static int32_t clean_box(int32_t p, small_number s);
static void fetch(int32_t a);
static void make_over(int32_t q);
static void make_under(int32_t q);
static void make_vcenter(int32_t q);
static void make_radical(int32_t q);
static scaled_t compute_ot_math_accent_pos(int32_t p);
static void make_math_accent(int32_t q);
static void make_fraction(int32_t q);
static scaled_t make_op(int32_t q);
static void make_ord(int32_t q);
static int32_t attach_hkern_to_new_hlist(int32_t q, scaled_t delta);
static void make_scripts(int32_t q, scaled_t delta);
static small_number make_left_right(int32_t q, small_number style, scaled_t max_d, scaled_t max_h);
static void mlist_to_hlist(void);


static b16x4 null_delimiter;
static int32_t cur_mlist;
static small_number cur_style;
static int32_t cur_size;
static scaled_t cur_mu;
static bool mlist_penalties;

void
initialize_math_variables(void)
{
    null_delimiter.s3 = 0;
    null_delimiter.s2 = 0;
    null_delimiter.s1 = 0;
    null_delimiter.s0 = 0;
}


void init_math(void)
{
    scaled_t w;
    int32_t j;
    int32_t x;
    scaled_t l;
    scaled_t s;
    int32_t p;
    int32_t q;
    internal_font_number f;
    int32_t n;
    scaled_t v;
    scaled_t d;

    get_token();

    if ((cur_cmd == MATH_SHIFT) && (cur_list.mode > 0)) { /*1180: */
        j = TEX_NULL;
        w = -MAX_HALFWORD;
        if (cur_list.head == cur_list.tail) {       /*1520: */
            pop_nest();
            if (cur_list.eTeX_aux == TEX_NULL)
                x = 0;
            else if (mem[cur_list.eTeX_aux].b32.s0 >= R_CODE)
                x = -1;
            else
                x = 1 /*:1519 */ ;
        } else {
            line_break(true);
            /*1528: */
            if (GLUEPAR(right_skip) == 0)
                j = new_kern(0);
            else
                j = new_param_glue(GLUE_PAR__right_skip);

            if (GLUEPAR(left_skip) == 0)
                p = new_kern(0);
            else
                p = new_param_glue(GLUE_PAR__left_skip);

            mem[p].b32.s1 = j;

            j = new_null_box();
            mem[j + 1].b32.s1 = mem[just_box + 1].b32.s1;
            mem[j + 4].b32.s1 = mem[just_box + 4].b32.s1;
            mem[j + 5].b32.s1 = p;
            mem[j + 5].b16.s0 = mem[just_box + 5].b16.s0;
            mem[j + 5].b16.s1 = mem[just_box + 5].b16.s1;
            BOX_glue_set(j) = BOX_glue_set(just_box);

            v = mem[just_box + 4].b32.s1;
            if (cur_list.eTeX_aux == TEX_NULL)
                x = 0;
            else if (mem[cur_list.eTeX_aux].b32.s0 >= R_CODE)
                x = -1;
            else
                x = 1 /*:1519 */ ;
            if (x >= 0) {
                p = mem[just_box + 5].b32.s1;
                mem[TEMP_HEAD].b32.s1 = TEX_NULL;
            } else {

                v = -(int32_t) v - mem[just_box + 1].b32.s1;
                p = new_math(0, BEGIN_L_CODE);
                mem[TEMP_HEAD].b32.s1 = p;
                just_copy(mem[just_box + 5].b32.s1, p, new_math(0, END_L_CODE));
                cur_dir = RIGHT_TO_LEFT;
            }
            v = v + 2 * font_info[QUAD_CODE + param_base[eqtb[CUR_FONT_LOC].b32.s1]].b32.s1;
            if (INTPAR(texxet) > 0) {    /*1497: */
                temp_ptr = get_avail();
                mem[temp_ptr].b32.s0 = BEFORE;
                mem[temp_ptr].b32.s1 = LR_ptr;
                LR_ptr = temp_ptr;
            }
            while (p != TEX_NULL) {

            reswitch:
                if ((is_char_node(p))) {
                    f = CHAR_NODE_font(p);
                    d = FONT_CHARACTER_WIDTH(f,
                                             effective_char(true, f, CHAR_NODE_character(p)));
                    goto found;
                }
                switch (mem[p].b16.s1) {
                case 0:
                case 1:
                case 2:
                    {
                        d = mem[p + 1].b32.s1;
                        goto found;
                    }
                    break;
                case 6:
                    {
                        mem[GARBAGE] = mem[p + 1];
                        mem[GARBAGE].b32.s1 = mem[p].b32.s1;
                        p = GARBAGE;
                        xtx_ligature_present = true;
                        goto reswitch;
                    }
                    break;
                case 11:
                    d = mem[p + 1].b32.s1;
                    break;
                case 40:
                    d = mem[p + 1].b32.s1;
                    break;
                case 9:
                    {
                        d = mem[p + 1].b32.s1;
                        if (INTPAR(texxet) > 0) {        /*1525: */
                            if (odd(mem[p].b16.s0)) {
                                if (mem[LR_ptr].b32.s0 == (L_CODE * (mem[p].b16.s0 / L_CODE) + 3)) {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem[temp_ptr].b32.s1;
                                    {
                                        mem[temp_ptr].b32.s1 = avail;
                                        avail = temp_ptr;
                                    }
                                } else if (mem[p].b16.s0 > L_CODE) {
                                    w = MAX_HALFWORD;
                                    goto done;
                                }
                            } else {

                                {
                                    temp_ptr = get_avail();
                                    mem[temp_ptr].b32.s0 = (L_CODE * (mem[p].b16.s0 / L_CODE) + 3);
                                    mem[temp_ptr].b32.s1 = LR_ptr;
                                    LR_ptr = temp_ptr;
                                }
                                if ((mem[p].b16.s0 / R_CODE) != cur_dir) {
                                    just_reverse(p);
                                    p = TEMP_HEAD;
                                }
                            }
                        } else if (mem[p].b16.s0 >= L_CODE) {
                            w = MAX_HALFWORD;
                            goto done;
                        }
                    }
                    break;
                case 14:
                    {
                        d = mem[p + 1].b32.s1;
                        cur_dir = mem[p].b16.s0;
                    }
                    break;
                case 10:
                    {
                        q = mem[p + 1].b32.s0;
                        d = mem[q + 1].b32.s1;
                        if (mem[just_box + 5].b16.s1 == STRETCHING) {
                            if ((mem[just_box + 5].b16.s0 == mem[q].b16.s1) && (mem[q + 2].b32.s1 != 0))
                                v = MAX_HALFWORD;
                        } else if (mem[just_box + 5].b16.s1 == SHRINKING) {
                            if ((mem[just_box + 5].b16.s0 == mem[q].b16.s0) && (mem[q + 3].b32.s1 != 0))
                                v = MAX_HALFWORD;
                        }
                        if (mem[p].b16.s0 >= A_LEADERS)
                            goto found;
                    }
                    break;
                case 8:
                    if ((mem[p].b16.s0 == NATIVE_WORD_NODE) || (mem[p].b16.s0 == NATIVE_WORD_NODE_AT)
                        || (mem[p].b16.s0 == GLYPH_NODE) || (mem[p].b16.s0 == PIC_NODE)
                        || (mem[p].b16.s0 == PDF_NODE)) {
                        d = mem[p + 1].b32.s1;
                        goto found;
                    } else
                        d = 0 /*:1398 */ ;
                    break;
                default:
                    d = 0;
                    break;
                }
                if (v < MAX_HALFWORD)
                    v = v + d;
                goto not_found;
 found:
                if (v < MAX_HALFWORD) {
                    v = v + d;
                    w = v;
                } else {

                    w = MAX_HALFWORD;
                    goto done;
                }
            not_found:
                p = LLIST_link(p);
            } /*1523:*/
        done:
            if (INTPAR(texxet) > 0) {
                while (LR_ptr != TEX_NULL) {

                    temp_ptr = LR_ptr;
                    LR_ptr = mem[temp_ptr].b32.s1;
                    {
                        mem[temp_ptr].b32.s1 = avail;
                        avail = temp_ptr;
                    }
                }
                if (LR_problems != 0) {
                    w = MAX_HALFWORD;
                    LR_problems = 0;
                }
            }
            cur_dir = LEFT_TO_RIGHT;
            flush_node_list(mem[TEMP_HEAD].b32.s1);
        }
        if (LOCAL(par_shape) == TEX_NULL) {

            if ((DIMENPAR(hang_indent) != 0)
                &&
                (((INTPAR(hang_after) >= 0)
                  && (cur_list.prev_graf + 2 > INTPAR(hang_after)))
                 || (cur_list.prev_graf + 1 < -(int32_t) INTPAR(hang_after)))) {
                l = DIMENPAR(hsize) - abs(DIMENPAR(hang_indent));
                if (DIMENPAR(hang_indent) > 0)
                    s = DIMENPAR(hang_indent);
                else
                    s = 0;
            } else {

                l = DIMENPAR(hsize);
                s = 0;
            }
        } else {

            n = mem[LOCAL(par_shape)].b32.s0;
            if (cur_list.prev_graf + 2 >= n)
                p = LOCAL(par_shape) + 2 * n;
            else
                p = LOCAL(par_shape) + 2 * (cur_list.prev_graf + 2);
            s = mem[p - 1].b32.s1;
            l = mem[p].b32.s1;
        }
        push_math(MATH_SHIFT_GROUP);
        cur_list.mode = MMODE;
        eq_word_define(INT_BASE + INT_PAR__cur_fam, -1);
        eq_word_define(DIMEN_BASE + DIMEN_PAR__pre_display_size, w);
        cur_list.eTeX_aux = j;
        eq_word_define(INT_BASE + INT_PAR__pre_display_correction, x);
        eq_word_define(DIMEN_BASE + DIMEN_PAR__display_width, l);
        eq_word_define(DIMEN_BASE + DIMEN_PAR__display_indent, s);
        if (LOCAL(every_display) != TEX_NULL)
            begin_token_list(LOCAL(every_display), EVERY_DISPLAY_TEXT);
        if (nest_ptr == 1)
            build_page();
    } else {

        back_input();
        {
            push_math(MATH_SHIFT_GROUP);
            eq_word_define(INT_BASE + INT_PAR__cur_fam, -1);
            if ((insert_src_special_every_math))
                insert_src_special();
            if (LOCAL(every_math) != TEX_NULL)
                begin_token_list(LOCAL(every_math), EVERY_MATH_TEXT);
        }
    }
}

void start_eq_no(void)
{

    save_stack[save_ptr + 0].b32.s1 = cur_chr;
    save_ptr++;

    push_math(MATH_SHIFT_GROUP);
    eq_word_define(INT_BASE + INT_PAR__cur_fam, -1);
    if (insert_src_special_every_math)
        insert_src_special();
    if (LOCAL(every_math) != TEX_NULL)
        begin_token_list(LOCAL(every_math), EVERY_MATH_TEXT);
}

void math_limit_switch(void)
{
    if (cur_list.head != cur_list.tail) {

        if (mem[cur_list.tail].b16.s1 == OP_NOAD) {
            mem[cur_list.tail].b16.s0 = cur_chr;
            return;
        }
    }
    error_here_with_diagnostic("Limit controls must follow a math operator");
    capture_to_diagnostic(NULL);
    {
        help_ptr = 1;
        help_line[0] = "I'm ignoring this misplaced \\limits or \\nolimits command.";
    }
    error();
}


static void
scan_delimiter(int32_t p, bool r)
{
    if (r) {
        if (cur_chr == 1) {
            cur_val1 = 0x40000000;
            scan_math_fam_int();
            cur_val1 += cur_val * 0x200000;
            scan_usv_num();
            cur_val += cur_val1;
        } else {
            scan_delimiter_int();
        }
    } else {
        do {
            get_x_token();
        } while (cur_cmd == SPACER || cur_cmd == RELAX);

        switch (cur_cmd) {
        case LETTER:
        case OTHER_CHAR:
            cur_val = DEL_CODE(cur_chr);
            break;

        case DELIM_NUM:
            if (cur_chr == 1) {
                cur_val1 = 0x40000000;
                scan_math_class_int();
                scan_math_fam_int();
                cur_val1 += cur_val * 0x20000;
                scan_usv_num();
                cur_val += cur_val1;
            } else {
                scan_delimiter_int();
            }
            break;

        default:
            cur_val = -1;
            break;
        }
    }

    if (cur_val < 0) {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl_cstr("! ");
        print_cstr("Missing delimiter (. inserted)");
        help_ptr = 6;
        help_line[5] = "I was expecting to see something like `(' or `\\{' or";
        help_line[4] = "`\\}' here. If you typed, e.g., `{' instead of `\\{', you";
        help_line[3] = "should probably delete the `{' by typing `1' now, so that";
        help_line[2] = "braces don't get unbalanced. Otherwise just proceed.";
        help_line[1] = "Acceptable delimiters are characters whose \\delcode is";
        help_line[0] = "nonnegative, or you can use `\\delimiter <delimiter code>'.";
        back_error();
        cur_val = 0;
    }

    if (cur_val >= 0x40000000) {
        mem[p].b16.s3 = ((cur_val % 0x200000) / 0x10000) * 0x100 + (cur_val / 0x200000) % 0x100;
        mem[p].b16.s2 = cur_val % 0x10000;
        mem[p].b16.s1 = 0;
        mem[p].b16.s0 = 0;
    } else {
        mem[p].b16.s3 = (cur_val / 0x100000) % 16;
        mem[p].b16.s2 = (cur_val / 0x1000) % 0x100;
        mem[p].b16.s1 = (cur_val / 0x100) % 16;
        mem[p].b16.s0 = cur_val % 0x100;
    }
}


void math_radical(void)
{
    {
        mem[cur_list.tail].b32.s1 = get_node(RADICAL_NOAD_SIZE);
        cur_list.tail = LLIST_link(cur_list.tail);
    }
    mem[cur_list.tail].b16.s1 = RADICAL_NOAD;
    mem[cur_list.tail].b16.s0 = NORMAL;
    mem[cur_list.tail + 1].b32 = empty;
    mem[cur_list.tail + 3].b32 = empty;
    mem[cur_list.tail + 2].b32 = empty;
    scan_delimiter(cur_list.tail + 4, true);
    scan_math(cur_list.tail + 1);
}

void math_ac(void)
{
    int32_t c;

    if (cur_cmd == ACCENT) {   /*1201: */
        error_here_with_diagnostic("Please use ");
        print_esc_cstr("mathaccent");
        print_cstr(" for accents in math mode");
        capture_to_diagnostic(NULL);
        {
            help_ptr = 2;
            help_line[1] = "I'm changing \\accent to \\mathaccent here; wish me luck.";
            help_line[0] = "(Accents are not the same in formulas as they are in text.)";
        }
        error();
    }
    {
        mem[cur_list.tail].b32.s1 = get_node(ACCENT_NOAD_SIZE);
        cur_list.tail = LLIST_link(cur_list.tail);
    }
    mem[cur_list.tail].b16.s1 = ACCENT_NOAD;
    mem[cur_list.tail].b16.s0 = NORMAL;
    mem[cur_list.tail + 1].b32 = empty;
    mem[cur_list.tail + 3].b32 = empty;
    mem[cur_list.tail + 2].b32 = empty;
    mem[cur_list.tail + 4].b32.s1 = MATH_CHAR;
    if (cur_chr == 1) {
        if (scan_keyword("fixed"))
            mem[cur_list.tail].b16.s0 = FIXED_ACC;
        else if (scan_keyword("bottom")) {
            if (scan_keyword("fixed"))
                mem[cur_list.tail].b16.s0 = (BOTTOM_ACC + 1);
            else
                mem[cur_list.tail].b16.s0 = BOTTOM_ACC;
        }
        scan_math_class_int();
        c = set_class(cur_val);
        scan_math_fam_int();
        c = c + set_family(cur_val);
        scan_usv_num();
        cur_val = cur_val + c;
    } else {

        scan_fifteen_bit_int();
        cur_val = set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);
    }
    mem[cur_list.tail + 4].b16.s0 = cur_val % 65536L;
    if ((math_class(cur_val) == 7)
        && ((INTPAR(cur_fam) >= 0)
            && (INTPAR(cur_fam) < NUMBER_MATH_FAMILIES)))
        mem[cur_list.tail + 4].b16.s1 = INTPAR(cur_fam);
    else
        mem[cur_list.tail + 4].b16.s1 = math_fam(cur_val);
    mem[cur_list.tail + 4].b16.s1 = mem[cur_list.tail + 4].b16.s1 + (math_char(cur_val) / 65536L) * 256;
    scan_math(cur_list.tail + 1);
}

void append_choices(void)
{
    {
        mem[cur_list.tail].b32.s1 = new_choice();
        cur_list.tail = LLIST_link(cur_list.tail);
    }
    save_ptr++;
    save_stack[save_ptr - 1].b32.s1 = 0;
    push_math(MATH_CHOICE_GROUP);
    scan_left_brace();
}

int32_t fin_mlist(int32_t p)
{
    int32_t q;
    if (cur_list.aux.b32.s1 != TEX_NULL) {       /*1220: */
        mem[cur_list.aux.b32.s1 + 3].b32.s1 = SUB_MLIST;
        mem[cur_list.aux.b32.s1 + 3].b32.s0 = mem[cur_list.head].b32.s1;
        if (p == TEX_NULL)
            q = cur_list.aux.b32.s1;
        else {

            q = mem[cur_list.aux.b32.s1 + 2].b32.s0;
            if ((mem[q].b16.s1 != LEFT_NOAD) || (cur_list.eTeX_aux == TEX_NULL))
                confusion("right");
            mem[cur_list.aux.b32.s1 + 2].b32.s0 = mem[cur_list.eTeX_aux].b32.s1;
            mem[cur_list.eTeX_aux].b32.s1 = cur_list.aux.b32.s1;
            mem[cur_list.aux.b32.s1].b32.s1 = p;
        }
    } else {

        mem[cur_list.tail].b32.s1 = p;
        q = mem[cur_list.head].b32.s1;
    }
    pop_nest();
    return q;
}

void build_choices(void)
{
    int32_t p;
    unsave();
    p = fin_mlist(TEX_NULL);
    switch (save_stack[save_ptr - 1].b32.s1) {
    case 0:
        mem[cur_list.tail + 1].b32.s0 = p;
        break;
    case 1:
        mem[cur_list.tail + 1].b32.s1 = p;
        break;
    case 2:
        mem[cur_list.tail + 2].b32.s0 = p;
        break;
    case 3:
        {
            mem[cur_list.tail + 2].b32.s1 = p;
            save_ptr--;
            return;
        }
        break;
    }
    save_stack[save_ptr - 1].b32.s1++;
    push_math(MATH_CHOICE_GROUP);
    scan_left_brace();
}

void sub_sup(void)
{
    small_number t;
    int32_t p;
    t = EMPTY;
    p = TEX_NULL;
    if (cur_list.tail != cur_list.head) {

        if ((mem[cur_list.tail].b16.s1 >= ORD_NOAD)
            && (mem[cur_list.tail].b16.s1 < LEFT_NOAD)) {
            p = cur_list.tail + 2 + cur_cmd - 7;
            t = mem[p].b32.s1;
        }
    }
    if ((p == TEX_NULL) || (t != EMPTY)) {   /*1212: */
        {
            mem[cur_list.tail].b32.s1 = new_noad();
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        p = cur_list.tail + 2 + cur_cmd - 7;
        if (t != EMPTY) {
            if (cur_cmd == SUP_MARK) {
                error_here_with_diagnostic("Double superscript");
                capture_to_diagnostic(NULL);
                {
                    help_ptr = 1;
                    help_line[0] = "I treat `x^1^2' essentially like `x^1{}^2'.";
                }
            } else {

                error_here_with_diagnostic("Double subscript");
                capture_to_diagnostic(NULL);
                {
                    help_ptr = 1;
                    help_line[0] = "I treat `x_1_2' essentially like `x_1{}_2'.";
                }
            }
            error();
        }
    }
    scan_math(p);
}


void
math_fraction(void)
{
    small_number c;

    c = cur_chr;

    if (cur_list.aux.b32.s1 != TEX_NULL) { /*1218:*/
        if (c >= DELIMITED_CODE) {
            scan_delimiter(GARBAGE, false);
            scan_delimiter(GARBAGE, false);
        }

        if (c % DELIMITED_CODE == ABOVE_CODE)
            scan_dimen(false, false, false);

        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl_cstr("! ");
        print_cstr("Ambiguous; you need another { and }");
        help_ptr = 3;
        help_line[2] = "I'm ignoring this fraction specification, since I don't";
        help_line[1] = "know whether a construction like `x \\over y \\over z'";
        help_line[0] = "means `{x \\over y} \\over z' or `x \\over {y \\over z}'.";
        error();
    } else {
        cur_list.aux.b32.s1 = get_node(FRACTION_NOAD_SIZE);
        mem[cur_list.aux.b32.s1].b16.s1 = FRACTION_NOAD;
        mem[cur_list.aux.b32.s1].b16.s0 = NORMAL;
        mem[cur_list.aux.b32.s1 + 2].b32.s1 = SUB_MLIST;
        mem[cur_list.aux.b32.s1 + 2].b32.s0 = mem[cur_list.head].b32.s1;
        mem[cur_list.aux.b32.s1 + 3].b32 = empty;
        mem[cur_list.aux.b32.s1 + 4].b16 = null_delimiter;
        mem[cur_list.aux.b32.s1 + 5].b16 = null_delimiter;
        mem[cur_list.head].b32.s1 = TEX_NULL;

        cur_list.tail = cur_list.head;

        if (c >= DELIMITED_CODE) {
            scan_delimiter(cur_list.aux.b32.s1 + 4, false);
            scan_delimiter(cur_list.aux.b32.s1 + 5, false);
        }

        switch (c % DELIMITED_CODE) {
        case ABOVE_CODE:
            scan_dimen(false, false, false);
            mem[cur_list.aux.b32.s1 + 1].b32.s1 = cur_val;
            break;
        case OVER_CODE:
            mem[cur_list.aux.b32.s1 + 1].b32.s1 = DEFAULT_CODE;
            break;
        case ATOP_CODE:
            mem[cur_list.aux.b32.s1 + 1].b32.s1 = 0;
            break;
        }
    }
}


void math_left_right(void)
{
    small_number t;
    int32_t p;
    int32_t q;
    t = cur_chr;
    if ((t != LEFT_NOAD) && (cur_group != MATH_LEFT_GROUP)) { /*1227: */
        if (cur_group == MATH_SHIFT_GROUP) {
            scan_delimiter(GARBAGE, false);
            error_here_with_diagnostic("Extra ");
            if (t == 1) {
                print_esc_cstr("middle");
                {
                    help_ptr = 1;
                    help_line[0] = "I'm ignoring a \\middle that had no matching \\left.";
                }
            } else {

                print_esc_cstr("right");
                {
                    help_ptr = 1;
                    help_line[0] = "I'm ignoring a \\right that had no matching \\left.";
                }
            }
            capture_to_diagnostic(NULL);
            error();
        } else
            off_save();
    } else {

        p = new_noad();
        mem[p].b16.s1 = t;
        scan_delimiter(p + 1, false);
        if (t == 1) {
            mem[p].b16.s1 = RIGHT_NOAD;
            mem[p].b16.s0 = 1;
        }
        if (t == LEFT_NOAD)
            q = p;
        else {

            q = fin_mlist(p);
            unsave();
        }
        if (t != RIGHT_NOAD) {
            push_math(MATH_LEFT_GROUP);
            mem[cur_list.head].b32.s1 = q;
            cur_list.tail = p;
            cur_list.eTeX_aux = p;
        } else {

            {
                mem[cur_list.tail].b32.s1 = new_noad();
                cur_list.tail = LLIST_link(cur_list.tail);
            }
            mem[cur_list.tail].b16.s1 = INNER_NOAD;
            mem[cur_list.tail + 1].b32.s1 = SUB_MLIST;
            mem[cur_list.tail + 1].b32.s0 = q;
        }
    }
}


static void
app_display(int32_t j, int32_t b, scaled_t d)
{
    scaled_t z;
    scaled_t s;
    scaled_t e;
    int32_t x;
    int32_t p, q, r, t, u;

    s = DIMENPAR(display_indent);
    x = INTPAR(pre_display_correction);

    if (x == 0)
        mem[b + 4].b32.s1 = s + d;
    else {

        z = DIMENPAR(display_width);
        p = b;
        if (x > 0)
            e = z - d - mem[p + 1].b32.s1;
        else {

            e = d;
            d = z - e - mem[p + 1].b32.s1;
        }
        if (j != TEX_NULL) {
            b = copy_node_list(j);
            mem[b + 3].b32.s1 = mem[p + 3].b32.s1;
            mem[b + 2].b32.s1 = mem[p + 2].b32.s1;
            s = s - mem[b + 4].b32.s1;
            d = d + s;
            e = e + mem[b + 1].b32.s1 - z - s;
        }
        if ((mem[p].b16.s0) == DLIST)
            q = p;
        else {

            r = mem[p + 5].b32.s1;
            free_node(p, BOX_NODE_SIZE);
            if (r == TEX_NULL)
                confusion("LR4");
            if (x > 0) {
                p = r;
                do {
                    q = r;
                    r = LLIST_link(r);
                } while (!(r == TEX_NULL));
            } else {

                p = TEX_NULL;
                q = r;
                do {
                    t = mem[r].b32.s1;
                    mem[r].b32.s1 = p;
                    p = r;
                    r = t;
                } while (!(r == TEX_NULL));
            }
        }
        if (j == TEX_NULL) {
            r = new_kern(0);
            t = new_kern(0);
        } else {

            r = mem[b + 5].b32.s1;
            t = mem[r].b32.s1;
        }
        u = new_math(0, END_M_CODE);
        if (NODE_type(t) == GLUE_NODE) {
            j = new_skip_param(GLUE_PAR__right_skip);
            mem[q].b32.s1 = j;
            mem[j].b32.s1 = u;
            j = mem[t + 1].b32.s0;
            mem[temp_ptr].b16.s1 = mem[j].b16.s1;
            mem[temp_ptr].b16.s0 = mem[j].b16.s0;
            mem[temp_ptr + 1].b32.s1 = e - mem[j + 1].b32.s1;
            mem[temp_ptr + 2].b32.s1 = -(int32_t) mem[j + 2].b32.s1;
            mem[temp_ptr + 3].b32.s1 = -(int32_t) mem[j + 3].b32.s1;
            mem[u].b32.s1 = t;
        } else {

            mem[t + 1].b32.s1 = e;
            mem[t].b32.s1 = u;
            mem[q].b32.s1 = t;
        }
        u = new_math(0, BEGIN_M_CODE);
        if (NODE_type(r) == GLUE_NODE) {
            j = new_skip_param(GLUE_PAR__left_skip);
            mem[u].b32.s1 = j;
            mem[j].b32.s1 = p;
            j = mem[r + 1].b32.s0;
            mem[temp_ptr].b16.s1 = mem[j].b16.s1;
            mem[temp_ptr].b16.s0 = mem[j].b16.s0;
            mem[temp_ptr + 1].b32.s1 = d - mem[j + 1].b32.s1;
            mem[temp_ptr + 2].b32.s1 = -(int32_t) mem[j + 2].b32.s1;
            mem[temp_ptr + 3].b32.s1 = -(int32_t) mem[j + 3].b32.s1;
            mem[r].b32.s1 = u;
        } else {

            mem[r + 1].b32.s1 = d;
            mem[r].b32.s1 = p;
            mem[u].b32.s1 = r;
            if (j == TEX_NULL) {
                b = hpack(u, 0, ADDITIONAL);
                mem[b + 4].b32.s1 = s;
            } else
                mem[b + 5].b32.s1 = u;
        }
    }
    append_to_vlist(b);
}

void after_math(void)
{
    bool l;
    bool danger;
    int32_t m;
    int32_t p;
    int32_t a;
    int32_t b;
    scaled_t w;
    scaled_t z;
    scaled_t e;
    scaled_t q;
    scaled_t d;
    scaled_t s;
    small_number g1, g2;
    int32_t r;
    int32_t t;
    int32_t pre_t;
    int32_t j = TEX_NULL;

    danger = false;

    if (cur_list.mode == MMODE)
        j = cur_list.eTeX_aux /*:1530 */ ;
    if (((font_params[MATH_FONT(2)] < TOTAL_MATHSY_PARAMS)
         &&
         (!((font_area[MATH_FONT(2)] == OTGR_FONT_FLAG)
            && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2)])))))
        || ((font_params[MATH_FONT(2 + SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
            &&
            (!((font_area[MATH_FONT(2 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
               && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SIZE)])))))
        || ((font_params[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
            &&
            (!((font_area[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
               && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)])))))) {
        error_here_with_diagnostic("Math formula deleted: Insufficient symbol fonts");
        capture_to_diagnostic(NULL);
        {
            help_ptr = 3;
            help_line[2] = "Sorry, but I can't typeset math unless \\textfont 2";
            help_line[1] = "and \\scriptfont 2 and \\scriptscriptfont 2 have all";
            help_line[0] = "the \\fontdimen values needed in math symbol fonts.";
        }
        error();
        flush_math();
        danger = true;
    } else
        if (((font_params[MATH_FONT(3 + TEXT_SIZE)] < TOTAL_MATHEX_PARAMS)
             &&
             (!((font_area[MATH_FONT(3 + TEXT_SIZE)] == OTGR_FONT_FLAG)
                && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + TEXT_SIZE)])))))
            || ((font_params[MATH_FONT(3 + SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                &&
                (!((font_area[MATH_FONT(3 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SIZE)])))))
            || ((font_params[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                &&
                (!((font_area[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)])))))) {
        error_here_with_diagnostic("Math formula deleted: Insufficient extension fonts");
        capture_to_diagnostic(NULL);
        {
            help_ptr = 3;
            help_line[2] = "Sorry, but I can't typeset math unless \\textfont 3";
            help_line[1] = "and \\scriptfont 3 and \\scriptscriptfont 3 have all";
            help_line[0] = "the \\fontdimen values needed in math extension fonts.";
        }
        error();
        flush_math();
        danger = true;
    }
    m = cur_list.mode;
    l = false;
    p = fin_mlist(TEX_NULL);
    if (cur_list.mode == -(int32_t) m) {
        {
            get_x_token();
            if (cur_cmd != MATH_SHIFT) {
                error_here_with_diagnostic("Display math should end with $$");
                capture_to_diagnostic(NULL);
                {
                    help_ptr = 2;
                    help_line[1] = "The `$' that I just saw supposedly matches a previous `$$'.";
                    help_line[0] = "So I shall assume that you typed `$$' both times.";
                }
                back_error();
            }
        }
        cur_mlist = p;
        cur_style = TEXT_STYLE;
        mlist_penalties = false;
        mlist_to_hlist();
        a = hpack(mem[TEMP_HEAD].b32.s1, 0, ADDITIONAL);
        mem[a].b16.s0 = DLIST;
        unsave();
        save_ptr--;
        if (save_stack[save_ptr + 0].b32.s1 == 1)
            l = true;
        danger = false;
        if (cur_list.mode == MMODE)
            j = cur_list.eTeX_aux /*:1530 */ ;
        if (((font_params[MATH_FONT(2)] < TOTAL_MATHSY_PARAMS)
             &&
             (!((font_area[MATH_FONT(2)] == OTGR_FONT_FLAG)
                && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2)])))))
            || ((font_params[MATH_FONT(2 + SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
                &&
                (!((font_area[MATH_FONT(2 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SIZE)])))))
            || ((font_params[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
                &&
                (!((font_area[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)])))))) {
            error_here_with_diagnostic("Math formula deleted: Insufficient symbol fonts");
            capture_to_diagnostic(NULL);
            {
                help_ptr = 3;
                help_line[2] = "Sorry, but I can't typeset math unless \\textfont 2";
                help_line[1] = "and \\scriptfont 2 and \\scriptscriptfont 2 have all";
                help_line[0] = "the \\fontdimen values needed in math symbol fonts.";
            }
            error();
            flush_math();
            danger = true;
        } else
            if (((font_params[MATH_FONT(3 + TEXT_SIZE)] < TOTAL_MATHEX_PARAMS)
                 &&
                 (!((font_area[MATH_FONT(3 + TEXT_SIZE)] == OTGR_FONT_FLAG)
                    && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + TEXT_SIZE)])))))
                || ((font_params[MATH_FONT(3 + SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                    &&
                    (!((font_area[MATH_FONT(3 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                       && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SIZE)])))))
                || ((font_params[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                    &&
                    (!((font_area[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                       &&
                       (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)])))))) {
            error_here_with_diagnostic("Math formula deleted: Insufficient extension fonts");
            capture_to_diagnostic(NULL);
            {
                help_ptr = 3;
                help_line[2] = "Sorry, but I can't typeset math unless \\textfont 3";
                help_line[1] = "and \\scriptfont 3 and \\scriptscriptfont 3 have all";
                help_line[0] = "the \\fontdimen values needed in math extension fonts.";
            }
            error();
            flush_math();
            danger = true;
        }
        m = cur_list.mode;
        p = fin_mlist(TEX_NULL);
    } else
        a = TEX_NULL;
    if (m < 0) {                /*1231: */
        {
            mem[cur_list.tail].b32.s1 = new_math(DIMENPAR(math_surround), BEFORE);
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        cur_mlist = p;
        cur_style = TEXT_STYLE;
        mlist_penalties = (cur_list.mode > 0);
        mlist_to_hlist();
        mem[cur_list.tail].b32.s1 = mem[TEMP_HEAD].b32.s1;
        while (mem[cur_list.tail].b32.s1 != TEX_NULL)
            cur_list.tail = LLIST_link(cur_list.tail);
        {
            mem[cur_list.tail].b32.s1 = new_math(DIMENPAR(math_surround), AFTER);
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        cur_list.aux.b32.s0 = 1000;
        unsave();
    } else {

        if (a == TEX_NULL) { /*1232: */
            get_x_token();
            if (cur_cmd != MATH_SHIFT) {
                error_here_with_diagnostic("Display math should end with $$");
                capture_to_diagnostic(NULL);
                {
                    help_ptr = 2;
                    help_line[1] = "The `$' that I just saw supposedly matches a previous `$$'.";
                    help_line[0] = "So I shall assume that you typed `$$' both times.";
                }
                back_error();
            }
        }
        cur_mlist = p;
        cur_style = DISPLAY_STYLE;
        mlist_penalties = false;
        mlist_to_hlist();
        p = mem[TEMP_HEAD].b32.s1;
        adjust_tail = ADJUST_HEAD;
        pre_adjust_tail = PRE_ADJUST_HEAD;
        b = hpack(p, 0, ADDITIONAL);
        p = mem[b + 5].b32.s1;
        t = adjust_tail;
        adjust_tail = TEX_NULL;
        pre_t = pre_adjust_tail;
        pre_adjust_tail = TEX_NULL;
        w = mem[b + 1].b32.s1;
        z = DIMENPAR(display_width);
        s = DIMENPAR(display_indent);
        if (INTPAR(pre_display_correction) < 0)
            s = -(int32_t) s - z;
        if ((a == TEX_NULL) || danger) {
            e = 0;
            q = 0;
        } else {

            e = mem[a + 1].b32.s1;
            q = e + math_quad(TEXT_SIZE);
        }
        if (w + q > z) {        /*1236: */
            if ((e != 0)
                && ((w - total_shrink[NORMAL] + q <= z) || (total_shrink[FIL] != 0)
                    || (total_shrink[FILL] != 0) || (total_shrink[FILLL] != 0))) {
                free_node(b, BOX_NODE_SIZE);
                b = hpack(p, z - q, EXACTLY);
            } else {

                e = 0;
                if (w > z) {
                    free_node(b, BOX_NODE_SIZE);
                    b = hpack(p, z, EXACTLY);
                }
            }
            w = mem[b + 1].b32.s1;
        }
        mem[b].b16.s0 = DLIST;
        d = half(z - w);
        if ((e > 0) && (d < 2 * e)) {
            d = half(z - w - e);
            if (p != TEX_NULL) {

                if (!(is_char_node(p))) {

                    if (NODE_type(p) == GLUE_NODE)
                        d = 0;
                }
            }
        }
        {
            mem[cur_list.tail].b32.s1 = new_penalty(INTPAR(pre_display_penalty));
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        if ((d + s <= DIMENPAR(pre_display_size)) || l) {
            g1 = GLUE_PAR__above_display_skip;
            g2 = GLUE_PAR__below_display_skip;
        } else {
            g1 = GLUE_PAR__above_display_short_skip;
            g2 = GLUE_PAR__below_display_short_skip;
        }
        if (l && (e == 0)) {
            app_display(j, a, 0);
            {
                mem[cur_list.tail].b32.s1 = new_penalty(INF_PENALTY);
                cur_list.tail = LLIST_link(cur_list.tail);
            }
        } else {

            mem[cur_list.tail].b32.s1 = new_param_glue(g1);
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        if (e != 0) {
            r = new_kern(z - w - e - d);
            if (l) {
                mem[a].b32.s1 = r;
                mem[r].b32.s1 = b;
                b = a;
                d = 0;
            } else {

                mem[b].b32.s1 = r;
                mem[r].b32.s1 = a;
            }
            b = hpack(b, 0, ADDITIONAL);
        }
        app_display(j, b, d);
        if ((a != TEX_NULL) && (e == 0) && !l) {
            {
                mem[cur_list.tail].b32.s1 = new_penalty(INF_PENALTY);
                cur_list.tail = LLIST_link(cur_list.tail);
            }
            app_display(j, a, z - mem[a + 1].b32.s1);
            g2 = 0;
        }
        if (t != ADJUST_HEAD) {
            mem[cur_list.tail].b32.s1 = mem[ADJUST_HEAD].b32.s1;
            cur_list.tail = t;
        }
        if (pre_t != PRE_ADJUST_HEAD) {
            mem[cur_list.tail].b32.s1 = mem[PRE_ADJUST_HEAD].b32.s1;
            cur_list.tail = pre_t;
        }
        {
            mem[cur_list.tail].b32.s1 = new_penalty(INTPAR(post_display_penalty));
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        if (g2 > 0) {
            mem[cur_list.tail].b32.s1 = new_param_glue(g2);
            cur_list.tail = LLIST_link(cur_list.tail);
        }
        flush_node_list(j);
        resume_after_display();
    }
}

void resume_after_display(void)
{

    if (cur_group != MATH_SHIFT_GROUP)
        confusion("display");

    unsave();
    cur_list.prev_graf = cur_list.prev_graf + 3;
    push_nest();
    cur_list.mode = HMODE;
    cur_list.aux.b32.s0 = 1000;
    if (INTPAR(language) <= 0)
        cur_lang = 0;
    else if (INTPAR(language) > BIGGEST_LANG)
        cur_lang = 0;
    else
        cur_lang = INTPAR(language);
    cur_list.aux.b32.s1 = cur_lang;
    cur_list.prev_graf =
        (norm_min(INTPAR(left_hyphen_min)) * 64 +
         norm_min(INTPAR(right_hyphen_min))) * 65536L + cur_lang;
    {
        get_x_token();
        if (cur_cmd != SPACER)
            back_input();
    }
    if (nest_ptr == 1)
        build_page();
}


static scaled_t
math_x_height(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 5);
    else
        rval = font_info[5 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
math_quad(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 6);
    else
        rval = font_info[6 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
num1(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 8);
    else
        rval = font_info[8 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
num2(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 9);
    else
        rval = font_info[9 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
num3(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 10);
    else
        rval = font_info[10 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
denom1(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 11);
    else
        rval = font_info[11 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
denom2(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 12);
    else
        rval = font_info[12 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
sup1(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 13);
    else
        rval = font_info[13 + param_base[f]].b32.s1;
    return rval;
}


static scaled_t
sup2(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 14);
    else
        rval = font_info[14 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
sup3(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 15);
    else
        rval = font_info[15 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
sub1(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 16);
    else
        rval = font_info[16 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
sub2(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 17);
    else
        rval = font_info[17 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
sup_drop(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 18);
    else
        rval = font_info[18 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
sub_drop(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 19);
    else
        rval = font_info[19 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
delim1(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 20);
    else
        rval = font_info[20 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
delim2(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 21);
    else
        rval = font_info[21 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
axis_height(int32_t size_code)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 22);
    else
        rval = font_info[22 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
default_rule_thickness(void)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 8);
    else
        rval = font_info[8 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
big_op_spacing1(void)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 9);
    else
        rval = font_info[9 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
big_op_spacing2(void)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 10);
    else
        rval = font_info[10 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
big_op_spacing3(void)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 11);
    else
        rval = font_info[11 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
big_op_spacing4(void)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 12);
    else
        rval = font_info[12 + param_base[f]].b32.s1;
    return rval;
}

static scaled_t
big_op_spacing5(void)
{
    int32_t f;
    scaled_t rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 13);
    else
        rval = font_info[13 + param_base[f]].b32.s1;
    return rval;
}

static int32_t
fraction_rule(scaled_t t)
{
    int32_t p;
    p = new_rule();
    mem[p + 3].b32.s1 = t;
    mem[p + 2].b32.s1 = 0;
    return p;
}


static int32_t
overbar(int32_t b, scaled_t k, scaled_t t)
{
    int32_t p, q;
    p = new_kern(k);
    mem[p].b32.s1 = b;
    q = fraction_rule(t);
    mem[q].b32.s1 = p;
    p = new_kern(t);
    mem[p].b32.s1 = q;
    return vpackage(p, 0, ADDITIONAL, MAX_HALFWORD);
}


static int32_t
math_glue(int32_t g, scaled_t m)
{
    int32_t p;
    int32_t n;
    scaled_t f;
    n = x_over_n(m, 65536L);
    f = tex_remainder;
    if (f < 0) {
        n--;
        f = f + 65536L;
    }
    p = get_node(GLUE_SPEC_SIZE);
    mem[p + 1].b32.s1 = mult_and_add(n, mem[g + 1].b32.s1, xn_over_d(mem[g + 1].b32.s1, f, 65536L), MAX_HALFWORD);
    mem[p].b16.s1 = mem[g].b16.s1;
    if (mem[p].b16.s1 == NORMAL)
        mem[p + 2].b32.s1 = mult_and_add(n, mem[g + 2].b32.s1, xn_over_d(mem[g + 2].b32.s1, f, 65536L), MAX_HALFWORD);
    else
        mem[p + 2].b32.s1 = mem[g + 2].b32.s1;
    mem[p].b16.s0 = mem[g].b16.s0;
    if (GLUE_SPEC_shrink_order(p) == NORMAL)
        mem[p + 3].b32.s1 = mult_and_add(n, mem[g + 3].b32.s1, xn_over_d(mem[g + 3].b32.s1, f, 65536L), MAX_HALFWORD);
    else
        mem[p + 3].b32.s1 = mem[g + 3].b32.s1;
    return p;
}


static void
math_kern(int32_t p, scaled_t m)
{
    int32_t n;
    scaled_t f;
    if (mem[p].b16.s0 == MU_GLUE) {
        n = x_over_n(m, 65536L);
        f = tex_remainder;
        if (f < 0) {
            n--;
            f = f + 65536L;
        }
        mem[p + 1].b32.s1 = mult_and_add(n, mem[p + 1].b32.s1, xn_over_d(mem[p + 1].b32.s1, f, 65536L), MAX_HALFWORD);
        NODE_subtype(p) = EXPLICIT;
    }
}


void
flush_math(void)
{
    flush_node_list(mem[cur_list.head].b32.s1);
    flush_node_list(cur_list.aux.b32.s1);
    mem[cur_list.head].b32.s1 = TEX_NULL;
    cur_list.tail = cur_list.head;
    cur_list.aux.b32.s1 = TEX_NULL;
}


static int32_t
clean_box(int32_t p, small_number s)
{
    int32_t q;
    small_number save_style;
    int32_t x;
    int32_t r;
    switch (mem[p].b32.s1) {
    case 1:
        {
            cur_mlist = new_noad();
            mem[cur_mlist + 1] = mem[p];
        }
        break;
    case 2:
        q = mem[p].b32.s0;
        goto found;
    case 3:
        cur_mlist = mem[p].b32.s0;
        break;
    default:
        q = new_null_box();
        goto found;
    }
    save_style = cur_style;
    cur_style = s;
    mlist_penalties = false;
    mlist_to_hlist();
    q = mem[TEMP_HEAD].b32.s1;
    cur_style = save_style;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
found:
    if ((is_char_node(q)) || (q == TEX_NULL))
        x = hpack(q, 0, ADDITIONAL);
    else if ((mem[q].b32.s1 == TEX_NULL) && (NODE_type(q) <= VLIST_NODE) && (mem[q + 4].b32.s1 == 0))
        x = q;
    else
        x = hpack(q, 0, ADDITIONAL);
    q = mem[x + 5].b32.s1;
    if ((is_char_node(q))) {
        r = mem[q].b32.s1;
        if (r != TEX_NULL) {

            if (mem[r].b32.s1 == TEX_NULL) {

                if (!(is_char_node(r))) {

                    if (NODE_type(r) == KERN_NODE) {
                        free_node(r, MEDIUM_NODE_SIZE);
                        mem[q].b32.s1 = TEX_NULL;
                    }
                }
            }
        }
    }
    return x;
}


static void
fetch(int32_t a)
{
    cur_c = (unsigned short) mem[a].b16.s0;
    cur_f = MATH_FONT((mem[a].b16.s1 % 256) + cur_size);
    cur_c = cur_c + (mem[a].b16.s1 / 256) * 65536L;
    if (cur_f == FONT_BASE) {   /*749: */
        error_here_with_diagnostic("");
        print_size(cur_size);
        print_char(' ');
        print_int((mem[a].b16.s1 % 256));
        print_cstr(" is undefined (character ");
        print(cur_c);
        print_char(')');
        capture_to_diagnostic(NULL);
        {
            help_ptr = 4;
            help_line[3] = "Somewhere in the math formula just ended, you used the";
            help_line[2] = "stated character from an undefined font family. For example,";
            help_line[1] = "plain TeX doesn't allow \\it or \\sl in subscripts. Proceed,";
            help_line[0] = "and I'll try to forget that I needed that character.";
        }
        error();
        cur_i = null_character;
        mem[a].b32.s1 = EMPTY;
    } else if (((font_area[cur_f] == AAT_FONT_FLAG) || (font_area[cur_f] == OTGR_FONT_FLAG))) {
        cur_i = null_character;
    } else {

        if ((cur_c >= font_bc[cur_f]) && (cur_c <= font_ec[cur_f]))
            cur_i = FONT_CHARACTER_INFO(cur_f, cur_c);
        else
            cur_i = null_character;
        if (!((cur_i.s3 > 0))) {
            char_warning(cur_f, cur_c);
            mem[a].b32.s1 = EMPTY;
        }
    }
}


static void
make_over(int32_t q)
{
        mem[q + 1].b32.s0 =
        overbar(clean_box(q + 1, 2 * (cur_style / 2) + 1), 3 * default_rule_thickness(), default_rule_thickness());
    mem[q + 1].b32.s1 = SUB_BOX;
}


static void
make_under(int32_t q)
{
    int32_t p, x, y;
    scaled_t delta;
    x = clean_box(q + 1, cur_style);
    p = new_kern(3 * default_rule_thickness());
    mem[x].b32.s1 = p;
    mem[p].b32.s1 = fraction_rule(default_rule_thickness());
    y = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
    delta = mem[y + 3].b32.s1 + mem[y + 2].b32.s1 + default_rule_thickness();
    mem[y + 3].b32.s1 = mem[x + 3].b32.s1;
    mem[y + 2].b32.s1 = delta - mem[y + 3].b32.s1;
    mem[q + 1].b32.s0 = y;
    mem[q + 1].b32.s1 = SUB_BOX;
}


static void
make_vcenter(int32_t q)
{
    int32_t v;
    scaled_t delta;
    v = mem[q + 1].b32.s0;
    if (NODE_type(v) != VLIST_NODE)
        confusion("vcenter");
    delta = mem[v + 3].b32.s1 + mem[v + 2].b32.s1;
    mem[v + 3].b32.s1 = axis_height(cur_size) + half(delta);
    mem[v + 2].b32.s1 = delta - mem[v + 3].b32.s1;
}


static void
make_radical(int32_t q)
{
    int32_t x, y;
    internal_font_number f;
    scaled_t rule_thickness;
    scaled_t delta, clr;

    f = MATH_FONT((mem[q + 4].b16.s3 % 256) + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rule_thickness = get_ot_math_constant(f, RADICALRULETHICKNESS);
    else
        rule_thickness = default_rule_thickness();
    x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f])))) {
        if (cur_style < TEXT_STYLE)
            clr = get_ot_math_constant(f, RADICALDISPLAYSTYLEVERTICALGAP);
        else
            clr = get_ot_math_constant(f, RADICALVERTICALGAP);
    } else {

        if (cur_style < TEXT_STYLE)
            clr = rule_thickness + (abs(math_x_height(cur_size)) / 4);
        else {

            clr = rule_thickness;
            clr = clr + (abs(clr) / 4);
        }
    }
    y = var_delimiter(q + 4, cur_size, mem[x + 3].b32.s1 + mem[x + 2].b32.s1 + clr + rule_thickness);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f])))) {
        mem[y + 2].b32.s1 = mem[y + 3].b32.s1 + mem[y + 2].b32.s1 - rule_thickness;
        mem[y + 3].b32.s1 = rule_thickness;
    }
    delta = mem[y + 2].b32.s1 - (mem[x + 3].b32.s1 + mem[x + 2].b32.s1 + clr);
    if (delta > 0)
        clr = clr + half(delta);
    mem[y + 4].b32.s1 = -(int32_t) (mem[x + 3].b32.s1 + clr);
    mem[y].b32.s1 = overbar(x, clr, mem[y + 3].b32.s1);
    mem[q + 1].b32.s0 = hpack(y, 0, ADDITIONAL);
    mem[q + 1].b32.s1 = SUB_BOX;
}


static scaled_t
compute_ot_math_accent_pos(int32_t p)
{
    int32_t q, r;
    scaled_t s, g;
    if (mem[p + 1].b32.s1 == MATH_CHAR) {
        fetch(p + 1);
        q = new_native_character(cur_f, cur_c);
        g = get_native_glyph(q, 0);
        s = get_ot_math_accent_pos(cur_f, g);
    } else {

        if (mem[p + 1].b32.s1 == SUB_MLIST) {
            r = mem[p + 1].b32.s0;
            if ((r != TEX_NULL) && (mem[r].b16.s1 == ACCENT_NOAD))
                s = compute_ot_math_accent_pos(r);
            else
                s = TEX_INFINITY;
        } else
            s = TEX_INFINITY;
    }
    return s;
}


static void
make_math_accent(int32_t q)
{
    int32_t p, x, y;
    int32_t a;
    int32_t c, g;
    internal_font_number f;
    b16x4 i;
    scaled_t s, sa;
    scaled_t h;
    scaled_t delta;
    scaled_t w, w2;
    void *ot_assembly_ptr;
    fetch(q + 4);
    x = TEX_NULL;
    ot_assembly_ptr = NULL;
    if (((font_area[cur_f] == AAT_FONT_FLAG) || (font_area[cur_f] == OTGR_FONT_FLAG))) {
        c = cur_c;
        f = cur_f;
        if (!((mem[q].b16.s0 == BOTTOM_ACC) || (mem[q].b16.s0 == (BOTTOM_ACC + 1))))
            s = compute_ot_math_accent_pos(q);
        else
            s = 0;
        x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
        w = mem[x + 1].b32.s1;
        h = mem[x + 3].b32.s1;
    } else if ((cur_i.s3 > 0)) {
        i = cur_i;
        c = cur_c;
        f = cur_f;
        s = 0;
        if (mem[q + 1].b32.s1 == MATH_CHAR) {
            fetch(q + 1);
            if (((cur_i.s1) % 4) == LIG_TAG) {
                a = lig_kern_base[cur_f] + cur_i.s0;
                cur_i = font_info[a].b16;
                if (cur_i.s3 > 128) {
                    a = lig_kern_base[cur_f] + 256 * cur_i.s1 + cur_i.s0 + 32768L - 256 * (128);
                    cur_i = font_info[a].b16;
                }
                while (true) {

                    if (cur_i.s2 == skew_char[cur_f]) {
                        if (cur_i.s1 >= 128) {

                            if (cur_i.s3 <= 128)
                                s = font_info[kern_base[cur_f] + 256 * cur_i.s1 + cur_i.s0].b32.s1;
                        }
                        goto done1;
                    }
                    if (cur_i.s3 >= 128)
                        goto done1;
                    a = a + cur_i.s3 + 1;
                    cur_i = font_info[a].b16;
                }
            }
        }
    done1:
        x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
        w = mem[x + 1].b32.s1;
        h = mem[x + 3].b32.s1;
        while (true) {

            if (((i.s1) % 4) != LIST_TAG)
                goto done;
            y = i.s0;
            i = FONT_CHARACTER_INFO(f, y);
            if (!(i.s3 > 0))
                goto done;
            if (FONT_CHARINFO_WIDTH(f, i) > w)
                goto done;
            c = y;
        }
        /*:767*/
    done:
        ;
    }
    if (x != TEX_NULL) {
        if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f])))) {

            if (((mem[q].b16.s0 == BOTTOM_ACC) || (mem[q].b16.s0 == (BOTTOM_ACC + 1))))
                delta = 0;
            else if (h < get_ot_math_constant(f, ACCENTBASEHEIGHT))
                delta = h;
            else
                delta = get_ot_math_constant(f, ACCENTBASEHEIGHT);
        } else if (h < font_info[X_HEIGHT_CODE + param_base[f]].b32.s1)
            delta = h;
        else
            delta = font_info[X_HEIGHT_CODE + param_base[f]].b32.s1;
        if ((mem[q + 2].b32.s1 != EMPTY) || (mem[q + 3].b32.s1 != EMPTY)) {

            if (mem[q + 1].b32.s1 == MATH_CHAR) {      /*769: */
                flush_node_list(x);
                x = new_noad();
                mem[x + 1] = mem[q + 1];
                mem[x + 2] = mem[q + 2];
                mem[x + 3] = mem[q + 3];
                mem[q + 2].b32 = empty;
                mem[q + 3].b32 = empty;
                mem[q + 1].b32.s1 = SUB_MLIST;
                mem[q + 1].b32.s0 = x;
                x = clean_box(q + 1, cur_style);
                delta = delta + mem[x + 3].b32.s1 - h;
                h = mem[x + 3].b32.s1;
            }
        }
        y = char_box(f, c);
        if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
            p = get_node(GLYPH_NODE_SIZE);
            NODE_type(p) = WHATSIT_NODE;
            mem[p].b16.s0 = GLYPH_NODE;
            mem[p + 4].b16.s2 = f;
            mem[p + 4].b16.s1 = get_native_glyph(mem[y + 5].b32.s1, 0);
            set_native_glyph_metrics(p, 1);
            free_node(mem[y + 5].b32.s1, mem[mem[y + 5].b32.s1 + 4].b16.s3);
            mem[y + 5].b32.s1 = p;
            if (odd(mem[q].b16.s0))
                set_native_glyph_metrics(p, 1);
            else {

                c = mem[p + 4].b16.s1;
                a = 0;
                do {
                    g = get_ot_math_variant(f, c, a, &w2, 1);
                    if ((w2 > 0) && (w2 <= w)) {
                        mem[p + 4].b16.s1 = g;
                        set_native_glyph_metrics(p, 1);
                        a++;
                    }
                } while (!((w2 < 0) || (w2 >= w)));
                if ((w2 < 0)) {
                    ot_assembly_ptr = get_ot_assembly_ptr(f, c, 1);
                    if (ot_assembly_ptr != NULL) {
                        free_node(p, GLYPH_NODE_SIZE);
                        p = build_opentype_assembly(f, ot_assembly_ptr, w, 1);
                        mem[y + 5].b32.s1 = p;
                        goto found;
                    }
                } else
                    set_native_glyph_metrics(p, 1);
            }
        found:
            mem[y + 1].b32.s1 = mem[p + 1].b32.s1;
            mem[y + 3].b32.s1 = mem[p + 3].b32.s1;
            mem[y + 2].b32.s1 = mem[p + 2].b32.s1;
            if (((mem[q].b16.s0 == BOTTOM_ACC) || (mem[q].b16.s0 == (BOTTOM_ACC + 1)))) {
                if (mem[y + 3].b32.s1 < 0)
                    mem[y + 3].b32.s1 = 0;
            } else if (mem[y + 2].b32.s1 < 0)
                mem[y + 2].b32.s1 = 0;
            if ((((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
                  && (mem[p].b16.s0 == GLYPH_NODE)))) {
                sa = get_ot_math_accent_pos(f, mem[p + 4].b16.s1);
                if (sa == TEX_INFINITY)
                    sa = half(mem[y + 1].b32.s1);
            } else
                sa = half(mem[y + 1].b32.s1);
            if (((mem[q].b16.s0 == BOTTOM_ACC) || (mem[q].b16.s0 == (BOTTOM_ACC + 1))) || (s == TEX_INFINITY))
                s = half(w);
            mem[y + 4].b32.s1 = s - sa;
        } else
            mem[y + 4].b32.s1 = s + half(w - mem[y + 1].b32.s1);
        mem[y + 1].b32.s1 = 0;
        if (((mem[q].b16.s0 == BOTTOM_ACC) || (mem[q].b16.s0 == (BOTTOM_ACC + 1)))) {
            mem[x].b32.s1 = y;
            y = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
            mem[y + 4].b32.s1 = -(int32_t) (h - mem[y + 3].b32.s1);
        } else {

            p = new_kern(-(int32_t) delta);
            mem[p].b32.s1 = x;
            mem[y].b32.s1 = p;
            y = vpackage(y, 0, ADDITIONAL, MAX_HALFWORD);
            if (mem[y + 3].b32.s1 < h) {  /*765: */
                p = new_kern(h - mem[y + 3].b32.s1);
                mem[p].b32.s1 = mem[y + 5].b32.s1;
                mem[y + 5].b32.s1 = p;
                mem[y + 3].b32.s1 = h;
            }
        }
        mem[y + 1].b32.s1 = mem[x + 1].b32.s1;
        mem[q + 1].b32.s0 = y;
        mem[q + 1].b32.s1 = SUB_BOX;
    }

    free_ot_assembly(ot_assembly_ptr);
}


static void
make_fraction(int32_t q)
{
    int32_t p, v, x, y, z;
    scaled_t delta, delta1, delta2, shift_up, shift_down, clr;

    if (mem[q + 1].b32.s1 == DEFAULT_CODE)
        mem[q + 1].b32.s1 = default_rule_thickness();

    x = clean_box(q + 2, cur_style + 2 - 2 * (cur_style / 6));
    z = clean_box(q + 3, 2 * (cur_style / 2) + 3 - 2 * (cur_style / 6));

    if (mem[x + 1].b32.s1 < mem[z + 1].b32.s1)
        x = rebox(x, mem[z + 1].b32.s1);
    else
        z = rebox(z, mem[x + 1].b32.s1);

    if (cur_style < TEXT_STYLE) {
        shift_up = num1(cur_size);
        shift_down = denom1(cur_size);
    } else {
        shift_down = denom2(cur_size);
        if (mem[q + 1].b32.s1 != 0)
            shift_up = num2(cur_size);
        else
            shift_up = num3(cur_size);
    }

    if (mem[q + 1].b32.s1 == 0) { /*772:*/
        if (font_area[cur_f] == OTGR_FONT_FLAG && isOpenTypeMathFont(font_layout_engine[cur_f])) {
            if (cur_style < TEXT_STYLE)
                clr = get_ot_math_constant(cur_f, STACKDISPLAYSTYLEGAPMIN);
            else
                clr = get_ot_math_constant(cur_f, STACKGAPMIN);
        } else {
            if (cur_style < TEXT_STYLE)
                clr = 7 * default_rule_thickness();
            else
                clr = 3 * default_rule_thickness();
        }

        delta = half(clr - ((shift_up - mem[x + 2].b32.s1) - (mem[z + 3].b32.s1 - shift_down)));

        if (delta > 0) {
            shift_up = shift_up + delta;
            shift_down = shift_down + delta;
        }
    } else { /*773:*/
        if (font_area[cur_f] == OTGR_FONT_FLAG && isOpenTypeMathFont(font_layout_engine[cur_f])) {
            delta = half(mem[q + 1].b32.s1);

            if (cur_style < TEXT_STYLE)
                clr = get_ot_math_constant(cur_f, FRACTIONNUMDISPLAYSTYLEGAPMIN);
            else
                clr = get_ot_math_constant(cur_f, FRACTIONNUMERATORGAPMIN);

            delta1 = clr - ((shift_up - mem[x + 2].b32.s1) - (axis_height(cur_size) + delta));

            if (cur_style < TEXT_STYLE)
                clr = get_ot_math_constant(cur_f, FRACTIONDENOMDISPLAYSTYLEGAPMIN);
            else
                clr = get_ot_math_constant(cur_f, FRACTIONDENOMINATORGAPMIN);

            delta2 = clr - ((axis_height(cur_size) - delta) - (mem[z + 3].b32.s1 - shift_down));
        } else {
            if (cur_style < TEXT_STYLE)
                clr = 3 * mem[q + 1].b32.s1;
            else
                clr = mem[q + 1].b32.s1;
            delta = half(mem[q + 1].b32.s1);
            delta1 = clr - ((shift_up - mem[x + 2].b32.s1) - (axis_height(cur_size) + delta));
            delta2 = clr - ((axis_height(cur_size) - delta) - (mem[z + 3].b32.s1 - shift_down));
        }

        if (delta1 > 0)
            shift_up = shift_up + delta1;

        if (delta2 > 0)
            shift_down = shift_down + delta2;
    }

    v = new_null_box();
    NODE_type(v) = VLIST_NODE;
    mem[v + 3].b32.s1 = shift_up + mem[x + 3].b32.s1;
    mem[v + 2].b32.s1 = mem[z + 2].b32.s1 + shift_down;
    mem[v + 1].b32.s1 = mem[x + 1].b32.s1;

    if (mem[q + 1].b32.s1 == 0) {
        p = new_kern((shift_up - mem[x + 2].b32.s1) - (mem[z + 3].b32.s1 - shift_down));
        mem[p].b32.s1 = z;
    } else {
        y = fraction_rule(mem[q + 1].b32.s1);
        p = new_kern((axis_height(cur_size) - delta) - (mem[z + 3].b32.s1 - shift_down));
        mem[y].b32.s1 = p;
        mem[p].b32.s1 = z;
        p = new_kern((shift_up - mem[x + 2].b32.s1) - (axis_height(cur_size) + delta));
        mem[p].b32.s1 = y;
    }

    mem[x].b32.s1 = p;
    mem[v + 5].b32.s1 = x; /*:774*/

    if (cur_style < TEXT_STYLE)
        delta = delim1(cur_size);
    else
        delta = delim2(cur_size);

    x = var_delimiter(q + 4, cur_size, delta);
    mem[x].b32.s1 = v;
    z = var_delimiter(q + 5, cur_size, delta);
    mem[v].b32.s1 = z;
    mem[q + 1].b32.s1 = hpack(x, 0, ADDITIONAL); /*:775*/
}


static scaled_t
make_op(int32_t q)
{
    scaled_t delta;
    int32_t p, v, x, y, z;
    uint16_t c;
    b16x4 i;
    scaled_t shift_up, shift_down;
    scaled_t h1, h2;
    int32_t n, g;
    void *ot_assembly_ptr;
    internal_font_number save_f;
    if ((mem[q].b16.s0 == NORMAL) && (cur_style < TEXT_STYLE))
        mem[q].b16.s0 = LIMITS;
    delta = 0;
    ot_assembly_ptr = NULL;
    if (mem[q + 1].b32.s1 == MATH_CHAR) {
        fetch(q + 1);
        if (!((font_area[cur_f] == OTGR_FONT_FLAG) && (usingOpenType(font_layout_engine[cur_f])))) {
            if ((cur_style < TEXT_STYLE) && (((cur_i.s1) % 4) == LIST_TAG)) {
                c = cur_i.s0;
                i = FONT_CHARACTER_INFO(cur_f, c);
                if ((i.s3 > 0)) {
                    cur_c = c;
                    cur_i = i;
                    mem[q + 1].b16.s0 = c;
                }
            }
            delta = FONT_CHARINFO_ITALCORR(cur_f, cur_i);
        }
        x = clean_box(q + 1, cur_style);
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
            p = mem[x + 5].b32.s1;
            if ((((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
                  && (mem[p].b16.s0 == GLYPH_NODE)))) {
                if (cur_style < TEXT_STYLE) {
                    h1 = get_ot_math_constant(cur_f, DISPLAYOPERATORMINHEIGHT);
                    if (h1 < (mem[p + 3].b32.s1 + mem[p + 2].b32.s1) * 5 / ((double)4))
                        h1 = (mem[p + 3].b32.s1 + mem[p + 2].b32.s1) * 5 / ((double)4);
                    c = mem[p + 4].b16.s1;
                    n = 0;
                    do {
                        g = get_ot_math_variant(cur_f, c, n, &h2, 0);
                        if (h2 > 0) {
                            mem[p + 4].b16.s1 = g;
                            set_native_glyph_metrics(p, 1);
                        }
                        n++;
                    } while (!((h2 < 0) || (h2 >= h1)));
                    if ((h2 < 0)) {
                        ot_assembly_ptr = get_ot_assembly_ptr(cur_f, c, 0);
                        if (ot_assembly_ptr != NULL) {
                            free_node(p, GLYPH_NODE_SIZE);
                            p = build_opentype_assembly(cur_f, ot_assembly_ptr, h1, 0);
                            mem[x + 5].b32.s1 = p;
                            delta = 0;
                            goto found;
                        }
                    } else
                        set_native_glyph_metrics(p, 1);
                }
                delta = get_ot_math_ital_corr(cur_f, mem[p + 4].b16.s1);
            found:
                mem[x + 1].b32.s1 = mem[p + 1].b32.s1;
                mem[x + 3].b32.s1 = mem[p + 3].b32.s1;
                mem[x + 2].b32.s1 = mem[p + 2].b32.s1;
            }
        }
        if ((mem[q + 3].b32.s1 != EMPTY) && (mem[q].b16.s0 != LIMITS))
            mem[x + 1].b32.s1 = mem[x + 1].b32.s1 - delta;
        mem[x + 4].b32.s1 = half(mem[x + 3].b32.s1 - mem[x + 2].b32.s1) - axis_height(cur_size);
        mem[q + 1].b32.s1 = SUB_BOX;
        mem[q + 1].b32.s0 = x;
    }
    save_f = cur_f;
    if (mem[q].b16.s0 == LIMITS) {       /*777: */
        x = clean_box(q + 2, 2 * (cur_style / 4) + 4 + (cur_style % 2));
        y = clean_box(q + 1, cur_style);
        z = clean_box(q + 3, 2 * (cur_style / 4) + 5);
        v = new_null_box();
        NODE_type(v) = VLIST_NODE;
        mem[v + 1].b32.s1 = mem[y + 1].b32.s1;
        if (mem[x + 1].b32.s1 > mem[v + 1].b32.s1)
            mem[v + 1].b32.s1 = mem[x + 1].b32.s1;
        if (mem[z + 1].b32.s1 > mem[v + 1].b32.s1)
            mem[v + 1].b32.s1 = mem[z + 1].b32.s1;
        x = rebox(x, mem[v + 1].b32.s1);
        y = rebox(y, mem[v + 1].b32.s1);
        z = rebox(z, mem[v + 1].b32.s1);
        mem[x + 4].b32.s1 = half(delta);
        mem[z + 4].b32.s1 = -(int32_t) mem[x + 4].b32.s1;
        mem[v + 3].b32.s1 = mem[y + 3].b32.s1;
        mem[v + 2].b32.s1 = mem[y + 2].b32.s1;
        cur_f = save_f;
        if (mem[q + 2].b32.s1 == EMPTY) {
            free_node(x, BOX_NODE_SIZE);
            mem[v + 5].b32.s1 = y;
        } else {

            shift_up = big_op_spacing3() - mem[x + 2].b32.s1;
            if (shift_up < big_op_spacing1())
                shift_up = big_op_spacing1();
            p = new_kern(shift_up);
            mem[p].b32.s1 = y;
            mem[x].b32.s1 = p;
            p = new_kern(big_op_spacing5());
            mem[p].b32.s1 = x;
            mem[v + 5].b32.s1 = p;
            mem[v + 3].b32.s1 = mem[v + 3].b32.s1 + big_op_spacing5() + mem[x + 3].b32.s1 + mem[x + 2].b32.s1 + shift_up;
        }
        if (mem[q + 3].b32.s1 == EMPTY)
            free_node(z, BOX_NODE_SIZE);
        else {

            shift_down = big_op_spacing4() - mem[z + 3].b32.s1;
            if (shift_down < big_op_spacing2())
                shift_down = big_op_spacing2();
            p = new_kern(shift_down);
            mem[y].b32.s1 = p;
            mem[p].b32.s1 = z;
            p = new_kern(big_op_spacing5());
            mem[z].b32.s1 = p;
            mem[v + 2].b32.s1 = mem[v + 2].b32.s1 + big_op_spacing5() + mem[z + 3].b32.s1 + mem[z + 2].b32.s1 + shift_down;
        }
        mem[q + 1].b32.s1 = v;
    }
    free_ot_assembly(ot_assembly_ptr);
    return delta;
}


static void
make_ord(int32_t q)
{
    int32_t a;
    int32_t p, r;

restart:
    if (mem[q + 3].b32.s1 == EMPTY) {

        if (mem[q + 2].b32.s1 == EMPTY) {

            if (mem[q + 1].b32.s1 == MATH_CHAR) {
                p = mem[q].b32.s1;
                if (p != TEX_NULL) {

                    if ((mem[p].b16.s1 >= ORD_NOAD) && (mem[p].b16.s1 <= PUNCT_NOAD)) {

                        if (mem[p + 1].b32.s1 == MATH_CHAR) {

                            if ((mem[p + 1].b16.s1 % 256) == (mem[q + 1].b16.s1 % 256)) {
                                mem[q + 1].b32.s1 = MATH_TEXT_CHAR;
                                fetch(q + 1);
                                if (((cur_i.s1) % 4) == LIG_TAG) {
                                    a = lig_kern_base[cur_f] + cur_i.s0;
                                    cur_c = mem[p + 1].b16.s0;
                                    cur_i = font_info[a].b16;
                                    if (cur_i.s3 > 128) {
                                        a = lig_kern_base[cur_f] + 256 * cur_i.s1 + cur_i.s0 + 32768L - 256 * (128);
                                        cur_i = font_info[a].b16;
                                    }
                                    while (true) {

                                        if (cur_i.s2 == cur_c) {

                                            if (cur_i.s3 <= 128) {

                                                if (cur_i.s1 >= 128) {
                                                    p = new_kern(font_info
                                                                 [kern_base[cur_f] + 256 * cur_i.s1 + cur_i.s0].b32.s1);
                                                    mem[p].b32.s1 = mem[q].b32.s1;
                                                    mem[q].b32.s1 = p;
                                                    return;
                                                } else {
                                                    switch (cur_i.s1) {
                                                    case 1:
                                                    case 5:
                                                        mem[q + 1].b16.s0 = cur_i.s0;
                                                        break;
                                                    case 2:
                                                    case 6:
                                                        mem[p + 1].b16.s0 = cur_i.s0;
                                                        break;
                                                    case 3:
                                                    case 7:
                                                    case 11:
                                                        {
                                                            r = new_noad();
                                                            mem[r + 1].b16.s0 = cur_i.s0;
                                                            mem[r + 1].b16.s1 = (mem[q + 1].b16.s1 % 256);
                                                            mem[q].b32.s1 = r;
                                                            mem[r].b32.s1 = p;
                                                            if (cur_i.s1 < 11)
                                                                mem[r + 1].b32.s1 = MATH_CHAR;
                                                            else
                                                                mem[r + 1].b32.s1 = MATH_TEXT_CHAR;
                                                        }
                                                        break;
                                                    default:
                                                        {
                                                            mem[q].b32.s1 = mem[p].b32.s1;
                                                            mem[q + 1].b16.s0 = cur_i.s0;
                                                            mem[q + 3] = mem[p + 3];
                                                            mem[q + 2] = mem[p + 2];
                                                            free_node(p, NOAD_SIZE);
                                                        }
                                                        break;
                                                    }
                                                    if (cur_i.s1 > 3)
                                                        return;
                                                    mem[q + 1].b32.s1 = MATH_CHAR;
                                                    goto restart;
                                                }
                                            }
                                        }
                                        if (cur_i.s3 >= 128)
                                            return;
                                        a = a + cur_i.s3 + 1;
                                        cur_i = font_info[a].b16;
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


static int32_t
attach_hkern_to_new_hlist(int32_t q, scaled_t delta)
{
    int32_t y, z;
    z = new_kern(delta);
    if (mem[q + 1].b32.s1 == TEX_NULL)
        mem[q + 1].b32.s1 = z;
    else {

        y = mem[q + 1].b32.s1;
        while (mem[y].b32.s1 != TEX_NULL)
            y = LLIST_link(y);
        mem[y].b32.s1 = z;
    }
    return mem[q + 1].b32.s1;
}


static void
make_scripts(int32_t q, scaled_t delta)
{
    int32_t p, x, y, z;
    scaled_t shift_up, shift_down, clr, sub_kern, sup_kern;
    int32_t script_c;
    uint16_t script_g;
    internal_font_number script_f;
    int32_t t;
    internal_font_number save_f;

    p = mem[q + 1].b32.s1;
    script_c = TEX_NULL;
    script_g = 0;
    script_f = 0;
    sup_kern = 0;
    sub_kern = 0;
    if ((is_char_node(p))
        ||
        (((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
          && (mem[p].b16.s0 == GLYPH_NODE)))) {
        shift_up = 0;
        shift_down = 0;
    } else {

        z = hpack(p, 0, ADDITIONAL);
        if (cur_style < SCRIPT_STYLE)
            t = SCRIPT_SIZE;
        else
            t = SCRIPT_SCRIPT_SIZE;
        shift_up = mem[z + 3].b32.s1 - sup_drop(t);
        shift_down = mem[z + 2].b32.s1 + sub_drop(t);
        free_node(z, BOX_NODE_SIZE);
    }
    if (mem[q + 2].b32.s1 == EMPTY) {  /*784: */
        save_f = cur_f;
        x = clean_box(q + 3, 2 * (cur_style / 4) + 5);
        cur_f = save_f;
        mem[x + 1].b32.s1 = mem[x + 1].b32.s1 + DIMENPAR(script_space);
        if (shift_down < sub1(cur_size))
            shift_down = sub1(cur_size);
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
            clr = mem[x + 3].b32.s1 - get_ot_math_constant(cur_f, SUBSCRIPTTOPMAX);
        else
            clr = mem[x + 3].b32.s1 - (abs(math_x_height(cur_size) * 4) / 5);
        if (shift_down < clr)
            shift_down = clr;
        mem[x + 4].b32.s1 = shift_down;
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {   /*787: */
            if (mem[q + 3].b32.s1 == MATH_CHAR) {
                save_f = cur_f;
                fetch(q + 3);
                if (((font_area[cur_f] == OTGR_FONT_FLAG)
                     && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                    script_c = new_native_character(cur_f, cur_c);
                    script_g = get_native_glyph(script_c, 0);
                    script_f = cur_f;
                } else {

                    script_g = 0;
                    script_f = 0;
                }
                cur_f = save_f;
            }
            if ((((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
                  && (mem[p].b16.s0 == GLYPH_NODE))))
                sub_kern =
                    get_ot_math_kern(mem[p + 4].b16.s2, mem[p + 4].b16.s1, script_f, script_g, SUB_CMD,
                                     shift_down);
            if (sub_kern != 0)
                p = attach_hkern_to_new_hlist(q, sub_kern);
        }
    } else {

        {
            save_f = cur_f;
            x = clean_box(q + 2, 2 * (cur_style / 4) + 4 + (cur_style % 2));
            cur_f = save_f;
            mem[x + 1].b32.s1 = mem[x + 1].b32.s1 + DIMENPAR(script_space);
            if (odd(cur_style))
                clr = sup3(cur_size);
            else if (cur_style < TEXT_STYLE)
                clr = sup1(cur_size);
            else
                clr = sup2(cur_size);
            if (shift_up < clr)
                shift_up = clr;
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
                clr = mem[x + 2].b32.s1 + get_ot_math_constant(cur_f, SUPERSCRIPTBOTTOMMIN);
            else
                clr = mem[x + 2].b32.s1 + (abs(math_x_height(cur_size)) / 4);
            if (shift_up < clr)
                shift_up = clr;
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {       /*788: */
                if (mem[q + 2].b32.s1 == MATH_CHAR) {
                    save_f = cur_f;
                    fetch(q + 2);
                    if (((font_area[cur_f] == OTGR_FONT_FLAG)
                         && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                        script_c = new_native_character(cur_f, cur_c);
                        script_g = get_native_glyph(script_c, 0);
                        script_f = cur_f;
                    } else {

                        script_g = 0;
                        script_f = 0;
                    }
                    cur_f = save_f;
                }
                if ((((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
                      && (mem[p].b16.s0 == GLYPH_NODE))))
                    sup_kern =
                        get_ot_math_kern(mem[p + 4].b16.s2, mem[p + 4].b16.s1, script_f, script_g, SUP_CMD,
                                         shift_up);
                if ((sup_kern != 0) && (mem[q + 3].b32.s1 == EMPTY))
                    p = attach_hkern_to_new_hlist(q, sup_kern);
            }
        }
        if (mem[q + 3].b32.s1 == EMPTY)
            mem[x + 4].b32.s1 = -(int32_t) shift_up;
        else {                  /*786: */

            save_f = cur_f;
            y = clean_box(q + 3, 2 * (cur_style / 4) + 5);
            cur_f = save_f;
            mem[y + 1].b32.s1 = mem[y + 1].b32.s1 + DIMENPAR(script_space);
            if (shift_down < sub2(cur_size))
                shift_down = sub2(cur_size);
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
                clr =
                    get_ot_math_constant(cur_f,
                                         SUBSUPERSCRIPTGAPMIN) - ((shift_up - mem[x + 2].b32.s1) -
                                                                           (mem[y + 3].b32.s1 - shift_down));
            else
                clr = 4 * default_rule_thickness() - ((shift_up - mem[x + 2].b32.s1) - (mem[y + 3].b32.s1 - shift_down));
            if (clr > 0) {
                shift_down = shift_down + clr;
                if (((font_area[cur_f] == OTGR_FONT_FLAG)
                     && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
                    clr =
                        get_ot_math_constant(cur_f,
                                             SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT) - (shift_up - mem[x + 2].b32.s1);
                else
                    clr = (abs(math_x_height(cur_size) * 4) / 5) - (shift_up - mem[x + 2].b32.s1);
                if (clr > 0) {
                    shift_up = shift_up + clr;
                    shift_down = shift_down - clr;
                }
            }
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                {
                    if (mem[q + 3].b32.s1 == MATH_CHAR) {
                        save_f = cur_f;
                        fetch(q + 3);
                        if (((font_area[cur_f] == OTGR_FONT_FLAG)
                             && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                            script_c = new_native_character(cur_f, cur_c);
                            script_g = get_native_glyph(script_c, 0);
                            script_f = cur_f;
                        } else {

                            script_g = 0;
                            script_f = 0;
                        }
                        cur_f = save_f;
                    }
                    if ((((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
                          && (mem[p].b16.s0 == GLYPH_NODE))))
                        sub_kern =
                            get_ot_math_kern(mem[p + 4].b16.s2, mem[p + 4].b16.s1, script_f, script_g,
                                             SUB_CMD, shift_down);
                    if (sub_kern != 0)
                        p = attach_hkern_to_new_hlist(q, sub_kern);
                }
                {
                    if (mem[q + 2].b32.s1 == MATH_CHAR) {
                        save_f = cur_f;
                        fetch(q + 2);
                        if (((font_area[cur_f] == OTGR_FONT_FLAG)
                             && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                            script_c = new_native_character(cur_f, cur_c);
                            script_g = get_native_glyph(script_c, 0);
                            script_f = cur_f;
                        } else {

                            script_g = 0;
                            script_f = 0;
                        }
                        cur_f = save_f;
                    }
                    if ((((p) != TEX_NULL && (!(is_char_node(p))) && (NODE_type(p) == WHATSIT_NODE)
                          && (mem[p].b16.s0 == GLYPH_NODE))))
                        sup_kern =
                            get_ot_math_kern(mem[p + 4].b16.s2, mem[p + 4].b16.s1, script_f, script_g,
                                             SUP_CMD, shift_up);
                    if ((sup_kern != 0) && (mem[q + 3].b32.s1 == EMPTY))
                        p = attach_hkern_to_new_hlist(q, sup_kern);
                }
            }
            mem[x + 4].b32.s1 = sup_kern + delta - sub_kern;
            p = new_kern((shift_up - mem[x + 2].b32.s1) - (mem[y + 3].b32.s1 - shift_down));
            mem[x].b32.s1 = p;
            mem[p].b32.s1 = y;
            x = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
            mem[x + 4].b32.s1 = shift_down;
        }
    }
    if (mem[q + 1].b32.s1 == TEX_NULL)
        mem[q + 1].b32.s1 = x;
    else {

        p = mem[q + 1].b32.s1;
        while (mem[p].b32.s1 != TEX_NULL)
            p = LLIST_link(p);
        mem[p].b32.s1 = x;
    }
}


static small_number
make_left_right(int32_t q, small_number style, scaled_t max_d, scaled_t max_h)
{
    scaled_t delta, delta1, delta2;

    cur_style = style;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
    delta2 = max_d + axis_height(cur_size);
    delta1 = max_h + max_d - delta2;
    if (delta2 > delta1)
        delta1 = delta2;
    delta = (delta1 / 500) * INTPAR(delimiter_factor);
    delta2 = delta1 + delta1 - DIMENPAR(delimiter_shortfall);
    if (delta < delta2)
        delta = delta2;
    mem[q + 1].b32.s1 = var_delimiter(q + 1, cur_size, delta);
    return mem[q].b16.s1 - ((LEFT_NOAD - 20));
}


static void
mlist_to_hlist(void)
{
    int32_t mlist;
    bool penalties;
    small_number style;
    small_number save_style;
    int32_t q;
    int32_t r;
    small_number r_type;
    small_number t;
    int32_t p = TEX_NULL, x, y, z;
    int32_t pen;
    small_number s;
    scaled_t max_h, max_d;
    scaled_t delta;

    mlist = cur_mlist;
    penalties = mlist_penalties;
    style = cur_style;
    q = mlist;
    r = TEX_NULL;
    r_type = OP_NOAD;
    max_h = 0;
    max_d = 0;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
    while (q != TEX_NULL) {  /*753: */

    reswitch:
        delta = 0;
        switch (mem[q].b16.s1) {
        case BIN_NOAD:
            switch (r_type) {
            case BIN_NOAD:
            case OP_NOAD:
            case REL_NOAD:
            case OPEN_NOAD:
            case PUNCT_NOAD:
            case LEFT_NOAD:
                mem[q].b16.s1 = ORD_NOAD;
                goto reswitch;
            default:
                break;
            }
            break;
        case REL_NOAD:
        case CLOSE_NOAD:
        case PUNCT_NOAD:
        case RIGHT_NOAD:
            {
                if (r_type == BIN_NOAD)
                    mem[r].b16.s1 = ORD_NOAD /*:755 */ ;
                if (mem[q].b16.s1 == RIGHT_NOAD)
                    goto lab80;
            }
            break;
        case LEFT_NOAD:
            goto lab80;
            break;
        case FRACTION_NOAD:
            make_fraction(q);
            goto lab82;
        case OP_NOAD:
            delta = make_op(q);
            if (mem[q].b16.s0 == LIMITS)
                goto lab82;
            break;
        case ORD_NOAD:
            make_ord(q);
            break;
        case OPEN_NOAD:
        case INNER_NOAD:
            break;
        case RADICAL_NOAD:
            make_radical(q);
            break;
        case OVER_NOAD:
            make_over(q);
            break;
        case UNDER_NOAD:
            make_under(q);
            break;
        case ACCENT_NOAD:
            make_math_accent(q);
            break;
        case VCENTER_NOAD:
            make_vcenter(q);
            break;
        case STYLE_NODE:
            {
                cur_style = mem[q].b16.s0;
                {
                    if (cur_style < SCRIPT_STYLE)
                        cur_size = TEXT_SIZE;
                    else
                        cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }
                goto lab81;
            }
            break;
        case CHOICE_NODE:
            {
                switch (cur_style / 2) {
                case 0:
                    {
                        p = mem[q + 1].b32.s0;
                        mem[q + 1].b32.s0 = TEX_NULL;
                    }
                    break;
                case 1:
                    {
                        p = mem[q + 1].b32.s1;
                        mem[q + 1].b32.s1 = TEX_NULL;
                    }
                    break;
                case 2:
                    {
                        p = mem[q + 2].b32.s0;
                        mem[q + 2].b32.s0 = TEX_NULL;
                    }
                    break;
                case 3:
                    {
                        p = mem[q + 2].b32.s1;
                        mem[q + 2].b32.s1 = TEX_NULL;
                    }
                    break;
                }
                flush_node_list(mem[q + 1].b32.s0);
                flush_node_list(mem[q + 1].b32.s1);
                flush_node_list(mem[q + 2].b32.s0);
                flush_node_list(mem[q + 2].b32.s1);
                NODE_type(q) = STYLE_NODE;
                mem[q].b16.s0 = cur_style;
                mem[q + 1].b32.s1 = 0;
                mem[q + 2].b32.s1 = 0;
                if (p != TEX_NULL) {
                    z = mem[q].b32.s1;
                    mem[q].b32.s1 = p;
                    while (mem[p].b32.s1 != TEX_NULL)
                        p = LLIST_link(p);
                    mem[p].b32.s1 = z;
                }
                goto lab81;
            }
            break;
        case INS_NODE:
        case MARK_NODE:
        case ADJUST_NODE:
        case WHATSIT_NODE:
        case PENALTY_NODE:
        case DISC_NODE:
            goto lab81;
        case RULE_NODE:
            if (mem[q + 3].b32.s1 > max_h)
                max_h = mem[q + 3].b32.s1;
            if (mem[q + 2].b32.s1 > max_d)
                max_d = mem[q + 2].b32.s1;
            goto lab81;
        case GLUE_NODE:
            {
                if (mem[q].b16.s0 == MU_GLUE) {
                    x = mem[q + 1].b32.s0;
                    y = math_glue(x, cur_mu);
                    delete_glue_ref(x);
                    mem[q + 1].b32.s0 = y;
                    mem[q].b16.s0 = NORMAL;
                } else if ((cur_size != TEXT_SIZE) && (mem[q].b16.s0 == COND_MATH_GLUE)) {
                    p = mem[q].b32.s1;
                    if (p != TEX_NULL) {

                        if ((NODE_type(p) == GLUE_NODE) || (NODE_type(p) == KERN_NODE)) {
                            mem[q].b32.s1 = mem[p].b32.s1;
                            mem[p].b32.s1 = TEX_NULL;
                            flush_node_list(p);
                        }
                    }
                }
                goto lab81;
            }
            break;
        case KERN_NODE:
            {
                math_kern(q, cur_mu);
                goto lab81;
            }
            break;
        default:
            confusion("mlist1");
            break;
        }
        switch (mem[q + 1].b32.s1) {
        case 1:
        case 4:
            {
                fetch(q + 1);
                if (((font_area[cur_f] == AAT_FONT_FLAG)
                     || (font_area[cur_f] == OTGR_FONT_FLAG))) {
                    z = new_native_character(cur_f, cur_c);
                    p = get_node(GLYPH_NODE_SIZE);
                    NODE_type(p) = WHATSIT_NODE;
                    mem[p].b16.s0 = GLYPH_NODE;
                    mem[p + 4].b16.s2 = cur_f;
                    mem[p + 4].b16.s1 = get_native_glyph(z, 0);
                    set_native_glyph_metrics(p, 1);
                    free_node(z, mem[z + 4].b16.s3);
                    delta = get_ot_math_ital_corr(cur_f, mem[p + 4].b16.s1);
                    if ((mem[q + 1].b32.s1 == MATH_TEXT_CHAR)
                        &&
                        (!((font_area[cur_f] == OTGR_FONT_FLAG)
                           && (isOpenTypeMathFont(font_layout_engine[cur_f]))) != 0))
                        delta = 0;
                    if ((mem[q + 3].b32.s1 == EMPTY) && (delta != 0)) {
                        mem[p].b32.s1 = new_kern(delta);
                        delta = 0;
                    }
                } else if ((cur_i.s3 > 0)) {
                    delta = FONT_CHARINFO_ITALCORR(cur_f, cur_i);
                    p = new_character(cur_f, cur_c);
                    if ((mem[q + 1].b32.s1 == MATH_TEXT_CHAR)
                        && (font_info[SPACE_CODE + param_base[cur_f]].b32.s1 != 0))
                        delta = 0;
                    if ((mem[q + 3].b32.s1 == EMPTY) && (delta != 0)) {
                        mem[p].b32.s1 = new_kern(delta);
                        delta = 0;
                    }
                } else
                    p = TEX_NULL;
            }
            break;
        case 0:
            p = TEX_NULL;
            break;
        case 2:
            p = mem[q + 1].b32.s0;
            break;
        case 3:
            {
                cur_mlist = mem[q + 1].b32.s0;
                save_style = cur_style;
                mlist_penalties = false;
                mlist_to_hlist();
                cur_style = save_style;
                {
                    if (cur_style < SCRIPT_STYLE)
                        cur_size = TEXT_SIZE;
                    else
                        cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }
                p = hpack(mem[TEMP_HEAD].b32.s1, 0, ADDITIONAL);
            }
            break;
        default:
            confusion("mlist2");
            break;
        }
        mem[q + 1].b32.s1 = p;
        if ((mem[q + 3].b32.s1 == EMPTY) && (mem[q + 2].b32.s1 == EMPTY))
            goto lab82;
        make_scripts(q, delta);
 lab82:/*check_dimensions */ z = hpack(mem[q + 1].b32.s1, 0, ADDITIONAL);
        if (mem[z + 3].b32.s1 > max_h)
            max_h = mem[z + 3].b32.s1;
        if (mem[z + 2].b32.s1 > max_d)
            max_d = mem[z + 2].b32.s1;
        free_node(z, BOX_NODE_SIZE);
 lab80:                        /*done_with_noad */ r = q;
        r_type = mem[r].b16.s1;
        if (r_type == RIGHT_NOAD) {
            r_type = LEFT_NOAD;
            cur_style = style;
            {
                if (cur_style < SCRIPT_STYLE)
                    cur_size = TEXT_SIZE;
                else
                    cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                cur_mu = x_over_n(math_quad(cur_size), 18);
            }
        }
 lab81:                        /*done_with_node */ q = LLIST_link(q);
    }
    if (r_type == BIN_NOAD)
        mem[r].b16.s1 = 16 /*ord_noad *//*:755 */ ;
    p = TEMP_HEAD;
    mem[p].b32.s1 = TEX_NULL;
    q = mlist;
    r_type = 0;
    cur_style = style;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
    while (q != TEX_NULL) {

        t = ORD_NOAD;
        s = NOAD_SIZE;
        pen = INF_PENALTY;
        switch (mem[q].b16.s1) {
        case OP_NOAD:
        case OPEN_NOAD:
        case CLOSE_NOAD:
        case PUNCT_NOAD:
        case INNER_NOAD:
            t = mem[q].b16.s1;
            break;
        case BIN_NOAD:
            {
                t = BIN_NOAD;
                pen = INTPAR(bin_op_penalty);
            }
            break;
        case REL_NOAD:
            {
                t = REL_NOAD;
                pen = INTPAR(rel_penalty);
            }
            break;
        case ORD_NOAD:
        case VCENTER_NOAD:
        case OVER_NOAD:
        case UNDER_NOAD:
            break;
        case RADICAL_NOAD:
            s = RADICAL_NOAD_SIZE;
            break;
        case ACCENT_NOAD:
            s = ACCENT_NOAD_SIZE;
            break;
        case FRACTION_NOAD:
            t = INNER_NOAD;
            s = FRACTION_NOAD_SIZE;
            break;
        case LEFT_NOAD:
        case RIGHT_NOAD:
            t = make_left_right(q, style, max_d, max_h);
            break;
        case STYLE_NODE:
            {
                cur_style = mem[q].b16.s0;
                s = STYLE_NODE_SIZE;
                {
                    if (cur_style < SCRIPT_STYLE)
                        cur_size = TEXT_SIZE;
                    else
                        cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }
                goto lab83;
            }
            break;
        case WHATSIT_NODE:
        case PENALTY_NODE:
        case RULE_NODE:
        case DISC_NODE:
        case ADJUST_NODE:
        case INS_NODE:
        case MARK_NODE:
        case GLUE_NODE:
        case KERN_NODE:
            {
                mem[p].b32.s1 = q;
                p = q;
                q = LLIST_link(q);
                mem[p].b32.s1 = TEX_NULL;
                goto done;
            }
            break;
        default:
            confusion("mlist3");
            break;
        }
        if (r_type > 0) {
            const char* offset_table[] = {
                "02340001",
                "22*40001",
                "33**3**3",
                "44*04004",
                "00*00000",
                "02340001",
                "11*11111",
                "12341011"
            };
            // The inter-element spacing in math formulas depends on a 8x8 table.
            // The table indices range from ORD_NOAD to INNER_NOAD.
            // The chars of this table have the following significance:
            switch (offset_table[r_type - ORD_NOAD][t - ORD_NOAD]) {
            case '0': // no space
                x = 0;
                break;
            case '1': // a conditional thin space
                if (cur_style < SCRIPT_STYLE)
                    x = GLUE_PAR__thin_mu_skip;
                else
                    x = 0;
                break;
            case '2': // a thin space
                x = GLUE_PAR__thin_mu_skip;
                break;
            case '3': // a conditional medium space
                if (cur_style < SCRIPT_STYLE)
                    x = GLUE_PAR__med_mu_skip;
                else
                    x = 0;
                break;
            case '4': // a conditional thick space
                if (cur_style < SCRIPT_STYLE)
                    x = GLUE_PAR__thick_mu_skip;
                else
                    x = 0;
                break;
            default: // impossible
                confusion("mlist4");
                break;
            }
            if (x != 0) {
                y = math_glue(eqtb[GLUE_BASE + x].b32.s1, cur_mu);
                z = new_glue(y);
                mem[y].b32.s1 = TEX_NULL;
                mem[p].b32.s1 = z;
                p = z;
                mem[z].b16.s0 = x + 1;
            }
        }
        if (mem[q + 1].b32.s1 != TEX_NULL) {
            mem[p].b32.s1 = mem[q + 1].b32.s1;
            do {
                p = LLIST_link(p);
            } while (!(mem[p].b32.s1 == TEX_NULL));
        }
        if (penalties) {

            if (mem[q].b32.s1 != TEX_NULL) {

                if (pen < INF_PENALTY) {
                    r_type = mem[mem[q].b32.s1].b16.s1;
                    if (r_type != PENALTY_NODE) {

                        if (r_type != REL_NOAD) {
                            z = new_penalty(pen);
                            mem[p].b32.s1 = z;
                            p = z;
                        }
                    }
                }
            }
        }
        if (mem[q].b16.s1 == RIGHT_NOAD)
            t = OPEN_NOAD;
        r_type = t;
 lab83:                        /*delete_q */ r = q;
        q = LLIST_link(q);
        free_node(r, s);
    done:
        ;
    }
}


static int32_t
var_delimiter(int32_t d, int32_t s, scaled_t v)
{
    int32_t b;
    void *ot_assembly_ptr;
    internal_font_number f, g;
    uint16_t c, x, y;
    int32_t m, n;
    scaled_t u;
    scaled_t w;
    b16x4 q = { 0, 0, 0, 0 };
    b16x4 r;
    int32_t z;
    bool large_attempt;

    f = FONT_BASE;
    w = 0;
    large_attempt = false;
    z = (mem[d].b16.s3 % 256);
    x = (mem[d].b16.s2 + (mem[d].b16.s3 / 256) * 65536L);
    ot_assembly_ptr = NULL;
    while (true) {

        if ((z != 0) || (x != 0)) {
            z = z + s + 256;
            do {
                z = z - 256;
                g = MATH_FONT(z);
                if (g != FONT_BASE) {   /*734: */

                    if (((font_area[g] == OTGR_FONT_FLAG) && (usingOpenType(font_layout_engine[g])))) {
                        x = map_char_to_glyph(g, x);
                        f = g;
                        c = x;
                        w = 0;
                        n = 0;
                        do {
                            y = get_ot_math_variant(g, x, n, &u, 0);
                            if (u > w) {
                                c = y;
                                w = u;
                                if (u >= v)
                                    goto found;
                            }
                            n = n + 1;
                        } while (!(u < 0));
                        ot_assembly_ptr = get_ot_assembly_ptr(g, x, 0);
                        if (ot_assembly_ptr != NULL)
                            goto found;
                    } else {

                        y = x;
                        if ((y >= font_bc[g]) && (y <= font_ec[g])) {
                        continue_:
                            q = FONT_CHARACTER_INFO(g, y);
                            if ((q.s3 > 0)) {
                                if (((q.s1) % 4) == EXT_TAG) {
                                    f = g;
                                    c = y;
                                    goto found;
                                }
                                u = FONT_CHARINFO_HEIGHT(g, q) + FONT_CHARINFO_DEPTH(g, q);
                                if (u > w) {
                                    f = g;
                                    c = y;
                                    w = u;
                                    if (u >= v)
                                        goto found;
                                }
                                if (((q.s1) % 4) == LIST_TAG) {
                                    y = q.s0;
                                    goto continue_;
                                }
                            }
                        }
                    }
                }
            } while (!(z < SCRIPT_SIZE));
        }
        if (large_attempt)
            goto found;
        large_attempt = true;
        z = (mem[d].b16.s1 % 256);
        x = (mem[d].b16.s0 + (mem[d].b16.s1 / 256) * 65536L);
    }
 found:
    if (f != FONT_BASE) {
        if (!((font_area[f] == OTGR_FONT_FLAG) && (usingOpenType(font_layout_engine[f])))) {       /*736: */

            if (((q.s1) % 4) == EXT_TAG) {      /*739: */
                b = new_null_box();
                NODE_type(b) = VLIST_NODE;
                r = font_info[exten_base[f] + q.s0].b16;
                c = r.s0;
                u = height_plus_depth(f, c);
                w = 0;
                q = FONT_CHARACTER_INFO(f, effective_char(true, f, c));
                mem[b + 1].b32.s1 = FONT_CHARINFO_WIDTH(f, q) + FONT_CHARINFO_ITALCORR(f, q);
                c = r.s1;
                if (c != 0)
                    w = w + height_plus_depth(f, c);
                c = r.s2;
                if (c != 0)
                    w = w + height_plus_depth(f, c);
                c = r.s3;
                if (c != 0)
                    w = w + height_plus_depth(f, c);
                n = 0;
                if (u > 0)
                    while (w < v) {

                        w = w + u;
                        n++;
                        if (r.s2 != 0)
                            w = w + u;
                    }
                c = r.s1;
                if (c != 0)
                    stack_into_box(b, f, c);
                c = r.s0;
                {
                    register int32_t for_end;
                    m = 1;
                    for_end = n;
                    if (m <= for_end)
                        do
                            stack_into_box(b, f, c);
                        while (m++ < for_end);
                }
                c = r.s2;
                if (c != 0) {
                    stack_into_box(b, f, c);
                    c = r.s0;
                    {
                        register int32_t for_end;
                        m = 1;
                        for_end = n;
                        if (m <= for_end)
                            do
                                stack_into_box(b, f, c);
                            while (m++ < for_end);
                    }
                }
                c = r.s3;
                if (c != 0)
                    stack_into_box(b, f, c);
                mem[b + 2].b32.s1 = w - mem[b + 3].b32.s1;
            } else
                b = char_box(f, c) /*:736 */ ;
        } else {

            if (ot_assembly_ptr != NULL)
                b = build_opentype_assembly(f, ot_assembly_ptr, v, 0);
            else {

                b = new_null_box();
                NODE_type(b) = VLIST_NODE;
                mem[b + 5].b32.s1 = get_node(GLYPH_NODE_SIZE);
                NODE_type(mem[b + 5].b32.s1) = WHATSIT_NODE;
                mem[mem[b + 5].b32.s1].b16.s0 = GLYPH_NODE;
                mem[mem[b + 5].b32.s1 + 4].b16.s2 = f;
                mem[mem[b + 5].b32.s1 + 4].b16.s1 = c;
                set_native_glyph_metrics(mem[b + 5].b32.s1, 1);
                mem[b + 1].b32.s1 = mem[mem[b + 5].b32.s1 + 1].b32.s1;
                mem[b + 3].b32.s1 = mem[mem[b + 5].b32.s1 + 3].b32.s1;
                mem[b + 2].b32.s1 = mem[mem[b + 5].b32.s1 + 2].b32.s1;
            }
        }
    } else {

        b = new_null_box();
        mem[b + 1].b32.s1 = DIMENPAR(null_delimiter_space);
    }
    mem[b + 4].b32.s1 = half(mem[b + 3].b32.s1 - mem[b + 2].b32.s1) - axis_height(s);
    free_ot_assembly(ot_assembly_ptr);
    return b;
}


static int32_t
char_box(internal_font_number f, int32_t c)
{
    b16x4 q;
    int32_t b, p;
    if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
        b = new_null_box();
        p = new_native_character(f, c);
        mem[b + 5].b32.s1 = p;
        mem[b + 3].b32.s1 = mem[p + 3].b32.s1;
        mem[b + 1].b32.s1 = mem[p + 1].b32.s1;
        if (mem[p + 2].b32.s1 < 0)
            mem[b + 2].b32.s1 = 0;
        else
            mem[b + 2].b32.s1 = mem[p + 2].b32.s1;
    } else {

        q = FONT_CHARACTER_INFO(f, effective_char(true, f, c));
        b = new_null_box();
        mem[b + 1].b32.s1 = FONT_CHARINFO_WIDTH(f, q) + FONT_CHARINFO_ITALCORR(f, q);
        mem[b + 3].b32.s1 = FONT_CHARINFO_HEIGHT(f, q);
        mem[b + 2].b32.s1 = FONT_CHARINFO_DEPTH(f, q);
        p = get_avail();
        mem[p].b16.s0 = c;
        mem[p].b16.s1 = f;
    }
    mem[b + 5].b32.s1 = p;
    return b;
}


static void
stack_into_box(int32_t b, internal_font_number f, uint16_t c)
{
    int32_t p;
    p = char_box(f, c);
    mem[p].b32.s1 = mem[b + 5].b32.s1;
    mem[b + 5].b32.s1 = p;
    mem[b + 3].b32.s1 = mem[p + 3].b32.s1;
}


static scaled_t
height_plus_depth(internal_font_number f, uint16_t c)
{
    b16x4 q = FONT_CHARACTER_INFO(f, effective_char(true, f, c));
    return FONT_CHARINFO_HEIGHT(f, q) + FONT_CHARINFO_DEPTH(f, q);
}


static void
stack_glyph_into_box(int32_t b, internal_font_number f, int32_t g)
{
    int32_t p, q;
    p = get_node(GLYPH_NODE_SIZE);
    NODE_type(p) = WHATSIT_NODE;
    mem[p].b16.s0 = GLYPH_NODE;
    mem[p + 4].b16.s2 = f;
    mem[p + 4].b16.s1 = g;
    set_native_glyph_metrics(p, 1);
    if (NODE_type(b) == HLIST_NODE) {
        q = mem[b + 5].b32.s1;
        if (q == TEX_NULL)
            mem[b + 5].b32.s1 = p;
        else {

            while (mem[q].b32.s1 != TEX_NULL)
                q = LLIST_link(q);
            mem[q].b32.s1 = p;
            if ((mem[b + 3].b32.s1 < mem[p + 3].b32.s1))
                mem[b + 3].b32.s1 = mem[p + 3].b32.s1;
            if ((mem[b + 2].b32.s1 < mem[p + 2].b32.s1))
                mem[b + 2].b32.s1 = mem[p + 2].b32.s1;
        }
    } else {

        mem[p].b32.s1 = mem[b + 5].b32.s1;
        mem[b + 5].b32.s1 = p;
        mem[b + 3].b32.s1 = mem[p + 3].b32.s1;
        if ((mem[b + 1].b32.s1 < mem[p + 1].b32.s1))
            mem[b + 1].b32.s1 = mem[p + 1].b32.s1;
    }
}


static void
stack_glue_into_box(int32_t b, scaled_t min, scaled_t max)
{
    int32_t p, q;
    q = new_spec(0);
    mem[q + 1].b32.s1 = min;
    mem[q + 2].b32.s1 = max - min;
    p = new_glue(q);
    if (NODE_type(b) == HLIST_NODE) {
        q = mem[b + 5].b32.s1;
        if (q == TEX_NULL)
            mem[b + 5].b32.s1 = p;
        else {

            while (mem[q].b32.s1 != TEX_NULL)
                q = LLIST_link(q);
            mem[q].b32.s1 = p;
        }
    } else {

        mem[p].b32.s1 = mem[b + 5].b32.s1;
        mem[b + 5].b32.s1 = p;
        mem[b + 3].b32.s1 = mem[p + 3].b32.s1;
        mem[b + 1].b32.s1 = mem[p + 1].b32.s1;
    }
}


static int32_t
build_opentype_assembly(internal_font_number f, void *a, scaled_t s, bool horiz)
{
    int32_t b;
    int32_t n;
    int32_t i, j;
    int32_t g;
    int32_t p;
    scaled_t s_max, o, oo, prev_o, min_o;
    bool no_extenders;
    scaled_t nat, str;
    b = new_null_box();
    if (horiz)
        NODE_type(b) = HLIST_NODE;
        else
            NODE_type(b) = VLIST_NODE;
    n = -1;
    no_extenders = true;
    min_o = ot_min_connector_overlap(f);
    do {
        n = n + 1;
        s_max = 0;
        prev_o = 0;
        {
            register int32_t for_end;
            i = 0;
            for_end = ot_part_count(a) - 1;
            if (i <= for_end)
                do {
                    if (ot_part_is_extender(a, i)) {
                        no_extenders = false;
                        {
                            register int32_t for_end;
                            j = 1;
                            for_end = n;
                            if (j <= for_end)
                                do {
                                    o = ot_part_start_connector(f, a, i);
                                    if (min_o < o)
                                        o = min_o;
                                    if (prev_o < o)
                                        o = prev_o;
                                    s_max = s_max - o + ot_part_full_advance(f, a, i);
                                    prev_o = ot_part_end_connector(f, a, i);
                                }
                                while (j++ < for_end);
                        }
                    } else {

                        o = ot_part_start_connector(f, a, i);
                        if (min_o < o)
                            o = min_o;
                        if (prev_o < o)
                            o = prev_o;
                        s_max = s_max - o + ot_part_full_advance(f, a, i);
                        prev_o = ot_part_end_connector(f, a, i);
                    }
                }
                while (i++ < for_end);
        }
    } while (!((s_max >= s) || no_extenders));
    prev_o = 0;
    {
        register int32_t for_end;
        i = 0;
        for_end = ot_part_count(a) - 1;
        if (i <= for_end)
            do {
                if (ot_part_is_extender(a, i)) {
                    {
                        register int32_t for_end;
                        j = 1;
                        for_end = n;
                        if (j <= for_end)
                            do {
                                o = ot_part_start_connector(f, a, i);
                                if (prev_o < o)
                                    o = prev_o;
                                oo = o;
                                if (min_o < o)
                                    o = min_o;
                                if (oo > 0)
                                    stack_glue_into_box(b, -(int32_t) oo, -(int32_t) o);
                                g = ot_part_glyph(a, i);
                                stack_glyph_into_box(b, f, g);
                                prev_o = ot_part_end_connector(f, a, i);
                            }
                            while (j++ < for_end);
                    }
                } else {

                    o = ot_part_start_connector(f, a, i);
                    if (prev_o < o)
                        o = prev_o;
                    oo = o;
                    if (min_o < o)
                        o = min_o;
                    if (oo > 0)
                        stack_glue_into_box(b, -(int32_t) oo, -(int32_t) o);
                    g = ot_part_glyph(a, i);
                    stack_glyph_into_box(b, f, g);
                    prev_o = ot_part_end_connector(f, a, i);
                }
            }
            while (i++ < for_end);
    }
    p = mem[b + 5].b32.s1;
    nat = 0;
    str = 0;
    while (p != TEX_NULL) {

        if (NODE_type(p) == WHATSIT_NODE) {
            if (horiz)
                nat = nat + mem[p + 1].b32.s1;
            else
                nat = nat + mem[p + 3].b32.s1 + mem[p + 2].b32.s1;
        } else if (NODE_type(p) == GLUE_NODE) {
            nat = nat + mem[mem[p + 1].b32.s0 + 1].b32.s1;
            str = str + mem[mem[p + 1].b32.s0 + 2].b32.s1;
        }
        p = LLIST_link(p);
    }
    o = 0;
    if ((s > nat) && (str > 0)) {
        o = (s - nat);
        if ((o > str))
            o = str;
        mem[b + 5].b16.s0 = NORMAL;
        mem[b + 5].b16.s1 = STRETCHING;
        BOX_glue_set(b) = o / ((double)str);
        if (horiz)
            mem[b + 1].b32.s1 = nat + tex_round(str * BOX_glue_set(b));
        else
            mem[b + 3].b32.s1 = nat + tex_round(str * BOX_glue_set(b));
    } else if (horiz)
        mem[b + 1].b32.s1 = nat;
    else
        mem[b + 3].b32.s1 = nat;
    return b;
}


static int32_t
rebox(int32_t b, scaled_t w)
{
    int32_t p;
    internal_font_number f;
    scaled_t v;
    if ((mem[b + 1].b32.s1 != w) && (mem[b + 5].b32.s1 != TEX_NULL)) {
        if (NODE_type(b) == VLIST_NODE)
            b = hpack(b, 0, ADDITIONAL);
        p = mem[b + 5].b32.s1;
        if (((is_char_node(p))) && (mem[p].b32.s1 == TEX_NULL)) {
            f = CHAR_NODE_font(p);
            v = FONT_CHARACTER_WIDTH(f,
                                     effective_char(true, f, CHAR_NODE_character(p)));
            if (v != mem[b + 1].b32.s1)
                mem[p].b32.s1 = new_kern(mem[b + 1].b32.s1 - v);
        }
        free_node(b, BOX_NODE_SIZE);
        b = new_glue(12);
        mem[b].b32.s1 = p;
        while (mem[p].b32.s1 != TEX_NULL)
            p = LLIST_link(p);
        mem[p].b32.s1 = new_glue(12);
        return hpack(b, w, EXACTLY);
    } else {

        mem[b + 1].b32.s1 = w;
        return b;
    }
}
