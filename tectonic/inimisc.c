/* tectonic/inimisc.c -- random routines originally in xetexini.c
   Copyright 2016-2017 The Tectonic Project
   Licensed under the MIT License.
*/

#include "tectonic.h"
#include "internals.h"
#include "xetexd.h"
#include "core-bridge.h"


static void post_line_break(bool d);

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
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
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

    if (cur_list.tail >= hi_mem_min) { /* is_char_node */
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

    if (mem[GLUEPAR(left_skip)].b16.s0 != NORMAL && mem[GLUEPAR(left_skip) + 3].b32.s1 != 0)
        GLUEPAR(left_skip) = finite_shrink(GLUEPAR(left_skip));

    if (mem[GLUEPAR(right_skip)].b16.s0 != NORMAL && mem[GLUEPAR(right_skip) + 3].b32.s1 != 0)
        GLUEPAR(right_skip) = finite_shrink(GLUEPAR(right_skip));

    q = GLUEPAR(left_skip);
    r = GLUEPAR(right_skip);

    background[1] = mem[q + 1].b32.s1 + mem[r + 1].b32.s1;
    background[2] = 0;
    background[3] = 0;
    background[4] = 0;
    background[5] = 0;
    background[2 + mem[q].b16.s1] = mem[q + 2].b32.s1;
    background[2 + mem[r].b16.s1] = background[2 + mem[r].b16.s1] + mem[r + 2].b32.s1;
    background[6] = mem[q + 3].b32.s1 + mem[r + 3].b32.s1;

    /* 1631: "check for special treatment of last line of paragraph" (\lastlinefit > 0) */

    do_last_line_fit = false;
    active_node_size = ACTIVE_NODE_SIZE_NORMAL;

    if (INTPAR(last_line_fit) > 0) {
        q = mem[last_line_fill + 1].b32.s0;

        if (mem[q + 2].b32.s1 > 0 && mem[q].b16.s1 > NORMAL) {
            if (background[3] == 0 && background[4] == 0 && background[5] == 0) {
                do_last_line_fit = true;
                active_node_size = ACTIVE_NODE_SIZE_EXTENDED;
                fill_width[0] = 0;
                fill_width[1] = 0;
                fill_width[2] = 0;
                fill_width[mem[q].b16.s1 - 1] = mem[q + 2].b32.s1;
            }
        }
    }

    minimum_demerits = AWFUL_BAD; /*863:*/
    minimal_demerits[TIGHT_FIT] = AWFUL_BAD;
    minimal_demerits[DECENT_FIT] = AWFUL_BAD;
    minimal_demerits[LOOSE_FIT] = AWFUL_BAD;
    minimal_demerits[VERY_LOOSE_FIT] = AWFUL_BAD;

    /* Prep relating to par_shape (877) */

    if (LOCAL(par_shape) == MIN_HALFWORD) {
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
        last_special_line = mem[LOCAL(par_shape)].b32.s0 - 1;
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

        q = get_node(active_node_size);
        mem[q].b16.s1 = UNHYPHENATED;
        mem[q].b16.s0 = DECENT_FIT;
        mem[q].b32.s1 = ACTIVE_LIST;
        mem[q + 1].b32.s1 = MIN_HALFWORD;
        mem[q + 1].b32.s0 = cur_list.prev_graf + 1;
        mem[q + 2].b32.s1 = 0;
        mem[ACTIVE_LIST].b32.s1 = q;

        if (do_last_line_fit) { /*1633:*/
            mem[q + 3].b32.s1 = 0;
            mem[q + 4].b32.s1 = 0;
        }

        active_width[1] = background[1];
        active_width[2] = background[2];
        active_width[3] = background[3];
        active_width[4] = background[4];
        active_width[5] = background[5];
        active_width[6] = background[6];
        passive = MIN_HALFWORD;
        printed_node = TEMP_HEAD;
        pass_number = 0;
        font_in_short_display = 0; /*:893*/
        cur_p = mem[TEMP_HEAD].b32.s1;
        auto_breaking = true;

        prev_p = global_prev_p = cur_p;
        first_p = cur_p;

        while (cur_p != MIN_HALFWORD && mem[ACTIVE_LIST].b32.s1 != ACTIVE_LIST) { /*895:*/
            if (cur_p >= hi_mem_min) { /*896:*/
                prev_p = global_prev_p = cur_p;

                do {
                    int32_t eff_char;
                    uint16_t char_info;

                    f = mem[cur_p].b16.s1;
                    eff_char = effective_char(true, f, mem[cur_p].b16.s0);
                    char_info = font_info[char_base[f] + eff_char].b16.s3;
                    active_width[1] += font_info[width_base[f] + char_info].b32.s1;
                    cur_p = mem[cur_p].b32.s1;
                } while (cur_p >= hi_mem_min);
            }

            switch (mem[cur_p].b16.s1) {
            case HLIST_NODE:
            case VLIST_NODE:
            case RULE_NODE:
                active_width[1] += mem[cur_p + 1].b32.s1;
                break;

            case WHATSIT_NODE:
                if (mem[cur_p].b16.s0 == LANGUAGE_NODE) {
                    cur_lang = mem[cur_p + 1].b32.s1;
                    l_hyf = mem[cur_p + 1].b16.s1;
                    r_hyf = mem[cur_p + 1].b16.s0;
                    if (trie_trc[hyph_start + cur_lang] != cur_lang)
                        hyph_index = 0;
                    else
                        hyph_index = trie_trl[hyph_start + cur_lang];
                } else if (mem[cur_p].b16.s0 == NATIVE_WORD_NODE
                           || mem[cur_p].b16.s0 == NATIVE_WORD_NODE_AT
                           || mem[cur_p].b16.s0 == GLYPH_NODE
                           || mem[cur_p].b16.s0 == PIC_NODE
                           || mem[cur_p].b16.s0 == PDF_NODE) {
                    active_width[1] += mem[cur_p + 1].b32.s1;
                }
                break;

            case GLUE_NODE:
                if (auto_breaking) {
                    if (prev_p >= hi_mem_min)
                        try_break(0, UNHYPHENATED);
                    else if (NODE_type(prev_p) < MATH_NODE)
                        try_break(0, UNHYPHENATED);
                    else if (NODE_type(prev_p) == KERN_NODE && mem[prev_p].b16.s0 != EXPLICIT)
                        try_break(0, UNHYPHENATED);
                }

                if (mem[mem[cur_p + 1].b32.s0].b16.s0 != NORMAL && mem[mem[cur_p + 1].b32.s0 + 3].b32.s1 != 0)
                    mem[cur_p + 1].b32.s0 = finite_shrink(mem[cur_p + 1].b32.s0);

                q = mem[cur_p + 1].b32.s0;
                active_width[1] = active_width[1] + mem[q + 1].b32.s1;
                active_width[2 + mem[q].b16.s1] = active_width[2 + mem[q].b16.s1] + mem[q + 2].b32.s1;
                active_width[6] = active_width[6] + mem[q + 3].b32.s1; /*:897*/

                if (second_pass && auto_breaking) { /*924:*/
                    prev_s = cur_p;
                    s = mem[prev_s].b32.s1;

                    if (s != MIN_HALFWORD) {
                        while (true) {
                            if (s >= hi_mem_min) {
                                c = mem[s].b16.s0;
                                hf = mem[s].b16.s1;
                            } else if (NODE_type(s) == LIGATURE_NODE) {
                                if (mem[s + 1].b32.s1 == MIN_HALFWORD)
                                    goto _continue;

                                q = mem[s + 1].b32.s1;
                                c = mem[q].b16.s0;
                                hf = mem[q].b16.s1;
                            } else if (NODE_type(s) == KERN_NODE && mem[s].b16.s0 == NORMAL) {
                                goto _continue;
                            } else if (NODE_type(s) == MATH_NODE && mem[s].b16.s0 >= L_CODE) {
                                goto _continue;
                            } else if (NODE_type(s) == WHATSIT_NODE) {
                                if (mem[s].b16.s0 == NATIVE_WORD_NODE || mem[s].b16.s0 == NATIVE_WORD_NODE_AT) {
                                    for (l = 0; l <= mem[s + 4].b16.s1 - 1; l++) {
                                        c = get_native_usv(s, l);
                                        if (LC_CODE(c) != 0) {
                                            hf = mem[s + 4].b16.s2;
                                            prev_s = s;
                                            goto done2;
                                        }

                                        if (c >= 65536L)
                                            l++;
                                    }
                                }

                                if (mem[s].b16.s0 == LANGUAGE_NODE) {
                                    cur_lang = mem[s + 1].b32.s1;
                                    l_hyf = mem[s + 1].b16.s1;
                                    r_hyf = mem[s + 1].b16.s0;
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
                            s = mem[prev_s].b32.s1;
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

                        if (ha != MIN_HALFWORD &&
                            ha < hi_mem_min &&
                            NODE_type(ha) == WHATSIT_NODE &&
                            (mem[ha].b16.s0 == NATIVE_WORD_NODE || mem[ha].b16.s0 == NATIVE_WORD_NODE_AT))
                        {
                            s = mem[ha].b32.s1;

                            while (true) {
                                if (s < hi_mem_min) {
                                    switch (mem[s].b16.s1) {
                                    case LIGATURE_NODE:
                                        break;

                                    case KERN_NODE:
                                        if (mem[s].b16.s0 != NORMAL)
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

                                s = mem[s].b32.s1;
                            }

                        done6:
                            hn = 0;

                        restart:
                            /* 'ha' can change in the loop, so for safety: */
                            for_end_1 = mem[ha + 4].b16.s1 - 1;

                            for (l = 0; l <= for_end_1; l++) {
                                c = get_native_usv(ha, l);

                                if (hyph_index == 0 || c > 255)
                                    hc[0] = LC_CODE(c);
                                else if (trie_trc[hyph_index + c] != c)
                                    hc[0] = 0;
                                else
                                    hc[0] = trie_tro[hyph_index + c];

                                if (hc[0] == 0) {
                                    if (hn > 0) {
                                        q = new_native_word_node(hf, mem[ha + 4].b16.s1 - l);
                                        mem[q].b16.s0 = mem[ha].b16.s0;

                                        for (i = l; i <= mem[ha + 4].b16.s1 - 1; i++)
                                            set_native_char(q, i - l, get_native_char(ha, i));

                                        set_native_metrics(q, (INTPAR(xetex_use_glyph_metrics) > 0));
                                        mem[q].b32.s1 = mem[ha].b32.s1;
                                        mem[ha].b32.s1 = q;
                                        mem[ha + 4].b16.s1 = l;
                                        set_native_metrics(ha, (INTPAR(xetex_use_glyph_metrics) > 0));
                                        goto done3;
                                    }
                                } else if (hn == 0 && l > 0) {
                                    q = new_native_word_node(hf, mem[ha + 4].b16.s1 - l);
                                    mem[q].b16.s0 = mem[ha].b16.s0;

                                    for (i = l; i <= mem[ha + 4].b16.s1 - 1; i++)
                                        set_native_char(q, i - l, get_native_char(ha, i));

                                    set_native_metrics(q, (INTPAR(xetex_use_glyph_metrics) > 0));
                                    mem[q].b32.s1 = mem[ha].b32.s1;
                                    mem[ha].b32.s1 = q;
                                    mem[ha + 4].b16.s1 = l;
                                    set_native_metrics(ha, (INTPAR(xetex_use_glyph_metrics) > 0));
                                    ha = mem[ha].b32.s1;
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
                            hn = 0;

                            while (true) {
                                if (s >= hi_mem_min) {
                                    if (mem[s].b16.s1 != hf)
                                        goto done3;

                                    hyf_bchar = mem[s].b16.s0;
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
                                } else if (NODE_type(s) == LIGATURE_NODE) { /*932:*/
                                    if (mem[s + 1].b16.s1 != hf)
                                        goto done3;

                                    j = hn;
                                    q = mem[s + 1].b32.s1;

                                    if (q > MIN_HALFWORD)
                                        hyf_bchar = mem[q].b16.s0;

                                    while (q > MIN_HALFWORD) {
                                        c = mem[q].b16.s0;
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
                                        q = mem[q].b32.s1;
                                    }

                                    hb = s;
                                    hn = j;

                                    if (odd(mem[s].b16.s0))
                                        hyf_bchar = font_bchar[hf];
                                    else
                                        hyf_bchar = TOO_BIG_CHAR;
                                } else if (NODE_type(s) == KERN_NODE && mem[s].b16.s0 == NORMAL) {
                                    hb = s;
                                    hyf_bchar = font_bchar[hf];
                                } else {
                                    goto done3;
                                }

                                s = mem[s].b32.s1;
                            }
                        done3:
                            ;
                        }

                        if (hn < l_hyf + r_hyf)
                            goto done1;

                        while (true) {
                            if (s < hi_mem_min) {
                                switch (mem[s].b16.s1) {
                                case LIGATURE_NODE:
                                    break;
                                case KERN_NODE:
                                    if (mem[s].b16.s0 != NORMAL)
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
                                    if (mem[s].b16.s0 >= L_CODE)
                                        goto done4;
                                    else
                                        goto done1;
                                    break;
                                default:
                                    goto done1;
                                    break;
                                }
                            }

                            s = mem[s].b32.s1;
                        }

                    done4:
                        hyphenate();
                    }

                done1:
                    ;
                }
                break; /* that was a long-ass GLUE_NODE case */

            case KERN_NODE:
                if (mem[cur_p].b16.s0 == EXPLICIT) {
                    if (mem[cur_p].b32.s1 < hi_mem_min && auto_breaking) {
                        if (NODE_type(mem[cur_p].b32.s1) == GLUE_NODE)
                            try_break(0, UNHYPHENATED);
                    }
                    active_width[1] += mem[cur_p + 1].b32.s1;
                } else
                    active_width[1] += mem[cur_p + 1].b32.s1;
                break;

            case LIGATURE_NODE:
                f = mem[cur_p + 1].b16.s1;
                xtx_ligature_present = true;
                active_width[1] =
                    active_width[1] + font_info[width_base[f] +
                                                font_info[char_base[f] +
                                                          effective_char(true, f,
                                                                         mem[cur_p + 1].b16.s0)].b16.s3].b32.s1;
                break;

            case DISC_NODE:
                s = mem[cur_p + 1].b32.s0;
                disc_width = 0;

                if (s == MIN_HALFWORD) {
                    try_break(INTPAR(ex_hyphen_penalty), HYPHENATED);
                } else {
                    do {
                        /*899:*/
                        if (s >= hi_mem_min) {
                            int32_t eff_char;
                            uint16_t char_info;

                            f = mem[s].b16.s1;
                            eff_char = effective_char(true, f, mem[s].b16.s0);
                            char_info = font_info[char_base[f] + eff_char].b16.s3;
                            disc_width += font_info[width_base[f] + char_info].b32.s1;
                        } else {
                            switch (mem[s].b16.s1) {
                            case LIGATURE_NODE:
                            {
                                int32_t eff_char;
                                uint16_t char_info;

                                f = mem[s + 1].b16.s1;
                                xtx_ligature_present = true;
                                eff_char = effective_char(true, f, mem[s + 1].b16.s0);
                                char_info = font_info[char_base[f] + eff_char].b16.s3;
                                disc_width += font_info[width_base[f] + char_info].b32.s1;
                                break;
                            }
                            case HLIST_NODE:
                            case VLIST_NODE:
                            case RULE_NODE:
                            case KERN_NODE:
                                disc_width += mem[s + 1].b32.s1;
                                break;
                            case WHATSIT_NODE:
                                if (mem[s].b16.s0 == NATIVE_WORD_NODE ||
                                    mem[s].b16.s0 == NATIVE_WORD_NODE_AT ||
                                    mem[s].b16.s0 == GLYPH_NODE ||
                                    mem[s].b16.s0 == PIC_NODE ||
                                    mem[s].b16.s0 == PDF_NODE)
                                    disc_width += mem[s + 1].b32.s1;
                                else
                                    confusion("disc3a");
                                break;
                            default:
                                confusion("disc3");
                                break;
                            }
                        }

                        s = mem[s].b32.s1;
                    } while (s != MIN_HALFWORD);

                    active_width[1] += disc_width;
                    try_break(INTPAR(hyphen_penalty), HYPHENATED);
                    active_width[1] -= disc_width;
                }

                r = mem[cur_p].b16.s0;
                s = mem[cur_p].b32.s1;

                while (r > 0) {
                    if (s >= hi_mem_min) {
                        int32_t eff_char;
                        uint16_t char_info;

                        f = mem[s].b16.s1;
                        eff_char = effective_char(true, f, mem[s].b16.s0);
                        char_info = font_info[char_base[f] + eff_char].b16.s3;
                        active_width[1] += font_info[width_base[f] + char_info].b32.s1;
                    } else {
                        switch (mem[s].b16.s1) {
                        case LIGATURE_NODE:
                        {
                            int32_t eff_char;
                            uint16_t char_info;

                            f = mem[s + 1].b16.s1;
                            xtx_ligature_present = true;
                            eff_char = effective_char(true, f, mem[s + 1].b16.s0);
                            char_info = font_info[char_base[f] + eff_char].b16.s3;
                            active_width[1] += font_info[width_base[f] + char_info].b32.s1;
                            break;
                        }
                        case HLIST_NODE:
                        case VLIST_NODE:
                        case RULE_NODE:
                        case KERN_NODE:
                            active_width[1] += mem[s + 1].b32.s1;
                            break;
                        case WHATSIT_NODE:
                            if (mem[s].b16.s0 == NATIVE_WORD_NODE ||
                                mem[s].b16.s0 == NATIVE_WORD_NODE_AT ||
                                mem[s].b16.s0 == GLYPH_NODE ||
                                mem[s].b16.s0 == PIC_NODE ||
                                mem[s].b16.s0 == PDF_NODE)
                                active_width[1] += mem[s + 1].b32.s1;
                            else
                                confusion("disc4a");
                            break;
                        default:
                            confusion("disc4");
                            break;
                        }
                    }

                    r--;
                    s = mem[s].b32.s1;
                }

                prev_p = global_prev_p = cur_p;
                cur_p = s;
                goto done5;
                break; /* big DISC_NODE case */

            case MATH_NODE:
                if (mem[cur_p].b16.s0 < L_CODE)
                    auto_breaking = odd(mem[cur_p].b16.s0);

                if (mem[cur_p].b32.s1 < hi_mem_min && auto_breaking) {
                    if (NODE_type(mem[cur_p].b32.s1) == GLUE_NODE)
                        try_break(0, UNHYPHENATED);
                }

                active_width[1] += mem[cur_p + 1].b32.s1;
                break;

            case PENALTY_NODE:
                try_break(mem[cur_p + 1].b32.s1, UNHYPHENATED);
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
            cur_p = mem[cur_p].b32.s1;
        done5:
            ;
        }

        if (cur_p == MIN_HALFWORD) { /*902:*/
            try_break(EJECT_PENALTY, HYPHENATED);

            if (mem[ACTIVE_LIST].b32.s1 != ACTIVE_LIST) {
                r = mem[ACTIVE_LIST].b32.s1;
                fewest_demerits = MAX_HALFWORD;
                do {
                    if (NODE_type(r) != DELTA_NODE) {
                        if (mem[r + 2].b32.s1 < fewest_demerits) {
                            fewest_demerits = mem[r + 2].b32.s1;
                            best_bet = r;
                        }
                    }
                    r = mem[r].b32.s1;
                } while (r != ACTIVE_LIST);

                best_line = mem[best_bet + 1].b32.s0; /*:903*/

                if (INTPAR(looseness) == 0)
                    goto done;

                r = mem[ACTIVE_LIST].b32.s1;
                actual_looseness = 0;
                do {
                    if (NODE_type(r) != DELTA_NODE) {
                        line_diff = mem[r + 1].b32.s0 - best_line;
                        if (((line_diff < actual_looseness) && (INTPAR(looseness) <= line_diff))
                            || ((line_diff > actual_looseness)
                                && (INTPAR(looseness) >= line_diff))) {
                            best_bet = r;
                            actual_looseness = line_diff;
                            fewest_demerits = mem[r + 2].b32.s1;
                        } else if ((line_diff == actual_looseness) && (mem[r + 2].b32.s1 < fewest_demerits)) {
                            best_bet = r;
                            fewest_demerits = mem[r + 2].b32.s1;
                        }
                    }
                    r = mem[r].b32.s1;
                } while (r != ACTIVE_LIST);

                best_line = mem[best_bet + 1].b32.s0;

                if (actual_looseness == INTPAR(looseness) || final_pass)
                    goto done;
            }
        }

        q = mem[ACTIVE_LIST].b32.s1;

        while (q != ACTIVE_LIST) {
            cur_p = mem[q].b32.s1;
            if (NODE_type(q) == DELTA_NODE)
                free_node(q, DELTA_NODE_SIZE);
            else
                free_node(q, active_node_size);
            q = cur_p;
        }

        q = passive;

        while (q != MIN_HALFWORD) {
            cur_p = mem[q].b32.s1;
            free_node(q, PASSIVE_NODE_SIZE);
            q = cur_p;
        }

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
        if (mem[best_bet + 3].b32.s1 == 0) {
            do_last_line_fit = false;
        } else {
            q = new_spec(mem[last_line_fill + 1].b32.s0);
            delete_glue_ref(mem[last_line_fill + 1].b32.s0);
            mem[q + 1].b32.s1 = mem[q + 1].b32.s1 + mem[best_bet + 3].b32.s1 - mem[best_bet + 4].b32.s1;
            mem[q + 2].b32.s1 = 0;
            mem[last_line_fill + 1].b32.s0 = q;
        }
    }

    post_line_break(d);

    /* Clean up by removing break nodes (894) */

    q = mem[ACTIVE_LIST].b32.s1;

    while (q != ACTIVE_LIST) {
        cur_p = mem[q].b32.s1;
        if (NODE_type(q) == DELTA_NODE)
            free_node(q, DELTA_NODE_SIZE);
        else
            free_node(q, active_node_size);
        q = cur_p;
    }

    q = passive;

    while (q != MIN_HALFWORD) {
        cur_p = mem[q].b32.s1;
        free_node(q, PASSIVE_NODE_SIZE);
        q = cur_p;
    }

    /* All done */
    pack_begin_line = 0;
}


/* This was just separated out to prevent line_break() from becoming
 * proposterously long. */
static void
post_line_break(bool d)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
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

    q = mem[best_bet + 1].b32.s1;
    cur_p = MIN_HALFWORD;

    do {
        r = q;
        q = mem[q + 1].b32.s0;
        mem[r + 1].b32.s0 = cur_p;
        cur_p = r;
    } while (q != MIN_HALFWORD); /*:907*/

    cur_line = cur_list.prev_graf + 1;

    do {
        /* 909: justify the line ending at breakpoint cur_p and append it to
         * the current vertical list, with associated penalties and
         * insertions. The current line starts a TEMP_HEAD.link and ends at
         * cur_p.cur_break.
         **/

        if (INTPAR(texxet) > 0) { /*1494:*/
            q = mem[TEMP_HEAD].b32.s1;

            if (LR_ptr != MIN_HALFWORD) {
                temp_ptr = LR_ptr;
                r = q;

                do {
                    s = new_math(0, (mem[temp_ptr].b32.s0 - 1));
                    mem[s].b32.s1 = r;
                    r = s;
                    temp_ptr = mem[temp_ptr].b32.s1;
                } while (temp_ptr != MIN_HALFWORD);

                mem[TEMP_HEAD].b32.s1 = r;
            }

            while (q != mem[cur_p + 1].b32.s1) {
                if (q < hi_mem_min && NODE_type(q) == MATH_NODE) { /*1495:*/
                    if (odd(mem[q].b16.s0)) {
                        if (LR_ptr != MIN_HALFWORD && mem[LR_ptr].b32.s0 == (L_CODE * (mem[q].b16.s0 / L_CODE) + 3)) {
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

                q = mem[q].b32.s1;
            }
        }

        /* 910: "Modify the end of the line to reflect the nature of the break
         * and to include \rightskip; also set the proper value of
         * disc_break" */

        q = mem[cur_p + 1].b32.s1;
        disc_break = false;
        post_disc_break = false;
        glue_break = false;

        if (q == MIN_HALFWORD) {
            q = TEMP_HEAD;
            while (mem[q].b32.s1 != MIN_HALFWORD)
                q = mem[q].b32.s1;
        } else {
            if (NODE_type(q) == GLUE_NODE) {
                delete_glue_ref(mem[q + 1].b32.s0);
                mem[q + 1].b32.s0 = GLUEPAR(right_skip);
                mem[q].b16.s0 = (GLUE_PAR__right_skip + 1);
                mem[GLUEPAR(right_skip)].b32.s1++;
                glue_break = true;
                goto done;
            } else {
                if (NODE_type(q) == DISC_NODE) { /*911:*/
                    t = mem[q].b16.s0;

                    if (t == 0) {
                        r = mem[q].b32.s1;
                    } else {
                        r = q;

                        while (t > 1) {
                            r = mem[r].b32.s1;
                            t--;
                        }

                        s = mem[r].b32.s1;
                        r = mem[s].b32.s1;
                        mem[s].b32.s1 = MIN_HALFWORD;
                        flush_node_list(mem[q].b32.s1);
                        mem[q].b16.s0 = 0;
                    }

                    if (mem[q + 1].b32.s1 != MIN_HALFWORD) { /*913:*/
                        s = mem[q + 1].b32.s1;
                        while (mem[s].b32.s1 != MIN_HALFWORD)
                            s = mem[s].b32.s1;
                        mem[s].b32.s1 = r;
                        r = mem[q + 1].b32.s1;
                        mem[q + 1].b32.s1 = MIN_HALFWORD;
                        post_disc_break = true;
                    }

                    if (mem[q + 1].b32.s0 != MIN_HALFWORD) { /*914:*/
                        s = mem[q + 1].b32.s0;
                        mem[q].b32.s1 = s;
                        while (mem[s].b32.s1 != MIN_HALFWORD)
                            s = mem[s].b32.s1;
                        mem[q + 1].b32.s0 = MIN_HALFWORD;
                        q = s;
                    }

                    mem[q].b32.s1 = r;
                    disc_break = true;
                } else if (NODE_type(q) == KERN_NODE) {
                    mem[q + 1].b32.s1 = 0;
                } else if (NODE_type(q) == MATH_NODE) {
                    mem[q + 1].b32.s1 = 0;

                    if (INTPAR(texxet) > 0) { /*1495:*/
                        if (odd(mem[q].b16.s0)) {
                            if (LR_ptr != MIN_HALFWORD && mem[LR_ptr].b32.s0 == (L_CODE * (mem[q].b16.s0 / L_CODE) + 3)) {
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
            }
        }

    done:
        if (INTPAR(xetex_protrude_chars) > 0) {
            if (disc_break && (q >= hi_mem_min || NODE_type(q) != DISC_NODE)) {
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
                    q = mem[q].b32.s1;
            }
        }

        if (!glue_break) {
            r = new_param_glue(GLUE_PAR__right_skip);
            mem[r].b32.s1 = mem[q].b32.s1;
            mem[q].b32.s1 = r;
            q = r;
        } /*:915*/

        if (INTPAR(texxet) > 0) { /*1496:*/
            if (LR_ptr != MIN_HALFWORD) {
                s = TEMP_HEAD;
                r = mem[s].b32.s1;

                while (r != q) {
                    s = r;
                    r = mem[s].b32.s1;
                }

                r = LR_ptr;

                while (r != MIN_HALFWORD) {
                    temp_ptr = new_math(0, mem[r].b32.s0);
                    mem[s].b32.s1 = temp_ptr;
                    s = temp_ptr;
                    r = mem[r].b32.s1;
                }

                mem[s].b32.s1 = q;
            }
        }

        /* 916: Put \leftskip at the left and detach this line. */

        r = mem[q].b32.s1;
        mem[q].b32.s1 = MIN_HALFWORD;
        q = mem[TEMP_HEAD].b32.s1;
        mem[TEMP_HEAD].b32.s1 = r;

        if (INTPAR(xetex_protrude_chars) > 0) {
            p = q;
            p = find_protchar_left(p, false);
            w = char_pw(p, 0);
            if (w != 0) {
                k = new_margin_kern(-(int32_t) w, last_leftmost_char, 0);
                mem[k].b32.s1 = q;
                q = k;
            }
        }

        if (GLUEPAR(left_skip) != 0) {
            r = new_param_glue(GLUE_PAR__left_skip);
            mem[r].b32.s1 = q;
            q = r;
        }

        /* 918: q points to the hlist that represents the current line. Pack
         * it up at the right width. */

        if (cur_line > last_special_line) {
            cur_width = second_width;
            cur_indent = second_indent;
        } else if (LOCAL(par_shape) == MIN_HALFWORD) {
            cur_width = first_width;
            cur_indent = first_indent;
        } else {
            cur_width = mem[LOCAL(par_shape) + 2 * cur_line].b32.s1;
            cur_indent = mem[LOCAL(par_shape) + 2 * cur_line - 1].b32.s1;
        }

        adjust_tail = ADJUST_HEAD;
        pre_adjust_tail = PRE_ADJUST_HEAD;
        just_box = hpack(q, cur_width, EXACTLY);
        mem[just_box + 4].b32.s1 = cur_indent; /*:918*/

        /* 917: append the new box to the urrent vertical list, followed
         * by any of its special nodes that were taken out */

        if (PRE_ADJUST_HEAD != pre_adjust_tail) {
            mem[cur_list.tail].b32.s1 = mem[PRE_ADJUST_HEAD].b32.s1;
            cur_list.tail = pre_adjust_tail;
        }

        pre_adjust_tail = MIN_HALFWORD;
        append_to_vlist(just_box);

        if (ADJUST_HEAD != adjust_tail) {
            mem[cur_list.tail].b32.s1 = mem[ADJUST_HEAD].b32.s1;
            cur_list.tail = adjust_tail;
        }

        adjust_tail = MIN_HALFWORD; /*:917*/

        /* 919: Set `pen` to all of the penalties relevant to this line. */

        if (cur_line + 1 != best_line) {
            q = eqtb[INTER_LINE_PENALTIES_LOC].b32.s1;

            if (q != MIN_HALFWORD) {
                r = cur_line;
                if (r > mem[q + 1].b32.s1)
                    r = mem[q + 1].b32.s1;
                pen = mem[q + r + 1].b32.s1;
            } else {
                pen = INTPAR(inter_line_penalty);
            }

            q = eqtb[CLUB_PENALTIES_LOC].b32.s1;
            if (q != MIN_HALFWORD) {
                r = cur_line - cur_list.prev_graf;
                if (r > mem[q + 1].b32.s1)
                    r = mem[q + 1].b32.s1;
                pen += mem[q + r + 1].b32.s1;
            } else if (cur_line == cur_list.prev_graf + 1) {
                pen += INTPAR(club_penalty);
            }

            if (d)
                q = eqtb[DISPLAY_WIDOW_PENALTIES_LOC].b32.s1;
            else
                q = eqtb[WIDOW_PENALTIES_LOC].b32.s1;

            if (q != MIN_HALFWORD) {
                r = best_line - cur_line - 1;
                if (r > mem[q + 1].b32.s1)
                    r = mem[q + 1].b32.s1;
                pen += mem[q + r + 1].b32.s1;
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
                mem[cur_list.tail].b32.s1 = r;
                cur_list.tail = r;
            }
        }

        /* Done justifying this line. */

        cur_line++;
        cur_p = mem[cur_p + 1].b32.s0;

        if (cur_p != MIN_HALFWORD) {
            if (!post_disc_break) {
                /* 908: "prune unwanted nodes at the beginning of the next
                 * line". Delete glues, penalties, kerns, and math nodes at
                 * the beginning of the line, unless the node in question is
                 * the chosen breakpoint. */
                r = TEMP_HEAD;

                while (true) {
                    q = mem[r].b32.s1;
                    if (q == mem[cur_p + 1].b32.s1)
                        goto done1;
                    if (q >= hi_mem_min) /* character node? */
                        goto done1;
                    if (NODE_type(q) < MATH_NODE) /* non_discardable(q) */
                        goto done1;
                    if (NODE_type(q) == KERN_NODE && mem[q].b16.s0 != EXPLICIT && mem[q].b16.s0 != SPACE_ADJUSTMENT)
                        goto done1;

                    r = q;

                    if (NODE_type(q) == MATH_NODE && INTPAR(texxet) > 0) { /*1495:*/
                        if (odd(mem[q].b16.s0)) {
                            if (LR_ptr != MIN_HALFWORD && mem[LR_ptr].b32.s0 == (L_CODE * (mem[q].b16.s0 / L_CODE) + 3)) {
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

            done1:
                if (r != TEMP_HEAD) {
                    mem[r].b32.s1 = MIN_HALFWORD;
                    flush_node_list(mem[TEMP_HEAD].b32.s1);
                    mem[TEMP_HEAD].b32.s1 = q;
                }
            }
        }
    } while (cur_p != MIN_HALFWORD);

    if (cur_line != best_line || mem[TEMP_HEAD].b32.s1 != MIN_HALFWORD)
        confusion("line breaking");

    cur_list.prev_graf = best_line - 1;
    cur_list.eTeX_aux = LR_ptr;
}


int32_t
prune_page_top(int32_t p, bool s)
{
    memory_word *mem = zmem;
    int32_t prev_p;
    int32_t q, r = MIN_HALFWORD;

    prev_p = TEMP_HEAD;
    mem[TEMP_HEAD].b32.s1 = p;

    while (p != MIN_HALFWORD) {
        switch (mem[p].b16.s1) {
        case HLIST_NODE:
        case VLIST_NODE:
        case RULE_NODE:
            q = new_skip_param(GLUE_PAR__split_top_skip);
            mem[prev_p].b32.s1 = q;
            mem[q].b32.s1 = p;
            if (mem[temp_ptr + 1].b32.s1 > mem[p + 3].b32.s1)
                mem[temp_ptr + 1].b32.s1 = mem[temp_ptr + 1].b32.s1 - mem[p + 3].b32.s1;
            else
                mem[temp_ptr + 1].b32.s1 = 0;
            p = MIN_HALFWORD;
            break;
        case WHATSIT_NODE:
        case MARK_NODE:
        case INS_NODE:
            prev_p = p;
            p = mem[prev_p].b32.s1;
            break;
        case GLUE_NODE:
        case KERN_NODE:
        case PENALTY_NODE:
            q = p;
            p = mem[q].b32.s1;
            mem[q].b32.s1 = MIN_HALFWORD;
            mem[prev_p].b32.s1 = p;
            if (s) {
                if (disc_ptr[VSPLIT_CODE] == MIN_HALFWORD)
                    disc_ptr[VSPLIT_CODE] = q;
                else
                    mem[r].b32.s1 = q;
                r = q;
            } else {
                flush_node_list(q);
            }
            break;
        default:
            confusion("pruning");
            break;
        }
    }

    return mem[TEMP_HEAD].b32.s1;
}


bool
do_marks(small_number a, small_number l, int32_t q)
{
    memory_word *mem = zmem;
    small_number i;

    if (l < 4) {
        for (i = 0; i <= 15; i++) {
            if (odd(i))
                cur_ptr = mem[q + (i / 2) + 1].b32.s1;
            else
                cur_ptr = mem[q + (i / 2) + 1].b32.s0;

            if (cur_ptr != MIN_HALFWORD) {
                if (do_marks(a, l + 1, cur_ptr)) {
                    if (odd(i))
                        mem[q + (i / 2) + 1].b32.s1 = MIN_HALFWORD;
                    else
                        mem[q + (i / 2) + 1].b32.s0 = MIN_HALFWORD;
                    mem[q].b16.s0--;
                }
            }
        }

        if (mem[q].b16.s0 == 0) {
            free_node(q, INDEX_NODE_SIZE);
            q = MIN_HALFWORD;
        }
    } else {
        switch (a) { /*1614: */
        case VSPLIT_INIT:
            if (mem[q + 2].b32.s1 != MIN_HALFWORD) {
                delete_token_ref(mem[q + 2].b32.s1);
                mem[q + 2].b32.s1 = MIN_HALFWORD;
                delete_token_ref(mem[q + 3].b32.s0);
                mem[q + 3].b32.s0 = MIN_HALFWORD;
            }
            break;

        case FIRE_UP_INIT:
            if (mem[q + 2].b32.s0 != MIN_HALFWORD) {
                if (mem[q + 1].b32.s0 != MIN_HALFWORD)
                    delete_token_ref(mem[q + 1].b32.s0);
                delete_token_ref(mem[q + 1].b32.s1);
                mem[q + 1].b32.s1 = MIN_HALFWORD;
                if (mem[mem[q + 2].b32.s0].b32.s1 == MIN_HALFWORD) {
                    delete_token_ref(mem[q + 2].b32.s0);
                    mem[q + 2].b32.s0 = MIN_HALFWORD;
                } else
                    mem[mem[q + 2].b32.s0].b32.s0++;
                mem[q + 1].b32.s0 = mem[q + 2].b32.s0;
            }
            break;

        case FIRE_UP_DONE:
            if ((mem[q + 1].b32.s0 != MIN_HALFWORD) && (mem[q + 1].b32.s1 == MIN_HALFWORD)) {
                mem[q + 1].b32.s1 = mem[q + 1].b32.s0;
                mem[mem[q + 1].b32.s0].b32.s0++;
            }
            break;

        case DESTROY_MARKS:
            for (i = TOP_MARK_CODE; i <= SPLIT_BOT_MARK_CODE; i++) {
                if (odd(i))
                    cur_ptr = mem[q + (i / 2) + 1].b32.s1;
                else
                    cur_ptr = mem[q + (i / 2) + 1].b32.s0;

                if (cur_ptr != MIN_HALFWORD) {
                    delete_token_ref(cur_ptr);
                    if (odd(i))
                        mem[q + (i / 2) + 1].b32.s1 = MIN_HALFWORD;
                    else
                        mem[q + (i / 2) + 1].b32.s0 = MIN_HALFWORD;
                }
            }
            break;
        }

        if (mem[q + 2].b32.s0 == MIN_HALFWORD) {
            if (mem[q + 3].b32.s0 == MIN_HALFWORD) {
                free_node(q, MARK_CLASS_NODE_SIZE);
                q = MIN_HALFWORD;
            }
        }
    }

    return (q == MIN_HALFWORD);
}


void
do_assignments(void)
{
    while (true) {
        do {
            get_x_token();
        } while (cur_cmd == SPACER || cur_cmd == RELAX);

        if (cur_cmd <= MAX_NON_PREFIXED_COMMAND)
            return;

        set_box_allowed = false;
        prefixed_command();
        set_box_allowed = true;
    }
}


void
new_whatsit(small_number s, small_number w)
{
    memory_word *mem = zmem;
    int32_t p;

    p = get_node(w);
    NODE_type(p) = WHATSIT_NODE;
    mem[p].b16.s0 = s;
    mem[cur_list.tail].b32.s1 = p;
    cur_list.tail = p;
}
