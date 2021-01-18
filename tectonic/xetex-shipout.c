/* Copyright 2016-2018 The Tectonic Project
 * Licensed under the MIT License.
 */

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-synctex.h"
#include "tectonic_bridge_core.h"


#define DVI_BUF_SIZE 16384
#define HALF_BUF 8192
#define FNT_NUM_0 171 /* DVI code */

static rust_output_handle_t dvi_file;
static str_number output_file_name;
static eight_bits *dvi_buf = NULL;
static int32_t dvi_limit;
static int32_t g;
static int32_t lq, lr;
static int32_t dvi_ptr;
static int32_t dvi_offset;
static int32_t dvi_gone;
static int32_t down_ptr, right_ptr;
static scaled_t dvi_h, dvi_v;
static internal_font_number dvi_f;
static int32_t cur_s;


static void hlist_out(void);
static void vlist_out(void);
static int32_t reverse(int32_t this_box, int32_t t, scaled_t * cur_g, double * cur_glue);
static void dvi_native_font_def(internal_font_number f);
static void movement(scaled_t w, eight_bits o);
static void prune_movements(int32_t l);
static void special_out(int32_t p);
static void write_out(int32_t p);
static void pic_out(int32_t p);
static void write_to_dvi(int32_t a, int32_t b);
static void dvi_swap(void);
static void dvi_four(int32_t x);
static void dvi_two(UTF16_code s);
static void dvi_pop(int32_t l);
static void dvi_font_def(internal_font_number f);


void
initialize_shipout_variables(void)
{
    output_file_name = 0;
    dvi_buf = xmalloc_array(eight_bits, DVI_BUF_SIZE);
    dvi_limit = DVI_BUF_SIZE;
    dvi_ptr = 0;
    dvi_offset = 0;
    dvi_gone = 0;
    down_ptr = TEX_NULL;
    right_ptr = TEX_NULL;
    cur_s = -1;
}


void
deinitialize_shipout_variables(void)
{
    free(dvi_buf);
    dvi_buf = NULL;
}


static inline void
dvi_out(eight_bits c)
{
    dvi_buf[dvi_ptr++] = c;
    if (dvi_ptr == dvi_limit)
        dvi_swap();
}


/*660: output the box `p` */
void
ship_out(int32_t p)
{
    int32_t page_loc;
    unsigned char j, k;
    pool_pointer s;
    unsigned char old_setting;
    unsigned char l;
    const char *output_comment = "tectonic";

    synctex_sheet(INTPAR(mag));

    if (job_name == 0)
        open_log_file();

    if (INTPAR(tracing_output) > 0) {
        print_nl_cstr("");
        print_ln();
        print_cstr("Completed box being shipped out");
    }

    if (term_offset > max_print_line - 9)
        print_ln();
    else if (term_offset > 0 || file_offset > 0)
        print_char(' ' );

    print_char('[' );
    j = 9;
    while (j > 0 && COUNT_REG(j) == 0)
        j--;

    for (k = 0; k <= j; k++) {
        print_int(COUNT_REG(k));
        if (k < j)
            print_char('.' );
    }

    ttstub_output_flush(rust_stdout);

    if (INTPAR(tracing_output) > 0) {
        print_char(']' );
        begin_diagnostic();
        show_box(p);
        end_diagnostic(true);
    }

    /*662: "Ship box `p` out." */
    /*663: "Update the values of max_h and max_v; but if the page is too
     * large, goto done". */

    if (BOX_height(p) > MAX_HALFWORD ||
        BOX_depth(p) > MAX_HALFWORD ||
        BOX_height(p) + BOX_depth(p) + DIMENPAR(v_offset) > MAX_HALFWORD ||
        BOX_width(p) + DIMENPAR(h_offset) > MAX_HALFWORD)
    {
        error_here_with_diagnostic("Huge page cannot be shipped out");
        capture_to_diagnostic(NULL);
        help_ptr = 2;
        help_line[1] = "The page just created is more than 18 feet tall or";
        help_line[0] = "more than 18 feet wide, so I suspect something went wrong.";
        error();

        if (INTPAR(tracing_output) <= 0) {
            begin_diagnostic();
            print_nl_cstr("The following box has been deleted:");
            show_box(p);
            end_diagnostic(true);
        }
        goto done;
    }

    if (BOX_height(p) + BOX_depth(p) + DIMENPAR(v_offset) > max_v)
        max_v = BOX_height(p) + BOX_depth(p) + DIMENPAR(v_offset);

    if (BOX_width(p) + DIMENPAR(h_offset) > max_h)
        max_h = BOX_width(p) + DIMENPAR(h_offset);

    /*637: "Initialize variables as ship_out begins." */

    dvi_h = 0;
    dvi_v = 0;
    cur_h = DIMENPAR(h_offset);
    dvi_f = FONT_BASE;

    /*1405: "Calculate page dimensions and margins" */
    /* 4736287 = round(0xFFFF * 72.27) ; i.e., 1 inch expressed as a scaled_t */
    cur_h_offset = DIMENPAR(h_offset) + 4736287;
    cur_v_offset = DIMENPAR(v_offset) + 4736287;

    if (DIMENPAR(pdf_page_width) != 0)
        cur_page_width = DIMENPAR(pdf_page_width);
    else
        cur_page_width = BOX_width(p) + 2 * cur_h_offset;

    if (DIMENPAR(pdf_page_height) != 0)
        cur_page_height = DIMENPAR(pdf_page_height);
    else
        cur_page_height = BOX_height(p) + BOX_depth(p) + 2 * cur_v_offset;

    /* ... resuming 637 ... open up the DVI file if needed */

    if (output_file_name == 0) {
        if (job_name == 0)
            open_log_file();
        pack_job_name(output_file_extension);
        dvi_file = ttstub_output_open(name_of_file, 0);
        if (dvi_file == NULL)
            _tt_abort("cannot open output file \"%s\"", name_of_file);
        output_file_name = make_name_string();
    }

    /* First page? Emit preamble items. */

    if (total_pages == 0) {
        dvi_out(PRE);

        if (semantic_pagination_enabled)
            dvi_out(SPX_ID_BYTE);
        else
            dvi_out(XDV_ID_BYTE);

        dvi_four(25400000L); /* magic values: conversion ratio for sp */
        dvi_four(473628672L); /* magic values: conversion ratio for sp */

        prepare_mag();
        dvi_four(INTPAR(mag));

        l = strlen(output_comment);
        dvi_out(l);
        for (s = 0; s < l; s++)
            dvi_out(output_comment[s]);
    }

    /* ... resuming 662 ... Emit per-page preamble. */

    page_loc = dvi_offset + dvi_ptr;

    dvi_out(BOP);

    for (k = 0; k < 10; k++)
        dvi_four(COUNT_REG(k));

    dvi_four(last_bop);
    last_bop = page_loc;

    /* Generate a PDF pagesize special unilaterally */

    old_setting = selector;
    selector = SELECTOR_NEW_STRING;
    print_cstr("pdf:pagesize ");
    if (DIMENPAR(pdf_page_width) <= 0 || DIMENPAR(pdf_page_height) <= 0) {
        print_cstr("default");
    } else {
        print_cstr("width");
        print(' ' );
        print_scaled(DIMENPAR(pdf_page_width));
        print_cstr("pt");
        print(' ' );
        print_cstr("height");
        print(' ' );
        print_scaled(DIMENPAR(pdf_page_height));
        print_cstr("pt");
    }
    selector = old_setting;

    dvi_out(XXX1);
    dvi_out(cur_length());

    for (s = str_start[str_ptr - TOO_BIG_CHAR]; s < pool_ptr; s++)
        dvi_out(str_pool[s]);

    pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];

    /* Done with the synthesized special. The meat: emit this page box. */

    cur_v = BOX_height(p) + DIMENPAR(v_offset); /*"Does this need changing for upwards mode???"*/
    temp_ptr = p;
    if (NODE_type(p) == VLIST_NODE)
        vlist_out();
    else
        hlist_out();

    dvi_out(EOP);
    total_pages++;
    cur_s = -1;

done:
    /*1518: "Check for LR anomalies at the end of ship_out" */

    if (LR_problems > 0) {
        print_ln();
        print_nl_cstr("\\endL or \\endR problem (");
        print_int(LR_problems / 10000);
        print_cstr(" missing, ");
        print_int(LR_problems % 10000);
        print_cstr(" extra");
        LR_problems = 0;
        print_char(')');
        print_ln();
    }

    if (LR_ptr != TEX_NULL || cur_dir != LEFT_TO_RIGHT)
        confusion("LR3");

    if (INTPAR(tracing_output) <= 0)
        print_char(']');

    dead_cycles = 0;
    ttstub_output_flush(rust_stdout);
    flush_node_list(p);
    synctex_teehs();
}


/*639: Output an hlist */
static void
hlist_out(void)
{
    scaled_t base_line;
    scaled_t left_edge;
    scaled_t save_h, save_v;
    int32_t this_box;
    glue_ord g_order;
    unsigned char g_sign;
    int32_t p;
    int32_t save_loc;
    int32_t leader_box;
    scaled_t leader_wd;
    scaled_t lx;
    bool outer_doing_leaders;
    scaled_t edge;
    int32_t prev_p;
    int32_t len;
    int32_t q, r;
    int32_t k, j;
    double glue_temp;
    double cur_glue;
    scaled_t cur_g;
    uint16_t c;
    internal_font_number f;

    cur_g = 0;
    cur_glue = 0.0;
    this_box = temp_ptr;
    g_order = BOX_glue_order(this_box);
    g_sign = BOX_glue_sign(this_box);

    if (INTPAR(xetex_interword_space_shaping) > 1) {
        /*640: "Extra stuff for justifiable AAT text..." "Merge sequences of
         * words using native fonts and inter-word spaces into single
         * nodes" */

        p = BOX_list_ptr(this_box);
        prev_p = this_box + 5; /* this gets the list within the box */

        while (p != TEX_NULL) {
            if (LLIST_link(p) != TEX_NULL) {
                if (p != TEX_NULL
                    && !is_char_node(p)
                    && NODE_type(p) == WHATSIT_NODE
                    && (NODE_subtype(p) == NATIVE_WORD_NODE || NODE_subtype(p) == NATIVE_WORD_NODE_AT)
                    && font_letter_space[NATIVE_NODE_font(p)] == 0
                ) {
                    /* "got a word in an AAT font, might be the start of a run" */
                    r = p;
                    k = NATIVE_NODE_length(r);
                    q = LLIST_link(p);

                check_next:
                    /*641: "Advance `q` past ignorable nodes." This test is
                     * mostly `node_is_invisible_to_interword_space`. 641 is
                     * reused a few times here. */

                    while (q != TEX_NULL
                           && !is_char_node(q)
                           && (NODE_type(q) == PENALTY_NODE
                               || NODE_type(q) == INS_NODE
                               || NODE_type(q) == MARK_NODE
                               || NODE_type(q) == ADJUST_NODE
                               || (NODE_type(q) == WHATSIT_NODE && NODE_subtype(q) <= 4)
                    ))
                        q = LLIST_link(q);

                    if (q != TEX_NULL && !is_char_node(q)) {
                        if (NODE_type(q) == GLUE_NODE && GLUE_SPEC_shrink_order(q) == NORMAL) {
                            if (GLUE_NODE_glue_ptr(q) == font_glue[NATIVE_NODE_font(r)]) {
                                /* "Found a normal space; if the next node is
                                 * another word in the same font, we'll
                                 * merge." */

                                q = LLIST_link(q);

                                while (q != TEX_NULL && !is_char_node(q)
                                       && (NODE_type(q) == PENALTY_NODE
                                           || NODE_type(q) == INS_NODE
                                           || NODE_type(q) == MARK_NODE
                                           || NODE_type(q) == ADJUST_NODE
                                           || (NODE_type(q) == WHATSIT_NODE && NODE_subtype(q) <= 4)
                                           ))
                                    q = LLIST_link(q);

                                if (q != TEX_NULL
                                    && !is_char_node(q)
                                    && NODE_type(q) == WHATSIT_NODE
                                    && (NODE_subtype(q) == NATIVE_WORD_NODE || NODE_subtype(q) == NATIVE_WORD_NODE_AT)
                                    && (mem[q + 4].b16.s2 == mem[r + 4].b16.s2)
                                ) {
                                    p = q;
                                    k += 1 + NATIVE_NODE_length(q);
                                    q = LLIST_link(q);
                                    goto check_next;
                                }
                            } else {
                                q = LLIST_link(q);
                            }

                            if (q != TEX_NULL
                                && !is_char_node(q)
                                && NODE_type(q) == KERN_NODE
                                && NODE_subtype(q) == SPACE_ADJUSTMENT
                            ) {
                                q = LLIST_link(q);

                                while (q != TEX_NULL
                                       && !is_char_node(q)
                                       && (NODE_type(q) == PENALTY_NODE
                                           || NODE_type(q) == INS_NODE
                                           || NODE_type(q) == MARK_NODE
                                           || NODE_type(q) == ADJUST_NODE
                                           || (NODE_type(q) == WHATSIT_NODE && NODE_subtype(q) <= 4)
                                ))
                                    q = LLIST_link(q);

                                if (q != TEX_NULL
                                    && !is_char_node(q)
                                    && NODE_type(q) == WHATSIT_NODE
                                    && (NODE_subtype(q) == NATIVE_WORD_NODE || NODE_subtype(q) == NATIVE_WORD_NODE_AT)
                                    && NATIVE_NODE_font(q) == NATIVE_NODE_font(r)
                                ) {
                                    p = q;
                                    k += 1 + NATIVE_NODE_length(q);
                                    q = LLIST_link(q);
                                    goto check_next;
                                }
                            }
                            goto end_node_run;
                        }

                        if (q != TEX_NULL
                            && !is_char_node(q)
                            && NODE_type(q) == WHATSIT_NODE
                            && (NODE_subtype(q) == NATIVE_WORD_NODE || NODE_subtype(q) == NATIVE_WORD_NODE_AT)
                            && NATIVE_NODE_font(q) == NATIVE_NODE_font(r)
                        ) {
                            p = q;
                            q = LLIST_link(q);
                            goto check_next;
                        }
                    }

                end_node_run:
                    /* "Now r points to the first native_word_node of the run,
                     * and p to the last." */

                    if (p != r) {
                        if (pool_ptr + k > pool_size)
                            overflow("pool size", pool_size - init_pool_ptr);

                        k = 0;
                        q = r;

                        while (true) {
                            if (NODE_type(q) == WHATSIT_NODE) {
                                if (NODE_subtype(q) == NATIVE_WORD_NODE || NODE_subtype(q) == NATIVE_WORD_NODE_AT) {
                                    for (j = 0; j < NATIVE_NODE_length(q); j++) {
                                        str_pool[pool_ptr] = NATIVE_NODE_text(q)[j];
                                        pool_ptr++;
                                    }

                                    k += BOX_width(q);
                                }
                            } else if (NODE_type(q) == GLUE_NODE) {
                                str_pool[pool_ptr] = ' ';
                                pool_ptr++;
                                g = GLUE_NODE_glue_ptr(q);
                                k += BOX_width(g);

                                if (g_sign != NORMAL) {
                                    if (g_sign == STRETCHING) {
                                        if (GLUE_SPEC_stretch_order(g) == g_order)
                                            k += tex_round(BOX_glue_set(this_box) * GLUE_SPEC_stretch(g));
                                    } else {
                                        if (GLUE_SPEC_shrink_order(g) == g_order)
                                            k -= tex_round(BOX_glue_set(this_box) * GLUE_SPEC_shrink(g));
                                    }
                                }
                            } else if (NODE_type(q) == KERN_NODE) {
                                k += BOX_width(q);
                            }

                            if (q == p)
                                break;
                            else
                                q = LLIST_link(q);
                        }

                        q = new_native_word_node(NATIVE_NODE_font(r), cur_length());
                        NODE_subtype(q) = NODE_subtype(r);

                        for (j = 0; j < cur_length(); j++)
                            NATIVE_NODE_text(q)[j] = str_pool[str_start[str_ptr - TOO_BIG_CHAR] + j];

                        /* "Link q into the list in place of r...p" */

                        BOX_width(q) = k;
                        set_justified_native_glyphs(q);
                        LLIST_link(prev_p) = q;
                        LLIST_link(q) = LLIST_link(p);
                        LLIST_link(p) = TEX_NULL;
                        prev_p = r;
                        p = LLIST_link(r);

                        /* "Extract any 'invisible' nodes from the old list
                         * and insert them after the new node, so we don't
                         * lose them altogether. Note that the first node
                         * cannot be one of these, as we always start merging
                         * at a native_word node." */

                        while (p != TEX_NULL) {
                            if (!is_char_node(p)
                                && (NODE_type(p) == PENALTY_NODE
                                    || NODE_type(p) == INS_NODE
                                    || NODE_type(p) == MARK_NODE
                                    || NODE_type(p) == ADJUST_NODE
                                    || (NODE_type(p) == WHATSIT_NODE && NODE_subtype(p) <= 4))
                            ) {
                                LLIST_link(prev_p) = LLIST_link(p);
                                LLIST_link(p) = LLIST_link(q);
                                LLIST_link(q) = p;
                                q = p;
                            }

                            prev_p = p;
                            p = LLIST_link(p);
                        }

                        flush_node_list(r);
                        pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];
                        p = q;
                    }
                }

                prev_p = p;
            }

            p = LLIST_link(p);
        }
    }

    /* ... resuming 639 ... */

    p = BOX_list_ptr(this_box);
    cur_s++;
    if (cur_s > 0)
        dvi_out(PUSH);

    if (cur_s > max_push)
        max_push = cur_s;

    save_loc = dvi_offset + dvi_ptr;
    base_line = cur_v;
    prev_p = this_box + 5; /* this is list_offset, the offset of the box list pointer */

    /*1501: "Initialize hlist_out for mixed direction typesetting" */

    temp_ptr = get_avail();
    LLIST_info(temp_ptr) = BEFORE;
    LLIST_link(temp_ptr) = LR_ptr;
    LR_ptr = temp_ptr;

    if (BOX_lr_mode(this_box) == DLIST) {
        if (cur_dir == RIGHT_TO_LEFT) {
            cur_dir = LEFT_TO_RIGHT;
            cur_h -= BOX_width(this_box);
        } else {
            BOX_lr_mode(this_box) = 0;
        }
    }

    if (cur_dir == RIGHT_TO_LEFT && BOX_lr_mode(this_box) != REVERSED) {
        /*1508: "Reverse the complete hlist and set the subtype to reversed." */
        save_h = cur_h;
        temp_ptr = p;
        p = new_kern(0);
        SYNCTEX_tag(p, MEDIUM_NODE_SIZE) = 0; /* "SyncTeX: do nothing, it is too late" */
        LLIST_link(prev_p) = p;
        cur_h = 0;
        LLIST_link(p) = reverse(this_box, TEX_NULL, &cur_g, &cur_glue);
        BOX_width(p) = -cur_h;
        cur_h = save_h;
        BOX_lr_mode(this_box) = REVERSED;
    }

    /* ... resuming 639 ... */

    left_edge = cur_h;
    synctex_hlist(this_box);

    while (p != TEX_NULL) {
        /*642: "Output node `p` for `hlist_out` and move to the next node,
        * maintaining the condition `cur_v = base_line`." ... "We ought to
        * give special care to the efficiency [here] since it belongs to TeX's
        * inner loop. When a `char_node` is encountered, we save a little time
        * by processing several nodes in succession[.] The program uses the
        * fact that `set_char_0 = 0`. */

    reswitch:
        if (is_char_node(p)) {
            if (cur_h != dvi_h) {
                movement(cur_h - dvi_h, RIGHT1);
                dvi_h = cur_h;
            }

            if (cur_v != dvi_v) {
                movement(cur_v - dvi_v, DOWN1);
                dvi_v = cur_v;
            }

            do {
                f = CHAR_NODE_font(p);
                c = CHAR_NODE_character(p);
                if (p != LIG_TRICK && font_mapping[f] != NULL)
                    c = apply_tfm_font_mapping(font_mapping[f], c);

                if (f != dvi_f) {
                    /*643: "Change font dvi_f to f" */
                    if (!font_used[f]) {
                        dvi_font_def(f);
                        font_used[f] = true;
                    }

                    if (f <= 64) {
                        dvi_out(f + FNT_NUM_0 - 1);
                    } else if (f <= 256) {
                        dvi_out(FNT1);
                        dvi_out(f - 1);
                    } else {
                        dvi_out(FNT1 + 1);
                        dvi_out((f - 1) / 256);
                        dvi_out((f - 1) % 256);
                    }

                    dvi_f = f;
                }

                if (font_ec[f] >= c) {
                    if (font_bc[f] <= c) {
                        if (FONT_CHARACTER_INFO(f, c).s3 > 0) { /* if (char_exists(orig_char_info(f)(c))) */
                            if (c >= 128)
                                dvi_out(SET1);
                            dvi_out(c);
                            cur_h += FONT_CHARACTER_WIDTH(f, c);
                        }
                    }
                }

                prev_p = LLIST_link(prev_p);
                p = LLIST_link(p);
            } while (is_char_node(p));

            synctex_current();
            dvi_h = cur_h;
        } else {
            /*644: "Output the non-char_node `p` and move to the next node" */

            switch (NODE_type(p)) {
            case HLIST_NODE:
            case VLIST_NODE:
                if (BOX_list_ptr(p) == TEX_NULL) {
                    if (NODE_type(p) == VLIST_NODE) {
                        synctex_void_vlist(p, this_box);
                    } else {
                        synctex_void_hlist(p, this_box);
                    }
                    cur_h += BOX_width(p);
                } else {
                    save_h = dvi_h;
                    save_v = dvi_v;
                    cur_v = base_line + BOX_shift_amount(p);
                    temp_ptr = p;
                    edge = cur_h + BOX_width(p);

                    if (cur_dir == RIGHT_TO_LEFT)
                        cur_h = edge;

                    if (NODE_type(p) == VLIST_NODE)
                        vlist_out();
                    else
                        hlist_out();

                    dvi_h = save_h;
                    dvi_v = save_v;
                    cur_h = edge;
                    cur_v = base_line;
                }
                break;

            case RULE_NODE:
                rule_ht = BOX_height(p);
                rule_dp = BOX_depth(p);
                rule_wd = BOX_width(p);
                goto fin_rule;
                break;

            case WHATSIT_NODE:
                /*1407: "Output the whatsit node p in an hlist" */
                switch (NODE_subtype(p)) {
                case NATIVE_WORD_NODE:
                case NATIVE_WORD_NODE_AT:
                case GLYPH_NODE:
                    if (cur_h != dvi_h) {
                        movement(cur_h - dvi_h, RIGHT1);
                        dvi_h = cur_h;
                    }

                    if (cur_v != dvi_v) {
                        movement(cur_v - dvi_v, DOWN1);
                        dvi_v = cur_v;
                    }

                    f = NATIVE_NODE_font(p);

                    if (f != dvi_f) {
                        if (!font_used[f]) {
                            dvi_font_def(f);
                            font_used[f] = true;
                        }

                        if (f <= 64) {
                            dvi_out(f + 170);
                        } else if (f <= 256) {
                            dvi_out(FNT1);
                            dvi_out(f - 1);
                        } else {
                            dvi_out(FNT1 + 1);
                            dvi_out((f - 1) / 256);
                            dvi_out((f - 1) % 256);
                        }

                        dvi_f = f;
                    }

                    if (NODE_subtype(p) == GLYPH_NODE) {
                        dvi_out(SET_GLYPHS);
                        dvi_four(BOX_width(p));
                        dvi_two(1); /* glyph count */
                        dvi_four(0); /* x offset, as fixed-point */
                        dvi_four(0); /* y offset, as fixed-point */
                        dvi_two(NATIVE_NODE_glyph(p));
                        cur_h += BOX_width(p);
                    } else {
                        if (NODE_subtype(p) == NATIVE_WORD_NODE_AT) {
                            if (NATIVE_NODE_length(p) > 0 || NATIVE_NODE_glyph_info_ptr(p) != NULL) {
                                dvi_out(SET_TEXT_AND_GLYPHS);
                                len = NATIVE_NODE_length(p);
                                dvi_two(len);

                                for (k = 0; k < len; k++)
                                            dvi_two(NATIVE_NODE_text(p)[k]);

                                len = make_xdv_glyph_array_data(p);

                                for (k = 0; k < len; k++)
                                    dvi_out(xdv_buffer[k]);
                            }
                        } else {
                            if (NATIVE_NODE_glyph_info_ptr(p) != NULL) {
                                dvi_out(SET_GLYPHS);
                                len = make_xdv_glyph_array_data(p);

                                for (k = 0; k < len; k++)
                                    dvi_out(xdv_buffer[k]);
                            }
                        }

                        cur_h += BOX_width(p);
                    }

                    dvi_h = cur_h;
                    break;

                case PIC_NODE:
                case PDF_NODE:
                    save_h = dvi_h;
                    save_v = dvi_v;
                    cur_v = base_line;
                    edge = cur_h + BOX_width(p);
                    pic_out(p);
                    dvi_h = save_h;
                    dvi_v = save_v;
                    cur_h = edge;
                    cur_v = base_line;
                    break;

                case PDF_SAVE_POS_NODE:
                    /* These magic numbers are in the original XeTeX source. */
                    pdf_last_x_pos = cur_h + 4736286L;
                    pdf_last_y_pos = cur_page_height - cur_v - 4736286L;
                    break;

                default:
                    out_what(p);
                    break;
                }
                break; /* end of WHATSIT_NODE case */

            case GLUE_NODE:
                /*647: "Move right or output leaders" */
                g = GLUE_NODE_glue_ptr(p);
                rule_wd = BOX_width(g) - cur_g;

                if (g_sign != NORMAL) {
                    if (g_sign == STRETCHING) {
                        if (GLUE_SPEC_stretch_order(g) == g_order) {
                            cur_glue += GLUE_SPEC_stretch(g);
                            glue_temp = BOX_glue_set(this_box) * cur_glue;

                            if (glue_temp > 1000000000.0)
                                glue_temp = 1000000000.0;
                            else if (glue_temp < -1000000000.0)
                                glue_temp = -1000000000.0;

                            cur_g = tex_round(glue_temp);
                        }
                    } else if (GLUE_SPEC_shrink_order(g) == g_order) {
                        cur_glue -= GLUE_SPEC_shrink(g);
                        glue_temp = BOX_glue_set(this_box) * cur_glue;

                        if (glue_temp > 1000000000.0)
                            glue_temp = 1000000000.0;
                        else if (glue_temp < -1000000000.0)
                            glue_temp = -1000000000.0;

                        cur_g = tex_round(glue_temp);
                    }
                }

                rule_wd += cur_g;

                /*1486: "Handle a glue node for mixed direction typesetting". */

                if ((g_sign == STRETCHING && GLUE_SPEC_stretch_order(g) == g_order) ||
                    (g_sign == SHRINKING && GLUE_SPEC_shrink_order(g) == g_order)) {
                    if (GLUE_SPEC_ref_count(g) == TEX_NULL)
                        free_node(g, GLUE_SPEC_SIZE);
                    else
                        GLUE_SPEC_ref_count(g)--;

                    if (NODE_subtype(p) < A_LEADERS) {
                        NODE_type(p) = KERN_NODE;
                        BOX_width(p) = rule_wd;
                    } else {
                        g = get_node(GLUE_SPEC_SIZE);
                        GLUE_SPEC_stretch_order(g) = FILLL + 1; /* "will never match" */
                        GLUE_SPEC_shrink_order(g) = FILLL + 1;
                        BOX_width(g) = rule_wd;
                        GLUE_SPEC_stretch(g) = 0;
                        GLUE_SPEC_shrink(g) = 0;
                        GLUE_NODE_glue_ptr(p) = g;
                    }
                }

                if (NODE_subtype(p) >= A_LEADERS) {
                    /*648: "Output leaders into an hlist, goto fin_rule if a
                     * rule or next_p if done." */

                    leader_box = GLUE_NODE_leader_ptr(p);

                    if (NODE_type(leader_box) == RULE_NODE) {
                        rule_ht = BOX_height(leader_box);
                        rule_dp = BOX_depth(leader_box);
                        goto fin_rule;
                    }

                    leader_wd = BOX_width(leader_box);

                    if (leader_wd > 0 && rule_wd > 0) {
                        rule_wd += 10; /* "compensate for floating-point rounding" ?? */

                        if (cur_dir == RIGHT_TO_LEFT)
                            cur_h -= 10;

                        edge = cur_h + rule_wd;
                        lx = 0;

                        /*649: "Let cur_h be the position of the first pox,
                         * and set leader_wd + lx to the spacing between
                         * corresponding parts of boxes". Additional
                         * explanator comments in XTTP. */

                        if (NODE_subtype(p) == A_LEADERS) {
                            save_h = cur_h;
                            cur_h = left_edge + leader_wd * ((cur_h - left_edge) / leader_wd);
                            if (cur_h < save_h)
                                cur_h = cur_h + leader_wd;
                        } else {
                            lq = rule_wd / leader_wd;
                            lr = rule_wd % leader_wd;

                            if (NODE_subtype(p) == C_LEADERS)
                                cur_h = cur_h + (lr / 2);
                            else {
                                lx = lr / (lq + 1);
                                cur_h = cur_h + ((lr - (lq - 1) * lx) / 2);
                            }
                        }

                        while (cur_h + leader_wd <= edge) {
                            /*650: "Output a leader box at cur_h, then advance cur_h by leader_wd + lx" */

                            cur_v = base_line + BOX_shift_amount(leader_box);

                            if (cur_v != dvi_v) {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }

                            save_v = dvi_v;

                            if (cur_h != dvi_h) {
                                movement(cur_h - dvi_h, RIGHT1);
                                dvi_h = cur_h;
                            }

                            save_h = dvi_h;

                            temp_ptr = leader_box;
                            if (cur_dir == RIGHT_TO_LEFT)
                                cur_h += leader_wd;

                            outer_doing_leaders = doing_leaders;
                            doing_leaders = true;

                            if (NODE_type(leader_box) == VLIST_NODE)
                                vlist_out();
                            else
                                hlist_out();

                            doing_leaders = outer_doing_leaders;
                            dvi_v = save_v;
                            dvi_h = save_h;
                            cur_v = base_line;
                            cur_h = save_h + leader_wd + lx;
                        }

                        if (cur_dir == RIGHT_TO_LEFT)
                            cur_h = edge;
                        else
                            cur_h = edge - 10;

                        goto next_p;
                    }
                }

                goto move_past;
                break; /* end GLUE_NODE case */

            case MARGIN_KERN_NODE:
                cur_h += BOX_width(p);
                break;

            case KERN_NODE:
                synctex_kern(p, this_box);
                cur_h += BOX_width(p);
                break;

            case MATH_NODE:
                synctex_math(p, this_box);

                /* 1504: "Adjust the LR stack...; if necessary reverse and
                 * hlist segment and goto reswitch." "Breaking a paragraph
                 * into lines while TeXXeT is disabled may result in lines
                 * with unpaired math nodes. Such hlists are silently accepted
                 * in the absence of text direction directives." */

                if (odd(NODE_subtype(p))) { /* <= this is end_LR(p) */
                    if (LLIST_info(LR_ptr) == MATH_NODE_end_lr_type(p)) {
                        temp_ptr = LR_ptr;
                        LR_ptr = LLIST_link(temp_ptr);
                        LLIST_link(temp_ptr) = avail;
                        avail = temp_ptr;
                    } else {
                        if (NODE_subtype(p) > L_CODE)
                            LR_problems++;
                    }
                } else {
                    temp_ptr = get_avail();
                    LLIST_info(temp_ptr) = MATH_NODE_end_lr_type(p);
                    LLIST_link(temp_ptr) = LR_ptr;
                    LR_ptr = temp_ptr;

                    if (MATH_NODE_lr_dir(p) != cur_dir) {
                        /*1509: "Reverse an hlist segment and goto reswitch" */
                        save_h = cur_h;
                        temp_ptr = LLIST_link(p);
                        rule_wd = BOX_width(p);
                        free_node(p, MEDIUM_NODE_SIZE);
                        cur_dir = 1 - cur_dir;
                        p = new_edge(cur_dir, rule_wd);
                        LLIST_link(prev_p) = p;
                        cur_h = cur_h - left_edge + rule_wd;
                        LLIST_link(p) = reverse(this_box, new_edge(1 - cur_dir, 0), &cur_g, &cur_glue);
                        EDGE_NODE_edge_dist(p) = cur_h;
                        cur_dir = 1 - cur_dir;
                        cur_h = save_h;
                        goto reswitch;
                    }
                }

                NODE_type(p) = KERN_NODE;
                cur_h += BOX_width(p);
                break;

            case LIGATURE_NODE:
                /* 675: "Make node p look like a char_node and goto reswitch" */
                mem[LIG_TRICK] = mem[p + 1]; /* = lig_char(p) */
                LLIST_link(LIG_TRICK) = LLIST_link(p);
                p = LIG_TRICK;
                xtx_ligature_present = true;
                goto reswitch;
                break;

            /*1507: "Cases of hlist_out that arise in mixed direction text only" */
            case EDGE_NODE:
                cur_h += BOX_width(p);
                left_edge = cur_h + EDGE_NODE_edge_dist(p);
                cur_dir = NODE_subtype(p);
                break;

            default:
                break;
            }

            goto next_p;

        fin_rule:
            /*646: "Output a rule in an hlist" */
            if (rule_ht == NULL_FLAG)
                rule_ht = BOX_height(this_box);

            if (rule_dp == NULL_FLAG)
                rule_dp = BOX_depth(this_box);

            rule_ht += rule_dp;

            if (rule_ht > 0 && rule_wd > 0) {
                if (cur_h != dvi_h) {
                    movement(cur_h - dvi_h, RIGHT1);
                    dvi_h = cur_h;
                }

                cur_v = base_line + rule_dp;

                if (cur_v != dvi_v) {
                    movement(cur_v - dvi_v, DOWN1);
                    dvi_v = cur_v;
                }

                dvi_out(SET_RULE);
                dvi_four(rule_ht);
                dvi_four(rule_wd);
                cur_v = base_line;
                dvi_h += rule_wd;
            }

            /* ... resuming 644 ... */
        move_past:
            cur_h += rule_wd;
            synctex_horizontal_rule_or_glue(p, this_box);

        next_p:
            prev_p = p;
            p = LLIST_link(p);
        }
    }

    synctex_tsilh(this_box);

    /*1502: "Finish hlist_out for mixed direction typesetting" */
    /*1505: "Check for LR anomalies" */

    while (LLIST_info(LR_ptr) != BEFORE) {
        if (LLIST_info(LR_ptr) > L_CODE)
            LR_problems += 10000;

        temp_ptr = LR_ptr;
        LR_ptr = LLIST_link(temp_ptr);
        LLIST_link(temp_ptr) = avail;
        avail = temp_ptr;
    }

    temp_ptr = LR_ptr;
    LR_ptr = LLIST_link(temp_ptr);
    LLIST_link(temp_ptr) = avail;
    avail = temp_ptr;

    if (BOX_lr_mode(this_box) == DLIST)
        cur_dir = RIGHT_TO_LEFT;

    /* ... finishing 639 */

    prune_movements(save_loc);
    if (cur_s > 0)
        dvi_pop(save_loc);
    cur_s--;
}


/*651: "When vlist_out is called, its duty is to output the box represented by
 * the vlist_node pointed to by temp_ptr. The reference point of that box has
 * coordinates (cur_h, cur_v)." */
static void
vlist_out(void)
{
    scaled_t left_edge;
    scaled_t top_edge;
    scaled_t save_h, save_v;
    int32_t this_box;
    glue_ord g_order;
    unsigned char g_sign;
    int32_t p;
    int32_t save_loc;
    int32_t leader_box;
    scaled_t leader_ht;
    scaled_t lx;
    bool outer_doing_leaders;
    scaled_t edge;
    double glue_temp;
    double cur_glue;
    scaled_t cur_g;
    bool upwards;
    internal_font_number f;

    cur_g = 0;
    cur_glue = 0.0;
    this_box = temp_ptr;
    g_order = BOX_glue_order(this_box);
    g_sign = BOX_glue_sign(this_box);
    p = BOX_list_ptr(this_box);
    upwards = (NODE_subtype(this_box) == 1);

    cur_s++;
    if (cur_s > 0)
        dvi_out(PUSH);

    if (cur_s > max_push)
        max_push = cur_s;

    save_loc = dvi_offset + dvi_ptr;
    left_edge = cur_h;
    synctex_vlist(this_box);

    if (upwards)
        cur_v += BOX_depth(this_box);
    else
        cur_v -= BOX_height(this_box);

    top_edge = cur_v;

    while (p != TEX_NULL) {
        /*652: "Output node p and move to the next node, maintaining the
         * condition cur_h = left_edge" */

        if (is_char_node(p))
            confusion("vlistout");
        else {
            /*653: "Output the non-char_node p" */

            switch (NODE_type(p)) {
            case HLIST_NODE:
            case VLIST_NODE:
                /*654: "Output a box in a vlist" */
                if (BOX_list_ptr(p) == TEX_NULL) {
                    if (upwards)
                        cur_v -= BOX_depth(p);
                    else
                        cur_v += BOX_height(p);

                    if (NODE_type(p) == VLIST_NODE) {
                        synctex_void_vlist(p, this_box);
                    } else {
                        synctex_void_hlist(p, this_box);
                    }

                    if (upwards)
                        cur_v -= BOX_height(p);
                    else
                        cur_v += BOX_depth(p);
                } else {
                    if (upwards)
                        cur_v -= BOX_depth(p);
                    else
                        cur_v += BOX_height(p);

                    if (cur_v != dvi_v) {
                        movement(cur_v - dvi_v, DOWN1);
                        dvi_v = cur_v;
                    }

                    save_h = dvi_h;
                    save_v = dvi_v;

                    if (cur_dir == RIGHT_TO_LEFT)
                        cur_h = left_edge - BOX_shift_amount(p);
                    else
                        cur_h = left_edge + BOX_shift_amount(p);

                    temp_ptr = p;
                    if (NODE_type(p) == VLIST_NODE)
                        vlist_out();
                    else
                        hlist_out();

                    dvi_h = save_h;
                    dvi_v = save_v;

                    if (upwards)
                        cur_v = save_v - BOX_height(p);
                    else
                        cur_v = save_v + BOX_depth(p);

                    cur_h = left_edge;
                }
                break;

            case RULE_NODE:
                rule_ht = BOX_height(p);
                rule_dp = BOX_depth(p);
                rule_wd = BOX_width(p);
                goto fin_rule;
                break;

            case WHATSIT_NODE:
                /*1403: "Output the whatsit node p in a vlist" */
                switch (NODE_subtype(p)) {
                case GLYPH_NODE:
                    cur_v = cur_v + BOX_height(p);
                    cur_h = left_edge;

                    if (cur_h != dvi_h) {
                        movement(cur_h - dvi_h, RIGHT1);
                        dvi_h = cur_h;
                    }

                    if (cur_v != dvi_v) {
                        movement(cur_v - dvi_v, DOWN1);
                        dvi_v = cur_v;
                    }

                    f = NATIVE_NODE_font(p);

                    if (f != dvi_f) {
                        /*643:*/
                        if (!font_used[f]) {
                            dvi_font_def(f);
                            font_used[f] = true;
                        }

                        if (f <= 64) {
                            dvi_out(f + 170);
                        } else if (f <= 256) {
                            dvi_out(FNT1);
                            dvi_out(f - 1);
                        } else {
                            dvi_out(FNT1 + 1);
                            dvi_out((f - 1) / 256);
                            dvi_out((f - 1) % 256);
                        }

                        dvi_f = f;
                    }

                    dvi_out(SET_GLYPHS);
                    dvi_four(0); /* width */
                    dvi_two(1); /* glyph count */
                    dvi_four(0); /* x offset as fixed-point */
                    dvi_four(0); /* y offset as fixed-point */
                    dvi_two(NATIVE_NODE_glyph(p));

                    cur_v += BOX_depth(p);
                    cur_h = left_edge;
                    break;

                case PIC_NODE:
                case PDF_NODE:
                    save_h = dvi_h;
                    save_v = dvi_v;
                    cur_v = cur_v + BOX_height(p);
                    pic_out(p);
                    dvi_h = save_h;
                    dvi_v = save_v;
                    cur_v = save_v + BOX_depth(p);
                    cur_h = left_edge;
                    break;

                case PDF_SAVE_POS_NODE:
                    /* These magic numbers are in the original XeTeX source. */
                    pdf_last_x_pos = cur_h + 4736286L;
                    pdf_last_y_pos = cur_page_height - cur_v - 4736286L;
                    break;

                default:
                    out_what(p);
                    break;
                }
                break; /* end WHATSIT_NODE case */

            case GLUE_NODE:
                /*656: "Move down or output leaders" */
                g = GLUE_NODE_glue_ptr(p);
                rule_ht = BOX_width(g) - cur_g;

                if (g_sign != NORMAL) {
                    if (g_sign == STRETCHING) {
                        if (GLUE_SPEC_stretch_order(g) == g_order) {
                            cur_glue += GLUE_SPEC_stretch(g);
                            glue_temp = BOX_glue_set(this_box) * cur_glue;

                            if (glue_temp > 1000000000.0)
                                glue_temp = 1000000000.0;
                            else if (glue_temp < -1000000000.0)
                                glue_temp = -1000000000.0;

                            cur_g = tex_round(glue_temp);
                        }
                    } else if (GLUE_SPEC_shrink_order(g) == g_order) {
                        cur_glue -= GLUE_SPEC_shrink(g);
                        glue_temp = BOX_glue_set(this_box) * cur_glue;

                        if (glue_temp > 1000000000.0)
                            glue_temp = 1000000000.0;
                        else if (glue_temp < -1000000000.0)
                            glue_temp = -1000000000.0;

                        cur_g = tex_round(glue_temp);
                    }
                }

                rule_ht += cur_g;

                if (NODE_subtype(p) >= A_LEADERS) {
                    /*657: "Output leaders in a vlist, goto fin_rule if a rule
                     * or next_p if done" */

                    leader_box = GLUE_NODE_leader_ptr(p);

                    if (NODE_type(leader_box) == RULE_NODE) {
                        rule_wd = BOX_width(leader_box);
                        rule_dp = 0;
                        goto fin_rule;
                    }

                    leader_ht = BOX_height(leader_box) + BOX_depth(leader_box);

                    if (leader_ht > 0 && rule_ht > 0) {
                        rule_ht += 10; /* "compensate for floating-point rounding" */
                        edge = cur_v + rule_ht;
                        lx = 0;

                        /*658: "Let cur_v be the position of the first box,
                         * and set leader_ht + lx to the spacing between
                         * corresponding parts of boxes" */

                        if (NODE_subtype(p) == A_LEADERS) {
                            save_v = cur_v;
                            cur_v = top_edge + leader_ht * ((cur_v - top_edge) / leader_ht);

                            if (cur_v < save_v)
                                cur_v = cur_v + leader_ht;
                        } else {
                            lq = rule_ht / leader_ht;
                            lr = rule_ht % leader_ht;

                            if (mem[p].b16.s0 == C_LEADERS)
                                cur_v = cur_v + (lr / 2);
                            else {
                                lx = lr / (lq + 1);
                                cur_v = cur_v + ((lr - (lq - 1) * lx) / 2);
                            }
                        }

                        while (cur_v + leader_ht <= edge) {
                            /*659: "Output a leader box at cur_v, then advance
                             * cur_v by leader_ht + lx". "When we reach this
                             * part of the program, cur_v indicates the top of
                             * a leader box, not its baseline." */

                            if (cur_dir == RIGHT_TO_LEFT)
                                cur_h = left_edge - BOX_shift_amount(leader_box);
                            else
                                cur_h = left_edge + BOX_shift_amount(leader_box);

                            if (cur_h != dvi_h) {
                                movement(cur_h - dvi_h, RIGHT1);
                                dvi_h = cur_h;
                            }

                            save_h = dvi_h;
                            cur_v += BOX_height(leader_box);

                            if (cur_v != dvi_v) {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }

                            save_v = dvi_v;
                            temp_ptr = leader_box;
                            outer_doing_leaders = doing_leaders;
                            doing_leaders = true;

                            if (NODE_type(leader_box) == VLIST_NODE)
                                vlist_out();
                            else
                                hlist_out();

                            doing_leaders = outer_doing_leaders;
                            dvi_v = save_v;
                            dvi_h = save_h;
                            cur_h = left_edge;
                            cur_v = save_v - BOX_height(leader_box) + leader_ht + lx;
                        }

                        cur_v = edge - 10;
                        goto next_p;
                    }
                }

                goto move_past;
                break;

            case KERN_NODE:
                if (upwards)
                    cur_v -= BOX_width(p);
                else
                    cur_v += BOX_width(p);
                break;

            default:
                break;
            }

            goto next_p;

        fin_rule:
            /*655: "Output a rule in a vlist, goto next_p */
            if (rule_wd == NULL_FLAG)
                rule_wd = BOX_width(this_box);

            rule_ht += rule_dp;

            if (upwards)
                cur_v -= rule_ht;
            else
                cur_v += rule_ht;

            if (rule_ht > 0 && rule_wd > 0) {
                if (cur_dir == RIGHT_TO_LEFT)
                    cur_h -= rule_wd;

                if (cur_h != dvi_h) {
                    movement(cur_h - dvi_h, RIGHT1);
                    dvi_h = cur_h;
                }

                if (cur_v != dvi_v) {
                    movement(cur_v - dvi_v, DOWN1);
                    dvi_v = cur_v;
                }

                dvi_out(PUT_RULE);
                dvi_four(rule_ht);
                dvi_four(rule_wd);
                cur_h = left_edge;
            }

            goto next_p;

        move_past:
            if (upwards)
                cur_v -= rule_ht;
            else
                cur_v += rule_ht;
        }

    next_p:
        p = LLIST_link(p);
    }

    synctex_tsilv(this_box);
    prune_movements(save_loc);

    if (cur_s > 0)
        dvi_pop(save_loc);
    cur_s--;
}


/*1510: "The reverse function defined here is responsible for reversing the
 * nodes of an hlist (segment). this_box is the enclosing hlist_node; t is to
 * become the tail of the reversed list; and the global variable temp_ptr is
 * the head of the list to be reversed. cur_g and cur_glue are the current
 * glue rounding state variables, to be updated by this function. We remove
 * nodes from the original list and add them to the head of the new one."
 */
static int32_t
reverse(int32_t this_box, int32_t t, scaled_t *cur_g, double *cur_glue)
{
    int32_t l;
    int32_t p;
    int32_t q;
    glue_ord g_order;
    unsigned char g_sign;
    double glue_temp;
    int32_t m, n;
    uint16_t c;
    internal_font_number f;

    g_order = BOX_glue_order(this_box);
    g_sign = BOX_glue_sign(this_box);
    l = t;
    p = temp_ptr;
    m = MIN_HALFWORD;
    n = MIN_HALFWORD;

    while (true) {
        while (p != TEX_NULL) {
            /*1511: "Move node p to the new list and go to the next node; or
             * goto done if the end of the reflected segment has been
             * reached." */

        reswitch:
            if (is_char_node(p)) {
                do {
                    f = CHAR_NODE_font(p);
                    c = CHAR_NODE_character(p);
                    cur_h += FONT_CHARACTER_WIDTH(f, effective_char(true, f, c));
                    q = LLIST_link(p);
                    LLIST_link(p) = l;
                    l = p;
                    p = q;
                } while (is_char_node(p));
            } else {
                q = LLIST_link(p);
                switch (NODE_type(p)) {
                case HLIST_NODE:
                case VLIST_NODE:
                case RULE_NODE:
                case KERN_NODE:
                    rule_wd = BOX_width(p);
                    break;

                case WHATSIT_NODE:
                    if (NODE_subtype(p) == NATIVE_WORD_NODE
                        || NODE_subtype(p) == NATIVE_WORD_NODE_AT
                        || NODE_subtype(p) == GLYPH_NODE
                        || NODE_subtype(p) == PIC_NODE
                        || NODE_subtype(p) == PDF_NODE
                    )
                        rule_wd = BOX_width(p);
                    else
                        goto next_p;
                    break;

                case GLUE_NODE:
                    /*1486: "Handle a glue node for mixed direction typesetting" */
                    g = GLUE_NODE_glue_ptr(p);
                    rule_wd = BOX_width(g) - *cur_g;

                    if (g_sign != NORMAL) {
                        if (g_sign == STRETCHING) {
                            if (GLUE_SPEC_stretch_order(g) == g_order) {
                                *cur_glue = *cur_glue + GLUE_SPEC_stretch(g);
                                glue_temp = BOX_glue_set(this_box) * *cur_glue;

                                if (glue_temp > 1000000000.0)
                                    glue_temp = 1000000000.0;
                                else if (glue_temp < -1000000000.0)
                                    glue_temp = -1000000000.0;

                                *cur_g = tex_round(glue_temp);
                            }
                        } else if (GLUE_SPEC_shrink_order(g) == g_order) {
                            *cur_glue = *cur_glue - GLUE_SPEC_shrink(g);
                            glue_temp = BOX_glue_set(this_box) * *cur_glue;

                            if (glue_temp > 1000000000.0)
                                glue_temp = 1000000000.0;
                            else if (glue_temp < -1000000000.0)
                                glue_temp = -1000000000.0;

                            *cur_g = tex_round(glue_temp);
                        }
                    }

                    rule_wd += *cur_g;

                    if ((g_sign == STRETCHING && mem[g].b16.s1 == g_order)
                         || (g_sign == SHRINKING && mem[g].b16.s0 == g_order)) {
                        if (GLUE_SPEC_ref_count(g) == TEX_NULL)
                            free_node(g, GLUE_SPEC_SIZE);
                        else
                            GLUE_SPEC_ref_count(g)--;

                        if (NODE_subtype(p) < A_LEADERS) {
                            NODE_type(p) = KERN_NODE;
                            BOX_width(p) = rule_wd;
                        } else {
                            g = get_node(GLUE_SPEC_SIZE);
                            GLUE_SPEC_stretch_order(g) = FILLL + 1; /* "will never match" */
                            GLUE_SPEC_shrink_order(g) = FILLL + 1;
                            BOX_width(g) = rule_wd;
                            GLUE_SPEC_stretch(g) = 0;
                            GLUE_SPEC_shrink(g) = 0;
                            GLUE_NODE_glue_ptr(p) = g;
                        }
                    }
                    break; /* end GLUE_NODE case */

                case LIGATURE_NODE:
                    flush_node_list(LIGATURE_NODE_lig_ptr(p));
                    temp_ptr = p;
                    p = get_avail();
                    mem[p] = mem[temp_ptr + 1]; /* = mem[lig_char(temp_ptr)] */
                    LLIST_link(p) = q;
                    free_node(temp_ptr, SMALL_NODE_SIZE);
                    goto reswitch;

                case MATH_NODE:
                    /*1516: "Math nodes in an inner reflected segment are
                     * modified, those at the outer level are changed into
                     * kern nodes." */
                    rule_wd = BOX_width(p);

                    if (odd(BOX_lr_mode(p))) {
                        if (LLIST_info(LR_ptr) != MATH_NODE_end_lr_type(p)) {
                            NODE_type(p) = KERN_NODE;
                            LR_problems++;
                        } else {
                            temp_ptr = LR_ptr;
                            LR_ptr = LLIST_link(temp_ptr);
                            LLIST_link(temp_ptr) = avail;
                            avail = temp_ptr;

                            if (n > MIN_HALFWORD) {
                                n--;
                                NODE_subtype(p)--;
                            } else {
                                NODE_type(p) = KERN_NODE;

                                if (m > MIN_HALFWORD) {
                                    m--;
                                } else {
                                    /*1517: "Finish the reverse hlist segment and goto done" */
                                    free_node(p, MEDIUM_NODE_SIZE);
                                    LLIST_link(t) = q;
                                    BOX_width(t) = rule_wd;
                                    EDGE_NODE_edge_dist(t) = -cur_h - rule_wd;
                                    goto done;
                                }
                            }
                        }
                    } else {
                        temp_ptr = get_avail();
                        LLIST_info(temp_ptr) = MATH_NODE_end_lr_type(p);
                        LLIST_link(temp_ptr) = LR_ptr;
                        LR_ptr = temp_ptr;

                        if (n > MIN_HALFWORD || MATH_NODE_lr_dir(p) != cur_dir) {
                            n++;
                            NODE_subtype(p)++;
                        } else {
                            NODE_type(p) = KERN_NODE;
                            m++;
                        }
                    }
                    break;

                case EDGE_NODE:
                    confusion("LR2");
                    break;

                default:
                    goto next_p;
                }

                cur_h += rule_wd;

            next_p:
                LLIST_link(p) = l;

                if (NODE_type(p) == KERN_NODE) {
                    if (rule_wd == 0 || l == TEX_NULL) {
                        free_node(p, MEDIUM_NODE_SIZE);
                        p = l;
                    }
                }

                l = p;
                p = q;
            }
        }

        /* ... resuming 1510 ... */

        if (t == TEX_NULL && m == MIN_HALFWORD && n == MIN_HALFWORD)
            goto done;

        p = new_math(0, LLIST_info(LR_ptr)); /* "Manufacture a missing math node" */
        LR_problems += 10000;
    }

done:
    return l;
}


/*1506: Create a new edge node of subtype `s` and width `w` */
int32_t
new_edge(small_number s, scaled_t w)
{
    int32_t p;

    p = get_node(EDGE_NODE_SIZE);
    NODE_type(p) = EDGE_NODE;
    NODE_subtype(p) = s;
    BOX_width(p) = w;
    EDGE_NODE_edge_dist(p) = 0;
    return p;
}


void
out_what(int32_t p)
{
    small_number j;
    unsigned char old_setting;

    switch (mem[p].b16.s0) {
    case OPEN_NODE:
    case WRITE_NODE:
    case CLOSE_NODE:
        if (doing_leaders)
            break;

        j = mem[p + 1].b32.s0;
        if (mem[p].b16.s0 == WRITE_NODE) {
            write_out(p);
            break;
        }

        if (write_open[j])
            ttstub_output_close(write_file[j]);

        if (mem[p].b16.s0 == CLOSE_NODE) {
            write_open[j] = false;
            break;
        }

        /* By this point must be OPEN_NODE */

        if (j >= 16)
            break;

        cur_name = mem[p + 1].b32.s1;
        cur_area = mem[p + 2].b32.s0;
        cur_ext = mem[p + 2].b32.s1;
        if (length(cur_ext) == 0)
            cur_ext = maketexstring(".tex");

        pack_file_name(cur_name, cur_area, cur_ext);

        write_file[j] = ttstub_output_open(name_of_file, 0);
        if (write_file[j] == NULL)
            _tt_abort("cannot open output file \"%s\"", name_of_file);

        write_open[j] = true;

        if (log_opened) {
            old_setting = selector;
            if (INTPAR(tracing_online) <= 0)
                selector = SELECTOR_LOG_ONLY;
            else
                selector = SELECTOR_TERM_AND_LOG;
            print_nl_cstr("\\openout");
            print_int(j);
            print_cstr(" = `");
            print_file_name(cur_name, cur_area, cur_ext);
            print_cstr("'.");
            print_nl_cstr("");
            print_ln();
            selector = old_setting;
        }
        break;

    case SPECIAL_NODE:
        special_out(p);
        break;

    case LANGUAGE_NODE:
        break;

    default:
        confusion("ext4");
        break;
    }
}


static void
dvi_native_font_def(internal_font_number f)
{
    int32_t font_def_length, i;

    dvi_out(DEFINE_NATIVE_FONT);
    dvi_four(f - 1);
    font_def_length = make_font_def(f);

    for (i = 0; i < font_def_length; i++)
        dvi_out(xdv_buffer[i]);
}


void
dvi_font_def(internal_font_number f)
{
    pool_pointer k;
    int32_t l;

    if (font_area[f] == AAT_FONT_FLAG || font_area[f] == OTGR_FONT_FLAG)
        dvi_native_font_def(f);
    else {
        if (f <= 256) {
            dvi_out(FNT_DEF1);
            dvi_out(f - 1);
        } else {
            dvi_out(FNT_DEF1 + 1);
            dvi_out((f - 1) / 256);
            dvi_out((f - 1) % 256);
        }

        dvi_out(font_check[f].s3);
        dvi_out(font_check[f].s2);
        dvi_out(font_check[f].s1);
        dvi_out(font_check[f].s0);
        dvi_four(font_size[f]);
        dvi_four(font_dsize[f]);
        dvi_out(length(font_area[f]));
        l = 0;
        k = str_start[(font_name[f]) - 65536L];

        while ((l == 0) && (k < str_start[(font_name[f] + 1) - 65536L])) {

            if (str_pool[k] == ':' )
                l = k - str_start[(font_name[f]) - 65536L];
            k++;
        }

        if (l == 0)
            l = length(font_name[f]);

        dvi_out(l);

        {
            register int32_t for_end;
            k = str_start[(font_area[f]) - 65536L];
            for_end = str_start[(font_area[f] + 1) - 65536L] - 1;
            if (k <= for_end)
                do {
                    dvi_out(str_pool[k]);
                }
                while (k++ < for_end);
        }

        {
            register int32_t for_end;
            k = str_start[(font_name[f]) - 65536L];
            for_end = str_start[(font_name[f]) - 65536L] + l - 1;
            if (k <= for_end)
                do {
                    dvi_out(str_pool[k]);
                }
                while (k++ < for_end);
        }
    }
}


static void
movement(scaled_t w, eight_bits o)
{
    small_number mstate;
    int32_t p, q;
    int32_t k;

    q = get_node(MOVEMENT_NODE_SIZE);
    mem[q + 1].b32.s1 = w;
    mem[q + 2].b32.s1 = dvi_offset + dvi_ptr;

    if (o == DOWN1) {
        mem[q].b32.s1 = down_ptr;
        down_ptr = q;
    } else {
        mem[q].b32.s1 = right_ptr;
        right_ptr = q;
    }

    p = mem[q].b32.s1;
    mstate = MOV_NONE_SEEN;

    while (p != TEX_NULL) {
        if (mem[p + 1].b32.s1 == w) { /*632:*/
            switch (mstate + mem[p].b32.s0) {
            case (MOV_NONE_SEEN + MOV_YZ_OK):
            case (MOV_NONE_SEEN + MOV_Y_OK):
            case (MOV_Z_SEEN + MOV_YZ_OK):
            case (MOV_Z_SEEN + MOV_Y_OK):
                if (mem[p + 2].b32.s1 < dvi_gone) {
                    goto not_found;
                } else { /*633:*/
                    k = mem[p + 2].b32.s1 - dvi_offset;
                    if (k < 0)
                        k = k + DVI_BUF_SIZE;
                    dvi_buf[k] = dvi_buf[k] + 5;
                    mem[p].b32.s0 = MOV_Y_HERE;
                    goto found;
                }
                break;

            case (MOV_NONE_SEEN + MOV_Z_OK):
            case (MOV_Y_SEEN + MOV_YZ_OK):
            case (MOV_Y_SEEN + MOV_Z_OK):
                if (mem[p + 2].b32.s1 < dvi_gone) {
                    goto not_found;
                } else { /*634:*/
                    k = mem[p + 2].b32.s1 - dvi_offset;
                    if (k < 0)
                        k = k + DVI_BUF_SIZE;
                    dvi_buf[k] = dvi_buf[k] + 10;
                    mem[p].b32.s0 = MOV_Z_HERE;
                    goto found;
                }
                break;

            case (MOV_NONE_SEEN + MOV_Y_HERE):
            case (MOV_NONE_SEEN + MOV_Z_HERE):
            case (MOV_Y_SEEN + MOV_Z_HERE):
            case (MOV_Z_SEEN + MOV_Y_HERE):
                goto found;
                break;

            default:
                break;
            }
        } else {
            switch (mstate + mem[p].b32.s0) {
            case (MOV_NONE_SEEN + MOV_Y_HERE):
                mstate = MOV_Y_SEEN;
                break;
            case (MOV_NONE_SEEN + MOV_Z_HERE):
                mstate = MOV_Z_SEEN;
                break;
            case (MOV_Y_SEEN + MOV_Z_HERE):
            case (MOV_Z_SEEN + MOV_Y_HERE):
                goto not_found;
                break;
            default:
                break;
            }
        }

        p = LLIST_link(p);
    }

not_found:
    mem[q].b32.s0 = MOV_YZ_OK;

    if (abs(w) >= 0x800000) {
        dvi_out(o + 3);
        dvi_four(w);
        return;
    }

    if (abs(w) >= 0x8000) {
        dvi_out(o + 2);

        if (w < 0)
            w = w + 0x1000000;

        dvi_out(w / 0x10000);
        w = w % 0x10000;
        goto lab2;
    }

    if (abs(w) >= 128) {
        dvi_out(o + 1);

        if (w < 0)
            w = w + 0x10000;
        goto lab2;
    }

    dvi_out(o);

    if (w < 0)
        w = w + 256;
    goto lab1;

lab2:
    dvi_out(w / 256);

lab1:
    dvi_out(w % 256);

    return;

found: /*629:*/
    mem[q].b32.s0 = mem[p].b32.s0;

    if (mem[q].b32.s0 == MOV_Y_HERE) {
        dvi_out(o + 4);

        while (mem[q].b32.s1 != p) {
            q = LLIST_link(q);

            switch (mem[q].b32.s0) {
            case MOV_YZ_OK:
                mem[q].b32.s0 = MOV_Z_OK;
                break;
            case MOV_Y_OK:
                mem[q].b32.s0 = MOV_D_FIXED;
                break;
            }
        }
    } else {
        dvi_out(o + 9);

        while (mem[q].b32.s1 != p) {
            q = LLIST_link(q);

            switch (mem[q].b32.s0) {
            case MOV_YZ_OK:
                mem[q].b32.s0 = MOV_Y_OK;
                break;
            case MOV_Z_OK:
                mem[q].b32.s0 = MOV_D_FIXED;
                break;
            }
        }
    }
}


static void
prune_movements(int32_t l)
{
    int32_t p;

    while (down_ptr != TEX_NULL) {
        if (mem[down_ptr + 2].b32.s1 < l)
            break;

        p = down_ptr;
        down_ptr = mem[p].b32.s1;
        free_node(p, MOVEMENT_NODE_SIZE);
    }

    while (right_ptr != TEX_NULL) {
        if (mem[right_ptr + 2].b32.s1 < l)
            return;
        p = right_ptr;
        right_ptr = mem[p].b32.s1;
        free_node(p, MOVEMENT_NODE_SIZE);
    }
}


static void
special_out(int32_t p)
{
    unsigned char /*max_selector */ old_setting;
    pool_pointer k;

    if (cur_h != dvi_h) {
        movement(cur_h - dvi_h, RIGHT1);
        dvi_h = cur_h;
    }
    if (cur_v != dvi_v) {
        movement(cur_v - dvi_v, DOWN1);
        dvi_v = cur_v;
    }
    doing_special = true;
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    show_token_list(mem[mem[p + 1].b32.s1].b32.s1, TEX_NULL, pool_size - pool_ptr);
    selector = old_setting;

    if (pool_ptr + 1 > pool_size)
        overflow("pool size", pool_size - init_pool_ptr);

    if (cur_length() < 256) {
        dvi_out(XXX1);
        dvi_out(cur_length());
    } else {
        dvi_out(XXX4);
        dvi_four(cur_length());
    }

    {
        register int32_t for_end;
        k = str_start[str_ptr - TOO_BIG_CHAR];
        for_end = pool_ptr - 1;
        if (k <= for_end)
            do {
                dvi_out(str_pool[k]);
            }
            while (k++ < for_end);
    }
    pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];
    doing_special = false;
}


static void
write_out(int32_t p)
{
    unsigned char old_setting; /* max_selector enum */
    int32_t old_mode;
    small_number j;
    int32_t q, r;
    int32_t d;

    q = get_avail();
    mem[q].b32.s0 = (RIGHT_BRACE_TOKEN + '}' );
    r = get_avail();
    mem[q].b32.s1 = r;
    mem[r].b32.s0 = CS_TOKEN_FLAG + END_WRITE;
    begin_token_list(q, INSERTED);
    begin_token_list(mem[p + 1].b32.s1, WRITE_TEXT);
    q = get_avail();
    mem[q].b32.s0 = (LEFT_BRACE_TOKEN + '{' );
    begin_token_list(q, INSERTED);

    old_mode = cur_list.mode;
    cur_list.mode = 0;
    cur_cs = write_loc;
    q = scan_toks(false, true);
    get_token();

    if (cur_tok != CS_TOKEN_FLAG + END_WRITE) { /*1412:*/
        error_here_with_diagnostic("Unbalanced write command");
        capture_to_diagnostic(NULL);
        help_ptr = 2;
        help_line[1] = "On this page there's a \\write with fewer real {'s than }'s.";
        help_line[0] = "I can't handle that very well; good luck.";
        error();

        do {
            get_token();
        } while (cur_tok != CS_TOKEN_FLAG + END_WRITE);
    }

    cur_list.mode = old_mode;
    end_token_list();
    old_setting = selector;
    j = mem[p + 1].b32.s0;

    if (j == 18) {
        selector = SELECTOR_NEW_STRING;
    } else if (write_open[j]) {
        selector = j;
    } else {
        if (j == 17 && selector == SELECTOR_TERM_AND_LOG)
            selector = SELECTOR_LOG_ONLY;
        print_nl_cstr("");
    }

    token_show(def_ref);
    print_ln();
    flush_list(def_ref);

    if (j == 18) {
        if (INTPAR(tracing_online) <= 0)
            selector = SELECTOR_LOG_ONLY;
        else
            selector = SELECTOR_TERM_AND_LOG;

        if (!log_opened)
            selector = SELECTOR_TERM_ONLY;

        // Existing warning for shell-escape
        diagnostic_begin_capture_warning_here();

        print_nl_cstr("runsystem(");
        for (d = 0; d <= (cur_length()) - 1; d++)
            print(str_pool[str_start[str_ptr - TOO_BIG_CHAR] + d]);

        print_cstr(")...");
        if (!shell_escape_enabled) {
            print_cstr("disabled");
            print_char('.');
        } else {
            // Currently, -Z shell-escape is implemented but hidden (see
            // src/unstable_opts.rs). When this gets actually implemented,
            // uncomment the relevant parts in that file.

            print_cstr("enabled but not implemented yet!");
        }

        capture_to_diagnostic(NULL);

        print_nl_cstr("");
        print_ln();

        // Clear shell escape command
        pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];
    }

    selector = old_setting;
}


static void
pic_out(int32_t p)
{
    unsigned char /*max_selector */ old_setting;
    int32_t i;
    pool_pointer k;

    if (cur_h != dvi_h) {
        movement(cur_h - dvi_h, RIGHT1);
        dvi_h = cur_h;
    }

    if (cur_v != dvi_v) {
        movement(cur_v - dvi_v, DOWN1);
        dvi_v = cur_v;
    }

    old_setting = selector;
    selector = SELECTOR_NEW_STRING;
    print_cstr("pdf:image ");
    print_cstr("matrix ");
    print_scaled(mem[p + 5].b32.s0);
    print(' ');
    print_scaled(mem[p + 5].b32.s1);
    print(' ');
    print_scaled(mem[p + 6].b32.s0);
    print(' ');
    print_scaled(mem[p + 6].b32.s1);
    print(' ');
    print_scaled(mem[p + 7].b32.s0);
    print(' ');
    print_scaled(mem[p + 7].b32.s1);
    print(' ');
    print_cstr("page ");
    print_int(mem[p + 4].b16.s0);
    print(' ');

    switch (mem[p + 8].b16.s1) {
    case 1:
        print_cstr("pagebox cropbox ");
        break;
    case 2:
        print_cstr("pagebox mediabox ");
        break;
    case 3:
        print_cstr("pagebox bleedbox ");
        break;
    case 5:
        print_cstr("pagebox artbox ");
        break;
    case 4:
        print_cstr("pagebox trimbox ");
        break;
    default:
        break;
    }

    print('(');
    for (i = 0; i < PIC_NODE_path_len(p); i++)
        print_raw_char(PIC_NODE_path(p)[i], true);
    print(')');

    selector = old_setting;
    if (cur_length() < 256) {
        dvi_out(XXX1);
        dvi_out(cur_length());
    } else {
        dvi_out(XXX4);
        dvi_four(cur_length());
    }

    for (k = str_start[str_ptr - TOO_BIG_CHAR]; k < pool_ptr; k++)
        dvi_out(str_pool[k]);

    pool_ptr = str_start[str_ptr - TOO_BIG_CHAR]; /* discard the string we just made */
}


void
finalize_dvi_file(void)
{
    unsigned char k;

    while (cur_s > -1) {
        if (cur_s > 0) {
            dvi_out(POP);
        } else {
            dvi_out(EOP);
            total_pages++;
        }

        cur_s--;
    }

    if (total_pages == 0) {
        print_nl_cstr("No pages of output.");
        return;
    }

    if (cur_s == -2)
        /* This happens when the DVI gets too big; a message has already been printed */
        return;

    dvi_out(POST);
    dvi_four(last_bop);
    last_bop = dvi_offset + dvi_ptr - 5;
    dvi_four(25400000L); /* magic values: conversion ratio for sp */
    dvi_four(473628672L); /* magic values: conversion ratio for sp */
    prepare_mag();
    dvi_four(INTPAR(mag));
    dvi_four(max_v);
    dvi_four(max_h);
    dvi_out(max_push / 256);
    dvi_out(max_push % 256);
    dvi_out((total_pages / 256) % 256);
    dvi_out(total_pages % 256);

    while (font_ptr > FONT_BASE) {
        if (font_used[font_ptr])
            dvi_font_def(font_ptr);
        font_ptr--;
    }

    dvi_out(POST_POST);
    dvi_four(last_bop);

    if (semantic_pagination_enabled)
        dvi_out(SPX_ID_BYTE);
    else
        dvi_out(XDV_ID_BYTE);

    k = 4 + (DVI_BUF_SIZE - dvi_ptr) % 4;

    while (k > 0) {
        dvi_out(223);
        k--;
    }

    if (dvi_limit == HALF_BUF)
        write_to_dvi(HALF_BUF, DVI_BUF_SIZE - 1);

    if (dvi_ptr > TEX_INFINITY - dvi_offset) {
        cur_s = -2;
        fatal_error("dvi length exceeds 0x7FFFFFFF");
    }

    if (dvi_ptr > 0)
        write_to_dvi(0, dvi_ptr - 1);

    k = ttstub_output_close(dvi_file);

    if (k == 0) {
        print_nl_cstr("Output written on ");
        print(output_file_name);
        print_cstr(" (");
        print_int(total_pages);
        if (total_pages != 1)
            print_cstr(" pages");
        else
            print_cstr(" page");
        print_cstr(", ");
        print_int(dvi_offset + dvi_ptr);
        print_cstr(" bytes).");
    } else {
        print_nl_cstr("Error ");
        print_int(k);
        print_cstr(" (");
        print_c_string(strerror(k));
        print_cstr(") generating output;");
        print_nl_cstr("file ");
        print(output_file_name);
        print_cstr(" may not be valid.");
        /* XeTeX adds history = OUTPUT_FAILURE = 4 here; I'm not implementing that. */
    }
}


static void
write_to_dvi(int32_t a, int32_t b)
{
    int32_t n = b - a + 1;

    if (ttstub_output_write (dvi_file, (char *) &dvi_buf[a], n) != n)
        _tt_abort ("failed to write data to XDV file");
}


static void
dvi_swap(void)
{
    if (dvi_ptr > (TEX_INFINITY - dvi_offset)) {
        cur_s = -2;
        fatal_error("dvi length exceeds 0x7FFFFFFF");
    }

    if (dvi_limit == DVI_BUF_SIZE) {
        write_to_dvi(0, HALF_BUF - 1);
        dvi_limit = HALF_BUF;
        dvi_offset = dvi_offset + DVI_BUF_SIZE;
        dvi_ptr = 0;
    } else {
        write_to_dvi(HALF_BUF, DVI_BUF_SIZE - 1);
        dvi_limit = DVI_BUF_SIZE;
    }

    dvi_gone = dvi_gone + HALF_BUF;
}


static void
dvi_four(int32_t x)
{
    if (x >= 0) {
        dvi_out(x / 0x1000000);
    } else {
        x = x + 0x40000000;
        x = x + 0x40000000;
        dvi_out((x / 0x1000000) + 128);
    }

    x = x % 0x1000000;
    dvi_out(x / 0x10000);

    x = x % 0x10000;
    dvi_out(x / 0x100);
    dvi_out(x % 0x100);
}


static void
dvi_two(UTF16_code s)
{
    dvi_out(s / 0x100);
    dvi_out(s % 0x100);
}


static void
dvi_pop(int32_t l)
{
    if (l == dvi_offset + dvi_ptr && dvi_ptr > 0)
        dvi_ptr--;
    else
        dvi_out(POP);
}
