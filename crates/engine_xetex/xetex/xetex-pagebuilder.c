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

#include "xetex-core.h"
#include "xetex-xetexd.h"


static int32_t best_page_break;
static scaled_t best_size;
static int32_t least_page_cost;
static scaled_t page_max_depth;
/* XXX other variables belong here but pop up all over the code */

void
initialize_pagebuilder_variables(void)
{
    page_max_depth = 0;
}


static void
freeze_page_specs(small_number s)
{
    page_contents = s;
    page_so_far[0] = DIMENPAR(vsize);
    page_max_depth = DIMENPAR(max_depth);
    page_so_far[7] = 0;
    page_so_far[1] = 0;
    page_so_far[2] = 0;
    page_so_far[3] = 0;
    page_so_far[4] = 0;
    page_so_far[5] = 0;
    page_so_far[6] = 0;
    least_page_cost = MAX_HALFWORD;
}


static void
ensure_vbox(eight_bits n)
{
    int32_t p = BOX_REG(n);

    if (p == TEX_NULL)
        return;

    if (NODE_type(p) != HLIST_NODE)
        return;

    error_here_with_diagnostic("Insertions can only be added to a vbox");
    capture_to_diagnostic(NULL);
    help_ptr = 3;
    help_line[2] = "Tut tut: You're trying to \\insert into a";
    help_line[1] = "\\box register that now contains an \\hbox.";
    help_line[0] = "Proceed, and I'll discard its present contents.";
    box_error(n);
}

/*1047: "The fire_up subroutine prepares to output the curent page at the best
 * place; then it fires up the user's output routine, if there is one, or it
 * simple ships out the page. There is one parameter, `c`, which represents
 * the node that was being contributed to the page when the decision to force
 * an output was made." */
static void
fire_up(int32_t c)
{
    int32_t p, q, r, s;
    int32_t prev_p;
    unsigned char n;
    bool wait;
    int32_t save_vbadness;
    scaled_t save_vfuzz;
    int32_t save_split_top_skip;
    bool process_inserts;

    /*1048: "Set the value of output_penalty" */
    if (NODE_type(best_page_break) == PENALTY_NODE) {
        geq_word_define(INT_BASE + INT_PAR__output_penalty, PENALTY_NODE_penalty(best_page_break));
        PENALTY_NODE_penalty(best_page_break) = INF_PENALTY;
    } else {
        geq_word_define(INT_BASE + INT_PAR__output_penalty, INF_PENALTY);
    }

    /* ... resuming 1047 ... "We set the values of top_mark, first_mark, and
     * bot_mark. The program uses the fact that `bot_mark != null` implies
     * `first_mark != null`; it also knows that `bot_mark == null` implies
     * `top_mark = first_mark = null`." The do_marks() call basically does the
     * same thing as the code immediately below it, but for all "mark classes"
     * beyond the default one -- a "mark class" being a concept introduced in
     * e-TeX. */

    if (sa_root[MARK_VAL] != TEX_NULL) {
        if (do_marks(FIRE_UP_INIT, 0, sa_root[MARK_VAL]))
            sa_root[MARK_VAL] = TEX_NULL;
    }

    if (cur_mark[BOT_MARK_CODE] != TEX_NULL) {
        if (cur_mark[TOP_MARK_CODE] != TEX_NULL)
            delete_token_ref(cur_mark[TOP_MARK_CODE]);

        cur_mark[TOP_MARK_CODE] = cur_mark[BOT_MARK_CODE];
        TOKEN_LIST_ref_count(cur_mark[TOP_MARK_CODE])++;
        delete_token_ref(cur_mark[FIRST_MARK_CODE]);
        cur_mark[FIRST_MARK_CODE] = TEX_NULL;
    }

    /*1049: "Put the optimal current page into box 255, update first_mark and
     * bot_mark, append insertions to their boxes, and put the remaining nodes
     * back on the contribution list." */

    if (c == best_page_break)
        best_page_break = TEX_NULL; /* "c not yet linked in" */

    if (BOX_REG(255) != TEX_NULL) { /*1050:*/
        error_here_with_diagnostic("");
        print_esc_cstr("box");
        print_cstr("255 is not void");
        capture_to_diagnostic(NULL);
        help_ptr = 2;
        help_line[1] = "You shouldn't use \\box255 except in \\output routines.";
        help_line[0] = "Proceed, and I'll discard its present contents.";
        box_error(255);
    }

    insert_penalties = 0; /* "this will count the number of insertions held over" */
    save_split_top_skip = GLUEPAR(split_top_skip);

    /* Tectonic: in semantic pagination mode, we act as if holding_inserts is
     * always active. */

    process_inserts = (INTPAR(holding_inserts) <= 0) && !semantic_pagination_enabled;

    if (process_inserts) {
        /*1053: "Prepare all the boxes involved in insertions to act as
         * queues". Namely: for each insert being tracked, set the
         * `last_ins_ptr` field of its data structure to the last node in its
         * associated vlist. If holding_inserts is positive, the inserts are
         * just kept in the page vlist without any processing, I believe with
         * the expectation that the output routine will do something clever
         * with them. */
        r = LLIST_link(PAGE_INS_HEAD);

        while (r != PAGE_INS_HEAD) {
            if (PAGE_INS_NODE_best_ins_ptr(r) != TEX_NULL) {
                n = NODE_subtype(r);
                ensure_vbox(n);

                if (BOX_REG(n) == TEX_NULL)
                    BOX_REG(n) = new_null_box();

                p = BOX_REG(n) + 5; /* 5 = list_offset, "position of the list inside the box" */
                while (LLIST_link(p) != TEX_NULL)
                    p = LLIST_link(p);

                PAGE_INS_NODE_last_ins_ptr(r) = p;
            }

            r = LLIST_link(r);
        }
    }

    q = HOLD_HEAD;
    LLIST_link(q) = TEX_NULL;
    prev_p = PAGE_HEAD;
    p = LLIST_link(prev_p);

    while (p != best_page_break) {
        if (NODE_type(p) == INS_NODE) {
            if (process_inserts) {
                /*1055: "Either insert the material specified by node p into
                 * the appropriate box, or hold it for the next page; also
                 * delete node p from the current page." */
                r = LLIST_link(PAGE_INS_HEAD);

                while (NODE_subtype(r) != NODE_subtype(p))
                    r = LLIST_link(r);

                if (PAGE_INS_NODE_best_ins_ptr(r) == TEX_NULL) {
                    wait = true;
                } else {
                    wait = false;

                    s = PAGE_INS_NODE_last_ins_ptr(r);
                    LLIST_link(s) = INSERTION_NODE_ins_ptr(p);

                    if (PAGE_INS_NODE_best_ins_ptr(r) == p) {
                        /*1056: "Wrap up the box specified by node r,
                         * splitting node p if called for; set wait = true if
                         * node p holds a remainder after splitting" */

                        if (NODE_type(r) == SPLIT_UP) {
                            if (PAGE_INS_NODE_broken_ins(r) == p && PAGE_INS_NODE_broken_ptr(r) != TEX_NULL) {
                                while (LLIST_link(s) != PAGE_INS_NODE_broken_ptr(r))
                                    s = LLIST_link(s);

                                LLIST_link(s) = TEX_NULL;
                                GLUEPAR(split_top_skip) = INSERTION_NODE_split_top_ptr(p);
                                INSERTION_NODE_ins_ptr(p) = prune_page_top(PAGE_INS_NODE_broken_ptr(r), false);

                                if (INSERTION_NODE_ins_ptr(p) != TEX_NULL) {
                                    temp_ptr = vpackage(INSERTION_NODE_ins_ptr(p), 0, ADDITIONAL, MAX_HALFWORD);
                                    BOX_height(p) = BOX_height(temp_ptr) + BOX_depth(temp_ptr);
                                    free_node(temp_ptr, BOX_NODE_SIZE);
                                    wait = true;
                                }
                            }
                        }

                        PAGE_INS_NODE_best_ins_ptr(r) = TEX_NULL;
                        n = NODE_subtype(r);
                        temp_ptr = BOX_list_ptr(BOX_REG(n));
                        free_node(BOX_REG(n), BOX_NODE_SIZE);
                        BOX_REG(n) = vpackage(temp_ptr, 0, ADDITIONAL, MAX_HALFWORD);
                    } else {
                        while (LLIST_link(s) != TEX_NULL)
                            s = LLIST_link(s);
                        PAGE_INS_NODE_last_ins_ptr(r) = s;
                    }
                }

                /*1057: "Either append the insertion node p after node q, and
                 * remove it from the current page, or delete node(p)" */

                LLIST_link(prev_p) = LLIST_link(p);
                LLIST_link(p) = TEX_NULL;

                if (wait) {
                    LLIST_link(q) = p;
                    q = p;
                    insert_penalties++;
                } else {
                    delete_glue_ref(INSERTION_NODE_split_top_ptr(p));
                    free_node(p, INS_NODE_SIZE);
                }

                p = prev_p; /*:1057 */
            }
        } else if (NODE_type(p) == MARK_NODE) {
            if (MARK_NODE_class(p) != 0) {
                /*1618: "Update the current marks" */
                find_sa_element(MARK_VAL, MARK_NODE_class(p), true);

                if (ETEX_MARK_sa_first_mark(cur_ptr) == TEX_NULL) {
                    ETEX_MARK_sa_first_mark(cur_ptr) = MARK_NODE_ptr(p);
                    TOKEN_LIST_ref_count(MARK_NODE_ptr(p))++;
                }

                if (ETEX_MARK_sa_bot_mark(cur_ptr) != TEX_NULL)
                    delete_token_ref(ETEX_MARK_sa_bot_mark(cur_ptr));

                ETEX_MARK_sa_bot_mark(cur_ptr) = MARK_NODE_ptr(p);
                TOKEN_LIST_ref_count(MARK_NODE_ptr(p))++;
            } else {
                /*1051: "Update the values of first_mark and bot_mark" */
                if (cur_mark[FIRST_MARK_CODE] == TEX_NULL) {
                    cur_mark[FIRST_MARK_CODE] = MARK_NODE_ptr(p);
                    TOKEN_LIST_ref_count(cur_mark[FIRST_MARK_CODE])++;
                }

                if (cur_mark[BOT_MARK_CODE] != TEX_NULL)
                    delete_token_ref(cur_mark[BOT_MARK_CODE]);

                cur_mark[BOT_MARK_CODE] = MARK_NODE_ptr(p);
                TOKEN_LIST_ref_count(cur_mark[BOT_MARK_CODE])++;
            }
        }

        prev_p = p;
        p = LLIST_link(prev_p);
    }

    GLUEPAR(split_top_skip) = save_split_top_skip;

    /*1052: "Break the current page at node p, put it in box 255, and put the
     * remaining nodes on the contribution list". */

    if (p != TEX_NULL) {
        if (LLIST_link(CONTRIB_HEAD) == TEX_NULL) {
            if (nest_ptr == 0)
                cur_list.tail = page_tail;
            else
                nest[0].tail = page_tail;
        }

        LLIST_link(page_tail) = LLIST_link(CONTRIB_HEAD);
        LLIST_link(CONTRIB_HEAD) = p;
        LLIST_link(prev_p) = TEX_NULL;
    }

    /* Temporarily futz some variables to inhibit error messages */
    save_vbadness = INTPAR(vbadness);
    INTPAR(vbadness) = INF_BAD;
    save_vfuzz = DIMENPAR(vfuzz);
    DIMENPAR(vfuzz) = MAX_HALFWORD;
    BOX_REG(255) = vpackage(LLIST_link(PAGE_HEAD), best_size, EXACTLY, page_max_depth);
    INTPAR(vbadness) = save_vbadness;
    DIMENPAR(vfuzz) = save_vfuzz;

    if (last_glue != MAX_HALFWORD)
        delete_glue_ref(last_glue);

    /*1026: "Start a new current page" */
    page_contents = EMPTY;
    page_tail = PAGE_HEAD;
    LLIST_link(PAGE_HEAD) = TEX_NULL;
    last_glue = MAX_HALFWORD;
    last_penalty = 0;
    last_kern = 0;
    last_node_type = -1;
    page_so_far[7] = 0;
    page_max_depth = 0;

    if (q != HOLD_HEAD) {
        LLIST_link(PAGE_HEAD) = LLIST_link(HOLD_HEAD);
        page_tail = q;
    }

    /*1054: "Delete the page-insertion nodes" */
    r = LLIST_link(PAGE_INS_HEAD);
    while (r != PAGE_INS_HEAD) {
        q = LLIST_link(r);
        free_node(r, PAGE_INS_NODE_SIZE);
        r = q;
    }

    LLIST_link(PAGE_INS_HEAD) = PAGE_INS_HEAD;

    /* ... resuming 1047 ... */

    if (sa_root[MARK_VAL] != TEX_NULL) {
        if (do_marks(FIRE_UP_DONE, 0, sa_root[MARK_VAL]))
            sa_root[MARK_VAL] = TEX_NULL;
    }

    if (cur_mark[TOP_MARK_CODE] != TEX_NULL && cur_mark[FIRST_MARK_CODE] == TEX_NULL) {
        cur_mark[FIRST_MARK_CODE] = cur_mark[TOP_MARK_CODE];
        TOKEN_LIST_ref_count(cur_mark[TOP_MARK_CODE])++;
    }

    /* Tectonic: in semantic pagination mode, ignore the output routine. */

    if (LOCAL(output_routine) != TEX_NULL && !semantic_pagination_enabled) {
        if (dead_cycles >= INTPAR(max_dead_cycles)) {
            /*1059: "Explain that too many dead cycles have happened in a row." */
            error_here_with_diagnostic("Output loop---");
            print_int(dead_cycles);
            print_cstr(" consecutive dead cycles");
            capture_to_diagnostic(NULL);
            help_ptr = 3;
            help_line[2] = "I've concluded that your \\output is awry; it never does a";
            help_line[1] = "\\shipout, so I'm shipping \\box255 out myself. Next time";
            help_line[0] = "increase \\maxdeadcycles if you want me to be more patient!";
            error();
        } else {
            /*1060: "Fire up the user's output routine and return" */
            output_active = true;
            dead_cycles++;
            push_nest();
            cur_list.mode = -VMODE;
            cur_list.aux.b32.s1 = IGNORE_DEPTH; /* this is `prev_depth` */
            cur_list.mode_line = -line;
            begin_token_list(LOCAL(output_routine), OUTPUT_TEXT);
            new_save_level(OUTPUT_GROUP);
            normal_paragraph();
            scan_left_brace();
            return;
        }
    }

    /*1058: "Perform the default output routine." */
    if (LLIST_link(PAGE_HEAD) != TEX_NULL) {
        if (LLIST_link(CONTRIB_HEAD) == TEX_NULL) {
            if (nest_ptr == 0)
                cur_list.tail = page_tail;
            else
                nest[0].tail = page_tail;
        } else {
            LLIST_link(page_tail) = LLIST_link(CONTRIB_HEAD);
        }

        LLIST_link(CONTRIB_HEAD) = LLIST_link(PAGE_HEAD);
        LLIST_link(PAGE_HEAD) = TEX_NULL;
        page_tail = PAGE_HEAD;
    }

    flush_node_list(disc_ptr[LAST_BOX_CODE]);
    disc_ptr[LAST_BOX_CODE] = TEX_NULL;
    ship_out(BOX_REG(255));
    BOX_REG(255) = TEX_NULL;
}


/*1029: "When TeX has appended new material in vertical mode, it calls the
 * procedure build_page, which tries to catch up by moving nodes from the
 * contribution list to the current page. This procedure will succeed in its
 * goal of emptying the contribution list, unless a page break is discovered,
 * i.e., unless the current page has grown to the point where the optimum next
 * page break has been determined. In the latter case, the nodes after the
 * optimum break will go back onto the contribution list, and control will
 * effectively pass to the user's output routine." ... "TeX is not always in
 * vertical mode at the time build_page is called; the current mode reflects
 * what TeX should return to, after the contribution list has been emptied. A
 * call on build_page should be immediate followed by `goto big_switch`, which
 * is TeX's central control point." */

#define AWFUL_BAD MAX_HALFWORD /* XXX redundant with xetex-linebreak.c */

void
build_page(void)
{
    int32_t p;
    int32_t q, r;
    int32_t b, c;
    int32_t pi;
    unsigned char /*biggest_reg */ n;
    scaled_t delta, h, w;

    if (LLIST_link(CONTRIB_HEAD) == TEX_NULL || output_active)
        return;

    do {
        p = LLIST_link(CONTRIB_HEAD);

        /*1031: "Update the values of last_glue, last_penalty, and last_kern" */
        if (last_glue != MAX_HALFWORD)
            delete_glue_ref(last_glue);

        last_penalty = 0;
        last_kern = 0;
        last_node_type = NODE_type(p) + 1;

        if (NODE_type(p) == GLUE_NODE) {
            last_glue = GLUE_NODE_glue_ptr(p);
            GLUE_SPEC_ref_count(last_glue)++;
        } else {
            last_glue = MAX_HALFWORD;

            if (NODE_type(p) == PENALTY_NODE)
                last_penalty = PENALTY_NODE_penalty(p);
            else if (NODE_type(p) == KERN_NODE)
                last_kern = BOX_width(p);
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

        switch (NODE_type(p)) {
        case HLIST_NODE:
        case VLIST_NODE:
        case RULE_NODE:
            if (page_contents < BOX_THERE) {
                /*1036: "Initialize the current page, insert the \topskip glue
                 * ahead of p, and goto continue." */
                if (page_contents == EMPTY)
                    freeze_page_specs(BOX_THERE);
                else
                    page_contents = BOX_THERE;

                q = new_skip_param(GLUE_PAR__top_skip); /* "now temp_ptr = glue_ptr(q) */

                if (BOX_width(temp_ptr) > BOX_height(p))
                    BOX_width(temp_ptr) -= BOX_height(p);
                else
                    BOX_width(temp_ptr) = 0;

                LLIST_link(q) = p;
                LLIST_link(CONTRIB_HEAD) = q;
                continue;
            } else {
                /*1037: "Prepare to move a box or rule node to the current
                 * page, then goto contribute." */

                page_so_far[1] += page_so_far[7] + BOX_height(p);
                page_so_far[7] = BOX_depth(p);
                goto contribute;
            }
            break;

        case WHATSIT_NODE:
            /*1401: "Prepare to move whatsit p to the current page, then goto contribute" */
            if (NODE_subtype(p) == PIC_NODE || NODE_subtype(p) == PDF_NODE) {
                page_so_far[1] += page_so_far[7] + BOX_height(p);
                page_so_far[7] = BOX_depth(p);
            }
            goto contribute;
            break;

        case GLUE_NODE:
            if (page_contents < BOX_THERE)
                goto done1;
            else if (is_non_discardable_node(page_tail))
                pi = 0;
            else
                goto update_heights;
            break;

        case KERN_NODE:
            if (page_contents < BOX_THERE)
                goto done1;
            else if (LLIST_link(p) == TEX_NULL)
                return;
            else if (NODE_type(LLIST_link(p)) == GLUE_NODE)
                pi = 0;
            else
                goto update_heights;
            break;

        case PENALTY_NODE:
            if (page_contents < BOX_THERE)
                goto done1;
            else
                pi = PENALTY_NODE_penalty(p);
            break;

        case MARK_NODE:
            goto contribute;
            break;

        case INS_NODE:
            /*1043: "Append an insertion to the current page and goto contribute" */
            if (page_contents == EMPTY)
                freeze_page_specs(INSERTS_ONLY);

            n = NODE_subtype(p);
            r = PAGE_INS_HEAD;

            while (n >= NODE_subtype(LLIST_link(r)))
                r = LLIST_link(r);

            if (NODE_subtype(r) != n) {
                /*1044: "Create a page insertion node with subtype(r) = n, and
                 * include the glue correction for box `n` in the current page
                 * state" */
                q = get_node(PAGE_INS_NODE_SIZE);
                LLIST_link(q) = LLIST_link(r);
                LLIST_link(r) = q;
                r = q;

                NODE_subtype(r) = n;
                NODE_type(r) = INSERTING;
                ensure_vbox(n);

                if (BOX_REG(n) == TEX_NULL)
                    BOX_height(r) = 0;
                else
                    BOX_height(r) = BOX_height(BOX_REG(n)) + BOX_depth(BOX_REG(n));

                PAGE_INS_NODE_best_ins_ptr(r) = TEX_NULL;
                q = SKIP_REG(n);

                if (COUNT_REG(n) == 1000)
                    h = BOX_height(r);
                else
                    h = x_over_n(BOX_height(r), 1000) * COUNT_REG(n);

                page_so_far[0] -= h + BOX_width(q);
                page_so_far[2 + GLUE_SPEC_stretch_order(q)] += GLUE_SPEC_stretch(q);
                page_so_far[6] += GLUE_SPEC_shrink(q);

                if (GLUE_SPEC_shrink_order(q) != NORMAL && GLUE_SPEC_shrink(q) != 0) {
                    error_here_with_diagnostic("Infinite glue shrinkage inserted from ");
                    print_esc_cstr("skip");
                    print_int(n);
                    capture_to_diagnostic(NULL);
                    help_ptr = 3;
                    help_line[2] = "The correction glue for page breaking with insertions";
                    help_line[1] = "must have finite shrinkability. But you may proceed,";
                    help_line[0] = "since the offensive shrinkability has been made finite.";
                    error();
                }
            }

            if (NODE_type(r) == SPLIT_UP) {
                insert_penalties += INSERTION_NODE_float_cost(p);
            } else {
                PAGE_INS_NODE_last_ins_ptr(r) = p;
                delta = page_so_far[0] - page_so_far[1] - page_so_far[7] + page_so_far[6];

                if (COUNT_REG(n) == 1000)
                    h = BOX_height(p);
                else
                    h = x_over_n(BOX_height(p), 1000) * COUNT_REG(n);

                if ((h <= 0 || h <= delta) && BOX_height(p) + BOX_height(r) <= SCALED_REG(n)) {
                    page_so_far[0] -= h;
                    BOX_height(r) += BOX_height(p);
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
                    if (COUNT_REG(n) <= 0)
                        w = MAX_HALFWORD;
                    else {
                        w = page_so_far[0] - page_so_far[1] - page_so_far[7];
                        if (COUNT_REG(n) != 1000)
                            w = x_over_n(w, COUNT_REG(n)) * 1000;
                    }

                    if (w > SCALED_REG(n) - BOX_height(r))
                        w = SCALED_REG(n) - BOX_height(r);

                    q = vert_break(INSERTION_NODE_ins_ptr(p), w, BOX_depth(p));
                    BOX_height(r) += best_height_plus_depth;

                    if (COUNT_REG(n) != 1000)
                        best_height_plus_depth = x_over_n(best_height_plus_depth, 1000) * COUNT_REG(n);

                    page_so_far[0] -= best_height_plus_depth;
                    NODE_type(r) = SPLIT_UP;
                    PAGE_INS_NODE_broken_ptr(r) = q;
                    PAGE_INS_NODE_broken_ins(r) = p;

                    if (q == TEX_NULL)
                        insert_penalties += EJECT_PENALTY;
                    else if (NODE_type(q) == PENALTY_NODE)
                        insert_penalties += PENALTY_NODE_penalty(q);
                }
            }
            goto contribute;
            break; /* end of INS_NODE */

        default:
            confusion("page");
            break;
        }

        /*1040: "Check if node p is the new champion breakpoint; then if it is
         * time for a page break, prepare for output, and either fire up the
         * user's output routine and return or ship out the page and goto
         * done." We reach this point when p is a glue, kern, or penalty, and
         * there's already content on the page -- so this might be a place to
         * break the page. */

        if (pi < INF_PENALTY) {
            /*1042: "Compute the badness b of the current page, using
             * awful_bad if the box is too full." */
            if (page_so_far[1] < page_so_far[0]) {
                if (page_so_far[3] != 0 || page_so_far[4] != 0 || page_so_far[5] != 0)
                    b = 0;
                else
                    b = badness(page_so_far[0] - page_so_far[1], page_so_far[2]);
            } else if (page_so_far[1] - page_so_far[0] > page_so_far[6]) {
                b = AWFUL_BAD;
            } else {
                b = badness(page_so_far[1] - page_so_far[0], page_so_far[6]);
            }

            if (b < AWFUL_BAD) {
                if (pi <= EJECT_PENALTY)
                    c = pi;
                else if (b < INF_BAD)
                    c = b + pi + insert_penalties;
                else
                    c = 100000L; /* DEPLORABLE */
            } else {
                c = b;
            }

            if (insert_penalties >= 10000)
                c = MAX_HALFWORD;

            if (c <= least_page_cost) {
                best_page_break = p;
                best_size = page_so_far[0];
                least_page_cost = c;
                r = LLIST_link(PAGE_INS_HEAD);

                while (r != PAGE_INS_HEAD) {
                    PAGE_INS_NODE_best_ins_ptr(r) = PAGE_INS_NODE_last_ins_ptr(r);
                    r = LLIST_link(r);
                }
            }

            if (c == AWFUL_BAD || pi <= EJECT_PENALTY) {
                fire_up(p);
                if (output_active) /* "user's output routine will act" */
                    return;
                goto done; /* "the page has been shipped out by the default output routine" */
            }
        }

        /* ... resuming 1032 ... I believe the "goto" here can only be
         * triggered if p is a penalty node, and we decided not to break. */

        if (NODE_type(p) < GLUE_NODE || NODE_type(p) > KERN_NODE)
            goto contribute;

 update_heights:
        /*1039: "Update the current page measurements with respect to the glue or kern
         * specified by node p" */

        if (NODE_type(p) == KERN_NODE) {
            q = p;
        } else {
            q = GLUE_NODE_glue_ptr(p);
            page_so_far[2 + GLUE_SPEC_stretch_order(q)] += GLUE_SPEC_stretch(q);
            page_so_far[6] += GLUE_SPEC_shrink(q);

            if (GLUE_SPEC_shrink_order(q) != NORMAL && GLUE_SPEC_shrink(q) != 0) {
                error_here_with_diagnostic("Infinite glue shrinkage found on current page");
                capture_to_diagnostic(NULL);

                help_ptr = 4;
                help_line[3] = "The page about to be output contains some infinitely";
                help_line[2] = "shrinkable glue, e.g., `\\vss' or `\\vskip 0pt minus 1fil'.";
                help_line[1] = "Such glue doesn't belong there; but you can safely proceed,";
                help_line[0] = "since the offensive shrinkability has been made finite.";
                error();

                r = new_spec(q);
                GLUE_SPEC_shrink_order(r) = NORMAL;
                delete_glue_ref(q);
                GLUE_NODE_glue_ptr(p) = r;
                q = r;
            }
        }

        page_so_far[1] += page_so_far[7] + BOX_width(q);
        page_so_far[7] = 0;

    contribute:
        /*1038: "Make sure that page_max_depth is not exceeded." */
        if (page_so_far[7] > page_max_depth) {
            page_so_far[1] += page_so_far[7] - page_max_depth;
            page_so_far[7] = page_max_depth;
        }

        /*1033: "Link node p into the current page and goto done." */
        LLIST_link(page_tail) = p;
        page_tail = p;
        LLIST_link(CONTRIB_HEAD) = LLIST_link(p);
        LLIST_link(p) = TEX_NULL;
        goto done;

    done1:
        /*1034: "Recycle node p". This codepath is triggered if we encountered
         * something nonprinting (glue, kern, penalty) and there aren't any
         * yes-printing boxes at the top of the page yet. When that happens,
         * we just discard the nonprinting node. */
        LLIST_link(CONTRIB_HEAD) = LLIST_link(p);
        LLIST_link(p) = TEX_NULL;

        if (INTPAR(saving_vdiscards) <= 0) {
            flush_node_list(p);
        } else {
            /* `disc_ptr[LAST_BOX_CODE]` is `tail_page_disc`, the last item
             * removed by the page builder. `disc_ptr[LAST_BOX_CODE]` is
             * `page_disc`, the first item removed by the page builder.
             * `disc_ptr[VSPLIT_CODE]` is `split_disc`, the first item removed
             * by \vsplit. */

            if (disc_ptr[LAST_BOX_CODE] == TEX_NULL)
                disc_ptr[LAST_BOX_CODE] = p;
            else
                LLIST_link(disc_ptr[COPY_CODE]) = p;

            disc_ptr[COPY_CODE] = p;
        }

    done:
        ;
    } while (LLIST_link(CONTRIB_HEAD) != TEX_NULL);

    if (nest_ptr == 0)
        cur_list.tail = CONTRIB_HEAD; /* "vertical mode" */
    else
        nest[0].tail = CONTRIB_HEAD; /* "other modes" */
}
