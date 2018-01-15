/* tectonic/inimisc.c -- random routines originally in xetexini.c
   Copyright 2016-2017 The Tectonic Project
   Licensed under the MIT License.
*/

#include "tectonic.h"
#include "internals.h"
#include "xetexd.h"
#include "core-bridge.h"


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
    integer l;
    integer i;
    integer for_end_1;

    pack_begin_line = cur_list.ml;
    mem[mem_top - 3].hh.H0 = mem[cur_list.head].hh.H0;

    if (cur_list.tail >= hi_mem_min) {
        mem[cur_list.tail].hh.H0 = new_penalty(INF_PENALTY);
        cur_list.tail = mem[cur_list.tail].hh.H0;
    } else if (mem[cur_list.tail].b16.s1 != GLUE_NODE) {
        mem[cur_list.tail].hh.H0 = new_penalty(INF_PENALTY);
        cur_list.tail = mem[cur_list.tail].hh.H0;
    } else {
        mem[cur_list.tail].b16.s1 = PENALTY_NODE;
        delete_glue_ref(mem[cur_list.tail + 1].hh.H1);
        flush_node_list(mem[cur_list.tail + 1].hh.H0);
        mem[cur_list.tail + 1].hh.H0 = INF_PENALTY;
    }

    mem[cur_list.tail].hh.H0 = new_param_glue(GLUE_PAR__par_fill_skip);
    last_line_fill = mem[cur_list.tail].hh.H0;
    init_cur_lang = cur_list.pg % 65536L;
    init_l_hyf = cur_list.pg / 0x0400000;
    init_r_hyf = (cur_list.pg / 65536L) % 64;

    pop_nest();

    no_shrink_error_yet = true;

    if (mem[GLUEPAR(left_skip)].b16.s0 != NORMAL && mem[GLUEPAR(left_skip) + 3].hh.H0 != 0)
        GLUEPAR(left_skip) = finite_shrink(GLUEPAR(left_skip));

    if (mem[GLUEPAR(right_skip)].b16.s0 != NORMAL && mem[GLUEPAR(right_skip) + 3].hh.H0 != 0)
        GLUEPAR(right_skip) = finite_shrink(GLUEPAR(right_skip));

    q = GLUEPAR(left_skip);
    r = GLUEPAR(right_skip);

    background[1] = mem[q + 1].hh.H0 + mem[r + 1].hh.H0;
    background[2] = 0;
    background[3] = 0;
    background[4] = 0;
    background[5] = 0;
    background[2 + mem[q].b16.s1] = mem[q + 2].hh.H0;
    background[2 + mem[r].b16.s1] = background[2 + mem[r].b16.s1] + mem[r + 2].hh.H0;
    background[6] = mem[q + 3].hh.H0 + mem[r + 3].hh.H0;
    do_last_line_fit = false;
    active_node_size = ACTIVE_NODE_SIZE_NORMAL;

    if (INTPAR(last_line_fit) > 0) {
        q = mem[last_line_fill + 1].hh.H1;

        if (mem[q + 2].hh.H0 > 0 && mem[q].b16.s1 > NORMAL) {
            if (background[3] == 0 && background[4] == 0 && background[5] == 0) {
                do_last_line_fit = true;
                active_node_size = ACTIVE_NODE_SIZE_EXTENDED;
                fill_width[0] = 0;
                fill_width[1] = 0;
                fill_width[2] = 0;
                fill_width[mem[q].b16.s1 - 1] = mem[q + 2].hh.H0;
            }
        }
    }

    minimum_demerits = MAX_HALFWORD;
    minimal_demerits[TIGHT_FIT] = MAX_HALFWORD;
    minimal_demerits[DECENT_FIT] = MAX_HALFWORD;
    minimal_demerits[LOOSE_FIT] = MAX_HALFWORD;
    minimal_demerits[VERY_LOOSE_FIT] = MAX_HALFWORD;

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
        last_special_line = mem[LOCAL(par_shape)].hh.H1 - 1;
        second_width = mem[LOCAL(par_shape) + 2 * (last_special_line + 1)].hh.H0;
        second_indent = mem[LOCAL(par_shape) + 2 * last_special_line + 1].hh.H0;
    }

    if (INTPAR(looseness) == 0)
        easy_line = last_special_line;
    else
        easy_line = MAX_HALFWORD; /*:877*/

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
        mem[q].hh.H0 = mem_top - 7;
        mem[q + 1].hh.H0 = MIN_HALFWORD;
        mem[q + 1].hh.H1 = cur_list.pg + 1;
        mem[q + 2].hh.H0 = 0;
        mem[mem_top - 7].hh.H0 = q;

        if (do_last_line_fit) { /*1633:*/
            mem[q + 3].hh.H0 = 0;
            mem[q + 4].hh.H0 = 0;
        }

        active_width[1] = background[1];
        active_width[2] = background[2];
        active_width[3] = background[3];
        active_width[4] = background[4];
        active_width[5] = background[5];
        active_width[6] = background[6];
        passive = MIN_HALFWORD;
        printed_node = mem_top - 3;
        pass_number = 0;
        font_in_short_display = 0; /*:893*/
        cur_p = mem[mem_top - 3].hh.H0;
        auto_breaking = true;

        prev_p = global_prev_p = cur_p;
        first_p = cur_p;

        while (cur_p != MIN_HALFWORD && mem[mem_top - 7].hh.H0 != mem_top - 7) { /*895:*/
            if (cur_p >= hi_mem_min) { /*896:*/
                prev_p = global_prev_p = cur_p;

                do {
                    integer eff_char;
                    uint16_t char_info;

                    f = mem[cur_p].b16.s1;
                    eff_char = effective_char(true, f, mem[cur_p].b16.s0);
                    char_info = font_info[char_base[f] + eff_char].b16.s3;
                    active_width[1] += font_info[width_base[f] + char_info].hh.H0;
                    cur_p = mem[cur_p].hh.H0;
                } while (cur_p >= hi_mem_min);
            }

            switch (mem[cur_p].b16.s1) {
            case HLIST_NODE:
            case VLIST_NODE:
            case RULE_NODE:
                active_width[1] += mem[cur_p + 1].hh.H0;
                break;

            case WHATSIT_NODE:
                if (mem[cur_p].b16.s0 == LANGUAGE_NODE) {
                    cur_lang = mem[cur_p + 1].hh.H0;
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
                    active_width[1] += mem[cur_p + 1].hh.H0;
                }
                break;

            case GLUE_NODE:
                if (auto_breaking) {
                    if (prev_p >= hi_mem_min)
                        try_break(0, UNHYPHENATED);
                    else if (mem[prev_p].b16.s1 < MATH_NODE)
                        try_break(0, UNHYPHENATED);
                    else if (mem[prev_p].b16.s1 == KERN_NODE && mem[prev_p].b16.s0 != EXPLICIT)
                        try_break(0, UNHYPHENATED);
                }

                if (mem[mem[cur_p + 1].hh.H1].b16.s0 != NORMAL && mem[mem[cur_p + 1].hh.H1 + 3].hh.H0 != 0)
                    mem[cur_p + 1].hh.H1 = finite_shrink(mem[cur_p + 1].hh.H1);

                q = mem[cur_p + 1].hh.H1;
                active_width[1] = active_width[1] + mem[q + 1].hh.H0;
                active_width[2 + mem[q].b16.s1] = active_width[2 + mem[q].b16.s1] + mem[q + 2].hh.H0;
                active_width[6] = active_width[6] + mem[q + 3].hh.H0; /*:897*/

                if (second_pass && auto_breaking) { /*924:*/
                    prev_s = cur_p;
                    s = mem[prev_s].hh.H0;

                    if (s != MIN_HALFWORD) {
                        while (true) {
                            if (s >= hi_mem_min) {
                                c = mem[s].b16.s0;
                                hf = mem[s].b16.s1;
                            } else if (mem[s].b16.s1 == LIGATURE_NODE) {
                                if (mem[s + 1].hh.H0 == MIN_HALFWORD)
                                    goto _continue;

                                q = mem[s + 1].hh.H0;
                                c = mem[q].b16.s0;
                                hf = mem[q].b16.s1;
                            } else if (mem[s].b16.s1 == KERN_NODE && mem[s].b16.s0 == NORMAL) {
                                goto _continue;
                            } else if (mem[s].b16.s1 == MATH_NODE && mem[s].b16.s0 >= L_CODE) {
                                goto _continue;
                            } else if (mem[s].b16.s1 == WHATSIT_NODE) {
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
                                    cur_lang = mem[s + 1].hh.H0;
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
                            s = mem[prev_s].hh.H0;
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
                            mem[ha].b16.s1 == WHATSIT_NODE &&
                            (mem[ha].b16.s0 == NATIVE_WORD_NODE || mem[ha].b16.s0 == NATIVE_WORD_NODE_AT))
                        {
                            s = mem[ha].hh.H0;

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

                                s = mem[s].hh.H0;
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
                                        mem[q].hh.H0 = mem[ha].hh.H0;
                                        mem[ha].hh.H0 = q;
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
                                    mem[q].hh.H0 = mem[ha].hh.H0;
                                    mem[ha].hh.H0 = q;
                                    mem[ha + 4].b16.s1 = l;
                                    set_native_metrics(ha, (INTPAR(xetex_use_glyph_metrics) > 0));
                                    ha = mem[ha].hh.H0;
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
                                } else if (mem[s].b16.s1 == LIGATURE_NODE) { /*932:*/
                                    if (mem[s + 1].b16.s1 != hf)
                                        goto done3;

                                    j = hn;
                                    q = mem[s + 1].hh.H0;

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
                                        q = mem[q].hh.H0;
                                    }

                                    hb = s;
                                    hn = j;

                                    if (odd(mem[s].b16.s0))
                                        hyf_bchar = font_bchar[hf];
                                    else
                                        hyf_bchar = TOO_BIG_CHAR;
                                } else if (mem[s].b16.s1 == KERN_NODE && mem[s].b16.s0 == NORMAL) {
                                    hb = s;
                                    hyf_bchar = font_bchar[hf];
                                } else {
                                    goto done3;
                                }

                                s = mem[s].hh.H0;
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

                            s = mem[s].hh.H0;
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
                    if (mem[cur_p].hh.H0 < hi_mem_min && auto_breaking) {
                        if (mem[mem[cur_p].hh.H0].b16.s1 == GLUE_NODE)
                            try_break(0, UNHYPHENATED);
                    }
                    active_width[1] += mem[cur_p + 1].hh.H0;
                } else
                    active_width[1] += mem[cur_p + 1].hh.H0;
                break;

            case LIGATURE_NODE:
                f = mem[cur_p + 1].b16.s1;
                xtx_ligature_present = true;
                active_width[1] =
                    active_width[1] + font_info[width_base[f] +
                                                font_info[char_base[f] +
                                                          effective_char(true, f,
                                                                         mem[cur_p + 1].b16.s0)].b16.s3].hh.H0;
                break;

            case DISC_NODE:
                s = mem[cur_p + 1].hh.H1;
                disc_width = 0;

                if (s == MIN_HALFWORD) {
                    try_break(INTPAR(ex_hyphen_penalty), HYPHENATED);
                } else {
                    do {
                        /*899:*/
                        if (s >= hi_mem_min) {
                            integer eff_char;
                            uint16_t char_info;

                            f = mem[s].b16.s1;
                            eff_char = effective_char(true, f, mem[s].b16.s0);
                            char_info = font_info[char_base[f] + eff_char].b16.s3;
                            disc_width += font_info[width_base[f] + char_info].hh.H0;
                        } else {
                            switch (mem[s].b16.s1) {
                            case LIGATURE_NODE:
                            {
                                integer eff_char;
                                uint16_t char_info;

                                f = mem[s + 1].b16.s1;
                                xtx_ligature_present = true;
                                eff_char = effective_char(true, f, mem[s + 1].b16.s0);
                                char_info = font_info[char_base[f] + eff_char].b16.s3;
                                disc_width += font_info[width_base[f] + char_info].hh.H0;
                                break;
                            }
                            case HLIST_NODE:
                            case VLIST_NODE:
                            case RULE_NODE:
                            case KERN_NODE:
                                disc_width += mem[s + 1].hh.H0;
                                break;
                            case WHATSIT_NODE:
                                if (mem[s].b16.s0 == NATIVE_WORD_NODE ||
                                    mem[s].b16.s0 == NATIVE_WORD_NODE_AT ||
                                    mem[s].b16.s0 == GLYPH_NODE ||
                                    mem[s].b16.s0 == PIC_NODE ||
                                    mem[s].b16.s0 == PDF_NODE)
                                    disc_width += mem[s + 1].hh.H0;
                                else
                                    confusion("disc3a");
                                break;
                            default:
                                confusion("disc3");
                                break;
                            }
                        }

                        s = mem[s].hh.H0;
                    } while (s != MIN_HALFWORD);

                    active_width[1] += disc_width;
                    try_break(INTPAR(hyphen_penalty), HYPHENATED);
                    active_width[1] -= disc_width;
                }

                r = mem[cur_p].b16.s0;
                s = mem[cur_p].hh.H0;

                while (r > 0) {
                    if (s >= hi_mem_min) {
                        integer eff_char;
                        uint16_t char_info;

                        f = mem[s].b16.s1;
                        eff_char = effective_char(true, f, mem[s].b16.s0);
                        char_info = font_info[char_base[f] + eff_char].b16.s3;
                        active_width[1] += font_info[width_base[f] + char_info].hh.H0;
                    } else {
                        switch (mem[s].b16.s1) {
                        case LIGATURE_NODE:
                        {
                            integer eff_char;
                            uint16_t char_info;

                            f = mem[s + 1].b16.s1;
                            xtx_ligature_present = true;
                            eff_char = effective_char(true, f, mem[s + 1].b16.s0);
                            char_info = font_info[char_base[f] + eff_char].b16.s3;
                            active_width[1] += font_info[width_base[f] + char_info].hh.H0;
                            break;
                        }
                        case HLIST_NODE:
                        case VLIST_NODE:
                        case RULE_NODE:
                        case KERN_NODE:
                            active_width[1] += mem[s + 1].hh.H0;
                            break;
                        case WHATSIT_NODE:
                            if (mem[s].b16.s0 == NATIVE_WORD_NODE ||
                                mem[s].b16.s0 == NATIVE_WORD_NODE_AT ||
                                mem[s].b16.s0 == GLYPH_NODE ||
                                mem[s].b16.s0 == PIC_NODE ||
                                mem[s].b16.s0 == PDF_NODE)
                                active_width[1] += mem[s + 1].hh.H0;
                            else
                                confusion("disc4a");
                            break;
                        default:
                            confusion("disc4");
                            break;
                        }
                    }

                    r--;
                    s = mem[s].hh.H0;
                }

                prev_p = global_prev_p = cur_p;
                cur_p = s;
                goto done5;
                break; /* big DISC_NODE case */

            case MATH_NODE:
                if (mem[cur_p].b16.s0 < L_CODE)
                    auto_breaking = odd(mem[cur_p].b16.s0);

                if (mem[cur_p].hh.H0 < hi_mem_min && auto_breaking) {
                    if (mem[mem[cur_p].hh.H0].b16.s1 == GLUE_NODE)
                        try_break(0, UNHYPHENATED);
                }

                active_width[1] += mem[cur_p + 1].hh.H0;
                break;

            case PENALTY_NODE:
                try_break(mem[cur_p + 1].hh.H0, UNHYPHENATED);
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
            cur_p = mem[cur_p].hh.H0;
        done5:
            ;
        }

        if (cur_p == MIN_HALFWORD) { /*902:*/
            try_break(EJECT_PENALTY, HYPHENATED);

            if (mem[mem_top - 7].hh.H0 != mem_top - 7) {
                r = mem[mem_top - 7].hh.H0;
                fewest_demerits = MAX_HALFWORD;
                do {
                    if (mem[r].b16.s1 != DELTA_NODE) {
                        if (mem[r + 2].hh.H0 < fewest_demerits) {
                            fewest_demerits = mem[r + 2].hh.H0;
                            best_bet = r;
                        }
                    }
                    r = mem[r].hh.H0;
                } while (r != mem_top - 7);

                best_line = mem[best_bet + 1].hh.H1; /*:903*/

                if (INTPAR(looseness) == 0)
                    goto done;

                r = mem[mem_top - 7].hh.H0;
                actual_looseness = 0;
                do {
                    if (mem[r].b16.s1 != DELTA_NODE) {
                        line_diff = mem[r + 1].hh.H1 - best_line;
                        if (((line_diff < actual_looseness) && (INTPAR(looseness) <= line_diff))
                            || ((line_diff > actual_looseness)
                                && (INTPAR(looseness) >= line_diff))) {
                            best_bet = r;
                            actual_looseness = line_diff;
                            fewest_demerits = mem[r + 2].hh.H0;
                        } else if ((line_diff == actual_looseness) && (mem[r + 2].hh.H0 < fewest_demerits)) {
                            best_bet = r;
                            fewest_demerits = mem[r + 2].hh.H0;
                        }
                    }
                    r = mem[r].hh.H0;
                } while (r != mem_top - 7);

                best_line = mem[best_bet + 1].hh.H1;

                if (actual_looseness == INTPAR(looseness) || final_pass)
                    goto done;
            }
        }

        q = mem[mem_top - 7].hh.H0;

        while (q != mem_top - 7) {
            cur_p = mem[q].hh.H0;
            if (mem[q].b16.s1 == DELTA_NODE)
                free_node(q, DELTA_NODE_SIZE);
            else
                free_node(q, active_node_size);
            q = cur_p;
        }

        q = passive;

        while (q != MIN_HALFWORD) {
            cur_p = mem[q].hh.H0;
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
        if (mem[best_bet + 3].hh.H0 == 0) {
            do_last_line_fit = false;
        } else {
            q = new_spec(mem[last_line_fill + 1].hh.H1);
            delete_glue_ref(mem[last_line_fill + 1].hh.H1);
            mem[q + 1].hh.H0 = mem[q + 1].hh.H0 + mem[best_bet + 3].hh.H0 - mem[best_bet + 4].hh.H0;
            mem[q + 2].hh.H0 = 0;
            mem[last_line_fill + 1].hh.H1 = q;
        }
    }

    post_line_break(d);

    q = mem[mem_top - 7].hh.H0;

    while (q != mem_top - 7) {
        cur_p = mem[q].hh.H0;
        if (mem[q].b16.s1 == DELTA_NODE)
            free_node(q, DELTA_NODE_SIZE);
        else
            free_node(q, active_node_size);
        q = cur_p;
    }

    q = passive;

    while (q != MIN_HALFWORD) {
        cur_p = mem[q].hh.H0;
        free_node(q, PASSIVE_NODE_SIZE);
        q = cur_p;
    }

    pack_begin_line = 0;
}


int32_t
prune_page_top(int32_t p, bool s)
{
    memory_word *mem = zmem;
    int32_t prev_p;
    int32_t q, r = MIN_HALFWORD;

    prev_p = mem_top - 3;
    mem[mem_top - 3].hh.H0 = p;

    while (p != MIN_HALFWORD) {
        switch (mem[p].b16.s1) {
        case HLIST_NODE:
        case VLIST_NODE:
        case RULE_NODE:
            q = new_skip_param(GLUE_PAR__split_top_skip);
            mem[prev_p].hh.H0 = q;
            mem[q].hh.H0 = p;
            if (mem[temp_ptr + 1].hh.H0 > mem[p + 3].hh.H0)
                mem[temp_ptr + 1].hh.H0 = mem[temp_ptr + 1].hh.H0 - mem[p + 3].hh.H0;
            else
                mem[temp_ptr + 1].hh.H0 = 0;
            p = MIN_HALFWORD;
            break;
        case WHATSIT_NODE:
        case MARK_NODE:
        case INS_NODE:
            prev_p = p;
            p = mem[prev_p].hh.H0;
            break;
        case GLUE_NODE:
        case KERN_NODE:
        case PENALTY_NODE:
            q = p;
            p = mem[q].hh.H0;
            mem[q].hh.H0 = MIN_HALFWORD;
            mem[prev_p].hh.H0 = p;
            if (s) {
                if (disc_ptr[VSPLIT_CODE] == MIN_HALFWORD)
                    disc_ptr[VSPLIT_CODE] = q;
                else
                    mem[r].hh.H0 = q;
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

    return mem[mem_top - 3].hh.H0;
}


bool
do_marks(small_number a, small_number l, int32_t q)
{
    memory_word *mem = zmem;
    small_number i;

    if (l < 4) {
        for (i = 0; i <= 15; i++) {
            if (odd(i))
                cur_ptr = mem[q + (i / 2) + 1].hh.H0;
            else
                cur_ptr = mem[q + (i / 2) + 1].hh.H1;

            if (cur_ptr != MIN_HALFWORD) {
                if (do_marks(a, l + 1, cur_ptr)) {
                    if (odd(i))
                        mem[q + (i / 2) + 1].hh.H0 = MIN_HALFWORD;
                    else
                        mem[q + (i / 2) + 1].hh.H1 = MIN_HALFWORD;
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
            if (mem[q + 2].hh.H0 != MIN_HALFWORD) {
                delete_token_ref(mem[q + 2].hh.H0);
                mem[q + 2].hh.H0 = MIN_HALFWORD;
                delete_token_ref(mem[q + 3].hh.H1);
                mem[q + 3].hh.H1 = MIN_HALFWORD;
            }
            break;

        case FIRE_UP_INIT:
            if (mem[q + 2].hh.H1 != MIN_HALFWORD) {
                if (mem[q + 1].hh.H1 != MIN_HALFWORD)
                    delete_token_ref(mem[q + 1].hh.H1);
                delete_token_ref(mem[q + 1].hh.H0);
                mem[q + 1].hh.H0 = MIN_HALFWORD;
                if (mem[mem[q + 2].hh.H1].hh.H0 == MIN_HALFWORD) {
                    delete_token_ref(mem[q + 2].hh.H1);
                    mem[q + 2].hh.H1 = MIN_HALFWORD;
                } else
                    mem[mem[q + 2].hh.H1].hh.H1++;
                mem[q + 1].hh.H1 = mem[q + 2].hh.H1;
            }
            break;

        case FIRE_UP_DONE:
            if ((mem[q + 1].hh.H1 != MIN_HALFWORD) && (mem[q + 1].hh.H0 == MIN_HALFWORD)) {
                mem[q + 1].hh.H0 = mem[q + 1].hh.H1;
                mem[mem[q + 1].hh.H1].hh.H1++;
            }
            break;

        case DESTROY_MARKS:
            for (i = TOP_MARK_CODE; i <= SPLIT_BOT_MARK_CODE; i++) {
                if (odd(i))
                    cur_ptr = mem[q + (i / 2) + 1].hh.H0;
                else
                    cur_ptr = mem[q + (i / 2) + 1].hh.H1;

                if (cur_ptr != MIN_HALFWORD) {
                    delete_token_ref(cur_ptr);
                    if (odd(i))
                        mem[q + (i / 2) + 1].hh.H0 = MIN_HALFWORD;
                    else
                        mem[q + (i / 2) + 1].hh.H1 = MIN_HALFWORD;
                }
            }
            break;
        }

        if (mem[q + 2].hh.H1 == MIN_HALFWORD) {
            if (mem[q + 3].hh.H1 == MIN_HALFWORD) {
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
    mem[p].b16.s1 = WHATSIT_NODE;
    mem[p].b16.s0 = s;
    mem[cur_list.tail].hh.H0 = p;
    cur_list.tail = p;
}
