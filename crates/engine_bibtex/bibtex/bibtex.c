/* Copyright 2020 the Tectonic Project
 * Licensed under the MIT License.
 */

#include "tectonic_bridge_core.h"
#include "bibtex_bindings.h"

#include <stdio.h> /* EOF, snprintf */

/* hack: the name eof conflicts with other function declarations under mingw. */
#define eof tectonic_eof

/* duplicated from xetexd.h: */

#include <setjmp.h>

static jmp_buf error_jmpbuf, recover_jmpbuf;

typedef size_t hash_loc;
typedef size_t cite_number;
typedef size_t field_loc;

static void unwrap(CResult res) {
    switch (res) {
    case CResult_Error:
        longjmp(error_jmpbuf, 1);
        break;
    case CResult_Recover:
        longjmp(recover_jmpbuf, 1);
        break;
    case CResult_Ok:
        break;
    }
}

static int32_t unwrap_int(CResultInt res) {
    switch (res.tag) {
    case CResultInt_Error:
        longjmp(error_jmpbuf, 1);
        break;
    case CResultInt_Recover:
        longjmp(recover_jmpbuf, 1);
        break;
    case CResultInt_Ok:
        break;
    }
    return res.ok;
}

static bool unwrap_bool(CResultBool res) {
    switch (res.tag) {
    case CResultBool_Error:
        longjmp(error_jmpbuf, 1);
        break;
    case CResultBool_Recover:
        longjmp(recover_jmpbuf, 1);
        break;
    case CResultBool_Ok:
        break;
    }
    return res.ok;
}

static LookupRes unwrap_lookup(CResultLookup res) {
    switch (res.tag) {
    case CResultLookup_Error:
        longjmp(error_jmpbuf, 1);
        break;
    case CResultLookup_Ok:
        break;
    }
    return res.ok;
}

#define FMT_BUF_SIZE 1024
static char fmt_buf[FMT_BUF_SIZE] = "";

PRINTF_FUNC(1,2) static void
printf_log(const char *fmt, ...)
{
    va_list ap;

    va_start (ap, fmt);
    vsnprintf (fmt_buf, FMT_BUF_SIZE, fmt, ap);
    va_end (ap);

    puts_log(fmt_buf);
}

static void bst_read_command(Bibtex* ctx)
{
    BufPointer tmp_ptr;

    if (ctx->read_seen) {
        puts_log("Illegal, another read command");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    ctx->read_seen = true;
    if (!ctx->entry_seen) {
        puts_log("Illegal, read command before entry command");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }

    BufPointer sv_offset1 = bib_buf_offset(BUF_TY_BASE, 2);
    BufPointer sv_offset2 = bib_buf_len(BUF_TY_BASE);
    tmp_ptr = sv_offset1;
    while (tmp_ptr < sv_offset2) {

        bib_set_buf(BUF_TY_SV, tmp_ptr, bib_buf(BUF_TY_BASE, tmp_ptr));
        tmp_ptr = tmp_ptr + 1;
    }
    {
        {
            {
                check_field_overflow(num_fields() * num_cites());
                field_loc field_ptr = 0;
                while (field_ptr < max_fields()) {
                    set_field_info(field_ptr, 0 /*missing */);
                    field_ptr = field_ptr + 1;
                }
            }
            {
                set_cite_ptr(0);
                while (cite_ptr() < max_cites()) {

                    set_type_list(cite_ptr(), 0 /*empty */ );
                    set_cite_info(cite_ptr(), 0 /*any_value */);
                    set_cite_ptr(cite_ptr() + 1);
                }
                set_old_num_cites(num_cites());
                if (ctx->all_entries) {
                    set_cite_ptr(all_marker());
                    while (cite_ptr() < old_num_cites()) {
                        set_cite_info(cite_ptr(), cite_list(cite_ptr()));
                        set_entry_exists(cite_ptr(), false);
                        set_cite_ptr(cite_ptr() + 1);
                    }
                    set_cite_ptr(all_marker());
                } else {

                    set_cite_ptr(num_cites());
                    set_all_marker(0 /*any_value */);
                }
            }
        }
        ctx->read_performed = true;
        set_bib_ptr(0);
        while (bib_ptr() < ctx->num_bib_files) {
            if (ctx->config.verbose) {
                printf_log("Database file #%ld: ", (long) bib_ptr() + 1);
                unwrap(print_bib_name());
            } else {
                char buf[512];
                snprintf(buf, sizeof(buf) - 1, "Database file #%ld: ", (long) bib_ptr() + 1);
                bib_log_prints(buf);
                unwrap(log_pr_bib_name());
            }
            set_bib_line_num(0);
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_len(BUF_TY_BASE));
            hash_loc cur_macro_loc = 0;
            hash_loc field_name_loc = 0;
            while (!eof(cur_bib_file()))
                unwrap(get_bib_command_or_entry_and_process(ctx, &cur_macro_loc, &field_name_loc));
            peekable_close(cur_bib_file());
            set_cur_bib_file(NULL);
            set_bib_ptr(bib_ptr() + 1);
        }
        ctx->reading_completed = true;

        set_num_cites(cite_ptr());
        ctx->num_preamble_strings = preamble_ptr();
        CiteNumber cites = num_cites();
        if (cites > 0) {
            cites -= 1;
        }
        if (cites * num_fields() + crossref_num() >= max_fields()) {
//                    puts_log("First One\n");
//                    puts_log("(num_cites - 1) * num_fields + crossref_num >= max_fields\n");
//                    printf_log("(%d - 1) * %d + %d >= %d\n", num_cites(), num_fields(), crossref_num(), max_fields());
            puts_log("field_info index is out of range");
            print_confusion();
            longjmp(error_jmpbuf, 1);
        }
        set_cite_ptr(0);
        while (cite_ptr() < num_cites()) {
            field_loc field_ptr = cite_ptr() * num_fields() + crossref_num();
            if (field_info(field_ptr) != 0 /*missing */ ) {
                FindCiteLocs find = find_cite_locs_for_this_cite_key(field_info(field_ptr));
                cite_number lc_cite_loc = find.lc_cite_loc;
                if (find.lc_found) {
                    cite_number cite_loc = ilk_info(lc_cite_loc);
                    set_field_info(field_ptr, hash_text(cite_loc));
                    cite_number cite_parent_ptr = ilk_info(cite_loc);
                    field_ptr = cite_ptr() * num_fields() + num_pre_defined_fields();
                    field_loc field_end_ptr = field_ptr - num_pre_defined_fields() + num_fields();
                    field_loc field_parent_ptr = cite_parent_ptr * num_fields() + num_pre_defined_fields();
                    while (field_ptr < field_end_ptr) {

                        if (field_info(field_ptr) == 0 /*missing */ )
                            set_field_info(field_ptr, field_info(field_parent_ptr));
                        field_ptr = field_ptr + 1;
                        field_parent_ptr = field_parent_ptr + 1;
                    }
                }
            }
            set_cite_ptr(cite_ptr() + 1);
        }
        cites = num_cites();
        if (cites > 0) {
            cites -= 1;
        }
        if (cites * num_fields() + crossref_num() >= max_fields()) {
//                    puts_log("Second One\n");
//                    puts_log("(num_cites - 1) * num_fields + crossref_num >= max_fields\n");
//                    printf_log("(%d - 1) * %d + %d >= %d\n", num_cites(), num_fields(), crossref_num(), max_fields());
            puts_log("field_info index is out of range");
            print_confusion();
            longjmp(error_jmpbuf, 1);
        }
        set_cite_ptr(0);
        while (cite_ptr() < num_cites()) {
            field_loc field_ptr = cite_ptr() * num_fields() + crossref_num();
            if (field_info(field_ptr) != 0 /*missing */ ) {
                FindCiteLocs find = find_cite_locs_for_this_cite_key(field_info(field_ptr));
                cite_number cite_loc = find.cite_loc;
                cite_number lc_cite_loc = find.lc_cite_loc;
                if (!find.lc_found) {
                    if (find.cite_found) {
                        hash_cite_confusion();
                        longjmp(error_jmpbuf, 1);
                    }
                    unwrap(nonexistent_cross_reference_error(field_ptr));
                    set_field_info(field_ptr, 0 /*missing */);
                } else {

                    if (cite_loc != (size_t)ilk_info(lc_cite_loc)) {
                        hash_cite_confusion();
                        longjmp(error_jmpbuf, 1);
                    }
                    cite_number cite_parent_ptr = ilk_info(cite_loc);
                    if (type_list(cite_parent_ptr) == 0 /*empty */ ) {
                        unwrap(nonexistent_cross_reference_error(field_ptr));
                        set_field_info(field_ptr, 0 /*missing */);
                    } else {

                        field_loc field_parent_ptr = cite_parent_ptr * num_fields() + crossref_num();
                        if (field_info(field_parent_ptr) != 0 /*missing */ ) {
                            puts_log("Warning--you've nested cross references");
                            unwrap(bad_cross_reference_print(cite_list(cite_parent_ptr)));
                            puts_log("\", which also refers to something\n");
                            mark_warning();
                        }
                        if (((!ctx->all_entries) && (cite_parent_ptr >= old_num_cites())
                             && (cite_info(cite_parent_ptr) < ctx->config.min_crossrefs)))
                            set_field_info(field_ptr, 0 /*missing */);
                    }
                }
            }
            set_cite_ptr(cite_ptr() + 1);
        }
        set_cite_ptr(0);
        while (cite_ptr() < num_cites()) {

            if (type_list(cite_ptr()) == 0 /*empty */ ) {
                unwrap(print_missing_entry(cite_list(cite_ptr())));
            } else if ((ctx->all_entries) || (cite_ptr() < old_num_cites()) || (cite_info(cite_ptr()) >= ctx->config.min_crossrefs)) {
                if (cite_ptr() > ctx->cite_xptr) {
                    if ((ctx->cite_xptr + 1) * num_fields() > max_fields()) {
                        puts_log("field_info index is out of range");
                        print_confusion();
                        longjmp(error_jmpbuf, 1);
                    }
                    set_cite_list(ctx->cite_xptr, cite_list(cite_ptr()));
                    set_type_list(ctx->cite_xptr, type_list(cite_ptr()));
                    FindCiteLocs find = find_cite_locs_for_this_cite_key(cite_list(cite_ptr()));
                    cite_number cite_loc = find.cite_loc;
                    cite_number lc_cite_loc = find.lc_cite_loc;
                    if (!find.lc_found) {
                        cite_key_disappeared_confusion();
                        longjmp(error_jmpbuf, 1);
                    }
                    if ((!find.cite_found) || (cite_loc != (size_t)ilk_info(lc_cite_loc))) {
                        hash_cite_confusion();
                        longjmp(error_jmpbuf, 1);
                    }
                    set_ilk_info(cite_loc, ctx->cite_xptr);
                    field_loc field_ptr = ctx->cite_xptr * num_fields();
                    field_loc field_end_ptr = field_ptr + num_fields();
                    tmp_ptr = cite_ptr() * num_fields();
                    while (field_ptr < field_end_ptr) {

                        set_field_info(field_ptr, field_info(tmp_ptr));
                        field_ptr = field_ptr + 1;
                        tmp_ptr = tmp_ptr + 1;
                    }
                }
                ctx->cite_xptr = ctx->cite_xptr + 1;
            }
            set_cite_ptr(cite_ptr() + 1);
        }
        set_num_cites(ctx->cite_xptr);
        if (ctx->all_entries) {
            set_cite_ptr(all_marker());
            while (cite_ptr() < old_num_cites()) {

                if (!entry_exists(cite_ptr()))
                    unwrap(print_missing_entry(cite_info(cite_ptr())));
                set_cite_ptr(cite_ptr() + 1);
            }
        }
        init_entry_ints();
        init_entry_strs();
        set_cite_ptr(0);
        while (cite_ptr() < num_cites()) {
            set_cite_info(cite_ptr(), cite_ptr());
            set_cite_ptr(cite_ptr() + 1);
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, sv_offset1);
    bib_set_buf_len(BUF_TY_BASE, sv_offset2);
    tmp_ptr = bib_buf_offset(BUF_TY_BASE, 2);
    while (tmp_ptr < bib_buf_len(BUF_TY_BASE)) {
        bib_set_buf(BUF_TY_BASE, tmp_ptr, bib_buf(BUF_TY_SV, tmp_ptr));
        tmp_ptr = tmp_ptr + 1;
    }
}

static void bst_reverse_command(ExecCtx* ctx)
{
    if (!ctx->glbl_ctx->read_seen) {
        puts_log("Illegal, reverse command before read command");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("reverse");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("reverse");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("reverse");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
    else {
        unwrap(bst_id_print(scan_result));
        puts_log("reverse");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    hash_loc fn_loc = 0;
    if (unwrap_bool(bad_argument_token(ctx->glbl_ctx, &fn_loc)))
        return;
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("reverse");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        bst_right_brace_print();
        puts_log("reverse");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);

    init_command_execution(ctx);
    ctx->mess_with_entries = true;
    if (num_cites() > 0) {
        cite_number sort_cite_ptr = num_cites();
        do {
            sort_cite_ptr = sort_cite_ptr - 1;
            set_cite_ptr(cite_info(sort_cite_ptr));

            execute_fn(ctx, fn_loc);
            unwrap(check_command_execution(ctx));
        } while (sort_cite_ptr != 0);
    }
}


static void
bst_sort_command(Bibtex* ctx)
{
    if (!ctx->read_seen) {
        puts_log("Illegal, sort command before read command");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }

    if (num_cites() > 1)
        quick_sort(0, num_cites() - 1);
}


static void
bst_strings_command(Bibtex* ctx)
{
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("strings");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }

    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("strings");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }

    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);

    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("strings");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }

    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if (scan_result != SCAN_RES_WHITESPACE_ADJACENT && scan_result != SCAN_RES_SPECIFIED_CHAR_ADJACENT) {
            unwrap(bst_id_print(scan_result));
            puts_log("strings");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }

        lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1));
        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                  bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1),
                                                  STR_ILK_BST_FN, true));
        hash_loc fn_loc = hash.loc;
        if (hash.exists) {
            unwrap(already_seen_function_print(ctx, fn_loc));
            return;
        }

        set_fn_type(fn_loc, FN_CLASS_STR_GLBL_VAR);
        set_ilk_info(fn_loc, num_glb_strs());

        check_grow_global_strs();

        set_num_glb_strs(num_glb_strs() + 1);

        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("strings");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }
    }

    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
}


static void
get_bst_command_and_process(ExecCtx* ctx)
{
    if (!scan_alpha()) {
        printf_log("\"%c\" can't start a style-file command", bib_buf_at_offset(BUF_TY_BASE, 2));
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }

    lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_BST_COMMAND, false));
    int32_t command_num = ilk_info(hash.loc);
    if (!hash.exists) {
        print_a_token();
        puts_log(" is an illegal style-file command");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }

    switch (command_num) {
    case 0:
        unwrap(bst_entry_command(ctx));
        break;
    case 1:
        unwrap(bst_execute_command(ctx));
        break;
    case 2:
        unwrap(bst_function_command(ctx));
        break;
    case 3:
        unwrap(bst_integers_command(ctx));
        break;
    case 4:
        unwrap(bst_iterate_command(ctx));
        break;
    case 5:
        unwrap(bst_macro_command(ctx));
        break;
    case 6:
        bst_read_command(ctx->glbl_ctx);
        break;
    case 7:
        bst_reverse_command(ctx);
        break;
    case 8:
        bst_sort_command(ctx->glbl_ctx);
        break;
    case 9:
        bst_strings_command(ctx->glbl_ctx);
        break;
    default:
        puts_log("Unknown style-file command");
        print_confusion();
        longjmp(error_jmpbuf, 1);
        break;
    }
}

History
bibtex_main(Bibtex* glbl_ctx, const char *aux_file_name)
{
    reset_all();

    if (!init_standard_output())
        return HISTORY_FATAL_ERROR;

    if (unwrap_int(initialize(glbl_ctx, aux_file_name))) {
        /* TODO: log initialization or get_the_..() error */
        return HISTORY_FATAL_ERROR;
    }

    if (setjmp(error_jmpbuf) == 1)
        goto close_up_shop;

    if (glbl_ctx->config.verbose)
        puts_log("This is BibTeX, Version 0.99d\n");
    else
        bib_log_prints("This is BibTeX, Version 0.99d\n");

    {
        char buf[512];
        snprintf (buf, sizeof(buf) - 1, "Capacity: max_strings=%ld, hash_size=%ld, hash_prime=%ld\n",
                  (long) bib_max_strings(), (long) hash_size(), (long) hash_prime());
        bib_log_prints(buf);
    }

    if (glbl_ctx->config.verbose) {
        puts_log("The top-level auxiliary file: ");
        unwrap(print_aux_name());
    } else {
        bib_log_prints("The top-level auxiliary file: ");
        unwrap(log_pr_aux_name());
    }

    while (true) {
        set_cur_aux_ln(cur_aux_ln() + 1);

        if (!input_ln(cur_aux_file())) {
            if (pop_the_aux_stack())
                break;
        } else {
            unwrap(get_aux_command_and_process(glbl_ctx));
        }
    }

    unwrap(last_check_for_aux_errors(glbl_ctx));

    if (glbl_ctx->bst_str == 0)
        goto no_bst_file;

    glbl_ctx->bst_line_num = 0;
    glbl_ctx->bbl_line_num = 1;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_len(BUF_TY_BASE));

    if (setjmp(recover_jmpbuf) == 0) {
        ExecCtx ctx = init_exec_ctx(glbl_ctx);
        while(true) {
            if (!eat_bst_white_space(ctx.glbl_ctx))
                break;
            get_bst_command_and_process(&ctx);
        }
    }

    peekable_close(glbl_ctx->bst_file);
    glbl_ctx->bst_file = NULL;

 no_bst_file:
    ttstub_output_close (glbl_ctx->bbl_file);

close_up_shop:

    if (glbl_ctx->read_performed && !glbl_ctx->reading_completed) {
        printf_log("Aborted at line %ld of file ", (long) bib_line_num());
        unwrap(print_bib_name());
    }

    switch (get_history()) {
    case HISTORY_SPOTLESS:
        break;
    case HISTORY_WARNING_ISSUED:
        if (err_count() == 1)
            puts_log("(There was 1 warning)\n");
        else
            printf_log("(There were %ld warnings)\n", (long) err_count());
        break;
    case HISTORY_ERROR_ISSUED:
        if (err_count() == 1)
            puts_log("(There was 1 error message)\n");
        else
            printf_log("(There were %ld error messages)\n", (long) err_count());
        break;
    case HISTORY_FATAL_ERROR:
        puts_log("(That was a fatal error)\n");
        break;
    default:
        puts_log("History is bunk");
        print_confusion();
        break;
    }

    bib_close_log();
    return get_history();
}


History
tt_engine_bibtex_main(ttbc_state_t *api, Bibtex* ctx, const char *aux_file_name)
{
    History rv;

    if (setjmp(*ttbc_global_engine_enter(api))) {
        ttbc_global_engine_exit();
        return HISTORY_ABORTED;
    }

    rv = bibtex_main(ctx, aux_file_name);
    ttbc_global_engine_exit();
    return rv;
}