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

#define aux_stack_size 20

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

static StrNumber unwrap_str(CResultStr res) {
    switch (res.tag) {
    case CResultStr_Error:
        longjmp(error_jmpbuf, 1);
        break;
    case CResultStr_Recover:
        longjmp(recover_jmpbuf, 1);
        break;
    case CResultStr_Ok:
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

static void aux_bib_data_command(Bibtex* ctx)
{
    if (ctx->bib_seen) {
        unwrap(aux_err_illegal_another_print(0 /*n_aux_bibdata */ ));
        unwrap(aux_err_print());
        return;
    }
    ctx->bib_seen = true;
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        if (!scan2_white(125 /*right_brace */ , 44 /*comma */)) {
            aux_err_no_right_brace_print();
            unwrap(aux_err_print());
            return;
        }
        if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
            aux_err_white_space_in_argument_print();
            unwrap(aux_err_print());
            return;
        }
        if ((bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) && (bib_buf_at_offset(BUF_TY_BASE, 2) == 125 /*right_brace */ )) {
            aux_err_stuff_after_right_brace_print();
            unwrap(aux_err_print());
            return;
        }

        check_bib_files(bib_ptr());

        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                  (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                  STR_ILK_BIB_FILE, true));
        set_cur_bib(hash_text(hash.loc));
        if (hash.exists) {
            puts_log("This database file appears more than once: ");
            unwrap(print_bib_name());
            unwrap(aux_err_print());
            return;
        }
        NameAndLen nal = start_name(cur_bib());
        PeekableInput* bib_in = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_BIB);
        if (bib_in == NULL) {
            puts_log("I couldn't open database file ");
            unwrap(print_bib_name());
            unwrap(aux_err_print());
            free(nal.name_of_file);
            return;
        }
        set_cur_bib_file(bib_in);
        free(nal.name_of_file);

        set_bib_ptr(bib_ptr() + 1);
    }
}

static void aux_bib_style_command(Bibtex* ctx)
{
    if (ctx->bst_seen) {
        unwrap(aux_err_illegal_another_print(1 /*n_aux_bibstyle */ ));
        unwrap(aux_err_print());
        return;
    }
    ctx->bst_seen = true;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!scan1_white(125 /*right_brace */)) {
        aux_err_no_right_brace_print();
        unwrap(aux_err_print());
        return;
    }
    if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
        aux_err_white_space_in_argument_print();
        unwrap(aux_err_print());
        return;
    }
    if (bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) {
        aux_err_stuff_after_right_brace_print();
        unwrap(aux_err_print());
        return;
    }

    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_BST_FILE, true));
    ctx->bst_str = hash_text(hash.loc);
    if (hash.exists) {
        puts_log("Already encountered style file");
        print_confusion();
        longjmp(error_jmpbuf, 1);
    }
    NameAndLen nal = start_name(ctx->bst_str);
    if ((ctx->bst_file = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_BST)) == NULL) {
        puts_log("I couldn't open style file ");
        unwrap(print_bst_name(ctx));
        ctx->bst_str = 0;
        unwrap(aux_err_print());
        free(nal.name_of_file);
        return;
    }
    free(nal.name_of_file);
    if (ctx->config.verbose) {
        puts_log("The style file: ");
        unwrap(print_bst_name(ctx));
    } else {
        bib_log_prints("The style file: ");
        unwrap(log_pr_bst_name(ctx));
    }
}

static void aux_citation_command(Bibtex* ctx)
{
    BufPointer tmp_ptr;

    ctx->citation_seen = true;
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        if (!scan2_white(125 /*right_brace */ , 44 /*comma */)) {
            aux_err_no_right_brace_print();
            unwrap(aux_err_print());
            return;
        }
        if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
            aux_err_white_space_in_argument_print();
            unwrap(aux_err_print());
            return;
        }
        if ((bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) && (bib_buf_at_offset(BUF_TY_BASE, 2) == 125 /*right_brace */ )) {
            aux_err_stuff_after_right_brace_print();
            unwrap(aux_err_print());
            return;
        }
        if ((bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)) == 1) {
            if (bib_buf_at_offset(BUF_TY_BASE, 1) == 42 /*star */ ) {
                if (ctx->all_entries) {
                    puts_log("Multiple inclusions of entire database\n");
                    unwrap(aux_err_print());
                    return;
                } else {
                    ctx->all_entries = true;
                    set_all_marker(cite_ptr());
                    goto lab23;
                }
            }
        }
        tmp_ptr = bib_buf_offset(BUF_TY_BASE, 1);
        while (tmp_ptr < bib_buf_offset(BUF_TY_BASE, 2)) {
            bib_set_buf(BUF_TY_EX, tmp_ptr, bib_buf(BUF_TY_BASE, tmp_ptr));
            tmp_ptr = tmp_ptr + 1;
        }
        lower_case(BUF_TY_EX, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_EX, bib_buf_offset(BUF_TY_BASE, 1),
                                                  (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                  STR_ILK_LC_CITE, true));
        cite_number lc_cite_loc = hash.loc;
        if (hash.exists) {
            hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                            (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                            STR_ILK_CITE, false));
            if (!hash.exists) {
                puts_log("Case mismatch error between cite keys ");
                print_a_token();
                puts_log(" and ");
                unwrap(print_a_pool_str(cite_list(ilk_info(ilk_info(lc_cite_loc)))));
                putc_log('\n');
                unwrap(aux_err_print());
                return;
            }
        } else {
            hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                            (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                            STR_ILK_CITE, true));
            cite_number cite_loc = hash.loc;
            if (hash.exists) {
                hash_cite_confusion();
                longjmp(error_jmpbuf, 1);
            }
            check_cite_overflow(cite_ptr());
            set_cite_list(cite_ptr(), hash_text(cite_loc));
            set_ilk_info(cite_loc, cite_ptr());
            set_ilk_info(lc_cite_loc, cite_loc);
            set_cite_ptr(cite_ptr() + 1);
        }
 lab23:                        /*next_cite */ ;
    }
}

static void aux_input_command(Bibtex* ctx)
{
    bool aux_extension_ok;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!scan1_white(125 /*right_brace */)) {
        aux_err_no_right_brace_print();
        unwrap(aux_err_print());
        return;
    }
    if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
        aux_err_white_space_in_argument_print();
        unwrap(aux_err_print());
        return;
    }
    if (bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) {
        aux_err_stuff_after_right_brace_print();
        unwrap(aux_err_print());
        return;
    }
    set_aux_ptr(aux_ptr() + 1);
    if (aux_ptr() == aux_stack_size) {
        print_a_token();
        puts_log(": ");
        print_overflow();
        printf_log("auxiliary file depth %ld\n", (long) aux_stack_size);
        longjmp(error_jmpbuf, 1);
    }
    aux_extension_ok = true;
    if ((bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)) < (bib_str_start(ctx->s_aux_extension + 1) - bib_str_start(ctx->s_aux_extension)))
        aux_extension_ok = false;
    else if (!bib_str_eq_buf(
            ctx->s_aux_extension,
            BUF_TY_BASE,
            bib_buf_offset(BUF_TY_BASE, 2) - (bib_str_start(ctx->s_aux_extension + 1) - bib_str_start(ctx->s_aux_extension)),
            (bib_str_start(ctx->s_aux_extension + 1) - bib_str_start(ctx->s_aux_extension))
            ))
        aux_extension_ok = false;
    if (!aux_extension_ok) {
        print_a_token();
        puts_log(" has a wrong extension");
        set_aux_ptr(aux_ptr() - 1);
        unwrap(aux_err_print());
        return;
    }
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_AUX_FILE, true));
    set_cur_aux(hash_text(hash.loc));
    if (hash.exists) {
        puts_log("Already encountered file ");
        unwrap(print_aux_name());
        set_aux_ptr(aux_ptr() - 1);
        unwrap(aux_err_print());
        return;
    }

    NameAndLen nal = start_name(cur_aux());
    int32_t name_ptr = nal.name_length;
    nal.name_of_file[name_ptr] = 0;
    PeekableInput* aux_file = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_TEX);
    if (aux_file == NULL) {
        puts_log("I couldn't open auxiliary file ");
        unwrap(print_aux_name());
        set_aux_ptr(aux_ptr() - 1);
        unwrap(aux_err_print());
        free(nal.name_of_file);
        return;
    }
    set_cur_aux_file(aux_file);
    free(nal.name_of_file);

    printf_log("A level-%ld auxiliary file: ", (long) aux_ptr());
    unwrap(log_pr_aux_name());
    set_cur_aux_ln(0);
}

static int
pop_the_aux_stack(void)
{
    peekable_close (cur_aux_file());
    set_cur_aux_file(NULL);

    if (aux_ptr() == 0)
        return 1;

    set_aux_ptr(aux_ptr() - 1);
    return 0;
}

static void get_aux_command_and_process(Bibtex* ctx)
{
    bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    if (!scan1(123 /*left_brace */))
        return;
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_AUX_COMMAND, false));
    int32_t command_num = ilk_info(hash.loc);
    if (hash.exists)
        switch ((command_num)) {
        case 0:
            aux_bib_data_command(ctx);
            break;
        case 1:
            aux_bib_style_command(ctx);
            break;
        case 2:
            aux_citation_command(ctx);
            break;
        case 3:
            aux_input_command(ctx);
            break;
        default:
            puts_log("Unknown auxiliary-file command");
            print_confusion();
            longjmp(error_jmpbuf, 1);
            break;
        }
}

static void last_check_for_aux_errors(Bibtex* ctx)
{
    set_num_cites(cite_ptr());
    ctx->num_bib_files = bib_ptr();
    if (!ctx->citation_seen) {
        aux_end1_err_print();
        puts_log("\\citation commands");
        unwrap(aux_end2_err_print());
    } else if ((num_cites() == 0) && (!ctx->all_entries)) {
        aux_end1_err_print();
        puts_log("cite keys");
        unwrap(aux_end2_err_print());
    }
    if (!ctx->bib_seen) {
        aux_end1_err_print();
        puts_log("\\bibdata command");
        unwrap(aux_end2_err_print());
    } else if (ctx->num_bib_files == 0) {
        aux_end1_err_print();
        puts_log("database files");
        unwrap(aux_end2_err_print());
    }
    if (!ctx->bst_seen) {
        aux_end1_err_print();
        puts_log("\\bibstyle command");
        unwrap(aux_end2_err_print());
    } else if (ctx->bst_str == 0) {
        aux_end1_err_print();
        puts_log("style file");
        unwrap(aux_end2_err_print());
    }
}

static void bst_entry_command(Bibtex* ctx)
{
    if (ctx->entry_seen) {
        puts_log("Illegal, another entry command");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    ctx->entry_seen = true;
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            unwrap(bst_id_print(scan_result));
            puts_log("entry");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }

        lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                  (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                  STR_ILK_BST_FN, true));
        hash_loc fn_loc = hash.loc;
        if (hash.exists) {
            unwrap(already_seen_function_print(ctx, fn_loc));
            return;
        }
        set_fn_type(fn_loc, FN_CLASS_FIELD);
        set_ilk_info(fn_loc, num_fields());
        set_num_fields(num_fields() + 1);

        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("entry");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (num_fields() == num_pre_defined_fields()) {
        puts_log("Warning--I didn't find any fields");
        unwrap(bst_warn_print(ctx));
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            unwrap(bst_id_print(scan_result));
            puts_log("entry");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }

        lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                  (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                  STR_ILK_BST_FN, true));
        hash_loc fn_loc = hash.loc;
        if (hash.exists) {
            unwrap(already_seen_function_print(ctx, fn_loc));
            return;
        }
        set_fn_type(fn_loc, FN_CLASS_INT_ENTRY_VAR);
        set_ilk_info(fn_loc, num_ent_ints());
        set_num_ent_ints(num_ent_ints() + 1);

        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("entry");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("entry");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            unwrap(bst_id_print(scan_result));
            puts_log("entry");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }

        lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                  (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                  STR_ILK_BST_FN, true));
        hash_loc fn_loc = hash.loc;
        if (hash.exists) {
            unwrap(already_seen_function_print(ctx, fn_loc));
            return;
        }
        set_fn_type(fn_loc, FN_CLASS_STR_ENTRY_VAR);
        set_ilk_info(fn_loc, num_ent_strs());
        set_num_ent_strs(num_ent_strs() + 1);

        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("entry");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
}

static bool bad_argument_token(Bibtex* ctx, hash_loc* fn_out)
{
    lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_BST_FN, false));
    hash_loc fn_loc = hash.loc;
    if (fn_out != NULL) {
        *fn_out = fn_loc;
    }
    if (!hash.exists) {
        print_a_token();
        puts_log(" is an unknown function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return true;
    } else if ((fn_type(fn_loc) != FN_CLASS_BUILTIN) && (fn_type(fn_loc) != FN_CLASS_WIZARD)) {
        print_a_token();
        puts_log(" has bad function type ");
        print_fn_class(fn_loc);
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return true;
    }
    return false;
}

static void bst_execute_command(ExecCtx* ctx)
{
    if (!ctx->glbl_ctx->read_seen) {
        puts_log("Illegal, execute command before read command");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("execute");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("execute");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("execute");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
    else {
        unwrap(bst_id_print(scan_result));
        puts_log("execute");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    hash_loc fn_loc = 0;
    if (bad_argument_token(ctx->glbl_ctx, &fn_loc))
        return;
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("execute");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        bst_right_brace_print();
        puts_log("execute");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);

    init_command_execution(ctx);
    ctx->mess_with_entries = false;
    execute_fn(ctx, fn_loc);
    unwrap(check_command_execution(ctx));
}

static void bst_function_command(ExecCtx* ctx)
{
    hash_loc wiz_loc = 0;

    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }


    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);

    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }

    ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
    else {
        unwrap(bst_id_print(scan_result));
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }

    lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_BST_FN, true));
    wiz_loc = hash.loc;
    if (hash.exists) {
        unwrap(already_seen_function_print(ctx->glbl_ctx, wiz_loc));
        return;
    }
    set_fn_type(wiz_loc, FN_CLASS_WIZARD);
    if (hash_text(wiz_loc) == ctx->glbl_ctx->s_default)
        ctx->_default = wiz_loc;
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        bst_right_brace_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }

    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("function");
        unwrap(bst_err_print_and_look_for_blank_line(ctx->glbl_ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    unwrap(scan_fn_def(ctx->glbl_ctx, wiz_loc, wiz_loc));
}

static void bst_integers_command(Bibtex* ctx)
{
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("integers");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("integers");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("integers");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            unwrap(bst_id_print(scan_result));
            puts_log("integers");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }

        lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
        LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                  (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                  STR_ILK_BST_FN, true));
        hash_loc fn_loc = hash.loc;
        if (hash.exists) {
            unwrap(already_seen_function_print(ctx, fn_loc));
            return;
        }
        set_fn_type(fn_loc, FN_CLASS_INT_GLBL_VAR);
        set_ilk_info(fn_loc, 0);
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("integers");
            unwrap(bst_err_print_and_look_for_blank_line(ctx));
            return;
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
}

static void bst_iterate_command(ExecCtx* ctx)
{
    if (!ctx->glbl_ctx->read_seen) {
        puts_log("Illegal, iterate command before read command");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("iterate");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("iterate");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("iterate");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
    else {
        unwrap(bst_id_print(scan_result));
        puts_log("iterate");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    hash_loc fn_loc = 0;
    if (bad_argument_token(ctx->glbl_ctx, &fn_loc))
        return;
    if (!eat_bst_white_space(ctx->glbl_ctx)) {
        eat_bst_print();
        puts_log("iterate");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        bst_right_brace_print();
        puts_log("iterate");
        bst_err_print_and_look_for_blank_line(ctx->glbl_ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);

    init_command_execution(ctx);
    ctx->mess_with_entries = true;
    cite_number sort_cite_ptr = 0;
    while (sort_cite_ptr < num_cites()) {

        set_cite_ptr(cite_info(sort_cite_ptr));

        execute_fn(ctx, fn_loc);
        unwrap(check_command_execution(ctx));
        sort_cite_ptr = sort_cite_ptr + 1;
    }
}

static void bst_macro_command(Bibtex* ctx)
{
    if (ctx->read_seen) {
        puts_log("Illegal, macro command after read command");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    hash_loc macro_name_loc = 0;
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
    else {
        unwrap(bst_id_print(scan_result));
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_MACRO, true));
    macro_name_loc = hash.loc;
    if (hash.exists) {
        print_a_token();
        puts_log(" is already defined as a macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    set_ilk_info(macro_name_loc, hash_text(macro_name_loc));
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        bst_right_brace_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 34 /*double_quote */ ) {
        puts_log("A macro definition must be \"-delimited");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!scan1(34 /*double_quote */)) {
        puts_log("There's no `\"' to end macro definition");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }

    hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                    (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), STR_ILK_TEXT,
                                    true));
    hash_loc macro_def_loc = hash.loc;
    set_fn_type(macro_def_loc, FN_CLASS_STR_LIT);
    set_ilk_info(macro_name_loc, hash_text(macro_def_loc));
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        bst_right_brace_print();
        puts_log("macro");
        unwrap(bst_err_print_and_look_for_blank_line(ctx));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
}

static void get_bib_command_or_entry_and_process(Bibtex* ctx, hash_loc* cur_macro_loc, hash_loc* field_name_loc)
{
    BufPointer tmp_ptr, tmp_end_ptr;
    int32_t command_num = 0;
    cite_number lc_cite_loc = 0;
    hash_loc entry_type_loc = 0;
    bool type_exists = false;
    bool at_bib_command = false;
    ASCIICode right_outer_delim;

    while (!scan1(64 /*at_sign */)) {

        if (!input_ln(cur_bib_file()))
            return;
        set_bib_line_num(bib_line_num() + 1);
        bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 64 /*at_sign */ ) {
        puts_log("An \"@\" disappeared");
        print_confusion();
        longjmp(error_jmpbuf, 1);
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bib_white_space()) {
        unwrap(eat_bib_print(at_bib_command));
        return;
    }
    ScanRes scan_result = scan_identifier(123 /*left_brace */ , 40 /*left_paren */ , 40 /*left_paren */);
    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
    else {
        unwrap(bib_id_print(scan_result));
        puts_log("an entry type");
        unwrap(bib_err_print(at_bib_command));
        return;
    }

    lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    LookupRes hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                              (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                              STR_ILK_BIB_COMMAND, false));
    command_num = ilk_info(hash.loc);
    if (hash.exists) {
        at_bib_command = true;
        switch ((command_num)) {
        case 0:
            return;
            break;
        case 1:
            check_bib_files(preamble_ptr());
            if (!eat_bib_white_space()) {
                unwrap(eat_bib_print(at_bib_command));
                return;
            }
            if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ )
                right_outer_delim = 125 /*right_brace */ ;
            else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 40 /*left_paren */ )
                right_outer_delim = 41 /*right_paren */ ;
            else {
                unwrap(bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */, at_bib_command));
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            if (!eat_bib_white_space()) {
                unwrap(eat_bib_print(at_bib_command));
                return;
            }
            if (!unwrap_bool(
                    scan_and_store_the_field_value_and_eat_white(ctx, true, at_bib_command, command_num, &lc_cite_loc,
                                                                 *cur_macro_loc, right_outer_delim, *field_name_loc)))
                return;
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != right_outer_delim) {
                printf_log("Missing \"%c\" in preamble command", right_outer_delim);
                unwrap(bib_err_print(at_bib_command));
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            return;
            break;
        case 2:
            if (!eat_bib_white_space()) {
                unwrap(eat_bib_print(at_bib_command));
                return;
            }
            if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ )
                right_outer_delim = 125 /*right_brace */ ;
            else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 40 /*left_paren */ )
                right_outer_delim = 41 /*right_paren */ ;
            else {
                unwrap(bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */, at_bib_command));
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            if (!eat_bib_white_space()) {
                unwrap(eat_bib_print(at_bib_command));
                return;
            }
            scan_result = scan_identifier(61 /*equals_sign */ , 61 /*equals_sign */ , 61 /*equals_sign */);
            if (((scan_result == SCAN_RES_WHITESPACE_ADJACENT)
                 || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT))) ;
            else {
                unwrap(bib_id_print(scan_result));
                puts_log("a string name");
                unwrap(bib_err_print(at_bib_command));
                return;
            }
            lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                            (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                            STR_ILK_MACRO, true));
            *cur_macro_loc = hash.loc;
            set_ilk_info(*cur_macro_loc, hash_text(*cur_macro_loc));
            if (!eat_bib_white_space()) {
                unwrap(eat_bib_print(at_bib_command));
                return;
            }
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 61 /*equals_sign */ ) {
                unwrap(bib_equals_sign_print(at_bib_command));
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            if (!eat_bib_white_space()) {
                unwrap(eat_bib_print(at_bib_command));
                return;
            }
            if (!unwrap_bool(
                    scan_and_store_the_field_value_and_eat_white(ctx, true, at_bib_command, command_num, &lc_cite_loc,
                                                                 *cur_macro_loc, right_outer_delim, *field_name_loc)))
                return;
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != right_outer_delim) {
                printf_log("Missing \"%c\" in string command", right_outer_delim);
                unwrap(bib_err_print(at_bib_command));
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            return;
            break;
        default:
            bib_cmd_confusion();
            longjmp(error_jmpbuf, 1);
            break;
        }
    } else {
        hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                        (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                        STR_ILK_BST_FN, false));
        entry_type_loc = hash.loc;
        if ((!hash.exists) || (fn_type(entry_type_loc) != FN_CLASS_WIZARD))
            type_exists = false;
        else
            type_exists = true;
    }
    if (!eat_bib_white_space()) {
        unwrap(eat_bib_print(at_bib_command));
        return;
    }
    if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ )
        right_outer_delim = 125 /*right_brace */ ;
    else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 40 /*left_paren */ )
        right_outer_delim = 41 /*right_paren */ ;
    else {

        unwrap(bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */, at_bib_command));
        return;
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!eat_bib_white_space()) {
        unwrap(eat_bib_print(at_bib_command));
        return;
    }
    if (right_outer_delim == 41 /*right_paren */ ) {
        scan1_white(44 /*comma */);
    } else {
        scan2_white(44 /*comma */ , 125 /*right_brace */);
    }

    tmp_ptr = bib_buf_offset(BUF_TY_BASE, 1);
    while (tmp_ptr < bib_buf_offset(BUF_TY_BASE, 2)) {
        bib_set_buf(BUF_TY_EX, tmp_ptr, bib_buf(BUF_TY_BASE, tmp_ptr));
        tmp_ptr = tmp_ptr + 1;
    }
    lower_case(BUF_TY_EX, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    hash = unwrap_lookup(str_lookup(BUF_TY_EX, bib_buf_offset(BUF_TY_BASE, 1),
                                    (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                    STR_ILK_LC_CITE, ctx->all_entries));
    lc_cite_loc = hash.loc;
    if (hash.exists) {
        set_entry_cite_ptr(ilk_info(ilk_info(lc_cite_loc)));
        if ((!ctx->all_entries) || (entry_cite_ptr() < all_marker()) || (entry_cite_ptr() >= old_num_cites())) {
            if (type_list(entry_cite_ptr()) == 0 /*empty */ ) {
                if ((!ctx->all_entries) && (entry_cite_ptr() >= old_num_cites())) {
                    hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                                    (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                                    STR_ILK_CITE, true));
                    cite_number cite_loc = hash.loc;
                    if (!hash.exists) {
                        set_ilk_info(lc_cite_loc, cite_loc);
                        set_ilk_info(cite_loc, entry_cite_ptr());
                        set_cite_list(entry_cite_ptr(), hash_text(cite_loc));
                        hash.exists = true;
                    }
                }
                goto lab26;
            }
        } else if (!entry_exists(entry_cite_ptr())) {
            bib_set_buf_offset(BUF_TY_EX, 1, 0);
            tmp_ptr = bib_str_start(cite_info(entry_cite_ptr()));
            tmp_end_ptr = bib_str_start(cite_info(entry_cite_ptr()) + 1);
            while (tmp_ptr < tmp_end_ptr) {

                bib_set_buf(BUF_TY_EX, bib_buf_offset(BUF_TY_EX, 1), bib_str_pool(tmp_ptr));
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                tmp_ptr = tmp_ptr + 1;
            }
            lower_case(BUF_TY_EX, 0,
                       (bib_str_start(cite_info(entry_cite_ptr()) + 1) -
                        bib_str_start(cite_info(entry_cite_ptr()))));
            hash = unwrap_lookup(str_lookup(BUF_TY_EX, 0,
                                            (bib_str_start(cite_info(entry_cite_ptr()) + 1) -
                                             bib_str_start(cite_info(entry_cite_ptr()))), STR_ILK_LC_CITE, false));
            if (!hash.exists) {
                cite_key_disappeared_confusion();
                longjmp(error_jmpbuf, 1);
            }
            if (hash.loc == lc_cite_loc)
                goto lab26;
        }
        if (type_list(entry_cite_ptr()) == 0 /*empty */ ) {
            puts_log("The cite list is messed up");
            print_confusion();
            longjmp(error_jmpbuf, 1);
        }

        puts_log("Repeated entry");
        unwrap(bib_err_print(at_bib_command));
        return;
lab26:                        /*first_time_entry */ ;
    }
    bool store_entry = true;
    if (ctx->all_entries) {
        cite_number cite_loc = 0;
        if (hash.exists) {
            if (entry_cite_ptr() < all_marker())
                goto lab22;
            else {
                set_entry_exists(entry_cite_ptr(), true);
                cite_loc = ilk_info(lc_cite_loc);
            }
        } else {
            hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                            (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                            STR_ILK_CITE, true));
            cite_loc = hash.loc;
            if (hash.exists) {
                hash_cite_confusion();
                longjmp(error_jmpbuf, 1);
            }
        }
        set_entry_cite_ptr(cite_ptr());
        set_cite_ptr(add_database_cite(cite_ptr(), cite_loc, lc_cite_loc));
lab22:                        /*cite_already_set */ ;
    } else if (!hash.exists)
        store_entry = false;
    if (store_entry) {
        if (type_exists)
            set_type_list(entry_cite_ptr(), entry_type_loc);
        else {
            set_type_list(entry_cite_ptr(), undefined());
            puts_log("Warning--entry type for \"");
            print_a_token();
            puts_log("\" isn't style-file defined\n");
            unwrap(bib_warn_print());
        }
    }
    if (!eat_bib_white_space()) {
        unwrap(eat_bib_print(at_bib_command));
        return;
    }
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != right_outer_delim) {

        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 44 /*comma */ ) {
            unwrap(bib_one_of_two_print(44 /*comma */ , right_outer_delim, at_bib_command));
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        if (!eat_bib_white_space()) {
            unwrap(eat_bib_print(at_bib_command));
            return;
        }
        if (bib_buf_at_offset(BUF_TY_BASE, 2) == right_outer_delim)
            goto loop_exit;
        scan_result = scan_identifier(61 /*equals_sign */ , 61 /*equals_sign */ , 61 /*equals_sign */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            unwrap(bib_id_print(scan_result));
            puts_log("a field name");
            unwrap(bib_err_print(at_bib_command));
            return;
        }

        *field_name_loc = 0;
        bool store_field = false;
        if (store_entry) {
            lower_case(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            hash = unwrap_lookup(str_lookup(BUF_TY_BASE, bib_buf_offset(BUF_TY_BASE, 1),
                                            (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)),
                                            STR_ILK_BST_FN, false));
            *field_name_loc = hash.loc;
            if (hash.exists) {
                if (fn_type(*field_name_loc) == FN_CLASS_FIELD)
                    store_field = true;
            }
        }
        if (!eat_bib_white_space()) {
            unwrap(eat_bib_print(at_bib_command));
            return;
        }
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 61 /*equals_sign */ ) {
            unwrap(bib_equals_sign_print(at_bib_command));
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        if (!eat_bib_white_space()) {
            unwrap(eat_bib_print(at_bib_command));
            return;
        }
        if (!unwrap_bool(
                scan_and_store_the_field_value_and_eat_white(ctx, store_field, at_bib_command, command_num, NULL,
                                                             *cur_macro_loc, right_outer_delim, *field_name_loc)))
            return;
    }
loop_exit:
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
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
                get_bib_command_or_entry_and_process(ctx, &cur_macro_loc, &field_name_loc);
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
    if (bad_argument_token(ctx->glbl_ctx, &fn_loc))
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
        bst_entry_command(ctx->glbl_ctx);
        break;
    case 1:
        bst_execute_command(ctx);
        break;
    case 2:
        bst_function_command(ctx);
        break;
    case 3:
        bst_integers_command(ctx->glbl_ctx);
        break;
    case 4:
        bst_iterate_command(ctx);
        break;
    case 5:
        bst_macro_command(ctx->glbl_ctx);
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

static int
initialize(Bibtex* ctx, const char *aux_file_name)
{
    bib_set_pool_ptr(0);
    bib_set_str_ptr(1);
    bib_set_str_start(bib_str_ptr(), 0);
    ctx->bib_seen = false;
    ctx->bst_seen = false;
    ctx->citation_seen = false;
    ctx->all_entries = false;

    ctx->entry_seen = false;
    ctx->read_seen = false;
    ctx->read_performed = false;
    ctx->reading_completed = false;
    ctx->impl_fn_num = 0;
    bib_set_buf_len(BUF_TY_OUT, 0);

    unwrap(pre_def_certain_strings(ctx));
    return unwrap_str(get_the_top_level_aux_file_name(ctx, aux_file_name));
}


History
bibtex_main(Bibtex* glbl_ctx, const char *aux_file_name)
{
    reset_all();

    if (!init_standard_output())
        return HISTORY_FATAL_ERROR;

    if (initialize(glbl_ctx, aux_file_name)) {
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
            get_aux_command_and_process(glbl_ctx);
        }
    }

    last_check_for_aux_errors(glbl_ctx);

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