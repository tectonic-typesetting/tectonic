/* Copyright 2016-2018 The Tectonic Project
 * Licensed under the MIT License.
 */

/* Customizations for Tectonic:
 *
 * In semantic pagination mode, we don't actually linebreak paragraphs. We
 * just set them as one big line at its natural width no matter what. In the
 * context of this algorithm that means that try_break is always a no-op
 * unless we're trying to break at the very end of the paragraph.
 */

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "tectonic_bridge_core.h"

#define AWFUL_BAD 0x3FFFFFFF

#define VERY_LOOSE_FIT 0
#define LOOSE_FIT 1
#define DECENT_FIT 2
#define TIGHT_FIT 3

static int32_t passive;
static scaled_t cur_active_width[7];
static scaled_t background[7];
static scaled_t break_width[7];
static int32_t best_place[4];
static int32_t best_pl_line[4];
static scaled_t disc_width;
static bool no_shrink_error_yet;
static int32_t cur_p;
static bool second_pass;
static bool final_pass;
static int32_t threshold;
static int32_t minimal_demerits[4];
static int32_t minimum_demerits;
static int32_t easy_line;
static int32_t last_special_line;
static scaled_t first_width;
static scaled_t second_width;
static scaled_t first_indent;
static scaled_t second_indent;
static int32_t best_bet;
static int32_t fewest_demerits;
static int32_t best_line;
static int32_t actual_looseness;
static int32_t line_diff;
static small_number hn;
static int32_t ha, hb;
static int32_t hyf_char;
static unsigned char init_cur_lang;
static int32_t l_hyf, r_hyf, init_l_hyf, init_r_hyf;
static int32_t hyf_bchar;
static int32_t last_line_fill;
static bool do_last_line_fit;
static small_number active_node_size;
static scaled_t fill_width[3];
static scaled_t best_pl_short[4];
static scaled_t best_pl_glue[4];


static void post_line_break(bool d);
static void try_break(int32_t pi, small_number break_type);
static void hyphenate(void);
static int32_t finite_shrink(int32_t p);
static small_number reconstitute(small_number j, small_number n, int32_t bchar, int32_t hchar);
static scaled_t total_pw(int32_t q, int32_t p);
static int32_t find_protchar_left(int32_t l, bool d);
static int32_t find_protchar_right(int32_t l, int32_t r);
static void push_node(int32_t p);
static int32_t pop_node(void);

/* "The active list ends where it begins" */
#define LAST_ACTIVE ACTIVE_LIST

static inline UnicodeScalar
get_native_usv(int32_t p, int32_t i) {
    unsigned short c = NATIVE_NODE_text(p)[i];

    if (c >= 0xD800 && c < 0xDC00)
        return 0x10000 + (c - 0xD800) * 0x400 + NATIVE_NODE_text(p)[i+1] - 0xDC00;

    return c;
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
void
line_break(bool d)
{
    bool auto_breaking;
    int32_t prev_p;
    int32_t q, r, s, prev_s;
    internal_font_number f;
    small_number j;
    UnicodeScalar c;
    int32_t l;
    int32_t i;
    int32_t for_end_1;

    pack_begin_line = cur_list.mode_line; /* "this is for over/underfull box messages" */

    LLIST_link(TEMP_HEAD) = LLIST_link(cur_list.head);

    /* Remove trailing space or glue if present; add infinite penalty then par_fill_skip */

    if (is_char_node(cur_list.tail)) { /* is_char_node */
        cur_list.tail = LLIST_link(cur_list.tail) = new_penalty(INF_PENALTY);
    } else if (NODE_type(cur_list.tail) != GLUE_NODE) {
        cur_list.tail = LLIST_link(cur_list.tail) = new_penalty(INF_PENALTY);
    } else {
        NODE_type(cur_list.tail) = PENALTY_NODE;
        delete_glue_ref(GLUE_NODE_glue_ptr(cur_list.tail));
        flush_node_list(GLUE_NODE_leader_ptr(cur_list.tail));
        PENALTY_NODE_penalty(cur_list.tail) = INF_PENALTY;
    }

    last_line_fill = LLIST_link(cur_list.tail) = new_param_glue(GLUE_PAR__par_fill_skip);

    /* Yet more initialization of various kinds */

    init_cur_lang = cur_list.prev_graf % 65536L;
    init_l_hyf = cur_list.prev_graf / 0x0400000;
    init_r_hyf = (cur_list.prev_graf / 65536L) % 64;

    pop_nest();

    no_shrink_error_yet = true;

    if (GLUE_SPEC_shrink_order(GLUEPAR(left_skip)) != NORMAL && GLUE_SPEC_shrink(GLUEPAR(left_skip)) != 0)
        GLUEPAR(left_skip) = finite_shrink(GLUEPAR(left_skip));

    if (GLUE_SPEC_shrink_order(GLUEPAR(right_skip)) != NORMAL && GLUE_SPEC_shrink(GLUEPAR(right_skip)) != 0)
        GLUEPAR(right_skip) = finite_shrink(GLUEPAR(right_skip));

    q = GLUEPAR(left_skip);
    r = GLUEPAR(right_skip);

    background[1] = BOX_width(q) + BOX_width(r);
    background[2] = 0;
    background[3] = 0;
    background[4] = 0;
    background[5] = 0;
    background[2 + GLUE_SPEC_stretch_order(q)] = GLUE_SPEC_stretch(q);
    background[2 + GLUE_SPEC_stretch_order(r)] += GLUE_SPEC_stretch(r);
    background[6] = GLUE_SPEC_shrink(q) + GLUE_SPEC_shrink(r);

    /* 1631: "check for special treatment of last line of paragraph" (\lastlinefit > 0) */

    do_last_line_fit = false;
    active_node_size = ACTIVE_NODE_SIZE_NORMAL;

    if (INTPAR(last_line_fit) > 0) {
        q = GLUE_NODE_glue_ptr(last_line_fill);

        if (GLUE_SPEC_stretch(q) > 0 && GLUE_SPEC_stretch_order(q) > NORMAL) {
            if (background[3] == 0 && background[4] == 0 && background[5] == 0) {
                do_last_line_fit = true;
                active_node_size = ACTIVE_NODE_SIZE_EXTENDED;
                fill_width[0] = 0;
                fill_width[1] = 0;
                fill_width[2] = 0;
                fill_width[GLUE_SPEC_stretch_order(q) - 1] = GLUE_SPEC_stretch(q);
            }
        }
    }

    minimum_demerits = AWFUL_BAD; /*863:*/
    minimal_demerits[TIGHT_FIT] = AWFUL_BAD;
    minimal_demerits[DECENT_FIT] = AWFUL_BAD;
    minimal_demerits[LOOSE_FIT] = AWFUL_BAD;
    minimal_demerits[VERY_LOOSE_FIT] = AWFUL_BAD;

    /* Prep relating to par_shape (877) */

    if (LOCAL(par_shape) == TEX_NULL) {
        if (DIMENPAR(hang_indent) == 0) {
            last_special_line = 0;
            second_width = DIMENPAR(hsize);
            second_indent = 0;
        } else { /*878:*/
            last_special_line = abs(INTPAR(hang_after));

            if (INTPAR(hang_after) < 0) {
                first_width = DIMENPAR(hsize) - abs(DIMENPAR(hang_indent));
                if (DIMENPAR(hang_indent) >= 0)
                    first_indent = DIMENPAR(hang_indent);
                else
                    first_indent = 0;
                second_width = DIMENPAR(hsize);
                second_indent = 0;
            } else {
                first_width = DIMENPAR(hsize);
                first_indent = 0;
                second_width = DIMENPAR(hsize) - abs(DIMENPAR(hang_indent));
                if (DIMENPAR(hang_indent) >= 0)
                    second_indent = DIMENPAR(hang_indent);
                else
                    second_indent = 0;
            }
        }
    } else {
        last_special_line = LLIST_info(LOCAL(par_shape)) - 1;
        /* These direct `mem` accesses are in the original WEB code */
        second_width = mem[LOCAL(par_shape) + 2 * (last_special_line + 1)].b32.s1;
        second_indent = mem[LOCAL(par_shape) + 2 * last_special_line + 1].b32.s1;
    }

    if (INTPAR(looseness) == 0)
        easy_line = last_special_line;
    else
        easy_line = MAX_HALFWORD; /*:877*/

    /* Start finding optimal breakpoints (892) */

    threshold = INTPAR(pretolerance);

    if (threshold >= 0) {
        second_pass = false;
        final_pass = false;
    } else {
        threshold = INTPAR(tolerance);
        second_pass = true;
        final_pass = (DIMENPAR(emergency_stretch) <= 0);
    }

    while (true) {
        if (threshold > INF_BAD)
            threshold = INF_BAD;

        if (second_pass) { /*920:*/
            if (trie_not_ready)
                init_trie();

            cur_lang = init_cur_lang;
            l_hyf = init_l_hyf;
            r_hyf = init_r_hyf;

            if (trie_trc[hyph_start + cur_lang] != cur_lang)
                hyph_index = 0;
            else
                hyph_index = trie_trl[hyph_start + cur_lang];
        }

        q = get_node(active_node_size); /*893:*/
        NODE_type(q) = UNHYPHENATED;
        ACTIVE_NODE_fitness(q) = DECENT_FIT;
        LLIST_link(q) = LAST_ACTIVE;
        ACTIVE_NODE_break_node(q) = TEX_NULL;
        ACTIVE_NODE_line_number(q) = cur_list.prev_graf + 1;
        ACTIVE_NODE_total_demerits(q) = 0;
        LLIST_link(ACTIVE_LIST) = q;

        if (do_last_line_fit) { /*1633:*/
            ACTIVE_NODE_shortfall(q) = 0;
            ACTIVE_NODE_glue(q) = 0;
        }

        active_width[1] = background[1];
        active_width[2] = background[2];
        active_width[3] = background[3];
        active_width[4] = background[4];
        active_width[5] = background[5];
        active_width[6] = background[6];
        passive = TEX_NULL;
        font_in_short_display = 0; /*:893*/
        cur_p = LLIST_link(TEMP_HEAD);
        auto_breaking = true;

        prev_p = global_prev_p = cur_p;
        first_p = cur_p;

        while (cur_p != TEX_NULL && LLIST_link(ACTIVE_LIST) != LAST_ACTIVE) {
            /*895: "Call try_break if cur_p is a legal breakpoint; on the
             * second pass, also try to hyphenate the next word, if cur_p is a
             * glue node; then advance cur_p to the next node of the paragraph
             * that could possibly be a legal breakpoint." */
            if (is_char_node(cur_p)) { /*896:*/
                prev_p = global_prev_p = cur_p;

                do {
                    int32_t eff_char;

                    f = CHAR_NODE_font(cur_p);
                    eff_char = effective_char(true, f, CHAR_NODE_character(cur_p));
                    active_width[1] += FONT_CHARACTER_WIDTH(f, eff_char);
                    cur_p = LLIST_link(cur_p);
                } while (is_char_node(cur_p));
            }

            switch (NODE_type(cur_p)) {
            case HLIST_NODE:
            case VLIST_NODE:
            case RULE_NODE:
                active_width[1] += BOX_width(cur_p);
                break;

            case WHATSIT_NODE:
                if (NODE_subtype(cur_p) == LANGUAGE_NODE) {
                    cur_lang = LANGUAGE_NODE_what_lang(cur_p);
                    l_hyf = LANGUAGE_NODE_what_lhm(cur_p);
                    r_hyf = LANGUAGE_NODE_what_rhm(cur_p);

                    if (trie_trc[hyph_start + cur_lang] != cur_lang)
                        hyph_index = 0;
                    else
                        hyph_index = trie_trl[hyph_start + cur_lang];
                } else if (NODE_subtype(cur_p) == NATIVE_WORD_NODE
                           || NODE_subtype(cur_p) == NATIVE_WORD_NODE_AT
                           || NODE_subtype(cur_p) == GLYPH_NODE
                           || NODE_subtype(cur_p) == PIC_NODE
                           || NODE_subtype(cur_p) == PDF_NODE) {
                    active_width[1] += BOX_width(cur_p);
                }
                break;

            case GLUE_NODE:
                if (auto_breaking) {
                    if (is_char_node(prev_p))
                        try_break(0, UNHYPHENATED);
                    else if (is_non_discardable_node(prev_p))
                        try_break(0, UNHYPHENATED);
                    else if (NODE_type(prev_p) == KERN_NODE && NODE_subtype(prev_p) != EXPLICIT)
                        try_break(0, UNHYPHENATED);
                }

                q = GLUE_NODE_glue_ptr(cur_p);

                if (GLUE_SPEC_shrink_order(q) != NORMAL && GLUE_SPEC_shrink(q) != 0)
                    q = GLUE_NODE_glue_ptr(cur_p) = finite_shrink(q);

                active_width[1] += BOX_width(q);
                active_width[2 + GLUE_SPEC_stretch_order(q)] += GLUE_SPEC_stretch(q);
                active_width[6] += GLUE_SPEC_shrink(q); /*:897*/

                if (second_pass && auto_breaking) {
                    /*924: "Try to hyphenate the following word." */
                    prev_s = cur_p;
                    s = LLIST_link(prev_s);

                    if (s != TEX_NULL) {
                        /*930: skip to node ha, or goto done1 if no hyphenation should be attempted */
                        while (true) {
                            if (is_char_node(s)) {
                                c = CHAR_NODE_character(s);
                                hf = CHAR_NODE_font(s);
                            } else if (NODE_type(s) == LIGATURE_NODE) {
                                if (LIGATURE_NODE_lig_ptr(s) == TEX_NULL)
                                    goto _continue;

                                q = LIGATURE_NODE_lig_ptr(s);
                                c = CHAR_NODE_character(q);
                                hf = CHAR_NODE_font(q);
                            } else if (NODE_type(s) == KERN_NODE && NODE_subtype(s) == NORMAL) {
                                goto _continue;
                            } else if (NODE_type(s) == MATH_NODE && NODE_subtype(s) >= L_CODE) {
                                goto _continue;
                            } else if (NODE_type(s) == WHATSIT_NODE) {
                                if (NODE_subtype(s) == NATIVE_WORD_NODE || NODE_subtype(s) == NATIVE_WORD_NODE_AT) {
                                    for (l = 0; l < NATIVE_NODE_length(s); l++) {
                                        c = get_native_usv(s, l);
                                        if (LC_CODE(c) != 0) {
                                            hf = NATIVE_NODE_font(s);
                                            prev_s = s;
                                            if (LC_CODE(c) == c || INTPAR(uc_hyph) > 0)
                                                goto done2;
                                            else
                                                goto done1;
                                        }

                                        if (c >= 65536L)
                                            l++;
                                    }
                                }

                                if (NODE_subtype(s) == LANGUAGE_NODE) {
                                    cur_lang = LANGUAGE_NODE_what_lang(s);
                                    l_hyf = LANGUAGE_NODE_what_lhm(s);
                                    r_hyf = LANGUAGE_NODE_what_rhm(s);

                                    if (trie_trc[hyph_start + cur_lang] != cur_lang)
                                        hyph_index = 0;
                                    else
                                        hyph_index = trie_trl[hyph_start + cur_lang];
                                }

                                goto _continue;
                            } else {
                                goto done1;
                            }

                            if (hyph_index == 0 || c > 255)
                                hc[0] = LC_CODE(c);
                            else if (trie_trc[hyph_index + c] != c)
                                hc[0] = 0;
                            else
                                hc[0] = trie_tro[hyph_index + c];

                            if (hc[0] != 0) {
                                if (hc[0] == c || INTPAR(uc_hyph) > 0)
                                    goto done2;
                                else
                                    goto done1;
                            }

                        _continue:
                            prev_s = s;
                            s = LLIST_link(prev_s);
                        }

                    done2:
                        hyf_char = hyphen_char[hf];
                        if (hyf_char < 0)
                            goto done1;
                        if (hyf_char > BIGGEST_CHAR)
                            goto done1;

                        ha = prev_s; /*:930*/

                        if (l_hyf + r_hyf > max_hyphenatable_length())
                            goto done1;

                        if (ha != TEX_NULL && ha < hi_mem_min && NODE_type(ha) == WHATSIT_NODE &&
                            (NODE_subtype(ha) == NATIVE_WORD_NODE || NODE_subtype(ha) == NATIVE_WORD_NODE_AT))
                        {
                            /*926: check that nodes after native_word permit hyphenation; if not, goto done1 */
                            s = LLIST_link(ha);

                            while (true) {
                                if (!is_char_node(s)) {
                                    switch (NODE_type(s)) {
                                    case LIGATURE_NODE:
                                        break;

                                    case KERN_NODE:
                                        if (NODE_subtype(s) != NORMAL)
                                            goto done6;
                                        break;

                                    case WHATSIT_NODE:
                                    case GLUE_NODE:
                                    case PENALTY_NODE:
                                    case INS_NODE:
                                    case ADJUST_NODE:
                                    case MARK_NODE:
                                        goto done6;
                                        break;

                                    default:
                                        goto done1;
                                        break;
                                    }
                                }

                                s = LLIST_link(s);
                            }

                        done6:
                            /*927: prepare a native_word_node for hyphenation.
                             * "Note that if there are chars with lccode = 0,
                             * we split them out into separate native_word
                             * nodes." */
                            hn = 0;

                        restart:
                            /* 'ha' can change in the loop, so for safety: */
                            for_end_1 = NATIVE_NODE_length(ha);

                            for (l = 0; l < for_end_1; l++) {
                                c = get_native_usv(ha, l);

                                if (hyph_index == 0 || c > 255)
                                    hc[0] = LC_CODE(c);
                                else if (trie_trc[hyph_index + c] != c)
                                    hc[0] = 0;
                                else
                                    hc[0] = trie_tro[hyph_index + c];

                                if (hc[0] == 0) {
                                    if (hn > 0) {
                                        q = new_native_word_node(hf, NATIVE_NODE_length(ha) - l);
                                        NODE_subtype(q) = NODE_subtype(ha);

                                        for (i = l; i < NATIVE_NODE_length(ha); i++)
                                            NATIVE_NODE_text(q)[i - l] = NATIVE_NODE_text(ha)[i];

                                        set_native_metrics(q, (INTPAR(xetex_use_glyph_metrics) > 0));
                                        LLIST_link(q) = LLIST_link(ha);
                                        LLIST_link(ha) = q;
                                        NATIVE_NODE_length(ha) = l;
                                        set_native_metrics(ha, (INTPAR(xetex_use_glyph_metrics) > 0));
                                        goto done3;
                                    }
                                } else if (hn == 0 && l > 0) {
                                    q = new_native_word_node(hf, NATIVE_NODE_length(ha) - l);
                                    NODE_subtype(q) = NODE_subtype(ha);

                                    for (i = l; i < NATIVE_NODE_length(ha); i++)
                                        NATIVE_NODE_text(q)[i - l] = NATIVE_NODE_text(ha)[i];

                                    set_native_metrics(q, (INTPAR(xetex_use_glyph_metrics) > 0));
                                    LLIST_link(q) = LLIST_link(ha);
                                    LLIST_link(ha) = q;
                                    NATIVE_NODE_length(ha) = l;
                                    set_native_metrics(ha, (INTPAR(xetex_use_glyph_metrics) > 0));
                                    ha = LLIST_link(ha);
                                    goto restart;
                                } else if (hn == max_hyphenatable_length()) {
                                    goto done3;
                                } else {
                                    hn++;

                                    if (c < 65536L) {
                                        hu[hn] = c;
                                        hc[hn] = hc[0];
                                    } else {
                                        hu[hn] = (c - 65536L) / 1024 + 0xD800;
                                        hc[hn] = (hc[0] - 65536L) / 1024 + 0xD800;
                                        hn++;
                                        hu[hn] = c % 1024 + 0xDC00;
                                        hc[hn] = hc[0] % 1024 + 0xDC00;
                                        l++;
                                    }

                                    hyf_bchar = TOO_BIG_CHAR;
                                }
                            }
                        } else {
                            /*931: skip to node hb, putting letters into hu and hc */
                            hn = 0;

                            while (true) {
                                if (is_char_node(s)) {
                                    if (CHAR_NODE_font(s) != hf)
                                        goto done3;

                                    hyf_bchar = CHAR_NODE_character(s);
                                    c = hyf_bchar;

                                    if (hyph_index == 0 || c > 255)
                                        hc[0] = LC_CODE(c);
                                    else if (trie_trc[hyph_index + c] != c)
                                        hc[0] = 0;
                                    else
                                        hc[0] = trie_tro[hyph_index + c];

                                    if (hc[0] == 0)
                                        goto done3;
                                    if (hc[0] > max_hyph_char)
                                        goto done3;
                                    if (hn == max_hyphenatable_length())
                                        goto done3;

                                    hb = s;
                                    hn++;
                                    hu[hn] = c;
                                    hc[hn] = hc[0];
                                    hyf_bchar = TOO_BIG_CHAR;
                                } else if (NODE_type(s) == LIGATURE_NODE) {
                                    /*932: move the characters of a ligature node to hu and hc; but goto done3
                                     * if they are not all letters. */
                                    if (LIGATURE_NODE_lig_font(s) != hf)
                                        goto done3;

                                    j = hn;
                                    q = LIGATURE_NODE_lig_ptr(s);

                                    if (q > TEX_NULL)
                                        hyf_bchar = CHAR_NODE_character(q);

                                    while (q > TEX_NULL) {
                                        c = CHAR_NODE_character(q);

                                        if (hyph_index == 0 || c > 255)
                                            hc[0] = LC_CODE(c);
                                        else if (trie_trc[hyph_index + c] != c)
                                            hc[0] = 0;
                                        else
                                            hc[0] = trie_tro[hyph_index + c];

                                        if (hc[0] == 0)
                                            goto done3;
                                        if (hc[0] > max_hyph_char)
                                            goto done3;
                                        if (j == max_hyphenatable_length())
                                            goto done3;

                                        j++;
                                        hu[j] = c;
                                        hc[j] = hc[0];
                                        q = LLIST_link(q);
                                    }

                                    hb = s;
                                    hn = j;

                                    if (odd(NODE_subtype(s)))
                                        hyf_bchar = font_bchar[hf];
                                    else
                                        hyf_bchar = TOO_BIG_CHAR; /*:932*/
                                } else if (NODE_type(s) == KERN_NODE && NODE_subtype(s) == NORMAL) {
                                    hb = s;
                                    hyf_bchar = font_bchar[hf];
                                } else {
                                    goto done3;
                                }

                                s = LLIST_link(s);
                            }

                        done3:
                            ;
                        }

                        /*933: check that the nodes following hb permit
                         * hyphenation and that at least l_hyf + r_hyf letters
                         * have been found, otherwise goto done1 */

                        if (hn < l_hyf + r_hyf)
                            goto done1;

                        while (true) {
                            if (!is_char_node(s)) {
                                switch (NODE_type(s)) {
                                case LIGATURE_NODE:
                                    break;

                                case KERN_NODE:
                                    if (NODE_subtype(s) != NORMAL)
                                        goto done4;
                                    break;

                                case WHATSIT_NODE:
                                case GLUE_NODE:
                                case PENALTY_NODE:
                                case INS_NODE:
                                case ADJUST_NODE:
                                case MARK_NODE:
                                    goto done4;
                                    break;

                                case MATH_NODE:
                                    if (NODE_subtype(s) >= L_CODE)
                                        goto done4;
                                    else
                                        goto done1;
                                    break;

                                default:
                                    goto done1;
                                    break;
                                }
                            }

                            s = LLIST_link(s);
                        }

                    done4: /*:933*/
                        hyphenate();
                    }

                done1: /*:924*/
                    ;
                }
                break; /* that was a long-ass GLUE_NODE case */

            /* ... resuming 895 ... */
            case KERN_NODE:
                if (NODE_subtype(cur_p) == EXPLICIT) {
                    if (!is_char_node(LLIST_link(cur_p)) && auto_breaking) {
                        if (NODE_type(LLIST_link(cur_p)) == GLUE_NODE)
                            try_break(0, UNHYPHENATED);
                    }
                    active_width[1] += BOX_width(cur_p);
                } else
                    active_width[1] += BOX_width(cur_p);
                break;

            case LIGATURE_NODE:
                f = LIGATURE_NODE_lig_font(cur_p);
                xtx_ligature_present = true;
                active_width[1] += FONT_CHARACTER_WIDTH(f, effective_char(true, f, LIGATURE_NODE_lig_char(cur_p)));
                break;

            case DISC_NODE:
                /*898: try to break after a discretionary fragment, then goto done5 */
                s = DISCRETIONARY_NODE_pre_break(cur_p);
                disc_width = 0;

                if (s == TEX_NULL) {
                    try_break(INTPAR(ex_hyphen_penalty), HYPHENATED);
                } else {
                    do {
                        /*899:*/
                        if (is_char_node(s)) {
                            int32_t eff_char;

                            f = CHAR_NODE_font(s);
                            eff_char = effective_char(true, f, CHAR_NODE_character(s));
                            disc_width += FONT_CHARACTER_WIDTH(f, eff_char);
                        } else {
                            switch (NODE_type(s)) {
                            case LIGATURE_NODE:
                            {
                                int32_t eff_char;

                                f = LIGATURE_NODE_lig_font(s);
                                xtx_ligature_present = true;
                                eff_char = effective_char(true, f, LIGATURE_NODE_lig_char(s));
                                disc_width += FONT_CHARACTER_WIDTH(f, eff_char);
                                break;
                            }

                            case HLIST_NODE:
                            case VLIST_NODE:
                            case RULE_NODE:
                            case KERN_NODE:
                                disc_width += BOX_width(s);
                                break;

                            case WHATSIT_NODE:
                                if (NODE_subtype(s) == NATIVE_WORD_NODE ||
                                    NODE_subtype(s) == NATIVE_WORD_NODE_AT ||
                                    NODE_subtype(s) == GLYPH_NODE ||
                                    NODE_subtype(s) == PIC_NODE ||
                                    NODE_subtype(s) == PDF_NODE)
                                    disc_width += BOX_width(s);
                                else
                                    confusion("disc3a");
                                break;

                            default:
                                confusion("disc3");
                                break;
                            }
                        }

                        s = LLIST_link(s);
                    } while (s != TEX_NULL);

                    active_width[1] += disc_width;
                    try_break(INTPAR(hyphen_penalty), HYPHENATED);
                    active_width[1] -= disc_width;
                }

                r = DISCRETIONARY_NODE_replace_count(cur_p);
                s = LLIST_link(cur_p);

                while (r > 0) {
                    if (is_char_node(s)) {
                        int32_t eff_char;

                        f = CHAR_NODE_font(s);
                        eff_char = effective_char(true, f, CHAR_NODE_character(s));
                        active_width[1] += FONT_CHARACTER_WIDTH(f, eff_char);
                    } else {
                        switch (NODE_type(s)) {
                        case LIGATURE_NODE:
                        {
                            int32_t eff_char;

                            f = LIGATURE_NODE_lig_font(s);
                            xtx_ligature_present = true;
                            eff_char = effective_char(true, f, LIGATURE_NODE_lig_char(s));
                            active_width[1] += FONT_CHARACTER_WIDTH(f, eff_char);
                            break;
                        }

                        case HLIST_NODE:
                        case VLIST_NODE:
                        case RULE_NODE:
                        case KERN_NODE:
                            active_width[1] += BOX_width(s);
                            break;

                        case WHATSIT_NODE:
                            if (NODE_subtype(s) == NATIVE_WORD_NODE ||
                                NODE_subtype(s) == NATIVE_WORD_NODE_AT ||
                                NODE_subtype(s) == GLYPH_NODE ||
                                NODE_subtype(s) == PIC_NODE ||
                                NODE_subtype(s) == PDF_NODE)
                                active_width[1] += BOX_width(s);
                            else
                                confusion("disc4a");
                            break;

                        default:
                            confusion("disc4");
                            break;
                        }
                    }

                    r--;
                    s = LLIST_link(s);
                }

                prev_p = global_prev_p = cur_p;
                cur_p = s;
                goto done5;
                break; /*:898 big DISC_NODE case */

            case MATH_NODE:
                if (NODE_subtype(cur_p) < L_CODE)
                    auto_breaking = odd(NODE_subtype(cur_p));

                if (!is_char_node(LLIST_link(cur_p)) && auto_breaking) {
                    if (NODE_type(LLIST_link(cur_p)) == GLUE_NODE)
                        try_break(0, UNHYPHENATED);
                }

                active_width[1] += BOX_width(cur_p);
                break;

            case PENALTY_NODE:
                try_break(PENALTY_NODE_penalty(cur_p), UNHYPHENATED);
                break;

            case MARK_NODE:
            case INS_NODE:
            case ADJUST_NODE:
                break;

            default:
                confusion("paragraph");
                break;
            }

            prev_p = global_prev_p = cur_p;
            cur_p = LLIST_link(cur_p);
        done5:
            ; /*:895*/
        }

        if (cur_p == TEX_NULL) {
            /*902: "Try the final line break at the end of the paragraph, and
             * goto done if the desired breakpoints have been found." */
            try_break(EJECT_PENALTY, HYPHENATED);

            if (LLIST_link(ACTIVE_LIST) != LAST_ACTIVE) { /*903:*/
                r = LLIST_link(ACTIVE_LIST);
                fewest_demerits = MAX_HALFWORD;

                do {
                    if (NODE_type(r) != DELTA_NODE) {
                        if (ACTIVE_NODE_total_demerits(r) < fewest_demerits) {
                            fewest_demerits = ACTIVE_NODE_total_demerits(r);
                            best_bet = r;
                        }
                    }

                    r = LLIST_link(r);
                } while (r != LAST_ACTIVE);

                best_line = ACTIVE_NODE_line_number(best_bet); /*:903*/

                if (INTPAR(looseness) == 0)
                    goto done;

                r = LLIST_link(ACTIVE_LIST); /*904:*/
                actual_looseness = 0;

                do {
                    if (NODE_type(r) != DELTA_NODE) {
                        line_diff = ACTIVE_NODE_line_number(r) - best_line;

                        if ((line_diff < actual_looseness && INTPAR(looseness) <= line_diff)
                            || (line_diff > actual_looseness && INTPAR(looseness) >= line_diff)) {
                            best_bet = r;
                            actual_looseness = line_diff;
                            fewest_demerits = ACTIVE_NODE_total_demerits(r);
                        } else if (line_diff == actual_looseness && ACTIVE_NODE_total_demerits(r) < fewest_demerits) {
                            best_bet = r;
                            fewest_demerits = ACTIVE_NODE_total_demerits(r);
                        }
                    }

                    r = LLIST_link(r);
                } while (r != LAST_ACTIVE);

                best_line = ACTIVE_NODE_line_number(best_bet); /*:904*/

                if (actual_looseness == INTPAR(looseness) || final_pass)
                    goto done;
            } /*:902*/
        }

        /*894: clean up the memory by removing the break nodes */

        q = LLIST_link(ACTIVE_LIST);

        while (q != LAST_ACTIVE) {
            cur_p = LLIST_link(q);

            if (NODE_type(q) == DELTA_NODE)
                free_node(q, DELTA_NODE_SIZE);
            else
                free_node(q, active_node_size);

            q = cur_p;
        }

        q = passive;

        while (q != TEX_NULL) {
            cur_p = LLIST_link(q);
            free_node(q, PASSIVE_NODE_SIZE);
            q = cur_p;
        }

        /* ... resuming 892 ... */

        if (!second_pass) {
            threshold = INTPAR(tolerance);
            second_pass = true;
            final_pass = (DIMENPAR(emergency_stretch) <= 0);
        } else {
            background[2] = background[2] + DIMENPAR(emergency_stretch);
            final_pass = true;
        }
    }

done:
    if (do_last_line_fit) { /*1641:*/
        if (ACTIVE_NODE_shortfall(best_bet) == 0) {
            do_last_line_fit = false;
        } else {
            q = new_spec(GLUE_NODE_glue_ptr(last_line_fill));
            delete_glue_ref(GLUE_NODE_glue_ptr(last_line_fill));
            BOX_width(q) += ACTIVE_NODE_shortfall(best_bet) - ACTIVE_NODE_glue(best_bet);
            GLUE_SPEC_stretch(q) = 0;
            GLUE_NODE_glue_ptr(last_line_fill) = q;
        }
    }

    post_line_break(d);

    /* Clean up by removing break nodes (894, again) */

    q = LLIST_link(ACTIVE_LIST);

    while (q != ACTIVE_LIST) {
        int32_t next = LLIST_link(q);

        if (NODE_type(q) == DELTA_NODE)
            free_node(q, DELTA_NODE_SIZE);
        else
            free_node(q, active_node_size);

        q = next;
    }

    q = passive;

    while (q != TEX_NULL) {
        int32_t next = LLIST_link(q);
        free_node(q, PASSIVE_NODE_SIZE);
        q = next;
    }

    /* All done */
    pack_begin_line = 0;
}


/* This was just separated out to prevent line_break() from becoming
 * proposterously long. */
static void
post_line_break(bool d)
{
    int32_t q, r, s;
    int32_t p, k;
    scaled_t w;
    bool glue_break;
    int32_t ptmp;
    bool disc_break;
    bool post_disc_break;
    scaled_t cur_width;
    scaled_t cur_indent;
    uint16_t t;
    int32_t pen;
    int32_t cur_line;
    int32_t LR_ptr;

    LR_ptr = cur_list.eTeX_aux;

    /* Reverse the list of break nodes (907) */

    q = ACTIVE_NODE_break_node(best_bet);
    cur_p = TEX_NULL;

    do {
        r = q;
        q = PASSIVE_NODE_prev_break(q);
        PASSIVE_NODE_next_break(r) = cur_p;
        cur_p = r;
    } while (q != TEX_NULL); /*:907*/

    cur_line = cur_list.prev_graf + 1;

    do {
        /* 909: justify the line ending at breakpoint cur_p and append it to
         * the current vertical list, with associated penalties and
         * insertions. The current line starts a TEMP_HEAD.link and ends at
         * cur_p.cur_break.
         **/

        if (INTPAR(texxet) > 0) { /*1494:*/
            q = mem[TEMP_HEAD].b32.s1;

            if (LR_ptr != TEX_NULL) {
                temp_ptr = LR_ptr;
                r = q;

                do {
                    s = new_math(0, (mem[temp_ptr].b32.s0 - 1));
                    mem[s].b32.s1 = r;
                    r = s;
                    temp_ptr = LLIST_link(temp_ptr);
                } while (temp_ptr != TEX_NULL);

                mem[TEMP_HEAD].b32.s1 = r;
            }

            while (q != mem[cur_p + 1].b32.s1) {
                if (q < hi_mem_min && NODE_type(q) == MATH_NODE) { /*1495:*/
                    if (odd(mem[q].b16.s0)) {
                        if (LR_ptr != TEX_NULL && mem[LR_ptr].b32.s0 == (L_CODE * (mem[q].b16.s0 / L_CODE) + 3)) {
                            temp_ptr = LR_ptr;
                            LR_ptr = mem[temp_ptr].b32.s1;
                            mem[temp_ptr].b32.s1 = avail;
                            avail = temp_ptr;
                        }
                    } else {
                        temp_ptr = get_avail();
                        mem[temp_ptr].b32.s0 = (L_CODE * (mem[q].b16.s0 / L_CODE) + 3);
                        mem[temp_ptr].b32.s1 = LR_ptr;
                        LR_ptr = temp_ptr;
                    }
                }

                q = LLIST_link(q);
            }
        }

        /* 910: "Modify the end of the line to reflect the nature of the break
         * and to include \rightskip; also set the proper value of
         * disc_break" */

        q = PASSIVE_NODE_cur_break(cur_p);
        disc_break = false;
        post_disc_break = false;
        glue_break = false;

        if (q == TEX_NULL) {
            q = TEMP_HEAD;

            while (LLIST_link(q) != TEX_NULL)
                q = LLIST_link(q);
        } else if (NODE_type(q) == GLUE_NODE) {
            delete_glue_ref(GLUE_NODE_glue_ptr(q));
            GLUE_NODE_glue_ptr(q) = GLUEPAR(right_skip);
            NODE_subtype(q) = GLUE_PAR__right_skip + 1;
            GLUE_SPEC_ref_count(GLUEPAR(right_skip))++;
            glue_break = true;
        } else if (NODE_type(q) == DISC_NODE) { /*911:*/
            t = DISCRETIONARY_NODE_replace_count(q);

            if (t == 0) {
                r = LLIST_link(q);
            } else {
                r = q;

                while (t > 1) {
                    r = LLIST_link(r);
                    t--;
                }

                s = LLIST_link(r);
                r = LLIST_link(s);
                LLIST_link(s) = TEX_NULL;
                flush_node_list(LLIST_link(q));
                DISCRETIONARY_NODE_replace_count(q) = 0;
            }

            if (DISCRETIONARY_NODE_post_break(q) != TEX_NULL) { /*913:*/
                s = DISCRETIONARY_NODE_post_break(q);

                while (LLIST_link(s) != TEX_NULL)
                    s = LLIST_link(s);

                LLIST_link(s) = r;

                r = DISCRETIONARY_NODE_post_break(q);
                DISCRETIONARY_NODE_post_break(q) = TEX_NULL;
                post_disc_break = true;
            }

            if (DISCRETIONARY_NODE_pre_break(q) != TEX_NULL) { /*914:*/
                s = DISCRETIONARY_NODE_pre_break(q);
                LLIST_link(q) = s;

                while (LLIST_link(s) != TEX_NULL)
                    s = LLIST_link(s);

                DISCRETIONARY_NODE_pre_break(q) = TEX_NULL;
                q = s;
            }

            LLIST_link(q) = r;
            disc_break = true;
        } else if (NODE_type(q) == KERN_NODE) {
            BOX_width(q) = 0;
        } else if (NODE_type(q) == MATH_NODE) {
            BOX_width(q) = 0;

            if (INTPAR(texxet) > 0) { /*1495:*/
                if (odd(mem[q].b16.s0)) {
                    if (LR_ptr != TEX_NULL && mem[LR_ptr].b32.s0 == (L_CODE * (mem[q].b16.s0 / L_CODE) + 3)) {
                        temp_ptr = LR_ptr;
                        LR_ptr = mem[temp_ptr].b32.s1;
                        mem[temp_ptr].b32.s1 = avail;
                        avail = temp_ptr;
                    }
                } else {
                    temp_ptr = get_avail();
                    mem[temp_ptr].b32.s0 = (L_CODE * (mem[q].b16.s0 / L_CODE) + 3);
                    mem[temp_ptr].b32.s1 = LR_ptr;
                    LR_ptr = temp_ptr;
                }
            }
        }

        /* "at this point q is the rightmost breakpoint; the only exception is
         * the case of a discretionary break with non-empty pre_break -- then
         * q has been changed to the last node of the pre-break list" */

        if (INTPAR(xetex_protrude_chars) > 0) {
            if (disc_break && (is_char_node(q) || NODE_type(q) != DISC_NODE)) {
                p = q;
                ptmp = p;
            } else {
                p = prev_rightmost(mem[TEMP_HEAD].b32.s1, q);
                ptmp = p;
                p = find_protchar_right(mem[TEMP_HEAD].b32.s1, p);
            }

            w = char_pw(p, 1);

            if (w != 0) {
                k = new_margin_kern(-(int32_t) w, last_rightmost_char, 1);
                mem[k].b32.s1 = mem[ptmp].b32.s1;
                mem[ptmp].b32.s1 = k;
                if (ptmp == q)
                    q = LLIST_link(q);
            }
        }

        if (!glue_break) {
            r = new_param_glue(GLUE_PAR__right_skip);
            LLIST_link(r) = LLIST_link(q);
            LLIST_link(q) = r;
            q = r;
        } /*:915*/

        if (INTPAR(texxet) > 0) { /*1496:*/
            if (LR_ptr != TEX_NULL) {
                s = TEMP_HEAD;
                r = mem[s].b32.s1;

                while (r != q) {
                    s = r;
                    r = mem[s].b32.s1;
                }

                r = LR_ptr;

                while (r != TEX_NULL) {
                    temp_ptr = new_math(0, mem[r].b32.s0);
                    mem[s].b32.s1 = temp_ptr;
                    s = temp_ptr;
                    r = LLIST_link(r);
                }

                mem[s].b32.s1 = q;
            }
        }

        /* 916: Put \leftskip at the left and detach this line. */

        r = LLIST_link(q);
        LLIST_link(q) = TEX_NULL;
        q = LLIST_link(TEMP_HEAD);
        LLIST_link(TEMP_HEAD) = r;

        /* "at this point q is the leftmost node; all discardable nodes have been discarded */

        if (INTPAR(xetex_protrude_chars) > 0) {
            p = q;
            p = find_protchar_left(p, false);
            w = char_pw(p, 0);
            if (w != 0) {
                k = new_margin_kern(-(int32_t) w, last_leftmost_char, 0);
                LLIST_link(k) = q;
                q = k;
            }
        }

        if (GLUEPAR(left_skip) != 0) {
            r = new_param_glue(GLUE_PAR__left_skip);
            LLIST_link(r) = q;
            q = r;
        }

        /* 918: q points to the hlist that represents the current line. Pack
         * it up at the right width. */

        if (cur_line > last_special_line) {
            cur_width = second_width;
            cur_indent = second_indent;
        } else if (LOCAL(par_shape) == TEX_NULL) {
            cur_width = first_width;
            cur_indent = first_indent;
        } else {
            /* These manual `mem` indices are in the original WEB code */
            cur_width = mem[LOCAL(par_shape) + 2 * cur_line].b32.s1;
            cur_indent = mem[LOCAL(par_shape) + 2 * cur_line - 1].b32.s1;
        }

        adjust_tail = ADJUST_HEAD;
        pre_adjust_tail = PRE_ADJUST_HEAD;

        /* Tectonic: in semantic pagination mode, set each "line" (really the
         * whole paragraph) at its natural width. */

        if (semantic_pagination_enabled) {
            just_box = hpack(q, 0, ADDITIONAL);
        } else {
            just_box = hpack(q, cur_width, EXACTLY);
        }

        BOX_shift_amount(just_box) = cur_indent; /*:918*/

        /* 917: append the new box to the current vertical list, followed
         * by any of its special nodes that were taken out */

        if (PRE_ADJUST_HEAD != pre_adjust_tail) {
            LLIST_link(cur_list.tail) = LLIST_link(PRE_ADJUST_HEAD);
            cur_list.tail = pre_adjust_tail;
        }

        pre_adjust_tail = TEX_NULL;
        append_to_vlist(just_box);

        if (ADJUST_HEAD != adjust_tail) {
            LLIST_link(cur_list.tail) = LLIST_link(ADJUST_HEAD);
            cur_list.tail = adjust_tail;
        }

        adjust_tail = TEX_NULL; /*:917*/

        /* 919: Set `pen` to all of the penalties relevant to this line. */

        if (cur_line + 1 != best_line) {
            q = eqtb[INTER_LINE_PENALTIES_LOC].b32.s1;

            if (q != TEX_NULL) {
                r = cur_line;
                if (r > PENALTY_NODE_penalty(q))
                    r = PENALTY_NODE_penalty(q);
                pen = PENALTY_NODE_penalty(q + r);
            } else {
                pen = INTPAR(inter_line_penalty);
            }

            q = eqtb[CLUB_PENALTIES_LOC].b32.s1;

            if (q != TEX_NULL) {
                r = cur_line - cur_list.prev_graf;
                if (r > PENALTY_NODE_penalty(q))
                    r = PENALTY_NODE_penalty(q);
                pen += PENALTY_NODE_penalty(q + r);
            } else if (cur_line == cur_list.prev_graf + 1) {
                pen += INTPAR(club_penalty);
            }

            if (d)
                q = eqtb[DISPLAY_WIDOW_PENALTIES_LOC].b32.s1;
            else
                q = eqtb[WIDOW_PENALTIES_LOC].b32.s1;

            if (q != TEX_NULL) {
                r = best_line - cur_line - 1;
                if (r > PENALTY_NODE_penalty(q))
                    r = PENALTY_NODE_penalty(q);
                pen += PENALTY_NODE_penalty(q + r);
            } else if (cur_line + 2 == best_line) {
                if (d)
                    pen += INTPAR(display_widow_penalty);
                else
                    pen += INTPAR(widow_penalty);
            }

            if (disc_break)
                pen += INTPAR(broken_penalty);

            if (pen != 0) {
                r = new_penalty(pen);
                LLIST_link(cur_list.tail) = r;
                cur_list.tail = r;
            }
        }

        /* Done justifying this line. */

        cur_line++;
        cur_p = PASSIVE_NODE_next_break(cur_p);

        if (cur_p != TEX_NULL) {
            if (!post_disc_break) {
                /* 908: "prune unwanted nodes at the beginning of the next
                 * line". Delete glues, penalties, kerns, and math nodes at
                 * the beginning of the line, unless the node in question is
                 * the chosen breakpoint. */
                r = TEMP_HEAD;

                while (true) {
                    q = LLIST_link(r);

                    if (q == PASSIVE_NODE_cur_break(cur_p))
                        break;
                    if (is_char_node(q))
                        break;
                    if (is_non_discardable_node(q))
                        break;
                    if (NODE_type(q) == KERN_NODE && NODE_subtype(q) != EXPLICIT && NODE_subtype(q) != SPACE_ADJUSTMENT)
                        break;

                    r = q;

                    if (NODE_type(q) == MATH_NODE && INTPAR(texxet) > 0) { /*1495:*/
                        if (odd(mem[q].b16.s0)) {
                            if (LR_ptr != TEX_NULL && mem[LR_ptr].b32.s0 == (L_CODE * (mem[q].b16.s0 / L_CODE) + 3)) {
                                temp_ptr = LR_ptr;
                                LR_ptr = mem[temp_ptr].b32.s1;
                                mem[temp_ptr].b32.s1 = avail;
                                avail = temp_ptr;
                            }
                        } else {
                            temp_ptr = get_avail();
                            mem[temp_ptr].b32.s0 = (L_CODE * (mem[q].b16.s0 / L_CODE) + 3);
                            mem[temp_ptr].b32.s1 = LR_ptr;
                            LR_ptr = temp_ptr;
                        }
                    }
                }

                if (r != TEMP_HEAD) {
                    LLIST_link(r) = TEX_NULL;
                    flush_node_list(LLIST_link(TEMP_HEAD));
                    LLIST_link(TEMP_HEAD) = q;
                }
            }
        }
    } while (cur_p != TEX_NULL);

    if (cur_line != best_line || LLIST_link(TEMP_HEAD) != TEX_NULL)
        confusion("line breaking");

    cur_list.prev_graf = best_line - 1;
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
static void
try_break(int32_t pi, small_number break_type)
{
    int32_t r;
    int32_t prev_r;
    int32_t old_l;
    bool no_break_yet;
    int32_t prev_prev_r = TEX_NULL;
    int32_t s;
    int32_t q;
    int32_t v;
    int32_t t;
    internal_font_number f;
    int32_t l;
    bool node_r_stays_active;
    scaled_t line_width = 0;
    unsigned char /*tight_fit */ fit_class;
    int32_t b;
    int32_t d;
    bool artificial_demerits;
    scaled_t shortfall;
    scaled_t g = 0;

    /* Tectonic: no-op except at the end of the paragraph. We know we're at
     * the very end of the paragraph when cur_p is TEX_NULL. */
    if (semantic_pagination_enabled && cur_p != TEX_NULL)
        return;

    if (abs(pi) >= INF_PENALTY) {
        if (pi > 0)
            return;

        pi = EJECT_PENALTY;
    }

    no_break_yet = true;
    prev_r = ACTIVE_LIST;
    old_l = 0;
    cur_active_width[1] = active_width[1];
    cur_active_width[2] = active_width[2];
    cur_active_width[3] = active_width[3];
    cur_active_width[4] = active_width[4];
    cur_active_width[5] = active_width[5];
    cur_active_width[6] = active_width[6];

    while (true) {
        r = LLIST_link(prev_r);

        /*861: "If node r is of type delta_node, update cur_active_width, set
         * prev_r and prev_prev_r, then goto continue" */

        if (NODE_type(r) == DELTA_NODE) {
            cur_active_width[1] += DELTA_NODE_dwidth(r);
            cur_active_width[2] += DELTA_NODE_dstretch0(r);
            cur_active_width[3] += DELTA_NODE_dstretch1(r);
            cur_active_width[4] += DELTA_NODE_dstretch2(r);
            cur_active_width[5] += DELTA_NODE_dstretch3(r);
            cur_active_width[6] += DELTA_NODE_dshrink(r);
            prev_prev_r = prev_r;
            prev_r = r;
            continue;
        }

        /*864: "If a line number class has ended, create new active nodes for
         * the best feasible breaks in that class; then return if r =
         * last_active, otherwise compute the new line_width." */

        l = ACTIVE_NODE_line_number(r);

        if (l > old_l) { /* "now we are no longer in the inner loop" */
            if (minimum_demerits < AWFUL_BAD && (old_l != easy_line || r == LAST_ACTIVE)) {
                /*865: "Create new active nodes for the best feasible breaks
                 * just found." */
                if (no_break_yet) {
                    /*866: "Compute the values of break_width". */
                    no_break_yet = false;
                    break_width[1] = background[1];
                    break_width[2] = background[2];
                    break_width[3] = background[3];
                    break_width[4] = background[4];
                    break_width[5] = background[5];
                    break_width[6] = background[6];
                    s = cur_p;

                    if (break_type > UNHYPHENATED) {
                        /*869: "Compute the discretionary break_width values" */
                        if (cur_p != TEX_NULL) {
                            t = DISCRETIONARY_NODE_replace_count(cur_p);
                            v = cur_p;
                            s = DISCRETIONARY_NODE_post_break(cur_p);

                            while (t > 0) {
                                t--;
                                v = LLIST_link(v);

                                /*870: "subtract the width of node v from break_width" */
                                if (is_char_node(v)) {
                                    int32_t eff_char;

                                    f = CHAR_NODE_font(v);
                                    eff_char = effective_char(true, f, CHAR_NODE_character(v));
                                    break_width[1] -= FONT_CHARACTER_WIDTH(f, eff_char);
                                } else
                                    switch (NODE_type(v)) {
                                    case LIGATURE_NODE:
                                    {
                                        int32_t eff_char;

                                        f = LIGATURE_NODE_lig_font(v);
                                        xtx_ligature_present = true;
                                        eff_char = effective_char(true, f, LIGATURE_NODE_lig_char(v));
                                        break_width[1] -= FONT_CHARACTER_WIDTH(f, eff_char);
                                        break;
                                    }

                                    case HLIST_NODE:
                                    case VLIST_NODE:
                                    case RULE_NODE:
                                    case KERN_NODE:
                                        break_width[1] -= BOX_width(v);
                                        break;

                                    case WHATSIT_NODE:
                                        if (NODE_subtype(v) == NATIVE_WORD_NODE
                                            || NODE_subtype(v) == NATIVE_WORD_NODE_AT
                                            || NODE_subtype(v) == GLYPH_NODE
                                            || NODE_subtype(v) == PIC_NODE
                                            || NODE_subtype(v) == PDF_NODE)
                                            break_width[1] -= BOX_width(v);
                                        else
                                            confusion("disc1a");
                                        break;

                                    default:
                                        confusion("disc1");
                                        break;
                                    }
                            }

                            /*871: "add the width of node s to break_width" */
                            while (s != TEX_NULL) {
                                if (is_char_node(s)) {
                                    int32_t eff_char;

                                    f = CHAR_NODE_font(s);
                                    eff_char = effective_char(true, f, CHAR_NODE_character(s));
                                    break_width[1] += FONT_CHARACTER_WIDTH(f, eff_char);;
                                } else
                                    switch (mem[s].b16.s1) {
                                    case LIGATURE_NODE:
                                    {
                                        int32_t eff_char;

                                        f = LIGATURE_NODE_lig_font(s);
                                        xtx_ligature_present = true;
                                        eff_char = effective_char(true, f, LIGATURE_NODE_lig_char(s));
                                        break_width[1] += FONT_CHARACTER_WIDTH(f, eff_char);
                                        break;
                                    }

                                    case HLIST_NODE:
                                    case VLIST_NODE:
                                    case RULE_NODE:
                                    case KERN_NODE:
                                        break_width[1] += BOX_width(s);
                                        break;

                                    case WHATSIT_NODE:
                                        if (NODE_subtype(s) == NATIVE_WORD_NODE
                                            || NODE_subtype(s) == NATIVE_WORD_NODE_AT
                                            || NODE_subtype(s) == GLYPH_NODE
                                            || NODE_subtype(s) == PIC_NODE
                                            || NODE_subtype(s) == PDF_NODE)
                                            break_width[1] += BOX_width(s);
                                        else
                                            confusion("disc2a");
                                        break;

                                    default:
                                        confusion("disc2");
                                        break;
                                    }

                                s = LLIST_link(s);
                            }

                            break_width[1] += disc_width;
                            if (DISCRETIONARY_NODE_post_break(cur_p) == TEX_NULL)
                                s = LLIST_link(v);
                        }
                    }

                    while (s != TEX_NULL) {
                        if (is_char_node(s))
                            goto done;

                        switch (NODE_type(s)) {
                        case GLUE_NODE:
                            v = GLUE_NODE_glue_ptr(s);
                            break_width[1] -= BOX_width(v);
                            break_width[2 + GLUE_SPEC_stretch_order(v)] -= GLUE_SPEC_stretch(v);
                            break_width[6] -= GLUE_SPEC_shrink(v);
                            break;

                        case PENALTY_NODE:
                            break;

                        case MATH_NODE:
                            break_width[1] -= BOX_width(s);
                            break;

                        case KERN_NODE:
                            if (NODE_subtype(s) != EXPLICIT)
                                goto done;
                            break_width[1] -= BOX_width(s);
                            break;

                        default:
                            goto done;
                            break;
                        }

                        s = LLIST_link(s);
                    }

                done:
                    ;
                }

                /*872: "Insert a delta node to prepare for breaks at cur_p" */
                if (NODE_type(prev_r) == DELTA_NODE) {
                    DELTA_NODE_dwidth(prev_r) += -cur_active_width[1] + break_width[1];
                    DELTA_NODE_dstretch0(prev_r) += -cur_active_width[2] + break_width[2];
                    DELTA_NODE_dstretch1(prev_r) += -cur_active_width[3] + break_width[3];
                    DELTA_NODE_dstretch2(prev_r) += -cur_active_width[4] + break_width[4];
                    DELTA_NODE_dstretch3(prev_r) += -cur_active_width[5] + break_width[5];
                    DELTA_NODE_dshrink(prev_r) += -cur_active_width[6] + break_width[6];
                } else if (prev_r == ACTIVE_LIST) {
                    active_width[1] = break_width[1];
                    active_width[2] = break_width[2];
                    active_width[3] = break_width[3];
                    active_width[4] = break_width[4];
                    active_width[5] = break_width[5];
                    active_width[6] = break_width[6];
                } else {
                    q = get_node(DELTA_NODE_SIZE);
                    LLIST_link(q) = r;
                    NODE_type(q) = DELTA_NODE;
                    NODE_subtype(q) = 0; /* this is unused */
                    DELTA_NODE_dwidth(q) = break_width[1] - cur_active_width[1];
                    DELTA_NODE_dstretch0(q) = break_width[2] - cur_active_width[2];
                    DELTA_NODE_dstretch1(q) = break_width[3] - cur_active_width[3];
                    DELTA_NODE_dstretch2(q) = break_width[4] - cur_active_width[4];
                    DELTA_NODE_dstretch3(q) = break_width[5] - cur_active_width[5];
                    DELTA_NODE_dshrink(q) = break_width[6] - cur_active_width[6];
                    LLIST_link(prev_r) = q;
                    prev_prev_r = prev_r;
                    prev_r = q;
                }

                /* ... resuming 865 ... */
                if (abs(INTPAR(adj_demerits)) >= MAX_HALFWORD - minimum_demerits)
                    minimum_demerits = AWFUL_BAD - 1;
                else
                    minimum_demerits = minimum_demerits + abs(INTPAR(adj_demerits));

                for (fit_class = VERY_LOOSE_FIT; fit_class <= TIGHT_FIT; fit_class++) {
                    if (minimal_demerits[fit_class] <= minimum_demerits) {
                        /*874: "Insert a new active node from best_place[fit_class] to cur_p" */
                        q = get_node(PASSIVE_NODE_SIZE);
                        LLIST_link(q) = passive;
                        passive = q;
                        PASSIVE_NODE_cur_break(q) = cur_p;
                        PASSIVE_NODE_prev_break(q) = best_place[fit_class];

                        q = get_node(active_node_size);
                        ACTIVE_NODE_break_node(q) = passive;
                        ACTIVE_NODE_line_number(q) = best_pl_line[fit_class] + 1;
                        ACTIVE_NODE_fitness(q) = fit_class;
                        NODE_type(q) = break_type;
                        ACTIVE_NODE_total_demerits(q) = minimal_demerits[fit_class];

                        if (do_last_line_fit) {     /*1639: */
                            ACTIVE_NODE_shortfall(q) = best_pl_short[fit_class];
                            ACTIVE_NODE_glue(q) = best_pl_glue[fit_class];
                        }

                        LLIST_link(q) = r;
                        LLIST_link(prev_r) = q;
                        prev_r = q;
                    }

                    minimal_demerits[fit_class] = MAX_HALFWORD;
                }

                minimum_demerits = MAX_HALFWORD;

                /*873: "Insert a delta node to prepare for the next active node" */
                if (r != LAST_ACTIVE) {
                    q = get_node(DELTA_NODE_SIZE);
                    LLIST_link(q) = r;
                    NODE_type(q) = DELTA_NODE;
                    NODE_subtype(q) = 0; /* subtype is not used */
                    DELTA_NODE_dwidth(q) = cur_active_width[1] - break_width[1];
                    DELTA_NODE_dstretch0(q) = cur_active_width[2] - break_width[2];
                    DELTA_NODE_dstretch1(q) = cur_active_width[3] - break_width[3];
                    DELTA_NODE_dstretch2(q) = cur_active_width[4] - break_width[4];
                    DELTA_NODE_dstretch3(q) = cur_active_width[5] - break_width[5];
                    DELTA_NODE_dshrink(q) = cur_active_width[6] - break_width[6];
                    LLIST_link(prev_r) = q;
                    prev_prev_r = prev_r;
                    prev_r = q;
                }
            }

            /* ... resuming 864 ... */
            if (r == LAST_ACTIVE)
                return;

            /*879: "Compute the new line width" */
            if (l > easy_line) {
                line_width = second_width;
                old_l = MAX_HALFWORD - 1;
            } else {
                old_l = l;

                if (l > last_special_line)
                    line_width = second_width;
                else if (LOCAL(par_shape) == TEX_NULL)
                    line_width = first_width;
                else
                    line_width = mem[LOCAL(par_shape) + 2 * l].b32.s1; /* this mem access is in the WEB */
            }
        }

        /*880: "Consider the demerits for a line from r to cur_p; deactivate
         * node r if it should no longer be active; then goto continue if a
         * line from r to cur_p is infeasible; otherwise record a new feasible
         * break" */

        /* Tectonic: if we got here, we must be "considering" a linebreak
         * at the very end of the paragraph. How amazing, it's a perfect fit!
         */

        if (semantic_pagination_enabled) {
            line_width = cur_active_width[1];
            artificial_demerits = true;
            shortfall = 0;
        } else {
            artificial_demerits = false;
            shortfall = line_width - cur_active_width[1];

            if (INTPAR(xetex_protrude_chars) > 1)
                shortfall = shortfall + total_pw(r, cur_p);
        }

        if (shortfall > 0) {
            /*881: "Set the value of b to the badness for stretching the line,
             * and compute the corresponding fit_class" */
            if (cur_active_width[3] != 0 || cur_active_width[4] != 0 || cur_active_width[5] != 0) {
                if (do_last_line_fit) {
                    if (cur_p == TEX_NULL) {
                        /*1634: "Perform computations for the last line and goto found" */
                        if (ACTIVE_NODE_shortfall(r) == 0 || ACTIVE_NODE_glue(r) <= 0)
                            goto not_found;

                        if (cur_active_width[3] != fill_width[0] || cur_active_width[4] != fill_width[1]
                            || cur_active_width[5] != fill_width[2])
                            goto not_found;

                        if (ACTIVE_NODE_shortfall(r) > 0)
                            g = cur_active_width[2];
                        else
                            g = cur_active_width[6];

                        if (g <= 0)
                            goto not_found;

                        arith_error = false;
                        g = fract(g, ACTIVE_NODE_shortfall(r), ACTIVE_NODE_glue(r), MAX_HALFWORD);
                        if (INTPAR(last_line_fit) < 1000)
                            g = fract(g, INTPAR(last_line_fit), 1000, MAX_HALFWORD);

                        if (arith_error) {
                            if (ACTIVE_NODE_shortfall(r) > 0)
                                g = MAX_HALFWORD;
                            else
                                g = -MAX_HALFWORD;
                        }

                        if (g > 0) {
                            /*1635: "Set the value of b to the badness of the
                            * last line for stretching, compute the
                            * corresponding fit_class, and goto found" */
                            if (g > shortfall)
                                g = shortfall;

                            if (g > 7230584L) { /* XXX: magic number in original WEB code */
                                if (cur_active_width[2] < 1663497L) { /* XXX: magic number in original WEB code */
                                    b = INF_BAD;
                                    fit_class = VERY_LOOSE_FIT;
                                    goto found;
                                }
                            }

                            b = badness(g, cur_active_width[2]);

                            if (b > 12) {
                                if (b > 99)
                                    fit_class = VERY_LOOSE_FIT;
                                else
                                    fit_class = LOOSE_FIT;
                            } else {
                                fit_class = DECENT_FIT;
                            }

                            goto found;
                        } else if (g < 0) {
                            /*1636: "Set the value of b to the badness of the
                            * last line for shrinking, compute the
                            * corresponding fit_class, and goto found" */
                            if (-g > cur_active_width[6])
                                g = -cur_active_width[6];

                            b = badness(-g, cur_active_width[6]);
                            if (b > 12) /* XXX hardcoded in WEB */
                                fit_class = TIGHT_FIT;
                            else
                                fit_class = DECENT_FIT;

                            goto found;
                        }

                    not_found:
                        ;
                    }

                    shortfall = 0;
                }

                b = 0;
                fit_class = DECENT_FIT;
            } else {
                if (shortfall > 7230584L) { /* XXX: magic number in original WEB code */
                    if (cur_active_width[2] < 1663497L) { /* XXX: magic number in original WEB code */
                        b = INF_BAD;
                        fit_class = VERY_LOOSE_FIT;
                        goto done1;
                    }
                }

                b = badness(shortfall, cur_active_width[2]);
                if (b > 12) {
                    if (b > 99)
                        fit_class = VERY_LOOSE_FIT;
                    else
                        fit_class = LOOSE_FIT;
                } else {
                    fit_class = DECENT_FIT;
                }

            done1:
                ;
            }
        } else {
            /*882: "Set the value of b to the badness for shrinking the line,
            * and compute the corresponding fit_class" */
            if (-shortfall > cur_active_width[6])
                b = (INF_BAD + 1);
            else
                b = badness(-shortfall, cur_active_width[6]);

            if (b > 12)
                fit_class = TIGHT_FIT;
            else
                fit_class = DECENT_FIT;
        }

        if (do_last_line_fit) {
            /*1637: "Adjust the additional data for last line" */
            if (cur_p == TEX_NULL)
                shortfall = 0;

            if (shortfall > 0)
                g = cur_active_width[2];
            else if (shortfall < 0)
                g = cur_active_width[6];
            else
                g = 0;
        }

    found:
        if (b > INF_BAD || pi == EJECT_PENALTY) {
            /*883: "Prepare to deactivate node r, and goto deactivate unless
             * there is a reason to consider lines of text from r to cur_p" */
            if (final_pass && minimum_demerits == AWFUL_BAD && LLIST_link(r) == LAST_ACTIVE && prev_r == ACTIVE_LIST)
                artificial_demerits = true;
            else if (b > threshold)
                goto deactivate;

            node_r_stays_active = false;
        } else {
            prev_r = r;
            if (b > threshold)
                continue;
            node_r_stays_active = true;
        }

        if (artificial_demerits) {
            d = 0;
        } else {
            /*888: "Compute the demerits, d, from r to cur_p" */
            d = INTPAR(line_penalty) + b;
            if (abs(d) >= 10000)
                d = 100000000L; /* algorithmic constant */
            else
                d = d * d;

            if (pi != 0) {
                if (pi > 0)
                    d = d + pi * pi;
                else if (pi > EJECT_PENALTY)
                    d = d - pi * pi;
            }

            if (break_type == HYPHENATED && NODE_type(r) == HYPHENATED) {
                if (cur_p != TEX_NULL)
                    d = d + INTPAR(double_hyphen_demerits);
                else
                    d = d + INTPAR(final_hyphen_demerits);
            }

            if (abs(fit_class - ACTIVE_NODE_fitness(r)) > 1)
                d = d + INTPAR(adj_demerits);
        }

        /* resuming 884: */
        d = d + ACTIVE_NODE_total_demerits(r);

        if (d <= minimal_demerits[fit_class]) {
            minimal_demerits[fit_class] = d;
            best_place[fit_class] = ACTIVE_NODE_break_node(r);
            best_pl_line[fit_class] = l;

            if (do_last_line_fit) { /*1638:*/
                best_pl_short[fit_class] = shortfall;
                best_pl_glue[fit_class] = g;
            }

            if (d < minimum_demerits)
                minimum_demerits = d;
        }

        if (node_r_stays_active)
            continue;

    deactivate:
        /*889: "Deactivate node r" */
        LLIST_link(prev_r) = LLIST_link(r);
        free_node(r, active_node_size);

        if (prev_r == ACTIVE_LIST) {
            /*890: "Update the active widths, since the first active node has been deleted" */
            r = LLIST_link(ACTIVE_LIST);

            if (NODE_type(r) == DELTA_NODE) {
                active_width[1] += DELTA_NODE_dwidth(r);
                active_width[2] += DELTA_NODE_dstretch0(r);
                active_width[3] += DELTA_NODE_dstretch1(r);
                active_width[4] += DELTA_NODE_dstretch2(r);
                active_width[5] += DELTA_NODE_dstretch3(r);
                active_width[6] += DELTA_NODE_dshrink(r);
                cur_active_width[1] = active_width[1];
                cur_active_width[2] = active_width[2];
                cur_active_width[3] = active_width[3];
                cur_active_width[4] = active_width[4];
                cur_active_width[5] = active_width[5];
                cur_active_width[6] = active_width[6];
                LLIST_link(ACTIVE_LIST) = LLIST_link(r);
                free_node(r, DELTA_NODE_SIZE);
            }
        } else if (NODE_type(prev_r) == DELTA_NODE) {
            r = LLIST_link(prev_r);

            if (r == LAST_ACTIVE) {
                cur_active_width[1] -= DELTA_NODE_dwidth(prev_r);
                cur_active_width[2] -= DELTA_NODE_dstretch0(prev_r);
                cur_active_width[3] -= DELTA_NODE_dstretch1(prev_r);
                cur_active_width[4] -= DELTA_NODE_dstretch2(prev_r);
                cur_active_width[5] -= DELTA_NODE_dstretch3(prev_r);
                cur_active_width[6] -= DELTA_NODE_dshrink(prev_r);
                LLIST_link(prev_prev_r) = LAST_ACTIVE;
                free_node(prev_r, DELTA_NODE_SIZE);
                prev_r = prev_prev_r;
            } else if (NODE_type(r) == DELTA_NODE) {
                cur_active_width[1] += DELTA_NODE_dwidth(r);
                cur_active_width[2] += DELTA_NODE_dstretch0(r);
                cur_active_width[3] += DELTA_NODE_dstretch1(r);
                cur_active_width[4] += DELTA_NODE_dstretch2(r);
                cur_active_width[5] += DELTA_NODE_dstretch3(r);
                cur_active_width[6] += DELTA_NODE_dshrink(r);
                DELTA_NODE_dwidth(prev_r) += DELTA_NODE_dwidth(r);
                DELTA_NODE_dstretch0(prev_r) += DELTA_NODE_dstretch0(r);
                DELTA_NODE_dstretch1(prev_r) += DELTA_NODE_dstretch1(r);
                DELTA_NODE_dstretch2(prev_r) += DELTA_NODE_dstretch2(r);
                DELTA_NODE_dstretch3(prev_r) += DELTA_NODE_dstretch3(r);
                DELTA_NODE_dshrink(prev_r) += DELTA_NODE_dshrink(r);
                LLIST_link(prev_r) = LLIST_link(r);
                free_node(r, DELTA_NODE_SIZE);
            }
        }
    }
}


static void
hyphenate(void)
{
    short /*hyphenatable_length_limit 2 */ i, j, l;
    int32_t q, r, s;
    int32_t bchar;
    int32_t major_tail, minor_tail;
    UnicodeScalar c = 0;
    short /*hyphenatable_length_limit */ c_loc;
    int32_t r_count;
    int32_t hyf_node;
    trie_pointer z;
    int32_t v;
    hyph_pointer h;
    str_number k;
    pool_pointer u;

    {
        register int32_t for_end;
        j = 0;
        for_end = hn;
        if (j <= for_end)
            do
                hyf[j] = 0;
            while (j++ < for_end);
    }
    h = hc[1];
    hn++;
    hc[hn] = cur_lang;
    {
        register int32_t for_end;
        j = 2;
        for_end = hn;
        if (j <= for_end)
            do
                h = (h + h + hc[j]) % HYPH_PRIME;
            while (j++ < for_end);
    }
    while (true) {

        k = hyph_word[h];
        if (k == 0)
            goto not_found;
        if (length(k) == hn) {
            j = 1;
            u = str_start[(k) - 65536L];
            do {
                if (str_pool[u] != hc[j])
                    goto done;
                j++;
                u++;
            } while (!(j > hn));
            s = hyph_list[h];
            while (s != TEX_NULL) {

                hyf[mem[s].b32.s0] = 1;
                s = LLIST_link(s);
            }
            hn--;
            goto found;
        } /*:966 */
    done:
        h = hyph_link[h];
        if (h == 0)
            goto not_found;
        h--;
    }
not_found:
    hn--;
    if (trie_trc[cur_lang + 1] != cur_lang)
        return;
    hc[0] = 0;
    hc[hn + 1] = 0;
    hc[hn + 2] = max_hyph_char;
    {
        register int32_t for_end;
        j = 0;
        for_end = hn - r_hyf + 1;
        if (j <= for_end)
            do {
                z = trie_trl[cur_lang + 1] + hc[j];
                l = j;
                while (hc[l] == trie_trc[z]) {

                    if (trie_tro[z] != MIN_TRIE_OP) {   /*959: */
                        v = trie_tro[z];
                        do {
                            v = v + op_start[cur_lang];
                            i = l - hyf_distance[v];
                            if (hyf_num[v] > hyf[i])
                                hyf[i] = hyf_num[v];
                            v = hyf_next[v];
                        } while (!(v == MIN_TRIE_OP));
                    }
                    l++;
                    z = trie_trl[z] + hc[l];
                }
            }
            while (j++ < for_end);
    }
 found:
    {
        register int32_t for_end;
        j = 0;
        for_end = l_hyf - 1;
        if (j <= for_end)
            do
                hyf[j] = 0;
            while (j++ < for_end);
    }
    {
        register int32_t for_end;
        j = 0;
        for_end = r_hyf - 1;
        if (j <= for_end)
            do
                hyf[hn - j] = 0 /*:958 */ ;
            while (j++ < for_end);
    }
    {
        register int32_t for_end;
        j = l_hyf;
        for_end = hn - r_hyf;
        if (j <= for_end)
            do
                if (odd(hyf[j]))
                    goto found1;
            while (j++ < for_end) ;
    }
    return;

found1:
    if ((((ha) != TEX_NULL && (!(is_char_node(ha))) && (NODE_type(ha) == WHATSIT_NODE)
          && ((mem[ha].b16.s0 == NATIVE_WORD_NODE) || (mem[ha].b16.s0 == NATIVE_WORD_NODE_AT))))) {
        s = cur_p;
        while (mem[s].b32.s1 != ha)
            s = LLIST_link(s);
        hyphen_passed = 0;
        {
            register int32_t for_end;
            j = l_hyf;
            for_end = hn - r_hyf;
            if (j <= for_end)
                do {
                    if (odd(hyf[j])) {
                        q = new_native_word_node(hf, j - hyphen_passed);
                        mem[q].b16.s0 = mem[ha].b16.s0;
                        {
                            register int32_t for_end;
                            i = 0;
                            for_end = j - hyphen_passed - 1;
                            if (i <= for_end)
                                do
                                    NATIVE_NODE_text(q)[i] = NATIVE_NODE_text(ha)[i + hyphen_passed];
                                while (i++ < for_end);
                        }
                        set_native_metrics(q, (INTPAR(xetex_use_glyph_metrics) > 0));
                        mem[s].b32.s1 = q;
                        s = q;
                        q = new_disc();
                        mem[q + 1].b32.s0 = new_native_character(hf, hyf_char);
                        mem[s].b32.s1 = q;
                        s = q;
                        hyphen_passed = j;
                    }
                }
                while (j++ < for_end);
        }
        hn = mem[ha + 4].b16.s1;
        q = new_native_word_node(hf, hn - hyphen_passed);
        mem[q].b16.s0 = mem[ha].b16.s0;
        {
            register int32_t for_end;
            i = 0;
            for_end = hn - hyphen_passed - 1;
            if (i <= for_end)
                do
                    NATIVE_NODE_text(q)[i] = NATIVE_NODE_text(ha)[i + hyphen_passed];
                while (i++ < for_end);
        }
        set_native_metrics(q, (INTPAR(xetex_use_glyph_metrics) > 0));
        mem[s].b32.s1 = q;
        s = q;
        q = mem[ha].b32.s1;
        mem[s].b32.s1 = q;
        mem[ha].b32.s1 = TEX_NULL;
        flush_node_list(ha);
    } else {

        q = mem[hb].b32.s1;
        mem[hb].b32.s1 = TEX_NULL;
        r = mem[ha].b32.s1;
        mem[ha].b32.s1 = TEX_NULL;
        bchar = hyf_bchar;
        if ((is_char_node(ha))) {

            if (mem[ha].b16.s1 != hf)
                goto found2;
            else {

                init_list = ha;
                init_lig = false;
                hu[0] = mem[ha].b16.s0;
            }
        } else if (NODE_type(ha) == LIGATURE_NODE) {

            if (mem[ha + 1].b16.s1 != hf)
                goto found2;
            else {

                init_list = mem[ha + 1].b32.s1;
                init_lig = true;
                init_lft = (mem[ha].b16.s0 > 1);
                hu[0] = mem[ha + 1].b16.s0;
                if (init_list == TEX_NULL) {

                    if (init_lft) {
                        hu[0] = max_hyph_char;
                        init_lig = false;
                    }
                }
                free_node(ha, SMALL_NODE_SIZE);
            }
        } else {

            if (!(is_char_node(r))) {

                if (NODE_type(r) == LIGATURE_NODE) {

                    if (mem[r].b16.s0 > 1)
                        goto found2;
                }
            }
            j = 1;
            s = ha;
            init_list = TEX_NULL;
            goto common_ending;
        }
        s = cur_p;
        while (mem[s].b32.s1 != ha)
            s = LLIST_link(s);
        j = 0;
        goto common_ending;
    found2:
        s = ha;
        j = 0;
        hu[0] = max_hyph_char;
        init_lig = false;
        init_list = TEX_NULL;

    common_ending:
        flush_node_list(r);
        do {
            l = j;
            j = reconstitute(j, hn, bchar, hyf_char) + 1;
            if (hyphen_passed == 0) {
                mem[s].b32.s1 = mem[HOLD_HEAD].b32.s1;
                while (mem[s].b32.s1 > TEX_NULL)
                    s = LLIST_link(s);
                if (odd(hyf[j - 1])) {
                    l = j;
                    hyphen_passed = j - 1;
                    mem[HOLD_HEAD].b32.s1 = TEX_NULL;
                }
            }
            if (hyphen_passed > 0) /*949: */
                do {
                    r = get_node(SMALL_NODE_SIZE);
                    mem[r].b32.s1 = mem[HOLD_HEAD].b32.s1;
                    NODE_type(r) = DISC_NODE;
                    major_tail = r;
                    r_count = 0;
                    while (mem[major_tail].b32.s1 > TEX_NULL) {

                        major_tail = LLIST_link(major_tail);
                        r_count++;
                    }
                    i = hyphen_passed;
                    hyf[i] = 0;
                    minor_tail = TEX_NULL;
                    mem[r + 1].b32.s0 = TEX_NULL;
                    hyf_node = new_character(hf, hyf_char);
                    if (hyf_node != TEX_NULL) {
                        i++;
                        c = hu[i];
                        hu[i] = hyf_char;
                        {
                            mem[hyf_node].b32.s1 = avail;
                            avail = hyf_node;
                        }
                    }
                    while (l <= i) {

                        l = reconstitute(l, i, font_bchar[hf], TOO_BIG_CHAR) + 1;
                        if (mem[HOLD_HEAD].b32.s1 > TEX_NULL) {
                            if (minor_tail == TEX_NULL)
                                mem[r + 1].b32.s0 = mem[HOLD_HEAD].b32.s1;
                            else
                                mem[minor_tail].b32.s1 = mem[HOLD_HEAD].b32.s1;
                            minor_tail = mem[HOLD_HEAD].b32.s1;
                            while (mem[minor_tail].b32.s1 > TEX_NULL)
                                minor_tail = LLIST_link(minor_tail);
                        }
                    }
                    if (hyf_node != TEX_NULL) {
                        hu[i] = c;
                        l = i;
                        i--;
                    }
                    minor_tail = TEX_NULL;
                    mem[r + 1].b32.s1 = TEX_NULL;
                    c_loc = 0;
                    if (bchar_label[hf] != NON_ADDRESS) {
                        l--;
                        c = hu[l];
                        c_loc = l;
                        hu[l] = max_hyph_char;
                    }
                    while (l < j) {

                        do {
                            l = reconstitute(l, hn, bchar, TOO_BIG_CHAR) + 1;
                            if (c_loc > 0) {
                                hu[c_loc] = c;
                                c_loc = 0;
                            }
                            if (mem[HOLD_HEAD].b32.s1 > TEX_NULL) {
                                if (minor_tail == TEX_NULL)
                                    mem[r + 1].b32.s1 = mem[HOLD_HEAD].b32.s1;
                                else
                                    mem[minor_tail].b32.s1 = mem[HOLD_HEAD].b32.s1;
                                minor_tail = mem[HOLD_HEAD].b32.s1;
                                while (mem[minor_tail].b32.s1 > TEX_NULL)
                                    minor_tail = LLIST_link(minor_tail);
                            }
                        } while (!(l >= j));
                        while (l > j) { /*952: */

                            j = reconstitute(j, hn, bchar, TOO_BIG_CHAR) + 1;
                            mem[major_tail].b32.s1 = mem[HOLD_HEAD].b32.s1;
                            while (mem[major_tail].b32.s1 > TEX_NULL) {

                                major_tail = LLIST_link(major_tail);
                                r_count++;
                            }
                        }
                    }
                    if (r_count > 127) {
                        mem[s].b32.s1 = mem[r].b32.s1;
                        mem[r].b32.s1 = TEX_NULL;
                        flush_node_list(r);
                    } else {

                        mem[s].b32.s1 = r;
                        mem[r].b16.s0 = r_count;
                    }
                    s = /*:953 */ major_tail;
                    hyphen_passed = j - 1;
                    mem[HOLD_HEAD].b32.s1 = TEX_NULL;
                } while (!(!odd(hyf[j - 1]) /*:949 */ ));
        } while (!(j > hn));
        mem[s].b32.s1 = /*:948 */ q;
        flush_list(init_list);
    }
}


static int32_t
finite_shrink(int32_t p)
{
    int32_t q;
    if (no_shrink_error_yet) {
        no_shrink_error_yet = false;
        error_here_with_diagnostic("Infinite glue shrinkage found in a paragraph");
        capture_to_diagnostic(NULL);
        {
            help_ptr = 5;
            help_line[4] = "The paragraph just ended includes some glue that has";
            help_line[3] = "infinite shrinkability, e.g., `\\hskip 0pt minus 1fil'.";
            help_line[2] = "Such glue doesn't belong there---it allows a paragraph";
            help_line[1] = "of any length to fit on one line. But it's safe to proceed,";
            help_line[0] = "since the offensive shrinkability has been made finite.";
        }
        error();
    }
    q = new_spec(p);
    GLUE_SPEC_shrink_order(q) = NORMAL;
    delete_glue_ref(p);
    return q;
}


static small_number
reconstitute(small_number j, small_number n, int32_t bchar, int32_t hchar)
{
    int32_t p;
    int32_t t;
    b16x4 q;
    int32_t cur_rh;
    int32_t test_char;
    scaled_t w;
    font_index k;
    hyphen_passed = 0;
    t = HOLD_HEAD;
    w = 0;
    mem[HOLD_HEAD].b32.s1 = TEX_NULL;
    cur_l = hu[j];
    cur_q = t;
    if (j == 0) {
        ligature_present = init_lig;
        p = init_list;
        if (ligature_present)
            lft_hit = init_lft;
        while (p > TEX_NULL) {

            {
                mem[t].b32.s1 = get_avail();
                t = LLIST_link(t);
                mem[t].b16.s1 = hf;
                mem[t].b16.s0 = mem[p].b16.s0;
            }
            p = LLIST_link(p);
        }
    } else if (cur_l < TOO_BIG_CHAR) {
        mem[t].b32.s1 = get_avail();
        t = LLIST_link(t);
        mem[t].b16.s1 = hf;
        mem[t].b16.s0 = cur_l;
    }
    lig_stack = TEX_NULL;
    {
        if (j < n)
            cur_r = hu[j + 1];
        else
            cur_r = bchar;
        if (odd(hyf[j]))
            cur_rh = hchar;
        else
            cur_rh = TOO_BIG_CHAR;
    }
continue_:
    if (cur_l == TOO_BIG_CHAR) {
        k = bchar_label[hf];
        if (k == NON_ADDRESS)
            goto done;
        else
            q = font_info[k].b16;
    } else {

        q = FONT_CHARACTER_INFO(hf, effective_char(true, hf, cur_l));
        if (((q.s1) % 4) != LIG_TAG)
            goto done;
        k = lig_kern_base[hf] + q.s0;
        q = font_info[k].b16;
        if (q.s3 > 128) {
            k = lig_kern_base[hf] + 256 * q.s1 + q.s0 + 32768L - 256 * (128);
            q = font_info[k].b16;
        }
    }
    if (cur_rh < TOO_BIG_CHAR)
        test_char = cur_rh;
    else
        test_char = cur_r;
    while (true) {

        if (q.s2 == test_char) {

            if (q.s3 <= 128) {

                if (cur_rh < TOO_BIG_CHAR) {
                    hyphen_passed = j;
                    hchar = TOO_BIG_CHAR;
                    cur_rh = TOO_BIG_CHAR;
                    goto continue_;
                } else {

                    if (hchar < TOO_BIG_CHAR) {

                        if (odd(hyf[j])) {
                            hyphen_passed = j;
                            hchar = TOO_BIG_CHAR;
                        }
                    }
                    if (q.s1 < 128) {   /*946: */
                        if (cur_l == TOO_BIG_CHAR)
                            lft_hit = true;
                        if (j == n) {

                            if (lig_stack == TEX_NULL)
                                rt_hit = true;
                        }
                        switch (q.s1) {
                        case 1:
                        case 5:
                            {
                                cur_l = q.s0;
                                ligature_present = true;
                            }
                            break;
                        case 2:
                        case 6:
                            {
                                cur_r = q.s0;
                                if (lig_stack > TEX_NULL)
                                    mem[lig_stack].b16.s0 = cur_r;
                                else {

                                    lig_stack = new_lig_item(cur_r);
                                    if (j == n)
                                        bchar = TOO_BIG_CHAR;
                                    else {

                                        p = get_avail();
                                        mem[lig_stack + 1].b32.s1 = p;
                                        mem[p].b16.s0 = hu[j + 1];
                                        mem[p].b16.s1 = hf;
                                    }
                                }
                            }
                            break;
                        case 3:
                            {
                                cur_r = q.s0;
                                p = lig_stack;
                                lig_stack = new_lig_item(cur_r);
                                mem[lig_stack].b32.s1 = p;
                            }
                            break;
                        case 7:
                        case 11:
                            {
                                if (ligature_present) {
                                    p = new_ligature(hf, cur_l, mem[cur_q].b32.s1);
                                    if (lft_hit) {
                                        mem[p].b16.s0 = 2;
                                        lft_hit = false;
                                    }
                                    if (false) {

                                        if (lig_stack == TEX_NULL) {
                                            mem[p].b16.s0++;
                                            rt_hit = false;
                                        }
                                    }
                                    mem[cur_q].b32.s1 = p;
                                    t = p;
                                    ligature_present = false;
                                }
                                cur_q = t;
                                cur_l = q.s0;
                                ligature_present = true;
                            }
                            break;
                        default:
                            {
                                cur_l = q.s0;
                                ligature_present = true;
                                if (lig_stack > TEX_NULL) {
                                    if (mem[lig_stack + 1].b32.s1 > TEX_NULL) {
                                        mem[t].b32.s1 = mem[lig_stack + 1].b32.s1;
                                        t = LLIST_link(t);
                                        j++;
                                    }
                                    p = lig_stack;
                                    lig_stack = mem[p].b32.s1;
                                    free_node(p, SMALL_NODE_SIZE);
                                    if (lig_stack == TEX_NULL) {
                                        if (j < n)
                                            cur_r = hu[j + 1];
                                        else
                                            cur_r = bchar;
                                        if (odd(hyf[j]))
                                            cur_rh = hchar;
                                        else
                                            cur_rh = TOO_BIG_CHAR;
                                    } else
                                        cur_r = mem[lig_stack].b16.s0;
                                } else if (j == n)
                                    goto done;
                                else {

                                    {
                                        mem[t].b32.s1 = get_avail();
                                        t = LLIST_link(t);
                                        mem[t].b16.s1 = hf;
                                        mem[t].b16.s0 = cur_r;
                                    }
                                    j++;
                                    {
                                        if (j < n)
                                            cur_r = hu[j + 1];
                                        else
                                            cur_r = bchar;
                                        if (odd(hyf[j]))
                                            cur_rh = hchar;
                                        else
                                            cur_rh = TOO_BIG_CHAR;
                                    }
                                }
                            }
                            break;
                        }
                        if (q.s1 > 4) {

                            if (q.s1 != 7)
                                goto done;
                        }
                        goto continue_;
                    }
                    w = font_info[kern_base[hf] + 256 * q.s1 + q.s0].b32.s1;
                    goto done;
                }
            }
        }
        if (q.s3 >= 128) {

            if (cur_rh == TOO_BIG_CHAR)
                goto done;
            else {

                cur_rh = TOO_BIG_CHAR;
                goto continue_;
            }
        }
        k = k + q.s3 + 1;
        q = font_info[k].b16;
    } /*:944*/
done:
    if (ligature_present) {
        p = new_ligature(hf, cur_l, mem[cur_q].b32.s1);
        if (lft_hit) {
            mem[p].b16.s0 = 2;
            lft_hit = false;
        }
        if (rt_hit) {

            if (lig_stack == TEX_NULL) {
                mem[p].b16.s0++;
                rt_hit = false;
            }
        }
        mem[cur_q].b32.s1 = p;
        t = p;
        ligature_present = false;
    }
    if (w != 0) {
        mem[t].b32.s1 = new_kern(w);
        t = LLIST_link(t);
        w = 0;
        mem[t + 2].b32.s0 = 0;
    }
    if (lig_stack > TEX_NULL) {
        cur_q = t;
        cur_l = mem[lig_stack].b16.s0;
        ligature_present = true;
        {
            if (mem[lig_stack + 1].b32.s1 > TEX_NULL) {
                mem[t].b32.s1 = mem[lig_stack + 1].b32.s1;
                t = LLIST_link(t);
                j++;
            }
            p = lig_stack;
            lig_stack = mem[p].b32.s1;
            free_node(p, SMALL_NODE_SIZE);
            if (lig_stack == TEX_NULL) {
                if (j < n)
                    cur_r = hu[j + 1];
                else
                    cur_r = bchar;
                if (odd(hyf[j]))
                    cur_rh = hchar;
                else
                    cur_rh = TOO_BIG_CHAR;
            } else
                cur_r = mem[lig_stack].b16.s0;
        }
        goto continue_;
    }
    return j;
}


static scaled_t
total_pw(int32_t q, int32_t p)
{
    int32_t l, r;
    int32_t n;
    if (mem[q + 1].b32.s1 == TEX_NULL)
        l = first_p;
    else
        l = mem[mem[q + 1].b32.s1 + 1].b32.s1;
    r = prev_rightmost(global_prev_p, p);
    if ((p != TEX_NULL) && (NODE_type(p) == DISC_NODE) && (mem[p + 1].b32.s0 != TEX_NULL)) {
        r = mem[p + 1].b32.s0;
        while (mem[r].b32.s1 != TEX_NULL)
            r = LLIST_link(r);
    } else
        r = find_protchar_right(l, r);
    if ((l != TEX_NULL) && (NODE_type(l) == DISC_NODE)) {
        if (mem[l + 1].b32.s1 != TEX_NULL) {
            l = mem[l + 1].b32.s1;
            goto done;
        } else {

            n = mem[l].b16.s0;
            l = LLIST_link(l);
            while (n > 0) {

                if (mem[l].b32.s1 != TEX_NULL)
                    l = LLIST_link(l);
                n--;
            }
        }
    }
    l = find_protchar_left(l, true);

done:
    return char_pw(l, 0) + char_pw(r, 1);
}


static int32_t
find_protchar_left(int32_t l, bool d)
{
    int32_t t;
    bool run;
    if ((mem[l].b32.s1 != TEX_NULL) && (NODE_type(l) == HLIST_NODE) && (mem[l + 1].b32.s1 == 0)
        && (mem[l + 3].b32.s1 == 0) && (mem[l + 2].b32.s1 == 0) && (mem[l + 5].b32.s1 == TEX_NULL))
        l = LLIST_link(l);
    else if (d)
        while ((mem[l].b32.s1 != TEX_NULL) && (!((is_char_node(l)) || (is_non_discardable_node(l)))))
            l = LLIST_link(l);
    hlist_stack_level = 0;
    run = true;
    do {
        t = l;
        while (run && (NODE_type(l) == HLIST_NODE) && (mem[l + 5].b32.s1 != TEX_NULL)) {

            push_node(l);
            l = mem[l + 5].b32.s1;
        }
        while (run
               && (!(is_char_node(l))
                   && ((NODE_type(l) == INS_NODE) || (NODE_type(l) == MARK_NODE)
                       || (NODE_type(l) == ADJUST_NODE) || (NODE_type(l) == PENALTY_NODE)
                       || ((NODE_type(l) == DISC_NODE) && (mem[l + 1].b32.s0 == TEX_NULL)
                           && (mem[l + 1].b32.s1 == TEX_NULL) && (mem[l].b16.s0 == 0))
                       || ((NODE_type(l) == MATH_NODE) && (mem[l + 1].b32.s1 == 0))
                       || ((NODE_type(l) == KERN_NODE)
                           && ((mem[l + 1].b32.s1 == 0) || (mem[l].b16.s0 == NORMAL)))
                       || ((NODE_type(l) == GLUE_NODE) && (mem[l + 1].b32.s0 == 0))
                       || ((NODE_type(l) == HLIST_NODE) && (mem[l + 1].b32.s1 == 0) && (mem[l + 3].b32.s1 == 0)
                           && (mem[l + 2].b32.s1 == 0) && (mem[l + 5].b32.s1 == TEX_NULL))))) {

            while ((mem[l].b32.s1 == TEX_NULL) && (hlist_stack_level > 0)) {

                l = pop_node();
            }
            if (mem[l].b32.s1 != TEX_NULL)
                l = LLIST_link(l);
            else if (hlist_stack_level == 0)
                run = false;
        }
    } while (!(t == l));
    return l;
}


static int32_t
find_protchar_right(int32_t l, int32_t r)
{
    int32_t t;
    bool run;
    if (r == TEX_NULL)
        return TEX_NULL;
    hlist_stack_level = 0;
    run = true;
    do {
        t = r;
        while (run && (NODE_type(r) == HLIST_NODE) && (mem[r + 5].b32.s1 != TEX_NULL)) {

            push_node(l);
            push_node(r);
            l = mem[r + 5].b32.s1;
            r = l;
            while (mem[r].b32.s1 != TEX_NULL)
                r = LLIST_link(r);
        }
        while (run
               && (!(is_char_node(r))
                   && ((NODE_type(r) == INS_NODE) || (NODE_type(r) == MARK_NODE)
                       || (NODE_type(r) == ADJUST_NODE) || (NODE_type(r) == PENALTY_NODE)
                       || ((NODE_type(r) == DISC_NODE) && (mem[r + 1].b32.s0 == TEX_NULL)
                           && (mem[r + 1].b32.s1 == TEX_NULL) && (mem[r].b16.s0 == 0))
                       || ((NODE_type(r) == MATH_NODE) && (mem[r + 1].b32.s1 == 0))
                       || ((NODE_type(r) == KERN_NODE)
                           && ((mem[r + 1].b32.s1 == 0) || (mem[r].b16.s0 == NORMAL)))
                       || ((NODE_type(r) == GLUE_NODE) && (mem[r + 1].b32.s0 == 0))
                       || ((NODE_type(r) == HLIST_NODE) && (mem[r + 1].b32.s1 == 0) && (mem[r + 3].b32.s1 == 0)
                           && (mem[r + 2].b32.s1 == 0) && (mem[r + 5].b32.s1 == TEX_NULL))))) {

            while ((r == l) && (hlist_stack_level > 0)) {

                r = pop_node();
                l = pop_node();
            }
            if ((r != l) && (r != TEX_NULL))
                r = prev_rightmost(l, r);
            else if ((r == l) && (hlist_stack_level == 0))
                run = false;
        }
    } while (!(t == r));
    return r;
}


static void
push_node(int32_t p)
{
    if (hlist_stack_level > MAX_HLIST_STACK)
        pdf_error("push_node", "stack overflow");
    hlist_stack[hlist_stack_level] = p;
    hlist_stack_level = hlist_stack_level + 1;
}


static int32_t
pop_node(void)
{
    hlist_stack_level = hlist_stack_level - 1;
    if (hlist_stack_level < 0)
        pdf_error("pop_node", "stack underflow (internal error)");
    return hlist_stack[hlist_stack_level];
}
